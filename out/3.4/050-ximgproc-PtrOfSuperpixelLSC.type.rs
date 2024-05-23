ptr_extern! { crate::ximgproc::SuperpixelLSC,
	cv_PtrLcv_ximgproc_SuperpixelLSCG_new_null_const, cv_PtrLcv_ximgproc_SuperpixelLSCG_delete, cv_PtrLcv_ximgproc_SuperpixelLSCG_getInnerPtr_const, cv_PtrLcv_ximgproc_SuperpixelLSCG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::SuperpixelLSC> {
	#[inline] pub fn as_raw_PtrOfSuperpixelLSC(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSuperpixelLSC(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::SuperpixelLSCTraitConst for core::Ptr<crate::ximgproc::SuperpixelLSC> {
	#[inline] fn as_raw_SuperpixelLSC(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SuperpixelLSCTrait for core::Ptr<crate::ximgproc::SuperpixelLSC> {
	#[inline] fn as_raw_mut_SuperpixelLSC(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::SuperpixelLSC> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::SuperpixelLSC> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::SuperpixelLSC>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_SuperpixelLSCG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::SuperpixelLSC> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSuperpixelLSC")
			.finish()
	}
}

