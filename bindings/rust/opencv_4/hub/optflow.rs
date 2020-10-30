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
	pub use { super::PCAPriorTrait, super::OpticalFlowPCAFlowTrait, super::GPCPatchDescriptorTrait, super::GPCPatchSampleTrait, super::GPCTrainingSamplesTrait, super::GPCTreeTrait, super::GPCDetailsTrait, super::RLOFOpticalFlowParameterTrait, super::DenseRLOFOpticalFlow, super::SparseRLOFOpticalFlow, super::DualTVL1OpticalFlow };
}

/// Better quality but slow
pub const GPC_DESCRIPTOR_DCT: i32 = 0;
/// Worse quality but much faster
pub const GPC_DESCRIPTOR_WHT: i32 = 1;
/// <  Edge-preserving interpolation using ximgproc::EdgeAwareInterpolator, see [Revaud2015](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Revaud2015),Geistert2016.
pub const INTERP_EPIC: i32 = 1;
/// <  Fast geodesic interpolation, see [Geistert2016](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Geistert2016)
pub const INTERP_GEO: i32 = 0;
/// <  SLIC based robust interpolation using ximgproc::RICInterpolator, see [Hu2017](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Hu2017).
pub const INTERP_RIC: i32 = 2;
/// <  Apply a adaptive support region obtained by cross-based segmentation
/// as described in [Senst2014](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2014)
pub const SR_CROSS: i32 = 1;
/// <  Apply a constant support region
pub const SR_FIXED: i32 = 0;
/// < Apply optimized iterative refinement based bilinear equation solutions
/// as described in [Senst2013](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2013)
pub const ST_BILINEAR: i32 = 1;
/// < Apply standard iterative refinement
pub const ST_STANDART: i32 = 0;
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

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InterpolationType {
	/// <  Fast geodesic interpolation, see [Geistert2016](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Geistert2016)
	INTERP_GEO = 0,
	/// <  Edge-preserving interpolation using ximgproc::EdgeAwareInterpolator, see [Revaud2015](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Revaud2015),Geistert2016.
	INTERP_EPIC = 1,
	/// <  SLIC based robust interpolation using ximgproc::RICInterpolator, see [Hu2017](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Hu2017).
	INTERP_RIC = 2,
}

opencv_type_enum! { crate::optflow::InterpolationType }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SolverType {
	/// < Apply standard iterative refinement
	ST_STANDART = 0,
	/// < Apply optimized iterative refinement based bilinear equation solutions
	/// as described in [Senst2013](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2013)
	ST_BILINEAR = 1,
}

opencv_type_enum! { crate::optflow::SolverType }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SupportRegionType {
	/// <  Apply a constant support region
	SR_FIXED = 0,
	/// <  Apply a adaptive support region obtained by cross-based segmentation
	/// as described in [Senst2014](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2014)
	SR_CROSS = 1,
}

opencv_type_enum! { crate::optflow::SupportRegionType }

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
/// templates technique described in [Davis97](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Davis97) and [Bradski00](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Bradski00) .
pub fn update_motion_history(silhouette: &dyn core::ToInputArray, mhi: &mut dyn core::ToInputOutputArray, timestamp: f64, duration: f64) -> Result<()> {
	input_array_arg!(silhouette);
	input_output_array_arg!(mhi);
	unsafe { sys::cv_motempl_updateMotionHistory_const__InputArrayR_const__InputOutputArrayR_double_double(silhouette.as_raw__InputArray(), mhi.as_raw__InputOutputArray(), timestamp, duration) }.into_result()
}

