#![allow(unused_parens)]
//! # Optical Flow
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::CUDA_DenseOpticalFlow, super::CUDA_SparseOpticalFlow, super::CUDA_BroxOpticalFlow, super::CUDA_SparsePyrLKOpticalFlow, super::CUDA_DensePyrLKOpticalFlow, super::CUDA_FarnebackOpticalFlow, super::CUDA_OpticalFlowDual_TVL1 };
}

/// Class computing the optical flow for two images using Brox et al Optical Flow algorithm ([Brox2004](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_Brox2004)).
pub trait CUDA_BroxOpticalFlow: crate::cudaoptflow::CUDA_DenseOpticalFlow {
	fn as_raw_CUDA_BroxOpticalFlow(&self) -> *const c_void;
	fn as_raw_mut_CUDA_BroxOpticalFlow(&mut self) -> *mut c_void;

	fn get_flow_smoothness(&self) -> Result<f64> {
		unsafe { sys::cv_cuda_BroxOpticalFlow_getFlowSmoothness_const(self.as_raw_CUDA_BroxOpticalFlow()) }.into_result()
	}
	
	fn set_flow_smoothness(&mut self, alpha: f64) -> Result<()> {
		unsafe { sys::cv_cuda_BroxOpticalFlow_setFlowSmoothness_double(self.as_raw_mut_CUDA_BroxOpticalFlow(), alpha) }.into_result()
	}
	
	fn get_gradient_constancy_importance(&self) -> Result<f64> {
		unsafe { sys::cv_cuda_BroxOpticalFlow_getGradientConstancyImportance_const(self.as_raw_CUDA_BroxOpticalFlow()) }.into_result()
	}
	
	fn set_gradient_constancy_importance(&mut self, gamma: f64) -> Result<()> {
		unsafe { sys::cv_cuda_BroxOpticalFlow_setGradientConstancyImportance_double(self.as_raw_mut_CUDA_BroxOpticalFlow(), gamma) }.into_result()
	}
	
	fn get_pyramid_scale_factor(&self) -> Result<f64> {
		unsafe { sys::cv_cuda_BroxOpticalFlow_getPyramidScaleFactor_const(self.as_raw_CUDA_BroxOpticalFlow()) }.into_result()
	}
	
	fn set_pyramid_scale_factor(&mut self, scale_factor: f64) -> Result<()> {
		unsafe { sys::cv_cuda_BroxOpticalFlow_setPyramidScaleFactor_double(self.as_raw_mut_CUDA_BroxOpticalFlow(), scale_factor) }.into_result()
	}
	
	/// number of lagged non-linearity iterations (inner loop)
	fn get_inner_iterations(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_BroxOpticalFlow_getInnerIterations_const(self.as_raw_CUDA_BroxOpticalFlow()) }.into_result()
	}
	
	fn set_inner_iterations(&mut self, inner_iterations: i32) -> Result<()> {
		unsafe { sys::cv_cuda_BroxOpticalFlow_setInnerIterations_int(self.as_raw_mut_CUDA_BroxOpticalFlow(), inner_iterations) }.into_result()
	}
	
	/// number of warping iterations (number of pyramid levels)
	fn get_outer_iterations(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_BroxOpticalFlow_getOuterIterations_const(self.as_raw_CUDA_BroxOpticalFlow()) }.into_result()
	}
	
	fn set_outer_iterations(&mut self, outer_iterations: i32) -> Result<()> {
		unsafe { sys::cv_cuda_BroxOpticalFlow_setOuterIterations_int(self.as_raw_mut_CUDA_BroxOpticalFlow(), outer_iterations) }.into_result()
	}
	
	/// number of linear system solver iterations
	fn get_solver_iterations(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_BroxOpticalFlow_getSolverIterations_const(self.as_raw_CUDA_BroxOpticalFlow()) }.into_result()
	}
	
	fn set_solver_iterations(&mut self, solver_iterations: i32) -> Result<()> {
		unsafe { sys::cv_cuda_BroxOpticalFlow_setSolverIterations_int(self.as_raw_mut_CUDA_BroxOpticalFlow(), solver_iterations) }.into_result()
	}
	
}

