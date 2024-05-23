//! # OGRE 3D Visualiser
//!
//! ovis is a simplified rendering wrapper around [ogre3d](https://www.ogre3d.org/).
//! The [Ogre terminology](https://ogrecave.github.io/ogre/api/latest/_the-_core-_objects.html) is used in the API
//! and [Ogre Script](https://ogrecave.github.io/ogre/api/latest/_scripts.html) is assumed to be used for advanced customization.
//!
//! Besides the API you see here, there are several environment variables that control the behavior of ovis.
//! They are documented in [createWindow].
//!
//! ## Loading geometry
//!
//! You can create geometry [on the fly]([createTriangleMesh]) or by loading Ogre `.mesh` files.
//!
//! ### Blender
//! For converting/ creating geometry [Blender](https://www.blender.org/) is recommended.
//! - Blender 2.7x is better tested, but Blender 2.8x should work too
//! - install [blender2ogre](https://github.com/OGRECave/blender2ogre) matching your Blender version
//! - download the [Ogre MSVC SDK](https://www.ogre3d.org/download/sdk/sdk-ogre) which contains `OgreXMLConverter.exe` (in `bin/`) and set the path in the blender2ogre settings
//! - get [ogre-meshviewer](https://github.com/OGRECave/ogre-meshviewer) to enable the preview function in blender2ogre as well as for verifying the exported files
//! - in case the exported materials are not exactly how you want them, consult the [Ogre Manual](https://ogrecave.github.io/ogre/api/latest/_material-_scripts.html)
//!
//! ### Assimp
//! When using Ogre 1.12.9 or later, enabling the Assimp plugin allows to load arbitrary geometry.
//! Simply pass `bunny.obj` instead of `bunny.mesh` as `meshname` in [WindowScene::createEntity].
//!
//! You should still use ogre-meshviewer to verify that the geometry is converted correctly.
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{WindowSceneTrait, WindowSceneTraitConst};
}

// ENTITY_AABB_WORLD /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:78
pub const ENTITY_AABB_WORLD: i32 = 2;
// ENTITY_ANIMBLEND_MODE /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:79
pub const ENTITY_ANIMBLEND_MODE: i32 = 3;
// ENTITY_CAST_SHADOWS /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:80
pub const ENTITY_CAST_SHADOWS: i32 = 4;
// ENTITY_MATERIAL /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:76
pub const ENTITY_MATERIAL: i32 = 0;
// ENTITY_SCALE /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:77
pub const ENTITY_SCALE: i32 = 1;
// MATERIAL_DIFFUSE /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:66
pub const MATERIAL_DIFFUSE: i32 = 4;
// MATERIAL_EMISSIVE /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:65
pub const MATERIAL_EMISSIVE: i32 = 3;
// MATERIAL_LINE_WIDTH /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:63
pub const MATERIAL_LINE_WIDTH: i32 = 1;
// MATERIAL_OPACITY /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:64
pub const MATERIAL_OPACITY: i32 = 2;
// MATERIAL_POINT_SIZE /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:62
pub const MATERIAL_POINT_SIZE: i32 = 0;
// MATERIAL_TEXTURE /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:68
pub const MATERIAL_TEXTURE: i32 = 5;
// MATERIAL_TEXTURE0 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:67
pub const MATERIAL_TEXTURE0: i32 = 5;
// MATERIAL_TEXTURE1 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:69
pub const MATERIAL_TEXTURE1: i32 = 6;
// MATERIAL_TEXTURE2 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:70
pub const MATERIAL_TEXTURE2: i32 = 7;
// MATERIAL_TEXTURE3 /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:71
pub const MATERIAL_TEXTURE3: i32 = 8;
/// Apply anti-aliasing. The first window determines the setting for all windows.
// SCENE_AA /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:53
pub const SCENE_AA: i32 = 8;
/// allow the user to control the camera.
// SCENE_INTERACTIVE /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:49
pub const SCENE_INTERACTIVE: i32 = 2;
/// Render off-screen without a window. Allows separate AA setting. Requires manual update via [WindowScene::update]
// SCENE_OFFSCREEN /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:55
pub const SCENE_OFFSCREEN: i32 = 16;
/// the window will use a separate scene. The scene will be shared otherwise.
// SCENE_SEPARATE /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:47
pub const SCENE_SEPARATE: i32 = 1;
/// Enable real-time shadows in the scene. All entities cast shadows by default. Control via [ENTITY_CAST_SHADOWS]
// SCENE_SHADOWS /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:57
pub const SCENE_SHADOWS: i32 = 32;
/// draw coordinate system crosses for debugging
// SCENE_SHOW_CS_CROSS /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:51
pub const SCENE_SHOW_CS_CROSS: i32 = 4;
// EntityProperty /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:74
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EntityProperty {
	ENTITY_MATERIAL = 0,
	ENTITY_SCALE = 1,
	ENTITY_AABB_WORLD = 2,
	ENTITY_ANIMBLEND_MODE = 3,
	ENTITY_CAST_SHADOWS = 4,
}

impl TryFrom<i32> for EntityProperty {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::ENTITY_MATERIAL),
			1 => Ok(Self::ENTITY_SCALE),
			2 => Ok(Self::ENTITY_AABB_WORLD),
			3 => Ok(Self::ENTITY_ANIMBLEND_MODE),
			4 => Ok(Self::ENTITY_CAST_SHADOWS),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::ovis::EntityProperty"))),
		}
	}
}

opencv_type_enum! { crate::ovis::EntityProperty }

// MaterialProperty /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:60
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MaterialProperty {
	MATERIAL_POINT_SIZE = 0,
	MATERIAL_LINE_WIDTH = 1,
	MATERIAL_OPACITY = 2,
	MATERIAL_EMISSIVE = 3,
	MATERIAL_DIFFUSE = 4,
	MATERIAL_TEXTURE0 = 5,
	// Duplicate, use MATERIAL_TEXTURE0 instead
	// MATERIAL_TEXTURE = 5,
	MATERIAL_TEXTURE1 = 6,
	MATERIAL_TEXTURE2 = 7,
	MATERIAL_TEXTURE3 = 8,
}

impl TryFrom<i32> for MaterialProperty {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::MATERIAL_POINT_SIZE),
			1 => Ok(Self::MATERIAL_LINE_WIDTH),
			2 => Ok(Self::MATERIAL_OPACITY),
			3 => Ok(Self::MATERIAL_EMISSIVE),
			4 => Ok(Self::MATERIAL_DIFFUSE),
			5 => Ok(Self::MATERIAL_TEXTURE0),
			// Duplicate of MATERIAL_TEXTURE0
			// 5 => Ok(Self::MATERIAL_TEXTURE),
			6 => Ok(Self::MATERIAL_TEXTURE1),
			7 => Ok(Self::MATERIAL_TEXTURE2),
			8 => Ok(Self::MATERIAL_TEXTURE3),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::ovis::MaterialProperty"))),
		}
	}
}

opencv_type_enum! { crate::ovis::MaterialProperty }

// SceneSettings /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:44
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SceneSettings {
	/// the window will use a separate scene. The scene will be shared otherwise.
	SCENE_SEPARATE = 1,
	/// allow the user to control the camera.
	SCENE_INTERACTIVE = 2,
	/// draw coordinate system crosses for debugging
	SCENE_SHOW_CS_CROSS = 4,
	/// Apply anti-aliasing. The first window determines the setting for all windows.
	SCENE_AA = 8,
	/// Render off-screen without a window. Allows separate AA setting. Requires manual update via [WindowScene::update]
	SCENE_OFFSCREEN = 16,
	/// Enable real-time shadows in the scene. All entities cast shadows by default. Control via [ENTITY_CAST_SHADOWS]
	SCENE_SHADOWS = 32,
}

