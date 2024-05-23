ptr_extern! { crate::xphoto::GrayworldWB,
	cv_PtrLcv_xphoto_GrayworldWBG_new_null_const, cv_PtrLcv_xphoto_GrayworldWBG_delete, cv_PtrLcv_xphoto_GrayworldWBG_getInnerPtr_const, cv_PtrLcv_xphoto_GrayworldWBG_getInnerPtrMut
}

impl core::Ptr<crate::xphoto::GrayworldWB> {
	#[inline] pub fn as_raw_PtrOfGrayworldWB(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGrayworldWB(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xphoto::GrayworldWBTraitConst for core::Ptr<crate::xphoto::GrayworldWB> {
	#[inline] fn as_raw_GrayworldWB(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xphoto::GrayworldWBTrait for core::Ptr<crate::xphoto::GrayworldWB> {
	#[inline] fn as_raw_mut_GrayworldWB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::xphoto::GrayworldWB> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::xphoto::GrayworldWB> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xphoto::GrayworldWB>, core::Ptr<core::Algorithm>, cv_PtrLcv_xphoto_GrayworldWBG_to_PtrOfAlgorithm }

impl crate::xphoto::WhiteBalancerTraitConst for core::Ptr<crate::xphoto::GrayworldWB> {
	#[inline] fn as_raw_WhiteBalancer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xphoto::WhiteBalancerTrait for core::Ptr<crate::xphoto::GrayworldWB> {
	#[inline] fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::xphoto::GrayworldWB>, core::Ptr<crate::xphoto::WhiteBalancer>, cv_PtrLcv_xphoto_GrayworldWBG_to_PtrOfWhiteBalancer }

impl std::fmt::Debug for core::Ptr<crate::xphoto::GrayworldWB> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfGrayworldWB")
			.finish()
	}
}

