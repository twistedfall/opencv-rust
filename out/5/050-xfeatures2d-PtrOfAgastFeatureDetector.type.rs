ptr_extern! { crate::xfeatures2d::AgastFeatureDetector,
	cv_PtrLcv_xfeatures2d_AgastFeatureDetectorG_new_null_const, cv_PtrLcv_xfeatures2d_AgastFeatureDetectorG_delete, cv_PtrLcv_xfeatures2d_AgastFeatureDetectorG_getInnerPtr_const, cv_PtrLcv_xfeatures2d_AgastFeatureDetectorG_getInnerPtrMut
}

impl core::Ptr<crate::xfeatures2d::AgastFeatureDetector> {
	#[inline] pub fn as_raw_PtrOfAgastFeatureDetector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAgastFeatureDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xfeatures2d::AgastFeatureDetectorTraitConst for core::Ptr<crate::xfeatures2d::AgastFeatureDetector> {
	#[inline] fn as_raw_AgastFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::AgastFeatureDetectorTrait for core::Ptr<crate::xfeatures2d::AgastFeatureDetector> {
	#[inline] fn as_raw_mut_AgastFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::AgastFeatureDetector> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::AgastFeatureDetector> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::AgastFeatureDetector>, core::Ptr<core::Algorithm>, cv_PtrLcv_xfeatures2d_AgastFeatureDetectorG_to_PtrOfAlgorithm }

impl crate::features::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::AgastFeatureDetector> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features::Feature2DTrait for core::Ptr<crate::xfeatures2d::AgastFeatureDetector> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::AgastFeatureDetector>, core::Ptr<crate::features::Feature2D>, cv_PtrLcv_xfeatures2d_AgastFeatureDetectorG_to_PtrOfFeature2D }

impl std::fmt::Debug for core::Ptr<crate::xfeatures2d::AgastFeatureDetector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfAgastFeatureDetector")
			.finish()
	}
}