impl TryFrom<i32> for SceneSettings {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			1 => Ok(Self::SCENE_SEPARATE),
			2 => Ok(Self::SCENE_INTERACTIVE),
			4 => Ok(Self::SCENE_SHOW_CS_CROSS),
			8 => Ok(Self::SCENE_AA),
			16 => Ok(Self::SCENE_OFFSCREEN),
			32 => Ok(Self::SCENE_SHADOWS),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::ovis::SceneSettings"))),
		}
	}
}

opencv_type_enum! { crate::ovis::SceneSettings }

/// Add an additional resource location that is search for meshes, textures and materials
///
/// must be called before the first createWindow. If give path does not exist, retries inside
/// Ogre Media Directory.
/// ## Parameters
/// * path: folder or Zip archive.
// addResourceLocation(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:324
// ("cv::ovis::addResourceLocation", vec![(pred!(mut, ["path"], ["const cv::String*"]), _)]),
#[inline]
pub fn add_resource_location(path: &str) -> Result<()> {
	extern_container_arg!(path);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ovis_addResourceLocation_const_StringR(path.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// creates a grid
///
/// creates a material with the same name
/// ## Parameters
/// * name: name of the mesh
/// * size: extents of the grid
/// * segments: number of segments per side
///
/// ## Note
/// This alternative version of [create_grid_mesh] function uses the following default values for its arguments:
/// * segments: Size(1,1)
// cv::ovis::createGridMesh(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:404
// ("cv::ovis::createGridMesh", vec![(pred!(mut, ["name", "size"], ["const cv::String*", "const cv::Size2f*"]), _)]),
#[inline]
pub fn create_grid_mesh_def(name: &str, size: core::Size2f) -> Result<()> {
	extern_container_arg!(name);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ovis_createGridMesh_const_StringR_const_Size2fR(name.opencv_as_extern(), &size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// creates a grid
///
/// creates a material with the same name
/// ## Parameters
/// * name: name of the mesh
/// * size: extents of the grid
/// * segments: number of segments per side
///
/// ## C++ default parameters
/// * segments: Size(1,1)
// createGridMesh(const String &, const Size2f &, const Size &)(InString, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:404
// ("cv::ovis::createGridMesh", vec![(pred!(mut, ["name", "size", "segments"], ["const cv::String*", "const cv::Size2f*", "const cv::Size*"]), _)]),
#[inline]
pub fn create_grid_mesh(name: &str, size: core::Size2f, segments: core::Size) -> Result<()> {
	extern_container_arg!(name);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ovis_createGridMesh_const_StringR_const_Size2fR_const_SizeR(name.opencv_as_extern(), &size, &segments, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// create a 2D plane, X right, Y down, Z up
///
/// creates a material with the same name
/// ## Parameters
/// * name: name of the mesh
/// * size: size in world units
/// * image: optional texture
///
/// ## Note
/// This alternative version of [create_plane_mesh] function uses the following default values for its arguments:
/// * image: noArray()
// cv::ovis::createPlaneMesh(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:384
// ("cv::ovis::createPlaneMesh", vec![(pred!(mut, ["name", "size"], ["const cv::String*", "const cv::Size2f*"]), _)]),
#[inline]
pub fn create_plane_mesh_def(name: &str, size: core::Size2f) -> Result<()> {
	extern_container_arg!(name);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ovis_createPlaneMesh_const_StringR_const_Size2fR(name.opencv_as_extern(), &size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// create a 2D plane, X right, Y down, Z up
///
/// creates a material with the same name
/// ## Parameters
/// * name: name of the mesh
/// * size: size in world units
/// * image: optional texture
///
/// ## C++ default parameters
/// * image: noArray()
// createPlaneMesh(const String &, const Size2f &, InputArray)(InString, SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:384
// ("cv::ovis::createPlaneMesh", vec![(pred!(mut, ["name", "size", "image"], ["const cv::String*", "const cv::Size2f*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn create_plane_mesh(name: &str, size: core::Size2f, image: &impl ToInputArray) -> Result<()> {
	extern_container_arg!(name);
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ovis_createPlaneMesh_const_StringR_const_Size2fR_const__InputArrayR(name.opencv_as_extern(), &size, image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// creates a point cloud mesh
///
/// creates a material with the same name
/// ## Parameters
/// * name: name of the mesh
/// * vertices: float vector of positions
/// * colors: uchar vector of colors
///
/// ## Note
/// This alternative version of [create_point_cloud_mesh] function uses the following default values for its arguments:
/// * colors: noArray()
// cv::ovis::createPointCloudMesh(InString, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:394
// ("cv::ovis::createPointCloudMesh", vec![(pred!(mut, ["name", "vertices"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn create_point_cloud_mesh_def(name: &str, vertices: &impl ToInputArray) -> Result<()> {
	extern_container_arg!(name);
	input_array_arg!(vertices);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ovis_createPointCloudMesh_const_StringR_const__InputArrayR(name.opencv_as_extern(), vertices.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// creates a point cloud mesh
///
/// creates a material with the same name
/// ## Parameters
/// * name: name of the mesh
/// * vertices: float vector of positions
/// * colors: uchar vector of colors
///
/// ## C++ default parameters
/// * colors: noArray()
// createPointCloudMesh(const String &, InputArray, InputArray)(InString, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:394
// ("cv::ovis::createPointCloudMesh", vec![(pred!(mut, ["name", "vertices", "colors"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn create_point_cloud_mesh(name: &str, vertices: &impl ToInputArray, colors: &impl ToInputArray) -> Result<()> {
	extern_container_arg!(name);
	input_array_arg!(vertices);
	input_array_arg!(colors);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ovis_createPointCloudMesh_const_StringR_const__InputArrayR_const__InputArrayR(name.opencv_as_extern(), vertices.as_raw__InputArray(), colors.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// creates a triangle mesh from vertex-vertex or face-vertex representation
///
/// creates a material with the same name
/// ## Parameters
/// * name: name of the mesh
/// * vertices: float vector of positions
/// * normals: float vector of normals
/// * indices: int vector of indices
///
/// ## Note
/// This alternative version of [create_triangle_mesh] function uses the following default values for its arguments:
/// * normals: noArray()
/// * indices: noArray()
// cv::ovis::createTriangleMesh(InString, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:415
// ("cv::ovis::createTriangleMesh", vec![(pred!(mut, ["name", "vertices"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn create_triangle_mesh_def(name: &str, vertices: &impl ToInputArray) -> Result<()> {
	extern_container_arg!(name);
	input_array_arg!(vertices);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ovis_createTriangleMesh_const_StringR_const__InputArrayR(name.opencv_as_extern(), vertices.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// creates a triangle mesh from vertex-vertex or face-vertex representation
///
/// creates a material with the same name
/// ## Parameters
/// * name: name of the mesh
/// * vertices: float vector of positions
/// * normals: float vector of normals
/// * indices: int vector of indices
///
/// ## C++ default parameters
/// * normals: noArray()
/// * indices: noArray()
// createTriangleMesh(const String &, InputArray, InputArray, InputArray)(InString, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:415
// ("cv::ovis::createTriangleMesh", vec![(pred!(mut, ["name", "vertices", "normals", "indices"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn create_triangle_mesh(name: &str, vertices: &impl ToInputArray, normals: &impl ToInputArray, indices: &impl ToInputArray) -> Result<()> {
	extern_container_arg!(name);
	input_array_arg!(vertices);
	input_array_arg!(normals);
	input_array_arg!(indices);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ovis_createTriangleMesh_const_StringR_const__InputArrayR_const__InputArrayR_const__InputArrayR(name.opencv_as_extern(), vertices.as_raw__InputArray(), normals.as_raw__InputArray(), indices.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// create a new rendering window/ viewport
/// ## Parameters
/// * title: window title
/// * size: size of the window
/// * flags: a combination of [SceneSettings]
///
/// Furthermore, the behavior is controlled by the following environment variables
/// - OPENCV_OVIS_VERBOSE_LOG: print all of OGRE log output
/// - OPENCV_OVIS_RENDERSYSTEM: the name of the OGRE RenderSystem to use
/// - OPENCV_OVIS_NOVSYNC: disable VSYNC for all windows
///
/// ## Note
/// This alternative version of [create_window] function uses the following default values for its arguments:
/// * flags: SCENE_INTERACTIVE|SCENE_AA
// cv::ovis::createWindow(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:337
// ("cv::ovis::createWindow", vec![(pred!(mut, ["title", "size"], ["const cv::String*", "const cv::Size*"]), _)]),
#[inline]
pub fn create_window_def(title: &str, size: core::Size) -> Result<core::Ptr<crate::ovis::WindowScene>> {
	extern_container_arg!(title);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ovis_createWindow_const_StringR_const_SizeR(title.opencv_as_extern(), &size, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::ovis::WindowScene>::opencv_from_extern(ret) };
	Ok(ret)
}

/// create a new rendering window/ viewport
/// ## Parameters
/// * title: window title
/// * size: size of the window
/// * flags: a combination of [SceneSettings]
///
/// Furthermore, the behavior is controlled by the following environment variables
/// - OPENCV_OVIS_VERBOSE_LOG: print all of OGRE log output
/// - OPENCV_OVIS_RENDERSYSTEM: the name of the OGRE RenderSystem to use
/// - OPENCV_OVIS_NOVSYNC: disable VSYNC for all windows
///
/// ## C++ default parameters
/// * flags: SCENE_INTERACTIVE|SCENE_AA
// createWindow(const String &, const Size &, int)(InString, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:337
// ("cv::ovis::createWindow", vec![(pred!(mut, ["title", "size", "flags"], ["const cv::String*", "const cv::Size*", "int"]), _)]),
#[inline]
pub fn create_window(title: &str, size: core::Size, flags: i32) -> Result<core::Ptr<crate::ovis::WindowScene>> {
	extern_container_arg!(title);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ovis_createWindow_const_StringR_const_SizeR_int(title.opencv_as_extern(), &size, flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<crate::ovis::WindowScene>::opencv_from_extern(ret) };
	Ok(ret)
}

/// set the shader property of a material to the given value
/// ## Parameters
/// * name: material name
/// * prop: property name
/// * value: the value
// setMaterialProperty(const String &, const String &, const Scalar &)(InString, InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:374
// ("cv::ovis::setMaterialProperty", vec![(pred!(mut, ["name", "prop", "value"], ["const cv::String*", "const cv::String*", "const cv::Scalar*"]), _)]),
#[inline]
pub fn set_material_property_2(name: &str, prop: &str, value: core::Scalar) -> Result<()> {
	extern_container_arg!(name);
	extern_container_arg!(prop);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ovis_setMaterialProperty_const_StringR_const_StringR_const_ScalarR(name.opencv_as_extern(), prop.opencv_as_extern(), &value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// set the property of a material to the given value
/// ## Parameters
/// * name: material name
/// * prop: [MaterialProperty]
/// * value: the value
// setMaterialProperty(const String &, int, const Scalar &)(InString, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:355
// ("cv::ovis::setMaterialProperty", vec![(pred!(mut, ["name", "prop", "value"], ["const cv::String*", "int", "const cv::Scalar*"]), _)]),
#[inline]
pub fn set_material_property(name: &str, prop: i32, value: core::Scalar) -> Result<()> {
	extern_container_arg!(name);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ovis_setMaterialProperty_const_StringR_int_const_ScalarR(name.opencv_as_extern(), prop, &value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// set the property of a material to the given value
/// ## Parameters
/// * name: material name
/// * prop: [MaterialProperty]
/// * value: the value
///
/// ## Overloaded parameters
// setMaterialProperty(const String &, int, const String &)(InString, Primitive, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:358
// ("cv::ovis::setMaterialProperty", vec![(pred!(mut, ["name", "prop", "value"], ["const cv::String*", "int", "const cv::String*"]), _)]),
#[inline]
pub fn set_material_property_1(name: &str, prop: i32, value: &str) -> Result<()> {
	extern_container_arg!(name);
	extern_container_arg!(value);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ovis_setMaterialProperty_const_StringR_int_const_StringR(name.opencv_as_extern(), prop, value.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// set the texture of a material to the given value
/// ## Parameters
/// * name: material name
/// * prop: [MaterialProperty]
/// * value: the texture data
// setMaterialProperty(const String &, int, InputArray)(InString, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:366
// ("cv::ovis::setMaterialProperty", vec![(pred!(mut, ["name", "prop", "value"], ["const cv::String*", "int", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn set_material_texture(name: &str, prop: i32, value: &impl ToInputArray) -> Result<()> {
	extern_container_arg!(name);
	input_array_arg!(value);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ovis_setMaterialProperty_const_StringR_int_const__InputArrayR(name.opencv_as_extern(), prop, value.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

///
/// **Deprecated**: use setMaterialProperty
#[deprecated = "use setMaterialProperty"]
// updateTexture(const String &, InputArray)(InString, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:418
// ("cv::ovis::updateTexture", vec![(pred!(mut, ["name", "image"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
#[inline]
pub fn update_texture(name: &str, image: &impl ToInputArray) -> Result<()> {
	extern_container_arg!(name);
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ovis_updateTexture_const_StringR_const__InputArrayR(name.opencv_as_extern(), image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// update all windows and wait for keyboard event
///
/// ## Parameters
/// * delay: 0 is the special value that means "forever".
///        Any positive number returns after sync to blank (typically 16ms).
/// ## Returns
/// the code of the pressed key or -1 if no key was pressed
///
/// ## Note
/// This alternative version of [wait_key] function uses the following default values for its arguments:
/// * delay: 0
// cv::ovis::waitKey() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:347
// ("cv::ovis::waitKey", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn wait_key_def() -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ovis_waitKey(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// update all windows and wait for keyboard event
///
/// ## Parameters
/// * delay: 0 is the special value that means "forever".
///        Any positive number returns after sync to blank (typically 16ms).
/// ## Returns
/// the code of the pressed key or -1 if no key was pressed
///
/// ## C++ default parameters
/// * delay: 0
// waitKey(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:347
// ("cv::ovis::waitKey", vec![(pred!(mut, ["delay"], ["int"]), _)]),
#[inline]
pub fn wait_key(delay: i32) -> Result<i32> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_ovis_waitKey_int(delay, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Constant methods for [crate::ovis::WindowScene]
// WindowScene /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:86
pub trait WindowSceneTraitConst {
	fn as_raw_WindowScene(&self) -> *const c_void;

}

/// Mutable methods for [crate::ovis::WindowScene]
pub trait WindowSceneTrait: crate::ovis::WindowSceneTraitConst {
	fn as_raw_mut_WindowScene(&mut self) -> *mut c_void;

	/// set window background to custom image/ color
	/// ## Parameters
	/// * image: 
	// setBackground(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:94
	// ("cv::ovis::WindowScene::setBackground", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn set_background(&mut self, image: &impl ToInputArray) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_setBackground_const__InputArrayR(self.as_raw_mut_WindowScene(), image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// set window background to custom image/ color
	/// ## Parameters
	/// * image: 
	///
	/// ## Overloaded parameters
	// setBackground(const Scalar &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:97
	// ("cv::ovis::WindowScene::setBackground", vec![(pred!(mut, ["color"], ["const cv::Scalar*"]), _)]),
	#[inline]
	fn set_background_color(&mut self, color: core::Scalar) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_setBackground_const_ScalarR(self.as_raw_mut_WindowScene(), &color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// enable an ordered chain of full-screen post processing effects
	///
	/// this way you can add distortion or SSAO effects.
	/// The effects themselves must be defined inside Ogre .compositor scripts.
	/// ## Parameters
	/// * names: compositor names that will be applied in order of appearance
	/// ## See also
	/// addResourceLocation
	// setCompositors(const std::vector<String> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:107
	// ("cv::ovis::WindowScene::setCompositors", vec![(pred!(mut, ["names"], ["const std::vector<cv::String>*"]), _)]),
	#[inline]
	fn set_compositors(&mut self, names: &core::Vector<String>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_setCompositors_const_vectorLStringGR(self.as_raw_mut_WindowScene(), names.as_raw_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// place an entity of a mesh in the scene
	///
	/// the mesh needs to be created beforehand. Either programmatically
	/// by e.g. [createPointCloudMesh] or by placing the respective file in a resource location.
	/// ## Parameters
	/// * name: entity name
	/// * meshname: mesh name
	/// * tvec: translation
	/// * rot: [Rodrigues] vector or 3x3 rotation matrix
	/// ## See also
	/// addResourceLocation
	///
	/// ## C++ default parameters
	/// * tvec: noArray()
	/// * rot: noArray()
	// createEntity(const String &, const String &, InputArray, InputArray)(InString, InString, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:120
	// ("cv::ovis::WindowScene::createEntity", vec![(pred!(mut, ["name", "meshname", "tvec", "rot"], ["const cv::String*", "const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn create_entity(&mut self, name: &str, meshname: &str, tvec: &impl ToInputArray, rot: &impl ToInputArray) -> Result<()> {
		extern_container_arg!(name);
		extern_container_arg!(meshname);
		input_array_arg!(tvec);
		input_array_arg!(rot);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_createEntity_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_WindowScene(), name.opencv_as_extern(), meshname.opencv_as_extern(), tvec.as_raw__InputArray(), rot.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// place an entity of a mesh in the scene
	///
	/// the mesh needs to be created beforehand. Either programmatically
	/// by e.g. [createPointCloudMesh] or by placing the respective file in a resource location.
	/// ## Parameters
	/// * name: entity name
	/// * meshname: mesh name
	/// * tvec: translation
	/// * rot: [Rodrigues] vector or 3x3 rotation matrix
	/// ## See also
	/// addResourceLocation
	///
	/// ## Note
	/// This alternative version of [WindowSceneTrait::create_entity] function uses the following default values for its arguments:
	/// * tvec: noArray()
	/// * rot: noArray()
	// cv::ovis::WindowScene::createEntity(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:120
	// ("cv::ovis::WindowScene::createEntity", vec![(pred!(mut, ["name", "meshname"], ["const cv::String*", "const cv::String*"]), _)]),
	#[inline]
	fn create_entity_def(&mut self, name: &str, meshname: &str) -> Result<()> {
		extern_container_arg!(name);
		extern_container_arg!(meshname);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_createEntity_const_StringR_const_StringR(self.as_raw_mut_WindowScene(), name.opencv_as_extern(), meshname.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// remove an entity from the scene
	/// ## Parameters
	/// * name: entity name
	// removeEntity(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:127
	// ("cv::ovis::WindowScene::removeEntity", vec![(pred!(mut, ["name"], ["const cv::String*"]), _)]),
	#[inline]
	fn remove_entity(&mut self, name: &str) -> Result<()> {
		extern_container_arg!(name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_removeEntity_const_StringR(self.as_raw_mut_WindowScene(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// set the property of an entity to the given value
	/// ## Parameters
	/// * name: entity name
	/// * prop: [EntityProperty]
	/// * value: the value
	/// * subEntityIdx: index of the sub-entity (default: all)
	///
	/// ## C++ default parameters
	/// * sub_entity_idx: -1
	// setEntityProperty(const String &, int, const String &, int)(InString, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:136
	// ("cv::ovis::WindowScene::setEntityProperty", vec![(pred!(mut, ["name", "prop", "value", "subEntityIdx"], ["const cv::String*", "int", "const cv::String*", "int"]), _)]),
	#[inline]
	fn set_entity_property(&mut self, name: &str, prop: i32, value: &str, sub_entity_idx: i32) -> Result<()> {
		extern_container_arg!(name);
		extern_container_arg!(value);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_setEntityProperty_const_StringR_int_const_StringR_int(self.as_raw_mut_WindowScene(), name.opencv_as_extern(), prop, value.opencv_as_extern(), sub_entity_idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// set the property of an entity to the given value
	/// ## Parameters
	/// * name: entity name
	/// * prop: [EntityProperty]
	/// * value: the value
	/// * subEntityIdx: index of the sub-entity (default: all)
	///
	/// ## Note
	/// This alternative version of [WindowSceneTrait::set_entity_property] function uses the following default values for its arguments:
	/// * sub_entity_idx: -1
	// cv::ovis::WindowScene::setEntityProperty(InString, Primitive, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:136
	// ("cv::ovis::WindowScene::setEntityProperty", vec![(pred!(mut, ["name", "prop", "value"], ["const cv::String*", "int", "const cv::String*"]), _)]),
	#[inline]
	fn set_entity_property_def(&mut self, name: &str, prop: i32, value: &str) -> Result<()> {
		extern_container_arg!(name);
		extern_container_arg!(value);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_setEntityProperty_const_StringR_int_const_StringR(self.as_raw_mut_WindowScene(), name.opencv_as_extern(), prop, value.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// set the property of an entity to the given value
	/// ## Parameters
	/// * name: entity name
	/// * prop: [EntityProperty]
	/// * value: the value
	/// * subEntityIdx: index of the sub-entity (default: all)
	///
	/// ## Overloaded parameters
	// setEntityProperty(const String &, int, const Scalar &)(InString, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:140
	// ("cv::ovis::WindowScene::setEntityProperty", vec![(pred!(mut, ["name", "prop", "value"], ["const cv::String*", "int", "const cv::Scalar*"]), _)]),
	#[inline]
	fn set_entity_property_1(&mut self, name: &str, prop: i32, value: core::Scalar) -> Result<()> {
		extern_container_arg!(name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_setEntityProperty_const_StringR_int_const_ScalarR(self.as_raw_mut_WindowScene(), name.opencv_as_extern(), prop, &value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// get the property of an entity
	/// ## Parameters
	/// * name: entity name
	/// * prop: [EntityProperty]
	/// * value: the value
	// getEntityProperty(const String &, int, OutputArray)(InString, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:148
	// ("cv::ovis::WindowScene::getEntityProperty", vec![(pred!(mut, ["name", "prop", "value"], ["const cv::String*", "int", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_entity_property(&mut self, name: &str, prop: i32, value: &mut impl ToOutputArray) -> Result<()> {
		extern_container_arg!(name);
		output_array_arg!(value);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_getEntityProperty_const_StringR_int_const__OutputArrayR(self.as_raw_mut_WindowScene(), name.opencv_as_extern(), prop, value.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// convenience method to visualize a camera position
	///
	/// ## Parameters
	/// * name: entity name
	/// * K: intrinsic matrix
	/// * imsize: image size
	/// * zFar: far plane in camera coordinates
	/// * tvec: translation
	/// * rot: [Rodrigues] vector or 3x3 rotation matrix
	/// * color: line color
	/// ## Returns
	/// the extents of the Frustum at far plane, where the top left corner denotes the principal
	/// point offset
	///
	/// ## C++ default parameters
	/// * tvec: noArray()
	/// * rot: noArray()
	/// * color: Scalar::all(1)
	// createCameraEntity(const String &, InputArray, const Size &, float, InputArray, InputArray, const Scalar &)(InString, InputArray, SimpleClass, Primitive, InputArray, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:163
	// ("cv::ovis::WindowScene::createCameraEntity", vec![(pred!(mut, ["name", "K", "imsize", "zFar", "tvec", "rot", "color"], ["const cv::String*", "const cv::_InputArray*", "const cv::Size*", "float", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Scalar*"]), _)]),
	#[inline]
	fn create_camera_entity(&mut self, name: &str, k: &impl ToInputArray, imsize: core::Size, z_far: f32, tvec: &impl ToInputArray, rot: &impl ToInputArray, color: core::Scalar) -> Result<core::Rect2d> {
		extern_container_arg!(name);
		input_array_arg!(k);
		input_array_arg!(tvec);
		input_array_arg!(rot);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_createCameraEntity_const_StringR_const__InputArrayR_const_SizeR_float_const__InputArrayR_const__InputArrayR_const_ScalarR(self.as_raw_mut_WindowScene(), name.opencv_as_extern(), k.as_raw__InputArray(), &imsize, z_far, tvec.as_raw__InputArray(), rot.as_raw__InputArray(), &color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// convenience method to visualize a camera position
	///
	/// ## Parameters
	/// * name: entity name
	/// * K: intrinsic matrix
	/// * imsize: image size
	/// * zFar: far plane in camera coordinates
	/// * tvec: translation
	/// * rot: [Rodrigues] vector or 3x3 rotation matrix
	/// * color: line color
	/// ## Returns
	/// the extents of the Frustum at far plane, where the top left corner denotes the principal
	/// point offset
	///
	/// ## Note
	/// This alternative version of [WindowSceneTrait::create_camera_entity] function uses the following default values for its arguments:
	/// * tvec: noArray()
	/// * rot: noArray()
	/// * color: Scalar::all(1)
	// cv::ovis::WindowScene::createCameraEntity(InString, InputArray, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:163
	// ("cv::ovis::WindowScene::createCameraEntity", vec![(pred!(mut, ["name", "K", "imsize", "zFar"], ["const cv::String*", "const cv::_InputArray*", "const cv::Size*", "float"]), _)]),
	#[inline]
	fn create_camera_entity_def(&mut self, name: &str, k: &impl ToInputArray, imsize: core::Size, z_far: f32) -> Result<core::Rect2d> {
		extern_container_arg!(name);
		input_array_arg!(k);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_createCameraEntity_const_StringR_const__InputArrayR_const_SizeR_float(self.as_raw_mut_WindowScene(), name.opencv_as_extern(), k.as_raw__InputArray(), &imsize, z_far, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// creates a point light in the scene
	/// ## Parameters
	/// * name: entity name
	/// * tvec: translation
	/// * rot: [Rodrigues] vector or 3x3 rotation matrix
	/// * diffuseColor: 
	/// * specularColor: 
	///
	/// ## C++ default parameters
	/// * tvec: noArray()
	/// * rot: noArray()
	/// * diffuse_color: Scalar::all(1)
	/// * specular_color: Scalar::all(1)
	// createLightEntity(const String &, InputArray, InputArray, const Scalar &, const Scalar &)(InString, InputArray, InputArray, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:176
	// ("cv::ovis::WindowScene::createLightEntity", vec![(pred!(mut, ["name", "tvec", "rot", "diffuseColor", "specularColor"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::Scalar*", "const cv::Scalar*"]), _)]),
	#[inline]
	fn create_light_entity(&mut self, name: &str, tvec: &impl ToInputArray, rot: &impl ToInputArray, diffuse_color: core::Scalar, specular_color: core::Scalar) -> Result<()> {
		extern_container_arg!(name);
		input_array_arg!(tvec);
		input_array_arg!(rot);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_createLightEntity_const_StringR_const__InputArrayR_const__InputArrayR_const_ScalarR_const_ScalarR(self.as_raw_mut_WindowScene(), name.opencv_as_extern(), tvec.as_raw__InputArray(), rot.as_raw__InputArray(), &diffuse_color, &specular_color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// creates a point light in the scene
	/// ## Parameters
	/// * name: entity name
	/// * tvec: translation
	/// * rot: [Rodrigues] vector or 3x3 rotation matrix
	/// * diffuseColor: 
	/// * specularColor: 
	///
	/// ## Note
	/// This alternative version of [WindowSceneTrait::create_light_entity] function uses the following default values for its arguments:
	/// * tvec: noArray()
	/// * rot: noArray()
	/// * diffuse_color: Scalar::all(1)
	/// * specular_color: Scalar::all(1)
	// cv::ovis::WindowScene::createLightEntity(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:176
	// ("cv::ovis::WindowScene::createLightEntity", vec![(pred!(mut, ["name"], ["const cv::String*"]), _)]),
	#[inline]
	fn create_light_entity_def(&mut self, name: &str) -> Result<()> {
		extern_container_arg!(name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_createLightEntity_const_StringR(self.as_raw_mut_WindowScene(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// update entity pose by transformation in the parent coordinate space. (pre-rotation)
	/// ## Parameters
	/// * name: entity name
	/// * tvec: translation
	/// * rot: [Rodrigues] vector or 3x3 rotation matrix
	///
	/// ## C++ default parameters
	/// * tvec: noArray()
	/// * rot: noArray()
	// updateEntityPose(const String &, InputArray, InputArray)(InString, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:187
	// ("cv::ovis::WindowScene::updateEntityPose", vec![(pred!(mut, ["name", "tvec", "rot"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn update_entity_pose(&mut self, name: &str, tvec: &impl ToInputArray, rot: &impl ToInputArray) -> Result<()> {
		extern_container_arg!(name);
		input_array_arg!(tvec);
		input_array_arg!(rot);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_updateEntityPose_const_StringR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_WindowScene(), name.opencv_as_extern(), tvec.as_raw__InputArray(), rot.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// update entity pose by transformation in the parent coordinate space. (pre-rotation)
	/// ## Parameters
	/// * name: entity name
	/// * tvec: translation
	/// * rot: [Rodrigues] vector or 3x3 rotation matrix
	///
	/// ## Note
	/// This alternative version of [WindowSceneTrait::update_entity_pose] function uses the following default values for its arguments:
	/// * tvec: noArray()
	/// * rot: noArray()
	// cv::ovis::WindowScene::updateEntityPose(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:187
	// ("cv::ovis::WindowScene::updateEntityPose", vec![(pred!(mut, ["name"], ["const cv::String*"]), _)]),
	#[inline]
	fn update_entity_pose_def(&mut self, name: &str) -> Result<()> {
		extern_container_arg!(name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_updateEntityPose_const_StringR(self.as_raw_mut_WindowScene(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// set entity pose in the world coordinate space.
	/// ## Parameters
	/// * name: enitity name
	/// * tvec: translation
	/// * rot: [Rodrigues] vector or 3x3 rotation matrix
	/// * invert: use the inverse of the given pose
	///
	/// ## C++ default parameters
	/// * tvec: noArray()
	/// * rot: noArray()
	/// * invert: false
	// setEntityPose(const String &, InputArray, InputArray, bool)(InString, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:197
	// ("cv::ovis::WindowScene::setEntityPose", vec![(pred!(mut, ["name", "tvec", "rot", "invert"], ["const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "bool"]), _)]),
	#[inline]
	fn set_entity_pose(&mut self, name: &str, tvec: &impl ToInputArray, rot: &impl ToInputArray, invert: bool) -> Result<()> {
		extern_container_arg!(name);
		input_array_arg!(tvec);
		input_array_arg!(rot);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_setEntityPose_const_StringR_const__InputArrayR_const__InputArrayR_bool(self.as_raw_mut_WindowScene(), name.opencv_as_extern(), tvec.as_raw__InputArray(), rot.as_raw__InputArray(), invert, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// set entity pose in the world coordinate space.
	/// ## Parameters
	/// * name: enitity name
	/// * tvec: translation
	/// * rot: [Rodrigues] vector or 3x3 rotation matrix
	/// * invert: use the inverse of the given pose
	///
	/// ## Note
	/// This alternative version of [WindowSceneTrait::set_entity_pose] function uses the following default values for its arguments:
	/// * tvec: noArray()
	/// * rot: noArray()
	/// * invert: false
	// cv::ovis::WindowScene::setEntityPose(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:197
	// ("cv::ovis::WindowScene::setEntityPose", vec![(pred!(mut, ["name"], ["const cv::String*"]), _)]),
	#[inline]
	fn set_entity_pose_def(&mut self, name: &str) -> Result<()> {
		extern_container_arg!(name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_setEntityPose_const_StringR(self.as_raw_mut_WindowScene(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Retrieves the current pose of an entity
	/// ## Parameters
	/// * name: entity name
	/// * R: 3x3 rotation matrix
	/// * tvec: translation vector
	/// * invert: return the inverted pose
	///
	/// ## C++ default parameters
	/// * r: noArray()
	/// * tvec: noArray()
	/// * invert: false
	// getEntityPose(const String &, OutputArray, OutputArray, bool)(InString, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:207
	// ("cv::ovis::WindowScene::getEntityPose", vec![(pred!(mut, ["name", "R", "tvec", "invert"], ["const cv::String*", "const cv::_OutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
	#[inline]
	fn get_entity_pose(&mut self, name: &str, r: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray, invert: bool) -> Result<()> {
		extern_container_arg!(name);
		output_array_arg!(r);
		output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_getEntityPose_const_StringR_const__OutputArrayR_const__OutputArrayR_bool(self.as_raw_mut_WindowScene(), name.opencv_as_extern(), r.as_raw__OutputArray(), tvec.as_raw__OutputArray(), invert, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Retrieves the current pose of an entity
	/// ## Parameters
	/// * name: entity name
	/// * R: 3x3 rotation matrix
	/// * tvec: translation vector
	/// * invert: return the inverted pose
	///
	/// ## Note
	/// This alternative version of [WindowSceneTrait::get_entity_pose] function uses the following default values for its arguments:
	/// * r: noArray()
	/// * tvec: noArray()
	/// * invert: false
	// cv::ovis::WindowScene::getEntityPose(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:207
	// ("cv::ovis::WindowScene::getEntityPose", vec![(pred!(mut, ["name"], ["const cv::String*"]), _)]),
	#[inline]
	fn get_entity_pose_def(&mut self, name: &str) -> Result<()> {
		extern_container_arg!(name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_getEntityPose_const_StringR(self.as_raw_mut_WindowScene(), name.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// get a list of available entity animations
	/// ## Parameters
	/// * name: entity name
	/// * out: the animation names
	// getEntityAnimations(const String &, std::vector<String> &)(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:215
	// ("cv::ovis::WindowScene::getEntityAnimations", vec![(pred!(mut, ["name", "out"], ["const cv::String*", "std::vector<cv::String>*"]), _)]),
	#[inline]
	fn get_entity_animations(&mut self, name: &str, out: &mut core::Vector<String>) -> Result<()> {
		extern_container_arg!(name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_getEntityAnimations_const_StringR_vectorLStringGR(self.as_raw_mut_WindowScene(), name.opencv_as_extern(), out.as_raw_mut_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// play entity animation
	/// ## Parameters
	/// * name: entity name
	/// * animname: animation name
	/// * loop: enable or disable animation loop
	/// ## See also
	/// getEntityAnimations
	///
	/// ## C++ default parameters
	/// * loop_: true
	// playEntityAnimation(const String &, const String &, bool)(InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:224
	// ("cv::ovis::WindowScene::playEntityAnimation", vec![(pred!(mut, ["name", "animname", "loop"], ["const cv::String*", "const cv::String*", "bool"]), _)]),
	#[inline]
	fn play_entity_animation(&mut self, name: &str, animname: &str, loop_: bool) -> Result<()> {
		extern_container_arg!(name);
		extern_container_arg!(animname);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_playEntityAnimation_const_StringR_const_StringR_bool(self.as_raw_mut_WindowScene(), name.opencv_as_extern(), animname.opencv_as_extern(), loop_, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// play entity animation
	/// ## Parameters
	/// * name: entity name
	/// * animname: animation name
	/// * loop: enable or disable animation loop
	/// ## See also
	/// getEntityAnimations
	///
	/// ## Note
	/// This alternative version of [WindowSceneTrait::play_entity_animation] function uses the following default values for its arguments:
	/// * loop_: true
	// cv::ovis::WindowScene::playEntityAnimation(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:224
	// ("cv::ovis::WindowScene::playEntityAnimation", vec![(pred!(mut, ["name", "animname"], ["const cv::String*", "const cv::String*"]), _)]),
	#[inline]
	fn play_entity_animation_def(&mut self, name: &str, animname: &str) -> Result<()> {
		extern_container_arg!(name);
		extern_container_arg!(animname);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_playEntityAnimation_const_StringR_const_StringR(self.as_raw_mut_WindowScene(), name.opencv_as_extern(), animname.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// stop entity animation
	/// ## Parameters
	/// * name: enitity name
	/// * animname: animation name
	// stopEntityAnimation(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:232
	// ("cv::ovis::WindowScene::stopEntityAnimation", vec![(pred!(mut, ["name", "animname"], ["const cv::String*", "const cv::String*"]), _)]),
	#[inline]
	fn stop_entity_animation(&mut self, name: &str, animname: &str) -> Result<()> {
		extern_container_arg!(name);
		extern_container_arg!(animname);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_stopEntityAnimation_const_StringR_const_StringR(self.as_raw_mut_WindowScene(), name.opencv_as_extern(), animname.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// read back the image generated by the last call to [waitKey]
	// getScreenshot(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:237
	// ("cv::ovis::WindowScene::getScreenshot", vec![(pred!(mut, ["frame"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_screenshot(&mut self, frame: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(frame);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_getScreenshot_const__OutputArrayR(self.as_raw_mut_WindowScene(), frame.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// read back the texture of an active compositor
	/// ## Parameters
	/// * compname: name of the compositor
	/// * texname: name of the texture inside the compositor
	/// * mrtIndex: if texture is a MRT, specifies the attachment
	/// * out: the texture contents
	///
	/// ## C++ default parameters
	/// * mrt_index: 0
	// getCompositorTexture(const String &, const String &, OutputArray, int)(InString, InString, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:246
	// ("cv::ovis::WindowScene::getCompositorTexture", vec![(pred!(mut, ["compname", "texname", "out", "mrtIndex"], ["const cv::String*", "const cv::String*", "const cv::_OutputArray*", "int"]), _)]),
	#[inline]
	fn get_compositor_texture(&mut self, compname: &str, texname: &str, out: &mut impl ToOutputArray, mrt_index: i32) -> Result<()> {
		extern_container_arg!(compname);
		extern_container_arg!(texname);
		output_array_arg!(out);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_getCompositorTexture_const_StringR_const_StringR_const__OutputArrayR_int(self.as_raw_mut_WindowScene(), compname.opencv_as_extern(), texname.opencv_as_extern(), out.as_raw__OutputArray(), mrt_index, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// read back the texture of an active compositor
	/// ## Parameters
	/// * compname: name of the compositor
	/// * texname: name of the texture inside the compositor
	/// * mrtIndex: if texture is a MRT, specifies the attachment
	/// * out: the texture contents
	///
	/// ## Note
	/// This alternative version of [WindowSceneTrait::get_compositor_texture] function uses the following default values for its arguments:
	/// * mrt_index: 0
	// cv::ovis::WindowScene::getCompositorTexture(InString, InString, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:246
	// ("cv::ovis::WindowScene::getCompositorTexture", vec![(pred!(mut, ["compname", "texname", "out"], ["const cv::String*", "const cv::String*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_compositor_texture_def(&mut self, compname: &str, texname: &str, out: &mut impl ToOutputArray) -> Result<()> {
		extern_container_arg!(compname);
		extern_container_arg!(texname);
		output_array_arg!(out);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_getCompositorTexture_const_StringR_const_StringR_const__OutputArrayR(self.as_raw_mut_WindowScene(), compname.opencv_as_extern(), texname.opencv_as_extern(), out.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// get the depth for the current frame.
	///
	/// return the per pixel distance to the camera in world units
	// getDepth(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:254
	// ("cv::ovis::WindowScene::getDepth", vec![(pred!(mut, ["depth"], ["const cv::_OutputArray*"]), _)]),
	#[inline]
	fn get_depth(&mut self, depth: &mut impl ToOutputArray) -> Result<()> {
		output_array_arg!(depth);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_getDepth_const__OutputArrayR(self.as_raw_mut_WindowScene(), depth.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// convenience method to force the "up" axis to stay fixed
	///
	/// works with both programmatic changes and SCENE_INTERACTIVE
	/// ## Parameters
	/// * useFixed: whether to enforce the fixed yaw axis
	/// * up: the axis to be fixed
	///
	/// ## C++ default parameters
	/// * up: noArray()
	// fixCameraYawAxis(bool, InputArray)(Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:263
	// ("cv::ovis::WindowScene::fixCameraYawAxis", vec![(pred!(mut, ["useFixed", "up"], ["bool", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn fix_camera_yaw_axis(&mut self, use_fixed: bool, up: &impl ToInputArray) -> Result<()> {
		input_array_arg!(up);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_fixCameraYawAxis_bool_const__InputArrayR(self.as_raw_mut_WindowScene(), use_fixed, up.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// convenience method to force the "up" axis to stay fixed
	///
	/// works with both programmatic changes and SCENE_INTERACTIVE
	/// ## Parameters
	/// * useFixed: whether to enforce the fixed yaw axis
	/// * up: the axis to be fixed
	///
	/// ## Note
	/// This alternative version of [WindowSceneTrait::fix_camera_yaw_axis] function uses the following default values for its arguments:
	/// * up: noArray()
	// cv::ovis::WindowScene::fixCameraYawAxis(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:263
	// ("cv::ovis::WindowScene::fixCameraYawAxis", vec![(pred!(mut, ["useFixed"], ["bool"]), _)]),
	#[inline]
	fn fix_camera_yaw_axis_def(&mut self, use_fixed: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_fixCameraYawAxis_bool(self.as_raw_mut_WindowScene(), use_fixed, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the current camera pose
	/// ## Parameters
	/// * tvec: translation
	/// * rot: [Rodrigues] vector or 3x3 rotation matrix
	/// * invert: use the inverse of the given pose
	///
	/// ## C++ default parameters
	/// * tvec: noArray()
	/// * rot: noArray()
	/// * invert: false
	// setCameraPose(InputArray, InputArray, bool)(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:271
	// ("cv::ovis::WindowScene::setCameraPose", vec![(pred!(mut, ["tvec", "rot", "invert"], ["const cv::_InputArray*", "const cv::_InputArray*", "bool"]), _)]),
	#[inline]
	fn set_camera_pose(&mut self, tvec: &impl ToInputArray, rot: &impl ToInputArray, invert: bool) -> Result<()> {
		input_array_arg!(tvec);
		input_array_arg!(rot);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_setCameraPose_const__InputArrayR_const__InputArrayR_bool(self.as_raw_mut_WindowScene(), tvec.as_raw__InputArray(), rot.as_raw__InputArray(), invert, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Sets the current camera pose
	/// ## Parameters
	/// * tvec: translation
	/// * rot: [Rodrigues] vector or 3x3 rotation matrix
	/// * invert: use the inverse of the given pose
	///
	/// ## Note
	/// This alternative version of [WindowSceneTrait::set_camera_pose] function uses the following default values for its arguments:
	/// * tvec: noArray()
	/// * rot: noArray()
	/// * invert: false
	// cv::ovis::WindowScene::setCameraPose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:271
	// ("cv::ovis::WindowScene::setCameraPose", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn set_camera_pose_def(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_setCameraPose(self.as_raw_mut_WindowScene(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// convenience method to orient the camera to a specific entity
	/// ## Parameters
	/// * target: entity name
	/// * offset: offset from entity centre
	///
	/// ## C++ default parameters
	/// * offset: noArray()
	// setCameraLookAt(const String &, InputArray)(InString, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:279
	// ("cv::ovis::WindowScene::setCameraLookAt", vec![(pred!(mut, ["target", "offset"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn set_camera_look_at(&mut self, target: &str, offset: &impl ToInputArray) -> Result<()> {
		extern_container_arg!(target);
		input_array_arg!(offset);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_setCameraLookAt_const_StringR_const__InputArrayR(self.as_raw_mut_WindowScene(), target.opencv_as_extern(), offset.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// convenience method to orient the camera to a specific entity
	/// ## Parameters
	/// * target: entity name
	/// * offset: offset from entity centre
	///
	/// ## Note
	/// This alternative version of [WindowSceneTrait::set_camera_look_at] function uses the following default values for its arguments:
	/// * offset: noArray()
	// cv::ovis::WindowScene::setCameraLookAt(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:279
	// ("cv::ovis::WindowScene::setCameraLookAt", vec![(pred!(mut, ["target"], ["const cv::String*"]), _)]),
	#[inline]
	fn set_camera_look_at_def(&mut self, target: &str) -> Result<()> {
		extern_container_arg!(target);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_setCameraLookAt_const_StringR(self.as_raw_mut_WindowScene(), target.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// convenience method to orient an entity to a specific entity.
	/// If target is an empty string the entity looks at the given offset point
	/// ## Parameters
	/// * origin: entity to make look at
	/// * target: name of target entity
	/// * offset: offset from entity centre
	///
	/// ## C++ default parameters
	/// * offset: noArray()
	// setEntityLookAt(const String &, const String &, InputArray)(InString, InString, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:288
	// ("cv::ovis::WindowScene::setEntityLookAt", vec![(pred!(mut, ["origin", "target", "offset"], ["const cv::String*", "const cv::String*", "const cv::_InputArray*"]), _)]),
	#[inline]
	fn set_entity_look_at(&mut self, origin: &str, target: &str, offset: &impl ToInputArray) -> Result<()> {
		extern_container_arg!(origin);
		extern_container_arg!(target);
		input_array_arg!(offset);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_setEntityLookAt_const_StringR_const_StringR_const__InputArrayR(self.as_raw_mut_WindowScene(), origin.opencv_as_extern(), target.opencv_as_extern(), offset.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// convenience method to orient an entity to a specific entity.
	/// If target is an empty string the entity looks at the given offset point
	/// ## Parameters
	/// * origin: entity to make look at
	/// * target: name of target entity
	/// * offset: offset from entity centre
	///
	/// ## Note
	/// This alternative version of [WindowSceneTrait::set_entity_look_at] function uses the following default values for its arguments:
	/// * offset: noArray()
	// cv::ovis::WindowScene::setEntityLookAt(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:288
	// ("cv::ovis::WindowScene::setEntityLookAt", vec![(pred!(mut, ["origin", "target"], ["const cv::String*", "const cv::String*"]), _)]),
	#[inline]
	fn set_entity_look_at_def(&mut self, origin: &str, target: &str) -> Result<()> {
		extern_container_arg!(origin);
		extern_container_arg!(target);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_setEntityLookAt_const_StringR_const_StringR(self.as_raw_mut_WindowScene(), origin.opencv_as_extern(), target.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Retrieves the current camera pose
	/// ## Parameters
	/// * R: 3x3 rotation matrix
	/// * tvec: translation vector
	/// * invert: return the inverted pose
	///
	/// ## C++ default parameters
	/// * r: noArray()
	/// * tvec: noArray()
	/// * invert: false
	// getCameraPose(OutputArray, OutputArray, bool)(OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:296
	// ("cv::ovis::WindowScene::getCameraPose", vec![(pred!(mut, ["R", "tvec", "invert"], ["const cv::_OutputArray*", "const cv::_OutputArray*", "bool"]), _)]),
	#[inline]
	fn get_camera_pose(&mut self, r: &mut impl ToOutputArray, tvec: &mut impl ToOutputArray, invert: bool) -> Result<()> {
		output_array_arg!(r);
		output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_getCameraPose_const__OutputArrayR_const__OutputArrayR_bool(self.as_raw_mut_WindowScene(), r.as_raw__OutputArray(), tvec.as_raw__OutputArray(), invert, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Retrieves the current camera pose
	/// ## Parameters
	/// * R: 3x3 rotation matrix
	/// * tvec: translation vector
	/// * invert: return the inverted pose
	///
	/// ## Note
	/// This alternative version of [WindowSceneTrait::get_camera_pose] function uses the following default values for its arguments:
	/// * r: noArray()
	/// * tvec: noArray()
	/// * invert: false
	// cv::ovis::WindowScene::getCameraPose() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:296
	// ("cv::ovis::WindowScene::getCameraPose", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn get_camera_pose_def(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_getCameraPose(self.as_raw_mut_WindowScene(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// set intrinsics of the camera
	///
	/// ## Parameters
	/// * K: intrinsic matrix or noArray(). If noArray() is specified, imsize
	/// is ignored and zNear/ zFar can be set separately.
	/// * imsize: image size
	/// * zNear: near clip distance or -1 to keep the current
	/// * zFar: far clip distance or -1 to keep the current
	///
	/// ## C++ default parameters
	/// * z_near: -1
	/// * z_far: -1
	// setCameraIntrinsics(InputArray, const Size &, float, float)(InputArray, SimpleClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:308
	// ("cv::ovis::WindowScene::setCameraIntrinsics", vec![(pred!(mut, ["K", "imsize", "zNear", "zFar"], ["const cv::_InputArray*", "const cv::Size*", "float", "float"]), _)]),
	#[inline]
	fn set_camera_intrinsics(&mut self, k: &impl ToInputArray, imsize: core::Size, z_near: f32, z_far: f32) -> Result<()> {
		input_array_arg!(k);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_setCameraIntrinsics_const__InputArrayR_const_SizeR_float_float(self.as_raw_mut_WindowScene(), k.as_raw__InputArray(), &imsize, z_near, z_far, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// set intrinsics of the camera
	///
	/// ## Parameters
	/// * K: intrinsic matrix or noArray(). If noArray() is specified, imsize
	/// is ignored and zNear/ zFar can be set separately.
	/// * imsize: image size
	/// * zNear: near clip distance or -1 to keep the current
	/// * zFar: far clip distance or -1 to keep the current
	///
	/// ## Note
	/// This alternative version of [WindowSceneTrait::set_camera_intrinsics] function uses the following default values for its arguments:
	/// * z_near: -1
	/// * z_far: -1
	// cv::ovis::WindowScene::setCameraIntrinsics(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:308
	// ("cv::ovis::WindowScene::setCameraIntrinsics", vec![(pred!(mut, ["K", "imsize"], ["const cv::_InputArray*", "const cv::Size*"]), _)]),
	#[inline]
	fn set_camera_intrinsics_def(&mut self, k: &impl ToInputArray, imsize: core::Size) -> Result<()> {
		input_array_arg!(k);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_setCameraIntrinsics_const__InputArrayR_const_SizeR(self.as_raw_mut_WindowScene(), k.as_raw__InputArray(), &imsize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// render this window, but do not swap buffers. Automatically called by [ovis::waitKey]
	// update()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:314
	// ("cv::ovis::WindowScene::update", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn update(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_WindowScene_update(self.as_raw_mut_WindowScene(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// A 3D viewport and the associated scene
// WindowScene /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/ovis.hpp:86
pub struct WindowScene {
	ptr: *mut c_void,
}

opencv_type_boxed! { WindowScene }

impl Drop for WindowScene {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_ovis_WindowScene_delete(self.as_raw_mut_WindowScene()) };
	}
}

unsafe impl Send for WindowScene {}

impl crate::ovis::WindowSceneTraitConst for WindowScene {
	#[inline] fn as_raw_WindowScene(&self) -> *const c_void { self.as_raw() }
}

impl crate::ovis::WindowSceneTrait for WindowScene {
	#[inline] fn as_raw_mut_WindowScene(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { WindowScene, crate::ovis::WindowSceneTraitConst, as_raw_WindowScene, crate::ovis::WindowSceneTrait, as_raw_mut_WindowScene }

impl WindowScene {
}

impl std::fmt::Debug for WindowScene {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("WindowScene")
			.finish()
	}
}
