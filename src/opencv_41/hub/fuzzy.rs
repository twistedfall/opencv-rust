//! # Image processing based on fuzzy mathematics
//!
//! Namespace for all functions is `ft`. The module brings implementation of the last image processing algorithms based on fuzzy mathematics. Method are named based on the pattern `FT`_degree_dimension`_`method.
//! # Math with F0-transform support
//!
//! Fuzzy transform (<span lang='latex'>F^0</span>-transform) of the 0th degree transforms whole image to a matrix of its components. These components are used in latter computation where each of them represents average color of certain subarea.
//!
//! # Math with F1-transform support
//!
//! Fuzzy transform (<span lang='latex'>F^1</span>-transform) of the 1th degree transforms whole image to a matrix of its components. Each component is polynomial of the 1th degree carrying information about average color and average gradient of certain subarea.
//!
//! # Fuzzy image processing
//!
//! Image proceesing based on fuzzy mathematics namely F-transform.
use std::os::raw::{c_char, c_void};
use libc::{ptrdiff_t, size_t};
use crate::{Error, Result, core, sys, types};

/// processing in several iterations
pub const ITERATIVE: i32 = 3;
/// linear (triangular) shape
pub const LINEAR: i32 = 1;
/// processing in multiple step
pub const MULTI_STEP: i32 = 2;
/// processing in one step
pub const ONE_STEP: i32 = 1;
/// sinusoidal shape
pub const SINUS: i32 = 2;

/// Sligtly less accurate version of <span lang='latex'>F^0</span>-transfrom computation optimized for higher speed. The methods counts with linear basic function.
/// ## Parameters
/// * matrix: Input 3 channels matrix.
/// * radius: Radius of the `ft::LINEAR` basic function.
/// * output: Output array.
///
/// This function computes F-transfrom and inverse F-transfotm using linear basic function in one step. It is ~10 times faster than `ft::FT02D_process` method.
pub fn ft02_d_fl_process(matrix: &core::Mat, radius: i32, output: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_ft_FT02D_FL_process_Mat_int_Mat(matrix.as_raw_Mat(), radius, output.as_raw_Mat()) }.into_result()
}

/// Sligtly less accurate version of <span lang='latex'>F^0</span>-transfrom computation optimized for higher speed. The methods counts with linear basic function.
/// ## Parameters
/// * matrix: Input 3 channels matrix.
/// * radius: Radius of the `ft::LINEAR` basic function.
/// * output: Output array.
///
/// This function computes F-transfrom and inverse F-transfotm using linear basic function in one step. It is ~9 times faster then `ft::FT02D_process` method and more accurate than `ft::FT02D_FL_process` method.
pub fn ft02_d_fl_process_float(matrix: &core::Mat, radius: i32, output: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_ft_FT02D_FL_process_float_Mat_int_Mat(matrix.as_raw_Mat(), radius, output.as_raw_Mat()) }.into_result()
}

