#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Image Processing
//! 
//! This module includes image-processing functions.
//!    # Image Filtering
//! 
//! Functions and classes described in this section are used to perform various linear or non-linear
//! filtering operations on 2D images (represented as Mat's). It means that for each pixel location
//! ![inline formula](https://latex.codecogs.com/png.latex?%28x%2Cy%29) in the source image (normally, rectangular), its neighborhood is considered and used to
//! compute the response. In case of a linear filter, it is a weighted sum of pixel values. In case of
//! morphological operations, it is the minimum or maximum values, and so on. The computed response is
//! stored in the destination image at the same location ![inline formula](https://latex.codecogs.com/png.latex?%28x%2Cy%29). It means that the output image
//! will be of the same size as the input image. Normally, the functions support multi-channel arrays,
//! in which case every channel is processed independently. Therefore, the output image will also have
//! the same number of channels as the input one.
//! 
//! Another common feature of the functions and classes described in this section is that, unlike
//! simple arithmetic functions, they need to extrapolate values of some non-existing pixels. For
//! example, if you want to smooth an image using a Gaussian ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%203) filter, then, when
//! processing the left-most pixels in each row, you need pixels to the left of them, that is, outside
//! of the image. You can let these pixels be the same as the left-most image pixels ("replicated
//! border" extrapolation method), or assume that all the non-existing pixels are zeros ("constant
//! border" extrapolation method), and so on. OpenCV enables you to specify the extrapolation method.
//! For details, see #BorderTypes
//! 
//! @anchor filter_depths
//! ### Depth combinations
//! Input depth (src.depth()) | Output depth (ddepth)
//! --------------------------|----------------------
//! CV_8U                     | -1/CV_16S/CV_32F/CV_64F
//! CV_16U/CV_16S             | -1/CV_32F/CV_64F
//! CV_32F                    | -1/CV_32F/CV_64F
//! CV_64F                    | -1/CV_64F
//! 
//! 
//! Note: when ddepth=-1, the output image will have the same depth as the source.
//! 
//!    # Geometric Image Transformations
//! 
//! The functions in this section perform various geometrical transformations of 2D images. They do not
//! change the image content but deform the pixel grid and map this deformed grid to the destination
//! image. In fact, to avoid sampling artifacts, the mapping is done in the reverse order, from
//! destination to the source. That is, for each pixel ![inline formula](https://latex.codecogs.com/png.latex?%28x%2C%20y%29) of the destination image, the
//! functions compute coordinates of the corresponding "donor" pixel in the source image and copy the
//! pixel value:
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%3D%20%5Ctexttt%7Bsrc%7D%20%28f%5Fx%28x%2Cy%29%2C%20f%5Fy%28x%2Cy%29%29)
//! 
//! In case when you specify the forward mapping ![inline formula](https://latex.codecogs.com/png.latex?%5Cleft%3Cg%5Fx%2C%20g%5Fy%5Cright%3E%3A%20%5Ctexttt%7Bsrc%7D%20%5Crightarrow%0A%5Ctexttt%7Bdst%7D), the OpenCV functions first compute the corresponding inverse mapping
//! ![inline formula](https://latex.codecogs.com/png.latex?%5Cleft%3Cf%5Fx%2C%20f%5Fy%5Cright%3E%3A%20%5Ctexttt%7Bdst%7D%20%5Crightarrow%20%5Ctexttt%7Bsrc%7D) and then use the above formula.
//! 
//! The actual implementations of the geometrical transformations, from the most generic remap and to
//! the simplest and the fastest resize, need to solve two main problems with the above formula:
//! 
//! - Extrapolation of non-existing pixels. Similarly to the filtering functions described in the
//! previous section, for some ![inline formula](https://latex.codecogs.com/png.latex?%28x%2Cy%29), either one of ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx%28x%2Cy%29), or ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy%28x%2Cy%29), or both
//! of them may fall outside of the image. In this case, an extrapolation method needs to be used.
//! OpenCV provides the same selection of extrapolation methods as in the filtering functions. In
//! addition, it provides the method #BORDER_TRANSPARENT. This means that the corresponding pixels in
//! the destination image will not be modified at all.
//! 
//! - Interpolation of pixel values. Usually ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx%28x%2Cy%29) and ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy%28x%2Cy%29) are floating-point
//! numbers. This means that ![inline formula](https://latex.codecogs.com/png.latex?%5Cleft%3Cf%5Fx%2C%20f%5Fy%5Cright%3E) can be either an affine or perspective
//! transformation, or radial lens distortion correction, and so on. So, a pixel value at fractional
//! coordinates needs to be retrieved. In the simplest case, the coordinates can be just rounded to the
//! nearest integer coordinates and the corresponding pixel can be used. This is called a
//! nearest-neighbor interpolation. However, a better result can be achieved by using more
//! sophisticated [interpolation methods](http://en.wikipedia.org/wiki/Multivariate_interpolation) ,
//! where a polynomial function is fit into some neighborhood of the computed pixel ![inline formula](https://latex.codecogs.com/png.latex?%28f%5Fx%28x%2Cy%29%2C%0Af%5Fy%28x%2Cy%29%29), and then the value of the polynomial at ![inline formula](https://latex.codecogs.com/png.latex?%28f%5Fx%28x%2Cy%29%2C%20f%5Fy%28x%2Cy%29%29) is taken as the
//! interpolated pixel value. In OpenCV, you can choose between several interpolation methods. See
//! resize for details.
//! 
//! 
//! Note: The geometrical transformations do not work with `CV_8S` or `CV_32S` images.
//! 
//!    # Miscellaneous Image Transformations
//!    # Drawing Functions
//! 
//! Drawing functions work with matrices/images of arbitrary depth. The boundaries of the shapes can be
//! rendered with antialiasing (implemented only for 8-bit images for now). All the functions include
//! the parameter color that uses an RGB value (that may be constructed with the Scalar constructor )
//! for color images and brightness for grayscale images. For color images, the channel ordering is
//! normally *Blue, Green, Red*. This is what imshow, imread, and imwrite expect. So, if you form a
//! color using the Scalar constructor, it should look like:
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BScalar%7D%20%28blue%20%5C%5F%20component%2C%20green%20%5C%5F%20component%2C%20red%20%5C%5F%20component%5B%2C%20alpha%20%5C%5F%20component%5D%29)
//! 
//! If you are using your own image rendering and I/O functions, you can use any channel ordering. The
//! drawing functions process each channel independently and do not depend on the channel order or even
//! on the used color space. The whole image can be converted from BGR to RGB or to a different color
//! space using cvtColor .
//! 
//! If a drawn figure is partially or completely outside the image, the drawing functions clip it. Also,
//! many drawing functions can handle pixel coordinates specified with sub-pixel accuracy. This means
//! that the coordinates can be passed as fixed-point numbers encoded as integers. The number of
//! fractional bits is specified by the shift parameter and the real point coordinates are calculated as
//! ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BPoint%7D%28x%2Cy%29%5Crightarrow%5Ctexttt%7BPoint2f%7D%28x%2A2%5E%7B%2Dshift%7D%2Cy%2A2%5E%7B%2Dshift%7D%29) . This feature is
//! especially effective when rendering antialiased shapes.
//! 
//! 
//! Note: The functions do not support alpha-transparency when the target image is 4-channel. In this
//! case, the color[3] is simply copied to the repainted pixels. Thus, if you want to paint
//! semi-transparent shapes, you can paint them in a separate buffer and then blend it with the main
//! image.
//! 
//!    # Color Space Conversions
//!    # ColorMaps in OpenCV
//! 
//! The human perception isn't built for observing fine changes in grayscale images. Human eyes are more
//! sensitive to observing changes between colors, so you often need to recolor your grayscale images to
//! get a clue about them. OpenCV now comes with various colormaps to enhance the visualization in your
//! computer vision application.
//! 
//! In OpenCV you only need applyColorMap to apply a colormap on a given image. The following sample
//! code reads the path to an image from command line, applies a Jet colormap on it and shows the
//! result:
//! 
//! @include snippets/imgproc_applyColorMap.cpp
//! ## See also
//! #ColormapTypes
//! 
//!    # Planar Subdivision
//! 
//! The Subdiv2D class described in this section is used to perform various planar subdivision on
//! a set of 2D points (represented as vector of Point2f). OpenCV subdivides a plane into triangles
//! using the Delaunay's algorithm, which corresponds to the dual graph of the Voronoi diagram.
//! In the figure below, the Delaunay's triangulation is marked with black lines and the Voronoi
//! diagram with red lines.
//! 
//! ![Delaunay triangulation (black) and Voronoi (red)](https://docs.opencv.org/4.3.0/delaunay_voronoi.png)
//! 
//! The subdivisions can be used for the 3D piece-wise transformation of a plane, morphing, fast
//! location of points on the plane, building special graphs (such as NNG,RNG), and so forth.
//! 
//!    # Histograms
//!    # Structural Analysis and Shape Descriptors
//!    # Motion Analysis and Object Tracking
//!    # Feature Detection
//!    # Object Detection
//!    # C API
//!    # Hardware Acceleration Layer
//!        # Functions
//!        # Interface
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::TrackerCSRT_ParamsTrait, super::TrackerCSRT, super::TrackerKCF };
}