/// Fast dense optical flow computation based on robust local optical flow (RLOF) algorithms and sparse-to-dense interpolation scheme.
/// 
/// The RLOF is a fast local optical flow approach described in [Senst2012](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2012) [Senst2013](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2013) [Senst2014](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2014)
/// and [Senst2016](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2016) similar to the pyramidal iterative Lucas-Kanade method as
/// proposed by [Bouguet00](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Bouguet00). More details and experiments can be found in the following thesis [Senst2019](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2019).
/// The implementation is derived from optflow::calcOpticalFlowPyrLK().
/// 
/// The sparse-to-dense interpolation scheme allows for fast computation of dense optical flow using RLOF (see [Geistert2016](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Geistert2016)).
/// For this scheme the following steps are applied:
/// -# motion vector seeded at a regular sampled grid are computed. The sparsity of this grid can be configured with setGridStep
/// -# (optinally) errornous motion vectors are filter based on the forward backward confidence. The threshold can be configured
/// with setForwardBackward. The filter is only applied if the threshold >0 but than the runtime is doubled due to the estimation
/// of the backward flow.
/// -# Vector field interpolation is applied to the motion vector set to obtain a dense vector field.
/// 
/// ## Parameters
/// * I0: first 8-bit input image. If The cross-based RLOF is used (by selecting optflow::RLOFOpticalFlowParameter::supportRegionType
/// = SupportRegionType::SR_CROSS) image has to be a 8-bit 3 channel image.
/// * I1: second 8-bit input image. If The cross-based RLOF is used (by selecting optflow::RLOFOpticalFlowParameter::supportRegionType
/// = SupportRegionType::SR_CROSS) image has to be a 8-bit 3 channel image.
/// * flow: computed flow image that has the same size as I0 and type CV_32FC2.
/// * rlofParam: see optflow::RLOFOpticalFlowParameter
/// * forwardBackwardThreshold: Threshold for the forward backward confidence check.
/// For each grid point ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cmathbf%7Bx%7D%20) a motion vector ![inline formula](https://latex.codecogs.com/png.latex?%20d%5F%7BI0%2CI1%7D%28%5Cmathbf%7Bx%7D%29%20) is computed.
/// If the forward backward error ![block formula](https://latex.codecogs.com/png.latex?%20EP%5F%7BFB%7D%20%3D%20%7C%7C%20d%5F%7BI0%2CI1%7D%20%2B%20d%5F%7BI1%2CI0%7D%20%7C%7C%20)
/// is larger than threshold given by this function then the motion vector will not be used by the following
/// vector field interpolation. ![inline formula](https://latex.codecogs.com/png.latex?%20d%5F%7BI1%2CI0%7D%20) denotes the backward flow. Note, the forward backward test
///    will only be applied if the threshold > 0. This may results into a doubled runtime for the motion estimation.
/// * gridStep: Size of the grid to spawn the motion vectors. For each grid point a motion vector is computed.
/// Some motion vectors will be removed due to the forwatd backward threshold (if set >0). The rest will be the
/// base of the vector field interpolation.
/// * interp_type: interpolation method used to compute the dense optical flow. Two interpolation algorithms are
/// supported:
/// - **INTERP_GEO** applies the fast geodesic interpolation, see [Geistert2016](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Geistert2016).
/// - **INTERP_EPIC_RESIDUAL** applies the edge-preserving interpolation, see [Revaud2015](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Revaud2015),Geistert2016.
/// * epicK: see ximgproc::EdgeAwareInterpolator sets the respective parameter.
/// * epicSigma: see ximgproc::EdgeAwareInterpolator sets the respective parameter.
/// * epicLambda: see ximgproc::EdgeAwareInterpolator sets the respective parameter.
/// * ricSPSize: see ximgproc::RICInterpolator sets the respective parameter.
/// * ricSLICType: see ximgproc::RICInterpolator sets the respective parameter.
/// * use_post_proc: enables ximgproc::fastGlobalSmootherFilter() parameter.
/// * fgsLambda: sets the respective ximgproc::fastGlobalSmootherFilter() parameter.
/// * fgsSigma: sets the respective ximgproc::fastGlobalSmootherFilter() parameter.
/// * use_variational_refinement: enables VariationalRefinement
/// 
/// Parameters have been described in [Senst2012](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2012), [Senst2013](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2013), [Senst2014](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2014), [Senst2016](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2016).
/// For the RLOF configuration see optflow::RLOFOpticalFlowParameter for further details.
/// 
/// Note: If the grid size is set to (1,1) and the forward backward threshold <= 0 that the dense optical flow field is purely
/// computed with the RLOF.
/// 
/// 
/// Note: SIMD parallelization is only available when compiling with SSE4.1.
/// ## See also
/// optflow::DenseRLOFOpticalFlow, optflow::RLOFOpticalFlowParameter
/// 
/// ## C++ default parameters
/// * rlof_param: Ptr<RLOFOpticalFlowParameter>()
/// * forward_backward_threshold: 0
/// * grid_step: Size(6,6)
/// * interp_type: InterpolationType::INTERP_EPIC
/// * epic_k: 128
/// * epic_sigma: 0.05f
/// * epic_lambda: 100.f
/// * ric_sp_size: 15
/// * ric_slic_type: 100
/// * use_post_proc: true
/// * fgs_lambda: 500.0f
/// * fgs_sigma: 1.5f
/// * use_variational_refinement: false
pub fn calc_optical_flow_dense_rlof(i0: &dyn core::ToInputArray, i1: &dyn core::ToInputArray, flow: &mut dyn core::ToInputOutputArray, mut rlof_param: core::Ptr::<crate::optflow::RLOFOpticalFlowParameter>, forward_backward_threshold: f32, grid_step: core::Size, interp_type: crate::optflow::InterpolationType, epic_k: i32, epic_sigma: f32, epic_lambda: f32, ric_sp_size: i32, ric_slic_type: i32, use_post_proc: bool, fgs_lambda: f32, fgs_sigma: f32, use_variational_refinement: bool) -> Result<()> {
	input_array_arg!(i0);
	input_array_arg!(i1);
	input_output_array_arg!(flow);
	unsafe { sys::cv_optflow_calcOpticalFlowDenseRLOF_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_Ptr_RLOFOpticalFlowParameter__float_Size_InterpolationType_int_float_float_int_int_bool_float_float_bool(i0.as_raw__InputArray(), i1.as_raw__InputArray(), flow.as_raw__InputOutputArray(), rlof_param.as_raw_mut_PtrOfRLOFOpticalFlowParameter(), forward_backward_threshold, grid_step.opencv_as_extern(), interp_type, epic_k, epic_sigma, epic_lambda, ric_sp_size, ric_slic_type, use_post_proc, fgs_lambda, fgs_sigma, use_variational_refinement) }.into_result()
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
/// See [Tao2012](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Tao2012) . And site of project - <http://graphics.berkeley.edu/papers/Tao-SAN-2012-05/>.
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
/// See [Tao2012](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Tao2012) . And site of project - <http://graphics.berkeley.edu/papers/Tao-SAN-2012-05/>.
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

/// Calculates fast optical flow for a sparse feature set using the robust local optical flow (RLOF) similar
/// to optflow::calcOpticalFlowPyrLK().
/// 
/// The RLOF is a fast local optical flow approach described in [Senst2012](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2012) [Senst2013](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2013) [Senst2014](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2014)
/// and [Senst2016](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2016) similar to the pyramidal iterative Lucas-Kanade method as
/// proposed by [Bouguet00](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Bouguet00). More details and experiments can be found in the following thesis [Senst2019](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2019).
/// The implementation is derived from optflow::calcOpticalFlowPyrLK().
/// 
/// ## Parameters
/// * prevImg: first 8-bit input image. If The cross-based RLOF is used (by selecting optflow::RLOFOpticalFlowParameter::supportRegionType
/// = SupportRegionType::SR_CROSS) image has to be a 8-bit 3 channel image.
/// * nextImg: second 8-bit input image. If The cross-based RLOF is used (by selecting optflow::RLOFOpticalFlowParameter::supportRegionType
/// = SupportRegionType::SR_CROSS) image has to be a 8-bit 3 channel image.
/// * prevPts: vector of 2D points for which the flow needs to be found; point coordinates must be single-precision
/// floating-point numbers.
/// * nextPts: output vector of 2D points (with single-precision floating-point coordinates) containing the calculated
/// new positions of input features in the second image; when optflow::RLOFOpticalFlowParameter::useInitialFlow variable is true  the vector must
/// have the same size as in the input and contain the initialization point correspondences.
/// * status: output status vector (of unsigned chars); each element of the vector is set to 1 if the flow for the
/// corresponding features has passed the forward backward check.
/// * err: output vector of errors; each element of the vector is set to the forward backward error for the corresponding feature.
/// * rlofParam: see optflow::RLOFOpticalFlowParameter
/// * forwardBackwardThreshold: Threshold for the forward backward confidence check. If forewardBackwardThreshold <=0 the forward
/// 
/// 
/// Note: SIMD parallelization is only available when compiling with SSE4.1.
/// 
/// Parameters have been described in [Senst2012](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2012), [Senst2013](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2013), [Senst2014](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2014) and [Senst2016](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2016).
/// For the RLOF configuration see optflow::RLOFOpticalFlowParameter for further details.
/// 
/// ## C++ default parameters
/// * rlof_param: Ptr<RLOFOpticalFlowParameter>()
/// * forward_backward_threshold: 0
pub fn calc_optical_flow_sparse_rlof(prev_img: &dyn core::ToInputArray, next_img: &dyn core::ToInputArray, prev_pts: &dyn core::ToInputArray, next_pts: &mut dyn core::ToInputOutputArray, status: &mut dyn core::ToOutputArray, err: &mut dyn core::ToOutputArray, mut rlof_param: core::Ptr::<crate::optflow::RLOFOpticalFlowParameter>, forward_backward_threshold: f32) -> Result<()> {
	input_array_arg!(prev_img);
	input_array_arg!(next_img);
	input_array_arg!(prev_pts);
	input_output_array_arg!(next_pts);
	output_array_arg!(status);
	output_array_arg!(err);
	unsafe { sys::cv_optflow_calcOpticalFlowSparseRLOF_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_Ptr_RLOFOpticalFlowParameter__float(prev_img.as_raw__InputArray(), next_img.as_raw__InputArray(), prev_pts.as_raw__InputArray(), next_pts.as_raw__InputOutputArray(), status.as_raw__OutputArray(), err.as_raw__OutputArray(), rlof_param.as_raw_mut_PtrOfRLOFOpticalFlowParameter(), forward_backward_threshold) }.into_result()
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

/// DeepFlow optical flow algorithm implementation.
/// 
/// The class implements the DeepFlow optical flow algorithm described in [Weinzaepfel2013](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Weinzaepfel2013) . See
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

/// Additional interface to the Dense RLOF algorithm - optflow::calcOpticalFlowDenseRLOF()
pub fn create_opt_flow_dense_rlof() -> Result<core::Ptr::<dyn crate::video::DenseOpticalFlow>> {
	unsafe { sys::cv_optflow_createOptFlow_DenseRLOF() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::video::DenseOpticalFlow>::opencv_from_extern(r) } )
}

/// Creates instance of cv::DenseOpticalFlow
pub fn create_opt_flow_dual_tvl1() -> Result<core::Ptr::<dyn crate::optflow::DualTVL1OpticalFlow>> {
	unsafe { sys::cv_optflow_createOptFlow_DualTVL1() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::optflow::DualTVL1OpticalFlow>::opencv_from_extern(r) } )
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

/// Additional interface to the Sparse RLOF algorithm - optflow::calcOpticalFlowSparseRLOF()
pub fn create_opt_flow_sparse_rlof() -> Result<core::Ptr::<dyn crate::video::SparseOpticalFlow>> {
	unsafe { sys::cv_optflow_createOptFlow_SparseRLOF() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::video::SparseOpticalFlow>::opencv_from_extern(r) } )
}

/// Additional interface to the SparseToDenseFlow algorithm - calcOpticalFlowSparseToDense()
pub fn create_opt_flow_sparse_to_dense() -> Result<core::Ptr::<dyn crate::video::DenseOpticalFlow>> {
	unsafe { sys::cv_optflow_createOptFlow_SparseToDense() }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::video::DenseOpticalFlow>::opencv_from_extern(r) } )
}

pub fn read(fn_: &core::FileNode, node: &mut crate::optflow::GPCTree_Node, unnamed: crate::optflow::GPCTree_Node) -> Result<()> {
	unsafe { sys::cv_read_const_FileNodeR_NodeR_Node(fn_.as_raw_FileNode(), node, unnamed.opencv_as_extern()) }.into_result()
}

