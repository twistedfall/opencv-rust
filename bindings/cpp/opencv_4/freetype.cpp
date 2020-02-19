#include "common.hpp"
#include <opencv2/freetype.hpp>
#include "freetype_types.hpp"

extern "C" {
	Result<void*> cv_freetype_createFreeType2() {
		try {
			cv::Ptr<cv::freetype::FreeType2> ret = cv::freetype::createFreeType2();
			return Ok<void*>(new cv::Ptr<cv::freetype::FreeType2>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_freetype_FreeType2_loadFontData_String_int(void* instance, char* fontFileName, int id) {
		try {
			reinterpret_cast<cv::freetype::FreeType2*>(instance)->loadFontData(std::string(fontFileName), id);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_freetype_FreeType2_setSplitNumber_int(void* instance, int num) {
		try {
			reinterpret_cast<cv::freetype::FreeType2*>(instance)->setSplitNumber(num);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_freetype_FreeType2_putText_const__InputOutputArrayX_const_StringX_Point_int_Scalar_int_int_bool(void* instance, void* img, const char* text, cv::Point org, int fontHeight, cv::Scalar color, int thickness, int line_type, bool bottomLeftOrigin) {
		try {
			reinterpret_cast<cv::freetype::FreeType2*>(instance)->putText(*reinterpret_cast<const cv::_InputOutputArray*>(img), std::string(text), org, fontHeight, color, thickness, line_type, bottomLeftOrigin);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Size> cv_freetype_FreeType2_getTextSize_const_StringX_int_int_intX(void* instance, const char* text, int fontHeight, int thickness, int* baseLine) {
		try {
			cv::Size ret = reinterpret_cast<cv::freetype::FreeType2*>(instance)->getTextSize(std::string(text), fontHeight, thickness, baseLine);
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
}
