#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # 3D Visualizer
//! 
//! This section describes 3D visualization window as well as classes and methods that are used to
//! interact with it.
//! 
//! 3D visualization window (see Viz3d) is used to display widgets (see Widget), and it provides several
//! methods to interact with scene and widgets.
//!    # Widget
//! 
//! In this section, the widget framework is explained. Widgets represent 2D or 3D objects, varying from
//! simple ones such as lines to complex ones such as point clouds and meshes.
//! 
//! Widgets are **implicitly shared**. Therefore, one can add a widget to the scene, and modify the
//! widget without re-adding the widget.
//! 
//! ```ignore
//! // Create a cloud widget
//! viz::WCloud cw(cloud, viz::Color::red());
//! // Display it in a window
//! myWindow.showWidget("CloudWidget1", cw);
//! // Modify it, and it will be modified in the window.
//! cw.setColor(viz::Color::yellow());
//! ```
//! 
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::ColorTraitConst, super::ColorTrait, super::MeshTraitConst, super::MeshTrait, super::CameraTraitConst, super::CameraTrait, super::KeyboardEventTraitConst, super::KeyboardEventTrait, super::MouseEventTraitConst, super::MouseEventTrait, super::WidgetTraitConst, super::WidgetTrait, super::Widget3DTraitConst, super::Widget3DTrait, super::Widget2DTraitConst, super::Widget2DTrait, super::WLineTraitConst, super::WLineTrait, super::WPlaneTraitConst, super::WPlaneTrait, super::WSphereTraitConst, super::WSphereTrait, super::WArrowTraitConst, super::WArrowTrait, super::WCircleTraitConst, super::WCircleTrait, super::WConeTraitConst, super::WConeTrait, super::WCylinderTraitConst, super::WCylinderTrait, super::WCubeTraitConst, super::WCubeTrait, super::WPolyLineTraitConst, super::WPolyLineTrait, super::WTextTraitConst, super::WTextTrait, super::WText3DTraitConst, super::WText3DTrait, super::WImageOverlayTraitConst, super::WImageOverlayTrait, super::WImage3DTraitConst, super::WImage3DTrait, super::WCoordinateSystemTraitConst, super::WCoordinateSystemTrait, super::WGridTraitConst, super::WGridTrait, super::WCameraPositionTraitConst, super::WCameraPositionTrait, super::WTrajectoryTraitConst, super::WTrajectoryTrait, super::WTrajectoryFrustumsTraitConst, super::WTrajectoryFrustumsTrait, super::WTrajectorySpheresTraitConst, super::WTrajectorySpheresTrait, super::WCloudTraitConst, super::WCloudTrait, super::WPaintedCloudTraitConst, super::WPaintedCloudTrait, super::WCloudCollectionTraitConst, super::WCloudCollectionTrait, super::WCloudNormalsTraitConst, super::WCloudNormalsTrait, super::WMeshTraitConst, super::WMeshTrait, super::WWidgetMergerTraitConst, super::WWidgetMergerTrait, super::Viz3dTraitConst, super::Viz3dTrait };
}

pub const AMBIENT: i32 = 7;
pub const FONT_SIZE: i32 = 3;
pub const IMMEDIATE_RENDERING: i32 = 5;
pub const KeyboardEvent_ALT: i32 = 1;
pub const KeyboardEvent_CTRL: i32 = 2;
pub const KeyboardEvent_NONE: i32 = 0;
pub const KeyboardEvent_SHIFT: i32 = 4;
pub const LIGHTING: i32 = 8;
pub const LINE_WIDTH: i32 = 2;
pub const Mesh_LOAD_AUTO: i32 = 0;
pub const Mesh_LOAD_OBJ: i32 = 2;
pub const Mesh_LOAD_PLY: i32 = 1;
pub const OPACITY: i32 = 1;
pub const POINT_SIZE: i32 = 0;
pub const REPRESENTATION: i32 = 4;
pub const REPRESENTATION_POINTS: i32 = 0;
pub const REPRESENTATION_SURFACE: i32 = 2;
pub const REPRESENTATION_WIREFRAME: i32 = 1;
pub const SHADING: i32 = 6;
pub const SHADING_FLAT: i32 = 0;
pub const SHADING_GOURAUD: i32 = 1;
pub const SHADING_PHONG: i32 = 2;
pub const WTrajectory_BOTH: i32 = 3;
pub const WTrajectory_FRAMES: i32 = 1;
pub const WTrajectory_PATH: i32 = 2;
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KeyboardEvent_Action {
	KEY_UP = 0,
	KEY_DOWN = 1,
}

opencv_type_enum! { crate::viz::KeyboardEvent_Action }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MouseEvent_MouseButton {
	NoButton = 0,
	LeftButton = 1,
	MiddleButton = 2,
	RightButton = 3,
	VScroll = 4,
}

opencv_type_enum! { crate::viz::MouseEvent_MouseButton }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MouseEvent_Type {
	MouseMove = 1,
	MouseButtonPress = 2,
	MouseButtonRelease = 3,
	MouseScrollDown = 4,
	MouseScrollUp = 5,
	MouseDblClick = 6,
}

opencv_type_enum! { crate::viz::MouseEvent_Type }

/// //////////////////////////////////////////////////////////////////////////
/// Widget rendering properties
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RenderingProperties {
	POINT_SIZE = 0,
	OPACITY = 1,
	LINE_WIDTH = 2,
	FONT_SIZE = 3,
	REPRESENTATION = 4,
	IMMEDIATE_RENDERING = 5,
	SHADING = 6,
	AMBIENT = 7,
	LIGHTING = 8,
}

opencv_type_enum! { crate::viz::RenderingProperties }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RepresentationValues {
	REPRESENTATION_POINTS = 0,
	REPRESENTATION_WIREFRAME = 1,
	REPRESENTATION_SURFACE = 2,
}

opencv_type_enum! { crate::viz::RepresentationValues }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShadingValues {
	SHADING_FLAT = 0,
	SHADING_GOURAUD = 1,
	SHADING_PHONG = 2,
}

opencv_type_enum! { crate::viz::ShadingValues }

pub type Viz3d_Color = crate::viz::Color;
pub type Viz3d_KeyboardCallback = Option<Box<dyn FnMut(*const c_void) -> () + Send + Sync + 'static>>;
pub type Viz3d_MouseCallback = Option<Box<dyn FnMut(*const c_void) -> () + Send + Sync + 'static>>;
/// ////////////////////////////////////////////////////////////////////////////////////////////
/// Computing normals for mesh
/// ## Parameters
/// * mesh: Input mesh.
/// * normals: Normals at very point in the mesh of type CV_64FC3.
#[inline]
pub fn compute_normals(mesh: &crate::viz::Mesh, normals: &mut dyn core::ToOutputArray) -> Result<()> {
	output_array_arg!(normals);
	unsafe { sys::cv_viz_computeNormals_const_MeshR_const__OutputArrayR(mesh.as_raw_Mesh(), normals.as_raw__OutputArray()) }.into_result()
}

/// Retrieves a window by its name.
/// 
/// ## Parameters
/// * window_name: Name of the window that is to be retrieved.
/// 
/// This function returns a Viz3d object with the given name.
/// 
/// 
/// Note: If the window with that name already exists, that window is returned. Otherwise, new window is
/// created with the given name, and it is returned.
#[inline]
pub fn get_window_by_name(window_name: &str) -> Result<crate::viz::Viz3d> {
	extern_container_arg!(window_name);
	unsafe { sys::cv_viz_getWindowByName_const_StringR(window_name.opencv_as_extern()) }.into_result().map(|r| unsafe { crate::viz::Viz3d::opencv_from_extern(r) } )
}

/// Displays image in specified window
/// 
/// ## C++ default parameters
/// * window_size: Size(-1,-1)
#[inline]
pub fn imshow(window_name: &str, image: &dyn core::ToInputArray, window_size: core::Size) -> Result<crate::viz::Viz3d> {
	extern_container_arg!(window_name);
	input_array_arg!(image);
	unsafe { sys::cv_viz_imshow_const_StringR_const__InputArrayR_const_SizeR(window_name.opencv_as_extern(), image.as_raw__InputArray(), &window_size) }.into_result().map(|r| unsafe { crate::viz::Viz3d::opencv_from_extern(r) } )
}

/// Constructs camera pose from position, focal_point and up_vector (see gluLookAt() for more
/// information).
/// 
/// ## Parameters
/// * position: Position of the camera in global coordinate frame.
/// * focal_point: Focal point of the camera in global coordinate frame.
/// * y_dir: Up vector of the camera in global coordinate frame.
/// 
/// This function returns pose of the camera in global coordinate frame.
#[inline]
pub fn make_camera_pose(position: core::Vec3d, focal_point: core::Vec3d, y_dir: core::Vec3d) -> Result<core::Affine3d> {
	unsafe { sys::cv_viz_makeCameraPose_const_Vec3dR_const_Vec3dR_const_Vec3dR(&position, &focal_point, &y_dir) }.into_result()
}

/// Takes coordinate frame data and builds transform to global coordinate frame.
/// 
/// ## Parameters
/// * axis_x: X axis vector in global coordinate frame.
/// * axis_y: Y axis vector in global coordinate frame.
/// * axis_z: Z axis vector in global coordinate frame.
/// * origin: Origin of the coordinate frame in global coordinate frame.
/// 
/// ## Returns
/// An affine transform that describes transformation between global coordinate frame
/// and a given coordinate frame.
/// The returned transforms can transform a point in the given coordinate frame to the global
/// coordinate frame.
/// 
/// ## C++ default parameters
/// * origin: Vec3d::all(0)
#[inline]
pub fn make_transform_to_global(axis_x: core::Vec3d, axis_y: core::Vec3d, axis_z: core::Vec3d, origin: core::Vec3d) -> Result<core::Affine3d> {
	unsafe { sys::cv_viz_makeTransformToGlobal_const_Vec3dR_const_Vec3dR_const_Vec3dR_const_Vec3dR(&axis_x, &axis_y, &axis_z, &origin) }.into_result()
}

