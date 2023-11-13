use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;
use std::rc::Rc;

use clang::{Availability, Entity, EntityKind, ExceptionSpecification};
use once_cell::sync::Lazy;
use regex::Regex;

pub use desc::{FuncCppBody, FuncDesc, FuncRustBody, FuncRustExtern};

use crate::comment::strip_comment_markers;
use crate::debug::{DefinitionLocation, LocationName};
use crate::element::{ExcludeKind, UNNAMED};
use crate::entity::WalkAction;
use crate::field::FieldDesc;
use crate::settings::TypeRefFactory;
use crate::type_ref::{Constness, CppNameStyle, TypeRefDesc, TypeRefTypeHint};
use crate::writer::rust_native::element::RustElement;
use crate::{
	settings, Class, DefaultElement, Element, EntityExt, Field, FieldTypeHint, GeneratedType, GeneratorEnv, IteratorExt,
	NameDebug, StrExt, StringExt, TypeRef,
};

mod desc;

#[derive(Clone)]
pub enum Func<'tu, 'ge> {
	Clang {
		entity: Entity<'tu>,
		rust_custom_leafname: Option<Rc<str>>,
		gen_env: &'ge GeneratorEnv<'tu>,
	},
	Desc(Rc<FuncDesc<'tu, 'ge>>),
}

