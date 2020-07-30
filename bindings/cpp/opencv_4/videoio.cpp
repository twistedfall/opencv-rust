#include "videoio.hpp"
#include "videoio_types.hpp"

extern "C" {
	Result<void*> cv_videoio_registry_getBackendName_VideoCaptureAPIs(cv::VideoCaptureAPIs api) {
		try {
			cv::String ret = cv::videoio_registry::getBackendName(api);
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<std::vector<cv::VideoCaptureAPIs>*> cv_videoio_registry_getBackends() {
		try {
			std::vector<cv::VideoCaptureAPIs> ret = cv::videoio_registry::getBackends();
			return Ok(new std::vector<cv::VideoCaptureAPIs>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::VideoCaptureAPIs>*>))
	}
	
	Result<std::vector<cv::VideoCaptureAPIs>*> cv_videoio_registry_getCameraBackends() {
		try {
			std::vector<cv::VideoCaptureAPIs> ret = cv::videoio_registry::getCameraBackends();
			return Ok(new std::vector<cv::VideoCaptureAPIs>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::VideoCaptureAPIs>*>))
	}
	
	Result<std::vector<cv::VideoCaptureAPIs>*> cv_videoio_registry_getStreamBackends() {
		try {
			std::vector<cv::VideoCaptureAPIs> ret = cv::videoio_registry::getStreamBackends();
			return Ok(new std::vector<cv::VideoCaptureAPIs>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::VideoCaptureAPIs>*>))
	}
	
	Result<std::vector<cv::VideoCaptureAPIs>*> cv_videoio_registry_getWriterBackends() {
		try {
			std::vector<cv::VideoCaptureAPIs> ret = cv::videoio_registry::getWriterBackends();
			return Ok(new std::vector<cv::VideoCaptureAPIs>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::VideoCaptureAPIs>*>))
	}
	
	Result<bool> cv_videoio_registry_hasBackend_VideoCaptureAPIs(cv::VideoCaptureAPIs api) {
		try {
			bool ret = cv::videoio_registry::hasBackend(api);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_VideoCapture_delete(cv::VideoCapture* instance) {
		delete instance;
	}
	Result<cv::VideoCapture*> cv_VideoCapture_VideoCapture() {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture();
			return Ok<cv::VideoCapture*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::VideoCapture*>))
	}
	
	Result<cv::VideoCapture*> cv_VideoCapture_VideoCapture_const_StringR_int(const char* filename, int apiPreference) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture(std::string(filename), apiPreference);
			return Ok<cv::VideoCapture*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::VideoCapture*>))
	}
	
	Result<cv::VideoCapture*> cv_VideoCapture_VideoCapture_int_int(int index, int apiPreference) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture(index, apiPreference);
			return Ok<cv::VideoCapture*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::VideoCapture*>))
	}
	
	Result<bool> cv_VideoCapture_open_const_StringR_int(cv::VideoCapture* instance, const char* filename, int apiPreference) {
		try {
			bool ret = instance->open(std::string(filename), apiPreference);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_VideoCapture_open_int_int(cv::VideoCapture* instance, int index, int apiPreference) {
		try {
			bool ret = instance->open(index, apiPreference);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_VideoCapture_isOpened_const(const cv::VideoCapture* instance) {
		try {
			bool ret = instance->isOpened();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_VideoCapture_release(cv::VideoCapture* instance) {
		try {
			instance->release();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_VideoCapture_grab(cv::VideoCapture* instance) {
		try {
			bool ret = instance->grab();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_VideoCapture_retrieve_const__OutputArrayR_int(cv::VideoCapture* instance, const cv::_OutputArray* image, int flag) {
		try {
			bool ret = instance->retrieve(*image, flag);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_VideoCapture_read_const__OutputArrayR(cv::VideoCapture* instance, const cv::_OutputArray* image) {
		try {
			bool ret = instance->read(*image);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_VideoCapture_set_int_double(cv::VideoCapture* instance, int propId, double value) {
		try {
			bool ret = instance->set(propId, value);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<double> cv_VideoCapture_get_const_int(const cv::VideoCapture* instance, int propId) {
		try {
			double ret = instance->get(propId);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<void*> cv_VideoCapture_getBackendName_const(const cv::VideoCapture* instance) {
		try {
			cv::String ret = instance->getBackendName();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_VideoCapture_setExceptionMode_bool(cv::VideoCapture* instance, bool enable) {
		try {
			instance->setExceptionMode(enable);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_VideoCapture_getExceptionMode(cv::VideoCapture* instance) {
		try {
			bool ret = instance->getExceptionMode();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_VideoCapture_waitAny_const_vector_VideoCapture_R_vector_int_R_int64_t(const std::vector<cv::VideoCapture>* streams, std::vector<int>* readyIndex, int64_t timeoutNs) {
		try {
			bool ret = cv::VideoCapture::waitAny(*streams, *readyIndex, timeoutNs);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_VideoWriter_delete(cv::VideoWriter* instance) {
		delete instance;
	}
	Result<cv::VideoWriter*> cv_VideoWriter_VideoWriter() {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter();
			return Ok<cv::VideoWriter*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::VideoWriter*>))
	}
	
	Result<cv::VideoWriter*> cv_VideoWriter_VideoWriter_const_StringR_int_double_Size_bool(const char* filename, int fourcc, double fps, cv::Size* frameSize, bool isColor) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter(std::string(filename), fourcc, fps, *frameSize, isColor);
			return Ok<cv::VideoWriter*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::VideoWriter*>))
	}
	
	Result<cv::VideoWriter*> cv_VideoWriter_VideoWriter_const_StringR_int_int_double_Size_bool(const char* filename, int apiPreference, int fourcc, double fps, cv::Size* frameSize, bool isColor) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter(std::string(filename), apiPreference, fourcc, fps, *frameSize, isColor);
			return Ok<cv::VideoWriter*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::VideoWriter*>))
	}
	
	Result<cv::VideoWriter*> cv_VideoWriter_VideoWriter_const_StringR_int_double_const_SizeR_const_vector_int_R(const char* filename, int fourcc, double fps, const cv::Size* frameSize, const std::vector<int>* params) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter(std::string(filename), fourcc, fps, *frameSize, *params);
			return Ok<cv::VideoWriter*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::VideoWriter*>))
	}
	
	Result<cv::VideoWriter*> cv_VideoWriter_VideoWriter_const_StringR_int_int_double_const_SizeR_const_vector_int_R(const char* filename, int apiPreference, int fourcc, double fps, const cv::Size* frameSize, const std::vector<int>* params) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter(std::string(filename), apiPreference, fourcc, fps, *frameSize, *params);
			return Ok<cv::VideoWriter*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::VideoWriter*>))
	}
	
	Result<bool> cv_VideoWriter_open_const_StringR_int_double_Size_bool(cv::VideoWriter* instance, const char* filename, int fourcc, double fps, cv::Size* frameSize, bool isColor) {
		try {
			bool ret = instance->open(std::string(filename), fourcc, fps, *frameSize, isColor);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_VideoWriter_open_const_StringR_int_int_double_Size_bool(cv::VideoWriter* instance, const char* filename, int apiPreference, int fourcc, double fps, cv::Size* frameSize, bool isColor) {
		try {
			bool ret = instance->open(std::string(filename), apiPreference, fourcc, fps, *frameSize, isColor);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_VideoWriter_open_const_StringR_int_double_const_SizeR_const_vector_int_R(cv::VideoWriter* instance, const char* filename, int fourcc, double fps, const cv::Size* frameSize, const std::vector<int>* params) {
		try {
			bool ret = instance->open(std::string(filename), fourcc, fps, *frameSize, *params);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_VideoWriter_open_const_StringR_int_int_double_const_SizeR_const_vector_int_R(cv::VideoWriter* instance, const char* filename, int apiPreference, int fourcc, double fps, const cv::Size* frameSize, const std::vector<int>* params) {
		try {
			bool ret = instance->open(std::string(filename), apiPreference, fourcc, fps, *frameSize, *params);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<bool> cv_VideoWriter_isOpened_const(const cv::VideoWriter* instance) {
		try {
			bool ret = instance->isOpened();
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result_void cv_VideoWriter_release(cv::VideoWriter* instance) {
		try {
			instance->release();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_VideoWriter_write_const__InputArrayR(cv::VideoWriter* instance, const cv::_InputArray* image) {
		try {
			instance->write(*image);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<bool> cv_VideoWriter_set_int_double(cv::VideoWriter* instance, int propId, double value) {
		try {
			bool ret = instance->set(propId, value);
			return Ok<bool>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	Result<double> cv_VideoWriter_get_const_int(const cv::VideoWriter* instance, int propId) {
		try {
			double ret = instance->get(propId);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result<int> cv_VideoWriter_fourcc_char_char_char_char(char c1, char c2, char c3, char c4) {
		try {
			int ret = cv::VideoWriter::fourcc(c1, c2, c3, c4);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<void*> cv_VideoWriter_getBackendName_const(const cv::VideoWriter* instance) {
		try {
			cv::String ret = instance->getBackendName();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
}
