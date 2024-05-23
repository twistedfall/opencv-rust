ptr_extern! { crate::text::ERFilter,
	cv_PtrLcv_text_ERFilterG_new_null_const, cv_PtrLcv_text_ERFilterG_delete, cv_PtrLcv_text_ERFilterG_getInnerPtr_const, cv_PtrLcv_text_ERFilterG_getInnerPtrMut
}

impl core::Ptr<crate::text::ERFilter> {
	#[inline] pub fn as_raw_PtrOfERFilter(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfERFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::text::ERFilterTraitConst for core::Ptr<crate::text::ERFilter> {
	#[inline] fn as_raw_ERFilter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::ERFilterTrait for core::Ptr<crate::text::ERFilter> {
	#[inline] fn as_raw_mut_ERFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::text::ERFilter> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::text::ERFilter> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::text::ERFilter>, core::Ptr<core::Algorithm>, cv_PtrLcv_text_ERFilterG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::text::ERFilter> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfERFilter")
			.finish()
	}
}

