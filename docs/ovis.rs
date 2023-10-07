pub mod ovis {
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
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::WindowSceneTraitConst, super::WindowSceneTrait };
	}
	
	pub const ENTITY_AABB_WORLD: i32 = 2;
	pub const ENTITY_ANIMBLEND_MODE: i32 = 3;
	pub const ENTITY_CAST_SHADOWS: i32 = 4;
	pub const ENTITY_MATERIAL: i32 = 0;
	pub const ENTITY_SCALE: i32 = 1;
	pub const MATERIAL_DIFFUSE: i32 = 4;
	pub const MATERIAL_EMISSIVE: i32 = 3;
	pub const MATERIAL_LINE_WIDTH: i32 = 1;
	pub const MATERIAL_OPACITY: i32 = 2;
	pub const MATERIAL_POINT_SIZE: i32 = 0;
	pub const MATERIAL_TEXTURE: i32 = 5;
	pub const MATERIAL_TEXTURE0: i32 = 5;
	pub const MATERIAL_TEXTURE1: i32 = 6;
	pub const MATERIAL_TEXTURE2: i32 = 7;
	pub const MATERIAL_TEXTURE3: i32 = 8;
	/// Apply anti-aliasing. The first window determines the setting for all windows.
	pub const SCENE_AA: i32 = 8;
	/// allow the user to control the camera.
	pub const SCENE_INTERACTIVE: i32 = 2;
	/// Render off-screen without a window. Allows separate AA setting. Requires manual update via [WindowScene::update]
	pub const SCENE_OFFSCREEN: i32 = 16;
	/// the window will use a separate scene. The scene will be shared otherwise.
	pub const SCENE_SEPARATE: i32 = 1;
	/// Enable real-time shadows in the scene. All entities cast shadows by default. Control via [ENTITY_CAST_SHADOWS]
	pub const SCENE_SHADOWS: i32 = 32;
	/// draw coordinate system crosses for debugging
	pub const SCENE_SHOW_CS_CROSS: i32 = 4;
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum EntityProperty {
		ENTITY_MATERIAL = 0,
		ENTITY_SCALE = 1,
		ENTITY_AABB_WORLD = 2,
		ENTITY_ANIMBLEND_MODE = 3,
		ENTITY_CAST_SHADOWS = 4,
	}
	
	opencv_type_enum! { crate::ovis::EntityProperty }
	
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
	
	opencv_type_enum! { crate::ovis::MaterialProperty }
	
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
	
	opencv_type_enum! { crate::ovis::SceneSettings }
	
	/// Add an additional resource location that is search for meshes, textures and materials
	/// 
	/// must be called before the first createWindow. If give path does not exist, retries inside
	/// Ogre Media Directory.
	/// ## Parameters
	/// * path: folder or Zip archive.
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
	#[inline]
	pub fn create_plane_mesh(name: &str, size: core::Size2f, image: &impl core::ToInputArray) -> Result<()> {
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
	#[inline]
	pub fn create_point_cloud_mesh_def(name: &str, vertices: &impl core::ToInputArray) -> Result<()> {
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
	#[inline]
	pub fn create_point_cloud_mesh(name: &str, vertices: &impl core::ToInputArray, colors: &impl core::ToInputArray) -> Result<()> {
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
	#[inline]
	pub fn create_triangle_mesh_def(name: &str, vertices: &impl core::ToInputArray) -> Result<()> {
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
	#[inline]
	pub fn create_triangle_mesh(name: &str, vertices: &impl core::ToInputArray, normals: &impl core::ToInputArray, indices: &impl core::ToInputArray) -> Result<()> {
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
	#[inline]
	pub fn set_material_texture(name: &str, prop: i32, value: &impl core::ToInputArray) -> Result<()> {
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
	#[inline]
	pub fn update_texture(name: &str, image: &impl core::ToInputArray) -> Result<()> {
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
	#[inline]
	pub fn wait_key(delay: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ovis_waitKey_int(delay, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Constant methods for [crate::ovis::WindowScene]
	pub trait WindowSceneTraitConst {
		fn as_raw_WindowScene(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::ovis::WindowScene]
	pub trait WindowSceneTrait: crate::ovis::WindowSceneTraitConst {
		fn as_raw_mut_WindowScene(&mut self) -> *mut c_void;
	
		/// set window background to custom image/ color
		/// ## Parameters
		/// * image: 
		#[inline]
		fn set_background(&mut self, image: &impl core::ToInputArray) -> Result<()> {
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
		#[inline]
		fn create_entity(&mut self, name: &str, meshname: &str, tvec: &impl core::ToInputArray, rot: &impl core::ToInputArray) -> Result<()> {
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
		/// This alternative version of [create_entity] function uses the following default values for its arguments:
		/// * tvec: noArray()
		/// * rot: noArray()
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
		/// This alternative version of [set_entity_property] function uses the following default values for its arguments:
		/// * sub_entity_idx: -1
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
		#[inline]
		fn get_entity_property(&mut self, name: &str, prop: i32, value: &mut impl core::ToOutputArray) -> Result<()> {
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
		#[inline]
		fn create_camera_entity(&mut self, name: &str, k: &impl core::ToInputArray, imsize: core::Size, z_far: f32, tvec: &impl core::ToInputArray, rot: &impl core::ToInputArray, color: core::Scalar) -> Result<core::Rect2d> {
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
		/// This alternative version of [create_camera_entity] function uses the following default values for its arguments:
		/// * tvec: noArray()
		/// * rot: noArray()
		/// * color: Scalar::all(1)
		#[inline]
		fn create_camera_entity_def(&mut self, name: &str, k: &impl core::ToInputArray, imsize: core::Size, z_far: f32) -> Result<core::Rect2d> {
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
		#[inline]
		fn create_light_entity(&mut self, name: &str, tvec: &impl core::ToInputArray, rot: &impl core::ToInputArray, diffuse_color: core::Scalar, specular_color: core::Scalar) -> Result<()> {
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
		/// This alternative version of [create_light_entity] function uses the following default values for its arguments:
		/// * tvec: noArray()
		/// * rot: noArray()
		/// * diffuse_color: Scalar::all(1)
		/// * specular_color: Scalar::all(1)
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
		#[inline]
		fn update_entity_pose(&mut self, name: &str, tvec: &impl core::ToInputArray, rot: &impl core::ToInputArray) -> Result<()> {
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
		/// This alternative version of [update_entity_pose] function uses the following default values for its arguments:
		/// * tvec: noArray()
		/// * rot: noArray()
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
		#[inline]
		fn set_entity_pose(&mut self, name: &str, tvec: &impl core::ToInputArray, rot: &impl core::ToInputArray, invert: bool) -> Result<()> {
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
		/// This alternative version of [set_entity_pose] function uses the following default values for its arguments:
		/// * tvec: noArray()
		/// * rot: noArray()
		/// * invert: false
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
		#[inline]
		fn get_entity_pose(&mut self, name: &str, r: &mut impl core::ToOutputArray, tvec: &mut impl core::ToOutputArray, invert: bool) -> Result<()> {
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
		/// This alternative version of [get_entity_pose] function uses the following default values for its arguments:
		/// * r: noArray()
		/// * tvec: noArray()
		/// * invert: false
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
		/// This alternative version of [play_entity_animation] function uses the following default values for its arguments:
		/// * loop_: true
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
		#[inline]
		fn get_screenshot(&mut self, frame: &mut impl core::ToOutputArray) -> Result<()> {
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
		#[inline]
		fn get_compositor_texture(&mut self, compname: &str, texname: &str, out: &mut impl core::ToOutputArray, mrt_index: i32) -> Result<()> {
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
		/// This alternative version of [get_compositor_texture] function uses the following default values for its arguments:
		/// * mrt_index: 0
		#[inline]
		fn get_compositor_texture_def(&mut self, compname: &str, texname: &str, out: &mut impl core::ToOutputArray) -> Result<()> {
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
		#[inline]
		fn get_depth(&mut self, depth: &mut impl core::ToOutputArray) -> Result<()> {
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
		#[inline]
		fn fix_camera_yaw_axis(&mut self, use_fixed: bool, up: &impl core::ToInputArray) -> Result<()> {
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
		/// This alternative version of [fix_camera_yaw_axis] function uses the following default values for its arguments:
		/// * up: noArray()
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
		#[inline]
		fn set_camera_pose(&mut self, tvec: &impl core::ToInputArray, rot: &impl core::ToInputArray, invert: bool) -> Result<()> {
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
		/// This alternative version of [set_camera_pose] function uses the following default values for its arguments:
		/// * tvec: noArray()
		/// * rot: noArray()
		/// * invert: false
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
		#[inline]
		fn set_camera_look_at(&mut self, target: &str, offset: &impl core::ToInputArray) -> Result<()> {
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
		/// This alternative version of [set_camera_look_at] function uses the following default values for its arguments:
		/// * offset: noArray()
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
		#[inline]
		fn set_entity_look_at(&mut self, origin: &str, target: &str, offset: &impl core::ToInputArray) -> Result<()> {
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
		/// This alternative version of [set_entity_look_at] function uses the following default values for its arguments:
		/// * offset: noArray()
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
		#[inline]
		fn get_camera_pose(&mut self, r: &mut impl core::ToOutputArray, tvec: &mut impl core::ToOutputArray, invert: bool) -> Result<()> {
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
		/// This alternative version of [get_camera_pose] function uses the following default values for its arguments:
		/// * r: noArray()
		/// * tvec: noArray()
		/// * invert: false
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
		#[inline]
		fn set_camera_intrinsics(&mut self, k: &impl core::ToInputArray, imsize: core::Size, z_near: f32, z_far: f32) -> Result<()> {
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
		/// This alternative version of [set_camera_intrinsics] function uses the following default values for its arguments:
		/// * z_near: -1
		/// * z_far: -1
		#[inline]
		fn set_camera_intrinsics_def(&mut self, k: &impl core::ToInputArray, imsize: core::Size) -> Result<()> {
			input_array_arg!(k);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ovis_WindowScene_setCameraIntrinsics_const__InputArrayR_const_SizeR(self.as_raw_mut_WindowScene(), k.as_raw__InputArray(), &imsize, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// render this window, but do not swap buffers. Automatically called by [ovis::waitKey]
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
	pub struct WindowScene {
		ptr: *mut c_void
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
	
	impl WindowScene {
	}
	
	impl std::fmt::Debug for WindowScene {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("WindowScene")
				.finish()
		}
	}
}
