ptr_extern! { crate::ximgproc::DTFilter,
	cv_PtrLcv_ximgproc_DTFilterG_new_null_const, cv_PtrLcv_ximgproc_DTFilterG_delete, cv_PtrLcv_ximgproc_DTFilterG_getInnerPtr_const, cv_PtrLcv_ximgproc_DTFilterG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::DTFilter> {
	#[inline] pub fn as_raw_PtrOfDTFilter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDTFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::DTFilterTraitConst for core::Ptr<crate::ximgproc::DTFilter> {
	#[inline] fn as_raw_DTFilter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::DTFilterTrait for core::Ptr<crate::ximgproc::DTFilter> {
	#[inline] fn as_raw_mut_DTFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::DTFilter> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::DTFilter> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::DTFilter>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_DTFilterG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::DTFilter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDTFilter")
			.finish()
	}
}

