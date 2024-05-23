ptr_extern! { crate::ml::LogisticRegression,
	cv_PtrLcv_ml_LogisticRegressionG_new_null_const, cv_PtrLcv_ml_LogisticRegressionG_delete, cv_PtrLcv_ml_LogisticRegressionG_getInnerPtr_const, cv_PtrLcv_ml_LogisticRegressionG_getInnerPtrMut
}

impl core::Ptr<crate::ml::LogisticRegression> {
	#[inline] pub fn as_raw_PtrOfLogisticRegression(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLogisticRegression(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ml::LogisticRegressionTraitConst for core::Ptr<crate::ml::LogisticRegression> {
	#[inline] fn as_raw_LogisticRegression(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::LogisticRegressionTrait for core::Ptr<crate::ml::LogisticRegression> {
	#[inline] fn as_raw_mut_LogisticRegression(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ml::LogisticRegression> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ml::LogisticRegression> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ml::LogisticRegression>, core::Ptr<core::Algorithm>, cv_PtrLcv_ml_LogisticRegressionG_to_PtrOfAlgorithm }

impl crate::ml::StatModelTraitConst for core::Ptr<crate::ml::LogisticRegression> {
	#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::StatModelTrait for core::Ptr<crate::ml::LogisticRegression> {
	#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ml::LogisticRegression>, core::Ptr<crate::ml::StatModel>, cv_PtrLcv_ml_LogisticRegressionG_to_PtrOfStatModel }

impl std::fmt::Debug for core::Ptr<crate::ml::LogisticRegression> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfLogisticRegression")
			.finish()
	}
}

