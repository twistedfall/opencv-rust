use std::{
	collections::{HashMap, HashSet},
	fmt,
	fs::File,
	io::{Read, Seek, SeekFrom},
	path::{Path, PathBuf},
};

use clang::{Entity, EntityKind, EntityVisitResult, Type};

use crate::{
	Class,
	class::Kind as ClassKind,
	comment,
	Element,
	EntityWalker,
	EntityWalkerVisitor,
	is_opencv_path,
	main_module_from_path,
	memo_map,
	MemoizeMap,
	NamePool,
	settings,
	TypeRef,
};

#[derive(Clone, Debug, Default)]
pub struct ExportConfig {
	pub simple: bool,
	pub deprecated: bool,
	pub no_return: bool,
	pub no_except: bool,
	pub only_dependent_types: bool,
	pub rename: Option<String>,
}

impl ExportConfig {
	pub fn simple() -> Self {
		ExportConfig { simple: true, ..Default::default() }
	}
}

type ExportIdx = (PathBuf, u32, u32);

struct CacheFiller<'ge, 'tu> {
	gen_env: &'ge mut GeneratorEnv<'tu>,
}

impl<'tu> EntityWalkerVisitor<'tu> for CacheFiller<'_, 'tu> {
	fn wants_file(&mut self, path: &Path) -> bool {
		is_opencv_path(path)
			|| path.ends_with("ocvrs_resolve_types.hpp")
			|| main_module_from_path(path).map_or(false, |m| m == self.gen_env.module)
	}

	fn visit_resolve_type(&mut self, typ: Type<'tu>) -> bool {
		let type_ref = TypeRef::new(typ, self.gen_env);
		let name = type_ref.cpp_full().into_owned();
		self.gen_env.type_resolve_cache.insert(name, typ);
		true
	}

	fn visit_entity(&mut self, entity: Entity<'tu>) -> bool {
		match entity.get_kind() {
			EntityKind::ClassDecl | EntityKind::StructDecl => {
				entity.visit_children(|c, _| {
					match c.get_kind() {
//						EntityKind::BaseSpecifier => {
//							let c_decl = c.get_definition().expect("Can't get base class definition");
//							self.gen_env.has_descendants_cache.insert(c_decl.usr().into_owned());
//						}
						EntityKind::Constructor | EntityKind::Method | EntityKind::FunctionTemplate
						| EntityKind::ConversionFunction => {
							self.gen_env.add_func_comment(c);
						}
						_ => {}
					}
					EntityVisitResult::Continue
				});
			}
			EntityKind::FunctionDecl => {
				self.gen_env.add_func_comment(entity);
			}
			_ => {}
		}
		true
	}
}

pub struct GeneratorEnv<'tu> {
	root_entity: Entity<'tu>,
	module: &'tu str,
	export_map: HashMap<ExportIdx, ExportConfig>,
	pub func_names: NamePool,
	func_comments: HashMap<String, String>,
	has_descendants_cache: HashSet<String>,
	class_kind_cache: MemoizeMap<String, Option<ClassKind>>,
	type_resolve_cache: HashMap<String, Type<'tu>>,
}

impl<'tu> GeneratorEnv<'tu> {
	pub fn new(root_entity: Entity<'tu>, module: &'tu str) -> Self {
		let mut out = Self {
			root_entity,
			module,
			export_map: HashMap::with_capacity(1024),
			func_names: NamePool::with_capacity(512),
			func_comments: HashMap::with_capacity(2048),
			has_descendants_cache: HashSet::with_capacity(128),
			class_kind_cache: MemoizeMap::new(HashMap::with_capacity(32)),
			type_resolve_cache: HashMap::with_capacity(32),
		};
		out.fill_caches();
		out
	}

	fn fill_caches(&mut self) {
		let walker = EntityWalker::new(self.root_entity);
		walker.walk_opencv_entities(CacheFiller { gen_env: self });
	}

	fn add_func_comment(&mut self, entity: Entity<'tu>) {
		let raw_comment = entity.get_comment().unwrap_or_default();
		if !raw_comment.is_empty() && !raw_comment.contains("@overload") {
			let name = entity.cpp_fullname().into_owned();
			self.func_comments.insert(name, comment::strip_comment_markers(&raw_comment));
		}
	}

	pub fn module(&self) -> &str {
		self.module
	}