pub fn write(fs: &mut core::FileStorage, name: &str, node: crate::optflow::GPCTree_Node) -> Result<()> {
	extern_container_arg!(name);
	unsafe { sys::cv_write_FileStorageR_const_StringR_const_NodeR(fs.as_raw_mut_FileStorage(), name.opencv_as_extern(), &node) }.into_result()
}

/// Fast dense optical flow computation based on robust local optical flow (RLOF) algorithms and sparse-to-dense interpolation
/// scheme.
/// 
/// The RLOF is a fast local optical flow approach described in [Senst2012](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2012) [Senst2013](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2013) [Senst2014](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2014)
/// and [Senst2016](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2016) similar to the pyramidal iterative Lucas-Kanade method as
/// proposed by [Bouguet00](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Bouguet00). More details and experiments can be found in the following thesis [Senst2019](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2019).
/// The implementation is derived from optflow::calcOpticalFlowPyrLK().
/// 
/// The sparse-to-dense interpolation scheme allows for fast computation of dense optical flow using RLOF (see [Geistert2016](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Geistert2016)).
/// For this scheme the following steps are applied:
/// -# motion vector seeded at a regular sampled grid are computed. The sparsity of this grid can be configured with setGridStep
/// -# (optinally) errornous motion vectors are filter based on the forward backward confidence. The threshold can be configured
/// with setForwardBackward. The filter is only applied if the threshold >0 but than the runtime is doubled due to the estimation
/// of the backward flow.
/// -# Vector field interpolation is applied to the motion vector set to obtain a dense vector field.
/// 
/// For the RLOF configuration see optflow::RLOFOpticalFlowParameter for further details.
/// Parameters have been described in [Senst2012](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2012) [Senst2013](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2013) [Senst2014](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2014) and [Senst2016](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2016).
/// 
/// 
/// Note: If the grid size is set to (1,1) and the forward backward threshold <= 0 than pixelwise dense optical flow field is
/// computed by RLOF without using interpolation.
/// ## See also
/// optflow::calcOpticalFlowDenseRLOF(), optflow::RLOFOpticalFlowParameter
pub trait DenseRLOFOpticalFlow: crate::video::DenseOpticalFlow {
	fn as_raw_DenseRLOFOpticalFlow(&self) -> *const c_void;
	fn as_raw_mut_DenseRLOFOpticalFlow(&mut self) -> *mut c_void;

