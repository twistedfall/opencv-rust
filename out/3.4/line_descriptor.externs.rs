// cv::line_descriptor::drawKeylines(TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:1399
// ("cv::line_descriptor::drawKeylines", vec![(pred!(mut, ["image", "keylines", "outImage"], ["const cv::Mat*", "const std::vector<cv::line_descriptor::KeyLine>*", "cv::Mat*"]), _)]),
pub fn cv_line_descriptor_drawKeylines_const_MatR_const_vectorLKeyLineGR_MatR(image: *const c_void, keylines: *const c_void, out_image: *mut c_void, ocvrs_return: *mut Result<()>);
// drawKeylines(const Mat &, const std::vector<KeyLine> &, Mat &, const Scalar &, int)(TraitClass, CppPassByVoidPtr, TraitClass, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:1399
// ("cv::line_descriptor::drawKeylines", vec![(pred!(mut, ["image", "keylines", "outImage", "color", "flags"], ["const cv::Mat*", "const std::vector<cv::line_descriptor::KeyLine>*", "cv::Mat*", "const cv::Scalar*", "int"]), _)]),
pub fn cv_line_descriptor_drawKeylines_const_MatR_const_vectorLKeyLineGR_MatR_const_ScalarR_int(image: *const c_void, keylines: *const c_void, out_image: *mut c_void, color: *const core::Scalar, flags: i32, ocvrs_return: *mut Result<()>);
// cv::line_descriptor::drawLineMatches(TraitClass, CppPassByVoidPtr, TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:1386
// ("cv::line_descriptor::drawLineMatches", vec![(pred!(mut, ["img1", "keylines1", "img2", "keylines2", "matches1to2", "outImg"], ["const cv::Mat*", "const std::vector<cv::line_descriptor::KeyLine>*", "const cv::Mat*", "const std::vector<cv::line_descriptor::KeyLine>*", "const std::vector<cv::DMatch>*", "cv::Mat*"]), _)]),
pub fn cv_line_descriptor_drawLineMatches_const_MatR_const_vectorLKeyLineGR_const_MatR_const_vectorLKeyLineGR_const_vectorLDMatchGR_MatR(img1: *const c_void, keylines1: *const c_void, img2: *const c_void, keylines2: *const c_void, matches1to2: *const c_void, out_img: *mut c_void, ocvrs_return: *mut Result<()>);
// drawLineMatches(const Mat &, const std::vector<KeyLine> &, const Mat &, const std::vector<KeyLine> &, const std::vector<DMatch> &, Mat &, const Scalar &, const Scalar &, const std::vector<char> &, int)(TraitClass, CppPassByVoidPtr, TraitClass, CppPassByVoidPtr, CppPassByVoidPtr, TraitClass, SimpleClass, SimpleClass, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:1386
// ("cv::line_descriptor::drawLineMatches", vec![(pred!(mut, ["img1", "keylines1", "img2", "keylines2", "matches1to2", "outImg", "matchColor", "singleLineColor", "matchesMask", "flags"], ["const cv::Mat*", "const std::vector<cv::line_descriptor::KeyLine>*", "const cv::Mat*", "const std::vector<cv::line_descriptor::KeyLine>*", "const std::vector<cv::DMatch>*", "cv::Mat*", "const cv::Scalar*", "const cv::Scalar*", "const std::vector<char>*", "int"]), _)]),
pub fn cv_line_descriptor_drawLineMatches_const_MatR_const_vectorLKeyLineGR_const_MatR_const_vectorLKeyLineGR_const_vectorLDMatchGR_MatR_const_ScalarR_const_ScalarR_const_vectorLcharGR_int(img1: *const c_void, keylines1: *const c_void, img2: *const c_void, keylines2: *const c_void, matches1to2: *const c_void, out_img: *mut c_void, match_color: *const core::Scalar, single_line_color: *const core::Scalar, matches_mask: *const c_void, flags: i32, ocvrs_return: *mut Result<()>);
// BinaryDescriptor(const BinaryDescriptor::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:219
// ("cv::line_descriptor::BinaryDescriptor::BinaryDescriptor", vec![(pred!(mut, ["parameters"], ["const cv::line_descriptor::BinaryDescriptor::Params*"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_BinaryDescriptor_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::line_descriptor::BinaryDescriptor::BinaryDescriptor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:219
// ("cv::line_descriptor::BinaryDescriptor::BinaryDescriptor", vec![(pred!(mut, [], []), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_BinaryDescriptor(ocvrs_return: *mut Result<*mut c_void>);
// createBinaryDescriptor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:224
// ("cv::line_descriptor::BinaryDescriptor::createBinaryDescriptor", vec![(pred!(mut, [], []), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_createBinaryDescriptor(ocvrs_return: *mut Result<*mut c_void>);
// createBinaryDescriptor(Params)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:225
// ("cv::line_descriptor::BinaryDescriptor::createBinaryDescriptor", vec![(pred!(mut, ["parameters"], ["cv::line_descriptor::BinaryDescriptor::Params"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_createBinaryDescriptor_Params(parameters: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// getNumOfOctaves()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:232
// ("cv::line_descriptor::BinaryDescriptor::getNumOfOctaves", vec![(pred!(mut, [], []), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_getNumOfOctaves(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// setNumOfOctaves(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:236
// ("cv::line_descriptor::BinaryDescriptor::setNumOfOctaves", vec![(pred!(mut, ["octaves"], ["int"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_setNumOfOctaves_int(instance: *mut c_void, octaves: i32, ocvrs_return: *mut Result<()>);
// getWidthOfBand()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:239
// ("cv::line_descriptor::BinaryDescriptor::getWidthOfBand", vec![(pred!(mut, [], []), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_getWidthOfBand(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// setWidthOfBand(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:243
// ("cv::line_descriptor::BinaryDescriptor::setWidthOfBand", vec![(pred!(mut, ["width"], ["int"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_setWidthOfBand_int(instance: *mut c_void, width: i32, ocvrs_return: *mut Result<()>);
// getReductionRatio()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:246
// ("cv::line_descriptor::BinaryDescriptor::getReductionRatio", vec![(pred!(mut, [], []), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_getReductionRatio(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// setReductionRatio(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:250
// ("cv::line_descriptor::BinaryDescriptor::setReductionRatio", vec![(pred!(mut, ["rRatio"], ["int"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_setReductionRatio_int(instance: *mut c_void, r_ratio: i32, ocvrs_return: *mut Result<()>);
// read(const cv::FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:256
// ("cv::line_descriptor::BinaryDescriptor::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(cv::FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:262
// ("cv::line_descriptor::BinaryDescriptor::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// detect(const Mat &, std::vector<KeyLine> &, const Mat &)(TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:270
// ("cv::line_descriptor::BinaryDescriptor::detect", vec![(pred!(mut, ["image", "keypoints", "mask"], ["const cv::Mat*", "std::vector<cv::line_descriptor::KeyLine>*", "const cv::Mat*"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_detect_const_MatR_vectorLKeyLineGR_const_MatR(instance: *mut c_void, image: *const c_void, keypoints: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::line_descriptor::BinaryDescriptor::detect(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:270
// ("cv::line_descriptor::BinaryDescriptor::detect", vec![(pred!(mut, ["image", "keypoints"], ["const cv::Mat*", "std::vector<cv::line_descriptor::KeyLine>*"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_detect_const_MatR_vectorLKeyLineGR(instance: *mut c_void, image: *const c_void, keypoints: *mut c_void, ocvrs_return: *mut Result<()>);
// detect(const std::vector<Mat> &, std::vector<std::vector<KeyLine>> &, const std::vector<Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:278
// ("cv::line_descriptor::BinaryDescriptor::detect", vec![(pred!(const, ["images", "keylines", "masks"], ["const std::vector<cv::Mat>*", "std::vector<std::vector<cv::line_descriptor::KeyLine>>*", "const std::vector<cv::Mat>*"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_detect_const_const_vectorLMatGR_vectorLvectorLKeyLineGGR_const_vectorLMatGR(instance: *const c_void, images: *const c_void, keylines: *mut c_void, masks: *const c_void, ocvrs_return: *mut Result<()>);
// cv::line_descriptor::BinaryDescriptor::detect(CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:278
// ("cv::line_descriptor::BinaryDescriptor::detect", vec![(pred!(const, ["images", "keylines"], ["const std::vector<cv::Mat>*", "std::vector<std::vector<cv::line_descriptor::KeyLine>>*"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_detect_const_const_vectorLMatGR_vectorLvectorLKeyLineGGR(instance: *const c_void, images: *const c_void, keylines: *mut c_void, ocvrs_return: *mut Result<()>);
// compute(const Mat &, std::vector<KeyLine> &, Mat &, bool)(TraitClass, CppPassByVoidPtr, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:288
// ("cv::line_descriptor::BinaryDescriptor::compute", vec![(pred!(const, ["image", "keylines", "descriptors", "returnFloatDescr"], ["const cv::Mat*", "std::vector<cv::line_descriptor::KeyLine>*", "cv::Mat*", "bool"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_compute_const_const_MatR_vectorLKeyLineGR_MatR_bool(instance: *const c_void, image: *const c_void, keylines: *mut c_void, descriptors: *mut c_void, return_float_descr: bool, ocvrs_return: *mut Result<()>);
// cv::line_descriptor::BinaryDescriptor::compute(TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:288
// ("cv::line_descriptor::BinaryDescriptor::compute", vec![(pred!(const, ["image", "keylines", "descriptors"], ["const cv::Mat*", "std::vector<cv::line_descriptor::KeyLine>*", "cv::Mat*"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_compute_const_const_MatR_vectorLKeyLineGR_MatR(instance: *const c_void, image: *const c_void, keylines: *mut c_void, descriptors: *mut c_void, ocvrs_return: *mut Result<()>);
// compute(const std::vector<Mat> &, std::vector<std::vector<KeyLine>> &, std::vector<Mat> &, bool)(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:297
// ("cv::line_descriptor::BinaryDescriptor::compute", vec![(pred!(const, ["images", "keylines", "descriptors", "returnFloatDescr"], ["const std::vector<cv::Mat>*", "std::vector<std::vector<cv::line_descriptor::KeyLine>>*", "std::vector<cv::Mat>*", "bool"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_compute_const_const_vectorLMatGR_vectorLvectorLKeyLineGGR_vectorLMatGR_bool(instance: *const c_void, images: *const c_void, keylines: *mut c_void, descriptors: *mut c_void, return_float_descr: bool, ocvrs_return: *mut Result<()>);
// cv::line_descriptor::BinaryDescriptor::compute(CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:297
// ("cv::line_descriptor::BinaryDescriptor::compute", vec![(pred!(const, ["images", "keylines", "descriptors"], ["const std::vector<cv::Mat>*", "std::vector<std::vector<cv::line_descriptor::KeyLine>>*", "std::vector<cv::Mat>*"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_compute_const_const_vectorLMatGR_vectorLvectorLKeyLineGGR_vectorLMatGR(instance: *const c_void, images: *const c_void, keylines: *mut c_void, descriptors: *mut c_void, ocvrs_return: *mut Result<()>);
// descriptorSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:302
// ("cv::line_descriptor::BinaryDescriptor::descriptorSize", vec![(pred!(const, [], []), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_descriptorSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// descriptorType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:306
// ("cv::line_descriptor::BinaryDescriptor::descriptorType", vec![(pred!(const, [], []), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_descriptorType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// defaultNorm()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:310
// ("cv::line_descriptor::BinaryDescriptor::defaultNorm", vec![(pred!(const, [], []), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_defaultNorm_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// operator()(InputArray, InputArray, std::vector<KeyLine> &, OutputArray, bool, bool)(InputArray, InputArray, CppPassByVoidPtr, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:324
// ("cv::line_descriptor::BinaryDescriptor::operator()", vec![(pred!(const, ["image", "mask", "keylines", "descriptors", "useProvidedKeyLines", "returnFloatDescr"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::line_descriptor::KeyLine>*", "const cv::_OutputArray*", "bool", "bool"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_operator___const_const__InputArrayR_const__InputArrayR_vectorLKeyLineGR_const__OutputArrayR_bool_bool(instance: *const c_void, image: *const c_void, mask: *const c_void, keylines: *mut c_void, descriptors: *const c_void, use_provided_key_lines: bool, return_float_descr: bool, ocvrs_return: *mut Result<()>);
// cv::line_descriptor::BinaryDescriptor::operator()(InputArray, InputArray, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:324
// ("cv::line_descriptor::BinaryDescriptor::operator()", vec![(pred!(const, ["image", "mask", "keylines", "descriptors"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::line_descriptor::KeyLine>*", "const cv::_OutputArray*"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_operator___const_const__InputArrayR_const__InputArrayR_vectorLKeyLineGR_const__OutputArrayR(instance: *const c_void, image: *const c_void, mask: *const c_void, keylines: *mut c_void, descriptors: *const c_void, ocvrs_return: *mut Result<()>);
// cv::line_descriptor::BinaryDescriptor::to_Algorithm() generated
// ("cv::line_descriptor::BinaryDescriptor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::line_descriptor::BinaryDescriptor::delete() generated
// ("cv::line_descriptor::BinaryDescriptor::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:189
// ("cv::line_descriptor::BinaryDescriptor::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:205
// ("cv::line_descriptor::BinaryDescriptor::Params::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_Params_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:208
// ("cv::line_descriptor::BinaryDescriptor::Params::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_Params_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::line_descriptor::BinaryDescriptor::Params::numOfOctave_() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:193
// ("cv::line_descriptor::BinaryDescriptor::Params::numOfOctave_", vec![(pred!(const, [], []), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_Params_propNumOfOctave__const(instance: *const c_void) -> i32;
// cv::line_descriptor::BinaryDescriptor::Params::setNumOfOctave_(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:193
// ("cv::line_descriptor::BinaryDescriptor::Params::setNumOfOctave_", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_Params_propNumOfOctave__const_int(instance: *mut c_void, val: i32);
// cv::line_descriptor::BinaryDescriptor::Params::widthOfBand_() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:197
// ("cv::line_descriptor::BinaryDescriptor::Params::widthOfBand_", vec![(pred!(const, [], []), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_Params_propWidthOfBand__const(instance: *const c_void) -> i32;
// cv::line_descriptor::BinaryDescriptor::Params::setWidthOfBand_(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:197
// ("cv::line_descriptor::BinaryDescriptor::Params::setWidthOfBand_", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_Params_propWidthOfBand__const_int(instance: *mut c_void, val: i32);
// cv::line_descriptor::BinaryDescriptor::Params::reductionRatio() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:200
// ("cv::line_descriptor::BinaryDescriptor::Params::reductionRatio", vec![(pred!(const, [], []), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_Params_propReductionRatio_const(instance: *const c_void) -> i32;
// cv::line_descriptor::BinaryDescriptor::Params::setReductionRatio(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:200
// ("cv::line_descriptor::BinaryDescriptor::Params::setReductionRatio", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_Params_propReductionRatio_const_int(instance: *mut c_void, val: i32);
// cv::line_descriptor::BinaryDescriptor::Params::ksize_() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:202
// ("cv::line_descriptor::BinaryDescriptor::Params::ksize_", vec![(pred!(const, [], []), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_Params_propKsize__const(instance: *const c_void) -> i32;
// cv::line_descriptor::BinaryDescriptor::Params::setKsize_(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:202
// ("cv::line_descriptor::BinaryDescriptor::Params::setKsize_", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_Params_propKsize__const_int(instance: *mut c_void, val: i32);
// cv::line_descriptor::BinaryDescriptor::Params::delete() generated
// ("cv::line_descriptor::BinaryDescriptor::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_line_descriptor_BinaryDescriptor_Params_delete(instance: *mut c_void);
// match(const Mat &, const Mat &, std::vector<DMatch> &, const Mat &)(TraitClass, TraitClass, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:1029
// ("cv::line_descriptor::BinaryDescriptorMatcher::match", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "mask"], ["const cv::Mat*", "const cv::Mat*", "std::vector<cv::DMatch>*", "const cv::Mat*"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptorMatcher_match_const_const_MatR_const_MatR_vectorLDMatchGR_const_MatR(instance: *const c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::line_descriptor::BinaryDescriptorMatcher::match(TraitClass, TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:1029
// ("cv::line_descriptor::BinaryDescriptorMatcher::match", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches"], ["const cv::Mat*", "const cv::Mat*", "std::vector<cv::DMatch>*"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptorMatcher_match_const_const_MatR_const_MatR_vectorLDMatchGR(instance: *const c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, ocvrs_return: *mut Result<()>);
// match(const Mat &, std::vector<DMatch> &, const std::vector<Mat> &)(TraitClass, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:1038
// ("cv::line_descriptor::BinaryDescriptorMatcher::match", vec![(pred!(mut, ["queryDescriptors", "matches", "masks"], ["const cv::Mat*", "std::vector<cv::DMatch>*", "const std::vector<cv::Mat>*"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptorMatcher_match_const_MatR_vectorLDMatchGR_const_vectorLMatGR(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, masks: *const c_void, ocvrs_return: *mut Result<()>);
// cv::line_descriptor::BinaryDescriptorMatcher::match(TraitClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:1038
// ("cv::line_descriptor::BinaryDescriptorMatcher::match", vec![(pred!(mut, ["queryDescriptors", "matches"], ["const cv::Mat*", "std::vector<cv::DMatch>*"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptorMatcher_match_const_MatR_vectorLDMatchGR(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, ocvrs_return: *mut Result<()>);
// knnMatch(const Mat &, const Mat &, std::vector<std::vector<DMatch>> &, int, const Mat &, bool)(TraitClass, TraitClass, CppPassByVoidPtr, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:1051
// ("cv::line_descriptor::BinaryDescriptorMatcher::knnMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "k", "mask", "compactResult"], ["const cv::Mat*", "const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "int", "const cv::Mat*", "bool"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptorMatcher_knnMatch_const_const_MatR_const_MatR_vectorLvectorLDMatchGGR_int_const_MatR_bool(instance: *const c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, k: i32, mask: *const c_void, compact_result: bool, ocvrs_return: *mut Result<()>);
// cv::line_descriptor::BinaryDescriptorMatcher::knnMatch(TraitClass, TraitClass, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:1051
// ("cv::line_descriptor::BinaryDescriptorMatcher::knnMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "k"], ["const cv::Mat*", "const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "int"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptorMatcher_knnMatch_const_const_MatR_const_MatR_vectorLvectorLDMatchGGR_int(instance: *const c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, k: i32, ocvrs_return: *mut Result<()>);
// knnMatch(const Mat &, std::vector<std::vector<DMatch>> &, int, const std::vector<Mat> &, bool)(TraitClass, CppPassByVoidPtr, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:1064
// ("cv::line_descriptor::BinaryDescriptorMatcher::knnMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "k", "masks", "compactResult"], ["const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "int", "const std::vector<cv::Mat>*", "bool"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptorMatcher_knnMatch_const_MatR_vectorLvectorLDMatchGGR_int_const_vectorLMatGR_bool(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, k: i32, masks: *const c_void, compact_result: bool, ocvrs_return: *mut Result<()>);
// cv::line_descriptor::BinaryDescriptorMatcher::knnMatch(TraitClass, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:1064
// ("cv::line_descriptor::BinaryDescriptorMatcher::knnMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "k"], ["const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "int"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptorMatcher_knnMatch_const_MatR_vectorLvectorLDMatchGGR_int(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, k: i32, ocvrs_return: *mut Result<()>);
// radiusMatch(const Mat &, const Mat &, std::vector<std::vector<DMatch>> &, float, const Mat &, bool)(TraitClass, TraitClass, CppPassByVoidPtr, Primitive, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:1078
// ("cv::line_descriptor::BinaryDescriptorMatcher::radiusMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "maxDistance", "mask", "compactResult"], ["const cv::Mat*", "const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "float", "const cv::Mat*", "bool"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptorMatcher_radiusMatch_const_const_MatR_const_MatR_vectorLvectorLDMatchGGR_float_const_MatR_bool(instance: *const c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, max_distance: f32, mask: *const c_void, compact_result: bool, ocvrs_return: *mut Result<()>);
// cv::line_descriptor::BinaryDescriptorMatcher::radiusMatch(TraitClass, TraitClass, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:1078
// ("cv::line_descriptor::BinaryDescriptorMatcher::radiusMatch", vec![(pred!(const, ["queryDescriptors", "trainDescriptors", "matches", "maxDistance"], ["const cv::Mat*", "const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "float"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptorMatcher_radiusMatch_const_const_MatR_const_MatR_vectorLvectorLDMatchGGR_float(instance: *const c_void, query_descriptors: *const c_void, train_descriptors: *const c_void, matches: *mut c_void, max_distance: f32, ocvrs_return: *mut Result<()>);
// radiusMatch(const Mat &, std::vector<std::vector<DMatch>> &, float, const std::vector<Mat> &, bool)(TraitClass, CppPassByVoidPtr, Primitive, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:1091
// ("cv::line_descriptor::BinaryDescriptorMatcher::radiusMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "maxDistance", "masks", "compactResult"], ["const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "float", "const std::vector<cv::Mat>*", "bool"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptorMatcher_radiusMatch_const_MatR_vectorLvectorLDMatchGGR_float_const_vectorLMatGR_bool(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, max_distance: f32, masks: *const c_void, compact_result: bool, ocvrs_return: *mut Result<()>);
// cv::line_descriptor::BinaryDescriptorMatcher::radiusMatch(TraitClass, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:1091
// ("cv::line_descriptor::BinaryDescriptorMatcher::radiusMatch", vec![(pred!(mut, ["queryDescriptors", "matches", "maxDistance"], ["const cv::Mat*", "std::vector<std::vector<cv::DMatch>>*", "float"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptorMatcher_radiusMatch_const_MatR_vectorLvectorLDMatchGGR_float(instance: *mut c_void, query_descriptors: *const c_void, matches: *mut c_void, max_distance: f32, ocvrs_return: *mut Result<()>);
// add(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:1102
// ("cv::line_descriptor::BinaryDescriptorMatcher::add", vec![(pred!(mut, ["descriptors"], ["const std::vector<cv::Mat>*"]), _)]),
pub fn cv_line_descriptor_BinaryDescriptorMatcher_add_const_vectorLMatGR(instance: *mut c_void, descriptors: *const c_void, ocvrs_return: *mut Result<()>);
// train()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:1109
// ("cv::line_descriptor::BinaryDescriptorMatcher::train", vec![(pred!(mut, [], []), _)]),
pub fn cv_line_descriptor_BinaryDescriptorMatcher_train(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// createBinaryDescriptorMatcher()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:1113
// ("cv::line_descriptor::BinaryDescriptorMatcher::createBinaryDescriptorMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_line_descriptor_BinaryDescriptorMatcher_createBinaryDescriptorMatcher(ocvrs_return: *mut Result<*mut c_void>);
// clear()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:1117
// ("cv::line_descriptor::BinaryDescriptorMatcher::clear", vec![(pred!(mut, [], []), _)]),
pub fn cv_line_descriptor_BinaryDescriptorMatcher_clear(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// BinaryDescriptorMatcher()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:1123
// ("cv::line_descriptor::BinaryDescriptorMatcher::BinaryDescriptorMatcher", vec![(pred!(mut, [], []), _)]),
pub fn cv_line_descriptor_BinaryDescriptorMatcher_BinaryDescriptorMatcher(ocvrs_return: *mut Result<*mut c_void>);
// cv::line_descriptor::BinaryDescriptorMatcher::to_Algorithm() generated
// ("cv::line_descriptor::BinaryDescriptorMatcher::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_line_descriptor_BinaryDescriptorMatcher_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::line_descriptor::BinaryDescriptorMatcher::delete() generated
// ("cv::line_descriptor::BinaryDescriptorMatcher::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_line_descriptor_BinaryDescriptorMatcher_delete(instance: *mut c_void);
// getStartPoint()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:144
// ("cv::line_descriptor::KeyLine::getStartPoint", vec![(pred!(const, [], []), _)]),
pub fn cv_line_descriptor_KeyLine_getStartPoint_const(instance: *const crate::line_descriptor::KeyLine, ocvrs_return: *mut Result<core::Point2f>);
// getEndPoint()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:150
// ("cv::line_descriptor::KeyLine::getEndPoint", vec![(pred!(const, [], []), _)]),
pub fn cv_line_descriptor_KeyLine_getEndPoint_const(instance: *const crate::line_descriptor::KeyLine, ocvrs_return: *mut Result<core::Point2f>);
// getStartPointInOctave()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:156
// ("cv::line_descriptor::KeyLine::getStartPointInOctave", vec![(pred!(const, [], []), _)]),
pub fn cv_line_descriptor_KeyLine_getStartPointInOctave_const(instance: *const crate::line_descriptor::KeyLine, ocvrs_return: *mut Result<core::Point2f>);
// getEndPointInOctave()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:162
// ("cv::line_descriptor::KeyLine::getEndPointInOctave", vec![(pred!(const, [], []), _)]),
pub fn cv_line_descriptor_KeyLine_getEndPointInOctave_const(instance: *const crate::line_descriptor::KeyLine, ocvrs_return: *mut Result<core::Point2f>);
// KeyLine()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:168
// ("cv::line_descriptor::KeyLine::KeyLine", vec![(pred!(mut, [], []), _)]),
pub fn cv_line_descriptor_KeyLine_KeyLine(ocvrs_return: *mut Result<crate::line_descriptor::KeyLine>);
// LSDDetector()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:927
// ("cv::line_descriptor::LSDDetector::LSDDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_line_descriptor_LSDDetector_LSDDetector(ocvrs_return: *mut Result<*mut c_void>);
// LSDDetector(LSDParam)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:932
// ("cv::line_descriptor::LSDDetector::LSDDetector", vec![(pred!(mut, ["_params"], ["cv::line_descriptor::LSDParam"]), _)]),
pub fn cv_line_descriptor_LSDDetector_LSDDetector_LSDParam(_params: *const crate::line_descriptor::LSDParam, ocvrs_return: *mut Result<*mut c_void>);
// createLSDDetector()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:939
// ("cv::line_descriptor::LSDDetector::createLSDDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_line_descriptor_LSDDetector_createLSDDetector(ocvrs_return: *mut Result<*mut c_void>);
// createLSDDetector(LSDParam)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:942
// ("cv::line_descriptor::LSDDetector::createLSDDetector", vec![(pred!(mut, ["params"], ["cv::line_descriptor::LSDParam"]), _)]),
pub fn cv_line_descriptor_LSDDetector_createLSDDetector_LSDParam(params: *const crate::line_descriptor::LSDParam, ocvrs_return: *mut Result<*mut c_void>);
// detect(const Mat &, std::vector<KeyLine> &, int, int, const Mat &)(TraitClass, CppPassByVoidPtr, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:953
// ("cv::line_descriptor::LSDDetector::detect", vec![(pred!(mut, ["image", "keypoints", "scale", "numOctaves", "mask"], ["const cv::Mat*", "std::vector<cv::line_descriptor::KeyLine>*", "int", "int", "const cv::Mat*"]), _)]),
pub fn cv_line_descriptor_LSDDetector_detect_const_MatR_vectorLKeyLineGR_int_int_const_MatR(instance: *mut c_void, image: *const c_void, keypoints: *mut c_void, scale: i32, num_octaves: i32, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::line_descriptor::LSDDetector::detect(TraitClass, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:953
// ("cv::line_descriptor::LSDDetector::detect", vec![(pred!(mut, ["image", "keypoints", "scale", "numOctaves"], ["const cv::Mat*", "std::vector<cv::line_descriptor::KeyLine>*", "int", "int"]), _)]),
pub fn cv_line_descriptor_LSDDetector_detect_const_MatR_vectorLKeyLineGR_int_int(instance: *mut c_void, image: *const c_void, keypoints: *mut c_void, scale: i32, num_octaves: i32, ocvrs_return: *mut Result<()>);
// detect(const std::vector<Mat> &, std::vector<std::vector<KeyLine>> &, int, int, const std::vector<Mat> &)(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:962
// ("cv::line_descriptor::LSDDetector::detect", vec![(pred!(const, ["images", "keylines", "scale", "numOctaves", "masks"], ["const std::vector<cv::Mat>*", "std::vector<std::vector<cv::line_descriptor::KeyLine>>*", "int", "int", "const std::vector<cv::Mat>*"]), _)]),
pub fn cv_line_descriptor_LSDDetector_detect_const_const_vectorLMatGR_vectorLvectorLKeyLineGGR_int_int_const_vectorLMatGR(instance: *const c_void, images: *const c_void, keylines: *mut c_void, scale: i32, num_octaves: i32, masks: *const c_void, ocvrs_return: *mut Result<()>);
// cv::line_descriptor::LSDDetector::detect(CppPassByVoidPtr, CppPassByVoidPtr, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:962
// ("cv::line_descriptor::LSDDetector::detect", vec![(pred!(const, ["images", "keylines", "scale", "numOctaves"], ["const std::vector<cv::Mat>*", "std::vector<std::vector<cv::line_descriptor::KeyLine>>*", "int", "int"]), _)]),
pub fn cv_line_descriptor_LSDDetector_detect_const_const_vectorLMatGR_vectorLvectorLKeyLineGGR_int_int(instance: *const c_void, images: *const c_void, keylines: *mut c_void, scale: i32, num_octaves: i32, ocvrs_return: *mut Result<()>);
// cv::line_descriptor::LSDDetector::to_Algorithm() generated
// ("cv::line_descriptor::LSDDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_line_descriptor_LSDDetector_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::line_descriptor::LSDDetector::delete() generated
// ("cv::line_descriptor::LSDDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_line_descriptor_LSDDetector_delete(instance: *mut c_void);
// LSDParam()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/line_descriptor/descriptor.hpp:912
// ("cv::line_descriptor::LSDParam::LSDParam", vec![(pred!(mut, [], []), _)]),
pub fn cv_line_descriptor_LSDParam_LSDParam(ocvrs_return: *mut Result<crate::line_descriptor::LSDParam>);
