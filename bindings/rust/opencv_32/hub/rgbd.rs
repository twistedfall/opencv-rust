#![allow(unused_parens)]
//! # RGB-Depth Processing
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::RgbdNormalsTrait, super::DepthCleanerTrait, super::RgbdPlaneTrait, super::RgbdFrameTrait, super::OdometryFrameTrait, super::Odometry, super::RgbdOdometryTrait, super::Linemod_FeatureTrait, super::Linemod_TemplateTrait, super::Linemod_QuantizedPyramid, super::Linemod_Modality, super::Linemod_ColorGradientTrait, super::Linemod_DepthNormalTrait, super::Linemod_MatchTrait, super::Linemod_DetectorTrait };
}

pub const OdometryFrame_CACHE_ALL: i32 = 3;
pub const OdometryFrame_CACHE_DST: i32 = 2;
pub const OdometryFrame_CACHE_SRC: i32 = 1;
pub const Odometry_RIGID_BODY_MOTION: i32 = 4;
pub const Odometry_ROTATION: i32 = 1;
pub const Odometry_TRANSLATION: i32 = 2;
/// NIL method is from
/// ``Modeling Kinect Sensor Noise for Improved 3d Reconstruction and Tracking``
/// by C. Nguyen, S. Izadi, D. Lovel
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DepthCleaner_DEPTH_CLEANER_METHOD {
	DEPTH_CLEANER_NIL = 0 as isize,
}

opencv_type_enum! { crate::rgbd::DepthCleaner_DEPTH_CLEANER_METHOD }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RgbdNormals_RGBD_NORMALS_METHOD {
	RGBD_NORMALS_METHOD_FALS = 0 as isize,
	RGBD_NORMALS_METHOD_LINEMOD = 1 as isize,
	RGBD_NORMALS_METHOD_SRI = 2 as isize,
}

opencv_type_enum! { crate::rgbd::RgbdNormals_RGBD_NORMALS_METHOD }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RgbdPlane_RGBD_PLANE_METHOD {
	RGBD_PLANE_METHOD_DEFAULT = 0 as isize,
}

opencv_type_enum! { crate::rgbd::RgbdPlane_RGBD_PLANE_METHOD }

/// \brief Factory function for detector using LINE algorithm with color gradients.
/// 
/// Default parameter settings suitable for VGA images.
pub fn get_default_line() -> Result<core::Ptr::<crate::rgbd::Linemod_Detector>> {
	unsafe { sys::cv_linemod_getDefaultLINE() }.into_result().map(|r| unsafe { core::Ptr::<crate::rgbd::Linemod_Detector>::opencv_from_extern(r) } )
}

/// \brief Factory function for detector using LINE-MOD algorithm with color gradients
/// and depth normals.
/// 
/// Default parameter settings suitable for VGA images.
pub fn get_default_linemod() -> Result<core::Ptr::<crate::rgbd::Linemod_Detector>> {
	unsafe { sys::cv_linemod_getDefaultLINEMOD() }.into_result().map(|r| unsafe { core::Ptr::<crate::rgbd::Linemod_Detector>::opencv_from_extern(r) } )
}

/// ## Parameters
/// * depth: the depth image
/// * in_K: 
/// * in_points: the list of xy coordinates
/// * points3d: the resulting 3d points
pub fn depth_to3d_sparse(depth: &dyn core::ToInputArray, in_k: &dyn core::ToInputArray, in_points: &dyn core::ToInputArray, points3d: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(depth);
	input_array_arg!(in_k);
	input_array_arg!(in_points);
	output_array_arg!(points3d);
	unsafe { sys::cv_rgbd_depthTo3dSparse_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX(depth.as_raw__InputArray(), in_k.as_raw__InputArray(), in_points.as_raw__InputArray(), points3d.as_raw__OutputArray()) }.into_result()
}

/// Converts a depth image to an organized set of 3d points.
/// The coordinate system is x pointing left, y down and z away from the camera
/// ## Parameters
/// * depth: the depth image (if given as short int CV_U, it is assumed to be the depth in millimeters
///              (as done with the Microsoft Kinect), otherwise, if given as CV_32F or CV_64F, it is assumed in meters)
/// * K: The calibration matrix
/// * points3d: the resulting 3d points. They are of depth the same as `depth` if it is CV_32F or CV_64F, and the
///        depth of `K` if `depth` is of depth CV_U
/// * mask: the mask of the points to consider (can be empty)
/// 
/// ## C++ default parameters
/// * mask: noArray()
pub fn depth_to3d(depth: &dyn core::ToInputArray, k: &dyn core::ToInputArray, points3d: &mut dyn core::ToOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
	input_array_arg!(depth);
	input_array_arg!(k);
	output_array_arg!(points3d);
	input_array_arg!(mask);
	unsafe { sys::cv_rgbd_depthTo3d_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__InputArrayX(depth.as_raw__InputArray(), k.as_raw__InputArray(), points3d.as_raw__OutputArray(), mask.as_raw__InputArray()) }.into_result()
}

pub fn is_valid_depth_1(depth: &f64) -> Result<bool> {
	unsafe { sys::cv_rgbd_isValidDepth_const_doubleR(depth) }.into_result()
}

/// Checks if the value is a valid depth. For CV_16U or CV_16S, the convention is to be invalid if it is
/// a limit. For a float/double, we just check if it is a NaN
/// ## Parameters
/// * depth: the depth to check for validity
pub fn is_valid_depth(depth: &f32) -> Result<bool> {
	unsafe { sys::cv_rgbd_isValidDepth_const_floatR(depth) }.into_result()
}

pub fn is_valid_depth_4(depth: &i32) -> Result<bool> {
	unsafe { sys::cv_rgbd_isValidDepth_const_intR(depth) }.into_result()
}

pub fn is_valid_depth_2(depth: &i16) -> Result<bool> {
	unsafe { sys::cv_rgbd_isValidDepth_const_shortR(depth) }.into_result()
}

pub fn is_valid_depth_5(depth: &u32) -> Result<bool> {
	unsafe { sys::cv_rgbd_isValidDepth_const_unsigned_intR(depth) }.into_result()
}

pub fn is_valid_depth_3(depth: &u16) -> Result<bool> {
	unsafe { sys::cv_rgbd_isValidDepth_const_unsigned_shortR(depth) }.into_result()
}

/// Registers depth data to an external camera
/// Registration is performed by creating a depth cloud, transforming the cloud by
/// the rigid body transformation between the cameras, and then projecting the
/// transformed points into the RGB camera.
/// 
/// uv_rgb = K_rgb * [R | t] * z * inv(K_ir) * uv_ir
/// 
/// Currently does not check for negative depth values.
/// 
/// ## Parameters
/// * unregisteredCameraMatrix: the camera matrix of the depth camera
/// * registeredCameraMatrix: the camera matrix of the external camera
/// * registeredDistCoeffs: the distortion coefficients of the external camera
/// * Rt: the rigid body transform between the cameras. Transforms points from depth camera frame to external camera frame.
/// * unregisteredDepth: the input depth data
/// * outputImagePlaneSize: the image plane dimensions of the external camera (width, height)
/// * registeredDepth: the result of transforming the depth into the external camera
/// * depthDilation: whether or not the depth is dilated to avoid holes and occlusion errors (optional)
/// 
/// ## C++ default parameters
/// * depth_dilation: false
pub fn register_depth(unregistered_camera_matrix: &dyn core::ToInputArray, registered_camera_matrix: &dyn core::ToInputArray, registered_dist_coeffs: &dyn core::ToInputArray, rt: &dyn core::ToInputArray, unregistered_depth: &dyn core::ToInputArray, output_image_plane_size: core::Size, registered_depth: &mut dyn core::ToOutputArray, depth_dilation: bool) -> Result<()> {
	input_array_arg!(unregistered_camera_matrix);
	input_array_arg!(registered_camera_matrix);
	input_array_arg!(registered_dist_coeffs);
	input_array_arg!(rt);
	input_array_arg!(unregistered_depth);
	output_array_arg!(registered_depth);
	unsafe { sys::cv_rgbd_registerDepth_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const_SizeX_const__OutputArrayX_bool(unregistered_camera_matrix.as_raw__InputArray(), registered_camera_matrix.as_raw__InputArray(), registered_dist_coeffs.as_raw__InputArray(), rt.as_raw__InputArray(), unregistered_depth.as_raw__InputArray(), &output_image_plane_size, registered_depth.as_raw__OutputArray(), depth_dilation) }.into_result()
}

/// If the input image is of type CV_16UC1 (like the Kinect one), the image is converted to floats, divided
/// by 1000 to get a depth in meters, and the values 0 are converted to std::numeric_limits<float>::quiet_NaN()
/// Otherwise, the image is simply converted to floats
/// ## Parameters
/// * in: the depth image (if given as short int CV_U, it is assumed to be the depth in millimeters
///              (as done with the Microsoft Kinect), it is assumed in meters)
/// * depth: the desired output depth (floats or double)
/// * out: The rescaled float depth image
pub fn rescale_depth(in_: &dyn core::ToInputArray, depth: i32, out: &mut dyn core::ToOutputArray) -> Result<()> {
	input_array_arg!(in_);
	output_array_arg!(out);
	unsafe { sys::cv_rgbd_rescaleDepth_const__InputArrayX_int_const__OutputArrayX(in_.as_raw__InputArray(), depth, out.as_raw__OutputArray()) }.into_result()
}

/// Warp the image: compute 3d points from the depth, transform them using given transformation,
/// then project color point cloud to an image plane.
/// This function can be used to visualize results of the Odometry algorithm.
/// ## Parameters
/// * image: The image (of CV_8UC1 or CV_8UC3 type)
/// * depth: The depth (of type used in depthTo3d fuction)
/// * mask: The mask of used pixels (of CV_8UC1), it can be empty
/// * Rt: The transformation that will be applied to the 3d points computed from the depth
/// * cameraMatrix: Camera matrix
/// * distCoeff: Distortion coefficients
/// * warpedImage: The warped image.
/// * warpedDepth: The warped depth.
/// * warpedMask: The warped mask.
/// 
/// ## C++ default parameters
/// * warped_depth: 0
/// * warped_mask: 0
pub fn warp_frame(image: &core::Mat, depth: &core::Mat, mask: &core::Mat, rt: &core::Mat, camera_matrix: &core::Mat, dist_coeff: &core::Mat, warped_image: &mut core::Mat, warped_depth: &mut core::Mat, warped_mask: &mut core::Mat) -> Result<()> {
	unsafe { sys::cv_rgbd_warpFrame_const_MatX_const_MatX_const_MatX_const_MatX_const_MatX_const_MatX_MatX_MatX_MatX(image.as_raw_Mat(), depth.as_raw_Mat(), mask.as_raw_Mat(), rt.as_raw_Mat(), camera_matrix.as_raw_Mat(), dist_coeff.as_raw_Mat(), warped_image.as_raw_mut_Mat(), warped_depth.as_raw_mut_Mat(), warped_mask.as_raw_mut_Mat()) }.into_result()
}

