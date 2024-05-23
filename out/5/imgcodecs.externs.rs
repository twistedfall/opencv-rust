// haveImageReader(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:447
// ("cv::haveImageReader", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_haveImageReader_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<bool>);
// haveImageWriter(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:463
// ("cv::haveImageWriter", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_haveImageWriter_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<bool>);
// cv::imcount(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:315
// ("cv::imcount", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_imcount_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<size_t>);
// imcount(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:315
// ("cv::imcount", vec![(pred!(mut, ["filename", "flags"], ["const cv::String*", "int"]), _)]),
pub fn cv_imcount_const_StringR_int(filename: *const c_char, flags: i32, ocvrs_return: *mut Result<size_t>);
// imdecode(InputArray, int)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:379
// ("cv::imdecode", vec![(pred!(mut, ["buf", "flags"], ["const cv::_InputArray*", "int"]), _)]),
pub fn cv_imdecode_const__InputArrayR_int(buf: *const c_void, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
// imdecode(InputArray, int, Mat *)(InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:388
// ("cv::imdecode", vec![(pred!(mut, ["buf", "flags", "dst"], ["const cv::_InputArray*", "int", "cv::Mat*"]), _)]),
pub fn cv_imdecode_const__InputArrayR_int_MatX(buf: *const c_void, flags: i32, dst: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::imdecodemulti(InputArray, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:403
// ("cv::imdecodemulti", vec![(pred!(mut, ["buf", "flags", "mats"], ["const cv::_InputArray*", "int", "std::vector<cv::Mat>*"]), _)]),
pub fn cv_imdecodemulti_const__InputArrayR_int_vectorLMatGR(buf: *const c_void, flags: i32, mats: *mut c_void, ocvrs_return: *mut Result<bool>);
// imdecodemulti(InputArray, int, std::vector<Mat> &, const cv::Range &)(InputArray, Primitive, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:403
// ("cv::imdecodemulti", vec![(pred!(mut, ["buf", "flags", "mats", "range"], ["const cv::_InputArray*", "int", "std::vector<cv::Mat>*", "const cv::Range*"]), _)]),
pub fn cv_imdecodemulti_const__InputArrayR_int_vectorLMatGR_const_RangeR(buf: *const c_void, flags: i32, mats: *mut c_void, range: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::imencode(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:415
// ("cv::imencode", vec![(pred!(mut, ["ext", "img", "buf"], ["const cv::String*", "const cv::_InputArray*", "std::vector<unsigned char>*"]), _)]),
pub fn cv_imencode_const_StringR_const__InputArrayR_vectorLunsigned_charGR(ext: *const c_char, img: *const c_void, buf: *mut c_void, ocvrs_return: *mut Result<bool>);
// imencode(const String &, InputArray, std::vector<uchar> &, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:415
// ("cv::imencode", vec![(pred!(mut, ["ext", "img", "buf", "params"], ["const cv::String*", "const cv::_InputArray*", "std::vector<unsigned char>*", "const std::vector<int>*"]), _)]),
pub fn cv_imencode_const_StringR_const__InputArrayR_vectorLunsigned_charGR_const_vectorLintGR(ext: *const c_char, img: *const c_void, buf: *mut c_void, params: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::imencodemulti(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:429
// ("cv::imencodemulti", vec![(pred!(mut, ["ext", "imgs", "buf"], ["const cv::String*", "const cv::_InputArray*", "std::vector<unsigned char>*"]), _)]),
pub fn cv_imencodemulti_const_StringR_const__InputArrayR_vectorLunsigned_charGR(ext: *const c_char, imgs: *const c_void, buf: *mut c_void, ocvrs_return: *mut Result<bool>);
// imencodemulti(const String &, InputArrayOfArrays, std::vector<uchar> &, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:429
// ("cv::imencodemulti", vec![(pred!(mut, ["ext", "imgs", "buf", "params"], ["const cv::String*", "const cv::_InputArray*", "std::vector<unsigned char>*", "const std::vector<int>*"]), _)]),
pub fn cv_imencodemulti_const_StringR_const__InputArrayR_vectorLunsigned_charGR_const_vectorLintGR(ext: *const c_char, imgs: *const c_void, buf: *mut c_void, params: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::imread(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:272
// ("cv::imread", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_imread_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::imread(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:283
// ("cv::imread", vec![(pred!(mut, ["filename", "dst"], ["const cv::String*", "const cv::_OutputArray*"]), _)]),
pub fn cv_imread_const_StringR_const__OutputArrayR(filename: *const c_char, dst: *const c_void, ocvrs_return: *mut Result<()>);
// imread(const String &, OutputArray, int)(InString, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:283
// ("cv::imread", vec![(pred!(mut, ["filename", "dst", "flags"], ["const cv::String*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_imread_const_StringR_const__OutputArrayR_int(filename: *const c_char, dst: *const c_void, flags: i32, ocvrs_return: *mut Result<()>);
// imread(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:272
// ("cv::imread", vec![(pred!(mut, ["filename", "flags"], ["const cv::String*", "int"]), _)]),
pub fn cv_imread_const_StringR_int(filename: *const c_char, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::imreadmulti(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:293
// ("cv::imreadmulti", vec![(pred!(mut, ["filename", "mats"], ["const cv::String*", "std::vector<cv::Mat>*"]), _)]),
pub fn cv_imreadmulti_const_StringR_vectorLMatGR(filename: *const c_char, mats: *mut c_void, ocvrs_return: *mut Result<bool>);
// imreadmulti(const String &, std::vector<Mat> &, int)(InString, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:293
// ("cv::imreadmulti", vec![(pred!(mut, ["filename", "mats", "flags"], ["const cv::String*", "std::vector<cv::Mat>*", "int"]), _)]),
pub fn cv_imreadmulti_const_StringR_vectorLMatGR_int(filename: *const c_char, mats: *mut c_void, flags: i32, ocvrs_return: *mut Result<bool>);
// cv::imreadmulti(InString, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:305
// ("cv::imreadmulti", vec![(pred!(mut, ["filename", "mats", "start", "count"], ["const cv::String*", "std::vector<cv::Mat>*", "int", "int"]), _)]),
pub fn cv_imreadmulti_const_StringR_vectorLMatGR_int_int(filename: *const c_char, mats: *mut c_void, start: i32, count: i32, ocvrs_return: *mut Result<bool>);
// imreadmulti(const String &, std::vector<Mat> &, int, int, int)(InString, CppPassByVoidPtr, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:305
// ("cv::imreadmulti", vec![(pred!(mut, ["filename", "mats", "start", "count", "flags"], ["const cv::String*", "std::vector<cv::Mat>*", "int", "int", "int"]), _)]),
pub fn cv_imreadmulti_const_StringR_vectorLMatGR_int_int_int(filename: *const c_char, mats: *mut c_void, start: i32, count: i32, flags: i32, ocvrs_return: *mut Result<bool>);
// cv::imwrite(InString, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:357
// ("cv::imwrite", vec![(pred!(mut, ["filename", "img"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
pub fn cv_imwrite_const_StringR_const__InputArrayR(filename: *const c_char, img: *const c_void, ocvrs_return: *mut Result<bool>);
// imwrite(const String &, InputArray, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:357
// ("cv::imwrite", vec![(pred!(mut, ["filename", "img", "params"], ["const cv::String*", "const cv::_InputArray*", "const std::vector<int>*"]), _)]),
pub fn cv_imwrite_const_StringR_const__InputArrayR_const_vectorLintGR(filename: *const c_char, img: *const c_void, params: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::imwritemulti(InString, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:362
// ("cv::imwritemulti", vec![(pred!(mut, ["filename", "img"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
pub fn cv_imwritemulti_const_StringR_const__InputArrayR(filename: *const c_char, img: *const c_void, ocvrs_return: *mut Result<bool>);
// imwritemulti(const String &, InputArrayOfArrays, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:362
// ("cv::imwritemulti", vec![(pred!(mut, ["filename", "img", "params"], ["const cv::String*", "const cv::_InputArray*", "const std::vector<int>*"]), _)]),
pub fn cv_imwritemulti_const_StringR_const__InputArrayR_const_vectorLintGR(filename: *const c_char, img: *const c_void, params: *const c_void, ocvrs_return: *mut Result<bool>);
// ImageCollection()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:495
// ("cv::ImageCollection::ImageCollection", vec![(pred!(mut, [], []), _)]),
pub fn cv_ImageCollection_ImageCollection(ocvrs_return: *mut Result<*mut c_void>);
// ImageCollection(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:496
// ("cv::ImageCollection::ImageCollection", vec![(pred!(mut, ["filename", "flags"], ["const cv::String*", "int"]), _)]),
pub fn cv_ImageCollection_ImageCollection_const_StringR_int(filename: *const c_char, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
// init(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:497
// ("cv::ImageCollection::init", vec![(pred!(mut, ["img", "flags"], ["const cv::String*", "int"]), _)]),
pub fn cv_ImageCollection_init_const_StringR_int(instance: *mut c_void, img: *const c_char, flags: i32, ocvrs_return: *mut Result<()>);
// size()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:498
// ("cv::ImageCollection::size", vec![(pred!(const, [], []), _)]),
pub fn cv_ImageCollection_size_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// at(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:499
// ("cv::ImageCollection::at", vec![(pred!(mut, ["index"], ["int"]), _)]),
pub fn cv_ImageCollection_at_int(instance: *mut c_void, index: i32, ocvrs_return: *mut Result<*mut c_void>);
// operator[](int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:500
// ("cv::ImageCollection::operator[]", vec![(pred!(mut, ["index"], ["int"]), _)]),
pub fn cv_ImageCollection_operator___int(instance: *mut c_void, index: i32, ocvrs_return: *mut Result<*mut c_void>);
// releaseCache(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:501
// ("cv::ImageCollection::releaseCache", vec![(pred!(mut, ["index"], ["int"]), _)]),
pub fn cv_ImageCollection_releaseCache_int(instance: *mut c_void, index: i32, ocvrs_return: *mut Result<()>);
// begin()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:502
// ("cv::ImageCollection::begin", vec![(pred!(mut, [], []), _)]),
pub fn cv_ImageCollection_begin(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// end()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:503
// ("cv::ImageCollection::end", vec![(pred!(mut, [], []), _)]),
pub fn cv_ImageCollection_end(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::ImageCollection::delete() generated
// ("cv::ImageCollection::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ImageCollection_delete(instance: *mut c_void);
// iterator(ImageCollection *)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:481
// ("cv::ImageCollection::iterator::iterator", vec![(pred!(mut, ["col"], ["cv::ImageCollection*"]), _)]),
pub fn cv_ImageCollection_iterator_iterator_ImageCollectionX(col: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// iterator(ImageCollection *, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:482
// ("cv::ImageCollection::iterator::iterator", vec![(pred!(mut, ["col", "end"], ["cv::ImageCollection*", "int"]), _)]),
pub fn cv_ImageCollection_iterator_iterator_ImageCollectionX_int(col: *mut c_void, end: i32, ocvrs_return: *mut Result<*mut c_void>);
// operator*()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:483
// ("cv::ImageCollection::iterator::operator*", vec![(pred!(mut, [], []), _)]),
pub fn cv_ImageCollection_iterator_operatorX(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator++()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/imgcodecs.hpp:485
// ("cv::ImageCollection::iterator::operator++", vec![(pred!(mut, [], []), _)]),
pub fn cv_ImageCollection_iterator_operatorAA(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::ImageCollection::iterator::delete() generated
// ("cv::ImageCollection::iterator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ImageCollection_iterator_delete(instance: *mut c_void);
