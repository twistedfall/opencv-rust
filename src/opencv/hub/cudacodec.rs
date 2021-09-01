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
pub const YUV420: i32 = 1;
pub const YUV422: i32 = 2;
pub const YUV444: i32 = 3;
/// Chroma formats supported by cudacodec::VideoReader .
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

/// Creates video reader.
/// 
/// ## Parameters
/// * filename: Name of the input video file.
/// 
/// FFMPEG is used to read videos. User can implement own demultiplexing with cudacodec::RawVideoSource
/// 
/// ## Overloaded parameters
/// 
/// * source: RAW video source implemented by user.
pub fn create_video_reader_1(source: &core::Ptr<dyn crate::cudacodec::RawVideoSource>) -> Result<core::Ptr<dyn crate::cudacodec::VideoReader>> {
	unsafe { sys::cv_cudacodec_createVideoReader_const_Ptr_RawVideoSource_R(source.as_raw_PtrOfRawVideoSource()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudacodec::VideoReader>::opencv_from_extern(r) } )
}

/// Creates video reader.
/// 
/// ## Parameters
/// * filename: Name of the input video file.
/// 
/// FFMPEG is used to read videos. User can implement own demultiplexing with cudacodec::RawVideoSource
pub fn create_video_reader(filename: &str) -> Result<core::Ptr<dyn crate::cudacodec::VideoReader>> {
	extern_container_arg!(filename);
	unsafe { sys::cv_cudacodec_createVideoReader_const_StringR(filename.opencv_as_extern()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudacodec::VideoReader>::opencv_from_extern(r) } )
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
pub fn create_video_writer_2(encoder_callback: &core::Ptr<dyn crate::cudacodec::EncoderCallBack>, frame_size: core::Size, fps: f64, format: crate::cudacodec::SurfaceFormat) -> Result<core::Ptr<dyn crate::cudacodec::VideoWriter>> {
	unsafe { sys::cv_cudacodec_createVideoWriter_const_Ptr_EncoderCallBack_R_Size_double_SurfaceFormat(encoder_callback.as_raw_PtrOfEncoderCallBack(), frame_size.opencv_as_extern(), fps, format) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudacodec::VideoWriter>::opencv_from_extern(r) } )
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
/// * params: Encoder parameters. See cudacodec::EncoderParams .
/// * format: Surface format of input frames ( SF_UYVY , SF_YUY2 , SF_YV12 , SF_NV12 ,
/// SF_IYUV , SF_BGR or SF_GRAY). BGR or gray frames will be converted to YV12 format before
/// encoding, frames with other formats will be used as is.
/// 
/// ## C++ default parameters
/// * format: SF_BGR
pub fn create_video_writer_3(encoder_callback: &core::Ptr<dyn crate::cudacodec::EncoderCallBack>, frame_size: core::Size, fps: f64, params: &crate::cudacodec::EncoderParams, format: crate::cudacodec::SurfaceFormat) -> Result<core::Ptr<dyn crate::cudacodec::VideoWriter>> {
	unsafe { sys::cv_cudacodec_createVideoWriter_const_Ptr_EncoderCallBack_R_Size_double_const_EncoderParamsR_SurfaceFormat(encoder_callback.as_raw_PtrOfEncoderCallBack(), frame_size.opencv_as_extern(), fps, params.as_raw_EncoderParams(), format) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudacodec::VideoWriter>::opencv_from_extern(r) } )
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
pub fn create_video_writer(file_name: &str, frame_size: core::Size, fps: f64, format: crate::cudacodec::SurfaceFormat) -> Result<core::Ptr<dyn crate::cudacodec::VideoWriter>> {
	extern_container_arg!(file_name);
	unsafe { sys::cv_cudacodec_createVideoWriter_const_StringR_Size_double_SurfaceFormat(file_name.opencv_as_extern(), frame_size.opencv_as_extern(), fps, format) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudacodec::VideoWriter>::opencv_from_extern(r) } )
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
pub fn create_video_writer_1(file_name: &str, frame_size: core::Size, fps: f64, params: &crate::cudacodec::EncoderParams, format: crate::cudacodec::SurfaceFormat) -> Result<core::Ptr<dyn crate::cudacodec::VideoWriter>> {
	extern_container_arg!(file_name);
	unsafe { sys::cv_cudacodec_createVideoWriter_const_StringR_Size_double_const_EncoderParamsR_SurfaceFormat(file_name.opencv_as_extern(), frame_size.opencv_as_extern(), fps, params.as_raw_EncoderParams(), format) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::cudacodec::VideoWriter>::opencv_from_extern(r) } )
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
	fn acquire_bit_stream(&mut self, buffer_size: &mut i32) -> Result<&mut u8> {
		unsafe { sys::cv_cudacodec_EncoderCallBack_acquireBitStream_intX(self.as_raw_mut_EncoderCallBack(), buffer_size) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string())))
	}
	
	/// Callback function to signal that the encoded bitstream is ready to be written to file.
	fn release_bit_stream(&mut self, data: &mut u8, size: i32) -> Result<()> {
		unsafe { sys::cv_cudacodec_EncoderCallBack_releaseBitStream_unsigned_charX_int(self.as_raw_mut_EncoderCallBack(), data, size) }.into_result()
	}
	
	/// Callback function to signal that the encoding operation on the frame has started.
	/// 
	/// ## Parameters
	/// * frameNumber: 
	/// * picType: Specify frame type (I-Frame, P-Frame or B-Frame).
	fn on_begin_frame(&mut self, frame_number: i32, pic_type: crate::cudacodec::EncoderCallBack_PicType) -> Result<()> {
		unsafe { sys::cv_cudacodec_EncoderCallBack_onBeginFrame_int_PicType(self.as_raw_mut_EncoderCallBack(), frame_number, pic_type) }.into_result()
	}
	
	/// Callback function signals that the encoding operation on the frame has finished.
	/// 
	/// ## Parameters
	/// * frameNumber: 
	/// * picType: Specify frame type (I-Frame, P-Frame or B-Frame).
	fn on_end_frame(&mut self, frame_number: i32, pic_type: crate::cudacodec::EncoderCallBack_PicType) -> Result<()> {
		unsafe { sys::cv_cudacodec_EncoderCallBack_onEndFrame_int_PicType(self.as_raw_mut_EncoderCallBack(), frame_number, pic_type) }.into_result()
	}
	
}

