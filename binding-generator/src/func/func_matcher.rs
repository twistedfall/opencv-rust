use std::borrow::Cow;
use std::borrow::Cow::Owned;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::ops::ControlFlow;
use std::sync::RwLock;

use clang::{Entity, EntityKind};

use crate::element::UNNAMED;
use crate::func::FuncDesc;
use crate::field::Field;
use crate::type_ref::Constness;
use crate::{CowMapBorrowedExt, CppNameStyle, Element, EntityExt, Func, IteratorExt};

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
		let args = if let EntityKind::FunctionTemplate = entity.get_kind() {
			let mut args = Vec::with_capacity(8);
			entity.walk_children_while(|child| {
				if child.get_kind() == EntityKind::ParmDecl {
					args.push(child.get_name().map_or(UNNAMED.into(), Cow::Owned));
				}
				ControlFlow::Continue(())
			});
			args
		} else {
			entity
				.get_arguments()
				.into_iter()
				.flatten()
				.map(|a| a.get_name().map_or(UNNAMED.into(), Cow::Owned))
				.collect()
		};
		FuncId {
			name: entity.cpp_name(CppNameStyle::Reference).into_owned().into(),
			constness: Constness::from_is_const(entity.is_const_method()),
			args,
		}
	}

	pub fn from_desc(desc: &'f FuncDesc) -> FuncId<'f> {
		let mut name = desc.kind.as_instance_method().map_or_else(
			|| "".to_string(),
			|cls| format!("{}::", cls.cpp_name(CppNameStyle::Reference)),
		);
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
			"FuncId::new{cnst}(\"{name}\", [{args}])",
			cnst = self.constness.rust_function_name_qual(),
			name = self.name,
			args = self.args.iter().map(|a| format!("\"{a}\"")).join(", ")
		)
	}
}

#[derive(Debug)]
pub struct FuncMatchProperties<'f> {
	func: &'f Func<'f, 'f>,
	name: Cow<'f, str>,
	constness: Option<Constness>,
	ret: Option<Cow<'f, str>>,
	arg_names: Option<Vec<Cow<'f, str>>>,
	arg_types: Option<Vec<Cow<'f, str>>>,
}

impl<'f> FuncMatchProperties<'f> {
	pub fn new(func: &'f Func<'f, 'f>, name: Cow<'f, str>) -> Self {
		Self {
			func,
			name,
			constness: None,
			ret: None,
			arg_names: None,
			arg_types: None,
		}
	}

	pub fn name(&mut self) -> &str {
		self.name.as_ref()
	}

	pub fn constness(&mut self) -> Constness {
		*self.constness.get_or_insert_with(|| self.func.constness())
	}

	pub fn return_type(&mut self) -> &str {
		self.ret.get_or_insert_with(|| {
			self
				.func
				.return_type_ref()
				.map_borrowed(|ret| ret.cpp_name(CppNameStyle::Reference))
		})
	}

	pub fn arg_names(&mut self) -> &[Cow<str>] {
		self
			.arg_names
			.get_or_insert_with(|| {
				match &self.func {
					Func::Clang { entity, .. } => {
						self
							.func
							.clang_arguments(*entity)
							.iter()
							.map(|arg| Owned(arg.cpp_name(CppNameStyle::Declaration).into_owned())) // todo: find a way to store borrowed
							.collect()
					}
					Func::Desc(desc) => desc
						.arguments
						.iter()
						.map(|arg| arg.cpp_name(CppNameStyle::Declaration))
						.collect(),
				}
			})
			.as_slice()
	}

	pub fn arg_types(&mut self) -> &[Cow<str>] {
		self
			.arg_types
			.get_or_insert_with(|| match &self.func {
				Func::Clang { entity, gen_env, .. } => self
					.func
					.clang_arguments(*entity)
					.iter()
					.map(|arg| {
						Owned(
							Field::new(*arg, gen_env)
								.type_ref()
								.cpp_name(CppNameStyle::Reference)
								.into_owned(),
						)
					})
					.collect(),
				Func::Desc(desc) => desc
					.arguments
					.iter()
					.map(|arg| {
						arg.type_ref()
							.map_borrowed(|type_ref| type_ref.cpp_name(CppNameStyle::Reference))
					})
					.collect(),
			})
			.as_slice()
	}

