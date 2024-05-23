ptr_extern! { crate::xphoto::LearningBasedWB,
	cv_PtrLcv_xphoto_LearningBasedWBG_new_null_const, cv_PtrLcv_xphoto_LearningBasedWBG_delete, cv_PtrLcv_xphoto_LearningBasedWBG_getInnerPtr_const, cv_PtrLcv_xphoto_LearningBasedWBG_getInnerPtrMut
}

impl core::Ptr<crate::xphoto::LearningBasedWB> {
	#[inline] pub fn as_raw_PtrOfLearningBasedWB(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLearningBasedWB(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xphoto::LearningBasedWBTraitConst for core::Ptr<crate::xphoto::LearningBasedWB> {
	#[inline] fn as_raw_LearningBasedWB(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xphoto::LearningBasedWBTrait for core::Ptr<crate::xphoto::LearningBasedWB> {
	#[inline] fn as_raw_mut_LearningBasedWB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::xphoto::LearningBasedWB> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::xphoto::LearningBasedWB> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xphoto::LearningBasedWB>, core::Ptr<core::Algorithm>, cv_PtrLcv_xphoto_LearningBasedWBG_to_PtrOfAlgorithm }

impl crate::xphoto::WhiteBalancerTraitConst for core::Ptr<crate::xphoto::LearningBasedWB> {
	#[inline] fn as_raw_WhiteBalancer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xphoto::WhiteBalancerTrait for core::Ptr<crate::xphoto::LearningBasedWB> {
	#[inline] fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xphoto::LearningBasedWB>, core::Ptr<crate::xphoto::WhiteBalancer>, cv_PtrLcv_xphoto_LearningBasedWBG_to_PtrOfWhiteBalancer }

impl std::fmt::Debug for core::Ptr<crate::xphoto::LearningBasedWB> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfLearningBasedWB")
			.finish()
	}
}

