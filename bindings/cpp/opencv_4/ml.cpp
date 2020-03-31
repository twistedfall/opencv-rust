#include "common.hpp"
#include <opencv2/ml.hpp>
#include "ml_types.hpp"

extern "C" {
	Result_void cv_ml_createConcentricSpheresTestSet_int_int_int_const__OutputArrayX_const__OutputArrayX(int nsamples, int nfeatures, int nclasses, const cv::_OutputArray* samples, const cv::_OutputArray* responses) {
		try {
			cv::ml::createConcentricSpheresTestSet(nsamples, nfeatures, nclasses, *samples, *responses);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ml_randMVNormal_const__InputArrayX_const__InputArrayX_int_const__OutputArrayX(const cv::_InputArray* mean, const cv::_InputArray* cov, int nsamples, const cv::_OutputArray* samples) {
		try {
			cv::ml::randMVNormal(*mean, *cov, nsamples, *samples);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ml_ANN_MLP_setTrainMethod_int_double_double(cv::ml::ANN_MLP* instance, int method, double param1, double param2) {
		try {
			instance->setTrainMethod(method, param1, param2);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_ANN_MLP_getTrainMethod_const(const cv::ml::ANN_MLP* instance) {
		try {
			int ret = instance->getTrainMethod();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_ANN_MLP_setActivationFunction_int_double_double(cv::ml::ANN_MLP* instance, int type, double param1, double param2) {
		try {
			instance->setActivationFunction(type, param1, param2);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ml_ANN_MLP_setLayerSizes_const__InputArrayX(cv::ml::ANN_MLP* instance, const cv::_InputArray* _layer_sizes) {
		try {
			instance->setLayerSizes(*_layer_sizes);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_ml_ANN_MLP_getLayerSizes_const(const cv::ml::ANN_MLP* instance) {
		try {
			cv::Mat ret = instance->getLayerSizes();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::TermCriteria> cv_ml_ANN_MLP_getTermCriteria_const(const cv::ml::ANN_MLP* instance) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::TermCriteria>)
	}
	
	Result_void cv_ml_ANN_MLP_setTermCriteria_TermCriteria(cv::ml::ANN_MLP* instance, const cv::TermCriteria* val) {
		try {
			instance->setTermCriteria(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ANN_MLP_getBackpropWeightScale_const(const cv::ml::ANN_MLP* instance) {
		try {
			double ret = instance->getBackpropWeightScale();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ANN_MLP_setBackpropWeightScale_double(cv::ml::ANN_MLP* instance, double val) {
		try {
			instance->setBackpropWeightScale(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ANN_MLP_getBackpropMomentumScale_const(const cv::ml::ANN_MLP* instance) {
		try {
			double ret = instance->getBackpropMomentumScale();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ANN_MLP_setBackpropMomentumScale_double(cv::ml::ANN_MLP* instance, double val) {
		try {
			instance->setBackpropMomentumScale(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ANN_MLP_getRpropDW0_const(const cv::ml::ANN_MLP* instance) {
		try {
			double ret = instance->getRpropDW0();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ANN_MLP_setRpropDW0_double(cv::ml::ANN_MLP* instance, double val) {
		try {
			instance->setRpropDW0(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ANN_MLP_getRpropDWPlus_const(const cv::ml::ANN_MLP* instance) {
		try {
			double ret = instance->getRpropDWPlus();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ANN_MLP_setRpropDWPlus_double(cv::ml::ANN_MLP* instance, double val) {
		try {
			instance->setRpropDWPlus(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ANN_MLP_getRpropDWMinus_const(const cv::ml::ANN_MLP* instance) {
		try {
			double ret = instance->getRpropDWMinus();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ANN_MLP_setRpropDWMinus_double(cv::ml::ANN_MLP* instance, double val) {
		try {
			instance->setRpropDWMinus(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ANN_MLP_getRpropDWMin_const(const cv::ml::ANN_MLP* instance) {
		try {
			double ret = instance->getRpropDWMin();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ANN_MLP_setRpropDWMin_double(cv::ml::ANN_MLP* instance, double val) {
		try {
			instance->setRpropDWMin(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ANN_MLP_getRpropDWMax_const(const cv::ml::ANN_MLP* instance) {
		try {
			double ret = instance->getRpropDWMax();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ANN_MLP_setRpropDWMax_double(cv::ml::ANN_MLP* instance, double val) {
		try {
			instance->setRpropDWMax(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ANN_MLP_getAnnealInitialT_const(const cv::ml::ANN_MLP* instance) {
		try {
			double ret = instance->getAnnealInitialT();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ANN_MLP_setAnnealInitialT_double(cv::ml::ANN_MLP* instance, double val) {
		try {
			instance->setAnnealInitialT(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ANN_MLP_getAnnealFinalT_const(const cv::ml::ANN_MLP* instance) {
		try {
			double ret = instance->getAnnealFinalT();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ANN_MLP_setAnnealFinalT_double(cv::ml::ANN_MLP* instance, double val) {
		try {
			instance->setAnnealFinalT(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ANN_MLP_getAnnealCoolingRatio_const(const cv::ml::ANN_MLP* instance) {
		try {
			double ret = instance->getAnnealCoolingRatio();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ANN_MLP_setAnnealCoolingRatio_double(cv::ml::ANN_MLP* instance, double val) {
		try {
			instance->setAnnealCoolingRatio(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_ANN_MLP_getAnnealItePerStep_const(const cv::ml::ANN_MLP* instance) {
		try {
			int ret = instance->getAnnealItePerStep();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_ANN_MLP_setAnnealItePerStep_int(cv::ml::ANN_MLP* instance, int val) {
		try {
			instance->setAnnealItePerStep(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ml_ANN_MLP_setAnnealEnergyRNG_const_RNGX(cv::ml::ANN_MLP* instance, const cv::RNG* rng) {
		try {
			instance->setAnnealEnergyRNG(*rng);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_ml_ANN_MLP_getWeights_const_int(const cv::ml::ANN_MLP* instance, int layerIdx) {
		try {
			cv::Mat ret = instance->getWeights(layerIdx);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Ptr<cv::ml::ANN_MLP>*> cv_ml_ANN_MLP_create() {
		try {
			cv::Ptr<cv::ml::ANN_MLP> ret = cv::ml::ANN_MLP::create();
			return Ok(new cv::Ptr<cv::ml::ANN_MLP>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::ANN_MLP>*>)
	}
	
	Result<cv::Ptr<cv::ml::ANN_MLP>*> cv_ml_ANN_MLP_load_const_StringX(const char* filepath) {
		try {
			cv::Ptr<cv::ml::ANN_MLP> ret = cv::ml::ANN_MLP::load(std::string(filepath));
			return Ok(new cv::Ptr<cv::ml::ANN_MLP>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::ANN_MLP>*>)
	}
	
	Result<int> cv_ml_Boost_getBoostType_const(const cv::ml::Boost* instance) {
		try {
			int ret = instance->getBoostType();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_Boost_setBoostType_int(cv::ml::Boost* instance, int val) {
		try {
			instance->setBoostType(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_Boost_getWeakCount_const(const cv::ml::Boost* instance) {
		try {
			int ret = instance->getWeakCount();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_Boost_setWeakCount_int(cv::ml::Boost* instance, int val) {
		try {
			instance->setWeakCount(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_Boost_getWeightTrimRate_const(const cv::ml::Boost* instance) {
		try {
			double ret = instance->getWeightTrimRate();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_Boost_setWeightTrimRate_double(cv::ml::Boost* instance, double val) {
		try {
			instance->setWeightTrimRate(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::ml::Boost>*> cv_ml_Boost_create() {
		try {
			cv::Ptr<cv::ml::Boost> ret = cv::ml::Boost::create();
			return Ok(new cv::Ptr<cv::ml::Boost>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::Boost>*>)
	}
	
	Result<cv::Ptr<cv::ml::Boost>*> cv_ml_Boost_load_const_StringX_const_StringX(const char* filepath, const char* nodeName) {
		try {
			cv::Ptr<cv::ml::Boost> ret = cv::ml::Boost::load(std::string(filepath), std::string(nodeName));
			return Ok(new cv::Ptr<cv::ml::Boost>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::Boost>*>)
	}
	
	Result<int> cv_ml_DTrees_getMaxCategories_const(const cv::ml::DTrees* instance) {
		try {
			int ret = instance->getMaxCategories();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_setMaxCategories_int(cv::ml::DTrees* instance, int val) {
		try {
			instance->setMaxCategories(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_DTrees_getMaxDepth_const(const cv::ml::DTrees* instance) {
		try {
			int ret = instance->getMaxDepth();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_setMaxDepth_int(cv::ml::DTrees* instance, int val) {
		try {
			instance->setMaxDepth(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_DTrees_getMinSampleCount_const(const cv::ml::DTrees* instance) {
		try {
			int ret = instance->getMinSampleCount();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_setMinSampleCount_int(cv::ml::DTrees* instance, int val) {
		try {
			instance->setMinSampleCount(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_DTrees_getCVFolds_const(const cv::ml::DTrees* instance) {
		try {
			int ret = instance->getCVFolds();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_setCVFolds_int(cv::ml::DTrees* instance, int val) {
		try {
			instance->setCVFolds(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_ml_DTrees_getUseSurrogates_const(const cv::ml::DTrees* instance) {
		try {
			bool ret = instance->getUseSurrogates();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_ml_DTrees_setUseSurrogates_bool(cv::ml::DTrees* instance, bool val) {
		try {
			instance->setUseSurrogates(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_ml_DTrees_getUse1SERule_const(const cv::ml::DTrees* instance) {
		try {
			bool ret = instance->getUse1SERule();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_ml_DTrees_setUse1SERule_bool(cv::ml::DTrees* instance, bool val) {
		try {
			instance->setUse1SERule(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_ml_DTrees_getTruncatePrunedTree_const(const cv::ml::DTrees* instance) {
		try {
			bool ret = instance->getTruncatePrunedTree();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_ml_DTrees_setTruncatePrunedTree_bool(cv::ml::DTrees* instance, bool val) {
		try {
			instance->setTruncatePrunedTree(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ml_DTrees_getRegressionAccuracy_const(const cv::ml::DTrees* instance) {
		try {
			float ret = instance->getRegressionAccuracy();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ml_DTrees_setRegressionAccuracy_float(cv::ml::DTrees* instance, float val) {
		try {
			instance->setRegressionAccuracy(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_ml_DTrees_getPriors_const(const cv::ml::DTrees* instance) {
		try {
			cv::Mat ret = instance->getPriors();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_ml_DTrees_setPriors_const_MatX(cv::ml::DTrees* instance, const cv::Mat* val) {
		try {
			instance->setPriors(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<std::vector<int>*> cv_ml_DTrees_getRoots_const(const cv::ml::DTrees* instance) {
		try {
			std::vector<int> ret = instance->getRoots();
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<std::vector<int>*>)
	}
	
	Result<std::vector<cv::ml::DTrees::Node>*> cv_ml_DTrees_getNodes_const(const cv::ml::DTrees* instance) {
		try {
			std::vector<cv::ml::DTrees::Node> ret = instance->getNodes();
			return Ok(new std::vector<cv::ml::DTrees::Node>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::ml::DTrees::Node>*>)
	}
	
	Result<std::vector<cv::ml::DTrees::Split>*> cv_ml_DTrees_getSplits_const(const cv::ml::DTrees* instance) {
		try {
			std::vector<cv::ml::DTrees::Split> ret = instance->getSplits();
			return Ok(new std::vector<cv::ml::DTrees::Split>(ret));
		} OCVRS_CATCH(Result<std::vector<cv::ml::DTrees::Split>*>)
	}
	
	Result<std::vector<int>*> cv_ml_DTrees_getSubsets_const(const cv::ml::DTrees* instance) {
		try {
			std::vector<int> ret = instance->getSubsets();
			return Ok(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<std::vector<int>*>)
	}
	
	Result<cv::Ptr<cv::ml::DTrees>*> cv_ml_DTrees_create() {
		try {
			cv::Ptr<cv::ml::DTrees> ret = cv::ml::DTrees::create();
			return Ok(new cv::Ptr<cv::ml::DTrees>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::DTrees>*>)
	}
	
	Result<cv::Ptr<cv::ml::DTrees>*> cv_ml_DTrees_load_const_StringX_const_StringX(const char* filepath, const char* nodeName) {
		try {
			cv::Ptr<cv::ml::DTrees> ret = cv::ml::DTrees::load(std::string(filepath), std::string(nodeName));
			return Ok(new cv::Ptr<cv::ml::DTrees>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::DTrees>*>)
	}
	
	Result<double> cv_ml_DTrees_Node_value_const(const cv::ml::DTrees::Node* instance) {
		try {
			double ret = instance->value;
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_DTrees_Node_setValue_double(cv::ml::DTrees::Node* instance, double val) {
		try {
			instance->value = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_DTrees_Node_classIdx_const(const cv::ml::DTrees::Node* instance) {
		try {
			int ret = instance->classIdx;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_Node_setClassIdx_int(cv::ml::DTrees::Node* instance, int val) {
		try {
			instance->classIdx = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_DTrees_Node_parent_const(const cv::ml::DTrees::Node* instance) {
		try {
			int ret = instance->parent;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_Node_setParent_int(cv::ml::DTrees::Node* instance, int val) {
		try {
			instance->parent = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_DTrees_Node_left_const(const cv::ml::DTrees::Node* instance) {
		try {
			int ret = instance->left;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_Node_setLeft_int(cv::ml::DTrees::Node* instance, int val) {
		try {
			instance->left = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_DTrees_Node_right_const(const cv::ml::DTrees::Node* instance) {
		try {
			int ret = instance->right;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_Node_setRight_int(cv::ml::DTrees::Node* instance, int val) {
		try {
			instance->right = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_DTrees_Node_defaultDir_const(const cv::ml::DTrees::Node* instance) {
		try {
			int ret = instance->defaultDir;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_Node_setDefaultDir_int(cv::ml::DTrees::Node* instance, int val) {
		try {
			instance->defaultDir = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_DTrees_Node_split_const(const cv::ml::DTrees::Node* instance) {
		try {
			int ret = instance->split;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_Node_setSplit_int(cv::ml::DTrees::Node* instance, int val) {
		try {
			instance->split = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_DTrees_Node_delete(cv::ml::DTrees::Node* instance) {
		delete instance;
	}
	Result<cv::ml::DTrees::Node*> cv_ml_DTrees_Node_Node() {
		try {
			cv::ml::DTrees::Node* ret = new cv::ml::DTrees::Node();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::ml::DTrees::Node*>)
	}
	
	Result<int> cv_ml_DTrees_Split_varIdx_const(const cv::ml::DTrees::Split* instance) {
		try {
			int ret = instance->varIdx;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_Split_setVarIdx_int(cv::ml::DTrees::Split* instance, int val) {
		try {
			instance->varIdx = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_ml_DTrees_Split_inversed_const(const cv::ml::DTrees::Split* instance) {
		try {
			bool ret = instance->inversed;
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_ml_DTrees_Split_setInversed_bool(cv::ml::DTrees::Split* instance, bool val) {
		try {
			instance->inversed = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ml_DTrees_Split_quality_const(const cv::ml::DTrees::Split* instance) {
		try {
			float ret = instance->quality;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ml_DTrees_Split_setQuality_float(cv::ml::DTrees::Split* instance, float val) {
		try {
			instance->quality = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_DTrees_Split_next_const(const cv::ml::DTrees::Split* instance) {
		try {
			int ret = instance->next;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_Split_setNext_int(cv::ml::DTrees::Split* instance, int val) {
		try {
			instance->next = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ml_DTrees_Split_c_const(const cv::ml::DTrees::Split* instance) {
		try {
			float ret = instance->c;
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ml_DTrees_Split_setC_float(cv::ml::DTrees::Split* instance, float val) {
		try {
			instance->c = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_DTrees_Split_subsetOfs_const(const cv::ml::DTrees::Split* instance) {
		try {
			int ret = instance->subsetOfs;
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_Split_setSubsetOfs_int(cv::ml::DTrees::Split* instance, int val) {
		try {
			instance->subsetOfs = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_DTrees_Split_delete(cv::ml::DTrees::Split* instance) {
		delete instance;
	}
	Result<cv::ml::DTrees::Split*> cv_ml_DTrees_Split_Split() {
		try {
			cv::ml::DTrees::Split* ret = new cv::ml::DTrees::Split();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::ml::DTrees::Split*>)
	}
	
	Result<int> cv_ml_EM_getClustersNumber_const(const cv::ml::EM* instance) {
		try {
			int ret = instance->getClustersNumber();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_EM_setClustersNumber_int(cv::ml::EM* instance, int val) {
		try {
			instance->setClustersNumber(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_EM_getCovarianceMatrixType_const(const cv::ml::EM* instance) {
		try {
			int ret = instance->getCovarianceMatrixType();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_EM_setCovarianceMatrixType_int(cv::ml::EM* instance, int val) {
		try {
			instance->setCovarianceMatrixType(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::TermCriteria> cv_ml_EM_getTermCriteria_const(const cv::ml::EM* instance) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::TermCriteria>)
	}
	
	Result_void cv_ml_EM_setTermCriteria_const_TermCriteriaX(cv::ml::EM* instance, const cv::TermCriteria* val) {
		try {
			instance->setTermCriteria(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_ml_EM_getWeights_const(const cv::ml::EM* instance) {
		try {
			cv::Mat ret = instance->getWeights();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_ml_EM_getMeans_const(const cv::ml::EM* instance) {
		try {
			cv::Mat ret = instance->getMeans();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_ml_EM_getCovs_const_vector_Mat_X(const cv::ml::EM* instance, std::vector<cv::Mat>* covs) {
		try {
			instance->getCovs(*covs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ml_EM_predict_const_const__InputArrayX_const__OutputArrayX_int(const cv::ml::EM* instance, const cv::_InputArray* samples, const cv::_OutputArray* results, int flags) {
		try {
			float ret = instance->predict(*samples, *results, flags);
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<cv::Vec2d> cv_ml_EM_predict2_const_const__InputArrayX_const__OutputArrayX(const cv::ml::EM* instance, const cv::_InputArray* sample, const cv::_OutputArray* probs) {
		try {
			cv::Vec2d ret = instance->predict2(*sample, *probs);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Vec2d>)
	}
	
	Result<bool> cv_ml_EM_trainEM_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX(cv::ml::EM* instance, const cv::_InputArray* samples, const cv::_OutputArray* logLikelihoods, const cv::_OutputArray* labels, const cv::_OutputArray* probs) {
		try {
			bool ret = instance->trainEM(*samples, *logLikelihoods, *labels, *probs);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ml_EM_trainE_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX(cv::ml::EM* instance, const cv::_InputArray* samples, const cv::_InputArray* means0, const cv::_InputArray* covs0, const cv::_InputArray* weights0, const cv::_OutputArray* logLikelihoods, const cv::_OutputArray* labels, const cv::_OutputArray* probs) {
		try {
			bool ret = instance->trainE(*samples, *means0, *covs0, *weights0, *logLikelihoods, *labels, *probs);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ml_EM_trainM_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX(cv::ml::EM* instance, const cv::_InputArray* samples, const cv::_InputArray* probs0, const cv::_OutputArray* logLikelihoods, const cv::_OutputArray* labels, const cv::_OutputArray* probs) {
		try {
			bool ret = instance->trainM(*samples, *probs0, *logLikelihoods, *labels, *probs);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<cv::Ptr<cv::ml::EM>*> cv_ml_EM_create() {
		try {
			cv::Ptr<cv::ml::EM> ret = cv::ml::EM::create();
			return Ok(new cv::Ptr<cv::ml::EM>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::EM>*>)
	}
	
	Result<cv::Ptr<cv::ml::EM>*> cv_ml_EM_load_const_StringX_const_StringX(const char* filepath, const char* nodeName) {
		try {
			cv::Ptr<cv::ml::EM> ret = cv::ml::EM::load(std::string(filepath), std::string(nodeName));
			return Ok(new cv::Ptr<cv::ml::EM>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::EM>*>)
	}
	
	Result<int> cv_ml_KNearest_getDefaultK_const(const cv::ml::KNearest* instance) {
		try {
			int ret = instance->getDefaultK();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_KNearest_setDefaultK_int(cv::ml::KNearest* instance, int val) {
		try {
			instance->setDefaultK(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_ml_KNearest_getIsClassifier_const(const cv::ml::KNearest* instance) {
		try {
			bool ret = instance->getIsClassifier();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_ml_KNearest_setIsClassifier_bool(cv::ml::KNearest* instance, bool val) {
		try {
			instance->setIsClassifier(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_KNearest_getEmax_const(const cv::ml::KNearest* instance) {
		try {
			int ret = instance->getEmax();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_KNearest_setEmax_int(cv::ml::KNearest* instance, int val) {
		try {
			instance->setEmax(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_KNearest_getAlgorithmType_const(const cv::ml::KNearest* instance) {
		try {
			int ret = instance->getAlgorithmType();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_KNearest_setAlgorithmType_int(cv::ml::KNearest* instance, int val) {
		try {
			instance->setAlgorithmType(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ml_KNearest_findNearest_const_const__InputArrayX_int_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX(const cv::ml::KNearest* instance, const cv::_InputArray* samples, int k, const cv::_OutputArray* results, const cv::_OutputArray* neighborResponses, const cv::_OutputArray* dist) {
		try {
			float ret = instance->findNearest(*samples, k, *results, *neighborResponses, *dist);
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<cv::Ptr<cv::ml::KNearest>*> cv_ml_KNearest_create() {
		try {
			cv::Ptr<cv::ml::KNearest> ret = cv::ml::KNearest::create();
			return Ok(new cv::Ptr<cv::ml::KNearest>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::KNearest>*>)
	}
	
	Result<cv::Ptr<cv::ml::KNearest>*> cv_ml_KNearest_load_const_StringX(const char* filepath) {
		try {
			cv::Ptr<cv::ml::KNearest> ret = cv::ml::KNearest::load(std::string(filepath));
			return Ok(new cv::Ptr<cv::ml::KNearest>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::KNearest>*>)
	}
	
	Result<double> cv_ml_LogisticRegression_getLearningRate_const(const cv::ml::LogisticRegression* instance) {
		try {
			double ret = instance->getLearningRate();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_LogisticRegression_setLearningRate_double(cv::ml::LogisticRegression* instance, double val) {
		try {
			instance->setLearningRate(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_LogisticRegression_getIterations_const(const cv::ml::LogisticRegression* instance) {
		try {
			int ret = instance->getIterations();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_LogisticRegression_setIterations_int(cv::ml::LogisticRegression* instance, int val) {
		try {
			instance->setIterations(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_LogisticRegression_getRegularization_const(const cv::ml::LogisticRegression* instance) {
		try {
			int ret = instance->getRegularization();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_LogisticRegression_setRegularization_int(cv::ml::LogisticRegression* instance, int val) {
		try {
			instance->setRegularization(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_LogisticRegression_getTrainMethod_const(const cv::ml::LogisticRegression* instance) {
		try {
			int ret = instance->getTrainMethod();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_LogisticRegression_setTrainMethod_int(cv::ml::LogisticRegression* instance, int val) {
		try {
			instance->setTrainMethod(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_LogisticRegression_getMiniBatchSize_const(const cv::ml::LogisticRegression* instance) {
		try {
			int ret = instance->getMiniBatchSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_LogisticRegression_setMiniBatchSize_int(cv::ml::LogisticRegression* instance, int val) {
		try {
			instance->setMiniBatchSize(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::TermCriteria> cv_ml_LogisticRegression_getTermCriteria_const(const cv::ml::LogisticRegression* instance) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::TermCriteria>)
	}
	
	Result_void cv_ml_LogisticRegression_setTermCriteria_TermCriteria(cv::ml::LogisticRegression* instance, const cv::TermCriteria* val) {
		try {
			instance->setTermCriteria(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ml_LogisticRegression_predict_const_const__InputArrayX_const__OutputArrayX_int(const cv::ml::LogisticRegression* instance, const cv::_InputArray* samples, const cv::_OutputArray* results, int flags) {
		try {
			float ret = instance->predict(*samples, *results, flags);
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<cv::Mat*> cv_ml_LogisticRegression_get_learnt_thetas_const(const cv::ml::LogisticRegression* instance) {
		try {
			cv::Mat ret = instance->get_learnt_thetas();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Ptr<cv::ml::LogisticRegression>*> cv_ml_LogisticRegression_create() {
		try {
			cv::Ptr<cv::ml::LogisticRegression> ret = cv::ml::LogisticRegression::create();
			return Ok(new cv::Ptr<cv::ml::LogisticRegression>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::LogisticRegression>*>)
	}
	
	Result<cv::Ptr<cv::ml::LogisticRegression>*> cv_ml_LogisticRegression_load_const_StringX_const_StringX(const char* filepath, const char* nodeName) {
		try {
			cv::Ptr<cv::ml::LogisticRegression> ret = cv::ml::LogisticRegression::load(std::string(filepath), std::string(nodeName));
			return Ok(new cv::Ptr<cv::ml::LogisticRegression>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::LogisticRegression>*>)
	}
	
	Result<float> cv_ml_NormalBayesClassifier_predictProb_const_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_int(const cv::ml::NormalBayesClassifier* instance, const cv::_InputArray* inputs, const cv::_OutputArray* outputs, const cv::_OutputArray* outputProbs, int flags) {
		try {
			float ret = instance->predictProb(*inputs, *outputs, *outputProbs, flags);
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<cv::Ptr<cv::ml::NormalBayesClassifier>*> cv_ml_NormalBayesClassifier_create() {
		try {
			cv::Ptr<cv::ml::NormalBayesClassifier> ret = cv::ml::NormalBayesClassifier::create();
			return Ok(new cv::Ptr<cv::ml::NormalBayesClassifier>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::NormalBayesClassifier>*>)
	}
	
	Result<cv::Ptr<cv::ml::NormalBayesClassifier>*> cv_ml_NormalBayesClassifier_load_const_StringX_const_StringX(const char* filepath, const char* nodeName) {
		try {
			cv::Ptr<cv::ml::NormalBayesClassifier> ret = cv::ml::NormalBayesClassifier::load(std::string(filepath), std::string(nodeName));
			return Ok(new cv::Ptr<cv::ml::NormalBayesClassifier>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::NormalBayesClassifier>*>)
	}
	
	Result<double> cv_ml_ParamGrid_minVal_const(const cv::ml::ParamGrid* instance) {
		try {
			double ret = instance->minVal;
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ParamGrid_setMinVal_double(cv::ml::ParamGrid* instance, double val) {
		try {
			instance->minVal = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ParamGrid_maxVal_const(const cv::ml::ParamGrid* instance) {
		try {
			double ret = instance->maxVal;
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ParamGrid_setMaxVal_double(cv::ml::ParamGrid* instance, double val) {
		try {
			instance->maxVal = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ParamGrid_logStep_const(const cv::ml::ParamGrid* instance) {
		try {
			double ret = instance->logStep;
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ParamGrid_setLogStep_double(cv::ml::ParamGrid* instance, double val) {
		try {
			instance->logStep = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_ParamGrid_delete(cv::ml::ParamGrid* instance) {
		delete instance;
	}
	Result<cv::ml::ParamGrid*> cv_ml_ParamGrid_ParamGrid() {
		try {
			cv::ml::ParamGrid* ret = new cv::ml::ParamGrid();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::ml::ParamGrid*>)
	}
	
	Result<cv::ml::ParamGrid*> cv_ml_ParamGrid_ParamGrid_double_double_double(double _minVal, double _maxVal, double _logStep) {
		try {
			cv::ml::ParamGrid* ret = new cv::ml::ParamGrid(_minVal, _maxVal, _logStep);
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::ml::ParamGrid*>)
	}
	
	Result<cv::Ptr<cv::ml::ParamGrid>*> cv_ml_ParamGrid_create_double_double_double(double minVal, double maxVal, double logstep) {
		try {
			cv::Ptr<cv::ml::ParamGrid> ret = cv::ml::ParamGrid::create(minVal, maxVal, logstep);
			return Ok(new cv::Ptr<cv::ml::ParamGrid>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::ParamGrid>*>)
	}
	
	Result<bool> cv_ml_RTrees_getCalculateVarImportance_const(const cv::ml::RTrees* instance) {
		try {
			bool ret = instance->getCalculateVarImportance();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_ml_RTrees_setCalculateVarImportance_bool(cv::ml::RTrees* instance, bool val) {
		try {
			instance->setCalculateVarImportance(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_RTrees_getActiveVarCount_const(const cv::ml::RTrees* instance) {
		try {
			int ret = instance->getActiveVarCount();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_RTrees_setActiveVarCount_int(cv::ml::RTrees* instance, int val) {
		try {
			instance->setActiveVarCount(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::TermCriteria> cv_ml_RTrees_getTermCriteria_const(const cv::ml::RTrees* instance) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::TermCriteria>)
	}
	
	Result_void cv_ml_RTrees_setTermCriteria_const_TermCriteriaX(cv::ml::RTrees* instance, const cv::TermCriteria* val) {
		try {
			instance->setTermCriteria(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_ml_RTrees_getVarImportance_const(const cv::ml::RTrees* instance) {
		try {
			cv::Mat ret = instance->getVarImportance();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_ml_RTrees_getVotes_const_const__InputArrayX_const__OutputArrayX_int(const cv::ml::RTrees* instance, const cv::_InputArray* samples, const cv::_OutputArray* results, int flags) {
		try {
			instance->getVotes(*samples, *results, flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::ml::RTrees>*> cv_ml_RTrees_create() {
		try {
			cv::Ptr<cv::ml::RTrees> ret = cv::ml::RTrees::create();
			return Ok(new cv::Ptr<cv::ml::RTrees>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::RTrees>*>)
	}
	
	Result<cv::Ptr<cv::ml::RTrees>*> cv_ml_RTrees_load_const_StringX_const_StringX(const char* filepath, const char* nodeName) {
		try {
			cv::Ptr<cv::ml::RTrees> ret = cv::ml::RTrees::load(std::string(filepath), std::string(nodeName));
			return Ok(new cv::Ptr<cv::ml::RTrees>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::RTrees>*>)
	}
	
	Result<int> cv_ml_SVM_getType_const(const cv::ml::SVM* instance) {
		try {
			int ret = instance->getType();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_SVM_setType_int(cv::ml::SVM* instance, int val) {
		try {
			instance->setType(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_SVM_getGamma_const(const cv::ml::SVM* instance) {
		try {
			double ret = instance->getGamma();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_SVM_setGamma_double(cv::ml::SVM* instance, double val) {
		try {
			instance->setGamma(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_SVM_getCoef0_const(const cv::ml::SVM* instance) {
		try {
			double ret = instance->getCoef0();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_SVM_setCoef0_double(cv::ml::SVM* instance, double val) {
		try {
			instance->setCoef0(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_SVM_getDegree_const(const cv::ml::SVM* instance) {
		try {
			double ret = instance->getDegree();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_SVM_setDegree_double(cv::ml::SVM* instance, double val) {
		try {
			instance->setDegree(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_SVM_getC_const(const cv::ml::SVM* instance) {
		try {
			double ret = instance->getC();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_SVM_setC_double(cv::ml::SVM* instance, double val) {
		try {
			instance->setC(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_SVM_getNu_const(const cv::ml::SVM* instance) {
		try {
			double ret = instance->getNu();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_SVM_setNu_double(cv::ml::SVM* instance, double val) {
		try {
			instance->setNu(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_SVM_getP_const(const cv::ml::SVM* instance) {
		try {
			double ret = instance->getP();
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_SVM_setP_double(cv::ml::SVM* instance, double val) {
		try {
			instance->setP(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_ml_SVM_getClassWeights_const(const cv::ml::SVM* instance) {
		try {
			cv::Mat ret = instance->getClassWeights();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_ml_SVM_setClassWeights_const_MatX(cv::ml::SVM* instance, const cv::Mat* val) {
		try {
			instance->setClassWeights(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::TermCriteria> cv_ml_SVM_getTermCriteria_const(const cv::ml::SVM* instance) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::TermCriteria>)
	}
	
	Result_void cv_ml_SVM_setTermCriteria_const_TermCriteriaX(cv::ml::SVM* instance, const cv::TermCriteria* val) {
		try {
			instance->setTermCriteria(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_SVM_getKernelType_const(const cv::ml::SVM* instance) {
		try {
			int ret = instance->getKernelType();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_SVM_setKernel_int(cv::ml::SVM* instance, int kernelType) {
		try {
			instance->setKernel(kernelType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ml_SVM_setCustomKernel_const_Ptr_Kernel_X(cv::ml::SVM* instance, const cv::Ptr<cv::ml::SVM::Kernel>* _kernel) {
		try {
			instance->setCustomKernel(*_kernel);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_ml_SVM_trainAuto_const_Ptr_TrainData_X_int_ParamGrid_ParamGrid_ParamGrid_ParamGrid_ParamGrid_ParamGrid_bool(cv::ml::SVM* instance, const cv::Ptr<cv::ml::TrainData>* data, int kFold, cv::ml::ParamGrid* Cgrid, cv::ml::ParamGrid* gammaGrid, cv::ml::ParamGrid* pGrid, cv::ml::ParamGrid* nuGrid, cv::ml::ParamGrid* coeffGrid, cv::ml::ParamGrid* degreeGrid, bool balanced) {
		try {
			bool ret = instance->trainAuto(*data, kFold, *Cgrid, *gammaGrid, *pGrid, *nuGrid, *coeffGrid, *degreeGrid, balanced);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ml_SVM_trainAuto_const__InputArrayX_int_const__InputArrayX_int_Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__bool(cv::ml::SVM* instance, const cv::_InputArray* samples, int layout, const cv::_InputArray* responses, int kFold, cv::Ptr<cv::ml::ParamGrid>* Cgrid, cv::Ptr<cv::ml::ParamGrid>* gammaGrid, cv::Ptr<cv::ml::ParamGrid>* pGrid, cv::Ptr<cv::ml::ParamGrid>* nuGrid, cv::Ptr<cv::ml::ParamGrid>* coeffGrid, cv::Ptr<cv::ml::ParamGrid>* degreeGrid, bool balanced) {
		try {
			bool ret = instance->trainAuto(*samples, layout, *responses, kFold, *Cgrid, *gammaGrid, *pGrid, *nuGrid, *coeffGrid, *degreeGrid, balanced);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<cv::Mat*> cv_ml_SVM_getSupportVectors_const(const cv::ml::SVM* instance) {
		try {
			cv::Mat ret = instance->getSupportVectors();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_ml_SVM_getUncompressedSupportVectors_const(const cv::ml::SVM* instance) {
		try {
			cv::Mat ret = instance->getUncompressedSupportVectors();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<double> cv_ml_SVM_getDecisionFunction_const_int_const__OutputArrayX_const__OutputArrayX(const cv::ml::SVM* instance, int i, const cv::_OutputArray* alpha, const cv::_OutputArray* svidx) {
		try {
			double ret = instance->getDecisionFunction(i, *alpha, *svidx);
			return Ok(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<cv::ml::ParamGrid*> cv_ml_SVM_getDefaultGrid_int(int param_id) {
		try {
			cv::ml::ParamGrid ret = cv::ml::SVM::getDefaultGrid(param_id);
			return Ok(new cv::ml::ParamGrid(ret));
		} OCVRS_CATCH(Result<cv::ml::ParamGrid*>)
	}
	
	Result<cv::Ptr<cv::ml::ParamGrid>*> cv_ml_SVM_getDefaultGridPtr_int(int param_id) {
		try {
			cv::Ptr<cv::ml::ParamGrid> ret = cv::ml::SVM::getDefaultGridPtr(param_id);
			return Ok(new cv::Ptr<cv::ml::ParamGrid>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::ParamGrid>*>)
	}
	
	Result<cv::Ptr<cv::ml::SVM>*> cv_ml_SVM_create() {
		try {
			cv::Ptr<cv::ml::SVM> ret = cv::ml::SVM::create();
			return Ok(new cv::Ptr<cv::ml::SVM>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::SVM>*>)
	}
	
	Result<cv::Ptr<cv::ml::SVM>*> cv_ml_SVM_load_const_StringX(const char* filepath) {
		try {
			cv::Ptr<cv::ml::SVM> ret = cv::ml::SVM::load(std::string(filepath));
			return Ok(new cv::Ptr<cv::ml::SVM>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::SVM>*>)
	}
	
	Result<int> cv_ml_SVM_Kernel_getType_const(const cv::ml::SVM::Kernel* instance) {
		try {
			int ret = instance->getType();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_SVM_Kernel_calc_int_int_const_floatX_const_floatX_floatX(cv::ml::SVM::Kernel* instance, int vcount, int n, const float* vecs, const float* another, float* results) {
		try {
			instance->calc(vcount, n, vecs, another, results);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_ml_SVMSGD_getWeights(cv::ml::SVMSGD* instance) {
		try {
			cv::Mat ret = instance->getWeights();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<float> cv_ml_SVMSGD_getShift(cv::ml::SVMSGD* instance) {
		try {
			float ret = instance->getShift();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<cv::Ptr<cv::ml::SVMSGD>*> cv_ml_SVMSGD_create() {
		try {
			cv::Ptr<cv::ml::SVMSGD> ret = cv::ml::SVMSGD::create();
			return Ok(new cv::Ptr<cv::ml::SVMSGD>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::SVMSGD>*>)
	}
	
	Result<cv::Ptr<cv::ml::SVMSGD>*> cv_ml_SVMSGD_load_const_StringX_const_StringX(const char* filepath, const char* nodeName) {
		try {
			cv::Ptr<cv::ml::SVMSGD> ret = cv::ml::SVMSGD::load(std::string(filepath), std::string(nodeName));
			return Ok(new cv::Ptr<cv::ml::SVMSGD>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::SVMSGD>*>)
	}
	
	Result_void cv_ml_SVMSGD_setOptimalParameters_int_int(cv::ml::SVMSGD* instance, int svmsgdType, int marginType) {
		try {
			instance->setOptimalParameters(svmsgdType, marginType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_SVMSGD_getSvmsgdType_const(const cv::ml::SVMSGD* instance) {
		try {
			int ret = instance->getSvmsgdType();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_SVMSGD_setSvmsgdType_int(cv::ml::SVMSGD* instance, int svmsgdType) {
		try {
			instance->setSvmsgdType(svmsgdType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_SVMSGD_getMarginType_const(const cv::ml::SVMSGD* instance) {
		try {
			int ret = instance->getMarginType();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_SVMSGD_setMarginType_int(cv::ml::SVMSGD* instance, int marginType) {
		try {
			instance->setMarginType(marginType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ml_SVMSGD_getMarginRegularization_const(const cv::ml::SVMSGD* instance) {
		try {
			float ret = instance->getMarginRegularization();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ml_SVMSGD_setMarginRegularization_float(cv::ml::SVMSGD* instance, float marginRegularization) {
		try {
			instance->setMarginRegularization(marginRegularization);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ml_SVMSGD_getInitialStepSize_const(const cv::ml::SVMSGD* instance) {
		try {
			float ret = instance->getInitialStepSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ml_SVMSGD_setInitialStepSize_float(cv::ml::SVMSGD* instance, float InitialStepSize) {
		try {
			instance->setInitialStepSize(InitialStepSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ml_SVMSGD_getStepDecreasingPower_const(const cv::ml::SVMSGD* instance) {
		try {
			float ret = instance->getStepDecreasingPower();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ml_SVMSGD_setStepDecreasingPower_float(cv::ml::SVMSGD* instance, float stepDecreasingPower) {
		try {
			instance->setStepDecreasingPower(stepDecreasingPower);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::TermCriteria> cv_ml_SVMSGD_getTermCriteria_const(const cv::ml::SVMSGD* instance) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::TermCriteria>)
	}
	
	Result_void cv_ml_SVMSGD_setTermCriteria_const_TermCriteriaX(cv::ml::SVMSGD* instance, const cv::TermCriteria* val) {
		try {
			instance->setTermCriteria(*val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_StatModel_getVarCount_const(const cv::ml::StatModel* instance) {
		try {
			int ret = instance->getVarCount();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<bool> cv_ml_StatModel_empty_const(const cv::ml::StatModel* instance) {
		try {
			bool ret = instance->empty();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ml_StatModel_isTrained_const(const cv::ml::StatModel* instance) {
		try {
			bool ret = instance->isTrained();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ml_StatModel_isClassifier_const(const cv::ml::StatModel* instance) {
		try {
			bool ret = instance->isClassifier();
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ml_StatModel_train_const_Ptr_TrainData_X_int(cv::ml::StatModel* instance, const cv::Ptr<cv::ml::TrainData>* trainData, int flags) {
		try {
			bool ret = instance->train(*trainData, flags);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ml_StatModel_train_const__InputArrayX_int_const__InputArrayX(cv::ml::StatModel* instance, const cv::_InputArray* samples, int layout, const cv::_InputArray* responses) {
		try {
			bool ret = instance->train(*samples, layout, *responses);
			return Ok(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<float> cv_ml_StatModel_calcError_const_const_Ptr_TrainData_X_bool_const__OutputArrayX(const cv::ml::StatModel* instance, const cv::Ptr<cv::ml::TrainData>* data, bool test, const cv::_OutputArray* resp) {
		try {
			float ret = instance->calcError(*data, test, *resp);
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<float> cv_ml_StatModel_predict_const_const__InputArrayX_const__OutputArrayX_int(const cv::ml::StatModel* instance, const cv::_InputArray* samples, const cv::_OutputArray* results, int flags) {
		try {
			float ret = instance->predict(*samples, *results, flags);
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<float> cv_ml_TrainData_missingValue() {
		try {
			float ret = cv::ml::TrainData::missingValue();
			return Ok(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<int> cv_ml_TrainData_getLayout_const(const cv::ml::TrainData* instance) {
		try {
			int ret = instance->getLayout();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ml_TrainData_getNTrainSamples_const(const cv::ml::TrainData* instance) {
		try {
			int ret = instance->getNTrainSamples();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ml_TrainData_getNTestSamples_const(const cv::ml::TrainData* instance) {
		try {
			int ret = instance->getNTestSamples();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ml_TrainData_getNSamples_const(const cv::ml::TrainData* instance) {
		try {
			int ret = instance->getNSamples();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ml_TrainData_getNVars_const(const cv::ml::TrainData* instance) {
		try {
			int ret = instance->getNVars();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ml_TrainData_getNAllVars_const(const cv::ml::TrainData* instance) {
		try {
			int ret = instance->getNAllVars();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_TrainData_getSample_const_const__InputArrayX_int_floatX(const cv::ml::TrainData* instance, const cv::_InputArray* varIdx, int sidx, float* buf) {
		try {
			instance->getSample(*varIdx, sidx, buf);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getSamples_const(const cv::ml::TrainData* instance) {
		try {
			cv::Mat ret = instance->getSamples();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getMissing_const(const cv::ml::TrainData* instance) {
		try {
			cv::Mat ret = instance->getMissing();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getTrainSamples_const_int_bool_bool(const cv::ml::TrainData* instance, int layout, bool compressSamples, bool compressVars) {
		try {
			cv::Mat ret = instance->getTrainSamples(layout, compressSamples, compressVars);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getTrainResponses_const(const cv::ml::TrainData* instance) {
		try {
			cv::Mat ret = instance->getTrainResponses();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getTrainNormCatResponses_const(const cv::ml::TrainData* instance) {
		try {
			cv::Mat ret = instance->getTrainNormCatResponses();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getTestResponses_const(const cv::ml::TrainData* instance) {
		try {
			cv::Mat ret = instance->getTestResponses();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getTestNormCatResponses_const(const cv::ml::TrainData* instance) {
		try {
			cv::Mat ret = instance->getTestNormCatResponses();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getResponses_const(const cv::ml::TrainData* instance) {
		try {
			cv::Mat ret = instance->getResponses();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getNormCatResponses_const(const cv::ml::TrainData* instance) {
		try {
			cv::Mat ret = instance->getNormCatResponses();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getSampleWeights_const(const cv::ml::TrainData* instance) {
		try {
			cv::Mat ret = instance->getSampleWeights();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getTrainSampleWeights_const(const cv::ml::TrainData* instance) {
		try {
			cv::Mat ret = instance->getTrainSampleWeights();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getTestSampleWeights_const(const cv::ml::TrainData* instance) {
		try {
			cv::Mat ret = instance->getTestSampleWeights();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getVarIdx_const(const cv::ml::TrainData* instance) {
		try {
			cv::Mat ret = instance->getVarIdx();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getVarType_const(const cv::ml::TrainData* instance) {
		try {
			cv::Mat ret = instance->getVarType();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getVarSymbolFlags_const(const cv::ml::TrainData* instance) {
		try {
			cv::Mat ret = instance->getVarSymbolFlags();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<int> cv_ml_TrainData_getResponseType_const(const cv::ml::TrainData* instance) {
		try {
			int ret = instance->getResponseType();
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getTrainSampleIdx_const(const cv::ml::TrainData* instance) {
		try {
			cv::Mat ret = instance->getTrainSampleIdx();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getTestSampleIdx_const(const cv::ml::TrainData* instance) {
		try {
			cv::Mat ret = instance->getTestSampleIdx();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_ml_TrainData_getValues_const_int_const__InputArrayX_floatX(const cv::ml::TrainData* instance, int vi, const cv::_InputArray* sidx, float* values) {
		try {
			instance->getValues(vi, *sidx, values);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ml_TrainData_getNormCatValues_const_int_const__InputArrayX_intX(const cv::ml::TrainData* instance, int vi, const cv::_InputArray* sidx, int* values) {
		try {
			instance->getNormCatValues(vi, *sidx, values);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getDefaultSubstValues_const(const cv::ml::TrainData* instance) {
		try {
			cv::Mat ret = instance->getDefaultSubstValues();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<int> cv_ml_TrainData_getCatCount_const_int(const cv::ml::TrainData* instance, int vi) {
		try {
			int ret = instance->getCatCount(vi);
			return Ok(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getClassLabels_const(const cv::ml::TrainData* instance) {
		try {
			cv::Mat ret = instance->getClassLabels();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getCatOfs_const(const cv::ml::TrainData* instance) {
		try {
			cv::Mat ret = instance->getCatOfs();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getCatMap_const(const cv::ml::TrainData* instance) {
		try {
			cv::Mat ret = instance->getCatMap();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_ml_TrainData_setTrainTestSplit_int_bool(cv::ml::TrainData* instance, int count, bool shuffle) {
		try {
			instance->setTrainTestSplit(count, shuffle);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ml_TrainData_setTrainTestSplitRatio_double_bool(cv::ml::TrainData* instance, double ratio, bool shuffle) {
		try {
			instance->setTrainTestSplitRatio(ratio, shuffle);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ml_TrainData_shuffleTrainTest(cv::ml::TrainData* instance) {
		try {
			instance->shuffleTrainTest();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getTestSamples_const(const cv::ml::TrainData* instance) {
		try {
			cv::Mat ret = instance->getTestSamples();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_ml_TrainData_getNames_const_vector_String_X(const cv::ml::TrainData* instance, std::vector<cv::String>* names) {
		try {
			instance->getNames(*names);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getSubVector_const_MatX_const_MatX(const cv::Mat* vec, const cv::Mat* idx) {
		try {
			cv::Mat ret = cv::ml::TrainData::getSubVector(*vec, *idx);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_ml_TrainData_getSubMatrix_const_MatX_const_MatX_int(const cv::Mat* matrix, const cv::Mat* idx, int layout) {
		try {
			cv::Mat ret = cv::ml::TrainData::getSubMatrix(*matrix, *idx, layout);
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Ptr<cv::ml::TrainData>*> cv_ml_TrainData_loadFromCSV_const_StringX_int_int_int_const_StringX_char_char(const char* filename, int headerLineCount, int responseStartIdx, int responseEndIdx, const char* varTypeSpec, char delimiter, char missch) {
		try {
			cv::Ptr<cv::ml::TrainData> ret = cv::ml::TrainData::loadFromCSV(std::string(filename), headerLineCount, responseStartIdx, responseEndIdx, std::string(varTypeSpec), delimiter, missch);
			return Ok(new cv::Ptr<cv::ml::TrainData>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::TrainData>*>)
	}
	
	Result<cv::Ptr<cv::ml::TrainData>*> cv_ml_TrainData_create_const__InputArrayX_int_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX(const cv::_InputArray* samples, int layout, const cv::_InputArray* responses, const cv::_InputArray* varIdx, const cv::_InputArray* sampleIdx, const cv::_InputArray* sampleWeights, const cv::_InputArray* varType) {
		try {
			cv::Ptr<cv::ml::TrainData> ret = cv::ml::TrainData::create(*samples, layout, *responses, *varIdx, *sampleIdx, *sampleWeights, *varType);
			return Ok(new cv::Ptr<cv::ml::TrainData>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::ml::TrainData>*>)
	}
	
}
