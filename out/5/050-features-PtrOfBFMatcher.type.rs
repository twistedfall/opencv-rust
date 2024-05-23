ptr_extern! { crate::features::BFMatcher,
	cv_PtrLcv_BFMatcherG_new_null_const, cv_PtrLcv_BFMatcherG_delete, cv_PtrLcv_BFMatcherG_getInnerPtr_const, cv_PtrLcv_BFMatcherG_getInnerPtrMut
}

ptr_extern_ctor! { crate::features::BFMatcher, cv_PtrLcv_BFMatcherG_new_const_BFMatcher }
impl core::Ptr<crate::features::BFMatcher> {
	#[inline] pub fn as_raw_PtrOfBFMatcher(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBFMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::features::BFMatcherTraitConst for core::Ptr<crate::features::BFMatcher> {
	#[inline] fn as_raw_BFMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features::BFMatcherTrait for core::Ptr<crate::features::BFMatcher> {
	#[inline] fn as_raw_mut_BFMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::features::BFMatcher> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::features::BFMatcher> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::features::BFMatcher>, core::Ptr<core::Algorithm>, cv_PtrLcv_BFMatcherG_to_PtrOfAlgorithm }

impl crate::features::DescriptorMatcherTraitConst for core::Ptr<crate::features::BFMatcher> {
	#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features::DescriptorMatcherTrait for core::Ptr<crate::features::BFMatcher> {
	#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::features::BFMatcher>, core::Ptr<crate::features::DescriptorMatcher>, cv_PtrLcv_BFMatcherG_to_PtrOfDescriptorMatcher }

impl std::fmt::Debug for core::Ptr<crate::features::BFMatcher> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBFMatcher")
			.finish()
	}
}

