ptr_extern! { crate::shape::HausdorffDistanceExtractor,
	cv_PtrLcv_HausdorffDistanceExtractorG_new_null_const, cv_PtrLcv_HausdorffDistanceExtractorG_delete, cv_PtrLcv_HausdorffDistanceExtractorG_getInnerPtr_const, cv_PtrLcv_HausdorffDistanceExtractorG_getInnerPtrMut
}

impl core::Ptr<crate::shape::HausdorffDistanceExtractor> {
	#[inline] pub fn as_raw_PtrOfHausdorffDistanceExtractor(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfHausdorffDistanceExtractor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::shape::HausdorffDistanceExtractorTraitConst for core::Ptr<crate::shape::HausdorffDistanceExtractor> {
	#[inline] fn as_raw_HausdorffDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::HausdorffDistanceExtractorTrait for core::Ptr<crate::shape::HausdorffDistanceExtractor> {
	#[inline] fn as_raw_mut_HausdorffDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::shape::HausdorffDistanceExtractor> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::shape::HausdorffDistanceExtractor> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::shape::HausdorffDistanceExtractor>, core::Ptr<core::Algorithm>, cv_PtrLcv_HausdorffDistanceExtractorG_to_PtrOfAlgorithm }

impl crate::shape::ShapeDistanceExtractorTraitConst for core::Ptr<crate::shape::HausdorffDistanceExtractor> {
	#[inline] fn as_raw_ShapeDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::shape::ShapeDistanceExtractorTrait for core::Ptr<crate::shape::HausdorffDistanceExtractor> {
	#[inline] fn as_raw_mut_ShapeDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::shape::HausdorffDistanceExtractor>, core::Ptr<crate::shape::ShapeDistanceExtractor>, cv_PtrLcv_HausdorffDistanceExtractorG_to_PtrOfShapeDistanceExtractor }

impl std::fmt::Debug for core::Ptr<crate::shape::HausdorffDistanceExtractor> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfHausdorffDistanceExtractor")
			.finish()
	}
}

