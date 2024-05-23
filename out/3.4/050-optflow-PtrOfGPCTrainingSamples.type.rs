ptr_extern! { crate::optflow::GPCTrainingSamples,
	cv_PtrLcv_optflow_GPCTrainingSamplesG_new_null_const, cv_PtrLcv_optflow_GPCTrainingSamplesG_delete, cv_PtrLcv_optflow_GPCTrainingSamplesG_getInnerPtr_const, cv_PtrLcv_optflow_GPCTrainingSamplesG_getInnerPtrMut
}

ptr_extern_ctor! { crate::optflow::GPCTrainingSamples, cv_PtrLcv_optflow_GPCTrainingSamplesG_new_const_GPCTrainingSamples }
impl core::Ptr<crate::optflow::GPCTrainingSamples> {
	#[inline] pub fn as_raw_PtrOfGPCTrainingSamples(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGPCTrainingSamples(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::optflow::GPCTrainingSamplesTraitConst for core::Ptr<crate::optflow::GPCTrainingSamples> {
	#[inline] fn as_raw_GPCTrainingSamples(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::optflow::GPCTrainingSamplesTrait for core::Ptr<crate::optflow::GPCTrainingSamples> {
	#[inline] fn as_raw_mut_GPCTrainingSamples(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::optflow::GPCTrainingSamples> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfGPCTrainingSamples")
			.finish()
	}
}

