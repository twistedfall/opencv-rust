ptr_extern! { crate::videostab::ColorInpainter,
	cv_PtrLcv_videostab_ColorInpainterG_new_null_const, cv_PtrLcv_videostab_ColorInpainterG_delete, cv_PtrLcv_videostab_ColorInpainterG_getInnerPtr_const, cv_PtrLcv_videostab_ColorInpainterG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::ColorInpainter, cv_PtrLcv_videostab_ColorInpainterG_new_const_ColorInpainter }
impl core::Ptr<crate::videostab::ColorInpainter> {
	#[inline] pub fn as_raw_PtrOfColorInpainter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfColorInpainter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::ColorInpainterTraitConst for core::Ptr<crate::videostab::ColorInpainter> {
	#[inline] fn as_raw_ColorInpainter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ColorInpainterTrait for core::Ptr<crate::videostab::ColorInpainter> {
	#[inline] fn as_raw_mut_ColorInpainter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::InpainterBaseTraitConst for core::Ptr<crate::videostab::ColorInpainter> {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::InpainterBaseTrait for core::Ptr<crate::videostab::ColorInpainter> {
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::ColorInpainter>, core::Ptr<crate::videostab::InpainterBase>, cv_PtrLcv_videostab_ColorInpainterG_to_PtrOfInpainterBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::ColorInpainter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfColorInpainter")
			.finish()
	}
}

