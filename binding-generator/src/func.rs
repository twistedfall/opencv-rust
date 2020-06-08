use std::{
	borrow::Cow,
	collections::HashMap,
	fmt,
};

use clang::{Availability, Entity, EntityKind};
use maplit::hashmap;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{
	Class,
	comment,
	CompiledInterpolation,
	Constness,
	DefaultElement,
	DefinitionLocation,
	DependentTypeMode,
	Element,
	EntityElement,
	EntityExt,
	Field,
	FieldTypeHint,
	GeneratedElement,
	GeneratorEnv,
	get_debug,
	IteratorExt,
	NamePool,
	reserved_rename,
	settings::{self, SliceHint},
	StrExt,
	StringExt,
	type_ref::Kind as TypeRefKind,
	TypeRef,
};

#[derive(Debug, PartialEq)]
pub enum OperatorKind {
	Unsupported,
	Index,
	Add,
	Sub,
	Mul,
	Div,
	Deref,
}

impl OperatorKind {
	pub fn new(token: &str, arg_count: usize) -> Self {
		match token.trim() {
			"[]" => {
				OperatorKind::Index
			}
			"+" => {
				OperatorKind::Add
			}
			"-" => {
				OperatorKind::Sub
			}
			"*" => {
				if arg_count == 0 {
					OperatorKind::Deref
				} else {
					OperatorKind::Mul
				}
			}
			"/" => {
				OperatorKind::Div
			}
			_ => {
				OperatorKind::Unsupported
			},
		}
	}
}

