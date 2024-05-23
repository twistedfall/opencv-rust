// MapHist(const cuda::GpuMat &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:348
// ("cv::cudacodec::MapHist", vec![(pred!(mut, ["hist", "histFull"], ["const cv::cuda::GpuMat*", "cv::Mat*"]), _)]),
pub fn cv_cudacodec_MapHist_const_GpuMatR_MatR(hist: *const c_void, hist_full: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cudacodec::createNVSurfaceToColorConverter(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:404
// ("cv::cudacodec::createNVSurfaceToColorConverter", vec![(pred!(mut, ["colorSpace"], ["const cv::cudacodec::ColorSpaceStandard"]), _)]),
pub fn cv_cudacodec_createNVSurfaceToColorConverter_const_ColorSpaceStandard(color_space: crate::cudacodec::CUDA_ColorSpaceStandard, ocvrs_return: *mut Result<*mut c_void>);
// createNVSurfaceToColorConverter(const ColorSpaceStandard, const bool)(Enum, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:404
// ("cv::cudacodec::createNVSurfaceToColorConverter", vec![(pred!(mut, ["colorSpace", "videoFullRangeFlag"], ["const cv::cudacodec::ColorSpaceStandard", "const bool"]), _)]),
pub fn cv_cudacodec_createNVSurfaceToColorConverter_const_ColorSpaceStandard_const_bool(color_space: crate::cudacodec::CUDA_ColorSpaceStandard, video_full_range_flag: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::cudacodec::createVideoReader(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:667
// ("cv::cudacodec::createVideoReader", vec![(pred!(mut, ["source"], ["const cv::Ptr<cv::cudacodec::RawVideoSource>*"]), _)]),
pub fn cv_cudacodec_createVideoReader_const_PtrLRawVideoSourceGR(source: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// createVideoReader(const Ptr<RawVideoSource> &, const VideoReaderInitParams)(CppPassByVoidPtr, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:667
// ("cv::cudacodec::createVideoReader", vec![(pred!(mut, ["source", "params"], ["const cv::Ptr<cv::cudacodec::RawVideoSource>*", "const cv::cudacodec::VideoReaderInitParams"]), _)]),
pub fn cv_cudacodec_createVideoReader_const_PtrLRawVideoSourceGR_const_VideoReaderInitParams(source: *const c_void, params: *const crate::cudacodec::CUDA_VideoReaderInitParams, ocvrs_return: *mut Result<*mut c_void>);
// cv::cudacodec::createVideoReader(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:661
// ("cv::cudacodec::createVideoReader", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_cudacodec_createVideoReader_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// createVideoReader(const String &, const std::vector<int> &, const VideoReaderInitParams)(InString, CppPassByVoidPtr, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:661
// ("cv::cudacodec::createVideoReader", vec![(pred!(mut, ["filename", "sourceParams", "params"], ["const cv::String*", "const std::vector<int>*", "const cv::cudacodec::VideoReaderInitParams"]), _)]),
pub fn cv_cudacodec_createVideoReader_const_StringR_const_vectorLintGR_const_VideoReaderInitParams(filename: *const c_char, source_params: *const c_void, params: *const crate::cudacodec::CUDA_VideoReaderInitParams, ocvrs_return: *mut Result<*mut c_void>);
// cv::cudacodec::createVideoWriter(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:273
// ("cv::cudacodec::createVideoWriter", vec![(pred!(mut, ["fileName", "frameSize"], ["const cv::String*", "const cv::Size"]), _)]),
pub fn cv_cudacodec_createVideoWriter_const_StringR_const_Size(file_name: *const c_char, frame_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// createVideoWriter(const String &, const Size, const Codec, const double, const ColorFormat, Ptr<EncoderCallback>, const cuda::Stream &)(InString, SimpleClass, Enum, Primitive, Enum, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:273
// ("cv::cudacodec::createVideoWriter", vec![(pred!(mut, ["fileName", "frameSize", "codec", "fps", "colorFormat", "encoderCallback", "stream"], ["const cv::String*", "const cv::Size", "const cv::cudacodec::Codec", "const double", "const cv::cudacodec::ColorFormat", "cv::Ptr<cv::cudacodec::EncoderCallback>", "const cv::cuda::Stream*"]), _)]),
pub fn cv_cudacodec_createVideoWriter_const_StringR_const_Size_const_Codec_const_double_const_ColorFormat_PtrLEncoderCallbackG_const_StreamR(file_name: *const c_char, frame_size: *const core::Size, codec: crate::cudacodec::CUDA_Codec, fps: f64, color_format: crate::cudacodec::CUDA_ColorFormat, encoder_callback: *mut c_void, stream: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cudacodec::createVideoWriter(InString, SimpleClass, Enum, Primitive, Enum, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:287
// ("cv::cudacodec::createVideoWriter", vec![(pred!(mut, ["fileName", "frameSize", "codec", "fps", "colorFormat", "params"], ["const cv::String*", "const cv::Size", "const cv::cudacodec::Codec", "const double", "const cv::cudacodec::ColorFormat", "const cv::cudacodec::EncoderParams*"]), _)]),
pub fn cv_cudacodec_createVideoWriter_const_StringR_const_Size_const_Codec_const_double_const_ColorFormat_const_EncoderParamsR(file_name: *const c_char, frame_size: *const core::Size, codec: crate::cudacodec::CUDA_Codec, fps: f64, color_format: crate::cudacodec::CUDA_ColorFormat, params: *const crate::cudacodec::CUDA_EncoderParams, ocvrs_return: *mut Result<*mut c_void>);
// createVideoWriter(const String &, const Size, const Codec, const double, const ColorFormat, const EncoderParams &, Ptr<EncoderCallback>, const cuda::Stream &)(InString, SimpleClass, Enum, Primitive, Enum, SimpleClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:287
// ("cv::cudacodec::createVideoWriter", vec![(pred!(mut, ["fileName", "frameSize", "codec", "fps", "colorFormat", "params", "encoderCallback", "stream"], ["const cv::String*", "const cv::Size", "const cv::cudacodec::Codec", "const double", "const cv::cudacodec::ColorFormat", "const cv::cudacodec::EncoderParams*", "cv::Ptr<cv::cudacodec::EncoderCallback>", "const cv::cuda::Stream*"]), _)]),
pub fn cv_cudacodec_createVideoWriter_const_StringR_const_Size_const_Codec_const_double_const_ColorFormat_const_EncoderParamsR_PtrLEncoderCallbackG_const_StreamR(file_name: *const c_char, frame_size: *const core::Size, codec: crate::cudacodec::CUDA_Codec, fps: f64, color_format: crate::cudacodec::CUDA_ColorFormat, params: *const crate::cudacodec::CUDA_EncoderParams, encoder_callback: *mut c_void, stream: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator==(const EncoderParams &, const EncoderParams &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:202
// ("cv::cudacodec::operator==", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::cudacodec::EncoderParams*", "const cv::cudacodec::EncoderParams*"]), _)]),
pub fn cv_cudacodec_operatorEQ_const_EncoderParamsR_const_EncoderParamsR(lhs: *const crate::cudacodec::CUDA_EncoderParams, rhs: *const crate::cudacodec::CUDA_EncoderParams, ocvrs_return: *mut Result<bool>);
// onEncoded(const std::vector<std::vector<uint8_t>> &, const std::vector<uint64_t> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:215
// ("cv::cudacodec::EncoderCallback::onEncoded", vec![(pred!(mut, ["vPacket", "pts"], ["const std::vector<std::vector<uint8_t>>*", "const std::vector<uint64_t>*"]), _)]),
pub fn cv_cudacodec_EncoderCallback_onEncoded_const_vectorLvectorLuint8_tGGR_const_vectorLuint64_tGR(instance: *mut c_void, v_packet: *const c_void, pts: *const c_void, ocvrs_return: *mut Result<()>);
// setFrameIntervalP(const int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:221
// ("cv::cudacodec::EncoderCallback::setFrameIntervalP", vec![(pred!(mut, ["frameIntervalP"], ["const int"]), _)]),
pub fn cv_cudacodec_EncoderCallback_setFrameIntervalP_const_int(instance: *mut c_void, frame_interval_p: i32, ocvrs_return: *mut Result<bool>);
// onEncodingFinished()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:225
// ("cv::cudacodec::EncoderCallback::onEncodingFinished", vec![(pred!(mut, [], []), _)]),
pub fn cv_cudacodec_EncoderCallback_onEncodingFinished(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cudacodec::EncoderCallback::delete() generated
// ("cv::cudacodec::EncoderCallback::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cudacodec_EncoderCallback_delete(instance: *mut c_void);
// EncoderParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:187
// ("cv::cudacodec::EncoderParams::EncoderParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_cudacodec_EncoderParams_EncoderParams(ocvrs_return: *mut Result<crate::cudacodec::CUDA_EncoderParams>);
// FormatInfo()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:354
// ("cv::cudacodec::FormatInfo::FormatInfo", vec![(pred!(mut, [], []), _)]),
pub fn cv_cudacodec_FormatInfo_FormatInfo(ocvrs_return: *mut Result<crate::cudacodec::CUDA_FormatInfo>);
// convert(InputArray, OutputArray, const SurfaceFormat, const ColorFormat, const BitDepth, const bool, const bool, cuda::Stream &)(InputArray, OutputArray, Enum, Enum, Enum, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:397
// ("cv::cudacodec::NVSurfaceToColorConverter::convert", vec![(pred!(mut, ["yuv", "color", "surfaceFormat", "outputFormat", "bitDepth", "planar", "videoFullRangeFlag", "stream"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::cudacodec::SurfaceFormat", "const cv::cudacodec::ColorFormat", "const cv::cudacodec::BitDepth", "const bool", "const bool", "cv::cuda::Stream*"]), _)]),
pub fn cv_cudacodec_NVSurfaceToColorConverter_convert_const__InputArrayR_const__OutputArrayR_const_SurfaceFormat_const_ColorFormat_const_BitDepth_const_bool_const_bool_StreamR(instance: *mut c_void, yuv: *const c_void, color: *const c_void, surface_format: crate::cudacodec::CUDA_SurfaceFormat, output_format: crate::cudacodec::CUDA_ColorFormat, bit_depth: crate::cudacodec::CUDA_BitDepth, planar: bool, video_full_range_flag: bool, stream: *mut c_void, ocvrs_return: *mut Result<bool>);
// cv::cudacodec::NVSurfaceToColorConverter::convert(InputArray, OutputArray, Enum, Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:397
// ("cv::cudacodec::NVSurfaceToColorConverter::convert", vec![(pred!(mut, ["yuv", "color", "surfaceFormat", "outputFormat"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::cudacodec::SurfaceFormat", "const cv::cudacodec::ColorFormat"]), _)]),
pub fn cv_cudacodec_NVSurfaceToColorConverter_convert_const__InputArrayR_const__OutputArrayR_const_SurfaceFormat_const_ColorFormat(instance: *mut c_void, yuv: *const c_void, color: *const c_void, surface_format: crate::cudacodec::CUDA_SurfaceFormat, output_format: crate::cudacodec::CUDA_ColorFormat, ocvrs_return: *mut Result<bool>);
// cv::cudacodec::NVSurfaceToColorConverter::delete() generated
// ("cv::cudacodec::NVSurfaceToColorConverter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cudacodec_NVSurfaceToColorConverter_delete(instance: *mut c_void);
// getNextPacket(unsigned char **, size_t *)(Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:581
// ("cv::cudacodec::RawVideoSource::getNextPacket", vec![(pred!(mut, ["data", "size"], ["unsigned char**", "size_t*"]), _)]),
pub fn cv_cudacodec_RawVideoSource_getNextPacket_unsigned_charXX_size_tX(instance: *mut c_void, data: *mut *mut u8, size: *mut size_t, ocvrs_return: *mut Result<bool>);
// lastPacketContainsKeyFrame()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:585
// ("cv::cudacodec::RawVideoSource::lastPacketContainsKeyFrame", vec![(pred!(const, [], []), _)]),
pub fn cv_cudacodec_RawVideoSource_lastPacketContainsKeyFrame_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// format()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:589
// ("cv::cudacodec::RawVideoSource::format", vec![(pred!(const, [], []), _)]),
pub fn cv_cudacodec_RawVideoSource_format_const(instance: *const c_void, ocvrs_return: *mut Result<crate::cudacodec::CUDA_FormatInfo>);
// updateFormat(const FormatInfo &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:593
// ("cv::cudacodec::RawVideoSource::updateFormat", vec![(pred!(mut, ["videoFormat"], ["const cv::cudacodec::FormatInfo*"]), _)]),
pub fn cv_cudacodec_RawVideoSource_updateFormat_const_FormatInfoR(instance: *mut c_void, video_format: *const crate::cudacodec::CUDA_FormatInfo, ocvrs_return: *mut Result<()>);
// getExtraData(cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:599
// ("cv::cudacodec::RawVideoSource::getExtraData", vec![(pred!(const, ["extraData"], ["cv::Mat*"]), _)]),
pub fn cv_cudacodec_RawVideoSource_getExtraData_const_MatR(instance: *const c_void, extra_data: *mut c_void, ocvrs_return: *mut Result<()>);
// get(const int, double &)(Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:609
// ("cv::cudacodec::RawVideoSource::get", vec![(pred!(const, ["propertyId", "propertyVal"], ["const int", "double*"]), _)]),
pub fn cv_cudacodec_RawVideoSource_get_const_const_int_doubleR(instance: *const c_void, property_id: i32, property_val: *mut f64, ocvrs_return: *mut Result<bool>);
// getFirstFrameIdx()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:617
// ("cv::cudacodec::RawVideoSource::getFirstFrameIdx", vec![(pred!(const, [], []), _)]),
pub fn cv_cudacodec_RawVideoSource_getFirstFrameIdx_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::cudacodec::RawVideoSource::delete() generated
// ("cv::cudacodec::RawVideoSource::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cudacodec_RawVideoSource_delete(instance: *mut c_void);
// nextFrame(cuda::GpuMat &, cuda::Stream &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:449
// ("cv::cudacodec::VideoReader::nextFrame", vec![(pred!(mut, ["frame", "stream"], ["cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cudacodec_VideoReader_nextFrame_GpuMatR_StreamR(instance: *mut c_void, frame: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result<bool>);
// cv::cudacodec::VideoReader::nextFrame(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:449
// ("cv::cudacodec::VideoReader::nextFrame", vec![(pred!(mut, ["frame"], ["cv::cuda::GpuMat*"]), _)]),
pub fn cv_cudacodec_VideoReader_nextFrame_GpuMatR(instance: *mut c_void, frame: *mut c_void, ocvrs_return: *mut Result<bool>);
// nextFrame(cuda::GpuMat &, cuda::GpuMat &, cuda::Stream &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:463
// ("cv::cudacodec::VideoReader::nextFrame", vec![(pred!(mut, ["frame", "histogram", "stream"], ["cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cudacodec_VideoReader_nextFrame_GpuMatR_GpuMatR_StreamR(instance: *mut c_void, frame: *mut c_void, histogram: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result<bool>);
// cv::cudacodec::VideoReader::nextFrame(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:463
// ("cv::cudacodec::VideoReader::nextFrame", vec![(pred!(mut, ["frame", "histogram"], ["cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_cudacodec_VideoReader_nextFrame_GpuMatR_GpuMatR(instance: *mut c_void, frame: *mut c_void, histogram: *mut c_void, ocvrs_return: *mut Result<bool>);
// format()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:467
// ("cv::cudacodec::VideoReader::format", vec![(pred!(const, [], []), _)]),
pub fn cv_cudacodec_VideoReader_format_const(instance: *const c_void, ocvrs_return: *mut Result<crate::cudacodec::CUDA_FormatInfo>);
// grab(cuda::Stream &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:480
// ("cv::cudacodec::VideoReader::grab", vec![(pred!(mut, ["stream"], ["cv::cuda::Stream*"]), _)]),
pub fn cv_cudacodec_VideoReader_grab_StreamR(instance: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result<bool>);
// cv::cudacodec::VideoReader::grab() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:480
// ("cv::cudacodec::VideoReader::grab", vec![(pred!(mut, [], []), _)]),
pub fn cv_cudacodec_VideoReader_grab(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
// retrieve(OutputArray, const size_t)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:494
// ("cv::cudacodec::VideoReader::retrieve", vec![(pred!(const, ["frame", "idx"], ["const cv::_OutputArray*", "const size_t"]), _)]),
pub fn cv_cudacodec_VideoReader_retrieve_const_const__OutputArrayR_const_size_t(instance: *const c_void, frame: *const c_void, idx: size_t, ocvrs_return: *mut Result<bool>);
// cv::cudacodec::VideoReader::retrieve(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:494
// ("cv::cudacodec::VideoReader::retrieve", vec![(pred!(const, ["frame"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_cudacodec_VideoReader_retrieve_const_const__OutputArrayR(instance: *const c_void, frame: *const c_void, ocvrs_return: *mut Result<bool>);
// retrieve(Mat &, const size_t)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:507
// ("cv::cudacodec::VideoReader::retrieve", vec![(pred!(const, ["frame", "idx"], ["cv::Mat*", "const size_t"]), _)]),
pub fn cv_cudacodec_VideoReader_retrieve_const_MatR_const_size_t(instance: *const c_void, frame: *mut c_void, idx: size_t, ocvrs_return: *mut Result<bool>);
// retrieve(cuda::GpuMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:519
// ("cv::cudacodec::VideoReader::retrieve", vec![(pred!(const, ["frame"], ["cv::cuda::GpuMat*"]), _)]),
pub fn cv_cudacodec_VideoReader_retrieve_const_GpuMatR(instance: *const c_void, frame: *mut c_void, ocvrs_return: *mut Result<bool>);
// set(const VideoReaderProps, const double)(Enum, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:530
// ("cv::cudacodec::VideoReader::set", vec![(pred!(mut, ["propertyId", "propertyVal"], ["const cv::cudacodec::VideoReaderProps", "const double"]), _)]),
pub fn cv_cudacodec_VideoReader_set_const_VideoReaderProps_const_double(instance: *mut c_void, property_id: crate::cudacodec::CUDA_VideoReaderProps, property_val: f64, ocvrs_return: *mut Result<bool>);
// setVideoReaderProps(const VideoReaderProps, double)(Enum, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:531
// ("cv::cudacodec::VideoReader::setVideoReaderProps", vec![(pred!(mut, ["propertyId", "propertyVal"], ["const cv::cudacodec::VideoReaderProps", "double"]), _)]),
pub fn cv_cudacodec_VideoReader_setVideoReaderProps_const_VideoReaderProps_double(instance: *mut c_void, property_id: crate::cudacodec::CUDA_VideoReaderProps, property_val: f64, ocvrs_return: *mut Result<bool>);
// set(const ColorFormat, const BitDepth, const bool)(Enum, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:542
// ("cv::cudacodec::VideoReader::set", vec![(pred!(mut, ["colorFormat", "bitDepth", "planar"], ["const cv::cudacodec::ColorFormat", "const cv::cudacodec::BitDepth", "const bool"]), _)]),
pub fn cv_cudacodec_VideoReader_set_const_ColorFormat_const_BitDepth_const_bool(instance: *mut c_void, color_format: crate::cudacodec::CUDA_ColorFormat, bit_depth: crate::cudacodec::CUDA_BitDepth, planar: bool, ocvrs_return: *mut Result<bool>);
// cv::cudacodec::VideoReader::set(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:542
// ("cv::cudacodec::VideoReader::set", vec![(pred!(mut, ["colorFormat"], ["const cv::cudacodec::ColorFormat"]), _)]),
pub fn cv_cudacodec_VideoReader_set_const_ColorFormat(instance: *mut c_void, color_format: crate::cudacodec::CUDA_ColorFormat, ocvrs_return: *mut Result<bool>);
// get(const VideoReaderProps, double &)(Enum, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:553
// ("cv::cudacodec::VideoReader::get", vec![(pred!(const, ["propertyId", "propertyVal"], ["const cv::cudacodec::VideoReaderProps", "double*"]), _)]),
pub fn cv_cudacodec_VideoReader_get_const_const_VideoReaderProps_doubleR(instance: *const c_void, property_id: crate::cudacodec::CUDA_VideoReaderProps, property_val: *mut f64, ocvrs_return: *mut Result<bool>);
// getVideoReaderProps(const VideoReaderProps, double &, double)(Enum, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:554
// ("cv::cudacodec::VideoReader::getVideoReaderProps", vec![(pred!(const, ["propertyId", "propertyValOut", "propertyValIn"], ["const cv::cudacodec::VideoReaderProps", "double*", "double"]), _)]),
pub fn cv_cudacodec_VideoReader_getVideoReaderProps_const_const_VideoReaderProps_doubleR_double(instance: *const c_void, property_id: crate::cudacodec::CUDA_VideoReaderProps, property_val_out: *mut f64, property_val_in: f64, ocvrs_return: *mut Result<bool>);
// cv::cudacodec::VideoReader::getVideoReaderProps(Enum, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:554
// ("cv::cudacodec::VideoReader::getVideoReaderProps", vec![(pred!(const, ["propertyId", "propertyValOut"], ["const cv::cudacodec::VideoReaderProps", "double*"]), _)]),
pub fn cv_cudacodec_VideoReader_getVideoReaderProps_const_const_VideoReaderProps_doubleR(instance: *const c_void, property_id: crate::cudacodec::CUDA_VideoReaderProps, property_val_out: *mut f64, ocvrs_return: *mut Result<bool>);
// get(const int, double &)(Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:564
// ("cv::cudacodec::VideoReader::get", vec![(pred!(const, ["propertyId", "propertyVal"], ["const int", "double*"]), _)]),
pub fn cv_cudacodec_VideoReader_get_const_const_int_doubleR(instance: *const c_void, property_id: i32, property_val: *mut f64, ocvrs_return: *mut Result<bool>);
// cv::cudacodec::VideoReader::delete() generated
// ("cv::cudacodec::VideoReader::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cudacodec_VideoReader_delete(instance: *mut c_void);
// VideoReaderInitParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:638
// ("cv::cudacodec::VideoReaderInitParams::VideoReaderInitParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_cudacodec_VideoReaderInitParams_VideoReaderInitParams(ocvrs_return: *mut Result<crate::cudacodec::CUDA_VideoReaderInitParams>);
// write(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:252
// ("cv::cudacodec::VideoWriter::write", vec![(pred!(mut, ["frame"], ["const cv::_InputArray*"]), _)]),
pub fn cv_cudacodec_VideoWriter_write_const__InputArrayR(instance: *mut c_void, frame: *const c_void, ocvrs_return: *mut Result<()>);
// getEncoderParams()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:256
// ("cv::cudacodec::VideoWriter::getEncoderParams", vec![(pred!(const, [], []), _)]),
pub fn cv_cudacodec_VideoWriter_getEncoderParams_const(instance: *const c_void, ocvrs_return: *mut Result<crate::cudacodec::CUDA_EncoderParams>);
// release()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/cudacodec.hpp:260
// ("cv::cudacodec::VideoWriter::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_cudacodec_VideoWriter_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cudacodec::VideoWriter::delete() generated
// ("cv::cudacodec::VideoWriter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cudacodec_VideoWriter_delete(instance: *mut c_void);