	fn key(entity: Entity) -> ExportIdx {
		let (loc, offset) = if entity.get_kind() == EntityKind::MacroExpansion {
			// sometimes CV_EXPORT macros are located on a separate line so for those we compensate the offset
			let l = entity.get_range().expect("Can't get export macro range").get_end().get_spelling_location();
			let mut f = File::open(l.file.expect("Can't get export macro file").get_path()).expect("Can't open export macro file");
			f.seek(SeekFrom::Start(l.offset as u64)).expect("Can't seek export macro file");
			let mut buf = [0; 8];
			let read = f.read(&mut buf).expect("Can't read file");
			let line_offset = buf[0..read].iter()
				.take_while(|&&c| c == b'\n')
				.count();
			if line_offset > 1 {
				panic!("Line offset more than 1 is not supported, modify fuzzy_key in get_export_config() to support higher values");
			}
			(l, line_offset as u32)
		} else {
			(entity.get_range().expect("Can't get entity range").get_start().get_spelling_location(), 0)
		};
		(loc.file.expect("Can't get exported entry file").get_path(), loc.line, offset)
	}

	pub fn make_export_config(&mut self, entity: Entity) -> &mut ExportConfig {
		let key = Self::key(entity);
		self.export_map.entry(key).or_default()
	}

	pub fn get_export_config(&self, entity: Entity) -> Option<&ExportConfig> {
		settings::ELEMENT_EXPORT.get(entity.cpp_fullname().as_ref()).or_else(|| {
			let key = Self::key(entity);
			self.export_map.get(&key)
				.or_else(|| {
					// for cases where CV_EXPORTS is on the separate line but entity.get_range() spans into it
					let fuzzy_key = (key.0, key.1, 1);
					self.export_map.get(&fuzzy_key)
						.or_else(|| if fuzzy_key.1 >= 1 {
							// for cases where CV_EXPORTS is on the separate line but entity.get_range() start on the next line
							let fuzzy_key = (fuzzy_key.0, fuzzy_key.1 - 1, fuzzy_key.2);
							self.export_map.get(&fuzzy_key)
						} else {
							None
						})
				})
		})
	}

	pub fn get_func_comment(&self, cpp_fullname: &str) -> Option<&str> {
		self.func_comments.get(cpp_fullname).map(|x| x.as_str())
	}

	pub fn has_descendants(&self, entity: Entity<'tu>) -> bool {
		self.has_descendants_cache.contains(entity.usr().as_ref())
	}

	pub fn get_class_kind(&self, entity: Entity<'tu>) -> Option<ClassKind> {
		let id = entity.usr();
		memo_map(&self.class_kind_cache, id.as_ref(), || {
			if let Some(range) = entity.get_range() {
				let name_ranges = if let Some(tpl_decl) = entity.get_template() {
					tpl_decl
				} else {
					entity
				}.get_name_ranges();
				if !name_ranges.is_empty() {
					let file_location = range.get_start().get_file_location();
					if let Some(file) = file_location.file {
						let start = file_location.offset as u64;
						let end = name_ranges[0].get_start().get_file_location().offset as u64;
						let mut buf = vec![0; (end - start) as usize];
						if let Ok(mut f) = File::open(file.get_path()) {
							f.seek(SeekFrom::Start(start)).expect("Can't seek");
							f.read_exact(buf.as_mut_slice()).expect("Can't read");
						}
						let export_decl = String::from_utf8(buf).expect("Not a valid UTF-8");
						if export_decl.contains("CV_EXPORTS_W_SIMPLE") || export_decl.contains("CV_EXPORTS_W_MAP") {
							return Some(ClassKind::Simple)
						} else if export_decl.contains("CV_EXPORTS") {
							return Some(ClassKind::Boxed)
						}
					}
				}
			}
			let cls = Class::new(entity, self);
			if cls.detect_class_simplicity() {
				return Some(ClassKind::Simple)
			}
			None
		})
	}

	pub fn resolve_type(&self, typ: &str) -> Option<Type<'tu>> {
		self.type_resolve_cache.get(typ).copied()
	}
}

impl fmt::Debug for GeneratorEnv<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("GeneratorEnv")
			.field("export_map", &format!("{} elements", self.export_map.len()))
			.field("func_comments", &format!("{} elements", self.func_comments.len()))
			.field("has_descendants_cache", &format!("{} elements", self.has_descendants_cache.len()))
			.field("class_kind_cache", &format!("{} elements", self.class_kind_cache.borrow().len()))
			.field("type_resolve_cache", &format!("{} elements", self.type_resolve_cache.len()))
			.finish()
	}
}
