ptr_extern! { crate::quality::QualityGMSD,
	cv_PtrLcv_quality_QualityGMSDG_new_null_const, cv_PtrLcv_quality_QualityGMSDG_delete, cv_PtrLcv_quality_QualityGMSDG_getInnerPtr_const, cv_PtrLcv_quality_QualityGMSDG_getInnerPtrMut
}

ptr_extern_ctor! { crate::quality::QualityGMSD, cv_PtrLcv_quality_QualityGMSDG_new_const_QualityGMSD }
impl core::Ptr<crate::quality::QualityGMSD> {
	#[inline] pub fn as_raw_PtrOfQualityGMSD(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfQualityGMSD(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::quality::QualityGMSDTraitConst for core::Ptr<crate::quality::QualityGMSD> {
	#[inline] fn as_raw_QualityGMSD(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::quality::QualityGMSDTrait for core::Ptr<crate::quality::QualityGMSD> {
	#[inline] fn as_raw_mut_QualityGMSD(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::quality::QualityGMSD> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::quality::QualityGMSD> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::quality::QualityGMSD>, core::Ptr<core::Algorithm>, cv_PtrLcv_quality_QualityGMSDG_to_PtrOfAlgorithm }

impl crate::quality::QualityBaseTraitConst for core::Ptr<crate::quality::QualityGMSD> {
	#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::quality::QualityBaseTrait for core::Ptr<crate::quality::QualityGMSD> {
	#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::quality::QualityGMSD>, core::Ptr<crate::quality::QualityBase>, cv_PtrLcv_quality_QualityGMSDG_to_PtrOfQualityBase }

impl std::fmt::Debug for core::Ptr<crate::quality::QualityGMSD> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfQualityGMSD")
			.finish()
	}
}

