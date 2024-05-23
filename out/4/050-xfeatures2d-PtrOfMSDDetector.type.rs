ptr_extern! { crate::xfeatures2d::MSDDetector,
	cv_PtrLcv_xfeatures2d_MSDDetectorG_new_null_const, cv_PtrLcv_xfeatures2d_MSDDetectorG_delete, cv_PtrLcv_xfeatures2d_MSDDetectorG_getInnerPtr_const, cv_PtrLcv_xfeatures2d_MSDDetectorG_getInnerPtrMut
}

impl core::Ptr<crate::xfeatures2d::MSDDetector> {
	#[inline] pub fn as_raw_PtrOfMSDDetector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMSDDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xfeatures2d::MSDDetectorTraitConst for core::Ptr<crate::xfeatures2d::MSDDetector> {
	#[inline] fn as_raw_MSDDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::MSDDetectorTrait for core::Ptr<crate::xfeatures2d::MSDDetector> {
	#[inline] fn as_raw_mut_MSDDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::MSDDetector> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::MSDDetector> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::MSDDetector>, core::Ptr<core::Algorithm>, cv_PtrLcv_xfeatures2d_MSDDetectorG_to_PtrOfAlgorithm }

impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::MSDDetector> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::MSDDetector> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::MSDDetector>, core::Ptr<crate::features2d::Feature2D>, cv_PtrLcv_xfeatures2d_MSDDetectorG_to_PtrOfFeature2D }

impl std::fmt::Debug for core::Ptr<crate::xfeatures2d::MSDDetector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMSDDetector")
			.finish()
	}
}

