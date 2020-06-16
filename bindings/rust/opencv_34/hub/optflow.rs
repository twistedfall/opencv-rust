#![allow(unused_parens)]
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
//! ![Delaunay triangulation (black) and Voronoi (red)](https://docs.opencv.org/3.4.10/delaunay_voronoi.png)
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
	pub use { super::PCAPriorTrait, super::OpticalFlowPCAFlowTrait, super::GPCPatchDescriptorTrait, super::GPCPatchSampleTrait, super::GPCTrainingSamplesTrait, super::GPCTreeTrait, super::GPCDetailsTrait, super::VariationalRefinement, super::DISOpticalFlow };
}

pub const DISOpticalFlow_PRESET_FAST: i32 = 1;
pub const DISOpticalFlow_PRESET_MEDIUM: i32 = 2;
pub const DISOpticalFlow_PRESET_ULTRAFAST: i32 = 0;
/// Better quality but slow
pub const GPC_DESCRIPTOR_DCT: i32 = 0;
/// Worse quality but much faster
pub const GPC_DESCRIPTOR_WHT: i32 = 1;
/// Descriptor types for the Global Patch Collider.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GPCDescType {
	/// Better quality but slow
	GPC_DESCRIPTOR_DCT = 0,
	/// Worse quality but much faster
	GPC_DESCRIPTOR_WHT = 1,
}

opencv_type_enum! { crate::optflow::GPCDescType }

/// Calculates a global motion orientation in a selected region.
/// 
/// ## Parameters
/// * orientation: Motion gradient orientation image calculated by the function calcMotionGradient
/// * mask: Mask image. It may be a conjunction of a valid gradient mask, also calculated by
/// calcMotionGradient , and the mask of a region whose direction needs to be calculated.
/// * mhi: Motion history image calculated by updateMotionHistory .
/// * timestamp: Timestamp passed to updateMotionHistory .
/// * duration: Maximum duration of a motion track in milliseconds, passed to updateMotionHistory
/// 
/// The function calculates an average motion direction in the selected region and returns the angle
/// between 0 degrees and 360 degrees. The average direction is computed from the weighted orientation
/// histogram, where a recent motion has a larger weight and the motion occurred in the past has a
/// smaller weight, as recorded in mhi .
pub fn calc_global_orientation(orientation: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, mhi: &dyn core::ToInputArray, timestamp: f64, duration: f64) -> Result<f64> {
	input_array_arg!(orientation);
	input_array_arg!(mask);
	input_array_arg!(mhi);
	unsafe { sys::cv_motempl_calcGlobalOrientation_const__InputArrayR_const__InputArrayR_const__InputArrayR_double_double(orientation.as_raw__InputArray(), mask.as_raw__InputArray(), mhi.as_raw__InputArray(), timestamp, duration) }.into_result()
}

/// Calculates a gradient orientation of a motion history image.
/// 
/// ## Parameters
/// * mhi: Motion history single-channel floating-point image.
/// * mask: Output mask image that has the type CV_8UC1 and the same size as mhi . Its non-zero
/// elements mark pixels where the motion gradient data is correct.
/// * orientation: Output motion gradient orientation image that has the same type and the same
/// size as mhi . Each pixel of the image is a motion orientation, from 0 to 360 degrees.
/// * delta1: Minimal (or maximal) allowed difference between mhi values within a pixel
/// neighborhood.
/// * delta2: Maximal (or minimal) allowed difference between mhi values within a pixel
/// neighborhood. That is, the function finds the minimum ( ![inline formula](https://latex.codecogs.com/png.latex?m%28x%2Cy%29) ) and maximum ( ![inline formula](https://latex.codecogs.com/png.latex?M%28x%2Cy%29) ) mhi
/// values over ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%203) neighborhood of each pixel and marks the motion orientation at ![inline formula](https://latex.codecogs.com/png.latex?%28x%2C%20y%29)
/// as valid only if
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cmin%20%28%20%5Ctexttt%7Bdelta1%7D%20%20%2C%20%20%5Ctexttt%7Bdelta2%7D%20%20%29%20%20%5Cle%20%20M%28x%2Cy%29%2Dm%28x%2Cy%29%20%20%5Cle%20%20%20%5Cmax%20%28%20%5Ctexttt%7Bdelta1%7D%20%20%2C%20%5Ctexttt%7Bdelta2%7D%20%29%2E)
/// * apertureSize: Aperture size of the Sobel operator.
/// 
/// The function calculates a gradient orientation at each pixel ![inline formula](https://latex.codecogs.com/png.latex?%28x%2C%20y%29) as:
/// 
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Borientation%7D%20%28x%2Cy%29%3D%20%5Carctan%7B%5Cfrac%7Bd%5Ctexttt%7Bmhi%7D%2Fdy%7D%7Bd%5Ctexttt%7Bmhi%7D%2Fdx%7D%7D)
/// 
/// In fact, fastAtan2 and phase are used so that the computed angle is measured in degrees and covers
/// the full range 0..360. Also, the mask is filled to indicate pixels where the computed angle is
/// valid.
/// 
/// 
/// Note:
///    *   (Python) An example on how to perform a motion template technique can be found at
///        opencv_source_code/samples/python2/motempl.py
/// 
/// ## C++ default parameters
/// * aperture_size: 3
pub fn calc_motion_gradient(mhi: &dyn core::ToInputArray, mask: &mut dyn core::ToOutputArray, orientation: &mut dyn core::ToOutputArray, delta1: f64, delta2: f64, aperture_size: i32) -> Result<()> {
	input_array_arg!(mhi);
	output_array_arg!(mask);
	output_array_arg!(orientation);
	unsafe { sys::cv_motempl_calcMotionGradient_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_double_int(mhi.as_raw__InputArray(), mask.as_raw__OutputArray(), orientation.as_raw__OutputArray(), delta1, delta2, aperture_size) }.into_result()
}

/// Splits a motion history image into a few parts corresponding to separate independent motions (for
/// example, left hand, right hand).
/// 
/// ## Parameters
/// * mhi: Motion history image.
/// * segmask: Image where the found mask should be stored, single-channel, 32-bit floating-point.
/// * boundingRects: Vector containing ROIs of motion connected components.
/// * timestamp: Current time in milliseconds or other units.
/// * segThresh: Segmentation threshold that is recommended to be equal to the interval between
/// motion history "steps" or greater.
/// 
/// The function finds all of the motion segments and marks them in segmask with individual values
/// (1,2,...). It also computes a vector with ROIs of motion connected components. After that the motion
/// direction for every component can be calculated with calcGlobalOrientation using the extracted mask
/// of the particular component.
pub fn segment_motion(mhi: &dyn core::ToInputArray, segmask: &mut dyn core::ToOutputArray, bounding_rects: &mut core::Vector::<core::Rect>, timestamp: f64, seg_thresh: f64) -> Result<()> {
	input_array_arg!(mhi);
	output_array_arg!(segmask);
	unsafe { sys::cv_motempl_segmentMotion_const__InputArrayR_const__OutputArrayR_vector_Rect_R_double_double(mhi.as_raw__InputArray(), segmask.as_raw__OutputArray(), bounding_rects.as_raw_mut_VectorOfRect(), timestamp, seg_thresh) }.into_result()
}