/// \brief Modality that computes quantized gradient orientations from a color image.
pub trait Linemod_ColorGradientTrait: crate::rgbd::Linemod_Modality {
	fn as_raw_Linemod_ColorGradient(&self) -> *const c_void;
	fn as_raw_mut_Linemod_ColorGradient(&mut self) -> *mut c_void;

	fn weak_threshold(&self) -> f32 {
		unsafe { sys::cv_linemod_ColorGradient_getPropWeak_threshold_const(self.as_raw_Linemod_ColorGradient()) }.into_result().expect("Infallible function failed: weak_threshold")
	}
	
	fn set_weak_threshold(&mut self, val: f32) -> () {
		unsafe { sys::cv_linemod_ColorGradient_setPropWeak_threshold_float(self.as_raw_mut_Linemod_ColorGradient(), val) }.into_result().expect("Infallible function failed: set_weak_threshold")
	}
	
	fn num_features(&self) -> size_t {
		unsafe { sys::cv_linemod_ColorGradient_getPropNum_features_const(self.as_raw_Linemod_ColorGradient()) }.into_result().expect("Infallible function failed: num_features")
	}
	
	fn set_num_features(&mut self, val: size_t) -> () {
		unsafe { sys::cv_linemod_ColorGradient_setPropNum_features_size_t(self.as_raw_mut_Linemod_ColorGradient(), val) }.into_result().expect("Infallible function failed: set_num_features")
	}
	
	fn strong_threshold(&self) -> f32 {
		unsafe { sys::cv_linemod_ColorGradient_getPropStrong_threshold_const(self.as_raw_Linemod_ColorGradient()) }.into_result().expect("Infallible function failed: strong_threshold")
	}
	
	fn set_strong_threshold(&mut self, val: f32) -> () {
		unsafe { sys::cv_linemod_ColorGradient_setPropStrong_threshold_float(self.as_raw_mut_Linemod_ColorGradient(), val) }.into_result().expect("Infallible function failed: set_strong_threshold")
	}
	
	fn name(&self) -> Result<String> {
		unsafe { sys::cv_linemod_ColorGradient_name_const(self.as_raw_Linemod_ColorGradient()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_linemod_ColorGradient_read_const_FileNodeX(self.as_raw_mut_Linemod_ColorGradient(), fn_.as_raw_FileNode()) }.into_result()
	}
	
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_linemod_ColorGradient_write_const_FileStorageX(self.as_raw_Linemod_ColorGradient(), fs.as_raw_mut_FileStorage()) }.into_result()
	}
	
}

/// \brief Modality that computes quantized gradient orientations from a color image.
pub struct Linemod_ColorGradient {
	ptr: *mut c_void
}

opencv_type_boxed! { Linemod_ColorGradient }

impl Drop for Linemod_ColorGradient {
	fn drop(&mut self) {
		extern "C" { fn cv_Linemod_ColorGradient_delete(instance: *mut c_void); }
		unsafe { cv_Linemod_ColorGradient_delete(self.as_raw_mut_Linemod_ColorGradient()) };
	}
}

