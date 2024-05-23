#include "ocvrs_common.hpp"
#include <opencv2/imgcodecs.hpp>
#include "imgcodecs_types.hpp"

extern "C" {
	// imdecode(InputArray, int)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:277
	// ("cv::imdecode", vec![(pred!(mut, ["buf", "flags"], ["const cv::_InputArray*", "int"]), _)]),
	void cv_imdecode_const__InputArrayR_int(const cv::_InputArray* buf, int flags, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::imdecode(*buf, flags);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imdecode(InputArray, int, Mat *)(InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:285
	// ("cv::imdecode", vec![(pred!(mut, ["buf", "flags", "dst"], ["const cv::_InputArray*", "int", "cv::Mat*"]), _)]),
	void cv_imdecode_const__InputArrayR_int_MatX(const cv::_InputArray* buf, int flags, cv::Mat* dst, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::imdecode(*buf, flags, dst);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imencode(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:297
	// ("cv::imencode", vec![(pred!(mut, ["ext", "img", "buf"], ["const cv::String*", "const cv::_InputArray*", "std::vector<unsigned char>*"]), _)]),
	void cv_imencode_const_StringR_const__InputArrayR_vectorLunsigned_charGR(const char* ext, const cv::_InputArray* img, std::vector<unsigned char>* buf, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imencode(cv::String(ext), *img, *buf);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imencode(const String &, InputArray, std::vector<uchar> &, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:297
	// ("cv::imencode", vec![(pred!(mut, ["ext", "img", "buf", "params"], ["const cv::String*", "const cv::_InputArray*", "std::vector<unsigned char>*", "const std::vector<int>*"]), _)]),
	void cv_imencode_const_StringR_const__InputArrayR_vectorLunsigned_charGR_const_vectorLintGR(const char* ext, const cv::_InputArray* img, std::vector<unsigned char>* buf, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imencode(cv::String(ext), *img, *buf, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imread(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:206
	// ("cv::imread", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
	void cv_imread_const_StringR(const char* filename, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::imread(cv::String(filename));
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imread(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:206
	// ("cv::imread", vec![(pred!(mut, ["filename", "flags"], ["const cv::String*", "int"]), _)]),
	void cv_imread_const_StringR_int(const char* filename, int flags, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::imread(cv::String(filename), flags);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imreadmulti(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:216
	// ("cv::imreadmulti", vec![(pred!(mut, ["filename", "mats"], ["const cv::String*", "std::vector<cv::Mat>*"]), _)]),
	void cv_imreadmulti_const_StringR_vectorLMatGR(const char* filename, std::vector<cv::Mat>* mats, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imreadmulti(cv::String(filename), *mats);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imreadmulti(const String &, std::vector<Mat> &, int)(InString, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:216
	// ("cv::imreadmulti", vec![(pred!(mut, ["filename", "mats", "flags"], ["const cv::String*", "std::vector<cv::Mat>*", "int"]), _)]),
	void cv_imreadmulti_const_StringR_vectorLMatGR_int(const char* filename, std::vector<cv::Mat>* mats, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imreadmulti(cv::String(filename), *mats, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imwrite(InString, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:255
	// ("cv::imwrite", vec![(pred!(mut, ["filename", "img"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
	void cv_imwrite_const_StringR_const__InputArrayR(const char* filename, const cv::_InputArray* img, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imwrite(cv::String(filename), *img);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imwrite(const String &, InputArray, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:255
	// ("cv::imwrite", vec![(pred!(mut, ["filename", "img", "params"], ["const cv::String*", "const cv::_InputArray*", "const std::vector<int>*"]), _)]),
	void cv_imwrite_const_StringR_const__InputArrayR_const_vectorLintGR(const char* filename, const cv::_InputArray* img, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imwrite(cv::String(filename), *img, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::imwritemulti(InString, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:260
	// ("cv::imwritemulti", vec![(pred!(mut, ["filename", "img"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
	void cv_imwritemulti_const_StringR_const__InputArrayR(const char* filename, const cv::_InputArray* img, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imwritemulti(cv::String(filename), *img);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imwritemulti(const String &, InputArrayOfArrays, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:260
	// ("cv::imwritemulti", vec![(pred!(mut, ["filename", "img", "params"], ["const cv::String*", "const cv::_InputArray*", "const std::vector<int>*"]), _)]),
	void cv_imwritemulti_const_StringR_const__InputArrayR_const_vectorLintGR(const char* filename, const cv::_InputArray* img, const std::vector<int>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::imwritemulti(cv::String(filename), *img, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

}