/// Different parameters for CUDA video encoder.
pub trait EncoderParamsTraitConst {
	fn as_raw_EncoderParams(&self) -> *const c_void;

	/// NVVE_P_INTERVAL,
	fn p_interval(&self) -> i32 {
		unsafe { sys::cv_cudacodec_EncoderParams_getPropP_Interval_const(self.as_raw_EncoderParams()) }.into_result().expect("Infallible function failed: p_interval")
	}
	
	/// NVVE_IDR_PERIOD,
	fn idr_period(&self) -> i32 {
		unsafe { sys::cv_cudacodec_EncoderParams_getPropIDR_Period_const(self.as_raw_EncoderParams()) }.into_result().expect("Infallible function failed: idr_period")
	}
	
	/// NVVE_DYNAMIC_GOP,
	fn dynamic_gop(&self) -> i32 {
		unsafe { sys::cv_cudacodec_EncoderParams_getPropDynamicGOP_const(self.as_raw_EncoderParams()) }.into_result().expect("Infallible function failed: dynamic_gop")
	}
	
	/// NVVE_RC_TYPE,
	fn rc_type(&self) -> i32 {
		unsafe { sys::cv_cudacodec_EncoderParams_getPropRCType_const(self.as_raw_EncoderParams()) }.into_result().expect("Infallible function failed: rc_type")
	}
	
	/// NVVE_AVG_BITRATE,
	fn avg_bitrate(&self) -> i32 {
		unsafe { sys::cv_cudacodec_EncoderParams_getPropAvgBitrate_const(self.as_raw_EncoderParams()) }.into_result().expect("Infallible function failed: avg_bitrate")
	}
	
