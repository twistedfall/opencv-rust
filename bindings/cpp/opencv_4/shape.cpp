#include "common.hpp"
#include <opencv2/shape.hpp>
#include "shape_types.hpp"

extern "C" {
	Result<float> cv_EMDL1_const__InputArrayX_const__InputArrayX(const cv::_InputArray* signature1, const cv::_InputArray* signature2) {
		try {
			float ret = cv::EMDL1(*signature1, *signature2);
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<cv::Ptr<cv::AffineTransformer>*> cv_createAffineTransformer_bool(bool fullAffine) {
		try {
			cv::Ptr<cv::AffineTransformer> ret = cv::createAffineTransformer(fullAffine);
			return Ok(new cv::Ptr<cv::AffineTransformer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::AffineTransformer>*>)
	}
	
	Result<cv::Ptr<cv::HistogramCostExtractor>*> cv_createChiHistogramCostExtractor_int_float(int nDummies, float defaultCost) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = cv::createChiHistogramCostExtractor(nDummies, defaultCost);
			return Ok(new cv::Ptr<cv::HistogramCostExtractor>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::HistogramCostExtractor>*>)
	}
	
	Result<cv::Ptr<cv::HistogramCostExtractor>*> cv_createEMDHistogramCostExtractor_int_int_float(int flag, int nDummies, float defaultCost) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = cv::createEMDHistogramCostExtractor(flag, nDummies, defaultCost);
			return Ok(new cv::Ptr<cv::HistogramCostExtractor>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::HistogramCostExtractor>*>)
	}
	
	Result<cv::Ptr<cv::HistogramCostExtractor>*> cv_createEMDL1HistogramCostExtractor_int_float(int nDummies, float defaultCost) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = cv::createEMDL1HistogramCostExtractor(nDummies, defaultCost);
			return Ok(new cv::Ptr<cv::HistogramCostExtractor>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::HistogramCostExtractor>*>)
	}
	
	Result<cv::Ptr<cv::HausdorffDistanceExtractor>*> cv_createHausdorffDistanceExtractor_int_float(int distanceFlag, float rankProp) {
		try {
			cv::Ptr<cv::HausdorffDistanceExtractor> ret = cv::createHausdorffDistanceExtractor(distanceFlag, rankProp);
			return Ok(new cv::Ptr<cv::HausdorffDistanceExtractor>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::HausdorffDistanceExtractor>*>)
	}
	
	Result<cv::Ptr<cv::HistogramCostExtractor>*> cv_createNormHistogramCostExtractor_int_int_float(int flag, int nDummies, float defaultCost) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = cv::createNormHistogramCostExtractor(flag, nDummies, defaultCost);
			return Ok(new cv::Ptr<cv::HistogramCostExtractor>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::HistogramCostExtractor>*>)
	}
	
	Result<cv::Ptr<cv::ShapeContextDistanceExtractor>*> cv_createShapeContextDistanceExtractor_int_int_float_float_int_const_Ptr_HistogramCostExtractor_X_const_Ptr_ShapeTransformer_X(int nAngularBins, int nRadialBins, float innerRadius, float outerRadius, int iterations, const cv::Ptr<cv::HistogramCostExtractor>* comparer, const cv::Ptr<cv::ShapeTransformer>* transformer) {
		try {
			cv::Ptr<cv::ShapeContextDistanceExtractor> ret = cv::createShapeContextDistanceExtractor(nAngularBins, nRadialBins, innerRadius, outerRadius, iterations, *comparer, *transformer);
			return Ok(new cv::Ptr<cv::ShapeContextDistanceExtractor>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ShapeContextDistanceExtractor>*>)
	}
	
	Result<cv::Ptr<cv::ThinPlateSplineShapeTransformer>*> cv_createThinPlateSplineShapeTransformer_double(double regularizationParameter) {
		try {
			cv::Ptr<cv::ThinPlateSplineShapeTransformer> ret = cv::createThinPlateSplineShapeTransformer(regularizationParameter);
			return Ok(new cv::Ptr<cv::ThinPlateSplineShapeTransformer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ThinPlateSplineShapeTransformer>*>)
	}
	
	Result_void cv_AffineTransformer_setFullAffine_bool(cv::AffineTransformer* instance, bool fullAffine) {
		try {
			instance->setFullAffine(fullAffine);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_AffineTransformer_getFullAffine_const(const cv::AffineTransformer* instance) {
		try {
			bool ret = instance->getFullAffine();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_EMDHistogramCostExtractor_setNormFlag_int(cv::EMDHistogramCostExtractor* instance, int flag) {
		try {
			instance->setNormFlag(flag);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_EMDHistogramCostExtractor_getNormFlag_const(const cv::EMDHistogramCostExtractor* instance) {
		try {
			int ret = instance->getNormFlag();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_HausdorffDistanceExtractor_setDistanceFlag_int(cv::HausdorffDistanceExtractor* instance, int distanceFlag) {
		try {
			instance->setDistanceFlag(distanceFlag);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_HausdorffDistanceExtractor_getDistanceFlag_const(const cv::HausdorffDistanceExtractor* instance) {
		try {
			int ret = instance->getDistanceFlag();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_HausdorffDistanceExtractor_setRankProportion_float(cv::HausdorffDistanceExtractor* instance, float rankProportion) {
		try {
			instance->setRankProportion(rankProportion);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_HausdorffDistanceExtractor_getRankProportion_const(const cv::HausdorffDistanceExtractor* instance) {
		try {
			float ret = instance->getRankProportion();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_HistogramCostExtractor_buildCostMatrix_const__InputArrayX_const__InputArrayX_const__OutputArrayX(cv::HistogramCostExtractor* instance, const cv::_InputArray* descriptors1, const cv::_InputArray* descriptors2, const cv::_OutputArray* costMatrix) {
		try {
			instance->buildCostMatrix(*descriptors1, *descriptors2, *costMatrix);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HistogramCostExtractor_setNDummies_int(cv::HistogramCostExtractor* instance, int nDummies) {
		try {
			instance->setNDummies(nDummies);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_HistogramCostExtractor_getNDummies_const(const cv::HistogramCostExtractor* instance) {
		try {
			int ret = instance->getNDummies();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_HistogramCostExtractor_setDefaultCost_float(cv::HistogramCostExtractor* instance, float defaultCost) {
		try {
			instance->setDefaultCost(defaultCost);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_HistogramCostExtractor_getDefaultCost_const(const cv::HistogramCostExtractor* instance) {
		try {
			float ret = instance->getDefaultCost();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_NormHistogramCostExtractor_setNormFlag_int(cv::NormHistogramCostExtractor* instance, int flag) {
		try {
			instance->setNormFlag(flag);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_NormHistogramCostExtractor_getNormFlag_const(const cv::NormHistogramCostExtractor* instance) {
		try {
			int ret = instance->getNormFlag();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setAngularBins_int(cv::ShapeContextDistanceExtractor* instance, int nAngularBins) {
		try {
			instance->setAngularBins(nAngularBins);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ShapeContextDistanceExtractor_getAngularBins_const(const cv::ShapeContextDistanceExtractor* instance) {
		try {
			int ret = instance->getAngularBins();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setRadialBins_int(cv::ShapeContextDistanceExtractor* instance, int nRadialBins) {
		try {
			instance->setRadialBins(nRadialBins);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ShapeContextDistanceExtractor_getRadialBins_const(const cv::ShapeContextDistanceExtractor* instance) {
		try {
			int ret = instance->getRadialBins();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setInnerRadius_float(cv::ShapeContextDistanceExtractor* instance, float innerRadius) {
		try {
			instance->setInnerRadius(innerRadius);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ShapeContextDistanceExtractor_getInnerRadius_const(const cv::ShapeContextDistanceExtractor* instance) {
		try {
			float ret = instance->getInnerRadius();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setOuterRadius_float(cv::ShapeContextDistanceExtractor* instance, float outerRadius) {
		try {
			instance->setOuterRadius(outerRadius);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ShapeContextDistanceExtractor_getOuterRadius_const(const cv::ShapeContextDistanceExtractor* instance) {
		try {
			float ret = instance->getOuterRadius();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setRotationInvariant_bool(cv::ShapeContextDistanceExtractor* instance, bool rotationInvariant) {
		try {
			instance->setRotationInvariant(rotationInvariant);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_ShapeContextDistanceExtractor_getRotationInvariant_const(const cv::ShapeContextDistanceExtractor* instance) {
		try {
			bool ret = instance->getRotationInvariant();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setShapeContextWeight_float(cv::ShapeContextDistanceExtractor* instance, float shapeContextWeight) {
		try {
			instance->setShapeContextWeight(shapeContextWeight);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ShapeContextDistanceExtractor_getShapeContextWeight_const(const cv::ShapeContextDistanceExtractor* instance) {
		try {
			float ret = instance->getShapeContextWeight();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setImageAppearanceWeight_float(cv::ShapeContextDistanceExtractor* instance, float imageAppearanceWeight) {
		try {
			instance->setImageAppearanceWeight(imageAppearanceWeight);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ShapeContextDistanceExtractor_getImageAppearanceWeight_const(const cv::ShapeContextDistanceExtractor* instance) {
		try {
			float ret = instance->getImageAppearanceWeight();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setBendingEnergyWeight_float(cv::ShapeContextDistanceExtractor* instance, float bendingEnergyWeight) {
		try {
			instance->setBendingEnergyWeight(bendingEnergyWeight);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ShapeContextDistanceExtractor_getBendingEnergyWeight_const(const cv::ShapeContextDistanceExtractor* instance) {
		try {
			float ret = instance->getBendingEnergyWeight();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setImages_const__InputArrayX_const__InputArrayX(cv::ShapeContextDistanceExtractor* instance, const cv::_InputArray* image1, const cv::_InputArray* image2) {
		try {
			instance->setImages(*image1, *image2);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_getImages_const_const__OutputArrayX_const__OutputArrayX(const cv::ShapeContextDistanceExtractor* instance, const cv::_OutputArray* image1, const cv::_OutputArray* image2) {
		try {
			instance->getImages(*image1, *image2);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setIterations_int(cv::ShapeContextDistanceExtractor* instance, int iterations) {
		try {
			instance->setIterations(iterations);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ShapeContextDistanceExtractor_getIterations_const(const cv::ShapeContextDistanceExtractor* instance) {
		try {
			int ret = instance->getIterations();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setCostExtractor_Ptr_HistogramCostExtractor_(cv::ShapeContextDistanceExtractor* instance, cv::Ptr<cv::HistogramCostExtractor>* comparer) {
		try {
			instance->setCostExtractor(*comparer);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::HistogramCostExtractor>*> cv_ShapeContextDistanceExtractor_getCostExtractor_const(const cv::ShapeContextDistanceExtractor* instance) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = instance->getCostExtractor();
			return Ok(new cv::Ptr<cv::HistogramCostExtractor>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::HistogramCostExtractor>*>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setStdDev_float(cv::ShapeContextDistanceExtractor* instance, float sigma) {
		try {
			instance->setStdDev(sigma);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ShapeContextDistanceExtractor_getStdDev_const(const cv::ShapeContextDistanceExtractor* instance) {
		try {
			float ret = instance->getStdDev();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setTransformAlgorithm_Ptr_ShapeTransformer_(cv::ShapeContextDistanceExtractor* instance, cv::Ptr<cv::ShapeTransformer>* transformer) {
		try {
			instance->setTransformAlgorithm(*transformer);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::ShapeTransformer>*> cv_ShapeContextDistanceExtractor_getTransformAlgorithm_const(const cv::ShapeContextDistanceExtractor* instance) {
		try {
			cv::Ptr<cv::ShapeTransformer> ret = instance->getTransformAlgorithm();
			return Ok(new cv::Ptr<cv::ShapeTransformer>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ShapeTransformer>*>)
	}
	
	Result<float> cv_ShapeDistanceExtractor_computeDistance_const__InputArrayX_const__InputArrayX(cv::ShapeDistanceExtractor* instance, const cv::_InputArray* contour1, const cv::_InputArray* contour2) {
		try {
			float ret = instance->computeDistance(*contour1, *contour2);
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ShapeTransformer_estimateTransformation_const__InputArrayX_const__InputArrayX_vector_DMatch_X(cv::ShapeTransformer* instance, const cv::_InputArray* transformingShape, const cv::_InputArray* targetShape, std::vector<cv::DMatch>* matches) {
		try {
			instance->estimateTransformation(*transformingShape, *targetShape, *matches);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ShapeTransformer_applyTransformation_const__InputArrayX_const__OutputArrayX(cv::ShapeTransformer* instance, const cv::_InputArray* input, const cv::_OutputArray* output) {
		try {
			float ret = instance->applyTransformation(*input, *output);
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ShapeTransformer_warpImage_const_const__InputArrayX_const__OutputArrayX_int_int_const_ScalarX(const cv::ShapeTransformer* instance, const cv::_InputArray* transformingImage, const cv::_OutputArray* output, int flags, int borderMode, const cv::Scalar* borderValue) {
		try {
			instance->warpImage(*transformingImage, *output, flags, borderMode, *borderValue);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ThinPlateSplineShapeTransformer_setRegularizationParameter_double(cv::ThinPlateSplineShapeTransformer* instance, double beta) {
		try {
			instance->setRegularizationParameter(beta);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ThinPlateSplineShapeTransformer_getRegularizationParameter_const(const cv::ThinPlateSplineShapeTransformer* instance) {
		try {
			double ret = instance->getRegularizationParameter();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
}
