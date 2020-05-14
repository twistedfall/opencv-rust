use std::{
	borrow::Cow,
	fmt,
	fs::File,
	io::BufReader,
	path::{Path, PathBuf},
};

use clang::{
	Clang,
	Entity,
	EntityKind,
	Index,
	Type,
};
use dunce::canonicalize;

use crate::{
	Class,
	Const,
	Element,
	EntityExt,
	EntityWalker,
	EntityWalkerVisitor,
	Enum,
	Func,
	FunctionTypeHint,
	GeneratedElement,
	GeneratorEnv,
	get_definition_text,
	line_reader,
	settings,
	Typedef,
};

#[allow(unused)]
pub trait GeneratorVisitor {
	fn wants_file(&mut self, path: &Path) -> bool { true }

	fn visit_module_comment(&mut self, comment: String) {}
	fn visit_const(&mut self, cnst: Const) {}
	fn visit_enum(&mut self, enm: Enum) {}
	fn visit_func(&mut self, func: Func) {}
	fn visit_typedef(&mut self, typedef: Typedef) {}
	fn visit_class(&mut self, class: Class) {}
	fn visit_dependent_type(&mut self, typ: &dyn GeneratedElement) {}
}

#[derive(Debug)]
pub struct Generator<'m, 'c> {
	clang_include_dirs: Vec<PathBuf>,
	opencv_include_dir: PathBuf,
	src_cpp_dir: PathBuf,
	module: &'m str,
	clang: &'c Clang,
}

struct OpenCVWalker<'tu, V: GeneratorVisitor> {
	opencv_include_dir: &'tu Path,
	visitor: V,
	gen_env: GeneratorEnv<'tu>,
	comment_found: bool,
}

