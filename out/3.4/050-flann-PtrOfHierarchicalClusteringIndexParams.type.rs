ptr_extern! { crate::flann::HierarchicalClusteringIndexParams,
	cv_PtrLcv_flann_HierarchicalClusteringIndexParamsG_new_null_const, cv_PtrLcv_flann_HierarchicalClusteringIndexParamsG_delete, cv_PtrLcv_flann_HierarchicalClusteringIndexParamsG_getInnerPtr_const, cv_PtrLcv_flann_HierarchicalClusteringIndexParamsG_getInnerPtrMut
}

ptr_extern_ctor! { crate::flann::HierarchicalClusteringIndexParams, cv_PtrLcv_flann_HierarchicalClusteringIndexParamsG_new_const_HierarchicalClusteringIndexParams }
impl core::Ptr<crate::flann::HierarchicalClusteringIndexParams> {
	#[inline] pub fn as_raw_PtrOfHierarchicalClusteringIndexParams(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfHierarchicalClusteringIndexParams(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::flann::HierarchicalClusteringIndexParamsTraitConst for core::Ptr<crate::flann::HierarchicalClusteringIndexParams> {
	#[inline] fn as_raw_HierarchicalClusteringIndexParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::flann::HierarchicalClusteringIndexParamsTrait for core::Ptr<crate::flann::HierarchicalClusteringIndexParams> {
	#[inline] fn as_raw_mut_HierarchicalClusteringIndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::flann::IndexParamsTraitConst for core::Ptr<crate::flann::HierarchicalClusteringIndexParams> {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::flann::IndexParamsTrait for core::Ptr<crate::flann::HierarchicalClusteringIndexParams> {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::flann::HierarchicalClusteringIndexParams>, core::Ptr<crate::flann::IndexParams>, cv_PtrLcv_flann_HierarchicalClusteringIndexParamsG_to_PtrOfIndexParams }

impl std::fmt::Debug for core::Ptr<crate::flann::HierarchicalClusteringIndexParams> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfHierarchicalClusteringIndexParams")
			.finish()
	}
}

