ptr_extern! { crate::photo::TonemapDrago,
	cv_PtrLcv_TonemapDragoG_new_null_const, cv_PtrLcv_TonemapDragoG_delete, cv_PtrLcv_TonemapDragoG_getInnerPtr_const, cv_PtrLcv_TonemapDragoG_getInnerPtrMut
}

impl core::Ptr<crate::photo::TonemapDrago> {
	#[inline] pub fn as_raw_PtrOfTonemapDrago(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTonemapDrago(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::photo::TonemapDragoTraitConst for core::Ptr<crate::photo::TonemapDrago> {
	#[inline] fn as_raw_TonemapDrago(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::TonemapDragoTrait for core::Ptr<crate::photo::TonemapDrago> {
	#[inline] fn as_raw_mut_TonemapDrago(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::photo::TonemapDrago> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::photo::TonemapDrago> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::photo::TonemapDrago>, core::Ptr<core::Algorithm>, cv_PtrLcv_TonemapDragoG_to_PtrOfAlgorithm }

impl crate::photo::TonemapTraitConst for core::Ptr<crate::photo::TonemapDrago> {
	#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::TonemapTrait for core::Ptr<crate::photo::TonemapDrago> {
	#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::photo::TonemapDrago>, core::Ptr<crate::photo::Tonemap>, cv_PtrLcv_TonemapDragoG_to_PtrOfTonemap }

impl std::fmt::Debug for core::Ptr<crate::photo::TonemapDrago> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTonemapDrago")
			.finish()
	}
}

