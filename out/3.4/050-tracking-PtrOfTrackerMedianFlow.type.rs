ptr_extern! { crate::tracking::TrackerMedianFlow,
	cv_PtrLcv_TrackerMedianFlowG_new_null_const, cv_PtrLcv_TrackerMedianFlowG_delete, cv_PtrLcv_TrackerMedianFlowG_getInnerPtr_const, cv_PtrLcv_TrackerMedianFlowG_getInnerPtrMut
}

impl core::Ptr<crate::tracking::TrackerMedianFlow> {
	#[inline] pub fn as_raw_PtrOfTrackerMedianFlow(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerMedianFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::tracking::TrackerMedianFlowTraitConst for core::Ptr<crate::tracking::TrackerMedianFlow> {
	#[inline] fn as_raw_TrackerMedianFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerMedianFlowTrait for core::Ptr<crate::tracking::TrackerMedianFlow> {
	#[inline] fn as_raw_mut_TrackerMedianFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::tracking::TrackerMedianFlow> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::tracking::TrackerMedianFlow> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerMedianFlow>, core::Ptr<core::Algorithm>, cv_PtrLcv_TrackerMedianFlowG_to_PtrOfAlgorithm }

impl crate::tracking::TrackerTraitConst for core::Ptr<crate::tracking::TrackerMedianFlow> {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerTrait for core::Ptr<crate::tracking::TrackerMedianFlow> {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::tracking::TrackerMedianFlow>, core::Ptr<crate::tracking::Tracker>, cv_PtrLcv_TrackerMedianFlowG_to_PtrOfTracker }

impl std::fmt::Debug for core::Ptr<crate::tracking::TrackerMedianFlow> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfTrackerMedianFlow")
			.finish()
	}
}

