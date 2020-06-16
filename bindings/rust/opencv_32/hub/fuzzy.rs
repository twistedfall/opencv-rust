#![allow(unused_parens)]
//! # Image processing based on fuzzy mathematics
//! 
//! Namespace for all functions is **ft**. The module brings implementation of the last image processing algorithms based on fuzzy mathematics.
//!    # Math with F0-transfrom support
//! 
//! Fuzzy transform (F-transform) of the 0th degree transform whole image to a vector of its components. These components are used in latter computation.
//! 
//!    # Fuzzy image processing
//! 
//! Image proceesing based on F-transform is fast to process and easy to understand.
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use {  };
}

pub const ITERATIVE: i32 = 3;
pub const LINEAR: i32 = 1;
pub const MULTI_STEP: i32 = 2;
pub const ONE_STEP: i32 = 1;
pub const SINUS: i32 = 2;
/// Computes components of the array using direct F0-transform.
/// ## Parameters
/// * matrix: Input array.
/// * kernel: Kernel used for processing. Function **createKernel** can be used.
/// * components: Output 32-bit array for the components.
/// 
/// The function computes components using predefined kernel.
/// 
/// 
/// Note:
///    F-transform technique is described in paper [Perf:FT](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Perf:FT).
pub fn ft02_d_components(matrix: &dyn core::ToInputArray, kernel: &dyn core::ToInputArray, components: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(matrix);
	input_array_arg!(kernel);
	output_array_arg!(components);
	unsafe { sys::cv_ft_FT02D_components_const__InputArrayR_const__InputArrayR_const__OutputArrayR(matrix.as_raw__InputArray(), kernel.as_raw__InputArray(), components.as_raw__OutputArray()) }.into_result()
}

/// Computes components of the array using direct F0-transform.
/// ## Parameters
/// * matrix: Input array.
/// * kernel: Kernel used for processing. Function **createKernel** can be used.
/// * components: Output 32-bit array for the components.
/// * mask: Mask can be used for unwanted area marking.
/// 
/// The function computes components using predefined kernel and mask.
/// 
/// 
/// Note:
///    F-transform technique is described in paper [Perf:FT](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Perf:FT).
pub fn ft02_d_components1(matrix: &dyn core::ToInputArray, kernel: &dyn core::ToInputArray, components: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(matrix);
	input_array_arg!(kernel);
	output_array_arg!(components);
	input_array_arg!(mask);
	unsafe { sys::cv_ft_FT02D_components_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(matrix.as_raw__InputArray(), kernel.as_raw__InputArray(), components.as_raw__OutputArray(), mask.as_raw__InputArray()) }.into_result()
}

/// Computes inverse F0-transfrom.
/// ## Parameters
/// * components: Input 32-bit single channel array for the components.
/// * kernel: Kernel used for processing. Function **createKernel** can be used.
/// * output: Output 32-bit array.
/// * width: Width of the output array.
/// * height: Height of the output array.
/// 
/// 
/// Note:
///    F-transform technique is described in paper [Perf:FT](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Perf:FT).
pub fn ft02_d_inverse_ft(components: &dyn core::ToInputArray, kernel: &dyn core::ToInputArray, output: &mut dyn core::ToOutputArray, width: i32, height: i32) -> Result<()> {
	input_array_arg!(components);
	input_array_arg!(kernel);
	output_array_arg!(output);
	unsafe { sys::cv_ft_FT02D_inverseFT_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int(components.as_raw__InputArray(), kernel.as_raw__InputArray(), output.as_raw__OutputArray(), width, height) }.into_result()
}

/// Computes F0-transfrom and inverse F0-transfrom at once and return state.
/// ## Parameters
/// * matrix: Input matrix.
/// * kernel: Kernel used for processing. Function **createKernel** can be used.
/// * output: Output 32-bit array.
/// * mask: Mask used for unwanted area marking.
/// * maskOutput: Mask after one iteration.
/// * firstStop: If **true** function returns -1 when first problem appears. In case of **false**, the process is completed and summation of all problems returned.
/// 
/// This function computes iteration of F-transfrom and inverse F-transfotm and handle image and mask change. The function is used in *inpaint* function.
pub fn ft02_d_iteration(matrix: &dyn core::ToInputArray, kernel: &dyn core::ToInputArray, output: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray, mask_output: &mut dyn core::ToOutputArray, first_stop: bool) -> Result<i32> {
	input_array_arg!(matrix);
	input_array_arg!(kernel);
	output_array_arg!(output);
	input_array_arg!(mask);
	output_array_arg!(mask_output);
	unsafe { sys::cv_ft_FT02D_iteration_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__OutputArrayR_bool(matrix.as_raw__InputArray(), kernel.as_raw__InputArray(), output.as_raw__OutputArray(), mask.as_raw__InputArray(), mask_output.as_raw__OutputArray(), first_stop) }.into_result()
}

