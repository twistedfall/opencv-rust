ptr_extern! { crate::videostab::MotionInpainter,
	cv_PtrLcv_videostab_MotionInpainterG_new_null_const, cv_PtrLcv_videostab_MotionInpainterG_delete, cv_PtrLcv_videostab_MotionInpainterG_getInnerPtr_const, cv_PtrLcv_videostab_MotionInpainterG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::MotionInpainter, cv_PtrLcv_videostab_MotionInpainterG_new_const_MotionInpainter }
impl core::Ptr<crate::videostab::MotionInpainter> {
	#[inline] pub fn as_raw_PtrOfMotionInpainter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMotionInpainter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::MotionInpainterTraitConst for core::Ptr<crate::videostab::MotionInpainter> {
	#[inline] fn as_raw_MotionInpainter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MotionInpainterTrait for core::Ptr<crate::videostab::MotionInpainter> {
	#[inline] fn as_raw_mut_MotionInpainter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::InpainterBaseTraitConst for core::Ptr<crate::videostab::MotionInpainter> {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::InpainterBaseTrait for core::Ptr<crate::videostab::MotionInpainter> {
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::MotionInpainter>, core::Ptr<crate::videostab::InpainterBase>, cv_PtrLcv_videostab_MotionInpainterG_to_PtrOfInpainterBase }

impl std::fmt::Debug for core::Ptr<crate::videostab::MotionInpainter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMotionInpainter")
			.finish()
	}
}

