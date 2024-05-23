ptr_extern! { crate::videostab::ColorAverageInpainter,
	cv_PtrLcv_videostab_ColorAverageInpainterG_new_null_const, cv_PtrLcv_videostab_ColorAverageInpainterG_delete, cv_PtrLcv_videostab_ColorAverageInpainterG_getInnerPtr_const, cv_PtrLcv_videostab_ColorAverageInpainterG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::ColorAverageInpainter, cv_PtrLcv_videostab_ColorAverageInpainterG_new_const_ColorAverageInpainter }
impl core::Ptr<crate::videostab::ColorAverageInpainter> {
	#[inline] pub fn as_raw_PtrOfColorAverageInpainter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfColorAverageInpainter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::ColorAverageInpainterTraitConst for core::Ptr<crate::videostab::ColorAverageInpainter> {
	#[inline] fn as_raw_ColorAverageInpainter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ColorAverageInpainterTrait for core::Ptr<crate::videostab::ColorAverageInpainter> {
	#[inline] fn as_raw_mut_ColorAverageInpainter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::InpainterBaseTraitConst for core::Ptr<crate::videostab::ColorAverageInpainter> {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::InpainterBaseTrait for core::Ptr<crate::videostab::ColorAverageInpainter> {
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::ColorAverageInpainter>, core::Ptr<crate::videostab::InpainterBase>, cv_PtrLcv_videostab_ColorAverageInpainterG_to_PtrOfInpainterBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::ColorAverageInpainter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfColorAverageInpainter")
			.finish()
	}
}

