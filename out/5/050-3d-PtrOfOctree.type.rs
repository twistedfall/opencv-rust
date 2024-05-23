ptr_extern! { crate::mod_3d::Octree,
	cv_PtrLcv_OctreeG_new_null_const, cv_PtrLcv_OctreeG_delete, cv_PtrLcv_OctreeG_getInnerPtr_const, cv_PtrLcv_OctreeG_getInnerPtrMut
}

ptr_extern_ctor! { crate::mod_3d::Octree, cv_PtrLcv_OctreeG_new_const_Octree }
impl core::Ptr<crate::mod_3d::Octree> {
	#[inline] pub fn as_raw_PtrOfOctree(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOctree(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::mod_3d::OctreeTraitConst for core::Ptr<crate::mod_3d::Octree> {
	#[inline] fn as_raw_Octree(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::mod_3d::OctreeTrait for core::Ptr<crate::mod_3d::Octree> {
	#[inline] fn as_raw_mut_Octree(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::mod_3d::Octree> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfOctree")
			.finish()
	}
}

