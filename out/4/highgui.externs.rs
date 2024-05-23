// addText(const Mat &, const String &, Point, const QtFont &)(TraitClass, InString, SimpleClass, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:725
// ("cv::addText", vec![(pred!(mut, ["img", "text", "org", "font"], ["const cv::Mat*", "const cv::String*", "cv::Point", "const cv::QtFont*"]), _)]),
pub fn cv_addText_const_MatR_const_StringR_Point_const_QtFontR(img: *const c_void, text: *const c_char, org: *const core::Point, font: *const c_void, ocvrs_return: *mut Result<()>);
// cv::addText(TraitClass, InString, SimpleClass, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:741
// ("cv::addText", vec![(pred!(mut, ["img", "text", "org", "nameFont"], ["const cv::Mat*", "const cv::String*", "cv::Point", "const cv::String*"]), _)]),
pub fn cv_addText_const_MatR_const_StringR_Point_const_StringR(img: *const c_void, text: *const c_char, org: *const core::Point, name_font: *const c_char, ocvrs_return: *mut Result<()>);
// addText(const Mat &, const String &, Point, const String &, int, Scalar, int, int, int)(TraitClass, InString, SimpleClass, InString, Primitive, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:741
// ("cv::addText", vec![(pred!(mut, ["img", "text", "org", "nameFont", "pointSize", "color", "weight", "style", "spacing"], ["const cv::Mat*", "const cv::String*", "cv::Point", "const cv::String*", "int", "cv::Scalar", "int", "int", "int"]), _)]),
pub fn cv_addText_const_MatR_const_StringR_Point_const_StringR_int_Scalar_int_int_int(img: *const c_void, text: *const c_char, org: *const core::Point, name_font: *const c_char, point_size: i32, color: *const core::Scalar, weight: i32, style: i32, spacing: i32, ocvrs_return: *mut Result<()>);
// cv::createButton(InString, Function, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:820
// ("cv::createButton", vec![(pred!(mut, ["bar_name", "on_change", "userdata"], ["const cv::String*", "cv::ButtonCallback", "void*"]), _)]),
pub fn cv_createButton_const_StringR_ButtonCallback_voidX(bar_name: *const c_char, on_change: Option<unsafe extern "C" fn(i32, *mut c_void) -> ()>, userdata: *mut c_void, ocvrs_return: *mut Result<i32>);
// createButton(const String &, ButtonCallback, void *, int, bool)(InString, Function, Indirect, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:820
// ("cv::createButton", vec![(pred!(mut, ["bar_name", "on_change", "userdata", "type", "initial_button_state"], ["const cv::String*", "cv::ButtonCallback", "void*", "int", "bool"]), _)]),
pub fn cv_createButton_const_StringR_ButtonCallback_voidX_int_bool(bar_name: *const c_char, on_change: Option<unsafe extern "C" fn(i32, *mut c_void) -> ()>, userdata: *mut c_void, typ: i32, initial_button_state: bool, ocvrs_return: *mut Result<i32>);
// createTrackbar(const String &, const String &, int *, int, TrackbarCallback, void *)(InString, InString, Indirect, Primitive, Function, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:549
// ("cv::createTrackbar", vec![(pred!(mut, ["trackbarname", "winname", "value", "count", "onChange", "userdata"], ["const cv::String*", "const cv::String*", "int*", "int", "cv::TrackbarCallback", "void*"]), _)]),
pub fn cv_createTrackbar_const_StringR_const_StringR_intX_int_TrackbarCallback_voidX(trackbarname: *const c_char, winname: *const c_char, value: *mut i32, count: i32, on_change: Option<unsafe extern "C" fn(i32, *mut c_void) -> ()>, userdata: *mut c_void, ocvrs_return: *mut Result<i32>);
// currentUIFramework()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:294
// ("cv::currentUIFramework", vec![(pred!(mut, [], []), _)]),
pub fn cv_currentUIFramework(ocvrs_return: *mut Result<*mut c_void>);
// destroyAllWindows()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:286
// ("cv::destroyAllWindows", vec![(pred!(mut, [], []), _)]),
pub fn cv_destroyAllWindows(ocvrs_return: *mut Result<()>);
// destroyWindow(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:280
// ("cv::destroyWindow", vec![(pred!(mut, ["winname"], ["const cv::String*"]), _)]),
pub fn cv_destroyWindow_const_StringR(winname: *const c_char, ocvrs_return: *mut Result<()>);
// cv::displayOverlay(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:756
// ("cv::displayOverlay", vec![(pred!(mut, ["winname", "text"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_displayOverlay_const_StringR_const_StringR(winname: *const c_char, text: *const c_char, ocvrs_return: *mut Result<()>);
// displayOverlay(const String &, const String &, int)(InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:756
// ("cv::displayOverlay", vec![(pred!(mut, ["winname", "text", "delayms"], ["const cv::String*", "const cv::String*", "int"]), _)]),
pub fn cv_displayOverlay_const_StringR_const_StringR_int(winname: *const c_char, text: *const c_char, delayms: i32, ocvrs_return: *mut Result<()>);
// cv::displayStatusBar(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:770
// ("cv::displayStatusBar", vec![(pred!(mut, ["winname", "text"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_displayStatusBar_const_StringR_const_StringR(winname: *const c_char, text: *const c_char, ocvrs_return: *mut Result<()>);
// displayStatusBar(const String &, const String &, int)(InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:770
// ("cv::displayStatusBar", vec![(pred!(mut, ["winname", "text", "delayms"], ["const cv::String*", "const cv::String*", "int"]), _)]),
pub fn cv_displayStatusBar_const_StringR_const_StringR_int(winname: *const c_char, text: *const c_char, delayms: i32, ocvrs_return: *mut Result<()>);
// cv::fontQt(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:711
// ("cv::fontQt", vec![(pred!(mut, ["nameFont"], ["const cv::String*"]), _)]),
pub fn cv_fontQt_const_StringR(name_font: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// fontQt(const String &, int, Scalar, int, int, int)(InString, Primitive, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:711
// ("cv::fontQt", vec![(pred!(mut, ["nameFont", "pointSize", "color", "weight", "style", "spacing"], ["const cv::String*", "int", "cv::Scalar", "int", "int", "int"]), _)]),
pub fn cv_fontQt_const_StringR_int_Scalar_int_int_int(name_font: *const c_char, point_size: i32, color: *const core::Scalar, weight: i32, style: i32, spacing: i32, ocvrs_return: *mut Result<*mut c_void>);
// getMouseWheelDelta(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:477
// ("cv::getMouseWheelDelta", vec![(pred!(mut, ["flags"], ["int"]), _)]),
pub fn cv_getMouseWheelDelta_int(flags: i32, ocvrs_return: *mut Result<i32>);
// getTrackbarPos(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:564
// ("cv::getTrackbarPos", vec![(pred!(mut, ["trackbarname", "winname"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_getTrackbarPos_const_StringR_const_StringR(trackbarname: *const c_char, winname: *const c_char, ocvrs_return: *mut Result<i32>);
// getWindowImageRect(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:448
// ("cv::getWindowImageRect", vec![(pred!(mut, ["winname"], ["const cv::String*"]), _)]),
pub fn cv_getWindowImageRect_const_StringR(winname: *const c_char, ocvrs_return: *mut Result<core::Rect>);
// getWindowProperty(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:436
// ("cv::getWindowProperty", vec![(pred!(mut, ["winname", "prop_id"], ["const cv::String*", "int"]), _)]),
pub fn cv_getWindowProperty_const_StringR_int(winname: *const c_char, prop_id: i32, ocvrs_return: *mut Result<f64>);
// imshow(const String &, InputArray)(InString, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:378
// ("cv::imshow", vec![(pred!(mut, ["winname", "mat"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
pub fn cv_imshow_const_StringR_const__InputArrayR(winname: *const c_char, mat: *const c_void, ocvrs_return: *mut Result<()>);
// loadWindowParameters(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:788
// ("cv::loadWindowParameters", vec![(pred!(mut, ["windowName"], ["const cv::String*"]), _)]),
pub fn cv_loadWindowParameters_const_StringR(window_name: *const c_char, ocvrs_return: *mut Result<()>);
// moveWindow(const String &, int, int)(InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:405
// ("cv::moveWindow", vec![(pred!(mut, ["winname", "x", "y"], ["const cv::String*", "int", "int"]), _)]),
pub fn cv_moveWindow_const_StringR_int_int(winname: *const c_char, x: i32, y: i32, ocvrs_return: *mut Result<()>);
// cv::namedWindow(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:272
// ("cv::namedWindow", vec![(pred!(mut, ["winname"], ["const cv::String*"]), _)]),
pub fn cv_namedWindow_const_StringR(winname: *const c_char, ocvrs_return: *mut Result<()>);
// namedWindow(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:272
// ("cv::namedWindow", vec![(pred!(mut, ["winname", "flags"], ["const cv::String*", "int"]), _)]),
pub fn cv_namedWindow_const_StringR_int(winname: *const c_char, flags: i32, ocvrs_return: *mut Result<()>);
// pollKey()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:338
// ("cv::pollKey", vec![(pred!(mut, [], []), _)]),
pub fn cv_pollKey(ocvrs_return: *mut Result<i32>);
// resizeWindow(const String &, const cv::Size &)(InString, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:395
// ("cv::resizeWindow", vec![(pred!(mut, ["winname", "size"], ["const cv::String*", "const cv::Size*"]), _)]),
pub fn cv_resizeWindow_const_StringR_const_SizeR(winname: *const c_char, size: *const core::Size, ocvrs_return: *mut Result<()>);
// resizeWindow(const String &, int, int)(InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:389
// ("cv::resizeWindow", vec![(pred!(mut, ["winname", "width", "height"], ["const cv::String*", "int", "int"]), _)]),
pub fn cv_resizeWindow_const_StringR_int_int(winname: *const c_char, width: i32, height: i32, ocvrs_return: *mut Result<()>);
// saveWindowParameters(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:779
// ("cv::saveWindowParameters", vec![(pred!(mut, ["windowName"], ["const cv::String*"]), _)]),
pub fn cv_saveWindowParameters_const_StringR(window_name: *const c_char, ocvrs_return: *mut Result<()>);
// cv::selectROI(InString, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:495
// ("cv::selectROI", vec![(pred!(mut, ["windowName", "img"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
pub fn cv_selectROI_const_StringR_const__InputArrayR(window_name: *const c_char, img: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// selectROI(const String &, InputArray, bool, bool, bool)(InString, InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:495
// ("cv::selectROI", vec![(pred!(mut, ["windowName", "img", "showCrosshair", "fromCenter", "printNotice"], ["const cv::String*", "const cv::_InputArray*", "bool", "bool", "bool"]), _)]),
pub fn cv_selectROI_const_StringR_const__InputArrayR_bool_bool_bool(window_name: *const c_char, img: *const c_void, show_crosshair: bool, from_center: bool, print_notice: bool, ocvrs_return: *mut Result<core::Rect>);
// cv::selectROI(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:499
// ("cv::selectROI", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
pub fn cv_selectROI_const__InputArrayR(img: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// selectROI(InputArray, bool, bool, bool)(InputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:499
// ("cv::selectROI", vec![(pred!(mut, ["img", "showCrosshair", "fromCenter", "printNotice"], ["const cv::_InputArray*", "bool", "bool", "bool"]), _)]),
pub fn cv_selectROI_const__InputArrayR_bool_bool_bool(img: *const c_void, show_crosshair: bool, from_center: bool, print_notice: bool, ocvrs_return: *mut Result<core::Rect>);
// cv::selectROIs(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:518
// ("cv::selectROIs", vec![(pred!(mut, ["windowName", "img", "boundingBoxes"], ["const cv::String*", "const cv::_InputArray*", "std::vector<cv::Rect>*"]), _)]),
pub fn cv_selectROIs_const_StringR_const__InputArrayR_vectorLRectGR(window_name: *const c_char, img: *const c_void, bounding_boxes: *mut c_void, ocvrs_return: *mut Result<()>);
// selectROIs(const String &, InputArray, std::vector<Rect> &, bool, bool, bool)(InString, InputArray, CppPassByVoidPtr, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:518
// ("cv::selectROIs", vec![(pred!(mut, ["windowName", "img", "boundingBoxes", "showCrosshair", "fromCenter", "printNotice"], ["const cv::String*", "const cv::_InputArray*", "std::vector<cv::Rect>*", "bool", "bool", "bool"]), _)]),
pub fn cv_selectROIs_const_StringR_const__InputArrayR_vectorLRectGR_bool_bool_bool(window_name: *const c_char, img: *const c_void, bounding_boxes: *mut c_void, show_crosshair: bool, from_center: bool, print_notice: bool, ocvrs_return: *mut Result<()>);
// setMouseCallback(const String &, MouseCallback, void *)(InString, Function, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:459
// ("cv::setMouseCallback", vec![(pred!(mut, ["winname", "onMouse", "userdata"], ["const cv::String*", "cv::MouseCallback", "void*"]), _)]),
pub fn cv_setMouseCallback_const_StringR_MouseCallback_voidX(winname: *const c_char, on_mouse: Option<unsafe extern "C" fn(i32, i32, i32, i32, *mut c_void) -> ()>, userdata: *mut c_void, ocvrs_return: *mut Result<()>);
// setOpenGlContext(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:661
// ("cv::setOpenGlContext", vec![(pred!(mut, ["winname"], ["const cv::String*"]), _)]),
pub fn cv_setOpenGlContext_const_StringR(winname: *const c_char, ocvrs_return: *mut Result<()>);
// setOpenGlDrawCallback(const String &, OpenGlDrawCallback, void *)(InString, Function, Indirect) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:655
// ("cv::setOpenGlDrawCallback", vec![(pred!(mut, ["winname", "onOpenGlDraw", "userdata"], ["const cv::String*", "cv::OpenGlDrawCallback", "void*"]), _)]),
pub fn cv_setOpenGlDrawCallback_const_StringR_OpenGlDrawCallback_voidX(winname: *const c_char, on_opengl_draw: Option<unsafe extern "C" fn(*mut c_void) -> ()>, userdata: *mut c_void, ocvrs_return: *mut Result<()>);
// setTrackbarMax(const String &, const String &, int)(InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:590
// ("cv::setTrackbarMax", vec![(pred!(mut, ["trackbarname", "winname", "maxval"], ["const cv::String*", "const cv::String*", "int"]), _)]),
pub fn cv_setTrackbarMax_const_StringR_const_StringR_int(trackbarname: *const c_char, winname: *const c_char, maxval: i32, ocvrs_return: *mut Result<()>);
// setTrackbarMin(const String &, const String &, int)(InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:603
// ("cv::setTrackbarMin", vec![(pred!(mut, ["trackbarname", "winname", "minval"], ["const cv::String*", "const cv::String*", "int"]), _)]),
pub fn cv_setTrackbarMin_const_StringR_const_StringR_int(trackbarname: *const c_char, winname: *const c_char, minval: i32, ocvrs_return: *mut Result<()>);
// setTrackbarPos(const String &, const String &, int)(InString, InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:577
// ("cv::setTrackbarPos", vec![(pred!(mut, ["trackbarname", "winname", "pos"], ["const cv::String*", "const cv::String*", "int"]), _)]),
pub fn cv_setTrackbarPos_const_StringR_const_StringR_int(trackbarname: *const c_char, winname: *const c_char, pos: i32, ocvrs_return: *mut Result<()>);
// setWindowProperty(const String &, int, double)(InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:417
// ("cv::setWindowProperty", vec![(pred!(mut, ["winname", "prop_id", "prop_value"], ["const cv::String*", "int", "double"]), _)]),
pub fn cv_setWindowProperty_const_StringR_int_double(winname: *const c_char, prop_id: i32, prop_value: f64, ocvrs_return: *mut Result<()>);
// setWindowTitle(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:423
// ("cv::setWindowTitle", vec![(pred!(mut, ["winname", "title"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_setWindowTitle_const_StringR_const_StringR(winname: *const c_char, title: *const c_char, ocvrs_return: *mut Result<()>);
// startLoop(int (*)(int, char **), int, char **)(Function, Primitive, VariableArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:790
// ("cv::startLoop", vec![(pred!(mut, ["pt2Func", "argc", "argv"], ["int (*)(int, char**)", "int", "char**"]), _)]),
pub fn cv_startLoop_int__X__int__charXX__int_charXX(pt2_func: Option<unsafe extern "C" fn(i32, *mut *mut c_char) -> i32>, argc: i32, argv: *mut *mut c_char, ocvrs_return: *mut Result<i32>);
// startWindowThread()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:297
// ("cv::startWindowThread", vec![(pred!(mut, [], []), _)]),
pub fn cv_startWindowThread(ocvrs_return: *mut Result<i32>);
// stopLoop()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:792
// ("cv::stopLoop", vec![(pred!(mut, [], []), _)]),
pub fn cv_stopLoop(ocvrs_return: *mut Result<()>);
// updateWindow(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:667
// ("cv::updateWindow", vec![(pred!(mut, ["winname"], ["const cv::String*"]), _)]),
pub fn cv_updateWindow_const_StringR(winname: *const c_char, ocvrs_return: *mut Result<()>);
// cv::waitKey() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:324
// ("cv::waitKey", vec![(pred!(mut, [], []), _)]),
pub fn cv_waitKey(ocvrs_return: *mut Result<i32>);
// cv::waitKeyEx() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:304
// ("cv::waitKeyEx", vec![(pred!(mut, [], []), _)]),
pub fn cv_waitKeyEx(ocvrs_return: *mut Result<i32>);
// waitKeyEx(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:304
// ("cv::waitKeyEx", vec![(pred!(mut, ["delay"], ["int"]), _)]),
pub fn cv_waitKeyEx_int(delay: i32, ocvrs_return: *mut Result<i32>);
// waitKey(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:324
// ("cv::waitKey", vec![(pred!(mut, ["delay"], ["int"]), _)]),
pub fn cv_waitKey_int(delay: i32, ocvrs_return: *mut Result<i32>);
// cv::QtFont::defaultNew() generated
// ("cv::QtFont::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_QtFont_defaultNew_const() -> *mut c_void;
// cv::QtFont::nameFont() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:678
// ("cv::QtFont::nameFont", vec![(pred!(const, [], []), _)]),
pub fn cv_QtFont_propNameFont_const(instance: *const c_void) -> *mut c_void;
// cv::QtFont::color() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:679
// ("cv::QtFont::color", vec![(pred!(const, [], []), _)]),
pub fn cv_QtFont_propColor_const(instance: *const c_void, ocvrs_return: *mut core::Scalar);
// cv::QtFont::setColor(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:679
// ("cv::QtFont::setColor", vec![(pred!(mut, ["val"], ["const cv::Scalar"]), _)]),
pub fn cv_QtFont_propColor_const_Scalar(instance: *mut c_void, val: *const core::Scalar);
// cv::QtFont::font_face() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:680
// ("cv::QtFont::font_face", vec![(pred!(const, [], []), _)]),
pub fn cv_QtFont_propFont_face_const(instance: *const c_void) -> i32;
// cv::QtFont::setFont_face(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:680
// ("cv::QtFont::setFont_face", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_QtFont_propFont_face_const_int(instance: *mut c_void, val: i32);
// cv::QtFont::ascii() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:681
// ("cv::QtFont::ascii", vec![(pred!(const, [], []), _)]),
pub fn cv_QtFont_propAscii_const(instance: *const c_void) -> *const i32;
// cv::QtFont::greek() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:682
// ("cv::QtFont::greek", vec![(pred!(const, [], []), _)]),
pub fn cv_QtFont_propGreek_const(instance: *const c_void) -> *const i32;
// cv::QtFont::cyrillic() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:683
// ("cv::QtFont::cyrillic", vec![(pred!(const, [], []), _)]),
pub fn cv_QtFont_propCyrillic_const(instance: *const c_void) -> *const i32;
// cv::QtFont::hscale() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:684
// ("cv::QtFont::hscale", vec![(pred!(const, [], []), _)]),
pub fn cv_QtFont_propHscale_const(instance: *const c_void) -> f32;
// cv::QtFont::setHscale(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:684
// ("cv::QtFont::setHscale", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_QtFont_propHscale_const_float(instance: *mut c_void, val: f32);
// cv::QtFont::vscale() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:684
// ("cv::QtFont::vscale", vec![(pred!(const, [], []), _)]),
pub fn cv_QtFont_propVscale_const(instance: *const c_void) -> f32;
// cv::QtFont::setVscale(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:684
// ("cv::QtFont::setVscale", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_QtFont_propVscale_const_float(instance: *mut c_void, val: f32);
// cv::QtFont::shear() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:685
// ("cv::QtFont::shear", vec![(pred!(const, [], []), _)]),
pub fn cv_QtFont_propShear_const(instance: *const c_void) -> f32;
// cv::QtFont::setShear(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:685
// ("cv::QtFont::setShear", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_QtFont_propShear_const_float(instance: *mut c_void, val: f32);
// cv::QtFont::thickness() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:686
// ("cv::QtFont::thickness", vec![(pred!(const, [], []), _)]),
pub fn cv_QtFont_propThickness_const(instance: *const c_void) -> i32;
// cv::QtFont::setThickness(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:686
// ("cv::QtFont::setThickness", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_QtFont_propThickness_const_int(instance: *mut c_void, val: i32);
// cv::QtFont::dx() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:687
// ("cv::QtFont::dx", vec![(pred!(const, [], []), _)]),
pub fn cv_QtFont_propDx_const(instance: *const c_void) -> f32;
// cv::QtFont::setDx(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:687
// ("cv::QtFont::setDx", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_QtFont_propDx_const_float(instance: *mut c_void, val: f32);
// cv::QtFont::line_type() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:688
// ("cv::QtFont::line_type", vec![(pred!(const, [], []), _)]),
pub fn cv_QtFont_propLine_type_const(instance: *const c_void) -> i32;
// cv::QtFont::setLine_type(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/highgui.hpp:688
// ("cv::QtFont::setLine_type", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_QtFont_propLine_type_const_int(instance: *mut c_void, val: i32);
// cv::QtFont::delete() generated
// ("cv::QtFont::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_QtFont_delete(instance: *mut c_void);
