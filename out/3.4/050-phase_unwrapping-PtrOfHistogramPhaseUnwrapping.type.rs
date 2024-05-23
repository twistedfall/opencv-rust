ptr_extern! { crate::phase_unwrapping::HistogramPhaseUnwrapping,
	cv_PtrLcv_phase_unwrapping_HistogramPhaseUnwrappingG_new_null_const, cv_PtrLcv_phase_unwrapping_HistogramPhaseUnwrappingG_delete, cv_PtrLcv_phase_unwrapping_HistogramPhaseUnwrappingG_getInnerPtr_const, cv_PtrLcv_phase_unwrapping_HistogramPhaseUnwrappingG_getInnerPtrMut
}

impl core::Ptr<crate::phase_unwrapping::HistogramPhaseUnwrapping> {
	#[inline] pub fn as_raw_PtrOfHistogramPhaseUnwrapping(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfHistogramPhaseUnwrapping(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::phase_unwrapping::HistogramPhaseUnwrappingTraitConst for core::Ptr<crate::phase_unwrapping::HistogramPhaseUnwrapping> {
	#[inline] fn as_raw_HistogramPhaseUnwrapping(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::phase_unwrapping::HistogramPhaseUnwrappingTrait for core::Ptr<crate::phase_unwrapping::HistogramPhaseUnwrapping> {
	#[inline] fn as_raw_mut_HistogramPhaseUnwrapping(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::phase_unwrapping::HistogramPhaseUnwrapping> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::phase_unwrapping::HistogramPhaseUnwrapping> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::phase_unwrapping::HistogramPhaseUnwrapping>, core::Ptr<core::Algorithm>, cv_PtrLcv_phase_unwrapping_HistogramPhaseUnwrappingG_to_PtrOfAlgorithm }

impl crate::phase_unwrapping::PhaseUnwrappingTraitConst for core::Ptr<crate::phase_unwrapping::HistogramPhaseUnwrapping> {
	#[inline] fn as_raw_PhaseUnwrapping(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::phase_unwrapping::PhaseUnwrappingTrait for core::Ptr<crate::phase_unwrapping::HistogramPhaseUnwrapping> {
	#[inline] fn as_raw_mut_PhaseUnwrapping(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::phase_unwrapping::HistogramPhaseUnwrapping>, core::Ptr<crate::phase_unwrapping::PhaseUnwrapping>, cv_PtrLcv_phase_unwrapping_HistogramPhaseUnwrappingG_to_PtrOfPhaseUnwrapping }

impl std::fmt::Debug for core::Ptr<crate::phase_unwrapping::HistogramPhaseUnwrapping> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfHistogramPhaseUnwrapping")
			.finish()
	}
}

