ptr_extern! { crate::xstereo::QuasiDenseStereo,
	cv_PtrLcv_stereo_QuasiDenseStereoG_new_null_const, cv_PtrLcv_stereo_QuasiDenseStereoG_delete, cv_PtrLcv_stereo_QuasiDenseStereoG_getInnerPtr_const, cv_PtrLcv_stereo_QuasiDenseStereoG_getInnerPtrMut
}

impl core::Ptr<crate::xstereo::QuasiDenseStereo> {
	#[inline] pub fn as_raw_PtrOfQuasiDenseStereo(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfQuasiDenseStereo(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xstereo::QuasiDenseStereoTraitConst for core::Ptr<crate::xstereo::QuasiDenseStereo> {
	#[inline] fn as_raw_QuasiDenseStereo(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xstereo::QuasiDenseStereoTrait for core::Ptr<crate::xstereo::QuasiDenseStereo> {
	#[inline] fn as_raw_mut_QuasiDenseStereo(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::xstereo::QuasiDenseStereo> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfQuasiDenseStereo")
			.field("param", &crate::xstereo::QuasiDenseStereoTraitConst::param(self))
			.finish()
	}
}

