ptr_extern! { crate::video::FarnebackOpticalFlow,
	cv_PtrLcv_FarnebackOpticalFlowG_new_null_const, cv_PtrLcv_FarnebackOpticalFlowG_delete, cv_PtrLcv_FarnebackOpticalFlowG_getInnerPtr_const, cv_PtrLcv_FarnebackOpticalFlowG_getInnerPtrMut
}

impl core::Ptr<crate::video::FarnebackOpticalFlow> {
	#[inline] pub fn as_raw_PtrOfFarnebackOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFarnebackOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::video::FarnebackOpticalFlowTraitConst for core::Ptr<crate::video::FarnebackOpticalFlow> {
	#[inline] fn as_raw_FarnebackOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::FarnebackOpticalFlowTrait for core::Ptr<crate::video::FarnebackOpticalFlow> {
	#[inline] fn as_raw_mut_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::video::FarnebackOpticalFlow> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::video::FarnebackOpticalFlow> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::video::FarnebackOpticalFlow>, core::Ptr<core::Algorithm>, cv_PtrLcv_FarnebackOpticalFlowG_to_PtrOfAlgorithm }

impl crate::video::DenseOpticalFlowTraitConst for core::Ptr<crate::video::FarnebackOpticalFlow> {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::DenseOpticalFlowTrait for core::Ptr<crate::video::FarnebackOpticalFlow> {
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::video::FarnebackOpticalFlow>, core::Ptr<crate::video::DenseOpticalFlow>, cv_PtrLcv_FarnebackOpticalFlowG_to_PtrOfDenseOpticalFlow }

impl std::fmt::Debug for core::Ptr<crate::video::FarnebackOpticalFlow> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfFarnebackOpticalFlow")
			.finish()
	}
}

