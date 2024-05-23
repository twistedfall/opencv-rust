// ColorCorrectionModel(const Mat &, CONST_COLOR)(TraitClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:374
// ("cv::ccm::ColorCorrectionModel::ColorCorrectionModel", vec![(pred!(mut, ["src", "constcolor"], ["const cv::Mat*", "cv::ccm::CONST_COLOR"]), _)]),
pub fn cv_ccm_ColorCorrectionModel_ColorCorrectionModel_const_MatR_CONST_COLOR(src: *const c_void, constcolor: crate::mcc::CONST_COLOR, ocvrs_return: *mut Result<*mut c_void>);
// ColorCorrectionModel(const Mat &, Mat, COLOR_SPACE)(TraitClass, TraitClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:383
// ("cv::ccm::ColorCorrectionModel::ColorCorrectionModel", vec![(pred!(mut, ["src", "colors", "ref_cs"], ["const cv::Mat*", "cv::Mat", "cv::ccm::COLOR_SPACE"]), _)]),
pub fn cv_ccm_ColorCorrectionModel_ColorCorrectionModel_const_MatR_Mat_COLOR_SPACE(src: *const c_void, colors: *mut c_void, ref_cs: crate::mcc::COLOR_SPACE, ocvrs_return: *mut Result<*mut c_void>);
// ColorCorrectionModel(const Mat &, Mat, COLOR_SPACE, Mat)(TraitClass, TraitClass, Enum, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:393
// ("cv::ccm::ColorCorrectionModel::ColorCorrectionModel", vec![(pred!(mut, ["src", "colors", "ref_cs", "colored"], ["const cv::Mat*", "cv::Mat", "cv::ccm::COLOR_SPACE", "cv::Mat"]), _)]),
pub fn cv_ccm_ColorCorrectionModel_ColorCorrectionModel_const_MatR_Mat_COLOR_SPACE_Mat(src: *const c_void, colors: *mut c_void, ref_cs: crate::mcc::COLOR_SPACE, colored: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// setColorSpace(COLOR_SPACE)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:409
// ("cv::ccm::ColorCorrectionModel::setColorSpace", vec![(pred!(mut, ["cs"], ["cv::ccm::COLOR_SPACE"]), _)]),
pub fn cv_ccm_ColorCorrectionModel_setColorSpace_COLOR_SPACE(instance: *mut c_void, cs: crate::mcc::COLOR_SPACE, ocvrs_return: *mut Result<()>);
// setCCM_TYPE(CCM_TYPE)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:415
// ("cv::ccm::ColorCorrectionModel::setCCM_TYPE", vec![(pred!(mut, ["ccm_type"], ["cv::ccm::CCM_TYPE"]), _)]),
pub fn cv_ccm_ColorCorrectionModel_setCCM_TYPE_CCM_TYPE(instance: *mut c_void, ccm_type: crate::mcc::CCM_TYPE, ocvrs_return: *mut Result<()>);
// setDistance(DISTANCE_TYPE)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:421
// ("cv::ccm::ColorCorrectionModel::setDistance", vec![(pred!(mut, ["distance"], ["cv::ccm::DISTANCE_TYPE"]), _)]),
pub fn cv_ccm_ColorCorrectionModel_setDistance_DISTANCE_TYPE(instance: *mut c_void, distance: crate::mcc::DISTANCE_TYPE, ocvrs_return: *mut Result<()>);
// setLinear(LINEAR_TYPE)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:427
// ("cv::ccm::ColorCorrectionModel::setLinear", vec![(pred!(mut, ["linear_type"], ["cv::ccm::LINEAR_TYPE"]), _)]),
pub fn cv_ccm_ColorCorrectionModel_setLinear_LINEAR_TYPE(instance: *mut c_void, linear_type: crate::mcc::LINEAR_TYPE, ocvrs_return: *mut Result<()>);
// setLinearGamma(const double &)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:436
// ("cv::ccm::ColorCorrectionModel::setLinearGamma", vec![(pred!(mut, ["gamma"], ["const double*"]), _)]),
pub fn cv_ccm_ColorCorrectionModel_setLinearGamma_const_doubleR(instance: *mut c_void, gamma: *const f64, ocvrs_return: *mut Result<()>);
// setLinearDegree(const int &)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:449
// ("cv::ccm::ColorCorrectionModel::setLinearDegree", vec![(pred!(mut, ["deg"], ["const int*"]), _)]),
pub fn cv_ccm_ColorCorrectionModel_setLinearDegree_const_intR(instance: *mut c_void, deg: *const i32, ocvrs_return: *mut Result<()>);
// setSaturatedThreshold(const double &, const double &)(Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:459
// ("cv::ccm::ColorCorrectionModel::setSaturatedThreshold", vec![(pred!(mut, ["lower", "upper"], ["const double*", "const double*"]), _)]),
pub fn cv_ccm_ColorCorrectionModel_setSaturatedThreshold_const_doubleR_const_doubleR(instance: *mut c_void, lower: *const f64, upper: *const f64, ocvrs_return: *mut Result<()>);
// setWeightsList(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:465
// ("cv::ccm::ColorCorrectionModel::setWeightsList", vec![(pred!(mut, ["weights_list"], ["const cv::Mat*"]), _)]),
pub fn cv_ccm_ColorCorrectionModel_setWeightsList_const_MatR(instance: *mut c_void, weights_list: *const c_void, ocvrs_return: *mut Result<()>);
// setWeightCoeff(const double &)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:471
// ("cv::ccm::ColorCorrectionModel::setWeightCoeff", vec![(pred!(mut, ["weights_coeff"], ["const double*"]), _)]),
pub fn cv_ccm_ColorCorrectionModel_setWeightCoeff_const_doubleR(instance: *mut c_void, weights_coeff: *const f64, ocvrs_return: *mut Result<()>);
// setInitialMethod(INITIAL_METHOD_TYPE)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:477
// ("cv::ccm::ColorCorrectionModel::setInitialMethod", vec![(pred!(mut, ["initial_method_type"], ["cv::ccm::INITIAL_METHOD_TYPE"]), _)]),
pub fn cv_ccm_ColorCorrectionModel_setInitialMethod_INITIAL_METHOD_TYPE(instance: *mut c_void, initial_method_type: crate::mcc::INITIAL_METHOD_TYPE, ocvrs_return: *mut Result<()>);
// setMaxCount(const int &)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:484
// ("cv::ccm::ColorCorrectionModel::setMaxCount", vec![(pred!(mut, ["max_count"], ["const int*"]), _)]),
pub fn cv_ccm_ColorCorrectionModel_setMaxCount_const_intR(instance: *mut c_void, max_count: *const i32, ocvrs_return: *mut Result<()>);
// setEpsilon(const double &)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:491
// ("cv::ccm::ColorCorrectionModel::setEpsilon", vec![(pred!(mut, ["epsilon"], ["const double*"]), _)]),
pub fn cv_ccm_ColorCorrectionModel_setEpsilon_const_doubleR(instance: *mut c_void, epsilon: *const f64, ocvrs_return: *mut Result<()>);
// run()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:494
// ("cv::ccm::ColorCorrectionModel::run", vec![(pred!(mut, [], []), _)]),
pub fn cv_ccm_ColorCorrectionModel_run(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// getCCM()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:496
// ("cv::ccm::ColorCorrectionModel::getCCM", vec![(pred!(const, [], []), _)]),
pub fn cv_ccm_ColorCorrectionModel_getCCM_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getLoss()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:497
// ("cv::ccm::ColorCorrectionModel::getLoss", vec![(pred!(const, [], []), _)]),
pub fn cv_ccm_ColorCorrectionModel_getLoss_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// get_src_rgbl()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:498
// ("cv::ccm::ColorCorrectionModel::get_src_rgbl", vec![(pred!(const, [], []), _)]),
pub fn cv_ccm_ColorCorrectionModel_get_src_rgbl_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// get_dst_rgbl()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:499
// ("cv::ccm::ColorCorrectionModel::get_dst_rgbl", vec![(pred!(const, [], []), _)]),
pub fn cv_ccm_ColorCorrectionModel_get_dst_rgbl_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getMask()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:500
// ("cv::ccm::ColorCorrectionModel::getMask", vec![(pred!(const, [], []), _)]),
pub fn cv_ccm_ColorCorrectionModel_getMask_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getWeights()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:501
// ("cv::ccm::ColorCorrectionModel::getWeights", vec![(pred!(const, [], []), _)]),
pub fn cv_ccm_ColorCorrectionModel_getWeights_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// infer(const Mat &, bool)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:508
// ("cv::ccm::ColorCorrectionModel::infer", vec![(pred!(mut, ["img", "islinear"], ["const cv::Mat*", "bool"]), _)]),
pub fn cv_ccm_ColorCorrectionModel_infer_const_MatR_bool(instance: *mut c_void, img: *const c_void, islinear: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::ccm::ColorCorrectionModel::infer(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:508
// ("cv::ccm::ColorCorrectionModel::infer", vec![(pred!(mut, ["img"], ["const cv::Mat*"]), _)]),
pub fn cv_ccm_ColorCorrectionModel_infer_const_MatR(instance: *mut c_void, img: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::ccm::ColorCorrectionModel::delete() generated
// ("cv::ccm::ColorCorrectionModel::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ccm_ColorCorrectionModel_delete(instance: *mut c_void);
// create()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:73
// ("cv::mcc::CChecker::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_mcc_CChecker_create(ocvrs_return: *mut Result<*mut c_void>);
// setTarget(TYPECHART)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:83
// ("cv::mcc::CChecker::setTarget", vec![(pred!(mut, ["_target"], ["cv::mcc::TYPECHART"]), _)]),
pub fn cv_mcc_CChecker_setTarget_TYPECHART(instance: *mut c_void, _target: crate::mcc::MCC_TYPECHART, ocvrs_return: *mut Result<()>);
// setBox(std::vector<Point2f>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:84
// ("cv::mcc::CChecker::setBox", vec![(pred!(mut, ["_box"], ["std::vector<cv::Point2f>"]), _)]),
pub fn cv_mcc_CChecker_setBox_vectorLPoint2fG(instance: *mut c_void, _box: *mut c_void, ocvrs_return: *mut Result<()>);
// setChartsRGB(Mat)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:85
// ("cv::mcc::CChecker::setChartsRGB", vec![(pred!(mut, ["_chartsRGB"], ["cv::Mat"]), _)]),
pub fn cv_mcc_CChecker_setChartsRGB_Mat(instance: *mut c_void, _charts_rgb: *mut c_void, ocvrs_return: *mut Result<()>);
// setChartsYCbCr(Mat)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:86
// ("cv::mcc::CChecker::setChartsYCbCr", vec![(pred!(mut, ["_chartsYCbCr"], ["cv::Mat"]), _)]),
pub fn cv_mcc_CChecker_setChartsYCbCr_Mat(instance: *mut c_void, _charts_y_cb_cr: *mut c_void, ocvrs_return: *mut Result<()>);
// setCost(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:87
// ("cv::mcc::CChecker::setCost", vec![(pred!(mut, ["_cost"], ["float"]), _)]),
pub fn cv_mcc_CChecker_setCost_float(instance: *mut c_void, _cost: f32, ocvrs_return: *mut Result<()>);
// setCenter(Point2f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:88
// ("cv::mcc::CChecker::setCenter", vec![(pred!(mut, ["_center"], ["cv::Point2f"]), _)]),
pub fn cv_mcc_CChecker_setCenter_Point2f(instance: *mut c_void, _center: *const core::Point2f, ocvrs_return: *mut Result<()>);
// getTarget()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:90
// ("cv::mcc::CChecker::getTarget", vec![(pred!(mut, [], []), _)]),
pub fn cv_mcc_CChecker_getTarget(instance: *mut c_void, ocvrs_return: *mut Result<crate::mcc::MCC_TYPECHART>);
// getBox()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:91
// ("cv::mcc::CChecker::getBox", vec![(pred!(mut, [], []), _)]),
pub fn cv_mcc_CChecker_getBox(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// getColorCharts()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:99
// ("cv::mcc::CChecker::getColorCharts", vec![(pred!(mut, [], []), _)]),
pub fn cv_mcc_CChecker_getColorCharts(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// getChartsRGB()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:101
// ("cv::mcc::CChecker::getChartsRGB", vec![(pred!(mut, [], []), _)]),
pub fn cv_mcc_CChecker_getChartsRGB(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// getChartsYCbCr()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:102
// ("cv::mcc::CChecker::getChartsYCbCr", vec![(pred!(mut, [], []), _)]),
pub fn cv_mcc_CChecker_getChartsYCbCr(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// getCost()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:103
// ("cv::mcc::CChecker::getCost", vec![(pred!(mut, [], []), _)]),
pub fn cv_mcc_CChecker_getCost(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
// getCenter()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:104
// ("cv::mcc::CChecker::getCenter", vec![(pred!(mut, [], []), _)]),
pub fn cv_mcc_CChecker_getCenter(instance: *mut c_void, ocvrs_return: *mut Result<core::Point2f>);
// cv::mcc::CChecker::delete() generated
// ("cv::mcc::CChecker::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_mcc_CChecker_delete(instance: *mut c_void);
// setNet(dnn::Net)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:144
// ("cv::mcc::CCheckerDetector::setNet", vec![(pred!(mut, ["net"], ["cv::dnn::Net"]), _)]),
pub fn cv_mcc_CCheckerDetector_setNet_Net(instance: *mut c_void, net: *mut c_void, ocvrs_return: *mut Result<bool>);
// process(InputArray, const TYPECHART, const std::vector<Rect> &, const int, bool, const Ptr<DetectorParameters> &)(InputArray, Enum, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:167
// ("cv::mcc::CCheckerDetector::process", vec![(pred!(mut, ["image", "chartType", "regionsOfInterest", "nc", "useNet", "params"], ["const cv::_InputArray*", "const cv::mcc::TYPECHART", "const std::vector<cv::Rect>*", "const int", "bool", "const cv::Ptr<cv::mcc::DetectorParameters>*"]), _)]),
pub fn cv_mcc_CCheckerDetector_process_const__InputArrayR_const_TYPECHART_const_vectorLRectGR_const_int_bool_const_PtrLDetectorParametersGR(instance: *mut c_void, image: *const c_void, chart_type: crate::mcc::MCC_TYPECHART, regions_of_interest: *const c_void, nc: i32, use_net: bool, params: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::mcc::CCheckerDetector::process(InputArray, Enum, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:167
// ("cv::mcc::CCheckerDetector::process", vec![(pred!(mut, ["image", "chartType", "regionsOfInterest"], ["const cv::_InputArray*", "const cv::mcc::TYPECHART", "const std::vector<cv::Rect>*"]), _)]),
pub fn cv_mcc_CCheckerDetector_process_const__InputArrayR_const_TYPECHART_const_vectorLRectGR(instance: *mut c_void, image: *const c_void, chart_type: crate::mcc::MCC_TYPECHART, regions_of_interest: *const c_void, ocvrs_return: *mut Result<bool>);
// process(InputArray, const TYPECHART, const int, bool, const Ptr<DetectorParameters> &)(InputArray, Enum, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:195
// ("cv::mcc::CCheckerDetector::process", vec![(pred!(mut, ["image", "chartType", "nc", "useNet", "params"], ["const cv::_InputArray*", "const cv::mcc::TYPECHART", "const int", "bool", "const cv::Ptr<cv::mcc::DetectorParameters>*"]), _)]),
pub fn cv_mcc_CCheckerDetector_process_const__InputArrayR_const_TYPECHART_const_int_bool_const_PtrLDetectorParametersGR(instance: *mut c_void, image: *const c_void, chart_type: crate::mcc::MCC_TYPECHART, nc: i32, use_net: bool, params: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::mcc::CCheckerDetector::process(InputArray, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:195
// ("cv::mcc::CCheckerDetector::process", vec![(pred!(mut, ["image", "chartType"], ["const cv::_InputArray*", "const cv::mcc::TYPECHART"]), _)]),
pub fn cv_mcc_CCheckerDetector_process_const__InputArrayR_const_TYPECHART(instance: *mut c_void, image: *const c_void, chart_type: crate::mcc::MCC_TYPECHART, ocvrs_return: *mut Result<bool>);
// getBestColorChecker()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:204
// ("cv::mcc::CCheckerDetector::getBestColorChecker", vec![(pred!(mut, [], []), _)]),
pub fn cv_mcc_CCheckerDetector_getBestColorChecker(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// getListColorChecker()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:209
// ("cv::mcc::CCheckerDetector::getListColorChecker", vec![(pred!(mut, [], []), _)]),
pub fn cv_mcc_CCheckerDetector_getListColorChecker(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// create()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:214
// ("cv::mcc::CCheckerDetector::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_mcc_CCheckerDetector_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::mcc::CCheckerDetector::to_Algorithm() generated
// ("cv::mcc::CCheckerDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_mcc_CCheckerDetector_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::mcc::CCheckerDetector::delete() generated
// ("cv::mcc::CCheckerDetector::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_mcc_CCheckerDetector_delete(instance: *mut c_void);
// draw(InputOutputArray)(InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:129
// ("cv::mcc::CCheckerDraw::draw", vec![(pred!(mut, ["img"], ["const cv::_InputOutputArray*"]), _)]),
pub fn cv_mcc_CCheckerDraw_draw_const__InputOutputArrayR(instance: *mut c_void, img: *const c_void, ocvrs_return: *mut Result<()>);
// create(Ptr<CChecker>, cv::Scalar, int)(CppPassByVoidPtr, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:138
// ("cv::mcc::CCheckerDraw::create", vec![(pred!(mut, ["pChecker", "color", "thickness"], ["cv::Ptr<cv::mcc::CChecker>", "cv::Scalar", "int"]), _)]),
pub fn cv_mcc_CCheckerDraw_create_PtrLCCheckerG_Scalar_int(p_checker: *mut c_void, color: *const core::Scalar, thickness: i32, ocvrs_return: *mut Result<*mut c_void>);
// cv::mcc::CCheckerDraw::create(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:138
// ("cv::mcc::CCheckerDraw::create", vec![(pred!(mut, ["pChecker"], ["cv::Ptr<cv::mcc::CChecker>"]), _)]),
pub fn cv_mcc_CCheckerDraw_create_PtrLCCheckerG(p_checker: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::mcc::CCheckerDraw::delete() generated
// ("cv::mcc::CCheckerDraw::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_mcc_CCheckerDraw_delete(instance: *mut c_void);
// DetectorParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:102
// ("cv::mcc::DetectorParameters::DetectorParameters", vec![(pred!(mut, [], []), _)]),
pub fn cv_mcc_DetectorParameters_DetectorParameters(ocvrs_return: *mut Result<*mut c_void>);
// create()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:104
// ("cv::mcc::DetectorParameters::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_mcc_DetectorParameters_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::mcc::DetectorParameters::adaptiveThreshWinSizeMin() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:106
// ("cv::mcc::DetectorParameters::adaptiveThreshWinSizeMin", vec![(pred!(const, [], []), _)]),
pub fn cv_mcc_DetectorParameters_propAdaptiveThreshWinSizeMin_const(instance: *const c_void) -> i32;
// cv::mcc::DetectorParameters::setAdaptiveThreshWinSizeMin(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:106
// ("cv::mcc::DetectorParameters::setAdaptiveThreshWinSizeMin", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_mcc_DetectorParameters_propAdaptiveThreshWinSizeMin_const_int(instance: *mut c_void, val: i32);
// cv::mcc::DetectorParameters::adaptiveThreshWinSizeMax() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:107
// ("cv::mcc::DetectorParameters::adaptiveThreshWinSizeMax", vec![(pred!(const, [], []), _)]),
pub fn cv_mcc_DetectorParameters_propAdaptiveThreshWinSizeMax_const(instance: *const c_void) -> i32;
// cv::mcc::DetectorParameters::setAdaptiveThreshWinSizeMax(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:107
// ("cv::mcc::DetectorParameters::setAdaptiveThreshWinSizeMax", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_mcc_DetectorParameters_propAdaptiveThreshWinSizeMax_const_int(instance: *mut c_void, val: i32);
// cv::mcc::DetectorParameters::adaptiveThreshWinSizeStep() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:108
// ("cv::mcc::DetectorParameters::adaptiveThreshWinSizeStep", vec![(pred!(const, [], []), _)]),
pub fn cv_mcc_DetectorParameters_propAdaptiveThreshWinSizeStep_const(instance: *const c_void) -> i32;
// cv::mcc::DetectorParameters::setAdaptiveThreshWinSizeStep(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:108
// ("cv::mcc::DetectorParameters::setAdaptiveThreshWinSizeStep", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_mcc_DetectorParameters_propAdaptiveThreshWinSizeStep_const_int(instance: *mut c_void, val: i32);
// cv::mcc::DetectorParameters::adaptiveThreshConstant() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:109
// ("cv::mcc::DetectorParameters::adaptiveThreshConstant", vec![(pred!(const, [], []), _)]),
pub fn cv_mcc_DetectorParameters_propAdaptiveThreshConstant_const(instance: *const c_void) -> f64;
// cv::mcc::DetectorParameters::setAdaptiveThreshConstant(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:109
// ("cv::mcc::DetectorParameters::setAdaptiveThreshConstant", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_mcc_DetectorParameters_propAdaptiveThreshConstant_const_double(instance: *mut c_void, val: f64);
// cv::mcc::DetectorParameters::minContoursAreaRate() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:110
// ("cv::mcc::DetectorParameters::minContoursAreaRate", vec![(pred!(const, [], []), _)]),
pub fn cv_mcc_DetectorParameters_propMinContoursAreaRate_const(instance: *const c_void) -> f64;
// cv::mcc::DetectorParameters::setMinContoursAreaRate(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:110
// ("cv::mcc::DetectorParameters::setMinContoursAreaRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_mcc_DetectorParameters_propMinContoursAreaRate_const_double(instance: *mut c_void, val: f64);
// cv::mcc::DetectorParameters::minContoursArea() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:111
// ("cv::mcc::DetectorParameters::minContoursArea", vec![(pred!(const, [], []), _)]),
pub fn cv_mcc_DetectorParameters_propMinContoursArea_const(instance: *const c_void) -> f64;
// cv::mcc::DetectorParameters::setMinContoursArea(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:111
// ("cv::mcc::DetectorParameters::setMinContoursArea", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_mcc_DetectorParameters_propMinContoursArea_const_double(instance: *mut c_void, val: f64);
// cv::mcc::DetectorParameters::confidenceThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:112
// ("cv::mcc::DetectorParameters::confidenceThreshold", vec![(pred!(const, [], []), _)]),
pub fn cv_mcc_DetectorParameters_propConfidenceThreshold_const(instance: *const c_void) -> f64;
// cv::mcc::DetectorParameters::setConfidenceThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:112
// ("cv::mcc::DetectorParameters::setConfidenceThreshold", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_mcc_DetectorParameters_propConfidenceThreshold_const_double(instance: *mut c_void, val: f64);
// cv::mcc::DetectorParameters::minContourSolidity() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:113
// ("cv::mcc::DetectorParameters::minContourSolidity", vec![(pred!(const, [], []), _)]),
pub fn cv_mcc_DetectorParameters_propMinContourSolidity_const(instance: *const c_void) -> f64;
// cv::mcc::DetectorParameters::setMinContourSolidity(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:113
// ("cv::mcc::DetectorParameters::setMinContourSolidity", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_mcc_DetectorParameters_propMinContourSolidity_const_double(instance: *mut c_void, val: f64);
// cv::mcc::DetectorParameters::findCandidatesApproxPolyDPEpsMultiplier() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:114
// ("cv::mcc::DetectorParameters::findCandidatesApproxPolyDPEpsMultiplier", vec![(pred!(const, [], []), _)]),
pub fn cv_mcc_DetectorParameters_propFindCandidatesApproxPolyDPEpsMultiplier_const(instance: *const c_void) -> f64;
// cv::mcc::DetectorParameters::setFindCandidatesApproxPolyDPEpsMultiplier(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:114
// ("cv::mcc::DetectorParameters::setFindCandidatesApproxPolyDPEpsMultiplier", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_mcc_DetectorParameters_propFindCandidatesApproxPolyDPEpsMultiplier_const_double(instance: *mut c_void, val: f64);
// cv::mcc::DetectorParameters::borderWidth() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:115
// ("cv::mcc::DetectorParameters::borderWidth", vec![(pred!(const, [], []), _)]),
pub fn cv_mcc_DetectorParameters_propBorderWidth_const(instance: *const c_void) -> i32;
// cv::mcc::DetectorParameters::setBorderWidth(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:115
// ("cv::mcc::DetectorParameters::setBorderWidth", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_mcc_DetectorParameters_propBorderWidth_const_int(instance: *mut c_void, val: i32);
// cv::mcc::DetectorParameters::B0factor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:116
// ("cv::mcc::DetectorParameters::B0factor", vec![(pred!(const, [], []), _)]),
pub fn cv_mcc_DetectorParameters_propB0factor_const(instance: *const c_void) -> f32;
// cv::mcc::DetectorParameters::setB0factor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:116
// ("cv::mcc::DetectorParameters::setB0factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_mcc_DetectorParameters_propB0factor_const_float(instance: *mut c_void, val: f32);
// cv::mcc::DetectorParameters::maxError() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:117
// ("cv::mcc::DetectorParameters::maxError", vec![(pred!(const, [], []), _)]),
pub fn cv_mcc_DetectorParameters_propMaxError_const(instance: *const c_void) -> f32;
// cv::mcc::DetectorParameters::setMaxError(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:117
// ("cv::mcc::DetectorParameters::setMaxError", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_mcc_DetectorParameters_propMaxError_const_float(instance: *mut c_void, val: f32);
// cv::mcc::DetectorParameters::minContourPointsAllowed() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:118
// ("cv::mcc::DetectorParameters::minContourPointsAllowed", vec![(pred!(const, [], []), _)]),
pub fn cv_mcc_DetectorParameters_propMinContourPointsAllowed_const(instance: *const c_void) -> i32;
// cv::mcc::DetectorParameters::setMinContourPointsAllowed(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:118
// ("cv::mcc::DetectorParameters::setMinContourPointsAllowed", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_mcc_DetectorParameters_propMinContourPointsAllowed_const_int(instance: *mut c_void, val: i32);
// cv::mcc::DetectorParameters::minContourLengthAllowed() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:119
// ("cv::mcc::DetectorParameters::minContourLengthAllowed", vec![(pred!(const, [], []), _)]),
pub fn cv_mcc_DetectorParameters_propMinContourLengthAllowed_const(instance: *const c_void) -> i32;
// cv::mcc::DetectorParameters::setMinContourLengthAllowed(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:119
// ("cv::mcc::DetectorParameters::setMinContourLengthAllowed", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_mcc_DetectorParameters_propMinContourLengthAllowed_const_int(instance: *mut c_void, val: i32);
// cv::mcc::DetectorParameters::minInterContourDistance() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:120
// ("cv::mcc::DetectorParameters::minInterContourDistance", vec![(pred!(const, [], []), _)]),
pub fn cv_mcc_DetectorParameters_propMinInterContourDistance_const(instance: *const c_void) -> i32;
// cv::mcc::DetectorParameters::setMinInterContourDistance(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:120
// ("cv::mcc::DetectorParameters::setMinInterContourDistance", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_mcc_DetectorParameters_propMinInterContourDistance_const_int(instance: *mut c_void, val: i32);
// cv::mcc::DetectorParameters::minInterCheckerDistance() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:121
// ("cv::mcc::DetectorParameters::minInterCheckerDistance", vec![(pred!(const, [], []), _)]),
pub fn cv_mcc_DetectorParameters_propMinInterCheckerDistance_const(instance: *const c_void) -> i32;
// cv::mcc::DetectorParameters::setMinInterCheckerDistance(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:121
// ("cv::mcc::DetectorParameters::setMinInterCheckerDistance", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_mcc_DetectorParameters_propMinInterCheckerDistance_const_int(instance: *mut c_void, val: i32);
// cv::mcc::DetectorParameters::minImageSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:122
// ("cv::mcc::DetectorParameters::minImageSize", vec![(pred!(const, [], []), _)]),
pub fn cv_mcc_DetectorParameters_propMinImageSize_const(instance: *const c_void) -> i32;
// cv::mcc::DetectorParameters::setMinImageSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:122
// ("cv::mcc::DetectorParameters::setMinImageSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_mcc_DetectorParameters_propMinImageSize_const_int(instance: *mut c_void, val: i32);
// cv::mcc::DetectorParameters::minGroupSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:123
// ("cv::mcc::DetectorParameters::minGroupSize", vec![(pred!(const, [], []), _)]),
pub fn cv_mcc_DetectorParameters_propMinGroupSize_const(instance: *const c_void) -> u32;
// cv::mcc::DetectorParameters::setMinGroupSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:123
// ("cv::mcc::DetectorParameters::setMinGroupSize", vec![(pred!(mut, ["val"], ["const unsigned int"]), _)]),
pub fn cv_mcc_DetectorParameters_propMinGroupSize_const_unsigned_int(instance: *mut c_void, val: u32);
// cv::mcc::DetectorParameters::delete() generated
// ("cv::mcc::DetectorParameters::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_mcc_DetectorParameters_delete(instance: *mut c_void);
