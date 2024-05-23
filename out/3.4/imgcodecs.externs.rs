// imdecode(InputArray, int)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:277
// ("cv::imdecode", vec![(pred!(mut, ["buf", "flags"], ["const cv::_InputArray*", "int"]), _)]),
pub fn cv_imdecode_const__InputArrayR_int(buf: *const c_void, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
// imdecode(InputArray, int, Mat *)(InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:285
// ("cv::imdecode", vec![(pred!(mut, ["buf", "flags", "dst"], ["const cv::_InputArray*", "int", "cv::Mat*"]), _)]),
pub fn cv_imdecode_const__InputArrayR_int_MatX(buf: *const c_void, flags: i32, dst: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::imencode(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:297
// ("cv::imencode", vec![(pred!(mut, ["ext", "img", "buf"], ["const cv::String*", "const cv::_InputArray*", "std::vector<unsigned char>*"]), _)]),
pub fn cv_imencode_const_StringR_const__InputArrayR_vectorLunsigned_charGR(ext: *const c_char, img: *const c_void, buf: *mut c_void, ocvrs_return: *mut Result<bool>);
// imencode(const String &, InputArray, std::vector<uchar> &, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:297
// ("cv::imencode", vec![(pred!(mut, ["ext", "img", "buf", "params"], ["const cv::String*", "const cv::_InputArray*", "std::vector<unsigned char>*", "const std::vector<int>*"]), _)]),
pub fn cv_imencode_const_StringR_const__InputArrayR_vectorLunsigned_charGR_const_vectorLintGR(ext: *const c_char, img: *const c_void, buf: *mut c_void, params: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::imread(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:206
// ("cv::imread", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_imread_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// imread(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:206
// ("cv::imread", vec![(pred!(mut, ["filename", "flags"], ["const cv::String*", "int"]), _)]),
pub fn cv_imread_const_StringR_int(filename: *const c_char, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::imreadmulti(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:216
// ("cv::imreadmulti", vec![(pred!(mut, ["filename", "mats"], ["const cv::String*", "std::vector<cv::Mat>*"]), _)]),
pub fn cv_imreadmulti_const_StringR_vectorLMatGR(filename: *const c_char, mats: *mut c_void, ocvrs_return: *mut Result<bool>);
// imreadmulti(const String &, std::vector<Mat> &, int)(InString, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:216
// ("cv::imreadmulti", vec![(pred!(mut, ["filename", "mats", "flags"], ["const cv::String*", "std::vector<cv::Mat>*", "int"]), _)]),
pub fn cv_imreadmulti_const_StringR_vectorLMatGR_int(filename: *const c_char, mats: *mut c_void, flags: i32, ocvrs_return: *mut Result<bool>);
// cv::imwrite(InString, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:255
// ("cv::imwrite", vec![(pred!(mut, ["filename", "img"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
pub fn cv_imwrite_const_StringR_const__InputArrayR(filename: *const c_char, img: *const c_void, ocvrs_return: *mut Result<bool>);
// imwrite(const String &, InputArray, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:255
// ("cv::imwrite", vec![(pred!(mut, ["filename", "img", "params"], ["const cv::String*", "const cv::_InputArray*", "const std::vector<int>*"]), _)]),
pub fn cv_imwrite_const_StringR_const__InputArrayR_const_vectorLintGR(filename: *const c_char, img: *const c_void, params: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::imwritemulti(InString, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:260
// ("cv::imwritemulti", vec![(pred!(mut, ["filename", "img"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
pub fn cv_imwritemulti_const_StringR_const__InputArrayR(filename: *const c_char, img: *const c_void, ocvrs_return: *mut Result<bool>);
// imwritemulti(const String &, InputArrayOfArrays, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/imgcodecs.hpp:260
// ("cv::imwritemulti", vec![(pred!(mut, ["filename", "img", "params"], ["const cv::String*", "const cv::_InputArray*", "const std::vector<int>*"]), _)]),
pub fn cv_imwritemulti_const_StringR_const__InputArrayR_const_vectorLintGR(filename: *const c_char, img: *const c_void, params: *const c_void, ocvrs_return: *mut Result<bool>);
