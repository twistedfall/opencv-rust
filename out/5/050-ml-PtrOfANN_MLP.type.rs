ptr_extern! { crate::ml::ANN_MLP,
	cv_PtrLcv_ml_ANN_MLPG_new_null_const, cv_PtrLcv_ml_ANN_MLPG_delete, cv_PtrLcv_ml_ANN_MLPG_getInnerPtr_const, cv_PtrLcv_ml_ANN_MLPG_getInnerPtrMut
}

impl core::Ptr<crate::ml::ANN_MLP> {
	#[inline] pub fn as_raw_PtrOfANN_MLP(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfANN_MLP(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ml::ANN_MLPTraitConst for core::Ptr<crate::ml::ANN_MLP> {
	#[inline] fn as_raw_ANN_MLP(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::ANN_MLPTrait for core::Ptr<crate::ml::ANN_MLP> {
	#[inline] fn as_raw_mut_ANN_MLP(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ml::ANN_MLP> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ml::ANN_MLP> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ml::ANN_MLP>, core::Ptr<core::Algorithm>, cv_PtrLcv_ml_ANN_MLPG_to_PtrOfAlgorithm }

impl crate::ml::StatModelTraitConst for core::Ptr<crate::ml::ANN_MLP> {
	#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::StatModelTrait for core::Ptr<crate::ml::ANN_MLP> {
	#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ml::ANN_MLP>, core::Ptr<crate::ml::StatModel>, cv_PtrLcv_ml_ANN_MLPG_to_PtrOfStatModel }

impl std::fmt::Debug for core::Ptr<crate::ml::ANN_MLP> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfANN_MLP")
			.finish()
	}
}