	pub fn dump(&mut self) -> String {
		let constness = self.constness();
		let name = self.name().to_string();
		let arg_names = self.arg_names().iter().map(|s| s.to_string()).collect::<Vec<_>>();
		let arg_types = self.arg_types().iter().map(|s| s.to_string()).collect::<Vec<_>>();
		format!(
			"(\"{name}\", vec![(pred!({cnst}, {arg_names:?}, {arg_types:?}), _)]),",
			cnst = constness.rust_qual_ptr().trim()
		)
	}
}

pub type FuncMatcherInner<'l, RES> = HashMap<&'l str, Vec<(&'l [Pred<'l>], RES)>>;

pub type UsageTracker<'l> = (&'l str, &'l [Pred<'l>]);

#[derive(Debug)]
pub struct FuncMatcher<'l, RES> {
	inner: FuncMatcherInner<'l, RES>,
	usage_tracking: Option<RwLock<HashSet<UsageTracker<'l>>>>,
}

impl<'l, RES> FuncMatcher<'l, RES> {
	pub fn empty() -> Self {
		Self {
			inner: HashMap::new(),
			usage_tracking: None,
		}
	}

	pub fn create(inner: FuncMatcherInner<'l, RES>) -> Self {
		Self {
			inner,
			usage_tracking: None,
		}
	}

	pub fn get(&self, f: &mut FuncMatchProperties) -> Option<&RES> {
		let mtch = self.inner.get(f.name()).and_then(|matchers| {
			matchers
				.iter()
				.find_map(|(preds, res)| preds.iter().all(|m| m.matches(f)).then_some((preds, res)))
		});
		if let Some((preds, res)) = mtch {
			if let Some(usage_tracking) = self.usage_tracking.as_ref() {
				let needs_removal = if let Ok(usage_tracking) = usage_tracking.read() {
					usage_tracking.contains(&(f.name(), *preds))
				} else {
					false
				};
				if needs_removal {
					if let Ok(mut usage_tracking) = usage_tracking.write() {
						usage_tracking.retain(|x| x != &(f.name(), *preds));
					}
				}
			}
			Some(res)
		} else {
			None
		}
	}

	pub fn start_usage_tracking(&mut self) {
		if !self.inner.is_empty() {
			let mut usage_tracking = HashSet::new();
			for (name, matchers) in &self.inner {
				for (predicates, _) in matchers {
					usage_tracking.insert((*name, *predicates));
				}
			}
			self.usage_tracking = Some(RwLock::new(usage_tracking));
		}
	}

	pub fn finish_usage_tracking(&mut self) -> HashSet<UsageTracker> {
		if let Some(out) = self.usage_tracking.take() {
			if let Ok(usage_tracking) = out.into_inner() {
				if !usage_tracking.is_empty() {
					return usage_tracking;
				}
			}
		}
		HashSet::new()
	}
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Pred<'l> {
	Constness(Constness),
	Return(&'l str),
	ArgNames(&'l [&'l str]),
	ArgTypes(&'l [&'l str]),
}

impl Pred<'_> {
	pub fn matches(&self, f: &mut FuncMatchProperties) -> bool {
		match self {
			Self::Constness(cnst) => f.constness() == *cnst,
			Self::Return(ret_type) => f.return_type() == *ret_type,
			Self::ArgNames(arg_names) => f.arg_names() == *arg_names,
			Self::ArgTypes(arg_types) => f.arg_types() == *arg_types,
		}
	}
}

#[cfg(test)]
mod test {
	use std::collections::{HashMap, HashSet};

	use crate::class::ClassDesc;
	use crate::field::{Field, FieldDesc};
	use crate::func::func_matcher::{FuncMatcher, Pred};
	use crate::func::{FuncDesc, FuncKind, ReturnKind};
	use crate::type_ref::{Constness, TypeRef, TypeRefDesc, TypeRefTypeHint};
	use crate::writer::rust_native::type_ref::Lifetime;
	use crate::Func;

