// MSERsToERStats(InputArray, std::vector<std::vector<Point>> &, std::vector<std::vector<ERStat>> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:347
// ("cv::text::MSERsToERStats", vec![(pred!(mut, ["image", "contours", "regions"], ["const cv::_InputArray*", "std::vector<std::vector<cv::Point>>*", "std::vector<std::vector<cv::text::ERStat>>*"]), _)]),
pub fn cv_text_MSERsToERStats_const__InputArrayR_vectorLvectorLPointGGR_vectorLvectorLERStatGGR(image: *const c_void, contours: *mut c_void, regions: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::text::computeNMChannels(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:262
// ("cv::text::computeNMChannels", vec![(pred!(mut, ["_src", "_channels"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_text_computeNMChannels_const__InputArrayR_const__OutputArrayR(_src: *const c_void, _channels: *const c_void, ocvrs_return: *mut Result<()>);
// computeNMChannels(InputArray, OutputArrayOfArrays, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:262
// ("cv::text::computeNMChannels", vec![(pred!(mut, ["_src", "_channels", "_mode"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_text_computeNMChannels_const__InputArrayR_const__OutputArrayR_int(_src: *const c_void, _channels: *const c_void, _mode: i32, ocvrs_return: *mut Result<()>);
// cv::text::createERFilterNM1(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:187
// ("cv::text::createERFilterNM1", vec![(pred!(mut, ["cb"], ["const cv::Ptr<cv::text::ERFilter::Callback>*"]), _)]),
pub fn cv_text_createERFilterNM1_const_PtrLCallbackGR(cb: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// createERFilterNM1(const Ptr<ERFilter::Callback> &, int, float, float, float, bool, float)(CppPassByVoidPtr, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:187
// ("cv::text::createERFilterNM1", vec![(pred!(mut, ["cb", "thresholdDelta", "minArea", "maxArea", "minProbability", "nonMaxSuppression", "minProbabilityDiff"], ["const cv::Ptr<cv::text::ERFilter::Callback>*", "int", "float", "float", "float", "bool", "float"]), _)]),
pub fn cv_text_createERFilterNM1_const_PtrLCallbackGR_int_float_float_float_bool_float(cb: *const c_void, threshold_delta: i32, min_area: f32, max_area: f32, min_probability: f32, non_max_suppression: bool, min_probability_diff: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::text::createERFilterNM1(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:212
// ("cv::text::createERFilterNM1", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_text_createERFilterNM1_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// createERFilterNM1(const String &, int, float, float, float, bool, float)(InString, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:212
// ("cv::text::createERFilterNM1", vec![(pred!(mut, ["filename", "thresholdDelta", "minArea", "maxArea", "minProbability", "nonMaxSuppression", "minProbabilityDiff"], ["const cv::String*", "int", "float", "float", "float", "bool", "float"]), _)]),
pub fn cv_text_createERFilterNM1_const_StringR_int_float_float_float_bool_float(filename: *const c_char, threshold_delta: i32, min_area: f32, max_area: f32, min_probability: f32, non_max_suppression: bool, min_probability_diff: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::text::createERFilterNM2(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:204
// ("cv::text::createERFilterNM2", vec![(pred!(mut, ["cb"], ["const cv::Ptr<cv::text::ERFilter::Callback>*"]), _)]),
pub fn cv_text_createERFilterNM2_const_PtrLCallbackGR(cb: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// createERFilterNM2(const Ptr<ERFilter::Callback> &, float)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:204
// ("cv::text::createERFilterNM2", vec![(pred!(mut, ["cb", "minProbability"], ["const cv::Ptr<cv::text::ERFilter::Callback>*", "float"]), _)]),
pub fn cv_text_createERFilterNM2_const_PtrLCallbackGR_float(cb: *const c_void, min_probability: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::text::createERFilterNM2(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:223
// ("cv::text::createERFilterNM2", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_text_createERFilterNM2_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// createERFilterNM2(const String &, float)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:223
// ("cv::text::createERFilterNM2", vec![(pred!(mut, ["filename", "minProbability"], ["const cv::String*", "float"]), _)]),
pub fn cv_text_createERFilterNM2_const_StringR_float(filename: *const c_char, min_probability: f32, ocvrs_return: *mut Result<*mut c_void>);
// createOCRHMMTransitionsTable(const String &, std::vector<cv::String> &)(InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:391
// ("cv::text::createOCRHMMTransitionsTable", vec![(pred!(mut, ["vocabulary", "lexicon"], ["const cv::String*", "std::vector<cv::String>*"]), _)]),
pub fn cv_text_createOCRHMMTransitionsTable_const_StringR_vectorLStringGR(vocabulary: *const c_char, lexicon: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// createOCRHMMTransitionsTable(std::string &, std::vector<std::string> &, OutputArray)(OutString, CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:389
// ("cv::text::createOCRHMMTransitionsTable", vec![(pred!(mut, ["vocabulary", "lexicon", "transition_probabilities_table"], ["std::string*", "std::vector<std::string>*", "const cv::_OutputArray*"]), _)]),
pub fn cv_text_createOCRHMMTransitionsTable_stringR_vectorLstringGR_const__OutputArrayR(vocabulary: *mut *mut c_void, lexicon: *mut c_void, transition_probabilities_table: *const c_void, ocvrs_return: *mut Result<()>);
// cv::text::detectRegions(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:366
// ("cv::text::detectRegions", vec![(pred!(mut, ["image", "er_filter1", "er_filter2", "groups_rects"], ["const cv::_InputArray*", "const cv::Ptr<cv::text::ERFilter>*", "const cv::Ptr<cv::text::ERFilter>*", "std::vector<cv::Rect>*"]), _)]),
pub fn cv_text_detectRegions_const__InputArrayR_const_PtrLERFilterGR_const_PtrLERFilterGR_vectorLRectGR(image: *const c_void, er_filter1: *const c_void, er_filter2: *const c_void, groups_rects: *mut c_void, ocvrs_return: *mut Result<()>);
// detectRegions(InputArray, const Ptr<ERFilter> &, const Ptr<ERFilter> &, std::vector<Rect> &, int, const String &, float)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:366
// ("cv::text::detectRegions", vec![(pred!(mut, ["image", "er_filter1", "er_filter2", "groups_rects", "method", "filename", "minProbability"], ["const cv::_InputArray*", "const cv::Ptr<cv::text::ERFilter>*", "const cv::Ptr<cv::text::ERFilter>*", "std::vector<cv::Rect>*", "int", "const cv::String*", "float"]), _)]),
pub fn cv_text_detectRegions_const__InputArrayR_const_PtrLERFilterGR_const_PtrLERFilterGR_vectorLRectGR_int_const_StringR_float(image: *const c_void, er_filter1: *const c_void, er_filter2: *const c_void, groups_rects: *mut c_void, method: i32, filename: *const c_char, min_probability: f32, ocvrs_return: *mut Result<()>);
// detectRegions(InputArray, const Ptr<ERFilter> &, const Ptr<ERFilter> &, std::vector<std::vector<Point>> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:351
// ("cv::text::detectRegions", vec![(pred!(mut, ["image", "er_filter1", "er_filter2", "regions"], ["const cv::_InputArray*", "const cv::Ptr<cv::text::ERFilter>*", "const cv::Ptr<cv::text::ERFilter>*", "std::vector<std::vector<cv::Point>>*"]), _)]),
pub fn cv_text_detectRegions_const__InputArrayR_const_PtrLERFilterGR_const_PtrLERFilterGR_vectorLvectorLPointGGR(image: *const c_void, er_filter1: *const c_void, er_filter2: *const c_void, regions: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::text::erGrouping(InputArray, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:316
// ("cv::text::erGrouping", vec![(pred!(mut, ["img", "channels", "regions", "groups", "groups_rects"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::text::ERStat>>*", "std::vector<std::vector<cv::Vec2i>>*", "std::vector<cv::Rect>*"]), _)]),
pub fn cv_text_erGrouping_const__InputArrayR_const__InputArrayR_vectorLvectorLERStatGGR_vectorLvectorLVec2iGGR_vectorLRectGR(img: *const c_void, channels: *const c_void, regions: *mut c_void, groups: *mut c_void, groups_rects: *mut c_void, ocvrs_return: *mut Result<()>);
// erGrouping(InputArray, InputArrayOfArrays, std::vector<std::vector<ERStat>> &, std::vector<std::vector<Vec2i>> &, std::vector<Rect> &, int, const std::string &, float)(InputArray, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:316
// ("cv::text::erGrouping", vec![(pred!(mut, ["img", "channels", "regions", "groups", "groups_rects", "method", "filename", "minProbablity"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::text::ERStat>>*", "std::vector<std::vector<cv::Vec2i>>*", "std::vector<cv::Rect>*", "int", "const std::string*", "float"]), _)]),
pub fn cv_text_erGrouping_const__InputArrayR_const__InputArrayR_vectorLvectorLERStatGGR_vectorLvectorLVec2iGGR_vectorLRectGR_int_const_stringR_float(img: *const c_void, channels: *const c_void, regions: *mut c_void, groups: *mut c_void, groups_rects: *mut c_void, method: i32, filename: *const c_char, min_probablity: f32, ocvrs_return: *mut Result<()>);
// cv::text::erGrouping(InputArray, InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:324
// ("cv::text::erGrouping", vec![(pred!(mut, ["image", "channel", "regions", "groups_rects"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::Point>>", "std::vector<cv::Rect>*"]), _)]),
pub fn cv_text_erGrouping_const__InputArrayR_const__InputArrayR_vectorLvectorLPointGG_vectorLRectGR(image: *const c_void, channel: *const c_void, regions: *mut c_void, groups_rects: *mut c_void, ocvrs_return: *mut Result<()>);
// erGrouping(InputArray, InputArray, std::vector<std::vector<Point>>, std::vector<Rect> &, int, const String &, float)(InputArray, InputArray, CppPassByVoidPtr, CppPassByVoidPtr, Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:324
// ("cv::text::erGrouping", vec![(pred!(mut, ["image", "channel", "regions", "groups_rects", "method", "filename", "minProbablity"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<std::vector<cv::Point>>", "std::vector<cv::Rect>*", "int", "const cv::String*", "float"]), _)]),
pub fn cv_text_erGrouping_const__InputArrayR_const__InputArrayR_vectorLvectorLPointGG_vectorLRectGR_int_const_StringR_float(image: *const c_void, channel: *const c_void, regions: *mut c_void, groups_rects: *mut c_void, method: i32, filename: *const c_char, min_probablity: f32, ocvrs_return: *mut Result<()>);
// loadClassifierNM1(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:232
// ("cv::text::loadClassifierNM1", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_text_loadClassifierNM1_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// loadClassifierNM2(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:240
// ("cv::text::loadClassifierNM2", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_text_loadClassifierNM2_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// loadOCRBeamSearchClassifierCNN(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:541
// ("cv::text::loadOCRBeamSearchClassifierCNN", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_text_loadOCRBeamSearchClassifierCNN_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// loadOCRHMMClassifierCNN(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:364
// ("cv::text::loadOCRHMMClassifierCNN", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_text_loadOCRHMMClassifierCNN_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// loadOCRHMMClassifierNM(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:351
// ("cv::text::loadOCRHMMClassifierNM", vec![(pred!(mut, ["filename"], ["const cv::String*"]), _)]),
pub fn cv_text_loadOCRHMMClassifierNM_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// loadOCRHMMClassifier(const String &, int)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:373
// ("cv::text::loadOCRHMMClassifier", vec![(pred!(mut, ["filename", "classifier"], ["const cv::String*", "int"]), _)]),
pub fn cv_text_loadOCRHMMClassifier_const_StringR_int(filename: *const c_char, classifier: i32, ocvrs_return: *mut Result<*mut c_void>);
// run(Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int)(TraitClass, OutString, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:96
// ("cv::text::BaseOCR::run", vec![(pred!(mut, ["image", "output_text", "component_rects", "component_texts", "component_confidences", "component_level"], ["cv::Mat*", "std::string*", "std::vector<cv::Rect>*", "std::vector<std::string>*", "std::vector<float>*", "int"]), _)]),
pub fn cv_text_BaseOCR_run_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(instance: *mut c_void, image: *mut c_void, output_text: *mut *mut c_void, component_rects: *mut c_void, component_texts: *mut c_void, component_confidences: *mut c_void, component_level: i32, ocvrs_return: *mut Result<()>);
// cv::text::BaseOCR::run(TraitClass, OutString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:96
// ("cv::text::BaseOCR::run", vec![(pred!(mut, ["image", "output_text"], ["cv::Mat*", "std::string*"]), _)]),
pub fn cv_text_BaseOCR_run_MatR_stringR(instance: *mut c_void, image: *mut c_void, output_text: *mut *mut c_void, ocvrs_return: *mut Result<()>);
// run(Mat &, Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int)(TraitClass, TraitClass, OutString, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:99
// ("cv::text::BaseOCR::run", vec![(pred!(mut, ["image", "mask", "output_text", "component_rects", "component_texts", "component_confidences", "component_level"], ["cv::Mat*", "cv::Mat*", "std::string*", "std::vector<cv::Rect>*", "std::vector<std::string>*", "std::vector<float>*", "int"]), _)]),
pub fn cv_text_BaseOCR_run_MatR_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(instance: *mut c_void, image: *mut c_void, mask: *mut c_void, output_text: *mut *mut c_void, component_rects: *mut c_void, component_texts: *mut c_void, component_confidences: *mut c_void, component_level: i32, ocvrs_return: *mut Result<()>);
// cv::text::BaseOCR::run(TraitClass, TraitClass, OutString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:99
// ("cv::text::BaseOCR::run", vec![(pred!(mut, ["image", "mask", "output_text"], ["cv::Mat*", "cv::Mat*", "std::string*"]), _)]),
pub fn cv_text_BaseOCR_run_MatR_MatR_stringR(instance: *mut c_void, image: *mut c_void, mask: *mut c_void, output_text: *mut *mut c_void, ocvrs_return: *mut Result<()>);
// cv::text::BaseOCR::to_OCRBeamSearchDecoder() generated
// ("cv::text::BaseOCR::to_OCRBeamSearchDecoder", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_BaseOCR_to_OCRBeamSearchDecoder(instance: *mut c_void) -> *mut c_void;
// cv::text::BaseOCR::to_OCRHMMDecoder() generated
// ("cv::text::BaseOCR::to_OCRHMMDecoder", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_BaseOCR_to_OCRHMMDecoder(instance: *mut c_void) -> *mut c_void;
// cv::text::BaseOCR::to_OCRHolisticWordRecognizer() generated
// ("cv::text::BaseOCR::to_OCRHolisticWordRecognizer", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_BaseOCR_to_OCRHolisticWordRecognizer(instance: *mut c_void) -> *mut c_void;
// cv::text::BaseOCR::to_OCRTesseract() generated
// ("cv::text::BaseOCR::to_OCRTesseract", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_BaseOCR_to_OCRTesseract(instance: *mut c_void) -> *mut c_void;
// cv::text::BaseOCR::delete() generated
// ("cv::text::BaseOCR::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_BaseOCR_delete(instance: *mut c_void);
// run(InputArray, std::vector<ERStat> &)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:151
// ("cv::text::ERFilter::run", vec![(pred!(mut, ["image", "regions"], ["const cv::_InputArray*", "std::vector<cv::text::ERStat>*"]), _)]),
pub fn cv_text_ERFilter_run_const__InputArrayR_vectorLERStatGR(instance: *mut c_void, image: *const c_void, regions: *mut c_void, ocvrs_return: *mut Result<()>);
// setCallback(const Ptr<ERFilter::Callback> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:155
// ("cv::text::ERFilter::setCallback", vec![(pred!(mut, ["cb"], ["const cv::Ptr<cv::text::ERFilter::Callback>*"]), _)]),
pub fn cv_text_ERFilter_setCallback_const_PtrLCallbackGR(instance: *mut c_void, cb: *const c_void, ocvrs_return: *mut Result<()>);
// setThresholdDelta(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:156
// ("cv::text::ERFilter::setThresholdDelta", vec![(pred!(mut, ["thresholdDelta"], ["int"]), _)]),
pub fn cv_text_ERFilter_setThresholdDelta_int(instance: *mut c_void, threshold_delta: i32, ocvrs_return: *mut Result<()>);
// setMinArea(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:157
// ("cv::text::ERFilter::setMinArea", vec![(pred!(mut, ["minArea"], ["float"]), _)]),
pub fn cv_text_ERFilter_setMinArea_float(instance: *mut c_void, min_area: f32, ocvrs_return: *mut Result<()>);
// setMaxArea(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:158
// ("cv::text::ERFilter::setMaxArea", vec![(pred!(mut, ["maxArea"], ["float"]), _)]),
pub fn cv_text_ERFilter_setMaxArea_float(instance: *mut c_void, max_area: f32, ocvrs_return: *mut Result<()>);
// setMinProbability(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:159
// ("cv::text::ERFilter::setMinProbability", vec![(pred!(mut, ["minProbability"], ["float"]), _)]),
pub fn cv_text_ERFilter_setMinProbability_float(instance: *mut c_void, min_probability: f32, ocvrs_return: *mut Result<()>);
// setMinProbabilityDiff(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:160
// ("cv::text::ERFilter::setMinProbabilityDiff", vec![(pred!(mut, ["minProbabilityDiff"], ["float"]), _)]),
pub fn cv_text_ERFilter_setMinProbabilityDiff_float(instance: *mut c_void, min_probability_diff: f32, ocvrs_return: *mut Result<()>);
// setNonMaxSuppression(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:161
// ("cv::text::ERFilter::setNonMaxSuppression", vec![(pred!(mut, ["nonMaxSuppression"], ["bool"]), _)]),
pub fn cv_text_ERFilter_setNonMaxSuppression_bool(instance: *mut c_void, non_max_suppression: bool, ocvrs_return: *mut Result<()>);
// getNumRejected()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:162
// ("cv::text::ERFilter::getNumRejected", vec![(pred!(const, [], []), _)]),
pub fn cv_text_ERFilter_getNumRejected_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::text::ERFilter::to_Algorithm() generated
// ("cv::text::ERFilter::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_ERFilter_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::text::ERFilter::delete() generated
// ("cv::text::ERFilter::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_ERFilter_delete(instance: *mut c_void);
// eval(const ERStat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:135
// ("cv::text::ERFilter::Callback::eval", vec![(pred!(mut, ["stat"], ["const cv::text::ERStat*"]), _)]),
pub fn cv_text_ERFilter_Callback_eval_const_ERStatR(instance: *mut c_void, stat: *const c_void, ocvrs_return: *mut Result<f64>);
// cv::text::ERFilter::Callback::delete() generated
// ("cv::text::ERFilter::Callback::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_ERFilter_Callback_delete(instance: *mut c_void);
// ERStat(int, int, int, int)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:70
// ("cv::text::ERStat::ERStat", vec![(pred!(mut, ["level", "pixel", "x", "y"], ["int", "int", "int", "int"]), _)]),
pub fn cv_text_ERStat_ERStat_int_int_int_int(level: i32, pixel: i32, x: i32, y: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::text::ERStat::ERStat() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:70
// ("cv::text::ERStat::ERStat", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_ERStat_ERStat(ocvrs_return: *mut Result<*mut c_void>);
// cv::text::ERStat::pixel() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:75
// ("cv::text::ERStat::pixel", vec![(pred!(const, [], []), _)]),
pub fn cv_text_ERStat_propPixel_const(instance: *const c_void) -> i32;
// cv::text::ERStat::setPixel(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:75
// ("cv::text::ERStat::setPixel", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_text_ERStat_propPixel_const_int(instance: *mut c_void, val: i32);
// cv::text::ERStat::level() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:76
// ("cv::text::ERStat::level", vec![(pred!(const, [], []), _)]),
pub fn cv_text_ERStat_propLevel_const(instance: *const c_void) -> i32;
// cv::text::ERStat::setLevel(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:76
// ("cv::text::ERStat::setLevel", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_text_ERStat_propLevel_const_int(instance: *mut c_void, val: i32);
// cv::text::ERStat::area() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:79
// ("cv::text::ERStat::area", vec![(pred!(const, [], []), _)]),
pub fn cv_text_ERStat_propArea_const(instance: *const c_void) -> i32;
// cv::text::ERStat::setArea(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:79
// ("cv::text::ERStat::setArea", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_text_ERStat_propArea_const_int(instance: *mut c_void, val: i32);
// cv::text::ERStat::perimeter() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:80
// ("cv::text::ERStat::perimeter", vec![(pred!(const, [], []), _)]),
pub fn cv_text_ERStat_propPerimeter_const(instance: *const c_void) -> i32;
// cv::text::ERStat::setPerimeter(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:80
// ("cv::text::ERStat::setPerimeter", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_text_ERStat_propPerimeter_const_int(instance: *mut c_void, val: i32);
// cv::text::ERStat::euler() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:81
// ("cv::text::ERStat::euler", vec![(pred!(const, [], []), _)]),
pub fn cv_text_ERStat_propEuler_const(instance: *const c_void) -> i32;
// cv::text::ERStat::setEuler(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:81
// ("cv::text::ERStat::setEuler", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_text_ERStat_propEuler_const_int(instance: *mut c_void, val: i32);
// cv::text::ERStat::rect() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:82
// ("cv::text::ERStat::rect", vec![(pred!(const, [], []), _)]),
pub fn cv_text_ERStat_propRect_const(instance: *const c_void, ocvrs_return: *mut core::Rect);
// cv::text::ERStat::setRect(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:82
// ("cv::text::ERStat::setRect", vec![(pred!(mut, ["val"], ["const cv::Rect"]), _)]),
pub fn cv_text_ERStat_propRect_const_Rect(instance: *mut c_void, val: *const core::Rect);
// cv::text::ERStat::raw_moments() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:83
// ("cv::text::ERStat::raw_moments", vec![(pred!(const, [], []), _)]),
pub fn cv_text_ERStat_propRaw_moments_const(instance: *const c_void) -> *const [f64; 2];
// cv::text::ERStat::raw_momentsMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:83
// ("cv::text::ERStat::raw_momentsMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_ERStat_propRaw_moments(instance: *mut c_void) -> *mut [f64; 2];
// cv::text::ERStat::central_moments() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:84
// ("cv::text::ERStat::central_moments", vec![(pred!(const, [], []), _)]),
pub fn cv_text_ERStat_propCentral_moments_const(instance: *const c_void) -> *const [f64; 3];
// cv::text::ERStat::central_momentsMut() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:84
// ("cv::text::ERStat::central_momentsMut", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_ERStat_propCentral_moments(instance: *mut c_void) -> *mut [f64; 3];
// cv::text::ERStat::med_crossings() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:86
// ("cv::text::ERStat::med_crossings", vec![(pred!(const, [], []), _)]),
pub fn cv_text_ERStat_propMed_crossings_const(instance: *const c_void) -> f32;
// cv::text::ERStat::setMed_crossings(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:86
// ("cv::text::ERStat::setMed_crossings", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_text_ERStat_propMed_crossings_const_float(instance: *mut c_void, val: f32);
// cv::text::ERStat::hole_area_ratio() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:89
// ("cv::text::ERStat::hole_area_ratio", vec![(pred!(const, [], []), _)]),
pub fn cv_text_ERStat_propHole_area_ratio_const(instance: *const c_void) -> f32;
// cv::text::ERStat::setHole_area_ratio(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:89
// ("cv::text::ERStat::setHole_area_ratio", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_text_ERStat_propHole_area_ratio_const_float(instance: *mut c_void, val: f32);
// cv::text::ERStat::convex_hull_ratio() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:90
// ("cv::text::ERStat::convex_hull_ratio", vec![(pred!(const, [], []), _)]),
pub fn cv_text_ERStat_propConvex_hull_ratio_const(instance: *const c_void) -> f32;
// cv::text::ERStat::setConvex_hull_ratio(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:90
// ("cv::text::ERStat::setConvex_hull_ratio", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_text_ERStat_propConvex_hull_ratio_const_float(instance: *mut c_void, val: f32);
// cv::text::ERStat::num_inflexion_points() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:91
// ("cv::text::ERStat::num_inflexion_points", vec![(pred!(const, [], []), _)]),
pub fn cv_text_ERStat_propNum_inflexion_points_const(instance: *const c_void) -> f32;
// cv::text::ERStat::setNum_inflexion_points(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:91
// ("cv::text::ERStat::setNum_inflexion_points", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_text_ERStat_propNum_inflexion_points_const_float(instance: *mut c_void, val: f32);
// cv::text::ERStat::probability() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:100
// ("cv::text::ERStat::probability", vec![(pred!(const, [], []), _)]),
pub fn cv_text_ERStat_propProbability_const(instance: *const c_void) -> f64;
// cv::text::ERStat::setProbability(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:100
// ("cv::text::ERStat::setProbability", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_text_ERStat_propProbability_const_double(instance: *mut c_void, val: f64);
// cv::text::ERStat::parent() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:103
// ("cv::text::ERStat::parent", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_ERStat_propParent(instance: *mut c_void) -> *mut c_void;
// cv::text::ERStat::setParent(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:103
// ("cv::text::ERStat::setParent", vec![(pred!(mut, ["val"], ["cv::text::ERStat*"]), _)]),
pub fn cv_text_ERStat_propParent_ERStatX(instance: *mut c_void, val: *const c_void);
// cv::text::ERStat::child() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:104
// ("cv::text::ERStat::child", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_ERStat_propChild(instance: *mut c_void) -> *mut c_void;
// cv::text::ERStat::setChild(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:104
// ("cv::text::ERStat::setChild", vec![(pred!(mut, ["val"], ["cv::text::ERStat*"]), _)]),
pub fn cv_text_ERStat_propChild_ERStatX(instance: *mut c_void, val: *const c_void);
// cv::text::ERStat::next() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:105
// ("cv::text::ERStat::next", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_ERStat_propNext(instance: *mut c_void) -> *mut c_void;
// cv::text::ERStat::setNext(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:105
// ("cv::text::ERStat::setNext", vec![(pred!(mut, ["val"], ["cv::text::ERStat*"]), _)]),
pub fn cv_text_ERStat_propNext_ERStatX(instance: *mut c_void, val: *const c_void);
// cv::text::ERStat::prev() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:106
// ("cv::text::ERStat::prev", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_ERStat_propPrev(instance: *mut c_void) -> *mut c_void;
// cv::text::ERStat::setPrev(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:106
// ("cv::text::ERStat::setPrev", vec![(pred!(mut, ["val"], ["cv::text::ERStat*"]), _)]),
pub fn cv_text_ERStat_propPrev_ERStatX(instance: *mut c_void, val: *const c_void);
// cv::text::ERStat::local_maxima() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:109
// ("cv::text::ERStat::local_maxima", vec![(pred!(const, [], []), _)]),
pub fn cv_text_ERStat_propLocal_maxima_const(instance: *const c_void) -> bool;
// cv::text::ERStat::setLocal_maxima(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:109
// ("cv::text::ERStat::setLocal_maxima", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_text_ERStat_propLocal_maxima_const_bool(instance: *mut c_void, val: bool);
// cv::text::ERStat::max_probability_ancestor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:110
// ("cv::text::ERStat::max_probability_ancestor", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_ERStat_propMax_probability_ancestor(instance: *mut c_void) -> *mut c_void;
// cv::text::ERStat::setMax_probability_ancestor(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:110
// ("cv::text::ERStat::setMax_probability_ancestor", vec![(pred!(mut, ["val"], ["cv::text::ERStat*"]), _)]),
pub fn cv_text_ERStat_propMax_probability_ancestor_ERStatX(instance: *mut c_void, val: *const c_void);
// cv::text::ERStat::min_probability_ancestor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:111
// ("cv::text::ERStat::min_probability_ancestor", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_ERStat_propMin_probability_ancestor(instance: *mut c_void) -> *mut c_void;
// cv::text::ERStat::setMin_probability_ancestor(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/erfilter.hpp:111
// ("cv::text::ERStat::setMin_probability_ancestor", vec![(pred!(mut, ["val"], ["cv::text::ERStat*"]), _)]),
pub fn cv_text_ERStat_propMin_probability_ancestor_ERStatX(instance: *mut c_void, val: *const c_void);
// cv::text::ERStat::delete() generated
// ("cv::text::ERStat::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_ERStat_delete(instance: *mut c_void);
// run(Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int)(TraitClass, OutString, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:456
// ("cv::text::OCRBeamSearchDecoder::run", vec![(pred!(mut, ["image", "output_text", "component_rects", "component_texts", "component_confidences", "component_level"], ["cv::Mat*", "std::string*", "std::vector<cv::Rect>*", "std::vector<std::string>*", "std::vector<float>*", "int"]), _)]),
pub fn cv_text_OCRBeamSearchDecoder_run_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(instance: *mut c_void, image: *mut c_void, output_text: *mut *mut c_void, component_rects: *mut c_void, component_texts: *mut c_void, component_confidences: *mut c_void, component_level: i32, ocvrs_return: *mut Result<()>);
// cv::text::OCRBeamSearchDecoder::run(TraitClass, OutString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:456
// ("cv::text::OCRBeamSearchDecoder::run", vec![(pred!(mut, ["image", "output_text"], ["cv::Mat*", "std::string*"]), _)]),
pub fn cv_text_OCRBeamSearchDecoder_run_MatR_stringR(instance: *mut c_void, image: *mut c_void, output_text: *mut *mut c_void, ocvrs_return: *mut Result<()>);
// run(Mat &, Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int)(TraitClass, TraitClass, OutString, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:460
// ("cv::text::OCRBeamSearchDecoder::run", vec![(pred!(mut, ["image", "mask", "output_text", "component_rects", "component_texts", "component_confidences", "component_level"], ["cv::Mat*", "cv::Mat*", "std::string*", "std::vector<cv::Rect>*", "std::vector<std::string>*", "std::vector<float>*", "int"]), _)]),
pub fn cv_text_OCRBeamSearchDecoder_run_MatR_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(instance: *mut c_void, image: *mut c_void, mask: *mut c_void, output_text: *mut *mut c_void, component_rects: *mut c_void, component_texts: *mut c_void, component_confidences: *mut c_void, component_level: i32, ocvrs_return: *mut Result<()>);
// cv::text::OCRBeamSearchDecoder::run(TraitClass, TraitClass, OutString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:460
// ("cv::text::OCRBeamSearchDecoder::run", vec![(pred!(mut, ["image", "mask", "output_text"], ["cv::Mat*", "cv::Mat*", "std::string*"]), _)]),
pub fn cv_text_OCRBeamSearchDecoder_run_MatR_MatR_stringR(instance: *mut c_void, image: *mut c_void, mask: *mut c_void, output_text: *mut *mut c_void, ocvrs_return: *mut Result<()>);
// run(InputArray, int, int)(InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:465
// ("cv::text::OCRBeamSearchDecoder::run", vec![(pred!(mut, ["image", "min_confidence", "component_level"], ["const cv::_InputArray*", "int", "int"]), _)]),
pub fn cv_text_OCRBeamSearchDecoder_run_const__InputArrayR_int_int(instance: *mut c_void, image: *const c_void, min_confidence: i32, component_level: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::text::OCRBeamSearchDecoder::run(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:465
// ("cv::text::OCRBeamSearchDecoder::run", vec![(pred!(mut, ["image", "min_confidence"], ["const cv::_InputArray*", "int"]), _)]),
pub fn cv_text_OCRBeamSearchDecoder_run_const__InputArrayR_int(instance: *mut c_void, image: *const c_void, min_confidence: i32, ocvrs_return: *mut Result<*mut c_void>);
// run(InputArray, InputArray, int, int)(InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:467
// ("cv::text::OCRBeamSearchDecoder::run", vec![(pred!(mut, ["image", "mask", "min_confidence", "component_level"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "int"]), _)]),
pub fn cv_text_OCRBeamSearchDecoder_run_const__InputArrayR_const__InputArrayR_int_int(instance: *mut c_void, image: *const c_void, mask: *const c_void, min_confidence: i32, component_level: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::text::OCRBeamSearchDecoder::run(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:467
// ("cv::text::OCRBeamSearchDecoder::run", vec![(pred!(mut, ["image", "mask", "min_confidence"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_text_OCRBeamSearchDecoder_run_const__InputArrayR_const__InputArrayR_int(instance: *mut c_void, image: *const c_void, mask: *const c_void, min_confidence: i32, ocvrs_return: *mut Result<*mut c_void>);
// create(const Ptr<OCRBeamSearchDecoder::ClassifierCallback>, const std::string &, InputArray, InputArray, decoder_mode, int)(CppPassByVoidPtr, InString, InputArray, InputArray, Enum, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:487
// ("cv::text::OCRBeamSearchDecoder::create", vec![(pred!(mut, ["classifier", "vocabulary", "transition_probabilities_table", "emission_probabilities_table", "mode", "beam_size"], ["const cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback>", "const std::string*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::text::decoder_mode", "int"]), _)]),
pub fn cv_text_OCRBeamSearchDecoder_create_const_PtrLClassifierCallbackG_const_stringR_const__InputArrayR_const__InputArrayR_decoder_mode_int(classifier: *const c_void, vocabulary: *const c_char, transition_probabilities_table: *const c_void, emission_probabilities_table: *const c_void, mode: crate::text::decoder_mode, beam_size: i32, ocvrs_return: *mut Result<*mut c_void>);
// create(const Ptr<OCRBeamSearchDecoder::ClassifierCallback>, const String &, InputArray, InputArray, int, int)(CppPassByVoidPtr, InString, InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:497
// ("cv::text::OCRBeamSearchDecoder::create", vec![(pred!(mut, ["classifier", "vocabulary", "transition_probabilities_table", "emission_probabilities_table", "mode", "beam_size"], ["const cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback>", "const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int"]), _)]),
pub fn cv_text_OCRBeamSearchDecoder_create_const_PtrLClassifierCallbackG_const_StringR_const__InputArrayR_const__InputArrayR_int_int(classifier: *const c_void, vocabulary: *const c_char, transition_probabilities_table: *const c_void, emission_probabilities_table: *const c_void, mode: i32, beam_size: i32, ocvrs_return: *mut Result<*mut c_void>);
// create(const String &, const String &, InputArray, InputArray, int, int)(InString, InString, InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:512
// ("cv::text::OCRBeamSearchDecoder::create", vec![(pred!(mut, ["filename", "vocabulary", "transition_probabilities_table", "emission_probabilities_table", "mode", "beam_size"], ["const cv::String*", "const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int"]), _)]),
pub fn cv_text_OCRBeamSearchDecoder_create_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR_int_int(filename: *const c_char, vocabulary: *const c_char, transition_probabilities_table: *const c_void, emission_probabilities_table: *const c_void, mode: i32, beam_size: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::text::OCRBeamSearchDecoder::create(InString, InString, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:512
// ("cv::text::OCRBeamSearchDecoder::create", vec![(pred!(mut, ["filename", "vocabulary", "transition_probabilities_table", "emission_probabilities_table"], ["const cv::String*", "const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_text_OCRBeamSearchDecoder_create_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR(filename: *const c_char, vocabulary: *const c_char, transition_probabilities_table: *const c_void, emission_probabilities_table: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::text::OCRBeamSearchDecoder::defaultNew() generated
// ("cv::text::OCRBeamSearchDecoder::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_text_OCRBeamSearchDecoder_defaultNew_const() -> *mut c_void;
// cv::text::OCRBeamSearchDecoder::to_BaseOCR() generated
// ("cv::text::OCRBeamSearchDecoder::to_BaseOCR", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_OCRBeamSearchDecoder_to_BaseOCR(instance: *mut c_void) -> *mut c_void;
// cv::text::OCRBeamSearchDecoder::delete() generated
// ("cv::text::OCRBeamSearchDecoder::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_OCRBeamSearchDecoder_delete(instance: *mut c_void);
// eval(InputArray, std::vector<std::vector<double>> &, std::vector<int> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:428
// ("cv::text::OCRBeamSearchDecoder::ClassifierCallback::eval", vec![(pred!(mut, ["image", "recognition_probabilities", "oversegmentation"], ["const cv::_InputArray*", "std::vector<std::vector<double>>*", "std::vector<int>*"]), _)]),
pub fn cv_text_OCRBeamSearchDecoder_ClassifierCallback_eval_const__InputArrayR_vectorLvectorLdoubleGGR_vectorLintGR(instance: *mut c_void, image: *const c_void, recognition_probabilities: *mut c_void, oversegmentation: *mut c_void, ocvrs_return: *mut Result<()>);
// getWindowSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:430
// ("cv::text::OCRBeamSearchDecoder::ClassifierCallback::getWindowSize", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_OCRBeamSearchDecoder_ClassifierCallback_getWindowSize(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// getStepSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:431
// ("cv::text::OCRBeamSearchDecoder::ClassifierCallback::getStepSize", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_OCRBeamSearchDecoder_ClassifierCallback_getStepSize(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
// cv::text::OCRBeamSearchDecoder::ClassifierCallback::defaultNew() generated
// ("cv::text::OCRBeamSearchDecoder::ClassifierCallback::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_text_OCRBeamSearchDecoder_ClassifierCallback_defaultNew_const() -> *mut c_void;
// cv::text::OCRBeamSearchDecoder::ClassifierCallback::delete() generated
// ("cv::text::OCRBeamSearchDecoder::ClassifierCallback::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_OCRBeamSearchDecoder_ClassifierCallback_delete(instance: *mut c_void);
// run(Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int)(TraitClass, OutString, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:243
// ("cv::text::OCRHMMDecoder::run", vec![(pred!(mut, ["image", "output_text", "component_rects", "component_texts", "component_confidences", "component_level"], ["cv::Mat*", "std::string*", "std::vector<cv::Rect>*", "std::vector<std::string>*", "std::vector<float>*", "int"]), _)]),
pub fn cv_text_OCRHMMDecoder_run_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(instance: *mut c_void, image: *mut c_void, output_text: *mut *mut c_void, component_rects: *mut c_void, component_texts: *mut c_void, component_confidences: *mut c_void, component_level: i32, ocvrs_return: *mut Result<()>);
// cv::text::OCRHMMDecoder::run(TraitClass, OutString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:243
// ("cv::text::OCRHMMDecoder::run", vec![(pred!(mut, ["image", "output_text"], ["cv::Mat*", "std::string*"]), _)]),
pub fn cv_text_OCRHMMDecoder_run_MatR_stringR(instance: *mut c_void, image: *mut c_void, output_text: *mut *mut c_void, ocvrs_return: *mut Result<()>);
// run(Mat &, Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int)(TraitClass, TraitClass, OutString, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:270
// ("cv::text::OCRHMMDecoder::run", vec![(pred!(mut, ["image", "mask", "output_text", "component_rects", "component_texts", "component_confidences", "component_level"], ["cv::Mat*", "cv::Mat*", "std::string*", "std::vector<cv::Rect>*", "std::vector<std::string>*", "std::vector<float>*", "int"]), _)]),
pub fn cv_text_OCRHMMDecoder_run_MatR_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(instance: *mut c_void, image: *mut c_void, mask: *mut c_void, output_text: *mut *mut c_void, component_rects: *mut c_void, component_texts: *mut c_void, component_confidences: *mut c_void, component_level: i32, ocvrs_return: *mut Result<()>);
// cv::text::OCRHMMDecoder::run(TraitClass, TraitClass, OutString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:270
// ("cv::text::OCRHMMDecoder::run", vec![(pred!(mut, ["image", "mask", "output_text"], ["cv::Mat*", "cv::Mat*", "std::string*"]), _)]),
pub fn cv_text_OCRHMMDecoder_run_MatR_MatR_stringR(instance: *mut c_void, image: *mut c_void, mask: *mut c_void, output_text: *mut *mut c_void, ocvrs_return: *mut Result<()>);
// run(InputArray, int, int)(InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:275
// ("cv::text::OCRHMMDecoder::run", vec![(pred!(mut, ["image", "min_confidence", "component_level"], ["const cv::_InputArray*", "int", "int"]), _)]),
pub fn cv_text_OCRHMMDecoder_run_const__InputArrayR_int_int(instance: *mut c_void, image: *const c_void, min_confidence: i32, component_level: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::text::OCRHMMDecoder::run(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:275
// ("cv::text::OCRHMMDecoder::run", vec![(pred!(mut, ["image", "min_confidence"], ["const cv::_InputArray*", "int"]), _)]),
pub fn cv_text_OCRHMMDecoder_run_const__InputArrayR_int(instance: *mut c_void, image: *const c_void, min_confidence: i32, ocvrs_return: *mut Result<*mut c_void>);
// run(InputArray, InputArray, int, int)(InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:277
// ("cv::text::OCRHMMDecoder::run", vec![(pred!(mut, ["image", "mask", "min_confidence", "component_level"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "int"]), _)]),
pub fn cv_text_OCRHMMDecoder_run_const__InputArrayR_const__InputArrayR_int_int(instance: *mut c_void, image: *const c_void, mask: *const c_void, min_confidence: i32, component_level: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::text::OCRHMMDecoder::run(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:277
// ("cv::text::OCRHMMDecoder::run", vec![(pred!(mut, ["image", "mask", "min_confidence"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_text_OCRHMMDecoder_run_const__InputArrayR_const__InputArrayR_int(instance: *mut c_void, image: *const c_void, mask: *const c_void, min_confidence: i32, ocvrs_return: *mut Result<*mut c_void>);
// create(const Ptr<OCRHMMDecoder::ClassifierCallback>, const std::string &, InputArray, InputArray, decoder_mode)(CppPassByVoidPtr, InString, InputArray, InputArray, Enum) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:295
// ("cv::text::OCRHMMDecoder::create", vec![(pred!(mut, ["classifier", "vocabulary", "transition_probabilities_table", "emission_probabilities_table", "mode"], ["const cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>", "const std::string*", "const cv::_InputArray*", "const cv::_InputArray*", "cv::text::decoder_mode"]), _)]),
pub fn cv_text_OCRHMMDecoder_create_const_PtrLClassifierCallbackG_const_stringR_const__InputArrayR_const__InputArrayR_decoder_mode(classifier: *const c_void, vocabulary: *const c_char, transition_probabilities_table: *const c_void, emission_probabilities_table: *const c_void, mode: crate::text::decoder_mode, ocvrs_return: *mut Result<*mut c_void>);
// cv::text::OCRHMMDecoder::create(CppPassByVoidPtr, InString, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:295
// ("cv::text::OCRHMMDecoder::create", vec![(pred!(mut, ["classifier", "vocabulary", "transition_probabilities_table", "emission_probabilities_table"], ["const cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>", "const std::string*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_text_OCRHMMDecoder_create_const_PtrLClassifierCallbackG_const_stringR_const__InputArrayR_const__InputArrayR(classifier: *const c_void, vocabulary: *const c_char, transition_probabilities_table: *const c_void, emission_probabilities_table: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(const Ptr<OCRHMMDecoder::ClassifierCallback>, const String &, InputArray, InputArray, int)(CppPassByVoidPtr, InString, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:304
// ("cv::text::OCRHMMDecoder::create", vec![(pred!(mut, ["classifier", "vocabulary", "transition_probabilities_table", "emission_probabilities_table", "mode"], ["const cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>", "const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_text_OCRHMMDecoder_create_const_PtrLClassifierCallbackG_const_StringR_const__InputArrayR_const__InputArrayR_int(classifier: *const c_void, vocabulary: *const c_char, transition_probabilities_table: *const c_void, emission_probabilities_table: *const c_void, mode: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::text::OCRHMMDecoder::create(CppPassByVoidPtr, InString, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:304
// ("cv::text::OCRHMMDecoder::create", vec![(pred!(mut, ["classifier", "vocabulary", "transition_probabilities_table", "emission_probabilities_table"], ["const cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>", "const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_text_OCRHMMDecoder_create_const_PtrLClassifierCallbackG_const_StringR_const__InputArrayR_const__InputArrayR(classifier: *const c_void, vocabulary: *const c_char, transition_probabilities_table: *const c_void, emission_probabilities_table: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(const String &, const String &, InputArray, InputArray, int, int)(InString, InString, InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:317
// ("cv::text::OCRHMMDecoder::create", vec![(pred!(mut, ["filename", "vocabulary", "transition_probabilities_table", "emission_probabilities_table", "mode", "classifier"], ["const cv::String*", "const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*", "int", "int"]), _)]),
pub fn cv_text_OCRHMMDecoder_create_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR_int_int(filename: *const c_char, vocabulary: *const c_char, transition_probabilities_table: *const c_void, emission_probabilities_table: *const c_void, mode: i32, classifier: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::text::OCRHMMDecoder::create(InString, InString, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:317
// ("cv::text::OCRHMMDecoder::create", vec![(pred!(mut, ["filename", "vocabulary", "transition_probabilities_table", "emission_probabilities_table"], ["const cv::String*", "const cv::String*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_text_OCRHMMDecoder_create_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR(filename: *const c_char, vocabulary: *const c_char, transition_probabilities_table: *const c_void, emission_probabilities_table: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::text::OCRHMMDecoder::defaultNew() generated
// ("cv::text::OCRHMMDecoder::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_text_OCRHMMDecoder_defaultNew_const() -> *mut c_void;
// cv::text::OCRHMMDecoder::to_BaseOCR() generated
// ("cv::text::OCRHMMDecoder::to_BaseOCR", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_OCRHMMDecoder_to_BaseOCR(instance: *mut c_void) -> *mut c_void;
// cv::text::OCRHMMDecoder::delete() generated
// ("cv::text::OCRHMMDecoder::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_OCRHMMDecoder_delete(instance: *mut c_void);
// eval(InputArray, std::vector<int> &, std::vector<double> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:218
// ("cv::text::OCRHMMDecoder::ClassifierCallback::eval", vec![(pred!(mut, ["image", "out_class", "out_confidence"], ["const cv::_InputArray*", "std::vector<int>*", "std::vector<double>*"]), _)]),
pub fn cv_text_OCRHMMDecoder_ClassifierCallback_eval_const__InputArrayR_vectorLintGR_vectorLdoubleGR(instance: *mut c_void, image: *const c_void, out_class: *mut c_void, out_confidence: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::text::OCRHMMDecoder::ClassifierCallback::defaultNew() generated
// ("cv::text::OCRHMMDecoder::ClassifierCallback::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_text_OCRHMMDecoder_ClassifierCallback_defaultNew_const() -> *mut c_void;
// cv::text::OCRHMMDecoder::ClassifierCallback::delete() generated
// ("cv::text::OCRHMMDecoder::ClassifierCallback::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_OCRHMMDecoder_ClassifierCallback_delete(instance: *mut c_void);
// run(Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int)(TraitClass, OutString, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:555
// ("cv::text::OCRHolisticWordRecognizer::run", vec![(pred!(mut, ["image", "output_text", "component_rects", "component_texts", "component_confidences", "component_level"], ["cv::Mat*", "std::string*", "std::vector<cv::Rect>*", "std::vector<std::string>*", "std::vector<float>*", "int"]), _)]),
pub fn cv_text_OCRHolisticWordRecognizer_run_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(instance: *mut c_void, image: *mut c_void, output_text: *mut *mut c_void, component_rects: *mut c_void, component_texts: *mut c_void, component_confidences: *mut c_void, component_level: i32, ocvrs_return: *mut Result<()>);
// cv::text::OCRHolisticWordRecognizer::run(TraitClass, OutString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:555
// ("cv::text::OCRHolisticWordRecognizer::run", vec![(pred!(mut, ["image", "output_text"], ["cv::Mat*", "std::string*"]), _)]),
pub fn cv_text_OCRHolisticWordRecognizer_run_MatR_stringR(instance: *mut c_void, image: *mut c_void, output_text: *mut *mut c_void, ocvrs_return: *mut Result<()>);
// run(Mat &, Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int)(TraitClass, TraitClass, OutString, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:585
// ("cv::text::OCRHolisticWordRecognizer::run", vec![(pred!(mut, ["image", "mask", "output_text", "component_rects", "component_texts", "component_confidences", "component_level"], ["cv::Mat*", "cv::Mat*", "std::string*", "std::vector<cv::Rect>*", "std::vector<std::string>*", "std::vector<float>*", "int"]), _)]),
pub fn cv_text_OCRHolisticWordRecognizer_run_MatR_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(instance: *mut c_void, image: *mut c_void, mask: *mut c_void, output_text: *mut *mut c_void, component_rects: *mut c_void, component_texts: *mut c_void, component_confidences: *mut c_void, component_level: i32, ocvrs_return: *mut Result<()>);
// cv::text::OCRHolisticWordRecognizer::run(TraitClass, TraitClass, OutString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:585
// ("cv::text::OCRHolisticWordRecognizer::run", vec![(pred!(mut, ["image", "mask", "output_text"], ["cv::Mat*", "cv::Mat*", "std::string*"]), _)]),
pub fn cv_text_OCRHolisticWordRecognizer_run_MatR_MatR_stringR(instance: *mut c_void, image: *mut c_void, mask: *mut c_void, output_text: *mut *mut c_void, ocvrs_return: *mut Result<()>);
// create(const std::string &, const std::string &, const std::string &)(InString, InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:595
// ("cv::text::OCRHolisticWordRecognizer::create", vec![(pred!(mut, ["archFilename", "weightsFilename", "wordsFilename"], ["const std::string*", "const std::string*", "const std::string*"]), _)]),
pub fn cv_text_OCRHolisticWordRecognizer_create_const_stringR_const_stringR_const_stringR(arch_filename: *const c_char, weights_filename: *const c_char, words_filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::text::OCRHolisticWordRecognizer::to_BaseOCR() generated
// ("cv::text::OCRHolisticWordRecognizer::to_BaseOCR", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_OCRHolisticWordRecognizer_to_BaseOCR(instance: *mut c_void) -> *mut c_void;
// cv::text::OCRHolisticWordRecognizer::delete() generated
// ("cv::text::OCRHolisticWordRecognizer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_OCRHolisticWordRecognizer_delete(instance: *mut c_void);
// run(Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int)(TraitClass, OutString, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:135
// ("cv::text::OCRTesseract::run", vec![(pred!(mut, ["image", "output_text", "component_rects", "component_texts", "component_confidences", "component_level"], ["cv::Mat*", "std::string*", "std::vector<cv::Rect>*", "std::vector<std::string>*", "std::vector<float>*", "int"]), _)]),
pub fn cv_text_OCRTesseract_run_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(instance: *mut c_void, image: *mut c_void, output_text: *mut *mut c_void, component_rects: *mut c_void, component_texts: *mut c_void, component_confidences: *mut c_void, component_level: i32, ocvrs_return: *mut Result<()>);
// cv::text::OCRTesseract::run(TraitClass, OutString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:135
// ("cv::text::OCRTesseract::run", vec![(pred!(mut, ["image", "output_text"], ["cv::Mat*", "std::string*"]), _)]),
pub fn cv_text_OCRTesseract_run_MatR_stringR(instance: *mut c_void, image: *mut c_void, output_text: *mut *mut c_void, ocvrs_return: *mut Result<()>);
// run(Mat &, Mat &, std::string &, std::vector<Rect> *, std::vector<std::string> *, std::vector<float> *, int)(TraitClass, TraitClass, OutString, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:139
// ("cv::text::OCRTesseract::run", vec![(pred!(mut, ["image", "mask", "output_text", "component_rects", "component_texts", "component_confidences", "component_level"], ["cv::Mat*", "cv::Mat*", "std::string*", "std::vector<cv::Rect>*", "std::vector<std::string>*", "std::vector<float>*", "int"]), _)]),
pub fn cv_text_OCRTesseract_run_MatR_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(instance: *mut c_void, image: *mut c_void, mask: *mut c_void, output_text: *mut *mut c_void, component_rects: *mut c_void, component_texts: *mut c_void, component_confidences: *mut c_void, component_level: i32, ocvrs_return: *mut Result<()>);
// cv::text::OCRTesseract::run(TraitClass, TraitClass, OutString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:139
// ("cv::text::OCRTesseract::run", vec![(pred!(mut, ["image", "mask", "output_text"], ["cv::Mat*", "cv::Mat*", "std::string*"]), _)]),
pub fn cv_text_OCRTesseract_run_MatR_MatR_stringR(instance: *mut c_void, image: *mut c_void, mask: *mut c_void, output_text: *mut *mut c_void, ocvrs_return: *mut Result<()>);
// run(InputArray, int, int)(InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:144
// ("cv::text::OCRTesseract::run", vec![(pred!(mut, ["image", "min_confidence", "component_level"], ["const cv::_InputArray*", "int", "int"]), _)]),
pub fn cv_text_OCRTesseract_run_const__InputArrayR_int_int(instance: *mut c_void, image: *const c_void, min_confidence: i32, component_level: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::text::OCRTesseract::run(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:144
// ("cv::text::OCRTesseract::run", vec![(pred!(mut, ["image", "min_confidence"], ["const cv::_InputArray*", "int"]), _)]),
pub fn cv_text_OCRTesseract_run_const__InputArrayR_int(instance: *mut c_void, image: *const c_void, min_confidence: i32, ocvrs_return: *mut Result<*mut c_void>);
// run(InputArray, InputArray, int, int)(InputArray, InputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:146
// ("cv::text::OCRTesseract::run", vec![(pred!(mut, ["image", "mask", "min_confidence", "component_level"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "int"]), _)]),
pub fn cv_text_OCRTesseract_run_const__InputArrayR_const__InputArrayR_int_int(instance: *mut c_void, image: *const c_void, mask: *const c_void, min_confidence: i32, component_level: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::text::OCRTesseract::run(InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:146
// ("cv::text::OCRTesseract::run", vec![(pred!(mut, ["image", "mask", "min_confidence"], ["const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_text_OCRTesseract_run_const__InputArrayR_const__InputArrayR_int(instance: *mut c_void, image: *const c_void, mask: *const c_void, min_confidence: i32, ocvrs_return: *mut Result<*mut c_void>);
// setWhiteList(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:148
// ("cv::text::OCRTesseract::setWhiteList", vec![(pred!(mut, ["char_whitelist"], ["const cv::String*"]), _)]),
pub fn cv_text_OCRTesseract_setWhiteList_const_StringR(instance: *mut c_void, char_whitelist: *const c_char, ocvrs_return: *mut Result<()>);
// create(const char *, const char *, const char *, int, int)(InString, InString, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:167
// ("cv::text::OCRTesseract::create", vec![(pred!(mut, ["datapath", "language", "char_whitelist", "oem", "psmode"], ["const char*", "const char*", "const char*", "int", "int"]), _)]),
pub fn cv_text_OCRTesseract_create_const_charX_const_charX_const_charX_int_int(datapath: *const c_char, language: *const c_char, char_whitelist: *const c_char, oem: i32, psmode: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::text::OCRTesseract::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/ocr.hpp:167
// ("cv::text::OCRTesseract::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_OCRTesseract_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::text::OCRTesseract::to_BaseOCR() generated
// ("cv::text::OCRTesseract::to_BaseOCR", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_OCRTesseract_to_BaseOCR(instance: *mut c_void) -> *mut c_void;
// cv::text::OCRTesseract::delete() generated
// ("cv::text::OCRTesseract::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_OCRTesseract_delete(instance: *mut c_void);
// detect(InputArray, std::vector<Rect> &, std::vector<float> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/textDetector.hpp:30
// ("cv::text::TextDetector::detect", vec![(pred!(mut, ["inputImage", "Bbox", "confidence"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<float>*"]), _)]),
pub fn cv_text_TextDetector_detect_const__InputArrayR_vectorLRectGR_vectorLfloatGR(instance: *mut c_void, input_image: *const c_void, bbox: *mut c_void, confidence: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::text::TextDetector::to_TextDetectorCNN() generated
// ("cv::text::TextDetector::to_TextDetectorCNN", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_TextDetector_to_TextDetectorCNN(instance: *mut c_void) -> *mut c_void;
// cv::text::TextDetector::delete() generated
// ("cv::text::TextDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_TextDetector_delete(instance: *mut c_void);
// detect(InputArray, std::vector<Rect> &, std::vector<float> &)(InputArray, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/textDetector.hpp:51
// ("cv::text::TextDetectorCNN::detect", vec![(pred!(mut, ["inputImage", "Bbox", "confidence"], ["const cv::_InputArray*", "std::vector<cv::Rect>*", "std::vector<float>*"]), _)]),
pub fn cv_text_TextDetectorCNN_detect_const__InputArrayR_vectorLRectGR_vectorLfloatGR(instance: *mut c_void, input_image: *const c_void, bbox: *mut c_void, confidence: *mut c_void, ocvrs_return: *mut Result<()>);
// create(const String &, const String &, std::vector<Size>)(InString, InString, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/textDetector.hpp:60
// ("cv::text::TextDetectorCNN::create", vec![(pred!(mut, ["modelArchFilename", "modelWeightsFilename", "detectionSizes"], ["const cv::String*", "const cv::String*", "std::vector<cv::Size>"]), _)]),
pub fn cv_text_TextDetectorCNN_create_const_StringR_const_StringR_vectorLSizeG(model_arch_filename: *const c_char, model_weights_filename: *const c_char, detection_sizes: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// create(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/text/textDetector.hpp:65
// ("cv::text::TextDetectorCNN::create", vec![(pred!(mut, ["modelArchFilename", "modelWeightsFilename"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_text_TextDetectorCNN_create_const_StringR_const_StringR(model_arch_filename: *const c_char, model_weights_filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::text::TextDetectorCNN::to_TextDetector() generated
// ("cv::text::TextDetectorCNN::to_TextDetector", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_TextDetectorCNN_to_TextDetector(instance: *mut c_void) -> *mut c_void;
// cv::text::TextDetectorCNN::delete() generated
// ("cv::text::TextDetectorCNN::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_text_TextDetectorCNN_delete(instance: *mut c_void);