/// Updates the motion history image by a moving silhouette.
/// 
/// ## Parameters
/// * silhouette: Silhouette mask that has non-zero pixels where the motion occurs.
/// * mhi: Motion history image that is updated by the function (single-channel, 32-bit
/// floating-point).
/// * timestamp: Current time in milliseconds or other units.
/// * duration: Maximal duration of the motion track in the same units as timestamp .
/// 
/// The function updates the motion history image as follows:
/// 
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bmhi%7D%20%28x%2Cy%29%3D%20%5Cforkthree%7B%5Ctexttt%7Btimestamp%7D%7D%7Bif%20%5C%28%5Ctexttt%7Bsilhouette%7D%28x%2Cy%29%20%5Cne%200%5C%29%7D%7B0%7D%7Bif%20%5C%28%5Ctexttt%7Bsilhouette%7D%28x%2Cy%29%20%3D%200%5C%29%20and%20%5C%28%5Ctexttt%7Bmhi%7D%20%3C%20%28%5Ctexttt%7Btimestamp%7D%20%2D%20%5Ctexttt%7Bduration%7D%29%5C%29%7D%7B%5Ctexttt%7Bmhi%7D%28x%2Cy%29%7D%7Botherwise%7D)
/// 
/// That is, MHI pixels where the motion occurs are set to the current timestamp , while the pixels
/// where the motion happened last time a long time ago are cleared.
/// 
/// The function, together with calcMotionGradient and calcGlobalOrientation , implements a motion
/// templates technique described in [Davis97](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_Davis97) and [Bradski00](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_Bradski00) .
pub fn update_motion_history(silhouette: &dyn core::ToInputArray, mhi: &mut dyn core::ToInputOutputArray, timestamp: f64, duration: f64) -> Result<()> {
	input_array_arg!(silhouette);
	input_output_array_arg!(mhi);
	unsafe { sys::cv_motempl_updateMotionHistory_const__InputArrayR_const__InputOutputArrayR_double_double(silhouette.as_raw__InputArray(), mhi.as_raw__InputOutputArray(), timestamp, duration) }.into_result()
}

/// Calculate an optical flow using "SimpleFlow" algorithm.
/// 
/// ## Parameters
/// * from: First 8-bit 3-channel image.
/// * to: Second 8-bit 3-channel image of the same size as prev
/// * flow: computed flow image that has the same size as prev and type CV_32FC2
/// * layers: Number of layers
/// * averaging_block_size: Size of block through which we sum up when calculate cost function
/// for pixel
/// * max_flow: maximal flow that we search at each level
/// * sigma_dist: vector smooth spatial sigma parameter
/// * sigma_color: vector smooth color sigma parameter
/// * postprocess_window: window size for postprocess cross bilateral filter
/// * sigma_dist_fix: spatial sigma for postprocess cross bilateralf filter
/// * sigma_color_fix: color sigma for postprocess cross bilateral filter
/// * occ_thr: threshold for detecting occlusions
/// * upscale_averaging_radius: window size for bilateral upscale operation
/// * upscale_sigma_dist: spatial sigma for bilateral upscale operation
/// * upscale_sigma_color: color sigma for bilateral upscale operation
/// * speed_up_thr: threshold to detect point with irregular flow - where flow should be
/// recalculated after upscale
/// 
/// See [Tao2012](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_Tao2012) . And site of project - <http://graphics.berkeley.edu/papers/Tao-SAN-2012-05/>.
/// 
/// 
/// Note:
///    *   An example using the simpleFlow algorithm can be found at samples/simpleflow_demo.cpp
/// 
/// ## Overloaded parameters
pub fn calc_optical_flow_sf(from: &dyn core::ToInputArray, to: &dyn core::ToInputArray, flow: &mut dyn core::ToOutputArray, layers: i32, averaging_block_size: i32, max_flow: i32) -> Result<()> {
	input_array_arg!(from);
	input_array_arg!(to);
	output_array_arg!(flow);
	unsafe { sys::cv_optflow_calcOpticalFlowSF_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int(from.as_raw__InputArray(), to.as_raw__InputArray(), flow.as_raw__OutputArray(), layers, averaging_block_size, max_flow) }.into_result()
}

/// Calculate an optical flow using "SimpleFlow" algorithm.
/// 
/// ## Parameters
/// * from: First 8-bit 3-channel image.
/// * to: Second 8-bit 3-channel image of the same size as prev
/// * flow: computed flow image that has the same size as prev and type CV_32FC2
/// * layers: Number of layers
/// * averaging_block_size: Size of block through which we sum up when calculate cost function
/// for pixel
/// * max_flow: maximal flow that we search at each level
/// * sigma_dist: vector smooth spatial sigma parameter
/// * sigma_color: vector smooth color sigma parameter
/// * postprocess_window: window size for postprocess cross bilateral filter
/// * sigma_dist_fix: spatial sigma for postprocess cross bilateralf filter
/// * sigma_color_fix: color sigma for postprocess cross bilateral filter
/// * occ_thr: threshold for detecting occlusions
/// * upscale_averaging_radius: window size for bilateral upscale operation
/// * upscale_sigma_dist: spatial sigma for bilateral upscale operation
/// * upscale_sigma_color: color sigma for bilateral upscale operation
/// * speed_up_thr: threshold to detect point with irregular flow - where flow should be
/// recalculated after upscale
/// 
/// See [Tao2012](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_Tao2012) . And site of project - <http://graphics.berkeley.edu/papers/Tao-SAN-2012-05/>.
/// 
/// 
/// Note:
///    *   An example using the simpleFlow algorithm can be found at samples/simpleflow_demo.cpp
pub fn calc_optical_flow_sf_1(from: &dyn core::ToInputArray, to: &dyn core::ToInputArray, flow: &mut dyn core::ToOutputArray, layers: i32, averaging_block_size: i32, max_flow: i32, sigma_dist: f64, sigma_color: f64, postprocess_window: i32, sigma_dist_fix: f64, sigma_color_fix: f64, occ_thr: f64, upscale_averaging_radius: i32, upscale_sigma_dist: f64, upscale_sigma_color: f64, speed_up_thr: f64) -> Result<()> {
	input_array_arg!(from);
	input_array_arg!(to);
	output_array_arg!(flow);
	unsafe { sys::cv_optflow_calcOpticalFlowSF_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int_double_double_int_double_double_double_int_double_double_double(from.as_raw__InputArray(), to.as_raw__InputArray(), flow.as_raw__OutputArray(), layers, averaging_block_size, max_flow, sigma_dist, sigma_color, postprocess_window, sigma_dist_fix, sigma_color_fix, occ_thr, upscale_averaging_radius, upscale_sigma_dist, upscale_sigma_color, speed_up_thr) }.into_result()
}

