ptr_extern! { crate::ml::ANN_MLP_ANNEAL,
	cv_PtrLcv_ml_ANN_MLP_ANNEALG_new_null_const, cv_PtrLcv_ml_ANN_MLP_ANNEALG_delete, cv_PtrLcv_ml_ANN_MLP_ANNEALG_getInnerPtr_const, cv_PtrLcv_ml_ANN_MLP_ANNEALG_getInnerPtrMut
}

impl core::Ptr<crate::ml::ANN_MLP_ANNEAL> {
	#[inline] pub fn as_raw_PtrOfANN_MLP_ANNEAL(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfANN_MLP_ANNEAL(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ml::ANN_MLP_ANNEALTraitConst for core::Ptr<crate::ml::ANN_MLP_ANNEAL> {
	#[inline] fn as_raw_ANN_MLP_ANNEAL(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::ANN_MLP_ANNEALTrait for core::Ptr<crate::ml::ANN_MLP_ANNEAL> {
	#[inline] fn as_raw_mut_ANN_MLP_ANNEAL(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ml::ANN_MLP_ANNEAL> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ml::ANN_MLP_ANNEAL> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ml::ANN_MLP_ANNEAL>, core::Ptr<core::Algorithm>, cv_PtrLcv_ml_ANN_MLP_ANNEALG_to_PtrOfAlgorithm }

impl crate::ml::ANN_MLPTraitConst for core::Ptr<crate::ml::ANN_MLP_ANNEAL> {
	#[inline] fn as_raw_ANN_MLP(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::ANN_MLPTrait for core::Ptr<crate::ml::ANN_MLP_ANNEAL> {
	#[inline] fn as_raw_mut_ANN_MLP(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ml::ANN_MLP_ANNEAL>, core::Ptr<crate::ml::ANN_MLP>, cv_PtrLcv_ml_ANN_MLP_ANNEALG_to_PtrOfANN_MLP }

impl crate::ml::StatModelTraitConst for core::Ptr<crate::ml::ANN_MLP_ANNEAL> {
	#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::StatModelTrait for core::Ptr<crate::ml::ANN_MLP_ANNEAL> {
	#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ml::ANN_MLP_ANNEAL>, core::Ptr<crate::ml::StatModel>, cv_PtrLcv_ml_ANN_MLP_ANNEALG_to_PtrOfStatModel }

impl std::fmt::Debug for core::Ptr<crate::ml::ANN_MLP_ANNEAL> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfANN_MLP_ANNEAL")
			.finish()
	}
}

