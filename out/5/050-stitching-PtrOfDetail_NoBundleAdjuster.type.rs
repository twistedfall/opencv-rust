ptr_extern! { crate::stitching::Detail_NoBundleAdjuster,
	cv_PtrLcv_detail_NoBundleAdjusterG_new_null_const, cv_PtrLcv_detail_NoBundleAdjusterG_delete, cv_PtrLcv_detail_NoBundleAdjusterG_getInnerPtr_const, cv_PtrLcv_detail_NoBundleAdjusterG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_NoBundleAdjuster, cv_PtrLcv_detail_NoBundleAdjusterG_new_const_NoBundleAdjuster }
impl core::Ptr<crate::stitching::Detail_NoBundleAdjuster> {
	#[inline] pub fn as_raw_PtrOfDetail_NoBundleAdjuster(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_NoBundleAdjuster(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_NoBundleAdjusterTraitConst for core::Ptr<crate::stitching::Detail_NoBundleAdjuster> {
	#[inline] fn as_raw_Detail_NoBundleAdjuster(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_NoBundleAdjusterTrait for core::Ptr<crate::stitching::Detail_NoBundleAdjuster> {
	#[inline] fn as_raw_mut_Detail_NoBundleAdjuster(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_BundleAdjusterBaseTraitConst for core::Ptr<crate::stitching::Detail_NoBundleAdjuster> {
	#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterBaseTrait for core::Ptr<crate::stitching::Detail_NoBundleAdjuster> {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_NoBundleAdjuster>, core::Ptr<crate::stitching::Detail_BundleAdjusterBase>, cv_PtrLcv_detail_NoBundleAdjusterG_to_PtrOfDetail_BundleAdjusterBase }

impl crate::stitching::Detail_EstimatorTraitConst for core::Ptr<crate::stitching::Detail_NoBundleAdjuster> {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_EstimatorTrait for core::Ptr<crate::stitching::Detail_NoBundleAdjuster> {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_NoBundleAdjuster>, core::Ptr<crate::stitching::Detail_Estimator>, cv_PtrLcv_detail_NoBundleAdjusterG_to_PtrOfDetail_Estimator }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_NoBundleAdjuster> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_NoBundleAdjuster")
			.finish()
	}
}