/// ## Parameters
/// * file: Filename with extension. Supported formats: PLY, XYZ, OBJ and STL.
/// * colors: Used by PLY and STL formats only.
/// * normals: Used by PLY, OBJ and STL formats only.
/// ## Returns
/// A mat containing the point coordinates with depth CV_32F or CV_64F and number of
///        channels 3 or 4 with only 1 row.
/// 
/// ## C++ default parameters
/// * colors: noArray()
/// * normals: noArray()
#[inline]
pub fn read_cloud(file: &str, colors: &mut dyn core::ToOutputArray, normals: &mut dyn core::ToOutputArray) -> Result<core::Mat> {
	extern_container_arg!(file);
	output_array_arg!(colors);
	output_array_arg!(normals);
	unsafe { sys::cv_viz_readCloud_const_StringR_const__OutputArrayR_const__OutputArrayR(file.opencv_as_extern(), colors.as_raw__OutputArray(), normals.as_raw__OutputArray()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
}

/// ////////////////////////////////////////////////////////////////////////////////////////////
/// Reads mesh. Only ply format is supported now and no texture load support
#[inline]
pub fn read_mesh(file: &str) -> Result<crate::viz::Mesh> {
	extern_container_arg!(file);
	unsafe { sys::cv_viz_readMesh_const_StringR(file.opencv_as_extern()) }.into_result().map(|r| unsafe { crate::viz::Mesh::opencv_from_extern(r) } )
}

/// ## Parameters
/// * file: Filename of type supported by cv::FileStorage.
/// * pose: Output matrix.
/// * tag: Name of the pose in the file.
/// 
/// ## C++ default parameters
/// * tag: "pose"
#[inline]
pub fn read_pose(file: &str, pose: &mut core::Affine3d, tag: &str) -> Result<bool> {
	extern_container_arg!(file);
	extern_container_arg!(tag);
	unsafe { sys::cv_viz_readPose_const_StringR_Affine3dR_const_StringR(file.opencv_as_extern(), pose, tag.opencv_as_extern()) }.into_result()
}

/// takes vector<Affine3<T>> with T = float/dobule and loads poses from sequence of files
/// 
/// ## Parameters
/// * traj: Output array containing a lists of poses. It can be
///            - std::vector<cv::Affine3f>, std::vector<cv::Affine3d>
///            - cv::Mat
/// * files_format: Format specifier string for constructing filenames.
///                    The only placeholder in the string should support `int`.
/// * start: The initial counter for files_format. It must be greater than or equal to 0.
/// * end: The final  counter for files_format.
/// * tag: Name of the matrix in the file.
/// 
/// ## C++ default parameters
/// * files_format: "pose%05d.xml"
/// * start: 0
/// * end: INT_MAX
/// * tag: "pose"
#[inline]
pub fn read_trajectory(traj: &mut dyn core::ToOutputArray, files_format: &str, start: i32, end: i32, tag: &str) -> Result<()> {
	output_array_arg!(traj);
	extern_container_arg!(files_format);
	extern_container_arg!(tag);
	unsafe { sys::cv_viz_readTrajectory_const__OutputArrayR_const_StringR_int_int_const_StringR(traj.as_raw__OutputArray(), files_format.opencv_as_extern(), start, end, tag.opencv_as_extern()) }.into_result()
}

/// Unregisters all Viz windows from internal database. After it 'getWindowByName()' will create new windows instead of getting existing from the database.
#[inline]
pub fn unregister_all_windows() -> Result<()> {
	unsafe { sys::cv_viz_unregisterAllWindows() }.into_result()
}

/// ## Parameters
/// * file: Filename with extension. Supported formats: PLY, XYZ and OBJ.
/// * cloud: Supported depths: CV_32F and CV_64F. Supported channels: 3 and 4.
/// * colors: Used by PLY format only. Supported depth: CV_8U. Supported channels: 1, 3 and 4.
/// * normals: Used by PLY and OBJ format only. Supported depths: CV_32F and CV_64F.
///                Supported channels: 3 and 4.
/// * binary: Used only for PLY format.
/// 
/// ## C++ default parameters
/// * colors: noArray()
/// * normals: noArray()
/// * binary: false
#[inline]
pub fn write_cloud(file: &str, cloud: &dyn core::ToInputArray, colors: &dyn core::ToInputArray, normals: &dyn core::ToInputArray, binary: bool) -> Result<()> {
	extern_container_arg!(file);
	input_array_arg!(cloud);
	input_array_arg!(colors);
	input_array_arg!(normals);
	unsafe { sys::cv_viz_writeCloud_const_StringR_const__InputArrayR_const__InputArrayR_const__InputArrayR_bool(file.opencv_as_extern(), cloud.as_raw__InputArray(), colors.as_raw__InputArray(), normals.as_raw__InputArray(), binary) }.into_result()
}

/// ## Parameters
/// * file: Filename.
/// * pose: Input matrix.
/// * tag: Name of the pose to be saved into the given file.
/// 
/// ## C++ default parameters
/// * tag: "pose"
#[inline]
pub fn write_pose(file: &str, pose: core::Affine3d, tag: &str) -> Result<()> {
	extern_container_arg!(file);
	extern_container_arg!(tag);
	unsafe { sys::cv_viz_writePose_const_StringR_const_Affine3dR_const_StringR(file.opencv_as_extern(), &pose, tag.opencv_as_extern()) }.into_result()
}

/// takes vector<Affine3<T>> with T = float/dobule and writes to a sequence of files with given filename format
/// ## Parameters
/// * traj: Trajectory containing a list of poses. It can be
///          - std::vector<cv::Mat>, each cv::Mat is of type CV_32F16 or CV_64FC16
///          - std::vector<cv::Affine3f>, std::vector<cv::Affine3d>
///          - cv::Mat of type CV_32FC16 OR CV_64F16
/// * files_format: Format specifier string for constructing filenames.
///                    The only placeholder in the string should support `int`.
/// * start: The initial counter for files_format.
/// * tag: Name of the matrix in the file.
/// 
/// ## C++ default parameters
/// * files_format: "pose%05d.xml"
/// * start: 0
/// * tag: "pose"
#[inline]
pub fn write_trajectory(traj: &dyn core::ToInputArray, files_format: &str, start: i32, tag: &str) -> Result<()> {
	input_array_arg!(traj);
	extern_container_arg!(files_format);
	extern_container_arg!(tag);
	unsafe { sys::cv_viz_writeTrajectory_const__InputArrayR_const_StringR_int_const_StringR(traj.as_raw__InputArray(), files_format.opencv_as_extern(), start, tag.opencv_as_extern()) }.into_result()
}

/// This class wraps intrinsic parameters of a camera.
/// 
/// It provides several constructors that can extract the intrinsic parameters from field of
/// view, intrinsic matrix and projection matrix. :
pub trait CameraTraitConst {
	fn as_raw_Camera(&self) -> *const c_void;

	#[inline]
	fn get_clip(&self) -> Result<core::Vec2d> {
		unsafe { sys::cv_viz_Camera_getClip_const(self.as_raw_Camera()) }.into_result()
	}
	
	#[inline]
	fn get_window_size(&self) -> Result<core::Size> {
		unsafe { sys::cv_viz_Camera_getWindowSize_const(self.as_raw_Camera()) }.into_result()
	}
	
	#[inline]
	fn get_fov(&self) -> Result<core::Vec2d> {
		unsafe { sys::cv_viz_Camera_getFov_const(self.as_raw_Camera()) }.into_result()
	}
	
	#[inline]
	fn get_principal_point(&self) -> Result<core::Vec2d> {
		unsafe { sys::cv_viz_Camera_getPrincipalPoint_const(self.as_raw_Camera()) }.into_result()
	}
	
	#[inline]
	fn get_focal_length(&self) -> Result<core::Vec2d> {
		unsafe { sys::cv_viz_Camera_getFocalLength_const(self.as_raw_Camera()) }.into_result()
	}
	
	/// Computes projection matrix using intrinsic parameters of the camera.
	/// 
	/// 
	/// ## Parameters
	/// * proj: Output projection matrix with the following form
	/// ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%5Cfrac%7B2n%7D%7Br%2Dl%7D%20%26%20%20%20%20%20%20%20%200%20%20%20%20%20%20%20%26%20%5Cfrac%7Br%2Bl%7D%7Br%2Dl%7D%20%20%26%200%5C%5C%0A%20%20%20%20%20%20%20%200%20%20%20%20%20%20%20%20%26%20%5Cfrac%7B2n%7D%7Bt%2Db%7D%20%26%20%5Cfrac%7Bt%2Bb%7D%7Bt%2Db%7D%20%20%26%200%5C%5C%0A%20%20%20%20%20%20%20%200%20%20%20%20%20%20%20%20%26%20%20%20%20%20%20%20%200%20%20%20%20%20%20%20%26%20%2D%5Cfrac%7Bf%2Bn%7D%7Bf%2Dn%7D%20%26%20%2D%5Cfrac%7B2fn%7D%7Bf%2Dn%7D%5C%5C%0A%20%20%20%20%20%20%20%200%20%20%20%20%20%20%20%20%26%20%20%20%20%20%20%20%200%20%20%20%20%20%20%20%26%20%2D1%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%26%200%5C%5C%0A%20%20%5Cend%7Bbmatrix%7D%0A)
	#[inline]
	fn compute_projection_matrix(&self, proj: &mut core::Matx44d) -> Result<()> {
		unsafe { sys::cv_viz_Camera_computeProjectionMatrix_const_Matx44dR(self.as_raw_Camera(), proj) }.into_result()
	}
	
}

pub trait CameraTrait: crate::viz::CameraTraitConst {
	fn as_raw_mut_Camera(&mut self) -> *mut c_void;

	#[inline]
	fn set_clip(&mut self, clip: core::Vec2d) -> Result<()> {
		unsafe { sys::cv_viz_Camera_setClip_const_Vec2dR(self.as_raw_mut_Camera(), &clip) }.into_result()
	}
	
	#[inline]
	fn set_window_size(&mut self, window_size: core::Size) -> Result<()> {
		unsafe { sys::cv_viz_Camera_setWindowSize_const_SizeR(self.as_raw_mut_Camera(), &window_size) }.into_result()
	}
	
	#[inline]
	fn set_fov(&mut self, fov: core::Vec2d) -> Result<()> {
		unsafe { sys::cv_viz_Camera_setFov_const_Vec2dR(self.as_raw_mut_Camera(), &fov) }.into_result()
	}
	
}

/// This class wraps intrinsic parameters of a camera.
/// 
/// It provides several constructors that can extract the intrinsic parameters from field of
/// view, intrinsic matrix and projection matrix. :
pub struct Camera {
	ptr: *mut c_void
}

opencv_type_boxed! { Camera }

impl Drop for Camera {
	fn drop(&mut self) {
		extern "C" { fn cv_Camera_delete(instance: *mut c_void); }
		unsafe { cv_Camera_delete(self.as_raw_mut_Camera()) };
	}
}

unsafe impl Send for Camera {}

impl crate::viz::CameraTraitConst for Camera {
	#[inline] fn as_raw_Camera(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::CameraTrait for Camera {
	#[inline] fn as_raw_mut_Camera(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Camera {
	/// Constructs a Camera.
	/// 
	/// ## Parameters
	/// * fx: Horizontal focal length.
	/// * fy: Vertical focal length.
	/// * cx: x coordinate of the principal point.
	/// * cy: y coordinate of the principal point.
	/// * window_size: Size of the window. This together with focal length and principal
	/// point determines the field of view.
	#[inline]
	pub fn new(fx: f64, fy: f64, cx: f64, cy: f64, window_size: core::Size) -> Result<crate::viz::Camera> {
		unsafe { sys::cv_viz_Camera_Camera_double_double_double_double_const_SizeR(fx, fy, cx, cy, &window_size) }.into_result().map(|r| unsafe { crate::viz::Camera::opencv_from_extern(r) } )
	}
	
	/// Constructs a Camera.
	/// 
	/// ## Parameters
	/// * fx: Horizontal focal length.
	/// * fy: Vertical focal length.
	/// * cx: x coordinate of the principal point.
	/// * cy: y coordinate of the principal point.
	/// * window_size: Size of the window. This together with focal length and principal
	/// point determines the field of view.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * fov: Field of view (horizontal, vertical) in radians
	/// * window_size: Size of the window. Principal point is at the center of the window
	///            by default.
	#[inline]
	pub fn new_1(fov: core::Vec2d, window_size: core::Size) -> Result<crate::viz::Camera> {
		unsafe { sys::cv_viz_Camera_Camera_const_Vec2dR_const_SizeR(&fov, &window_size) }.into_result().map(|r| unsafe { crate::viz::Camera::opencv_from_extern(r) } )
	}
	
	/// Constructs a Camera.
	/// 
	/// ## Parameters
	/// * fx: Horizontal focal length.
	/// * fy: Vertical focal length.
	/// * cx: x coordinate of the principal point.
	/// * cy: y coordinate of the principal point.
	/// * window_size: Size of the window. This together with focal length and principal
	/// point determines the field of view.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * K: Intrinsic matrix of the camera with the following form
	///            ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20f%5Fx%20%26%20%20%200%20%26%20c%5Fx%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%26%20%20%200%20%26%20%20%201%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20)
	/// * window_size: Size of the window. This together with intrinsic matrix determines
	///            the field of view.
	#[inline]
	pub fn new_2(k: core::Matx33d, window_size: core::Size) -> Result<crate::viz::Camera> {
		unsafe { sys::cv_viz_Camera_Camera_const_Matx33dR_const_SizeR(&k, &window_size) }.into_result().map(|r| unsafe { crate::viz::Camera::opencv_from_extern(r) } )
	}
	
	/// Constructs a Camera.
	/// 
	/// ## Parameters
	/// * fx: Horizontal focal length.
	/// * fy: Vertical focal length.
	/// * cx: x coordinate of the principal point.
	/// * cy: y coordinate of the principal point.
	/// * window_size: Size of the window. This together with focal length and principal
	/// point determines the field of view.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * proj: Projection matrix of the camera with the following form
	///            ![block formula](https://latex.codecogs.com/png.latex?%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cbegin%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cfrac%7B2n%7D%7Br%2Dl%7D%20%26%20%20%20%20%20%20%20%200%20%20%20%20%20%20%20%26%20%5Cfrac%7Br%2Bl%7D%7Br%2Dl%7D%20%20%26%200%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%20%20%20%20%20%20%20%26%20%5Cfrac%7B2n%7D%7Bt%2Db%7D%20%26%20%5Cfrac%7Bt%2Bb%7D%7Bt%2Db%7D%20%20%26%200%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%20%20%20%20%20%20%20%26%20%20%20%20%20%20%20%200%20%20%20%20%20%20%20%26%20%2D%5Cfrac%7Bf%2Bn%7D%7Bf%2Dn%7D%20%26%20%2D%5Cfrac%7B2fn%7D%7Bf%2Dn%7D%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%200%20%20%20%20%20%20%20%20%26%20%20%20%20%20%20%20%200%20%20%20%20%20%20%20%26%20%2D1%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%26%200%5C%5C%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20%5Cend%7Bbmatrix%7D%0A%20%20%20%20%20%20%20%20%20%20%20%20)
	/// 
	/// * window_size: Size of the window. This together with projection matrix determines
	///            the field of view.
	#[inline]
	pub fn new_3(proj: core::Matx44d, window_size: core::Size) -> Result<crate::viz::Camera> {
		unsafe { sys::cv_viz_Camera_Camera_const_Matx44dR_const_SizeR(&proj, &window_size) }.into_result().map(|r| unsafe { crate::viz::Camera::opencv_from_extern(r) } )
	}
	
	/// Creates a Kinect Camera with
	///   - fx = fy = 525
	///   - cx = 320
	///   - cy = 240
	/// 
	/// ## Parameters
	/// * window_size: Size of the window. This together with intrinsic matrix of a Kinect Camera
	/// determines the field of view.
	#[inline]
	pub fn kinect_camera(window_size: core::Size) -> Result<crate::viz::Camera> {
		unsafe { sys::cv_viz_Camera_KinectCamera_const_SizeR(&window_size) }.into_result().map(|r| unsafe { crate::viz::Camera::opencv_from_extern(r) } )
	}
	
}

/// This class represents color in BGR order.
pub trait ColorTraitConst {
	fn as_raw_Color(&self) -> *const c_void;

	#[inline]
	fn to_vec3b(&self) -> Result<core::Vec3b> {
		unsafe { sys::cv_viz_Color_operator_cv_Vec3b_const(self.as_raw_Color()) }.into_result()
	}
	
}

pub trait ColorTrait: crate::viz::ColorTraitConst {
	fn as_raw_mut_Color(&mut self) -> *mut c_void;

}

/// This class represents color in BGR order.
pub struct Color {
	ptr: *mut c_void
}

opencv_type_boxed! { Color }

impl Drop for Color {
	fn drop(&mut self) {
		extern "C" { fn cv_Color_delete(instance: *mut c_void); }
		unsafe { cv_Color_delete(self.as_raw_mut_Color()) };
	}
}

unsafe impl Send for Color {}

impl crate::viz::ColorTraitConst for Color {
	#[inline] fn as_raw_Color(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::ColorTrait for Color {
	#[inline] fn as_raw_mut_Color(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Color {
	/// ///////////////////////////////////////////////////////////////////////////////////////////////////
	/// cv::viz::Color
	#[inline]
	pub fn default() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_Color() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	/// The three channels will have the same value equal to gray.
	#[inline]
	pub fn new(gray: f64) -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_Color_double(gray) }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn new_1(blue: f64, green: f64, red: f64) -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_Color_double_double_double(blue, green, red) }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn new_2(color: core::Scalar) -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_Color_const_ScalarR(&color) }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn black() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_black() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn blue() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_blue() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn green() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_green() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn red() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_red() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn cyan() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_cyan() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn yellow() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_yellow() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn magenta() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_magenta() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn white() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_white() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn gray() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_gray() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn silver() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_silver() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn mlab() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_mlab() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn navy() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_navy() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn maroon() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_maroon() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn teal() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_teal() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn olive() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_olive() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn purple() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_purple() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn azure() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_azure() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn chartreuse() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_chartreuse() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn rose() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_rose() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn lime() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_lime() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn gold() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_gold() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn orange() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_orange() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn orange_red() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_orange_red() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn indigo() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_indigo() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn brown() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_brown() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn apricot() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_apricot() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn pink() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_pink() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn raspberry() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_raspberry() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn cherry() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_cherry() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn violet() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_violet() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn amethyst() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_amethyst() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn bluberry() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_bluberry() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn celestial_blue() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_celestial_blue() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn turquoise() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_turquoise() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn not_set() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_not_set() }.into_result().map(|r| unsafe { crate::viz::Color::opencv_from_extern(r) } )
	}
	
}

/// This class represents a keyboard event.
pub trait KeyboardEventTraitConst {
	fn as_raw_KeyboardEvent(&self) -> *const c_void;

	#[inline]
	fn action(&self) -> crate::viz::KeyboardEvent_Action {
		unsafe { sys::cv_viz_KeyboardEvent_getPropAction_const(self.as_raw_KeyboardEvent()) }.into_result().expect("Infallible function failed: action")
	}
	
	#[inline]
	fn symbol(&self) -> String {
		unsafe { sys::cv_viz_KeyboardEvent_getPropSymbol_const(self.as_raw_KeyboardEvent()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } ).expect("Infallible function failed: symbol")
	}
	
	#[inline]
	fn code(&self) -> u8 {
		unsafe { sys::cv_viz_KeyboardEvent_getPropCode_const(self.as_raw_KeyboardEvent()) }.into_result().expect("Infallible function failed: code")
	}
	
	#[inline]
	fn modifiers(&self) -> i32 {
		unsafe { sys::cv_viz_KeyboardEvent_getPropModifiers_const(self.as_raw_KeyboardEvent()) }.into_result().expect("Infallible function failed: modifiers")
	}
	
}

pub trait KeyboardEventTrait: crate::viz::KeyboardEventTraitConst {
	fn as_raw_mut_KeyboardEvent(&mut self) -> *mut c_void;

	#[inline]
	fn set_action(&mut self, val: crate::viz::KeyboardEvent_Action) {
		unsafe { sys::cv_viz_KeyboardEvent_setPropAction_Action(self.as_raw_mut_KeyboardEvent(), val) }.into_result().expect("Infallible function failed: set_action")
	}
	
	#[inline]
	fn set_symbol(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		unsafe { sys::cv_viz_KeyboardEvent_setPropSymbol_String(self.as_raw_mut_KeyboardEvent(), val.opencv_as_extern_mut()) }.into_result().expect("Infallible function failed: set_symbol")
	}
	
	#[inline]
	fn set_code(&mut self, val: u8) {
		unsafe { sys::cv_viz_KeyboardEvent_setPropCode_unsigned_char(self.as_raw_mut_KeyboardEvent(), val) }.into_result().expect("Infallible function failed: set_code")
	}
	
	#[inline]
	fn set_modifiers(&mut self, val: i32) {
		unsafe { sys::cv_viz_KeyboardEvent_setPropModifiers_int(self.as_raw_mut_KeyboardEvent(), val) }.into_result().expect("Infallible function failed: set_modifiers")
	}
	
}

/// This class represents a keyboard event.
pub struct KeyboardEvent {
	ptr: *mut c_void
}

opencv_type_boxed! { KeyboardEvent }

impl Drop for KeyboardEvent {
	fn drop(&mut self) {
		extern "C" { fn cv_KeyboardEvent_delete(instance: *mut c_void); }
		unsafe { cv_KeyboardEvent_delete(self.as_raw_mut_KeyboardEvent()) };
	}
}

unsafe impl Send for KeyboardEvent {}

impl crate::viz::KeyboardEventTraitConst for KeyboardEvent {
	#[inline] fn as_raw_KeyboardEvent(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::KeyboardEventTrait for KeyboardEvent {
	#[inline] fn as_raw_mut_KeyboardEvent(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl KeyboardEvent {
	/// Constructs a KeyboardEvent.
	/// 
	/// ## Parameters
	/// * action: Signals if key is pressed or released.
	/// * symbol: Name of the key.
	/// * code: Code of the key.
	/// * modifiers: Signals if alt, ctrl or shift are pressed or their combination.
	#[inline]
	pub fn new(action: crate::viz::KeyboardEvent_Action, symbol: &str, code: u8, modifiers: i32) -> Result<crate::viz::KeyboardEvent> {
		extern_container_arg!(symbol);
		unsafe { sys::cv_viz_KeyboardEvent_KeyboardEvent_Action_const_StringR_unsigned_char_int(action, symbol.opencv_as_extern(), code, modifiers) }.into_result().map(|r| unsafe { crate::viz::KeyboardEvent::opencv_from_extern(r) } )
	}
	
}

/// This class wraps mesh attributes, and it can load a mesh from a ply file. :
pub trait MeshTraitConst {
	fn as_raw_Mesh(&self) -> *const c_void;

