#include "ocvrs_common.hpp"
#include <opencv2/highgui.hpp>
#include "highgui_types.hpp"

extern "C" {
	Result_void cv_addText_const_MatR_const_StringR_Point_const_QtFontR(const cv::Mat* img, const char* text, cv::Point* org, const cv::QtFont* font) {
		try {
			cv::addText(*img, cv::String(text), *org, *font);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_addText_const_MatR_const_StringR_Point_const_StringR_int_Scalar_int_int_int(const cv::Mat* img, const char* text, cv::Point* org, const char* nameFont, int pointSize, cv::Scalar* color, int weight, int style, int spacing) {
		try {
			cv::addText(*img, cv::String(text), *org, cv::String(nameFont), pointSize, *color, weight, style, spacing);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_createButton_const_StringR_ButtonCallback_voidX_int_bool(const char* bar_name, cv::ButtonCallback on_change, void* userdata, int type, bool initial_button_state) {
		try {
			int ret = cv::createButton(cv::String(bar_name), on_change, userdata, type, initial_button_state);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_createTrackbar_const_StringR_const_StringR_intX_int_TrackbarCallback_voidX(const char* trackbarname, const char* winname, int* value, int count, cv::TrackbarCallback onChange, void* userdata) {
		try {
			int ret = cv::createTrackbar(cv::String(trackbarname), cv::String(winname), value, count, onChange, userdata);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_destroyAllWindows() {
		try {
			cv::destroyAllWindows();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_destroyWindow_const_StringR(const char* winname) {
		try {
			cv::destroyWindow(cv::String(winname));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_displayOverlay_const_StringR_const_StringR_int(const char* winname, const char* text, int delayms) {
		try {
			cv::displayOverlay(cv::String(winname), cv::String(text), delayms);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_displayStatusBar_const_StringR_const_StringR_int(const char* winname, const char* text, int delayms) {
		try {
			cv::displayStatusBar(cv::String(winname), cv::String(text), delayms);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::QtFont*> cv_fontQt_const_StringR_int_Scalar_int_int_int(const char* nameFont, int pointSize, cv::Scalar* color, int weight, int style, int spacing) {
		try {
			cv::QtFont ret = cv::fontQt(cv::String(nameFont), pointSize, *color, weight, style, spacing);
			return Ok(new cv::QtFont(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::QtFont*>))
	}
	
	Result<int> cv_getMouseWheelDelta_int(int flags) {
		try {
			int ret = cv::getMouseWheelDelta(flags);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_getTrackbarPos_const_StringR_const_StringR(const char* trackbarname, const char* winname) {
		try {
			int ret = cv::getTrackbarPos(cv::String(trackbarname), cv::String(winname));
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<double> cv_getWindowProperty_const_StringR_int(const char* winname, int prop_id) {
		try {
			double ret = cv::getWindowProperty(cv::String(winname), prop_id);
			return Ok<double>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	Result_void cv_imshow_const_StringR_const__InputArrayR(const char* winname, const cv::_InputArray* mat) {
		try {
			cv::imshow(cv::String(winname), *mat);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_loadWindowParameters_const_StringR(const char* windowName) {
		try {
			cv::loadWindowParameters(cv::String(windowName));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_moveWindow_const_StringR_int_int(const char* winname, int x, int y) {
		try {
			cv::moveWindow(cv::String(winname), x, y);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_namedWindow_const_StringR_int(const char* winname, int flags) {
		try {
			cv::namedWindow(cv::String(winname), flags);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_resizeWindow_const_StringR_int_int(const char* winname, int width, int height) {
		try {
			cv::resizeWindow(cv::String(winname), width, height);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_saveWindowParameters_const_StringR(const char* windowName) {
		try {
			cv::saveWindowParameters(cv::String(windowName));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_setMouseCallback_const_StringR_MouseCallback_voidX(const char* winname, cv::MouseCallback onMouse, void* userdata) {
		try {
			cv::setMouseCallback(cv::String(winname), onMouse, userdata);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_setOpenGlContext_const_StringR(const char* winname) {
		try {
			cv::setOpenGlContext(cv::String(winname));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_setOpenGlDrawCallback_const_StringR_OpenGlDrawCallback_voidX(const char* winname, cv::OpenGlDrawCallback onOpenGlDraw, void* userdata) {
		try {
			cv::setOpenGlDrawCallback(cv::String(winname), onOpenGlDraw, userdata);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_setTrackbarMax_const_StringR_const_StringR_int(const char* trackbarname, const char* winname, int maxval) {
		try {
			cv::setTrackbarMax(cv::String(trackbarname), cv::String(winname), maxval);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_setTrackbarMin_const_StringR_const_StringR_int(const char* trackbarname, const char* winname, int minval) {
		try {
			cv::setTrackbarMin(cv::String(trackbarname), cv::String(winname), minval);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_setTrackbarPos_const_StringR_const_StringR_int(const char* trackbarname, const char* winname, int pos) {
		try {
			cv::setTrackbarPos(cv::String(trackbarname), cv::String(winname), pos);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_setWindowProperty_const_StringR_int_double(const char* winname, int prop_id, double prop_value) {
		try {
			cv::setWindowProperty(cv::String(winname), prop_id, prop_value);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_setWindowTitle_const_StringR_const_StringR(const char* winname, const char* title) {
		try {
			cv::setWindowTitle(cv::String(winname), cv::String(title));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_startLoop_int__X__int__charXX__int_charXX(int (*pt2Func)(int, char**), int argc, char** argv) {
		try {
			int ret = cv::startLoop(pt2Func, argc, argv);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_startWindowThread() {
		try {
			int ret = cv::startWindowThread();
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_stopLoop() {
		try {
			cv::stopLoop();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_updateWindow_const_StringR(const char* winname) {
		try {
			cv::updateWindow(cv::String(winname));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_waitKeyEx_int(int delay) {
		try {
			int ret = cv::waitKeyEx(delay);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<int> cv_waitKey_int(int delay) {
		try {
			int ret = cv::waitKey(delay);
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result<void*> cv_QtFont_getPropNameFont_const(const cv::QtFont* instance) {
		try {
			const char* ret = instance->nameFont;
			return Ok(ocvrs_create_string(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result<cv::Scalar> cv_QtFont_getPropColor_const(const cv::QtFont* instance) {
		try {
			cv::Scalar ret = instance->color;
			return Ok<cv::Scalar>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	Result_void cv_QtFont_setPropColor_Scalar(cv::QtFont* instance, cv::Scalar* val) {
		try {
			instance->color = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_QtFont_getPropFont_face_const(const cv::QtFont* instance) {
		try {
			int ret = instance->font_face;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_QtFont_setPropFont_face_int(cv::QtFont* instance, int val) {
		try {
			instance->font_face = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<const int*> cv_QtFont_getPropAscii_const(const cv::QtFont* instance) {
		try {
			const int* ret = instance->ascii;
			return Ok<const int*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const int*>))
	}
	
	Result<const int*> cv_QtFont_getPropGreek_const(const cv::QtFont* instance) {
		try {
			const int* ret = instance->greek;
			return Ok<const int*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const int*>))
	}
	
	Result<const int*> cv_QtFont_getPropCyrillic_const(const cv::QtFont* instance) {
		try {
			const int* ret = instance->cyrillic;
			return Ok<const int*>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const int*>))
	}
	
	Result<float> cv_QtFont_getPropHscale_const(const cv::QtFont* instance) {
		try {
			float ret = instance->hscale;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_QtFont_setPropHscale_float(cv::QtFont* instance, float val) {
		try {
			instance->hscale = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_QtFont_getPropVscale_const(const cv::QtFont* instance) {
		try {
			float ret = instance->vscale;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_QtFont_setPropVscale_float(cv::QtFont* instance, float val) {
		try {
			instance->vscale = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_QtFont_getPropShear_const(const cv::QtFont* instance) {
		try {
			float ret = instance->shear;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_QtFont_setPropShear_float(cv::QtFont* instance, float val) {
		try {
			instance->shear = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_QtFont_getPropThickness_const(const cv::QtFont* instance) {
		try {
			int ret = instance->thickness;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_QtFont_setPropThickness_int(cv::QtFont* instance, int val) {
		try {
			instance->thickness = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<float> cv_QtFont_getPropDx_const(const cv::QtFont* instance) {
		try {
			float ret = instance->dx;
			return Ok<float>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	Result_void cv_QtFont_setPropDx_float(cv::QtFont* instance, float val) {
		try {
			instance->dx = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<int> cv_QtFont_getPropLine_type_const(const cv::QtFont* instance) {
		try {
			int ret = instance->line_type;
			return Ok<int>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	Result_void cv_QtFont_setPropLine_type_int(cv::QtFont* instance, int val) {
		try {
			instance->line_type = val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_QtFont_delete(cv::QtFont* instance) {
		delete instance;
	}
}