/// Fast dense optical flow based on PyrLK sparse matches interpolation.
/// 
/// ## Parameters
/// * from: first 8-bit 3-channel or 1-channel image.
/// * to: second 8-bit 3-channel or 1-channel image of the same size as from
/// * flow: computed flow image that has the same size as from and CV_32FC2 type
/// * grid_step: stride used in sparse match computation. Lower values usually
///        result in higher quality but slow down the algorithm.
/// * k: number of nearest-neighbor matches considered, when fitting a locally affine
///        model. Lower values can make the algorithm noticeably faster at the cost of
///        some quality degradation.
/// * sigma: parameter defining how fast the weights decrease in the locally-weighted affine
///        fitting. Higher values can help preserve fine details, lower values can help to get rid
///        of the noise in the output flow.
/// * use_post_proc: defines whether the ximgproc::fastGlobalSmootherFilter() is used
///        for post-processing after interpolation
/// * fgs_lambda: see the respective parameter of the ximgproc::fastGlobalSmootherFilter()
/// * fgs_sigma: see the respective parameter of the ximgproc::fastGlobalSmootherFilter()
/// 
/// ## C++ default parameters
/// * grid_step: 8
/// * k: 128
/// * sigma: 0.05f
/// * use_post_proc: true
/// * fgs_lambda: 500.0f
/// * fgs_sigma: 1.5f
pub fn calc_optical_flow_sparse_to_dense(from: &dyn core::ToInputArray, to: &dyn core::ToInputArray, flow: &mut dyn core::ToOutputArray, grid_step: i32, k: i32, sigma: f32, use_post_proc: bool, fgs_lambda: f32, fgs_sigma: f32) -> Result<()> {
	input_array_arg!(from);
	input_array_arg!(to);
	output_array_arg!(flow);
	unsafe { sys::cv_optflow_calcOpticalFlowSparseToDense_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_float_bool_float_float(from.as_raw__InputArray(), to.as_raw__InputArray(), flow.as_raw__OutputArray(), grid_step, k, sigma, use_post_proc, fgs_lambda, fgs_sigma) }.into_result()
}

/// Creates an instance of DISOpticalFlow
/// 
/// ## Parameters
/// * preset: one of PRESET_ULTRAFAST, PRESET_FAST and PRESET_MEDIUM
/// 
/// ## C++ default parameters
/// * preset: DISOpticalFlow::PRESET_FAST
pub fn create_opt_flow_dis(preset: i32) -> Result<core::Ptr::<dyn crate::optflow::DISOpticalFlow>> {
	unsafe { sys::cv_optflow_createOptFlow_DIS_int(preset) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::optflow::DISOpticalFlow>::opencv_from_extern(r) } )
}

/// DeepFlow optical flow algorithm implementation.
/// 
/// The class implements the DeepFlow optical flow algorithm described in [Weinzaepfel2013](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_Weinzaepfel2013) . See
/// also <http://lear.inrialpes.fr/src/deepmatching/> .
/// Parameters - class fields - that may be modified after creating a class instance:
/// *   member float alpha
/// Smoothness assumption weight
/// *   member float delta
/// Color constancy assumption weight
/// *   member float gamma
/// Gradient constancy weight
/// *   member float sigma
/// Gaussian smoothing parameter
/// *   member int minSize
/// Minimal dimension of an image in the pyramid (next, smaller images in the pyramid are generated
/// until one of the dimensions reaches this size)
/// *   member float downscaleFactor
/// Scaling factor in the image pyramid (must be \< 1)
/// *   member int fixedPointIterations
/// How many iterations on each level of the pyramid
/// *   member int sorIterations
/// Iterations of Succesive Over-Relaxation (solver)
/// *   member float omega
/// Relaxation factor in SOR
pub fn create_opt_flow_deep_flow() -> Result<core::Ptr::<dyn crate::video::DenseOpticalFlow>> {
	unsafe { sys::cv_optflow_createOptFlow_DeepFlow() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::video::DenseOpticalFlow>::opencv_from_extern(r) } )
}

/// Additional interface to the Farneback's algorithm - calcOpticalFlowFarneback()
pub fn create_opt_flow_farneback() -> Result<core::Ptr::<dyn crate::video::DenseOpticalFlow>> {
	unsafe { sys::cv_optflow_createOptFlow_Farneback() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::video::DenseOpticalFlow>::opencv_from_extern(r) } )
}

/// Creates an instance of PCAFlow
pub fn create_opt_flow_pca_flow() -> Result<core::Ptr::<dyn crate::video::DenseOpticalFlow>> {
	unsafe { sys::cv_optflow_createOptFlow_PCAFlow() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::video::DenseOpticalFlow>::opencv_from_extern(r) } )
}

/// Additional interface to the SimpleFlow algorithm - calcOpticalFlowSF()
pub fn create_opt_flow_simple_flow() -> Result<core::Ptr::<dyn crate::video::DenseOpticalFlow>> {
	unsafe { sys::cv_optflow_createOptFlow_SimpleFlow() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::video::DenseOpticalFlow>::opencv_from_extern(r) } )
}

/// Additional interface to the SparseToDenseFlow algorithm - calcOpticalFlowSparseToDense()
pub fn create_opt_flow_sparse_to_dense() -> Result<core::Ptr::<dyn crate::video::DenseOpticalFlow>> {
	unsafe { sys::cv_optflow_createOptFlow_SparseToDense() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::video::DenseOpticalFlow>::opencv_from_extern(r) } )
}

/// Creates an instance of VariationalRefinement
pub fn create_variational_flow_refinement() -> Result<core::Ptr::<dyn crate::optflow::VariationalRefinement>> {
	unsafe { sys::cv_optflow_createVariationalFlowRefinement() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::optflow::VariationalRefinement>::opencv_from_extern(r) } )
}