	/// point coordinates of type CV_32FC3 or CV_64FC3 with only 1 row
	#[inline]
	fn cloud(&self) -> core::Mat {
		unsafe { sys::cv_viz_Mesh_getPropCloud_const(self.as_raw_Mesh()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: cloud")
	}
	
	/// point color of type CV_8UC3 or CV_8UC4 with only 1 row
	#[inline]
	fn colors(&self) -> core::Mat {
		unsafe { sys::cv_viz_Mesh_getPropColors_const(self.as_raw_Mesh()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: colors")
	}
	
	/// point normals of type CV_32FC3, CV_32FC4, CV_64FC3 or CV_64FC4 with only 1 row
	#[inline]
	fn normals(&self) -> core::Mat {
		unsafe { sys::cv_viz_Mesh_getPropNormals_const(self.as_raw_Mesh()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: normals")
	}
	
	/// CV_32SC1 with only 1 row
	#[inline]
	fn polygons(&self) -> core::Mat {
		unsafe { sys::cv_viz_Mesh_getPropPolygons_const(self.as_raw_Mesh()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: polygons")
	}
	
	#[inline]
	fn texture(&self) -> core::Mat {
		unsafe { sys::cv_viz_Mesh_getPropTexture_const(self.as_raw_Mesh()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: texture")
	}
	
	/// CV_32FC2 or CV_64FC2 with only 1 row
	#[inline]
	fn tcoords(&self) -> core::Mat {
		unsafe { sys::cv_viz_Mesh_getPropTcoords_const(self.as_raw_Mesh()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } ).expect("Infallible function failed: tcoords")
	}
	
}

pub trait MeshTrait: crate::viz::MeshTraitConst {
	fn as_raw_mut_Mesh(&mut self) -> *mut c_void;

	/// point coordinates of type CV_32FC3 or CV_64FC3 with only 1 row
	#[inline]
	fn set_cloud(&mut self, mut val: core::Mat) {
		unsafe { sys::cv_viz_Mesh_setPropCloud_Mat(self.as_raw_mut_Mesh(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_cloud")
	}
	
	/// point color of type CV_8UC3 or CV_8UC4 with only 1 row
	#[inline]
	fn set_colors(&mut self, mut val: core::Mat) {
		unsafe { sys::cv_viz_Mesh_setPropColors_Mat(self.as_raw_mut_Mesh(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_colors")
	}
	
	/// point normals of type CV_32FC3, CV_32FC4, CV_64FC3 or CV_64FC4 with only 1 row
	#[inline]
	fn set_normals(&mut self, mut val: core::Mat) {
		unsafe { sys::cv_viz_Mesh_setPropNormals_Mat(self.as_raw_mut_Mesh(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_normals")
	}
	
	/// CV_32SC1 with only 1 row
	#[inline]
	fn set_polygons(&mut self, mut val: core::Mat) {
		unsafe { sys::cv_viz_Mesh_setPropPolygons_Mat(self.as_raw_mut_Mesh(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_polygons")
	}
	
	#[inline]
	fn set_texture(&mut self, mut val: core::Mat) {
		unsafe { sys::cv_viz_Mesh_setPropTexture_Mat(self.as_raw_mut_Mesh(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_texture")
	}
	
	/// CV_32FC2 or CV_64FC2 with only 1 row
	#[inline]
	fn set_tcoords(&mut self, mut val: core::Mat) {
		unsafe { sys::cv_viz_Mesh_setPropTcoords_Mat(self.as_raw_mut_Mesh(), val.as_raw_mut_Mat()) }.into_result().expect("Infallible function failed: set_tcoords")
	}
	
}

/// This class wraps mesh attributes, and it can load a mesh from a ply file. :
pub struct Mesh {
	ptr: *mut c_void
}

opencv_type_boxed! { Mesh }

impl Drop for Mesh {
	fn drop(&mut self) {
		extern "C" { fn cv_Mesh_delete(instance: *mut c_void); }
		unsafe { cv_Mesh_delete(self.as_raw_mut_Mesh()) };
	}
}

unsafe impl Send for Mesh {}

impl crate::viz::MeshTraitConst for Mesh {
	#[inline] fn as_raw_Mesh(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::MeshTrait for Mesh {
	#[inline] fn as_raw_mut_Mesh(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Mesh {
	#[inline]
	pub fn default() -> Result<crate::viz::Mesh> {
		unsafe { sys::cv_viz_Mesh_Mesh() }.into_result().map(|r| unsafe { crate::viz::Mesh::opencv_from_extern(r) } )
	}
	
	/// Loads a mesh from a ply or a obj file.
	/// 
	/// ## Parameters
	/// * file: File name
	/// * type: File type (for now only PLY and OBJ are supported)
	/// 
	/// **File type** can be one of the following:
	/// *   **LOAD_PLY**
	/// *   **LOAD_OBJ**
	/// 
	/// ## C++ default parameters
	/// * typ: LOAD_PLY
	#[inline]
	pub fn load(file: &str, typ: i32) -> Result<crate::viz::Mesh> {
		extern_container_arg!(file);
		unsafe { sys::cv_viz_Mesh_load_const_StringR_int(file.opencv_as_extern(), typ) }.into_result().map(|r| unsafe { crate::viz::Mesh::opencv_from_extern(r) } )
	}
	
}

/// This class represents a mouse event.
pub trait MouseEventTraitConst {
	fn as_raw_MouseEvent(&self) -> *const c_void;

	#[inline]
	fn typ(&self) -> crate::viz::MouseEvent_Type {
		unsafe { sys::cv_viz_MouseEvent_getPropType_const(self.as_raw_MouseEvent()) }.into_result().expect("Infallible function failed: typ")
	}
	
	#[inline]
	fn button(&self) -> crate::viz::MouseEvent_MouseButton {
		unsafe { sys::cv_viz_MouseEvent_getPropButton_const(self.as_raw_MouseEvent()) }.into_result().expect("Infallible function failed: button")
	}
	
	#[inline]
	fn pointer(&self) -> core::Point {
		unsafe { sys::cv_viz_MouseEvent_getPropPointer_const(self.as_raw_MouseEvent()) }.into_result().expect("Infallible function failed: pointer")
	}
	
	#[inline]
	fn modifiers(&self) -> i32 {
		unsafe { sys::cv_viz_MouseEvent_getPropModifiers_const(self.as_raw_MouseEvent()) }.into_result().expect("Infallible function failed: modifiers")
	}
	
}

pub trait MouseEventTrait: crate::viz::MouseEventTraitConst {
	fn as_raw_mut_MouseEvent(&mut self) -> *mut c_void;

	#[inline]
	fn set_type(&mut self, val: crate::viz::MouseEvent_Type) {
		unsafe { sys::cv_viz_MouseEvent_setPropType_Type(self.as_raw_mut_MouseEvent(), val) }.into_result().expect("Infallible function failed: set_type")
	}
	
	#[inline]
	fn set_button(&mut self, val: crate::viz::MouseEvent_MouseButton) {
		unsafe { sys::cv_viz_MouseEvent_setPropButton_MouseButton(self.as_raw_mut_MouseEvent(), val) }.into_result().expect("Infallible function failed: set_button")
	}
	
	#[inline]
	fn set_pointer(&mut self, val: core::Point) {
		unsafe { sys::cv_viz_MouseEvent_setPropPointer_Point(self.as_raw_mut_MouseEvent(), val.opencv_as_extern()) }.into_result().expect("Infallible function failed: set_pointer")
	}
	
	#[inline]
	fn set_modifiers(&mut self, val: i32) {
		unsafe { sys::cv_viz_MouseEvent_setPropModifiers_int(self.as_raw_mut_MouseEvent(), val) }.into_result().expect("Infallible function failed: set_modifiers")
	}
	
}

/// This class represents a mouse event.
pub struct MouseEvent {
	ptr: *mut c_void
}

opencv_type_boxed! { MouseEvent }

impl Drop for MouseEvent {
	fn drop(&mut self) {
		extern "C" { fn cv_MouseEvent_delete(instance: *mut c_void); }
		unsafe { cv_MouseEvent_delete(self.as_raw_mut_MouseEvent()) };
	}
}

unsafe impl Send for MouseEvent {}

impl crate::viz::MouseEventTraitConst for MouseEvent {
	#[inline] fn as_raw_MouseEvent(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::MouseEventTrait for MouseEvent {
	#[inline] fn as_raw_mut_MouseEvent(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MouseEvent {
	/// Constructs a MouseEvent.
	/// 
	/// ## Parameters
	/// * type: Type of the event. This can be **MouseMove**, **MouseButtonPress**,
	/// **MouseButtonRelease**, **MouseScrollDown**, **MouseScrollUp**, **MouseDblClick**.
	/// * button: Mouse button. This can be **NoButton**, **LeftButton**, **MiddleButton**,
	/// **RightButton**, **VScroll**.
	/// * pointer: Position of the event.
	/// * modifiers: Signals if alt, ctrl or shift are pressed or their combination.
	#[inline]
	pub fn new(typ: crate::viz::MouseEvent_Type, button: crate::viz::MouseEvent_MouseButton, pointer: core::Point, modifiers: i32) -> Result<crate::viz::MouseEvent> {
		unsafe { sys::cv_viz_MouseEvent_MouseEvent_const_TypeR_const_MouseButtonR_const_PointR_int(&typ, &button, &pointer, modifiers) }.into_result().map(|r| unsafe { crate::viz::MouseEvent::opencv_from_extern(r) } )
	}
	
}

/// The Viz3d class represents a 3D visualizer window. This class is implicitly shared.
pub trait Viz3dTraitConst {
	fn as_raw_Viz3d(&self) -> *const c_void;

	/// Retrieves a widget from the window.
	/// 
	/// A widget is implicitly shared; that is, if the returned widget is modified, the changes
	/// will be immediately visible in the window.
	/// 
	/// ## Parameters
	/// * id: The id of the widget that will be returned.
	#[inline]
	fn get_widget(&self, id: &str) -> Result<crate::viz::Widget> {
		extern_container_arg!(id);
		unsafe { sys::cv_viz_Viz3d_getWidget_const_const_StringR(self.as_raw_Viz3d(), id.opencv_as_extern()) }.into_result().map(|r| unsafe { crate::viz::Widget::opencv_from_extern(r) } )
	}
	
	/// Returns the current pose of a widget in the window.
	/// 
	/// ## Parameters
	/// * id: The id of the widget whose pose will be returned.
	#[inline]
	fn get_widget_pose(&self, id: &str) -> Result<core::Affine3d> {
		extern_container_arg!(id);
		unsafe { sys::cv_viz_Viz3d_getWidgetPose_const_const_StringR(self.as_raw_Viz3d(), id.opencv_as_extern()) }.into_result()
	}
	
	/// Returns a camera object that contains intrinsic parameters of the current viewer.
	#[inline]
	fn get_camera(&self) -> Result<crate::viz::Camera> {
		unsafe { sys::cv_viz_Viz3d_getCamera_const(self.as_raw_Viz3d()) }.into_result().map(|r| unsafe { crate::viz::Camera::opencv_from_extern(r) } )
	}
	
	/// Returns the current pose of the viewer.
	#[inline]
	fn get_viewer_pose(&self) -> Result<core::Affine3d> {
		unsafe { sys::cv_viz_Viz3d_getViewerPose_const(self.as_raw_Viz3d()) }.into_result()
	}
	
	/// Returns the current size of the window.
	#[inline]
	fn get_window_size(&self) -> Result<core::Size> {
		unsafe { sys::cv_viz_Viz3d_getWindowSize_const(self.as_raw_Viz3d()) }.into_result()
	}
	
	/// Returns the name of the window which has been set in the constructor.
	/// `Viz - ` is prepended to the name if necessary.
	#[inline]
	fn get_window_name(&self) -> Result<String> {
		unsafe { sys::cv_viz_Viz3d_getWindowName_const(self.as_raw_Viz3d()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
	/// Returns the Mat screenshot of the current scene.
	#[inline]
	fn get_screenshot(&self) -> Result<core::Mat> {
		unsafe { sys::cv_viz_Viz3d_getScreenshot_const(self.as_raw_Viz3d()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	/// Returns whether the event loop has been stopped.
	#[inline]
	fn was_stopped(&self) -> Result<bool> {
		unsafe { sys::cv_viz_Viz3d_wasStopped_const(self.as_raw_Viz3d()) }.into_result()
	}
	
}

pub trait Viz3dTrait: crate::viz::Viz3dTraitConst {
	fn as_raw_mut_Viz3d(&mut self) -> *mut c_void;

	/// Shows a widget in the window.
	/// 
	/// ## Parameters
	/// * id: A unique id for the widget. @param widget The widget to be displayed in the window.
	/// * pose: Pose of the widget.
	/// 
	/// ## C++ default parameters
	/// * pose: Affine3d::Identity()
	#[inline]
	fn show_widget(&mut self, id: &str, widget: &crate::viz::Widget, pose: core::Affine3d) -> Result<()> {
		extern_container_arg!(id);
		unsafe { sys::cv_viz_Viz3d_showWidget_const_StringR_const_WidgetR_const_Affine3dR(self.as_raw_mut_Viz3d(), id.opencv_as_extern(), widget.as_raw_Widget(), &pose) }.into_result()
	}
	
	/// Removes a widget from the window.
	/// 
	/// ## Parameters
	/// * id: The id of the widget that will be removed.
	#[inline]
	fn remove_widget(&mut self, id: &str) -> Result<()> {
		extern_container_arg!(id);
		unsafe { sys::cv_viz_Viz3d_removeWidget_const_StringR(self.as_raw_mut_Viz3d(), id.opencv_as_extern()) }.into_result()
	}
	
	/// Removes all widgets from the window.
	#[inline]
	fn remove_all_widgets(&mut self) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_removeAllWidgets(self.as_raw_mut_Viz3d()) }.into_result()
	}
	
	/// Removed all widgets and displays image scaled to whole window area.
	/// 
	/// ## Parameters
	/// * image: Image to be displayed.
	/// * window_size: Size of Viz3d window. Default value means no change.
	/// 
	/// ## C++ default parameters
	/// * window_size: Size(-1,-1)
	#[inline]
	fn show_image(&mut self, image: &dyn core::ToInputArray, window_size: core::Size) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_viz_Viz3d_showImage_const__InputArrayR_const_SizeR(self.as_raw_mut_Viz3d(), image.as_raw__InputArray(), &window_size) }.into_result()
	}
	
	/// Sets pose of a widget in the window.
	/// 
	/// ## Parameters
	/// * id: The id of the widget whose pose will be set. @param pose The new pose of the widget.
	#[inline]
	fn set_widget_pose(&mut self, id: &str, pose: core::Affine3d) -> Result<()> {
		extern_container_arg!(id);
		unsafe { sys::cv_viz_Viz3d_setWidgetPose_const_StringR_const_Affine3dR(self.as_raw_mut_Viz3d(), id.opencv_as_extern(), &pose) }.into_result()
	}
	
	/// Updates pose of a widget in the window by pre-multiplying its current pose.
	/// 
	/// ## Parameters
	/// * id: The id of the widget whose pose will be updated. @param pose The pose that the current
	/// pose of the widget will be pre-multiplied by.
	#[inline]
	fn update_widget_pose(&mut self, id: &str, pose: core::Affine3d) -> Result<()> {
		extern_container_arg!(id);
		unsafe { sys::cv_viz_Viz3d_updateWidgetPose_const_StringR_const_Affine3dR(self.as_raw_mut_Viz3d(), id.opencv_as_extern(), &pose) }.into_result()
	}
	
	/// Sets the intrinsic parameters of the viewer using Camera.
	/// 
	/// ## Parameters
	/// * camera: Camera object wrapping intrinsic parameters.
	#[inline]
	fn set_camera(&mut self, camera: &crate::viz::Camera) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_setCamera_const_CameraR(self.as_raw_mut_Viz3d(), camera.as_raw_Camera()) }.into_result()
	}
	
	/// Sets pose of the viewer.
	/// 
	/// ## Parameters
	/// * pose: The new pose of the viewer.
	#[inline]
	fn set_viewer_pose(&mut self, pose: core::Affine3d) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_setViewerPose_const_Affine3dR(self.as_raw_mut_Viz3d(), &pose) }.into_result()
	}
	
	/// Resets camera viewpoint to a 3D widget in the scene.
	/// 
	/// ## Parameters
	/// * id: Id of a 3D widget.
	#[inline]
	fn reset_camera_viewpoint(&mut self, id: &str) -> Result<()> {
		extern_container_arg!(id);
		unsafe { sys::cv_viz_Viz3d_resetCameraViewpoint_const_StringR(self.as_raw_mut_Viz3d(), id.opencv_as_extern()) }.into_result()
	}
	
	/// Resets camera.
	#[inline]
	fn reset_camera(&mut self) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_resetCamera(self.as_raw_mut_Viz3d()) }.into_result()
	}
	
	/// Transforms a point in world coordinate system to window coordinate system.
	/// 
	/// ## Parameters
	/// * pt: Point in world coordinate system.
	/// * window_coord: Output point in window coordinate system.
	#[inline]
	fn convert_to_window_coordinates(&mut self, pt: core::Point3d, window_coord: &mut core::Point3d) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_convertToWindowCoordinates_const_Point3dR_Point3dR(self.as_raw_mut_Viz3d(), &pt, window_coord) }.into_result()
	}
	
	/// Transforms a point in window coordinate system to a 3D ray in world coordinate system.
	/// 
	/// ## Parameters
	/// * window_coord: Point in window coordinate system. @param origin Output origin of the ray.
	/// * direction: Output direction of the ray.
	#[inline]
	fn conver_to3_d_ray(&mut self, window_coord: core::Point3d, origin: &mut core::Point3d, direction: &mut core::Vec3d) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_converTo3DRay_const_Point3dR_Point3dR_Vec3dR(self.as_raw_mut_Viz3d(), &window_coord, origin, direction) }.into_result()
	}
	
	/// Sets the size of the window.
	/// 
	/// ## Parameters
	/// * window_size: New size of the window.
	#[inline]
	fn set_window_size(&mut self, window_size: core::Size) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_setWindowSize_const_SizeR(self.as_raw_mut_Viz3d(), &window_size) }.into_result()
	}
	
	/// Saves screenshot of the current scene.
	/// 
	/// ## Parameters
	/// * file: Name of the file.
	#[inline]
	fn save_screenshot(&mut self, file: &str) -> Result<()> {
		extern_container_arg!(file);
		unsafe { sys::cv_viz_Viz3d_saveScreenshot_const_StringR(self.as_raw_mut_Viz3d(), file.opencv_as_extern()) }.into_result()
	}
	
	/// Sets the position of the window in the screen.
	/// 
	/// ## Parameters
	/// * window_position: coordinates of the window
	#[inline]
	fn set_window_position(&mut self, window_position: core::Point) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_setWindowPosition_const_PointR(self.as_raw_mut_Viz3d(), &window_position) }.into_result()
	}
	
	/// Sets or unsets full-screen rendering mode.
	/// 
	/// ## Parameters
	/// * mode: If true, window will use full-screen mode.
	/// 
	/// ## C++ default parameters
	/// * mode: true
	#[inline]
	fn set_full_screen(&mut self, mode: bool) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_setFullScreen_bool(self.as_raw_mut_Viz3d(), mode) }.into_result()
	}
	
	/// Sets background color.
	/// 
	/// ## C++ default parameters
	/// * color: Color::black()
	/// * color2: Color::not_set()
	#[inline]
	fn set_background_color(&mut self, color: &crate::viz::Viz3d_Color, color2: &crate::viz::Viz3d_Color) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_setBackgroundColor_const_ColorR_const_ColorR(self.as_raw_mut_Viz3d(), color.as_raw_Color(), color2.as_raw_Color()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * image: noArray()
	#[inline]
	fn set_background_texture(&mut self, image: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_viz_Viz3d_setBackgroundTexture_const__InputArrayR(self.as_raw_mut_Viz3d(), image.as_raw__InputArray()) }.into_result()
	}
	
	#[inline]
	fn set_background_mesh_lab(&mut self) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_setBackgroundMeshLab(self.as_raw_mut_Viz3d()) }.into_result()
	}
	
	/// The window renders and starts the event loop.
	#[inline]
	fn spin(&mut self) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_spin(self.as_raw_mut_Viz3d()) }.into_result()
	}
	
	/// Starts the event loop for a given time.
	/// 
	/// ## Parameters
	/// * time: Amount of time in milliseconds for the event loop to keep running.
	/// * force_redraw: If true, window renders.
	/// 
	/// ## C++ default parameters
	/// * time: 1
	/// * force_redraw: false
	#[inline]
	fn spin_once(&mut self, time: i32, force_redraw: bool) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_spinOnce_int_bool(self.as_raw_mut_Viz3d(), time, force_redraw) }.into_result()
	}
	
	/// Create a window in memory instead of on the screen.
	#[inline]
	fn set_off_screen_rendering(&mut self) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_setOffScreenRendering(self.as_raw_mut_Viz3d()) }.into_result()
	}
	
	/// Remove all lights from the current scene.
	#[inline]
	fn remove_all_lights(&mut self) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_removeAllLights(self.as_raw_mut_Viz3d()) }.into_result()
	}
	
	/// Add a light in the scene.
	/// 
	/// ## Parameters
	/// * position: The position of the light.
	/// * focalPoint: The point at which the light is shining
	/// * color: The color of the light
	/// * diffuseColor: The diffuse color of the light
	/// * ambientColor: The ambient color of the light
	/// * specularColor: The specular color of the light
	/// 
	/// ## C++ default parameters
	/// * focal_point: Vec3d(0,0,0)
	/// * color: Color::white()
	/// * diffuse_color: Color::white()
	/// * ambient_color: Color::black()
	/// * specular_color: Color::white()
	#[inline]
	fn add_light(&mut self, position: core::Vec3d, focal_point: core::Vec3d, color: &crate::viz::Viz3d_Color, diffuse_color: &crate::viz::Viz3d_Color, ambient_color: &crate::viz::Viz3d_Color, specular_color: &crate::viz::Viz3d_Color) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_addLight_const_Vec3dR_const_Vec3dR_const_ColorR_const_ColorR_const_ColorR_const_ColorR(self.as_raw_mut_Viz3d(), &position, &focal_point, color.as_raw_Color(), diffuse_color.as_raw_Color(), ambient_color.as_raw_Color(), specular_color.as_raw_Color()) }.into_result()
	}
	
	#[inline]
	fn close(&mut self) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_close(self.as_raw_mut_Viz3d()) }.into_result()
	}
	
	/// Sets keyboard handler.
	/// 
	/// ## Parameters
	/// * callback: Keyboard callback (void (\*KeyboardCallbackFunction(const
	/// KeyboardEvent&, void\*)).
	/// * cookie: The optional parameter passed to the callback.
	/// 
	/// ## C++ default parameters
	/// * cookie: 0
	#[inline]
	fn register_keyboard_callback(&mut self, callback: crate::viz::Viz3d_KeyboardCallback) -> Result<()> {
		callback_arg!(callback_trampoline(unnamed: *const c_void, unnamed_1: *mut c_void) -> () => unnamed_1 in callbacks => callback(unnamed: *const c_void) -> ());
		userdata_arg!(cookie in callbacks => callback);
		unsafe { sys::cv_viz_Viz3d_registerKeyboardCallback_KeyboardCallback_voidX(self.as_raw_mut_Viz3d(), callback_trampoline, cookie) }.into_result()
	}
	
	/// Sets mouse handler.
	/// 
	/// ## Parameters
	/// * callback: Mouse callback (void (\*MouseCallback)(const MouseEvent&, void\*)).
	/// * cookie: The optional parameter passed to the callback.
	/// 
	/// ## C++ default parameters
	/// * cookie: 0
	#[inline]
	fn register_mouse_callback(&mut self, callback: crate::viz::Viz3d_MouseCallback) -> Result<()> {
		callback_arg!(callback_trampoline(unnamed: *const c_void, unnamed_1: *mut c_void) -> () => unnamed_1 in callbacks => callback(unnamed: *const c_void) -> ());
		userdata_arg!(cookie in callbacks => callback);
		unsafe { sys::cv_viz_Viz3d_registerMouseCallback_MouseCallback_voidX(self.as_raw_mut_Viz3d(), callback_trampoline, cookie) }.into_result()
	}
	
	/// Sets rendering property of a widget.
	/// 
	/// ## Parameters
	/// * id: Id of the widget.
	/// * property: Property that will be modified.
	/// * value: The new value of the property.
	/// 
	/// Rendering property can be one of the following:
	/// *   **POINT_SIZE**
	/// *   **OPACITY**
	/// *   **LINE_WIDTH**
	/// *   **FONT_SIZE**
	/// 
	/// REPRESENTATION: Expected values are
	/// *   **REPRESENTATION_POINTS**
	/// *   **REPRESENTATION_WIREFRAME**
	/// *   **REPRESENTATION_SURFACE**
	/// 
	/// IMMEDIATE_RENDERING:
	/// *   Turn on immediate rendering by setting the value to 1.
	/// *   Turn off immediate rendering by setting the value to 0.
	/// 
	/// SHADING: Expected values are
	/// *   **SHADING_FLAT**
	/// *   **SHADING_GOURAUD**
	/// *   **SHADING_PHONG**
	#[inline]
	fn set_rendering_property(&mut self, id: &str, property: i32, value: f64) -> Result<()> {
		extern_container_arg!(id);
		unsafe { sys::cv_viz_Viz3d_setRenderingProperty_const_StringR_int_double(self.as_raw_mut_Viz3d(), id.opencv_as_extern(), property, value) }.into_result()
	}
	
	/// Returns rendering property of a widget.
	/// 
	/// ## Parameters
	/// * id: Id of the widget.
	/// * property: Property.
	/// 
	/// Rendering property can be one of the following:
	/// *   **POINT_SIZE**
	/// *   **OPACITY**
	/// *   **LINE_WIDTH**
	/// *   **FONT_SIZE**
	/// 
	/// REPRESENTATION: Expected values are
	/// *   **REPRESENTATION_POINTS**
	/// *   **REPRESENTATION_WIREFRAME**
	/// *   **REPRESENTATION_SURFACE**
	/// 
	/// IMMEDIATE_RENDERING:
	/// *   Turn on immediate rendering by setting the value to 1.
	/// *   Turn off immediate rendering by setting the value to 0.
	/// 
	/// SHADING: Expected values are
	/// *   **SHADING_FLAT**
	/// *   **SHADING_GOURAUD**
	/// *   **SHADING_PHONG**
	#[inline]
	fn get_rendering_property(&mut self, id: &str, property: i32) -> Result<f64> {
		extern_container_arg!(id);
		unsafe { sys::cv_viz_Viz3d_getRenderingProperty_const_StringR_int(self.as_raw_mut_Viz3d(), id.opencv_as_extern(), property) }.into_result()
	}
	
	/// Sets geometry representation of the widgets to surface, wireframe or points.
	/// 
	/// ## Parameters
	/// * representation: Geometry representation which can be one of the following:
	/// *   **REPRESENTATION_POINTS**
	/// *   **REPRESENTATION_WIREFRAME**
	/// *   **REPRESENTATION_SURFACE**
	#[inline]
	fn set_representation(&mut self, representation: i32) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_setRepresentation_int(self.as_raw_mut_Viz3d(), representation) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * enabled: false
	#[inline]
	fn set_global_warnings(&mut self, enabled: bool) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_setGlobalWarnings_bool(self.as_raw_mut_Viz3d(), enabled) }.into_result()
	}
	
}

