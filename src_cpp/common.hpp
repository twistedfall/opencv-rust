#define OCVRS_ONLY_DEPENDENT_TYPES

#define CODE_CATCH(return_type, exc_type, code, msg) \
catch (exc_type& e) { \
	return_type ret; \
	memset(&ret, 0x00, sizeof(ret)); \
	ret.error_code = code; \
	ret.error_msg = ocvrs_create_string(e.what()); \
	return ret; \
}

#define OCVRS_CATCH(return_type) \
catch (cv::Exception& e) { \
	return_type ret; \
	memset(&ret, 0x00, sizeof(ret)); \
	ret.error_code = e.code; \
	ret.error_msg = ocvrs_create_string(e.what()); \
	return ret; \
} catch (...) { \
	return_type ret; \
	memset(&ret, 0x00, sizeof(ret)); \
	ret.error_code = -99999; \
	ret.error_msg = ocvrs_create_string("unspecified error in OpenCV guts"); \
	return ret; \
}

#define VEC_CATCH(return_type) CODE_CATCH(return_type, std::out_of_range, cv::Error::Code::StsOutOfRange, "index out of bounds")

template<typename T> struct Result {
	int error_code;
	void* error_msg;
	T result;
};

struct Result_void {
	int error_code;
	void* error_msg;
};

// these dummy explicit specializations are needed for cl.exe to prevent
// "C linkage function cannot return C++ class" error

template<typename T> inline Result<T> Ok(T result) {
	return Result<T> { .error_code = 0, .error_msg = 0, .result = result };
}

inline Result_void Ok() {
	return Result_void { .error_code = 0, .error_msg = 0 };
}

// defined in src/templ.rs
extern "C" void* ocvrs_create_string(const char*);
