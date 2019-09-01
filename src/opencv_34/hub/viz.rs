//! # 3D Visualizer
//!
//! This section describes 3D visualization window as well as classes and methods that are used to
//! interact with it.
//!
//! 3D visualization window (see Viz3d) is used to display widgets (see Widget), and it provides several
//! methods to interact with scene and widgets.
//! # Widget
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
use std::os::raw::{c_char, c_void};
use libc::{ptrdiff_t, size_t};
use crate::{Error, Result, core, sys, types};

pub const AMBIENT: i32 = 7;
pub const FONT_SIZE: i32 = 3;
pub const IMMEDIATE_RENDERING: i32 = 5;
pub const KeyboardEvent_ALT: i32 = 1;
pub const KeyboardEvent_CTRL: i32 = 2;
pub const KeyboardEvent_KEY_DOWN: i32 = 1;
pub const KeyboardEvent_KEY_UP: i32 = 0;
pub const KeyboardEvent_NONE: i32 = 0;
pub const KeyboardEvent_SHIFT: i32 = 4;
pub const LIGHTING: i32 = 8;
pub const LINE_WIDTH: i32 = 2;
pub const Mesh_LOAD_AUTO: i32 = 0;
pub const Mesh_LOAD_OBJ: i32 = 2;
pub const Mesh_LOAD_PLY: i32 = 1;
pub const MouseEvent_LeftButton: i32 = 0+1;
pub const MouseEvent_MiddleButton: i32 = 0+2;
pub const MouseEvent_MouseButtonPress: i32 = 1+1;
pub const MouseEvent_MouseButtonRelease: i32 = 1+2;
pub const MouseEvent_MouseDblClick: i32 = 1+5;
pub const MouseEvent_MouseMove: i32 = 1;
pub const MouseEvent_MouseScrollDown: i32 = 1+3;
pub const MouseEvent_MouseScrollUp: i32 = 1+4;
pub const MouseEvent_NoButton: i32 = 0;
pub const MouseEvent_RightButton: i32 = 0+3;
pub const MouseEvent_VScroll: i32 = 0+4;
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
pub const WTrajectory_FRAMES: i32 = 1;
pub const WTrajectory_PATH: i32 = 2;

pub type KeyboardCallback = dyn FnMut(crate::viz::KeyboardEvent, &mut c_void) + Send + Sync + 'static;
#[doc(hidden)] pub type KeyboardCallbackExtern = Option<extern "C" fn(unnamed_arg: *mut c_void, unnamed_arg_1: *mut c_void)>;

pub type MouseCallback = dyn FnMut(crate::viz::MouseEvent, &mut c_void) + Send + Sync + 'static;
#[doc(hidden)] pub type MouseCallbackExtern = Option<extern "C" fn(unnamed_arg: *mut c_void, unnamed_arg_1: *mut c_void)>;

pub fn vec3b() -> Result<()> {
    unsafe { sys::cv_Vec3b_const() }.into_result()
}

/// Computing normals for mesh
/// ## Parameters
/// * mesh: Input mesh.
/// * normals: Normals at very point in the mesh of type CV_64FC3.
pub fn compute_normals(mesh: &crate::viz::Mesh, normals: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_viz_computeNormals_Mesh_Mat(mesh.as_raw_Mesh(), normals.as_raw_Mat()) }.into_result()
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
///
///
/// Note: Window names are automatically prefixed by "Viz - " if it is not done by the user.
/// ```ignore
/// /// window and window_2 are the same windows.
/// viz::Viz3d window   = viz::getWindowByName("myWindow");
/// viz::Viz3d window_2 = viz::getWindowByName("Viz - myWindow");
/// ```
pub fn get_window_by_name(window_name: &str) -> Result<crate::viz::Viz3d> {
    string_arg!(window_name);
    unsafe { sys::cv_viz_getWindowByName_String(window_name.as_ptr()) }.into_result().map(|ptr| crate::viz::Viz3d { ptr })
}

/// Displays image in specified window
///
/// ## C++ default parameters
/// * window_size: Size(-1, -1)
pub fn imshow(window_name: &str, image: &core::Mat, window_size: core::Size) -> Result<crate::viz::Viz3d> {
    string_arg!(window_name);
    unsafe { sys::cv_viz_imshow_String_Mat_Size(window_name.as_ptr(), image.as_raw_Mat(), window_size) }.into_result().map(|ptr| crate::viz::Viz3d { ptr })
}

/// Checks **float/double** value for nan.
///
/// ## Parameters
/// * x: return true if nan.
pub fn is_nan(x: f64) -> Result<bool> {
    unsafe { sys::cv_viz_isNan_double(x) }.into_result()
}

/// Checks **float/double** value for nan.
///
/// ## Parameters
/// * x: return true if nan.
pub fn is_nan_1(x: f32) -> Result<bool> {
    unsafe { sys::cv_viz_isNan_float(x) }.into_result()
}

/// ## Parameters
/// * file: Filename with extension. Supported formats: PLY, XYZ, OBJ and STL.
/// * colors: Used by PLY and STL formats only.
/// * normals: Used by PLY, OBJ and STL formats only.
/// ## Returns
/// A mat containing the point coordinates with depth CV_32F or CV_64F and number of
///   channels 3 or 4 with only 1 row.
///
/// ## C++ default parameters
/// * colors: noArray()
/// * normals: noArray()
pub fn read_cloud(file: &str, colors: &mut core::Mat, normals: &mut core::Mat) -> Result<core::Mat> {
    string_arg!(file);
    unsafe { sys::cv_viz_readCloud_String_Mat_Mat(file.as_ptr(), colors.as_raw_Mat(), normals.as_raw_Mat()) }.into_result().map(|ptr| core::Mat { ptr })
}

pub fn read_mesh(file: &str) -> Result<crate::viz::Mesh> {
    string_arg!(file);
    unsafe { sys::cv_viz_readMesh_String(file.as_ptr()) }.into_result().map(|ptr| crate::viz::Mesh { ptr })
}

/// takes vector<Affine3<T>> with T = float/dobule and loads poses from sequence of files
///
/// ## Parameters
/// * traj: Output array containing a lists of poses. It can be
///    - std::vector<cv::Affine3f>, std::vector<cv::Affine3d>
///    - cv::Mat
/// * files_format: Format specifier string for constructing filenames.
///      The only placeholder in the string should support `int`.
/// * start: The initial counter for files_format. It must be greater than or equal to 0.
/// * end: The final  counter for files_format.
/// * tag: Name of the matrix in the file.
///
/// ## C++ default parameters
/// * files_format: "pose%05d.xml"
/// * start: 0
/// * end: INT_MAX
/// * tag: "pose"
pub fn read_trajectory(traj: &mut core::Mat, files_format: &str, start: i32, end: i32, tag: &str) -> Result<()> {
    string_arg!(files_format);
    string_arg!(tag);
    unsafe { sys::cv_viz_readTrajectory_Mat_String_int_int_String(traj.as_raw_Mat(), files_format.as_ptr(), start, end, tag.as_ptr()) }.into_result()
}

