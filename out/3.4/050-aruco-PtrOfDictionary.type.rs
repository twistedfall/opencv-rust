ptr_extern! { crate::aruco::Dictionary,
	cv_PtrLcv_aruco_DictionaryG_new_null_const, cv_PtrLcv_aruco_DictionaryG_delete, cv_PtrLcv_aruco_DictionaryG_getInnerPtr_const, cv_PtrLcv_aruco_DictionaryG_getInnerPtrMut
}

ptr_extern_ctor! { crate::aruco::Dictionary, cv_PtrLcv_aruco_DictionaryG_new_const_Dictionary }
impl core::Ptr<crate::aruco::Dictionary> {
	#[inline] pub fn as_raw_PtrOfDictionary(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDictionary(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::aruco::DictionaryTraitConst for core::Ptr<crate::aruco::Dictionary> {
	#[inline] fn as_raw_Dictionary(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::aruco::DictionaryTrait for core::Ptr<crate::aruco::Dictionary> {
	#[inline] fn as_raw_mut_Dictionary(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::aruco::Dictionary> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDictionary")
			.field("bytes_list", &crate::aruco::DictionaryTraitConst::bytes_list(self))
			.field("marker_size", &crate::aruco::DictionaryTraitConst::marker_size(self))
			.field("max_correction_bits", &crate::aruco::DictionaryTraitConst::max_correction_bits(self))
			.finish()
	}
}

