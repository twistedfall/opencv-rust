#include "ocvrs_common.hpp"
#include <opencv2/highgui.hpp>
#include "highgui_types.hpp"

extern "C" {
	// addText(const Mat &, const String &, Point, const QtFont &)(TraitClass, InString, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:748
	// ("cv::addText", vec![(pred!(mut, ["img", "text", "org", "font"], ["const cv::Mat*", "const cv::String*", "cv::Point", "const cv::QtFont*"]), _)]),
	void cv_addText_const_MatR_const_StringR_Point_const_QtFontR(const cv::Mat* img, const char* text, cv::Point* org, const cv::QtFont* font, ResultVoid* ocvrs_return) {
		try {
			cv::addText(*img, cv::String(text), *org, *font);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::addText(TraitClass, InString, SimpleClass, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:764
	// ("cv::addText", vec![(pred!(mut, ["img", "text", "org", "nameFont"], ["const cv::Mat*", "const cv::String*", "cv::Point", "const cv::String*"]), _)]),
	void cv_addText_const_MatR_const_StringR_Point_const_StringR(const cv::Mat* img, const char* text, cv::Point* org, const char* nameFont, ResultVoid* ocvrs_return) {
		try {
			cv::addText(*img, cv::String(text), *org, cv::String(nameFont));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// addText(const Mat &, const String &, Point, const String &, int, Scalar, int, int, int)(TraitClass, InString, SimpleClass, InString, Primitive, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:764
	// ("cv::addText", vec![(pred!(mut, ["img", "text", "org", "nameFont", "pointSize", "color", "weight", "style", "spacing"], ["const cv::Mat*", "const cv::String*", "cv::Point", "const cv::String*", "int", "cv::Scalar", "int", "int", "int"]), _)]),
	void cv_addText_const_MatR_const_StringR_Point_const_StringR_int_Scalar_int_int_int(const cv::Mat* img, const char* text, cv::Point* org, const char* nameFont, int pointSize, cv::Scalar* color, int weight, int style, int spacing, ResultVoid* ocvrs_return) {
		try {
			cv::addText(*img, cv::String(text), *org, cv::String(nameFont), pointSize, *color, weight, style, spacing);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::createButton(InString, Function, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:843
	// ("cv::createButton", vec![(pred!(mut, ["bar_name", "on_change", "userdata"], ["const cv::String*", "cv::ButtonCallback", "void*"]), _)]),
	void cv_createButton_const_StringR_ButtonCallback_voidX(const char* bar_name, cv::ButtonCallback on_change, void* userdata, Result<int>* ocvrs_return) {
		try {
			int ret = cv::createButton(cv::String(bar_name), on_change, userdata);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createButton(const String &, ButtonCallback, void *, int, bool)(InString, Function, Indirect, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:843
	// ("cv::createButton", vec![(pred!(mut, ["bar_name", "on_change", "userdata", "type", "initial_button_state"], ["const cv::String*", "cv::ButtonCallback", "void*", "int", "bool"]), _)]),
	void cv_createButton_const_StringR_ButtonCallback_voidX_int_bool(const char* bar_name, cv::ButtonCallback on_change, void* userdata, int type, bool initial_button_state, Result<int>* ocvrs_return) {
		try {
			int ret = cv::createButton(cv::String(bar_name), on_change, userdata, type, initial_button_state);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createTrackbar(const String &, const String &, int *, int, TrackbarCallback, void *)(InString, InString, Indirect, Primitive, Function, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:564
	// ("cv::createTrackbar", vec![(pred!(mut, ["trackbarname", "winname", "value", "count", "onChange", "userdata"], ["const cv::String*", "const cv::String*", "int*", "int", "cv::TrackbarCallback", "void*"]), _)]),
	void cv_createTrackbar_const_StringR_const_StringR_intX_int_TrackbarCallback_voidX(const char* trackbarname, const char* winname, int* value, int count, cv::TrackbarCallback onChange, void* userdata, Result<int>* ocvrs_return) {
		try {
			int ret = cv::createTrackbar(cv::String(trackbarname), cv::String(winname), value, count, onChange, userdata);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// destroyAllWindows()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:331
	// ("cv::destroyAllWindows", vec![(pred!(mut, [], []), _)]),
	void cv_destroyAllWindows(ResultVoid* ocvrs_return) {
		try {
			cv::destroyAllWindows();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// destroyWindow(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:325
	// ("cv::destroyWindow", vec![(pred!(mut, ["winname"], ["const cv::String*"]), _)]),
	void cv_destroyWindow_const_StringR(const char* winname, ResultVoid* ocvrs_return) {
		try {
			cv::destroyWindow(cv::String(winname));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::displayOverlay(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:779
	// ("cv::displayOverlay", vec![(pred!(mut, ["winname", "text"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_displayOverlay_const_StringR_const_StringR(const char* winname, const char* text, ResultVoid* ocvrs_return) {
		try {
			cv::displayOverlay(cv::String(winname), cv::String(text));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// displayOverlay(const String &, const String &, int)(InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:779
	// ("cv::displayOverlay", vec![(pred!(mut, ["winname", "text", "delayms"], ["const cv::String*", "const cv::String*", "int"]), _)]),
	void cv_displayOverlay_const_StringR_const_StringR_int(const char* winname, const char* text, int delayms, ResultVoid* ocvrs_return) {
		try {
			cv::displayOverlay(cv::String(winname), cv::String(text), delayms);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::displayStatusBar(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:793
	// ("cv::displayStatusBar", vec![(pred!(mut, ["winname", "text"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_displayStatusBar_const_StringR_const_StringR(const char* winname, const char* text, ResultVoid* ocvrs_return) {
		try {
			cv::displayStatusBar(cv::String(winname), cv::String(text));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// displayStatusBar(const String &, const String &, int)(InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:793
	// ("cv::displayStatusBar", vec![(pred!(mut, ["winname", "text", "delayms"], ["const cv::String*", "const cv::String*", "int"]), _)]),
	void cv_displayStatusBar_const_StringR_const_StringR_int(const char* winname, const char* text, int delayms, ResultVoid* ocvrs_return) {
		try {
			cv::displayStatusBar(cv::String(winname), cv::String(text), delayms);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::fontQt(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:734
	// ("cv::fontQt", vec![(pred!(mut, ["nameFont"], ["const cv::String*"]), _)]),
	void cv_fontQt_const_StringR(const char* nameFont, Result<cv::QtFont*>* ocvrs_return) {
		try {
			cv::QtFont ret = cv::fontQt(cv::String(nameFont));
			Ok(new cv::QtFont(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// fontQt(const String &, int, Scalar, int, int, int)(InString, Primitive, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:734
	// ("cv::fontQt", vec![(pred!(mut, ["nameFont", "pointSize", "color", "weight", "style", "spacing"], ["const cv::String*", "int", "cv::Scalar", "int", "int", "int"]), _)]),
	void cv_fontQt_const_StringR_int_Scalar_int_int_int(const char* nameFont, int pointSize, cv::Scalar* color, int weight, int style, int spacing, Result<cv::QtFont*>* ocvrs_return) {
		try {
			cv::QtFont ret = cv::fontQt(cv::String(nameFont), pointSize, *color, weight, style, spacing);
			Ok(new cv::QtFont(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMouseWheelDelta(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:496
	// ("cv::getMouseWheelDelta", vec![(pred!(mut, ["flags"], ["int"]), _)]),
	void cv_getMouseWheelDelta_int(int flags, Result<int>* ocvrs_return) {
		try {
			int ret = cv::getMouseWheelDelta(flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTrackbarPos(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:581
	// ("cv::getTrackbarPos", vec![(pred!(mut, ["trackbarname", "winname"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_getTrackbarPos_const_StringR_const_StringR(const char* trackbarname, const char* winname, Result<int>* ocvrs_return) {
		try {
			int ret = cv::getTrackbarPos(cv::String(trackbarname), cv::String(winname));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWindowImageRect(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:465
	// ("cv::getWindowImageRect", vec![(pred!(mut, ["winname"], ["const cv::String*"]), _)]),
	void cv_getWindowImageRect_const_StringR(const char* winname, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = cv::getWindowImageRect(cv::String(winname));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWindowProperty(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:455
	// ("cv::getWindowProperty", vec![(pred!(mut, ["winname", "prop_id"], ["const cv::String*", "int"]), _)]),
	void cv_getWindowProperty_const_StringR_int(const char* winname, int prop_id, Result<double>* ocvrs_return) {
		try {
			double ret = cv::getWindowProperty(cv::String(winname), prop_id);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// imshow(const String &, InputArray)(InString, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:401
	// ("cv::imshow", vec![(pred!(mut, ["winname", "mat"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
	void cv_imshow_const_StringR_const__InputArrayR(const char* winname, const cv::_InputArray* mat, ResultVoid* ocvrs_return) {
		try {
			cv::imshow(cv::String(winname), *mat);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// loadWindowParameters(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:811
	// ("cv::loadWindowParameters", vec![(pred!(mut, ["windowName"], ["const cv::String*"]), _)]),
	void cv_loadWindowParameters_const_StringR(const char* windowName, ResultVoid* ocvrs_return) {
		try {
			cv::loadWindowParameters(cv::String(windowName));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// moveWindow(const String &, int, int)(InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:428
	// ("cv::moveWindow", vec![(pred!(mut, ["winname", "x", "y"], ["const cv::String*", "int", "int"]), _)]),
	void cv_moveWindow_const_StringR_int_int(const char* winname, int x, int y, ResultVoid* ocvrs_return) {
		try {
			cv::moveWindow(cv::String(winname), x, y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::namedWindow(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:317
	// ("cv::namedWindow", vec![(pred!(mut, ["winname"], ["const cv::String*"]), _)]),
	void cv_namedWindow_const_StringR(const char* winname, ResultVoid* ocvrs_return) {
		try {
			cv::namedWindow(cv::String(winname));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// namedWindow(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:317
	// ("cv::namedWindow", vec![(pred!(mut, ["winname", "flags"], ["const cv::String*", "int"]), _)]),
	void cv_namedWindow_const_StringR_int(const char* winname, int flags, ResultVoid* ocvrs_return) {
		try {
			cv::namedWindow(cv::String(winname), flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resizeWindow(const String &, const cv::Size &)(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:420
	// ("cv::resizeWindow", vec![(pred!(mut, ["winname", "size"], ["const cv::String*", "const cv::Size*"]), _)]),
	void cv_resizeWindow_const_StringR_const_SizeR(const char* winname, const cv::Size* size, ResultVoid* ocvrs_return) {
		try {
			cv::resizeWindow(cv::String(winname), *size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// resizeWindow(const String &, int, int)(InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:414
	// ("cv::resizeWindow", vec![(pred!(mut, ["winname", "width", "height"], ["const cv::String*", "int", "int"]), _)]),
	void cv_resizeWindow_const_StringR_int_int(const char* winname, int width, int height, ResultVoid* ocvrs_return) {
		try {
			cv::resizeWindow(cv::String(winname), width, height);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// saveWindowParameters(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:802
	// ("cv::saveWindowParameters", vec![(pred!(mut, ["windowName"], ["const cv::String*"]), _)]),
	void cv_saveWindowParameters_const_StringR(const char* windowName, ResultVoid* ocvrs_return) {
		try {
			cv::saveWindowParameters(cv::String(windowName));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::selectROI(InString, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:513
	// ("cv::selectROI", vec![(pred!(mut, ["windowName", "img"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
	void cv_selectROI_const_StringR_const__InputArrayR(const char* windowName, const cv::_InputArray* img, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = cv::selectROI(cv::String(windowName), *img);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// selectROI(const String &, InputArray, bool, bool)(InString, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:513
	// ("cv::selectROI", vec![(pred!(mut, ["windowName", "img", "showCrosshair", "fromCenter"], ["const cv::String*", "const cv::_InputArray*", "bool", "bool"]), _)]),
	void cv_selectROI_const_StringR_const__InputArrayR_bool_bool(const char* windowName, const cv::_InputArray* img, bool showCrosshair, bool fromCenter, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = cv::selectROI(cv::String(windowName), *img, showCrosshair, fromCenter);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::selectROI(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:517
	// ("cv::selectROI", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
	void cv_selectROI_const__InputArrayR(const cv::_InputArray* img, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = cv::selectROI(*img);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// selectROI(InputArray, bool, bool)(InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:517
	// ("cv::selectROI", vec![(pred!(mut, ["img", "showCrosshair", "fromCenter"], ["const cv::_InputArray*", "bool", "bool"]), _)]),
	void cv_selectROI_const__InputArrayR_bool_bool(const cv::_InputArray* img, bool showCrosshair, bool fromCenter, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = cv::selectROI(*img, showCrosshair, fromCenter);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::selectROIs(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:535
	// ("cv::selectROIs", vec![(pred!(mut, ["windowName", "img", "boundingBoxes"], ["const cv::String*", "const cv::_InputArray*", "std::vector<cv::Rect>*"]), _)]),
	void cv_selectROIs_const_StringR_const__InputArrayR_vectorLRectGR(const char* windowName, const cv::_InputArray* img, std::vector<cv::Rect>* boundingBoxes, ResultVoid* ocvrs_return) {
		try {
			cv::selectROIs(cv::String(windowName), *img, *boundingBoxes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// selectROIs(const String &, InputArray, std::vector<Rect> &, bool, bool)(InString, InputArray, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:535
	// ("cv::selectROIs", vec![(pred!(mut, ["windowName", "img", "boundingBoxes", "showCrosshair", "fromCenter"], ["const cv::String*", "const cv::_InputArray*", "std::vector<cv::Rect>*", "bool", "bool"]), _)]),
	void cv_selectROIs_const_StringR_const__InputArrayR_vectorLRectGR_bool_bool(const char* windowName, const cv::_InputArray* img, std::vector<cv::Rect>* boundingBoxes, bool showCrosshair, bool fromCenter, ResultVoid* ocvrs_return) {
		try {
			cv::selectROIs(cv::String(windowName), *img, *boundingBoxes, showCrosshair, fromCenter);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMouseCallback(const String &, MouseCallback, void *)(InString, Function, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:476
	// ("cv::setMouseCallback", vec![(pred!(mut, ["winname", "onMouse", "userdata"], ["const cv::String*", "cv::MouseCallback", "void*"]), _)]),
	void cv_setMouseCallback_const_StringR_MouseCallback_voidX(const char* winname, cv::MouseCallback onMouse, void* userdata, ResultVoid* ocvrs_return) {
		try {
			cv::setMouseCallback(cv::String(winname), onMouse, userdata);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOpenGlContext(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:684
	// ("cv::setOpenGlContext", vec![(pred!(mut, ["winname"], ["const cv::String*"]), _)]),
	void cv_setOpenGlContext_const_StringR(const char* winname, ResultVoid* ocvrs_return) {
		try {
			cv::setOpenGlContext(cv::String(winname));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOpenGlDrawCallback(const String &, OpenGlDrawCallback, void *)(InString, Function, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:678
	// ("cv::setOpenGlDrawCallback", vec![(pred!(mut, ["winname", "onOpenGlDraw", "userdata"], ["const cv::String*", "cv::OpenGlDrawCallback", "void*"]), _)]),
	void cv_setOpenGlDrawCallback_const_StringR_OpenGlDrawCallback_voidX(const char* winname, cv::OpenGlDrawCallback onOpenGlDraw, void* userdata, ResultVoid* ocvrs_return) {
		try {
			cv::setOpenGlDrawCallback(cv::String(winname), onOpenGlDraw, userdata);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTrackbarMax(const String &, const String &, int)(InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:611
	// ("cv::setTrackbarMax", vec![(pred!(mut, ["trackbarname", "winname", "maxval"], ["const cv::String*", "const cv::String*", "int"]), _)]),
	void cv_setTrackbarMax_const_StringR_const_StringR_int(const char* trackbarname, const char* winname, int maxval, ResultVoid* ocvrs_return) {
		try {
			cv::setTrackbarMax(cv::String(trackbarname), cv::String(winname), maxval);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTrackbarMin(const String &, const String &, int)(InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:626
	// ("cv::setTrackbarMin", vec![(pred!(mut, ["trackbarname", "winname", "minval"], ["const cv::String*", "const cv::String*", "int"]), _)]),
	void cv_setTrackbarMin_const_StringR_const_StringR_int(const char* trackbarname, const char* winname, int minval, ResultVoid* ocvrs_return) {
		try {
			cv::setTrackbarMin(cv::String(trackbarname), cv::String(winname), minval);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTrackbarPos(const String &, const String &, int)(InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:596
	// ("cv::setTrackbarPos", vec![(pred!(mut, ["trackbarname", "winname", "pos"], ["const cv::String*", "const cv::String*", "int"]), _)]),
	void cv_setTrackbarPos_const_StringR_const_StringR_int(const char* trackbarname, const char* winname, int pos, ResultVoid* ocvrs_return) {
		try {
			cv::setTrackbarPos(cv::String(trackbarname), cv::String(winname), pos);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWindowProperty(const String &, int, double)(InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:438
	// ("cv::setWindowProperty", vec![(pred!(mut, ["winname", "prop_id", "prop_value"], ["const cv::String*", "int", "double"]), _)]),
	void cv_setWindowProperty_const_StringR_int_double(const char* winname, int prop_id, double prop_value, ResultVoid* ocvrs_return) {
		try {
			cv::setWindowProperty(cv::String(winname), prop_id, prop_value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWindowTitle(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:444
	// ("cv::setWindowTitle", vec![(pred!(mut, ["winname", "title"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_setWindowTitle_const_StringR_const_StringR(const char* winname, const char* title, ResultVoid* ocvrs_return) {
		try {
			cv::setWindowTitle(cv::String(winname), cv::String(title));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// startLoop(int (*)(int, char **), int, char **)(Function, Primitive, VariableArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:813
	// ("cv::startLoop", vec![(pred!(mut, ["pt2Func", "argc", "argv"], ["int (*)(int, char**)", "int", "char**"]), _)]),
	void cv_startLoop_int__X__int__charXX__int_charXX(int (*pt2Func)(int, char**), int argc, char** argv, Result<int>* ocvrs_return) {
		try {
			int ret = cv::startLoop(pt2Func, argc, argv);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// startWindowThread()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:333
	// ("cv::startWindowThread", vec![(pred!(mut, [], []), _)]),
	void cv_startWindowThread(Result<int>* ocvrs_return) {
		try {
			int ret = cv::startWindowThread();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// stopLoop()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:815
	// ("cv::stopLoop", vec![(pred!(mut, [], []), _)]),
	void cv_stopLoop(ResultVoid* ocvrs_return) {
		try {
			cv::stopLoop();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// updateWindow(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:690
	// ("cv::updateWindow", vec![(pred!(mut, ["winname"], ["const cv::String*"]), _)]),
	void cv_updateWindow_const_StringR(const char* winname, ResultVoid* ocvrs_return) {
		try {
			cv::updateWindow(cv::String(winname));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::waitKey() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:365
	// ("cv::waitKey", vec![(pred!(mut, [], []), _)]),
	void cv_waitKey(Result<int>* ocvrs_return) {
		try {
			int ret = cv::waitKey();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::waitKeyEx() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:342
	// ("cv::waitKeyEx", vec![(pred!(mut, [], []), _)]),
	void cv_waitKeyEx(Result<int>* ocvrs_return) {
		try {
			int ret = cv::waitKeyEx();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// waitKeyEx(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:342
	// ("cv::waitKeyEx", vec![(pred!(mut, ["delay"], ["int"]), _)]),
	void cv_waitKeyEx_int(int delay, Result<int>* ocvrs_return) {
		try {
			int ret = cv::waitKeyEx(delay);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// waitKey(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:365
	// ("cv::waitKey", vec![(pred!(mut, ["delay"], ["int"]), _)]),
	void cv_waitKey_int(int delay, Result<int>* ocvrs_return) {
		try {
			int ret = cv::waitKey(delay);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::QtFont::defaultNew() generated
	// ("cv::QtFont::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::QtFont* cv_QtFont_defaultNew_const() {
			cv::QtFont* ret = new cv::QtFont();
			return ret;
	}

	// cv::QtFont::nameFont() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:701
	// ("cv::QtFont::nameFont", vec![(pred!(const, [], []), _)]),
	void* cv_QtFont_propNameFont_const(const cv::QtFont* instance) {
			const char* ret = instance->nameFont;
			return ocvrs_create_string(ret);
	}

	// cv::QtFont::color() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:702
	// ("cv::QtFont::color", vec![(pred!(const, [], []), _)]),
	void cv_QtFont_propColor_const(const cv::QtFont* instance, cv::Scalar* ocvrs_return) {
			cv::Scalar ret = instance->color;
			*ocvrs_return = ret;
	}

	// cv::QtFont::setColor(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:702
	// ("cv::QtFont::setColor", vec![(pred!(mut, ["val"], ["const cv::Scalar"]), _)]),
	void cv_QtFont_propColor_const_Scalar(cv::QtFont* instance, const cv::Scalar* val) {
			instance->color = *val;
	}

	// cv::QtFont::font_face() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:703
	// ("cv::QtFont::font_face", vec![(pred!(const, [], []), _)]),
	int cv_QtFont_propFont_face_const(const cv::QtFont* instance) {
			int ret = instance->font_face;
			return ret;
	}

	// cv::QtFont::setFont_face(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:703
	// ("cv::QtFont::setFont_face", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_QtFont_propFont_face_const_int(cv::QtFont* instance, const int val) {
			instance->font_face = val;
	}

	// cv::QtFont::ascii() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:704
	// ("cv::QtFont::ascii", vec![(pred!(const, [], []), _)]),
	const int* cv_QtFont_propAscii_const(const cv::QtFont* instance) {
			const int* ret = instance->ascii;
			return ret;
	}

	// cv::QtFont::greek() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:705
	// ("cv::QtFont::greek", vec![(pred!(const, [], []), _)]),
	const int* cv_QtFont_propGreek_const(const cv::QtFont* instance) {
			const int* ret = instance->greek;
			return ret;
	}

	// cv::QtFont::cyrillic() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:706
	// ("cv::QtFont::cyrillic", vec![(pred!(const, [], []), _)]),
	const int* cv_QtFont_propCyrillic_const(const cv::QtFont* instance) {
			const int* ret = instance->cyrillic;
			return ret;
	}

	// cv::QtFont::hscale() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:707
	// ("cv::QtFont::hscale", vec![(pred!(const, [], []), _)]),
	float cv_QtFont_propHscale_const(const cv::QtFont* instance) {
			float ret = instance->hscale;
			return ret;
	}

	// cv::QtFont::setHscale(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:707
	// ("cv::QtFont::setHscale", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_QtFont_propHscale_const_float(cv::QtFont* instance, const float val) {
			instance->hscale = val;
	}

	// cv::QtFont::vscale() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:707
	// ("cv::QtFont::vscale", vec![(pred!(const, [], []), _)]),
	float cv_QtFont_propVscale_const(const cv::QtFont* instance) {
			float ret = instance->vscale;
			return ret;
	}

	// cv::QtFont::setVscale(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:707
	// ("cv::QtFont::setVscale", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_QtFont_propVscale_const_float(cv::QtFont* instance, const float val) {
			instance->vscale = val;
	}

	// cv::QtFont::shear() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:708
	// ("cv::QtFont::shear", vec![(pred!(const, [], []), _)]),
	float cv_QtFont_propShear_const(const cv::QtFont* instance) {
			float ret = instance->shear;
			return ret;
	}

	// cv::QtFont::setShear(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:708
	// ("cv::QtFont::setShear", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_QtFont_propShear_const_float(cv::QtFont* instance, const float val) {
			instance->shear = val;
	}

	// cv::QtFont::thickness() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:709
	// ("cv::QtFont::thickness", vec![(pred!(const, [], []), _)]),
	int cv_QtFont_propThickness_const(const cv::QtFont* instance) {
			int ret = instance->thickness;
			return ret;
	}

	// cv::QtFont::setThickness(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:709
	// ("cv::QtFont::setThickness", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_QtFont_propThickness_const_int(cv::QtFont* instance, const int val) {
			instance->thickness = val;
	}

	// cv::QtFont::dx() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:710
	// ("cv::QtFont::dx", vec![(pred!(const, [], []), _)]),
	float cv_QtFont_propDx_const(const cv::QtFont* instance) {
			float ret = instance->dx;
			return ret;
	}

	// cv::QtFont::setDx(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:710
	// ("cv::QtFont::setDx", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_QtFont_propDx_const_float(cv::QtFont* instance, const float val) {
			instance->dx = val;
	}

	// cv::QtFont::line_type() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:711
	// ("cv::QtFont::line_type", vec![(pred!(const, [], []), _)]),
	int cv_QtFont_propLine_type_const(const cv::QtFont* instance) {
			int ret = instance->line_type;
			return ret;
	}

	// cv::QtFont::setLine_type(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/highgui.hpp:711
	// ("cv::QtFont::setLine_type", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_QtFont_propLine_type_const_int(cv::QtFont* instance, const int val) {
			instance->line_type = val;
	}

	// cv::QtFont::delete() generated
	// ("cv::QtFont::delete", vec![(pred!(mut, [], []), _)]),
	void cv_QtFont_delete(cv::QtFont* instance) {
			delete instance;
	}

}
