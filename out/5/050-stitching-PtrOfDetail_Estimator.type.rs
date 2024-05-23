ptr_extern! { crate::stitching::Detail_Estimator,
	cv_PtrLcv_detail_EstimatorG_new_null_const, cv_PtrLcv_detail_EstimatorG_delete, cv_PtrLcv_detail_EstimatorG_getInnerPtr_const, cv_PtrLcv_detail_EstimatorG_getInnerPtrMut
}

impl core::Ptr<crate::stitching::Detail_Estimator> {
	#[inline] pub fn as_raw_PtrOfDetail_Estimator(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_Estimator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_EstimatorTraitConst for core::Ptr<crate::stitching::Detail_Estimator> {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_EstimatorTrait for core::Ptr<crate::stitching::Detail_Estimator> {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_Estimator> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_Estimator")
			.finish()
	}
}