impl<'tu, 'ge> Func<'tu, 'ge> {
	pub fn new(entity: Entity<'tu>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self::Clang {
			entity,
			rust_custom_leafname: None,
			gen_env,
		}
	}

	pub fn new_ext(entity: Entity<'tu>, rust_custom_leafname: Option<Rc<str>>, gen_env: &'ge GeneratorEnv<'tu>) -> Self {
		Self::Clang {
			entity,
			rust_custom_leafname,
			gen_env,
		}
	}

	pub fn new_desc(desc: FuncDesc<'tu, 'ge>) -> Self {
		Self::Desc(Rc::new(desc))
	}

	fn with_desc_mut(desc: &mut Rc<FuncDesc<'tu, 'ge>>, cb: impl FnOnce(&mut FuncDesc<'tu, 'ge>)) {
		if let Some(desc) = Rc::get_mut(desc) {
			cb(desc);
		} else {
			let mut new_desc = desc.as_ref().clone();
			cb(&mut new_desc);
			*desc = Rc::new(new_desc);
		}
	}

	/// Sets custom rust_leafname for this func, used to e.g. disambiguate function names
	pub fn set_rust_custom_leafname(&mut self, rust_custom_leafname: Option<Rc<str>>) {
		match self {
			Self::Clang {
				rust_custom_leafname: old_rust_custom_leafname,
				..
			} => *old_rust_custom_leafname = rust_custom_leafname,
			Self::Desc(desc) => {
				if desc.rust_custom_leafname != rust_custom_leafname {
					Self::with_desc_mut(desc, |desc| desc.rust_custom_leafname = rust_custom_leafname);
				}
			}
		}
	}

	pub fn rust_custom_leafname(&self) -> Option<&str> {
		match self {
			Self::Clang {
				rust_custom_leafname, ..
			} => rust_custom_leafname.as_deref(),
			Self::Desc(desc) => desc.rust_custom_leafname.as_ref().map(|n| n.as_ref()),
		}
	}

	pub fn specialize(self, spec: &HashMap<&str, TypeRefFactory>) -> Self {
		let specialized = |type_ref: &TypeRef| -> Option<TypeRef<'static, 'static>> {
			if type_ref.is_generic() {
				spec
					.get(type_ref.source().cpp_name(CppNameStyle::Declaration).as_ref())
					.map(|spec_type| type_ref.map(|_| spec_type().with_constness(type_ref.constness())))
			} else {
				None
			}
		};

		let arguments = self
			.arguments()
			.iter()
			.map(|arg| {
				let arg_type_ref = arg.type_ref();
				specialized(&arg_type_ref).map_or_else(
					|| arg.clone(),
					|type_ref| {
						Field::new_desc(FieldDesc {
							cpp_fullname: arg.cpp_name(CppNameStyle::Reference).into(),
							type_ref,
							type_hint: FieldTypeHint::None,
							default_value: arg.default_value().map(|v| v.into()),
						})
					},
				)
			})
			.collect();
		let return_type_ref = self.return_type_ref();
		let kind = match self.kind().into_owned() {
			FuncKind::GenericFunction => FuncKind::Function,
			FuncKind::GenericInstanceMethod(cls) => FuncKind::InstanceMethod(cls),
			kind => kind,
		};
		let generic = spec
			.values()
			.map(|s| s().cpp_name(CppNameStyle::Reference).into_owned())
			.join(", ");
		let mut desc = self.to_desc(InheritConfig::empty().kind().arguments().return_type_ref());
		desc.kind = kind;
		desc.type_hint = FuncTypeHint::Specialized;
		desc.arguments = arguments;
		desc.return_type_ref = specialized(&return_type_ref).unwrap_or(return_type_ref);
		desc.cpp_body = FuncCppBody::ManualCall(format!("{{{{name}}}}<{generic}>({{{{args}}}})").into());
		Self::new_desc(desc)
	}

	pub(crate) fn to_desc(&self, skip_config: InheritConfig) -> FuncDesc<'tu, 'ge> {
		let kind = if skip_config.kind {
			FuncKind::Function
		} else {
			self.kind().into_owned()
		};
		let cpp_name = if skip_config.name {
			Rc::from("")
		} else {
			self.cpp_name(CppNameStyle::Reference).into()
		};
		let doc_comment = if skip_config.doc_comment {
			Rc::from("")
		} else {
			self.doc_comment().into()
		};
		let arguments: Rc<[Field]> = if skip_config.arguments {
			Rc::new([])
		} else {
			self.arguments().into_owned().into()
		};
		let return_type_ref = if skip_config.return_type_ref {
			TypeRefDesc::void()
		} else {
			self.return_type_ref()
		};
		let def_loc = if skip_config.definition_location {
			DefinitionLocation::Generated
		} else {
			self.file_line_name().location
		};
		FuncDesc {
			kind,
			type_hint: FuncTypeHint::None,
			constness: self.constness(),
			return_kind: self.return_kind(),
			cpp_name,
			rust_custom_leafname: self.rust_custom_leafname().map(Rc::from),
			rust_module: self.rust_module().into(),
			doc_comment,
			def_loc,
			rust_generic_decls: Rc::new([]),
			arguments,
			return_type_ref,
			cpp_body: FuncCppBody::Auto,
			rust_body: FuncRustBody::Auto,
			rust_extern_definition: FuncRustExtern::Auto,
		}
	}

	pub fn inherit(&mut self, ancestor: &Func<'tu, 'ge>, inherit_config: InheritConfig) {
		#[inline]
		fn transfer<'tu, 'ge>(desc: &mut FuncDesc<'tu, 'ge>, ancestor: &Func<'tu, 'ge>, config: InheritConfig) {
			if config.kind {
				desc.kind = ancestor.kind().into_owned();
			}
			if config.name {
				desc.cpp_name = ancestor.cpp_name(CppNameStyle::Reference).into();
			}
			if config.doc_comment {
				desc.doc_comment = ancestor.doc_comment_overloaded().into();
			}
			if config.arguments {
				desc.arguments = ancestor.arguments().into();
			}
			if config.return_type_ref {
				desc.return_type_ref = ancestor.return_type_ref();
			}
			if config.definition_location {
				desc.def_loc = ancestor.file_line_name().location;
			}
		}

		match self {
			Func::Clang { .. } => {
				if inherit_config.any_enabled() {
					let mut desc = self.to_desc(inherit_config);
					transfer(&mut desc, ancestor, inherit_config);
					Self::new_desc(desc);
				}
			}
			Func::Desc(desc) => Self::with_desc_mut(desc, |desc| transfer(desc, ancestor, inherit_config)),
		}
	}

	/// Returns true if function was specialized
	///
	/// It's used to add the return type to function identifier because otherwise it will generate identical function names when
	/// we specialize only on the return type.
	pub fn is_specialized(&self) -> bool {
		match self {
			&Self::Clang { .. } => false,
			Self::Desc(desc) => matches!(desc.type_hint, FuncTypeHint::Specialized),
		}
	}

	pub fn kind(&self) -> Cow<FuncKind<'tu, 'ge>> {
		match self {
			&Self::Clang { entity, gen_env, .. } => {
				const OPERATOR: &str = "operator";
				Cow::Owned(match entity.get_kind() {
					EntityKind::FunctionDecl => {
						if let Some(operator) = entity.cpp_name(CppNameStyle::Declaration).strip_prefix(OPERATOR) {
							let arg_count = entity.get_arguments().map_or(0, |v| v.len());
							FuncKind::FunctionOperator(OperatorKind::new(operator.trim(), arg_count))
						} else {
							FuncKind::Function
						}
					}
					EntityKind::Constructor => FuncKind::Constructor(Class::new(
						entity.get_semantic_parent().expect("Can't get parent of constructor"),
						gen_env,
					)),
					EntityKind::Method => {
						let class = Class::new(entity.get_semantic_parent().expect("Can't get parent of method"), gen_env);
						if entity.is_static_method() {
							FuncKind::StaticMethod(class)
						} else if let Some(operator) = entity.cpp_name(CppNameStyle::Declaration).strip_prefix(OPERATOR) {
							let arg_count = entity.get_arguments().map_or(0, |v| v.len());
							FuncKind::InstanceOperator(class, OperatorKind::new(operator.trim(), arg_count))
						} else {
							FuncKind::InstanceMethod(class)
						}
					}
					EntityKind::ConversionFunction => FuncKind::ConversionMethod(Class::new(
						entity.get_semantic_parent().expect("Can't get parent of method"),
						gen_env,
					)),
					EntityKind::FunctionTemplate => match entity.get_template_kind() {
						Some(EntityKind::Method) => FuncKind::GenericInstanceMethod(Class::new(
							entity.get_semantic_parent().expect("Can't get parent of generic method"),
							gen_env,
						)),
						_ => FuncKind::GenericFunction,
					},
					_ => unreachable!("Unknown function entity: {:#?}", entity),
				})
			}
			Self::Desc(desc) => Cow::Borrowed(&desc.kind),
		}
	}

	pub fn constness(&self) -> Constness {
		if settings::FORCE_CONSTANT_METHOD.contains(self.cpp_name(CppNameStyle::Reference).as_ref()) {
			Constness::Const
		} else {
			match self {
				&Self::Clang { entity, .. } => Constness::from_is_const(entity.is_const_method()),
				Self::Desc(desc) => desc.constness,
			}
		}
	}

	pub fn is_abstract(&self) -> bool {
		match self {
			Self::Clang { entity, .. } => entity.is_pure_virtual_method(),
			Self::Desc(_) => false,
		}
	}

	pub fn is_generic(&self) -> bool {
		match self.kind().as_ref() {
			FuncKind::GenericFunction | FuncKind::GenericInstanceMethod(..) => true,
			FuncKind::Function
			| FuncKind::Constructor(..)
			| FuncKind::InstanceMethod(..)
			| FuncKind::StaticMethod(..)
			| FuncKind::FieldAccessor(..)
			| FuncKind::ConversionMethod(..)
			| FuncKind::FunctionOperator(..)
			| FuncKind::InstanceOperator(..) => false,
		}
	}

	/// Like [Self::doc_comment], but processes `@overload` and `@copybrief` directives if possible by copying the corresponding
	/// doccomment
	pub fn doc_comment_overloaded(&self) -> Cow<str> {
		let mut out = self.doc_comment();
		match self {
			Func::Clang { gen_env, .. } => {
				let line = self.file_line_name().location.as_file().map_or(0, |(_, line)| line);
				const OVERLOAD: &str = "@overload";
				if let Some(idx) = out.find(OVERLOAD) {
					let rep = if let Some(copy) = gen_env.get_func_comment(line, self.cpp_name(CppNameStyle::Reference).as_ref()) {
						Cow::Owned(format!("{copy}\n\n## Overloaded parameters\n"))
					} else {
						"This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.".into()
					};
					out.to_mut().replace_range(idx..idx + OVERLOAD.len(), &rep);
				}
				static COPY_BRIEF: Lazy<Regex> = Lazy::new(|| Regex::new(r"@copybrief\s+(\w+)").unwrap());
				out.to_mut().replace_in_place_regex_cb(&COPY_BRIEF, |comment, caps| {
					let copy_name = caps.get(1).map(|(s, e)| &comment[s..e]).expect("Impossible");
					let mut copy_full_name = self.cpp_namespace().into_owned();
					copy_full_name.extend_sep("::", copy_name);
					if let Some(copy) = gen_env.get_func_comment(line, &copy_full_name) {
						Some(copy.into())
					} else {
						Some("".into())
					}
				});
			}
			Func::Desc(_) => {}
		}
		out
	}

	pub fn return_kind(&self) -> ReturnKind {
		match self {
			Self::Clang { entity, .. } => {
				let is_infallible = matches!(
					entity.get_exception_specification(),
					Some(ExceptionSpecification::BasicNoexcept) | Some(ExceptionSpecification::Unevaluated)
				) || settings::FORCE_INFALLIBLE.contains(&self.func_id());
				if is_infallible {
					if self.return_type_ref().return_as_naked() {
						ReturnKind::InfallibleNaked
					} else {
						ReturnKind::InfallibleViaArg
					}
				} else {
					ReturnKind::Fallible
				}
			}
			Self::Desc(desc) => desc.return_kind,
		}
	}

	pub fn safety(&self) -> Safety {
		Safety::from_unsafe(
			settings::FUNC_UNSAFE.contains(&self.func_id())
				|| self
					.arguments()
					.iter()
					.any(|a| a.type_ref().is_rust_by_ptr() && !a.is_user_data()),
		)
	}

	pub fn is_default_constructor(&self) -> bool {
		match self {
			&Self::Clang { entity, .. } => entity.is_default_constructor() && !self.has_arguments(),
			Self::Desc(_) => false,
		}
	}

	pub fn is_clone(&self) -> bool {
		if self.cpp_name(CppNameStyle::Declaration) == "clone" {
			if let Some(c) = self.kind().as_instance_method() {
				!self.has_arguments() && self.return_type_ref().as_class().map_or(false, |r| r == *c)
			} else {
				false
			}
		} else {
			false
		}
	}

	pub fn is_no_discard(&self) -> bool {
		match self {
			&Self::Clang { entity, gen_env, .. } => gen_env.get_export_config(entity).map_or(false, |c| c.no_discard),
			Self::Desc(_) => false,
		}
	}

	pub fn return_type_ref(&self) -> TypeRef<'tu, 'ge> {
		match self {
			&Self::Clang { entity, gen_env, .. } => match self.kind().as_ref() {
				FuncKind::Constructor(cls) => cls.type_ref(),
				// `operator =` returns a reference to the `self` value and it's quite cumbersome to handle correctly
				FuncKind::InstanceOperator(_, OperatorKind::Set) => TypeRefDesc::void(),
				FuncKind::Function
				| FuncKind::InstanceMethod(..)
				| FuncKind::StaticMethod(..)
				| FuncKind::FieldAccessor(..)
				| FuncKind::ConversionMethod(..)
				| FuncKind::GenericInstanceMethod(..)
				| FuncKind::GenericFunction
				| FuncKind::FunctionOperator(..)
				| FuncKind::InstanceOperator(..) => {
					let mut out = TypeRef::new_ext(
						entity.get_result_type().expect("Can't get return type"),
						TypeRefTypeHint::PrimitiveRefAsPointer,
						None,
						gen_env,
					);
					if let Some(&over) = settings::ARGUMENT_OVERRIDE.get(&self.func_id()).and_then(|x| x.get("return")) {
						out = out.with_type_hint(TypeRefTypeHint::ArgOverride(over))
					}
					if let Some(type_ref) = out.as_reference() {
						type_ref
					} else {
						out
					}
				}
			},
			Self::Desc(desc) => desc.return_type_ref.clone(),
		}
	}

	pub fn has_arguments(&self) -> bool {
		self.num_arguments() > 0
	}

	pub fn num_arguments(&self) -> usize {
		match self {
			&Func::Clang { entity, .. } => self.clang_arguments(entity).len(),
			Func::Desc(desc) => desc.arguments.len(),
		}
	}

	fn clang_arguments(&self, entity: Entity<'tu>) -> Vec<Entity<'tu>> {
		match self.kind().as_ref() {
			FuncKind::GenericFunction | FuncKind::GenericInstanceMethod(..) => {
				let mut out = vec![];
				entity.walk_children_while(|child| {
					if child.get_kind() == EntityKind::ParmDecl {
						out.push(child);
					}
					WalkAction::Continue
				});
				out
			}
			_ => entity.get_arguments().expect("Can't get arguments"),
		}
	}

	pub fn arguments(&self) -> Cow<[Field<'tu, 'ge>]> {
		match self {
			&Self::Clang { entity, gen_env, .. } => {
				let arg_overrides = settings::ARGUMENT_OVERRIDE.get(&self.func_id());
				self
					.clang_arguments(entity)
					.into_iter()
					.map(|a| {
						let arg_override = arg_overrides.and_then(|o| a.get_name().and_then(|arg_name| o.get(arg_name.as_str())));
						if let Some(arg_override) = arg_override {
							return Field::new_ext(a, FieldTypeHint::ArgOverride(*arg_override), gen_env);
						}

						Field::new(a, gen_env)
					})
					.collect()
			}
			Self::Desc(desc) => desc.arguments.as_ref().into(),
		}
	}

	pub fn generated_types(&self) -> Vec<GeneratedType<'tu, 'ge>> {
		self
			.arguments()
			.iter()
			.map(|a| a.type_ref())
			.filter(|t| !t.exclude_kind().is_ignored())
			.flat_map(|t| t.generated_types())
			.chain(self.return_type_ref().generated_types())
			.collect()
	}

	pub fn identifier(&self) -> String {
		let args = self.arguments();
		let mut out: String = if let Some((_, fld)) = self.kind().as_field_accessor() {
			let mut out: String = self.cpp_namespace().into_owned();
			if !out.is_empty() {
				out += "::";
			}
			let decl_name = fld.cpp_name(CppNameStyle::Declaration);
			let (first_letter, rest) = decl_name.split_at(1);
			write!(out, "prop{}{rest}", first_letter.to_uppercase()).expect("write! to String shouldn't fail");
			out
		} else {
			self.cpp_name(CppNameStyle::Reference).into_owned()
		};
		out.cleanup_name();
		// add return type to function id for cases when we specialize on the return type, in theory we should be able to apply
		// this to all of the functions, but it's too much work to rename all those entries in FUNC_RENAME
		if self.is_specialized() {
			out.push('_');
			let mut typ = self.return_type_ref().cpp_name(CppNameStyle::Reference).into_owned();
			typ.cleanup_name();
			out.push_str(&typ);
		}
		if self.constness().is_const() {
			out += "_const";
		}
		for arg in args.as_ref() {
			out.push('_');
			let type_ref = arg.type_ref();
			out += &type_ref.cpp_safe_id();
		}
		out
	}

	pub fn func_id(&self) -> FuncId {
		match self {
			&Self::Clang { entity, .. } => FuncId::from_entity(entity),
			Self::Desc(desc) => FuncId::from_desc(desc),
		}
	}

	pub fn rust_body(&self) -> &FuncRustBody {
		match self {
			Self::Clang { .. } => &FuncRustBody::Auto,
			Self::Desc(desc) => &desc.rust_body,
		}
	}

	pub fn rust_extern_definition(&self) -> FuncRustExtern {
		match self {
			Self::Clang { .. } => FuncRustExtern::Auto,
			Self::Desc(desc) => desc.rust_extern_definition,
		}
	}

	pub fn cpp_body(&self) -> &FuncCppBody {
		match self {
			Self::Clang { .. } => &FuncCppBody::Auto,
			Self::Desc(desc) => &desc.cpp_body,
		}
	}
}

