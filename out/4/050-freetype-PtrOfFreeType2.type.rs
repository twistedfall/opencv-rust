ptr_extern! { crate::freetype::FreeType2,
	cv_PtrLcv_freetype_FreeType2G_new_null_const, cv_PtrLcv_freetype_FreeType2G_delete, cv_PtrLcv_freetype_FreeType2G_getInnerPtr_const, cv_PtrLcv_freetype_FreeType2G_getInnerPtrMut
}

impl core::Ptr<crate::freetype::FreeType2> {
	#[inline] pub fn as_raw_PtrOfFreeType2(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFreeType2(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::freetype::FreeType2TraitConst for core::Ptr<crate::freetype::FreeType2> {
	#[inline] fn as_raw_FreeType2(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::freetype::FreeType2Trait for core::Ptr<crate::freetype::FreeType2> {
	#[inline] fn as_raw_mut_FreeType2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::freetype::FreeType2> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::freetype::FreeType2> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::freetype::FreeType2>, core::Ptr<core::Algorithm>, cv_PtrLcv_freetype_FreeType2G_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::freetype::FreeType2> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFreeType2")
			.finish()
	}
}

