use std::{
	collections::{HashMap, HashSet},
	fmt,
	fs::File,
	io::{Read, Seek, SeekFrom},
	path::{Path, PathBuf},
};

use clang::{Entity, EntityKind, EntityVisitResult, StorageClass, Type};

use crate::{
	Class,
	class::Kind as ClassKind,
	comment,
	Const,
	Element,
	EntityWalker,
	EntityWalkerVisitor,
	Func,
	is_ephemeral_header,
	is_opencv_path,
	memo_map,
	MemoizeMap,
	NamePool,
	opencv_module_from_path,
	settings,
	TypeRef,
};

#[derive(Clone, Debug, Default)]
pub struct ExportConfig {
	pub simple: bool,
	pub deprecated: bool,
	pub no_return: bool,
	pub no_except: bool,
	pub no_discard: bool,
	pub only_dependent_types: bool,
}

pub struct RenameConfig {
	pub rename: String,
}

impl ExportConfig {
	pub fn simple() -> Self {
		ExportConfig { simple: true, ..Default::default() }
	}

	pub fn make_noexcept(src: &mut ExportConfig) {
		src.no_except = true;
	}

	pub fn make_boxed(src: &mut ExportConfig) {
		src.simple = false;
	}
}

type ExportIdx = (PathBuf, u32, u32);

struct DbPopulator<'tu, 'ge> {
	gen_env: &'ge mut GeneratorEnv<'tu>,
}

impl<'tu> DbPopulator<'tu, '_> {
	fn add_func_comment(&mut self, entity: Entity) {
		let raw_comment = entity.get_comment().unwrap_or_default();
		if !raw_comment.is_empty() && !raw_comment.contains("@overload") {
			let name = entity.cpp_fullname().into_owned();
			let line = entity.get_location().map(|l| l.get_file_location().line).unwrap_or(0);
			let defs = self.gen_env.func_comments.entry(name).or_insert_with(Vec::new);
			defs.push((line, comment::strip_comment_markers(&raw_comment)));
			// reverse sort due to how we're querying this
			defs.sort_unstable_by(|(left_line, _), (right_line, _)| right_line.cmp(left_line));
		}
	}

	fn add_class_constant(&mut self, cnst: Entity<'tu>) {
		let cnst = Const::new(cnst);
		let mut full_name = cnst.cpp_fullname().into_owned();
		self.gen_env.class_constants.insert(full_name.clone(), cnst.clone());
		const SEP: &str = "::";
		while let Some(idx) = full_name.find(SEP) {
			full_name.drain(..idx + SEP.len());
			self.gen_env.class_constants.insert(full_name.clone(), cnst.clone());
		}
		self.gen_env.class_constants.insert(full_name, cnst);
	}

	fn add_used_in_smart_ptr(&mut self, func: Entity<'tu>) {
		let args = Func::new(func, self.gen_env).arguments().into_iter()
			.filter_map(|arg| arg.type_ref().as_smart_ptr())
			.filter_map(|smart_ptr| smart_ptr.pointee().clang_type().get_declaration())
			.collect::<Vec<_>>();
		for arg in args {
			self.gen_env.used_in_smart_ptr.insert(arg);
		}
	}
}

impl<'tu> EntityWalkerVisitor<'tu> for DbPopulator<'tu, '_> {
	fn wants_file(&mut self, path: &Path) -> bool {
		is_opencv_path(path)
			|| is_ephemeral_header(path)
			|| opencv_module_from_path(path).map_or(false, |m| m == self.gen_env.module)
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
						EntityKind::BaseSpecifier => {
							let c_decl = c.get_definition().expect("Can't get base class definition");
							self.gen_env.descendants.entry(c_decl.cpp_fullname().into_owned())
								.or_insert_with(|| HashSet::with_capacity(4))
								.insert(entity);
						}
						EntityKind::Constructor | EntityKind::Method | EntityKind::FunctionTemplate
						| EntityKind::ConversionFunction => {
							self.add_func_comment(c);
							self.add_used_in_smart_ptr(c);
						}
						EntityKind::VarDecl => {
							if let Some(StorageClass::Static) = c.get_storage_class() {
								if c.evaluate().is_some() {
									self.add_class_constant(c);
								}
							}
						}
						_ => {}
					}
					EntityVisitResult::Continue
				});
			}
			EntityKind::FunctionDecl => {
				self.add_func_comment(entity);
				self.add_used_in_smart_ptr(entity);
			}
			_ => {}
		}
		true
	}
}

pub struct GeneratorEnv<'tu> {
	module: &'tu str,
	export_map: HashMap<ExportIdx, ExportConfig>,
	rename_map: HashMap<ExportIdx, RenameConfig>,
	pub func_names: NamePool,
	func_comments: HashMap<String, Vec<(u32, String)>>,
	class_kind_cache: MemoizeMap<String, Option<ClassKind>>,
	class_constants: HashMap<String, Const<'tu>>,
	type_resolve_cache: HashMap<String, Type<'tu>>,
	used_in_smart_ptr: HashSet<Entity<'tu>>,
	pub descendants: HashMap<String, HashSet<Entity<'tu>>>,
}