impl dyn CUDA_BroxOpticalFlow + '_ {
	/// ## C++ default parameters
	/// * alpha: 0.197
	/// * gamma: 50.0
	/// * scale_factor: 0.8
	/// * inner_iterations: 5
	/// * outer_iterations: 150
	/// * solver_iterations: 10
	pub fn create(alpha: f64, gamma: f64, scale_factor: f64, inner_iterations: i32, outer_iterations: i32, solver_iterations: i32) -> Result<core::Ptr::<dyn crate::cudaoptflow::CUDA_BroxOpticalFlow>> {
		unsafe { sys::cv_cuda_BroxOpticalFlow_create_double_double_double_int_int_int(alpha, gamma, scale_factor, inner_iterations, outer_iterations, solver_iterations) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudaoptflow::CUDA_BroxOpticalFlow>::opencv_from_extern(r) } )
	}
	
}
/// Base interface for dense optical flow algorithms.
pub trait CUDA_DenseOpticalFlow: core::AlgorithmTrait {
	fn as_raw_CUDA_DenseOpticalFlow(&self) -> *const c_void;
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
	fn calc(&mut self, i0: &dyn core::ToInputArray, i1: &dyn core::ToInputArray, flow: &mut dyn core::ToInputOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(i0);
		input_array_arg!(i1);
		input_output_array_arg!(flow);
		unsafe { sys::cv_cuda_DenseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_StreamR(self.as_raw_mut_CUDA_DenseOpticalFlow(), i0.as_raw__InputArray(), i1.as_raw__InputArray(), flow.as_raw__InputOutputArray(), stream.as_raw_mut_Stream()) }.into_result()
	}
	
}

/// Class used for calculating a dense optical flow.
/// 
/// The class can calculate an optical flow for a dense optical flow using the
/// iterative Lucas-Kanade method with pyramids.
pub trait CUDA_DensePyrLKOpticalFlow: crate::cudaoptflow::CUDA_DenseOpticalFlow {
	fn as_raw_CUDA_DensePyrLKOpticalFlow(&self) -> *const c_void;
	fn as_raw_mut_CUDA_DensePyrLKOpticalFlow(&mut self) -> *mut c_void;

	fn get_win_size(&self) -> Result<core::Size> {
		unsafe { sys::cv_cuda_DensePyrLKOpticalFlow_getWinSize_const(self.as_raw_CUDA_DensePyrLKOpticalFlow()) }.into_result()
	}
	
	fn set_win_size(&mut self, win_size: core::Size) -> Result<()> {
		unsafe { sys::cv_cuda_DensePyrLKOpticalFlow_setWinSize_Size(self.as_raw_mut_CUDA_DensePyrLKOpticalFlow(), win_size.opencv_to_extern()) }.into_result()
	}
	
	fn get_max_level(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DensePyrLKOpticalFlow_getMaxLevel_const(self.as_raw_CUDA_DensePyrLKOpticalFlow()) }.into_result()
	}
	
	fn set_max_level(&mut self, max_level: i32) -> Result<()> {
		unsafe { sys::cv_cuda_DensePyrLKOpticalFlow_setMaxLevel_int(self.as_raw_mut_CUDA_DensePyrLKOpticalFlow(), max_level) }.into_result()
	}
	
	fn get_num_iters(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_DensePyrLKOpticalFlow_getNumIters_const(self.as_raw_CUDA_DensePyrLKOpticalFlow()) }.into_result()
	}
	
	fn set_num_iters(&mut self, iters: i32) -> Result<()> {
		unsafe { sys::cv_cuda_DensePyrLKOpticalFlow_setNumIters_int(self.as_raw_mut_CUDA_DensePyrLKOpticalFlow(), iters) }.into_result()
	}
	
	fn get_use_initial_flow(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_DensePyrLKOpticalFlow_getUseInitialFlow_const(self.as_raw_CUDA_DensePyrLKOpticalFlow()) }.into_result()
	}
	
	fn set_use_initial_flow(&mut self, use_initial_flow: bool) -> Result<()> {
		unsafe { sys::cv_cuda_DensePyrLKOpticalFlow_setUseInitialFlow_bool(self.as_raw_mut_CUDA_DensePyrLKOpticalFlow(), use_initial_flow) }.into_result()
	}
	
}