/// \brief Feature type to be used in the tracking grayscale, colornames, compressed color-names
/// The modes available now:
///  *   "GRAY" -- Use grayscale values as the feature
///  *   "CN" -- Color-names feature
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TrackerKCF_MODE {
	GRAY = 1,
	CN = 2,
	CUSTOM = 4,
}

opencv_type_enum! { crate::tracking::TrackerKCF_MODE }

pub type TrackerKCF_FeatureExtractorCallbackFN = Option<unsafe extern "C" fn(*const c_void, core::Rect, *mut c_void) -> ()>;
/// the CSRT tracker
/// 
/// The implementation is based on [Lukezic_IJCV2018](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Lukezic_IJCV2018) Discriminative Correlation Filter with Channel and Spatial Reliability
pub trait TrackerCSRT: crate::video::Tracker {
	fn as_raw_TrackerCSRT(&self) -> *const c_void;
	fn as_raw_mut_TrackerCSRT(&mut self) -> *mut c_void;

	fn set_initial_mask(&mut self, mask: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(mask);
		unsafe { sys::cv_tracking_TrackerCSRT_setInitialMask_const__InputArrayR(self.as_raw_mut_TrackerCSRT(), mask.as_raw__InputArray()) }.into_result()
	}
	
}

impl dyn TrackerCSRT + '_ {
	/// Create CSRT tracker instance
	/// ## Parameters
	/// * parameters: CSRT parameters TrackerCSRT::Params
	/// 
	/// ## C++ default parameters
	/// * parameters: TrackerCSRT::Params()
	pub fn create(parameters: &crate::tracking::TrackerCSRT_Params) -> Result<core::Ptr::<dyn crate::tracking::TrackerCSRT>> {
		unsafe { sys::cv_tracking_TrackerCSRT_create_const_ParamsR(parameters.as_raw_TrackerCSRT_Params()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::tracking::TrackerCSRT>::opencv_from_extern(r) } )
	}
	
}
pub trait TrackerCSRT_ParamsTrait {
	fn as_raw_TrackerCSRT_Params(&self) -> *const c_void;
	fn as_raw_mut_TrackerCSRT_Params(&mut self) -> *mut c_void;

