use crate::field::Field;
use crate::Class;

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
