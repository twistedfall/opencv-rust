ptr_extern! { crate::xfeatures2d::FREAK,
	cv_PtrLcv_xfeatures2d_FREAKG_new_null_const, cv_PtrLcv_xfeatures2d_FREAKG_delete, cv_PtrLcv_xfeatures2d_FREAKG_getInnerPtr_const, cv_PtrLcv_xfeatures2d_FREAKG_getInnerPtrMut
}

impl core::Ptr<crate::xfeatures2d::FREAK> {
	#[inline] pub fn as_raw_PtrOfFREAK(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFREAK(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xfeatures2d::FREAKTraitConst for core::Ptr<crate::xfeatures2d::FREAK> {
	#[inline] fn as_raw_FREAK(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::FREAKTrait for core::Ptr<crate::xfeatures2d::FREAK> {
	#[inline] fn as_raw_mut_FREAK(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::FREAK> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::FREAK> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::FREAK>, core::Ptr<core::Algorithm>, cv_PtrLcv_xfeatures2d_FREAKG_to_PtrOfAlgorithm }

impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::FREAK> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::FREAK> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::FREAK>, core::Ptr<crate::features2d::Feature2D>, cv_PtrLcv_xfeatures2d_FREAKG_to_PtrOfFeature2D }

impl std::fmt::Debug for core::Ptr<crate::xfeatures2d::FREAK> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFREAK")
			.finish()
	}
}

