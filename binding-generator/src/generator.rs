use std::{
	borrow::Cow,
	collections::{HashMap, HashSet},
	fmt::{self, Write},
	fs::File,
	io::BufReader,
	path::{Path, PathBuf},
};

use clang::{
	Clang,
	diagnostic::Diagnostic,
	diagnostic::Severity,
	Entity,
	EntityKind,
	EntityVisitResult,
	Index,
	TranslationUnit,
	Type,
	Unsaved,
};
use dunce::canonicalize;
use maplit::hashmap;
use once_cell::sync::Lazy;

use crate::{
	AbstractRefWrapper,
	Class,
	CompiledInterpolation,
	Const,
	Element,
	EntityExt,
	EntityWalker,
	EntityWalkerVisitor,
	Enum,
	Func,
	FunctionTypeHint,
	GeneratorEnv,
	get_definition_text,
	line_reader,
	opencv_module_from_path,
	return_type_wrapper::ReturnTypeWrapper,
	settings,
	smart_ptr::SmartPtr,
	StrExt,
	type_ref::{FishStyle, Kind as TypeRefKind},
	Typedef,
	vector::Vector,
};

#[derive(Debug)]
pub enum DependentType<'tu, 'ge> {
	ReturnTypeWrapper(ReturnTypeWrapper<'tu, 'ge>),
	AbstractRefWrapper(AbstractRefWrapper<'tu, 'ge>),
	Vector(Vector<'tu, 'ge>),
	SmartPtr(SmartPtr<'tu, 'ge>),
}

impl<'tu, 'ge> DependentType<'tu, 'ge> {
	pub fn from_return_type_wrapper(s: ReturnTypeWrapper<'tu, 'ge>) -> Self {
		DependentType::ReturnTypeWrapper(s)
	}

	pub fn from_abstract_ref_wrapper(s: AbstractRefWrapper<'tu, 'ge>) -> Self {
		DependentType::AbstractRefWrapper(s)
	}

	pub fn from_vector(s: Vector<'tu, 'ge>) -> Self {
		DependentType::Vector(s)
	}

	pub fn from_smart_ptr(s: SmartPtr<'tu, 'ge>) -> Self {
		DependentType::SmartPtr(s)
	}
}

#[allow(unused)]
pub trait GeneratorVisitor {
	fn wants_file(&mut self, path: &Path) -> bool { true }

	fn visit_module_comment(&mut self, comment: String) {}
	fn visit_const(&mut self, cnst: Const) {}
	fn visit_enum(&mut self, enm: Enum) {}
	fn visit_func(&mut self, func: Func) {}
	fn visit_typedef(&mut self, typedef: Typedef) {}
	fn visit_class(&mut self, class: Class) {}
	fn visit_dependent_type(&mut self, typ: DependentType) {}

	fn visit_ephemeral_header(&mut self, contents: &str) {}
}

struct EphemeralGenerator<'m> {
	module: &'m str,
	used_in_smart_ptr: HashSet<String>,
	descendants: HashMap<String, HashSet<String>>,
}

impl<'m> EphemeralGenerator<'m> {
	pub fn new(module: &'m str) -> Self {
		Self {
			module,
			used_in_smart_ptr: HashSet::with_capacity(32),
			descendants: HashMap::with_capacity(16),
		}
	}

	fn add_used_in_smart_ptr(&mut self, func: Entity) {
		for arg in func.get_arguments().into_iter().flatten() {
			if let Some(arg_type) = arg.get_type() {
				if arg_type.get_declaration().map_or(false, |ent| ent.cpp_fullname().starts_with("cv::Ptr")) {
					let inner_type_ent = arg_type.get_template_argument_types().into_iter()
						.flatten()
						.flatten()
						.next()
						.and_then(|t| t.get_declaration());
					if let Some(inner_type_ent) = inner_type_ent {
						self.used_in_smart_ptr.insert(inner_type_ent.cpp_fullname().into_owned());
					}
				}
			}
		}
	}