	/// Configuration of the RLOF alogrithm.
	/// ## See also
	/// optflow::RLOFOpticalFlowParameter, getRLOFOpticalFlowParameter
	fn set_rlof_optical_flow_parameter(&mut self, mut val: core::Ptr::<crate::optflow::RLOFOpticalFlowParameter>) -> Result<()> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setRLOFOpticalFlowParameter_Ptr_RLOFOpticalFlowParameter_(self.as_raw_mut_DenseRLOFOpticalFlow(), val.as_raw_mut_PtrOfRLOFOpticalFlowParameter()) }.into_result()
	}
	
	/// Configuration of the RLOF alogrithm.
	/// ## See also
	/// optflow::RLOFOpticalFlowParameter, getRLOFOpticalFlowParameter
	///    optflow::RLOFOpticalFlowParameter, setRLOFOpticalFlowParameter
	fn get_rlof_optical_flow_parameter(&self) -> Result<core::Ptr::<crate::optflow::RLOFOpticalFlowParameter>> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getRLOFOpticalFlowParameter_const(self.as_raw_DenseRLOFOpticalFlow()) }.into_result().map(|r| unsafe { core::Ptr::<crate::optflow::RLOFOpticalFlowParameter>::opencv_from_extern(r) } )
	}
	
	/// Threshold for the forward backward confidence check
	/// For each grid point ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cmathbf%7Bx%7D%20) a motion vector ![inline formula](https://latex.codecogs.com/png.latex?%20d%5F%7BI0%2CI1%7D%28%5Cmathbf%7Bx%7D%29%20) is computed.
	///      *     If the forward backward error ![block formula](https://latex.codecogs.com/png.latex?%20EP%5F%7BFB%7D%20%3D%20%7C%7C%20d%5F%7BI0%2CI1%7D%20%2B%20d%5F%7BI1%2CI0%7D%20%7C%7C%20)
	///      *     is larger than threshold given by this function then the motion vector will not be used by the following
	///      *    vector field interpolation. ![inline formula](https://latex.codecogs.com/png.latex?%20d%5F%7BI1%2CI0%7D%20) denotes the backward flow. Note, the forward backward test
	///      *    will only be applied if the threshold > 0. This may results into a doubled runtime for the motion estimation.
	///      *    see also: getForwardBackward, setGridStep
	fn set_forward_backward(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setForwardBackward_float(self.as_raw_mut_DenseRLOFOpticalFlow(), val) }.into_result()
	}
	
	/// Threshold for the forward backward confidence check
	/// For each grid point ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cmathbf%7Bx%7D%20) a motion vector ![inline formula](https://latex.codecogs.com/png.latex?%20d%5F%7BI0%2CI1%7D%28%5Cmathbf%7Bx%7D%29%20) is computed.
	///      *     If the forward backward error ![block formula](https://latex.codecogs.com/png.latex?%20EP%5F%7BFB%7D%20%3D%20%7C%7C%20d%5F%7BI0%2CI1%7D%20%2B%20d%5F%7BI1%2CI0%7D%20%7C%7C%20)
	///      *     is larger than threshold given by this function then the motion vector will not be used by the following
	///      *    vector field interpolation. ![inline formula](https://latex.codecogs.com/png.latex?%20d%5F%7BI1%2CI0%7D%20) denotes the backward flow. Note, the forward backward test
	///      *    will only be applied if the threshold > 0. This may results into a doubled runtime for the motion estimation.
	///      *    getForwardBackward, setGridStep
	/// ## See also
	/// setForwardBackward
	fn get_forward_backward(&self) -> Result<f32> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getForwardBackward_const(self.as_raw_DenseRLOFOpticalFlow()) }.into_result()
	}
	
	/// Size of the grid to spawn the motion vectors.
	/// For each grid point a motion vector is computed. Some motion vectors will be removed due to the forwatd backward
	///      *  threshold (if set >0). The rest will be the base of the vector field interpolation.
	///      *    see also: getForwardBackward, setGridStep
	fn get_grid_step(&self) -> Result<core::Size> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getGridStep_const(self.as_raw_DenseRLOFOpticalFlow()) }.into_result()
	}
	
	/// Size of the grid to spawn the motion vectors.
	/// For each grid point a motion vector is computed. Some motion vectors will be removed due to the forwatd backward
	///      *  threshold (if set >0). The rest will be the base of the vector field interpolation.
	///      *    see also: getForwardBackward, setGridStep
	///      *    see also: getGridStep
	fn set_grid_step(&mut self, val: core::Size) -> Result<()> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setGridStep_Size(self.as_raw_mut_DenseRLOFOpticalFlow(), val.opencv_as_extern()) }.into_result()
	}
	
	/// Interpolation used to compute the dense optical flow.
	/// Two interpolation algorithms are supported
	///      * - **INTERP_GEO** applies the fast geodesic interpolation, see [Geistert2016](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Geistert2016).
	///      * - **INTERP_EPIC_RESIDUAL** applies the edge-preserving interpolation, see [Revaud2015](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Revaud2015),Geistert2016.
	///      * see also: ximgproc::EdgeAwareInterpolator, getInterpolation
	fn set_interpolation(&mut self, val: crate::optflow::InterpolationType) -> Result<()> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setInterpolation_InterpolationType(self.as_raw_mut_DenseRLOFOpticalFlow(), val) }.into_result()
	}
	
	/// Interpolation used to compute the dense optical flow.
	/// Two interpolation algorithms are supported
	///      * - **INTERP_GEO** applies the fast geodesic interpolation, see [Geistert2016](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Geistert2016).
	///      * - **INTERP_EPIC_RESIDUAL** applies the edge-preserving interpolation, see [Revaud2015](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Revaud2015),Geistert2016.
	///      * see also: ximgproc::EdgeAwareInterpolator, getInterpolation
	///      *    see also: ximgproc::EdgeAwareInterpolator, setInterpolation
	fn get_interpolation(&self) -> Result<crate::optflow::InterpolationType> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getInterpolation_const(self.as_raw_DenseRLOFOpticalFlow()) }.into_result()
	}
	
	/// see ximgproc::EdgeAwareInterpolator() K value.
	/// K is a number of nearest-neighbor matches considered, when fitting a locally affine
	///      *    model. Usually it should be around 128. However, lower values would make the interpolation noticeably faster.
	///      *    see also: ximgproc::EdgeAwareInterpolator,  setEPICK
	fn get_epick(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getEPICK_const(self.as_raw_DenseRLOFOpticalFlow()) }.into_result()
	}
	
	/// see ximgproc::EdgeAwareInterpolator() K value.
	/// K is a number of nearest-neighbor matches considered, when fitting a locally affine
	///      *    model. Usually it should be around 128. However, lower values would make the interpolation noticeably faster.
	///      *    see also: ximgproc::EdgeAwareInterpolator,  setEPICK
	///      *    see also: ximgproc::EdgeAwareInterpolator, getEPICK
	fn set_epick(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setEPICK_int(self.as_raw_mut_DenseRLOFOpticalFlow(), val) }.into_result()
	}
	
	/// see ximgproc::EdgeAwareInterpolator() sigma value.
	/// Sigma is a parameter defining how fast the weights decrease in the locally-weighted affine
	///      *  fitting. Higher values can help preserve fine details, lower values can help to get rid of noise in the
	///      *  output flow.
	///      *    see also: ximgproc::EdgeAwareInterpolator, setEPICSigma
	fn get_epic_sigma(&self) -> Result<f32> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getEPICSigma_const(self.as_raw_DenseRLOFOpticalFlow()) }.into_result()
	}
	
	/// see ximgproc::EdgeAwareInterpolator() sigma value.
	/// Sigma is a parameter defining how fast the weights decrease in the locally-weighted affine
	///      *  fitting. Higher values can help preserve fine details, lower values can help to get rid of noise in the
	///      *  output flow.
	///      *    see also: ximgproc::EdgeAwareInterpolator, setEPICSigma
	///      *  see also: ximgproc::EdgeAwareInterpolator, getEPICSigma
	fn set_epic_sigma(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setEPICSigma_float(self.as_raw_mut_DenseRLOFOpticalFlow(), val) }.into_result()
	}
	
	///  see ximgproc::EdgeAwareInterpolator() lambda value.
	/// Lambda is a parameter defining the weight of the edge-aware term in geodesic distance,
	///      *    should be in the range of 0 to 1000.
	///      *    see also: ximgproc::EdgeAwareInterpolator, setEPICSigma
	fn get_epic_lambda(&self) -> Result<f32> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getEPICLambda_const(self.as_raw_DenseRLOFOpticalFlow()) }.into_result()
	}
	
	///  see ximgproc::EdgeAwareInterpolator() lambda value.
	/// Lambda is a parameter defining the weight of the edge-aware term in geodesic distance,
	///      *    should be in the range of 0 to 1000.
	///      *    see also: ximgproc::EdgeAwareInterpolator, setEPICSigma
	///      *    see also: ximgproc::EdgeAwareInterpolator, getEPICLambda
	fn set_epic_lambda(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setEPICLambda_float(self.as_raw_mut_DenseRLOFOpticalFlow(), val) }.into_result()
	}
	
	/// see ximgproc::EdgeAwareInterpolator().
	/// Sets the respective fastGlobalSmootherFilter() parameter.
	///      *    see also: ximgproc::EdgeAwareInterpolator, setFgsLambda
	fn get_fgs_lambda(&self) -> Result<f32> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getFgsLambda_const(self.as_raw_DenseRLOFOpticalFlow()) }.into_result()
	}
	
	/// see ximgproc::EdgeAwareInterpolator().
	/// Sets the respective fastGlobalSmootherFilter() parameter.
	///      *    see also: ximgproc::EdgeAwareInterpolator, setFgsLambda
	///      *    see also: ximgproc::EdgeAwareInterpolator, ximgproc::fastGlobalSmootherFilter, getFgsLambda
	fn set_fgs_lambda(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setFgsLambda_float(self.as_raw_mut_DenseRLOFOpticalFlow(), val) }.into_result()
	}
	
	/// see ximgproc::EdgeAwareInterpolator().
	/// Sets the respective fastGlobalSmootherFilter() parameter.
	///      *    see also: ximgproc::EdgeAwareInterpolator, ximgproc::fastGlobalSmootherFilter, setFgsSigma
	fn get_fgs_sigma(&self) -> Result<f32> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getFgsSigma_const(self.as_raw_DenseRLOFOpticalFlow()) }.into_result()
	}
	
	/// see ximgproc::EdgeAwareInterpolator().
	/// Sets the respective fastGlobalSmootherFilter() parameter.
	///      *    see also: ximgproc::EdgeAwareInterpolator, ximgproc::fastGlobalSmootherFilter, setFgsSigma
	///      *    see also: ximgproc::EdgeAwareInterpolator, ximgproc::fastGlobalSmootherFilter, getFgsSigma
	fn set_fgs_sigma(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setFgsSigma_float(self.as_raw_mut_DenseRLOFOpticalFlow(), val) }.into_result()
	}
	
	/// enables ximgproc::fastGlobalSmootherFilter
	/// 
	/// * see also: getUsePostProc
	fn set_use_post_proc(&mut self, val: bool) -> Result<()> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setUsePostProc_bool(self.as_raw_mut_DenseRLOFOpticalFlow(), val) }.into_result()
	}
	
	/// enables ximgproc::fastGlobalSmootherFilter
	/// 
	/// * see also: getUsePostProc
	///      *    see also: ximgproc::fastGlobalSmootherFilter, setUsePostProc
	fn get_use_post_proc(&self) -> Result<bool> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getUsePostProc_const(self.as_raw_DenseRLOFOpticalFlow()) }.into_result()
	}
	
	/// enables VariationalRefinement
	/// 
	/// * see also: getUseVariationalRefinement
	fn set_use_variational_refinement(&mut self, val: bool) -> Result<()> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setUseVariationalRefinement_bool(self.as_raw_mut_DenseRLOFOpticalFlow(), val) }.into_result()
	}
	
	/// enables VariationalRefinement
	/// 
	/// * see also: getUseVariationalRefinement
	///      *    see also: ximgproc::fastGlobalSmootherFilter, setUsePostProc
	fn get_use_variational_refinement(&self) -> Result<bool> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getUseVariationalRefinement_const(self.as_raw_DenseRLOFOpticalFlow()) }.into_result()
	}
	
	/// Parameter to tune the approximate size of the superpixel used for oversegmentation.
	/// 
	/// * see also: cv::ximgproc::createSuperpixelSLIC, cv::ximgproc::RICInterpolator
	fn set_ricsp_size(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setRICSPSize_int(self.as_raw_mut_DenseRLOFOpticalFlow(), val) }.into_result()
	}
	
	/// Parameter to tune the approximate size of the superpixel used for oversegmentation.
	/// 
	/// * see also: cv::ximgproc::createSuperpixelSLIC, cv::ximgproc::RICInterpolator
	///    *    see also: setRICSPSize
	fn get_ricsp_size(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getRICSPSize_const(self.as_raw_DenseRLOFOpticalFlow()) }.into_result()
	}
	
	/// Parameter to choose superpixel algorithm variant to use:
	/// - cv::ximgproc::SLICType SLIC segments image using a desired region_size (value: 100)
	/// - cv::ximgproc::SLICType SLICO will optimize using adaptive compactness factor (value: 101)
	/// - cv::ximgproc::SLICType MSLIC will optimize using manifold methods resulting in more content-sensitive superpixels (value: 102).
	/// ## See also
	/// cv::ximgproc::createSuperpixelSLIC, cv::ximgproc::RICInterpolator
	fn set_ricslic_type(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_setRICSLICType_int(self.as_raw_mut_DenseRLOFOpticalFlow(), val) }.into_result()
	}
	
	/// Parameter to choose superpixel algorithm variant to use:
	/// - cv::ximgproc::SLICType SLIC segments image using a desired region_size (value: 100)
	/// - cv::ximgproc::SLICType SLICO will optimize using adaptive compactness factor (value: 101)
	/// - cv::ximgproc::SLICType MSLIC will optimize using manifold methods resulting in more content-sensitive superpixels (value: 102).
	/// ## See also
	/// cv::ximgproc::createSuperpixelSLIC, cv::ximgproc::RICInterpolator
	///      *    setRICSLICType
	fn get_ricslic_type(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_getRICSLICType_const(self.as_raw_DenseRLOFOpticalFlow()) }.into_result()
	}
	
}