	/// NVVE_PEAK_BITRATE,
	fn peak_bitrate(&self) -> i32 {
		unsafe { sys::cv_cudacodec_EncoderParams_getPropPeakBitrate_const(self.as_raw_EncoderParams()) }.into_result().expect("Infallible function failed: peak_bitrate")
	}
	
	/// NVVE_QP_LEVEL_INTRA,
	fn qp_level_intra(&self) -> i32 {
		unsafe { sys::cv_cudacodec_EncoderParams_getPropQP_Level_Intra_const(self.as_raw_EncoderParams()) }.into_result().expect("Infallible function failed: qp_level_intra")
	}
	
	/// NVVE_QP_LEVEL_INTER_P,
	fn qp_level_inter_p(&self) -> i32 {
		unsafe { sys::cv_cudacodec_EncoderParams_getPropQP_Level_InterP_const(self.as_raw_EncoderParams()) }.into_result().expect("Infallible function failed: qp_level_inter_p")
	}
	
	/// NVVE_QP_LEVEL_INTER_B,
	fn qp_level_inter_b(&self) -> i32 {
		unsafe { sys::cv_cudacodec_EncoderParams_getPropQP_Level_InterB_const(self.as_raw_EncoderParams()) }.into_result().expect("Infallible function failed: qp_level_inter_b")
	}
	
	/// NVVE_DEBLOCK_MODE,
	fn deblock_mode(&self) -> i32 {
		unsafe { sys::cv_cudacodec_EncoderParams_getPropDeblockMode_const(self.as_raw_EncoderParams()) }.into_result().expect("Infallible function failed: deblock_mode")
	}
	
	/// NVVE_PROFILE_LEVEL,
	fn profile_level(&self) -> i32 {
		unsafe { sys::cv_cudacodec_EncoderParams_getPropProfileLevel_const(self.as_raw_EncoderParams()) }.into_result().expect("Infallible function failed: profile_level")
	}
	
	/// NVVE_FORCE_INTRA,
	fn force_intra(&self) -> i32 {
		unsafe { sys::cv_cudacodec_EncoderParams_getPropForceIntra_const(self.as_raw_EncoderParams()) }.into_result().expect("Infallible function failed: force_intra")
	}
	
	/// NVVE_FORCE_IDR,
	fn force_idr(&self) -> i32 {
		unsafe { sys::cv_cudacodec_EncoderParams_getPropForceIDR_const(self.as_raw_EncoderParams()) }.into_result().expect("Infallible function failed: force_idr")
	}
	
	/// NVVE_CLEAR_STAT,
	fn clear_stat(&self) -> i32 {
		unsafe { sys::cv_cudacodec_EncoderParams_getPropClearStat_const(self.as_raw_EncoderParams()) }.into_result().expect("Infallible function failed: clear_stat")
	}
	
	/// NVVE_SET_DEINTERLACE,
	fn di_mode(&self) -> i32 {
		unsafe { sys::cv_cudacodec_EncoderParams_getPropDIMode_const(self.as_raw_EncoderParams()) }.into_result().expect("Infallible function failed: di_mode")
	}
	
	/// NVVE_PRESETS,
	fn presets(&self) -> i32 {
		unsafe { sys::cv_cudacodec_EncoderParams_getPropPresets_const(self.as_raw_EncoderParams()) }.into_result().expect("Infallible function failed: presets")
	}
	
	/// NVVE_DISABLE_CABAC,
	fn disable_cabac(&self) -> i32 {
		unsafe { sys::cv_cudacodec_EncoderParams_getPropDisableCabac_const(self.as_raw_EncoderParams()) }.into_result().expect("Infallible function failed: disable_cabac")
	}
	
	/// NVVE_CONFIGURE_NALU_FRAMING_TYPE
	fn nalu_framing_type(&self) -> i32 {
		unsafe { sys::cv_cudacodec_EncoderParams_getPropNaluFramingType_const(self.as_raw_EncoderParams()) }.into_result().expect("Infallible function failed: nalu_framing_type")
	}
	