/// Read a .flo file
/// 
/// ## Parameters
/// * path: Path to the file to be loaded
/// 
/// The function readOpticalFlow loads a flow field from a file and returns it as a single matrix.
/// Resulting Mat has a type CV_32FC2 - floating-point, 2-channel. First channel corresponds to the
/// flow in the horizontal direction (u), second - vertical (v).
pub fn read_optical_flow(path: &str) -> Result<core::Mat> {
	extern_container_arg!(path);
	unsafe { sys::cv_optflow_readOpticalFlow_const_StringR(path.opencv_to_extern()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
}

/// Write a .flo to disk
/// 
/// ## Parameters
/// * path: Path to the file to be written
/// * flow: Flow field to be stored
/// 
/// The function stores a flow field in a file, returns true on success, false otherwise.
/// The flow field must be a 2-channel, floating-point matrix (CV_32FC2). First channel corresponds
/// to the flow in the horizontal direction (u), second - vertical (v).
pub fn write_optical_flow(path: &str, flow: &dyn core::ToInputArray) -> Result<bool> {
	extern_container_arg!(path);
	input_array_arg!(flow);
	unsafe { sys::cv_optflow_writeOpticalFlow_const_StringR_const__InputArrayR(path.opencv_to_extern(), flow.as_raw__InputArray()) }.into_result()
}

pub fn read(fn_: &core::FileNode, node: &mut crate::optflow::GPCTree_Node, unnamed: crate::optflow::GPCTree_Node) -> Result<()> {
	unsafe { sys::cv_read_const_FileNodeR_NodeR_Node(fn_.as_raw_FileNode(), node, unnamed.opencv_to_extern()) }.into_result()
}

pub fn write(fs: &mut core::FileStorage, name: &str, node: crate::optflow::GPCTree_Node) -> Result<()> {
	extern_container_arg!(name);
	unsafe { sys::cv_write_FileStorageR_const_StringR_const_NodeR(fs.as_raw_mut_FileStorage(), name.opencv_to_extern(), &node) }.into_result()
}

/// DIS optical flow algorithm.
/// 
/// This class implements the Dense Inverse Search (DIS) optical flow algorithm. More
/// details about the algorithm can be found at [Kroeger2016](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_Kroeger2016) . Includes three presets with preselected
/// parameters to provide reasonable trade-off between speed and quality. However, even the slowest preset is
/// still relatively fast, use DeepFlow if you need better quality and don't care about speed.
/// 
/// This implementation includes several additional features compared to the algorithm described in the paper,
/// including spatial propagation of flow vectors (@ref getUseSpatialPropagation), as well as an option to
/// utilize an initial flow approximation passed to @ref calc (which is, essentially, temporal propagation,
/// if the previous frame's flow field is passed).
pub trait DISOpticalFlow: crate::video::DenseOpticalFlow {
	fn as_raw_DISOpticalFlow(&self) -> *const c_void;
	fn as_raw_mut_DISOpticalFlow(&mut self) -> *mut c_void;

	/// Finest level of the Gaussian pyramid on which the flow is computed (zero level
	/// corresponds to the original image resolution). The final flow is obtained by bilinear upscaling.
	/// ## See also
	/// setFinestScale
	fn get_finest_scale(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_DISOpticalFlow_getFinestScale_const(self.as_raw_DISOpticalFlow()) }.into_result()
	}
	
	/// Finest level of the Gaussian pyramid on which the flow is computed (zero level
	/// corresponds to the original image resolution). The final flow is obtained by bilinear upscaling.
	/// ## See also
	/// setFinestScale getFinestScale
	fn set_finest_scale(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_optflow_DISOpticalFlow_setFinestScale_int(self.as_raw_mut_DISOpticalFlow(), val) }.into_result()
	}
	
	/// Size of an image patch for matching (in pixels). Normally, default 8x8 patches work well
	/// enough in most cases.
	/// ## See also
	/// setPatchSize
	fn get_patch_size(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_DISOpticalFlow_getPatchSize_const(self.as_raw_DISOpticalFlow()) }.into_result()
	}
	
	/// Size of an image patch for matching (in pixels). Normally, default 8x8 patches work well
	/// enough in most cases.
	/// ## See also
	/// setPatchSize getPatchSize
	fn set_patch_size(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_optflow_DISOpticalFlow_setPatchSize_int(self.as_raw_mut_DISOpticalFlow(), val) }.into_result()
	}
	
	/// Stride between neighbor patches. Must be less than patch size. Lower values correspond
	/// to higher flow quality.
	/// ## See also
	/// setPatchStride
	fn get_patch_stride(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_DISOpticalFlow_getPatchStride_const(self.as_raw_DISOpticalFlow()) }.into_result()
	}
	
	/// Stride between neighbor patches. Must be less than patch size. Lower values correspond
	/// to higher flow quality.
	/// ## See also
	/// setPatchStride getPatchStride
	fn set_patch_stride(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_optflow_DISOpticalFlow_setPatchStride_int(self.as_raw_mut_DISOpticalFlow(), val) }.into_result()
	}
	
	/// Maximum number of gradient descent iterations in the patch inverse search stage. Higher values
	/// may improve quality in some cases.
	/// ## See also
	/// setGradientDescentIterations
	fn get_gradient_descent_iterations(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_DISOpticalFlow_getGradientDescentIterations_const(self.as_raw_DISOpticalFlow()) }.into_result()
	}
	
	/// Maximum number of gradient descent iterations in the patch inverse search stage. Higher values
	/// may improve quality in some cases.
	/// ## See also
	/// setGradientDescentIterations getGradientDescentIterations
	fn set_gradient_descent_iterations(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_optflow_DISOpticalFlow_setGradientDescentIterations_int(self.as_raw_mut_DISOpticalFlow(), val) }.into_result()
	}
	
	/// Number of fixed point iterations of variational refinement per scale. Set to zero to
	///    disable variational refinement completely. Higher values will typically result in more smooth and
	///    high-quality flow.
	/// ## See also
	/// setGradientDescentIterations
	fn get_variational_refinement_iterations(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_DISOpticalFlow_getVariationalRefinementIterations_const(self.as_raw_DISOpticalFlow()) }.into_result()
	}
	
	/// Maximum number of gradient descent iterations in the patch inverse search stage. Higher values
	/// may improve quality in some cases.
	/// ## See also
	/// setGradientDescentIterations getGradientDescentIterations
	fn set_variational_refinement_iterations(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_optflow_DISOpticalFlow_setVariationalRefinementIterations_int(self.as_raw_mut_DISOpticalFlow(), val) }.into_result()
	}
	
	/// Weight of the smoothness term
	/// ## See also
	/// setVariationalRefinementAlpha
	fn get_variational_refinement_alpha(&self) -> Result<f32> {
		unsafe { sys::cv_optflow_DISOpticalFlow_getVariationalRefinementAlpha_const(self.as_raw_DISOpticalFlow()) }.into_result()
	}
	
	/// Weight of the smoothness term
	/// ## See also
	/// setVariationalRefinementAlpha getVariationalRefinementAlpha
	fn set_variational_refinement_alpha(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_optflow_DISOpticalFlow_setVariationalRefinementAlpha_float(self.as_raw_mut_DISOpticalFlow(), val) }.into_result()
	}
	
	/// Weight of the color constancy term
	/// ## See also
	/// setVariationalRefinementDelta
	fn get_variational_refinement_delta(&self) -> Result<f32> {
		unsafe { sys::cv_optflow_DISOpticalFlow_getVariationalRefinementDelta_const(self.as_raw_DISOpticalFlow()) }.into_result()
	}
	
	/// Weight of the color constancy term
	/// ## See also
	/// setVariationalRefinementDelta getVariationalRefinementDelta
	fn set_variational_refinement_delta(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_optflow_DISOpticalFlow_setVariationalRefinementDelta_float(self.as_raw_mut_DISOpticalFlow(), val) }.into_result()
	}
	
	/// Weight of the gradient constancy term
	/// ## See also
	/// setVariationalRefinementGamma
	fn get_variational_refinement_gamma(&self) -> Result<f32> {
		unsafe { sys::cv_optflow_DISOpticalFlow_getVariationalRefinementGamma_const(self.as_raw_DISOpticalFlow()) }.into_result()
	}
	
	/// Weight of the gradient constancy term
	/// ## See also
	/// setVariationalRefinementGamma getVariationalRefinementGamma
	fn set_variational_refinement_gamma(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_optflow_DISOpticalFlow_setVariationalRefinementGamma_float(self.as_raw_mut_DISOpticalFlow(), val) }.into_result()
	}
	
	/// Whether to use mean-normalization of patches when computing patch distance. It is turned on
	///    by default as it typically provides a noticeable quality boost because of increased robustness to
	///    illumination variations. Turn it off if you are certain that your sequence doesn't contain any changes
	///    in illumination.
	/// ## See also
	/// setUseMeanNormalization
	fn get_use_mean_normalization(&self) -> Result<bool> {
		unsafe { sys::cv_optflow_DISOpticalFlow_getUseMeanNormalization_const(self.as_raw_DISOpticalFlow()) }.into_result()
	}
	
	/// Whether to use mean-normalization of patches when computing patch distance. It is turned on
	///    by default as it typically provides a noticeable quality boost because of increased robustness to
	///    illumination variations. Turn it off if you are certain that your sequence doesn't contain any changes
	///    in illumination.
	/// ## See also
	/// setUseMeanNormalization getUseMeanNormalization
	fn set_use_mean_normalization(&mut self, val: bool) -> Result<()> {
		unsafe { sys::cv_optflow_DISOpticalFlow_setUseMeanNormalization_bool(self.as_raw_mut_DISOpticalFlow(), val) }.into_result()
	}
	
	/// Whether to use spatial propagation of good optical flow vectors. This option is turned on by
	///    default, as it tends to work better on average and can sometimes help recover from major errors
	///    introduced by the coarse-to-fine scheme employed by the DIS optical flow algorithm. Turning this
	///    option off can make the output flow field a bit smoother, however.
	/// ## See also
	/// setUseSpatialPropagation
	fn get_use_spatial_propagation(&self) -> Result<bool> {
		unsafe { sys::cv_optflow_DISOpticalFlow_getUseSpatialPropagation_const(self.as_raw_DISOpticalFlow()) }.into_result()
	}
	
	/// Whether to use spatial propagation of good optical flow vectors. This option is turned on by
	///    default, as it tends to work better on average and can sometimes help recover from major errors
	///    introduced by the coarse-to-fine scheme employed by the DIS optical flow algorithm. Turning this
	///    option off can make the output flow field a bit smoother, however.
	/// ## See also
	/// setUseSpatialPropagation getUseSpatialPropagation
	fn set_use_spatial_propagation(&mut self, val: bool) -> Result<()> {
		unsafe { sys::cv_optflow_DISOpticalFlow_setUseSpatialPropagation_bool(self.as_raw_mut_DISOpticalFlow(), val) }.into_result()
	}
	
}

