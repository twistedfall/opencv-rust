ptr_extern! { crate::xfeatures2d::HarrisLaplaceFeatureDetector,
	cv_PtrLcv_xfeatures2d_HarrisLaplaceFeatureDetectorG_new_null_const, cv_PtrLcv_xfeatures2d_HarrisLaplaceFeatureDetectorG_delete, cv_PtrLcv_xfeatures2d_HarrisLaplaceFeatureDetectorG_getInnerPtr_const, cv_PtrLcv_xfeatures2d_HarrisLaplaceFeatureDetectorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::xfeatures2d::HarrisLaplaceFeatureDetector, cv_PtrLcv_xfeatures2d_HarrisLaplaceFeatureDetectorG_new_const_HarrisLaplaceFeatureDetector }
impl core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector> {
	#[inline] pub fn as_raw_PtrOfHarrisLaplaceFeatureDetector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfHarrisLaplaceFeatureDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xfeatures2d::HarrisLaplaceFeatureDetectorTraitConst for core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector> {
	#[inline] fn as_raw_HarrisLaplaceFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::HarrisLaplaceFeatureDetectorTrait for core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector> {
	#[inline] fn as_raw_mut_HarrisLaplaceFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector>, core::Ptr<core::Algorithm>, cv_PtrLcv_xfeatures2d_HarrisLaplaceFeatureDetectorG_to_PtrOfAlgorithm }

impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector>, core::Ptr<crate::features2d::Feature2D>, cv_PtrLcv_xfeatures2d_HarrisLaplaceFeatureDetectorG_to_PtrOfFeature2D }

impl std::fmt::Debug for core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfHarrisLaplaceFeatureDetector")
			.finish()
	}
}

