ptr_extern! { crate::flann::LshIndexParams,
	cv_PtrLcv_flann_LshIndexParamsG_new_null_const, cv_PtrLcv_flann_LshIndexParamsG_delete, cv_PtrLcv_flann_LshIndexParamsG_getInnerPtr_const, cv_PtrLcv_flann_LshIndexParamsG_getInnerPtrMut
}

ptr_extern_ctor! { crate::flann::LshIndexParams, cv_PtrLcv_flann_LshIndexParamsG_new_const_LshIndexParams }
impl core::Ptr<crate::flann::LshIndexParams> {
	#[inline] pub fn as_raw_PtrOfLshIndexParams(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLshIndexParams(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::flann::LshIndexParamsTraitConst for core::Ptr<crate::flann::LshIndexParams> {
	#[inline] fn as_raw_LshIndexParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::flann::LshIndexParamsTrait for core::Ptr<crate::flann::LshIndexParams> {
	#[inline] fn as_raw_mut_LshIndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::flann::IndexParamsTraitConst for core::Ptr<crate::flann::LshIndexParams> {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::flann::IndexParamsTrait for core::Ptr<crate::flann::LshIndexParams> {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::flann::LshIndexParams>, core::Ptr<crate::flann::IndexParams>, cv_PtrLcv_flann_LshIndexParamsG_to_PtrOfIndexParams }

impl std::fmt::Debug for core::Ptr<crate::flann::LshIndexParams> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfLshIndexParams")
			.finish()
	}
}

