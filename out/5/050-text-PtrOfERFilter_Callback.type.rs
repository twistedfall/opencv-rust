ptr_extern! { crate::text::ERFilter_Callback,
	cv_PtrLcv_text_ERFilter_CallbackG_new_null_const, cv_PtrLcv_text_ERFilter_CallbackG_delete, cv_PtrLcv_text_ERFilter_CallbackG_getInnerPtr_const, cv_PtrLcv_text_ERFilter_CallbackG_getInnerPtrMut
}

impl core::Ptr<crate::text::ERFilter_Callback> {
	#[inline] pub fn as_raw_PtrOfERFilter_Callback(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfERFilter_Callback(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::text::ERFilter_CallbackTraitConst for core::Ptr<crate::text::ERFilter_Callback> {
	#[inline] fn as_raw_ERFilter_Callback(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::ERFilter_CallbackTrait for core::Ptr<crate::text::ERFilter_Callback> {
	#[inline] fn as_raw_mut_ERFilter_Callback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::text::ERFilter_Callback> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfERFilter_Callback")
			.finish()
	}
}

