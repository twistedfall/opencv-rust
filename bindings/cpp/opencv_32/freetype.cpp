#include "common.hpp"
#include <opencv2/freetype.hpp>
#include "freetype_types.hpp"

extern "C" {
	Result<cv::Ptr<cv::freetype::FreeType2>*> cv_freetype_createFreeType2() {
		try {
			cv::Ptr<cv::freetype::FreeType2> ret = cv::freetype::createFreeType2();
			return Ok(new cv::Ptr<cv::freetype::FreeType2>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::freetype::FreeType2>*>))
	}
	
	Result_void cv_freetype_FreeType2_loadFontData_String_int(cv::freetype::FreeType2* instance, char* fontFileName, int id) {
		try {
			instance->loadFontData(cv::String(fontFileName), id);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_freetype_FreeType2_setSplitNumber_int(cv::freetype::FreeType2* instance, int num) {
		try {
			instance->setSplitNumber(num);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_freetype_FreeType2_putText_const__InputOutputArrayX_const_StringX_Point_int_Scalar_int_int_bool(cv::freetype::FreeType2* instance, const cv::_InputOutputArray* img, const char* text, const cv::Point* org, int fontHeight, const cv::Scalar* color, int thickness, int line_type, bool bottomLeftOrigin) {
		try {
			instance->putText(*img, cv::String(text), *org, fontHeight, *color, thickness, line_type, bottomLeftOrigin);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
