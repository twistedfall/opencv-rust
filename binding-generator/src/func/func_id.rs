use std::borrow::Cow;
use std::fmt;

use clang::{Entity, EntityKind};

use crate::element::UNNAMED;
use crate::func::FuncDesc;
use crate::type_ref::Constness;
use crate::{CppNameStyle, Element, EntityExt, WalkAction};

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
