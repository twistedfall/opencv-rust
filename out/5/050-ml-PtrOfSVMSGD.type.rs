ptr_extern! { crate::ml::SVMSGD,
	cv_PtrLcv_ml_SVMSGDG_new_null_const, cv_PtrLcv_ml_SVMSGDG_delete, cv_PtrLcv_ml_SVMSGDG_getInnerPtr_const, cv_PtrLcv_ml_SVMSGDG_getInnerPtrMut
}

impl core::Ptr<crate::ml::SVMSGD> {
	#[inline] pub fn as_raw_PtrOfSVMSGD(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSVMSGD(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ml::SVMSGDTraitConst for core::Ptr<crate::ml::SVMSGD> {
	#[inline] fn as_raw_SVMSGD(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::SVMSGDTrait for core::Ptr<crate::ml::SVMSGD> {
	#[inline] fn as_raw_mut_SVMSGD(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ml::SVMSGD> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ml::SVMSGD> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ml::SVMSGD>, core::Ptr<core::Algorithm>, cv_PtrLcv_ml_SVMSGDG_to_PtrOfAlgorithm }

impl crate::ml::StatModelTraitConst for core::Ptr<crate::ml::SVMSGD> {
	#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::StatModelTrait for core::Ptr<crate::ml::SVMSGD> {
	#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ml::SVMSGD>, core::Ptr<crate::ml::StatModel>, cv_PtrLcv_ml_SVMSGDG_to_PtrOfStatModel }

impl std::fmt::Debug for core::Ptr<crate::ml::SVMSGD> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSVMSGD")
			.finish()
	}
}