impl Element for Func<'_, '_> {
	fn exclude_kind(&self) -> ExcludeKind {
		let func_id = self.func_id();
		if settings::FUNC_REPLACE.contains_key(&func_id) {
			return ExcludeKind::Included.with_is_excluded(|| settings::FUNC_EXCLUDE.contains(self.identifier().as_str()));
		}
		let kind = self.kind();
		DefaultElement::exclude_kind(self)
			.with_reference_exclude_kind(|| self.return_type_ref().exclude_kind())
			.with_is_excluded(|| {
				let identifier = self.identifier();
				let is_unavailable = match self {
					Func::Clang { entity, .. } => entity.get_availability() == Availability::Unavailable,
					Func::Desc(_) => false,
				};
				is_unavailable
					|| settings::FUNC_EXCLUDE.contains(identifier.as_str())
					|| (self.is_generic())
					|| self.arguments().iter().any(|a| a.type_ref().exclude_kind().is_ignored())
					|| kind.as_operator().map_or(false, |(_, kind)| match kind {
						OperatorKind::Unsupported => true,
						// filter out postfix version of ++ and --: https://en.cppreference.com/w/cpp/language/operator_incdec
						OperatorKind::Incr | OperatorKind::Decr if self.num_arguments() == 1 => true,
						_ => false,
					}) || kind.as_constructor().map_or(false, |cls| cls.is_abstract()) // don't generate constructors of abstract classes
			})
	}

