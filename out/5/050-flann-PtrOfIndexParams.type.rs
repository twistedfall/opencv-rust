ptr_extern! { crate::flann::IndexParams,
	cv_PtrLcv_flann_IndexParamsG_new_null_const, cv_PtrLcv_flann_IndexParamsG_delete, cv_PtrLcv_flann_IndexParamsG_getInnerPtr_const, cv_PtrLcv_flann_IndexParamsG_getInnerPtrMut
}

ptr_extern_ctor! { crate::flann::IndexParams, cv_PtrLcv_flann_IndexParamsG_new_const_IndexParams }
impl core::Ptr<crate::flann::IndexParams> {
	#[inline] pub fn as_raw_PtrOfIndexParams(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfIndexParams(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::flann::IndexParamsTraitConst for core::Ptr<crate::flann::IndexParams> {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::flann::IndexParamsTrait for core::Ptr<crate::flann::IndexParams> {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::flann::IndexParams> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfIndexParams")
			.finish()
	}
}

