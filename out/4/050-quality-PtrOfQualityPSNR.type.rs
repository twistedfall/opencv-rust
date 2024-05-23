ptr_extern! { crate::quality::QualityPSNR,
	cv_PtrLcv_quality_QualityPSNRG_new_null_const, cv_PtrLcv_quality_QualityPSNRG_delete, cv_PtrLcv_quality_QualityPSNRG_getInnerPtr_const, cv_PtrLcv_quality_QualityPSNRG_getInnerPtrMut
}

ptr_extern_ctor! { crate::quality::QualityPSNR, cv_PtrLcv_quality_QualityPSNRG_new_const_QualityPSNR }
impl core::Ptr<crate::quality::QualityPSNR> {
	#[inline] pub fn as_raw_PtrOfQualityPSNR(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfQualityPSNR(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::quality::QualityPSNRTraitConst for core::Ptr<crate::quality::QualityPSNR> {
	#[inline] fn as_raw_QualityPSNR(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::quality::QualityPSNRTrait for core::Ptr<crate::quality::QualityPSNR> {
	#[inline] fn as_raw_mut_QualityPSNR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::quality::QualityPSNR> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::quality::QualityPSNR> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::quality::QualityPSNR>, core::Ptr<core::Algorithm>, cv_PtrLcv_quality_QualityPSNRG_to_PtrOfAlgorithm }

impl crate::quality::QualityBaseTraitConst for core::Ptr<crate::quality::QualityPSNR> {
	#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::quality::QualityBaseTrait for core::Ptr<crate::quality::QualityPSNR> {
	#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::quality::QualityPSNR>, core::Ptr<crate::quality::QualityBase>, cv_PtrLcv_quality_QualityPSNRG_to_PtrOfQualityBase }

impl std::fmt::Debug for core::Ptr<crate::quality::QualityPSNR> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfQualityPSNR")
			.finish()
	}
}