	pub fn generate_header(&self) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/ephemeral/ephemeral.tpl.hpp").compile_interpolation()
		);

		let mut includes = String::with_capacity(128);
		let mut resolve_types = String::with_capacity(1024);
		let mut generate_types: Vec<Cow<_>> = Vec::with_capacity(32);

		let mut resolve_types_idx = 0usize;
		let global_tweaks = settings::GENERATOR_MODULE_TWEAKS.get("*");
		let module_tweaks = settings::GENERATOR_MODULE_TWEAKS.get(self.module);
		for tweak in global_tweaks.iter().chain(module_tweaks.iter()) {
			for &include in &tweak.includes {
				writeln!(&mut includes, "#include <opencv2/{}>", include).expect("Can't fail");
			}
			for &res_type in &tweak.resolve_types {
				writeln!(&mut resolve_types, "typedef {} ephem{};", res_type, resolve_types_idx).expect("Can't fail");
				resolve_types_idx += 1;
			}
			for &gen_type in &tweak.generate_types {
				generate_types.push(gen_type.into());
			}
		}
		let mut used_in_smart_ptr = self.used_in_smart_ptr.iter().collect::<Vec<_>>();
		used_in_smart_ptr.sort_unstable();
		for used_cppfull in used_in_smart_ptr {
			let mut descendants = self.descendants.get(used_cppfull).into_iter().flatten().collect::<Vec<_>>();
			descendants.sort_unstable();
			for desc_cppfull in descendants {
				if !self.used_in_smart_ptr.contains(desc_cppfull) {
					generate_types.push(format!("cv::Ptr<{}>", desc_cppfull).into());
				}
			}
		}

		TPL.interpolate(&hashmap! {
			"includes" => includes,
			"resolve_types" => resolve_types,
			"generate_types" => generate_types.join(",\n"),
		})
	}
}

impl EntityWalkerVisitor<'_> for &mut EphemeralGenerator<'_> {
	fn wants_file(&mut self, path: &Path) -> bool {
		opencv_module_from_path(path).map_or(false, |m| m == self.module)
	}

	fn visit_entity(&mut self, entity: Entity<'_>) -> bool {
		match entity.get_kind() {
			EntityKind::ClassDecl | EntityKind::StructDecl => {
				entity.visit_children(|c, _| {
					match c.get_kind() {
						EntityKind::BaseSpecifier => {
							let c_decl = c.get_definition().expect("Can't get base class definition");
							self.descendants.entry(c_decl.cpp_fullname().into_owned())
								.or_insert_with(|| HashSet::with_capacity(4))
								.insert(entity.cpp_fullname().into_owned());
						}
						EntityKind::Constructor | EntityKind::Method | EntityKind::FunctionTemplate
						| EntityKind::ConversionFunction => {
							self.add_used_in_smart_ptr(c)
						}
						_ => {}
					}
					EntityVisitResult::Continue
				});
			}
			EntityKind::FunctionDecl => {
				self.add_used_in_smart_ptr(entity);
			}
			_ => {}
		}
		true
	}
}

#[derive(Debug)]
pub struct Generator {
	clang_include_dirs: Vec<PathBuf>,
	opencv_include_dir: PathBuf,
	opencv_module_header_dir: PathBuf,
	src_cpp_dir: PathBuf,
	clang: Clang,
}

struct OpenCvWalker<'tu, 'r, V: GeneratorVisitor> {
	opencv_module_header_dir: &'r Path,
	module: &'r str,
	visitor: V,
	gen_env: GeneratorEnv<'tu>,
	comment_found: bool,
}

impl<'tu, V: GeneratorVisitor> EntityWalkerVisitor<'tu> for OpenCvWalker<'tu, '_, V> {
	fn wants_file(&mut self, path: &Path) -> bool {
		self.visitor.wants_file(path) || path.ends_with("ocvrs_common.hpp")
	}

