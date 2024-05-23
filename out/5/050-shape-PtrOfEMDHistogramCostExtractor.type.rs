ptr_extern! { crate::shape::EMDHistogramCostExtractor,
	cv_PtrLcv_EMDHistogramCostExtractorG_new_null_const, cv_PtrLcv_EMDHistogramCostExtractorG_delete, cv_PtrLcv_EMDHistogramCostExtractorG_getInnerPtr_const, cv_PtrLcv_EMDHistogramCostExtractorG_getInnerPtrMut
}

impl core::Ptr<crate::shape::EMDHistogramCostExtractor> {
	#[inline] pub fn as_raw_PtrOfEMDHistogramCostExtractor(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfEMDHistogramCostExtractor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::shape::EMDHistogramCostExtractorTraitConst for core::Ptr<crate::shape::EMDHistogramCostExtractor> {
	#[inline] fn as_raw_EMDHistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::EMDHistogramCostExtractorTrait for core::Ptr<crate::shape::EMDHistogramCostExtractor> {
	#[inline] fn as_raw_mut_EMDHistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::shape::EMDHistogramCostExtractor> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::shape::EMDHistogramCostExtractor> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::shape::EMDHistogramCostExtractor>, core::Ptr<core::Algorithm>, cv_PtrLcv_EMDHistogramCostExtractorG_to_PtrOfAlgorithm }

impl crate::shape::HistogramCostExtractorTraitConst for core::Ptr<crate::shape::EMDHistogramCostExtractor> {
	#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::HistogramCostExtractorTrait for core::Ptr<crate::shape::EMDHistogramCostExtractor> {
	#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::shape::EMDHistogramCostExtractor>, core::Ptr<crate::shape::HistogramCostExtractor>, cv_PtrLcv_EMDHistogramCostExtractorG_to_PtrOfHistogramCostExtractor }

impl std::fmt::Debug for core::Ptr<crate::shape::EMDHistogramCostExtractor> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfEMDHistogramCostExtractor")
			.finish()
	}
}

