#include "ocvrs_common.hpp"
#include <opencv2/ml.hpp>
#include "ml_types.hpp"

extern "C" {
	// createConcentricSpheresTestSet(int, int, int, OutputArray, OutputArray)(Primitive, Primitive, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1898
	// ("cv::ml::createConcentricSpheresTestSet", vec![(pred!(mut, ["nsamples", "nfeatures", "nclasses", "samples", "responses"], ["int", "int", "int", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ml_createConcentricSpheresTestSet_int_int_int_const__OutputArrayR_const__OutputArrayR(int nsamples, int nfeatures, int nclasses, const cv::_OutputArray* samples, const cv::_OutputArray* responses, ResultVoid* ocvrs_return) {
		try {
			cv::ml::createConcentricSpheresTestSet(nsamples, nfeatures, nclasses, *samples, *responses);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// randMVNormal(InputArray, InputArray, int, OutputArray)(InputArray, InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1895
	// ("cv::ml::randMVNormal", vec![(pred!(mut, ["mean", "cov", "nsamples", "samples"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "const cv::_OutputArray*"]), _)]),
	void cv_ml_randMVNormal_const__InputArrayR_const__InputArrayR_int_const__OutputArrayR(const cv::_InputArray* mean, const cv::_InputArray* cov, int nsamples, const cv::_OutputArray* samples, ResultVoid* ocvrs_return) {
		try {
			cv::ml::randMVNormal(*mean, *cov, nsamples, *samples);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTrainMethod(int, double, double)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1446
	// ("cv::ml::ANN_MLP::setTrainMethod", vec![(pred!(mut, ["method", "param1", "param2"], ["int", "double", "double"]), _)]),
	void cv_ml_ANN_MLP_setTrainMethod_int_double_double(cv::ml::ANN_MLP* instance, int method, double param1, double param2, ResultVoid* ocvrs_return) {
		try {
			instance->setTrainMethod(method, param1, param2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::ANN_MLP::setTrainMethod(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1446
	// ("cv::ml::ANN_MLP::setTrainMethod", vec![(pred!(mut, ["method"], ["int"]), _)]),
	void cv_ml_ANN_MLP_setTrainMethod_int(cv::ml::ANN_MLP* instance, int method, ResultVoid* ocvrs_return) {
		try {
			instance->setTrainMethod(method);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTrainMethod()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1449
	// ("cv::ml::ANN_MLP::getTrainMethod", vec![(pred!(const, [], []), _)]),
	void cv_ml_ANN_MLP_getTrainMethod_const(const cv::ml::ANN_MLP* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTrainMethod();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setActivationFunction(int, double, double)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1457
	// ("cv::ml::ANN_MLP::setActivationFunction", vec![(pred!(mut, ["type", "param1", "param2"], ["int", "double", "double"]), _)]),
	void cv_ml_ANN_MLP_setActivationFunction_int_double_double(cv::ml::ANN_MLP* instance, int type, double param1, double param2, ResultVoid* ocvrs_return) {
		try {
			instance->setActivationFunction(type, param1, param2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::ANN_MLP::setActivationFunction(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1457
	// ("cv::ml::ANN_MLP::setActivationFunction", vec![(pred!(mut, ["type"], ["int"]), _)]),
	void cv_ml_ANN_MLP_setActivationFunction_int(cv::ml::ANN_MLP* instance, int type, ResultVoid* ocvrs_return) {
		try {
			instance->setActivationFunction(type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLayerSizes(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1463
	// ("cv::ml::ANN_MLP::setLayerSizes", vec![(pred!(mut, ["_layer_sizes"], ["const cv::_InputArray*"]), _)]),
	void cv_ml_ANN_MLP_setLayerSizes_const__InputArrayR(cv::ml::ANN_MLP* instance, const cv::_InputArray* _layer_sizes, ResultVoid* ocvrs_return) {
		try {
			instance->setLayerSizes(*_layer_sizes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayerSizes()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1469
	// ("cv::ml::ANN_MLP::getLayerSizes", vec![(pred!(const, [], []), _)]),
	void cv_ml_ANN_MLP_getLayerSizes_const(const cv::ml::ANN_MLP* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getLayerSizes();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTermCriteria()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1476
	// ("cv::ml::ANN_MLP::getTermCriteria", vec![(pred!(const, [], []), _)]),
	void cv_ml_ANN_MLP_getTermCriteria_const(const cv::ml::ANN_MLP* instance, Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTermCriteria(TermCriteria)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1478
	// ("cv::ml::ANN_MLP::setTermCriteria", vec![(pred!(mut, ["val"], ["cv::TermCriteria"]), _)]),
	void cv_ml_ANN_MLP_setTermCriteria_TermCriteria(cv::ml::ANN_MLP* instance, cv::TermCriteria* val, ResultVoid* ocvrs_return) {
		try {
			instance->setTermCriteria(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackpropWeightScale()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1483
	// ("cv::ml::ANN_MLP::getBackpropWeightScale", vec![(pred!(const, [], []), _)]),
	void cv_ml_ANN_MLP_getBackpropWeightScale_const(const cv::ml::ANN_MLP* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getBackpropWeightScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBackpropWeightScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1485
	// ("cv::ml::ANN_MLP::setBackpropWeightScale", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ml_ANN_MLP_setBackpropWeightScale_double(cv::ml::ANN_MLP* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setBackpropWeightScale(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackpropMomentumScale()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1492
	// ("cv::ml::ANN_MLP::getBackpropMomentumScale", vec![(pred!(const, [], []), _)]),
	void cv_ml_ANN_MLP_getBackpropMomentumScale_const(const cv::ml::ANN_MLP* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getBackpropMomentumScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBackpropMomentumScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1494
	// ("cv::ml::ANN_MLP::setBackpropMomentumScale", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ml_ANN_MLP_setBackpropMomentumScale_double(cv::ml::ANN_MLP* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setBackpropMomentumScale(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRpropDW0()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1499
	// ("cv::ml::ANN_MLP::getRpropDW0", vec![(pred!(const, [], []), _)]),
	void cv_ml_ANN_MLP_getRpropDW0_const(const cv::ml::ANN_MLP* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getRpropDW0();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRpropDW0(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1501
	// ("cv::ml::ANN_MLP::setRpropDW0", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ml_ANN_MLP_setRpropDW0_double(cv::ml::ANN_MLP* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setRpropDW0(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRpropDWPlus()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1506
	// ("cv::ml::ANN_MLP::getRpropDWPlus", vec![(pred!(const, [], []), _)]),
	void cv_ml_ANN_MLP_getRpropDWPlus_const(const cv::ml::ANN_MLP* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getRpropDWPlus();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRpropDWPlus(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1508
	// ("cv::ml::ANN_MLP::setRpropDWPlus", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ml_ANN_MLP_setRpropDWPlus_double(cv::ml::ANN_MLP* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setRpropDWPlus(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRpropDWMinus()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1513
	// ("cv::ml::ANN_MLP::getRpropDWMinus", vec![(pred!(const, [], []), _)]),
	void cv_ml_ANN_MLP_getRpropDWMinus_const(const cv::ml::ANN_MLP* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getRpropDWMinus();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRpropDWMinus(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1515
	// ("cv::ml::ANN_MLP::setRpropDWMinus", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ml_ANN_MLP_setRpropDWMinus_double(cv::ml::ANN_MLP* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setRpropDWMinus(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRpropDWMin()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1520
	// ("cv::ml::ANN_MLP::getRpropDWMin", vec![(pred!(const, [], []), _)]),
	void cv_ml_ANN_MLP_getRpropDWMin_const(const cv::ml::ANN_MLP* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getRpropDWMin();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRpropDWMin(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1522
	// ("cv::ml::ANN_MLP::setRpropDWMin", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ml_ANN_MLP_setRpropDWMin_double(cv::ml::ANN_MLP* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setRpropDWMin(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRpropDWMax()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1527
	// ("cv::ml::ANN_MLP::getRpropDWMax", vec![(pred!(const, [], []), _)]),
	void cv_ml_ANN_MLP_getRpropDWMax_const(const cv::ml::ANN_MLP* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getRpropDWMax();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRpropDWMax(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1529
	// ("cv::ml::ANN_MLP::setRpropDWMax", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ml_ANN_MLP_setRpropDWMax_double(cv::ml::ANN_MLP* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setRpropDWMax(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAnnealInitialT()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1534
	// ("cv::ml::ANN_MLP::getAnnealInitialT", vec![(pred!(const, [], []), _)]),
	void cv_ml_ANN_MLP_getAnnealInitialT_const(const cv::ml::ANN_MLP* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAnnealInitialT();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAnnealInitialT(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1536
	// ("cv::ml::ANN_MLP::setAnnealInitialT", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ml_ANN_MLP_setAnnealInitialT_double(cv::ml::ANN_MLP* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setAnnealInitialT(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAnnealFinalT()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1541
	// ("cv::ml::ANN_MLP::getAnnealFinalT", vec![(pred!(const, [], []), _)]),
	void cv_ml_ANN_MLP_getAnnealFinalT_const(const cv::ml::ANN_MLP* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAnnealFinalT();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAnnealFinalT(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1543
	// ("cv::ml::ANN_MLP::setAnnealFinalT", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ml_ANN_MLP_setAnnealFinalT_double(cv::ml::ANN_MLP* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setAnnealFinalT(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAnnealCoolingRatio()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1548
	// ("cv::ml::ANN_MLP::getAnnealCoolingRatio", vec![(pred!(const, [], []), _)]),
	void cv_ml_ANN_MLP_getAnnealCoolingRatio_const(const cv::ml::ANN_MLP* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAnnealCoolingRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAnnealCoolingRatio(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1550
	// ("cv::ml::ANN_MLP::setAnnealCoolingRatio", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ml_ANN_MLP_setAnnealCoolingRatio_double(cv::ml::ANN_MLP* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setAnnealCoolingRatio(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAnnealItePerStep()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1555
	// ("cv::ml::ANN_MLP::getAnnealItePerStep", vec![(pred!(const, [], []), _)]),
	void cv_ml_ANN_MLP_getAnnealItePerStep_const(const cv::ml::ANN_MLP* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getAnnealItePerStep();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAnnealItePerStep(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1557
	// ("cv::ml::ANN_MLP::setAnnealItePerStep", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_ml_ANN_MLP_setAnnealItePerStep_int(cv::ml::ANN_MLP* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setAnnealItePerStep(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAnnealEnergyRNG(const RNG &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1560
	// ("cv::ml::ANN_MLP::setAnnealEnergyRNG", vec![(pred!(mut, ["rng"], ["const cv::RNG*"]), _)]),
	void cv_ml_ANN_MLP_setAnnealEnergyRNG_const_RNGR(cv::ml::ANN_MLP* instance, const cv::RNG* rng, ResultVoid* ocvrs_return) {
		try {
			instance->setAnnealEnergyRNG(*rng);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWeights(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1597
	// ("cv::ml::ANN_MLP::getWeights", vec![(pred!(const, ["layerIdx"], ["int"]), _)]),
	void cv_ml_ANN_MLP_getWeights_const_int(const cv::ml::ANN_MLP* instance, int layerIdx, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getWeights(layerIdx);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1604
	// ("cv::ml::ANN_MLP::create", vec![(pred!(mut, [], []), _)]),
	void cv_ml_ANN_MLP_create(Result<cv::Ptr<cv::ml::ANN_MLP>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::ANN_MLP> ret = cv::ml::ANN_MLP::create();
			Ok(new cv::Ptr<cv::ml::ANN_MLP>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1613
	// ("cv::ml::ANN_MLP::load", vec![(pred!(mut, ["filepath"], ["const cv::String*"]), _)]),
	void cv_ml_ANN_MLP_load_const_StringR(const char* filepath, Result<cv::Ptr<cv::ml::ANN_MLP>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::ANN_MLP> ret = cv::ml::ANN_MLP::load(cv::String(filepath));
			Ok(new cv::Ptr<cv::ml::ANN_MLP>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::ANN_MLP::to_ANN_MLP_ANNEAL() generated
	// ("cv::ml::ANN_MLP::to_ANN_MLP_ANNEAL", vec![(pred!(mut, [], []), _)]),
	cv::ml::ANN_MLP_ANNEAL* cv_ml_ANN_MLP_to_ANN_MLP_ANNEAL(cv::ml::ANN_MLP* instance) {
			return dynamic_cast<cv::ml::ANN_MLP_ANNEAL*>(instance);
	}

	// cv::ml::ANN_MLP::to_Algorithm() generated
	// ("cv::ml::ANN_MLP::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ml_ANN_MLP_to_Algorithm(cv::ml::ANN_MLP* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ml::ANN_MLP::to_StatModel() generated
	// ("cv::ml::ANN_MLP::to_StatModel", vec![(pred!(mut, [], []), _)]),
	cv::ml::StatModel* cv_ml_ANN_MLP_to_StatModel(cv::ml::ANN_MLP* instance) {
			return dynamic_cast<cv::ml::StatModel*>(instance);
	}

	// cv::ml::ANN_MLP::delete() generated
	// ("cv::ml::ANN_MLP::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ml_ANN_MLP_delete(cv::ml::ANN_MLP* instance) {
			delete instance;
	}

	// getAnnealInitialT()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1909
	// ("cv::ml::ANN_MLP_ANNEAL::getAnnealInitialT", vec![(pred!(const, [], []), _)]),
	void cv_ml_ANN_MLP_ANNEAL_getAnnealInitialT_const(const cv::ml::ANN_MLP_ANNEAL* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAnnealInitialT();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAnnealInitialT(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1911
	// ("cv::ml::ANN_MLP_ANNEAL::setAnnealInitialT", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ml_ANN_MLP_ANNEAL_setAnnealInitialT_double(cv::ml::ANN_MLP_ANNEAL* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setAnnealInitialT(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAnnealFinalT()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1916
	// ("cv::ml::ANN_MLP_ANNEAL::getAnnealFinalT", vec![(pred!(const, [], []), _)]),
	void cv_ml_ANN_MLP_ANNEAL_getAnnealFinalT_const(const cv::ml::ANN_MLP_ANNEAL* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAnnealFinalT();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAnnealFinalT(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1918
	// ("cv::ml::ANN_MLP_ANNEAL::setAnnealFinalT", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ml_ANN_MLP_ANNEAL_setAnnealFinalT_double(cv::ml::ANN_MLP_ANNEAL* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setAnnealFinalT(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAnnealCoolingRatio()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1923
	// ("cv::ml::ANN_MLP_ANNEAL::getAnnealCoolingRatio", vec![(pred!(const, [], []), _)]),
	void cv_ml_ANN_MLP_ANNEAL_getAnnealCoolingRatio_const(const cv::ml::ANN_MLP_ANNEAL* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAnnealCoolingRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAnnealCoolingRatio(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1925
	// ("cv::ml::ANN_MLP_ANNEAL::setAnnealCoolingRatio", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ml_ANN_MLP_ANNEAL_setAnnealCoolingRatio_double(cv::ml::ANN_MLP_ANNEAL* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setAnnealCoolingRatio(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAnnealItePerStep()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1930
	// ("cv::ml::ANN_MLP_ANNEAL::getAnnealItePerStep", vec![(pred!(const, [], []), _)]),
	void cv_ml_ANN_MLP_ANNEAL_getAnnealItePerStep_const(const cv::ml::ANN_MLP_ANNEAL* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getAnnealItePerStep();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAnnealItePerStep(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1932
	// ("cv::ml::ANN_MLP_ANNEAL::setAnnealItePerStep", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_ml_ANN_MLP_ANNEAL_setAnnealItePerStep_int(cv::ml::ANN_MLP_ANNEAL* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setAnnealItePerStep(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAnnealEnergyRNG(const RNG &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1935
	// ("cv::ml::ANN_MLP_ANNEAL::setAnnealEnergyRNG", vec![(pred!(mut, ["rng"], ["const cv::RNG*"]), _)]),
	void cv_ml_ANN_MLP_ANNEAL_setAnnealEnergyRNG_const_RNGR(cv::ml::ANN_MLP_ANNEAL* instance, const cv::RNG* rng, ResultVoid* ocvrs_return) {
		try {
			instance->setAnnealEnergyRNG(*rng);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::ANN_MLP_ANNEAL::to_ANN_MLP() generated
	// ("cv::ml::ANN_MLP_ANNEAL::to_ANN_MLP", vec![(pred!(mut, [], []), _)]),
	cv::ml::ANN_MLP* cv_ml_ANN_MLP_ANNEAL_to_ANN_MLP(cv::ml::ANN_MLP_ANNEAL* instance) {
			return dynamic_cast<cv::ml::ANN_MLP*>(instance);
	}

	// cv::ml::ANN_MLP_ANNEAL::to_Algorithm() generated
	// ("cv::ml::ANN_MLP_ANNEAL::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ml_ANN_MLP_ANNEAL_to_Algorithm(cv::ml::ANN_MLP_ANNEAL* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ml::ANN_MLP_ANNEAL::to_StatModel() generated
	// ("cv::ml::ANN_MLP_ANNEAL::to_StatModel", vec![(pred!(mut, [], []), _)]),
	cv::ml::StatModel* cv_ml_ANN_MLP_ANNEAL_to_StatModel(cv::ml::ANN_MLP_ANNEAL* instance) {
			return dynamic_cast<cv::ml::StatModel*>(instance);
	}

	// cv::ml::ANN_MLP_ANNEAL::delete() generated
	// ("cv::ml::ANN_MLP_ANNEAL::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ml_ANN_MLP_ANNEAL_delete(cv::ml::ANN_MLP_ANNEAL* instance) {
			delete instance;
	}

	// getBoostType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1338
	// ("cv::ml::Boost::getBoostType", vec![(pred!(const, [], []), _)]),
	void cv_ml_Boost_getBoostType_const(const cv::ml::Boost* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getBoostType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBoostType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1340
	// ("cv::ml::Boost::setBoostType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_ml_Boost_setBoostType_int(cv::ml::Boost* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setBoostType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWeakCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1345
	// ("cv::ml::Boost::getWeakCount", vec![(pred!(const, [], []), _)]),
	void cv_ml_Boost_getWeakCount_const(const cv::ml::Boost* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getWeakCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeakCount(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1347
	// ("cv::ml::Boost::setWeakCount", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_ml_Boost_setWeakCount_int(cv::ml::Boost* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setWeakCount(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWeightTrimRate()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1353
	// ("cv::ml::Boost::getWeightTrimRate", vec![(pred!(const, [], []), _)]),
	void cv_ml_Boost_getWeightTrimRate_const(const cv::ml::Boost* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getWeightTrimRate();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setWeightTrimRate(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1355
	// ("cv::ml::Boost::setWeightTrimRate", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ml_Boost_setWeightTrimRate_double(cv::ml::Boost* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setWeightTrimRate(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1370
	// ("cv::ml::Boost::create", vec![(pred!(mut, [], []), _)]),
	void cv_ml_Boost_create(Result<cv::Ptr<cv::ml::Boost>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::Boost> ret = cv::ml::Boost::create();
			Ok(new cv::Ptr<cv::ml::Boost>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1381
	// ("cv::ml::Boost::load", vec![(pred!(mut, ["filepath", "nodeName"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_ml_Boost_load_const_StringR_const_StringR(const char* filepath, const char* nodeName, Result<cv::Ptr<cv::ml::Boost>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::Boost> ret = cv::ml::Boost::load(cv::String(filepath), cv::String(nodeName));
			Ok(new cv::Ptr<cv::ml::Boost>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::Boost::load(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1381
	// ("cv::ml::Boost::load", vec![(pred!(mut, ["filepath"], ["const cv::String*"]), _)]),
	void cv_ml_Boost_load_const_StringR(const char* filepath, Result<cv::Ptr<cv::ml::Boost>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::Boost> ret = cv::ml::Boost::load(cv::String(filepath));
			Ok(new cv::Ptr<cv::ml::Boost>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::Boost::to_Algorithm() generated
	// ("cv::ml::Boost::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ml_Boost_to_Algorithm(cv::ml::Boost* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ml::Boost::to_DTrees() generated
	// ("cv::ml::Boost::to_DTrees", vec![(pred!(mut, [], []), _)]),
	cv::ml::DTrees* cv_ml_Boost_to_DTrees(cv::ml::Boost* instance) {
			return dynamic_cast<cv::ml::DTrees*>(instance);
	}

	// cv::ml::Boost::to_StatModel() generated
	// ("cv::ml::Boost::to_StatModel", vec![(pred!(mut, [], []), _)]),
	cv::ml::StatModel* cv_ml_Boost_to_StatModel(cv::ml::Boost* instance) {
			return dynamic_cast<cv::ml::StatModel*>(instance);
	}

	// cv::ml::Boost::delete() generated
	// ("cv::ml::Boost::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ml_Boost_delete(cv::ml::Boost* instance) {
			delete instance;
	}

	// getMaxCategories()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1071
	// ("cv::ml::DTrees::getMaxCategories", vec![(pred!(const, [], []), _)]),
	void cv_ml_DTrees_getMaxCategories_const(const cv::ml::DTrees* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxCategories();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxCategories(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1073
	// ("cv::ml::DTrees::setMaxCategories", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_ml_DTrees_setMaxCategories_int(cv::ml::DTrees* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxCategories(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1081
	// ("cv::ml::DTrees::getMaxDepth", vec![(pred!(const, [], []), _)]),
	void cv_ml_DTrees_getMaxDepth_const(const cv::ml::DTrees* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxDepth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1083
	// ("cv::ml::DTrees::setMaxDepth", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_ml_DTrees_setMaxDepth_int(cv::ml::DTrees* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxDepth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinSampleCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1089
	// ("cv::ml::DTrees::getMinSampleCount", vec![(pred!(const, [], []), _)]),
	void cv_ml_DTrees_getMinSampleCount_const(const cv::ml::DTrees* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinSampleCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinSampleCount(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1091
	// ("cv::ml::DTrees::setMinSampleCount", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_ml_DTrees_setMinSampleCount_int(cv::ml::DTrees* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setMinSampleCount(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCVFolds()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1097
	// ("cv::ml::DTrees::getCVFolds", vec![(pred!(const, [], []), _)]),
	void cv_ml_DTrees_getCVFolds_const(const cv::ml::DTrees* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getCVFolds();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCVFolds(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1099
	// ("cv::ml::DTrees::setCVFolds", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_ml_DTrees_setCVFolds_int(cv::ml::DTrees* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setCVFolds(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseSurrogates()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1106
	// ("cv::ml::DTrees::getUseSurrogates", vec![(pred!(const, [], []), _)]),
	void cv_ml_DTrees_getUseSurrogates_const(const cv::ml::DTrees* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseSurrogates();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseSurrogates(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1108
	// ("cv::ml::DTrees::setUseSurrogates", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_ml_DTrees_setUseSurrogates_bool(cv::ml::DTrees* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setUseSurrogates(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUse1SERule()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1114
	// ("cv::ml::DTrees::getUse1SERule", vec![(pred!(const, [], []), _)]),
	void cv_ml_DTrees_getUse1SERule_const(const cv::ml::DTrees* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUse1SERule();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUse1SERule(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1116
	// ("cv::ml::DTrees::setUse1SERule", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_ml_DTrees_setUse1SERule_bool(cv::ml::DTrees* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setUse1SERule(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTruncatePrunedTree()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1122
	// ("cv::ml::DTrees::getTruncatePrunedTree", vec![(pred!(const, [], []), _)]),
	void cv_ml_DTrees_getTruncatePrunedTree_const(const cv::ml::DTrees* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getTruncatePrunedTree();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTruncatePrunedTree(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1124
	// ("cv::ml::DTrees::setTruncatePrunedTree", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_ml_DTrees_setTruncatePrunedTree_bool(cv::ml::DTrees* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setTruncatePrunedTree(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRegressionAccuracy()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1131
	// ("cv::ml::DTrees::getRegressionAccuracy", vec![(pred!(const, [], []), _)]),
	void cv_ml_DTrees_getRegressionAccuracy_const(const cv::ml::DTrees* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getRegressionAccuracy();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRegressionAccuracy(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1133
	// ("cv::ml::DTrees::setRegressionAccuracy", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_ml_DTrees_setRegressionAccuracy_float(cv::ml::DTrees* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setRegressionAccuracy(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getPriors()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1151
	// ("cv::ml::DTrees::getPriors", vec![(pred!(const, [], []), _)]),
	void cv_ml_DTrees_getPriors_const(const cv::ml::DTrees* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getPriors();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setPriors(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1153
	// ("cv::ml::DTrees::setPriors", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_ml_DTrees_setPriors_const_MatR(cv::ml::DTrees* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setPriors(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRoots()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1202
	// ("cv::ml::DTrees::getRoots", vec![(pred!(const, [], []), _)]),
	void cv_ml_DTrees_getRoots_const(const cv::ml::DTrees* instance, Result<std::vector<int>*>* ocvrs_return) {
		try {
			const std::vector<int> ret = instance->getRoots();
			Ok(new const std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNodes()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1207
	// ("cv::ml::DTrees::getNodes", vec![(pred!(const, [], []), _)]),
	void cv_ml_DTrees_getNodes_const(const cv::ml::DTrees* instance, Result<std::vector<cv::ml::DTrees::Node>*>* ocvrs_return) {
		try {
			const std::vector<cv::ml::DTrees::Node> ret = instance->getNodes();
			Ok(new const std::vector<cv::ml::DTrees::Node>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSplits()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1212
	// ("cv::ml::DTrees::getSplits", vec![(pred!(const, [], []), _)]),
	void cv_ml_DTrees_getSplits_const(const cv::ml::DTrees* instance, Result<std::vector<cv::ml::DTrees::Split>*>* ocvrs_return) {
		try {
			const std::vector<cv::ml::DTrees::Split> ret = instance->getSplits();
			Ok(new const std::vector<cv::ml::DTrees::Split>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSubsets()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1217
	// ("cv::ml::DTrees::getSubsets", vec![(pred!(const, [], []), _)]),
	void cv_ml_DTrees_getSubsets_const(const cv::ml::DTrees* instance, Result<std::vector<int>*>* ocvrs_return) {
		try {
			const std::vector<int> ret = instance->getSubsets();
			Ok(new const std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1225
	// ("cv::ml::DTrees::create", vec![(pred!(mut, [], []), _)]),
	void cv_ml_DTrees_create(Result<cv::Ptr<cv::ml::DTrees>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::DTrees> ret = cv::ml::DTrees::create();
			Ok(new cv::Ptr<cv::ml::DTrees>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1236
	// ("cv::ml::DTrees::load", vec![(pred!(mut, ["filepath", "nodeName"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_ml_DTrees_load_const_StringR_const_StringR(const char* filepath, const char* nodeName, Result<cv::Ptr<cv::ml::DTrees>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::DTrees> ret = cv::ml::DTrees::load(cv::String(filepath), cv::String(nodeName));
			Ok(new cv::Ptr<cv::ml::DTrees>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::DTrees::load(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1236
	// ("cv::ml::DTrees::load", vec![(pred!(mut, ["filepath"], ["const cv::String*"]), _)]),
	void cv_ml_DTrees_load_const_StringR(const char* filepath, Result<cv::Ptr<cv::ml::DTrees>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::DTrees> ret = cv::ml::DTrees::load(cv::String(filepath));
			Ok(new cv::Ptr<cv::ml::DTrees>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::DTrees::to_Boost() generated
	// ("cv::ml::DTrees::to_Boost", vec![(pred!(mut, [], []), _)]),
	cv::ml::Boost* cv_ml_DTrees_to_Boost(cv::ml::DTrees* instance) {
			return dynamic_cast<cv::ml::Boost*>(instance);
	}

	// cv::ml::DTrees::to_RTrees() generated
	// ("cv::ml::DTrees::to_RTrees", vec![(pred!(mut, [], []), _)]),
	cv::ml::RTrees* cv_ml_DTrees_to_RTrees(cv::ml::DTrees* instance) {
			return dynamic_cast<cv::ml::RTrees*>(instance);
	}

	// cv::ml::DTrees::to_Algorithm() generated
	// ("cv::ml::DTrees::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ml_DTrees_to_Algorithm(cv::ml::DTrees* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ml::DTrees::to_StatModel() generated
	// ("cv::ml::DTrees::to_StatModel", vec![(pred!(mut, [], []), _)]),
	cv::ml::StatModel* cv_ml_DTrees_to_StatModel(cv::ml::DTrees* instance) {
			return dynamic_cast<cv::ml::StatModel*>(instance);
	}

	// cv::ml::DTrees::delete() generated
	// ("cv::ml::DTrees::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ml_DTrees_delete(cv::ml::DTrees* instance) {
			delete instance;
	}

	// Node()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1160
	// ("cv::ml::DTrees::Node::Node", vec![(pred!(mut, [], []), _)]),
	void cv_ml_DTrees_Node_Node(Result<cv::ml::DTrees::Node*>* ocvrs_return) {
		try {
			cv::ml::DTrees::Node* ret = new cv::ml::DTrees::Node();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::DTrees::Node::value() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1161
	// ("cv::ml::DTrees::Node::value", vec![(pred!(const, [], []), _)]),
	double cv_ml_DTrees_Node_propValue_const(const cv::ml::DTrees::Node* instance) {
			double ret = instance->value;
			return ret;
	}

	// cv::ml::DTrees::Node::setValue(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1161
	// ("cv::ml::DTrees::Node::setValue", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_ml_DTrees_Node_propValue_const_double(cv::ml::DTrees::Node* instance, const double val) {
			instance->value = val;
	}

	// cv::ml::DTrees::Node::classIdx() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1163
	// ("cv::ml::DTrees::Node::classIdx", vec![(pred!(const, [], []), _)]),
	int cv_ml_DTrees_Node_propClassIdx_const(const cv::ml::DTrees::Node* instance) {
			int ret = instance->classIdx;
			return ret;
	}

	// cv::ml::DTrees::Node::setClassIdx(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1163
	// ("cv::ml::DTrees::Node::setClassIdx", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_ml_DTrees_Node_propClassIdx_const_int(cv::ml::DTrees::Node* instance, const int val) {
			instance->classIdx = val;
	}

	// cv::ml::DTrees::Node::parent() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1165
	// ("cv::ml::DTrees::Node::parent", vec![(pred!(const, [], []), _)]),
	int cv_ml_DTrees_Node_propParent_const(const cv::ml::DTrees::Node* instance) {
			int ret = instance->parent;
			return ret;
	}

	// cv::ml::DTrees::Node::setParent(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1165
	// ("cv::ml::DTrees::Node::setParent", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_ml_DTrees_Node_propParent_const_int(cv::ml::DTrees::Node* instance, const int val) {
			instance->parent = val;
	}

	// cv::ml::DTrees::Node::left() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1166
	// ("cv::ml::DTrees::Node::left", vec![(pred!(const, [], []), _)]),
	int cv_ml_DTrees_Node_propLeft_const(const cv::ml::DTrees::Node* instance) {
			int ret = instance->left;
			return ret;
	}

	// cv::ml::DTrees::Node::setLeft(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1166
	// ("cv::ml::DTrees::Node::setLeft", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_ml_DTrees_Node_propLeft_const_int(cv::ml::DTrees::Node* instance, const int val) {
			instance->left = val;
	}

	// cv::ml::DTrees::Node::right() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1167
	// ("cv::ml::DTrees::Node::right", vec![(pred!(const, [], []), _)]),
	int cv_ml_DTrees_Node_propRight_const(const cv::ml::DTrees::Node* instance) {
			int ret = instance->right;
			return ret;
	}

	// cv::ml::DTrees::Node::setRight(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1167
	// ("cv::ml::DTrees::Node::setRight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_ml_DTrees_Node_propRight_const_int(cv::ml::DTrees::Node* instance, const int val) {
			instance->right = val;
	}

	// cv::ml::DTrees::Node::defaultDir() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1168
	// ("cv::ml::DTrees::Node::defaultDir", vec![(pred!(const, [], []), _)]),
	int cv_ml_DTrees_Node_propDefaultDir_const(const cv::ml::DTrees::Node* instance) {
			int ret = instance->defaultDir;
			return ret;
	}

	// cv::ml::DTrees::Node::setDefaultDir(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1168
	// ("cv::ml::DTrees::Node::setDefaultDir", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_ml_DTrees_Node_propDefaultDir_const_int(cv::ml::DTrees::Node* instance, const int val) {
			instance->defaultDir = val;
	}

	// cv::ml::DTrees::Node::split() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1170
	// ("cv::ml::DTrees::Node::split", vec![(pred!(const, [], []), _)]),
	int cv_ml_DTrees_Node_propSplit_const(const cv::ml::DTrees::Node* instance) {
			int ret = instance->split;
			return ret;
	}

	// cv::ml::DTrees::Node::setSplit(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1170
	// ("cv::ml::DTrees::Node::setSplit", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_ml_DTrees_Node_propSplit_const_int(cv::ml::DTrees::Node* instance, const int val) {
			instance->split = val;
	}

	// cv::ml::DTrees::Node::delete() generated
	// ("cv::ml::DTrees::Node::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ml_DTrees_Node_delete(cv::ml::DTrees::Node* instance) {
			delete instance;
	}

	// Split()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1178
	// ("cv::ml::DTrees::Split::Split", vec![(pred!(mut, [], []), _)]),
	void cv_ml_DTrees_Split_Split(Result<cv::ml::DTrees::Split*>* ocvrs_return) {
		try {
			cv::ml::DTrees::Split* ret = new cv::ml::DTrees::Split();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::DTrees::Split::varIdx() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1179
	// ("cv::ml::DTrees::Split::varIdx", vec![(pred!(const, [], []), _)]),
	int cv_ml_DTrees_Split_propVarIdx_const(const cv::ml::DTrees::Split* instance) {
			int ret = instance->varIdx;
			return ret;
	}

	// cv::ml::DTrees::Split::setVarIdx(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1179
	// ("cv::ml::DTrees::Split::setVarIdx", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_ml_DTrees_Split_propVarIdx_const_int(cv::ml::DTrees::Split* instance, const int val) {
			instance->varIdx = val;
	}

	// cv::ml::DTrees::Split::inversed() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1180
	// ("cv::ml::DTrees::Split::inversed", vec![(pred!(const, [], []), _)]),
	bool cv_ml_DTrees_Split_propInversed_const(const cv::ml::DTrees::Split* instance) {
			bool ret = instance->inversed;
			return ret;
	}

	// cv::ml::DTrees::Split::setInversed(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1180
	// ("cv::ml::DTrees::Split::setInversed", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	void cv_ml_DTrees_Split_propInversed_const_bool(cv::ml::DTrees::Split* instance, const bool val) {
			instance->inversed = val;
	}

	// cv::ml::DTrees::Split::quality() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1182
	// ("cv::ml::DTrees::Split::quality", vec![(pred!(const, [], []), _)]),
	float cv_ml_DTrees_Split_propQuality_const(const cv::ml::DTrees::Split* instance) {
			float ret = instance->quality;
			return ret;
	}

	// cv::ml::DTrees::Split::setQuality(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1182
	// ("cv::ml::DTrees::Split::setQuality", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_ml_DTrees_Split_propQuality_const_float(cv::ml::DTrees::Split* instance, const float val) {
			instance->quality = val;
	}

	// cv::ml::DTrees::Split::next() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1183
	// ("cv::ml::DTrees::Split::next", vec![(pred!(const, [], []), _)]),
	int cv_ml_DTrees_Split_propNext_const(const cv::ml::DTrees::Split* instance) {
			int ret = instance->next;
			return ret;
	}

	// cv::ml::DTrees::Split::setNext(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1183
	// ("cv::ml::DTrees::Split::setNext", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_ml_DTrees_Split_propNext_const_int(cv::ml::DTrees::Split* instance, const int val) {
			instance->next = val;
	}

	// cv::ml::DTrees::Split::c() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1184
	// ("cv::ml::DTrees::Split::c", vec![(pred!(const, [], []), _)]),
	float cv_ml_DTrees_Split_propC_const(const cv::ml::DTrees::Split* instance) {
			float ret = instance->c;
			return ret;
	}

	// cv::ml::DTrees::Split::setC(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1184
	// ("cv::ml::DTrees::Split::setC", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	void cv_ml_DTrees_Split_propC_const_float(cv::ml::DTrees::Split* instance, const float val) {
			instance->c = val;
	}

	// cv::ml::DTrees::Split::subsetOfs() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1191
	// ("cv::ml::DTrees::Split::subsetOfs", vec![(pred!(const, [], []), _)]),
	int cv_ml_DTrees_Split_propSubsetOfs_const(const cv::ml::DTrees::Split* instance) {
			int ret = instance->subsetOfs;
			return ret;
	}

	// cv::ml::DTrees::Split::setSubsetOfs(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1191
	// ("cv::ml::DTrees::Split::setSubsetOfs", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	void cv_ml_DTrees_Split_propSubsetOfs_const_int(cv::ml::DTrees::Split* instance, const int val) {
			instance->subsetOfs = val;
	}

	// cv::ml::DTrees::Split::delete() generated
	// ("cv::ml::DTrees::Split::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ml_DTrees_Split_delete(cv::ml::DTrees::Split* instance) {
			delete instance;
	}

	// getClustersNumber()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:871
	// ("cv::ml::EM::getClustersNumber", vec![(pred!(const, [], []), _)]),
	void cv_ml_EM_getClustersNumber_const(const cv::ml::EM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getClustersNumber();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setClustersNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:873
	// ("cv::ml::EM::setClustersNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_ml_EM_setClustersNumber_int(cv::ml::EM* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setClustersNumber(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCovarianceMatrixType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:878
	// ("cv::ml::EM::getCovarianceMatrixType", vec![(pred!(const, [], []), _)]),
	void cv_ml_EM_getCovarianceMatrixType_const(const cv::ml::EM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getCovarianceMatrixType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCovarianceMatrixType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:880
	// ("cv::ml::EM::setCovarianceMatrixType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_ml_EM_setCovarianceMatrixType_int(cv::ml::EM* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setCovarianceMatrixType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTermCriteria()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:887
	// ("cv::ml::EM::getTermCriteria", vec![(pred!(const, [], []), _)]),
	void cv_ml_EM_getTermCriteria_const(const cv::ml::EM* instance, Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTermCriteria(const TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:889
	// ("cv::ml::EM::setTermCriteria", vec![(pred!(mut, ["val"], ["const cv::TermCriteria*"]), _)]),
	void cv_ml_EM_setTermCriteria_const_TermCriteriaR(cv::ml::EM* instance, const cv::TermCriteria* val, ResultVoid* ocvrs_return) {
		try {
			instance->setTermCriteria(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getWeights()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:895
	// ("cv::ml::EM::getWeights", vec![(pred!(const, [], []), _)]),
	void cv_ml_EM_getWeights_const(const cv::ml::EM* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getWeights();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMeans()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:901
	// ("cv::ml::EM::getMeans", vec![(pred!(const, [], []), _)]),
	void cv_ml_EM_getMeans_const(const cv::ml::EM* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMeans();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCovs(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:907
	// ("cv::ml::EM::getCovs", vec![(pred!(const, ["covs"], ["std::vector<cv::Mat>*"]), _)]),
	void cv_ml_EM_getCovs_const_vectorLMatGR(const cv::ml::EM* instance, std::vector<cv::Mat>* covs, ResultVoid* ocvrs_return) {
		try {
			instance->getCovs(*covs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// predict(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:916
	// ("cv::ml::EM::predict", vec![(pred!(const, ["samples", "results", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_ml_EM_predict_const_const__InputArrayR_const__OutputArrayR_int(const cv::ml::EM* instance, const cv::_InputArray* samples, const cv::_OutputArray* results, int flags, Result<float>* ocvrs_return) {
		try {
			float ret = instance->predict(*samples, *results, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::EM::predict(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:916
	// ("cv::ml::EM::predict", vec![(pred!(const, ["samples"], ["const cv::_InputArray*"]), _)]),
	void cv_ml_EM_predict_const_const__InputArrayR(const cv::ml::EM* instance, const cv::_InputArray* samples, Result<float>* ocvrs_return) {
		try {
			float ret = instance->predict(*samples);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// predict2(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:930
	// ("cv::ml::EM::predict2", vec![(pred!(const, ["sample", "probs"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ml_EM_predict2_const_const__InputArrayR_const__OutputArrayR(const cv::ml::EM* instance, const cv::_InputArray* sample, const cv::_OutputArray* probs, Result<cv::Vec2d>* ocvrs_return) {
		try {
			cv::Vec2d ret = instance->predict2(*sample, *probs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// trainEM(InputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:960
	// ("cv::ml::EM::trainEM", vec![(pred!(mut, ["samples", "logLikelihoods", "labels", "probs"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ml_EM_trainEM_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::ml::EM* instance, const cv::_InputArray* samples, const cv::_OutputArray* logLikelihoods, const cv::_OutputArray* labels, const cv::_OutputArray* probs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->trainEM(*samples, *logLikelihoods, *labels, *probs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::EM::trainEM(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:960
	// ("cv::ml::EM::trainEM", vec![(pred!(mut, ["samples"], ["const cv::_InputArray*"]), _)]),
	void cv_ml_EM_trainEM_const__InputArrayR(cv::ml::EM* instance, const cv::_InputArray* samples, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->trainEM(*samples);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// trainE(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:992
	// ("cv::ml::EM::trainE", vec![(pred!(mut, ["samples", "means0", "covs0", "weights0", "logLikelihoods", "labels", "probs"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ml_EM_trainE_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::ml::EM* instance, const cv::_InputArray* samples, const cv::_InputArray* means0, const cv::_InputArray* covs0, const cv::_InputArray* weights0, const cv::_OutputArray* logLikelihoods, const cv::_OutputArray* labels, const cv::_OutputArray* probs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->trainE(*samples, *means0, *covs0, *weights0, *logLikelihoods, *labels, *probs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::EM::trainE(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:992
	// ("cv::ml::EM::trainE", vec![(pred!(mut, ["samples", "means0"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_ml_EM_trainE_const__InputArrayR_const__InputArrayR(cv::ml::EM* instance, const cv::_InputArray* samples, const cv::_InputArray* means0, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->trainE(*samples, *means0);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// trainM(InputArray, InputArray, OutputArray, OutputArray, OutputArray)(InputArray, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1017
	// ("cv::ml::EM::trainM", vec![(pred!(mut, ["samples", "probs0", "logLikelihoods", "labels", "probs"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ml_EM_trainM_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::ml::EM* instance, const cv::_InputArray* samples, const cv::_InputArray* probs0, const cv::_OutputArray* logLikelihoods, const cv::_OutputArray* labels, const cv::_OutputArray* probs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->trainM(*samples, *probs0, *logLikelihoods, *labels, *probs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::EM::trainM(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1017
	// ("cv::ml::EM::trainM", vec![(pred!(mut, ["samples", "probs0"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_ml_EM_trainM_const__InputArrayR_const__InputArrayR(cv::ml::EM* instance, const cv::_InputArray* samples, const cv::_InputArray* probs0, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->trainM(*samples, *probs0);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1026
	// ("cv::ml::EM::create", vec![(pred!(mut, [], []), _)]),
	void cv_ml_EM_create(Result<cv::Ptr<cv::ml::EM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::EM> ret = cv::ml::EM::create();
			Ok(new cv::Ptr<cv::ml::EM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1037
	// ("cv::ml::EM::load", vec![(pred!(mut, ["filepath", "nodeName"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_ml_EM_load_const_StringR_const_StringR(const char* filepath, const char* nodeName, Result<cv::Ptr<cv::ml::EM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::EM> ret = cv::ml::EM::load(cv::String(filepath), cv::String(nodeName));
			Ok(new cv::Ptr<cv::ml::EM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::EM::load(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1037
	// ("cv::ml::EM::load", vec![(pred!(mut, ["filepath"], ["const cv::String*"]), _)]),
	void cv_ml_EM_load_const_StringR(const char* filepath, Result<cv::Ptr<cv::ml::EM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::EM> ret = cv::ml::EM::load(cv::String(filepath));
			Ok(new cv::Ptr<cv::ml::EM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::EM::to_Algorithm() generated
	// ("cv::ml::EM::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ml_EM_to_Algorithm(cv::ml::EM* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ml::EM::to_StatModel() generated
	// ("cv::ml::EM::to_StatModel", vec![(pred!(mut, [], []), _)]),
	cv::ml::StatModel* cv_ml_EM_to_StatModel(cv::ml::EM* instance) {
			return dynamic_cast<cv::ml::StatModel*>(instance);
	}

	// cv::ml::EM::delete() generated
	// ("cv::ml::EM::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ml_EM_delete(cv::ml::EM* instance) {
			delete instance;
	}

	// getDefaultK()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:442
	// ("cv::ml::KNearest::getDefaultK", vec![(pred!(const, [], []), _)]),
	void cv_ml_KNearest_getDefaultK_const(const cv::ml::KNearest* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDefaultK();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDefaultK(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:444
	// ("cv::ml::KNearest::setDefaultK", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_ml_KNearest_setDefaultK_int(cv::ml::KNearest* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setDefaultK(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIsClassifier()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:448
	// ("cv::ml::KNearest::getIsClassifier", vec![(pred!(const, [], []), _)]),
	void cv_ml_KNearest_getIsClassifier_const(const cv::ml::KNearest* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getIsClassifier();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIsClassifier(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:450
	// ("cv::ml::KNearest::setIsClassifier", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_ml_KNearest_setIsClassifier_bool(cv::ml::KNearest* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setIsClassifier(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getEmax()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:454
	// ("cv::ml::KNearest::getEmax", vec![(pred!(const, [], []), _)]),
	void cv_ml_KNearest_getEmax_const(const cv::ml::KNearest* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getEmax();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setEmax(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:456
	// ("cv::ml::KNearest::setEmax", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_ml_KNearest_setEmax_int(cv::ml::KNearest* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setEmax(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getAlgorithmType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:460
	// ("cv::ml::KNearest::getAlgorithmType", vec![(pred!(const, [], []), _)]),
	void cv_ml_KNearest_getAlgorithmType_const(const cv::ml::KNearest* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getAlgorithmType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setAlgorithmType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:462
	// ("cv::ml::KNearest::setAlgorithmType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_ml_KNearest_setAlgorithmType_int(cv::ml::KNearest* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setAlgorithmType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// findNearest(InputArray, int, OutputArray, OutputArray, OutputArray)(InputArray, Primitive, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:490
	// ("cv::ml::KNearest::findNearest", vec![(pred!(const, ["samples", "k", "results", "neighborResponses", "dist"], ["const cv::_InputArray*", "int", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ml_KNearest_findNearest_const_const__InputArrayR_int_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::ml::KNearest* instance, const cv::_InputArray* samples, int k, const cv::_OutputArray* results, const cv::_OutputArray* neighborResponses, const cv::_OutputArray* dist, Result<float>* ocvrs_return) {
		try {
			float ret = instance->findNearest(*samples, k, *results, *neighborResponses, *dist);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::KNearest::findNearest(InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:490
	// ("cv::ml::KNearest::findNearest", vec![(pred!(const, ["samples", "k", "results"], ["const cv::_InputArray*", "int", "const cv::_OutputArray*"]), _)]),
	void cv_ml_KNearest_findNearest_const_const__InputArrayR_int_const__OutputArrayR(const cv::ml::KNearest* instance, const cv::_InputArray* samples, int k, const cv::_OutputArray* results, Result<float>* ocvrs_return) {
		try {
			float ret = instance->findNearest(*samples, k, *results);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:507
	// ("cv::ml::KNearest::create", vec![(pred!(mut, [], []), _)]),
	void cv_ml_KNearest_create(Result<cv::Ptr<cv::ml::KNearest>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::KNearest> ret = cv::ml::KNearest::create();
			Ok(new cv::Ptr<cv::ml::KNearest>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:515
	// ("cv::ml::KNearest::load", vec![(pred!(mut, ["filepath"], ["const cv::String*"]), _)]),
	void cv_ml_KNearest_load_const_StringR(const char* filepath, Result<cv::Ptr<cv::ml::KNearest>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::KNearest> ret = cv::ml::KNearest::load(cv::String(filepath));
			Ok(new cv::Ptr<cv::ml::KNearest>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::KNearest::to_Algorithm() generated
	// ("cv::ml::KNearest::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ml_KNearest_to_Algorithm(cv::ml::KNearest* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ml::KNearest::to_StatModel() generated
	// ("cv::ml::KNearest::to_StatModel", vec![(pred!(mut, [], []), _)]),
	cv::ml::StatModel* cv_ml_KNearest_to_StatModel(cv::ml::KNearest* instance) {
			return dynamic_cast<cv::ml::StatModel*>(instance);
	}

	// cv::ml::KNearest::delete() generated
	// ("cv::ml::KNearest::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ml_KNearest_delete(cv::ml::KNearest* instance) {
			delete instance;
	}

	// getLearningRate()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1631
	// ("cv::ml::LogisticRegression::getLearningRate", vec![(pred!(const, [], []), _)]),
	void cv_ml_LogisticRegression_getLearningRate_const(const cv::ml::LogisticRegression* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getLearningRate();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setLearningRate(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1633
	// ("cv::ml::LogisticRegression::setLearningRate", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ml_LogisticRegression_setLearningRate_double(cv::ml::LogisticRegression* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setLearningRate(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIterations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1637
	// ("cv::ml::LogisticRegression::getIterations", vec![(pred!(const, [], []), _)]),
	void cv_ml_LogisticRegression_getIterations_const(const cv::ml::LogisticRegression* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1639
	// ("cv::ml::LogisticRegression::setIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_ml_LogisticRegression_setIterations_int(cv::ml::LogisticRegression* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRegularization()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1643
	// ("cv::ml::LogisticRegression::getRegularization", vec![(pred!(const, [], []), _)]),
	void cv_ml_LogisticRegression_getRegularization_const(const cv::ml::LogisticRegression* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getRegularization();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRegularization(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1645
	// ("cv::ml::LogisticRegression::setRegularization", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_ml_LogisticRegression_setRegularization_int(cv::ml::LogisticRegression* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setRegularization(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTrainMethod()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1649
	// ("cv::ml::LogisticRegression::getTrainMethod", vec![(pred!(const, [], []), _)]),
	void cv_ml_LogisticRegression_getTrainMethod_const(const cv::ml::LogisticRegression* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTrainMethod();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTrainMethod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1651
	// ("cv::ml::LogisticRegression::setTrainMethod", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_ml_LogisticRegression_setTrainMethod_int(cv::ml::LogisticRegression* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setTrainMethod(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMiniBatchSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1657
	// ("cv::ml::LogisticRegression::getMiniBatchSize", vec![(pred!(const, [], []), _)]),
	void cv_ml_LogisticRegression_getMiniBatchSize_const(const cv::ml::LogisticRegression* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMiniBatchSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMiniBatchSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1659
	// ("cv::ml::LogisticRegression::setMiniBatchSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_ml_LogisticRegression_setMiniBatchSize_int(cv::ml::LogisticRegression* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setMiniBatchSize(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTermCriteria()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1663
	// ("cv::ml::LogisticRegression::getTermCriteria", vec![(pred!(const, [], []), _)]),
	void cv_ml_LogisticRegression_getTermCriteria_const(const cv::ml::LogisticRegression* instance, Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTermCriteria(TermCriteria)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1665
	// ("cv::ml::LogisticRegression::setTermCriteria", vec![(pred!(mut, ["val"], ["cv::TermCriteria"]), _)]),
	void cv_ml_LogisticRegression_setTermCriteria_TermCriteria(cv::ml::LogisticRegression* instance, cv::TermCriteria* val, ResultVoid* ocvrs_return) {
		try {
			instance->setTermCriteria(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// predict(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1687
	// ("cv::ml::LogisticRegression::predict", vec![(pred!(const, ["samples", "results", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_ml_LogisticRegression_predict_const_const__InputArrayR_const__OutputArrayR_int(const cv::ml::LogisticRegression* instance, const cv::_InputArray* samples, const cv::_OutputArray* results, int flags, Result<float>* ocvrs_return) {
		try {
			float ret = instance->predict(*samples, *results, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::LogisticRegression::predict(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1687
	// ("cv::ml::LogisticRegression::predict", vec![(pred!(const, ["samples"], ["const cv::_InputArray*"]), _)]),
	void cv_ml_LogisticRegression_predict_const_const__InputArrayR(const cv::ml::LogisticRegression* instance, const cv::_InputArray* samples, Result<float>* ocvrs_return) {
		try {
			float ret = instance->predict(*samples);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// get_learnt_thetas()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1694
	// ("cv::ml::LogisticRegression::get_learnt_thetas", vec![(pred!(const, [], []), _)]),
	void cv_ml_LogisticRegression_get_learnt_thetas_const(const cv::ml::LogisticRegression* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->get_learnt_thetas();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1700
	// ("cv::ml::LogisticRegression::create", vec![(pred!(mut, [], []), _)]),
	void cv_ml_LogisticRegression_create(Result<cv::Ptr<cv::ml::LogisticRegression>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::LogisticRegression> ret = cv::ml::LogisticRegression::create();
			Ok(new cv::Ptr<cv::ml::LogisticRegression>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1711
	// ("cv::ml::LogisticRegression::load", vec![(pred!(mut, ["filepath", "nodeName"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_ml_LogisticRegression_load_const_StringR_const_StringR(const char* filepath, const char* nodeName, Result<cv::Ptr<cv::ml::LogisticRegression>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::LogisticRegression> ret = cv::ml::LogisticRegression::load(cv::String(filepath), cv::String(nodeName));
			Ok(new cv::Ptr<cv::ml::LogisticRegression>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::LogisticRegression::load(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1711
	// ("cv::ml::LogisticRegression::load", vec![(pred!(mut, ["filepath"], ["const cv::String*"]), _)]),
	void cv_ml_LogisticRegression_load_const_StringR(const char* filepath, Result<cv::Ptr<cv::ml::LogisticRegression>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::LogisticRegression> ret = cv::ml::LogisticRegression::load(cv::String(filepath));
			Ok(new cv::Ptr<cv::ml::LogisticRegression>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::LogisticRegression::to_Algorithm() generated
	// ("cv::ml::LogisticRegression::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ml_LogisticRegression_to_Algorithm(cv::ml::LogisticRegression* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ml::LogisticRegression::to_StatModel() generated
	// ("cv::ml::LogisticRegression::to_StatModel", vec![(pred!(mut, [], []), _)]),
	cv::ml::StatModel* cv_ml_LogisticRegression_to_StatModel(cv::ml::LogisticRegression* instance) {
			return dynamic_cast<cv::ml::StatModel*>(instance);
	}

	// cv::ml::LogisticRegression::delete() generated
	// ("cv::ml::LogisticRegression::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ml_LogisticRegression_delete(cv::ml::LogisticRegression* instance) {
			delete instance;
	}

	// predictProb(InputArray, OutputArray, OutputArray, int)(InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:409
	// ("cv::ml::NormalBayesClassifier::predictProb", vec![(pred!(const, ["inputs", "outputs", "outputProbs", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_ml_NormalBayesClassifier_predictProb_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int(const cv::ml::NormalBayesClassifier* instance, const cv::_InputArray* inputs, const cv::_OutputArray* outputs, const cv::_OutputArray* outputProbs, int flags, Result<float>* ocvrs_return) {
		try {
			float ret = instance->predictProb(*inputs, *outputs, *outputProbs, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::NormalBayesClassifier::predictProb(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:409
	// ("cv::ml::NormalBayesClassifier::predictProb", vec![(pred!(const, ["inputs", "outputs", "outputProbs"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ml_NormalBayesClassifier_predictProb_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::ml::NormalBayesClassifier* instance, const cv::_InputArray* inputs, const cv::_OutputArray* outputs, const cv::_OutputArray* outputProbs, Result<float>* ocvrs_return) {
		try {
			float ret = instance->predictProb(*inputs, *outputs, *outputProbs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:414
	// ("cv::ml::NormalBayesClassifier::create", vec![(pred!(mut, [], []), _)]),
	void cv_ml_NormalBayesClassifier_create(Result<cv::Ptr<cv::ml::NormalBayesClassifier>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::NormalBayesClassifier> ret = cv::ml::NormalBayesClassifier::create();
			Ok(new cv::Ptr<cv::ml::NormalBayesClassifier>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:425
	// ("cv::ml::NormalBayesClassifier::load", vec![(pred!(mut, ["filepath", "nodeName"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_ml_NormalBayesClassifier_load_const_StringR_const_StringR(const char* filepath, const char* nodeName, Result<cv::Ptr<cv::ml::NormalBayesClassifier>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::NormalBayesClassifier> ret = cv::ml::NormalBayesClassifier::load(cv::String(filepath), cv::String(nodeName));
			Ok(new cv::Ptr<cv::ml::NormalBayesClassifier>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::NormalBayesClassifier::load(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:425
	// ("cv::ml::NormalBayesClassifier::load", vec![(pred!(mut, ["filepath"], ["const cv::String*"]), _)]),
	void cv_ml_NormalBayesClassifier_load_const_StringR(const char* filepath, Result<cv::Ptr<cv::ml::NormalBayesClassifier>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::NormalBayesClassifier> ret = cv::ml::NormalBayesClassifier::load(cv::String(filepath));
			Ok(new cv::Ptr<cv::ml::NormalBayesClassifier>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::NormalBayesClassifier::to_Algorithm() generated
	// ("cv::ml::NormalBayesClassifier::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ml_NormalBayesClassifier_to_Algorithm(cv::ml::NormalBayesClassifier* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ml::NormalBayesClassifier::to_StatModel() generated
	// ("cv::ml::NormalBayesClassifier::to_StatModel", vec![(pred!(mut, [], []), _)]),
	cv::ml::StatModel* cv_ml_NormalBayesClassifier_to_StatModel(cv::ml::NormalBayesClassifier* instance) {
			return dynamic_cast<cv::ml::StatModel*>(instance);
	}

	// cv::ml::NormalBayesClassifier::delete() generated
	// ("cv::ml::NormalBayesClassifier::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ml_NormalBayesClassifier_delete(cv::ml::NormalBayesClassifier* instance) {
			delete instance;
	}

	// ParamGrid()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:111
	// ("cv::ml::ParamGrid::ParamGrid", vec![(pred!(mut, [], []), _)]),
	void cv_ml_ParamGrid_ParamGrid(Result<cv::ml::ParamGrid*>* ocvrs_return) {
		try {
			cv::ml::ParamGrid* ret = new cv::ml::ParamGrid();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// ParamGrid(double, double, double)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:113
	// ("cv::ml::ParamGrid::ParamGrid", vec![(pred!(mut, ["_minVal", "_maxVal", "_logStep"], ["double", "double", "double"]), _)]),
	void cv_ml_ParamGrid_ParamGrid_double_double_double(double _minVal, double _maxVal, double _logStep, Result<cv::ml::ParamGrid*>* ocvrs_return) {
		try {
			cv::ml::ParamGrid* ret = new cv::ml::ParamGrid(_minVal, _maxVal, _logStep);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(double, double, double)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:133
	// ("cv::ml::ParamGrid::create", vec![(pred!(mut, ["minVal", "maxVal", "logstep"], ["double", "double", "double"]), _)]),
	void cv_ml_ParamGrid_create_double_double_double(double minVal, double maxVal, double logstep, Result<cv::Ptr<cv::ml::ParamGrid>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::ParamGrid> ret = cv::ml::ParamGrid::create(minVal, maxVal, logstep);
			Ok(new cv::Ptr<cv::ml::ParamGrid>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::ParamGrid::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:133
	// ("cv::ml::ParamGrid::create", vec![(pred!(mut, [], []), _)]),
	void cv_ml_ParamGrid_create(Result<cv::Ptr<cv::ml::ParamGrid>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::ParamGrid> ret = cv::ml::ParamGrid::create();
			Ok(new cv::Ptr<cv::ml::ParamGrid>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::ParamGrid::minVal() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:115
	// ("cv::ml::ParamGrid::minVal", vec![(pred!(const, [], []), _)]),
	double cv_ml_ParamGrid_propMinVal_const(const cv::ml::ParamGrid* instance) {
			double ret = instance->minVal;
			return ret;
	}

	// cv::ml::ParamGrid::setMinVal(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:115
	// ("cv::ml::ParamGrid::setMinVal", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_ml_ParamGrid_propMinVal_const_double(cv::ml::ParamGrid* instance, const double val) {
			instance->minVal = val;
	}

	// cv::ml::ParamGrid::maxVal() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:116
	// ("cv::ml::ParamGrid::maxVal", vec![(pred!(const, [], []), _)]),
	double cv_ml_ParamGrid_propMaxVal_const(const cv::ml::ParamGrid* instance) {
			double ret = instance->maxVal;
			return ret;
	}

	// cv::ml::ParamGrid::setMaxVal(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:116
	// ("cv::ml::ParamGrid::setMaxVal", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_ml_ParamGrid_propMaxVal_const_double(cv::ml::ParamGrid* instance, const double val) {
			instance->maxVal = val;
	}

	// cv::ml::ParamGrid::logStep() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:125
	// ("cv::ml::ParamGrid::logStep", vec![(pred!(const, [], []), _)]),
	double cv_ml_ParamGrid_propLogStep_const(const cv::ml::ParamGrid* instance) {
			double ret = instance->logStep;
			return ret;
	}

	// cv::ml::ParamGrid::setLogStep(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:125
	// ("cv::ml::ParamGrid::setLogStep", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	void cv_ml_ParamGrid_propLogStep_const_double(cv::ml::ParamGrid* instance, const double val) {
			instance->logStep = val;
	}

	// cv::ml::ParamGrid::delete() generated
	// ("cv::ml::ParamGrid::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ml_ParamGrid_delete(cv::ml::ParamGrid* instance) {
			delete instance;
	}

	// getCalculateVarImportance()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1254
	// ("cv::ml::RTrees::getCalculateVarImportance", vec![(pred!(const, [], []), _)]),
	void cv_ml_RTrees_getCalculateVarImportance_const(const cv::ml::RTrees* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getCalculateVarImportance();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCalculateVarImportance(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1256
	// ("cv::ml::RTrees::setCalculateVarImportance", vec![(pred!(mut, ["val"], ["bool"]), _)]),
	void cv_ml_RTrees_setCalculateVarImportance_bool(cv::ml::RTrees* instance, bool val, ResultVoid* ocvrs_return) {
		try {
			instance->setCalculateVarImportance(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getActiveVarCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1263
	// ("cv::ml::RTrees::getActiveVarCount", vec![(pred!(const, [], []), _)]),
	void cv_ml_RTrees_getActiveVarCount_const(const cv::ml::RTrees* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getActiveVarCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setActiveVarCount(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1265
	// ("cv::ml::RTrees::setActiveVarCount", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_ml_RTrees_setActiveVarCount_int(cv::ml::RTrees* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setActiveVarCount(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTermCriteria()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1275
	// ("cv::ml::RTrees::getTermCriteria", vec![(pred!(const, [], []), _)]),
	void cv_ml_RTrees_getTermCriteria_const(const cv::ml::RTrees* instance, Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTermCriteria(const TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1277
	// ("cv::ml::RTrees::setTermCriteria", vec![(pred!(mut, ["val"], ["const cv::TermCriteria*"]), _)]),
	void cv_ml_RTrees_setTermCriteria_const_TermCriteriaR(cv::ml::RTrees* instance, const cv::TermCriteria* val, ResultVoid* ocvrs_return) {
		try {
			instance->setTermCriteria(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVarImportance()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1284
	// ("cv::ml::RTrees::getVarImportance", vec![(pred!(const, [], []), _)]),
	void cv_ml_RTrees_getVarImportance_const(const cv::ml::RTrees* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getVarImportance();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVotes(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1295
	// ("cv::ml::RTrees::getVotes", vec![(pred!(const, ["samples", "results", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_ml_RTrees_getVotes_const_const__InputArrayR_const__OutputArrayR_int(const cv::ml::RTrees* instance, const cv::_InputArray* samples, const cv::_OutputArray* results, int flags, ResultVoid* ocvrs_return) {
		try {
			instance->getVotes(*samples, *results, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOOBError()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1301
	// ("cv::ml::RTrees::getOOBError", vec![(pred!(const, [], []), _)]),
	void cv_ml_RTrees_getOOBError_const(const cv::ml::RTrees* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getOOBError();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1310
	// ("cv::ml::RTrees::create", vec![(pred!(mut, [], []), _)]),
	void cv_ml_RTrees_create(Result<cv::Ptr<cv::ml::RTrees>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::RTrees> ret = cv::ml::RTrees::create();
			Ok(new cv::Ptr<cv::ml::RTrees>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1321
	// ("cv::ml::RTrees::load", vec![(pred!(mut, ["filepath", "nodeName"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_ml_RTrees_load_const_StringR_const_StringR(const char* filepath, const char* nodeName, Result<cv::Ptr<cv::ml::RTrees>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::RTrees> ret = cv::ml::RTrees::load(cv::String(filepath), cv::String(nodeName));
			Ok(new cv::Ptr<cv::ml::RTrees>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::RTrees::load(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1321
	// ("cv::ml::RTrees::load", vec![(pred!(mut, ["filepath"], ["const cv::String*"]), _)]),
	void cv_ml_RTrees_load_const_StringR(const char* filepath, Result<cv::Ptr<cv::ml::RTrees>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::RTrees> ret = cv::ml::RTrees::load(cv::String(filepath));
			Ok(new cv::Ptr<cv::ml::RTrees>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::RTrees::to_Algorithm() generated
	// ("cv::ml::RTrees::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ml_RTrees_to_Algorithm(cv::ml::RTrees* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ml::RTrees::to_DTrees() generated
	// ("cv::ml::RTrees::to_DTrees", vec![(pred!(mut, [], []), _)]),
	cv::ml::DTrees* cv_ml_RTrees_to_DTrees(cv::ml::RTrees* instance) {
			return dynamic_cast<cv::ml::DTrees*>(instance);
	}

	// cv::ml::RTrees::to_StatModel() generated
	// ("cv::ml::RTrees::to_StatModel", vec![(pred!(mut, [], []), _)]),
	cv::ml::StatModel* cv_ml_RTrees_to_StatModel(cv::ml::RTrees* instance) {
			return dynamic_cast<cv::ml::StatModel*>(instance);
	}

	// cv::ml::RTrees::delete() generated
	// ("cv::ml::RTrees::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ml_RTrees_delete(cv::ml::RTrees* instance) {
			delete instance;
	}

	// getType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:540
	// ("cv::ml::SVM::getType", vec![(pred!(const, [], []), _)]),
	void cv_ml_SVM_getType_const(const cv::ml::SVM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:542
	// ("cv::ml::SVM::setType", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_ml_SVM_setType_int(cv::ml::SVM* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setType(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getGamma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:547
	// ("cv::ml::SVM::getGamma", vec![(pred!(const, [], []), _)]),
	void cv_ml_SVM_getGamma_const(const cv::ml::SVM* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getGamma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setGamma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:549
	// ("cv::ml::SVM::setGamma", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ml_SVM_setGamma_double(cv::ml::SVM* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setGamma(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCoef0()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:554
	// ("cv::ml::SVM::getCoef0", vec![(pred!(const, [], []), _)]),
	void cv_ml_SVM_getCoef0_const(const cv::ml::SVM* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getCoef0();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCoef0(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:556
	// ("cv::ml::SVM::setCoef0", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ml_SVM_setCoef0_double(cv::ml::SVM* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setCoef0(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDegree()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:561
	// ("cv::ml::SVM::getDegree", vec![(pred!(const, [], []), _)]),
	void cv_ml_SVM_getDegree_const(const cv::ml::SVM* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDegree();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDegree(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:563
	// ("cv::ml::SVM::setDegree", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ml_SVM_setDegree_double(cv::ml::SVM* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setDegree(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getC()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:568
	// ("cv::ml::SVM::getC", vec![(pred!(const, [], []), _)]),
	void cv_ml_SVM_getC_const(const cv::ml::SVM* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getC();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setC(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:570
	// ("cv::ml::SVM::setC", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ml_SVM_setC_double(cv::ml::SVM* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setC(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNu()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:575
	// ("cv::ml::SVM::getNu", vec![(pred!(const, [], []), _)]),
	void cv_ml_SVM_getNu_const(const cv::ml::SVM* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getNu();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNu(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:577
	// ("cv::ml::SVM::setNu", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ml_SVM_setNu_double(cv::ml::SVM* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setNu(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getP()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:582
	// ("cv::ml::SVM::getP", vec![(pred!(const, [], []), _)]),
	void cv_ml_SVM_getP_const(const cv::ml::SVM* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getP();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setP(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:584
	// ("cv::ml::SVM::setP", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_ml_SVM_setP_double(cv::ml::SVM* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setP(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getClassWeights()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:592
	// ("cv::ml::SVM::getClassWeights", vec![(pred!(const, [], []), _)]),
	void cv_ml_SVM_getClassWeights_const(const cv::ml::SVM* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getClassWeights();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setClassWeights(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:594
	// ("cv::ml::SVM::setClassWeights", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
	void cv_ml_SVM_setClassWeights_const_MatR(cv::ml::SVM* instance, const cv::Mat* val, ResultVoid* ocvrs_return) {
		try {
			instance->setClassWeights(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTermCriteria()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:601
	// ("cv::ml::SVM::getTermCriteria", vec![(pred!(const, [], []), _)]),
	void cv_ml_SVM_getTermCriteria_const(const cv::ml::SVM* instance, Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTermCriteria(const cv::TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:603
	// ("cv::ml::SVM::setTermCriteria", vec![(pred!(mut, ["val"], ["const cv::TermCriteria*"]), _)]),
	void cv_ml_SVM_setTermCriteria_const_TermCriteriaR(cv::ml::SVM* instance, const cv::TermCriteria* val, ResultVoid* ocvrs_return) {
		try {
			instance->setTermCriteria(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getKernelType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:607
	// ("cv::ml::SVM::getKernelType", vec![(pred!(const, [], []), _)]),
	void cv_ml_SVM_getKernelType_const(const cv::ml::SVM* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getKernelType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setKernel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:611
	// ("cv::ml::SVM::setKernel", vec![(pred!(mut, ["kernelType"], ["int"]), _)]),
	void cv_ml_SVM_setKernel_int(cv::ml::SVM* instance, int kernelType, ResultVoid* ocvrs_return) {
		try {
			instance->setKernel(kernelType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setCustomKernel(const Ptr<Kernel> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:615
	// ("cv::ml::SVM::setCustomKernel", vec![(pred!(mut, ["_kernel"], ["const cv::Ptr<cv::ml::SVM::Kernel>*"]), _)]),
	void cv_ml_SVM_setCustomKernel_const_PtrLKernelGR(cv::ml::SVM* instance, const cv::Ptr<cv::ml::SVM::Kernel>* _kernel, ResultVoid* ocvrs_return) {
		try {
			instance->setCustomKernel(*_kernel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// trainAuto(const Ptr<TrainData> &, int, ParamGrid, ParamGrid, ParamGrid, ParamGrid, ParamGrid, ParamGrid, bool)(CppPassByVoidPtr, Primitive, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:712
	// ("cv::ml::SVM::trainAuto", vec![(pred!(mut, ["data", "kFold", "Cgrid", "gammaGrid", "pGrid", "nuGrid", "coeffGrid", "degreeGrid", "balanced"], ["const cv::Ptr<cv::ml::TrainData>*", "int", "cv::ml::ParamGrid", "cv::ml::ParamGrid", "cv::ml::ParamGrid", "cv::ml::ParamGrid", "cv::ml::ParamGrid", "cv::ml::ParamGrid", "bool"]), _)]),
	void cv_ml_SVM_trainAuto_const_PtrLTrainDataGR_int_ParamGrid_ParamGrid_ParamGrid_ParamGrid_ParamGrid_ParamGrid_bool(cv::ml::SVM* instance, const cv::Ptr<cv::ml::TrainData>* data, int kFold, cv::ml::ParamGrid* Cgrid, cv::ml::ParamGrid* gammaGrid, cv::ml::ParamGrid* pGrid, cv::ml::ParamGrid* nuGrid, cv::ml::ParamGrid* coeffGrid, cv::ml::ParamGrid* degreeGrid, bool balanced, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->trainAuto(*data, kFold, *Cgrid, *gammaGrid, *pGrid, *nuGrid, *coeffGrid, *degreeGrid, balanced);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::SVM::trainAuto(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:712
	// ("cv::ml::SVM::trainAuto", vec![(pred!(mut, ["data"], ["const cv::Ptr<cv::ml::TrainData>*"]), _)]),
	void cv_ml_SVM_trainAuto_const_PtrLTrainDataGR(cv::ml::SVM* instance, const cv::Ptr<cv::ml::TrainData>* data, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->trainAuto(*data);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// trainAuto(InputArray, int, InputArray, int, Ptr<ParamGrid>, Ptr<ParamGrid>, Ptr<ParamGrid>, Ptr<ParamGrid>, Ptr<ParamGrid>, Ptr<ParamGrid>, bool)(InputArray, Primitive, InputArray, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:749
	// ("cv::ml::SVM::trainAuto", vec![(pred!(mut, ["samples", "layout", "responses", "kFold", "Cgrid", "gammaGrid", "pGrid", "nuGrid", "coeffGrid", "degreeGrid", "balanced"], ["const cv::_InputArray*", "int", "const cv::_InputArray*", "int", "cv::Ptr<cv::ml::ParamGrid>", "cv::Ptr<cv::ml::ParamGrid>", "cv::Ptr<cv::ml::ParamGrid>", "cv::Ptr<cv::ml::ParamGrid>", "cv::Ptr<cv::ml::ParamGrid>", "cv::Ptr<cv::ml::ParamGrid>", "bool"]), _)]),
	void cv_ml_SVM_trainAuto_const__InputArrayR_int_const__InputArrayR_int_PtrLParamGridG_PtrLParamGridG_PtrLParamGridG_PtrLParamGridG_PtrLParamGridG_PtrLParamGridG_bool(cv::ml::SVM* instance, const cv::_InputArray* samples, int layout, const cv::_InputArray* responses, int kFold, cv::Ptr<cv::ml::ParamGrid>* Cgrid, cv::Ptr<cv::ml::ParamGrid>* gammaGrid, cv::Ptr<cv::ml::ParamGrid>* pGrid, cv::Ptr<cv::ml::ParamGrid>* nuGrid, cv::Ptr<cv::ml::ParamGrid>* coeffGrid, cv::Ptr<cv::ml::ParamGrid>* degreeGrid, bool balanced, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->trainAuto(*samples, layout, *responses, kFold, *Cgrid, *gammaGrid, *pGrid, *nuGrid, *coeffGrid, *degreeGrid, balanced);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::SVM::trainAuto(InputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:749
	// ("cv::ml::SVM::trainAuto", vec![(pred!(mut, ["samples", "layout", "responses"], ["const cv::_InputArray*", "int", "const cv::_InputArray*"]), _)]),
	void cv_ml_SVM_trainAuto_const__InputArrayR_int_const__InputArrayR(cv::ml::SVM* instance, const cv::_InputArray* samples, int layout, const cv::_InputArray* responses, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->trainAuto(*samples, layout, *responses);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSupportVectors()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:766
	// ("cv::ml::SVM::getSupportVectors", vec![(pred!(const, [], []), _)]),
	void cv_ml_SVM_getSupportVectors_const(const cv::ml::SVM* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getSupportVectors();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUncompressedSupportVectors()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:774
	// ("cv::ml::SVM::getUncompressedSupportVectors", vec![(pred!(const, [], []), _)]),
	void cv_ml_SVM_getUncompressedSupportVectors_const(const cv::ml::SVM* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getUncompressedSupportVectors();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDecisionFunction(int, OutputArray, OutputArray)(Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:791
	// ("cv::ml::SVM::getDecisionFunction", vec![(pred!(const, ["i", "alpha", "svidx"], ["int", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_ml_SVM_getDecisionFunction_const_int_const__OutputArrayR_const__OutputArrayR(const cv::ml::SVM* instance, int i, const cv::_OutputArray* alpha, const cv::_OutputArray* svidx, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDecisionFunction(i, *alpha, *svidx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultGrid(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:801
	// ("cv::ml::SVM::getDefaultGrid", vec![(pred!(mut, ["param_id"], ["int"]), _)]),
	void cv_ml_SVM_getDefaultGrid_int(int param_id, Result<cv::ml::ParamGrid*>* ocvrs_return) {
		try {
			cv::ml::ParamGrid ret = cv::ml::SVM::getDefaultGrid(param_id);
			Ok(new cv::ml::ParamGrid(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultGridPtr(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:811
	// ("cv::ml::SVM::getDefaultGridPtr", vec![(pred!(mut, ["param_id"], ["int"]), _)]),
	void cv_ml_SVM_getDefaultGridPtr_int(int param_id, Result<cv::Ptr<cv::ml::ParamGrid>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::ParamGrid> ret = cv::ml::SVM::getDefaultGridPtr(param_id);
			Ok(new cv::Ptr<cv::ml::ParamGrid>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:816
	// ("cv::ml::SVM::create", vec![(pred!(mut, [], []), _)]),
	void cv_ml_SVM_create(Result<cv::Ptr<cv::ml::SVM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::SVM> ret = cv::ml::SVM::create();
			Ok(new cv::Ptr<cv::ml::SVM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:825
	// ("cv::ml::SVM::load", vec![(pred!(mut, ["filepath"], ["const cv::String*"]), _)]),
	void cv_ml_SVM_load_const_StringR(const char* filepath, Result<cv::Ptr<cv::ml::SVM>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::SVM> ret = cv::ml::SVM::load(cv::String(filepath));
			Ok(new cv::Ptr<cv::ml::SVM>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::SVM::to_Algorithm() generated
	// ("cv::ml::SVM::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ml_SVM_to_Algorithm(cv::ml::SVM* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ml::SVM::to_StatModel() generated
	// ("cv::ml::SVM::to_StatModel", vec![(pred!(mut, [], []), _)]),
	cv::ml::StatModel* cv_ml_SVM_to_StatModel(cv::ml::SVM* instance) {
			return dynamic_cast<cv::ml::StatModel*>(instance);
	}

	// cv::ml::SVM::delete() generated
	// ("cv::ml::SVM::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ml_SVM_delete(cv::ml::SVM* instance) {
			delete instance;
	}

	// getType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:533
	// ("cv::ml::SVM::Kernel::getType", vec![(pred!(const, [], []), _)]),
	void cv_ml_SVM_Kernel_getType_const(const cv::ml::SVM::Kernel* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calc(int, int, const float *, const float *, float *)(Primitive, Primitive, Indirect, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:534
	// ("cv::ml::SVM::Kernel::calc", vec![(pred!(mut, ["vcount", "n", "vecs", "another", "results"], ["int", "int", "const float*", "const float*", "float*"]), _)]),
	void cv_ml_SVM_Kernel_calc_int_int_const_floatX_const_floatX_floatX(cv::ml::SVM::Kernel* instance, int vcount, int n, const float* vecs, const float* another, float* results, ResultVoid* ocvrs_return) {
		try {
			instance->calc(vcount, n, vecs, another, results);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::SVM::Kernel::to_Algorithm() generated
	// ("cv::ml::SVM::Kernel::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ml_SVM_Kernel_to_Algorithm(cv::ml::SVM::Kernel* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ml::SVM::Kernel::delete() generated
	// ("cv::ml::SVM::Kernel::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ml_SVM_Kernel_delete(cv::ml::SVM::Kernel* instance) {
			delete instance;
	}

	// getWeights()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1814
	// ("cv::ml::SVMSGD::getWeights", vec![(pred!(mut, [], []), _)]),
	void cv_ml_SVMSGD_getWeights(cv::ml::SVMSGD* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getWeights();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getShift()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1819
	// ("cv::ml::SVMSGD::getShift", vec![(pred!(mut, [], []), _)]),
	void cv_ml_SVMSGD_getShift(cv::ml::SVMSGD* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getShift();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1825
	// ("cv::ml::SVMSGD::create", vec![(pred!(mut, [], []), _)]),
	void cv_ml_SVMSGD_create(Result<cv::Ptr<cv::ml::SVMSGD>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::SVMSGD> ret = cv::ml::SVMSGD::create();
			Ok(new cv::Ptr<cv::ml::SVMSGD>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// load(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1836
	// ("cv::ml::SVMSGD::load", vec![(pred!(mut, ["filepath", "nodeName"], ["const cv::String*", "const cv::String*"]), _)]),
	void cv_ml_SVMSGD_load_const_StringR_const_StringR(const char* filepath, const char* nodeName, Result<cv::Ptr<cv::ml::SVMSGD>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::SVMSGD> ret = cv::ml::SVMSGD::load(cv::String(filepath), cv::String(nodeName));
			Ok(new cv::Ptr<cv::ml::SVMSGD>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::SVMSGD::load(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1836
	// ("cv::ml::SVMSGD::load", vec![(pred!(mut, ["filepath"], ["const cv::String*"]), _)]),
	void cv_ml_SVMSGD_load_const_StringR(const char* filepath, Result<cv::Ptr<cv::ml::SVMSGD>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::SVMSGD> ret = cv::ml::SVMSGD::load(cv::String(filepath));
			Ok(new cv::Ptr<cv::ml::SVMSGD>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOptimalParameters(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1842
	// ("cv::ml::SVMSGD::setOptimalParameters", vec![(pred!(mut, ["svmsgdType", "marginType"], ["int", "int"]), _)]),
	void cv_ml_SVMSGD_setOptimalParameters_int_int(cv::ml::SVMSGD* instance, int svmsgdType, int marginType, ResultVoid* ocvrs_return) {
		try {
			instance->setOptimalParameters(svmsgdType, marginType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::SVMSGD::setOptimalParameters() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1842
	// ("cv::ml::SVMSGD::setOptimalParameters", vec![(pred!(mut, [], []), _)]),
	void cv_ml_SVMSGD_setOptimalParameters(cv::ml::SVMSGD* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setOptimalParameters();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSvmsgdType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1846
	// ("cv::ml::SVMSGD::getSvmsgdType", vec![(pred!(const, [], []), _)]),
	void cv_ml_SVMSGD_getSvmsgdType_const(const cv::ml::SVMSGD* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSvmsgdType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSvmsgdType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1848
	// ("cv::ml::SVMSGD::setSvmsgdType", vec![(pred!(mut, ["svmsgdType"], ["int"]), _)]),
	void cv_ml_SVMSGD_setSvmsgdType_int(cv::ml::SVMSGD* instance, int svmsgdType, ResultVoid* ocvrs_return) {
		try {
			instance->setSvmsgdType(svmsgdType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMarginType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1852
	// ("cv::ml::SVMSGD::getMarginType", vec![(pred!(const, [], []), _)]),
	void cv_ml_SVMSGD_getMarginType_const(const cv::ml::SVMSGD* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMarginType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMarginType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1854
	// ("cv::ml::SVMSGD::setMarginType", vec![(pred!(mut, ["marginType"], ["int"]), _)]),
	void cv_ml_SVMSGD_setMarginType_int(cv::ml::SVMSGD* instance, int marginType, ResultVoid* ocvrs_return) {
		try {
			instance->setMarginType(marginType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMarginRegularization()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1858
	// ("cv::ml::SVMSGD::getMarginRegularization", vec![(pred!(const, [], []), _)]),
	void cv_ml_SVMSGD_getMarginRegularization_const(const cv::ml::SVMSGD* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMarginRegularization();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMarginRegularization(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1860
	// ("cv::ml::SVMSGD::setMarginRegularization", vec![(pred!(mut, ["marginRegularization"], ["float"]), _)]),
	void cv_ml_SVMSGD_setMarginRegularization_float(cv::ml::SVMSGD* instance, float marginRegularization, ResultVoid* ocvrs_return) {
		try {
			instance->setMarginRegularization(marginRegularization);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInitialStepSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1864
	// ("cv::ml::SVMSGD::getInitialStepSize", vec![(pred!(const, [], []), _)]),
	void cv_ml_SVMSGD_getInitialStepSize_const(const cv::ml::SVMSGD* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getInitialStepSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInitialStepSize(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1866
	// ("cv::ml::SVMSGD::setInitialStepSize", vec![(pred!(mut, ["InitialStepSize"], ["float"]), _)]),
	void cv_ml_SVMSGD_setInitialStepSize_float(cv::ml::SVMSGD* instance, float InitialStepSize, ResultVoid* ocvrs_return) {
		try {
			instance->setInitialStepSize(InitialStepSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getStepDecreasingPower()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1870
	// ("cv::ml::SVMSGD::getStepDecreasingPower", vec![(pred!(const, [], []), _)]),
	void cv_ml_SVMSGD_getStepDecreasingPower_const(const cv::ml::SVMSGD* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getStepDecreasingPower();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setStepDecreasingPower(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1872
	// ("cv::ml::SVMSGD::setStepDecreasingPower", vec![(pred!(mut, ["stepDecreasingPower"], ["float"]), _)]),
	void cv_ml_SVMSGD_setStepDecreasingPower_float(cv::ml::SVMSGD* instance, float stepDecreasingPower, ResultVoid* ocvrs_return) {
		try {
			instance->setStepDecreasingPower(stepDecreasingPower);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTermCriteria()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1878
	// ("cv::ml::SVMSGD::getTermCriteria", vec![(pred!(const, [], []), _)]),
	void cv_ml_SVMSGD_getTermCriteria_const(const cv::ml::SVMSGD* instance, Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTermCriteria(const cv::TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1880
	// ("cv::ml::SVMSGD::setTermCriteria", vec![(pred!(mut, ["val"], ["const cv::TermCriteria*"]), _)]),
	void cv_ml_SVMSGD_setTermCriteria_const_TermCriteriaR(cv::ml::SVMSGD* instance, const cv::TermCriteria* val, ResultVoid* ocvrs_return) {
		try {
			instance->setTermCriteria(*val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::SVMSGD::to_Algorithm() generated
	// ("cv::ml::SVMSGD::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ml_SVMSGD_to_Algorithm(cv::ml::SVMSGD* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ml::SVMSGD::to_StatModel() generated
	// ("cv::ml::SVMSGD::to_StatModel", vec![(pred!(mut, [], []), _)]),
	cv::ml::StatModel* cv_ml_SVMSGD_to_StatModel(cv::ml::SVMSGD* instance) {
			return dynamic_cast<cv::ml::StatModel*>(instance);
	}

	// cv::ml::SVMSGD::delete() generated
	// ("cv::ml::SVMSGD::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ml_SVMSGD_delete(cv::ml::SVMSGD* instance) {
			delete instance;
	}

	// getVarCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:330
	// ("cv::ml::StatModel::getVarCount", vec![(pred!(const, [], []), _)]),
	void cv_ml_StatModel_getVarCount_const(const cv::ml::StatModel* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getVarCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:332
	// ("cv::ml::StatModel::empty", vec![(pred!(const, [], []), _)]),
	void cv_ml_StatModel_empty_const(const cv::ml::StatModel* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isTrained()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:335
	// ("cv::ml::StatModel::isTrained", vec![(pred!(const, [], []), _)]),
	void cv_ml_StatModel_isTrained_const(const cv::ml::StatModel* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isTrained();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// isClassifier()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:337
	// ("cv::ml::StatModel::isClassifier", vec![(pred!(const, [], []), _)]),
	void cv_ml_StatModel_isClassifier_const(const cv::ml::StatModel* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isClassifier();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// train(const Ptr<TrainData> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:346
	// ("cv::ml::StatModel::train", vec![(pred!(mut, ["trainData", "flags"], ["const cv::Ptr<cv::ml::TrainData>*", "int"]), _)]),
	void cv_ml_StatModel_train_const_PtrLTrainDataGR_int(cv::ml::StatModel* instance, const cv::Ptr<cv::ml::TrainData>* trainData, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->train(*trainData, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::StatModel::train(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:346
	// ("cv::ml::StatModel::train", vec![(pred!(mut, ["trainData"], ["const cv::Ptr<cv::ml::TrainData>*"]), _)]),
	void cv_ml_StatModel_train_const_PtrLTrainDataGR(cv::ml::StatModel* instance, const cv::Ptr<cv::ml::TrainData>* trainData, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->train(*trainData);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// train(InputArray, int, InputArray)(InputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:354
	// ("cv::ml::StatModel::train", vec![(pred!(mut, ["samples", "layout", "responses"], ["const cv::_InputArray*", "int", "const cv::_InputArray*"]), _)]),
	void cv_ml_StatModel_train_const__InputArrayR_int_const__InputArrayR(cv::ml::StatModel* instance, const cv::_InputArray* samples, int layout, const cv::_InputArray* responses, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->train(*samples, layout, *responses);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// calcError(const Ptr<TrainData> &, bool, OutputArray)(CppPassByVoidPtr, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:369
	// ("cv::ml::StatModel::calcError", vec![(pred!(const, ["data", "test", "resp"], ["const cv::Ptr<cv::ml::TrainData>*", "bool", "const cv::_OutputArray*"]), _)]),
	void cv_ml_StatModel_calcError_const_const_PtrLTrainDataGR_bool_const__OutputArrayR(const cv::ml::StatModel* instance, const cv::Ptr<cv::ml::TrainData>* data, bool test, const cv::_OutputArray* resp, Result<float>* ocvrs_return) {
		try {
			float ret = instance->calcError(*data, test, *resp);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// predict(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:377
	// ("cv::ml::StatModel::predict", vec![(pred!(const, ["samples", "results", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
	void cv_ml_StatModel_predict_const_const__InputArrayR_const__OutputArrayR_int(const cv::ml::StatModel* instance, const cv::_InputArray* samples, const cv::_OutputArray* results, int flags, Result<float>* ocvrs_return) {
		try {
			float ret = instance->predict(*samples, *results, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::StatModel::predict(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:377
	// ("cv::ml::StatModel::predict", vec![(pred!(const, ["samples"], ["const cv::_InputArray*"]), _)]),
	void cv_ml_StatModel_predict_const_const__InputArrayR(const cv::ml::StatModel* instance, const cv::_InputArray* samples, Result<float>* ocvrs_return) {
		try {
			float ret = instance->predict(*samples);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::StatModel::to_ANN_MLP() generated
	// ("cv::ml::StatModel::to_ANN_MLP", vec![(pred!(mut, [], []), _)]),
	cv::ml::ANN_MLP* cv_ml_StatModel_to_ANN_MLP(cv::ml::StatModel* instance) {
			return dynamic_cast<cv::ml::ANN_MLP*>(instance);
	}

	// cv::ml::StatModel::to_ANN_MLP_ANNEAL() generated
	// ("cv::ml::StatModel::to_ANN_MLP_ANNEAL", vec![(pred!(mut, [], []), _)]),
	cv::ml::ANN_MLP_ANNEAL* cv_ml_StatModel_to_ANN_MLP_ANNEAL(cv::ml::StatModel* instance) {
			return dynamic_cast<cv::ml::ANN_MLP_ANNEAL*>(instance);
	}

	// cv::ml::StatModel::to_Boost() generated
	// ("cv::ml::StatModel::to_Boost", vec![(pred!(mut, [], []), _)]),
	cv::ml::Boost* cv_ml_StatModel_to_Boost(cv::ml::StatModel* instance) {
			return dynamic_cast<cv::ml::Boost*>(instance);
	}

	// cv::ml::StatModel::to_DTrees() generated
	// ("cv::ml::StatModel::to_DTrees", vec![(pred!(mut, [], []), _)]),
	cv::ml::DTrees* cv_ml_StatModel_to_DTrees(cv::ml::StatModel* instance) {
			return dynamic_cast<cv::ml::DTrees*>(instance);
	}

	// cv::ml::StatModel::to_EM() generated
	// ("cv::ml::StatModel::to_EM", vec![(pred!(mut, [], []), _)]),
	cv::ml::EM* cv_ml_StatModel_to_EM(cv::ml::StatModel* instance) {
			return dynamic_cast<cv::ml::EM*>(instance);
	}

	// cv::ml::StatModel::to_KNearest() generated
	// ("cv::ml::StatModel::to_KNearest", vec![(pred!(mut, [], []), _)]),
	cv::ml::KNearest* cv_ml_StatModel_to_KNearest(cv::ml::StatModel* instance) {
			return dynamic_cast<cv::ml::KNearest*>(instance);
	}

	// cv::ml::StatModel::to_LogisticRegression() generated
	// ("cv::ml::StatModel::to_LogisticRegression", vec![(pred!(mut, [], []), _)]),
	cv::ml::LogisticRegression* cv_ml_StatModel_to_LogisticRegression(cv::ml::StatModel* instance) {
			return dynamic_cast<cv::ml::LogisticRegression*>(instance);
	}

	// cv::ml::StatModel::to_NormalBayesClassifier() generated
	// ("cv::ml::StatModel::to_NormalBayesClassifier", vec![(pred!(mut, [], []), _)]),
	cv::ml::NormalBayesClassifier* cv_ml_StatModel_to_NormalBayesClassifier(cv::ml::StatModel* instance) {
			return dynamic_cast<cv::ml::NormalBayesClassifier*>(instance);
	}

	// cv::ml::StatModel::to_RTrees() generated
	// ("cv::ml::StatModel::to_RTrees", vec![(pred!(mut, [], []), _)]),
	cv::ml::RTrees* cv_ml_StatModel_to_RTrees(cv::ml::StatModel* instance) {
			return dynamic_cast<cv::ml::RTrees*>(instance);
	}

	// cv::ml::StatModel::to_SVM() generated
	// ("cv::ml::StatModel::to_SVM", vec![(pred!(mut, [], []), _)]),
	cv::ml::SVM* cv_ml_StatModel_to_SVM(cv::ml::StatModel* instance) {
			return dynamic_cast<cv::ml::SVM*>(instance);
	}

	// cv::ml::StatModel::to_SVMSGD() generated
	// ("cv::ml::StatModel::to_SVMSGD", vec![(pred!(mut, [], []), _)]),
	cv::ml::SVMSGD* cv_ml_StatModel_to_SVMSGD(cv::ml::StatModel* instance) {
			return dynamic_cast<cv::ml::SVMSGD*>(instance);
	}

	// cv::ml::StatModel::to_Algorithm() generated
	// ("cv::ml::StatModel::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_ml_StatModel_to_Algorithm(cv::ml::StatModel* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::ml::StatModel::delete() generated
	// ("cv::ml::StatModel::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ml_StatModel_delete(cv::ml::StatModel* instance) {
			delete instance;
	}

	// missingValue()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:148
	// ("cv::ml::TrainData::missingValue", vec![(pred!(mut, [], []), _)]),
	void cv_ml_TrainData_missingValue(Result<float>* ocvrs_return) {
		try {
			float ret = cv::ml::TrainData::missingValue();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getLayout()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:151
	// ("cv::ml::TrainData::getLayout", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getLayout_const(const cv::ml::TrainData* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getLayout();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNTrainSamples()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:152
	// ("cv::ml::TrainData::getNTrainSamples", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getNTrainSamples_const(const cv::ml::TrainData* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNTrainSamples();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNTestSamples()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:153
	// ("cv::ml::TrainData::getNTestSamples", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getNTestSamples_const(const cv::ml::TrainData* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNTestSamples();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNSamples()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:154
	// ("cv::ml::TrainData::getNSamples", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getNSamples_const(const cv::ml::TrainData* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNSamples();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNVars()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:155
	// ("cv::ml::TrainData::getNVars", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getNVars_const(const cv::ml::TrainData* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNVars();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNAllVars()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:156
	// ("cv::ml::TrainData::getNAllVars", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getNAllVars_const(const cv::ml::TrainData* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNAllVars();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSample(InputArray, int, float *)(InputArray, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:158
	// ("cv::ml::TrainData::getSample", vec![(pred!(const, ["varIdx", "sidx", "buf"], ["const cv::_InputArray*", "int", "float*"]), _)]),
	void cv_ml_TrainData_getSample_const_const__InputArrayR_int_floatX(const cv::ml::TrainData* instance, const cv::_InputArray* varIdx, int sidx, float* buf, ResultVoid* ocvrs_return) {
		try {
			instance->getSample(*varIdx, sidx, buf);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSamples()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:159
	// ("cv::ml::TrainData::getSamples", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getSamples_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getSamples();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMissing()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:160
	// ("cv::ml::TrainData::getMissing", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getMissing_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMissing();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTrainSamples(int, bool, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:174
	// ("cv::ml::TrainData::getTrainSamples", vec![(pred!(const, ["layout", "compressSamples", "compressVars"], ["int", "bool", "bool"]), _)]),
	void cv_ml_TrainData_getTrainSamples_const_int_bool_bool(const cv::ml::TrainData* instance, int layout, bool compressSamples, bool compressVars, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getTrainSamples(layout, compressSamples, compressVars);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::TrainData::getTrainSamples() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:174
	// ("cv::ml::TrainData::getTrainSamples", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getTrainSamples_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getTrainSamples();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTrainResponses()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:183
	// ("cv::ml::TrainData::getTrainResponses", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getTrainResponses_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getTrainResponses();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTrainNormCatResponses()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:191
	// ("cv::ml::TrainData::getTrainNormCatResponses", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getTrainNormCatResponses_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getTrainNormCatResponses();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTestResponses()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:192
	// ("cv::ml::TrainData::getTestResponses", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getTestResponses_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getTestResponses();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTestNormCatResponses()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:193
	// ("cv::ml::TrainData::getTestNormCatResponses", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getTestNormCatResponses_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getTestNormCatResponses();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getResponses()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:194
	// ("cv::ml::TrainData::getResponses", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getResponses_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getResponses();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormCatResponses()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:195
	// ("cv::ml::TrainData::getNormCatResponses", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getNormCatResponses_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getNormCatResponses();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSampleWeights()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:196
	// ("cv::ml::TrainData::getSampleWeights", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getSampleWeights_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getSampleWeights();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTrainSampleWeights()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:197
	// ("cv::ml::TrainData::getTrainSampleWeights", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getTrainSampleWeights_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getTrainSampleWeights();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTestSampleWeights()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:198
	// ("cv::ml::TrainData::getTestSampleWeights", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getTestSampleWeights_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getTestSampleWeights();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVarIdx()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:199
	// ("cv::ml::TrainData::getVarIdx", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getVarIdx_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getVarIdx();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVarType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:200
	// ("cv::ml::TrainData::getVarType", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getVarType_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getVarType();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getVarSymbolFlags()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:201
	// ("cv::ml::TrainData::getVarSymbolFlags", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getVarSymbolFlags_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getVarSymbolFlags();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getResponseType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:202
	// ("cv::ml::TrainData::getResponseType", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getResponseType_const(const cv::ml::TrainData* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getResponseType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTrainSampleIdx()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:203
	// ("cv::ml::TrainData::getTrainSampleIdx", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getTrainSampleIdx_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getTrainSampleIdx();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTestSampleIdx()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:204
	// ("cv::ml::TrainData::getTestSampleIdx", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getTestSampleIdx_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getTestSampleIdx();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getValues(int, InputArray, float *)(Primitive, InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:205
	// ("cv::ml::TrainData::getValues", vec![(pred!(const, ["vi", "sidx", "values"], ["int", "const cv::_InputArray*", "float*"]), _)]),
	void cv_ml_TrainData_getValues_const_int_const__InputArrayR_floatX(const cv::ml::TrainData* instance, int vi, const cv::_InputArray* sidx, float* values, ResultVoid* ocvrs_return) {
		try {
			instance->getValues(vi, *sidx, values);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNormCatValues(int, InputArray, int *)(Primitive, InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:206
	// ("cv::ml::TrainData::getNormCatValues", vec![(pred!(const, ["vi", "sidx", "values"], ["int", "const cv::_InputArray*", "int*"]), _)]),
	void cv_ml_TrainData_getNormCatValues_const_int_const__InputArrayR_intX(const cv::ml::TrainData* instance, int vi, const cv::_InputArray* sidx, int* values, ResultVoid* ocvrs_return) {
		try {
			instance->getNormCatValues(vi, *sidx, values);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultSubstValues()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:207
	// ("cv::ml::TrainData::getDefaultSubstValues", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getDefaultSubstValues_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getDefaultSubstValues();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCatCount(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:209
	// ("cv::ml::TrainData::getCatCount", vec![(pred!(const, ["vi"], ["int"]), _)]),
	void cv_ml_TrainData_getCatCount_const_int(const cv::ml::TrainData* instance, int vi, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getCatCount(vi);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getClassLabels()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:215
	// ("cv::ml::TrainData::getClassLabels", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getClassLabels_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getClassLabels();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCatOfs()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:217
	// ("cv::ml::TrainData::getCatOfs", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getCatOfs_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getCatOfs();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getCatMap()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:218
	// ("cv::ml::TrainData::getCatMap", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getCatMap_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getCatMap();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTrainTestSplit(int, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:223
	// ("cv::ml::TrainData::setTrainTestSplit", vec![(pred!(mut, ["count", "shuffle"], ["int", "bool"]), _)]),
	void cv_ml_TrainData_setTrainTestSplit_int_bool(cv::ml::TrainData* instance, int count, bool shuffle, ResultVoid* ocvrs_return) {
		try {
			instance->setTrainTestSplit(count, shuffle);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::TrainData::setTrainTestSplit(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:223
	// ("cv::ml::TrainData::setTrainTestSplit", vec![(pred!(mut, ["count"], ["int"]), _)]),
	void cv_ml_TrainData_setTrainTestSplit_int(cv::ml::TrainData* instance, int count, ResultVoid* ocvrs_return) {
		try {
			instance->setTrainTestSplit(count);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTrainTestSplitRatio(double, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:233
	// ("cv::ml::TrainData::setTrainTestSplitRatio", vec![(pred!(mut, ["ratio", "shuffle"], ["double", "bool"]), _)]),
	void cv_ml_TrainData_setTrainTestSplitRatio_double_bool(cv::ml::TrainData* instance, double ratio, bool shuffle, ResultVoid* ocvrs_return) {
		try {
			instance->setTrainTestSplitRatio(ratio, shuffle);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::TrainData::setTrainTestSplitRatio(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:233
	// ("cv::ml::TrainData::setTrainTestSplitRatio", vec![(pred!(mut, ["ratio"], ["double"]), _)]),
	void cv_ml_TrainData_setTrainTestSplitRatio_double(cv::ml::TrainData* instance, double ratio, ResultVoid* ocvrs_return) {
		try {
			instance->setTrainTestSplitRatio(ratio);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// shuffleTrainTest()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:234
	// ("cv::ml::TrainData::shuffleTrainTest", vec![(pred!(mut, [], []), _)]),
	void cv_ml_TrainData_shuffleTrainTest(cv::ml::TrainData* instance, ResultVoid* ocvrs_return) {
		try {
			instance->shuffleTrainTest();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getTestSamples()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:237
	// ("cv::ml::TrainData::getTestSamples", vec![(pred!(const, [], []), _)]),
	void cv_ml_TrainData_getTestSamples_const(const cv::ml::TrainData* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getTestSamples();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNames(std::vector<String> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:240
	// ("cv::ml::TrainData::getNames", vec![(pred!(const, ["names"], ["std::vector<cv::String>*"]), _)]),
	void cv_ml_TrainData_getNames_const_vectorLStringGR(const cv::ml::TrainData* instance, std::vector<cv::String>* names, ResultVoid* ocvrs_return) {
		try {
			instance->getNames(*names);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSubVector(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:246
	// ("cv::ml::TrainData::getSubVector", vec![(pred!(mut, ["vec", "idx"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	void cv_ml_TrainData_getSubVector_const_MatR_const_MatR(const cv::Mat* vec, const cv::Mat* idx, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::ml::TrainData::getSubVector(*vec, *idx);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSubMatrix(const Mat &, const Mat &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:253
	// ("cv::ml::TrainData::getSubMatrix", vec![(pred!(mut, ["matrix", "idx", "layout"], ["const cv::Mat*", "const cv::Mat*", "int"]), _)]),
	void cv_ml_TrainData_getSubMatrix_const_MatR_const_MatR_int(const cv::Mat* matrix, const cv::Mat* idx, int layout, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::ml::TrainData::getSubMatrix(*matrix, *idx, layout);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// loadFromCSV(const String &, int, int, int, const String &, char, char)(InString, Primitive, Primitive, Primitive, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:284
	// ("cv::ml::TrainData::loadFromCSV", vec![(pred!(mut, ["filename", "headerLineCount", "responseStartIdx", "responseEndIdx", "varTypeSpec", "delimiter", "missch"], ["const cv::String*", "int", "int", "int", "const cv::String*", "char", "char"]), _)]),
	void cv_ml_TrainData_loadFromCSV_const_StringR_int_int_int_const_StringR_char_char(const char* filename, int headerLineCount, int responseStartIdx, int responseEndIdx, const char* varTypeSpec, char delimiter, char missch, Result<cv::Ptr<cv::ml::TrainData>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::TrainData> ret = cv::ml::TrainData::loadFromCSV(cv::String(filename), headerLineCount, responseStartIdx, responseEndIdx, cv::String(varTypeSpec), delimiter, missch);
			Ok(new cv::Ptr<cv::ml::TrainData>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::TrainData::loadFromCSV(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:284
	// ("cv::ml::TrainData::loadFromCSV", vec![(pred!(mut, ["filename", "headerLineCount"], ["const cv::String*", "int"]), _)]),
	void cv_ml_TrainData_loadFromCSV_const_StringR_int(const char* filename, int headerLineCount, Result<cv::Ptr<cv::ml::TrainData>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::TrainData> ret = cv::ml::TrainData::loadFromCSV(cv::String(filename), headerLineCount);
			Ok(new cv::Ptr<cv::ml::TrainData>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(InputArray, int, InputArray, InputArray, InputArray, InputArray, InputArray)(InputArray, Primitive, InputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:311
	// ("cv::ml::TrainData::create", vec![(pred!(mut, ["samples", "layout", "responses", "varIdx", "sampleIdx", "sampleWeights", "varType"], ["const cv::_InputArray*", "int", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_ml_TrainData_create_const__InputArrayR_int_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* samples, int layout, const cv::_InputArray* responses, const cv::_InputArray* varIdx, const cv::_InputArray* sampleIdx, const cv::_InputArray* sampleWeights, const cv::_InputArray* varType, Result<cv::Ptr<cv::ml::TrainData>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::TrainData> ret = cv::ml::TrainData::create(*samples, layout, *responses, *varIdx, *sampleIdx, *sampleWeights, *varType);
			Ok(new cv::Ptr<cv::ml::TrainData>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::TrainData::create(InputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:311
	// ("cv::ml::TrainData::create", vec![(pred!(mut, ["samples", "layout", "responses"], ["const cv::_InputArray*", "int", "const cv::_InputArray*"]), _)]),
	void cv_ml_TrainData_create_const__InputArrayR_int_const__InputArrayR(const cv::_InputArray* samples, int layout, const cv::_InputArray* responses, Result<cv::Ptr<cv::ml::TrainData>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ml::TrainData> ret = cv::ml::TrainData::create(*samples, layout, *responses);
			Ok(new cv::Ptr<cv::ml::TrainData>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::ml::TrainData::delete() generated
	// ("cv::ml::TrainData::delete", vec![(pred!(mut, [], []), _)]),
	void cv_ml_TrainData_delete(cv::ml::TrainData* instance) {
			delete instance;
	}

}
