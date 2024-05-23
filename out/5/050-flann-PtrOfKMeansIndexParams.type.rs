ptr_extern! { crate::flann::KMeansIndexParams,
	cv_PtrLcv_flann_KMeansIndexParamsG_new_null_const, cv_PtrLcv_flann_KMeansIndexParamsG_delete, cv_PtrLcv_flann_KMeansIndexParamsG_getInnerPtr_const, cv_PtrLcv_flann_KMeansIndexParamsG_getInnerPtrMut
}

ptr_extern_ctor! { crate::flann::KMeansIndexParams, cv_PtrLcv_flann_KMeansIndexParamsG_new_const_KMeansIndexParams }
impl core::Ptr<crate::flann::KMeansIndexParams> {
	#[inline] pub fn as_raw_PtrOfKMeansIndexParams(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfKMeansIndexParams(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::flann::KMeansIndexParamsTraitConst for core::Ptr<crate::flann::KMeansIndexParams> {
	#[inline] fn as_raw_KMeansIndexParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::flann::KMeansIndexParamsTrait for core::Ptr<crate::flann::KMeansIndexParams> {
	#[inline] fn as_raw_mut_KMeansIndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::flann::IndexParamsTraitConst for core::Ptr<crate::flann::KMeansIndexParams> {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::flann::IndexParamsTrait for core::Ptr<crate::flann::KMeansIndexParams> {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::flann::KMeansIndexParams>, core::Ptr<crate::flann::IndexParams>, cv_PtrLcv_flann_KMeansIndexParamsG_to_PtrOfIndexParams }

impl std::fmt::Debug for core::Ptr<crate::flann::KMeansIndexParams> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfKMeansIndexParams")
			.finish()
	}
}

