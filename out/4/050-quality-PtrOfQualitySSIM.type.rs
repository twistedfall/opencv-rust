ptr_extern! { crate::quality::QualitySSIM,
	cv_PtrLcv_quality_QualitySSIMG_new_null_const, cv_PtrLcv_quality_QualitySSIMG_delete, cv_PtrLcv_quality_QualitySSIMG_getInnerPtr_const, cv_PtrLcv_quality_QualitySSIMG_getInnerPtrMut
}

ptr_extern_ctor! { crate::quality::QualitySSIM, cv_PtrLcv_quality_QualitySSIMG_new_const_QualitySSIM }
impl core::Ptr<crate::quality::QualitySSIM> {
	#[inline] pub fn as_raw_PtrOfQualitySSIM(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfQualitySSIM(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::quality::QualitySSIMTraitConst for core::Ptr<crate::quality::QualitySSIM> {
	#[inline] fn as_raw_QualitySSIM(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::quality::QualitySSIMTrait for core::Ptr<crate::quality::QualitySSIM> {
	#[inline] fn as_raw_mut_QualitySSIM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::quality::QualitySSIM> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::quality::QualitySSIM> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::quality::QualitySSIM>, core::Ptr<core::Algorithm>, cv_PtrLcv_quality_QualitySSIMG_to_PtrOfAlgorithm }

impl crate::quality::QualityBaseTraitConst for core::Ptr<crate::quality::QualitySSIM> {
	#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::quality::QualityBaseTrait for core::Ptr<crate::quality::QualitySSIM> {
	#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::quality::QualitySSIM>, core::Ptr<crate::quality::QualityBase>, cv_PtrLcv_quality_QualitySSIMG_to_PtrOfQualityBase }

impl std::fmt::Debug for core::Ptr<crate::quality::QualitySSIM> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfQualitySSIM")
			.finish()
	}
}

