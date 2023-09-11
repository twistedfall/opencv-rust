use std::borrow::Cow;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use clang::diagnostic::{Diagnostic, Severity};
use clang::{Clang, Entity, EntityKind, Index};
use dunce::canonicalize;

use crate::entity::WalkAction;
use crate::type_ref::{CppNameStyle, FishStyle, TypeRef, TypeRefKind};
use crate::typedef::NewTypedefResult;
use crate::writer::rust_native::element::RustElement;
use crate::{
	get_definition_text, line_reader, settings, AbstractRefWrapper, Class, ClassSimplicity, Const, Element, EntityExt,
	EntityWalkerExt, EntityWalkerVisitor, Enum, Func, GeneratorEnv, LineReaderAction, SmartPtr, Tuple, Typedef, Vector,
};

#[derive(Debug)]
pub enum GeneratedType<'tu, 'ge> {
	AbstractRefWrapper(AbstractRefWrapper<'tu, 'ge>),
	Vector(Vector<'tu, 'ge>),
	SmartPtr(SmartPtr<'tu, 'ge>),
	Tuple(Tuple<'tu, 'ge>),
}

impl<'tu, 'ge> TryFrom<TypeRef<'tu, 'ge>> for GeneratedType<'tu, 'ge> {
	type Error = ();

	fn try_from(value: TypeRef<'tu, 'ge>) -> Result<Self, Self::Error> {
		match value.kind().into_owned() {
			TypeRefKind::StdVector(vec) => Ok(Self::Vector(vec)),
			TypeRefKind::StdTuple(tuple) => Ok(Self::Tuple(tuple)),
			TypeRefKind::SmartPtr(ptr) => Ok(Self::SmartPtr(ptr)),
			_ => Err(()),
		}
	}
}

/// Visitor of the different supported OpenCV entities, used in conjunction with [Generator]
#[allow(unused)]
pub trait GeneratorVisitor {
	/// Check whether the visitor is interested in entities from the specified file
	fn wants_file(&mut self, path: &Path) -> bool {
		true
	}

	/// Top-level module comment
	fn visit_module_comment(&mut self, comment: String) {}

	/// Top-level constant
	fn visit_const(&mut self, cnst: Const) {}

	/// Top-level enum
	fn visit_enum(&mut self, enm: Enum) {}

	/// Top-level function
	fn visit_func(&mut self, func: Func) {}

	/// Top-level type alias
	fn visit_typedef(&mut self, typedef: Typedef) {}

	/// Top-level class or an internal class of another class
	fn visit_class(&mut self, class: Class) {}

	/// Dependent generated type like `std::vector<Mat>` or `std::tuple<1, 2, 3>`
	fn visit_generated_type(&mut self, typ: GeneratedType) {}
}

/// Bridge between [EntityWalkerVisitor] and [GeneratorVisitor]
///
/// It takes [Entity]s supplied by the entity walker, extracts their export data (whether the entity should appear in bindings at
/// all or is internal) and calls the corresponding method in [GeneratorVisitor] based on their type. This is the 2nd pass of the
/// binding generation.
struct OpenCvWalker<'tu, 'r, V: GeneratorVisitor> {
	opencv_module_header_dir: &'r Path,
	visitor: V,
	gen_env: GeneratorEnv<'tu>,
}

impl<'tu, V: GeneratorVisitor> EntityWalkerVisitor<'tu> for OpenCvWalker<'tu, '_, V> {
	fn wants_file(&mut self, path: &Path) -> bool {
		self.visitor.wants_file(path) || path.ends_with("ocvrs_common.hpp")
	}

