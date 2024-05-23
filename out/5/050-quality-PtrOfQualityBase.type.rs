ptr_extern! { crate::quality::QualityBase,
	cv_PtrLcv_quality_QualityBaseG_new_null_const, cv_PtrLcv_quality_QualityBaseG_delete, cv_PtrLcv_quality_QualityBaseG_getInnerPtr_const, cv_PtrLcv_quality_QualityBaseG_getInnerPtrMut
}

impl core::Ptr<crate::quality::QualityBase> {
	#[inline] pub fn as_raw_PtrOfQualityBase(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfQualityBase(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::quality::QualityBaseTraitConst for core::Ptr<crate::quality::QualityBase> {
	#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::quality::QualityBaseTrait for core::Ptr<crate::quality::QualityBase> {
	#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::quality::QualityBase> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::quality::QualityBase> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::quality::QualityBase>, core::Ptr<core::Algorithm>, cv_PtrLcv_quality_QualityBaseG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::quality::QualityBase> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfQualityBase")
			.finish()
	}
}