	fn use_hog(&self) -> bool {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropUse_hog_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: use_hog")
	}
	
	fn set_use_hog(&mut self, val: bool) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropUse_hog_bool(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_use_hog")
	}
	
	fn use_color_names(&self) -> bool {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropUse_color_names_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: use_color_names")
	}
	
	fn set_use_color_names(&mut self, val: bool) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropUse_color_names_bool(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_use_color_names")
	}
	
	fn use_gray(&self) -> bool {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropUse_gray_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: use_gray")
	}
	
	fn set_use_gray(&mut self, val: bool) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropUse_gray_bool(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_use_gray")
	}
	
	fn use_rgb(&self) -> bool {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropUse_rgb_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: use_rgb")
	}
	
	fn set_use_rgb(&mut self, val: bool) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropUse_rgb_bool(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_use_rgb")
	}
	
	fn use_channel_weights(&self) -> bool {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropUse_channel_weights_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: use_channel_weights")
	}
	
	fn set_use_channel_weights(&mut self, val: bool) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropUse_channel_weights_bool(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_use_channel_weights")
	}
	
	fn use_segmentation(&self) -> bool {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropUse_segmentation_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: use_segmentation")
	}
	
	fn set_use_segmentation(&mut self, val: bool) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropUse_segmentation_bool(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_use_segmentation")
	}
	
	/// Window function: "hann", "cheb", "kaiser"
	fn window_function(&self) -> String {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropWindow_function_const(self.as_raw_TrackerCSRT_Params()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: window_function")
	}
	
	/// Window function: "hann", "cheb", "kaiser"
	fn set_window_function(&mut self, val: &str) -> () {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropWindow_function_string(self.as_raw_mut_TrackerCSRT_Params(), val.opencv_as_extern_mut()) }.into_result().expect("Infallible function failed: set_window_function")
	}
	
