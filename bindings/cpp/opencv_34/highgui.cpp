#include "common.hpp"
#include <opencv2/highgui.hpp>
#include "highgui_types.hpp"

extern "C" {
	Result_void cv_addText_const_MatX_const_StringX_Point_const_QtFontX(const cv::Mat* img, const char* text, const cv::Point* org, const cv::QtFont* font) {
		try {
			cv::addText(*img, cv::String(text), *org, *font);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_addText_const_MatX_const_StringX_Point_const_StringX_int_Scalar_int_int_int(const cv::Mat* img, const char* text, const cv::Point* org, const char* nameFont, int pointSize, const cv::Scalar* color, int weight, int style, int spacing) {
		try {
			cv::addText(*img, cv::String(text), *org, cv::String(nameFont), pointSize, *color, weight, style, spacing);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_createButton_const_StringX_ButtonCallback_voidX_int_bool(const char* bar_name, cv::ButtonCallback on_change, void* userdata, int type, bool initial_button_state) {
		try {
			int ret = cv::createButton(cv::String(bar_name), on_change, userdata, type, initial_button_state);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_createTrackbar_const_StringX_const_StringX_intX_int_TrackbarCallback_voidX(const char* trackbarname, const char* winname, int* value, int count, cv::TrackbarCallback onChange, void* userdata) {
		try {
			int ret = cv::createTrackbar(cv::String(trackbarname), cv::String(winname), value, count, onChange, userdata);
			return Ok(ret);
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
	
	Result<cv::QtFont*> cv_fontQt_const_StringX_int_Scalar_int_int_int(const char* nameFont, int pointSize, const cv::Scalar* color, int weight, int style, int spacing) {
		try {
			cv::QtFont ret = cv::fontQt(cv::String(nameFont), pointSize, *color, weight, style, spacing);
			return Ok(new cv::QtFont(ret));
		} OCVRS_CATCH(Result<cv::QtFont*>)
	}
	
	Result<int> cv_getMouseWheelDelta_int(int flags) {
		try {
			int ret = cv::getMouseWheelDelta(flags);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_getTrackbarPos_const_StringX_const_StringX(const char* trackbarname, const char* winname) {
		try {
			int ret = cv::getTrackbarPos(cv::String(trackbarname), cv::String(winname));
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<cv::Rect> cv_getWindowImageRect_const_StringX(const char* winname) {
		try {
			cv::Rect ret = cv::getWindowImageRect(cv::String(winname));
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<double> cv_getWindowProperty_const_StringX_int(const char* winname, int prop_id) {
		try {
			double ret = cv::getWindowProperty(cv::String(winname), prop_id);
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_imshow_const_StringX_const__InputArrayX(const char* winname, const cv::_InputArray* mat) {
		try {
			cv::imshow(cv::String(winname), *mat);
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
	
	Result_void cv_resizeWindow_const_StringX_const_SizeX(const char* winname, const cv::Size* size) {
		try {
			cv::resizeWindow(cv::String(winname), *size);
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
	
	Result<cv::Rect> cv_selectROI_const_StringX_const__InputArrayX_bool_bool(const char* windowName, const cv::_InputArray* img, bool showCrosshair, bool fromCenter) {
		try {
			cv::Rect ret = cv::selectROI(cv::String(windowName), *img, showCrosshair, fromCenter);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result<cv::Rect> cv_selectROI_const__InputArrayX_bool_bool(const cv::_InputArray* img, bool showCrosshair, bool fromCenter) {
		try {
			cv::Rect ret = cv::selectROI(*img, showCrosshair, fromCenter);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Rect>)
	}
	
	Result_void cv_selectROIs_const_StringX_const__InputArrayX_vector_Rect_X_bool_bool(const char* windowName, const cv::_InputArray* img, std::vector<cv::Rect>* boundingBoxes, bool showCrosshair, bool fromCenter) {
		try {
			cv::selectROIs(cv::String(windowName), *img, *boundingBoxes, showCrosshair, fromCenter);
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
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_startWindowThread() {
		try {
			int ret = cv::startWindowThread();
			return Ok(ret);
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
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_waitKey_int(int delay) {
		try {
			int ret = cv::waitKey(delay);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_QtFont_nameFont_const(const cv::QtFont* instance) {
		try {
			const char* ret = instance->nameFont;
			return Ok(ocvrs_create_string(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::Scalar> cv_QtFont_color_const(const cv::QtFont* instance) {
		try {
			cv::Scalar ret = instance->color;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Scalar>)
	}
	
	Result_void cv_QtFont_setColor_Scalar(cv::QtFont* instance, const cv::Scalar* val) {
		try {
			instance->color = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_QtFont_font_face_const(const cv::QtFont* instance) {
		try {
			int ret = instance->font_face;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_QtFont_setFont_face_int(cv::QtFont* instance, int val) {
		try {
			instance->font_face = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<const int*> cv_QtFont_ascii_const(const cv::QtFont* instance) {
		try {
			const int* ret = instance->ascii;
			return Ok(ret);
		} OCVRS_CATCH(Result<const int*>)
	}
	
	Result<const int*> cv_QtFont_greek_const(const cv::QtFont* instance) {
		try {
			const int* ret = instance->greek;
			return Ok(ret);
		} OCVRS_CATCH(Result<const int*>)
	}
	
	Result<const int*> cv_QtFont_cyrillic_const(const cv::QtFont* instance) {
		try {
			const int* ret = instance->cyrillic;
			return Ok(ret);
		} OCVRS_CATCH(Result<const int*>)
	}
	
	Result<float> cv_QtFont_hscale_const(const cv::QtFont* instance) {
		try {
			float ret = instance->hscale;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_QtFont_setHscale_float(cv::QtFont* instance, float val) {
		try {
			instance->hscale = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_QtFont_vscale_const(const cv::QtFont* instance) {
		try {
			float ret = instance->vscale;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_QtFont_setVscale_float(cv::QtFont* instance, float val) {
		try {
			instance->vscale = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_QtFont_shear_const(const cv::QtFont* instance) {
		try {
			float ret = instance->shear;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_QtFont_setShear_float(cv::QtFont* instance, float val) {
		try {
			instance->shear = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_QtFont_thickness_const(const cv::QtFont* instance) {
		try {
			int ret = instance->thickness;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_QtFont_setThickness_int(cv::QtFont* instance, int val) {
		try {
			instance->thickness = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_QtFont_dx_const(const cv::QtFont* instance) {
		try {
			float ret = instance->dx;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_QtFont_setDx_float(cv::QtFont* instance, float val) {
		try {
			instance->dx = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_QtFont_line_type_const(const cv::QtFont* instance) {
		try {
			int ret = instance->line_type;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_QtFont_setLine_type_int(cv::QtFont* instance, int val) {
		try {
			instance->line_type = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_QtFont_delete(cv::QtFont* instance) {
		delete instance;
	}
}
