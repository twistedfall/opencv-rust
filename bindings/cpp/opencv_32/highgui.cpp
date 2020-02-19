#include "common.hpp"
#include <opencv2/highgui.hpp>
#include "highgui_types.hpp"

extern "C" {
	Result_void cv_addText_const_MatX_const_StringX_Point_const_QtFontX(void* img, const char* text, cv::Point org, void* font) {
		try {
			cv::addText(*reinterpret_cast<const cv::Mat*>(img), cv::String(text), org, *reinterpret_cast<const cv::QtFont*>(font));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_addText_const_MatX_const_StringX_Point_const_StringX_int_Scalar_int_int_int(void* img, const char* text, cv::Point org, const char* nameFont, int pointSize, cv::Scalar color, int weight, int style, int spacing) {
		try {
			cv::addText(*reinterpret_cast<const cv::Mat*>(img), cv::String(text), org, cv::String(nameFont), pointSize, color, weight, style, spacing);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_createButton_const_StringX_ButtonCallback_voidX_int_bool(const char* bar_name, cv::ButtonCallback on_change, void* userdata, int type, bool initial_button_state) {
		try {
			int ret = cv::createButton(cv::String(bar_name), on_change, userdata, type, initial_button_state);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_createTrackbar_const_StringX_const_StringX_intX_int_TrackbarCallback_voidX(const char* trackbarname, const char* winname, int* value, int count, cv::TrackbarCallback onChange, void* userdata) {
		try {
			int ret = cv::createTrackbar(cv::String(trackbarname), cv::String(winname), value, count, onChange, userdata);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_destroyAllWindows() {
		try {
			cv::destroyAllWindows();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_destroyWindow_const_StringX(const char* winname) {
		try {
			cv::destroyWindow(cv::String(winname));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_displayOverlay_const_StringX_const_StringX_int(const char* winname, const char* text, int delayms) {
		try {
			cv::displayOverlay(cv::String(winname), cv::String(text), delayms);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_displayStatusBar_const_StringX_const_StringX_int(const char* winname, const char* text, int delayms) {
		try {
			cv::displayStatusBar(cv::String(winname), cv::String(text), delayms);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_fontQt_const_StringX_int_Scalar_int_int_int(const char* nameFont, int pointSize, cv::Scalar color, int weight, int style, int spacing) {
		try {
			cv::QtFont ret = cv::fontQt(cv::String(nameFont), pointSize, color, weight, style, spacing);
			return Ok<void*>(new cv::QtFont(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_getMouseWheelDelta_int(int flags) {
		try {
			int ret = cv::getMouseWheelDelta(flags);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_getTrackbarPos_const_StringX_const_StringX(const char* trackbarname, const char* winname) {
		try {
			int ret = cv::getTrackbarPos(cv::String(trackbarname), cv::String(winname));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<double> cv_getWindowProperty_const_StringX_int(const char* winname, int prop_id) {
		try {
			double ret = cv::getWindowProperty(cv::String(winname), prop_id);
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_imshow_const_StringX_const__InputArrayX(const char* winname, void* mat) {
		try {
			cv::imshow(cv::String(winname), *reinterpret_cast<const cv::_InputArray*>(mat));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_loadWindowParameters_const_StringX(const char* windowName) {
		try {
			cv::loadWindowParameters(cv::String(windowName));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_moveWindow_const_StringX_int_int(const char* winname, int x, int y) {
		try {
			cv::moveWindow(cv::String(winname), x, y);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_namedWindow_const_StringX_int(const char* winname, int flags) {
		try {
			cv::namedWindow(cv::String(winname), flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_resizeWindow_const_StringX_int_int(const char* winname, int width, int height) {
		try {
			cv::resizeWindow(cv::String(winname), width, height);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_saveWindowParameters_const_StringX(const char* windowName) {
		try {
			cv::saveWindowParameters(cv::String(windowName));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_setMouseCallback_const_StringX_MouseCallback_voidX(const char* winname, cv::MouseCallback onMouse, void* userdata) {
		try {
			cv::setMouseCallback(cv::String(winname), onMouse, userdata);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_setOpenGlContext_const_StringX(const char* winname) {
		try {
			cv::setOpenGlContext(cv::String(winname));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_setOpenGlDrawCallback_const_StringX_OpenGlDrawCallback_voidX(const char* winname, cv::OpenGlDrawCallback onOpenGlDraw, void* userdata) {
		try {
			cv::setOpenGlDrawCallback(cv::String(winname), onOpenGlDraw, userdata);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_setTrackbarMax_const_StringX_const_StringX_int(const char* trackbarname, const char* winname, int maxval) {
		try {
			cv::setTrackbarMax(cv::String(trackbarname), cv::String(winname), maxval);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_setTrackbarMin_const_StringX_const_StringX_int(const char* trackbarname, const char* winname, int minval) {
		try {
			cv::setTrackbarMin(cv::String(trackbarname), cv::String(winname), minval);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_setTrackbarPos_const_StringX_const_StringX_int(const char* trackbarname, const char* winname, int pos) {
		try {
			cv::setTrackbarPos(cv::String(trackbarname), cv::String(winname), pos);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_setWindowProperty_const_StringX_int_double(const char* winname, int prop_id, double prop_value) {
		try {
			cv::setWindowProperty(cv::String(winname), prop_id, prop_value);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_setWindowTitle_const_StringX_const_StringX(const char* winname, const char* title) {
		try {
			cv::setWindowTitle(cv::String(winname), cv::String(title));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_startLoop_int__X__int__charXX__int_charXX(int (*pt2Func)(int, char**), int argc, char** argv) {
		try {
			int ret = cv::startLoop(pt2Func, argc, argv);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_startWindowThread() {
		try {
			int ret = cv::startWindowThread();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_stopLoop() {
		try {
			cv::stopLoop();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_updateWindow_const_StringX(const char* winname) {
		try {
			cv::updateWindow(cv::String(winname));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_waitKeyEx_int(int delay) {
		try {
			int ret = cv::waitKeyEx(delay);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_waitKey_int(int delay) {
		try {
			int ret = cv::waitKey(delay);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_QtFont_nameFont_const(void* instance) {
		try {
			const char* ret = reinterpret_cast<cv::QtFont*>(instance)->nameFont;
			return Ok<void*>(ocvrs_create_string(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Scalar> cv_QtFont_color_const(void* instance) {
		try {
			cv::Scalar ret = reinterpret_cast<cv::QtFont*>(instance)->color;
			return Ok<cv::Scalar>(ret);
		} OCVRS_CATCH(Result<cv::Scalar>)
	}
	
	Result_void cv_QtFont_setColor_Scalar(void* instance, cv::Scalar val) {
		try {
			reinterpret_cast<cv::QtFont*>(instance)->color = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_QtFont_font_face_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::QtFont*>(instance)->font_face;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_QtFont_setFont_face_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::QtFont*>(instance)->font_face = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<const int*> cv_QtFont_ascii_const(void* instance) {
		try {
			const int* ret = reinterpret_cast<cv::QtFont*>(instance)->ascii;
			return Ok<const int*>(ret);
		} OCVRS_CATCH(Result<const int*>)
	}
	
	Result<const int*> cv_QtFont_greek_const(void* instance) {
		try {
			const int* ret = reinterpret_cast<cv::QtFont*>(instance)->greek;
			return Ok<const int*>(ret);
		} OCVRS_CATCH(Result<const int*>)
	}
	
	Result<const int*> cv_QtFont_cyrillic_const(void* instance) {
		try {
			const int* ret = reinterpret_cast<cv::QtFont*>(instance)->cyrillic;
			return Ok<const int*>(ret);
		} OCVRS_CATCH(Result<const int*>)
	}
	
	Result<float> cv_QtFont_hscale_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::QtFont*>(instance)->hscale;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_QtFont_setHscale_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::QtFont*>(instance)->hscale = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_QtFont_vscale_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::QtFont*>(instance)->vscale;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_QtFont_setVscale_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::QtFont*>(instance)->vscale = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_QtFont_shear_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::QtFont*>(instance)->shear;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_QtFont_setShear_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::QtFont*>(instance)->shear = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_QtFont_thickness_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::QtFont*>(instance)->thickness;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_QtFont_setThickness_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::QtFont*>(instance)->thickness = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_QtFont_dx_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::QtFont*>(instance)->dx;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_QtFont_setDx_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::QtFont*>(instance)->dx = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_QtFont_line_type_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::QtFont*>(instance)->line_type;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_QtFont_setLine_type_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::QtFont*>(instance)->line_type = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_QtFont_delete(cv::QtFont* instance) {
		delete instance;
	}
}
