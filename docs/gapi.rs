pub mod gapi {
	//! \defgroup gapi G-API framework
	//!    # G-API Main Classes
	//!    # G-API Data Types
	//!       # G-API Metadata Descriptors
	//!    # G-API Standard Backends
	//!    # G-API Graph Compilation Arguments
	//!    # G-API Serialization functionality
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::anyTraitConst, super::anyTrait, super::TextTraitConst, super::TextTrait, super::ImageTraitConst, super::ImageTrait, super::PolyTraitConst, super::PolyTrait, super::GCompileArgTraitConst, super::GCompileArgTrait, super::GMatTraitConst, super::GMatTrait, super::GMatPTraitConst, super::GMatPTrait, super::GMatDescTraitConst, super::GMatDescTrait, super::GScalarTraitConst, super::GScalarTrait, super::GScalarDescTraitConst, super::GScalarDescTrait, super::GArrayDescTraitConst, super::GArrayDescTrait, super::Detail_GArrayUTraitConst, super::Detail_GArrayUTrait, super::GOpaqueDescTraitConst, super::GOpaqueDescTrait, super::Detail_GOpaqueUTraitConst, super::Detail_GOpaqueUTrait, super::GFrameTraitConst, super::GFrameTrait, super::GFrameDescTraitConst, super::GFrameDescTrait, super::ScalarTraitConst, super::ScalarTrait, super::MediaFrameTraitConst, super::MediaFrameTrait, super::MediaFrame_ViewTraitConst, super::MediaFrame_ViewTrait, super::MediaFrame_IAdapterTraitConst, super::MediaFrame_IAdapterTrait, super::RMat_ViewTraitConst, super::RMat_ViewTrait, super::RMat_IAdapterTraitConst, super::RMat_IAdapterTrait, super::RMatTraitConst, super::RMatTrait, super::GArgTraitConst, super::GArgTrait, super::GRunArgTraitConst, super::GRunArgTrait, super::DataTraitConst, super::DataTrait, super::Detail_ExtractArgsCallbackTraitConst, super::Detail_ExtractArgsCallbackTrait, super::Detail_ExtractMetaCallbackTraitConst, super::Detail_ExtractMetaCallbackTrait, super::GCompiledTraitConst, super::GCompiledTrait, super::GStreamingCompiledTraitConst, super::GStreamingCompiledTrait, super::GComputationTraitConst, super::GComputationTrait, super::GCallTraitConst, super::GCallTrait, super::GTransformTraitConst, super::GTransformTrait, super::GTypeInfoTraitConst, super::GTypeInfoTrait, super::GKernelTraitConst, super::GKernelTrait, super::GKernelImplTraitConst, super::GKernelImplTrait, super::GBackendTraitConst, super::GBackendTrait, super::GFunctorTraitConst, super::GFunctorTrait, super::GKernelPackageTraitConst, super::GKernelPackageTrait, super::use_onlyTraitConst, super::use_onlyTrait };
	}
	
	pub const Detail_ArgKind_GARRAY: i32 = 6;
	pub const Detail_ArgKind_GFRAME: i32 = 4;
	pub const Detail_ArgKind_GMAT: i32 = 2;
	pub const Detail_ArgKind_GMATP: i32 = 3;
	pub const Detail_ArgKind_GOBJREF: i32 = 1;
	pub const Detail_ArgKind_GOPAQUE: i32 = 7;
	pub const Detail_ArgKind_GSCALAR: i32 = 5;
	pub const Detail_ArgKind_OPAQUE: i32 = 0;
	pub const Detail_ArgKind_OPAQUE_VAL: i32 = 0;
	pub const Detail_OpaqueKind_CV_BOOL: i32 = 1;
	pub const Detail_OpaqueKind_CV_DOUBLE: i32 = 4;
	pub const Detail_OpaqueKind_CV_DRAW_PRIM: i32 = 15;
	pub const Detail_OpaqueKind_CV_FLOAT: i32 = 5;
	pub const Detail_OpaqueKind_CV_INT: i32 = 2;
	pub const Detail_OpaqueKind_CV_INT64: i32 = 3;
	pub const Detail_OpaqueKind_CV_MAT: i32 = 14;
	pub const Detail_OpaqueKind_CV_POINT: i32 = 8;
	pub const Detail_OpaqueKind_CV_POINT2F: i32 = 9;
	pub const Detail_OpaqueKind_CV_POINT3F: i32 = 10;
	pub const Detail_OpaqueKind_CV_RECT: i32 = 12;
	pub const Detail_OpaqueKind_CV_SCALAR: i32 = 13;
	pub const Detail_OpaqueKind_CV_SIZE: i32 = 11;
	pub const Detail_OpaqueKind_CV_STRING: i32 = 7;
	pub const Detail_OpaqueKind_CV_UINT64: i32 = 6;
	pub const Detail_OpaqueKind_CV_UNKNOWN: i32 = 0;
	pub const GShape_GARRAY: i32 = 2;
	pub const GShape_GFRAME: i32 = 4;
	pub const GShape_GMAT: i32 = 0;
	pub const GShape_GOPAQUE: i32 = 3;
	pub const GShape_GSCALAR: i32 = 1;
	pub const MediaFormat_BGR: i32 = 0;
	pub const MediaFormat_GRAY: i32 = 2;
	pub const MediaFormat_NV12: i32 = 1;
	/// Access data for reading
	pub const MediaFrame_Access_R: i32 = 0;
	/// Access data for writing
	pub const MediaFrame_Access_W: i32 = 1;
	pub const RMat_Access_R: i32 = 0;
	pub const RMat_Access_W: i32 = 1;
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum Detail_ArgKind {
		OPAQUE_VAL = 0,
		// Duplicate, use OPAQUE_VAL instead
		// OPAQUE = 0,
		GOBJREF = 1,
		GMAT = 2,
		GMATP = 3,
		GFRAME = 4,
		GSCALAR = 5,
		GARRAY = 6,
		GOPAQUE = 7,
	}
	
	opencv_type_enum! { crate::gapi::Detail_ArgKind }
	
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum Detail_OpaqueKind {
		CV_UNKNOWN = 0,
		CV_BOOL = 1,
		CV_INT = 2,
		CV_INT64 = 3,
		CV_DOUBLE = 4,
		CV_FLOAT = 5,
		CV_UINT64 = 6,
		CV_STRING = 7,
		CV_POINT = 8,
		CV_POINT2F = 9,
		CV_POINT3F = 10,
		CV_SIZE = 11,
		CV_RECT = 12,
		CV_SCALAR = 13,
		CV_MAT = 14,
		CV_DRAW_PRIM = 15,
	}
	
	opencv_type_enum! { crate::gapi::Detail_OpaqueKind }
	
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum GShape {
		GMAT = 0,
		GSCALAR = 1,
		GARRAY = 2,
		GOPAQUE = 3,
		GFRAME = 4,
	}
	
	opencv_type_enum! { crate::gapi::GShape }
	
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum MediaFormat {
		BGR = 0,
		NV12 = 1,
		GRAY = 2,
	}
	
	opencv_type_enum! { crate::gapi::MediaFormat }
	
	/// This enum defines different types of cv::MediaFrame provided
	/// access to the underlying data. Note that different flags can't
	/// be combined in this version.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum MediaFrame_Access {
		/// Access data for reading
		R = 0,
		/// Access data for writing
		W = 1,
	}
	
	opencv_type_enum! { crate::gapi::MediaFrame_Access }
	
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum RMat_Access {
		R = 0,
		W = 1,
	}
	
	opencv_type_enum! { crate::gapi::RMat_Access }
	
	pub type GArgs = core::Vector<crate::gapi::GArg>;
	pub type GCompileArgs = core::Vector<crate::gapi::GCompileArg>;
	pub type GKinds = core::Vector<crate::gapi::Detail_OpaqueKind>;
	pub type GRunArgs = core::Vector<crate::gapi::GRunArg>;
	pub type GShapes = core::Vector<crate::gapi::GShape>;
	pub type GTypesInfo = core::Vector<crate::gapi::GTypeInfo>;
	pub type RMat_Adapter = crate::gapi::RMat_IAdapter;
	pub type RMat_View_stepsT = core::Vector<size_t>;
	pub type GMat2 = core::Tuple<(crate::gapi::GMat, crate::gapi::GMat)>;
	pub type GMat3 = core::Tuple<(crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat)>;
	pub type GMat4 = core::Tuple<(crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat)>;
	pub type GMatScalar = core::Tuple<(crate::gapi::GMat, crate::gapi::GScalar)>;
	pub type ImgProc_cont_method = crate::imgproc::ContourApproximationModes;
	pub type ImgProc_GMat2 = core::Tuple<(crate::gapi::GMat, crate::gapi::GMat)>;
	pub type ImgProc_GMat3 = core::Tuple<(crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat)>;
	pub type ImgProc_retr_mode = crate::imgproc::RetrievalModes;
	#[inline]
	pub fn descr_of_2(mat: &core::Mat) -> Result<crate::gapi::GMatDesc> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_descr_of_const_MatR(mat.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn descr_of_4(frame: &crate::gapi::MediaFrame) -> Result<crate::gapi::GFrameDesc> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_descr_of_const_MediaFrameR(frame.as_raw_MediaFrame(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GFrameDesc::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn descr_of_1(mat: &crate::gapi::RMat) -> Result<crate::gapi::GMatDesc> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_descr_of_const_RMatR(mat.as_raw_RMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn descr_of_3(scalar: core::Scalar) -> Result<crate::gapi::GScalarDesc> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_descr_of_const_ScalarR(&scalar, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GScalarDesc::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn descr_of(mat: &core::UMat) -> Result<crate::gapi::GMatDesc> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_descr_of_const_UMatR(mat.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn empty_array_desc() -> Result<crate::gapi::GArrayDesc> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_empty_array_desc(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GArrayDesc::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn empty_gopaque_desc() -> Result<crate::gapi::GOpaqueDesc> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_empty_gopaque_desc(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GOpaqueDesc::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn empty_scalar_desc() -> Result<crate::gapi::GScalarDesc> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_empty_scalar_desc(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GScalarDesc::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from BGR color space to gray-scaled.
	/// 
	/// The conventional ranges for B, G, and R channel values are 0 to 255.
	/// Resulting gray color value computed as
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7B0%2E114%7D%20%2A%20%5Ctexttt%7Bsrc%7D%28I%29%2EB%20%2B%20%5Ctexttt%7B0%2E587%7D%20%2A%20%5Ctexttt%7Bsrc%7D%28I%29%2EG%20%20%2B%20%5Ctexttt%7B0%2E299%7D%20%2A%20%5Ctexttt%7Bsrc%7D%28I%29%2ER%20)
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.bgr2gray"
	/// 
	/// ## Parameters
	/// * src: input image: 8-bit unsigned 3-channel image [CV_8UC1].
	/// ## See also
	/// BGR2LUV
	#[inline]
	pub fn bgr2_gray(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_BGR2Gray_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from BGR color space to I420 color space.
	/// 
	/// The function converts an input image from BGR color space to I420.
	/// The conventional ranges for R, G, and B channel values are 0 to 255.
	/// 
	/// Output image must be 8-bit unsigned 1-channel image. [CV_8UC1].
	/// Width of I420 output image must be the same as width of input image.
	/// Height of I420 output image must be equal 3/2 from height of input image.
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.bgr2i420"
	/// 
	/// ## Parameters
	/// * src: input image: 8-bit unsigned 3-channel image [CV_8UC3].
	/// ## See also
	/// I4202BGR
	#[inline]
	pub fn bgr2_i420(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_BGR2I420_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from BGR color space to LUV color space.
	/// 
	/// The function converts an input image from BGR color space to LUV.
	/// The conventional ranges for B, G, and R channel values are 0 to 255.
	/// 
	/// Output image must be 8-bit unsigned 3-channel image [CV_8UC3].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.bgr2luv"
	/// 
	/// ## Parameters
	/// * src: input image: 8-bit unsigned 3-channel image [CV_8UC3].
	/// ## See also
	/// RGB2Lab, RGB2LUV
	#[inline]
	pub fn bgr2_luv(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_BGR2LUV_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from BGR color space to RGB color space.
	/// 
	/// The function converts an input image from BGR color space to RGB.
	/// The conventional ranges for B, G, and R channel values are 0 to 255.
	/// 
	/// Output image is 8-bit unsigned 3-channel image [CV_8UC3].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.bgr2rgb"
	/// 
	/// ## Parameters
	/// * src: input image: 8-bit unsigned 3-channel image [CV_8UC3].
	/// ## See also
	/// RGB2BGR
	#[inline]
	pub fn bgr2_rgb(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_BGR2RGB_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from BGR color space to YUV color space.
	/// 
	/// The function converts an input image from BGR color space to YUV.
	/// The conventional ranges for B, G, and R channel values are 0 to 255.
	/// 
	/// Output image must be 8-bit unsigned 3-channel image [CV_8UC3].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.bgr2yuv"
	/// 
	/// ## Parameters
	/// * src: input image: 8-bit unsigned 3-channel image [CV_8UC3].
	/// ## See also
	/// YUV2BGR
	#[inline]
	pub fn bgr2_yuv(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_BGR2YUV_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from BayerGR color space to RGB.
	/// The function converts an input image from BayerGR color space to RGB.
	/// The conventional ranges for G, R, and B channel values are 0 to 255.
	/// 
	/// Output image must be 8-bit unsigned 3-channel image [CV_8UC3].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.bayergr2rgb"
	/// 
	/// ## Parameters
	/// * src_gr: input image: 8-bit unsigned 1-channel image [CV_8UC1].
	/// ## See also
	/// YUV2BGR, NV12toRGB
	#[inline]
	pub fn bayer_gr2_rgb(src_gr: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_BayerGR2RGB_const_GMatR(src_gr.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Finds edges in an image using the Canny algorithm.
	/// 
	/// The function finds edges in the input image and marks them in the output map edges using the
	/// Canny algorithm. The smallest value between threshold1 and threshold2 is used for edge linking. The
	/// largest value is used to find initial segments of strong edges. See
	/// <http://en.wikipedia.org/wiki/Canny_edge_detector>
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.feature.canny"
	/// 
	/// ## Parameters
	/// * image: 8-bit input image.
	/// * threshold1: first threshold for the hysteresis procedure.
	/// * threshold2: second threshold for the hysteresis procedure.
	/// * apertureSize: aperture size for the Sobel operator.
	/// * L2gradient: a flag, indicating whether a more accurate ![inline formula](https://latex.codecogs.com/png.latex?L%5F2) norm
	/// ![inline formula](https://latex.codecogs.com/png.latex?%3D%5Csqrt%7B%28dI%2Fdx%29%5E2%20%2B%20%28dI%2Fdy%29%5E2%7D) should be used to calculate the image gradient magnitude (
	/// L2gradient=true ), or whether the default ![inline formula](https://latex.codecogs.com/png.latex?L%5F1) norm ![inline formula](https://latex.codecogs.com/png.latex?%3D%7CdI%2Fdx%7C%2B%7CdI%2Fdy%7C) is enough (
	/// L2gradient=false ).
	/// 
	/// ## Note
	/// This alternative version of [canny] function uses the following default values for its arguments:
	/// * aperture_size: 3
	/// * l2gradient: false
	#[inline]
	pub fn canny_def(image: &crate::gapi::GMat, threshold1: f64, threshold2: f64) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_Canny_const_GMatR_double_double(image.as_raw_GMat(), threshold1, threshold2, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Finds edges in an image using the Canny algorithm.
	/// 
	/// The function finds edges in the input image and marks them in the output map edges using the
	/// Canny algorithm. The smallest value between threshold1 and threshold2 is used for edge linking. The
	/// largest value is used to find initial segments of strong edges. See
	/// <http://en.wikipedia.org/wiki/Canny_edge_detector>
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.feature.canny"
	/// 
	/// ## Parameters
	/// * image: 8-bit input image.
	/// * threshold1: first threshold for the hysteresis procedure.
	/// * threshold2: second threshold for the hysteresis procedure.
	/// * apertureSize: aperture size for the Sobel operator.
	/// * L2gradient: a flag, indicating whether a more accurate ![inline formula](https://latex.codecogs.com/png.latex?L%5F2) norm
	/// ![inline formula](https://latex.codecogs.com/png.latex?%3D%5Csqrt%7B%28dI%2Fdx%29%5E2%20%2B%20%28dI%2Fdy%29%5E2%7D) should be used to calculate the image gradient magnitude (
	/// L2gradient=true ), or whether the default ![inline formula](https://latex.codecogs.com/png.latex?L%5F1) norm ![inline formula](https://latex.codecogs.com/png.latex?%3D%7CdI%2Fdx%7C%2B%7CdI%2Fdy%7C) is enough (
	/// L2gradient=false ).
	/// 
	/// ## C++ default parameters
	/// * aperture_size: 3
	/// * l2gradient: false
	#[inline]
	pub fn canny(image: &crate::gapi::GMat, threshold1: f64, threshold2: f64, aperture_size: i32, l2gradient: bool) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_Canny_const_GMatR_double_double_int_bool(image.as_raw_GMat(), threshold1, threshold2, aperture_size, l2gradient, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from I420 color space to BGR color space.
	/// 
	/// The function converts an input image from I420 color space to BGR.
	/// The conventional ranges for B, G, and R channel values are 0 to 255.
	/// 
	/// Output image must be 8-bit unsigned 3-channel image. [CV_8UC3].
	/// Width of BGR output image must be the same as width of input image.
	/// Height of BGR output image must be equal 2/3 from height of input image.
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.i4202bgr"
	/// 
	/// ## Parameters
	/// * src: input image: 8-bit unsigned 1-channel image [CV_8UC1].
	/// ## See also
	/// BGR2I420
	#[inline]
	pub fn i4202_bgr(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_I4202BGR_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from I420 color space to BGR color space.
	/// 
	/// The function converts an input image from I420 color space to BGR.
	/// The conventional ranges for B, G, and R channel values are 0 to 255.
	/// 
	/// Output image must be 8-bit unsigned 3-channel image. [CV_8UC3].
	/// Width of RGB output image must be the same as width of input image.
	/// Height of RGB output image must be equal 2/3 from height of input image.
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.i4202rgb"
	/// 
	/// ## Parameters
	/// * src: input image: 8-bit unsigned 1-channel image [CV_8UC1].
	/// ## See also
	/// RGB2I420
	#[inline]
	pub fn i4202_rgb(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_I4202RGB_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Performs a look-up table transform of a matrix.
	/// 
	/// The function LUT fills the output matrix with values from the look-up table. Indices of the entries
	/// are taken from the input matrix. That is, the function processes each element of src as follows:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%20%5Cleftarrow%20%5Ctexttt%7Blut%28src%28I%29%29%7D)
	/// 
	/// Supported matrix data types are [CV_8UC1].
	/// Output is a matrix of the same size and number of channels as src, and the same depth as lut.
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.transform.LUT"
	/// 
	/// ## Parameters
	/// * src: input matrix of 8-bit elements.
	/// * lut: look-up table of 256 elements; in case of multi-channel input array, the table should
	/// either have a single channel (in this case the same table is used for all channels) or the same
	/// number of channels as in the input matrix.
	#[inline]
	pub fn lut(src: &crate::gapi::GMat, lut: &core::Mat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_LUT_const_GMatR_const_MatR(src.as_raw_GMat(), lut.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from LUV color space to BGR color space.
	/// 
	/// The function converts an input image from LUV color space to BGR.
	/// The conventional ranges for B, G, and R channel values are 0 to 255.
	/// 
	/// Output image must be 8-bit unsigned 3-channel image [CV_8UC3].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.luv2bgr"
	/// 
	/// ## Parameters
	/// * src: input image: 8-bit unsigned 3-channel image [CV_8UC3].
	/// ## See also
	/// BGR2LUV
	#[inline]
	pub fn luv2_bgr(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_LUV2BGR_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the Laplacian of an image.
	/// 
	/// The function calculates the Laplacian of the source image by adding up the second x and y
	/// derivatives calculated using the Sobel operator:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%5CDelta%20%5Ctexttt%7Bsrc%7D%20%3D%20%20%5Cfrac%7B%5Cpartial%5E2%20%5Ctexttt%7Bsrc%7D%7D%7B%5Cpartial%20x%5E2%7D%20%2B%20%20%5Cfrac%7B%5Cpartial%5E2%20%5Ctexttt%7Bsrc%7D%7D%7B%5Cpartial%20y%5E2%7D)
	/// 
	/// This is done when `ksize > 1`. When `ksize == 1`, the Laplacian is computed by filtering the image
	/// with the following ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%203) aperture:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%200%20%26%201%20%26%200%5C%5C%201%20%26%20%2D4%20%26%201%5C%5C%200%20%26%201%20%26%200%20%5Cend%7Bbmatrix%7D)
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.filters.laplacian"
	/// 
	/// ## Parameters
	/// * src: Source image.
	/// * ddepth: Desired depth of the destination image.
	/// * ksize: Aperture size used to compute the second-derivative filters. See [get_deriv_kernels] for
	/// details. The size must be positive and odd.
	/// * scale: Optional scale factor for the computed Laplacian values. By default, no scaling is
	/// applied. See [get_deriv_kernels] for details.
	/// * delta: Optional delta value that is added to the results prior to storing them in dst .
	/// * borderType: Pixel extrapolation method, see #BorderTypes. [BORDER_WRAP] is not supported.
	/// ## Returns
	/// Destination image of the same size and the same number of channels as src.
	/// ## See also
	/// Sobel, Scharr
	/// 
	/// ## Note
	/// This alternative version of [laplacian] function uses the following default values for its arguments:
	/// * ksize: 1
	/// * scale: 1
	/// * delta: 0
	/// * border_type: BORDER_DEFAULT
	#[inline]
	pub fn laplacian_def(src: &crate::gapi::GMat, ddepth: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_Laplacian_const_GMatR_int(src.as_raw_GMat(), ddepth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the Laplacian of an image.
	/// 
	/// The function calculates the Laplacian of the source image by adding up the second x and y
	/// derivatives calculated using the Sobel operator:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%5CDelta%20%5Ctexttt%7Bsrc%7D%20%3D%20%20%5Cfrac%7B%5Cpartial%5E2%20%5Ctexttt%7Bsrc%7D%7D%7B%5Cpartial%20x%5E2%7D%20%2B%20%20%5Cfrac%7B%5Cpartial%5E2%20%5Ctexttt%7Bsrc%7D%7D%7B%5Cpartial%20y%5E2%7D)
	/// 
	/// This is done when `ksize > 1`. When `ksize == 1`, the Laplacian is computed by filtering the image
	/// with the following ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%203) aperture:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%200%20%26%201%20%26%200%5C%5C%201%20%26%20%2D4%20%26%201%5C%5C%200%20%26%201%20%26%200%20%5Cend%7Bbmatrix%7D)
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.filters.laplacian"
	/// 
	/// ## Parameters
	/// * src: Source image.
	/// * ddepth: Desired depth of the destination image.
	/// * ksize: Aperture size used to compute the second-derivative filters. See [get_deriv_kernels] for
	/// details. The size must be positive and odd.
	/// * scale: Optional scale factor for the computed Laplacian values. By default, no scaling is
	/// applied. See [get_deriv_kernels] for details.
	/// * delta: Optional delta value that is added to the results prior to storing them in dst .
	/// * borderType: Pixel extrapolation method, see #BorderTypes. [BORDER_WRAP] is not supported.
	/// ## Returns
	/// Destination image of the same size and the same number of channels as src.
	/// ## See also
	/// Sobel, Scharr
	/// 
	/// ## C++ default parameters
	/// * ksize: 1
	/// * scale: 1
	/// * delta: 0
	/// * border_type: BORDER_DEFAULT
	#[inline]
	pub fn laplacian(src: &crate::gapi::GMat, ddepth: i32, ksize: i32, scale: f64, delta: f64, border_type: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_Laplacian_const_GMatR_int_int_double_double_int(src.as_raw_GMat(), ddepth, ksize, scale, delta, border_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from NV12 (YUV420p) color space to BGR.
	/// The function converts an input image from NV12 color space to RGB.
	/// The conventional ranges for Y, U, and V channel values are 0 to 255.
	/// 
	/// Output image must be 8-bit unsigned 3-channel image [CV_8UC3].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.nv12tobgr"
	/// 
	/// ## Parameters
	/// * src_y: input image: 8-bit unsigned 1-channel image [CV_8UC1].
	/// * src_uv: input image: 8-bit unsigned 2-channel image [CV_8UC2].
	/// ## See also
	/// YUV2BGR, NV12toRGB
	#[inline]
	pub fn nv12to_bgr(src_y: &crate::gapi::GMat, src_uv: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_NV12toBGR_const_GMatR_const_GMatR(src_y.as_raw_GMat(), src_uv.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from NV12 (YUV420p) color space to BGR.
	/// The function converts an input image from NV12 color space to BGR.
	/// The conventional ranges for Y, U, and V channel values are 0 to 255.
	/// 
	/// Output image must be 8-bit unsigned planar 3-channel image [CV_8UC1].
	/// Planar image memory layout is three planes laying in the memory contiguously,
	/// so the image height should be plane_height*plane_number,
	/// image type is [CV_8UC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.nv12torgbp"
	/// 
	/// ## Parameters
	/// * src_y: input image: 8-bit unsigned 1-channel image [CV_8UC1].
	/// * src_uv: input image: 8-bit unsigned 2-channel image [CV_8UC2].
	/// ## See also
	/// YUV2RGB, NV12toRGBp, NV12toBGR
	#[inline]
	pub fn nv12to_bg_rp(src_y: &crate::gapi::GMat, src_uv: &crate::gapi::GMat) -> Result<crate::gapi::GMatP> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_NV12toBGRp_const_GMatR_const_GMatR(src_y.as_raw_GMat(), src_uv.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMatP::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from NV12 (YUV420p) color space to gray-scaled.
	/// The function converts an input image from NV12 color space to gray-scaled.
	/// The conventional ranges for Y, U, and V channel values are 0 to 255.
	/// 
	/// Output image must be 8-bit unsigned 1-channel image [CV_8UC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.nv12togray"
	/// 
	/// ## Parameters
	/// * src_y: input image: 8-bit unsigned 1-channel image [CV_8UC1].
	/// * src_uv: input image: 8-bit unsigned 2-channel image [CV_8UC2].
	/// ## See also
	/// YUV2RGB, NV12toBGR
	#[inline]
	pub fn nv12to_gray(src_y: &crate::gapi::GMat, src_uv: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_NV12toGray_const_GMatR_const_GMatR(src_y.as_raw_GMat(), src_uv.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from NV12 (YUV420p) color space to RGB.
	/// The function converts an input image from NV12 color space to RGB.
	/// The conventional ranges for Y, U, and V channel values are 0 to 255.
	/// 
	/// Output image must be 8-bit unsigned 3-channel image [CV_8UC3].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.nv12torgb"
	/// 
	/// ## Parameters
	/// * src_y: input image: 8-bit unsigned 1-channel image [CV_8UC1].
	/// * src_uv: input image: 8-bit unsigned 2-channel image [CV_8UC2].
	/// ## See also
	/// YUV2RGB, NV12toBGR
	#[inline]
	pub fn nv12to_rgb(src_y: &crate::gapi::GMat, src_uv: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_NV12toRGB_const_GMatR_const_GMatR(src_y.as_raw_GMat(), src_uv.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from NV12 (YUV420p) color space to RGB.
	/// The function converts an input image from NV12 color space to RGB.
	/// The conventional ranges for Y, U, and V channel values are 0 to 255.
	/// 
	/// Output image must be 8-bit unsigned planar 3-channel image [CV_8UC1].
	/// Planar image memory layout is three planes laying in the memory contiguously,
	/// so the image height should be plane_height*plane_number,
	/// image type is [CV_8UC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.nv12torgbp"
	/// 
	/// ## Parameters
	/// * src_y: input image: 8-bit unsigned 1-channel image [CV_8UC1].
	/// * src_uv: input image: 8-bit unsigned 2-channel image [CV_8UC2].
	/// ## See also
	/// YUV2RGB, NV12toBGRp, NV12toRGB
	#[inline]
	pub fn nv12to_rg_bp(src_y: &crate::gapi::GMat, src_uv: &crate::gapi::GMat) -> Result<crate::gapi::GMatP> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_NV12toRGBp_const_GMatR_const_GMatR(src_y.as_raw_GMat(), src_uv.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMatP::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from RGB color space to gray-scaled.
	/// 
	/// The conventional ranges for R, G, and B channel values are 0 to 255.
	/// Resulting gray color value computed as
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7B0%2E299%7D%20%2A%20%5Ctexttt%7Bsrc%7D%28I%29%2ER%20%2B%20%5Ctexttt%7B0%2E587%7D%20%2A%20%5Ctexttt%7Bsrc%7D%28I%29%2EG%20%20%2B%20%5Ctexttt%7B0%2E114%7D%20%2A%20%5Ctexttt%7Bsrc%7D%28I%29%2EB%20)
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.rgb2gray"
	/// 
	/// ## Parameters
	/// * src: input image: 8-bit unsigned 3-channel image [CV_8UC1].
	/// ## See also
	/// RGB2YUV
	#[inline]
	pub fn rgb2_gray(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_RGB2Gray_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from RGB color space to gray-scaled.
	/// 
	/// The conventional ranges for R, G, and B channel values are 0 to 255.
	/// Resulting gray color value computed as
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7B0%2E299%7D%20%2A%20%5Ctexttt%7Bsrc%7D%28I%29%2ER%20%2B%20%5Ctexttt%7B0%2E587%7D%20%2A%20%5Ctexttt%7Bsrc%7D%28I%29%2EG%20%20%2B%20%5Ctexttt%7B0%2E114%7D%20%2A%20%5Ctexttt%7Bsrc%7D%28I%29%2EB%20)
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.rgb2gray"
	/// 
	/// ## Parameters
	/// * src: input image: 8-bit unsigned 3-channel image [CV_8UC1].
	/// ## See also
	/// RGB2YUV
	/// 
	/// ## Overloaded parameters
	/// 
	/// Resulting gray color value computed as
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7BrY%7D%20%2A%20%5Ctexttt%7Bsrc%7D%28I%29%2ER%20%2B%20%5Ctexttt%7BgY%7D%20%2A%20%5Ctexttt%7Bsrc%7D%28I%29%2EG%20%20%2B%20%5Ctexttt%7BbY%7D%20%2A%20%5Ctexttt%7Bsrc%7D%28I%29%2EB%20)
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.rgb2graycustom"
	/// 
	/// * src: input image: 8-bit unsigned 3-channel image [CV_8UC1].
	/// * rY: float multiplier for R channel.
	/// * gY: float multiplier for G channel.
	/// * bY: float multiplier for B channel.
	/// RGB2YUV
	#[inline]
	pub fn rgb2_gray_1(src: &crate::gapi::GMat, r_y: f32, g_y: f32, b_y: f32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_RGB2Gray_const_GMatR_float_float_float(src.as_raw_GMat(), r_y, g_y, b_y, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from RGB color space to HSV.
	/// The function converts an input image from RGB color space to HSV.
	/// The conventional ranges for R, G, and B channel values are 0 to 255.
	/// 
	/// Output image must be 8-bit unsigned 3-channel image [CV_8UC3].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.rgb2hsv"
	/// 
	/// ## Parameters
	/// * src: input image: 8-bit unsigned 3-channel image [CV_8UC3].
	/// ## See also
	/// YUV2BGR, NV12toRGB
	#[inline]
	pub fn rgb2_hsv(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_RGB2HSV_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from RGB color space to I420 color space.
	/// 
	/// The function converts an input image from RGB color space to I420.
	/// The conventional ranges for R, G, and B channel values are 0 to 255.
	/// 
	/// Output image must be 8-bit unsigned 1-channel image. [CV_8UC1].
	/// Width of I420 output image must be the same as width of input image.
	/// Height of I420 output image must be equal 3/2 from height of input image.
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.rgb2i420"
	/// 
	/// ## Parameters
	/// * src: input image: 8-bit unsigned 3-channel image [CV_8UC3].
	/// ## See also
	/// I4202RGB
	#[inline]
	pub fn rgb2_i420(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_RGB2I420_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from RGB color space to Lab color space.
	/// 
	/// The function converts an input image from BGR color space to Lab.
	/// The conventional ranges for R, G, and B channel values are 0 to 255.
	/// 
	/// Output image must be 8-bit unsigned 3-channel image [CV_8UC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.rgb2lab"
	/// 
	/// ## Parameters
	/// * src: input image: 8-bit unsigned 3-channel image [CV_8UC1].
	/// ## See also
	/// RGB2YUV, RGB2LUV
	#[inline]
	pub fn rgb2_lab(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_RGB2Lab_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from RGB color space to YUV422.
	/// The function converts an input image from RGB color space to YUV422.
	/// The conventional ranges for R, G, and B channel values are 0 to 255.
	/// 
	/// Output image must be 8-bit unsigned 2-channel image [CV_8UC2].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.rgb2yuv422"
	/// 
	/// ## Parameters
	/// * src: input image: 8-bit unsigned 3-channel image [CV_8UC3].
	/// ## See also
	/// YUV2BGR, NV12toRGB
	#[inline]
	pub fn rgb2_yuv422(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_RGB2YUV422_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from RGB color space to YUV color space.
	/// 
	/// The function converts an input image from RGB color space to YUV.
	/// The conventional ranges for R, G, and B channel values are 0 to 255.
	/// 
	/// In case of linear transformations, the range does not matter. But in case of a non-linear
	/// transformation, an input RGB image should be normalized to the proper value range to get the correct
	/// results, like here, at RGB ![inline formula](https://latex.codecogs.com/png.latex?%5Crightarrow) Y\*u\*v\* transformation.
	/// Output image must be 8-bit unsigned 3-channel image [CV_8UC3].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.rgb2yuv"
	/// 
	/// ## Parameters
	/// * src: input image: 8-bit unsigned 3-channel image [CV_8UC3].
	/// ## See also
	/// YUV2RGB, RGB2Lab
	#[inline]
	pub fn rgb2_yuv(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_RGB2YUV_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the first, second, third, or mixed image derivatives using an extended Sobel operator.
	/// 
	/// In all cases except one, the ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bksize%7D%20%5Ctimes%20%5Ctexttt%7Bksize%7D) separable kernel is used to
	/// calculate the derivative. When ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bksize%20%3D%201%7D), the ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%201) or ![inline formula](https://latex.codecogs.com/png.latex?1%20%5Ctimes%203)
	/// kernel is used (that is, no Gaussian smoothing is done). `ksize = 1` can only be used for the first
	/// or the second x- or y- derivatives.
	/// 
	/// There is also the special value `ksize = FILTER_SCHARR (-1)` that corresponds to the ![inline formula](https://latex.codecogs.com/png.latex?3%5Ctimes3) Scharr
	/// filter that may give more accurate results than the ![inline formula](https://latex.codecogs.com/png.latex?3%5Ctimes3) Sobel. The Scharr aperture is
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20%2D3%20%26%200%20%26%203%5C%5C%20%2D10%20%26%200%20%26%2010%5C%5C%20%2D3%20%26%200%20%26%203%20%5Cend%7Bbmatrix%7D)
	/// 
	/// for the x-derivative, or transposed for the y-derivative.
	/// 
	/// The function calculates an image derivative by convolving the image with the appropriate kernel:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%5Cfrac%7B%5Cpartial%5E%7Bxorder%2Byorder%7D%20%5Ctexttt%7Bsrc%7D%7D%7B%5Cpartial%20x%5E%7Bxorder%7D%20%5Cpartial%20y%5E%7Byorder%7D%7D)
	/// 
	/// The Sobel operators combine Gaussian smoothing and differentiation, so the result is more or less
	/// resistant to the noise. Most often, the function is called with ( xorder = 1, yorder = 0, ksize = 3)
	/// or ( xorder = 0, yorder = 1, ksize = 3) to calculate the first x- or y- image derivative. The first
	/// case corresponds to a kernel of:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20%2D1%20%26%200%20%26%201%5C%5C%20%2D2%20%26%200%20%26%202%5C%5C%20%2D1%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D)
	/// 
	/// The second case corresponds to a kernel of:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20%2D1%20%26%20%2D2%20%26%20%2D1%5C%5C%200%20%26%200%20%26%200%5C%5C%201%20%26%202%20%26%201%20%5Cend%7Bbmatrix%7D)
	/// 
	/// 
	/// Note:
	///  - First returned matrix correspons to dx derivative while the second one to dy.
	///  - Rounding to nearest even is procedeed if hardware supports it, if not - to nearest.
	///  - Function textual ID is "org.opencv.imgproc.filters.sobelxy"
	/// 
	/// ## Parameters
	/// * src: input image.
	/// * ddepth: output image depth, see [filter_depths] "combinations"; in the case of
	///    8-bit input images it will result in truncated derivatives.
	/// * order: order of the derivatives.
	/// * ksize: size of the extended Sobel kernel; it must be odd.
	/// * scale: optional scale factor for the computed derivative values; by default, no scaling is
	/// applied (see cv::getDerivKernels for details).
	/// * delta: optional delta value that is added to the results prior to storing them in dst.
	/// * borderType: pixel extrapolation method, see cv::BorderTypes
	/// * borderValue: border value in case of constant border type
	/// ## See also
	/// filter2D, gaussianBlur, cartToPolar
	/// 
	/// ## Note
	/// This alternative version of [sobel_xy] function uses the following default values for its arguments:
	/// * ksize: 3
	/// * scale: 1
	/// * delta: 0
	/// * border_type: BORDER_DEFAULT
	/// * border_value: Scalar(0)
	#[inline]
	pub fn sobel_xy_def(src: &crate::gapi::GMat, ddepth: i32, order: i32) -> Result<core::Tuple<(crate::gapi::GMat, crate::gapi::GMat)>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_SobelXY_const_GMatR_int_int(src.as_raw_GMat(), ddepth, order, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Tuple::<(crate::gapi::GMat, crate::gapi::GMat)>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the first, second, third, or mixed image derivatives using an extended Sobel operator.
	/// 
	/// In all cases except one, the ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bksize%7D%20%5Ctimes%20%5Ctexttt%7Bksize%7D) separable kernel is used to
	/// calculate the derivative. When ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bksize%20%3D%201%7D), the ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%201) or ![inline formula](https://latex.codecogs.com/png.latex?1%20%5Ctimes%203)
	/// kernel is used (that is, no Gaussian smoothing is done). `ksize = 1` can only be used for the first
	/// or the second x- or y- derivatives.
	/// 
	/// There is also the special value `ksize = FILTER_SCHARR (-1)` that corresponds to the ![inline formula](https://latex.codecogs.com/png.latex?3%5Ctimes3) Scharr
	/// filter that may give more accurate results than the ![inline formula](https://latex.codecogs.com/png.latex?3%5Ctimes3) Sobel. The Scharr aperture is
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20%2D3%20%26%200%20%26%203%5C%5C%20%2D10%20%26%200%20%26%2010%5C%5C%20%2D3%20%26%200%20%26%203%20%5Cend%7Bbmatrix%7D)
	/// 
	/// for the x-derivative, or transposed for the y-derivative.
	/// 
	/// The function calculates an image derivative by convolving the image with the appropriate kernel:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%5Cfrac%7B%5Cpartial%5E%7Bxorder%2Byorder%7D%20%5Ctexttt%7Bsrc%7D%7D%7B%5Cpartial%20x%5E%7Bxorder%7D%20%5Cpartial%20y%5E%7Byorder%7D%7D)
	/// 
	/// The Sobel operators combine Gaussian smoothing and differentiation, so the result is more or less
	/// resistant to the noise. Most often, the function is called with ( xorder = 1, yorder = 0, ksize = 3)
	/// or ( xorder = 0, yorder = 1, ksize = 3) to calculate the first x- or y- image derivative. The first
	/// case corresponds to a kernel of:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20%2D1%20%26%200%20%26%201%5C%5C%20%2D2%20%26%200%20%26%202%5C%5C%20%2D1%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D)
	/// 
	/// The second case corresponds to a kernel of:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20%2D1%20%26%20%2D2%20%26%20%2D1%5C%5C%200%20%26%200%20%26%200%5C%5C%201%20%26%202%20%26%201%20%5Cend%7Bbmatrix%7D)
	/// 
	/// 
	/// Note:
	///  - First returned matrix correspons to dx derivative while the second one to dy.
	///  - Rounding to nearest even is procedeed if hardware supports it, if not - to nearest.
	///  - Function textual ID is "org.opencv.imgproc.filters.sobelxy"
	/// 
	/// ## Parameters
	/// * src: input image.
	/// * ddepth: output image depth, see [filter_depths] "combinations"; in the case of
	///    8-bit input images it will result in truncated derivatives.
	/// * order: order of the derivatives.
	/// * ksize: size of the extended Sobel kernel; it must be odd.
	/// * scale: optional scale factor for the computed derivative values; by default, no scaling is
	/// applied (see cv::getDerivKernels for details).
	/// * delta: optional delta value that is added to the results prior to storing them in dst.
	/// * borderType: pixel extrapolation method, see cv::BorderTypes
	/// * borderValue: border value in case of constant border type
	/// ## See also
	/// filter2D, gaussianBlur, cartToPolar
	/// 
	/// ## C++ default parameters
	/// * ksize: 3
	/// * scale: 1
	/// * delta: 0
	/// * border_type: BORDER_DEFAULT
	/// * border_value: Scalar(0)
	#[inline]
	pub fn sobel_xy(src: &crate::gapi::GMat, ddepth: i32, order: i32, ksize: i32, scale: f64, delta: f64, border_type: i32, border_value: core::Scalar) -> Result<core::Tuple<(crate::gapi::GMat, crate::gapi::GMat)>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_SobelXY_const_GMatR_int_int_int_double_double_int_const_ScalarR(src.as_raw_GMat(), ddepth, order, ksize, scale, delta, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Tuple::<(crate::gapi::GMat, crate::gapi::GMat)>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the first, second, third, or mixed image derivatives using an extended Sobel operator.
	/// 
	/// In all cases except one, the ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bksize%7D%20%5Ctimes%20%5Ctexttt%7Bksize%7D) separable kernel is used to
	/// calculate the derivative. When ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bksize%20%3D%201%7D), the ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%201) or ![inline formula](https://latex.codecogs.com/png.latex?1%20%5Ctimes%203)
	/// kernel is used (that is, no Gaussian smoothing is done). `ksize = 1` can only be used for the first
	/// or the second x- or y- derivatives.
	/// 
	/// There is also the special value `ksize = FILTER_SCHARR (-1)` that corresponds to the ![inline formula](https://latex.codecogs.com/png.latex?3%5Ctimes3) Scharr
	/// filter that may give more accurate results than the ![inline formula](https://latex.codecogs.com/png.latex?3%5Ctimes3) Sobel. The Scharr aperture is
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20%2D3%20%26%200%20%26%203%5C%5C%20%2D10%20%26%200%20%26%2010%5C%5C%20%2D3%20%26%200%20%26%203%20%5Cend%7Bbmatrix%7D)
	/// 
	/// for the x-derivative, or transposed for the y-derivative.
	/// 
	/// The function calculates an image derivative by convolving the image with the appropriate kernel:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%5Cfrac%7B%5Cpartial%5E%7Bxorder%2Byorder%7D%20%5Ctexttt%7Bsrc%7D%7D%7B%5Cpartial%20x%5E%7Bxorder%7D%20%5Cpartial%20y%5E%7Byorder%7D%7D)
	/// 
	/// The Sobel operators combine Gaussian smoothing and differentiation, so the result is more or less
	/// resistant to the noise. Most often, the function is called with ( xorder = 1, yorder = 0, ksize = 3)
	/// or ( xorder = 0, yorder = 1, ksize = 3) to calculate the first x- or y- image derivative. The first
	/// case corresponds to a kernel of:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20%2D1%20%26%200%20%26%201%5C%5C%20%2D2%20%26%200%20%26%202%5C%5C%20%2D1%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D)
	/// 
	/// The second case corresponds to a kernel of:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20%2D1%20%26%20%2D2%20%26%20%2D1%5C%5C%200%20%26%200%20%26%200%5C%5C%201%20%26%202%20%26%201%20%5Cend%7Bbmatrix%7D)
	/// 
	/// 
	/// Note:
	///  - Rounding to nearest even is procedeed if hardware supports it, if not - to nearest.
	///  - Function textual ID is "org.opencv.imgproc.filters.sobel"
	/// 
	/// ## Parameters
	/// * src: input image.
	/// * ddepth: output image depth, see [filter_depths] "combinations"; in the case of
	///    8-bit input images it will result in truncated derivatives.
	/// * dx: order of the derivative x.
	/// * dy: order of the derivative y.
	/// * ksize: size of the extended Sobel kernel; it must be odd.
	/// * scale: optional scale factor for the computed derivative values; by default, no scaling is
	/// applied (see cv::getDerivKernels for details).
	/// * delta: optional delta value that is added to the results prior to storing them in dst.
	/// * borderType: pixel extrapolation method, see cv::BorderTypes
	/// * borderValue: border value in case of constant border type
	/// ## See also
	/// filter2D, gaussianBlur, cartToPolar
	/// 
	/// ## Note
	/// This alternative version of [sobel] function uses the following default values for its arguments:
	/// * ksize: 3
	/// * scale: 1
	/// * delta: 0
	/// * border_type: BORDER_DEFAULT
	/// * border_value: Scalar(0)
	#[inline]
	pub fn sobel_def(src: &crate::gapi::GMat, ddepth: i32, dx: i32, dy: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_Sobel_const_GMatR_int_int_int(src.as_raw_GMat(), ddepth, dx, dy, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the first, second, third, or mixed image derivatives using an extended Sobel operator.
	/// 
	/// In all cases except one, the ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bksize%7D%20%5Ctimes%20%5Ctexttt%7Bksize%7D) separable kernel is used to
	/// calculate the derivative. When ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bksize%20%3D%201%7D), the ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%201) or ![inline formula](https://latex.codecogs.com/png.latex?1%20%5Ctimes%203)
	/// kernel is used (that is, no Gaussian smoothing is done). `ksize = 1` can only be used for the first
	/// or the second x- or y- derivatives.
	/// 
	/// There is also the special value `ksize = FILTER_SCHARR (-1)` that corresponds to the ![inline formula](https://latex.codecogs.com/png.latex?3%5Ctimes3) Scharr
	/// filter that may give more accurate results than the ![inline formula](https://latex.codecogs.com/png.latex?3%5Ctimes3) Sobel. The Scharr aperture is
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20%2D3%20%26%200%20%26%203%5C%5C%20%2D10%20%26%200%20%26%2010%5C%5C%20%2D3%20%26%200%20%26%203%20%5Cend%7Bbmatrix%7D)
	/// 
	/// for the x-derivative, or transposed for the y-derivative.
	/// 
	/// The function calculates an image derivative by convolving the image with the appropriate kernel:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%5Cfrac%7B%5Cpartial%5E%7Bxorder%2Byorder%7D%20%5Ctexttt%7Bsrc%7D%7D%7B%5Cpartial%20x%5E%7Bxorder%7D%20%5Cpartial%20y%5E%7Byorder%7D%7D)
	/// 
	/// The Sobel operators combine Gaussian smoothing and differentiation, so the result is more or less
	/// resistant to the noise. Most often, the function is called with ( xorder = 1, yorder = 0, ksize = 3)
	/// or ( xorder = 0, yorder = 1, ksize = 3) to calculate the first x- or y- image derivative. The first
	/// case corresponds to a kernel of:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20%2D1%20%26%200%20%26%201%5C%5C%20%2D2%20%26%200%20%26%202%5C%5C%20%2D1%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D)
	/// 
	/// The second case corresponds to a kernel of:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bbmatrix%7D%20%2D1%20%26%20%2D2%20%26%20%2D1%5C%5C%200%20%26%200%20%26%200%5C%5C%201%20%26%202%20%26%201%20%5Cend%7Bbmatrix%7D)
	/// 
	/// 
	/// Note:
	///  - Rounding to nearest even is procedeed if hardware supports it, if not - to nearest.
	///  - Function textual ID is "org.opencv.imgproc.filters.sobel"
	/// 
	/// ## Parameters
	/// * src: input image.
	/// * ddepth: output image depth, see [filter_depths] "combinations"; in the case of
	///    8-bit input images it will result in truncated derivatives.
	/// * dx: order of the derivative x.
	/// * dy: order of the derivative y.
	/// * ksize: size of the extended Sobel kernel; it must be odd.
	/// * scale: optional scale factor for the computed derivative values; by default, no scaling is
	/// applied (see cv::getDerivKernels for details).
	/// * delta: optional delta value that is added to the results prior to storing them in dst.
	/// * borderType: pixel extrapolation method, see cv::BorderTypes
	/// * borderValue: border value in case of constant border type
	/// ## See also
	/// filter2D, gaussianBlur, cartToPolar
	/// 
	/// ## C++ default parameters
	/// * ksize: 3
	/// * scale: 1
	/// * delta: 0
	/// * border_type: BORDER_DEFAULT
	/// * border_value: Scalar(0)
	#[inline]
	pub fn sobel(src: &crate::gapi::GMat, ddepth: i32, dx: i32, dy: i32, ksize: i32, scale: f64, delta: f64, border_type: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_Sobel_const_GMatR_int_int_int_int_double_double_int_const_ScalarR(src.as_raw_GMat(), ddepth, dx, dy, ksize, scale, delta, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from YUV color space to BGR color space.
	/// 
	/// The function converts an input image from YUV color space to BGR.
	/// The conventional ranges for B, G, and R channel values are 0 to 255.
	/// 
	/// Output image must be 8-bit unsigned 3-channel image [CV_8UC3].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.yuv2bgr"
	/// 
	/// ## Parameters
	/// * src: input image: 8-bit unsigned 3-channel image [CV_8UC3].
	/// ## See also
	/// BGR2YUV
	#[inline]
	pub fn yuv2_bgr(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_YUV2BGR_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts an image from YUV color space to RGB.
	/// The function converts an input image from YUV color space to RGB.
	/// The conventional ranges for Y, U, and V channel values are 0 to 255.
	/// 
	/// Output image must be 8-bit unsigned 3-channel image [CV_8UC3].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.colorconvert.yuv2rgb"
	/// 
	/// ## Parameters
	/// * src: input image: 8-bit unsigned 3-channel image [CV_8UC3].
	/// ## See also
	/// RGB2Lab, RGB2YUV
	#[inline]
	pub fn yuv2_rgb(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_YUV2RGB_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates absolute value of matrix elements.
	/// 
	/// The function abs calculates absolute difference between matrix elements and given scalar value:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%28I%29%20%3D%20%20%5Ctexttt%7Bsaturate%7D%20%28%7C%20%5Ctexttt%7Bsrc1%7D%28I%29%20%2D%20%20%5Ctexttt%7BmatC%7D%28I%29%7C%29)
	///    where matC is constructed from given scalar c and has the same sizes and depth as input matrix src.
	/// 
	/// Output matrix must be of the same size and depth as src.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.matrixop.absdiffC"
	/// ## Parameters
	/// * src: input matrix.
	/// * c: scalar to be subtracted.
	/// ## See also
	/// min, max
	#[inline]
	pub fn abs_diff_c(src: &crate::gapi::GMat, c: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_absDiffC_const_GMatR_const_GScalarR(src.as_raw_GMat(), c.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the per-element absolute difference between two matrices.
	/// 
	/// The function absDiff calculates absolute difference between two matrices of the same size and depth:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%28I%29%20%3D%20%20%5Ctexttt%7Bsaturate%7D%20%28%7C%20%5Ctexttt%7Bsrc1%7D%28I%29%20%2D%20%20%5Ctexttt%7Bsrc2%7D%28I%29%7C%29)
	///    where I is a multi-dimensional index of matrix elements. In case of
	///    multi-channel matrices, each channel is processed independently.
	/// Output matrix must have the same size and depth as input matrices.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.matrixop.absdiff"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix.
	/// ## See also
	/// abs
	#[inline]
	pub fn abs_diff(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_absDiff_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the per-element sum of matrix and given scalar.
	/// 
	/// The function addC adds a given scalar value to each element of given matrix.
	/// The function can be replaced with matrix expressions:
	/// 
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%2B%20%5Ctexttt%7Bc%7D)
	/// 
	/// Depth of the output matrix is determined by the ddepth parameter.
	/// If ddepth is set to default -1, the depth of output matrix will be the same as the depth of input matrix.
	/// The matrices can be single or multi channel. Output matrix must have the same size and number of channels as the input matrix.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.addC"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * c: scalar value to be added.
	/// * ddepth: optional depth of the output matrix.
	/// ## See also
	/// sub, addWeighted
	/// 
	/// ## Note
	/// This alternative version of [add_c] function uses the following default values for its arguments:
	/// * ddepth: -1
	#[inline]
	pub fn add_c_def(src1: &crate::gapi::GMat, c: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_addC_const_GMatR_const_GScalarR(src1.as_raw_GMat(), c.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the per-element sum of matrix and given scalar.
	/// 
	/// The function addC adds a given scalar value to each element of given matrix.
	/// The function can be replaced with matrix expressions:
	/// 
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%2B%20%5Ctexttt%7Bc%7D)
	/// 
	/// Depth of the output matrix is determined by the ddepth parameter.
	/// If ddepth is set to default -1, the depth of output matrix will be the same as the depth of input matrix.
	/// The matrices can be single or multi channel. Output matrix must have the same size and number of channels as the input matrix.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.addC"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * c: scalar value to be added.
	/// * ddepth: optional depth of the output matrix.
	/// ## See also
	/// sub, addWeighted
	/// 
	/// ## C++ default parameters
	/// * ddepth: -1
	#[inline]
	pub fn add_c(src1: &crate::gapi::GMat, c: &crate::gapi::GScalar, ddepth: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_addC_const_GMatR_const_GScalarR_int(src1.as_raw_GMat(), c.as_raw_GScalar(), ddepth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// @overload
	/// 
	/// ## Note
	/// This alternative version of [add_c_1] function uses the following default values for its arguments:
	/// * ddepth: -1
	#[inline]
	pub fn add_c_1_def(c: &crate::gapi::GScalar, src1: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_addC_const_GScalarR_const_GMatR(c.as_raw_GScalar(), src1.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the per-element sum of matrix and given scalar.
	/// 
	/// The function addC adds a given scalar value to each element of given matrix.
	/// The function can be replaced with matrix expressions:
	/// 
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%2B%20%5Ctexttt%7Bc%7D)
	/// 
	/// Depth of the output matrix is determined by the ddepth parameter.
	/// If ddepth is set to default -1, the depth of output matrix will be the same as the depth of input matrix.
	/// The matrices can be single or multi channel. Output matrix must have the same size and number of channels as the input matrix.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.addC"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * c: scalar value to be added.
	/// * ddepth: optional depth of the output matrix.
	/// ## See also
	/// sub, addWeighted
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * ddepth: -1
	#[inline]
	pub fn add_c_1(c: &crate::gapi::GScalar, src1: &crate::gapi::GMat, ddepth: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_addC_const_GScalarR_const_GMatR_int(c.as_raw_GScalar(), src1.as_raw_GMat(), ddepth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the weighted sum of two matrices.
	/// 
	/// The function addWeighted calculates the weighted sum of two matrices as follows:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Bsaturate%7D%20%28%20%5Ctexttt%7Bsrc1%7D%20%28I%29%2A%20%5Ctexttt%7Balpha%7D%20%2B%20%20%5Ctexttt%7Bsrc2%7D%20%28I%29%2A%20%5Ctexttt%7Bbeta%7D%20%2B%20%20%5Ctexttt%7Bgamma%7D%20%29)
	/// where I is a multi-dimensional index of array elements. In case of multi-channel matrices, each
	/// channel is processed independently.
	/// 
	/// The function can be replaced with a matrix expression:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%28I%29%20%3D%20%20%5Ctexttt%7Balpha%7D%20%2A%20%5Ctexttt%7Bsrc1%7D%28I%29%20%2D%20%5Ctexttt%7Bbeta%7D%20%2A%20%5Ctexttt%7Bsrc2%7D%28I%29%20%2B%20%5Ctexttt%7Bgamma%7D%20)
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.matrixop.addweighted"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * alpha: weight of the first matrix elements.
	/// * src2: second input matrix of the same size and channel number as src1.
	/// * beta: weight of the second matrix elements.
	/// * gamma: scalar added to each sum.
	/// * ddepth: optional depth of the output matrix.
	/// ## See also
	/// add, sub
	/// 
	/// ## Note
	/// This alternative version of [add_weighted] function uses the following default values for its arguments:
	/// * ddepth: -1
	#[inline]
	pub fn add_weighted_def(src1: &crate::gapi::GMat, alpha: f64, src2: &crate::gapi::GMat, beta: f64, gamma: f64) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_addWeighted_const_GMatR_double_const_GMatR_double_double(src1.as_raw_GMat(), alpha, src2.as_raw_GMat(), beta, gamma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the weighted sum of two matrices.
	/// 
	/// The function addWeighted calculates the weighted sum of two matrices as follows:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Bsaturate%7D%20%28%20%5Ctexttt%7Bsrc1%7D%20%28I%29%2A%20%5Ctexttt%7Balpha%7D%20%2B%20%20%5Ctexttt%7Bsrc2%7D%20%28I%29%2A%20%5Ctexttt%7Bbeta%7D%20%2B%20%20%5Ctexttt%7Bgamma%7D%20%29)
	/// where I is a multi-dimensional index of array elements. In case of multi-channel matrices, each
	/// channel is processed independently.
	/// 
	/// The function can be replaced with a matrix expression:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%28I%29%20%3D%20%20%5Ctexttt%7Balpha%7D%20%2A%20%5Ctexttt%7Bsrc1%7D%28I%29%20%2D%20%5Ctexttt%7Bbeta%7D%20%2A%20%5Ctexttt%7Bsrc2%7D%28I%29%20%2B%20%5Ctexttt%7Bgamma%7D%20)
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.matrixop.addweighted"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * alpha: weight of the first matrix elements.
	/// * src2: second input matrix of the same size and channel number as src1.
	/// * beta: weight of the second matrix elements.
	/// * gamma: scalar added to each sum.
	/// * ddepth: optional depth of the output matrix.
	/// ## See also
	/// add, sub
	/// 
	/// ## C++ default parameters
	/// * ddepth: -1
	#[inline]
	pub fn add_weighted(src1: &crate::gapi::GMat, alpha: f64, src2: &crate::gapi::GMat, beta: f64, gamma: f64, ddepth: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_addWeighted_const_GMatR_double_const_GMatR_double_double_int(src1.as_raw_GMat(), alpha, src2.as_raw_GMat(), beta, gamma, ddepth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the per-element sum of two matrices.
	/// 
	/// The function add calculates sum of two matrices of the same size and the same number of channels:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%28I%29%20%3D%20%20%5Ctexttt%7Bsaturate%7D%20%28%20%5Ctexttt%7Bsrc1%7D%28I%29%20%2B%20%20%5Ctexttt%7Bsrc2%7D%28I%29%29%20%5Cquad%20%5Ctexttt%7Bif%20mask%7D%28I%29%20%5Cne0)
	/// 
	/// The function can be replaced with matrix expressions:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%2B%20%5Ctexttt%7Bsrc2%7D)
	/// 
	/// The input matrices and the output matrix can all have the same or different depths. For example, you
	/// can add a 16-bit unsigned matrix to a 8-bit signed matrix and store the sum as a 32-bit
	/// floating-point matrix. Depth of the output matrix is determined by the ddepth parameter.
	/// If src1.depth() == src2.depth(), ddepth can be set to the default -1. In this case, the output matrix will have
	/// the same depth as the input matrices.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.add"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix.
	/// * ddepth: optional depth of the output matrix.
	/// ## See also
	/// sub, addWeighted
	/// 
	/// ## Note
	/// This alternative version of [add] function uses the following default values for its arguments:
	/// * ddepth: -1
	#[inline]
	pub fn add_def(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_add_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the per-element sum of two matrices.
	/// 
	/// The function add calculates sum of two matrices of the same size and the same number of channels:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%28I%29%20%3D%20%20%5Ctexttt%7Bsaturate%7D%20%28%20%5Ctexttt%7Bsrc1%7D%28I%29%20%2B%20%20%5Ctexttt%7Bsrc2%7D%28I%29%29%20%5Cquad%20%5Ctexttt%7Bif%20mask%7D%28I%29%20%5Cne0)
	/// 
	/// The function can be replaced with matrix expressions:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%2B%20%5Ctexttt%7Bsrc2%7D)
	/// 
	/// The input matrices and the output matrix can all have the same or different depths. For example, you
	/// can add a 16-bit unsigned matrix to a 8-bit signed matrix and store the sum as a 32-bit
	/// floating-point matrix. Depth of the output matrix is determined by the ddepth parameter.
	/// If src1.depth() == src2.depth(), ddepth can be set to the default -1. In this case, the output matrix will have
	/// the same depth as the input matrices.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.add"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix.
	/// * ddepth: optional depth of the output matrix.
	/// ## See also
	/// sub, addWeighted
	/// 
	/// ## C++ default parameters
	/// * ddepth: -1
	#[inline]
	pub fn add(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat, ddepth: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_add_const_GMatR_const_GMatR_int(src1.as_raw_GMat(), src2.as_raw_GMat(), ddepth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Applies the bilateral filter to an image.
	/// 
	/// The function applies bilateral filtering to the input image, as described in
	/// <http://www.dai.ed.ac.uk/CVonline/LOCAL_COPIES/MANDUCHI1/Bilateral_Filtering.html>
	/// bilateralFilter can reduce unwanted noise very well while keeping edges fairly sharp. However, it is
	/// very slow compared to most filters.
	/// 
	/// _Sigma values_: For simplicity, you can set the 2 sigma values to be the same. If they are small (\<
	/// 10), the filter will not have much effect, whereas if they are large (\> 150), they will have a very
	/// strong effect, making the image look "cartoonish".
	/// 
	/// _Filter size_: Large filters (d \> 5) are very slow, so it is recommended to use d=5 for real-time
	/// applications, and perhaps d=9 for offline applications that need heavy noise filtering.
	/// 
	/// This filter does not work inplace.
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.filters.bilateralfilter"
	/// 
	/// ## Parameters
	/// * src: Source 8-bit or floating-point, 1-channel or 3-channel image.
	/// * d: Diameter of each pixel neighborhood that is used during filtering. If it is non-positive,
	/// it is computed from sigmaSpace.
	/// * sigmaColor: Filter sigma in the color space. A larger value of the parameter means that
	/// farther colors within the pixel neighborhood (see sigmaSpace) will be mixed together, resulting
	/// in larger areas of semi-equal color.
	/// * sigmaSpace: Filter sigma in the coordinate space. A larger value of the parameter means that
	/// farther pixels will influence each other as long as their colors are close enough (see sigmaColor
	/// ). When d\>0, it specifies the neighborhood size regardless of sigmaSpace. Otherwise, d is
	/// proportional to sigmaSpace.
	/// * borderType: border mode used to extrapolate pixels outside of the image, see [border_types]
	/// ## Returns
	/// Destination image of the same size and type as src.
	/// 
	/// ## Note
	/// This alternative version of [bilateral_filter] function uses the following default values for its arguments:
	/// * border_type: BORDER_DEFAULT
	#[inline]
	pub fn bilateral_filter_def(src: &crate::gapi::GMat, d: i32, sigma_color: f64, sigma_space: f64) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_bilateralFilter_const_GMatR_int_double_double(src.as_raw_GMat(), d, sigma_color, sigma_space, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Applies the bilateral filter to an image.
	/// 
	/// The function applies bilateral filtering to the input image, as described in
	/// <http://www.dai.ed.ac.uk/CVonline/LOCAL_COPIES/MANDUCHI1/Bilateral_Filtering.html>
	/// bilateralFilter can reduce unwanted noise very well while keeping edges fairly sharp. However, it is
	/// very slow compared to most filters.
	/// 
	/// _Sigma values_: For simplicity, you can set the 2 sigma values to be the same. If they are small (\<
	/// 10), the filter will not have much effect, whereas if they are large (\> 150), they will have a very
	/// strong effect, making the image look "cartoonish".
	/// 
	/// _Filter size_: Large filters (d \> 5) are very slow, so it is recommended to use d=5 for real-time
	/// applications, and perhaps d=9 for offline applications that need heavy noise filtering.
	/// 
	/// This filter does not work inplace.
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.filters.bilateralfilter"
	/// 
	/// ## Parameters
	/// * src: Source 8-bit or floating-point, 1-channel or 3-channel image.
	/// * d: Diameter of each pixel neighborhood that is used during filtering. If it is non-positive,
	/// it is computed from sigmaSpace.
	/// * sigmaColor: Filter sigma in the color space. A larger value of the parameter means that
	/// farther colors within the pixel neighborhood (see sigmaSpace) will be mixed together, resulting
	/// in larger areas of semi-equal color.
	/// * sigmaSpace: Filter sigma in the coordinate space. A larger value of the parameter means that
	/// farther pixels will influence each other as long as their colors are close enough (see sigmaColor
	/// ). When d\>0, it specifies the neighborhood size regardless of sigmaSpace. Otherwise, d is
	/// proportional to sigmaSpace.
	/// * borderType: border mode used to extrapolate pixels outside of the image, see [border_types]
	/// ## Returns
	/// Destination image of the same size and type as src.
	/// 
	/// ## C++ default parameters
	/// * border_type: BORDER_DEFAULT
	#[inline]
	pub fn bilateral_filter(src: &crate::gapi::GMat, d: i32, sigma_color: f64, sigma_space: f64, border_type: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_bilateralFilter_const_GMatR_int_double_double_int(src.as_raw_GMat(), d, sigma_color, sigma_space, border_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// computes bitwise conjunction of the two matrixes (src1 & src2)
	/// Calculates the per-element bit-wise logical conjunction of two matrices of the same size.
	/// 
	/// In case of floating-point matrices, their machine-specific bit
	/// representations (usually IEEE754-compliant) are used for the operation.
	/// In case of multi-channel matrices, each channel is processed
	/// independently. Output matrix must have the same size and depth as the input
	/// matrices.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.bitwise_and"
	/// 
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix.
	#[inline]
	pub fn bitwise_and(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_bitwise_and_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// computes bitwise conjunction of the two matrixes (src1 & src2)
	/// Calculates the per-element bit-wise logical conjunction of two matrices of the same size.
	/// 
	/// In case of floating-point matrices, their machine-specific bit
	/// representations (usually IEEE754-compliant) are used for the operation.
	/// In case of multi-channel matrices, each channel is processed
	/// independently. Output matrix must have the same size and depth as the input
	/// matrices.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.bitwise_and"
	/// 
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix.
	/// 
	/// ## Overloaded parameters
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.bitwise_andS"
	/// * src1: first input matrix.
	/// * src2: scalar, which will be per-lemenetly conjuncted with elements of src1.
	#[inline]
	pub fn bitwise_and_1(src1: &crate::gapi::GMat, src2: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_bitwise_and_const_GMatR_const_GScalarR(src1.as_raw_GMat(), src2.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Inverts every bit of an array.
	/// 
	/// The function bitwise_not calculates per-element bit-wise inversion of the input
	/// matrix:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Cneg%20%5Ctexttt%7Bsrc%7D%20%28I%29)
	/// 
	/// In case of floating-point matrices, their machine-specific bit
	/// representations (usually IEEE754-compliant) are used for the operation.
	/// In case of multi-channel matrices, each channel is processed
	/// independently. Output matrix must have the same size and depth as the input
	/// matrix.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.bitwise_not"
	/// 
	/// ## Parameters
	/// * src: input matrix.
	#[inline]
	pub fn bitwise_not(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_bitwise_not_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// computes bitwise disjunction of the two matrixes (src1 | src2)
	/// Calculates the per-element bit-wise logical disjunction of two matrices of the same size.
	/// 
	/// In case of floating-point matrices, their machine-specific bit
	/// representations (usually IEEE754-compliant) are used for the operation.
	/// In case of multi-channel matrices, each channel is processed
	/// independently. Output matrix must have the same size and depth as the input
	/// matrices.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.bitwise_or"
	/// 
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix.
	#[inline]
	pub fn bitwise_or(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_bitwise_or_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// computes bitwise disjunction of the two matrixes (src1 | src2)
	/// Calculates the per-element bit-wise logical disjunction of two matrices of the same size.
	/// 
	/// In case of floating-point matrices, their machine-specific bit
	/// representations (usually IEEE754-compliant) are used for the operation.
	/// In case of multi-channel matrices, each channel is processed
	/// independently. Output matrix must have the same size and depth as the input
	/// matrices.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.bitwise_or"
	/// 
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix.
	/// 
	/// ## Overloaded parameters
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.bitwise_orS"
	/// * src1: first input matrix.
	/// * src2: scalar, which will be per-lemenetly disjuncted with elements of src1.
	#[inline]
	pub fn bitwise_or_1(src1: &crate::gapi::GMat, src2: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_bitwise_or_const_GMatR_const_GScalarR(src1.as_raw_GMat(), src2.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// computes bitwise logical "exclusive or" of the two matrixes (src1 ^ src2)
	/// Calculates the per-element bit-wise logical "exclusive or" of two matrices of the same size.
	/// 
	/// In case of floating-point matrices, their machine-specific bit
	/// representations (usually IEEE754-compliant) are used for the operation.
	/// In case of multi-channel matrices, each channel is processed
	/// independently. Output matrix must have the same size and depth as the input
	/// matrices.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.bitwise_xor"
	/// 
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix.
	#[inline]
	pub fn bitwise_xor(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_bitwise_xor_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// computes bitwise logical "exclusive or" of the two matrixes (src1 ^ src2)
	/// Calculates the per-element bit-wise logical "exclusive or" of two matrices of the same size.
	/// 
	/// In case of floating-point matrices, their machine-specific bit
	/// representations (usually IEEE754-compliant) are used for the operation.
	/// In case of multi-channel matrices, each channel is processed
	/// independently. Output matrix must have the same size and depth as the input
	/// matrices.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.bitwise_xor"
	/// 
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix.
	/// 
	/// ## Overloaded parameters
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.bitwise_xorS"
	/// * src1: first input matrix.
	/// * src2: scalar, for which per-lemenet "logical or" operation on elements of src1 will be performed.
	#[inline]
	pub fn bitwise_xor_1(src1: &crate::gapi::GMat, src2: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_bitwise_xor_const_GMatR_const_GScalarR(src1.as_raw_GMat(), src2.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Blurs an image using the normalized box filter.
	/// 
	/// The function smooths an image using the kernel:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BK%7D%20%3D%20%20%5Cfrac%7B1%7D%7B%5Ctexttt%7Bksize%2Ewidth%2Aksize%2Eheight%7D%7D%20%5Cbegin%7Bbmatrix%7D%201%20%26%201%20%26%201%20%26%20%20%5Ccdots%20%26%201%20%26%201%20%20%5C%5C%201%20%26%201%20%26%201%20%26%20%20%5Ccdots%20%26%201%20%26%201%20%20%5C%5C%20%5Cdots%20%5C%5C%201%20%26%201%20%26%201%20%26%20%20%5Ccdots%20%26%201%20%26%201%20%20%5C%5C%20%5Cend%7Bbmatrix%7D)
	/// 
	/// The call `blur(src, ksize, anchor, borderType)` is equivalent to `boxFilter(src, src.type(), ksize, anchor,
	/// true, borderType)`.
	/// 
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// Output image must have the same type, size, and number of channels as the input image.
	/// 
	/// Note:
	///  - Rounding to nearest even is procedeed if hardware supports it, if not - to nearest.
	///  - Function textual ID is "org.opencv.imgproc.filters.blur"
	/// 
	/// ## Parameters
	/// * src: Source image.
	/// * ksize: blurring kernel size.
	/// * anchor: anchor point; default value Point(-1,-1) means that the anchor is at the kernel
	/// center.
	/// * borderType: border mode used to extrapolate pixels outside of the image, see cv::BorderTypes
	/// * borderValue: border value in case of constant border type
	/// ## See also
	/// boxFilter, bilateralFilter, GaussianBlur, medianBlur
	/// 
	/// ## Note
	/// This alternative version of [blur] function uses the following default values for its arguments:
	/// * anchor: Point(-1,-1)
	/// * border_type: BORDER_DEFAULT
	/// * border_value: Scalar(0)
	#[inline]
	pub fn blur_def(src: &crate::gapi::GMat, ksize: core::Size) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_blur_const_GMatR_const_SizeR(src.as_raw_GMat(), &ksize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Blurs an image using the normalized box filter.
	/// 
	/// The function smooths an image using the kernel:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BK%7D%20%3D%20%20%5Cfrac%7B1%7D%7B%5Ctexttt%7Bksize%2Ewidth%2Aksize%2Eheight%7D%7D%20%5Cbegin%7Bbmatrix%7D%201%20%26%201%20%26%201%20%26%20%20%5Ccdots%20%26%201%20%26%201%20%20%5C%5C%201%20%26%201%20%26%201%20%26%20%20%5Ccdots%20%26%201%20%26%201%20%20%5C%5C%20%5Cdots%20%5C%5C%201%20%26%201%20%26%201%20%26%20%20%5Ccdots%20%26%201%20%26%201%20%20%5C%5C%20%5Cend%7Bbmatrix%7D)
	/// 
	/// The call `blur(src, ksize, anchor, borderType)` is equivalent to `boxFilter(src, src.type(), ksize, anchor,
	/// true, borderType)`.
	/// 
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// Output image must have the same type, size, and number of channels as the input image.
	/// 
	/// Note:
	///  - Rounding to nearest even is procedeed if hardware supports it, if not - to nearest.
	///  - Function textual ID is "org.opencv.imgproc.filters.blur"
	/// 
	/// ## Parameters
	/// * src: Source image.
	/// * ksize: blurring kernel size.
	/// * anchor: anchor point; default value Point(-1,-1) means that the anchor is at the kernel
	/// center.
	/// * borderType: border mode used to extrapolate pixels outside of the image, see cv::BorderTypes
	/// * borderValue: border value in case of constant border type
	/// ## See also
	/// boxFilter, bilateralFilter, GaussianBlur, medianBlur
	/// 
	/// ## C++ default parameters
	/// * anchor: Point(-1,-1)
	/// * border_type: BORDER_DEFAULT
	/// * border_value: Scalar(0)
	#[inline]
	pub fn blur(src: &crate::gapi::GMat, ksize: core::Size, anchor: core::Point, border_type: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_blur_const_GMatR_const_SizeR_const_PointR_int_const_ScalarR(src.as_raw_GMat(), &ksize, &anchor, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Blurs an image using the box filter.
	/// 
	/// The function smooths an image using the kernel:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BK%7D%20%3D%20%20%5Calpha%20%5Cbegin%7Bbmatrix%7D%201%20%26%201%20%26%201%20%26%20%20%5Ccdots%20%26%201%20%26%201%20%20%5C%5C%201%20%26%201%20%26%201%20%26%20%20%5Ccdots%20%26%201%20%26%201%20%20%5C%5C%20%5Cdots%20%5C%5C%201%20%26%201%20%26%201%20%26%20%20%5Ccdots%20%26%201%20%26%201%20%5Cend%7Bbmatrix%7D)
	/// 
	/// where
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Calpha%20%3D%20%5Cbegin%7Bcases%7D%20%5Cfrac%7B1%7D%7B%5Ctexttt%7Bksize%2Ewidth%2Aksize%2Eheight%7D%7D%20%26%20%5Ctexttt%7Bwhen%20%7D%20%5Ctexttt%7Bnormalize%3Dtrue%7D%20%20%5C%5C1%20%26%20%5Ctexttt%7Botherwise%7D%20%5Cend%7Bcases%7D)
	/// 
	/// Unnormalized box filter is useful for computing various integral characteristics over each pixel
	/// neighborhood, such as covariance matrices of image derivatives (used in dense optical flow
	/// algorithms, and so on). If you need to compute pixel sums over variable-size windows, use cv::integral.
	/// 
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// Output image must have the same type, size, and number of channels as the input image.
	/// 
	/// Note:
	///  - Rounding to nearest even is procedeed if hardware supports it, if not - to nearest.
	///  - Function textual ID is "org.opencv.imgproc.filters.boxfilter"
	/// 
	/// ## Parameters
	/// * src: Source image.
	/// * dtype: the output image depth (-1 to set the input image data type).
	/// * ksize: blurring kernel size.
	/// * anchor: Anchor position within the kernel. The default value ![inline formula](https://latex.codecogs.com/png.latex?%28%2D1%2C%2D1%29) means that the anchor
	/// is at the kernel center.
	/// * normalize: flag, specifying whether the kernel is normalized by its area or not.
	/// * borderType: Pixel extrapolation method, see cv::BorderTypes
	/// * borderValue: border value in case of constant border type
	/// ## See also
	/// sepFilter, gaussianBlur, medianBlur, integral
	/// 
	/// ## Note
	/// This alternative version of [box_filter] function uses the following default values for its arguments:
	/// * anchor: Point(-1,-1)
	/// * normalize: true
	/// * border_type: BORDER_DEFAULT
	/// * border_value: Scalar(0)
	#[inline]
	pub fn box_filter_def(src: &crate::gapi::GMat, dtype: i32, ksize: core::Size) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_boxFilter_const_GMatR_int_const_SizeR(src.as_raw_GMat(), dtype, &ksize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Blurs an image using the box filter.
	/// 
	/// The function smooths an image using the kernel:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BK%7D%20%3D%20%20%5Calpha%20%5Cbegin%7Bbmatrix%7D%201%20%26%201%20%26%201%20%26%20%20%5Ccdots%20%26%201%20%26%201%20%20%5C%5C%201%20%26%201%20%26%201%20%26%20%20%5Ccdots%20%26%201%20%26%201%20%20%5C%5C%20%5Cdots%20%5C%5C%201%20%26%201%20%26%201%20%26%20%20%5Ccdots%20%26%201%20%26%201%20%5Cend%7Bbmatrix%7D)
	/// 
	/// where
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Calpha%20%3D%20%5Cbegin%7Bcases%7D%20%5Cfrac%7B1%7D%7B%5Ctexttt%7Bksize%2Ewidth%2Aksize%2Eheight%7D%7D%20%26%20%5Ctexttt%7Bwhen%20%7D%20%5Ctexttt%7Bnormalize%3Dtrue%7D%20%20%5C%5C1%20%26%20%5Ctexttt%7Botherwise%7D%20%5Cend%7Bcases%7D)
	/// 
	/// Unnormalized box filter is useful for computing various integral characteristics over each pixel
	/// neighborhood, such as covariance matrices of image derivatives (used in dense optical flow
	/// algorithms, and so on). If you need to compute pixel sums over variable-size windows, use cv::integral.
	/// 
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// Output image must have the same type, size, and number of channels as the input image.
	/// 
	/// Note:
	///  - Rounding to nearest even is procedeed if hardware supports it, if not - to nearest.
	///  - Function textual ID is "org.opencv.imgproc.filters.boxfilter"
	/// 
	/// ## Parameters
	/// * src: Source image.
	/// * dtype: the output image depth (-1 to set the input image data type).
	/// * ksize: blurring kernel size.
	/// * anchor: Anchor position within the kernel. The default value ![inline formula](https://latex.codecogs.com/png.latex?%28%2D1%2C%2D1%29) means that the anchor
	/// is at the kernel center.
	/// * normalize: flag, specifying whether the kernel is normalized by its area or not.
	/// * borderType: Pixel extrapolation method, see cv::BorderTypes
	/// * borderValue: border value in case of constant border type
	/// ## See also
	/// sepFilter, gaussianBlur, medianBlur, integral
	/// 
	/// ## C++ default parameters
	/// * anchor: Point(-1,-1)
	/// * normalize: true
	/// * border_type: BORDER_DEFAULT
	/// * border_value: Scalar(0)
	#[inline]
	pub fn box_filter(src: &crate::gapi::GMat, dtype: i32, ksize: core::Size, anchor: core::Point, normalize: bool, border_type: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_boxFilter_const_GMatR_int_const_SizeR_const_PointR_bool_int_const_ScalarR(src.as_raw_GMat(), dtype, &ksize, &anchor, normalize, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the magnitude and angle of 2D vectors.
	/// 
	/// The function cartToPolar calculates either the magnitude, angle, or both
	/// for every 2D vector (x(I),y(I)):
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20%5Ctexttt%7Bmagnitude%7D%20%28I%29%3D%20%5Csqrt%7B%5Ctexttt%7Bx%7D%28I%29%5E2%2B%5Ctexttt%7By%7D%28I%29%5E2%7D%20%2C%20%5C%5C%20%5Ctexttt%7Bangle%7D%20%28I%29%3D%20%5Ctexttt%7Batan2%7D%20%28%20%5Ctexttt%7By%7D%20%28I%29%2C%20%5Ctexttt%7Bx%7D%20%28I%29%29%5B%20%5Ccdot180%20%2F%20%5Cpi%20%5D%20%5Cend%7Barray%7D)
	/// 
	/// The angles are calculated with accuracy about 0.3 degrees. For the point
	/// (0,0), the angle is set to 0.
	/// 
	/// First output is a matrix of magnitudes of the same size and depth as input x.
	/// Second output is a matrix of angles that has the same size and depth as
	/// x; the angles are measured in radians (from 0 to 2\*Pi) or in degrees (0 to 360 degrees).
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.cartToPolar"
	/// 
	/// ## Parameters
	/// * x: matrix of [CV_32FC1] x-coordinates.
	/// * y: array of [CV_32FC1] y-coordinates.
	/// * angleInDegrees: a flag, indicating whether the angles are measured
	/// in radians (which is by default), or in degrees.
	/// ## See also
	/// polarToCart
	/// 
	/// ## Note
	/// This alternative version of [cart_to_polar] function uses the following default values for its arguments:
	/// * angle_in_degrees: false
	#[inline]
	pub fn cart_to_polar_def(x: &crate::gapi::GMat, y: &crate::gapi::GMat) -> Result<core::Tuple<(crate::gapi::GMat, crate::gapi::GMat)>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_cartToPolar_const_GMatR_const_GMatR(x.as_raw_GMat(), y.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Tuple::<(crate::gapi::GMat, crate::gapi::GMat)>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the magnitude and angle of 2D vectors.
	/// 
	/// The function cartToPolar calculates either the magnitude, angle, or both
	/// for every 2D vector (x(I),y(I)):
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20%5Ctexttt%7Bmagnitude%7D%20%28I%29%3D%20%5Csqrt%7B%5Ctexttt%7Bx%7D%28I%29%5E2%2B%5Ctexttt%7By%7D%28I%29%5E2%7D%20%2C%20%5C%5C%20%5Ctexttt%7Bangle%7D%20%28I%29%3D%20%5Ctexttt%7Batan2%7D%20%28%20%5Ctexttt%7By%7D%20%28I%29%2C%20%5Ctexttt%7Bx%7D%20%28I%29%29%5B%20%5Ccdot180%20%2F%20%5Cpi%20%5D%20%5Cend%7Barray%7D)
	/// 
	/// The angles are calculated with accuracy about 0.3 degrees. For the point
	/// (0,0), the angle is set to 0.
	/// 
	/// First output is a matrix of magnitudes of the same size and depth as input x.
	/// Second output is a matrix of angles that has the same size and depth as
	/// x; the angles are measured in radians (from 0 to 2\*Pi) or in degrees (0 to 360 degrees).
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.cartToPolar"
	/// 
	/// ## Parameters
	/// * x: matrix of [CV_32FC1] x-coordinates.
	/// * y: array of [CV_32FC1] y-coordinates.
	/// * angleInDegrees: a flag, indicating whether the angles are measured
	/// in radians (which is by default), or in degrees.
	/// ## See also
	/// polarToCart
	/// 
	/// ## C++ default parameters
	/// * angle_in_degrees: false
	#[inline]
	pub fn cart_to_polar(x: &crate::gapi::GMat, y: &crate::gapi::GMat, angle_in_degrees: bool) -> Result<core::Tuple<(crate::gapi::GMat, crate::gapi::GMat)>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_cartToPolar_const_GMatR_const_GMatR_bool(x.as_raw_GMat(), y.as_raw_GMat(), angle_in_degrees, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Tuple::<(crate::gapi::GMat, crate::gapi::GMat)>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Performs the per-element comparison of two matrices checking if elements from first matrix are equal to elements in second.
	/// 
	/// The function compares elements of two matrices src1 and src2 of the same size:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%3D%3D%20%20%5Ctexttt%7Bsrc2%7D%20%28I%29)
	/// 
	/// When the comparison result is true, the corresponding element of output
	/// array is set to 255. The comparison operations can be replaced with the
	/// equivalent matrix expressions:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%20%5Ctexttt%7Bsrc1%7D%20%3D%3D%20%5Ctexttt%7Bsrc2%7D)
	/// 
	/// Output matrix of depth [CV_8U] must have the same size and the same number of channels as
	///    the input matrices.
	/// 
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.compare.cmpEQ"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix/scalar of the same depth as first input matrix.
	/// ## See also
	/// min, max, threshold, cmpNE
	#[inline]
	pub fn cmp_eq(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_cmpEQ_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Performs the per-element comparison of two matrices checking if elements from first matrix are equal to elements in second.
	/// 
	/// The function compares elements of two matrices src1 and src2 of the same size:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%3D%3D%20%20%5Ctexttt%7Bsrc2%7D%20%28I%29)
	/// 
	/// When the comparison result is true, the corresponding element of output
	/// array is set to 255. The comparison operations can be replaced with the
	/// equivalent matrix expressions:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%20%5Ctexttt%7Bsrc1%7D%20%3D%3D%20%5Ctexttt%7Bsrc2%7D)
	/// 
	/// Output matrix of depth [CV_8U] must have the same size and the same number of channels as
	///    the input matrices.
	/// 
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.compare.cmpEQ"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix/scalar of the same depth as first input matrix.
	/// ## See also
	/// min, max, threshold, cmpNE
	/// 
	/// ## Overloaded parameters
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.compare.cmpEQScalar"
	#[inline]
	pub fn cmp_eq_1(src1: &crate::gapi::GMat, src2: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_cmpEQ_const_GMatR_const_GScalarR(src1.as_raw_GMat(), src2.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Performs the per-element comparison of two matrices checking if elements from first matrix are greater or equal compare to elements in second.
	/// 
	/// The function compares elements of two matrices src1 and src2 of the same size:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%3E%3D%20%5Ctexttt%7Bsrc2%7D%20%28I%29)
	/// 
	/// When the comparison result is true, the corresponding element of output
	/// array is set to 255. The comparison operations can be replaced with the
	/// equivalent matrix expressions:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%20%5Ctexttt%7Bsrc1%7D%20%3E%3D%20%5Ctexttt%7Bsrc2%7D)
	/// 
	/// Output matrix of depth [CV_8U] must have the same size and the same number of channels as
	///    the input matrices.
	/// 
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.compare.cmpGE"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix/scalar of the same depth as first input matrix.
	/// ## See also
	/// min, max, threshold, cmpLE, cmpGT, cmpLT
	#[inline]
	pub fn cmp_ge(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_cmpGE_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Performs the per-element comparison of two matrices checking if elements from first matrix are greater or equal compare to elements in second.
	/// 
	/// The function compares elements of two matrices src1 and src2 of the same size:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%3E%3D%20%5Ctexttt%7Bsrc2%7D%20%28I%29)
	/// 
	/// When the comparison result is true, the corresponding element of output
	/// array is set to 255. The comparison operations can be replaced with the
	/// equivalent matrix expressions:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%20%5Ctexttt%7Bsrc1%7D%20%3E%3D%20%5Ctexttt%7Bsrc2%7D)
	/// 
	/// Output matrix of depth [CV_8U] must have the same size and the same number of channels as
	///    the input matrices.
	/// 
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.compare.cmpGE"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix/scalar of the same depth as first input matrix.
	/// ## See also
	/// min, max, threshold, cmpLE, cmpGT, cmpLT
	/// 
	/// ## Overloaded parameters
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.compare.cmpLGEcalar"
	#[inline]
	pub fn cmp_ge_1(src1: &crate::gapi::GMat, src2: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_cmpGE_const_GMatR_const_GScalarR(src1.as_raw_GMat(), src2.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Performs the per-element comparison of two matrices checking if elements from first matrix are greater compare to elements in second.
	/// 
	/// The function compares elements of two matrices src1 and src2 of the same size:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%3E%20%5Ctexttt%7Bsrc2%7D%20%28I%29)
	/// 
	/// When the comparison result is true, the corresponding element of output
	/// array is set to 255. The comparison operations can be replaced with the
	/// equivalent matrix expressions:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%20%5Ctexttt%7Bsrc1%7D%20%3E%20%5Ctexttt%7Bsrc2%7D)
	/// 
	/// Output matrix of depth [CV_8U] must have the same size and the same number of channels as
	///    the input matrices/matrix.
	/// 
	/// Supported input matrix data types are [CV_8UC1], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.compare.cmpGT"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix/scalar of the same depth as first input matrix.
	/// ## See also
	/// min, max, threshold, cmpLE, cmpGE, cmpLT
	#[inline]
	pub fn cmp_gt(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_cmpGT_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Performs the per-element comparison of two matrices checking if elements from first matrix are greater compare to elements in second.
	/// 
	/// The function compares elements of two matrices src1 and src2 of the same size:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%3E%20%5Ctexttt%7Bsrc2%7D%20%28I%29)
	/// 
	/// When the comparison result is true, the corresponding element of output
	/// array is set to 255. The comparison operations can be replaced with the
	/// equivalent matrix expressions:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%20%5Ctexttt%7Bsrc1%7D%20%3E%20%5Ctexttt%7Bsrc2%7D)
	/// 
	/// Output matrix of depth [CV_8U] must have the same size and the same number of channels as
	///    the input matrices/matrix.
	/// 
	/// Supported input matrix data types are [CV_8UC1], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.compare.cmpGT"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix/scalar of the same depth as first input matrix.
	/// ## See also
	/// min, max, threshold, cmpLE, cmpGE, cmpLT
	/// 
	/// ## Overloaded parameters
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.compare.cmpGTScalar"
	#[inline]
	pub fn cmp_gt_1(src1: &crate::gapi::GMat, src2: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_cmpGT_const_GMatR_const_GScalarR(src1.as_raw_GMat(), src2.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Performs the per-element comparison of two matrices checking if elements from first matrix are less or equal compare to elements in second.
	/// 
	/// The function compares elements of two matrices src1 and src2 of the same size:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%3C%3D%20%20%5Ctexttt%7Bsrc2%7D%20%28I%29)
	/// 
	/// When the comparison result is true, the corresponding element of output
	/// array is set to 255. The comparison operations can be replaced with the
	/// equivalent matrix expressions:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%20%5Ctexttt%7Bsrc1%7D%20%3C%3D%20%5Ctexttt%7Bsrc2%7D)
	/// 
	/// Output matrix of depth [CV_8U] must have the same size and the same number of channels as
	///    the input matrices.
	/// 
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.compare.cmpLE"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix/scalar of the same depth as first input matrix.
	/// ## See also
	/// min, max, threshold, cmpGT, cmpGE, cmpLT
	#[inline]
	pub fn cmp_le(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_cmpLE_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Performs the per-element comparison of two matrices checking if elements from first matrix are less or equal compare to elements in second.
	/// 
	/// The function compares elements of two matrices src1 and src2 of the same size:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%3C%3D%20%20%5Ctexttt%7Bsrc2%7D%20%28I%29)
	/// 
	/// When the comparison result is true, the corresponding element of output
	/// array is set to 255. The comparison operations can be replaced with the
	/// equivalent matrix expressions:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%20%5Ctexttt%7Bsrc1%7D%20%3C%3D%20%5Ctexttt%7Bsrc2%7D)
	/// 
	/// Output matrix of depth [CV_8U] must have the same size and the same number of channels as
	///    the input matrices.
	/// 
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.compare.cmpLE"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix/scalar of the same depth as first input matrix.
	/// ## See also
	/// min, max, threshold, cmpGT, cmpGE, cmpLT
	/// 
	/// ## Overloaded parameters
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.compare.cmpLEScalar"
	#[inline]
	pub fn cmp_le_1(src1: &crate::gapi::GMat, src2: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_cmpLE_const_GMatR_const_GScalarR(src1.as_raw_GMat(), src2.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Performs the per-element comparison of two matrices checking if elements from first matrix are less than elements in second.
	/// 
	/// The function compares elements of two matrices src1 and src2 of the same size:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%3C%20%5Ctexttt%7Bsrc2%7D%20%28I%29)
	/// 
	/// When the comparison result is true, the corresponding element of output
	/// array is set to 255. The comparison operations can be replaced with the
	/// equivalent matrix expressions:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%20%5Ctexttt%7Bsrc1%7D%20%3C%20%5Ctexttt%7Bsrc2%7D)
	/// 
	/// Output matrix of depth [CV_8U] must have the same size and the same number of channels as
	///    the input matrices/matrix.
	/// 
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.compare.cmpLT"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix/scalar of the same depth as first input matrix.
	/// ## See also
	/// min, max, threshold, cmpLE, cmpGE, cmpGT
	#[inline]
	pub fn cmp_lt(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_cmpLT_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Performs the per-element comparison of two matrices checking if elements from first matrix are less than elements in second.
	/// 
	/// The function compares elements of two matrices src1 and src2 of the same size:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%3C%20%5Ctexttt%7Bsrc2%7D%20%28I%29)
	/// 
	/// When the comparison result is true, the corresponding element of output
	/// array is set to 255. The comparison operations can be replaced with the
	/// equivalent matrix expressions:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%20%5Ctexttt%7Bsrc1%7D%20%3C%20%5Ctexttt%7Bsrc2%7D)
	/// 
	/// Output matrix of depth [CV_8U] must have the same size and the same number of channels as
	///    the input matrices/matrix.
	/// 
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.compare.cmpLT"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix/scalar of the same depth as first input matrix.
	/// ## See also
	/// min, max, threshold, cmpLE, cmpGE, cmpGT
	/// 
	/// ## Overloaded parameters
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.compare.cmpLTScalar"
	#[inline]
	pub fn cmp_lt_1(src1: &crate::gapi::GMat, src2: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_cmpLT_const_GMatR_const_GScalarR(src1.as_raw_GMat(), src2.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Performs the per-element comparison of two matrices checking if elements from first matrix are not equal to elements in second.
	/// 
	/// The function compares elements of two matrices src1 and src2 of the same size:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%21%3D%20%20%5Ctexttt%7Bsrc2%7D%20%28I%29)
	/// 
	/// When the comparison result is true, the corresponding element of output
	/// array is set to 255. The comparison operations can be replaced with the
	/// equivalent matrix expressions:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%20%5Ctexttt%7Bsrc1%7D%20%21%3D%20%5Ctexttt%7Bsrc2%7D)
	/// 
	/// Output matrix of depth [CV_8U] must have the same size and the same number of channels as
	///    the input matrices.
	/// 
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.compare.cmpNE"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix/scalar of the same depth as first input matrix.
	/// ## See also
	/// min, max, threshold, cmpEQ
	#[inline]
	pub fn cmp_ne(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_cmpNE_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Performs the per-element comparison of two matrices checking if elements from first matrix are not equal to elements in second.
	/// 
	/// The function compares elements of two matrices src1 and src2 of the same size:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%21%3D%20%20%5Ctexttt%7Bsrc2%7D%20%28I%29)
	/// 
	/// When the comparison result is true, the corresponding element of output
	/// array is set to 255. The comparison operations can be replaced with the
	/// equivalent matrix expressions:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%20%5Ctexttt%7Bsrc1%7D%20%21%3D%20%5Ctexttt%7Bsrc2%7D)
	/// 
	/// Output matrix of depth [CV_8U] must have the same size and the same number of channels as
	///    the input matrices.
	/// 
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.compare.cmpNE"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix/scalar of the same depth as first input matrix.
	/// ## See also
	/// min, max, threshold, cmpEQ
	/// 
	/// ## Overloaded parameters
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.compare.cmpNEScalar"
	#[inline]
	pub fn cmp_ne_1(src1: &crate::gapi::GMat, src2: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_cmpNE_const_GMatR_const_GScalarR(src1.as_raw_GMat(), src2.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Create a new package based on `lhs` and `rhs`.
	/// 
	/// ## Parameters
	/// * lhs: "Left-hand-side" package in the process
	/// * rhs: "Right-hand-side" package in the process
	/// ## Returns
	/// a new kernel package.
	#[inline]
	pub fn combine(lhs: &crate::gapi::GKernelPackage, rhs: &crate::gapi::GKernelPackage) -> Result<crate::gapi::GKernelPackage> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_combine_const_GKernelPackageR_const_GKernelPackageR(lhs.as_raw_GKernelPackage(), rhs.as_raw_GKernelPackage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GKernelPackage::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Applies horizontal concatenation to given matrices.
	/// 
	/// The function horizontally concatenates two GMat matrices (with the same number of rows).
	/// ```C++
	///    GMat A = { 1, 4,
	///                2, 5,
	///                3, 6 };
	///    GMat B = { 7, 10,
	///                8, 11,
	///                9, 12 };
	/// 
	///    GMat C = gapi::concatHor(A, B);
	///    //C:
	///    //[1, 4, 7, 10;
	///    // 2, 5, 8, 11;
	///    // 3, 6, 9, 12]
	/// ```
	/// 
	/// Output matrix must the same number of rows and depth as the src1 and src2, and the sum of cols of the src1 and src2.
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.transform.concatHor"
	/// 
	/// ## Parameters
	/// * src1: first input matrix to be considered for horizontal concatenation.
	/// * src2: second input matrix to be considered for horizontal concatenation.
	/// ## See also
	/// concatVert
	#[inline]
	pub fn concat_hor(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_concatHor_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Applies horizontal concatenation to given matrices.
	/// 
	/// The function horizontally concatenates two GMat matrices (with the same number of rows).
	/// ```C++
	///    GMat A = { 1, 4,
	///                2, 5,
	///                3, 6 };
	///    GMat B = { 7, 10,
	///                8, 11,
	///                9, 12 };
	/// 
	///    GMat C = gapi::concatHor(A, B);
	///    //C:
	///    //[1, 4, 7, 10;
	///    // 2, 5, 8, 11;
	///    // 3, 6, 9, 12]
	/// ```
	/// 
	/// Output matrix must the same number of rows and depth as the src1 and src2, and the sum of cols of the src1 and src2.
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.transform.concatHor"
	/// 
	/// ## Parameters
	/// * src1: first input matrix to be considered for horizontal concatenation.
	/// * src2: second input matrix to be considered for horizontal concatenation.
	/// ## See also
	/// concatVert
	/// 
	/// ## Overloaded parameters
	/// 
	/// The function horizontally concatenates given number of GMat matrices (with the same number of columns).
	/// Output matrix must the same number of columns and depth as the input matrices, and the sum of rows of input matrices.
	/// 
	/// * v: vector of input matrices to be concatenated horizontally.
	#[inline]
	pub fn concat_hor_1(v: &core::Vector<crate::gapi::GMat>) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_concatHor_const_vectorLGMatGR(v.as_raw_VectorOfGMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Applies vertical concatenation to given matrices.
	/// 
	/// The function vertically concatenates two GMat matrices (with the same number of cols).
	///  ```C++
	///    GMat A = { 1, 7,
	///                2, 8,
	///                3, 9 };
	///    GMat B = { 4, 10,
	///                5, 11,
	///                6, 12 };
	/// 
	///    GMat C = gapi::concatVert(A, B);
	///    //C:
	///    //[1, 7;
	///    // 2, 8;
	///    // 3, 9;
	///    // 4, 10;
	///    // 5, 11;
	///    // 6, 12]
	///  ```
	/// 
	/// 
	/// Output matrix must the same number of cols and depth as the src1 and src2, and the sum of rows of the src1 and src2.
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.transform.concatVert"
	/// 
	/// ## Parameters
	/// * src1: first input matrix to be considered for vertical concatenation.
	/// * src2: second input matrix to be considered for vertical concatenation.
	/// ## See also
	/// concatHor
	#[inline]
	pub fn concat_vert(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_concatVert_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Applies vertical concatenation to given matrices.
	/// 
	/// The function vertically concatenates two GMat matrices (with the same number of cols).
	///  ```C++
	///    GMat A = { 1, 7,
	///                2, 8,
	///                3, 9 };
	///    GMat B = { 4, 10,
	///                5, 11,
	///                6, 12 };
	/// 
	///    GMat C = gapi::concatVert(A, B);
	///    //C:
	///    //[1, 7;
	///    // 2, 8;
	///    // 3, 9;
	///    // 4, 10;
	///    // 5, 11;
	///    // 6, 12]
	///  ```
	/// 
	/// 
	/// Output matrix must the same number of cols and depth as the src1 and src2, and the sum of rows of the src1 and src2.
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.transform.concatVert"
	/// 
	/// ## Parameters
	/// * src1: first input matrix to be considered for vertical concatenation.
	/// * src2: second input matrix to be considered for vertical concatenation.
	/// ## See also
	/// concatHor
	/// 
	/// ## Overloaded parameters
	/// 
	/// The function vertically concatenates given number of GMat matrices (with the same number of columns).
	/// Output matrix must the same number of columns and depth as the input matrices, and the sum of rows of input matrices.
	/// 
	/// * v: vector of input matrices to be concatenated vertically.
	#[inline]
	pub fn concat_vert_1(v: &core::Vector<crate::gapi::GMat>) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_concatVert_const_vectorLGMatGR(v.as_raw_VectorOfGMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts a matrix to another data depth with optional scaling.
	/// 
	/// The method converts source pixel values to the target data depth. saturate_cast\<\> is applied at
	/// the end to avoid possible overflows:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?m%28x%2Cy%29%20%3D%20saturate%20%5C%5F%20cast%3CrType%3E%28%20%5Calpha%20%28%2Athis%29%28x%2Cy%29%20%2B%20%20%5Cbeta%20%29)
	/// Output matrix must be of the same size as input one.
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.transform.convertTo"
	/// ## Parameters
	/// * src: input matrix to be converted from.
	/// * rdepth: desired output matrix depth or, rather, the depth since the number of channels are the
	/// same as the input has; if rdepth is negative, the output matrix will have the same depth as the input.
	/// * alpha: optional scale factor.
	/// * beta: optional delta added to the scaled values.
	/// 
	/// ## Note
	/// This alternative version of [convert_to] function uses the following default values for its arguments:
	/// * alpha: 1
	/// * beta: 0
	#[inline]
	pub fn convert_to_def(src: &crate::gapi::GMat, rdepth: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_convertTo_const_GMatR_int(src.as_raw_GMat(), rdepth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Converts a matrix to another data depth with optional scaling.
	/// 
	/// The method converts source pixel values to the target data depth. saturate_cast\<\> is applied at
	/// the end to avoid possible overflows:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?m%28x%2Cy%29%20%3D%20saturate%20%5C%5F%20cast%3CrType%3E%28%20%5Calpha%20%28%2Athis%29%28x%2Cy%29%20%2B%20%20%5Cbeta%20%29)
	/// Output matrix must be of the same size as input one.
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.transform.convertTo"
	/// ## Parameters
	/// * src: input matrix to be converted from.
	/// * rdepth: desired output matrix depth or, rather, the depth since the number of channels are the
	/// same as the input has; if rdepth is negative, the output matrix will have the same depth as the input.
	/// * alpha: optional scale factor.
	/// * beta: optional delta added to the scaled values.
	/// 
	/// ## C++ default parameters
	/// * alpha: 1
	/// * beta: 0
	#[inline]
	pub fn convert_to(src: &crate::gapi::GMat, rdepth: i32, alpha: f64, beta: f64) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_convertTo_const_GMatR_int_double_double(src.as_raw_GMat(), rdepth, alpha, beta, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Makes a copy of the input frame. Note that this copy may be not real
	/// (no actual data copied). Use this function to maintain graph contracts,
	/// e.g when graph's input needs to be passed directly to output, like in Streaming mode.
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.streaming.copy"
	/// 
	/// ## Parameters
	/// * in: Input frame
	/// ## Returns
	/// Copy of the input
	#[inline]
	pub fn copy_1(in_: &crate::gapi::GFrame) -> Result<crate::gapi::GFrame> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_copy_const_GFrameR(in_.as_raw_GFrame(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GFrame::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Makes a copy of the input image. Note that this copy may be not real
	/// (no actual data copied). Use this function to maintain graph contracts,
	/// e.g when graph's input needs to be passed directly to output, like in Streaming mode.
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.streaming.copy"
	/// 
	/// ## Parameters
	/// * in: Input image
	/// ## Returns
	/// Copy of the input
	#[inline]
	pub fn copy(in_: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_copy_const_GMatR(in_.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Crops a 2D matrix.
	/// 
	/// The function crops the matrix by given cv::Rect.
	/// 
	/// Output matrix must be of the same depth as input one, size is specified by given rect size.
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.transform.crop"
	/// 
	/// ## Parameters
	/// * src: input matrix.
	/// * rect: a rect to crop a matrix to
	/// ## See also
	/// resize
	#[inline]
	pub fn crop(src: &crate::gapi::GMat, rect: core::Rect) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_crop_const_GMatR_const_RectR(src.as_raw_GMat(), &rect, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Dilates an image by using 3 by 3 rectangular structuring element.
	/// 
	/// The function dilates the source image using the specified structuring element that determines the
	/// shape of a pixel neighborhood over which the maximum is taken:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Cmax%20%5F%7B%28x%27%2Cy%27%29%3A%20%20%5C%2C%20%5Ctexttt%7Belement%7D%20%28x%27%2Cy%27%29%20%5Cne0%20%7D%20%5Ctexttt%7Bsrc%7D%20%28x%2Bx%27%2Cy%2By%27%29)
	/// 
	/// Dilation can be applied several (iterations) times. In case of multi-channel images, each channel is processed independently.
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], and [CV_32FC1].
	/// Output image must have the same type, size, and number of channels as the input image.
	/// 
	/// Note:
	///  - Rounding to nearest even is procedeed if hardware supports it, if not - to nearest.
	///  - Function textual ID is "org.opencv.imgproc.filters.dilate"
	/// 
	/// ## Parameters
	/// * src: input image.
	/// * iterations: number of times dilation is applied.
	/// * borderType: pixel extrapolation method, see cv::BorderTypes
	/// * borderValue: border value in case of a constant border
	/// ## See also
	/// dilate, erode3x3
	/// 
	/// ## Note
	/// This alternative version of [dilate3x3] function uses the following default values for its arguments:
	/// * iterations: 1
	/// * border_type: BORDER_CONSTANT
	/// * border_value: morphologyDefaultBorderValue()
	#[inline]
	pub fn dilate3x3_def(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_dilate3x3_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Dilates an image by using 3 by 3 rectangular structuring element.
	/// 
	/// The function dilates the source image using the specified structuring element that determines the
	/// shape of a pixel neighborhood over which the maximum is taken:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Cmax%20%5F%7B%28x%27%2Cy%27%29%3A%20%20%5C%2C%20%5Ctexttt%7Belement%7D%20%28x%27%2Cy%27%29%20%5Cne0%20%7D%20%5Ctexttt%7Bsrc%7D%20%28x%2Bx%27%2Cy%2By%27%29)
	/// 
	/// Dilation can be applied several (iterations) times. In case of multi-channel images, each channel is processed independently.
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], and [CV_32FC1].
	/// Output image must have the same type, size, and number of channels as the input image.
	/// 
	/// Note:
	///  - Rounding to nearest even is procedeed if hardware supports it, if not - to nearest.
	///  - Function textual ID is "org.opencv.imgproc.filters.dilate"
	/// 
	/// ## Parameters
	/// * src: input image.
	/// * iterations: number of times dilation is applied.
	/// * borderType: pixel extrapolation method, see cv::BorderTypes
	/// * borderValue: border value in case of a constant border
	/// ## See also
	/// dilate, erode3x3
	/// 
	/// ## C++ default parameters
	/// * iterations: 1
	/// * border_type: BORDER_CONSTANT
	/// * border_value: morphologyDefaultBorderValue()
	#[inline]
	pub fn dilate3x3(src: &crate::gapi::GMat, iterations: i32, border_type: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_dilate3x3_const_GMatR_int_int_const_ScalarR(src.as_raw_GMat(), iterations, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Dilates an image by using a specific structuring element.
	/// 
	/// The function dilates the source image using the specified structuring element that determines the
	/// shape of a pixel neighborhood over which the maximum is taken:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Cmax%20%5F%7B%28x%27%2Cy%27%29%3A%20%20%5C%2C%20%5Ctexttt%7Belement%7D%20%28x%27%2Cy%27%29%20%5Cne0%20%7D%20%5Ctexttt%7Bsrc%7D%20%28x%2Bx%27%2Cy%2By%27%29)
	/// 
	/// Dilation can be applied several (iterations) times. In case of multi-channel images, each channel is processed independently.
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], and [CV_32FC1].
	/// Output image must have the same type, size, and number of channels as the input image.
	/// 
	/// Note:
	///  - Rounding to nearest even is procedeed if hardware supports it, if not - to nearest.
	///  - Function textual ID is "org.opencv.imgproc.filters.dilate"
	/// 
	/// ## Parameters
	/// * src: input image.
	/// * kernel: structuring element used for dilation; if elemenat=Mat(), a 3 x 3 rectangular
	/// structuring element is used. Kernel can be created using getStructuringElement
	/// * anchor: position of the anchor within the element; default value (-1, -1) means that the
	/// anchor is at the element center.
	/// * iterations: number of times dilation is applied.
	/// * borderType: pixel extrapolation method, see cv::BorderTypes
	/// * borderValue: border value in case of a constant border
	/// ## See also
	/// erode, morphologyEx, getStructuringElement
	/// 
	/// ## Note
	/// This alternative version of [dilate] function uses the following default values for its arguments:
	/// * anchor: Point(-1,-1)
	/// * iterations: 1
	/// * border_type: BORDER_CONSTANT
	/// * border_value: morphologyDefaultBorderValue()
	#[inline]
	pub fn dilate_def(src: &crate::gapi::GMat, kernel: &core::Mat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_dilate_const_GMatR_const_MatR(src.as_raw_GMat(), kernel.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Dilates an image by using a specific structuring element.
	/// 
	/// The function dilates the source image using the specified structuring element that determines the
	/// shape of a pixel neighborhood over which the maximum is taken:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Cmax%20%5F%7B%28x%27%2Cy%27%29%3A%20%20%5C%2C%20%5Ctexttt%7Belement%7D%20%28x%27%2Cy%27%29%20%5Cne0%20%7D%20%5Ctexttt%7Bsrc%7D%20%28x%2Bx%27%2Cy%2By%27%29)
	/// 
	/// Dilation can be applied several (iterations) times. In case of multi-channel images, each channel is processed independently.
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], and [CV_32FC1].
	/// Output image must have the same type, size, and number of channels as the input image.
	/// 
	/// Note:
	///  - Rounding to nearest even is procedeed if hardware supports it, if not - to nearest.
	///  - Function textual ID is "org.opencv.imgproc.filters.dilate"
	/// 
	/// ## Parameters
	/// * src: input image.
	/// * kernel: structuring element used for dilation; if elemenat=Mat(), a 3 x 3 rectangular
	/// structuring element is used. Kernel can be created using getStructuringElement
	/// * anchor: position of the anchor within the element; default value (-1, -1) means that the
	/// anchor is at the element center.
	/// * iterations: number of times dilation is applied.
	/// * borderType: pixel extrapolation method, see cv::BorderTypes
	/// * borderValue: border value in case of a constant border
	/// ## See also
	/// erode, morphologyEx, getStructuringElement
	/// 
	/// ## C++ default parameters
	/// * anchor: Point(-1,-1)
	/// * iterations: 1
	/// * border_type: BORDER_CONSTANT
	/// * border_value: morphologyDefaultBorderValue()
	#[inline]
	pub fn dilate(src: &crate::gapi::GMat, kernel: &core::Mat, anchor: core::Point, iterations: i32, border_type: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_dilate_const_GMatR_const_MatR_const_PointR_int_int_const_ScalarR(src.as_raw_GMat(), kernel.as_raw_Mat(), &anchor, iterations, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Divides matrix by scalar.
	/// 
	/// The function divC divides each element of matrix src by given scalar value:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%28I%29%20%3D%20saturate%28src%28I%29%2Ascale%2Fdivisor%29%7D)
	/// 
	/// When divisor is zero, dst(I) will also be zero. Different channels of
	/// multi-channel matrices are processed independently.
	/// The matrices can be single or multi channel. Output matrix must have the same size and depth as src.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.divC"
	/// ## Parameters
	/// * src: input matrix.
	/// * divisor: number to be divided by.
	/// * ddepth: optional depth of the output matrix. If -1, the depth of output matrix will be the same as input matrix depth.
	/// * scale: scale factor.
	/// ## See also
	/// add, sub, div, addWeighted
	/// 
	/// ## Note
	/// This alternative version of [div_c] function uses the following default values for its arguments:
	/// * ddepth: -1
	#[inline]
	pub fn div_c_def(src: &crate::gapi::GMat, divisor: &crate::gapi::GScalar, scale: f64) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_divC_const_GMatR_const_GScalarR_double(src.as_raw_GMat(), divisor.as_raw_GScalar(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Divides matrix by scalar.
	/// 
	/// The function divC divides each element of matrix src by given scalar value:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%28I%29%20%3D%20saturate%28src%28I%29%2Ascale%2Fdivisor%29%7D)
	/// 
	/// When divisor is zero, dst(I) will also be zero. Different channels of
	/// multi-channel matrices are processed independently.
	/// The matrices can be single or multi channel. Output matrix must have the same size and depth as src.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.divC"
	/// ## Parameters
	/// * src: input matrix.
	/// * divisor: number to be divided by.
	/// * ddepth: optional depth of the output matrix. If -1, the depth of output matrix will be the same as input matrix depth.
	/// * scale: scale factor.
	/// ## See also
	/// add, sub, div, addWeighted
	/// 
	/// ## C++ default parameters
	/// * ddepth: -1
	#[inline]
	pub fn div_c(src: &crate::gapi::GMat, divisor: &crate::gapi::GScalar, scale: f64, ddepth: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_divC_const_GMatR_const_GScalarR_double_int(src.as_raw_GMat(), divisor.as_raw_GScalar(), scale, ddepth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Divides scalar by matrix.
	/// 
	/// The function divRC divides given scalar by each element of matrix src and keep the division result in new matrix of the same size and type as src:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%28I%29%20%3D%20saturate%28divident%2Ascale%2Fsrc%28I%29%29%7D)
	/// 
	/// When src(I) is zero, dst(I) will also be zero. Different channels of
	/// multi-channel matrices are processed independently.
	/// The matrices can be single or multi channel. Output matrix must have the same size and depth as src.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.divRC"
	/// ## Parameters
	/// * src: input matrix.
	/// * divident: number to be divided.
	/// * ddepth: optional depth of the output matrix. If -1, the depth of output matrix will be the same as input matrix depth.
	/// * scale: scale factor
	/// ## See also
	/// add, sub, div, addWeighted
	/// 
	/// ## Note
	/// This alternative version of [div_rc] function uses the following default values for its arguments:
	/// * ddepth: -1
	#[inline]
	pub fn div_rc_def(divident: &crate::gapi::GScalar, src: &crate::gapi::GMat, scale: f64) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_divRC_const_GScalarR_const_GMatR_double(divident.as_raw_GScalar(), src.as_raw_GMat(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Divides scalar by matrix.
	/// 
	/// The function divRC divides given scalar by each element of matrix src and keep the division result in new matrix of the same size and type as src:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%28I%29%20%3D%20saturate%28divident%2Ascale%2Fsrc%28I%29%29%7D)
	/// 
	/// When src(I) is zero, dst(I) will also be zero. Different channels of
	/// multi-channel matrices are processed independently.
	/// The matrices can be single or multi channel. Output matrix must have the same size and depth as src.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.divRC"
	/// ## Parameters
	/// * src: input matrix.
	/// * divident: number to be divided.
	/// * ddepth: optional depth of the output matrix. If -1, the depth of output matrix will be the same as input matrix depth.
	/// * scale: scale factor
	/// ## See also
	/// add, sub, div, addWeighted
	/// 
	/// ## C++ default parameters
	/// * ddepth: -1
	#[inline]
	pub fn div_rc(divident: &crate::gapi::GScalar, src: &crate::gapi::GMat, scale: f64, ddepth: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_divRC_const_GScalarR_const_GMatR_double_int(divident.as_raw_GScalar(), src.as_raw_GMat(), scale, ddepth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Performs per-element division of two matrices.
	/// 
	/// The function divides one matrix by another:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%28I%29%20%3D%20saturate%28src1%28I%29%2Ascale%2Fsrc2%28I%29%29%7D)
	/// 
	/// For integer types when src2(I) is zero, dst(I) will also be zero.
	/// Floating point case returns Inf/NaN (according to IEEE).
	/// 
	/// Different channels of
	/// multi-channel matrices are processed independently.
	/// The matrices can be single or multi channel. Output matrix must have the same size and depth as src.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.div"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix of the same size and depth as src1.
	/// * scale: scalar factor.
	/// * ddepth: optional depth of the output matrix; you can only pass -1 when src1.depth() == src2.depth().
	/// ## See also
	/// mul, add, sub
	/// 
	/// ## Note
	/// This alternative version of [div] function uses the following default values for its arguments:
	/// * ddepth: -1
	#[inline]
	pub fn div_def(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat, scale: f64) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_div_const_GMatR_const_GMatR_double(src1.as_raw_GMat(), src2.as_raw_GMat(), scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Performs per-element division of two matrices.
	/// 
	/// The function divides one matrix by another:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%28I%29%20%3D%20saturate%28src1%28I%29%2Ascale%2Fsrc2%28I%29%29%7D)
	/// 
	/// For integer types when src2(I) is zero, dst(I) will also be zero.
	/// Floating point case returns Inf/NaN (according to IEEE).
	/// 
	/// Different channels of
	/// multi-channel matrices are processed independently.
	/// The matrices can be single or multi channel. Output matrix must have the same size and depth as src.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.div"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix of the same size and depth as src1.
	/// * scale: scalar factor.
	/// * ddepth: optional depth of the output matrix; you can only pass -1 when src1.depth() == src2.depth().
	/// ## See also
	/// mul, add, sub
	/// 
	/// ## C++ default parameters
	/// * ddepth: -1
	#[inline]
	pub fn div(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat, scale: f64, ddepth: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_div_const_GMatR_const_GMatR_double_int(src1.as_raw_GMat(), src2.as_raw_GMat(), scale, ddepth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Equalizes the histogram of a grayscale image.
	/// 
	/// //!
	/// 
	/// The function equalizes the histogram of the input image using the following algorithm:
	/// 
	/// - Calculate the histogram ![inline formula](https://latex.codecogs.com/png.latex?H) for src .
	/// - Normalize the histogram so that the sum of histogram bins is 255.
	/// - Compute the integral of the histogram:
	/// ![block formula](https://latex.codecogs.com/png.latex?H%27%5Fi%20%3D%20%20%5Csum%20%5F%7B0%20%20%5Cle%20j%20%3C%20i%7D%20H%28j%29)
	/// - Transform the image using ![inline formula](https://latex.codecogs.com/png.latex?H%27) as a look-up table: ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%28x%2Cy%29%20%3D%20H%27%28%5Ctexttt%7Bsrc%7D%28x%2Cy%29%29)
	/// 
	/// The algorithm normalizes the brightness and increases the contrast of the image.
	/// 
	/// Note:
	///  - The returned image is of the same size and type as input.
	///  - Function textual ID is "org.opencv.imgproc.equalizeHist"
	/// 
	/// ## Parameters
	/// * src: Source 8-bit single channel image.
	#[inline]
	pub fn equalize_hist(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_equalizeHist_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Erodes an image by using 3 by 3 rectangular structuring element.
	/// 
	/// The function erodes the source image using the rectangular structuring element with rectangle center as an anchor.
	/// Erosion can be applied several (iterations) times. In case of multi-channel images, each channel is processed independently.
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], and [CV_32FC1].
	/// Output image must have the same type, size, and number of channels as the input image.
	/// 
	/// Note:
	///  - Rounding to nearest even is procedeed if hardware supports it, if not - to nearest.
	///  - Function textual ID is "org.opencv.imgproc.filters.erode"
	/// 
	/// ## Parameters
	/// * src: input image
	/// * iterations: number of times erosion is applied.
	/// * borderType: pixel extrapolation method, see cv::BorderTypes
	/// * borderValue: border value in case of a constant border
	/// ## See also
	/// erode, dilate3x3
	/// 
	/// ## Note
	/// This alternative version of [erode3x3] function uses the following default values for its arguments:
	/// * iterations: 1
	/// * border_type: BORDER_CONSTANT
	/// * border_value: morphologyDefaultBorderValue()
	#[inline]
	pub fn erode3x3_def(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_erode3x3_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Erodes an image by using 3 by 3 rectangular structuring element.
	/// 
	/// The function erodes the source image using the rectangular structuring element with rectangle center as an anchor.
	/// Erosion can be applied several (iterations) times. In case of multi-channel images, each channel is processed independently.
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], and [CV_32FC1].
	/// Output image must have the same type, size, and number of channels as the input image.
	/// 
	/// Note:
	///  - Rounding to nearest even is procedeed if hardware supports it, if not - to nearest.
	///  - Function textual ID is "org.opencv.imgproc.filters.erode"
	/// 
	/// ## Parameters
	/// * src: input image
	/// * iterations: number of times erosion is applied.
	/// * borderType: pixel extrapolation method, see cv::BorderTypes
	/// * borderValue: border value in case of a constant border
	/// ## See also
	/// erode, dilate3x3
	/// 
	/// ## C++ default parameters
	/// * iterations: 1
	/// * border_type: BORDER_CONSTANT
	/// * border_value: morphologyDefaultBorderValue()
	#[inline]
	pub fn erode3x3(src: &crate::gapi::GMat, iterations: i32, border_type: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_erode3x3_const_GMatR_int_int_const_ScalarR(src.as_raw_GMat(), iterations, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Erodes an image by using a specific structuring element.
	/// 
	/// The function erodes the source image using the specified structuring element that determines the
	/// shape of a pixel neighborhood over which the minimum is taken:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Cmin%20%5F%7B%28x%27%2Cy%27%29%3A%20%20%5C%2C%20%5Ctexttt%7Belement%7D%20%28x%27%2Cy%27%29%20%5Cne0%20%7D%20%5Ctexttt%7Bsrc%7D%20%28x%2Bx%27%2Cy%2By%27%29)
	/// 
	/// Erosion can be applied several (iterations) times. In case of multi-channel images, each channel is processed independently.
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], and [CV_32FC1].
	/// Output image must have the same type, size, and number of channels as the input image.
	/// 
	/// Note:
	///  - Rounding to nearest even is procedeed if hardware supports it, if not - to nearest.
	///  - Function textual ID is "org.opencv.imgproc.filters.erode"
	/// 
	/// ## Parameters
	/// * src: input image
	/// * kernel: structuring element used for erosion; if `element=Mat()`, a `3 x 3` rectangular
	/// structuring element is used. Kernel can be created using getStructuringElement.
	/// * anchor: position of the anchor within the element; default value (-1, -1) means that the
	/// anchor is at the element center.
	/// * iterations: number of times erosion is applied.
	/// * borderType: pixel extrapolation method, see cv::BorderTypes
	/// * borderValue: border value in case of a constant border
	/// ## See also
	/// dilate, morphologyEx
	/// 
	/// ## Note
	/// This alternative version of [erode] function uses the following default values for its arguments:
	/// * anchor: Point(-1,-1)
	/// * iterations: 1
	/// * border_type: BORDER_CONSTANT
	/// * border_value: morphologyDefaultBorderValue()
	#[inline]
	pub fn erode_def(src: &crate::gapi::GMat, kernel: &core::Mat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_erode_const_GMatR_const_MatR(src.as_raw_GMat(), kernel.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Erodes an image by using a specific structuring element.
	/// 
	/// The function erodes the source image using the specified structuring element that determines the
	/// shape of a pixel neighborhood over which the minimum is taken:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Cmin%20%5F%7B%28x%27%2Cy%27%29%3A%20%20%5C%2C%20%5Ctexttt%7Belement%7D%20%28x%27%2Cy%27%29%20%5Cne0%20%7D%20%5Ctexttt%7Bsrc%7D%20%28x%2Bx%27%2Cy%2By%27%29)
	/// 
	/// Erosion can be applied several (iterations) times. In case of multi-channel images, each channel is processed independently.
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], and [CV_32FC1].
	/// Output image must have the same type, size, and number of channels as the input image.
	/// 
	/// Note:
	///  - Rounding to nearest even is procedeed if hardware supports it, if not - to nearest.
	///  - Function textual ID is "org.opencv.imgproc.filters.erode"
	/// 
	/// ## Parameters
	/// * src: input image
	/// * kernel: structuring element used for erosion; if `element=Mat()`, a `3 x 3` rectangular
	/// structuring element is used. Kernel can be created using getStructuringElement.
	/// * anchor: position of the anchor within the element; default value (-1, -1) means that the
	/// anchor is at the element center.
	/// * iterations: number of times erosion is applied.
	/// * borderType: pixel extrapolation method, see cv::BorderTypes
	/// * borderValue: border value in case of a constant border
	/// ## See also
	/// dilate, morphologyEx
	/// 
	/// ## C++ default parameters
	/// * anchor: Point(-1,-1)
	/// * iterations: 1
	/// * border_type: BORDER_CONSTANT
	/// * border_value: morphologyDefaultBorderValue()
	#[inline]
	pub fn erode(src: &crate::gapi::GMat, kernel: &core::Mat, anchor: core::Point, iterations: i32, border_type: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_erode_const_GMatR_const_MatR_const_PointR_int_int_const_ScalarR(src.as_raw_GMat(), kernel.as_raw_Mat(), &anchor, iterations, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Convolves an image with the kernel.
	/// 
	/// The function applies an arbitrary linear filter to an image. When
	/// the aperture is partially outside the image, the function interpolates outlier pixel values
	/// according to the specified border mode.
	/// 
	/// The function does actually compute correlation, not the convolution:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Csum%20%5F%7B%20%5Csubstack%7B0%5Cleq%20x%27%20%3C%20%5Ctexttt%7Bkernel%2Ecols%7D%5C%5C%7B0%5Cleq%20y%27%20%3C%20%5Ctexttt%7Bkernel%2Erows%7D%7D%7D%7D%20%20%5Ctexttt%7Bkernel%7D%20%28x%27%2Cy%27%29%2A%20%5Ctexttt%7Bsrc%7D%20%28x%2Bx%27%2D%20%5Ctexttt%7Banchor%2Ex%7D%20%2Cy%2By%27%2D%20%5Ctexttt%7Banchor%2Ey%7D%20%29)
	/// 
	/// That is, the kernel is not mirrored around the anchor point. If you need a real convolution, flip
	/// the kernel using flip and set the new anchor to `(kernel.cols - anchor.x - 1, kernel.rows -
	/// anchor.y - 1)`.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// Output image must have the same size and number of channels an input image.
	/// 
	/// Note:
	///  - Rounding to nearest even is procedeed if hardware supports it, if not - to nearest.
	///  - Function textual ID is "org.opencv.imgproc.filters.filter2D"
	/// 
	/// ## Parameters
	/// * src: input image.
	/// * ddepth: desired depth of the destination image
	/// * kernel: convolution kernel (or rather a correlation kernel), a single-channel floating point
	/// matrix; if you want to apply different kernels to different channels, split the image into
	/// separate color planes using split and process them individually.
	/// * anchor: anchor of the kernel that indicates the relative position of a filtered point within
	/// the kernel; the anchor should lie within the kernel; default value (-1,-1) means that the anchor
	/// is at the kernel center.
	/// * delta: optional value added to the filtered pixels before storing them in dst.
	/// * borderType: pixel extrapolation method, see cv::BorderTypes
	/// * borderValue: border value in case of constant border type
	/// ## See also
	/// sepFilter
	/// 
	/// ## Note
	/// This alternative version of [filter_2d] function uses the following default values for its arguments:
	/// * anchor: Point(-1,-1)
	/// * delta: Scalar(0)
	/// * border_type: BORDER_DEFAULT
	/// * border_value: Scalar(0)
	#[inline]
	pub fn filter_2d_def(src: &crate::gapi::GMat, ddepth: i32, kernel: &core::Mat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_filter2D_const_GMatR_int_const_MatR(src.as_raw_GMat(), ddepth, kernel.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Convolves an image with the kernel.
	/// 
	/// The function applies an arbitrary linear filter to an image. When
	/// the aperture is partially outside the image, the function interpolates outlier pixel values
	/// according to the specified border mode.
	/// 
	/// The function does actually compute correlation, not the convolution:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Csum%20%5F%7B%20%5Csubstack%7B0%5Cleq%20x%27%20%3C%20%5Ctexttt%7Bkernel%2Ecols%7D%5C%5C%7B0%5Cleq%20y%27%20%3C%20%5Ctexttt%7Bkernel%2Erows%7D%7D%7D%7D%20%20%5Ctexttt%7Bkernel%7D%20%28x%27%2Cy%27%29%2A%20%5Ctexttt%7Bsrc%7D%20%28x%2Bx%27%2D%20%5Ctexttt%7Banchor%2Ex%7D%20%2Cy%2By%27%2D%20%5Ctexttt%7Banchor%2Ey%7D%20%29)
	/// 
	/// That is, the kernel is not mirrored around the anchor point. If you need a real convolution, flip
	/// the kernel using flip and set the new anchor to `(kernel.cols - anchor.x - 1, kernel.rows -
	/// anchor.y - 1)`.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// Output image must have the same size and number of channels an input image.
	/// 
	/// Note:
	///  - Rounding to nearest even is procedeed if hardware supports it, if not - to nearest.
	///  - Function textual ID is "org.opencv.imgproc.filters.filter2D"
	/// 
	/// ## Parameters
	/// * src: input image.
	/// * ddepth: desired depth of the destination image
	/// * kernel: convolution kernel (or rather a correlation kernel), a single-channel floating point
	/// matrix; if you want to apply different kernels to different channels, split the image into
	/// separate color planes using split and process them individually.
	/// * anchor: anchor of the kernel that indicates the relative position of a filtered point within
	/// the kernel; the anchor should lie within the kernel; default value (-1,-1) means that the anchor
	/// is at the kernel center.
	/// * delta: optional value added to the filtered pixels before storing them in dst.
	/// * borderType: pixel extrapolation method, see cv::BorderTypes
	/// * borderValue: border value in case of constant border type
	/// ## See also
	/// sepFilter
	/// 
	/// ## C++ default parameters
	/// * anchor: Point(-1,-1)
	/// * delta: Scalar(0)
	/// * border_type: BORDER_DEFAULT
	/// * border_value: Scalar(0)
	#[inline]
	pub fn filter_2d(src: &crate::gapi::GMat, ddepth: i32, kernel: &core::Mat, anchor: core::Point, delta: core::Scalar, border_type: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_filter2D_const_GMatR_int_const_MatR_const_PointR_const_ScalarR_int_const_ScalarR(src.as_raw_GMat(), ddepth, kernel.as_raw_Mat(), &anchor, &delta, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Flips a 2D matrix around vertical, horizontal, or both axes.
	/// 
	/// The function flips the matrix in one of three different ways (row
	/// and column indices are 0-based):
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%5F%7Bij%7D%20%3D%0A%5Cleft%5C%7B%0A%5Cbegin%7Barray%7D%7Bl%20l%7D%0A%5Ctexttt%7Bsrc%7D%20%5F%7B%5Ctexttt%7Bsrc%2Erows%7D%2Di%2D1%2Cj%7D%20%26%20if%5C%3B%20%20%5Ctexttt%7BflipCode%7D%20%3D%200%20%5C%5C%0A%5Ctexttt%7Bsrc%7D%20%5F%7Bi%2C%20%5Ctexttt%7Bsrc%2Ecols%7D%20%2Dj%2D1%7D%20%26%20if%5C%3B%20%20%5Ctexttt%7BflipCode%7D%20%3E%200%20%5C%5C%0A%5Ctexttt%7Bsrc%7D%20%5F%7B%20%5Ctexttt%7Bsrc%2Erows%7D%20%2Di%2D1%2C%20%5Ctexttt%7Bsrc%2Ecols%7D%20%2Dj%2D1%7D%20%26%20if%5C%3B%20%5Ctexttt%7BflipCode%7D%20%3C%200%20%5C%5C%0A%5Cend%7Barray%7D%0A%5Cright%2E)
	/// The example scenarios of using the function are the following:
	/// *   Vertical flipping of the image (flipCode == 0) to switch between
	///    top-left and bottom-left image origin. This is a typical operation
	///    in video processing on Microsoft Windows\* OS.
	/// *   Horizontal flipping of the image with the subsequent horizontal
	///    shift and absolute difference calculation to check for a
	///    vertical-axis symmetry (flipCode \> 0).
	/// *   Simultaneous horizontal and vertical flipping of the image with
	///    the subsequent shift and absolute difference calculation to check
	///    for a central symmetry (flipCode \< 0).
	/// *   Reversing the order of point arrays (flipCode \> 0 or
	///    flipCode == 0).
	/// Output image must be of the same depth as input one, size should be correct for given flipCode.
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.transform.flip"
	/// 
	/// ## Parameters
	/// * src: input matrix.
	/// * flipCode: a flag to specify how to flip the array; 0 means
	/// flipping around the x-axis and positive value (for example, 1) means
	/// flipping around y-axis. Negative value (for example, -1) means flipping
	/// around both axes.
	/// ## See also
	/// remap
	#[inline]
	pub fn flip(src: &crate::gapi::GMat, flip_code: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_flip_const_GMatR_int(src.as_raw_GMat(), flip_code, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Blurs an image using a Gaussian filter.
	/// 
	/// The function filter2Ds the source image with the specified Gaussian kernel.
	/// Output image must have the same type and number of channels an input image.
	/// 
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// Output image must have the same type, size, and number of channels as the input image.
	/// 
	/// Note:
	///  - Rounding to nearest even is procedeed if hardware supports it, if not - to nearest.
	///  - Function textual ID is "org.opencv.imgproc.filters.gaussianBlur"
	/// 
	/// ## Parameters
	/// * src: input image;
	/// * ksize: Gaussian kernel size. ksize.width and ksize.height can differ but they both must be
	/// positive and odd. Or, they can be zero's and then they are computed from sigma.
	/// * sigmaX: Gaussian kernel standard deviation in X direction.
	/// * sigmaY: Gaussian kernel standard deviation in Y direction; if sigmaY is zero, it is set to be
	/// equal to sigmaX, if both sigmas are zeros, they are computed from ksize.width and ksize.height,
	/// respectively (see cv::getGaussianKernel for details); to fully control the result regardless of
	/// possible future modifications of all this semantics, it is recommended to specify all of ksize,
	/// sigmaX, and sigmaY.
	/// * borderType: pixel extrapolation method, see cv::BorderTypes
	/// * borderValue: border value in case of constant border type
	/// ## See also
	/// sepFilter, boxFilter, medianBlur
	/// 
	/// ## Note
	/// This alternative version of [gaussian_blur] function uses the following default values for its arguments:
	/// * sigma_y: 0
	/// * border_type: BORDER_DEFAULT
	/// * border_value: Scalar(0)
	#[inline]
	pub fn gaussian_blur_def(src: &crate::gapi::GMat, ksize: core::Size, sigma_x: f64) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_gaussianBlur_const_GMatR_const_SizeR_double(src.as_raw_GMat(), &ksize, sigma_x, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Blurs an image using a Gaussian filter.
	/// 
	/// The function filter2Ds the source image with the specified Gaussian kernel.
	/// Output image must have the same type and number of channels an input image.
	/// 
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// Output image must have the same type, size, and number of channels as the input image.
	/// 
	/// Note:
	///  - Rounding to nearest even is procedeed if hardware supports it, if not - to nearest.
	///  - Function textual ID is "org.opencv.imgproc.filters.gaussianBlur"
	/// 
	/// ## Parameters
	/// * src: input image;
	/// * ksize: Gaussian kernel size. ksize.width and ksize.height can differ but they both must be
	/// positive and odd. Or, they can be zero's and then they are computed from sigma.
	/// * sigmaX: Gaussian kernel standard deviation in X direction.
	/// * sigmaY: Gaussian kernel standard deviation in Y direction; if sigmaY is zero, it is set to be
	/// equal to sigmaX, if both sigmas are zeros, they are computed from ksize.width and ksize.height,
	/// respectively (see cv::getGaussianKernel for details); to fully control the result regardless of
	/// possible future modifications of all this semantics, it is recommended to specify all of ksize,
	/// sigmaX, and sigmaY.
	/// * borderType: pixel extrapolation method, see cv::BorderTypes
	/// * borderValue: border value in case of constant border type
	/// ## See also
	/// sepFilter, boxFilter, medianBlur
	/// 
	/// ## C++ default parameters
	/// * sigma_y: 0
	/// * border_type: BORDER_DEFAULT
	/// * border_value: Scalar(0)
	#[inline]
	pub fn gaussian_blur(src: &crate::gapi::GMat, ksize: core::Size, sigma_x: f64, sigma_y: f64, border_type: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_gaussianBlur_const_GMatR_const_SizeR_double_double_int_const_ScalarR(src.as_raw_GMat(), &ksize, sigma_x, sigma_y, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Applies a range-level threshold to each matrix element.
	/// 
	/// The function applies range-level thresholding to a single- or multiple-channel matrix.
	/// It sets output pixel value to OxFF if the corresponding pixel value of input matrix is in specified range,or 0 otherwise.
	/// 
	/// Input and output matrices must be CV_8UC1.
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.matrixop.inRange"
	/// 
	/// ## Parameters
	/// * src: input matrix (CV_8UC1).
	/// * threshLow: lower boundary value.
	/// * threshUp: upper boundary value.
	/// ## See also
	/// threshold
	#[inline]
	pub fn in_range(src: &crate::gapi::GMat, thresh_low: &crate::gapi::GScalar, thresh_up: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_inRange_const_GMatR_const_GScalarR_const_GScalarR(src.as_raw_GMat(), thresh_low.as_raw_GScalar(), thresh_up.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the integral of an image.
	/// 
	/// The function calculates one or more integral images for the source image as follows:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bsum%7D%20%28X%2CY%29%20%3D%20%20%5Csum%20%5F%7Bx%3CX%2Cy%3CY%7D%20%20%5Ctexttt%7Bimage%7D%20%28x%2Cy%29)
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bsqsum%7D%20%28X%2CY%29%20%3D%20%20%5Csum%20%5F%7Bx%3CX%2Cy%3CY%7D%20%20%5Ctexttt%7Bimage%7D%20%28x%2Cy%29%5E2)
	/// 
	/// The function return integral image as ![inline formula](https://latex.codecogs.com/png.latex?%28W%2B1%29%5Ctimes%20%28H%2B1%29) , 32-bit integer or floating-point (32f or 64f) and
	///  integral image for squared pixel values; it is ![inline formula](https://latex.codecogs.com/png.latex?%28W%2B1%29%5Ctimes%20%28H%2B%29), double-precision floating-point (64f) array.
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.matrixop.integral"
	/// 
	/// ## Parameters
	/// * src: input image.
	/// * sdepth: desired depth of the integral and the tilted integral images, CV_32S, CV_32F, or
	/// CV_64F.
	/// * sqdepth: desired depth of the integral image of squared pixel values, CV_32F or CV_64F.
	/// 
	/// ## Note
	/// This alternative version of [integral] function uses the following default values for its arguments:
	/// * sdepth: -1
	/// * sqdepth: -1
	#[inline]
	pub fn integral_def(src: &crate::gapi::GMat) -> Result<core::Tuple<(crate::gapi::GMat, crate::gapi::GMat)>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_integral_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Tuple::<(crate::gapi::GMat, crate::gapi::GMat)>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the integral of an image.
	/// 
	/// The function calculates one or more integral images for the source image as follows:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bsum%7D%20%28X%2CY%29%20%3D%20%20%5Csum%20%5F%7Bx%3CX%2Cy%3CY%7D%20%20%5Ctexttt%7Bimage%7D%20%28x%2Cy%29)
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bsqsum%7D%20%28X%2CY%29%20%3D%20%20%5Csum%20%5F%7Bx%3CX%2Cy%3CY%7D%20%20%5Ctexttt%7Bimage%7D%20%28x%2Cy%29%5E2)
	/// 
	/// The function return integral image as ![inline formula](https://latex.codecogs.com/png.latex?%28W%2B1%29%5Ctimes%20%28H%2B1%29) , 32-bit integer or floating-point (32f or 64f) and
	///  integral image for squared pixel values; it is ![inline formula](https://latex.codecogs.com/png.latex?%28W%2B1%29%5Ctimes%20%28H%2B%29), double-precision floating-point (64f) array.
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.matrixop.integral"
	/// 
	/// ## Parameters
	/// * src: input image.
	/// * sdepth: desired depth of the integral and the tilted integral images, CV_32S, CV_32F, or
	/// CV_64F.
	/// * sqdepth: desired depth of the integral image of squared pixel values, CV_32F or CV_64F.
	/// 
	/// ## C++ default parameters
	/// * sdepth: -1
	/// * sqdepth: -1
	#[inline]
	pub fn integral(src: &crate::gapi::GMat, sdepth: i32, sqdepth: i32) -> Result<core::Tuple<(crate::gapi::GMat, crate::gapi::GMat)>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_integral_const_GMatR_int_int(src.as_raw_GMat(), sdepth, sqdepth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Tuple::<(crate::gapi::GMat, crate::gapi::GMat)>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Applies a mask to a matrix.
	/// 
	/// The function mask set value from given matrix if the corresponding pixel value in mask matrix set to true,
	/// and set the matrix value to 0 otherwise.
	/// 
	/// Supported src matrix data types are [CV_8UC1], [CV_16SC1], [CV_16UC1]. Supported mask data type is [CV_8UC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.mask"
	/// ## Parameters
	/// * src: input matrix.
	/// * mask: input mask matrix.
	#[inline]
	pub fn mask(src: &crate::gapi::GMat, mask: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_mask_const_GMatR_const_GMatR(src.as_raw_GMat(), mask.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates per-element maximum of two matrices.
	/// 
	/// The function max calculates the per-element maximum of two matrices of the same size, number of channels and depth:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Cmax%20%28%20%5Ctexttt%7Bsrc1%7D%20%28I%29%2C%20%5Ctexttt%7Bsrc2%7D%20%28I%29%29)
	///    where I is a multi-dimensional index of matrix elements. In case of
	///    multi-channel matrices, each channel is processed independently.
	/// Output matrix must be of the same size and depth as src1.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.matrixop.max"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix of the same size and depth as src1.
	/// ## See also
	/// min, compare, cmpEQ, cmpGT, cmpGE
	#[inline]
	pub fn max(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_max_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates an average (mean) of matrix elements.
	/// 
	/// The function mean calculates the mean value M of matrix elements,
	/// independently for each channel, and return it.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.mean"
	/// ## Parameters
	/// * src: input matrix.
	/// ## See also
	/// countNonZero, min, max
	#[inline]
	pub fn mean(src: &crate::gapi::GMat) -> Result<crate::gapi::GScalar> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_mean_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GScalar::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Blurs an image using the median filter.
	/// 
	/// The function smoothes an image using the median filter with the ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bksize%7D%20%5Ctimes%0A%5Ctexttt%7Bksize%7D) aperture. Each channel of a multi-channel image is processed independently.
	/// Output image must have the same type, size, and number of channels as the input image.
	/// 
	/// Note:
	///  - Rounding to nearest even is procedeed if hardware supports it, if not - to nearest.
	/// The median filter uses cv::BORDER_REPLICATE internally to cope with border pixels, see cv::BorderTypes
	///  - Function textual ID is "org.opencv.imgproc.filters.medianBlur"
	/// 
	/// ## Parameters
	/// * src: input matrix (image)
	/// * ksize: aperture linear size; it must be odd and greater than 1, for example: 3, 5, 7 ...
	/// ## See also
	/// boxFilter, gaussianBlur
	#[inline]
	pub fn median_blur(src: &crate::gapi::GMat, ksize: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_medianBlur_const_GMatR_int(src.as_raw_GMat(), ksize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates one 3-channel matrix out of 3 single-channel ones.
	/// 
	/// The function merges several matrices to make a single multi-channel matrix. That is, each
	/// element of the output matrix will be a concatenation of the elements of the input matrices, where
	/// elements of i-th input matrix are treated as mv[i].channels()-element vectors.
	/// Output matrix must be of [CV_8UC3] type.
	/// 
	/// The function split3 does the reverse operation.
	/// 
	/// 
	/// Note:
	///  - Function textual ID is "org.opencv.core.transform.merge3"
	/// 
	/// ## Parameters
	/// * src1: first input [CV_8UC1] matrix to be merged.
	/// * src2: second input [CV_8UC1] matrix to be merged.
	/// * src3: third input [CV_8UC1] matrix to be merged.
	/// ## See also
	/// merge4, split4, split3
	#[inline]
	pub fn merge3(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat, src3: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_merge3_const_GMatR_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), src3.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates one 4-channel matrix out of 4 single-channel ones.
	/// 
	/// The function merges several matrices to make a single multi-channel matrix. That is, each
	/// element of the output matrix will be a concatenation of the elements of the input matrices, where
	/// elements of i-th input matrix are treated as mv[i].channels()-element vectors.
	/// Output matrix must be of [CV_8UC4] type.
	/// 
	/// The function split4 does the reverse operation.
	/// 
	/// 
	/// Note:
	///  - Function textual ID is "org.opencv.core.transform.merge4"
	/// 
	/// ## Parameters
	/// * src1: first input [CV_8UC1] matrix to be merged.
	/// * src2: second input [CV_8UC1] matrix to be merged.
	/// * src3: third input [CV_8UC1] matrix to be merged.
	/// * src4: fourth input [CV_8UC1] matrix to be merged.
	/// ## See also
	/// merge3, split4, split3
	#[inline]
	pub fn merge4(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat, src3: &crate::gapi::GMat, src4: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_merge4_const_GMatR_const_GMatR_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), src3.as_raw_GMat(), src4.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates per-element minimum of two matrices.
	/// 
	/// The function min calculates the per-element minimum of two matrices of the same size, number of channels and depth:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Cmin%20%28%20%5Ctexttt%7Bsrc1%7D%20%28I%29%2C%20%5Ctexttt%7Bsrc2%7D%20%28I%29%29)
	///    where I is a multi-dimensional index of matrix elements. In case of
	///    multi-channel matrices, each channel is processed independently.
	/// Output matrix must be of the same size and depth as src1.
	/// 
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.matrixop.min"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix of the same size and depth as src1.
	/// ## See also
	/// max, cmpEQ, cmpLT, cmpLE
	#[inline]
	pub fn min(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_min_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Performs advanced morphological transformations.
	/// 
	/// The function can perform advanced morphological transformations using an erosion and dilation as
	/// basic operations.
	/// 
	/// Any of the operations can be done in-place. In case of multi-channel images, each channel is
	/// processed independently.
	/// 
	/// 
	/// Note:
	///  - Function textual ID is "org.opencv.imgproc.filters.morphologyEx"
	///  - The number of iterations is the number of times erosion or dilatation operation will be
	/// applied. For instance, an opening operation (#MORPH_OPEN) with two iterations is equivalent to
	/// apply successively: erode -> erode -> dilate -> dilate
	/// (and not erode -> dilate -> erode -> dilate).
	/// 
	/// ## Parameters
	/// * src: Input image.
	/// * op: Type of a morphological operation, see [morph_types]
	/// * kernel: Structuring element. It can be created using #getStructuringElement.
	/// * anchor: Anchor position within the element. Both negative values mean that the anchor is at
	/// the kernel center.
	/// * iterations: Number of times erosion and dilation are applied.
	/// * borderType: Pixel extrapolation method, see #BorderTypes. [BORDER_WRAP] is not supported.
	/// * borderValue: Border value in case of a constant border. The default value has a special
	/// meaning.
	/// ## See also
	/// dilate, erode, getStructuringElement
	/// 
	/// ## Note
	/// This alternative version of [morphology_ex] function uses the following default values for its arguments:
	/// * anchor: Point(-1,-1)
	/// * iterations: 1
	/// * border_type: BORDER_CONSTANT
	/// * border_value: morphologyDefaultBorderValue()
	#[inline]
	pub fn morphology_ex_def(src: &crate::gapi::GMat, op: crate::imgproc::MorphTypes, kernel: &core::Mat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_morphologyEx_const_GMatR_const_MorphTypes_const_MatR(src.as_raw_GMat(), op, kernel.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Performs advanced morphological transformations.
	/// 
	/// The function can perform advanced morphological transformations using an erosion and dilation as
	/// basic operations.
	/// 
	/// Any of the operations can be done in-place. In case of multi-channel images, each channel is
	/// processed independently.
	/// 
	/// 
	/// Note:
	///  - Function textual ID is "org.opencv.imgproc.filters.morphologyEx"
	///  - The number of iterations is the number of times erosion or dilatation operation will be
	/// applied. For instance, an opening operation (#MORPH_OPEN) with two iterations is equivalent to
	/// apply successively: erode -> erode -> dilate -> dilate
	/// (and not erode -> dilate -> erode -> dilate).
	/// 
	/// ## Parameters
	/// * src: Input image.
	/// * op: Type of a morphological operation, see [morph_types]
	/// * kernel: Structuring element. It can be created using #getStructuringElement.
	/// * anchor: Anchor position within the element. Both negative values mean that the anchor is at
	/// the kernel center.
	/// * iterations: Number of times erosion and dilation are applied.
	/// * borderType: Pixel extrapolation method, see #BorderTypes. [BORDER_WRAP] is not supported.
	/// * borderValue: Border value in case of a constant border. The default value has a special
	/// meaning.
	/// ## See also
	/// dilate, erode, getStructuringElement
	/// 
	/// ## C++ default parameters
	/// * anchor: Point(-1,-1)
	/// * iterations: 1
	/// * border_type: BORDER_CONSTANT
	/// * border_value: morphologyDefaultBorderValue()
	#[inline]
	pub fn morphology_ex(src: &crate::gapi::GMat, op: crate::imgproc::MorphTypes, kernel: &core::Mat, anchor: core::Point, iterations: i32, border_type: core::BorderTypes, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_morphologyEx_const_GMatR_const_MorphTypes_const_MatR_const_PointR_const_int_const_BorderTypes_const_ScalarR(src.as_raw_GMat(), op, kernel.as_raw_Mat(), &anchor, iterations, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// @overload
	/// 
	/// ## Note
	/// This alternative version of [mul_c_1] function uses the following default values for its arguments:
	/// * ddepth: -1
	#[inline]
	pub fn mul_c_1_def(src: &crate::gapi::GMat, multiplier: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_mulC_const_GMatR_const_GScalarR(src.as_raw_GMat(), multiplier.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Multiplies matrix by scalar.
	/// 
	/// The function mulC multiplies each element of matrix src by given scalar value:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Bsaturate%7D%20%28%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%5Ccdot%20%5Ctexttt%7Bmultiplier%7D%20%29)
	/// 
	/// The matrices can be single or multi channel. Output matrix must have the same size as src.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.mulC"
	/// ## Parameters
	/// * src: input matrix.
	/// * multiplier: factor to be multiplied.
	/// * ddepth: optional depth of the output matrix. If -1, the depth of output matrix will be the same as input matrix depth.
	/// ## See also
	/// add, sub, div, addWeighted
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * ddepth: -1
	#[inline]
	pub fn mul_c_1(src: &crate::gapi::GMat, multiplier: &crate::gapi::GScalar, ddepth: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_mulC_const_GMatR_const_GScalarR_int(src.as_raw_GMat(), multiplier.as_raw_GScalar(), ddepth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Multiplies matrix by scalar.
	/// 
	/// The function mulC multiplies each element of matrix src by given scalar value:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Bsaturate%7D%20%28%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%5Ccdot%20%5Ctexttt%7Bmultiplier%7D%20%29)
	/// 
	/// The matrices can be single or multi channel. Output matrix must have the same size as src.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.mulC"
	/// ## Parameters
	/// * src: input matrix.
	/// * multiplier: factor to be multiplied.
	/// * ddepth: optional depth of the output matrix. If -1, the depth of output matrix will be the same as input matrix depth.
	/// ## See also
	/// add, sub, div, addWeighted
	/// 
	/// ## Note
	/// This alternative version of [mul_c] function uses the following default values for its arguments:
	/// * ddepth: -1
	#[inline]
	pub fn mul_c_def(src: &crate::gapi::GMat, multiplier: f64) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_mulC_const_GMatR_double(src.as_raw_GMat(), multiplier, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Multiplies matrix by scalar.
	/// 
	/// The function mulC multiplies each element of matrix src by given scalar value:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Bsaturate%7D%20%28%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%5Ccdot%20%5Ctexttt%7Bmultiplier%7D%20%29)
	/// 
	/// The matrices can be single or multi channel. Output matrix must have the same size as src.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.mulC"
	/// ## Parameters
	/// * src: input matrix.
	/// * multiplier: factor to be multiplied.
	/// * ddepth: optional depth of the output matrix. If -1, the depth of output matrix will be the same as input matrix depth.
	/// ## See also
	/// add, sub, div, addWeighted
	/// 
	/// ## C++ default parameters
	/// * ddepth: -1
	#[inline]
	pub fn mul_c(src: &crate::gapi::GMat, multiplier: f64, ddepth: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_mulC_const_GMatR_double_int(src.as_raw_GMat(), multiplier, ddepth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// @overload
	/// 
	/// ## Note
	/// This alternative version of [mul_c_2] function uses the following default values for its arguments:
	/// * ddepth: -1
	#[inline]
	pub fn mul_c_2_def(multiplier: &crate::gapi::GScalar, src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_mulC_const_GScalarR_const_GMatR(multiplier.as_raw_GScalar(), src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Multiplies matrix by scalar.
	/// 
	/// The function mulC multiplies each element of matrix src by given scalar value:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Bsaturate%7D%20%28%20%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%5Ccdot%20%5Ctexttt%7Bmultiplier%7D%20%29)
	/// 
	/// The matrices can be single or multi channel. Output matrix must have the same size as src.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.mulC"
	/// ## Parameters
	/// * src: input matrix.
	/// * multiplier: factor to be multiplied.
	/// * ddepth: optional depth of the output matrix. If -1, the depth of output matrix will be the same as input matrix depth.
	/// ## See also
	/// add, sub, div, addWeighted
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * ddepth: -1
	#[inline]
	pub fn mul_c_2(multiplier: &crate::gapi::GScalar, src: &crate::gapi::GMat, ddepth: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_mulC_const_GScalarR_const_GMatR_int(multiplier.as_raw_GScalar(), src.as_raw_GMat(), ddepth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the per-element scaled product of two matrices.
	/// 
	/// The function mul calculates the per-element product of two matrices:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Bsaturate%7D%20%28%20%5Ctexttt%7Bscale%7D%20%5Ccdot%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%5Ccdot%20%5Ctexttt%7Bsrc2%7D%20%28I%29%29)
	/// 
	/// If src1.depth() == src2.depth(), ddepth can be set to the default -1. In this case, the output matrix will have
	/// the same depth as the input matrices. The matrices can be single or multi channel.
	/// Output matrix must have the same size as input matrices.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.mul"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix of the same size and the same depth as src1.
	/// * scale: optional scale factor.
	/// * ddepth: optional depth of the output matrix.
	/// ## See also
	/// add, sub, div, addWeighted
	/// 
	/// ## Note
	/// This alternative version of [mul] function uses the following default values for its arguments:
	/// * scale: 1.0
	/// * ddepth: -1
	#[inline]
	pub fn mul_def(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_mul_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the per-element scaled product of two matrices.
	/// 
	/// The function mul calculates the per-element product of two matrices:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Bsaturate%7D%20%28%20%5Ctexttt%7Bscale%7D%20%5Ccdot%20%5Ctexttt%7Bsrc1%7D%20%28I%29%20%20%5Ccdot%20%5Ctexttt%7Bsrc2%7D%20%28I%29%29)
	/// 
	/// If src1.depth() == src2.depth(), ddepth can be set to the default -1. In this case, the output matrix will have
	/// the same depth as the input matrices. The matrices can be single or multi channel.
	/// Output matrix must have the same size as input matrices.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.mul"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix of the same size and the same depth as src1.
	/// * scale: optional scale factor.
	/// * ddepth: optional depth of the output matrix.
	/// ## See also
	/// add, sub, div, addWeighted
	/// 
	/// ## C++ default parameters
	/// * scale: 1.0
	/// * ddepth: -1
	#[inline]
	pub fn mul(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat, scale: f64, ddepth: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_mul_const_GMatR_const_GMatR_double_int(src1.as_raw_GMat(), src2.as_raw_GMat(), scale, ddepth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the absolute infinite norm of a matrix.
	/// 
	/// This version of normInf calculates the absolute infinite norm of src.
	/// 
	/// As example for one array consider the function ![inline formula](https://latex.codecogs.com/png.latex?r%28x%29%3D%20%5Cbegin%7Bpmatrix%7D%20x%20%5C%5C%201%2Dx%20%5Cend%7Bpmatrix%7D%2C%20x%20%5Cin%20%5B%2D1%3B1%5D).
	/// The ![inline formula](https://latex.codecogs.com/png.latex?%20L%5F%7B%5Cinfty%7D%20) norm for the sample value ![inline formula](https://latex.codecogs.com/png.latex?r%28%2D1%29%20%3D%20%5Cbegin%7Bpmatrix%7D%20%2D1%20%5C%5C%202%20%5Cend%7Bpmatrix%7D)
	/// is calculated as follows
	/// \f{align*}
	///    \| r(-1) \|_{L_\infty} &= \max(|-1|,|2|) = 2
	/// \f}
	/// and for ![inline formula](https://latex.codecogs.com/png.latex?r%280%2E5%29%20%3D%20%5Cbegin%7Bpmatrix%7D%200%2E5%20%5C%5C%200%2E5%20%5Cend%7Bpmatrix%7D) the calculation is
	/// \f{align*}
	///    \| r(0.5) \|_{L_\infty} &= \max(|0.5|,|0.5|) = 0.5.
	/// \f}
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.matrixop.norminf"
	/// ## Parameters
	/// * src: input matrix.
	/// ## See also
	/// normL1, normL2
	#[inline]
	pub fn norm_inf(src: &crate::gapi::GMat) -> Result<crate::gapi::GScalar> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_normInf_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GScalar::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the  absolute L1 norm of a matrix.
	/// 
	/// This version of normL1 calculates the absolute L1 norm of src.
	/// 
	/// As example for one array consider the function ![inline formula](https://latex.codecogs.com/png.latex?r%28x%29%3D%20%5Cbegin%7Bpmatrix%7D%20x%20%5C%5C%201%2Dx%20%5Cend%7Bpmatrix%7D%2C%20x%20%5Cin%20%5B%2D1%3B1%5D).
	/// The ![inline formula](https://latex.codecogs.com/png.latex?%20L%5F%7B1%7D%20) norm for the sample value ![inline formula](https://latex.codecogs.com/png.latex?r%28%2D1%29%20%3D%20%5Cbegin%7Bpmatrix%7D%20%2D1%20%5C%5C%202%20%5Cend%7Bpmatrix%7D)
	/// is calculated as follows
	/// \f{align*}
	///    \| r(-1) \|_{L_1} &= |-1| + |2| = 3 \\
	/// \f}
	/// and for ![inline formula](https://latex.codecogs.com/png.latex?r%280%2E5%29%20%3D%20%5Cbegin%7Bpmatrix%7D%200%2E5%20%5C%5C%200%2E5%20%5Cend%7Bpmatrix%7D) the calculation is
	/// \f{align*}
	///    \| r(0.5) \|_{L_1} &= |0.5| + |0.5| = 1 \\
	/// \f}
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.matrixop.norml1"
	/// ## Parameters
	/// * src: input matrix.
	/// ## See also
	/// normL2, normInf
	#[inline]
	pub fn norm_l1(src: &crate::gapi::GMat) -> Result<crate::gapi::GScalar> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_normL1_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GScalar::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the absolute L2 norm of a matrix.
	/// 
	/// This version of normL2 calculates the absolute L2 norm of src.
	/// 
	/// As example for one array consider the function ![inline formula](https://latex.codecogs.com/png.latex?r%28x%29%3D%20%5Cbegin%7Bpmatrix%7D%20x%20%5C%5C%201%2Dx%20%5Cend%7Bpmatrix%7D%2C%20x%20%5Cin%20%5B%2D1%3B1%5D).
	/// The ![inline formula](https://latex.codecogs.com/png.latex?%20L%5F%7B2%7D%20)  norm for the sample value ![inline formula](https://latex.codecogs.com/png.latex?r%28%2D1%29%20%3D%20%5Cbegin%7Bpmatrix%7D%20%2D1%20%5C%5C%202%20%5Cend%7Bpmatrix%7D)
	/// is calculated as follows
	/// \f{align*}
	///    \| r(-1) \|_{L_2} &= \sqrt{(-1)^{2} + (2)^{2}} = \sqrt{5} \\
	/// \f}
	/// and for ![inline formula](https://latex.codecogs.com/png.latex?r%280%2E5%29%20%3D%20%5Cbegin%7Bpmatrix%7D%200%2E5%20%5C%5C%200%2E5%20%5Cend%7Bpmatrix%7D) the calculation is
	/// \f{align*}
	///    \| r(0.5) \|_{L_2} &= \sqrt{(0.5)^{2} + (0.5)^{2}} = \sqrt{0.5} \\
	/// \f}
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// Note: Function textual ID is "org.opencv.core.matrixop.norml2"
	/// ## Parameters
	/// * src: input matrix.
	/// ## See also
	/// normL1, normInf
	#[inline]
	pub fn norm_l2(src: &crate::gapi::GMat) -> Result<crate::gapi::GScalar> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_normL2_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GScalar::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Normalizes the norm or value range of an array.
	/// 
	/// The function normalizes scale and shift the input array elements so that
	/// ![block formula](https://latex.codecogs.com/png.latex?%5C%7C%20%5Ctexttt%7Bdst%7D%20%5C%7C%20%5F%7BL%5Fp%7D%3D%20%5Ctexttt%7Balpha%7D)
	/// (where p=Inf, 1 or 2) when normType=NORM_INF, NORM_L1, or NORM_L2, respectively; or so that
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cmin%20%5FI%20%20%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Balpha%7D%20%2C%20%5C%2C%20%5C%2C%20%5Cmax%20%5FI%20%20%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Bbeta%7D)
	/// when normType=NORM_MINMAX (for dense arrays only).
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.normalize"
	/// 
	/// ## Parameters
	/// * src: input array.
	/// * alpha: norm value to normalize to or the lower range boundary in case of the range
	/// normalization.
	/// * beta: upper range boundary in case of the range normalization; it is not used for the norm
	/// normalization.
	/// * norm_type: normalization type (see cv::NormTypes).
	/// * ddepth: when negative, the output array has the same type as src; otherwise, it has the same
	/// number of channels as src and the depth =ddepth.
	/// ## See also
	/// norm, Mat::convertTo
	/// 
	/// ## Note
	/// This alternative version of [normalize] function uses the following default values for its arguments:
	/// * ddepth: -1
	#[inline]
	pub fn normalize_def(src: &crate::gapi::GMat, alpha: f64, beta: f64, norm_type: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_normalize_const_GMatR_double_double_int(src.as_raw_GMat(), alpha, beta, norm_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Normalizes the norm or value range of an array.
	/// 
	/// The function normalizes scale and shift the input array elements so that
	/// ![block formula](https://latex.codecogs.com/png.latex?%5C%7C%20%5Ctexttt%7Bdst%7D%20%5C%7C%20%5F%7BL%5Fp%7D%3D%20%5Ctexttt%7Balpha%7D)
	/// (where p=Inf, 1 or 2) when normType=NORM_INF, NORM_L1, or NORM_L2, respectively; or so that
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cmin%20%5FI%20%20%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Balpha%7D%20%2C%20%5C%2C%20%5C%2C%20%5Cmax%20%5FI%20%20%5Ctexttt%7Bdst%7D%20%28I%29%3D%20%5Ctexttt%7Bbeta%7D)
	/// when normType=NORM_MINMAX (for dense arrays only).
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.normalize"
	/// 
	/// ## Parameters
	/// * src: input array.
	/// * alpha: norm value to normalize to or the lower range boundary in case of the range
	/// normalization.
	/// * beta: upper range boundary in case of the range normalization; it is not used for the norm
	/// normalization.
	/// * norm_type: normalization type (see cv::NormTypes).
	/// * ddepth: when negative, the output array has the same type as src; otherwise, it has the same
	/// number of channels as src and the depth =ddepth.
	/// ## See also
	/// norm, Mat::convertTo
	/// 
	/// ## C++ default parameters
	/// * ddepth: -1
	#[inline]
	pub fn normalize(src: &crate::gapi::GMat, alpha: f64, beta: f64, norm_type: i32, ddepth: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_normalize_const_GMatR_double_double_int_int(src.as_raw_GMat(), alpha, beta, norm_type, ddepth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the rotation angle of 2D vectors.
	/// 
	/// The function cv::phase calculates the rotation angle of each 2D vector that
	/// is formed from the corresponding elements of x and y :
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bangle%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Batan2%7D%20%28%20%5Ctexttt%7By%7D%20%28I%29%2C%20%5Ctexttt%7Bx%7D%20%28I%29%29)
	/// 
	/// The angle estimation accuracy is about 0.3 degrees. When x(I)=y(I)=0 ,
	/// the corresponding angle(I) is set to 0.
	/// ## Parameters
	/// * x: input floating-point array of x-coordinates of 2D vectors.
	/// * y: input array of y-coordinates of 2D vectors; it must have the
	/// same size and the same type as x.
	/// * angleInDegrees: when true, the function calculates the angle in
	/// degrees, otherwise, they are measured in radians.
	/// ## Returns
	/// array of vector angles; it has the same size and same type as x.
	/// 
	/// ## Note
	/// This alternative version of [phase] function uses the following default values for its arguments:
	/// * angle_in_degrees: false
	#[inline]
	pub fn phase_def(x: &crate::gapi::GMat, y: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_phase_const_GMatR_const_GMatR(x.as_raw_GMat(), y.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the rotation angle of 2D vectors.
	/// 
	/// The function cv::phase calculates the rotation angle of each 2D vector that
	/// is formed from the corresponding elements of x and y :
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bangle%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Batan2%7D%20%28%20%5Ctexttt%7By%7D%20%28I%29%2C%20%5Ctexttt%7Bx%7D%20%28I%29%29)
	/// 
	/// The angle estimation accuracy is about 0.3 degrees. When x(I)=y(I)=0 ,
	/// the corresponding angle(I) is set to 0.
	/// ## Parameters
	/// * x: input floating-point array of x-coordinates of 2D vectors.
	/// * y: input array of y-coordinates of 2D vectors; it must have the
	/// same size and the same type as x.
	/// * angleInDegrees: when true, the function calculates the angle in
	/// degrees, otherwise, they are measured in radians.
	/// ## Returns
	/// array of vector angles; it has the same size and same type as x.
	/// 
	/// ## C++ default parameters
	/// * angle_in_degrees: false
	#[inline]
	pub fn phase(x: &crate::gapi::GMat, y: &crate::gapi::GMat, angle_in_degrees: bool) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_phase_const_GMatR_const_GMatR_bool(x.as_raw_GMat(), y.as_raw_GMat(), angle_in_degrees, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates x and y coordinates of 2D vectors from their magnitude and angle.
	/// 
	/// The function polarToCart calculates the Cartesian coordinates of each 2D
	/// vector represented by the corresponding elements of magnitude and angle:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20%5Ctexttt%7Bx%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bmagnitude%7D%20%28I%29%20%5Ccos%20%28%20%5Ctexttt%7Bangle%7D%20%28I%29%29%20%5C%5C%20%5Ctexttt%7By%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bmagnitude%7D%20%28I%29%20%5Csin%20%28%20%5Ctexttt%7Bangle%7D%20%28I%29%29%20%5C%5C%20%5Cend%7Barray%7D)
	/// 
	/// The relative accuracy of the estimated coordinates is about 1e-6.
	/// 
	/// First output is a matrix of x-coordinates of 2D vectors.
	/// Second output is a matrix of y-coordinates of 2D vectors.
	/// Both output must have the same size and depth as input matrices.
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.polarToCart"
	/// 
	/// ## Parameters
	/// * magnitude: input floating-point [CV_32FC1] matrix (1xN) of magnitudes of 2D vectors;
	/// * angle: input floating-point [CV_32FC1] matrix (1xN) of angles of 2D vectors.
	/// * angleInDegrees: when true, the input angles are measured in
	/// degrees, otherwise, they are measured in radians.
	/// ## See also
	/// cartToPolar, exp, log, pow, sqrt
	/// 
	/// ## Note
	/// This alternative version of [polar_to_cart] function uses the following default values for its arguments:
	/// * angle_in_degrees: false
	#[inline]
	pub fn polar_to_cart_def(magnitude: &crate::gapi::GMat, angle: &crate::gapi::GMat) -> Result<core::Tuple<(crate::gapi::GMat, crate::gapi::GMat)>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_polarToCart_const_GMatR_const_GMatR(magnitude.as_raw_GMat(), angle.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Tuple::<(crate::gapi::GMat, crate::gapi::GMat)>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates x and y coordinates of 2D vectors from their magnitude and angle.
	/// 
	/// The function polarToCart calculates the Cartesian coordinates of each 2D
	/// vector represented by the corresponding elements of magnitude and angle:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20%5Ctexttt%7Bx%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bmagnitude%7D%20%28I%29%20%5Ccos%20%28%20%5Ctexttt%7Bangle%7D%20%28I%29%29%20%5C%5C%20%5Ctexttt%7By%7D%20%28I%29%20%3D%20%20%5Ctexttt%7Bmagnitude%7D%20%28I%29%20%5Csin%20%28%20%5Ctexttt%7Bangle%7D%20%28I%29%29%20%5C%5C%20%5Cend%7Barray%7D)
	/// 
	/// The relative accuracy of the estimated coordinates is about 1e-6.
	/// 
	/// First output is a matrix of x-coordinates of 2D vectors.
	/// Second output is a matrix of y-coordinates of 2D vectors.
	/// Both output must have the same size and depth as input matrices.
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.polarToCart"
	/// 
	/// ## Parameters
	/// * magnitude: input floating-point [CV_32FC1] matrix (1xN) of magnitudes of 2D vectors;
	/// * angle: input floating-point [CV_32FC1] matrix (1xN) of angles of 2D vectors.
	/// * angleInDegrees: when true, the input angles are measured in
	/// degrees, otherwise, they are measured in radians.
	/// ## See also
	/// cartToPolar, exp, log, pow, sqrt
	/// 
	/// ## C++ default parameters
	/// * angle_in_degrees: false
	#[inline]
	pub fn polar_to_cart(magnitude: &crate::gapi::GMat, angle: &crate::gapi::GMat, angle_in_degrees: bool) -> Result<core::Tuple<(crate::gapi::GMat, crate::gapi::GMat)>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_polarToCart_const_GMatR_const_GMatR_bool(magnitude.as_raw_GMat(), angle.as_raw_GMat(), angle_in_degrees, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Tuple::<(crate::gapi::GMat, crate::gapi::GMat)>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Applies a generic geometrical transformation to an image.
	/// 
	/// The function remap transforms the source image using the specified map:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Ctexttt%7Bsrc%7D%20%28map%5Fx%28x%2Cy%29%2Cmap%5Fy%28x%2Cy%29%29)
	/// 
	/// where values of pixels with non-integer coordinates are computed using one of available
	/// interpolation methods. ![inline formula](https://latex.codecogs.com/png.latex?map%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?map%5Fy) can be encoded as separate floating-point maps
	/// in ![inline formula](https://latex.codecogs.com/png.latex?map%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?map%5F2) respectively, or interleaved floating-point maps of ![inline formula](https://latex.codecogs.com/png.latex?%28x%2Cy%29) in
	/// ![inline formula](https://latex.codecogs.com/png.latex?map%5F1), or fixed-point maps created by using convertMaps. The reason you might want to
	/// convert from floating to fixed-point representations of a map is that they can yield much faster
	/// (\~2x) remapping operations. In the converted case, ![inline formula](https://latex.codecogs.com/png.latex?map%5F1) contains pairs (cvFloor(x),
	/// cvFloor(y)) and ![inline formula](https://latex.codecogs.com/png.latex?map%5F2) contains indices in a table of interpolation coefficients.
	/// Output image must be of the same size and depth as input one.
	/// 
	/// 
	/// Note:
	///  - Function textual ID is "org.opencv.core.transform.remap"
	///  - Due to current implementation limitations the size of an input and output images should be less than 32767x32767.
	/// 
	/// ## Parameters
	/// * src: Source image.
	/// * map1: The first map of either (x,y) points or just x values having the type CV_16SC2,
	/// CV_32FC1, or CV_32FC2.
	/// * map2: The second map of y values having the type CV_16UC1, CV_32FC1, or none (empty map
	/// if map1 is (x,y) points), respectively.
	/// * interpolation: Interpolation method (see cv::InterpolationFlags). The methods [INTER_AREA]
	/// and [INTER_LINEAR_EXACT] are not supported by this function.
	/// * borderMode: Pixel extrapolation method (see cv::BorderTypes). When
	/// borderMode=BORDER_TRANSPARENT, it means that the pixels in the destination image that
	/// corresponds to the "outliers" in the source image are not modified by the function.
	/// * borderValue: Value used in case of a constant border. By default, it is 0.
	/// 
	/// ## Note
	/// This alternative version of [remap] function uses the following default values for its arguments:
	/// * border_mode: BORDER_CONSTANT
	/// * border_value: Scalar()
	#[inline]
	pub fn remap_def(src: &crate::gapi::GMat, map1: &core::Mat, map2: &core::Mat, interpolation: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_remap_const_GMatR_const_MatR_const_MatR_int(src.as_raw_GMat(), map1.as_raw_Mat(), map2.as_raw_Mat(), interpolation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Applies a generic geometrical transformation to an image.
	/// 
	/// The function remap transforms the source image using the specified map:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Ctexttt%7Bsrc%7D%20%28map%5Fx%28x%2Cy%29%2Cmap%5Fy%28x%2Cy%29%29)
	/// 
	/// where values of pixels with non-integer coordinates are computed using one of available
	/// interpolation methods. ![inline formula](https://latex.codecogs.com/png.latex?map%5Fx) and ![inline formula](https://latex.codecogs.com/png.latex?map%5Fy) can be encoded as separate floating-point maps
	/// in ![inline formula](https://latex.codecogs.com/png.latex?map%5F1) and ![inline formula](https://latex.codecogs.com/png.latex?map%5F2) respectively, or interleaved floating-point maps of ![inline formula](https://latex.codecogs.com/png.latex?%28x%2Cy%29) in
	/// ![inline formula](https://latex.codecogs.com/png.latex?map%5F1), or fixed-point maps created by using convertMaps. The reason you might want to
	/// convert from floating to fixed-point representations of a map is that they can yield much faster
	/// (\~2x) remapping operations. In the converted case, ![inline formula](https://latex.codecogs.com/png.latex?map%5F1) contains pairs (cvFloor(x),
	/// cvFloor(y)) and ![inline formula](https://latex.codecogs.com/png.latex?map%5F2) contains indices in a table of interpolation coefficients.
	/// Output image must be of the same size and depth as input one.
	/// 
	/// 
	/// Note:
	///  - Function textual ID is "org.opencv.core.transform.remap"
	///  - Due to current implementation limitations the size of an input and output images should be less than 32767x32767.
	/// 
	/// ## Parameters
	/// * src: Source image.
	/// * map1: The first map of either (x,y) points or just x values having the type CV_16SC2,
	/// CV_32FC1, or CV_32FC2.
	/// * map2: The second map of y values having the type CV_16UC1, CV_32FC1, or none (empty map
	/// if map1 is (x,y) points), respectively.
	/// * interpolation: Interpolation method (see cv::InterpolationFlags). The methods [INTER_AREA]
	/// and [INTER_LINEAR_EXACT] are not supported by this function.
	/// * borderMode: Pixel extrapolation method (see cv::BorderTypes). When
	/// borderMode=BORDER_TRANSPARENT, it means that the pixels in the destination image that
	/// corresponds to the "outliers" in the source image are not modified by the function.
	/// * borderValue: Value used in case of a constant border. By default, it is 0.
	/// 
	/// ## C++ default parameters
	/// * border_mode: BORDER_CONSTANT
	/// * border_value: Scalar()
	#[inline]
	pub fn remap(src: &crate::gapi::GMat, map1: &core::Mat, map2: &core::Mat, interpolation: i32, border_mode: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_remap_const_GMatR_const_MatR_const_MatR_int_int_const_ScalarR(src.as_raw_GMat(), map1.as_raw_Mat(), map2.as_raw_Mat(), interpolation, border_mode, &border_value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Resizes a planar image.
	/// 
	/// The function resizes the image src down to or up to the specified size.
	/// Planar image memory layout is three planes laying in the memory contiguously,
	/// so the image height should be plane_height*plane_number, image type is [CV_8UC1].
	/// 
	/// Output image size will have the size dsize, the depth of output is the same as of src.
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.transform.resizeP"
	/// 
	/// ## Parameters
	/// * src: input image, must be of [CV_8UC1] type;
	/// * dsize: output image size;
	/// * interpolation: interpolation method, only cv::INTER_LINEAR is supported at the moment
	/// ## See also
	/// warpAffine, warpPerspective, remap, resize
	/// 
	/// ## Note
	/// This alternative version of [resize_p] function uses the following default values for its arguments:
	/// * interpolation: cv::INTER_LINEAR
	#[inline]
	pub fn resize_p_def(src: &crate::gapi::GMatP, dsize: core::Size) -> Result<crate::gapi::GMatP> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_resizeP_const_GMatPR_const_SizeR(src.as_raw_GMatP(), &dsize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMatP::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Resizes a planar image.
	/// 
	/// The function resizes the image src down to or up to the specified size.
	/// Planar image memory layout is three planes laying in the memory contiguously,
	/// so the image height should be plane_height*plane_number, image type is [CV_8UC1].
	/// 
	/// Output image size will have the size dsize, the depth of output is the same as of src.
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.transform.resizeP"
	/// 
	/// ## Parameters
	/// * src: input image, must be of [CV_8UC1] type;
	/// * dsize: output image size;
	/// * interpolation: interpolation method, only cv::INTER_LINEAR is supported at the moment
	/// ## See also
	/// warpAffine, warpPerspective, remap, resize
	/// 
	/// ## C++ default parameters
	/// * interpolation: cv::INTER_LINEAR
	#[inline]
	pub fn resize_p(src: &crate::gapi::GMatP, dsize: core::Size, interpolation: i32) -> Result<crate::gapi::GMatP> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_resizeP_const_GMatPR_const_SizeR_int(src.as_raw_GMatP(), &dsize, interpolation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMatP::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Resizes an image.
	/// 
	/// The function resizes the image src down to or up to the specified size.
	/// 
	/// Output image size will have the size dsize (when dsize is non-zero) or the size computed from
	/// src.size(), fx, and fy; the depth of output is the same as of src.
	/// 
	/// If you want to resize src so that it fits the pre-created dst,
	/// you may call the function as follows:
	/// ```C++
	///  explicitly specify dsize=dst.size(); fx and fy will be computed from that.
	///    resize(src, dst, dst.size(), 0, 0, interpolation);
	/// ```
	/// 
	/// If you want to decimate the image by factor of 2 in each direction, you can call the function this
	/// way:
	/// ```C++
	///  specify fx and fy and let the function compute the destination image size.
	///    resize(src, dst, Size(), 0.5, 0.5, interpolation);
	/// ```
	/// 
	/// To shrink an image, it will generally look best with cv::INTER_AREA interpolation, whereas to
	/// enlarge an image, it will generally look best with cv::INTER_CUBIC (slow) or cv::INTER_LINEAR
	/// (faster but still looks OK).
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.transform.resize"
	/// 
	/// ## Parameters
	/// * src: input image.
	/// * dsize: output image size; if it equals zero, it is computed as:
	///  ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdsize%20%3D%20Size%28round%28fx%2Asrc%2Ecols%29%2C%20round%28fy%2Asrc%2Erows%29%29%7D)
	///  Either dsize or both fx and fy must be non-zero.
	/// * fx: scale factor along the horizontal axis; when it equals 0, it is computed as
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7B%28double%29dsize%2Ewidth%2Fsrc%2Ecols%7D)
	/// * fy: scale factor along the vertical axis; when it equals 0, it is computed as
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7B%28double%29dsize%2Eheight%2Fsrc%2Erows%7D)
	/// * interpolation: interpolation method, see cv::InterpolationFlags
	/// ## See also
	/// warpAffine, warpPerspective, remap, resizeP
	/// 
	/// ## Note
	/// This alternative version of [resize] function uses the following default values for its arguments:
	/// * fx: 0
	/// * fy: 0
	/// * interpolation: INTER_LINEAR
	#[inline]
	pub fn resize_def(src: &crate::gapi::GMat, dsize: core::Size) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_resize_const_GMatR_const_SizeR(src.as_raw_GMat(), &dsize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Resizes an image.
	/// 
	/// The function resizes the image src down to or up to the specified size.
	/// 
	/// Output image size will have the size dsize (when dsize is non-zero) or the size computed from
	/// src.size(), fx, and fy; the depth of output is the same as of src.
	/// 
	/// If you want to resize src so that it fits the pre-created dst,
	/// you may call the function as follows:
	/// ```C++
	///  explicitly specify dsize=dst.size(); fx and fy will be computed from that.
	///    resize(src, dst, dst.size(), 0, 0, interpolation);
	/// ```
	/// 
	/// If you want to decimate the image by factor of 2 in each direction, you can call the function this
	/// way:
	/// ```C++
	///  specify fx and fy and let the function compute the destination image size.
	///    resize(src, dst, Size(), 0.5, 0.5, interpolation);
	/// ```
	/// 
	/// To shrink an image, it will generally look best with cv::INTER_AREA interpolation, whereas to
	/// enlarge an image, it will generally look best with cv::INTER_CUBIC (slow) or cv::INTER_LINEAR
	/// (faster but still looks OK).
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.imgproc.transform.resize"
	/// 
	/// ## Parameters
	/// * src: input image.
	/// * dsize: output image size; if it equals zero, it is computed as:
	///  ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdsize%20%3D%20Size%28round%28fx%2Asrc%2Ecols%29%2C%20round%28fy%2Asrc%2Erows%29%29%7D)
	///  Either dsize or both fx and fy must be non-zero.
	/// * fx: scale factor along the horizontal axis; when it equals 0, it is computed as
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7B%28double%29dsize%2Ewidth%2Fsrc%2Ecols%7D)
	/// * fy: scale factor along the vertical axis; when it equals 0, it is computed as
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7B%28double%29dsize%2Eheight%2Fsrc%2Erows%7D)
	/// * interpolation: interpolation method, see cv::InterpolationFlags
	/// ## See also
	/// warpAffine, warpPerspective, remap, resizeP
	/// 
	/// ## C++ default parameters
	/// * fx: 0
	/// * fy: 0
	/// * interpolation: INTER_LINEAR
	#[inline]
	pub fn resize(src: &crate::gapi::GMat, dsize: core::Size, fx: f64, fy: f64, interpolation: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_resize_const_GMatR_const_SizeR_double_double_int(src.as_raw_GMat(), &dsize, fx, fy, interpolation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Select values from either first or second of input matrices by given mask.
	/// The function set to the output matrix either the value from the first input matrix if corresponding value of mask matrix is 255,
	///  or value from the second input matrix (if value of mask matrix set to 0).
	/// 
	/// Input mask matrix must be of [CV_8UC1] type, two other inout matrices and output matrix should be of the same type. The size should
	/// be the same for all input and output matrices.
	/// Supported input matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.pixelwise.select"
	/// 
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix.
	/// * mask: mask input matrix.
	#[inline]
	pub fn select(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat, mask: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_select_const_GMatR_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), mask.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Applies a separable linear filter to a matrix(image).
	/// 
	/// The function applies a separable linear filter to the matrix. That is, first, every row of src is
	/// filtered with the 1D kernel kernelX. Then, every column of the result is filtered with the 1D
	/// kernel kernelY. The final result is returned.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// Output image must have the same type, size, and number of channels as the input image.
	/// 
	/// Note:
	///  - In case of floating-point computation, rounding to nearest even is procedeed
	/// if hardware supports it (if not - to nearest value).
	///  - Function textual ID is "org.opencv.imgproc.filters.sepfilter"
	/// ## Parameters
	/// * src: Source image.
	/// * ddepth: desired depth of the destination image (the following combinations of src.depth() and ddepth are supported:
	/// 
	///        src.depth() = CV_8U, ddepth = -1/CV_16S/CV_32F/CV_64F
	///        src.depth() = CV_16U/CV_16S, ddepth = -1/CV_32F/CV_64F
	///        src.depth() = CV_32F, ddepth = -1/CV_32F/CV_64F
	///        src.depth() = CV_64F, ddepth = -1/CV_64F
	/// 
	/// when ddepth=-1, the output image will have the same depth as the source)
	/// * kernelX: Coefficients for filtering each row.
	/// * kernelY: Coefficients for filtering each column.
	/// * anchor: Anchor position within the kernel. The default value ![inline formula](https://latex.codecogs.com/png.latex?%28%2D1%2C%2D1%29) means that the anchor
	/// is at the kernel center.
	/// * delta: Value added to the filtered results before storing them.
	/// * borderType: Pixel extrapolation method, see cv::BorderTypes
	/// * borderValue: border value in case of constant border type
	/// ## See also
	/// boxFilter, gaussianBlur, medianBlur
	/// 
	/// ## Note
	/// This alternative version of [sep_filter] function uses the following default values for its arguments:
	/// * border_type: BORDER_DEFAULT
	/// * border_value: Scalar(0)
	#[inline]
	pub fn sep_filter_def(src: &crate::gapi::GMat, ddepth: i32, kernel_x: &core::Mat, kernel_y: &core::Mat, anchor: core::Point, delta: core::Scalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_sepFilter_const_GMatR_int_const_MatR_const_MatR_const_PointR_const_ScalarR(src.as_raw_GMat(), ddepth, kernel_x.as_raw_Mat(), kernel_y.as_raw_Mat(), &anchor, &delta, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Applies a separable linear filter to a matrix(image).
	/// 
	/// The function applies a separable linear filter to the matrix. That is, first, every row of src is
	/// filtered with the 1D kernel kernelX. Then, every column of the result is filtered with the 1D
	/// kernel kernelY. The final result is returned.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// Output image must have the same type, size, and number of channels as the input image.
	/// 
	/// Note:
	///  - In case of floating-point computation, rounding to nearest even is procedeed
	/// if hardware supports it (if not - to nearest value).
	///  - Function textual ID is "org.opencv.imgproc.filters.sepfilter"
	/// ## Parameters
	/// * src: Source image.
	/// * ddepth: desired depth of the destination image (the following combinations of src.depth() and ddepth are supported:
	/// 
	///        src.depth() = CV_8U, ddepth = -1/CV_16S/CV_32F/CV_64F
	///        src.depth() = CV_16U/CV_16S, ddepth = -1/CV_32F/CV_64F
	///        src.depth() = CV_32F, ddepth = -1/CV_32F/CV_64F
	///        src.depth() = CV_64F, ddepth = -1/CV_64F
	/// 
	/// when ddepth=-1, the output image will have the same depth as the source)
	/// * kernelX: Coefficients for filtering each row.
	/// * kernelY: Coefficients for filtering each column.
	/// * anchor: Anchor position within the kernel. The default value ![inline formula](https://latex.codecogs.com/png.latex?%28%2D1%2C%2D1%29) means that the anchor
	/// is at the kernel center.
	/// * delta: Value added to the filtered results before storing them.
	/// * borderType: Pixel extrapolation method, see cv::BorderTypes
	/// * borderValue: border value in case of constant border type
	/// ## See also
	/// boxFilter, gaussianBlur, medianBlur
	/// 
	/// ## C++ default parameters
	/// * border_type: BORDER_DEFAULT
	/// * border_value: Scalar(0)
	#[inline]
	pub fn sep_filter(src: &crate::gapi::GMat, ddepth: i32, kernel_x: &core::Mat, kernel_y: &core::Mat, anchor: core::Point, delta: core::Scalar, border_type: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_sepFilter_const_GMatR_int_const_MatR_const_MatR_const_PointR_const_ScalarR_int_const_ScalarR(src.as_raw_GMat(), ddepth, kernel_x.as_raw_Mat(), kernel_y.as_raw_Mat(), &anchor, &delta, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Divides a 3-channel matrix into 3 single-channel matrices.
	/// 
	/// The function splits a 3-channel matrix into 3 single-channel matrices:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bmv%7D%20%5Bc%5D%28I%29%20%3D%20%20%5Ctexttt%7Bsrc%7D%20%28I%29%5Fc)
	/// 
	/// All output matrices must be of [CV_8UC1] type.
	/// 
	/// The function merge3 does the reverse operation.
	/// 
	/// 
	/// Note:
	///  - Function textual ID is "org.opencv.core.transform.split3"
	/// 
	/// ## Parameters
	/// * src: input [CV_8UC3] matrix.
	/// ## See also
	/// split4, merge3, merge4
	#[inline]
	pub fn split3(src: &crate::gapi::GMat) -> Result<core::Tuple<(crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat)>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_split3_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Tuple::<(crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat)>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Divides a 4-channel matrix into 4 single-channel matrices.
	/// 
	/// The function splits a 4-channel matrix into 4 single-channel matrices:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bmv%7D%20%5Bc%5D%28I%29%20%3D%20%20%5Ctexttt%7Bsrc%7D%20%28I%29%5Fc)
	/// 
	/// All output matrices must be of [CV_8UC1] type.
	/// 
	/// The function merge4 does the reverse operation.
	/// 
	/// 
	/// Note:
	///  - Function textual ID is "org.opencv.core.transform.split4"
	/// 
	/// ## Parameters
	/// * src: input [CV_8UC4] matrix.
	/// ## See also
	/// split3, merge3, merge4
	#[inline]
	pub fn split4(src: &crate::gapi::GMat) -> Result<core::Tuple<(crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat)>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_split4_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Tuple::<(crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat)>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates a square root of array elements.
	/// 
	/// The function cv::gapi::sqrt calculates a square root of each input array element.
	/// In case of multi-channel arrays, each channel is processed
	/// independently. The accuracy is approximately the same as of the built-in
	/// std::sqrt .
	/// ## Parameters
	/// * src: input floating-point array.
	/// ## Returns
	/// output array of the same size and type as src.
	#[inline]
	pub fn sqrt(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_sqrt_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Gets bgr plane from input frame
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.streaming.BGR"
	/// 
	/// ## Parameters
	/// * in: Input frame
	/// ## Returns
	/// Image in BGR format
	#[inline]
	pub fn bgr(in_: &crate::gapi::GFrame) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_streaming_BGR_const_GFrameR(in_.as_raw_GFrame(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Extracts UV plane from media frame.
	/// 
	/// Output image is 8-bit 2-channel image of [CV_8UC2].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.streaming.UV"
	/// 
	/// ## Parameters
	/// * frame: input media frame.
	#[inline]
	pub fn uv(frame: &crate::gapi::GFrame) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_streaming_UV_const_GFrameR(frame.as_raw_GFrame(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Extracts Y plane from media frame.
	/// 
	/// Output image is 8-bit 1-channel image of [CV_8UC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.streaming.Y"
	/// 
	/// ## Parameters
	/// * frame: input media frame.
	#[inline]
	pub fn y(frame: &crate::gapi::GFrame) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_streaming_Y_const_GFrameR(frame.as_raw_GFrame(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn desync_1(f: &crate::gapi::GFrame) -> Result<crate::gapi::GFrame> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_streaming_desync_const_GFrameR(f.as_raw_GFrame(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GFrame::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Starts a desynchronized branch in the graph.
	/// 
	/// This operation takes a single G-API data object and returns a
	/// graph-level "duplicate" of this object.
	/// 
	/// Operations which use this data object can be desynchronized
	/// from the rest of the graph.
	/// 
	/// This operation has no effect when a GComputation is compiled with
	/// regular cv::GComputation::compile(), since cv::GCompiled objects
	/// always produce their full output vectors.
	/// 
	/// This operation only makes sense when a GComputation is compiled in
	/// streaming mode with cv::GComputation::compileStreaming(). If this
	/// operation is used and there are desynchronized outputs, the user
	/// should use a special version of cv::GStreamingCompiled::pull()
	/// which produces an array of cv::util::optional<> objects.
	/// 
	/// 
	/// Note: This feature is highly experimental now and is currently
	/// limited to a single GMat/GFrame argument only.
	#[inline]
	pub fn desync(g: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_streaming_desync_const_GMatR(g.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn kernels() -> Result<crate::gapi::GKernelPackage> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_streaming_kernels(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GKernelPackage::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the per-element difference between matrix and given scalar.
	/// 
	/// The function can be replaced with matrix expressions:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%5Ctexttt%7Bsrc%7D%20%2D%20%5Ctexttt%7Bc%7D)
	/// 
	/// Depth of the output matrix is determined by the ddepth parameter.
	/// If ddepth is set to default -1, the depth of output matrix will be the same as the depth of input matrix.
	/// The matrices can be single or multi channel. Output matrix must have the same size as src.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.subC"
	/// ## Parameters
	/// * src: first input matrix.
	/// * c: scalar value to subtracted.
	/// * ddepth: optional depth of the output matrix.
	/// ## See also
	/// add, addC, subRC
	/// 
	/// ## Note
	/// This alternative version of [sub_c] function uses the following default values for its arguments:
	/// * ddepth: -1
	#[inline]
	pub fn sub_c_def(src: &crate::gapi::GMat, c: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_subC_const_GMatR_const_GScalarR(src.as_raw_GMat(), c.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the per-element difference between matrix and given scalar.
	/// 
	/// The function can be replaced with matrix expressions:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%5Ctexttt%7Bsrc%7D%20%2D%20%5Ctexttt%7Bc%7D)
	/// 
	/// Depth of the output matrix is determined by the ddepth parameter.
	/// If ddepth is set to default -1, the depth of output matrix will be the same as the depth of input matrix.
	/// The matrices can be single or multi channel. Output matrix must have the same size as src.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.subC"
	/// ## Parameters
	/// * src: first input matrix.
	/// * c: scalar value to subtracted.
	/// * ddepth: optional depth of the output matrix.
	/// ## See also
	/// add, addC, subRC
	/// 
	/// ## C++ default parameters
	/// * ddepth: -1
	#[inline]
	pub fn sub_c(src: &crate::gapi::GMat, c: &crate::gapi::GScalar, ddepth: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_subC_const_GMatR_const_GScalarR_int(src.as_raw_GMat(), c.as_raw_GScalar(), ddepth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the per-element difference between given scalar and the matrix.
	/// 
	/// The function can be replaced with matrix expressions:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%5Ctexttt%7Bc%7D%20%2D%20%5Ctexttt%7Bsrc%7D)
	/// 
	/// Depth of the output matrix is determined by the ddepth parameter.
	/// If ddepth is set to default -1, the depth of output matrix will be the same as the depth of input matrix.
	/// The matrices can be single or multi channel. Output matrix must have the same size as src.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.subRC"
	/// ## Parameters
	/// * c: scalar value to subtract from.
	/// * src: input matrix to be subtracted.
	/// * ddepth: optional depth of the output matrix.
	/// ## See also
	/// add, addC, subC
	/// 
	/// ## Note
	/// This alternative version of [sub_rc] function uses the following default values for its arguments:
	/// * ddepth: -1
	#[inline]
	pub fn sub_rc_def(c: &crate::gapi::GScalar, src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_subRC_const_GScalarR_const_GMatR(c.as_raw_GScalar(), src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the per-element difference between given scalar and the matrix.
	/// 
	/// The function can be replaced with matrix expressions:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%5Ctexttt%7Bc%7D%20%2D%20%5Ctexttt%7Bsrc%7D)
	/// 
	/// Depth of the output matrix is determined by the ddepth parameter.
	/// If ddepth is set to default -1, the depth of output matrix will be the same as the depth of input matrix.
	/// The matrices can be single or multi channel. Output matrix must have the same size as src.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.subRC"
	/// ## Parameters
	/// * c: scalar value to subtract from.
	/// * src: input matrix to be subtracted.
	/// * ddepth: optional depth of the output matrix.
	/// ## See also
	/// add, addC, subC
	/// 
	/// ## C++ default parameters
	/// * ddepth: -1
	#[inline]
	pub fn sub_rc(c: &crate::gapi::GScalar, src: &crate::gapi::GMat, ddepth: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_subRC_const_GScalarR_const_GMatR_int(c.as_raw_GScalar(), src.as_raw_GMat(), ddepth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the per-element difference between two matrices.
	/// 
	/// The function sub calculates difference between two matrices, when both matrices have the same size and the same number of
	/// channels:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%28I%29%20%3D%20%20%20%5Ctexttt%7Bsrc1%7D%28I%29%20%2D%20%20%5Ctexttt%7Bsrc2%7D%28I%29)
	/// 
	/// The function can be replaced with matrix expressions:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%20%5Ctexttt%7Bsrc1%7D%20%2D%20%20%5Ctexttt%7Bsrc2%7D)
	/// 
	/// The input matrices and the output matrix can all have the same or different depths. For example, you
	/// can subtract two 8-bit unsigned matrices store the result as a 16-bit signed matrix.
	/// Depth of the output matrix is determined by the ddepth parameter.
	/// If src1.depth() == src2.depth(), ddepth can be set to the default -1. In this case, the output matrix will have
	/// the same depth as the input matrices. The matrices can be single or multi channel.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.sub"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix.
	/// * ddepth: optional depth of the output matrix.
	/// ## See also
	/// add, addC
	/// 
	/// ## Note
	/// This alternative version of [sub] function uses the following default values for its arguments:
	/// * ddepth: -1
	#[inline]
	pub fn sub_def(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_sub_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates the per-element difference between two matrices.
	/// 
	/// The function sub calculates difference between two matrices, when both matrices have the same size and the same number of
	/// channels:
	///    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%28I%29%20%3D%20%20%20%5Ctexttt%7Bsrc1%7D%28I%29%20%2D%20%20%5Ctexttt%7Bsrc2%7D%28I%29)
	/// 
	/// The function can be replaced with matrix expressions:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%20%20%5Ctexttt%7Bsrc1%7D%20%2D%20%20%5Ctexttt%7Bsrc2%7D)
	/// 
	/// The input matrices and the output matrix can all have the same or different depths. For example, you
	/// can subtract two 8-bit unsigned matrices store the result as a 16-bit signed matrix.
	/// Depth of the output matrix is determined by the ddepth parameter.
	/// If src1.depth() == src2.depth(), ddepth can be set to the default -1. In this case, the output matrix will have
	/// the same depth as the input matrices. The matrices can be single or multi channel.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.math.sub"
	/// ## Parameters
	/// * src1: first input matrix.
	/// * src2: second input matrix.
	/// * ddepth: optional depth of the output matrix.
	/// ## See also
	/// add, addC
	/// 
	/// ## C++ default parameters
	/// * ddepth: -1
	#[inline]
	pub fn sub(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat, ddepth: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_sub_const_GMatR_const_GMatR_int(src1.as_raw_GMat(), src2.as_raw_GMat(), ddepth, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Calculates sum of all matrix elements.
	/// 
	/// The function sum calculates sum of all matrix elements, independently for each channel.
	/// 
	/// Supported matrix data types are [CV_8UC1], [CV_8UC3], [CV_16UC1], [CV_16SC1], [CV_32FC1].
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.matrixop.sum"
	/// ## Parameters
	/// * src: input matrix.
	/// ## See also
	/// countNonZero, mean, min, max
	#[inline]
	pub fn sum(src: &crate::gapi::GMat) -> Result<crate::gapi::GScalar> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_sum_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GScalar::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Applies a fixed-level threshold to each matrix element.
	/// 
	/// The function applies fixed-level thresholding to a single- or multiple-channel matrix.
	/// The function is typically used to get a bi-level (binary) image out of a grayscale image ( cmp functions could be also used for
	/// this purpose) or for removing a noise, that is, filtering out pixels with too small or too large
	/// values. There are several types of thresholding supported by the function. They are determined by
	/// type parameter.
	/// 
	/// Also, the special values cv::THRESH_OTSU or cv::THRESH_TRIANGLE may be combined with one of the
	/// above values. In these cases, the function determines the optimal threshold value using the Otsu's
	/// or Triangle algorithm and uses it instead of the specified thresh . The function returns the
	/// computed threshold value in addititon to thresholded matrix.
	/// The Otsu's and Triangle methods are implemented only for 8-bit matrices.
	/// 
	/// Input image should be single channel only in case of cv::THRESH_OTSU or cv::THRESH_TRIANGLE flags.
	/// Output matrix must be of the same size and depth as src.
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.matrixop.threshold"
	/// 
	/// ## Parameters
	/// * src: input matrix ([CV_8UC1], [CV_8UC3], or [CV_32FC1]).
	/// * thresh: threshold value.
	/// * maxval: maximum value to use with the cv::THRESH_BINARY and cv::THRESH_BINARY_INV thresholding
	/// types.
	/// * type: thresholding type (see the cv::ThresholdTypes).
	/// ## See also
	/// min, max, cmpGT, cmpLE, cmpGE, cmpLT
	#[inline]
	pub fn threshold(src: &crate::gapi::GMat, thresh: &crate::gapi::GScalar, maxval: &crate::gapi::GScalar, typ: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_threshold_const_GMatR_const_GScalarR_const_GScalarR_int(src.as_raw_GMat(), thresh.as_raw_GScalar(), maxval.as_raw_GScalar(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Applies a fixed-level threshold to each matrix element.
	/// 
	/// The function applies fixed-level thresholding to a single- or multiple-channel matrix.
	/// The function is typically used to get a bi-level (binary) image out of a grayscale image ( cmp functions could be also used for
	/// this purpose) or for removing a noise, that is, filtering out pixels with too small or too large
	/// values. There are several types of thresholding supported by the function. They are determined by
	/// type parameter.
	/// 
	/// Also, the special values cv::THRESH_OTSU or cv::THRESH_TRIANGLE may be combined with one of the
	/// above values. In these cases, the function determines the optimal threshold value using the Otsu's
	/// or Triangle algorithm and uses it instead of the specified thresh . The function returns the
	/// computed threshold value in addititon to thresholded matrix.
	/// The Otsu's and Triangle methods are implemented only for 8-bit matrices.
	/// 
	/// Input image should be single channel only in case of cv::THRESH_OTSU or cv::THRESH_TRIANGLE flags.
	/// Output matrix must be of the same size and depth as src.
	/// 
	/// 
	/// Note: Function textual ID is "org.opencv.core.matrixop.threshold"
	/// 
	/// ## Parameters
	/// * src: input matrix ([CV_8UC1], [CV_8UC3], or [CV_32FC1]).
	/// * thresh: threshold value.
	/// * maxval: maximum value to use with the cv::THRESH_BINARY and cv::THRESH_BINARY_INV thresholding
	/// types.
	/// * type: thresholding type (see the cv::ThresholdTypes).
	/// ## See also
	/// min, max, cmpGT, cmpLE, cmpGE, cmpLT
	/// 
	/// ## Overloaded parameters
	/// 
	/// This function applicable for all threshold types except cv::THRESH_OTSU and cv::THRESH_TRIANGLE
	/// 
	/// Note: Function textual ID is "org.opencv.core.matrixop.thresholdOT"
	#[inline]
	pub fn threshold_1(src: &crate::gapi::GMat, maxval: &crate::gapi::GScalar, typ: i32) -> Result<core::Tuple<(crate::gapi::GMat, crate::gapi::GScalar)>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_threshold_const_GMatR_const_GScalarR_int(src.as_raw_GMat(), maxval.as_raw_GScalar(), typ, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Tuple::<(crate::gapi::GMat, crate::gapi::GScalar)>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Transposes a matrix.
	/// 
	/// The function transposes the matrix:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28i%2Cj%29%20%3D%20%20%5Ctexttt%7Bsrc%7D%20%28j%2Ci%29)
	/// 
	/// 
	/// Note:
	///  - Function textual ID is "org.opencv.core.transpose"
	///  - No complex conjugation is done in case of a complex matrix. It should be done separately if needed.
	/// 
	/// ## Parameters
	/// * src: input array.
	#[inline]
	pub fn transpose(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_transpose_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Applies an affine transformation to an image.
	/// 
	/// The function warpAffine transforms the source image using the specified matrix:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Ctexttt%7Bsrc%7D%20%28%20%5Ctexttt%7BM%7D%20%5F%7B11%7D%20x%20%2B%20%20%5Ctexttt%7BM%7D%20%5F%7B12%7D%20y%20%2B%20%20%5Ctexttt%7BM%7D%20%5F%7B13%7D%2C%20%5Ctexttt%7BM%7D%20%5F%7B21%7D%20x%20%2B%20%20%5Ctexttt%7BM%7D%20%5F%7B22%7D%20y%20%2B%20%20%5Ctexttt%7BM%7D%20%5F%7B23%7D%29)
	/// 
	/// when the flag [WARP_INVERSE_MAP] is set. Otherwise, the transformation is first inverted
	/// with [invert_affine_transform] and then put in the formula above instead of M. The function cannot
	/// operate in-place.
	/// 
	/// ## Parameters
	/// * src: input image.
	/// * M: ![inline formula](https://latex.codecogs.com/png.latex?2%5Ctimes%203) transformation matrix.
	/// * dsize: size of the output image.
	/// * flags: combination of interpolation methods (see #InterpolationFlags) and the optional
	/// flag [WARP_INVERSE_MAP] that means that M is the inverse transformation (
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%5Crightarrow%5Ctexttt%7Bsrc%7D) ).
	/// * borderMode: pixel extrapolation method (see #BorderTypes);
	/// borderMode=[BORDER_TRANSPARENT] isn't supported
	/// * borderValue: value used in case of a constant border; by default, it is 0.
	/// ## See also
	/// warpPerspective, resize, remap, getRectSubPix, transform
	/// 
	/// ## Note
	/// This alternative version of [warp_affine] function uses the following default values for its arguments:
	/// * flags: cv::INTER_LINEAR
	/// * border_mode: cv::BORDER_CONSTANT
	/// * border_value: Scalar()
	#[inline]
	pub fn warp_affine_def(src: &crate::gapi::GMat, m: &core::Mat, dsize: core::Size) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_warpAffine_const_GMatR_const_MatR_const_SizeR(src.as_raw_GMat(), m.as_raw_Mat(), &dsize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Applies an affine transformation to an image.
	/// 
	/// The function warpAffine transforms the source image using the specified matrix:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Ctexttt%7Bsrc%7D%20%28%20%5Ctexttt%7BM%7D%20%5F%7B11%7D%20x%20%2B%20%20%5Ctexttt%7BM%7D%20%5F%7B12%7D%20y%20%2B%20%20%5Ctexttt%7BM%7D%20%5F%7B13%7D%2C%20%5Ctexttt%7BM%7D%20%5F%7B21%7D%20x%20%2B%20%20%5Ctexttt%7BM%7D%20%5F%7B22%7D%20y%20%2B%20%20%5Ctexttt%7BM%7D%20%5F%7B23%7D%29)
	/// 
	/// when the flag [WARP_INVERSE_MAP] is set. Otherwise, the transformation is first inverted
	/// with [invert_affine_transform] and then put in the formula above instead of M. The function cannot
	/// operate in-place.
	/// 
	/// ## Parameters
	/// * src: input image.
	/// * M: ![inline formula](https://latex.codecogs.com/png.latex?2%5Ctimes%203) transformation matrix.
	/// * dsize: size of the output image.
	/// * flags: combination of interpolation methods (see #InterpolationFlags) and the optional
	/// flag [WARP_INVERSE_MAP] that means that M is the inverse transformation (
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%5Crightarrow%5Ctexttt%7Bsrc%7D) ).
	/// * borderMode: pixel extrapolation method (see #BorderTypes);
	/// borderMode=[BORDER_TRANSPARENT] isn't supported
	/// * borderValue: value used in case of a constant border; by default, it is 0.
	/// ## See also
	/// warpPerspective, resize, remap, getRectSubPix, transform
	/// 
	/// ## C++ default parameters
	/// * flags: cv::INTER_LINEAR
	/// * border_mode: cv::BORDER_CONSTANT
	/// * border_value: Scalar()
	#[inline]
	pub fn warp_affine(src: &crate::gapi::GMat, m: &core::Mat, dsize: core::Size, flags: i32, border_mode: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_warpAffine_const_GMatR_const_MatR_const_SizeR_int_int_const_ScalarR(src.as_raw_GMat(), m.as_raw_Mat(), &dsize, flags, border_mode, &border_value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Applies a perspective transformation to an image.
	/// 
	/// The function warpPerspective transforms the source image using the specified matrix:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Ctexttt%7Bsrc%7D%20%5Cleft%20%28%20%5Cfrac%7BM%5F%7B11%7D%20x%20%2B%20M%5F%7B12%7D%20y%20%2B%20M%5F%7B13%7D%7D%7BM%5F%7B31%7D%20x%20%2B%20M%5F%7B32%7D%20y%20%2B%20M%5F%7B33%7D%7D%20%2C%0A%20%20%20%20%20%5Cfrac%7BM%5F%7B21%7D%20x%20%2B%20M%5F%7B22%7D%20y%20%2B%20M%5F%7B23%7D%7D%7BM%5F%7B31%7D%20x%20%2B%20M%5F%7B32%7D%20y%20%2B%20M%5F%7B33%7D%7D%20%5Cright%20%29)
	/// 
	/// when the flag [WARP_INVERSE_MAP] is set. Otherwise, the transformation is first inverted with invert
	/// and then put in the formula above instead of M. The function cannot operate in-place.
	/// 
	/// ## Parameters
	/// * src: input image.
	/// * M: ![inline formula](https://latex.codecogs.com/png.latex?3%5Ctimes%203) transformation matrix.
	/// * dsize: size of the output image.
	/// * flags: combination of interpolation methods ([INTER_LINEAR] or #INTER_NEAREST) and the
	/// optional flag #WARP_INVERSE_MAP, that sets M as the inverse transformation (
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%5Crightarrow%5Ctexttt%7Bsrc%7D) ).
	/// * borderMode: pixel extrapolation method ([BORDER_CONSTANT] or #BORDER_REPLICATE).
	/// * borderValue: value used in case of a constant border; by default, it equals 0.
	/// ## See also
	/// warpAffine, resize, remap, getRectSubPix, perspectiveTransform
	/// 
	/// ## Note
	/// This alternative version of [warp_perspective] function uses the following default values for its arguments:
	/// * flags: cv::INTER_LINEAR
	/// * border_mode: cv::BORDER_CONSTANT
	/// * border_value: Scalar()
	#[inline]
	pub fn warp_perspective_def(src: &crate::gapi::GMat, m: &core::Mat, dsize: core::Size) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_warpPerspective_const_GMatR_const_MatR_const_SizeR(src.as_raw_GMat(), m.as_raw_Mat(), &dsize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Applies a perspective transformation to an image.
	/// 
	/// The function warpPerspective transforms the source image using the specified matrix:
	/// 
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Ctexttt%7Bsrc%7D%20%5Cleft%20%28%20%5Cfrac%7BM%5F%7B11%7D%20x%20%2B%20M%5F%7B12%7D%20y%20%2B%20M%5F%7B13%7D%7D%7BM%5F%7B31%7D%20x%20%2B%20M%5F%7B32%7D%20y%20%2B%20M%5F%7B33%7D%7D%20%2C%0A%20%20%20%20%20%5Cfrac%7BM%5F%7B21%7D%20x%20%2B%20M%5F%7B22%7D%20y%20%2B%20M%5F%7B23%7D%7D%7BM%5F%7B31%7D%20x%20%2B%20M%5F%7B32%7D%20y%20%2B%20M%5F%7B33%7D%7D%20%5Cright%20%29)
	/// 
	/// when the flag [WARP_INVERSE_MAP] is set. Otherwise, the transformation is first inverted with invert
	/// and then put in the formula above instead of M. The function cannot operate in-place.
	/// 
	/// ## Parameters
	/// * src: input image.
	/// * M: ![inline formula](https://latex.codecogs.com/png.latex?3%5Ctimes%203) transformation matrix.
	/// * dsize: size of the output image.
	/// * flags: combination of interpolation methods ([INTER_LINEAR] or #INTER_NEAREST) and the
	/// optional flag #WARP_INVERSE_MAP, that sets M as the inverse transformation (
	/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%5Crightarrow%5Ctexttt%7Bsrc%7D) ).
	/// * borderMode: pixel extrapolation method ([BORDER_CONSTANT] or #BORDER_REPLICATE).
	/// * borderValue: value used in case of a constant border; by default, it equals 0.
	/// ## See also
	/// warpAffine, resize, remap, getRectSubPix, perspectiveTransform
	/// 
	/// ## C++ default parameters
	/// * flags: cv::INTER_LINEAR
	/// * border_mode: cv::BORDER_CONSTANT
	/// * border_value: Scalar()
	#[inline]
	pub fn warp_perspective(src: &crate::gapi::GMat, m: &core::Mat, dsize: core::Size, flags: i32, border_mode: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_warpPerspective_const_GMatR_const_MatR_const_SizeR_int_int_const_ScalarR(src.as_raw_GMat(), m.as_raw_Mat(), &dsize, flags, border_mode, &border_value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn add_gmat_gmat(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorA_const_GMatR_const_GMatR(lhs.as_raw_GMat(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn add_gmat_gscalar(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorA_const_GMatR_const_GScalarR(lhs.as_raw_GMat(), rhs.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn add_gscalar_gmat(lhs: &crate::gapi::GScalar, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorA_const_GScalarR_const_GMatR(lhs.as_raw_GScalar(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn div_gmat_gmat(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorD_const_GMatR_const_GMatR(lhs.as_raw_GMat(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn div_gmat_gscalar(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorD_const_GMatR_const_GScalarR(lhs.as_raw_GMat(), rhs.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn div_gscalar_gmat(lhs: &crate::gapi::GScalar, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorD_const_GScalarR_const_GMatR(lhs.as_raw_GScalar(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn equals_gmat_gmat(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorEQ_const_GMatR_const_GMatR(lhs.as_raw_GMat(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn equals_gmat_gscalar(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorEQ_const_GMatR_const_GScalarR(lhs.as_raw_GMat(), rhs.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn equals_gscalar_gmat(lhs: &crate::gapi::GScalar, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorEQ_const_GScalarR_const_GMatR(lhs.as_raw_GScalar(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn greater_than_or_equal_gmat_gmat(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorGE_const_GMatR_const_GMatR(lhs.as_raw_GMat(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn greater_than_or_equal_gmat_gscalar(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorGE_const_GMatR_const_GScalarR(lhs.as_raw_GMat(), rhs.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn greater_than_or_equal_gscalar_gmat(lhs: &crate::gapi::GScalar, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorGE_const_GScalarR_const_GMatR(lhs.as_raw_GScalar(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn greater_than_gmat_gmat(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorG_const_GMatR_const_GMatR(lhs.as_raw_GMat(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn greater_than_gmat_gscalar(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorG_const_GMatR_const_GScalarR(lhs.as_raw_GMat(), rhs.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn greater_than_gscalar_gmat(lhs: &crate::gapi::GScalar, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorG_const_GScalarR_const_GMatR(lhs.as_raw_GScalar(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn less_than_or_equal_gmat_gmat(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorLE_const_GMatR_const_GMatR(lhs.as_raw_GMat(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn less_than_or_equal_gmat_gscalar(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorLE_const_GMatR_const_GScalarR(lhs.as_raw_GMat(), rhs.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn less_than_or_equal_gscalar_gmat(lhs: &crate::gapi::GScalar, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorLE_const_GScalarR_const_GMatR(lhs.as_raw_GScalar(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn less_than_gmat_gmat(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorL_const_GMatR_const_GMatR(lhs.as_raw_GMat(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn less_than_gmat_gscalar(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorL_const_GMatR_const_GScalarR(lhs.as_raw_GMat(), rhs.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn less_than_gscalar_gmat(lhs: &crate::gapi::GScalar, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorL_const_GScalarR_const_GMatR(lhs.as_raw_GScalar(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn not_equals_gmat_gmat(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorNE_const_GMatR_const_GMatR(lhs.as_raw_GMat(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn not_equals_gmat_gscalar(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorNE_const_GMatR_const_GScalarR(lhs.as_raw_GMat(), rhs.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn not_equals_gscalar_gmat(lhs: &crate::gapi::GScalar, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorNE_const_GScalarR_const_GMatR(lhs.as_raw_GScalar(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn negate(lhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorNOTB_const_GMatR(lhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn or_gmat_gmat(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorOR_const_GMatR_const_GMatR(lhs.as_raw_GMat(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn or_gmat_gscalar(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorOR_const_GMatR_const_GScalarR(lhs.as_raw_GMat(), rhs.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn or_gscalar_gmat(lhs: &crate::gapi::GScalar, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorOR_const_GScalarR_const_GMatR(lhs.as_raw_GScalar(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn and_gmat_gmat(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorR_const_GMatR_const_GMatR(lhs.as_raw_GMat(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn and_gmat_gscalar(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorR_const_GMatR_const_GScalarR(lhs.as_raw_GMat(), rhs.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn and_gscalar_gmat(lhs: &crate::gapi::GScalar, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorR_const_GScalarR_const_GMatR(lhs.as_raw_GScalar(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn sub_gmat_gmat(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorS_const_GMatR_const_GMatR(lhs.as_raw_GMat(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn sub_gmat_gscalar(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorS_const_GMatR_const_GScalarR(lhs.as_raw_GMat(), rhs.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn sub_gscalar_gmat(lhs: &crate::gapi::GScalar, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorS_const_GScalarR_const_GMatR(lhs.as_raw_GScalar(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn xor_gmat_gmat(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorXOR_const_GMatR_const_GMatR(lhs.as_raw_GMat(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn xor_gmat_gscalar(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorXOR_const_GMatR_const_GScalarR(lhs.as_raw_GMat(), rhs.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn xor_gscalar_gmat(lhs: &crate::gapi::GScalar, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorXOR_const_GScalarR_const_GMatR(lhs.as_raw_GScalar(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn mul_gmat_gscalar(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorX_const_GMatR_const_GScalarR(lhs.as_raw_GMat(), rhs.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn mul_gmat_f32(lhs: &crate::gapi::GMat, rhs: f32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorX_const_GMatR_float(lhs.as_raw_GMat(), rhs, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn mul_gscalar_gmat(lhs: &crate::gapi::GScalar, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorX_const_GScalarR_const_GMatR(lhs.as_raw_GScalar(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn mul_f32_gmat(lhs: f32, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_operatorX_float_const_GMatR(lhs, rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn validate_input_arg(arg: &crate::gapi::GRunArg) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_validate_input_arg_const_GRunArgR(arg.as_raw_GRunArg(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn validate_input_args(args: &crate::gapi::GRunArgs) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_validate_input_args_const_GRunArgsR(args.as_raw_VectorOfGRunArg(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Constant methods for [crate::gapi::GArg]
	pub trait GArgTraitConst {
		fn as_raw_GArg(&self) -> *const c_void;
	
		#[inline]
		fn kind(&self) -> crate::gapi::Detail_ArgKind {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GArg_propKind_const(self.as_raw_GArg(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		#[inline]
		fn opaque_kind(&self) -> crate::gapi::Detail_OpaqueKind {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GArg_propOpaque_kind_const(self.as_raw_GArg(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
	}
	
	/// Mutable methods for [crate::gapi::GArg]
	pub trait GArgTrait: crate::gapi::GArgTraitConst {
		fn as_raw_mut_GArg(&mut self) -> *mut c_void;
	
		/// ## C++ default parameters
		/// * val: detail::ArgKind::OPAQUE_VAL
		#[inline]
		fn set_kind(&mut self, val: crate::gapi::Detail_ArgKind) {
			let ret = unsafe { sys::cv_GArg_propKind_ArgKind(self.as_raw_mut_GArg(), val) };
			ret
		}
		
		/// ## C++ default parameters
		/// * val: detail::OpaqueKind::CV_UNKNOWN
		#[inline]
		fn set_opaque_kind(&mut self, val: crate::gapi::Detail_OpaqueKind) {
			let ret = unsafe { sys::cv_GArg_propOpaque_kind_OpaqueKind(self.as_raw_mut_GArg(), val) };
			ret
		}
		
	}
	
	pub struct GArg {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GArg }
	
	impl Drop for GArg {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GArg_delete(self.as_raw_mut_GArg()) };
		}
	}
	
	unsafe impl Send for GArg {}
	
	impl crate::gapi::GArgTraitConst for GArg {
		#[inline] fn as_raw_GArg(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GArgTrait for GArg {
		#[inline] fn as_raw_mut_GArg(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GArg {
		#[inline]
		pub fn default() -> Result<crate::gapi::GArg> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GArg_GArg(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GArg::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for GArg {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GArg")
				.field("kind", &crate::gapi::GArgTraitConst::kind(self))
				.field("opaque_kind", &crate::gapi::GArgTraitConst::opaque_kind(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::GArrayDesc]
	pub trait GArrayDescTraitConst {
		fn as_raw_GArrayDesc(&self) -> *const c_void;
	
		#[inline]
		fn equals(&self, unnamed: &crate::gapi::GArrayDesc) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GArrayDesc_operatorEQ_const_const_GArrayDescR(self.as_raw_GArrayDesc(), unnamed.as_raw_GArrayDesc(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::gapi::GArrayDesc]
	pub trait GArrayDescTrait: crate::gapi::GArrayDescTraitConst {
		fn as_raw_mut_GArrayDesc(&mut self) -> *mut c_void;
	
	}
	
	/// \addtogroup gapi_meta_args
	pub struct GArrayDesc {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GArrayDesc }
	
	impl Drop for GArrayDesc {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GArrayDesc_delete(self.as_raw_mut_GArrayDesc()) };
		}
	}
	
	unsafe impl Send for GArrayDesc {}
	
	impl crate::gapi::GArrayDescTraitConst for GArrayDesc {
		#[inline] fn as_raw_GArrayDesc(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GArrayDescTrait for GArrayDesc {
		#[inline] fn as_raw_mut_GArrayDesc(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GArrayDesc {
		fn default() -> Self {
			unsafe { Self::from_raw(sys::cv_GArrayDesc_defaultNew_const()) }
		}
		
	}
	
	impl Clone for GArrayDesc {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_GArrayDesc_implicitClone_const(self.as_raw_GArrayDesc())) }
		}
	}
	
	impl std::fmt::Debug for GArrayDesc {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GArrayDesc")
				.finish()
		}
	}
	
	impl Default for GArrayDesc {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}
	
	/// Constant methods for [crate::gapi::GCall]
	pub trait GCallTraitConst {
		fn as_raw_GCall(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::gapi::GCall]
	pub trait GCallTrait: crate::gapi::GCallTraitConst {
		fn as_raw_mut_GCall(&mut self) -> *mut c_void;
	
		/// ## C++ default parameters
		/// * output: 0
		#[inline]
		fn yield_(&mut self, output: i32) -> Result<crate::gapi::GMat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GCall_yield_int(self.as_raw_mut_GCall(), output, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [yield_] function uses the following default values for its arguments:
		/// * output: 0
		#[inline]
		fn yield__def(&mut self) -> Result<crate::gapi::GMat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GCall_yield(self.as_raw_mut_GCall(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * output: 0
		#[inline]
		fn yield_p(&mut self, output: i32) -> Result<crate::gapi::GMatP> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GCall_yieldP_int(self.as_raw_mut_GCall(), output, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GMatP::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [yield_p] function uses the following default values for its arguments:
		/// * output: 0
		#[inline]
		fn yield_p_def(&mut self) -> Result<crate::gapi::GMatP> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GCall_yieldP(self.as_raw_mut_GCall(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GMatP::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * output: 0
		#[inline]
		fn yield_scalar(&mut self, output: i32) -> Result<crate::gapi::GScalar> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GCall_yieldScalar_int(self.as_raw_mut_GCall(), output, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GScalar::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [yield_scalar] function uses the following default values for its arguments:
		/// * output: 0
		#[inline]
		fn yield_scalar_def(&mut self) -> Result<crate::gapi::GScalar> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GCall_yieldScalar(self.as_raw_mut_GCall(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GScalar::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * output: 0
		#[inline]
		fn yield_frame(&mut self, output: i32) -> Result<crate::gapi::GFrame> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GCall_yieldFrame_int(self.as_raw_mut_GCall(), output, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GFrame::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [yield_frame] function uses the following default values for its arguments:
		/// * output: 0
		#[inline]
		fn yield_frame_def(&mut self) -> Result<crate::gapi::GFrame> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GCall_yieldFrame(self.as_raw_mut_GCall(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GFrame::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn kernel(&mut self) -> Result<crate::gapi::GKernel> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GCall_kernel(self.as_raw_mut_GCall(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GKernel::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn params(&mut self) -> Result<crate::gapi::any> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GCall_params(self.as_raw_mut_GCall(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::any::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn set_args(&mut self, mut args: core::Vector<crate::gapi::GArg>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GCall_setArgs_vectorLGArgGRR(self.as_raw_mut_GCall(), args.as_raw_mut_VectorOfGArg(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct GCall {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GCall }
	
	impl Drop for GCall {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GCall_delete(self.as_raw_mut_GCall()) };
		}
	}
	
	unsafe impl Send for GCall {}
	
	impl crate::gapi::GCallTraitConst for GCall {
		#[inline] fn as_raw_GCall(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GCallTrait for GCall {
		#[inline] fn as_raw_mut_GCall(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GCall {
		#[inline]
		pub fn new(k: &crate::gapi::GKernel) -> Result<crate::gapi::GCall> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GCall_GCall_const_GKernelR(k.as_raw_GKernel(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GCall::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for GCall {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GCall")
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::GCompileArg]
	pub trait GCompileArgTraitConst {
		fn as_raw_GCompileArg(&self) -> *const c_void;
	
		#[inline]
		fn tag(&self) -> String {
			let ret = unsafe { sys::cv_GCompileArg_propTag_const(self.as_raw_GCompileArg()) };
			let ret = unsafe { String::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::gapi::GCompileArg]
	pub trait GCompileArgTrait: crate::gapi::GCompileArgTraitConst {
		fn as_raw_mut_GCompileArg(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_tag(&mut self, val: &str) {
			extern_container_arg!(nofail mut val);
			let ret = unsafe { sys::cv_GCompileArg_propTag_string(self.as_raw_mut_GCompileArg(), val.opencv_as_extern_mut()) };
			ret
		}
		
	}
	
	/// Represents an arbitrary compilation argument.
	/// 
	/// Any value can be wrapped into cv::GCompileArg, but only known ones
	/// (to G-API or its backends) can be interpreted correctly.
	/// 
	/// Normally objects of this class shouldn't be created manually, use
	/// cv::compile_args() function which automatically wraps everything
	/// passed in (a variadic template parameter pack) into a vector of
	/// cv::GCompileArg objects.
	pub struct GCompileArg {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GCompileArg }
	
	impl Drop for GCompileArg {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GCompileArg_delete(self.as_raw_mut_GCompileArg()) };
		}
	}
	
	unsafe impl Send for GCompileArg {}
	
	impl crate::gapi::GCompileArgTraitConst for GCompileArg {
		#[inline] fn as_raw_GCompileArg(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GCompileArgTrait for GCompileArg {
		#[inline] fn as_raw_mut_GCompileArg(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GCompileArg {
		#[inline]
		pub fn default() -> crate::gapi::GCompileArg {
			let ret = unsafe { sys::cv_GCompileArg_GCompileArg() };
			let ret = unsafe { crate::gapi::GCompileArg::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	impl std::fmt::Debug for GCompileArg {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GCompileArg")
				.field("tag", &crate::gapi::GCompileArgTraitConst::tag(self))
				.finish()
		}
	}
	
	impl Default for GCompileArg {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}
	
	/// Constant methods for [crate::gapi::GCompiled]
	pub trait GCompiledTraitConst {
		fn as_raw_GCompiled(&self) -> *const c_void;
	
		/// Check if compiled object is valid (non-empty)
		/// 
		/// ## Returns
		/// true if the object is runnable (valid), false otherwise
		#[inline]
		fn to_bool(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GCompiled_operator_bool_const(self.as_raw_GCompiled(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Check if the underlying backends support reshape or not.
		/// 
		/// ## Returns
		/// true if supported, false otherwise.
		#[inline]
		fn can_reshape(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GCompiled_canReshape_const(self.as_raw_GCompiled(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::gapi::GCompiled]
	pub trait GCompiledTrait: crate::gapi::GCompiledTraitConst {
		fn as_raw_mut_GCompiled(&mut self) -> *mut c_void;
	
		/// Execute an unary computation
		/// 
		///  Run the compiled computation, a generic version.
		/// 
		/// ## Parameters
		/// * ins: vector of inputs to process.
		/// * outs: vector of outputs to produce.
		/// 
		/// Input/output vectors must have the same number of elements as
		/// defined in the cv::GComputation protocol (at the moment of its
		/// construction). Shapes of elements also must conform to protocol
		/// (e.g. cv::Mat needs to be passed where cv::GMat has been
		/// declared as input, and so on). Run-time exception is generated
		/// otherwise.
		/// 
		/// Objects in output vector may remain empty (like cv::Mat) --
		/// G-API will automatically initialize output objects to proper formats.
		/// 
		/// 
		/// Note: Don't construct GRunArgs/GRunArgsP objects manually, use
		/// cv::gin()/cv::gout() wrappers instead.
		/// 
		/// ## Overloaded parameters
		/// 
		/// * in: input cv::Mat for unary computation
		/// * out: output cv::Mat for unary computation
		///  process.
		#[inline]
		fn apply(&mut self, mut in_: core::Mat, out: &mut core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GCompiled_operator___Mat_MatR(self.as_raw_mut_GCompiled(), in_.as_raw_mut_Mat(), out.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Execute an unary computation
		/// 
		///  Run the compiled computation, a generic version.
		/// 
		/// ## Parameters
		/// * ins: vector of inputs to process.
		/// * outs: vector of outputs to produce.
		/// 
		/// Input/output vectors must have the same number of elements as
		/// defined in the cv::GComputation protocol (at the moment of its
		/// construction). Shapes of elements also must conform to protocol
		/// (e.g. cv::Mat needs to be passed where cv::GMat has been
		/// declared as input, and so on). Run-time exception is generated
		/// otherwise.
		/// 
		/// Objects in output vector may remain empty (like cv::Mat) --
		/// G-API will automatically initialize output objects to proper formats.
		/// 
		/// 
		/// Note: Don't construct GRunArgs/GRunArgsP objects manually, use
		/// cv::gin()/cv::gout() wrappers instead.
		/// 
		/// ## Overloaded parameters
		/// 
		/// * in: input cv::Mat for unary computation
		/// * out: output cv::Scalar for unary computation
		///  process.
		#[inline]
		fn apply_1(&mut self, mut in_: core::Mat, out: &mut core::Scalar) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GCompiled_operator___Mat_ScalarR(self.as_raw_mut_GCompiled(), in_.as_raw_mut_Mat(), out, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Execute a binary computation
		/// 
		///  Run the compiled computation, a generic version.
		/// 
		/// ## Parameters
		/// * ins: vector of inputs to process.
		/// * outs: vector of outputs to produce.
		/// 
		/// Input/output vectors must have the same number of elements as
		/// defined in the cv::GComputation protocol (at the moment of its
		/// construction). Shapes of elements also must conform to protocol
		/// (e.g. cv::Mat needs to be passed where cv::GMat has been
		/// declared as input, and so on). Run-time exception is generated
		/// otherwise.
		/// 
		/// Objects in output vector may remain empty (like cv::Mat) --
		/// G-API will automatically initialize output objects to proper formats.
		/// 
		/// 
		/// Note: Don't construct GRunArgs/GRunArgsP objects manually, use
		/// cv::gin()/cv::gout() wrappers instead.
		/// 
		/// ## Overloaded parameters
		/// 
		/// * in1: first input cv::Mat for binary computation
		/// * in2: second input cv::Mat for binary computation
		/// * out: output cv::Mat for binary computation
		///  process.
		#[inline]
		fn apply_2(&mut self, mut in1: core::Mat, mut in2: core::Mat, out: &mut core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GCompiled_operator___Mat_Mat_MatR(self.as_raw_mut_GCompiled(), in1.as_raw_mut_Mat(), in2.as_raw_mut_Mat(), out.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Execute an binary computation
		/// 
		///  Run the compiled computation, a generic version.
		/// 
		/// ## Parameters
		/// * ins: vector of inputs to process.
		/// * outs: vector of outputs to produce.
		/// 
		/// Input/output vectors must have the same number of elements as
		/// defined in the cv::GComputation protocol (at the moment of its
		/// construction). Shapes of elements also must conform to protocol
		/// (e.g. cv::Mat needs to be passed where cv::GMat has been
		/// declared as input, and so on). Run-time exception is generated
		/// otherwise.
		/// 
		/// Objects in output vector may remain empty (like cv::Mat) --
		/// G-API will automatically initialize output objects to proper formats.
		/// 
		/// 
		/// Note: Don't construct GRunArgs/GRunArgsP objects manually, use
		/// cv::gin()/cv::gout() wrappers instead.
		/// 
		/// ## Overloaded parameters
		/// 
		/// * in1: first input cv::Mat for binary computation
		/// * in2: second input cv::Mat for binary computation
		/// * out: output cv::Scalar for binary computation
		///  process.
		#[inline]
		fn apply_3(&mut self, mut in1: core::Mat, mut in2: core::Mat, out: &mut core::Scalar) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GCompiled_operator___Mat_Mat_ScalarR(self.as_raw_mut_GCompiled(), in1.as_raw_mut_Mat(), in2.as_raw_mut_Mat(), out, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Execute a computation with arbitrary number of
		///  inputs/outputs.
		/// 
		///  Run the compiled computation, a generic version.
		/// 
		/// ## Parameters
		/// * ins: vector of inputs to process.
		/// * outs: vector of outputs to produce.
		/// 
		/// Input/output vectors must have the same number of elements as
		/// defined in the cv::GComputation protocol (at the moment of its
		/// construction). Shapes of elements also must conform to protocol
		/// (e.g. cv::Mat needs to be passed where cv::GMat has been
		/// declared as input, and so on). Run-time exception is generated
		/// otherwise.
		/// 
		/// Objects in output vector may remain empty (like cv::Mat) --
		/// G-API will automatically initialize output objects to proper formats.
		/// 
		/// 
		/// Note: Don't construct GRunArgs/GRunArgsP objects manually, use
		/// cv::gin()/cv::gout() wrappers instead.
		/// 
		/// ## Overloaded parameters
		/// 
		/// * ins: vector of input cv::Mat objects to process by the
		///  computation.
		/// * outs: vector of output cv::Mat objects to produce by the
		///  computation.
		/// 
		///  Numbers of elements in ins/outs vectors must match numbers of
		///  inputs/outputs which were used to define the source GComputation.
		#[inline]
		fn apply_4(&mut self, ins: &core::Vector<core::Mat>, outs: &core::Vector<core::Mat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GCompiled_operator___const_vectorLMatGR_const_vectorLMatGR(self.as_raw_mut_GCompiled(), ins.as_raw_VectorOfMat(), outs.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Prepare inner kernels states for a new video-stream.
		/// 
		/// GCompiled objects may be used to process video streams frame by frame.
		/// In this case, a GCompiled is called on every image frame individually.
		/// Starting OpenCV 4.4, some kernels in the graph may have their internal
		/// states (see GAPI_OCV_KERNEL_ST for the OpenCV backend).
		/// In this case, if user starts processing another video stream with
		/// this GCompiled, this method needs to be called to let kernels re-initialize
		/// their internal states to a new video stream.
		#[inline]
		fn prepare_for_new_stream(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GCompiled_prepareForNewStream(self.as_raw_mut_GCompiled(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// \addtogroup gapi_main_classes
	/// /
	/// 
	///  Represents a compiled computation (graph). Can only be used
	///  with image / data formats & resolutions it was compiled for, with
	///  some exceptions.
	/// 
	///  This class represents a product of graph compilation (calling
	///  cv::GComputation::compile()). Objects of this class actually do
	///  data processing, and graph execution is incapsulated into objects
	///  of this class. Execution model itself depends on kernels and
	///  backends which were using during the compilation, see [gapi_compile_args] for details.
	/// 
	///  In a general case, GCompiled objects can be applied to data only in
	///  that formats/resolutions they were compiled for (see [gapi_meta_args]). However, if the underlying backends allow, a
	///  compiled object can be _reshaped_ to handle data (images) of
	///  different resolution, though formats and types must remain the same.
	/// 
	///  GCompiled is very similar to `std::function<>` in its semantics --
	///  running it looks like a function call in the user code.
	/// 
	///  At the moment, GCompiled objects are not reentrant -- generally,
	///  the objects are stateful since graph execution itself is a stateful
	///  process and this state is now maintained in GCompiled's own memory
	///  (not on the process stack).
	/// 
	///  At the same time, two different GCompiled objects produced from the
	///  single cv::GComputation are completely independent and can be used
	///  concurrently.
	/// ## See also
	/// GStreamingCompiled
	pub struct GCompiled {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GCompiled }
	
	impl Drop for GCompiled {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GCompiled_delete(self.as_raw_mut_GCompiled()) };
		}
	}
	
	unsafe impl Send for GCompiled {}
	
	impl crate::gapi::GCompiledTraitConst for GCompiled {
		#[inline] fn as_raw_GCompiled(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GCompiledTrait for GCompiled {
		#[inline] fn as_raw_mut_GCompiled(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GCompiled {
		/// Constructs an empty object
		#[inline]
		pub fn default() -> Result<crate::gapi::GCompiled> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GCompiled_GCompiled(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GCompiled::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for GCompiled {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GCompiled")
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::GComputation]
	pub trait GComputationTraitConst {
		fn as_raw_GComputation(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::gapi::GComputation]
	pub trait GComputationTrait: crate::gapi::GComputationTraitConst {
		fn as_raw_mut_GComputation(&mut self) -> *mut c_void;
	
		/// @private -- Exclude this function from OpenCV documentation
		/// 
		/// ## C++ default parameters
		/// * args: {}
		#[inline]
		fn apply(&mut self, callback: &crate::gapi::Detail_ExtractArgsCallback, mut args: crate::gapi::GCompileArgs) -> Result<core::Vector<crate::gapi::GRunArg>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_apply_const_ExtractArgsCallbackR_GCompileArgsRR(self.as_raw_mut_GComputation(), callback.as_raw_Detail_ExtractArgsCallback(), args.as_raw_mut_VectorOfGCompileArg(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<crate::gapi::GRunArg>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// @private -- Exclude this function from OpenCV documentation
		/// 
		/// ## Note
		/// This alternative version of [apply] function uses the following default values for its arguments:
		/// * args: {}
		#[inline]
		fn apply_def(&mut self, callback: &crate::gapi::Detail_ExtractArgsCallback) -> Result<core::Vector<crate::gapi::GRunArg>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_apply_const_ExtractArgsCallbackR(self.as_raw_mut_GComputation(), callback.as_raw_Detail_ExtractArgsCallback(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<crate::gapi::GRunArg>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// @private -- Exclude this function from OpenCV documentation
		/// 
		/// ## C++ default parameters
		/// * args: {}
		#[inline]
		fn apply_1(&mut self, ins: &core::Vector<core::Mat>, outs: &core::Vector<core::Mat>, mut args: crate::gapi::GCompileArgs) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_apply_const_vectorLMatGR_const_vectorLMatGR_GCompileArgsRR(self.as_raw_mut_GComputation(), ins.as_raw_VectorOfMat(), outs.as_raw_VectorOfMat(), args.as_raw_mut_VectorOfGCompileArg(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// @private -- Exclude this function from OpenCV documentation
		/// 
		/// ## Note
		/// This alternative version of [apply] function uses the following default values for its arguments:
		/// * args: {}
		#[inline]
		fn apply_def_1(&mut self, ins: &core::Vector<core::Mat>, outs: &core::Vector<core::Mat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_apply_const_vectorLMatGR_const_vectorLMatGR(self.as_raw_mut_GComputation(), ins.as_raw_VectorOfMat(), outs.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Execute an unary computation (with compilation on the fly)
		/// 
		///  @private -- Exclude this function from OpenCV documentation
		/// 
		/// ## Overloaded parameters
		/// 
		/// ## Parameters
		/// * in: input cv::Mat for unary computation
		/// * out: output cv::Mat for unary computation
		/// * args: compilation arguments for underlying compilation
		///  process.
		/// 
		/// ## C++ default parameters
		/// * args: {}
		#[inline]
		fn apply_2(&mut self, mut in_: core::Mat, out: &mut core::Mat, mut args: crate::gapi::GCompileArgs) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_apply_Mat_MatR_GCompileArgsRR(self.as_raw_mut_GComputation(), in_.as_raw_mut_Mat(), out.as_raw_mut_Mat(), args.as_raw_mut_VectorOfGCompileArg(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Execute an unary computation (with compilation on the fly)
		/// 
		/// @overload
		/// ## Parameters
		/// * in: input cv::Mat for unary computation
		/// * out: output cv::Mat for unary computation
		/// * args: compilation arguments for underlying compilation
		/// process.
		/// 
		/// ## Note
		/// This alternative version of [apply] function uses the following default values for its arguments:
		/// * args: {}
		#[inline]
		fn apply_def_2(&mut self, mut in_: core::Mat, out: &mut core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_apply_Mat_MatR(self.as_raw_mut_GComputation(), in_.as_raw_mut_Mat(), out.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Execute an unary computation (with compilation on the fly)
		/// 
		///  @private -- Exclude this function from OpenCV documentation
		/// 
		/// ## Overloaded parameters
		/// 
		/// ## Parameters
		/// * in: input cv::Mat for unary computation
		/// * out: output cv::Scalar for unary computation
		/// * args: compilation arguments for underlying compilation
		///  process.
		/// 
		/// ## C++ default parameters
		/// * args: {}
		#[inline]
		fn apply_3(&mut self, mut in_: core::Mat, out: &mut core::Scalar, mut args: crate::gapi::GCompileArgs) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_apply_Mat_ScalarR_GCompileArgsRR(self.as_raw_mut_GComputation(), in_.as_raw_mut_Mat(), out, args.as_raw_mut_VectorOfGCompileArg(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Execute an unary computation (with compilation on the fly)
		/// 
		/// @overload
		/// ## Parameters
		/// * in: input cv::Mat for unary computation
		/// * out: output cv::Scalar for unary computation
		/// * args: compilation arguments for underlying compilation
		/// process.
		/// 
		/// ## Note
		/// This alternative version of [apply] function uses the following default values for its arguments:
		/// * args: {}
		#[inline]
		fn apply_def_3(&mut self, mut in_: core::Mat, out: &mut core::Scalar) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_apply_Mat_ScalarR(self.as_raw_mut_GComputation(), in_.as_raw_mut_Mat(), out, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Execute a binary computation (with compilation on the fly)
		/// 
		///  @private -- Exclude this function from OpenCV documentation
		/// 
		/// ## Overloaded parameters
		/// 
		/// ## Parameters
		/// * in1: first input cv::Mat for binary computation
		/// * in2: second input cv::Mat for binary computation
		/// * out: output cv::Mat for binary computation
		/// * args: compilation arguments for underlying compilation
		///  process.
		/// 
		/// ## C++ default parameters
		/// * args: {}
		#[inline]
		fn apply_4(&mut self, mut in1: core::Mat, mut in2: core::Mat, out: &mut core::Mat, mut args: crate::gapi::GCompileArgs) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_apply_Mat_Mat_MatR_GCompileArgsRR(self.as_raw_mut_GComputation(), in1.as_raw_mut_Mat(), in2.as_raw_mut_Mat(), out.as_raw_mut_Mat(), args.as_raw_mut_VectorOfGCompileArg(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Execute a binary computation (with compilation on the fly)
		/// 
		/// @overload
		/// ## Parameters
		/// * in1: first input cv::Mat for binary computation
		/// * in2: second input cv::Mat for binary computation
		/// * out: output cv::Mat for binary computation
		/// * args: compilation arguments for underlying compilation
		/// process.
		/// 
		/// ## Note
		/// This alternative version of [apply] function uses the following default values for its arguments:
		/// * args: {}
		#[inline]
		fn apply_def_4(&mut self, mut in1: core::Mat, mut in2: core::Mat, out: &mut core::Mat) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_apply_Mat_Mat_MatR(self.as_raw_mut_GComputation(), in1.as_raw_mut_Mat(), in2.as_raw_mut_Mat(), out.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Execute an binary computation (with compilation on the fly)
		/// 
		///  @private -- Exclude this function from OpenCV documentation
		/// 
		/// ## Overloaded parameters
		/// 
		/// ## Parameters
		/// * in1: first input cv::Mat for binary computation
		/// * in2: second input cv::Mat for binary computation
		/// * out: output cv::Scalar for binary computation
		/// * args: compilation arguments for underlying compilation
		///  process.
		/// 
		/// ## C++ default parameters
		/// * args: {}
		#[inline]
		fn apply_5(&mut self, mut in1: core::Mat, mut in2: core::Mat, out: &mut core::Scalar, mut args: crate::gapi::GCompileArgs) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_apply_Mat_Mat_ScalarR_GCompileArgsRR(self.as_raw_mut_GComputation(), in1.as_raw_mut_Mat(), in2.as_raw_mut_Mat(), out, args.as_raw_mut_VectorOfGCompileArg(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Execute an binary computation (with compilation on the fly)
		/// 
		/// @overload
		/// ## Parameters
		/// * in1: first input cv::Mat for binary computation
		/// * in2: second input cv::Mat for binary computation
		/// * out: output cv::Scalar for binary computation
		/// * args: compilation arguments for underlying compilation
		/// process.
		/// 
		/// ## Note
		/// This alternative version of [apply] function uses the following default values for its arguments:
		/// * args: {}
		#[inline]
		fn apply_def_5(&mut self, mut in1: core::Mat, mut in2: core::Mat, out: &mut core::Scalar) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_apply_Mat_Mat_ScalarR(self.as_raw_mut_GComputation(), in1.as_raw_mut_Mat(), in2.as_raw_mut_Mat(), out, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Execute a computation with arbitrary number of
		///  inputs/outputs (with compilation on-the-fly).
		/// 
		///  @private -- Exclude this function from OpenCV documentation
		/// 
		/// ## Overloaded parameters
		/// 
		/// ## Parameters
		/// * ins: vector of input cv::Mat objects to process by the
		///  computation.
		/// * outs: vector of output cv::Mat objects to produce by the
		///  computation.
		/// * args: compilation arguments for underlying compilation
		///  process.
		/// 
		///  Numbers of elements in ins/outs vectors must match numbers of
		///  inputs/outputs which were used to define this GComputation.
		/// 
		/// ## C++ default parameters
		/// * args: {}
		#[inline]
		fn apply_6(&mut self, ins: &core::Vector<core::Mat>, outs: &mut core::Vector<core::Mat>, mut args: crate::gapi::GCompileArgs) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_apply_const_vectorLMatGR_vectorLMatGR_GCompileArgsRR(self.as_raw_mut_GComputation(), ins.as_raw_VectorOfMat(), outs.as_raw_mut_VectorOfMat(), args.as_raw_mut_VectorOfGCompileArg(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Execute a computation with arbitrary number of
		/// inputs/outputs (with compilation on-the-fly).
		/// 
		/// @overload
		/// ## Parameters
		/// * ins: vector of input cv::Mat objects to process by the
		/// computation.
		/// * outs: vector of output cv::Mat objects to produce by the
		/// computation.
		/// * args: compilation arguments for underlying compilation
		/// process.
		/// 
		/// Numbers of elements in ins/outs vectors must match numbers of
		/// inputs/outputs which were used to define this GComputation.
		/// 
		/// ## Note
		/// This alternative version of [apply] function uses the following default values for its arguments:
		/// * args: {}
		#[inline]
		fn apply_def_6(&mut self, ins: &core::Vector<core::Mat>, outs: &mut core::Vector<core::Mat>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_apply_const_vectorLMatGR_vectorLMatGR(self.as_raw_mut_GComputation(), ins.as_raw_VectorOfMat(), outs.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Compile the computation for streaming mode.
		/// 
		/// This method triggers compilation process and produces a new
		/// GStreamingCompiled object which then can process video stream
		/// data in any format. Underlying mechanisms will be adjusted to
		/// every new input video stream automatically, but please note that
		/// _not all_ existing backends support this (see reshape()).
		/// 
		/// ## Parameters
		/// * args: compilation arguments for this compilation
		/// process. Compilation arguments directly affect what kind of
		/// executable object would be produced, e.g. which kernels (and
		/// thus, devices) would be used to execute computation.
		/// 
		/// ## Returns
		/// GStreamingCompiled, a streaming-oriented executable
		/// computation compiled for any input image format.
		/// ## See also
		/// [gapi_compile_args]
		/// 
		/// ## C++ default parameters
		/// * args: {}
		#[inline]
		fn compile_streaming(&mut self, mut args: crate::gapi::GCompileArgs) -> Result<crate::gapi::GStreamingCompiled> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_compileStreaming_GCompileArgsRR(self.as_raw_mut_GComputation(), args.as_raw_mut_VectorOfGCompileArg(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GStreamingCompiled::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Compile the computation for streaming mode.
		/// 
		/// This method triggers compilation process and produces a new
		/// GStreamingCompiled object which then can process video stream
		/// data in any format. Underlying mechanisms will be adjusted to
		/// every new input video stream automatically, but please note that
		/// _not all_ existing backends support this (see reshape()).
		/// 
		/// ## Parameters
		/// * args: compilation arguments for this compilation
		/// process. Compilation arguments directly affect what kind of
		/// executable object would be produced, e.g. which kernels (and
		/// thus, devices) would be used to execute computation.
		/// 
		/// ## Returns
		/// GStreamingCompiled, a streaming-oriented executable
		/// computation compiled for any input image format.
		/// ## See also
		/// [gapi_compile_args]
		/// 
		/// ## Note
		/// This alternative version of [compile_streaming] function uses the following default values for its arguments:
		/// * args: {}
		#[inline]
		fn compile_streaming_def(&mut self) -> Result<crate::gapi::GStreamingCompiled> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_compileStreaming(self.as_raw_mut_GComputation(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GStreamingCompiled::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// @private -- Exclude this function from OpenCV documentation
		/// 
		/// ## C++ default parameters
		/// * args: {}
		#[inline]
		fn compile_streaming_1(&mut self, callback: &crate::gapi::Detail_ExtractMetaCallback, mut args: crate::gapi::GCompileArgs) -> Result<crate::gapi::GStreamingCompiled> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_compileStreaming_const_ExtractMetaCallbackR_GCompileArgsRR(self.as_raw_mut_GComputation(), callback.as_raw_Detail_ExtractMetaCallback(), args.as_raw_mut_VectorOfGCompileArg(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GStreamingCompiled::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// @private -- Exclude this function from OpenCV documentation
		/// 
		/// ## Note
		/// This alternative version of [compile_streaming] function uses the following default values for its arguments:
		/// * args: {}
		#[inline]
		fn compile_streaming_def_1(&mut self, callback: &crate::gapi::Detail_ExtractMetaCallback) -> Result<crate::gapi::GStreamingCompiled> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_compileStreaming_const_ExtractMetaCallbackR(self.as_raw_mut_GComputation(), callback.as_raw_Detail_ExtractMetaCallback(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GStreamingCompiled::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// \addtogroup gapi_main_classes
	/// 
	///  G-API classes for constructed and compiled graphs.
	/// /
	/// 
	///  GComputation class represents a captured computation
	///  graph. GComputation objects form boundaries for expression code
	///  user writes with G-API, allowing to compile and execute it.
	/// 
	///  G-API computations are defined with input/output data
	///  objects. G-API will track automatically which operations connect
	///  specified outputs to the inputs, forming up a call graph to be
	///  executed. The below example expresses calculation of Sobel operator
	///  for edge detection (![inline formula](https://latex.codecogs.com/png.latex?G%20%3D%20%5Csqrt%7BG%5Fx%5E2%20%2B%20G%5Fy%5E2%7D)):
	/// 
	///  [graph_def](https://github.com/opencv/opencv_contrib/blob/4.8.1/modules/hdf/samples/cpp/tutorial_code/gapi/doc_snippets/api_ref_snippets.cpp#L1)
	/// 
	///  Full pipeline can be now captured with this object declaration:
	/// 
	///  [graph_cap_full](https://github.com/opencv/opencv_contrib/blob/4.8.1/modules/hdf/samples/cpp/tutorial_code/gapi/doc_snippets/api_ref_snippets.cpp#L1)
	/// 
	///  Input/output data objects on which a call graph should be
	///  reconstructed are passed using special wrappers cv::GIn and
	///  cv::GOut. G-API will track automatically which operations form a
	///  path from inputs to outputs and build the execution graph appropriately.
	/// 
	///  Note that cv::GComputation doesn't take ownership on data objects
	///  it is defined. Moreover, multiple GComputation objects may be
	///  defined on the same expressions, e.g. a smaller pipeline which
	///  expects that image gradients are already pre-calculated may be
	///  defined like this:
	/// 
	///  [graph_cap_sub](https://github.com/opencv/opencv_contrib/blob/4.8.1/modules/hdf/samples/cpp/tutorial_code/gapi/doc_snippets/api_ref_snippets.cpp#L1)
	/// 
	///  The resulting graph would expect two inputs and produce one
	///  output. In this case, it doesn't matter if gx/gy data objects are
	///  results of cv::gapi::Sobel operators -- G-API will stop unrolling
	///  expressions and building the underlying graph one reaching this
	///  data objects.
	/// 
	///  The way how GComputation is defined is important as its definition
	///  specifies graph _protocol_ -- the way how the graph should be
	///  used. Protocol is defined by number of inputs, number of outputs,
	///  and shapes of inputs and outputs.
	/// 
	///  In the above example, sobelEdge expects one Mat on input and
	///  produces one Mat; while sobelEdgeSub expects two Mats on input and
	///  produces one Mat. GComputation's protocol defines how other
	///  computation methods should be used -- cv::GComputation::compile() and
	///  cv::GComputation::apply(). For example, if a graph is defined on
	///  two GMat inputs, two cv::Mat objects have to be passed to apply()
	///  for execution. GComputation checks protocol correctness in runtime
	///  so passing a different number of objects in apply() or passing
	///  cv::Scalar instead of cv::Mat there would compile well as a C++
	///  source but raise an exception in run-time. G-API also comes with a
	///  typed wrapper cv::GComputationT<> which introduces this type-checking in
	///  compile-time.
	/// 
	///  cv::GComputation itself is a thin object which just captures what
	///  the graph is. The compiled graph (which actually process data) is
	///  represented by class GCompiled. Use compile() method to generate a
	///  compiled graph with given compile options. cv::GComputation can
	///  also be used to process data with implicit graph compilation
	///  on-the-fly, see apply() for details.
	/// 
	///  GComputation is a reference-counted object -- once defined, all its
	///  copies will refer to the same instance.
	/// ## See also
	/// GCompiled
	pub struct GComputation {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GComputation }
	
	impl Drop for GComputation {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GComputation_delete(self.as_raw_mut_GComputation()) };
		}
	}
	
	unsafe impl Send for GComputation {}
	
	impl crate::gapi::GComputationTraitConst for GComputation {
		#[inline] fn as_raw_GComputation(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GComputationTrait for GComputation {
		#[inline] fn as_raw_mut_GComputation(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GComputation {
		/// Defines an unary (one input -- one output) computation
		/// 
		///  Generic GComputation constructor.
		/// 
		/// Constructs a new graph with a given protocol, specified as a
		/// flow of operations connecting input/output objects. Throws if
		/// the passed boundaries are invalid, e.g. if there's no
		/// functional dependency (path) between given outputs and inputs.
		/// 
		/// ## Parameters
		/// * ins: Input data vector.
		/// * outs: Output data vector.
		/// 
		/// 
		/// Note: Don't construct GProtoInputArgs/GProtoOutputArgs objects
		/// directly, use cv::GIn()/cv::GOut() wrapper functions instead.
		/// ## See also
		/// [gapi_data_objects]
		/// 
		/// ## Overloaded parameters
		/// 
		/// * in: input GMat of the defined unary computation
		/// * out: output GMat of the defined unary computation
		#[inline]
		pub fn new(mut in_: crate::gapi::GMat, mut out: crate::gapi::GMat) -> Result<crate::gapi::GComputation> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_GComputation_GMat_GMat(in_.as_raw_mut_GMat(), out.as_raw_mut_GMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GComputation::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Defines an unary (one input -- one output) computation
		/// 
		///  Generic GComputation constructor.
		/// 
		/// Constructs a new graph with a given protocol, specified as a
		/// flow of operations connecting input/output objects. Throws if
		/// the passed boundaries are invalid, e.g. if there's no
		/// functional dependency (path) between given outputs and inputs.
		/// 
		/// ## Parameters
		/// * ins: Input data vector.
		/// * outs: Output data vector.
		/// 
		/// 
		/// Note: Don't construct GProtoInputArgs/GProtoOutputArgs objects
		/// directly, use cv::GIn()/cv::GOut() wrapper functions instead.
		/// ## See also
		/// [gapi_data_objects]
		/// 
		/// ## Overloaded parameters
		/// 
		/// * in: input GMat of the defined unary computation
		/// * out: output GScalar of the defined unary computation
		#[inline]
		pub fn new_1(mut in_: crate::gapi::GMat, mut out: crate::gapi::GScalar) -> Result<crate::gapi::GComputation> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_GComputation_GMat_GScalar(in_.as_raw_mut_GMat(), out.as_raw_mut_GScalar(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GComputation::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Defines a binary (two inputs -- one output) computation
		/// 
		///  Generic GComputation constructor.
		/// 
		/// Constructs a new graph with a given protocol, specified as a
		/// flow of operations connecting input/output objects. Throws if
		/// the passed boundaries are invalid, e.g. if there's no
		/// functional dependency (path) between given outputs and inputs.
		/// 
		/// ## Parameters
		/// * ins: Input data vector.
		/// * outs: Output data vector.
		/// 
		/// 
		/// Note: Don't construct GProtoInputArgs/GProtoOutputArgs objects
		/// directly, use cv::GIn()/cv::GOut() wrapper functions instead.
		/// ## See also
		/// [gapi_data_objects]
		/// 
		/// ## Overloaded parameters
		/// 
		/// * in1: first input GMat of the defined binary computation
		/// * in2: second input GMat of the defined binary computation
		/// * out: output GMat of the defined binary computation
		#[inline]
		pub fn new_2(mut in1: crate::gapi::GMat, mut in2: crate::gapi::GMat, mut out: crate::gapi::GMat) -> Result<crate::gapi::GComputation> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_GComputation_GMat_GMat_GMat(in1.as_raw_mut_GMat(), in2.as_raw_mut_GMat(), out.as_raw_mut_GMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GComputation::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Defines a binary (two inputs -- one output) computation
		/// 
		///  Generic GComputation constructor.
		/// 
		/// Constructs a new graph with a given protocol, specified as a
		/// flow of operations connecting input/output objects. Throws if
		/// the passed boundaries are invalid, e.g. if there's no
		/// functional dependency (path) between given outputs and inputs.
		/// 
		/// ## Parameters
		/// * ins: Input data vector.
		/// * outs: Output data vector.
		/// 
		/// 
		/// Note: Don't construct GProtoInputArgs/GProtoOutputArgs objects
		/// directly, use cv::GIn()/cv::GOut() wrapper functions instead.
		/// ## See also
		/// [gapi_data_objects]
		/// 
		/// ## Overloaded parameters
		/// 
		/// * in1: first input GMat of the defined binary computation
		/// * in2: second input GMat of the defined binary computation
		/// * out: output GScalar of the defined binary computation
		#[inline]
		pub fn new_3(mut in1: crate::gapi::GMat, mut in2: crate::gapi::GMat, mut out: crate::gapi::GScalar) -> Result<crate::gapi::GComputation> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_GComputation_GMat_GMat_GScalar(in1.as_raw_mut_GMat(), in2.as_raw_mut_GMat(), out.as_raw_mut_GScalar(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GComputation::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Defines a computation with arbitrary input/output number.
		/// 
		///  Generic GComputation constructor.
		/// 
		/// Constructs a new graph with a given protocol, specified as a
		/// flow of operations connecting input/output objects. Throws if
		/// the passed boundaries are invalid, e.g. if there's no
		/// functional dependency (path) between given outputs and inputs.
		/// 
		/// ## Parameters
		/// * ins: Input data vector.
		/// * outs: Output data vector.
		/// 
		/// 
		/// Note: Don't construct GProtoInputArgs/GProtoOutputArgs objects
		/// directly, use cv::GIn()/cv::GOut() wrapper functions instead.
		/// ## See also
		/// [gapi_data_objects]
		/// 
		/// ## Overloaded parameters
		/// 
		/// * ins: vector of inputs GMats for this computation
		/// * outs: vector of outputs GMats for this computation
		/// 
		///  Use this overload for cases when number of computation
		///  inputs/outputs is not known in compile-time -- e.g. when graph
		///  is programmatically generated to build an image pyramid with
		///  the given number of levels, etc.
		#[inline]
		pub fn new_4(ins: &core::Vector<crate::gapi::GMat>, outs: &core::Vector<crate::gapi::GMat>) -> Result<crate::gapi::GComputation> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GComputation_GComputation_const_vectorLGMatGR_const_vectorLGMatGR(ins.as_raw_VectorOfGMat(), outs.as_raw_VectorOfGMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GComputation::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for GComputation {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GComputation")
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::GFrame]
	pub trait GFrameTraitConst {
		fn as_raw_GFrame(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::gapi::GFrame]
	pub trait GFrameTrait: crate::gapi::GFrameTraitConst {
		fn as_raw_mut_GFrame(&mut self) -> *mut c_void;
	
	}
	
	/// \addtogroup gapi_data_objects
	/// /
	/// 
	///  GFrame class represents an image or media frame in the graph.
	/// 
	///  GFrame doesn't store any data itself, instead it describes a
	///  functional relationship between operations consuming and producing
	///  GFrame objects.
	/// 
	///  GFrame is introduced to handle various media formats (e.g., NV12 or
	///  I420) under the same type. Various image formats may differ in the
	///  number of planes (e.g. two for NV12, three for I420) and the pixel
	///  layout inside. GFrame type allows to handle these media formats in
	///  the graph uniformly -- the graph structure will not change if the
	///  media format changes, e.g. a different camera or decoder is used
	///  with the same graph. G-API provides a number of operations which
	///  operate directly on GFrame, like `infer<>()` or
	///  renderFrame(); these operations are expected to handle different
	///  media formats inside. There is also a number of accessor
	///  operations like BGR(), Y(), UV() -- these operations provide
	///  access to frame's data in the familiar cv::GMat form, which can be
	///  used with the majority of the existing G-API operations. These
	///  accessor functions may perform color space conversion on the fly if
	///  the image format of the GFrame they are applied to differs from the
	///  operation's semantic (e.g. the BGR() accessor is called on an NV12
	///  image frame).
	/// 
	///  GFrame is a virtual counterpart of cv::MediaFrame.
	/// ## See also
	/// cv::MediaFrame, cv::GFrameDesc, BGR(), Y(), UV(), infer<>().
	pub struct GFrame {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GFrame }
	
	impl Drop for GFrame {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GFrame_delete(self.as_raw_mut_GFrame()) };
		}
	}
	
	unsafe impl Send for GFrame {}
	
	impl crate::gapi::GFrameTraitConst for GFrame {
		#[inline] fn as_raw_GFrame(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GFrameTrait for GFrame {
		#[inline] fn as_raw_mut_GFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GFrame {
		/// Constructs an empty GFrame
		/// 
		/// Normally, empty G-API data objects denote a starting point of
		/// the graph. When an empty GFrame is assigned to a result of some
		/// operation, it obtains a functional link to this operation (and
		/// is not empty anymore).
		#[inline]
		pub fn default() -> Result<crate::gapi::GFrame> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFrame_GFrame(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GFrame::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl Clone for GFrame {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_GFrame_implicitClone_const(self.as_raw_GFrame())) }
		}
	}
	
	impl std::fmt::Debug for GFrame {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GFrame")
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::GFrameDesc]
	pub trait GFrameDescTraitConst {
		fn as_raw_GFrameDesc(&self) -> *const c_void;
	
		#[inline]
		fn fmt(&self) -> crate::gapi::MediaFormat {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFrameDesc_propFmt_const(self.as_raw_GFrameDesc(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		#[inline]
		fn size(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFrameDesc_propSize_const(self.as_raw_GFrameDesc(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		#[inline]
		fn equals(&self, unnamed: &crate::gapi::GFrameDesc) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GFrameDesc_operatorEQ_const_const_GFrameDescR(self.as_raw_GFrameDesc(), unnamed.as_raw_GFrameDesc(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::gapi::GFrameDesc]
	pub trait GFrameDescTrait: crate::gapi::GFrameDescTraitConst {
		fn as_raw_mut_GFrameDesc(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_fmt(&mut self, val: crate::gapi::MediaFormat) {
			let ret = unsafe { sys::cv_GFrameDesc_propFmt_MediaFormat(self.as_raw_mut_GFrameDesc(), val) };
			ret
		}
		
		#[inline]
		fn set_size(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_GFrameDesc_propSize_Size(self.as_raw_mut_GFrameDesc(), val.opencv_as_extern()) };
			ret
		}
		
	}
	
	/// \addtogroup gapi_meta_args
	pub struct GFrameDesc {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GFrameDesc }
	
	impl Drop for GFrameDesc {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GFrameDesc_delete(self.as_raw_mut_GFrameDesc()) };
		}
	}
	
	unsafe impl Send for GFrameDesc {}
	
	impl crate::gapi::GFrameDescTraitConst for GFrameDesc {
		#[inline] fn as_raw_GFrameDesc(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GFrameDescTrait for GFrameDesc {
		#[inline] fn as_raw_mut_GFrameDesc(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GFrameDesc {
	}
	
	impl std::fmt::Debug for GFrameDesc {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GFrameDesc")
				.field("fmt", &crate::gapi::GFrameDescTraitConst::fmt(self))
				.field("size", &crate::gapi::GFrameDescTraitConst::size(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::GKernel]
	pub trait GKernelTraitConst {
		fn as_raw_GKernel(&self) -> *const c_void;
	
		#[inline]
		fn name(&self) -> String {
			let ret = unsafe { sys::cv_GKernel_propName_const(self.as_raw_GKernel()) };
			let ret = unsafe { String::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn tag(&self) -> String {
			let ret = unsafe { sys::cv_GKernel_propTag_const(self.as_raw_GKernel()) };
			let ret = unsafe { String::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn out_shapes(&self) -> core::Vector<crate::gapi::GShape> {
			let ret = unsafe { sys::cv_GKernel_propOutShapes_const(self.as_raw_GKernel()) };
			let ret = unsafe { core::Vector::<crate::gapi::GShape>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn in_kinds(&self) -> core::Vector<crate::gapi::Detail_OpaqueKind> {
			let ret = unsafe { sys::cv_GKernel_propInKinds_const(self.as_raw_GKernel()) };
			let ret = unsafe { core::Vector::<crate::gapi::Detail_OpaqueKind>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn out_kinds(&self) -> core::Vector<crate::gapi::Detail_OpaqueKind> {
			let ret = unsafe { sys::cv_GKernel_propOutKinds_const(self.as_raw_GKernel()) };
			let ret = unsafe { core::Vector::<crate::gapi::Detail_OpaqueKind>::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::gapi::GKernel]
	pub trait GKernelTrait: crate::gapi::GKernelTraitConst {
		fn as_raw_mut_GKernel(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_name(&mut self, val: &str) {
			extern_container_arg!(nofail mut val);
			let ret = unsafe { sys::cv_GKernel_propName_string(self.as_raw_mut_GKernel(), val.opencv_as_extern_mut()) };
			ret
		}
		
		#[inline]
		fn set_tag(&mut self, val: &str) {
			extern_container_arg!(nofail mut val);
			let ret = unsafe { sys::cv_GKernel_propTag_string(self.as_raw_mut_GKernel(), val.opencv_as_extern_mut()) };
			ret
		}
		
		#[inline]
		fn set_out_shapes(&mut self, mut val: crate::gapi::GShapes) {
			let ret = unsafe { sys::cv_GKernel_propOutShapes_GShapes(self.as_raw_mut_GKernel(), val.as_raw_mut_VectorOfGShape()) };
			ret
		}
		
		#[inline]
		fn set_in_kinds(&mut self, mut val: crate::gapi::GKinds) {
			let ret = unsafe { sys::cv_GKernel_propInKinds_GKinds(self.as_raw_mut_GKernel(), val.as_raw_mut_VectorOfDetail_OpaqueKind()) };
			ret
		}
		
		#[inline]
		fn set_out_kinds(&mut self, mut val: crate::gapi::GKinds) {
			let ret = unsafe { sys::cv_GKernel_propOutKinds_GKinds(self.as_raw_mut_GKernel(), val.as_raw_mut_VectorOfDetail_OpaqueKind()) };
			ret
		}
		
	}
	
	pub struct GKernel {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GKernel }
	
	impl Drop for GKernel {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GKernel_delete(self.as_raw_mut_GKernel()) };
		}
	}
	
	unsafe impl Send for GKernel {}
	
	impl crate::gapi::GKernelTraitConst for GKernel {
		#[inline] fn as_raw_GKernel(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GKernelTrait for GKernel {
		#[inline] fn as_raw_mut_GKernel(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GKernel {
	}
	
	impl std::fmt::Debug for GKernel {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GKernel")
				.field("name", &crate::gapi::GKernelTraitConst::name(self))
				.field("tag", &crate::gapi::GKernelTraitConst::tag(self))
				.field("out_shapes", &crate::gapi::GKernelTraitConst::out_shapes(self))
				.field("in_kinds", &crate::gapi::GKernelTraitConst::in_kinds(self))
				.field("out_kinds", &crate::gapi::GKernelTraitConst::out_kinds(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::GKernelImpl]
	pub trait GKernelImplTraitConst {
		fn as_raw_GKernelImpl(&self) -> *const c_void;
	
		#[inline]
		fn opaque(&self) -> crate::gapi::any {
			let ret = unsafe { sys::cv_GKernelImpl_propOpaque_const(self.as_raw_GKernelImpl()) };
			let ret = unsafe { crate::gapi::any::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::gapi::GKernelImpl]
	pub trait GKernelImplTrait: crate::gapi::GKernelImplTraitConst {
		fn as_raw_mut_GKernelImpl(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_opaque(&mut self, mut val: crate::gapi::any) {
			let ret = unsafe { sys::cv_GKernelImpl_propOpaque_any(self.as_raw_mut_GKernelImpl(), val.as_raw_mut_any()) };
			ret
		}
		
	}
	
	pub struct GKernelImpl {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GKernelImpl }
	
	impl Drop for GKernelImpl {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GKernelImpl_delete(self.as_raw_mut_GKernelImpl()) };
		}
	}
	
	unsafe impl Send for GKernelImpl {}
	
	impl crate::gapi::GKernelImplTraitConst for GKernelImpl {
		#[inline] fn as_raw_GKernelImpl(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GKernelImplTrait for GKernelImpl {
		#[inline] fn as_raw_mut_GKernelImpl(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GKernelImpl {
	}
	
	impl std::fmt::Debug for GKernelImpl {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GKernelImpl")
				.field("opaque", &crate::gapi::GKernelImplTraitConst::opaque(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::GKernelPackage]
	pub trait GKernelPackageTraitConst {
		fn as_raw_GKernelPackage(&self) -> *const c_void;
	
		/// Returns vector of transformations included in the package
		/// 
		/// ## Returns
		/// vector of transformations included in the package
		#[inline]
		fn get_transformations(&self) -> Result<core::Vector<crate::gapi::GTransform>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GKernelPackage_get_transformations_const(self.as_raw_GKernelPackage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<crate::gapi::GTransform>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Returns vector of kernel ids included in the package
		/// 
		/// ## Returns
		/// vector of kernel ids included in the package
		#[inline]
		fn get_kernel_ids(&self) -> Result<core::Vector<String>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GKernelPackage_get_kernel_ids_const(self.as_raw_GKernelPackage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<String>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// @private
		#[inline]
		fn includes_api(&self, id: &str) -> Result<bool> {
			extern_container_arg!(id);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GKernelPackage_includesAPI_const_const_stringR(self.as_raw_GKernelPackage(), id.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// @private
		#[inline]
		fn lookup(&self, id: &str) -> Result<core::Tuple<(crate::gapi::GBackend, crate::gapi::GKernelImpl)>> {
			extern_container_arg!(id);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GKernelPackage_lookup_const_const_stringR(self.as_raw_GKernelPackage(), id.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Tuple::<(crate::gapi::GBackend, crate::gapi::GKernelImpl)>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Lists all backends which are included into package
		/// 
		/// ## Returns
		/// vector of backends
		#[inline]
		fn backends(&self) -> Result<core::Vector<crate::gapi::GBackend>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GKernelPackage_backends_const(self.as_raw_GKernelPackage(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<crate::gapi::GBackend>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::gapi::GKernelPackage]
	pub trait GKernelPackageTrait: crate::gapi::GKernelPackageTraitConst {
		fn as_raw_mut_GKernelPackage(&mut self) -> *mut c_void;
	
		#[inline]
		fn include(&mut self, functor: &crate::gapi::GFunctor) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GKernelPackage_include_const_GFunctorR(self.as_raw_mut_GKernelPackage(), functor.as_raw_GFunctor(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Remove all kernels associated with the given backend
		/// from the package.
		/// 
		/// Does nothing if there's no kernels of this backend in the package.
		/// 
		/// ## Parameters
		/// * backend: backend which kernels to remove
		#[inline]
		fn remove(&mut self, backend: &crate::gapi::GBackend) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GKernelPackage_remove_const_GBackendR(self.as_raw_mut_GKernelPackage(), backend.as_raw_GBackend(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Adds a new kernel based on it's backend and id into the kernel package
		/// 
		/// ## Parameters
		/// * backend: backend associated with the kernel
		/// * kernel_id: a name/id of the kernel
		#[inline]
		fn include_1(&mut self, backend: &crate::gapi::GBackend, kernel_id: &str) -> Result<()> {
			extern_container_arg!(kernel_id);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GKernelPackage_include_const_GBackendR_const_stringR(self.as_raw_mut_GKernelPackage(), backend.as_raw_GBackend(), kernel_id.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// A container class for heterogeneous kernel
	/// implementation collections and graph transformations.
	/// 
	/// GKernelPackage is a special container class which stores kernel
	/// _implementations_ and graph _transformations_. Objects of this class
	/// are created and passed to cv::GComputation::compile() to specify
	/// which kernels to use and which transformations to apply in the
	/// compiled graph. GKernelPackage may contain kernels of
	/// different backends, e.g. be heterogeneous.
	/// 
	/// The most easy way to create a kernel package is to use function
	/// cv::gapi::kernels(). This template functions takes kernel
	/// implementations in form of type list (variadic template) and
	/// generates a kernel package atop of that.
	/// 
	/// Kernel packages can be also generated programmatically, starting
	/// with an empty package (created with the default constructor)
	/// and then by populating it with kernels via call to
	/// GKernelPackage::include(). Note this method is also a template
	/// one since G-API kernel and transformation implementations are _types_,
	/// not objects.
	/// 
	/// Finally, two kernel packages can be combined into a new one
	/// with function cv::gapi::combine().
	pub struct GKernelPackage {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GKernelPackage }
	
	impl Drop for GKernelPackage {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GKernelPackage_delete(self.as_raw_mut_GKernelPackage()) };
		}
	}
	
	unsafe impl Send for GKernelPackage {}
	
	impl crate::gapi::GKernelPackageTraitConst for GKernelPackage {
		#[inline] fn as_raw_GKernelPackage(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GKernelPackageTrait for GKernelPackage {
		#[inline] fn as_raw_mut_GKernelPackage(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GKernelPackage {
		fn default() -> Self {
			unsafe { Self::from_raw(sys::cv_GKernelPackage_defaultNew_const()) }
		}
		
	}
	
	impl Clone for GKernelPackage {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_GKernelPackage_implicitClone_const(self.as_raw_GKernelPackage())) }
		}
	}
	
	impl std::fmt::Debug for GKernelPackage {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GKernelPackage")
				.finish()
		}
	}
	
	impl Default for GKernelPackage {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}
	
	/// Constant methods for [crate::gapi::GMat]
	pub trait GMatTraitConst {
		fn as_raw_GMat(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::gapi::GMat]
	pub trait GMatTrait: crate::gapi::GMatTraitConst {
		fn as_raw_mut_GMat(&mut self) -> *mut c_void;
	
	}
	
	/// \addtogroup gapi_data_objects
	/// 
	///  G-API data objects used to build G-API expressions.
	/// 
	///  These objects do not own any particular data (except compile-time
	///  associated values like with cv::GScalar or `cv::GArray<T>`) and are
	///  used only to construct graphs.
	/// 
	///  Every graph in G-API starts and ends with data objects.
	/// 
	///  Once constructed and compiled, G-API operates with regular host-side
	///  data instead. Refer to the below table to find the mapping between
	///  G-API and regular data types when passing input and output data
	///  structures to G-API:
	/// 
	///    G-API data type    | I/O data type
	///    ------------------ | -------------
	///    cv::GMat           | cv::Mat, cv::UMat, cv::RMat
	///    cv::GScalar        | cv::Scalar
	///    `cv::GArray<T>`    | std::vector<T>
	///    `cv::GOpaque<T>`   | T
	///    cv::GFrame         | cv::MediaFrame
	/// /
	/// 
	///  GMat class represents image or tensor data in the
	///  graph.
	/// 
	///  GMat doesn't store any data itself, instead it describes a
	///  functional relationship between operations consuming and producing
	///  GMat objects.
	/// 
	///  GMat is a virtual counterpart of Mat and UMat, but it
	///  doesn't mean G-API use Mat or UMat objects internally to represent
	///  GMat objects -- the internal data representation may be
	///  backend-specific or optimized out at all.
	/// ## See also
	/// Mat, GMatDesc
	pub struct GMat {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GMat }
	
	impl Drop for GMat {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GMat_delete(self.as_raw_mut_GMat()) };
		}
	}
	
	unsafe impl Send for GMat {}
	
	impl crate::gapi::GMatTraitConst for GMat {
		#[inline] fn as_raw_GMat(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GMatTrait for GMat {
		#[inline] fn as_raw_mut_GMat(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GMat {
		/// Constructs an empty GMat
		/// 
		/// Normally, empty G-API data objects denote a starting point of
		/// the graph. When an empty GMat is assigned to a result of some
		/// operation, it obtains a functional link to this operation (and
		/// is not empty anymore).
		#[inline]
		pub fn default() -> Result<crate::gapi::GMat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GMat_GMat(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl Clone for GMat {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_GMat_implicitClone_const(self.as_raw_GMat())) }
		}
	}
	
	impl std::fmt::Debug for GMat {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GMat")
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::GMatDesc]
	pub trait GMatDescTraitConst {
		fn as_raw_GMatDesc(&self) -> *const c_void;
	
		#[inline]
		fn depth(&self) -> i32 {
			let ret = unsafe { sys::cv_GMatDesc_propDepth_const(self.as_raw_GMatDesc()) };
			ret
		}
		
		#[inline]
		fn chan(&self) -> i32 {
			let ret = unsafe { sys::cv_GMatDesc_propChan_const(self.as_raw_GMatDesc()) };
			ret
		}
		
		#[inline]
		fn size(&self) -> core::Size {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GMatDesc_propSize_const(self.as_raw_GMatDesc(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		#[inline]
		fn planar(&self) -> bool {
			let ret = unsafe { sys::cv_GMatDesc_propPlanar_const(self.as_raw_GMatDesc()) };
			ret
		}
		
		#[inline]
		fn dims(&self) -> core::Vector<i32> {
			let ret = unsafe { sys::cv_GMatDesc_propDims_const(self.as_raw_GMatDesc()) };
			let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn equals(&self, rhs: &crate::gapi::GMatDesc) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GMatDesc_operatorEQ_const_const_GMatDescR(self.as_raw_GMatDesc(), rhs.as_raw_GMatDesc(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn not_equals(&self, rhs: &crate::gapi::GMatDesc) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GMatDesc_operatorNE_const_const_GMatDescR(self.as_raw_GMatDesc(), rhs.as_raw_GMatDesc(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn is_nd(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GMatDesc_isND_const(self.as_raw_GMatDesc(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn can_describe(&self, mat: &core::Mat) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GMatDesc_canDescribe_const_const_MatR(self.as_raw_GMatDesc(), mat.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn can_describe_1(&self, mat: &crate::gapi::RMat) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GMatDesc_canDescribe_const_const_RMatR(self.as_raw_GMatDesc(), mat.as_raw_RMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn with_size_delta(&self, delta: core::Size) -> Result<crate::gapi::GMatDesc> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GMatDesc_withSizeDelta_const_Size(self.as_raw_GMatDesc(), delta.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn with_size_delta_1(&self, dx: i32, dy: i32) -> Result<crate::gapi::GMatDesc> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GMatDesc_withSizeDelta_const_int_int(self.as_raw_GMatDesc(), dx, dy, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn with_size(&self, sz: core::Size) -> Result<crate::gapi::GMatDesc> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GMatDesc_withSize_const_Size(self.as_raw_GMatDesc(), sz.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn with_depth(&self, ddepth: i32) -> Result<crate::gapi::GMatDesc> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GMatDesc_withDepth_const_int(self.as_raw_GMatDesc(), ddepth, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn with_type(&self, ddepth: i32, dchan: i32) -> Result<crate::gapi::GMatDesc> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GMatDesc_withType_const_int_int(self.as_raw_GMatDesc(), ddepth, dchan, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn as_planar(&self) -> Result<crate::gapi::GMatDesc> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GMatDesc_asPlanar_const(self.as_raw_GMatDesc(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn as_planar_1(&self, planes: i32) -> Result<crate::gapi::GMatDesc> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GMatDesc_asPlanar_const_int(self.as_raw_GMatDesc(), planes, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn as_interleaved(&self) -> Result<crate::gapi::GMatDesc> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GMatDesc_asInterleaved_const(self.as_raw_GMatDesc(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::gapi::GMatDesc]
	pub trait GMatDescTrait: crate::gapi::GMatDescTraitConst {
		fn as_raw_mut_GMatDesc(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_depth(&mut self, val: i32) {
			let ret = unsafe { sys::cv_GMatDesc_propDepth_int(self.as_raw_mut_GMatDesc(), val) };
			ret
		}
		
		#[inline]
		fn set_chan(&mut self, val: i32) {
			let ret = unsafe { sys::cv_GMatDesc_propChan_int(self.as_raw_mut_GMatDesc(), val) };
			ret
		}
		
		#[inline]
		fn set_size(&mut self, val: core::Size) {
			let ret = unsafe { sys::cv_GMatDesc_propSize_Size(self.as_raw_mut_GMatDesc(), val.opencv_as_extern()) };
			ret
		}
		
		#[inline]
		fn set_planar(&mut self, val: bool) {
			let ret = unsafe { sys::cv_GMatDesc_propPlanar_bool(self.as_raw_mut_GMatDesc(), val) };
			ret
		}
		
		#[inline]
		fn set_dims(&mut self, mut val: core::Vector<i32>) {
			let ret = unsafe { sys::cv_GMatDesc_propDims_vectorLintG(self.as_raw_mut_GMatDesc(), val.as_raw_mut_VectorOfi32()) };
			ret
		}
		
	}
	
	/// \addtogroup gapi_meta_args
	pub struct GMatDesc {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GMatDesc }
	
	impl Drop for GMatDesc {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GMatDesc_delete(self.as_raw_mut_GMatDesc()) };
		}
	}
	
	unsafe impl Send for GMatDesc {}
	
	impl crate::gapi::GMatDescTraitConst for GMatDesc {
		#[inline] fn as_raw_GMatDesc(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GMatDescTrait for GMatDesc {
		#[inline] fn as_raw_mut_GMatDesc(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GMatDesc {
		/// ## C++ default parameters
		/// * p: false
		#[inline]
		pub fn new(d: i32, c: i32, s: core::Size, p: bool) -> Result<crate::gapi::GMatDesc> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GMatDesc_GMatDesc_int_int_Size_bool(d, c, s.opencv_as_extern(), p, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * p: false
		#[inline]
		pub fn new_def(d: i32, c: i32, s: core::Size) -> Result<crate::gapi::GMatDesc> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GMatDesc_GMatDesc_int_int_Size(d, c, s.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn new_1(d: i32, dd: &core::Vector<i32>) -> Result<crate::gapi::GMatDesc> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GMatDesc_GMatDesc_int_const_vectorLintGR(d, dd.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn new_2(d: i32, mut dd: core::Vector<i32>) -> Result<crate::gapi::GMatDesc> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GMatDesc_GMatDesc_int_vectorLintGRR(d, dd.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn default() -> Result<crate::gapi::GMatDesc> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GMatDesc_GMatDesc(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl Clone for GMatDesc {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_GMatDesc_implicitClone_const(self.as_raw_GMatDesc())) }
		}
	}
	
	impl std::fmt::Debug for GMatDesc {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GMatDesc")
				.field("depth", &crate::gapi::GMatDescTraitConst::depth(self))
				.field("chan", &crate::gapi::GMatDescTraitConst::chan(self))
				.field("size", &crate::gapi::GMatDescTraitConst::size(self))
				.field("planar", &crate::gapi::GMatDescTraitConst::planar(self))
				.field("dims", &crate::gapi::GMatDescTraitConst::dims(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::GMatP]
	pub trait GMatPTraitConst: crate::gapi::GMatTraitConst {
		fn as_raw_GMatP(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::gapi::GMatP]
	pub trait GMatPTrait: crate::gapi::GMatPTraitConst + crate::gapi::GMatTrait {
		fn as_raw_mut_GMatP(&mut self) -> *mut c_void;
	
	}
	
	pub struct GMatP {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GMatP }
	
	impl Drop for GMatP {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GMatP_delete(self.as_raw_mut_GMatP()) };
		}
	}
	
	unsafe impl Send for GMatP {}
	
	impl crate::gapi::GMatTraitConst for GMatP {
		#[inline] fn as_raw_GMat(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GMatTrait for GMatP {
		#[inline] fn as_raw_mut_GMat(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::gapi::GMatPTraitConst for GMatP {
		#[inline] fn as_raw_GMatP(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GMatPTrait for GMatP {
		#[inline] fn as_raw_mut_GMatP(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GMatP {
	}
	
	boxed_cast_base! { GMatP, crate::gapi::GMat, cv_GMatP_to_GMat }
	
	impl std::fmt::Debug for GMatP {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GMatP")
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::GOpaqueDesc]
	pub trait GOpaqueDescTraitConst {
		fn as_raw_GOpaqueDesc(&self) -> *const c_void;
	
		#[inline]
		fn equals(&self, unnamed: &crate::gapi::GOpaqueDesc) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GOpaqueDesc_operatorEQ_const_const_GOpaqueDescR(self.as_raw_GOpaqueDesc(), unnamed.as_raw_GOpaqueDesc(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::gapi::GOpaqueDesc]
	pub trait GOpaqueDescTrait: crate::gapi::GOpaqueDescTraitConst {
		fn as_raw_mut_GOpaqueDesc(&mut self) -> *mut c_void;
	
	}
	
	/// \addtogroup gapi_meta_args
	pub struct GOpaqueDesc {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GOpaqueDesc }
	
	impl Drop for GOpaqueDesc {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GOpaqueDesc_delete(self.as_raw_mut_GOpaqueDesc()) };
		}
	}
	
	unsafe impl Send for GOpaqueDesc {}
	
	impl crate::gapi::GOpaqueDescTraitConst for GOpaqueDesc {
		#[inline] fn as_raw_GOpaqueDesc(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GOpaqueDescTrait for GOpaqueDesc {
		#[inline] fn as_raw_mut_GOpaqueDesc(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GOpaqueDesc {
		fn default() -> Self {
			unsafe { Self::from_raw(sys::cv_GOpaqueDesc_defaultNew_const()) }
		}
		
	}
	
	impl Clone for GOpaqueDesc {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_GOpaqueDesc_implicitClone_const(self.as_raw_GOpaqueDesc())) }
		}
	}
	
	impl std::fmt::Debug for GOpaqueDesc {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GOpaqueDesc")
				.finish()
		}
	}
	
	impl Default for GOpaqueDesc {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}
	
	/// Constant methods for [crate::gapi::GRunArg]
	pub trait GRunArgTraitConst {
		fn as_raw_GRunArg(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::gapi::GRunArg]
	pub trait GRunArgTrait: crate::gapi::GRunArgTraitConst {
		fn as_raw_mut_GRunArg(&mut self) -> *mut c_void;
	
	}
	
	pub struct GRunArg {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GRunArg }
	
	impl Drop for GRunArg {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GRunArg_delete(self.as_raw_mut_GRunArg()) };
		}
	}
	
	unsafe impl Send for GRunArg {}
	
	impl crate::gapi::GRunArgTraitConst for GRunArg {
		#[inline] fn as_raw_GRunArg(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GRunArgTrait for GRunArg {
		#[inline] fn as_raw_mut_GRunArg(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GRunArg {
		#[inline]
		pub fn default() -> Result<crate::gapi::GRunArg> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GRunArg_GRunArg(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GRunArg::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn copy(arg: &crate::gapi::GRunArg) -> Result<crate::gapi::GRunArg> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GRunArg_GRunArg_const_GRunArgR(arg.as_raw_GRunArg(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GRunArg::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn copy_mut(mut arg: crate::gapi::GRunArg) -> Result<crate::gapi::GRunArg> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GRunArg_GRunArg_GRunArgRR(arg.as_raw_mut_GRunArg(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GRunArg::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for GRunArg {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GRunArg")
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::GScalar]
	pub trait GScalarTraitConst {
		fn as_raw_GScalar(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::gapi::GScalar]
	pub trait GScalarTrait: crate::gapi::GScalarTraitConst {
		fn as_raw_mut_GScalar(&mut self) -> *mut c_void;
	
	}
	
	/// \addtogroup gapi_data_objects
	/// /
	/// 
	///  GScalar class represents cv::Scalar data in the graph.
	/// 
	///  GScalar may be associated with a cv::Scalar value, which becomes
	///  its constant value bound in graph compile time. cv::GScalar describes a
	///  functional relationship between operations consuming and producing
	///  GScalar objects.
	/// 
	///  GScalar is a virtual counterpart of cv::Scalar, which is usually used
	///  to represent the GScalar data in G-API during the execution.
	/// ## See also
	/// Scalar
	pub struct GScalar {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GScalar }
	
	impl Drop for GScalar {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GScalar_delete(self.as_raw_mut_GScalar()) };
		}
	}
	
	unsafe impl Send for GScalar {}
	
	impl crate::gapi::GScalarTraitConst for GScalar {
		#[inline] fn as_raw_GScalar(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GScalarTrait for GScalar {
		#[inline] fn as_raw_mut_GScalar(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GScalar {
		/// Constructs an empty GScalar
		/// 
		/// Normally, empty G-API data objects denote a starting point of
		/// the graph. When an empty GScalar is assigned to a result of some
		/// operation, it obtains a functional link to this operation (and
		/// is not empty anymore).
		#[inline]
		pub fn default() -> Result<crate::gapi::GScalar> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GScalar_GScalar(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GScalar::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Constructs a value-initialized GScalar
		/// 
		/// In contrast with GMat (which can be either an explicit graph input
		/// or a result of some operation), GScalars may have their values
		/// be associated at graph construction time. It is useful when
		/// some operation has a GScalar input which doesn't change during
		/// the program execution, and is set only once. In this case,
		/// there is no need to declare such GScalar as a graph input.
		/// 
		/// 
		/// Note: The value of GScalar may be overwritten by assigning some
		/// other GScalar to the object using `operator=` -- on the
		/// assignment, the old GScalar value is discarded.
		/// 
		/// ## Parameters
		/// * s: a cv::Scalar value to associate with this GScalar object.
		#[inline]
		pub fn new(s: core::Scalar) -> Result<crate::gapi::GScalar> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GScalar_GScalar_const_ScalarR(&s, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GScalar::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Constructs a value-initialized GScalar
		/// 
		/// In contrast with GMat (which can be either an explicit graph input
		/// or a result of some operation), GScalars may have their values
		/// be associated at graph construction time. It is useful when
		/// some operation has a GScalar input which doesn't change during
		/// the program execution, and is set only once. In this case,
		/// there is no need to declare such GScalar as a graph input.
		/// 
		/// 
		/// Note: The value of GScalar may be overwritten by assigning some
		/// other GScalar to the object using `operator=` -- on the
		/// assignment, the old GScalar value is discarded.
		/// 
		/// ## Parameters
		/// * s: a cv::Scalar value to associate with this GScalar object.
		/// 
		/// ## Overloaded parameters
		/// 
		///  Constructs a value-initialized GScalar
		/// 
		/// * s: a cv::Scalar value to associate with this GScalar object.
		#[inline]
		pub fn new_1(mut s: core::Scalar) -> Result<crate::gapi::GScalar> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GScalar_GScalar_ScalarRR(&mut s, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GScalar::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Constructs a value-initialized GScalar
		/// 
		/// In contrast with GMat (which can be either an explicit graph input
		/// or a result of some operation), GScalars may have their values
		/// be associated at graph construction time. It is useful when
		/// some operation has a GScalar input which doesn't change during
		/// the program execution, and is set only once. In this case,
		/// there is no need to declare such GScalar as a graph input.
		/// 
		/// 
		/// Note: The value of GScalar may be overwritten by assigning some
		/// other GScalar to the object using `operator=` -- on the
		/// assignment, the old GScalar value is discarded.
		/// 
		/// ## Parameters
		/// * s: a cv::Scalar value to associate with this GScalar object.
		/// 
		/// ## Overloaded parameters
		/// 
		///  Constructs a value-initialized GScalar
		/// 
		/// * v0: A `double` value to associate with this GScalar. Note
		///   that only the first component of a four-component cv::Scalar is
		///   set to this value, with others remain zeros.
		/// 
		///  This constructor overload is not marked `explicit` and can be
		///  used in G-API expression code like this:
		/// 
		///  [gscalar_implicit](https://github.com/opencv/opencv_contrib/blob/4.8.1/modules/hdf/samples/cpp/tutorial_code/gapi/doc_snippets/api_ref_snippets.cpp#L1)
		/// 
		///  Here operator+(GMat,GScalar) is used to wrap cv::gapi::addC()
		///  and a value-initialized GScalar is created on the fly.
		/// 
		///  @overload
		#[inline]
		pub fn new_2(v0: f64) -> Result<crate::gapi::GScalar> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GScalar_GScalar_double(v0, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GScalar::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl Clone for GScalar {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_GScalar_implicitClone_const(self.as_raw_GScalar())) }
		}
	}
	
	impl std::fmt::Debug for GScalar {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GScalar")
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::GScalarDesc]
	pub trait GScalarDescTraitConst {
		fn as_raw_GScalarDesc(&self) -> *const c_void;
	
		#[inline]
		fn equals(&self, unnamed: &crate::gapi::GScalarDesc) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GScalarDesc_operatorEQ_const_const_GScalarDescR(self.as_raw_GScalarDesc(), unnamed.as_raw_GScalarDesc(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn not_equals(&self, rhs: &crate::gapi::GScalarDesc) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GScalarDesc_operatorNE_const_const_GScalarDescR(self.as_raw_GScalarDesc(), rhs.as_raw_GScalarDesc(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::gapi::GScalarDesc]
	pub trait GScalarDescTrait: crate::gapi::GScalarDescTraitConst {
		fn as_raw_mut_GScalarDesc(&mut self) -> *mut c_void;
	
	}
	
	/// \addtogroup gapi_meta_args
	pub struct GScalarDesc {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GScalarDesc }
	
	impl Drop for GScalarDesc {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GScalarDesc_delete(self.as_raw_mut_GScalarDesc()) };
		}
	}
	
	unsafe impl Send for GScalarDesc {}
	
	impl crate::gapi::GScalarDescTraitConst for GScalarDesc {
		#[inline] fn as_raw_GScalarDesc(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GScalarDescTrait for GScalarDesc {
		#[inline] fn as_raw_mut_GScalarDesc(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GScalarDesc {
		fn default() -> Self {
			unsafe { Self::from_raw(sys::cv_GScalarDesc_defaultNew_const()) }
		}
		
	}
	
	impl Clone for GScalarDesc {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_GScalarDesc_implicitClone_const(self.as_raw_GScalarDesc())) }
		}
	}
	
	impl std::fmt::Debug for GScalarDesc {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GScalarDesc")
				.finish()
		}
	}
	
	impl Default for GScalarDesc {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}
	
	/// Constant methods for [crate::gapi::GStreamingCompiled]
	pub trait GStreamingCompiledTraitConst {
		fn as_raw_GStreamingCompiled(&self) -> *const c_void;
	
		/// Test if the pipeline is running.
		/// 
		/// 
		/// Note: This method is not thread-safe (with respect to the user
		/// side) at the moment. Protect the access if
		/// start()/stop()/setSource() may be called on the same object in
		/// multiple threads in your application.
		/// 
		/// ## Returns
		/// true if the current stream is not over yet.
		#[inline]
		fn running(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GStreamingCompiled_running_const(self.as_raw_GStreamingCompiled(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Check if compiled object is valid (non-empty)
		/// 
		/// ## Returns
		/// true if the object is runnable (valid), false otherwise
		#[inline]
		fn to_bool(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GStreamingCompiled_operator_bool_const(self.as_raw_GStreamingCompiled(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::gapi::GStreamingCompiled]
	pub trait GStreamingCompiledTrait: crate::gapi::GStreamingCompiledTraitConst {
		fn as_raw_mut_GStreamingCompiled(&mut self) -> *mut c_void;
	
		/// Specify the input data to GStreamingCompiled for
		/// processing, a generic version.
		/// 
		/// Use gin() to create an input parameter vector.
		/// 
		/// Input vectors must have the same number of elements as defined
		/// in the cv::GComputation protocol (at the moment of its
		/// construction). Shapes of elements also must conform to protocol
		/// (e.g. cv::Mat needs to be passed where cv::GMat has been
		/// declared as input, and so on). Run-time exception is generated
		/// on type mismatch.
		/// 
		/// In contrast with regular GCompiled, user can also pass an
		/// object of type GVideoCapture for a GMat parameter of the parent
		/// GComputation.  The compiled pipeline will start fetching data
		/// from that GVideoCapture and feeding it into the
		/// pipeline. Pipeline stops when a GVideoCapture marks end of the
		/// stream (or when stop() is called).
		/// 
		/// Passing a regular Mat for a GMat parameter makes it "infinite"
		/// source -- pipeline may run forever feeding with this Mat until
		/// stopped explicitly.
		/// 
		/// Currently only a single GVideoCapture is supported as input. If
		/// the parent GComputation is declared with multiple input GMat's,
		/// one of those can be specified as GVideoCapture but all others
		/// must be regular Mat objects.
		/// 
		/// Throws if pipeline is already running. Use stop() and then
		/// setSource() to run the graph on a new video stream.
		/// 
		/// 
		/// Note: This method is not thread-safe (with respect to the user
		/// side) at the moment. Protect the access if
		/// start()/stop()/setSource() may be called on the same object in
		/// multiple threads in your application.
		/// 
		/// ## Parameters
		/// * ins: vector of inputs to process.
		/// ## See also
		/// gin
		#[inline]
		fn set_source(&mut self, mut ins: crate::gapi::GRunArgs) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GStreamingCompiled_setSource_GRunArgsRR(self.as_raw_mut_GStreamingCompiled(), ins.as_raw_mut_VectorOfGRunArg(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// @private -- Exclude this function from OpenCV documentation
		#[inline]
		fn set_source_1(&mut self, callback: &crate::gapi::Detail_ExtractArgsCallback) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GStreamingCompiled_setSource_const_ExtractArgsCallbackR(self.as_raw_mut_GStreamingCompiled(), callback.as_raw_Detail_ExtractArgsCallback(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Start the pipeline execution.
		/// 
		/// Use pull()/try_pull() to obtain data. Throws an exception if
		/// a video source was not specified.
		/// 
		/// setSource() must be called first, even if the pipeline has been
		/// working already and then stopped (explicitly via stop() or due
		/// stream completion)
		/// 
		/// 
		/// Note: This method is not thread-safe (with respect to the user
		/// side) at the moment. Protect the access if
		/// start()/stop()/setSource() may be called on the same object in
		/// multiple threads in your application.
		#[inline]
		fn start(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GStreamingCompiled_start(self.as_raw_mut_GStreamingCompiled(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Stop (abort) processing the pipeline.
		/// 
		/// Note - it is not pause but a complete stop. Calling start()
		/// will cause G-API to start processing the stream from the early beginning.
		/// 
		/// Throws if the pipeline is not running.
		#[inline]
		fn stop(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GStreamingCompiled_stop(self.as_raw_mut_GStreamingCompiled(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// \addtogroup gapi_main_classes
	/// /
	/// 
	///  Represents a computation (graph) compiled for streaming.
	/// 
	///  This class represents a product of graph compilation (calling
	///  cv::GComputation::compileStreaming()). Objects of this class
	///  actually do stream processing, and the whole pipeline execution
	///  complexity is incapsulated into objects of this class. Execution
	///  model has two levels: at the very top, the execution of a
	///  heterogeneous graph is aggressively pipelined; at the very bottom
	///  the execution of every internal block is determined by its
	///  associated backend. Backends are selected based on kernel packages
	///  passed via compilation arguments ( see [gapi_compile_args],
	///  GNetworkPackage, GKernelPackage for details).
	/// 
	///  GStreamingCompiled objects have a "player" semantics -- there are
	///  methods like start() and stop(). GStreamingCompiled has a full
	///  control over a videostream and so is stateful. You need to specify the
	///  input stream data using setSource() and then call start() to
	///  actually start processing. After that, use pull() or try_pull() to
	///  obtain next processed data frame from the graph in a blocking or
	///  non-blocking way, respectively.
	/// 
	///  Currently a single GStreamingCompiled can process only one video
	///  streat at time. Produce multiple GStreamingCompiled objects to run the
	///  same graph on multiple video streams.
	/// ## See also
	/// GCompiled
	pub struct GStreamingCompiled {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GStreamingCompiled }
	
	impl Drop for GStreamingCompiled {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GStreamingCompiled_delete(self.as_raw_mut_GStreamingCompiled()) };
		}
	}
	
	unsafe impl Send for GStreamingCompiled {}
	
	impl crate::gapi::GStreamingCompiledTraitConst for GStreamingCompiled {
		#[inline] fn as_raw_GStreamingCompiled(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GStreamingCompiledTrait for GStreamingCompiled {
		#[inline] fn as_raw_mut_GStreamingCompiled(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GStreamingCompiled {
		#[inline]
		pub fn default() -> Result<crate::gapi::GStreamingCompiled> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GStreamingCompiled_GStreamingCompiled(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GStreamingCompiled::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl Clone for GStreamingCompiled {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_GStreamingCompiled_implicitClone_const(self.as_raw_GStreamingCompiled())) }
		}
	}
	
	impl std::fmt::Debug for GStreamingCompiled {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GStreamingCompiled")
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::GTransform]
	pub trait GTransformTraitConst {
		fn as_raw_GTransform(&self) -> *const c_void;
	
		#[inline]
		fn description(&self) -> String {
			let ret = unsafe { sys::cv_GTransform_propDescription_const(self.as_raw_GTransform()) };
			let ret = unsafe { String::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::gapi::GTransform]
	pub trait GTransformTrait: crate::gapi::GTransformTraitConst {
		fn as_raw_mut_GTransform(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_description(&mut self, val: &str) {
			extern_container_arg!(nofail mut val);
			let ret = unsafe { sys::cv_GTransform_propDescription_string(self.as_raw_mut_GTransform(), val.opencv_as_extern_mut()) };
			ret
		}
		
	}
	
	pub struct GTransform {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GTransform }
	
	impl Drop for GTransform {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GTransform_delete(self.as_raw_mut_GTransform()) };
		}
	}
	
	unsafe impl Send for GTransform {}
	
	impl crate::gapi::GTransformTraitConst for GTransform {
		#[inline] fn as_raw_GTransform(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GTransformTrait for GTransform {
		#[inline] fn as_raw_mut_GTransform(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GTransform {
	}
	
	impl std::fmt::Debug for GTransform {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GTransform")
				.field("description", &crate::gapi::GTransformTraitConst::description(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::GTypeInfo]
	pub trait GTypeInfoTraitConst {
		fn as_raw_GTypeInfo(&self) -> *const c_void;
	
		#[inline]
		fn shape(&self) -> crate::gapi::GShape {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GTypeInfo_propShape_const(self.as_raw_GTypeInfo(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		#[inline]
		fn kind(&self) -> crate::gapi::Detail_OpaqueKind {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_GTypeInfo_propKind_const(self.as_raw_GTypeInfo(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
	}
	
	/// Mutable methods for [crate::gapi::GTypeInfo]
	pub trait GTypeInfoTrait: crate::gapi::GTypeInfoTraitConst {
		fn as_raw_mut_GTypeInfo(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_shape(&mut self, val: crate::gapi::GShape) {
			let ret = unsafe { sys::cv_GTypeInfo_propShape_GShape(self.as_raw_mut_GTypeInfo(), val) };
			ret
		}
		
		#[inline]
		fn set_kind(&mut self, val: crate::gapi::Detail_OpaqueKind) {
			let ret = unsafe { sys::cv_GTypeInfo_propKind_OpaqueKind(self.as_raw_mut_GTypeInfo(), val) };
			ret
		}
		
	}
	
	pub struct GTypeInfo {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GTypeInfo }
	
	impl Drop for GTypeInfo {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_GTypeInfo_delete(self.as_raw_mut_GTypeInfo()) };
		}
	}
	
	unsafe impl Send for GTypeInfo {}
	
	impl crate::gapi::GTypeInfoTraitConst for GTypeInfo {
		#[inline] fn as_raw_GTypeInfo(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GTypeInfoTrait for GTypeInfo {
		#[inline] fn as_raw_mut_GTypeInfo(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GTypeInfo {
		fn default() -> Self {
			unsafe { Self::from_raw(sys::cv_GTypeInfo_defaultNew_const()) }
		}
		
	}
	
	impl Clone for GTypeInfo {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_GTypeInfo_implicitClone_const(self.as_raw_GTypeInfo())) }
		}
	}
	
	impl std::fmt::Debug for GTypeInfo {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GTypeInfo")
				.field("shape", &crate::gapi::GTypeInfoTraitConst::shape(self))
				.field("kind", &crate::gapi::GTypeInfoTraitConst::kind(self))
				.finish()
		}
	}
	
	impl Default for GTypeInfo {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}
	
	/// Constant methods for [crate::gapi::MediaFrame]
	pub trait MediaFrameTraitConst {
		fn as_raw_MediaFrame(&self) -> *const c_void;
	
		/// Returns a media frame descriptor -- the information
		/// about the media format, dimensions, etc.
		/// ## Returns
		/// a cv::GFrameDesc
		#[inline]
		fn desc(&self) -> Result<crate::gapi::GFrameDesc> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MediaFrame_desc_const(self.as_raw_MediaFrame(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GFrameDesc::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// @private -- exclude from the OpenCV documentation for now.
		#[inline]
		fn blob_params(&self) -> Result<crate::gapi::any> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MediaFrame_blobParams_const(self.as_raw_MediaFrame(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::any::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::gapi::MediaFrame]
	pub trait MediaFrameTrait: crate::gapi::MediaFrameTraitConst {
		fn as_raw_mut_MediaFrame(&mut self) -> *mut c_void;
	
	}
	
	/// \addtogroup gapi_data_structures
	/// 
	///  Extra G-API data structures used to pass input/output data
	///  to the graph for processing.
	/// /
	/// 
	///  cv::MediaFrame class represents an image/media frame
	///  obtained from an external source.
	/// 
	///  cv::MediaFrame represents image data as specified in
	///  cv::MediaFormat. cv::MediaFrame is designed to be a thin wrapper over some
	///  external memory of buffer; the class itself provides an uniform
	///  interface over such types of memory. cv::MediaFrame wraps data from
	///  a camera driver or from a media codec and provides an abstraction
	///  layer over this memory to G-API. MediaFrame defines a compact interface
	///  to access and manage the underlying data; the implementation is
	///  fully defined by the associated Adapter (which is usually
	///  user-defined).
	/// ## See also
	/// cv::RMat
	pub struct MediaFrame {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { MediaFrame }
	
	impl Drop for MediaFrame {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_MediaFrame_delete(self.as_raw_mut_MediaFrame()) };
		}
	}
	
	unsafe impl Send for MediaFrame {}
	
	impl crate::gapi::MediaFrameTraitConst for MediaFrame {
		#[inline] fn as_raw_MediaFrame(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::MediaFrameTrait for MediaFrame {
		#[inline] fn as_raw_mut_MediaFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl MediaFrame {
		/// Constructs an empty MediaFrame
		/// 
		/// The constructed object has no any data associated with it.
		#[inline]
		pub fn default() -> Result<crate::gapi::MediaFrame> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MediaFrame_MediaFrame(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::MediaFrame::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for MediaFrame {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("MediaFrame")
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::MediaFrame_IAdapter]
	pub trait MediaFrame_IAdapterTraitConst {
		fn as_raw_MediaFrame_IAdapter(&self) -> *const c_void;
	
		#[inline]
		fn meta(&self) -> Result<crate::gapi::GFrameDesc> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MediaFrame_IAdapter_meta_const(self.as_raw_MediaFrame_IAdapter(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GFrameDesc::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn blob_params(&self) -> Result<crate::gapi::any> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_MediaFrame_IAdapter_blobParams_const(self.as_raw_MediaFrame_IAdapter(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::any::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::gapi::MediaFrame_IAdapter]
	pub trait MediaFrame_IAdapterTrait: crate::gapi::MediaFrame_IAdapterTraitConst {
		fn as_raw_mut_MediaFrame_IAdapter(&mut self) -> *mut c_void;
	
	}
	
	/// An interface class for MediaFrame data adapters.
	/// 
	/// Implement this interface to wrap media data in the MediaFrame. It
	/// makes sense to implement this class if there is a custom
	/// cv::gapi::wip::IStreamSource defined -- in this case, a stream
	/// source can produce MediaFrame objects with this adapter and the
	/// media data may be passed to graph without any copy. For example, a
	/// GStreamer-based stream source can implement an adapter over
	/// `GstBuffer` and G-API will transparently use it in the graph.
	pub struct MediaFrame_IAdapter {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { MediaFrame_IAdapter }
	
	impl Drop for MediaFrame_IAdapter {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_MediaFrame_IAdapter_delete(self.as_raw_mut_MediaFrame_IAdapter()) };
		}
	}
	
	unsafe impl Send for MediaFrame_IAdapter {}
	
	impl crate::gapi::MediaFrame_IAdapterTraitConst for MediaFrame_IAdapter {
		#[inline] fn as_raw_MediaFrame_IAdapter(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::MediaFrame_IAdapterTrait for MediaFrame_IAdapter {
		#[inline] fn as_raw_mut_MediaFrame_IAdapter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl MediaFrame_IAdapter {
	}
	
	impl std::fmt::Debug for MediaFrame_IAdapter {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("MediaFrame_IAdapter")
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::MediaFrame_View]
	pub trait MediaFrame_ViewTraitConst {
		fn as_raw_MediaFrame_View(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::gapi::MediaFrame_View]
	pub trait MediaFrame_ViewTrait: crate::gapi::MediaFrame_ViewTraitConst {
		fn as_raw_mut_MediaFrame_View(&mut self) -> *mut c_void;
	
	}
	
	/// Provides access to the MediaFrame's underlying data.
	/// 
	/// This object contains the necessary information to access the pixel
	/// data of the associated MediaFrame: arrays of pointers and strides
	/// (distance between every plane row, in bytes) for every image
	/// plane, as defined in cv::MediaFormat.
	/// There may be up to four image planes in MediaFrame.
	/// 
	/// Depending on the MediaFrame::Access flag passed in
	/// MediaFrame::access(), a MediaFrame::View may be read- or
	/// write-only.
	/// 
	/// Depending on the MediaFrame::IAdapter implementation associated
	/// with the parent MediaFrame, writing to memory with
	/// MediaFrame::Access::R flag may have no effect or lead to
	/// undefined behavior. Same applies to reading the memory with
	/// MediaFrame::Access::W flag -- again, depending on the IAdapter
	/// implementation, the host-side buffer the view provides access to
	/// may have no current data stored in (so in-place editing of the
	/// buffer contents may not be possible).
	/// 
	/// MediaFrame::View objects must be handled carefully, as an external
	/// resource associated with MediaFrame may be locked for the time the
	/// MediaFrame::View object exists. Obtaining MediaFrame::View should
	/// be seen as "map" and destroying it as "unmap" in the "map/unmap"
	/// idiom (applicable to OpenCL, device memory, remote
	/// memory).
	/// 
	/// When a MediaFrame buffer is accessed for writing, and the memory
	/// under MediaFrame::View::Ptrs is altered, the data synchronization
	/// of a host-side and device/remote buffer is not guaranteed until the
	/// MediaFrame::View is destroyed. In other words, the real data on the
	/// device or in a remote target may be updated at the MediaFrame::View
	/// destruction only -- but it depends on the associated
	/// MediaFrame::IAdapter implementation.
	pub struct MediaFrame_View {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { MediaFrame_View }
	
	impl Drop for MediaFrame_View {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_MediaFrame_View_delete(self.as_raw_mut_MediaFrame_View()) };
		}
	}
	
	unsafe impl Send for MediaFrame_View {}
	
	impl crate::gapi::MediaFrame_ViewTraitConst for MediaFrame_View {
		#[inline] fn as_raw_MediaFrame_View(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::MediaFrame_ViewTrait for MediaFrame_View {
		#[inline] fn as_raw_mut_MediaFrame_View(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl MediaFrame_View {
		pub const MAX_PLANES: u32 = 4;
		/// @private
		#[inline]
		pub fn copy_mut(mut unnamed: crate::gapi::MediaFrame_View) -> crate::gapi::MediaFrame_View {
			let ret = unsafe { sys::cv_MediaFrame_View_View_ViewRR(unnamed.as_raw_mut_MediaFrame_View()) };
			let ret = unsafe { crate::gapi::MediaFrame_View::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	impl std::fmt::Debug for MediaFrame_View {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("MediaFrame_View")
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::RMat]
	pub trait RMatTraitConst {
		fn as_raw_RMat(&self) -> *const c_void;
	
		#[inline]
		fn desc(&self) -> Result<crate::gapi::GMatDesc> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RMat_desc_const(self.as_raw_RMat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::gapi::RMat]
	pub trait RMatTrait: crate::gapi::RMatTraitConst {
		fn as_raw_mut_RMat(&mut self) -> *mut c_void;
	
	}
	
	/// \addtogroup gapi_data_structures
	pub struct RMat {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { RMat }
	
	impl Drop for RMat {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_RMat_delete(self.as_raw_mut_RMat()) };
		}
	}
	
	unsafe impl Send for RMat {}
	
	impl crate::gapi::RMatTraitConst for RMat {
		#[inline] fn as_raw_RMat(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::RMatTrait for RMat {
		#[inline] fn as_raw_mut_RMat(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl RMat {
		#[inline]
		pub fn default() -> crate::gapi::RMat {
			let ret = unsafe { sys::cv_RMat_RMat() };
			let ret = unsafe { crate::gapi::RMat::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	impl std::fmt::Debug for RMat {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("RMat")
				.finish()
		}
	}
	
	impl Default for RMat {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}
	
	/// Constant methods for [crate::gapi::RMat_IAdapter]
	pub trait RMat_IAdapterTraitConst {
		fn as_raw_RMat_IAdapter(&self) -> *const c_void;
	
		#[inline]
		fn desc(&self) -> Result<crate::gapi::GMatDesc> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RMat_IAdapter_desc_const(self.as_raw_RMat_IAdapter(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::gapi::RMat_IAdapter]
	pub trait RMat_IAdapterTrait: crate::gapi::RMat_IAdapterTraitConst {
		fn as_raw_mut_RMat_IAdapter(&mut self) -> *mut c_void;
	
	}
	
	pub struct RMat_IAdapter {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { RMat_IAdapter }
	
	impl Drop for RMat_IAdapter {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_RMat_IAdapter_delete(self.as_raw_mut_RMat_IAdapter()) };
		}
	}
	
	unsafe impl Send for RMat_IAdapter {}
	
	impl crate::gapi::RMat_IAdapterTraitConst for RMat_IAdapter {
		#[inline] fn as_raw_RMat_IAdapter(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::RMat_IAdapterTrait for RMat_IAdapter {
		#[inline] fn as_raw_mut_RMat_IAdapter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl RMat_IAdapter {
	}
	
	impl std::fmt::Debug for RMat_IAdapter {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("RMat_IAdapter")
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::RMat_View]
	pub trait RMat_ViewTraitConst {
		fn as_raw_RMat_View(&self) -> *const c_void;
	
		#[inline]
		fn size(&self) -> Result<core::Size> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RMat_View_size_const(self.as_raw_RMat_View(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn dims(&self) -> Result<core::Vector<i32>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RMat_View_dims_const(self.as_raw_RMat_View(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn cols(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RMat_View_cols_const(self.as_raw_RMat_View(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn rows(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RMat_View_rows_const(self.as_raw_RMat_View(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn typ(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RMat_View_type_const(self.as_raw_RMat_View(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn depth(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RMat_View_depth_const(self.as_raw_RMat_View(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn chan(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RMat_View_chan_const(self.as_raw_RMat_View(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn elem_size(&self) -> Result<size_t> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RMat_View_elemSize_const(self.as_raw_RMat_View(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * i: 0
		#[inline]
		fn step(&self, i: size_t) -> Result<size_t> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RMat_View_step_const_size_t(self.as_raw_RMat_View(), i, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [step] function uses the following default values for its arguments:
		/// * i: 0
		#[inline]
		fn step_def(&self) -> Result<size_t> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RMat_View_step_const(self.as_raw_RMat_View(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn steps(&self) -> Result<core::Vector<size_t>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_RMat_View_steps_const(self.as_raw_RMat_View(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::gapi::RMat_View]
	pub trait RMat_ViewTrait: crate::gapi::RMat_ViewTraitConst {
		fn as_raw_mut_RMat_View(&mut self) -> *mut c_void;
	
	}
	
	pub struct RMat_View {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { RMat_View }
	
	impl Drop for RMat_View {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_RMat_View_delete(self.as_raw_mut_RMat_View()) };
		}
	}
	
	unsafe impl Send for RMat_View {}
	
	impl crate::gapi::RMat_ViewTraitConst for RMat_View {
		#[inline] fn as_raw_RMat_View(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::RMat_ViewTrait for RMat_View {
		#[inline] fn as_raw_mut_RMat_View(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl RMat_View {
		#[inline]
		pub fn default() -> crate::gapi::RMat_View {
			let ret = unsafe { sys::cv_RMat_View_View() };
			let ret = unsafe { crate::gapi::RMat_View::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		pub fn copy_mut(mut unnamed: crate::gapi::RMat_View) -> crate::gapi::RMat_View {
			let ret = unsafe { sys::cv_RMat_View_View_ViewRR(unnamed.as_raw_mut_RMat_View()) };
			let ret = unsafe { crate::gapi::RMat_View::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	impl std::fmt::Debug for RMat_View {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("RMat_View")
				.finish()
		}
	}
	
	impl Default for RMat_View {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}
	
	/// Constant methods for [crate::gapi::Detail_ExtractArgsCallback]
	pub trait Detail_ExtractArgsCallbackTraitConst {
		fn as_raw_Detail_ExtractArgsCallback(&self) -> *const c_void;
	
		#[inline]
		fn apply(&self, info: &crate::gapi::GTypesInfo) -> Result<core::Vector<crate::gapi::GRunArg>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_detail_ExtractArgsCallback_operator___const_const_GTypesInfoR(self.as_raw_Detail_ExtractArgsCallback(), info.as_raw_VectorOfGTypeInfo(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Vector::<crate::gapi::GRunArg>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::gapi::Detail_ExtractArgsCallback]
	pub trait Detail_ExtractArgsCallbackTrait: crate::gapi::Detail_ExtractArgsCallbackTraitConst {
		fn as_raw_mut_Detail_ExtractArgsCallback(&mut self) -> *mut c_void;
	
	}
	
	pub struct Detail_ExtractArgsCallback {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Detail_ExtractArgsCallback }
	
	impl Drop for Detail_ExtractArgsCallback {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_ExtractArgsCallback_delete(self.as_raw_mut_Detail_ExtractArgsCallback()) };
		}
	}
	
	unsafe impl Send for Detail_ExtractArgsCallback {}
	
	impl crate::gapi::Detail_ExtractArgsCallbackTraitConst for Detail_ExtractArgsCallback {
		#[inline] fn as_raw_Detail_ExtractArgsCallback(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::Detail_ExtractArgsCallbackTrait for Detail_ExtractArgsCallback {
		#[inline] fn as_raw_mut_Detail_ExtractArgsCallback(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_ExtractArgsCallback {
	}
	
	impl std::fmt::Debug for Detail_ExtractArgsCallback {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_ExtractArgsCallback")
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::Detail_ExtractMetaCallback]
	pub trait Detail_ExtractMetaCallbackTraitConst {
		fn as_raw_Detail_ExtractMetaCallback(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::gapi::Detail_ExtractMetaCallback]
	pub trait Detail_ExtractMetaCallbackTrait: crate::gapi::Detail_ExtractMetaCallbackTraitConst {
		fn as_raw_mut_Detail_ExtractMetaCallback(&mut self) -> *mut c_void;
	
	}
	
	pub struct Detail_ExtractMetaCallback {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Detail_ExtractMetaCallback }
	
	impl Drop for Detail_ExtractMetaCallback {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_ExtractMetaCallback_delete(self.as_raw_mut_Detail_ExtractMetaCallback()) };
		}
	}
	
	unsafe impl Send for Detail_ExtractMetaCallback {}
	
	impl crate::gapi::Detail_ExtractMetaCallbackTraitConst for Detail_ExtractMetaCallback {
		#[inline] fn as_raw_Detail_ExtractMetaCallback(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::Detail_ExtractMetaCallbackTrait for Detail_ExtractMetaCallback {
		#[inline] fn as_raw_mut_Detail_ExtractMetaCallback(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_ExtractMetaCallback {
	}
	
	impl std::fmt::Debug for Detail_ExtractMetaCallback {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_ExtractMetaCallback")
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::Detail_GArrayU]
	pub trait Detail_GArrayUTraitConst {
		fn as_raw_Detail_GArrayU(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::gapi::Detail_GArrayU]
	pub trait Detail_GArrayUTrait: crate::gapi::Detail_GArrayUTraitConst {
		fn as_raw_mut_Detail_GArrayU(&mut self) -> *mut c_void;
	
	}
	
	pub struct Detail_GArrayU {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Detail_GArrayU }
	
	impl Drop for Detail_GArrayU {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_GArrayU_delete(self.as_raw_mut_Detail_GArrayU()) };
		}
	}
	
	unsafe impl Send for Detail_GArrayU {}
	
	impl crate::gapi::Detail_GArrayUTraitConst for Detail_GArrayU {
		#[inline] fn as_raw_Detail_GArrayU(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::Detail_GArrayUTrait for Detail_GArrayU {
		#[inline] fn as_raw_mut_Detail_GArrayU(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_GArrayU {
	}
	
	impl std::fmt::Debug for Detail_GArrayU {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_GArrayU")
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::Detail_GOpaqueU]
	pub trait Detail_GOpaqueUTraitConst {
		fn as_raw_Detail_GOpaqueU(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::gapi::Detail_GOpaqueU]
	pub trait Detail_GOpaqueUTrait: crate::gapi::Detail_GOpaqueUTraitConst {
		fn as_raw_mut_Detail_GOpaqueU(&mut self) -> *mut c_void;
	
	}
	
	pub struct Detail_GOpaqueU {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Detail_GOpaqueU }
	
	impl Drop for Detail_GOpaqueU {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_detail_GOpaqueU_delete(self.as_raw_mut_Detail_GOpaqueU()) };
		}
	}
	
	unsafe impl Send for Detail_GOpaqueU {}
	
	impl crate::gapi::Detail_GOpaqueUTraitConst for Detail_GOpaqueU {
		#[inline] fn as_raw_Detail_GOpaqueU(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::Detail_GOpaqueUTrait for Detail_GOpaqueU {
		#[inline] fn as_raw_mut_Detail_GOpaqueU(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Detail_GOpaqueU {
	}
	
	impl std::fmt::Debug for Detail_GOpaqueU {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Detail_GOpaqueU")
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::GBackend]
	pub trait GBackendTraitConst {
		fn as_raw_GBackend(&self) -> *const c_void;
	
		#[inline]
		fn equals(&self, rhs: &crate::gapi::GBackend) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_GBackend_operatorEQ_const_const_GBackendR(self.as_raw_GBackend(), rhs.as_raw_GBackend(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::gapi::GBackend]
	pub trait GBackendTrait: crate::gapi::GBackendTraitConst {
		fn as_raw_mut_GBackend(&mut self) -> *mut c_void;
	
	}
	
	/// @private
	pub struct GBackend {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GBackend }
	
	impl Drop for GBackend {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_gapi_GBackend_delete(self.as_raw_mut_GBackend()) };
		}
	}
	
	unsafe impl Send for GBackend {}
	
	impl crate::gapi::GBackendTraitConst for GBackend {
		#[inline] fn as_raw_GBackend(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GBackendTrait for GBackend {
		#[inline] fn as_raw_mut_GBackend(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GBackend {
		#[inline]
		pub fn default() -> Result<crate::gapi::GBackend> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_GBackend_GBackend(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GBackend::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for GBackend {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GBackend")
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::GFunctor]
	pub trait GFunctorTraitConst {
		fn as_raw_GFunctor(&self) -> *const c_void;
	
		#[inline]
		fn impl_(&self) -> Result<crate::gapi::GKernelImpl> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_GFunctor_impl_const(self.as_raw_GFunctor(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GKernelImpl::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn backend(&self) -> Result<crate::gapi::GBackend> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_GFunctor_backend_const(self.as_raw_GFunctor(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::GBackend::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn id(&self) -> Result<String> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_GFunctor_id_const(self.as_raw_GFunctor(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::gapi::GFunctor]
	pub trait GFunctorTrait: crate::gapi::GFunctorTraitConst {
		fn as_raw_mut_GFunctor(&mut self) -> *mut c_void;
	
	}
	
	/// @private
	pub struct GFunctor {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { GFunctor }
	
	impl Drop for GFunctor {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_gapi_GFunctor_delete(self.as_raw_mut_GFunctor()) };
		}
	}
	
	unsafe impl Send for GFunctor {}
	
	impl crate::gapi::GFunctorTraitConst for GFunctor {
		#[inline] fn as_raw_GFunctor(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GFunctorTrait for GFunctor {
		#[inline] fn as_raw_mut_GFunctor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl GFunctor {
	}
	
	impl std::fmt::Debug for GFunctor {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("GFunctor")
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::Scalar]
	pub trait ScalarTraitConst {
		fn as_raw_Scalar(&self) -> *const c_void;
	
		#[inline]
		fn get(&self, i: i32) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_own_Scalar_operator___const_int(self.as_raw_Scalar(), i, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::gapi::Scalar]
	pub trait ScalarTrait: crate::gapi::ScalarTraitConst {
		fn as_raw_mut_Scalar(&mut self) -> *mut c_void;
	
		#[inline]
		fn val(&mut self) -> &mut [f64; 4] {
			let ret = unsafe { sys::cv_gapi_own_Scalar_propVal(self.as_raw_mut_Scalar()) };
			let ret = unsafe { ret.as_mut() }.expect("Function returned null pointer");
			ret
		}
		
		#[inline]
		fn get_mut(&mut self, i: i32) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_own_Scalar_operator___int(self.as_raw_mut_Scalar(), i, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	pub struct Scalar {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Scalar }
	
	impl Drop for Scalar {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_gapi_own_Scalar_delete(self.as_raw_mut_Scalar()) };
		}
	}
	
	unsafe impl Send for Scalar {}
	
	impl crate::gapi::ScalarTraitConst for Scalar {
		#[inline] fn as_raw_Scalar(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::ScalarTrait for Scalar {
		#[inline] fn as_raw_mut_Scalar(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Scalar {
		#[inline]
		pub fn default() -> crate::gapi::Scalar {
			let ret = unsafe { sys::cv_gapi_own_Scalar_Scalar() };
			let ret = unsafe { crate::gapi::Scalar::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		pub fn new(v0: f64) -> Result<crate::gapi::Scalar> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_own_Scalar_Scalar_double(v0, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::Scalar::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * v2: 0
		/// * v3: 0
		#[inline]
		pub fn new_1(v0: f64, v1: f64, v2: f64, v3: f64) -> Result<crate::gapi::Scalar> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_own_Scalar_Scalar_double_double_double_double(v0, v1, v2, v3, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::Scalar::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * v2: 0
		/// * v3: 0
		#[inline]
		pub fn new_def(v0: f64, v1: f64) -> Result<crate::gapi::Scalar> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_own_Scalar_Scalar_double_double(v0, v1, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::Scalar::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn all(v0: f64) -> Result<crate::gapi::Scalar> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_own_Scalar_all_double(v0, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::Scalar::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for Scalar {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Scalar")
				.finish()
		}
	}
	
	impl Default for Scalar {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}
	
	/// Specify queue capacity for streaming execution.
	/// 
	/// In the streaming mode the pipeline steps are connected with queues
	/// and this compile argument controls every queue's size.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct queue_capacity {
		pub capacity: size_t,
	}
	
	opencv_type_simple! { crate::gapi::queue_capacity }
	
	impl queue_capacity {
		/// ## C++ default parameters
		/// * cap: 1
		#[inline]
		pub fn new(cap: size_t) -> Result<crate::gapi::queue_capacity> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_streaming_queue_capacity_queue_capacity_size_t(cap, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * cap: 1
		#[inline]
		pub fn new_def() -> Result<crate::gapi::queue_capacity> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_streaming_queue_capacity_queue_capacity(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Constant methods for [crate::gapi::use_only]
	pub trait use_onlyTraitConst {
		fn as_raw_use_only(&self) -> *const c_void;
	
		#[inline]
		fn pkg(&self) -> crate::gapi::GKernelPackage {
			let ret = unsafe { sys::cv_gapi_use_only_propPkg_const(self.as_raw_use_only()) };
			let ret = unsafe { crate::gapi::GKernelPackage::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::gapi::use_only]
	pub trait use_onlyTrait: crate::gapi::use_onlyTraitConst {
		fn as_raw_mut_use_only(&mut self) -> *mut c_void;
	
		#[inline]
		fn set_pkg(&mut self, mut val: crate::gapi::GKernelPackage) {
			let ret = unsafe { sys::cv_gapi_use_only_propPkg_GKernelPackage(self.as_raw_mut_use_only(), val.as_raw_mut_GKernelPackage()) };
			ret
		}
		
	}
	
	/// \addtogroup gapi_compile_args
	/// /
	/// 
	///  cv::gapi::use_only() is a special combinator which hints G-API to use only
	///  kernels specified in cv::GComputation::compile() (and not to extend kernels available by
	///  default with that package).
	pub struct use_only {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { use_only }
	
	impl Drop for use_only {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_gapi_use_only_delete(self.as_raw_mut_use_only()) };
		}
	}
	
	unsafe impl Send for use_only {}
	
	impl crate::gapi::use_onlyTraitConst for use_only {
		#[inline] fn as_raw_use_only(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::use_onlyTrait for use_only {
		#[inline] fn as_raw_mut_use_only(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl use_only {
	}
	
	impl std::fmt::Debug for use_only {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("use_only")
				.field("pkg", &crate::gapi::use_onlyTraitConst::pkg(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::gapi::Data]
	pub trait DataTraitConst: crate::gapi::GRunArgTraitConst {
		fn as_raw_Data(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::gapi::Data]
	pub trait DataTrait: crate::gapi::DataTraitConst + crate::gapi::GRunArgTrait {
		fn as_raw_mut_Data(&mut self) -> *mut c_void;
	
	}
	
	/// This aggregate type represents all types which G-API can
	/// handle (via variant).
	/// 
	/// It only exists to overcome C++ language limitations (where a
	/// `using`-defined class can't be forward-declared).
	pub struct Data {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Data }
	
	impl Drop for Data {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_gapi_wip_Data_delete(self.as_raw_mut_Data()) };
		}
	}
	
	unsafe impl Send for Data {}
	
	impl crate::gapi::GRunArgTraitConst for Data {
		#[inline] fn as_raw_GRunArg(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::GRunArgTrait for Data {
		#[inline] fn as_raw_mut_GRunArg(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::gapi::DataTraitConst for Data {
		#[inline] fn as_raw_Data(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::DataTrait for Data {
		#[inline] fn as_raw_mut_Data(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Data {
	}
	
	boxed_cast_base! { Data, crate::gapi::GRunArg, cv_gapi_wip_Data_to_GRunArg }
	
	impl std::fmt::Debug for Data {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Data")
				.finish()
		}
	}
	
	/// This structure represents a circle to draw.
	/// 
	/// Parameters match cv::circle().
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct Circle {
		/// The center of the circle
		pub center: core::Point,
		/// The radius of the circle
		pub radius: i32,
		/// The color of the  circle
		pub color: core::Scalar,
		/// The thickness of the circle outline, if positive. Negative values, like #FILLED, mean that a filled circle is to be drawn
		pub thick: i32,
		/// The Type of the circle boundary. See #LineTypes
		pub lt: i32,
		/// The Number of fractional bits in the coordinates of the center and in the radius value
		pub shift: i32,
	}
	
	opencv_type_simple! { crate::gapi::Circle }
	
	impl Circle {
		/// Circle constructor
		/// 
		/// ## Parameters
		/// * center_: The center of the circle
		/// * radius_: The radius of the circle
		/// * color_: The color of the  circle
		/// * thick_: The thickness of the circle outline, if positive. Negative values, like #FILLED, mean that a filled circle is to be drawn
		/// * lt_: The Type of the circle boundary. See [line_types]
		/// * shift_: The Number of fractional bits in the coordinates of the center and in the radius value
		/// 
		/// ## C++ default parameters
		/// * thick_: 1
		/// * lt_: 8
		/// * shift_: 0
		#[inline]
		pub fn new(center_: core::Point, radius_: i32, color_: core::Scalar, thick_: i32, lt_: i32, shift_: i32) -> Result<crate::gapi::Circle> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_wip_draw_Circle_Circle_const_PointR_int_const_ScalarR_int_int_int(&center_, radius_, &color_, thick_, lt_, shift_, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Circle constructor
		/// 
		/// ## Parameters
		/// * center_: The center of the circle
		/// * radius_: The radius of the circle
		/// * color_: The color of the  circle
		/// * thick_: The thickness of the circle outline, if positive. Negative values, like #FILLED, mean that a filled circle is to be drawn
		/// * lt_: The Type of the circle boundary. See [line_types]
		/// * shift_: The Number of fractional bits in the coordinates of the center and in the radius value
		/// 
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * thick_: 1
		/// * lt_: 8
		/// * shift_: 0
		#[inline]
		pub fn new_def(center_: core::Point, radius_: i32, color_: core::Scalar) -> Result<crate::gapi::Circle> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_wip_draw_Circle_Circle_const_PointR_int_const_ScalarR(&center_, radius_, &color_, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn default() -> crate::gapi::Circle {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_wip_draw_Circle_Circle(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
	}
	
	impl Default for Circle {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}
	
	/// Constant methods for [crate::gapi::Image]
	pub trait ImageTraitConst {
		fn as_raw_Image(&self) -> *const c_void;
	
		/// The bottom-left corner of the image
		#[inline]
		fn org(&self) -> core::Point {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_wip_draw_Image_propOrg_const(self.as_raw_Image(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// Image to draw
		#[inline]
		fn img(&self) -> core::Mat {
			let ret = unsafe { sys::cv_gapi_wip_draw_Image_propImg_const(self.as_raw_Image()) };
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			ret
		}
		
		/// Alpha channel for image to draw (same size and number of channels)
		#[inline]
		fn alpha(&self) -> core::Mat {
			let ret = unsafe { sys::cv_gapi_wip_draw_Image_propAlpha_const(self.as_raw_Image()) };
			let ret = unsafe { core::Mat::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::gapi::Image]
	pub trait ImageTrait: crate::gapi::ImageTraitConst {
		fn as_raw_mut_Image(&mut self) -> *mut c_void;
	
		/// The bottom-left corner of the image
		#[inline]
		fn set_org(&mut self, val: core::Point) {
			let ret = unsafe { sys::cv_gapi_wip_draw_Image_propOrg_Point(self.as_raw_mut_Image(), val.opencv_as_extern()) };
			ret
		}
		
		/// Image to draw
		#[inline]
		fn set_img(&mut self, mut val: core::Mat) {
			let ret = unsafe { sys::cv_gapi_wip_draw_Image_propImg_Mat(self.as_raw_mut_Image(), val.as_raw_mut_Mat()) };
			ret
		}
		
		/// Alpha channel for image to draw (same size and number of channels)
		#[inline]
		fn set_alpha(&mut self, mut val: core::Mat) {
			let ret = unsafe { sys::cv_gapi_wip_draw_Image_propAlpha_Mat(self.as_raw_mut_Image(), val.as_raw_mut_Mat()) };
			ret
		}
		
	}
	
	/// This structure represents an image to draw.
	/// 
	/// Image is blended on a frame using the specified mask.
	pub struct Image {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Image }
	
	impl Drop for Image {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_gapi_wip_draw_Image_delete(self.as_raw_mut_Image()) };
		}
	}
	
	unsafe impl Send for Image {}
	
	impl crate::gapi::ImageTraitConst for Image {
		#[inline] fn as_raw_Image(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::ImageTrait for Image {
		#[inline] fn as_raw_mut_Image(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Image {
		/// Mosaic constructor
		/// 
		/// ## Parameters
		/// * org_: The bottom-left corner of the image
		/// * img_: Image to draw
		/// * alpha_: Alpha channel for image to draw (same size and number of channels)
		#[inline]
		pub fn new(org_: core::Point, img_: &core::Mat, alpha_: &core::Mat) -> Result<crate::gapi::Image> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_wip_draw_Image_Image_const_PointR_const_MatR_const_MatR(&org_, img_.as_raw_Mat(), alpha_.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::Image::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn default() -> crate::gapi::Image {
			let ret = unsafe { sys::cv_gapi_wip_draw_Image_Image() };
			let ret = unsafe { crate::gapi::Image::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	impl Clone for Image {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_gapi_wip_draw_Image_implicitClone_const(self.as_raw_Image())) }
		}
	}
	
	impl std::fmt::Debug for Image {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Image")
				.field("org", &crate::gapi::ImageTraitConst::org(self))
				.field("img", &crate::gapi::ImageTraitConst::img(self))
				.field("alpha", &crate::gapi::ImageTraitConst::alpha(self))
				.finish()
		}
	}
	
	impl Default for Image {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}
	
	/// This structure represents a line to draw.
	/// 
	/// Parameters match cv::line().
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct Line {
		/// The first point of the line segment
		pub pt1: core::Point,
		/// The second point of the line segment
		pub pt2: core::Point,
		/// The line color
		pub color: core::Scalar,
		/// The thickness of line
		pub thick: i32,
		/// The Type of the line. See #LineTypes
		pub lt: i32,
		/// The number of fractional bits in the point coordinates
		pub shift: i32,
	}
	
	opencv_type_simple! { crate::gapi::Line }
	
	impl Line {
		/// Line constructor
		/// 
		/// ## Parameters
		/// * pt1_: The first point of the line segment
		/// * pt2_: The second point of the line segment
		/// * color_: The line color
		/// * thick_: The thickness of line
		/// * lt_: The Type of the line. See [line_types]
		/// * shift_: The number of fractional bits in the point coordinates
		/// 
		/// ## C++ default parameters
		/// * thick_: 1
		/// * lt_: 8
		/// * shift_: 0
		#[inline]
		pub fn new(pt1_: core::Point, pt2_: core::Point, color_: core::Scalar, thick_: i32, lt_: i32, shift_: i32) -> Result<crate::gapi::Line> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_wip_draw_Line_Line_const_PointR_const_PointR_const_ScalarR_int_int_int(&pt1_, &pt2_, &color_, thick_, lt_, shift_, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Line constructor
		/// 
		/// ## Parameters
		/// * pt1_: The first point of the line segment
		/// * pt2_: The second point of the line segment
		/// * color_: The line color
		/// * thick_: The thickness of line
		/// * lt_: The Type of the line. See [line_types]
		/// * shift_: The number of fractional bits in the point coordinates
		/// 
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * thick_: 1
		/// * lt_: 8
		/// * shift_: 0
		#[inline]
		pub fn new_def(pt1_: core::Point, pt2_: core::Point, color_: core::Scalar) -> Result<crate::gapi::Line> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_wip_draw_Line_Line_const_PointR_const_PointR_const_ScalarR(&pt1_, &pt2_, &color_, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn default() -> crate::gapi::Line {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_wip_draw_Line_Line(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
	}
	
	impl Default for Line {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}
	
	/// This structure represents a mosaicing operation.
	/// 
	/// Mosaicing is a very basic method to obfuscate regions in the image.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct Mosaic {
		/// Coordinates of the mosaic
		pub mos: core::Rect,
		/// Cell size (same for X, Y)
		pub cell_sz: i32,
		/// Decimation (0 stands for no decimation)
		pub decim: i32,
	}
	
	opencv_type_simple! { crate::gapi::Mosaic }
	
	impl Mosaic {
		/// Mosaic constructor
		/// 
		/// ## Parameters
		/// * mos_: Coordinates of the mosaic
		/// * cellSz_: Cell size (same for X, Y)
		/// * decim_: Decimation (0 stands for no decimation)
		#[inline]
		pub fn new(mos_: core::Rect, cell_sz_: i32, decim_: i32) -> Result<crate::gapi::Mosaic> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_wip_draw_Mosaic_Mosaic_const_RectR_int_int(&mos_, cell_sz_, decim_, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn default() -> Result<crate::gapi::Mosaic> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_wip_draw_Mosaic_Mosaic(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Constant methods for [crate::gapi::Poly]
	pub trait PolyTraitConst {
		fn as_raw_Poly(&self) -> *const c_void;
	
		/// Points to connect
		#[inline]
		fn points(&self) -> core::Vector<core::Point> {
			let ret = unsafe { sys::cv_gapi_wip_draw_Poly_propPoints_const(self.as_raw_Poly()) };
			let ret = unsafe { core::Vector::<core::Point>::opencv_from_extern(ret) };
			ret
		}
		
		/// The line color
		#[inline]
		fn color(&self) -> core::Scalar {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_wip_draw_Poly_propColor_const(self.as_raw_Poly(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// The thickness of line
		#[inline]
		fn thick(&self) -> i32 {
			let ret = unsafe { sys::cv_gapi_wip_draw_Poly_propThick_const(self.as_raw_Poly()) };
			ret
		}
		
		/// The Type of the line. See #LineTypes
		#[inline]
		fn lt(&self) -> i32 {
			let ret = unsafe { sys::cv_gapi_wip_draw_Poly_propLt_const(self.as_raw_Poly()) };
			ret
		}
		
		/// The number of fractional bits in the point coordinate
		#[inline]
		fn shift(&self) -> i32 {
			let ret = unsafe { sys::cv_gapi_wip_draw_Poly_propShift_const(self.as_raw_Poly()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::gapi::Poly]
	pub trait PolyTrait: crate::gapi::PolyTraitConst {
		fn as_raw_mut_Poly(&mut self) -> *mut c_void;
	
		/// Points to connect
		#[inline]
		fn set_points(&mut self, mut val: core::Vector<core::Point>) {
			let ret = unsafe { sys::cv_gapi_wip_draw_Poly_propPoints_vectorLPointG(self.as_raw_mut_Poly(), val.as_raw_mut_VectorOfPoint()) };
			ret
		}
		
		/// The line color
		#[inline]
		fn set_color(&mut self, val: core::Scalar) {
			let ret = unsafe { sys::cv_gapi_wip_draw_Poly_propColor_Scalar(self.as_raw_mut_Poly(), val.opencv_as_extern()) };
			ret
		}
		
		/// The thickness of line
		#[inline]
		fn set_thick(&mut self, val: i32) {
			let ret = unsafe { sys::cv_gapi_wip_draw_Poly_propThick_int(self.as_raw_mut_Poly(), val) };
			ret
		}
		
		/// The Type of the line. See #LineTypes
		#[inline]
		fn set_lt(&mut self, val: i32) {
			let ret = unsafe { sys::cv_gapi_wip_draw_Poly_propLt_int(self.as_raw_mut_Poly(), val) };
			ret
		}
		
		/// The number of fractional bits in the point coordinate
		#[inline]
		fn set_shift(&mut self, val: i32) {
			let ret = unsafe { sys::cv_gapi_wip_draw_Poly_propShift_int(self.as_raw_mut_Poly(), val) };
			ret
		}
		
	}
	
	/// This structure represents a polygon to draw.
	pub struct Poly {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Poly }
	
	impl Drop for Poly {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_gapi_wip_draw_Poly_delete(self.as_raw_mut_Poly()) };
		}
	}
	
	unsafe impl Send for Poly {}
	
	impl crate::gapi::PolyTraitConst for Poly {
		#[inline] fn as_raw_Poly(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::PolyTrait for Poly {
		#[inline] fn as_raw_mut_Poly(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Poly {
		/// Mosaic constructor
		/// 
		/// ## Parameters
		/// * points_: Points to connect
		/// * color_: The line color
		/// * thick_: The thickness of line
		/// * lt_: The Type of the line. See [line_types]
		/// * shift_: The number of fractional bits in the point coordinate
		/// 
		/// ## C++ default parameters
		/// * thick_: 1
		/// * lt_: 8
		/// * shift_: 0
		#[inline]
		pub fn new(points_: &core::Vector<core::Point>, color_: core::Scalar, thick_: i32, lt_: i32, shift_: i32) -> Result<crate::gapi::Poly> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_wip_draw_Poly_Poly_const_vectorLPointGR_const_ScalarR_int_int_int(points_.as_raw_VectorOfPoint(), &color_, thick_, lt_, shift_, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::Poly::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Mosaic constructor
		/// 
		/// ## Parameters
		/// * points_: Points to connect
		/// * color_: The line color
		/// * thick_: The thickness of line
		/// * lt_: The Type of the line. See [line_types]
		/// * shift_: The number of fractional bits in the point coordinate
		/// 
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * thick_: 1
		/// * lt_: 8
		/// * shift_: 0
		#[inline]
		pub fn new_def(points_: &core::Vector<core::Point>, color_: core::Scalar) -> Result<crate::gapi::Poly> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_wip_draw_Poly_Poly_const_vectorLPointGR_const_ScalarR(points_.as_raw_VectorOfPoint(), &color_, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::Poly::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn default() -> crate::gapi::Poly {
			let ret = unsafe { sys::cv_gapi_wip_draw_Poly_Poly() };
			let ret = unsafe { crate::gapi::Poly::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	impl Clone for Poly {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_gapi_wip_draw_Poly_implicitClone_const(self.as_raw_Poly())) }
		}
	}
	
	impl std::fmt::Debug for Poly {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Poly")
				.field("points", &crate::gapi::PolyTraitConst::points(self))
				.field("color", &crate::gapi::PolyTraitConst::color(self))
				.field("thick", &crate::gapi::PolyTraitConst::thick(self))
				.field("lt", &crate::gapi::PolyTraitConst::lt(self))
				.field("shift", &crate::gapi::PolyTraitConst::shift(self))
				.finish()
		}
	}
	
	impl Default for Poly {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}
	
	/// This structure represents a rectangle to draw.
	/// 
	/// Parameters match cv::rectangle().
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct Rect {
		/// Coordinates of the rectangle
		pub rect: core::Rect,
		/// The rectangle color or brightness (grayscale image)
		pub color: core::Scalar,
		/// The thickness of lines that make up the rectangle. Negative values, like #FILLED, mean that the function has to draw a filled rectangle
		pub thick: i32,
		/// The type of the line. See #LineTypes
		pub lt: i32,
		/// The number of fractional bits in the point coordinates
		pub shift: i32,
	}
	
	opencv_type_simple! { crate::gapi::Rect }
	
	impl Rect {
		/// Rect constructor
		/// 
		/// ## Parameters
		/// * rect_: Coordinates of the rectangle
		/// * color_: The bottom-left corner of the text string in the image
		/// * thick_: The thickness of lines that make up the rectangle. Negative values, like #FILLED, mean that the function has to draw a filled rectangle
		/// * lt_: The type of the line. See [line_types]
		/// * shift_: The number of fractional bits in the point coordinates
		/// 
		/// ## C++ default parameters
		/// * thick_: 1
		/// * lt_: 8
		/// * shift_: 0
		#[inline]
		pub fn new(rect_: core::Rect, color_: core::Scalar, thick_: i32, lt_: i32, shift_: i32) -> Result<crate::gapi::Rect> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_wip_draw_Rect_Rect_const_RectR_const_ScalarR_int_int_int(&rect_, &color_, thick_, lt_, shift_, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Rect constructor
		/// 
		/// ## Parameters
		/// * rect_: Coordinates of the rectangle
		/// * color_: The bottom-left corner of the text string in the image
		/// * thick_: The thickness of lines that make up the rectangle. Negative values, like #FILLED, mean that the function has to draw a filled rectangle
		/// * lt_: The type of the line. See [line_types]
		/// * shift_: The number of fractional bits in the point coordinates
		/// 
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * thick_: 1
		/// * lt_: 8
		/// * shift_: 0
		#[inline]
		pub fn new_def(rect_: core::Rect, color_: core::Scalar) -> Result<crate::gapi::Rect> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_wip_draw_Rect_Rect_const_RectR_const_ScalarR(&rect_, &color_, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		pub fn default() -> crate::gapi::Rect {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_wip_draw_Rect_Rect(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
	}
	
	impl Default for Rect {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}
	
	/// Constant methods for [crate::gapi::Text]
	pub trait TextTraitConst {
		fn as_raw_Text(&self) -> *const c_void;
	
		/// The text string to be drawn
		#[inline]
		fn text(&self) -> String {
			let ret = unsafe { sys::cv_gapi_wip_draw_Text_propText_const(self.as_raw_Text()) };
			let ret = unsafe { String::opencv_from_extern(ret) };
			ret
		}
		
		/// The bottom-left corner of the text string in the image
		#[inline]
		fn org(&self) -> core::Point {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_wip_draw_Text_propOrg_const(self.as_raw_Text(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// The font type, see #HersheyFonts
		#[inline]
		fn ff(&self) -> i32 {
			let ret = unsafe { sys::cv_gapi_wip_draw_Text_propFf_const(self.as_raw_Text()) };
			ret
		}
		
		/// The font scale factor that is multiplied by the font-specific base size
		#[inline]
		fn fs(&self) -> f64 {
			let ret = unsafe { sys::cv_gapi_wip_draw_Text_propFs_const(self.as_raw_Text()) };
			ret
		}
		
		/// The text color
		#[inline]
		fn color(&self) -> core::Scalar {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_wip_draw_Text_propColor_const(self.as_raw_Text(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// The thickness of the lines used to draw a text
		#[inline]
		fn thick(&self) -> i32 {
			let ret = unsafe { sys::cv_gapi_wip_draw_Text_propThick_const(self.as_raw_Text()) };
			ret
		}
		
		/// The line type. See #LineTypes
		#[inline]
		fn lt(&self) -> i32 {
			let ret = unsafe { sys::cv_gapi_wip_draw_Text_propLt_const(self.as_raw_Text()) };
			ret
		}
		
		/// When true, the image data origin is at the bottom-left corner. Otherwise, it is at the top-left corner
		#[inline]
		fn bottom_left_origin(&self) -> bool {
			let ret = unsafe { sys::cv_gapi_wip_draw_Text_propBottom_left_origin_const(self.as_raw_Text()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::gapi::Text]
	pub trait TextTrait: crate::gapi::TextTraitConst {
		fn as_raw_mut_Text(&mut self) -> *mut c_void;
	
		/// The text string to be drawn
		#[inline]
		fn set_text(&mut self, val: &str) {
			extern_container_arg!(nofail mut val);
			let ret = unsafe { sys::cv_gapi_wip_draw_Text_propText_string(self.as_raw_mut_Text(), val.opencv_as_extern_mut()) };
			ret
		}
		
		/// The bottom-left corner of the text string in the image
		#[inline]
		fn set_org(&mut self, val: core::Point) {
			let ret = unsafe { sys::cv_gapi_wip_draw_Text_propOrg_Point(self.as_raw_mut_Text(), val.opencv_as_extern()) };
			ret
		}
		
		/// The font type, see #HersheyFonts
		#[inline]
		fn set_ff(&mut self, val: i32) {
			let ret = unsafe { sys::cv_gapi_wip_draw_Text_propFf_int(self.as_raw_mut_Text(), val) };
			ret
		}
		
		/// The font scale factor that is multiplied by the font-specific base size
		#[inline]
		fn set_fs(&mut self, val: f64) {
			let ret = unsafe { sys::cv_gapi_wip_draw_Text_propFs_double(self.as_raw_mut_Text(), val) };
			ret
		}
		
		/// The text color
		#[inline]
		fn set_color(&mut self, val: core::Scalar) {
			let ret = unsafe { sys::cv_gapi_wip_draw_Text_propColor_Scalar(self.as_raw_mut_Text(), val.opencv_as_extern()) };
			ret
		}
		
		/// The thickness of the lines used to draw a text
		#[inline]
		fn set_thick(&mut self, val: i32) {
			let ret = unsafe { sys::cv_gapi_wip_draw_Text_propThick_int(self.as_raw_mut_Text(), val) };
			ret
		}
		
		/// The line type. See #LineTypes
		#[inline]
		fn set_lt(&mut self, val: i32) {
			let ret = unsafe { sys::cv_gapi_wip_draw_Text_propLt_int(self.as_raw_mut_Text(), val) };
			ret
		}
		
		/// When true, the image data origin is at the bottom-left corner. Otherwise, it is at the top-left corner
		#[inline]
		fn set_bottom_left_origin(&mut self, val: bool) {
			let ret = unsafe { sys::cv_gapi_wip_draw_Text_propBottom_left_origin_bool(self.as_raw_mut_Text(), val) };
			ret
		}
		
	}
	
	/// * This structure represents a text string to draw.
	/// *
	/// * Parameters match cv::putText().
	pub struct Text {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { Text }
	
	impl Drop for Text {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_gapi_wip_draw_Text_delete(self.as_raw_mut_Text()) };
		}
	}
	
	unsafe impl Send for Text {}
	
	impl crate::gapi::TextTraitConst for Text {
		#[inline] fn as_raw_Text(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::TextTrait for Text {
		#[inline] fn as_raw_mut_Text(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl Text {
		/// Text constructor
		/// 
		/// ## Parameters
		/// * text_: The text string to be drawn
		/// * org_: The bottom-left corner of the text string in the image
		/// * ff_: The font type, see [hershey_fonts]
		/// * fs_: The font scale factor that is multiplied by the font-specific base size
		/// * color_: The text color
		/// * thick_: The thickness of the lines used to draw a text
		/// * lt_: The line type. See [line_types]
		/// * bottom_left_origin_: When true, the image data origin is at the bottom-left corner. Otherwise, it is at the top-left corner
		/// 
		/// ## C++ default parameters
		/// * thick_: 1
		/// * lt_: 8
		/// * bottom_left_origin_: false
		#[inline]
		pub fn new(text_: &str, org_: core::Point, ff_: i32, fs_: f64, color_: core::Scalar, thick_: i32, lt_: i32, bottom_left_origin_: bool) -> Result<crate::gapi::Text> {
			extern_container_arg!(text_);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_wip_draw_Text_Text_const_stringR_const_PointR_int_double_const_ScalarR_int_int_bool(text_.opencv_as_extern(), &org_, ff_, fs_, &color_, thick_, lt_, bottom_left_origin_, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::Text::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Text constructor
		/// 
		/// ## Parameters
		/// * text_: The text string to be drawn
		/// * org_: The bottom-left corner of the text string in the image
		/// * ff_: The font type, see [hershey_fonts]
		/// * fs_: The font scale factor that is multiplied by the font-specific base size
		/// * color_: The text color
		/// * thick_: The thickness of the lines used to draw a text
		/// * lt_: The line type. See [line_types]
		/// * bottom_left_origin_: When true, the image data origin is at the bottom-left corner. Otherwise, it is at the top-left corner
		/// 
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * thick_: 1
		/// * lt_: 8
		/// * bottom_left_origin_: false
		#[inline]
		pub fn new_def(text_: &str, org_: core::Point, ff_: i32, fs_: f64, color_: core::Scalar) -> Result<crate::gapi::Text> {
			extern_container_arg!(text_);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_gapi_wip_draw_Text_Text_const_stringR_const_PointR_int_double_const_ScalarR(text_.opencv_as_extern(), &org_, ff_, fs_, &color_, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::Text::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn default() -> crate::gapi::Text {
			let ret = unsafe { sys::cv_gapi_wip_draw_Text_Text() };
			let ret = unsafe { crate::gapi::Text::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	impl Clone for Text {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { Self::from_raw(sys::cv_gapi_wip_draw_Text_implicitClone_const(self.as_raw_Text())) }
		}
	}
	
	impl std::fmt::Debug for Text {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("Text")
				.field("text", &crate::gapi::TextTraitConst::text(self))
				.field("org", &crate::gapi::TextTraitConst::org(self))
				.field("ff", &crate::gapi::TextTraitConst::ff(self))
				.field("fs", &crate::gapi::TextTraitConst::fs(self))
				.field("color", &crate::gapi::TextTraitConst::color(self))
				.field("thick", &crate::gapi::TextTraitConst::thick(self))
				.field("lt", &crate::gapi::TextTraitConst::lt(self))
				.field("bottom_left_origin", &crate::gapi::TextTraitConst::bottom_left_origin(self))
				.finish()
		}
	}
	
	impl Default for Text {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}
	
	/// Constant methods for [crate::gapi::any]
	pub trait anyTraitConst {
		fn as_raw_any(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::gapi::any]
	pub trait anyTrait: crate::gapi::anyTraitConst {
		fn as_raw_mut_any(&mut self) -> *mut c_void;
	
	}
	
	pub struct any {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { any }
	
	impl Drop for any {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_util_any_delete(self.as_raw_mut_any()) };
		}
	}
	
	unsafe impl Send for any {}
	
	impl crate::gapi::anyTraitConst for any {
		#[inline] fn as_raw_any(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::gapi::anyTrait for any {
		#[inline] fn as_raw_mut_any(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl any {
		#[inline]
		pub fn copy(src: &crate::gapi::any) -> Result<crate::gapi::any> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_util_any_any_const_anyR(src.as_raw_any(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::any::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn copy_mut(src: &mut crate::gapi::any) -> Result<crate::gapi::any> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_util_any_any_anyR(src.as_raw_mut_any(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::gapi::any::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		pub fn default() -> crate::gapi::any {
			let ret = unsafe { sys::cv_util_any_any() };
			let ret = unsafe { crate::gapi::any::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		pub fn copy_mut_1(mut unnamed: crate::gapi::any) -> crate::gapi::any {
			let ret = unsafe { sys::cv_util_any_any_anyRR(unnamed.as_raw_mut_any()) };
			let ret = unsafe { crate::gapi::any::opencv_from_extern(ret) };
			ret
		}
		
	}
	
	impl std::fmt::Debug for any {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("any")
				.finish()
		}
	}
	
	impl Default for any {
		#[inline]
		/// Forwards to infallible Self::default()
		fn default() -> Self {
			Self::default()
		}
	}
}
