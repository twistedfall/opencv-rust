ptr_extern! { crate::shape::ShapeDistanceExtractor,
	cv_PtrLcv_ShapeDistanceExtractorG_new_null_const, cv_PtrLcv_ShapeDistanceExtractorG_delete, cv_PtrLcv_ShapeDistanceExtractorG_getInnerPtr_const, cv_PtrLcv_ShapeDistanceExtractorG_getInnerPtrMut
}

impl core::Ptr<crate::shape::ShapeDistanceExtractor> {
	#[inline] pub fn as_raw_PtrOfShapeDistanceExtractor(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfShapeDistanceExtractor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::shape::ShapeDistanceExtractorTraitConst for core::Ptr<crate::shape::ShapeDistanceExtractor> {
	#[inline] fn as_raw_ShapeDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::ShapeDistanceExtractorTrait for core::Ptr<crate::shape::ShapeDistanceExtractor> {
	#[inline] fn as_raw_mut_ShapeDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::shape::ShapeDistanceExtractor> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::shape::ShapeDistanceExtractor> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::shape::ShapeDistanceExtractor>, core::Ptr<core::Algorithm>, cv_PtrLcv_ShapeDistanceExtractorG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::shape::ShapeDistanceExtractor> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfShapeDistanceExtractor")
			.finish()
	}
}

