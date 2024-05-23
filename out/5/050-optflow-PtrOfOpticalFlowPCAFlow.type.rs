ptr_extern! { crate::optflow::OpticalFlowPCAFlow,
	cv_PtrLcv_optflow_OpticalFlowPCAFlowG_new_null_const, cv_PtrLcv_optflow_OpticalFlowPCAFlowG_delete, cv_PtrLcv_optflow_OpticalFlowPCAFlowG_getInnerPtr_const, cv_PtrLcv_optflow_OpticalFlowPCAFlowG_getInnerPtrMut
}

ptr_extern_ctor! { crate::optflow::OpticalFlowPCAFlow, cv_PtrLcv_optflow_OpticalFlowPCAFlowG_new_const_OpticalFlowPCAFlow }
impl core::Ptr<crate::optflow::OpticalFlowPCAFlow> {
	#[inline] pub fn as_raw_PtrOfOpticalFlowPCAFlow(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOpticalFlowPCAFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::optflow::OpticalFlowPCAFlowTraitConst for core::Ptr<crate::optflow::OpticalFlowPCAFlow> {
	#[inline] fn as_raw_OpticalFlowPCAFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::optflow::OpticalFlowPCAFlowTrait for core::Ptr<crate::optflow::OpticalFlowPCAFlow> {
	#[inline] fn as_raw_mut_OpticalFlowPCAFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::optflow::OpticalFlowPCAFlow> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::optflow::OpticalFlowPCAFlow> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::optflow::OpticalFlowPCAFlow>, core::Ptr<core::Algorithm>, cv_PtrLcv_optflow_OpticalFlowPCAFlowG_to_PtrOfAlgorithm }

impl crate::video::DenseOpticalFlowTraitConst for core::Ptr<crate::optflow::OpticalFlowPCAFlow> {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::DenseOpticalFlowTrait for core::Ptr<crate::optflow::OpticalFlowPCAFlow> {
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { core::Ptr<crate::optflow::OpticalFlowPCAFlow>, core::Ptr<crate::video::DenseOpticalFlow>, cv_PtrLcv_optflow_OpticalFlowPCAFlowG_to_PtrOfDenseOpticalFlow }

impl std::fmt::Debug for core::Ptr<crate::optflow::OpticalFlowPCAFlow> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfOpticalFlowPCAFlow")
			.finish()
	}
}

