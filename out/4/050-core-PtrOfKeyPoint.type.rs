ptr_extern! { core::KeyPoint,
	cv_PtrLcv_KeyPointG_new_null_const, cv_PtrLcv_KeyPointG_delete, cv_PtrLcv_KeyPointG_getInnerPtr_const, cv_PtrLcv_KeyPointG_getInnerPtrMut
}

ptr_extern_ctor! { core::KeyPoint, cv_PtrLcv_KeyPointG_new_const_KeyPoint }
impl core::Ptr<core::KeyPoint> {
	#[inline] pub fn as_raw_PtrOfKeyPoint(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfKeyPoint(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl core::KeyPointTraitConst for core::Ptr<core::KeyPoint> {
	#[inline] fn as_raw_KeyPoint(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::KeyPointTrait for core::Ptr<core::KeyPoint> {
	#[inline] fn as_raw_mut_KeyPoint(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<core::KeyPoint> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfKeyPoint")
			.finish()
	}
}

