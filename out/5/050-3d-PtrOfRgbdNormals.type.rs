ptr_extern! { crate::mod_3d::RgbdNormals,
	cv_PtrLcv_RgbdNormalsG_new_null_const, cv_PtrLcv_RgbdNormalsG_delete, cv_PtrLcv_RgbdNormalsG_getInnerPtr_const, cv_PtrLcv_RgbdNormalsG_getInnerPtrMut
}

impl core::Ptr<crate::mod_3d::RgbdNormals> {
	#[inline] pub fn as_raw_PtrOfRgbdNormals(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRgbdNormals(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::mod_3d::RgbdNormalsTraitConst for core::Ptr<crate::mod_3d::RgbdNormals> {
	#[inline] fn as_raw_RgbdNormals(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::mod_3d::RgbdNormalsTrait for core::Ptr<crate::mod_3d::RgbdNormals> {
	#[inline] fn as_raw_mut_RgbdNormals(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::mod_3d::RgbdNormals> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfRgbdNormals")
			.finish()
	}
}

