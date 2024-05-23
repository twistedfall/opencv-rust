ptr_extern! { crate::quality::QualityMSE,
	cv_PtrLcv_quality_QualityMSEG_new_null_const, cv_PtrLcv_quality_QualityMSEG_delete, cv_PtrLcv_quality_QualityMSEG_getInnerPtr_const, cv_PtrLcv_quality_QualityMSEG_getInnerPtrMut
}

ptr_extern_ctor! { crate::quality::QualityMSE, cv_PtrLcv_quality_QualityMSEG_new_const_QualityMSE }
impl core::Ptr<crate::quality::QualityMSE> {
	#[inline] pub fn as_raw_PtrOfQualityMSE(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfQualityMSE(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::quality::QualityMSETraitConst for core::Ptr<crate::quality::QualityMSE> {
	#[inline] fn as_raw_QualityMSE(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::quality::QualityMSETrait for core::Ptr<crate::quality::QualityMSE> {
	#[inline] fn as_raw_mut_QualityMSE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::quality::QualityMSE> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::quality::QualityMSE> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::quality::QualityMSE>, core::Ptr<core::Algorithm>, cv_PtrLcv_quality_QualityMSEG_to_PtrOfAlgorithm }

impl crate::quality::QualityBaseTraitConst for core::Ptr<crate::quality::QualityMSE> {
	#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::quality::QualityBaseTrait for core::Ptr<crate::quality::QualityMSE> {
	#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::quality::QualityMSE>, core::Ptr<crate::quality::QualityBase>, cv_PtrLcv_quality_QualityMSEG_to_PtrOfQualityBase }

impl std::fmt::Debug for core::Ptr<crate::quality::QualityMSE> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfQualityMSE")
			.finish()
	}
}

