ptr_extern! { crate::shape::ShapeContextDistanceExtractor,
	cv_PtrLcv_ShapeContextDistanceExtractorG_new_null_const, cv_PtrLcv_ShapeContextDistanceExtractorG_delete, cv_PtrLcv_ShapeContextDistanceExtractorG_getInnerPtr_const, cv_PtrLcv_ShapeContextDistanceExtractorG_getInnerPtrMut
}

impl core::Ptr<crate::shape::ShapeContextDistanceExtractor> {
	#[inline] pub fn as_raw_PtrOfShapeContextDistanceExtractor(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfShapeContextDistanceExtractor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::shape::ShapeContextDistanceExtractorTraitConst for core::Ptr<crate::shape::ShapeContextDistanceExtractor> {
	#[inline] fn as_raw_ShapeContextDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::ShapeContextDistanceExtractorTrait for core::Ptr<crate::shape::ShapeContextDistanceExtractor> {
	#[inline] fn as_raw_mut_ShapeContextDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::shape::ShapeContextDistanceExtractor> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::shape::ShapeContextDistanceExtractor> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::shape::ShapeContextDistanceExtractor>, core::Ptr<core::Algorithm>, cv_PtrLcv_ShapeContextDistanceExtractorG_to_PtrOfAlgorithm }

impl crate::shape::ShapeDistanceExtractorTraitConst for core::Ptr<crate::shape::ShapeContextDistanceExtractor> {
	#[inline] fn as_raw_ShapeDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::ShapeDistanceExtractorTrait for core::Ptr<crate::shape::ShapeContextDistanceExtractor> {
	#[inline] fn as_raw_mut_ShapeDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::shape::ShapeContextDistanceExtractor>, core::Ptr<crate::shape::ShapeDistanceExtractor>, cv_PtrLcv_ShapeContextDistanceExtractorG_to_PtrOfShapeDistanceExtractor }

impl std::fmt::Debug for core::Ptr<crate::shape::ShapeContextDistanceExtractor> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfShapeContextDistanceExtractor")
			.finish()
	}
}

