ptr_extern! { crate::line_descriptor::BinaryDescriptorMatcher,
	cv_PtrLcv_line_descriptor_BinaryDescriptorMatcherG_new_null_const, cv_PtrLcv_line_descriptor_BinaryDescriptorMatcherG_delete, cv_PtrLcv_line_descriptor_BinaryDescriptorMatcherG_getInnerPtr_const, cv_PtrLcv_line_descriptor_BinaryDescriptorMatcherG_getInnerPtrMut
}

ptr_extern_ctor! { crate::line_descriptor::BinaryDescriptorMatcher, cv_PtrLcv_line_descriptor_BinaryDescriptorMatcherG_new_const_BinaryDescriptorMatcher }
impl core::Ptr<crate::line_descriptor::BinaryDescriptorMatcher> {
	#[inline] pub fn as_raw_PtrOfBinaryDescriptorMatcher(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBinaryDescriptorMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::line_descriptor::BinaryDescriptorMatcherTraitConst for core::Ptr<crate::line_descriptor::BinaryDescriptorMatcher> {
	#[inline] fn as_raw_BinaryDescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::line_descriptor::BinaryDescriptorMatcherTrait for core::Ptr<crate::line_descriptor::BinaryDescriptorMatcher> {
	#[inline] fn as_raw_mut_BinaryDescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::line_descriptor::BinaryDescriptorMatcher> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::line_descriptor::BinaryDescriptorMatcher> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::line_descriptor::BinaryDescriptorMatcher>, core::Ptr<core::Algorithm>, cv_PtrLcv_line_descriptor_BinaryDescriptorMatcherG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::line_descriptor::BinaryDescriptorMatcher> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBinaryDescriptorMatcher")
			.finish()
	}
}

