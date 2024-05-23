#include "ocvrs_common.hpp"
#include <opencv2/cudacodec.hpp>
#include "cudacodec_types.hpp"

extern "C" {
	// MapHist(const cuda::GpuMat &, Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:321
	// ("cv::cudacodec::MapHist", vec![(pred!(mut, ["hist", "histFull"], ["const cv::cuda::GpuMat*", "cv::Mat*"]), _)]),
	void cv_cudacodec_MapHist_const_GpuMatR_MatR(const cv::cuda::GpuMat* hist, cv::Mat* histFull, ResultVoid* ocvrs_return) {
		try {
			cv::cudacodec::MapHist(*hist, *histFull);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cudacodec::createVideoReader(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:611
	// ("cv::cudacodec::createVideoReader", vec![(pred!(mut, ["source"], ["const cv::Ptr<cv::cudacodec::RawVideoSource>*"]), _)]),
	void cv_cudacodec_createVideoReader_const_PtrLRawVideoSourceGR(const cv::Ptr<cv::cudacodec::RawVideoSource>* source, Result<cv::Ptr<cv::cudacodec::VideoReader>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cudacodec::VideoReader> ret = cv::cudacodec::createVideoReader(*source);
			Ok(new cv::Ptr<cv::cudacodec::VideoReader>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createVideoReader(const Ptr<RawVideoSource> &, const VideoReaderInitParams)(CppPassByVoidPtr, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:611
	// ("cv::cudacodec::createVideoReader", vec![(pred!(mut, ["source", "params"], ["const cv::Ptr<cv::cudacodec::RawVideoSource>*", "const cv::cudacodec::VideoReaderInitParams"]), _)]),
	void cv_cudacodec_createVideoReader_const_PtrLRawVideoSourceGR_const_VideoReaderInitParams(const cv::Ptr<cv::cudacodec::RawVideoSource>* source, const cv::cudacodec::VideoReaderInitParams* params, Result<cv::Ptr<cv::cudacodec::VideoReader>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cudacodec::VideoReader> ret = cv::cudacodec::createVideoReader(*source, *params);
			Ok(new cv::Ptr<cv::cudacodec::VideoReader>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cudacodec::createVideoReader(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:605
	// ("cv::cudacodec::createVideoReader", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_cudacodec_createVideoReader_const_StringR(const char* filename, Result<cv::Ptr<cv::cudacodec::VideoReader>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cudacodec::VideoReader> ret = cv::cudacodec::createVideoReader(std::string(filename));
			Ok(new cv::Ptr<cv::cudacodec::VideoReader>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createVideoReader(const String &, const std::vector<int> &, const VideoReaderInitParams)(InString, CppPassByVoidPtr, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:605
	// ("cv::cudacodec::createVideoReader", vec![(pred!(mut, ["filename", "sourceParams", "params"], ["const cv::String*", "const std::vector<int>*", "const cv::cudacodec::VideoReaderInitParams"]), _)]),
	void cv_cudacodec_createVideoReader_const_StringR_const_vectorLintGR_const_VideoReaderInitParams(const char* filename, const std::vector<int>* sourceParams, const cv::cudacodec::VideoReaderInitParams* params, Result<cv::Ptr<cv::cudacodec::VideoReader>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cudacodec::VideoReader> ret = cv::cudacodec::createVideoReader(std::string(filename), *sourceParams, *params);
			Ok(new cv::Ptr<cv::cudacodec::VideoReader>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cudacodec::createVideoWriter(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:271
	// ("cv::cudacodec::createVideoWriter", vec![(pred!(mut, ["fileName", "frameSize"], ["const cv::String*", "const cv::Size"]), _)]),
	void cv_cudacodec_createVideoWriter_const_StringR_const_Size(const char* fileName, const cv::Size* frameSize, Result<cv::Ptr<cv::cudacodec::VideoWriter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cudacodec::VideoWriter> ret = cv::cudacodec::createVideoWriter(std::string(fileName), *frameSize);
			Ok(new cv::Ptr<cv::cudacodec::VideoWriter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createVideoWriter(const String &, const Size, const Codec, const double, const ColorFormat, Ptr<EncoderCallback>, const cuda::Stream &)(InString, SimpleClass, Enum, Primitive, Enum, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:271
	// ("cv::cudacodec::createVideoWriter", vec![(pred!(mut, ["fileName", "frameSize", "codec", "fps", "colorFormat", "encoderCallback", "stream"], ["const cv::String*", "const cv::Size", "const cv::cudacodec::Codec", "const double", "const cv::cudacodec::ColorFormat", "cv::Ptr<cv::cudacodec::EncoderCallback>", "const cv::cuda::Stream*"]), _)]),
	void cv_cudacodec_createVideoWriter_const_StringR_const_Size_const_Codec_const_double_const_ColorFormat_PtrLEncoderCallbackG_const_StreamR(const char* fileName, const cv::Size* frameSize, const cv::cudacodec::Codec codec, const double fps, const cv::cudacodec::ColorFormat colorFormat, cv::Ptr<cv::cudacodec::EncoderCallback>* encoderCallback, const cv::cuda::Stream* stream, Result<cv::Ptr<cv::cudacodec::VideoWriter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cudacodec::VideoWriter> ret = cv::cudacodec::createVideoWriter(std::string(fileName), *frameSize, codec, fps, colorFormat, *encoderCallback, *stream);
			Ok(new cv::Ptr<cv::cudacodec::VideoWriter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cudacodec::createVideoWriter(InString, SimpleClass, Enum, Primitive, Enum, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:285
	// ("cv::cudacodec::createVideoWriter", vec![(pred!(mut, ["fileName", "frameSize", "codec", "fps", "colorFormat", "params"], ["const cv::String*", "const cv::Size", "const cv::cudacodec::Codec", "const double", "const cv::cudacodec::ColorFormat", "const cv::cudacodec::EncoderParams*"]), _)]),
	void cv_cudacodec_createVideoWriter_const_StringR_const_Size_const_Codec_const_double_const_ColorFormat_const_EncoderParamsR(const char* fileName, const cv::Size* frameSize, const cv::cudacodec::Codec codec, const double fps, const cv::cudacodec::ColorFormat colorFormat, const cv::cudacodec::EncoderParams* params, Result<cv::Ptr<cv::cudacodec::VideoWriter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cudacodec::VideoWriter> ret = cv::cudacodec::createVideoWriter(std::string(fileName), *frameSize, codec, fps, colorFormat, *params);
			Ok(new cv::Ptr<cv::cudacodec::VideoWriter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createVideoWriter(const String &, const Size, const Codec, const double, const ColorFormat, const EncoderParams &, Ptr<EncoderCallback>, const cuda::Stream &)(InString, SimpleClass, Enum, Primitive, Enum, SimpleClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:285
	// ("cv::cudacodec::createVideoWriter", vec![(pred!(mut, ["fileName", "frameSize", "codec", "fps", "colorFormat", "params", "encoderCallback", "stream"], ["const cv::String*", "const cv::Size", "const cv::cudacodec::Codec", "const double", "const cv::cudacodec::ColorFormat", "const cv::cudacodec::EncoderParams*", "cv::Ptr<cv::cudacodec::EncoderCallback>", "const cv::cuda::Stream*"]), _)]),
	void cv_cudacodec_createVideoWriter_const_StringR_const_Size_const_Codec_const_double_const_ColorFormat_const_EncoderParamsR_PtrLEncoderCallbackG_const_StreamR(const char* fileName, const cv::Size* frameSize, const cv::cudacodec::Codec codec, const double fps, const cv::cudacodec::ColorFormat colorFormat, const cv::cudacodec::EncoderParams* params, cv::Ptr<cv::cudacodec::EncoderCallback>* encoderCallback, const cv::cuda::Stream* stream, Result<cv::Ptr<cv::cudacodec::VideoWriter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cudacodec::VideoWriter> ret = cv::cudacodec::createVideoWriter(std::string(fileName), *frameSize, codec, fps, colorFormat, *params, *encoderCallback, *stream);
			Ok(new cv::Ptr<cv::cudacodec::VideoWriter>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// operator==(const EncoderParams &, const EncoderParams &)(SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:200
	// ("cv::cudacodec::operator==", vec![(pred!(mut, ["lhs", "rhs"], ["const cv::cudacodec::EncoderParams*", "const cv::cudacodec::EncoderParams*"]), _)]),
	void cv_cudacodec_operatorEQ_const_EncoderParamsR_const_EncoderParamsR(const cv::cudacodec::EncoderParams* lhs, const cv::cudacodec::EncoderParams* rhs, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::cudacodec::operator==(*lhs, *rhs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// onEncoded(const std::vector<std::vector<uint8_t>> &, const std::vector<uint64_t> &)(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:213
	// ("cv::cudacodec::EncoderCallback::onEncoded", vec![(pred!(mut, ["vPacket", "pts"], ["const std::vector<std::vector<uint8_t>>*", "const std::vector<uint64_t>*"]), _)]),
	void cv_cudacodec_EncoderCallback_onEncoded_const_vectorLvectorLuint8_tGGR_const_vectorLuint64_tGR(cv::cudacodec::EncoderCallback* instance, const std::vector<std::vector<uint8_t>>* vPacket, const std::vector<uint64_t>* pts, ResultVoid* ocvrs_return) {
		try {
			instance->onEncoded(*vPacket, *pts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFrameIntervalP(const int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:219
	// ("cv::cudacodec::EncoderCallback::setFrameIntervalP", vec![(pred!(mut, ["frameIntervalP"], ["const int"]), _)]),
	void cv_cudacodec_EncoderCallback_setFrameIntervalP_const_int(cv::cudacodec::EncoderCallback* instance, const int frameIntervalP, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->setFrameIntervalP(frameIntervalP);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// onEncodingFinished()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:223
	// ("cv::cudacodec::EncoderCallback::onEncodingFinished", vec![(pred!(mut, [], []), _)]),
	void cv_cudacodec_EncoderCallback_onEncodingFinished(cv::cudacodec::EncoderCallback* instance, ResultVoid* ocvrs_return) {
		try {
			instance->onEncodingFinished();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cudacodec::EncoderCallback::delete() generated
	// ("cv::cudacodec::EncoderCallback::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cudacodec_EncoderCallback_delete(cv::cudacodec::EncoderCallback* instance) {
			delete instance;
	}

	// EncoderParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:185
	// ("cv::cudacodec::EncoderParams::EncoderParams", vec![(pred!(mut, [], []), _)]),
	void cv_cudacodec_EncoderParams_EncoderParams(Result<cv::cudacodec::EncoderParams>* ocvrs_return) {
		try {
			cv::cudacodec::EncoderParams ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// FormatInfo()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:327
	// ("cv::cudacodec::FormatInfo::FormatInfo", vec![(pred!(mut, [], []), _)]),
	void cv_cudacodec_FormatInfo_FormatInfo(Result<cv::cudacodec::FormatInfo>* ocvrs_return) {
		try {
			cv::cudacodec::FormatInfo ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNextPacket(unsigned char **, size_t *)(Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:525
	// ("cv::cudacodec::RawVideoSource::getNextPacket", vec![(pred!(mut, ["data", "size"], ["unsigned char**", "size_t*"]), _)]),
	void cv_cudacodec_RawVideoSource_getNextPacket_unsigned_charXX_size_tX(cv::cudacodec::RawVideoSource* instance, unsigned char** data, size_t* size, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getNextPacket(data, size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// lastPacketContainsKeyFrame()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:529
	// ("cv::cudacodec::RawVideoSource::lastPacketContainsKeyFrame", vec![(pred!(const, [], []), _)]),
	void cv_cudacodec_RawVideoSource_lastPacketContainsKeyFrame_const(const cv::cudacodec::RawVideoSource* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->lastPacketContainsKeyFrame();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// format()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:533
	// ("cv::cudacodec::RawVideoSource::format", vec![(pred!(const, [], []), _)]),
	void cv_cudacodec_RawVideoSource_format_const(const cv::cudacodec::RawVideoSource* instance, Result<cv::cudacodec::FormatInfo>* ocvrs_return) {
		try {
			cv::cudacodec::FormatInfo ret = instance->format();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// updateFormat(const FormatInfo &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:537
	// ("cv::cudacodec::RawVideoSource::updateFormat", vec![(pred!(mut, ["videoFormat"], ["const cv::cudacodec::FormatInfo*"]), _)]),
	void cv_cudacodec_RawVideoSource_updateFormat_const_FormatInfoR(cv::cudacodec::RawVideoSource* instance, const cv::cudacodec::FormatInfo* videoFormat, ResultVoid* ocvrs_return) {
		try {
			instance->updateFormat(*videoFormat);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getExtraData(cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:543
	// ("cv::cudacodec::RawVideoSource::getExtraData", vec![(pred!(const, ["extraData"], ["cv::Mat*"]), _)]),
	void cv_cudacodec_RawVideoSource_getExtraData_const_MatR(const cv::cudacodec::RawVideoSource* instance, cv::Mat* extraData, ResultVoid* ocvrs_return) {
		try {
			instance->getExtraData(*extraData);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// get(const int, double &)(Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:553
	// ("cv::cudacodec::RawVideoSource::get", vec![(pred!(const, ["propertyId", "propertyVal"], ["const int", "double*"]), _)]),
	void cv_cudacodec_RawVideoSource_get_const_const_int_doubleR(const cv::cudacodec::RawVideoSource* instance, const int propertyId, double* propertyVal, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->get(propertyId, *propertyVal);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFirstFrameIdx()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:561
	// ("cv::cudacodec::RawVideoSource::getFirstFrameIdx", vec![(pred!(const, [], []), _)]),
	void cv_cudacodec_RawVideoSource_getFirstFrameIdx_const(const cv::cudacodec::RawVideoSource* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFirstFrameIdx();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cudacodec::RawVideoSource::delete() generated
	// ("cv::cudacodec::RawVideoSource::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cudacodec_RawVideoSource_delete(cv::cudacodec::RawVideoSource* instance) {
			delete instance;
	}

	// nextFrame(cuda::GpuMat &, cuda::Stream &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:395
	// ("cv::cudacodec::VideoReader::nextFrame", vec![(pred!(mut, ["frame", "stream"], ["cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
	void cv_cudacodec_VideoReader_nextFrame_GpuMatR_StreamR(cv::cudacodec::VideoReader* instance, cv::cuda::GpuMat* frame, cv::cuda::Stream* stream, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->nextFrame(*frame, *stream);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cudacodec::VideoReader::nextFrame(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:395
	// ("cv::cudacodec::VideoReader::nextFrame", vec![(pred!(mut, ["frame"], ["cv::cuda::GpuMat*"]), _)]),
	void cv_cudacodec_VideoReader_nextFrame_GpuMatR(cv::cudacodec::VideoReader* instance, cv::cuda::GpuMat* frame, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->nextFrame(*frame);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// nextFrame(cuda::GpuMat &, cuda::GpuMat &, cuda::Stream &)(TraitClass, TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:409
	// ("cv::cudacodec::VideoReader::nextFrame", vec![(pred!(mut, ["frame", "histogram", "stream"], ["cv::cuda::GpuMat*", "cv::cuda::GpuMat*", "cv::cuda::Stream*"]), _)]),
	void cv_cudacodec_VideoReader_nextFrame_GpuMatR_GpuMatR_StreamR(cv::cudacodec::VideoReader* instance, cv::cuda::GpuMat* frame, cv::cuda::GpuMat* histogram, cv::cuda::Stream* stream, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->nextFrame(*frame, *histogram, *stream);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cudacodec::VideoReader::nextFrame(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:409
	// ("cv::cudacodec::VideoReader::nextFrame", vec![(pred!(mut, ["frame", "histogram"], ["cv::cuda::GpuMat*", "cv::cuda::GpuMat*"]), _)]),
	void cv_cudacodec_VideoReader_nextFrame_GpuMatR_GpuMatR(cv::cudacodec::VideoReader* instance, cv::cuda::GpuMat* frame, cv::cuda::GpuMat* histogram, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->nextFrame(*frame, *histogram);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// format()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:413
	// ("cv::cudacodec::VideoReader::format", vec![(pred!(const, [], []), _)]),
	void cv_cudacodec_VideoReader_format_const(const cv::cudacodec::VideoReader* instance, Result<cv::cudacodec::FormatInfo>* ocvrs_return) {
		try {
			cv::cudacodec::FormatInfo ret = instance->format();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// grab(cuda::Stream &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:426
	// ("cv::cudacodec::VideoReader::grab", vec![(pred!(mut, ["stream"], ["cv::cuda::Stream*"]), _)]),
	void cv_cudacodec_VideoReader_grab_StreamR(cv::cudacodec::VideoReader* instance, cv::cuda::Stream* stream, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->grab(*stream);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cudacodec::VideoReader::grab() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:426
	// ("cv::cudacodec::VideoReader::grab", vec![(pred!(mut, [], []), _)]),
	void cv_cudacodec_VideoReader_grab(cv::cudacodec::VideoReader* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->grab();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// retrieve(OutputArray, const size_t)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:440
	// ("cv::cudacodec::VideoReader::retrieve", vec![(pred!(const, ["frame", "idx"], ["const cv::_OutputArray*", "const size_t"]), _)]),
	void cv_cudacodec_VideoReader_retrieve_const_const__OutputArrayR_const_size_t(const cv::cudacodec::VideoReader* instance, const cv::_OutputArray* frame, const size_t idx, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->retrieve(*frame, idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cudacodec::VideoReader::retrieve(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:440
	// ("cv::cudacodec::VideoReader::retrieve", vec![(pred!(const, ["frame"], ["const cv::_OutputArray*"]), _)]),
	void cv_cudacodec_VideoReader_retrieve_const_const__OutputArrayR(const cv::cudacodec::VideoReader* instance, const cv::_OutputArray* frame, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->retrieve(*frame);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// retrieve(Mat &, const size_t)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:453
	// ("cv::cudacodec::VideoReader::retrieve", vec![(pred!(const, ["frame", "idx"], ["cv::Mat*", "const size_t"]), _)]),
	void cv_cudacodec_VideoReader_retrieve_const_MatR_const_size_t(const cv::cudacodec::VideoReader* instance, cv::Mat* frame, const size_t idx, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->retrieve(*frame, idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// retrieve(cuda::GpuMat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:465
	// ("cv::cudacodec::VideoReader::retrieve", vec![(pred!(const, ["frame"], ["cv::cuda::GpuMat*"]), _)]),
	void cv_cudacodec_VideoReader_retrieve_const_GpuMatR(const cv::cudacodec::VideoReader* instance, cv::cuda::GpuMat* frame, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->retrieve(*frame);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// set(const VideoReaderProps, const double)(Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:476
	// ("cv::cudacodec::VideoReader::set", vec![(pred!(mut, ["propertyId", "propertyVal"], ["const cv::cudacodec::VideoReaderProps", "const double"]), _)]),
	void cv_cudacodec_VideoReader_set_const_VideoReaderProps_const_double(cv::cudacodec::VideoReader* instance, const cv::cudacodec::VideoReaderProps propertyId, const double propertyVal, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->set(propertyId, propertyVal);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setVideoReaderProps(const VideoReaderProps, double)(Enum, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:477
	// ("cv::cudacodec::VideoReader::setVideoReaderProps", vec![(pred!(mut, ["propertyId", "propertyVal"], ["const cv::cudacodec::VideoReaderProps", "double"]), _)]),
	void cv_cudacodec_VideoReader_setVideoReaderProps_const_VideoReaderProps_double(cv::cudacodec::VideoReader* instance, const cv::cudacodec::VideoReaderProps propertyId, double propertyVal, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->setVideoReaderProps(propertyId, propertyVal);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// set(const ColorFormat)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:486
	// ("cv::cudacodec::VideoReader::set", vec![(pred!(mut, ["colorFormat"], ["const cv::cudacodec::ColorFormat"]), _)]),
	void cv_cudacodec_VideoReader_set_const_ColorFormat(cv::cudacodec::VideoReader* instance, const cv::cudacodec::ColorFormat colorFormat, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->set(colorFormat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// get(const VideoReaderProps, double &)(Enum, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:497
	// ("cv::cudacodec::VideoReader::get", vec![(pred!(const, ["propertyId", "propertyVal"], ["const cv::cudacodec::VideoReaderProps", "double*"]), _)]),
	void cv_cudacodec_VideoReader_get_const_const_VideoReaderProps_doubleR(const cv::cudacodec::VideoReader* instance, const cv::cudacodec::VideoReaderProps propertyId, double* propertyVal, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->get(propertyId, *propertyVal);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVideoReaderProps(const VideoReaderProps, double &, double)(Enum, Indirect, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:498
	// ("cv::cudacodec::VideoReader::getVideoReaderProps", vec![(pred!(const, ["propertyId", "propertyValOut", "propertyValIn"], ["const cv::cudacodec::VideoReaderProps", "double*", "double"]), _)]),
	void cv_cudacodec_VideoReader_getVideoReaderProps_const_const_VideoReaderProps_doubleR_double(const cv::cudacodec::VideoReader* instance, const cv::cudacodec::VideoReaderProps propertyId, double* propertyValOut, double propertyValIn, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getVideoReaderProps(propertyId, *propertyValOut, propertyValIn);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cudacodec::VideoReader::getVideoReaderProps(Enum, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:498
	// ("cv::cudacodec::VideoReader::getVideoReaderProps", vec![(pred!(const, ["propertyId", "propertyValOut"], ["const cv::cudacodec::VideoReaderProps", "double*"]), _)]),
	void cv_cudacodec_VideoReader_getVideoReaderProps_const_const_VideoReaderProps_doubleR(const cv::cudacodec::VideoReader* instance, const cv::cudacodec::VideoReaderProps propertyId, double* propertyValOut, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getVideoReaderProps(propertyId, *propertyValOut);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// get(const int, double &)(Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:508
	// ("cv::cudacodec::VideoReader::get", vec![(pred!(const, ["propertyId", "propertyVal"], ["const int", "double*"]), _)]),
	void cv_cudacodec_VideoReader_get_const_const_int_doubleR(const cv::cudacodec::VideoReader* instance, const int propertyId, double* propertyVal, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->get(propertyId, *propertyVal);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cudacodec::VideoReader::delete() generated
	// ("cv::cudacodec::VideoReader::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cudacodec_VideoReader_delete(cv::cudacodec::VideoReader* instance) {
			delete instance;
	}

	// VideoReaderInitParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:582
	// ("cv::cudacodec::VideoReaderInitParams::VideoReaderInitParams", vec![(pred!(mut, [], []), _)]),
	void cv_cudacodec_VideoReaderInitParams_VideoReaderInitParams(Result<cv::cudacodec::VideoReaderInitParams>* ocvrs_return) {
		try {
			cv::cudacodec::VideoReaderInitParams ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:250
	// ("cv::cudacodec::VideoWriter::write", vec![(pred!(mut, ["frame"], ["const cv::_InputArray*"]), _)]),
	void cv_cudacodec_VideoWriter_write_const__InputArrayR(cv::cudacodec::VideoWriter* instance, const cv::_InputArray* frame, ResultVoid* ocvrs_return) {
		try {
			instance->write(*frame);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEncoderParams()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:254
	// ("cv::cudacodec::VideoWriter::getEncoderParams", vec![(pred!(const, [], []), _)]),
	void cv_cudacodec_VideoWriter_getEncoderParams_const(const cv::cudacodec::VideoWriter* instance, Result<cv::cudacodec::EncoderParams>* ocvrs_return) {
		try {
			cv::cudacodec::EncoderParams ret = instance->getEncoderParams();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// release()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/cudacodec.hpp:258
	// ("cv::cudacodec::VideoWriter::release", vec![(pred!(mut, [], []), _)]),
	void cv_cudacodec_VideoWriter_release(cv::cudacodec::VideoWriter* instance, ResultVoid* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::cudacodec::VideoWriter::delete() generated
	// ("cv::cudacodec::VideoWriter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_cudacodec_VideoWriter_delete(cv::cudacodec::VideoWriter* instance) {
			delete instance;
	}

}
