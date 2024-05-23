ptr_extern! { crate::shape::ChiHistogramCostExtractor,
	cv_PtrLcv_ChiHistogramCostExtractorG_new_null_const, cv_PtrLcv_ChiHistogramCostExtractorG_delete, cv_PtrLcv_ChiHistogramCostExtractorG_getInnerPtr_const, cv_PtrLcv_ChiHistogramCostExtractorG_getInnerPtrMut
}

impl core::Ptr<crate::shape::ChiHistogramCostExtractor> {
	#[inline] pub fn as_raw_PtrOfChiHistogramCostExtractor(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfChiHistogramCostExtractor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::shape::ChiHistogramCostExtractorTraitConst for core::Ptr<crate::shape::ChiHistogramCostExtractor> {
	#[inline] fn as_raw_ChiHistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::ChiHistogramCostExtractorTrait for core::Ptr<crate::shape::ChiHistogramCostExtractor> {
	#[inline] fn as_raw_mut_ChiHistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::shape::ChiHistogramCostExtractor> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::shape::ChiHistogramCostExtractor> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::shape::ChiHistogramCostExtractor>, core::Ptr<core::Algorithm>, cv_PtrLcv_ChiHistogramCostExtractorG_to_PtrOfAlgorithm }

impl crate::shape::HistogramCostExtractorTraitConst for core::Ptr<crate::shape::ChiHistogramCostExtractor> {
	#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::HistogramCostExtractorTrait for core::Ptr<crate::shape::ChiHistogramCostExtractor> {
	#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::shape::ChiHistogramCostExtractor>, core::Ptr<crate::shape::HistogramCostExtractor>, cv_PtrLcv_ChiHistogramCostExtractorG_to_PtrOfHistogramCostExtractor }

impl std::fmt::Debug for core::Ptr<crate::shape::ChiHistogramCostExtractor> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfChiHistogramCostExtractor")
			.finish()
	}
}