	fn visit_entity(&mut self, entity: Entity<'tu>) -> bool {
		match entity.get_kind() {
			EntityKind::Namespace => {
				if !self.comment_found {
					if let Some(c) = entity.get_comment() {
						if c.contains("@defgroup") {
							if let Some(entity_file) = entity.get_location().and_then(|loc| loc.get_file_location().file) {
								if !settings::IGNORE_CLANG_MODULE_COMMENT.contains(self.module) && opencv_module_from_path(&entity_file.get_path()).map_or(false, |m| m == self.module) {
									self.comment_found = true;
									self.visitor.visit_module_comment(c)
								}
							}
						}
					}
				}
			}
			EntityKind::MacroDefinition => {
				self.process_const(entity)
			}
			EntityKind::MacroExpansion => {
				if let Some(name) = entity.get_name() {
					if name == "CV_EXPORTS" || name == "CV_EXPORTS_W" || name == "CV_WRAP" || name == "GAPI_EXPORTS" {
						self.gen_env.make_export_config(entity);
					} else if name == "CV_EXPORTS_W_SIMPLE" || name == "CV_EXPORTS_W_MAP" {
						self.gen_env.make_export_config(entity).simple = true;
					} else if name == "CV_EXPORTS_AS" || name == "CV_WRAP_AS" {
						let definition = get_definition_text(entity);
						let definition = if let Some(d) = definition.strip_prefix("CV_EXPORTS_AS(").and_then(|d| d.strip_suffix(')')) {
							d.trim()
						} else if let Some(d) = definition.strip_prefix("CV_WRAP_AS(").and_then(|d| d.strip_suffix(')')) {
							d.trim()
						} else {
							unreachable!("Incorrect CV_EXPORTS_AS(..) or CV_WRAP_AS(..) usage")
						};
						self.gen_env.make_export_config(entity);
						self.gen_env.make_rename_config(entity).rename = definition.to_string();
					} else if name == "CV_NORETURN" {
						self.gen_env.make_export_config(entity).no_return = true;
					} else if name == "CV_NOEXCEPT" {
						self.gen_env.make_export_config(entity).no_except = true;
					} else if name == "CV_DEPRECATED" || name == "CV_DEPRECATED_EXTERNAL" {
						self.gen_env.make_export_config(entity).deprecated = true;
					} else if name == "CV_NODISCARD_STD" || name == "CV_NODISCARD" {
						self.gen_env.make_export_config(entity).no_discard = true;
					} else if name == "OCVRS_ONLY_DEPENDENT_TYPES" {
						self.gen_env.make_export_config(entity).only_dependent_types = true;
					}
				}
			}
			EntityKind::ClassDecl | EntityKind::ClassTemplate | EntityKind::ClassTemplatePartialSpecialization
			| EntityKind::StructDecl => {
				Self::process_class(&mut self.visitor, &mut self.gen_env, entity)
			}
			EntityKind::EnumDecl => {
				Self::process_enum(&mut self.visitor, entity)
			}
			EntityKind::FunctionDecl => {
				Self::process_func(&mut self.visitor, &mut self.gen_env, entity)
			}
			EntityKind::TypedefDecl | EntityKind::TypeAliasDecl => {
				Self::process_typedef(&mut self.visitor, &mut self.gen_env, entity)
			}
			EntityKind::VarDecl => {
				if !entity.is_mutable() {
					self.process_const(entity);
				} else {
					unreachable!("Unsupported VarDecl: {:#?}", entity)
				}
			}
			_ => {
				unreachable!("Unsupported entity: {:#?}", entity)
			}
		}
		true
	}
}

impl<'tu, 'r, V: GeneratorVisitor> OpenCvWalker<'tu, 'r, V> {
	pub fn new(opencv_module_header_dir: &'r Path, module: &'r str, visitor: V, gen_env: GeneratorEnv<'tu>) -> Self {
		Self { opencv_module_header_dir, module, visitor, gen_env, comment_found: false }
	}

	fn process_const(&mut self, const_decl: Entity) {
		let cnst = Const::new(const_decl);
		if !cnst.is_excluded() {
			self.visitor.visit_const(cnst);
		}
	}

	fn process_class(visitor: &mut V, gen_env: &mut GeneratorEnv<'tu>, class_decl: Entity<'tu>) {
		if gen_env.get_export_config(class_decl).is_some() {
			let cls = Class::new(class_decl, gen_env);
			if !cls.is_excluded() {
				cls.dependent_types().into_iter()
					.for_each(|dep| {
						visitor.visit_dependent_type(dep);
					});
				class_decl.walk_enums_while(|enm| {
					let enm = Enum::new(enm);
					if enm.rust_leafname(FishStyle::No) != "unnamed" {
						if !enm.is_excluded() {
							visitor.visit_enum(enm);
						}
					} else {
						for cnst in enm.consts() {
							if !cnst.is_excluded() {
								visitor.visit_const(cnst);
							}
						}
					}
					true
				});
				class_decl.walk_classes_while(|sub_cls| {
					if !gen_env.get_export_config(sub_cls).is_some() {
						let cls = Class::new(sub_cls, gen_env);
						let is_simple = cls.detect_class_simplicity();
						gen_env.make_export_config(sub_cls).simple = is_simple;
					}
					Self::process_class(visitor, gen_env, sub_cls);
					true
				});
				class_decl.walk_typedefs_while(|tdef| {
					let typedef = Typedef::new(tdef, gen_env);
					if !typedef.is_excluded() {
						visitor.visit_typedef(typedef);
					}
					true
				});
				let cls = Class::new(class_decl, gen_env);
				visitor.visit_class(cls)
			}
		}
	}

