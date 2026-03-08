pub mod ptcloud {
	//! # Point Clound Processing
	use crate::mod_prelude::*;
	use crate::{core, sys, types};
	pub mod prelude {
		pub use super::{OctreeTrait, OctreeTraitConst, OdometryFrameTrait, OdometryFrameTraitConst, OdometrySettingsTrait, OdometrySettingsTraitConst, OdometryTrait, OdometryTraitConst, RgbdNormalsTrait, RgbdNormalsTraitConst, VolumeSettingsTrait, VolumeSettingsTraitConst, VolumeTrait, VolumeTraitConst};
	}

	pub const N_PYRAMIDS: i32 = 9;
	pub const OdometryAlgoType_COMMON: i32 = 0;
	pub const OdometryAlgoType_FAST: i32 = 1;
	pub const OdometryType_DEPTH: i32 = 0;
	pub const OdometryType_RGB: i32 = 1;
	pub const OdometryType_RGB_DEPTH: i32 = 2;
	/// The pyramid of point clouds, produced from the pyramid of depths
	pub const PYR_CLOUD: i32 = 3;
	/// The pyramid of depth images
	pub const PYR_DEPTH: i32 = 1;
	/// The pyramid of dI/dx derivative images
	pub const PYR_DIX: i32 = 4;
	/// The pyramid of dI/dy derivative images
	pub const PYR_DIY: i32 = 5;
	/// The pyramid of grayscale images
	pub const PYR_IMAGE: i32 = 0;
	/// The pyramid of masks
	pub const PYR_MASK: i32 = 2;
	/// The pyramid of normals
	pub const PYR_NORM: i32 = 7;
	/// The pyramid of normals masks
	pub const PYR_NORMMASK: i32 = 8;
	/// The pyramid of "textured" masks (i.e. additional masks for normals or grayscale images)
	pub const PYR_TEXMASK: i32 = 6;
	/// Color and depth have their natural values and converted to internal formats if needed
	pub const RASTERIZE_COMPAT_DISABLED: i32 = 0;
	/// Color is natural, Depth is transformed from [-zNear; -zFar] to [0; 1]
	/// by the following formula: ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cfrac%7Bz%5F%7Bfar%7D%20%5Cleft%28z%20%2B%20z%5F%7Bnear%7D%5Cright%29%7D%7Bz%20%5Cleft%28z%5F%7Bfar%7D%20%2D%20z%5F%7Bnear%7D%5Cright%29%7D%20) 
	///
	/// In this mode the input/output depthBuf is considered to be in this format,
	/// therefore it's faster since there're no conversions performed
	pub const RASTERIZE_COMPAT_INVDEPTH: i32 = 1;
	/// triangles which vertices are given in counterclockwork order are drawn
	pub const RASTERIZE_CULLING_CCW: i32 = 2;
	/// triangles which vertices are given in clockwork order are drawn
	pub const RASTERIZE_CULLING_CW: i32 = 1;
	/// all faces are drawn, no culling is actually performed
	pub const RASTERIZE_CULLING_NONE: i32 = 0;
	/// a color of 1st vertex of each triangle is used
	pub const RASTERIZE_SHADING_FLAT: i32 = 1;
	/// a color is interpolated between 3 vertices with perspective correction
	pub const RASTERIZE_SHADING_SHADED: i32 = 2;
	/// a white color is used for the whole triangle
	pub const RASTERIZE_SHADING_WHITE: i32 = 0;
	pub const RGBD_PLANE_METHOD_DEFAULT: i32 = 0;
	pub const RgbdNormals_RGBD_NORMALS_METHOD_CROSS_PRODUCT: i32 = 3;
	pub const RgbdNormals_RGBD_NORMALS_METHOD_FALS: i32 = 0;
	pub const RgbdNormals_RGBD_NORMALS_METHOD_LINEMOD: i32 = 1;
	pub const RgbdNormals_RGBD_NORMALS_METHOD_SRI: i32 = 2;
	pub const VolumeType_ColorTSDF: i32 = 2;
	pub const VolumeType_HashTSDF: i32 = 1;
	pub const VolumeType_TSDF: i32 = 0;
	pub const Volume_VOLUME_UNIT: i32 = 0;
	pub const Volume_VOXEL: i32 = 1;
	/// These constants are used to set the speed and accuracy of odometry
	/// ## Parameters
	/// * COMMON: accurate but not so fast
	/// * FAST: less accurate but faster
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum OdometryAlgoType {
		COMMON = 0,
		FAST = 1,
	}

	opencv_type_enum! { crate::ptcloud::OdometryAlgoType { COMMON, FAST } }

	/// Indicates what pyramid is to access using getPyramidAt() method:
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum OdometryFramePyramidType {
		/// The pyramid of grayscale images
		PYR_IMAGE = 0,
		/// The pyramid of depth images
		PYR_DEPTH = 1,
		/// The pyramid of masks
		PYR_MASK = 2,
		/// The pyramid of point clouds, produced from the pyramid of depths
		PYR_CLOUD = 3,
		/// The pyramid of dI/dx derivative images
		PYR_DIX = 4,
		/// The pyramid of dI/dy derivative images
		PYR_DIY = 5,
		/// The pyramid of "textured" masks (i.e. additional masks for normals or grayscale images)
		PYR_TEXMASK = 6,
		/// The pyramid of normals
		PYR_NORM = 7,
		/// The pyramid of normals masks
		PYR_NORMMASK = 8,
		N_PYRAMIDS = 9,
	}

	opencv_type_enum! { crate::ptcloud::OdometryFramePyramidType { PYR_IMAGE, PYR_DEPTH, PYR_MASK, PYR_CLOUD, PYR_DIX, PYR_DIY, PYR_TEXMASK, PYR_NORM, PYR_NORMMASK, N_PYRAMIDS } }

	/// These constants are used to set a type of data which odometry will use
	/// ## Parameters
	/// * DEPTH: only depth data
	/// * RGB: only rgb image
	/// * RGB_DEPTH: depth and rgb data simultaneously
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum OdometryType {
		DEPTH = 0,
		RGB = 1,
		RGB_DEPTH = 2,
	}

	opencv_type_enum! { crate::ptcloud::OdometryType { DEPTH, RGB, RGB_DEPTH } }

	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum RgbdNormals_RgbdNormalsMethod {
		RGBD_NORMALS_METHOD_FALS = 0,
		RGBD_NORMALS_METHOD_LINEMOD = 1,
		RGBD_NORMALS_METHOD_SRI = 2,
		RGBD_NORMALS_METHOD_CROSS_PRODUCT = 3,
	}

	opencv_type_enum! { crate::ptcloud::RgbdNormals_RgbdNormalsMethod { RGBD_NORMALS_METHOD_FALS, RGBD_NORMALS_METHOD_LINEMOD, RGBD_NORMALS_METHOD_SRI, RGBD_NORMALS_METHOD_CROSS_PRODUCT } }

	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum RgbdPlaneMethod {
		RGBD_PLANE_METHOD_DEFAULT = 0,
	}

	opencv_type_enum! { crate::ptcloud::RgbdPlaneMethod { RGBD_PLANE_METHOD_DEFAULT } }

	/// Face culling settings: what faces are drawn after face culling
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum TriangleCullingMode {
		/// all faces are drawn, no culling is actually performed
		RASTERIZE_CULLING_NONE = 0,
		/// triangles which vertices are given in clockwork order are drawn
		RASTERIZE_CULLING_CW = 1,
		/// triangles which vertices are given in counterclockwork order are drawn
		RASTERIZE_CULLING_CCW = 2,
	}

	opencv_type_enum! { crate::ptcloud::TriangleCullingMode { RASTERIZE_CULLING_NONE, RASTERIZE_CULLING_CW, RASTERIZE_CULLING_CCW } }

	/// GL compatibility settings
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum TriangleGlCompatibleMode {
		/// Color and depth have their natural values and converted to internal formats if needed
		RASTERIZE_COMPAT_DISABLED = 0,
		/// Color is natural, Depth is transformed from [-zNear; -zFar] to [0; 1]
		/// by the following formula: ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cfrac%7Bz%5F%7Bfar%7D%20%5Cleft%28z%20%2B%20z%5F%7Bnear%7D%5Cright%29%7D%7Bz%20%5Cleft%28z%5F%7Bfar%7D%20%2D%20z%5F%7Bnear%7D%5Cright%29%7D%20) 
		///
		/// In this mode the input/output depthBuf is considered to be in this format,
		/// therefore it's faster since there're no conversions performed
		RASTERIZE_COMPAT_INVDEPTH = 1,
	}

	opencv_type_enum! { crate::ptcloud::TriangleGlCompatibleMode { RASTERIZE_COMPAT_DISABLED, RASTERIZE_COMPAT_INVDEPTH } }

	/// Triangle fill settings
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum TriangleShadingType {
		/// a white color is used for the whole triangle
		RASTERIZE_SHADING_WHITE = 0,
		/// a color of 1st vertex of each triangle is used
		RASTERIZE_SHADING_FLAT = 1,
		/// a color is interpolated between 3 vertices with perspective correction
		RASTERIZE_SHADING_SHADED = 2,
	}

	opencv_type_enum! { crate::ptcloud::TriangleShadingType { RASTERIZE_SHADING_WHITE, RASTERIZE_SHADING_FLAT, RASTERIZE_SHADING_SHADED } }

	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum VolumeType {
		TSDF = 0,
		HashTSDF = 1,
		ColorTSDF = 2,
	}

	opencv_type_enum! { crate::ptcloud::VolumeType { TSDF, HashTSDF, ColorTSDF } }

	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum Volume_BoundingBoxPrecision {
		VOLUME_UNIT = 0,
		VOXEL = 1,
	}

	opencv_type_enum! { crate::ptcloud::Volume_BoundingBoxPrecision { VOLUME_UNIT, VOXEL } }

	/// ## Parameters
	/// * depth: the depth image
	/// * in_K: 
	/// * in_points: the list of xy coordinates
	/// * points3d: the resulting 3d points (point is represented by 4 chanels value [x, y, z, 0])
	#[inline]
	pub fn depth_to3d_sparse(depth: &impl ToInputArray, in_k: &impl ToInputArray, in_points: &impl ToInputArray, points3d: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(depth);
		input_array_arg!(in_k);
		input_array_arg!(in_points);
		output_array_arg!(points3d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_depthTo3dSparse_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(depth.as_raw__InputArray(), in_k.as_raw__InputArray(), in_points.as_raw__InputArray(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Converts a depth image to 3d points. If the mask is empty then the resulting array has the same dimensions as `depth`,
	/// otherwise it is 1d vector containing mask-enabled values only.
	/// The coordinate system is x pointing left, y down and z away from the camera
	/// ## Parameters
	/// * depth: the depth image (if given as short int CV_U, it is assumed to be the depth in millimeters
	///              (as done with the Microsoft Kinect), otherwise, if given as CV_32F or CV_64F, it is assumed in meters)
	/// * K: The calibration matrix
	/// * points3d: the resulting 3d points (point is represented by 4 channels value [x, y, z, 0]). They are of the same depth as `depth` if it is CV_32F or CV_64F, and the
	///        depth of `K` if `depth` is of depth CV_16U or CV_16S
	/// * mask: the mask of the points to consider (can be empty)
	///
	/// ## Note
	/// This alternative version of [depth_to3d] function uses the following default values for its arguments:
	/// * mask: noArray()
	#[inline]
	pub fn depth_to3d_def(depth: &impl ToInputArray, k: &impl ToInputArray, points3d: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(depth);
		input_array_arg!(k);
		output_array_arg!(points3d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_depthTo3d_const__InputArrayR_const__InputArrayR_const__OutputArrayR(depth.as_raw__InputArray(), k.as_raw__InputArray(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Converts a depth image to 3d points. If the mask is empty then the resulting array has the same dimensions as `depth`,
	/// otherwise it is 1d vector containing mask-enabled values only.
	/// The coordinate system is x pointing left, y down and z away from the camera
	/// ## Parameters
	/// * depth: the depth image (if given as short int CV_U, it is assumed to be the depth in millimeters
	///              (as done with the Microsoft Kinect), otherwise, if given as CV_32F or CV_64F, it is assumed in meters)
	/// * K: The calibration matrix
	/// * points3d: the resulting 3d points (point is represented by 4 channels value [x, y, z, 0]). They are of the same depth as `depth` if it is CV_32F or CV_64F, and the
	///        depth of `K` if `depth` is of depth CV_16U or CV_16S
	/// * mask: the mask of the points to consider (can be empty)
	///
	/// ## C++ default parameters
	/// * mask: noArray()
	#[inline]
	pub fn depth_to3d(depth: &impl ToInputArray, k: &impl ToInputArray, points3d: &mut impl ToOutputArray, mask: &impl ToInputArray) -> Result<()> {
		input_array_arg!(depth);
		input_array_arg!(k);
		output_array_arg!(points3d);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_depthTo3d_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(depth.as_raw__InputArray(), k.as_raw__InputArray(), points3d.as_raw__OutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Find the planes in a depth image
	/// ## Parameters
	/// * points3d: the 3d points organized like the depth image: rows x cols with 3 channels
	/// * normals: the normals for every point in the depth image; optional, can be empty
	/// * mask: An image where each pixel is labeled with the plane it belongs to
	///        and 255 if it does not belong to any plane
	/// * plane_coefficients: the coefficients of the corresponding planes (a,b,c,d) such that ax+by+cz+d=0, norm(a,b,c)=1
	///        and c < 0 (so that the normal points towards the camera)
	/// * block_size: The size of the blocks to look at for a stable MSE
	/// * min_size: The minimum size of a cluster to be considered a plane
	/// * threshold: The maximum distance of a point from a plane to belong to it (in meters)
	/// * sensor_error_a: coefficient of the sensor error. 0 by default, use 0.0075 for a Kinect
	/// * sensor_error_b: coefficient of the sensor error. 0 by default
	/// * sensor_error_c: coefficient of the sensor error. 0 by default
	/// * method: The method to use to compute the planes.
	///
	/// ## Note
	/// This alternative version of [find_planes] function uses the following default values for its arguments:
	/// * block_size: 40
	/// * min_size: 40*40
	/// * threshold: 0.01
	/// * sensor_error_a: 0
	/// * sensor_error_b: 0
	/// * sensor_error_c: 0
	/// * method: RGBD_PLANE_METHOD_DEFAULT
	#[inline]
	pub fn find_planes_def(points3d: &impl ToInputArray, normals: &impl ToInputArray, mask: &mut impl ToOutputArray, plane_coefficients: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(points3d);
		input_array_arg!(normals);
		output_array_arg!(mask);
		output_array_arg!(plane_coefficients);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findPlanes_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(points3d.as_raw__InputArray(), normals.as_raw__InputArray(), mask.as_raw__OutputArray(), plane_coefficients.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Find the planes in a depth image
	/// ## Parameters
	/// * points3d: the 3d points organized like the depth image: rows x cols with 3 channels
	/// * normals: the normals for every point in the depth image; optional, can be empty
	/// * mask: An image where each pixel is labeled with the plane it belongs to
	///        and 255 if it does not belong to any plane
	/// * plane_coefficients: the coefficients of the corresponding planes (a,b,c,d) such that ax+by+cz+d=0, norm(a,b,c)=1
	///        and c < 0 (so that the normal points towards the camera)
	/// * block_size: The size of the blocks to look at for a stable MSE
	/// * min_size: The minimum size of a cluster to be considered a plane
	/// * threshold: The maximum distance of a point from a plane to belong to it (in meters)
	/// * sensor_error_a: coefficient of the sensor error. 0 by default, use 0.0075 for a Kinect
	/// * sensor_error_b: coefficient of the sensor error. 0 by default
	/// * sensor_error_c: coefficient of the sensor error. 0 by default
	/// * method: The method to use to compute the planes.
	///
	/// ## C++ default parameters
	/// * block_size: 40
	/// * min_size: 40*40
	/// * threshold: 0.01
	/// * sensor_error_a: 0
	/// * sensor_error_b: 0
	/// * sensor_error_c: 0
	/// * method: RGBD_PLANE_METHOD_DEFAULT
	#[inline]
	pub fn find_planes(points3d: &impl ToInputArray, normals: &impl ToInputArray, mask: &mut impl ToOutputArray, plane_coefficients: &mut impl ToOutputArray, block_size: i32, min_size: i32, threshold: f64, sensor_error_a: f64, sensor_error_b: f64, sensor_error_c: f64, method: crate::ptcloud::RgbdPlaneMethod) -> Result<()> {
		input_array_arg!(points3d);
		input_array_arg!(normals);
		output_array_arg!(mask);
		output_array_arg!(plane_coefficients);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_findPlanes_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_int_double_double_double_double_RgbdPlaneMethod(points3d.as_raw__InputArray(), normals.as_raw__InputArray(), mask.as_raw__OutputArray(), plane_coefficients.as_raw__OutputArray(), block_size, min_size, threshold, sensor_error_a, sensor_error_b, sensor_error_c, method, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Loads a mesh from a file.
	///
	/// The function loads mesh from the specified file and returns it.
	/// If the mesh cannot be read, throws an error
	/// Vertex attributes (i.e. space and texture coodinates, normals and colors) are returned in same-sized
	/// arrays with corresponding elements having the same indices.
	/// This means that if a face uses a vertex with a normal or a texture coordinate with different indices
	/// (which is typical for OBJ files for example), this vertex will be duplicated for each face it uses.
	///
	/// Currently, the following file formats are supported:
	/// *  [Wavefront obj file *.obj](https://en.wikipedia.org/wiki/Wavefront_.obj_file) (ONLY TRIANGULATED FACES)
	/// *  [Polygon File Format *.ply](https://en.wikipedia.org/wiki/PLY_(file_format))
	/// ## Parameters
	/// * filename: Name of the file
	/// * vertices: vertex coordinates, each value contains 3 floats
	/// * indices: per-face list of vertices, each value is a vector of ints
	/// * normals: per-vertex normals, each value contains 3 floats
	/// * colors: per-vertex colors, each value contains 3 floats
	/// * texCoords: per-vertex texture coordinates, each value contains 2 or 3 floats
	///
	/// ## Note
	/// This alternative version of [load_mesh] function uses the following default values for its arguments:
	/// * normals: noArray()
	/// * colors: noArray()
	/// * tex_coords: noArray()
	#[inline]
	pub fn load_mesh_def(filename: &str, vertices: &mut impl ToOutputArray, indices: &mut impl ToOutputArray) -> Result<()> {
		extern_container_arg!(filename);
		output_array_arg!(vertices);
		output_array_arg!(indices);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_loadMesh_const_StringR_const__OutputArrayR_const__OutputArrayR(filename.opencv_as_extern(), vertices.as_raw__OutputArray(), indices.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Loads a mesh from a file.
	///
	/// The function loads mesh from the specified file and returns it.
	/// If the mesh cannot be read, throws an error
	/// Vertex attributes (i.e. space and texture coodinates, normals and colors) are returned in same-sized
	/// arrays with corresponding elements having the same indices.
	/// This means that if a face uses a vertex with a normal or a texture coordinate with different indices
	/// (which is typical for OBJ files for example), this vertex will be duplicated for each face it uses.
	///
	/// Currently, the following file formats are supported:
	/// *  [Wavefront obj file *.obj](https://en.wikipedia.org/wiki/Wavefront_.obj_file) (ONLY TRIANGULATED FACES)
	/// *  [Polygon File Format *.ply](https://en.wikipedia.org/wiki/PLY_(file_format))
	/// ## Parameters
	/// * filename: Name of the file
	/// * vertices: vertex coordinates, each value contains 3 floats
	/// * indices: per-face list of vertices, each value is a vector of ints
	/// * normals: per-vertex normals, each value contains 3 floats
	/// * colors: per-vertex colors, each value contains 3 floats
	/// * texCoords: per-vertex texture coordinates, each value contains 2 or 3 floats
	///
	/// ## C++ default parameters
	/// * normals: noArray()
	/// * colors: noArray()
	/// * tex_coords: noArray()
	#[inline]
	pub fn load_mesh(filename: &str, vertices: &mut impl ToOutputArray, indices: &mut impl ToOutputArray, normals: &mut impl ToOutputArray, colors: &mut impl ToOutputArray, tex_coords: &mut impl ToOutputArray) -> Result<()> {
		extern_container_arg!(filename);
		output_array_arg!(vertices);
		output_array_arg!(indices);
		output_array_arg!(normals);
		output_array_arg!(colors);
		output_array_arg!(tex_coords);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_loadMesh_const_StringR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(filename.opencv_as_extern(), vertices.as_raw__OutputArray(), indices.as_raw__OutputArray(), normals.as_raw__OutputArray(), colors.as_raw__OutputArray(), tex_coords.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Loads a point cloud from a file.
	///
	/// The function loads point cloud from the specified file and returns it.
	/// If the cloud cannot be read, throws an error.
	/// Vertex coordinates, normals and colors are returned as they are saved in the file
	/// even if these arrays have different sizes and their elements do not correspond to each other
	/// (which is typical for OBJ files for example)
	///
	/// Currently, the following file formats are supported:
	/// *  [Wavefront obj file *.obj](https://en.wikipedia.org/wiki/Wavefront_.obj_file)
	/// *  [Polygon File Format *.ply](https://en.wikipedia.org/wiki/PLY_(file_format))
	///
	/// ## Parameters
	/// * filename: Name of the file
	/// * vertices: vertex coordinates, each value contains 3 floats
	/// * normals: per-vertex normals, each value contains 3 floats
	/// * rgb: per-vertex colors, each value contains 3 floats
	///
	/// ## Note
	/// This alternative version of [load_point_cloud] function uses the following default values for its arguments:
	/// * normals: noArray()
	/// * rgb: noArray()
	#[inline]
	pub fn load_point_cloud_def(filename: &str, vertices: &mut impl ToOutputArray) -> Result<()> {
		extern_container_arg!(filename);
		output_array_arg!(vertices);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_loadPointCloud_const_StringR_const__OutputArrayR(filename.opencv_as_extern(), vertices.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Loads a point cloud from a file.
	///
	/// The function loads point cloud from the specified file and returns it.
	/// If the cloud cannot be read, throws an error.
	/// Vertex coordinates, normals and colors are returned as they are saved in the file
	/// even if these arrays have different sizes and their elements do not correspond to each other
	/// (which is typical for OBJ files for example)
	///
	/// Currently, the following file formats are supported:
	/// *  [Wavefront obj file *.obj](https://en.wikipedia.org/wiki/Wavefront_.obj_file)
	/// *  [Polygon File Format *.ply](https://en.wikipedia.org/wiki/PLY_(file_format))
	///
	/// ## Parameters
	/// * filename: Name of the file
	/// * vertices: vertex coordinates, each value contains 3 floats
	/// * normals: per-vertex normals, each value contains 3 floats
	/// * rgb: per-vertex colors, each value contains 3 floats
	///
	/// ## C++ default parameters
	/// * normals: noArray()
	/// * rgb: noArray()
	#[inline]
	pub fn load_point_cloud(filename: &str, vertices: &mut impl ToOutputArray, normals: &mut impl ToOutputArray, rgb: &mut impl ToOutputArray) -> Result<()> {
		extern_container_arg!(filename);
		output_array_arg!(vertices);
		output_array_arg!(normals);
		output_array_arg!(rgb);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_loadPointCloud_const_StringR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(filename.opencv_as_extern(), vertices.as_raw__OutputArray(), normals.as_raw__OutputArray(), rgb.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ## Note
	/// This alternative version of [register_depth] function uses the following default values for its arguments:
	/// * depth_dilation: false
	#[inline]
	pub fn register_depth_def(unregistered_camera_matrix: &impl ToInputArray, registered_camera_matrix: &impl ToInputArray, registered_dist_coeffs: &impl ToInputArray, rt: &impl ToInputArray, unregistered_depth: &impl ToInputArray, output_image_plane_size: core::Size, registered_depth: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(unregistered_camera_matrix);
		input_array_arg!(registered_camera_matrix);
		input_array_arg!(registered_dist_coeffs);
		input_array_arg!(rt);
		input_array_arg!(unregistered_depth);
		output_array_arg!(registered_depth);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_registerDepth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__OutputArrayR(unregistered_camera_matrix.as_raw__InputArray(), registered_camera_matrix.as_raw__InputArray(), registered_dist_coeffs.as_raw__InputArray(), rt.as_raw__InputArray(), unregistered_depth.as_raw__InputArray(), &output_image_plane_size, registered_depth.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	#[inline]
	pub fn register_depth(unregistered_camera_matrix: &impl ToInputArray, registered_camera_matrix: &impl ToInputArray, registered_dist_coeffs: &impl ToInputArray, rt: &impl ToInputArray, unregistered_depth: &impl ToInputArray, output_image_plane_size: core::Size, registered_depth: &mut impl ToOutputArray, depth_dilation: bool) -> Result<()> {
		input_array_arg!(unregistered_camera_matrix);
		input_array_arg!(registered_camera_matrix);
		input_array_arg!(registered_dist_coeffs);
		input_array_arg!(rt);
		input_array_arg!(unregistered_depth);
		output_array_arg!(registered_depth);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_registerDepth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__OutputArrayR_bool(unregistered_camera_matrix.as_raw__InputArray(), registered_camera_matrix.as_raw__InputArray(), registered_dist_coeffs.as_raw__InputArray(), rt.as_raw__InputArray(), unregistered_depth.as_raw__InputArray(), &output_image_plane_size, registered_depth.as_raw__OutputArray(), depth_dilation, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// If the input image is of type CV_16UC1 (like the Kinect one), the image is converted to floats, divided
	/// by depth_factor to get a depth in meters, and the values 0 are converted to std::numeric_limits<float>::quiet_NaN()
	/// Otherwise, the image is simply converted to floats
	/// ## Parameters
	/// * in: the depth image (if given as short int CV_U, it is assumed to be the depth in millimeters
	///              (as done with the Microsoft Kinect), it is assumed in meters)
	/// * type: the desired output depth (CV_32F or CV_64F)
	/// * out: The rescaled float depth image
	/// * depth_factor: (optional) factor by which depth is converted to distance (by default = 1000.0 for Kinect sensor)
	///
	/// ## Note
	/// This alternative version of [rescale_depth] function uses the following default values for its arguments:
	/// * depth_factor: 1000.0
	#[inline]
	pub fn rescale_depth_def(in_: &impl ToInputArray, typ: i32, out: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(in_);
		output_array_arg!(out);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rescaleDepth_const__InputArrayR_int_const__OutputArrayR(in_.as_raw__InputArray(), typ, out.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// If the input image is of type CV_16UC1 (like the Kinect one), the image is converted to floats, divided
	/// by depth_factor to get a depth in meters, and the values 0 are converted to std::numeric_limits<float>::quiet_NaN()
	/// Otherwise, the image is simply converted to floats
	/// ## Parameters
	/// * in: the depth image (if given as short int CV_U, it is assumed to be the depth in millimeters
	///              (as done with the Microsoft Kinect), it is assumed in meters)
	/// * type: the desired output depth (CV_32F or CV_64F)
	/// * out: The rescaled float depth image
	/// * depth_factor: (optional) factor by which depth is converted to distance (by default = 1000.0 for Kinect sensor)
	///
	/// ## C++ default parameters
	/// * depth_factor: 1000.0
	#[inline]
	pub fn rescale_depth(in_: &impl ToInputArray, typ: i32, out: &mut impl ToOutputArray, depth_factor: f64) -> Result<()> {
		input_array_arg!(in_);
		output_array_arg!(out);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rescaleDepth_const__InputArrayR_int_const__OutputArrayR_double(in_.as_raw__InputArray(), typ, out.as_raw__OutputArray(), depth_factor, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Saves a mesh to a specified file.
	///
	/// The function saves mesh to the specified file.
	/// File format is chosen based on the filename extension.
	///
	/// ## Parameters
	/// * filename: Name of the file.
	/// * vertices: vertex coordinates, each value contains 3 floats
	/// * indices: per-face list of vertices, each value is a vector of ints
	/// * normals: per-vertex normals, each value contains 3 floats
	/// * colors: per-vertex colors, each value contains 3 floats
	/// * texCoords: per-vertex texture coordinates, each value contains 2 or 3 floats
	///
	/// ## Note
	/// This alternative version of [save_mesh] function uses the following default values for its arguments:
	/// * normals: noArray()
	/// * colors: noArray()
	/// * tex_coords: noArray()
	#[inline]
	pub fn save_mesh_def(filename: &str, vertices: &impl ToInputArray, indices: &impl ToInputArray) -> Result<()> {
		extern_container_arg!(filename);
		input_array_arg!(vertices);
		input_array_arg!(indices);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saveMesh_const_StringR_const__InputArrayR_const__InputArrayR(filename.opencv_as_extern(), vertices.as_raw__InputArray(), indices.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Saves a mesh to a specified file.
	///
	/// The function saves mesh to the specified file.
	/// File format is chosen based on the filename extension.
	///
	/// ## Parameters
	/// * filename: Name of the file.
	/// * vertices: vertex coordinates, each value contains 3 floats
	/// * indices: per-face list of vertices, each value is a vector of ints
	/// * normals: per-vertex normals, each value contains 3 floats
	/// * colors: per-vertex colors, each value contains 3 floats
	/// * texCoords: per-vertex texture coordinates, each value contains 2 or 3 floats
	///
	/// ## C++ default parameters
	/// * normals: noArray()
	/// * colors: noArray()
	/// * tex_coords: noArray()
	#[inline]
	pub fn save_mesh(filename: &str, vertices: &impl ToInputArray, indices: &impl ToInputArray, normals: &impl ToInputArray, colors: &impl ToInputArray, tex_coords: &impl ToInputArray) -> Result<()> {
		extern_container_arg!(filename);
		input_array_arg!(vertices);
		input_array_arg!(indices);
		input_array_arg!(normals);
		input_array_arg!(colors);
		input_array_arg!(tex_coords);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saveMesh_const_StringR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(filename.opencv_as_extern(), vertices.as_raw__InputArray(), indices.as_raw__InputArray(), normals.as_raw__InputArray(), colors.as_raw__InputArray(), tex_coords.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Saves a point cloud to a specified file.
	///
	/// The function saves point cloud to the specified file.
	/// File format is chosen based on the filename extension.
	///
	/// ## Parameters
	/// * filename: Name of the file
	/// * vertices: vertex coordinates, each value contains 3 floats
	/// * normals: per-vertex normals, each value contains 3 floats
	/// * rgb: per-vertex colors, each value contains 3 floats
	///
	/// ## Note
	/// This alternative version of [save_point_cloud] function uses the following default values for its arguments:
	/// * normals: noArray()
	/// * rgb: noArray()
	#[inline]
	pub fn save_point_cloud_def(filename: &str, vertices: &impl ToInputArray) -> Result<()> {
		extern_container_arg!(filename);
		input_array_arg!(vertices);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_savePointCloud_const_StringR_const__InputArrayR(filename.opencv_as_extern(), vertices.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Saves a point cloud to a specified file.
	///
	/// The function saves point cloud to the specified file.
	/// File format is chosen based on the filename extension.
	///
	/// ## Parameters
	/// * filename: Name of the file
	/// * vertices: vertex coordinates, each value contains 3 floats
	/// * normals: per-vertex normals, each value contains 3 floats
	/// * rgb: per-vertex colors, each value contains 3 floats
	///
	/// ## C++ default parameters
	/// * normals: noArray()
	/// * rgb: noArray()
	#[inline]
	pub fn save_point_cloud(filename: &str, vertices: &impl ToInputArray, normals: &impl ToInputArray, rgb: &impl ToInputArray) -> Result<()> {
		extern_container_arg!(filename);
		input_array_arg!(vertices);
		input_array_arg!(normals);
		input_array_arg!(rgb);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_savePointCloud_const_StringR_const__InputArrayR_const__InputArrayR_const__InputArrayR(filename.opencv_as_extern(), vertices.as_raw__InputArray(), normals.as_raw__InputArray(), rgb.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Overloaded version of triangleRasterize() with color-only rendering
	///
	/// ## Parameters
	/// * vertices: vertices coordinates array. Should contain values of CV_32FC3 type or a compatible one (e.g. cv::Vec3f, etc.)
	/// * indices: triangle vertices index array, 3 per triangle. Each index indicates a vertex in a vertices array.
	/// Should contain CV_32SC3 values or compatible
	/// * colors: per-vertex colors of CV_32FC3 type or compatible. Can be empty or the same size as vertices array.
	/// If the values are out of [0; 1] range, the result correctness is not guaranteed
	/// * colorBuf: an array representing the final rendered image. Should containt CV_32FC3 values and be the same size as depthBuf.
	/// Not cleared before rendering, i.e. the content is reused as there is some pre-rendered scene.
	/// * world2cam: a 4x3 or 4x4 float or double matrix containing inverted (sic!) camera pose
	/// * fovY: field of view in vertical direction, given in radians
	/// * zNear: minimum Z value to render, everything closer is clipped
	/// * zFar: maximum Z value to render, everything farther is clipped
	/// * settings: see TriangleRasterizeSettings. By default the smooth shading is on,
	/// with CW culling and with disabled GL compatibility
	///
	/// ## Note
	/// This alternative version of [triangle_rasterize_color] function uses the following default values for its arguments:
	/// * settings: TriangleRasterizeSettings()
	#[inline]
	pub fn triangle_rasterize_color_def(vertices: &impl ToInputArray, indices: &impl ToInputArray, colors: &impl ToInputArray, color_buf: &mut impl ToInputOutputArray, world2cam: &impl ToInputArray, fov_y: f64, z_near: f64, z_far: f64) -> Result<()> {
		input_array_arg!(vertices);
		input_array_arg!(indices);
		input_array_arg!(colors);
		input_output_array_arg!(color_buf);
		input_array_arg!(world2cam);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_triangleRasterizeColor_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double(vertices.as_raw__InputArray(), indices.as_raw__InputArray(), colors.as_raw__InputArray(), color_buf.as_raw__InputOutputArray(), world2cam.as_raw__InputArray(), fov_y, z_near, z_far, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Overloaded version of triangleRasterize() with color-only rendering
	///
	/// ## Parameters
	/// * vertices: vertices coordinates array. Should contain values of CV_32FC3 type or a compatible one (e.g. cv::Vec3f, etc.)
	/// * indices: triangle vertices index array, 3 per triangle. Each index indicates a vertex in a vertices array.
	/// Should contain CV_32SC3 values or compatible
	/// * colors: per-vertex colors of CV_32FC3 type or compatible. Can be empty or the same size as vertices array.
	/// If the values are out of [0; 1] range, the result correctness is not guaranteed
	/// * colorBuf: an array representing the final rendered image. Should containt CV_32FC3 values and be the same size as depthBuf.
	/// Not cleared before rendering, i.e. the content is reused as there is some pre-rendered scene.
	/// * world2cam: a 4x3 or 4x4 float or double matrix containing inverted (sic!) camera pose
	/// * fovY: field of view in vertical direction, given in radians
	/// * zNear: minimum Z value to render, everything closer is clipped
	/// * zFar: maximum Z value to render, everything farther is clipped
	/// * settings: see TriangleRasterizeSettings. By default the smooth shading is on,
	/// with CW culling and with disabled GL compatibility
	///
	/// ## C++ default parameters
	/// * settings: TriangleRasterizeSettings()
	#[inline]
	pub fn triangle_rasterize_color(vertices: &impl ToInputArray, indices: &impl ToInputArray, colors: &impl ToInputArray, color_buf: &mut impl ToInputOutputArray, world2cam: &impl ToInputArray, fov_y: f64, z_near: f64, z_far: f64, settings: crate::ptcloud::TriangleRasterizeSettings) -> Result<()> {
		input_array_arg!(vertices);
		input_array_arg!(indices);
		input_array_arg!(colors);
		input_output_array_arg!(color_buf);
		input_array_arg!(world2cam);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_triangleRasterizeColor_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double_const_TriangleRasterizeSettingsR(vertices.as_raw__InputArray(), indices.as_raw__InputArray(), colors.as_raw__InputArray(), color_buf.as_raw__InputOutputArray(), world2cam.as_raw__InputArray(), fov_y, z_near, z_far, &settings, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Overloaded version of triangleRasterize() with depth-only rendering
	///
	/// ## Parameters
	/// * vertices: vertices coordinates array. Should contain values of CV_32FC3 type or a compatible one (e.g. cv::Vec3f, etc.)
	/// * indices: triangle vertices index array, 3 per triangle. Each index indicates a vertex in a vertices array.
	/// Should contain CV_32SC3 values or compatible
	/// * depthBuf: an array of floats containing resulting Z buffer. Should contain float values and be the same size as colorBuf.
	/// Not cleared before rendering, i.e. the content is reused as there is some pre-rendered scene.
	/// Empty scene corresponds to all values set to zFar (or to 1.0 in INVDEPTH mode)
	/// * world2cam: a 4x3 or 4x4 float or double matrix containing inverted (sic!) camera pose
	/// * fovY: field of view in vertical direction, given in radians
	/// * zNear: minimum Z value to render, everything closer is clipped
	/// * zFar: maximum Z value to render, everything farther is clipped
	/// * settings: see TriangleRasterizeSettings. By default the smooth shading is on,
	/// with CW culling and with disabled GL compatibility
	///
	/// ## Note
	/// This alternative version of [triangle_rasterize_depth] function uses the following default values for its arguments:
	/// * settings: TriangleRasterizeSettings()
	#[inline]
	pub fn triangle_rasterize_depth_def(vertices: &impl ToInputArray, indices: &impl ToInputArray, depth_buf: &mut impl ToInputOutputArray, world2cam: &impl ToInputArray, fov_y: f64, z_near: f64, z_far: f64) -> Result<()> {
		input_array_arg!(vertices);
		input_array_arg!(indices);
		input_output_array_arg!(depth_buf);
		input_array_arg!(world2cam);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_triangleRasterizeDepth_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double(vertices.as_raw__InputArray(), indices.as_raw__InputArray(), depth_buf.as_raw__InputOutputArray(), world2cam.as_raw__InputArray(), fov_y, z_near, z_far, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Overloaded version of triangleRasterize() with depth-only rendering
	///
	/// ## Parameters
	/// * vertices: vertices coordinates array. Should contain values of CV_32FC3 type or a compatible one (e.g. cv::Vec3f, etc.)
	/// * indices: triangle vertices index array, 3 per triangle. Each index indicates a vertex in a vertices array.
	/// Should contain CV_32SC3 values or compatible
	/// * depthBuf: an array of floats containing resulting Z buffer. Should contain float values and be the same size as colorBuf.
	/// Not cleared before rendering, i.e. the content is reused as there is some pre-rendered scene.
	/// Empty scene corresponds to all values set to zFar (or to 1.0 in INVDEPTH mode)
	/// * world2cam: a 4x3 or 4x4 float or double matrix containing inverted (sic!) camera pose
	/// * fovY: field of view in vertical direction, given in radians
	/// * zNear: minimum Z value to render, everything closer is clipped
	/// * zFar: maximum Z value to render, everything farther is clipped
	/// * settings: see TriangleRasterizeSettings. By default the smooth shading is on,
	/// with CW culling and with disabled GL compatibility
	///
	/// ## C++ default parameters
	/// * settings: TriangleRasterizeSettings()
	#[inline]
	pub fn triangle_rasterize_depth(vertices: &impl ToInputArray, indices: &impl ToInputArray, depth_buf: &mut impl ToInputOutputArray, world2cam: &impl ToInputArray, fov_y: f64, z_near: f64, z_far: f64, settings: crate::ptcloud::TriangleRasterizeSettings) -> Result<()> {
		input_array_arg!(vertices);
		input_array_arg!(indices);
		input_output_array_arg!(depth_buf);
		input_array_arg!(world2cam);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_triangleRasterizeDepth_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double_const_TriangleRasterizeSettingsR(vertices.as_raw__InputArray(), indices.as_raw__InputArray(), depth_buf.as_raw__InputOutputArray(), world2cam.as_raw__InputArray(), fov_y, z_near, z_far, &settings, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Renders a set of triangles on a depth and color image
	///
	/// Triangles can be drawn white (1.0, 1.0, 1.0), flat-shaded or with a color interpolation between vertices.
	/// In flat-shaded mode the 1st vertex color of each triangle is used to fill the whole triangle.
	///
	/// The world2cam is an inverted camera pose matrix in fact. It transforms vertices from world to
	/// camera coordinate system.
	///
	/// The camera coordinate system emulates the OpenGL's coordinate system having coordinate origin in a screen center,
	/// X axis pointing right, Y axis pointing up and Z axis pointing towards the viewer
	/// except that image is vertically flipped after the render.
	/// This means that all visible objects are placed in z-negative area, or exactly in -zNear > z > -zFar since
	/// zNear and zFar are positive.
	/// For example, at fovY = PI/2 the point (0, 1, -1) will be projected to (width/2, 0) screen point,
	/// (1, 0, -1) to (width/2 + height/2, height/2). Increasing fovY makes projection smaller and vice versa.
	///
	/// The function does not create or clear output images before the rendering. This means that it can be used
	/// for drawing over an existing image or for rendering a model into a 3D scene using pre-filled Z-buffer.
	///
	/// Empty scene results in a depth buffer filled by the maximum value since every pixel is infinitely far from the camera.
	/// Therefore, before rendering anything from scratch the depthBuf should be filled by zFar values (or by ones in INVDEPTH mode).
	///
	/// There are special versions of this function named triangleRasterizeDepth and triangleRasterizeColor
	/// for cases if a user needs a color image or a depth image alone; they may run slightly faster.
	///
	/// ## Parameters
	/// * vertices: vertices coordinates array. Should contain values of CV_32FC3 type or a compatible one (e.g. cv::Vec3f, etc.)
	/// * indices: triangle vertices index array, 3 per triangle. Each index indicates a vertex in a vertices array.
	/// Should contain CV_32SC3 values or compatible
	/// * colors: per-vertex colors of CV_32FC3 type or compatible. Can be empty or the same size as vertices array.
	/// If the values are out of [0; 1] range, the result correctness is not guaranteed
	/// * colorBuf: an array representing the final rendered image. Should containt CV_32FC3 values and be the same size as depthBuf.
	/// Not cleared before rendering, i.e. the content is reused as there is some pre-rendered scene.
	/// * depthBuf: an array of floats containing resulting Z buffer. Should contain float values and be the same size as colorBuf.
	/// Not cleared before rendering, i.e. the content is reused as there is some pre-rendered scene.
	/// Empty scene corresponds to all values set to zFar (or to 1.0 in INVDEPTH mode)
	/// * world2cam: a 4x3 or 4x4 float or double matrix containing inverted (sic!) camera pose
	/// * fovY: field of view in vertical direction, given in radians
	/// * zNear: minimum Z value to render, everything closer is clipped
	/// * zFar: maximum Z value to render, everything farther is clipped
	/// * settings: see TriangleRasterizeSettings. By default the smooth shading is on,
	/// with CW culling and with disabled GL compatibility
	///
	/// ## Note
	/// This alternative version of [triangle_rasterize] function uses the following default values for its arguments:
	/// * settings: TriangleRasterizeSettings()
	#[inline]
	pub fn triangle_rasterize_def(vertices: &impl ToInputArray, indices: &impl ToInputArray, colors: &impl ToInputArray, color_buf: &mut impl ToInputOutputArray, depth_buf: &mut impl ToInputOutputArray, world2cam: &impl ToInputArray, fov_y: f64, z_near: f64, z_far: f64) -> Result<()> {
		input_array_arg!(vertices);
		input_array_arg!(indices);
		input_array_arg!(colors);
		input_output_array_arg!(color_buf);
		input_output_array_arg!(depth_buf);
		input_array_arg!(world2cam);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_triangleRasterize_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double(vertices.as_raw__InputArray(), indices.as_raw__InputArray(), colors.as_raw__InputArray(), color_buf.as_raw__InputOutputArray(), depth_buf.as_raw__InputOutputArray(), world2cam.as_raw__InputArray(), fov_y, z_near, z_far, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Renders a set of triangles on a depth and color image
	///
	/// Triangles can be drawn white (1.0, 1.0, 1.0), flat-shaded or with a color interpolation between vertices.
	/// In flat-shaded mode the 1st vertex color of each triangle is used to fill the whole triangle.
	///
	/// The world2cam is an inverted camera pose matrix in fact. It transforms vertices from world to
	/// camera coordinate system.
	///
	/// The camera coordinate system emulates the OpenGL's coordinate system having coordinate origin in a screen center,
	/// X axis pointing right, Y axis pointing up and Z axis pointing towards the viewer
	/// except that image is vertically flipped after the render.
	/// This means that all visible objects are placed in z-negative area, or exactly in -zNear > z > -zFar since
	/// zNear and zFar are positive.
	/// For example, at fovY = PI/2 the point (0, 1, -1) will be projected to (width/2, 0) screen point,
	/// (1, 0, -1) to (width/2 + height/2, height/2). Increasing fovY makes projection smaller and vice versa.
	///
	/// The function does not create or clear output images before the rendering. This means that it can be used
	/// for drawing over an existing image or for rendering a model into a 3D scene using pre-filled Z-buffer.
	///
	/// Empty scene results in a depth buffer filled by the maximum value since every pixel is infinitely far from the camera.
	/// Therefore, before rendering anything from scratch the depthBuf should be filled by zFar values (or by ones in INVDEPTH mode).
	///
	/// There are special versions of this function named triangleRasterizeDepth and triangleRasterizeColor
	/// for cases if a user needs a color image or a depth image alone; they may run slightly faster.
	///
	/// ## Parameters
	/// * vertices: vertices coordinates array. Should contain values of CV_32FC3 type or a compatible one (e.g. cv::Vec3f, etc.)
	/// * indices: triangle vertices index array, 3 per triangle. Each index indicates a vertex in a vertices array.
	/// Should contain CV_32SC3 values or compatible
	/// * colors: per-vertex colors of CV_32FC3 type or compatible. Can be empty or the same size as vertices array.
	/// If the values are out of [0; 1] range, the result correctness is not guaranteed
	/// * colorBuf: an array representing the final rendered image. Should containt CV_32FC3 values and be the same size as depthBuf.
	/// Not cleared before rendering, i.e. the content is reused as there is some pre-rendered scene.
	/// * depthBuf: an array of floats containing resulting Z buffer. Should contain float values and be the same size as colorBuf.
	/// Not cleared before rendering, i.e. the content is reused as there is some pre-rendered scene.
	/// Empty scene corresponds to all values set to zFar (or to 1.0 in INVDEPTH mode)
	/// * world2cam: a 4x3 or 4x4 float or double matrix containing inverted (sic!) camera pose
	/// * fovY: field of view in vertical direction, given in radians
	/// * zNear: minimum Z value to render, everything closer is clipped
	/// * zFar: maximum Z value to render, everything farther is clipped
	/// * settings: see TriangleRasterizeSettings. By default the smooth shading is on,
	/// with CW culling and with disabled GL compatibility
	///
	/// ## C++ default parameters
	/// * settings: TriangleRasterizeSettings()
	#[inline]
	pub fn triangle_rasterize(vertices: &impl ToInputArray, indices: &impl ToInputArray, colors: &impl ToInputArray, color_buf: &mut impl ToInputOutputArray, depth_buf: &mut impl ToInputOutputArray, world2cam: &impl ToInputArray, fov_y: f64, z_near: f64, z_far: f64, settings: crate::ptcloud::TriangleRasterizeSettings) -> Result<()> {
		input_array_arg!(vertices);
		input_array_arg!(indices);
		input_array_arg!(colors);
		input_output_array_arg!(color_buf);
		input_output_array_arg!(depth_buf);
		input_array_arg!(world2cam);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_triangleRasterize_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const__InputArrayR_double_double_double_const_TriangleRasterizeSettingsR(vertices.as_raw__InputArray(), indices.as_raw__InputArray(), colors.as_raw__InputArray(), color_buf.as_raw__InputOutputArray(), depth_buf.as_raw__InputOutputArray(), world2cam.as_raw__InputArray(), fov_y, z_near, z_far, &settings, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Warps depth or RGB-D image by reprojecting it in 3d, applying Rt transformation
	/// and then projecting it back onto the image plane.
	/// This function can be used to visualize the results of the Odometry algorithm.
	/// ## Parameters
	/// * depth: Depth data, should be 1-channel CV_16U, CV_16S, CV_32F or CV_64F
	/// * image: RGB image (optional), should be 1-, 3- or 4-channel CV_8U
	/// * mask: Mask of used pixels (optional), should be CV_8UC1, CV_8SC1 or CV_BoolC1
	/// * Rt: Rotation+translation matrix (3x4 or 4x4) to be applied to depth points
	/// * cameraMatrix: Camera intrinsics matrix (3x3)
	/// * warpedDepth: The warped depth data (optional)
	/// * warpedImage: The warped RGB image (optional)
	/// * warpedMask: The mask of valid pixels in warped image (optional)
	///
	/// ## Note
	/// This alternative version of [warp_frame] function uses the following default values for its arguments:
	/// * warped_depth: noArray()
	/// * warped_image: noArray()
	/// * warped_mask: noArray()
	#[inline]
	pub fn warp_frame_def(depth: &impl ToInputArray, image: &impl ToInputArray, mask: &impl ToInputArray, rt: &impl ToInputArray, camera_matrix: &impl ToInputArray) -> Result<()> {
		input_array_arg!(depth);
		input_array_arg!(image);
		input_array_arg!(mask);
		input_array_arg!(rt);
		input_array_arg!(camera_matrix);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_warpFrame_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(depth.as_raw__InputArray(), image.as_raw__InputArray(), mask.as_raw__InputArray(), rt.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Warps depth or RGB-D image by reprojecting it in 3d, applying Rt transformation
	/// and then projecting it back onto the image plane.
	/// This function can be used to visualize the results of the Odometry algorithm.
	/// ## Parameters
	/// * depth: Depth data, should be 1-channel CV_16U, CV_16S, CV_32F or CV_64F
	/// * image: RGB image (optional), should be 1-, 3- or 4-channel CV_8U
	/// * mask: Mask of used pixels (optional), should be CV_8UC1, CV_8SC1 or CV_BoolC1
	/// * Rt: Rotation+translation matrix (3x4 or 4x4) to be applied to depth points
	/// * cameraMatrix: Camera intrinsics matrix (3x3)
	/// * warpedDepth: The warped depth data (optional)
	/// * warpedImage: The warped RGB image (optional)
	/// * warpedMask: The mask of valid pixels in warped image (optional)
	///
	/// ## C++ default parameters
	/// * warped_depth: noArray()
	/// * warped_image: noArray()
	/// * warped_mask: noArray()
	#[inline]
	pub fn warp_frame(depth: &impl ToInputArray, image: &impl ToInputArray, mask: &impl ToInputArray, rt: &impl ToInputArray, camera_matrix: &impl ToInputArray, warped_depth: &mut impl ToOutputArray, warped_image: &mut impl ToOutputArray, warped_mask: &mut impl ToOutputArray) -> Result<()> {
		input_array_arg!(depth);
		input_array_arg!(image);
		input_array_arg!(mask);
		input_array_arg!(rt);
		input_array_arg!(camera_matrix);
		output_array_arg!(warped_depth);
		output_array_arg!(warped_image);
		output_array_arg!(warped_mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_warpFrame_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(depth.as_raw__InputArray(), image.as_raw__InputArray(), mask.as_raw__InputArray(), rt.as_raw__InputArray(), camera_matrix.as_raw__InputArray(), warped_depth.as_raw__OutputArray(), warped_image.as_raw__OutputArray(), warped_mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Octree for 3D vision.
	///
	/// In 3D vision filed, the Octree is used to process and accelerate the pointcloud data. The class Octree represents
	/// the Octree data structure. Each Octree will have a fixed depth. The depth of Octree refers to the distance from
	/// the root node to the leaf node.All OctreeNodes will not exceed this depth.Increasing the depth will increase
	/// the amount of calculation exponentially. And the small number of depth refers low resolution of Octree.
	/// Each node contains 8 children, which are used to divide the space cube into eight parts. Each octree node represents
	/// a cube. And these eight children will have a fixed order, the order is described as follows:
	///
	/// For illustration, assume,
	///
	/// rootNode: origin == (0, 0, 0), size == 2
	///
	/// Then,
	///
	/// children[0]: origin == (0, 0, 0), size == 1
	///
	/// children[1]: origin == (1, 0, 0), size == 1, along X-axis next to child 0
	///
	/// children[2]: origin == (0, 1, 0), size == 1, along Y-axis next to child 0
	///
	/// children[3]: origin == (1, 1, 0), size == 1, in X-Y plane
	///
	/// children[4]: origin == (0, 0, 1), size == 1, along Z-axis next to child 0
	///
	/// children[5]: origin == (1, 0, 1), size == 1, in X-Z plane
	///
	/// children[6]: origin == (0, 1, 1), size == 1, in Y-Z plane
	///
	/// children[7]: origin == (1, 1, 1), size == 1, furthest from child 0
	pub struct Octree {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Octree }

	impl Drop for Octree {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_Octree_delete(self.as_raw_mut_Octree()) };
		}
	}

	unsafe impl Send for Octree {}

	impl Octree {
		/// Default constructor.
		#[inline]
		pub fn default() -> Result<crate::ptcloud::Octree> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_Octree(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::ptcloud::Octree::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
		/// Creates an empty Octree with given maximum depth
		///
		/// ## Parameters
		/// * maxDepth: The max depth of the Octree
		/// * size: bounding box size for the Octree
		/// * origin: Initial center coordinate
		/// * withColors: Whether to keep per-point colors or not
		/// ## Returns
		/// resulting Octree
		///
		/// ## C++ default parameters
		/// * origin: {}
		/// * with_colors: false
		#[inline]
		pub fn create_with_depth(max_depth: i32, size: f64, origin: core::Point3f, with_colors: bool) -> Result<core::Ptr<crate::ptcloud::Octree>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_createWithDepth_int_double_const_Point3fR_bool(max_depth, size, &origin, with_colors, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::ptcloud::Octree>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
		/// Creates an empty Octree with given maximum depth
		///
		/// ## Parameters
		/// * maxDepth: The max depth of the Octree
		/// * size: bounding box size for the Octree
		/// * origin: Initial center coordinate
		/// * withColors: Whether to keep per-point colors or not
		/// ## Returns
		/// resulting Octree
		///
		/// ## Note
		/// This alternative version of [Octree::create_with_depth] function uses the following default values for its arguments:
		/// * origin: {}
		/// * with_colors: false
		#[inline]
		pub fn create_with_depth_def(max_depth: i32, size: f64) -> Result<core::Ptr<crate::ptcloud::Octree>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_createWithDepth_int_double(max_depth, size, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::ptcloud::Octree>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
		/// Create an Octree from the PointCloud data with the specific maxDepth
		///
		/// ## Parameters
		/// * maxDepth: Max depth of the octree
		/// * pointCloud: point cloud data, should be 3-channel float array
		/// * colors: color attribute of point cloud in the same 3-channel float format
		/// ## Returns
		/// resulting Octree
		///
		/// ## C++ default parameters
		/// * colors: noArray()
		#[inline]
		pub fn create_with_depth_1(max_depth: i32, point_cloud: &impl ToInputArray, colors: &impl ToInputArray) -> Result<core::Ptr<crate::ptcloud::Octree>> {
			input_array_arg!(point_cloud);
			input_array_arg!(colors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_createWithDepth_int_const__InputArrayR_const__InputArrayR(max_depth, point_cloud.as_raw__InputArray(), colors.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::ptcloud::Octree>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
		/// Create an Octree from the PointCloud data with the specific maxDepth
		///
		/// ## Parameters
		/// * maxDepth: Max depth of the octree
		/// * pointCloud: point cloud data, should be 3-channel float array
		/// * colors: color attribute of point cloud in the same 3-channel float format
		/// ## Returns
		/// resulting Octree
		///
		/// ## Note
		/// This alternative version of [Octree::create_with_depth] function uses the following default values for its arguments:
		/// * colors: noArray()
		#[inline]
		pub fn create_with_depth_def_1(max_depth: i32, point_cloud: &impl ToInputArray) -> Result<core::Ptr<crate::ptcloud::Octree>> {
			input_array_arg!(point_cloud);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_createWithDepth_int_const__InputArrayR(max_depth, point_cloud.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::ptcloud::Octree>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
		/// Creates an empty Octree with given resolution
		///
		/// ## Parameters
		/// * resolution: The size of the octree leaf node
		/// * size: bounding box size for the Octree
		/// * origin: Initial center coordinate
		/// * withColors: Whether to keep per-point colors or not
		/// ## Returns
		/// resulting Octree
		///
		/// ## C++ default parameters
		/// * origin: {}
		/// * with_colors: false
		#[inline]
		pub fn create_with_resolution(resolution: f64, size: f64, origin: core::Point3f, with_colors: bool) -> Result<core::Ptr<crate::ptcloud::Octree>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_createWithResolution_double_double_const_Point3fR_bool(resolution, size, &origin, with_colors, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::ptcloud::Octree>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
		/// Creates an empty Octree with given resolution
		///
		/// ## Parameters
		/// * resolution: The size of the octree leaf node
		/// * size: bounding box size for the Octree
		/// * origin: Initial center coordinate
		/// * withColors: Whether to keep per-point colors or not
		/// ## Returns
		/// resulting Octree
		///
		/// ## Note
		/// This alternative version of [Octree::create_with_resolution] function uses the following default values for its arguments:
		/// * origin: {}
		/// * with_colors: false
		#[inline]
		pub fn create_with_resolution_def(resolution: f64, size: f64) -> Result<core::Ptr<crate::ptcloud::Octree>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_createWithResolution_double_double(resolution, size, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::ptcloud::Octree>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
		/// Create an Octree from the PointCloud data with the specific resolution
		///
		/// ## Parameters
		/// * resolution: The size of the octree leaf node
		/// * pointCloud: point cloud data, should be 3-channel float array
		/// * colors: color attribute of point cloud in the same 3-channel float format
		/// ## Returns
		/// resulting octree
		///
		/// ## C++ default parameters
		/// * colors: noArray()
		#[inline]
		pub fn create_with_resolution_1(resolution: f64, point_cloud: &impl ToInputArray, colors: &impl ToInputArray) -> Result<core::Ptr<crate::ptcloud::Octree>> {
			input_array_arg!(point_cloud);
			input_array_arg!(colors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_createWithResolution_double_const__InputArrayR_const__InputArrayR(resolution, point_cloud.as_raw__InputArray(), colors.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::ptcloud::Octree>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
		/// Create an Octree from the PointCloud data with the specific resolution
		///
		/// ## Parameters
		/// * resolution: The size of the octree leaf node
		/// * pointCloud: point cloud data, should be 3-channel float array
		/// * colors: color attribute of point cloud in the same 3-channel float format
		/// ## Returns
		/// resulting octree
		///
		/// ## Note
		/// This alternative version of [Octree::create_with_resolution] function uses the following default values for its arguments:
		/// * colors: noArray()
		#[inline]
		pub fn create_with_resolution_def_1(resolution: f64, point_cloud: &impl ToInputArray) -> Result<core::Ptr<crate::ptcloud::Octree>> {
			input_array_arg!(point_cloud);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_createWithResolution_double_const__InputArrayR(resolution, point_cloud.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::ptcloud::Octree>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::ptcloud::Octree]
	pub trait OctreeTraitConst {
		fn as_raw_Octree(&self) -> *const c_void;

		/// Determine whether the point is within the space range of the specific cube.
		///
		/// ## Parameters
		/// * point: The point coordinates.
		/// ## Returns
		/// If point is in bound, return ture. Otherwise, false.
		#[inline]
		fn is_point_in_bound(&self, point: core::Point3f) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_isPointInBound_const_const_Point3fR(self.as_raw_Octree(), &point, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// returns true if the rootnode is NULL.
		#[inline]
		fn empty(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_empty_const(self.as_raw_Octree(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Radius Nearest Neighbor Search in Octree.
		///
		/// Search all points that are less than or equal to radius.
		/// And return the number of searched points.
		/// ## Parameters
		/// * query: Query point.
		/// * radius: Retrieved radius value.
		/// * points: Point output. Contains searched points in 3-float format, and output vector is not in order,
		/// can be replaced by noArray() if not needed
		/// * squareDists: Dist output. Contains searched squared distance in floats, and output vector is not in order,
		/// can be omitted if not needed
		/// ## Returns
		/// the number of searched points.
		///
		/// ## C++ default parameters
		/// * square_dists: noArray()
		#[inline]
		fn radius_nn_search(&self, query: core::Point3f, radius: f32, points: &mut impl ToOutputArray, square_dists: &mut impl ToOutputArray) -> Result<i32> {
			output_array_arg!(points);
			output_array_arg!(square_dists);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_radiusNNSearch_const_const_Point3fR_float_const__OutputArrayR_const__OutputArrayR(self.as_raw_Octree(), &query, radius, points.as_raw__OutputArray(), square_dists.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Radius Nearest Neighbor Search in Octree.
		///
		/// Search all points that are less than or equal to radius.
		/// And return the number of searched points.
		/// ## Parameters
		/// * query: Query point.
		/// * radius: Retrieved radius value.
		/// * points: Point output. Contains searched points in 3-float format, and output vector is not in order,
		/// can be replaced by noArray() if not needed
		/// * squareDists: Dist output. Contains searched squared distance in floats, and output vector is not in order,
		/// can be omitted if not needed
		/// ## Returns
		/// the number of searched points.
		///
		/// ## Note
		/// This alternative version of [OctreeTraitConst::radius_nn_search] function uses the following default values for its arguments:
		/// * square_dists: noArray()
		#[inline]
		fn radius_nn_search_def(&self, query: core::Point3f, radius: f32, points: &mut impl ToOutputArray) -> Result<i32> {
			output_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_radiusNNSearch_const_const_Point3fR_float_const__OutputArrayR(self.as_raw_Octree(), &query, radius, points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Radius Nearest Neighbor Search in Octree.
		///
		/// Search all points that are less than or equal to radius.
		/// And return the number of searched points.
		/// ## Parameters
		/// * query: Query point.
		/// * radius: Retrieved radius value.
		/// * points: Point output. Contains searched points in 3-float format, and output vector is not in order,
		/// can be replaced by noArray() if not needed
		/// * squareDists: Dist output. Contains searched squared distance in floats, and output vector is not in order,
		/// can be omitted if not needed
		/// ## Returns
		/// the number of searched points.
		///
		/// ## Overloaded parameters
		///
		///  Radius Nearest Neighbor Search in Octree.
		///
		/// Search all points that are less than or equal to radius.
		/// And return the number of searched points.
		/// * query: Query point.
		/// * radius: Retrieved radius value.
		/// * points: Point output. Contains searched points in 3-float format, and output vector is not in order,
		/// can be replaced by noArray() if not needed
		/// * colors: Color output. Contains colors corresponding to points in pointSet, can be replaced by noArray() if not needed
		/// * squareDists: Dist output. Contains searched squared distance in floats, and output vector is not in order,
		/// can be replaced by noArray() if not needed
		/// ## Returns
		/// the number of searched points.
		#[inline]
		fn radius_nn_search_1(&self, query: core::Point3f, radius: f32, points: &mut impl ToOutputArray, colors: &mut impl ToOutputArray, square_dists: &mut impl ToOutputArray) -> Result<i32> {
			output_array_arg!(points);
			output_array_arg!(colors);
			output_array_arg!(square_dists);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_radiusNNSearch_const_const_Point3fR_float_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Octree(), &query, radius, points.as_raw__OutputArray(), colors.as_raw__OutputArray(), square_dists.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// K Nearest Neighbor Search in Octree.
		///
		/// Find the K nearest neighbors to the query point.
		/// ## Parameters
		/// * query: Query point.
		/// * K: amount of nearest neighbors to find
		/// * points: Point output. Contains K points in 3-float format, arranged in order of distance from near to far,
		/// can be replaced by noArray() if not needed
		/// * squareDists: Dist output. Contains K squared distance in floats, arranged in order of distance from near to far,
		/// can be omitted if not needed
		///
		/// ## C++ default parameters
		/// * square_dists: noArray()
		#[inline]
		fn knn_search(&self, query: core::Point3f, k: i32, points: &mut impl ToOutputArray, square_dists: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(points);
			output_array_arg!(square_dists);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_KNNSearch_const_const_Point3fR_const_int_const__OutputArrayR_const__OutputArrayR(self.as_raw_Octree(), &query, k, points.as_raw__OutputArray(), square_dists.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// K Nearest Neighbor Search in Octree.
		///
		/// Find the K nearest neighbors to the query point.
		/// ## Parameters
		/// * query: Query point.
		/// * K: amount of nearest neighbors to find
		/// * points: Point output. Contains K points in 3-float format, arranged in order of distance from near to far,
		/// can be replaced by noArray() if not needed
		/// * squareDists: Dist output. Contains K squared distance in floats, arranged in order of distance from near to far,
		/// can be omitted if not needed
		///
		/// ## Note
		/// This alternative version of [OctreeTraitConst::knn_search] function uses the following default values for its arguments:
		/// * square_dists: noArray()
		#[inline]
		fn knn_search_def(&self, query: core::Point3f, k: i32, points: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(points);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_KNNSearch_const_const_Point3fR_const_int_const__OutputArrayR(self.as_raw_Octree(), &query, k, points.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// K Nearest Neighbor Search in Octree.
		///
		/// Find the K nearest neighbors to the query point.
		/// ## Parameters
		/// * query: Query point.
		/// * K: amount of nearest neighbors to find
		/// * points: Point output. Contains K points in 3-float format, arranged in order of distance from near to far,
		/// can be replaced by noArray() if not needed
		/// * squareDists: Dist output. Contains K squared distance in floats, arranged in order of distance from near to far,
		/// can be omitted if not needed
		///
		/// ## Overloaded parameters
		///
		///  K Nearest Neighbor Search in Octree.
		///
		/// Find the K nearest neighbors to the query point.
		/// * query: Query point.
		/// * K: amount of nearest neighbors to find
		/// * points: Point output. Contains K points in 3-float format, arranged in order of distance from near to far,
		/// can be replaced by noArray() if not needed
		/// * colors: Color output. Contains colors corresponding to points in pointSet, can be replaced by noArray() if not needed
		/// * squareDists: Dist output. Contains K squared distance in floats, arranged in order of distance from near to far,
		/// can be replaced by noArray() if not needed
		#[inline]
		fn knn_search_1(&self, query: core::Point3f, k: i32, points: &mut impl ToOutputArray, colors: &mut impl ToOutputArray, square_dists: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(points);
			output_array_arg!(colors);
			output_array_arg!(square_dists);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_KNNSearch_const_const_Point3fR_const_int_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Octree(), &query, k, points.as_raw__OutputArray(), colors.as_raw__OutputArray(), square_dists.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::ptcloud::Octree]
	pub trait OctreeTrait: crate::ptcloud::OctreeTraitConst {
		fn as_raw_mut_Octree(&mut self) -> *mut c_void;

		/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
		/// Insert a point data with color to a OctreeNode.
		///
		/// ## Parameters
		/// * point: The point data in Point3f format.
		/// * color: The color attribute of point in Point3f format.
		/// ## Returns
		/// Returns whether the insertion is successful.
		///
		/// ## C++ default parameters
		/// * color: {}
		#[inline]
		fn insert_point(&mut self, point: core::Point3f, color: core::Point3f) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_insertPoint_const_Point3fR_const_Point3fR(self.as_raw_mut_Octree(), &point, &color, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
		/// Insert a point data with color to a OctreeNode.
		///
		/// ## Parameters
		/// * point: The point data in Point3f format.
		/// * color: The color attribute of point in Point3f format.
		/// ## Returns
		/// Returns whether the insertion is successful.
		///
		/// ## Note
		/// This alternative version of [OctreeTrait::insert_point] function uses the following default values for its arguments:
		/// * color: {}
		#[inline]
		fn insert_point_def(&mut self, point: core::Point3f) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_insertPoint_const_Point3fR(self.as_raw_mut_Octree(), &point, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Reset all octree parameter.
		///
		/// Clear all the nodes of the octree and initialize the parameters.
		#[inline]
		fn clear(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_clear(self.as_raw_mut_Octree(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Delete a given point from the Octree.
		///
		/// Delete the corresponding element from the pointList in the corresponding leaf node. If the leaf node
		/// does not contain other points after deletion, this node will be deleted. In the same way,
		/// its parent node may also be deleted if its last child is deleted.
		/// ## Parameters
		/// * point: The point coordinates, comparison is epsilon-based
		/// ## Returns
		/// return ture if the point is deleted successfully.
		#[inline]
		fn delete_point(&mut self, point: core::Point3f) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_deletePoint_const_Point3fR(self.as_raw_mut_Octree(), &point, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// restore point cloud data from Octree.
		///
		/// Restore the point cloud data from existing octree. The points in same leaf node will be seen as the same point.
		/// This point is the center of the leaf node. If the resolution is small, it will work as a downSampling function.
		/// ## Parameters
		/// * restoredPointCloud: The output point cloud data, can be replaced by noArray() if not needed
		/// * restoredColor: The color attribute of point cloud data, can be omitted if not needed
		///
		/// ## C++ default parameters
		/// * restored_color: noArray()
		#[inline]
		fn get_point_cloud_by_octree(&mut self, restored_point_cloud: &mut impl ToOutputArray, restored_color: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(restored_point_cloud);
			output_array_arg!(restored_color);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_getPointCloudByOctree_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_Octree(), restored_point_cloud.as_raw__OutputArray(), restored_color.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// restore point cloud data from Octree.
		///
		/// Restore the point cloud data from existing octree. The points in same leaf node will be seen as the same point.
		/// This point is the center of the leaf node. If the resolution is small, it will work as a downSampling function.
		/// ## Parameters
		/// * restoredPointCloud: The output point cloud data, can be replaced by noArray() if not needed
		/// * restoredColor: The color attribute of point cloud data, can be omitted if not needed
		///
		/// ## Note
		/// This alternative version of [OctreeTrait::get_point_cloud_by_octree] function uses the following default values for its arguments:
		/// * restored_color: noArray()
		#[inline]
		fn get_point_cloud_by_octree_def(&mut self, restored_point_cloud: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(restored_point_cloud);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Octree_getPointCloudByOctree_const__OutputArrayR(self.as_raw_mut_Octree(), restored_point_cloud.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Octree {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Octree")
				.finish()
		}
	}

	impl crate::ptcloud::OctreeTraitConst for Octree {
		#[inline] fn as_raw_Octree(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::ptcloud::OctreeTrait for Octree {
		#[inline] fn as_raw_mut_Octree(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Octree, crate::ptcloud::OctreeTraitConst, as_raw_Octree, crate::ptcloud::OctreeTrait, as_raw_mut_Octree }

	pub struct Odometry {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Odometry }

	impl Drop for Odometry {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_Odometry_delete(self.as_raw_mut_Odometry()) };
		}
	}

	unsafe impl Send for Odometry {}

	impl Odometry {
		#[inline]
		pub fn default() -> Result<crate::ptcloud::Odometry> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Odometry_Odometry(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::ptcloud::Odometry::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		pub fn new(otype: crate::ptcloud::OdometryType) -> Result<crate::ptcloud::Odometry> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Odometry_Odometry_OdometryType(otype, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::ptcloud::Odometry::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		pub fn new_1(otype: crate::ptcloud::OdometryType, settings: &impl crate::ptcloud::OdometrySettingsTraitConst, algtype: crate::ptcloud::OdometryAlgoType) -> Result<crate::ptcloud::Odometry> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Odometry_Odometry_OdometryType_const_OdometrySettingsR_OdometryAlgoType(otype, settings.as_raw_OdometrySettings(), algtype, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::ptcloud::Odometry::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::ptcloud::Odometry]
	pub trait OdometryTraitConst {
		fn as_raw_Odometry(&self) -> *const c_void;

		/// Prepare frame for odometry calculation
		/// ## Parameters
		/// * frame: odometry prepare this frame as src frame and dst frame simultaneously
		#[inline]
		fn prepare_frame(&self, frame: &mut impl crate::ptcloud::OdometryFrameTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Odometry_prepareFrame_const_OdometryFrameR(self.as_raw_Odometry(), frame.as_raw_mut_OdometryFrame(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Prepare frame for odometry calculation
		/// ## Parameters
		/// * srcFrame: frame will be prepared as src frame ("original" image)
		/// * dstFrame: frame will be prepared as dsr frame ("rotated" image)
		#[inline]
		fn prepare_frames(&self, src_frame: &mut impl crate::ptcloud::OdometryFrameTrait, dst_frame: &mut impl crate::ptcloud::OdometryFrameTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Odometry_prepareFrames_const_OdometryFrameR_OdometryFrameR(self.as_raw_Odometry(), src_frame.as_raw_mut_OdometryFrame(), dst_frame.as_raw_mut_OdometryFrame(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Compute Rigid Transformation between two frames so that Rt * src = dst
		/// Both frames, source and destination, should have been prepared by calling prepareFrame() first
		///
		/// ## Parameters
		/// * srcFrame: src frame ("original" image)
		/// * dstFrame: dst frame ("rotated" image)
		/// * Rt: Rigid transformation, which will be calculated, in form:
		/// { R_11 R_12 R_13 t_1
		///   R_21 R_22 R_23 t_2
		///   R_31 R_32 R_33 t_3
		///   0    0    0    1  }
		/// ## Returns
		/// true on success, false if failed to find the transformation
		#[inline]
		fn compute(&self, src_frame: &impl crate::ptcloud::OdometryFrameTraitConst, dst_frame: &impl crate::ptcloud::OdometryFrameTraitConst, rt: &mut impl ToOutputArray) -> Result<bool> {
			output_array_arg!(rt);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Odometry_compute_const_const_OdometryFrameR_const_OdometryFrameR_const__OutputArrayR(self.as_raw_Odometry(), src_frame.as_raw_OdometryFrame(), dst_frame.as_raw_OdometryFrame(), rt.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Compute Rigid Transformation between two frames so that Rt * src = dst
		///
		/// ## Parameters
		/// * srcDepth: source depth ("original" image)
		/// * dstDepth: destination depth ("rotated" image)
		/// * Rt: Rigid transformation, which will be calculated, in form:
		/// { R_11 R_12 R_13 t_1
		///   R_21 R_22 R_23 t_2
		///   R_31 R_32 R_33 t_3
		///   0    0    0    1  }
		/// ## Returns
		/// true on success, false if failed to find the transformation
		#[inline]
		fn compute_1(&self, src_depth: &impl ToInputArray, dst_depth: &impl ToInputArray, rt: &mut impl ToOutputArray) -> Result<bool> {
			input_array_arg!(src_depth);
			input_array_arg!(dst_depth);
			output_array_arg!(rt);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Odometry_compute_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_Odometry(), src_depth.as_raw__InputArray(), dst_depth.as_raw__InputArray(), rt.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Compute Rigid Transformation between two frames so that Rt * src = dst
		///
		/// ## Parameters
		/// * srcDepth: source depth ("original" image)
		/// * srcRGB: source RGB
		/// * dstDepth: destination depth ("rotated" image)
		/// * dstRGB: destination RGB
		/// * Rt: Rigid transformation, which will be calculated, in form:
		/// { R_11 R_12 R_13 t_1
		///   R_21 R_22 R_23 t_2
		///   R_31 R_32 R_33 t_3
		///   0    0    0    1  }
		/// ## Returns
		/// true on success, false if failed to find the transformation
		#[inline]
		fn compute_2(&self, src_depth: &impl ToInputArray, src_rgb: &impl ToInputArray, dst_depth: &impl ToInputArray, dst_rgb: &impl ToInputArray, rt: &mut impl ToOutputArray) -> Result<bool> {
			input_array_arg!(src_depth);
			input_array_arg!(src_rgb);
			input_array_arg!(dst_depth);
			input_array_arg!(dst_rgb);
			output_array_arg!(rt);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Odometry_compute_const_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_Odometry(), src_depth.as_raw__InputArray(), src_rgb.as_raw__InputArray(), dst_depth.as_raw__InputArray(), dst_rgb.as_raw__InputArray(), rt.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the normals computer object used for normals calculation (if presented).
		/// The normals computer is generated at first need during prepareFrame when normals are required for the ICP algorithm
		/// but not presented by a user. Re-generated each time the related settings change or a new frame arrives with the different size.
		#[inline]
		fn get_normals_computer(&self) -> Result<core::Ptr<crate::ptcloud::RgbdNormals>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Odometry_getNormalsComputer_const(self.as_raw_Odometry(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::ptcloud::RgbdNormals>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::ptcloud::Odometry]
	pub trait OdometryTrait: crate::ptcloud::OdometryTraitConst {
		fn as_raw_mut_Odometry(&mut self) -> *mut c_void;

	}

	impl std::fmt::Debug for Odometry {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Odometry")
				.finish()
		}
	}

	impl crate::ptcloud::OdometryTraitConst for Odometry {
		#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::ptcloud::OdometryTrait for Odometry {
		#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Odometry, crate::ptcloud::OdometryTraitConst, as_raw_Odometry, crate::ptcloud::OdometryTrait, as_raw_mut_Odometry }

	/// An object that keeps per-frame data for Odometry algorithms from user-provided images to algorithm-specific precalculated data.
	/// When not empty, it contains a depth image, a mask of valid pixels and a set of pyramids generated from that data.
	/// A BGR/Gray image and normals are optional.
	/// OdometryFrame is made to be used together with Odometry class to reuse precalculated data between Rt data calculations.
	/// A correct way to do that is to call Odometry::prepareFrames() on prev and next frames and then pass them to Odometry::compute() method.
	pub struct OdometryFrame {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { OdometryFrame }

	impl Drop for OdometryFrame {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_OdometryFrame_delete(self.as_raw_mut_OdometryFrame()) };
		}
	}

	unsafe impl Send for OdometryFrame {}

	impl OdometryFrame {
		/// Construct a new OdometryFrame object. All non-empty images should have the same size.
		///
		/// ## Parameters
		/// * depth: A depth image, should be CV_8UC1
		/// * image: An BGR or grayscale image (or noArray() if it's not required for used ICP algorithm).
		/// Should be CV_8UC3 or CV_8C4 if it's BGR image or CV_8UC1 if it's grayscale. If it's BGR then it's converted to grayscale
		/// image automatically.
		/// * mask: A user-provided mask of valid pixels, should be CV_8UC1
		/// * normals: A user-provided normals to the depth surface, should be CV_32FC4
		///
		/// ## C++ default parameters
		/// * depth: noArray()
		/// * image: noArray()
		/// * mask: noArray()
		/// * normals: noArray()
		#[inline]
		pub fn new(depth: &impl ToInputArray, image: &impl ToInputArray, mask: &impl ToInputArray, normals: &impl ToInputArray) -> Result<crate::ptcloud::OdometryFrame> {
			input_array_arg!(depth);
			input_array_arg!(image);
			input_array_arg!(mask);
			input_array_arg!(normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometryFrame_OdometryFrame_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(depth.as_raw__InputArray(), image.as_raw__InputArray(), mask.as_raw__InputArray(), normals.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::ptcloud::OdometryFrame::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Construct a new OdometryFrame object. All non-empty images should have the same size.
		///
		/// ## Parameters
		/// * depth: A depth image, should be CV_8UC1
		/// * image: An BGR or grayscale image (or noArray() if it's not required for used ICP algorithm).
		/// Should be CV_8UC3 or CV_8C4 if it's BGR image or CV_8UC1 if it's grayscale. If it's BGR then it's converted to grayscale
		/// image automatically.
		/// * mask: A user-provided mask of valid pixels, should be CV_8UC1
		/// * normals: A user-provided normals to the depth surface, should be CV_32FC4
		///
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * depth: noArray()
		/// * image: noArray()
		/// * mask: noArray()
		/// * normals: noArray()
		#[inline]
		pub fn new_def() -> Result<crate::ptcloud::OdometryFrame> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometryFrame_OdometryFrame(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::ptcloud::OdometryFrame::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::ptcloud::OdometryFrame]
	pub trait OdometryFrameTraitConst {
		fn as_raw_OdometryFrame(&self) -> *const c_void;

		/// Get the original user-provided BGR/Gray image
		///
		/// ## Parameters
		/// * image: Output image
		#[inline]
		fn get_image(&self, image: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometryFrame_getImage_const_const__OutputArrayR(self.as_raw_OdometryFrame(), image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the gray image generated from the user-provided BGR/Gray image
		///
		/// ## Parameters
		/// * image: Output image
		#[inline]
		fn get_gray_image(&self, image: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometryFrame_getGrayImage_const_const__OutputArrayR(self.as_raw_OdometryFrame(), image.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the original user-provided depth image
		///
		/// ## Parameters
		/// * depth: Output image
		#[inline]
		fn get_depth(&self, depth: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(depth);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometryFrame_getDepth_const_const__OutputArrayR(self.as_raw_OdometryFrame(), depth.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the depth image generated from the user-provided one after conversion, rescale or filtering for ICP algorithm needs
		///
		/// ## Parameters
		/// * depth: Output image
		#[inline]
		fn get_processed_depth(&self, depth: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(depth);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometryFrame_getProcessedDepth_const_const__OutputArrayR(self.as_raw_OdometryFrame(), depth.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the valid pixels mask generated for the ICP calculations intersected with the user-provided mask
		///
		/// ## Parameters
		/// * mask: Output image
		#[inline]
		fn get_mask(&self, mask: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometryFrame_getMask_const_const__OutputArrayR(self.as_raw_OdometryFrame(), mask.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the normals image either generated for the ICP calculations or user-provided
		///
		/// ## Parameters
		/// * normals: Output image
		#[inline]
		fn get_normals(&self, normals: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometryFrame_getNormals_const_const__OutputArrayR(self.as_raw_OdometryFrame(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the amount of levels in pyramids (all of them if not empty should have the same number of levels)
		/// or 0 if no pyramids were prepared yet
		#[inline]
		fn get_pyramid_levels(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometryFrame_getPyramidLevels_const(self.as_raw_OdometryFrame(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Get the image generated for the ICP calculations from one of the pyramids specified by pyrType. Returns empty image if
		/// the pyramid is empty or there's no such pyramid level
		///
		/// ## Parameters
		/// * img: Output image
		/// * pyrType: Type of pyramid
		/// * level: Level in the pyramid
		#[inline]
		fn get_pyramid_at(&self, img: &mut impl ToOutputArray, pyr_type: crate::ptcloud::OdometryFramePyramidType, level: size_t) -> Result<()> {
			output_array_arg!(img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometryFrame_getPyramidAt_const_const__OutputArrayR_OdometryFramePyramidType_size_t(self.as_raw_OdometryFrame(), img.as_raw__OutputArray(), pyr_type, level, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::ptcloud::OdometryFrame]
	pub trait OdometryFrameTrait: crate::ptcloud::OdometryFrameTraitConst {
		fn as_raw_mut_OdometryFrame(&mut self) -> *mut c_void;

	}

	impl Clone for OdometryFrame {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_OdometryFrame_implicitClone_const(self.as_raw_OdometryFrame())) }
		}
	}

	impl std::fmt::Debug for OdometryFrame {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("OdometryFrame")
				.finish()
		}
	}

	impl crate::ptcloud::OdometryFrameTraitConst for OdometryFrame {
		#[inline] fn as_raw_OdometryFrame(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::ptcloud::OdometryFrameTrait for OdometryFrame {
		#[inline] fn as_raw_mut_OdometryFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { OdometryFrame, crate::ptcloud::OdometryFrameTraitConst, as_raw_OdometryFrame, crate::ptcloud::OdometryFrameTrait, as_raw_mut_OdometryFrame }

	pub struct OdometrySettings {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { OdometrySettings }

	impl Drop for OdometrySettings {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_OdometrySettings_delete(self.as_raw_mut_OdometrySettings()) };
		}
	}

	unsafe impl Send for OdometrySettings {}

	impl OdometrySettings {
		#[inline]
		pub fn default() -> Result<crate::ptcloud::OdometrySettings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_OdometrySettings(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::ptcloud::OdometrySettings::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		pub fn copy(unnamed: &impl crate::ptcloud::OdometrySettingsTraitConst) -> Result<crate::ptcloud::OdometrySettings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_OdometrySettings_const_OdometrySettingsR(unnamed.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::ptcloud::OdometrySettings::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::ptcloud::OdometrySettings]
	pub trait OdometrySettingsTraitConst {
		fn as_raw_OdometrySettings(&self) -> *const c_void;

		#[inline]
		fn get_camera_matrix(&self, val: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(val);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_getCameraMatrix_const_const__OutputArrayR(self.as_raw_OdometrySettings(), val.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_iter_counts(&self, val: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(val);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_getIterCounts_const_const__OutputArrayR(self.as_raw_OdometrySettings(), val.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_min_depth(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_getMinDepth_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_max_depth(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_getMaxDepth_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_max_depth_diff(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_getMaxDepthDiff_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_max_points_part(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_getMaxPointsPart_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_sobel_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_getSobelSize_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_sobel_scale(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_getSobelScale_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_normal_win_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_getNormalWinSize_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_normal_diff_threshold(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_getNormalDiffThreshold_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_normal_method(&self) -> Result<crate::ptcloud::RgbdNormals_RgbdNormalsMethod> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_getNormalMethod_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_angle_threshold(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_getAngleThreshold_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_max_translation(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_getMaxTranslation_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_max_rotation(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_getMaxRotation_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_min_gradient_magnitude(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_getMinGradientMagnitude_const(self.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_min_gradient_magnitudes(&self, val: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(val);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_getMinGradientMagnitudes_const_const__OutputArrayR(self.as_raw_OdometrySettings(), val.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::ptcloud::OdometrySettings]
	pub trait OdometrySettingsTrait: crate::ptcloud::OdometrySettingsTraitConst {
		fn as_raw_mut_OdometrySettings(&mut self) -> *mut c_void;

		#[inline]
		fn set(&mut self, unnamed: &impl crate::ptcloud::OdometrySettingsTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_operatorST_const_OdometrySettingsR(self.as_raw_mut_OdometrySettings(), unnamed.as_raw_OdometrySettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_camera_matrix(&mut self, val: &impl ToInputArray) -> Result<()> {
			input_array_arg!(val);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_setCameraMatrix_const__InputArrayR(self.as_raw_mut_OdometrySettings(), val.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_iter_counts(&mut self, val: &impl ToInputArray) -> Result<()> {
			input_array_arg!(val);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_setIterCounts_const__InputArrayR(self.as_raw_mut_OdometrySettings(), val.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_min_depth(&mut self, val: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_setMinDepth_float(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_max_depth(&mut self, val: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_setMaxDepth_float(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_max_depth_diff(&mut self, val: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_setMaxDepthDiff_float(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_max_points_part(&mut self, val: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_setMaxPointsPart_float(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_sobel_size(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_setSobelSize_int(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_sobel_scale(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_setSobelScale_double(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_normal_win_size(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_setNormalWinSize_int(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_normal_diff_threshold(&mut self, val: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_setNormalDiffThreshold_float(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_normal_method(&mut self, nm: crate::ptcloud::RgbdNormals_RgbdNormalsMethod) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_setNormalMethod_RgbdNormalsMethod(self.as_raw_mut_OdometrySettings(), nm, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_angle_threshold(&mut self, val: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_setAngleThreshold_float(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_max_translation(&mut self, val: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_setMaxTranslation_float(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_max_rotation(&mut self, val: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_setMaxRotation_float(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_min_gradient_magnitude(&mut self, val: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_setMinGradientMagnitude_float(self.as_raw_mut_OdometrySettings(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_min_gradient_magnitudes(&mut self, val: &impl ToInputArray) -> Result<()> {
			input_array_arg!(val);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_OdometrySettings_setMinGradientMagnitudes_const__InputArrayR(self.as_raw_mut_OdometrySettings(), val.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl Clone for OdometrySettings {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_OdometrySettings_implicitClone_const(self.as_raw_OdometrySettings())) }
		}
	}

	impl std::fmt::Debug for OdometrySettings {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("OdometrySettings")
				.finish()
		}
	}

	impl crate::ptcloud::OdometrySettingsTraitConst for OdometrySettings {
		#[inline] fn as_raw_OdometrySettings(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::ptcloud::OdometrySettingsTrait for OdometrySettings {
		#[inline] fn as_raw_mut_OdometrySettings(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { OdometrySettings, crate::ptcloud::OdometrySettingsTraitConst, as_raw_OdometrySettings, crate::ptcloud::OdometrySettingsTrait, as_raw_mut_OdometrySettings }

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
		ptr: *mut c_void,
	}

	opencv_type_boxed! { RgbdNormals }

	impl Drop for RgbdNormals {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_RgbdNormals_delete(self.as_raw_mut_RgbdNormals()) };
		}
	}

	unsafe impl Send for RgbdNormals {}

	impl RgbdNormals {
		/// Creates new RgbdNormals object
		/// ## Parameters
		/// * rows: the number of rows of the depth image normals will be computed on
		/// * cols: the number of cols of the depth image normals will be computed on
		/// * depth: the depth of the normals (only CV_32F or CV_64F)
		/// * K: the calibration matrix to use
		/// * window_size: the window size to compute the normals: can only be 1,3,5 or 7
		/// * diff_threshold: threshold in depth difference, used in LINEMOD algirithm
		/// * method: one of the methods to use: RGBD_NORMALS_METHOD_SRI, RGBD_NORMALS_METHOD_FALS
		///
		/// ## C++ default parameters
		/// * rows: 0
		/// * cols: 0
		/// * depth: 0
		/// * k: noArray()
		/// * window_size: 5
		/// * diff_threshold: 50.f
		/// * method: RgbdNormals::RgbdNormalsMethod::RGBD_NORMALS_METHOD_FALS
		#[inline]
		pub fn create(rows: i32, cols: i32, depth: i32, k: &impl ToInputArray, window_size: i32, diff_threshold: f32, method: crate::ptcloud::RgbdNormals_RgbdNormalsMethod) -> Result<core::Ptr<crate::ptcloud::RgbdNormals>> {
			input_array_arg!(k);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RgbdNormals_create_int_int_int_const__InputArrayR_int_float_RgbdNormalsMethod(rows, cols, depth, k.as_raw__InputArray(), window_size, diff_threshold, method, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::ptcloud::RgbdNormals>::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Creates new RgbdNormals object
		/// ## Parameters
		/// * rows: the number of rows of the depth image normals will be computed on
		/// * cols: the number of cols of the depth image normals will be computed on
		/// * depth: the depth of the normals (only CV_32F or CV_64F)
		/// * K: the calibration matrix to use
		/// * window_size: the window size to compute the normals: can only be 1,3,5 or 7
		/// * diff_threshold: threshold in depth difference, used in LINEMOD algirithm
		/// * method: one of the methods to use: RGBD_NORMALS_METHOD_SRI, RGBD_NORMALS_METHOD_FALS
		///
		/// ## Note
		/// This alternative version of [RgbdNormals::create] function uses the following default values for its arguments:
		/// * rows: 0
		/// * cols: 0
		/// * depth: 0
		/// * k: noArray()
		/// * window_size: 5
		/// * diff_threshold: 50.f
		/// * method: RgbdNormals::RgbdNormalsMethod::RGBD_NORMALS_METHOD_FALS
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::ptcloud::RgbdNormals>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RgbdNormals_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::ptcloud::RgbdNormals>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::ptcloud::RgbdNormals]
	pub trait RgbdNormalsTraitConst {
		fn as_raw_RgbdNormals(&self) -> *const c_void;

		/// Given a set of 3d points in a depth image, compute the normals at each point.
		/// ## Parameters
		/// * points: a rows x cols x 3 matrix of CV_32F/CV64F or a rows x cols x 1 CV_U16S
		/// * normals: a rows x cols x 3 matrix
		#[inline]
		fn apply(&self, points: &impl ToInputArray, normals: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(points);
			output_array_arg!(normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RgbdNormals_apply_const_const__InputArrayR_const__OutputArrayR(self.as_raw_RgbdNormals(), points.as_raw__InputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Prepares cached data required for calculation
		/// If not called by user, called automatically at first calculation
		#[inline]
		fn cache(&self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RgbdNormals_cache_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_rows(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RgbdNormals_getRows_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_cols(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RgbdNormals_getCols_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_window_size(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RgbdNormals_getWindowSize_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_depth(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RgbdNormals_getDepth_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_k(&self, val: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(val);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RgbdNormals_getK_const_const__OutputArrayR(self.as_raw_RgbdNormals(), val.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_method(&self) -> Result<crate::ptcloud::RgbdNormals_RgbdNormalsMethod> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RgbdNormals_getMethod_const(self.as_raw_RgbdNormals(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::ptcloud::RgbdNormals]
	pub trait RgbdNormalsTrait: crate::ptcloud::RgbdNormalsTraitConst {
		fn as_raw_mut_RgbdNormals(&mut self) -> *mut c_void;

		#[inline]
		fn set_rows(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RgbdNormals_setRows_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_cols(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RgbdNormals_setCols_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_window_size(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RgbdNormals_setWindowSize_int(self.as_raw_mut_RgbdNormals(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_k(&mut self, val: &impl ToInputArray) -> Result<()> {
			input_array_arg!(val);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RgbdNormals_setK_const__InputArrayR(self.as_raw_mut_RgbdNormals(), val.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for RgbdNormals {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("RgbdNormals")
				.finish()
		}
	}

	impl crate::ptcloud::RgbdNormalsTraitConst for RgbdNormals {
		#[inline] fn as_raw_RgbdNormals(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::ptcloud::RgbdNormalsTrait for RgbdNormals {
		#[inline] fn as_raw_mut_RgbdNormals(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { RgbdNormals, crate::ptcloud::RgbdNormalsTraitConst, as_raw_RgbdNormals, crate::ptcloud::RgbdNormalsTrait, as_raw_mut_RgbdNormals }

	/// Structure to keep settings for rasterization
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct TriangleRasterizeSettings {
		pub shading_type: crate::ptcloud::TriangleShadingType,
		pub culling_mode: crate::ptcloud::TriangleCullingMode,
		pub gl_compatible_mode: crate::ptcloud::TriangleGlCompatibleMode,
	}

	opencv_type_simple! { crate::ptcloud::TriangleRasterizeSettings }

	impl TriangleRasterizeSettings {
		#[inline]
		pub fn default() -> Result<crate::ptcloud::TriangleRasterizeSettings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_TriangleRasterizeSettings_TriangleRasterizeSettings(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		pub fn set_shading_type(self, st: crate::ptcloud::TriangleShadingType) -> Result<crate::ptcloud::TriangleRasterizeSettings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_TriangleRasterizeSettings_setShadingType_TriangleShadingType(&self, st, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		pub fn set_culling_mode(self, cm: crate::ptcloud::TriangleCullingMode) -> Result<crate::ptcloud::TriangleRasterizeSettings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_TriangleRasterizeSettings_setCullingMode_TriangleCullingMode(&self, cm, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		pub fn set_gl_compatible_mode(self, gm: crate::ptcloud::TriangleGlCompatibleMode) -> Result<crate::ptcloud::TriangleRasterizeSettings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_TriangleRasterizeSettings_setGlCompatibleMode_TriangleGlCompatibleMode(&self, gm, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	pub struct Volume {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { Volume }

	impl Drop for Volume {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_Volume_delete(self.as_raw_mut_Volume()) };
		}
	}

	unsafe impl Send for Volume {}

	impl Volume {
		/// Constructor of custom volume.
		/// ## Parameters
		/// * vtype: the volume type [TSDF, HashTSDF, ColorTSDF].
		/// * settings: the custom settings for volume.
		///
		/// ## C++ default parameters
		/// * vtype: VolumeType::TSDF
		/// * settings: VolumeSettings(VolumeType::TSDF)
		#[inline]
		pub fn new(vtype: crate::ptcloud::VolumeType, settings: &impl crate::ptcloud::VolumeSettingsTraitConst) -> Result<crate::ptcloud::Volume> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Volume_Volume_VolumeType_const_VolumeSettingsR(vtype, settings.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::ptcloud::Volume::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Constructor of custom volume.
		/// ## Parameters
		/// * vtype: the volume type [TSDF, HashTSDF, ColorTSDF].
		/// * settings: the custom settings for volume.
		///
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * vtype: VolumeType::TSDF
		/// * settings: VolumeSettings(VolumeType::TSDF)
		#[inline]
		pub fn new_def() -> Result<crate::ptcloud::Volume> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Volume_Volume(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::ptcloud::Volume::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::ptcloud::Volume]
	pub trait VolumeTraitConst {
		fn as_raw_Volume(&self) -> *const c_void;

		/// Renders the volume contents into an image. The resulting points and normals are in camera's coordinate system.
		///
		/// Rendered image size and camera intrinsics are taken from volume settings structure.
		///
		/// ## Parameters
		/// * cameraPose: the pose of camera in global coordinates.
		/// * points: image to store rendered points.
		/// * normals: image to store rendered normals corresponding to points.
		#[inline]
		fn raycast(&self, camera_pose: &impl ToInputArray, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(camera_pose);
			output_array_arg!(points);
			output_array_arg!(normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Volume_raycast_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Volume(), camera_pose.as_raw__InputArray(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Renders the volume contents into an image. The resulting points and normals are in camera's coordinate system.
		///
		/// Rendered image size and camera intrinsics are taken from volume settings structure.
		///
		/// ## Parameters
		/// * cameraPose: the pose of camera in global coordinates.
		/// * points: image to store rendered points.
		/// * normals: image to store rendered normals corresponding to points.
		/// * colors: image to store rendered colors corresponding to points (only for ColorTSDF).
		#[inline]
		fn raycast_color(&self, camera_pose: &impl ToInputArray, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray, colors: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(camera_pose);
			output_array_arg!(points);
			output_array_arg!(normals);
			output_array_arg!(colors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Volume_raycast_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Volume(), camera_pose.as_raw__InputArray(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), colors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Renders the volume contents into an image. The resulting points and normals are in camera's coordinate system.
		///
		/// Rendered image size and camera intrinsics are taken from volume settings structure.
		///
		/// ## Parameters
		/// * cameraPose: the pose of camera in global coordinates.
		/// * height: the height of result image
		/// * width: the width of result image
		/// * K: camera raycast intrinsics
		/// * points: image to store rendered points.
		/// * normals: image to store rendered normals corresponding to points.
		#[inline]
		fn raycast_ex(&self, camera_pose: &impl ToInputArray, height: i32, width: i32, k: &impl ToInputArray, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(camera_pose);
			input_array_arg!(k);
			output_array_arg!(points);
			output_array_arg!(normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Volume_raycast_const_const__InputArrayR_int_int_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Volume(), camera_pose.as_raw__InputArray(), height, width, k.as_raw__InputArray(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Renders the volume contents into an image. The resulting points and normals are in camera's coordinate system.
		///
		/// Rendered image size and camera intrinsics are taken from volume settings structure.
		///
		/// ## Parameters
		/// * cameraPose: the pose of camera in global coordinates.
		/// * height: the height of result image
		/// * width: the width of result image
		/// * K: camera raycast intrinsics
		/// * points: image to store rendered points.
		/// * normals: image to store rendered normals corresponding to points.
		/// * colors: image to store rendered colors corresponding to points (only for ColorTSDF).
		#[inline]
		fn raycast_ex_color(&self, camera_pose: &impl ToInputArray, height: i32, width: i32, k: &impl ToInputArray, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray, colors: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(camera_pose);
			input_array_arg!(k);
			output_array_arg!(points);
			output_array_arg!(normals);
			output_array_arg!(colors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Volume_raycast_const_const__InputArrayR_int_int_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Volume(), camera_pose.as_raw__InputArray(), height, width, k.as_raw__InputArray(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), colors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Extract the all data from volume.
		/// ## Parameters
		/// * points: the input exist point.
		/// * normals: the storage of normals (corresponding to input points) in the image.
		#[inline]
		fn fetch_normals(&self, points: &impl ToInputArray, normals: &mut impl ToOutputArray) -> Result<()> {
			input_array_arg!(points);
			output_array_arg!(normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Volume_fetchNormals_const_const__InputArrayR_const__OutputArrayR(self.as_raw_Volume(), points.as_raw__InputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Extract the all data from volume.
		/// ## Parameters
		/// * points: the storage of all points.
		/// * normals: the storage of all normals, corresponding to points.
		#[inline]
		fn fetch_points_normals(&self, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(points);
			output_array_arg!(normals);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Volume_fetchPointsNormals_const_const__OutputArrayR_const__OutputArrayR(self.as_raw_Volume(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Extract the all data from volume.
		/// ## Parameters
		/// * points: the storage of all points.
		/// * normals: the storage of all normals, corresponding to points.
		/// * colors: the storage of all colors, corresponding to points (only for ColorTSDF).
		#[inline]
		fn fetch_points_normals_colors(&self, points: &mut impl ToOutputArray, normals: &mut impl ToOutputArray, colors: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(points);
			output_array_arg!(normals);
			output_array_arg!(colors);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Volume_fetchPointsNormalsColors_const_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_Volume(), points.as_raw__OutputArray(), normals.as_raw__OutputArray(), colors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// return visible blocks in volume.
		#[inline]
		fn get_visible_blocks(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Volume_getVisibleBlocks_const(self.as_raw_Volume(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// return number of volume units in volume.
		#[inline]
		fn get_total_volume_units(&self) -> Result<size_t> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Volume_getTotalVolumeUnits_const(self.as_raw_Volume(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Gets bounding box in volume coordinates with given precision:
		/// VOLUME_UNIT - up to volume unit
		/// VOXEL - up to voxel (currently not supported)
		/// ## Parameters
		/// * bb: 6-float 1d array containing (min_x, min_y, min_z, max_x, max_y, max_z) in volume coordinates
		/// * precision: bounding box calculation precision
		#[inline]
		fn get_bounding_box(&self, bb: &mut impl ToOutputArray, precision: i32) -> Result<()> {
			output_array_arg!(bb);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Volume_getBoundingBox_const_const__OutputArrayR_int(self.as_raw_Volume(), bb.as_raw__OutputArray(), precision, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns if new volume units are allocated during integration or not.
		/// Makes sense for HashTSDF only.
		#[inline]
		fn get_enable_growth(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Volume_getEnableGrowth_const(self.as_raw_Volume(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::ptcloud::Volume]
	pub trait VolumeTrait: crate::ptcloud::VolumeTraitConst {
		fn as_raw_mut_Volume(&mut self) -> *mut c_void;

		/// Integrates the input data to the volume.
		///
		/// Camera intrinsics are taken from volume settings structure.
		///
		/// ## Parameters
		/// * frame: the object from which to take depth and image data.
		///   For color TSDF a depth data should be registered with color data, i.e. have the same intrinsics & camera pose.
		///   This can be done using function registerDepth() from 3d module.
		/// * pose: the pose of camera in global coordinates.
		#[inline]
		fn integrate_frame(&mut self, frame: &impl crate::ptcloud::OdometryFrameTraitConst, pose: &impl ToInputArray) -> Result<()> {
			input_array_arg!(pose);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Volume_integrate_const_OdometryFrameR_const__InputArrayR(self.as_raw_mut_Volume(), frame.as_raw_OdometryFrame(), pose.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Integrates the input data to the volume.
		///
		/// Camera intrinsics are taken from volume settings structure.
		///
		/// ## Parameters
		/// * depth: the depth image.
		/// * pose: the pose of camera in global coordinates.
		#[inline]
		fn integrate(&mut self, depth: &impl ToInputArray, pose: &impl ToInputArray) -> Result<()> {
			input_array_arg!(depth);
			input_array_arg!(pose);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Volume_integrate_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Volume(), depth.as_raw__InputArray(), pose.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Integrates the input data to the volume.
		///
		/// Camera intrinsics are taken from volume settings structure.
		///
		/// ## Parameters
		/// * depth: the depth image.
		/// * image: the color image (only for ColorTSDF).
		///   For color TSDF a depth data should be registered with color data, i.e. have the same intrinsics & camera pose.
		///   This can be done using function registerDepth() from 3d module.
		/// * pose: the pose of camera in global coordinates.
		#[inline]
		fn integrate_color(&mut self, depth: &impl ToInputArray, image: &impl ToInputArray, pose: &impl ToInputArray) -> Result<()> {
			input_array_arg!(depth);
			input_array_arg!(image);
			input_array_arg!(pose);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Volume_integrate_const__InputArrayR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_Volume(), depth.as_raw__InputArray(), image.as_raw__InputArray(), pose.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// clear all data in volume.
		#[inline]
		fn reset(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Volume_reset(self.as_raw_mut_Volume(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Enables or disables new volume unit allocation during integration.
		/// Makes sense for HashTSDF only.
		#[inline]
		fn set_enable_growth(&mut self, v: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_Volume_setEnableGrowth_bool(self.as_raw_mut_Volume(), v, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for Volume {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Volume")
				.finish()
		}
	}

	impl crate::ptcloud::VolumeTraitConst for Volume {
		#[inline] fn as_raw_Volume(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::ptcloud::VolumeTrait for Volume {
		#[inline] fn as_raw_mut_Volume(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { Volume, crate::ptcloud::VolumeTraitConst, as_raw_Volume, crate::ptcloud::VolumeTrait, as_raw_mut_Volume }

	pub struct VolumeSettings {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { VolumeSettings }

	impl Drop for VolumeSettings {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_VolumeSettings_delete(self.as_raw_mut_VolumeSettings()) };
		}
	}

	unsafe impl Send for VolumeSettings {}

	impl VolumeSettings {
		/// Constructor of settings for custom Volume type.
		/// ## Parameters
		/// * volumeType: volume type.
		///
		/// ## C++ default parameters
		/// * volume_type: VolumeType::TSDF
		#[inline]
		pub fn new(volume_type: crate::ptcloud::VolumeType) -> Result<crate::ptcloud::VolumeSettings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_VolumeSettings_VolumeType(volume_type, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::ptcloud::VolumeSettings::opencv_from_extern(ret) };
			Ok(ret)
		}

		/// Constructor of settings for custom Volume type.
		/// ## Parameters
		/// * volumeType: volume type.
		///
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * volume_type: VolumeType::TSDF
		#[inline]
		pub fn new_def() -> Result<crate::ptcloud::VolumeSettings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_VolumeSettings(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::ptcloud::VolumeSettings::opencv_from_extern(ret) };
			Ok(ret)
		}

		#[inline]
		pub fn copy(vs: &impl crate::ptcloud::VolumeSettingsTraitConst) -> Result<crate::ptcloud::VolumeSettings> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_VolumeSettings_const_VolumeSettingsR(vs.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::ptcloud::VolumeSettings::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::ptcloud::VolumeSettings]
	pub trait VolumeSettingsTraitConst {
		fn as_raw_VolumeSettings(&self) -> *const c_void;

		/// Returns the width of the image for integration.
		#[inline]
		fn get_integrate_width(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_getIntegrateWidth_const(self.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns the height of the image for integration.
		#[inline]
		fn get_integrate_height(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_getIntegrateHeight_const(self.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns the width of the raycasted image, used when user does not provide it at raycast() call.
		#[inline]
		fn get_raycast_width(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_getRaycastWidth_const(self.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns the height of the raycasted image, used when user does not provide it at raycast() call.
		#[inline]
		fn get_raycast_height(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_getRaycastHeight_const(self.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns depth factor, witch is the number for depth scaling.
		#[inline]
		fn get_depth_factor(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_getDepthFactor_const(self.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns the size of voxel.
		#[inline]
		fn get_voxel_size(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_getVoxelSize_const(self.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns TSDF truncation distance. Distances greater than value from surface will be truncated to 1.0.
		#[inline]
		fn get_tsdf_truncate_distance(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_getTsdfTruncateDistance_const(self.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns threshold for depth truncation in meters. Truncates the depth greater than threshold to 0.
		#[inline]
		fn get_max_depth(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_getMaxDepth_const(self.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns max number of frames to integrate per voxel.
		/// Represents the max number of frames over which a running average of the TSDF is calculated for a voxel.
		#[inline]
		fn get_max_weight(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_getMaxWeight_const(self.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns length of single raycast step.
		/// Describes the percentage of voxel length that is skipped per march.
		#[inline]
		fn get_raycast_step_factor(&self) -> Result<f32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_getRaycastStepFactor_const(self.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Sets volume pose.
		/// ## Parameters
		/// * val: output value.
		#[inline]
		fn get_volume_pose(&self, val: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(val);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_getVolumePose_const_const__OutputArrayR(self.as_raw_VolumeSettings(), val.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Resolution of voxel space.
		///    Number of voxels in each dimension.
		///    Applicable only for TSDF Volume.
		///    HashTSDF volume only supports equal resolution in all three dimensions.
		/// ## Parameters
		/// * val: output value.
		#[inline]
		fn get_volume_resolution(&self, val: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(val);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_getVolumeResolution_const_const__OutputArrayR(self.as_raw_VolumeSettings(), val.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns 3 integers representing strides by x, y and z dimension.
		///    Can be used to iterate over raw volume unit data.
		/// ## Parameters
		/// * val: output value.
		#[inline]
		fn get_volume_strides(&self, val: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(val);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_getVolumeStrides_const_const__OutputArrayR(self.as_raw_VolumeSettings(), val.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns intrinsics of camera for integrations.
		/// Format of output:
		/// [ fx  0 cx ]
		/// [  0 fy cy ]
		/// [  0  0  1 ]
		/// where fx and fy are focus points of Ox and Oy axises, and cx and cy are central points of Ox and Oy axises.
		/// ## Parameters
		/// * val: output value.
		#[inline]
		fn get_camera_integrate_intrinsics(&self, val: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(val);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_getCameraIntegrateIntrinsics_const_const__OutputArrayR(self.as_raw_VolumeSettings(), val.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Returns camera intrinsics for raycast image, used when user does not provide them at raycast() call.
		/// Format of output:
		/// [ fx  0 cx ]
		/// [  0 fy cy ]
		/// [  0  0  1 ]
		/// where fx and fy are focus points of Ox and Oy axises, and cx and cy are central points of Ox and Oy axises.
		/// ## Parameters
		/// * val: output value.
		#[inline]
		fn get_camera_raycast_intrinsics(&self, val: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(val);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_getCameraRaycastIntrinsics_const_const__OutputArrayR(self.as_raw_VolumeSettings(), val.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::ptcloud::VolumeSettings]
	pub trait VolumeSettingsTrait: crate::ptcloud::VolumeSettingsTraitConst {
		fn as_raw_mut_VolumeSettings(&mut self) -> *mut c_void;

		#[inline]
		fn set(&mut self, unnamed: &impl crate::ptcloud::VolumeSettingsTraitConst) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_operatorST_const_VolumeSettingsR(self.as_raw_mut_VolumeSettings(), unnamed.as_raw_VolumeSettings(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Sets the width of the image for integration.
		/// ## Parameters
		/// * val: input value.
		#[inline]
		fn set_integrate_width(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_setIntegrateWidth_int(self.as_raw_mut_VolumeSettings(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Sets the height of the image for integration.
		/// ## Parameters
		/// * val: input value.
		#[inline]
		fn set_integrate_height(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_setIntegrateHeight_int(self.as_raw_mut_VolumeSettings(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Sets the width of the raycasted image, used when user does not provide it at raycast() call.
		/// ## Parameters
		/// * val: input value.
		#[inline]
		fn set_raycast_width(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_setRaycastWidth_int(self.as_raw_mut_VolumeSettings(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Sets the height of the raycasted image, used when user does not provide it at raycast() call.
		/// ## Parameters
		/// * val: input value.
		#[inline]
		fn set_raycast_height(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_setRaycastHeight_int(self.as_raw_mut_VolumeSettings(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Sets depth factor, witch is the number for depth scaling.
		/// ## Parameters
		/// * val: input value.
		#[inline]
		fn set_depth_factor(&mut self, val: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_setDepthFactor_float(self.as_raw_mut_VolumeSettings(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Sets the size of voxel.
		/// ## Parameters
		/// * val: input value.
		#[inline]
		fn set_voxel_size(&mut self, val: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_setVoxelSize_float(self.as_raw_mut_VolumeSettings(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Sets TSDF truncation distance. Distances greater than value from surface will be truncated to 1.0.
		/// ## Parameters
		/// * val: input value.
		#[inline]
		fn set_tsdf_truncate_distance(&mut self, val: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_setTsdfTruncateDistance_float(self.as_raw_mut_VolumeSettings(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Sets threshold for depth truncation in meters. Truncates the depth greater than threshold to 0.
		/// ## Parameters
		/// * val: input value.
		#[inline]
		fn set_max_depth(&mut self, val: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_setMaxDepth_float(self.as_raw_mut_VolumeSettings(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Sets max number of frames to integrate per voxel.
		///    Represents the max number of frames over which a running average of the TSDF is calculated for a voxel.
		/// ## Parameters
		/// * val: input value.
		#[inline]
		fn set_max_weight(&mut self, val: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_setMaxWeight_int(self.as_raw_mut_VolumeSettings(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Sets length of single raycast step.
		///    Describes the percentage of voxel length that is skipped per march.
		/// ## Parameters
		/// * val: input value.
		#[inline]
		fn set_raycast_step_factor(&mut self, val: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_setRaycastStepFactor_float(self.as_raw_mut_VolumeSettings(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Sets volume pose.
		/// ## Parameters
		/// * val: input value.
		#[inline]
		fn set_volume_pose(&mut self, val: &impl ToInputArray) -> Result<()> {
			input_array_arg!(val);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_setVolumePose_const__InputArrayR(self.as_raw_mut_VolumeSettings(), val.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Resolution of voxel space.
		///    Number of voxels in each dimension.
		///    Applicable only for TSDF Volume.
		///    HashTSDF volume only supports equal resolution in all three dimensions.
		/// ## Parameters
		/// * val: input value.
		#[inline]
		fn set_volume_resolution(&mut self, val: &impl ToInputArray) -> Result<()> {
			input_array_arg!(val);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_setVolumeResolution_const__InputArrayR(self.as_raw_mut_VolumeSettings(), val.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Sets intrinsics of camera for integrations.
		/// Format of input:
		/// [ fx  0 cx ]
		/// [  0 fy cy ]
		/// [  0  0  1 ]
		/// where fx and fy are focus points of Ox and Oy axises, and cx and cy are central points of Ox and Oy axises.
		/// ## Parameters
		/// * val: input value.
		#[inline]
		fn set_camera_integrate_intrinsics(&mut self, val: &impl ToInputArray) -> Result<()> {
			input_array_arg!(val);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_setCameraIntegrateIntrinsics_const__InputArrayR(self.as_raw_mut_VolumeSettings(), val.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// Sets camera intrinsics for raycast image which, used when user does not provide them at raycast() call.
		/// Format of input:
		/// [ fx  0 cx ]
		/// [  0 fy cy ]
		/// [  0  0  1 ]
		/// where fx and fy are focus points of Ox and Oy axises, and cx and cy are central points of Ox and Oy axises.
		/// ## Parameters
		/// * val: input value.
		#[inline]
		fn set_camera_raycast_intrinsics(&mut self, val: &impl ToInputArray) -> Result<()> {
			input_array_arg!(val);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_VolumeSettings_setCameraRaycastIntrinsics_const__InputArrayR(self.as_raw_mut_VolumeSettings(), val.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl Clone for VolumeSettings {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_VolumeSettings_implicitClone_const(self.as_raw_VolumeSettings())) }
		}
	}

	impl std::fmt::Debug for VolumeSettings {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("VolumeSettings")
				.finish()
		}
	}

	impl crate::ptcloud::VolumeSettingsTraitConst for VolumeSettings {
		#[inline] fn as_raw_VolumeSettings(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::ptcloud::VolumeSettingsTrait for VolumeSettings {
		#[inline] fn as_raw_mut_VolumeSettings(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { VolumeSettings, crate::ptcloud::VolumeSettingsTraitConst, as_raw_VolumeSettings, crate::ptcloud::VolumeSettingsTrait, as_raw_mut_VolumeSettings }

}