	#[test]
	fn test_func_matcher() {
		let f = Func::new_desc(
			FuncDesc::new(
				FuncKind::Constructor(ClassDesc::cv_input_array()),
				Constness::Mut,
				ReturnKind::Fallible,
				"_InputArray",
				"core",
				[
					Field::new_desc(FieldDesc::new(
						"vec",
						TypeRef::new_array(TypeRefDesc::uchar().with_inherent_constness(Constness::Const), None),
					)),
					Field::new_desc(FieldDesc::new(
						"n",
						TypeRefDesc::int().with_type_hint(TypeRefTypeHint::LenForSlice(["vec".to_string()].as_slice().into(), 1)),
					)),
				],
				TypeRefDesc::cv_input_array()
					.with_inherent_constness(Constness::Const)
					.with_type_hint(TypeRefTypeHint::BoxedAsRef(Constness::Const, "vec", Lifetime::Elided)),
			)
			.rust_custom_leafname("from_byte_slice"),
		);

		// match with all predicates
		{
			let matcher = FuncMatcher::create(HashMap::from([(
				"cv::_InputArray::_InputArray",
				vec![(
					[
						Pred::Return("const cv::_InputArray"),
						Pred::ArgNames(&["vec", "n"]),
						Pred::ArgTypes(&["const unsigned char*", "int"]),
						Pred::Constness(Constness::Mut),
					]
					.as_slice(),
					"MATCH",
				)],
			)]));

			let mut f_matcher = f.matcher();
			let res = matcher.get(&mut f_matcher);
			assert_eq!(Some(&"MATCH"), res);
		}

		// match with limited predicates
		{
			let matcher = FuncMatcher::create(HashMap::from([(
				"cv::_InputArray::_InputArray",
				vec![(
					[Pred::ArgNames(&["vec", "n"]), Pred::Constness(Constness::Mut)].as_slice(),
					"MATCH",
				)],
			)]));

			let mut f_matcher = f.matcher();
			let res = matcher.get(&mut f_matcher);
			assert_eq!(Some(&"MATCH"), res);
		}

		// no match with limited predicates
		{
			let matcher = FuncMatcher::create(HashMap::from([(
				"cv::_InputArray::_InputArray",
				vec![(
					[Pred::ArgNames(&["vec", "notN"]), Pred::Constness(Constness::Mut)].as_slice(),
					"MATCH",
				)],
			)]));

			let mut f_matcher = f.matcher();
			let res = matcher.get(&mut f_matcher);
			assert_eq!(None, res);
		}
	}

	#[test]
	fn test_func_matcher_usage_tracking() {
		let f = Func::new_desc(
			FuncDesc::new(
				FuncKind::Constructor(ClassDesc::cv_input_array()),
				Constness::Mut,
				ReturnKind::Fallible,
				"_InputArray",
				"core",
				[
					Field::new_desc(FieldDesc::new(
						"vec",
						TypeRef::new_array(TypeRefDesc::uchar().with_inherent_constness(Constness::Const), None),
					)),
					Field::new_desc(FieldDesc::new(
						"n",
						TypeRefDesc::int().with_type_hint(TypeRefTypeHint::LenForSlice(["vec".to_string()].as_slice().into(), 1)),
					)),
				],
				TypeRefDesc::cv_input_array()
					.with_inherent_constness(Constness::Const)
					.with_type_hint(TypeRefTypeHint::BoxedAsRef(Constness::Const, "vec", Lifetime::Elided)),
			)
			.rust_custom_leafname("from_byte_slice"),
		);

		// with match
		{
			let mut matcher = FuncMatcher::create(HashMap::from([(
				"cv::_InputArray::_InputArray",
				vec![(
					[
						Pred::Return("const cv::_InputArray"),
						Pred::ArgNames(&["vec", "n"]),
						Pred::ArgTypes(&["const unsigned char*", "int"]),
						Pred::Constness(Constness::Mut),
					]
					.as_slice(),
					"MATCH",
				)],
			)]));
			matcher.start_usage_tracking();
			let mut f_matcher = f.matcher();
			matcher.get(&mut f_matcher);
			let usage_tracking = matcher.finish_usage_tracking();
			assert!(usage_tracking.is_empty());
		}

		// no match
		{
			let mut matcher = FuncMatcher::create(HashMap::from([(
				"cv::_InputArray::_InputArray",
				vec![([Pred::ArgNames(&["vec", "notN"])].as_slice(), "MATCH")],
			)]));
			matcher.start_usage_tracking();
			let mut f_matcher = f.matcher();
			matcher.get(&mut f_matcher);
			let usage_tracking = matcher.finish_usage_tracking();
			assert_eq!(
				HashSet::from([("cv::_InputArray::_InputArray", [Pred::ArgNames(&["vec", "notN"])].as_slice())]),
				usage_tracking
			);
		}
	}
}