	fn process_enum(visitor: &mut V, enum_decl: Entity) {
		let enm = Enum::new(enum_decl);
		if !enm.is_excluded() {
			if !enm.as_typedefed().is_some() {
				for cnst in enm.consts() {
					if !cnst.is_excluded() {
						visitor.visit_const(cnst);
					}
				}
			}
			if enm.rust_leafname(FishStyle::No) != "unnamed" {
				visitor.visit_enum(enm);
			}
		}
	}

	fn process_func(visitor: &mut V, gen_env: &mut GeneratorEnv<'tu>, func_decl: Entity<'tu>) {
		if let Some(only_dependent_types) = gen_env.get_export_config(func_decl).map(|e| e.only_dependent_types) {
			let func = Func::new(func_decl, gen_env);
			if !func.is_excluded() {
				let specs = settings::FUNC_SPECIALIZE.get(func.identifier().as_ref())
					.map_or_else(|| vec![FunctionTypeHint::None], |specs| specs.iter()
						.map(FunctionTypeHint::Specialized)
						.collect::<Vec<_>>(),
					);
				for type_hint in specs {
					let mut name_hint = None;
					if !only_dependent_types {
						let mut name = Func::new_ext(func_decl, type_hint, None, gen_env).rust_leafname(FishStyle::No).into_owned().into();
						gen_env.func_names.make_unique_name(&mut name);
						if let Cow::Owned(name) = name {
							name_hint = Some(name);
						}
					}
					let func = Func::new_ext(func_decl, type_hint, name_hint, gen_env);
					func.dependent_types().into_iter()
						.for_each(|dep| {
							visitor.visit_dependent_type(dep);
						});
					if !only_dependent_types {
						visitor.visit_func(func);
					}
				}
			}
		}
	}

	fn process_typedef(visitor: &mut V, gen_env: &mut GeneratorEnv<'tu>, typedef_decl: Entity<'tu>) {
		let typedef = Typedef::new(typedef_decl, gen_env);
		let type_ref = typedef.type_ref();
		match type_ref.kind() {
			TypeRefKind::Class(..) => {
				return Self::process_class(visitor, gen_env, typedef_decl);
			}
			TypeRefKind::Enum(..) => {
				return Self::process_enum(visitor, typedef_decl);
			}
			_ => {}
		}
		let mut export = gen_env.get_export_config(typedef_decl).is_some()
			// we need to have a typedef even if it's not exported for e.g. cv::Size
			|| type_ref.is_data_type();
		export = export || {
			let underlying_type = typedef.underlying_type_ref();
			underlying_type.as_function().is_some()
				|| !underlying_type.is_excluded()
				|| if let Some(templ) = underlying_type.as_template() {
				let cpp_fullname = templ.cpp_fullname();
				settings::IMPLEMENTED_GENERICS.contains(cpp_fullname.as_ref())
					|| settings::IMPLEMENTED_CONST_GENERICS.contains(cpp_fullname.as_ref())
			} else {
				false
			}
		};
		if export && !typedef.is_excluded() {
			typedef.dependent_types().into_iter()
				.for_each(|dep| {
					visitor.visit_dependent_type(dep)
				});
			visitor.visit_typedef(typedef)
		}
	}
}

