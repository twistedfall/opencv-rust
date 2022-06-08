#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Video Encoding/Decoding
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::EncoderParamsTraitConst, super::EncoderParamsTrait, super::EncoderCallBackConst, super::EncoderCallBack, super::VideoWriterConst, super::VideoWriter, super::VideoReaderConst, super::VideoReader, super::RawVideoSourceConst, super::RawVideoSource };
}

pub const AV1: i32 = 11;
pub const Adaptive: i32 = 2;
pub const Bob: i32 = 1;
pub const ColorFormat_BGR: i32 = 2;
pub const ColorFormat_BGRA: i32 = 1;
pub const ColorFormat_GRAY: i32 = 3;
pub const ColorFormat_PROP_NOT_SUPPORTED: i32 = 5;
pub const ColorFormat_YUV: i32 = 4;
pub const H264: i32 = 4;
pub const H264_MVC: i32 = 7;
pub const H264_SVC: i32 = 6;
pub const HEVC: i32 = 8;
pub const JPEG: i32 = 5;
pub const MPEG1: i32 = 0;
pub const MPEG2: i32 = 1;
pub const MPEG4: i32 = 2;
pub const Monochrome: i32 = 0;
pub const NumCodecs: i32 = 12;
pub const NumFormats: i32 = 4;
pub const SF_BGR: i32 = 5;
pub const SF_GRAY: i32 = 5;
pub const SF_IYUV: i32 = 4;
pub const SF_NV12: i32 = 3;
pub const SF_UYVY: i32 = 0;
pub const SF_YUY2: i32 = 1;
pub const SF_YV12: i32 = 2;
/// Y,UV  (4:2:0)
pub const Uncompressed_NV12: i32 = 1314271538;
/// UYVY (4:2:2)
pub const Uncompressed_UYVY: i32 = 1431918169;
/// Y,U,V (4:2:0)
pub const Uncompressed_YUV420: i32 = 1230591318;
/// YUYV/YUY2 (4:2:2)
pub const Uncompressed_YUYV: i32 = 1498765654;
/// Y,V,U (4:2:0)
pub const Uncompressed_YV12: i32 = 1498820914;
pub const VC1: i32 = 3;
pub const VP8: i32 = 9;
pub const VP9: i32 = 10;
/// Status of VideoReaderInitParams::allowFrameDrop initialization.
pub const VideoReaderProps_PROP_ALLOW_FRAME_DROP: i32 = 8;
/// Set the ColorFormat of the decoded frame.  This can be changed before every call to nextFrame() and retrieve().
pub const VideoReaderProps_PROP_COLOR_FORMAT: i32 = 6;
/// Index for retrieving the decoded frame using retrieve().
pub const VideoReaderProps_PROP_DECODED_FRAME_IDX: i32 = 0;
/// Index for retrieving the extra data associated with a video source using retrieve().
pub const VideoReaderProps_PROP_EXTRA_DATA_INDEX: i32 = 1;
/// FFmpeg source only - Indicates whether the Last Raw Frame (LRF), output from VideoReader::retrieve() when VideoReader is initialized in raw mode, contains encoded data for a key frame.
pub const VideoReaderProps_PROP_LRF_HAS_KEY_FRAME: i32 = 5;
pub const VideoReaderProps_PROP_NOT_SUPPORTED: i32 = 9;
/// Number of raw packages recieved since the last call to grab().
pub const VideoReaderProps_PROP_NUMBER_OF_RAW_PACKAGES_SINCE_LAST_GRAB: i32 = 3;
/// Status of raw mode.
pub const VideoReaderProps_PROP_RAW_MODE: i32 = 4;
/// Base index for retrieving raw encoded data using retrieve().
pub const VideoReaderProps_PROP_RAW_PACKAGES_BASE_INDEX: i32 = 2;
/// Status of VideoReaderInitParams::udpSource initialization.
pub const VideoReaderProps_PROP_UDP_SOURCE: i32 = 7;
pub const Weave: i32 = 0;
pub const YUV420: i32 = 1;
pub const YUV422: i32 = 2;
pub const YUV444: i32 = 3;
/// Chroma formats supported by cudacodec::VideoReader.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ChromaFormat {
	Monochrome = 0,
	YUV420 = 1,
	YUV422 = 2,
	YUV444 = 3,
	NumFormats = 4,
}

opencv_type_enum! { crate::cudacodec::ChromaFormat }

/// Video codecs supported by cudacodec::VideoReader .
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Codec {
	MPEG1 = 0,
	MPEG2 = 1,
	MPEG4 = 2,
	VC1 = 3,
	H264 = 4,
	JPEG = 5,
	H264_SVC = 6,
	H264_MVC = 7,
	HEVC = 8,
	VP8 = 9,
	VP9 = 10,
	AV1 = 11,
	NumCodecs = 12,
	/// Y,U,V (4:2:0)
	Uncompressed_YUV420 = 1230591318,
	/// Y,V,U (4:2:0)
	Uncompressed_YV12 = 1498820914,
	/// Y,UV  (4:2:0)
	Uncompressed_NV12 = 1314271538,
	/// YUYV/YUY2 (4:2:2)
	Uncompressed_YUYV = 1498765654,
	/// UYVY (4:2:2)
	Uncompressed_UYVY = 1431918169,
}

opencv_type_enum! { crate::cudacodec::Codec }

/// ColorFormat for the frame returned by nextFrame()/retrieve().
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ColorFormat {
	BGRA = 1,
	BGR = 2,
	GRAY = 3,
	YUV = 4,
	PROP_NOT_SUPPORTED = 5,
}

opencv_type_enum! { crate::cudacodec::ColorFormat }

