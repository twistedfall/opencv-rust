ptr_extern! { crate::flann::CompositeIndexParams,
	cv_PtrLcv_flann_CompositeIndexParamsG_new_null_const, cv_PtrLcv_flann_CompositeIndexParamsG_delete, cv_PtrLcv_flann_CompositeIndexParamsG_getInnerPtr_const, cv_PtrLcv_flann_CompositeIndexParamsG_getInnerPtrMut
}

ptr_extern_ctor! { crate::flann::CompositeIndexParams, cv_PtrLcv_flann_CompositeIndexParamsG_new_const_CompositeIndexParams }
impl core::Ptr<crate::flann::CompositeIndexParams> {
	#[inline] pub fn as_raw_PtrOfCompositeIndexParams(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCompositeIndexParams(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::flann::CompositeIndexParamsTraitConst for core::Ptr<crate::flann::CompositeIndexParams> {
	#[inline] fn as_raw_CompositeIndexParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::flann::CompositeIndexParamsTrait for core::Ptr<crate::flann::CompositeIndexParams> {
	#[inline] fn as_raw_mut_CompositeIndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::flann::IndexParamsTraitConst for core::Ptr<crate::flann::CompositeIndexParams> {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::flann::IndexParamsTrait for core::Ptr<crate::flann::CompositeIndexParams> {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::flann::CompositeIndexParams>, core::Ptr<crate::flann::IndexParams>, cv_PtrLcv_flann_CompositeIndexParamsG_to_PtrOfIndexParams }

impl std::fmt::Debug for core::Ptr<crate::flann::CompositeIndexParams> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfCompositeIndexParams")
			.finish()
	}
}

