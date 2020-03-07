#include "videoio.hpp"
#include "videoio_types.hpp"

extern "C" {
	void cv_VideoCapture_delete(cv::VideoCapture* instance) {
		delete instance;
	}
	Result<void*> cv_VideoCapture_VideoCapture() {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_VideoCapture_VideoCapture_const_StringX(const char* filename) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture(cv::String(filename));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_VideoCapture_VideoCapture_const_StringX_int(const char* filename, int apiPreference) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture(cv::String(filename), apiPreference);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_VideoCapture_VideoCapture_int(int index) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture(index);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_VideoCapture_open_const_StringX(void* instance, const char* filename) {
		try {
			bool ret = reinterpret_cast<cv::VideoCapture*>(instance)->open(cv::String(filename));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_VideoCapture_open_int(void* instance, int index) {
		try {
			bool ret = reinterpret_cast<cv::VideoCapture*>(instance)->open(index);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_VideoCapture_open_int_int(void* instance, int cameraNum, int apiPreference) {
		try {
			bool ret = reinterpret_cast<cv::VideoCapture*>(instance)->open(cameraNum, apiPreference);
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
	
	Result<bool> cv_VideoCapture_open_const_StringX_int(void* instance, const char* filename, int apiPreference) {
		try {
			bool ret = reinterpret_cast<cv::VideoCapture*>(instance)->open(cv::String(filename), apiPreference);
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
			cv::VideoWriter* ret = new cv::VideoWriter(cv::String(filename), fourcc, fps, *frameSize, isColor);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_VideoWriter_open_const_StringX_int_double_Size_bool(void* instance, const char* filename, int fourcc, double fps, const cv::Size* frameSize, bool isColor) {
		try {
			bool ret = reinterpret_cast<cv::VideoWriter*>(instance)->open(cv::String(filename), fourcc, fps, *frameSize, isColor);
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
	
	Result_void cv_VideoWriter_write_const_MatX(void* instance, void* image) {
		try {
			reinterpret_cast<cv::VideoWriter*>(instance)->write(*reinterpret_cast<const cv::Mat*>(image));
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
	
}
