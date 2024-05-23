ptr_extern! { crate::flann::LinearIndexParams,
	cv_PtrLcv_flann_LinearIndexParamsG_new_null_const, cv_PtrLcv_flann_LinearIndexParamsG_delete, cv_PtrLcv_flann_LinearIndexParamsG_getInnerPtr_const, cv_PtrLcv_flann_LinearIndexParamsG_getInnerPtrMut
}

ptr_extern_ctor! { crate::flann::LinearIndexParams, cv_PtrLcv_flann_LinearIndexParamsG_new_const_LinearIndexParams }
impl core::Ptr<crate::flann::LinearIndexParams> {
	#[inline] pub fn as_raw_PtrOfLinearIndexParams(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLinearIndexParams(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::flann::LinearIndexParamsTraitConst for core::Ptr<crate::flann::LinearIndexParams> {
	#[inline] fn as_raw_LinearIndexParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::flann::LinearIndexParamsTrait for core::Ptr<crate::flann::LinearIndexParams> {
	#[inline] fn as_raw_mut_LinearIndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::flann::IndexParamsTraitConst for core::Ptr<crate::flann::LinearIndexParams> {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::flann::IndexParamsTrait for core::Ptr<crate::flann::LinearIndexParams> {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::flann::LinearIndexParams>, core::Ptr<crate::flann::IndexParams>, cv_PtrLcv_flann_LinearIndexParamsG_to_PtrOfIndexParams }

impl std::fmt::Debug for core::Ptr<crate::flann::LinearIndexParams> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfLinearIndexParams")
			.finish()
	}
}

