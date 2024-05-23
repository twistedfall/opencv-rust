ptr_extern! { crate::structured_light::GrayCodePattern,
	cv_PtrLcv_structured_light_GrayCodePatternG_new_null_const, cv_PtrLcv_structured_light_GrayCodePatternG_delete, cv_PtrLcv_structured_light_GrayCodePatternG_getInnerPtr_const, cv_PtrLcv_structured_light_GrayCodePatternG_getInnerPtrMut
}

impl core::Ptr<crate::structured_light::GrayCodePattern> {
	#[inline] pub fn as_raw_PtrOfGrayCodePattern(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGrayCodePattern(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::structured_light::GrayCodePatternTraitConst for core::Ptr<crate::structured_light::GrayCodePattern> {
	#[inline] fn as_raw_GrayCodePattern(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::structured_light::GrayCodePatternTrait for core::Ptr<crate::structured_light::GrayCodePattern> {
	#[inline] fn as_raw_mut_GrayCodePattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::structured_light::GrayCodePattern> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::structured_light::GrayCodePattern> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::structured_light::GrayCodePattern>, core::Ptr<core::Algorithm>, cv_PtrLcv_structured_light_GrayCodePatternG_to_PtrOfAlgorithm }

impl crate::structured_light::StructuredLightPatternTraitConst for core::Ptr<crate::structured_light::GrayCodePattern> {
	#[inline] fn as_raw_StructuredLightPattern(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::structured_light::StructuredLightPatternTrait for core::Ptr<crate::structured_light::GrayCodePattern> {
	#[inline] fn as_raw_mut_StructuredLightPattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::structured_light::GrayCodePattern>, core::Ptr<crate::structured_light::StructuredLightPattern>, cv_PtrLcv_structured_light_GrayCodePatternG_to_PtrOfStructuredLightPattern }

impl std::fmt::Debug for core::Ptr<crate::structured_light::GrayCodePattern> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfGrayCodePattern")
			.finish()
	}
}

