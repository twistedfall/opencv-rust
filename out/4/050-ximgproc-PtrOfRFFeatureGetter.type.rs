ptr_extern! { crate::ximgproc::RFFeatureGetter,
	cv_PtrLcv_ximgproc_RFFeatureGetterG_new_null_const, cv_PtrLcv_ximgproc_RFFeatureGetterG_delete, cv_PtrLcv_ximgproc_RFFeatureGetterG_getInnerPtr_const, cv_PtrLcv_ximgproc_RFFeatureGetterG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::RFFeatureGetter> {
	#[inline] pub fn as_raw_PtrOfRFFeatureGetter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRFFeatureGetter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::RFFeatureGetterTraitConst for core::Ptr<crate::ximgproc::RFFeatureGetter> {
	#[inline] fn as_raw_RFFeatureGetter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::RFFeatureGetterTrait for core::Ptr<crate::ximgproc::RFFeatureGetter> {
	#[inline] fn as_raw_mut_RFFeatureGetter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::RFFeatureGetter> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::RFFeatureGetter> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::RFFeatureGetter>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_RFFeatureGetterG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::RFFeatureGetter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfRFFeatureGetter")
			.finish()
	}
}