impl dyn DenseRLOFOpticalFlow + '_ {
	/// Creates instance of optflow::DenseRLOFOpticalFlow
	/// 
	/// ## Parameters
	/// * rlofParam: see optflow::RLOFOpticalFlowParameter
	/// * forwardBackwardThreshold: see setForwardBackward
	/// * gridStep: see setGridStep
	/// * interp_type: see setInterpolation
	/// * epicK: see setEPICK
	/// * epicSigma: see setEPICSigma
	/// * epicLambda: see setEPICLambda
	/// * ricSPSize: see setRICSPSize
	/// * ricSLICType: see setRICSLICType
	/// * use_post_proc: see setUsePostProc
	/// * fgsLambda: see setFgsLambda
	/// * fgsSigma: see setFgsSigma
	/// * use_variational_refinement: see setUseVariationalRefinement
	/// 
	/// ## C++ default parameters
	/// * rlof_param: Ptr<RLOFOpticalFlowParameter>()
	/// * forward_backward_threshold: 1.f
	/// * grid_step: Size(6,6)
	/// * interp_type: InterpolationType::INTERP_EPIC
	/// * epic_k: 128
	/// * epic_sigma: 0.05f
	/// * epic_lambda: 999.0f
	/// * ric_sp_size: 15
	/// * ric_slic_type: 100
	/// * use_post_proc: true
	/// * fgs_lambda: 500.0f
	/// * fgs_sigma: 1.5f
	/// * use_variational_refinement: false
	pub fn create(mut rlof_param: core::Ptr::<crate::optflow::RLOFOpticalFlowParameter>, forward_backward_threshold: f32, grid_step: core::Size, interp_type: crate::optflow::InterpolationType, epic_k: i32, epic_sigma: f32, epic_lambda: f32, ric_sp_size: i32, ric_slic_type: i32, use_post_proc: bool, fgs_lambda: f32, fgs_sigma: f32, use_variational_refinement: bool) -> Result<core::Ptr::<dyn crate::optflow::DenseRLOFOpticalFlow>> {
		unsafe { sys::cv_optflow_DenseRLOFOpticalFlow_create_Ptr_RLOFOpticalFlowParameter__float_Size_InterpolationType_int_float_float_int_int_bool_float_float_bool(rlof_param.as_raw_mut_PtrOfRLOFOpticalFlowParameter(), forward_backward_threshold, grid_step.opencv_as_extern(), interp_type, epic_k, epic_sigma, epic_lambda, ric_sp_size, ric_slic_type, use_post_proc, fgs_lambda, fgs_sigma, use_variational_refinement) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::optflow::DenseRLOFOpticalFlow>::opencv_from_extern(r) } )
	}
	
}
/// "Dual TV L1" Optical Flow Algorithm.
/// 
/// The class implements the "Dual TV L1" optical flow algorithm described in [Zach2007](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Zach2007) and
/// [Javier2012](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Javier2012) .
/// Here are important members of the class that control the algorithm, which you can set after
/// constructing the class instance:
/// 
/// *   member double tau
///    Time step of the numerical scheme.
/// 
/// *   member double lambda
///    Weight parameter for the data term, attachment parameter. This is the most relevant
///    parameter, which determines the smoothness of the output. The smaller this parameter is,
///    the smoother the solutions we obtain. It depends on the range of motions of the images, so
///    its value should be adapted to each image sequence.
/// 
/// *   member double theta
///    Weight parameter for (u - v)\^2, tightness parameter. It serves as a link between the
///    attachment and the regularization terms. In theory, it should have a small value in order
///    to maintain both parts in correspondence. The method is stable for a large range of values
///    of this parameter.
/// 
/// *   member int nscales
///    Number of scales used to create the pyramid of images.
/// 
/// *   member int warps
///    Number of warpings per scale. Represents the number of times that I1(x+u0) and grad(
///    I1(x+u0) ) are computed per scale. This is a parameter that assures the stability of the
///    method. It also affects the running time, so it is a compromise between speed and
///    accuracy.
/// 
/// *   member double epsilon
///    Stopping criterion threshold used in the numerical scheme, which is a trade-off between
///    precision and running time. A small value will yield more accurate solutions at the
///    expense of a slower convergence.
/// 
/// *   member int iterations
///    Stopping criterion iterations number used in the numerical scheme.
/// 
/// C. Zach, T. Pock and H. Bischof, "A Duality Based Approach for Realtime TV-L1 Optical Flow".
/// Javier Sanchez, Enric Meinhardt-Llopis and Gabriele Facciolo. "TV-L1 Optical Flow Estimation".
pub trait DualTVL1OpticalFlow: crate::video::DenseOpticalFlow {
	fn as_raw_DualTVL1OpticalFlow(&self) -> *const c_void;
	fn as_raw_mut_DualTVL1OpticalFlow(&mut self) -> *mut c_void;

