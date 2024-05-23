ptr_extern! { crate::features2d::BFMatcher,
	cv_PtrLcv_BFMatcherG_new_null_const, cv_PtrLcv_BFMatcherG_delete, cv_PtrLcv_BFMatcherG_getInnerPtr_const, cv_PtrLcv_BFMatcherG_getInnerPtrMut
}

ptr_extern_ctor! { crate::features2d::BFMatcher, cv_PtrLcv_BFMatcherG_new_const_BFMatcher }
impl core::Ptr<crate::features2d::BFMatcher> {
	#[inline] pub fn as_raw_PtrOfBFMatcher(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBFMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::features2d::BFMatcherTraitConst for core::Ptr<crate::features2d::BFMatcher> {
	#[inline] fn as_raw_BFMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::BFMatcherTrait for core::Ptr<crate::features2d::BFMatcher> {
	#[inline] fn as_raw_mut_BFMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::BFMatcher> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::features2d::BFMatcher> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::features2d::BFMatcher>, core::Ptr<core::Algorithm>, cv_PtrLcv_BFMatcherG_to_PtrOfAlgorithm }

impl crate::features2d::DescriptorMatcherTraitConst for core::Ptr<crate::features2d::BFMatcher> {
	#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::DescriptorMatcherTrait for core::Ptr<crate::features2d::BFMatcher> {
	#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::features2d::BFMatcher>, core::Ptr<crate::features2d::DescriptorMatcher>, cv_PtrLcv_BFMatcherG_to_PtrOfDescriptorMatcher }

impl std::fmt::Debug for core::Ptr<crate::features2d::BFMatcher> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBFMatcher")
			.finish()
	}
}

