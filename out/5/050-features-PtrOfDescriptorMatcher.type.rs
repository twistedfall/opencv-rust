ptr_extern! { crate::features::DescriptorMatcher,
	cv_PtrLcv_DescriptorMatcherG_new_null_const, cv_PtrLcv_DescriptorMatcherG_delete, cv_PtrLcv_DescriptorMatcherG_getInnerPtr_const, cv_PtrLcv_DescriptorMatcherG_getInnerPtrMut
}

impl core::Ptr<crate::features::DescriptorMatcher> {
	#[inline] pub fn as_raw_PtrOfDescriptorMatcher(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDescriptorMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::features::DescriptorMatcherTraitConst for core::Ptr<crate::features::DescriptorMatcher> {
	#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features::DescriptorMatcherTrait for core::Ptr<crate::features::DescriptorMatcher> {
	#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::features::DescriptorMatcher> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::features::DescriptorMatcher> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::features::DescriptorMatcher>, core::Ptr<core::Algorithm>, cv_PtrLcv_DescriptorMatcherG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::features::DescriptorMatcher> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDescriptorMatcher")
			.finish()
	}
}