	fn visit_entity(&mut self, entity: Entity<'tu>) -> WalkAction {
		match entity.get_kind() {
			EntityKind::MacroDefinition => Self::process_const(&mut self.visitor, entity),
			EntityKind::MacroExpansion => {
				if let Some(name) = entity.get_name() {
					const BOXED: [&str; 6] = [
						"CV_EXPORTS",
						"CV_EXPORTS_W",
						"CV_WRAP",
						"GAPI_EXPORTS",
						"GAPI_EXPORTS_W",
						"GAPI_WRAP",
					];
					const SIMPLE: [&str; 3] = ["CV_EXPORTS_W_SIMPLE", "CV_EXPORTS_W_MAP", "GAPI_EXPORTS_W_SIMPLE"];
					const RENAME: [&str; 2] = ["CV_EXPORTS_AS", "CV_WRAP_AS"];
					if BOXED.contains(&name.as_str()) {
						self.gen_env.make_export_config(entity);
					} else if SIMPLE.contains(&name.as_str()) {
						self.gen_env.make_export_config(entity).simplicity = ClassSimplicity::Simple;
					} else if let Some(rename_macro) = RENAME.iter().find(|&r| r == &name) {
						if let Some(new_name) = get_definition_text(entity)
							.strip_prefix(&format!("{rename_macro}("))
							.and_then(|d| d.strip_suffix(')'))
						{
							self.gen_env.make_export_config(entity);
							self.gen_env.make_rename_config(entity).rename = new_name.trim().to_string();
						}
					} else if name == "CV_NORETURN" {
						self.gen_env.make_export_config(entity).no_return = true;
					} else if name == "CV_NOEXCEPT" {
						self.gen_env.make_export_config(entity).no_except = true;
					} else if name == "CV_DEPRECATED" || name == "CV_DEPRECATED_EXTERNAL" {
						self.gen_env.make_export_config(entity).deprecated = true;
					} else if name == "CV_NODISCARD_STD" || name == "CV_NODISCARD" {
						self.gen_env.make_export_config(entity).no_discard = true;
					} else if name == "OCVRS_ONLY_DEPENDENT_TYPES" {
						self.gen_env.make_export_config(entity).only_generated_types = true;
					}
				}
			}
			EntityKind::ClassDecl
			| EntityKind::ClassTemplate
			| EntityKind::ClassTemplatePartialSpecialization
			| EntityKind::StructDecl => Self::process_class(&mut self.visitor, &mut self.gen_env, entity),
			EntityKind::EnumDecl => Self::process_enum(&mut self.visitor, entity),
			EntityKind::FunctionDecl => Self::process_func(&mut self.visitor, &mut self.gen_env, entity),
			EntityKind::TypedefDecl | EntityKind::TypeAliasDecl => {
				Self::process_typedef(&mut self.visitor, &mut self.gen_env, entity)
			}
			EntityKind::VarDecl => {
				if !entity.is_mutable() {
					Self::process_const(&mut self.visitor, entity);
				} else {
					unreachable!("Unsupported VarDecl: {:#?}", entity)
				}
			}
			_ => {
				unreachable!("Unsupported entity: {:#?}", entity)
			}
		}
		WalkAction::Continue
	}
}

impl<'tu, 'r, V: GeneratorVisitor> OpenCvWalker<'tu, 'r, V> {
	pub fn new(opencv_module_header_dir: &'r Path, visitor: V, gen_env: GeneratorEnv<'tu>) -> Self {
		Self {
			opencv_module_header_dir,
			visitor,
			gen_env,
		}
	}

	fn process_const(visitor: &mut V, const_decl: Entity) {
		let cnst = Const::new(const_decl);
		if cnst.exclude_kind().is_included() {
			visitor.visit_const(cnst);
		}
	}

	fn process_class(visitor: &mut V, gen_env: &mut GeneratorEnv<'tu>, class_decl: Entity<'tu>) {
		if gen_env.get_export_config(class_decl).is_some() {
			let cls = Class::new(class_decl, gen_env);
			if cls.exclude_kind().is_included() {
				cls.generated_types().into_iter().for_each(|dep| {
					visitor.visit_generated_type(dep);
				});
				class_decl.walk_enums_while(|enm| {
					Self::process_enum(visitor, enm);
					WalkAction::Continue
				});
				class_decl.walk_classes_while(|sub_cls| {
					if !gen_env.get_export_config(sub_cls).is_some() {
						gen_env.make_export_config(sub_cls).simplicity = if Class::new(sub_cls, gen_env).can_be_simple() {
							ClassSimplicity::Simple
						} else {
							ClassSimplicity::Boxed
						};
					}
					Self::process_class(visitor, gen_env, sub_cls);
					WalkAction::Continue
				});
				class_decl.walk_typedefs_while(|tdef| {
					Self::process_typedef(visitor, gen_env, tdef);
					WalkAction::Continue
				});
				let cls = Class::new(class_decl, gen_env);
				if let Some(enm) = cls.as_enum() {
					visitor.visit_enum(enm);
				} else {
					visitor.visit_class(cls);
				}
			}
		}
	}

