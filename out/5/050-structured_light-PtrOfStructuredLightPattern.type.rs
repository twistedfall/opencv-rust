ptr_extern! { crate::structured_light::StructuredLightPattern,
	cv_PtrLcv_structured_light_StructuredLightPatternG_new_null_const, cv_PtrLcv_structured_light_StructuredLightPatternG_delete, cv_PtrLcv_structured_light_StructuredLightPatternG_getInnerPtr_const, cv_PtrLcv_structured_light_StructuredLightPatternG_getInnerPtrMut
}

impl core::Ptr<crate::structured_light::StructuredLightPattern> {
	#[inline] pub fn as_raw_PtrOfStructuredLightPattern(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfStructuredLightPattern(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::structured_light::StructuredLightPatternTraitConst for core::Ptr<crate::structured_light::StructuredLightPattern> {
	#[inline] fn as_raw_StructuredLightPattern(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::structured_light::StructuredLightPatternTrait for core::Ptr<crate::structured_light::StructuredLightPattern> {
	#[inline] fn as_raw_mut_StructuredLightPattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::structured_light::StructuredLightPattern> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::structured_light::StructuredLightPattern> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::structured_light::StructuredLightPattern>, core::Ptr<core::Algorithm>, cv_PtrLcv_structured_light_StructuredLightPatternG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::structured_light::StructuredLightPattern> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfStructuredLightPattern")
			.finish()
	}
}