impl<'tu, V: GeneratorVisitor> EntityWalkerVisitor<'tu> for OpenCVWalker<'tu, V> {
	fn wants_file(&mut self, path: &Path) -> bool {
		self.visitor.wants_file(path) || path.ends_with("common.hpp")
	}

	fn visit_entity(&mut self, entity: Entity<'tu>) -> bool {
		match entity.get_kind() {
			EntityKind::Namespace => {
				if !self.comment_found {
					if let Some(c) = entity.get_comment() {
						if c.contains("@defgroup") {
							self.comment_found = true;
							self.visitor.visit_module_comment(c)
						}
					}
				}
			}
			EntityKind::MacroDefinition => {
				self.process_const(entity)
			}
			EntityKind::MacroExpansion => {
				if let Some(name) = entity.get_name() {
					if name == "CV_EXPORTS" || name == "CV_EXPORTS_W" || name == "CV_WRAP" {
						self.gen_env.make_export_config(entity);
					} else if name == "CV_EXPORTS_W_SIMPLE" || name == "CV_EXPORTS_W_MAP" {
						self.gen_env.make_export_config(entity).simple = true;
					} else if name == "CV_EXPORTS_AS" || name == "CV_WRAP_AS" {
						let definition = get_definition_text(entity);
						const CV_EXPORTS_AS: &str = "CV_EXPORTS_AS(";
						const CV_WRAP_AS: &str = "CV_WRAP_AS(";
						let definition = if definition.starts_with(CV_EXPORTS_AS) && definition.ends_with(')') {
							definition[CV_EXPORTS_AS.len()..definition.len() - 1].trim()
						} else if definition.starts_with(CV_WRAP_AS) && definition.ends_with(')') {
							definition[CV_WRAP_AS.len()..definition.len() - 1].trim()
						} else {
							unreachable!("Incorrect CV_EXPORTS_AS(..) or CV_WRAP_AS(..) usage")
						};
						self.gen_env.make_export_config(entity).rename = Some(definition.to_string());
					} else if name == "CV_NORETURN" {
						self.gen_env.make_export_config(entity).no_return = true;
					} else if name == "CV_NOEXCEPT" {
						self.gen_env.make_export_config(entity).no_except = true;
					} else if name == "CV_DEPRECATED" || name == "CV_DEPRECATED_EXTERNAL" {
						self.gen_env.make_export_config(entity).deprecated = true;
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
			EntityKind::TypedefDecl => {
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

impl<'tu, V: GeneratorVisitor> OpenCVWalker<'tu, V> {
	pub fn new(opencv_include_dir: &'tu Path, visitor: V, gen_env: GeneratorEnv<'tu>) -> Self {
		Self { opencv_include_dir, visitor, gen_env, comment_found: false }
	}

	fn process_const(&mut self, const_decl: Entity) {
		let cnst = Const::new(const_decl);
		if !cnst.is_excluded() {
			self.visitor.visit_const(cnst);
		}
	}

	fn process_class(visitor: &mut impl GeneratorVisitor, gen_env: &mut GeneratorEnv<'tu>, class_decl: Entity<'tu>) {
		if gen_env.get_export_config(class_decl).is_some() {
			let cls = Class::new(class_decl, gen_env);
			if !cls.is_excluded() {
				cls.dependent_types().into_iter()
					.for_each(|dep| {
						visitor.visit_dependent_type(dep.as_ref());
					});
				class_decl.walk_enums_while(|enm| {
					let enm = Enum::new(enm);
					if enm.rust_leafname() != "unnamed" {
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

	fn process_enum(visitor: &mut impl GeneratorVisitor, enum_decl: Entity) {
		let enm = Enum::new(enum_decl);
		if !enm.is_excluded() {
			for cnst in enm.consts() {
				if !cnst.is_excluded() {
					visitor.visit_const(cnst);
				}
			}
			if enm.rust_leafname() != "unnamed" {
				visitor.visit_enum(enm);
			}
		}
	}

	fn process_func(visitor: &mut impl GeneratorVisitor, gen_env: &mut GeneratorEnv<'tu>, func_decl: Entity<'tu>) {
		if let Some(export_config) = gen_env.get_export_config(func_decl) {
			let only_dependent_types = export_config.only_dependent_types;
			let func = Func::new(func_decl, gen_env);
			if !func.is_excluded() {
				let specs = settings::FUNC_SPECIALIZE.get(func.identifier().as_ref())
					.map_or_else(|| vec![FunctionTypeHint::None], |specs| specs.iter()
						.map(|s| FunctionTypeHint::Specialized(s))
						.collect::<Vec<_>>(),
					);
				for type_hint in specs {
					let func = Func::new_ext(func_decl, type_hint, None, gen_env);
					let name = func.rust_leafname().into_owned().into();
					let name = if !only_dependent_types {
						gen_env.func_names.get_name(name)
					} else {
						name
					};
					// we need to stop holding &gen_env for a while to be able to mutate it above
					let func = Func::new_ext(func_decl, type_hint, Some(name.as_ref()), gen_env);
					func.dependent_types().into_iter()
						.for_each(|dep| {
							visitor.visit_dependent_type(dep.as_ref());
						});
					if !only_dependent_types {
						visitor.visit_func(func);
					}
				}
			}
		}
	}

	fn process_typedef(visitor: &mut impl GeneratorVisitor, gen_env: &mut GeneratorEnv<'tu>, typedef_decl: Entity<'tu>) {
		let typedef = Typedef::new(typedef_decl, gen_env);
		let mut export = gen_env.get_export_config(typedef_decl).is_some()
			// we need to have a typedef even if it's not exported for e.g. cv::Size
			|| typedef.type_ref().is_data_type();
		export = export || {
			let underlying_type = typedef.underlying_type_ref();
			underlying_type.as_function().is_some()
				|| !underlying_type.is_excluded()
				|| if let Some(templ) = underlying_type.as_template() {
				settings::IMPLEMENTED_GENERICS.contains(templ.cpp_fullname().as_ref())
			} else {
				false
			}
		};
		if export && !typedef.is_excluded() {
			typedef.dependent_types().into_iter()
				.for_each(|dep| {
					visitor.visit_dependent_type(dep.as_ref())
				});
			visitor.visit_typedef(typedef)
		}
	}
}

impl<V: GeneratorVisitor> Drop for OpenCVWalker<'_, V> {
	fn drop(&mut self) {
		if !self.comment_found {
			// some module level comments like "bioinspired" are not attached to anything and libclang
			// doesn't seem to offer a way to extract them, do it the hard way then
			let mut module_path = self.opencv_include_dir.join("opencv2");
			module_path.push(self.gen_env.module());
			module_path.set_extension("hpp");
			let mut comment = String::with_capacity(2048);
			let f = BufReader::new(File::open(module_path).expect("Can't open main module file"));
			let mut found_module_comment = false;
			let mut defgroup_found = false;
			line_reader(f, |line| {
				if !found_module_comment {
					if line.trim_start().starts_with("/**") {
						found_module_comment = true;
						defgroup_found = false;
					}
				}
				if found_module_comment {
					if comment.contains("@defgroup") {
						defgroup_found = true;
					}
					comment.push_str(&line);
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

impl<'m, 'c> Generator<'m, 'c> {
	pub fn new(opencv_include_dir: &Path, src_cpp_dir: &Path, module: &'m str, clang: &'c Clang) -> Self {
		let clang_bin = clang_sys::support::Clang::find(None, &[]).expect("Can't find clang binary");
		Self {
			clang_include_dirs: clang_bin.cpp_search_paths.unwrap_or_default(),
			opencv_include_dir: canonicalize(opencv_include_dir).expect("Can't canonicalize opencv_include_dir"),
			src_cpp_dir: canonicalize(src_cpp_dir).expect("Can't canonicalize src_cpp_dir"),
			module,
			clang,
		}
	}

	pub fn build_clang_command_line_args(&self) -> Vec<Cow<'static, str>> {
		let mut args = self.clang_include_dirs.iter()
			.map(|d| format!("-isystem{}", d.to_str().expect("Incorrect system include path")).into())
			.chain([&self.opencv_include_dir, &self.src_cpp_dir].iter()
				.map(|d| format!("-I{}", d.to_str().expect("Incorrect include path")).into())
			)
			.collect::<Vec<_>>();
		args.push("-DOCVRS_PARSING_HEADERS".into());
		args.push("-includeocvrs_resolve_types.hpp".into());
		args
	}

	pub fn process_file<V: GeneratorVisitor>(&self, file: &Path, visitor: V) {
		let index = Index::new(&self.clang, true, false);
		let top = index.parser(canonicalize(file).expect("Can't canonicalize file"))
			.arguments(&self.build_clang_command_line_args())
			.detailed_preprocessing_record(true)
			.skip_function_bodies(true)
			.parse().expect("Cannot parse");
		let top_entity = top.get_entity();
		let opencv_walker = OpenCVWalker::new(
			&self.opencv_include_dir,
			visitor,
			GeneratorEnv::new(top_entity, self.module),
		);
		let walker = EntityWalker::new(top_entity);
		walker.walk_opencv_entities(opencv_walker);
	}
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
