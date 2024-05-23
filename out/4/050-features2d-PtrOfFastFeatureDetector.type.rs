ptr_extern! { crate::features2d::FastFeatureDetector,
	cv_PtrLcv_FastFeatureDetectorG_new_null_const, cv_PtrLcv_FastFeatureDetectorG_delete, cv_PtrLcv_FastFeatureDetectorG_getInnerPtr_const, cv_PtrLcv_FastFeatureDetectorG_getInnerPtrMut
}

impl core::Ptr<crate::features2d::FastFeatureDetector> {
	#[inline] pub fn as_raw_PtrOfFastFeatureDetector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFastFeatureDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::features2d::FastFeatureDetectorTraitConst for core::Ptr<crate::features2d::FastFeatureDetector> {
	#[inline] fn as_raw_FastFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::FastFeatureDetectorTrait for core::Ptr<crate::features2d::FastFeatureDetector> {
	#[inline] fn as_raw_mut_FastFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::FastFeatureDetector> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::features2d::FastFeatureDetector> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::features2d::FastFeatureDetector>, core::Ptr<core::Algorithm>, cv_PtrLcv_FastFeatureDetectorG_to_PtrOfAlgorithm }

impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::features2d::FastFeatureDetector> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for core::Ptr<crate::features2d::FastFeatureDetector> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::features2d::FastFeatureDetector>, core::Ptr<crate::features2d::Feature2D>, cv_PtrLcv_FastFeatureDetectorG_to_PtrOfFeature2D }

impl std::fmt::Debug for core::Ptr<crate::features2d::FastFeatureDetector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFastFeatureDetector")
			.finish()
	}
}