/// Deinterlacing mode used by decoder.
/// ## Parameters
/// * Weave: Weave both fields (no deinterlacing). For progressive content and for content that doesn't need deinterlacing.
/// Bob Drop one field.
/// * Adaptive: Adaptive deinterlacing needs more video memory than other deinterlacing modes.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DeinterlaceMode {
	Weave = 0,
	Bob = 1,
	Adaptive = 2,
}

opencv_type_enum! { crate::cudacodec::DeinterlaceMode }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EncoderCallBack_PicType {
	IFRAME = 1,
	PFRAME = 2,
	BFRAME = 3,
}

opencv_type_enum! { crate::cudacodec::EncoderCallBack_PicType }

/// /////////////////////////////// Video Encoding //////////////////////////////////
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SurfaceFormat {
	SF_UYVY = 0,
	SF_YUY2 = 1,
	SF_YV12 = 2,
	SF_NV12 = 3,
	SF_IYUV = 4,
	SF_BGR = 5,
	// SF_GRAY = 5 as isize, // duplicate discriminant
}

opencv_type_enum! { crate::cudacodec::SurfaceFormat }

/// cv::cudacodec::VideoReader generic properties identifier.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VideoReaderProps {
	/// Index for retrieving the decoded frame using retrieve().
	PROP_DECODED_FRAME_IDX = 0,
	/// Index for retrieving the extra data associated with a video source using retrieve().
	PROP_EXTRA_DATA_INDEX = 1,
	/// Base index for retrieving raw encoded data using retrieve().
	PROP_RAW_PACKAGES_BASE_INDEX = 2,
	/// Number of raw packages recieved since the last call to grab().
	PROP_NUMBER_OF_RAW_PACKAGES_SINCE_LAST_GRAB = 3,
	/// Status of raw mode.
	PROP_RAW_MODE = 4,
	/// FFmpeg source only - Indicates whether the Last Raw Frame (LRF), output from VideoReader::retrieve() when VideoReader is initialized in raw mode, contains encoded data for a key frame.
	PROP_LRF_HAS_KEY_FRAME = 5,
	/// Set the ColorFormat of the decoded frame.  This can be changed before every call to nextFrame() and retrieve().
	PROP_COLOR_FORMAT = 6,
	/// Status of VideoReaderInitParams::udpSource initialization.
	PROP_UDP_SOURCE = 7,
	/// Status of VideoReaderInitParams::allowFrameDrop initialization.
	PROP_ALLOW_FRAME_DROP = 8,
	PROP_NOT_SUPPORTED = 9,
}

opencv_type_enum! { crate::cudacodec::VideoReaderProps }

