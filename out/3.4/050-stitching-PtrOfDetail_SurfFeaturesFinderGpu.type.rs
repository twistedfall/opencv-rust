ptr_extern! { crate::stitching::Detail_SurfFeaturesFinderGpu,
	cv_PtrLcv_detail_SurfFeaturesFinderGpuG_new_null_const, cv_PtrLcv_detail_SurfFeaturesFinderGpuG_delete, cv_PtrLcv_detail_SurfFeaturesFinderGpuG_getInnerPtr_const, cv_PtrLcv_detail_SurfFeaturesFinderGpuG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_SurfFeaturesFinderGpu, cv_PtrLcv_detail_SurfFeaturesFinderGpuG_new_const_SurfFeaturesFinderGpu }
impl core::Ptr<crate::stitching::Detail_SurfFeaturesFinderGpu> {
	#[inline] pub fn as_raw_PtrOfDetail_SurfFeaturesFinderGpu(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_SurfFeaturesFinderGpu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_SurfFeaturesFinderGpuTraitConst for core::Ptr<crate::stitching::Detail_SurfFeaturesFinderGpu> {
	#[inline] fn as_raw_Detail_SurfFeaturesFinderGpu(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_SurfFeaturesFinderGpuTrait for core::Ptr<crate::stitching::Detail_SurfFeaturesFinderGpu> {
	#[inline] fn as_raw_mut_Detail_SurfFeaturesFinderGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_FeaturesFinderTraitConst for core::Ptr<crate::stitching::Detail_SurfFeaturesFinderGpu> {
	#[inline] fn as_raw_Detail_FeaturesFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_FeaturesFinderTrait for core::Ptr<crate::stitching::Detail_SurfFeaturesFinderGpu> {
	#[inline] fn as_raw_mut_Detail_FeaturesFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::stitching::Detail_SurfFeaturesFinderGpu>, core::Ptr<crate::stitching::Detail_FeaturesFinder>, cv_PtrLcv_detail_SurfFeaturesFinderGpuG_to_PtrOfDetail_FeaturesFinder }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_SurfFeaturesFinderGpu> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_SurfFeaturesFinderGpu")
			.finish()
	}
}

