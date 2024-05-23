ptr_extern! { crate::ml::SVM,
	cv_PtrLcv_ml_SVMG_new_null_const, cv_PtrLcv_ml_SVMG_delete, cv_PtrLcv_ml_SVMG_getInnerPtr_const, cv_PtrLcv_ml_SVMG_getInnerPtrMut
}

impl core::Ptr<crate::ml::SVM> {
	#[inline] pub fn as_raw_PtrOfSVM(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSVM(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ml::SVMTraitConst for core::Ptr<crate::ml::SVM> {
	#[inline] fn as_raw_SVM(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::SVMTrait for core::Ptr<crate::ml::SVM> {
	#[inline] fn as_raw_mut_SVM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ml::SVM> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ml::SVM> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ml::SVM>, core::Ptr<core::Algorithm>, cv_PtrLcv_ml_SVMG_to_PtrOfAlgorithm }

impl crate::ml::StatModelTraitConst for core::Ptr<crate::ml::SVM> {
	#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::StatModelTrait for core::Ptr<crate::ml::SVM> {
	#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ml::SVM>, core::Ptr<crate::ml::StatModel>, cv_PtrLcv_ml_SVMG_to_PtrOfStatModel }

impl std::fmt::Debug for core::Ptr<crate::ml::SVM> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSVM")
			.finish()
	}
}

