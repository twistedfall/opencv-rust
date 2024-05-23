ptr_extern! { crate::phase_unwrapping::PhaseUnwrapping,
	cv_PtrLcv_phase_unwrapping_PhaseUnwrappingG_new_null_const, cv_PtrLcv_phase_unwrapping_PhaseUnwrappingG_delete, cv_PtrLcv_phase_unwrapping_PhaseUnwrappingG_getInnerPtr_const, cv_PtrLcv_phase_unwrapping_PhaseUnwrappingG_getInnerPtrMut
}

impl core::Ptr<crate::phase_unwrapping::PhaseUnwrapping> {
	#[inline] pub fn as_raw_PtrOfPhaseUnwrapping(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPhaseUnwrapping(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::phase_unwrapping::PhaseUnwrappingTraitConst for core::Ptr<crate::phase_unwrapping::PhaseUnwrapping> {
	#[inline] fn as_raw_PhaseUnwrapping(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::phase_unwrapping::PhaseUnwrappingTrait for core::Ptr<crate::phase_unwrapping::PhaseUnwrapping> {
	#[inline] fn as_raw_mut_PhaseUnwrapping(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::phase_unwrapping::PhaseUnwrapping> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::phase_unwrapping::PhaseUnwrapping> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::phase_unwrapping::PhaseUnwrapping>, core::Ptr<core::Algorithm>, cv_PtrLcv_phase_unwrapping_PhaseUnwrappingG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::phase_unwrapping::PhaseUnwrapping> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfPhaseUnwrapping")
			.finish()
	}
}

