//! # Image processing based on fuzzy mathematics
//!
//! Namespace for all functions is **ft**. The module brings implementation of the last image processing algorithms based on fuzzy mathematics.
//! # Math with F0-transfrom support
//!
//! Fuzzy transform (F-transform) of the 0th degree transform whole image to a vector of its components. These components are used in latter computation.
//!
//! # Fuzzy image processing
//!
//! Image proceesing based on F-transform is fast to process and easy to understand.
use std::os::raw::{c_char, c_void};
use libc::{ptrdiff_t, size_t};
use crate::{Error, Result, core, sys, types};

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
/// F-transform technique is described in paper [Perf](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Perf):FT.
pub fn ft02_d_components(matrix: &core::Mat, kernel: &core::Mat, components: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_ft_FT02D_components_Mat_Mat_Mat(matrix.as_raw_Mat(), kernel.as_raw_Mat(), components.as_raw_Mat()) }.into_result()
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
/// F-transform technique is described in paper [Perf](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Perf):FT.
pub fn ft02_d_components_1(matrix: &core::Mat, kernel: &core::Mat, components: &mut core::Mat, mask: &core::Mat) -> Result<()> {
    unsafe { sys::cv_ft_FT02D_components_Mat_Mat_Mat_Mat(matrix.as_raw_Mat(), kernel.as_raw_Mat(), components.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
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
/// F-transform technique is described in paper [Perf](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Perf):FT.
pub fn ft02_d_inverse_ft(components: &core::Mat, kernel: &core::Mat, output: &mut core::Mat, width: i32, height: i32) -> Result<()> {
    unsafe { sys::cv_ft_FT02D_inverseFT_Mat_Mat_Mat_int_int(components.as_raw_Mat(), kernel.as_raw_Mat(), output.as_raw_Mat(), width, height) }.into_result()
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
pub fn ft02_d_iteration(matrix: &core::Mat, kernel: &core::Mat, output: &mut core::Mat, mask: &core::Mat, mask_output: &mut core::Mat, first_stop: bool) -> Result<i32> {
    unsafe { sys::cv_ft_FT02D_iteration_Mat_Mat_Mat_Mat_Mat_bool(matrix.as_raw_Mat(), kernel.as_raw_Mat(), output.as_raw_Mat(), mask.as_raw_Mat(), mask_output.as_raw_Mat(), first_stop) }.into_result()
}

/// Computes F0-transfrom and inverse F0-transfrom at once.
/// ## Parameters
/// * matrix: Input matrix.
/// * kernel: Kernel used for processing. Function **createKernel** can be used.
/// * output: Output 32-bit array.
///
/// This function computes F-transfrom and inverse F-transfotm in one step. It is fully sufficient and optimized for **Mat**.
pub fn ft02_d_process(matrix: &core::Mat, kernel: &core::Mat, output: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_ft_FT02D_process_Mat_Mat_Mat(matrix.as_raw_Mat(), kernel.as_raw_Mat(), output.as_raw_Mat()) }.into_result()
}

/// Computes F0-transfrom and inverse F0-transfrom at once.
/// ## Parameters
/// * matrix: Input matrix.
/// * kernel: Kernel used for processing. Function **createKernel** can be used.
/// * output: Output 32-bit array.
/// * mask: Mask used for unwanted area marking.
///
/// This function computes F-transfrom and inverse F-transfotm in one step. It is fully sufficient and optimized for **Mat**.
pub fn ft02_d_process_1(matrix: &core::Mat, kernel: &core::Mat, output: &mut core::Mat, mask: &core::Mat) -> Result<()> {
    unsafe { sys::cv_ft_FT02D_process_Mat_Mat_Mat_Mat(matrix.as_raw_Mat(), kernel.as_raw_Mat(), output.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
}

/// Creates kernel from basic functions.
/// ## Parameters
/// * A: Basic function used in axis **x**.
/// * B: Basic function used in axis **y**.
/// * kernel: Final 32-b kernel derived from **A** and **B**.
/// * chn: Number of kernel channels.
///
/// The function creates kernel usable for latter fuzzy image processing.
pub fn create_kernel(a: &core::Mat, b: &core::Mat, kernel: &mut core::Mat, chn: i32) -> Result<()> {
    unsafe { sys::cv_ft_createKernel_Mat_Mat_Mat_int(a.as_raw_Mat(), b.as_raw_Mat(), kernel.as_raw_Mat(), chn) }.into_result()
}

/// Creates kernel from general functions.
/// ## Parameters
/// * function: Function type could be one of the following:
/// *   **LINEAR** Linear basic function.
/// * radius: Radius of the basic function.
/// * kernel: Final 32-b kernel.
/// * chn: Number of kernel channels.
///
/// The function creates kernel from predefined functions.
pub fn create_kernel_1(function: i32, radius: i32, kernel: &mut core::Mat, chn: i32) -> Result<()> {
    unsafe { sys::cv_ft_createKernel_int_int_Mat_int(function, radius, kernel.as_raw_Mat(), chn) }.into_result()
}

/// Image filtering
/// ## Parameters
/// * image: Input image.
/// * kernel: Final 32-bit kernel.
/// * output: Output 32-bit image.
///
/// Filtering of the input image by means of F-transform.
pub fn filter(image: &core::Mat, kernel: &core::Mat, output: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_ft_filter_Mat_Mat_Mat(image.as_raw_Mat(), kernel.as_raw_Mat(), output.as_raw_Mat()) }.into_result()
}

/// Image inpainting
/// ## Parameters
/// * image: Input image.
/// * mask: Mask used for unwanted area marking.
/// * output: Output 32-bit image.
/// * radius: Radius of the basic function.
/// * function: Function type could be one of the following:
/// *   **LINEAR** Linear basic function.
/// * algorithm: Algorithm could be one of the following:
/// *   **ONE_STEP** One step algorithm.
/// *   **MULTI_STEP** Algorithm automaticaly increasing radius of the basic function.
/// *   **ITERATIVE** Iterative algorithm running in more steps using partial computations.
///
/// This function provides inpainting technique based on the fuzzy mathematic.
///
///
/// Note:
/// The algorithms are described in paper [Perf](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Perf):rec.
pub fn inpaint(image: &core::Mat, mask: &core::Mat, output: &mut core::Mat, radius: i32, function: i32, algorithm: i32) -> Result<()> {
    unsafe { sys::cv_ft_inpaint_Mat_Mat_Mat_int_int_int(image.as_raw_Mat(), mask.as_raw_Mat(), output.as_raw_Mat(), radius, function, algorithm) }.into_result()
}

