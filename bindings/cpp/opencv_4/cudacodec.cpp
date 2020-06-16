#include "common.hpp"
#include <opencv2/cudacodec.hpp>
#include "cudacodec_types.hpp"

extern "C" {
	Result<cv::Ptr<cv::cudacodec::VideoReader>*> cv_cudacodec_createVideoReader_const_Ptr_RawVideoSource_R(const cv::Ptr<cv::cudacodec::RawVideoSource>* source) {
		try {
			cv::Ptr<cv::cudacodec::VideoReader> ret = cv::cudacodec::createVideoReader(*source);
			return Ok(new cv::Ptr<cv::cudacodec::VideoReader>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cudacodec::VideoReader>*>))
	}
	
	Result<cv::Ptr<cv::cudacodec::VideoReader>*> cv_cudacodec_createVideoReader_const_StringR(const char* filename) {
		try {
			cv::Ptr<cv::cudacodec::VideoReader> ret = cv::cudacodec::createVideoReader(std::string(filename));
			return Ok(new cv::Ptr<cv::cudacodec::VideoReader>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cudacodec::VideoReader>*>))
	}
	
	Result<cv::Ptr<cv::cudacodec::VideoWriter>*> cv_cudacodec_createVideoWriter_const_Ptr_EncoderCallBack_R_Size_double_SurfaceFormat(const cv::Ptr<cv::cudacodec::EncoderCallBack>* encoderCallback, const cv::Size* frameSize, double fps, cv::cudacodec::SurfaceFormat format) {
		try {
			cv::Ptr<cv::cudacodec::VideoWriter> ret = cv::cudacodec::createVideoWriter(*encoderCallback, *frameSize, fps, format);
			return Ok(new cv::Ptr<cv::cudacodec::VideoWriter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cudacodec::VideoWriter>*>))
	}
	
	Result<cv::Ptr<cv::cudacodec::VideoWriter>*> cv_cudacodec_createVideoWriter_const_Ptr_EncoderCallBack_R_Size_double_const_EncoderParamsR_SurfaceFormat(const cv::Ptr<cv::cudacodec::EncoderCallBack>* encoderCallback, const cv::Size* frameSize, double fps, const cv::cudacodec::EncoderParams* params, cv::cudacodec::SurfaceFormat format) {
		try {
			cv::Ptr<cv::cudacodec::VideoWriter> ret = cv::cudacodec::createVideoWriter(*encoderCallback, *frameSize, fps, *params, format);
			return Ok(new cv::Ptr<cv::cudacodec::VideoWriter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cudacodec::VideoWriter>*>))
	}
	
	Result<cv::Ptr<cv::cudacodec::VideoWriter>*> cv_cudacodec_createVideoWriter_const_StringR_Size_double_SurfaceFormat(const char* fileName, const cv::Size* frameSize, double fps, cv::cudacodec::SurfaceFormat format) {
		try {
			cv::Ptr<cv::cudacodec::VideoWriter> ret = cv::cudacodec::createVideoWriter(std::string(fileName), *frameSize, fps, format);
			return Ok(new cv::Ptr<cv::cudacodec::VideoWriter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cudacodec::VideoWriter>*>))
	}
	
	Result<cv::Ptr<cv::cudacodec::VideoWriter>*> cv_cudacodec_createVideoWriter_const_StringR_Size_double_const_EncoderParamsR_SurfaceFormat(const char* fileName, const cv::Size* frameSize, double fps, const cv::cudacodec::EncoderParams* params, cv::cudacodec::SurfaceFormat format) {
		try {
			cv::Ptr<cv::cudacodec::VideoWriter> ret = cv::cudacodec::createVideoWriter(std::string(fileName), *frameSize, fps, *params, format);
			return Ok(new cv::Ptr<cv::cudacodec::VideoWriter>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cudacodec::VideoWriter>*>))
	}
	
	Result<unsigned char*> cv_cudacodec_EncoderCallBack_acquireBitStream_intX(cv::cudacodec::EncoderCallBack* instance, int* bufferSize) {
		try {
			unsigned char* ret = instance->acquireBitStream(bufferSize);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	Result_void cv_cudacodec_EncoderCallBack_releaseBitStream_unsigned_charX_int(cv::cudacodec::EncoderCallBack* instance, unsigned char* data, int size) {
		try {
			instance->releaseBitStream(data, size);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cudacodec_EncoderCallBack_onBeginFrame_int_PicType(cv::cudacodec::EncoderCallBack* instance, int frameNumber, cv::cudacodec::EncoderCallBack::PicType picType) {
		try {
			instance->onBeginFrame(frameNumber, picType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cudacodec_EncoderCallBack_onEndFrame_int_PicType(cv::cudacodec::EncoderCallBack* instance, int frameNumber, cv::cudacodec::EncoderCallBack::PicType picType) {
		try {
			instance->onEndFrame(frameNumber, picType);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cudacodec_EncoderParams_getPropP_Interval_const(const cv::cudacodec::EncoderParams* instance) {
		try {
			int ret = instance->P_Interval;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cudacodec_EncoderParams_setPropP_Interval_int(cv::cudacodec::EncoderParams* instance, int val) {
		try {
			instance->P_Interval = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cudacodec_EncoderParams_getPropIDR_Period_const(const cv::cudacodec::EncoderParams* instance) {
		try {
			int ret = instance->IDR_Period;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cudacodec_EncoderParams_setPropIDR_Period_int(cv::cudacodec::EncoderParams* instance, int val) {
		try {
			instance->IDR_Period = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cudacodec_EncoderParams_getPropDynamicGOP_const(const cv::cudacodec::EncoderParams* instance) {
		try {
			int ret = instance->DynamicGOP;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cudacodec_EncoderParams_setPropDynamicGOP_int(cv::cudacodec::EncoderParams* instance, int val) {
		try {
			instance->DynamicGOP = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cudacodec_EncoderParams_getPropRCType_const(const cv::cudacodec::EncoderParams* instance) {
		try {
			int ret = instance->RCType;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cudacodec_EncoderParams_setPropRCType_int(cv::cudacodec::EncoderParams* instance, int val) {
		try {
			instance->RCType = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cudacodec_EncoderParams_getPropAvgBitrate_const(const cv::cudacodec::EncoderParams* instance) {
		try {
			int ret = instance->AvgBitrate;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cudacodec_EncoderParams_setPropAvgBitrate_int(cv::cudacodec::EncoderParams* instance, int val) {
		try {
			instance->AvgBitrate = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cudacodec_EncoderParams_getPropPeakBitrate_const(const cv::cudacodec::EncoderParams* instance) {
		try {
			int ret = instance->PeakBitrate;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cudacodec_EncoderParams_setPropPeakBitrate_int(cv::cudacodec::EncoderParams* instance, int val) {
		try {
			instance->PeakBitrate = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cudacodec_EncoderParams_getPropQP_Level_Intra_const(const cv::cudacodec::EncoderParams* instance) {
		try {
			int ret = instance->QP_Level_Intra;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cudacodec_EncoderParams_setPropQP_Level_Intra_int(cv::cudacodec::EncoderParams* instance, int val) {
		try {
			instance->QP_Level_Intra = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cudacodec_EncoderParams_getPropQP_Level_InterP_const(const cv::cudacodec::EncoderParams* instance) {
		try {
			int ret = instance->QP_Level_InterP;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cudacodec_EncoderParams_setPropQP_Level_InterP_int(cv::cudacodec::EncoderParams* instance, int val) {
		try {
			instance->QP_Level_InterP = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cudacodec_EncoderParams_getPropQP_Level_InterB_const(const cv::cudacodec::EncoderParams* instance) {
		try {
			int ret = instance->QP_Level_InterB;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cudacodec_EncoderParams_setPropQP_Level_InterB_int(cv::cudacodec::EncoderParams* instance, int val) {
		try {
			instance->QP_Level_InterB = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cudacodec_EncoderParams_getPropDeblockMode_const(const cv::cudacodec::EncoderParams* instance) {
		try {
			int ret = instance->DeblockMode;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cudacodec_EncoderParams_setPropDeblockMode_int(cv::cudacodec::EncoderParams* instance, int val) {
		try {
			instance->DeblockMode = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cudacodec_EncoderParams_getPropProfileLevel_const(const cv::cudacodec::EncoderParams* instance) {
		try {
			int ret = instance->ProfileLevel;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cudacodec_EncoderParams_setPropProfileLevel_int(cv::cudacodec::EncoderParams* instance, int val) {
		try {
			instance->ProfileLevel = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cudacodec_EncoderParams_getPropForceIntra_const(const cv::cudacodec::EncoderParams* instance) {
		try {
			int ret = instance->ForceIntra;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cudacodec_EncoderParams_setPropForceIntra_int(cv::cudacodec::EncoderParams* instance, int val) {
		try {
			instance->ForceIntra = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cudacodec_EncoderParams_getPropForceIDR_const(const cv::cudacodec::EncoderParams* instance) {
		try {
			int ret = instance->ForceIDR;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cudacodec_EncoderParams_setPropForceIDR_int(cv::cudacodec::EncoderParams* instance, int val) {
		try {
			instance->ForceIDR = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cudacodec_EncoderParams_getPropClearStat_const(const cv::cudacodec::EncoderParams* instance) {
		try {
			int ret = instance->ClearStat;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cudacodec_EncoderParams_setPropClearStat_int(cv::cudacodec::EncoderParams* instance, int val) {
		try {
			instance->ClearStat = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cudacodec_EncoderParams_getPropDIMode_const(const cv::cudacodec::EncoderParams* instance) {
		try {
			int ret = instance->DIMode;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cudacodec_EncoderParams_setPropDIMode_int(cv::cudacodec::EncoderParams* instance, int val) {
		try {
			instance->DIMode = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cudacodec_EncoderParams_getPropPresets_const(const cv::cudacodec::EncoderParams* instance) {
		try {
			int ret = instance->Presets;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cudacodec_EncoderParams_setPropPresets_int(cv::cudacodec::EncoderParams* instance, int val) {
		try {
			instance->Presets = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cudacodec_EncoderParams_getPropDisableCabac_const(const cv::cudacodec::EncoderParams* instance) {
		try {
			int ret = instance->DisableCabac;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cudacodec_EncoderParams_setPropDisableCabac_int(cv::cudacodec::EncoderParams* instance, int val) {
		try {
			instance->DisableCabac = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cudacodec_EncoderParams_getPropNaluFramingType_const(const cv::cudacodec::EncoderParams* instance) {
		try {
			int ret = instance->NaluFramingType;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cudacodec_EncoderParams_setPropNaluFramingType_int(cv::cudacodec::EncoderParams* instance, int val) {
		try {
			instance->NaluFramingType = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_cudacodec_EncoderParams_getPropDisableSPSPPS_const(const cv::cudacodec::EncoderParams* instance) {
		try {
			int ret = instance->DisableSPSPPS;
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_cudacodec_EncoderParams_setPropDisableSPSPPS_int(cv::cudacodec::EncoderParams* instance, int val) {
		try {
			instance->DisableSPSPPS = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_EncoderParams_delete(cv::cudacodec::EncoderParams* instance) {
		delete instance;
	}
	Result<cv::cudacodec::EncoderParams*> cv_cudacodec_EncoderParams_EncoderParams() {
		try {
			cv::cudacodec::EncoderParams* ret = new cv::cudacodec::EncoderParams();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cudacodec::EncoderParams*>))
	}
	
	Result<cv::cudacodec::EncoderParams*> cv_cudacodec_EncoderParams_EncoderParams_const_StringR(const char* configFile) {
		try {
			cv::cudacodec::EncoderParams* ret = new cv::cudacodec::EncoderParams(std::string(configFile));
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cudacodec::EncoderParams*>))
	}
	
	Result_void cv_cudacodec_EncoderParams_load_const_StringR(cv::cudacodec::EncoderParams* instance, const char* configFile) {
		try {
			instance->load(std::string(configFile));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_cudacodec_EncoderParams_save_const_const_StringR(const cv::cudacodec::EncoderParams* instance, const char* configFile) {
		try {
			instance->save(std::string(configFile));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_cudacodec_RawVideoSource_getNextPacket_unsigned_charXX_size_tX(cv::cudacodec::RawVideoSource* instance, unsigned char** data, size_t* size) {
		try {
			bool ret = instance->getNextPacket(data, size);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::cudacodec::FormatInfo> cv_cudacodec_RawVideoSource_format_const(const cv::cudacodec::RawVideoSource* instance) {
		try {
			cv::cudacodec::FormatInfo ret = instance->format();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cudacodec::FormatInfo>))
	}
	
	Result<bool> cv_cudacodec_VideoReader_nextFrame_GpuMatR_StreamR(cv::cudacodec::VideoReader* instance, cv::cuda::GpuMat* frame, cv::cuda::Stream* stream) {
		try {
			bool ret = instance->nextFrame(*frame, *stream);
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<cv::cudacodec::FormatInfo> cv_cudacodec_VideoReader_format_const(const cv::cudacodec::VideoReader* instance) {
		try {
			cv::cudacodec::FormatInfo ret = instance->format();
			return Ok(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cudacodec::FormatInfo>))
	}
	
	Result_void cv_cudacodec_VideoWriter_write_const__InputArrayR_bool(cv::cudacodec::VideoWriter* instance, const cv::_InputArray* frame, bool lastFrame) {
		try {
			instance->write(*frame, lastFrame);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::cudacodec::EncoderParams*> cv_cudacodec_VideoWriter_getEncoderParams_const(const cv::cudacodec::VideoWriter* instance) {
		try {
			cv::cudacodec::EncoderParams ret = instance->getEncoderParams();
			return Ok(new cv::cudacodec::EncoderParams(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cudacodec::EncoderParams*>))
	}
	
}