	fn is_system(&self) -> bool {
		match self {
			&Self::Clang { entity, .. } => DefaultElement::is_system(entity),
			Self::Desc(_) => false,
		}
	}

	fn is_public(&self) -> bool {
		match self {
			&Self::Clang { entity, .. } => DefaultElement::is_public(entity),
			Self::Desc(_) => true,
		}
	}

	fn doc_comment(&self) -> Cow<str> {
		match self {
			&Self::Clang { entity, .. } => strip_comment_markers(&entity.get_comment().unwrap_or_default()).into(),
			Self::Desc(desc) => desc.doc_comment.as_ref().into(),
		}
	}

	fn cpp_namespace(&self) -> Cow<str> {
		match self {
			&Self::Clang { entity, .. } => DefaultElement::cpp_namespace(entity).into(),
			Self::Desc(desc) => match self.kind().as_ref() {
				FuncKind::Function | FuncKind::FunctionOperator(_) | FuncKind::GenericFunction => desc.cpp_name.namespace().into(),
				FuncKind::Constructor(cls)
				| FuncKind::InstanceMethod(cls)
				| FuncKind::StaticMethod(cls)
				| FuncKind::FieldAccessor(cls, _)
				| FuncKind::ConversionMethod(cls)
				| FuncKind::InstanceOperator(cls, _)
				| FuncKind::GenericInstanceMethod(cls) => cls.cpp_name(CppNameStyle::Reference).into_owned().into(),
			},
		}
	}

