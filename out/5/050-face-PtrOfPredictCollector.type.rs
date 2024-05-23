ptr_extern! { crate::face::PredictCollector,
	cv_PtrLcv_face_PredictCollectorG_new_null_const, cv_PtrLcv_face_PredictCollectorG_delete, cv_PtrLcv_face_PredictCollectorG_getInnerPtr_const, cv_PtrLcv_face_PredictCollectorG_getInnerPtrMut
}

impl core::Ptr<crate::face::PredictCollector> {
	#[inline] pub fn as_raw_PtrOfPredictCollector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPredictCollector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::face::PredictCollectorTraitConst for core::Ptr<crate::face::PredictCollector> {
	#[inline] fn as_raw_PredictCollector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::PredictCollectorTrait for core::Ptr<crate::face::PredictCollector> {
	#[inline] fn as_raw_mut_PredictCollector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::face::PredictCollector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfPredictCollector")
			.finish()
	}
}

