use std::{
	borrow::Cow,
	collections::HashMap,
	fmt::{self, Write},
};

use clang::{Availability, Entity, EntityKind, ExceptionSpecification};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{
	Class,
	comment,
	Constness,
	DefaultElement,
	DefinitionLocation,
	DependentType,
	DependentTypeMode,
	Element,
	EntityElement,
	EntityExt,
	Field,
	FieldTypeHint,
	GeneratorEnv,
	reserved_rename,
	settings,
	StrExt,
	StringExt,
	type_ref::{FishStyle, TypeRefTypeHint},
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
	Equals,
	Incr,
	Decr,
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
			"==" => {
				OperatorKind::Equals
			}
			"++" => {
				OperatorKind::Incr
			}
			"--" => {
				OperatorKind::Decr
			}
			_ => {
				OperatorKind::Unsupported
			},
		}
	}
}

#[derive(Debug)]
pub enum Kind<'tu, 'ge> {
	Function,
	FunctionOperator(OperatorKind),
	Constructor(Class<'tu, 'ge>),
	InstanceMethod(Class<'tu, 'ge>),
	StaticMethod(Class<'tu, 'ge>),
	FieldAccessor(Class<'tu, 'ge>),
	ConversionMethod(Class<'tu, 'ge>),
	InstanceOperator(Class<'tu, 'ge>, OperatorKind),
	GenericFunction,
	GenericInstanceMethod(Class<'tu, 'ge>),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FunctionTypeHint {
	None,
	FieldSetter,
	Specialized(&'static HashMap<&'static str, &'static str>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FuncId<'f> {
	name: Cow<'f, str>,
	args: Vec<Cow<'f, str>>,
}

impl<'f> FuncId<'f> {
	/// # Parameters
	/// name: fully qualified C++ function name (e.g. cv::Mat::create)
	/// args: C++ argument names ("unnamed" for unnamed ones)
	pub fn new<const ARGS: usize>(name: &'static str, args: [&'static str; ARGS]) -> FuncId<'static> {
		FuncId {
			name: name.into(),
			args: IntoIterator::into_iter(args).map(|a| a.into()).collect(),
		}
	}

	pub fn from_entity(entity: Entity) -> Self {
		let name = entity.cpp_fullname().into_owned().into();
		let args = if let EntityKind::FunctionTemplate = entity.get_kind() {
			let mut args = vec![];
			entity.walk_children_while(|child| {
				if child.get_kind() == EntityKind::ParmDecl {
					args.push(child.get_name()
						.map(Cow::Owned)
						.unwrap_or_else(|| "unnamed".into()));
				}
				true
			});
			args
		} else {
			entity.get_arguments().into_iter()
				.flatten()
				.map(|a| a.get_name()
					.map(Cow::Owned)
					.unwrap_or_else(|| "unnamed".into()))
				.collect()
		};
		Self { name, args }
	}

	pub fn name(&self) -> &str {
		self.name.as_ref()
	}

	pub fn args(&self) -> &[Cow<str>] {
		&self.args
	}
}

impl fmt::Display for FuncId<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}({})", self.name, self.args.join(", "))
	}
}

#[derive(Clone)]
pub struct Func<'tu, 'ge> {
	entity: Entity<'tu>,
	type_hint: FunctionTypeHint,
	name_hint: Option<String>,
	gen_env: &'ge GeneratorEnv<'tu>,
}

impl<'tu, 'ge> Func<'tu, 'ge> {
	pub fn new(entity: Entity<'tu>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self { entity, type_hint: FunctionTypeHint::None, name_hint: None, gen_env }
	}

	pub fn new_ext(entity: Entity<'tu>, type_hint: FunctionTypeHint, name_hint: Option<String>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self { entity, type_hint, name_hint, gen_env }
	}

	pub fn set_name_hint(&mut self, name_hint: Option<String>) {
		self.name_hint = name_hint;
	}

	pub fn name_hint(&self) -> Option<&str> {
		self.name_hint.as_deref()
	}

	pub fn type_hint(&self) -> FunctionTypeHint {
		self.type_hint
	}