impl dyn CUDA_DensePyrLKOpticalFlow + '_ {
	/// ## C++ default parameters
	/// * win_size: Size(13,13)
	/// * max_level: 3
	/// * iters: 30
	/// * use_initial_flow: false
	pub fn create(win_size: core::Size, max_level: i32, iters: i32, use_initial_flow: bool) -> Result<core::Ptr::<dyn crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow>> {
		unsafe { sys::cv_cuda_DensePyrLKOpticalFlow_create_Size_int_int_bool(win_size.opencv_to_extern(), max_level, iters, use_initial_flow) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow>::opencv_from_extern(r) } )
	}
	
}
/// Class computing a dense optical flow using the Gunnar Farneback's algorithm.
pub trait CUDA_FarnebackOpticalFlow: crate::cudaoptflow::CUDA_DenseOpticalFlow {
	fn as_raw_CUDA_FarnebackOpticalFlow(&self) -> *const c_void;
	fn as_raw_mut_CUDA_FarnebackOpticalFlow(&mut self) -> *mut c_void;

	fn get_num_levels(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_FarnebackOpticalFlow_getNumLevels_const(self.as_raw_CUDA_FarnebackOpticalFlow()) }.into_result()
	}
	
	fn set_num_levels(&mut self, num_levels: i32) -> Result<()> {
		unsafe { sys::cv_cuda_FarnebackOpticalFlow_setNumLevels_int(self.as_raw_mut_CUDA_FarnebackOpticalFlow(), num_levels) }.into_result()
	}
	
	fn get_pyr_scale(&self) -> Result<f64> {
		unsafe { sys::cv_cuda_FarnebackOpticalFlow_getPyrScale_const(self.as_raw_CUDA_FarnebackOpticalFlow()) }.into_result()
	}
	
	fn set_pyr_scale(&mut self, pyr_scale: f64) -> Result<()> {
		unsafe { sys::cv_cuda_FarnebackOpticalFlow_setPyrScale_double(self.as_raw_mut_CUDA_FarnebackOpticalFlow(), pyr_scale) }.into_result()
	}
	
	fn get_fast_pyramids(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_FarnebackOpticalFlow_getFastPyramids_const(self.as_raw_CUDA_FarnebackOpticalFlow()) }.into_result()
	}
	
	fn set_fast_pyramids(&mut self, fast_pyramids: bool) -> Result<()> {
		unsafe { sys::cv_cuda_FarnebackOpticalFlow_setFastPyramids_bool(self.as_raw_mut_CUDA_FarnebackOpticalFlow(), fast_pyramids) }.into_result()
	}
	
	fn get_win_size(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_FarnebackOpticalFlow_getWinSize_const(self.as_raw_CUDA_FarnebackOpticalFlow()) }.into_result()
	}
	
	fn set_win_size(&mut self, win_size: i32) -> Result<()> {
		unsafe { sys::cv_cuda_FarnebackOpticalFlow_setWinSize_int(self.as_raw_mut_CUDA_FarnebackOpticalFlow(), win_size) }.into_result()
	}
	
	fn get_num_iters(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_FarnebackOpticalFlow_getNumIters_const(self.as_raw_CUDA_FarnebackOpticalFlow()) }.into_result()
	}
	
	fn set_num_iters(&mut self, num_iters: i32) -> Result<()> {
		unsafe { sys::cv_cuda_FarnebackOpticalFlow_setNumIters_int(self.as_raw_mut_CUDA_FarnebackOpticalFlow(), num_iters) }.into_result()
	}
	
	fn get_poly_n(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_FarnebackOpticalFlow_getPolyN_const(self.as_raw_CUDA_FarnebackOpticalFlow()) }.into_result()
	}
	
	fn set_poly_n(&mut self, poly_n: i32) -> Result<()> {
		unsafe { sys::cv_cuda_FarnebackOpticalFlow_setPolyN_int(self.as_raw_mut_CUDA_FarnebackOpticalFlow(), poly_n) }.into_result()
	}
	
	fn get_poly_sigma(&self) -> Result<f64> {
		unsafe { sys::cv_cuda_FarnebackOpticalFlow_getPolySigma_const(self.as_raw_CUDA_FarnebackOpticalFlow()) }.into_result()
	}
	
	fn set_poly_sigma(&mut self, poly_sigma: f64) -> Result<()> {
		unsafe { sys::cv_cuda_FarnebackOpticalFlow_setPolySigma_double(self.as_raw_mut_CUDA_FarnebackOpticalFlow(), poly_sigma) }.into_result()
	}
	
	fn get_flags(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_FarnebackOpticalFlow_getFlags_const(self.as_raw_CUDA_FarnebackOpticalFlow()) }.into_result()
	}
	
	fn set_flags(&mut self, flags: i32) -> Result<()> {
		unsafe { sys::cv_cuda_FarnebackOpticalFlow_setFlags_int(self.as_raw_mut_CUDA_FarnebackOpticalFlow(), flags) }.into_result()
	}
	
}

