ptr_extern! { crate::ml::EM,
	cv_PtrLcv_ml_EMG_new_null_const, cv_PtrLcv_ml_EMG_delete, cv_PtrLcv_ml_EMG_getInnerPtr_const, cv_PtrLcv_ml_EMG_getInnerPtrMut
}

impl core::Ptr<crate::ml::EM> {
	#[inline] pub fn as_raw_PtrOfEM(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfEM(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ml::EMTraitConst for core::Ptr<crate::ml::EM> {
	#[inline] fn as_raw_EM(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::EMTrait for core::Ptr<crate::ml::EM> {
	#[inline] fn as_raw_mut_EM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ml::EM> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ml::EM> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ml::EM>, core::Ptr<core::Algorithm>, cv_PtrLcv_ml_EMG_to_PtrOfAlgorithm }

impl crate::ml::StatModelTraitConst for core::Ptr<crate::ml::EM> {
	#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::StatModelTrait for core::Ptr<crate::ml::EM> {
	#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ml::EM>, core::Ptr<crate::ml::StatModel>, cv_PtrLcv_ml_EMG_to_PtrOfStatModel }

impl std::fmt::Debug for core::Ptr<crate::ml::EM> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfEM")
			.finish()
	}
}

