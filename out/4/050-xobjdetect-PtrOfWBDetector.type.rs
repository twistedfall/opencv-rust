ptr_extern! { crate::xobjdetect::WBDetector,
	cv_PtrLcv_xobjdetect_WBDetectorG_new_null_const, cv_PtrLcv_xobjdetect_WBDetectorG_delete, cv_PtrLcv_xobjdetect_WBDetectorG_getInnerPtr_const, cv_PtrLcv_xobjdetect_WBDetectorG_getInnerPtrMut
}

impl core::Ptr<crate::xobjdetect::WBDetector> {
	#[inline] pub fn as_raw_PtrOfWBDetector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfWBDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xobjdetect::WBDetectorTraitConst for core::Ptr<crate::xobjdetect::WBDetector> {
	#[inline] fn as_raw_WBDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xobjdetect::WBDetectorTrait for core::Ptr<crate::xobjdetect::WBDetector> {
	#[inline] fn as_raw_mut_WBDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::xobjdetect::WBDetector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfWBDetector")
			.finish()
	}
}