/// Unregisters all Viz windows from internal database. After it 'getWindowByName()' will create new windows instead of getting existing from the database.
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
pub fn write_cloud(file: &str, cloud: &core::Mat, colors: &core::Mat, normals: &core::Mat, binary: bool) -> Result<()> {
    string_arg!(file);
    unsafe { sys::cv_viz_writeCloud_String_Mat_Mat_Mat_bool(file.as_ptr(), cloud.as_raw_Mat(), colors.as_raw_Mat(), normals.as_raw_Mat(), binary) }.into_result()
}

/// takes vector<Affine3<T>> with T = float/dobule and writes to a sequence of files with given filename format
/// ## Parameters
/// * traj: Trajectory containing a list of poses. It can be
///          - std::vector<cv::Mat>, each cv::Mat is of type CV_32F16 or CV_64FC16
///          - std::vector<cv::Affine3f>, std::vector<cv::Affine3d>
///          - cv::Mat of type CV_32FC16 OR CV_64F16
/// * files_format: Format specifier string for constructing filenames.
///      The only placeholder in the string should support `int`.
/// * start: The initial counter for files_format.
/// * tag: Name of the matrix in the file.
///
/// ## C++ default parameters
/// * files_format: "pose%05d.xml"
/// * start: 0
/// * tag: "pose"
pub fn write_trajectory(traj: &core::Mat, files_format: &str, start: i32, tag: &str) -> Result<()> {
    string_arg!(files_format);
    string_arg!(tag);
    unsafe { sys::cv_viz_writeTrajectory_Mat_String_int_String(traj.as_raw_Mat(), files_format.as_ptr(), start, tag.as_ptr()) }.into_result()
}

// boxed class cv::viz::Camera
/// This class wraps intrinsic parameters of a camera.
///
/// It provides several constructors that can extract the intrinsic parameters from field of
/// view, intrinsic matrix and projection matrix. :
pub struct Camera {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::Camera {
    fn drop(&mut self) {
        unsafe { sys::cv_Camera_delete(self.ptr) };
    }
}
impl crate::viz::Camera {
    #[inline(always)] pub fn as_raw_Camera(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for Camera {}

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
    pub fn new(fx: f64, fy: f64, cx: f64, cy: f64, window_size: core::Size) -> Result<crate::viz::Camera> {
        unsafe { sys::cv_viz_Camera_Camera_double_double_double_double_Size(fx, fy, cx, cy, window_size) }.into_result().map(|ptr| crate::viz::Camera { ptr })
    }
    
    /// ## Parameters
    /// * fov: Field of view (horizontal, vertical)
    /// * window_size: Size of the window. Principal point is at the center of the window
    /// by default.
    pub fn new_1(fov: core::Vec2d, window_size: core::Size) -> Result<crate::viz::Camera> {
        unsafe { sys::cv_viz_Camera_Camera_Vec2d_Size(fov, window_size) }.into_result().map(|ptr| crate::viz::Camera { ptr })
    }
    
    pub fn get_clip(&self) -> Result<core::Vec2d> {
        unsafe { sys::cv_viz_Camera_getClip_const(self.as_raw_Camera()) }.into_result()
    }
    
    pub fn set_clip(&mut self, clip: core::Vec2d) -> Result<()> {
        unsafe { sys::cv_viz_Camera_setClip_Vec2d(self.as_raw_Camera(), clip) }.into_result()
    }
    
    pub fn get_window_size(&self) -> Result<core::Size> {
        unsafe { sys::cv_viz_Camera_getWindowSize_const(self.as_raw_Camera()) }.into_result()
    }
    
    pub fn set_window_size(&mut self, window_size: core::Size) -> Result<()> {
        unsafe { sys::cv_viz_Camera_setWindowSize_Size(self.as_raw_Camera(), window_size) }.into_result()
    }
    
    pub fn get_fov(&self) -> Result<core::Vec2d> {
        unsafe { sys::cv_viz_Camera_getFov_const(self.as_raw_Camera()) }.into_result()
    }
    
    pub fn set_fov(&mut self, fov: core::Vec2d) -> Result<()> {
        unsafe { sys::cv_viz_Camera_setFov_Vec2d(self.as_raw_Camera(), fov) }.into_result()
    }
    
    pub fn get_principal_point(&self) -> Result<core::Vec2d> {
        unsafe { sys::cv_viz_Camera_getPrincipalPoint_const(self.as_raw_Camera()) }.into_result()
    }
    
    pub fn get_focal_length(&self) -> Result<core::Vec2d> {
        unsafe { sys::cv_viz_Camera_getFocalLength_const(self.as_raw_Camera()) }.into_result()
    }
    
    /// Creates a Kinect Camera with
    /// - fx = fy = 525
    /// - cx = 320
    /// - cy = 240
    ///
    /// ## Parameters
    /// * window_size: Size of the window. This together with intrinsic matrix of a Kinect Camera
    /// determines the field of view.
    pub fn kinect_camera(window_size: core::Size) -> Result<crate::viz::Camera> {
        unsafe { sys::cv_viz_Camera_KinectCamera_Size(window_size) }.into_result().map(|ptr| crate::viz::Camera { ptr })
    }
    
}

// boxed class cv::viz::Color
/// This class represents color in BGR order.
pub struct Color {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::Color {
    fn drop(&mut self) {
        unsafe { sys::cv_Color_delete(self.ptr) };
    }
}
impl crate::viz::Color {
    #[inline(always)] pub fn as_raw_Color(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for Color {}

impl Color {

    pub fn default() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_Color() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    /// The three channels will have the same value equal to gray.
    pub fn new(gray: f64) -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_Color_double(gray) }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn new_1(blue: f64, green: f64, red: f64) -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_Color_double_double_double(blue, green, red) }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn new_2(color: core::Scalar) -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_Color_Scalar(color) }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn to_vec3b(&self) -> Result<core::Vec3b> {
        unsafe { sys::cv_viz_Color_operator_Vec3b_const(self.as_raw_Color()) }.into_result()
    }
    
    pub fn black() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_black() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn blue() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_blue() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn green() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_green() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn cyan() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_cyan() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn red() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_red() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn magenta() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_magenta() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn yellow() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_yellow() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn white() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_white() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn gray() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_gray() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn mlab() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_mlab() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn navy() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_navy() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn olive() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_olive() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn maroon() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_maroon() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn teal() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_teal() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn rose() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_rose() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn azure() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_azure() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn lime() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_lime() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn gold() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_gold() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn brown() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_brown() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn orange() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_orange() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn chartreuse() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_chartreuse() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn orange_red() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_orange_red() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn purple() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_purple() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn indigo() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_indigo() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn pink() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_pink() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn cherry() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_cherry() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn bluberry() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_bluberry() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn raspberry() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_raspberry() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn silver() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_silver() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn violet() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_violet() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn apricot() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_apricot() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn turquoise() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_turquoise() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn celestial_blue() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_celestial_blue() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn amethyst() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_amethyst() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
    pub fn not_set() -> Result<crate::viz::Color> {
        unsafe { sys::cv_viz_Color_not_set() }.into_result().map(|ptr| crate::viz::Color { ptr })
    }
    
}

