use crate::GeneratedType;

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

impl<'ne, 'tu: 'ne, 'ge: 'ne> AsRef<dyn RustNativeGeneratedElement + 'ne> for GeneratedType<'tu, 'ge> {
	fn as_ref(&self) -> &(dyn RustNativeGeneratedElement + 'ne) {
		match self {
			GeneratedType::AbstractRefWrapper(r) => r,
			GeneratedType::Vector(vec) => vec,
			GeneratedType::SmartPtr(ptr) => ptr,
			GeneratedType::Tuple(tuple) => tuple,
		}
	}
}
