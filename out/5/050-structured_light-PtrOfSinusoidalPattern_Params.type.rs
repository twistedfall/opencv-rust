ptr_extern! { crate::structured_light::SinusoidalPattern_Params,
	cv_PtrLcv_structured_light_SinusoidalPattern_ParamsG_new_null_const, cv_PtrLcv_structured_light_SinusoidalPattern_ParamsG_delete, cv_PtrLcv_structured_light_SinusoidalPattern_ParamsG_getInnerPtr_const, cv_PtrLcv_structured_light_SinusoidalPattern_ParamsG_getInnerPtrMut
}

ptr_extern_ctor! { crate::structured_light::SinusoidalPattern_Params, cv_PtrLcv_structured_light_SinusoidalPattern_ParamsG_new_const_Params }
impl core::Ptr<crate::structured_light::SinusoidalPattern_Params> {
	#[inline] pub fn as_raw_PtrOfSinusoidalPattern_Params(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSinusoidalPattern_Params(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::structured_light::SinusoidalPattern_ParamsTraitConst for core::Ptr<crate::structured_light::SinusoidalPattern_Params> {
	#[inline] fn as_raw_SinusoidalPattern_Params(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::structured_light::SinusoidalPattern_ParamsTrait for core::Ptr<crate::structured_light::SinusoidalPattern_Params> {
	#[inline] fn as_raw_mut_SinusoidalPattern_Params(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::structured_light::SinusoidalPattern_Params> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSinusoidalPattern_Params")
			.field("width", &crate::structured_light::SinusoidalPattern_ParamsTraitConst::width(self))
			.field("height", &crate::structured_light::SinusoidalPattern_ParamsTraitConst::height(self))
			.field("nbr_of_periods", &crate::structured_light::SinusoidalPattern_ParamsTraitConst::nbr_of_periods(self))
			.field("shift_value", &crate::structured_light::SinusoidalPattern_ParamsTraitConst::shift_value(self))
			.field("method_id", &crate::structured_light::SinusoidalPattern_ParamsTraitConst::method_id(self))
			.field("nbr_of_pixels_between_markers", &crate::structured_light::SinusoidalPattern_ParamsTraitConst::nbr_of_pixels_between_markers(self))
			.field("horizontal", &crate::structured_light::SinusoidalPattern_ParamsTraitConst::horizontal(self))
			.field("set_markers", &crate::structured_light::SinusoidalPattern_ParamsTraitConst::set_markers(self))
			.field("markers_location", &crate::structured_light::SinusoidalPattern_ParamsTraitConst::markers_location(self))
			.finish()
	}
}

