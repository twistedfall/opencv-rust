pub mod cudacodec {
	//! # Video Encoding/Decoding
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::CUDA_EncoderCallbackTraitConst, super::CUDA_EncoderCallbackTrait, super::CUDA_VideoWriterTraitConst, super::CUDA_VideoWriterTrait, super::CUDA_VideoReaderTraitConst, super::CUDA_VideoReaderTrait, super::CUDA_RawVideoSourceTraitConst, super::CUDA_RawVideoSourceTrait };
	}
	
	pub const CUDA_AV1: i32 = 11;
	pub const CUDA_Adaptive: i32 = 2;
	pub const CUDA_Bob: i32 = 1;
	/// OpenCV color format, can be used with both VideoReader and VideoWriter.
	pub const CUDA_ColorFormat_BGR: i32 = 2;
	/// OpenCV color format, can be used with both VideoReader and VideoWriter.
	pub const CUDA_ColorFormat_BGRA: i32 = 1;
	/// OpenCV color format, can be used with both VideoReader and VideoWriter.
	pub const CUDA_ColorFormat_GRAY: i32 = 3;
	/// Nvidia Buffer Format - 8 bit Packed A8Y8U8V8. This is a word-ordered format where a pixel is represented by a 32-bit word with V in the lowest 8 bits, U in the next 8 bits, Y in the 8 bits after that and A in the highest 8 bits, can only be used with VideoWriter.
	pub const CUDA_ColorFormat_NV_AYUV: i32 = 11;
	/// Nvidia Buffer Format - Planar YUV [Y plane followed by U and V planes], use with VideoReader, can only be used with VideoWriter.
	pub const CUDA_ColorFormat_NV_IYUV: i32 = 9;
	/// Nvidia color format - equivalent to YUV - Semi-Planar YUV [Y plane followed by interleaved UV plane], can be used with both VideoReader and VideoWriter.
	pub const CUDA_ColorFormat_NV_NV12: i32 = 4;
	/// Nvidia Buffer Format - Planar YUV [Y plane followed by U and V planes], use with VideoReader, can only be used with VideoWriter.
	pub const CUDA_ColorFormat_NV_YUV444: i32 = 10;
	/// Nvidia Buffer Format - Planar YUV [Y plane followed by V and U planes], use with VideoReader, can only be used with VideoWriter.
	pub const CUDA_ColorFormat_NV_YV12: i32 = 8;
	pub const CUDA_ColorFormat_PROP_NOT_SUPPORTED: i32 = 12;
	/// OpenCV color format, can only be used with VideoWriter.
	pub const CUDA_ColorFormat_RGB: i32 = 5;
	/// OpenCV color format, can only be used with VideoWriter.
	pub const CUDA_ColorFormat_RGBA: i32 = 6;
	pub const CUDA_ColorFormat_UNDEFINED: i32 = 0;
	pub const CUDA_ENC_CODEC_PROFILE_AUTOSELECT: i32 = 0;
	pub const CUDA_ENC_H264_PROFILE_BASELINE: i32 = 1;
	pub const CUDA_ENC_H264_PROFILE_CONSTRAINED_HIGH: i32 = 7;
	pub const CUDA_ENC_H264_PROFILE_HIGH: i32 = 3;
	pub const CUDA_ENC_H264_PROFILE_HIGH_444: i32 = 4;
	pub const CUDA_ENC_H264_PROFILE_MAIN: i32 = 2;
	pub const CUDA_ENC_H264_PROFILE_PROGRESSIVE_HIGH: i32 = 6;
	pub const CUDA_ENC_H264_PROFILE_STEREO: i32 = 5;
	pub const CUDA_ENC_HEVC_PROFILE_FREXT: i32 = 10;
	pub const CUDA_ENC_HEVC_PROFILE_MAIN: i32 = 8;
	pub const CUDA_ENC_HEVC_PROFILE_MAIN10: i32 = 9;
	/// Single Pass.
	pub const CUDA_ENC_MULTI_PASS_DISABLED: i32 = 0;
	/// Constant bitrate mode.
	pub const CUDA_ENC_PARAMS_RC_CBR: i32 = 2;
	/// Constant QP mode.
	pub const CUDA_ENC_PARAMS_RC_CONSTQP: i32 = 0;
	/// Variable bitrate mode.
	pub const CUDA_ENC_PARAMS_RC_VBR: i32 = 1;
	pub const CUDA_ENC_PRESET_P1: i32 = 1;
	pub const CUDA_ENC_PRESET_P2: i32 = 2;
	pub const CUDA_ENC_PRESET_P3: i32 = 3;
	pub const CUDA_ENC_PRESET_P4: i32 = 4;
	pub const CUDA_ENC_PRESET_P5: i32 = 5;
	pub const CUDA_ENC_PRESET_P6: i32 = 6;
	pub const CUDA_ENC_PRESET_P7: i32 = 7;
	pub const CUDA_ENC_TUNING_INFO_COUNT: i32 = 5;
	/// Tune presets for latency tolerant encoding.
	pub const CUDA_ENC_TUNING_INFO_HIGH_QUALITY: i32 = 1;
	/// Tune presets for lossless encoding.
	pub const CUDA_ENC_TUNING_INFO_LOSSLESS: i32 = 4;
	/// Tune presets for low latency streaming.
	pub const CUDA_ENC_TUNING_INFO_LOW_LATENCY: i32 = 2;
	/// Tune presets for ultra low latency streaming.
	pub const CUDA_ENC_TUNING_INFO_ULTRA_LOW_LATENCY: i32 = 3;
	/// Undefined tuningInfo. Invalid value for encoding.
	pub const CUDA_ENC_TUNING_INFO_UNDEFINED: i32 = 0;
	/// Two Pass encoding is enabled where first Pass is full resolution.
	pub const CUDA_ENC_TWO_PASS_FULL_RESOLUTION: i32 = 2;
	/// Two Pass encoding is enabled where first Pass is quarter resolution.
	pub const CUDA_ENC_TWO_PASS_QUARTER_RESOLUTION: i32 = 1;
	pub const CUDA_H264: i32 = 4;
	pub const CUDA_H264_MVC: i32 = 7;
	pub const CUDA_H264_SVC: i32 = 6;
	pub const CUDA_HEVC: i32 = 8;
	pub const CUDA_JPEG: i32 = 5;
	pub const CUDA_MPEG1: i32 = 0;
	pub const CUDA_MPEG2: i32 = 1;
	pub const CUDA_MPEG4: i32 = 2;
	pub const CUDA_Monochrome: i32 = 0;
	pub const CUDA_NumCodecs: i32 = 12;
	pub const CUDA_NumFormats: i32 = 4;
	/// Y,UV  (4:2:0)
	pub const CUDA_Uncompressed_NV12: i32 = 1314271538;
	/// UYVY (4:2:2)
	pub const CUDA_Uncompressed_UYVY: i32 = 1431918169;
	/// Y,U,V (4:2:0)
	pub const CUDA_Uncompressed_YUV420: i32 = 1230591318;
	/// YUYV/YUY2 (4:2:2)
	pub const CUDA_Uncompressed_YUYV: i32 = 1498765654;
	/// Y,V,U (4:2:0)
	pub const CUDA_Uncompressed_YV12: i32 = 1498820914;
	pub const CUDA_VC1: i32 = 3;
	pub const CUDA_VP8: i32 = 9;
	pub const CUDA_VP9: i32 = 10;
	/// Status of VideoReaderInitParams::allowFrameDrop initialization.
	pub const CUDA_VideoReaderProps_PROP_ALLOW_FRAME_DROP: i32 = 8;
	/// Set the ColorFormat of the decoded frame.  This can be changed before every call to nextFrame() and retrieve().
	pub const CUDA_VideoReaderProps_PROP_COLOR_FORMAT: i32 = 6;
	/// Index for retrieving the decoded frame using retrieve().
	pub const CUDA_VideoReaderProps_PROP_DECODED_FRAME_IDX: i32 = 0;
	/// Index for retrieving the extra data associated with a video source using retrieve().
	pub const CUDA_VideoReaderProps_PROP_EXTRA_DATA_INDEX: i32 = 1;
	/// FFmpeg source only - Indicates whether the Last Raw Frame (LRF), output from VideoReader::retrieve() when VideoReader is initialized in raw mode, contains encoded data for a key frame.
	pub const CUDA_VideoReaderProps_PROP_LRF_HAS_KEY_FRAME: i32 = 5;
	pub const CUDA_VideoReaderProps_PROP_NOT_SUPPORTED: i32 = 9;
	/// Number of raw packages recieved since the last call to grab().
	pub const CUDA_VideoReaderProps_PROP_NUMBER_OF_RAW_PACKAGES_SINCE_LAST_GRAB: i32 = 3;
	/// Status of raw mode.
	pub const CUDA_VideoReaderProps_PROP_RAW_MODE: i32 = 4;
	/// Base index for retrieving raw encoded data using retrieve().
	pub const CUDA_VideoReaderProps_PROP_RAW_PACKAGES_BASE_INDEX: i32 = 2;
	/// Status of VideoReaderInitParams::udpSource initialization.
	pub const CUDA_VideoReaderProps_PROP_UDP_SOURCE: i32 = 7;
	pub const CUDA_Weave: i32 = 0;
	pub const CUDA_YUV420: i32 = 1;
	pub const CUDA_YUV422: i32 = 2;
	pub const CUDA_YUV444: i32 = 3;
	/// Chroma formats supported by cudacodec::VideoReader.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum CUDA_ChromaFormat {
		Monochrome = 0,
		YUV420 = 1,
		YUV422 = 2,
		YUV444 = 3,
		NumFormats = 4,
	}
	
	opencv_type_enum! { crate::cudacodec::CUDA_ChromaFormat }
	
	/// Video codecs supported by cudacodec::VideoReader and cudacodec::VideoWriter.
	/// 
	/// Note:
	///    *   Support will depend on your hardware, refer to the Nvidia Video Codec SDK Video Encode and Decode GPU Support Matrix for details.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum CUDA_Codec {
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
	
	opencv_type_enum! { crate::cudacodec::CUDA_Codec }
	
	/// ColorFormat for the frame returned by VideoReader::nextFrame() and VideoReader::retrieve() or used to initialize a VideoWriter.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum CUDA_ColorFormat {
		UNDEFINED = 0,
		/// OpenCV color format, can be used with both VideoReader and VideoWriter.
		BGRA = 1,
		/// OpenCV color format, can be used with both VideoReader and VideoWriter.
		BGR = 2,
		/// OpenCV color format, can be used with both VideoReader and VideoWriter.
		GRAY = 3,
		/// Nvidia color format - equivalent to YUV - Semi-Planar YUV [Y plane followed by interleaved UV plane], can be used with both VideoReader and VideoWriter.
		NV_NV12 = 4,
		/// OpenCV color format, can only be used with VideoWriter.
		RGB = 5,
		/// OpenCV color format, can only be used with VideoWriter.
		RGBA = 6,
		/// Nvidia Buffer Format - Planar YUV [Y plane followed by V and U planes], use with VideoReader, can only be used with VideoWriter.
		NV_YV12 = 8,
		/// Nvidia Buffer Format - Planar YUV [Y plane followed by U and V planes], use with VideoReader, can only be used with VideoWriter.
		NV_IYUV = 9,
		/// Nvidia Buffer Format - Planar YUV [Y plane followed by U and V planes], use with VideoReader, can only be used with VideoWriter.
		NV_YUV444 = 10,
		/// Nvidia Buffer Format - 8 bit Packed A8Y8U8V8. This is a word-ordered format where a pixel is represented by a 32-bit word with V in the lowest 8 bits, U in the next 8 bits, Y in the 8 bits after that and A in the highest 8 bits, can only be used with VideoWriter.
		NV_AYUV = 11,
		PROP_NOT_SUPPORTED = 12,
	}
	
	opencv_type_enum! { crate::cudacodec::CUDA_ColorFormat }
	
	/// Deinterlacing mode used by decoder.
	/// ## Parameters
	/// * Weave: Weave both fields (no deinterlacing). For progressive content and for content that doesn't need deinterlacing.
	/// Bob Drop one field.
	/// * Adaptive: Adaptive deinterlacing needs more video memory than other deinterlacing modes.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum CUDA_DeinterlaceMode {
		Weave = 0,
		Bob = 1,
		Adaptive = 2,
	}
	
	opencv_type_enum! { crate::cudacodec::CUDA_DeinterlaceMode }
	
	/// Multi Pass Encoding.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum CUDA_EncodeMultiPass {
		/// Single Pass.
		ENC_MULTI_PASS_DISABLED = 0,
		/// Two Pass encoding is enabled where first Pass is quarter resolution.
		ENC_TWO_PASS_QUARTER_RESOLUTION = 1,
		/// Two Pass encoding is enabled where first Pass is full resolution.
		ENC_TWO_PASS_FULL_RESOLUTION = 2,
	}
	
	opencv_type_enum! { crate::cudacodec::CUDA_EncodeMultiPass }
	
	/// Rate Control Modes.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum CUDA_EncodeParamsRcMode {
		/// Constant QP mode.
		ENC_PARAMS_RC_CONSTQP = 0,
		/// Variable bitrate mode.
		ENC_PARAMS_RC_VBR = 1,
		/// Constant bitrate mode.
		ENC_PARAMS_RC_CBR = 2,
	}
	
	opencv_type_enum! { crate::cudacodec::CUDA_EncodeParamsRcMode }
	
	/// Nvidia Encoding Presets. Performance degrades and quality improves as we move from P1 to P7.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum CUDA_EncodePreset {
		ENC_PRESET_P1 = 1,
		ENC_PRESET_P2 = 2,
		ENC_PRESET_P3 = 3,
		ENC_PRESET_P4 = 4,
		ENC_PRESET_P5 = 5,
		ENC_PRESET_P6 = 6,
		ENC_PRESET_P7 = 7,
	}
	
	opencv_type_enum! { crate::cudacodec::CUDA_EncodePreset }
	
	/// Supported Encoder Profiles.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum CUDA_EncodeProfile {
		ENC_CODEC_PROFILE_AUTOSELECT = 0,
		ENC_H264_PROFILE_BASELINE = 1,
		ENC_H264_PROFILE_MAIN = 2,
		ENC_H264_PROFILE_HIGH = 3,
		ENC_H264_PROFILE_HIGH_444 = 4,
		ENC_H264_PROFILE_STEREO = 5,
		ENC_H264_PROFILE_PROGRESSIVE_HIGH = 6,
		ENC_H264_PROFILE_CONSTRAINED_HIGH = 7,
		ENC_HEVC_PROFILE_MAIN = 8,
		ENC_HEVC_PROFILE_MAIN10 = 9,
		ENC_HEVC_PROFILE_FREXT = 10,
	}
	
	opencv_type_enum! { crate::cudacodec::CUDA_EncodeProfile }
	
	/// Tuning information.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum CUDA_EncodeTuningInfo {
		/// Undefined tuningInfo. Invalid value for encoding.
		ENC_TUNING_INFO_UNDEFINED = 0,
		/// Tune presets for latency tolerant encoding.
		ENC_TUNING_INFO_HIGH_QUALITY = 1,
		/// Tune presets for low latency streaming.
		ENC_TUNING_INFO_LOW_LATENCY = 2,
		/// Tune presets for ultra low latency streaming.
		ENC_TUNING_INFO_ULTRA_LOW_LATENCY = 3,
		/// Tune presets for lossless encoding.
		ENC_TUNING_INFO_LOSSLESS = 4,
		ENC_TUNING_INFO_COUNT = 5,
	}
	
	opencv_type_enum! { crate::cudacodec::CUDA_EncodeTuningInfo }
	
	/// cv::cudacodec::VideoReader generic properties identifier.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum CUDA_VideoReaderProps {
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
	
	opencv_type_enum! { crate::cudacodec::CUDA_VideoReaderProps }
	
	/// @overload
	/// ## Parameters
	/// * source: RAW video source implemented by user.
	/// * params: Initializaton parameters. See cv::cudacodec::VideoReaderInitParams.
	/// 
	/// ## Note
	/// This alternative version of [create_video_reader_1] function uses the following default values for its arguments:
	/// * params: VideoReaderInitParams()
	#[inline]
	pub fn create_video_reader_1_def(source: &core::Ptr<crate::cudacodec::CUDA_RawVideoSource>) -> Result<core::Ptr<crate::cudacodec::CUDA_VideoReader>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_createVideoReader_const_PtrLRawVideoSourceGR(source.as_raw_PtrOfCUDA_RawVideoSource(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudacodec::CUDA_VideoReader>::opencv_from_extern(ret) };
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
	/// ## Overloaded parameters
	/// 
	/// * source: RAW video source implemented by user.
	/// * params: Initializaton parameters. See cv::cudacodec::VideoReaderInitParams.
	/// 
	/// ## C++ default parameters
	/// * params: VideoReaderInitParams()
	#[inline]
	pub fn create_video_reader_1(source: &core::Ptr<crate::cudacodec::CUDA_RawVideoSource>, params: crate::cudacodec::CUDA_VideoReaderInitParams) -> Result<core::Ptr<crate::cudacodec::CUDA_VideoReader>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_createVideoReader_const_PtrLRawVideoSourceGR_const_VideoReaderInitParams(source.as_raw_PtrOfCUDA_RawVideoSource(), params.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudacodec::CUDA_VideoReader>::opencv_from_extern(ret) };
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
	/// ## Note
	/// This alternative version of [create_video_reader] function uses the following default values for its arguments:
	/// * source_params: {}
	/// * params: VideoReaderInitParams()
	#[inline]
	pub fn create_video_reader_def(filename: &str) -> Result<core::Ptr<crate::cudacodec::CUDA_VideoReader>> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_createVideoReader_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudacodec::CUDA_VideoReader>::opencv_from_extern(ret) };
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
	pub fn create_video_reader(filename: &str, source_params: &core::Vector<i32>, params: crate::cudacodec::CUDA_VideoReaderInitParams) -> Result<core::Ptr<crate::cudacodec::CUDA_VideoReader>> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_createVideoReader_const_StringR_const_vectorLintGR_const_VideoReaderInitParams(filename.opencv_as_extern(), source_params.as_raw_VectorOfi32(), params.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudacodec::CUDA_VideoReader>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates video writer.
	/// 
	/// ## Parameters
	/// * fileName: Name of the output video file. Only raw h264 or hevc files are supported.
	/// * frameSize: Size of the input video frames.
	/// * codec: Codec.
	/// * fps: Framerate of the created video stream.
	/// * colorFormat: OpenCv color format of the frames to be encoded.
	/// * encoderCallback: Callbacks for video encoder. See cudacodec::EncoderCallback. Required for working with the encoded video stream.
	/// * stream: Stream for frame pre-processing.
	/// 
	/// ## Note
	/// This alternative version of [create_video_writer] function uses the following default values for its arguments:
	/// * codec: Codec::H264
	/// * fps: 25.0
	/// * color_format: ColorFormat::BGR
	/// * encoder_callback: 0
	/// * stream: Stream::Null()
	#[inline]
	pub fn create_video_writer_def(file_name: &str, frame_size: core::Size) -> Result<core::Ptr<crate::cudacodec::CUDA_VideoWriter>> {
		extern_container_arg!(file_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_createVideoWriter_const_StringR_const_Size(file_name.opencv_as_extern(), frame_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudacodec::CUDA_VideoWriter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates video writer.
	/// 
	/// ## Parameters
	/// * fileName: Name of the output video file. Only raw h264 or hevc files are supported.
	/// * frameSize: Size of the input video frames.
	/// * codec: Codec.
	/// * fps: Framerate of the created video stream.
	/// * colorFormat: OpenCv color format of the frames to be encoded.
	/// * encoderCallback: Callbacks for video encoder. See cudacodec::EncoderCallback. Required for working with the encoded video stream.
	/// * stream: Stream for frame pre-processing.
	/// 
	/// ## C++ default parameters
	/// * codec: Codec::H264
	/// * fps: 25.0
	/// * color_format: ColorFormat::BGR
	/// * encoder_callback: 0
	/// * stream: Stream::Null()
	#[inline]
	pub fn create_video_writer(file_name: &str, frame_size: core::Size, codec: crate::cudacodec::CUDA_Codec, fps: f64, color_format: crate::cudacodec::CUDA_ColorFormat, mut encoder_callback: core::Ptr<crate::cudacodec::CUDA_EncoderCallback>, stream: &core::Stream) -> Result<core::Ptr<crate::cudacodec::CUDA_VideoWriter>> {
		extern_container_arg!(file_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_createVideoWriter_const_StringR_const_Size_const_Codec_const_double_const_ColorFormat_PtrLEncoderCallbackG_const_StreamR(file_name.opencv_as_extern(), frame_size.opencv_as_extern(), codec, fps, color_format, encoder_callback.as_raw_mut_PtrOfCUDA_EncoderCallback(), stream.as_raw_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudacodec::CUDA_VideoWriter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates video writer.
	/// 
	/// ## Parameters
	/// * fileName: Name of the output video file. Only raw h264 or hevc files are supported.
	/// * frameSize: Size of the input video frames.
	/// * codec: Codec.
	/// * fps: Framerate of the created video stream.
	/// * colorFormat: OpenCv color format of the frames to be encoded.
	/// * params: Additional encoding parameters.
	/// * encoderCallback: Callbacks for video encoder. See cudacodec::EncoderCallback. Required for working with the encoded video stream.
	/// * stream: Stream for frame pre-processing.
	/// 
	/// ## Note
	/// This alternative version of [create_video_writer_1] function uses the following default values for its arguments:
	/// * encoder_callback: 0
	/// * stream: Stream::Null()
	#[inline]
	pub fn create_video_writer_1_def(file_name: &str, frame_size: core::Size, codec: crate::cudacodec::CUDA_Codec, fps: f64, color_format: crate::cudacodec::CUDA_ColorFormat, params: crate::cudacodec::CUDA_EncoderParams) -> Result<core::Ptr<crate::cudacodec::CUDA_VideoWriter>> {
		extern_container_arg!(file_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_createVideoWriter_const_StringR_const_Size_const_Codec_const_double_const_ColorFormat_const_EncoderParamsR(file_name.opencv_as_extern(), frame_size.opencv_as_extern(), codec, fps, color_format, &params, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudacodec::CUDA_VideoWriter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates video writer.
	/// 
	/// ## Parameters
	/// * fileName: Name of the output video file. Only raw h264 or hevc files are supported.
	/// * frameSize: Size of the input video frames.
	/// * codec: Codec.
	/// * fps: Framerate of the created video stream.
	/// * colorFormat: OpenCv color format of the frames to be encoded.
	/// * params: Additional encoding parameters.
	/// * encoderCallback: Callbacks for video encoder. See cudacodec::EncoderCallback. Required for working with the encoded video stream.
	/// * stream: Stream for frame pre-processing.
	/// 
	/// ## C++ default parameters
	/// * encoder_callback: 0
	/// * stream: Stream::Null()
	#[inline]
	pub fn create_video_writer_1(file_name: &str, frame_size: core::Size, codec: crate::cudacodec::CUDA_Codec, fps: f64, color_format: crate::cudacodec::CUDA_ColorFormat, params: crate::cudacodec::CUDA_EncoderParams, mut encoder_callback: core::Ptr<crate::cudacodec::CUDA_EncoderCallback>, stream: &core::Stream) -> Result<core::Ptr<crate::cudacodec::CUDA_VideoWriter>> {
		extern_container_arg!(file_name);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_createVideoWriter_const_StringR_const_Size_const_Codec_const_double_const_ColorFormat_const_EncoderParamsR_PtrLEncoderCallbackG_const_StreamR(file_name.opencv_as_extern(), frame_size.opencv_as_extern(), codec, fps, color_format, &params, encoder_callback.as_raw_mut_PtrOfCUDA_EncoderCallback(), stream.as_raw_Stream(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::cudacodec::CUDA_VideoWriter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn equals_cuda_encoderparams_cuda_encoderparams(lhs: crate::cudacodec::CUDA_EncoderParams, rhs: crate::cudacodec::CUDA_EncoderParams) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cudacodec_operatorEQ_const_EncoderParamsR_const_EncoderParamsR(&lhs, &rhs, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Quantization Parameter for each type of frame when using ENC_PARAMS_RC_MODE::ENC_PARAMS_RC_CONSTQP.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct CUDA_EncodeQp {
		/// Specifies QP value for P-frame.
		pub qp_inter_p: u32,
		/// Specifies QP value for B-frame.
		pub qp_inter_b: u32,
		/// Specifies QP value for Intra Frame.
		pub qp_intra: u32,
	}
	
	opencv_type_simple! { crate::cudacodec::CUDA_EncodeQp }
	
	impl CUDA_EncodeQp {
	}
	
	/// Constant methods for [crate::cudacodec::CUDA_EncoderCallback]
	pub trait CUDA_EncoderCallbackTraitConst {
		fn as_raw_CUDA_EncoderCallback(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::cudacodec::CUDA_EncoderCallback]
	pub trait CUDA_EncoderCallbackTrait: crate::cudacodec::CUDA_EncoderCallbackTraitConst {
		fn as_raw_mut_CUDA_EncoderCallback(&mut self) -> *mut c_void;
	
		/// Callback function to signal that the encoded bitstream for one or more frames is ready.
		/// 
		/// ## Parameters
		/// * vPacket: The raw bitstream for one or more frames.
		#[inline]
		fn on_encoded(&mut self, mut v_packet: core::Vector<core::Vector<u8>>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_EncoderCallback_onEncoded_vectorLvectorLuint8_tGG(self.as_raw_mut_CUDA_EncoderCallback(), v_packet.as_raw_mut_VectorOfVectorOfu8(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Callback function to that the encoding has finished.
		#[inline]
		fn on_encoding_finished(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_EncoderCallback_onEncodingFinished(self.as_raw_mut_CUDA_EncoderCallback(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Interface for encoder callbacks.
	/// 
	/// User can implement own multiplexing by implementing this interface.
	pub struct CUDA_EncoderCallback {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CUDA_EncoderCallback }
	
	impl Drop for CUDA_EncoderCallback {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cudacodec_EncoderCallback_delete(self.as_raw_mut_CUDA_EncoderCallback()) };
		}
	}
	
	unsafe impl Send for CUDA_EncoderCallback {}
	
	impl crate::cudacodec::CUDA_EncoderCallbackTraitConst for CUDA_EncoderCallback {
		#[inline] fn as_raw_CUDA_EncoderCallback(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudacodec::CUDA_EncoderCallbackTrait for CUDA_EncoderCallback {
		#[inline] fn as_raw_mut_CUDA_EncoderCallback(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CUDA_EncoderCallback {
	}
	
	impl std::fmt::Debug for CUDA_EncoderCallback {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_EncoderCallback")
				.finish()
		}
	}
	
	/// Different parameters for CUDA video encoder.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct CUDA_EncoderParams {
		pub nv_preset: crate::cudacodec::CUDA_EncodePreset,
		pub tuning_info: crate::cudacodec::CUDA_EncodeTuningInfo,
		pub encoding_profile: crate::cudacodec::CUDA_EncodeProfile,
		pub rate_control_mode: crate::cudacodec::CUDA_EncodeParamsRcMode,
		pub multi_pass_encoding: crate::cudacodec::CUDA_EncodeMultiPass,
		/// QP's for ENC_PARAMS_RC_CONSTQP.
		pub const_qp: crate::cudacodec::CUDA_EncodeQp,
		/// target bitrate for ENC_PARAMS_RC_VBR and ENC_PARAMS_RC_CBR.
		pub average_bit_rate: i32,
		/// upper bound on bitrate for ENC_PARAMS_RC_VBR and ENC_PARAMS_RC_CONSTQP.
		pub max_bit_rate: i32,
		/// value 0 - 51 where video quality decreases as targetQuality increases, used with ENC_PARAMS_RC_VBR.
		pub target_quality: u8,
		pub gop_length: i32,
	}
	
	opencv_type_simple! { crate::cudacodec::CUDA_EncoderParams }
	
	impl CUDA_EncoderParams {
		#[inline]
		pub fn default() -> Result<crate::cudacodec::CUDA_EncoderParams> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_EncoderParams_EncoderParams(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Struct providing information about video file format. :
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct CUDA_FormatInfo {
		pub codec: crate::cudacodec::CUDA_Codec,
		pub chroma_format: crate::cudacodec::CUDA_ChromaFormat,
		pub n_bit_depth_minus8: i32,
		pub n_bit_depth_chroma_minus8: i32,
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
		pub deinterlace_mode: crate::cudacodec::CUDA_DeinterlaceMode,
		/// Post-processed size of the output frame.
		pub target_sz: core::Size,
		/// Region of interest decoded from video source.
		pub src_roi: core::Rect,
		/// Region of interest in the output frame containing the decoded frame.
		pub target_roi: core::Rect,
		/// Output value indicating if the black level, luma and chroma of the source are represented using the full or limited range (AKA TV or "analogue" range) of values as defined in Annex E of the ITU-T Specification.  Internally the conversion from NV12 to BGR obeys ITU 709.
		pub video_full_range_flag: bool,
	}
	
	opencv_type_simple! { crate::cudacodec::CUDA_FormatInfo }
	
	impl CUDA_FormatInfo {
		#[inline]
		pub fn default() -> Result<crate::cudacodec::CUDA_FormatInfo> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_FormatInfo_FormatInfo(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Constant methods for [crate::cudacodec::CUDA_RawVideoSource]
	pub trait CUDA_RawVideoSourceTraitConst {
		fn as_raw_CUDA_RawVideoSource(&self) -> *const c_void;
	
		/// Returns true if the last packet contained a key frame.
		#[inline]
		fn last_packet_contains_key_frame(&self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_RawVideoSource_lastPacketContainsKeyFrame_const(self.as_raw_CUDA_RawVideoSource(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Returns information about video file format.
		#[inline]
		fn format(&self) -> Result<crate::cudacodec::CUDA_FormatInfo> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_RawVideoSource_format_const(self.as_raw_CUDA_RawVideoSource(), ocvrs_return.as_mut_ptr()) };
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
			unsafe { sys::cv_cudacodec_RawVideoSource_getExtraData_const_MatR(self.as_raw_CUDA_RawVideoSource(), extra_data.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Retrieves the specified property used by the VideoSource.
		/// 
		/// ## Parameters
		/// * propertyId: Property identifier from cv::VideoCaptureProperties (eg. cv::CAP_PROP_POS_MSEC, cv::CAP_PROP_POS_FRAMES, ...)
		/// or one from [videoio_flags_others].
		/// * propertyVal: Value for the specified property.
		/// 
		/// ## Returns
		/// `true` unless the property is unset set or not supported.
		#[inline]
		fn get(&self, property_id: i32, property_val: &mut f64) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_RawVideoSource_get_const_const_int_doubleR(self.as_raw_CUDA_RawVideoSource(), property_id, property_val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::cudacodec::CUDA_RawVideoSource]
	pub trait CUDA_RawVideoSourceTrait: crate::cudacodec::CUDA_RawVideoSourceTraitConst {
		fn as_raw_mut_CUDA_RawVideoSource(&mut self) -> *mut c_void;
	
		/// Returns next packet with RAW video frame.
		/// 
		/// ## Parameters
		/// * data: Pointer to frame data.
		/// * size: Size in bytes of current frame.
		#[inline]
		fn get_next_packet(&mut self, data: &mut &mut u8, size: &mut size_t) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_RawVideoSource_getNextPacket_unsigned_charXX_size_tX(self.as_raw_mut_CUDA_RawVideoSource(), data as *mut  _ as *mut  *mut  _, size, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Updates the coded width and height inside format.
		#[inline]
		fn update_format(&mut self, video_format: crate::cudacodec::CUDA_FormatInfo) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_RawVideoSource_updateFormat_const_FormatInfoR(self.as_raw_mut_CUDA_RawVideoSource(), &video_format, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Interface for video demultiplexing. :
	/// 
	/// User can implement own demultiplexing by implementing this interface.
	pub struct CUDA_RawVideoSource {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CUDA_RawVideoSource }
	
	impl Drop for CUDA_RawVideoSource {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cudacodec_RawVideoSource_delete(self.as_raw_mut_CUDA_RawVideoSource()) };
		}
	}
	
	unsafe impl Send for CUDA_RawVideoSource {}
	
	impl crate::cudacodec::CUDA_RawVideoSourceTraitConst for CUDA_RawVideoSource {
		#[inline] fn as_raw_CUDA_RawVideoSource(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudacodec::CUDA_RawVideoSourceTrait for CUDA_RawVideoSource {
		#[inline] fn as_raw_mut_CUDA_RawVideoSource(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CUDA_RawVideoSource {
	}
	
	impl std::fmt::Debug for CUDA_RawVideoSource {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_RawVideoSource")
				.finish()
		}
	}
	
	/// Constant methods for [crate::cudacodec::CUDA_VideoReader]
	pub trait CUDA_VideoReaderTraitConst {
		fn as_raw_CUDA_VideoReader(&self) -> *const c_void;
	
		/// Returns information about video file format.
		#[inline]
		fn format(&self) -> Result<crate::cudacodec::CUDA_FormatInfo> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_VideoReader_format_const(self.as_raw_CUDA_VideoReader(), ocvrs_return.as_mut_ptr()) };
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
		fn retrieve(&self, frame: &mut impl core::ToOutputArray, idx: size_t) -> Result<bool> {
			output_array_arg!(frame);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_VideoReader_retrieve_const_const__OutputArrayR_const_size_t(self.as_raw_CUDA_VideoReader(), frame.as_raw__OutputArray(), idx, ocvrs_return.as_mut_ptr()) };
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
		/// ## Note
		/// This alternative version of [retrieve] function uses the following default values for its arguments:
		/// * idx: static_cast<size_t>(VideoReaderProps::PROP_DECODED_FRAME_IDX)
		#[inline]
		fn retrieve_def(&self, frame: &mut impl core::ToOutputArray) -> Result<bool> {
			output_array_arg!(frame);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_VideoReader_retrieve_const_const__OutputArrayR(self.as_raw_CUDA_VideoReader(), frame.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
			unsafe { sys::cv_cudacodec_VideoReader_retrieve_const_MatR_const_size_t(self.as_raw_CUDA_VideoReader(), frame.as_raw_mut_Mat(), idx, ocvrs_return.as_mut_ptr()) };
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
			unsafe { sys::cv_cudacodec_VideoReader_retrieve_const_GpuMatR(self.as_raw_CUDA_VideoReader(), frame.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
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
		fn get(&self, property_id: crate::cudacodec::CUDA_VideoReaderProps, property_val: &mut f64) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_VideoReader_get_const_const_VideoReaderProps_doubleR(self.as_raw_CUDA_VideoReader(), property_id, property_val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * property_val_in: 0
		#[inline]
		fn get_video_reader_props(&self, property_id: crate::cudacodec::CUDA_VideoReaderProps, property_val_out: &mut f64, property_val_in: f64) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_VideoReader_getVideoReaderProps_const_const_VideoReaderProps_doubleR_double(self.as_raw_CUDA_VideoReader(), property_id, property_val_out, property_val_in, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [get_video_reader_props] function uses the following default values for its arguments:
		/// * property_val_in: 0
		#[inline]
		fn get_video_reader_props_def(&self, property_id: crate::cudacodec::CUDA_VideoReaderProps, property_val_out: &mut f64) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_VideoReader_getVideoReaderProps_const_const_VideoReaderProps_doubleR(self.as_raw_CUDA_VideoReader(), property_id, property_val_out, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Retrieves the specified property used by the VideoSource.
		/// 
		/// ## Parameters
		/// * propertyId: Property identifier from cv::VideoCaptureProperties (eg. cv::CAP_PROP_POS_MSEC, cv::CAP_PROP_POS_FRAMES, ...)
		/// or one from [videoio_flags_others].
		/// * propertyVal: Value for the specified property.
		/// 
		/// ## Returns
		/// `true` unless the property is unset set or not supported.
		#[inline]
		fn get_1(&self, property_id: i32, property_val: &mut f64) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_VideoReader_get_const_const_int_doubleR(self.as_raw_CUDA_VideoReader(), property_id, property_val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::cudacodec::CUDA_VideoReader]
	pub trait CUDA_VideoReaderTrait: crate::cudacodec::CUDA_VideoReaderTraitConst {
		fn as_raw_mut_CUDA_VideoReader(&mut self) -> *mut c_void;
	
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
			unsafe { sys::cv_cudacodec_VideoReader_nextFrame_GpuMatR_StreamR(self.as_raw_mut_CUDA_VideoReader(), frame.as_raw_mut_GpuMat(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
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
		/// ## Note
		/// This alternative version of [next_frame] function uses the following default values for its arguments:
		/// * stream: Stream::Null()
		#[inline]
		fn next_frame_def(&mut self, frame: &mut core::GpuMat) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_VideoReader_nextFrame_GpuMatR(self.as_raw_mut_CUDA_VideoReader(), frame.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
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
			unsafe { sys::cv_cudacodec_VideoReader_grab_StreamR(self.as_raw_mut_CUDA_VideoReader(), stream.as_raw_mut_Stream(), ocvrs_return.as_mut_ptr()) };
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
		/// ## Note
		/// This alternative version of [grab] function uses the following default values for its arguments:
		/// * stream: Stream::Null()
		#[inline]
		fn grab_def(&mut self) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_VideoReader_grab(self.as_raw_mut_CUDA_VideoReader(), ocvrs_return.as_mut_ptr()) };
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
		fn set(&mut self, property_id: crate::cudacodec::CUDA_VideoReaderProps, property_val: f64) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_VideoReader_set_const_VideoReaderProps_const_double(self.as_raw_mut_CUDA_VideoReader(), property_id, property_val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_video_reader_props(&mut self, property_id: crate::cudacodec::CUDA_VideoReaderProps, property_val: f64) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_VideoReader_setVideoReaderProps_const_VideoReaderProps_double(self.as_raw_mut_CUDA_VideoReader(), property_id, property_val, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Set the desired ColorFormat for the frame returned by nextFrame()/retrieve().
		/// 
		/// ## Parameters
		/// * colorFormat: Value of the ColorFormat.
		/// ## Returns
		/// `true` unless the colorFormat is not supported.
		#[inline]
		fn set_1(&mut self, color_format: crate::cudacodec::CUDA_ColorFormat) -> Result<bool> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_VideoReader_set_const_ColorFormat(self.as_raw_mut_CUDA_VideoReader(), color_format, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Video reader interface.
	/// 
	/// Available when built with WITH_NVCUVID=ON while Nvidia's Video Codec SDK is installed.
	/// 
	/// Decoding support is dependent on the GPU, refer to the Nvidia Video Codec SDK Video Encode and Decode GPU Support Matrix for details.
	/// 
	/// 
	/// Note:
	///    *   An example on how to use the videoReader class can be found at
	///        opencv_source_code/samples/gpu/video_reader.cpp
	pub struct CUDA_VideoReader {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CUDA_VideoReader }
	
	impl Drop for CUDA_VideoReader {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cudacodec_VideoReader_delete(self.as_raw_mut_CUDA_VideoReader()) };
		}
	}
	
	unsafe impl Send for CUDA_VideoReader {}
	
	impl crate::cudacodec::CUDA_VideoReaderTraitConst for CUDA_VideoReader {
		#[inline] fn as_raw_CUDA_VideoReader(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudacodec::CUDA_VideoReaderTrait for CUDA_VideoReader {
		#[inline] fn as_raw_mut_CUDA_VideoReader(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CUDA_VideoReader {
	}
	
	impl std::fmt::Debug for CUDA_VideoReader {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_VideoReader")
				.finish()
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
	/// * targetSz: Post-processed size (width/height should be multiples of 2) of the output frame, defaults to the size of the encoded video source.
	/// * srcRoi: Region of interest (x/width should be multiples of 4 and y/height multiples of 2) decoded from video source, defaults to the full frame.
	/// * targetRoi: Region of interest (x/width should be multiples of 4 and y/height multiples of 2) within the output frame to copy and resize the decoded frame to,
	/// defaults to the full frame.
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq)]
	pub struct CUDA_VideoReaderInitParams {
		pub udp_source: bool,
		pub allow_frame_drop: bool,
		pub min_num_decode_surfaces: i32,
		pub raw_mode: bool,
		pub target_sz: core::Size,
		pub src_roi: core::Rect,
		pub target_roi: core::Rect,
	}
	
	opencv_type_simple! { crate::cudacodec::CUDA_VideoReaderInitParams }
	
	impl CUDA_VideoReaderInitParams {
		#[inline]
		pub fn default() -> Result<crate::cudacodec::CUDA_VideoReaderInitParams> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_VideoReaderInitParams_VideoReaderInitParams(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Constant methods for [crate::cudacodec::CUDA_VideoWriter]
	pub trait CUDA_VideoWriterTraitConst {
		fn as_raw_CUDA_VideoWriter(&self) -> *const c_void;
	
		/// Retrieve the encoding parameters.
		#[inline]
		fn get_encoder_params(&self) -> Result<crate::cudacodec::CUDA_EncoderParams> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_VideoWriter_getEncoderParams_const(self.as_raw_CUDA_VideoWriter(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::cudacodec::CUDA_VideoWriter]
	pub trait CUDA_VideoWriterTrait: crate::cudacodec::CUDA_VideoWriterTraitConst {
		fn as_raw_mut_CUDA_VideoWriter(&mut self) -> *mut c_void;
	
		/// Writes the next video frame.
		/// 
		/// ## Parameters
		/// * frame: The framet to be written.
		/// 
		/// The method encodes the specified image to a video stream. The image must have the same size and the same
		/// surface format as has been specified when opening the video writer.
		#[inline]
		fn write(&mut self, frame: &impl core::ToInputArray) -> Result<()> {
			input_array_arg!(frame);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_VideoWriter_write_const__InputArrayR(self.as_raw_mut_CUDA_VideoWriter(), frame.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// Waits until the encoding process has finished before calling EncoderCallback::onEncodingFinished().
		#[inline]
		fn release(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_cudacodec_VideoWriter_release(self.as_raw_mut_CUDA_VideoWriter(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Video writer interface.
	/// 
	/// Available when built with WITH_NVCUVENC=ON while Nvidia's Video Codec SDK is installed.
	/// 
	/// Encoding support is dependent on the GPU, refer to the Nvidia Video Codec SDK Video Encode and Decode GPU Support Matrix for details.
	/// 
	/// 
	/// Note:
	///    *   An example on how to use the videoWriter class can be found at
	///        opencv_source_code/samples/gpu/video_writer.cpp
	pub struct CUDA_VideoWriter {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { CUDA_VideoWriter }
	
	impl Drop for CUDA_VideoWriter {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_cudacodec_VideoWriter_delete(self.as_raw_mut_CUDA_VideoWriter()) };
		}
	}
	
	unsafe impl Send for CUDA_VideoWriter {}
	
	impl crate::cudacodec::CUDA_VideoWriterTraitConst for CUDA_VideoWriter {
		#[inline] fn as_raw_CUDA_VideoWriter(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::cudacodec::CUDA_VideoWriterTrait for CUDA_VideoWriter {
		#[inline] fn as_raw_mut_CUDA_VideoWriter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl CUDA_VideoWriter {
	}
	
	impl std::fmt::Debug for CUDA_VideoWriter {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("CUDA_VideoWriter")
				.finish()
		}
	}
}
