ptr_extern! { crate::quality::QualityBRISQUE,
	cv_PtrLcv_quality_QualityBRISQUEG_new_null_const, cv_PtrLcv_quality_QualityBRISQUEG_delete, cv_PtrLcv_quality_QualityBRISQUEG_getInnerPtr_const, cv_PtrLcv_quality_QualityBRISQUEG_getInnerPtrMut
}

ptr_extern_ctor! { crate::quality::QualityBRISQUE, cv_PtrLcv_quality_QualityBRISQUEG_new_const_QualityBRISQUE }
impl core::Ptr<crate::quality::QualityBRISQUE> {
	#[inline] pub fn as_raw_PtrOfQualityBRISQUE(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfQualityBRISQUE(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::quality::QualityBRISQUETraitConst for core::Ptr<crate::quality::QualityBRISQUE> {
	#[inline] fn as_raw_QualityBRISQUE(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::quality::QualityBRISQUETrait for core::Ptr<crate::quality::QualityBRISQUE> {
	#[inline] fn as_raw_mut_QualityBRISQUE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::quality::QualityBRISQUE> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::quality::QualityBRISQUE> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::quality::QualityBRISQUE>, core::Ptr<core::Algorithm>, cv_PtrLcv_quality_QualityBRISQUEG_to_PtrOfAlgorithm }

impl crate::quality::QualityBaseTraitConst for core::Ptr<crate::quality::QualityBRISQUE> {
	#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::quality::QualityBaseTrait for core::Ptr<crate::quality::QualityBRISQUE> {
	#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::quality::QualityBRISQUE>, core::Ptr<crate::quality::QualityBase>, cv_PtrLcv_quality_QualityBRISQUEG_to_PtrOfQualityBase }

impl std::fmt::Debug for core::Ptr<crate::quality::QualityBRISQUE> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfQualityBRISQUE")
			.finish()
	}
}

