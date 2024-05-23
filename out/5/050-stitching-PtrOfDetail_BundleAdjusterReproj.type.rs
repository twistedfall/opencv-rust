ptr_extern! { crate::stitching::Detail_BundleAdjusterReproj,
	cv_PtrLcv_detail_BundleAdjusterReprojG_new_null_const, cv_PtrLcv_detail_BundleAdjusterReprojG_delete, cv_PtrLcv_detail_BundleAdjusterReprojG_getInnerPtr_const, cv_PtrLcv_detail_BundleAdjusterReprojG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_BundleAdjusterReproj, cv_PtrLcv_detail_BundleAdjusterReprojG_new_const_BundleAdjusterReproj }
impl core::Ptr<crate::stitching::Detail_BundleAdjusterReproj> {
	#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterReproj(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterReproj(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_BundleAdjusterReprojTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterReproj> {
	#[inline] fn as_raw_Detail_BundleAdjusterReproj(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterReprojTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterReproj> {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterReproj(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_BundleAdjusterBaseTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterReproj> {
	#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterBaseTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterReproj> {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_BundleAdjusterReproj>, core::Ptr<crate::stitching::Detail_BundleAdjusterBase>, cv_PtrLcv_detail_BundleAdjusterReprojG_to_PtrOfDetail_BundleAdjusterBase }

impl crate::stitching::Detail_EstimatorTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterReproj> {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_EstimatorTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterReproj> {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_BundleAdjusterReproj>, core::Ptr<crate::stitching::Detail_Estimator>, cv_PtrLcv_detail_BundleAdjusterReprojG_to_PtrOfDetail_Estimator }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_BundleAdjusterReproj> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_BundleAdjusterReproj")
			.finish()
	}
}

