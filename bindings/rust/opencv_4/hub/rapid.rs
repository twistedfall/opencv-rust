//! # silhouette based 3D object tracking
//! 
//! implements "RAPID-a video rate object tracker" [harris1990rapid](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_harris1990rapid) with the dynamic control point extraction of [drummond2002real](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_drummond2002real)
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use {  };
}

/// Debug draw markers of matched correspondences onto a lineBundle
/// ## Parameters
/// * bundle: the lineBundle
/// * srcLocations: the according source locations
/// * newLocations: matched source locations
/// * colors: colors for the markers. Defaults to white.
/// 
/// ## C++ default parameters
/// * colors: noArray()
pub fn draw_correspondencies(bundle: &mut dyn core::ToInputOutputArray, src_locations: &dyn core::ToInputArray, new_locations: &dyn core::ToInputArray, colors: &dyn core::ToInputArray) -> Result<()> {
	input_output_array_arg!(bundle);
	input_array_arg!(src_locations);
	input_array_arg!(new_locations);
	input_array_arg!(colors);
	unsafe { sys::cv_rapid_drawCorrespondencies_const__InputOutputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX(bundle.as_raw__InputOutputArray(), src_locations.as_raw__InputArray(), new_locations.as_raw__InputArray(), colors.as_raw__InputArray()) }.into_result()
}

/// Debug draw search lines onto an image
/// ## Parameters
/// * img: the output image
/// * locations: the source locations of a line bundle
/// * color: the line color
pub fn draw_search_lines(img: &mut dyn core::ToInputOutputArray, locations: &dyn core::ToInputArray, color: core::Scalar) -> Result<()> {
	input_output_array_arg!(img);
	input_array_arg!(locations);
	unsafe { sys::cv_rapid_drawSearchLines_const__InputOutputArrayX_const__InputArrayX_const_ScalarX(img.as_raw__InputOutputArray(), locations.as_raw__InputArray(), &color) }.into_result()
}

/// Draw a wireframe of a triangle mesh
/// ## Parameters
/// * img: the output image
/// * pts2d: the 2d points obtained by @ref projectPoints
/// * tris: triangle face connectivity
/// * color: line color
/// * type: line type. See @ref LineTypes.
/// * cullBackface: enable back-face culling based on CCW order
/// 
/// ## C++ default parameters
/// * typ: LINE_8
/// * cull_backface: false
pub fn draw_wireframe(img: &mut dyn core::ToInputOutputArray, pts2d: &dyn core::ToInputArray, tris: &dyn core::ToInputArray, color: core::Scalar, typ: i32, cull_backface: bool) -> Result<()> {
	input_output_array_arg!(img);
	input_array_arg!(pts2d);
	input_array_arg!(tris);
	unsafe { sys::cv_rapid_drawWireframe_const__InputOutputArrayX_const__InputArrayX_const__InputArrayX_const_ScalarX_int_bool(img.as_raw__InputOutputArray(), pts2d.as_raw__InputArray(), tris.as_raw__InputArray(), &color, typ, cull_backface) }.into_result()
}

/// Extract control points from the projected silhouette of a mesh
/// 
/// see [drummond2002real](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_drummond2002real) Sec 2.1, Step b
/// ## Parameters
/// * num: number of control points
/// * len: search radius (used to restrict the ROI)
/// * pts3d: the 3D points of the mesh
/// * rvec: rotation between mesh and camera
/// * tvec: translation between mesh and camera
/// * K: camera intrinsic
/// * imsize: size of the video frame
/// * tris: triangle face connectivity
/// * ctl2d: the 2D locations of the control points
/// * ctl3d: matching 3D points of the mesh
pub fn extract_control_points(num: i32, len: i32, pts3d: &dyn core::ToInputArray, rvec: &dyn core::ToInputArray, tvec: &dyn core::ToInputArray, k: &dyn core::ToInputArray, imsize: core::Size, tris: &dyn core::ToInputArray, ctl2d: &mut dyn core::ToOutputArray, ctl3d: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(pts3d);
	input_array_arg!(rvec);
	input_array_arg!(tvec);
	input_array_arg!(k);
	input_array_arg!(tris);
	output_array_arg!(ctl2d);
	output_array_arg!(ctl3d);
	unsafe { sys::cv_rapid_extractControlPoints_int_int_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const_SizeX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(num, len, pts3d.as_raw__InputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), k.as_raw__InputArray(), &imsize, tris.as_raw__InputArray(), ctl2d.as_raw__OutputArray(), ctl3d.as_raw__OutputArray()) }.into_result()
}

