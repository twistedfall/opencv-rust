ptr_extern! { crate::ximgproc::EdgeDrawing,
	cv_PtrLcv_ximgproc_EdgeDrawingG_new_null_const, cv_PtrLcv_ximgproc_EdgeDrawingG_delete, cv_PtrLcv_ximgproc_EdgeDrawingG_getInnerPtr_const, cv_PtrLcv_ximgproc_EdgeDrawingG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::EdgeDrawing> {
	#[inline] pub fn as_raw_PtrOfEdgeDrawing(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfEdgeDrawing(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::EdgeDrawingTraitConst for core::Ptr<crate::ximgproc::EdgeDrawing> {
	#[inline] fn as_raw_EdgeDrawing(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::EdgeDrawingTrait for core::Ptr<crate::ximgproc::EdgeDrawing> {
	#[inline] fn as_raw_mut_EdgeDrawing(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::EdgeDrawing> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::EdgeDrawing> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::EdgeDrawing>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_EdgeDrawingG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::EdgeDrawing> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfEdgeDrawing")
			.field("params", &crate::ximgproc::EdgeDrawingTraitConst::params(self))
			.finish()
	}
}

