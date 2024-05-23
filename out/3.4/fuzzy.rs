//! # Image processing based on fuzzy mathematics
//!
//! Namespace for all functions is `ft`. The module brings implementation of the last image processing algorithms based on fuzzy mathematics. Method are named based on the pattern `FT`_degree_dimension`_`method.
//!    # Math with F0-transform support
//!
//! Fuzzy transform (![inline formula](https://latex.codecogs.com/png.latex?F%5E0)-transform) of the 0th degree transforms whole image to a matrix of its components. These components are used in latter computation where each of them represents average color of certain subarea.
//!
//!    # Math with F1-transform support
//!
//! Fuzzy transform (![inline formula](https://latex.codecogs.com/png.latex?F%5E1)-transform) of the 1th degree transforms whole image to a matrix of its components. Each component is polynomial of the 1th degree carrying information about average color and average gradient of certain subarea.
//!
//!    # Fuzzy image processing
//!
//! Image proceesing based on fuzzy mathematics namely F-transform.
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
}

/// processing in several iterations
// ITERATIVE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/types.hpp:63
pub const ITERATIVE: i32 = 3;
/// linear (triangular) shape
// LINEAR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/types.hpp:55
pub const LINEAR: i32 = 1;
/// processing in multiple step
// MULTI_STEP /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/types.hpp:62
pub const MULTI_STEP: i32 = 2;
/// processing in one step
// ONE_STEP /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/types.hpp:61
pub const ONE_STEP: i32 = 1;
/// sinusoidal shape
// SINUS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/types.hpp:56
pub const SINUS: i32 = 2;
/// Sligtly less accurate version of ![inline formula](https://latex.codecogs.com/png.latex?F%5E0)-transfrom computation optimized for higher speed. The methods counts with linear basic function.
/// ## Parameters
/// * matrix: Input 3 channels matrix.
/// * radius: Radius of the `ft::LINEAR` basic function.
/// * output: Output array.
///
/// This function computes F-transfrom and inverse F-transfotm using linear basic function in one step. It is ~10 times faster than `ft::FT02D_process` method.
// FT02D_FL_process(InputArray, const int, OutputArray)(InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F0_math.hpp:106
// ("cv::ft::FT02D_FL_process", vec![(pred!(mut, ["matrix", "radius", "output"], ["const cv::_InputArray*", "const int", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn ft0_2d_fl_process(matrix: &impl ToInputArray, radius: i32, output: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(matrix);
	output_array_arg!(output);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT02D_FL_process_const__InputArrayR_const_int_const__OutputArrayR(matrix.as_raw__InputArray(), radius, output.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Sligtly less accurate version of ![inline formula](https://latex.codecogs.com/png.latex?F%5E0)-transfrom computation optimized for higher speed. The methods counts with linear basic function.
/// ## Parameters
/// * matrix: Input 3 channels matrix.
/// * radius: Radius of the `ft::LINEAR` basic function.
/// * output: Output array.
///
/// This function computes F-transfrom and inverse F-transfotm using linear basic function in one step. It is ~9 times faster then `ft::FT02D_process` method and more accurate than `ft::FT02D_FL_process` method.
// FT02D_FL_process_float(InputArray, const int, OutputArray)(InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F0_math.hpp:115
// ("cv::ft::FT02D_FL_process_float", vec![(pred!(mut, ["matrix", "radius", "output"], ["const cv::_InputArray*", "const int", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn ft0_2d_fl_process_float(matrix: &impl ToInputArray, radius: i32, output: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(matrix);
	output_array_arg!(output);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT02D_FL_process_float_const__InputArrayR_const_int_const__OutputArrayR(matrix.as_raw__InputArray(), radius, output.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes components of the array using direct ![inline formula](https://latex.codecogs.com/png.latex?F%5E0)-transform.
/// ## Parameters
/// * matrix: Input array.
/// * kernel: Kernel used for processing. Function `ft::createKernel` can be used.
/// * components: Output 32-bit float array for the components.
/// * mask: Mask can be used for unwanted area marking.
///
/// The function computes components using predefined kernel and mask.
///
/// ## Note
/// This alternative version of [ft0_2d_components] function uses the following default values for its arguments:
/// * mask: noArray()
// cv::ft::FT02D_components(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F0_math.hpp:64
// ("cv::ft::FT02D_components", vec![(pred!(mut, ["matrix", "kernel", "components"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn ft0_2d_components_def(matrix: &impl ToInputArray, kernel: &impl ToInputArray, components: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(matrix);
	input_array_arg!(kernel);
	output_array_arg!(components);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT02D_components_const__InputArrayR_const__InputArrayR_const__OutputArrayR(matrix.as_raw__InputArray(), kernel.as_raw__InputArray(), components.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes components of the array using direct ![inline formula](https://latex.codecogs.com/png.latex?F%5E0)-transform.
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
// FT02D_components(InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F0_math.hpp:64
// ("cv::ft::FT02D_components", vec![(pred!(mut, ["matrix", "kernel", "components", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn ft0_2d_components(matrix: &impl ToInputArray, kernel: &impl ToInputArray, components: &mut impl ToOutputArray, mask: &impl ToInputArray) -> Result<()> {
	input_array_arg!(matrix);
	input_array_arg!(kernel);
	output_array_arg!(components);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT02D_components_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(matrix.as_raw__InputArray(), kernel.as_raw__InputArray(), components.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes inverse ![inline formula](https://latex.codecogs.com/png.latex?F%5E0)-transfrom.
/// ## Parameters
/// * components: Input 32-bit float single channel array for the components.
/// * kernel: Kernel used for processing. Function `ft::createKernel` can be used.
/// * output: Output 32-bit float array.
/// * width: Width of the output array.
/// * height: Height of the output array.
///
/// Computation of inverse F-transform.
// FT02D_inverseFT(InputArray, InputArray, OutputArray, int, int)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F0_math.hpp:75
// ("cv::ft::FT02D_inverseFT", vec![(pred!(mut, ["components", "kernel", "output", "width", "height"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
#[inline]
pub fn ft0_2d_inverse_ft(components: &impl ToInputArray, kernel: &impl ToInputArray, output: &mut impl ToOutputArray, width: i32, height: i32) -> Result<()> {
	input_array_arg!(components);
	input_array_arg!(kernel);
	output_array_arg!(output);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT02D_inverseFT_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int(components.as_raw__InputArray(), kernel.as_raw__InputArray(), output.as_raw__OutputArray(), width, height, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes ![inline formula](https://latex.codecogs.com/png.latex?F%5E0)-transfrom and inverse ![inline formula](https://latex.codecogs.com/png.latex?F%5E0)-transfrom at once and return state.
/// ## Parameters
/// * matrix: Input matrix.
/// * kernel: Kernel used for processing. Function `ft::createKernel` can be used.
/// * output: Output 32-bit float array.
/// * mask: Mask used for unwanted area marking.
/// * maskOutput: Mask after one iteration.
/// * firstStop: If **true** function returns -1 when first problem appears. In case of `false` the process is completed and summation of all problems returned.
///
/// This function computes iteration of F-transfrom and inverse F-transfotm and handle image and mask change. The function is used in `ft::inpaint` function.
// FT02D_iteration(InputArray, InputArray, OutputArray, InputArray, OutputArray, bool)(InputArray, InputArray, OutputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F0_math.hpp:97
// ("cv::ft::FT02D_iteration", vec![(pred!(mut, ["matrix", "kernel", "output", "mask", "maskOutput", "firstStop"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "bool"]), _)]),
#[inline]
pub fn ft0_2d_iteration(matrix: &impl ToInputArray, kernel: &impl ToInputArray, output: &mut impl ToOutputArray, mask: &impl ToInputArray, mask_output: &mut impl ToOutputArray, first_stop: bool) -> Result<i32> {
	input_array_arg!(matrix);
	input_array_arg!(kernel);
	output_array_arg!(output);
	input_array_arg!(mask);
	output_array_arg!(mask_output);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT02D_iteration_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__OutputArrayR_bool(matrix.as_raw__InputArray(), kernel.as_raw__InputArray(), output.as_raw__OutputArray(), mask.as_raw__InputArray(), mask_output.as_raw__OutputArray(), first_stop, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes ![inline formula](https://latex.codecogs.com/png.latex?F%5E0)-transfrom and inverse ![inline formula](https://latex.codecogs.com/png.latex?F%5E0)-transfrom at once.
/// ## Parameters
/// * matrix: Input matrix.
/// * kernel: Kernel used for processing. Function `ft::createKernel` can be used.
/// * output: Output 32-bit float array.
/// * mask: Mask used for unwanted area marking.
///
/// This function computes F-transfrom and inverse F-transfotm in one step. It is fully sufficient and optimized for `cv::Mat`.
///
/// ## Note
/// This alternative version of [ft0_2d_process] function uses the following default values for its arguments:
/// * mask: noArray()
// cv::ft::FT02D_process(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F0_math.hpp:85
// ("cv::ft::FT02D_process", vec![(pred!(mut, ["matrix", "kernel", "output"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn ft0_2d_process_def(matrix: &impl ToInputArray, kernel: &impl ToInputArray, output: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(matrix);
	input_array_arg!(kernel);
	output_array_arg!(output);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT02D_process_const__InputArrayR_const__InputArrayR_const__OutputArrayR(matrix.as_raw__InputArray(), kernel.as_raw__InputArray(), output.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes ![inline formula](https://latex.codecogs.com/png.latex?F%5E0)-transfrom and inverse ![inline formula](https://latex.codecogs.com/png.latex?F%5E0)-transfrom at once.
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
// FT02D_process(InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F0_math.hpp:85
// ("cv::ft::FT02D_process", vec![(pred!(mut, ["matrix", "kernel", "output", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn ft0_2d_process(matrix: &impl ToInputArray, kernel: &impl ToInputArray, output: &mut impl ToOutputArray, mask: &impl ToInputArray) -> Result<()> {
	input_array_arg!(matrix);
	input_array_arg!(kernel);
	output_array_arg!(output);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT02D_process_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(matrix.as_raw__InputArray(), kernel.as_raw__InputArray(), output.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes components of the array using direct ![inline formula](https://latex.codecogs.com/png.latex?F%5E1)-transform.
/// ## Parameters
/// * matrix: Input array.
/// * kernel: Kernel used for processing. Function `ft::createKernel` can be used.
/// * components: Output 32-bit float array for the components.
///
/// The function computes linear components using predefined kernel.
// FT12D_components(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F1_math.hpp:63
// ("cv::ft::FT12D_components", vec![(pred!(mut, ["matrix", "kernel", "components"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn ft1_2d_components(matrix: &impl ToInputArray, kernel: &impl ToInputArray, components: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(matrix);
	input_array_arg!(kernel);
	output_array_arg!(components);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT12D_components_const__InputArrayR_const__InputArrayR_const__OutputArrayR(matrix.as_raw__InputArray(), kernel.as_raw__InputArray(), components.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Creates horizontal matrix for ![inline formula](https://latex.codecogs.com/png.latex?F%5E1)-transform computation.
/// ## Parameters
/// * radius: Radius of the basic function.
/// * matrix: The horizontal matrix.
/// * chn: Number of channels.
///
/// The function creates helper horizontal matrix for ![inline formula](https://latex.codecogs.com/png.latex?F%5E1)-transfrom processing. It is used for gradient computation.
// FT12D_createPolynomMatrixHorizontal(int, OutputArray, const int)(Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F1_math.hpp:94
// ("cv::ft::FT12D_createPolynomMatrixHorizontal", vec![(pred!(mut, ["radius", "matrix", "chn"], ["int", "const cv::_OutputArray*", "const int"]), _)]),
#[inline]
pub fn ft1_2d_create_polynom_matrix_horizontal(radius: i32, matrix: &mut impl ToOutputArray, chn: i32) -> Result<()> {
	output_array_arg!(matrix);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT12D_createPolynomMatrixHorizontal_int_const__OutputArrayR_const_int(radius, matrix.as_raw__OutputArray(), chn, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Creates vertical matrix for ![inline formula](https://latex.codecogs.com/png.latex?F%5E1)-transform computation.
/// ## Parameters
/// * radius: Radius of the basic function.
/// * matrix: The vertical matrix.
/// * chn: Number of channels.
///
/// The function creates helper vertical matrix for ![inline formula](https://latex.codecogs.com/png.latex?F%5E1)-transfrom processing. It is used for gradient computation.
// FT12D_createPolynomMatrixVertical(int, OutputArray, const int)(Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F1_math.hpp:85
// ("cv::ft::FT12D_createPolynomMatrixVertical", vec![(pred!(mut, ["radius", "matrix", "chn"], ["int", "const cv::_OutputArray*", "const int"]), _)]),
#[inline]
pub fn ft1_2d_create_polynom_matrix_vertical(radius: i32, matrix: &mut impl ToOutputArray, chn: i32) -> Result<()> {
	output_array_arg!(matrix);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT12D_createPolynomMatrixVertical_int_const__OutputArrayR_const_int(radius, matrix.as_raw__OutputArray(), chn, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes inverse ![inline formula](https://latex.codecogs.com/png.latex?F%5E1)-transfrom.
/// ## Parameters
/// * components: Input 32-bit float single channel array for the components.
/// * kernel: Kernel used for processing. The same kernel as for components computation must be used.
/// * output: Output 32-bit float array.
/// * width: Width of the output array.
/// * height: Height of the output array.
///
/// Computation of inverse ![inline formula](https://latex.codecogs.com/png.latex?F%5E1)-transform.
// FT12D_inverseFT(InputArray, InputArray, OutputArray, int, int)(InputArray, InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F1_math.hpp:118
// ("cv::ft::FT12D_inverseFT", vec![(pred!(mut, ["components", "kernel", "output", "width", "height"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
#[inline]
pub fn ft1_2d_inverse_ft(components: &impl ToInputArray, kernel: &impl ToInputArray, output: &mut impl ToOutputArray, width: i32, height: i32) -> Result<()> {
	input_array_arg!(components);
	input_array_arg!(kernel);
	output_array_arg!(output);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT12D_inverseFT_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int(components.as_raw__InputArray(), kernel.as_raw__InputArray(), output.as_raw__OutputArray(), width, height, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes elements of ![inline formula](https://latex.codecogs.com/png.latex?F%5E1)-transform components.
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
/// ## Note
/// This alternative version of [ft1_2d_polynomial] function uses the following default values for its arguments:
/// * mask: noArray()
// cv::ft::FT12D_polynomial(InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F1_math.hpp:76
// ("cv::ft::FT12D_polynomial", vec![(pred!(mut, ["matrix", "kernel", "c00", "c10", "c01", "components"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn ft1_2d_polynomial_def(matrix: &impl ToInputArray, kernel: &impl ToInputArray, c00: &mut impl ToOutputArray, c10: &mut impl ToOutputArray, c01: &mut impl ToOutputArray, components: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(matrix);
	input_array_arg!(kernel);
	output_array_arg!(c00);
	output_array_arg!(c10);
	output_array_arg!(c01);
	output_array_arg!(components);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT12D_polynomial_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(matrix.as_raw__InputArray(), kernel.as_raw__InputArray(), c00.as_raw__OutputArray(), c10.as_raw__OutputArray(), c01.as_raw__OutputArray(), components.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes elements of ![inline formula](https://latex.codecogs.com/png.latex?F%5E1)-transform components.
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
// FT12D_polynomial(InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, OutputArray, OutputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F1_math.hpp:76
// ("cv::ft::FT12D_polynomial", vec![(pred!(mut, ["matrix", "kernel", "c00", "c10", "c01", "components", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn ft1_2d_polynomial(matrix: &impl ToInputArray, kernel: &impl ToInputArray, c00: &mut impl ToOutputArray, c10: &mut impl ToOutputArray, c01: &mut impl ToOutputArray, components: &mut impl ToOutputArray, mask: &impl ToInputArray) -> Result<()> {
	input_array_arg!(matrix);
	input_array_arg!(kernel);
	output_array_arg!(c00);
	output_array_arg!(c10);
	output_array_arg!(c01);
	output_array_arg!(components);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT12D_polynomial_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR(matrix.as_raw__InputArray(), kernel.as_raw__InputArray(), c00.as_raw__OutputArray(), c10.as_raw__OutputArray(), c01.as_raw__OutputArray(), components.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes ![inline formula](https://latex.codecogs.com/png.latex?F%5E1)-transfrom and inverse ![inline formula](https://latex.codecogs.com/png.latex?F%5E1)-transfrom at once.
/// ## Parameters
/// * matrix: Input matrix.
/// * kernel: Kernel used for processing. Function `ft::createKernel` can be used.
/// * output: Output 32-bit float array.
/// * mask: Mask used for unwanted area marking.
///
/// This function computes ![inline formula](https://latex.codecogs.com/png.latex?F%5E1)-transfrom and inverse ![inline formula](https://latex.codecogs.com/png.latex?F%5E1)-transfotm in one step. It is fully sufficient and optimized for `cv::Mat`.
///
///
/// Note:
///    F-transform technique of first degreee is described in paper [Vlas:FT](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Vlas:FT).
///
/// ## Note
/// This alternative version of [ft1_2d_process] function uses the following default values for its arguments:
/// * mask: noArray()
// cv::ft::FT12D_process(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F1_math.hpp:107
// ("cv::ft::FT12D_process", vec![(pred!(mut, ["matrix", "kernel", "output"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn ft1_2d_process_def(matrix: &impl ToInputArray, kernel: &impl ToInputArray, output: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(matrix);
	input_array_arg!(kernel);
	output_array_arg!(output);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT12D_process_const__InputArrayR_const__InputArrayR_const__OutputArrayR(matrix.as_raw__InputArray(), kernel.as_raw__InputArray(), output.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Computes ![inline formula](https://latex.codecogs.com/png.latex?F%5E1)-transfrom and inverse ![inline formula](https://latex.codecogs.com/png.latex?F%5E1)-transfrom at once.
/// ## Parameters
/// * matrix: Input matrix.
/// * kernel: Kernel used for processing. Function `ft::createKernel` can be used.
/// * output: Output 32-bit float array.
/// * mask: Mask used for unwanted area marking.
///
/// This function computes ![inline formula](https://latex.codecogs.com/png.latex?F%5E1)-transfrom and inverse ![inline formula](https://latex.codecogs.com/png.latex?F%5E1)-transfotm in one step. It is fully sufficient and optimized for `cv::Mat`.
///
///
/// Note:
///    F-transform technique of first degreee is described in paper [Vlas:FT](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Vlas:FT).
///
/// ## C++ default parameters
/// * mask: noArray()
// FT12D_process(InputArray, InputArray, OutputArray, InputArray)(InputArray, InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_F1_math.hpp:107
// ("cv::ft::FT12D_process", vec![(pred!(mut, ["matrix", "kernel", "output", "mask"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn ft1_2d_process(matrix: &impl ToInputArray, kernel: &impl ToInputArray, output: &mut impl ToOutputArray, mask: &impl ToInputArray) -> Result<()> {
	input_array_arg!(matrix);
	input_array_arg!(kernel);
	output_array_arg!(output);
	input_array_arg!(mask);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_FT12D_process_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(matrix.as_raw__InputArray(), kernel.as_raw__InputArray(), output.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Creates kernel from basic functions.
/// ## Parameters
/// * A: Basic function used in axis **x**.
/// * B: Basic function used in axis **y**.
/// * kernel: Final 32-bit kernel derived from **A** and **B**.
/// * chn: Number of kernel channels.
///
/// The function creates kernel usable for latter fuzzy image processing.
// createKernel(InputArray, InputArray, OutputArray, const int)(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_image.hpp:64
// ("cv::ft::createKernel", vec![(pred!(mut, ["A", "B", "kernel", "chn"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const int"]), _)]),
#[inline]
pub fn create_kernel1(a: &impl ToInputArray, b: &impl ToInputArray, kernel: &mut impl ToOutputArray, chn: i32) -> Result<()> {
	input_array_arg!(a);
	input_array_arg!(b);
	output_array_arg!(kernel);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_createKernel_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_int(a.as_raw__InputArray(), b.as_raw__InputArray(), kernel.as_raw__OutputArray(), chn, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Creates kernel from general functions.
/// ## Parameters
/// * function: Function type could be one of the following:
///    *   **LINEAR** Linear basic function.
/// * radius: Radius of the basic function.
/// * kernel: Final 32-bit kernel.
/// * chn: Number of kernel channels.
///
/// The function creates kernel from predefined functions.
// createKernel(int, int, OutputArray, const int)(Primitive, Primitive, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_image.hpp:75
// ("cv::ft::createKernel", vec![(pred!(mut, ["function", "radius", "kernel", "chn"], ["int", "int", "const cv::_OutputArray*", "const int"]), _)]),
#[inline]
pub fn create_kernel(function: i32, radius: i32, kernel: &mut impl ToOutputArray, chn: i32) -> Result<()> {
	output_array_arg!(kernel);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_createKernel_int_int_const__OutputArrayR_const_int(function, radius, kernel.as_raw__OutputArray(), chn, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Image filtering
/// ## Parameters
/// * image: Input image.
/// * kernel: Final 32-bit kernel.
/// * output: Output 32-bit image.
///
/// Filtering of the input image by means of F-transform.
// filter(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_image.hpp:103
// ("cv::ft::filter", vec![(pred!(mut, ["image", "kernel", "output"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
#[inline]
pub fn filter(image: &impl ToInputArray, kernel: &impl ToInputArray, output: &mut impl ToOutputArray) -> Result<()> {
	input_array_arg!(image);
	input_array_arg!(kernel);
	output_array_arg!(output);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR(image.as_raw__InputArray(), kernel.as_raw__InputArray(), output.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Image inpainting
/// ## Parameters
/// * image: Input image.
/// * mask: Mask used for unwanted area marking.
/// * output: Output 32-bit image.
/// * radius: Radius of the basic function.
/// * function: Function type could be one of the following:
///    *   `ft::LINEAR` Linear basic function.
/// * algorithm: Algorithm could be one of the following:
///    *   `ft::ONE_STEP` One step algorithm.
///    *   `ft::MULTI_STEP` This algorithm automaticaly increases radius of the basic function.
///    *   `ft::ITERATIVE` Iterative algorithm running in more steps using partial computations.
///
/// This function provides inpainting technique based on the fuzzy mathematic.
///
///
/// Note:
///    The algorithms are described in paper [Perf:rec](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Perf:rec).
// inpaint(InputArray, InputArray, OutputArray, int, int, int)(InputArray, InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/fuzzy/fuzzy_image.hpp:94
// ("cv::ft::inpaint", vec![(pred!(mut, ["image", "mask", "output", "radius", "function", "algorithm"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
#[inline]
pub fn inpaint(image: &impl ToInputArray, mask: &impl ToInputArray, output: &mut impl ToOutputArray, radius: i32, function: i32, algorithm: i32) -> Result<()> {
	input_array_arg!(image);
	input_array_arg!(mask);
	output_array_arg!(output);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ft_inpaint_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int(image.as_raw__InputArray(), mask.as_raw__InputArray(), output.as_raw__OutputArray(), radius, function, algorithm, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}