impl Linemod_ColorGradient {
	pub fn as_raw_Linemod_ColorGradient(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_Linemod_ColorGradient(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Linemod_ColorGradient {}

impl crate::rgbd::Linemod_ColorGradientTrait for Linemod_ColorGradient {
	fn as_raw_Linemod_ColorGradient(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_Linemod_ColorGradient(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::Linemod_Modality for Linemod_ColorGradient {
	fn as_raw_Linemod_Modality(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_Linemod_Modality(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Linemod_ColorGradient {
	/// \brief Default constructor. Uses reasonable default parameter values.
	pub fn default() -> Result<crate::rgbd::Linemod_ColorGradient> {
		unsafe { sys::cv_linemod_ColorGradient_ColorGradient() }.into_result().map(|r| unsafe { crate::rgbd::Linemod_ColorGradient::opencv_from_extern(r) } )
	}
	
	/// \brief Constructor.
	/// 
	/// \param weak_threshold   When quantizing, discard gradients with magnitude less than this.
	/// \param num_features     How many features a template must contain.
	/// \param strong_threshold Consider as candidate features only gradients whose norms are
	///                         larger than this.
	pub fn new(weak_threshold: f32, num_features: size_t, strong_threshold: f32) -> Result<crate::rgbd::Linemod_ColorGradient> {
		unsafe { sys::cv_linemod_ColorGradient_ColorGradient_float_size_t_float(weak_threshold, num_features, strong_threshold) }.into_result().map(|r| unsafe { crate::rgbd::Linemod_ColorGradient::opencv_from_extern(r) } )
	}
	
}

/// \brief Modality that computes quantized surface normals from a dense depth map.
pub trait Linemod_DepthNormalTrait: crate::rgbd::Linemod_Modality {
	fn as_raw_Linemod_DepthNormal(&self) -> *const c_void;
	fn as_raw_mut_Linemod_DepthNormal(&mut self) -> *mut c_void;

	fn distance_threshold(&self) -> i32 {
		unsafe { sys::cv_linemod_DepthNormal_getPropDistance_threshold_const(self.as_raw_Linemod_DepthNormal()) }.into_result().expect("Infallible function failed: distance_threshold")
	}
	
	fn set_distance_threshold(&mut self, val: i32) -> () {
		unsafe { sys::cv_linemod_DepthNormal_setPropDistance_threshold_int(self.as_raw_mut_Linemod_DepthNormal(), val) }.into_result().expect("Infallible function failed: set_distance_threshold")
	}
	
	fn difference_threshold(&self) -> i32 {
		unsafe { sys::cv_linemod_DepthNormal_getPropDifference_threshold_const(self.as_raw_Linemod_DepthNormal()) }.into_result().expect("Infallible function failed: difference_threshold")
	}
	
	fn set_difference_threshold(&mut self, val: i32) -> () {
		unsafe { sys::cv_linemod_DepthNormal_setPropDifference_threshold_int(self.as_raw_mut_Linemod_DepthNormal(), val) }.into_result().expect("Infallible function failed: set_difference_threshold")
	}
	
	fn num_features(&self) -> size_t {
		unsafe { sys::cv_linemod_DepthNormal_getPropNum_features_const(self.as_raw_Linemod_DepthNormal()) }.into_result().expect("Infallible function failed: num_features")
	}
	
	fn set_num_features(&mut self, val: size_t) -> () {
		unsafe { sys::cv_linemod_DepthNormal_setPropNum_features_size_t(self.as_raw_mut_Linemod_DepthNormal(), val) }.into_result().expect("Infallible function failed: set_num_features")
	}
	
	fn extract_threshold(&self) -> i32 {
		unsafe { sys::cv_linemod_DepthNormal_getPropExtract_threshold_const(self.as_raw_Linemod_DepthNormal()) }.into_result().expect("Infallible function failed: extract_threshold")
	}
	
	fn set_extract_threshold(&mut self, val: i32) -> () {
		unsafe { sys::cv_linemod_DepthNormal_setPropExtract_threshold_int(self.as_raw_mut_Linemod_DepthNormal(), val) }.into_result().expect("Infallible function failed: set_extract_threshold")
	}
	
	fn name(&self) -> Result<String> {
		unsafe { sys::cv_linemod_DepthNormal_name_const(self.as_raw_Linemod_DepthNormal()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_linemod_DepthNormal_read_const_FileNodeX(self.as_raw_mut_Linemod_DepthNormal(), fn_.as_raw_FileNode()) }.into_result()
	}
	
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_linemod_DepthNormal_write_const_FileStorageX(self.as_raw_Linemod_DepthNormal(), fs.as_raw_mut_FileStorage()) }.into_result()
	}
	
}

/// \brief Modality that computes quantized surface normals from a dense depth map.
pub struct Linemod_DepthNormal {
	ptr: *mut c_void
}

opencv_type_boxed! { Linemod_DepthNormal }

impl Drop for Linemod_DepthNormal {
	fn drop(&mut self) {
		extern "C" { fn cv_Linemod_DepthNormal_delete(instance: *mut c_void); }
		unsafe { cv_Linemod_DepthNormal_delete(self.as_raw_mut_Linemod_DepthNormal()) };
	}
}

impl Linemod_DepthNormal {
	pub fn as_raw_Linemod_DepthNormal(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_Linemod_DepthNormal(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Linemod_DepthNormal {}

impl crate::rgbd::Linemod_DepthNormalTrait for Linemod_DepthNormal {
	fn as_raw_Linemod_DepthNormal(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_Linemod_DepthNormal(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::Linemod_Modality for Linemod_DepthNormal {
	fn as_raw_Linemod_Modality(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_Linemod_Modality(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Linemod_DepthNormal {
	/// \brief Default constructor. Uses reasonable default parameter values.
	pub fn default() -> Result<crate::rgbd::Linemod_DepthNormal> {
		unsafe { sys::cv_linemod_DepthNormal_DepthNormal() }.into_result().map(|r| unsafe { crate::rgbd::Linemod_DepthNormal::opencv_from_extern(r) } )
	}
	
	/// \brief Constructor.
	/// 
	/// \param distance_threshold   Ignore pixels beyond this distance.
	/// \param difference_threshold When computing normals, ignore contributions of pixels whose
	///                             depth difference with the central pixel is above this threshold.
	/// \param num_features         How many features a template must contain.
	/// \param extract_threshold    Consider as candidate feature only if there are no differing
	///                             orientations within a distance of extract_threshold.
	pub fn new(distance_threshold: i32, difference_threshold: i32, num_features: size_t, extract_threshold: i32) -> Result<crate::rgbd::Linemod_DepthNormal> {
		unsafe { sys::cv_linemod_DepthNormal_DepthNormal_int_int_size_t_int(distance_threshold, difference_threshold, num_features, extract_threshold) }.into_result().map(|r| unsafe { crate::rgbd::Linemod_DepthNormal::opencv_from_extern(r) } )
	}
	
}

/// \brief Object detector using the LINE template matching algorithm with any set of
/// modalities.
pub trait Linemod_DetectorTrait {
	fn as_raw_Linemod_Detector(&self) -> *const c_void;
	fn as_raw_mut_Linemod_Detector(&mut self) -> *mut c_void;

	/// \brief Detect objects by template matching.
	/// 
	/// Matches globally at the lowest pyramid level, then refines locally stepping up the pyramid.
	/// 
	/// \param      sources   Source images, one for each modality.
	/// \param      threshold Similarity threshold, a percentage between 0 and 100.
	/// \param[out] matches   Template matches, sorted by similarity score.
	/// \param      class_ids If non-empty, only search for the desired object classes.
	/// \param[out] quantized_images Optionally return vector<Mat> of quantized images.
	/// \param      masks     The masks for consideration during matching. The masks should be CV_8UC1
	///                       where 255 represents a valid pixel.  If non-empty, the vector must be
	///                       the same size as sources.  Each element must be
	///                       empty or the same size as its corresponding source.
	/// 
	/// ## C++ default parameters
	/// * class_ids: std::vector<String>()
	/// * quantized_images: noArray()
	/// * masks: std::vector<Mat>()
	fn match_(&self, sources: &core::Vector::<core::Mat>, threshold: f32, matches: &mut core::Vector::<crate::rgbd::Linemod_Match>, class_ids: &core::Vector::<String>, quantized_images: &mut dyn core::ToOutputArray, masks: &core::Vector::<core::Mat>) -> Result<()> {
		output_array_arg!(quantized_images);
		unsafe { sys::cv_linemod_Detector_match_const_const_vector_Mat_X_float_vector_Match_X_const_vector_String_X_const__OutputArrayX_const_vector_Mat_X(self.as_raw_Linemod_Detector(), sources.as_raw_VectorOfMat(), threshold, matches.as_raw_mut_VectorOfLinemod_Match(), class_ids.as_raw_VectorOfString(), quantized_images.as_raw__OutputArray(), masks.as_raw_VectorOfMat()) }.into_result()
	}
	
	/// \brief Add new object template.
	/// 
	/// \param      sources      Source images, one for each modality.
	/// \param      class_id     Object class ID.
	/// \param      object_mask  Mask separating object from background.
	/// \param[out] bounding_box Optionally return bounding box of the extracted features.
	/// 
	/// \return Template ID, or -1 if failed to extract a valid template.
	/// 
	/// ## C++ default parameters
	/// * bounding_box: NULL
	fn add_template(&mut self, sources: &core::Vector::<core::Mat>, class_id: &str, object_mask: &core::Mat, bounding_box: &mut core::Rect) -> Result<i32> {
		extern_container_arg!(class_id);
		unsafe { sys::cv_linemod_Detector_addTemplate_const_vector_Mat_X_const_StringX_const_MatX_RectX(self.as_raw_mut_Linemod_Detector(), sources.as_raw_VectorOfMat(), class_id.opencv_to_extern(), object_mask.as_raw_Mat(), bounding_box) }.into_result()
	}
	
	/// \brief Add a new object template computed by external means.
	fn add_synthetic_template(&mut self, templates: &core::Vector::<crate::rgbd::Linemod_Template>, class_id: &str) -> Result<i32> {
		extern_container_arg!(class_id);
		unsafe { sys::cv_linemod_Detector_addSyntheticTemplate_const_vector_Template_X_const_StringX(self.as_raw_mut_Linemod_Detector(), templates.as_raw_VectorOfLinemod_Template(), class_id.opencv_to_extern()) }.into_result()
	}
	
	/// \brief Get the modalities used by this detector.
	/// 
	/// You are not permitted to add/remove modalities, but you may dynamic_cast them to
	/// tweak parameters.
	fn get_modalities(&self) -> Result<core::Vector::<core::Ptr::<dyn crate::rgbd::Linemod_Modality>>> {
		unsafe { sys::cv_linemod_Detector_getModalities_const(self.as_raw_Linemod_Detector()) }.into_result().map(|r| unsafe { core::Vector::<core::Ptr::<dyn crate::rgbd::Linemod_Modality>>::opencv_from_extern(r) } )
	}
	
	/// \brief Get sampling step T at pyramid_level.
	fn get_t(&self, pyramid_level: i32) -> Result<i32> {
		unsafe { sys::cv_linemod_Detector_getT_const_int(self.as_raw_Linemod_Detector(), pyramid_level) }.into_result()
	}
	
	/// \brief Get number of pyramid levels used by this detector.
	fn pyramid_levels(&self) -> Result<i32> {
		unsafe { sys::cv_linemod_Detector_pyramidLevels_const(self.as_raw_Linemod_Detector()) }.into_result()
	}
	
	/// \brief Get the template pyramid identified by template_id.
	/// 
	/// For example, with 2 modalities (Gradient, Normal) and two pyramid levels
	/// (L0, L1), the order is (GradientL0, NormalL0, GradientL1, NormalL1).
	fn get_templates(&self, class_id: &str, template_id: i32) -> Result<core::Vector::<crate::rgbd::Linemod_Template>> {
		extern_container_arg!(class_id);
		unsafe { sys::cv_linemod_Detector_getTemplates_const_const_StringX_int(self.as_raw_Linemod_Detector(), class_id.opencv_to_extern(), template_id) }.into_result().map(|r| unsafe { core::Vector::<crate::rgbd::Linemod_Template>::opencv_from_extern(r) } )
	}
	
	fn num_templates(&self) -> Result<i32> {
		unsafe { sys::cv_linemod_Detector_numTemplates_const(self.as_raw_Linemod_Detector()) }.into_result()
	}
	
	fn num_templates_1(&self, class_id: &str) -> Result<i32> {
		extern_container_arg!(class_id);
		unsafe { sys::cv_linemod_Detector_numTemplates_const_const_StringX(self.as_raw_Linemod_Detector(), class_id.opencv_to_extern()) }.into_result()
	}
	
	fn num_classes(&self) -> Result<i32> {
		unsafe { sys::cv_linemod_Detector_numClasses_const(self.as_raw_Linemod_Detector()) }.into_result()
	}
	
	fn class_ids(&self) -> Result<core::Vector::<String>> {
		unsafe { sys::cv_linemod_Detector_classIds_const(self.as_raw_Linemod_Detector()) }.into_result().map(|r| unsafe { core::Vector::<String>::opencv_from_extern(r) } )
	}
	
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_linemod_Detector_read_const_FileNodeX(self.as_raw_mut_Linemod_Detector(), fn_.as_raw_FileNode()) }.into_result()
	}
	
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_linemod_Detector_write_const_FileStorageX(self.as_raw_Linemod_Detector(), fs.as_raw_mut_FileStorage()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * class_id_override: ""
	fn read_class(&mut self, fn_: &core::FileNode, class_id_override: &str) -> Result<String> {
		extern_container_arg!(class_id_override);
		unsafe { sys::cv_linemod_Detector_readClass_const_FileNodeX_const_StringX(self.as_raw_mut_Linemod_Detector(), fn_.as_raw_FileNode(), class_id_override.opencv_to_extern()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	fn write_class(&self, class_id: &str, fs: &mut core::FileStorage) -> Result<()> {
		extern_container_arg!(class_id);
		unsafe { sys::cv_linemod_Detector_writeClass_const_const_StringX_FileStorageX(self.as_raw_Linemod_Detector(), class_id.opencv_to_extern(), fs.as_raw_mut_FileStorage()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * format: "templates_%s.yml.gz"
	fn read_classes(&mut self, class_ids: &core::Vector::<String>, format: &str) -> Result<()> {
		extern_container_arg!(format);
		unsafe { sys::cv_linemod_Detector_readClasses_const_vector_String_X_const_StringX(self.as_raw_mut_Linemod_Detector(), class_ids.as_raw_VectorOfString(), format.opencv_to_extern()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * format: "templates_%s.yml.gz"
	fn write_classes(&self, format: &str) -> Result<()> {
		extern_container_arg!(format);
		unsafe { sys::cv_linemod_Detector_writeClasses_const_const_StringX(self.as_raw_Linemod_Detector(), format.opencv_to_extern()) }.into_result()
	}
	
}

/// \brief Object detector using the LINE template matching algorithm with any set of
/// modalities.
pub struct Linemod_Detector {
	ptr: *mut c_void
}

opencv_type_boxed! { Linemod_Detector }

impl Drop for Linemod_Detector {
	fn drop(&mut self) {
		extern "C" { fn cv_Linemod_Detector_delete(instance: *mut c_void); }
		unsafe { cv_Linemod_Detector_delete(self.as_raw_mut_Linemod_Detector()) };
	}
}

impl Linemod_Detector {
	pub fn as_raw_Linemod_Detector(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_Linemod_Detector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Linemod_Detector {}

impl crate::rgbd::Linemod_DetectorTrait for Linemod_Detector {
	fn as_raw_Linemod_Detector(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_Linemod_Detector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Linemod_Detector {
	/// \brief Empty constructor, initialize with read().
	pub fn default() -> Result<crate::rgbd::Linemod_Detector> {
		unsafe { sys::cv_linemod_Detector_Detector() }.into_result().map(|r| unsafe { crate::rgbd::Linemod_Detector::opencv_from_extern(r) } )
	}
	
	/// \brief Constructor.
	/// 
	/// \param modalities       Modalities to use (color gradients, depth normals, ...).
	/// \param T_pyramid        Value of the sampling step T at each pyramid level. The
	///                         number of pyramid levels is T_pyramid.size().
	pub fn new(modalities: &core::Vector::<core::Ptr::<dyn crate::rgbd::Linemod_Modality>>, t_pyramid: &core::Vector::<i32>) -> Result<crate::rgbd::Linemod_Detector> {
		unsafe { sys::cv_linemod_Detector_Detector_const_vector_Ptr_Modality__X_const_vector_int_X(modalities.as_raw_VectorOfPtrOfLinemod_Modality(), t_pyramid.as_raw_VectorOfi32()) }.into_result().map(|r| unsafe { crate::rgbd::Linemod_Detector::opencv_from_extern(r) } )
	}
	
}

/// \brief Discriminant feature described by its location and label.
pub trait Linemod_FeatureTrait {
	fn as_raw_Linemod_Feature(&self) -> *const c_void;
	fn as_raw_mut_Linemod_Feature(&mut self) -> *mut c_void;

	/// x offset
	fn x(&self) -> i32 {
		unsafe { sys::cv_linemod_Feature_getPropX_const(self.as_raw_Linemod_Feature()) }.into_result().expect("Infallible function failed: x")
	}
	
	/// x offset
	fn set_x(&mut self, val: i32) -> () {
		unsafe { sys::cv_linemod_Feature_setPropX_int(self.as_raw_mut_Linemod_Feature(), val) }.into_result().expect("Infallible function failed: set_x")
	}
	
	/// y offset
	fn y(&self) -> i32 {
		unsafe { sys::cv_linemod_Feature_getPropY_const(self.as_raw_Linemod_Feature()) }.into_result().expect("Infallible function failed: y")
	}
	
	/// y offset
	fn set_y(&mut self, val: i32) -> () {
		unsafe { sys::cv_linemod_Feature_setPropY_int(self.as_raw_mut_Linemod_Feature(), val) }.into_result().expect("Infallible function failed: set_y")
	}
	
	/// Quantization
	fn label(&self) -> i32 {
		unsafe { sys::cv_linemod_Feature_getPropLabel_const(self.as_raw_Linemod_Feature()) }.into_result().expect("Infallible function failed: label")
	}
	
	/// Quantization
	fn set_label(&mut self, val: i32) -> () {
		unsafe { sys::cv_linemod_Feature_setPropLabel_int(self.as_raw_mut_Linemod_Feature(), val) }.into_result().expect("Infallible function failed: set_label")
	}
	
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_linemod_Feature_read_const_FileNodeX(self.as_raw_mut_Linemod_Feature(), fn_.as_raw_FileNode()) }.into_result()
	}
	
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_linemod_Feature_write_const_FileStorageX(self.as_raw_Linemod_Feature(), fs.as_raw_mut_FileStorage()) }.into_result()
	}
	
}

/// \brief Discriminant feature described by its location and label.
pub struct Linemod_Feature {
	ptr: *mut c_void
}

opencv_type_boxed! { Linemod_Feature }

impl Drop for Linemod_Feature {
	fn drop(&mut self) {
		extern "C" { fn cv_Linemod_Feature_delete(instance: *mut c_void); }
		unsafe { cv_Linemod_Feature_delete(self.as_raw_mut_Linemod_Feature()) };
	}
}

impl Linemod_Feature {
	pub fn as_raw_Linemod_Feature(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_Linemod_Feature(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Linemod_Feature {}

impl crate::rgbd::Linemod_FeatureTrait for Linemod_Feature {
	fn as_raw_Linemod_Feature(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_Linemod_Feature(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Linemod_Feature {
	pub fn default() -> Result<crate::rgbd::Linemod_Feature> {
		unsafe { sys::cv_linemod_Feature_Feature() }.into_result().map(|r| unsafe { crate::rgbd::Linemod_Feature::opencv_from_extern(r) } )
	}
	
	pub fn new(x: i32, y: i32, label: i32) -> Result<crate::rgbd::Linemod_Feature> {
		unsafe { sys::cv_linemod_Feature_Feature_int_int_int(x, y, label) }.into_result().map(|r| unsafe { crate::rgbd::Linemod_Feature::opencv_from_extern(r) } )
	}
	
}

/// \brief Represents a successful template match.
pub trait Linemod_MatchTrait {
	fn as_raw_Linemod_Match(&self) -> *const c_void;
	fn as_raw_mut_Linemod_Match(&mut self) -> *mut c_void;

	fn x(&self) -> i32 {
		unsafe { sys::cv_linemod_Match_getPropX_const(self.as_raw_Linemod_Match()) }.into_result().expect("Infallible function failed: x")
	}
	
	fn set_x(&mut self, val: i32) -> () {
		unsafe { sys::cv_linemod_Match_setPropX_int(self.as_raw_mut_Linemod_Match(), val) }.into_result().expect("Infallible function failed: set_x")
	}
	
	fn y(&self) -> i32 {
		unsafe { sys::cv_linemod_Match_getPropY_const(self.as_raw_Linemod_Match()) }.into_result().expect("Infallible function failed: y")
	}
	
	fn set_y(&mut self, val: i32) -> () {
		unsafe { sys::cv_linemod_Match_setPropY_int(self.as_raw_mut_Linemod_Match(), val) }.into_result().expect("Infallible function failed: set_y")
	}
	
	fn similarity(&self) -> f32 {
		unsafe { sys::cv_linemod_Match_getPropSimilarity_const(self.as_raw_Linemod_Match()) }.into_result().expect("Infallible function failed: similarity")
	}
	
	fn set_similarity(&mut self, val: f32) -> () {
		unsafe { sys::cv_linemod_Match_setPropSimilarity_float(self.as_raw_mut_Linemod_Match(), val) }.into_result().expect("Infallible function failed: set_similarity")
	}
	
	fn class_id(&self) -> String {
		unsafe { sys::cv_linemod_Match_getPropClass_id_const(self.as_raw_Linemod_Match()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: class_id")
	}
	
	fn set_class_id(&mut self, val: &str) -> () {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_linemod_Match_setPropClass_id_String(self.as_raw_mut_Linemod_Match(), val.opencv_to_extern_mut()) }.into_result().expect("Infallible function failed: set_class_id")
	}
	
	fn template_id(&self) -> i32 {
		unsafe { sys::cv_linemod_Match_getPropTemplate_id_const(self.as_raw_Linemod_Match()) }.into_result().expect("Infallible function failed: template_id")
	}
	
	fn set_template_id(&mut self, val: i32) -> () {
		unsafe { sys::cv_linemod_Match_setPropTemplate_id_int(self.as_raw_mut_Linemod_Match(), val) }.into_result().expect("Infallible function failed: set_template_id")
	}
	
}

/// \brief Represents a successful template match.
pub struct Linemod_Match {
	ptr: *mut c_void
}

opencv_type_boxed! { Linemod_Match }

impl Drop for Linemod_Match {
	fn drop(&mut self) {
		extern "C" { fn cv_Linemod_Match_delete(instance: *mut c_void); }
		unsafe { cv_Linemod_Match_delete(self.as_raw_mut_Linemod_Match()) };
	}
}

impl Linemod_Match {
	pub fn as_raw_Linemod_Match(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_Linemod_Match(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Linemod_Match {}

impl crate::rgbd::Linemod_MatchTrait for Linemod_Match {
	fn as_raw_Linemod_Match(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_Linemod_Match(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Linemod_Match {
	pub fn default() -> Result<crate::rgbd::Linemod_Match> {
		unsafe { sys::cv_linemod_Match_Match() }.into_result().map(|r| unsafe { crate::rgbd::Linemod_Match::opencv_from_extern(r) } )
	}
	
	pub fn new(x: i32, y: i32, similarity: f32, class_id: &str, template_id: i32) -> Result<crate::rgbd::Linemod_Match> {
		extern_container_arg!(class_id);
		unsafe { sys::cv_linemod_Match_Match_int_int_float_const_StringX_int(x, y, similarity, class_id.opencv_to_extern(), template_id) }.into_result().map(|r| unsafe { crate::rgbd::Linemod_Match::opencv_from_extern(r) } )
	}
	
}

/// \brief Interface for modalities that plug into the LINE template matching representation.
/// 
/// \todo Max response, to allow optimization of summing (255/MAX) features as uint8
pub trait Linemod_Modality {
	fn as_raw_Linemod_Modality(&self) -> *const c_void;
	fn as_raw_mut_Linemod_Modality(&mut self) -> *mut c_void;

	/// \brief Form a quantized image pyramid from a source image.
	/// 
	/// \param[in] src  The source image. Type depends on the modality.
	/// \param[in] mask Optional mask. If not empty, unmasked pixels are set to zero
	///                in quantized image and cannot be extracted as features.
	/// 
	/// ## C++ default parameters
	/// * mask: Mat()
	fn process(&self, src: &core::Mat, mask: &core::Mat) -> Result<core::Ptr::<dyn crate::rgbd::Linemod_QuantizedPyramid>> {
		unsafe { sys::cv_linemod_Modality_process_const_const_MatX_const_MatX(self.as_raw_Linemod_Modality(), src.as_raw_Mat(), mask.as_raw_Mat()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::rgbd::Linemod_QuantizedPyramid>::opencv_from_extern(r) } )
	}
	
	fn name(&self) -> Result<String> {
		unsafe { sys::cv_linemod_Modality_name_const(self.as_raw_Linemod_Modality()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_linemod_Modality_read_const_FileNodeX(self.as_raw_mut_Linemod_Modality(), fn_.as_raw_FileNode()) }.into_result()
	}
	
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_linemod_Modality_write_const_FileStorageX(self.as_raw_Linemod_Modality(), fs.as_raw_mut_FileStorage()) }.into_result()
	}
	
}

impl dyn Linemod_Modality + '_ {
	/// \brief Create modality by name.
	/// 
	/// The following modality types are supported:
	/// - "ColorGradient"
	/// - "DepthNormal"
	pub fn create(modality_type: &str) -> Result<core::Ptr::<dyn crate::rgbd::Linemod_Modality>> {
		extern_container_arg!(modality_type);
		unsafe { sys::cv_linemod_Modality_create_const_StringX(modality_type.opencv_to_extern()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::rgbd::Linemod_Modality>::opencv_from_extern(r) } )
	}
	
	/// \brief Load a modality from file.
	pub fn create_1(fn_: &core::FileNode) -> Result<core::Ptr::<dyn crate::rgbd::Linemod_Modality>> {
		unsafe { sys::cv_linemod_Modality_create_const_FileNodeX(fn_.as_raw_FileNode()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::rgbd::Linemod_Modality>::opencv_from_extern(r) } )
	}
	
}
/// \brief Represents a modality operating over an image pyramid.
pub trait Linemod_QuantizedPyramid {
	fn as_raw_Linemod_QuantizedPyramid(&self) -> *const c_void;
	fn as_raw_mut_Linemod_QuantizedPyramid(&mut self) -> *mut c_void;

	/// \brief Compute quantized image at current pyramid level for online detection.
	/// 
	/// \param[out] dst The destination 8-bit image. For each pixel at most one bit is set,
	///                representing its classification.
	fn quantize(&self, dst: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_linemod_QuantizedPyramid_quantize_const_MatX(self.as_raw_Linemod_QuantizedPyramid(), dst.as_raw_mut_Mat()) }.into_result()
	}
	
	/// \brief Extract most discriminant features at current pyramid level to form a new template.
	/// 
	/// \param[out] templ The new template.
	fn extract_template(&self, templ: &mut crate::rgbd::Linemod_Template) -> Result<bool> {
		unsafe { sys::cv_linemod_QuantizedPyramid_extractTemplate_const_TemplateX(self.as_raw_Linemod_QuantizedPyramid(), templ.as_raw_mut_Linemod_Template()) }.into_result()
	}
	
	/// \brief Go to the next pyramid level.
	/// 
	/// \todo Allow pyramid scale factor other than 2
	fn pyr_down(&mut self) -> Result<()> {
		unsafe { sys::cv_linemod_QuantizedPyramid_pyrDown(self.as_raw_mut_Linemod_QuantizedPyramid()) }.into_result()
	}
	
}

pub trait Linemod_TemplateTrait {
	fn as_raw_Linemod_Template(&self) -> *const c_void;
	fn as_raw_mut_Linemod_Template(&mut self) -> *mut c_void;

	fn width(&self) -> i32 {
		unsafe { sys::cv_linemod_Template_getPropWidth_const(self.as_raw_Linemod_Template()) }.into_result().expect("Infallible function failed: width")
	}
	
	fn set_width(&mut self, val: i32) -> () {
		unsafe { sys::cv_linemod_Template_setPropWidth_int(self.as_raw_mut_Linemod_Template(), val) }.into_result().expect("Infallible function failed: set_width")
	}
	
	fn height(&self) -> i32 {
		unsafe { sys::cv_linemod_Template_getPropHeight_const(self.as_raw_Linemod_Template()) }.into_result().expect("Infallible function failed: height")
	}
	
	fn set_height(&mut self, val: i32) -> () {
		unsafe { sys::cv_linemod_Template_setPropHeight_int(self.as_raw_mut_Linemod_Template(), val) }.into_result().expect("Infallible function failed: set_height")
	}
	
	fn pyramid_level(&self) -> i32 {
		unsafe { sys::cv_linemod_Template_getPropPyramid_level_const(self.as_raw_Linemod_Template()) }.into_result().expect("Infallible function failed: pyramid_level")
	}
	
	fn set_pyramid_level(&mut self, val: i32) -> () {
		unsafe { sys::cv_linemod_Template_setPropPyramid_level_int(self.as_raw_mut_Linemod_Template(), val) }.into_result().expect("Infallible function failed: set_pyramid_level")
	}
	
	fn features(&mut self) -> core::Vector::<crate::rgbd::Linemod_Feature> {
		unsafe { sys::cv_linemod_Template_getPropFeatures(self.as_raw_mut_Linemod_Template()) }.into_result().map(|r| unsafe { core::Vector::<crate::rgbd::Linemod_Feature>::opencv_from_extern(r) } ).expect("Infallible function failed: features")
	}
	
	fn set_features(&mut self, mut val: core::Vector::<crate::rgbd::Linemod_Feature>) -> () {
		unsafe { sys::cv_linemod_Template_setPropFeatures_vector_Feature_(self.as_raw_mut_Linemod_Template(), val.as_raw_mut_VectorOfLinemod_Feature()) }.into_result().expect("Infallible function failed: set_features")
	}
	
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_linemod_Template_read_const_FileNodeX(self.as_raw_mut_Linemod_Template(), fn_.as_raw_FileNode()) }.into_result()
	}
	
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_linemod_Template_write_const_FileStorageX(self.as_raw_Linemod_Template(), fs.as_raw_mut_FileStorage()) }.into_result()
	}
	
}

pub struct Linemod_Template {
	ptr: *mut c_void
}

opencv_type_boxed! { Linemod_Template }

impl Drop for Linemod_Template {
	fn drop(&mut self) {
		extern "C" { fn cv_Linemod_Template_delete(instance: *mut c_void); }
		unsafe { cv_Linemod_Template_delete(self.as_raw_mut_Linemod_Template()) };
	}
}

impl Linemod_Template {
	pub fn as_raw_Linemod_Template(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_Linemod_Template(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for Linemod_Template {}

impl crate::rgbd::Linemod_TemplateTrait for Linemod_Template {
	fn as_raw_Linemod_Template(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_Linemod_Template(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Linemod_Template {
}

/// Object that can clean a noisy depth image
pub trait DepthCleanerTrait: core::AlgorithmTrait {
	fn as_raw_DepthCleaner(&self) -> *const c_void;
	fn as_raw_mut_DepthCleaner(&mut self) -> *mut c_void;

	/// Initializes some data that is cached for later computation
	/// If that function is not called, it will be called the first time normals are computed
	fn initialize(&self) -> Result<()> {
		unsafe { sys::cv_rgbd_DepthCleaner_initialize_const(self.as_raw_DepthCleaner()) }.into_result()
	}
	
	fn get_window_size(&self) -> Result<i32> {
		unsafe { sys::cv_rgbd_DepthCleaner_getWindowSize_const(self.as_raw_DepthCleaner()) }.into_result()
	}
	
	fn set_window_size(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_rgbd_DepthCleaner_setWindowSize_int(self.as_raw_mut_DepthCleaner(), val) }.into_result()
	}
	
	fn get_depth(&self) -> Result<i32> {
		unsafe { sys::cv_rgbd_DepthCleaner_getDepth_const(self.as_raw_DepthCleaner()) }.into_result()
	}
	
	fn set_depth(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_rgbd_DepthCleaner_setDepth_int(self.as_raw_mut_DepthCleaner(), val) }.into_result()
	}
	
	fn get_method(&self) -> Result<i32> {
		unsafe { sys::cv_rgbd_DepthCleaner_getMethod_const(self.as_raw_DepthCleaner()) }.into_result()
	}
	
	fn set_method(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_rgbd_DepthCleaner_setMethod_int(self.as_raw_mut_DepthCleaner(), val) }.into_result()
	}
	
}

/// Object that can clean a noisy depth image
pub struct DepthCleaner {
	ptr: *mut c_void
}

opencv_type_boxed! { DepthCleaner }

impl Drop for DepthCleaner {
	fn drop(&mut self) {
		extern "C" { fn cv_DepthCleaner_delete(instance: *mut c_void); }
		unsafe { cv_DepthCleaner_delete(self.as_raw_mut_DepthCleaner()) };
	}
}

impl DepthCleaner {
	pub fn as_raw_DepthCleaner(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_DepthCleaner(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for DepthCleaner {}

impl core::AlgorithmTrait for DepthCleaner {
	fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::DepthCleanerTrait for DepthCleaner {
	fn as_raw_DepthCleaner(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_DepthCleaner(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl DepthCleaner {
	pub fn default() -> Result<crate::rgbd::DepthCleaner> {
		unsafe { sys::cv_rgbd_DepthCleaner_DepthCleaner() }.into_result().map(|r| unsafe { crate::rgbd::DepthCleaner::opencv_from_extern(r) } )
	}
	
	/// Constructor
	/// ## Parameters
	/// * depth: the depth of the normals (only CV_32F or CV_64F)
	/// * window_size: the window size to compute the normals: can only be 1,3,5 or 7
	/// * method: one of the methods to use: RGBD_NORMALS_METHOD_SRI, RGBD_NORMALS_METHOD_FALS
	/// 
	/// ## C++ default parameters
	/// * window_size: 5
	/// * method: DEPTH_CLEANER_NIL
	pub fn new(depth: i32, window_size: i32, method: i32) -> Result<crate::rgbd::DepthCleaner> {
		unsafe { sys::cv_rgbd_DepthCleaner_DepthCleaner_int_int_int(depth, window_size, method) }.into_result().map(|r| unsafe { crate::rgbd::DepthCleaner::opencv_from_extern(r) } )
	}
	
}

/// Base class for computation of odometry.
pub trait Odometry: core::AlgorithmTrait {
	fn as_raw_Odometry(&self) -> *const c_void;
	fn as_raw_mut_Odometry(&mut self) -> *mut c_void;

	/// Method to compute a transformation from the source frame to the destination one.
	/// Some odometry algorithms do not used some data of frames (eg. ICP does not use images).
	/// In such case corresponding arguments can be set as empty Mat.
	/// The method returns true if all internal computions were possible (e.g. there were enough correspondences,
	/// system of equations has a solution, etc) and resulting transformation satisfies some test if it's provided
	/// by the Odometry inheritor implementation (e.g. thresholds for maximum translation and rotation).
	/// ## Parameters
	/// * srcImage: Image data of the source frame (CV_8UC1)
	/// * srcDepth: Depth data of the source frame (CV_32FC1, in meters)
	/// * srcMask: Mask that sets which pixels have to be used from the source frame (CV_8UC1)
	/// * dstImage: Image data of the destination frame (CV_8UC1)
	/// * dstDepth: Depth data of the destination frame (CV_32FC1, in meters)
	/// * dstMask: Mask that sets which pixels have to be used from the destination frame (CV_8UC1)
	/// * Rt: Resulting transformation from the source frame to the destination one (rigid body motion):
	///   dst_p = Rt * src_p, where dst_p is a homogeneous point in the destination frame and src_p is
	///   homogeneous point in the source frame,
	///   Rt is 4x4 matrix of CV_64FC1 type.
	/// * initRt: Initial transformation from the source frame to the destination one (optional)
	/// 
	/// ## C++ default parameters
	/// * init_rt: Mat()
	fn compute(&self, src_image: &core::Mat, src_depth: &core::Mat, src_mask: &core::Mat, dst_image: &core::Mat, dst_depth: &core::Mat, dst_mask: &core::Mat, rt: &mut core::Mat, init_rt: &core::Mat) -> Result<bool> {
		unsafe { sys::cv_rgbd_Odometry_compute_const_const_MatX_const_MatX_const_MatX_const_MatX_const_MatX_const_MatX_MatX_const_MatX(self.as_raw_Odometry(), src_image.as_raw_Mat(), src_depth.as_raw_Mat(), src_mask.as_raw_Mat(), dst_image.as_raw_Mat(), dst_depth.as_raw_Mat(), dst_mask.as_raw_Mat(), rt.as_raw_mut_Mat(), init_rt.as_raw_Mat()) }.into_result()
	}
	
	/// One more method to compute a transformation from the source frame to the destination one.
	/// It is designed to save on computing the frame data (image pyramids, normals, etc.).
	/// 
	/// ## C++ default parameters
	/// * init_rt: Mat()
	fn compute_1(&self, src_frame: &mut core::Ptr::<crate::rgbd::OdometryFrame>, dst_frame: &mut core::Ptr::<crate::rgbd::OdometryFrame>, rt: &mut core::Mat, init_rt: &core::Mat) -> Result<bool> {
		unsafe { sys::cv_rgbd_Odometry_compute_const_Ptr_OdometryFrame_X_Ptr_OdometryFrame_X_MatX_const_MatX(self.as_raw_Odometry(), src_frame.as_raw_mut_PtrOfOdometryFrame(), dst_frame.as_raw_mut_PtrOfOdometryFrame(), rt.as_raw_mut_Mat(), init_rt.as_raw_Mat()) }.into_result()
	}
	
	/// Prepare a cache for the frame. The function checks the precomputed/passed data (throws the error if this data
	/// does not satisfy) and computes all remaining cache data needed for the frame. Returned size is a resolution
	/// of the prepared frame.
	/// ## Parameters
	/// * frame: The odometry which will process the frame.
	/// * cacheType: The cache type: CACHE_SRC, CACHE_DST or CACHE_ALL.
	fn prepare_frame_cache(&self, frame: &mut core::Ptr::<crate::rgbd::OdometryFrame>, cache_type: i32) -> Result<core::Size> {
		unsafe { sys::cv_rgbd_Odometry_prepareFrameCache_const_Ptr_OdometryFrame_X_int(self.as_raw_Odometry(), frame.as_raw_mut_PtrOfOdometryFrame(), cache_type) }.into_result()
	}
	
	/// ## See also
	/// setCameraMatrix
	fn get_camera_matrix(&self) -> Result<core::Mat> {
		unsafe { sys::cv_rgbd_Odometry_getCameraMatrix_const(self.as_raw_Odometry()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// ## See also
	/// setCameraMatrix getCameraMatrix
	fn set_camera_matrix(&mut self, val: &core::Mat) -> Result<()> {
		unsafe { sys::cv_rgbd_Odometry_setCameraMatrix_const_MatX(self.as_raw_mut_Odometry(), val.as_raw_Mat()) }.into_result()
	}
	
	/// ## See also
	/// setTransformType
	fn get_transform_type(&self) -> Result<i32> {
		unsafe { sys::cv_rgbd_Odometry_getTransformType_const(self.as_raw_Odometry()) }.into_result()
	}
	
	/// ## See also
	/// setTransformType getTransformType
	fn set_transform_type(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_rgbd_Odometry_setTransformType_int(self.as_raw_mut_Odometry(), val) }.into_result()
	}
	
}

impl dyn Odometry + '_ {
	pub fn default_min_depth() -> Result<f32> {
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MIN_DEPTH() }.into_result()
	}
	
	pub fn default_max_depth() -> Result<f32> {
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_DEPTH() }.into_result()
	}
	
	pub fn default_max_depth_diff() -> Result<f32> {
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_DEPTH_DIFF() }.into_result()
	}
	
	pub fn default_max_points_part() -> Result<f32> {
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_POINTS_PART() }.into_result()
	}
	
	pub fn default_max_translation() -> Result<f32> {
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_TRANSLATION() }.into_result()
	}
	
	pub fn default_max_rotation() -> Result<f32> {
		unsafe { sys::cv_rgbd_Odometry_DEFAULT_MAX_ROTATION() }.into_result()
	}
	
	pub fn create(odometry_type: &str) -> Result<core::Ptr::<dyn crate::rgbd::Odometry>> {
		extern_container_arg!(odometry_type);
		unsafe { sys::cv_rgbd_Odometry_create_const_StringX(odometry_type.opencv_to_extern()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::rgbd::Odometry>::opencv_from_extern(r) } )
	}
	
}
/// Object that contains a frame data that is possibly needed for the Odometry.
/// It's used for the efficiency (to pass precomputed/cached data of the frame that participates
/// in the Odometry processing several times).
pub trait OdometryFrameTrait: crate::rgbd::RgbdFrameTrait {
	fn as_raw_OdometryFrame(&self) -> *const c_void;
	fn as_raw_mut_OdometryFrame(&mut self) -> *mut c_void;

	fn pyramid_image(&mut self) -> core::Vector::<core::Mat> {
		unsafe { sys::cv_rgbd_OdometryFrame_getPropPyramidImage(self.as_raw_mut_OdometryFrame()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } ).expect("Infallible function failed: pyramid_image")
	}
	
	fn set_pyramid_image(&mut self, mut val: core::Vector::<core::Mat>) -> () {
		unsafe { sys::cv_rgbd_OdometryFrame_setPropPyramidImage_vector_Mat_(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) }.into_result().expect("Infallible function failed: set_pyramid_image")
	}
	
	fn pyramid_depth(&mut self) -> core::Vector::<core::Mat> {
		unsafe { sys::cv_rgbd_OdometryFrame_getPropPyramidDepth(self.as_raw_mut_OdometryFrame()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } ).expect("Infallible function failed: pyramid_depth")
	}
	
	fn set_pyramid_depth(&mut self, mut val: core::Vector::<core::Mat>) -> () {
		unsafe { sys::cv_rgbd_OdometryFrame_setPropPyramidDepth_vector_Mat_(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) }.into_result().expect("Infallible function failed: set_pyramid_depth")
	}
	
	fn pyramid_mask(&mut self) -> core::Vector::<core::Mat> {
		unsafe { sys::cv_rgbd_OdometryFrame_getPropPyramidMask(self.as_raw_mut_OdometryFrame()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } ).expect("Infallible function failed: pyramid_mask")
	}
	
	fn set_pyramid_mask(&mut self, mut val: core::Vector::<core::Mat>) -> () {
		unsafe { sys::cv_rgbd_OdometryFrame_setPropPyramidMask_vector_Mat_(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) }.into_result().expect("Infallible function failed: set_pyramid_mask")
	}
	
	fn pyramid_cloud(&mut self) -> core::Vector::<core::Mat> {
		unsafe { sys::cv_rgbd_OdometryFrame_getPropPyramidCloud(self.as_raw_mut_OdometryFrame()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } ).expect("Infallible function failed: pyramid_cloud")
	}
	
	fn set_pyramid_cloud(&mut self, mut val: core::Vector::<core::Mat>) -> () {
		unsafe { sys::cv_rgbd_OdometryFrame_setPropPyramidCloud_vector_Mat_(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) }.into_result().expect("Infallible function failed: set_pyramid_cloud")
	}
	
	fn pyramid_d_i_dx(&mut self) -> core::Vector::<core::Mat> {
		unsafe { sys::cv_rgbd_OdometryFrame_getPropPyramid_dI_dx(self.as_raw_mut_OdometryFrame()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } ).expect("Infallible function failed: pyramid_d_i_dx")
	}
	
	fn set_pyramid_d_i_dx(&mut self, mut val: core::Vector::<core::Mat>) -> () {
		unsafe { sys::cv_rgbd_OdometryFrame_setPropPyramid_dI_dx_vector_Mat_(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) }.into_result().expect("Infallible function failed: set_pyramid_d_i_dx")
	}
	
	fn pyramid_d_i_dy(&mut self) -> core::Vector::<core::Mat> {
		unsafe { sys::cv_rgbd_OdometryFrame_getPropPyramid_dI_dy(self.as_raw_mut_OdometryFrame()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } ).expect("Infallible function failed: pyramid_d_i_dy")
	}
	
	fn set_pyramid_d_i_dy(&mut self, mut val: core::Vector::<core::Mat>) -> () {
		unsafe { sys::cv_rgbd_OdometryFrame_setPropPyramid_dI_dy_vector_Mat_(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) }.into_result().expect("Infallible function failed: set_pyramid_d_i_dy")
	}
	
	fn pyramid_textured_mask(&mut self) -> core::Vector::<core::Mat> {
		unsafe { sys::cv_rgbd_OdometryFrame_getPropPyramidTexturedMask(self.as_raw_mut_OdometryFrame()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } ).expect("Infallible function failed: pyramid_textured_mask")
	}
	
	fn set_pyramid_textured_mask(&mut self, mut val: core::Vector::<core::Mat>) -> () {
		unsafe { sys::cv_rgbd_OdometryFrame_setPropPyramidTexturedMask_vector_Mat_(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) }.into_result().expect("Infallible function failed: set_pyramid_textured_mask")
	}
	
	fn pyramid_normals(&mut self) -> core::Vector::<core::Mat> {
		unsafe { sys::cv_rgbd_OdometryFrame_getPropPyramidNormals(self.as_raw_mut_OdometryFrame()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } ).expect("Infallible function failed: pyramid_normals")
	}
	
	fn set_pyramid_normals(&mut self, mut val: core::Vector::<core::Mat>) -> () {
		unsafe { sys::cv_rgbd_OdometryFrame_setPropPyramidNormals_vector_Mat_(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) }.into_result().expect("Infallible function failed: set_pyramid_normals")
	}
	
	fn pyramid_normals_mask(&mut self) -> core::Vector::<core::Mat> {
		unsafe { sys::cv_rgbd_OdometryFrame_getPropPyramidNormalsMask(self.as_raw_mut_OdometryFrame()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } ).expect("Infallible function failed: pyramid_normals_mask")
	}
	
	fn set_pyramid_normals_mask(&mut self, mut val: core::Vector::<core::Mat>) -> () {
		unsafe { sys::cv_rgbd_OdometryFrame_setPropPyramidNormalsMask_vector_Mat_(self.as_raw_mut_OdometryFrame(), val.as_raw_mut_VectorOfMat()) }.into_result().expect("Infallible function failed: set_pyramid_normals_mask")
	}
	
	fn release(&mut self) -> Result<()> {
		unsafe { sys::cv_rgbd_OdometryFrame_release(self.as_raw_mut_OdometryFrame()) }.into_result()
	}
	
	fn release_pyramids(&mut self) -> Result<()> {
		unsafe { sys::cv_rgbd_OdometryFrame_releasePyramids(self.as_raw_mut_OdometryFrame()) }.into_result()
	}
	
}

/// Object that contains a frame data that is possibly needed for the Odometry.
/// It's used for the efficiency (to pass precomputed/cached data of the frame that participates
/// in the Odometry processing several times).
pub struct OdometryFrame {
	ptr: *mut c_void
}

opencv_type_boxed! { OdometryFrame }

impl Drop for OdometryFrame {
	fn drop(&mut self) {
		extern "C" { fn cv_OdometryFrame_delete(instance: *mut c_void); }
		unsafe { cv_OdometryFrame_delete(self.as_raw_mut_OdometryFrame()) };
	}
}

impl OdometryFrame {
	pub fn as_raw_OdometryFrame(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_OdometryFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for OdometryFrame {}

impl crate::rgbd::OdometryFrameTrait for OdometryFrame {
	fn as_raw_OdometryFrame(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_OdometryFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::RgbdFrameTrait for OdometryFrame {
	fn as_raw_RgbdFrame(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_RgbdFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl OdometryFrame {
	pub fn default() -> Result<crate::rgbd::OdometryFrame> {
		unsafe { sys::cv_rgbd_OdometryFrame_OdometryFrame() }.into_result().map(|r| unsafe { crate::rgbd::OdometryFrame::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * mask: Mat()
	/// * normals: Mat()
	/// * id: -1
	pub fn new(image: &core::Mat, depth: &core::Mat, mask: &core::Mat, normals: &core::Mat, id: i32) -> Result<crate::rgbd::OdometryFrame> {
		unsafe { sys::cv_rgbd_OdometryFrame_OdometryFrame_const_MatX_const_MatX_const_MatX_const_MatX_int(image.as_raw_Mat(), depth.as_raw_Mat(), mask.as_raw_Mat(), normals.as_raw_Mat(), id) }.into_result().map(|r| unsafe { crate::rgbd::OdometryFrame::opencv_from_extern(r) } )
	}
	
}

/// Object that contains a frame data.
pub trait RgbdFrameTrait {
	fn as_raw_RgbdFrame(&self) -> *const c_void;
	fn as_raw_mut_RgbdFrame(&mut self) -> *mut c_void;

	fn id(&self) -> i32 {
		unsafe { sys::cv_rgbd_RgbdFrame_getPropID_const(self.as_raw_RgbdFrame()) }.into_result().expect("Infallible function failed: id")
	}
	
	fn set_id(&mut self, val: i32) -> () {
		unsafe { sys::cv_rgbd_RgbdFrame_setPropID_int(self.as_raw_mut_RgbdFrame(), val) }.into_result().expect("Infallible function failed: set_id")
	}
	
	fn image(&mut self) -> core::Mat {
		unsafe { sys::cv_rgbd_RgbdFrame_getPropImage(self.as_raw_mut_RgbdFrame()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: image")
	}
	
	fn set_image(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_rgbd_RgbdFrame_setPropImage_Mat(self.as_raw_mut_RgbdFrame(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_image")
	}
	
	fn depth(&mut self) -> core::Mat {
		unsafe { sys::cv_rgbd_RgbdFrame_getPropDepth(self.as_raw_mut_RgbdFrame()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: depth")
	}
	
	fn set_depth(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_rgbd_RgbdFrame_setPropDepth_Mat(self.as_raw_mut_RgbdFrame(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_depth")
	}
	
	fn mask(&mut self) -> core::Mat {
		unsafe { sys::cv_rgbd_RgbdFrame_getPropMask(self.as_raw_mut_RgbdFrame()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: mask")
	}
	
	fn set_mask(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_rgbd_RgbdFrame_setPropMask_Mat(self.as_raw_mut_RgbdFrame(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_mask")
	}
	
	fn normals(&mut self) -> core::Mat {
		unsafe { sys::cv_rgbd_RgbdFrame_getPropNormals(self.as_raw_mut_RgbdFrame()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: normals")
	}
	
	fn set_normals(&mut self, mut val: core::Mat) -> () {
		unsafe { sys::cv_rgbd_RgbdFrame_setPropNormals_Mat(self.as_raw_mut_RgbdFrame(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_normals")
	}
	
	fn release(&mut self) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdFrame_release(self.as_raw_mut_RgbdFrame()) }.into_result()
	}
	
}

/// Object that contains a frame data.
pub struct RgbdFrame {
	ptr: *mut c_void
}

opencv_type_boxed! { RgbdFrame }

impl Drop for RgbdFrame {
	fn drop(&mut self) {
		extern "C" { fn cv_RgbdFrame_delete(instance: *mut c_void); }
		unsafe { cv_RgbdFrame_delete(self.as_raw_mut_RgbdFrame()) };
	}
}

impl RgbdFrame {
	pub fn as_raw_RgbdFrame(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_RgbdFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for RgbdFrame {}

impl crate::rgbd::RgbdFrameTrait for RgbdFrame {
	fn as_raw_RgbdFrame(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_RgbdFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RgbdFrame {
	pub fn default() -> Result<crate::rgbd::RgbdFrame> {
		unsafe { sys::cv_rgbd_RgbdFrame_RgbdFrame() }.into_result().map(|r| unsafe { crate::rgbd::RgbdFrame::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * mask: Mat()
	/// * normals: Mat()
	/// * id: -1
	pub fn new(image: &core::Mat, depth: &core::Mat, mask: &core::Mat, normals: &core::Mat, id: i32) -> Result<crate::rgbd::RgbdFrame> {
		unsafe { sys::cv_rgbd_RgbdFrame_RgbdFrame_const_MatX_const_MatX_const_MatX_const_MatX_int(image.as_raw_Mat(), depth.as_raw_Mat(), mask.as_raw_Mat(), normals.as_raw_Mat(), id) }.into_result().map(|r| unsafe { crate::rgbd::RgbdFrame::opencv_from_extern(r) } )
	}
	
}

/// Object that can compute the normals in an image.
/// It is an object as it can cache data for speed efficiency
/// The implemented methods are either:
/// - FALS (the fastest) and SRI from
/// ``Fast and Accurate Computation of Surface Normals from Range Images``
/// by H. Badino, D. Huber, Y. Park and T. Kanade
/// - the normals with bilateral filtering on a depth image from
/// ``Gradient Response Maps for Real-Time Detection of Texture-Less Objects``
/// by S. Hinterstoisser, C. Cagniart, S. Ilic, P. Sturm, N. Navab, P. Fua, and V. Lepetit
pub trait RgbdNormalsTrait: core::AlgorithmTrait {
	fn as_raw_RgbdNormals(&self) -> *const c_void;
	fn as_raw_mut_RgbdNormals(&mut self) -> *mut c_void;

	/// Initializes some data that is cached for later computation
	/// If that function is not called, it will be called the first time normals are computed
	fn initialize(&self) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdNormals_initialize_const(self.as_raw_RgbdNormals()) }.into_result()
	}
	
	fn get_rows(&self) -> Result<i32> {
		unsafe { sys::cv_rgbd_RgbdNormals_getRows_const(self.as_raw_RgbdNormals()) }.into_result()
	}
	
	fn set_rows(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdNormals_setRows_int(self.as_raw_mut_RgbdNormals(), val) }.into_result()
	}
	
	fn get_cols(&self) -> Result<i32> {
		unsafe { sys::cv_rgbd_RgbdNormals_getCols_const(self.as_raw_RgbdNormals()) }.into_result()
	}
	
	fn set_cols(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdNormals_setCols_int(self.as_raw_mut_RgbdNormals(), val) }.into_result()
	}
	
	fn get_window_size(&self) -> Result<i32> {
		unsafe { sys::cv_rgbd_RgbdNormals_getWindowSize_const(self.as_raw_RgbdNormals()) }.into_result()
	}
	
	fn set_window_size(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdNormals_setWindowSize_int(self.as_raw_mut_RgbdNormals(), val) }.into_result()
	}
	
	fn get_depth(&self) -> Result<i32> {
		unsafe { sys::cv_rgbd_RgbdNormals_getDepth_const(self.as_raw_RgbdNormals()) }.into_result()
	}
	
	fn set_depth(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdNormals_setDepth_int(self.as_raw_mut_RgbdNormals(), val) }.into_result()
	}
	
	fn get_k(&self) -> Result<core::Mat> {
		unsafe { sys::cv_rgbd_RgbdNormals_getK_const(self.as_raw_RgbdNormals()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	fn set_k(&mut self, val: &core::Mat) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdNormals_setK_const_MatX(self.as_raw_mut_RgbdNormals(), val.as_raw_Mat()) }.into_result()
	}
	
	fn get_method(&self) -> Result<i32> {
		unsafe { sys::cv_rgbd_RgbdNormals_getMethod_const(self.as_raw_RgbdNormals()) }.into_result()
	}
	
	fn set_method(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdNormals_setMethod_int(self.as_raw_mut_RgbdNormals(), val) }.into_result()
	}
	
}

/// Object that can compute the normals in an image.
/// It is an object as it can cache data for speed efficiency
/// The implemented methods are either:
/// - FALS (the fastest) and SRI from
/// ``Fast and Accurate Computation of Surface Normals from Range Images``
/// by H. Badino, D. Huber, Y. Park and T. Kanade
/// - the normals with bilateral filtering on a depth image from
/// ``Gradient Response Maps for Real-Time Detection of Texture-Less Objects``
/// by S. Hinterstoisser, C. Cagniart, S. Ilic, P. Sturm, N. Navab, P. Fua, and V. Lepetit
pub struct RgbdNormals {
	ptr: *mut c_void
}

opencv_type_boxed! { RgbdNormals }

impl Drop for RgbdNormals {
	fn drop(&mut self) {
		extern "C" { fn cv_RgbdNormals_delete(instance: *mut c_void); }
		unsafe { cv_RgbdNormals_delete(self.as_raw_mut_RgbdNormals()) };
	}
}

impl RgbdNormals {
	pub fn as_raw_RgbdNormals(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_RgbdNormals(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for RgbdNormals {}

impl core::AlgorithmTrait for RgbdNormals {
	fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::RgbdNormalsTrait for RgbdNormals {
	fn as_raw_RgbdNormals(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_RgbdNormals(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RgbdNormals {
	pub fn default() -> Result<crate::rgbd::RgbdNormals> {
		unsafe { sys::cv_rgbd_RgbdNormals_RgbdNormals() }.into_result().map(|r| unsafe { crate::rgbd::RgbdNormals::opencv_from_extern(r) } )
	}
	
	/// Constructor
	/// ## Parameters
	/// * rows: the number of rows of the depth image normals will be computed on
	/// * cols: the number of cols of the depth image normals will be computed on
	/// * depth: the depth of the normals (only CV_32F or CV_64F)
	/// * K: the calibration matrix to use
	/// * window_size: the window size to compute the normals: can only be 1,3,5 or 7
	/// * method: one of the methods to use: RGBD_NORMALS_METHOD_SRI, RGBD_NORMALS_METHOD_FALS
	/// 
	/// ## C++ default parameters
	/// * window_size: 5
	/// * method: RGBD_NORMALS_METHOD_FALS
	pub fn new(rows: i32, cols: i32, depth: i32, k: &dyn core::ToInputArray, window_size: i32, method: i32) -> Result<crate::rgbd::RgbdNormals> {
		input_array_arg!(k);
		unsafe { sys::cv_rgbd_RgbdNormals_RgbdNormals_int_int_int_const__InputArrayX_int_int(rows, cols, depth, k.as_raw__InputArray(), window_size, method) }.into_result().map(|r| unsafe { crate::rgbd::RgbdNormals::opencv_from_extern(r) } )
	}
	
}

/// Odometry based on the paper "Real-Time Visual Odometry from Dense RGB-D Images",
/// F. Steinbucker, J. Strum, D. Cremers, ICCV, 2011.
pub trait RgbdOdometryTrait: crate::rgbd::Odometry {
	fn as_raw_RgbdOdometry(&self) -> *const c_void;
	fn as_raw_mut_RgbdOdometry(&mut self) -> *mut c_void;

	fn prepare_frame_cache(&self, frame: &mut core::Ptr::<crate::rgbd::OdometryFrame>, cache_type: i32) -> Result<core::Size> {
		unsafe { sys::cv_rgbd_RgbdOdometry_prepareFrameCache_const_Ptr_OdometryFrame_X_int(self.as_raw_RgbdOdometry(), frame.as_raw_mut_PtrOfOdometryFrame(), cache_type) }.into_result()
	}
	
	fn get_camera_matrix(&self) -> Result<core::Mat> {
		unsafe { sys::cv_rgbd_RgbdOdometry_getCameraMatrix_const(self.as_raw_RgbdOdometry()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	fn set_camera_matrix(&mut self, val: &core::Mat) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdOdometry_setCameraMatrix_const_MatX(self.as_raw_mut_RgbdOdometry(), val.as_raw_Mat()) }.into_result()
	}
	
	fn get_min_depth(&self) -> Result<f64> {
		unsafe { sys::cv_rgbd_RgbdOdometry_getMinDepth_const(self.as_raw_RgbdOdometry()) }.into_result()
	}
	
	fn set_min_depth(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdOdometry_setMinDepth_double(self.as_raw_mut_RgbdOdometry(), val) }.into_result()
	}
	
	fn get_max_depth(&self) -> Result<f64> {
		unsafe { sys::cv_rgbd_RgbdOdometry_getMaxDepth_const(self.as_raw_RgbdOdometry()) }.into_result()
	}
	
	fn set_max_depth(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdOdometry_setMaxDepth_double(self.as_raw_mut_RgbdOdometry(), val) }.into_result()
	}
	
	fn get_max_depth_diff(&self) -> Result<f64> {
		unsafe { sys::cv_rgbd_RgbdOdometry_getMaxDepthDiff_const(self.as_raw_RgbdOdometry()) }.into_result()
	}
	
	fn set_max_depth_diff(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdOdometry_setMaxDepthDiff_double(self.as_raw_mut_RgbdOdometry(), val) }.into_result()
	}
	
	fn get_iteration_counts(&self) -> Result<core::Mat> {
		unsafe { sys::cv_rgbd_RgbdOdometry_getIterationCounts_const(self.as_raw_RgbdOdometry()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	fn set_iteration_counts(&mut self, val: &core::Mat) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdOdometry_setIterationCounts_const_MatX(self.as_raw_mut_RgbdOdometry(), val.as_raw_Mat()) }.into_result()
	}
	
	fn get_min_gradient_magnitudes(&self) -> Result<core::Mat> {
		unsafe { sys::cv_rgbd_RgbdOdometry_getMinGradientMagnitudes_const(self.as_raw_RgbdOdometry()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	fn set_min_gradient_magnitudes(&mut self, val: &core::Mat) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdOdometry_setMinGradientMagnitudes_const_MatX(self.as_raw_mut_RgbdOdometry(), val.as_raw_Mat()) }.into_result()
	}
	
	fn get_max_points_part(&self) -> Result<f64> {
		unsafe { sys::cv_rgbd_RgbdOdometry_getMaxPointsPart_const(self.as_raw_RgbdOdometry()) }.into_result()
	}
	
	fn set_max_points_part(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdOdometry_setMaxPointsPart_double(self.as_raw_mut_RgbdOdometry(), val) }.into_result()
	}
	
	fn get_transform_type(&self) -> Result<i32> {
		unsafe { sys::cv_rgbd_RgbdOdometry_getTransformType_const(self.as_raw_RgbdOdometry()) }.into_result()
	}
	
	fn set_transform_type(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdOdometry_setTransformType_int(self.as_raw_mut_RgbdOdometry(), val) }.into_result()
	}
	
	fn get_max_translation(&self) -> Result<f64> {
		unsafe { sys::cv_rgbd_RgbdOdometry_getMaxTranslation_const(self.as_raw_RgbdOdometry()) }.into_result()
	}
	
	fn set_max_translation(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdOdometry_setMaxTranslation_double(self.as_raw_mut_RgbdOdometry(), val) }.into_result()
	}
	
	fn get_max_rotation(&self) -> Result<f64> {
		unsafe { sys::cv_rgbd_RgbdOdometry_getMaxRotation_const(self.as_raw_RgbdOdometry()) }.into_result()
	}
	
	fn set_max_rotation(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdOdometry_setMaxRotation_double(self.as_raw_mut_RgbdOdometry(), val) }.into_result()
	}
	
}

/// Odometry based on the paper "Real-Time Visual Odometry from Dense RGB-D Images",
/// F. Steinbucker, J. Strum, D. Cremers, ICCV, 2011.
pub struct RgbdOdometry {
	ptr: *mut c_void
}

opencv_type_boxed! { RgbdOdometry }

impl Drop for RgbdOdometry {
	fn drop(&mut self) {
		extern "C" { fn cv_RgbdOdometry_delete(instance: *mut c_void); }
		unsafe { cv_RgbdOdometry_delete(self.as_raw_mut_RgbdOdometry()) };
	}
}

impl RgbdOdometry {
	pub fn as_raw_RgbdOdometry(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_RgbdOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for RgbdOdometry {}

impl core::AlgorithmTrait for RgbdOdometry {
	fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::Odometry for RgbdOdometry {
	fn as_raw_Odometry(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::RgbdOdometryTrait for RgbdOdometry {
	fn as_raw_RgbdOdometry(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_RgbdOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RgbdOdometry {
	pub fn default() -> Result<crate::rgbd::RgbdOdometry> {
		unsafe { sys::cv_rgbd_RgbdOdometry_RgbdOdometry() }.into_result().map(|r| unsafe { crate::rgbd::RgbdOdometry::opencv_from_extern(r) } )
	}
	
	/// Constructor.
	/// ## Parameters
	/// * cameraMatrix: Camera matrix
	/// * minDepth: Pixels with depth less than minDepth will not be used (in meters)
	/// * maxDepth: Pixels with depth larger than maxDepth will not be used (in meters)
	/// * maxDepthDiff: Correspondences between pixels of two given frames will be filtered out
	///                    if their depth difference is larger than maxDepthDiff (in meters)
	/// * iterCounts: Count of iterations on each pyramid level.
	/// * minGradientMagnitudes: For each pyramid level the pixels will be filtered out
	///                              if they have gradient magnitude less than minGradientMagnitudes[level].
	/// * maxPointsPart: The method uses a random pixels subset of size frameWidth x frameHeight x pointsPart
	/// * transformType: Class of transformation
	/// 
	/// ## C++ default parameters
	/// * min_depth: DEFAULT_MIN_DEPTH()
	/// * max_depth: DEFAULT_MAX_DEPTH()
	/// * max_depth_diff: DEFAULT_MAX_DEPTH_DIFF()
	/// * iter_counts: std::vector<int>()
	/// * min_gradient_magnitudes: std::vector<float>()
	/// * max_points_part: DEFAULT_MAX_POINTS_PART()
	/// * transform_type: RIGID_BODY_MOTION
	pub fn new(camera_matrix: &core::Mat, min_depth: f32, max_depth: f32, max_depth_diff: f32, iter_counts: &core::Vector::<i32>, min_gradient_magnitudes: &core::Vector::<f32>, max_points_part: f32, transform_type: i32) -> Result<crate::rgbd::RgbdOdometry> {
		unsafe { sys::cv_rgbd_RgbdOdometry_RgbdOdometry_const_MatX_float_float_float_const_vector_int_X_const_vector_float_X_float_int(camera_matrix.as_raw_Mat(), min_depth, max_depth, max_depth_diff, iter_counts.as_raw_VectorOfi32(), min_gradient_magnitudes.as_raw_VectorOff32(), max_points_part, transform_type) }.into_result().map(|r| unsafe { crate::rgbd::RgbdOdometry::opencv_from_extern(r) } )
	}
	
}

/// Object that can compute planes in an image
pub trait RgbdPlaneTrait: core::AlgorithmTrait {
	fn as_raw_RgbdPlane(&self) -> *const c_void;
	fn as_raw_mut_RgbdPlane(&mut self) -> *mut c_void;

	fn get_block_size(&self) -> Result<i32> {
		unsafe { sys::cv_rgbd_RgbdPlane_getBlockSize_const(self.as_raw_RgbdPlane()) }.into_result()
	}
	
	fn set_block_size(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdPlane_setBlockSize_int(self.as_raw_mut_RgbdPlane(), val) }.into_result()
	}
	
	fn get_min_size(&self) -> Result<i32> {
		unsafe { sys::cv_rgbd_RgbdPlane_getMinSize_const(self.as_raw_RgbdPlane()) }.into_result()
	}
	
	fn set_min_size(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdPlane_setMinSize_int(self.as_raw_mut_RgbdPlane(), val) }.into_result()
	}
	
	fn get_method(&self) -> Result<i32> {
		unsafe { sys::cv_rgbd_RgbdPlane_getMethod_const(self.as_raw_RgbdPlane()) }.into_result()
	}
	
	fn set_method(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdPlane_setMethod_int(self.as_raw_mut_RgbdPlane(), val) }.into_result()
	}
	
	fn get_threshold(&self) -> Result<f64> {
		unsafe { sys::cv_rgbd_RgbdPlane_getThreshold_const(self.as_raw_RgbdPlane()) }.into_result()
	}
	
	fn set_threshold(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdPlane_setThreshold_double(self.as_raw_mut_RgbdPlane(), val) }.into_result()
	}
	
	fn get_sensor_error_a(&self) -> Result<f64> {
		unsafe { sys::cv_rgbd_RgbdPlane_getSensorErrorA_const(self.as_raw_RgbdPlane()) }.into_result()
	}
	
	fn set_sensor_error_a(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdPlane_setSensorErrorA_double(self.as_raw_mut_RgbdPlane(), val) }.into_result()
	}
	
	fn get_sensor_error_b(&self) -> Result<f64> {
		unsafe { sys::cv_rgbd_RgbdPlane_getSensorErrorB_const(self.as_raw_RgbdPlane()) }.into_result()
	}
	
	fn set_sensor_error_b(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdPlane_setSensorErrorB_double(self.as_raw_mut_RgbdPlane(), val) }.into_result()
	}
	
	fn get_sensor_error_c(&self) -> Result<f64> {
		unsafe { sys::cv_rgbd_RgbdPlane_getSensorErrorC_const(self.as_raw_RgbdPlane()) }.into_result()
	}
	
	fn set_sensor_error_c(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_rgbd_RgbdPlane_setSensorErrorC_double(self.as_raw_mut_RgbdPlane(), val) }.into_result()
	}
	
}

/// Object that can compute planes in an image
pub struct RgbdPlane {
	ptr: *mut c_void
}

opencv_type_boxed! { RgbdPlane }

impl Drop for RgbdPlane {
	fn drop(&mut self) {
		extern "C" { fn cv_RgbdPlane_delete(instance: *mut c_void); }
		unsafe { cv_RgbdPlane_delete(self.as_raw_mut_RgbdPlane()) };
	}
}

impl RgbdPlane {
	pub fn as_raw_RgbdPlane(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_RgbdPlane(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for RgbdPlane {}

impl core::AlgorithmTrait for RgbdPlane {
	fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::RgbdPlaneTrait for RgbdPlane {
	fn as_raw_RgbdPlane(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_RgbdPlane(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RgbdPlane {
	/// ## C++ default parameters
	/// * method: RGBD_PLANE_METHOD_DEFAULT
	pub fn new(method: crate::rgbd::RgbdPlane_RGBD_PLANE_METHOD) -> Result<crate::rgbd::RgbdPlane> {
		unsafe { sys::cv_rgbd_RgbdPlane_RgbdPlane_RGBD_PLANE_METHOD(method) }.into_result().map(|r| unsafe { crate::rgbd::RgbdPlane::opencv_from_extern(r) } )
	}
	
}