	fn process_enum(visitor: &mut V, enum_decl: Entity) {
		let enm = Enum::new(enum_decl);
		if enm.exclude_kind().is_included() {
			for cnst in enm.consts() {
				if cnst.exclude_kind().is_included() {
					visitor.visit_const(cnst);
				}
			}
			if !enm.is_anonymous() {
				visitor.visit_enum(enm);
			}
		}
	}

	fn process_func(visitor: &mut V, gen_env: &mut GeneratorEnv<'tu>, func_decl: Entity<'tu>) {
		if let Some(e) = gen_env.get_export_config(func_decl) {
			let func = Func::new(func_decl, gen_env);
			if func.exclude_kind().is_included() {
				let func_id = func.func_id().make_static();
				let mut processor = |spec| {
					let func = if e.only_generated_types {
						Func::new(func_decl, gen_env)
					} else {
						let mut name = Func::new(func_decl, gen_env).rust_leafname(FishStyle::No).into_owned().into();
						let mut rust_custom_leafname = None;
						if gen_env.func_names.make_unique_name(&mut name).is_changed() {
							rust_custom_leafname = Some(name.into());
						}
						Func::new_ext(func_decl, rust_custom_leafname, gen_env)
					};
					let func = if let Some(spec) = spec {
						func.specialize(spec)
					} else {
						func
					};
					func.generated_types().into_iter().for_each(|dep| {
						visitor.visit_generated_type(dep);
					});
					if !e.only_generated_types {
						visitor.visit_func(func);
					}
				};
				if let Some(specs) = settings::FUNC_SPECIALIZE.get(&func_id) {
					for spec in specs {
						processor(Some(spec));
					}
				} else {
					processor(None);
				}
			}
		}
	}

	fn process_typedef(visitor: &mut V, gen_env: &mut GeneratorEnv<'tu>, typedef_decl: Entity<'tu>) {
		let typedef = Typedef::try_new(typedef_decl, gen_env);
		if typedef.exclude_kind().is_included() {
			match typedef {
				NewTypedefResult::Typedef(typedef) => {
					let type_ref = typedef.type_ref();
					let export = gen_env.get_export_config(typedef_decl).is_some()
						// we need to have a typedef even if it's not exported for e.g. cv::Size
						|| type_ref.is_data_type()
						|| {
						let underlying_type = typedef.underlying_type_ref();
						underlying_type.as_function().is_some()
							|| !underlying_type.exclude_kind().is_ignored()
							|| underlying_type.template_kind().as_template_specialization().map_or(false, |templ| {
							settings::IMPLEMENTED_GENERICS.contains(templ.cpp_name(CppNameStyle::Reference).as_ref())
						})
					};

					if export {
						typedef
							.generated_types()
							.into_iter()
							.for_each(|dep| visitor.visit_generated_type(dep));
						visitor.visit_typedef(typedef)
					}
				}
				NewTypedefResult::Class(_) | NewTypedefResult::Enum(_) => {
					// don't generate those because libclang will also emit normal classes and enums too
				}
			}
		}
	}
}

impl<V: GeneratorVisitor> Drop for OpenCvWalker<'_, '_, V> {
	fn drop(&mut self) {
		// Some module level comments like "bioinspired" are not attached to anything and libclang
		// doesn't seem to offer a way to extract them, do it the hard way then.
		// That's actually the case for all modules starting with OpenCV 4.8.0 so this is now a single
		// method of extracting comments
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
						return LineReaderAction::Break;
					} else {
						comment.clear();
						found_module_comment = false;
					}
				}
			}
			LineReaderAction::Continue
		});
		if found_module_comment {
			self.visitor.visit_module_comment(comment);
		}
		for inject_func_fact in settings::FUNC_INJECT_MANUAL.get(self.gen_env.module()).into_iter().flatten() {
			let inject_func: Func = inject_func_fact();
			if !inject_func.kind().as_class_method().is_some() {
				self.visitor.visit_func(inject_func);
			}
		}
		if let Some(tweaks) = settings::GENERATOR_MODULE_TWEAKS.get(self.gen_env.module()) {
			for generated in &tweaks.generate_types {
				if let Ok(generated) = GeneratedType::try_from(generated()) {
					self.visitor.visit_generated_type(generated);
				}
			}
		}
	}
}