pub trait GPCDetailsTrait {
	fn as_raw_GPCDetails(&self) -> *const c_void;
	fn as_raw_mut_GPCDetails(&mut self) -> *mut c_void;

}

pub struct GPCDetails {
	ptr: *mut c_void
}

opencv_type_boxed! { GPCDetails }

impl Drop for GPCDetails {
	fn drop(&mut self) {
		extern "C" { fn cv_GPCDetails_delete(instance: *mut c_void); }
		unsafe { cv_GPCDetails_delete(self.as_raw_mut_GPCDetails()) };
	}
}

impl GPCDetails {
	#[inline] pub fn as_raw_GPCDetails(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_GPCDetails(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for GPCDetails {}

impl crate::optflow::GPCDetailsTrait for GPCDetails {
	#[inline] fn as_raw_GPCDetails(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_GPCDetails(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GPCDetails {
	pub fn get_all_descriptors_for_image(img_ch: &core::Mat, descr: &mut core::Vector::<crate::optflow::GPCPatchDescriptor>, mp: crate::optflow::GPCMatchingParams, typ: i32) -> Result<()> {
		unsafe { sys::cv_optflow_GPCDetails_getAllDescriptorsForImage_const_MatX_vector_GPCPatchDescriptor_R_const_GPCMatchingParamsR_int(img_ch.as_raw_Mat(), descr.as_raw_mut_VectorOfGPCPatchDescriptor(), &mp, typ) }.into_result()
	}
	
	pub fn get_coordinates_from_index(index: size_t, sz: core::Size, x: &mut i32, y: &mut i32) -> Result<()> {
		unsafe { sys::cv_optflow_GPCDetails_getCoordinatesFromIndex_size_t_Size_intR_intR(index, sz.opencv_to_extern(), x, y) }.into_result()
	}
	
}

/// Class encapsulating matching parameters.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPCMatchingParams {
	/// Whether to use OpenCL to speed up the matching.
	pub use_opencl: bool,
}

opencv_type_simple! { crate::optflow::GPCMatchingParams }

impl GPCMatchingParams {
	/// ## C++ default parameters
	/// * _use_opencl: false
	pub fn new(_use_opencl: bool) -> Result<crate::optflow::GPCMatchingParams> {
		unsafe { sys::cv_optflow_GPCMatchingParams_GPCMatchingParams_bool(_use_opencl) }.into_result()
	}
	
	pub fn copy(params: crate::optflow::GPCMatchingParams) -> Result<crate::optflow::GPCMatchingParams> {
		unsafe { sys::cv_optflow_GPCMatchingParams_GPCMatchingParams_const_GPCMatchingParamsR(&params) }.into_result()
	}
	
}

pub trait GPCPatchDescriptorTrait {
	fn as_raw_GPCPatchDescriptor(&self) -> *const c_void;
	fn as_raw_mut_GPCPatchDescriptor(&mut self) -> *mut c_void;

	fn feature(&self) -> core::Vec18<f64> {
		unsafe { sys::cv_optflow_GPCPatchDescriptor_getPropFeature_const(self.as_raw_GPCPatchDescriptor()) }.into_result().expect("Infallible function failed: feature")
	}
	
	fn set_feature(&mut self, val: core::Vec18<f64>) -> () {
		unsafe { sys::cv_optflow_GPCPatchDescriptor_setPropFeature_Vec_double__18_(self.as_raw_mut_GPCPatchDescriptor(), val.opencv_to_extern()) }.into_result().expect("Infallible function failed: set_feature")
	}
	
	fn dot(&self, coef: core::Vec18<f64>) -> Result<f64> {
		unsafe { sys::cv_optflow_GPCPatchDescriptor_dot_const_const_Vec_double__18_R(self.as_raw_GPCPatchDescriptor(), &coef) }.into_result()
	}
	
	fn mark_as_separated(&mut self) -> Result<()> {
		unsafe { sys::cv_optflow_GPCPatchDescriptor_markAsSeparated(self.as_raw_mut_GPCPatchDescriptor()) }.into_result()
	}
	
	fn is_separated(&self) -> Result<bool> {
		unsafe { sys::cv_optflow_GPCPatchDescriptor_isSeparated_const(self.as_raw_GPCPatchDescriptor()) }.into_result()
	}
	
}

pub struct GPCPatchDescriptor {
	ptr: *mut c_void
}

opencv_type_boxed! { GPCPatchDescriptor }

impl Drop for GPCPatchDescriptor {
	fn drop(&mut self) {
		extern "C" { fn cv_GPCPatchDescriptor_delete(instance: *mut c_void); }
		unsafe { cv_GPCPatchDescriptor_delete(self.as_raw_mut_GPCPatchDescriptor()) };
	}
}

impl GPCPatchDescriptor {
	#[inline] pub fn as_raw_GPCPatchDescriptor(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_GPCPatchDescriptor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for GPCPatchDescriptor {}

impl crate::optflow::GPCPatchDescriptorTrait for GPCPatchDescriptor {
	#[inline] fn as_raw_GPCPatchDescriptor(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_GPCPatchDescriptor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GPCPatchDescriptor {
	/// number of features in a patch descriptor
	pub const nFeatures: u32 = 18;
}

pub trait GPCPatchSampleTrait {
	fn as_raw_GPCPatchSample(&self) -> *const c_void;
	fn as_raw_mut_GPCPatchSample(&mut self) -> *mut c_void;

	fn ref_(&mut self) -> crate::optflow::GPCPatchDescriptor {
		unsafe { sys::cv_optflow_GPCPatchSample_getPropRef(self.as_raw_mut_GPCPatchSample()) }.into_result().map(|r| unsafe { crate::optflow::GPCPatchDescriptor::opencv_from_extern(r) } ).expect("Infallible function failed: ref_")
	}
	
	fn set_ref(&mut self, mut val: crate::optflow::GPCPatchDescriptor) -> () {
		unsafe { sys::cv_optflow_GPCPatchSample_setPropRef_GPCPatchDescriptor(self.as_raw_mut_GPCPatchSample(), val.as_raw_mut_GPCPatchDescriptor()) }.into_result().expect("Infallible function failed: set_ref")
	}
	
