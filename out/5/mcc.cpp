#include "ocvrs_common.hpp"
#include <opencv2/mcc.hpp>
#include "mcc_types.hpp"

extern "C" {
	// ColorCorrectionModel(const Mat &, CONST_COLOR)(TraitClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:374
	// ("cv::ccm::ColorCorrectionModel::ColorCorrectionModel", vec![(pred!(mut, ["src", "constcolor"], ["const cv::Mat*", "cv::ccm::CONST_COLOR"]), _)]),
	void cv_ccm_ColorCorrectionModel_ColorCorrectionModel_const_MatR_CONST_COLOR(const cv::Mat* src, cv::ccm::CONST_COLOR constcolor, Result<cv::ccm::ColorCorrectionModel*>* ocvrs_return) {
		try {
			cv::ccm::ColorCorrectionModel* ret = new cv::ccm::ColorCorrectionModel(*src, constcolor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ColorCorrectionModel(const Mat &, Mat, COLOR_SPACE)(TraitClass, TraitClass, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:383
	// ("cv::ccm::ColorCorrectionModel::ColorCorrectionModel", vec![(pred!(mut, ["src", "colors", "ref_cs"], ["const cv::Mat*", "cv::Mat", "cv::ccm::COLOR_SPACE"]), _)]),
	void cv_ccm_ColorCorrectionModel_ColorCorrectionModel_const_MatR_Mat_COLOR_SPACE(const cv::Mat* src, cv::Mat* colors, cv::ccm::COLOR_SPACE ref_cs, Result<cv::ccm::ColorCorrectionModel*>* ocvrs_return) {
		try {
			cv::ccm::ColorCorrectionModel* ret = new cv::ccm::ColorCorrectionModel(*src, *colors, ref_cs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ColorCorrectionModel(const Mat &, Mat, COLOR_SPACE, Mat)(TraitClass, TraitClass, Enum, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:393
	// ("cv::ccm::ColorCorrectionModel::ColorCorrectionModel", vec![(pred!(mut, ["src", "colors", "ref_cs", "colored"], ["const cv::Mat*", "cv::Mat", "cv::ccm::COLOR_SPACE", "cv::Mat"]), _)]),
	void cv_ccm_ColorCorrectionModel_ColorCorrectionModel_const_MatR_Mat_COLOR_SPACE_Mat(const cv::Mat* src, cv::Mat* colors, cv::ccm::COLOR_SPACE ref_cs, cv::Mat* colored, Result<cv::ccm::ColorCorrectionModel*>* ocvrs_return) {
		try {
			cv::ccm::ColorCorrectionModel* ret = new cv::ccm::ColorCorrectionModel(*src, *colors, ref_cs, *colored);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setColorSpace(COLOR_SPACE)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:409
	// ("cv::ccm::ColorCorrectionModel::setColorSpace", vec![(pred!(mut, ["cs"], ["cv::ccm::COLOR_SPACE"]), _)]),
	void cv_ccm_ColorCorrectionModel_setColorSpace_COLOR_SPACE(cv::ccm::ColorCorrectionModel* instance, cv::ccm::COLOR_SPACE cs, ResultVoid* ocvrs_return) {
		try {
			instance->setColorSpace(cs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCCM_TYPE(CCM_TYPE)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:415
	// ("cv::ccm::ColorCorrectionModel::setCCM_TYPE", vec![(pred!(mut, ["ccm_type"], ["cv::ccm::CCM_TYPE"]), _)]),
	void cv_ccm_ColorCorrectionModel_setCCM_TYPE_CCM_TYPE(cv::ccm::ColorCorrectionModel* instance, cv::ccm::CCM_TYPE ccm_type, ResultVoid* ocvrs_return) {
		try {
			instance->setCCM_TYPE(ccm_type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDistance(DISTANCE_TYPE)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:421
	// ("cv::ccm::ColorCorrectionModel::setDistance", vec![(pred!(mut, ["distance"], ["cv::ccm::DISTANCE_TYPE"]), _)]),
	void cv_ccm_ColorCorrectionModel_setDistance_DISTANCE_TYPE(cv::ccm::ColorCorrectionModel* instance, cv::ccm::DISTANCE_TYPE distance, ResultVoid* ocvrs_return) {
		try {
			instance->setDistance(distance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLinear(LINEAR_TYPE)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:427
	// ("cv::ccm::ColorCorrectionModel::setLinear", vec![(pred!(mut, ["linear_type"], ["cv::ccm::LINEAR_TYPE"]), _)]),
	void cv_ccm_ColorCorrectionModel_setLinear_LINEAR_TYPE(cv::ccm::ColorCorrectionModel* instance, cv::ccm::LINEAR_TYPE linear_type, ResultVoid* ocvrs_return) {
		try {
			instance->setLinear(linear_type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLinearGamma(const double &)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:436
	// ("cv::ccm::ColorCorrectionModel::setLinearGamma", vec![(pred!(mut, ["gamma"], ["const double*"]), _)]),
	void cv_ccm_ColorCorrectionModel_setLinearGamma_const_doubleR(cv::ccm::ColorCorrectionModel* instance, const double* gamma, ResultVoid* ocvrs_return) {
		try {
			instance->setLinearGamma(*gamma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLinearDegree(const int &)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:449
	// ("cv::ccm::ColorCorrectionModel::setLinearDegree", vec![(pred!(mut, ["deg"], ["const int*"]), _)]),
	void cv_ccm_ColorCorrectionModel_setLinearDegree_const_intR(cv::ccm::ColorCorrectionModel* instance, const int* deg, ResultVoid* ocvrs_return) {
		try {
			instance->setLinearDegree(*deg);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSaturatedThreshold(const double &, const double &)(Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:459
	// ("cv::ccm::ColorCorrectionModel::setSaturatedThreshold", vec![(pred!(mut, ["lower", "upper"], ["const double*", "const double*"]), _)]),
	void cv_ccm_ColorCorrectionModel_setSaturatedThreshold_const_doubleR_const_doubleR(cv::ccm::ColorCorrectionModel* instance, const double* lower, const double* upper, ResultVoid* ocvrs_return) {
		try {
			instance->setSaturatedThreshold(*lower, *upper);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeightsList(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:465
	// ("cv::ccm::ColorCorrectionModel::setWeightsList", vec![(pred!(mut, ["weights_list"], ["const cv::Mat*"]), _)]),
	void cv_ccm_ColorCorrectionModel_setWeightsList_const_MatR(cv::ccm::ColorCorrectionModel* instance, const cv::Mat* weights_list, ResultVoid* ocvrs_return) {
		try {
			instance->setWeightsList(*weights_list);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeightCoeff(const double &)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:471
	// ("cv::ccm::ColorCorrectionModel::setWeightCoeff", vec![(pred!(mut, ["weights_coeff"], ["const double*"]), _)]),
	void cv_ccm_ColorCorrectionModel_setWeightCoeff_const_doubleR(cv::ccm::ColorCorrectionModel* instance, const double* weights_coeff, ResultVoid* ocvrs_return) {
		try {
			instance->setWeightCoeff(*weights_coeff);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInitialMethod(INITIAL_METHOD_TYPE)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:477
	// ("cv::ccm::ColorCorrectionModel::setInitialMethod", vec![(pred!(mut, ["initial_method_type"], ["cv::ccm::INITIAL_METHOD_TYPE"]), _)]),
	void cv_ccm_ColorCorrectionModel_setInitialMethod_INITIAL_METHOD_TYPE(cv::ccm::ColorCorrectionModel* instance, cv::ccm::INITIAL_METHOD_TYPE initial_method_type, ResultVoid* ocvrs_return) {
		try {
			instance->setInitialMethod(initial_method_type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxCount(const int &)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:484
	// ("cv::ccm::ColorCorrectionModel::setMaxCount", vec![(pred!(mut, ["max_count"], ["const int*"]), _)]),
	void cv_ccm_ColorCorrectionModel_setMaxCount_const_intR(cv::ccm::ColorCorrectionModel* instance, const int* max_count, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxCount(*max_count);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEpsilon(const double &)(Indirect) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:491
	// ("cv::ccm::ColorCorrectionModel::setEpsilon", vec![(pred!(mut, ["epsilon"], ["const double*"]), _)]),
	void cv_ccm_ColorCorrectionModel_setEpsilon_const_doubleR(cv::ccm::ColorCorrectionModel* instance, const double* epsilon, ResultVoid* ocvrs_return) {
		try {
			instance->setEpsilon(*epsilon);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:494
	// ("cv::ccm::ColorCorrectionModel::run", vec![(pred!(mut, [], []), _)]),
	void cv_ccm_ColorCorrectionModel_run(cv::ccm::ColorCorrectionModel* instance, ResultVoid* ocvrs_return) {
		try {
			instance->run();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCCM()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:496
	// ("cv::ccm::ColorCorrectionModel::getCCM", vec![(pred!(const, [], []), _)]),
	void cv_ccm_ColorCorrectionModel_getCCM_const(const cv::ccm::ColorCorrectionModel* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getCCM();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLoss()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:497
	// ("cv::ccm::ColorCorrectionModel::getLoss", vec![(pred!(const, [], []), _)]),
	void cv_ccm_ColorCorrectionModel_getLoss_const(const cv::ccm::ColorCorrectionModel* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getLoss();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// get_src_rgbl()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:498
	// ("cv::ccm::ColorCorrectionModel::get_src_rgbl", vec![(pred!(const, [], []), _)]),
	void cv_ccm_ColorCorrectionModel_get_src_rgbl_const(const cv::ccm::ColorCorrectionModel* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->get_src_rgbl();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// get_dst_rgbl()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:499
	// ("cv::ccm::ColorCorrectionModel::get_dst_rgbl", vec![(pred!(const, [], []), _)]),
	void cv_ccm_ColorCorrectionModel_get_dst_rgbl_const(const cv::ccm::ColorCorrectionModel* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->get_dst_rgbl();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMask()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:500
	// ("cv::ccm::ColorCorrectionModel::getMask", vec![(pred!(const, [], []), _)]),
	void cv_ccm_ColorCorrectionModel_getMask_const(const cv::ccm::ColorCorrectionModel* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMask();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWeights()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:501
	// ("cv::ccm::ColorCorrectionModel::getWeights", vec![(pred!(const, [], []), _)]),
	void cv_ccm_ColorCorrectionModel_getWeights_const(const cv::ccm::ColorCorrectionModel* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getWeights();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// infer(const Mat &, bool)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:508
	// ("cv::ccm::ColorCorrectionModel::infer", vec![(pred!(mut, ["img", "islinear"], ["const cv::Mat*", "bool"]), _)]),
	void cv_ccm_ColorCorrectionModel_infer_const_MatR_bool(cv::ccm::ColorCorrectionModel* instance, const cv::Mat* img, bool islinear, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->infer(*img, islinear);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ccm::ColorCorrectionModel::infer(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/ccm.hpp:508
	// ("cv::ccm::ColorCorrectionModel::infer", vec![(pred!(mut, ["img"], ["const cv::Mat*"]), _)]),
	void cv_ccm_ColorCorrectionModel_infer_const_MatR(cv::ccm::ColorCorrectionModel* instance, const cv::Mat* img, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->infer(*img);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ccm::ColorCorrectionModel::delete() generated
	// ("cv::ccm::ColorCorrectionModel::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ccm_ColorCorrectionModel_delete(cv::ccm::ColorCorrectionModel* instance) {
			delete instance;
	}

	// create()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:73
	// ("cv::mcc::CChecker::create", vec![(pred!(mut, [], []), _)]),
	void cv_mcc_CChecker_create(Result<cv::Ptr<cv::mcc::CChecker>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::mcc::CChecker> ret = cv::mcc::CChecker::create();
			Ok(new cv::Ptr<cv::mcc::CChecker>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTarget(TYPECHART)(Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:83
	// ("cv::mcc::CChecker::setTarget", vec![(pred!(mut, ["_target"], ["cv::mcc::TYPECHART"]), _)]),
	void cv_mcc_CChecker_setTarget_TYPECHART(cv::mcc::CChecker* instance, cv::mcc::TYPECHART _target, ResultVoid* ocvrs_return) {
		try {
			instance->setTarget(_target);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBox(std::vector<Point2f>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:84
	// ("cv::mcc::CChecker::setBox", vec![(pred!(mut, ["_box"], ["std::vector<cv::Point2f>"]), _)]),
	void cv_mcc_CChecker_setBox_vectorLPoint2fG(cv::mcc::CChecker* instance, std::vector<cv::Point2f>* _box, ResultVoid* ocvrs_return) {
		try {
			instance->setBox(*_box);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setChartsRGB(Mat)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:85
	// ("cv::mcc::CChecker::setChartsRGB", vec![(pred!(mut, ["_chartsRGB"], ["cv::Mat"]), _)]),
	void cv_mcc_CChecker_setChartsRGB_Mat(cv::mcc::CChecker* instance, cv::Mat* _chartsRGB, ResultVoid* ocvrs_return) {
		try {
			instance->setChartsRGB(*_chartsRGB);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setChartsYCbCr(Mat)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:86
	// ("cv::mcc::CChecker::setChartsYCbCr", vec![(pred!(mut, ["_chartsYCbCr"], ["cv::Mat"]), _)]),
	void cv_mcc_CChecker_setChartsYCbCr_Mat(cv::mcc::CChecker* instance, cv::Mat* _chartsYCbCr, ResultVoid* ocvrs_return) {
		try {
			instance->setChartsYCbCr(*_chartsYCbCr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCost(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:87
	// ("cv::mcc::CChecker::setCost", vec![(pred!(mut, ["_cost"], ["float"]), _)]),
	void cv_mcc_CChecker_setCost_float(cv::mcc::CChecker* instance, float _cost, ResultVoid* ocvrs_return) {
		try {
			instance->setCost(_cost);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCenter(Point2f)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:88
	// ("cv::mcc::CChecker::setCenter", vec![(pred!(mut, ["_center"], ["cv::Point2f"]), _)]),
	void cv_mcc_CChecker_setCenter_Point2f(cv::mcc::CChecker* instance, cv::Point2f* _center, ResultVoid* ocvrs_return) {
		try {
			instance->setCenter(*_center);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTarget()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:90
	// ("cv::mcc::CChecker::getTarget", vec![(pred!(mut, [], []), _)]),
	void cv_mcc_CChecker_getTarget(cv::mcc::CChecker* instance, Result<cv::mcc::TYPECHART>* ocvrs_return) {
		try {
			cv::mcc::TYPECHART ret = instance->getTarget();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBox()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:91
	// ("cv::mcc::CChecker::getBox", vec![(pred!(mut, [], []), _)]),
	void cv_mcc_CChecker_getBox(cv::mcc::CChecker* instance, Result<std::vector<cv::Point2f>*>* ocvrs_return) {
		try {
			std::vector<cv::Point2f> ret = instance->getBox();
			Ok(new std::vector<cv::Point2f>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getColorCharts()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:99
	// ("cv::mcc::CChecker::getColorCharts", vec![(pred!(mut, [], []), _)]),
	void cv_mcc_CChecker_getColorCharts(cv::mcc::CChecker* instance, Result<std::vector<cv::Point2f>*>* ocvrs_return) {
		try {
			std::vector<cv::Point2f> ret = instance->getColorCharts();
			Ok(new std::vector<cv::Point2f>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getChartsRGB()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:101
	// ("cv::mcc::CChecker::getChartsRGB", vec![(pred!(mut, [], []), _)]),
	void cv_mcc_CChecker_getChartsRGB(cv::mcc::CChecker* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getChartsRGB();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getChartsYCbCr()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:102
	// ("cv::mcc::CChecker::getChartsYCbCr", vec![(pred!(mut, [], []), _)]),
	void cv_mcc_CChecker_getChartsYCbCr(cv::mcc::CChecker* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getChartsYCbCr();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCost()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:103
	// ("cv::mcc::CChecker::getCost", vec![(pred!(mut, [], []), _)]),
	void cv_mcc_CChecker_getCost(cv::mcc::CChecker* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getCost();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCenter()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:104
	// ("cv::mcc::CChecker::getCenter", vec![(pred!(mut, [], []), _)]),
	void cv_mcc_CChecker_getCenter(cv::mcc::CChecker* instance, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->getCenter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::mcc::CChecker::delete() generated
	// ("cv::mcc::CChecker::delete", vec![(pred!(mut, [], []), _)]),
	void cv_mcc_CChecker_delete(cv::mcc::CChecker* instance) {
			delete instance;
	}

	// setNet(dnn::Net)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:144
	// ("cv::mcc::CCheckerDetector::setNet", vec![(pred!(mut, ["net"], ["cv::dnn::Net"]), _)]),
	void cv_mcc_CCheckerDetector_setNet_Net(cv::mcc::CCheckerDetector* instance, cv::dnn::Net* net, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->setNet(*net);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// process(InputArray, const TYPECHART, const std::vector<Rect> &, const int, bool, const Ptr<DetectorParameters> &)(InputArray, Enum, CppPassByVoidPtr, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:167
	// ("cv::mcc::CCheckerDetector::process", vec![(pred!(mut, ["image", "chartType", "regionsOfInterest", "nc", "useNet", "params"], ["const cv::_InputArray*", "const cv::mcc::TYPECHART", "const std::vector<cv::Rect>*", "const int", "bool", "const cv::Ptr<cv::mcc::DetectorParameters>*"]), _)]),
	void cv_mcc_CCheckerDetector_process_const__InputArrayR_const_TYPECHART_const_vectorLRectGR_const_int_bool_const_PtrLDetectorParametersGR(cv::mcc::CCheckerDetector* instance, const cv::_InputArray* image, const cv::mcc::TYPECHART chartType, const std::vector<cv::Rect>* regionsOfInterest, const int nc, bool useNet, const cv::Ptr<cv::mcc::DetectorParameters>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->process(*image, chartType, *regionsOfInterest, nc, useNet, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::mcc::CCheckerDetector::process(InputArray, Enum, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:167
	// ("cv::mcc::CCheckerDetector::process", vec![(pred!(mut, ["image", "chartType", "regionsOfInterest"], ["const cv::_InputArray*", "const cv::mcc::TYPECHART", "const std::vector<cv::Rect>*"]), _)]),
	void cv_mcc_CCheckerDetector_process_const__InputArrayR_const_TYPECHART_const_vectorLRectGR(cv::mcc::CCheckerDetector* instance, const cv::_InputArray* image, const cv::mcc::TYPECHART chartType, const std::vector<cv::Rect>* regionsOfInterest, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->process(*image, chartType, *regionsOfInterest);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// process(InputArray, const TYPECHART, const int, bool, const Ptr<DetectorParameters> &)(InputArray, Enum, Primitive, Primitive, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:195
	// ("cv::mcc::CCheckerDetector::process", vec![(pred!(mut, ["image", "chartType", "nc", "useNet", "params"], ["const cv::_InputArray*", "const cv::mcc::TYPECHART", "const int", "bool", "const cv::Ptr<cv::mcc::DetectorParameters>*"]), _)]),
	void cv_mcc_CCheckerDetector_process_const__InputArrayR_const_TYPECHART_const_int_bool_const_PtrLDetectorParametersGR(cv::mcc::CCheckerDetector* instance, const cv::_InputArray* image, const cv::mcc::TYPECHART chartType, const int nc, bool useNet, const cv::Ptr<cv::mcc::DetectorParameters>* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->process(*image, chartType, nc, useNet, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::mcc::CCheckerDetector::process(InputArray, Enum) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:195
	// ("cv::mcc::CCheckerDetector::process", vec![(pred!(mut, ["image", "chartType"], ["const cv::_InputArray*", "const cv::mcc::TYPECHART"]), _)]),
	void cv_mcc_CCheckerDetector_process_const__InputArrayR_const_TYPECHART(cv::mcc::CCheckerDetector* instance, const cv::_InputArray* image, const cv::mcc::TYPECHART chartType, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->process(*image, chartType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBestColorChecker()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:204
	// ("cv::mcc::CCheckerDetector::getBestColorChecker", vec![(pred!(mut, [], []), _)]),
	void cv_mcc_CCheckerDetector_getBestColorChecker(cv::mcc::CCheckerDetector* instance, Result<cv::Ptr<cv::mcc::CChecker>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::mcc::CChecker> ret = instance->getBestColorChecker();
			Ok(new cv::Ptr<cv::mcc::CChecker>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getListColorChecker()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:209
	// ("cv::mcc::CCheckerDetector::getListColorChecker", vec![(pred!(mut, [], []), _)]),
	void cv_mcc_CCheckerDetector_getListColorChecker(cv::mcc::CCheckerDetector* instance, Result<std::vector<cv::Ptr<cv::mcc::CChecker>>*>* ocvrs_return) {
		try {
			std::vector<cv::Ptr<cv::mcc::CChecker>> ret = instance->getListColorChecker();
			Ok(new std::vector<cv::Ptr<cv::mcc::CChecker>>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:214
	// ("cv::mcc::CCheckerDetector::create", vec![(pred!(mut, [], []), _)]),
	void cv_mcc_CCheckerDetector_create(Result<cv::Ptr<cv::mcc::CCheckerDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::mcc::CCheckerDetector> ret = cv::mcc::CCheckerDetector::create();
			Ok(new cv::Ptr<cv::mcc::CCheckerDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::mcc::CCheckerDetector::to_Algorithm() generated
	// ("cv::mcc::CCheckerDetector::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_mcc_CCheckerDetector_to_Algorithm(cv::mcc::CCheckerDetector* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::mcc::CCheckerDetector::delete() generated
	// ("cv::mcc::CCheckerDetector::delete", vec![(pred!(mut, [], []), _)]),
	void cv_mcc_CCheckerDetector_delete(cv::mcc::CCheckerDetector* instance) {
			delete instance;
	}

	// draw(InputOutputArray)(InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:129
	// ("cv::mcc::CCheckerDraw::draw", vec![(pred!(mut, ["img"], ["const cv::_InputOutputArray*"]), _)]),
	void cv_mcc_CCheckerDraw_draw_const__InputOutputArrayR(cv::mcc::CCheckerDraw* instance, const cv::_InputOutputArray* img, ResultVoid* ocvrs_return) {
		try {
			instance->draw(*img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(Ptr<CChecker>, cv::Scalar, int)(CppPassByVoidPtr, SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:138
	// ("cv::mcc::CCheckerDraw::create", vec![(pred!(mut, ["pChecker", "color", "thickness"], ["cv::Ptr<cv::mcc::CChecker>", "cv::Scalar", "int"]), _)]),
	void cv_mcc_CCheckerDraw_create_PtrLCCheckerG_Scalar_int(cv::Ptr<cv::mcc::CChecker>* pChecker, cv::Scalar* color, int thickness, Result<cv::Ptr<cv::mcc::CCheckerDraw>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::mcc::CCheckerDraw> ret = cv::mcc::CCheckerDraw::create(*pChecker, *color, thickness);
			Ok(new cv::Ptr<cv::mcc::CCheckerDraw>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::mcc::CCheckerDraw::create(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_model.hpp:138
	// ("cv::mcc::CCheckerDraw::create", vec![(pred!(mut, ["pChecker"], ["cv::Ptr<cv::mcc::CChecker>"]), _)]),
	void cv_mcc_CCheckerDraw_create_PtrLCCheckerG(cv::Ptr<cv::mcc::CChecker>* pChecker, Result<cv::Ptr<cv::mcc::CCheckerDraw>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::mcc::CCheckerDraw> ret = cv::mcc::CCheckerDraw::create(*pChecker);
			Ok(new cv::Ptr<cv::mcc::CCheckerDraw>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::mcc::CCheckerDraw::delete() generated
	// ("cv::mcc::CCheckerDraw::delete", vec![(pred!(mut, [], []), _)]),
	void cv_mcc_CCheckerDraw_delete(cv::mcc::CCheckerDraw* instance) {
			delete instance;
	}

	// DetectorParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:102
	// ("cv::mcc::DetectorParameters::DetectorParameters", vec![(pred!(mut, [], []), _)]),
	void cv_mcc_DetectorParameters_DetectorParameters(Result<cv::mcc::DetectorParameters*>* ocvrs_return) {
		try {
			cv::mcc::DetectorParameters* ret = new cv::mcc::DetectorParameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:104
	// ("cv::mcc::DetectorParameters::create", vec![(pred!(mut, [], []), _)]),
	void cv_mcc_DetectorParameters_create(Result<cv::Ptr<cv::mcc::DetectorParameters>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::mcc::DetectorParameters> ret = cv::mcc::DetectorParameters::create();
			Ok(new cv::Ptr<cv::mcc::DetectorParameters>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::mcc::DetectorParameters::adaptiveThreshWinSizeMin() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:106
	// ("cv::mcc::DetectorParameters::adaptiveThreshWinSizeMin", vec![(pred!(const, [], []), _)]),
	int cv_mcc_DetectorParameters_propAdaptiveThreshWinSizeMin_const(const cv::mcc::DetectorParameters* instance) {
			int ret = instance->adaptiveThreshWinSizeMin;
			return ret;
	}

	// cv::mcc::DetectorParameters::setAdaptiveThreshWinSizeMin(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:106
	// ("cv::mcc::DetectorParameters::setAdaptiveThreshWinSizeMin", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_mcc_DetectorParameters_propAdaptiveThreshWinSizeMin_const_int(cv::mcc::DetectorParameters* instance, const int val) {
			instance->adaptiveThreshWinSizeMin = val;
	}

	// cv::mcc::DetectorParameters::adaptiveThreshWinSizeMax() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:107
	// ("cv::mcc::DetectorParameters::adaptiveThreshWinSizeMax", vec![(pred!(const, [], []), _)]),
	int cv_mcc_DetectorParameters_propAdaptiveThreshWinSizeMax_const(const cv::mcc::DetectorParameters* instance) {
			int ret = instance->adaptiveThreshWinSizeMax;
			return ret;
	}

	// cv::mcc::DetectorParameters::setAdaptiveThreshWinSizeMax(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:107
	// ("cv::mcc::DetectorParameters::setAdaptiveThreshWinSizeMax", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_mcc_DetectorParameters_propAdaptiveThreshWinSizeMax_const_int(cv::mcc::DetectorParameters* instance, const int val) {
			instance->adaptiveThreshWinSizeMax = val;
	}

	// cv::mcc::DetectorParameters::adaptiveThreshWinSizeStep() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:108
	// ("cv::mcc::DetectorParameters::adaptiveThreshWinSizeStep", vec![(pred!(const, [], []), _)]),
	int cv_mcc_DetectorParameters_propAdaptiveThreshWinSizeStep_const(const cv::mcc::DetectorParameters* instance) {
			int ret = instance->adaptiveThreshWinSizeStep;
			return ret;
	}

	// cv::mcc::DetectorParameters::setAdaptiveThreshWinSizeStep(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:108
	// ("cv::mcc::DetectorParameters::setAdaptiveThreshWinSizeStep", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_mcc_DetectorParameters_propAdaptiveThreshWinSizeStep_const_int(cv::mcc::DetectorParameters* instance, const int val) {
			instance->adaptiveThreshWinSizeStep = val;
	}

	// cv::mcc::DetectorParameters::adaptiveThreshConstant() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:109
	// ("cv::mcc::DetectorParameters::adaptiveThreshConstant", vec![(pred!(const, [], []), _)]),
	double cv_mcc_DetectorParameters_propAdaptiveThreshConstant_const(const cv::mcc::DetectorParameters* instance) {
			double ret = instance->adaptiveThreshConstant;
			return ret;
	}

	// cv::mcc::DetectorParameters::setAdaptiveThreshConstant(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:109
	// ("cv::mcc::DetectorParameters::setAdaptiveThreshConstant", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_mcc_DetectorParameters_propAdaptiveThreshConstant_const_double(cv::mcc::DetectorParameters* instance, const double val) {
			instance->adaptiveThreshConstant = val;
	}

	// cv::mcc::DetectorParameters::minContoursAreaRate() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:110
	// ("cv::mcc::DetectorParameters::minContoursAreaRate", vec![(pred!(const, [], []), _)]),
	double cv_mcc_DetectorParameters_propMinContoursAreaRate_const(const cv::mcc::DetectorParameters* instance) {
			double ret = instance->minContoursAreaRate;
			return ret;
	}

	// cv::mcc::DetectorParameters::setMinContoursAreaRate(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:110
	// ("cv::mcc::DetectorParameters::setMinContoursAreaRate", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_mcc_DetectorParameters_propMinContoursAreaRate_const_double(cv::mcc::DetectorParameters* instance, const double val) {
			instance->minContoursAreaRate = val;
	}

	// cv::mcc::DetectorParameters::minContoursArea() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:111
	// ("cv::mcc::DetectorParameters::minContoursArea", vec![(pred!(const, [], []), _)]),
	double cv_mcc_DetectorParameters_propMinContoursArea_const(const cv::mcc::DetectorParameters* instance) {
			double ret = instance->minContoursArea;
			return ret;
	}

	// cv::mcc::DetectorParameters::setMinContoursArea(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:111
	// ("cv::mcc::DetectorParameters::setMinContoursArea", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_mcc_DetectorParameters_propMinContoursArea_const_double(cv::mcc::DetectorParameters* instance, const double val) {
			instance->minContoursArea = val;
	}

	// cv::mcc::DetectorParameters::confidenceThreshold() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:112
	// ("cv::mcc::DetectorParameters::confidenceThreshold", vec![(pred!(const, [], []), _)]),
	double cv_mcc_DetectorParameters_propConfidenceThreshold_const(const cv::mcc::DetectorParameters* instance) {
			double ret = instance->confidenceThreshold;
			return ret;
	}

	// cv::mcc::DetectorParameters::setConfidenceThreshold(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:112
	// ("cv::mcc::DetectorParameters::setConfidenceThreshold", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_mcc_DetectorParameters_propConfidenceThreshold_const_double(cv::mcc::DetectorParameters* instance, const double val) {
			instance->confidenceThreshold = val;
	}

	// cv::mcc::DetectorParameters::minContourSolidity() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:113
	// ("cv::mcc::DetectorParameters::minContourSolidity", vec![(pred!(const, [], []), _)]),
	double cv_mcc_DetectorParameters_propMinContourSolidity_const(const cv::mcc::DetectorParameters* instance) {
			double ret = instance->minContourSolidity;
			return ret;
	}

	// cv::mcc::DetectorParameters::setMinContourSolidity(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:113
	// ("cv::mcc::DetectorParameters::setMinContourSolidity", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_mcc_DetectorParameters_propMinContourSolidity_const_double(cv::mcc::DetectorParameters* instance, const double val) {
			instance->minContourSolidity = val;
	}

	// cv::mcc::DetectorParameters::findCandidatesApproxPolyDPEpsMultiplier() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:114
	// ("cv::mcc::DetectorParameters::findCandidatesApproxPolyDPEpsMultiplier", vec![(pred!(const, [], []), _)]),
	double cv_mcc_DetectorParameters_propFindCandidatesApproxPolyDPEpsMultiplier_const(const cv::mcc::DetectorParameters* instance) {
			double ret = instance->findCandidatesApproxPolyDPEpsMultiplier;
			return ret;
	}

	// cv::mcc::DetectorParameters::setFindCandidatesApproxPolyDPEpsMultiplier(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:114
	// ("cv::mcc::DetectorParameters::setFindCandidatesApproxPolyDPEpsMultiplier", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_mcc_DetectorParameters_propFindCandidatesApproxPolyDPEpsMultiplier_const_double(cv::mcc::DetectorParameters* instance, const double val) {
			instance->findCandidatesApproxPolyDPEpsMultiplier = val;
	}

	// cv::mcc::DetectorParameters::borderWidth() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:115
	// ("cv::mcc::DetectorParameters::borderWidth", vec![(pred!(const, [], []), _)]),
	int cv_mcc_DetectorParameters_propBorderWidth_const(const cv::mcc::DetectorParameters* instance) {
			int ret = instance->borderWidth;
			return ret;
	}

	// cv::mcc::DetectorParameters::setBorderWidth(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:115
	// ("cv::mcc::DetectorParameters::setBorderWidth", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_mcc_DetectorParameters_propBorderWidth_const_int(cv::mcc::DetectorParameters* instance, const int val) {
			instance->borderWidth = val;
	}

	// cv::mcc::DetectorParameters::B0factor() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:116
	// ("cv::mcc::DetectorParameters::B0factor", vec![(pred!(const, [], []), _)]),
	float cv_mcc_DetectorParameters_propB0factor_const(const cv::mcc::DetectorParameters* instance) {
			float ret = instance->B0factor;
			return ret;
	}

	// cv::mcc::DetectorParameters::setB0factor(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:116
	// ("cv::mcc::DetectorParameters::setB0factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_mcc_DetectorParameters_propB0factor_const_float(cv::mcc::DetectorParameters* instance, const float val) {
			instance->B0factor = val;
	}

	// cv::mcc::DetectorParameters::maxError() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:117
	// ("cv::mcc::DetectorParameters::maxError", vec![(pred!(const, [], []), _)]),
	float cv_mcc_DetectorParameters_propMaxError_const(const cv::mcc::DetectorParameters* instance) {
			float ret = instance->maxError;
			return ret;
	}

	// cv::mcc::DetectorParameters::setMaxError(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:117
	// ("cv::mcc::DetectorParameters::setMaxError", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_mcc_DetectorParameters_propMaxError_const_float(cv::mcc::DetectorParameters* instance, const float val) {
			instance->maxError = val;
	}

	// cv::mcc::DetectorParameters::minContourPointsAllowed() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:118
	// ("cv::mcc::DetectorParameters::minContourPointsAllowed", vec![(pred!(const, [], []), _)]),
	int cv_mcc_DetectorParameters_propMinContourPointsAllowed_const(const cv::mcc::DetectorParameters* instance) {
			int ret = instance->minContourPointsAllowed;
			return ret;
	}

	// cv::mcc::DetectorParameters::setMinContourPointsAllowed(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:118
	// ("cv::mcc::DetectorParameters::setMinContourPointsAllowed", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_mcc_DetectorParameters_propMinContourPointsAllowed_const_int(cv::mcc::DetectorParameters* instance, const int val) {
			instance->minContourPointsAllowed = val;
	}

	// cv::mcc::DetectorParameters::minContourLengthAllowed() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:119
	// ("cv::mcc::DetectorParameters::minContourLengthAllowed", vec![(pred!(const, [], []), _)]),
	int cv_mcc_DetectorParameters_propMinContourLengthAllowed_const(const cv::mcc::DetectorParameters* instance) {
			int ret = instance->minContourLengthAllowed;
			return ret;
	}

	// cv::mcc::DetectorParameters::setMinContourLengthAllowed(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:119
	// ("cv::mcc::DetectorParameters::setMinContourLengthAllowed", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_mcc_DetectorParameters_propMinContourLengthAllowed_const_int(cv::mcc::DetectorParameters* instance, const int val) {
			instance->minContourLengthAllowed = val;
	}

	// cv::mcc::DetectorParameters::minInterContourDistance() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:120
	// ("cv::mcc::DetectorParameters::minInterContourDistance", vec![(pred!(const, [], []), _)]),
	int cv_mcc_DetectorParameters_propMinInterContourDistance_const(const cv::mcc::DetectorParameters* instance) {
			int ret = instance->minInterContourDistance;
			return ret;
	}

	// cv::mcc::DetectorParameters::setMinInterContourDistance(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:120
	// ("cv::mcc::DetectorParameters::setMinInterContourDistance", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_mcc_DetectorParameters_propMinInterContourDistance_const_int(cv::mcc::DetectorParameters* instance, const int val) {
			instance->minInterContourDistance = val;
	}

	// cv::mcc::DetectorParameters::minInterCheckerDistance() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:121
	// ("cv::mcc::DetectorParameters::minInterCheckerDistance", vec![(pred!(const, [], []), _)]),
	int cv_mcc_DetectorParameters_propMinInterCheckerDistance_const(const cv::mcc::DetectorParameters* instance) {
			int ret = instance->minInterCheckerDistance;
			return ret;
	}

	// cv::mcc::DetectorParameters::setMinInterCheckerDistance(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:121
	// ("cv::mcc::DetectorParameters::setMinInterCheckerDistance", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_mcc_DetectorParameters_propMinInterCheckerDistance_const_int(cv::mcc::DetectorParameters* instance, const int val) {
			instance->minInterCheckerDistance = val;
	}

	// cv::mcc::DetectorParameters::minImageSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:122
	// ("cv::mcc::DetectorParameters::minImageSize", vec![(pred!(const, [], []), _)]),
	int cv_mcc_DetectorParameters_propMinImageSize_const(const cv::mcc::DetectorParameters* instance) {
			int ret = instance->minImageSize;
			return ret;
	}

	// cv::mcc::DetectorParameters::setMinImageSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:122
	// ("cv::mcc::DetectorParameters::setMinImageSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_mcc_DetectorParameters_propMinImageSize_const_int(cv::mcc::DetectorParameters* instance, const int val) {
			instance->minImageSize = val;
	}

	// cv::mcc::DetectorParameters::minGroupSize() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:123
	// ("cv::mcc::DetectorParameters::minGroupSize", vec![(pred!(const, [], []), _)]),
	unsigned int cv_mcc_DetectorParameters_propMinGroupSize_const(const cv::mcc::DetectorParameters* instance) {
			unsigned int ret = instance->minGroupSize;
			return ret;
	}

	// cv::mcc::DetectorParameters::setMinGroupSize(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/mcc/checker_detector.hpp:123
	// ("cv::mcc::DetectorParameters::setMinGroupSize", vec![(pred!(mut, ["val"], ["const unsigned int"]), _)]),
	void cv_mcc_DetectorParameters_propMinGroupSize_const_unsigned_int(cv::mcc::DetectorParameters* instance, const unsigned int val) {
			instance->minGroupSize = val;
	}

	// cv::mcc::DetectorParameters::delete() generated
	// ("cv::mcc::DetectorParameters::delete", vec![(pred!(mut, [], []), _)]),
	void cv_mcc_DetectorParameters_delete(cv::mcc::DetectorParameters* instance) {
			delete instance;
	}

}
