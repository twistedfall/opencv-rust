pub mod rapid {
	//! # silhouette based 3D object tracking
	//! 
	//! implements "RAPID-a video rate object tracker" [harris1990rapid](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_harris1990rapid) with the dynamic control point extraction of [drummond2002real](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_drummond2002real)
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::Rapid_TrackerTraitConst, super::Rapid_TrackerTrait, super::Rapid_RapidTraitConst, super::Rapid_RapidTrait, super::Rapid_OLSTrackerTraitConst, super::Rapid_OLSTrackerTrait, super::Rapid_GOSTrackerTraitConst, super::Rapid_GOSTrackerTrait };
	}
	
	/// Collect corresponding 2d and 3d points based on correspondencies and mask
	/// ## Parameters
	/// * cols: correspondence-position per line in line-bundle-space
	/// * srcLocations: the source image location
	/// * pts2d: 2d points
	/// * pts3d: 3d points
	/// * mask: mask containing non-zero values for the elements to be retained
	/// 
	/// ## Note
	/// This alternative version of [convert_correspondencies] function uses the following default values for its arguments:
	/// * pts3d: noArray()
	/// * mask: noArray()
	#[inline]
	pub fn convert_correspondencies_def(cols: &impl core::ToInputArray, src_locations: &impl core::ToInputArray, pts2d: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(cols);
		input_array_arg!(src_locations);
		output_array_arg!(pts2d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_convertCorrespondencies_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cols.as_raw__InputArray(), src_locations.as_raw__InputArray(), pts2d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Collect corresponding 2d and 3d points based on correspondencies and mask
	/// ## Parameters
	/// * cols: correspondence-position per line in line-bundle-space
	/// * srcLocations: the source image location
	/// * pts2d: 2d points
	/// * pts3d: 3d points
	/// * mask: mask containing non-zero values for the elements to be retained
	/// 
	/// ## C++ default parameters
	/// * pts3d: noArray()
	/// * mask: noArray()
	#[inline]
	pub fn convert_correspondencies(cols: &impl core::ToInputArray, src_locations: &impl core::ToInputArray, pts2d: &mut impl core::ToOutputArray, pts3d: &mut impl core::ToInputOutputArray, mask: &impl core::ToInputArray) -> Result<()> {
		input_array_arg!(cols);
		input_array_arg!(src_locations);
		output_array_arg!(pts2d);
		input_output_array_arg!(pts3d);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_convertCorrespondencies_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__InputArrayR(cols.as_raw__InputArray(), src_locations.as_raw__InputArray(), pts2d.as_raw__OutputArray(), pts3d.as_raw__InputOutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Debug draw markers of matched correspondences onto a lineBundle
	/// ## Parameters
	/// * bundle: the lineBundle
	/// * cols: column coordinates in the line bundle
	/// * colors: colors for the markers. Defaults to white.
	/// 
	/// ## Note
	/// This alternative version of [draw_correspondencies] function uses the following default values for its arguments:
	/// * colors: noArray()
	#[inline]
	pub fn draw_correspondencies_def(bundle: &mut impl core::ToInputOutputArray, cols: &impl core::ToInputArray) -> Result<()> {
		input_output_array_arg!(bundle);
		input_array_arg!(cols);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_drawCorrespondencies_const__InputOutputArrayR_const__InputArrayR(bundle.as_raw__InputOutputArray(), cols.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Debug draw markers of matched correspondences onto a lineBundle
	/// ## Parameters
	/// * bundle: the lineBundle
	/// * cols: column coordinates in the line bundle
	/// * colors: colors for the markers. Defaults to white.
	/// 
	/// ## C++ default parameters
	/// * colors: noArray()
	#[inline]
	pub fn draw_correspondencies(bundle: &mut impl core::ToInputOutputArray, cols: &impl core::ToInputArray, colors: &impl core::ToInputArray) -> Result<()> {
		input_output_array_arg!(bundle);
		input_array_arg!(cols);
		input_array_arg!(colors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_drawCorrespondencies_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR(bundle.as_raw__InputOutputArray(), cols.as_raw__InputArray(), colors.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Debug draw search lines onto an image
	/// ## Parameters
	/// * img: the output image
	/// * locations: the source locations of a line bundle
	/// * color: the line color
	#[inline]
	pub fn draw_search_lines(img: &mut impl core::ToInputOutputArray, locations: &impl core::ToInputArray, color: core::Scalar) -> Result<()> {
		input_output_array_arg!(img);
		input_array_arg!(locations);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_drawSearchLines_const__InputOutputArrayR_const__InputArrayR_const_ScalarR(img.as_raw__InputOutputArray(), locations.as_raw__InputArray(), &color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Draw a wireframe of a triangle mesh
	/// ## Parameters
	/// * img: the output image
	/// * pts2d: the 2d points obtained by [projectPoints]
	/// * tris: triangle face connectivity
	/// * color: line color
	/// * type: line type. See [LineTypes].
	/// * cullBackface: enable back-face culling based on CCW order
	/// 
	/// ## Note
	/// This alternative version of [draw_wireframe] function uses the following default values for its arguments:
	/// * typ: LINE_8
	/// * cull_backface: false
	#[inline]
	pub fn draw_wireframe_def(img: &mut impl core::ToInputOutputArray, pts2d: &impl core::ToInputArray, tris: &impl core::ToInputArray, color: core::Scalar) -> Result<()> {
		input_output_array_arg!(img);
		input_array_arg!(pts2d);
		input_array_arg!(tris);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_drawWireframe_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const_ScalarR(img.as_raw__InputOutputArray(), pts2d.as_raw__InputArray(), tris.as_raw__InputArray(), &color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Draw a wireframe of a triangle mesh
	/// ## Parameters
	/// * img: the output image
	/// * pts2d: the 2d points obtained by [projectPoints]
	/// * tris: triangle face connectivity
	/// * color: line color
	/// * type: line type. See [LineTypes].
	/// * cullBackface: enable back-face culling based on CCW order
	/// 
	/// ## C++ default parameters
	/// * typ: LINE_8
	/// * cull_backface: false
	#[inline]
	pub fn draw_wireframe(img: &mut impl core::ToInputOutputArray, pts2d: &impl core::ToInputArray, tris: &impl core::ToInputArray, color: core::Scalar, typ: i32, cull_backface: bool) -> Result<()> {
		input_output_array_arg!(img);
		input_array_arg!(pts2d);
		input_array_arg!(tris);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_drawWireframe_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const_ScalarR_int_bool(img.as_raw__InputOutputArray(), pts2d.as_raw__InputArray(), tris.as_raw__InputArray(), &color, typ, cull_backface, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Extract control points from the projected silhouette of a mesh
	/// 
	/// see [drummond2002real](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_drummond2002real) Sec 2.1, Step b
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
	#[inline]
	pub fn extract_control_points(num: i32, len: i32, pts3d: &impl core::ToInputArray, rvec: &impl core::ToInputArray, tvec: &impl core::ToInputArray, k: &impl core::ToInputArray, imsize: core::Size, tris: &impl core::ToInputArray, ctl2d: &mut impl core::ToOutputArray, ctl3d: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(pts3d);
		input_array_arg!(rvec);
		input_array_arg!(tvec);
		input_array_arg!(k);
		input_array_arg!(tris);
		output_array_arg!(ctl2d);
		output_array_arg!(ctl3d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_extractControlPoints_int_int_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(num, len, pts3d.as_raw__InputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), k.as_raw__InputArray(), &imsize, tris.as_raw__InputArray(), ctl2d.as_raw__OutputArray(), ctl3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Extract the line bundle from an image
	/// ## Parameters
	/// * len: the search radius. The bundle will have `2*len + 1` columns.
	/// * ctl2d: the search lines will be centered at this points and orthogonal to the contour defined by
	/// them. The bundle will have as many rows.
	/// * img: the image to read the pixel intensities values from
	/// * bundle: line bundle image with size `ctl2d.rows() x (2 * len + 1)` and the same type as @p img
	/// * srcLocations: the source pixel locations of @p bundle in @p img as CV_16SC2
	#[inline]
	pub fn extract_line_bundle(len: i32, ctl2d: &impl core::ToInputArray, img: &impl core::ToInputArray, bundle: &mut impl core::ToOutputArray, src_locations: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(ctl2d);
		input_array_arg!(img);
		output_array_arg!(bundle);
		output_array_arg!(src_locations);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_extractLineBundle_int_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(len, ctl2d.as_raw__InputArray(), img.as_raw__InputArray(), bundle.as_raw__OutputArray(), src_locations.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Find corresponding image locations by searching for a maximal sobel edge along the search line (a single
	/// row in the bundle)
	/// ## Parameters
	/// * bundle: the line bundle
	/// * cols: correspondence-position per line in line-bundle-space
	/// * response: the sobel response for the selected point
	/// 
	/// ## Note
	/// This alternative version of [find_correspondencies] function uses the following default values for its arguments:
	/// * response: noArray()
	#[inline]
	pub fn find_correspondencies_def(bundle: &impl core::ToInputArray, cols: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(bundle);
		output_array_arg!(cols);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_findCorrespondencies_const__InputArrayR_const__OutputArrayR(bundle.as_raw__InputArray(), cols.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Find corresponding image locations by searching for a maximal sobel edge along the search line (a single
	/// row in the bundle)
	/// ## Parameters
	/// * bundle: the line bundle
	/// * cols: correspondence-position per line in line-bundle-space
	/// * response: the sobel response for the selected point
	/// 
	/// ## C++ default parameters
	/// * response: noArray()
	#[inline]
	pub fn find_correspondencies(bundle: &impl core::ToInputArray, cols: &mut impl core::ToOutputArray, response: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(bundle);
		output_array_arg!(cols);
		output_array_arg!(response);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_findCorrespondencies_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(bundle.as_raw__InputArray(), cols.as_raw__OutputArray(), response.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// High level function to execute a single rapid [harris1990rapid](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_harris1990rapid) iteration
	/// 
	/// 1. [extractControlPoints]
	/// 2. [extractLineBundle]
	/// 3. [findCorrespondencies]
	/// 4. [convertCorrespondencies]
	/// 5. [solvePnPRefineLM]
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
	/// * rmsd: the 2d reprojection difference
	/// ## Returns
	/// ratio of search lines that could be extracted and matched
	/// 
	/// ## Note
	/// This alternative version of [rapid] function uses the following default values for its arguments:
	/// * rmsd: 0
	#[inline]
	pub fn rapid_def(img: &impl core::ToInputArray, num: i32, len: i32, pts3d: &impl core::ToInputArray, tris: &impl core::ToInputArray, k: &impl core::ToInputArray, rvec: &mut impl core::ToInputOutputArray, tvec: &mut impl core::ToInputOutputArray) -> Result<f32> {
		input_array_arg!(img);
		input_array_arg!(pts3d);
		input_array_arg!(tris);
		input_array_arg!(k);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_rapid_const__InputArrayR_int_int_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(img.as_raw__InputArray(), num, len, pts3d.as_raw__InputArray(), tris.as_raw__InputArray(), k.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// High level function to execute a single rapid [harris1990rapid](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_harris1990rapid) iteration
	/// 
	/// 1. [extractControlPoints]
	/// 2. [extractLineBundle]
	/// 3. [findCorrespondencies]
	/// 4. [convertCorrespondencies]
	/// 5. [solvePnPRefineLM]
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
	/// * rmsd: the 2d reprojection difference
	/// ## Returns
	/// ratio of search lines that could be extracted and matched
	/// 
	/// ## C++ default parameters
	/// * rmsd: 0
	#[inline]
	pub fn rapid(img: &impl core::ToInputArray, num: i32, len: i32, pts3d: &impl core::ToInputArray, tris: &impl core::ToInputArray, k: &impl core::ToInputArray, rvec: &mut impl core::ToInputOutputArray, tvec: &mut impl core::ToInputOutputArray, rmsd: &mut f64) -> Result<f32> {
		input_array_arg!(img);
		input_array_arg!(pts3d);
		input_array_arg!(tris);
		input_array_arg!(k);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_rapid_const__InputArrayR_int_int_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_doubleX(img.as_raw__InputArray(), num, len, pts3d.as_raw__InputArray(), tris.as_raw__InputArray(), k.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), rmsd, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Constant methods for [crate::rapid::Rapid_GOSTracker]
	pub trait Rapid_GOSTrackerTraitConst: crate::rapid::Rapid_TrackerTraitConst {
		fn as_raw_Rapid_GOSTracker(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::rapid::Rapid_GOSTracker]
	pub trait Rapid_GOSTrackerTrait: crate::rapid::Rapid_GOSTrackerTraitConst + crate::rapid::Rapid_TrackerTrait {
		fn as_raw_mut_Rapid_GOSTracker(&mut self) -> *mut c_void;
	
	}
	
	/// implements "Global optimal searching for textureless 3D object tracking" [wang2015global](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_wang2015global)
	pub struct Rapid_GOSTracker {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Rapid_GOSTracker }
	
	impl Drop for Rapid_GOSTracker {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_rapid_GOSTracker_delete(self.as_raw_mut_Rapid_GOSTracker()) };
		}
	}
	
	unsafe impl Send for Rapid_GOSTracker {}
	
	impl core::AlgorithmTraitConst for Rapid_GOSTracker {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for Rapid_GOSTracker {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rapid::Rapid_TrackerTraitConst for Rapid_GOSTracker {
		#[inline] fn as_raw_Rapid_Tracker(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::rapid::Rapid_TrackerTrait for Rapid_GOSTracker {
		#[inline] fn as_raw_mut_Rapid_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rapid::Rapid_GOSTrackerTraitConst for Rapid_GOSTracker {
		#[inline] fn as_raw_Rapid_GOSTracker(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::rapid::Rapid_GOSTrackerTrait for Rapid_GOSTracker {
		#[inline] fn as_raw_mut_Rapid_GOSTracker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Rapid_GOSTracker {
		/// ## C++ default parameters
		/// * hist_bins: 4
		/// * sobel_thesh: 10
		#[inline]
		pub fn create(pts3d: &impl core::ToInputArray, tris: &impl core::ToInputArray, hist_bins: i32, sobel_thesh: u8) -> Result<core::Ptr<crate::rapid::Rapid_OLSTracker>> {
			input_array_arg!(pts3d);
			input_array_arg!(tris);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rapid_GOSTracker_create_const__InputArrayR_const__InputArrayR_int_unsigned_char(pts3d.as_raw__InputArray(), tris.as_raw__InputArray(), hist_bins, sobel_thesh, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rapid::Rapid_OLSTracker>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * hist_bins: 4
		/// * sobel_thesh: 10
		#[inline]
		pub fn create_def(pts3d: &impl core::ToInputArray, tris: &impl core::ToInputArray) -> Result<core::Ptr<crate::rapid::Rapid_OLSTracker>> {
			input_array_arg!(pts3d);
			input_array_arg!(tris);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rapid_GOSTracker_create_const__InputArrayR_const__InputArrayR(pts3d.as_raw__InputArray(), tris.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rapid::Rapid_OLSTracker>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { Rapid_GOSTracker, core::Algorithm, cv_rapid_GOSTracker_to_Algorithm }
	
	boxed_cast_base! { Rapid_GOSTracker, crate::rapid::Rapid_Tracker, cv_rapid_GOSTracker_to_Rapid_Tracker }
	
	impl std::fmt::Debug for Rapid_GOSTracker {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Rapid_GOSTracker")
				.finish()
		}
	}
	
	/// Constant methods for [crate::rapid::Rapid_OLSTracker]
	pub trait Rapid_OLSTrackerTraitConst: crate::rapid::Rapid_TrackerTraitConst {
		fn as_raw_Rapid_OLSTracker(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::rapid::Rapid_OLSTracker]
	pub trait Rapid_OLSTrackerTrait: crate::rapid::Rapid_OLSTrackerTraitConst + crate::rapid::Rapid_TrackerTrait {
		fn as_raw_mut_Rapid_OLSTracker(&mut self) -> *mut c_void;
	
	}
	
	/// implements "Optimal local searching for fast and robust textureless 3D object tracking in highly
	/// cluttered backgrounds" [seo2013optimal](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_seo2013optimal)
	pub struct Rapid_OLSTracker {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Rapid_OLSTracker }
	
	impl Drop for Rapid_OLSTracker {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_rapid_OLSTracker_delete(self.as_raw_mut_Rapid_OLSTracker()) };
		}
	}
	
	unsafe impl Send for Rapid_OLSTracker {}
	
	impl core::AlgorithmTraitConst for Rapid_OLSTracker {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for Rapid_OLSTracker {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rapid::Rapid_TrackerTraitConst for Rapid_OLSTracker {
		#[inline] fn as_raw_Rapid_Tracker(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::rapid::Rapid_TrackerTrait for Rapid_OLSTracker {
		#[inline] fn as_raw_mut_Rapid_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rapid::Rapid_OLSTrackerTraitConst for Rapid_OLSTracker {
		#[inline] fn as_raw_Rapid_OLSTracker(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::rapid::Rapid_OLSTrackerTrait for Rapid_OLSTracker {
		#[inline] fn as_raw_mut_Rapid_OLSTracker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Rapid_OLSTracker {
		/// ## C++ default parameters
		/// * hist_bins: 8
		/// * sobel_thesh: 10
		#[inline]
		pub fn create(pts3d: &impl core::ToInputArray, tris: &impl core::ToInputArray, hist_bins: i32, sobel_thesh: u8) -> Result<core::Ptr<crate::rapid::Rapid_OLSTracker>> {
			input_array_arg!(pts3d);
			input_array_arg!(tris);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rapid_OLSTracker_create_const__InputArrayR_const__InputArrayR_int_unsigned_char(pts3d.as_raw__InputArray(), tris.as_raw__InputArray(), hist_bins, sobel_thesh, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rapid::Rapid_OLSTracker>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * hist_bins: 8
		/// * sobel_thesh: 10
		#[inline]
		pub fn create_def(pts3d: &impl core::ToInputArray, tris: &impl core::ToInputArray) -> Result<core::Ptr<crate::rapid::Rapid_OLSTracker>> {
			input_array_arg!(pts3d);
			input_array_arg!(tris);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rapid_OLSTracker_create_const__InputArrayR_const__InputArrayR(pts3d.as_raw__InputArray(), tris.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rapid::Rapid_OLSTracker>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { Rapid_OLSTracker, core::Algorithm, cv_rapid_OLSTracker_to_Algorithm }
	
	boxed_cast_base! { Rapid_OLSTracker, crate::rapid::Rapid_Tracker, cv_rapid_OLSTracker_to_Rapid_Tracker }
	
	impl std::fmt::Debug for Rapid_OLSTracker {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Rapid_OLSTracker")
				.finish()
		}
	}
	
	/// Constant methods for [crate::rapid::Rapid_Rapid]
	pub trait Rapid_RapidTraitConst: crate::rapid::Rapid_TrackerTraitConst {
		fn as_raw_Rapid_Rapid(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::rapid::Rapid_Rapid]
	pub trait Rapid_RapidTrait: crate::rapid::Rapid_RapidTraitConst + crate::rapid::Rapid_TrackerTrait {
		fn as_raw_mut_Rapid_Rapid(&mut self) -> *mut c_void;
	
	}
	
	/// wrapper around [rapid] function for uniform access
	pub struct Rapid_Rapid {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Rapid_Rapid }
	
	impl Drop for Rapid_Rapid {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_rapid_Rapid_delete(self.as_raw_mut_Rapid_Rapid()) };
		}
	}
	
	unsafe impl Send for Rapid_Rapid {}
	
	impl core::AlgorithmTraitConst for Rapid_Rapid {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for Rapid_Rapid {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rapid::Rapid_TrackerTraitConst for Rapid_Rapid {
		#[inline] fn as_raw_Rapid_Tracker(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::rapid::Rapid_TrackerTrait for Rapid_Rapid {
		#[inline] fn as_raw_mut_Rapid_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rapid::Rapid_RapidTraitConst for Rapid_Rapid {
		#[inline] fn as_raw_Rapid_Rapid(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::rapid::Rapid_RapidTrait for Rapid_Rapid {
		#[inline] fn as_raw_mut_Rapid_Rapid(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Rapid_Rapid {
		#[inline]
		pub fn create(pts3d: &impl core::ToInputArray, tris: &impl core::ToInputArray) -> Result<core::Ptr<crate::rapid::Rapid_Rapid>> {
			input_array_arg!(pts3d);
			input_array_arg!(tris);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rapid_Rapid_create_const__InputArrayR_const__InputArrayR(pts3d.as_raw__InputArray(), tris.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::rapid::Rapid_Rapid>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { Rapid_Rapid, core::Algorithm, cv_rapid_Rapid_to_Algorithm }
	
	boxed_cast_base! { Rapid_Rapid, crate::rapid::Rapid_Tracker, cv_rapid_Rapid_to_Rapid_Tracker }
	
	impl std::fmt::Debug for Rapid_Rapid {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Rapid_Rapid")
				.finish()
		}
	}
	
	/// Constant methods for [crate::rapid::Rapid_Tracker]
	pub trait Rapid_TrackerTraitConst: core::AlgorithmTraitConst {
		fn as_raw_Rapid_Tracker(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::rapid::Rapid_Tracker]
	pub trait Rapid_TrackerTrait: core::AlgorithmTrait + crate::rapid::Rapid_TrackerTraitConst {
		fn as_raw_mut_Rapid_Tracker(&mut self) -> *mut c_void;
	
		/// ## C++ default parameters
		/// * termcrit: TermCriteria(TermCriteria::MAX_ITER|TermCriteria::EPS,5,1.5)
		#[inline]
		fn compute(&mut self, img: &impl core::ToInputArray, num: i32, len: i32, k: &impl core::ToInputArray, rvec: &mut impl core::ToInputOutputArray, tvec: &mut impl core::ToInputOutputArray, termcrit: core::TermCriteria) -> Result<f32> {
			input_array_arg!(img);
			input_array_arg!(k);
			input_output_array_arg!(rvec);
			input_output_array_arg!(tvec);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rapid_Tracker_compute_const__InputArrayR_int_int_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const_TermCriteriaR(self.as_raw_mut_Rapid_Tracker(), img.as_raw__InputArray(), num, len, k.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), &termcrit, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [compute] function uses the following default values for its arguments:
		/// * termcrit: TermCriteria(TermCriteria::MAX_ITER|TermCriteria::EPS,5,1.5)
		#[inline]
		fn compute_def(&mut self, img: &impl core::ToInputArray, num: i32, len: i32, k: &impl core::ToInputArray, rvec: &mut impl core::ToInputOutputArray, tvec: &mut impl core::ToInputOutputArray) -> Result<f32> {
			input_array_arg!(img);
			input_array_arg!(k);
			input_output_array_arg!(rvec);
			input_output_array_arg!(tvec);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rapid_Tracker_compute_const__InputArrayR_int_int_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_mut_Rapid_Tracker(), img.as_raw__InputArray(), num, len, k.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn clear_state(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rapid_Tracker_clearState(self.as_raw_mut_Rapid_Tracker(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Abstract base class for stateful silhouette trackers
	pub struct Rapid_Tracker {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Rapid_Tracker }
	
	impl Drop for Rapid_Tracker {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_rapid_Tracker_delete(self.as_raw_mut_Rapid_Tracker()) };
		}
	}
	
	unsafe impl Send for Rapid_Tracker {}
	
	impl core::AlgorithmTraitConst for Rapid_Tracker {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for Rapid_Tracker {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rapid::Rapid_TrackerTraitConst for Rapid_Tracker {
		#[inline] fn as_raw_Rapid_Tracker(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::rapid::Rapid_TrackerTrait for Rapid_Tracker {
		#[inline] fn as_raw_mut_Rapid_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Rapid_Tracker {
	}
	
	boxed_cast_descendant! { Rapid_Tracker, crate::rapid::Rapid_GOSTracker, cv_rapid_Tracker_to_Rapid_GOSTracker }
	
	boxed_cast_descendant! { Rapid_Tracker, crate::rapid::Rapid_OLSTracker, cv_rapid_Tracker_to_Rapid_OLSTracker }
	
	boxed_cast_descendant! { Rapid_Tracker, crate::rapid::Rapid_Rapid, cv_rapid_Tracker_to_Rapid_Rapid }
	
	boxed_cast_base! { Rapid_Tracker, core::Algorithm, cv_rapid_Tracker_to_Algorithm }
	
	impl std::fmt::Debug for Rapid_Tracker {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Rapid_Tracker")
				.finish()
		}
	}
}