	fn pos(&mut self) -> crate::optflow::GPCPatchDescriptor {
		unsafe { sys::cv_optflow_GPCPatchSample_getPropPos(self.as_raw_mut_GPCPatchSample()) }.into_result().map(|r| unsafe { crate::optflow::GPCPatchDescriptor::opencv_from_extern(r) } ).expect("Infallible function failed: pos")
	}
	
	fn set_pos(&mut self, mut val: crate::optflow::GPCPatchDescriptor) -> () {
		unsafe { sys::cv_optflow_GPCPatchSample_setPropPos_GPCPatchDescriptor(self.as_raw_mut_GPCPatchSample(), val.as_raw_mut_GPCPatchDescriptor()) }.into_result().expect("Infallible function failed: set_pos")
	}
	
	fn neg(&mut self) -> crate::optflow::GPCPatchDescriptor {
		unsafe { sys::cv_optflow_GPCPatchSample_getPropNeg(self.as_raw_mut_GPCPatchSample()) }.into_result().map(|r| unsafe { crate::optflow::GPCPatchDescriptor::opencv_from_extern(r) } ).expect("Infallible function failed: neg")
	}
	
	fn set_neg(&mut self, mut val: crate::optflow::GPCPatchDescriptor) -> () {
		unsafe { sys::cv_optflow_GPCPatchSample_setPropNeg_GPCPatchDescriptor(self.as_raw_mut_GPCPatchSample(), val.as_raw_mut_GPCPatchDescriptor()) }.into_result().expect("Infallible function failed: set_neg")
	}
	
	fn get_directions(&self, refdir: &mut bool, posdir: &mut bool, negdir: &mut bool, coef: core::Vec18<f64>, rhs: f64) -> Result<()> {
		unsafe { sys::cv_optflow_GPCPatchSample_getDirections_const_boolR_boolR_boolR_const_Vec_double__18_R_double(self.as_raw_GPCPatchSample(), refdir, posdir, negdir, &coef, rhs) }.into_result()
	}
	
}

pub struct GPCPatchSample {
	ptr: *mut c_void
}

opencv_type_boxed! { GPCPatchSample }

impl Drop for GPCPatchSample {
	fn drop(&mut self) {
		extern "C" { fn cv_GPCPatchSample_delete(instance: *mut c_void); }
		unsafe { cv_GPCPatchSample_delete(self.as_raw_mut_GPCPatchSample()) };
	}
}

impl GPCPatchSample {
	#[inline] pub fn as_raw_GPCPatchSample(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_GPCPatchSample(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for GPCPatchSample {}

impl crate::optflow::GPCPatchSampleTrait for GPCPatchSample {
	#[inline] fn as_raw_GPCPatchSample(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_GPCPatchSample(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GPCPatchSample {
}

/// Class encapsulating training parameters.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPCTrainingParams {
	/// Maximum tree depth to stop partitioning.
	pub max_tree_depth: u32,
	/// Minimum number of samples in the node to stop partitioning.
	pub min_number_of_samples: i32,
	/// Type of descriptors to use.
	pub descriptor_type: i32,
	/// Print progress to stdout.
	pub print_progress: bool,
}

opencv_type_simple! { crate::optflow::GPCTrainingParams }

impl GPCTrainingParams {
	/// ## C++ default parameters
	/// * _max_tree_depth: 20
	/// * _min_number_of_samples: 3
	/// * _descriptor_type: GPC_DESCRIPTOR_DCT
	/// * _print_progress: true
	pub fn new(_max_tree_depth: u32, _min_number_of_samples: i32, _descriptor_type: crate::optflow::GPCDescType, _print_progress: bool) -> Result<crate::optflow::GPCTrainingParams> {
		unsafe { sys::cv_optflow_GPCTrainingParams_GPCTrainingParams_unsigned_int_int_GPCDescType_bool(_max_tree_depth, _min_number_of_samples, _descriptor_type, _print_progress) }.into_result()
	}
	
	pub fn check(self) -> Result<bool> {
		unsafe { sys::cv_optflow_GPCTrainingParams_check_const(self.opencv_to_extern()) }.into_result()
	}
	
}

/// Class encapsulating training samples.
pub trait GPCTrainingSamplesTrait {
	fn as_raw_GPCTrainingSamples(&self) -> *const c_void;
	fn as_raw_mut_GPCTrainingSamples(&mut self) -> *mut c_void;

	fn size(&self) -> Result<size_t> {
		unsafe { sys::cv_optflow_GPCTrainingSamples_size_const(self.as_raw_GPCTrainingSamples()) }.into_result()
	}
	
	fn typ(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_GPCTrainingSamples_type_const(self.as_raw_GPCTrainingSamples()) }.into_result()
	}
	
}

/// Class encapsulating training samples.
pub struct GPCTrainingSamples {
	ptr: *mut c_void
}

opencv_type_boxed! { GPCTrainingSamples }

impl Drop for GPCTrainingSamples {
	fn drop(&mut self) {
		extern "C" { fn cv_GPCTrainingSamples_delete(instance: *mut c_void); }
		unsafe { cv_GPCTrainingSamples_delete(self.as_raw_mut_GPCTrainingSamples()) };
	}
}

impl GPCTrainingSamples {
	#[inline] pub fn as_raw_GPCTrainingSamples(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_GPCTrainingSamples(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for GPCTrainingSamples {}

impl crate::optflow::GPCTrainingSamplesTrait for GPCTrainingSamples {
	#[inline] fn as_raw_GPCTrainingSamples(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_GPCTrainingSamples(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GPCTrainingSamples {
	/// This function can be used to extract samples from a pair of images and a ground truth flow.
	/// Sizes of all the provided vectors must be equal.
	pub fn create(images_from: &core::Vector::<String>, images_to: &core::Vector::<String>, gt: &core::Vector::<String>, descriptor_type: i32) -> Result<core::Ptr::<crate::optflow::GPCTrainingSamples>> {
		unsafe { sys::cv_optflow_GPCTrainingSamples_create_const_vector_String_R_const_vector_String_R_const_vector_String_R_int(images_from.as_raw_VectorOfString(), images_to.as_raw_VectorOfString(), gt.as_raw_VectorOfString(), descriptor_type) }.into_result().map(|r| unsafe { core::Ptr::<crate::optflow::GPCTrainingSamples>::opencv_from_extern(r) } )
	}
	