	fn kaiser_alpha(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropKaiser_alpha_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: kaiser_alpha")
	}
	
	fn set_kaiser_alpha(&mut self, val: f32) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropKaiser_alpha_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_kaiser_alpha")
	}
	
	fn cheb_attenuation(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropCheb_attenuation_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: cheb_attenuation")
	}
	
	fn set_cheb_attenuation(&mut self, val: f32) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropCheb_attenuation_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_cheb_attenuation")
	}
	
	fn template_size(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropTemplate_size_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: template_size")
	}
	
	fn set_template_size(&mut self, val: f32) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropTemplate_size_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_template_size")
	}
	
	fn gsl_sigma(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropGsl_sigma_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: gsl_sigma")
	}
	
	fn set_gsl_sigma(&mut self, val: f32) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropGsl_sigma_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_gsl_sigma")
	}
	
	fn hog_orientations(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropHog_orientations_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: hog_orientations")
	}
	
	fn set_hog_orientations(&mut self, val: f32) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropHog_orientations_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_hog_orientations")
	}
	
	fn hog_clip(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropHog_clip_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: hog_clip")
	}
	
	fn set_hog_clip(&mut self, val: f32) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropHog_clip_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_hog_clip")
	}
	
	fn padding(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropPadding_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: padding")
	}
	
	fn set_padding(&mut self, val: f32) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropPadding_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_padding")
	}
	
	fn filter_lr(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropFilter_lr_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: filter_lr")
	}
	
	fn set_filter_lr(&mut self, val: f32) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropFilter_lr_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_filter_lr")
	}
	
	fn weights_lr(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropWeights_lr_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: weights_lr")
	}
	
	fn set_weights_lr(&mut self, val: f32) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropWeights_lr_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_weights_lr")
	}
	
	fn num_hog_channels_used(&self) -> i32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropNum_hog_channels_used_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: num_hog_channels_used")
	}
	
	fn set_num_hog_channels_used(&mut self, val: i32) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropNum_hog_channels_used_int(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_num_hog_channels_used")
	}
	
	fn admm_iterations(&self) -> i32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropAdmm_iterations_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: admm_iterations")
	}
	
	fn set_admm_iterations(&mut self, val: i32) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropAdmm_iterations_int(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_admm_iterations")
	}
	
	fn histogram_bins(&self) -> i32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropHistogram_bins_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: histogram_bins")
	}
	
	fn set_histogram_bins(&mut self, val: i32) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropHistogram_bins_int(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_histogram_bins")
	}
	
	fn histogram_lr(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropHistogram_lr_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: histogram_lr")
	}
	
	fn set_histogram_lr(&mut self, val: f32) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropHistogram_lr_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_histogram_lr")
	}
	
	fn background_ratio(&self) -> i32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropBackground_ratio_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: background_ratio")
	}
	
	fn set_background_ratio(&mut self, val: i32) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropBackground_ratio_int(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_background_ratio")
	}
	
	fn number_of_scales(&self) -> i32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropNumber_of_scales_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: number_of_scales")
	}
	
	fn set_number_of_scales(&mut self, val: i32) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropNumber_of_scales_int(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_number_of_scales")
	}
	
	fn scale_sigma_factor(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropScale_sigma_factor_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: scale_sigma_factor")
	}
	
	fn set_scale_sigma_factor(&mut self, val: f32) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropScale_sigma_factor_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_scale_sigma_factor")
	}
	
	fn scale_model_max_area(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropScale_model_max_area_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: scale_model_max_area")
	}
	
	fn set_scale_model_max_area(&mut self, val: f32) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropScale_model_max_area_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_scale_model_max_area")
	}
	
	fn scale_lr(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropScale_lr_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: scale_lr")
	}
	
	fn set_scale_lr(&mut self, val: f32) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropScale_lr_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_scale_lr")
	}
	
	fn scale_step(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropScale_step_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: scale_step")
	}
	
	fn set_scale_step(&mut self, val: f32) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropScale_step_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_scale_step")
	}
	
	/// we lost the target, if the psr is lower than this.
	fn psr_threshold(&self) -> f32 {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_getPropPsr_threshold_const(self.as_raw_TrackerCSRT_Params()) }.into_result().expect("Infallible function failed: psr_threshold")
	}
	
	/// we lost the target, if the psr is lower than this.
	fn set_psr_threshold(&mut self, val: f32) -> () {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_setPropPsr_threshold_float(self.as_raw_mut_TrackerCSRT_Params(), val) }.into_result().expect("Infallible function failed: set_psr_threshold")
	}
	
}

