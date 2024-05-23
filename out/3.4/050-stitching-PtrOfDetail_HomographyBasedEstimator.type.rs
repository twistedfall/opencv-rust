ptr_extern! { crate::stitching::Detail_HomographyBasedEstimator,
	cv_PtrLcv_detail_HomographyBasedEstimatorG_new_null_const, cv_PtrLcv_detail_HomographyBasedEstimatorG_delete, cv_PtrLcv_detail_HomographyBasedEstimatorG_getInnerPtr_const, cv_PtrLcv_detail_HomographyBasedEstimatorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_HomographyBasedEstimator, cv_PtrLcv_detail_HomographyBasedEstimatorG_new_const_HomographyBasedEstimator }
impl core::Ptr<crate::stitching::Detail_HomographyBasedEstimator> {
	#[inline] pub fn as_raw_PtrOfDetail_HomographyBasedEstimator(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_HomographyBasedEstimator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_HomographyBasedEstimatorTraitConst for core::Ptr<crate::stitching::Detail_HomographyBasedEstimator> {
	#[inline] fn as_raw_Detail_HomographyBasedEstimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_HomographyBasedEstimatorTrait for core::Ptr<crate::stitching::Detail_HomographyBasedEstimator> {
	#[inline] fn as_raw_mut_Detail_HomographyBasedEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_EstimatorTraitConst for core::Ptr<crate::stitching::Detail_HomographyBasedEstimator> {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_EstimatorTrait for core::Ptr<crate::stitching::Detail_HomographyBasedEstimator> {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_HomographyBasedEstimator>, core::Ptr<crate::stitching::Detail_Estimator>, cv_PtrLcv_detail_HomographyBasedEstimatorG_to_PtrOfDetail_Estimator }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_HomographyBasedEstimator> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_HomographyBasedEstimator")
			.finish()
	}
}