/// The Viz3d class represents a 3D visualizer window. This class is implicitly shared.
pub struct Viz3d {
	ptr: *mut c_void
}

opencv_type_boxed! { Viz3d }

impl Drop for Viz3d {
	fn drop(&mut self) {
		extern "C" { fn cv_Viz3d_delete(instance: *mut c_void); }
		unsafe { cv_Viz3d_delete(self.as_raw_mut_Viz3d()) };
	}
}

unsafe impl Send for Viz3d {}

impl crate::viz::Viz3dTraitConst for Viz3d {
	#[inline] fn as_raw_Viz3d(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Viz3dTrait for Viz3d {
	#[inline] fn as_raw_mut_Viz3d(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Viz3d {
	/// The constructors.
	/// 
	/// ## Parameters
	/// * window_name: Name of the window.
	/// 
	/// ## C++ default parameters
	/// * window_name: String()
	#[inline]
	pub fn new(window_name: &str) -> Result<crate::viz::Viz3d> {
		extern_container_arg!(window_name);
		unsafe { sys::cv_viz_Viz3d_Viz3d_const_StringR(window_name.opencv_as_extern()) }.into_result().map(|r| unsafe { crate::viz::Viz3d::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn copy(unnamed: &crate::viz::Viz3d) -> Result<crate::viz::Viz3d> {
		unsafe { sys::cv_viz_Viz3d_Viz3d_const_Viz3dR(unnamed.as_raw_Viz3d()) }.into_result().map(|r| unsafe { crate::viz::Viz3d::opencv_from_extern(r) } )
	}
	
}

/// This 3D Widget defines an arrow.
pub trait WArrowTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WArrow(&self) -> *const c_void;

}

pub trait WArrowTrait: crate::viz::WArrowTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WArrow(&mut self) -> *mut c_void;

}

/// This 3D Widget defines an arrow.
pub struct WArrow {
	ptr: *mut c_void
}

opencv_type_boxed! { WArrow }

impl Drop for WArrow {
	fn drop(&mut self) {
		extern "C" { fn cv_WArrow_delete(instance: *mut c_void); }
		unsafe { cv_WArrow_delete(self.as_raw_mut_WArrow()) };
	}
}

unsafe impl Send for WArrow {}

impl crate::viz::WidgetTraitConst for WArrow {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WArrow {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WArrow {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WArrow {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WArrowTraitConst for WArrow {
	#[inline] fn as_raw_WArrow(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WArrowTrait for WArrow {
	#[inline] fn as_raw_mut_WArrow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WArrow {
	/// Constructs an WArrow.
	/// 
	/// ## Parameters
	/// * pt1: Start point of the arrow.
	/// * pt2: End point of the arrow.
	/// * thickness: Thickness of the arrow. Thickness of arrow head is also adjusted
	/// accordingly.
	/// * color: Color of the arrow.
	/// 
	/// Arrow head is located at the end point of the arrow.
	/// 
	/// ## C++ default parameters
	/// * thickness: 0.03
	/// * color: Color::white()
	#[inline]
	pub fn new(pt1: core::Point3d, pt2: core::Point3d, thickness: f64, color: &crate::viz::Color) -> Result<crate::viz::WArrow> {
		unsafe { sys::cv_viz_WArrow_WArrow_const_Point3dR_const_Point3dR_double_const_ColorR(&pt1, &pt2, thickness, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WArrow::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WArrow, crate::viz::Widget, cv_WArrow_to_Widget }

boxed_cast_base! { WArrow, crate::viz::Widget3D, cv_WArrow_to_Widget3D }

/// This 3D Widget represents camera position in a scene by its axes or viewing frustum. :
pub trait WCameraPositionTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WCameraPosition(&self) -> *const c_void;

}

pub trait WCameraPositionTrait: crate::viz::WCameraPositionTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WCameraPosition(&mut self) -> *mut c_void;

}

/// This 3D Widget represents camera position in a scene by its axes or viewing frustum. :
pub struct WCameraPosition {
	ptr: *mut c_void
}

opencv_type_boxed! { WCameraPosition }

impl Drop for WCameraPosition {
	fn drop(&mut self) {
		extern "C" { fn cv_WCameraPosition_delete(instance: *mut c_void); }
		unsafe { cv_WCameraPosition_delete(self.as_raw_mut_WCameraPosition()) };
	}
}

unsafe impl Send for WCameraPosition {}

impl crate::viz::WidgetTraitConst for WCameraPosition {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WCameraPosition {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WCameraPosition {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WCameraPosition {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WCameraPositionTraitConst for WCameraPosition {
	#[inline] fn as_raw_WCameraPosition(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WCameraPositionTrait for WCameraPosition {
	#[inline] fn as_raw_mut_WCameraPosition(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WCameraPosition {
	/// Creates camera coordinate frame at the origin.
	/// 
	/// ![Camera coordinate frame](https://docs.opencv.org/4.5.4/cpw1.png)
	/// 
	/// ## C++ default parameters
	/// * scale: 1.0
	#[inline]
	pub fn new(scale: f64) -> Result<crate::viz::WCameraPosition> {
		unsafe { sys::cv_viz_WCameraPosition_WCameraPosition_double(scale) }.into_result().map(|r| unsafe { crate::viz::WCameraPosition::opencv_from_extern(r) } )
	}
	
