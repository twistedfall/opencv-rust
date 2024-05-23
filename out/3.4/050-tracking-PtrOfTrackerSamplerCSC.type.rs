ptr_extern! { crate::tracking::TrackerSamplerCSC,
	cv_PtrLcv_TrackerSamplerCSCG_new_null_const, cv_PtrLcv_TrackerSamplerCSCG_delete, cv_PtrLcv_TrackerSamplerCSCG_getInnerPtr_const, cv_PtrLcv_TrackerSamplerCSCG_getInnerPtrMut
}

ptr_extern_ctor! { crate::tracking::TrackerSamplerCSC, cv_PtrLcv_TrackerSamplerCSCG_new_const_TrackerSamplerCSC }
impl core::Ptr<crate::tracking::TrackerSamplerCSC> {
	#[inline] pub fn as_raw_PtrOfTrackerSamplerCSC(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerSamplerCSC(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::tracking::TrackerSamplerCSCTraitConst for core::Ptr<crate::tracking::TrackerSamplerCSC> {
	#[inline] fn as_raw_TrackerSamplerCSC(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerSamplerCSCTrait for core::Ptr<crate::tracking::TrackerSamplerCSC> {
	#[inline] fn as_raw_mut_TrackerSamplerCSC(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::tracking::TrackerSamplerAlgorithmTraitConst for core::Ptr<crate::tracking::TrackerSamplerCSC> {
	#[inline] fn as_raw_TrackerSamplerAlgorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerSamplerAlgorithmTrait for core::Ptr<crate::tracking::TrackerSamplerCSC> {
	#[inline] fn as_raw_mut_TrackerSamplerAlgorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerSamplerCSC>, core::Ptr<crate::tracking::TrackerSamplerAlgorithm>, cv_PtrLcv_TrackerSamplerCSCG_to_PtrOfTrackerSamplerAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::tracking::TrackerSamplerCSC> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerSamplerCSC")
			.finish()
	}
}

