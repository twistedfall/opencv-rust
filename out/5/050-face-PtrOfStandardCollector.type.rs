ptr_extern! { crate::face::StandardCollector,
	cv_PtrLcv_face_StandardCollectorG_new_null_const, cv_PtrLcv_face_StandardCollectorG_delete, cv_PtrLcv_face_StandardCollectorG_getInnerPtr_const, cv_PtrLcv_face_StandardCollectorG_getInnerPtrMut
}

ptr_extern_ctor! { crate::face::StandardCollector, cv_PtrLcv_face_StandardCollectorG_new_const_StandardCollector }
impl core::Ptr<crate::face::StandardCollector> {
	#[inline] pub fn as_raw_PtrOfStandardCollector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfStandardCollector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::face::StandardCollectorTraitConst for core::Ptr<crate::face::StandardCollector> {
	#[inline] fn as_raw_StandardCollector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::StandardCollectorTrait for core::Ptr<crate::face::StandardCollector> {
	#[inline] fn as_raw_mut_StandardCollector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::face::PredictCollectorTraitConst for core::Ptr<crate::face::StandardCollector> {
	#[inline] fn as_raw_PredictCollector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::PredictCollectorTrait for core::Ptr<crate::face::StandardCollector> {
	#[inline] fn as_raw_mut_PredictCollector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::face::StandardCollector>, core::Ptr<crate::face::PredictCollector>, cv_PtrLcv_face_StandardCollectorG_to_PtrOfPredictCollector }

impl std::fmt::Debug for core::Ptr<crate::face::StandardCollector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfStandardCollector")
			.finish()
	}
}

