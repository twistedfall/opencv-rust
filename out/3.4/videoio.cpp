#include "videoio.hpp"
#include "videoio_types.hpp"

extern "C" {
	// getBackendName(VideoCaptureAPIs)(Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio/registry.hpp:27
	// ("cv::videoio_registry::getBackendName", vec![(pred!(mut, ["api"], ["cv::VideoCaptureAPIs"]), _)]),
	void cv_videoio_registry_getBackendName_VideoCaptureAPIs(cv::VideoCaptureAPIs api, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::videoio_registry::getBackendName(api);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackends()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio/registry.hpp:30
	// ("cv::videoio_registry::getBackends", vec![(pred!(mut, [], []), _)]),
	void cv_videoio_registry_getBackends(Result<std::vector<cv::VideoCaptureAPIs>*>* ocvrs_return) {
		try {
			std::vector<cv::VideoCaptureAPIs> ret = cv::videoio_registry::getBackends();
			Ok(new std::vector<cv::VideoCaptureAPIs>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCameraBackends()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio/registry.hpp:33
	// ("cv::videoio_registry::getCameraBackends", vec![(pred!(mut, [], []), _)]),
	void cv_videoio_registry_getCameraBackends(Result<std::vector<cv::VideoCaptureAPIs>*>* ocvrs_return) {
		try {
			std::vector<cv::VideoCaptureAPIs> ret = cv::videoio_registry::getCameraBackends();
			Ok(new std::vector<cv::VideoCaptureAPIs>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getStreamBackends()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio/registry.hpp:36
	// ("cv::videoio_registry::getStreamBackends", vec![(pred!(mut, [], []), _)]),
	void cv_videoio_registry_getStreamBackends(Result<std::vector<cv::VideoCaptureAPIs>*>* ocvrs_return) {
		try {
			std::vector<cv::VideoCaptureAPIs> ret = cv::videoio_registry::getStreamBackends();
			Ok(new std::vector<cv::VideoCaptureAPIs>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWriterBackends()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio/registry.hpp:39
	// ("cv::videoio_registry::getWriterBackends", vec![(pred!(mut, [], []), _)]),
	void cv_videoio_registry_getWriterBackends(Result<std::vector<cv::VideoCaptureAPIs>*>* ocvrs_return) {
		try {
			std::vector<cv::VideoCaptureAPIs> ret = cv::videoio_registry::getWriterBackends();
			Ok(new std::vector<cv::VideoCaptureAPIs>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// VideoCapture()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:630
	// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, [], []), _)]),
	void cv_VideoCapture_VideoCapture(Result<cv::VideoCapture*>* ocvrs_return) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// VideoCapture(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:637
	// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_VideoCapture_VideoCapture_const_StringR(const char* filename, Result<cv::VideoCapture*>* ocvrs_return) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture(cv::String(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// VideoCapture(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:654
	// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["filename", "apiPreference"], ["const cv::String*", "int"]), _)]),
	void cv_VideoCapture_VideoCapture_const_StringR_int(const char* filename, int apiPreference, Result<cv::VideoCapture*>* ocvrs_return) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture(cv::String(filename), apiPreference);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// VideoCapture(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:665
	// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["index"], ["int"]), _)]),
	void cv_VideoCapture_VideoCapture_int(int index, Result<cv::VideoCapture*>* ocvrs_return) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture(index);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// VideoCapture(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:677
	// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["index", "apiPreference"], ["int", "int"]), _)]),
	void cv_VideoCapture_VideoCapture_int_int(int index, int apiPreference, Result<cv::VideoCapture*>* ocvrs_return) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture(index, apiPreference);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// open(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:694
	// ("cv::VideoCapture::open", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_VideoCapture_open_const_StringR(cv::VideoCapture* instance, const char* filename, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(cv::String(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// open(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:705
	// ("cv::VideoCapture::open", vec![(pred!(mut, ["index"], ["int"]), _)]),
	void cv_VideoCapture_open_int(cv::VideoCapture* instance, int index, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(index);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// open(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:715
	// ("cv::VideoCapture::open", vec![(pred!(mut, ["cameraNum", "apiPreference"], ["int", "int"]), _)]),
	void cv_VideoCapture_open_int_int(cv::VideoCapture* instance, int cameraNum, int apiPreference, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(cameraNum, apiPreference);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isOpened()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:722
	// ("cv::VideoCapture::isOpened", vec![(pred!(const, [], []), _)]),
	void cv_VideoCapture_isOpened_const(const cv::VideoCapture* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isOpened();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:731
	// ("cv::VideoCapture::release", vec![(pred!(mut, [], []), _)]),
	void cv_VideoCapture_release(cv::VideoCapture* instance, ResultVoid* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// grab()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:752
	// ("cv::VideoCapture::grab", vec![(pred!(mut, [], []), _)]),
	void cv_VideoCapture_grab(cv::VideoCapture* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->grab();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// retrieve(OutputArray, int)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:770
	// ("cv::VideoCapture::retrieve", vec![(pred!(mut, ["image", "flag"], ["const cv::_OutputArray*", "int"]), _)]),
	void cv_VideoCapture_retrieve_const__OutputArrayR_int(cv::VideoCapture* instance, const cv::_OutputArray* image, int flag, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->retrieve(*image, flag);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::VideoCapture::retrieve(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:770
	// ("cv::VideoCapture::retrieve", vec![(pred!(mut, ["image"], ["const cv::_OutputArray*"]), _)]),
	void cv_VideoCapture_retrieve_const__OutputArrayR(cv::VideoCapture* instance, const cv::_OutputArray* image, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->retrieve(*image);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:796
	// ("cv::VideoCapture::read", vec![(pred!(mut, ["image"], ["const cv::_OutputArray*"]), _)]),
	void cv_VideoCapture_read_const__OutputArrayR(cv::VideoCapture* instance, const cv::_OutputArray* image, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->read(*image);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// set(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:807
	// ("cv::VideoCapture::set", vec![(pred!(mut, ["propId", "value"], ["int", "double"]), _)]),
	void cv_VideoCapture_set_int_double(cv::VideoCapture* instance, int propId, double value, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->set(propId, value);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// get(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:826
	// ("cv::VideoCapture::get", vec![(pred!(const, ["propId"], ["int"]), _)]),
	void cv_VideoCapture_get_const_int(const cv::VideoCapture* instance, int propId, Result<double>* ocvrs_return) {
		try {
			double ret = instance->get(propId);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// open(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:837
	// ("cv::VideoCapture::open", vec![(pred!(mut, ["filename", "apiPreference"], ["const cv::String*", "int"]), _)]),
	void cv_VideoCapture_open_const_StringR_int(cv::VideoCapture* instance, const char* filename, int apiPreference, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(cv::String(filename), apiPreference);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackendName()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:843
	// ("cv::VideoCapture::getBackendName", vec![(pred!(const, [], []), _)]),
	void cv_VideoCapture_getBackendName_const(const cv::VideoCapture* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getBackendName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::VideoCapture::delete() generated
	// ("cv::VideoCapture::delete", vec![(pred!(mut, [], []), _)]),
	void cv_VideoCapture_delete(cv::VideoCapture* instance) {
			delete instance;
	}

	// VideoWriter()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:874
	// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, [], []), _)]),
	void cv_VideoWriter_VideoWriter(Result<cv::VideoWriter*>* ocvrs_return) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// VideoWriter(const String &, int, double, Size, bool)(InString, Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:899
	// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize", "isColor"], ["const cv::String*", "int", "double", "cv::Size", "bool"]), _)]),
	void cv_VideoWriter_VideoWriter_const_StringR_int_double_Size_bool(const char* filename, int fourcc, double fps, cv::Size* frameSize, bool isColor, Result<cv::VideoWriter*>* ocvrs_return) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter(cv::String(filename), fourcc, fps, *frameSize, isColor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::VideoWriter::VideoWriter(InString, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:899
	// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize"], ["const cv::String*", "int", "double", "cv::Size"]), _)]),
	void cv_VideoWriter_VideoWriter_const_StringR_int_double_Size(const char* filename, int fourcc, double fps, cv::Size* frameSize, Result<cv::VideoWriter*>* ocvrs_return) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter(cv::String(filename), fourcc, fps, *frameSize);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// VideoWriter(const String &, int, int, double, Size, bool)(InString, Primitive, Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:906
	// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize", "isColor"], ["const cv::String*", "int", "int", "double", "cv::Size", "bool"]), _)]),
	void cv_VideoWriter_VideoWriter_const_StringR_int_int_double_Size_bool(const char* filename, int apiPreference, int fourcc, double fps, cv::Size* frameSize, bool isColor, Result<cv::VideoWriter*>* ocvrs_return) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter(cv::String(filename), apiPreference, fourcc, fps, *frameSize, isColor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::VideoWriter::VideoWriter(InString, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:906
	// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize"], ["const cv::String*", "int", "int", "double", "cv::Size"]), _)]),
	void cv_VideoWriter_VideoWriter_const_StringR_int_int_double_Size(const char* filename, int apiPreference, int fourcc, double fps, cv::Size* frameSize, Result<cv::VideoWriter*>* ocvrs_return) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter(cv::String(filename), apiPreference, fourcc, fps, *frameSize);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// open(const String &, int, double, Size, bool)(InString, Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:923
	// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize", "isColor"], ["const cv::String*", "int", "double", "cv::Size", "bool"]), _)]),
	void cv_VideoWriter_open_const_StringR_int_double_Size_bool(cv::VideoWriter* instance, const char* filename, int fourcc, double fps, cv::Size* frameSize, bool isColor, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(cv::String(filename), fourcc, fps, *frameSize, isColor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::VideoWriter::open(InString, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:923
	// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize"], ["const cv::String*", "int", "double", "cv::Size"]), _)]),
	void cv_VideoWriter_open_const_StringR_int_double_Size(cv::VideoWriter* instance, const char* filename, int fourcc, double fps, cv::Size* frameSize, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(cv::String(filename), fourcc, fps, *frameSize);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// open(const String &, int, int, double, Size, bool)(InString, Primitive, Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:928
	// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize", "isColor"], ["const cv::String*", "int", "int", "double", "cv::Size", "bool"]), _)]),
	void cv_VideoWriter_open_const_StringR_int_int_double_Size_bool(cv::VideoWriter* instance, const char* filename, int apiPreference, int fourcc, double fps, cv::Size* frameSize, bool isColor, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(cv::String(filename), apiPreference, fourcc, fps, *frameSize, isColor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::VideoWriter::open(InString, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:928
	// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize"], ["const cv::String*", "int", "int", "double", "cv::Size"]), _)]),
	void cv_VideoWriter_open_const_StringR_int_int_double_Size(cv::VideoWriter* instance, const char* filename, int apiPreference, int fourcc, double fps, cv::Size* frameSize, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(cv::String(filename), apiPreference, fourcc, fps, *frameSize);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isOpened()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:933
	// ("cv::VideoWriter::isOpened", vec![(pred!(const, [], []), _)]),
	void cv_VideoWriter_isOpened_const(const cv::VideoWriter* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isOpened();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// release()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:940
	// ("cv::VideoWriter::release", vec![(pred!(mut, [], []), _)]),
	void cv_VideoWriter_release(cv::VideoWriter* instance, ResultVoid* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:954
	// ("cv::VideoWriter::write", vec![(pred!(mut, ["image"], ["const cv::Mat*"]), _)]),
	void cv_VideoWriter_write_const_MatR(cv::VideoWriter* instance, const cv::Mat* image, ResultVoid* ocvrs_return) {
		try {
			instance->write(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// set(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:964
	// ("cv::VideoWriter::set", vec![(pred!(mut, ["propId", "value"], ["int", "double"]), _)]),
	void cv_VideoWriter_set_int_double(cv::VideoWriter* instance, int propId, double value, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->set(propId, value);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// get(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:974
	// ("cv::VideoWriter::get", vec![(pred!(const, ["propId"], ["int"]), _)]),
	void cv_VideoWriter_get_const_int(const cv::VideoWriter* instance, int propId, Result<double>* ocvrs_return) {
		try {
			double ret = instance->get(propId);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fourcc(char, char, char, char)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:983
	// ("cv::VideoWriter::fourcc", vec![(pred!(mut, ["c1", "c2", "c3", "c4"], ["char", "char", "char", "char"]), _)]),
	void cv_VideoWriter_fourcc_char_char_char_char(char c1, char c2, char c3, char c4, Result<int>* ocvrs_return) {
		try {
			int ret = cv::VideoWriter::fourcc(c1, c2, c3, c4);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackendName()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/videoio.hpp:989
	// ("cv::VideoWriter::getBackendName", vec![(pred!(const, [], []), _)]),
	void cv_VideoWriter_getBackendName_const(const cv::VideoWriter* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getBackendName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::VideoWriter::delete() generated
	// ("cv::VideoWriter::delete", vec![(pred!(mut, [], []), _)]),
	void cv_VideoWriter_delete(cv::VideoWriter* instance) {
			delete instance;
	}

}
