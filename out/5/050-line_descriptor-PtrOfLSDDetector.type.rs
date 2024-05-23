ptr_extern! { crate::line_descriptor::LSDDetector,
	cv_PtrLcv_line_descriptor_LSDDetectorG_new_null_const, cv_PtrLcv_line_descriptor_LSDDetectorG_delete, cv_PtrLcv_line_descriptor_LSDDetectorG_getInnerPtr_const, cv_PtrLcv_line_descriptor_LSDDetectorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::line_descriptor::LSDDetector, cv_PtrLcv_line_descriptor_LSDDetectorG_new_const_LSDDetector }
impl core::Ptr<crate::line_descriptor::LSDDetector> {
	#[inline] pub fn as_raw_PtrOfLSDDetector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLSDDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::line_descriptor::LSDDetectorTraitConst for core::Ptr<crate::line_descriptor::LSDDetector> {
	#[inline] fn as_raw_LSDDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::line_descriptor::LSDDetectorTrait for core::Ptr<crate::line_descriptor::LSDDetector> {
	#[inline] fn as_raw_mut_LSDDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::line_descriptor::LSDDetector> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::line_descriptor::LSDDetector> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::line_descriptor::LSDDetector>, core::Ptr<core::Algorithm>, cv_PtrLcv_line_descriptor_LSDDetectorG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::line_descriptor::LSDDetector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfLSDDetector")
			.finish()
	}
}

