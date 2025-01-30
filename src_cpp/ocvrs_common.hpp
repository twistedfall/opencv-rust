#ifndef __OCVRS_COMMON_HPP__
#define __OCVRS_COMMON_HPP__

#include <memory>
#include <opencv2/cvconfig.h>
// defining HAVE_VA starts to rely on <va/va.h> for VADisplay and VASurfaceID instead of OpenCV stubs, and we stop generating
// bindings for the functions that use them
#undef HAVE_VA

#define CV_COLLECT_IMPL_DATA
#ifdef OCVRS_PARSING_HEADERS
	// strip dnn experimental ns when generating headers
	#define CV_DNN_DONT_ADD_EXPERIMENTAL_NS
	#define CV_DNN_DONT_ADD_INLINE_NS
	// the FFI export suffix only matters during actual linking
	#define OCVRS_FFI_EXPORT_SUFFIX
#endif

#include <opencv2/core.hpp>

#define OCVRS_ONLY_DEPENDENT_TYPES

#define OCVRS_HANDLE(code, msg, return_name) Err(code, msg, return_name)

#define OCVRS_CATCH(return_name) \
catch (cv::Exception& e) { \
	OCVRS_HANDLE(e.code, e.what(), return_name); \
} catch (std::exception &e) { \
	OCVRS_HANDLE(cv::Error::StsError, e.what(), return_name); \
} catch (...) { \
	OCVRS_HANDLE(cv::Error::StsError, "Unspecified error, neither from OpenCV nor from std", return_name); \
}

// double-expansion macro trick to expand the OCVRS_FFI_EXPORT_SUFFIX macro
#define CONCATENATE(prefix, suffix) prefix##suffix
#define SUFFIXED_NAME(name, suffix) CONCATENATE(name, suffix)

// defined in build/generator/collector.rs Collector::inject_ffi_exports, see `inject_ffi_exports()` function for explanation
extern "C" void* SUFFIXED_NAME(ocvrs_create_string, OCVRS_FFI_EXPORT_SUFFIX)(const char*);
extern "C" void* SUFFIXED_NAME(ocvrs_create_byte_string, OCVRS_FFI_EXPORT_SUFFIX)(const char*, size_t);

// "aliases" for the above functions provided by Rust, to be used in the generated code
inline void* ocvrs_create_string(const char* s) { return SUFFIXED_NAME(ocvrs_create_string, OCVRS_FFI_EXPORT_SUFFIX)(s); }
inline void* ocvrs_create_byte_string(const char* s, size_t len) { return SUFFIXED_NAME(ocvrs_create_byte_string, OCVRS_FFI_EXPORT_SUFFIX)(s, len); }

template<typename T> struct Result {
	int error_code;
	void* error_msg;
	T result;
};

struct ResultVoid {
	int error_code;
	void* error_msg;
};

template<typename T, typename R> inline void Ok(T result, Result<R>* ocvrs_return) {
	ocvrs_return->error_code = 0;
	ocvrs_return->error_msg = NULL;
	ocvrs_return->result = *const_cast<R*>(&result);
}

inline void Ok(ResultVoid* ocvrs_return) {
	ocvrs_return->error_code = 0;
	ocvrs_return->error_msg = NULL;
}

template<typename T> inline void Err(int code, const char* msg, T* ocvrs_return) {
	ocvrs_return->error_code = code;
	ocvrs_return->error_msg = ocvrs_create_string(msg);
	// it's ok to leave result uninitialized because the Rust implementation only assumes it as init if error_msg is NULL
}

#endif
