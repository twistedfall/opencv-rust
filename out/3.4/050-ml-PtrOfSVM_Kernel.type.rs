ptr_extern! { crate::ml::SVM_Kernel,
	cv_PtrLcv_ml_SVM_KernelG_new_null_const, cv_PtrLcv_ml_SVM_KernelG_delete, cv_PtrLcv_ml_SVM_KernelG_getInnerPtr_const, cv_PtrLcv_ml_SVM_KernelG_getInnerPtrMut
}

impl core::Ptr<crate::ml::SVM_Kernel> {
	#[inline] pub fn as_raw_PtrOfSVM_Kernel(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSVM_Kernel(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::ml::SVM_KernelTraitConst for core::Ptr<crate::ml::SVM_Kernel> {
	#[inline] fn as_raw_SVM_Kernel(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::SVM_KernelTrait for core::Ptr<crate::ml::SVM_Kernel> {
	#[inline] fn as_raw_mut_SVM_Kernel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::ml::SVM_Kernel> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::ml::SVM_Kernel> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::ml::SVM_Kernel>, core::Ptr<core::Algorithm>, cv_PtrLcv_ml_SVM_KernelG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::ml::SVM_Kernel> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfSVM_Kernel")
			.finish()
	}
}

