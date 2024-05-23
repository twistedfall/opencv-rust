ptr_extern! { crate::shape::HistogramCostExtractor,
	cv_PtrLcv_HistogramCostExtractorG_new_null_const, cv_PtrLcv_HistogramCostExtractorG_delete, cv_PtrLcv_HistogramCostExtractorG_getInnerPtr_const, cv_PtrLcv_HistogramCostExtractorG_getInnerPtrMut
}

impl core::Ptr<crate::shape::HistogramCostExtractor> {
	#[inline] pub fn as_raw_PtrOfHistogramCostExtractor(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfHistogramCostExtractor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::shape::HistogramCostExtractorTraitConst for core::Ptr<crate::shape::HistogramCostExtractor> {
	#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::HistogramCostExtractorTrait for core::Ptr<crate::shape::HistogramCostExtractor> {
	#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::shape::HistogramCostExtractor> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::shape::HistogramCostExtractor> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::shape::HistogramCostExtractor>, core::Ptr<core::Algorithm>, cv_PtrLcv_HistogramCostExtractorG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::shape::HistogramCostExtractor> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfHistogramCostExtractor")
			.finish()
	}
}