	fn cpp_name(&self, style: CppNameStyle) -> Cow<str> {
		let decl_name = if self.kind().as_conversion_method().is_some() {
			format!("operator {}", self.return_type_ref().cpp_name(CppNameStyle::Reference)).into()
		} else {
			match self {
				&Self::Clang { entity, .. } => DefaultElement::cpp_name(self, entity, CppNameStyle::Declaration),
				Self::Desc(desc) => desc.cpp_name.cpp_name_from_fullname(CppNameStyle::Declaration).into(),
			}
		};
		match style {
			CppNameStyle::Declaration => decl_name,
			CppNameStyle::Reference => DefaultElement::cpp_decl_name_with_namespace(self, &decl_name),
		}
	}
}

impl<'me> NameDebug<'me> for &'me Func<'_, '_> {
	fn file_line_name(self) -> LocationName<'me> {
		match self {
			Func::Clang { entity, .. } => entity.file_line_name(),
			Func::Desc(desc) => LocationName::new(desc.def_loc.clone(), self.cpp_name(CppNameStyle::Reference)),
		}
	}
}

impl fmt::Debug for Func<'_, '_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut debug_struct = f.debug_struct(match self {
			Self::Clang { .. } => "Func::Clang",
			Self::Desc(_) => "Func::Desc",
		});
		self
			.update_debug_struct(&mut debug_struct)
			.field("func_id", &self.func_id())
			.field("constness", &self.constness())
			.field("is_specialized", &self.is_specialized())
			.field("return_kind", &self.return_kind())
			.field("kind", &self.kind())
			.field("return_type", &self.return_type_ref())
			.field("arguments", &self.arguments())
			.finish()
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Safety {
	Safe,
	Unsafe,
}

