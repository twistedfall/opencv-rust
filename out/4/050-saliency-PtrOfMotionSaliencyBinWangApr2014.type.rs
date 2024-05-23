ptr_extern! { crate::saliency::MotionSaliencyBinWangApr2014,
	cv_PtrLcv_saliency_MotionSaliencyBinWangApr2014G_new_null_const, cv_PtrLcv_saliency_MotionSaliencyBinWangApr2014G_delete, cv_PtrLcv_saliency_MotionSaliencyBinWangApr2014G_getInnerPtr_const, cv_PtrLcv_saliency_MotionSaliencyBinWangApr2014G_getInnerPtrMut
}

ptr_extern_ctor! { crate::saliency::MotionSaliencyBinWangApr2014, cv_PtrLcv_saliency_MotionSaliencyBinWangApr2014G_new_const_MotionSaliencyBinWangApr2014 }
impl core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014> {
	#[inline] pub fn as_raw_PtrOfMotionSaliencyBinWangApr2014(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMotionSaliencyBinWangApr2014(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::saliency::MotionSaliencyBinWangApr2014TraitConst for core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014> {
	#[inline] fn as_raw_MotionSaliencyBinWangApr2014(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::MotionSaliencyBinWangApr2014Trait for core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014> {
	#[inline] fn as_raw_mut_MotionSaliencyBinWangApr2014(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014>, core::Ptr<core::Algorithm>, cv_PtrLcv_saliency_MotionSaliencyBinWangApr2014G_to_PtrOfAlgorithm }

impl crate::saliency::MotionSaliencyTraitConst for core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014> {
	#[inline] fn as_raw_MotionSaliency(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::MotionSaliencyTrait for core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014> {
	#[inline] fn as_raw_mut_MotionSaliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014>, core::Ptr<crate::saliency::MotionSaliency>, cv_PtrLcv_saliency_MotionSaliencyBinWangApr2014G_to_PtrOfMotionSaliency }

impl crate::saliency::SaliencyTraitConst for core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014> {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::SaliencyTrait for core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014> {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014>, core::Ptr<crate::saliency::Saliency>, cv_PtrLcv_saliency_MotionSaliencyBinWangApr2014G_to_PtrOfSaliency }

impl std::fmt::Debug for core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMotionSaliencyBinWangApr2014")
			.finish()
	}
}

