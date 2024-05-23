ptr_extern! { crate::shape::EMDL1HistogramCostExtractor,
	cv_PtrLcv_EMDL1HistogramCostExtractorG_new_null_const, cv_PtrLcv_EMDL1HistogramCostExtractorG_delete, cv_PtrLcv_EMDL1HistogramCostExtractorG_getInnerPtr_const, cv_PtrLcv_EMDL1HistogramCostExtractorG_getInnerPtrMut
}

impl core::Ptr<crate::shape::EMDL1HistogramCostExtractor> {
	#[inline] pub fn as_raw_PtrOfEMDL1HistogramCostExtractor(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfEMDL1HistogramCostExtractor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::shape::EMDL1HistogramCostExtractorTraitConst for core::Ptr<crate::shape::EMDL1HistogramCostExtractor> {
	#[inline] fn as_raw_EMDL1HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::EMDL1HistogramCostExtractorTrait for core::Ptr<crate::shape::EMDL1HistogramCostExtractor> {
	#[inline] fn as_raw_mut_EMDL1HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::shape::EMDL1HistogramCostExtractor> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::shape::EMDL1HistogramCostExtractor> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::shape::EMDL1HistogramCostExtractor>, core::Ptr<core::Algorithm>, cv_PtrLcv_EMDL1HistogramCostExtractorG_to_PtrOfAlgorithm }

impl crate::shape::HistogramCostExtractorTraitConst for core::Ptr<crate::shape::EMDL1HistogramCostExtractor> {
	#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::HistogramCostExtractorTrait for core::Ptr<crate::shape::EMDL1HistogramCostExtractor> {
	#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::shape::EMDL1HistogramCostExtractor>, core::Ptr<crate::shape::HistogramCostExtractor>, cv_PtrLcv_EMDL1HistogramCostExtractorG_to_PtrOfHistogramCostExtractor }

impl std::fmt::Debug for core::Ptr<crate::shape::EMDL1HistogramCostExtractor> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfEMDL1HistogramCostExtractor")
			.finish()
	}
}

