ptr_extern! { crate::ximgproc::SuperpixelSLIC,
	cv_PtrLcv_ximgproc_SuperpixelSLICG_new_null_const, cv_PtrLcv_ximgproc_SuperpixelSLICG_delete, cv_PtrLcv_ximgproc_SuperpixelSLICG_getInnerPtr_const, cv_PtrLcv_ximgproc_SuperpixelSLICG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::SuperpixelSLIC> {
	#[inline] pub fn as_raw_PtrOfSuperpixelSLIC(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSuperpixelSLIC(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::SuperpixelSLICTraitConst for core::Ptr<crate::ximgproc::SuperpixelSLIC> {
	#[inline] fn as_raw_SuperpixelSLIC(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SuperpixelSLICTrait for core::Ptr<crate::ximgproc::SuperpixelSLIC> {
	#[inline] fn as_raw_mut_SuperpixelSLIC(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::SuperpixelSLIC> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::SuperpixelSLIC> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::SuperpixelSLIC>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_SuperpixelSLICG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::SuperpixelSLIC> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSuperpixelSLIC")
			.finish()
	}
}