impl dyn CUDA_FarnebackOpticalFlow + '_ {
	/// ## C++ default parameters
	/// * num_levels: 5
	/// * pyr_scale: 0.5
	/// * fast_pyramids: false
	/// * win_size: 13
	/// * num_iters: 10
	/// * poly_n: 5
	/// * poly_sigma: 1.1
	/// * flags: 0
	pub fn create(num_levels: i32, pyr_scale: f64, fast_pyramids: bool, win_size: i32, num_iters: i32, poly_n: i32, poly_sigma: f64, flags: i32) -> Result<core::Ptr::<dyn crate::cudaoptflow::CUDA_FarnebackOpticalFlow>> {
		unsafe { sys::cv_cuda_FarnebackOpticalFlow_create_int_double_bool_int_int_int_double_int(num_levels, pyr_scale, fast_pyramids, win_size, num_iters, poly_n, poly_sigma, flags) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudaoptflow::CUDA_FarnebackOpticalFlow>::opencv_from_extern(r) } )
	}
	
}
/// Implementation of the Zach, Pock and Bischof Dual TV-L1 Optical Flow method.
/// 
/// 
/// Note: C. Zach, T. Pock and H. Bischof, "A Duality Based Approach for Realtime TV-L1 Optical Flow".
/// 
/// Note: Javier Sanchez, Enric Meinhardt-Llopis and Gabriele Facciolo. "TV-L1 Optical Flow Estimation".
pub trait CUDA_OpticalFlowDual_TVL1: crate::cudaoptflow::CUDA_DenseOpticalFlow {
	fn as_raw_CUDA_OpticalFlowDual_TVL1(&self) -> *const c_void;
	fn as_raw_mut_CUDA_OpticalFlowDual_TVL1(&mut self) -> *mut c_void;

