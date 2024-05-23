ptr_extern! { crate::features::SimpleBlobDetector,
	cv_PtrLcv_SimpleBlobDetectorG_new_null_const, cv_PtrLcv_SimpleBlobDetectorG_delete, cv_PtrLcv_SimpleBlobDetectorG_getInnerPtr_const, cv_PtrLcv_SimpleBlobDetectorG_getInnerPtrMut
}

impl core::Ptr<crate::features::SimpleBlobDetector> {
	#[inline] pub fn as_raw_PtrOfSimpleBlobDetector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSimpleBlobDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::features::SimpleBlobDetectorTraitConst for core::Ptr<crate::features::SimpleBlobDetector> {
	#[inline] fn as_raw_SimpleBlobDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features::SimpleBlobDetectorTrait for core::Ptr<crate::features::SimpleBlobDetector> {
	#[inline] fn as_raw_mut_SimpleBlobDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::features::SimpleBlobDetector> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::features::SimpleBlobDetector> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::features::SimpleBlobDetector>, core::Ptr<core::Algorithm>, cv_PtrLcv_SimpleBlobDetectorG_to_PtrOfAlgorithm }

impl crate::features::Feature2DTraitConst for core::Ptr<crate::features::SimpleBlobDetector> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features::Feature2DTrait for core::Ptr<crate::features::SimpleBlobDetector> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::features::SimpleBlobDetector>, core::Ptr<crate::features::Feature2D>, cv_PtrLcv_SimpleBlobDetectorG_to_PtrOfFeature2D }

impl std::fmt::Debug for core::Ptr<crate::features::SimpleBlobDetector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSimpleBlobDetector")
			.finish()
	}
}