#[derive(Debug)]
enum Kind<'tu, 'g> {
	Function,
	FunctionOperator(OperatorKind),
	Constructor(Class<'tu, 'g>),
	InstanceMethod(Class<'tu, 'g>),
	StaticMethod(Class<'tu, 'g>),
	FieldAccessor(Class<'tu, 'g>),
	ConversionMethod(Class<'tu, 'g>),
	InstanceOperator(Class<'tu, 'g>, OperatorKind),
	GenericFunction,
	GenericInstanceMethod(Class<'tu, 'g>),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FunctionTypeHint {
	None,
	FieldSetter,
	Specialized(&'static HashMap<&'static str, &'static str>),
}

#[derive(Clone)]
pub struct Func<'tu, 'g> {
	entity: Entity<'tu>,
	type_hint: FunctionTypeHint,
	name_hint: Option<&'g str>,
	gen_env: &'g GeneratorEnv<'tu>,
}

impl<'tu, 'g> Func<'tu, 'g> {
	pub fn new(entity: Entity<'tu>, gen_env: &'g GeneratorEnv<'tu>) -> Self {
		Self { entity, type_hint: FunctionTypeHint::None, name_hint: None, gen_env }
	}

	pub fn new_ext(entity: Entity<'tu>, type_hint: FunctionTypeHint, name_hint: Option<&'g str>, gen_env: &'g GeneratorEnv<'tu>) -> Self {
		Self { entity, type_hint, name_hint, gen_env }
	}

	pub fn rust_generate_funcs(fns: impl IntoIterator<Item=&'g Func<'tu, 'g>>, opencv_version: &str) -> String where 'tu: 'g {
		let fns = fns.into_iter()
			.filter(|f| !f.is_excluded());
		Self::rust_disambiguate_names(fns)
			.map(|(name, func)| func.gen_rust_with_name(&name, opencv_version))
			.join("")
	}

	pub fn rust_disambiguate_names(fns: impl IntoIterator<Item=&'g Func<'tu, 'g>>) -> impl Iterator<Item=(String, &'g Func<'tu, 'g>)> where 'tu: 'g {
		let args = fns.into_iter();
		NamePool::with_capacity(args.size_hint().1.unwrap_or_default())
			.into_disambiguator(args, |f| f.rust_leafname())
	}

	fn kind(&self) -> Kind<'tu, 'g> {
		const OPERATOR: &str = "operator";
		match self.entity.get_kind() {
			EntityKind::FunctionDecl => {
				if let Some(operator) = self.entity.cpp_localname().strip_str_prefix(OPERATOR) {
					let arg_count = self.entity.get_arguments().map_or(0, |v| v.len());
					Kind::FunctionOperator(OperatorKind::new(operator.trim(), arg_count))
				} else {
					Kind::Function
				}
			}
			EntityKind::Constructor => {
				Kind::Constructor(Class::new(
					self.entity.get_semantic_parent().expect("Can't get parent of constructor"),
					self.gen_env,
				))
			}
			EntityKind::Method => {
				let class = Class::new(
					self.entity.get_semantic_parent().expect("Can't get parent of method"),
					self.gen_env,
				);
				if self.entity.is_static_method() {
					Kind::StaticMethod(class)
				} else {
					if let Some(operator) = self.entity.cpp_localname().strip_str_prefix(OPERATOR) {
						let arg_count = self.entity.get_arguments().map_or(0, |v| v.len());
						Kind::InstanceOperator(class, OperatorKind::new(operator.trim(), arg_count))
					} else {
						Kind::InstanceMethod(class)
					}
				}
			}
			EntityKind::FieldDecl | EntityKind::VarDecl => {
				Kind::FieldAccessor(Field::new(self.entity, self.gen_env).parent())
			}
			EntityKind::ConversionFunction => {
				Kind::ConversionMethod(Class::new(
					self.entity.get_semantic_parent().expect("Can't get parent of method"),
					self.gen_env,
				))
			}
			EntityKind::FunctionTemplate => {
				match self.entity.get_template_kind() {
					Some(EntityKind::Method) => {
						Kind::GenericInstanceMethod(Class::new(
							self.entity.get_semantic_parent().expect("Can't get parent of generic method"),
							self.gen_env,
						))
					}
					_ => {
						Kind::GenericFunction
					}
				}
			}
			_ => unreachable!("Unknown function entity: {:#?}", self.entity)
		}
	}

	pub fn as_constructor(&self) -> Option<Class<'tu, 'g>> {
		if let Kind::Constructor(out) = self.kind() {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_instance_method(&self) -> Option<Class<'tu, 'g>> {
		match self.kind() {
			Kind::InstanceMethod(out) | Kind::FieldAccessor(out) | Kind::GenericInstanceMethod(out)
			| Kind::ConversionMethod(out) | Kind::InstanceOperator(out, ..) => {
				Some(out)
			}
			_ => {
				None
			}
		}
	}

	pub fn as_static_method(&self) -> Option<Class<'tu, 'g>> {
		if let Kind::StaticMethod(out) = self.kind() {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_field_accessor(&self) -> Option<Field<'tu, 'g>> {
		if let Kind::FieldAccessor(..) = self.kind() {
			Some(Field::new(self.entity, self.gen_env))
		} else {
			None
		}
	}

	pub fn as_field_setter(&self) -> Option<Field<'tu, 'g>> {
		if self.as_field_accessor().is_some() && self.type_hint == FunctionTypeHint::FieldSetter {
			Some(Field::new_ext(self.entity, FieldTypeHint::FieldSetter, self.gen_env))
		} else {
			None
		}
	}

	pub fn as_conversion_method(&self) -> Option<Class<'tu, 'g>> {
		if let Kind::ConversionMethod(out) = self.kind() {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_operator(&self) -> Option<(Option<Class<'tu, 'g>>, OperatorKind)> {
		match self.kind() {
			Kind::FunctionOperator(kind) => {
				Some((None, kind))
			}
			Kind::InstanceOperator(cls, kind) => {
				Some((Some(cls), kind))
			}
			_ => {
				None
			}
		}
	}

	pub fn is_const(&self) -> bool {
		if settings::FORCE_CONSTANT_METHOD.contains(self.cpp_fullname().as_ref()) {
			true
		}
		else if let Some(fld) = self.as_field_accessor() {
			self.type_hint != FunctionTypeHint::FieldSetter && {
				let type_ref = fld.type_ref();
				type_ref.is_const() || type_ref.is_copy() || type_ref.is_cv_string() || type_ref.is_std_string()
			}
		} else {
			self.entity.is_const_method()
		}
	}

	pub fn is_abstract(&self) -> bool {
		self.entity.is_pure_virtual_method()
	}

	pub fn is_generic(&self) -> bool {
		match self.kind() {
			Kind::GenericFunction | Kind::GenericInstanceMethod(..) => {
				!self.as_specialized().is_some()
			}
			Kind::Function | Kind::Constructor(..) | Kind::InstanceMethod(..)
			| Kind::StaticMethod(..) | Kind::FieldAccessor(..) | Kind::ConversionMethod(..)
			| Kind::FunctionOperator(..) | Kind::InstanceOperator(..) => {
				false
			}
		}
	}

	fn is_infallible(&self) -> bool {
		self.as_field_accessor().is_some()
			|| self.gen_env.get_export_config(self.entity).map_or(false, |e| e.no_except)
	}

	pub fn as_specialized(&self) -> Option<&'static HashMap<&'static str, &'static str>> {
		if let FunctionTypeHint::Specialized(spec) = self.type_hint {
			Some(spec)
		} else {
			None
		}
	}

	pub fn return_type(&self) -> TypeRef<'tu, 'g> {
		match self.kind() {
			Kind::Constructor(cls) => {
				cls.type_ref()
			}
			Kind::Function | Kind::InstanceMethod(..) | Kind::StaticMethod(..)
			| Kind::ConversionMethod(..) | Kind::GenericInstanceMethod(..) | Kind::GenericFunction
			| Kind::FunctionOperator(..) | Kind::InstanceOperator(..) => {
				let out = TypeRef::new(
					self.entity.get_result_type().expect("Can't get return type"),
					self.gen_env,
				);
				let mut out = if let TypeRefKind::Reference(type_ref) = out.canonical().kind() {
					type_ref
				} else {
					out
				};
				if let Some(spec) = self.as_specialized() {
					if out.is_generic() {
						let spec_type = spec.get(out.base().cpp_full().as_ref())
							.and_then(|s| self.gen_env.resolve_type(s));
						if let Some(spec_type) = spec_type {
							out.specialize(spec_type);
						}
					}
				}
				out
			}
			Kind::FieldAccessor(..) => {
				if self.type_hint == FunctionTypeHint::FieldSetter {
					TypeRef::new(self.gen_env.resolve_type("void").expect("Can't resolve void type"), self.gen_env)
				} else {
					Field::new(self.entity, self.gen_env).type_ref()
				}
			}
		}
	}

	pub fn arguments(&self) -> Vec<Field<'tu, 'g>> {
		let args = match self.kind() {
			Kind::GenericFunction | Kind::GenericInstanceMethod(..) => {
				let mut out = vec![];
				self.entity.walk_children_while(|child| {
					if child.get_kind() == EntityKind::ParmDecl {
						out.push(child);
					}
					true
				});
				out
			}
			Kind::FieldAccessor(..) => {
				if self.type_hint == FunctionTypeHint::FieldSetter {
					vec![self.entity]
				} else {
					vec![]
				}
			}
			_ => {
				self.entity.get_arguments().expect("Can't get arguments")
			}
		};

		let empty_hashmap = HashMap::new();
		let spec = if let Some(spec) = self.as_specialized() {
			spec
		} else {
			&empty_hashmap
		};
		let args_len = args.len();
		let func_name = self.cpp_fullname();
		let is_field_setter = self.as_field_setter().is_some();

		args.into_iter()
			.map(|a| {
				if is_field_setter {
					return Field::new_ext(a, FieldTypeHint::FieldSetter, self.gen_env)
				}

				if let Some(slice_arg) = settings::SLICE_ARGUMENT.get(&(func_name.as_ref(), args_len)) {
					match slice_arg {
						&SliceHint::ForceSlice(arg) => {
							if arg == a.rust_leafname().as_ref() {
								return Field::new_ext(a, FieldTypeHint::Slice, self.gen_env)
							}
						},
						&SliceHint::ConvertSlice(ptr_arg, len_arg, len_div) => {
							let arg_name = a.rust_leafname();
							if ptr_arg == arg_name.as_ref() {
								return Field::new_ext(a, FieldTypeHint::Slice, self.gen_env)
							} else if len_arg == arg_name.as_ref() {
								return Field::new_ext(a, FieldTypeHint::SliceLen(ptr_arg, len_div), self.gen_env)
							}
						}
					}
				}

				let out = Field::new(a, self.gen_env);
				let type_ref = out.type_ref();
				if type_ref.is_generic() {
					let spec_type = spec.get(type_ref.base().cpp_full().as_ref())
						.and_then(|s| self.gen_env.resolve_type(s));
					if let Some(spec_type) = spec_type {
						return Field::new_ext(a, FieldTypeHint::Specialized(spec_type), self.gen_env);
					}
				}
				out
			})
			.collect()
	}

	pub fn dependent_types(&self) -> Vec<Box<dyn GeneratedElement + 'g>> {
		self.arguments().into_iter()
			.map(|a| a.type_ref())
			.filter(|t| !t.is_ignored())
			.map(|t| t.dependent_types())
			.flatten()
			.chain(self.return_type().dependent_types_with_mode(DependentTypeMode::ForReturn(DefinitionLocation::Module)))
			.collect()
	}

	fn cpp_call_invoke(&self) -> String {
		static VOID_TPL: Lazy<CompiledInterpolation> = Lazy::new(||
			"{{name}}({{args}});".compile_interpolation()
		);

		static NORMAL_TPL: Lazy<CompiledInterpolation> = Lazy::new(||
			"{{ret_type}} = {{doref}}{{name}}{{generic}}({{args}});".compile_interpolation()
		);

		static FIELD_READ_TPL: Lazy<CompiledInterpolation> = Lazy::new(||
			"{{ret_type}} = {{doref}}{{name}};".compile_interpolation()
		);

		static FIELD_WRITE_TPL: Lazy<CompiledInterpolation> = Lazy::new(||
			"{{name}} = {{args}};".compile_interpolation()
		);

		static CONSTRUCTOR_TPL: Lazy<CompiledInterpolation> = Lazy::new(||
			"{{ret_type}} ret({{args}});".compile_interpolation()
		);

		static CONSTRUCTOR_NO_ARGS_TPL: Lazy<CompiledInterpolation> = Lazy::new(||
			"{{ret_type}} ret;".compile_interpolation()
		);

		static BOXED_CONSTRUCTOR_TPL: Lazy<CompiledInterpolation> = Lazy::new(||
			"{{ret_type}}* ret = new {{ret_type}}({{args}});".compile_interpolation()
		);

		let call_name = match self.kind() {
			Kind::Function | Kind::GenericFunction | Kind::StaticMethod(..)
			| Kind::FunctionOperator(..) => {
				self.cpp_fullname()
			}
			Kind::Constructor(class) => {
				class.cpp_fullname().into_owned().into()
			}
			Kind::FieldAccessor(class) if self.type_hint == FunctionTypeHint::FieldSetter => {
				class.cpp_method_call_name(DefaultElement::cpp_localname(self).as_ref()).into()
			}
			Kind::InstanceMethod(class) | Kind::FieldAccessor(class) | Kind::GenericInstanceMethod(class)
			| Kind::ConversionMethod(class) | Kind::InstanceOperator(class, ..) => {
				class.cpp_method_call_name(self.cpp_localname().as_ref()).into()
			}
		};

		let mut generic = String::new();
		if let Some(spec) = self.as_specialized() {
			generic.reserve(64);
			generic.push('<');
			generic.extend_join(spec.values(), ", ");
			generic.push('>');
		}

		let args = Field::cpp_disambiguate_names(self.arguments())
			.map(|(name, arg)| arg.type_ref().cpp_arg_func_call(name).into_owned())
			.join(", ");

		let return_type = self.return_type();
		let tpl = if let Some(cls) = self.as_constructor() {
			if cls.is_by_ptr() {
				&BOXED_CONSTRUCTOR_TPL
			} else if args.is_empty() {
				&CONSTRUCTOR_NO_ARGS_TPL
			} else {
				&CONSTRUCTOR_TPL
			}
		} else if let Kind::FieldAccessor(..) = self.kind() {
			if self.type_hint == FunctionTypeHint::FieldSetter {
				&FIELD_WRITE_TPL
			} else {
				&FIELD_READ_TPL
			}
		} else if return_type.is_void() {
			&VOID_TPL
		} else {
			&NORMAL_TPL
		};
		let ret_type = if self.as_constructor().is_some() {
			return_type.cpp_full()
		} else {
			return_type.cpp_full_with_name("ret")
		};
		let doref = if return_type.as_fixed_array().is_some() {
			"&"
		} else {
			""
		};
		tpl.interpolate(&hashmap! {
			"ret_type" => ret_type,
			"doref" => doref.into(),
			"name" => call_name,
			"generic" => generic.into(),
			"args" => args.into(),
		})
	}

	fn cpp_method_return(&self) -> Cow<str> {
		let return_type = self.return_type();
		if return_type.is_void() {
			"return Ok();".into()
		} else if return_type.is_by_ptr() && !self.as_constructor().is_some() {
			let out = return_type.source()
				.as_class()
				.and_then(|cls| if cls.is_abstract() {
					Some(Cow::Borrowed("return Ok(ret);"))
				} else {
					None
				});
			out.unwrap_or_else(|| format!("return Ok(new {typ}(ret));", typ=return_type.cpp_full()).into())
		} else if return_type.is_cv_string() || return_type.is_std_string() {
			"return Ok(ocvrs_create_string(ret.c_str()));".into()
		} else if return_type.is_char_ptr_string() {
			"return Ok(ocvrs_create_string(ret));".into()
		} else {
			"return Ok(ret);".into()
		}
	}

	fn pre_post_arg_handle(mut arg: String, args: &mut Vec<String>) {
		if !arg.is_empty() {
			arg.push(';');
			args.push(arg);
		}
	}

	pub fn gen_rust_with_name(&self, name: &str, opencv_version: &str) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/func/rust.tpl.rs").compile_interpolation()
		);

		let args = Field::rust_disambiguate_names(self.arguments()).collect::<Vec<_>>();
		let as_instance_method = self.as_instance_method();
		let is_method_const = self.is_const();
		let is_infallible = self.is_infallible();
		let mut decl_args = Vec::with_capacity(args.len());
		let mut call_args = Vec::with_capacity(args.len());
		let mut forward_args = Vec::with_capacity(args.len());
		let mut pre_call_args = Vec::with_capacity(args.len());
		let mut post_call_args = Vec::with_capacity(args.len());
		if let Some(cls) = &as_instance_method {
			decl_args.push(cls.type_ref().rust_self_func_decl(is_method_const));
			call_args.push(cls.type_ref().rust_self_func_call(is_method_const));
		}
		let mut callback_arg_name: Option<String> = None;
		for (name, arg) in args {
			let type_ref = arg.type_ref();
			if arg.is_user_data() {
				Self::pre_post_arg_handle(
					type_ref.rust_userdata_pre_call(&name, callback_arg_name.as_ref().map(|x| x.as_str()).expect("Can't get name of the callback arg")),
					&mut pre_call_args
				);
			} else {
				if type_ref.as_function().is_some() {
					callback_arg_name = Some(name.clone());
				}
				if !arg.as_slice_len().is_some() {
					decl_args.push(type_ref.rust_arg_func_decl(&name));
				}
				Self::pre_post_arg_handle(type_ref.rust_arg_pre_call(&name, is_infallible), &mut pre_call_args);
			}
			if let Some((slice_arg, len_div)) = arg.as_slice_len() {
				let slice_call = if len_div > 1 {
					format!("({slice_arg}.len() / {len_div}) as _", slice_arg=slice_arg, len_div=len_div)
				} else {
					format!("{slice_arg}.len() as _", slice_arg=slice_arg)
				};
				call_args.push(slice_call);
			} else {
				call_args.push(type_ref.rust_arg_func_call(&name, false));
			}
			forward_args.push(type_ref.rust_arg_forward(&name));
			Self::pre_post_arg_handle(type_ref.rust_arg_post_call(&name, is_infallible), &mut post_call_args);
		}

		let doc_comment = self.rendered_doc_comment(opencv_version);
		let debug = get_debug(self);
		let visibility = if let Some(cls) = as_instance_method {
			if cls.is_trait() {
				""
			} else {
				"pub "
			}
		} else {
			"pub "
		};
		let identifier = self.identifier();
		let is_safe = !settings::FUNC_UNSAFE.contains(identifier.as_ref());
		let return_type = self.return_type();
		let return_type_func_decl = if is_infallible {
			return_type.rust_return_func_decl()
		} else {
			return_type.rust_return_func_decl_wrapper()
		};
		let mut prefix = String::new();
		let mut suffix = if is_infallible {
			format!(".expect(\"Infallible function failed: {name}\")", name=name)
		} else {
			String::new()
		};
		if !post_call_args.is_empty() {
			post_call_args.push("out".into());
			prefix.push_str("let out = ");
			suffix.push(';');
		}
		let decl_args = decl_args.join(", ");
		let pre_call_args = pre_call_args.join("\n");
		let call_args = call_args.join(", ");
		let forward_args = forward_args.join(", ");
		let post_call_args = post_call_args.join("\n");
		let ret_map = return_type.rust_return_map(is_safe);
		let mut attributes = String::new();
		if let Some(attrs) = settings::FUNC_CFG_ATTR.get(identifier.as_ref()) {
			attributes = format!("#[cfg({})]", attrs.0);
		}

		let tpl = if let Some(tpl) = settings::FUNC_MANUAL.get(identifier.as_ref()) {
			tpl
		} else {
			&TPL
		};
		tpl.interpolate(&hashmap! {
			"doc_comment" => doc_comment.as_str(),
			"debug" => &debug,
			"attributes" => &attributes,
			"visibility" => &visibility,
			"unsafety_decl" => if is_safe { "" } else { "unsafe " },
			"name" => name,
			"generic_decl" => "",
			"decl_args" => &decl_args,
			"rv_rust_full" => return_type_func_decl.as_ref(),
			"pre_call_args" => &pre_call_args,
			"prefix" => &prefix,
			"unsafety_call" => if is_safe { "unsafe " } else { "" },
			"identifier" => identifier.as_ref(),
			"call_args" => &call_args,
			"forward_args" => &forward_args,
			"suffix" => &suffix,
			"post_call_args" => &post_call_args,
			"ret_map" => ret_map.as_ref(),
		})
	}
}

impl<'tu> EntityElement<'tu> for Func<'tu, '_> {
	fn entity(&self) -> Entity<'tu> {
		self.entity
	}
}

impl Element for Func<'_, '_> {
	fn is_excluded(&self) -> bool {
		let identifier = self.identifier();
		if settings::FUNC_MANUAL.contains_key(identifier.as_ref()) || settings::FUNC_SPECIALIZE.contains_key(identifier.as_ref()) {
			return false;
		}
		DefaultElement::is_excluded(self)
			|| self.is_generic()
			|| if let Some(cls) = self.as_constructor() { // don't generate constructors of abstract classes
			cls.is_abstract()
		} else {
			false
		}
	}

	fn is_ignored(&self) -> bool {
		let identifier = self.identifier();
		if settings::FUNC_MANUAL.contains_key(identifier.as_ref()) || settings::FUNC_SPECIALIZE.contains_key(identifier.as_ref()) {
			return false;
		}
		DefaultElement::is_ignored(self)
			|| self.entity.get_availability() == Availability::Unavailable
			|| self.as_operator().map_or(false, |(.., op)| op == OperatorKind::Unsupported)
			|| self.arguments().into_iter().any(|a| a.type_ref().is_ignored())
			|| {
					let ret = self.return_type();
					ret.is_ignored() || ret.as_class().map_or(false, |cls| cls.is_abstract())
				}
			|| settings::FUNC_RENAME.get(identifier.as_ref()).filter(|&&n| n == "-").is_some()
	}

	fn is_system(&self) -> bool {
		DefaultElement::is_system(self)
	}

	fn is_public(&self) -> bool {
		DefaultElement::is_public(self)
	}

	fn identifier(&self) -> Cow<str> {
		let mut out: String = DefaultElement::identifier(self).into_owned();
		if let Some(spec) = self.as_specialized() {
			for typ in spec.values() {
				out.push('_');
				let mut typ = typ.to_string();
				typ.cleanup_name();
				out.push_str(&typ);
			}
		}
		if self.is_const() {
			out += "_const";
		}
		for arg in self.arguments() {
			out.push('_');
			let type_ref = arg.type_ref();
			let mut safe_id = type_ref.cpp_safe_id();
			// workaround for duplicate function definition for
			// cv_ximgproc_ContourFitting_estimateTransformation_const__InputArray_const__InputArray_const__OutputArray_doubleX_bool
			// it has 2 definitions, with pointer and with reference
			if type_ref.as_reference().map_or(false, |inner| inner.is_primitive()) {
				let safe_id = safe_id.to_mut();
				if let Some((idx, last_char)) = safe_id.char_indices().rev().next() {
					if last_char == 'X' {
						safe_id.replace_range(idx..idx + 'X'.len_utf8(), "R");
					}
				}
			}
			out += &safe_id;
		}
		out.into()
	}

	fn usr(&self) -> Cow<str> {
		DefaultElement::usr(self)
	}

	fn rendered_doc_comment_with_prefix(&self, prefix: &str, opencv_version: &str) -> String {
		let mut comment = self.entity.get_comment().unwrap_or_default();
		const OVERLOAD: &str = "@overload";
		if let Some(idx) = comment.find(OVERLOAD) {
			let rep = if let Some(copy) = self.gen_env.get_func_comment(self.entity.cpp_fullname().as_ref()) {
				format!("{}\n\n## Overloaded parameters\n", copy)
			} else {
				"".to_string()
			};
			comment.replace_range(idx..idx + OVERLOAD.len(), &rep);
		}
		static COPY_BRIEF: Lazy<Regex> = Lazy::new(|| Regex::new(r#"@copybrief\s+(\w+)"#).unwrap());
		comment.replace_in_place_regex_cb(&COPY_BRIEF, |comment, caps| {
			let copy_name = caps.get(1).map(|(s, e)| &comment[s..e]).expect("Impossible");
			let mut copy_full_name = self.cpp_namespace().into_owned();
			copy_full_name += "::";
			copy_full_name += copy_name;
			if let Some(copy) = self.gen_env.get_func_comment(&copy_full_name) {
				Some(copy.into())
			} else {
				Some("".into())
			}
		});
		comment::render_doc_comment_with_processor(&comment, prefix, opencv_version, |out| {
			let mut default_args_comment = String::with_capacity(1024);
			for arg in self.arguments() {
				if let Some(def_val) = arg.default_value() {
					if default_args_comment.is_empty() {
						default_args_comment += "## C++ default parameters";
					}
					default_args_comment += &format!("\n* {name}: {val}", name=arg.rust_leafname(), val=def_val);
				}
			}
			if !default_args_comment.is_empty() {
				if !out.is_empty() {
					out.push_str("\n\n");
				}
				out.push_str(&default_args_comment);
			}
		})
	}

	fn cpp_namespace(&self) -> Cow<str> {
		DefaultElement::cpp_namespace(self)
	}

	fn cpp_localname(&self) -> Cow<str> {
		if self.as_conversion_method().is_some() {
			format!("operator {}", self.return_type().cpp_full()).into()
		} else if self.as_field_setter().is_some() {
			let name = DefaultElement::cpp_localname(self);
				format!("set{}{}", name[..1].to_uppercase(), &name[1..]).into()
		} else {
			DefaultElement::cpp_localname(self)
		}
	}

	fn rust_module(&self) -> Cow<str> {
		DefaultElement::rust_module(self)
	}

	fn rust_leafname(&self) -> Cow<str> {
		let cpp_name = if let Some(name) = self.gen_env.get_export_config(self.entity).and_then(|c| c.rename.as_ref()) {
			name.into()
		} else {
			self.cpp_localname()
		};
		let rust_name = if let Some(cls) = self.as_constructor() {
			let args = self.arguments();
			'ctor_name: loop { // fixme use named block when stable
				if args.is_empty() {
					break 'ctor_name "default";
				} else if args.len() == 1 {
					let arg_typeref = args[0].type_ref();
					let class_arg = arg_typeref.as_reference().and_then(|typ| {
						if let Some(ptr) = typ.as_smart_ptr() {
							ptr.pointee()
						} else {
							typ
						}.as_class()
					});
					if let Some(other) = class_arg {
						if cls == other {
							break 'ctor_name if arg_typeref.is_const() {
								"copy"
							} else {
								"copy_mut"
							};
						}
					}
				}
				break 'ctor_name "new";
			}.into()
		} else if let Some(..) = self.as_conversion_method() {
			let mut name: String = self.return_type().rust_local().into_owned();
			name.cleanup_name();
			format!("to_{}", name).into()
		} else if let Some((.., kind)) = self.as_operator() {
			if cpp_name.starts_with("operator") {
				match kind {
					OperatorKind::Unsupported => {
						cpp_name
					}
					OperatorKind::Index => {
						if self.is_const() {
							"get".into()
						} else {
							"get_mut".into()
						}
					}
					OperatorKind::Add => {
						"add".into()
					}
					OperatorKind::Sub => {
						"sub".into()
					}
					OperatorKind::Mul => {
						"mul".into()
					}
					OperatorKind::Div => {
						"div".into()
					}
					OperatorKind::Deref => {
						if self.is_const() {
							"try_deref".into()
						} else {
							"try_deref_mut".into()
						}
					}
				}
			} else {
				cpp_name
			}
		} else {
			cpp_name
		};
		if let Some(&name) = settings::FUNC_RENAME.get(self.identifier().as_ref()) {
			if name.contains('+') {
				reserved_rename(name.replace('+', rust_name.as_ref()).to_snake_case().into())
			} else {
				name.into()
			}
		} else {
			reserved_rename(rust_name.to_snake_case().into())
		}
	}

	fn rust_localname(&self) -> Cow<str> {
		DefaultElement::rust_localname(self)
	}
}

impl GeneratedElement for Func<'_, '_> {
	fn element_safe_id(&self) -> String {
		format!("{}-{}", self.rust_module(), self.rust_localname())
	}

	fn gen_rust(&self, opencv_version: &str) -> String {
		let name = if let Some(name_hint) = self.name_hint {
			name_hint.into()
		} else {
			self.rust_leafname()
		};
		self.gen_rust_with_name(&name, opencv_version)
	}

	fn gen_rust_exports(&self) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/func/rust_extern.tpl.rs").compile_interpolation()
		);

		if settings::FUNC_MANUAL.contains_key(self.identifier().as_ref()) {
			return "".to_string()
		}

		let identifier = self.identifier();
		let mut attributes = String::new();
		if let Some(attrs) = settings::FUNC_CFG_ATTR.get(identifier.as_ref()) {
			attributes = format!("#[cfg({})]", attrs.0);
		}
		let mut args = vec![];
		if let Some(cls) = self.as_instance_method() {
			args.push(cls.type_ref().rust_extern_self_func_decl(self.is_const()));
		}
		for arg in self.arguments() {
			args.push(arg.type_ref().rust_extern_arg_func_decl(&arg.rust_leafname(), Constness::Auto))
		}
		let return_type = self.return_type();
		let return_wrapper_type = return_type.rust_extern_return_wrapper_full();
		TPL.interpolate(&hashmap! {
			"attributes" => attributes.into(),
			"debug" => get_debug(self).into(),
			"identifier" => identifier,
			"args" => args.join(", ").into(),
			"return_wrapper_type" => return_wrapper_type,
		})
	}

	fn gen_cpp(&self) -> String {
		static TPL: Lazy<CompiledInterpolation> = Lazy::new(
			|| include_str!("../tpl/func/cpp.tpl.cpp").compile_interpolation()
		);

		if settings::FUNC_MANUAL.contains_key(self.identifier().as_ref()) {
			return "".to_string()
		}

		let identifier = self.identifier();
		let mut attributes_begin = String::new();
		let mut attributes_end = String::new();
		if let Some(attrs) = settings::FUNC_CFG_ATTR.get(identifier.as_ref()) {
			attributes_begin = format!("#if {}", attrs.1);
			attributes_end = "#endif".to_string();
		}
		let args = Field::cpp_disambiguate_names(self.arguments()).collect::<Vec<_>>();
		let mut decl_args = Vec::with_capacity(args.len());
		let mut pre_call_args = Vec::with_capacity(args.len());
		let mut post_call_args = Vec::with_capacity(args.len());
		if let Some(cls) = self.as_instance_method() {
			decl_args.push(cls.type_ref().cpp_self_func_decl(self.is_const()));
		}
		for (name, arg) in args {
			let type_ref = arg.type_ref();
			decl_args.push(type_ref.cpp_arg_func_decl(&name));
			let mut pre_call_arg = type_ref.cpp_arg_pre_call(&name);
			if !pre_call_arg.is_empty() {
				pre_call_arg.push(';');
				pre_call_args.push(pre_call_arg);
			}
			let mut post_call_arg = type_ref.cpp_arg_post_call(&name);
			if !post_call_arg.is_empty() {
				post_call_arg.push(';');
				post_call_args.push(post_call_arg);
			}
		}

		let return_type = self.return_type();

		TPL.interpolate(&hashmap! {
			"attributes_begin" => attributes_begin.into(),
			"debug" => get_debug(self).into(),
			"return_wrapper_type" => return_type.cpp_extern_return_wrapper_full(),
			"identifier" => identifier,
			"decl_args" => decl_args.join(", ").into(),
			"pre_call_args" => pre_call_args.join("\n").into(),
			"call" => self.cpp_call_invoke().into(),
			"post_call_args" => post_call_args.join("\n").into(),
			"return" => self.cpp_method_return(),
			"attributes_end" => attributes_end.into(),
		})
	}
}

impl fmt::Display for Func<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.entity.get_display_name().expect("Can't get display name"))
	}
}

impl fmt::Debug for Func<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut debug_struct = f.debug_struct("Func");
		self.update_debug_struct(&mut debug_struct)
			.field("export_config", &self.gen_env.get_export_config(self.entity))
			.field("is_const", &self.is_const())
			.field("type_hint", &self.type_hint)
			.field("kind", &self.kind())
			.field("return_type", &self.return_type())
			.field("arguments", &self.arguments())
			.finish()
	}
}
