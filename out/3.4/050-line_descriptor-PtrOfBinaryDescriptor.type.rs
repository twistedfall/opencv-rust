ptr_extern! { crate::line_descriptor::BinaryDescriptor,
	cv_PtrLcv_line_descriptor_BinaryDescriptorG_new_null_const, cv_PtrLcv_line_descriptor_BinaryDescriptorG_delete, cv_PtrLcv_line_descriptor_BinaryDescriptorG_getInnerPtr_const, cv_PtrLcv_line_descriptor_BinaryDescriptorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::line_descriptor::BinaryDescriptor, cv_PtrLcv_line_descriptor_BinaryDescriptorG_new_const_BinaryDescriptor }
impl core::Ptr<crate::line_descriptor::BinaryDescriptor> {
	#[inline] pub fn as_raw_PtrOfBinaryDescriptor(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBinaryDescriptor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::line_descriptor::BinaryDescriptorTraitConst for core::Ptr<crate::line_descriptor::BinaryDescriptor> {
	#[inline] fn as_raw_BinaryDescriptor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::line_descriptor::BinaryDescriptorTrait for core::Ptr<crate::line_descriptor::BinaryDescriptor> {
	#[inline] fn as_raw_mut_BinaryDescriptor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::line_descriptor::BinaryDescriptor> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::line_descriptor::BinaryDescriptor> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::line_descriptor::BinaryDescriptor>, core::Ptr<core::Algorithm>, cv_PtrLcv_line_descriptor_BinaryDescriptorG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::line_descriptor::BinaryDescriptor> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBinaryDescriptor")
			.finish()
	}
}