// boxed class cv::viz::KeyboardEvent
/// This class represents a keyboard event.
pub struct KeyboardEvent {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::KeyboardEvent {
    fn drop(&mut self) {
        unsafe { sys::cv_KeyboardEvent_delete(self.ptr) };
    }
}
impl crate::viz::KeyboardEvent {
    #[inline(always)] pub fn as_raw_KeyboardEvent(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for KeyboardEvent {}

impl KeyboardEvent {

}

// boxed class cv::viz::Mesh
/// This class wraps mesh attributes, and it can load a mesh from a ply file. :
pub struct Mesh {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::Mesh {
    fn drop(&mut self) {
        unsafe { sys::cv_Mesh_delete(self.ptr) };
    }
}
impl crate::viz::Mesh {
    #[inline(always)] pub fn as_raw_Mesh(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for Mesh {}

impl Mesh {

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
    /// * _type: LOAD_PLY
    pub fn load(file: &str, _type: i32) -> Result<crate::viz::Mesh> {
        string_arg!(file);
        unsafe { sys::cv_viz_Mesh_load_String_int(file.as_ptr(), _type) }.into_result().map(|ptr| crate::viz::Mesh { ptr })
    }
    
}

// boxed class cv::viz::MouseEvent
/// This class represents a mouse event.
pub struct MouseEvent {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::MouseEvent {
    fn drop(&mut self) {
        unsafe { sys::cv_MouseEvent_delete(self.ptr) };
    }
}
impl crate::viz::MouseEvent {
    #[inline(always)] pub fn as_raw_MouseEvent(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for MouseEvent {}

impl MouseEvent {

}

// boxed class cv::viz::Viz3d
/// The Viz3d class represents a 3D visualizer window. This class is implicitly shared.
pub struct Viz3d {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::Viz3d {
    fn drop(&mut self) {
        unsafe { sys::cv_Viz3d_delete(self.ptr) };
    }
}
impl crate::viz::Viz3d {
    #[inline(always)] pub fn as_raw_Viz3d(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for Viz3d {}

impl Viz3d {

    /// The constructors.
    ///
    /// ## Parameters
    /// * window_name: Name of the window.
    ///
    /// ## C++ default parameters
    /// * window_name: String()
    pub fn new(window_name: &str) -> Result<crate::viz::Viz3d> {
        string_arg!(window_name);
        unsafe { sys::cv_viz_Viz3d_Viz3d_String(window_name.as_ptr()) }.into_result().map(|ptr| crate::viz::Viz3d { ptr })
    }
    
    pub fn copy(unnamed_arg: &crate::viz::Viz3d) -> Result<crate::viz::Viz3d> {
        unsafe { sys::cv_viz_Viz3d_Viz3d_Viz3d(unnamed_arg.as_raw_Viz3d()) }.into_result().map(|ptr| crate::viz::Viz3d { ptr })
    }
    
    /// Removes a widget from the window.
    ///
    /// ## Parameters
    /// * id: The id of the widget that will be removed.
    pub fn remove_widget(&mut self, id: &str) -> Result<()> {
        string_arg!(id);
        unsafe { sys::cv_viz_Viz3d_removeWidget_String(self.as_raw_Viz3d(), id.as_ptr()) }.into_result()
    }
    
    /// Removes all widgets from the window.
    pub fn remove_all_widgets(&mut self) -> Result<()> {
        unsafe { sys::cv_viz_Viz3d_removeAllWidgets(self.as_raw_Viz3d()) }.into_result()
    }
    
    /// Removed all widgets and displays image scaled to whole window area.
    ///
    /// ## Parameters
    /// * image: Image to be displayed.
    /// * window_size: Size of Viz3d window. Default value means no change.
    ///
    /// ## C++ default parameters
    /// * window_size: Size(-1, -1)
    pub fn show_image(&mut self, image: &core::Mat, window_size: core::Size) -> Result<()> {
        unsafe { sys::cv_viz_Viz3d_showImage_Mat_Size(self.as_raw_Viz3d(), image.as_raw_Mat(), window_size) }.into_result()
    }
    
    /// Sets the intrinsic parameters of the viewer using Camera.
    ///
    /// ## Parameters
    /// * camera: Camera object wrapping intrinsic parameters.
    pub fn set_camera(&mut self, camera: &crate::viz::Camera) -> Result<()> {
        unsafe { sys::cv_viz_Viz3d_setCamera_Camera(self.as_raw_Viz3d(), camera.as_raw_Camera()) }.into_result()
    }
    
    /// Returns a camera object that contains intrinsic parameters of the current viewer.
    pub fn get_camera(&self) -> Result<crate::viz::Camera> {
        unsafe { sys::cv_viz_Viz3d_getCamera_const(self.as_raw_Viz3d()) }.into_result().map(|ptr| crate::viz::Camera { ptr })
    }
    
    /// Resets camera viewpoint to a 3D widget in the scene.
    ///
    /// ## Parameters
    /// * id: Id of a 3D widget.
    pub fn reset_camera_viewpoint(&mut self, id: &str) -> Result<()> {
        string_arg!(id);
        unsafe { sys::cv_viz_Viz3d_resetCameraViewpoint_String(self.as_raw_Viz3d(), id.as_ptr()) }.into_result()
    }
    
    /// Resets camera.
    pub fn reset_camera(&mut self) -> Result<()> {
        unsafe { sys::cv_viz_Viz3d_resetCamera(self.as_raw_Viz3d()) }.into_result()
    }
    
    /// Returns the current size of the window.
    pub fn get_window_size(&self) -> Result<core::Size> {
        unsafe { sys::cv_viz_Viz3d_getWindowSize_const(self.as_raw_Viz3d()) }.into_result()
    }
    
    /// Sets the size of the window.
    ///
    /// ## Parameters
    /// * window_size: New size of the window.
    pub fn set_window_size(&mut self, window_size: core::Size) -> Result<()> {
        unsafe { sys::cv_viz_Viz3d_setWindowSize_Size(self.as_raw_Viz3d(), window_size) }.into_result()
    }
    
