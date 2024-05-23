ptr_extern! { crate::ml::NormalBayesClassifier,
	cv_PtrLcv_ml_NormalBayesClassifierG_new_null_const, cv_PtrLcv_ml_NormalBayesClassifierG_delete, cv_PtrLcv_ml_NormalBayesClassifierG_getInnerPtr_const, cv_PtrLcv_ml_NormalBayesClassifierG_getInnerPtrMut
}

impl core::Ptr<crate::ml::NormalBayesClassifier> {
	#[inline] pub fn as_raw_PtrOfNormalBayesClassifier(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfNormalBayesClassifier(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ml::NormalBayesClassifierTraitConst for core::Ptr<crate::ml::NormalBayesClassifier> {
	#[inline] fn as_raw_NormalBayesClassifier(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::NormalBayesClassifierTrait for core::Ptr<crate::ml::NormalBayesClassifier> {
	#[inline] fn as_raw_mut_NormalBayesClassifier(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ml::NormalBayesClassifier> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ml::NormalBayesClassifier> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ml::NormalBayesClassifier>, core::Ptr<core::Algorithm>, cv_PtrLcv_ml_NormalBayesClassifierG_to_PtrOfAlgorithm }

impl crate::ml::StatModelTraitConst for core::Ptr<crate::ml::NormalBayesClassifier> {
	#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::StatModelTrait for core::Ptr<crate::ml::NormalBayesClassifier> {
	#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ml::NormalBayesClassifier>, core::Ptr<crate::ml::StatModel>, cv_PtrLcv_ml_NormalBayesClassifierG_to_PtrOfStatModel }

impl std::fmt::Debug for core::Ptr<crate::ml::NormalBayesClassifier> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfNormalBayesClassifier")
			.finish()
	}
}

