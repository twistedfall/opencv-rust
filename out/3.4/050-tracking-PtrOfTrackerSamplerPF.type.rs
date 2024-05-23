ptr_extern! { crate::tracking::TrackerSamplerPF,
	cv_PtrLcv_TrackerSamplerPFG_new_null_const, cv_PtrLcv_TrackerSamplerPFG_delete, cv_PtrLcv_TrackerSamplerPFG_getInnerPtr_const, cv_PtrLcv_TrackerSamplerPFG_getInnerPtrMut
}

ptr_extern_ctor! { crate::tracking::TrackerSamplerPF, cv_PtrLcv_TrackerSamplerPFG_new_const_TrackerSamplerPF }
impl core::Ptr<crate::tracking::TrackerSamplerPF> {
	#[inline] pub fn as_raw_PtrOfTrackerSamplerPF(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerSamplerPF(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::tracking::TrackerSamplerPFTraitConst for core::Ptr<crate::tracking::TrackerSamplerPF> {
	#[inline] fn as_raw_TrackerSamplerPF(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerSamplerPFTrait for core::Ptr<crate::tracking::TrackerSamplerPF> {
	#[inline] fn as_raw_mut_TrackerSamplerPF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::tracking::TrackerSamplerAlgorithmTraitConst for core::Ptr<crate::tracking::TrackerSamplerPF> {
	#[inline] fn as_raw_TrackerSamplerAlgorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerSamplerAlgorithmTrait for core::Ptr<crate::tracking::TrackerSamplerPF> {
	#[inline] fn as_raw_mut_TrackerSamplerAlgorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerSamplerPF>, core::Ptr<crate::tracking::TrackerSamplerAlgorithm>, cv_PtrLcv_TrackerSamplerPFG_to_PtrOfTrackerSamplerAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::tracking::TrackerSamplerPF> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerSamplerPF")
			.finish()
	}
}