	/// Time step of the numerical scheme.
	fn get_tau(&self) -> Result<f64> {
		unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_getTau_const(self.as_raw_CUDA_OpticalFlowDual_TVL1()) }.into_result()
	}
	
	fn set_tau(&mut self, tau: f64) -> Result<()> {
		unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_setTau_double(self.as_raw_mut_CUDA_OpticalFlowDual_TVL1(), tau) }.into_result()
	}
	
	/// Weight parameter for the data term, attachment parameter.
	/// This is the most relevant parameter, which determines the smoothness of the output.
	/// The smaller this parameter is, the smoother the solutions we obtain.
	/// It depends on the range of motions of the images, so its value should be adapted to each image sequence.
	fn get_lambda(&self) -> Result<f64> {
		unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_getLambda_const(self.as_raw_CUDA_OpticalFlowDual_TVL1()) }.into_result()
	}
	
	fn set_lambda(&mut self, lambda: f64) -> Result<()> {
		unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_setLambda_double(self.as_raw_mut_CUDA_OpticalFlowDual_TVL1(), lambda) }.into_result()
	}
	
	/// Weight parameter for (u - v)^2, tightness parameter.
	/// It serves as a link between the attachment and the regularization terms.
	/// In theory, it should have a small value in order to maintain both parts in correspondence.
	/// The method is stable for a large range of values of this parameter.
	fn get_gamma(&self) -> Result<f64> {
		unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_getGamma_const(self.as_raw_CUDA_OpticalFlowDual_TVL1()) }.into_result()
	}
	
	fn set_gamma(&mut self, gamma: f64) -> Result<()> {
		unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_setGamma_double(self.as_raw_mut_CUDA_OpticalFlowDual_TVL1(), gamma) }.into_result()
	}
	
	/// parameter used for motion estimation. It adds a variable allowing for illumination variations
	/// Set this parameter to 1. if you have varying illumination.
	/// See: Chambolle et al, A First-Order Primal-Dual Algorithm for Convex Problems with Applications to Imaging
	/// Journal of Mathematical imaging and vision, may 2011 Vol 40 issue 1, pp 120-145
	fn get_theta(&self) -> Result<f64> {
		unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_getTheta_const(self.as_raw_CUDA_OpticalFlowDual_TVL1()) }.into_result()
	}
	
	fn set_theta(&mut self, theta: f64) -> Result<()> {
		unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_setTheta_double(self.as_raw_mut_CUDA_OpticalFlowDual_TVL1(), theta) }.into_result()
	}
	
	/// Number of scales used to create the pyramid of images.
	fn get_num_scales(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_getNumScales_const(self.as_raw_CUDA_OpticalFlowDual_TVL1()) }.into_result()
	}
	
	fn set_num_scales(&mut self, nscales: i32) -> Result<()> {
		unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_setNumScales_int(self.as_raw_mut_CUDA_OpticalFlowDual_TVL1(), nscales) }.into_result()
	}
	
	/// Number of warpings per scale.
	/// Represents the number of times that I1(x+u0) and grad( I1(x+u0) ) are computed per scale.
	/// This is a parameter that assures the stability of the method.
	/// It also affects the running time, so it is a compromise between speed and accuracy.
	fn get_num_warps(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_getNumWarps_const(self.as_raw_CUDA_OpticalFlowDual_TVL1()) }.into_result()
	}
	
	fn set_num_warps(&mut self, warps: i32) -> Result<()> {
		unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_setNumWarps_int(self.as_raw_mut_CUDA_OpticalFlowDual_TVL1(), warps) }.into_result()
	}
	
	/// Stopping criterion threshold used in the numerical scheme, which is a trade-off between precision and running time.
	/// A small value will yield more accurate solutions at the expense of a slower convergence.
	fn get_epsilon(&self) -> Result<f64> {
		unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_getEpsilon_const(self.as_raw_CUDA_OpticalFlowDual_TVL1()) }.into_result()
	}
	
	fn set_epsilon(&mut self, epsilon: f64) -> Result<()> {
		unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_setEpsilon_double(self.as_raw_mut_CUDA_OpticalFlowDual_TVL1(), epsilon) }.into_result()
	}
	
	/// Stopping criterion iterations number used in the numerical scheme.
	fn get_num_iterations(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_getNumIterations_const(self.as_raw_CUDA_OpticalFlowDual_TVL1()) }.into_result()
	}
	
	fn set_num_iterations(&mut self, iterations: i32) -> Result<()> {
		unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_setNumIterations_int(self.as_raw_mut_CUDA_OpticalFlowDual_TVL1(), iterations) }.into_result()
	}
	
	fn get_scale_step(&self) -> Result<f64> {
		unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_getScaleStep_const(self.as_raw_CUDA_OpticalFlowDual_TVL1()) }.into_result()
	}
	
	fn set_scale_step(&mut self, scale_step: f64) -> Result<()> {
		unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_setScaleStep_double(self.as_raw_mut_CUDA_OpticalFlowDual_TVL1(), scale_step) }.into_result()
	}
	
	fn get_use_initial_flow(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_getUseInitialFlow_const(self.as_raw_CUDA_OpticalFlowDual_TVL1()) }.into_result()
	}
	
	fn set_use_initial_flow(&mut self, use_initial_flow: bool) -> Result<()> {
		unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_setUseInitialFlow_bool(self.as_raw_mut_CUDA_OpticalFlowDual_TVL1(), use_initial_flow) }.into_result()
	}
	
}

