#include "ocvrs_common.hpp"
#include <opencv2/freetype.hpp>
#include "freetype_types.hpp"

extern "C" {
	// createFreeType2()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/freetype.hpp:197
	// ("cv::freetype::createFreeType2", vec![(pred!(mut, [], []), _)]),
	void cv_freetype_createFreeType2(Result<cv::Ptr<cv::freetype::FreeType2>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::freetype::FreeType2> ret = cv::freetype::createFreeType2();
			Ok(new cv::Ptr<cv::freetype::FreeType2>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// loadFontData(String, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/freetype.hpp:85
	// ("cv::freetype::FreeType2::loadFontData", vec![(pred!(mut, ["fontFileName", "idx"], ["cv::String", "int"]), _)]),
	void cv_freetype_FreeType2_loadFontData_String_int(cv::freetype::FreeType2* instance, const char* fontFileName, int idx, ResultVoid* ocvrs_return) {
		try {
			instance->loadFontData(std::string(fontFileName), idx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// loadFontData(char *, size_t, int)(Indirect, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/freetype.hpp:98
	// ("cv::freetype::FreeType2::loadFontData", vec![(pred!(mut, ["pBuf", "bufSize", "idx"], ["char*", "size_t", "int"]), _)]),
	void cv_freetype_FreeType2_loadFontData_charX_size_t_int(cv::freetype::FreeType2* instance, char* pBuf, size_t bufSize, int idx, ResultVoid* ocvrs_return) {
		try {
			instance->loadFontData(pBuf, bufSize, idx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSplitNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/freetype.hpp:109
	// ("cv::freetype::FreeType2::setSplitNumber", vec![(pred!(mut, ["num"], ["int"]), _)]),
	void cv_freetype_FreeType2_setSplitNumber_int(cv::freetype::FreeType2* instance, int num, ResultVoid* ocvrs_return) {
		try {
			instance->setSplitNumber(num);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// putText(InputOutputArray, const String &, Point, int, Scalar, int, int, bool)(InputOutputArray, InString, SimpleClass, Primitive, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/freetype.hpp:125
	// ("cv::freetype::FreeType2::putText", vec![(pred!(mut, ["img", "text", "org", "fontHeight", "color", "thickness", "line_type", "bottomLeftOrigin"], ["const cv::_InputOutputArray*", "const cv::String*", "cv::Point", "int", "cv::Scalar", "int", "int", "bool"]), _)]),
	void cv_freetype_FreeType2_putText_const__InputOutputArrayR_const_StringR_Point_int_Scalar_int_int_bool(cv::freetype::FreeType2* instance, const cv::_InputOutputArray* img, const char* text, cv::Point* org, int fontHeight, cv::Scalar* color, int thickness, int line_type, bool bottomLeftOrigin, ResultVoid* ocvrs_return) {
		try {
			instance->putText(*img, std::string(text), *org, fontHeight, *color, thickness, line_type, bottomLeftOrigin);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTextSize(const String &, int, int, int *)(InString, Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/freetype.hpp:186
	// ("cv::freetype::FreeType2::getTextSize", vec![(pred!(mut, ["text", "fontHeight", "thickness", "baseLine"], ["const cv::String*", "int", "int", "int*"]), _)]),
	void cv_freetype_FreeType2_getTextSize_const_StringR_int_int_intX(cv::freetype::FreeType2* instance, const char* text, int fontHeight, int thickness, int* baseLine, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getTextSize(std::string(text), fontHeight, thickness, baseLine);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::freetype::FreeType2::to_Algorithm() generated
	// ("cv::freetype::FreeType2::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_freetype_FreeType2_to_Algorithm(cv::freetype::FreeType2* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::freetype::FreeType2::delete() generated
	// ("cv::freetype::FreeType2::delete", vec![(pred!(mut, [], []), _)]),
	void cv_freetype_FreeType2_delete(cv::freetype::FreeType2* instance) {
			delete instance;
	}

}