impl<V: GeneratorVisitor> Drop for OpenCvWalker<'_, '_, V> {
	fn drop(&mut self) {
		if !self.comment_found {
			// some module level comments like "bioinspired" are not attached to anything and libclang
			// doesn't seem to offer a way to extract them, do it the hard way then
			let mut comment = String::with_capacity(2048);
			let module_path = self.opencv_module_header_dir.join(format!("{}.hpp", self.gen_env.module()));
			let f = BufReader::new(File::open(module_path).expect("Can't open main module file"));
			let mut found_module_comment = false;
			let mut defgroup_found = false;
			line_reader(f, |line| {
				if !found_module_comment && line.trim_start().starts_with("/**") {
					found_module_comment = true;
					defgroup_found = false;
				}
				if found_module_comment {
					if comment.contains("@defgroup") {
						defgroup_found = true;
					}
					comment.push_str(line);
					if line.trim_end().ends_with("*/") {
						if defgroup_found {
							return false;
						} else {
							comment.clear();
							found_module_comment = false;
						}
					}
				}
				true
			});
			if !defgroup_found {
				comment.clear();
			}
			if found_module_comment {
				self.visitor.visit_module_comment(comment);
			}
		}
	}
}

impl Generator {
	pub fn new(opencv_include_dir: &Path, additional_include_dirs: &[PathBuf], src_cpp_dir: &Path, clang: Clang) -> Self {
		let clang_bin = clang_sys::support::Clang::find(None, &[]).expect("Can't find clang binary");
		let mut clang_include_dirs = clang_bin.cpp_search_paths.unwrap_or_default();
		for additional_dir in additional_include_dirs {
			match canonicalize(additional_dir) {
				Ok(dir) => clang_include_dirs.push(dir),
				Err(err) => {
					eprintln!(
						"=== Cannot canonicalize one of the additional_include_dirs: {}, reason: {}",
						additional_dir.display(), err
					);
				}
			};
		}
		let mut opencv_module_header_dir = opencv_include_dir.join("opencv2.framework/Headers");
		if !opencv_module_header_dir.exists() {
			opencv_module_header_dir = opencv_include_dir.join("opencv2");
		}
		Self {
			clang_include_dirs,
			opencv_include_dir: canonicalize(opencv_include_dir).expect("Can't canonicalize opencv_include_dir"),
			opencv_module_header_dir: canonicalize(opencv_module_header_dir).expect("Can't canonicalize opencv_module_header_dir"),
			src_cpp_dir: canonicalize(src_cpp_dir).expect("Can't canonicalize src_cpp_dir"),
			clang,
		}
	}

	fn make_ephemeral_header(&self, contents: &str) -> Unsaved {
		Unsaved::new(self.src_cpp_dir.join("ocvrs_ephemeral.hpp"), contents)
	}

	fn handle_diags(diags: &[Diagnostic], panic_on_error: bool) {
		if !diags.is_empty() {
			let mut has_error = false;
			eprintln!("=== WARNING: {} diagnostic messages", diags.len());
			for diag in diags {
				if !has_error && matches!(diag.get_severity(), Severity::Error | Severity::Fatal) {
					has_error = true;
				}
				eprintln!("===    {}", diag);
			}
			if has_error && panic_on_error {
				panic!("=== Errors during header parsing");
			}
		}
	}

	pub fn build_clang_command_line_args(&self) -> Vec<Cow<'static, str>> {
		let mut args = self.clang_include_dirs.iter()
			.map(|d| format!("-isystem{}", d.to_str().expect("Incorrect system include path")).into())
			.chain([&self.opencv_include_dir, &self.src_cpp_dir].iter()
				.flat_map(|d| {
					let include_path = d.to_str().expect("Incorrect include path");
					[
						format!("-I{}", include_path).into(),
						format!("-F{}", include_path).into(),
					]
				})
			)
			.collect::<Vec<_>>();
		args.push("-DOCVRS_PARSING_HEADERS".into());
		args.push("-includeocvrs_ephemeral.hpp".into());
		// need to have c++14 here because VS headers contain features that require it
		args.push("-std=c++14".into());
		args
	}

	pub fn process_module(&self, module: &str, panic_on_error: bool, entity_processor: impl FnOnce(TranslationUnit, &str)) {
		let index = Index::new(&self.clang, true, false);
		let mut module_file = self.src_cpp_dir.join(format!("{}.hpp", module));
		if !module_file.exists() {
			module_file = self.opencv_module_header_dir.join(format!("{}.hpp", module));
		}
		let mut root_tu: TranslationUnit = index.parser(module_file)
			.unsaved(&[self.make_ephemeral_header("")])
			.arguments(&self.build_clang_command_line_args())
			.detailed_preprocessing_record(true)
			.skip_function_bodies(true)
			.parse().unwrap_or_else(|_| panic!("Cannot parse module: {}", module));
		let mut ephem_gen = EphemeralGenerator::new(module);
		let walker = EntityWalker::new(root_tu.get_entity());
		walker.walk_opencv_entities(&mut ephem_gen);
		let hdr = ephem_gen.generate_header();
		root_tu = root_tu.reparse(&[self.make_ephemeral_header(&hdr)]).expect("Can't reparse file");
		Self::handle_diags(&root_tu.get_diagnostics(), panic_on_error);
		entity_processor(root_tu, &hdr);
	}

	pub fn process_opencv_module(&self, module: &str, mut visitor: impl GeneratorVisitor) {
		self.process_module(module, true, |root_tu, ephemeral_header| {
			let root_entity = root_tu.get_entity();
			visitor.visit_ephemeral_header(ephemeral_header);
			let gen_env = GeneratorEnv::new(root_entity, module);
			let opencv_walker = OpenCvWalker::new(
				&self.opencv_module_header_dir,
				module,
				visitor,
				gen_env,
			);
			let walker = EntityWalker::new(root_entity);
			walker.walk_opencv_entities(opencv_walker);
		});
	}
}