	/// Time step of the numerical scheme
	/// ## See also
	/// setTau
	fn get_tau(&self) -> Result<f64> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getTau_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// Time step of the numerical scheme
	/// ## See also
	/// setTau getTau
	fn set_tau(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setTau_double(self.as_raw_mut_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
	/// Weight parameter for the data term, attachment parameter
	/// ## See also
	/// setLambda
	fn get_lambda(&self) -> Result<f64> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getLambda_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// Weight parameter for the data term, attachment parameter
	/// ## See also
	/// setLambda getLambda
	fn set_lambda(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setLambda_double(self.as_raw_mut_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
	/// Weight parameter for (u - v)^2, tightness parameter
	/// ## See also
	/// setTheta
	fn get_theta(&self) -> Result<f64> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getTheta_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// Weight parameter for (u - v)^2, tightness parameter
	/// ## See also
	/// setTheta getTheta
	fn set_theta(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setTheta_double(self.as_raw_mut_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
	/// coefficient for additional illumination variation term
	/// ## See also
	/// setGamma
	fn get_gamma(&self) -> Result<f64> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getGamma_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// coefficient for additional illumination variation term
	/// ## See also
	/// setGamma getGamma
	fn set_gamma(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setGamma_double(self.as_raw_mut_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
	/// Number of scales used to create the pyramid of images
	/// ## See also
	/// setScalesNumber
	fn get_scales_number(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getScalesNumber_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// Number of scales used to create the pyramid of images
	/// ## See also
	/// setScalesNumber getScalesNumber
	fn set_scales_number(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setScalesNumber_int(self.as_raw_mut_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
	/// Number of warpings per scale
	/// ## See also
	/// setWarpingsNumber
	fn get_warpings_number(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getWarpingsNumber_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// Number of warpings per scale
	/// ## See also
	/// setWarpingsNumber getWarpingsNumber
	fn set_warpings_number(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setWarpingsNumber_int(self.as_raw_mut_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
	/// Stopping criterion threshold used in the numerical scheme, which is a trade-off between precision and running time
	/// ## See also
	/// setEpsilon
	fn get_epsilon(&self) -> Result<f64> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getEpsilon_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// Stopping criterion threshold used in the numerical scheme, which is a trade-off between precision and running time
	/// ## See also
	/// setEpsilon getEpsilon
	fn set_epsilon(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setEpsilon_double(self.as_raw_mut_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
	/// Inner iterations (between outlier filtering) used in the numerical scheme
	/// ## See also
	/// setInnerIterations
	fn get_inner_iterations(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getInnerIterations_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// Inner iterations (between outlier filtering) used in the numerical scheme
	/// ## See also
	/// setInnerIterations getInnerIterations
	fn set_inner_iterations(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setInnerIterations_int(self.as_raw_mut_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
	/// Outer iterations (number of inner loops) used in the numerical scheme
	/// ## See also
	/// setOuterIterations
	fn get_outer_iterations(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getOuterIterations_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// Outer iterations (number of inner loops) used in the numerical scheme
	/// ## See also
	/// setOuterIterations getOuterIterations
	fn set_outer_iterations(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setOuterIterations_int(self.as_raw_mut_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
	/// Use initial flow
	/// ## See also
	/// setUseInitialFlow
	fn get_use_initial_flow(&self) -> Result<bool> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getUseInitialFlow_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// Use initial flow
	/// ## See also
	/// setUseInitialFlow getUseInitialFlow
	fn set_use_initial_flow(&mut self, val: bool) -> Result<()> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setUseInitialFlow_bool(self.as_raw_mut_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
	/// Step between scales (<1)
	/// ## See also
	/// setScaleStep
	fn get_scale_step(&self) -> Result<f64> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getScaleStep_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// Step between scales (<1)
	/// ## See also
	/// setScaleStep getScaleStep
	fn set_scale_step(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setScaleStep_double(self.as_raw_mut_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
	/// Median filter kernel size (1 = no filter) (3 or 5)
	/// ## See also
	/// setMedianFiltering
	fn get_median_filtering(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_getMedianFiltering_const(self.as_raw_DualTVL1OpticalFlow()) }.into_result()
	}
	
	/// Median filter kernel size (1 = no filter) (3 or 5)
	/// ## See also
	/// setMedianFiltering getMedianFiltering
	fn set_median_filtering(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_setMedianFiltering_int(self.as_raw_mut_DualTVL1OpticalFlow(), val) }.into_result()
	}
	
}

impl dyn DualTVL1OpticalFlow + '_ {
	/// Creates instance of cv::DualTVL1OpticalFlow
	/// 
	/// ## C++ default parameters
	/// * tau: 0.25
	/// * lambda: 0.15
	/// * theta: 0.3
	/// * nscales: 5
	/// * warps: 5
	/// * epsilon: 0.01
	/// * innner_iterations: 30
	/// * outer_iterations: 10
	/// * scale_step: 0.8
	/// * gamma: 0.0
	/// * median_filtering: 5
	/// * use_initial_flow: false
	pub fn create(tau: f64, lambda: f64, theta: f64, nscales: i32, warps: i32, epsilon: f64, innner_iterations: i32, outer_iterations: i32, scale_step: f64, gamma: f64, median_filtering: i32, use_initial_flow: bool) -> Result<core::Ptr::<dyn crate::optflow::DualTVL1OpticalFlow>> {
		unsafe { sys::cv_optflow_DualTVL1OpticalFlow_create_double_double_double_int_int_double_int_int_double_double_int_bool(tau, lambda, theta, nscales, warps, epsilon, innner_iterations, outer_iterations, scale_step, gamma, median_filtering, use_initial_flow) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::optflow::DualTVL1OpticalFlow>::opencv_from_extern(r) } )
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
		unsafe { sys::cv_optflow_GPCDetails_getCoordinatesFromIndex_size_t_Size_intR_intR(index, sz.opencv_as_extern(), x, y) }.into_result()
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
		unsafe { sys::cv_optflow_GPCPatchDescriptor_setPropFeature_Vec_double__18_(self.as_raw_mut_GPCPatchDescriptor(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_feature")
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
		unsafe { sys::cv_optflow_GPCTrainingParams_check_const(self.opencv_as_extern()) }.into_result()
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
		unsafe { sys::cv_optflow_GPCTree_train_GPCTrainingSamplesR_const_GPCTrainingParams(self.as_raw_mut_GPCTree(), samples.as_raw_mut_GPCTrainingSamples(), params.opencv_as_extern()) }.into_result()
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
		unsafe { sys::cv_optflow_OpticalFlowPCAFlow_OpticalFlowPCAFlow_Ptr_const_PCAPrior__const_Size_float_float_float_float_float(_prior.as_raw_PtrOfPCAPrior(), _basis_size.opencv_as_extern(), _sparse_rate, _retained_corners_fraction, _occlusions_threshold, _damping_factor, _clahe_clip) }.into_result().map(|r| unsafe { crate::optflow::OpticalFlowPCAFlow::opencv_from_extern(r) } )
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
		unsafe { sys::cv_optflow_PCAPrior_PCAPrior_const_charX(path_to_prior.opencv_as_extern()) }.into_result().map(|r| unsafe { crate::optflow::PCAPrior::opencv_from_extern(r) } )
	}
	
}

/// This is used store and set up the parameters of the robust local optical flow (RLOF) algoritm.
/// 
/// The RLOF is a fast local optical flow approach described in [Senst2012](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2012) [Senst2013](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2013) [Senst2014](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2014)
/// and [Senst2016](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2016) similar to the pyramidal iterative Lucas-Kanade method as
/// proposed by [Bouguet00](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Bouguet00). More details and experiments can be found in the following thesis [Senst2019](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2019).
/// The implementation is derived from optflow::calcOpticalFlowPyrLK().
/// This RLOF implementation can be seen as an improved pyramidal iterative Lucas-Kanade and includes
/// a set of improving modules. The main improvements in respect to the pyramidal iterative Lucas-Kanade
/// are:
///  - A more robust redecending M-estimator framework (see [Senst2012](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2012)) to improve the accuracy at
///    motion boundaries and appearing and disappearing pixels.
///  - an adaptive support region strategies to improve the accuracy at motion boundaries to reduce the
///    corona effect, i.e oversmoothing of the PLK at motion/object boundaries. The cross-based segementation
///    strategy (SR_CROSS) proposed in [Senst2014](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2014) uses a simple segmenation approach to obtain the optimal
///    shape of the support region.
///  - To deal with illumination changes (outdoor sequences and shadow) the intensity constancy assumption
///    based optical flow equation has been adopt with the Gennert and Negahdaripour illumination model
///    (see [Senst2016](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2016)). This model can be switched on/off with the useIlluminationModel variable.
///  - By using a global motion prior initialization (see [Senst2016](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2016)) of the iterative refinement
///    the accuracy could be significantly improved for large displacements. This initialization can be
///    switched on and of with useGlobalMotionPrior variable.
/// 
/// The RLOF can be computed with the SparseOpticalFlow class or function interface to track a set of features
/// or with the DenseOpticalFlow class or function interface to compute dense optical flow.
/// ## See also
/// optflow::DenseRLOFOpticalFlow, optflow::calcOpticalFlowDenseRLOF(), optflow::SparseRLOFOpticalFlow, optflow::calcOpticalFlowSparseRLOF()
pub trait RLOFOpticalFlowParameterTrait {
	fn as_raw_RLOFOpticalFlowParameter(&self) -> *const c_void;
	fn as_raw_mut_RLOFOpticalFlowParameter(&mut self) -> *mut c_void;

	fn solver_type(&self) -> crate::optflow::SolverType {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropSolverType_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result().expect("Infallible function failed: solver_type")
	}
	
	fn set_solver_type(&mut self, val: crate::optflow::SolverType) -> () {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropSolverType_SolverType(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result().expect("Infallible function failed: set_solver_type")
	}
	
	fn support_region_type(&self) -> crate::optflow::SupportRegionType {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropSupportRegionType_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result().expect("Infallible function failed: support_region_type")
	}
	
	fn set_support_region_type(&mut self, val: crate::optflow::SupportRegionType) -> () {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropSupportRegionType_SupportRegionType(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result().expect("Infallible function failed: set_support_region_type")
	}
	
	fn norm_sigma0(&self) -> f32 {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropNormSigma0_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result().expect("Infallible function failed: norm_sigma0")
	}
	
	fn set_norm_sigma0(&mut self, val: f32) -> () {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropNormSigma0_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result().expect("Infallible function failed: set_norm_sigma0")
	}
	
	fn norm_sigma1(&self) -> f32 {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropNormSigma1_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result().expect("Infallible function failed: norm_sigma1")
	}
	
	fn set_norm_sigma1(&mut self, val: f32) -> () {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropNormSigma1_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result().expect("Infallible function failed: set_norm_sigma1")
	}
	
	fn small_win_size(&self) -> i32 {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropSmallWinSize_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result().expect("Infallible function failed: small_win_size")
	}
	
	fn set_small_win_size(&mut self, val: i32) -> () {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropSmallWinSize_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result().expect("Infallible function failed: set_small_win_size")
	}
	
	fn large_win_size(&self) -> i32 {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropLargeWinSize_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result().expect("Infallible function failed: large_win_size")
	}
	
	fn set_large_win_size(&mut self, val: i32) -> () {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropLargeWinSize_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result().expect("Infallible function failed: set_large_win_size")
	}
	
	fn cross_segmentation_threshold(&self) -> i32 {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropCrossSegmentationThreshold_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result().expect("Infallible function failed: cross_segmentation_threshold")
	}
	
	fn set_cross_segmentation_threshold(&mut self, val: i32) -> () {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropCrossSegmentationThreshold_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result().expect("Infallible function failed: set_cross_segmentation_threshold")
	}
	
	fn max_level(&self) -> i32 {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropMaxLevel_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result().expect("Infallible function failed: max_level")
	}
	
	fn set_max_level(&mut self, val: i32) -> () {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropMaxLevel_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result().expect("Infallible function failed: set_max_level")
	}
	
	fn use_initial_flow(&self) -> bool {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropUseInitialFlow_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result().expect("Infallible function failed: use_initial_flow")
	}
	
	fn set_use_initial_flow(&mut self, val: bool) -> () {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropUseInitialFlow_bool(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result().expect("Infallible function failed: set_use_initial_flow")
	}
	
	fn use_illumination_model(&self) -> bool {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropUseIlluminationModel_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result().expect("Infallible function failed: use_illumination_model")
	}
	
	fn set_use_illumination_model(&mut self, val: bool) -> () {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropUseIlluminationModel_bool(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result().expect("Infallible function failed: set_use_illumination_model")
	}
	
	fn use_global_motion_prior(&self) -> bool {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropUseGlobalMotionPrior_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result().expect("Infallible function failed: use_global_motion_prior")
	}
	
	fn set_use_global_motion_prior(&mut self, val: bool) -> () {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropUseGlobalMotionPrior_bool(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result().expect("Infallible function failed: set_use_global_motion_prior")
	}
	
	fn max_iteration(&self) -> i32 {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropMaxIteration_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result().expect("Infallible function failed: max_iteration")
	}
	
	fn set_max_iteration(&mut self, val: i32) -> () {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropMaxIteration_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result().expect("Infallible function failed: set_max_iteration")
	}
	
	fn min_eigen_value(&self) -> f32 {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropMinEigenValue_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result().expect("Infallible function failed: min_eigen_value")
	}
	
	fn set_min_eigen_value(&mut self, val: f32) -> () {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropMinEigenValue_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result().expect("Infallible function failed: set_min_eigen_value")
	}
	
	fn global_motion_ransac_threshold(&self) -> f32 {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getPropGlobalMotionRansacThreshold_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result().expect("Infallible function failed: global_motion_ransac_threshold")
	}
	
	fn set_global_motion_ransac_threshold(&mut self, val: f32) -> () {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setPropGlobalMotionRansacThreshold_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result().expect("Infallible function failed: set_global_motion_ransac_threshold")
	}
	
	/// Enable M-estimator or disable and use least-square estimator.
	/// Enables M-estimator by setting sigma parameters to (3.2, 7.0). Disabling M-estimator can reduce
	///      *  runtime, while enabling can improve the accuracy.
	/// ## Parameters
	/// * val: If true M-estimator is used. If false least-square estimator is used.
	///      *    see also: setNormSigma0, setNormSigma1
	fn set_use_m_estimator(&mut self, val: bool) -> Result<()> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setUseMEstimator_bool(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result()
	}
	
	fn set_solver_type_1(&mut self, val: crate::optflow::SolverType) -> Result<()> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setSolverType_SolverType(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result()
	}
	
	fn get_solver_type(&self) -> Result<crate::optflow::SolverType> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getSolverType_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result()
	}
	
	fn set_support_region_type_1(&mut self, val: crate::optflow::SupportRegionType) -> Result<()> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setSupportRegionType_SupportRegionType(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result()
	}
	
	fn get_support_region_type(&self) -> Result<crate::optflow::SupportRegionType> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getSupportRegionType_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result()
	}
	
	fn set_norm_sigma0_1(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setNormSigma0_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result()
	}
	
	fn get_norm_sigma0(&self) -> Result<f32> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getNormSigma0_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result()
	}
	
	fn set_norm_sigma1_1(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setNormSigma1_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result()
	}
	
	fn get_norm_sigma1(&self) -> Result<f32> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getNormSigma1_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result()
	}
	
	fn set_small_win_size_1(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setSmallWinSize_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result()
	}
	
	fn get_small_win_size(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getSmallWinSize_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result()
	}
	
	fn set_large_win_size_1(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setLargeWinSize_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result()
	}
	
	fn get_large_win_size(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getLargeWinSize_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result()
	}
	
	fn set_cross_segmentation_threshold_1(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setCrossSegmentationThreshold_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result()
	}
	
	fn get_cross_segmentation_threshold(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getCrossSegmentationThreshold_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result()
	}
	
	fn set_max_level_1(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setMaxLevel_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result()
	}
	
	fn get_max_level(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getMaxLevel_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result()
	}
	
	fn set_use_initial_flow_1(&mut self, val: bool) -> Result<()> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setUseInitialFlow_bool(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result()
	}
	
	fn get_use_initial_flow(&self) -> Result<bool> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getUseInitialFlow_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result()
	}
	
	fn set_use_illumination_model_1(&mut self, val: bool) -> Result<()> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setUseIlluminationModel_bool(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result()
	}
	
	fn get_use_illumination_model(&self) -> Result<bool> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getUseIlluminationModel_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result()
	}
	
	fn set_use_global_motion_prior_1(&mut self, val: bool) -> Result<()> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setUseGlobalMotionPrior_bool(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result()
	}
	
	fn get_use_global_motion_prior(&self) -> Result<bool> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getUseGlobalMotionPrior_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result()
	}
	
	fn set_max_iteration_1(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setMaxIteration_int(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result()
	}
	
	fn get_max_iteration(&self) -> Result<i32> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getMaxIteration_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result()
	}
	
	fn set_min_eigen_value_1(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setMinEigenValue_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result()
	}
	
	fn get_min_eigen_value(&self) -> Result<f32> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getMinEigenValue_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result()
	}
	
	fn set_global_motion_ransac_threshold_1(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_setGlobalMotionRansacThreshold_float(self.as_raw_mut_RLOFOpticalFlowParameter(), val) }.into_result()
	}
	
	fn get_global_motion_ransac_threshold(&self) -> Result<f32> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_getGlobalMotionRansacThreshold_const(self.as_raw_RLOFOpticalFlowParameter()) }.into_result()
	}
	
}

/// This is used store and set up the parameters of the robust local optical flow (RLOF) algoritm.
/// 
/// The RLOF is a fast local optical flow approach described in [Senst2012](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2012) [Senst2013](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2013) [Senst2014](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2014)
/// and [Senst2016](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2016) similar to the pyramidal iterative Lucas-Kanade method as
/// proposed by [Bouguet00](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Bouguet00). More details and experiments can be found in the following thesis [Senst2019](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2019).
/// The implementation is derived from optflow::calcOpticalFlowPyrLK().
/// This RLOF implementation can be seen as an improved pyramidal iterative Lucas-Kanade and includes
/// a set of improving modules. The main improvements in respect to the pyramidal iterative Lucas-Kanade
/// are:
///  - A more robust redecending M-estimator framework (see [Senst2012](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2012)) to improve the accuracy at
///    motion boundaries and appearing and disappearing pixels.
///  - an adaptive support region strategies to improve the accuracy at motion boundaries to reduce the
///    corona effect, i.e oversmoothing of the PLK at motion/object boundaries. The cross-based segementation
///    strategy (SR_CROSS) proposed in [Senst2014](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2014) uses a simple segmenation approach to obtain the optimal
///    shape of the support region.
///  - To deal with illumination changes (outdoor sequences and shadow) the intensity constancy assumption
///    based optical flow equation has been adopt with the Gennert and Negahdaripour illumination model
///    (see [Senst2016](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2016)). This model can be switched on/off with the useIlluminationModel variable.
///  - By using a global motion prior initialization (see [Senst2016](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2016)) of the iterative refinement
///    the accuracy could be significantly improved for large displacements. This initialization can be
///    switched on and of with useGlobalMotionPrior variable.
/// 
/// The RLOF can be computed with the SparseOpticalFlow class or function interface to track a set of features
/// or with the DenseOpticalFlow class or function interface to compute dense optical flow.
/// ## See also
/// optflow::DenseRLOFOpticalFlow, optflow::calcOpticalFlowDenseRLOF(), optflow::SparseRLOFOpticalFlow, optflow::calcOpticalFlowSparseRLOF()
pub struct RLOFOpticalFlowParameter {
	ptr: *mut c_void
}

opencv_type_boxed! { RLOFOpticalFlowParameter }

impl Drop for RLOFOpticalFlowParameter {
	fn drop(&mut self) {
		extern "C" { fn cv_RLOFOpticalFlowParameter_delete(instance: *mut c_void); }
		unsafe { cv_RLOFOpticalFlowParameter_delete(self.as_raw_mut_RLOFOpticalFlowParameter()) };
	}
}

impl RLOFOpticalFlowParameter {
	#[inline] pub fn as_raw_RLOFOpticalFlowParameter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_RLOFOpticalFlowParameter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for RLOFOpticalFlowParameter {}

impl crate::optflow::RLOFOpticalFlowParameterTrait for RLOFOpticalFlowParameter {
	#[inline] fn as_raw_RLOFOpticalFlowParameter(&self) -> *const c_void { self.as_raw() }
	#[inline] fn as_raw_mut_RLOFOpticalFlowParameter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RLOFOpticalFlowParameter {
	pub fn default() -> Result<crate::optflow::RLOFOpticalFlowParameter> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_RLOFOpticalFlowParameter() }.into_result().map(|r| unsafe { crate::optflow::RLOFOpticalFlowParameter::opencv_from_extern(r) } )
	}
	
	/// Creates instance of optflow::RLOFOpticalFlowParameter
	pub fn create() -> Result<core::Ptr::<crate::optflow::RLOFOpticalFlowParameter>> {
		unsafe { sys::cv_optflow_RLOFOpticalFlowParameter_create() }.into_result().map(|r| unsafe { core::Ptr::<crate::optflow::RLOFOpticalFlowParameter>::opencv_from_extern(r) } )
	}
	
}

/// Class used for calculation sparse optical flow and feature tracking with robust local optical flow (RLOF) algorithms.
/// 
/// The RLOF is a fast local optical flow approach described in [Senst2012](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2012) [Senst2013](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2013) [Senst2014](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2014)
/// and [Senst2016](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2016) similar to the pyramidal iterative Lucas-Kanade method as
/// proposed by [Bouguet00](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Bouguet00). More details and experiments can be found in the following thesis [Senst2019](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2019).
/// The implementation is derived from optflow::calcOpticalFlowPyrLK().
/// 
/// For the RLOF configuration see optflow::RLOFOpticalFlowParameter for further details.
/// Parameters have been described in [Senst2012](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2012), [Senst2013](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2013), [Senst2014](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2014) and [Senst2016](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_Senst2016).
/// 
/// 
/// Note: SIMD parallelization is only available when compiling with SSE4.1.
/// ## See also
/// optflow::calcOpticalFlowSparseRLOF(), optflow::RLOFOpticalFlowParameter
pub trait SparseRLOFOpticalFlow: crate::video::SparseOpticalFlow {
	fn as_raw_SparseRLOFOpticalFlow(&self) -> *const c_void;
	fn as_raw_mut_SparseRLOFOpticalFlow(&mut self) -> *mut c_void;

	/// @copydoc DenseRLOFOpticalFlow::setRLOFOpticalFlowParameter
	fn set_rlof_optical_flow_parameter(&mut self, mut val: core::Ptr::<crate::optflow::RLOFOpticalFlowParameter>) -> Result<()> {
		unsafe { sys::cv_optflow_SparseRLOFOpticalFlow_setRLOFOpticalFlowParameter_Ptr_RLOFOpticalFlowParameter_(self.as_raw_mut_SparseRLOFOpticalFlow(), val.as_raw_mut_PtrOfRLOFOpticalFlowParameter()) }.into_result()
	}
	
	/// @copydoc DenseRLOFOpticalFlow::setRLOFOpticalFlowParameter
	/// ## See also
	/// setRLOFOpticalFlowParameter
	fn get_rlof_optical_flow_parameter(&self) -> Result<core::Ptr::<crate::optflow::RLOFOpticalFlowParameter>> {
		unsafe { sys::cv_optflow_SparseRLOFOpticalFlow_getRLOFOpticalFlowParameter_const(self.as_raw_SparseRLOFOpticalFlow()) }.into_result().map(|r| unsafe { core::Ptr::<crate::optflow::RLOFOpticalFlowParameter>::opencv_from_extern(r) } )
	}
	
	/// Threshold for the forward backward confidence check
	/// For each feature point a motion vector ![inline formula](https://latex.codecogs.com/png.latex?%20d%5F%7BI0%2CI1%7D%28%5Cmathbf%7Bx%7D%29%20) is computed.
	///      *     If the forward backward error ![block formula](https://latex.codecogs.com/png.latex?%20EP%5F%7BFB%7D%20%3D%20%7C%7C%20d%5F%7BI0%2CI1%7D%20%2B%20d%5F%7BI1%2CI0%7D%20%7C%7C%20)
	///      *     is larger than threshold given by this function then the status  will not be used by the following
	///      *    vector field interpolation. ![inline formula](https://latex.codecogs.com/png.latex?%20d%5F%7BI1%2CI0%7D%20) denotes the backward flow. Note, the forward backward test
	///      *    will only be applied if the threshold > 0. This may results into a doubled runtime for the motion estimation.
	///      *    see also: setForwardBackward
	fn set_forward_backward(&mut self, val: f32) -> Result<()> {
		unsafe { sys::cv_optflow_SparseRLOFOpticalFlow_setForwardBackward_float(self.as_raw_mut_SparseRLOFOpticalFlow(), val) }.into_result()
	}
	
	/// Threshold for the forward backward confidence check
	/// For each feature point a motion vector ![inline formula](https://latex.codecogs.com/png.latex?%20d%5F%7BI0%2CI1%7D%28%5Cmathbf%7Bx%7D%29%20) is computed.
	///      *     If the forward backward error ![block formula](https://latex.codecogs.com/png.latex?%20EP%5F%7BFB%7D%20%3D%20%7C%7C%20d%5F%7BI0%2CI1%7D%20%2B%20d%5F%7BI1%2CI0%7D%20%7C%7C%20)
	///      *     is larger than threshold given by this function then the status  will not be used by the following
	///      *    vector field interpolation. ![inline formula](https://latex.codecogs.com/png.latex?%20d%5F%7BI1%2CI0%7D%20) denotes the backward flow. Note, the forward backward test
	///      *    will only be applied if the threshold > 0. This may results into a doubled runtime for the motion estimation.
	///      *    see also: setForwardBackward
	///      *    see also: setForwardBackward
	fn get_forward_backward(&self) -> Result<f32> {
		unsafe { sys::cv_optflow_SparseRLOFOpticalFlow_getForwardBackward_const(self.as_raw_SparseRLOFOpticalFlow()) }.into_result()
	}
	
}

impl dyn SparseRLOFOpticalFlow + '_ {
	/// Creates instance of SparseRLOFOpticalFlow
	/// 
	/// ## Parameters
	/// * rlofParam: see setRLOFOpticalFlowParameter
	/// * forwardBackwardThreshold: see setForwardBackward
	/// 
	/// ## C++ default parameters
	/// * rlof_param: Ptr<RLOFOpticalFlowParameter>()
	/// * forward_backward_threshold: 1.f
	pub fn create(mut rlof_param: core::Ptr::<crate::optflow::RLOFOpticalFlowParameter>, forward_backward_threshold: f32) -> Result<core::Ptr::<dyn crate::optflow::SparseRLOFOpticalFlow>> {
		unsafe { sys::cv_optflow_SparseRLOFOpticalFlow_create_Ptr_RLOFOpticalFlowParameter__float(rlof_param.as_raw_mut_PtrOfRLOFOpticalFlowParameter(), forward_backward_threshold) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::optflow::SparseRLOFOpticalFlow>::opencv_from_extern(r) } )
	}
	
}