ptr_extern! { crate::rgbd::LineMod_Detector,
	cv_PtrLcv_linemod_DetectorG_new_null_const, cv_PtrLcv_linemod_DetectorG_delete, cv_PtrLcv_linemod_DetectorG_getInnerPtr_const, cv_PtrLcv_linemod_DetectorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::rgbd::LineMod_Detector, cv_PtrLcv_linemod_DetectorG_new_const_Detector }
impl core::Ptr<crate::rgbd::LineMod_Detector> {
	#[inline] pub fn as_raw_PtrOfLineMod_Detector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLineMod_Detector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::LineMod_DetectorTraitConst for core::Ptr<crate::rgbd::LineMod_Detector> {
	#[inline] fn as_raw_LineMod_Detector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::LineMod_DetectorTrait for core::Ptr<crate::rgbd::LineMod_Detector> {
	#[inline] fn as_raw_mut_LineMod_Detector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::rgbd::LineMod_Detector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfLineMod_Detector")
			.finish()
	}
}