pub fn is_ephemeral_header(path: &Path) -> bool {
	path.ends_with("ocvrs_ephemeral.hpp")
}

#[allow(unused)]
pub fn dbg_clang_type(type_ref: Type) {
	struct TypeWrapper<'tu>(Type<'tu>);

	impl fmt::Debug for TypeWrapper<'_> {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			f.debug_struct("Type")
				.field("kind", &self.0.get_kind())
				.field("display_name", &self.0.get_display_name())
				.field("alignof", &self.0.get_alignof())
				.field("sizeof", &self.0.get_sizeof())
				.field("address_space", &self.0.get_address_space())
				.field("argument_types", &self.0.get_argument_types())
				.field("calling_convention", &self.0.get_calling_convention())
				.field("canonical_type", &self.0.get_canonical_type())
				.field("class_type", &self.0.get_class_type())
				.field("declaration", &self.0.get_declaration())
				.field("elaborated_type", &self.0.get_elaborated_type())
				.field("element_type", &self.0.get_element_type())
				.field("exception_specification", &self.0.get_exception_specification())
				.field("fields", &self.0.get_fields())
//				.field("modified_type", &self.0.get_modified_type())
//				.field("nullability", &self.0.get_nullability())
				.field("pointee_type", &self.0.get_pointee_type())
				.field("ref_qualifier", &self.0.get_ref_qualifier())
				.field("result_type", &self.0.get_result_type())
				.field("size", &self.0.get_size())
				.field("template_argument_types", &self.0.get_template_argument_types())
				.field("typedef_name", &self.0.get_typedef_name())
				.field("is_const_qualified", &self.0.is_const_qualified())
				.field("is_elaborated", &self.0.is_elaborated())
				.field("is_pod", &self.0.is_pod())
				.field("is_restrict_qualified", &self.0.is_restrict_qualified())
				.field("is_transparent_tag", &self.0.is_transparent_tag())
				.field("is_variadic", &self.0.is_variadic())
				.field("is_volatile_qualified", &self.0.is_volatile_qualified())
				.field("is_integer", &self.0.is_integer())
				.field("is_signed_integer", &self.0.is_signed_integer())
				.field("is_unsigned_integer", &self.0.is_unsigned_integer())
				.finish()
		}
	}
	eprintln!("{:#?}", TypeWrapper(type_ref));
}