impl Safety {
	pub fn from_unsafe(is_unsafe: bool) -> Safety {
		if is_unsafe {
			Self::Unsafe
		} else {
			Self::Safe
		}
	}

	pub fn is_safe(self) -> bool {
		match self {
			Self::Safe => true,
			Self::Unsafe => false,
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReturnKind {
	InfallibleNaked,
	InfallibleViaArg,
	Fallible,
}

impl ReturnKind {
	pub fn infallible(is_naked: bool) -> Self {
		if is_naked {
			Self::InfallibleNaked
		} else {
			Self::InfallibleViaArg
		}
	}

	pub fn is_infallible(&self) -> bool {
		match self {
			ReturnKind::InfallibleNaked | ReturnKind::InfallibleViaArg => true,
			ReturnKind::Fallible => false,
		}
	}

	pub fn is_naked(&self) -> bool {
		match self {
			ReturnKind::InfallibleNaked => true,
			ReturnKind::InfallibleViaArg | ReturnKind::Fallible => false,
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatorKind {
	Unsupported,
	Index,
	Add,
	Sub,
	Mul,
	Div,
	Deref,
	Apply,
	Set,
	Equals,
	NotEquals,
	GreaterThan,
	GreaterThanOrEqual,
	LessThan,
	LessThanOrEqual,
	Incr,
	Decr,
	And,
	Or,
	Xor,
	BitwiseNot,
}

impl OperatorKind {
	pub fn new(token: &str, arg_count: usize) -> Self {
		match token.trim() {
			"[]" => OperatorKind::Index,
			"+" => OperatorKind::Add,
			"-" => OperatorKind::Sub,
			"*" => {
				if arg_count == 0 {
					OperatorKind::Deref
				} else {
					OperatorKind::Mul
				}
			}
			"()" => OperatorKind::Apply,
			"=" => OperatorKind::Set,
			"/" => OperatorKind::Div,
			"==" => OperatorKind::Equals,
			"!=" => OperatorKind::NotEquals,
			">" => OperatorKind::GreaterThan,
			">=" => OperatorKind::GreaterThanOrEqual,
			"<" => OperatorKind::LessThan,
			"<=" => OperatorKind::LessThanOrEqual,
			"++" => OperatorKind::Incr,
			"--" => OperatorKind::Decr,
			"&" => OperatorKind::And,
			"|" => OperatorKind::Or,
			"^" => OperatorKind::Xor,
			"~" => OperatorKind::BitwiseNot,
			_ => OperatorKind::Unsupported,
		}
	}

	pub fn add_args_to_name(&self) -> bool {
		match self {
			OperatorKind::Index | OperatorKind::BitwiseNot | OperatorKind::Apply => false,
			OperatorKind::Unsupported
			| OperatorKind::Add
			| OperatorKind::Sub
			| OperatorKind::Mul
			| OperatorKind::Div
			| OperatorKind::Deref
			| OperatorKind::Equals
			| OperatorKind::NotEquals
			| OperatorKind::GreaterThan
			| OperatorKind::GreaterThanOrEqual
			| OperatorKind::LessThan
			| OperatorKind::LessThanOrEqual
			| OperatorKind::Incr
			| OperatorKind::Decr
			| OperatorKind::And
			| OperatorKind::Or
			| OperatorKind::Xor
			| OperatorKind::Set => true,
		}
	}
}

#[derive(Clone, Debug)]
pub enum FuncKind<'tu, 'ge> {
	Function,
	FunctionOperator(OperatorKind),
	Constructor(Class<'tu, 'ge>),
	InstanceMethod(Class<'tu, 'ge>),
	StaticMethod(Class<'tu, 'ge>),
	FieldAccessor(Class<'tu, 'ge>, Field<'tu, 'ge>),
	ConversionMethod(Class<'tu, 'ge>),
	InstanceOperator(Class<'tu, 'ge>, OperatorKind),
	GenericFunction,
	GenericInstanceMethod(Class<'tu, 'ge>),
}

impl<'tu, 'ge> FuncKind<'tu, 'ge> {
	pub fn as_instance_method(&self) -> Option<&Class<'tu, 'ge>> {
		match self {
			Self::InstanceMethod(out)
			| Self::FieldAccessor(out, _)
			| Self::GenericInstanceMethod(out)
			| Self::ConversionMethod(out)
			| Self::InstanceOperator(out, ..) => Some(out),
			_ => None,
		}
	}

	pub fn as_constructor(&self) -> Option<&Class<'tu, 'ge>> {
		if let Self::Constructor(out) = self {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_static_method(&self) -> Option<&Class<'tu, 'ge>> {
		if let Self::StaticMethod(out) = self {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_conversion_method(&self) -> Option<&Class<'tu, 'ge>> {
		if let Self::ConversionMethod(out) = self {
			Some(out)
		} else {
			None
		}
	}

	/// Any function with a connection to a class: instance method, static method or a constructor
	pub fn as_class_method(&self) -> Option<&Class<'tu, 'ge>> {
		if let Some(out) = self.as_instance_method() {
			Some(out)
		} else if let Some(out) = self.as_constructor() {
			Some(out)
		} else if let Some(out) = self.as_static_method() {
			Some(out)
		} else {
			None
		}
	}

	pub fn as_operator(&self) -> Option<(Option<&Class<'tu, 'ge>>, OperatorKind)> {
		match self {
			Self::FunctionOperator(kind) => Some((None, *kind)),
			Self::InstanceOperator(cls, kind) => Some((Some(cls), *kind)),
			_ => None,
		}
	}

	pub fn as_field_accessor(&self) -> Option<(&Class<'tu, 'ge>, &Field<'tu, 'ge>)> {
		if let FuncKind::FieldAccessor(cls, fld) = self {
			Some((cls, fld))
		} else {
			None
		}
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FuncTypeHint {
	None,
	Specialized,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FuncId<'f> {
	name: Cow<'f, str>,
	constness: Constness,
	args: Vec<Cow<'f, str>>,
}

impl<'f> FuncId<'f> {
	/// # Parameters
	/// name: fully qualified C++ function name (e.g. cv::Mat::create)
	/// args: C++ argument names ("unnamed" for unnamed ones)
	pub fn new_mut<const ARGS: usize>(name: &'static str, args: [&'static str; ARGS]) -> FuncId<'static> {
		FuncId {
			name: name.into(),
			constness: Constness::Mut,
			args: args.into_iter().map(|a| a.into()).collect(),
		}
	}

	/// # Parameters
	/// name: fully qualified C++ function name (e.g. cv::Mat::create)
	/// args: C++ argument names ("unnamed" for unnamed ones)
	pub fn new_const<const ARGS: usize>(name: &'static str, args: [&'static str; ARGS]) -> FuncId<'static> {
		FuncId {
			name: name.into(),
			constness: Constness::Const,
			args: args.into_iter().map(|a| a.into()).collect(),
		}
	}

	pub fn from_entity(entity: Entity) -> FuncId<'static> {
		let name = entity.cpp_name(CppNameStyle::Reference).into_owned().into();
		let args = if let EntityKind::FunctionTemplate = entity.get_kind() {
			let mut args = Vec::with_capacity(8);
			entity.walk_children_while(|child| {
				if child.get_kind() == EntityKind::ParmDecl {
					args.push(child.get_name().map_or_else(|| UNNAMED.into(), Cow::Owned));
				}
				WalkAction::Continue
			});
			args
		} else {
			entity
				.get_arguments()
				.into_iter()
				.flatten()
				.map(|a| a.get_name().map_or_else(|| UNNAMED.into(), Cow::Owned))
				.collect()
		};
		FuncId {
			name,
			constness: Constness::from_is_const(entity.is_const_method()),
			args,
		}
	}

	pub fn from_desc(desc: &'f FuncDesc) -> FuncId<'f> {
		let mut name = if let Some(cls) = desc.kind.as_instance_method() {
			format!("{}::", cls.cpp_name(CppNameStyle::Reference))
		} else {
			"".to_string()
		};
		name.push_str(desc.cpp_name.as_ref());
		let args = desc
			.arguments
			.iter()
			.map(|arg| arg.cpp_name(CppNameStyle::Declaration))
			.collect();

		FuncId {
			name: name.into(),
			constness: desc.constness,
			args,
		}
	}

	pub fn make_static(self) -> FuncId<'static> {
		FuncId {
			name: self.name.into_owned().into(),
			args: self.args.into_iter().map(|arg| arg.into_owned().into()).collect(),
			constness: self.constness,
		}
	}

	pub fn name(&self) -> &str {
		self.name.as_ref()
	}

	pub fn args(&self) -> &[Cow<str>] {
		&self.args
	}
}

impl fmt::Display for FuncId<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{cnst}{name}({args})",
			cnst = self.constness.rust_qual_ptr(),
			name = self.name,
			args = self.args.join(", ")
		)
	}
}

#[derive(Debug, Clone, Copy)]
pub struct InheritConfig {
	pub kind: bool,
	pub name: bool,
	pub doc_comment: bool,
	pub arguments: bool,
	pub return_type_ref: bool,
	pub definition_location: bool,
}

impl InheritConfig {
	pub const fn empty() -> Self {
		Self {
			kind: false,
			name: false,
			doc_comment: false,
			arguments: false,
			return_type_ref: false,
			definition_location: false,
		}
	}

	pub fn kind(mut self) -> Self {
		self.kind = true;
		self
	}

	pub fn with_name(mut self) -> Self {
		self.name = true;
		self
	}

	pub fn doc_comment(mut self) -> Self {
		self.doc_comment = true;
		self
	}

	pub fn arguments(mut self) -> Self {
		self.arguments = true;
		self
	}

	pub fn return_type_ref(mut self) -> Self {
		self.return_type_ref = true;
		self
	}

	pub fn definition_location(mut self) -> Self {
		self.definition_location = true;
		self
	}

	pub fn any_enabled(self) -> bool {
		self.kind || self.name || self.arguments || self.doc_comment || self.return_type_ref || self.definition_location
	}
}