	/// NVVE_DISABLE_SPS_PPS
	fn disable_spspps(&self) -> i32 {
		unsafe { sys::cv_cudacodec_EncoderParams_getPropDisableSPSPPS_const(self.as_raw_EncoderParams()) }.into_result().expect("Infallible function failed: disable_spspps")
	}
	
	/// Saves parameters to config file.
	/// 
	/// ## Parameters
	/// * configFile: Config file name.
	fn save(&self, config_file: &str) -> Result<()> {
		extern_container_arg!(config_file);
		unsafe { sys::cv_cudacodec_EncoderParams_save_const_const_StringR(self.as_raw_EncoderParams(), config_file.opencv_as_extern()) }.into_result()
	}
	
}

pub trait EncoderParamsTrait: crate::cudacodec::EncoderParamsTraitConst {
	fn as_raw_mut_EncoderParams(&mut self) -> *mut c_void;

	/// NVVE_P_INTERVAL,
	fn set_p_interval(&mut self, val: i32) -> () {
		unsafe { sys::cv_cudacodec_EncoderParams_setPropP_Interval_int(self.as_raw_mut_EncoderParams(), val) }.into_result().expect("Infallible function failed: set_p_interval")
	}
	
	/// NVVE_IDR_PERIOD,
	fn set_idr_period(&mut self, val: i32) -> () {
		unsafe { sys::cv_cudacodec_EncoderParams_setPropIDR_Period_int(self.as_raw_mut_EncoderParams(), val) }.into_result().expect("Infallible function failed: set_idr_period")
	}
	
	/// NVVE_DYNAMIC_GOP,
	fn set_dynamic_gop(&mut self, val: i32) -> () {
		unsafe { sys::cv_cudacodec_EncoderParams_setPropDynamicGOP_int(self.as_raw_mut_EncoderParams(), val) }.into_result().expect("Infallible function failed: set_dynamic_gop")
	}
	
	/// NVVE_RC_TYPE,
	fn set_rc_type(&mut self, val: i32) -> () {
		unsafe { sys::cv_cudacodec_EncoderParams_setPropRCType_int(self.as_raw_mut_EncoderParams(), val) }.into_result().expect("Infallible function failed: set_rc_type")
	}
	
	/// NVVE_AVG_BITRATE,
	fn set_avg_bitrate(&mut self, val: i32) -> () {
		unsafe { sys::cv_cudacodec_EncoderParams_setPropAvgBitrate_int(self.as_raw_mut_EncoderParams(), val) }.into_result().expect("Infallible function failed: set_avg_bitrate")
	}
	
	/// NVVE_PEAK_BITRATE,
	fn set_peak_bitrate(&mut self, val: i32) -> () {
		unsafe { sys::cv_cudacodec_EncoderParams_setPropPeakBitrate_int(self.as_raw_mut_EncoderParams(), val) }.into_result().expect("Infallible function failed: set_peak_bitrate")
	}
	
	/// NVVE_QP_LEVEL_INTRA,
	fn set_qp_level_intra(&mut self, val: i32) -> () {
		unsafe { sys::cv_cudacodec_EncoderParams_setPropQP_Level_Intra_int(self.as_raw_mut_EncoderParams(), val) }.into_result().expect("Infallible function failed: set_qp_level_intra")
	}
	
	/// NVVE_QP_LEVEL_INTER_P,
	fn set_qp_level_inter_p(&mut self, val: i32) -> () {
		unsafe { sys::cv_cudacodec_EncoderParams_setPropQP_Level_InterP_int(self.as_raw_mut_EncoderParams(), val) }.into_result().expect("Infallible function failed: set_qp_level_inter_p")
	}
	
	/// NVVE_QP_LEVEL_INTER_B,
	fn set_qp_level_inter_b(&mut self, val: i32) -> () {
		unsafe { sys::cv_cudacodec_EncoderParams_setPropQP_Level_InterB_int(self.as_raw_mut_EncoderParams(), val) }.into_result().expect("Infallible function failed: set_qp_level_inter_b")
	}
	
