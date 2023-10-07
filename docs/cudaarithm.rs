pub mod cudaarithm {
	//! # Operations on Matrices
	//!       # Core Operations on Matrices
	//!       # Per-element Operations
	//!       # Matrix Reductions
	//!       # Arithm Operations on Matrices
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::LookUpTableTraitConst, super::LookUpTableTrait, super::DFTTraitConst, super::DFTTrait, super::ConvolutionTraitConst, super::ConvolutionTrait };
	}
	
	/// Returns the sum of absolute values for matrix elements.
	/// 
	/// ## Parameters
	/// * src: Source image of any depth except for CV_64F .
	/// * mask: optional operation mask; it must have the same size as src1 and CV_8UC1 type.
	/// 
	/// ## Note
	/// This alternative version of [abs_sum] function uses the following default values for its arguments:
	/// * mask: noArray()
	#[inline]
	pub fn abs_sum_def(src: &impl core::ToInputArray) -> Result<core::Scalar> {
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_absSum_const__InputArrayR(src.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns the sum of absolute values for matrix elements.
	/// 
	/// ## Parameters
	/// * src: Source image of any depth except for CV_64F .
	/// * mask: optional operation mask; it must have the same size as src1 and CV_8UC1 type.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	#[inline]
	pub fn abs_sum(src: &impl core::ToInputArray, mask: &impl core::ToInputArray) -> Result<core::Scalar> {
		input_array_arg!(src);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_absSum_const__InputArrayR_const__InputArrayR(src.as_raw__InputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes an absolute value of each matrix element.
	/// 
	/// ## Parameters
	/// * src: Source matrix.
	/// * dst: Destination matrix with the same size and type as src .
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// abs
	/// 
	/// ## Note
	/// This alternative version of [abs] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn abs_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_abs_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes an absolute value of each matrix element.
	/// 
	/// ## Parameters
	/// * src: Source matrix.
	/// * dst: Destination matrix with the same size and type as src .
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// abs
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn abs(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_abs_const__InputArrayR_const__OutputArrayR_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes per-element absolute difference of two matrices (or of a matrix and scalar).
	/// 
	/// ## Parameters
	/// * src1: First source matrix or scalar.
	/// * src2: Second source matrix or scalar.
	/// * dst: Destination matrix that has the same size and type as the input array(s).
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// absdiff
	/// 
	/// ## Note
	/// This alternative version of [absdiff] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn absdiff_def(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_absdiff_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes per-element absolute difference of two matrices (or of a matrix and scalar).
	/// 
	/// ## Parameters
	/// * src1: First source matrix or scalar.
	/// * src2: Second source matrix or scalar.
	/// * dst: Destination matrix that has the same size and type as the input array(s).
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// absdiff
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn absdiff(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_absdiff_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes the weighted sum of two arrays.
	/// 
	/// ## Parameters
	/// * src1: First source array.
	/// * alpha: Weight for the first array elements.
	/// * src2: Second source array of the same size and channel number as src1 .
	/// * beta: Weight for the second array elements.
	/// * dst: Destination array that has the same size and number of channels as the input arrays.
	/// * gamma: Scalar added to each sum.
	/// * dtype: Optional depth of the destination array. When both input arrays have the same depth,
	/// dtype can be set to -1, which will be equivalent to src1.depth().
	/// * stream: Stream for the asynchronous version.
	/// 
	/// The function addWeighted calculates the weighted sum of two arrays as follows:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Bsaturate%7D%20%28%20%5Ctexttt%7Bsrc1%7D%20%28I%29%2A%20%5Ctexttt%7Balpha%7D%20%2B%20%20%5Ctexttt%7Bsrc2%7D%20%28I%29%2A%20%5Ctexttt%7Bbeta%7D%20%2B%20%20%5Ctexttt%7Bgamma%7D%20%29)
	/// 
	/// where I is a multi-dimensional index of array elements. In case of multi-channel arrays, each
	/// channel is processed independently.
	/// ## See also
	/// addWeighted
	/// 
	/// ## Note
	/// This alternative version of [add_weighted] function uses the following default values for its arguments:
	/// * dtype: -1
	/// * stream: Stream::Null()
	#[inline]
	pub fn add_weighted_def(src1: &impl core::ToInputArray, alpha: f64, src2: &impl core::ToInputArray, beta: f64, gamma: f64, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_addWeighted_const__InputArrayR_double_const__InputArrayR_double_double_const__OutputArrayR(src1.as_raw__InputArray(), alpha, src2.as_raw__InputArray(), beta, gamma, dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes the weighted sum of two arrays.
	/// 
	/// ## Parameters
	/// * src1: First source array.
	/// * alpha: Weight for the first array elements.
	/// * src2: Second source array of the same size and channel number as src1 .
	/// * beta: Weight for the second array elements.
	/// * dst: Destination array that has the same size and number of channels as the input arrays.
	/// * gamma: Scalar added to each sum.
	/// * dtype: Optional depth of the destination array. When both input arrays have the same depth,
	/// dtype can be set to -1, which will be equivalent to src1.depth().
	/// * stream: Stream for the asynchronous version.
	/// 
	/// The function addWeighted calculates the weighted sum of two arrays as follows:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Bsaturate%7D%20%28%20%5Ctexttt%7Bsrc1%7D%20%28I%29%2A%20%5Ctexttt%7Balpha%7D%20%2B%20%20%5Ctexttt%7Bsrc2%7D%20%28I%29%2A%20%5Ctexttt%7Bbeta%7D%20%2B%20%20%5Ctexttt%7Bgamma%7D%20%29)
	/// 
	/// where I is a multi-dimensional index of array elements. In case of multi-channel arrays, each
	/// channel is processed independently.
	/// ## See also
	/// addWeighted
	/// 
	/// ## C++ default parameters
	/// * dtype: -1
	/// * stream: Stream::Null()
	#[inline]
	pub fn add_weighted(src1: &impl core::ToInputArray, alpha: f64, src2: &impl core::ToInputArray, beta: f64, gamma: f64, dst: &mut impl core::ToOutputArray, dtype: i32, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_addWeighted_const__InputArrayR_double_const__InputArrayR_double_double_const__OutputArrayR_int_StreamR(src1.as_raw__InputArray(), alpha, src2.as_raw__InputArray(), beta, gamma, dst.as_raw__OutputArray(), dtype, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a matrix-matrix or matrix-scalar sum.
	/// 
	/// ## Parameters
	/// * src1: First source matrix or scalar.
	/// * src2: Second source matrix or scalar. Matrix should have the same size and type as src1 .
	/// * dst: Destination matrix that has the same size and number of channels as the input array(s).
	/// The depth is defined by dtype or src1 depth.
	/// * mask: Optional operation mask, 8-bit single channel array, that specifies elements of the
	/// destination array to be changed. The mask can be used only with single channel images.
	/// * dtype: Optional depth of the output array.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// add
	/// 
	/// ## Note
	/// This alternative version of [add] function uses the following default values for its arguments:
	/// * mask: noArray()
	/// * dtype: -1
	/// * stream: Stream::Null()
	#[inline]
	pub fn add_def(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_add_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a matrix-matrix or matrix-scalar sum.
	/// 
	/// ## Parameters
	/// * src1: First source matrix or scalar.
	/// * src2: Second source matrix or scalar. Matrix should have the same size and type as src1 .
	/// * dst: Destination matrix that has the same size and number of channels as the input array(s).
	/// The depth is defined by dtype or src1 depth.
	/// * mask: Optional operation mask, 8-bit single channel array, that specifies elements of the
	/// destination array to be changed. The mask can be used only with single channel images.
	/// * dtype: Optional depth of the output array.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// add
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * dtype: -1
	/// * stream: Stream::Null()
	#[inline]
	pub fn add(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, mask: &impl core::ToInputArray, dtype: i32, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_add_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int_StreamR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), dtype, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Performs a per-element bitwise conjunction of two matrices (or of matrix and scalar).
	/// 
	/// ## Parameters
	/// * src1: First source matrix or scalar.
	/// * src2: Second source matrix or scalar.
	/// * dst: Destination matrix that has the same size and type as the input array(s).
	/// * mask: Optional operation mask, 8-bit single channel array, that specifies elements of the
	/// destination array to be changed. The mask can be used only with single channel images.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## Note
	/// This alternative version of [bitwise_and] function uses the following default values for its arguments:
	/// * mask: noArray()
	/// * stream: Stream::Null()
	#[inline]
	pub fn bitwise_and_def(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_bitwise_and_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Performs a per-element bitwise conjunction of two matrices (or of matrix and scalar).
	/// 
	/// ## Parameters
	/// * src1: First source matrix or scalar.
	/// * src2: Second source matrix or scalar.
	/// * dst: Destination matrix that has the same size and type as the input array(s).
	/// * mask: Optional operation mask, 8-bit single channel array, that specifies elements of the
	/// destination array to be changed. The mask can be used only with single channel images.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * stream: Stream::Null()
	#[inline]
	pub fn bitwise_and(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, mask: &impl core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_bitwise_and_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Performs a per-element bitwise inversion.
	/// 
	/// ## Parameters
	/// * src: Source matrix.
	/// * dst: Destination matrix with the same size and type as src .
	/// * mask: Optional operation mask, 8-bit single channel array, that specifies elements of the
	/// destination array to be changed. The mask can be used only with single channel images.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## Note
	/// This alternative version of [bitwise_not] function uses the following default values for its arguments:
	/// * mask: noArray()
	/// * stream: Stream::Null()
	#[inline]
	pub fn bitwise_not_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_bitwise_not_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Performs a per-element bitwise inversion.
	/// 
	/// ## Parameters
	/// * src: Source matrix.
	/// * dst: Destination matrix with the same size and type as src .
	/// * mask: Optional operation mask, 8-bit single channel array, that specifies elements of the
	/// destination array to be changed. The mask can be used only with single channel images.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * stream: Stream::Null()
	#[inline]
	pub fn bitwise_not(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, mask: &impl core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_bitwise_not_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Performs a per-element bitwise disjunction of two matrices (or of matrix and scalar).
	/// 
	/// ## Parameters
	/// * src1: First source matrix or scalar.
	/// * src2: Second source matrix or scalar.
	/// * dst: Destination matrix that has the same size and type as the input array(s).
	/// * mask: Optional operation mask, 8-bit single channel array, that specifies elements of the
	/// destination array to be changed. The mask can be used only with single channel images.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## Note
	/// This alternative version of [bitwise_or] function uses the following default values for its arguments:
	/// * mask: noArray()
	/// * stream: Stream::Null()
	#[inline]
	pub fn bitwise_or_def(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_bitwise_or_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Performs a per-element bitwise disjunction of two matrices (or of matrix and scalar).
	/// 
	/// ## Parameters
	/// * src1: First source matrix or scalar.
	/// * src2: Second source matrix or scalar.
	/// * dst: Destination matrix that has the same size and type as the input array(s).
	/// * mask: Optional operation mask, 8-bit single channel array, that specifies elements of the
	/// destination array to be changed. The mask can be used only with single channel images.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * stream: Stream::Null()
	#[inline]
	pub fn bitwise_or(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, mask: &impl core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_bitwise_or_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Performs a per-element bitwise exclusive or operation of two matrices (or of matrix and scalar).
	/// 
	/// ## Parameters
	/// * src1: First source matrix or scalar.
	/// * src2: Second source matrix or scalar.
	/// * dst: Destination matrix that has the same size and type as the input array(s).
	/// * mask: Optional operation mask, 8-bit single channel array, that specifies elements of the
	/// destination array to be changed. The mask can be used only with single channel images.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## Note
	/// This alternative version of [bitwise_xor] function uses the following default values for its arguments:
	/// * mask: noArray()
	/// * stream: Stream::Null()
	#[inline]
	pub fn bitwise_xor_def(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_bitwise_xor_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Performs a per-element bitwise exclusive or operation of two matrices (or of matrix and scalar).
	/// 
	/// ## Parameters
	/// * src1: First source matrix or scalar.
	/// * src2: Second source matrix or scalar.
	/// * dst: Destination matrix that has the same size and type as the input array(s).
	/// * mask: Optional operation mask, 8-bit single channel array, that specifies elements of the
	/// destination array to be changed. The mask can be used only with single channel images.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * stream: Stream::Null()
	#[inline]
	pub fn bitwise_xor(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, mask: &impl core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_bitwise_xor_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// @overload
	/// 
	/// ## Note
	/// This alternative version of [calc_abs_sum] function uses the following default values for its arguments:
	/// * mask: noArray()
	/// * stream: Stream::Null()
	#[inline]
	pub fn calc_abs_sum_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_calcAbsSum_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * stream: Stream::Null()
	#[inline]
	pub fn calc_abs_sum(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, mask: &impl core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_calcAbsSum_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// @overload
	/// 
	/// ## Note
	/// This alternative version of [calc_norm_diff] function uses the following default values for its arguments:
	/// * norm_type: NORM_L2
	/// * stream: Stream::Null()
	#[inline]
	pub fn calc_norm_diff_def(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_calcNormDiff_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
	/// 
	/// ## C++ default parameters
	/// * norm_type: NORM_L2
	/// * stream: Stream::Null()
	#[inline]
	pub fn calc_norm_diff(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, norm_type: i32, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_calcNormDiff_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_StreamR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), norm_type, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// @overload
	/// 
	/// ## Note
	/// This alternative version of [calc_norm] function uses the following default values for its arguments:
	/// * mask: noArray()
	/// * stream: Stream::Null()
	#[inline]
	pub fn calc_norm_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, norm_type: i32) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_calcNorm_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), norm_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * stream: Stream::Null()
	#[inline]
	pub fn calc_norm(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, norm_type: i32, mask: &impl core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_calcNorm_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), norm_type, mask.as_raw__InputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// @overload
	/// 
	/// ## Note
	/// This alternative version of [calc_sqr_sum] function uses the following default values for its arguments:
	/// * mask: noArray()
	/// * stream: Stream::Null()
	#[inline]
	pub fn calc_sqr_sum_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_calcSqrSum_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * stream: Stream::Null()
	#[inline]
	pub fn calc_sqr_sum(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, mask: &impl core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_calcSqrSum_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// @overload
	/// 
	/// ## Note
	/// This alternative version of [calc_sum] function uses the following default values for its arguments:
	/// * mask: noArray()
	/// * stream: Stream::Null()
	#[inline]
	pub fn calc_sum_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_calcSum_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * stream: Stream::Null()
	#[inline]
	pub fn calc_sum(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, mask: &impl core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_calcSum_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Converts Cartesian coordinates into polar.
	/// 
	/// ## Parameters
	/// * x: Source matrix containing real components ( CV_32FC1 ).
	/// * y: Source matrix containing imaginary components ( CV_32FC1 ).
	/// * magnitude: Destination matrix of float magnitudes ( CV_32FC1 ).
	/// * angle: Destination matrix of angles ( CV_32FC1 ).
	/// * angleInDegrees: Flag for angles that must be evaluated in degrees.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// cartToPolar
	/// 
	/// ## Note
	/// This alternative version of [cart_to_polar] function uses the following default values for its arguments:
	/// * angle_in_degrees: false
	/// * stream: Stream::Null()
	#[inline]
	pub fn cart_to_polar_def(x: &impl core::ToInputArray, y: &impl core::ToInputArray, magnitude: &mut impl core::ToOutputArray, angle: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(x);
		input_array_arg!(y);
		output_array_arg!(magnitude);
		output_array_arg!(angle);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_cartToPolar_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(x.as_raw__InputArray(), y.as_raw__InputArray(), magnitude.as_raw__OutputArray(), angle.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Converts Cartesian coordinates into polar.
	/// 
	/// ## Parameters
	/// * x: Source matrix containing real components ( CV_32FC1 ).
	/// * y: Source matrix containing imaginary components ( CV_32FC1 ).
	/// * magnitude: Destination matrix of float magnitudes ( CV_32FC1 ).
	/// * angle: Destination matrix of angles ( CV_32FC1 ).
	/// * angleInDegrees: Flag for angles that must be evaluated in degrees.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// cartToPolar
	/// 
	/// ## C++ default parameters
	/// * angle_in_degrees: false
	/// * stream: Stream::Null()
	#[inline]
	pub fn cart_to_polar(x: &impl core::ToInputArray, y: &impl core::ToInputArray, magnitude: &mut impl core::ToOutputArray, angle: &mut impl core::ToOutputArray, angle_in_degrees: bool, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(x);
		input_array_arg!(y);
		output_array_arg!(magnitude);
		output_array_arg!(angle);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_cartToPolar_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_StreamR(x.as_raw__InputArray(), y.as_raw__InputArray(), magnitude.as_raw__OutputArray(), angle.as_raw__OutputArray(), angle_in_degrees, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Compares elements of two matrices (or of a matrix and scalar).
	/// 
	/// ## Parameters
	/// * src1: First source matrix or scalar.
	/// * src2: Second source matrix or scalar.
	/// * dst: Destination matrix that has the same size as the input array(s) and type CV_8U.
	/// * cmpop: Flag specifying the relation between the elements to be checked:
	/// *   **CMP_EQ:** a(.) == b(.)
	/// *   **CMP_GT:** a(.) \> b(.)
	/// *   **CMP_GE:** a(.) \>= b(.)
	/// *   **CMP_LT:** a(.) \< b(.)
	/// *   **CMP_LE:** a(.) \<= b(.)
	/// *   **CMP_NE:** a(.) != b(.)
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// compare
	/// 
	/// ## Note
	/// This alternative version of [compare] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn compare_def(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, cmpop: i32) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_compare_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), cmpop, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Compares elements of two matrices (or of a matrix and scalar).
	/// 
	/// ## Parameters
	/// * src1: First source matrix or scalar.
	/// * src2: Second source matrix or scalar.
	/// * dst: Destination matrix that has the same size as the input array(s) and type CV_8U.
	/// * cmpop: Flag specifying the relation between the elements to be checked:
	/// *   **CMP_EQ:** a(.) == b(.)
	/// *   **CMP_GT:** a(.) \> b(.)
	/// *   **CMP_GE:** a(.) \>= b(.)
	/// *   **CMP_LT:** a(.) \< b(.)
	/// *   **CMP_LE:** a(.) \<= b(.)
	/// *   **CMP_NE:** a(.) != b(.)
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// compare
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn compare(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, cmpop: i32, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_compare_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_StreamR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), cmpop, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Forms a border around an image.
	/// 
	/// ## Parameters
	/// * src: Source image. CV_8UC1 , CV_8UC4 , CV_32SC1 , and CV_32FC1 types are supported.
	/// * dst: Destination image with the same type as src. The size is
	/// Size(src.cols+left+right, src.rows+top+bottom) .
	/// * top: Number of top pixels
	/// * bottom: Number of bottom pixels
	/// * left: Number of left pixels
	/// * right: Number of pixels in each direction from the source image rectangle to extrapolate.
	/// For example: top=1, bottom=1, left=1, right=1 mean that 1 pixel-wide border needs to be built.
	/// * borderType: Border type. See borderInterpolate for details. BORDER_REFLECT101 ,
	/// BORDER_REPLICATE , BORDER_CONSTANT , BORDER_REFLECT and BORDER_WRAP are supported for now.
	/// * value: Border value.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## Note
	/// This alternative version of [copy_make_border] function uses the following default values for its arguments:
	/// * value: Scalar()
	/// * stream: Stream::Null()
	#[inline]
	pub fn copy_make_border_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, top: i32, bottom: i32, left: i32, right: i32, border_type: i32) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_copyMakeBorder_const__InputArrayR_const__OutputArrayR_int_int_int_int_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), top, bottom, left, right, border_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Forms a border around an image.
	/// 
	/// ## Parameters
	/// * src: Source image. CV_8UC1 , CV_8UC4 , CV_32SC1 , and CV_32FC1 types are supported.
	/// * dst: Destination image with the same type as src. The size is
	/// Size(src.cols+left+right, src.rows+top+bottom) .
	/// * top: Number of top pixels
	/// * bottom: Number of bottom pixels
	/// * left: Number of left pixels
	/// * right: Number of pixels in each direction from the source image rectangle to extrapolate.
	/// For example: top=1, bottom=1, left=1, right=1 mean that 1 pixel-wide border needs to be built.
	/// * borderType: Border type. See borderInterpolate for details. BORDER_REFLECT101 ,
	/// BORDER_REPLICATE , BORDER_CONSTANT , BORDER_REFLECT and BORDER_WRAP are supported for now.
	/// * value: Border value.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * value: Scalar()
	/// * stream: Stream::Null()
	#[inline]
	pub fn copy_make_border(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, top: i32, bottom: i32, left: i32, right: i32, border_type: i32, value: core::Scalar, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_copyMakeBorder_const__InputArrayR_const__OutputArrayR_int_int_int_int_int_Scalar_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), top, bottom, left, right, border_type, value.opencv_as_extern(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Counts non-zero matrix elements.
	/// 
	/// ## Parameters
	/// * src: Single-channel source image.
	/// 
	/// The function does not work with CV_64F images on GPUs with the compute capability \< 1.3.
	/// ## See also
	/// countNonZero
	#[inline]
	pub fn count_non_zero(src: &impl core::ToInputArray) -> Result<i32> {
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_countNonZero_const__InputArrayR(src.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// @overload
	/// 
	/// ## Note
	/// This alternative version of [count_non_zero_1] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn count_non_zero_1_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_countNonZero_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Counts non-zero matrix elements.
	/// 
	/// ## Parameters
	/// * src: Single-channel source image.
	/// 
	/// The function does not work with CV_64F images on GPUs with the compute capability \< 1.3.
	/// ## See also
	/// countNonZero
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn count_non_zero_1(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_countNonZero_const__InputArrayR_const__OutputArrayR_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Creates implementation for cuda::Convolution .
	/// 
	/// ## Parameters
	/// * user_block_size: Block size. If you leave default value Size(0,0) then automatic
	/// estimation of block size will be used (which is optimized for speed). By varying user_block_size
	/// you can reduce memory requirements at the cost of speed.
	/// 
	/// ## Note
	/// This alternative version of [create_convolution] function uses the following default values for its arguments:
	/// * user_block_size: Size()
	#[inline]
	pub fn create_convolution_def() -> Result<core::Ptr<crate::cudaarithm::Convolution>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createConvolution(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudaarithm::Convolution>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates implementation for cuda::Convolution .
	/// 
	/// ## Parameters
	/// * user_block_size: Block size. If you leave default value Size(0,0) then automatic
	/// estimation of block size will be used (which is optimized for speed). By varying user_block_size
	/// you can reduce memory requirements at the cost of speed.
	/// 
	/// ## C++ default parameters
	/// * user_block_size: Size()
	#[inline]
	pub fn create_convolution(user_block_size: core::Size) -> Result<core::Ptr<crate::cudaarithm::Convolution>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createConvolution_Size(user_block_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudaarithm::Convolution>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates implementation for cuda::DFT.
	/// 
	/// ## Parameters
	/// * dft_size: The image size.
	/// * flags: Optional flags:
	/// *   **DFT_ROWS** transforms each individual row of the source matrix.
	/// *   **DFT_SCALE** scales the result: divide it by the number of elements in the transform
	/// (obtained from dft_size ).
	/// *   **DFT_INVERSE** inverts DFT. Use for complex-complex cases (real-complex and complex-real
	/// cases are always forward and inverse, respectively).
	/// *   **DFT_COMPLEX_INPUT** Specifies that inputs will be complex with 2 channels.
	/// *   **DFT_REAL_OUTPUT** specifies the output as real. The source matrix is the result of
	/// real-complex transform, so the destination matrix must be real.
	#[inline]
	pub fn create_dft(dft_size: core::Size, flags: i32) -> Result<core::Ptr<crate::cudaarithm::DFT>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createDFT_Size_int(dft_size.opencv_as_extern(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudaarithm::DFT>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates implementation for cuda::LookUpTable .
	/// 
	/// ## Parameters
	/// * lut: Look-up table of 256 elements. It is a continuous CV_8U matrix.
	#[inline]
	pub fn create_look_up_table(lut: &impl core::ToInputArray) -> Result<core::Ptr<crate::cudaarithm::LookUpTable>> {
		input_array_arg!(lut);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createLookUpTable_const__InputArrayR(lut.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudaarithm::LookUpTable>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Performs a forward or inverse discrete Fourier transform (1D or 2D) of the floating point matrix.
	/// 
	/// ## Parameters
	/// * src: Source matrix (real or complex).
	/// * dst: Destination matrix (real or complex).
	/// * dft_size: Size of a discrete Fourier transform.
	/// * flags: Optional flags:
	/// *   **DFT_ROWS** transforms each individual row of the source matrix.
	/// *   **DFT_SCALE** scales the result: divide it by the number of elements in the transform
	/// (obtained from dft_size ).
	/// *   **DFT_INVERSE** inverts DFT. Use for complex-complex cases (real-complex and complex-real
	/// cases are always forward and inverse, respectively).
	/// *   **DFT_COMPLEX_INPUT** Specifies that input is complex input with 2 channels.
	/// *   **DFT_REAL_OUTPUT** specifies the output as real. The source matrix is the result of
	/// real-complex transform, so the destination matrix must be real.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// Use to handle real matrices ( CV32FC1 ) and complex matrices in the interleaved format ( CV32FC2 ).
	/// 
	/// The source matrix should be continuous, otherwise reallocation and data copying is performed. The
	/// function chooses an operation mode depending on the flags, size, and channel count of the source
	/// matrix:
	/// 
	/// *   If the source matrix is complex and the output is not specified as real, the destination
	/// matrix is complex and has the dft_size size and CV_32FC2 type. The destination matrix
	/// contains a full result of the DFT (forward or inverse).
	/// *   If the source matrix is complex and the output is specified as real, the function assumes that
	/// its input is the result of the forward transform (see the next item). The destination matrix
	/// has the dft_size size and CV_32FC1 type. It contains the result of the inverse DFT.
	/// *   If the source matrix is real (its type is CV_32FC1 ), forward DFT is performed. The result of
	/// the DFT is packed into complex ( CV_32FC2 ) matrix. So, the width of the destination matrix
	/// is dft_size.width / 2 + 1 . But if the source is a single column, the height is reduced
	/// instead of the width.
	/// ## See also
	/// dft
	/// 
	/// ## Note
	/// This alternative version of [dft] function uses the following default values for its arguments:
	/// * flags: 0
	/// * stream: Stream::Null()
	#[inline]
	pub fn dft_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, dft_size: core::Size) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_dft_const__InputArrayR_const__OutputArrayR_Size(src.as_raw__InputArray(), dst.as_raw__OutputArray(), dft_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Performs a forward or inverse discrete Fourier transform (1D or 2D) of the floating point matrix.
	/// 
	/// ## Parameters
	/// * src: Source matrix (real or complex).
	/// * dst: Destination matrix (real or complex).
	/// * dft_size: Size of a discrete Fourier transform.
	/// * flags: Optional flags:
	/// *   **DFT_ROWS** transforms each individual row of the source matrix.
	/// *   **DFT_SCALE** scales the result: divide it by the number of elements in the transform
	/// (obtained from dft_size ).
	/// *   **DFT_INVERSE** inverts DFT. Use for complex-complex cases (real-complex and complex-real
	/// cases are always forward and inverse, respectively).
	/// *   **DFT_COMPLEX_INPUT** Specifies that input is complex input with 2 channels.
	/// *   **DFT_REAL_OUTPUT** specifies the output as real. The source matrix is the result of
	/// real-complex transform, so the destination matrix must be real.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// Use to handle real matrices ( CV32FC1 ) and complex matrices in the interleaved format ( CV32FC2 ).
	/// 
	/// The source matrix should be continuous, otherwise reallocation and data copying is performed. The
	/// function chooses an operation mode depending on the flags, size, and channel count of the source
	/// matrix:
	/// 
	/// *   If the source matrix is complex and the output is not specified as real, the destination
	/// matrix is complex and has the dft_size size and CV_32FC2 type. The destination matrix
	/// contains a full result of the DFT (forward or inverse).
	/// *   If the source matrix is complex and the output is specified as real, the function assumes that
	/// its input is the result of the forward transform (see the next item). The destination matrix
	/// has the dft_size size and CV_32FC1 type. It contains the result of the inverse DFT.
	/// *   If the source matrix is real (its type is CV_32FC1 ), forward DFT is performed. The result of
	/// the DFT is packed into complex ( CV_32FC2 ) matrix. So, the width of the destination matrix
	/// is dft_size.width / 2 + 1 . But if the source is a single column, the height is reduced
	/// instead of the width.
	/// ## See also
	/// dft
	/// 
	/// ## C++ default parameters
	/// * flags: 0
	/// * stream: Stream::Null()
	#[inline]
	pub fn dft(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, dft_size: core::Size, flags: i32, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_dft_const__InputArrayR_const__OutputArrayR_Size_int_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), dft_size.opencv_as_extern(), flags, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a matrix-matrix or matrix-scalar division.
	/// 
	/// ## Parameters
	/// * src1: First source matrix or a scalar.
	/// * src2: Second source matrix or scalar.
	/// * dst: Destination matrix that has the same size and number of channels as the input array(s).
	/// The depth is defined by dtype or src1 depth.
	/// * scale: Optional scale factor.
	/// * dtype: Optional depth of the output array.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// This function, in contrast to divide, uses a round-down rounding mode.
	/// ## See also
	/// divide
	/// 
	/// ## Note
	/// This alternative version of [divide] function uses the following default values for its arguments:
	/// * scale: 1
	/// * dtype: -1
	/// * stream: Stream::Null()
	#[inline]
	pub fn divide_def(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_divide_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a matrix-matrix or matrix-scalar division.
	/// 
	/// ## Parameters
	/// * src1: First source matrix or a scalar.
	/// * src2: Second source matrix or scalar.
	/// * dst: Destination matrix that has the same size and number of channels as the input array(s).
	/// The depth is defined by dtype or src1 depth.
	/// * scale: Optional scale factor.
	/// * dtype: Optional depth of the output array.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// This function, in contrast to divide, uses a round-down rounding mode.
	/// ## See also
	/// divide
	/// 
	/// ## C++ default parameters
	/// * scale: 1
	/// * dtype: -1
	/// * stream: Stream::Null()
	#[inline]
	pub fn divide(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, scale: f64, dtype: i32, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_divide_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int_StreamR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), scale, dtype, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes an exponent of each matrix element.
	/// 
	/// ## Parameters
	/// * src: Source matrix.
	/// * dst: Destination matrix with the same size and type as src .
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// exp
	/// 
	/// ## Note
	/// This alternative version of [exp] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn exp_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_exp_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes an exponent of each matrix element.
	/// 
	/// ## Parameters
	/// * src: Source matrix.
	/// * dst: Destination matrix with the same size and type as src .
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// exp
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn exp(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_exp_const__InputArrayR_const__OutputArrayR_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// @overload
	/// 
	/// ## Note
	/// This alternative version of [find_min_max_loc] function uses the following default values for its arguments:
	/// * mask: noArray()
	/// * stream: Stream::Null()
	#[inline]
	pub fn find_min_max_loc_def(src: &impl core::ToInputArray, min_max_vals: &mut impl core::ToOutputArray, loc: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(min_max_vals);
		output_array_arg!(loc);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_findMinMaxLoc_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src.as_raw__InputArray(), min_max_vals.as_raw__OutputArray(), loc.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * stream: Stream::Null()
	#[inline]
	pub fn find_min_max_loc(src: &impl core::ToInputArray, min_max_vals: &mut impl core::ToOutputArray, loc: &mut impl core::ToOutputArray, mask: &impl core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(min_max_vals);
		output_array_arg!(loc);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_findMinMaxLoc_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src.as_raw__InputArray(), min_max_vals.as_raw__OutputArray(), loc.as_raw__OutputArray(), mask.as_raw__InputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// @overload
	/// 
	/// ## Note
	/// This alternative version of [find_min_max] function uses the following default values for its arguments:
	/// * mask: noArray()
	/// * stream: Stream::Null()
	#[inline]
	pub fn find_min_max_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_findMinMax_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * stream: Stream::Null()
	#[inline]
	pub fn find_min_max(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, mask: &impl core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_findMinMax_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Flips a 2D matrix around vertical, horizontal, or both axes.
	/// 
	/// ## Parameters
	/// * src: Source matrix. Supports 1, 3 and 4 channels images with CV_8U, CV_16U, CV_32S or
	/// CV_32F depth.
	/// * dst: Destination matrix.
	/// * flipCode: Flip mode for the source:
	/// *   0 Flips around x-axis.
	/// *   \> 0 Flips around y-axis.
	/// *   \< 0 Flips around both axes.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// flip
	/// 
	/// ## Note
	/// This alternative version of [flip] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn flip_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, flip_code: i32) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_flip_const__InputArrayR_const__OutputArrayR_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), flip_code, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Flips a 2D matrix around vertical, horizontal, or both axes.
	/// 
	/// ## Parameters
	/// * src: Source matrix. Supports 1, 3 and 4 channels images with CV_8U, CV_16U, CV_32S or
	/// CV_32F depth.
	/// * dst: Destination matrix.
	/// * flipCode: Flip mode for the source:
	/// *   0 Flips around x-axis.
	/// *   \> 0 Flips around y-axis.
	/// *   \< 0 Flips around both axes.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// flip
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn flip(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, flip_code: i32, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_flip_const__InputArrayR_const__OutputArrayR_int_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), flip_code, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Performs generalized matrix multiplication.
	/// 
	/// ## Parameters
	/// * src1: First multiplied input matrix that should have CV_32FC1 , CV_64FC1 , CV_32FC2 , or
	/// CV_64FC2 type.
	/// * src2: Second multiplied input matrix of the same type as src1 .
	/// * alpha: Weight of the matrix product.
	/// * src3: Third optional delta matrix added to the matrix product. It should have the same type
	/// as src1 and src2 .
	/// * beta: Weight of src3 .
	/// * dst: Destination matrix. It has the proper size and the same type as input matrices.
	/// * flags: Operation flags:
	/// *   **GEMM_1_T** transpose src1
	/// *   **GEMM_2_T** transpose src2
	/// *   **GEMM_3_T** transpose src3
	/// * stream: Stream for the asynchronous version.
	/// 
	/// The function performs generalized matrix multiplication similar to the gemm functions in BLAS level
	/// 3. For example, gemm(src1, src2, alpha, src3, beta, dst, GEMM_1_T + GEMM_3_T) corresponds to
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%5Ctexttt%7Balpha%7D%20%5Ccdot%20%5Ctexttt%7Bsrc1%7D%20%5ET%20%20%5Ccdot%20%5Ctexttt%7Bsrc2%7D%20%2B%20%20%5Ctexttt%7Bbeta%7D%20%5Ccdot%20%5Ctexttt%7Bsrc3%7D%20%5ET)
	/// 
	/// 
	/// Note: Transposition operation doesn't support CV_64FC2 input type.
	/// ## See also
	/// gemm
	/// 
	/// ## Note
	/// This alternative version of [gemm] function uses the following default values for its arguments:
	/// * flags: 0
	/// * stream: Stream::Null()
	#[inline]
	pub fn gemm_def(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, alpha: f64, src3: &impl core::ToInputArray, beta: f64, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		input_array_arg!(src3);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_gemm_const__InputArrayR_const__InputArrayR_double_const__InputArrayR_double_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), alpha, src3.as_raw__InputArray(), beta, dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Performs generalized matrix multiplication.
	/// 
	/// ## Parameters
	/// * src1: First multiplied input matrix that should have CV_32FC1 , CV_64FC1 , CV_32FC2 , or
	/// CV_64FC2 type.
	/// * src2: Second multiplied input matrix of the same type as src1 .
	/// * alpha: Weight of the matrix product.
	/// * src3: Third optional delta matrix added to the matrix product. It should have the same type
	/// as src1 and src2 .
	/// * beta: Weight of src3 .
	/// * dst: Destination matrix. It has the proper size and the same type as input matrices.
	/// * flags: Operation flags:
	/// *   **GEMM_1_T** transpose src1
	/// *   **GEMM_2_T** transpose src2
	/// *   **GEMM_3_T** transpose src3
	/// * stream: Stream for the asynchronous version.
	/// 
	/// The function performs generalized matrix multiplication similar to the gemm functions in BLAS level
	/// 3. For example, gemm(src1, src2, alpha, src3, beta, dst, GEMM_1_T + GEMM_3_T) corresponds to
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%5Ctexttt%7Balpha%7D%20%5Ccdot%20%5Ctexttt%7Bsrc1%7D%20%5ET%20%20%5Ccdot%20%5Ctexttt%7Bsrc2%7D%20%2B%20%20%5Ctexttt%7Bbeta%7D%20%5Ccdot%20%5Ctexttt%7Bsrc3%7D%20%5ET)
	/// 
	/// 
	/// Note: Transposition operation doesn't support CV_64FC2 input type.
	/// ## See also
	/// gemm
	/// 
	/// ## C++ default parameters
	/// * flags: 0
	/// * stream: Stream::Null()
	#[inline]
	pub fn gemm(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, alpha: f64, src3: &impl core::ToInputArray, beta: f64, dst: &mut impl core::ToOutputArray, flags: i32, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		input_array_arg!(src3);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_gemm_const__InputArrayR_const__InputArrayR_double_const__InputArrayR_double_const__OutputArrayR_int_StreamR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), alpha, src3.as_raw__InputArray(), beta, dst.as_raw__OutputArray(), flags, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Checks if array elements lie between two scalars.
	/// 
	/// The function checks the range as follows:
	/// *   For every element of a single-channel input array:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Blowerb%7D%5F0%20%20%5Cleq%20%5Ctexttt%7Bsrc%7D%20%28I%29%5F0%20%5Cleq%20%20%5Ctexttt%7Bupperb%7D%5F0)
	/// *   For two-channel arrays:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Blowerb%7D%5F0%20%20%5Cleq%20%5Ctexttt%7Bsrc%7D%20%28I%29%5F0%20%5Cleq%20%20%5Ctexttt%7Bupperb%7D%5F0%20%20%5Cland%20%5Ctexttt%7Blowerb%7D%5F1%20%20%5Cleq%20%5Ctexttt%7Bsrc%7D%20%28I%29%5F1%20%5Cleq%20%20%5Ctexttt%7Bupperb%7D%5F1)
	/// *   and so forth.
	/// 
	/// That is, dst (I) is set to 255 (all 1 -bits) if src (I) is within the
	/// specified 1D, 2D, 3D, ... box and 0 otherwise.
	/// 
	/// Note that unlike the CPU inRange, this does NOT accept an array for lowerb or
	/// upperb, only a cv::Scalar.
	/// 
	/// ## Parameters
	/// * src: first input array.
	/// * lowerb: inclusive lower boundary cv::Scalar.
	/// * upperb: inclusive upper boundary cv::Scalar.
	/// * dst: output array of the same size as src and CV_8U type.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// cv::inRange
	/// 
	/// ## Note
	/// This alternative version of [in_range] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn in_range_def(src: &impl core::ToInputArray, lowerb: core::Scalar, upperb: core::Scalar, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_inRange_const__InputArrayR_const_ScalarR_const_ScalarR_const__OutputArrayR(src.as_raw__InputArray(), &lowerb, &upperb, dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Checks if array elements lie between two scalars.
	/// 
	/// The function checks the range as follows:
	/// *   For every element of a single-channel input array:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Blowerb%7D%5F0%20%20%5Cleq%20%5Ctexttt%7Bsrc%7D%20%28I%29%5F0%20%5Cleq%20%20%5Ctexttt%7Bupperb%7D%5F0)
	/// *   For two-channel arrays:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Blowerb%7D%5F0%20%20%5Cleq%20%5Ctexttt%7Bsrc%7D%20%28I%29%5F0%20%5Cleq%20%20%5Ctexttt%7Bupperb%7D%5F0%20%20%5Cland%20%5Ctexttt%7Blowerb%7D%5F1%20%20%5Cleq%20%5Ctexttt%7Bsrc%7D%20%28I%29%5F1%20%5Cleq%20%20%5Ctexttt%7Bupperb%7D%5F1)
	/// *   and so forth.
	/// 
	/// That is, dst (I) is set to 255 (all 1 -bits) if src (I) is within the
	/// specified 1D, 2D, 3D, ... box and 0 otherwise.
	/// 
	/// Note that unlike the CPU inRange, this does NOT accept an array for lowerb or
	/// upperb, only a cv::Scalar.
	/// 
	/// ## Parameters
	/// * src: first input array.
	/// * lowerb: inclusive lower boundary cv::Scalar.
	/// * upperb: inclusive upper boundary cv::Scalar.
	/// * dst: output array of the same size as src and CV_8U type.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// cv::inRange
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn in_range(src: &impl core::ToInputArray, lowerb: core::Scalar, upperb: core::Scalar, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_inRange_const__InputArrayR_const_ScalarR_const_ScalarR_const__OutputArrayR_StreamR(src.as_raw__InputArray(), &lowerb, &upperb, dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes an integral image.
	/// 
	/// ## Parameters
	/// * src: Source image. Only CV_8UC1 images are supported for now.
	/// * sum: Integral image containing 32-bit unsigned integer values packed into CV_32SC1 .
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// integral
	/// 
	/// ## Note
	/// This alternative version of [integral] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn integral_def(src: &impl core::ToInputArray, sum: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(sum);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_integral_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), sum.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes an integral image.
	/// 
	/// ## Parameters
	/// * src: Source image. Only CV_8UC1 images are supported for now.
	/// * sum: Integral image containing 32-bit unsigned integer values packed into CV_32SC1 .
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// integral
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn integral(src: &impl core::ToInputArray, sum: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(sum);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_integral_const__InputArrayR_const__OutputArrayR_StreamR(src.as_raw__InputArray(), sum.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a natural logarithm of absolute value of each matrix element.
	/// 
	/// ## Parameters
	/// * src: Source matrix.
	/// * dst: Destination matrix with the same size and type as src .
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// log
	/// 
	/// ## Note
	/// This alternative version of [log] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn log_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_log_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a natural logarithm of absolute value of each matrix element.
	/// 
	/// ## Parameters
	/// * src: Source matrix.
	/// * dst: Destination matrix with the same size and type as src .
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// log
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn log(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_log_const__InputArrayR_const__OutputArrayR_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Performs pixel by pixel right left of an image by a constant value.
	/// 
	/// ## Parameters
	/// * src: Source matrix. Supports 1, 3 and 4 channels images with CV_8U , CV_16U or CV_32S
	/// depth.
	/// * val: Constant values, one per channel.
	/// * dst: Destination matrix with the same size and type as src .
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## Note
	/// This alternative version of [lshift] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn lshift_def(src: &impl core::ToInputArray, val: core::Scalar_<i32>, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_lshift_const__InputArrayR_Scalar_LintG_const__OutputArrayR(src.as_raw__InputArray(), val.opencv_as_extern(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Performs pixel by pixel right left of an image by a constant value.
	/// 
	/// ## Parameters
	/// * src: Source matrix. Supports 1, 3 and 4 channels images with CV_8U , CV_16U or CV_32S
	/// depth.
	/// * val: Constant values, one per channel.
	/// * dst: Destination matrix with the same size and type as src .
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn lshift(src: &impl core::ToInputArray, val: core::Scalar_<i32>, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_lshift_const__InputArrayR_Scalar_LintG_const__OutputArrayR_StreamR(src.as_raw__InputArray(), val.opencv_as_extern(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [lshift_1] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn lshift_1_def(src: &impl core::ToInputArray, val: core::Scalar, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_lshift_const__InputArrayR_Scalar_const__OutputArrayR(src.as_raw__InputArray(), val.opencv_as_extern(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn lshift_1(src: &impl core::ToInputArray, val: core::Scalar, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_lshift_const__InputArrayR_Scalar_const__OutputArrayR_StreamR(src.as_raw__InputArray(), val.opencv_as_extern(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// @overload
	///  computes squared magnitude of each (x(i), y(i)) vector
	///  supports only floating-point source
	/// ## Parameters
	/// * x: Source matrix containing real components ( CV_32FC1 ).
	/// * y: Source matrix containing imaginary components ( CV_32FC1 ).
	/// * magnitude: Destination matrix of float magnitude squares ( CV_32FC1 ).
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## Note
	/// This alternative version of [magnitude_sqr_1] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn magnitude_sqr_1_def(x: &impl core::ToInputArray, y: &impl core::ToInputArray, magnitude: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(x);
		input_array_arg!(y);
		output_array_arg!(magnitude);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_magnitudeSqr_const__InputArrayR_const__InputArrayR_const__OutputArrayR(x.as_raw__InputArray(), y.as_raw__InputArray(), magnitude.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes squared magnitudes of complex matrix elements.
	/// 
	/// ## Parameters
	/// * xy: Source complex matrix in the interleaved format ( CV_32FC2 ).
	/// * magnitude: Destination matrix of float magnitude squares ( CV_32FC1 ).
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## Overloaded parameters
	/// 
	///  computes squared magnitude of each (x(i), y(i)) vector
	///  supports only floating-point source
	/// * x: Source matrix containing real components ( CV_32FC1 ).
	/// * y: Source matrix containing imaginary components ( CV_32FC1 ).
	/// * magnitude: Destination matrix of float magnitude squares ( CV_32FC1 ).
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn magnitude_sqr_1(x: &impl core::ToInputArray, y: &impl core::ToInputArray, magnitude: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(x);
		input_array_arg!(y);
		output_array_arg!(magnitude);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_magnitudeSqr_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(x.as_raw__InputArray(), y.as_raw__InputArray(), magnitude.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes squared magnitudes of complex matrix elements.
	/// 
	/// ## Parameters
	/// * xy: Source complex matrix in the interleaved format ( CV_32FC2 ).
	/// * magnitude: Destination matrix of float magnitude squares ( CV_32FC1 ).
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## Note
	/// This alternative version of [magnitude_sqr] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn magnitude_sqr_def(xy: &impl core::ToInputArray, magnitude: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(xy);
		output_array_arg!(magnitude);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_magnitudeSqr_const__InputArrayR_const__OutputArrayR(xy.as_raw__InputArray(), magnitude.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes squared magnitudes of complex matrix elements.
	/// 
	/// ## Parameters
	/// * xy: Source complex matrix in the interleaved format ( CV_32FC2 ).
	/// * magnitude: Destination matrix of float magnitude squares ( CV_32FC1 ).
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn magnitude_sqr(xy: &impl core::ToInputArray, magnitude: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(xy);
		output_array_arg!(magnitude);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_magnitudeSqr_const__InputArrayR_const__OutputArrayR_StreamR(xy.as_raw__InputArray(), magnitude.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// @overload
	///  computes magnitude of each (x(i), y(i)) vector
	///  supports only floating-point source
	/// ## Parameters
	/// * x: Source matrix containing real components ( CV_32FC1 ).
	/// * y: Source matrix containing imaginary components ( CV_32FC1 ).
	/// * magnitude: Destination matrix of float magnitudes ( CV_32FC1 ).
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## Note
	/// This alternative version of [magnitude_1] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn magnitude_1_def(x: &impl core::ToInputArray, y: &impl core::ToInputArray, magnitude: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(x);
		input_array_arg!(y);
		output_array_arg!(magnitude);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_magnitude_const__InputArrayR_const__InputArrayR_const__OutputArrayR(x.as_raw__InputArray(), y.as_raw__InputArray(), magnitude.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes magnitudes of complex matrix elements.
	/// 
	/// ## Parameters
	/// * xy: Source complex matrix in the interleaved format ( CV_32FC2 ).
	/// * magnitude: Destination matrix of float magnitudes ( CV_32FC1 ).
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// magnitude
	/// 
	/// ## Overloaded parameters
	/// 
	///  computes magnitude of each (x(i), y(i)) vector
	///  supports only floating-point source
	/// * x: Source matrix containing real components ( CV_32FC1 ).
	/// * y: Source matrix containing imaginary components ( CV_32FC1 ).
	/// * magnitude: Destination matrix of float magnitudes ( CV_32FC1 ).
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn magnitude_1(x: &impl core::ToInputArray, y: &impl core::ToInputArray, magnitude: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(x);
		input_array_arg!(y);
		output_array_arg!(magnitude);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_magnitude_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(x.as_raw__InputArray(), y.as_raw__InputArray(), magnitude.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes magnitudes of complex matrix elements.
	/// 
	/// ## Parameters
	/// * xy: Source complex matrix in the interleaved format ( CV_32FC2 ).
	/// * magnitude: Destination matrix of float magnitudes ( CV_32FC1 ).
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// magnitude
	/// 
	/// ## Note
	/// This alternative version of [magnitude] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn magnitude_def(xy: &impl core::ToInputArray, magnitude: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(xy);
		output_array_arg!(magnitude);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_magnitude_const__InputArrayR_const__OutputArrayR(xy.as_raw__InputArray(), magnitude.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes magnitudes of complex matrix elements.
	/// 
	/// ## Parameters
	/// * xy: Source complex matrix in the interleaved format ( CV_32FC2 ).
	/// * magnitude: Destination matrix of float magnitudes ( CV_32FC1 ).
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// magnitude
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn magnitude(xy: &impl core::ToInputArray, magnitude: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(xy);
		output_array_arg!(magnitude);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_magnitude_const__InputArrayR_const__OutputArrayR_StreamR(xy.as_raw__InputArray(), magnitude.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes the per-element maximum of two matrices (or a matrix and a scalar).
	/// 
	/// ## Parameters
	/// * src1: First source matrix or scalar.
	/// * src2: Second source matrix or scalar.
	/// * dst: Destination matrix that has the same size and type as the input array(s).
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// max
	/// 
	/// ## Note
	/// This alternative version of [max] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn max_def(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_max_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes the per-element maximum of two matrices (or a matrix and a scalar).
	/// 
	/// ## Parameters
	/// * src1: First source matrix or scalar.
	/// * src2: Second source matrix or scalar.
	/// * dst: Destination matrix that has the same size and type as the input array(s).
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// max
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn max(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_max_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a mean value and a standard deviation of matrix elements.
	/// 
	/// ## Parameters
	/// * src: Source matrix. CV_8UC1 and CV_32FC1 matrices are supported for now.
	/// * dst: Target GpuMat with size 1x2 and type CV_64FC1. The first value is mean, the second - stddev.
	/// * mask: Operation mask.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// meanStdDev
	/// 
	/// ## Overloaded parameters
	/// 
	/// * mtx: Source matrix. CV_8UC1 and CV_32FC1 matrices are supported for now.
	/// * mean: Mean value.
	/// * stddev: Standard deviation value.
	#[inline]
	pub fn mean_std_dev_3(mtx: &impl core::ToInputArray, mean: &mut core::Scalar, stddev: &mut core::Scalar) -> Result<()> {
		input_array_arg!(mtx);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_meanStdDev_const__InputArrayR_ScalarR_ScalarR(mtx.as_raw__InputArray(), mean, stddev, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a mean value and a standard deviation of matrix elements.
	/// 
	/// ## Parameters
	/// * src: Source matrix. CV_8UC1 and CV_32FC1 matrices are supported for now.
	/// * dst: Target GpuMat with size 1x2 and type CV_64FC1. The first value is mean, the second - stddev.
	/// * mask: Operation mask.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// meanStdDev
	/// 
	/// ## Overloaded parameters
	/// 
	/// * src: Source matrix. CV_8UC1 and CV_32FC1 matrices are supported for now.
	/// * mean: Mean value.
	/// * stddev: Standard deviation value.
	/// * mask: Operation mask.
	#[inline]
	pub fn mean_std_dev_2(src: &impl core::ToInputArray, mean: &mut core::Scalar, stddev: &mut core::Scalar, mask: &impl core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_meanStdDev_const__InputArrayR_ScalarR_ScalarR_const__InputArrayR(src.as_raw__InputArray(), mean, stddev, mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// @overload
	/// ## Parameters
	/// * mtx: Source matrix. CV_8UC1 and CV_32FC1 matrices are supported for now.
	/// * dst: Target GpuMat with size 1x2 and type CV_64FC1. The first value is mean, the second - stddev.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## Note
	/// This alternative version of [mean_std_dev_1] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn mean_std_dev_1_def(mtx: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(mtx);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_meanStdDev_const__InputArrayR_const__OutputArrayR(mtx.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a mean value and a standard deviation of matrix elements.
	/// 
	/// ## Parameters
	/// * src: Source matrix. CV_8UC1 and CV_32FC1 matrices are supported for now.
	/// * dst: Target GpuMat with size 1x2 and type CV_64FC1. The first value is mean, the second - stddev.
	/// * mask: Operation mask.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// meanStdDev
	/// 
	/// ## Overloaded parameters
	/// 
	/// * mtx: Source matrix. CV_8UC1 and CV_32FC1 matrices are supported for now.
	/// * dst: Target GpuMat with size 1x2 and type CV_64FC1. The first value is mean, the second - stddev.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn mean_std_dev_1(mtx: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(mtx);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_meanStdDev_const__InputArrayR_const__OutputArrayR_StreamR(mtx.as_raw__InputArray(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a mean value and a standard deviation of matrix elements.
	/// 
	/// ## Parameters
	/// * src: Source matrix. CV_8UC1 and CV_32FC1 matrices are supported for now.
	/// * dst: Target GpuMat with size 1x2 and type CV_64FC1. The first value is mean, the second - stddev.
	/// * mask: Operation mask.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// meanStdDev
	/// 
	/// ## Note
	/// This alternative version of [mean_std_dev] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn mean_std_dev_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, mask: &impl core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_meanStdDev_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a mean value and a standard deviation of matrix elements.
	/// 
	/// ## Parameters
	/// * src: Source matrix. CV_8UC1 and CV_32FC1 matrices are supported for now.
	/// * dst: Target GpuMat with size 1x2 and type CV_64FC1. The first value is mean, the second - stddev.
	/// * mask: Operation mask.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// meanStdDev
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn mean_std_dev(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, mask: &impl core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_meanStdDev_const__InputArrayR_const__OutputArrayR_const__InputArrayR_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Makes a multi-channel matrix out of several single-channel matrices.
	/// 
	/// ## Parameters
	/// * src: Array/vector of source matrices.
	/// * n: Number of source matrices.
	/// * dst: Destination matrix.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// merge
	/// 
	/// ## Note
	/// This alternative version of [merge] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn merge_def(src: &core::GpuMat, n: size_t, dst: &mut impl core::ToOutputArray) -> Result<()> {
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_merge_const_GpuMatX_size_t_const__OutputArrayR(src.as_raw_GpuMat(), n, dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Makes a multi-channel matrix out of several single-channel matrices.
	/// 
	/// ## Parameters
	/// * src: Array/vector of source matrices.
	/// * n: Number of source matrices.
	/// * dst: Destination matrix.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// merge
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn merge(src: &core::GpuMat, n: size_t, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_merge_const_GpuMatX_size_t_const__OutputArrayR_StreamR(src.as_raw_GpuMat(), n, dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// @overload
	/// 
	/// ## Note
	/// This alternative version of [merge_1] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn merge_1_def(src: &core::Vector<core::GpuMat>, dst: &mut impl core::ToOutputArray) -> Result<()> {
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_merge_const_vectorLGpuMatGR_const__OutputArrayR(src.as_raw_VectorOfGpuMat(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Makes a multi-channel matrix out of several single-channel matrices.
	/// 
	/// ## Parameters
	/// * src: Array/vector of source matrices.
	/// * n: Number of source matrices.
	/// * dst: Destination matrix.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// merge
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn merge_1(src: &core::Vector<core::GpuMat>, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_merge_const_vectorLGpuMatGR_const__OutputArrayR_StreamR(src.as_raw_VectorOfGpuMat(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Finds global minimum and maximum matrix elements and returns their values with locations.
	/// 
	/// ## Parameters
	/// * src: Single-channel source image.
	/// * minVal: Pointer to the returned minimum value. Use NULL if not required.
	/// * maxVal: Pointer to the returned maximum value. Use NULL if not required.
	/// * minLoc: Pointer to the returned minimum location. Use NULL if not required.
	/// * maxLoc: Pointer to the returned maximum location. Use NULL if not required.
	/// * mask: Optional mask to select a sub-matrix.
	/// 
	/// The function does not work with CV_64F images on GPU with the compute capability \< 1.3.
	/// ## See also
	/// minMaxLoc
	/// 
	/// ## Note
	/// This alternative version of [min_max_loc] function uses the following default values for its arguments:
	/// * mask: noArray()
	#[inline]
	pub fn min_max_loc_def(src: &impl core::ToInputArray, min_val: &mut f64, max_val: &mut f64, min_loc: &mut core::Point, max_loc: &mut core::Point) -> Result<()> {
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_minMaxLoc_const__InputArrayR_doubleX_doubleX_PointX_PointX(src.as_raw__InputArray(), min_val, max_val, min_loc, max_loc, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Finds global minimum and maximum matrix elements and returns their values with locations.
	/// 
	/// ## Parameters
	/// * src: Single-channel source image.
	/// * minVal: Pointer to the returned minimum value. Use NULL if not required.
	/// * maxVal: Pointer to the returned maximum value. Use NULL if not required.
	/// * minLoc: Pointer to the returned minimum location. Use NULL if not required.
	/// * maxLoc: Pointer to the returned maximum location. Use NULL if not required.
	/// * mask: Optional mask to select a sub-matrix.
	/// 
	/// The function does not work with CV_64F images on GPU with the compute capability \< 1.3.
	/// ## See also
	/// minMaxLoc
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	#[inline]
	pub fn min_max_loc(src: &impl core::ToInputArray, min_val: &mut f64, max_val: &mut f64, min_loc: &mut core::Point, max_loc: &mut core::Point, mask: &impl core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_minMaxLoc_const__InputArrayR_doubleX_doubleX_PointX_PointX_const__InputArrayR(src.as_raw__InputArray(), min_val, max_val, min_loc, max_loc, mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Finds global minimum and maximum matrix elements and returns their values.
	/// 
	/// ## Parameters
	/// * src: Single-channel source image.
	/// * minVal: Pointer to the returned minimum value. Use NULL if not required.
	/// * maxVal: Pointer to the returned maximum value. Use NULL if not required.
	/// * mask: Optional mask to select a sub-matrix.
	/// 
	/// The function does not work with CV_64F images on GPUs with the compute capability \< 1.3.
	/// ## See also
	/// minMaxLoc
	/// 
	/// ## Note
	/// This alternative version of [min_max] function uses the following default values for its arguments:
	/// * mask: noArray()
	#[inline]
	pub fn min_max_def(src: &impl core::ToInputArray, min_val: &mut f64, max_val: &mut f64) -> Result<()> {
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_minMax_const__InputArrayR_doubleX_doubleX(src.as_raw__InputArray(), min_val, max_val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Finds global minimum and maximum matrix elements and returns their values.
	/// 
	/// ## Parameters
	/// * src: Single-channel source image.
	/// * minVal: Pointer to the returned minimum value. Use NULL if not required.
	/// * maxVal: Pointer to the returned maximum value. Use NULL if not required.
	/// * mask: Optional mask to select a sub-matrix.
	/// 
	/// The function does not work with CV_64F images on GPUs with the compute capability \< 1.3.
	/// ## See also
	/// minMaxLoc
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	#[inline]
	pub fn min_max(src: &impl core::ToInputArray, min_val: &mut f64, max_val: &mut f64, mask: &impl core::ToInputArray) -> Result<()> {
		input_array_arg!(src);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_minMax_const__InputArrayR_doubleX_doubleX_const__InputArrayR(src.as_raw__InputArray(), min_val, max_val, mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes the per-element minimum of two matrices (or a matrix and a scalar).
	/// 
	/// ## Parameters
	/// * src1: First source matrix or scalar.
	/// * src2: Second source matrix or scalar.
	/// * dst: Destination matrix that has the same size and type as the input array(s).
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// min
	/// 
	/// ## Note
	/// This alternative version of [min] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn min_def(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_min_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes the per-element minimum of two matrices (or a matrix and a scalar).
	/// 
	/// ## Parameters
	/// * src1: First source matrix or scalar.
	/// * src2: Second source matrix or scalar.
	/// * dst: Destination matrix that has the same size and type as the input array(s).
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// min
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn min(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_min_const__InputArrayR_const__InputArrayR_const__OutputArrayR_StreamR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Performs a per-element multiplication of two Fourier spectrums and scales the result.
	/// 
	/// ## Parameters
	/// * src1: First spectrum.
	/// * src2: Second spectrum with the same size and type as a .
	/// * dst: Destination spectrum.
	/// * flags: Mock parameter used for CPU/CUDA interfaces similarity, simply add a `0` value.
	/// * scale: Scale constant.
	/// * conjB: Optional flag to specify if the second spectrum needs to be conjugated before the
	/// multiplication.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// Only full (not packed) CV_32FC2 complex spectrums in the interleaved format are supported for now.
	/// ## See also
	/// mulSpectrums
	/// 
	/// ## Note
	/// This alternative version of [mul_and_scale_spectrums] function uses the following default values for its arguments:
	/// * conj_b: false
	/// * stream: Stream::Null()
	#[inline]
	pub fn mul_and_scale_spectrums_def(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, flags: i32, scale: f32) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_mulAndScaleSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_float(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), flags, scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Performs a per-element multiplication of two Fourier spectrums and scales the result.
	/// 
	/// ## Parameters
	/// * src1: First spectrum.
	/// * src2: Second spectrum with the same size and type as a .
	/// * dst: Destination spectrum.
	/// * flags: Mock parameter used for CPU/CUDA interfaces similarity, simply add a `0` value.
	/// * scale: Scale constant.
	/// * conjB: Optional flag to specify if the second spectrum needs to be conjugated before the
	/// multiplication.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// Only full (not packed) CV_32FC2 complex spectrums in the interleaved format are supported for now.
	/// ## See also
	/// mulSpectrums
	/// 
	/// ## C++ default parameters
	/// * conj_b: false
	/// * stream: Stream::Null()
	#[inline]
	pub fn mul_and_scale_spectrums(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, flags: i32, scale: f32, conj_b: bool, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_mulAndScaleSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_float_bool_StreamR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), flags, scale, conj_b, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Performs a per-element multiplication of two Fourier spectrums.
	/// 
	/// ## Parameters
	/// * src1: First spectrum.
	/// * src2: Second spectrum with the same size and type as a .
	/// * dst: Destination spectrum.
	/// * flags: Mock parameter used for CPU/CUDA interfaces similarity.
	/// * conjB: Optional flag to specify if the second spectrum needs to be conjugated before the
	/// multiplication.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// Only full (not packed) CV_32FC2 complex spectrums in the interleaved format are supported for now.
	/// ## See also
	/// mulSpectrums
	/// 
	/// ## Note
	/// This alternative version of [mul_spectrums] function uses the following default values for its arguments:
	/// * conj_b: false
	/// * stream: Stream::Null()
	#[inline]
	pub fn mul_spectrums_def(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, flags: i32) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_mulSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), flags, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Performs a per-element multiplication of two Fourier spectrums.
	/// 
	/// ## Parameters
	/// * src1: First spectrum.
	/// * src2: Second spectrum with the same size and type as a .
	/// * dst: Destination spectrum.
	/// * flags: Mock parameter used for CPU/CUDA interfaces similarity.
	/// * conjB: Optional flag to specify if the second spectrum needs to be conjugated before the
	/// multiplication.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// Only full (not packed) CV_32FC2 complex spectrums in the interleaved format are supported for now.
	/// ## See also
	/// mulSpectrums
	/// 
	/// ## C++ default parameters
	/// * conj_b: false
	/// * stream: Stream::Null()
	#[inline]
	pub fn mul_spectrums(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, flags: i32, conj_b: bool, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_mulSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_bool_StreamR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), flags, conj_b, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a matrix-matrix or matrix-scalar per-element product.
	/// 
	/// ## Parameters
	/// * src1: First source matrix or scalar.
	/// * src2: Second source matrix or scalar.
	/// * dst: Destination matrix that has the same size and number of channels as the input array(s).
	/// The depth is defined by dtype or src1 depth.
	/// * scale: Optional scale factor.
	/// * dtype: Optional depth of the output array.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// multiply
	/// 
	/// ## Note
	/// This alternative version of [multiply] function uses the following default values for its arguments:
	/// * scale: 1
	/// * dtype: -1
	/// * stream: Stream::Null()
	#[inline]
	pub fn multiply_def(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_multiply_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a matrix-matrix or matrix-scalar per-element product.
	/// 
	/// ## Parameters
	/// * src1: First source matrix or scalar.
	/// * src2: Second source matrix or scalar.
	/// * dst: Destination matrix that has the same size and number of channels as the input array(s).
	/// The depth is defined by dtype or src1 depth.
	/// * scale: Optional scale factor.
	/// * dtype: Optional depth of the output array.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// multiply
	/// 
	/// ## C++ default parameters
	/// * scale: 1
	/// * dtype: -1
	/// * stream: Stream::Null()
	#[inline]
	pub fn multiply(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, scale: f64, dtype: i32, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_multiply_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int_StreamR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), scale, dtype, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns the difference of two matrices.
	/// 
	/// ## Parameters
	/// * src1: Source matrix. Any matrices except 64F are supported.
	/// * src2: Second source matrix (if any) with the same size and type as src1.
	/// * normType: Norm type. NORM_L1 , NORM_L2 , and NORM_INF are supported for now.
	/// ## See also
	/// norm
	/// 
	/// ## Note
	/// This alternative version of [norm_1] function uses the following default values for its arguments:
	/// * norm_type: NORM_L2
	#[inline]
	pub fn norm_1_def(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray) -> Result<f64> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_norm_const__InputArrayR_const__InputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns the difference of two matrices.
	/// 
	/// ## Parameters
	/// * src1: Source matrix. Any matrices except 64F are supported.
	/// * src2: Second source matrix (if any) with the same size and type as src1.
	/// * normType: Norm type. NORM_L1 , NORM_L2 , and NORM_INF are supported for now.
	/// ## See also
	/// norm
	/// 
	/// ## C++ default parameters
	/// * norm_type: NORM_L2
	#[inline]
	pub fn norm_1(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, norm_type: i32) -> Result<f64> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_norm_const__InputArrayR_const__InputArrayR_int(src1.as_raw__InputArray(), src2.as_raw__InputArray(), norm_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns the norm of a matrix (or difference of two matrices).
	/// 
	/// ## Parameters
	/// * src1: Source matrix. Any matrices except 64F are supported.
	/// * normType: Norm type. NORM_L1 , NORM_L2 , and NORM_INF are supported for now.
	/// * mask: optional operation mask; it must have the same size as src1 and CV_8UC1 type.
	/// ## See also
	/// norm
	/// 
	/// ## Note
	/// This alternative version of [norm] function uses the following default values for its arguments:
	/// * mask: noArray()
	#[inline]
	pub fn norm_def(src1: &impl core::ToInputArray, norm_type: i32) -> Result<f64> {
		input_array_arg!(src1);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_norm_const__InputArrayR_int(src1.as_raw__InputArray(), norm_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns the norm of a matrix (or difference of two matrices).
	/// 
	/// ## Parameters
	/// * src1: Source matrix. Any matrices except 64F are supported.
	/// * normType: Norm type. NORM_L1 , NORM_L2 , and NORM_INF are supported for now.
	/// * mask: optional operation mask; it must have the same size as src1 and CV_8UC1 type.
	/// ## See also
	/// norm
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	#[inline]
	pub fn norm(src1: &impl core::ToInputArray, norm_type: i32, mask: &impl core::ToInputArray) -> Result<f64> {
		input_array_arg!(src1);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_norm_const__InputArrayR_int_const__InputArrayR(src1.as_raw__InputArray(), norm_type, mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Normalizes the norm or value range of an array.
	/// 
	/// ## Parameters
	/// * src: Input array.
	/// * dst: Output array of the same size as src .
	/// * alpha: Norm value to normalize to or the lower range boundary in case of the range
	/// normalization.
	/// * beta: Upper range boundary in case of the range normalization; it is not used for the norm
	/// normalization.
	/// * norm_type: Normalization type ( NORM_MINMAX , NORM_L2 , NORM_L1 or NORM_INF ).
	/// * dtype: When negative, the output array has the same type as src; otherwise, it has the same
	/// number of channels as src and the depth =CV_MAT_DEPTH(dtype).
	/// * mask: Optional operation mask.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// normalize
	/// 
	/// ## Note
	/// This alternative version of [normalize] function uses the following default values for its arguments:
	/// * mask: noArray()
	/// * stream: Stream::Null()
	#[inline]
	pub fn normalize_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, alpha: f64, beta: f64, norm_type: i32, dtype: i32) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_normalize_const__InputArrayR_const__OutputArrayR_double_double_int_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), alpha, beta, norm_type, dtype, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Normalizes the norm or value range of an array.
	/// 
	/// ## Parameters
	/// * src: Input array.
	/// * dst: Output array of the same size as src .
	/// * alpha: Norm value to normalize to or the lower range boundary in case of the range
	/// normalization.
	/// * beta: Upper range boundary in case of the range normalization; it is not used for the norm
	/// normalization.
	/// * norm_type: Normalization type ( NORM_MINMAX , NORM_L2 , NORM_L1 or NORM_INF ).
	/// * dtype: When negative, the output array has the same type as src; otherwise, it has the same
	/// number of channels as src and the depth =CV_MAT_DEPTH(dtype).
	/// * mask: Optional operation mask.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// normalize
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * stream: Stream::Null()
	#[inline]
	pub fn normalize(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, alpha: f64, beta: f64, norm_type: i32, dtype: i32, mask: &impl core::ToInputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_normalize_const__InputArrayR_const__OutputArrayR_double_double_int_int_const__InputArrayR_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), alpha, beta, norm_type, dtype, mask.as_raw__InputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes polar angles of complex matrix elements.
	/// 
	/// ## Parameters
	/// * x: Source matrix containing real components ( CV_32FC1 ).
	/// * y: Source matrix containing imaginary components ( CV_32FC1 ).
	/// * angle: Destination matrix of angles ( CV_32FC1 ).
	/// * angleInDegrees: Flag for angles that must be evaluated in degrees.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// phase
	/// 
	/// ## Note
	/// This alternative version of [phase] function uses the following default values for its arguments:
	/// * angle_in_degrees: false
	/// * stream: Stream::Null()
	#[inline]
	pub fn phase_def(x: &impl core::ToInputArray, y: &impl core::ToInputArray, angle: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(x);
		input_array_arg!(y);
		output_array_arg!(angle);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_phase_const__InputArrayR_const__InputArrayR_const__OutputArrayR(x.as_raw__InputArray(), y.as_raw__InputArray(), angle.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes polar angles of complex matrix elements.
	/// 
	/// ## Parameters
	/// * x: Source matrix containing real components ( CV_32FC1 ).
	/// * y: Source matrix containing imaginary components ( CV_32FC1 ).
	/// * angle: Destination matrix of angles ( CV_32FC1 ).
	/// * angleInDegrees: Flag for angles that must be evaluated in degrees.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// phase
	/// 
	/// ## C++ default parameters
	/// * angle_in_degrees: false
	/// * stream: Stream::Null()
	#[inline]
	pub fn phase(x: &impl core::ToInputArray, y: &impl core::ToInputArray, angle: &mut impl core::ToOutputArray, angle_in_degrees: bool, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(x);
		input_array_arg!(y);
		output_array_arg!(angle);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_phase_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool_StreamR(x.as_raw__InputArray(), y.as_raw__InputArray(), angle.as_raw__OutputArray(), angle_in_degrees, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Converts polar coordinates into Cartesian.
	/// 
	/// ## Parameters
	/// * magnitude: Source matrix containing magnitudes ( CV_32FC1 or CV_64FC1 ).
	/// * angle: Source matrix containing angles ( same type as magnitude ).
	/// * x: Destination matrix of real components ( same type as magnitude ).
	/// * y: Destination matrix of imaginary components ( same type as magnitude ).
	/// * angleInDegrees: Flag that indicates angles in degrees.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## Note
	/// This alternative version of [polar_to_cart] function uses the following default values for its arguments:
	/// * angle_in_degrees: false
	/// * stream: Stream::Null()
	#[inline]
	pub fn polar_to_cart_def(magnitude: &impl core::ToInputArray, angle: &impl core::ToInputArray, x: &mut impl core::ToOutputArray, y: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(magnitude);
		input_array_arg!(angle);
		output_array_arg!(x);
		output_array_arg!(y);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_polarToCart_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(magnitude.as_raw__InputArray(), angle.as_raw__InputArray(), x.as_raw__OutputArray(), y.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Converts polar coordinates into Cartesian.
	/// 
	/// ## Parameters
	/// * magnitude: Source matrix containing magnitudes ( CV_32FC1 or CV_64FC1 ).
	/// * angle: Source matrix containing angles ( same type as magnitude ).
	/// * x: Destination matrix of real components ( same type as magnitude ).
	/// * y: Destination matrix of imaginary components ( same type as magnitude ).
	/// * angleInDegrees: Flag that indicates angles in degrees.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * angle_in_degrees: false
	/// * stream: Stream::Null()
	#[inline]
	pub fn polar_to_cart(magnitude: &impl core::ToInputArray, angle: &impl core::ToInputArray, x: &mut impl core::ToOutputArray, y: &mut impl core::ToOutputArray, angle_in_degrees: bool, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(magnitude);
		input_array_arg!(angle);
		output_array_arg!(x);
		output_array_arg!(y);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_polarToCart_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool_StreamR(magnitude.as_raw__InputArray(), angle.as_raw__InputArray(), x.as_raw__OutputArray(), y.as_raw__OutputArray(), angle_in_degrees, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Raises every matrix element to a power.
	/// 
	/// ## Parameters
	/// * src: Source matrix.
	/// * power: Exponent of power.
	/// * dst: Destination matrix with the same size and type as src .
	/// * stream: Stream for the asynchronous version.
	/// 
	/// The function pow raises every element of the input matrix to power :
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Cfork%7B%5Ctexttt%7Bsrc%7D%28I%29%5Epower%7D%7Bif%20%5Ctexttt%7Bpower%7D%20is%20integer%7D%7B%7C%5Ctexttt%7Bsrc%7D%28I%29%7C%5Epower%7D%7Botherwise%7D)
	/// ## See also
	/// pow
	/// 
	/// ## Note
	/// This alternative version of [pow] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn pow_def(src: &impl core::ToInputArray, power: f64, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_pow_const__InputArrayR_double_const__OutputArrayR(src.as_raw__InputArray(), power, dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Raises every matrix element to a power.
	/// 
	/// ## Parameters
	/// * src: Source matrix.
	/// * power: Exponent of power.
	/// * dst: Destination matrix with the same size and type as src .
	/// * stream: Stream for the asynchronous version.
	/// 
	/// The function pow raises every element of the input matrix to power :
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Cfork%7B%5Ctexttt%7Bsrc%7D%28I%29%5Epower%7D%7Bif%20%5Ctexttt%7Bpower%7D%20is%20integer%7D%7B%7C%5Ctexttt%7Bsrc%7D%28I%29%7C%5Epower%7D%7Botherwise%7D)
	/// ## See also
	/// pow
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn pow(src: &impl core::ToInputArray, power: f64, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_pow_const__InputArrayR_double_const__OutputArrayR_StreamR(src.as_raw__InputArray(), power, dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a standard deviation of integral images.
	/// 
	/// ## Parameters
	/// * src: Source image. Only the CV_32SC1 type is supported.
	/// * sqr: Squared source image. Only the CV_32FC1 type is supported.
	/// * dst: Destination image with the same type and size as src.
	/// * rect: Rectangular window.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## Note
	/// This alternative version of [rect_std_dev] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn rect_std_dev_def(src: &impl core::ToInputArray, sqr: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, rect: core::Rect) -> Result<()> {
		input_array_arg!(src);
		input_array_arg!(sqr);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_rectStdDev_const__InputArrayR_const__InputArrayR_const__OutputArrayR_Rect(src.as_raw__InputArray(), sqr.as_raw__InputArray(), dst.as_raw__OutputArray(), rect.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a standard deviation of integral images.
	/// 
	/// ## Parameters
	/// * src: Source image. Only the CV_32SC1 type is supported.
	/// * sqr: Squared source image. Only the CV_32FC1 type is supported.
	/// * dst: Destination image with the same type and size as src.
	/// * rect: Rectangular window.
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn rect_std_dev(src: &impl core::ToInputArray, sqr: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, rect: core::Rect, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		input_array_arg!(sqr);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_rectStdDev_const__InputArrayR_const__InputArrayR_const__OutputArrayR_Rect_StreamR(src.as_raw__InputArray(), sqr.as_raw__InputArray(), dst.as_raw__OutputArray(), rect.opencv_as_extern(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Reduces a matrix to a vector.
	/// 
	/// ## Parameters
	/// * mtx: Source 2D matrix.
	/// * vec: Destination vector. Its size and type is defined by dim and dtype parameters.
	/// * dim: Dimension index along which the matrix is reduced. 0 means that the matrix is reduced
	/// to a single row. 1 means that the matrix is reduced to a single column.
	/// * reduceOp: Reduction operation that could be one of the following:
	/// *   **REDUCE_SUM** The output is the sum of all rows/columns of the matrix.
	/// *   **REDUCE_AVG** The output is the mean vector of all rows/columns of the matrix.
	/// *   **REDUCE_MAX** The output is the maximum (column/row-wise) of all rows/columns of the
	/// matrix.
	/// *   **REDUCE_MIN** The output is the minimum (column/row-wise) of all rows/columns of the
	/// matrix.
	/// * dtype: When it is negative, the destination vector will have the same type as the source
	/// matrix. Otherwise, its type will be CV_MAKE_TYPE(CV_MAT_DEPTH(dtype), mtx.channels()) .
	/// * stream: Stream for the asynchronous version.
	/// 
	/// The function reduce reduces the matrix to a vector by treating the matrix rows/columns as a set of
	/// 1D vectors and performing the specified operation on the vectors until a single row/column is
	/// obtained. For example, the function can be used to compute horizontal and vertical projections of a
	/// raster image. In case of REDUCE_SUM and REDUCE_AVG , the output may have a larger element
	/// bit-depth to preserve accuracy. And multi-channel arrays are also supported in these two reduction
	/// modes.
	/// ## See also
	/// reduce
	/// 
	/// ## Note
	/// This alternative version of [reduce] function uses the following default values for its arguments:
	/// * dtype: -1
	/// * stream: Stream::Null()
	#[inline]
	pub fn reduce_def(mtx: &impl core::ToInputArray, vec: &mut impl core::ToOutputArray, dim: i32, reduce_op: i32) -> Result<()> {
		input_array_arg!(mtx);
		output_array_arg!(vec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_reduce_const__InputArrayR_const__OutputArrayR_int_int(mtx.as_raw__InputArray(), vec.as_raw__OutputArray(), dim, reduce_op, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Reduces a matrix to a vector.
	/// 
	/// ## Parameters
	/// * mtx: Source 2D matrix.
	/// * vec: Destination vector. Its size and type is defined by dim and dtype parameters.
	/// * dim: Dimension index along which the matrix is reduced. 0 means that the matrix is reduced
	/// to a single row. 1 means that the matrix is reduced to a single column.
	/// * reduceOp: Reduction operation that could be one of the following:
	/// *   **REDUCE_SUM** The output is the sum of all rows/columns of the matrix.
	/// *   **REDUCE_AVG** The output is the mean vector of all rows/columns of the matrix.
	/// *   **REDUCE_MAX** The output is the maximum (column/row-wise) of all rows/columns of the
	/// matrix.
	/// *   **REDUCE_MIN** The output is the minimum (column/row-wise) of all rows/columns of the
	/// matrix.
	/// * dtype: When it is negative, the destination vector will have the same type as the source
	/// matrix. Otherwise, its type will be CV_MAKE_TYPE(CV_MAT_DEPTH(dtype), mtx.channels()) .
	/// * stream: Stream for the asynchronous version.
	/// 
	/// The function reduce reduces the matrix to a vector by treating the matrix rows/columns as a set of
	/// 1D vectors and performing the specified operation on the vectors until a single row/column is
	/// obtained. For example, the function can be used to compute horizontal and vertical projections of a
	/// raster image. In case of REDUCE_SUM and REDUCE_AVG , the output may have a larger element
	/// bit-depth to preserve accuracy. And multi-channel arrays are also supported in these two reduction
	/// modes.
	/// ## See also
	/// reduce
	/// 
	/// ## C++ default parameters
	/// * dtype: -1
	/// * stream: Stream::Null()
	#[inline]
	pub fn reduce(mtx: &impl core::ToInputArray, vec: &mut impl core::ToOutputArray, dim: i32, reduce_op: i32, dtype: i32, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(mtx);
		output_array_arg!(vec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_reduce_const__InputArrayR_const__OutputArrayR_int_int_int_StreamR(mtx.as_raw__InputArray(), vec.as_raw__OutputArray(), dim, reduce_op, dtype, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Performs pixel by pixel right shift of an image by a constant value.
	/// 
	/// ## Parameters
	/// * src: Source matrix. Supports 1, 3 and 4 channels images with integers elements.
	/// * val: Constant values, one per channel.
	/// * dst: Destination matrix with the same size and type as src .
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## Note
	/// This alternative version of [rshift] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn rshift_def(src: &impl core::ToInputArray, val: core::Scalar_<i32>, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_rshift_const__InputArrayR_Scalar_LintG_const__OutputArrayR(src.as_raw__InputArray(), val.opencv_as_extern(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Performs pixel by pixel right shift of an image by a constant value.
	/// 
	/// ## Parameters
	/// * src: Source matrix. Supports 1, 3 and 4 channels images with integers elements.
	/// * val: Constant values, one per channel.
	/// * dst: Destination matrix with the same size and type as src .
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn rshift(src: &impl core::ToInputArray, val: core::Scalar_<i32>, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_rshift_const__InputArrayR_Scalar_LintG_const__OutputArrayR_StreamR(src.as_raw__InputArray(), val.opencv_as_extern(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [rshift_1] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn rshift_1_def(src: &impl core::ToInputArray, val: core::Scalar, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_rshift_const__InputArrayR_Scalar_const__OutputArrayR(src.as_raw__InputArray(), val.opencv_as_extern(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn rshift_1(src: &impl core::ToInputArray, val: core::Scalar, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_rshift_const__InputArrayR_Scalar_const__OutputArrayR_StreamR(src.as_raw__InputArray(), val.opencv_as_extern(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Copies each plane of a multi-channel matrix into an array.
	/// 
	/// ## Parameters
	/// * src: Source matrix.
	/// * dst: Destination array/vector of single-channel matrices.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// split
	/// 
	/// ## Note
	/// This alternative version of [split] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn split_def(src: &impl core::ToInputArray, dst: &mut core::GpuMat) -> Result<()> {
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_split_const__InputArrayR_GpuMatX(src.as_raw__InputArray(), dst.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Copies each plane of a multi-channel matrix into an array.
	/// 
	/// ## Parameters
	/// * src: Source matrix.
	/// * dst: Destination array/vector of single-channel matrices.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// split
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn split(src: &impl core::ToInputArray, dst: &mut core::GpuMat, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_split_const__InputArrayR_GpuMatX_StreamR(src.as_raw__InputArray(), dst.as_raw_mut_GpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// @overload
	/// 
	/// ## Note
	/// This alternative version of [split_1] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn split_1_def(src: &impl core::ToInputArray, dst: &mut core::Vector<core::GpuMat>) -> Result<()> {
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_split_const__InputArrayR_vectorLGpuMatGR(src.as_raw__InputArray(), dst.as_raw_mut_VectorOfGpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Copies each plane of a multi-channel matrix into an array.
	/// 
	/// ## Parameters
	/// * src: Source matrix.
	/// * dst: Destination array/vector of single-channel matrices.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// split
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn split_1(src: &impl core::ToInputArray, dst: &mut core::Vector<core::GpuMat>, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_split_const__InputArrayR_vectorLGpuMatGR_StreamR(src.as_raw__InputArray(), dst.as_raw_mut_VectorOfGpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a squared integral image.
	/// 
	/// ## Parameters
	/// * src: Source image. Only CV_8UC1 images are supported for now.
	/// * sqsum: Squared integral image containing 64-bit unsigned integer values packed into
	/// CV_64FC1 .
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## Note
	/// This alternative version of [sqr_integral] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn sqr_integral_def(src: &impl core::ToInputArray, sqsum: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(sqsum);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_sqrIntegral_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), sqsum.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a squared integral image.
	/// 
	/// ## Parameters
	/// * src: Source image. Only CV_8UC1 images are supported for now.
	/// * sqsum: Squared integral image containing 64-bit unsigned integer values packed into
	/// CV_64FC1 .
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn sqr_integral(src: &impl core::ToInputArray, sqsum: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(sqsum);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_sqrIntegral_const__InputArrayR_const__OutputArrayR_StreamR(src.as_raw__InputArray(), sqsum.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns the squared sum of matrix elements.
	/// 
	/// ## Parameters
	/// * src: Source image of any depth except for CV_64F .
	/// * mask: optional operation mask; it must have the same size as src1 and CV_8UC1 type.
	/// 
	/// ## Note
	/// This alternative version of [sqr_sum] function uses the following default values for its arguments:
	/// * mask: noArray()
	#[inline]
	pub fn sqr_sum_def(src: &impl core::ToInputArray) -> Result<core::Scalar> {
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_sqrSum_const__InputArrayR(src.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns the squared sum of matrix elements.
	/// 
	/// ## Parameters
	/// * src: Source image of any depth except for CV_64F .
	/// * mask: optional operation mask; it must have the same size as src1 and CV_8UC1 type.
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	#[inline]
	pub fn sqr_sum(src: &impl core::ToInputArray, mask: &impl core::ToInputArray) -> Result<core::Scalar> {
		input_array_arg!(src);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_sqrSum_const__InputArrayR_const__InputArrayR(src.as_raw__InputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a square value of each matrix element.
	/// 
	/// ## Parameters
	/// * src: Source matrix.
	/// * dst: Destination matrix with the same size and type as src .
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## Note
	/// This alternative version of [sqr] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn sqr_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_sqr_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a square value of each matrix element.
	/// 
	/// ## Parameters
	/// * src: Source matrix.
	/// * dst: Destination matrix with the same size and type as src .
	/// * stream: Stream for the asynchronous version.
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn sqr(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_sqr_const__InputArrayR_const__OutputArrayR_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a square root of each matrix element.
	/// 
	/// ## Parameters
	/// * src: Source matrix.
	/// * dst: Destination matrix with the same size and type as src .
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// sqrt
	/// 
	/// ## Note
	/// This alternative version of [sqrt] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn sqrt_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_sqrt_const__InputArrayR_const__OutputArrayR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a square root of each matrix element.
	/// 
	/// ## Parameters
	/// * src: Source matrix.
	/// * dst: Destination matrix with the same size and type as src .
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// sqrt
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn sqrt(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_sqrt_const__InputArrayR_const__OutputArrayR_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a matrix-matrix or matrix-scalar difference.
	/// 
	/// ## Parameters
	/// * src1: First source matrix or scalar.
	/// * src2: Second source matrix or scalar. Matrix should have the same size and type as src1 .
	/// * dst: Destination matrix that has the same size and number of channels as the input array(s).
	/// The depth is defined by dtype or src1 depth.
	/// * mask: Optional operation mask, 8-bit single channel array, that specifies elements of the
	/// destination array to be changed. The mask can be used only with single channel images.
	/// * dtype: Optional depth of the output array.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// subtract
	/// 
	/// ## Note
	/// This alternative version of [subtract] function uses the following default values for its arguments:
	/// * mask: noArray()
	/// * dtype: -1
	/// * stream: Stream::Null()
	#[inline]
	pub fn subtract_def(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_subtract_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes a matrix-matrix or matrix-scalar difference.
	/// 
	/// ## Parameters
	/// * src1: First source matrix or scalar.
	/// * src2: Second source matrix or scalar. Matrix should have the same size and type as src1 .
	/// * dst: Destination matrix that has the same size and number of channels as the input array(s).
	/// The depth is defined by dtype or src1 depth.
	/// * mask: Optional operation mask, 8-bit single channel array, that specifies elements of the
	/// destination array to be changed. The mask can be used only with single channel images.
	/// * dtype: Optional depth of the output array.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// subtract
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	/// * dtype: -1
	/// * stream: Stream::Null()
	#[inline]
	pub fn subtract(src1: &impl core::ToInputArray, src2: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, mask: &impl core::ToInputArray, dtype: i32, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src1);
		input_array_arg!(src2);
		output_array_arg!(dst);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_subtract_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int_StreamR(src1.as_raw__InputArray(), src2.as_raw__InputArray(), dst.as_raw__OutputArray(), mask.as_raw__InputArray(), dtype, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns the sum of matrix elements.
	/// 
	/// ## Parameters
	/// * src: Source image of any depth except for CV_64F .
	/// * mask: optional operation mask; it must have the same size as src1 and CV_8UC1 type.
	/// ## See also
	/// sum
	/// 
	/// ## Note
	/// This alternative version of [sum] function uses the following default values for its arguments:
	/// * mask: noArray()
	#[inline]
	pub fn sum_def(src: &impl core::ToInputArray) -> Result<core::Scalar> {
		input_array_arg!(src);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_sum_const__InputArrayR(src.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns the sum of matrix elements.
	/// 
	/// ## Parameters
	/// * src: Source image of any depth except for CV_64F .
	/// * mask: optional operation mask; it must have the same size as src1 and CV_8UC1 type.
	/// ## See also
	/// sum
	/// 
	/// ## C++ default parameters
	/// * mask: noArray()
	#[inline]
	pub fn sum(src: &impl core::ToInputArray, mask: &impl core::ToInputArray) -> Result<core::Scalar> {
		input_array_arg!(src);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_sum_const__InputArrayR_const__InputArrayR(src.as_raw__InputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Applies a fixed-level threshold to each array element.
	/// 
	/// ## Parameters
	/// * src: Source array (single-channel).
	/// * dst: Destination array with the same size and type as src .
	/// * thresh: Threshold value.
	/// * maxval: Maximum value to use with THRESH_BINARY and THRESH_BINARY_INV threshold types.
	/// * type: Threshold type. For details, see threshold . The THRESH_OTSU and THRESH_TRIANGLE
	/// threshold types are not supported.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// threshold
	/// 
	/// ## Note
	/// This alternative version of [threshold] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn threshold_def(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, thresh: f64, maxval: f64, typ: i32) -> Result<f64> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_threshold_const__InputArrayR_const__OutputArrayR_double_double_int(src.as_raw__InputArray(), dst.as_raw__OutputArray(), thresh, maxval, typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Applies a fixed-level threshold to each array element.
	/// 
	/// ## Parameters
	/// * src: Source array (single-channel).
	/// * dst: Destination array with the same size and type as src .
	/// * thresh: Threshold value.
	/// * maxval: Maximum value to use with THRESH_BINARY and THRESH_BINARY_INV threshold types.
	/// * type: Threshold type. For details, see threshold . The THRESH_OTSU and THRESH_TRIANGLE
	/// threshold types are not supported.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// threshold
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn threshold(src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, thresh: f64, maxval: f64, typ: i32, stream: &mut core::Stream) -> Result<f64> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_threshold_const__InputArrayR_const__OutputArrayR_double_double_int_StreamR(src.as_raw__InputArray(), dst.as_raw__OutputArray(), thresh, maxval, typ, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Transposes a matrix.
	/// 
	/// ## Parameters
	/// * src1: Source matrix. 1-, 4-, 8-byte element sizes are supported for now.
	/// * dst: Destination matrix.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// transpose
	/// 
	/// ## Note
	/// This alternative version of [transpose] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn transpose_def(src1: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src1);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_transpose_const__InputArrayR_const__OutputArrayR(src1.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Transposes a matrix.
	/// 
	/// ## Parameters
	/// * src1: Source matrix. 1-, 4-, 8-byte element sizes are supported for now.
	/// * dst: Destination matrix.
	/// * stream: Stream for the asynchronous version.
	/// ## See also
	/// transpose
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn transpose(src1: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
		input_array_arg!(src1);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_transpose_const__InputArrayR_const__OutputArrayR_StreamR(src1.as_raw__InputArray(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Constant methods for [crate::cudaarithm::Convolution]
	pub trait ConvolutionTraitConst: core::AlgorithmTraitConst {
		fn as_raw_Convolution(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::cudaarithm::Convolution]
	pub trait ConvolutionTrait: core::AlgorithmTrait + crate::cudaarithm::ConvolutionTraitConst {
		fn as_raw_mut_Convolution(&mut self) -> *mut c_void;
	
		/// Computes a convolution (or cross-correlation) of two images.
		/// 
		/// ## Parameters
		/// * image: Source image. Only CV_32FC1 images are supported for now.
		/// * templ: Template image. The size is not greater than the image size. The type is the same as
		/// image .
		/// * result: Result image. If image is *W x H* and templ is *w x h*, then result must be *W-w+1 x
		/// H-h+1*.
		/// * ccorr: Flags to evaluate cross-correlation instead of convolution.
		/// * stream: Stream for the asynchronous version.
		/// 
		/// ## C++ default parameters
		/// * ccorr: false
		/// * stream: Stream::Null()
		#[inline]
		fn convolve(&mut self, image: &impl core::ToInputArray, templ: &impl core::ToInputArray, result: &mut impl core::ToOutputArray, ccorr: bool, stream: &mut core::Stream) -> Result<()> {
			input_array_arg!(image);
			input_array_arg!(templ);
			output_array_arg!(result);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_Convolution_convolve_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool_StreamR(self.as_raw_mut_Convolution(), image.as_raw__InputArray(), templ.as_raw__InputArray(), result.as_raw__OutputArray(), ccorr, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Computes a convolution (or cross-correlation) of two images.
		/// 
		/// ## Parameters
		/// * image: Source image. Only CV_32FC1 images are supported for now.
		/// * templ: Template image. The size is not greater than the image size. The type is the same as
		/// image .
		/// * result: Result image. If image is *W x H* and templ is *w x h*, then result must be *W-w+1 x
		/// H-h+1*.
		/// * ccorr: Flags to evaluate cross-correlation instead of convolution.
		/// * stream: Stream for the asynchronous version.
		/// 
		/// ## Note
		/// This alternative version of [convolve] function uses the following default values for its arguments:
		/// * ccorr: false
		/// * stream: Stream::Null()
		#[inline]
		fn convolve_def(&mut self, image: &impl core::ToInputArray, templ: &impl core::ToInputArray, result: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			input_array_arg!(templ);
			output_array_arg!(result);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_Convolution_convolve_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_Convolution(), image.as_raw__InputArray(), templ.as_raw__InputArray(), result.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Base class for convolution (or cross-correlation) operator. :
	pub struct Convolution {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Convolution }
	
	impl Drop for Convolution {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_Convolution_delete(self.as_raw_mut_Convolution()) };
		}
	}
	
	unsafe impl Send for Convolution {}
	
	impl core::AlgorithmTraitConst for Convolution {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for Convolution {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaarithm::ConvolutionTraitConst for Convolution {
		#[inline] fn as_raw_Convolution(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudaarithm::ConvolutionTrait for Convolution {
		#[inline] fn as_raw_mut_Convolution(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Convolution {
	}
	
	boxed_cast_base! { Convolution, core::Algorithm, cv_cuda_Convolution_to_Algorithm }
	
	impl std::fmt::Debug for Convolution {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Convolution")
				.finish()
		}
	}
	
	/// Constant methods for [crate::cudaarithm::DFT]
	pub trait DFTTraitConst: core::AlgorithmTraitConst {
		fn as_raw_DFT(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::cudaarithm::DFT]
	pub trait DFTTrait: core::AlgorithmTrait + crate::cudaarithm::DFTTraitConst {
		fn as_raw_mut_DFT(&mut self) -> *mut c_void;
	
		/// Computes an FFT of a given image.
		/// 
		/// ## Parameters
		/// * image: Source image. Only CV_32FC1 images are supported for now.
		/// * result: Result image.
		/// * stream: Stream for the asynchronous version.
		/// 
		/// ## C++ default parameters
		/// * stream: Stream::Null()
		#[inline]
		fn compute(&mut self, image: &impl core::ToInputArray, result: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(result);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DFT_compute_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_DFT(), image.as_raw__InputArray(), result.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Computes an FFT of a given image.
		/// 
		/// ## Parameters
		/// * image: Source image. Only CV_32FC1 images are supported for now.
		/// * result: Result image.
		/// * stream: Stream for the asynchronous version.
		/// 
		/// ## Note
		/// This alternative version of [compute] function uses the following default values for its arguments:
		/// * stream: Stream::Null()
		#[inline]
		fn compute_def(&mut self, image: &impl core::ToInputArray, result: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(result);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_DFT_compute_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_DFT(), image.as_raw__InputArray(), result.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Base class for DFT operator as a cv::Algorithm. :
	pub struct DFT {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { DFT }
	
	impl Drop for DFT {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_DFT_delete(self.as_raw_mut_DFT()) };
		}
	}
	
	unsafe impl Send for DFT {}
	
	impl core::AlgorithmTraitConst for DFT {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for DFT {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaarithm::DFTTraitConst for DFT {
		#[inline] fn as_raw_DFT(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudaarithm::DFTTrait for DFT {
		#[inline] fn as_raw_mut_DFT(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl DFT {
	}
	
	boxed_cast_base! { DFT, core::Algorithm, cv_cuda_DFT_to_Algorithm }
	
	impl std::fmt::Debug for DFT {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("DFT")
				.finish()
		}
	}
	
	/// Constant methods for [crate::cudaarithm::LookUpTable]
	pub trait LookUpTableTraitConst: core::AlgorithmTraitConst {
		fn as_raw_LookUpTable(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::cudaarithm::LookUpTable]
	pub trait LookUpTableTrait: core::AlgorithmTrait + crate::cudaarithm::LookUpTableTraitConst {
		fn as_raw_mut_LookUpTable(&mut self) -> *mut c_void;
	
		/// Transforms the source matrix into the destination matrix using the given look-up table:
		/// dst(I) = lut(src(I)) .
		/// 
		/// ## Parameters
		/// * src: Source matrix. CV_8UC1 and CV_8UC3 matrices are supported for now.
		/// * dst: Destination matrix.
		/// * stream: Stream for the asynchronous version.
		/// 
		/// ## C++ default parameters
		/// * stream: Stream::Null()
		#[inline]
		fn transform(&mut self, src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray, stream: &mut core::Stream) -> Result<()> {
			input_array_arg!(src);
			output_array_arg!(dst);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_LookUpTable_transform_const__InputArrayR_const__OutputArrayR_StreamR(self.as_raw_mut_LookUpTable(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Transforms the source matrix into the destination matrix using the given look-up table:
		/// dst(I) = lut(src(I)) .
		/// 
		/// ## Parameters
		/// * src: Source matrix. CV_8UC1 and CV_8UC3 matrices are supported for now.
		/// * dst: Destination matrix.
		/// * stream: Stream for the asynchronous version.
		/// 
		/// ## Note
		/// This alternative version of [transform] function uses the following default values for its arguments:
		/// * stream: Stream::Null()
		#[inline]
		fn transform_def(&mut self, src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(src);
			output_array_arg!(dst);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_LookUpTable_transform_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_LookUpTable(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Base class for transform using lookup table.
	pub struct LookUpTable {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { LookUpTable }
	
	impl Drop for LookUpTable {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_LookUpTable_delete(self.as_raw_mut_LookUpTable()) };
		}
	}
	
	unsafe impl Send for LookUpTable {}
	
	impl core::AlgorithmTraitConst for LookUpTable {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for LookUpTable {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaarithm::LookUpTableTraitConst for LookUpTable {
		#[inline] fn as_raw_LookUpTable(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudaarithm::LookUpTableTrait for LookUpTable {
		#[inline] fn as_raw_mut_LookUpTable(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl LookUpTable {
	}
	
	boxed_cast_base! { LookUpTable, core::Algorithm, cv_cuda_LookUpTable_to_Algorithm }
	
	impl std::fmt::Debug for LookUpTable {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("LookUpTable")
				.finish()
		}
	}
}
