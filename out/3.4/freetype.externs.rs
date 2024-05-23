// createFreeType2()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/freetype.hpp:184
// ("cv::freetype::createFreeType2", vec![(pred!(mut, [], []), _)]),
pub fn cv_freetype_createFreeType2(ocvrs_return: *mut Result<*mut c_void>);
// loadFontData(String, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/freetype.hpp:85
// ("cv::freetype::FreeType2::loadFontData", vec![(pred!(mut, ["fontFileName", "id"], ["cv::String", "int"]), _)]),
pub fn cv_freetype_FreeType2_loadFontData_String_int(instance: *mut c_void, font_file_name: *const c_char, id: i32, ocvrs_return: *mut Result<()>);
// setSplitNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/freetype.hpp:96
// ("cv::freetype::FreeType2::setSplitNumber", vec![(pred!(mut, ["num"], ["int"]), _)]),
pub fn cv_freetype_FreeType2_setSplitNumber_int(instance: *mut c_void, num: i32, ocvrs_return: *mut Result<()>);
// putText(InputOutputArray, const String &, Point, int, Scalar, int, int, bool)(InputOutputArray, InString, SimpleClass, Primitive, SimpleClass, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/freetype.hpp:112
// ("cv::freetype::FreeType2::putText", vec![(pred!(mut, ["img", "text", "org", "fontHeight", "color", "thickness", "line_type", "bottomLeftOrigin"], ["const cv::_InputOutputArray*", "const cv::String*", "cv::Point", "int", "cv::Scalar", "int", "int", "bool"]), _)]),
pub fn cv_freetype_FreeType2_putText_const__InputOutputArrayR_const_StringR_Point_int_Scalar_int_int_bool(instance: *mut c_void, img: *const c_void, text: *const c_char, org: *const core::Point, font_height: i32, color: *const core::Scalar, thickness: i32, line_type: i32, bottom_left_origin: bool, ocvrs_return: *mut Result<()>);
// getTextSize(const String &, int, int, int *)(InString, Primitive, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/freetype.hpp:173
// ("cv::freetype::FreeType2::getTextSize", vec![(pred!(mut, ["text", "fontHeight", "thickness", "baseLine"], ["const cv::String*", "int", "int", "int*"]), _)]),
pub fn cv_freetype_FreeType2_getTextSize_const_StringR_int_int_intX(instance: *mut c_void, text: *const c_char, font_height: i32, thickness: i32, base_line: *mut i32, ocvrs_return: *mut Result<core::Size>);
// cv::freetype::FreeType2::to_Algorithm() generated
// ("cv::freetype::FreeType2::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_freetype_FreeType2_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::freetype::FreeType2::delete() generated
// ("cv::freetype::FreeType2::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_freetype_FreeType2_delete(instance: *mut c_void);
