ptr_extern! { crate::features::FlannBasedMatcher,
	cv_PtrLcv_FlannBasedMatcherG_new_null_const, cv_PtrLcv_FlannBasedMatcherG_delete, cv_PtrLcv_FlannBasedMatcherG_getInnerPtr_const, cv_PtrLcv_FlannBasedMatcherG_getInnerPtrMut
}

ptr_extern_ctor! { crate::features::FlannBasedMatcher, cv_PtrLcv_FlannBasedMatcherG_new_const_FlannBasedMatcher }
impl core::Ptr<crate::features::FlannBasedMatcher> {
	#[inline] pub fn as_raw_PtrOfFlannBasedMatcher(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFlannBasedMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::features::FlannBasedMatcherTraitConst for core::Ptr<crate::features::FlannBasedMatcher> {
	#[inline] fn as_raw_FlannBasedMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features::FlannBasedMatcherTrait for core::Ptr<crate::features::FlannBasedMatcher> {
	#[inline] fn as_raw_mut_FlannBasedMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::features::FlannBasedMatcher> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::features::FlannBasedMatcher> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::features::FlannBasedMatcher>, core::Ptr<core::Algorithm>, cv_PtrLcv_FlannBasedMatcherG_to_PtrOfAlgorithm }

impl crate::features::DescriptorMatcherTraitConst for core::Ptr<crate::features::FlannBasedMatcher> {
	#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features::DescriptorMatcherTrait for core::Ptr<crate::features::FlannBasedMatcher> {
	#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::features::FlannBasedMatcher>, core::Ptr<crate::features::DescriptorMatcher>, cv_PtrLcv_FlannBasedMatcherG_to_PtrOfDescriptorMatcher }

impl std::fmt::Debug for core::Ptr<crate::features::FlannBasedMatcher> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFlannBasedMatcher")
			.finish()
	}
}

