pub mod cudaoptflow {
	//! # Optical Flow
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::CUDA_DenseOpticalFlowTraitConst, super::CUDA_DenseOpticalFlowTrait, super::CUDA_SparseOpticalFlowTraitConst, super::CUDA_SparseOpticalFlowTrait, super::CUDA_NvidiaHWOpticalFlowTraitConst, super::CUDA_NvidiaHWOpticalFlowTrait, super::CUDA_BroxOpticalFlowTraitConst, super::CUDA_BroxOpticalFlowTrait, super::CUDA_SparsePyrLKOpticalFlowTraitConst, super::CUDA_SparsePyrLKOpticalFlowTrait, super::CUDA_DensePyrLKOpticalFlowTraitConst, super::CUDA_DensePyrLKOpticalFlowTrait, super::CUDA_FarnebackOpticalFlowTraitConst, super::CUDA_FarnebackOpticalFlowTrait, super::CUDA_OpticalFlowDual_TVL1TraitConst, super::CUDA_OpticalFlowDual_TVL1Trait, super::CUDA_NvidiaOpticalFlow_1_0TraitConst, super::CUDA_NvidiaOpticalFlow_1_0Trait, super::CUDA_NvidiaOpticalFlow_2_0TraitConst, super::CUDA_NvidiaOpticalFlow_2_0Trait };
	}
	
	/// < Fast perf level results in high performance and low quality
	pub const CUDA_NvidiaOpticalFlow_1_0_NV_OF_PERF_LEVEL_FAST: i32 = 20;
	pub const CUDA_NvidiaOpticalFlow_1_0_NV_OF_PERF_LEVEL_MAX: i32 = 21;
	/// < Medium perf level results in low performance and medium quality
	pub const CUDA_NvidiaOpticalFlow_1_0_NV_OF_PERF_LEVEL_MEDIUM: i32 = 10;
	/// < Slow perf level results in lowest performance and best quality
	pub const CUDA_NvidiaOpticalFlow_1_0_NV_OF_PERF_LEVEL_SLOW: i32 = 5;
	pub const CUDA_NvidiaOpticalFlow_1_0_NV_OF_PERF_LEVEL_UNDEFINED: i32 = 0;
	/// < Hint buffer grid size is 1x1.
	pub const CUDA_NvidiaOpticalFlow_2_0_NV_OF_HINT_VECTOR_GRID_SIZE_1: i32 = 1;
	/// < Hint buffer grid size is 2x2.
	pub const CUDA_NvidiaOpticalFlow_2_0_NV_OF_HINT_VECTOR_GRID_SIZE_2: i32 = 2;
	/// < Hint buffer grid size is 4x4.
	pub const CUDA_NvidiaOpticalFlow_2_0_NV_OF_HINT_VECTOR_GRID_SIZE_4: i32 = 4;
	/// < Hint buffer grid size is 8x8.
	pub const CUDA_NvidiaOpticalFlow_2_0_NV_OF_HINT_VECTOR_GRID_SIZE_8: i32 = 8;
	pub const CUDA_NvidiaOpticalFlow_2_0_NV_OF_HINT_VECTOR_GRID_SIZE_MAX: i32 = 9;
	pub const CUDA_NvidiaOpticalFlow_2_0_NV_OF_HINT_VECTOR_GRID_SIZE_UNDEFINED: i32 = 0;
	/// < Output buffer grid size is 1x1
	pub const CUDA_NvidiaOpticalFlow_2_0_NV_OF_OUTPUT_VECTOR_GRID_SIZE_1: i32 = 1;
	/// < Output buffer grid size is 2x2
	pub const CUDA_NvidiaOpticalFlow_2_0_NV_OF_OUTPUT_VECTOR_GRID_SIZE_2: i32 = 2;
	/// < Output buffer grid size is 4x4
	pub const CUDA_NvidiaOpticalFlow_2_0_NV_OF_OUTPUT_VECTOR_GRID_SIZE_4: i32 = 4;
	pub const CUDA_NvidiaOpticalFlow_2_0_NV_OF_OUTPUT_VECTOR_GRID_SIZE_MAX: i32 = 5;
	pub const CUDA_NvidiaOpticalFlow_2_0_NV_OF_OUTPUT_VECTOR_GRID_SIZE_UNDEFINED: i32 = 0;
	/// < Fast perf level results in high performance and low quality
	pub const CUDA_NvidiaOpticalFlow_2_0_NV_OF_PERF_LEVEL_FAST: i32 = 20;
	pub const CUDA_NvidiaOpticalFlow_2_0_NV_OF_PERF_LEVEL_MAX: i32 = 21;
	/// < Medium perf level results in low performance and medium quality
	pub const CUDA_NvidiaOpticalFlow_2_0_NV_OF_PERF_LEVEL_MEDIUM: i32 = 10;
	/// < Slow perf level results in lowest performance and best quality
	pub const CUDA_NvidiaOpticalFlow_2_0_NV_OF_PERF_LEVEL_SLOW: i32 = 5;
	pub const CUDA_NvidiaOpticalFlow_2_0_NV_OF_PERF_LEVEL_UNDEFINED: i32 = 0;
	/// Supported optical flow performance levels.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum CUDA_NvidiaOpticalFlow_1_0_NVIDIA_OF_PERF_LEVEL {
		NV_OF_PERF_LEVEL_UNDEFINED = 0,
		/// < Slow perf level results in lowest performance and best quality
		NV_OF_PERF_LEVEL_SLOW = 5,
		/// < Medium perf level results in low performance and medium quality
		NV_OF_PERF_LEVEL_MEDIUM = 10,
		/// < Fast perf level results in high performance and low quality
		NV_OF_PERF_LEVEL_FAST = 20,
		NV_OF_PERF_LEVEL_MAX = 21,
	}
	
	opencv_type_enum! { crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0_NVIDIA_OF_PERF_LEVEL }
	
	/// Supported grid size for hint buffer.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum CUDA_NvidiaOpticalFlow_2_0_NVIDIA_OF_HINT_VECTOR_GRID_SIZE {
		NV_OF_HINT_VECTOR_GRID_SIZE_UNDEFINED = 0,
		/// < Hint buffer grid size is 1x1.
		NV_OF_HINT_VECTOR_GRID_SIZE_1 = 1,
		/// < Hint buffer grid size is 2x2.
		NV_OF_HINT_VECTOR_GRID_SIZE_2 = 2,
		/// < Hint buffer grid size is 4x4.
		NV_OF_HINT_VECTOR_GRID_SIZE_4 = 4,
		/// < Hint buffer grid size is 8x8.
		NV_OF_HINT_VECTOR_GRID_SIZE_8 = 8,
		NV_OF_HINT_VECTOR_GRID_SIZE_MAX = 9,
	}
	
	opencv_type_enum! { crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0_NVIDIA_OF_HINT_VECTOR_GRID_SIZE }
	
	/// Supported grid size for output buffer.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum CUDA_NvidiaOpticalFlow_2_0_NVIDIA_OF_OUTPUT_VECTOR_GRID_SIZE {
		NV_OF_OUTPUT_VECTOR_GRID_SIZE_UNDEFINED = 0,
		/// < Output buffer grid size is 1x1
		NV_OF_OUTPUT_VECTOR_GRID_SIZE_1 = 1,
		/// < Output buffer grid size is 2x2
		NV_OF_OUTPUT_VECTOR_GRID_SIZE_2 = 2,
		/// < Output buffer grid size is 4x4
		NV_OF_OUTPUT_VECTOR_GRID_SIZE_4 = 4,
		NV_OF_OUTPUT_VECTOR_GRID_SIZE_MAX = 5,
	}
	
	opencv_type_enum! { crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0_NVIDIA_OF_OUTPUT_VECTOR_GRID_SIZE }
	
	/// Supported optical flow performance levels.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum CUDA_NvidiaOpticalFlow_2_0_NVIDIA_OF_PERF_LEVEL {
		NV_OF_PERF_LEVEL_UNDEFINED = 0,
		/// < Slow perf level results in lowest performance and best quality
		NV_OF_PERF_LEVEL_SLOW = 5,
		/// < Medium perf level results in low performance and medium quality
		NV_OF_PERF_LEVEL_MEDIUM = 10,
		/// < Fast perf level results in high performance and low quality
		NV_OF_PERF_LEVEL_FAST = 20,
		NV_OF_PERF_LEVEL_MAX = 21,
	}
	
	opencv_type_enum! { crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0_NVIDIA_OF_PERF_LEVEL }
	
	/// Constant methods for [crate::cudaoptflow::CUDA_BroxOpticalFlow]
	pub trait CUDA_BroxOpticalFlowTraitConst: crate::cudaoptflow::CUDA_DenseOpticalFlowTraitConst {
		fn as_raw_CUDA_BroxOpticalFlow(&self) -> *const c_void;
	
		#[inline]
		fn get_flow_smoothness(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BroxOpticalFlow_getFlowSmoothness_const(self.as_raw_CUDA_BroxOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_gradient_constancy_importance(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BroxOpticalFlow_getGradientConstancyImportance_const(self.as_raw_CUDA_BroxOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_pyramid_scale_factor(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BroxOpticalFlow_getPyramidScaleFactor_const(self.as_raw_CUDA_BroxOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// number of lagged non-linearity iterations (inner loop)
		#[inline]
		fn get_inner_iterations(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BroxOpticalFlow_getInnerIterations_const(self.as_raw_CUDA_BroxOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// number of warping iterations (number of pyramid levels)
		#[inline]
		fn get_outer_iterations(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BroxOpticalFlow_getOuterIterations_const(self.as_raw_CUDA_BroxOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// number of linear system solver iterations
		#[inline]
		fn get_solver_iterations(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BroxOpticalFlow_getSolverIterations_const(self.as_raw_CUDA_BroxOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::cudaoptflow::CUDA_BroxOpticalFlow]
	pub trait CUDA_BroxOpticalFlowTrait: crate::cudaoptflow::CUDA_BroxOpticalFlowTraitConst + crate::cudaoptflow::CUDA_DenseOpticalFlowTrait {
		fn as_raw_mut_CUDA_BroxOpticalFlow(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_flow_smoothness(&mut self, alpha: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BroxOpticalFlow_setFlowSmoothness_double(self.as_raw_mut_CUDA_BroxOpticalFlow(), alpha, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_gradient_constancy_importance(&mut self, gamma: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BroxOpticalFlow_setGradientConstancyImportance_double(self.as_raw_mut_CUDA_BroxOpticalFlow(), gamma, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_pyramid_scale_factor(&mut self, scale_factor: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BroxOpticalFlow_setPyramidScaleFactor_double(self.as_raw_mut_CUDA_BroxOpticalFlow(), scale_factor, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_inner_iterations(&mut self, inner_iterations: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BroxOpticalFlow_setInnerIterations_int(self.as_raw_mut_CUDA_BroxOpticalFlow(), inner_iterations, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_outer_iterations(&mut self, outer_iterations: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BroxOpticalFlow_setOuterIterations_int(self.as_raw_mut_CUDA_BroxOpticalFlow(), outer_iterations, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_solver_iterations(&mut self, solver_iterations: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BroxOpticalFlow_setSolverIterations_int(self.as_raw_mut_CUDA_BroxOpticalFlow(), solver_iterations, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Class computing the optical flow for two images using Brox et al Optical Flow algorithm ([Brox2004](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Brox2004)).
	pub struct CUDA_BroxOpticalFlow {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CUDA_BroxOpticalFlow }
	
	impl Drop for CUDA_BroxOpticalFlow {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_BroxOpticalFlow_delete(self.as_raw_mut_CUDA_BroxOpticalFlow()) };
		}
	}
	
	unsafe impl Send for CUDA_BroxOpticalFlow {}
	
	impl core::AlgorithmTraitConst for CUDA_BroxOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CUDA_BroxOpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlowTraitConst for CUDA_BroxOpticalFlow {
		#[inline] fn as_raw_CUDA_DenseOpticalFlow(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlowTrait for CUDA_BroxOpticalFlow {
		#[inline] fn as_raw_mut_CUDA_DenseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_BroxOpticalFlowTraitConst for CUDA_BroxOpticalFlow {
		#[inline] fn as_raw_CUDA_BroxOpticalFlow(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_BroxOpticalFlowTrait for CUDA_BroxOpticalFlow {
		#[inline] fn as_raw_mut_CUDA_BroxOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CUDA_BroxOpticalFlow {
		/// ## C++ default parameters
		/// * alpha: 0.197
		/// * gamma: 50.0
		/// * scale_factor: 0.8
		/// * inner_iterations: 5
		/// * outer_iterations: 150
		/// * solver_iterations: 10
		#[inline]
		pub fn create(alpha: f64, gamma: f64, scale_factor: f64, inner_iterations: i32, outer_iterations: i32, solver_iterations: i32) -> Result<core::Ptr<crate::cudaoptflow::CUDA_BroxOpticalFlow>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BroxOpticalFlow_create_double_double_double_int_int_int(alpha, gamma, scale_factor, inner_iterations, outer_iterations, solver_iterations, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudaoptflow::CUDA_BroxOpticalFlow>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * alpha: 0.197
		/// * gamma: 50.0
		/// * scale_factor: 0.8
		/// * inner_iterations: 5
		/// * outer_iterations: 150
		/// * solver_iterations: 10
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::cudaoptflow::CUDA_BroxOpticalFlow>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BroxOpticalFlow_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudaoptflow::CUDA_BroxOpticalFlow>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { CUDA_BroxOpticalFlow, core::Algorithm, cv_cuda_BroxOpticalFlow_to_Algorithm }
	
	boxed_cast_base! { CUDA_BroxOpticalFlow, crate::cudaoptflow::CUDA_DenseOpticalFlow, cv_cuda_BroxOpticalFlow_to_CUDA_DenseOpticalFlow }
	
	impl std::fmt::Debug for CUDA_BroxOpticalFlow {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_BroxOpticalFlow")
				.finish()
		}
	}
	
	/// Constant methods for [crate::cudaoptflow::CUDA_DenseOpticalFlow]
	pub trait CUDA_DenseOpticalFlowTraitConst: core::AlgorithmTraitConst {
		fn as_raw_CUDA_DenseOpticalFlow(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::cudaoptflow::CUDA_DenseOpticalFlow]
	pub trait CUDA_DenseOpticalFlowTrait: core::AlgorithmTrait + crate::cudaoptflow::CUDA_DenseOpticalFlowTraitConst {
		fn as_raw_mut_CUDA_DenseOpticalFlow(&mut self) -> *mut c_void;
	
		/// Calculates a dense optical flow.
		/// 
		/// ## Parameters
		/// * I0: first input image.
		/// * I1: second input image of the same size and the same type as I0.
		/// * flow: computed flow image that has the same size as I0 and type CV_32FC2.
		/// * stream: Stream for the asynchronous version.
		/// 
		/// ## C++ default parameters
		/// * stream: Stream::Null()
		#[inline]
		fn calc(&mut self, i0: &impl core::ToInputArray, i1: &impl core::ToInputArray, flow: &mut impl core::ToInputOutputArray, stream: &mut core::Stream) -> Result<()> {
			input_array_arg!(i0);
			input_array_arg!(i1);
			input_output_array_arg!(flow);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DenseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_StreamR(self.as_raw_mut_CUDA_DenseOpticalFlow(), i0.as_raw__InputArray(), i1.as_raw__InputArray(), flow.as_raw__InputOutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Calculates a dense optical flow.
		/// 
		/// ## Parameters
		/// * I0: first input image.
		/// * I1: second input image of the same size and the same type as I0.
		/// * flow: computed flow image that has the same size as I0 and type CV_32FC2.
		/// * stream: Stream for the asynchronous version.
		/// 
		/// ## Note
		/// This alternative version of [calc] function uses the following default values for its arguments:
		/// * stream: Stream::Null()
		#[inline]
		fn calc_def(&mut self, i0: &impl core::ToInputArray, i1: &impl core::ToInputArray, flow: &mut impl core::ToInputOutputArray) -> Result<()> {
			input_array_arg!(i0);
			input_array_arg!(i1);
			input_output_array_arg!(flow);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DenseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(self.as_raw_mut_CUDA_DenseOpticalFlow(), i0.as_raw__InputArray(), i1.as_raw__InputArray(), flow.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Base interface for dense optical flow algorithms.
	pub struct CUDA_DenseOpticalFlow {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CUDA_DenseOpticalFlow }
	
	impl Drop for CUDA_DenseOpticalFlow {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_DenseOpticalFlow_delete(self.as_raw_mut_CUDA_DenseOpticalFlow()) };
		}
	}
	
	unsafe impl Send for CUDA_DenseOpticalFlow {}
	
	impl core::AlgorithmTraitConst for CUDA_DenseOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CUDA_DenseOpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlowTraitConst for CUDA_DenseOpticalFlow {
		#[inline] fn as_raw_CUDA_DenseOpticalFlow(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlowTrait for CUDA_DenseOpticalFlow {
		#[inline] fn as_raw_mut_CUDA_DenseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CUDA_DenseOpticalFlow {
	}
	
	boxed_cast_descendant! { CUDA_DenseOpticalFlow, crate::cudaoptflow::CUDA_BroxOpticalFlow, cv_cuda_DenseOpticalFlow_to_CUDA_BroxOpticalFlow }
	
	boxed_cast_descendant! { CUDA_DenseOpticalFlow, crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow, cv_cuda_DenseOpticalFlow_to_CUDA_DensePyrLKOpticalFlow }
	
	boxed_cast_descendant! { CUDA_DenseOpticalFlow, crate::cudaoptflow::CUDA_FarnebackOpticalFlow, cv_cuda_DenseOpticalFlow_to_CUDA_FarnebackOpticalFlow }
	
	boxed_cast_descendant! { CUDA_DenseOpticalFlow, crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1, cv_cuda_DenseOpticalFlow_to_CUDA_OpticalFlowDual_TVL1 }
	
	boxed_cast_base! { CUDA_DenseOpticalFlow, core::Algorithm, cv_cuda_DenseOpticalFlow_to_Algorithm }
	
	impl std::fmt::Debug for CUDA_DenseOpticalFlow {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_DenseOpticalFlow")
				.finish()
		}
	}
	
	/// Constant methods for [crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow]
	pub trait CUDA_DensePyrLKOpticalFlowTraitConst: crate::cudaoptflow::CUDA_DenseOpticalFlowTraitConst {
		fn as_raw_CUDA_DensePyrLKOpticalFlow(&self) -> *const c_void;
	
		#[inline]
		fn get_win_size(&self) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DensePyrLKOpticalFlow_getWinSize_const(self.as_raw_CUDA_DensePyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_max_level(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DensePyrLKOpticalFlow_getMaxLevel_const(self.as_raw_CUDA_DensePyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_num_iters(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DensePyrLKOpticalFlow_getNumIters_const(self.as_raw_CUDA_DensePyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_use_initial_flow(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DensePyrLKOpticalFlow_getUseInitialFlow_const(self.as_raw_CUDA_DensePyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow]
	pub trait CUDA_DensePyrLKOpticalFlowTrait: crate::cudaoptflow::CUDA_DenseOpticalFlowTrait + crate::cudaoptflow::CUDA_DensePyrLKOpticalFlowTraitConst {
		fn as_raw_mut_CUDA_DensePyrLKOpticalFlow(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_win_size(&mut self, win_size: core::Size) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DensePyrLKOpticalFlow_setWinSize_Size(self.as_raw_mut_CUDA_DensePyrLKOpticalFlow(), win_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_max_level(&mut self, max_level: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DensePyrLKOpticalFlow_setMaxLevel_int(self.as_raw_mut_CUDA_DensePyrLKOpticalFlow(), max_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_num_iters(&mut self, iters: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DensePyrLKOpticalFlow_setNumIters_int(self.as_raw_mut_CUDA_DensePyrLKOpticalFlow(), iters, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_use_initial_flow(&mut self, use_initial_flow: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DensePyrLKOpticalFlow_setUseInitialFlow_bool(self.as_raw_mut_CUDA_DensePyrLKOpticalFlow(), use_initial_flow, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Class used for calculating a dense optical flow.
	/// 
	/// The class can calculate an optical flow for a dense optical flow using the
	/// iterative Lucas-Kanade method with pyramids.
	pub struct CUDA_DensePyrLKOpticalFlow {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CUDA_DensePyrLKOpticalFlow }
	
	impl Drop for CUDA_DensePyrLKOpticalFlow {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_DensePyrLKOpticalFlow_delete(self.as_raw_mut_CUDA_DensePyrLKOpticalFlow()) };
		}
	}
	
	unsafe impl Send for CUDA_DensePyrLKOpticalFlow {}
	
	impl core::AlgorithmTraitConst for CUDA_DensePyrLKOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CUDA_DensePyrLKOpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlowTraitConst for CUDA_DensePyrLKOpticalFlow {
		#[inline] fn as_raw_CUDA_DenseOpticalFlow(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlowTrait for CUDA_DensePyrLKOpticalFlow {
		#[inline] fn as_raw_mut_CUDA_DenseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_DensePyrLKOpticalFlowTraitConst for CUDA_DensePyrLKOpticalFlow {
		#[inline] fn as_raw_CUDA_DensePyrLKOpticalFlow(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_DensePyrLKOpticalFlowTrait for CUDA_DensePyrLKOpticalFlow {
		#[inline] fn as_raw_mut_CUDA_DensePyrLKOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CUDA_DensePyrLKOpticalFlow {
		/// ## C++ default parameters
		/// * win_size: Size(13,13)
		/// * max_level: 3
		/// * iters: 30
		/// * use_initial_flow: false
		#[inline]
		pub fn create(win_size: core::Size, max_level: i32, iters: i32, use_initial_flow: bool) -> Result<core::Ptr<crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DensePyrLKOpticalFlow_create_Size_int_int_bool(win_size.opencv_as_extern(), max_level, iters, use_initial_flow, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * win_size: Size(13,13)
		/// * max_level: 3
		/// * iters: 30
		/// * use_initial_flow: false
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DensePyrLKOpticalFlow_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { CUDA_DensePyrLKOpticalFlow, core::Algorithm, cv_cuda_DensePyrLKOpticalFlow_to_Algorithm }
	
	boxed_cast_base! { CUDA_DensePyrLKOpticalFlow, crate::cudaoptflow::CUDA_DenseOpticalFlow, cv_cuda_DensePyrLKOpticalFlow_to_CUDA_DenseOpticalFlow }
	
	impl std::fmt::Debug for CUDA_DensePyrLKOpticalFlow {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_DensePyrLKOpticalFlow")
				.finish()
		}
	}
	
	/// Constant methods for [crate::cudaoptflow::CUDA_FarnebackOpticalFlow]
	pub trait CUDA_FarnebackOpticalFlowTraitConst: crate::cudaoptflow::CUDA_DenseOpticalFlowTraitConst {
		fn as_raw_CUDA_FarnebackOpticalFlow(&self) -> *const c_void;
	
		#[inline]
		fn get_num_levels(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FarnebackOpticalFlow_getNumLevels_const(self.as_raw_CUDA_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_pyr_scale(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FarnebackOpticalFlow_getPyrScale_const(self.as_raw_CUDA_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_fast_pyramids(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FarnebackOpticalFlow_getFastPyramids_const(self.as_raw_CUDA_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_win_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FarnebackOpticalFlow_getWinSize_const(self.as_raw_CUDA_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_num_iters(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FarnebackOpticalFlow_getNumIters_const(self.as_raw_CUDA_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_poly_n(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FarnebackOpticalFlow_getPolyN_const(self.as_raw_CUDA_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_poly_sigma(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FarnebackOpticalFlow_getPolySigma_const(self.as_raw_CUDA_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_flags(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FarnebackOpticalFlow_getFlags_const(self.as_raw_CUDA_FarnebackOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::cudaoptflow::CUDA_FarnebackOpticalFlow]
	pub trait CUDA_FarnebackOpticalFlowTrait: crate::cudaoptflow::CUDA_DenseOpticalFlowTrait + crate::cudaoptflow::CUDA_FarnebackOpticalFlowTraitConst {
		fn as_raw_mut_CUDA_FarnebackOpticalFlow(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_num_levels(&mut self, num_levels: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FarnebackOpticalFlow_setNumLevels_int(self.as_raw_mut_CUDA_FarnebackOpticalFlow(), num_levels, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_pyr_scale(&mut self, pyr_scale: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FarnebackOpticalFlow_setPyrScale_double(self.as_raw_mut_CUDA_FarnebackOpticalFlow(), pyr_scale, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_fast_pyramids(&mut self, fast_pyramids: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FarnebackOpticalFlow_setFastPyramids_bool(self.as_raw_mut_CUDA_FarnebackOpticalFlow(), fast_pyramids, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_win_size(&mut self, win_size: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FarnebackOpticalFlow_setWinSize_int(self.as_raw_mut_CUDA_FarnebackOpticalFlow(), win_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_num_iters(&mut self, num_iters: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FarnebackOpticalFlow_setNumIters_int(self.as_raw_mut_CUDA_FarnebackOpticalFlow(), num_iters, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_poly_n(&mut self, poly_n: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FarnebackOpticalFlow_setPolyN_int(self.as_raw_mut_CUDA_FarnebackOpticalFlow(), poly_n, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_poly_sigma(&mut self, poly_sigma: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FarnebackOpticalFlow_setPolySigma_double(self.as_raw_mut_CUDA_FarnebackOpticalFlow(), poly_sigma, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_flags(&mut self, flags: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FarnebackOpticalFlow_setFlags_int(self.as_raw_mut_CUDA_FarnebackOpticalFlow(), flags, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Class computing a dense optical flow using the Gunnar Farneback's algorithm.
	pub struct CUDA_FarnebackOpticalFlow {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CUDA_FarnebackOpticalFlow }
	
	impl Drop for CUDA_FarnebackOpticalFlow {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_FarnebackOpticalFlow_delete(self.as_raw_mut_CUDA_FarnebackOpticalFlow()) };
		}
	}
	
	unsafe impl Send for CUDA_FarnebackOpticalFlow {}
	
	impl core::AlgorithmTraitConst for CUDA_FarnebackOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CUDA_FarnebackOpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlowTraitConst for CUDA_FarnebackOpticalFlow {
		#[inline] fn as_raw_CUDA_DenseOpticalFlow(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlowTrait for CUDA_FarnebackOpticalFlow {
		#[inline] fn as_raw_mut_CUDA_DenseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_FarnebackOpticalFlowTraitConst for CUDA_FarnebackOpticalFlow {
		#[inline] fn as_raw_CUDA_FarnebackOpticalFlow(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_FarnebackOpticalFlowTrait for CUDA_FarnebackOpticalFlow {
		#[inline] fn as_raw_mut_CUDA_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CUDA_FarnebackOpticalFlow {
		/// ## C++ default parameters
		/// * num_levels: 5
		/// * pyr_scale: 0.5
		/// * fast_pyramids: false
		/// * win_size: 13
		/// * num_iters: 10
		/// * poly_n: 5
		/// * poly_sigma: 1.1
		/// * flags: 0
		#[inline]
		pub fn create(num_levels: i32, pyr_scale: f64, fast_pyramids: bool, win_size: i32, num_iters: i32, poly_n: i32, poly_sigma: f64, flags: i32) -> Result<core::Ptr<crate::cudaoptflow::CUDA_FarnebackOpticalFlow>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FarnebackOpticalFlow_create_int_double_bool_int_int_int_double_int(num_levels, pyr_scale, fast_pyramids, win_size, num_iters, poly_n, poly_sigma, flags, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudaoptflow::CUDA_FarnebackOpticalFlow>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * num_levels: 5
		/// * pyr_scale: 0.5
		/// * fast_pyramids: false
		/// * win_size: 13
		/// * num_iters: 10
		/// * poly_n: 5
		/// * poly_sigma: 1.1
		/// * flags: 0
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::cudaoptflow::CUDA_FarnebackOpticalFlow>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FarnebackOpticalFlow_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudaoptflow::CUDA_FarnebackOpticalFlow>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { CUDA_FarnebackOpticalFlow, core::Algorithm, cv_cuda_FarnebackOpticalFlow_to_Algorithm }
	
	boxed_cast_base! { CUDA_FarnebackOpticalFlow, crate::cudaoptflow::CUDA_DenseOpticalFlow, cv_cuda_FarnebackOpticalFlow_to_CUDA_DenseOpticalFlow }
	
	impl std::fmt::Debug for CUDA_FarnebackOpticalFlow {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_FarnebackOpticalFlow")
				.finish()
		}
	}
	
	/// Constant methods for [crate::cudaoptflow::CUDA_NvidiaHWOpticalFlow]
	pub trait CUDA_NvidiaHWOpticalFlowTraitConst: core::AlgorithmTraitConst {
		fn as_raw_CUDA_NvidiaHWOpticalFlow(&self) -> *const c_void;
	
		/// Returns grid size of output buffer as per the hardware's capability.
		#[inline]
		fn get_grid_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_NvidiaHWOpticalFlow_getGridSize_const(self.as_raw_CUDA_NvidiaHWOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::cudaoptflow::CUDA_NvidiaHWOpticalFlow]
	pub trait CUDA_NvidiaHWOpticalFlowTrait: core::AlgorithmTrait + crate::cudaoptflow::CUDA_NvidiaHWOpticalFlowTraitConst {
		fn as_raw_mut_CUDA_NvidiaHWOpticalFlow(&mut self) -> *mut c_void;
	
		/// Calculates Optical Flow using NVIDIA Optical Flow SDK.
		/// 
		/// * NVIDIA GPUs starting with Turing contain a dedicated hardware accelerator for computing optical flow vectors between pairs of images.
		/// * The optical flow hardware accelerator generates block-based optical flow vectors.
		/// * The size of the block depends on hardware in use, and can be queried using the function getGridSize().
		/// * The block-based flow vectors generated by the hardware can be converted to dense representation (i.e. per-pixel flow vectors) using upSampler() helper function, if needed.
		/// * The flow vectors are stored in CV_16SC2 format with x and y components of each flow vector in 16-bit signed fixed point representation S10.5.
		/// 
		/// ## Parameters
		/// * inputImage: Input image.
		/// * referenceImage: Reference image of the same size and the same type as input image.
		/// * flow: A buffer consisting of inputImage.Size() / getGridSize() flow vectors in CV_16SC2 format.
		/// * stream: It is highly recommended that CUDA streams for pre and post processing of optical flow vectors should be set once per session in create() function as a part of optical flow session creation.
		///               This parameter is left here for backward compatibility and may be removed in the future.
		///               Default value is NULL stream;
		/// * hint: Hint buffer if client provides external hints. Must have same size as flow buffer.
		///            Caller can provide flow vectors as hints for optical flow calculation.
		/// * cost: Cost buffer contains numbers indicating the confidence associated with each of the generated flow vectors.
		///            Higher the cost, lower the confidence. Cost buffer is of type CV_32SC1.
		/// 
		/// 
		/// Note:
		/// - Client must use critical sections around each calc() function if calling it from multiple threads.
		/// 
		/// ## C++ default parameters
		/// * stream: Stream::Null()
		/// * hint: cv::noArray()
		/// * cost: cv::noArray()
		#[inline]
		fn calc(&mut self, input_image: &impl core::ToInputArray, reference_image: &impl core::ToInputArray, flow: &mut impl core::ToInputOutputArray, stream: &mut core::Stream, hint: &impl core::ToInputArray, cost: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(input_image);
			input_array_arg!(reference_image);
			input_output_array_arg!(flow);
			input_array_arg!(hint);
			output_array_arg!(cost);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_NvidiaHWOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_StreamR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_CUDA_NvidiaHWOpticalFlow(), input_image.as_raw__InputArray(), reference_image.as_raw__InputArray(), flow.as_raw__InputOutputArray(), stream.as_raw_mut_Stream(), hint.as_raw__InputArray(), cost.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Calculates Optical Flow using NVIDIA Optical Flow SDK.
		/// 
		/// * NVIDIA GPUs starting with Turing contain a dedicated hardware accelerator for computing optical flow vectors between pairs of images.
		/// * The optical flow hardware accelerator generates block-based optical flow vectors.
		/// * The size of the block depends on hardware in use, and can be queried using the function getGridSize().
		/// * The block-based flow vectors generated by the hardware can be converted to dense representation (i.e. per-pixel flow vectors) using upSampler() helper function, if needed.
		/// * The flow vectors are stored in CV_16SC2 format with x and y components of each flow vector in 16-bit signed fixed point representation S10.5.
		/// 
		/// ## Parameters
		/// * inputImage: Input image.
		/// * referenceImage: Reference image of the same size and the same type as input image.
		/// * flow: A buffer consisting of inputImage.Size() / getGridSize() flow vectors in CV_16SC2 format.
		/// * stream: It is highly recommended that CUDA streams for pre and post processing of optical flow vectors should be set once per session in create() function as a part of optical flow session creation.
		///               This parameter is left here for backward compatibility and may be removed in the future.
		///               Default value is NULL stream;
		/// * hint: Hint buffer if client provides external hints. Must have same size as flow buffer.
		///            Caller can provide flow vectors as hints for optical flow calculation.
		/// * cost: Cost buffer contains numbers indicating the confidence associated with each of the generated flow vectors.
		///            Higher the cost, lower the confidence. Cost buffer is of type CV_32SC1.
		/// 
		/// 
		/// Note:
		/// - Client must use critical sections around each calc() function if calling it from multiple threads.
		/// 
		/// ## Note
		/// This alternative version of [calc] function uses the following default values for its arguments:
		/// * stream: Stream::Null()
		/// * hint: cv::noArray()
		/// * cost: cv::noArray()
		#[inline]
		fn calc_def(&mut self, input_image: &impl core::ToInputArray, reference_image: &impl core::ToInputArray, flow: &mut impl core::ToInputOutputArray) -> Result<()> {
			input_array_arg!(input_image);
			input_array_arg!(reference_image);
			input_output_array_arg!(flow);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_NvidiaHWOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(self.as_raw_mut_CUDA_NvidiaHWOpticalFlow(), input_image.as_raw__InputArray(), reference_image.as_raw__InputArray(), flow.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Releases all buffers, contexts and device pointers.
		#[inline]
		fn collect_garbage(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_NvidiaHWOpticalFlow_collectGarbage(self.as_raw_mut_CUDA_NvidiaHWOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Base Interface for optical flow algorithms using NVIDIA Optical Flow SDK.
	pub struct CUDA_NvidiaHWOpticalFlow {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CUDA_NvidiaHWOpticalFlow }
	
	impl Drop for CUDA_NvidiaHWOpticalFlow {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_NvidiaHWOpticalFlow_delete(self.as_raw_mut_CUDA_NvidiaHWOpticalFlow()) };
		}
	}
	
	unsafe impl Send for CUDA_NvidiaHWOpticalFlow {}
	
	impl core::AlgorithmTraitConst for CUDA_NvidiaHWOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CUDA_NvidiaHWOpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaHWOpticalFlowTraitConst for CUDA_NvidiaHWOpticalFlow {
		#[inline] fn as_raw_CUDA_NvidiaHWOpticalFlow(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaHWOpticalFlowTrait for CUDA_NvidiaHWOpticalFlow {
		#[inline] fn as_raw_mut_CUDA_NvidiaHWOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CUDA_NvidiaHWOpticalFlow {
	}
	
	boxed_cast_descendant! { CUDA_NvidiaHWOpticalFlow, crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0, cv_cuda_NvidiaHWOpticalFlow_to_CUDA_NvidiaOpticalFlow_1_0 }
	
	boxed_cast_descendant! { CUDA_NvidiaHWOpticalFlow, crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0, cv_cuda_NvidiaHWOpticalFlow_to_CUDA_NvidiaOpticalFlow_2_0 }
	
	boxed_cast_base! { CUDA_NvidiaHWOpticalFlow, core::Algorithm, cv_cuda_NvidiaHWOpticalFlow_to_Algorithm }
	
	impl std::fmt::Debug for CUDA_NvidiaHWOpticalFlow {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_NvidiaHWOpticalFlow")
				.finish()
		}
	}
	
	/// Constant methods for [crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0]
	pub trait CUDA_NvidiaOpticalFlow_1_0TraitConst: crate::cudaoptflow::CUDA_NvidiaHWOpticalFlowTraitConst {
		fn as_raw_CUDA_NvidiaOpticalFlow_1_0(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0]
	pub trait CUDA_NvidiaOpticalFlow_1_0Trait: crate::cudaoptflow::CUDA_NvidiaHWOpticalFlowTrait + crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0TraitConst {
		fn as_raw_mut_CUDA_NvidiaOpticalFlow_1_0(&mut self) -> *mut c_void;
	
		/// The NVIDIA optical flow hardware generates flow vectors at granularity gridSize, which can be queried via function getGridSize().
		/// Upsampler() helper function converts the hardware-generated flow vectors to dense representation (1 flow vector for each pixel)
		/// using nearest neighbour upsampling method.
		/// 
		/// ## Parameters
		/// * flow: Buffer of type CV_16FC2 containing flow vectors generated by calc().
		/// * imageSize: Size of the input image in pixels for which these flow vectors were generated.
		/// * gridSize: Granularity of the optical flow vectors returned by calc() function. Can be queried using getGridSize().
		/// * upsampledFlow: Buffer of type CV_32FC2, containing upsampled flow vectors, each flow vector for 1 pixel, in the pitch-linear layout.
		#[inline]
		fn up_sampler(&mut self, flow: &impl core::ToInputArray, image_size: core::Size, grid_size: i32, upsampled_flow: &mut impl core::ToInputOutputArray) -> Result<()> {
			input_array_arg!(flow);
			input_output_array_arg!(upsampled_flow);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_NvidiaOpticalFlow_1_0_upSampler_const__InputArrayR_Size_int_const__InputOutputArrayR(self.as_raw_mut_CUDA_NvidiaOpticalFlow_1_0(), flow.as_raw__InputArray(), image_size.opencv_as_extern(), grid_size, upsampled_flow.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Class for computing the optical flow vectors between two images using NVIDIA Optical Flow hardware and Optical Flow SDK 1.0.
	/// 
	/// Note:
	/// - A sample application demonstrating the use of NVIDIA Optical Flow can be found at
	/// opencv_contrib_source_code/modules/cudaoptflow/samples/nvidia_optical_flow.cpp
	/// - An example application comparing accuracy and performance of NVIDIA Optical Flow with other optical flow algorithms in OpenCV can be found at
	/// opencv_contrib_source_code/modules/cudaoptflow/samples/optical_flow.cpp
	pub struct CUDA_NvidiaOpticalFlow_1_0 {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CUDA_NvidiaOpticalFlow_1_0 }
	
	impl Drop for CUDA_NvidiaOpticalFlow_1_0 {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_NvidiaOpticalFlow_1_0_delete(self.as_raw_mut_CUDA_NvidiaOpticalFlow_1_0()) };
		}
	}
	
	unsafe impl Send for CUDA_NvidiaOpticalFlow_1_0 {}
	
	impl core::AlgorithmTraitConst for CUDA_NvidiaOpticalFlow_1_0 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CUDA_NvidiaOpticalFlow_1_0 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaHWOpticalFlowTraitConst for CUDA_NvidiaOpticalFlow_1_0 {
		#[inline] fn as_raw_CUDA_NvidiaHWOpticalFlow(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaHWOpticalFlowTrait for CUDA_NvidiaOpticalFlow_1_0 {
		#[inline] fn as_raw_mut_CUDA_NvidiaHWOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0TraitConst for CUDA_NvidiaOpticalFlow_1_0 {
		#[inline] fn as_raw_CUDA_NvidiaOpticalFlow_1_0(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0Trait for CUDA_NvidiaOpticalFlow_1_0 {
		#[inline] fn as_raw_mut_CUDA_NvidiaOpticalFlow_1_0(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CUDA_NvidiaOpticalFlow_1_0 {
		/// Instantiate NVIDIA Optical Flow
		/// 
		/// ## Parameters
		/// * imageSize: Size of input image in pixels.
		/// * perfPreset: Optional parameter. Refer [NV OF SDK documentation](https://developer.nvidia.com/opticalflow-sdk) for details about presets.
		///                   Defaults to NV_OF_PERF_LEVEL_SLOW.
		/// * enableTemporalHints: Optional parameter. Flag to enable temporal hints. When set to true, the hardware uses the flow vectors
		///                            generated in previous call to calc() as internal hints for the current call to calc().
		///                            Useful when computing flow vectors between successive video frames. Defaults to false.
		/// * enableExternalHints: Optional Parameter. Flag to enable passing external hints buffer to calc(). Defaults to false.
		/// * enableCostBuffer: Optional Parameter. Flag to enable cost buffer output from calc(). Defaults to false.
		/// * gpuId: Optional parameter to select the GPU ID on which the optical flow should be computed. Useful in multi-GPU systems. Defaults to 0.
		/// * inputStream: Optical flow algorithm may optionally involve cuda preprocessing on the input buffers.
		///                    The input cuda stream can be used to pipeline and synchronize the cuda preprocessing tasks with OF HW engine.
		///                    If input stream is not set, the execute function will use default stream which is NULL stream;
		/// * outputStream: Optical flow algorithm may optionally involve cuda post processing on the output flow vectors.
		///                    The output cuda stream can be used to pipeline and synchronize the cuda post processing tasks with OF HW engine.
		///                    If output stream is not set, the execute function will use default stream which is NULL stream;
		/// 
		/// ## C++ default parameters
		/// * perf_preset: cv::cuda::NvidiaOpticalFlow_1_0::NV_OF_PERF_LEVEL_SLOW
		/// * enable_temporal_hints: false
		/// * enable_external_hints: false
		/// * enable_cost_buffer: false
		/// * gpu_id: 0
		/// * input_stream: Stream::Null()
		/// * output_stream: Stream::Null()
		#[inline]
		pub fn create(image_size: core::Size, perf_preset: crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0_NVIDIA_OF_PERF_LEVEL, enable_temporal_hints: bool, enable_external_hints: bool, enable_cost_buffer: bool, gpu_id: i32, input_stream: &mut core::Stream, output_stream: &mut core::Stream) -> Result<core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_NvidiaOpticalFlow_1_0_create_Size_NVIDIA_OF_PERF_LEVEL_bool_bool_bool_int_StreamR_StreamR(image_size.opencv_as_extern(), perf_preset, enable_temporal_hints, enable_external_hints, enable_cost_buffer, gpu_id, input_stream.as_raw_mut_Stream(), output_stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Instantiate NVIDIA Optical Flow
		/// 
		/// ## Parameters
		/// * imageSize: Size of input image in pixels.
		/// * perfPreset: Optional parameter. Refer [NV OF SDK documentation](https://developer.nvidia.com/opticalflow-sdk) for details about presets.
		///                   Defaults to NV_OF_PERF_LEVEL_SLOW.
		/// * enableTemporalHints: Optional parameter. Flag to enable temporal hints. When set to true, the hardware uses the flow vectors
		///                            generated in previous call to calc() as internal hints for the current call to calc().
		///                            Useful when computing flow vectors between successive video frames. Defaults to false.
		/// * enableExternalHints: Optional Parameter. Flag to enable passing external hints buffer to calc(). Defaults to false.
		/// * enableCostBuffer: Optional Parameter. Flag to enable cost buffer output from calc(). Defaults to false.
		/// * gpuId: Optional parameter to select the GPU ID on which the optical flow should be computed. Useful in multi-GPU systems. Defaults to 0.
		/// * inputStream: Optical flow algorithm may optionally involve cuda preprocessing on the input buffers.
		///                    The input cuda stream can be used to pipeline and synchronize the cuda preprocessing tasks with OF HW engine.
		///                    If input stream is not set, the execute function will use default stream which is NULL stream;
		/// * outputStream: Optical flow algorithm may optionally involve cuda post processing on the output flow vectors.
		///                    The output cuda stream can be used to pipeline and synchronize the cuda post processing tasks with OF HW engine.
		///                    If output stream is not set, the execute function will use default stream which is NULL stream;
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * perf_preset: cv::cuda::NvidiaOpticalFlow_1_0::NV_OF_PERF_LEVEL_SLOW
		/// * enable_temporal_hints: false
		/// * enable_external_hints: false
		/// * enable_cost_buffer: false
		/// * gpu_id: 0
		/// * input_stream: Stream::Null()
		/// * output_stream: Stream::Null()
		#[inline]
		pub fn create_def(image_size: core::Size) -> Result<core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_NvidiaOpticalFlow_1_0_create_Size(image_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { CUDA_NvidiaOpticalFlow_1_0, core::Algorithm, cv_cuda_NvidiaOpticalFlow_1_0_to_Algorithm }
	
	boxed_cast_base! { CUDA_NvidiaOpticalFlow_1_0, crate::cudaoptflow::CUDA_NvidiaHWOpticalFlow, cv_cuda_NvidiaOpticalFlow_1_0_to_CUDA_NvidiaHWOpticalFlow }
	
	impl std::fmt::Debug for CUDA_NvidiaOpticalFlow_1_0 {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_NvidiaOpticalFlow_1_0")
				.finish()
		}
	}
	
	/// Constant methods for [crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0]
	pub trait CUDA_NvidiaOpticalFlow_2_0TraitConst: crate::cudaoptflow::CUDA_NvidiaHWOpticalFlowTraitConst {
		fn as_raw_CUDA_NvidiaOpticalFlow_2_0(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0]
	pub trait CUDA_NvidiaOpticalFlow_2_0Trait: crate::cudaoptflow::CUDA_NvidiaHWOpticalFlowTrait + crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0TraitConst {
		fn as_raw_mut_CUDA_NvidiaOpticalFlow_2_0(&mut self) -> *mut c_void;
	
		/// convertToFloat() helper function converts the hardware-generated flow vectors to floating point representation (1 flow vector for gridSize).
		/// gridSize can be queried via function getGridSize().
		/// 
		/// ## Parameters
		/// * flow: Buffer of type CV_16FC2 containing flow vectors generated by calc().
		/// * floatFlow: Buffer of type CV_32FC2, containing flow vectors in floating point representation, each flow vector for 1 pixel per gridSize, in the pitch-linear layout.
		#[inline]
		fn convert_to_float(&mut self, flow: &impl core::ToInputArray, float_flow: &mut impl core::ToInputOutputArray) -> Result<()> {
			input_array_arg!(flow);
			input_output_array_arg!(float_flow);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_NvidiaOpticalFlow_2_0_convertToFloat_const__InputArrayR_const__InputOutputArrayR(self.as_raw_mut_CUDA_NvidiaOpticalFlow_2_0(), flow.as_raw__InputArray(), float_flow.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Class for computing the optical flow vectors between two images using NVIDIA Optical Flow hardware and Optical Flow SDK 2.0.
	/// 
	/// Note:
	/// - A sample application demonstrating the use of NVIDIA Optical Flow can be found at
	/// opencv_contrib_source_code/modules/cudaoptflow/samples/nvidia_optical_flow.cpp
	/// - An example application comparing accuracy and performance of NVIDIA Optical Flow with other optical flow algorithms in OpenCV can be found at
	/// opencv_contrib_source_code/modules/cudaoptflow/samples/optical_flow.cpp
	pub struct CUDA_NvidiaOpticalFlow_2_0 {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CUDA_NvidiaOpticalFlow_2_0 }
	
	impl Drop for CUDA_NvidiaOpticalFlow_2_0 {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_NvidiaOpticalFlow_2_0_delete(self.as_raw_mut_CUDA_NvidiaOpticalFlow_2_0()) };
		}
	}
	
	unsafe impl Send for CUDA_NvidiaOpticalFlow_2_0 {}
	
	impl core::AlgorithmTraitConst for CUDA_NvidiaOpticalFlow_2_0 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CUDA_NvidiaOpticalFlow_2_0 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaHWOpticalFlowTraitConst for CUDA_NvidiaOpticalFlow_2_0 {
		#[inline] fn as_raw_CUDA_NvidiaHWOpticalFlow(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaHWOpticalFlowTrait for CUDA_NvidiaOpticalFlow_2_0 {
		#[inline] fn as_raw_mut_CUDA_NvidiaHWOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0TraitConst for CUDA_NvidiaOpticalFlow_2_0 {
		#[inline] fn as_raw_CUDA_NvidiaOpticalFlow_2_0(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0Trait for CUDA_NvidiaOpticalFlow_2_0 {
		#[inline] fn as_raw_mut_CUDA_NvidiaOpticalFlow_2_0(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CUDA_NvidiaOpticalFlow_2_0 {
		/// Instantiate NVIDIA Optical Flow
		/// 
		/// ## Parameters
		/// * imageSize: Size of input image in pixels.
		/// * perfPreset: Optional parameter. Refer [NV OF SDK documentation](https://developer.nvidia.com/opticalflow-sdk) for details about presets.
		///                   Defaults to NV_OF_PERF_LEVEL_SLOW.
		/// * outputGridSize: Optional parameter. Refer [NV OF SDK documentation](https://developer.nvidia.com/opticalflow-sdk) for details about output grid sizes.
		///                       Defaults to NV_OF_OUTPUT_VECTOR_GRID_SIZE_1.
		/// * hintGridSize: Optional parameter. Refer [NV OF SDK documentation](https://developer.nvidia.com/opticalflow-sdk) for details about hint grid sizes.
		///                    Defaults to NV_OF_HINT_VECTOR_GRID_SIZE_1.
		/// * enableTemporalHints: Optional parameter. Flag to enable temporal hints. When set to true, the hardware uses the flow vectors
		///                            generated in previous call to calc() as internal hints for the current call to calc().
		///                            Useful when computing flow vectors between successive video frames. Defaults to false.
		/// * enableExternalHints: Optional Parameter. Flag to enable passing external hints buffer to calc(). Defaults to false.
		/// * enableCostBuffer: Optional Parameter. Flag to enable cost buffer output from calc(). Defaults to false.
		/// * gpuId: Optional parameter to select the GPU ID on which the optical flow should be computed. Useful in multi-GPU systems. Defaults to 0.
		/// * inputStream: Optical flow algorithm may optionally involve cuda preprocessing on the input buffers.
		///                    The input cuda stream can be used to pipeline and synchronize the cuda preprocessing tasks with OF HW engine.
		///                    If input stream is not set, the execute function will use default stream which is NULL stream;
		/// * outputStream: Optical flow algorithm may optionally involve cuda post processing on the output flow vectors.
		///                    The output cuda stream can be used to pipeline and synchronize the cuda post processing tasks with OF HW engine.
		///                    If output stream is not set, the execute function will use default stream which is NULL stream;
		/// 
		/// ## C++ default parameters
		/// * perf_preset: cv::cuda::NvidiaOpticalFlow_2_0::NV_OF_PERF_LEVEL_SLOW
		/// * output_grid_size: cv::cuda::NvidiaOpticalFlow_2_0::NV_OF_OUTPUT_VECTOR_GRID_SIZE_1
		/// * hint_grid_size: cv::cuda::NvidiaOpticalFlow_2_0::NV_OF_HINT_VECTOR_GRID_SIZE_1
		/// * enable_temporal_hints: false
		/// * enable_external_hints: false
		/// * enable_cost_buffer: false
		/// * gpu_id: 0
		/// * input_stream: Stream::Null()
		/// * output_stream: Stream::Null()
		#[inline]
		pub fn create(image_size: core::Size, perf_preset: crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0_NVIDIA_OF_PERF_LEVEL, output_grid_size: crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0_NVIDIA_OF_OUTPUT_VECTOR_GRID_SIZE, hint_grid_size: crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0_NVIDIA_OF_HINT_VECTOR_GRID_SIZE, enable_temporal_hints: bool, enable_external_hints: bool, enable_cost_buffer: bool, gpu_id: i32, input_stream: &mut core::Stream, output_stream: &mut core::Stream) -> Result<core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_NvidiaOpticalFlow_2_0_create_Size_NVIDIA_OF_PERF_LEVEL_NVIDIA_OF_OUTPUT_VECTOR_GRID_SIZE_NVIDIA_OF_HINT_VECTOR_GRID_SIZE_bool_bool_bool_int_StreamR_StreamR(image_size.opencv_as_extern(), perf_preset, output_grid_size, hint_grid_size, enable_temporal_hints, enable_external_hints, enable_cost_buffer, gpu_id, input_stream.as_raw_mut_Stream(), output_stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Instantiate NVIDIA Optical Flow
		/// 
		/// ## Parameters
		/// * imageSize: Size of input image in pixels.
		/// * perfPreset: Optional parameter. Refer [NV OF SDK documentation](https://developer.nvidia.com/opticalflow-sdk) for details about presets.
		///                   Defaults to NV_OF_PERF_LEVEL_SLOW.
		/// * outputGridSize: Optional parameter. Refer [NV OF SDK documentation](https://developer.nvidia.com/opticalflow-sdk) for details about output grid sizes.
		///                       Defaults to NV_OF_OUTPUT_VECTOR_GRID_SIZE_1.
		/// * hintGridSize: Optional parameter. Refer [NV OF SDK documentation](https://developer.nvidia.com/opticalflow-sdk) for details about hint grid sizes.
		///                    Defaults to NV_OF_HINT_VECTOR_GRID_SIZE_1.
		/// * enableTemporalHints: Optional parameter. Flag to enable temporal hints. When set to true, the hardware uses the flow vectors
		///                            generated in previous call to calc() as internal hints for the current call to calc().
		///                            Useful when computing flow vectors between successive video frames. Defaults to false.
		/// * enableExternalHints: Optional Parameter. Flag to enable passing external hints buffer to calc(). Defaults to false.
		/// * enableCostBuffer: Optional Parameter. Flag to enable cost buffer output from calc(). Defaults to false.
		/// * gpuId: Optional parameter to select the GPU ID on which the optical flow should be computed. Useful in multi-GPU systems. Defaults to 0.
		/// * inputStream: Optical flow algorithm may optionally involve cuda preprocessing on the input buffers.
		///                    The input cuda stream can be used to pipeline and synchronize the cuda preprocessing tasks with OF HW engine.
		///                    If input stream is not set, the execute function will use default stream which is NULL stream;
		/// * outputStream: Optical flow algorithm may optionally involve cuda post processing on the output flow vectors.
		///                    The output cuda stream can be used to pipeline and synchronize the cuda post processing tasks with OF HW engine.
		///                    If output stream is not set, the execute function will use default stream which is NULL stream;
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * perf_preset: cv::cuda::NvidiaOpticalFlow_2_0::NV_OF_PERF_LEVEL_SLOW
		/// * output_grid_size: cv::cuda::NvidiaOpticalFlow_2_0::NV_OF_OUTPUT_VECTOR_GRID_SIZE_1
		/// * hint_grid_size: cv::cuda::NvidiaOpticalFlow_2_0::NV_OF_HINT_VECTOR_GRID_SIZE_1
		/// * enable_temporal_hints: false
		/// * enable_external_hints: false
		/// * enable_cost_buffer: false
		/// * gpu_id: 0
		/// * input_stream: Stream::Null()
		/// * output_stream: Stream::Null()
		#[inline]
		pub fn create_def(image_size: core::Size) -> Result<core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_NvidiaOpticalFlow_2_0_create_Size(image_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Instantiate NVIDIA Optical Flow with ROI Feature
		/// 
		/// ## Parameters
		/// * imageSize: Size of input image in pixels.
		/// * roiData: Pointer to ROI data.
		/// * perfPreset: Optional parameter. Refer [NV OF SDK documentation](https://developer.nvidia.com/opticalflow-sdk) for details about presets.
		///                   Defaults to NV_OF_PERF_LEVEL_SLOW.
		/// * outputGridSize: Optional parameter. Refer [NV OF SDK documentation](https://developer.nvidia.com/opticalflow-sdk) for details about output grid sizes.
		///                       Defaults to NV_OF_OUTPUT_VECTOR_GRID_SIZE_1.
		/// * hintGridSize: Optional parameter. Refer [NV OF SDK documentation](https://developer.nvidia.com/opticalflow-sdk) for details about hint grid sizes.
		///                    Defaults to NV_OF_HINT_VECTOR_GRID_SIZE_1.
		/// * enableTemporalHints: Optional parameter. Flag to enable temporal hints. When set to true, the hardware uses the flow vectors
		///                            generated in previous call to calc() as internal hints for the current call to calc().
		///                            Useful when computing flow vectors between successive video frames. Defaults to false.
		/// * enableExternalHints: Optional Parameter. Flag to enable passing external hints buffer to calc(). Defaults to false.
		/// * enableCostBuffer: Optional Parameter. Flag to enable cost buffer output from calc(). Defaults to false.
		/// * gpuId: Optional parameter to select the GPU ID on which the optical flow should be computed. Useful in multi-GPU systems. Defaults to 0.
		/// * inputStream: Optical flow algorithm may optionally involve cuda preprocessing on the input buffers.
		///                    The input cuda stream can be used to pipeline and synchronize the cuda preprocessing tasks with OF HW engine.
		///                    If input stream is not set, the execute function will use default stream which is NULL stream;
		/// * outputStream: Optical flow algorithm may optionally involve cuda post processing on the output flow vectors.
		///                    The output cuda stream can be used to pipeline and synchronize the cuda post processing tasks with OF HW engine.
		///                    If output stream is not set, the execute function will use default stream which is NULL stream;
		/// 
		/// ## C++ default parameters
		/// * perf_preset: cv::cuda::NvidiaOpticalFlow_2_0::NV_OF_PERF_LEVEL_SLOW
		/// * output_grid_size: cv::cuda::NvidiaOpticalFlow_2_0::NV_OF_OUTPUT_VECTOR_GRID_SIZE_1
		/// * hint_grid_size: cv::cuda::NvidiaOpticalFlow_2_0::NV_OF_HINT_VECTOR_GRID_SIZE_1
		/// * enable_temporal_hints: false
		/// * enable_external_hints: false
		/// * enable_cost_buffer: false
		/// * gpu_id: 0
		/// * input_stream: Stream::Null()
		/// * output_stream: Stream::Null()
		#[inline]
		pub fn create_1(image_size: core::Size, mut roi_data: core::Vector<core::Rect>, perf_preset: crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0_NVIDIA_OF_PERF_LEVEL, output_grid_size: crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0_NVIDIA_OF_OUTPUT_VECTOR_GRID_SIZE, hint_grid_size: crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0_NVIDIA_OF_HINT_VECTOR_GRID_SIZE, enable_temporal_hints: bool, enable_external_hints: bool, enable_cost_buffer: bool, gpu_id: i32, input_stream: &mut core::Stream, output_stream: &mut core::Stream) -> Result<core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_NvidiaOpticalFlow_2_0_create_Size_vectorLRectG_NVIDIA_OF_PERF_LEVEL_NVIDIA_OF_OUTPUT_VECTOR_GRID_SIZE_NVIDIA_OF_HINT_VECTOR_GRID_SIZE_bool_bool_bool_int_StreamR_StreamR(image_size.opencv_as_extern(), roi_data.as_raw_mut_VectorOfRect(), perf_preset, output_grid_size, hint_grid_size, enable_temporal_hints, enable_external_hints, enable_cost_buffer, gpu_id, input_stream.as_raw_mut_Stream(), output_stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Instantiate NVIDIA Optical Flow with ROI Feature
		/// 
		/// ## Parameters
		/// * imageSize: Size of input image in pixels.
		/// * roiData: Pointer to ROI data.
		/// * perfPreset: Optional parameter. Refer [NV OF SDK documentation](https://developer.nvidia.com/opticalflow-sdk) for details about presets.
		///                   Defaults to NV_OF_PERF_LEVEL_SLOW.
		/// * outputGridSize: Optional parameter. Refer [NV OF SDK documentation](https://developer.nvidia.com/opticalflow-sdk) for details about output grid sizes.
		///                       Defaults to NV_OF_OUTPUT_VECTOR_GRID_SIZE_1.
		/// * hintGridSize: Optional parameter. Refer [NV OF SDK documentation](https://developer.nvidia.com/opticalflow-sdk) for details about hint grid sizes.
		///                    Defaults to NV_OF_HINT_VECTOR_GRID_SIZE_1.
		/// * enableTemporalHints: Optional parameter. Flag to enable temporal hints. When set to true, the hardware uses the flow vectors
		///                            generated in previous call to calc() as internal hints for the current call to calc().
		///                            Useful when computing flow vectors between successive video frames. Defaults to false.
		/// * enableExternalHints: Optional Parameter. Flag to enable passing external hints buffer to calc(). Defaults to false.
		/// * enableCostBuffer: Optional Parameter. Flag to enable cost buffer output from calc(). Defaults to false.
		/// * gpuId: Optional parameter to select the GPU ID on which the optical flow should be computed. Useful in multi-GPU systems. Defaults to 0.
		/// * inputStream: Optical flow algorithm may optionally involve cuda preprocessing on the input buffers.
		///                    The input cuda stream can be used to pipeline and synchronize the cuda preprocessing tasks with OF HW engine.
		///                    If input stream is not set, the execute function will use default stream which is NULL stream;
		/// * outputStream: Optical flow algorithm may optionally involve cuda post processing on the output flow vectors.
		///                    The output cuda stream can be used to pipeline and synchronize the cuda post processing tasks with OF HW engine.
		///                    If output stream is not set, the execute function will use default stream which is NULL stream;
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * perf_preset: cv::cuda::NvidiaOpticalFlow_2_0::NV_OF_PERF_LEVEL_SLOW
		/// * output_grid_size: cv::cuda::NvidiaOpticalFlow_2_0::NV_OF_OUTPUT_VECTOR_GRID_SIZE_1
		/// * hint_grid_size: cv::cuda::NvidiaOpticalFlow_2_0::NV_OF_HINT_VECTOR_GRID_SIZE_1
		/// * enable_temporal_hints: false
		/// * enable_external_hints: false
		/// * enable_cost_buffer: false
		/// * gpu_id: 0
		/// * input_stream: Stream::Null()
		/// * output_stream: Stream::Null()
		#[inline]
		pub fn create_def_1(image_size: core::Size, mut roi_data: core::Vector<core::Rect>) -> Result<core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_NvidiaOpticalFlow_2_0_create_Size_vectorLRectG(image_size.opencv_as_extern(), roi_data.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { CUDA_NvidiaOpticalFlow_2_0, core::Algorithm, cv_cuda_NvidiaOpticalFlow_2_0_to_Algorithm }
	
	boxed_cast_base! { CUDA_NvidiaOpticalFlow_2_0, crate::cudaoptflow::CUDA_NvidiaHWOpticalFlow, cv_cuda_NvidiaOpticalFlow_2_0_to_CUDA_NvidiaHWOpticalFlow }
	
	impl std::fmt::Debug for CUDA_NvidiaOpticalFlow_2_0 {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_NvidiaOpticalFlow_2_0")
				.finish()
		}
	}
	
	/// Constant methods for [crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1]
	pub trait CUDA_OpticalFlowDual_TVL1TraitConst: crate::cudaoptflow::CUDA_DenseOpticalFlowTraitConst {
		fn as_raw_CUDA_OpticalFlowDual_TVL1(&self) -> *const c_void;
	
		/// Time step of the numerical scheme.
		#[inline]
		fn get_tau(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_getTau_const(self.as_raw_CUDA_OpticalFlowDual_TVL1(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Weight parameter for the data term, attachment parameter.
		/// This is the most relevant parameter, which determines the smoothness of the output.
		/// The smaller this parameter is, the smoother the solutions we obtain.
		/// It depends on the range of motions of the images, so its value should be adapted to each image sequence.
		#[inline]
		fn get_lambda(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_getLambda_const(self.as_raw_CUDA_OpticalFlowDual_TVL1(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Weight parameter for (u - v)^2, tightness parameter.
		/// It serves as a link between the attachment and the regularization terms.
		/// In theory, it should have a small value in order to maintain both parts in correspondence.
		/// The method is stable for a large range of values of this parameter.
		#[inline]
		fn get_gamma(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_getGamma_const(self.as_raw_CUDA_OpticalFlowDual_TVL1(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// parameter used for motion estimation. It adds a variable allowing for illumination variations
		/// Set this parameter to 1. if you have varying illumination.
		/// See: Chambolle et al, A First-Order Primal-Dual Algorithm for Convex Problems with Applications to Imaging
		/// Journal of Mathematical imaging and vision, may 2011 Vol 40 issue 1, pp 120-145
		#[inline]
		fn get_theta(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_getTheta_const(self.as_raw_CUDA_OpticalFlowDual_TVL1(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Number of scales used to create the pyramid of images.
		#[inline]
		fn get_num_scales(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_getNumScales_const(self.as_raw_CUDA_OpticalFlowDual_TVL1(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Number of warpings per scale.
		/// Represents the number of times that I1(x+u0) and grad( I1(x+u0) ) are computed per scale.
		/// This is a parameter that assures the stability of the method.
		/// It also affects the running time, so it is a compromise between speed and accuracy.
		#[inline]
		fn get_num_warps(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_getNumWarps_const(self.as_raw_CUDA_OpticalFlowDual_TVL1(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Stopping criterion threshold used in the numerical scheme, which is a trade-off between precision and running time.
		/// A small value will yield more accurate solutions at the expense of a slower convergence.
		#[inline]
		fn get_epsilon(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_getEpsilon_const(self.as_raw_CUDA_OpticalFlowDual_TVL1(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Stopping criterion iterations number used in the numerical scheme.
		#[inline]
		fn get_num_iterations(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_getNumIterations_const(self.as_raw_CUDA_OpticalFlowDual_TVL1(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_scale_step(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_getScaleStep_const(self.as_raw_CUDA_OpticalFlowDual_TVL1(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_use_initial_flow(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_getUseInitialFlow_const(self.as_raw_CUDA_OpticalFlowDual_TVL1(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1]
	pub trait CUDA_OpticalFlowDual_TVL1Trait: crate::cudaoptflow::CUDA_DenseOpticalFlowTrait + crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1TraitConst {
		fn as_raw_mut_CUDA_OpticalFlowDual_TVL1(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_tau(&mut self, tau: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_setTau_double(self.as_raw_mut_CUDA_OpticalFlowDual_TVL1(), tau, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_lambda(&mut self, lambda: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_setLambda_double(self.as_raw_mut_CUDA_OpticalFlowDual_TVL1(), lambda, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_gamma(&mut self, gamma: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_setGamma_double(self.as_raw_mut_CUDA_OpticalFlowDual_TVL1(), gamma, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_theta(&mut self, theta: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_setTheta_double(self.as_raw_mut_CUDA_OpticalFlowDual_TVL1(), theta, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_num_scales(&mut self, nscales: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_setNumScales_int(self.as_raw_mut_CUDA_OpticalFlowDual_TVL1(), nscales, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_num_warps(&mut self, warps: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_setNumWarps_int(self.as_raw_mut_CUDA_OpticalFlowDual_TVL1(), warps, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_epsilon(&mut self, epsilon: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_setEpsilon_double(self.as_raw_mut_CUDA_OpticalFlowDual_TVL1(), epsilon, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_num_iterations(&mut self, iterations: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_setNumIterations_int(self.as_raw_mut_CUDA_OpticalFlowDual_TVL1(), iterations, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_scale_step(&mut self, scale_step: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_setScaleStep_double(self.as_raw_mut_CUDA_OpticalFlowDual_TVL1(), scale_step, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_use_initial_flow(&mut self, use_initial_flow: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_setUseInitialFlow_bool(self.as_raw_mut_CUDA_OpticalFlowDual_TVL1(), use_initial_flow, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Implementation of the Zach, Pock and Bischof Dual TV-L1 Optical Flow method.
	/// 
	/// 
	/// Note: C. Zach, T. Pock and H. Bischof, "A Duality Based Approach for Realtime TV-L1 Optical Flow".
	/// 
	/// Note: Javier Sanchez, Enric Meinhardt-Llopis and Gabriele Facciolo. "TV-L1 Optical Flow Estimation".
	pub struct CUDA_OpticalFlowDual_TVL1 {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CUDA_OpticalFlowDual_TVL1 }
	
	impl Drop for CUDA_OpticalFlowDual_TVL1 {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_delete(self.as_raw_mut_CUDA_OpticalFlowDual_TVL1()) };
		}
	}
	
	unsafe impl Send for CUDA_OpticalFlowDual_TVL1 {}
	
	impl core::AlgorithmTraitConst for CUDA_OpticalFlowDual_TVL1 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CUDA_OpticalFlowDual_TVL1 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlowTraitConst for CUDA_OpticalFlowDual_TVL1 {
		#[inline] fn as_raw_CUDA_DenseOpticalFlow(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlowTrait for CUDA_OpticalFlowDual_TVL1 {
		#[inline] fn as_raw_mut_CUDA_DenseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1TraitConst for CUDA_OpticalFlowDual_TVL1 {
		#[inline] fn as_raw_CUDA_OpticalFlowDual_TVL1(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1Trait for CUDA_OpticalFlowDual_TVL1 {
		#[inline] fn as_raw_mut_CUDA_OpticalFlowDual_TVL1(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CUDA_OpticalFlowDual_TVL1 {
		/// ## C++ default parameters
		/// * tau: 0.25
		/// * lambda: 0.15
		/// * theta: 0.3
		/// * nscales: 5
		/// * warps: 5
		/// * epsilon: 0.01
		/// * iterations: 300
		/// * scale_step: 0.8
		/// * gamma: 0.0
		/// * use_initial_flow: false
		#[inline]
		pub fn create(tau: f64, lambda: f64, theta: f64, nscales: i32, warps: i32, epsilon: f64, iterations: i32, scale_step: f64, gamma: f64, use_initial_flow: bool) -> Result<core::Ptr<crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_create_double_double_double_int_int_double_int_double_double_bool(tau, lambda, theta, nscales, warps, epsilon, iterations, scale_step, gamma, use_initial_flow, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * tau: 0.25
		/// * lambda: 0.15
		/// * theta: 0.3
		/// * nscales: 5
		/// * warps: 5
		/// * epsilon: 0.01
		/// * iterations: 300
		/// * scale_step: 0.8
		/// * gamma: 0.0
		/// * use_initial_flow: false
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { CUDA_OpticalFlowDual_TVL1, core::Algorithm, cv_cuda_OpticalFlowDual_TVL1_to_Algorithm }
	
	boxed_cast_base! { CUDA_OpticalFlowDual_TVL1, crate::cudaoptflow::CUDA_DenseOpticalFlow, cv_cuda_OpticalFlowDual_TVL1_to_CUDA_DenseOpticalFlow }
	
	impl std::fmt::Debug for CUDA_OpticalFlowDual_TVL1 {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_OpticalFlowDual_TVL1")
				.finish()
		}
	}
	
	/// Constant methods for [crate::cudaoptflow::CUDA_SparseOpticalFlow]
	pub trait CUDA_SparseOpticalFlowTraitConst: core::AlgorithmTraitConst {
		fn as_raw_CUDA_SparseOpticalFlow(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::cudaoptflow::CUDA_SparseOpticalFlow]
	pub trait CUDA_SparseOpticalFlowTrait: core::AlgorithmTrait + crate::cudaoptflow::CUDA_SparseOpticalFlowTraitConst {
		fn as_raw_mut_CUDA_SparseOpticalFlow(&mut self) -> *mut c_void;
	
		/// Calculates a sparse optical flow.
		/// 
		/// ## Parameters
		/// * prevImg: First input image.
		/// * nextImg: Second input image of the same size and the same type as prevImg.
		/// * prevPts: Vector of 2D points for which the flow needs to be found.
		/// * nextPts: Output vector of 2D points containing the calculated new positions of input features in the second image.
		/// * status: Output status vector. Each element of the vector is set to 1 if the
		///               flow for the corresponding features has been found. Otherwise, it is set to 0.
		/// * err: Optional output vector that contains error response for each point (inverse confidence).
		/// * stream: Stream for the asynchronous version.
		/// 
		/// ## C++ default parameters
		/// * err: cv::noArray()
		/// * stream: Stream::Null()
		#[inline]
		fn calc(&mut self, prev_img: &impl core::ToInputArray, next_img: &impl core::ToInputArray, prev_pts: &impl core::ToInputArray, next_pts: &mut impl core::ToInputOutputArray, status: &mut impl core::ToOutputArray, err: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
			input_array_arg!(prev_img);
			input_array_arg!(next_img);
			input_array_arg!(prev_pts);
			input_output_array_arg!(next_pts);
			output_array_arg!(status);
			output_array_arg!(err);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_SparseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_SparseOpticalFlow(), prev_img.as_raw__InputArray(), next_img.as_raw__InputArray(), prev_pts.as_raw__InputArray(), next_pts.as_raw__InputOutputArray(), status.as_raw__OutputArray(), err.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Calculates a sparse optical flow.
		/// 
		/// ## Parameters
		/// * prevImg: First input image.
		/// * nextImg: Second input image of the same size and the same type as prevImg.
		/// * prevPts: Vector of 2D points for which the flow needs to be found.
		/// * nextPts: Output vector of 2D points containing the calculated new positions of input features in the second image.
		/// * status: Output status vector. Each element of the vector is set to 1 if the
		///               flow for the corresponding features has been found. Otherwise, it is set to 0.
		/// * err: Optional output vector that contains error response for each point (inverse confidence).
		/// * stream: Stream for the asynchronous version.
		/// 
		/// ## Note
		/// This alternative version of [calc] function uses the following default values for its arguments:
		/// * err: cv::noArray()
		/// * stream: Stream::Null()
		#[inline]
		fn calc_def(&mut self, prev_img: &impl core::ToInputArray, next_img: &impl core::ToInputArray, prev_pts: &impl core::ToInputArray, next_pts: &mut impl core::ToInputOutputArray, status: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(prev_img);
			input_array_arg!(next_img);
			input_array_arg!(prev_pts);
			input_output_array_arg!(next_pts);
			output_array_arg!(status);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_SparseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR(self.as_raw_mut_CUDA_SparseOpticalFlow(), prev_img.as_raw__InputArray(), next_img.as_raw__InputArray(), prev_pts.as_raw__InputArray(), next_pts.as_raw__InputOutputArray(), status.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Base interface for sparse optical flow algorithms.
	pub struct CUDA_SparseOpticalFlow {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CUDA_SparseOpticalFlow }
	
	impl Drop for CUDA_SparseOpticalFlow {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_SparseOpticalFlow_delete(self.as_raw_mut_CUDA_SparseOpticalFlow()) };
		}
	}
	
	unsafe impl Send for CUDA_SparseOpticalFlow {}
	
	impl core::AlgorithmTraitConst for CUDA_SparseOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CUDA_SparseOpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_SparseOpticalFlowTraitConst for CUDA_SparseOpticalFlow {
		#[inline] fn as_raw_CUDA_SparseOpticalFlow(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_SparseOpticalFlowTrait for CUDA_SparseOpticalFlow {
		#[inline] fn as_raw_mut_CUDA_SparseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CUDA_SparseOpticalFlow {
	}
	
	boxed_cast_descendant! { CUDA_SparseOpticalFlow, crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow, cv_cuda_SparseOpticalFlow_to_CUDA_SparsePyrLKOpticalFlow }
	
	boxed_cast_base! { CUDA_SparseOpticalFlow, core::Algorithm, cv_cuda_SparseOpticalFlow_to_Algorithm }
	
	impl std::fmt::Debug for CUDA_SparseOpticalFlow {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_SparseOpticalFlow")
				.finish()
		}
	}
	
	/// Constant methods for [crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow]
	pub trait CUDA_SparsePyrLKOpticalFlowTraitConst: crate::cudaoptflow::CUDA_SparseOpticalFlowTraitConst {
		fn as_raw_CUDA_SparsePyrLKOpticalFlow(&self) -> *const c_void;
	
		#[inline]
		fn get_win_size(&self) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_SparsePyrLKOpticalFlow_getWinSize_const(self.as_raw_CUDA_SparsePyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_max_level(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_SparsePyrLKOpticalFlow_getMaxLevel_const(self.as_raw_CUDA_SparsePyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_num_iters(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_SparsePyrLKOpticalFlow_getNumIters_const(self.as_raw_CUDA_SparsePyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_use_initial_flow(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_SparsePyrLKOpticalFlow_getUseInitialFlow_const(self.as_raw_CUDA_SparsePyrLKOpticalFlow(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow]
	pub trait CUDA_SparsePyrLKOpticalFlowTrait: crate::cudaoptflow::CUDA_SparseOpticalFlowTrait + crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlowTraitConst {
		fn as_raw_mut_CUDA_SparsePyrLKOpticalFlow(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_win_size(&mut self, win_size: core::Size) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_SparsePyrLKOpticalFlow_setWinSize_Size(self.as_raw_mut_CUDA_SparsePyrLKOpticalFlow(), win_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_max_level(&mut self, max_level: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_SparsePyrLKOpticalFlow_setMaxLevel_int(self.as_raw_mut_CUDA_SparsePyrLKOpticalFlow(), max_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_num_iters(&mut self, iters: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_SparsePyrLKOpticalFlow_setNumIters_int(self.as_raw_mut_CUDA_SparsePyrLKOpticalFlow(), iters, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_use_initial_flow(&mut self, use_initial_flow: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_SparsePyrLKOpticalFlow_setUseInitialFlow_bool(self.as_raw_mut_CUDA_SparsePyrLKOpticalFlow(), use_initial_flow, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Class used for calculating a sparse optical flow.
	/// 
	/// The class can calculate an optical flow for a sparse feature set using the
	/// iterative Lucas-Kanade method with pyramids.
	/// ## See also
	/// calcOpticalFlowPyrLK
	/// 
	/// 
	/// Note:
	///    *   An example of the Lucas Kanade optical flow algorithm can be found at
	///        opencv_source_code/samples/gpu/pyrlk_optical_flow.cpp
	pub struct CUDA_SparsePyrLKOpticalFlow {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CUDA_SparsePyrLKOpticalFlow }
	
	impl Drop for CUDA_SparsePyrLKOpticalFlow {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_SparsePyrLKOpticalFlow_delete(self.as_raw_mut_CUDA_SparsePyrLKOpticalFlow()) };
		}
	}
	
	unsafe impl Send for CUDA_SparsePyrLKOpticalFlow {}
	
	impl core::AlgorithmTraitConst for CUDA_SparsePyrLKOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for CUDA_SparsePyrLKOpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_SparseOpticalFlowTraitConst for CUDA_SparsePyrLKOpticalFlow {
		#[inline] fn as_raw_CUDA_SparseOpticalFlow(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_SparseOpticalFlowTrait for CUDA_SparsePyrLKOpticalFlow {
		#[inline] fn as_raw_mut_CUDA_SparseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlowTraitConst for CUDA_SparsePyrLKOpticalFlow {
		#[inline] fn as_raw_CUDA_SparsePyrLKOpticalFlow(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlowTrait for CUDA_SparsePyrLKOpticalFlow {
		#[inline] fn as_raw_mut_CUDA_SparsePyrLKOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CUDA_SparsePyrLKOpticalFlow {
		/// ## C++ default parameters
		/// * win_size: Size(21,21)
		/// * max_level: 3
		/// * iters: 30
		/// * use_initial_flow: false
		#[inline]
		pub fn create(win_size: core::Size, max_level: i32, iters: i32, use_initial_flow: bool) -> Result<core::Ptr<crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_SparsePyrLKOpticalFlow_create_Size_int_int_bool(win_size.opencv_as_extern(), max_level, iters, use_initial_flow, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * win_size: Size(21,21)
		/// * max_level: 3
		/// * iters: 30
		/// * use_initial_flow: false
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_SparsePyrLKOpticalFlow_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { CUDA_SparsePyrLKOpticalFlow, core::Algorithm, cv_cuda_SparsePyrLKOpticalFlow_to_Algorithm }
	
	boxed_cast_base! { CUDA_SparsePyrLKOpticalFlow, crate::cudaoptflow::CUDA_SparseOpticalFlow, cv_cuda_SparsePyrLKOpticalFlow_to_CUDA_SparseOpticalFlow }
	
	impl std::fmt::Debug for CUDA_SparsePyrLKOpticalFlow {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_SparsePyrLKOpticalFlow")
				.finish()
		}
	}
}
