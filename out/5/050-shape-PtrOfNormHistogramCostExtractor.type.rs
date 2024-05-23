ptr_extern! { crate::shape::NormHistogramCostExtractor,
	cv_PtrLcv_NormHistogramCostExtractorG_new_null_const, cv_PtrLcv_NormHistogramCostExtractorG_delete, cv_PtrLcv_NormHistogramCostExtractorG_getInnerPtr_const, cv_PtrLcv_NormHistogramCostExtractorG_getInnerPtrMut
}

impl core::Ptr<crate::shape::NormHistogramCostExtractor> {
	#[inline] pub fn as_raw_PtrOfNormHistogramCostExtractor(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfNormHistogramCostExtractor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::shape::NormHistogramCostExtractorTraitConst for core::Ptr<crate::shape::NormHistogramCostExtractor> {
	#[inline] fn as_raw_NormHistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::NormHistogramCostExtractorTrait for core::Ptr<crate::shape::NormHistogramCostExtractor> {
	#[inline] fn as_raw_mut_NormHistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::shape::NormHistogramCostExtractor> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::shape::NormHistogramCostExtractor> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::shape::NormHistogramCostExtractor>, core::Ptr<core::Algorithm>, cv_PtrLcv_NormHistogramCostExtractorG_to_PtrOfAlgorithm }

impl crate::shape::HistogramCostExtractorTraitConst for core::Ptr<crate::shape::NormHistogramCostExtractor> {
	#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::HistogramCostExtractorTrait for core::Ptr<crate::shape::NormHistogramCostExtractor> {
	#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::shape::NormHistogramCostExtractor>, core::Ptr<crate::shape::HistogramCostExtractor>, cv_PtrLcv_NormHistogramCostExtractorG_to_PtrOfHistogramCostExtractor }

impl std::fmt::Debug for core::Ptr<crate::shape::NormHistogramCostExtractor> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfNormHistogramCostExtractor")
			.finish()
	}
}

