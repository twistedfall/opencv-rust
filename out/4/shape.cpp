#include "ocvrs_common.hpp"
#include <opencv2/shape.hpp>
#include "shape_types.hpp"

extern "C" {
	// EMDL1(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/emdL1.hpp:66
	// ("cv::EMDL1", vec![(pred!(mut, ["signature1", "signature2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_EMDL1_const__InputArrayR_const__InputArrayR(const cv::_InputArray* signature1, const cv::_InputArray* signature2, Result<float>* ocvrs_return) {
		try {
			float ret = cv::EMDL1(*signature1, *signature2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createAffineTransformer(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_transformer.hpp:127
	// ("cv::createAffineTransformer", vec![(pred!(mut, ["fullAffine"], ["bool"]), _)]),
	void cv_createAffineTransformer_bool(bool fullAffine, Result<cv::Ptr<cv::AffineTransformer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::AffineTransformer> ret = cv::createAffineTransformer(fullAffine);
			Ok(new cv::Ptr<cv::AffineTransformer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::createChiHistogramCostExtractor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/hist_cost.hpp:98
	// ("cv::createChiHistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
	void cv_createChiHistogramCostExtractor(Result<cv::Ptr<cv::HistogramCostExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = cv::createChiHistogramCostExtractor();
			Ok(new cv::Ptr<cv::HistogramCostExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createChiHistogramCostExtractor(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/hist_cost.hpp:98
	// ("cv::createChiHistogramCostExtractor", vec![(pred!(mut, ["nDummies", "defaultCost"], ["int", "float"]), _)]),
	void cv_createChiHistogramCostExtractor_int_float(int nDummies, float defaultCost, Result<cv::Ptr<cv::HistogramCostExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = cv::createChiHistogramCostExtractor(nDummies, defaultCost);
			Ok(new cv::Ptr<cv::HistogramCostExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::createEMDHistogramCostExtractor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/hist_cost.hpp:91
	// ("cv::createEMDHistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
	void cv_createEMDHistogramCostExtractor(Result<cv::Ptr<cv::HistogramCostExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = cv::createEMDHistogramCostExtractor();
			Ok(new cv::Ptr<cv::HistogramCostExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createEMDHistogramCostExtractor(int, int, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/hist_cost.hpp:91
	// ("cv::createEMDHistogramCostExtractor", vec![(pred!(mut, ["flag", "nDummies", "defaultCost"], ["int", "int", "float"]), _)]),
	void cv_createEMDHistogramCostExtractor_int_int_float(int flag, int nDummies, float defaultCost, Result<cv::Ptr<cv::HistogramCostExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = cv::createEMDHistogramCostExtractor(flag, nDummies, defaultCost);
			Ok(new cv::Ptr<cv::HistogramCostExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::createEMDL1HistogramCostExtractor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/hist_cost.hpp:106
	// ("cv::createEMDL1HistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
	void cv_createEMDL1HistogramCostExtractor(Result<cv::Ptr<cv::HistogramCostExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = cv::createEMDL1HistogramCostExtractor();
			Ok(new cv::Ptr<cv::HistogramCostExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createEMDL1HistogramCostExtractor(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/hist_cost.hpp:106
	// ("cv::createEMDL1HistogramCostExtractor", vec![(pred!(mut, ["nDummies", "defaultCost"], ["int", "float"]), _)]),
	void cv_createEMDL1HistogramCostExtractor_int_float(int nDummies, float defaultCost, Result<cv::Ptr<cv::HistogramCostExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = cv::createEMDL1HistogramCostExtractor(nDummies, defaultCost);
			Ok(new cv::Ptr<cv::HistogramCostExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::createHausdorffDistanceExtractor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:222
	// ("cv::createHausdorffDistanceExtractor", vec![(pred!(mut, [], []), _)]),
	void cv_createHausdorffDistanceExtractor(Result<cv::Ptr<cv::HausdorffDistanceExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::HausdorffDistanceExtractor> ret = cv::createHausdorffDistanceExtractor();
			Ok(new cv::Ptr<cv::HausdorffDistanceExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createHausdorffDistanceExtractor(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:222
	// ("cv::createHausdorffDistanceExtractor", vec![(pred!(mut, ["distanceFlag", "rankProp"], ["int", "float"]), _)]),
	void cv_createHausdorffDistanceExtractor_int_float(int distanceFlag, float rankProp, Result<cv::Ptr<cv::HausdorffDistanceExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::HausdorffDistanceExtractor> ret = cv::createHausdorffDistanceExtractor(distanceFlag, rankProp);
			Ok(new cv::Ptr<cv::HausdorffDistanceExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::createNormHistogramCostExtractor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/hist_cost.hpp:79
	// ("cv::createNormHistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
	void cv_createNormHistogramCostExtractor(Result<cv::Ptr<cv::HistogramCostExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = cv::createNormHistogramCostExtractor();
			Ok(new cv::Ptr<cv::HistogramCostExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createNormHistogramCostExtractor(int, int, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/hist_cost.hpp:79
	// ("cv::createNormHistogramCostExtractor", vec![(pred!(mut, ["flag", "nDummies", "defaultCost"], ["int", "int", "float"]), _)]),
	void cv_createNormHistogramCostExtractor_int_int_float(int flag, int nDummies, float defaultCost, Result<cv::Ptr<cv::HistogramCostExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = cv::createNormHistogramCostExtractor(flag, nDummies, defaultCost);
			Ok(new cv::Ptr<cv::HistogramCostExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::createShapeContextDistanceExtractor() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:187
	// ("cv::createShapeContextDistanceExtractor", vec![(pred!(mut, [], []), _)]),
	void cv_createShapeContextDistanceExtractor(Result<cv::Ptr<cv::ShapeContextDistanceExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ShapeContextDistanceExtractor> ret = cv::createShapeContextDistanceExtractor();
			Ok(new cv::Ptr<cv::ShapeContextDistanceExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createShapeContextDistanceExtractor(int, int, float, float, int, const Ptr<HistogramCostExtractor> &, const Ptr<ShapeTransformer> &)(Primitive, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:187
	// ("cv::createShapeContextDistanceExtractor", vec![(pred!(mut, ["nAngularBins", "nRadialBins", "innerRadius", "outerRadius", "iterations", "comparer", "transformer"], ["int", "int", "float", "float", "int", "const cv::Ptr<cv::HistogramCostExtractor>*", "const cv::Ptr<cv::ShapeTransformer>*"]), _)]),
	void cv_createShapeContextDistanceExtractor_int_int_float_float_int_const_PtrLHistogramCostExtractorGR_const_PtrLShapeTransformerGR(int nAngularBins, int nRadialBins, float innerRadius, float outerRadius, int iterations, const cv::Ptr<cv::HistogramCostExtractor>* comparer, const cv::Ptr<cv::ShapeTransformer>* transformer, Result<cv::Ptr<cv::ShapeContextDistanceExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ShapeContextDistanceExtractor> ret = cv::createShapeContextDistanceExtractor(nAngularBins, nRadialBins, innerRadius, outerRadius, iterations, *comparer, *transformer);
			Ok(new cv::Ptr<cv::ShapeContextDistanceExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::createThinPlateSplineShapeTransformer() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_transformer.hpp:112
	// ("cv::createThinPlateSplineShapeTransformer", vec![(pred!(mut, [], []), _)]),
	void cv_createThinPlateSplineShapeTransformer(Result<cv::Ptr<cv::ThinPlateSplineShapeTransformer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ThinPlateSplineShapeTransformer> ret = cv::createThinPlateSplineShapeTransformer();
			Ok(new cv::Ptr<cv::ThinPlateSplineShapeTransformer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createThinPlateSplineShapeTransformer(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_transformer.hpp:112
	// ("cv::createThinPlateSplineShapeTransformer", vec![(pred!(mut, ["regularizationParameter"], ["double"]), _)]),
	void cv_createThinPlateSplineShapeTransformer_double(double regularizationParameter, Result<cv::Ptr<cv::ThinPlateSplineShapeTransformer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ThinPlateSplineShapeTransformer> ret = cv::createThinPlateSplineShapeTransformer(regularizationParameter);
			Ok(new cv::Ptr<cv::ThinPlateSplineShapeTransformer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setFullAffine(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_transformer.hpp:122
	// ("cv::AffineTransformer::setFullAffine", vec![(pred!(mut, ["fullAffine"], ["bool"]), _)]),
	void cv_AffineTransformer_setFullAffine_bool(cv::AffineTransformer* instance, bool fullAffine, ResultVoid* ocvrs_return) {
		try {
			instance->setFullAffine(fullAffine);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getFullAffine()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_transformer.hpp:123
	// ("cv::AffineTransformer::getFullAffine", vec![(pred!(const, [], []), _)]),
	void cv_AffineTransformer_getFullAffine_const(const cv::AffineTransformer* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getFullAffine();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::AffineTransformer::to_Algorithm() generated
	// ("cv::AffineTransformer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_AffineTransformer_to_Algorithm(cv::AffineTransformer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::AffineTransformer::to_ShapeTransformer() generated
	// ("cv::AffineTransformer::to_ShapeTransformer", vec![(pred!(mut, [], []), _)]),
	cv::ShapeTransformer* cv_AffineTransformer_to_ShapeTransformer(cv::AffineTransformer* instance) {
			return dynamic_cast<cv::ShapeTransformer*>(instance);
	}

	// cv::AffineTransformer::delete() generated
	// ("cv::AffineTransformer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_AffineTransformer_delete(cv::AffineTransformer* instance) {
			delete instance;
	}

	// cv::ChiHistogramCostExtractor::to_Algorithm() generated
	// ("cv::ChiHistogramCostExtractor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ChiHistogramCostExtractor_to_Algorithm(cv::ChiHistogramCostExtractor* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ChiHistogramCostExtractor::to_HistogramCostExtractor() generated
	// ("cv::ChiHistogramCostExtractor::to_HistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
	cv::HistogramCostExtractor* cv_ChiHistogramCostExtractor_to_HistogramCostExtractor(cv::ChiHistogramCostExtractor* instance) {
			return dynamic_cast<cv::HistogramCostExtractor*>(instance);
	}

	// cv::ChiHistogramCostExtractor::delete() generated
	// ("cv::ChiHistogramCostExtractor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ChiHistogramCostExtractor_delete(cv::ChiHistogramCostExtractor* instance) {
			delete instance;
	}

	// setNormFlag(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/hist_cost.hpp:86
	// ("cv::EMDHistogramCostExtractor::setNormFlag", vec![(pred!(mut, ["flag"], ["int"]), _)]),
	void cv_EMDHistogramCostExtractor_setNormFlag_int(cv::EMDHistogramCostExtractor* instance, int flag, ResultVoid* ocvrs_return) {
		try {
			instance->setNormFlag(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormFlag()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/hist_cost.hpp:87
	// ("cv::EMDHistogramCostExtractor::getNormFlag", vec![(pred!(const, [], []), _)]),
	void cv_EMDHistogramCostExtractor_getNormFlag_const(const cv::EMDHistogramCostExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNormFlag();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::EMDHistogramCostExtractor::to_Algorithm() generated
	// ("cv::EMDHistogramCostExtractor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_EMDHistogramCostExtractor_to_Algorithm(cv::EMDHistogramCostExtractor* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::EMDHistogramCostExtractor::to_HistogramCostExtractor() generated
	// ("cv::EMDHistogramCostExtractor::to_HistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
	cv::HistogramCostExtractor* cv_EMDHistogramCostExtractor_to_HistogramCostExtractor(cv::EMDHistogramCostExtractor* instance) {
			return dynamic_cast<cv::HistogramCostExtractor*>(instance);
	}

	// cv::EMDHistogramCostExtractor::delete() generated
	// ("cv::EMDHistogramCostExtractor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_EMDHistogramCostExtractor_delete(cv::EMDHistogramCostExtractor* instance) {
			delete instance;
	}

	// cv::EMDL1HistogramCostExtractor::to_Algorithm() generated
	// ("cv::EMDL1HistogramCostExtractor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_EMDL1HistogramCostExtractor_to_Algorithm(cv::EMDL1HistogramCostExtractor* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::EMDL1HistogramCostExtractor::to_HistogramCostExtractor() generated
	// ("cv::EMDL1HistogramCostExtractor::to_HistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
	cv::HistogramCostExtractor* cv_EMDL1HistogramCostExtractor_to_HistogramCostExtractor(cv::EMDL1HistogramCostExtractor* instance) {
			return dynamic_cast<cv::HistogramCostExtractor*>(instance);
	}

	// cv::EMDL1HistogramCostExtractor::delete() generated
	// ("cv::EMDL1HistogramCostExtractor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_EMDL1HistogramCostExtractor_delete(cv::EMDL1HistogramCostExtractor* instance) {
			delete instance;
	}

	// setDistanceFlag(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:208
	// ("cv::HausdorffDistanceExtractor::setDistanceFlag", vec![(pred!(mut, ["distanceFlag"], ["int"]), _)]),
	void cv_HausdorffDistanceExtractor_setDistanceFlag_int(cv::HausdorffDistanceExtractor* instance, int distanceFlag, ResultVoid* ocvrs_return) {
		try {
			instance->setDistanceFlag(distanceFlag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDistanceFlag()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:209
	// ("cv::HausdorffDistanceExtractor::getDistanceFlag", vec![(pred!(const, [], []), _)]),
	void cv_HausdorffDistanceExtractor_getDistanceFlag_const(const cv::HausdorffDistanceExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDistanceFlag();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRankProportion(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:217
	// ("cv::HausdorffDistanceExtractor::setRankProportion", vec![(pred!(mut, ["rankProportion"], ["float"]), _)]),
	void cv_HausdorffDistanceExtractor_setRankProportion_float(cv::HausdorffDistanceExtractor* instance, float rankProportion, ResultVoid* ocvrs_return) {
		try {
			instance->setRankProportion(rankProportion);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRankProportion()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:218
	// ("cv::HausdorffDistanceExtractor::getRankProportion", vec![(pred!(const, [], []), _)]),
	void cv_HausdorffDistanceExtractor_getRankProportion_const(const cv::HausdorffDistanceExtractor* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getRankProportion();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HausdorffDistanceExtractor::to_Algorithm() generated
	// ("cv::HausdorffDistanceExtractor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_HausdorffDistanceExtractor_to_Algorithm(cv::HausdorffDistanceExtractor* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::HausdorffDistanceExtractor::to_ShapeDistanceExtractor() generated
	// ("cv::HausdorffDistanceExtractor::to_ShapeDistanceExtractor", vec![(pred!(mut, [], []), _)]),
	cv::ShapeDistanceExtractor* cv_HausdorffDistanceExtractor_to_ShapeDistanceExtractor(cv::HausdorffDistanceExtractor* instance) {
			return dynamic_cast<cv::ShapeDistanceExtractor*>(instance);
	}

	// cv::HausdorffDistanceExtractor::delete() generated
	// ("cv::HausdorffDistanceExtractor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_HausdorffDistanceExtractor_delete(cv::HausdorffDistanceExtractor* instance) {
			delete instance;
	}

	// buildCostMatrix(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/hist_cost.hpp:60
	// ("cv::HistogramCostExtractor::buildCostMatrix", vec![(pred!(mut, ["descriptors1", "descriptors2", "costMatrix"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_HistogramCostExtractor_buildCostMatrix_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::HistogramCostExtractor* instance, const cv::_InputArray* descriptors1, const cv::_InputArray* descriptors2, const cv::_OutputArray* costMatrix, ResultVoid* ocvrs_return) {
		try {
			instance->buildCostMatrix(*descriptors1, *descriptors2, *costMatrix);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNDummies(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/hist_cost.hpp:62
	// ("cv::HistogramCostExtractor::setNDummies", vec![(pred!(mut, ["nDummies"], ["int"]), _)]),
	void cv_HistogramCostExtractor_setNDummies_int(cv::HistogramCostExtractor* instance, int nDummies, ResultVoid* ocvrs_return) {
		try {
			instance->setNDummies(nDummies);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNDummies()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/hist_cost.hpp:63
	// ("cv::HistogramCostExtractor::getNDummies", vec![(pred!(const, [], []), _)]),
	void cv_HistogramCostExtractor_getNDummies_const(const cv::HistogramCostExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNDummies();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDefaultCost(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/hist_cost.hpp:65
	// ("cv::HistogramCostExtractor::setDefaultCost", vec![(pred!(mut, ["defaultCost"], ["float"]), _)]),
	void cv_HistogramCostExtractor_setDefaultCost_float(cv::HistogramCostExtractor* instance, float defaultCost, ResultVoid* ocvrs_return) {
		try {
			instance->setDefaultCost(defaultCost);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultCost()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/hist_cost.hpp:66
	// ("cv::HistogramCostExtractor::getDefaultCost", vec![(pred!(const, [], []), _)]),
	void cv_HistogramCostExtractor_getDefaultCost_const(const cv::HistogramCostExtractor* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getDefaultCost();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::HistogramCostExtractor::to_ChiHistogramCostExtractor() generated
	// ("cv::HistogramCostExtractor::to_ChiHistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
	cv::ChiHistogramCostExtractor* cv_HistogramCostExtractor_to_ChiHistogramCostExtractor(cv::HistogramCostExtractor* instance) {
			return dynamic_cast<cv::ChiHistogramCostExtractor*>(instance);
	}

	// cv::HistogramCostExtractor::to_EMDHistogramCostExtractor() generated
	// ("cv::HistogramCostExtractor::to_EMDHistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
	cv::EMDHistogramCostExtractor* cv_HistogramCostExtractor_to_EMDHistogramCostExtractor(cv::HistogramCostExtractor* instance) {
			return dynamic_cast<cv::EMDHistogramCostExtractor*>(instance);
	}

	// cv::HistogramCostExtractor::to_EMDL1HistogramCostExtractor() generated
	// ("cv::HistogramCostExtractor::to_EMDL1HistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
	cv::EMDL1HistogramCostExtractor* cv_HistogramCostExtractor_to_EMDL1HistogramCostExtractor(cv::HistogramCostExtractor* instance) {
			return dynamic_cast<cv::EMDL1HistogramCostExtractor*>(instance);
	}

	// cv::HistogramCostExtractor::to_NormHistogramCostExtractor() generated
	// ("cv::HistogramCostExtractor::to_NormHistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
	cv::NormHistogramCostExtractor* cv_HistogramCostExtractor_to_NormHistogramCostExtractor(cv::HistogramCostExtractor* instance) {
			return dynamic_cast<cv::NormHistogramCostExtractor*>(instance);
	}

	// cv::HistogramCostExtractor::to_Algorithm() generated
	// ("cv::HistogramCostExtractor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_HistogramCostExtractor_to_Algorithm(cv::HistogramCostExtractor* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::HistogramCostExtractor::delete() generated
	// ("cv::HistogramCostExtractor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_HistogramCostExtractor_delete(cv::HistogramCostExtractor* instance) {
			delete instance;
	}

	// setNormFlag(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/hist_cost.hpp:74
	// ("cv::NormHistogramCostExtractor::setNormFlag", vec![(pred!(mut, ["flag"], ["int"]), _)]),
	void cv_NormHistogramCostExtractor_setNormFlag_int(cv::NormHistogramCostExtractor* instance, int flag, ResultVoid* ocvrs_return) {
		try {
			instance->setNormFlag(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormFlag()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/hist_cost.hpp:75
	// ("cv::NormHistogramCostExtractor::getNormFlag", vec![(pred!(const, [], []), _)]),
	void cv_NormHistogramCostExtractor_getNormFlag_const(const cv::NormHistogramCostExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNormFlag();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::NormHistogramCostExtractor::to_Algorithm() generated
	// ("cv::NormHistogramCostExtractor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_NormHistogramCostExtractor_to_Algorithm(cv::NormHistogramCostExtractor* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::NormHistogramCostExtractor::to_HistogramCostExtractor() generated
	// ("cv::NormHistogramCostExtractor::to_HistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
	cv::HistogramCostExtractor* cv_NormHistogramCostExtractor_to_HistogramCostExtractor(cv::NormHistogramCostExtractor* instance) {
			return dynamic_cast<cv::HistogramCostExtractor*>(instance);
	}

	// cv::NormHistogramCostExtractor::delete() generated
	// ("cv::NormHistogramCostExtractor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_NormHistogramCostExtractor_delete(cv::NormHistogramCostExtractor* instance) {
			delete instance;
	}

	// setAngularBins(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:89
	// ("cv::ShapeContextDistanceExtractor::setAngularBins", vec![(pred!(mut, ["nAngularBins"], ["int"]), _)]),
	void cv_ShapeContextDistanceExtractor_setAngularBins_int(cv::ShapeContextDistanceExtractor* instance, int nAngularBins, ResultVoid* ocvrs_return) {
		try {
			instance->setAngularBins(nAngularBins);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAngularBins()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:90
	// ("cv::ShapeContextDistanceExtractor::getAngularBins", vec![(pred!(const, [], []), _)]),
	void cv_ShapeContextDistanceExtractor_getAngularBins_const(const cv::ShapeContextDistanceExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getAngularBins();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRadialBins(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:97
	// ("cv::ShapeContextDistanceExtractor::setRadialBins", vec![(pred!(mut, ["nRadialBins"], ["int"]), _)]),
	void cv_ShapeContextDistanceExtractor_setRadialBins_int(cv::ShapeContextDistanceExtractor* instance, int nRadialBins, ResultVoid* ocvrs_return) {
		try {
			instance->setRadialBins(nRadialBins);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRadialBins()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:98
	// ("cv::ShapeContextDistanceExtractor::getRadialBins", vec![(pred!(const, [], []), _)]),
	void cv_ShapeContextDistanceExtractor_getRadialBins_const(const cv::ShapeContextDistanceExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getRadialBins();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInnerRadius(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:104
	// ("cv::ShapeContextDistanceExtractor::setInnerRadius", vec![(pred!(mut, ["innerRadius"], ["float"]), _)]),
	void cv_ShapeContextDistanceExtractor_setInnerRadius_float(cv::ShapeContextDistanceExtractor* instance, float innerRadius, ResultVoid* ocvrs_return) {
		try {
			instance->setInnerRadius(innerRadius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInnerRadius()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:105
	// ("cv::ShapeContextDistanceExtractor::getInnerRadius", vec![(pred!(const, [], []), _)]),
	void cv_ShapeContextDistanceExtractor_getInnerRadius_const(const cv::ShapeContextDistanceExtractor* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getInnerRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOuterRadius(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:111
	// ("cv::ShapeContextDistanceExtractor::setOuterRadius", vec![(pred!(mut, ["outerRadius"], ["float"]), _)]),
	void cv_ShapeContextDistanceExtractor_setOuterRadius_float(cv::ShapeContextDistanceExtractor* instance, float outerRadius, ResultVoid* ocvrs_return) {
		try {
			instance->setOuterRadius(outerRadius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOuterRadius()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:112
	// ("cv::ShapeContextDistanceExtractor::getOuterRadius", vec![(pred!(const, [], []), _)]),
	void cv_ShapeContextDistanceExtractor_getOuterRadius_const(const cv::ShapeContextDistanceExtractor* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getOuterRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRotationInvariant(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:114
	// ("cv::ShapeContextDistanceExtractor::setRotationInvariant", vec![(pred!(mut, ["rotationInvariant"], ["bool"]), _)]),
	void cv_ShapeContextDistanceExtractor_setRotationInvariant_bool(cv::ShapeContextDistanceExtractor* instance, bool rotationInvariant, ResultVoid* ocvrs_return) {
		try {
			instance->setRotationInvariant(rotationInvariant);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRotationInvariant()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:115
	// ("cv::ShapeContextDistanceExtractor::getRotationInvariant", vec![(pred!(const, [], []), _)]),
	void cv_ShapeContextDistanceExtractor_getRotationInvariant_const(const cv::ShapeContextDistanceExtractor* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getRotationInvariant();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setShapeContextWeight(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:124
	// ("cv::ShapeContextDistanceExtractor::setShapeContextWeight", vec![(pred!(mut, ["shapeContextWeight"], ["float"]), _)]),
	void cv_ShapeContextDistanceExtractor_setShapeContextWeight_float(cv::ShapeContextDistanceExtractor* instance, float shapeContextWeight, ResultVoid* ocvrs_return) {
		try {
			instance->setShapeContextWeight(shapeContextWeight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getShapeContextWeight()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:125
	// ("cv::ShapeContextDistanceExtractor::getShapeContextWeight", vec![(pred!(const, [], []), _)]),
	void cv_ShapeContextDistanceExtractor_getShapeContextWeight_const(const cv::ShapeContextDistanceExtractor* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getShapeContextWeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setImageAppearanceWeight(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:136
	// ("cv::ShapeContextDistanceExtractor::setImageAppearanceWeight", vec![(pred!(mut, ["imageAppearanceWeight"], ["float"]), _)]),
	void cv_ShapeContextDistanceExtractor_setImageAppearanceWeight_float(cv::ShapeContextDistanceExtractor* instance, float imageAppearanceWeight, ResultVoid* ocvrs_return) {
		try {
			instance->setImageAppearanceWeight(imageAppearanceWeight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getImageAppearanceWeight()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:137
	// ("cv::ShapeContextDistanceExtractor::getImageAppearanceWeight", vec![(pred!(const, [], []), _)]),
	void cv_ShapeContextDistanceExtractor_getImageAppearanceWeight_const(const cv::ShapeContextDistanceExtractor* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getImageAppearanceWeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBendingEnergyWeight(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:146
	// ("cv::ShapeContextDistanceExtractor::setBendingEnergyWeight", vec![(pred!(mut, ["bendingEnergyWeight"], ["float"]), _)]),
	void cv_ShapeContextDistanceExtractor_setBendingEnergyWeight_float(cv::ShapeContextDistanceExtractor* instance, float bendingEnergyWeight, ResultVoid* ocvrs_return) {
		try {
			instance->setBendingEnergyWeight(bendingEnergyWeight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBendingEnergyWeight()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:147
	// ("cv::ShapeContextDistanceExtractor::getBendingEnergyWeight", vec![(pred!(const, [], []), _)]),
	void cv_ShapeContextDistanceExtractor_getBendingEnergyWeight_const(const cv::ShapeContextDistanceExtractor* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getBendingEnergyWeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setImages(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:155
	// ("cv::ShapeContextDistanceExtractor::setImages", vec![(pred!(mut, ["image1", "image2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_ShapeContextDistanceExtractor_setImages_const__InputArrayR_const__InputArrayR(cv::ShapeContextDistanceExtractor* instance, const cv::_InputArray* image1, const cv::_InputArray* image2, ResultVoid* ocvrs_return) {
		try {
			instance->setImages(*image1, *image2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getImages(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:156
	// ("cv::ShapeContextDistanceExtractor::getImages", vec![(pred!(const, ["image1", "image2"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ShapeContextDistanceExtractor_getImages_const_const__OutputArrayR_const__OutputArrayR(const cv::ShapeContextDistanceExtractor* instance, const cv::_OutputArray* image1, const cv::_OutputArray* image2, ResultVoid* ocvrs_return) {
		try {
			instance->getImages(*image1, *image2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:158
	// ("cv::ShapeContextDistanceExtractor::setIterations", vec![(pred!(mut, ["iterations"], ["int"]), _)]),
	void cv_ShapeContextDistanceExtractor_setIterations_int(cv::ShapeContextDistanceExtractor* instance, int iterations, ResultVoid* ocvrs_return) {
		try {
			instance->setIterations(iterations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIterations()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:159
	// ("cv::ShapeContextDistanceExtractor::getIterations", vec![(pred!(const, [], []), _)]),
	void cv_ShapeContextDistanceExtractor_getIterations_const(const cv::ShapeContextDistanceExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCostExtractor(Ptr<HistogramCostExtractor>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:166
	// ("cv::ShapeContextDistanceExtractor::setCostExtractor", vec![(pred!(mut, ["comparer"], ["cv::Ptr<cv::HistogramCostExtractor>"]), _)]),
	void cv_ShapeContextDistanceExtractor_setCostExtractor_PtrLHistogramCostExtractorG(cv::ShapeContextDistanceExtractor* instance, cv::Ptr<cv::HistogramCostExtractor>* comparer, ResultVoid* ocvrs_return) {
		try {
			instance->setCostExtractor(*comparer);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCostExtractor()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:167
	// ("cv::ShapeContextDistanceExtractor::getCostExtractor", vec![(pred!(const, [], []), _)]),
	void cv_ShapeContextDistanceExtractor_getCostExtractor_const(const cv::ShapeContextDistanceExtractor* instance, Result<cv::Ptr<cv::HistogramCostExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = instance->getCostExtractor();
			Ok(new cv::Ptr<cv::HistogramCostExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setStdDev(float)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:173
	// ("cv::ShapeContextDistanceExtractor::setStdDev", vec![(pred!(mut, ["sigma"], ["float"]), _)]),
	void cv_ShapeContextDistanceExtractor_setStdDev_float(cv::ShapeContextDistanceExtractor* instance, float sigma, ResultVoid* ocvrs_return) {
		try {
			instance->setStdDev(sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getStdDev()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:174
	// ("cv::ShapeContextDistanceExtractor::getStdDev", vec![(pred!(const, [], []), _)]),
	void cv_ShapeContextDistanceExtractor_getStdDev_const(const cv::ShapeContextDistanceExtractor* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getStdDev();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTransformAlgorithm(Ptr<ShapeTransformer>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:181
	// ("cv::ShapeContextDistanceExtractor::setTransformAlgorithm", vec![(pred!(mut, ["transformer"], ["cv::Ptr<cv::ShapeTransformer>"]), _)]),
	void cv_ShapeContextDistanceExtractor_setTransformAlgorithm_PtrLShapeTransformerG(cv::ShapeContextDistanceExtractor* instance, cv::Ptr<cv::ShapeTransformer>* transformer, ResultVoid* ocvrs_return) {
		try {
			instance->setTransformAlgorithm(*transformer);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTransformAlgorithm()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:182
	// ("cv::ShapeContextDistanceExtractor::getTransformAlgorithm", vec![(pred!(const, [], []), _)]),
	void cv_ShapeContextDistanceExtractor_getTransformAlgorithm_const(const cv::ShapeContextDistanceExtractor* instance, Result<cv::Ptr<cv::ShapeTransformer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ShapeTransformer> ret = instance->getTransformAlgorithm();
			Ok(new cv::Ptr<cv::ShapeTransformer>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ShapeContextDistanceExtractor::to_Algorithm() generated
	// ("cv::ShapeContextDistanceExtractor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ShapeContextDistanceExtractor_to_Algorithm(cv::ShapeContextDistanceExtractor* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ShapeContextDistanceExtractor::to_ShapeDistanceExtractor() generated
	// ("cv::ShapeContextDistanceExtractor::to_ShapeDistanceExtractor", vec![(pred!(mut, [], []), _)]),
	cv::ShapeDistanceExtractor* cv_ShapeContextDistanceExtractor_to_ShapeDistanceExtractor(cv::ShapeContextDistanceExtractor* instance) {
			return dynamic_cast<cv::ShapeDistanceExtractor*>(instance);
	}

	// cv::ShapeContextDistanceExtractor::delete() generated
	// ("cv::ShapeContextDistanceExtractor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ShapeContextDistanceExtractor_delete(cv::ShapeContextDistanceExtractor* instance) {
			delete instance;
	}

	// computeDistance(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_distance.hpp:69
	// ("cv::ShapeDistanceExtractor::computeDistance", vec![(pred!(mut, ["contour1", "contour2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_ShapeDistanceExtractor_computeDistance_const__InputArrayR_const__InputArrayR(cv::ShapeDistanceExtractor* instance, const cv::_InputArray* contour1, const cv::_InputArray* contour2, Result<float>* ocvrs_return) {
		try {
			float ret = instance->computeDistance(*contour1, *contour2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ShapeDistanceExtractor::to_HausdorffDistanceExtractor() generated
	// ("cv::ShapeDistanceExtractor::to_HausdorffDistanceExtractor", vec![(pred!(mut, [], []), _)]),
	cv::HausdorffDistanceExtractor* cv_ShapeDistanceExtractor_to_HausdorffDistanceExtractor(cv::ShapeDistanceExtractor* instance) {
			return dynamic_cast<cv::HausdorffDistanceExtractor*>(instance);
	}

	// cv::ShapeDistanceExtractor::to_ShapeContextDistanceExtractor() generated
	// ("cv::ShapeDistanceExtractor::to_ShapeContextDistanceExtractor", vec![(pred!(mut, [], []), _)]),
	cv::ShapeContextDistanceExtractor* cv_ShapeDistanceExtractor_to_ShapeContextDistanceExtractor(cv::ShapeDistanceExtractor* instance) {
			return dynamic_cast<cv::ShapeContextDistanceExtractor*>(instance);
	}

	// cv::ShapeDistanceExtractor::to_Algorithm() generated
	// ("cv::ShapeDistanceExtractor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ShapeDistanceExtractor_to_Algorithm(cv::ShapeDistanceExtractor* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ShapeDistanceExtractor::delete() generated
	// ("cv::ShapeDistanceExtractor::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ShapeDistanceExtractor_delete(cv::ShapeDistanceExtractor* instance) {
			delete instance;
	}

	// estimateTransformation(InputArray, InputArray, std::vector<DMatch> &)(InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_transformer.hpp:67
	// ("cv::ShapeTransformer::estimateTransformation", vec![(pred!(mut, ["transformingShape", "targetShape", "matches"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::DMatch>*"]), _)]),
	void cv_ShapeTransformer_estimateTransformation_const__InputArrayR_const__InputArrayR_vectorLDMatchGR(cv::ShapeTransformer* instance, const cv::_InputArray* transformingShape, const cv::_InputArray* targetShape, std::vector<cv::DMatch>* matches, ResultVoid* ocvrs_return) {
		try {
			instance->estimateTransformation(*transformingShape, *targetShape, *matches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// applyTransformation(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_transformer.hpp:75
	// ("cv::ShapeTransformer::applyTransformation", vec![(pred!(mut, ["input", "output"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ShapeTransformer_applyTransformation_const__InputArrayR_const__OutputArrayR(cv::ShapeTransformer* instance, const cv::_InputArray* input, const cv::_OutputArray* output, Result<float>* ocvrs_return) {
		try {
			float ret = instance->applyTransformation(*input, *output);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ShapeTransformer::applyTransformation(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_transformer.hpp:75
	// ("cv::ShapeTransformer::applyTransformation", vec![(pred!(mut, ["input"], ["const cv::_InputArray*"]), _)]),
	void cv_ShapeTransformer_applyTransformation_const__InputArrayR(cv::ShapeTransformer* instance, const cv::_InputArray* input, Result<float>* ocvrs_return) {
		try {
			float ret = instance->applyTransformation(*input);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// warpImage(InputArray, OutputArray, int, int, const Scalar &)(InputArray, OutputArray, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_transformer.hpp:85
	// ("cv::ShapeTransformer::warpImage", vec![(pred!(const, ["transformingImage", "output", "flags", "borderMode", "borderValue"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "const cv::Scalar*"]), _)]),
	void cv_ShapeTransformer_warpImage_const_const__InputArrayR_const__OutputArrayR_int_int_const_ScalarR(const cv::ShapeTransformer* instance, const cv::_InputArray* transformingImage, const cv::_OutputArray* output, int flags, int borderMode, const cv::Scalar* borderValue, ResultVoid* ocvrs_return) {
		try {
			instance->warpImage(*transformingImage, *output, flags, borderMode, *borderValue);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ShapeTransformer::warpImage(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_transformer.hpp:85
	// ("cv::ShapeTransformer::warpImage", vec![(pred!(const, ["transformingImage", "output"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ShapeTransformer_warpImage_const_const__InputArrayR_const__OutputArrayR(const cv::ShapeTransformer* instance, const cv::_InputArray* transformingImage, const cv::_OutputArray* output, ResultVoid* ocvrs_return) {
		try {
			instance->warpImage(*transformingImage, *output);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ShapeTransformer::to_AffineTransformer() generated
	// ("cv::ShapeTransformer::to_AffineTransformer", vec![(pred!(mut, [], []), _)]),
	cv::AffineTransformer* cv_ShapeTransformer_to_AffineTransformer(cv::ShapeTransformer* instance) {
			return dynamic_cast<cv::AffineTransformer*>(instance);
	}

	// cv::ShapeTransformer::to_ThinPlateSplineShapeTransformer() generated
	// ("cv::ShapeTransformer::to_ThinPlateSplineShapeTransformer", vec![(pred!(mut, [], []), _)]),
	cv::ThinPlateSplineShapeTransformer* cv_ShapeTransformer_to_ThinPlateSplineShapeTransformer(cv::ShapeTransformer* instance) {
			return dynamic_cast<cv::ThinPlateSplineShapeTransformer*>(instance);
	}

	// cv::ShapeTransformer::to_Algorithm() generated
	// ("cv::ShapeTransformer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ShapeTransformer_to_Algorithm(cv::ShapeTransformer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ShapeTransformer::delete() generated
	// ("cv::ShapeTransformer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ShapeTransformer_delete(cv::ShapeTransformer* instance) {
			delete instance;
	}

	// setRegularizationParameter(double)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_transformer.hpp:106
	// ("cv::ThinPlateSplineShapeTransformer::setRegularizationParameter", vec![(pred!(mut, ["beta"], ["double"]), _)]),
	void cv_ThinPlateSplineShapeTransformer_setRegularizationParameter_double(cv::ThinPlateSplineShapeTransformer* instance, double beta, ResultVoid* ocvrs_return) {
		try {
			instance->setRegularizationParameter(beta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRegularizationParameter()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/shape/shape_transformer.hpp:107
	// ("cv::ThinPlateSplineShapeTransformer::getRegularizationParameter", vec![(pred!(const, [], []), _)]),
	void cv_ThinPlateSplineShapeTransformer_getRegularizationParameter_const(const cv::ThinPlateSplineShapeTransformer* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getRegularizationParameter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ThinPlateSplineShapeTransformer::to_Algorithm() generated
	// ("cv::ThinPlateSplineShapeTransformer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ThinPlateSplineShapeTransformer_to_Algorithm(cv::ThinPlateSplineShapeTransformer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ThinPlateSplineShapeTransformer::to_ShapeTransformer() generated
	// ("cv::ThinPlateSplineShapeTransformer::to_ShapeTransformer", vec![(pred!(mut, [], []), _)]),
	cv::ShapeTransformer* cv_ThinPlateSplineShapeTransformer_to_ShapeTransformer(cv::ThinPlateSplineShapeTransformer* instance) {
			return dynamic_cast<cv::ShapeTransformer*>(instance);
	}

	// cv::ThinPlateSplineShapeTransformer::delete() generated
	// ("cv::ThinPlateSplineShapeTransformer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ThinPlateSplineShapeTransformer_delete(cv::ThinPlateSplineShapeTransformer* instance) {
			delete instance;
	}

}
