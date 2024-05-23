ptr_extern! { crate::mod_3d::RegionGrowing3D,
	cv_PtrLcv_RegionGrowing3DG_new_null_const, cv_PtrLcv_RegionGrowing3DG_delete, cv_PtrLcv_RegionGrowing3DG_getInnerPtr_const, cv_PtrLcv_RegionGrowing3DG_getInnerPtrMut
}

impl core::Ptr<crate::mod_3d::RegionGrowing3D> {
	#[inline] pub fn as_raw_PtrOfRegionGrowing3D(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRegionGrowing3D(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::mod_3d::RegionGrowing3DTraitConst for core::Ptr<crate::mod_3d::RegionGrowing3D> {
	#[inline] fn as_raw_RegionGrowing3D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::mod_3d::RegionGrowing3DTrait for core::Ptr<crate::mod_3d::RegionGrowing3D> {
	#[inline] fn as_raw_mut_RegionGrowing3D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::mod_3d::RegionGrowing3D> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfRegionGrowing3D")
			.finish()
	}
}

