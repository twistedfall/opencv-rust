#include "videoio.hpp"
#include "videoio_types.hpp"

extern "C" {
	// getBackendName(VideoCaptureAPIs)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio/registry.hpp:27
	// ("cv::videoio_registry::getBackendName", vec![(pred!(mut, ["api"], ["cv::VideoCaptureAPIs"]), _)]),
	void cv_videoio_registry_getBackendName_VideoCaptureAPIs(cv::VideoCaptureAPIs api, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::videoio_registry::getBackendName(api);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackends()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio/registry.hpp:30
	// ("cv::videoio_registry::getBackends", vec![(pred!(mut, [], []), _)]),
	void cv_videoio_registry_getBackends(Result<std::vector<cv::VideoCaptureAPIs>*>* ocvrs_return) {
		try {
			std::vector<cv::VideoCaptureAPIs> ret = cv::videoio_registry::getBackends();
			Ok(new std::vector<cv::VideoCaptureAPIs>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCameraBackendPluginVersion(VideoCaptureAPIs, int &, int &)(Enum, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio/registry.hpp:51
	// ("cv::videoio_registry::getCameraBackendPluginVersion", vec![(pred!(mut, ["api", "version_ABI", "version_API"], ["cv::VideoCaptureAPIs", "int*", "int*"]), _)]),
	void cv_videoio_registry_getCameraBackendPluginVersion_VideoCaptureAPIs_intR_intR(cv::VideoCaptureAPIs api, int* version_ABI, int* version_API, Result<void*>* ocvrs_return) {
		try {
			std::string ret = cv::videoio_registry::getCameraBackendPluginVersion(api, *version_ABI, *version_API);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCameraBackends()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio/registry.hpp:33
	// ("cv::videoio_registry::getCameraBackends", vec![(pred!(mut, [], []), _)]),
	void cv_videoio_registry_getCameraBackends(Result<std::vector<cv::VideoCaptureAPIs>*>* ocvrs_return) {
		try {
			std::vector<cv::VideoCaptureAPIs> ret = cv::videoio_registry::getCameraBackends();
			Ok(new std::vector<cv::VideoCaptureAPIs>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getStreamBackendPluginVersion(VideoCaptureAPIs, int &, int &)(Enum, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio/registry.hpp:58
	// ("cv::videoio_registry::getStreamBackendPluginVersion", vec![(pred!(mut, ["api", "version_ABI", "version_API"], ["cv::VideoCaptureAPIs", "int*", "int*"]), _)]),
	void cv_videoio_registry_getStreamBackendPluginVersion_VideoCaptureAPIs_intR_intR(cv::VideoCaptureAPIs api, int* version_ABI, int* version_API, Result<void*>* ocvrs_return) {
		try {
			std::string ret = cv::videoio_registry::getStreamBackendPluginVersion(api, *version_ABI, *version_API);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getStreamBackends()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio/registry.hpp:36
	// ("cv::videoio_registry::getStreamBackends", vec![(pred!(mut, [], []), _)]),
	void cv_videoio_registry_getStreamBackends(Result<std::vector<cv::VideoCaptureAPIs>*>* ocvrs_return) {
		try {
			std::vector<cv::VideoCaptureAPIs> ret = cv::videoio_registry::getStreamBackends();
			Ok(new std::vector<cv::VideoCaptureAPIs>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getStreamBufferedBackendPluginVersion(VideoCaptureAPIs, int &, int &)(Enum, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio/registry.hpp:65
	// ("cv::videoio_registry::getStreamBufferedBackendPluginVersion", vec![(pred!(mut, ["api", "version_ABI", "version_API"], ["cv::VideoCaptureAPIs", "int*", "int*"]), _)]),
	void cv_videoio_registry_getStreamBufferedBackendPluginVersion_VideoCaptureAPIs_intR_intR(cv::VideoCaptureAPIs api, int* version_ABI, int* version_API, Result<void*>* ocvrs_return) {
		try {
			std::string ret = cv::videoio_registry::getStreamBufferedBackendPluginVersion(api, *version_ABI, *version_API);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getStreamBufferedBackends()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio/registry.hpp:39
	// ("cv::videoio_registry::getStreamBufferedBackends", vec![(pred!(mut, [], []), _)]),
	void cv_videoio_registry_getStreamBufferedBackends(Result<std::vector<cv::VideoCaptureAPIs>*>* ocvrs_return) {
		try {
			std::vector<cv::VideoCaptureAPIs> ret = cv::videoio_registry::getStreamBufferedBackends();
			Ok(new std::vector<cv::VideoCaptureAPIs>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWriterBackendPluginVersion(VideoCaptureAPIs, int &, int &)(Enum, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio/registry.hpp:72
	// ("cv::videoio_registry::getWriterBackendPluginVersion", vec![(pred!(mut, ["api", "version_ABI", "version_API"], ["cv::VideoCaptureAPIs", "int*", "int*"]), _)]),
	void cv_videoio_registry_getWriterBackendPluginVersion_VideoCaptureAPIs_intR_intR(cv::VideoCaptureAPIs api, int* version_ABI, int* version_API, Result<void*>* ocvrs_return) {
		try {
			std::string ret = cv::videoio_registry::getWriterBackendPluginVersion(api, *version_ABI, *version_API);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWriterBackends()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio/registry.hpp:42
	// ("cv::videoio_registry::getWriterBackends", vec![(pred!(mut, [], []), _)]),
	void cv_videoio_registry_getWriterBackends(Result<std::vector<cv::VideoCaptureAPIs>*>* ocvrs_return) {
		try {
			std::vector<cv::VideoCaptureAPIs> ret = cv::videoio_registry::getWriterBackends();
			Ok(new std::vector<cv::VideoCaptureAPIs>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// hasBackend(VideoCaptureAPIs)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio/registry.hpp:45
	// ("cv::videoio_registry::hasBackend", vec![(pred!(mut, ["api"], ["cv::VideoCaptureAPIs"]), _)]),
	void cv_videoio_registry_hasBackend_VideoCaptureAPIs(cv::VideoCaptureAPIs api, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::videoio_registry::hasBackend(api);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isBackendBuiltIn(VideoCaptureAPIs)(Enum) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio/registry.hpp:48
	// ("cv::videoio_registry::isBackendBuiltIn", vec![(pred!(mut, ["api"], ["cv::VideoCaptureAPIs"]), _)]),
	void cv_videoio_registry_isBackendBuiltIn_VideoCaptureAPIs(cv::VideoCaptureAPIs api, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::videoio_registry::isBackendBuiltIn(api);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(char *, long long)(OutString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:730
	// ("cv::IStreamReader::read", vec![(pred!(mut, ["buffer", "size"], ["char*", "long long"]), _)]),
	void cv_IStreamReader_read_charX_long_long(cv::IStreamReader* instance, void** buffer, long long size, Result<long long>* ocvrs_return) {
		try {
			std::unique_ptr<char[]> buffer_out = std::make_unique<char[]>(1024);
			long long ret = instance->read(buffer_out.get(), size);
			*buffer = ocvrs_create_string(buffer_out.get());
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// seek(long long, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:739
	// ("cv::IStreamReader::seek", vec![(pred!(mut, ["offset", "origin"], ["long long", "int"]), _)]),
	void cv_IStreamReader_seek_long_long_int(cv::IStreamReader* instance, long long offset, int origin, Result<long long>* ocvrs_return) {
		try {
			long long ret = instance->seek(offset, origin);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::IStreamReader::delete() generated
	// ("cv::IStreamReader::delete", vec![(pred!(mut, [], []), _)]),
	void cv_IStreamReader_delete(cv::IStreamReader* instance) {
			delete instance;
	}

	// VideoCapture()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:773
	// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, [], []), _)]),
	void cv_VideoCapture_VideoCapture(Result<cv::VideoCapture*>* ocvrs_return) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// VideoCapture(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:790
	// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["filename", "apiPreference"], ["const cv::String*", "int"]), _)]),
	void cv_VideoCapture_VideoCapture_const_StringR_int(const char* filename, int apiPreference, Result<cv::VideoCapture*>* ocvrs_return) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture(std::string(filename), apiPreference);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::VideoCapture::VideoCapture(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:790
	// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_VideoCapture_VideoCapture_const_StringR(const char* filename, Result<cv::VideoCapture*>* ocvrs_return) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// VideoCapture(const String &, int, const std::vector<int> &)(InString, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:798
	// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["filename", "apiPreference", "params"], ["const cv::String*", "int", "const std::vector<int>*"]), _)]),
	void cv_VideoCapture_VideoCapture_const_StringR_int_const_vectorLintGR(const char* filename, int apiPreference, const std::vector<int>* params, Result<cv::VideoCapture*>* ocvrs_return) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture(std::string(filename), apiPreference, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// VideoCapture(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:810
	// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["index", "apiPreference"], ["int", "int"]), _)]),
	void cv_VideoCapture_VideoCapture_int_int(int index, int apiPreference, Result<cv::VideoCapture*>* ocvrs_return) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture(index, apiPreference);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::VideoCapture::VideoCapture(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:810
	// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["index"], ["int"]), _)]),
	void cv_VideoCapture_VideoCapture_int(int index, Result<cv::VideoCapture*>* ocvrs_return) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture(index);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// VideoCapture(int, int, const std::vector<int> &)(Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:818
	// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["index", "apiPreference", "params"], ["int", "int", "const std::vector<int>*"]), _)]),
	void cv_VideoCapture_VideoCapture_int_int_const_vectorLintGR(int index, int apiPreference, const std::vector<int>* params, Result<cv::VideoCapture*>* ocvrs_return) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture(index, apiPreference, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// VideoCapture(const Ptr<IStreamReader> &, int, const std::vector<int> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:826
	// ("cv::VideoCapture::VideoCapture", vec![(pred!(mut, ["source", "apiPreference", "params"], ["const cv::Ptr<cv::IStreamReader>*", "int", "const std::vector<int>*"]), _)]),
	void cv_VideoCapture_VideoCapture_const_PtrLIStreamReaderGR_int_const_vectorLintGR(const cv::Ptr<cv::IStreamReader>* source, int apiPreference, const std::vector<int>* params, Result<cv::VideoCapture*>* ocvrs_return) {
		try {
			cv::VideoCapture* ret = new cv::VideoCapture(*source, apiPreference, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// open(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:843
	// ("cv::VideoCapture::open", vec![(pred!(mut, ["filename", "apiPreference"], ["const cv::String*", "int"]), _)]),
	void cv_VideoCapture_open_const_StringR_int(cv::VideoCapture* instance, const char* filename, int apiPreference, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(std::string(filename), apiPreference);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::VideoCapture::open(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:843
	// ("cv::VideoCapture::open", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_VideoCapture_open_const_StringR(cv::VideoCapture* instance, const char* filename, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// open(const String &, int, const std::vector<int> &)(InString, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:856
	// ("cv::VideoCapture::open", vec![(pred!(mut, ["filename", "apiPreference", "params"], ["const cv::String*", "int", "const std::vector<int>*"]), _)]),
	void cv_VideoCapture_open_const_StringR_int_const_vectorLintGR(cv::VideoCapture* instance, const char* filename, int apiPreference, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(std::string(filename), apiPreference, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// open(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:867
	// ("cv::VideoCapture::open", vec![(pred!(mut, ["index", "apiPreference"], ["int", "int"]), _)]),
	void cv_VideoCapture_open_int_int(cv::VideoCapture* instance, int index, int apiPreference, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(index, apiPreference);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::VideoCapture::open(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:867
	// ("cv::VideoCapture::open", vec![(pred!(mut, ["index"], ["int"]), _)]),
	void cv_VideoCapture_open_int(cv::VideoCapture* instance, int index, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(index);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// open(int, int, const std::vector<int> &)(Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:880
	// ("cv::VideoCapture::open", vec![(pred!(mut, ["index", "apiPreference", "params"], ["int", "int", "const std::vector<int>*"]), _)]),
	void cv_VideoCapture_open_int_int_const_vectorLintGR(cv::VideoCapture* instance, int index, int apiPreference, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(index, apiPreference, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// open(const Ptr<IStreamReader> &, int, const std::vector<int> &)(CppPassByVoidPtr, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:893
	// ("cv::VideoCapture::open", vec![(pred!(mut, ["source", "apiPreference", "params"], ["const cv::Ptr<cv::IStreamReader>*", "int", "const std::vector<int>*"]), _)]),
	void cv_VideoCapture_open_const_PtrLIStreamReaderGR_int_const_vectorLintGR(cv::VideoCapture* instance, const cv::Ptr<cv::IStreamReader>* source, int apiPreference, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(*source, apiPreference, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isOpened()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:900
	// ("cv::VideoCapture::isOpened", vec![(pred!(const, [], []), _)]),
	void cv_VideoCapture_isOpened_const(const cv::VideoCapture* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isOpened();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// release()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:909
	// ("cv::VideoCapture::release", vec![(pred!(mut, [], []), _)]),
	void cv_VideoCapture_release(cv::VideoCapture* instance, ResultVoid* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// grab()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:930
	// ("cv::VideoCapture::grab", vec![(pred!(mut, [], []), _)]),
	void cv_VideoCapture_grab(cv::VideoCapture* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->grab();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// retrieve(OutputArray, int)(OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:948
	// ("cv::VideoCapture::retrieve", vec![(pred!(mut, ["image", "flag"], ["const cv::_OutputArray*", "int"]), _)]),
	void cv_VideoCapture_retrieve_const__OutputArrayR_int(cv::VideoCapture* instance, const cv::_OutputArray* image, int flag, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->retrieve(*image, flag);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::VideoCapture::retrieve(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:948
	// ("cv::VideoCapture::retrieve", vec![(pred!(mut, ["image"], ["const cv::_OutputArray*"]), _)]),
	void cv_VideoCapture_retrieve_const__OutputArrayR(cv::VideoCapture* instance, const cv::_OutputArray* image, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->retrieve(*image);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:974
	// ("cv::VideoCapture::read", vec![(pred!(mut, ["image"], ["const cv::_OutputArray*"]), _)]),
	void cv_VideoCapture_read_const__OutputArrayR(cv::VideoCapture* instance, const cv::_OutputArray* image, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->read(*image);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// set(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:985
	// ("cv::VideoCapture::set", vec![(pred!(mut, ["propId", "value"], ["int", "double"]), _)]),
	void cv_VideoCapture_set_int_double(cv::VideoCapture* instance, int propId, double value, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->set(propId, value);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// get(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1004
	// ("cv::VideoCapture::get", vec![(pred!(const, ["propId"], ["int"]), _)]),
	void cv_VideoCapture_get_const_int(const cv::VideoCapture* instance, int propId, Result<double>* ocvrs_return) {
		try {
			double ret = instance->get(propId);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackendName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1010
	// ("cv::VideoCapture::getBackendName", vec![(pred!(const, [], []), _)]),
	void cv_VideoCapture_getBackendName_const(const cv::VideoCapture* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getBackendName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setExceptionMode(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1016
	// ("cv::VideoCapture::setExceptionMode", vec![(pred!(mut, ["enable"], ["bool"]), _)]),
	void cv_VideoCapture_setExceptionMode_bool(cv::VideoCapture* instance, bool enable, ResultVoid* ocvrs_return) {
		try {
			instance->setExceptionMode(enable);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getExceptionMode()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1019
	// ("cv::VideoCapture::getExceptionMode", vec![(pred!(const, [], []), _)]),
	void cv_VideoCapture_getExceptionMode_const(const cv::VideoCapture* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getExceptionMode();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// waitAny(const std::vector<VideoCapture> &, std::vector<int> &, int64)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1037
	// ("cv::VideoCapture::waitAny", vec![(pred!(mut, ["streams", "readyIndex", "timeoutNs"], ["const std::vector<cv::VideoCapture>*", "std::vector<int>*", "int64_t"]), _)]),
	void cv_VideoCapture_waitAny_const_vectorLVideoCaptureGR_vectorLintGR_int64_t(const std::vector<cv::VideoCapture>* streams, std::vector<int>* readyIndex, int64_t timeoutNs, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::VideoCapture::waitAny(*streams, *readyIndex, timeoutNs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::VideoCapture::waitAny(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1037
	// ("cv::VideoCapture::waitAny", vec![(pred!(mut, ["streams", "readyIndex"], ["const std::vector<cv::VideoCapture>*", "std::vector<int>*"]), _)]),
	void cv_VideoCapture_waitAny_const_vectorLVideoCaptureGR_vectorLintGR(const std::vector<cv::VideoCapture>* streams, std::vector<int>* readyIndex, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::VideoCapture::waitAny(*streams, *readyIndex);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::VideoCapture::delete() generated
	// ("cv::VideoCapture::delete", vec![(pred!(mut, [], []), _)]),
	void cv_VideoCapture_delete(cv::VideoCapture* instance) {
			delete instance;
	}

	// VideoWriter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1074
	// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, [], []), _)]),
	void cv_VideoWriter_VideoWriter(Result<cv::VideoWriter*>* ocvrs_return) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// VideoWriter(const String &, int, double, Size, bool)(InString, Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1102
	// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize", "isColor"], ["const cv::String*", "int", "double", "cv::Size", "bool"]), _)]),
	void cv_VideoWriter_VideoWriter_const_StringR_int_double_Size_bool(const char* filename, int fourcc, double fps, cv::Size* frameSize, bool isColor, Result<cv::VideoWriter*>* ocvrs_return) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter(std::string(filename), fourcc, fps, *frameSize, isColor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::VideoWriter::VideoWriter(InString, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1102
	// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize"], ["const cv::String*", "int", "double", "cv::Size"]), _)]),
	void cv_VideoWriter_VideoWriter_const_StringR_int_double_Size(const char* filename, int fourcc, double fps, cv::Size* frameSize, Result<cv::VideoWriter*>* ocvrs_return) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter(std::string(filename), fourcc, fps, *frameSize);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// VideoWriter(const String &, int, int, double, Size, bool)(InString, Primitive, Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1109
	// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize", "isColor"], ["const cv::String*", "int", "int", "double", "cv::Size", "bool"]), _)]),
	void cv_VideoWriter_VideoWriter_const_StringR_int_int_double_Size_bool(const char* filename, int apiPreference, int fourcc, double fps, cv::Size* frameSize, bool isColor, Result<cv::VideoWriter*>* ocvrs_return) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter(std::string(filename), apiPreference, fourcc, fps, *frameSize, isColor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::VideoWriter::VideoWriter(InString, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1109
	// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize"], ["const cv::String*", "int", "int", "double", "cv::Size"]), _)]),
	void cv_VideoWriter_VideoWriter_const_StringR_int_int_double_Size(const char* filename, int apiPreference, int fourcc, double fps, cv::Size* frameSize, Result<cv::VideoWriter*>* ocvrs_return) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter(std::string(filename), apiPreference, fourcc, fps, *frameSize);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// VideoWriter(const String &, int, double, const Size &, const std::vector<int> &)(InString, Primitive, Primitive, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1116
	// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize", "params"], ["const cv::String*", "int", "double", "const cv::Size*", "const std::vector<int>*"]), _)]),
	void cv_VideoWriter_VideoWriter_const_StringR_int_double_const_SizeR_const_vectorLintGR(const char* filename, int fourcc, double fps, const cv::Size* frameSize, const std::vector<int>* params, Result<cv::VideoWriter*>* ocvrs_return) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter(std::string(filename), fourcc, fps, *frameSize, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// VideoWriter(const String &, int, int, double, const Size &, const std::vector<int> &)(InString, Primitive, Primitive, Primitive, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1121
	// ("cv::VideoWriter::VideoWriter", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize", "params"], ["const cv::String*", "int", "int", "double", "const cv::Size*", "const std::vector<int>*"]), _)]),
	void cv_VideoWriter_VideoWriter_const_StringR_int_int_double_const_SizeR_const_vectorLintGR(const char* filename, int apiPreference, int fourcc, double fps, const cv::Size* frameSize, const std::vector<int>* params, Result<cv::VideoWriter*>* ocvrs_return) {
		try {
			cv::VideoWriter* ret = new cv::VideoWriter(std::string(filename), apiPreference, fourcc, fps, *frameSize, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// open(const String &, int, double, Size, bool)(InString, Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1138
	// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize", "isColor"], ["const cv::String*", "int", "double", "cv::Size", "bool"]), _)]),
	void cv_VideoWriter_open_const_StringR_int_double_Size_bool(cv::VideoWriter* instance, const char* filename, int fourcc, double fps, cv::Size* frameSize, bool isColor, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(std::string(filename), fourcc, fps, *frameSize, isColor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::VideoWriter::open(InString, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1138
	// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize"], ["const cv::String*", "int", "double", "cv::Size"]), _)]),
	void cv_VideoWriter_open_const_StringR_int_double_Size(cv::VideoWriter* instance, const char* filename, int fourcc, double fps, cv::Size* frameSize, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(std::string(filename), fourcc, fps, *frameSize);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// open(const String &, int, int, double, Size, bool)(InString, Primitive, Primitive, Primitive, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1143
	// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize", "isColor"], ["const cv::String*", "int", "int", "double", "cv::Size", "bool"]), _)]),
	void cv_VideoWriter_open_const_StringR_int_int_double_Size_bool(cv::VideoWriter* instance, const char* filename, int apiPreference, int fourcc, double fps, cv::Size* frameSize, bool isColor, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(std::string(filename), apiPreference, fourcc, fps, *frameSize, isColor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::VideoWriter::open(InString, Primitive, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1143
	// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize"], ["const cv::String*", "int", "int", "double", "cv::Size"]), _)]),
	void cv_VideoWriter_open_const_StringR_int_int_double_Size(cv::VideoWriter* instance, const char* filename, int apiPreference, int fourcc, double fps, cv::Size* frameSize, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(std::string(filename), apiPreference, fourcc, fps, *frameSize);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// open(const String &, int, double, const Size &, const std::vector<int> &)(InString, Primitive, Primitive, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1148
	// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "fourcc", "fps", "frameSize", "params"], ["const cv::String*", "int", "double", "const cv::Size*", "const std::vector<int>*"]), _)]),
	void cv_VideoWriter_open_const_StringR_int_double_const_SizeR_const_vectorLintGR(cv::VideoWriter* instance, const char* filename, int fourcc, double fps, const cv::Size* frameSize, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(std::string(filename), fourcc, fps, *frameSize, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// open(const String &, int, int, double, const Size &, const std::vector<int> &)(InString, Primitive, Primitive, Primitive, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1153
	// ("cv::VideoWriter::open", vec![(pred!(mut, ["filename", "apiPreference", "fourcc", "fps", "frameSize", "params"], ["const cv::String*", "int", "int", "double", "const cv::Size*", "const std::vector<int>*"]), _)]),
	void cv_VideoWriter_open_const_StringR_int_int_double_const_SizeR_const_vectorLintGR(cv::VideoWriter* instance, const char* filename, int apiPreference, int fourcc, double fps, const cv::Size* frameSize, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(std::string(filename), apiPreference, fourcc, fps, *frameSize, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isOpened()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1158
	// ("cv::VideoWriter::isOpened", vec![(pred!(const, [], []), _)]),
	void cv_VideoWriter_isOpened_const(const cv::VideoWriter* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isOpened();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// release()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1165
	// ("cv::VideoWriter::release", vec![(pred!(mut, [], []), _)]),
	void cv_VideoWriter_release(cv::VideoWriter* instance, ResultVoid* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1184
	// ("cv::VideoWriter::write", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
	void cv_VideoWriter_write_const__InputArrayR(cv::VideoWriter* instance, const cv::_InputArray* image, ResultVoid* ocvrs_return) {
		try {
			instance->write(*image);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// set(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1194
	// ("cv::VideoWriter::set", vec![(pred!(mut, ["propId", "value"], ["int", "double"]), _)]),
	void cv_VideoWriter_set_int_double(cv::VideoWriter* instance, int propId, double value, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->set(propId, value);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// get(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1204
	// ("cv::VideoWriter::get", vec![(pred!(const, ["propId"], ["int"]), _)]),
	void cv_VideoWriter_get_const_int(const cv::VideoWriter* instance, int propId, Result<double>* ocvrs_return) {
		try {
			double ret = instance->get(propId);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fourcc(char, char, char, char)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1213
	// ("cv::VideoWriter::fourcc", vec![(pred!(mut, ["c1", "c2", "c3", "c4"], ["char", "char", "char", "char"]), _)]),
	void cv_VideoWriter_fourcc_char_char_char_char(char c1, char c2, char c3, char c4, Result<int>* ocvrs_return) {
		try {
			int ret = cv::VideoWriter::fourcc(c1, c2, c3, c4);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackendName()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/videoio.hpp:1219
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
