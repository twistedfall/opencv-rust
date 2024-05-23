ptr_extern! { crate::ximgproc::EdgeBoxes,
	cv_PtrLcv_ximgproc_EdgeBoxesG_new_null_const, cv_PtrLcv_ximgproc_EdgeBoxesG_delete, cv_PtrLcv_ximgproc_EdgeBoxesG_getInnerPtr_const, cv_PtrLcv_ximgproc_EdgeBoxesG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::EdgeBoxes> {
	#[inline] pub fn as_raw_PtrOfEdgeBoxes(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfEdgeBoxes(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::EdgeBoxesTraitConst for core::Ptr<crate::ximgproc::EdgeBoxes> {
	#[inline] fn as_raw_EdgeBoxes(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::EdgeBoxesTrait for core::Ptr<crate::ximgproc::EdgeBoxes> {
	#[inline] fn as_raw_mut_EdgeBoxes(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::EdgeBoxes> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::EdgeBoxes> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::EdgeBoxes>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_EdgeBoxesG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::EdgeBoxes> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfEdgeBoxes")
			.finish()
	}
}

