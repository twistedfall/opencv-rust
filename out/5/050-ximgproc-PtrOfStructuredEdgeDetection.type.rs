ptr_extern! { crate::ximgproc::StructuredEdgeDetection,
	cv_PtrLcv_ximgproc_StructuredEdgeDetectionG_new_null_const, cv_PtrLcv_ximgproc_StructuredEdgeDetectionG_delete, cv_PtrLcv_ximgproc_StructuredEdgeDetectionG_getInnerPtr_const, cv_PtrLcv_ximgproc_StructuredEdgeDetectionG_getInnerPtrMut
}

impl core::Ptr<crate::ximgproc::StructuredEdgeDetection> {
	#[inline] pub fn as_raw_PtrOfStructuredEdgeDetection(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfStructuredEdgeDetection(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ximgproc::StructuredEdgeDetectionTraitConst for core::Ptr<crate::ximgproc::StructuredEdgeDetection> {
	#[inline] fn as_raw_StructuredEdgeDetection(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::StructuredEdgeDetectionTrait for core::Ptr<crate::ximgproc::StructuredEdgeDetection> {
	#[inline] fn as_raw_mut_StructuredEdgeDetection(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::StructuredEdgeDetection> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::StructuredEdgeDetection> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ximgproc::StructuredEdgeDetection>, core::Ptr<core::Algorithm>, cv_PtrLcv_ximgproc_StructuredEdgeDetectionG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::ximgproc::StructuredEdgeDetection> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfStructuredEdgeDetection")
			.finish()
	}
}

