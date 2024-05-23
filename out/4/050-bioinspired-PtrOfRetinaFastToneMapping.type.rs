ptr_extern! { crate::bioinspired::RetinaFastToneMapping,
	cv_PtrLcv_bioinspired_RetinaFastToneMappingG_new_null_const, cv_PtrLcv_bioinspired_RetinaFastToneMappingG_delete, cv_PtrLcv_bioinspired_RetinaFastToneMappingG_getInnerPtr_const, cv_PtrLcv_bioinspired_RetinaFastToneMappingG_getInnerPtrMut
}

impl core::Ptr<crate::bioinspired::RetinaFastToneMapping> {
	#[inline] pub fn as_raw_PtrOfRetinaFastToneMapping(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRetinaFastToneMapping(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::bioinspired::RetinaFastToneMappingTraitConst for core::Ptr<crate::bioinspired::RetinaFastToneMapping> {
	#[inline] fn as_raw_RetinaFastToneMapping(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::bioinspired::RetinaFastToneMappingTrait for core::Ptr<crate::bioinspired::RetinaFastToneMapping> {
	#[inline] fn as_raw_mut_RetinaFastToneMapping(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::bioinspired::RetinaFastToneMapping> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::bioinspired::RetinaFastToneMapping> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::bioinspired::RetinaFastToneMapping>, core::Ptr<core::Algorithm>, cv_PtrLcv_bioinspired_RetinaFastToneMappingG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::bioinspired::RetinaFastToneMapping> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfRetinaFastToneMapping")
			.finish()
	}
}

