// cv::tld::tld_InitDataset(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tldDataset.hpp:51
// ("cv::tld::tld_InitDataset", vec![(pred!(mut, ["videoInd"], ["int"]), _)]),
pub fn cv_tld_tld_InitDataset_int(video_ind: i32, ocvrs_return: *mut Result<core::Rect2d>);
// tld_InitDataset(int, const char *, int)(Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tldDataset.hpp:51
// ("cv::tld::tld_InitDataset", vec![(pred!(mut, ["videoInd", "rootPath", "datasetInd"], ["int", "const char*", "int"]), _)]),
pub fn cv_tld_tld_InitDataset_int_const_charX_int(video_ind: i32, root_path: *const c_char, dataset_ind: i32, ocvrs_return: *mut Result<core::Rect2d>);
// tld_getNextDatasetFrame()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tldDataset.hpp:52
// ("cv::tld::tld_getNextDatasetFrame", vec![(pred!(mut, [], []), _)]),
pub fn cv_tld_tld_getNextDatasetFrame(ocvrs_return: *mut Result<*mut c_void>);
// ClfMilBoost()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:70
// ("cv::ClfMilBoost::ClfMilBoost", vec![(pred!(mut, [], []), _)]),
pub fn cv_ClfMilBoost_ClfMilBoost(ocvrs_return: *mut Result<*mut c_void>);
// init(const ClfMilBoost::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:72
// ("cv::ClfMilBoost::init", vec![(pred!(mut, ["parameters"], ["const cv::ClfMilBoost::Params*"]), _)]),
pub fn cv_ClfMilBoost_init_const_ParamsR(instance: *mut c_void, parameters: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ClfMilBoost::init() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:72
// ("cv::ClfMilBoost::init", vec![(pred!(mut, [], []), _)]),
pub fn cv_ClfMilBoost_init(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// update(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:73
// ("cv::ClfMilBoost::update", vec![(pred!(mut, ["posx", "negx"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_ClfMilBoost_update_const_MatR_const_MatR(instance: *mut c_void, posx: *const c_void, negx: *const c_void, ocvrs_return: *mut Result<()>);
// classify(const Mat &, bool)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:74
// ("cv::ClfMilBoost::classify", vec![(pred!(mut, ["x", "logR"], ["const cv::Mat*", "bool"]), _)]),
pub fn cv_ClfMilBoost_classify_const_MatR_bool(instance: *mut c_void, x: *const c_void, log_r: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::ClfMilBoost::classify(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:74
// ("cv::ClfMilBoost::classify", vec![(pred!(mut, ["x"], ["const cv::Mat*"]), _)]),
pub fn cv_ClfMilBoost_classify_const_MatR(instance: *mut c_void, x: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// sigmoid(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:76
// ("cv::ClfMilBoost::sigmoid", vec![(pred!(mut, ["x"], ["float"]), _)]),
pub fn cv_ClfMilBoost_sigmoid_float(instance: *mut c_void, x: f32, ocvrs_return: *mut Result<f32>);
// cv::ClfMilBoost::delete() generated
// ("cv::ClfMilBoost::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ClfMilBoost_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:64
// ("cv::ClfMilBoost::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_ClfMilBoost_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// cv::ClfMilBoost::Params::_numSel() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:65
// ("cv::ClfMilBoost::Params::_numSel", vec![(pred!(const, [], []), _)]),
pub fn cv_ClfMilBoost_Params_prop_numSel_const(instance: *const c_void) -> i32;
// cv::ClfMilBoost::Params::set_numSel(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:65
// ("cv::ClfMilBoost::Params::set_numSel", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_ClfMilBoost_Params_prop_numSel_const_int(instance: *mut c_void, val: i32);
// cv::ClfMilBoost::Params::_numFeat() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:66
// ("cv::ClfMilBoost::Params::_numFeat", vec![(pred!(const, [], []), _)]),
pub fn cv_ClfMilBoost_Params_prop_numFeat_const(instance: *const c_void) -> i32;
// cv::ClfMilBoost::Params::set_numFeat(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:66
// ("cv::ClfMilBoost::Params::set_numFeat", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_ClfMilBoost_Params_prop_numFeat_const_int(instance: *mut c_void, val: i32);
// cv::ClfMilBoost::Params::_lRate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:67
// ("cv::ClfMilBoost::Params::_lRate", vec![(pred!(const, [], []), _)]),
pub fn cv_ClfMilBoost_Params_prop_lRate_const(instance: *const c_void) -> f32;
// cv::ClfMilBoost::Params::set_lRate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:67
// ("cv::ClfMilBoost::Params::set_lRate", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_ClfMilBoost_Params_prop_lRate_const_float(instance: *mut c_void, val: f32);
// cv::ClfMilBoost::Params::delete() generated
// ("cv::ClfMilBoost::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ClfMilBoost_Params_delete(instance: *mut c_void);
// init(const CvFeatureParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:147
// ("cv::CvFeatureParams::init", vec![(pred!(mut, ["fp"], ["const cv::CvFeatureParams*"]), _)]),
pub fn cv_CvFeatureParams_init_const_CvFeatureParamsR(instance: *mut c_void, fp: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:148
// ("cv::CvFeatureParams::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_CvFeatureParams_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:149
// ("cv::CvFeatureParams::read", vec![(pred!(mut, ["node"], ["const cv::FileNode*"]), _)]),
pub fn cv_CvFeatureParams_read_const_FileNodeR(instance: *mut c_void, node: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::CvFeatureParams::maxCatCount() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:151
// ("cv::CvFeatureParams::maxCatCount", vec![(pred!(const, [], []), _)]),
pub fn cv_CvFeatureParams_propMaxCatCount_const(instance: *const c_void) -> i32;
// cv::CvFeatureParams::setMaxCatCount(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:151
// ("cv::CvFeatureParams::setMaxCatCount", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_CvFeatureParams_propMaxCatCount_const_int(instance: *mut c_void, val: i32);
// cv::CvFeatureParams::featSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:152
// ("cv::CvFeatureParams::featSize", vec![(pred!(const, [], []), _)]),
pub fn cv_CvFeatureParams_propFeatSize_const(instance: *const c_void) -> i32;
// cv::CvFeatureParams::setFeatSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:152
// ("cv::CvFeatureParams::setFeatSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_CvFeatureParams_propFeatSize_const_int(instance: *mut c_void, val: i32);
// cv::CvFeatureParams::numFeatures() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:153
// ("cv::CvFeatureParams::numFeatures", vec![(pred!(const, [], []), _)]),
pub fn cv_CvFeatureParams_propNumFeatures_const(instance: *const c_void) -> i32;
// cv::CvFeatureParams::setNumFeatures(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:153
// ("cv::CvFeatureParams::setNumFeatures", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_CvFeatureParams_propNumFeatures_const_int(instance: *mut c_void, val: i32);
// cv::CvFeatureParams::delete() generated
// ("cv::CvFeatureParams::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_CvFeatureParams_delete(instance: *mut c_void);
// init(const CvFeatureParams *, int, Size)(TraitClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:254
// ("cv::CvHaarEvaluator::init", vec![(pred!(mut, ["_featureParams", "_maxSampleCount", "_winSize"], ["const cv::CvFeatureParams*", "int", "cv::Size"]), _)]),
pub fn cv_CvHaarEvaluator_init_const_CvFeatureParamsX_int_Size(instance: *mut c_void, _feature_params: *const c_void, _max_sample_count: i32, _win_size: *const core::Size, ocvrs_return: *mut Result<()>);
// setImage(const Mat &, uchar, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:255
// ("cv::CvHaarEvaluator::setImage", vec![(pred!(mut, ["img", "clsLabel", "idx"], ["const cv::Mat*", "unsigned char", "int"]), _)]),
pub fn cv_CvHaarEvaluator_setImage_const_MatR_unsigned_char_int(instance: *mut c_void, img: *const c_void, cls_label: u8, idx: i32, ocvrs_return: *mut Result<()>);
// cv::CvHaarEvaluator::setImage(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:255
// ("cv::CvHaarEvaluator::setImage", vec![(pred!(mut, ["img"], ["const cv::Mat*"]), _)]),
pub fn cv_CvHaarEvaluator_setImage_const_MatR(instance: *mut c_void, img: *const c_void, ocvrs_return: *mut Result<()>);
// operator()(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:256
// ("cv::CvHaarEvaluator::operator()", vec![(pred!(mut, ["featureIdx", "sampleIdx"], ["int", "int"]), _)]),
pub fn cv_CvHaarEvaluator_operator___int_int(instance: *mut c_void, feature_idx: i32, sample_idx: i32, ocvrs_return: *mut Result<f32>);
// writeFeatures(FileStorage &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:257
// ("cv::CvHaarEvaluator::writeFeatures", vec![(pred!(const, ["fs", "featureMap"], ["cv::FileStorage*", "const cv::Mat*"]), _)]),
pub fn cv_CvHaarEvaluator_writeFeatures_const_FileStorageR_const_MatR(instance: *const c_void, fs: *mut c_void, feature_map: *const c_void, ocvrs_return: *mut Result<()>);
// getFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:260
// ("cv::CvHaarEvaluator::getFeatures", vec![(pred!(mut, ["idx"], ["int"]), _)]),
pub fn cv_CvHaarEvaluator_getFeatures_int(instance: *mut c_void, idx: i32, ocvrs_return: *mut Result<*mut c_void>);
// generateFeatures()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:266
// ("cv::CvHaarEvaluator::generateFeatures", vec![(pred!(mut, [], []), _)]),
pub fn cv_CvHaarEvaluator_generateFeatures(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// generateFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:274
// ("cv::CvHaarEvaluator::generateFeatures", vec![(pred!(mut, ["numFeatures"], ["int"]), _)]),
pub fn cv_CvHaarEvaluator_generateFeatures_int(instance: *mut c_void, num_features: i32, ocvrs_return: *mut Result<()>);
// cv::CvHaarEvaluator::defaultNew() generated
// ("cv::CvHaarEvaluator::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_CvHaarEvaluator_defaultNew_const() -> *mut c_void;
// cv::CvHaarEvaluator::delete() generated
// ("cv::CvHaarEvaluator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_CvHaarEvaluator_delete(instance: *mut c_void);
// write(FileStorage)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:229
// ("cv::CvHaarEvaluator::FeatureHaar::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage"]), _)]),
pub fn cv_CvHaarEvaluator_FeatureHaar_write_const_FileStorage(instance: *const c_void, unnamed: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::CvHaarEvaluator::FeatureHaar::delete() generated
// ("cv::CvHaarEvaluator::FeatureHaar::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_CvHaarEvaluator_FeatureHaar_delete(instance: *mut c_void);
// MultiTracker()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1339
// ("cv::MultiTracker::MultiTracker", vec![(pred!(mut, [], []), _)]),
pub fn cv_MultiTracker_MultiTracker(ocvrs_return: *mut Result<*mut c_void>);
// add(Ptr<Tracker>, InputArray, const Rect2d &)(CppPassByVoidPtr, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1353
// ("cv::MultiTracker::add", vec![(pred!(mut, ["newTracker", "image", "boundingBox"], ["cv::Ptr<cv::Tracker>", "const cv::_InputArray*", "const cv::Rect2d*"]), _)]),
pub fn cv_MultiTracker_add_PtrLTrackerG_const__InputArrayR_const_Rect2dR(instance: *mut c_void, new_tracker: *mut c_void, image: *const c_void, bounding_box: *const core::Rect2d, ocvrs_return: *mut Result<bool>);
// add(std::vector<Ptr<Tracker>>, InputArray, std::vector<Rect2d>)(CppPassByVoidPtr, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1361
// ("cv::MultiTracker::add", vec![(pred!(mut, ["newTrackers", "image", "boundingBox"], ["std::vector<cv::Ptr<cv::Tracker>>", "const cv::_InputArray*", "std::vector<cv::Rect2d>"]), _)]),
pub fn cv_MultiTracker_add_vectorLPtrLTrackerGG_const__InputArrayR_vectorLRect2dG(instance: *mut c_void, new_trackers: *mut c_void, image: *const c_void, bounding_box: *mut c_void, ocvrs_return: *mut Result<bool>);
// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1368
// ("cv::MultiTracker::update", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
pub fn cv_MultiTracker_update_const__InputArrayR(instance: *mut c_void, image: *const c_void, ocvrs_return: *mut Result<bool>);
// update(InputArray, std::vector<Rect2d> &)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1375
// ("cv::MultiTracker::update", vec![(pred!(mut, ["image", "boundingBox"], ["const cv::_InputArray*", "std::vector<cv::Rect2d>*"]), _)]),
pub fn cv_MultiTracker_update_const__InputArrayR_vectorLRect2dGR(instance: *mut c_void, image: *const c_void, bounding_box: *mut c_void, ocvrs_return: *mut Result<bool>);
// getObjects()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1380
// ("cv::MultiTracker::getObjects", vec![(pred!(const, [], []), _)]),
pub fn cv_MultiTracker_getObjects_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1385
// ("cv::MultiTracker::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_MultiTracker_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::MultiTracker::to_Algorithm() generated
// ("cv::MultiTracker::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_MultiTracker_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::MultiTracker::delete() generated
// ("cv::MultiTracker::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_MultiTracker_delete(instance: *mut c_void);
// update_opt(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1473
// ("cv::MultiTrackerTLD::update_opt", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
pub fn cv_MultiTrackerTLD_update_opt_const__InputArrayR(instance: *mut c_void, image: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::MultiTrackerTLD::defaultNew() generated
// ("cv::MultiTrackerTLD::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_MultiTrackerTLD_defaultNew_const() -> *mut c_void;
// cv::MultiTrackerTLD::to_MultiTracker_Alt() generated
// ("cv::MultiTrackerTLD::to_MultiTracker_Alt", vec![(pred!(mut, [], []), _)]),
pub fn cv_MultiTrackerTLD_to_MultiTracker_Alt(instance: *mut c_void) -> *mut c_void;
// cv::MultiTrackerTLD::delete() generated
// ("cv::MultiTrackerTLD::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_MultiTrackerTLD_delete(instance: *mut c_void);
// MultiTracker_Alt()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1406
// ("cv::MultiTracker_Alt::MultiTracker_Alt", vec![(pred!(mut, [], []), _)]),
pub fn cv_MultiTracker_Alt_MultiTracker_Alt(ocvrs_return: *mut Result<*mut c_void>);
// addTarget(InputArray, const Rect2d &, Ptr<Tracker>)(InputArray, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1418
// ("cv::MultiTracker_Alt::addTarget", vec![(pred!(mut, ["image", "boundingBox", "tracker_algorithm"], ["const cv::_InputArray*", "const cv::Rect2d*", "cv::Ptr<cv::Tracker>"]), _)]),
pub fn cv_MultiTracker_Alt_addTarget_const__InputArrayR_const_Rect2dR_PtrLTrackerG(instance: *mut c_void, image: *const c_void, bounding_box: *const core::Rect2d, tracker_algorithm: *mut c_void, ocvrs_return: *mut Result<bool>);
// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1427
// ("cv::MultiTracker_Alt::update", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
pub fn cv_MultiTracker_Alt_update_const__InputArrayR(instance: *mut c_void, image: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::MultiTracker_Alt::targetNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1431
// ("cv::MultiTracker_Alt::targetNum", vec![(pred!(const, [], []), _)]),
pub fn cv_MultiTracker_Alt_propTargetNum_const(instance: *const c_void) -> i32;
// cv::MultiTracker_Alt::setTargetNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1431
// ("cv::MultiTracker_Alt::setTargetNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_MultiTracker_Alt_propTargetNum_const_int(instance: *mut c_void, val: i32);
// cv::MultiTracker_Alt::trackers() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1435
// ("cv::MultiTracker_Alt::trackers", vec![(pred!(const, [], []), _)]),
pub fn cv_MultiTracker_Alt_propTrackers_const(instance: *const c_void) -> *mut c_void;
// cv::MultiTracker_Alt::setTrackers(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1435
// ("cv::MultiTracker_Alt::setTrackers", vec![(pred!(mut, ["val"], ["const std::vector<cv::Ptr<cv::Tracker>>"]), _)]),
pub fn cv_MultiTracker_Alt_propTrackers_const_vectorLPtrLTrackerGG(instance: *mut c_void, val: *const c_void);
// cv::MultiTracker_Alt::boundingBoxes() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1439
// ("cv::MultiTracker_Alt::boundingBoxes", vec![(pred!(const, [], []), _)]),
pub fn cv_MultiTracker_Alt_propBoundingBoxes_const(instance: *const c_void) -> *mut c_void;
// cv::MultiTracker_Alt::setBoundingBoxes(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1439
// ("cv::MultiTracker_Alt::setBoundingBoxes", vec![(pred!(mut, ["val"], ["const std::vector<cv::Rect2d>"]), _)]),
pub fn cv_MultiTracker_Alt_propBoundingBoxes_const_vectorLRect2dG(instance: *mut c_void, val: *const c_void);
// cv::MultiTracker_Alt::colors() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1442
// ("cv::MultiTracker_Alt::colors", vec![(pred!(const, [], []), _)]),
pub fn cv_MultiTracker_Alt_propColors_const(instance: *const c_void) -> *mut c_void;
// cv::MultiTracker_Alt::setColors(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1442
// ("cv::MultiTracker_Alt::setColors", vec![(pred!(mut, ["val"], ["const std::vector<cv::Scalar>"]), _)]),
pub fn cv_MultiTracker_Alt_propColors_const_vectorLScalarG(instance: *mut c_void, val: *const c_void);
// cv::MultiTracker_Alt::delete() generated
// ("cv::MultiTracker_Alt::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_MultiTracker_Alt_delete(instance: *mut c_void);
// init(InputArray, const Rect2d &)(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:536
// ("cv::Tracker::init", vec![(pred!(mut, ["image", "boundingBox"], ["const cv::_InputArray*", "const cv::Rect2d*"]), _)]),
pub fn cv_Tracker_init_const__InputArrayR_const_Rect2dR(instance: *mut c_void, image: *const c_void, bounding_box: *const core::Rect2d, ocvrs_return: *mut Result<bool>);
// update(InputArray, Rect2d &)(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:547
// ("cv::Tracker::update", vec![(pred!(mut, ["image", "boundingBox"], ["const cv::_InputArray*", "cv::Rect2d*"]), _)]),
pub fn cv_Tracker_update_const__InputArrayR_Rect2dR(instance: *mut c_void, image: *const c_void, bounding_box: *mut core::Rect2d, ocvrs_return: *mut Result<bool>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:549
// ("cv::Tracker::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_Tracker_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:550
// ("cv::Tracker::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_Tracker_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::Tracker::to_TrackerBoosting() generated
// ("cv::Tracker::to_TrackerBoosting", vec![(pred!(mut, [], []), _)]),
pub fn cv_Tracker_to_TrackerBoosting(instance: *mut c_void) -> *mut c_void;
// cv::Tracker::to_TrackerCSRT() generated
// ("cv::Tracker::to_TrackerCSRT", vec![(pred!(mut, [], []), _)]),
pub fn cv_Tracker_to_TrackerCSRT(instance: *mut c_void) -> *mut c_void;
// cv::Tracker::to_TrackerGOTURN() generated
// ("cv::Tracker::to_TrackerGOTURN", vec![(pred!(mut, [], []), _)]),
pub fn cv_Tracker_to_TrackerGOTURN(instance: *mut c_void) -> *mut c_void;
// cv::Tracker::to_TrackerKCF() generated
// ("cv::Tracker::to_TrackerKCF", vec![(pred!(mut, [], []), _)]),
pub fn cv_Tracker_to_TrackerKCF(instance: *mut c_void) -> *mut c_void;
// cv::Tracker::to_TrackerMIL() generated
// ("cv::Tracker::to_TrackerMIL", vec![(pred!(mut, [], []), _)]),
pub fn cv_Tracker_to_TrackerMIL(instance: *mut c_void) -> *mut c_void;
// cv::Tracker::to_TrackerMOSSE() generated
// ("cv::Tracker::to_TrackerMOSSE", vec![(pred!(mut, [], []), _)]),
pub fn cv_Tracker_to_TrackerMOSSE(instance: *mut c_void) -> *mut c_void;
// cv::Tracker::to_TrackerMedianFlow() generated
// ("cv::Tracker::to_TrackerMedianFlow", vec![(pred!(mut, [], []), _)]),
pub fn cv_Tracker_to_TrackerMedianFlow(instance: *mut c_void) -> *mut c_void;
// cv::Tracker::to_TrackerTLD() generated
// ("cv::Tracker::to_TrackerTLD", vec![(pred!(mut, [], []), _)]),
pub fn cv_Tracker_to_TrackerTLD(instance: *mut c_void) -> *mut c_void;
// cv::Tracker::to_Algorithm() generated
// ("cv::Tracker::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_Tracker_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::Tracker::delete() generated
// ("cv::Tracker::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_Tracker_delete(instance: *mut c_void);
// create(const TrackerBoosting::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1125
// ("cv::TrackerBoosting::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerBoosting::Params*"]), _)]),
pub fn cv_TrackerBoosting_create_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1127
// ("cv::TrackerBoosting::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerBoosting_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerBoosting::to_Algorithm() generated
// ("cv::TrackerBoosting::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerBoosting_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::TrackerBoosting::to_Tracker() generated
// ("cv::TrackerBoosting::to_Tracker", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerBoosting_to_Tracker(instance: *mut c_void) -> *mut c_void;
// cv::TrackerBoosting::delete() generated
// ("cv::TrackerBoosting::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerBoosting_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1105
// ("cv::TrackerBoosting::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerBoosting_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1114
// ("cv::TrackerBoosting::Params::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_TrackerBoosting_Params_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1119
// ("cv::TrackerBoosting::Params::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_TrackerBoosting_Params_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::TrackerBoosting::Params::numClassifiers() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1106
// ("cv::TrackerBoosting::Params::numClassifiers", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerBoosting_Params_propNumClassifiers_const(instance: *const c_void) -> i32;
// cv::TrackerBoosting::Params::setNumClassifiers(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1106
// ("cv::TrackerBoosting::Params::setNumClassifiers", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerBoosting_Params_propNumClassifiers_const_int(instance: *mut c_void, val: i32);
// cv::TrackerBoosting::Params::samplerOverlap() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1107
// ("cv::TrackerBoosting::Params::samplerOverlap", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerBoosting_Params_propSamplerOverlap_const(instance: *const c_void) -> f32;
// cv::TrackerBoosting::Params::setSamplerOverlap(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1107
// ("cv::TrackerBoosting::Params::setSamplerOverlap", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerBoosting_Params_propSamplerOverlap_const_float(instance: *mut c_void, val: f32);
// cv::TrackerBoosting::Params::samplerSearchFactor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1108
// ("cv::TrackerBoosting::Params::samplerSearchFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerBoosting_Params_propSamplerSearchFactor_const(instance: *const c_void) -> f32;
// cv::TrackerBoosting::Params::setSamplerSearchFactor(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1108
// ("cv::TrackerBoosting::Params::setSamplerSearchFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerBoosting_Params_propSamplerSearchFactor_const_float(instance: *mut c_void, val: f32);
// cv::TrackerBoosting::Params::iterationInit() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1109
// ("cv::TrackerBoosting::Params::iterationInit", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerBoosting_Params_propIterationInit_const(instance: *const c_void) -> i32;
// cv::TrackerBoosting::Params::setIterationInit(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1109
// ("cv::TrackerBoosting::Params::setIterationInit", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerBoosting_Params_propIterationInit_const_int(instance: *mut c_void, val: i32);
// cv::TrackerBoosting::Params::featureSetNumFeatures() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1110
// ("cv::TrackerBoosting::Params::featureSetNumFeatures", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerBoosting_Params_propFeatureSetNumFeatures_const(instance: *const c_void) -> i32;
// cv::TrackerBoosting::Params::setFeatureSetNumFeatures(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1110
// ("cv::TrackerBoosting::Params::setFeatureSetNumFeatures", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerBoosting_Params_propFeatureSetNumFeatures_const_int(instance: *mut c_void, val: i32);
// cv::TrackerBoosting::Params::delete() generated
// ("cv::TrackerBoosting::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerBoosting_Params_delete(instance: *mut c_void);
// create(const TrackerCSRT::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1536
// ("cv::TrackerCSRT::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerCSRT::Params*"]), _)]),
pub fn cv_TrackerCSRT_create_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1538
// ("cv::TrackerCSRT::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerCSRT_create(ocvrs_return: *mut Result<*mut c_void>);
// setInitialMask(const Mat)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1540
// ("cv::TrackerCSRT::setInitialMask", vec![(pred!(mut, ["mask"], ["const cv::Mat"]), _)]),
pub fn cv_TrackerCSRT_setInitialMask_const_Mat(instance: *mut c_void, mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::TrackerCSRT::to_Algorithm() generated
// ("cv::TrackerCSRT::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerCSRT_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::TrackerCSRT::to_Tracker() generated
// ("cv::TrackerCSRT::to_Tracker", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerCSRT_to_Tracker(instance: *mut c_void) -> *mut c_void;
// cv::TrackerCSRT::delete() generated
// ("cv::TrackerCSRT::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerCSRT_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1489
// ("cv::TrackerCSRT::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerCSRT_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1494
// ("cv::TrackerCSRT::Params::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
pub fn cv_TrackerCSRT_Params_read_const_FileNodeR(instance: *mut c_void, unnamed: *const c_void, ocvrs_return: *mut Result<()>);
// write(cv::FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1499
// ("cv::TrackerCSRT::Params::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_TrackerCSRT_Params_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::TrackerCSRT::Params::use_hog() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1501
// ("cv::TrackerCSRT::Params::use_hog", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propUse_hog_const(instance: *const c_void) -> bool;
// cv::TrackerCSRT::Params::setUse_hog(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1501
// ("cv::TrackerCSRT::Params::setUse_hog", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_TrackerCSRT_Params_propUse_hog_const_bool(instance: *mut c_void, val: bool);
// cv::TrackerCSRT::Params::use_color_names() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1502
// ("cv::TrackerCSRT::Params::use_color_names", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propUse_color_names_const(instance: *const c_void) -> bool;
// cv::TrackerCSRT::Params::setUse_color_names(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1502
// ("cv::TrackerCSRT::Params::setUse_color_names", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_TrackerCSRT_Params_propUse_color_names_const_bool(instance: *mut c_void, val: bool);
// cv::TrackerCSRT::Params::use_gray() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1503
// ("cv::TrackerCSRT::Params::use_gray", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propUse_gray_const(instance: *const c_void) -> bool;
// cv::TrackerCSRT::Params::setUse_gray(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1503
// ("cv::TrackerCSRT::Params::setUse_gray", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_TrackerCSRT_Params_propUse_gray_const_bool(instance: *mut c_void, val: bool);
// cv::TrackerCSRT::Params::use_rgb() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1504
// ("cv::TrackerCSRT::Params::use_rgb", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propUse_rgb_const(instance: *const c_void) -> bool;
// cv::TrackerCSRT::Params::setUse_rgb(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1504
// ("cv::TrackerCSRT::Params::setUse_rgb", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_TrackerCSRT_Params_propUse_rgb_const_bool(instance: *mut c_void, val: bool);
// cv::TrackerCSRT::Params::use_channel_weights() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1505
// ("cv::TrackerCSRT::Params::use_channel_weights", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propUse_channel_weights_const(instance: *const c_void) -> bool;
// cv::TrackerCSRT::Params::setUse_channel_weights(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1505
// ("cv::TrackerCSRT::Params::setUse_channel_weights", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_TrackerCSRT_Params_propUse_channel_weights_const_bool(instance: *mut c_void, val: bool);
// cv::TrackerCSRT::Params::use_segmentation() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1506
// ("cv::TrackerCSRT::Params::use_segmentation", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propUse_segmentation_const(instance: *const c_void) -> bool;
// cv::TrackerCSRT::Params::setUse_segmentation(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1506
// ("cv::TrackerCSRT::Params::setUse_segmentation", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_TrackerCSRT_Params_propUse_segmentation_const_bool(instance: *mut c_void, val: bool);
// cv::TrackerCSRT::Params::window_function() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1508
// ("cv::TrackerCSRT::Params::window_function", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propWindow_function_const(instance: *const c_void) -> *mut c_void;
// cv::TrackerCSRT::Params::setWindow_function(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1508
// ("cv::TrackerCSRT::Params::setWindow_function", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
pub fn cv_TrackerCSRT_Params_propWindow_function_const_string(instance: *mut c_void, val: *const c_char);
// cv::TrackerCSRT::Params::kaiser_alpha() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1509
// ("cv::TrackerCSRT::Params::kaiser_alpha", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propKaiser_alpha_const(instance: *const c_void) -> f32;
// cv::TrackerCSRT::Params::setKaiser_alpha(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1509
// ("cv::TrackerCSRT::Params::setKaiser_alpha", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerCSRT_Params_propKaiser_alpha_const_float(instance: *mut c_void, val: f32);
// cv::TrackerCSRT::Params::cheb_attenuation() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1510
// ("cv::TrackerCSRT::Params::cheb_attenuation", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propCheb_attenuation_const(instance: *const c_void) -> f32;
// cv::TrackerCSRT::Params::setCheb_attenuation(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1510
// ("cv::TrackerCSRT::Params::setCheb_attenuation", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerCSRT_Params_propCheb_attenuation_const_float(instance: *mut c_void, val: f32);
// cv::TrackerCSRT::Params::template_size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1512
// ("cv::TrackerCSRT::Params::template_size", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propTemplate_size_const(instance: *const c_void) -> f32;
// cv::TrackerCSRT::Params::setTemplate_size(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1512
// ("cv::TrackerCSRT::Params::setTemplate_size", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerCSRT_Params_propTemplate_size_const_float(instance: *mut c_void, val: f32);
// cv::TrackerCSRT::Params::gsl_sigma() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1513
// ("cv::TrackerCSRT::Params::gsl_sigma", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propGsl_sigma_const(instance: *const c_void) -> f32;
// cv::TrackerCSRT::Params::setGsl_sigma(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1513
// ("cv::TrackerCSRT::Params::setGsl_sigma", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerCSRT_Params_propGsl_sigma_const_float(instance: *mut c_void, val: f32);
// cv::TrackerCSRT::Params::hog_orientations() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1514
// ("cv::TrackerCSRT::Params::hog_orientations", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propHog_orientations_const(instance: *const c_void) -> f32;
// cv::TrackerCSRT::Params::setHog_orientations(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1514
// ("cv::TrackerCSRT::Params::setHog_orientations", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerCSRT_Params_propHog_orientations_const_float(instance: *mut c_void, val: f32);
// cv::TrackerCSRT::Params::hog_clip() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1515
// ("cv::TrackerCSRT::Params::hog_clip", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propHog_clip_const(instance: *const c_void) -> f32;
// cv::TrackerCSRT::Params::setHog_clip(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1515
// ("cv::TrackerCSRT::Params::setHog_clip", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerCSRT_Params_propHog_clip_const_float(instance: *mut c_void, val: f32);
// cv::TrackerCSRT::Params::padding() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1516
// ("cv::TrackerCSRT::Params::padding", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propPadding_const(instance: *const c_void) -> f32;
// cv::TrackerCSRT::Params::setPadding(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1516
// ("cv::TrackerCSRT::Params::setPadding", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerCSRT_Params_propPadding_const_float(instance: *mut c_void, val: f32);
// cv::TrackerCSRT::Params::filter_lr() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1517
// ("cv::TrackerCSRT::Params::filter_lr", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propFilter_lr_const(instance: *const c_void) -> f32;
// cv::TrackerCSRT::Params::setFilter_lr(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1517
// ("cv::TrackerCSRT::Params::setFilter_lr", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerCSRT_Params_propFilter_lr_const_float(instance: *mut c_void, val: f32);
// cv::TrackerCSRT::Params::weights_lr() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1518
// ("cv::TrackerCSRT::Params::weights_lr", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propWeights_lr_const(instance: *const c_void) -> f32;
// cv::TrackerCSRT::Params::setWeights_lr(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1518
// ("cv::TrackerCSRT::Params::setWeights_lr", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerCSRT_Params_propWeights_lr_const_float(instance: *mut c_void, val: f32);
// cv::TrackerCSRT::Params::num_hog_channels_used() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1519
// ("cv::TrackerCSRT::Params::num_hog_channels_used", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propNum_hog_channels_used_const(instance: *const c_void) -> i32;
// cv::TrackerCSRT::Params::setNum_hog_channels_used(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1519
// ("cv::TrackerCSRT::Params::setNum_hog_channels_used", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerCSRT_Params_propNum_hog_channels_used_const_int(instance: *mut c_void, val: i32);
// cv::TrackerCSRT::Params::admm_iterations() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1520
// ("cv::TrackerCSRT::Params::admm_iterations", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propAdmm_iterations_const(instance: *const c_void) -> i32;
// cv::TrackerCSRT::Params::setAdmm_iterations(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1520
// ("cv::TrackerCSRT::Params::setAdmm_iterations", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerCSRT_Params_propAdmm_iterations_const_int(instance: *mut c_void, val: i32);
// cv::TrackerCSRT::Params::histogram_bins() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1521
// ("cv::TrackerCSRT::Params::histogram_bins", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propHistogram_bins_const(instance: *const c_void) -> i32;
// cv::TrackerCSRT::Params::setHistogram_bins(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1521
// ("cv::TrackerCSRT::Params::setHistogram_bins", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerCSRT_Params_propHistogram_bins_const_int(instance: *mut c_void, val: i32);
// cv::TrackerCSRT::Params::histogram_lr() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1522
// ("cv::TrackerCSRT::Params::histogram_lr", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propHistogram_lr_const(instance: *const c_void) -> f32;
// cv::TrackerCSRT::Params::setHistogram_lr(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1522
// ("cv::TrackerCSRT::Params::setHistogram_lr", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerCSRT_Params_propHistogram_lr_const_float(instance: *mut c_void, val: f32);
// cv::TrackerCSRT::Params::background_ratio() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1523
// ("cv::TrackerCSRT::Params::background_ratio", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propBackground_ratio_const(instance: *const c_void) -> i32;
// cv::TrackerCSRT::Params::setBackground_ratio(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1523
// ("cv::TrackerCSRT::Params::setBackground_ratio", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerCSRT_Params_propBackground_ratio_const_int(instance: *mut c_void, val: i32);
// cv::TrackerCSRT::Params::number_of_scales() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1524
// ("cv::TrackerCSRT::Params::number_of_scales", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propNumber_of_scales_const(instance: *const c_void) -> i32;
// cv::TrackerCSRT::Params::setNumber_of_scales(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1524
// ("cv::TrackerCSRT::Params::setNumber_of_scales", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerCSRT_Params_propNumber_of_scales_const_int(instance: *mut c_void, val: i32);
// cv::TrackerCSRT::Params::scale_sigma_factor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1525
// ("cv::TrackerCSRT::Params::scale_sigma_factor", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propScale_sigma_factor_const(instance: *const c_void) -> f32;
// cv::TrackerCSRT::Params::setScale_sigma_factor(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1525
// ("cv::TrackerCSRT::Params::setScale_sigma_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerCSRT_Params_propScale_sigma_factor_const_float(instance: *mut c_void, val: f32);
// cv::TrackerCSRT::Params::scale_model_max_area() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1526
// ("cv::TrackerCSRT::Params::scale_model_max_area", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propScale_model_max_area_const(instance: *const c_void) -> f32;
// cv::TrackerCSRT::Params::setScale_model_max_area(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1526
// ("cv::TrackerCSRT::Params::setScale_model_max_area", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerCSRT_Params_propScale_model_max_area_const_float(instance: *mut c_void, val: f32);
// cv::TrackerCSRT::Params::scale_lr() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1527
// ("cv::TrackerCSRT::Params::scale_lr", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propScale_lr_const(instance: *const c_void) -> f32;
// cv::TrackerCSRT::Params::setScale_lr(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1527
// ("cv::TrackerCSRT::Params::setScale_lr", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerCSRT_Params_propScale_lr_const_float(instance: *mut c_void, val: f32);
// cv::TrackerCSRT::Params::scale_step() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1528
// ("cv::TrackerCSRT::Params::scale_step", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propScale_step_const(instance: *const c_void) -> f32;
// cv::TrackerCSRT::Params::setScale_step(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1528
// ("cv::TrackerCSRT::Params::setScale_step", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerCSRT_Params_propScale_step_const_float(instance: *mut c_void, val: f32);
// cv::TrackerCSRT::Params::psr_threshold() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1530
// ("cv::TrackerCSRT::Params::psr_threshold", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerCSRT_Params_propPsr_threshold_const(instance: *const c_void) -> f32;
// cv::TrackerCSRT::Params::setPsr_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1530
// ("cv::TrackerCSRT::Params::setPsr_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerCSRT_Params_propPsr_threshold_const_float(instance: *mut c_void, val: f32);
// cv::TrackerCSRT::Params::delete() generated
// ("cv::TrackerCSRT::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerCSRT_Params_delete(instance: *mut c_void);
// compute(const std::vector<Mat> &, Mat &)(CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:80
// ("cv::TrackerFeature::compute", vec![(pred!(mut, ["images", "response"], ["const std::vector<cv::Mat>*", "cv::Mat*"]), _)]),
pub fn cv_TrackerFeature_compute_const_vectorLMatGR_MatR(instance: *mut c_void, images: *const c_void, response: *mut c_void, ocvrs_return: *mut Result<()>);
// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:95
// ("cv::TrackerFeature::create", vec![(pred!(mut, ["trackerFeatureType"], ["const cv::String*"]), _)]),
pub fn cv_TrackerFeature_create_const_StringR(tracker_feature_type: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// selection(Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:103
// ("cv::TrackerFeature::selection", vec![(pred!(mut, ["response", "npoints"], ["cv::Mat*", "int"]), _)]),
pub fn cv_TrackerFeature_selection_MatR_int(instance: *mut c_void, response: *mut c_void, npoints: i32, ocvrs_return: *mut Result<()>);
// getClassName()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:107
// ("cv::TrackerFeature::getClassName", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerFeature_getClassName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerFeature::to_TrackerFeatureFeature2d() generated
// ("cv::TrackerFeature::to_TrackerFeatureFeature2d", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerFeature_to_TrackerFeatureFeature2d(instance: *mut c_void) -> *mut c_void;
// cv::TrackerFeature::to_TrackerFeatureHAAR() generated
// ("cv::TrackerFeature::to_TrackerFeatureHAAR", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerFeature_to_TrackerFeatureHAAR(instance: *mut c_void) -> *mut c_void;
// cv::TrackerFeature::to_TrackerFeatureHOG() generated
// ("cv::TrackerFeature::to_TrackerFeatureHOG", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerFeature_to_TrackerFeatureHOG(instance: *mut c_void) -> *mut c_void;
// cv::TrackerFeature::to_TrackerFeatureLBP() generated
// ("cv::TrackerFeature::to_TrackerFeatureLBP", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerFeature_to_TrackerFeatureLBP(instance: *mut c_void) -> *mut c_void;
// cv::TrackerFeature::delete() generated
// ("cv::TrackerFeature::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerFeature_delete(instance: *mut c_void);
// TrackerFeatureFeature2d(String, String)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:940
// ("cv::TrackerFeatureFeature2d::TrackerFeatureFeature2d", vec![(pred!(mut, ["detectorType", "descriptorType"], ["cv::String", "cv::String"]), _)]),
pub fn cv_TrackerFeatureFeature2d_TrackerFeatureFeature2d_String_String(detector_type: *const c_char, descriptor_type: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// selection(Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:944
// ("cv::TrackerFeatureFeature2d::selection", vec![(pred!(mut, ["response", "npoints"], ["cv::Mat*", "int"]), _)]),
pub fn cv_TrackerFeatureFeature2d_selection_MatR_int(instance: *mut c_void, response: *mut c_void, npoints: i32, ocvrs_return: *mut Result<()>);
// cv::TrackerFeatureFeature2d::to_TrackerFeature() generated
// ("cv::TrackerFeatureFeature2d::to_TrackerFeature", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerFeatureFeature2d_to_TrackerFeature(instance: *mut c_void) -> *mut c_void;
// cv::TrackerFeatureFeature2d::delete() generated
// ("cv::TrackerFeatureFeature2d::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerFeatureFeature2d_delete(instance: *mut c_void);
// TrackerFeatureHAAR(const TrackerFeatureHAAR::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:991
// ("cv::TrackerFeatureHAAR::TrackerFeatureHAAR", vec![(pred!(mut, ["parameters"], ["const cv::TrackerFeatureHAAR::Params*"]), _)]),
pub fn cv_TrackerFeatureHAAR_TrackerFeatureHAAR_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerFeatureHAAR::TrackerFeatureHAAR() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:991
// ("cv::TrackerFeatureHAAR::TrackerFeatureHAAR", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerFeatureHAAR_TrackerFeatureHAAR(ocvrs_return: *mut Result<*mut c_void>);
// extractSelected(const std::vector<int>, const std::vector<Mat> &, Mat &)(CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1000
// ("cv::TrackerFeatureHAAR::extractSelected", vec![(pred!(mut, ["selFeatures", "images", "response"], ["const std::vector<int>", "const std::vector<cv::Mat>*", "cv::Mat*"]), _)]),
pub fn cv_TrackerFeatureHAAR_extractSelected_const_vectorLintG_const_vectorLMatGR_MatR(instance: *mut c_void, sel_features: *const c_void, images: *const c_void, response: *mut c_void, ocvrs_return: *mut Result<bool>);
// selection(Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1008
// ("cv::TrackerFeatureHAAR::selection", vec![(pred!(mut, ["response", "npoints"], ["cv::Mat*", "int"]), _)]),
pub fn cv_TrackerFeatureHAAR_selection_MatR_int(instance: *mut c_void, response: *mut c_void, npoints: i32, ocvrs_return: *mut Result<()>);
// swapFeature(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1014
// ("cv::TrackerFeatureHAAR::swapFeature", vec![(pred!(mut, ["source", "target"], ["int", "int"]), _)]),
pub fn cv_TrackerFeatureHAAR_swapFeature_int_int(instance: *mut c_void, source: i32, target: i32, ocvrs_return: *mut Result<bool>);
// swapFeature(int, CvHaarEvaluator::FeatureHaar &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1020
// ("cv::TrackerFeatureHAAR::swapFeature", vec![(pred!(mut, ["id", "feature"], ["int", "cv::CvHaarEvaluator::FeatureHaar*"]), _)]),
pub fn cv_TrackerFeatureHAAR_swapFeature_int_FeatureHaarR(instance: *mut c_void, id: i32, feature: *mut c_void, ocvrs_return: *mut Result<bool>);
// getFeatureAt(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1025
// ("cv::TrackerFeatureHAAR::getFeatureAt", vec![(pred!(mut, ["id"], ["int"]), _)]),
pub fn cv_TrackerFeatureHAAR_getFeatureAt_int(instance: *mut c_void, id: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerFeatureHAAR::to_TrackerFeature() generated
// ("cv::TrackerFeatureHAAR::to_TrackerFeature", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerFeatureHAAR_to_TrackerFeature(instance: *mut c_void) -> *mut c_void;
// cv::TrackerFeatureHAAR::delete() generated
// ("cv::TrackerFeatureHAAR::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerFeatureHAAR_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:982
// ("cv::TrackerFeatureHAAR::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerFeatureHAAR_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerFeatureHAAR::Params::numFeatures() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:983
// ("cv::TrackerFeatureHAAR::Params::numFeatures", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerFeatureHAAR_Params_propNumFeatures_const(instance: *const c_void) -> i32;
// cv::TrackerFeatureHAAR::Params::setNumFeatures(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:983
// ("cv::TrackerFeatureHAAR::Params::setNumFeatures", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerFeatureHAAR_Params_propNumFeatures_const_int(instance: *mut c_void, val: i32);
// cv::TrackerFeatureHAAR::Params::rectSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:984
// ("cv::TrackerFeatureHAAR::Params::rectSize", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerFeatureHAAR_Params_propRectSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::TrackerFeatureHAAR::Params::setRectSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:984
// ("cv::TrackerFeatureHAAR::Params::setRectSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_TrackerFeatureHAAR_Params_propRectSize_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::TrackerFeatureHAAR::Params::isIntegral() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:985
// ("cv::TrackerFeatureHAAR::Params::isIntegral", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerFeatureHAAR_Params_propIsIntegral_const(instance: *const c_void) -> bool;
// cv::TrackerFeatureHAAR::Params::setIsIntegral(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:985
// ("cv::TrackerFeatureHAAR::Params::setIsIntegral", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_TrackerFeatureHAAR_Params_propIsIntegral_const_bool(instance: *mut c_void, val: bool);
// cv::TrackerFeatureHAAR::Params::delete() generated
// ("cv::TrackerFeatureHAAR::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerFeatureHAAR_Params_delete(instance: *mut c_void);
// TrackerFeatureHOG()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:962
// ("cv::TrackerFeatureHOG::TrackerFeatureHOG", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerFeatureHOG_TrackerFeatureHOG(ocvrs_return: *mut Result<*mut c_void>);
// selection(Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:966
// ("cv::TrackerFeatureHOG::selection", vec![(pred!(mut, ["response", "npoints"], ["cv::Mat*", "int"]), _)]),
pub fn cv_TrackerFeatureHOG_selection_MatR_int(instance: *mut c_void, response: *mut c_void, npoints: i32, ocvrs_return: *mut Result<()>);
// cv::TrackerFeatureHOG::to_TrackerFeature() generated
// ("cv::TrackerFeatureHOG::to_TrackerFeature", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerFeatureHOG_to_TrackerFeature(instance: *mut c_void) -> *mut c_void;
// cv::TrackerFeatureHOG::delete() generated
// ("cv::TrackerFeatureHOG::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerFeatureHOG_delete(instance: *mut c_void);
// TrackerFeatureLBP()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1043
// ("cv::TrackerFeatureLBP::TrackerFeatureLBP", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerFeatureLBP_TrackerFeatureLBP(ocvrs_return: *mut Result<*mut c_void>);
// selection(Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1047
// ("cv::TrackerFeatureLBP::selection", vec![(pred!(mut, ["response", "npoints"], ["cv::Mat*", "int"]), _)]),
pub fn cv_TrackerFeatureLBP_selection_MatR_int(instance: *mut c_void, response: *mut c_void, npoints: i32, ocvrs_return: *mut Result<()>);
// cv::TrackerFeatureLBP::to_TrackerFeature() generated
// ("cv::TrackerFeatureLBP::to_TrackerFeature", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerFeatureLBP_to_TrackerFeature(instance: *mut c_void) -> *mut c_void;
// cv::TrackerFeatureLBP::delete() generated
// ("cv::TrackerFeatureLBP::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerFeatureLBP_delete(instance: *mut c_void);
// TrackerFeatureSet()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:132
// ("cv::TrackerFeatureSet::TrackerFeatureSet", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerFeatureSet_TrackerFeatureSet(ocvrs_return: *mut Result<*mut c_void>);
// extraction(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:139
// ("cv::TrackerFeatureSet::extraction", vec![(pred!(mut, ["images"], ["const std::vector<cv::Mat>*"]), _)]),
pub fn cv_TrackerFeatureSet_extraction_const_vectorLMatGR(instance: *mut c_void, images: *const c_void, ocvrs_return: *mut Result<()>);
// selection()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:143
// ("cv::TrackerFeatureSet::selection", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerFeatureSet_selection(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// removeOutliers()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:147
// ("cv::TrackerFeatureSet::removeOutliers", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerFeatureSet_removeOutliers(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// addTrackerFeature(String)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:174
// ("cv::TrackerFeatureSet::addTrackerFeature", vec![(pred!(mut, ["trackerFeatureType"], ["cv::String"]), _)]),
pub fn cv_TrackerFeatureSet_addTrackerFeature_String(instance: *mut c_void, tracker_feature_type: *const c_char, ocvrs_return: *mut Result<bool>);
// addTrackerFeature(Ptr<TrackerFeature> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:179
// ("cv::TrackerFeatureSet::addTrackerFeature", vec![(pred!(mut, ["feature"], ["cv::Ptr<cv::TrackerFeature>*"]), _)]),
pub fn cv_TrackerFeatureSet_addTrackerFeature_PtrLTrackerFeatureGR(instance: *mut c_void, feature: *mut c_void, ocvrs_return: *mut Result<bool>);
// getTrackerFeature()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:183
// ("cv::TrackerFeatureSet::getTrackerFeature", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerFeatureSet_getTrackerFeature_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getResponses()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:189
// ("cv::TrackerFeatureSet::getResponses", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerFeatureSet_getResponses_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerFeatureSet::delete() generated
// ("cv::TrackerFeatureSet::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerFeatureSet_delete(instance: *mut c_void);
// create(const TrackerGOTURN::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1302
// ("cv::TrackerGOTURN::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerGOTURN::Params*"]), _)]),
pub fn cv_TrackerGOTURN_create_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1304
// ("cv::TrackerGOTURN::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerGOTURN_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerGOTURN::to_Algorithm() generated
// ("cv::TrackerGOTURN::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerGOTURN_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::TrackerGOTURN::to_Tracker() generated
// ("cv::TrackerGOTURN::to_Tracker", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerGOTURN_to_Tracker(instance: *mut c_void) -> *mut c_void;
// cv::TrackerGOTURN::delete() generated
// ("cv::TrackerGOTURN::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerGOTURN_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1292
// ("cv::TrackerGOTURN::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerGOTURN_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1293
// ("cv::TrackerGOTURN::Params::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
pub fn cv_TrackerGOTURN_Params_read_const_FileNodeR(instance: *mut c_void, unnamed: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1294
// ("cv::TrackerGOTURN::Params::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
pub fn cv_TrackerGOTURN_Params_write_const_FileStorageR(instance: *const c_void, unnamed: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::TrackerGOTURN::Params::modelTxt() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1295
// ("cv::TrackerGOTURN::Params::modelTxt", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerGOTURN_Params_propModelTxt_const(instance: *const c_void) -> *mut c_void;
// cv::TrackerGOTURN::Params::setModelTxt(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1295
// ("cv::TrackerGOTURN::Params::setModelTxt", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
pub fn cv_TrackerGOTURN_Params_propModelTxt_const_String(instance: *mut c_void, val: *const c_char);
// cv::TrackerGOTURN::Params::modelBin() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1296
// ("cv::TrackerGOTURN::Params::modelBin", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerGOTURN_Params_propModelBin_const(instance: *const c_void) -> *mut c_void;
// cv::TrackerGOTURN::Params::setModelBin(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1296
// ("cv::TrackerGOTURN::Params::setModelBin", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
pub fn cv_TrackerGOTURN_Params_propModelBin_const_String(instance: *mut c_void, val: *const c_char);
// cv::TrackerGOTURN::Params::delete() generated
// ("cv::TrackerGOTURN::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerGOTURN_Params_delete(instance: *mut c_void);
// setFeatureExtractor(void (*)(const Mat, const Rect, Mat &), bool)(Function, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1260
// ("cv::TrackerKCF::setFeatureExtractor", vec![(pred!(mut, ["unnamed", "pca_func"], ["void (*)(const cv::Mat, const cv::Rect, cv::Mat&)", "bool"]), _)]),
pub fn cv_TrackerKCF_setFeatureExtractor_void__X__const_cv_Mat__const_cv_Rect__cv_MatR__bool(instance: *mut c_void, unnamed: Option<unsafe extern "C" fn(*const c_void, core::Rect, *mut c_void) -> ()>, pca_func: bool, ocvrs_return: *mut Result<()>);
// cv::TrackerKCF::setFeatureExtractor(Function) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1260
// ("cv::TrackerKCF::setFeatureExtractor", vec![(pred!(mut, ["unnamed"], ["void (*)(const cv::Mat, const cv::Rect, cv::Mat&)"]), _)]),
pub fn cv_TrackerKCF_setFeatureExtractor_void__X__const_cv_Mat__const_cv_Rect__cv_MatR_(instance: *mut c_void, unnamed: Option<unsafe extern "C" fn(*const c_void, core::Rect, *mut c_void) -> ()>, ocvrs_return: *mut Result<()>);
// create(const TrackerKCF::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1265
// ("cv::TrackerKCF::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerKCF::Params*"]), _)]),
pub fn cv_TrackerKCF_create_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1267
// ("cv::TrackerKCF::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerKCF_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerKCF::to_Algorithm() generated
// ("cv::TrackerKCF::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerKCF_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::TrackerKCF::to_Tracker() generated
// ("cv::TrackerKCF::to_Tracker", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerKCF_to_Tracker(instance: *mut c_void) -> *mut c_void;
// cv::TrackerKCF::delete() generated
// ("cv::TrackerKCF::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerKCF_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1232
// ("cv::TrackerKCF::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerKCF_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1237
// ("cv::TrackerKCF::Params::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
pub fn cv_TrackerKCF_Params_read_const_FileNodeR(instance: *mut c_void, unnamed: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1242
// ("cv::TrackerKCF::Params::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
pub fn cv_TrackerKCF_Params_write_const_FileStorageR(instance: *const c_void, unnamed: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::TrackerKCF::Params::detect_thresh() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1244
// ("cv::TrackerKCF::Params::detect_thresh", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerKCF_Params_propDetect_thresh_const(instance: *const c_void) -> f32;
// cv::TrackerKCF::Params::setDetect_thresh(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1244
// ("cv::TrackerKCF::Params::setDetect_thresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerKCF_Params_propDetect_thresh_const_float(instance: *mut c_void, val: f32);
// cv::TrackerKCF::Params::sigma() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1245
// ("cv::TrackerKCF::Params::sigma", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerKCF_Params_propSigma_const(instance: *const c_void) -> f32;
// cv::TrackerKCF::Params::setSigma(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1245
// ("cv::TrackerKCF::Params::setSigma", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerKCF_Params_propSigma_const_float(instance: *mut c_void, val: f32);
// cv::TrackerKCF::Params::lambda() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1246
// ("cv::TrackerKCF::Params::lambda", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerKCF_Params_propLambda_const(instance: *const c_void) -> f32;
// cv::TrackerKCF::Params::setLambda(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1246
// ("cv::TrackerKCF::Params::setLambda", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerKCF_Params_propLambda_const_float(instance: *mut c_void, val: f32);
// cv::TrackerKCF::Params::interp_factor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1247
// ("cv::TrackerKCF::Params::interp_factor", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerKCF_Params_propInterp_factor_const(instance: *const c_void) -> f32;
// cv::TrackerKCF::Params::setInterp_factor(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1247
// ("cv::TrackerKCF::Params::setInterp_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerKCF_Params_propInterp_factor_const_float(instance: *mut c_void, val: f32);
// cv::TrackerKCF::Params::output_sigma_factor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1248
// ("cv::TrackerKCF::Params::output_sigma_factor", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerKCF_Params_propOutput_sigma_factor_const(instance: *const c_void) -> f32;
// cv::TrackerKCF::Params::setOutput_sigma_factor(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1248
// ("cv::TrackerKCF::Params::setOutput_sigma_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerKCF_Params_propOutput_sigma_factor_const_float(instance: *mut c_void, val: f32);
// cv::TrackerKCF::Params::pca_learning_rate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1249
// ("cv::TrackerKCF::Params::pca_learning_rate", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerKCF_Params_propPca_learning_rate_const(instance: *const c_void) -> f32;
// cv::TrackerKCF::Params::setPca_learning_rate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1249
// ("cv::TrackerKCF::Params::setPca_learning_rate", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerKCF_Params_propPca_learning_rate_const_float(instance: *mut c_void, val: f32);
// cv::TrackerKCF::Params::resize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1250
// ("cv::TrackerKCF::Params::resize", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerKCF_Params_propResize_const(instance: *const c_void) -> bool;
// cv::TrackerKCF::Params::setResize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1250
// ("cv::TrackerKCF::Params::setResize", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_TrackerKCF_Params_propResize_const_bool(instance: *mut c_void, val: bool);
// cv::TrackerKCF::Params::split_coeff() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1251
// ("cv::TrackerKCF::Params::split_coeff", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerKCF_Params_propSplit_coeff_const(instance: *const c_void) -> bool;
// cv::TrackerKCF::Params::setSplit_coeff(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1251
// ("cv::TrackerKCF::Params::setSplit_coeff", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_TrackerKCF_Params_propSplit_coeff_const_bool(instance: *mut c_void, val: bool);
// cv::TrackerKCF::Params::wrap_kernel() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1252
// ("cv::TrackerKCF::Params::wrap_kernel", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerKCF_Params_propWrap_kernel_const(instance: *const c_void) -> bool;
// cv::TrackerKCF::Params::setWrap_kernel(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1252
// ("cv::TrackerKCF::Params::setWrap_kernel", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_TrackerKCF_Params_propWrap_kernel_const_bool(instance: *mut c_void, val: bool);
// cv::TrackerKCF::Params::compress_feature() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1253
// ("cv::TrackerKCF::Params::compress_feature", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerKCF_Params_propCompress_feature_const(instance: *const c_void) -> bool;
// cv::TrackerKCF::Params::setCompress_feature(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1253
// ("cv::TrackerKCF::Params::setCompress_feature", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_TrackerKCF_Params_propCompress_feature_const_bool(instance: *mut c_void, val: bool);
// cv::TrackerKCF::Params::max_patch_size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1254
// ("cv::TrackerKCF::Params::max_patch_size", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerKCF_Params_propMax_patch_size_const(instance: *const c_void) -> i32;
// cv::TrackerKCF::Params::setMax_patch_size(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1254
// ("cv::TrackerKCF::Params::setMax_patch_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerKCF_Params_propMax_patch_size_const_int(instance: *mut c_void, val: i32);
// cv::TrackerKCF::Params::compressed_size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1255
// ("cv::TrackerKCF::Params::compressed_size", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerKCF_Params_propCompressed_size_const(instance: *const c_void) -> i32;
// cv::TrackerKCF::Params::setCompressed_size(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1255
// ("cv::TrackerKCF::Params::setCompressed_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerKCF_Params_propCompressed_size_const_int(instance: *mut c_void, val: i32);
// cv::TrackerKCF::Params::desc_pca() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1256
// ("cv::TrackerKCF::Params::desc_pca", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerKCF_Params_propDesc_pca_const(instance: *const c_void) -> i32;
// cv::TrackerKCF::Params::setDesc_pca(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1256
// ("cv::TrackerKCF::Params::setDesc_pca", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerKCF_Params_propDesc_pca_const_int(instance: *mut c_void, val: i32);
// cv::TrackerKCF::Params::desc_npca() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1257
// ("cv::TrackerKCF::Params::desc_npca", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerKCF_Params_propDesc_npca_const(instance: *const c_void) -> i32;
// cv::TrackerKCF::Params::setDesc_npca(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1257
// ("cv::TrackerKCF::Params::setDesc_npca", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerKCF_Params_propDesc_npca_const_int(instance: *mut c_void, val: i32);
// cv::TrackerKCF::Params::delete() generated
// ("cv::TrackerKCF::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerKCF_Params_delete(instance: *mut c_void);
// create(const TrackerMIL::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1087
// ("cv::TrackerMIL::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerMIL::Params*"]), _)]),
pub fn cv_TrackerMIL_create_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1089
// ("cv::TrackerMIL::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerMIL_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerMIL::to_Algorithm() generated
// ("cv::TrackerMIL::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerMIL_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::TrackerMIL::to_Tracker() generated
// ("cv::TrackerMIL::to_Tracker", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerMIL_to_Tracker(instance: *mut c_void) -> *mut c_void;
// cv::TrackerMIL::delete() generated
// ("cv::TrackerMIL::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerMIL_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1070
// ("cv::TrackerMIL::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerMIL_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1080
// ("cv::TrackerMIL::Params::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
pub fn cv_TrackerMIL_Params_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1081
// ("cv::TrackerMIL::Params::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_TrackerMIL_Params_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::TrackerMIL::Params::samplerInitInRadius() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1072
// ("cv::TrackerMIL::Params::samplerInitInRadius", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerMIL_Params_propSamplerInitInRadius_const(instance: *const c_void) -> f32;
// cv::TrackerMIL::Params::setSamplerInitInRadius(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1072
// ("cv::TrackerMIL::Params::setSamplerInitInRadius", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerMIL_Params_propSamplerInitInRadius_const_float(instance: *mut c_void, val: f32);
// cv::TrackerMIL::Params::samplerInitMaxNegNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1073
// ("cv::TrackerMIL::Params::samplerInitMaxNegNum", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerMIL_Params_propSamplerInitMaxNegNum_const(instance: *const c_void) -> i32;
// cv::TrackerMIL::Params::setSamplerInitMaxNegNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1073
// ("cv::TrackerMIL::Params::setSamplerInitMaxNegNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerMIL_Params_propSamplerInitMaxNegNum_const_int(instance: *mut c_void, val: i32);
// cv::TrackerMIL::Params::samplerSearchWinSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1074
// ("cv::TrackerMIL::Params::samplerSearchWinSize", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerMIL_Params_propSamplerSearchWinSize_const(instance: *const c_void) -> f32;
// cv::TrackerMIL::Params::setSamplerSearchWinSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1074
// ("cv::TrackerMIL::Params::setSamplerSearchWinSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerMIL_Params_propSamplerSearchWinSize_const_float(instance: *mut c_void, val: f32);
// cv::TrackerMIL::Params::samplerTrackInRadius() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1075
// ("cv::TrackerMIL::Params::samplerTrackInRadius", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerMIL_Params_propSamplerTrackInRadius_const(instance: *const c_void) -> f32;
// cv::TrackerMIL::Params::setSamplerTrackInRadius(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1075
// ("cv::TrackerMIL::Params::setSamplerTrackInRadius", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerMIL_Params_propSamplerTrackInRadius_const_float(instance: *mut c_void, val: f32);
// cv::TrackerMIL::Params::samplerTrackMaxPosNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1076
// ("cv::TrackerMIL::Params::samplerTrackMaxPosNum", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerMIL_Params_propSamplerTrackMaxPosNum_const(instance: *const c_void) -> i32;
// cv::TrackerMIL::Params::setSamplerTrackMaxPosNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1076
// ("cv::TrackerMIL::Params::setSamplerTrackMaxPosNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerMIL_Params_propSamplerTrackMaxPosNum_const_int(instance: *mut c_void, val: i32);
// cv::TrackerMIL::Params::samplerTrackMaxNegNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1077
// ("cv::TrackerMIL::Params::samplerTrackMaxNegNum", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerMIL_Params_propSamplerTrackMaxNegNum_const(instance: *const c_void) -> i32;
// cv::TrackerMIL::Params::setSamplerTrackMaxNegNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1077
// ("cv::TrackerMIL::Params::setSamplerTrackMaxNegNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerMIL_Params_propSamplerTrackMaxNegNum_const_int(instance: *mut c_void, val: i32);
// cv::TrackerMIL::Params::featureSetNumFeatures() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1078
// ("cv::TrackerMIL::Params::featureSetNumFeatures", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerMIL_Params_propFeatureSetNumFeatures_const(instance: *const c_void) -> i32;
// cv::TrackerMIL::Params::setFeatureSetNumFeatures(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1078
// ("cv::TrackerMIL::Params::setFeatureSetNumFeatures", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerMIL_Params_propFeatureSetNumFeatures_const_int(instance: *mut c_void, val: i32);
// cv::TrackerMIL::Params::delete() generated
// ("cv::TrackerMIL::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerMIL_Params_delete(instance: *mut c_void);
// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1320
// ("cv::TrackerMOSSE::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerMOSSE_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerMOSSE::to_Algorithm() generated
// ("cv::TrackerMOSSE::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerMOSSE_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::TrackerMOSSE::to_Tracker() generated
// ("cv::TrackerMOSSE::to_Tracker", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerMOSSE_to_Tracker(instance: *mut c_void) -> *mut c_void;
// cv::TrackerMOSSE::delete() generated
// ("cv::TrackerMOSSE::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerMOSSE_delete(instance: *mut c_void);
// create(const TrackerMedianFlow::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1164
// ("cv::TrackerMedianFlow::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerMedianFlow::Params*"]), _)]),
pub fn cv_TrackerMedianFlow_create_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1166
// ("cv::TrackerMedianFlow::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerMedianFlow_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerMedianFlow::to_Algorithm() generated
// ("cv::TrackerMedianFlow::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerMedianFlow_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::TrackerMedianFlow::to_Tracker() generated
// ("cv::TrackerMedianFlow::to_Tracker", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerMedianFlow_to_Tracker(instance: *mut c_void) -> *mut c_void;
// cv::TrackerMedianFlow::delete() generated
// ("cv::TrackerMedianFlow::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerMedianFlow_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1147
// ("cv::TrackerMedianFlow::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerMedianFlow_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1157
// ("cv::TrackerMedianFlow::Params::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
pub fn cv_TrackerMedianFlow_Params_read_const_FileNodeR(instance: *mut c_void, unnamed: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1158
// ("cv::TrackerMedianFlow::Params::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
pub fn cv_TrackerMedianFlow_Params_write_const_FileStorageR(instance: *const c_void, unnamed: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::TrackerMedianFlow::Params::pointsInGrid() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1149
// ("cv::TrackerMedianFlow::Params::pointsInGrid", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerMedianFlow_Params_propPointsInGrid_const(instance: *const c_void) -> i32;
// cv::TrackerMedianFlow::Params::setPointsInGrid(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1149
// ("cv::TrackerMedianFlow::Params::setPointsInGrid", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerMedianFlow_Params_propPointsInGrid_const_int(instance: *mut c_void, val: i32);
// cv::TrackerMedianFlow::Params::winSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1151
// ("cv::TrackerMedianFlow::Params::winSize", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerMedianFlow_Params_propWinSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::TrackerMedianFlow::Params::setWinSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1151
// ("cv::TrackerMedianFlow::Params::setWinSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_TrackerMedianFlow_Params_propWinSize_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::TrackerMedianFlow::Params::maxLevel() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1152
// ("cv::TrackerMedianFlow::Params::maxLevel", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerMedianFlow_Params_propMaxLevel_const(instance: *const c_void) -> i32;
// cv::TrackerMedianFlow::Params::setMaxLevel(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1152
// ("cv::TrackerMedianFlow::Params::setMaxLevel", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerMedianFlow_Params_propMaxLevel_const_int(instance: *mut c_void, val: i32);
// cv::TrackerMedianFlow::Params::termCriteria() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1153
// ("cv::TrackerMedianFlow::Params::termCriteria", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerMedianFlow_Params_propTermCriteria_const(instance: *const c_void, ocvrs_return: *mut core::TermCriteria);
// cv::TrackerMedianFlow::Params::setTermCriteria(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1153
// ("cv::TrackerMedianFlow::Params::setTermCriteria", vec![(pred!(mut, ["val"], ["const cv::TermCriteria"]), _)]),
pub fn cv_TrackerMedianFlow_Params_propTermCriteria_const_TermCriteria(instance: *mut c_void, val: *const core::TermCriteria);
// cv::TrackerMedianFlow::Params::winSizeNCC() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1154
// ("cv::TrackerMedianFlow::Params::winSizeNCC", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerMedianFlow_Params_propWinSizeNCC_const(instance: *const c_void, ocvrs_return: *mut core::Size);
// cv::TrackerMedianFlow::Params::setWinSizeNCC(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1154
// ("cv::TrackerMedianFlow::Params::setWinSizeNCC", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
pub fn cv_TrackerMedianFlow_Params_propWinSizeNCC_const_Size(instance: *mut c_void, val: *const core::Size);
// cv::TrackerMedianFlow::Params::maxMedianLengthOfDisplacementDifference() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1155
// ("cv::TrackerMedianFlow::Params::maxMedianLengthOfDisplacementDifference", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerMedianFlow_Params_propMaxMedianLengthOfDisplacementDifference_const(instance: *const c_void) -> f64;
// cv::TrackerMedianFlow::Params::setMaxMedianLengthOfDisplacementDifference(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1155
// ("cv::TrackerMedianFlow::Params::setMaxMedianLengthOfDisplacementDifference", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_TrackerMedianFlow_Params_propMaxMedianLengthOfDisplacementDifference_const_double(instance: *mut c_void, val: f64);
// cv::TrackerMedianFlow::Params::delete() generated
// ("cv::TrackerMedianFlow::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerMedianFlow_Params_delete(instance: *mut c_void);
// setTrackerStateEstimator(Ptr<TrackerStateEstimator>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:464
// ("cv::TrackerModel::setTrackerStateEstimator", vec![(pred!(mut, ["trackerStateEstimator"], ["cv::Ptr<cv::TrackerStateEstimator>"]), _)]),
pub fn cv_TrackerModel_setTrackerStateEstimator_PtrLTrackerStateEstimatorG(instance: *mut c_void, tracker_state_estimator: *mut c_void, ocvrs_return: *mut Result<bool>);
// modelEstimation(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:471
// ("cv::TrackerModel::modelEstimation", vec![(pred!(mut, ["responses"], ["const std::vector<cv::Mat>*"]), _)]),
pub fn cv_TrackerModel_modelEstimation_const_vectorLMatGR(instance: *mut c_void, responses: *const c_void, ocvrs_return: *mut Result<()>);
// modelUpdate()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:477
// ("cv::TrackerModel::modelUpdate", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerModel_modelUpdate(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// runStateEstimator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:481
// ("cv::TrackerModel::runStateEstimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerModel_runStateEstimator(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
// setLastTargetState(const Ptr<TrackerTargetState> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:486
// ("cv::TrackerModel::setLastTargetState", vec![(pred!(mut, ["lastTargetState"], ["const cv::Ptr<cv::TrackerTargetState>*"]), _)]),
pub fn cv_TrackerModel_setLastTargetState_const_PtrLTrackerTargetStateGR(instance: *mut c_void, last_target_state: *const c_void, ocvrs_return: *mut Result<()>);
// getLastTargetState()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:490
// ("cv::TrackerModel::getLastTargetState", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerModel_getLastTargetState_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getConfidenceMaps()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:494
// ("cv::TrackerModel::getConfidenceMaps", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerModel_getConfidenceMaps_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getLastConfidenceMap()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:498
// ("cv::TrackerModel::getLastConfidenceMap", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerModel_getLastConfidenceMap_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getTrackerStateEstimator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:502
// ("cv::TrackerModel::getTrackerStateEstimator", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerModel_getTrackerStateEstimator_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerModel::delete() generated
// ("cv::TrackerModel::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerModel_delete(instance: *mut c_void);
// TrackerSampler()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:265
// ("cv::TrackerSampler::TrackerSampler", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerSampler_TrackerSampler(ocvrs_return: *mut Result<*mut c_void>);
// sampling(const Mat &, Rect)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:276
// ("cv::TrackerSampler::sampling", vec![(pred!(mut, ["image", "boundingBox"], ["const cv::Mat*", "cv::Rect"]), _)]),
pub fn cv_TrackerSampler_sampling_const_MatR_Rect(instance: *mut c_void, image: *const c_void, bounding_box: *const core::Rect, ocvrs_return: *mut Result<()>);
// getSamplers()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:280
// ("cv::TrackerSampler::getSamplers", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerSampler_getSamplers_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getSamples()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:284
// ("cv::TrackerSampler::getSamples", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerSampler_getSamples_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// addTrackerSamplerAlgorithm(String)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:307
// ("cv::TrackerSampler::addTrackerSamplerAlgorithm", vec![(pred!(mut, ["trackerSamplerAlgorithmType"], ["cv::String"]), _)]),
pub fn cv_TrackerSampler_addTrackerSamplerAlgorithm_String(instance: *mut c_void, tracker_sampler_algorithm_type: *const c_char, ocvrs_return: *mut Result<bool>);
// addTrackerSamplerAlgorithm(Ptr<TrackerSamplerAlgorithm> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:312
// ("cv::TrackerSampler::addTrackerSamplerAlgorithm", vec![(pred!(mut, ["sampler"], ["cv::Ptr<cv::TrackerSamplerAlgorithm>*"]), _)]),
pub fn cv_TrackerSampler_addTrackerSamplerAlgorithm_PtrLTrackerSamplerAlgorithmGR(instance: *mut c_void, sampler: *mut c_void, ocvrs_return: *mut Result<bool>);
// cv::TrackerSampler::delete() generated
// ("cv::TrackerSampler::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerSampler_delete(instance: *mut c_void);
// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:222
// ("cv::TrackerSamplerAlgorithm::create", vec![(pred!(mut, ["trackerSamplerType"], ["const cv::String*"]), _)]),
pub fn cv_TrackerSamplerAlgorithm_create_const_StringR(tracker_sampler_type: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// sampling(const Mat &, Rect, std::vector<Mat> &)(TraitClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:233
// ("cv::TrackerSamplerAlgorithm::sampling", vec![(pred!(mut, ["image", "boundingBox", "sample"], ["const cv::Mat*", "cv::Rect", "std::vector<cv::Mat>*"]), _)]),
pub fn cv_TrackerSamplerAlgorithm_sampling_const_MatR_Rect_vectorLMatGR(instance: *mut c_void, image: *const c_void, bounding_box: *const core::Rect, sample: *mut c_void, ocvrs_return: *mut Result<bool>);
// getClassName()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:237
// ("cv::TrackerSamplerAlgorithm::getClassName", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerSamplerAlgorithm_getClassName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerSamplerAlgorithm::to_TrackerSamplerCS() generated
// ("cv::TrackerSamplerAlgorithm::to_TrackerSamplerCS", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerSamplerAlgorithm_to_TrackerSamplerCS(instance: *mut c_void) -> *mut c_void;
// cv::TrackerSamplerAlgorithm::to_TrackerSamplerCSC() generated
// ("cv::TrackerSamplerAlgorithm::to_TrackerSamplerCSC", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerSamplerAlgorithm_to_TrackerSamplerCSC(instance: *mut c_void) -> *mut c_void;
// cv::TrackerSamplerAlgorithm::to_TrackerSamplerPF() generated
// ("cv::TrackerSamplerAlgorithm::to_TrackerSamplerPF", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerSamplerAlgorithm_to_TrackerSamplerPF(instance: *mut c_void) -> *mut c_void;
// cv::TrackerSamplerAlgorithm::delete() generated
// ("cv::TrackerSamplerAlgorithm::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerSamplerAlgorithm_delete(instance: *mut c_void);
// TrackerSamplerCS(const TrackerSamplerCS::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:848
// ("cv::TrackerSamplerCS::TrackerSamplerCS", vec![(pred!(mut, ["parameters"], ["const cv::TrackerSamplerCS::Params*"]), _)]),
pub fn cv_TrackerSamplerCS_TrackerSamplerCS_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerSamplerCS::TrackerSamplerCS() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:848
// ("cv::TrackerSamplerCS::TrackerSamplerCS", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerSamplerCS_TrackerSamplerCS(ocvrs_return: *mut Result<*mut c_void>);
// setMode(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:859
// ("cv::TrackerSamplerCS::setMode", vec![(pred!(mut, ["samplingMode"], ["int"]), _)]),
pub fn cv_TrackerSamplerCS_setMode_int(instance: *mut c_void, sampling_mode: i32, ocvrs_return: *mut Result<()>);
// samplingImpl(const Mat &, Rect, std::vector<Mat> &)(TraitClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:863
// ("cv::TrackerSamplerCS::samplingImpl", vec![(pred!(mut, ["image", "boundingBox", "sample"], ["const cv::Mat*", "cv::Rect", "std::vector<cv::Mat>*"]), _)]),
pub fn cv_TrackerSamplerCS_samplingImpl_const_MatR_Rect_vectorLMatGR(instance: *mut c_void, image: *const c_void, bounding_box: *const core::Rect, sample: *mut c_void, ocvrs_return: *mut Result<bool>);
// getROI()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:864
// ("cv::TrackerSamplerCS::getROI", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerSamplerCS_getROI_const(instance: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// cv::TrackerSamplerCS::to_TrackerSamplerAlgorithm() generated
// ("cv::TrackerSamplerCS::to_TrackerSamplerAlgorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerSamplerCS_to_TrackerSamplerAlgorithm(instance: *mut c_void) -> *mut c_void;
// cv::TrackerSamplerCS::delete() generated
// ("cv::TrackerSamplerCS::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerSamplerCS_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:841
// ("cv::TrackerSamplerCS::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerSamplerCS_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerSamplerCS::Params::overlap() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:842
// ("cv::TrackerSamplerCS::Params::overlap", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerSamplerCS_Params_propOverlap_const(instance: *const c_void) -> f32;
// cv::TrackerSamplerCS::Params::setOverlap(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:842
// ("cv::TrackerSamplerCS::Params::setOverlap", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerSamplerCS_Params_propOverlap_const_float(instance: *mut c_void, val: f32);
// cv::TrackerSamplerCS::Params::searchFactor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:843
// ("cv::TrackerSamplerCS::Params::searchFactor", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerSamplerCS_Params_propSearchFactor_const(instance: *const c_void) -> f32;
// cv::TrackerSamplerCS::Params::setSearchFactor(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:843
// ("cv::TrackerSamplerCS::Params::setSearchFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerSamplerCS_Params_propSearchFactor_const_float(instance: *mut c_void, val: f32);
// cv::TrackerSamplerCS::Params::delete() generated
// ("cv::TrackerSamplerCS::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerSamplerCS_Params_delete(instance: *mut c_void);
// TrackerSamplerCSC(const TrackerSamplerCSC::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:797
// ("cv::TrackerSamplerCSC::TrackerSamplerCSC", vec![(pred!(mut, ["parameters"], ["const cv::TrackerSamplerCSC::Params*"]), _)]),
pub fn cv_TrackerSamplerCSC_TrackerSamplerCSC_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerSamplerCSC::TrackerSamplerCSC() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:797
// ("cv::TrackerSamplerCSC::TrackerSamplerCSC", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerSamplerCSC_TrackerSamplerCSC(ocvrs_return: *mut Result<*mut c_void>);
// setMode(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:810
// ("cv::TrackerSamplerCSC::setMode", vec![(pred!(mut, ["samplingMode"], ["int"]), _)]),
pub fn cv_TrackerSamplerCSC_setMode_int(instance: *mut c_void, sampling_mode: i32, ocvrs_return: *mut Result<()>);
// cv::TrackerSamplerCSC::to_TrackerSamplerAlgorithm() generated
// ("cv::TrackerSamplerCSC::to_TrackerSamplerAlgorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerSamplerCSC_to_TrackerSamplerAlgorithm(instance: *mut c_void) -> *mut c_void;
// cv::TrackerSamplerCSC::delete() generated
// ("cv::TrackerSamplerCSC::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerSamplerCSC_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:785
// ("cv::TrackerSamplerCSC::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerSamplerCSC_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerSamplerCSC::Params::initInRad() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:786
// ("cv::TrackerSamplerCSC::Params::initInRad", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerSamplerCSC_Params_propInitInRad_const(instance: *const c_void) -> f32;
// cv::TrackerSamplerCSC::Params::setInitInRad(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:786
// ("cv::TrackerSamplerCSC::Params::setInitInRad", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerSamplerCSC_Params_propInitInRad_const_float(instance: *mut c_void, val: f32);
// cv::TrackerSamplerCSC::Params::trackInPosRad() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:787
// ("cv::TrackerSamplerCSC::Params::trackInPosRad", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerSamplerCSC_Params_propTrackInPosRad_const(instance: *const c_void) -> f32;
// cv::TrackerSamplerCSC::Params::setTrackInPosRad(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:787
// ("cv::TrackerSamplerCSC::Params::setTrackInPosRad", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerSamplerCSC_Params_propTrackInPosRad_const_float(instance: *mut c_void, val: f32);
// cv::TrackerSamplerCSC::Params::searchWinSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:788
// ("cv::TrackerSamplerCSC::Params::searchWinSize", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerSamplerCSC_Params_propSearchWinSize_const(instance: *const c_void) -> f32;
// cv::TrackerSamplerCSC::Params::setSearchWinSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:788
// ("cv::TrackerSamplerCSC::Params::setSearchWinSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_TrackerSamplerCSC_Params_propSearchWinSize_const_float(instance: *mut c_void, val: f32);
// cv::TrackerSamplerCSC::Params::initMaxNegNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:789
// ("cv::TrackerSamplerCSC::Params::initMaxNegNum", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerSamplerCSC_Params_propInitMaxNegNum_const(instance: *const c_void) -> i32;
// cv::TrackerSamplerCSC::Params::setInitMaxNegNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:789
// ("cv::TrackerSamplerCSC::Params::setInitMaxNegNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerSamplerCSC_Params_propInitMaxNegNum_const_int(instance: *mut c_void, val: i32);
// cv::TrackerSamplerCSC::Params::trackMaxPosNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:790
// ("cv::TrackerSamplerCSC::Params::trackMaxPosNum", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerSamplerCSC_Params_propTrackMaxPosNum_const(instance: *const c_void) -> i32;
// cv::TrackerSamplerCSC::Params::setTrackMaxPosNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:790
// ("cv::TrackerSamplerCSC::Params::setTrackMaxPosNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerSamplerCSC_Params_propTrackMaxPosNum_const_int(instance: *mut c_void, val: i32);
// cv::TrackerSamplerCSC::Params::trackMaxNegNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:791
// ("cv::TrackerSamplerCSC::Params::trackMaxNegNum", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerSamplerCSC_Params_propTrackMaxNegNum_const(instance: *const c_void) -> i32;
// cv::TrackerSamplerCSC::Params::setTrackMaxNegNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:791
// ("cv::TrackerSamplerCSC::Params::setTrackMaxNegNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerSamplerCSC_Params_propTrackMaxNegNum_const_int(instance: *mut c_void, val: i32);
// cv::TrackerSamplerCSC::Params::delete() generated
// ("cv::TrackerSamplerCSC::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerSamplerCSC_Params_delete(instance: *mut c_void);
// TrackerSamplerPF(const Mat &, const TrackerSamplerPF::Params &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:917
// ("cv::TrackerSamplerPF::TrackerSamplerPF", vec![(pred!(mut, ["chosenRect", "parameters"], ["const cv::Mat*", "const cv::TrackerSamplerPF::Params*"]), _)]),
pub fn cv_TrackerSamplerPF_TrackerSamplerPF_const_MatR_const_ParamsR(chosen_rect: *const c_void, parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerSamplerPF::TrackerSamplerPF(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:917
// ("cv::TrackerSamplerPF::TrackerSamplerPF", vec![(pred!(mut, ["chosenRect"], ["const cv::Mat*"]), _)]),
pub fn cv_TrackerSamplerPF_TrackerSamplerPF_const_MatR(chosen_rect: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerSamplerPF::to_TrackerSamplerAlgorithm() generated
// ("cv::TrackerSamplerPF::to_TrackerSamplerAlgorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerSamplerPF_to_TrackerSamplerAlgorithm(instance: *mut c_void) -> *mut c_void;
// cv::TrackerSamplerPF::delete() generated
// ("cv::TrackerSamplerPF::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerSamplerPF_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:905
// ("cv::TrackerSamplerPF::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerSamplerPF_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerSamplerPF::Params::iterationNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:906
// ("cv::TrackerSamplerPF::Params::iterationNum", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerSamplerPF_Params_propIterationNum_const(instance: *const c_void) -> i32;
// cv::TrackerSamplerPF::Params::setIterationNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:906
// ("cv::TrackerSamplerPF::Params::setIterationNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerSamplerPF_Params_propIterationNum_const_int(instance: *mut c_void, val: i32);
// cv::TrackerSamplerPF::Params::particlesNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:907
// ("cv::TrackerSamplerPF::Params::particlesNum", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerSamplerPF_Params_propParticlesNum_const(instance: *const c_void) -> i32;
// cv::TrackerSamplerPF::Params::setParticlesNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:907
// ("cv::TrackerSamplerPF::Params::setParticlesNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_TrackerSamplerPF_Params_propParticlesNum_const_int(instance: *mut c_void, val: i32);
// cv::TrackerSamplerPF::Params::alpha() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:908
// ("cv::TrackerSamplerPF::Params::alpha", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerSamplerPF_Params_propAlpha_const(instance: *const c_void) -> f64;
// cv::TrackerSamplerPF::Params::setAlpha(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:908
// ("cv::TrackerSamplerPF::Params::setAlpha", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_TrackerSamplerPF_Params_propAlpha_const_double(instance: *mut c_void, val: f64);
// cv::TrackerSamplerPF::Params::std() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:910
// ("cv::TrackerSamplerPF::Params::std", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerSamplerPF_Params_propStd_const(instance: *const c_void) -> *mut c_void;
// cv::TrackerSamplerPF::Params::setStd(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:910
// ("cv::TrackerSamplerPF::Params::setStd", vec![(pred!(mut, ["val"], ["const cv::Mat_<double>"]), _)]),
pub fn cv_TrackerSamplerPF_Params_propStd_const_Mat_LdoubleG(instance: *mut c_void, val: *const c_void);
// cv::TrackerSamplerPF::Params::delete() generated
// ("cv::TrackerSamplerPF::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerSamplerPF_Params_delete(instance: *mut c_void);
// estimate(const std::vector<ConfidenceMap> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:408
// ("cv::TrackerStateEstimator::estimate", vec![(pred!(mut, ["confidenceMaps"], ["const std::vector<cv::ConfidenceMap>*"]), _)]),
pub fn cv_TrackerStateEstimator_estimate_const_vectorLConfidenceMapGR(instance: *mut c_void, confidence_maps: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// update(std::vector<ConfidenceMap> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:413
// ("cv::TrackerStateEstimator::update", vec![(pred!(mut, ["confidenceMaps"], ["std::vector<cv::ConfidenceMap>*"]), _)]),
pub fn cv_TrackerStateEstimator_update_vectorLConfidenceMapGR(instance: *mut c_void, confidence_maps: *mut c_void, ocvrs_return: *mut Result<()>);
// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:426
// ("cv::TrackerStateEstimator::create", vec![(pred!(mut, ["trackeStateEstimatorType"], ["const cv::String*"]), _)]),
pub fn cv_TrackerStateEstimator_create_const_StringR(tracke_state_estimator_type: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// getClassName()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:430
// ("cv::TrackerStateEstimator::getClassName", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerStateEstimator_getClassName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerStateEstimator::to_TrackerStateEstimatorAdaBoosting() generated
// ("cv::TrackerStateEstimator::to_TrackerStateEstimatorAdaBoosting", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerStateEstimator_to_TrackerStateEstimatorAdaBoosting(instance: *mut c_void) -> *mut c_void;
// cv::TrackerStateEstimator::to_TrackerStateEstimatorMILBoosting() generated
// ("cv::TrackerStateEstimator::to_TrackerStateEstimatorMILBoosting", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerStateEstimator_to_TrackerStateEstimatorMILBoosting(instance: *mut c_void) -> *mut c_void;
// cv::TrackerStateEstimator::to_TrackerStateEstimatorSVM() generated
// ("cv::TrackerStateEstimator::to_TrackerStateEstimatorSVM", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerStateEstimator_to_TrackerStateEstimatorSVM(instance: *mut c_void) -> *mut c_void;
// cv::TrackerStateEstimator::delete() generated
// ("cv::TrackerStateEstimator::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerStateEstimator_delete(instance: *mut c_void);
// TrackerStateEstimatorAdaBoosting(int, int, int, Size, const Rect &)(Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:701
// ("cv::TrackerStateEstimatorAdaBoosting::TrackerStateEstimatorAdaBoosting", vec![(pred!(mut, ["numClassifer", "initIterations", "nFeatures", "patchSize", "ROI"], ["int", "int", "int", "cv::Size", "const cv::Rect*"]), _)]),
pub fn cv_TrackerStateEstimatorAdaBoosting_TrackerStateEstimatorAdaBoosting_int_int_int_Size_const_RectR(num_classifer: i32, init_iterations: i32, n_features: i32, patch_size: *const core::Size, roi: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
// getSampleROI()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:710
// ("cv::TrackerStateEstimatorAdaBoosting::getSampleROI", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerStateEstimatorAdaBoosting_getSampleROI_const(instance: *const c_void, ocvrs_return: *mut Result<core::Rect>);
// setSampleROI(const Rect &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:715
// ("cv::TrackerStateEstimatorAdaBoosting::setSampleROI", vec![(pred!(mut, ["ROI"], ["const cv::Rect*"]), _)]),
pub fn cv_TrackerStateEstimatorAdaBoosting_setSampleROI_const_RectR(instance: *mut c_void, roi: *const core::Rect, ocvrs_return: *mut Result<()>);
// setCurrentConfidenceMap(ConfidenceMap &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:720
// ("cv::TrackerStateEstimatorAdaBoosting::setCurrentConfidenceMap", vec![(pred!(mut, ["confidenceMap"], ["cv::ConfidenceMap*"]), _)]),
pub fn cv_TrackerStateEstimatorAdaBoosting_setCurrentConfidenceMap_ConfidenceMapR(instance: *mut c_void, confidence_map: *mut c_void, ocvrs_return: *mut Result<()>);
// computeSelectedWeakClassifier()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:724
// ("cv::TrackerStateEstimatorAdaBoosting::computeSelectedWeakClassifier", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerStateEstimatorAdaBoosting_computeSelectedWeakClassifier(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// computeReplacedClassifier()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:728
// ("cv::TrackerStateEstimatorAdaBoosting::computeReplacedClassifier", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerStateEstimatorAdaBoosting_computeReplacedClassifier(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// computeSwappedClassifier()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:732
// ("cv::TrackerStateEstimatorAdaBoosting::computeSwappedClassifier", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerStateEstimatorAdaBoosting_computeSwappedClassifier(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerStateEstimatorAdaBoosting::to_TrackerStateEstimator() generated
// ("cv::TrackerStateEstimatorAdaBoosting::to_TrackerStateEstimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerStateEstimatorAdaBoosting_to_TrackerStateEstimator(instance: *mut c_void) -> *mut c_void;
// cv::TrackerStateEstimatorAdaBoosting::delete() generated
// ("cv::TrackerStateEstimatorAdaBoosting::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerStateEstimatorAdaBoosting_delete(instance: *mut c_void);
#[cfg(not(target_os = "windows"))]
// TrackerAdaBoostingTargetState(const Point2f &, int, int, bool, const Mat &)(SimpleClass, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:663
// ("cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::TrackerAdaBoostingTargetState", vec![(pred!(mut, ["position", "width", "height", "foreground", "responses"], ["const cv::Point2f*", "int", "int", "bool", "const cv::Mat*"]), _)]),
pub fn cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_TrackerAdaBoostingTargetState_const_Point2fR_int_int_bool_const_MatR(position: *const core::Point2f, width: i32, height: i32, foreground: bool, responses: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
#[cfg(not(target_os = "windows"))]
// setTargetResponses(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:676
// ("cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::setTargetResponses", vec![(pred!(mut, ["responses"], ["const cv::Mat*"]), _)]),
pub fn cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_setTargetResponses_const_MatR(instance: *mut c_void, responses: *const c_void, ocvrs_return: *mut Result<()>);
#[cfg(not(target_os = "windows"))]
// setTargetFg(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:680
// ("cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::setTargetFg", vec![(pred!(mut, ["foreground"], ["bool"]), _)]),
pub fn cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_setTargetFg_bool(instance: *mut c_void, foreground: bool, ocvrs_return: *mut Result<()>);
#[cfg(not(target_os = "windows"))]
// getTargetResponses()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:683
// ("cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::getTargetResponses", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_getTargetResponses_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
#[cfg(not(target_os = "windows"))]
// isTargetFg()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:686
// ("cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::isTargetFg", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_isTargetFg_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::to_TrackerTargetState() generated
// ("cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::to_TrackerTargetState", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_to_TrackerTargetState(instance: *mut c_void) -> *mut c_void;
// cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::delete() generated
// ("cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_delete(instance: *mut c_void);
// TrackerStateEstimatorMILBoosting(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:621
// ("cv::TrackerStateEstimatorMILBoosting::TrackerStateEstimatorMILBoosting", vec![(pred!(mut, ["nFeatures"], ["int"]), _)]),
pub fn cv_TrackerStateEstimatorMILBoosting_TrackerStateEstimatorMILBoosting_int(n_features: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerStateEstimatorMILBoosting::TrackerStateEstimatorMILBoosting() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:621
// ("cv::TrackerStateEstimatorMILBoosting::TrackerStateEstimatorMILBoosting", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerStateEstimatorMILBoosting_TrackerStateEstimatorMILBoosting(ocvrs_return: *mut Result<*mut c_void>);
// setCurrentConfidenceMap(ConfidenceMap &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:627
// ("cv::TrackerStateEstimatorMILBoosting::setCurrentConfidenceMap", vec![(pred!(mut, ["confidenceMap"], ["cv::ConfidenceMap*"]), _)]),
pub fn cv_TrackerStateEstimatorMILBoosting_setCurrentConfidenceMap_ConfidenceMapR(instance: *mut c_void, confidence_map: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::TrackerStateEstimatorMILBoosting::to_TrackerStateEstimator() generated
// ("cv::TrackerStateEstimatorMILBoosting::to_TrackerStateEstimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerStateEstimatorMILBoosting_to_TrackerStateEstimator(instance: *mut c_void) -> *mut c_void;
// cv::TrackerStateEstimatorMILBoosting::delete() generated
// ("cv::TrackerStateEstimatorMILBoosting::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerStateEstimatorMILBoosting_delete(instance: *mut c_void);
#[cfg(not(target_os = "windows"))]
// TrackerMILTargetState(const Point2f &, int, int, bool, const Mat &)(SimpleClass, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:588
// ("cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::TrackerMILTargetState", vec![(pred!(mut, ["position", "width", "height", "foreground", "features"], ["const cv::Point2f*", "int", "int", "bool", "const cv::Mat*"]), _)]),
pub fn cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_TrackerMILTargetState_const_Point2fR_int_int_bool_const_MatR(position: *const core::Point2f, width: i32, height: i32, foreground: bool, features: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
#[cfg(not(target_os = "windows"))]
// setTargetFg(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:601
// ("cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::setTargetFg", vec![(pred!(mut, ["foreground"], ["bool"]), _)]),
pub fn cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_setTargetFg_bool(instance: *mut c_void, foreground: bool, ocvrs_return: *mut Result<()>);
#[cfg(not(target_os = "windows"))]
// setFeatures(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:605
// ("cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::setFeatures", vec![(pred!(mut, ["features"], ["const cv::Mat*"]), _)]),
pub fn cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_setFeatures_const_MatR(instance: *mut c_void, features: *const c_void, ocvrs_return: *mut Result<()>);
#[cfg(not(target_os = "windows"))]
// isTargetFg()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:608
// ("cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::isTargetFg", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_isTargetFg_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
#[cfg(not(target_os = "windows"))]
// getFeatures()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:611
// ("cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::getFeatures", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_getFeatures_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::to_TrackerTargetState() generated
// ("cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::to_TrackerTargetState", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_to_TrackerTargetState(instance: *mut c_void) -> *mut c_void;
// cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::delete() generated
// ("cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_delete(instance: *mut c_void);
// TrackerStateEstimatorSVM()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:759
// ("cv::TrackerStateEstimatorSVM::TrackerStateEstimatorSVM", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerStateEstimatorSVM_TrackerStateEstimatorSVM(ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerStateEstimatorSVM::to_TrackerStateEstimator() generated
// ("cv::TrackerStateEstimatorSVM::to_TrackerStateEstimator", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerStateEstimatorSVM_to_TrackerStateEstimator(instance: *mut c_void) -> *mut c_void;
// cv::TrackerStateEstimatorSVM::delete() generated
// ("cv::TrackerStateEstimatorSVM::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerStateEstimatorSVM_delete(instance: *mut c_void);
// create(const TrackerTLD::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1197
// ("cv::TrackerTLD::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerTLD::Params*"]), _)]),
pub fn cv_TrackerTLD_create_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1199
// ("cv::TrackerTLD::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerTLD_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::TrackerTLD::to_Algorithm() generated
// ("cv::TrackerTLD::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerTLD_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::TrackerTLD::to_Tracker() generated
// ("cv::TrackerTLD::to_Tracker", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerTLD_to_Tracker(instance: *mut c_void) -> *mut c_void;
// cv::TrackerTLD::delete() generated
// ("cv::TrackerTLD::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerTLD_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1189
// ("cv::TrackerTLD::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerTLD_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1190
// ("cv::TrackerTLD::Params::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
pub fn cv_TrackerTLD_Params_read_const_FileNodeR(instance: *mut c_void, unnamed: *const c_void, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1191
// ("cv::TrackerTLD::Params::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
pub fn cv_TrackerTLD_Params_write_const_FileStorageR(instance: *const c_void, unnamed: *mut c_void, ocvrs_return: *mut Result<()>);
// cv::TrackerTLD::Params::delete() generated
// ("cv::TrackerTLD::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerTLD_Params_delete(instance: *mut c_void);
// getTargetPosition()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:342
// ("cv::TrackerTargetState::getTargetPosition", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerTargetState_getTargetPosition_const(instance: *const c_void, ocvrs_return: *mut Result<core::Point2f>);
// setTargetPosition(const Point2f &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:348
// ("cv::TrackerTargetState::setTargetPosition", vec![(pred!(mut, ["position"], ["const cv::Point2f*"]), _)]),
pub fn cv_TrackerTargetState_setTargetPosition_const_Point2fR(instance: *mut c_void, position: *const core::Point2f, ocvrs_return: *mut Result<()>);
// getTargetWidth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:353
// ("cv::TrackerTargetState::getTargetWidth", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerTargetState_getTargetWidth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setTargetWidth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:359
// ("cv::TrackerTargetState::setTargetWidth", vec![(pred!(mut, ["width"], ["int"]), _)]),
pub fn cv_TrackerTargetState_setTargetWidth_int(instance: *mut c_void, width: i32, ocvrs_return: *mut Result<()>);
// getTargetHeight()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:364
// ("cv::TrackerTargetState::getTargetHeight", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerTargetState_getTargetHeight_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setTargetHeight(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:370
// ("cv::TrackerTargetState::setTargetHeight", vec![(pred!(mut, ["height"], ["int"]), _)]),
pub fn cv_TrackerTargetState_setTargetHeight_int(instance: *mut c_void, height: i32, ocvrs_return: *mut Result<()>);
// cv::TrackerTargetState::defaultNew() generated
// ("cv::TrackerTargetState::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_TrackerTargetState_defaultNew_const() -> *mut c_void;
// cv::TrackerTargetState::delete() generated
// ("cv::TrackerTargetState::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_TrackerTargetState_delete(instance: *mut c_void);