/// Computes F0-transfrom and inverse F0-transfrom at once.
/// ## Parameters
/// * matrix: Input matrix.
/// * kernel: Kernel used for processing. Function **createKernel** can be used.
/// * output: Output 32-bit array.
/// 
/// This function computes F-transfrom and inverse F-transfotm in one step. It is fully sufficient and optimized for **Mat**.
pub fn ft02_d_process(matrix: &dyn core::ToInputArray, kernel: &dyn core::ToInputArray, output: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(matrix);
	input_array_arg!(kernel);
	output_array_arg!(output);
	unsafe { sys::cv_ft_FT02D_process_const__InputArrayR_const__InputArrayR_const__OutputArrayR(matrix.as_raw__InputArray(), kernel.as_raw__InputArray(), output.as_raw__OutputArray()) }.into_result()
}

/// Computes F0-transfrom and inverse F0-transfrom at once.
/// ## Parameters
/// * matrix: Input matrix.
/// * kernel: Kernel used for processing. Function **createKernel** can be used.
/// * output: Output 32-bit array.
/// * mask: Mask used for unwanted area marking.
/// 
/// This function computes F-transfrom and inverse F-transfotm in one step. It is fully sufficient and optimized for **Mat**.
pub fn ft02_d_process1(matrix: &dyn core::ToInputArray, kernel: &dyn core::ToInputArray, output: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(matrix);
	input_array_arg!(kernel);
	output_array_arg!(output);
	input_array_arg!(mask);
	unsafe { sys::cv_ft_FT02D_process_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(matrix.as_raw__InputArray(), kernel.as_raw__InputArray(), output.as_raw__OutputArray(), mask.as_raw__InputArray()) }.into_result()
}

/// Creates kernel from basic functions.
/// ## Parameters
/// * A: Basic function used in axis **x**.
/// * B: Basic function used in axis **y**.
/// * kernel: Final 32-b kernel derived from **A** and **B**.
/// * chn: Number of kernel channels.
/// 
/// The function creates kernel usable for latter fuzzy image processing.
pub fn create_kernel1(a: &dyn core::ToInputArray, b: &dyn core::ToInputArray, kernel: &mut dyn core::ToOutputArray, chn: i32) -> Result<()> {
	input_array_arg!(a);
	input_array_arg!(b);
	output_array_arg!(kernel);
	unsafe { sys::cv_ft_createKernel_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(a.as_raw__InputArray(), b.as_raw__InputArray(), kernel.as_raw__OutputArray(), chn) }.into_result()
}

/// Creates kernel from general functions.
/// ## Parameters
/// * function: Function type could be one of the following:
///    *   **LINEAR** Linear basic function.
/// * radius: Radius of the basic function.
/// * kernel: Final 32-b kernel.
/// * chn: Number of kernel channels.
/// 
/// The function creates kernel from predefined functions.
pub fn create_kernel(function: i32, radius: i32, kernel: &mut dyn core::ToOutputArray, chn: i32) -> Result<()> {
	output_array_arg!(kernel);
	unsafe { sys::cv_ft_createKernel_int_int_const__OutputArrayR_int(function, radius, kernel.as_raw__OutputArray(), chn) }.into_result()
}

/// Image filtering
/// ## Parameters
/// * image: Input image.
/// * kernel: Final 32-bit kernel.
/// * output: Output 32-bit image.
/// 
/// Filtering of the input image by means of F-transform.
pub fn filter(image: &dyn core::ToInputArray, kernel: &dyn core::ToInputArray, output: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(image);
	input_array_arg!(kernel);
	output_array_arg!(output);
	unsafe { sys::cv_ft_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR(image.as_raw__InputArray(), kernel.as_raw__InputArray(), output.as_raw__OutputArray()) }.into_result()
}

/// Image inpainting
/// ## Parameters
/// * image: Input image.
/// * mask: Mask used for unwanted area marking.
/// * output: Output 32-bit image.
/// * radius: Radius of the basic function.
/// * function: Function type could be one of the following:
///    *   **LINEAR** Linear basic function.
/// * algorithm: Algorithm could be one of the following:
///    *   **ONE_STEP** One step algorithm.
///    *   **MULTI_STEP** Algorithm automaticaly increasing radius of the basic function.
///    *   **ITERATIVE** Iterative algorithm running in more steps using partial computations.
/// 
/// This function provides inpainting technique based on the fuzzy mathematic.
/// 
/// 
/// Note:
///    The algorithms are described in paper [Perf:rec](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Perf:rec).
pub fn inpaint(image: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, output: &mut dyn core::ToOutputArray, radius: i32, function: i32, algorithm: i32) -> Result<()> {
	input_array_arg!(image);
	input_array_arg!(mask);
	output_array_arg!(output);
	unsafe { sys::cv_ft_inpaint_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int(image.as_raw__InputArray(), mask.as_raw__InputArray(), output.as_raw__OutputArray(), radius, function, algorithm) }.into_result()
}
