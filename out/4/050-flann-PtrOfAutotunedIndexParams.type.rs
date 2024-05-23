ptr_extern! { crate::flann::AutotunedIndexParams,
	cv_PtrLcv_flann_AutotunedIndexParamsG_new_null_const, cv_PtrLcv_flann_AutotunedIndexParamsG_delete, cv_PtrLcv_flann_AutotunedIndexParamsG_getInnerPtr_const, cv_PtrLcv_flann_AutotunedIndexParamsG_getInnerPtrMut
}

ptr_extern_ctor! { crate::flann::AutotunedIndexParams, cv_PtrLcv_flann_AutotunedIndexParamsG_new_const_AutotunedIndexParams }
impl core::Ptr<crate::flann::AutotunedIndexParams> {
	#[inline] pub fn as_raw_PtrOfAutotunedIndexParams(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAutotunedIndexParams(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::flann::AutotunedIndexParamsTraitConst for core::Ptr<crate::flann::AutotunedIndexParams> {
	#[inline] fn as_raw_AutotunedIndexParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::flann::AutotunedIndexParamsTrait for core::Ptr<crate::flann::AutotunedIndexParams> {
	#[inline] fn as_raw_mut_AutotunedIndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::flann::IndexParamsTraitConst for core::Ptr<crate::flann::AutotunedIndexParams> {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::flann::IndexParamsTrait for core::Ptr<crate::flann::AutotunedIndexParams> {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::flann::AutotunedIndexParams>, core::Ptr<crate::flann::IndexParams>, cv_PtrLcv_flann_AutotunedIndexParamsG_to_PtrOfIndexParams }

impl std::fmt::Debug for core::Ptr<crate::flann::AutotunedIndexParams> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfAutotunedIndexParams")
			.finish()
	}
}

