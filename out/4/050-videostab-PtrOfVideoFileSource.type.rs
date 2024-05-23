ptr_extern! { crate::videostab::VideoFileSource,
	cv_PtrLcv_videostab_VideoFileSourceG_new_null_const, cv_PtrLcv_videostab_VideoFileSourceG_delete, cv_PtrLcv_videostab_VideoFileSourceG_getInnerPtr_const, cv_PtrLcv_videostab_VideoFileSourceG_getInnerPtrMut
}

ptr_extern_ctor! { crate::videostab::VideoFileSource, cv_PtrLcv_videostab_VideoFileSourceG_new_const_VideoFileSource }
impl core::Ptr<crate::videostab::VideoFileSource> {
	#[inline] pub fn as_raw_PtrOfVideoFileSource(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfVideoFileSource(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::videostab::VideoFileSourceTraitConst for core::Ptr<crate::videostab::VideoFileSource> {
	#[inline] fn as_raw_VideoFileSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::VideoFileSourceTrait for core::Ptr<crate::videostab::VideoFileSource> {
	#[inline] fn as_raw_mut_VideoFileSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IFrameSourceTraitConst for core::Ptr<crate::videostab::VideoFileSource> {
	#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IFrameSourceTrait for core::Ptr<crate::videostab::VideoFileSource> {
	#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::videostab::VideoFileSource>, core::Ptr<crate::videostab::IFrameSource>, cv_PtrLcv_videostab_VideoFileSourceG_to_PtrOfIFrameSource }

impl std::fmt::Debug for core::Ptr<crate::videostab::VideoFileSource> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfVideoFileSource")
			.finish()
	}
}

