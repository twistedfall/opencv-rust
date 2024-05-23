ptr_extern! { crate::flann::SavedIndexParams,
	cv_PtrLcv_flann_SavedIndexParamsG_new_null_const, cv_PtrLcv_flann_SavedIndexParamsG_delete, cv_PtrLcv_flann_SavedIndexParamsG_getInnerPtr_const, cv_PtrLcv_flann_SavedIndexParamsG_getInnerPtrMut
}

ptr_extern_ctor! { crate::flann::SavedIndexParams, cv_PtrLcv_flann_SavedIndexParamsG_new_const_SavedIndexParams }
impl core::Ptr<crate::flann::SavedIndexParams> {
	#[inline] pub fn as_raw_PtrOfSavedIndexParams(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSavedIndexParams(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::flann::SavedIndexParamsTraitConst for core::Ptr<crate::flann::SavedIndexParams> {
	#[inline] fn as_raw_SavedIndexParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::flann::SavedIndexParamsTrait for core::Ptr<crate::flann::SavedIndexParams> {
	#[inline] fn as_raw_mut_SavedIndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::flann::IndexParamsTraitConst for core::Ptr<crate::flann::SavedIndexParams> {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::flann::IndexParamsTrait for core::Ptr<crate::flann::SavedIndexParams> {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::flann::SavedIndexParams>, core::Ptr<crate::flann::IndexParams>, cv_PtrLcv_flann_SavedIndexParamsG_to_PtrOfIndexParams }

impl std::fmt::Debug for core::Ptr<crate::flann::SavedIndexParams> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSavedIndexParams")
			.finish()
	}
}