	pub fn create_1(images_from: &dyn core::ToInputArray, images_to: &dyn core::ToInputArray, gt: &dyn core::ToInputArray, descriptor_type: i32) -> Result<core::Ptr::<crate::optflow::GPCTrainingSamples>> {
		input_array_arg!(images_from);
		input_array_arg!(images_to);
		input_array_arg!(gt);
		unsafe { sys::cv_optflow_GPCTrainingSamples_create_const__InputArrayR_const__InputArrayR_const__InputArrayR_int(images_from.as_raw__InputArray(), images_to.as_raw__InputArray(), gt.as_raw__InputArray(), descriptor_type) }.into_result().map(|r| unsafe { core::Ptr::<crate::optflow::GPCTrainingSamples>::opencv_from_extern(r) } )
	}
	
}

/// Class for individual tree.
pub trait GPCTreeTrait: core::AlgorithmTrait {
	fn as_raw_GPCTree(&self) -> *const c_void;
	fn as_raw_mut_GPCTree(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * params: GPCTrainingParams()
	fn train(&mut self, samples: &mut crate::optflow::GPCTrainingSamples, params: crate::optflow::GPCTrainingParams) -> Result<()> {
		unsafe { sys::cv_optflow_GPCTree_train_GPCTrainingSamplesR_GPCTrainingParams(self.as_raw_mut_GPCTree(), samples.as_raw_mut_GPCTrainingSamples(), params.opencv_to_extern()) }.into_result()
	}
	
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_optflow_GPCTree_write_const_FileStorageR(self.as_raw_GPCTree(), fs.as_raw_mut_FileStorage()) }.into_result()
	}
	
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_optflow_GPCTree_read_const_FileNodeR(self.as_raw_mut_GPCTree(), fn_.as_raw_FileNode()) }.into_result()
	}
	
	fn find_leaf_for_patch(&self, descr: &crate::optflow::GPCPatchDescriptor) -> Result<u32> {
		unsafe { sys::cv_optflow_GPCTree_findLeafForPatch_const_const_GPCPatchDescriptorR(self.as_raw_GPCTree(), descr.as_raw_GPCPatchDescriptor()) }.into_result()
	}
	
	fn get_descriptor_type(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_GPCTree_getDescriptorType_const(self.as_raw_GPCTree()) }.into_result()
	}
	
}

/// Class for individual tree.
pub struct GPCTree {
	ptr: *mut c_void
}

opencv_type_boxed! { GPCTree }

impl Drop for GPCTree {
	fn drop(&mut self) {
		extern "C" { fn cv_GPCTree_delete(instance: *mut c_void); }
		unsafe { cv_GPCTree_delete(self.as_raw_mut_GPCTree()) };
	}
}

impl GPCTree {
	#[inline] pub fn as_raw_GPCTree(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_GPCTree(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for GPCTree {}

impl core::AlgorithmTrait for GPCTree {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::optflow::GPCTreeTrait for GPCTree {
	#[inline] fn as_raw_GPCTree(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_GPCTree(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GPCTree {
	pub fn create() -> Result<core::Ptr::<crate::optflow::GPCTree>> {
		unsafe { sys::cv_optflow_GPCTree_create() }.into_result().map(|r| unsafe { core::Ptr::<crate::optflow::GPCTree>::opencv_from_extern(r) } )
	}
	
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GPCTree_Node {
	/// Hyperplane coefficients
	pub coef: core::Vec18<f64>,
	/// Bias term of the hyperplane
	pub rhs: f64,
	pub left: u32,
	pub right: u32,
}

opencv_type_simple! { crate::optflow::GPCTree_Node }

impl GPCTree_Node {
}

/// PCAFlow algorithm.
pub trait OpticalFlowPCAFlowTrait: crate::video::DenseOpticalFlow {
	fn as_raw_OpticalFlowPCAFlow(&self) -> *const c_void;
	fn as_raw_mut_OpticalFlowPCAFlow(&mut self) -> *mut c_void;

	fn calc(&mut self, i0: &dyn core::ToInputArray, i1: &dyn core::ToInputArray, flow: &mut dyn core::ToInputOutputArray) -> Result<()> {
		input_array_arg!(i0);
		input_array_arg!(i1);
		input_output_array_arg!(flow);
		unsafe { sys::cv_optflow_OpticalFlowPCAFlow_calc_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(self.as_raw_mut_OpticalFlowPCAFlow(), i0.as_raw__InputArray(), i1.as_raw__InputArray(), flow.as_raw__InputOutputArray()) }.into_result()
	}
	
	fn collect_garbage(&mut self) -> Result<()> {
		unsafe { sys::cv_optflow_OpticalFlowPCAFlow_collectGarbage(self.as_raw_mut_OpticalFlowPCAFlow()) }.into_result()
	}
	
}

/// PCAFlow algorithm.
pub struct OpticalFlowPCAFlow {
	ptr: *mut c_void
}

opencv_type_boxed! { OpticalFlowPCAFlow }

impl Drop for OpticalFlowPCAFlow {
	fn drop(&mut self) {
		extern "C" { fn cv_OpticalFlowPCAFlow_delete(instance: *mut c_void); }
		unsafe { cv_OpticalFlowPCAFlow_delete(self.as_raw_mut_OpticalFlowPCAFlow()) };
	}
}

impl OpticalFlowPCAFlow {
	#[inline] pub fn as_raw_OpticalFlowPCAFlow(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_OpticalFlowPCAFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for OpticalFlowPCAFlow {}

impl core::AlgorithmTrait for OpticalFlowPCAFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::video::DenseOpticalFlow for OpticalFlowPCAFlow {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::optflow::OpticalFlowPCAFlowTrait for OpticalFlowPCAFlow {
	#[inline] fn as_raw_OpticalFlowPCAFlow(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_OpticalFlowPCAFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl OpticalFlowPCAFlow {
	/// Creates an instance of PCAFlow algorithm.
	/// ## Parameters
	/// * _prior: Learned prior or no prior (default). see also: cv::optflow::PCAPrior
	/// * _basisSize: Number of basis vectors.
	/// * _sparseRate: Controls density of sparse matches.
	/// * _retainedCornersFraction: Retained corners fraction.
	/// * _occlusionsThreshold: Occlusion threshold.
	/// * _dampingFactor: Regularization term for solving least-squares. It is not related to the prior regularization.
	/// * _claheClip: Clip parameter for CLAHE.
	/// 
	/// ## C++ default parameters
	/// * _prior: Ptr<constPCAPrior>()
	/// * _basis_size: Size(18,14)
	/// * _sparse_rate: 0.024
	/// * _retained_corners_fraction: 0.2
	/// * _occlusions_threshold: 0.0003
	/// * _damping_factor: 0.00002
	/// * _clahe_clip: 14
	pub fn new(_prior: core::Ptr::<crate::optflow::PCAPrior>, _basis_size: core::Size, _sparse_rate: f32, _retained_corners_fraction: f32, _occlusions_threshold: f32, _damping_factor: f32, _clahe_clip: f32) -> Result<crate::optflow::OpticalFlowPCAFlow> {
		unsafe { sys::cv_optflow_OpticalFlowPCAFlow_OpticalFlowPCAFlow_Ptr_PCAPrior__Size_float_float_float_float_float(_prior.as_raw_PtrOfPCAPrior(), _basis_size.opencv_to_extern(), _sparse_rate, _retained_corners_fraction, _occlusions_threshold, _damping_factor, _clahe_clip) }.into_result().map(|r| unsafe { crate::optflow::OpticalFlowPCAFlow::opencv_from_extern(r) } )
	}
	
}

/// @brief
/// This class can be used for imposing a learned prior on the resulting optical flow.
/// Solution will be regularized according to this prior.
/// You need to generate appropriate prior file with "learn_prior.py" script beforehand.
pub trait PCAPriorTrait {
	fn as_raw_PCAPrior(&self) -> *const c_void;
	fn as_raw_mut_PCAPrior(&mut self) -> *mut c_void;

	fn get_padding(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_PCAPrior_getPadding_const(self.as_raw_PCAPrior()) }.into_result()
	}
	
