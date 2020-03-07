#include "videoio.hpp"
#include "videoio_types.hpp"

extern "C" {
	Result<void*> cv_videoio_registry_getBackendName_VideoCaptureAPIs(cv::VideoCaptureAPIs api) {
		try {
			cv::String ret = cv::videoio_registry::getBackendName(api);
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_videoio_registry_getBackends() {
		try {
			std::vector<cv::VideoCaptureAPIs> ret = cv::videoio_registry::getBackends();
			return Ok<void*>(new std::vector<cv::VideoCaptureAPIs>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_videoio_registry_getCameraBackends() {
		try {
			std::vector<cv::VideoCaptureAPIs> ret = cv::videoio_registry::getCameraBackends();
			return Ok<void*>(new std::vector<cv::VideoCaptureAPIs>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_videoio_registry_getStreamBackends() {
		try {
			std::vector<cv::VideoCaptureAPIs> ret = cv::videoio_registry::getStreamBackends();
			return Ok<void*>(new std::vector<cv::VideoCaptureAPIs>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_videoio_registry_getWriterBackends() {
		try {
			std::vector<cv::VideoCaptureAPIs> ret = cv::videoio_registry::getWriterBackends();
			return Ok<void*>(new std::vector<cv::VideoCaptureAPIs>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_videoio_registry_hasBackend_VideoCaptureAPIs(cv::VideoCaptureAPIs api) {
		try {
			bool ret = cv::videoio_registry::hasBackend(api);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	void cv_VideoCapture_delete(cv::VideoCapture* instance) {
		delete instance;
	}
	Result<void*> cv_VideoCapture_VideoCapture() {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_VideoCapture_VideoCapture_const_StringX_int(const char* filename, int apiPreference) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture(std::string(filename), apiPreference);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_VideoCapture_VideoCapture_int_int(int index, int apiPreference) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture(index, apiPreference);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_VideoCapture_open_const_StringX_int(void* instance, const char* filename, int apiPreference) {
		try {
			bool ret = reinterpret_cast<cv::VideoCapture*>(instance)->open(std::string(filename), apiPreference);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_VideoCapture_open_int_int(void* instance, int index, int apiPreference) {
		try {
			bool ret = reinterpret_cast<cv::VideoCapture*>(instance)->open(index, apiPreference);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_VideoCapture_isOpened_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::VideoCapture*>(instance)->isOpened();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_VideoCapture_release(void* instance) {
		try {
			reinterpret_cast<cv::VideoCapture*>(instance)->release();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_VideoCapture_grab(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::VideoCapture*>(instance)->grab();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_VideoCapture_retrieve_const__OutputArrayX_int(void* instance, void* image, int flag) {
		try {
			bool ret = reinterpret_cast<cv::VideoCapture*>(instance)->retrieve(*reinterpret_cast<const cv::_OutputArray*>(image), flag);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_VideoCapture_read_const__OutputArrayX(void* instance, void* image) {
		try {
			bool ret = reinterpret_cast<cv::VideoCapture*>(instance)->read(*reinterpret_cast<const cv::_OutputArray*>(image));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_VideoCapture_set_int_double(void* instance, int propId, double value) {
		try {
			bool ret = reinterpret_cast<cv::VideoCapture*>(instance)->set(propId, value);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<double> cv_VideoCapture_get_const_int(void* instance, int propId) {
		try {
			double ret = reinterpret_cast<cv::VideoCapture*>(instance)->get(propId);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<void*> cv_VideoCapture_getBackendName_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::VideoCapture*>(instance)->getBackendName();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_VideoCapture_setExceptionMode_bool(void* instance, bool enable) {
		try {
			reinterpret_cast<cv::VideoCapture*>(instance)->setExceptionMode(enable);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_VideoCapture_getExceptionMode(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::VideoCapture*>(instance)->getExceptionMode();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_VideoCapture_waitAny_const_vector_VideoCapture_X_vector_int_X_int64_t(void* streams, void* readyIndex, int64_t timeoutNs) {
		try {
			bool ret = cv::VideoCapture::waitAny(*reinterpret_cast<const std::vector<cv::VideoCapture>*>(streams), *reinterpret_cast<std::vector<int>*>(readyIndex), timeoutNs);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	void cv_VideoWriter_delete(cv::VideoWriter* instance) {
		delete instance;
	}
	Result<void*> cv_VideoWriter_VideoWriter() {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_VideoWriter_VideoWriter_const_StringX_int_double_Size_bool(const char* filename, int fourcc, double fps, const cv::Size* frameSize, bool isColor) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter(std::string(filename), fourcc, fps, *frameSize, isColor);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_VideoWriter_VideoWriter_const_StringX_int_int_double_Size_bool(const char* filename, int apiPreference, int fourcc, double fps, const cv::Size* frameSize, bool isColor) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter(std::string(filename), apiPreference, fourcc, fps, *frameSize, isColor);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_VideoWriter_open_const_StringX_int_double_Size_bool(void* instance, const char* filename, int fourcc, double fps, const cv::Size* frameSize, bool isColor) {
		try {
			bool ret = reinterpret_cast<cv::VideoWriter*>(instance)->open(std::string(filename), fourcc, fps, *frameSize, isColor);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_VideoWriter_open_const_StringX_int_int_double_Size_bool(void* instance, const char* filename, int apiPreference, int fourcc, double fps, const cv::Size* frameSize, bool isColor) {
		try {
			bool ret = reinterpret_cast<cv::VideoWriter*>(instance)->open(std::string(filename), apiPreference, fourcc, fps, *frameSize, isColor);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_VideoWriter_isOpened_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::VideoWriter*>(instance)->isOpened();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_VideoWriter_release(void* instance) {
		try {
			reinterpret_cast<cv::VideoWriter*>(instance)->release();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_VideoWriter_write_const__InputArrayX(void* instance, void* image) {
		try {
			reinterpret_cast<cv::VideoWriter*>(instance)->write(*reinterpret_cast<const cv::_InputArray*>(image));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_VideoWriter_set_int_double(void* instance, int propId, double value) {
		try {
			bool ret = reinterpret_cast<cv::VideoWriter*>(instance)->set(propId, value);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<double> cv_VideoWriter_get_const_int(void* instance, int propId) {
		try {
			double ret = reinterpret_cast<cv::VideoWriter*>(instance)->get(propId);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<int> cv_VideoWriter_fourcc_char_char_char_char(char c1, char c2, char c3, char c4) {
		try {
			int ret = cv::VideoWriter::fourcc(c1, c2, c3, c4);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_VideoWriter_getBackendName_const(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::VideoWriter*>(instance)->getBackendName();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
}
