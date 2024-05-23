ptr_extern! { crate::rapid::Rapid_Rapid,
	cv_PtrLcv_rapid_RapidG_new_null_const, cv_PtrLcv_rapid_RapidG_delete, cv_PtrLcv_rapid_RapidG_getInnerPtr_const, cv_PtrLcv_rapid_RapidG_getInnerPtrMut
}

impl core::Ptr<crate::rapid::Rapid_Rapid> {
	#[inline] pub fn as_raw_PtrOfRapid_Rapid(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRapid_Rapid(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::rapid::Rapid_RapidTraitConst for core::Ptr<crate::rapid::Rapid_Rapid> {
	#[inline] fn as_raw_Rapid_Rapid(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rapid::Rapid_RapidTrait for core::Ptr<crate::rapid::Rapid_Rapid> {
	#[inline] fn as_raw_mut_Rapid_Rapid(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::rapid::Rapid_Rapid> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::rapid::Rapid_Rapid> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::rapid::Rapid_Rapid>, core::Ptr<core::Algorithm>, cv_PtrLcv_rapid_RapidG_to_PtrOfAlgorithm }

impl crate::rapid::Rapid_TrackerTraitConst for core::Ptr<crate::rapid::Rapid_Rapid> {
	#[inline] fn as_raw_Rapid_Tracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rapid::Rapid_TrackerTrait for core::Ptr<crate::rapid::Rapid_Rapid> {
	#[inline] fn as_raw_mut_Rapid_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::rapid::Rapid_Rapid>, core::Ptr<crate::rapid::Rapid_Tracker>, cv_PtrLcv_rapid_RapidG_to_PtrOfRapid_Tracker }

impl std::fmt::Debug for core::Ptr<crate::rapid::Rapid_Rapid> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfRapid_Rapid")
			.finish()
	}
}

