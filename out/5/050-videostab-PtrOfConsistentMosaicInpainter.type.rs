ptr_extern! { crate::videostab::ConsistentMosaicInpainter,
	cv_PtrLcv_videostab_ConsistentMosaicInpainterG_new_null_const, cv_PtrLcv_videostab_ConsistentMosaicInpainterG_delete, cv_PtrLcv_videostab_ConsistentMosaicInpainterG_getInnerPtr_const, cv_PtrLcv_videostab_ConsistentMosaicInpainterG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::ConsistentMosaicInpainter, cv_PtrLcv_videostab_ConsistentMosaicInpainterG_new_const_ConsistentMosaicInpainter }
impl core::Ptr<crate::videostab::ConsistentMosaicInpainter> {
	#[inline] pub fn as_raw_PtrOfConsistentMosaicInpainter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfConsistentMosaicInpainter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::ConsistentMosaicInpainterTraitConst for core::Ptr<crate::videostab::ConsistentMosaicInpainter> {
	#[inline] fn as_raw_ConsistentMosaicInpainter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ConsistentMosaicInpainterTrait for core::Ptr<crate::videostab::ConsistentMosaicInpainter> {
	#[inline] fn as_raw_mut_ConsistentMosaicInpainter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::InpainterBaseTraitConst for core::Ptr<crate::videostab::ConsistentMosaicInpainter> {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::InpainterBaseTrait for core::Ptr<crate::videostab::ConsistentMosaicInpainter> {
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::ConsistentMosaicInpainter>, core::Ptr<crate::videostab::InpainterBase>, cv_PtrLcv_videostab_ConsistentMosaicInpainterG_to_PtrOfInpainterBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::ConsistentMosaicInpainter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfConsistentMosaicInpainter")
			.finish()
	}
}

