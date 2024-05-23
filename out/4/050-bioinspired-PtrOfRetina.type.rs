ptr_extern! { crate::bioinspired::Retina,
	cv_PtrLcv_bioinspired_RetinaG_new_null_const, cv_PtrLcv_bioinspired_RetinaG_delete, cv_PtrLcv_bioinspired_RetinaG_getInnerPtr_const, cv_PtrLcv_bioinspired_RetinaG_getInnerPtrMut
}

impl core::Ptr<crate::bioinspired::Retina> {
	#[inline] pub fn as_raw_PtrOfRetina(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRetina(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::bioinspired::RetinaTraitConst for core::Ptr<crate::bioinspired::Retina> {
	#[inline] fn as_raw_Retina(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::bioinspired::RetinaTrait for core::Ptr<crate::bioinspired::Retina> {
	#[inline] fn as_raw_mut_Retina(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::bioinspired::Retina> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::bioinspired::Retina> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::bioinspired::Retina>, core::Ptr<core::Algorithm>, cv_PtrLcv_bioinspired_RetinaG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::bioinspired::Retina> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfRetina")
			.finish()
	}
}

