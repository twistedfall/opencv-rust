ptr_extern! { crate::sfm::SFMLibmvEuclideanReconstruction,
	cv_PtrLcv_sfm_SFMLibmvEuclideanReconstructionG_new_null_const, cv_PtrLcv_sfm_SFMLibmvEuclideanReconstructionG_delete, cv_PtrLcv_sfm_SFMLibmvEuclideanReconstructionG_getInnerPtr_const, cv_PtrLcv_sfm_SFMLibmvEuclideanReconstructionG_getInnerPtrMut
}

impl core::Ptr<crate::sfm::SFMLibmvEuclideanReconstruction> {
	#[inline] pub fn as_raw_PtrOfSFMLibmvEuclideanReconstruction(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSFMLibmvEuclideanReconstruction(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::sfm::SFMLibmvEuclideanReconstructionTraitConst for core::Ptr<crate::sfm::SFMLibmvEuclideanReconstruction> {
	#[inline] fn as_raw_SFMLibmvEuclideanReconstruction(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::sfm::SFMLibmvEuclideanReconstructionTrait for core::Ptr<crate::sfm::SFMLibmvEuclideanReconstruction> {
	#[inline] fn as_raw_mut_SFMLibmvEuclideanReconstruction(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::sfm::BaseSFMTraitConst for core::Ptr<crate::sfm::SFMLibmvEuclideanReconstruction> {
	#[inline] fn as_raw_BaseSFM(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::sfm::BaseSFMTrait for core::Ptr<crate::sfm::SFMLibmvEuclideanReconstruction> {
	#[inline] fn as_raw_mut_BaseSFM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::sfm::SFMLibmvEuclideanReconstruction>, core::Ptr<crate::sfm::BaseSFM>, cv_PtrLcv_sfm_SFMLibmvEuclideanReconstructionG_to_PtrOfBaseSFM }

impl std::fmt::Debug for core::Ptr<crate::sfm::SFMLibmvEuclideanReconstruction> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSFMLibmvEuclideanReconstruction")
			.finish()
	}
}