impl<'tu> GeneratorEnv<'tu> {
	pub fn new(root_entity: Entity<'tu>, module: &'tu str) -> Self {
		let mut out = Self {
			module,
			export_map: HashMap::with_capacity(1024),
			rename_map: HashMap::with_capacity(64),
			func_names: NamePool::with_capacity(512),
			func_comments: HashMap::with_capacity(2048),
			class_kind_cache: MemoizeMap::new(HashMap::with_capacity(32)),
			class_constants: HashMap::with_capacity(32),
			type_resolve_cache: HashMap::with_capacity(32),
			used_in_smart_ptr: HashSet::with_capacity(32),
			descendants: HashMap::with_capacity(16),
		};
		let walker = EntityWalker::new(root_entity);
		walker.walk_opencv_entities(DbPopulator { gen_env: &mut out });
		out
	}

	pub fn module(&self) -> &str {
		self.module
	}

	fn key(entity: Entity) -> ExportIdx {
		let (loc, offset) = if entity.get_kind() == EntityKind::MacroExpansion {
			// sometimes CV_EXPORT macros are located on a separate line so for those we compensate the offset
			let l = entity.get_range().expect("Can't get exported macro range").get_end().get_spelling_location();
			let path = l.file.expect("Can't get exported macro file").get_path();
			if is_ephemeral_header(&path) {
				(l, 0)
			} else {
				let mut f = File::open(path).expect("Can't open export macro file");
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
			}
		} else {
			let loc = if let Some(range) = entity.get_range() {
				range.get_start().get_spelling_location()
			} else {
				// for some reason Apple libclang on macos has problems with get_range() on FacemarkLBF::Params::pupils
				// see https://github.com/twistedfall/opencv-rust/issues/159#issuecomment-668234058
				entity.get_location().expect("Can't get entity location").get_spelling_location()
			};
			(loc, 0)
		};
		(loc.file.expect("Can't get exported entry file").get_path(), loc.line, offset)
	}

	pub fn make_export_config(&mut self, entity: Entity) -> &mut ExportConfig {
		let key = Self::key(entity);
		self.export_map.entry(key).or_default()
	}

	#[inline]
	fn get_with_fuzzy_key<T>(entity: Entity, f: impl Fn(&ExportIdx) -> Option<T>) -> Option<T> {
		let key = Self::key(entity);
		f(&key).or_else(|| {
			// for cases where CV_EXPORTS is on the separate line but entity.get_range() spans into it
			let fuzzy_key = (key.0, key.1, 1);
			f(&fuzzy_key).or_else(|| if fuzzy_key.1 >= 1 {
				// for cases where CV_EXPORTS is on the separate line but entity.get_range() start on the next line
				let fuzzy_key = (fuzzy_key.0, fuzzy_key.1 - 1, fuzzy_key.2);
				f(&fuzzy_key)
			} else {
				None
			})
		})
	}

	pub fn get_export_config(&self, entity: Entity) -> Option<ExportConfig> {
		let cpp_fullname = entity.cpp_fullname();
		settings::ELEMENT_EXPORT_MANUAL.get(cpp_fullname.as_ref()).or_else(|| {
			Self::get_with_fuzzy_key(entity, |key| self.export_map.get(key))
		})
			.cloned()
			.map(|mut e| {
				if let Some(cb) = settings::ELEMENT_EXPORT_TWEAK.get(cpp_fullname.as_ref()) {
					cb(&mut e);
				}
				e
			})
	}

	pub fn make_rename_config(&mut self, entity: Entity) -> &mut RenameConfig {
		let key = Self::key(entity);
		self.rename_map.entry(key).or_insert_with(|| RenameConfig { rename: String::new() })
	}

	pub fn get_rename_config(&self, entity: Entity) -> Option<&RenameConfig> {
		Self::get_with_fuzzy_key(entity, |key| self.rename_map.get(key))
	}

	pub fn get_func_comment(&self, line: u32, cpp_fullname: &str) -> Option<&str> {
		self.func_comments.get(cpp_fullname)
			.and_then(|comments| comments.iter()
				// try to find the source function comment that is closest to the requested
				.find(|(source_line, _)| *source_line <= line)
				// if it fails return at least something
				.or_else(|| comments.last()))
			.map(|(_, comment)| comment.as_str())
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
						} else if export_decl.contains("CV_EXPORTS") || export_decl.contains("GAPI_EXPORTS") {
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

	pub fn resolve_class_constant(&self, constant: &str) -> Option<&Const<'tu>> {
		self.class_constants.get(constant)
	}

	pub fn resolve_type(&self, typ: &str) -> Option<Type<'tu>> {
		self.type_resolve_cache.get(typ).copied()
	}

	pub fn is_used_in_smart_ptr(&self, entity: Entity) -> bool {
		self.used_in_smart_ptr.contains(&entity)
	}
}

impl fmt::Debug for GeneratorEnv<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("GeneratorEnv")
			.field("export_map", &format!("{} elements", self.export_map.len()))
			.field("func_comments", &format!("{} elements", self.func_comments.len()))
			.field("class_kind_cache", &format!("{} elements", self.class_kind_cache.borrow().len()))
			.field("type_resolve_cache", &format!("{} elements", self.type_resolve_cache.len()))
			.finish()
	}
}
