pub mod img_hash {
	//! # The module brings implementations of different image hashing algorithms.
	//! 
	//! Provide algorithms to extract the hash of images and fast way to figure out most similar images in
	//! huge data set.
	//! 
	//! Namespace for all functions is cv::img_hash.
	//! 
	//! ### Supported Algorithms
	//! 
	//! - Average hash (also called Different hash)
	//! - PHash (also called Perceptual hash)
	//! - Marr Hildreth Hash
	//! - Radial Variance Hash
	//! - Block Mean Hash (modes 0 and 1)
	//! - Color Moment Hash (this is the one and only hash algorithm resist to rotation attack(-90~90 degree))
	//! 
	//! You can study more about image hashing from following paper and websites:
	//! 
	//! - "Implementation and benchmarking of perceptual image hash functions" [zauner2010implementation](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_zauner2010implementation)
	//! - "Looks Like It" [lookslikeit](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_lookslikeit)
	//! 
	//! ### Code Example
	//! 
	//! @include samples/hash_samples.cpp
	//! 
	//! ### Performance under different attacks
	//! 
	//! ![Performance chart](https://docs.opencv.org/4.8.1/attack_performance.JPG)
	//! 
	//! ### Speed comparison with PHash library (100 images from ukbench)
	//! 
	//! ![Hash Computation chart](https://docs.opencv.org/4.8.1/hash_computation_chart.JPG)
	//! ![Hash comparison chart](https://docs.opencv.org/4.8.1/hash_comparison_chart.JPG)
	//! 
	//! As you can see, hash computation speed of img_hash module outperform [PHash library](http://www.phash.org/) a lot.
	//! 
	//! PS : I do not list out the comparison of Average hash, PHash and Color Moment hash, because I cannot
	//! find them in PHash.
	//! 
	//! ### Motivation
	//! 
	//! Collects useful image hash algorithms into opencv, so we do not need to rewrite them by ourselves
	//! again and again or rely on another 3rd party library(ex : PHash library). BOVW or correlation
	//! matching are good and robust, but they are very slow compare with image hash, if you need to deal
	//! with large scale CBIR(content based image retrieval) problem, image hash is a more reasonable
	//! solution.
	//! 
	//! ### More info
	//! 
	//! You can learn more about img_hash modules from following links, these links show you how to find
	//! similar image from ukbench dataset, provide thorough benchmark of different attacks(contrast, blur,
	//! noise(gaussion,pepper and salt), jpeg compression, watermark, resize).
	//! 
	//! * [Introduction to image hash module of opencv](http://qtandopencv.blogspot.my/2016/06/introduction-to-image-hash-module-of.html)
	//! * [Speed up image hashing of opencv(img_hash) and introduce color moment hash](http://qtandopencv.blogspot.my/2016/06/speed-up-image-hashing-of-opencvimghash.html)
	//! 
	//! ### Contributors
	//! 
	//! Tham Ngap Wei, thamngapwei@gmail.com
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::ImgHashBaseTraitConst, super::ImgHashBaseTrait, super::AverageHashTraitConst, super::AverageHashTrait, super::BlockMeanHashTraitConst, super::BlockMeanHashTrait, super::ColorMomentHashTraitConst, super::ColorMomentHashTrait, super::MarrHildrethHashTraitConst, super::MarrHildrethHashTrait, super::PHashTraitConst, super::PHashTrait, super::RadialVarianceHashTraitConst, super::RadialVarianceHashTrait };
	}
	
	/// use fewer block and generate 16*16/8 uchar hash value
	pub const BLOCK_MEAN_HASH_MODE_0: i32 = 0;
	/// use block blocks(step sizes/2), generate 31*31/8 + 1 uchar hash value
	pub const BLOCK_MEAN_HASH_MODE_1: i32 = 1;
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum BlockMeanHashMode {
		/// use fewer block and generate 16*16/8 uchar hash value
		BLOCK_MEAN_HASH_MODE_0 = 0,
		/// use block blocks(step sizes/2), generate 31*31/8 + 1 uchar hash value
		BLOCK_MEAN_HASH_MODE_1 = 1,
	}
	
	opencv_type_enum! { crate::img_hash::BlockMeanHashMode }
	
	/// Calculates img_hash::AverageHash in one call
	/// ## Parameters
	/// * inputArr: input image want to compute hash value, type should be CV_8UC4, CV_8UC3 or CV_8UC1.
	/// * outputArr: Hash value of input, it will contain 16 hex decimal number, return type is CV_8U
	#[inline]
	pub fn average_hash(input_arr: &impl core::ToInputArray, output_arr: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(input_arr);
		output_array_arg!(output_arr);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_img_hash_averageHash_const__InputArrayR_const__OutputArrayR(input_arr.as_raw__InputArray(), output_arr.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes block mean hash of the input image
	/// ## Parameters
	/// * inputArr: input image want to compute hash value, type should be CV_8UC4, CV_8UC3 or CV_8UC1.
	/// * outputArr: Hash value of input, it will contain 16 hex decimal number, return type is CV_8U
	/// * mode: the mode
	/// 
	/// ## Note
	/// This alternative version of [block_mean_hash] function uses the following default values for its arguments:
	/// * mode: BLOCK_MEAN_HASH_MODE_0
	#[inline]
	pub fn block_mean_hash_def(input_arr: &impl core::ToInputArray, output_arr: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(input_arr);
		output_array_arg!(output_arr);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_img_hash_blockMeanHash_const__InputArrayR_const__OutputArrayR(input_arr.as_raw__InputArray(), output_arr.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes block mean hash of the input image
	/// ## Parameters
	/// * inputArr: input image want to compute hash value, type should be CV_8UC4, CV_8UC3 or CV_8UC1.
	/// * outputArr: Hash value of input, it will contain 16 hex decimal number, return type is CV_8U
	/// * mode: the mode
	/// 
	/// ## C++ default parameters
	/// * mode: BLOCK_MEAN_HASH_MODE_0
	#[inline]
	pub fn block_mean_hash(input_arr: &impl core::ToInputArray, output_arr: &mut impl core::ToOutputArray, mode: i32) -> Result<()> {
		input_array_arg!(input_arr);
		output_array_arg!(output_arr);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_img_hash_blockMeanHash_const__InputArrayR_const__OutputArrayR_int(input_arr.as_raw__InputArray(), output_arr.as_raw__OutputArray(), mode, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes color moment hash of the input, the algorithm
	/// is come from the paper "Perceptual  Hashing  for  Color  Images
	/// Using  Invariant Moments"
	/// ## Parameters
	/// * inputArr: input image want to compute hash value,
	/// type should be CV_8UC4, CV_8UC3 or CV_8UC1.
	/// * outputArr: 42 hash values with type CV_64F(double)
	#[inline]
	pub fn color_moment_hash(input_arr: &impl core::ToInputArray, output_arr: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(input_arr);
		output_array_arg!(output_arr);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_img_hash_colorMomentHash_const__InputArrayR_const__OutputArrayR(input_arr.as_raw__InputArray(), output_arr.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes average hash value of the input image
	/// ## Parameters
	/// * inputArr: input image want to compute hash value,
	/// type should be CV_8UC4, CV_8UC3, CV_8UC1.
	/// * outputArr: Hash value of input, it will contain 16 hex
	/// decimal number, return type is CV_8U
	/// * alpha: int scale factor for marr wavelet (default=2).
	/// * scale: int level of scale factor (default = 1)
	/// 
	/// ## Note
	/// This alternative version of [marr_hildreth_hash] function uses the following default values for its arguments:
	/// * alpha: 2.0f
	/// * scale: 1.0f
	#[inline]
	pub fn marr_hildreth_hash_def(input_arr: &impl core::ToInputArray, output_arr: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(input_arr);
		output_array_arg!(output_arr);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_img_hash_marrHildrethHash_const__InputArrayR_const__OutputArrayR(input_arr.as_raw__InputArray(), output_arr.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes average hash value of the input image
	/// ## Parameters
	/// * inputArr: input image want to compute hash value,
	/// type should be CV_8UC4, CV_8UC3, CV_8UC1.
	/// * outputArr: Hash value of input, it will contain 16 hex
	/// decimal number, return type is CV_8U
	/// * alpha: int scale factor for marr wavelet (default=2).
	/// * scale: int level of scale factor (default = 1)
	/// 
	/// ## C++ default parameters
	/// * alpha: 2.0f
	/// * scale: 1.0f
	#[inline]
	pub fn marr_hildreth_hash(input_arr: &impl core::ToInputArray, output_arr: &mut impl core::ToOutputArray, alpha: f32, scale: f32) -> Result<()> {
		input_array_arg!(input_arr);
		output_array_arg!(output_arr);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_img_hash_marrHildrethHash_const__InputArrayR_const__OutputArrayR_float_float(input_arr.as_raw__InputArray(), output_arr.as_raw__OutputArray(), alpha, scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes pHash value of the input image
	/// ## Parameters
	/// * inputArr: input image want to compute hash value,
	///  type should be CV_8UC4, CV_8UC3, CV_8UC1.
	/// * outputArr: Hash value of input, it will contain 8 uchar value
	#[inline]
	pub fn p_hash(input_arr: &impl core::ToInputArray, output_arr: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(input_arr);
		output_array_arg!(output_arr);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_img_hash_pHash_const__InputArrayR_const__OutputArrayR(input_arr.as_raw__InputArray(), output_arr.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes radial variance hash of the input image
	/// ## Parameters
	/// * inputArr: input image want to compute hash value,
	/// type should be CV_8UC4, CV_8UC3, CV_8UC1.
	/// * outputArr: Hash value of input
	/// * sigma: Gaussian kernel standard deviation
	/// * numOfAngleLine: The number of angles to consider
	/// 
	/// ## Note
	/// This alternative version of [radial_variance_hash] function uses the following default values for its arguments:
	/// * sigma: 1
	/// * num_of_angle_line: 180
	#[inline]
	pub fn radial_variance_hash_def(input_arr: &impl core::ToInputArray, output_arr: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(input_arr);
		output_array_arg!(output_arr);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_img_hash_radialVarianceHash_const__InputArrayR_const__OutputArrayR(input_arr.as_raw__InputArray(), output_arr.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Computes radial variance hash of the input image
	/// ## Parameters
	/// * inputArr: input image want to compute hash value,
	/// type should be CV_8UC4, CV_8UC3, CV_8UC1.
	/// * outputArr: Hash value of input
	/// * sigma: Gaussian kernel standard deviation
	/// * numOfAngleLine: The number of angles to consider
	/// 
	/// ## C++ default parameters
	/// * sigma: 1
	/// * num_of_angle_line: 180
	#[inline]
	pub fn radial_variance_hash(input_arr: &impl core::ToInputArray, output_arr: &mut impl core::ToOutputArray, sigma: f64, num_of_angle_line: i32) -> Result<()> {
		input_array_arg!(input_arr);
		output_array_arg!(output_arr);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_img_hash_radialVarianceHash_const__InputArrayR_const__OutputArrayR_double_int(input_arr.as_raw__InputArray(), output_arr.as_raw__OutputArray(), sigma, num_of_angle_line, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Constant methods for [crate::img_hash::AverageHash]
	pub trait AverageHashTraitConst: crate::img_hash::ImgHashBaseTraitConst {
		fn as_raw_AverageHash(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::img_hash::AverageHash]
	pub trait AverageHashTrait: crate::img_hash::AverageHashTraitConst + crate::img_hash::ImgHashBaseTrait {
		fn as_raw_mut_AverageHash(&mut self) -> *mut c_void;
	
	}
	
	/// Computes average hash value of the input image
	/// 
	/// This is a fast image hashing algorithm, but only work on simple case. For more details, please
	/// refer to [lookslikeit](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_lookslikeit)
	pub struct AverageHash {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { AverageHash }
	
	impl Drop for AverageHash {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_img_hash_AverageHash_delete(self.as_raw_mut_AverageHash()) };
		}
	}
	
	unsafe impl Send for AverageHash {}
	
	impl core::AlgorithmTraitConst for AverageHash {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for AverageHash {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTraitConst for AverageHash {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for AverageHash {
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::img_hash::AverageHashTraitConst for AverageHash {
		#[inline] fn as_raw_AverageHash(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::img_hash::AverageHashTrait for AverageHash {
		#[inline] fn as_raw_mut_AverageHash(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl AverageHash {
		#[inline]
		pub fn create() -> Result<core::Ptr<crate::img_hash::AverageHash>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_AverageHash_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::img_hash::AverageHash>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { AverageHash, core::Algorithm, cv_img_hash_AverageHash_to_Algorithm }
	
	boxed_cast_base! { AverageHash, crate::img_hash::ImgHashBase, cv_img_hash_AverageHash_to_ImgHashBase }
	
	impl std::fmt::Debug for AverageHash {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("AverageHash")
				.finish()
		}
	}
	
	/// Constant methods for [crate::img_hash::BlockMeanHash]
	pub trait BlockMeanHashTraitConst: crate::img_hash::ImgHashBaseTraitConst {
		fn as_raw_BlockMeanHash(&self) -> *const c_void;
	
		#[inline]
		fn get_mean(&self) -> Result<core::Vector<f64>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_BlockMeanHash_getMean_const(self.as_raw_BlockMeanHash(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<f64>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::img_hash::BlockMeanHash]
	pub trait BlockMeanHashTrait: crate::img_hash::BlockMeanHashTraitConst + crate::img_hash::ImgHashBaseTrait {
		fn as_raw_mut_BlockMeanHash(&mut self) -> *mut c_void;
	
		/// Create BlockMeanHash object
		/// ## Parameters
		/// * mode: the mode
		#[inline]
		fn set_mode(&mut self, mode: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_BlockMeanHash_setMode_int(self.as_raw_mut_BlockMeanHash(), mode, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Image hash based on block mean.
	/// 
	/// See [zauner2010implementation](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_zauner2010implementation) for details.
	pub struct BlockMeanHash {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { BlockMeanHash }
	
	impl Drop for BlockMeanHash {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_img_hash_BlockMeanHash_delete(self.as_raw_mut_BlockMeanHash()) };
		}
	}
	
	unsafe impl Send for BlockMeanHash {}
	
	impl core::AlgorithmTraitConst for BlockMeanHash {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for BlockMeanHash {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTraitConst for BlockMeanHash {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for BlockMeanHash {
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::img_hash::BlockMeanHashTraitConst for BlockMeanHash {
		#[inline] fn as_raw_BlockMeanHash(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::img_hash::BlockMeanHashTrait for BlockMeanHash {
		#[inline] fn as_raw_mut_BlockMeanHash(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl BlockMeanHash {
		/// ## C++ default parameters
		/// * mode: BLOCK_MEAN_HASH_MODE_0
		#[inline]
		pub fn create(mode: i32) -> Result<core::Ptr<crate::img_hash::BlockMeanHash>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_BlockMeanHash_create_int(mode, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::img_hash::BlockMeanHash>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * mode: BLOCK_MEAN_HASH_MODE_0
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::img_hash::BlockMeanHash>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_BlockMeanHash_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::img_hash::BlockMeanHash>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { BlockMeanHash, core::Algorithm, cv_img_hash_BlockMeanHash_to_Algorithm }
	
	boxed_cast_base! { BlockMeanHash, crate::img_hash::ImgHashBase, cv_img_hash_BlockMeanHash_to_ImgHashBase }
	
	impl std::fmt::Debug for BlockMeanHash {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("BlockMeanHash")
				.finish()
		}
	}
	
	/// Constant methods for [crate::img_hash::ColorMomentHash]
	pub trait ColorMomentHashTraitConst: crate::img_hash::ImgHashBaseTraitConst {
		fn as_raw_ColorMomentHash(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::img_hash::ColorMomentHash]
	pub trait ColorMomentHashTrait: crate::img_hash::ColorMomentHashTraitConst + crate::img_hash::ImgHashBaseTrait {
		fn as_raw_mut_ColorMomentHash(&mut self) -> *mut c_void;
	
	}
	
	/// Image hash based on color moments.
	/// 
	/// See [tang2012perceptual](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_tang2012perceptual) for details.
	pub struct ColorMomentHash {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ColorMomentHash }
	
	impl Drop for ColorMomentHash {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_img_hash_ColorMomentHash_delete(self.as_raw_mut_ColorMomentHash()) };
		}
	}
	
	unsafe impl Send for ColorMomentHash {}
	
	impl core::AlgorithmTraitConst for ColorMomentHash {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ColorMomentHash {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTraitConst for ColorMomentHash {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for ColorMomentHash {
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::img_hash::ColorMomentHashTraitConst for ColorMomentHash {
		#[inline] fn as_raw_ColorMomentHash(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::img_hash::ColorMomentHashTrait for ColorMomentHash {
		#[inline] fn as_raw_mut_ColorMomentHash(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ColorMomentHash {
		#[inline]
		pub fn create() -> Result<core::Ptr<crate::img_hash::ColorMomentHash>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_ColorMomentHash_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::img_hash::ColorMomentHash>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { ColorMomentHash, core::Algorithm, cv_img_hash_ColorMomentHash_to_Algorithm }
	
	boxed_cast_base! { ColorMomentHash, crate::img_hash::ImgHashBase, cv_img_hash_ColorMomentHash_to_ImgHashBase }
	
	impl std::fmt::Debug for ColorMomentHash {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ColorMomentHash")
				.finish()
		}
	}
	
	/// Constant methods for [crate::img_hash::ImgHashBase]
	pub trait ImgHashBaseTraitConst: core::AlgorithmTraitConst {
		fn as_raw_ImgHashBase(&self) -> *const c_void;
	
		/// Compare the hash value between inOne and inTwo
		/// ## Parameters
		/// * hashOne: Hash value one
		/// * hashTwo: Hash value two
		/// ## Returns
		/// value indicate similarity between inOne and inTwo, the meaning
		/// of the value vary from algorithms to algorithms
		#[inline]
		fn compare(&self, hash_one: &impl core::ToInputArray, hash_two: &impl core::ToInputArray) -> Result<f64> {
			input_array_arg!(hash_one);
			input_array_arg!(hash_two);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_ImgHashBase_compare_const_const__InputArrayR_const__InputArrayR(self.as_raw_ImgHashBase(), hash_one.as_raw__InputArray(), hash_two.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::img_hash::ImgHashBase]
	pub trait ImgHashBaseTrait: core::AlgorithmTrait + crate::img_hash::ImgHashBaseTraitConst {
		fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void;
	
		/// Computes hash of the input image
		/// ## Parameters
		/// * inputArr: input image want to compute hash value
		/// * outputArr: hash of the image
		#[inline]
		fn compute(&mut self, input_arr: &impl core::ToInputArray, output_arr: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(input_arr);
			output_array_arg!(output_arr);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_ImgHashBase_compute_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_ImgHashBase(), input_arr.as_raw__InputArray(), output_arr.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// The base class for image hash algorithms
	pub struct ImgHashBase {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ImgHashBase }
	
	impl Drop for ImgHashBase {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_img_hash_ImgHashBase_delete(self.as_raw_mut_ImgHashBase()) };
		}
	}
	
	unsafe impl Send for ImgHashBase {}
	
	impl core::AlgorithmTraitConst for ImgHashBase {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ImgHashBase {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTraitConst for ImgHashBase {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for ImgHashBase {
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ImgHashBase {
	}
	
	boxed_cast_base! { ImgHashBase, core::Algorithm, cv_img_hash_ImgHashBase_to_Algorithm }
	
	impl std::fmt::Debug for ImgHashBase {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ImgHashBase")
				.finish()
		}
	}
	
	/// Constant methods for [crate::img_hash::MarrHildrethHash]
	pub trait MarrHildrethHashTraitConst: crate::img_hash::ImgHashBaseTraitConst {
		fn as_raw_MarrHildrethHash(&self) -> *const c_void;
	
		/// self explain
		#[inline]
		fn get_alpha(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_MarrHildrethHash_getAlpha_const(self.as_raw_MarrHildrethHash(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// self explain
		#[inline]
		fn get_scale(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_MarrHildrethHash_getScale_const(self.as_raw_MarrHildrethHash(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::img_hash::MarrHildrethHash]
	pub trait MarrHildrethHashTrait: crate::img_hash::ImgHashBaseTrait + crate::img_hash::MarrHildrethHashTraitConst {
		fn as_raw_mut_MarrHildrethHash(&mut self) -> *mut c_void;
	
		/// Set Mh kernel parameters
		/// ## Parameters
		/// * alpha: int scale factor for marr wavelet (default=2).
		/// * scale: int level of scale factor (default = 1)
		#[inline]
		fn set_kernel_param(&mut self, alpha: f32, scale: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_MarrHildrethHash_setKernelParam_float_float(self.as_raw_mut_MarrHildrethHash(), alpha, scale, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Marr-Hildreth Operator Based Hash, slowest but more discriminative.
	/// 
	/// See [zauner2010implementation](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_zauner2010implementation) for details.
	pub struct MarrHildrethHash {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { MarrHildrethHash }
	
	impl Drop for MarrHildrethHash {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_img_hash_MarrHildrethHash_delete(self.as_raw_mut_MarrHildrethHash()) };
		}
	}
	
	unsafe impl Send for MarrHildrethHash {}
	
	impl core::AlgorithmTraitConst for MarrHildrethHash {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for MarrHildrethHash {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTraitConst for MarrHildrethHash {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for MarrHildrethHash {
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::img_hash::MarrHildrethHashTraitConst for MarrHildrethHash {
		#[inline] fn as_raw_MarrHildrethHash(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::img_hash::MarrHildrethHashTrait for MarrHildrethHash {
		#[inline] fn as_raw_mut_MarrHildrethHash(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl MarrHildrethHash {
		/// ## Parameters
		/// * alpha: int scale factor for marr wavelet (default=2).
		/// * scale: int level of scale factor (default = 1)
		/// 
		/// ## C++ default parameters
		/// * alpha: 2.0f
		/// * scale: 1.0f
		#[inline]
		pub fn create(alpha: f32, scale: f32) -> Result<core::Ptr<crate::img_hash::MarrHildrethHash>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_MarrHildrethHash_create_float_float(alpha, scale, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::img_hash::MarrHildrethHash>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Parameters
		/// * alpha: int scale factor for marr wavelet (default=2).
		/// * scale: int level of scale factor (default = 1)
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * alpha: 2.0f
		/// * scale: 1.0f
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::img_hash::MarrHildrethHash>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_MarrHildrethHash_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::img_hash::MarrHildrethHash>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { MarrHildrethHash, core::Algorithm, cv_img_hash_MarrHildrethHash_to_Algorithm }
	
	boxed_cast_base! { MarrHildrethHash, crate::img_hash::ImgHashBase, cv_img_hash_MarrHildrethHash_to_ImgHashBase }
	
	impl std::fmt::Debug for MarrHildrethHash {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("MarrHildrethHash")
				.finish()
		}
	}
	
	/// Constant methods for [crate::img_hash::PHash]
	pub trait PHashTraitConst: crate::img_hash::ImgHashBaseTraitConst {
		fn as_raw_PHash(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::img_hash::PHash]
	pub trait PHashTrait: crate::img_hash::ImgHashBaseTrait + crate::img_hash::PHashTraitConst {
		fn as_raw_mut_PHash(&mut self) -> *mut c_void;
	
	}
	
	/// pHash
	/// 
	/// Slower than average_hash, but tolerant of minor modifications
	/// 
	/// This algorithm can combat more variation than averageHash, for more details please refer to [lookslikeit](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_lookslikeit)
	pub struct PHash {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { PHash }
	
	impl Drop for PHash {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_img_hash_PHash_delete(self.as_raw_mut_PHash()) };
		}
	}
	
	unsafe impl Send for PHash {}
	
	impl core::AlgorithmTraitConst for PHash {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for PHash {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTraitConst for PHash {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for PHash {
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::img_hash::PHashTraitConst for PHash {
		#[inline] fn as_raw_PHash(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::img_hash::PHashTrait for PHash {
		#[inline] fn as_raw_mut_PHash(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl PHash {
		#[inline]
		pub fn create() -> Result<core::Ptr<crate::img_hash::PHash>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_PHash_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::img_hash::PHash>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { PHash, core::Algorithm, cv_img_hash_PHash_to_Algorithm }
	
	boxed_cast_base! { PHash, crate::img_hash::ImgHashBase, cv_img_hash_PHash_to_ImgHashBase }
	
	impl std::fmt::Debug for PHash {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PHash")
				.finish()
		}
	}
	
	/// Constant methods for [crate::img_hash::RadialVarianceHash]
	pub trait RadialVarianceHashTraitConst: crate::img_hash::ImgHashBaseTraitConst {
		fn as_raw_RadialVarianceHash(&self) -> *const c_void;
	
		#[inline]
		fn get_num_of_angle_line(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_RadialVarianceHash_getNumOfAngleLine_const(self.as_raw_RadialVarianceHash(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_sigma(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_RadialVarianceHash_getSigma_const(self.as_raw_RadialVarianceHash(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::img_hash::RadialVarianceHash]
	pub trait RadialVarianceHashTrait: crate::img_hash::ImgHashBaseTrait + crate::img_hash::RadialVarianceHashTraitConst {
		fn as_raw_mut_RadialVarianceHash(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_num_of_angle_line(&mut self, value: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_RadialVarianceHash_setNumOfAngleLine_int(self.as_raw_mut_RadialVarianceHash(), value, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_sigma(&mut self, value: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_RadialVarianceHash_setSigma_double(self.as_raw_mut_RadialVarianceHash(), value, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_features(&mut self) -> Result<core::Vector<f64>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_RadialVarianceHash_getFeatures(self.as_raw_mut_RadialVarianceHash(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<f64>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_hash(&mut self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_RadialVarianceHash_getHash(self.as_raw_mut_RadialVarianceHash(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_pix_per_line(&mut self, input: &core::Mat) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_RadialVarianceHash_getPixPerLine_const_MatR(self.as_raw_mut_RadialVarianceHash(), input.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn get_projection(&mut self) -> Result<core::Mat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_RadialVarianceHash_getProjection(self.as_raw_mut_RadialVarianceHash(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Image hash based on Radon transform.
	/// 
	/// See [tang2012perceptual](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_tang2012perceptual) for details.
	pub struct RadialVarianceHash {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { RadialVarianceHash }
	
	impl Drop for RadialVarianceHash {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_img_hash_RadialVarianceHash_delete(self.as_raw_mut_RadialVarianceHash()) };
		}
	}
	
	unsafe impl Send for RadialVarianceHash {}
	
	impl core::AlgorithmTraitConst for RadialVarianceHash {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for RadialVarianceHash {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTraitConst for RadialVarianceHash {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for RadialVarianceHash {
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::img_hash::RadialVarianceHashTraitConst for RadialVarianceHash {
		#[inline] fn as_raw_RadialVarianceHash(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::img_hash::RadialVarianceHashTrait for RadialVarianceHash {
		#[inline] fn as_raw_mut_RadialVarianceHash(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl RadialVarianceHash {
		/// ## C++ default parameters
		/// * sigma: 1
		/// * num_of_angle_line: 180
		#[inline]
		pub fn create(sigma: f64, num_of_angle_line: i32) -> Result<core::Ptr<crate::img_hash::RadialVarianceHash>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_RadialVarianceHash_create_double_int(sigma, num_of_angle_line, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::img_hash::RadialVarianceHash>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * sigma: 1
		/// * num_of_angle_line: 180
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::img_hash::RadialVarianceHash>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_img_hash_RadialVarianceHash_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::img_hash::RadialVarianceHash>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { RadialVarianceHash, core::Algorithm, cv_img_hash_RadialVarianceHash_to_Algorithm }
	
	boxed_cast_base! { RadialVarianceHash, crate::img_hash::ImgHashBase, cv_img_hash_RadialVarianceHash_to_ImgHashBase }
	
	impl std::fmt::Debug for RadialVarianceHash {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("RadialVarianceHash")
				.finish()
		}
	}
}
