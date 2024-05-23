ptr_extern! { crate::xfeatures2d::BriefDescriptorExtractor,
	cv_PtrLcv_xfeatures2d_BriefDescriptorExtractorG_new_null_const, cv_PtrLcv_xfeatures2d_BriefDescriptorExtractorG_delete, cv_PtrLcv_xfeatures2d_BriefDescriptorExtractorG_getInnerPtr_const, cv_PtrLcv_xfeatures2d_BriefDescriptorExtractorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::xfeatures2d::BriefDescriptorExtractor, cv_PtrLcv_xfeatures2d_BriefDescriptorExtractorG_new_const_BriefDescriptorExtractor }
impl core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor> {
	#[inline] pub fn as_raw_PtrOfBriefDescriptorExtractor(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBriefDescriptorExtractor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xfeatures2d::BriefDescriptorExtractorTraitConst for core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor> {
	#[inline] fn as_raw_BriefDescriptorExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::BriefDescriptorExtractorTrait for core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor> {
	#[inline] fn as_raw_mut_BriefDescriptorExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor>, core::Ptr<core::Algorithm>, cv_PtrLcv_xfeatures2d_BriefDescriptorExtractorG_to_PtrOfAlgorithm }

impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor>, core::Ptr<crate::features2d::Feature2D>, cv_PtrLcv_xfeatures2d_BriefDescriptorExtractorG_to_PtrOfFeature2D }

impl std::fmt::Debug for core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBriefDescriptorExtractor")
			.finish()
	}
}