/// Creates video reader.
/// 
/// ## Parameters
/// * filename: Name of the input video file.
/// * sourceParams: Pass through parameters for VideoCapure.  VideoCapture with the FFMpeg back end (CAP_FFMPEG) is used to parse the video input.
/// The `sourceParams` parameter allows to specify extra parameters encoded as pairs `(paramId_1, paramValue_1, paramId_2, paramValue_2, ...)`.
///    See cv::VideoCaptureProperties
/// e.g. when streaming from an RTSP source CAP_PROP_OPEN_TIMEOUT_MSEC may need to be set.
/// * params: Initializaton parameters. See cv::cudacodec::VideoReaderInitParams.
/// 
/// FFMPEG is used to read videos. User can implement own demultiplexing with cudacodec::RawVideoSource
/// 
/// ## Overloaded parameters
/// 
/// * source: RAW video source implemented by user.
/// * params: Initializaton parameters. See cv::cudacodec::VideoReaderInitParams.
/// 
/// ## C++ default parameters
/// * params: VideoReaderInitParams()
#[inline]
pub fn create_video_reader_1(source: &core::Ptr<dyn crate::cudacodec::RawVideoSource>, params: crate::cudacodec::VideoReaderInitParams) -> Result<core::Ptr<dyn crate::cudacodec::VideoReader>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cudacodec_createVideoReader_const_Ptr_RawVideoSource_R_const_VideoReaderInitParams(source.as_raw_PtrOfRawVideoSource(), params.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::cudacodec::VideoReader>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Creates video reader.
/// 
/// ## Parameters
/// * filename: Name of the input video file.
/// * sourceParams: Pass through parameters for VideoCapure.  VideoCapture with the FFMpeg back end (CAP_FFMPEG) is used to parse the video input.
/// The `sourceParams` parameter allows to specify extra parameters encoded as pairs `(paramId_1, paramValue_1, paramId_2, paramValue_2, ...)`.
///    See cv::VideoCaptureProperties
/// e.g. when streaming from an RTSP source CAP_PROP_OPEN_TIMEOUT_MSEC may need to be set.
/// * params: Initializaton parameters. See cv::cudacodec::VideoReaderInitParams.
/// 
/// FFMPEG is used to read videos. User can implement own demultiplexing with cudacodec::RawVideoSource
/// 
/// ## C++ default parameters
/// * source_params: {}
/// * params: VideoReaderInitParams()
#[inline]
pub fn create_video_reader(filename: &str, source_params: &core::Vector<i32>, params: crate::cudacodec::VideoReaderInitParams) -> Result<core::Ptr<dyn crate::cudacodec::VideoReader>> {
	extern_container_arg!(filename);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cudacodec_createVideoReader_const_StringR_const_vector_int_R_const_VideoReaderInitParams(filename.opencv_as_extern(), source_params.as_raw_VectorOfi32(), params.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::cudacodec::VideoReader>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Creates video writer.
/// 
/// ## Parameters
/// * fileName: Name of the output video file. Only AVI file format is supported.
/// * frameSize: Size of the input video frames.
/// * fps: Framerate of the created video stream.
/// * format: Surface format of input frames ( SF_UYVY , SF_YUY2 , SF_YV12 , SF_NV12 ,
/// SF_IYUV , SF_BGR or SF_GRAY). BGR or gray frames will be converted to YV12 format before
/// encoding, frames with other formats will be used as is.
/// 
/// The constructors initialize video writer. FFMPEG is used to write videos. User can implement own
/// multiplexing with cudacodec::EncoderCallBack .
/// 
/// ## Overloaded parameters
/// 
/// * encoderCallback: Callbacks for video encoder. See cudacodec::EncoderCallBack . Use it if you
/// want to work with raw video stream.
/// * frameSize: Size of the input video frames.
/// * fps: Framerate of the created video stream.
/// * format: Surface format of input frames ( SF_UYVY , SF_YUY2 , SF_YV12 , SF_NV12 ,
/// SF_IYUV , SF_BGR or SF_GRAY). BGR or gray frames will be converted to YV12 format before
/// encoding, frames with other formats will be used as is.
/// 
/// ## C++ default parameters
/// * format: SF_BGR
#[inline]
pub fn create_video_writer_2(encoder_callback: &core::Ptr<dyn crate::cudacodec::EncoderCallBack>, frame_size: core::Size, fps: f64, format: crate::cudacodec::SurfaceFormat) -> Result<core::Ptr<dyn crate::cudacodec::VideoWriter>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cudacodec_createVideoWriter_const_Ptr_EncoderCallBack_R_Size_double_SurfaceFormat(encoder_callback.as_raw_PtrOfEncoderCallBack(), frame_size.opencv_as_extern(), fps, format, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::cudacodec::VideoWriter>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Creates video writer.
/// 
/// ## Parameters
/// * fileName: Name of the output video file. Only AVI file format is supported.
/// * frameSize: Size of the input video frames.
/// * fps: Framerate of the created video stream.
/// * format: Surface format of input frames ( SF_UYVY , SF_YUY2 , SF_YV12 , SF_NV12 ,
/// SF_IYUV , SF_BGR or SF_GRAY). BGR or gray frames will be converted to YV12 format before
/// encoding, frames with other formats will be used as is.
/// 
/// The constructors initialize video writer. FFMPEG is used to write videos. User can implement own
/// multiplexing with cudacodec::EncoderCallBack .
/// 
/// ## Overloaded parameters
/// 
/// * encoderCallback: Callbacks for video encoder. See cudacodec::EncoderCallBack . Use it if you
/// want to work with raw video stream.
/// * frameSize: Size of the input video frames.
/// * fps: Framerate of the created video stream.
/// * params: Encoder parameters. See cudacodec::EncoderParams.
/// * format: Surface format of input frames ( SF_UYVY , SF_YUY2 , SF_YV12 , SF_NV12 ,
/// SF_IYUV , SF_BGR or SF_GRAY). BGR or gray frames will be converted to YV12 format before
/// encoding, frames with other formats will be used as is.
/// 
/// ## C++ default parameters
/// * format: SF_BGR
#[inline]
pub fn create_video_writer_3(encoder_callback: &core::Ptr<dyn crate::cudacodec::EncoderCallBack>, frame_size: core::Size, fps: f64, params: &crate::cudacodec::EncoderParams, format: crate::cudacodec::SurfaceFormat) -> Result<core::Ptr<dyn crate::cudacodec::VideoWriter>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cudacodec_createVideoWriter_const_Ptr_EncoderCallBack_R_Size_double_const_EncoderParamsR_SurfaceFormat(encoder_callback.as_raw_PtrOfEncoderCallBack(), frame_size.opencv_as_extern(), fps, params.as_raw_EncoderParams(), format, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::cudacodec::VideoWriter>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Creates video writer.
/// 
/// ## Parameters
/// * fileName: Name of the output video file. Only AVI file format is supported.
/// * frameSize: Size of the input video frames.
/// * fps: Framerate of the created video stream.
/// * format: Surface format of input frames ( SF_UYVY , SF_YUY2 , SF_YV12 , SF_NV12 ,
/// SF_IYUV , SF_BGR or SF_GRAY). BGR or gray frames will be converted to YV12 format before
/// encoding, frames with other formats will be used as is.
/// 
/// The constructors initialize video writer. FFMPEG is used to write videos. User can implement own
/// multiplexing with cudacodec::EncoderCallBack .
/// 
/// ## C++ default parameters
/// * format: SF_BGR
#[inline]
pub fn create_video_writer(file_name: &str, frame_size: core::Size, fps: f64, format: crate::cudacodec::SurfaceFormat) -> Result<core::Ptr<dyn crate::cudacodec::VideoWriter>> {
	extern_container_arg!(file_name);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cudacodec_createVideoWriter_const_StringR_Size_double_SurfaceFormat(file_name.opencv_as_extern(), frame_size.opencv_as_extern(), fps, format, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::cudacodec::VideoWriter>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Creates video writer.
/// 
/// ## Parameters
/// * fileName: Name of the output video file. Only AVI file format is supported.
/// * frameSize: Size of the input video frames.
/// * fps: Framerate of the created video stream.
/// * format: Surface format of input frames ( SF_UYVY , SF_YUY2 , SF_YV12 , SF_NV12 ,
/// SF_IYUV , SF_BGR or SF_GRAY). BGR or gray frames will be converted to YV12 format before
/// encoding, frames with other formats will be used as is.
/// 
/// The constructors initialize video writer. FFMPEG is used to write videos. User can implement own
/// multiplexing with cudacodec::EncoderCallBack .
/// 
/// ## Overloaded parameters
/// 
/// * fileName: Name of the output video file. Only AVI file format is supported.
/// * frameSize: Size of the input video frames.
/// * fps: Framerate of the created video stream.
/// * params: Encoder parameters. See cudacodec::EncoderParams .
/// * format: Surface format of input frames ( SF_UYVY , SF_YUY2 , SF_YV12 , SF_NV12 ,
/// SF_IYUV , SF_BGR or SF_GRAY). BGR or gray frames will be converted to YV12 format before
/// encoding, frames with other formats will be used as is.
/// 
/// ## C++ default parameters
/// * format: SF_BGR
#[inline]
pub fn create_video_writer_1(file_name: &str, frame_size: core::Size, fps: f64, params: &crate::cudacodec::EncoderParams, format: crate::cudacodec::SurfaceFormat) -> Result<core::Ptr<dyn crate::cudacodec::VideoWriter>> {
	extern_container_arg!(file_name);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_cudacodec_createVideoWriter_const_StringR_Size_double_const_EncoderParamsR_SurfaceFormat(file_name.opencv_as_extern(), frame_size.opencv_as_extern(), fps, params.as_raw_EncoderParams(), format, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::cudacodec::VideoWriter>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Callbacks for CUDA video encoder.
pub trait EncoderCallBackConst {
	fn as_raw_EncoderCallBack(&self) -> *const c_void;

}

pub trait EncoderCallBack: crate::cudacodec::EncoderCallBackConst {
	fn as_raw_mut_EncoderCallBack(&mut self) -> *mut c_void;

	/// Callback function to signal the start of bitstream that is to be encoded.
	/// 
	/// Callback must allocate buffer for CUDA encoder and return pointer to it and it's size.
	#[inline]
	fn acquire_bit_stream(&mut self, buffer_size: &mut i32) -> Result<*mut u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_EncoderCallBack_acquireBitStream_intX(self.as_raw_mut_EncoderCallBack(), buffer_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Callback function to signal that the encoded bitstream is ready to be written to file.
	#[inline]
	fn release_bit_stream(&mut self, data: &mut u8, size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_EncoderCallBack_releaseBitStream_unsigned_charX_int(self.as_raw_mut_EncoderCallBack(), data, size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Callback function to signal that the encoding operation on the frame has started.
	/// 
	/// ## Parameters
	/// * frameNumber: 
	/// * picType: Specify frame type (I-Frame, P-Frame or B-Frame).
	#[inline]
	fn on_begin_frame(&mut self, frame_number: i32, pic_type: crate::cudacodec::EncoderCallBack_PicType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_EncoderCallBack_onBeginFrame_int_PicType(self.as_raw_mut_EncoderCallBack(), frame_number, pic_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Callback function signals that the encoding operation on the frame has finished.
	/// 
	/// ## Parameters
	/// * frameNumber: 
	/// * picType: Specify frame type (I-Frame, P-Frame or B-Frame).
	#[inline]
	fn on_end_frame(&mut self, frame_number: i32, pic_type: crate::cudacodec::EncoderCallBack_PicType) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_EncoderCallBack_onEndFrame_int_PicType(self.as_raw_mut_EncoderCallBack(), frame_number, pic_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Different parameters for CUDA video encoder.
pub trait EncoderParamsTraitConst {
	fn as_raw_EncoderParams(&self) -> *const c_void;

	/// NVVE_P_INTERVAL,
	#[inline]
	fn p_interval(&self) -> i32 {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_getPropP_Interval_const(self.as_raw_EncoderParams()) };
		ret
	}
	
	/// NVVE_IDR_PERIOD,
	#[inline]
	fn idr_period(&self) -> i32 {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_getPropIDR_Period_const(self.as_raw_EncoderParams()) };
		ret
	}
	
	/// NVVE_DYNAMIC_GOP,
	#[inline]
	fn dynamic_gop(&self) -> i32 {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_getPropDynamicGOP_const(self.as_raw_EncoderParams()) };
		ret
	}
	
	/// NVVE_RC_TYPE,
	#[inline]
	fn rc_type(&self) -> i32 {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_getPropRCType_const(self.as_raw_EncoderParams()) };
		ret
	}
	
	/// NVVE_AVG_BITRATE,
	#[inline]
	fn avg_bitrate(&self) -> i32 {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_getPropAvgBitrate_const(self.as_raw_EncoderParams()) };
		ret
	}
	
	/// NVVE_PEAK_BITRATE,
	#[inline]
	fn peak_bitrate(&self) -> i32 {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_getPropPeakBitrate_const(self.as_raw_EncoderParams()) };
		ret
	}
	
	/// NVVE_QP_LEVEL_INTRA,
	#[inline]
	fn qp_level_intra(&self) -> i32 {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_getPropQP_Level_Intra_const(self.as_raw_EncoderParams()) };
		ret
	}
	
	/// NVVE_QP_LEVEL_INTER_P,
	#[inline]
	fn qp_level_inter_p(&self) -> i32 {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_getPropQP_Level_InterP_const(self.as_raw_EncoderParams()) };
		ret
	}
	
	/// NVVE_QP_LEVEL_INTER_B,
	#[inline]
	fn qp_level_inter_b(&self) -> i32 {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_getPropQP_Level_InterB_const(self.as_raw_EncoderParams()) };
		ret
	}
	
	/// NVVE_DEBLOCK_MODE,
	#[inline]
	fn deblock_mode(&self) -> i32 {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_getPropDeblockMode_const(self.as_raw_EncoderParams()) };
		ret
	}
	
	/// NVVE_PROFILE_LEVEL,
	#[inline]
	fn profile_level(&self) -> i32 {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_getPropProfileLevel_const(self.as_raw_EncoderParams()) };
		ret
	}
	
	/// NVVE_FORCE_INTRA,
	#[inline]
	fn force_intra(&self) -> i32 {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_getPropForceIntra_const(self.as_raw_EncoderParams()) };
		ret
	}
	
	/// NVVE_FORCE_IDR,
	#[inline]
	fn force_idr(&self) -> i32 {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_getPropForceIDR_const(self.as_raw_EncoderParams()) };
		ret
	}
	
	/// NVVE_CLEAR_STAT,
	#[inline]
	fn clear_stat(&self) -> i32 {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_getPropClearStat_const(self.as_raw_EncoderParams()) };
		ret
	}
	
	/// NVVE_SET_DEINTERLACE,
	#[inline]
	fn di_mode(&self) -> i32 {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_getPropDIMode_const(self.as_raw_EncoderParams()) };
		ret
	}
	
	/// NVVE_PRESETS,
	#[inline]
	fn presets(&self) -> i32 {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_getPropPresets_const(self.as_raw_EncoderParams()) };
		ret
	}
	
	/// NVVE_DISABLE_CABAC,
	#[inline]
	fn disable_cabac(&self) -> i32 {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_getPropDisableCabac_const(self.as_raw_EncoderParams()) };
		ret
	}
	
	/// NVVE_CONFIGURE_NALU_FRAMING_TYPE
	#[inline]
	fn nalu_framing_type(&self) -> i32 {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_getPropNaluFramingType_const(self.as_raw_EncoderParams()) };
		ret
	}
	
	/// NVVE_DISABLE_SPS_PPS
	#[inline]
	fn disable_spspps(&self) -> i32 {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_getPropDisableSPSPPS_const(self.as_raw_EncoderParams()) };
		ret
	}
	
	/// Saves parameters to config file.
	/// 
	/// ## Parameters
	/// * configFile: Config file name.
	#[inline]
	fn save(&self, config_file: &str) -> Result<()> {
		extern_container_arg!(config_file);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_EncoderParams_save_const_const_StringR(self.as_raw_EncoderParams(), config_file.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait EncoderParamsTrait: crate::cudacodec::EncoderParamsTraitConst {
	fn as_raw_mut_EncoderParams(&mut self) -> *mut c_void;

	/// NVVE_P_INTERVAL,
	#[inline]
	fn set_p_interval(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_setPropP_Interval_int(self.as_raw_mut_EncoderParams(), val) };
		ret
	}
	
	/// NVVE_IDR_PERIOD,
	#[inline]
	fn set_idr_period(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_setPropIDR_Period_int(self.as_raw_mut_EncoderParams(), val) };
		ret
	}
	
	/// NVVE_DYNAMIC_GOP,
	#[inline]
	fn set_dynamic_gop(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_setPropDynamicGOP_int(self.as_raw_mut_EncoderParams(), val) };
		ret
	}
	
	/// NVVE_RC_TYPE,
	#[inline]
	fn set_rc_type(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_setPropRCType_int(self.as_raw_mut_EncoderParams(), val) };
		ret
	}
	
	/// NVVE_AVG_BITRATE,
	#[inline]
	fn set_avg_bitrate(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_setPropAvgBitrate_int(self.as_raw_mut_EncoderParams(), val) };
		ret
	}
	
	/// NVVE_PEAK_BITRATE,
	#[inline]
	fn set_peak_bitrate(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_setPropPeakBitrate_int(self.as_raw_mut_EncoderParams(), val) };
		ret
	}
	
	/// NVVE_QP_LEVEL_INTRA,
	#[inline]
	fn set_qp_level_intra(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_setPropQP_Level_Intra_int(self.as_raw_mut_EncoderParams(), val) };
		ret
	}
	
	/// NVVE_QP_LEVEL_INTER_P,
	#[inline]
	fn set_qp_level_inter_p(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_setPropQP_Level_InterP_int(self.as_raw_mut_EncoderParams(), val) };
		ret
	}
	
	/// NVVE_QP_LEVEL_INTER_B,
	#[inline]
	fn set_qp_level_inter_b(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_setPropQP_Level_InterB_int(self.as_raw_mut_EncoderParams(), val) };
		ret
	}
	
	/// NVVE_DEBLOCK_MODE,
	#[inline]
	fn set_deblock_mode(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_setPropDeblockMode_int(self.as_raw_mut_EncoderParams(), val) };
		ret
	}
	
	/// NVVE_PROFILE_LEVEL,
	#[inline]
	fn set_profile_level(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_setPropProfileLevel_int(self.as_raw_mut_EncoderParams(), val) };
		ret
	}
	
	/// NVVE_FORCE_INTRA,
	#[inline]
	fn set_force_intra(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_setPropForceIntra_int(self.as_raw_mut_EncoderParams(), val) };
		ret
	}
	
	/// NVVE_FORCE_IDR,
	#[inline]
	fn set_force_idr(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_setPropForceIDR_int(self.as_raw_mut_EncoderParams(), val) };
		ret
	}
	
	/// NVVE_CLEAR_STAT,
	#[inline]
	fn set_clear_stat(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_setPropClearStat_int(self.as_raw_mut_EncoderParams(), val) };
		ret
	}
	
	/// NVVE_SET_DEINTERLACE,
	#[inline]
	fn set_di_mode(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_setPropDIMode_int(self.as_raw_mut_EncoderParams(), val) };
		ret
	}
	
	/// NVVE_PRESETS,
	#[inline]
	fn set_presets(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_setPropPresets_int(self.as_raw_mut_EncoderParams(), val) };
		ret
	}
	
	/// NVVE_DISABLE_CABAC,
	#[inline]
	fn set_disable_cabac(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_setPropDisableCabac_int(self.as_raw_mut_EncoderParams(), val) };
		ret
	}
	
	/// NVVE_CONFIGURE_NALU_FRAMING_TYPE
	#[inline]
	fn set_nalu_framing_type(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_setPropNaluFramingType_int(self.as_raw_mut_EncoderParams(), val) };
		ret
	}
	
	/// NVVE_DISABLE_SPS_PPS
	#[inline]
	fn set_disable_spspps(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cudacodec_EncoderParams_setPropDisableSPSPPS_int(self.as_raw_mut_EncoderParams(), val) };
		ret
	}
	
	/// Reads parameters from config file.
	/// 
	/// ## Parameters
	/// * configFile: Config file name.
	#[inline]
	fn load(&mut self, config_file: &str) -> Result<()> {
		extern_container_arg!(config_file);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_EncoderParams_load_const_StringR(self.as_raw_mut_EncoderParams(), config_file.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Different parameters for CUDA video encoder.
pub struct EncoderParams {
	ptr: *mut c_void
}

opencv_type_boxed! { EncoderParams }

impl Drop for EncoderParams {
	fn drop(&mut self) {
		extern "C" { fn cv_EncoderParams_delete(instance: *mut c_void); }
		unsafe { cv_EncoderParams_delete(self.as_raw_mut_EncoderParams()) };
	}
}

unsafe impl Send for EncoderParams {}

impl crate::cudacodec::EncoderParamsTraitConst for EncoderParams {
	#[inline] fn as_raw_EncoderParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::cudacodec::EncoderParamsTrait for EncoderParams {
	#[inline] fn as_raw_mut_EncoderParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl EncoderParams {
	#[inline]
	pub fn default() -> Result<crate::cudacodec::EncoderParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_EncoderParams_EncoderParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::cudacodec::EncoderParams::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Constructors.
	/// 
	/// ## Parameters
	/// * configFile: Config file name.
	/// 
	/// Creates default parameters or reads parameters from config file.
	#[inline]
	pub fn new(config_file: &str) -> Result<crate::cudacodec::EncoderParams> {
		extern_container_arg!(config_file);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_EncoderParams_EncoderParams_const_StringR(config_file.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::cudacodec::EncoderParams::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

/// Struct providing information about video file format. :
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FormatInfo {
	pub codec: crate::cudacodec::Codec,
	pub chroma_format: crate::cudacodec::ChromaFormat,
	pub n_bit_depth_minus8: i32,
	/// Coded sequence width in pixels.
	pub ul_width: i32,
	/// Coded sequence height in pixels.
	pub ul_height: i32,
	/// Width of the decoded frame returned by nextFrame(frame).
	pub width: i32,
	/// Height of the decoded frame returned by nextFrame(frame).
	pub height: i32,
	pub ul_max_width: i32,
	pub ul_max_height: i32,
	/// ROI inside the decoded frame returned by nextFrame(frame), containing the useable video frame.
	pub display_area: core::Rect,
	pub valid: bool,
	pub fps: f64,
	/// Maximum number of internal decode surfaces.
	pub ul_num_decode_surfaces: i32,
	pub deinterlace_mode: crate::cudacodec::DeinterlaceMode,
}

opencv_type_simple! { crate::cudacodec::FormatInfo }

impl FormatInfo {
	#[inline]
	pub fn default() -> Result<crate::cudacodec::FormatInfo> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_FormatInfo_FormatInfo(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Interface for video demultiplexing. :
/// 
/// User can implement own demultiplexing by implementing this interface.
pub trait RawVideoSourceConst {
	fn as_raw_RawVideoSource(&self) -> *const c_void;

	/// Returns true if the last packet contained a key frame.
	#[inline]
	fn last_packet_contains_key_frame(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_RawVideoSource_lastPacketContainsKeyFrame_const(self.as_raw_RawVideoSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns information about video file format.
	#[inline]
	fn format(&self) -> Result<crate::cudacodec::FormatInfo> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_RawVideoSource_format_const(self.as_raw_RawVideoSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns any extra data associated with the video source.
	/// 
	/// ## Parameters
	/// * extraData: 1D cv::Mat containing the extra data if it exists.
	#[inline]
	fn get_extra_data(&self, extra_data: &mut core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_RawVideoSource_getExtraData_const_MatR(self.as_raw_RawVideoSource(), extra_data.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Retrieves the specified property used by the VideoSource.
	/// 
	/// ## Parameters
	/// * propertyId: Property identifier from cv::VideoCaptureProperties (eg. cv::CAP_PROP_POS_MSEC, cv::CAP_PROP_POS_FRAMES, ...)
	/// or one from @ref videoio_flags_others.
	/// * propertyVal: Value for the specified property.
	/// 
	/// ## Returns
	/// `true` unless the property is unset set or not supported.
	#[inline]
	fn get(&self, property_id: i32, property_val: &mut f64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_RawVideoSource_get_const_const_int_doubleR(self.as_raw_RawVideoSource(), property_id, property_val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait RawVideoSource: crate::cudacodec::RawVideoSourceConst {
	fn as_raw_mut_RawVideoSource(&mut self) -> *mut c_void;

	/// Returns next packet with RAW video frame.
	/// 
	/// ## Parameters
	/// * data: Pointer to frame data.
	/// * size: Size in bytes of current frame.
	#[inline]
	fn get_next_packet(&mut self, data: &mut &mut u8, size: &mut size_t) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_RawVideoSource_getNextPacket_unsigned_charXX_size_tX(self.as_raw_mut_RawVideoSource(), data as *mut  _ as *mut  *mut  _, size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Updates the coded width and height inside format.
	#[inline]
	fn update_format(&mut self, video_format: crate::cudacodec::FormatInfo) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_RawVideoSource_updateFormat_const_FormatInfoR(self.as_raw_mut_RawVideoSource(), &video_format, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Video reader interface.
/// 
/// 
/// Note:
///    *   An example on how to use the videoReader class can be found at
///        opencv_source_code/samples/gpu/video_reader.cpp
pub trait VideoReaderConst {
	fn as_raw_VideoReader(&self) -> *const c_void;

	/// Returns information about video file format.
	#[inline]
	fn format(&self) -> Result<crate::cudacodec::FormatInfo> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_VideoReader_format_const(self.as_raw_VideoReader(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns previously grabbed video data.
	/// 
	/// ## Parameters
	/// * frame:[out] The returned data which depends on the provided idx.
	/// * idx: Determines the returned data inside image. The returned data can be the:
	///  - Decoded frame, idx = get(PROP_DECODED_FRAME_IDX).
	///  - Extra data if available, idx = get(PROP_EXTRA_DATA_INDEX).
	///  - Raw encoded data package.  To retrieve package i,  idx = get(PROP_RAW_PACKAGES_BASE_INDEX) + i with i < get(PROP_NUMBER_OF_RAW_PACKAGES_SINCE_LAST_GRAB)
	/// ## Returns
	/// `false` if no frames have been grabbed
	/// 
	/// The method returns data associated with the current video source since the last call to grab() or the creation of the VideoReader. If no data is present
	/// the method returns false and the function returns an empty image.
	/// 
	/// ## C++ default parameters
	/// * idx: static_cast<size_t>(VideoReaderProps::PROP_DECODED_FRAME_IDX)
	#[inline]
	fn retrieve(&self, frame: &mut dyn core::ToOutputArray, idx: size_t) -> Result<bool> {
		output_array_arg!(frame);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_VideoReader_retrieve_const_const__OutputArrayR_const_size_t(self.as_raw_VideoReader(), frame.as_raw__OutputArray(), idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns previously grabbed encoded video data.
	/// 
	/// ## Parameters
	/// * frame:[out] The encoded video data.
	/// * idx: Determines the returned data inside image. The returned data can be the:
	///  - Extra data if available, idx = get(PROP_EXTRA_DATA_INDEX).
	///  - Raw encoded data package.  To retrieve package i,  idx = get(PROP_RAW_PACKAGES_BASE_INDEX) + i with i < get(PROP_NUMBER_OF_RAW_PACKAGES_SINCE_LAST_GRAB)
	/// ## Returns
	/// `false` if no frames have been grabbed
	/// 
	/// The method returns data associated with the current video source since the last call to grab() or the creation of the VideoReader. If no data is present
	/// the method returns false and the function returns an empty image.
	#[inline]
	fn retrieve_1(&self, frame: &mut core::Mat, idx: size_t) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_VideoReader_retrieve_const_MatR_const_size_t(self.as_raw_VideoReader(), frame.as_raw_mut_Mat(), idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns the next video frame.
	/// 
	/// ## Parameters
	/// * frame:[out] The video frame.  If grab() has not been called then this will be empty().
	/// ## Returns
	/// `false` if no frames have been grabbed
	/// 
	/// The method returns data associated with the current video source since the last call to grab(). If no data is present
	/// the method returns false and the function returns an empty image.
	#[inline]
	fn retrieve_2(&self, frame: &mut core::GpuMat) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_VideoReader_retrieve_const_GpuMatR(self.as_raw_VideoReader(), frame.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns the specified VideoReader property
	/// 
	/// ## Parameters
	/// * propertyId: Property identifier from cv::cudacodec::VideoReaderProps (eg. cv::cudacodec::VideoReaderProps::PROP_DECODED_FRAME_IDX,
	/// cv::cudacodec::VideoReaderProps::PROP_EXTRA_DATA_INDEX, ...).
	/// * propertyVal: 
	///  - In: Optional value required for querying specific propertyId's, e.g. the index of the raw package to be checked for a key frame (cv::cudacodec::VideoReaderProps::PROP_LRF_HAS_KEY_FRAME).
	///  - Out: Value of the property.
	/// ## Returns
	/// `true` unless the property is not supported.
	#[inline]
	fn get(&self, property_id: crate::cudacodec::VideoReaderProps, property_val: &mut f64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_VideoReader_get_const_const_VideoReaderProps_doubleR(self.as_raw_VideoReader(), property_id, property_val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * property_val_in: 0
	#[inline]
	fn get_video_reader_props(&self, property_id: crate::cudacodec::VideoReaderProps, property_val_out: &mut f64, property_val_in: f64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_VideoReader_getVideoReaderProps_const_const_VideoReaderProps_doubleR_double(self.as_raw_VideoReader(), property_id, property_val_out, property_val_in, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Retrieves the specified property used by the VideoSource.
	/// 
	/// ## Parameters
	/// * propertyId: Property identifier from cv::VideoCaptureProperties (eg. cv::CAP_PROP_POS_MSEC, cv::CAP_PROP_POS_FRAMES, ...)
	/// or one from @ref videoio_flags_others.
	/// * propertyVal: Value for the specified property.
	/// 
	/// ## Returns
	/// `true` unless the property is unset set or not supported.
	#[inline]
	fn get_1(&self, property_id: i32, property_val: &mut f64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_VideoReader_get_const_const_int_doubleR(self.as_raw_VideoReader(), property_id, property_val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait VideoReader: crate::cudacodec::VideoReaderConst {
	fn as_raw_mut_VideoReader(&mut self) -> *mut c_void;

	/// Grabs, decodes and returns the next video frame.
	/// 
	/// ## Parameters
	/// * frame:[out] The video frame.
	/// * stream: Stream for the asynchronous version.
	/// ## Returns
	/// `false` if no frames have been grabbed.
	/// 
	/// If no frames have been grabbed (there are no more frames in video file), the methods return false.
	/// The method throws an Exception if error occurs.
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	fn next_frame(&mut self, frame: &mut core::GpuMat, stream: &mut core::Stream) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_VideoReader_nextFrame_GpuMatR_StreamR(self.as_raw_mut_VideoReader(), frame.as_raw_mut_GpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Grabs the next frame from the video source.
	/// 
	/// ## Parameters
	/// * stream: Stream for the asynchronous version.
	/// ## Returns
	/// `true` (non-zero) in the case of success.
	/// 
	/// The method/function grabs the next frame from video file or camera and returns true (non-zero) in
	/// the case of success.
	/// 
	/// The primary use of the function is for reading both the encoded and decoded video data when rawMode is enabled.  With rawMode enabled
	/// retrieve() can be called following grab() to retrieve all the data associated with the current video source since the last call to grab() or the creation of the VideoReader.
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	#[inline]
	fn grab(&mut self, stream: &mut core::Stream) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_VideoReader_grab_StreamR(self.as_raw_mut_VideoReader(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Sets a property in the VideoReader.
	/// 
	/// ## Parameters
	/// * propertyId: Property identifier from cv::cudacodec::VideoReaderProps (eg. cv::cudacodec::VideoReaderProps::PROP_DECODED_FRAME_IDX,
	/// cv::cudacodec::VideoReaderProps::PROP_EXTRA_DATA_INDEX, ...).
	/// * propertyVal: Value of the property.
	/// ## Returns
	/// `true` if the property has been set.
	#[inline]
	fn set(&mut self, property_id: crate::cudacodec::VideoReaderProps, property_val: f64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_VideoReader_set_const_VideoReaderProps_const_double(self.as_raw_mut_VideoReader(), property_id, property_val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_video_reader_props(&mut self, property_id: crate::cudacodec::VideoReaderProps, property_val: f64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_VideoReader_setVideoReaderProps_const_VideoReaderProps_double(self.as_raw_mut_VideoReader(), property_id, property_val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Set the desired ColorFormat for the frame returned by nextFrame()/retrieve().
	/// 
	/// ## Parameters
	/// * colorFormat: Value of the ColorFormat.
	#[inline]
	fn set_1(&mut self, color_format: crate::cudacodec::ColorFormat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_VideoReader_set_const_ColorFormat(self.as_raw_mut_VideoReader(), color_format, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// VideoReader initialization parameters
/// ## Parameters
/// * udpSource: Remove validation which can cause VideoReader() to throw exceptions when reading from a UDP source.
/// * allowFrameDrop: Allow frames to be dropped when ingesting from a live capture source to prevent delay and eventual disconnection
/// when calls to nextFrame()/grab() cannot keep up with the source's fps.  Only use if delay and disconnection are a problem, i.e. not when decoding from
/// video files where setting this flag will cause frames to be unnecessarily discarded.
/// * minNumDecodeSurfaces: Minimum number of internal decode surfaces used by the hardware decoder.  NVDEC will automatically determine the minimum number of
/// surfaces it requires for correct functionality and optimal video memory usage but not necessarily for best performance, which depends on the design of the
/// overall application. The optimal number of decode surfaces (in terms of performance and memory utilization) should be decided by experimentation for each application,
/// but it cannot go below the number determined by NVDEC.
/// * rawMode: Allow the raw encoded data which has been read up until the last call to grab() to be retrieved by calling retrieve(rawData,RAW_DATA_IDX).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VideoReaderInitParams {
	pub udp_source: bool,
	pub allow_frame_drop: bool,
	pub min_num_decode_surfaces: i32,
	pub raw_mode: bool,
}

opencv_type_simple! { crate::cudacodec::VideoReaderInitParams }

impl VideoReaderInitParams {
	#[inline]
	pub fn default() -> Result<crate::cudacodec::VideoReaderInitParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_VideoReaderInitParams_VideoReaderInitParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Video writer interface.
/// 
/// The implementation uses H264 video codec.
/// 
/// 
/// Note: Currently only Windows platform is supported.
/// 
/// 
/// Note:
///    *   An example on how to use the videoWriter class can be found at
///        opencv_source_code/samples/gpu/video_writer.cpp
pub trait VideoWriterConst {
	fn as_raw_VideoWriter(&self) -> *const c_void;

	#[inline]
	fn get_encoder_params(&self) -> Result<crate::cudacodec::EncoderParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_VideoWriter_getEncoderParams_const(self.as_raw_VideoWriter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::cudacodec::EncoderParams::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait VideoWriter: crate::cudacodec::VideoWriterConst {
	fn as_raw_mut_VideoWriter(&mut self) -> *mut c_void;

	/// Writes the next video frame.
	/// 
	/// ## Parameters
	/// * frame: The written frame.
	/// * lastFrame: Indicates that it is end of stream. The parameter can be ignored.
	/// 
	/// The method write the specified image to video file. The image must have the same size and the same
	/// surface format as has been specified when opening the video writer.
	/// 
	/// ## C++ default parameters
	/// * last_frame: false
	#[inline]
	fn write(&mut self, frame: &dyn core::ToInputArray, last_frame: bool) -> Result<()> {
		input_array_arg!(frame);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_VideoWriter_write_const__InputArrayR_bool(self.as_raw_mut_VideoWriter(), frame.as_raw__InputArray(), last_frame, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}
