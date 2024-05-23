ptr_extern! { crate::flann::KDTreeIndexParams,
	cv_PtrLcv_flann_KDTreeIndexParamsG_new_null_const, cv_PtrLcv_flann_KDTreeIndexParamsG_delete, cv_PtrLcv_flann_KDTreeIndexParamsG_getInnerPtr_const, cv_PtrLcv_flann_KDTreeIndexParamsG_getInnerPtrMut
}

ptr_extern_ctor! { crate::flann::KDTreeIndexParams, cv_PtrLcv_flann_KDTreeIndexParamsG_new_const_KDTreeIndexParams }
impl core::Ptr<crate::flann::KDTreeIndexParams> {
	#[inline] pub fn as_raw_PtrOfKDTreeIndexParams(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfKDTreeIndexParams(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::flann::KDTreeIndexParamsTraitConst for core::Ptr<crate::flann::KDTreeIndexParams> {
	#[inline] fn as_raw_KDTreeIndexParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::flann::KDTreeIndexParamsTrait for core::Ptr<crate::flann::KDTreeIndexParams> {
	#[inline] fn as_raw_mut_KDTreeIndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::flann::IndexParamsTraitConst for core::Ptr<crate::flann::KDTreeIndexParams> {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::flann::IndexParamsTrait for core::Ptr<crate::flann::KDTreeIndexParams> {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::flann::KDTreeIndexParams>, core::Ptr<crate::flann::IndexParams>, cv_PtrLcv_flann_KDTreeIndexParamsG_to_PtrOfIndexParams }

impl std::fmt::Debug for core::Ptr<crate::flann::KDTreeIndexParams> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfKDTreeIndexParams")
			.finish()
	}
}

