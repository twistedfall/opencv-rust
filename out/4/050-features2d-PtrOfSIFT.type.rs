ptr_extern! { crate::features2d::SIFT,
	cv_PtrLcv_SIFTG_new_null_const, cv_PtrLcv_SIFTG_delete, cv_PtrLcv_SIFTG_getInnerPtr_const, cv_PtrLcv_SIFTG_getInnerPtrMut
}

impl core::Ptr<crate::features2d::SIFT> {
	#[inline] pub fn as_raw_PtrOfSIFT(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSIFT(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::features2d::SIFTTraitConst for core::Ptr<crate::features2d::SIFT> {
	#[inline] fn as_raw_SIFT(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::SIFTTrait for core::Ptr<crate::features2d::SIFT> {
	#[inline] fn as_raw_mut_SIFT(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::SIFT> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::features2d::SIFT> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::features2d::SIFT>, core::Ptr<core::Algorithm>, cv_PtrLcv_SIFTG_to_PtrOfAlgorithm }

impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::features2d::SIFT> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for core::Ptr<crate::features2d::SIFT> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::features2d::SIFT>, core::Ptr<crate::features2d::Feature2D>, cv_PtrLcv_SIFTG_to_PtrOfFeature2D }

impl std::fmt::Debug for core::Ptr<crate::features2d::SIFT> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSIFT")
			.finish()
	}
}

