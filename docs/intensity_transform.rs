pub mod intensity_transform {
	//! # The module brings implementations of intensity transformation algorithms to adjust image contrast.
	//! 
	//! Namespace for all functions is `cv::intensity_transform`.
	//! 
	//! ### Supported Algorithms
	//! - Autoscaling
	//! - Log Transformations
	//! - Power-Law (Gamma) Transformations
	//! - Contrast Stretching
	//! - BIMEF, A Bio-Inspired Multi-Exposure Fusion Framework for Low-light Image Enhancement [ying2017bio](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_ying2017bio) [ying2017new](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_ying2017new)
	//! 
	//! References from following book and websites:
	//! - Digital Image Processing 4th Edition Chapter 3 [Rafael C. Gonzalez, Richard E. Woods] [Gonzalez2018](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Gonzalez2018)
	//! - <http://www.cs.uregina.ca/Links/class-info/425/Lab3/> [lcs435lab](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_lcs435lab)
	//! - <https://theailearner.com/2019/01/30/contrast-stretching/> [theailearner](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_theailearner)
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use {  };
	}
	
	/// Given an input color image, enhance low-light images using the BIMEF method ([ying2017bio](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_ying2017bio) [ying2017new](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_ying2017new)).
	/// 
	/// ## Parameters
	/// * input: input color image.
	/// * output: resulting image.
	/// * mu: enhancement ratio.
	/// * a: a-parameter in the Camera Response Function (CRF).
	/// * b: b-parameter in the Camera Response Function (CRF).
	/// 
	/// @warning This is a C++ implementation of the [original MATLAB algorithm](https://github.com/baidut/BIMEF).
	/// Compared to the original code, this implementation is a little bit slower and does not provide the same results.
	/// In particular, quality of the image enhancement is degraded for the bright areas in certain conditions.
	/// 
	/// ## Note
	/// This alternative version of [bimef] function uses the following default values for its arguments:
	/// * mu: 0.5f
	/// * a: -0.3293f
	/// * b: 1.1258f
	#[inline]
	pub fn bimef_def(input: &impl core::ToInputArray, output: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(input);
		output_array_arg!(output);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_intensity_transform_BIMEF_const__InputArrayR_const__OutputArrayR(input.as_raw__InputArray(), output.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Given an input color image, enhance low-light images using the BIMEF method ([ying2017bio](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_ying2017bio) [ying2017new](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_ying2017new)).
	/// 
	/// ## Parameters
	/// * input: input color image.
	/// * output: resulting image.
	/// * mu: enhancement ratio.
	/// * a: a-parameter in the Camera Response Function (CRF).
	/// * b: b-parameter in the Camera Response Function (CRF).
	/// 
	/// @warning This is a C++ implementation of the [original MATLAB algorithm](https://github.com/baidut/BIMEF).
	/// Compared to the original code, this implementation is a little bit slower and does not provide the same results.
	/// In particular, quality of the image enhancement is degraded for the bright areas in certain conditions.
	/// 
	/// ## C++ default parameters
	/// * mu: 0.5f
	/// * a: -0.3293f
	/// * b: 1.1258f
	#[inline]
	pub fn bimef(input: &impl core::ToInputArray, output: &mut impl core::ToOutputArray, mu: f32, a: f32, b: f32) -> Result<()> {
		input_array_arg!(input);
		output_array_arg!(output);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_intensity_transform_BIMEF_const__InputArrayR_const__OutputArrayR_float_float_float(input.as_raw__InputArray(), output.as_raw__OutputArray(), mu, a, b, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Given an input color image, enhance low-light images using the BIMEF method ([ying2017bio](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_ying2017bio) [ying2017new](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_ying2017new)).
	/// 
	/// This is an overloaded function with the exposure ratio given as parameter.
	/// 
	/// ## Parameters
	/// * input: input color image.
	/// * output: resulting image.
	/// * k: exposure ratio.
	/// * mu: enhancement ratio.
	/// * a: a-parameter in the Camera Response Function (CRF).
	/// * b: b-parameter in the Camera Response Function (CRF).
	/// 
	/// @warning This is a C++ implementation of the [original MATLAB algorithm](https://github.com/baidut/BIMEF).
	/// Compared to the original code, this implementation is a little bit slower and does not provide the same results.
	/// In particular, quality of the image enhancement is degraded for the bright areas in certain conditions.
	#[inline]
	pub fn bimef2(input: &impl core::ToInputArray, output: &mut impl core::ToOutputArray, k: f32, mu: f32, a: f32, b: f32) -> Result<()> {
		input_array_arg!(input);
		output_array_arg!(output);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_intensity_transform_BIMEF_const__InputArrayR_const__OutputArrayR_float_float_float_float(input.as_raw__InputArray(), output.as_raw__OutputArray(), k, mu, a, b, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Given an input bgr or grayscale image, apply autoscaling on domain [0, 255] to increase
	/// the contrast of the input image and return the resulting image.
	/// 
	/// ## Parameters
	/// * input: input bgr or grayscale image.
	/// * output: resulting image of autoscaling.
	#[inline]
	pub fn autoscaling(input: core::Mat, output: &mut core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_intensity_transform_autoscaling_const_Mat_MatR(input.as_raw_Mat(), output.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Given an input bgr or grayscale image, apply linear contrast stretching on domain [0, 255]
	/// and return the resulting image.
	/// 
	/// ## Parameters
	/// * input: input bgr or grayscale image.
	/// * output: resulting image of contrast stretching.
	/// * r1: x coordinate of first point (r1, s1) in the transformation function.
	/// * s1: y coordinate of first point (r1, s1) in the transformation function.
	/// * r2: x coordinate of second point (r2, s2) in the transformation function.
	/// * s2: y coordinate of second point (r2, s2) in the transformation function.
	#[inline]
	pub fn contrast_stretching(input: core::Mat, output: &mut core::Mat, r1: i32, s1: i32, r2: i32, s2: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_intensity_transform_contrastStretching_const_Mat_MatR_const_int_const_int_const_int_const_int(input.as_raw_Mat(), output.as_raw_mut_Mat(), r1, s1, r2, s2, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Given an input bgr or grayscale image and constant gamma, apply power-law transformation,
	/// a.k.a. gamma correction to the image on domain [0, 255] and return the resulting image.
	/// 
	/// ## Parameters
	/// * input: input bgr or grayscale image.
	/// * output: resulting image of gamma corrections.
	/// * gamma: constant in c*r^gamma where r is pixel value.
	#[inline]
	pub fn gamma_correction(input: core::Mat, output: &mut core::Mat, gamma: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_intensity_transform_gammaCorrection_const_Mat_MatR_const_float(input.as_raw_Mat(), output.as_raw_mut_Mat(), gamma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Given an input bgr or grayscale image and constant c, apply log transformation to the image
	/// on domain [0, 255] and return the resulting image.
	/// 
	/// ## Parameters
	/// * input: input bgr or grayscale image.
	/// * output: resulting image of log transformations.
	#[inline]
	pub fn log_transform(input: core::Mat, output: &mut core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_intensity_transform_logTransform_const_Mat_MatR(input.as_raw_Mat(), output.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
}