	/// NVVE_DEBLOCK_MODE,
	fn set_deblock_mode(&mut self, val: i32) -> () {
		unsafe { sys::cv_cudacodec_EncoderParams_setPropDeblockMode_int(self.as_raw_mut_EncoderParams(), val) }.into_result().expect("Infallible function failed: set_deblock_mode")
	}
	
	/// NVVE_PROFILE_LEVEL,
	fn set_profile_level(&mut self, val: i32) -> () {
		unsafe { sys::cv_cudacodec_EncoderParams_setPropProfileLevel_int(self.as_raw_mut_EncoderParams(), val) }.into_result().expect("Infallible function failed: set_profile_level")
	}
	
	/// NVVE_FORCE_INTRA,
	fn set_force_intra(&mut self, val: i32) -> () {
		unsafe { sys::cv_cudacodec_EncoderParams_setPropForceIntra_int(self.as_raw_mut_EncoderParams(), val) }.into_result().expect("Infallible function failed: set_force_intra")
	}
	
	/// NVVE_FORCE_IDR,
	fn set_force_idr(&mut self, val: i32) -> () {
		unsafe { sys::cv_cudacodec_EncoderParams_setPropForceIDR_int(self.as_raw_mut_EncoderParams(), val) }.into_result().expect("Infallible function failed: set_force_idr")
	}
	
	/// NVVE_CLEAR_STAT,
	fn set_clear_stat(&mut self, val: i32) -> () {
		unsafe { sys::cv_cudacodec_EncoderParams_setPropClearStat_int(self.as_raw_mut_EncoderParams(), val) }.into_result().expect("Infallible function failed: set_clear_stat")
	}
	
	/// NVVE_SET_DEINTERLACE,
	fn set_di_mode(&mut self, val: i32) -> () {
		unsafe { sys::cv_cudacodec_EncoderParams_setPropDIMode_int(self.as_raw_mut_EncoderParams(), val) }.into_result().expect("Infallible function failed: set_di_mode")
	}
	
	/// NVVE_PRESETS,
	fn set_presets(&mut self, val: i32) -> () {
		unsafe { sys::cv_cudacodec_EncoderParams_setPropPresets_int(self.as_raw_mut_EncoderParams(), val) }.into_result().expect("Infallible function failed: set_presets")
	}
	
	/// NVVE_DISABLE_CABAC,
	fn set_disable_cabac(&mut self, val: i32) -> () {
		unsafe { sys::cv_cudacodec_EncoderParams_setPropDisableCabac_int(self.as_raw_mut_EncoderParams(), val) }.into_result().expect("Infallible function failed: set_disable_cabac")
	}
	
	/// NVVE_CONFIGURE_NALU_FRAMING_TYPE
	fn set_nalu_framing_type(&mut self, val: i32) -> () {
		unsafe { sys::cv_cudacodec_EncoderParams_setPropNaluFramingType_int(self.as_raw_mut_EncoderParams(), val) }.into_result().expect("Infallible function failed: set_nalu_framing_type")
	}
	
	/// NVVE_DISABLE_SPS_PPS
	fn set_disable_spspps(&mut self, val: i32) -> () {
		unsafe { sys::cv_cudacodec_EncoderParams_setPropDisableSPSPPS_int(self.as_raw_mut_EncoderParams(), val) }.into_result().expect("Infallible function failed: set_disable_spspps")
	}
	
