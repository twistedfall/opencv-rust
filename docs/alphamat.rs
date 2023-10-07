pub mod alphamat {
	//! # Alpha Matting
	//! Alpha matting is used to extract a foreground object with soft boundaries from a background image.
	//! 
	//! This module is dedicated to computing alpha matte of objects in images from a given input image and a greyscale trimap image that contains information about the foreground, background and unknown pixels. The unknown pixels are assumed to be a combination of foreground and background pixels. The algorithm uses a combination of multiple carefully defined pixels affinities to estimate the opacity of the foreground pixels in the unkown region.
	//! 
	//! The implementation is based on [aksoy2017designing](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_aksoy2017designing).
	//! 
	//! This module was developed by Muskaan Kularia and Sunita Nayak as a project
	//! for Google Summer of Code 2019 (GSoC 19).
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use {  };
	}
	
	/// Compute alpha matte of an object in an image
	/// ## Parameters
	/// * image: Input RGB image
	/// * tmap: Input greyscale trimap image
	/// * result: Output alpha matte image
	/// 
	/// The function infoFlow performs alpha matting on a RGB image using a greyscale trimap image, and outputs a greyscale alpha matte image. The output alpha matte can be used to softly extract the foreground object from a background image. Examples can be found in the samples directory.
	#[inline]
	pub fn info_flow(image: &impl core::ToInputArray, tmap: &impl core::ToInputArray, result: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(tmap);
		output_array_arg!(result);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_alphamat_infoFlow_const__InputArrayR_const__InputArrayR_const__OutputArrayR(image.as_raw__InputArray(), tmap.as_raw__InputArray(), result.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
}