#[allow(unused)]
pub fn dbg_clang_entity(entity: Entity) {
	struct EntityWrapper<'tu>(Entity<'tu>);

	impl fmt::Debug for EntityWrapper<'_> {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			f.debug_struct("Entity")
				.field("evaluate", &self.0.evaluate())
				.field("kind", &self.0.get_kind())
				.field("display_name", &self.0.get_display_name())
				.field("location", &self.0.get_location())
				.field("range", &self.0.get_range())
				.field("accessibility", &self.0.get_accessibility())
				.field("arguments", &self.0.get_arguments())
				.field("availability", &self.0.get_availability())
				.field("bit_field_width", &self.0.get_bit_field_width())
				.field("canonical_entity", &self.0.get_canonical_entity())
				.field("comment", &self.0.get_comment())
				.field("parsed_comment", &self.0.get_parsed_comment())
				.field("comment_brief", &self.0.get_comment_brief())
				.field("comment_range", &self.0.get_comment_range())
				.field("completion_string", &self.0.get_completion_string())
				.field("children", &self.0.get_children())
				.field("definition", &self.0.get_definition())
				.field("enum_constant_value", &self.0.get_enum_constant_value())
				.field("enum_underlying_type", &self.0.get_enum_underlying_type())
				.field("exception_specification", &self.0.get_exception_specification())
				.field("external_symbol", &self.0.get_external_symbol())
				.field("file", &self.0.get_file())
				.field("language", &self.0.get_language())
				.field("lexical_parent", &self.0.get_lexical_parent())
				.field("linkage", &self.0.get_linkage())
				.field("mangled_name", &self.0.get_mangled_name())
				.field("mangled_names", &self.0.get_mangled_names())
				.field("module", &self.0.get_module())
				.field("name", &self.0.get_name())
				.field("name_ranges", &self.0.get_name_ranges())
				.field("offset_of_field", &self.0.get_offset_of_field())
				.field("overloaded_declarations", &self.0.get_overloaded_declarations())
				.field("overridden_methods", &self.0.get_overridden_methods())
				.field("platform_availability", &self.0.get_platform_availability())
				.field("reference", &self.0.get_reference())
				.field("semantic_parent", &self.0.get_semantic_parent())
				.field("storage_class", &self.0.get_storage_class())
				.field("template", &self.0.get_template())
				.field("template_arguments", &self.0.get_template_arguments())
				.field("template_kind", &self.0.get_template_kind())
				.field("tls_kind", &self.0.get_tls_kind())
				.field("translation_unit", &self.0.get_translation_unit())
				.field("type", &self.0.get_type())
				.field("typedef_underlying_type", &self.0.get_typedef_underlying_type())
				.field("usr", &self.0.get_usr())
				.field("visibility", &self.0.get_visibility())
				.field("result_type", &self.0.get_result_type())
				.field("has_attributes", &self.0.has_attributes())
				.field("is_abstract_record", &self.0.is_abstract_record())
				.field("is_anonymous", &self.0.is_anonymous())
				.field("is_bit_field", &self.0.is_bit_field())
				.field("is_builtin_macro", &self.0.is_builtin_macro())
				.field("is_const_method", &self.0.is_const_method())
				.field("is_converting_constructor", &self.0.is_converting_constructor())
				.field("is_copy_constructor", &self.0.is_copy_constructor())
				.field("is_default_constructor", &self.0.is_default_constructor())
				.field("is_defaulted", &self.0.is_defaulted())
				.field("is_definition", &self.0.is_definition())
				.field("is_dynamic_call", &self.0.is_dynamic_call())
				.field("is_function_like_macro", &self.0.is_function_like_macro())
				.field("is_inline_function", &self.0.is_inline_function())
//				.field("is_invalid_declaration", &self.0.is_invalid_declaration())
				.field("is_move_constructor", &self.0.is_move_constructor())
				.field("is_mutable", &self.0.is_mutable())
				.field("is_pure_virtual_method", &self.0.is_pure_virtual_method())
				.field("is_scoped", &self.0.is_scoped())
				.field("is_static_method", &self.0.is_static_method())
				.field("is_variadic", &self.0.is_variadic())
				.field("is_virtual_base", &self.0.is_virtual_base())
				.field("is_virtual_method", &self.0.is_virtual_method())
				.field("is_attribute", &self.0.is_attribute())
				.field("is_declaration", &self.0.is_declaration())
				.field("is_expression", &self.0.is_expression())
				.field("is_preprocessing", &self.0.is_preprocessing())
				.field("is_reference", &self.0.is_reference())
				.field("is_statement", &self.0.is_statement())
				.field("is_unexposed", &self.0.is_unexposed())
				.field("is_in_main_file", &self.0.is_in_main_file())
				.field("is_in_system_header", &self.0.is_in_system_header())
				.finish()
		}
	}
	eprintln!("{:#?}", EntityWrapper(entity));
}