/// Main workhorse for generating OpenCV bindings for a specific module
///
/// Full binding generation for a module is happening in the following major phases:
/// 1. Headers are parsed with `libclang`
/// 2. [crate::generator_env::GeneratorEnvPopulator] collects the data necessary in the binding generation (1st pass)
/// 3. Binding entities are extracted using the data from step 2 (2nd pass)
/// 4. Specific source files are generated by [crate::writer::RustNativeBindingWriter] (at the moment)
#[derive(Debug)]
pub struct Generator {
	clang_include_dirs: Vec<PathBuf>,
	opencv_include_dir: PathBuf,
	opencv_module_header_dir: PathBuf,
	src_cpp_dir: PathBuf,
	clang: Clang,
}

impl Generator {
	pub fn new(opencv_include_dir: &Path, additional_include_dirs: &[&Path], src_cpp_dir: &Path) -> Self {
		let clang_bin = clang_sys::support::Clang::find(None, &[]).expect("Can't find clang binary");
		let mut clang_include_dirs = clang_bin.cpp_search_paths.unwrap_or_default();
		for additional_dir in additional_include_dirs {
			match canonicalize(additional_dir) {
				Ok(dir) => clang_include_dirs.push(dir),
				Err(err) => {
					eprintln!(
						"=== Cannot canonicalize one of the additional_include_dirs: {}, reason: {}",
						additional_dir.display(),
						err
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
			clang: Clang::new().expect("Can't initialize clang"),
		}
	}

	fn handle_diags(diags: &[Diagnostic], panic_on_error: bool) {
		if !diags.is_empty() {
			let mut has_error = false;
			eprintln!("=== WARNING: {} diagnostic messages", diags.len());
			for diag in diags {
				if !has_error && matches!(diag.get_severity(), Severity::Error | Severity::Fatal) {
					has_error = true;
				}
				eprintln!("===    {diag}");
			}
			if has_error && panic_on_error {
				panic!("=== Errors during header parsing");
			}
		}
	}

	pub fn clang_version(&self) -> String {
		clang::get_version()
	}

	pub fn build_clang_command_line_args(&self) -> Vec<Cow<'static, str>> {
		let mut args = self
			.clang_include_dirs
			.iter()
			.map(|d| format!("-isystem{}", d.to_str().expect("Incorrect system include path")).into())
			.chain([&self.opencv_include_dir, &self.src_cpp_dir].iter().flat_map(|d| {
				let include_path = d.to_str().expect("Incorrect include path");
				[format!("-I{include_path}").into(), format!("-F{include_path}").into()]
			}))
			.collect::<Vec<_>>();
		args.push("-DOCVRS_PARSING_HEADERS".into());
		args.push("-includeocvrs_common.hpp".into());
		// need to have c++14 here because VS headers contain features that require it
		args.push("-std=c++14".into());
		args
	}

	/// Runs the clang header parsing, check for the compilation errors and hands off to `entity_processor`
	pub fn pre_process(&self, module: &str, panic_on_error: bool, entity_processor: impl FnOnce(Entity)) {
		let index = Index::new(&self.clang, true, false);
		let mut module_file = self.src_cpp_dir.join(format!("{module}.hpp"));
		if !module_file.exists() {
			module_file = self.opencv_module_header_dir.join(format!("{module}.hpp"));
		}
		let root_tu = index
			.parser(module_file)
			.arguments(&self.build_clang_command_line_args())
			.detailed_preprocessing_record(true)
			.skip_function_bodies(true)
			.parse()
			.unwrap_or_else(|_| panic!("Cannot parse module: {module}"));
		Self::handle_diags(&root_tu.get_diagnostics(), panic_on_error);
		entity_processor(root_tu.get_entity());
	}

	/// Runs the full binding generation process using the supplied `visitor`
	pub fn generate(&self, module: &str, visitor: impl GeneratorVisitor) {
		self.pre_process(module, true, |root_entity| {
			let gen_env = GeneratorEnv::new(root_entity, module);
			let opencv_walker = OpenCvWalker::new(&self.opencv_module_header_dir, visitor, gen_env);
			root_entity.walk_opencv_entities(opencv_walker);
		});
	}
}
