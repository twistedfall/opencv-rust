ptr_extern! { crate::rgbd::LineMod_QuantizedPyramid,
	cv_PtrLcv_linemod_QuantizedPyramidG_new_null_const, cv_PtrLcv_linemod_QuantizedPyramidG_delete, cv_PtrLcv_linemod_QuantizedPyramidG_getInnerPtr_const, cv_PtrLcv_linemod_QuantizedPyramidG_getInnerPtrMut
}

impl core::Ptr<crate::rgbd::LineMod_QuantizedPyramid> {
	#[inline] pub fn as_raw_PtrOfLineMod_QuantizedPyramid(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLineMod_QuantizedPyramid(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rgbd::LineMod_QuantizedPyramidTraitConst for core::Ptr<crate::rgbd::LineMod_QuantizedPyramid> {
	#[inline] fn as_raw_LineMod_QuantizedPyramid(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::LineMod_QuantizedPyramidTrait for core::Ptr<crate::rgbd::LineMod_QuantizedPyramid> {
	#[inline] fn as_raw_mut_LineMod_QuantizedPyramid(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::rgbd::LineMod_QuantizedPyramid> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfLineMod_QuantizedPyramid")
			.finish()
	}
}

