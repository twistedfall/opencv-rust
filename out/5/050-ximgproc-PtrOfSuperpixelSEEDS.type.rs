ptr_extern! { crate::ximgproc::SuperpixelSEEDS,
	cv_PtrLcv_ximgproc_SuperpixelSEEDSG_new_null_const, cv_PtrLcv_ximgproc_SuperpixelSEEDSG_delete, cv_PtrLcv_ximgproc_SuperpixelSEEDSG_getInnerPtr_const, cv_PtrLcv_ximgproc_SuperpixelSEEDSG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::SuperpixelSEEDS> {
	#[inline] pub fn as_raw_PtrOfSuperpixelSEEDS(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSuperpixelSEEDS(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::SuperpixelSEEDSTraitConst for core::Ptr<crate::ximgproc::SuperpixelSEEDS> {
	#[inline] fn as_raw_SuperpixelSEEDS(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SuperpixelSEEDSTrait for core::Ptr<crate::ximgproc::SuperpixelSEEDS> {
	#[inline] fn as_raw_mut_SuperpixelSEEDS(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::SuperpixelSEEDS> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::SuperpixelSEEDS> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::SuperpixelSEEDS>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_SuperpixelSEEDSG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::SuperpixelSEEDS> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSuperpixelSEEDS")
			.finish()
	}
}