	/// Display the viewing frustum
	/// ## Parameters
	/// * K: Intrinsic matrix of the camera.
	/// * scale: Scale of the frustum.
	/// * color: Color of the frustum.
	/// 
	/// Creates viewing frustum of the camera based on its intrinsic matrix K.
	/// 
	/// ![Camera viewing frustum](https://docs.opencv.org/4.5.4/cpw2.png)
	/// 
	/// ## C++ default parameters
	/// * scale: 1.0
	/// * color: Color::white()
	#[inline]
	pub fn new_1(k: core::Matx33d, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WCameraPosition> {
		unsafe { sys::cv_viz_WCameraPosition_WCameraPosition_const_Matx33dR_double_const_ColorR(&k, scale, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WCameraPosition::opencv_from_extern(r) } )
	}
	
	/// Display the viewing frustum
	/// ## Parameters
	/// * fov: Field of view of the camera (horizontal, vertical).
	/// * scale: Scale of the frustum.
	/// * color: Color of the frustum.
	/// 
	/// Creates viewing frustum of the camera based on its field of view fov.
	/// 
	/// ![Camera viewing frustum](https://docs.opencv.org/4.5.4/cpw2.png)
	/// 
	/// ## C++ default parameters
	/// * scale: 1.0
	/// * color: Color::white()
	#[inline]
	pub fn new_2(fov: core::Vec2d, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WCameraPosition> {
		unsafe { sys::cv_viz_WCameraPosition_WCameraPosition_const_Vec2dR_double_const_ColorR(&fov, scale, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WCameraPosition::opencv_from_extern(r) } )
	}
	
	/// Display image on the far plane of the viewing frustum
	/// 
	/// ## Parameters
	/// * K: Intrinsic matrix of the camera.
	/// * image: BGR or Gray-Scale image that is going to be displayed on the far plane of the frustum.
	/// * scale: Scale of the frustum and image.
	/// * color: Color of the frustum.
	/// 
	/// Creates viewing frustum of the camera based on its intrinsic matrix K, and displays image on
	/// the far end plane.
	/// 
	/// ![Camera viewing frustum with image](https://docs.opencv.org/4.5.4/cpw3.png)
	/// 
	/// ## C++ default parameters
	/// * scale: 1.0
	/// * color: Color::white()
	#[inline]
	pub fn new_3(k: core::Matx33d, image: &dyn core::ToInputArray, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WCameraPosition> {
		input_array_arg!(image);
		unsafe { sys::cv_viz_WCameraPosition_WCameraPosition_const_Matx33dR_const__InputArrayR_double_const_ColorR(&k, image.as_raw__InputArray(), scale, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WCameraPosition::opencv_from_extern(r) } )
	}
	
	///  Display image on the far plane of the viewing frustum
	/// 
	/// ## Parameters
	/// * fov: Field of view of the camera (horizontal, vertical).
	/// * image: BGR or Gray-Scale image that is going to be displayed on the far plane of the frustum.
	/// * scale: Scale of the frustum and image.
	/// * color: Color of the frustum.
	/// 
	/// Creates viewing frustum of the camera based on its intrinsic matrix K, and displays image on
	/// the far end plane.
	/// 
	/// ![Camera viewing frustum with image](https://docs.opencv.org/4.5.4/cpw3.png)
	/// 
	/// ## C++ default parameters
	/// * scale: 1.0
	/// * color: Color::white()
	#[inline]
	pub fn new_4(fov: core::Vec2d, image: &dyn core::ToInputArray, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WCameraPosition> {
		input_array_arg!(image);
		unsafe { sys::cv_viz_WCameraPosition_WCameraPosition_const_Vec2dR_const__InputArrayR_double_const_ColorR(&fov, image.as_raw__InputArray(), scale, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WCameraPosition::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WCameraPosition, crate::viz::Widget, cv_WCameraPosition_to_Widget }

boxed_cast_base! { WCameraPosition, crate::viz::Widget3D, cv_WCameraPosition_to_Widget3D }

/// This 3D Widget defines a circle.
pub trait WCircleTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WCircle(&self) -> *const c_void;

}

pub trait WCircleTrait: crate::viz::WCircleTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WCircle(&mut self) -> *mut c_void;

}

/// This 3D Widget defines a circle.
pub struct WCircle {
	ptr: *mut c_void
}

opencv_type_boxed! { WCircle }

impl Drop for WCircle {
	fn drop(&mut self) {
		extern "C" { fn cv_WCircle_delete(instance: *mut c_void); }
		unsafe { cv_WCircle_delete(self.as_raw_mut_WCircle()) };
	}
}

unsafe impl Send for WCircle {}

impl crate::viz::WidgetTraitConst for WCircle {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WCircle {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WCircle {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WCircle {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WCircleTraitConst for WCircle {
	#[inline] fn as_raw_WCircle(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WCircleTrait for WCircle {
	#[inline] fn as_raw_mut_WCircle(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WCircle {
	/// Constructs default planar circle centered at origin with plane normal along z-axis
	/// 
	/// ## Parameters
	/// * radius: Radius of the circle.
	/// * thickness: Thickness of the circle.
	/// * color: Color of the circle.
	/// 
	/// ## C++ default parameters
	/// * thickness: 0.01
	/// * color: Color::white()
	#[inline]
	pub fn new(radius: f64, thickness: f64, color: &crate::viz::Color) -> Result<crate::viz::WCircle> {
		unsafe { sys::cv_viz_WCircle_WCircle_double_double_const_ColorR(radius, thickness, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WCircle::opencv_from_extern(r) } )
	}
	
	/// Constructs repositioned planar circle.
	/// 
	/// ## Parameters
	/// * radius: Radius of the circle.
	/// * center: Center of the circle.
	/// * normal: Normal of the plane in which the circle lies.
	/// * thickness: Thickness of the circle.
	/// * color: Color of the circle.
	/// 
	/// ## C++ default parameters
	/// * thickness: 0.01
	/// * color: Color::white()
	#[inline]
	pub fn new_1(radius: f64, center: core::Point3d, normal: core::Vec3d, thickness: f64, color: &crate::viz::Color) -> Result<crate::viz::WCircle> {
		unsafe { sys::cv_viz_WCircle_WCircle_double_const_Point3dR_const_Vec3dR_double_const_ColorR(radius, &center, &normal, thickness, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WCircle::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WCircle, crate::viz::Widget, cv_WCircle_to_Widget }

boxed_cast_base! { WCircle, crate::viz::Widget3D, cv_WCircle_to_Widget3D }

/// This 3D Widget defines a point cloud. :
/// 
/// 
/// Note: In case there are four channels in the cloud, fourth channel is ignored.
pub trait WCloudTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WCloud(&self) -> *const c_void;

}

pub trait WCloudTrait: crate::viz::WCloudTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WCloud(&mut self) -> *mut c_void;

}

/// This 3D Widget defines a point cloud. :
/// 
/// 
/// Note: In case there are four channels in the cloud, fourth channel is ignored.
pub struct WCloud {
	ptr: *mut c_void
}

opencv_type_boxed! { WCloud }

impl Drop for WCloud {
	fn drop(&mut self) {
		extern "C" { fn cv_WCloud_delete(instance: *mut c_void); }
		unsafe { cv_WCloud_delete(self.as_raw_mut_WCloud()) };
	}
}

unsafe impl Send for WCloud {}

impl crate::viz::WidgetTraitConst for WCloud {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WCloud {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WCloud {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WCloud {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WCloudTraitConst for WCloud {
	#[inline] fn as_raw_WCloud(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WCloudTrait for WCloud {
	#[inline] fn as_raw_mut_WCloud(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WCloud {
	/// Constructs a WCloud.
	/// 
	/// ## Parameters
	/// * cloud: Set of points which can be of type: CV_32FC3, CV_32FC4, CV_64FC3, CV_64FC4.
	/// * colors: Set of colors. It has to be of the same size with cloud.
	/// 
	/// Points in the cloud belong to mask when they are set to (NaN, NaN, NaN).
	#[inline]
	pub fn new(cloud: &dyn core::ToInputArray, colors: &dyn core::ToInputArray) -> Result<crate::viz::WCloud> {
		input_array_arg!(cloud);
		input_array_arg!(colors);
		unsafe { sys::cv_viz_WCloud_WCloud_const__InputArrayR_const__InputArrayR(cloud.as_raw__InputArray(), colors.as_raw__InputArray()) }.into_result().map(|r| unsafe { crate::viz::WCloud::opencv_from_extern(r) } )
	}
	
	/// Constructs a WCloud.
	/// ## Parameters
	/// * cloud: Set of points which can be of type: CV_32FC3, CV_32FC4, CV_64FC3, CV_64FC4.
	/// * color: A single Color for the whole cloud.
	/// 
	/// Points in the cloud belong to mask when they are set to (NaN, NaN, NaN).
	/// 
	/// ## C++ default parameters
	/// * color: Color::white()
	#[inline]
	pub fn new_1(cloud: &dyn core::ToInputArray, color: &crate::viz::Color) -> Result<crate::viz::WCloud> {
		input_array_arg!(cloud);
		unsafe { sys::cv_viz_WCloud_WCloud_const__InputArrayR_const_ColorR(cloud.as_raw__InputArray(), color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WCloud::opencv_from_extern(r) } )
	}
	
	/// Constructs a WCloud.
	/// ## Parameters
	/// * cloud: Set of points which can be of type: CV_32FC3, CV_32FC4, CV_64FC3, CV_64FC4.
	/// * colors: Set of colors. It has to be of the same size with cloud.
	/// * normals: Normals for each point in cloud. Size and type should match with the cloud parameter.
	/// 
	/// Points in the cloud belong to mask when they are set to (NaN, NaN, NaN).
	#[inline]
	pub fn new_2(cloud: &dyn core::ToInputArray, colors: &dyn core::ToInputArray, normals: &dyn core::ToInputArray) -> Result<crate::viz::WCloud> {
		input_array_arg!(cloud);
		input_array_arg!(colors);
		input_array_arg!(normals);
		unsafe { sys::cv_viz_WCloud_WCloud_const__InputArrayR_const__InputArrayR_const__InputArrayR(cloud.as_raw__InputArray(), colors.as_raw__InputArray(), normals.as_raw__InputArray()) }.into_result().map(|r| unsafe { crate::viz::WCloud::opencv_from_extern(r) } )
	}
	
	/// Constructs a WCloud.
	/// ## Parameters
	/// * cloud: Set of points which can be of type: CV_32FC3, CV_32FC4, CV_64FC3, CV_64FC4.
	/// * color: A single Color for the whole cloud.
	/// * normals: Normals for each point in cloud.
	/// 
	/// Size and type should match with the cloud parameter.
	/// Points in the cloud belong to mask when they are set to (NaN, NaN, NaN).
	#[inline]
	pub fn new_3(cloud: &dyn core::ToInputArray, color: &crate::viz::Color, normals: &dyn core::ToInputArray) -> Result<crate::viz::WCloud> {
		input_array_arg!(cloud);
		input_array_arg!(normals);
		unsafe { sys::cv_viz_WCloud_WCloud_const__InputArrayR_const_ColorR_const__InputArrayR(cloud.as_raw__InputArray(), color.as_raw_Color(), normals.as_raw__InputArray()) }.into_result().map(|r| unsafe { crate::viz::WCloud::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WCloud, crate::viz::Widget, cv_WCloud_to_Widget }

boxed_cast_base! { WCloud, crate::viz::Widget3D, cv_WCloud_to_Widget3D }

/// This 3D Widget defines a collection of clouds. :
/// 
/// Note: In case there are four channels in the cloud, fourth channel is ignored.
pub trait WCloudCollectionTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WCloudCollection(&self) -> *const c_void;

}

pub trait WCloudCollectionTrait: crate::viz::WCloudCollectionTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WCloudCollection(&mut self) -> *mut c_void;

	/// Adds a cloud to the collection.
	/// 
	/// ## Parameters
	/// * cloud: Point set which can be of type: CV_32FC3, CV_32FC4, CV_64FC3, CV_64FC4.
	/// * colors: Set of colors. It has to be of the same size with cloud.
	/// * pose: Pose of the cloud. Points in the cloud belong to mask when they are set to (NaN, NaN, NaN).
	/// 
	/// ## C++ default parameters
	/// * pose: Affine3d::Identity()
	#[inline]
	fn add_cloud(&mut self, cloud: &dyn core::ToInputArray, colors: &dyn core::ToInputArray, pose: core::Affine3d) -> Result<()> {
		input_array_arg!(cloud);
		input_array_arg!(colors);
		unsafe { sys::cv_viz_WCloudCollection_addCloud_const__InputArrayR_const__InputArrayR_const_Affine3dR(self.as_raw_mut_WCloudCollection(), cloud.as_raw__InputArray(), colors.as_raw__InputArray(), &pose) }.into_result()
	}
	
	/// Adds a cloud to the collection.
	/// 
	/// ## Parameters
	/// * cloud: Point set which can be of type: CV_32FC3, CV_32FC4, CV_64FC3, CV_64FC4.
	/// * color: A single Color for the whole cloud.
	/// * pose: Pose of the cloud. Points in the cloud belong to mask when they are set to (NaN, NaN, NaN).
	/// 
	/// ## C++ default parameters
	/// * color: Color::white()
	/// * pose: Affine3d::Identity()
	#[inline]
	fn add_cloud_1(&mut self, cloud: &dyn core::ToInputArray, color: &crate::viz::Color, pose: core::Affine3d) -> Result<()> {
		input_array_arg!(cloud);
		unsafe { sys::cv_viz_WCloudCollection_addCloud_const__InputArrayR_const_ColorR_const_Affine3dR(self.as_raw_mut_WCloudCollection(), cloud.as_raw__InputArray(), color.as_raw_Color(), &pose) }.into_result()
	}
	
	/// Finalizes cloud data by repacking to single cloud.
	/// 
	/// Useful for large cloud collections to reduce memory usage
	#[inline]
	fn finalize(&mut self) -> Result<()> {
		unsafe { sys::cv_viz_WCloudCollection_finalize(self.as_raw_mut_WCloudCollection()) }.into_result()
	}
	
}

/// This 3D Widget defines a collection of clouds. :
/// 
/// Note: In case there are four channels in the cloud, fourth channel is ignored.
pub struct WCloudCollection {
	ptr: *mut c_void
}

opencv_type_boxed! { WCloudCollection }

impl Drop for WCloudCollection {
	fn drop(&mut self) {
		extern "C" { fn cv_WCloudCollection_delete(instance: *mut c_void); }
		unsafe { cv_WCloudCollection_delete(self.as_raw_mut_WCloudCollection()) };
	}
}

unsafe impl Send for WCloudCollection {}

impl crate::viz::WidgetTraitConst for WCloudCollection {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WCloudCollection {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WCloudCollection {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WCloudCollection {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WCloudCollectionTraitConst for WCloudCollection {
	#[inline] fn as_raw_WCloudCollection(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WCloudCollectionTrait for WCloudCollection {
	#[inline] fn as_raw_mut_WCloudCollection(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WCloudCollection {
	#[inline]
	pub fn default() -> Result<crate::viz::WCloudCollection> {
		unsafe { sys::cv_viz_WCloudCollection_WCloudCollection() }.into_result().map(|r| unsafe { crate::viz::WCloudCollection::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WCloudCollection, crate::viz::Widget, cv_WCloudCollection_to_Widget }

boxed_cast_base! { WCloudCollection, crate::viz::Widget3D, cv_WCloudCollection_to_Widget3D }

/// This 3D Widget represents normals of a point cloud. :
pub trait WCloudNormalsTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WCloudNormals(&self) -> *const c_void;

}

pub trait WCloudNormalsTrait: crate::viz::WCloudNormalsTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WCloudNormals(&mut self) -> *mut c_void;

}

/// This 3D Widget represents normals of a point cloud. :
pub struct WCloudNormals {
	ptr: *mut c_void
}

opencv_type_boxed! { WCloudNormals }

impl Drop for WCloudNormals {
	fn drop(&mut self) {
		extern "C" { fn cv_WCloudNormals_delete(instance: *mut c_void); }
		unsafe { cv_WCloudNormals_delete(self.as_raw_mut_WCloudNormals()) };
	}
}

unsafe impl Send for WCloudNormals {}

impl crate::viz::WidgetTraitConst for WCloudNormals {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WCloudNormals {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WCloudNormals {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WCloudNormals {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WCloudNormalsTraitConst for WCloudNormals {
	#[inline] fn as_raw_WCloudNormals(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WCloudNormalsTrait for WCloudNormals {
	#[inline] fn as_raw_mut_WCloudNormals(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WCloudNormals {
	/// Constructs a WCloudNormals.
	/// 
	/// ## Parameters
	/// * cloud: Point set which can be of type: CV_32FC3, CV_32FC4, CV_64FC3, CV_64FC4.
	/// * normals: A set of normals that has to be of same type with cloud.
	/// * level: Display only every level th normal.
	/// * scale: Scale of the arrows that represent normals.
	/// * color: Color of the arrows that represent normals.
	/// 
	/// 
	/// Note: In case there are four channels in the cloud, fourth channel is ignored.
	/// 
	/// ## C++ default parameters
	/// * level: 64
	/// * scale: 0.1
	/// * color: Color::white()
	#[inline]
	pub fn new(cloud: &dyn core::ToInputArray, normals: &dyn core::ToInputArray, level: i32, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WCloudNormals> {
		input_array_arg!(cloud);
		input_array_arg!(normals);
		unsafe { sys::cv_viz_WCloudNormals_WCloudNormals_const__InputArrayR_const__InputArrayR_int_double_const_ColorR(cloud.as_raw__InputArray(), normals.as_raw__InputArray(), level, scale, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WCloudNormals::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WCloudNormals, crate::viz::Widget, cv_WCloudNormals_to_Widget }

boxed_cast_base! { WCloudNormals, crate::viz::Widget3D, cv_WCloudNormals_to_Widget3D }

/// This 3D Widget defines a cone. :
pub trait WConeTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WCone(&self) -> *const c_void;

}

pub trait WConeTrait: crate::viz::WConeTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WCone(&mut self) -> *mut c_void;

}

/// This 3D Widget defines a cone. :
pub struct WCone {
	ptr: *mut c_void
}

opencv_type_boxed! { WCone }

impl Drop for WCone {
	fn drop(&mut self) {
		extern "C" { fn cv_WCone_delete(instance: *mut c_void); }
		unsafe { cv_WCone_delete(self.as_raw_mut_WCone()) };
	}
}

unsafe impl Send for WCone {}

impl crate::viz::WidgetTraitConst for WCone {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WCone {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WCone {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WCone {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WConeTraitConst for WCone {
	#[inline] fn as_raw_WCone(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WConeTrait for WCone {
	#[inline] fn as_raw_mut_WCone(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WCone {
	/// Constructs default cone oriented along x-axis with center of its base located at origin
	/// 
	/// ## Parameters
	/// * length: Length of the cone.
	/// * radius: Radius of the cone.
	/// * resolution: Resolution of the cone.
	/// * color: Color of the cone.
	/// 
	/// ## C++ default parameters
	/// * resolution: 6
	/// * color: Color::white()
	#[inline]
	pub fn new(length: f64, radius: f64, resolution: i32, color: &crate::viz::Color) -> Result<crate::viz::WCone> {
		unsafe { sys::cv_viz_WCone_WCone_double_double_int_const_ColorR(length, radius, resolution, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WCone::opencv_from_extern(r) } )
	}
	
	/// Constructs repositioned planar cone.
	/// 
	/// ## Parameters
	/// * radius: Radius of the cone.
	/// * center: Center of the cone base.
	/// * tip: Tip of the cone.
	/// * resolution: Resolution of the cone.
	/// * color: Color of the cone.
	/// 
	/// ## C++ default parameters
	/// * resolution: 6
	/// * color: Color::white()
	#[inline]
	pub fn new_1(radius: f64, center: core::Point3d, tip: core::Point3d, resolution: i32, color: &crate::viz::Color) -> Result<crate::viz::WCone> {
		unsafe { sys::cv_viz_WCone_WCone_double_const_Point3dR_const_Point3dR_int_const_ColorR(radius, &center, &tip, resolution, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WCone::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WCone, crate::viz::Widget, cv_WCone_to_Widget }

boxed_cast_base! { WCone, crate::viz::Widget3D, cv_WCone_to_Widget3D }

/// This 3D Widget represents a coordinate system. :
pub trait WCoordinateSystemTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WCoordinateSystem(&self) -> *const c_void;

}

pub trait WCoordinateSystemTrait: crate::viz::WCoordinateSystemTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WCoordinateSystem(&mut self) -> *mut c_void;

}

/// This 3D Widget represents a coordinate system. :
pub struct WCoordinateSystem {
	ptr: *mut c_void
}

opencv_type_boxed! { WCoordinateSystem }

impl Drop for WCoordinateSystem {
	fn drop(&mut self) {
		extern "C" { fn cv_WCoordinateSystem_delete(instance: *mut c_void); }
		unsafe { cv_WCoordinateSystem_delete(self.as_raw_mut_WCoordinateSystem()) };
	}
}

unsafe impl Send for WCoordinateSystem {}

impl crate::viz::WidgetTraitConst for WCoordinateSystem {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WCoordinateSystem {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WCoordinateSystem {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WCoordinateSystem {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WCoordinateSystemTraitConst for WCoordinateSystem {
	#[inline] fn as_raw_WCoordinateSystem(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WCoordinateSystemTrait for WCoordinateSystem {
	#[inline] fn as_raw_mut_WCoordinateSystem(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WCoordinateSystem {
	/// Constructs a WCoordinateSystem.
	/// 
	/// ## Parameters
	/// * scale: Determines the size of the axes.
	/// 
	/// ## C++ default parameters
	/// * scale: 1.0
	#[inline]
	pub fn new(scale: f64) -> Result<crate::viz::WCoordinateSystem> {
		unsafe { sys::cv_viz_WCoordinateSystem_WCoordinateSystem_double(scale) }.into_result().map(|r| unsafe { crate::viz::WCoordinateSystem::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WCoordinateSystem, crate::viz::Widget, cv_WCoordinateSystem_to_Widget }

boxed_cast_base! { WCoordinateSystem, crate::viz::Widget3D, cv_WCoordinateSystem_to_Widget3D }

/// This 3D Widget defines a cube.
pub trait WCubeTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WCube(&self) -> *const c_void;

}

pub trait WCubeTrait: crate::viz::WCubeTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WCube(&mut self) -> *mut c_void;

}

/// This 3D Widget defines a cube.
pub struct WCube {
	ptr: *mut c_void
}

opencv_type_boxed! { WCube }

impl Drop for WCube {
	fn drop(&mut self) {
		extern "C" { fn cv_WCube_delete(instance: *mut c_void); }
		unsafe { cv_WCube_delete(self.as_raw_mut_WCube()) };
	}
}

unsafe impl Send for WCube {}

impl crate::viz::WidgetTraitConst for WCube {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WCube {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WCube {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WCube {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WCubeTraitConst for WCube {
	#[inline] fn as_raw_WCube(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WCubeTrait for WCube {
	#[inline] fn as_raw_mut_WCube(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WCube {
	/// Constructs a WCube.
	/// 
	/// ## Parameters
	/// * min_point: Specifies minimum (or maximum) point of the bounding box.
	/// * max_point: Specifies maximum (or minimum) point of the bounding box, opposite to the first parameter.
	/// * wire_frame: If true, cube is represented as wireframe.
	/// * color: Color of the cube.
	/// 
	/// ![Cube Widget](https://docs.opencv.org/4.5.4/cube_widget.png)
	/// 
	/// ## C++ default parameters
	/// * min_point: Vec3d::all(-0.5)
	/// * max_point: Vec3d::all(0.5)
	/// * wire_frame: true
	/// * color: Color::white()
	#[inline]
	pub fn new(min_point: core::Point3d, max_point: core::Point3d, wire_frame: bool, color: &crate::viz::Color) -> Result<crate::viz::WCube> {
		unsafe { sys::cv_viz_WCube_WCube_const_Point3dR_const_Point3dR_bool_const_ColorR(&min_point, &max_point, wire_frame, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WCube::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WCube, crate::viz::Widget, cv_WCube_to_Widget }

boxed_cast_base! { WCube, crate::viz::Widget3D, cv_WCube_to_Widget3D }

/// This 3D Widget defines a cylinder. :
pub trait WCylinderTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WCylinder(&self) -> *const c_void;

}

pub trait WCylinderTrait: crate::viz::WCylinderTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WCylinder(&mut self) -> *mut c_void;

}

/// This 3D Widget defines a cylinder. :
pub struct WCylinder {
	ptr: *mut c_void
}

opencv_type_boxed! { WCylinder }

impl Drop for WCylinder {
	fn drop(&mut self) {
		extern "C" { fn cv_WCylinder_delete(instance: *mut c_void); }
		unsafe { cv_WCylinder_delete(self.as_raw_mut_WCylinder()) };
	}
}

unsafe impl Send for WCylinder {}

impl crate::viz::WidgetTraitConst for WCylinder {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WCylinder {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WCylinder {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WCylinder {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WCylinderTraitConst for WCylinder {
	#[inline] fn as_raw_WCylinder(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WCylinderTrait for WCylinder {
	#[inline] fn as_raw_mut_WCylinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WCylinder {
	/// Constructs a WCylinder.
	/// 
	/// ## Parameters
	/// * axis_point1: A point1 on the axis of the cylinder.
	/// * axis_point2: A point2 on the axis of the cylinder.
	/// * radius: Radius of the cylinder.
	/// * numsides: Resolution of the cylinder.
	/// * color: Color of the cylinder.
	/// 
	/// ## C++ default parameters
	/// * numsides: 30
	/// * color: Color::white()
	#[inline]
	pub fn new(axis_point1: core::Point3d, axis_point2: core::Point3d, radius: f64, numsides: i32, color: &crate::viz::Color) -> Result<crate::viz::WCylinder> {
		unsafe { sys::cv_viz_WCylinder_WCylinder_const_Point3dR_const_Point3dR_double_int_const_ColorR(&axis_point1, &axis_point2, radius, numsides, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WCylinder::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WCylinder, crate::viz::Widget, cv_WCylinder_to_Widget }

boxed_cast_base! { WCylinder, crate::viz::Widget3D, cv_WCylinder_to_Widget3D }

/// This 3D Widget defines a grid. :
pub trait WGridTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WGrid(&self) -> *const c_void;

}

pub trait WGridTrait: crate::viz::WGridTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WGrid(&mut self) -> *mut c_void;

}

/// This 3D Widget defines a grid. :
pub struct WGrid {
	ptr: *mut c_void
}

opencv_type_boxed! { WGrid }

impl Drop for WGrid {
	fn drop(&mut self) {
		extern "C" { fn cv_WGrid_delete(instance: *mut c_void); }
		unsafe { cv_WGrid_delete(self.as_raw_mut_WGrid()) };
	}
}

unsafe impl Send for WGrid {}

impl crate::viz::WidgetTraitConst for WGrid {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WGrid {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WGrid {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WGrid {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WGridTraitConst for WGrid {
	#[inline] fn as_raw_WGrid(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WGridTrait for WGrid {
	#[inline] fn as_raw_mut_WGrid(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WGrid {
	/// Constructs a WGrid.
	/// 
	/// ## Parameters
	/// * cells: Number of cell columns and rows, respectively.
	/// * cells_spacing: Size of each cell, respectively.
	/// * color: Color of the grid.
	/// 
	/// ## C++ default parameters
	/// * cells: Vec2i::all(10)
	/// * cells_spacing: Vec2d::all(1.0)
	/// * color: Color::white()
	#[inline]
	pub fn new(cells: core::Vec2i, cells_spacing: core::Vec2d, color: &crate::viz::Color) -> Result<crate::viz::WGrid> {
		unsafe { sys::cv_viz_WGrid_WGrid_const_Vec2iR_const_Vec2dR_const_ColorR(&cells, &cells_spacing, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WGrid::opencv_from_extern(r) } )
	}
	
	/// Creates repositioned grid
	/// 
	/// ## C++ default parameters
	/// * cells: Vec2i::all(10)
	/// * cells_spacing: Vec2d::all(1.0)
	/// * color: Color::white()
	#[inline]
	pub fn new_1(center: core::Point3d, normal: core::Vec3d, new_yaxis: core::Vec3d, cells: core::Vec2i, cells_spacing: core::Vec2d, color: &crate::viz::Color) -> Result<crate::viz::WGrid> {
		unsafe { sys::cv_viz_WGrid_WGrid_const_Point3dR_const_Vec3dR_const_Vec3dR_const_Vec2iR_const_Vec2dR_const_ColorR(&center, &normal, &new_yaxis, &cells, &cells_spacing, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WGrid::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WGrid, crate::viz::Widget, cv_WGrid_to_Widget }

boxed_cast_base! { WGrid, crate::viz::Widget3D, cv_WGrid_to_Widget3D }

/// This 3D Widget represents an image in 3D space. :
pub trait WImage3DTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WImage3D(&self) -> *const c_void;

}

pub trait WImage3DTrait: crate::viz::WImage3DTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WImage3D(&mut self) -> *mut c_void;

	/// Sets the image content of the widget.
	/// 
	/// ## Parameters
	/// * image: BGR or Gray-Scale image.
	#[inline]
	fn set_image(&mut self, image: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_viz_WImage3D_setImage_const__InputArrayR(self.as_raw_mut_WImage3D(), image.as_raw__InputArray()) }.into_result()
	}
	
	/// Sets the image size of the widget.
	/// 
	/// ## Parameters
	/// * size: the new size of the image.
	#[inline]
	fn set_size(&mut self, size: core::Size) -> Result<()> {
		unsafe { sys::cv_viz_WImage3D_setSize_const_SizeR(self.as_raw_mut_WImage3D(), &size) }.into_result()
	}
	
}

/// This 3D Widget represents an image in 3D space. :
pub struct WImage3D {
	ptr: *mut c_void
}

opencv_type_boxed! { WImage3D }

impl Drop for WImage3D {
	fn drop(&mut self) {
		extern "C" { fn cv_WImage3D_delete(instance: *mut c_void); }
		unsafe { cv_WImage3D_delete(self.as_raw_mut_WImage3D()) };
	}
}

unsafe impl Send for WImage3D {}

impl crate::viz::WidgetTraitConst for WImage3D {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WImage3D {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WImage3D {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WImage3D {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WImage3DTraitConst for WImage3D {
	#[inline] fn as_raw_WImage3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WImage3DTrait for WImage3D {
	#[inline] fn as_raw_mut_WImage3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WImage3D {
	/// Constructs an WImage3D.
	/// 
	/// ## Parameters
	/// * image: BGR or Gray-Scale image.
	/// * size: Size of the image.
	#[inline]
	pub fn new(image: &dyn core::ToInputArray, size: core::Size2d) -> Result<crate::viz::WImage3D> {
		input_array_arg!(image);
		unsafe { sys::cv_viz_WImage3D_WImage3D_const__InputArrayR_const_Size2dR(image.as_raw__InputArray(), &size) }.into_result().map(|r| unsafe { crate::viz::WImage3D::opencv_from_extern(r) } )
	}
	
	/// Constructs an WImage3D.
	/// 
	/// ## Parameters
	/// * image: BGR or Gray-Scale image.
	/// * size: Size of the image.
	/// * center: Position of the image.
	/// * normal: Normal of the plane that represents the image.
	/// * up_vector: Determines orientation of the image.
	#[inline]
	pub fn new_1(image: &dyn core::ToInputArray, size: core::Size2d, center: core::Vec3d, normal: core::Vec3d, up_vector: core::Vec3d) -> Result<crate::viz::WImage3D> {
		input_array_arg!(image);
		unsafe { sys::cv_viz_WImage3D_WImage3D_const__InputArrayR_const_Size2dR_const_Vec3dR_const_Vec3dR_const_Vec3dR(image.as_raw__InputArray(), &size, &center, &normal, &up_vector) }.into_result().map(|r| unsafe { crate::viz::WImage3D::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WImage3D, crate::viz::Widget, cv_WImage3D_to_Widget }

boxed_cast_base! { WImage3D, crate::viz::Widget3D, cv_WImage3D_to_Widget3D }

/// This 2D Widget represents an image overlay. :
pub trait WImageOverlayTraitConst: crate::viz::Widget2DTraitConst {
	fn as_raw_WImageOverlay(&self) -> *const c_void;

}

pub trait WImageOverlayTrait: crate::viz::WImageOverlayTraitConst + crate::viz::Widget2DTrait {
	fn as_raw_mut_WImageOverlay(&mut self) -> *mut c_void;

	/// Sets the image content of the widget.
	/// 
	/// ## Parameters
	/// * image: BGR or Gray-Scale image.
	#[inline]
	fn set_image(&mut self, image: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_viz_WImageOverlay_setImage_const__InputArrayR(self.as_raw_mut_WImageOverlay(), image.as_raw__InputArray()) }.into_result()
	}
	
}

/// This 2D Widget represents an image overlay. :
pub struct WImageOverlay {
	ptr: *mut c_void
}

opencv_type_boxed! { WImageOverlay }

impl Drop for WImageOverlay {
	fn drop(&mut self) {
		extern "C" { fn cv_WImageOverlay_delete(instance: *mut c_void); }
		unsafe { cv_WImageOverlay_delete(self.as_raw_mut_WImageOverlay()) };
	}
}

unsafe impl Send for WImageOverlay {}

impl crate::viz::WidgetTraitConst for WImageOverlay {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WImageOverlay {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget2DTraitConst for WImageOverlay {
	#[inline] fn as_raw_Widget2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget2DTrait for WImageOverlay {
	#[inline] fn as_raw_mut_Widget2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WImageOverlayTraitConst for WImageOverlay {
	#[inline] fn as_raw_WImageOverlay(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WImageOverlayTrait for WImageOverlay {
	#[inline] fn as_raw_mut_WImageOverlay(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WImageOverlay {
	/// Constructs an WImageOverlay.
	/// 
	/// ## Parameters
	/// * image: BGR or Gray-Scale image.
	/// * rect: Image is scaled and positioned based on rect.
	#[inline]
	pub fn new(image: &dyn core::ToInputArray, rect: core::Rect) -> Result<crate::viz::WImageOverlay> {
		input_array_arg!(image);
		unsafe { sys::cv_viz_WImageOverlay_WImageOverlay_const__InputArrayR_const_RectR(image.as_raw__InputArray(), &rect) }.into_result().map(|r| unsafe { crate::viz::WImageOverlay::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WImageOverlay, crate::viz::Widget, cv_WImageOverlay_to_Widget }

boxed_cast_base! { WImageOverlay, crate::viz::Widget2D, cv_WImageOverlay_to_Widget2D }

/// This 3D Widget defines a finite line.
pub trait WLineTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WLine(&self) -> *const c_void;

}

pub trait WLineTrait: crate::viz::WLineTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WLine(&mut self) -> *mut c_void;

}

/// This 3D Widget defines a finite line.
pub struct WLine {
	ptr: *mut c_void
}

opencv_type_boxed! { WLine }

impl Drop for WLine {
	fn drop(&mut self) {
		extern "C" { fn cv_WLine_delete(instance: *mut c_void); }
		unsafe { cv_WLine_delete(self.as_raw_mut_WLine()) };
	}
}

unsafe impl Send for WLine {}

impl crate::viz::WidgetTraitConst for WLine {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WLine {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WLine {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WLine {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WLineTraitConst for WLine {
	#[inline] fn as_raw_WLine(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WLineTrait for WLine {
	#[inline] fn as_raw_mut_WLine(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WLine {
	/// Constructs a WLine.
	/// 
	/// ## Parameters
	/// * pt1: Start point of the line.
	/// * pt2: End point of the line.
	/// * color: Color of the line.
	/// 
	/// ## C++ default parameters
	/// * color: Color::white()
	#[inline]
	pub fn new(pt1: core::Point3d, pt2: core::Point3d, color: &crate::viz::Color) -> Result<crate::viz::WLine> {
		unsafe { sys::cv_viz_WLine_WLine_const_Point3dR_const_Point3dR_const_ColorR(&pt1, &pt2, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WLine::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WLine, crate::viz::Widget, cv_WLine_to_Widget }

boxed_cast_base! { WLine, crate::viz::Widget3D, cv_WLine_to_Widget3D }

/// Constructs a WMesh.
/// 
/// ## Parameters
/// * mesh: Mesh object that will be displayed.
/// * cloud: Points of the mesh object.
/// * polygons: Points of the mesh object.
/// * colors: Point colors.
/// * normals: Point normals.
pub trait WMeshTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WMesh(&self) -> *const c_void;

}

pub trait WMeshTrait: crate::viz::WMeshTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WMesh(&mut self) -> *mut c_void;

}

/// Constructs a WMesh.
/// 
/// ## Parameters
/// * mesh: Mesh object that will be displayed.
/// * cloud: Points of the mesh object.
/// * polygons: Points of the mesh object.
/// * colors: Point colors.
/// * normals: Point normals.
pub struct WMesh {
	ptr: *mut c_void
}

opencv_type_boxed! { WMesh }

impl Drop for WMesh {
	fn drop(&mut self) {
		extern "C" { fn cv_WMesh_delete(instance: *mut c_void); }
		unsafe { cv_WMesh_delete(self.as_raw_mut_WMesh()) };
	}
}

unsafe impl Send for WMesh {}

impl crate::viz::WidgetTraitConst for WMesh {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WMesh {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WMesh {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WMesh {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WMeshTraitConst for WMesh {
	#[inline] fn as_raw_WMesh(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WMeshTrait for WMesh {
	#[inline] fn as_raw_mut_WMesh(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WMesh {
	#[inline]
	pub fn new(mesh: &crate::viz::Mesh) -> Result<crate::viz::WMesh> {
		unsafe { sys::cv_viz_WMesh_WMesh_const_MeshR(mesh.as_raw_Mesh()) }.into_result().map(|r| unsafe { crate::viz::WMesh::opencv_from_extern(r) } )
	}
	
	/// ## C++ default parameters
	/// * colors: noArray()
	/// * normals: noArray()
	#[inline]
	pub fn new_1(cloud: &dyn core::ToInputArray, polygons: &dyn core::ToInputArray, colors: &dyn core::ToInputArray, normals: &dyn core::ToInputArray) -> Result<crate::viz::WMesh> {
		input_array_arg!(cloud);
		input_array_arg!(polygons);
		input_array_arg!(colors);
		input_array_arg!(normals);
		unsafe { sys::cv_viz_WMesh_WMesh_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(cloud.as_raw__InputArray(), polygons.as_raw__InputArray(), colors.as_raw__InputArray(), normals.as_raw__InputArray()) }.into_result().map(|r| unsafe { crate::viz::WMesh::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WMesh, crate::viz::Widget, cv_WMesh_to_Widget }

boxed_cast_base! { WMesh, crate::viz::Widget3D, cv_WMesh_to_Widget3D }

pub trait WPaintedCloudTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WPaintedCloud(&self) -> *const c_void;

}

pub trait WPaintedCloudTrait: crate::viz::WPaintedCloudTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WPaintedCloud(&mut self) -> *mut c_void;

}

pub struct WPaintedCloud {
	ptr: *mut c_void
}

opencv_type_boxed! { WPaintedCloud }

impl Drop for WPaintedCloud {
	fn drop(&mut self) {
		extern "C" { fn cv_WPaintedCloud_delete(instance: *mut c_void); }
		unsafe { cv_WPaintedCloud_delete(self.as_raw_mut_WPaintedCloud()) };
	}
}

unsafe impl Send for WPaintedCloud {}

impl crate::viz::WidgetTraitConst for WPaintedCloud {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WPaintedCloud {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WPaintedCloud {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WPaintedCloud {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WPaintedCloudTraitConst for WPaintedCloud {
	#[inline] fn as_raw_WPaintedCloud(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WPaintedCloudTrait for WPaintedCloud {
	#[inline] fn as_raw_mut_WPaintedCloud(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WPaintedCloud {
	/// Paint cloud with default gradient between cloud bounds points
	#[inline]
	pub fn new(cloud: &dyn core::ToInputArray) -> Result<crate::viz::WPaintedCloud> {
		input_array_arg!(cloud);
		unsafe { sys::cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayR(cloud.as_raw__InputArray()) }.into_result().map(|r| unsafe { crate::viz::WPaintedCloud::opencv_from_extern(r) } )
	}
	
	/// Paint cloud with default gradient between given points
	#[inline]
	pub fn new_1(cloud: &dyn core::ToInputArray, p1: core::Point3d, p2: core::Point3d) -> Result<crate::viz::WPaintedCloud> {
		input_array_arg!(cloud);
		unsafe { sys::cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayR_const_Point3dR_const_Point3dR(cloud.as_raw__InputArray(), &p1, &p2) }.into_result().map(|r| unsafe { crate::viz::WPaintedCloud::opencv_from_extern(r) } )
	}
	
	/// Paint cloud with gradient specified by given colors between given points
	#[inline]
	pub fn new_2(cloud: &dyn core::ToInputArray, p1: core::Point3d, p2: core::Point3d, c1: &crate::viz::Color, c2: crate::viz::Color) -> Result<crate::viz::WPaintedCloud> {
		input_array_arg!(cloud);
		unsafe { sys::cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayR_const_Point3dR_const_Point3dR_const_ColorR_const_Color(cloud.as_raw__InputArray(), &p1, &p2, c1.as_raw_Color(), c2.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WPaintedCloud::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WPaintedCloud, crate::viz::Widget, cv_WPaintedCloud_to_Widget }

boxed_cast_base! { WPaintedCloud, crate::viz::Widget3D, cv_WPaintedCloud_to_Widget3D }

/// This 3D Widget defines a finite plane.
pub trait WPlaneTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WPlane(&self) -> *const c_void;

}

pub trait WPlaneTrait: crate::viz::WPlaneTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WPlane(&mut self) -> *mut c_void;

}

/// This 3D Widget defines a finite plane.
pub struct WPlane {
	ptr: *mut c_void
}

opencv_type_boxed! { WPlane }

impl Drop for WPlane {
	fn drop(&mut self) {
		extern "C" { fn cv_WPlane_delete(instance: *mut c_void); }
		unsafe { cv_WPlane_delete(self.as_raw_mut_WPlane()) };
	}
}

unsafe impl Send for WPlane {}

impl crate::viz::WidgetTraitConst for WPlane {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WPlane {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WPlane {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WPlane {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WPlaneTraitConst for WPlane {
	#[inline] fn as_raw_WPlane(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WPlaneTrait for WPlane {
	#[inline] fn as_raw_mut_WPlane(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WPlane {
	/// Constructs a default plane with center point at origin and normal oriented along z-axis.
	/// 
	/// ## Parameters
	/// * size: Size of the plane
	/// * color: Color of the plane.
	/// 
	/// ## C++ default parameters
	/// * size: Size2d(1.0,1.0)
	/// * color: Color::white()
	#[inline]
	pub fn new(size: core::Size2d, color: &crate::viz::Color) -> Result<crate::viz::WPlane> {
		unsafe { sys::cv_viz_WPlane_WPlane_const_Size2dR_const_ColorR(&size, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WPlane::opencv_from_extern(r) } )
	}
	
	/// Constructs a repositioned plane
	/// 
	/// ## Parameters
	/// * center: Center of the plane
	/// * normal: Plane normal orientation
	/// * new_yaxis: Up-vector. New orientation of plane y-axis.
	/// * size: 
	/// * color: Color of the plane.
	/// 
	/// ## C++ default parameters
	/// * size: Size2d(1.0,1.0)
	/// * color: Color::white()
	#[inline]
	pub fn new_1(center: core::Point3d, normal: core::Vec3d, new_yaxis: core::Vec3d, size: core::Size2d, color: &crate::viz::Color) -> Result<crate::viz::WPlane> {
		unsafe { sys::cv_viz_WPlane_WPlane_const_Point3dR_const_Vec3dR_const_Vec3dR_const_Size2dR_const_ColorR(&center, &normal, &new_yaxis, &size, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WPlane::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WPlane, crate::viz::Widget, cv_WPlane_to_Widget }

boxed_cast_base! { WPlane, crate::viz::Widget3D, cv_WPlane_to_Widget3D }

/// This 3D Widget defines a poly line. :
pub trait WPolyLineTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WPolyLine(&self) -> *const c_void;

}

pub trait WPolyLineTrait: crate::viz::WPolyLineTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WPolyLine(&mut self) -> *mut c_void;

}

/// This 3D Widget defines a poly line. :
pub struct WPolyLine {
	ptr: *mut c_void
}

opencv_type_boxed! { WPolyLine }

impl Drop for WPolyLine {
	fn drop(&mut self) {
		extern "C" { fn cv_WPolyLine_delete(instance: *mut c_void); }
		unsafe { cv_WPolyLine_delete(self.as_raw_mut_WPolyLine()) };
	}
}

unsafe impl Send for WPolyLine {}

impl crate::viz::WidgetTraitConst for WPolyLine {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WPolyLine {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WPolyLine {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WPolyLine {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WPolyLineTraitConst for WPolyLine {
	#[inline] fn as_raw_WPolyLine(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WPolyLineTrait for WPolyLine {
	#[inline] fn as_raw_mut_WPolyLine(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WPolyLine {
	#[inline]
	pub fn new(points: &dyn core::ToInputArray, colors: &dyn core::ToInputArray) -> Result<crate::viz::WPolyLine> {
		input_array_arg!(points);
		input_array_arg!(colors);
		unsafe { sys::cv_viz_WPolyLine_WPolyLine_const__InputArrayR_const__InputArrayR(points.as_raw__InputArray(), colors.as_raw__InputArray()) }.into_result().map(|r| unsafe { crate::viz::WPolyLine::opencv_from_extern(r) } )
	}
	
	/// Constructs a WPolyLine.
	/// 
	/// ## Parameters
	/// * points: Point set.
	/// * color: Color of the poly line.
	/// 
	/// ## C++ default parameters
	/// * color: Color::white()
	#[inline]
	pub fn new_1(points: &dyn core::ToInputArray, color: &crate::viz::Color) -> Result<crate::viz::WPolyLine> {
		input_array_arg!(points);
		unsafe { sys::cv_viz_WPolyLine_WPolyLine_const__InputArrayR_const_ColorR(points.as_raw__InputArray(), color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WPolyLine::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WPolyLine, crate::viz::Widget, cv_WPolyLine_to_Widget }

boxed_cast_base! { WPolyLine, crate::viz::Widget3D, cv_WPolyLine_to_Widget3D }

/// This 3D Widget defines a sphere. :
pub trait WSphereTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WSphere(&self) -> *const c_void;

}

pub trait WSphereTrait: crate::viz::WSphereTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WSphere(&mut self) -> *mut c_void;

}

/// This 3D Widget defines a sphere. :
pub struct WSphere {
	ptr: *mut c_void
}

opencv_type_boxed! { WSphere }

impl Drop for WSphere {
	fn drop(&mut self) {
		extern "C" { fn cv_WSphere_delete(instance: *mut c_void); }
		unsafe { cv_WSphere_delete(self.as_raw_mut_WSphere()) };
	}
}

unsafe impl Send for WSphere {}

impl crate::viz::WidgetTraitConst for WSphere {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WSphere {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WSphere {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WSphere {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WSphereTraitConst for WSphere {
	#[inline] fn as_raw_WSphere(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WSphereTrait for WSphere {
	#[inline] fn as_raw_mut_WSphere(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WSphere {
	/// Constructs a WSphere.
	/// 
	/// ## Parameters
	/// * center: Center of the sphere.
	/// * radius: Radius of the sphere.
	/// * sphere_resolution: Resolution of the sphere.
	/// * color: Color of the sphere.
	/// 
	/// ## C++ default parameters
	/// * sphere_resolution: 10
	/// * color: Color::white()
	#[inline]
	pub fn new(center: core::Point3d, radius: f64, sphere_resolution: i32, color: &crate::viz::Color) -> Result<crate::viz::WSphere> {
		unsafe { sys::cv_viz_WSphere_WSphere_const_Point3dR_double_int_const_ColorR(&center, radius, sphere_resolution, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WSphere::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WSphere, crate::viz::Widget, cv_WSphere_to_Widget }

boxed_cast_base! { WSphere, crate::viz::Widget3D, cv_WSphere_to_Widget3D }

/// This 2D Widget represents text overlay.
pub trait WTextTraitConst: crate::viz::Widget2DTraitConst {
	fn as_raw_WText(&self) -> *const c_void;

	/// Returns the current text content of the widget.
	#[inline]
	fn get_text(&self) -> Result<String> {
		unsafe { sys::cv_viz_WText_getText_const(self.as_raw_WText()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
}

pub trait WTextTrait: crate::viz::WTextTraitConst + crate::viz::Widget2DTrait {
	fn as_raw_mut_WText(&mut self) -> *mut c_void;

	/// Sets the text content of the widget.
	/// 
	/// ## Parameters
	/// * text: Text content of the widget.
	#[inline]
	fn set_text(&mut self, text: &str) -> Result<()> {
		extern_container_arg!(text);
		unsafe { sys::cv_viz_WText_setText_const_StringR(self.as_raw_mut_WText(), text.opencv_as_extern()) }.into_result()
	}
	
}

/// This 2D Widget represents text overlay.
pub struct WText {
	ptr: *mut c_void
}

opencv_type_boxed! { WText }

impl Drop for WText {
	fn drop(&mut self) {
		extern "C" { fn cv_WText_delete(instance: *mut c_void); }
		unsafe { cv_WText_delete(self.as_raw_mut_WText()) };
	}
}

unsafe impl Send for WText {}

impl crate::viz::WidgetTraitConst for WText {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WText {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget2DTraitConst for WText {
	#[inline] fn as_raw_Widget2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget2DTrait for WText {
	#[inline] fn as_raw_mut_Widget2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WTextTraitConst for WText {
	#[inline] fn as_raw_WText(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WTextTrait for WText {
	#[inline] fn as_raw_mut_WText(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WText {
	/// Constructs a WText.
	/// 
	/// ## Parameters
	/// * text: Text content of the widget.
	/// * pos: Position of the text.
	/// * font_size: Font size.
	/// * color: Color of the text.
	/// 
	/// ## C++ default parameters
	/// * font_size: 20
	/// * color: Color::white()
	#[inline]
	pub fn new(text: &str, pos: core::Point, font_size: i32, color: &crate::viz::Color) -> Result<crate::viz::WText> {
		extern_container_arg!(text);
		unsafe { sys::cv_viz_WText_WText_const_StringR_const_PointR_int_const_ColorR(text.opencv_as_extern(), &pos, font_size, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WText::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WText, crate::viz::Widget, cv_WText_to_Widget }

boxed_cast_base! { WText, crate::viz::Widget2D, cv_WText_to_Widget2D }

/// This 3D Widget represents 3D text. The text always faces the camera.
pub trait WText3DTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WText3D(&self) -> *const c_void;

	/// Returns the current text content of the widget.
	#[inline]
	fn get_text(&self) -> Result<String> {
		unsafe { sys::cv_viz_WText3D_getText_const(self.as_raw_WText3D()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
}

pub trait WText3DTrait: crate::viz::WText3DTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WText3D(&mut self) -> *mut c_void;

	/// Sets the text content of the widget.
	/// 
	/// ## Parameters
	/// * text: Text content of the widget.
	#[inline]
	fn set_text(&mut self, text: &str) -> Result<()> {
		extern_container_arg!(text);
		unsafe { sys::cv_viz_WText3D_setText_const_StringR(self.as_raw_mut_WText3D(), text.opencv_as_extern()) }.into_result()
	}
	
}

/// This 3D Widget represents 3D text. The text always faces the camera.
pub struct WText3D {
	ptr: *mut c_void
}

opencv_type_boxed! { WText3D }

impl Drop for WText3D {
	fn drop(&mut self) {
		extern "C" { fn cv_WText3D_delete(instance: *mut c_void); }
		unsafe { cv_WText3D_delete(self.as_raw_mut_WText3D()) };
	}
}

unsafe impl Send for WText3D {}

impl crate::viz::WidgetTraitConst for WText3D {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WText3D {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WText3D {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WText3D {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WText3DTraitConst for WText3D {
	#[inline] fn as_raw_WText3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WText3DTrait for WText3D {
	#[inline] fn as_raw_mut_WText3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WText3D {
	/// Constructs a WText3D.
	/// 
	/// ## Parameters
	/// * text: Text content of the widget.
	/// * position: Position of the text.
	/// * text_scale: Size of the text.
	/// * face_camera: If true, text always faces the camera.
	/// * color: Color of the text.
	/// 
	/// ## C++ default parameters
	/// * text_scale: 1.
	/// * face_camera: true
	/// * color: Color::white()
	#[inline]
	pub fn new(text: &str, position: core::Point3d, text_scale: f64, face_camera: bool, color: &crate::viz::Color) -> Result<crate::viz::WText3D> {
		extern_container_arg!(text);
		unsafe { sys::cv_viz_WText3D_WText3D_const_StringR_const_Point3dR_double_bool_const_ColorR(text.opencv_as_extern(), &position, text_scale, face_camera, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WText3D::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WText3D, crate::viz::Widget, cv_WText3D_to_Widget }

boxed_cast_base! { WText3D, crate::viz::Widget3D, cv_WText3D_to_Widget3D }

/// This 3D Widget represents a trajectory. :
pub trait WTrajectoryTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WTrajectory(&self) -> *const c_void;

}

pub trait WTrajectoryTrait: crate::viz::WTrajectoryTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WTrajectory(&mut self) -> *mut c_void;

}

/// This 3D Widget represents a trajectory. :
pub struct WTrajectory {
	ptr: *mut c_void
}

opencv_type_boxed! { WTrajectory }

impl Drop for WTrajectory {
	fn drop(&mut self) {
		extern "C" { fn cv_WTrajectory_delete(instance: *mut c_void); }
		unsafe { cv_WTrajectory_delete(self.as_raw_mut_WTrajectory()) };
	}
}

unsafe impl Send for WTrajectory {}

impl crate::viz::WidgetTraitConst for WTrajectory {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WTrajectory {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WTrajectory {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WTrajectory {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WTrajectoryTraitConst for WTrajectory {
	#[inline] fn as_raw_WTrajectory(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WTrajectoryTrait for WTrajectory {
	#[inline] fn as_raw_mut_WTrajectory(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WTrajectory {
	/// Constructs a WTrajectory.
	/// 
	/// ## Parameters
	/// * path: List of poses on a trajectory. Takes std::vector\<Affine\<T\>\> with T == [float | double]
	/// * display_mode: Display mode. This can be PATH, FRAMES, and BOTH.
	/// * scale: Scale of the frames. Polyline is not affected.
	/// * color: Color of the polyline that represents path.
	/// 
	/// Frames are not affected.
	/// Displays trajectory of the given path as follows:
	/// *   PATH : Displays a poly line that represents the path.
	/// *   FRAMES : Displays coordinate frames at each pose.
	/// *   PATH & FRAMES : Displays both poly line and coordinate frames.
	/// 
	/// ## C++ default parameters
	/// * display_mode: WTrajectory::PATH
	/// * scale: 1.0
	/// * color: Color::white()
	#[inline]
	pub fn new(path: &dyn core::ToInputArray, display_mode: i32, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WTrajectory> {
		input_array_arg!(path);
		unsafe { sys::cv_viz_WTrajectory_WTrajectory_const__InputArrayR_int_double_const_ColorR(path.as_raw__InputArray(), display_mode, scale, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WTrajectory::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WTrajectory, crate::viz::Widget, cv_WTrajectory_to_Widget }

boxed_cast_base! { WTrajectory, crate::viz::Widget3D, cv_WTrajectory_to_Widget3D }

/// This 3D Widget represents a trajectory. :
pub trait WTrajectoryFrustumsTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WTrajectoryFrustums(&self) -> *const c_void;

}

pub trait WTrajectoryFrustumsTrait: crate::viz::WTrajectoryFrustumsTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WTrajectoryFrustums(&mut self) -> *mut c_void;

}

/// This 3D Widget represents a trajectory. :
pub struct WTrajectoryFrustums {
	ptr: *mut c_void
}

opencv_type_boxed! { WTrajectoryFrustums }

impl Drop for WTrajectoryFrustums {
	fn drop(&mut self) {
		extern "C" { fn cv_WTrajectoryFrustums_delete(instance: *mut c_void); }
		unsafe { cv_WTrajectoryFrustums_delete(self.as_raw_mut_WTrajectoryFrustums()) };
	}
}

unsafe impl Send for WTrajectoryFrustums {}

impl crate::viz::WidgetTraitConst for WTrajectoryFrustums {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WTrajectoryFrustums {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WTrajectoryFrustums {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WTrajectoryFrustums {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WTrajectoryFrustumsTraitConst for WTrajectoryFrustums {
	#[inline] fn as_raw_WTrajectoryFrustums(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WTrajectoryFrustumsTrait for WTrajectoryFrustums {
	#[inline] fn as_raw_mut_WTrajectoryFrustums(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WTrajectoryFrustums {
	/// Constructs a WTrajectoryFrustums.
	/// 
	/// ## Parameters
	/// * path: List of poses on a trajectory. Takes std::vector\<Affine\<T\>\> with T == [float | double]
	/// * K: Intrinsic matrix of the camera.
	/// * scale: Scale of the frustums.
	/// * color: Color of the frustums.
	/// 
	/// Displays frustums at each pose of the trajectory.
	/// 
	/// ## C++ default parameters
	/// * scale: 1.
	/// * color: Color::white()
	#[inline]
	pub fn new(path: &dyn core::ToInputArray, k: core::Matx33d, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WTrajectoryFrustums> {
		input_array_arg!(path);
		unsafe { sys::cv_viz_WTrajectoryFrustums_WTrajectoryFrustums_const__InputArrayR_const_Matx33dR_double_const_ColorR(path.as_raw__InputArray(), &k, scale, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WTrajectoryFrustums::opencv_from_extern(r) } )
	}
	
	/// Constructs a WTrajectoryFrustums.
	/// 
	/// ## Parameters
	/// * path: List of poses on a trajectory. Takes std::vector\<Affine\<T\>\> with T == [float | double]
	/// * fov: Field of view of the camera (horizontal, vertical).
	/// * scale: Scale of the frustums.
	/// * color: Color of the frustums.
	/// 
	/// Displays frustums at each pose of the trajectory.
	/// 
	/// ## C++ default parameters
	/// * scale: 1.
	/// * color: Color::white()
	#[inline]
	pub fn new_1(path: &dyn core::ToInputArray, fov: core::Vec2d, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WTrajectoryFrustums> {
		input_array_arg!(path);
		unsafe { sys::cv_viz_WTrajectoryFrustums_WTrajectoryFrustums_const__InputArrayR_const_Vec2dR_double_const_ColorR(path.as_raw__InputArray(), &fov, scale, color.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WTrajectoryFrustums::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WTrajectoryFrustums, crate::viz::Widget, cv_WTrajectoryFrustums_to_Widget }

boxed_cast_base! { WTrajectoryFrustums, crate::viz::Widget3D, cv_WTrajectoryFrustums_to_Widget3D }

/// This 3D Widget represents a trajectory using spheres and lines
/// 
/// where spheres represent the positions of the camera, and lines represent the direction from
/// previous position to the current. :
pub trait WTrajectorySpheresTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WTrajectorySpheres(&self) -> *const c_void;

}

pub trait WTrajectorySpheresTrait: crate::viz::WTrajectorySpheresTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WTrajectorySpheres(&mut self) -> *mut c_void;

}

/// This 3D Widget represents a trajectory using spheres and lines
/// 
/// where spheres represent the positions of the camera, and lines represent the direction from
/// previous position to the current. :
pub struct WTrajectorySpheres {
	ptr: *mut c_void
}

opencv_type_boxed! { WTrajectorySpheres }

impl Drop for WTrajectorySpheres {
	fn drop(&mut self) {
		extern "C" { fn cv_WTrajectorySpheres_delete(instance: *mut c_void); }
		unsafe { cv_WTrajectorySpheres_delete(self.as_raw_mut_WTrajectorySpheres()) };
	}
}

unsafe impl Send for WTrajectorySpheres {}

impl crate::viz::WidgetTraitConst for WTrajectorySpheres {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WTrajectorySpheres {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WTrajectorySpheres {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WTrajectorySpheres {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WTrajectorySpheresTraitConst for WTrajectorySpheres {
	#[inline] fn as_raw_WTrajectorySpheres(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WTrajectorySpheresTrait for WTrajectorySpheres {
	#[inline] fn as_raw_mut_WTrajectorySpheres(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WTrajectorySpheres {
	/// Constructs a WTrajectorySpheres.
	/// 
	/// ## Parameters
	/// * path: List of poses on a trajectory. Takes std::vector\<Affine\<T\>\> with T == [float | double]
	/// * line_length: Max length of the lines which point to previous position
	/// * radius: Radius of the spheres.
	/// * from: Color for first sphere.
	/// * to: Color for last sphere. Intermediate spheres will have interpolated color.
	/// 
	/// ## C++ default parameters
	/// * line_length: 0.05
	/// * radius: 0.007
	/// * from: Color::red()
	/// * to: Color::white()
	#[inline]
	pub fn new(path: &dyn core::ToInputArray, line_length: f64, radius: f64, from: &crate::viz::Color, to: &crate::viz::Color) -> Result<crate::viz::WTrajectorySpheres> {
		input_array_arg!(path);
		unsafe { sys::cv_viz_WTrajectorySpheres_WTrajectorySpheres_const__InputArrayR_double_double_const_ColorR_const_ColorR(path.as_raw__InputArray(), line_length, radius, from.as_raw_Color(), to.as_raw_Color()) }.into_result().map(|r| unsafe { crate::viz::WTrajectorySpheres::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WTrajectorySpheres, crate::viz::Widget, cv_WTrajectorySpheres_to_Widget }

boxed_cast_base! { WTrajectorySpheres, crate::viz::Widget3D, cv_WTrajectorySpheres_to_Widget3D }

/// This class allows to merge several widgets to single one.
/// 
/// It has quite limited functionality and can't merge widgets with different attributes. For
/// instance, if widgetA has color array and widgetB has only global color defined, then result
/// of merge won't have color at all. The class is suitable for merging large amount of similar
/// widgets. :
pub trait WWidgetMergerTraitConst: crate::viz::Widget3DTraitConst {
	fn as_raw_WWidgetMerger(&self) -> *const c_void;

}

pub trait WWidgetMergerTrait: crate::viz::WWidgetMergerTraitConst + crate::viz::Widget3DTrait {
	fn as_raw_mut_WWidgetMerger(&mut self) -> *mut c_void;

	/// Add widget to merge with optional position change
	/// 
	/// ## C++ default parameters
	/// * pose: Affine3d::Identity()
	#[inline]
	fn add_widget(&mut self, widget: &crate::viz::Widget3D, pose: core::Affine3d) -> Result<()> {
		unsafe { sys::cv_viz_WWidgetMerger_addWidget_const_Widget3DR_const_Affine3dR(self.as_raw_mut_WWidgetMerger(), widget.as_raw_Widget3D(), &pose) }.into_result()
	}
	
	/// Repacks internal structure to single widget
	#[inline]
	fn finalize(&mut self) -> Result<()> {
		unsafe { sys::cv_viz_WWidgetMerger_finalize(self.as_raw_mut_WWidgetMerger()) }.into_result()
	}
	
}

/// This class allows to merge several widgets to single one.
/// 
/// It has quite limited functionality and can't merge widgets with different attributes. For
/// instance, if widgetA has color array and widgetB has only global color defined, then result
/// of merge won't have color at all. The class is suitable for merging large amount of similar
/// widgets. :
pub struct WWidgetMerger {
	ptr: *mut c_void
}

opencv_type_boxed! { WWidgetMerger }

impl Drop for WWidgetMerger {
	fn drop(&mut self) {
		extern "C" { fn cv_WWidgetMerger_delete(instance: *mut c_void); }
		unsafe { cv_WWidgetMerger_delete(self.as_raw_mut_WWidgetMerger()) };
	}
}

unsafe impl Send for WWidgetMerger {}

impl crate::viz::WidgetTraitConst for WWidgetMerger {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for WWidgetMerger {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for WWidgetMerger {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for WWidgetMerger {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::WWidgetMergerTraitConst for WWidgetMerger {
	#[inline] fn as_raw_WWidgetMerger(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WWidgetMergerTrait for WWidgetMerger {
	#[inline] fn as_raw_mut_WWidgetMerger(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WWidgetMerger {
	#[inline]
	pub fn default() -> Result<crate::viz::WWidgetMerger> {
		unsafe { sys::cv_viz_WWidgetMerger_WWidgetMerger() }.into_result().map(|r| unsafe { crate::viz::WWidgetMerger::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { WWidgetMerger, crate::viz::Widget, cv_WWidgetMerger_to_Widget }

boxed_cast_base! { WWidgetMerger, crate::viz::Widget3D, cv_WWidgetMerger_to_Widget3D }

/// Base class of all widgets. Widget is implicitly shared.
pub trait WidgetTraitConst {
	fn as_raw_Widget(&self) -> *const c_void;

	/// Returns rendering property of the widget.
	/// 
	/// ## Parameters
	/// * property: Property.
	/// 
	/// Rendering property can be one of the following:
	/// *   **POINT_SIZE**
	/// *   **OPACITY**
	/// *   **LINE_WIDTH**
	/// *   **FONT_SIZE**
	/// *   **AMBIENT**
	/// 
	/// REPRESENTATION: Expected values are
	/// *   **REPRESENTATION_POINTS**
	/// *   **REPRESENTATION_WIREFRAME**
	/// *   **REPRESENTATION_SURFACE**
	/// 
	/// **IMMEDIATE_RENDERING**:
	/// *   Turn on immediate rendering by setting the value to 1.
	/// *   Turn off immediate rendering by setting the value to 0.
	/// 
	/// SHADING: Expected values are
	/// *   **SHADING_FLAT**
	/// *   **SHADING_GOURAUD**
	/// *   **SHADING_PHONG**
	#[inline]
	fn get_rendering_property(&self, property: i32) -> Result<f64> {
		unsafe { sys::cv_viz_Widget_getRenderingProperty_const_int(self.as_raw_Widget(), property) }.into_result()
	}
	
}

pub trait WidgetTrait: crate::viz::WidgetTraitConst {
	fn as_raw_mut_Widget(&mut self) -> *mut c_void;

	/// Sets rendering property of the widget.
	/// 
	/// ## Parameters
	/// * property: Property that will be modified.
	/// * value: The new value of the property.
	/// 
	/// Rendering property can be one of the following:
	/// *   **POINT_SIZE**
	/// *   **OPACITY**
	/// *   **LINE_WIDTH**
	/// *   **FONT_SIZE**
	/// 
	/// REPRESENTATION: Expected values are
	/// *   **REPRESENTATION_POINTS**
	/// *   **REPRESENTATION_WIREFRAME**
	/// *   **REPRESENTATION_SURFACE**
	/// 
	/// IMMEDIATE_RENDERING:
	/// *   Turn on immediate rendering by setting the value to 1.
	/// *   Turn off immediate rendering by setting the value to 0.
	/// 
	/// SHADING: Expected values are
	/// *   **SHADING_FLAT**
	/// *   **SHADING_GOURAUD**
	/// *   **SHADING_PHONG**
	#[inline]
	fn set_rendering_property(&mut self, property: i32, value: f64) -> Result<()> {
		unsafe { sys::cv_viz_Widget_setRenderingProperty_int_double(self.as_raw_mut_Widget(), property, value) }.into_result()
	}
	
}

/// Base class of all widgets. Widget is implicitly shared.
pub struct Widget {
	ptr: *mut c_void
}

opencv_type_boxed! { Widget }

impl Drop for Widget {
	fn drop(&mut self) {
		extern "C" { fn cv_Widget_delete(instance: *mut c_void); }
		unsafe { cv_Widget_delete(self.as_raw_mut_Widget()) };
	}
}

unsafe impl Send for Widget {}

impl crate::viz::WidgetTraitConst for Widget {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for Widget {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Widget {
	#[inline]
	pub fn default() -> Result<crate::viz::Widget> {
		unsafe { sys::cv_viz_Widget_Widget() }.into_result().map(|r| unsafe { crate::viz::Widget::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn copy(other: &crate::viz::Widget) -> Result<crate::viz::Widget> {
		unsafe { sys::cv_viz_Widget_Widget_const_WidgetR(other.as_raw_Widget()) }.into_result().map(|r| unsafe { crate::viz::Widget::opencv_from_extern(r) } )
	}
	
	/// Creates a widget from ply file.
	/// 
	/// ## Parameters
	/// * file_name: Ply file name.
	#[inline]
	pub fn from_ply_file(file_name: &str) -> Result<crate::viz::Widget> {
		extern_container_arg!(file_name);
		unsafe { sys::cv_viz_Widget_fromPlyFile_const_StringR(file_name.opencv_as_extern()) }.into_result().map(|r| unsafe { crate::viz::Widget::opencv_from_extern(r) } )
	}
	
}

/// Base class of all 2D widgets.
pub trait Widget2DTraitConst: crate::viz::WidgetTraitConst {
	fn as_raw_Widget2D(&self) -> *const c_void;

}

pub trait Widget2DTrait: crate::viz::Widget2DTraitConst + crate::viz::WidgetTrait {
	fn as_raw_mut_Widget2D(&mut self) -> *mut c_void;

	/// Sets the color of the widget.
	/// 
	/// ## Parameters
	/// * color: color of type Color
	#[inline]
	fn set_color(&mut self, color: &crate::viz::Color) -> Result<()> {
		unsafe { sys::cv_viz_Widget2D_setColor_const_ColorR(self.as_raw_mut_Widget2D(), color.as_raw_Color()) }.into_result()
	}
	
}

/// Base class of all 2D widgets.
pub struct Widget2D {
	ptr: *mut c_void
}

opencv_type_boxed! { Widget2D }

impl Drop for Widget2D {
	fn drop(&mut self) {
		extern "C" { fn cv_Widget2D_delete(instance: *mut c_void); }
		unsafe { cv_Widget2D_delete(self.as_raw_mut_Widget2D()) };
	}
}

unsafe impl Send for Widget2D {}

impl crate::viz::WidgetTraitConst for Widget2D {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for Widget2D {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget2DTraitConst for Widget2D {
	#[inline] fn as_raw_Widget2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget2DTrait for Widget2D {
	#[inline] fn as_raw_mut_Widget2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Widget2D {
	#[inline]
	pub fn default() -> Result<crate::viz::Widget2D> {
		unsafe { sys::cv_viz_Widget2D_Widget2D() }.into_result().map(|r| unsafe { crate::viz::Widget2D::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { Widget2D, crate::viz::Widget, cv_Widget2D_to_Widget }

/// Base class of all 3D widgets.
pub trait Widget3DTraitConst: crate::viz::WidgetTraitConst {
	fn as_raw_Widget3D(&self) -> *const c_void;

	/// Returns the current pose of the widget.
	#[inline]
	fn get_pose(&self) -> Result<core::Affine3d> {
		unsafe { sys::cv_viz_Widget3D_getPose_const(self.as_raw_Widget3D()) }.into_result()
	}
	
}

pub trait Widget3DTrait: crate::viz::Widget3DTraitConst + crate::viz::WidgetTrait {
	fn as_raw_mut_Widget3D(&mut self) -> *mut c_void;

	/// Sets pose of the widget.
	/// 
	/// ## Parameters
	/// * pose: The new pose of the widget.
	#[inline]
	fn set_pose(&mut self, pose: core::Affine3d) -> Result<()> {
		unsafe { sys::cv_viz_Widget3D_setPose_const_Affine3dR(self.as_raw_mut_Widget3D(), &pose) }.into_result()
	}
	
	/// Updates pose of the widget by pre-multiplying its current pose.
	/// 
	/// ## Parameters
	/// * pose: The pose that the current pose of the widget will be pre-multiplied by.
	#[inline]
	fn update_pose(&mut self, pose: core::Affine3d) -> Result<()> {
		unsafe { sys::cv_viz_Widget3D_updatePose_const_Affine3dR(self.as_raw_mut_Widget3D(), &pose) }.into_result()
	}
	
	/// Transforms internal widget data (i.e. points, normals) using the given transform.
	/// 
	/// ## Parameters
	/// * transform: Specified transformation to apply.
	#[inline]
	fn apply_transform(&mut self, transform: core::Affine3d) -> Result<()> {
		unsafe { sys::cv_viz_Widget3D_applyTransform_const_Affine3dR(self.as_raw_mut_Widget3D(), &transform) }.into_result()
	}
	
	/// Sets the color of the widget.
	/// 
	/// ## Parameters
	/// * color: color of type Color
	#[inline]
	fn set_color(&mut self, color: &crate::viz::Color) -> Result<()> {
		unsafe { sys::cv_viz_Widget3D_setColor_const_ColorR(self.as_raw_mut_Widget3D(), color.as_raw_Color()) }.into_result()
	}
	
}

/// Base class of all 3D widgets.
pub struct Widget3D {
	ptr: *mut c_void
}

opencv_type_boxed! { Widget3D }

impl Drop for Widget3D {
	fn drop(&mut self) {
		extern "C" { fn cv_Widget3D_delete(instance: *mut c_void); }
		unsafe { cv_Widget3D_delete(self.as_raw_mut_Widget3D()) };
	}
}

unsafe impl Send for Widget3D {}

impl crate::viz::WidgetTraitConst for Widget3D {
	#[inline] fn as_raw_Widget(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::WidgetTrait for Widget3D {
	#[inline] fn as_raw_mut_Widget(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::viz::Widget3DTraitConst for Widget3D {
	#[inline] fn as_raw_Widget3D(&self) -> *const c_void { self.as_raw() }
}

impl crate::viz::Widget3DTrait for Widget3D {
	#[inline] fn as_raw_mut_Widget3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Widget3D {
	#[inline]
	pub fn default() -> Result<crate::viz::Widget3D> {
		unsafe { sys::cv_viz_Widget3D_Widget3D() }.into_result().map(|r| unsafe { crate::viz::Widget3D::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { Widget3D, crate::viz::Widget, cv_Widget3D_to_Widget }
