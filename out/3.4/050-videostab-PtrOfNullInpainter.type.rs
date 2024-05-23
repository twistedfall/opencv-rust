ptr_extern! { crate::videostab::NullInpainter,
	cv_PtrLcv_videostab_NullInpainterG_new_null_const, cv_PtrLcv_videostab_NullInpainterG_delete, cv_PtrLcv_videostab_NullInpainterG_getInnerPtr_const, cv_PtrLcv_videostab_NullInpainterG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::NullInpainter, cv_PtrLcv_videostab_NullInpainterG_new_const_NullInpainter }
impl core::Ptr<crate::videostab::NullInpainter> {
	#[inline] pub fn as_raw_PtrOfNullInpainter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfNullInpainter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::NullInpainterTraitConst for core::Ptr<crate::videostab::NullInpainter> {
	#[inline] fn as_raw_NullInpainter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::NullInpainterTrait for core::Ptr<crate::videostab::NullInpainter> {
	#[inline] fn as_raw_mut_NullInpainter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::InpainterBaseTraitConst for core::Ptr<crate::videostab::NullInpainter> {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::InpainterBaseTrait for core::Ptr<crate::videostab::NullInpainter> {
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::NullInpainter>, core::Ptr<crate::videostab::InpainterBase>, cv_PtrLcv_videostab_NullInpainterG_to_PtrOfInpainterBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::NullInpainter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfNullInpainter")
			.finish()
	}
}

