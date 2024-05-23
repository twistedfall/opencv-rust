ptr_extern! { crate::ml::ParamGrid,
	cv_PtrLcv_ml_ParamGridG_new_null_const, cv_PtrLcv_ml_ParamGridG_delete, cv_PtrLcv_ml_ParamGridG_getInnerPtr_const, cv_PtrLcv_ml_ParamGridG_getInnerPtrMut
}

ptr_extern_ctor! { crate::ml::ParamGrid, cv_PtrLcv_ml_ParamGridG_new_const_ParamGrid }
impl core::Ptr<crate::ml::ParamGrid> {
	#[inline] pub fn as_raw_PtrOfParamGrid(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfParamGrid(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ml::ParamGridTraitConst for core::Ptr<crate::ml::ParamGrid> {
	#[inline] fn as_raw_ParamGrid(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::ParamGridTrait for core::Ptr<crate::ml::ParamGrid> {
	#[inline] fn as_raw_mut_ParamGrid(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::ml::ParamGrid> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfParamGrid")
			.field("min_val", &crate::ml::ParamGridTraitConst::min_val(self))
			.field("max_val", &crate::ml::ParamGridTraitConst::max_val(self))
			.field("log_step", &crate::ml::ParamGridTraitConst::log_step(self))
			.finish()
	}
}

