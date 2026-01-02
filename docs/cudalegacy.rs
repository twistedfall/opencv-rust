pub mod cudalegacy {
	//! # Legacy support
	use crate::mod_prelude::*;
	use crate::{core, sys, types};
	pub mod prelude {
		pub use super::{CUDA_BackgroundSubtractorFGDTrait, CUDA_BackgroundSubtractorFGDTraitConst, CUDA_BackgroundSubtractorGMGTrait, CUDA_BackgroundSubtractorGMGTraitConst, CUDA_FGDParamsTrait, CUDA_FGDParamsTraitConst, CUDA_FastOpticalFlowBMTrait, CUDA_FastOpticalFlowBMTraitConst, CUDA_ImagePyramidTrait, CUDA_ImagePyramidTraitConst};
	}

	pub const HAAR_STDDEV_BORDER: i32 = 1;
	pub const HaarFeature64_CreateCheck_MaxRectField: i32 = 0xFF;
	pub const HaarFeatureDescriptor32_CreateCheck_MaxFeatureOffset: i32 = 0x00FFFFFF;
	pub const HaarFeatureDescriptor32_CreateCheck_MaxNumFeatures: i32 = 0x1F;
	pub const HaarFeatureDescriptor32_Interpret_MaskFlagLeftNodeLeaf: i32 = 0x40000000;
	pub const HaarFeatureDescriptor32_Interpret_MaskFlagRightNodeLeaf: i32 = 0x20000000;
	pub const HaarFeatureDescriptor32_Interpret_MaskFlagTilted: i32 = 0x80000000u32 as i32;
	pub const HaarFeatureDescriptor32_NumFeatures_Shift: i32 = 24;
	pub const HaarStage64_Interpret_MaskRootNodeOffset: i32 = 0xFFFF0000u32 as i32;
	pub const HaarStage64_Interpret_MaskRootNodes: i32 = 0x0000FFFF;
	pub const HaarStage64_Interpret_ShiftRootNodeOffset: i32 = 16;
	pub const NCVMemoryTypeDevice: i32 = 3;
	pub const NCVMemoryTypeHostPageable: i32 = 1;
	pub const NCVMemoryTypeHostPinned: i32 = 2;
	pub const NCVMemoryTypeNone: i32 = 0;
	pub const NCVPipeObjDet_Default: i32 = 0;
	pub const NCVPipeObjDet_FindLargestObject: i32 = 2;
	pub const NCVPipeObjDet_UseFairImageScaling: i32 = 1;
	pub const NCVPipeObjDet_VisualizeInPlace: i32 = 4;
	pub const NCV_ALLOCATOR_BAD_ALLOC: i32 = 13;
	pub const NCV_ALLOCATOR_BAD_DEALLOC: i32 = 14;
	pub const NCV_ALLOCATOR_BAD_REUSE: i32 = 17;
	pub const NCV_ALLOCATOR_DEALLOC_ORDER: i32 = 16;
	pub const NCV_ALLOCATOR_INSUFFICIENT_CAPACITY: i32 = 15;
	pub const NCV_ALLOCATOR_NOT_INITIALIZED: i32 = 12;
	pub const NCV_CUDA_ERROR: i32 = 2;
	pub const NCV_DIMENSIONS_INVALID: i32 = 8;
	pub const NCV_FILE_ERROR: i32 = 4;
	pub const NCV_HAAR_INVALID_PIXEL_STEP: i32 = 21;
	pub const NCV_HAAR_TOO_LARGE_FEATURES: i32 = 24;
	pub const NCV_HAAR_TOO_MANY_FEATURES_IN_CASCADE: i32 = 23;
	pub const NCV_HAAR_TOO_MANY_FEATURES_IN_CLASSIFIER: i32 = 22;
	pub const NCV_HAAR_XML_LOADING_EXCEPTION: i32 = 25;
	pub const NCV_INCONSISTENT_INPUT: i32 = 6;
	pub const NCV_INVALID_ROI: i32 = 9;
	pub const NCV_INVALID_SCALE: i32 = 11;
	pub const NCV_INVALID_STEP: i32 = 10;
	/// Marker to continue error numeration in other files
	pub const NCV_LAST_STATUS: i32 = 14;
	pub const NCV_MEM_COPY_ERROR: i32 = 18;
	pub const NCV_MEM_INSUFFICIENT_CAPACITY: i32 = 20;
	pub const NCV_MEM_RESIDENCE_ERROR: i32 = 19;
	pub const NCV_NOIMPL_HAAR_TILTED_FEATURES: i32 = 26;
	pub const NCV_NOT_IMPLEMENTED: i32 = 27;
	pub const NCV_NPP_ERROR: i32 = 3;
	pub const NCV_NULL_PTR: i32 = 5;
	pub const NCV_SUCCESS: i32 = 0;
	pub const NCV_TEXTURE_BIND_ERROR: i32 = 7;
	pub const NCV_UNKNOWN_ERROR: i32 = 1;
	pub const NCV_WARNING_HAAR_DETECTIONS_VECTOR_OVERFLOW: i32 = 28;
	/// CUDA kernel execution error
	pub const NPPST_CUDA_KERNEL_EXECUTION_ERROR: i32 = 2;
	/// Unknown error
	pub const NPPST_ERROR: i32 = 1;
	/// Invalid region of interest argument
	pub const NPPST_INVALID_ROI: i32 = 8;
	/// Invalid scale parameter passed
	pub const NPPST_INVALID_SCALE: i32 = 10;
	/// Invalid image lines step argument (check sign, alignment, relation to image width)
	pub const NPPST_INVALID_STEP: i32 = 9;
	/// CUDA memory copy error
	pub const NPPST_MEMCPY_ERROR: i32 = 5;
	/// CUDA memory deallocation error
	pub const NPPST_MEMFREE_ERR: i32 = 7;
	/// CUDA memory allocation error
	pub const NPPST_MEM_ALLOC_ERR: i32 = 6;
	/// Insufficient user-allocated buffer
	pub const NPPST_MEM_INSUFFICIENT_BUFFER: i32 = 11;
	/// Internal memory management error
	pub const NPPST_MEM_INTERNAL_ERROR: i32 = 13;
	/// Memory residence error detected (check if pointers should be device or pinned)
	pub const NPPST_MEM_RESIDENCE_ERROR: i32 = 12;
	/// NULL pointer argument error
	pub const NPPST_NULL_POINTER_ERROR: i32 = 3;
	/// Successful operation (same as NPP_NO_ERROR)
	pub const NPPST_SUCCESS: i32 = 0;
	/// CUDA texture binding error or non-zero offset returned
	pub const NPPST_TEXTURE_BIND_ERROR: i32 = 4;
	pub const OBJDET_MASK_ELEMENT_INVALID_32U: i32 = 0xFFFFFFFFu32 as i32;
	pub const RECT_SIMILARITY_PROPORTION: f32 = 0.2;
	/// Bicubic convolution filter, a = -0.5 (cubic Hermite spline)
	pub const nppStBicubic: i32 = 1;
	/// Clamp out of range position to borders
	pub const nppStBorderClamp: i32 = 1;
	/// reflect out of range position across borders
	pub const nppStBorderMirror: i32 = 3;
	/// There is no need to define additional pixels, image is extended already
	pub const nppStBorderNone: i32 = 0;
	/// Wrap out of range position. Image becomes periodic.
	pub const nppStBorderWrap: i32 = 2;
	/// Supersampling. For downscaling only
	pub const nppStSupersample: i32 = 0;
	/// NCVMemoryType
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum NCVMemoryType {
		NCVMemoryTypeNone = 0,
		NCVMemoryTypeHostPageable = 1,
		NCVMemoryTypeHostPinned = 2,
		NCVMemoryTypeDevice = 3,
	}

	opencv_type_enum! { crate::cudalegacy::NCVMemoryType { NCVMemoryTypeNone, NCVMemoryTypeHostPageable, NCVMemoryTypeHostPinned, NCVMemoryTypeDevice } }

	/// Border type
	///
	/// Filtering operations assume that each pixel has a neighborhood of pixels.
	/// The following structure describes possible ways to define non-existent pixels.
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum NppStBorderType {
		/// There is no need to define additional pixels, image is extended already
		nppStBorderNone = 0,
		/// Clamp out of range position to borders
		nppStBorderClamp = 1,
		/// Wrap out of range position. Image becomes periodic.
		nppStBorderWrap = 2,
		/// reflect out of range position across borders
		nppStBorderMirror = 3,
	}

	opencv_type_enum! { crate::cudalegacy::NppStBorderType { nppStBorderNone, nppStBorderClamp, nppStBorderWrap, nppStBorderMirror } }

	/// Filter types for image resizing
	#[repr(i32)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum NppStInterpMode {
		/// Supersampling. For downscaling only
		nppStSupersample = 0,
		/// Bicubic convolution filter, a = -0.5 (cubic Hermite spline)
		nppStBicubic = 1,
	}

	opencv_type_enum! { crate::cudalegacy::NppStInterpMode { nppStSupersample, nppStBicubic } }

	pub type NCVDebugOutputHandler = Option<unsafe extern "C" fn(*const c_char) -> ()>;
	pub type NCVStatus = crate::cudalegacy::Ncv32u;
	pub type Ncv16s = i16;
	pub type Ncv16u = u16;
	pub type Ncv32f = f32;
	pub type Ncv32f_a = crate::cudalegacy::Ncv32f;
	pub type Ncv32s = i32;
	pub type Ncv32u = u32;
	pub type Ncv32u_a = crate::cudalegacy::Ncv32u;
	pub type Ncv64f = f64;
	pub type Ncv64s = i64;
	pub type Ncv64u = u64;
	pub type Ncv8s = i8;
	pub type Ncv8u = u8;
	pub type NcvBool = bool;
	/// Calculates optical flow for 2 images using block matching algorithm
	///
	/// ## Note
	/// This alternative version of [calc_optical_flow_bm] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn calc_optical_flow_bm_def(prev: &impl core::GpuMatTraitConst, curr: &impl core::GpuMatTraitConst, block_size: core::Size, shift_size: core::Size, max_range: core::Size, use_previous: bool, velx: &mut impl core::GpuMatTrait, vely: &mut impl core::GpuMatTrait, buf: &mut impl core::GpuMatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_calcOpticalFlowBM_const_GpuMatR_const_GpuMatR_Size_Size_Size_bool_GpuMatR_GpuMatR_GpuMatR(prev.as_raw_GpuMat(), curr.as_raw_GpuMat(), &block_size, &shift_size, &max_range, use_previous, velx.as_raw_mut_GpuMat(), vely.as_raw_mut_GpuMat(), buf.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Calculates optical flow for 2 images using block matching algorithm
	///
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn calc_optical_flow_bm(prev: &impl core::GpuMatTraitConst, curr: &impl core::GpuMatTraitConst, block_size: core::Size, shift_size: core::Size, max_range: core::Size, use_previous: bool, velx: &mut impl core::GpuMatTrait, vely: &mut impl core::GpuMatTrait, buf: &mut impl core::GpuMatTrait, stream: &mut impl core::StreamTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_calcOpticalFlowBM_const_GpuMatR_const_GpuMatR_Size_Size_Size_bool_GpuMatR_GpuMatR_GpuMatR_StreamR(prev.as_raw_GpuMat(), curr.as_raw_GpuMat(), &block_size, &shift_size, &max_range, use_previous, velx.as_raw_mut_GpuMat(), vely.as_raw_mut_GpuMat(), buf.as_raw_mut_GpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// compute mask for Generalized Flood fill componetns labeling.
	///
	/// ## Note
	/// This alternative version of [connectivity_mask] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn connectivity_mask_def(image: &impl core::GpuMatTraitConst, mask: &mut impl core::GpuMatTrait, lo: core::Scalar, hi: core::Scalar) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_connectivityMask_const_GpuMatR_GpuMatR_const_ScalarR_const_ScalarR(image.as_raw_GpuMat(), mask.as_raw_mut_GpuMat(), &lo, &hi, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// compute mask for Generalized Flood fill componetns labeling.
	///
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn connectivity_mask(image: &impl core::GpuMatTraitConst, mask: &mut impl core::GpuMatTrait, lo: core::Scalar, hi: core::Scalar, stream: &mut impl core::StreamTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_connectivityMask_const_GpuMatR_GpuMatR_const_ScalarR_const_ScalarR_StreamR(image.as_raw_GpuMat(), mask.as_raw_mut_GpuMat(), &lo, &hi, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Creates FGD Background Subtractor
	///
	/// ## Parameters
	/// * params: Algorithm's parameters. See [FGD2003](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_FGD2003) for explanation.
	///
	/// ## Note
	/// This alternative version of [create_background_subtractor_fgd] function uses the following default values for its arguments:
	/// * params: FGDParams()
	#[inline]
	pub fn create_background_subtractor_fgd_def() -> Result<core::Ptr<crate::cudalegacy::CUDA_BackgroundSubtractorFGD>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createBackgroundSubtractorFGD(ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudalegacy::CUDA_BackgroundSubtractorFGD>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates FGD Background Subtractor
	///
	/// ## Parameters
	/// * params: Algorithm's parameters. See [FGD2003](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_FGD2003) for explanation.
	///
	/// ## C++ default parameters
	/// * params: FGDParams()
	#[inline]
	pub fn create_background_subtractor_fgd(params: &impl crate::cudalegacy::CUDA_FGDParamsTraitConst) -> Result<core::Ptr<crate::cudalegacy::CUDA_BackgroundSubtractorFGD>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createBackgroundSubtractorFGD_const_FGDParamsR(params.as_raw_CUDA_FGDParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudalegacy::CUDA_BackgroundSubtractorFGD>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates GMG Background Subtractor
	///
	/// ## Parameters
	/// * initializationFrames: Number of frames of video to use to initialize histograms.
	/// * decisionThreshold: Value above which pixel is determined to be FG.
	///
	/// ## Note
	/// This alternative version of [create_background_subtractor_gmg] function uses the following default values for its arguments:
	/// * initialization_frames: 120
	/// * decision_threshold: 0.8
	#[inline]
	pub fn create_background_subtractor_gmg_def() -> Result<core::Ptr<crate::cudalegacy::CUDA_BackgroundSubtractorGMG>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createBackgroundSubtractorGMG(ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudalegacy::CUDA_BackgroundSubtractorGMG>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Creates GMG Background Subtractor
	///
	/// ## Parameters
	/// * initializationFrames: Number of frames of video to use to initialize histograms.
	/// * decisionThreshold: Value above which pixel is determined to be FG.
	///
	/// ## C++ default parameters
	/// * initialization_frames: 120
	/// * decision_threshold: 0.8
	#[inline]
	pub fn create_background_subtractor_gmg(initialization_frames: i32, decision_threshold: f64) -> Result<core::Ptr<crate::cudalegacy::CUDA_BackgroundSubtractorGMG>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createBackgroundSubtractorGMG_int_double(initialization_frames, decision_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudalegacy::CUDA_BackgroundSubtractorGMG>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [create_image_pyramid] function uses the following default values for its arguments:
	/// * n_layers: -1
	/// * stream: Stream::Null()
	#[inline]
	pub fn create_image_pyramid_def(img: &impl ToInputArray) -> Result<core::Ptr<crate::cudalegacy::CUDA_ImagePyramid>> {
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createImagePyramid_const__InputArrayR(img.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudalegacy::CUDA_ImagePyramid>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * n_layers: -1
	/// * stream: Stream::Null()
	#[inline]
	pub fn create_image_pyramid(img: &impl ToInputArray, n_layers: i32, stream: &mut impl core::StreamTrait) -> Result<core::Ptr<crate::cudalegacy::CUDA_ImagePyramid>> {
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createImagePyramid_const__InputArrayR_int_StreamR(img.as_raw__InputArray(), n_layers, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudalegacy::CUDA_ImagePyramid>::opencv_from_extern(ret) };
		Ok(ret)
	}

	#[inline]
	pub fn create_optical_flow_needle_map(u: &impl core::GpuMatTraitConst, v: &impl core::GpuMatTraitConst, vertex: &mut impl core::GpuMatTrait, colors: &mut impl core::GpuMatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_createOpticalFlowNeedleMap_const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR(u.as_raw_GpuMat(), v.as_raw_GpuMat(), vertex.as_raw_mut_GpuMat(), colors.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// performs labeling via graph cuts of a 2D regular 4-connected graph.
	///
	/// ## Note
	/// This alternative version of [graphcut] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn graphcut_def(terminals: &mut impl core::GpuMatTrait, left_transp: &mut impl core::GpuMatTrait, right_transp: &mut impl core::GpuMatTrait, top: &mut impl core::GpuMatTrait, bottom: &mut impl core::GpuMatTrait, labels: &mut impl core::GpuMatTrait, buf: &mut impl core::GpuMatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_graphcut_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR(terminals.as_raw_mut_GpuMat(), left_transp.as_raw_mut_GpuMat(), right_transp.as_raw_mut_GpuMat(), top.as_raw_mut_GpuMat(), bottom.as_raw_mut_GpuMat(), labels.as_raw_mut_GpuMat(), buf.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// performs labeling via graph cuts of a 2D regular 8-connected graph.
	///
	/// ## Note
	/// This alternative version of [graphcut_1] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn graphcut_1_def(terminals: &mut impl core::GpuMatTrait, left_transp: &mut impl core::GpuMatTrait, right_transp: &mut impl core::GpuMatTrait, top: &mut impl core::GpuMatTrait, top_left: &mut impl core::GpuMatTrait, top_right: &mut impl core::GpuMatTrait, bottom: &mut impl core::GpuMatTrait, bottom_left: &mut impl core::GpuMatTrait, bottom_right: &mut impl core::GpuMatTrait, labels: &mut impl core::GpuMatTrait, buf: &mut impl core::GpuMatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_graphcut_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR(terminals.as_raw_mut_GpuMat(), left_transp.as_raw_mut_GpuMat(), right_transp.as_raw_mut_GpuMat(), top.as_raw_mut_GpuMat(), top_left.as_raw_mut_GpuMat(), top_right.as_raw_mut_GpuMat(), bottom.as_raw_mut_GpuMat(), bottom_left.as_raw_mut_GpuMat(), bottom_right.as_raw_mut_GpuMat(), labels.as_raw_mut_GpuMat(), buf.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// performs labeling via graph cuts of a 2D regular 8-connected graph.
	///
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn graphcut_1(terminals: &mut impl core::GpuMatTrait, left_transp: &mut impl core::GpuMatTrait, right_transp: &mut impl core::GpuMatTrait, top: &mut impl core::GpuMatTrait, top_left: &mut impl core::GpuMatTrait, top_right: &mut impl core::GpuMatTrait, bottom: &mut impl core::GpuMatTrait, bottom_left: &mut impl core::GpuMatTrait, bottom_right: &mut impl core::GpuMatTrait, labels: &mut impl core::GpuMatTrait, buf: &mut impl core::GpuMatTrait, stream: &mut impl core::StreamTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_graphcut_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_StreamR(terminals.as_raw_mut_GpuMat(), left_transp.as_raw_mut_GpuMat(), right_transp.as_raw_mut_GpuMat(), top.as_raw_mut_GpuMat(), top_left.as_raw_mut_GpuMat(), top_right.as_raw_mut_GpuMat(), bottom.as_raw_mut_GpuMat(), bottom_left.as_raw_mut_GpuMat(), bottom_right.as_raw_mut_GpuMat(), labels.as_raw_mut_GpuMat(), buf.as_raw_mut_GpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// performs labeling via graph cuts of a 2D regular 4-connected graph.
	///
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn graphcut(terminals: &mut impl core::GpuMatTrait, left_transp: &mut impl core::GpuMatTrait, right_transp: &mut impl core::GpuMatTrait, top: &mut impl core::GpuMatTrait, bottom: &mut impl core::GpuMatTrait, labels: &mut impl core::GpuMatTrait, buf: &mut impl core::GpuMatTrait, stream: &mut impl core::StreamTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_graphcut_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_GpuMatR_StreamR(terminals.as_raw_mut_GpuMat(), left_transp.as_raw_mut_GpuMat(), right_transp.as_raw_mut_GpuMat(), top.as_raw_mut_GpuMat(), bottom.as_raw_mut_GpuMat(), labels.as_raw_mut_GpuMat(), buf.as_raw_mut_GpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Interpolates frames (images) using provided optical flow (displacement field).
	///
	/// ## Parameters
	/// * frame0: First frame (32-bit floating point images, single channel).
	/// * frame1: Second frame. Must have the same type and size as frame0 .
	/// * fu: Forward horizontal displacement.
	/// * fv: Forward vertical displacement.
	/// * bu: Backward horizontal displacement.
	/// * bv: Backward vertical displacement.
	/// * pos: New frame position.
	/// * newFrame: Output image.
	/// * buf: Temporary buffer, will have width x 6\*height size, CV_32FC1 type and contain 6
	/// GpuMat: occlusion masks for first frame, occlusion masks for second, interpolated forward
	/// horizontal flow, interpolated forward vertical flow, interpolated backward horizontal flow,
	/// interpolated backward vertical flow.
	/// * stream: Stream for the asynchronous version.
	///
	/// ## Note
	/// This alternative version of [interpolate_frames] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn interpolate_frames_def(frame0: &impl core::GpuMatTraitConst, frame1: &impl core::GpuMatTraitConst, fu: &impl core::GpuMatTraitConst, fv: &impl core::GpuMatTraitConst, bu: &impl core::GpuMatTraitConst, bv: &impl core::GpuMatTraitConst, pos: f32, new_frame: &mut impl core::GpuMatTrait, buf: &mut impl core::GpuMatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_interpolateFrames_const_GpuMatR_const_GpuMatR_const_GpuMatR_const_GpuMatR_const_GpuMatR_const_GpuMatR_float_GpuMatR_GpuMatR(frame0.as_raw_GpuMat(), frame1.as_raw_GpuMat(), fu.as_raw_GpuMat(), fv.as_raw_GpuMat(), bu.as_raw_GpuMat(), bv.as_raw_GpuMat(), pos, new_frame.as_raw_mut_GpuMat(), buf.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Interpolates frames (images) using provided optical flow (displacement field).
	///
	/// ## Parameters
	/// * frame0: First frame (32-bit floating point images, single channel).
	/// * frame1: Second frame. Must have the same type and size as frame0 .
	/// * fu: Forward horizontal displacement.
	/// * fv: Forward vertical displacement.
	/// * bu: Backward horizontal displacement.
	/// * bv: Backward vertical displacement.
	/// * pos: New frame position.
	/// * newFrame: Output image.
	/// * buf: Temporary buffer, will have width x 6\*height size, CV_32FC1 type and contain 6
	/// GpuMat: occlusion masks for first frame, occlusion masks for second, interpolated forward
	/// horizontal flow, interpolated forward vertical flow, interpolated backward horizontal flow,
	/// interpolated backward vertical flow.
	/// * stream: Stream for the asynchronous version.
	///
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn interpolate_frames(frame0: &impl core::GpuMatTraitConst, frame1: &impl core::GpuMatTraitConst, fu: &impl core::GpuMatTraitConst, fv: &impl core::GpuMatTraitConst, bu: &impl core::GpuMatTraitConst, bv: &impl core::GpuMatTraitConst, pos: f32, new_frame: &mut impl core::GpuMatTrait, buf: &mut impl core::GpuMatTrait, stream: &mut impl core::StreamTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_interpolateFrames_const_GpuMatR_const_GpuMatR_const_GpuMatR_const_GpuMatR_const_GpuMatR_const_GpuMatR_float_GpuMatR_GpuMatR_StreamR(frame0.as_raw_GpuMat(), frame1.as_raw_GpuMat(), fu.as_raw_GpuMat(), fv.as_raw_GpuMat(), bu.as_raw_GpuMat(), bv.as_raw_GpuMat(), pos, new_frame.as_raw_mut_GpuMat(), buf.as_raw_mut_GpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// performs connected componnents labeling.
	///
	/// ## Note
	/// This alternative version of [label_components] function uses the following default values for its arguments:
	/// * flags: 0
	/// * stream: Stream::Null()
	#[inline]
	pub fn label_components_def(mask: &impl core::GpuMatTraitConst, components: &mut impl core::GpuMatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_labelComponents_const_GpuMatR_GpuMatR(mask.as_raw_GpuMat(), components.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// performs connected componnents labeling.
	///
	/// ## C++ default parameters
	/// * flags: 0
	/// * stream: Stream::Null()
	#[inline]
	pub fn label_components(mask: &impl core::GpuMatTraitConst, components: &mut impl core::GpuMatTrait, flags: i32, stream: &mut impl core::StreamTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_labelComponents_const_GpuMatR_GpuMatR_int_StreamR(mask.as_raw_GpuMat(), components.as_raw_mut_GpuMat(), flags, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [project_points] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn project_points_def(src: &impl core::GpuMatTraitConst, rvec: &impl core::MatTraitConst, tvec: &impl core::MatTraitConst, camera_mat: &impl core::MatTraitConst, dist_coef: &impl core::MatTraitConst, dst: &mut impl core::GpuMatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_projectPoints_const_GpuMatR_const_MatR_const_MatR_const_MatR_const_MatR_GpuMatR(src.as_raw_GpuMat(), rvec.as_raw_Mat(), tvec.as_raw_Mat(), camera_mat.as_raw_Mat(), dist_coef.as_raw_Mat(), dst.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn project_points(src: &impl core::GpuMatTraitConst, rvec: &impl core::MatTraitConst, tvec: &impl core::MatTraitConst, camera_mat: &impl core::MatTraitConst, dist_coef: &impl core::MatTraitConst, dst: &mut impl core::GpuMatTrait, stream: &mut impl core::StreamTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_projectPoints_const_GpuMatR_const_MatR_const_MatR_const_MatR_const_MatR_GpuMatR_StreamR(src.as_raw_GpuMat(), rvec.as_raw_Mat(), tvec.as_raw_Mat(), camera_mat.as_raw_Mat(), dist_coef.as_raw_Mat(), dst.as_raw_mut_GpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds the object pose from 3D-2D point correspondences.
	///
	/// ## Parameters
	/// * object: Single-row matrix of object points.
	/// * image: Single-row matrix of image points.
	/// * camera_mat: 3x3 matrix of intrinsic camera parameters.
	/// * dist_coef: Distortion coefficients. See undistortPoints for details.
	/// * rvec: Output 3D rotation vector.
	/// * tvec: Output 3D translation vector.
	/// * use_extrinsic_guess: Flag to indicate that the function must use rvec and tvec as an
	/// initial transformation guess. It is not supported for now.
	/// * num_iters: Maximum number of RANSAC iterations.
	/// * max_dist: Euclidean distance threshold to detect whether point is inlier or not.
	/// * min_inlier_count: Flag to indicate that the function must stop if greater or equal number
	/// of inliers is achieved. It is not supported for now.
	/// * inliers: Output vector of inlier indices.
	///
	/// ## Note
	/// This alternative version of [solve_pnp_ransac] function uses the following default values for its arguments:
	/// * use_extrinsic_guess: false
	/// * num_iters: 100
	/// * max_dist: 8.0
	/// * min_inlier_count: 100
	/// * inliers: NULL
	#[inline]
	pub fn solve_pnp_ransac_def(object: &impl core::MatTraitConst, image: &impl core::MatTraitConst, camera_mat: &impl core::MatTraitConst, dist_coef: &impl core::MatTraitConst, rvec: &mut impl core::MatTrait, tvec: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_solvePnPRansac_const_MatR_const_MatR_const_MatR_const_MatR_MatR_MatR(object.as_raw_Mat(), image.as_raw_Mat(), camera_mat.as_raw_Mat(), dist_coef.as_raw_Mat(), rvec.as_raw_mut_Mat(), tvec.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Finds the object pose from 3D-2D point correspondences.
	///
	/// ## Parameters
	/// * object: Single-row matrix of object points.
	/// * image: Single-row matrix of image points.
	/// * camera_mat: 3x3 matrix of intrinsic camera parameters.
	/// * dist_coef: Distortion coefficients. See undistortPoints for details.
	/// * rvec: Output 3D rotation vector.
	/// * tvec: Output 3D translation vector.
	/// * use_extrinsic_guess: Flag to indicate that the function must use rvec and tvec as an
	/// initial transformation guess. It is not supported for now.
	/// * num_iters: Maximum number of RANSAC iterations.
	/// * max_dist: Euclidean distance threshold to detect whether point is inlier or not.
	/// * min_inlier_count: Flag to indicate that the function must stop if greater or equal number
	/// of inliers is achieved. It is not supported for now.
	/// * inliers: Output vector of inlier indices.
	///
	/// ## C++ default parameters
	/// * use_extrinsic_guess: false
	/// * num_iters: 100
	/// * max_dist: 8.0
	/// * min_inlier_count: 100
	/// * inliers: NULL
	#[inline]
	pub fn solve_pnp_ransac(object: &impl core::MatTraitConst, image: &impl core::MatTraitConst, camera_mat: &impl core::MatTraitConst, dist_coef: &impl core::MatTraitConst, rvec: &mut impl core::MatTrait, tvec: &mut impl core::MatTrait, use_extrinsic_guess: bool, num_iters: i32, max_dist: f32, min_inlier_count: i32, inliers: &mut core::Vector<i32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_solvePnPRansac_const_MatR_const_MatR_const_MatR_const_MatR_MatR_MatR_bool_int_float_int_vectorLintGX(object.as_raw_Mat(), image.as_raw_Mat(), camera_mat.as_raw_Mat(), dist_coef.as_raw_Mat(), rvec.as_raw_mut_Mat(), tvec.as_raw_mut_Mat(), use_extrinsic_guess, num_iters, max_dist, min_inlier_count, inliers.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [transform_points] function uses the following default values for its arguments:
	/// * stream: Stream::Null()
	#[inline]
	pub fn transform_points_def(src: &impl core::GpuMatTraitConst, rvec: &impl core::MatTraitConst, tvec: &impl core::MatTraitConst, dst: &mut impl core::GpuMatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_transformPoints_const_GpuMatR_const_MatR_const_MatR_GpuMatR(src.as_raw_GpuMat(), rvec.as_raw_Mat(), tvec.as_raw_Mat(), dst.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	pub fn transform_points(src: &impl core::GpuMatTraitConst, rvec: &impl core::MatTraitConst, tvec: &impl core::MatTraitConst, dst: &mut impl core::GpuMatTrait, stream: &mut impl core::StreamTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_transformPoints_const_GpuMatR_const_MatR_const_MatR_GpuMatR_StreamR(src.as_raw_GpuMat(), rvec.as_raw_Mat(), tvec.as_raw_Mat(), dst.as_raw_mut_GpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// The class discriminates between foreground and background pixels by building and maintaining a model
	/// of the background.
	///
	/// Any pixel which does not fit this model is then deemed to be foreground. The class implements
	/// algorithm described in [FGD2003](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_FGD2003) .
	/// ## See also
	/// BackgroundSubtractor
	pub struct CUDA_BackgroundSubtractorFGD {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { CUDA_BackgroundSubtractorFGD }

	impl Drop for CUDA_BackgroundSubtractorFGD {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_BackgroundSubtractorFGD_delete(self.as_raw_mut_CUDA_BackgroundSubtractorFGD()) };
		}
	}

	unsafe impl Send for CUDA_BackgroundSubtractorFGD {}

	/// Constant methods for [crate::cudalegacy::CUDA_BackgroundSubtractorFGD]
	pub trait CUDA_BackgroundSubtractorFGDTraitConst: crate::video::BackgroundSubtractorTraitConst {
		fn as_raw_CUDA_BackgroundSubtractorFGD(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::cudalegacy::CUDA_BackgroundSubtractorFGD]
	pub trait CUDA_BackgroundSubtractorFGDTrait: crate::cudalegacy::CUDA_BackgroundSubtractorFGDTraitConst + crate::video::BackgroundSubtractorTrait {
		fn as_raw_mut_CUDA_BackgroundSubtractorFGD(&mut self) -> *mut c_void;

		/// Returns the output foreground regions calculated by findContours.
		///
		/// ## Parameters
		/// * foreground_regions: Output array (CPU memory).
		#[inline]
		fn get_foreground_regions(&mut self, foreground_regions: &mut impl ToOutputArray) -> Result<()> {
			output_array_arg!(foreground_regions);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorFGD_getForegroundRegions_const__OutputArrayR(self.as_raw_mut_CUDA_BackgroundSubtractorFGD(), foreground_regions.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for CUDA_BackgroundSubtractorFGD {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_BackgroundSubtractorFGD")
				.finish()
		}
	}

	boxed_cast_base! { CUDA_BackgroundSubtractorFGD, core::Algorithm, cv_cuda_BackgroundSubtractorFGD_to_Algorithm }

	boxed_cast_base! { CUDA_BackgroundSubtractorFGD, crate::video::BackgroundSubtractor, cv_cuda_BackgroundSubtractorFGD_to_BackgroundSubtractor }

	impl core::AlgorithmTraitConst for CUDA_BackgroundSubtractorFGD {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for CUDA_BackgroundSubtractorFGD {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_BackgroundSubtractorFGD, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::video::BackgroundSubtractorTraitConst for CUDA_BackgroundSubtractorFGD {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::video::BackgroundSubtractorTrait for CUDA_BackgroundSubtractorFGD {
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_BackgroundSubtractorFGD, crate::video::BackgroundSubtractorTraitConst, as_raw_BackgroundSubtractor, crate::video::BackgroundSubtractorTrait, as_raw_mut_BackgroundSubtractor }

	impl crate::cudalegacy::CUDA_BackgroundSubtractorFGDTraitConst for CUDA_BackgroundSubtractorFGD {
		#[inline] fn as_raw_CUDA_BackgroundSubtractorFGD(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::cudalegacy::CUDA_BackgroundSubtractorFGDTrait for CUDA_BackgroundSubtractorFGD {
		#[inline] fn as_raw_mut_CUDA_BackgroundSubtractorFGD(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_BackgroundSubtractorFGD, crate::cudalegacy::CUDA_BackgroundSubtractorFGDTraitConst, as_raw_CUDA_BackgroundSubtractorFGD, crate::cudalegacy::CUDA_BackgroundSubtractorFGDTrait, as_raw_mut_CUDA_BackgroundSubtractorFGD }

	/// Background/Foreground Segmentation Algorithm.
	///
	/// The class discriminates between foreground and background pixels by building and maintaining a model
	/// of the background. Any pixel which does not fit this model is then deemed to be foreground. The
	/// class implements algorithm described in [Gold2012](https://docs.opencv.org/4.13.0/d0/de3/citelist.html#CITEREF_Gold2012) .
	pub struct CUDA_BackgroundSubtractorGMG {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { CUDA_BackgroundSubtractorGMG }

	impl Drop for CUDA_BackgroundSubtractorGMG {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_delete(self.as_raw_mut_CUDA_BackgroundSubtractorGMG()) };
		}
	}

	unsafe impl Send for CUDA_BackgroundSubtractorGMG {}

	/// Constant methods for [crate::cudalegacy::CUDA_BackgroundSubtractorGMG]
	pub trait CUDA_BackgroundSubtractorGMGTraitConst: crate::video::BackgroundSubtractorTraitConst {
		fn as_raw_CUDA_BackgroundSubtractorGMG(&self) -> *const c_void;

		#[inline]
		fn get_max_features(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_getMaxFeatures_const(self.as_raw_CUDA_BackgroundSubtractorGMG(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_default_learning_rate(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_getDefaultLearningRate_const(self.as_raw_CUDA_BackgroundSubtractorGMG(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_num_frames(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_getNumFrames_const(self.as_raw_CUDA_BackgroundSubtractorGMG(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_quantization_levels(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_getQuantizationLevels_const(self.as_raw_CUDA_BackgroundSubtractorGMG(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_background_prior(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_getBackgroundPrior_const(self.as_raw_CUDA_BackgroundSubtractorGMG(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_smoothing_radius(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_getSmoothingRadius_const(self.as_raw_CUDA_BackgroundSubtractorGMG(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_decision_threshold(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_getDecisionThreshold_const(self.as_raw_CUDA_BackgroundSubtractorGMG(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_update_background_model(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_getUpdateBackgroundModel_const(self.as_raw_CUDA_BackgroundSubtractorGMG(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_min_val(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_getMinVal_const(self.as_raw_CUDA_BackgroundSubtractorGMG(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn get_max_val(&self) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_getMaxVal_const(self.as_raw_CUDA_BackgroundSubtractorGMG(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::cudalegacy::CUDA_BackgroundSubtractorGMG]
	pub trait CUDA_BackgroundSubtractorGMGTrait: crate::cudalegacy::CUDA_BackgroundSubtractorGMGTraitConst + crate::video::BackgroundSubtractorTrait {
		fn as_raw_mut_CUDA_BackgroundSubtractorGMG(&mut self) -> *mut c_void;

		#[inline]
		fn apply(&mut self, image: &impl ToInputArray, fgmask: &mut impl ToOutputArray, learning_rate: f64, stream: &mut impl core::StreamTrait) -> Result<()> {
			input_array_arg!(image);
			output_array_arg!(fgmask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_apply_const__InputArrayR_const__OutputArrayR_double_StreamR(self.as_raw_mut_CUDA_BackgroundSubtractorGMG(), image.as_raw__InputArray(), fgmask.as_raw__OutputArray(), learning_rate, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn apply_1(&mut self, image: &impl ToInputArray, known_foreground_mask: &impl ToInputArray, fgmask: &mut impl ToOutputArray, learning_rate: f64, stream: &mut impl core::StreamTrait) -> Result<()> {
			input_array_arg!(image);
			input_array_arg!(known_foreground_mask);
			output_array_arg!(fgmask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_apply_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_StreamR(self.as_raw_mut_CUDA_BackgroundSubtractorGMG(), image.as_raw__InputArray(), known_foreground_mask.as_raw__InputArray(), fgmask.as_raw__OutputArray(), learning_rate, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_max_features(&mut self, max_features: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_setMaxFeatures_int(self.as_raw_mut_CUDA_BackgroundSubtractorGMG(), max_features, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_default_learning_rate(&mut self, lr: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_setDefaultLearningRate_double(self.as_raw_mut_CUDA_BackgroundSubtractorGMG(), lr, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_num_frames(&mut self, nframes: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_setNumFrames_int(self.as_raw_mut_CUDA_BackgroundSubtractorGMG(), nframes, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_quantization_levels(&mut self, nlevels: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_setQuantizationLevels_int(self.as_raw_mut_CUDA_BackgroundSubtractorGMG(), nlevels, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_background_prior(&mut self, bgprior: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_setBackgroundPrior_double(self.as_raw_mut_CUDA_BackgroundSubtractorGMG(), bgprior, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_smoothing_radius(&mut self, radius: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_setSmoothingRadius_int(self.as_raw_mut_CUDA_BackgroundSubtractorGMG(), radius, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_decision_threshold(&mut self, thresh: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_setDecisionThreshold_double(self.as_raw_mut_CUDA_BackgroundSubtractorGMG(), thresh, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_update_background_model(&mut self, update: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_setUpdateBackgroundModel_bool(self.as_raw_mut_CUDA_BackgroundSubtractorGMG(), update, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_min_val(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_setMinVal_double(self.as_raw_mut_CUDA_BackgroundSubtractorGMG(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		#[inline]
		fn set_max_val(&mut self, val: f64) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_BackgroundSubtractorGMG_setMaxVal_double(self.as_raw_mut_CUDA_BackgroundSubtractorGMG(), val, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl std::fmt::Debug for CUDA_BackgroundSubtractorGMG {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_BackgroundSubtractorGMG")
				.finish()
		}
	}

	boxed_cast_base! { CUDA_BackgroundSubtractorGMG, core::Algorithm, cv_cuda_BackgroundSubtractorGMG_to_Algorithm }

	boxed_cast_base! { CUDA_BackgroundSubtractorGMG, crate::video::BackgroundSubtractor, cv_cuda_BackgroundSubtractorGMG_to_BackgroundSubtractor }

	impl core::AlgorithmTraitConst for CUDA_BackgroundSubtractorGMG {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for CUDA_BackgroundSubtractorGMG {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_BackgroundSubtractorGMG, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::video::BackgroundSubtractorTraitConst for CUDA_BackgroundSubtractorGMG {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::video::BackgroundSubtractorTrait for CUDA_BackgroundSubtractorGMG {
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_BackgroundSubtractorGMG, crate::video::BackgroundSubtractorTraitConst, as_raw_BackgroundSubtractor, crate::video::BackgroundSubtractorTrait, as_raw_mut_BackgroundSubtractor }

	impl crate::cudalegacy::CUDA_BackgroundSubtractorGMGTraitConst for CUDA_BackgroundSubtractorGMG {
		#[inline] fn as_raw_CUDA_BackgroundSubtractorGMG(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::cudalegacy::CUDA_BackgroundSubtractorGMGTrait for CUDA_BackgroundSubtractorGMG {
		#[inline] fn as_raw_mut_CUDA_BackgroundSubtractorGMG(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_BackgroundSubtractorGMG, crate::cudalegacy::CUDA_BackgroundSubtractorGMGTraitConst, as_raw_CUDA_BackgroundSubtractorGMG, crate::cudalegacy::CUDA_BackgroundSubtractorGMGTrait, as_raw_mut_CUDA_BackgroundSubtractorGMG }

	pub struct CUDA_FGDParams {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { CUDA_FGDParams }

	impl Drop for CUDA_FGDParams {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_FGDParams_delete(self.as_raw_mut_CUDA_FGDParams()) };
		}
	}

	unsafe impl Send for CUDA_FGDParams {}

	impl CUDA_FGDParams {
		/// default Params
		#[inline]
		pub fn default() -> Result<crate::cudalegacy::CUDA_FGDParams> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FGDParams_FGDParams(ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::cudalegacy::CUDA_FGDParams::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	/// Constant methods for [crate::cudalegacy::CUDA_FGDParams]
	pub trait CUDA_FGDParamsTraitConst {
		fn as_raw_CUDA_FGDParams(&self) -> *const c_void;

		/// Quantized levels per 'color' component. Power of two, typically 32, 64 or 128.
		#[inline]
		fn lc(&self) -> i32 {
			let ret = unsafe { sys::cv_cuda_FGDParams_propLc_const(self.as_raw_CUDA_FGDParams()) };
			ret
		}

		/// Number of color vectors used to model normal background color variation at a given pixel.
		#[inline]
		fn n1c(&self) -> i32 {
			let ret = unsafe { sys::cv_cuda_FGDParams_propN1c_const(self.as_raw_CUDA_FGDParams()) };
			ret
		}

		/// Number of color vectors retained at given pixel.  Must be > N1c, typically ~ 5/3 of N1c.
		/// Used to allow the first N1c vectors to adapt over time to changing background.
		#[inline]
		fn n2c(&self) -> i32 {
			let ret = unsafe { sys::cv_cuda_FGDParams_propN2c_const(self.as_raw_CUDA_FGDParams()) };
			ret
		}

		/// Quantized levels per 'color co-occurrence' component.  Power of two, typically 16, 32 or 64.
		#[inline]
		fn lcc(&self) -> i32 {
			let ret = unsafe { sys::cv_cuda_FGDParams_propLcc_const(self.as_raw_CUDA_FGDParams()) };
			ret
		}

		/// Number of color co-occurrence vectors used to model normal background color variation at a given pixel.
		#[inline]
		fn n1cc(&self) -> i32 {
			let ret = unsafe { sys::cv_cuda_FGDParams_propN1cc_const(self.as_raw_CUDA_FGDParams()) };
			ret
		}

		/// Number of color co-occurrence vectors retained at given pixel.  Must be > N1cc, typically ~ 5/3 of N1cc.
		/// Used to allow the first N1cc vectors to adapt over time to changing background.
		#[inline]
		fn n2cc(&self) -> i32 {
			let ret = unsafe { sys::cv_cuda_FGDParams_propN2cc_const(self.as_raw_CUDA_FGDParams()) };
			ret
		}

		/// If TRUE we ignore holes within foreground blobs. Defaults to TRUE.
		#[inline]
		fn is_obj_without_holes(&self) -> bool {
			let ret = unsafe { sys::cv_cuda_FGDParams_propIs_obj_without_holes_const(self.as_raw_CUDA_FGDParams()) };
			ret
		}

		/// Number of erode-dilate-erode foreground-blob cleanup iterations.
		/// These erase one-pixel junk blobs and merge almost-touching blobs. Default value is 1.
		#[inline]
		fn perform_morphing(&self) -> i32 {
			let ret = unsafe { sys::cv_cuda_FGDParams_propPerform_morphing_const(self.as_raw_CUDA_FGDParams()) };
			ret
		}

		/// How quickly we forget old background pixel values seen. Typically set to 0.1.
		#[inline]
		fn alpha1(&self) -> f32 {
			let ret = unsafe { sys::cv_cuda_FGDParams_propAlpha1_const(self.as_raw_CUDA_FGDParams()) };
			ret
		}

		/// "Controls speed of feature learning". Depends on T. Typical value circa 0.005.
		#[inline]
		fn alpha2(&self) -> f32 {
			let ret = unsafe { sys::cv_cuda_FGDParams_propAlpha2_const(self.as_raw_CUDA_FGDParams()) };
			ret
		}

		/// Alternate to alpha2, used (e.g.) for quicker initial convergence. Typical value 0.1.
		#[inline]
		fn alpha3(&self) -> f32 {
			let ret = unsafe { sys::cv_cuda_FGDParams_propAlpha3_const(self.as_raw_CUDA_FGDParams()) };
			ret
		}

		/// Affects color and color co-occurrence quantization, typically set to 2.
		#[inline]
		fn delta(&self) -> f32 {
			let ret = unsafe { sys::cv_cuda_FGDParams_propDelta_const(self.as_raw_CUDA_FGDParams()) };
			ret
		}

		/// A percentage value which determines when new features can be recognized as new background. (Typically 0.9).
		#[inline]
		fn t(&self) -> f32 {
			let ret = unsafe { sys::cv_cuda_FGDParams_propT_const(self.as_raw_CUDA_FGDParams()) };
			ret
		}

		/// Discard foreground blobs whose bounding box is smaller than this threshold.
		#[inline]
		fn min_area(&self) -> f32 {
			let ret = unsafe { sys::cv_cuda_FGDParams_propMinArea_const(self.as_raw_CUDA_FGDParams()) };
			ret
		}

	}

	/// Mutable methods for [crate::cudalegacy::CUDA_FGDParams]
	pub trait CUDA_FGDParamsTrait: crate::cudalegacy::CUDA_FGDParamsTraitConst {
		fn as_raw_mut_CUDA_FGDParams(&mut self) -> *mut c_void;

		/// Quantized levels per 'color' component. Power of two, typically 32, 64 or 128.
		#[inline]
		fn set_lc(&mut self, val: i32) {
			let ret = unsafe { sys::cv_cuda_FGDParams_propLc_const_int(self.as_raw_mut_CUDA_FGDParams(), val) };
			ret
		}

		/// Number of color vectors used to model normal background color variation at a given pixel.
		#[inline]
		fn set_n1c(&mut self, val: i32) {
			let ret = unsafe { sys::cv_cuda_FGDParams_propN1c_const_int(self.as_raw_mut_CUDA_FGDParams(), val) };
			ret
		}

		/// Number of color vectors retained at given pixel.  Must be > N1c, typically ~ 5/3 of N1c.
		/// Used to allow the first N1c vectors to adapt over time to changing background.
		#[inline]
		fn set_n2c(&mut self, val: i32) {
			let ret = unsafe { sys::cv_cuda_FGDParams_propN2c_const_int(self.as_raw_mut_CUDA_FGDParams(), val) };
			ret
		}

		/// Quantized levels per 'color co-occurrence' component.  Power of two, typically 16, 32 or 64.
		#[inline]
		fn set_lcc(&mut self, val: i32) {
			let ret = unsafe { sys::cv_cuda_FGDParams_propLcc_const_int(self.as_raw_mut_CUDA_FGDParams(), val) };
			ret
		}

		/// Number of color co-occurrence vectors used to model normal background color variation at a given pixel.
		#[inline]
		fn set_n1cc(&mut self, val: i32) {
			let ret = unsafe { sys::cv_cuda_FGDParams_propN1cc_const_int(self.as_raw_mut_CUDA_FGDParams(), val) };
			ret
		}

		/// Number of color co-occurrence vectors retained at given pixel.  Must be > N1cc, typically ~ 5/3 of N1cc.
		/// Used to allow the first N1cc vectors to adapt over time to changing background.
		#[inline]
		fn set_n2cc(&mut self, val: i32) {
			let ret = unsafe { sys::cv_cuda_FGDParams_propN2cc_const_int(self.as_raw_mut_CUDA_FGDParams(), val) };
			ret
		}

		/// If TRUE we ignore holes within foreground blobs. Defaults to TRUE.
		#[inline]
		fn set_is_obj_without_holes(&mut self, val: bool) {
			let ret = unsafe { sys::cv_cuda_FGDParams_propIs_obj_without_holes_const_bool(self.as_raw_mut_CUDA_FGDParams(), val) };
			ret
		}

		/// Number of erode-dilate-erode foreground-blob cleanup iterations.
		/// These erase one-pixel junk blobs and merge almost-touching blobs. Default value is 1.
		#[inline]
		fn set_perform_morphing(&mut self, val: i32) {
			let ret = unsafe { sys::cv_cuda_FGDParams_propPerform_morphing_const_int(self.as_raw_mut_CUDA_FGDParams(), val) };
			ret
		}

		/// How quickly we forget old background pixel values seen. Typically set to 0.1.
		#[inline]
		fn set_alpha1(&mut self, val: f32) {
			let ret = unsafe { sys::cv_cuda_FGDParams_propAlpha1_const_float(self.as_raw_mut_CUDA_FGDParams(), val) };
			ret
		}

		/// "Controls speed of feature learning". Depends on T. Typical value circa 0.005.
		#[inline]
		fn set_alpha2(&mut self, val: f32) {
			let ret = unsafe { sys::cv_cuda_FGDParams_propAlpha2_const_float(self.as_raw_mut_CUDA_FGDParams(), val) };
			ret
		}

		/// Alternate to alpha2, used (e.g.) for quicker initial convergence. Typical value 0.1.
		#[inline]
		fn set_alpha3(&mut self, val: f32) {
			let ret = unsafe { sys::cv_cuda_FGDParams_propAlpha3_const_float(self.as_raw_mut_CUDA_FGDParams(), val) };
			ret
		}

		/// Affects color and color co-occurrence quantization, typically set to 2.
		#[inline]
		fn set_delta(&mut self, val: f32) {
			let ret = unsafe { sys::cv_cuda_FGDParams_propDelta_const_float(self.as_raw_mut_CUDA_FGDParams(), val) };
			ret
		}

		/// A percentage value which determines when new features can be recognized as new background. (Typically 0.9).
		#[inline]
		fn set_t(&mut self, val: f32) {
			let ret = unsafe { sys::cv_cuda_FGDParams_propT_const_float(self.as_raw_mut_CUDA_FGDParams(), val) };
			ret
		}

		/// Discard foreground blobs whose bounding box is smaller than this threshold.
		#[inline]
		fn set_min_area(&mut self, val: f32) {
			let ret = unsafe { sys::cv_cuda_FGDParams_propMinArea_const_float(self.as_raw_mut_CUDA_FGDParams(), val) };
			ret
		}

	}

	impl std::fmt::Debug for CUDA_FGDParams {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_FGDParams")
				.field("lc", &crate::cudalegacy::CUDA_FGDParamsTraitConst::lc(self))
				.field("n1c", &crate::cudalegacy::CUDA_FGDParamsTraitConst::n1c(self))
				.field("n2c", &crate::cudalegacy::CUDA_FGDParamsTraitConst::n2c(self))
				.field("lcc", &crate::cudalegacy::CUDA_FGDParamsTraitConst::lcc(self))
				.field("n1cc", &crate::cudalegacy::CUDA_FGDParamsTraitConst::n1cc(self))
				.field("n2cc", &crate::cudalegacy::CUDA_FGDParamsTraitConst::n2cc(self))
				.field("is_obj_without_holes", &crate::cudalegacy::CUDA_FGDParamsTraitConst::is_obj_without_holes(self))
				.field("perform_morphing", &crate::cudalegacy::CUDA_FGDParamsTraitConst::perform_morphing(self))
				.field("alpha1", &crate::cudalegacy::CUDA_FGDParamsTraitConst::alpha1(self))
				.field("alpha2", &crate::cudalegacy::CUDA_FGDParamsTraitConst::alpha2(self))
				.field("alpha3", &crate::cudalegacy::CUDA_FGDParamsTraitConst::alpha3(self))
				.field("delta", &crate::cudalegacy::CUDA_FGDParamsTraitConst::delta(self))
				.field("t", &crate::cudalegacy::CUDA_FGDParamsTraitConst::t(self))
				.field("min_area", &crate::cudalegacy::CUDA_FGDParamsTraitConst::min_area(self))
				.finish()
		}
	}

	impl crate::cudalegacy::CUDA_FGDParamsTraitConst for CUDA_FGDParams {
		#[inline] fn as_raw_CUDA_FGDParams(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::cudalegacy::CUDA_FGDParamsTrait for CUDA_FGDParams {
		#[inline] fn as_raw_mut_CUDA_FGDParams(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_FGDParams, crate::cudalegacy::CUDA_FGDParamsTraitConst, as_raw_CUDA_FGDParams, crate::cudalegacy::CUDA_FGDParamsTrait, as_raw_mut_CUDA_FGDParams }

	pub struct CUDA_FastOpticalFlowBM {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { CUDA_FastOpticalFlowBM }

	impl Drop for CUDA_FastOpticalFlowBM {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_FastOpticalFlowBM_delete(self.as_raw_mut_CUDA_FastOpticalFlowBM()) };
		}
	}

	unsafe impl Send for CUDA_FastOpticalFlowBM {}

	impl CUDA_FastOpticalFlowBM {
		/// Creates a default instance of the class by calling the default constructor
		#[inline]
		pub fn default() -> crate::cudalegacy::CUDA_FastOpticalFlowBM {
			let ret = unsafe { sys::cv_cuda_FastOpticalFlowBM_defaultNew_const() };
			let ret = unsafe { crate::cudalegacy::CUDA_FastOpticalFlowBM::opencv_from_extern(ret) };
			ret
		}

	}

	/// Constant methods for [crate::cudalegacy::CUDA_FastOpticalFlowBM]
	pub trait CUDA_FastOpticalFlowBMTraitConst {
		fn as_raw_CUDA_FastOpticalFlowBM(&self) -> *const c_void;

	}

	/// Mutable methods for [crate::cudalegacy::CUDA_FastOpticalFlowBM]
	pub trait CUDA_FastOpticalFlowBMTrait: crate::cudalegacy::CUDA_FastOpticalFlowBMTraitConst {
		fn as_raw_mut_CUDA_FastOpticalFlowBM(&mut self) -> *mut c_void;

		/// ## C++ default parameters
		/// * search_window: 21
		/// * block_window: 7
		/// * s: Stream::Null()
		#[inline]
		fn apply(&mut self, i0: &impl core::GpuMatTraitConst, i1: &impl core::GpuMatTraitConst, flowx: &mut impl core::GpuMatTrait, flowy: &mut impl core::GpuMatTrait, search_window: i32, block_window: i32, s: &mut impl core::StreamTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FastOpticalFlowBM_operator___const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR_int_int_StreamR(self.as_raw_mut_CUDA_FastOpticalFlowBM(), i0.as_raw_GpuMat(), i1.as_raw_GpuMat(), flowx.as_raw_mut_GpuMat(), flowy.as_raw_mut_GpuMat(), search_window, block_window, s.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// ## Note
		/// This alternative version of [CUDA_FastOpticalFlowBMTrait::apply] function uses the following default values for its arguments:
		/// * search_window: 21
		/// * block_window: 7
		/// * s: Stream::Null()
		#[inline]
		fn apply_def(&mut self, i0: &impl core::GpuMatTraitConst, i1: &impl core::GpuMatTraitConst, flowx: &mut impl core::GpuMatTrait, flowy: &mut impl core::GpuMatTrait) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_FastOpticalFlowBM_operator___const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR(self.as_raw_mut_CUDA_FastOpticalFlowBM(), i0.as_raw_GpuMat(), i1.as_raw_GpuMat(), flowx.as_raw_mut_GpuMat(), flowy.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	impl Default for CUDA_FastOpticalFlowBM {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}

	impl std::fmt::Debug for CUDA_FastOpticalFlowBM {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_FastOpticalFlowBM")
				.finish()
		}
	}

	impl crate::cudalegacy::CUDA_FastOpticalFlowBMTraitConst for CUDA_FastOpticalFlowBM {
		#[inline] fn as_raw_CUDA_FastOpticalFlowBM(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::cudalegacy::CUDA_FastOpticalFlowBMTrait for CUDA_FastOpticalFlowBM {
		#[inline] fn as_raw_mut_CUDA_FastOpticalFlowBM(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_FastOpticalFlowBM, crate::cudalegacy::CUDA_FastOpticalFlowBMTraitConst, as_raw_CUDA_FastOpticalFlowBM, crate::cudalegacy::CUDA_FastOpticalFlowBMTrait, as_raw_mut_CUDA_FastOpticalFlowBM }

	pub struct CUDA_ImagePyramid {
		ptr: *mut c_void,
	}

	opencv_type_boxed! { CUDA_ImagePyramid }

	impl Drop for CUDA_ImagePyramid {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cuda_ImagePyramid_delete(self.as_raw_mut_CUDA_ImagePyramid()) };
		}
	}

	unsafe impl Send for CUDA_ImagePyramid {}

	/// Constant methods for [crate::cudalegacy::CUDA_ImagePyramid]
	pub trait CUDA_ImagePyramidTraitConst: core::AlgorithmTraitConst {
		fn as_raw_CUDA_ImagePyramid(&self) -> *const c_void;

		/// ## C++ default parameters
		/// * stream: Stream::Null()
		#[inline]
		fn get_layer(&self, out_img: &mut impl ToOutputArray, out_roi: core::Size, stream: &mut impl core::StreamTrait) -> Result<()> {
			output_array_arg!(out_img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ImagePyramid_getLayer_const_const__OutputArrayR_Size_StreamR(self.as_raw_CUDA_ImagePyramid(), out_img.as_raw__OutputArray(), &out_roi, stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

		/// ## Note
		/// This alternative version of [CUDA_ImagePyramidTraitConst::get_layer] function uses the following default values for its arguments:
		/// * stream: Stream::Null()
		#[inline]
		fn get_layer_def(&self, out_img: &mut impl ToOutputArray, out_roi: core::Size) -> Result<()> {
			output_array_arg!(out_img);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cuda_ImagePyramid_getLayer_const_const__OutputArrayR_Size(self.as_raw_CUDA_ImagePyramid(), out_img.as_raw__OutputArray(), &out_roi, ocvrs_return.as_mut_ptr()) };
			return_receive!(ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}

	}

	/// Mutable methods for [crate::cudalegacy::CUDA_ImagePyramid]
	pub trait CUDA_ImagePyramidTrait: core::AlgorithmTrait + crate::cudalegacy::CUDA_ImagePyramidTraitConst {
		fn as_raw_mut_CUDA_ImagePyramid(&mut self) -> *mut c_void;

	}

	impl std::fmt::Debug for CUDA_ImagePyramid {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_ImagePyramid")
				.finish()
		}
	}

	boxed_cast_base! { CUDA_ImagePyramid, core::Algorithm, cv_cuda_ImagePyramid_to_Algorithm }

	impl core::AlgorithmTraitConst for CUDA_ImagePyramid {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}

	impl core::AlgorithmTrait for CUDA_ImagePyramid {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_ImagePyramid, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

	impl crate::cudalegacy::CUDA_ImagePyramidTraitConst for CUDA_ImagePyramid {
		#[inline] fn as_raw_CUDA_ImagePyramid(&self) -> *const c_void { self.as_raw() }
	}

	impl crate::cudalegacy::CUDA_ImagePyramidTrait for CUDA_ImagePyramid {
		#[inline] fn as_raw_mut_CUDA_ImagePyramid(&mut self) -> *mut c_void { self.as_raw_mut() }
	}

	boxed_ref! { CUDA_ImagePyramid, crate::cudalegacy::CUDA_ImagePyramidTraitConst, as_raw_CUDA_ImagePyramid, crate::cudalegacy::CUDA_ImagePyramidTrait, as_raw_mut_CUDA_ImagePyramid }

}
