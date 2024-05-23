// haveImageReader(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:535
// ("cv::haveImageReader", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_haveImageReader_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<bool>);
// haveImageWriter(const String &)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:551
// ("cv::haveImageWriter", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_haveImageWriter_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<bool>);
// cv::imcount(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:400
// ("cv::imcount", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_imcount_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<size_t>);
// imcount(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:400
// ("cv::imcount", vec![(pred!(mut, ["filename", "flags"], ["const cv::String*", "int"]), _)]),
pub fn cv_imcount_const_StringR_int(filename: *const c_char, flags: i32, ocvrs_return: *mut Result<size_t>);
// imdecode(InputArray, int)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:467
// ("cv::imdecode", vec![(pred!(mut, ["buf", "flags"], ["const cv::_InputArray*", "int"]), _)]),
pub fn cv_imdecode_const__InputArrayR_int(buf: *const c_void, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
// imdecode(InputArray, int, Mat *)(InputArray, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:476
// ("cv::imdecode", vec![(pred!(mut, ["buf", "flags", "dst"], ["const cv::_InputArray*", "int", "cv::Mat*"]), _)]),
pub fn cv_imdecode_const__InputArrayR_int_MatX(buf: *const c_void, flags: i32, dst: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::imdecodemulti(InputArray, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:491
// ("cv::imdecodemulti", vec![(pred!(mut, ["buf", "flags", "mats"], ["const cv::_InputArray*", "int", "std::vector<cv::Mat>*"]), _)]),
pub fn cv_imdecodemulti_const__InputArrayR_int_vectorLMatGR(buf: *const c_void, flags: i32, mats: *mut c_void, ocvrs_return: *mut Result<bool>);
// imdecodemulti(InputArray, int, std::vector<Mat> &, const cv::Range &)(InputArray, Primitive, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:491
// ("cv::imdecodemulti", vec![(pred!(mut, ["buf", "flags", "mats", "range"], ["const cv::_InputArray*", "int", "std::vector<cv::Mat>*", "const cv::Range*"]), _)]),
pub fn cv_imdecodemulti_const__InputArrayR_int_vectorLMatGR_const_RangeR(buf: *const c_void, flags: i32, mats: *mut c_void, range: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::imencode(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:503
// ("cv::imencode", vec![(pred!(mut, ["ext", "img", "buf"], ["const cv::String*", "const cv::_InputArray*", "std::vector<unsigned char>*"]), _)]),
pub fn cv_imencode_const_StringR_const__InputArrayR_vectorLunsigned_charGR(ext: *const c_char, img: *const c_void, buf: *mut c_void, ocvrs_return: *mut Result<bool>);
// imencode(const String &, InputArray, std::vector<uchar> &, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:503
// ("cv::imencode", vec![(pred!(mut, ["ext", "img", "buf", "params"], ["const cv::String*", "const cv::_InputArray*", "std::vector<unsigned char>*", "const std::vector<int>*"]), _)]),
pub fn cv_imencode_const_StringR_const__InputArrayR_vectorLunsigned_charGR_const_vectorLintGR(ext: *const c_char, img: *const c_void, buf: *mut c_void, params: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::imencodemulti(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:517
// ("cv::imencodemulti", vec![(pred!(mut, ["ext", "imgs", "buf"], ["const cv::String*", "const cv::_InputArray*", "std::vector<unsigned char>*"]), _)]),
pub fn cv_imencodemulti_const_StringR_const__InputArrayR_vectorLunsigned_charGR(ext: *const c_char, imgs: *const c_void, buf: *mut c_void, ocvrs_return: *mut Result<bool>);
// imencodemulti(const String &, InputArrayOfArrays, std::vector<uchar> &, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:517
// ("cv::imencodemulti", vec![(pred!(mut, ["ext", "imgs", "buf", "params"], ["const cv::String*", "const cv::_InputArray*", "std::vector<unsigned char>*", "const std::vector<int>*"]), _)]),
pub fn cv_imencodemulti_const_StringR_const__InputArrayR_vectorLunsigned_charGR_const_vectorLintGR(ext: *const c_char, imgs: *const c_void, buf: *mut c_void, params: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::imread(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:325
// ("cv::imread", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_imread_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::imread(InString, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:336
// ("cv::imread", vec![(pred!(mut, ["filename", "dst"], ["const cv::String*", "const cv::_OutputArray*"]), _)]),
pub fn cv_imread_const_StringR_const__OutputArrayR(filename: *const c_char, dst: *const c_void, ocvrs_return: *mut Result<()>);
// imread(const String &, OutputArray, int)(InString, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:336
// ("cv::imread", vec![(pred!(mut, ["filename", "dst", "flags"], ["const cv::String*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_imread_const_StringR_const__OutputArrayR_int(filename: *const c_char, dst: *const c_void, flags: i32, ocvrs_return: *mut Result<()>);
// imread(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:325
// ("cv::imread", vec![(pred!(mut, ["filename", "flags"], ["const cv::String*", "int"]), _)]),
pub fn cv_imread_const_StringR_int(filename: *const c_char, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::imreadanimation(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:376
// ("cv::imreadanimation", vec![(pred!(mut, ["filename", "animation"], ["const cv::String*", "cv::Animation*"]), _)]),
pub fn cv_imreadanimation_const_StringR_AnimationR(filename: *const c_char, animation: *mut c_void, ocvrs_return: *mut Result<bool>);
// imreadanimation(const String &, Animation &, int, int)(InString, TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:376
// ("cv::imreadanimation", vec![(pred!(mut, ["filename", "animation", "start", "count"], ["const cv::String*", "cv::Animation*", "int", "int"]), _)]),
pub fn cv_imreadanimation_const_StringR_AnimationR_int_int(filename: *const c_char, animation: *mut c_void, start: i32, count: i32, ocvrs_return: *mut Result<bool>);
// cv::imreadmulti(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:346
// ("cv::imreadmulti", vec![(pred!(mut, ["filename", "mats"], ["const cv::String*", "std::vector<cv::Mat>*"]), _)]),
pub fn cv_imreadmulti_const_StringR_vectorLMatGR(filename: *const c_char, mats: *mut c_void, ocvrs_return: *mut Result<bool>);
// imreadmulti(const String &, std::vector<Mat> &, int)(InString, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:346
// ("cv::imreadmulti", vec![(pred!(mut, ["filename", "mats", "flags"], ["const cv::String*", "std::vector<cv::Mat>*", "int"]), _)]),
pub fn cv_imreadmulti_const_StringR_vectorLMatGR_int(filename: *const c_char, mats: *mut c_void, flags: i32, ocvrs_return: *mut Result<bool>);
// cv::imreadmulti(InString, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:358
// ("cv::imreadmulti", vec![(pred!(mut, ["filename", "mats", "start", "count"], ["const cv::String*", "std::vector<cv::Mat>*", "int", "int"]), _)]),
pub fn cv_imreadmulti_const_StringR_vectorLMatGR_int_int(filename: *const c_char, mats: *mut c_void, start: i32, count: i32, ocvrs_return: *mut Result<bool>);
// imreadmulti(const String &, std::vector<Mat> &, int, int, int)(InString, CppPassByVoidPtr, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:358
// ("cv::imreadmulti", vec![(pred!(mut, ["filename", "mats", "start", "count", "flags"], ["const cv::String*", "std::vector<cv::Mat>*", "int", "int", "int"]), _)]),
pub fn cv_imreadmulti_const_StringR_vectorLMatGR_int_int_int(filename: *const c_char, mats: *mut c_void, start: i32, count: i32, flags: i32, ocvrs_return: *mut Result<bool>);
// cv::imwrite(InString, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:445
// ("cv::imwrite", vec![(pred!(mut, ["filename", "img"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
pub fn cv_imwrite_const_StringR_const__InputArrayR(filename: *const c_char, img: *const c_void, ocvrs_return: *mut Result<bool>);
// imwrite(const String &, InputArray, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:445
// ("cv::imwrite", vec![(pred!(mut, ["filename", "img", "params"], ["const cv::String*", "const cv::_InputArray*", "const std::vector<int>*"]), _)]),
pub fn cv_imwrite_const_StringR_const__InputArrayR_const_vectorLintGR(filename: *const c_char, img: *const c_void, params: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::imwriteanimation(InString, TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:390
// ("cv::imwriteanimation", vec![(pred!(mut, ["filename", "animation"], ["const cv::String*", "const cv::Animation*"]), _)]),
pub fn cv_imwriteanimation_const_StringR_const_AnimationR(filename: *const c_char, animation: *const c_void, ocvrs_return: *mut Result<bool>);
// imwriteanimation(const String &, const Animation &, const std::vector<int> &)(InString, TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:390
// ("cv::imwriteanimation", vec![(pred!(mut, ["filename", "animation", "params"], ["const cv::String*", "const cv::Animation*", "const std::vector<int>*"]), _)]),
pub fn cv_imwriteanimation_const_StringR_const_AnimationR_const_vectorLintGR(filename: *const c_char, animation: *const c_void, params: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::imwritemulti(InString, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:450
// ("cv::imwritemulti", vec![(pred!(mut, ["filename", "img"], ["const cv::String*", "const cv::_InputArray*"]), _)]),
pub fn cv_imwritemulti_const_StringR_const__InputArrayR(filename: *const c_char, img: *const c_void, ocvrs_return: *mut Result<bool>);
// imwritemulti(const String &, InputArrayOfArrays, const std::vector<int> &)(InString, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:450
// ("cv::imwritemulti", vec![(pred!(mut, ["filename", "img", "params"], ["const cv::String*", "const cv::_InputArray*", "const std::vector<int>*"]), _)]),
pub fn cv_imwritemulti_const_StringR_const__InputArrayR_const_vectorLintGR(filename: *const c_char, img: *const c_void, params: *const c_void, ocvrs_return: *mut Result<bool>);
// Animation(int, Scalar)(Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:270
// ("cv::Animation::Animation", vec![(pred!(mut, ["loopCount", "bgColor"], ["int", "cv::Scalar"]), _)]),
pub fn cv_Animation_Animation_int_Scalar(loop_count: i32, bg_color: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
// cv::Animation::Animation() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:270
// ("cv::Animation::Animation", vec![(pred!(mut, [], []), _)]),
pub fn cv_Animation_Animation(ocvrs_return: *mut Result<*mut c_void>);
// cv::Animation::implicitClone() generated
// ("cv::Animation::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_Animation_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::Animation::loop_count() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:250
// ("cv::Animation::loop_count", vec![(pred!(const, [], []), _)]),
pub fn cv_Animation_propLoop_count_const(instance: *const c_void) -> i32;
// cv::Animation::setLoop_count(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:250
// ("cv::Animation::setLoop_count", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_Animation_propLoop_count_const_int(instance: *mut c_void, val: i32);
// cv::Animation::bgcolor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:252
// ("cv::Animation::bgcolor", vec![(pred!(const, [], []), _)]),
pub fn cv_Animation_propBgcolor_const(instance: *const c_void, ocvrs_return: *mut core::Scalar);
// cv::Animation::setBgcolor(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:252
// ("cv::Animation::setBgcolor", vec![(pred!(mut, ["val"], ["const cv::Scalar"]), _)]),
pub fn cv_Animation_propBgcolor_const_Scalar(instance: *mut c_void, val: *const core::Scalar);
// cv::Animation::durations() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:254
// ("cv::Animation::durations", vec![(pred!(const, [], []), _)]),
pub fn cv_Animation_propDurations_const(instance: *const c_void) -> *mut c_void;
// cv::Animation::setDurations(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:254
// ("cv::Animation::setDurations", vec![(pred!(mut, ["val"], ["const std::vector<int>"]), _)]),
pub fn cv_Animation_propDurations_const_vectorLintG(instance: *mut c_void, val: *const c_void);
// cv::Animation::frames() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:256
// ("cv::Animation::frames", vec![(pred!(const, [], []), _)]),
pub fn cv_Animation_propFrames_const(instance: *const c_void) -> *mut c_void;
// cv::Animation::setFrames(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:256
// ("cv::Animation::setFrames", vec![(pred!(mut, ["val"], ["const std::vector<cv::Mat>"]), _)]),
pub fn cv_Animation_propFrames_const_vectorLMatG(instance: *mut c_void, val: *const c_void);
// cv::Animation::delete() generated
// ("cv::Animation::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Animation_delete(instance: *mut c_void);
// ImageCollection()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:583
// ("cv::ImageCollection::ImageCollection", vec![(pred!(mut, [], []), _)]),
pub fn cv_ImageCollection_ImageCollection(ocvrs_return: *mut Result<*mut c_void>);
// ImageCollection(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:584
// ("cv::ImageCollection::ImageCollection", vec![(pred!(mut, ["filename", "flags"], ["const cv::String*", "int"]), _)]),
pub fn cv_ImageCollection_ImageCollection_const_StringR_int(filename: *const c_char, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
// init(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:585
// ("cv::ImageCollection::init", vec![(pred!(mut, ["img", "flags"], ["const cv::String*", "int"]), _)]),
pub fn cv_ImageCollection_init_const_StringR_int(instance: *mut c_void, img: *const c_char, flags: i32, ocvrs_return: *mut Result<()>);
// size()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:586
// ("cv::ImageCollection::size", vec![(pred!(const, [], []), _)]),
pub fn cv_ImageCollection_size_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// at(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:587
// ("cv::ImageCollection::at", vec![(pred!(mut, ["index"], ["int"]), _)]),
pub fn cv_ImageCollection_at_int(instance: *mut c_void, index: i32, ocvrs_return: *mut Result<*mut c_void>);
// operator[](int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:588
// ("cv::ImageCollection::operator[]", vec![(pred!(mut, ["index"], ["int"]), _)]),
pub fn cv_ImageCollection_operator___int(instance: *mut c_void, index: i32, ocvrs_return: *mut Result<*mut c_void>);
// releaseCache(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:589
// ("cv::ImageCollection::releaseCache", vec![(pred!(mut, ["index"], ["int"]), _)]),
pub fn cv_ImageCollection_releaseCache_int(instance: *mut c_void, index: i32, ocvrs_return: *mut Result<()>);
// begin()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:590
// ("cv::ImageCollection::begin", vec![(pred!(mut, [], []), _)]),
pub fn cv_ImageCollection_begin(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// end()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:591
// ("cv::ImageCollection::end", vec![(pred!(mut, [], []), _)]),
pub fn cv_ImageCollection_end(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::ImageCollection::delete() generated
// ("cv::ImageCollection::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ImageCollection_delete(instance: *mut c_void);
// iterator(ImageCollection *)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:569
// ("cv::ImageCollection::iterator::iterator", vec![(pred!(mut, ["col"], ["cv::ImageCollection*"]), _)]),
pub fn cv_ImageCollection_iterator_iterator_ImageCollectionX(col: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// iterator(ImageCollection *, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:570
// ("cv::ImageCollection::iterator::iterator", vec![(pred!(mut, ["col", "end"], ["cv::ImageCollection*", "int"]), _)]),
pub fn cv_ImageCollection_iterator_iterator_ImageCollectionX_int(col: *mut c_void, end: i32, ocvrs_return: *mut Result<*mut c_void>);
// operator*()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:571
// ("cv::ImageCollection::iterator::operator*", vec![(pred!(mut, [], []), _)]),
pub fn cv_ImageCollection_iterator_operatorX(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// operator++()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/imgcodecs.hpp:573
// ("cv::ImageCollection::iterator::operator++", vec![(pred!(mut, [], []), _)]),
pub fn cv_ImageCollection_iterator_operatorAA(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::ImageCollection::iterator::delete() generated
// ("cv::ImageCollection::iterator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ImageCollection_iterator_delete(instance: *mut c_void);
