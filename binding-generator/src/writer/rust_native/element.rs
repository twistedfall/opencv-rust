use crate::DependentType;

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

impl RustNativeGeneratedElement for DependentType<'_, '_> {
	fn element_order(&self) -> u8 {
		match self {
			DependentType::ReturnTypeWrapper(ret) => ret.element_order(),
			DependentType::AbstractRefWrapper(r) => r.element_order(),
			DependentType::Vector(vec) => vec.element_order(),
			DependentType::SmartPtr(ptr) => ptr.element_order(),
		}
	}

	fn element_safe_id(&self) -> String {
		match self {
			DependentType::ReturnTypeWrapper(ret) => ret.element_safe_id(),
			DependentType::AbstractRefWrapper(r) => r.element_safe_id(),
			DependentType::Vector(vec) => vec.element_safe_id(),
			DependentType::SmartPtr(ptr) => ptr.element_safe_id(),
		}
	}

	fn gen_rust(&self, opencv_version: &str) -> String {
		match self {
			DependentType::ReturnTypeWrapper(ret) => ret.gen_rust(opencv_version),
			DependentType::AbstractRefWrapper(r) => r.gen_rust(opencv_version),
			DependentType::Vector(vec) => vec.gen_rust(opencv_version),
			DependentType::SmartPtr(ptr) => ptr.gen_rust(opencv_version),
		}
	}

	fn gen_rust_exports(&self) -> String {
		match self {
			DependentType::ReturnTypeWrapper(ret) => ret.gen_rust_exports(),
			DependentType::AbstractRefWrapper(r) => r.gen_rust_exports(),
			DependentType::Vector(vec) => vec.gen_rust_exports(),
			DependentType::SmartPtr(ptr) => ptr.gen_rust_exports(),
		}
	}

	fn gen_cpp(&self) -> String {
		match self {
			DependentType::ReturnTypeWrapper(ret) => ret.gen_cpp(),
			DependentType::AbstractRefWrapper(r) => r.gen_cpp(),
			DependentType::Vector(vec) => vec.gen_cpp(),
			DependentType::SmartPtr(ptr) => ptr.gen_cpp(),
		}
	}
}
