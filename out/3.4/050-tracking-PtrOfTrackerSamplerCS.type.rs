ptr_extern! { crate::tracking::TrackerSamplerCS,
	cv_PtrLcv_TrackerSamplerCSG_new_null_const, cv_PtrLcv_TrackerSamplerCSG_delete, cv_PtrLcv_TrackerSamplerCSG_getInnerPtr_const, cv_PtrLcv_TrackerSamplerCSG_getInnerPtrMut
}

ptr_extern_ctor! { crate::tracking::TrackerSamplerCS, cv_PtrLcv_TrackerSamplerCSG_new_const_TrackerSamplerCS }
impl core::Ptr<crate::tracking::TrackerSamplerCS> {
	#[inline] pub fn as_raw_PtrOfTrackerSamplerCS(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerSamplerCS(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::tracking::TrackerSamplerCSTraitConst for core::Ptr<crate::tracking::TrackerSamplerCS> {
	#[inline] fn as_raw_TrackerSamplerCS(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerSamplerCSTrait for core::Ptr<crate::tracking::TrackerSamplerCS> {
	#[inline] fn as_raw_mut_TrackerSamplerCS(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::tracking::TrackerSamplerAlgorithmTraitConst for core::Ptr<crate::tracking::TrackerSamplerCS> {
	#[inline] fn as_raw_TrackerSamplerAlgorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerSamplerAlgorithmTrait for core::Ptr<crate::tracking::TrackerSamplerCS> {
	#[inline] fn as_raw_mut_TrackerSamplerAlgorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerSamplerCS>, core::Ptr<crate::tracking::TrackerSamplerAlgorithm>, cv_PtrLcv_TrackerSamplerCSG_to_PtrOfTrackerSamplerAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::tracking::TrackerSamplerCS> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerSamplerCS")
			.finish()
	}
}

