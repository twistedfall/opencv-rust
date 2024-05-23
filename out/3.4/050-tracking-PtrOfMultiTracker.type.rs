ptr_extern! { crate::tracking::MultiTracker,
	cv_PtrLcv_MultiTrackerG_new_null_const, cv_PtrLcv_MultiTrackerG_delete, cv_PtrLcv_MultiTrackerG_getInnerPtr_const, cv_PtrLcv_MultiTrackerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::tracking::MultiTracker, cv_PtrLcv_MultiTrackerG_new_const_MultiTracker }
impl core::Ptr<crate::tracking::MultiTracker> {
	#[inline] pub fn as_raw_PtrOfMultiTracker(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMultiTracker(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::tracking::MultiTrackerTraitConst for core::Ptr<crate::tracking::MultiTracker> {
	#[inline] fn as_raw_MultiTracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::MultiTrackerTrait for core::Ptr<crate::tracking::MultiTracker> {
	#[inline] fn as_raw_mut_MultiTracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::tracking::MultiTracker> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::tracking::MultiTracker> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::MultiTracker>, core::Ptr<core::Algorithm>, cv_PtrLcv_MultiTrackerG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::tracking::MultiTracker> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMultiTracker")
			.finish()
	}
}

