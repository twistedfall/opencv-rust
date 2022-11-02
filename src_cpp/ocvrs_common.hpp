#ifndef __OCVRS_COMMON_HPP__
#define __OCVRS_COMMON_HPP__

#if defined(_WIN32) || defined(_WIN64) || defined(__WIN32__) || defined(__TOS_WIN__) || defined(__WINDOWS__) \
 || defined(__CYGWIN__) || defined(__MINGW32__) || defined(__BORLANDC__)
#define OCVRS_TARGET_OS_WINDOWS
#endif

#define CV_COLLECT_IMPL_DATA
// strip dnn experimental ns when generating headers
#ifdef OCVRS_PARSING_HEADERS
	#define CV_DNN_DONT_ADD_EXPERIMENTAL_NS
	#define CV_DNN_DONT_ADD_INLINE_NS
#endif
#include <opencv2/core.hpp>

#define OCVRS_ONLY_DEPENDENT_TYPES
// needed to be able to handle commas in the type name in call to OCVRS_CATCH
#define OCVRS_TYPE(...) __VA_ARGS__

#define OCVRS_HANDLE(code, msg, return_type, return_name) Err<return_type>(code, msg, return_name)

#define OCVRS_HANDLE_OPENCV(e, return_type, return_name) \
OCVRS_HANDLE(e.code, e.what(), OCVRS_TYPE(return_type), return_name)

#define OCVRS_HANDLE_UNSPECIFIED(return_type, return_name) \
OCVRS_HANDLE(-99999, "unspecified error in OpenCV guts", OCVRS_TYPE(return_type), return_name)

#define OCVRS_CATCH(return_type, return_name) \
catch (cv::Exception& e) { \
	OCVRS_HANDLE_OPENCV(e, OCVRS_TYPE(return_type), return_name); \
} catch (...) { \
	OCVRS_HANDLE_UNSPECIFIED(OCVRS_TYPE(return_type), return_name); \
}

// defined in src/templ.rs
extern "C" void* ocvrs_create_string(const char*);
extern "C" void* ocvrs_create_byte_string(const char*, size_t);

template<typename T> struct Result {
	int error_code;
	void* error_msg;
	T result;
};

struct Result_void {
	int error_code;
	void* error_msg;
};

template<typename T, typename R> inline void Ok(T result, Result<R>* ocvrs_return) {
	ocvrs_return->error_code = 0;
	ocvrs_return->error_msg = NULL;
	ocvrs_return->result = *const_cast<R*>(&result);
}

inline void Ok(Result_void* ocvrs_return) {
	ocvrs_return->error_code = 0;
	ocvrs_return->error_msg = NULL;
}

template<typename T> inline void Err(int code, const char* msg, T* ocvrs_return) {
	ocvrs_return->error_code = code;
	ocvrs_return->error_msg = ocvrs_create_string(msg);
	// it's ok to leave result uninitialized because the Rust implementation only assumes it as init if error_msg is NULL
}

#endif