	pub fn kind(&self) -> Kind<'tu, 'ge> {
		const OPERATOR: &str = "operator";
		match self.entity.get_kind() {
			EntityKind::FunctionDecl => {
				if let Some(operator) = self.entity.cpp_localname().strip_prefix(OPERATOR) {
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
				} else if let Some(operator) = self.entity.cpp_localname().strip_prefix(OPERATOR) {
					let arg_count = self.entity.get_arguments().map_or(0, |v| v.len());
					Kind::InstanceOperator(class, OperatorKind::new(operator.trim(), arg_count))
				} else {
					Kind::InstanceMethod(class)
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

	pub fn as_constructor(&self) -> Option<Class<'tu, 'ge>> {
		if let Kind::Constructor(out) = self.kind() {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_instance_method(&self) -> Option<Class<'tu, 'ge>> {
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

	pub fn as_static_method(&self) -> Option<Class<'tu, 'ge>> {
		if let Kind::StaticMethod(out) = self.kind() {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_field_accessor(&self) -> Option<Field<'tu, 'ge>> {
		if let Kind::FieldAccessor(..) = self.kind() {
			Some(Field::new(self.entity, self.gen_env))
		} else {
			None
		}
	}

	pub fn as_field_setter(&self) -> Option<Field<'tu, 'ge>> {
		if self.as_field_accessor().is_some() && self.type_hint == FunctionTypeHint::FieldSetter {
			Some(Field::new_ext(self.entity, FieldTypeHint::FieldSetter, self.gen_env))
		} else {
			None
		}
	}

	pub fn as_conversion_method(&self) -> Option<Class<'tu, 'ge>> {
		if let Kind::ConversionMethod(out) = self.kind() {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_operator(&self) -> Option<OperatorKind> {
		match self.kind() {
			Kind::FunctionOperator(kind) => {
				Some(kind)
			}
			Kind::InstanceOperator(_, kind) => {
				Some(kind)
			}
			_ => {
				None
			}
		}
	}

	pub fn constness(&self) -> Constness {
		if settings::FORCE_CONSTANT_METHOD.contains(self.cpp_fullname().as_ref()) {
			Constness::Const
		} else if let Some(fld) = self.as_field_accessor() {
			if self.type_hint == FunctionTypeHint::FieldSetter {
				Constness::Mut
			} else {
				let type_ref = fld.type_ref();
				Constness::from_is_mut(
					type_ref.as_array().is_some()
						|| type_ref.as_smart_ptr().is_some()
						|| type_ref.as_pointer().map_or(false, |r| r.constness().is_mut())
				)
			}
		} else {
			Constness::from_is_const(self.entity.is_const_method())
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

	pub fn is_infallible(&self) -> bool {
		self.as_field_accessor().is_some()
			|| matches!(self.entity.get_exception_specification(), Some(ExceptionSpecification::BasicNoexcept) | Some(ExceptionSpecification::Unevaluated))
			|| settings::FORCE_INFALLIBLE.contains(&self.func_id())
	}

	pub fn is_unsafe(&self) -> bool {
		settings::FUNC_UNSAFE.contains(&self.func_id())
			|| self.arguments().into_iter().any(|a| a.type_ref().is_pass_by_ptr() && !a.is_user_data())
	}

	pub fn is_default_constructor(&self) -> bool {
		self.entity.is_default_constructor() && !self.has_arguments()
	}

	pub fn is_clone(&self) -> bool {
		if self.rust_leafname(FishStyle::No) == "clone" {
			if let Some(c) = self.as_instance_method() {
				!self.has_arguments() && self.return_type().as_class().map_or(false, |r| r == c)
			} else {
				false
			}
		} else {
			false
		}
	}

	pub fn is_no_discard(&self) -> bool {
		self.gen_env.get_export_config(self.entity).map_or(false, |c| c.no_discard)
	}

	pub fn is_naked_return(&self) -> bool {
		self.is_infallible() && {
			let ret_type = self.return_type();
			ret_type.is_primitive() || ret_type.as_pointer().is_some() || ret_type.as_array().is_some()
				|| ret_type.is_by_ptr() || ret_type.as_string().is_some()
		}
	}

	pub fn as_specialized(&self) -> Option<&'static HashMap<&'static str, &'static str>> {
		if let FunctionTypeHint::Specialized(spec) = self.type_hint {
			Some(spec)
		} else {
			None
		}
	}

	pub fn return_type(&self) -> TypeRef<'tu, 'ge> {
		match self.kind() {
			Kind::Constructor(cls) => {
				cls.type_ref()
			}
			Kind::Function | Kind::InstanceMethod(..) | Kind::StaticMethod(..)
			| Kind::ConversionMethod(..) | Kind::GenericInstanceMethod(..) | Kind::GenericFunction
			| Kind::FunctionOperator(..) | Kind::InstanceOperator(..) => {
				let mut out = TypeRef::new_ext(self.entity.get_result_type().expect("Can't get return type"), TypeRefTypeHint::PrimitiveRefAsPointer, None, self.gen_env);
				if let Some(spec) = self.as_specialized() {
					if out.is_generic() {
						let spec_type = spec.get(out.base().cpp_full().as_ref())
							.and_then(|s| self.gen_env.resolve_type(s));
						if let Some(spec_type) = spec_type {
							out.set_type_hint(TypeRefTypeHint::Specialized(spec_type));
						}
					}
				} else if let Some(&over) = settings::ARGUMENT_OVERRIDE.get(&self.func_id()).and_then(|x| x.get("return")) {
					out.set_type_hint(TypeRefTypeHint::ArgOverride(over))
				}
				if let Some(type_ref) = out.as_reference() {
					type_ref
				} else {
					out
				}
			}
			Kind::FieldAccessor(..) => {
				if self.type_hint == FunctionTypeHint::FieldSetter {
					TypeRef::new(self.gen_env.resolve_type("void").expect("Can't resolve void type"), self.gen_env)
				} else {
					let mut out = Field::new(self.entity, self.gen_env).type_ref();
					out.set_type_hint(TypeRefTypeHint::PrimitiveRefAsPointer);
					out
				}
			}
		}
	}

	pub fn has_arguments(&self) -> bool {
		!self.clang_arguments().is_empty()
	}

	pub fn clang_arguments(&self) -> Vec<Entity<'tu>> {
		match self.kind() {
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
		}
	}

	pub fn arguments(&self) -> Vec<Field<'tu, 'ge>> {
		let empty_spec_hashmap = HashMap::new();
		let spec = self.as_specialized().unwrap_or(&empty_spec_hashmap);

		let is_field_setter = self.as_field_setter().is_some();
		let arg_overrides = settings::ARGUMENT_OVERRIDE.get(&self.func_id());

		self.clang_arguments().into_iter()
			.map(|a| {
				if is_field_setter {
					return Field::new_ext(a, FieldTypeHint::FieldSetter, self.gen_env)
				}

				let arg_override = arg_overrides
					.and_then(|o| a.get_name().and_then(|arg_name| o.get(arg_name.as_str())));
				if let Some(arg_override) = arg_override {
					return Field::new_ext(a, FieldTypeHint::ArgOverride(*arg_override), self.gen_env);
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

	pub fn dependent_types(&self) -> Vec<DependentType<'tu, 'ge>> {
		self.arguments().into_iter()
			.map(|a| a.type_ref())
			.filter(|t| !t.is_ignored())
			.flat_map(|t| t.dependent_types(DependentTypeMode::None))
			.chain(self.return_type().dependent_types(DependentTypeMode::ForReturn(DefinitionLocation::Module)))
			.collect()
	}

	pub fn identifier(&self) -> Cow<str> {
		let mut out: String = if self.as_field_accessor().is_some() {
			let mut out: String = self.cpp_namespace().into_owned();
			if !out.is_empty() {
				out += "::";
			}
			let local_name = DefaultElement::cpp_localname(self);
			let (first_letter, rest) = local_name.split_at(1);
			if self.as_field_setter().is_some() {
				write!(&mut out, "setProp{}{}", first_letter.to_uppercase(), rest).expect("write! to String shouldn't fail");
			} else {
				write!(&mut out, "getProp{}{}", first_letter.to_uppercase(), rest).expect("write! to String shouldn't fail");
			}
			out
		} else {
			self.cpp_fullname().into_owned()
		};
		out.cleanup_name();
		if let Some(spec) = self.as_specialized() {
			for typ in spec.values() {
				out.push('_');
				let mut typ = typ.to_string();
				typ.cleanup_name();
				out.push_str(&typ);
			}
		}
		if self.constness().is_const() {
			out += "_const";
		}
		for arg in self.arguments() {
			out.push('_');
			let type_ref = arg.type_ref();
			out += &type_ref.cpp_safe_id();
		}
		out.into()
	}

	pub fn func_id(&self) -> FuncId {
		let mut out = FuncId::from_entity(self.entity);
		if self.as_field_setter().is_some() {
			out.args.push("val".into());
		}
		out
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
			|| self.as_operator().map_or(false, |kind| {
				if matches!(kind, OperatorKind::Incr | OperatorKind::Decr) {
					// filter out postfix version of ++ and --: https://en.cppreference.com/w/cpp/language/operator_incdec
					self.clang_arguments().len() == 1
				} else {
					false
				}
			})
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
			|| self.as_operator().map_or(false, |op| op == OperatorKind::Unsupported)
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

	fn usr(&self) -> Cow<str> {
		DefaultElement::usr(self)
	}

	fn rendered_doc_comment_with_prefix(&self, prefix: &str, opencv_version: &str) -> String {
		let mut comment = self.entity.get_comment().unwrap_or_default();
		let line = self.entity.get_location().map(|l| l.get_file_location().line).unwrap_or(0);
		const OVERLOAD: &str = "@overload";
		if let Some(idx) = comment.find(OVERLOAD) {
			let rep = if let Some(copy) = self.gen_env.get_func_comment(line, self.entity.cpp_fullname().as_ref()) {
				format!("{}\n\n## Overloaded parameters\n", copy)
			} else {
				"This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.".to_string()
			};
			comment.replace_range(idx..idx + OVERLOAD.len(), &rep);
		}
		static COPY_BRIEF: Lazy<Regex> = Lazy::new(|| Regex::new(r#"@copybrief\s+(\w+)"#).unwrap());
		comment.replace_in_place_regex_cb(&COPY_BRIEF, |comment, caps| {
			let copy_name = caps.get(1).map(|(s, e)| &comment[s..e]).expect("Impossible");
			let mut copy_full_name = self.cpp_namespace().into_owned();
			copy_full_name += "::";
			copy_full_name += copy_name;
			if let Some(copy) = self.gen_env.get_func_comment(line, &copy_full_name) {
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
					write!(&mut default_args_comment, "\n* {name}: {val}", name = arg.rust_leafname(FishStyle::No), val = def_val)
						.expect("write! to String shouldn't fail");
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
		DefaultElement::cpp_namespace(self).into()
	}

	fn cpp_localname(&self) -> Cow<str> {
		if self.as_conversion_method().is_some() {
			format!("operator {}", self.return_type().cpp_full()).into()
		} else if self.as_field_setter().is_some() {
			let name = DefaultElement::cpp_localname(self);
			let (first_letter, rest) = name.split_at(1);
			format!("set{}{}", first_letter.to_uppercase(), rest).into()
		} else {
			DefaultElement::cpp_localname(self)
		}
	}

	fn rust_module(&self) -> Cow<str> {
		DefaultElement::rust_module(self)
	}

	fn rust_leafname(&self, _fish_style: FishStyle) -> Cow<str> {
		let cpp_name = if let Some(name) = self.gen_env.get_rename_config(self.entity).map(|c| &c.rename) {
			name.into()
		} else {
			self.cpp_localname()
		};
		let rust_name = if let Some(cls) = self.as_constructor() {
			let args = self.arguments();
			#[allow(clippy::never_loop)] // fixme use named block when stable
			'ctor_name: loop {
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
							break 'ctor_name if arg_typeref.constness().is_const() {
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
		} else if let Some(kind) = self.as_operator() {
			if cpp_name.starts_with("operator") {
				match kind {
					OperatorKind::Unsupported => {
						cpp_name
					}
					OperatorKind::Index => {
						if self.constness().is_const() {
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
						if self.constness().is_const() {
							"try_deref".into()
						} else {
							"try_deref_mut".into()
						}
					}
					OperatorKind::Equals => {
						"equals".into()
					}

					OperatorKind::Incr => {
						"incr".into()
					}
					OperatorKind::Decr => {
						"decr".into()
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

	fn rust_localname(&self, fish_style: FishStyle) -> Cow<str> {
		DefaultElement::rust_localname(self, fish_style)
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
			.field("is_const", &self.constness())
			.field("is_infallible", &self.is_infallible())
			.field("type_hint", &self.type_hint)
			.field("kind", &self.kind())
			.field("return_type", &self.return_type())
			.field("arguments", &self.arguments())
			.finish()
	}
}
