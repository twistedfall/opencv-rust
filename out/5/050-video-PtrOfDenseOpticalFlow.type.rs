ptr_extern! { crate::video::DenseOpticalFlow,
	cv_PtrLcv_DenseOpticalFlowG_new_null_const, cv_PtrLcv_DenseOpticalFlowG_delete, cv_PtrLcv_DenseOpticalFlowG_getInnerPtr_const, cv_PtrLcv_DenseOpticalFlowG_getInnerPtrMut
}

impl core::Ptr<crate::video::DenseOpticalFlow> {
	#[inline] pub fn as_raw_PtrOfDenseOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDenseOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::video::DenseOpticalFlowTraitConst for core::Ptr<crate::video::DenseOpticalFlow> {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::DenseOpticalFlowTrait for core::Ptr<crate::video::DenseOpticalFlow> {
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::video::DenseOpticalFlow> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::video::DenseOpticalFlow> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::video::DenseOpticalFlow>, core::Ptr<core::Algorithm>, cv_PtrLcv_DenseOpticalFlowG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::video::DenseOpticalFlow> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDenseOpticalFlow")
			.finish()
	}
}

