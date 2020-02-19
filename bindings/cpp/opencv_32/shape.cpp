#include "common.hpp"
#include <opencv2/shape.hpp>
#include "shape_types.hpp"

extern "C" {
	Result<float> cv_EMDL1_const__InputArrayX_const__InputArrayX(void* signature1, void* signature2) {
		try {
			float ret = cv::EMDL1(*reinterpret_cast<const cv::_InputArray*>(signature1), *reinterpret_cast<const cv::_InputArray*>(signature2));
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<void*> cv_createAffineTransformer_bool(bool fullAffine) {
		try {
			cv::Ptr<cv::AffineTransformer> ret = cv::createAffineTransformer(fullAffine);
			return Ok<void*>(new cv::Ptr<cv::AffineTransformer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_createChiHistogramCostExtractor_int_float(int nDummies, float defaultCost) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = cv::createChiHistogramCostExtractor(nDummies, defaultCost);
			return Ok<void*>(new cv::Ptr<cv::HistogramCostExtractor>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_createEMDHistogramCostExtractor_int_int_float(int flag, int nDummies, float defaultCost) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = cv::createEMDHistogramCostExtractor(flag, nDummies, defaultCost);
			return Ok<void*>(new cv::Ptr<cv::HistogramCostExtractor>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_createEMDL1HistogramCostExtractor_int_float(int nDummies, float defaultCost) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = cv::createEMDL1HistogramCostExtractor(nDummies, defaultCost);
			return Ok<void*>(new cv::Ptr<cv::HistogramCostExtractor>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_createHausdorffDistanceExtractor_int_float(int distanceFlag, float rankProp) {
		try {
			cv::Ptr<cv::HausdorffDistanceExtractor> ret = cv::createHausdorffDistanceExtractor(distanceFlag, rankProp);
			return Ok<void*>(new cv::Ptr<cv::HausdorffDistanceExtractor>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_createNormHistogramCostExtractor_int_int_float(int flag, int nDummies, float defaultCost) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = cv::createNormHistogramCostExtractor(flag, nDummies, defaultCost);
			return Ok<void*>(new cv::Ptr<cv::HistogramCostExtractor>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_createShapeContextDistanceExtractor_int_int_float_float_int_const_Ptr_HistogramCostExtractor_X_const_Ptr_ShapeTransformer_X(int nAngularBins, int nRadialBins, float innerRadius, float outerRadius, int iterations, void* comparer, void* transformer) {
		try {
			cv::Ptr<cv::ShapeContextDistanceExtractor> ret = cv::createShapeContextDistanceExtractor(nAngularBins, nRadialBins, innerRadius, outerRadius, iterations, *reinterpret_cast<const cv::Ptr<cv::HistogramCostExtractor>*>(comparer), *reinterpret_cast<const cv::Ptr<cv::ShapeTransformer>*>(transformer));
			return Ok<void*>(new cv::Ptr<cv::ShapeContextDistanceExtractor>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_createThinPlateSplineShapeTransformer_double(double regularizationParameter) {
		try {
			cv::Ptr<cv::ThinPlateSplineShapeTransformer> ret = cv::createThinPlateSplineShapeTransformer(regularizationParameter);
			return Ok<void*>(new cv::Ptr<cv::ThinPlateSplineShapeTransformer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_AffineTransformer_setFullAffine_bool(void* instance, bool fullAffine) {
		try {
			reinterpret_cast<cv::AffineTransformer*>(instance)->setFullAffine(fullAffine);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_AffineTransformer_getFullAffine_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::AffineTransformer*>(instance)->getFullAffine();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_EMDHistogramCostExtractor_setNormFlag_int(void* instance, int flag) {
		try {
			reinterpret_cast<cv::EMDHistogramCostExtractor*>(instance)->setNormFlag(flag);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_EMDHistogramCostExtractor_getNormFlag_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::EMDHistogramCostExtractor*>(instance)->getNormFlag();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_HausdorffDistanceExtractor_setDistanceFlag_int(void* instance, int distanceFlag) {
		try {
			reinterpret_cast<cv::HausdorffDistanceExtractor*>(instance)->setDistanceFlag(distanceFlag);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_HausdorffDistanceExtractor_getDistanceFlag_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::HausdorffDistanceExtractor*>(instance)->getDistanceFlag();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_HausdorffDistanceExtractor_setRankProportion_float(void* instance, float rankProportion) {
		try {
			reinterpret_cast<cv::HausdorffDistanceExtractor*>(instance)->setRankProportion(rankProportion);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_HausdorffDistanceExtractor_getRankProportion_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::HausdorffDistanceExtractor*>(instance)->getRankProportion();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_HistogramCostExtractor_buildCostMatrix_const__InputArrayX_const__InputArrayX_const__OutputArrayX(void* instance, void* descriptors1, void* descriptors2, void* costMatrix) {
		try {
			reinterpret_cast<cv::HistogramCostExtractor*>(instance)->buildCostMatrix(*reinterpret_cast<const cv::_InputArray*>(descriptors1), *reinterpret_cast<const cv::_InputArray*>(descriptors2), *reinterpret_cast<const cv::_OutputArray*>(costMatrix));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_HistogramCostExtractor_setNDummies_int(void* instance, int nDummies) {
		try {
			reinterpret_cast<cv::HistogramCostExtractor*>(instance)->setNDummies(nDummies);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_HistogramCostExtractor_getNDummies_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::HistogramCostExtractor*>(instance)->getNDummies();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_HistogramCostExtractor_setDefaultCost_float(void* instance, float defaultCost) {
		try {
			reinterpret_cast<cv::HistogramCostExtractor*>(instance)->setDefaultCost(defaultCost);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_HistogramCostExtractor_getDefaultCost_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::HistogramCostExtractor*>(instance)->getDefaultCost();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_NormHistogramCostExtractor_setNormFlag_int(void* instance, int flag) {
		try {
			reinterpret_cast<cv::NormHistogramCostExtractor*>(instance)->setNormFlag(flag);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_NormHistogramCostExtractor_getNormFlag_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::NormHistogramCostExtractor*>(instance)->getNormFlag();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setAngularBins_int(void* instance, int nAngularBins) {
		try {
			reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->setAngularBins(nAngularBins);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ShapeContextDistanceExtractor_getAngularBins_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->getAngularBins();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setRadialBins_int(void* instance, int nRadialBins) {
		try {
			reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->setRadialBins(nRadialBins);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ShapeContextDistanceExtractor_getRadialBins_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->getRadialBins();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setInnerRadius_float(void* instance, float innerRadius) {
		try {
			reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->setInnerRadius(innerRadius);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ShapeContextDistanceExtractor_getInnerRadius_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->getInnerRadius();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setOuterRadius_float(void* instance, float outerRadius) {
		try {
			reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->setOuterRadius(outerRadius);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ShapeContextDistanceExtractor_getOuterRadius_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->getOuterRadius();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setRotationInvariant_bool(void* instance, bool rotationInvariant) {
		try {
			reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->setRotationInvariant(rotationInvariant);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_ShapeContextDistanceExtractor_getRotationInvariant_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->getRotationInvariant();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setShapeContextWeight_float(void* instance, float shapeContextWeight) {
		try {
			reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->setShapeContextWeight(shapeContextWeight);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ShapeContextDistanceExtractor_getShapeContextWeight_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->getShapeContextWeight();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setImageAppearanceWeight_float(void* instance, float imageAppearanceWeight) {
		try {
			reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->setImageAppearanceWeight(imageAppearanceWeight);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ShapeContextDistanceExtractor_getImageAppearanceWeight_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->getImageAppearanceWeight();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setBendingEnergyWeight_float(void* instance, float bendingEnergyWeight) {
		try {
			reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->setBendingEnergyWeight(bendingEnergyWeight);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ShapeContextDistanceExtractor_getBendingEnergyWeight_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->getBendingEnergyWeight();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setImages_const__InputArrayX_const__InputArrayX(void* instance, void* image1, void* image2) {
		try {
			reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->setImages(*reinterpret_cast<const cv::_InputArray*>(image1), *reinterpret_cast<const cv::_InputArray*>(image2));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_getImages_const_const__OutputArrayX_const__OutputArrayX(void* instance, void* image1, void* image2) {
		try {
			reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->getImages(*reinterpret_cast<const cv::_OutputArray*>(image1), *reinterpret_cast<const cv::_OutputArray*>(image2));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setIterations_int(void* instance, int iterations) {
		try {
			reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->setIterations(iterations);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ShapeContextDistanceExtractor_getIterations_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->getIterations();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setCostExtractor_Ptr_HistogramCostExtractor_(void* instance, void* comparer) {
		try {
			reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->setCostExtractor(*reinterpret_cast<cv::Ptr<cv::HistogramCostExtractor>*>(comparer));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ShapeContextDistanceExtractor_getCostExtractor_const(void* instance) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->getCostExtractor();
			return Ok<void*>(new cv::Ptr<cv::HistogramCostExtractor>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setStdDev_float(void* instance, float sigma) {
		try {
			reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->setStdDev(sigma);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ShapeContextDistanceExtractor_getStdDev_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->getStdDev();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ShapeContextDistanceExtractor_setTransformAlgorithm_Ptr_ShapeTransformer_(void* instance, void* transformer) {
		try {
			reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->setTransformAlgorithm(*reinterpret_cast<cv::Ptr<cv::ShapeTransformer>*>(transformer));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ShapeContextDistanceExtractor_getTransformAlgorithm_const(void* instance) {
		try {
			cv::Ptr<cv::ShapeTransformer> ret = reinterpret_cast<cv::ShapeContextDistanceExtractor*>(instance)->getTransformAlgorithm();
			return Ok<void*>(new cv::Ptr<cv::ShapeTransformer>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<float> cv_ShapeDistanceExtractor_computeDistance_const__InputArrayX_const__InputArrayX(void* instance, void* contour1, void* contour2) {
		try {
			float ret = reinterpret_cast<cv::ShapeDistanceExtractor*>(instance)->computeDistance(*reinterpret_cast<const cv::_InputArray*>(contour1), *reinterpret_cast<const cv::_InputArray*>(contour2));
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ShapeTransformer_estimateTransformation_const__InputArrayX_const__InputArrayX_vector_DMatch_X(void* instance, void* transformingShape, void* targetShape, void* matches) {
		try {
			reinterpret_cast<cv::ShapeTransformer*>(instance)->estimateTransformation(*reinterpret_cast<const cv::_InputArray*>(transformingShape), *reinterpret_cast<const cv::_InputArray*>(targetShape), *reinterpret_cast<std::vector<cv::DMatch>*>(matches));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ShapeTransformer_applyTransformation_const__InputArrayX_const__OutputArrayX(void* instance, void* input, void* output) {
		try {
			float ret = reinterpret_cast<cv::ShapeTransformer*>(instance)->applyTransformation(*reinterpret_cast<const cv::_InputArray*>(input), *reinterpret_cast<const cv::_OutputArray*>(output));
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ShapeTransformer_warpImage_const_const__InputArrayX_const__OutputArrayX_int_int_const_ScalarX(void* instance, void* transformingImage, void* output, int flags, int borderMode, const cv::Scalar* borderValue) {
		try {
			reinterpret_cast<cv::ShapeTransformer*>(instance)->warpImage(*reinterpret_cast<const cv::_InputArray*>(transformingImage), *reinterpret_cast<const cv::_OutputArray*>(output), flags, borderMode, *borderValue);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ThinPlateSplineShapeTransformer_setRegularizationParameter_double(void* instance, double beta) {
		try {
			reinterpret_cast<cv::ThinPlateSplineShapeTransformer*>(instance)->setRegularizationParameter(beta);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ThinPlateSplineShapeTransformer_getRegularizationParameter_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ThinPlateSplineShapeTransformer*>(instance)->getRegularizationParameter();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
}