pub struct TrackerCSRT_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerCSRT_Params }

impl Drop for TrackerCSRT_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerCSRT_Params_delete(instance: *mut c_void); }
		unsafe { cv_TrackerCSRT_Params_delete(self.as_raw_mut_TrackerCSRT_Params()) };
	}
}

impl TrackerCSRT_Params {
	#[inline] pub fn as_raw_TrackerCSRT_Params(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_TrackerCSRT_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerCSRT_Params {}

impl crate::tracking::TrackerCSRT_ParamsTrait for TrackerCSRT_Params {
	#[inline] fn as_raw_TrackerCSRT_Params(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_TrackerCSRT_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerCSRT_Params {
	pub fn default() -> Result<crate::tracking::TrackerCSRT_Params> {
		unsafe { sys::cv_tracking_TrackerCSRT_Params_Params() }.into_result().map(|r| unsafe { crate::tracking::TrackerCSRT_Params::opencv_from_extern(r) } )
	}
	
}

/// the KCF (Kernelized Correlation Filter) tracker
/// 
/// * KCF is a novel tracking framework that utilizes properties of circulant matrix to enhance the processing speed.
/// * This tracking method is an implementation of [KCF_ECCV](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_KCF_ECCV) which is extended to KCF with color-names features ([KCF_CN](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_KCF_CN)).
/// * The original paper of KCF is available at <http://www.robots.ox.ac.uk/~joao/publications/henriques_tpami2015.pdf>
/// * as well as the matlab implementation. For more information about KCF with color-names features, please refer to
/// * <http://www.cvl.isy.liu.se/research/objrec/visualtracking/colvistrack/index.html>.
pub trait TrackerKCF: crate::video::Tracker {
	fn as_raw_TrackerKCF(&self) -> *const c_void;
	fn as_raw_mut_TrackerKCF(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * pca_func: false
	fn set_feature_extractor(&mut self, callback: crate::tracking::TrackerKCF_FeatureExtractorCallbackFN, pca_func: bool) -> Result<()> {
		unsafe { sys::cv_tracking_TrackerKCF_setFeatureExtractor_FeatureExtractorCallbackFN_bool(self.as_raw_mut_TrackerKCF(), callback, pca_func) }.into_result()
	}
	
}

impl dyn TrackerKCF + '_ {
	/// Create KCF tracker instance
	/// ## Parameters
	/// * parameters: KCF parameters TrackerKCF::Params
	/// 
	/// ## C++ default parameters
	/// * parameters: TrackerKCF::Params()
	pub fn create(parameters: crate::tracking::TrackerKCF_Params) -> Result<core::Ptr::<dyn crate::tracking::TrackerKCF>> {
		unsafe { sys::cv_tracking_TrackerKCF_create_const_ParamsR(&parameters) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::tracking::TrackerKCF>::opencv_from_extern(r) } )
	}
	
}
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TrackerKCF_Params {
	/// detection confidence threshold
	pub detect_thresh: f32,
	/// gaussian kernel bandwidth
	pub sigma: f32,
	/// regularization
	pub lambda: f32,
	/// linear interpolation factor for adaptation
	pub interp_factor: f32,
	/// spatial bandwidth (proportional to target)
	pub output_sigma_factor: f32,
	/// compression learning rate
	pub pca_learning_rate: f32,
	/// activate the resize feature to improve the processing speed
	pub resize: bool,
	/// split the training coefficients into two matrices
	pub split_coeff: bool,
	/// wrap around the kernel values
	pub wrap_kernel: bool,
	/// activate the pca method to compress the features
	pub compress_feature: bool,
	/// threshold for the ROI size
	pub max_patch_size: i32,
	/// feature size after compression
	pub compressed_size: i32,
	/// compressed descriptors of TrackerKCF::MODE
	pub desc_pca: i32,
	/// non-compressed descriptors of TrackerKCF::MODE
	pub desc_npca: i32,
}

opencv_type_simple! { crate::tracking::TrackerKCF_Params }

impl TrackerKCF_Params {
	pub fn default() -> Result<crate::tracking::TrackerKCF_Params> {
		unsafe { sys::cv_tracking_TrackerKCF_Params_Params() }.into_result()
	}
	
}