	/// Reads parameters from config file.
	/// 
	/// ## Parameters
	/// * configFile: Config file name.
	fn load(&mut self, config_file: &str) -> Result<()> {
		extern_container_arg!(config_file);
		unsafe { sys::cv_cudacodec_EncoderParams_load_const_StringR(self.as_raw_mut_EncoderParams(), config_file.opencv_as_extern()) }.into_result()
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
	pub fn default() -> Result<crate::cudacodec::EncoderParams> {
		unsafe { sys::cv_cudacodec_EncoderParams_EncoderParams() }.into_result().map(|r| unsafe { crate::cudacodec::EncoderParams::opencv_from_extern(r) } )
	}
	
	/// Constructors.
	/// 
	/// ## Parameters
	/// * configFile: Config file name.
	/// 
	/// Creates default parameters or reads parameters from config file.
	pub fn new(config_file: &str) -> Result<crate::cudacodec::EncoderParams> {
		extern_container_arg!(config_file);
		unsafe { sys::cv_cudacodec_EncoderParams_EncoderParams_const_StringR(config_file.opencv_as_extern()) }.into_result().map(|r| unsafe { crate::cudacodec::EncoderParams::opencv_from_extern(r) } )
	}
	
}

/// Struct providing information about video file format. :
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FormatInfo {
	pub codec: crate::cudacodec::Codec,
	pub chroma_format: crate::cudacodec::ChromaFormat,
	pub n_bit_depth_minus8: i32,
	pub width: i32,
	pub height: i32,
}

opencv_type_simple! { crate::cudacodec::FormatInfo }

impl FormatInfo {
}

/// Interface for video demultiplexing. :
/// 
/// User can implement own demultiplexing by implementing this interface.
pub trait RawVideoSourceConst {
	fn as_raw_RawVideoSource(&self) -> *const c_void;

	/// Returns information about video file format.
	fn format(&self) -> Result<crate::cudacodec::FormatInfo> {
		unsafe { sys::cv_cudacodec_RawVideoSource_format_const(self.as_raw_RawVideoSource()) }.into_result()
	}
	
}

pub trait RawVideoSource: crate::cudacodec::RawVideoSourceConst {
	fn as_raw_mut_RawVideoSource(&mut self) -> *mut c_void;

	/// Returns next packet with RAW video frame.
	/// 
	/// ## Parameters
	/// * data: Pointer to frame data.
	/// * size: Size in bytes of current frame.
	fn get_next_packet(&mut self, data: &mut &mut u8, size: &mut size_t) -> Result<bool> {
		unsafe { sys::cv_cudacodec_RawVideoSource_getNextPacket_unsigned_charXX_size_tX(self.as_raw_mut_RawVideoSource(), data as *mut  _ as *mut  *mut  _, size) }.into_result()
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
	fn format(&self) -> Result<crate::cudacodec::FormatInfo> {
		unsafe { sys::cv_cudacodec_VideoReader_format_const(self.as_raw_VideoReader()) }.into_result()
	}
	
}

pub trait VideoReader: crate::cudacodec::VideoReaderConst {
	fn as_raw_mut_VideoReader(&mut self) -> *mut c_void;

	/// Grabs, decodes and returns the next video frame.
	/// 
	/// If no frames has been grabbed (there are no more frames in video file), the methods return false .
	/// The method throws Exception if error occurs.
	/// 
	/// ## C++ default parameters
	/// * stream: Stream::Null()
	fn next_frame(&mut self, frame: &mut core::GpuMat, stream: &mut core::Stream) -> Result<bool> {
		unsafe { sys::cv_cudacodec_VideoReader_nextFrame_GpuMatR_StreamR(self.as_raw_mut_VideoReader(), frame.as_raw_mut_GpuMat(), stream.as_raw_mut_Stream()) }.into_result()
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

	fn get_encoder_params(&self) -> Result<crate::cudacodec::EncoderParams> {
		unsafe { sys::cv_cudacodec_VideoWriter_getEncoderParams_const(self.as_raw_VideoWriter()) }.into_result().map(|r| unsafe { crate::cudacodec::EncoderParams::opencv_from_extern(r) } )
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
	fn write(&mut self, frame: &dyn core::ToInputArray, last_frame: bool) -> Result<()> {
		input_array_arg!(frame);
		unsafe { sys::cv_cudacodec_VideoWriter_write_const__InputArrayR_bool(self.as_raw_mut_VideoWriter(), frame.as_raw__InputArray(), last_frame) }.into_result()
	}
	
}