/// Extract the line bundle from an image
/// ## Parameters
/// * len: the search radius. The bundle will have `2*len + 1` columns.
/// * ctl2d: the search lines will be centered at this points and orthogonal to the contour defined by
/// them. The bundle will have as many rows.
/// * img: the image to read the pixel intensities values from
/// * bundle: line bundle image with size `ctl2d.rows() x (2 * len + 1)` and the same type as @p img
/// * srcLocations: the source pixel locations of @p bundle in @p img as CV_16SC2
pub fn extract_line_bundle(len: i32, ctl2d: &dyn core::ToInputArray, img: &dyn core::ToInputArray, bundle: &mut dyn core::ToOutputArray, src_locations: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(ctl2d);
	input_array_arg!(img);
	output_array_arg!(bundle);
	output_array_arg!(src_locations);
	unsafe { sys::cv_rapid_extractLineBundle_int_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(len, ctl2d.as_raw__InputArray(), img.as_raw__InputArray(), bundle.as_raw__OutputArray(), src_locations.as_raw__OutputArray()) }.into_result()
}

/// Filter corresponding 2d and 3d points based on mask
/// ## Parameters
/// * pts2d: 2d points
/// * pts3d: 3d points
/// * mask: mask containing non-zero values for the elements to be retained
pub fn filter_correspondencies(pts2d: &mut dyn core::ToInputOutputArray, pts3d: &mut dyn core::ToInputOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
	input_output_array_arg!(pts2d);
	input_output_array_arg!(pts3d);
	input_array_arg!(mask);
	unsafe { sys::cv_rapid_filterCorrespondencies_const__InputOutputArrayX_const__InputOutputArrayX_const__InputArrayX(pts2d.as_raw__InputOutputArray(), pts3d.as_raw__InputOutputArray(), mask.as_raw__InputArray()) }.into_result()
}

/// Find corresponding image locations by searching for a maximal sobel edge along the search line (a single
/// row in the bundle)
/// ## Parameters
/// * bundle: the line bundle
/// * srcLocations: the according source image location
/// * newLocations: image locations with maximal edge along the search line
/// * response: the sobel response for the selected point
/// 
/// ## C++ default parameters
/// * response: noArray()
pub fn find_correspondencies(bundle: &dyn core::ToInputArray, src_locations: &dyn core::ToInputArray, new_locations: &mut dyn core::ToOutputArray, response: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(bundle);
	input_array_arg!(src_locations);
	output_array_arg!(new_locations);
	output_array_arg!(response);
	unsafe { sys::cv_rapid_findCorrespondencies_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(bundle.as_raw__InputArray(), src_locations.as_raw__InputArray(), new_locations.as_raw__OutputArray(), response.as_raw__OutputArray()) }.into_result()
}

/// High level function to execute a single rapid [harris1990rapid](https://docs.opencv.org/4.3.0/d0/de3/citelist.html#CITEREF_harris1990rapid) iteration
/// 
/// 1. @ref extractControlPoints
/// 2. @ref extractLineBundle
/// 3. @ref findCorrespondencies
/// 4. @ref filterCorrespondencies
/// 5. @ref solvePnPRefineLM
/// 
/// ## Parameters
/// * img: the video frame
/// * num: number of search lines
/// * len: search line radius
/// * pts3d: the 3D points of the mesh
/// * tris: triangle face connectivity
/// * K: camera matrix
/// * rvec: rotation between mesh and camera. Input values are used as an initial solution.
/// * tvec: translation between mesh and camera. Input values are used as an initial solution.
/// ## Returns
/// ratio of search lines that could be extracted and matched
pub fn rapid(img: &dyn core::ToInputArray, num: i32, len: i32, pts3d: &dyn core::ToInputArray, tris: &dyn core::ToInputArray, k: &dyn core::ToInputArray, rvec: &mut dyn core::ToInputOutputArray, tvec: &mut dyn core::ToInputOutputArray) -> Result<f32> {
	input_array_arg!(img);
	input_array_arg!(pts3d);
	input_array_arg!(tris);
	input_array_arg!(k);
	input_output_array_arg!(rvec);
	input_output_array_arg!(tvec);
	unsafe { sys::cv_rapid_rapid_const__InputArrayX_int_int_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputOutputArrayX_const__InputOutputArrayX(img.as_raw__InputArray(), num, len, pts3d.as_raw__InputArray(), tris.as_raw__InputArray(), k.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray()) }.into_result()
}
