#![allow(unused_parens)]
//! # Alpha Matting
//! This module is dedicated to compute alpha matting of images, given the input image and an input trimap.
//! The samples directory includes easy examples of how to use the module.
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use {  };
}

/// The implementation is based on Designing Effective Inter-Pixel Information Flow for Natural Image Matting by Yağız Aksoy, Tunç Ozan Aydın and Marc Pollefeys, CVPR 2019.
/// 
/// This module has been originally developed by Muskaan Kularia and Sunita Nayak as a project
/// for Google Summer of Code 2019 (GSoC 19).
pub fn info_flow(image: &dyn core::ToInputArray, tmap: &dyn core::ToInputArray, result: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(image);
	input_array_arg!(tmap);
	output_array_arg!(result);
	unsafe { sys::cv_alphamat_infoFlow_const__InputArrayR_const__InputArrayR_const__OutputArrayR(image.as_raw__InputArray(), tmap.as_raw__InputArray(), result.as_raw__OutputArray()) }.into_result()
}