impl dyn CUDA_OpticalFlowDual_TVL1 + '_ {
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
	pub fn create(tau: f64, lambda: f64, theta: f64, nscales: i32, warps: i32, epsilon: f64, iterations: i32, scale_step: f64, gamma: f64, use_initial_flow: bool) -> Result<core::Ptr::<dyn crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1>> {
		unsafe { sys::cv_cuda_OpticalFlowDual_TVL1_create_double_double_double_int_int_double_int_double_double_bool(tau, lambda, theta, nscales, warps, epsilon, iterations, scale_step, gamma, use_initial_flow) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1>::opencv_from_extern(r) } )
	}
	
}
/// Base interface for sparse optical flow algorithms.
pub trait CUDA_SparseOpticalFlow: core::AlgorithmTrait {
	fn as_raw_CUDA_SparseOpticalFlow(&self) -> *const c_void;
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
	fn calc(&mut self, prev_img: &dyn core::ToInputArray, next_img: &dyn core::ToInputArray, prev_pts: &dyn core::ToInputArray, next_pts: &mut dyn core::ToInputOutputArray, status: &mut dyn core::ToOutputArray, err: &mut dyn core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(prev_img);
		input_array_arg!(next_img);
		input_array_arg!(prev_pts);
		input_output_array_arg!(next_pts);
		output_array_arg!(status);
		output_array_arg!(err);
		unsafe { sys::cv_cuda_SparseOpticalFlow_calc_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_CUDA_SparseOpticalFlow(), prev_img.as_raw__InputArray(), next_img.as_raw__InputArray(), prev_pts.as_raw__InputArray(), next_pts.as_raw__InputOutputArray(), status.as_raw__OutputArray(), err.as_raw__OutputArray(), stream.as_raw_mut_Stream()) }.into_result()
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
pub trait CUDA_SparsePyrLKOpticalFlow: crate::cudaoptflow::CUDA_SparseOpticalFlow {
	fn as_raw_CUDA_SparsePyrLKOpticalFlow(&self) -> *const c_void;
	fn as_raw_mut_CUDA_SparsePyrLKOpticalFlow(&mut self) -> *mut c_void;

	fn get_win_size(&self) -> Result<core::Size> {
		unsafe { sys::cv_cuda_SparsePyrLKOpticalFlow_getWinSize_const(self.as_raw_CUDA_SparsePyrLKOpticalFlow()) }.into_result()
	}
	
	fn set_win_size(&mut self, win_size: core::Size) -> Result<()> {
		unsafe { sys::cv_cuda_SparsePyrLKOpticalFlow_setWinSize_Size(self.as_raw_mut_CUDA_SparsePyrLKOpticalFlow(), win_size.opencv_to_extern()) }.into_result()
	}
	
	fn get_max_level(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_SparsePyrLKOpticalFlow_getMaxLevel_const(self.as_raw_CUDA_SparsePyrLKOpticalFlow()) }.into_result()
	}
	
	fn set_max_level(&mut self, max_level: i32) -> Result<()> {
		unsafe { sys::cv_cuda_SparsePyrLKOpticalFlow_setMaxLevel_int(self.as_raw_mut_CUDA_SparsePyrLKOpticalFlow(), max_level) }.into_result()
	}
	
	fn get_num_iters(&self) -> Result<i32> {
		unsafe { sys::cv_cuda_SparsePyrLKOpticalFlow_getNumIters_const(self.as_raw_CUDA_SparsePyrLKOpticalFlow()) }.into_result()
	}
	
	fn set_num_iters(&mut self, iters: i32) -> Result<()> {
		unsafe { sys::cv_cuda_SparsePyrLKOpticalFlow_setNumIters_int(self.as_raw_mut_CUDA_SparsePyrLKOpticalFlow(), iters) }.into_result()
	}
	
	fn get_use_initial_flow(&self) -> Result<bool> {
		unsafe { sys::cv_cuda_SparsePyrLKOpticalFlow_getUseInitialFlow_const(self.as_raw_CUDA_SparsePyrLKOpticalFlow()) }.into_result()
	}
	
	fn set_use_initial_flow(&mut self, use_initial_flow: bool) -> Result<()> {
		unsafe { sys::cv_cuda_SparsePyrLKOpticalFlow_setUseInitialFlow_bool(self.as_raw_mut_CUDA_SparsePyrLKOpticalFlow(), use_initial_flow) }.into_result()
	}
	
}

impl dyn CUDA_SparsePyrLKOpticalFlow + '_ {
	/// ## C++ default parameters
	/// * win_size: Size(21,21)
	/// * max_level: 3
	/// * iters: 30
	/// * use_initial_flow: false
	pub fn create(win_size: core::Size, max_level: i32, iters: i32, use_initial_flow: bool) -> Result<core::Ptr::<dyn crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow>> {
		unsafe { sys::cv_cuda_SparsePyrLKOpticalFlow_create_Size_int_int_bool(win_size.opencv_to_extern(), max_level, iters, use_initial_flow) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow>::opencv_from_extern(r) } )
	}
	
}