    /// Returns the name of the window which has been set in the constructor.
    ///  `Viz - ` is prepended to the name if necessary.
    pub fn get_window_name(&self) -> Result<String> {
        unsafe { sys::cv_viz_Viz3d_getWindowName_const(self.as_raw_Viz3d()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
    /// Returns the Mat screenshot of the current scene.
    pub fn get_screenshot(&self) -> Result<core::Mat> {
        unsafe { sys::cv_viz_Viz3d_getScreenshot_const(self.as_raw_Viz3d()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Saves screenshot of the current scene.
    ///
    /// ## Parameters
    /// * file: Name of the file.
    pub fn save_screenshot(&mut self, file: &str) -> Result<()> {
        string_arg!(file);
        unsafe { sys::cv_viz_Viz3d_saveScreenshot_String(self.as_raw_Viz3d(), file.as_ptr()) }.into_result()
    }
    
    /// Sets the position of the window in the screen.
    ///
    /// ## Parameters
    /// * window_position: coordinates of the window
    pub fn set_window_position(&mut self, window_position: core::Point) -> Result<()> {
        unsafe { sys::cv_viz_Viz3d_setWindowPosition_Point(self.as_raw_Viz3d(), window_position) }.into_result()
    }
    
    /// Sets or unsets full-screen rendering mode.
    ///
    /// ## Parameters
    /// * mode: If true, window will use full-screen mode.
    ///
    /// ## C++ default parameters
    /// * mode: true
    pub fn set_full_screen(&mut self, mode: bool) -> Result<()> {
        unsafe { sys::cv_viz_Viz3d_setFullScreen_bool(self.as_raw_Viz3d(), mode) }.into_result()
    }
    
    /// Sets background color.
    ///
    /// ## C++ default parameters
    /// * color: Color::black()
    /// * color2: Color::not_set()
    pub fn set_background_color(&mut self, color: &crate::viz::Color, color2: &crate::viz::Color) -> Result<()> {
        unsafe { sys::cv_viz_Viz3d_setBackgroundColor_Color_Color(self.as_raw_Viz3d(), color.as_raw_Color(), color2.as_raw_Color()) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * image: noArray()
    pub fn set_background_texture(&mut self, image: &core::Mat) -> Result<()> {
        unsafe { sys::cv_viz_Viz3d_setBackgroundTexture_Mat(self.as_raw_Viz3d(), image.as_raw_Mat()) }.into_result()
    }
    
    pub fn set_background_mesh_lab(&mut self) -> Result<()> {
        unsafe { sys::cv_viz_Viz3d_setBackgroundMeshLab(self.as_raw_Viz3d()) }.into_result()
    }
    
    /// The window renders and starts the event loop.
    pub fn spin(&mut self) -> Result<()> {
        unsafe { sys::cv_viz_Viz3d_spin(self.as_raw_Viz3d()) }.into_result()
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
    pub fn spin_once(&mut self, time: i32, force_redraw: bool) -> Result<()> {
        unsafe { sys::cv_viz_Viz3d_spinOnce_int_bool(self.as_raw_Viz3d(), time, force_redraw) }.into_result()
    }
    
    /// Create a window in memory instead of on the screen.
    pub fn set_off_screen_rendering(&mut self) -> Result<()> {
        unsafe { sys::cv_viz_Viz3d_setOffScreenRendering(self.as_raw_Viz3d()) }.into_result()
    }
    
    /// Remove all lights from the current scene.
    pub fn remove_all_lights(&mut self) -> Result<()> {
        unsafe { sys::cv_viz_Viz3d_removeAllLights(self.as_raw_Viz3d()) }.into_result()
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
    /// * focal_point: Vec3d(0, 0, 0)
    /// * color: Color::white()
    /// * diffuse_color: Color::white()
    /// * ambient_color: Color::black()
    /// * specular_color: Color::white()
    pub fn add_light(&mut self, position: core::Vec3d, focal_point: core::Vec3d, color: &crate::viz::Color, diffuse_color: &crate::viz::Color, ambient_color: &crate::viz::Color, specular_color: &crate::viz::Color) -> Result<()> {
        unsafe { sys::cv_viz_Viz3d_addLight_Vec3d_Vec3d_Color_Color_Color_Color(self.as_raw_Viz3d(), position, focal_point, color.as_raw_Color(), diffuse_color.as_raw_Color(), ambient_color.as_raw_Color(), specular_color.as_raw_Color()) }.into_result()
    }
    
    /// Returns whether the event loop has been stopped.
    pub fn was_stopped(&self) -> Result<bool> {
        unsafe { sys::cv_viz_Viz3d_wasStopped_const(self.as_raw_Viz3d()) }.into_result()
    }
    
    pub fn close(&mut self) -> Result<()> {
        unsafe { sys::cv_viz_Viz3d_close(self.as_raw_Viz3d()) }.into_result()
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
    pub fn set_rendering_property(&mut self, id: &str, property: i32, value: f64) -> Result<()> {
        string_arg!(id);
        unsafe { sys::cv_viz_Viz3d_setRenderingProperty_String_int_double(self.as_raw_Viz3d(), id.as_ptr(), property, value) }.into_result()
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
    pub fn get_rendering_property(&mut self, id: &str, property: i32) -> Result<f64> {
        string_arg!(id);
        unsafe { sys::cv_viz_Viz3d_getRenderingProperty_String_int(self.as_raw_Viz3d(), id.as_ptr(), property) }.into_result()
    }
    
    /// Sets geometry representation of the widgets to surface, wireframe or points.
    ///
    /// ## Parameters
    /// * representation: Geometry representation which can be one of the following:
    /// *   **REPRESENTATION_POINTS**
    /// *   **REPRESENTATION_WIREFRAME**
    /// *   **REPRESENTATION_SURFACE**
    pub fn set_representation(&mut self, representation: i32) -> Result<()> {
        unsafe { sys::cv_viz_Viz3d_setRepresentation_int(self.as_raw_Viz3d(), representation) }.into_result()
    }
    
    ///
    /// ## C++ default parameters
    /// * enabled: false
    pub fn set_global_warnings(&mut self, enabled: bool) -> Result<()> {
        unsafe { sys::cv_viz_Viz3d_setGlobalWarnings_bool(self.as_raw_Viz3d(), enabled) }.into_result()
    }
    
}

// boxed class cv::viz::WArrow
/// This 3D Widget defines an arrow.
pub struct WArrow {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WArrow {
    fn drop(&mut self) {
        unsafe { sys::cv_WArrow_delete(self.ptr) };
    }
}
impl crate::viz::WArrow {
    #[inline(always)] pub fn as_raw_WArrow(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WArrow {}

impl crate::viz::Widget for WArrow {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WArrow {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WArrow {

}

// boxed class cv::viz::WCameraPosition
/// This 3D Widget represents camera position in a scene by its axes or viewing frustum. :
pub struct WCameraPosition {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WCameraPosition {
    fn drop(&mut self) {
        unsafe { sys::cv_WCameraPosition_delete(self.ptr) };
    }
}
impl crate::viz::WCameraPosition {
    #[inline(always)] pub fn as_raw_WCameraPosition(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WCameraPosition {}

impl crate::viz::Widget for WCameraPosition {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WCameraPosition {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WCameraPosition {

    /// Creates camera coordinate frame at the origin.
    ///
    /// ![Camera coordinate frame](https://docs.opencv.org/3.4.7/images/cpw1.png)
    ///
    /// ## C++ default parameters
    /// * scale: 1.0
    pub fn new(scale: f64) -> Result<crate::viz::WCameraPosition> {
        unsafe { sys::cv_viz_WCameraPosition_WCameraPosition_double(scale) }.into_result().map(|ptr| crate::viz::WCameraPosition { ptr })
    }
    
    /// Display the viewing frustum
    /// ## Parameters
    /// * fov: Field of view of the camera (horizontal, vertical).
    /// * scale: Scale of the frustum.
    /// * color: Color of the frustum.
    ///
    /// Creates viewing frustum of the camera based on its field of view fov.
    ///
    /// ![Camera viewing frustum](https://docs.opencv.org/3.4.7/images/cpw2.png)
    ///
    /// ## C++ default parameters
    /// * scale: 1.0
    /// * color: Color::white()
    pub fn new_1(fov: core::Vec2d, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WCameraPosition> {
        unsafe { sys::cv_viz_WCameraPosition_WCameraPosition_Vec2d_double_Color(fov, scale, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WCameraPosition { ptr })
    }
    
    /// Display image on the far plane of the viewing frustum
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
    /// ![Camera viewing frustum with image](https://docs.opencv.org/3.4.7/images/cpw3.png)
    ///
    /// ## C++ default parameters
    /// * scale: 1.0
    /// * color: Color::white()
    pub fn new_2(fov: core::Vec2d, image: &core::Mat, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WCameraPosition> {
        unsafe { sys::cv_viz_WCameraPosition_WCameraPosition_Vec2d_Mat_double_Color(fov, image.as_raw_Mat(), scale, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WCameraPosition { ptr })
    }
    
}

// boxed class cv::viz::WCircle
/// This 3D Widget defines a circle.
pub struct WCircle {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WCircle {
    fn drop(&mut self) {
        unsafe { sys::cv_WCircle_delete(self.ptr) };
    }
}
impl crate::viz::WCircle {
    #[inline(always)] pub fn as_raw_WCircle(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WCircle {}

impl crate::viz::Widget for WCircle {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WCircle {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
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
    pub fn new(radius: f64, thickness: f64, color: &crate::viz::Color) -> Result<crate::viz::WCircle> {
        unsafe { sys::cv_viz_WCircle_WCircle_double_double_Color(radius, thickness, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WCircle { ptr })
    }
    
}

// boxed class cv::viz::WCloud
/// This 3D Widget defines a point cloud. :
///
///
/// Note: In case there are four channels in the cloud, fourth channel is ignored.
pub struct WCloud {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WCloud {
    fn drop(&mut self) {
        unsafe { sys::cv_WCloud_delete(self.ptr) };
    }
}
impl crate::viz::WCloud {
    #[inline(always)] pub fn as_raw_WCloud(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WCloud {}

impl crate::viz::Widget for WCloud {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WCloud {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WCloud {

    /// Constructs a WCloud.
    ///
    /// ## Parameters
    /// * cloud: Set of points which can be of type: CV_32FC3, CV_32FC4, CV_64FC3, CV_64FC4.
    /// * colors: Set of colors. It has to be of the same size with cloud.
    ///
    /// Points in the cloud belong to mask when they are set to (NaN, NaN, NaN).
    pub fn new(cloud: &core::Mat, colors: &core::Mat) -> Result<crate::viz::WCloud> {
        unsafe { sys::cv_viz_WCloud_WCloud_Mat_Mat(cloud.as_raw_Mat(), colors.as_raw_Mat()) }.into_result().map(|ptr| crate::viz::WCloud { ptr })
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
    pub fn new_1(cloud: &core::Mat, color: &crate::viz::Color) -> Result<crate::viz::WCloud> {
        unsafe { sys::cv_viz_WCloud_WCloud_Mat_Color(cloud.as_raw_Mat(), color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WCloud { ptr })
    }
    
    /// Constructs a WCloud.
    /// ## Parameters
    /// * cloud: Set of points which can be of type: CV_32FC3, CV_32FC4, CV_64FC3, CV_64FC4.
    /// * colors: Set of colors. It has to be of the same size with cloud.
    /// * normals: Normals for each point in cloud. Size and type should match with the cloud parameter.
    ///
    /// Points in the cloud belong to mask when they are set to (NaN, NaN, NaN).
    pub fn new_2(cloud: &core::Mat, colors: &core::Mat, normals: &core::Mat) -> Result<crate::viz::WCloud> {
        unsafe { sys::cv_viz_WCloud_WCloud_Mat_Mat_Mat(cloud.as_raw_Mat(), colors.as_raw_Mat(), normals.as_raw_Mat()) }.into_result().map(|ptr| crate::viz::WCloud { ptr })
    }
    
    /// Constructs a WCloud.
    /// ## Parameters
    /// * cloud: Set of points which can be of type: CV_32FC3, CV_32FC4, CV_64FC3, CV_64FC4.
    /// * color: A single Color for the whole cloud.
    /// * normals: Normals for each point in cloud.
    ///
    /// Size and type should match with the cloud parameter.
    /// Points in the cloud belong to mask when they are set to (NaN, NaN, NaN).
    pub fn new_3(cloud: &core::Mat, color: &crate::viz::Color, normals: &core::Mat) -> Result<crate::viz::WCloud> {
        unsafe { sys::cv_viz_WCloud_WCloud_Mat_Color_Mat(cloud.as_raw_Mat(), color.as_raw_Color(), normals.as_raw_Mat()) }.into_result().map(|ptr| crate::viz::WCloud { ptr })
    }
    
}

// boxed class cv::viz::WCloudCollection
/// This 3D Widget defines a collection of clouds. :
///
/// Note: In case there are four channels in the cloud, fourth channel is ignored.
pub struct WCloudCollection {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WCloudCollection {
    fn drop(&mut self) {
        unsafe { sys::cv_WCloudCollection_delete(self.ptr) };
    }
}
impl crate::viz::WCloudCollection {
    #[inline(always)] pub fn as_raw_WCloudCollection(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WCloudCollection {}

impl crate::viz::Widget for WCloudCollection {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WCloudCollection {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WCloudCollection {

    pub fn default() -> Result<crate::viz::WCloudCollection> {
        unsafe { sys::cv_viz_WCloudCollection_WCloudCollection() }.into_result().map(|ptr| crate::viz::WCloudCollection { ptr })
    }
    
    /// Finalizes cloud data by repacking to single cloud.
    ///
    /// Useful for large cloud collections to reduce memory usage
    pub fn finalize(&mut self) -> Result<()> {
        unsafe { sys::cv_viz_WCloudCollection_finalize(self.as_raw_WCloudCollection()) }.into_result()
    }
    
}

// boxed class cv::viz::WCloudNormals
/// This 3D Widget represents normals of a point cloud. :
pub struct WCloudNormals {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WCloudNormals {
    fn drop(&mut self) {
        unsafe { sys::cv_WCloudNormals_delete(self.ptr) };
    }
}
impl crate::viz::WCloudNormals {
    #[inline(always)] pub fn as_raw_WCloudNormals(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WCloudNormals {}

impl crate::viz::Widget for WCloudNormals {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WCloudNormals {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
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
    pub fn new(cloud: &core::Mat, normals: &core::Mat, level: i32, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WCloudNormals> {
        unsafe { sys::cv_viz_WCloudNormals_WCloudNormals_Mat_Mat_int_double_Color(cloud.as_raw_Mat(), normals.as_raw_Mat(), level, scale, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WCloudNormals { ptr })
    }
    
}

// boxed class cv::viz::WCone
/// This 3D Widget defines a cone. :
pub struct WCone {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WCone {
    fn drop(&mut self) {
        unsafe { sys::cv_WCone_delete(self.ptr) };
    }
}
impl crate::viz::WCone {
    #[inline(always)] pub fn as_raw_WCone(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WCone {}

impl crate::viz::Widget for WCone {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WCone {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
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
    /// * resolution: 6.0
    /// * color: Color::white()
    pub fn new(length: f64, radius: f64, resolution: i32, color: &crate::viz::Color) -> Result<crate::viz::WCone> {
        unsafe { sys::cv_viz_WCone_WCone_double_double_int_Color(length, radius, resolution, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WCone { ptr })
    }
    
}

// boxed class cv::viz::WCoordinateSystem
/// This 3D Widget represents a coordinate system. :
pub struct WCoordinateSystem {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WCoordinateSystem {
    fn drop(&mut self) {
        unsafe { sys::cv_WCoordinateSystem_delete(self.ptr) };
    }
}
impl crate::viz::WCoordinateSystem {
    #[inline(always)] pub fn as_raw_WCoordinateSystem(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WCoordinateSystem {}

impl crate::viz::Widget for WCoordinateSystem {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WCoordinateSystem {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WCoordinateSystem {

    /// Constructs a WCoordinateSystem.
    ///
    /// ## Parameters
    /// * scale: Determines the size of the axes.
    ///
    /// ## C++ default parameters
    /// * scale: 1.0
    pub fn new(scale: f64) -> Result<crate::viz::WCoordinateSystem> {
        unsafe { sys::cv_viz_WCoordinateSystem_WCoordinateSystem_double(scale) }.into_result().map(|ptr| crate::viz::WCoordinateSystem { ptr })
    }
    
}

// boxed class cv::viz::WCube
/// This 3D Widget defines a cube.
pub struct WCube {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WCube {
    fn drop(&mut self) {
        unsafe { sys::cv_WCube_delete(self.ptr) };
    }
}
impl crate::viz::WCube {
    #[inline(always)] pub fn as_raw_WCube(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WCube {}

impl crate::viz::Widget for WCube {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WCube {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WCube {

}

// boxed class cv::viz::WCylinder
/// This 3D Widget defines a cylinder. :
pub struct WCylinder {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WCylinder {
    fn drop(&mut self) {
        unsafe { sys::cv_WCylinder_delete(self.ptr) };
    }
}
impl crate::viz::WCylinder {
    #[inline(always)] pub fn as_raw_WCylinder(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WCylinder {}

impl crate::viz::Widget for WCylinder {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WCylinder {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WCylinder {

}

// boxed class cv::viz::WGrid
/// This 3D Widget defines a grid. :
pub struct WGrid {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WGrid {
    fn drop(&mut self) {
        unsafe { sys::cv_WGrid_delete(self.ptr) };
    }
}
impl crate::viz::WGrid {
    #[inline(always)] pub fn as_raw_WGrid(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WGrid {}

impl crate::viz::Widget for WGrid {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WGrid {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
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
    pub fn new(cells: core::Vec2i, cells_spacing: core::Vec2d, color: &crate::viz::Color) -> Result<crate::viz::WGrid> {
        unsafe { sys::cv_viz_WGrid_WGrid_Vec2i_Vec2d_Color(cells, cells_spacing, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WGrid { ptr })
    }
    
}

// boxed class cv::viz::WImage3D
/// This 3D Widget represents an image in 3D space. :
pub struct WImage3D {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WImage3D {
    fn drop(&mut self) {
        unsafe { sys::cv_WImage3D_delete(self.ptr) };
    }
}
impl crate::viz::WImage3D {
    #[inline(always)] pub fn as_raw_WImage3D(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WImage3D {}

impl crate::viz::Widget for WImage3D {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WImage3D {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WImage3D {

    /// Constructs an WImage3D.
    ///
    /// ## Parameters
    /// * image: BGR or Gray-Scale image.
    /// * size: Size of the image.
    pub fn new(image: &core::Mat, size: core::Size2d) -> Result<crate::viz::WImage3D> {
        unsafe { sys::cv_viz_WImage3D_WImage3D_Mat_Size2d(image.as_raw_Mat(), size) }.into_result().map(|ptr| crate::viz::WImage3D { ptr })
    }
    
    /// Constructs an WImage3D.
    ///
    /// ## Parameters
    /// * image: BGR or Gray-Scale image.
    /// * size: Size of the image.
    /// * center: Position of the image.
    /// * normal: Normal of the plane that represents the image.
    /// * up_vector: Determines orientation of the image.
    pub fn new_1(image: &core::Mat, size: core::Size2d, center: core::Vec3d, normal: core::Vec3d, up_vector: core::Vec3d) -> Result<crate::viz::WImage3D> {
        unsafe { sys::cv_viz_WImage3D_WImage3D_Mat_Size2d_Vec3d_Vec3d_Vec3d(image.as_raw_Mat(), size, center, normal, up_vector) }.into_result().map(|ptr| crate::viz::WImage3D { ptr })
    }
    
    /// Sets the image content of the widget.
    ///
    /// ## Parameters
    /// * image: BGR or Gray-Scale image.
    pub fn set_image(&mut self, image: &core::Mat) -> Result<()> {
        unsafe { sys::cv_viz_WImage3D_setImage_Mat(self.as_raw_WImage3D(), image.as_raw_Mat()) }.into_result()
    }
    
    /// Sets the image size of the widget.
    ///
    /// ## Parameters
    /// * size: the new size of the image.
    pub fn set_size(&mut self, size: core::Size) -> Result<()> {
        unsafe { sys::cv_viz_WImage3D_setSize_Size(self.as_raw_WImage3D(), size) }.into_result()
    }
    
}

// boxed class cv::viz::WImageOverlay
/// This 2D Widget represents an image overlay. :
pub struct WImageOverlay {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WImageOverlay {
    fn drop(&mut self) {
        unsafe { sys::cv_WImageOverlay_delete(self.ptr) };
    }
}
impl crate::viz::WImageOverlay {
    #[inline(always)] pub fn as_raw_WImageOverlay(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WImageOverlay {}

impl crate::viz::Widget for WImageOverlay {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget2D for WImageOverlay {
    #[inline(always)] fn as_raw_Widget2D(&self) -> *mut c_void { self.ptr }
}

impl WImageOverlay {

    /// Constructs an WImageOverlay.
    ///
    /// ## Parameters
    /// * image: BGR or Gray-Scale image.
    /// * rect: Image is scaled and positioned based on rect.
    pub fn new(image: &core::Mat, rect: core::Rect) -> Result<crate::viz::WImageOverlay> {
        unsafe { sys::cv_viz_WImageOverlay_WImageOverlay_Mat_Rect(image.as_raw_Mat(), rect) }.into_result().map(|ptr| crate::viz::WImageOverlay { ptr })
    }
    
    /// Sets the image content of the widget.
    ///
    /// ## Parameters
    /// * image: BGR or Gray-Scale image.
    pub fn set_image(&mut self, image: &core::Mat) -> Result<()> {
        unsafe { sys::cv_viz_WImageOverlay_setImage_Mat(self.as_raw_WImageOverlay(), image.as_raw_Mat()) }.into_result()
    }
    
}

// boxed class cv::viz::WLine
/// This 3D Widget defines a finite line.
pub struct WLine {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WLine {
    fn drop(&mut self) {
        unsafe { sys::cv_WLine_delete(self.ptr) };
    }
}
impl crate::viz::WLine {
    #[inline(always)] pub fn as_raw_WLine(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WLine {}

impl crate::viz::Widget for WLine {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WLine {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WLine {

}

// boxed class cv::viz::WMesh
/// Constructs a WMesh.
///
/// ## Parameters
/// * mesh: Mesh object that will be displayed.
/// * cloud: Points of the mesh object.
/// * polygons: Points of the mesh object.
/// * colors: Point colors.
/// * normals: Point normals.
pub struct WMesh {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WMesh {
    fn drop(&mut self) {
        unsafe { sys::cv_WMesh_delete(self.ptr) };
    }
}
impl crate::viz::WMesh {
    #[inline(always)] pub fn as_raw_WMesh(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WMesh {}

impl crate::viz::Widget for WMesh {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WMesh {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WMesh {

    pub fn new(mesh: &crate::viz::Mesh) -> Result<crate::viz::WMesh> {
        unsafe { sys::cv_viz_WMesh_WMesh_Mesh(mesh.as_raw_Mesh()) }.into_result().map(|ptr| crate::viz::WMesh { ptr })
    }
    
    ///
    /// ## C++ default parameters
    /// * colors: noArray()
    /// * normals: noArray()
    pub fn new_1(cloud: &core::Mat, polygons: &core::Mat, colors: &core::Mat, normals: &core::Mat) -> Result<crate::viz::WMesh> {
        unsafe { sys::cv_viz_WMesh_WMesh_Mat_Mat_Mat_Mat(cloud.as_raw_Mat(), polygons.as_raw_Mat(), colors.as_raw_Mat(), normals.as_raw_Mat()) }.into_result().map(|ptr| crate::viz::WMesh { ptr })
    }
    
}

// boxed class cv::viz::WPaintedCloud
pub struct WPaintedCloud {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WPaintedCloud {
    fn drop(&mut self) {
        unsafe { sys::cv_WPaintedCloud_delete(self.ptr) };
    }
}
impl crate::viz::WPaintedCloud {
    #[inline(always)] pub fn as_raw_WPaintedCloud(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WPaintedCloud {}

impl crate::viz::Widget for WPaintedCloud {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WPaintedCloud {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WPaintedCloud {

    /// Paint cloud with default gradient between cloud bounds points
    pub fn new(cloud: &core::Mat) -> Result<crate::viz::WPaintedCloud> {
        unsafe { sys::cv_viz_WPaintedCloud_WPaintedCloud_Mat(cloud.as_raw_Mat()) }.into_result().map(|ptr| crate::viz::WPaintedCloud { ptr })
    }
    
}

// boxed class cv::viz::WPlane
/// This 3D Widget defines a finite plane.
pub struct WPlane {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WPlane {
    fn drop(&mut self) {
        unsafe { sys::cv_WPlane_delete(self.ptr) };
    }
}
impl crate::viz::WPlane {
    #[inline(always)] pub fn as_raw_WPlane(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WPlane {}

impl crate::viz::Widget for WPlane {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WPlane {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WPlane {

    /// Constructs a default plane with center point at origin and normal oriented along z-axis.
    ///
    /// ## Parameters
    /// * size: Size of the plane
    /// * color: Color of the plane.
    ///
    /// ## C++ default parameters
    /// * size: Size2d(1.0, 1.0)
    /// * color: Color::white()
    pub fn new(size: core::Size2d, color: &crate::viz::Color) -> Result<crate::viz::WPlane> {
        unsafe { sys::cv_viz_WPlane_WPlane_Size2d_Color(size, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WPlane { ptr })
    }
    
}

// boxed class cv::viz::WPolyLine
/// This 3D Widget defines a poly line. :
pub struct WPolyLine {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WPolyLine {
    fn drop(&mut self) {
        unsafe { sys::cv_WPolyLine_delete(self.ptr) };
    }
}
impl crate::viz::WPolyLine {
    #[inline(always)] pub fn as_raw_WPolyLine(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WPolyLine {}

impl crate::viz::Widget for WPolyLine {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WPolyLine {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WPolyLine {

    pub fn new(points: &core::Mat, colors: &core::Mat) -> Result<crate::viz::WPolyLine> {
        unsafe { sys::cv_viz_WPolyLine_WPolyLine_Mat_Mat(points.as_raw_Mat(), colors.as_raw_Mat()) }.into_result().map(|ptr| crate::viz::WPolyLine { ptr })
    }
    
    /// Constructs a WPolyLine.
    ///
    /// ## Parameters
    /// * points: Point set.
    /// * color: Color of the poly line.
    ///
    /// ## C++ default parameters
    /// * color: Color::white()
    pub fn new_1(points: &core::Mat, color: &crate::viz::Color) -> Result<crate::viz::WPolyLine> {
        unsafe { sys::cv_viz_WPolyLine_WPolyLine_Mat_Color(points.as_raw_Mat(), color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WPolyLine { ptr })
    }
    
}

// boxed class cv::viz::WSphere
/// This 3D Widget defines a sphere. :
pub struct WSphere {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WSphere {
    fn drop(&mut self) {
        unsafe { sys::cv_WSphere_delete(self.ptr) };
    }
}
impl crate::viz::WSphere {
    #[inline(always)] pub fn as_raw_WSphere(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WSphere {}

impl crate::viz::Widget for WSphere {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WSphere {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WSphere {

}

// boxed class cv::viz::WText
/// This 2D Widget represents text overlay.
pub struct WText {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WText {
    fn drop(&mut self) {
        unsafe { sys::cv_WText_delete(self.ptr) };
    }
}
impl crate::viz::WText {
    #[inline(always)] pub fn as_raw_WText(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WText {}

impl crate::viz::Widget for WText {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget2D for WText {
    #[inline(always)] fn as_raw_Widget2D(&self) -> *mut c_void { self.ptr }
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
    pub fn new(text: &str, pos: core::Point, font_size: i32, color: &crate::viz::Color) -> Result<crate::viz::WText> {
        string_arg!(text);
        unsafe { sys::cv_viz_WText_WText_String_Point_int_Color(text.as_ptr(), pos, font_size, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WText { ptr })
    }
    
    /// Sets the text content of the widget.
    ///
    /// ## Parameters
    /// * text: Text content of the widget.
    pub fn set_text(&mut self, text: &str) -> Result<()> {
        string_arg!(text);
        unsafe { sys::cv_viz_WText_setText_String(self.as_raw_WText(), text.as_ptr()) }.into_result()
    }
    
    /// Returns the current text content of the widget.
    pub fn get_text(&self) -> Result<String> {
        unsafe { sys::cv_viz_WText_getText_const(self.as_raw_WText()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
}

// boxed class cv::viz::WText3D
/// This 3D Widget represents 3D text. The text always faces the camera.
pub struct WText3D {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WText3D {
    fn drop(&mut self) {
        unsafe { sys::cv_WText3D_delete(self.ptr) };
    }
}
impl crate::viz::WText3D {
    #[inline(always)] pub fn as_raw_WText3D(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WText3D {}

impl crate::viz::Widget for WText3D {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WText3D {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WText3D {

    /// Sets the text content of the widget.
    ///
    /// ## Parameters
    /// * text: Text content of the widget.
    pub fn set_text(&mut self, text: &str) -> Result<()> {
        string_arg!(text);
        unsafe { sys::cv_viz_WText3D_setText_String(self.as_raw_WText3D(), text.as_ptr()) }.into_result()
    }
    
    /// Returns the current text content of the widget.
    pub fn get_text(&self) -> Result<String> {
        unsafe { sys::cv_viz_WText3D_getText_const(self.as_raw_WText3D()) }.into_result().map(crate::templ::receive_string_mut)
    }
    
}

// boxed class cv::viz::WTrajectory
/// This 3D Widget represents a trajectory. :
pub struct WTrajectory {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WTrajectory {
    fn drop(&mut self) {
        unsafe { sys::cv_WTrajectory_delete(self.ptr) };
    }
}
impl crate::viz::WTrajectory {
    #[inline(always)] pub fn as_raw_WTrajectory(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WTrajectory {}

impl crate::viz::Widget for WTrajectory {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WTrajectory {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
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
    pub fn new(path: &core::Mat, display_mode: i32, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WTrajectory> {
        unsafe { sys::cv_viz_WTrajectory_WTrajectory_Mat_int_double_Color(path.as_raw_Mat(), display_mode, scale, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WTrajectory { ptr })
    }
    
}

// boxed class cv::viz::WTrajectoryFrustums
/// This 3D Widget represents a trajectory. :
pub struct WTrajectoryFrustums {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WTrajectoryFrustums {
    fn drop(&mut self) {
        unsafe { sys::cv_WTrajectoryFrustums_delete(self.ptr) };
    }
}
impl crate::viz::WTrajectoryFrustums {
    #[inline(always)] pub fn as_raw_WTrajectoryFrustums(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WTrajectoryFrustums {}

impl crate::viz::Widget for WTrajectoryFrustums {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WTrajectoryFrustums {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WTrajectoryFrustums {

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
    pub fn new(path: &core::Mat, fov: core::Vec2d, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WTrajectoryFrustums> {
        unsafe { sys::cv_viz_WTrajectoryFrustums_WTrajectoryFrustums_Mat_Vec2d_double_Color(path.as_raw_Mat(), fov, scale, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WTrajectoryFrustums { ptr })
    }
    
}

// boxed class cv::viz::WTrajectorySpheres
/// This 3D Widget represents a trajectory using spheres and lines
///
/// where spheres represent the positions of the camera, and lines represent the direction from
/// previous position to the current. :
pub struct WTrajectorySpheres {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WTrajectorySpheres {
    fn drop(&mut self) {
        unsafe { sys::cv_WTrajectorySpheres_delete(self.ptr) };
    }
}
impl crate::viz::WTrajectorySpheres {
    #[inline(always)] pub fn as_raw_WTrajectorySpheres(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WTrajectorySpheres {}

impl crate::viz::Widget for WTrajectorySpheres {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WTrajectorySpheres {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
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
    pub fn new(path: &core::Mat, line_length: f64, radius: f64, from: &crate::viz::Color, to: &crate::viz::Color) -> Result<crate::viz::WTrajectorySpheres> {
        unsafe { sys::cv_viz_WTrajectorySpheres_WTrajectorySpheres_Mat_double_double_Color_Color(path.as_raw_Mat(), line_length, radius, from.as_raw_Color(), to.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WTrajectorySpheres { ptr })
    }
    
}

// boxed class cv::viz::WWidgetMerger
/// This class allows to merge several widgets to single one.
///
/// It has quite limited functionality and can't merge widgets with different attributes. For
/// instance, if widgetA has color array and widgetB has only global color defined, then result
/// of merge won't have color at all. The class is suitable for merging large amount of similar
/// widgets. :
pub struct WWidgetMerger {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for crate::viz::WWidgetMerger {
    fn drop(&mut self) {
        unsafe { sys::cv_WWidgetMerger_delete(self.ptr) };
    }
}
impl crate::viz::WWidgetMerger {
    #[inline(always)] pub fn as_raw_WWidgetMerger(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for WWidgetMerger {}

impl crate::viz::Widget for WWidgetMerger {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3D for WWidgetMerger {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WWidgetMerger {

    pub fn default() -> Result<crate::viz::WWidgetMerger> {
        unsafe { sys::cv_viz_WWidgetMerger_WWidgetMerger() }.into_result().map(|ptr| crate::viz::WWidgetMerger { ptr })
    }
    
    /// Repacks internal structure to single widget
    pub fn finalize(&mut self) -> Result<()> {
        unsafe { sys::cv_viz_WWidgetMerger_finalize(self.as_raw_WWidgetMerger()) }.into_result()
    }
    
}

// Generating impl for trait cv::viz::Widget (trait)
/// Base class of all widgets. Widget is implicitly shared.
pub trait Widget {
    #[inline(always)] fn as_raw_Widget(&self) -> *mut c_void;
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
    fn set_rendering_property(&mut self, property: i32, value: f64) -> Result<()> {
        unsafe { sys::cv_viz_Widget_setRenderingProperty_int_double(self.as_raw_Widget(), property, value) }.into_result()
    }
    
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
    fn get_rendering_property(&self, property: i32) -> Result<f64> {
        unsafe { sys::cv_viz_Widget_getRenderingProperty_const_int(self.as_raw_Widget(), property) }.into_result()
    }
    
}

impl dyn Widget + '_ {

}

// Generating impl for trait cv::viz::Widget2D (trait)
/// Base class of all 2D widgets.
pub trait Widget2D: crate::viz::Widget {
    #[inline(always)] fn as_raw_Widget2D(&self) -> *mut c_void;
    /// Sets the color of the widget.
    ///
    /// ## Parameters
    /// * color: color of type Color
    fn set_color(&mut self, color: &crate::viz::Color) -> Result<()> {
        unsafe { sys::cv_viz_Widget2D_setColor_Color(self.as_raw_Widget2D(), color.as_raw_Color()) }.into_result()
    }
    
}

// Generating impl for trait cv::viz::Widget3D (trait)
/// Base class of all 3D widgets.
pub trait Widget3D: crate::viz::Widget {
    #[inline(always)] fn as_raw_Widget3D(&self) -> *mut c_void;
    /// Sets the color of the widget.
    ///
    /// ## Parameters
    /// * color: color of type Color
    fn set_color(&mut self, color: &crate::viz::Color) -> Result<()> {
        unsafe { sys::cv_viz_Widget3D_setColor_Color(self.as_raw_Widget3D(), color.as_raw_Color()) }.into_result()
    }
    
}

pub const WTrajectory_BOTH: i32 = 0x3; // 3
