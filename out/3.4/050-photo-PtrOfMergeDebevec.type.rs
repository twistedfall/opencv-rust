ptr_extern! { crate::photo::MergeDebevec,
	cv_PtrLcv_MergeDebevecG_new_null_const, cv_PtrLcv_MergeDebevecG_delete, cv_PtrLcv_MergeDebevecG_getInnerPtr_const, cv_PtrLcv_MergeDebevecG_getInnerPtrMut
}

impl core::Ptr<crate::photo::MergeDebevec> {
	#[inline] pub fn as_raw_PtrOfMergeDebevec(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMergeDebevec(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::photo::MergeDebevecTraitConst for core::Ptr<crate::photo::MergeDebevec> {
	#[inline] fn as_raw_MergeDebevec(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::MergeDebevecTrait for core::Ptr<crate::photo::MergeDebevec> {
	#[inline] fn as_raw_mut_MergeDebevec(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::photo::MergeDebevec> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::photo::MergeDebevec> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::photo::MergeDebevec>, core::Ptr<core::Algorithm>, cv_PtrLcv_MergeDebevecG_to_PtrOfAlgorithm }

impl crate::photo::MergeExposuresTraitConst for core::Ptr<crate::photo::MergeDebevec> {
	#[inline] fn as_raw_MergeExposures(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::MergeExposuresTrait for core::Ptr<crate::photo::MergeDebevec> {
	#[inline] fn as_raw_mut_MergeExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::photo::MergeDebevec>, core::Ptr<crate::photo::MergeExposures>, cv_PtrLcv_MergeDebevecG_to_PtrOfMergeExposures }

impl std::fmt::Debug for core::Ptr<crate::photo::MergeDebevec> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMergeDebevec")
			.finish()
	}
}

