ptr_extern! { crate::photo::MergeRobertson,
	cv_PtrLcv_MergeRobertsonG_new_null_const, cv_PtrLcv_MergeRobertsonG_delete, cv_PtrLcv_MergeRobertsonG_getInnerPtr_const, cv_PtrLcv_MergeRobertsonG_getInnerPtrMut
}

impl core::Ptr<crate::photo::MergeRobertson> {
	#[inline] pub fn as_raw_PtrOfMergeRobertson(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMergeRobertson(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::photo::MergeRobertsonTraitConst for core::Ptr<crate::photo::MergeRobertson> {
	#[inline] fn as_raw_MergeRobertson(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::MergeRobertsonTrait for core::Ptr<crate::photo::MergeRobertson> {
	#[inline] fn as_raw_mut_MergeRobertson(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::photo::MergeRobertson> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::photo::MergeRobertson> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::photo::MergeRobertson>, core::Ptr<core::Algorithm>, cv_PtrLcv_MergeRobertsonG_to_PtrOfAlgorithm }

impl crate::photo::MergeExposuresTraitConst for core::Ptr<crate::photo::MergeRobertson> {
	#[inline] fn as_raw_MergeExposures(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::MergeExposuresTrait for core::Ptr<crate::photo::MergeRobertson> {
	#[inline] fn as_raw_mut_MergeExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::photo::MergeRobertson>, core::Ptr<crate::photo::MergeExposures>, cv_PtrLcv_MergeRobertsonG_to_PtrOfMergeExposures }

impl std::fmt::Debug for core::Ptr<crate::photo::MergeRobertson> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMergeRobertson")
			.finish()
	}
}