	fn get_basis_size(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_PCAPrior_getBasisSize_const(self.as_raw_PCAPrior()) }.into_result()
	}
	
	fn fill_constraints(&self, a1: &mut f32, a2: &mut f32, b1: &mut f32, b2: &mut f32) -> Result<()> {
		unsafe { sys::cv_optflow_PCAPrior_fillConstraints_const_floatX_floatX_floatX_floatX(self.as_raw_PCAPrior(), a1, a2, b1, b2) }.into_result()
	}
	
}

/// @brief
/// This class can be used for imposing a learned prior on the resulting optical flow.
/// Solution will be regularized according to this prior.
/// You need to generate appropriate prior file with "learn_prior.py" script beforehand.
pub struct PCAPrior {
	ptr: *mut c_void
}

opencv_type_boxed! { PCAPrior }

impl Drop for PCAPrior {
	fn drop(&mut self) {
		extern "C" { fn cv_PCAPrior_delete(instance: *mut c_void); }
		unsafe { cv_PCAPrior_delete(self.as_raw_mut_PCAPrior()) };
	}
}

impl PCAPrior {
	#[inline] pub fn as_raw_PCAPrior(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PCAPrior(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for PCAPrior {}

impl crate::optflow::PCAPriorTrait for PCAPrior {
	#[inline] fn as_raw_PCAPrior(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_PCAPrior(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl PCAPrior {
	pub fn new(path_to_prior: &str) -> Result<crate::optflow::PCAPrior> {
		extern_container_arg!(path_to_prior);
		unsafe { sys::cv_optflow_PCAPrior_PCAPrior_const_charX(path_to_prior.opencv_to_extern()) }.into_result().map(|r| unsafe { crate::optflow::PCAPrior::opencv_from_extern(r) } )
	}
	
}

/// Variational optical flow refinement
/// 
/// This class implements variational refinement of the input flow field, i.e.
/// it uses input flow to initialize the minimization of the following functional:
/// ![inline formula](https://latex.codecogs.com/png.latex?E%28U%29%20%3D%20%5Cint%5F%7B%5COmega%7D%20%5Cdelta%20%5CPsi%28E%5FI%29%20%2B%20%5Cgamma%20%5CPsi%28E%5FG%29%20%2B%20%5Calpha%20%5CPsi%28E%5FS%29%20),
/// where ![inline formula](https://latex.codecogs.com/png.latex?E%5FI%2CE%5FG%2CE%5FS) are color constancy, gradient constancy and smoothness terms
/// respectively. ![inline formula](https://latex.codecogs.com/png.latex?%5CPsi%28s%5E2%29%3D%5Csqrt%7Bs%5E2%2B%5Cepsilon%5E2%7D) is a robust penalizer to limit the
/// influence of outliers. A complete formulation and a description of the minimization
/// procedure can be found in [Brox2004](https://docs.opencv.org/3.4.10/d0/de3/citelist.html#CITEREF_Brox2004)
pub trait VariationalRefinement: crate::video::DenseOpticalFlow {
	fn as_raw_VariationalRefinement(&self) -> *const c_void;
	fn as_raw_mut_VariationalRefinement(&mut self) -> *mut c_void;

	/// @ref calc function overload to handle separate horizontal (u) and vertical (v) flow components
	/// (to avoid extra splits/merges)
	fn calc_uv(&mut self, i0: &dyn core::ToInputArray, i1: &dyn core::ToInputArray, flow_u: &mut dyn core::ToInputOutputArray, flow_v: &mut dyn core::ToInputOutputArray) -> Result<()> {
		input_array_arg!(i0);
		input_array_arg!(i1);
		input_output_array_arg!(flow_u);
		input_output_array_arg!(flow_v);
		unsafe { sys::cv_optflow_VariationalRefinement_calcUV_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_mut_VariationalRefinement(), i0.as_raw__InputArray(), i1.as_raw__InputArray(), flow_u.as_raw__InputOutputArray(), flow_v.as_raw__InputOutputArray()) }.into_result()
	}
	
	/// Number of outer (fixed-point) iterations in the minimization procedure.
	/// ## See also
	/// setFixedPointIterations
	fn get_fixed_point_iterations(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_VariationalRefinement_getFixedPointIterations_const(self.as_raw_VariationalRefinement()) }.into_result()
	}
	
	/// Number of outer (fixed-point) iterations in the minimization procedure.
	/// ## See also
	/// setFixedPointIterations getFixedPointIterations
	fn set_fixed_point_iterations(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_optflow_VariationalRefinement_setFixedPointIterations_int(self.as_raw_mut_VariationalRefinement(), val) }.into_result()
	}
	
	/// Number of inner successive over-relaxation (SOR) iterations
	///    in the minimization procedure to solve the respective linear system.
	/// ## See also
	/// setSorIterations
	fn get_sor_iterations(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_VariationalRefinement_getSorIterations_const(self.as_raw_VariationalRefinement()) }.into_result()
	}
	
	/// Number of inner successive over-relaxation (SOR) iterations
	///    in the minimization procedure to solve the respective linear system.
	/// ## See also
	/// setSorIterations getSorIterations
	fn set_sor_iterations(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_optflow_VariationalRefinement_setSorIterations_int(self.as_raw_mut_VariationalRefinement(), val) }.into_result()
	}
	
	/// Relaxation factor in SOR
	/// ## See also
	/// setOmega
	fn get_omega(&self) -> Result<f32> {
		unsafe { sys::cv_optflow_VariationalRefinement_getOmega_const(self.as_raw_VariationalRefinement()) }.into_result()
	}
	
	/// Relaxation factor in SOR
	/// ## See also
	/// setOmega getOmega
	fn set_omega(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_optflow_VariationalRefinement_setOmega_float(self.as_raw_mut_VariationalRefinement(), val) }.into_result()
	}
	
	/// Weight of the smoothness term
	/// ## See also
	/// setAlpha
	fn get_alpha(&self) -> Result<f32> {
		unsafe { sys::cv_optflow_VariationalRefinement_getAlpha_const(self.as_raw_VariationalRefinement()) }.into_result()
	}
	
	/// Weight of the smoothness term
	/// ## See also
	/// setAlpha getAlpha
	fn set_alpha(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_optflow_VariationalRefinement_setAlpha_float(self.as_raw_mut_VariationalRefinement(), val) }.into_result()
	}
	
	/// Weight of the color constancy term
	/// ## See also
	/// setDelta
	fn get_delta(&self) -> Result<f32> {
		unsafe { sys::cv_optflow_VariationalRefinement_getDelta_const(self.as_raw_VariationalRefinement()) }.into_result()
	}
	
	/// Weight of the color constancy term
	/// ## See also
	/// setDelta getDelta
	fn set_delta(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_optflow_VariationalRefinement_setDelta_float(self.as_raw_mut_VariationalRefinement(), val) }.into_result()
	}
	
	/// Weight of the gradient constancy term
	/// ## See also
	/// setGamma
	fn get_gamma(&self) -> Result<f32> {
		unsafe { sys::cv_optflow_VariationalRefinement_getGamma_const(self.as_raw_VariationalRefinement()) }.into_result()
	}
	
	/// Weight of the gradient constancy term
	/// ## See also
	/// setGamma getGamma
	fn set_gamma(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_optflow_VariationalRefinement_setGamma_float(self.as_raw_mut_VariationalRefinement(), val) }.into_result()
	}
	
}