/// Computes components of the array using direct <span lang='latex'>F^0</span>-transform.
/// ## Parameters
/// * matrix: Input array.
/// * kernel: Kernel used for processing. Function `ft::createKernel` can be used.
/// * components: Output 32-bit float array for the components.
/// * mask: Mask can be used for unwanted area marking.
///
/// The function computes components using predefined kernel and mask.
///
/// ## C++ default parameters
/// * mask: noArray()
pub fn ft02_d_components(matrix: &core::Mat, kernel: &core::Mat, components: &mut core::Mat, mask: &core::Mat) -> Result<()> {
    unsafe { sys::cv_ft_FT02D_components_Mat_Mat_Mat_Mat(matrix.as_raw_Mat(), kernel.as_raw_Mat(), components.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
}

/// Computes inverse <span lang='latex'>F^0</span>-transfrom.
/// ## Parameters
/// * components: Input 32-bit float single channel array for the components.
/// * kernel: Kernel used for processing. Function `ft::createKernel` can be used.
/// * output: Output 32-bit float array.
/// * width: Width of the output array.
/// * height: Height of the output array.
///
/// Computation of inverse F-transform.
pub fn ft02_d_inverse_ft(components: &core::Mat, kernel: &core::Mat, output: &mut core::Mat, width: i32, height: i32) -> Result<()> {
    unsafe { sys::cv_ft_FT02D_inverseFT_Mat_Mat_Mat_int_int(components.as_raw_Mat(), kernel.as_raw_Mat(), output.as_raw_Mat(), width, height) }.into_result()
}

/// Computes <span lang='latex'>F^0</span>-transfrom and inverse <span lang='latex'>F^0</span>-transfrom at once and return state.
/// ## Parameters
/// * matrix: Input matrix.
/// * kernel: Kernel used for processing. Function `ft::createKernel` can be used.
/// * output: Output 32-bit float array.
/// * mask: Mask used for unwanted area marking.
/// * maskOutput: Mask after one iteration.
/// * firstStop: If **true** function returns -1 when first problem appears. In case of `false` the process is completed and summation of all problems returned.
///
/// This function computes iteration of F-transfrom and inverse F-transfotm and handle image and mask change. The function is used in `ft::inpaint` function.
pub fn ft02_d_iteration(matrix: &core::Mat, kernel: &core::Mat, output: &mut core::Mat, mask: &core::Mat, mask_output: &mut core::Mat, first_stop: bool) -> Result<i32> {
    unsafe { sys::cv_ft_FT02D_iteration_Mat_Mat_Mat_Mat_Mat_bool(matrix.as_raw_Mat(), kernel.as_raw_Mat(), output.as_raw_Mat(), mask.as_raw_Mat(), mask_output.as_raw_Mat(), first_stop) }.into_result()
}

/// Computes <span lang='latex'>F^0</span>-transfrom and inverse <span lang='latex'>F^0</span>-transfrom at once.
/// ## Parameters
/// * matrix: Input matrix.
/// * kernel: Kernel used for processing. Function `ft::createKernel` can be used.
/// * output: Output 32-bit float array.
/// * mask: Mask used for unwanted area marking.
///
/// This function computes F-transfrom and inverse F-transfotm in one step. It is fully sufficient and optimized for `cv::Mat`.
///
/// ## C++ default parameters
/// * mask: noArray()
pub fn ft02_d_process(matrix: &core::Mat, kernel: &core::Mat, output: &mut core::Mat, mask: &core::Mat) -> Result<()> {
    unsafe { sys::cv_ft_FT02D_process_Mat_Mat_Mat_Mat(matrix.as_raw_Mat(), kernel.as_raw_Mat(), output.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
}

/// Computes components of the array using direct <span lang='latex'>F^1</span>-transform.
/// ## Parameters
/// * matrix: Input array.
/// * kernel: Kernel used for processing. Function `ft::createKernel` can be used.
/// * components: Output 32-bit float array for the components.
///
/// The function computes linear components using predefined kernel.
pub fn ft12_d_components(matrix: &core::Mat, kernel: &core::Mat, components: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_ft_FT12D_components_Mat_Mat_Mat(matrix.as_raw_Mat(), kernel.as_raw_Mat(), components.as_raw_Mat()) }.into_result()
}

/// Creates horizontal matrix for <span lang='latex'>F^1</span>-transform computation.
/// ## Parameters
/// * radius: Radius of the basic function.
/// * matrix: The horizontal matrix.
/// * chn: Number of channels.
///
/// The function creates helper horizontal matrix for <span lang='latex'>F^1</span>-transfrom processing. It is used for gradient computation.
pub fn ft12_d_create_polynom_matrix_horizontal(radius: i32, matrix: &mut core::Mat, chn: i32) -> Result<()> {
    unsafe { sys::cv_ft_FT12D_createPolynomMatrixHorizontal_int_Mat_int(radius, matrix.as_raw_Mat(), chn) }.into_result()
}

/// Creates vertical matrix for <span lang='latex'>F^1</span>-transform computation.
/// ## Parameters
/// * radius: Radius of the basic function.
/// * matrix: The vertical matrix.
/// * chn: Number of channels.
///
/// The function creates helper vertical matrix for <span lang='latex'>F^1</span>-transfrom processing. It is used for gradient computation.
pub fn ft12_d_create_polynom_matrix_vertical(radius: i32, matrix: &mut core::Mat, chn: i32) -> Result<()> {
    unsafe { sys::cv_ft_FT12D_createPolynomMatrixVertical_int_Mat_int(radius, matrix.as_raw_Mat(), chn) }.into_result()
}

/// Computes inverse <span lang='latex'>F^1</span>-transfrom.
/// ## Parameters
/// * components: Input 32-bit float single channel array for the components.
/// * kernel: Kernel used for processing. The same kernel as for components computation must be used.
/// * output: Output 32-bit float array.
/// * width: Width of the output array.
/// * height: Height of the output array.
///
/// Computation of inverse <span lang='latex'>F^1</span>-transform.
pub fn ft12_d_inverse_ft(components: &core::Mat, kernel: &core::Mat, output: &mut core::Mat, width: i32, height: i32) -> Result<()> {
    unsafe { sys::cv_ft_FT12D_inverseFT_Mat_Mat_Mat_int_int(components.as_raw_Mat(), kernel.as_raw_Mat(), output.as_raw_Mat(), width, height) }.into_result()
}

/// Computes elements of <span lang='latex'>F^1</span>-transform components.
/// ## Parameters
/// * matrix: Input array.
/// * kernel: Kernel used for processing. Function `ft::createKernel` can be used.
/// * c00: Elements represent average color.
/// * c10: Elements represent average vertical gradient.
/// * c01: Elements represent average horizontal gradient.
/// * components: Output 32-bit float array for the components.
/// * mask: Mask can be used for unwanted area marking.
///
/// The function computes components and its elements using predefined kernel and mask.
///
/// ## C++ default parameters
/// * mask: noArray()
pub fn ft12_d_polynomial(matrix: &core::Mat, kernel: &core::Mat, c00: &mut core::Mat, c10: &mut core::Mat, c01: &mut core::Mat, components: &mut core::Mat, mask: &core::Mat) -> Result<()> {
    unsafe { sys::cv_ft_FT12D_polynomial_Mat_Mat_Mat_Mat_Mat_Mat_Mat(matrix.as_raw_Mat(), kernel.as_raw_Mat(), c00.as_raw_Mat(), c10.as_raw_Mat(), c01.as_raw_Mat(), components.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
}

/// Computes <span lang='latex'>F^1</span>-transfrom and inverse <span lang='latex'>F^1</span>-transfrom at once.
/// ## Parameters
/// * matrix: Input matrix.
/// * kernel: Kernel used for processing. Function `ft::createKernel` can be used.
/// * output: Output 32-bit float array.
/// * mask: Mask used for unwanted area marking.
///
/// This function computes <span lang='latex'>F^1</span>-transfrom and inverse <span lang='latex'>F^1</span>-transfotm in one step. It is fully sufficient and optimized for `cv::Mat`.
///
///
/// Note:
/// F-transform technique of first degreee is described in paper [Vlas](https://docs.opencv.org/4.1.1/d0/de3/citelist.html#CITEREF_Vlas):FT.
///
/// ## C++ default parameters
/// * mask: noArray()
pub fn ft12_d_process(matrix: &core::Mat, kernel: &core::Mat, output: &mut core::Mat, mask: &core::Mat) -> Result<()> {
    unsafe { sys::cv_ft_FT12D_process_Mat_Mat_Mat_Mat(matrix.as_raw_Mat(), kernel.as_raw_Mat(), output.as_raw_Mat(), mask.as_raw_Mat()) }.into_result()
}

/// Creates kernel from basic functions.
/// ## Parameters
/// * A: Basic function used in axis **x**.
/// * B: Basic function used in axis **y**.
/// * kernel: Final 32-bit kernel derived from **A** and **B**.
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
/// * kernel: Final 32-bit kernel.
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
/// *   `ft::LINEAR` Linear basic function.
/// * algorithm: Algorithm could be one of the following:
/// *   `ft::ONE_STEP` One step algorithm.
/// *   `ft::MULTI_STEP` This algorithm automaticaly increases radius of the basic function.
/// *   `ft::ITERATIVE` Iterative algorithm running in more steps using partial computations.
///
/// This function provides inpainting technique based on the fuzzy mathematic.
///
///
/// Note:
/// The algorithms are described in paper [Perf](https://docs.opencv.org/4.1.1/d0/de3/citelist.html#CITEREF_Perf):rec.
pub fn inpaint(image: &core::Mat, mask: &core::Mat, output: &mut core::Mat, radius: i32, function: i32, algorithm: i32) -> Result<()> {
    unsafe { sys::cv_ft_inpaint_Mat_Mat_Mat_int_int_int(image.as_raw_Mat(), mask.as_raw_Mat(), output.as_raw_Mat(), radius, function, algorithm) }.into_result()
}

