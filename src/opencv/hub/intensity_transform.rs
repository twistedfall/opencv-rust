#![allow(unused_parens)]
//! # The module brings implementations of intensity transformation algorithms to adjust image contrast.
//! 
//! Namespace for all functions is cv::intensity_trasnform.
//! 
//! ### Supported Algorithms
//! - Autoscaling
//! - Log Transformations
//! - Power-Law (Gamma) Transformations
//! - Contrast Stretching
//! 
//! Reference from following book and websites:
//! - Digital Image Processing 4th Edition Chapter 3 [Rafael C. Gonzalez, Richard E. Woods] [Gonzalez2018](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Gonzalez2018)
//! - http://www.cs.uregina.ca/Links/class-info/425/Lab3/ [lcs435lab](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_lcs435lab)
//! - https://theailearner.com/2019/01/30/contrast-stretching/ [theailearner](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_theailearner)
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use {  };
}

/// Given an input bgr or grayscale image, apply autoscaling on domain [0, 255] to increase
/// the contrast of the input image and return the resulting image.
/// 
/// ## Parameters
/// * input: input bgr or grayscale image.
/// * output: resulting image of autoscaling.
pub fn autoscaling(input: core::Mat, output: &mut core::Mat) -> Result<()> {
	unsafe { sys::cv_intensity_transform_autoscaling_Mat_MatX(input.as_raw_Mat(), output.as_raw_Mat()) }.into_result()
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
pub fn contrast_stretching(input: core::Mat, output: &mut core::Mat, r1: i32, s1: i32, r2: i32, s2: i32) -> Result<()> {
	unsafe { sys::cv_intensity_transform_contrastStretching_Mat_MatX_int_int_int_int(input.as_raw_Mat(), output.as_raw_Mat(), r1, s1, r2, s2) }.into_result()
}

/// Given an input bgr or grayscale image and constant gamma, apply power-law transformation,
/// a.k.a. gamma correction to the image on domain [0, 255] and return the resulting image.
/// 
/// ## Parameters
/// * input: input bgr or grayscale image.
/// * output: resulting image of gamma corrections.
/// * gamma: constant in c*r^gamma where r is pixel value.
pub fn gamma_correction(input: core::Mat, output: &mut core::Mat, gamma: f32) -> Result<()> {
	unsafe { sys::cv_intensity_transform_gammaCorrection_Mat_MatX_float(input.as_raw_Mat(), output.as_raw_Mat(), gamma) }.into_result()
}

/// Given an input bgr or grayscale image and constant c, apply log transformation to the image
/// on domain [0, 255] and return the resulting image.
/// 
/// ## Parameters
/// * input: input bgr or grayscale image.
/// * output: resulting image of log transformations.
pub fn log_transform(input: core::Mat, output: &mut core::Mat) -> Result<()> {
	unsafe { sys::cv_intensity_transform_logTransform_Mat_MatX(input.as_raw_Mat(), output.as_raw_Mat()) }.into_result()
}
