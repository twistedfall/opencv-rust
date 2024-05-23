ptr_extern! { crate::flann::SearchParams,
	cv_PtrLcv_flann_SearchParamsG_new_null_const, cv_PtrLcv_flann_SearchParamsG_delete, cv_PtrLcv_flann_SearchParamsG_getInnerPtr_const, cv_PtrLcv_flann_SearchParamsG_getInnerPtrMut
}

ptr_extern_ctor! { crate::flann::SearchParams, cv_PtrLcv_flann_SearchParamsG_new_const_SearchParams }
impl core::Ptr<crate::flann::SearchParams> {
	#[inline] pub fn as_raw_PtrOfSearchParams(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSearchParams(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::flann::SearchParamsTraitConst for core::Ptr<crate::flann::SearchParams> {
	#[inline] fn as_raw_SearchParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::flann::SearchParamsTrait for core::Ptr<crate::flann::SearchParams> {
	#[inline] fn as_raw_mut_SearchParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::flann::IndexParamsTraitConst for core::Ptr<crate::flann::SearchParams> {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::flann::IndexParamsTrait for core::Ptr<crate::flann::SearchParams> {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::flann::SearchParams>, core::Ptr<crate::flann::IndexParams>, cv_PtrLcv_flann_SearchParamsG_to_PtrOfIndexParams }

impl std::fmt::Debug for core::Ptr<crate::flann::SearchParams> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSearchParams")
			.finish()
	}
}

