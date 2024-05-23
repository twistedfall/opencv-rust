// MapHist(const cuda::GpuMat &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:321
// ("cv::cudacodec::MapHist", vec![(pred!(mut, ["hist", "histFull"], ["const cv::cuda::GpuMat*", "cv::Mat*"]), _)]),
pub fn cv_cudacodec_MapHist_const_GpuMatR_MatR(hist: *const c_void, hist_full: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cudacodec::createVideoReader(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:611
// ("cv::cudacodec::createVideoReader", vec![(pred!(mut, ["source"], ["const cv::Ptr<cv::cudacodec::RawVideoSource>*"]), _)]),
pub fn cv_cudacodec_createVideoReader_const_PtrLRawVideoSourceGR(source: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// createVideoReader(const Ptr<RawVideoSource> &, const VideoReaderInitParams)(CppPassByVoidPtr, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:611
// ("cv::cudacodec::createVideoReader", vec![(pred!(mut, ["source", "params"], ["const cv::Ptr<cv::cudacodec::RawVideoSource>*", "const cv::cudacodec::VideoReaderInitParams"]), _)]),
pub fn cv_cudacodec_createVideoReader_const_PtrLRawVideoSourceGR_const_VideoReaderInitParams(source: *const c_void, params: *const crate::cudacodec::CUDA_VideoReaderInitParams, ocvrs_return: *mut Result<*mut c_void>);
// cv::cudacodec::createVideoReader(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:605
// ("cv::cudacodec::createVideoReader", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_cudacodec_createVideoReader_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// createVideoReader(const String &, const std::vector<int> &, const VideoReaderInitParams)(InString, CppPassByVoidPtr, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:605
// ("cv::cudacodec::createVideoReader", vec![(pred!(mut, ["filename", "sourceParams", "params"], ["const cv::String*", "const std::vector<int>*", "const cv::cudacodec::VideoReaderInitParams"]), _)]),
pub fn cv_cudacodec_createVideoReader_const_StringR_const_vectorLintGR_const_VideoReaderInitParams(filename: *const c_char, source_params: *const c_void, params: *const crate::cudacodec::CUDA_VideoReaderInitParams, ocvrs_return: *mut Result<*mut c_void>);
// cv::cudacodec::createVideoWriter(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:271
// ("cv::cudacodec::createVideoWriter", vec![(pred!(mut, ["fileName", "frameSize"], ["const cv::String*", "const cv::Size"]), _)]),
pub fn cv_cudacodec_createVideoWriter_const_StringR_const_Size(file_name: *const c_char, frame_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// createVideoWriter(const String &, const Size, const Codec, const double, const ColorFormat, Ptr<EncoderCallback>, const cuda::Stream &)(InString, SimpleClass, Enum, Primitive, Enum, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:271
// ("cv::cudacodec::createVideoWriter", vec![(pred!(mut, ["fileName", "frameSize", "codec", "fps", "colorFormat", "encoderCallback", "stream"], ["const cv::String*", "const cv::Size", "const cv::cudacodec::Codec", "const double", "const cv::cudacodec::ColorFormat", "cv::Ptr<cv::cudacodec::EncoderCallback>", "const cv::cuda::Stream*"]), _)]),
pub fn cv_cudacodec_createVideoWriter_const_StringR_const_Size_const_Codec_const_double_const_ColorFormat_PtrLEncoderCallbackG_const_StreamR(file_name: *const c_char, frame_size: *const core::Size, codec: crate::cudacodec::CUDA_Codec, fps: f64, color_format: crate::cudacodec::CUDA_ColorFormat, encoder_callback: *mut c_void, stream: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::cudacodec::createVideoWriter(InString, SimpleClass, Enum, Primitive, Enum, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:285
// ("cv::cudacodec::createVideoWriter", vec![(pred!(mut, ["fileName", "frameSize", "codec", "fps", "colorFormat", "params"], ["const cv::String*", "const cv::Size", "const cv::cudacodec::Codec", "const double", "const cv::cudacodec::ColorFormat", "const cv::cudacodec::EncoderParams*"]), _)]),
pub fn cv_cudacodec_createVideoWriter_const_StringR_const_Size_const_Codec_const_double_const_ColorFormat_const_EncoderParamsR(file_name: *const c_char, frame_size: *const core::Size, codec: crate::cudacodec::CUDA_Codec, fps: f64, color_format: crate::cudacodec::CUDA_ColorFormat, params: *const crate::cudacodec::CUDA_EncoderParams, ocvrs_return: *mut Result<*mut c_void>);
// createVideoWriter(const String &, const Size, const Codec, const double, const ColorFormat, const EncoderParams &, Ptr<EncoderCallback>, const cuda::Stream &)(InString, SimpleClass, Enum, Primitive, Enum, SimpleClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:285
// ("cv::cudacodec::createVideoWriter", vec![(pred!(mut, ["fileName", "frameSize", "codec", "fps", "colorFormat", "params", "encoderCallback", "stream"], ["const cv::String*", "const cv::Size", "const cv::cudacodec::Codec", "const double", "const cv::cudacodec::ColorFormat", "const cv::cudacodec::EncoderParams*", "cv::Ptr<cv::cudacodec::EncoderCallback>", "const cv::cuda::Stream*"]), _)]),
pub fn cv_cudacodec_createVideoWriter_const_StringR_const_Size_const_Codec_const_double_const_ColorFormat_const_EncoderParamsR_PtrLEncoderCallbackG_const_StreamR(file_name: *const c_char, frame_size: *const core::Size, codec: crate::cudacodec::CUDA_Codec, fps: f64, color_format: crate::cudacodec::CUDA_ColorFormat, params: *const crate::cudacodec::CUDA_EncoderParams, encoder_callback: *mut c_void, stream: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator==(const EncoderParams &, const EncoderParams &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:200
// ("cv::cudacodec::operator==", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::cudacodec::EncoderParams*", "const cv::cudacodec::EncoderParams*"]), _)]),
pub fn cv_cudacodec_operatorEQ_const_EncoderParamsR_const_EncoderParamsR(lhs: *const crate::cudacodec::CUDA_EncoderParams, rhs: *const crate::cudacodec::CUDA_EncoderParams, ocvrs_return: *mut Result<bool>);
// onEncoded(const std::vector<std::vector<uint8_t>> &, const std::vector<uint64_t> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:213
// ("cv::cudacodec::EncoderCallback::onEncoded", vec![(pred!(mut, ["vPacket", "pts"], ["const std::vector<std::vector<uint8_t>>*", "const std::vector<uint64_t>*"]), _)]),
pub fn cv_cudacodec_EncoderCallback_onEncoded_const_vectorLvectorLuint8_tGGR_const_vectorLuint64_tGR(instance: *mut c_void, v_packet: *const c_void, pts: *const c_void, ocvrs_return: *mut Result<()>);
// setFrameIntervalP(const int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:219
// ("cv::cudacodec::EncoderCallback::setFrameIntervalP", vec![(pred!(mut, ["frameIntervalP"], ["const int"]), _)]),
pub fn cv_cudacodec_EncoderCallback_setFrameIntervalP_const_int(instance: *mut c_void, frame_interval_p: i32, ocvrs_return: *mut Result<bool>);
// onEncodingFinished()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:223
// ("cv::cudacodec::EncoderCallback::onEncodingFinished", vec![(pred!(mut, [], []), _)]),
pub fn cv_cudacodec_EncoderCallback_onEncodingFinished(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cudacodec::EncoderCallback::delete() generated
// ("cv::cudacodec::EncoderCallback::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cudacodec_EncoderCallback_delete(instance: *mut c_void);
// EncoderParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:185
// ("cv::cudacodec::EncoderParams::EncoderParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_cudacodec_EncoderParams_EncoderParams(ocvrs_return: *mut Result<crate::cudacodec::CUDA_EncoderParams>);
// FormatInfo()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:327
// ("cv::cudacodec::FormatInfo::FormatInfo", vec![(pred!(mut, [], []), _)]),
pub fn cv_cudacodec_FormatInfo_FormatInfo(ocvrs_return: *mut Result<crate::cudacodec::CUDA_FormatInfo>);
// getNextPacket(unsigned char **, size_t *)(Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:525
// ("cv::cudacodec::RawVideoSource::getNextPacket", vec![(pred!(mut, ["data", "size"], ["unsigned char**", "size_t*"]), _)]),
pub fn cv_cudacodec_RawVideoSource_getNextPacket_unsigned_charXX_size_tX(instance: *mut c_void, data: *mut *mut u8, size: *mut size_t, ocvrs_return: *mut Result<bool>);
// lastPacketContainsKeyFrame()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:529
// ("cv::cudacodec::RawVideoSource::lastPacketContainsKeyFrame", vec![(pred!(const, [], []), _)]),
pub fn cv_cudacodec_RawVideoSource_lastPacketContainsKeyFrame_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// format()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:533
// ("cv::cudacodec::RawVideoSource::format", vec![(pred!(const, [], []), _)]),
pub fn cv_cudacodec_RawVideoSource_format_const(instance: *const c_void, ocvrs_return: *mut Result<crate::cudacodec::CUDA_FormatInfo>);
// updateFormat(const FormatInfo &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:537
// ("cv::cudacodec::RawVideoSource::updateFormat", vec![(pred!(mut, ["videoFormat"], ["const cv::cudacodec::FormatInfo*"]), _)]),
pub fn cv_cudacodec_RawVideoSource_updateFormat_const_FormatInfoR(instance: *mut c_void, video_format: *const crate::cudacodec::CUDA_FormatInfo, ocvrs_return: *mut Result<()>);
// getExtraData(cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:543
// ("cv::cudacodec::RawVideoSource::getExtraData", vec![(pred!(const, ["extraData"], ["cv::Mat*"]), _)]),
pub fn cv_cudacodec_RawVideoSource_getExtraData_const_MatR(instance: *const c_void, extra_data: *mut c_void, ocvrs_return: *mut Result<()>);
// get(const int, double &)(Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:553
// ("cv::cudacodec::RawVideoSource::get", vec![(pred!(const, ["propertyId", "propertyVal"], ["const int", "double*"]), _)]),
pub fn cv_cudacodec_RawVideoSource_get_const_const_int_doubleR(instance: *const c_void, property_id: i32, property_val: *mut f64, ocvrs_return: *mut Result<bool>);
// getFirstFrameIdx()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:561
// ("cv::cudacodec::RawVideoSource::getFirstFrameIdx", vec![(pred!(const, [], []), _)]),
pub fn cv_cudacodec_RawVideoSource_getFirstFrameIdx_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::cudacodec::RawVideoSource::delete() generated
// ("cv::cudacodec::RawVideoSource::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cudacodec_RawVideoSource_delete(instance: *mut c_void);
// nextFrame(cuda::GpuMat &, cuda::Stream &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:395
// ("cv::cudacodec::VideoReader::nextFrame", vec![(pred!(mut, ["frame", "stream"], ["cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cudacodec_VideoReader_nextFrame_GpuMatR_StreamR(instance: *mut c_void, frame: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result<bool>);
// cv::cudacodec::VideoReader::nextFrame(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:395
// ("cv::cudacodec::VideoReader::nextFrame", vec![(pred!(mut, ["frame"], ["cv::cuda::GpuMat*"]), _)]),
pub fn cv_cudacodec_VideoReader_nextFrame_GpuMatR(instance: *mut c_void, frame: *mut c_void, ocvrs_return: *mut Result<bool>);
// nextFrame(cuda::GpuMat &, cuda::GpuMat &, cuda::Stream &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:409
// ("cv::cudacodec::VideoReader::nextFrame", vec![(pred!(mut, ["frame", "histogram", "stream"], ["cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
pub fn cv_cudacodec_VideoReader_nextFrame_GpuMatR_GpuMatR_StreamR(instance: *mut c_void, frame: *mut c_void, histogram: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result<bool>);
// cv::cudacodec::VideoReader::nextFrame(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:409
// ("cv::cudacodec::VideoReader::nextFrame", vec![(pred!(mut, ["frame", "histogram"], ["cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
pub fn cv_cudacodec_VideoReader_nextFrame_GpuMatR_GpuMatR(instance: *mut c_void, frame: *mut c_void, histogram: *mut c_void, ocvrs_return: *mut Result<bool>);
// format()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:413
// ("cv::cudacodec::VideoReader::format", vec![(pred!(const, [], []), _)]),
pub fn cv_cudacodec_VideoReader_format_const(instance: *const c_void, ocvrs_return: *mut Result<crate::cudacodec::CUDA_FormatInfo>);
// grab(cuda::Stream &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:426
// ("cv::cudacodec::VideoReader::grab", vec![(pred!(mut, ["stream"], ["cv::cuda::Stream*"]), _)]),
pub fn cv_cudacodec_VideoReader_grab_StreamR(instance: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result<bool>);
// cv::cudacodec::VideoReader::grab() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:426
// ("cv::cudacodec::VideoReader::grab", vec![(pred!(mut, [], []), _)]),
pub fn cv_cudacodec_VideoReader_grab(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
// retrieve(OutputArray, const size_t)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:440
// ("cv::cudacodec::VideoReader::retrieve", vec![(pred!(const, ["frame", "idx"], ["const cv::_OutputArray*", "const size_t"]), _)]),
pub fn cv_cudacodec_VideoReader_retrieve_const_const__OutputArrayR_const_size_t(instance: *const c_void, frame: *const c_void, idx: size_t, ocvrs_return: *mut Result<bool>);
// cv::cudacodec::VideoReader::retrieve(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:440
// ("cv::cudacodec::VideoReader::retrieve", vec![(pred!(const, ["frame"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_cudacodec_VideoReader_retrieve_const_const__OutputArrayR(instance: *const c_void, frame: *const c_void, ocvrs_return: *mut Result<bool>);
// retrieve(Mat &, const size_t)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:453
// ("cv::cudacodec::VideoReader::retrieve", vec![(pred!(const, ["frame", "idx"], ["cv::Mat*", "const size_t"]), _)]),
pub fn cv_cudacodec_VideoReader_retrieve_const_MatR_const_size_t(instance: *const c_void, frame: *mut c_void, idx: size_t, ocvrs_return: *mut Result<bool>);
// retrieve(cuda::GpuMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:465
// ("cv::cudacodec::VideoReader::retrieve", vec![(pred!(const, ["frame"], ["cv::cuda::GpuMat*"]), _)]),
pub fn cv_cudacodec_VideoReader_retrieve_const_GpuMatR(instance: *const c_void, frame: *mut c_void, ocvrs_return: *mut Result<bool>);
// set(const VideoReaderProps, const double)(Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:476
// ("cv::cudacodec::VideoReader::set", vec![(pred!(mut, ["propertyId", "propertyVal"], ["const cv::cudacodec::VideoReaderProps", "const double"]), _)]),
pub fn cv_cudacodec_VideoReader_set_const_VideoReaderProps_const_double(instance: *mut c_void, property_id: crate::cudacodec::CUDA_VideoReaderProps, property_val: f64, ocvrs_return: *mut Result<bool>);
// setVideoReaderProps(const VideoReaderProps, double)(Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:477
// ("cv::cudacodec::VideoReader::setVideoReaderProps", vec![(pred!(mut, ["propertyId", "propertyVal"], ["const cv::cudacodec::VideoReaderProps", "double"]), _)]),
pub fn cv_cudacodec_VideoReader_setVideoReaderProps_const_VideoReaderProps_double(instance: *mut c_void, property_id: crate::cudacodec::CUDA_VideoReaderProps, property_val: f64, ocvrs_return: *mut Result<bool>);
// set(const ColorFormat)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:486
// ("cv::cudacodec::VideoReader::set", vec![(pred!(mut, ["colorFormat"], ["const cv::cudacodec::ColorFormat"]), _)]),
pub fn cv_cudacodec_VideoReader_set_const_ColorFormat(instance: *mut c_void, color_format: crate::cudacodec::CUDA_ColorFormat, ocvrs_return: *mut Result<bool>);
// get(const VideoReaderProps, double &)(Enum, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:497
// ("cv::cudacodec::VideoReader::get", vec![(pred!(const, ["propertyId", "propertyVal"], ["const cv::cudacodec::VideoReaderProps", "double*"]), _)]),
pub fn cv_cudacodec_VideoReader_get_const_const_VideoReaderProps_doubleR(instance: *const c_void, property_id: crate::cudacodec::CUDA_VideoReaderProps, property_val: *mut f64, ocvrs_return: *mut Result<bool>);
// getVideoReaderProps(const VideoReaderProps, double &, double)(Enum, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:498
// ("cv::cudacodec::VideoReader::getVideoReaderProps", vec![(pred!(const, ["propertyId", "propertyValOut", "propertyValIn"], ["const cv::cudacodec::VideoReaderProps", "double*", "double"]), _)]),
pub fn cv_cudacodec_VideoReader_getVideoReaderProps_const_const_VideoReaderProps_doubleR_double(instance: *const c_void, property_id: crate::cudacodec::CUDA_VideoReaderProps, property_val_out: *mut f64, property_val_in: f64, ocvrs_return: *mut Result<bool>);
// cv::cudacodec::VideoReader::getVideoReaderProps(Enum, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:498
// ("cv::cudacodec::VideoReader::getVideoReaderProps", vec![(pred!(const, ["propertyId", "propertyValOut"], ["const cv::cudacodec::VideoReaderProps", "double*"]), _)]),
pub fn cv_cudacodec_VideoReader_getVideoReaderProps_const_const_VideoReaderProps_doubleR(instance: *const c_void, property_id: crate::cudacodec::CUDA_VideoReaderProps, property_val_out: *mut f64, ocvrs_return: *mut Result<bool>);
// get(const int, double &)(Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:508
// ("cv::cudacodec::VideoReader::get", vec![(pred!(const, ["propertyId", "propertyVal"], ["const int", "double*"]), _)]),
pub fn cv_cudacodec_VideoReader_get_const_const_int_doubleR(instance: *const c_void, property_id: i32, property_val: *mut f64, ocvrs_return: *mut Result<bool>);
// cv::cudacodec::VideoReader::delete() generated
// ("cv::cudacodec::VideoReader::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cudacodec_VideoReader_delete(instance: *mut c_void);
// VideoReaderInitParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:582
// ("cv::cudacodec::VideoReaderInitParams::VideoReaderInitParams", vec![(pred!(mut, [], []), _)]),
pub fn cv_cudacodec_VideoReaderInitParams_VideoReaderInitParams(ocvrs_return: *mut Result<crate::cudacodec::CUDA_VideoReaderInitParams>);
// write(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:250
// ("cv::cudacodec::VideoWriter::write", vec![(pred!(mut, ["frame"], ["const cv::_InputArray*"]), _)]),
pub fn cv_cudacodec_VideoWriter_write_const__InputArrayR(instance: *mut c_void, frame: *const c_void, ocvrs_return: *mut Result<()>);
// getEncoderParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:254
// ("cv::cudacodec::VideoWriter::getEncoderParams", vec![(pred!(const, [], []), _)]),
pub fn cv_cudacodec_VideoWriter_getEncoderParams_const(instance: *const c_void, ocvrs_return: *mut Result<crate::cudacodec::CUDA_EncoderParams>);
// release()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:258
// ("cv::cudacodec::VideoWriter::release", vec![(pred!(mut, [], []), _)]),
pub fn cv_cudacodec_VideoWriter_release(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::cudacodec::VideoWriter::delete() generated
// ("cv::cudacodec::VideoWriter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_cudacodec_VideoWriter_delete(instance: *mut c_void);
