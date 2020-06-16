use crate::{
	AbstractRefWrapper,
	DependentType,
	ReturnTypeWrapper,
	SmartPtr,
	Vector,
};

pub trait RustNativeGeneratedElement {
	/// Element order in the output file, lower means earlier
	fn element_order(&self) -> u8 {
		50
	}

	fn element_safe_id(&self) -> String;

	fn gen_rust(&self, _opencv_version: &str) -> String {
		"".to_string()
	}

	fn gen_rust_exports(&self) -> String {
		"".to_string()
	}

	fn gen_cpp(&self) -> String {
		"".to_string()
	}
}

pub enum DepType<'tu> {
	ReturnTypeWrapper(ReturnTypeWrapper<'tu>),
	AbstractRefWrapper(AbstractRefWrapper<'tu>),
	Vector(Vector<'tu>),
	SmartPtr(SmartPtr<'tu>),
}

impl<'tu> DependentType<'tu> for DepType<'tu> {
	fn from_return_type_wrapper(s: ReturnTypeWrapper<'tu>) -> Self {
		DepType::ReturnTypeWrapper(s)
	}

	fn from_abstract_ref_wrapper(s: AbstractRefWrapper<'tu>) -> Self {
		DepType::AbstractRefWrapper(s)
	}

	fn from_vector(s: Vector<'tu>) -> Self {
		DepType::Vector(s)
	}

	fn from_smart_ptr(s: SmartPtr<'tu>) -> Self {
		DepType::SmartPtr(s)
	}
}

impl RustNativeGeneratedElement for DepType<'_> {
	fn element_order(&self) -> u8 {
		match self {
			DepType::ReturnTypeWrapper(ret) => ret.element_order(),
			DepType::AbstractRefWrapper(r) => r.element_order(),
			DepType::Vector(vec) => vec.element_order(),
			DepType::SmartPtr(ptr) => ptr.element_order(),
		}
	}

	fn element_safe_id(&self) -> String {
		match self {
			DepType::ReturnTypeWrapper(ret) => ret.element_safe_id(),
			DepType::AbstractRefWrapper(r) => r.element_safe_id(),
			DepType::Vector(vec) => vec.element_safe_id(),
			DepType::SmartPtr(ptr) => ptr.element_safe_id(),
		}
	}

	fn gen_rust(&self, opencv_version: &str) -> String {
		match self {
			DepType::ReturnTypeWrapper(ret) => ret.gen_rust(opencv_version),
			DepType::AbstractRefWrapper(r) => r.gen_rust(opencv_version),
			DepType::Vector(vec) => vec.gen_rust(opencv_version),
			DepType::SmartPtr(ptr) => ptr.gen_rust(opencv_version),
		}
	}

	fn gen_rust_exports(&self) -> String {
		match self {
			DepType::ReturnTypeWrapper(ret) => ret.gen_rust_exports(),
			DepType::AbstractRefWrapper(r) => r.gen_rust_exports(),
			DepType::Vector(vec) => vec.gen_rust_exports(),
			DepType::SmartPtr(ptr) => ptr.gen_rust_exports(),
		}
	}

	fn gen_cpp(&self) -> String {
		match self {
			DepType::ReturnTypeWrapper(ret) => ret.gen_cpp(),
			DepType::AbstractRefWrapper(r) => r.gen_cpp(),
			DepType::Vector(vec) => vec.gen_cpp(),
			DepType::SmartPtr(ptr) => ptr.gen_cpp(),
		}
	}
}
