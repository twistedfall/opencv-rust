ptr_extern! { crate::stitching::Detail_BundleAdjusterBase,
	cv_PtrLcv_detail_BundleAdjusterBaseG_new_null_const, cv_PtrLcv_detail_BundleAdjusterBaseG_delete, cv_PtrLcv_detail_BundleAdjusterBaseG_getInnerPtr_const, cv_PtrLcv_detail_BundleAdjusterBaseG_getInnerPtrMut
}

impl core::Ptr<crate::stitching::Detail_BundleAdjusterBase> {
	#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterBase(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterBase(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_BundleAdjusterBaseTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterBase> {
	#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterBaseTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterBase> {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_EstimatorTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterBase> {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_EstimatorTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterBase> {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_BundleAdjusterBase>, core::Ptr<crate::stitching::Detail_Estimator>, cv_PtrLcv_detail_BundleAdjusterBaseG_to_PtrOfDetail_Estimator }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_BundleAdjusterBase> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_BundleAdjusterBase")
			.finish()
	}
}

