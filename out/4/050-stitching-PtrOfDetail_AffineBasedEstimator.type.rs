ptr_extern! { crate::stitching::Detail_AffineBasedEstimator,
	cv_PtrLcv_detail_AffineBasedEstimatorG_new_null_const, cv_PtrLcv_detail_AffineBasedEstimatorG_delete, cv_PtrLcv_detail_AffineBasedEstimatorG_getInnerPtr_const, cv_PtrLcv_detail_AffineBasedEstimatorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_AffineBasedEstimator, cv_PtrLcv_detail_AffineBasedEstimatorG_new_const_AffineBasedEstimator }
impl core::Ptr<crate::stitching::Detail_AffineBasedEstimator> {
	#[inline] pub fn as_raw_PtrOfDetail_AffineBasedEstimator(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_AffineBasedEstimator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_AffineBasedEstimatorTraitConst for core::Ptr<crate::stitching::Detail_AffineBasedEstimator> {
	#[inline] fn as_raw_Detail_AffineBasedEstimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_AffineBasedEstimatorTrait for core::Ptr<crate::stitching::Detail_AffineBasedEstimator> {
	#[inline] fn as_raw_mut_Detail_AffineBasedEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_EstimatorTraitConst for core::Ptr<crate::stitching::Detail_AffineBasedEstimator> {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_EstimatorTrait for core::Ptr<crate::stitching::Detail_AffineBasedEstimator> {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_AffineBasedEstimator>, core::Ptr<crate::stitching::Detail_Estimator>, cv_PtrLcv_detail_AffineBasedEstimatorG_to_PtrOfDetail_Estimator }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_AffineBasedEstimator> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_AffineBasedEstimator")
			.finish()
	}
}

