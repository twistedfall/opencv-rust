#include "common.hpp"
#include <opencv2/ml.hpp>
#include "ml_types.hpp"

extern "C" {
	Result_void cv_ml_createConcentricSpheresTestSet_int_int_int_const__OutputArrayX_const__OutputArrayX(int nsamples, int nfeatures, int nclasses, void* samples, void* responses) {
		try {
			cv::ml::createConcentricSpheresTestSet(nsamples, nfeatures, nclasses, *reinterpret_cast<const cv::_OutputArray*>(samples), *reinterpret_cast<const cv::_OutputArray*>(responses));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ml_randMVNormal_const__InputArrayX_const__InputArrayX_int_const__OutputArrayX(void* mean, void* cov, int nsamples, void* samples) {
		try {
			cv::ml::randMVNormal(*reinterpret_cast<const cv::_InputArray*>(mean), *reinterpret_cast<const cv::_InputArray*>(cov), nsamples, *reinterpret_cast<const cv::_OutputArray*>(samples));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ml_ANN_MLP_setTrainMethod_int_double_double(void* instance, int method, double param1, double param2) {
		try {
			reinterpret_cast<cv::ml::ANN_MLP*>(instance)->setTrainMethod(method, param1, param2);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_ANN_MLP_getTrainMethod_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::ANN_MLP*>(instance)->getTrainMethod();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_ANN_MLP_setActivationFunction_int_double_double(void* instance, int type, double param1, double param2) {
		try {
			reinterpret_cast<cv::ml::ANN_MLP*>(instance)->setActivationFunction(type, param1, param2);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ml_ANN_MLP_setLayerSizes_const__InputArrayX(void* instance, void* _layer_sizes) {
		try {
			reinterpret_cast<cv::ml::ANN_MLP*>(instance)->setLayerSizes(*reinterpret_cast<const cv::_InputArray*>(_layer_sizes));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ml_ANN_MLP_getLayerSizes_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::ANN_MLP*>(instance)->getLayerSizes();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_ANN_MLP_getTermCriteria_const(void* instance) {
		try {
			cv::TermCriteria ret = reinterpret_cast<cv::ml::ANN_MLP*>(instance)->getTermCriteria();
			return Ok<void*>(new cv::TermCriteria(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ml_ANN_MLP_setTermCriteria_TermCriteria(void* instance, void* val) {
		try {
			reinterpret_cast<cv::ml::ANN_MLP*>(instance)->setTermCriteria(*reinterpret_cast<cv::TermCriteria*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ANN_MLP_getBackpropWeightScale_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ml::ANN_MLP*>(instance)->getBackpropWeightScale();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ANN_MLP_setBackpropWeightScale_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::ml::ANN_MLP*>(instance)->setBackpropWeightScale(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ANN_MLP_getBackpropMomentumScale_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ml::ANN_MLP*>(instance)->getBackpropMomentumScale();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ANN_MLP_setBackpropMomentumScale_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::ml::ANN_MLP*>(instance)->setBackpropMomentumScale(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ANN_MLP_getRpropDW0_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ml::ANN_MLP*>(instance)->getRpropDW0();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ANN_MLP_setRpropDW0_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::ml::ANN_MLP*>(instance)->setRpropDW0(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ANN_MLP_getRpropDWPlus_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ml::ANN_MLP*>(instance)->getRpropDWPlus();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ANN_MLP_setRpropDWPlus_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::ml::ANN_MLP*>(instance)->setRpropDWPlus(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ANN_MLP_getRpropDWMinus_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ml::ANN_MLP*>(instance)->getRpropDWMinus();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ANN_MLP_setRpropDWMinus_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::ml::ANN_MLP*>(instance)->setRpropDWMinus(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ANN_MLP_getRpropDWMin_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ml::ANN_MLP*>(instance)->getRpropDWMin();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ANN_MLP_setRpropDWMin_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::ml::ANN_MLP*>(instance)->setRpropDWMin(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ANN_MLP_getRpropDWMax_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ml::ANN_MLP*>(instance)->getRpropDWMax();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ANN_MLP_setRpropDWMax_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::ml::ANN_MLP*>(instance)->setRpropDWMax(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ANN_MLP_getAnnealInitialT_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ml::ANN_MLP*>(instance)->getAnnealInitialT();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ANN_MLP_setAnnealInitialT_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::ml::ANN_MLP*>(instance)->setAnnealInitialT(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ANN_MLP_getAnnealFinalT_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ml::ANN_MLP*>(instance)->getAnnealFinalT();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ANN_MLP_setAnnealFinalT_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::ml::ANN_MLP*>(instance)->setAnnealFinalT(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ANN_MLP_getAnnealCoolingRatio_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ml::ANN_MLP*>(instance)->getAnnealCoolingRatio();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ANN_MLP_setAnnealCoolingRatio_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::ml::ANN_MLP*>(instance)->setAnnealCoolingRatio(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_ANN_MLP_getAnnealItePerStep_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::ANN_MLP*>(instance)->getAnnealItePerStep();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_ANN_MLP_setAnnealItePerStep_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::ANN_MLP*>(instance)->setAnnealItePerStep(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ml_ANN_MLP_setAnnealEnergyRNG_const_RNGX(void* instance, void* rng) {
		try {
			reinterpret_cast<cv::ml::ANN_MLP*>(instance)->setAnnealEnergyRNG(*reinterpret_cast<const cv::RNG*>(rng));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ml_ANN_MLP_getWeights_const_int(void* instance, int layerIdx) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::ANN_MLP*>(instance)->getWeights(layerIdx);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_ANN_MLP_create() {
		try {
			cv::Ptr<cv::ml::ANN_MLP> ret = cv::ml::ANN_MLP::create();
			return Ok<void*>(new cv::Ptr<cv::ml::ANN_MLP>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_ANN_MLP_load_const_StringX(const char* filepath) {
		try {
			cv::Ptr<cv::ml::ANN_MLP> ret = cv::ml::ANN_MLP::load(std::string(filepath));
			return Ok<void*>(new cv::Ptr<cv::ml::ANN_MLP>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_ml_Boost_getBoostType_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::Boost*>(instance)->getBoostType();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_Boost_setBoostType_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::Boost*>(instance)->setBoostType(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_Boost_getWeakCount_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::Boost*>(instance)->getWeakCount();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_Boost_setWeakCount_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::Boost*>(instance)->setWeakCount(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_Boost_getWeightTrimRate_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ml::Boost*>(instance)->getWeightTrimRate();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_Boost_setWeightTrimRate_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::ml::Boost*>(instance)->setWeightTrimRate(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ml_Boost_create() {
		try {
			cv::Ptr<cv::ml::Boost> ret = cv::ml::Boost::create();
			return Ok<void*>(new cv::Ptr<cv::ml::Boost>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_Boost_load_const_StringX_const_StringX(const char* filepath, const char* nodeName) {
		try {
			cv::Ptr<cv::ml::Boost> ret = cv::ml::Boost::load(std::string(filepath), std::string(nodeName));
			return Ok<void*>(new cv::Ptr<cv::ml::Boost>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_ml_DTrees_getMaxCategories_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::DTrees*>(instance)->getMaxCategories();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_setMaxCategories_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::DTrees*>(instance)->setMaxCategories(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_DTrees_getMaxDepth_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::DTrees*>(instance)->getMaxDepth();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_setMaxDepth_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::DTrees*>(instance)->setMaxDepth(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_DTrees_getMinSampleCount_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::DTrees*>(instance)->getMinSampleCount();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_setMinSampleCount_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::DTrees*>(instance)->setMinSampleCount(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_DTrees_getCVFolds_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::DTrees*>(instance)->getCVFolds();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_setCVFolds_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::DTrees*>(instance)->setCVFolds(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_ml_DTrees_getUseSurrogates_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ml::DTrees*>(instance)->getUseSurrogates();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_ml_DTrees_setUseSurrogates_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::ml::DTrees*>(instance)->setUseSurrogates(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_ml_DTrees_getUse1SERule_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ml::DTrees*>(instance)->getUse1SERule();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_ml_DTrees_setUse1SERule_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::ml::DTrees*>(instance)->setUse1SERule(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_ml_DTrees_getTruncatePrunedTree_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ml::DTrees*>(instance)->getTruncatePrunedTree();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_ml_DTrees_setTruncatePrunedTree_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::ml::DTrees*>(instance)->setTruncatePrunedTree(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ml_DTrees_getRegressionAccuracy_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::ml::DTrees*>(instance)->getRegressionAccuracy();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ml_DTrees_setRegressionAccuracy_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::ml::DTrees*>(instance)->setRegressionAccuracy(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ml_DTrees_getPriors_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::DTrees*>(instance)->getPriors();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ml_DTrees_setPriors_const_MatX(void* instance, void* val) {
		try {
			reinterpret_cast<cv::ml::DTrees*>(instance)->setPriors(*reinterpret_cast<const cv::Mat*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ml_DTrees_getRoots_const(void* instance) {
		try {
			std::vector<int> ret = reinterpret_cast<cv::ml::DTrees*>(instance)->getRoots();
			return Ok<void*>(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_DTrees_getNodes_const(void* instance) {
		try {
			std::vector<cv::ml::DTrees::Node> ret = reinterpret_cast<cv::ml::DTrees*>(instance)->getNodes();
			return Ok<void*>(new std::vector<cv::ml::DTrees::Node>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_DTrees_getSplits_const(void* instance) {
		try {
			std::vector<cv::ml::DTrees::Split> ret = reinterpret_cast<cv::ml::DTrees*>(instance)->getSplits();
			return Ok<void*>(new std::vector<cv::ml::DTrees::Split>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_DTrees_getSubsets_const(void* instance) {
		try {
			std::vector<int> ret = reinterpret_cast<cv::ml::DTrees*>(instance)->getSubsets();
			return Ok<void*>(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_DTrees_create() {
		try {
			cv::Ptr<cv::ml::DTrees> ret = cv::ml::DTrees::create();
			return Ok<void*>(new cv::Ptr<cv::ml::DTrees>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_DTrees_load_const_StringX_const_StringX(const char* filepath, const char* nodeName) {
		try {
			cv::Ptr<cv::ml::DTrees> ret = cv::ml::DTrees::load(std::string(filepath), std::string(nodeName));
			return Ok<void*>(new cv::Ptr<cv::ml::DTrees>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_ml_DTrees_Node_value_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ml::DTrees::Node*>(instance)->value;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_DTrees_Node_setValue_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::ml::DTrees::Node*>(instance)->value = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_DTrees_Node_classIdx_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::DTrees::Node*>(instance)->classIdx;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_Node_setClassIdx_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::DTrees::Node*>(instance)->classIdx = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_DTrees_Node_parent_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::DTrees::Node*>(instance)->parent;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_Node_setParent_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::DTrees::Node*>(instance)->parent = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_DTrees_Node_left_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::DTrees::Node*>(instance)->left;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_Node_setLeft_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::DTrees::Node*>(instance)->left = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_DTrees_Node_right_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::DTrees::Node*>(instance)->right;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_Node_setRight_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::DTrees::Node*>(instance)->right = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_DTrees_Node_defaultDir_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::DTrees::Node*>(instance)->defaultDir;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_Node_setDefaultDir_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::DTrees::Node*>(instance)->defaultDir = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_DTrees_Node_split_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::DTrees::Node*>(instance)->split;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_Node_setSplit_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::DTrees::Node*>(instance)->split = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_DTrees_Node_delete(cv::ml::DTrees::Node* instance) {
		delete instance;
	}
	Result<void*> cv_ml_DTrees_Node_Node() {
		try {
			cv::ml::DTrees::Node* ret = new cv::ml::DTrees::Node();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_ml_DTrees_Split_varIdx_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::DTrees::Split*>(instance)->varIdx;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_Split_setVarIdx_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::DTrees::Split*>(instance)->varIdx = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_ml_DTrees_Split_inversed_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ml::DTrees::Split*>(instance)->inversed;
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_ml_DTrees_Split_setInversed_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::ml::DTrees::Split*>(instance)->inversed = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ml_DTrees_Split_quality_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::ml::DTrees::Split*>(instance)->quality;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ml_DTrees_Split_setQuality_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::ml::DTrees::Split*>(instance)->quality = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_DTrees_Split_next_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::DTrees::Split*>(instance)->next;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_Split_setNext_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::DTrees::Split*>(instance)->next = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ml_DTrees_Split_c_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::ml::DTrees::Split*>(instance)->c;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ml_DTrees_Split_setC_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::ml::DTrees::Split*>(instance)->c = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_DTrees_Split_subsetOfs_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::DTrees::Split*>(instance)->subsetOfs;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_DTrees_Split_setSubsetOfs_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::DTrees::Split*>(instance)->subsetOfs = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_DTrees_Split_delete(cv::ml::DTrees::Split* instance) {
		delete instance;
	}
	Result<void*> cv_ml_DTrees_Split_Split() {
		try {
			cv::ml::DTrees::Split* ret = new cv::ml::DTrees::Split();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_ml_EM_getClustersNumber_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::EM*>(instance)->getClustersNumber();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_EM_setClustersNumber_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::EM*>(instance)->setClustersNumber(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_EM_getCovarianceMatrixType_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::EM*>(instance)->getCovarianceMatrixType();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_EM_setCovarianceMatrixType_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::EM*>(instance)->setCovarianceMatrixType(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ml_EM_getTermCriteria_const(void* instance) {
		try {
			cv::TermCriteria ret = reinterpret_cast<cv::ml::EM*>(instance)->getTermCriteria();
			return Ok<void*>(new cv::TermCriteria(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ml_EM_setTermCriteria_const_TermCriteriaX(void* instance, void* val) {
		try {
			reinterpret_cast<cv::ml::EM*>(instance)->setTermCriteria(*reinterpret_cast<const cv::TermCriteria*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ml_EM_getWeights_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::EM*>(instance)->getWeights();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_EM_getMeans_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::EM*>(instance)->getMeans();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ml_EM_getCovs_const_vector_Mat_X(void* instance, void* covs) {
		try {
			reinterpret_cast<cv::ml::EM*>(instance)->getCovs(*reinterpret_cast<std::vector<cv::Mat>*>(covs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ml_EM_predict_const_const__InputArrayX_const__OutputArrayX_int(void* instance, void* samples, void* results, int flags) {
		try {
			float ret = reinterpret_cast<cv::ml::EM*>(instance)->predict(*reinterpret_cast<const cv::_InputArray*>(samples), *reinterpret_cast<const cv::_OutputArray*>(results), flags);
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<cv::Vec2d> cv_ml_EM_predict2_const_const__InputArrayX_const__OutputArrayX(void* instance, void* sample, void* probs) {
		try {
			cv::Vec2d ret = reinterpret_cast<cv::ml::EM*>(instance)->predict2(*reinterpret_cast<const cv::_InputArray*>(sample), *reinterpret_cast<const cv::_OutputArray*>(probs));
			return Ok<cv::Vec2d>(ret);
		} OCVRS_CATCH(Result<cv::Vec2d>)
	}
	
	Result<bool> cv_ml_EM_trainEM_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, void* samples, void* logLikelihoods, void* labels, void* probs) {
		try {
			bool ret = reinterpret_cast<cv::ml::EM*>(instance)->trainEM(*reinterpret_cast<const cv::_InputArray*>(samples), *reinterpret_cast<const cv::_OutputArray*>(logLikelihoods), *reinterpret_cast<const cv::_OutputArray*>(labels), *reinterpret_cast<const cv::_OutputArray*>(probs));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ml_EM_trainE_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, void* samples, void* means0, void* covs0, void* weights0, void* logLikelihoods, void* labels, void* probs) {
		try {
			bool ret = reinterpret_cast<cv::ml::EM*>(instance)->trainE(*reinterpret_cast<const cv::_InputArray*>(samples), *reinterpret_cast<const cv::_InputArray*>(means0), *reinterpret_cast<const cv::_InputArray*>(covs0), *reinterpret_cast<const cv::_InputArray*>(weights0), *reinterpret_cast<const cv::_OutputArray*>(logLikelihoods), *reinterpret_cast<const cv::_OutputArray*>(labels), *reinterpret_cast<const cv::_OutputArray*>(probs));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ml_EM_trainM_const__InputArrayX_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, void* samples, void* probs0, void* logLikelihoods, void* labels, void* probs) {
		try {
			bool ret = reinterpret_cast<cv::ml::EM*>(instance)->trainM(*reinterpret_cast<const cv::_InputArray*>(samples), *reinterpret_cast<const cv::_InputArray*>(probs0), *reinterpret_cast<const cv::_OutputArray*>(logLikelihoods), *reinterpret_cast<const cv::_OutputArray*>(labels), *reinterpret_cast<const cv::_OutputArray*>(probs));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_ml_EM_create() {
		try {
			cv::Ptr<cv::ml::EM> ret = cv::ml::EM::create();
			return Ok<void*>(new cv::Ptr<cv::ml::EM>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_EM_load_const_StringX_const_StringX(const char* filepath, const char* nodeName) {
		try {
			cv::Ptr<cv::ml::EM> ret = cv::ml::EM::load(std::string(filepath), std::string(nodeName));
			return Ok<void*>(new cv::Ptr<cv::ml::EM>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_ml_KNearest_getDefaultK_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::KNearest*>(instance)->getDefaultK();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_KNearest_setDefaultK_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::KNearest*>(instance)->setDefaultK(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_ml_KNearest_getIsClassifier_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ml::KNearest*>(instance)->getIsClassifier();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_ml_KNearest_setIsClassifier_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::ml::KNearest*>(instance)->setIsClassifier(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_KNearest_getEmax_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::KNearest*>(instance)->getEmax();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_KNearest_setEmax_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::KNearest*>(instance)->setEmax(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_KNearest_getAlgorithmType_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::KNearest*>(instance)->getAlgorithmType();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_KNearest_setAlgorithmType_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::KNearest*>(instance)->setAlgorithmType(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ml_KNearest_findNearest_const_const__InputArrayX_int_const__OutputArrayX_const__OutputArrayX_const__OutputArrayX(void* instance, void* samples, int k, void* results, void* neighborResponses, void* dist) {
		try {
			float ret = reinterpret_cast<cv::ml::KNearest*>(instance)->findNearest(*reinterpret_cast<const cv::_InputArray*>(samples), k, *reinterpret_cast<const cv::_OutputArray*>(results), *reinterpret_cast<const cv::_OutputArray*>(neighborResponses), *reinterpret_cast<const cv::_OutputArray*>(dist));
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<void*> cv_ml_KNearest_create() {
		try {
			cv::Ptr<cv::ml::KNearest> ret = cv::ml::KNearest::create();
			return Ok<void*>(new cv::Ptr<cv::ml::KNearest>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_KNearest_load_const_StringX(const char* filepath) {
		try {
			cv::Ptr<cv::ml::KNearest> ret = cv::ml::KNearest::load(std::string(filepath));
			return Ok<void*>(new cv::Ptr<cv::ml::KNearest>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_ml_LogisticRegression_getLearningRate_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ml::LogisticRegression*>(instance)->getLearningRate();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_LogisticRegression_setLearningRate_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::ml::LogisticRegression*>(instance)->setLearningRate(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_LogisticRegression_getIterations_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::LogisticRegression*>(instance)->getIterations();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_LogisticRegression_setIterations_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::LogisticRegression*>(instance)->setIterations(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_LogisticRegression_getRegularization_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::LogisticRegression*>(instance)->getRegularization();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_LogisticRegression_setRegularization_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::LogisticRegression*>(instance)->setRegularization(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_LogisticRegression_getTrainMethod_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::LogisticRegression*>(instance)->getTrainMethod();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_LogisticRegression_setTrainMethod_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::LogisticRegression*>(instance)->setTrainMethod(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_LogisticRegression_getMiniBatchSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::LogisticRegression*>(instance)->getMiniBatchSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_LogisticRegression_setMiniBatchSize_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::LogisticRegression*>(instance)->setMiniBatchSize(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ml_LogisticRegression_getTermCriteria_const(void* instance) {
		try {
			cv::TermCriteria ret = reinterpret_cast<cv::ml::LogisticRegression*>(instance)->getTermCriteria();
			return Ok<void*>(new cv::TermCriteria(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ml_LogisticRegression_setTermCriteria_TermCriteria(void* instance, void* val) {
		try {
			reinterpret_cast<cv::ml::LogisticRegression*>(instance)->setTermCriteria(*reinterpret_cast<cv::TermCriteria*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ml_LogisticRegression_predict_const_const__InputArrayX_const__OutputArrayX_int(void* instance, void* samples, void* results, int flags) {
		try {
			float ret = reinterpret_cast<cv::ml::LogisticRegression*>(instance)->predict(*reinterpret_cast<const cv::_InputArray*>(samples), *reinterpret_cast<const cv::_OutputArray*>(results), flags);
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<void*> cv_ml_LogisticRegression_get_learnt_thetas_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::LogisticRegression*>(instance)->get_learnt_thetas();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_LogisticRegression_create() {
		try {
			cv::Ptr<cv::ml::LogisticRegression> ret = cv::ml::LogisticRegression::create();
			return Ok<void*>(new cv::Ptr<cv::ml::LogisticRegression>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_LogisticRegression_load_const_StringX_const_StringX(const char* filepath, const char* nodeName) {
		try {
			cv::Ptr<cv::ml::LogisticRegression> ret = cv::ml::LogisticRegression::load(std::string(filepath), std::string(nodeName));
			return Ok<void*>(new cv::Ptr<cv::ml::LogisticRegression>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<float> cv_ml_NormalBayesClassifier_predictProb_const_const__InputArrayX_const__OutputArrayX_const__OutputArrayX_int(void* instance, void* inputs, void* outputs, void* outputProbs, int flags) {
		try {
			float ret = reinterpret_cast<cv::ml::NormalBayesClassifier*>(instance)->predictProb(*reinterpret_cast<const cv::_InputArray*>(inputs), *reinterpret_cast<const cv::_OutputArray*>(outputs), *reinterpret_cast<const cv::_OutputArray*>(outputProbs), flags);
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<void*> cv_ml_NormalBayesClassifier_create() {
		try {
			cv::Ptr<cv::ml::NormalBayesClassifier> ret = cv::ml::NormalBayesClassifier::create();
			return Ok<void*>(new cv::Ptr<cv::ml::NormalBayesClassifier>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_NormalBayesClassifier_load_const_StringX_const_StringX(const char* filepath, const char* nodeName) {
		try {
			cv::Ptr<cv::ml::NormalBayesClassifier> ret = cv::ml::NormalBayesClassifier::load(std::string(filepath), std::string(nodeName));
			return Ok<void*>(new cv::Ptr<cv::ml::NormalBayesClassifier>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_ml_ParamGrid_minVal_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ml::ParamGrid*>(instance)->minVal;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ParamGrid_setMinVal_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::ml::ParamGrid*>(instance)->minVal = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ParamGrid_maxVal_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ml::ParamGrid*>(instance)->maxVal;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ParamGrid_setMaxVal_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::ml::ParamGrid*>(instance)->maxVal = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_ParamGrid_logStep_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ml::ParamGrid*>(instance)->logStep;
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_ParamGrid_setLogStep_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::ml::ParamGrid*>(instance)->logStep = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_ParamGrid_delete(cv::ml::ParamGrid* instance) {
		delete instance;
	}
	Result<void*> cv_ml_ParamGrid_ParamGrid() {
		try {
			cv::ml::ParamGrid* ret = new cv::ml::ParamGrid();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_ParamGrid_ParamGrid_double_double_double(double _minVal, double _maxVal, double _logStep) {
		try {
			cv::ml::ParamGrid* ret = new cv::ml::ParamGrid(_minVal, _maxVal, _logStep);
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_ParamGrid_create_double_double_double(double minVal, double maxVal, double logstep) {
		try {
			cv::Ptr<cv::ml::ParamGrid> ret = cv::ml::ParamGrid::create(minVal, maxVal, logstep);
			return Ok<void*>(new cv::Ptr<cv::ml::ParamGrid>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<bool> cv_ml_RTrees_getCalculateVarImportance_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ml::RTrees*>(instance)->getCalculateVarImportance();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_ml_RTrees_setCalculateVarImportance_bool(void* instance, bool val) {
		try {
			reinterpret_cast<cv::ml::RTrees*>(instance)->setCalculateVarImportance(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_RTrees_getActiveVarCount_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::RTrees*>(instance)->getActiveVarCount();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_RTrees_setActiveVarCount_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::RTrees*>(instance)->setActiveVarCount(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ml_RTrees_getTermCriteria_const(void* instance) {
		try {
			cv::TermCriteria ret = reinterpret_cast<cv::ml::RTrees*>(instance)->getTermCriteria();
			return Ok<void*>(new cv::TermCriteria(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ml_RTrees_setTermCriteria_const_TermCriteriaX(void* instance, void* val) {
		try {
			reinterpret_cast<cv::ml::RTrees*>(instance)->setTermCriteria(*reinterpret_cast<const cv::TermCriteria*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ml_RTrees_getVarImportance_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::RTrees*>(instance)->getVarImportance();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ml_RTrees_getVotes_const_const__InputArrayX_const__OutputArrayX_int(void* instance, void* samples, void* results, int flags) {
		try {
			reinterpret_cast<cv::ml::RTrees*>(instance)->getVotes(*reinterpret_cast<const cv::_InputArray*>(samples), *reinterpret_cast<const cv::_OutputArray*>(results), flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ml_RTrees_create() {
		try {
			cv::Ptr<cv::ml::RTrees> ret = cv::ml::RTrees::create();
			return Ok<void*>(new cv::Ptr<cv::ml::RTrees>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_RTrees_load_const_StringX_const_StringX(const char* filepath, const char* nodeName) {
		try {
			cv::Ptr<cv::ml::RTrees> ret = cv::ml::RTrees::load(std::string(filepath), std::string(nodeName));
			return Ok<void*>(new cv::Ptr<cv::ml::RTrees>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_ml_SVM_getType_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::SVM*>(instance)->getType();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_SVM_setType_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::ml::SVM*>(instance)->setType(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_SVM_getGamma_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ml::SVM*>(instance)->getGamma();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_SVM_setGamma_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::ml::SVM*>(instance)->setGamma(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_SVM_getCoef0_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ml::SVM*>(instance)->getCoef0();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_SVM_setCoef0_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::ml::SVM*>(instance)->setCoef0(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_SVM_getDegree_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ml::SVM*>(instance)->getDegree();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_SVM_setDegree_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::ml::SVM*>(instance)->setDegree(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_SVM_getC_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ml::SVM*>(instance)->getC();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_SVM_setC_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::ml::SVM*>(instance)->setC(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_SVM_getNu_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ml::SVM*>(instance)->getNu();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_SVM_setNu_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::ml::SVM*>(instance)->setNu(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<double> cv_ml_SVM_getP_const(void* instance) {
		try {
			double ret = reinterpret_cast<cv::ml::SVM*>(instance)->getP();
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result_void cv_ml_SVM_setP_double(void* instance, double val) {
		try {
			reinterpret_cast<cv::ml::SVM*>(instance)->setP(val);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ml_SVM_getClassWeights_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::SVM*>(instance)->getClassWeights();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ml_SVM_setClassWeights_const_MatX(void* instance, void* val) {
		try {
			reinterpret_cast<cv::ml::SVM*>(instance)->setClassWeights(*reinterpret_cast<const cv::Mat*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ml_SVM_getTermCriteria_const(void* instance) {
		try {
			cv::TermCriteria ret = reinterpret_cast<cv::ml::SVM*>(instance)->getTermCriteria();
			return Ok<void*>(new cv::TermCriteria(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ml_SVM_setTermCriteria_const_TermCriteriaX(void* instance, void* val) {
		try {
			reinterpret_cast<cv::ml::SVM*>(instance)->setTermCriteria(*reinterpret_cast<const cv::TermCriteria*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_SVM_getKernelType_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::SVM*>(instance)->getKernelType();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_SVM_setKernel_int(void* instance, int kernelType) {
		try {
			reinterpret_cast<cv::ml::SVM*>(instance)->setKernel(kernelType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ml_SVM_setCustomKernel_const_Ptr_Kernel_X(void* instance, void* _kernel) {
		try {
			reinterpret_cast<cv::ml::SVM*>(instance)->setCustomKernel(*reinterpret_cast<const cv::Ptr<cv::ml::SVM::Kernel>*>(_kernel));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_ml_SVM_trainAuto_const_Ptr_TrainData_X_int_ParamGrid_ParamGrid_ParamGrid_ParamGrid_ParamGrid_ParamGrid_bool(void* instance, void* data, int kFold, void* Cgrid, void* gammaGrid, void* pGrid, void* nuGrid, void* coeffGrid, void* degreeGrid, bool balanced) {
		try {
			bool ret = reinterpret_cast<cv::ml::SVM*>(instance)->trainAuto(*reinterpret_cast<const cv::Ptr<cv::ml::TrainData>*>(data), kFold, *reinterpret_cast<cv::ml::ParamGrid*>(Cgrid), *reinterpret_cast<cv::ml::ParamGrid*>(gammaGrid), *reinterpret_cast<cv::ml::ParamGrid*>(pGrid), *reinterpret_cast<cv::ml::ParamGrid*>(nuGrid), *reinterpret_cast<cv::ml::ParamGrid*>(coeffGrid), *reinterpret_cast<cv::ml::ParamGrid*>(degreeGrid), balanced);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ml_SVM_trainAuto_const__InputArrayX_int_const__InputArrayX_int_Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__Ptr_ParamGrid__bool(void* instance, void* samples, int layout, void* responses, int kFold, void* Cgrid, void* gammaGrid, void* pGrid, void* nuGrid, void* coeffGrid, void* degreeGrid, bool balanced) {
		try {
			bool ret = reinterpret_cast<cv::ml::SVM*>(instance)->trainAuto(*reinterpret_cast<const cv::_InputArray*>(samples), layout, *reinterpret_cast<const cv::_InputArray*>(responses), kFold, *reinterpret_cast<cv::Ptr<cv::ml::ParamGrid>*>(Cgrid), *reinterpret_cast<cv::Ptr<cv::ml::ParamGrid>*>(gammaGrid), *reinterpret_cast<cv::Ptr<cv::ml::ParamGrid>*>(pGrid), *reinterpret_cast<cv::Ptr<cv::ml::ParamGrid>*>(nuGrid), *reinterpret_cast<cv::Ptr<cv::ml::ParamGrid>*>(coeffGrid), *reinterpret_cast<cv::Ptr<cv::ml::ParamGrid>*>(degreeGrid), balanced);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<void*> cv_ml_SVM_getSupportVectors_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::SVM*>(instance)->getSupportVectors();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_SVM_getUncompressedSupportVectors_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::SVM*>(instance)->getUncompressedSupportVectors();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<double> cv_ml_SVM_getDecisionFunction_const_int_const__OutputArrayX_const__OutputArrayX(void* instance, int i, void* alpha, void* svidx) {
		try {
			double ret = reinterpret_cast<cv::ml::SVM*>(instance)->getDecisionFunction(i, *reinterpret_cast<const cv::_OutputArray*>(alpha), *reinterpret_cast<const cv::_OutputArray*>(svidx));
			return Ok<double>(ret);
		} OCVRS_CATCH(Result<double>)
	}
	
	Result<void*> cv_ml_SVM_getDefaultGrid_int(int param_id) {
		try {
			cv::ml::ParamGrid ret = cv::ml::SVM::getDefaultGrid(param_id);
			return Ok<void*>(new cv::ml::ParamGrid(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_SVM_getDefaultGridPtr_int(int param_id) {
		try {
			cv::Ptr<cv::ml::ParamGrid> ret = cv::ml::SVM::getDefaultGridPtr(param_id);
			return Ok<void*>(new cv::Ptr<cv::ml::ParamGrid>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_SVM_create() {
		try {
			cv::Ptr<cv::ml::SVM> ret = cv::ml::SVM::create();
			return Ok<void*>(new cv::Ptr<cv::ml::SVM>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_SVM_load_const_StringX(const char* filepath) {
		try {
			cv::Ptr<cv::ml::SVM> ret = cv::ml::SVM::load(std::string(filepath));
			return Ok<void*>(new cv::Ptr<cv::ml::SVM>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_ml_SVM_Kernel_getType_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::SVM::Kernel*>(instance)->getType();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_SVM_Kernel_calc_int_int_const_floatX_const_floatX_floatX(void* instance, int vcount, int n, const float* vecs, const float* another, float* results) {
		try {
			reinterpret_cast<cv::ml::SVM::Kernel*>(instance)->calc(vcount, n, vecs, another, results);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ml_SVMSGD_getWeights(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::SVMSGD*>(instance)->getWeights();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<float> cv_ml_SVMSGD_getShift(void* instance) {
		try {
			float ret = reinterpret_cast<cv::ml::SVMSGD*>(instance)->getShift();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<void*> cv_ml_SVMSGD_create() {
		try {
			cv::Ptr<cv::ml::SVMSGD> ret = cv::ml::SVMSGD::create();
			return Ok<void*>(new cv::Ptr<cv::ml::SVMSGD>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_SVMSGD_load_const_StringX_const_StringX(const char* filepath, const char* nodeName) {
		try {
			cv::Ptr<cv::ml::SVMSGD> ret = cv::ml::SVMSGD::load(std::string(filepath), std::string(nodeName));
			return Ok<void*>(new cv::Ptr<cv::ml::SVMSGD>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ml_SVMSGD_setOptimalParameters_int_int(void* instance, int svmsgdType, int marginType) {
		try {
			reinterpret_cast<cv::ml::SVMSGD*>(instance)->setOptimalParameters(svmsgdType, marginType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_SVMSGD_getSvmsgdType_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::SVMSGD*>(instance)->getSvmsgdType();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_SVMSGD_setSvmsgdType_int(void* instance, int svmsgdType) {
		try {
			reinterpret_cast<cv::ml::SVMSGD*>(instance)->setSvmsgdType(svmsgdType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_SVMSGD_getMarginType_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::SVMSGD*>(instance)->getMarginType();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_SVMSGD_setMarginType_int(void* instance, int marginType) {
		try {
			reinterpret_cast<cv::ml::SVMSGD*>(instance)->setMarginType(marginType);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ml_SVMSGD_getMarginRegularization_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::ml::SVMSGD*>(instance)->getMarginRegularization();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ml_SVMSGD_setMarginRegularization_float(void* instance, float marginRegularization) {
		try {
			reinterpret_cast<cv::ml::SVMSGD*>(instance)->setMarginRegularization(marginRegularization);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ml_SVMSGD_getInitialStepSize_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::ml::SVMSGD*>(instance)->getInitialStepSize();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ml_SVMSGD_setInitialStepSize_float(void* instance, float InitialStepSize) {
		try {
			reinterpret_cast<cv::ml::SVMSGD*>(instance)->setInitialStepSize(InitialStepSize);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_ml_SVMSGD_getStepDecreasingPower_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::ml::SVMSGD*>(instance)->getStepDecreasingPower();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_ml_SVMSGD_setStepDecreasingPower_float(void* instance, float stepDecreasingPower) {
		try {
			reinterpret_cast<cv::ml::SVMSGD*>(instance)->setStepDecreasingPower(stepDecreasingPower);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ml_SVMSGD_getTermCriteria_const(void* instance) {
		try {
			cv::TermCriteria ret = reinterpret_cast<cv::ml::SVMSGD*>(instance)->getTermCriteria();
			return Ok<void*>(new cv::TermCriteria(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ml_SVMSGD_setTermCriteria_const_TermCriteriaX(void* instance, void* val) {
		try {
			reinterpret_cast<cv::ml::SVMSGD*>(instance)->setTermCriteria(*reinterpret_cast<const cv::TermCriteria*>(val));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_ml_StatModel_getVarCount_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::StatModel*>(instance)->getVarCount();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<bool> cv_ml_StatModel_empty_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ml::StatModel*>(instance)->empty();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ml_StatModel_isTrained_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ml::StatModel*>(instance)->isTrained();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ml_StatModel_isClassifier_const(void* instance) {
		try {
			bool ret = reinterpret_cast<cv::ml::StatModel*>(instance)->isClassifier();
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ml_StatModel_train_const_Ptr_TrainData_X_int(void* instance, void* trainData, int flags) {
		try {
			bool ret = reinterpret_cast<cv::ml::StatModel*>(instance)->train(*reinterpret_cast<const cv::Ptr<cv::ml::TrainData>*>(trainData), flags);
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_ml_StatModel_train_const__InputArrayX_int_const__InputArrayX(void* instance, void* samples, int layout, void* responses) {
		try {
			bool ret = reinterpret_cast<cv::ml::StatModel*>(instance)->train(*reinterpret_cast<const cv::_InputArray*>(samples), layout, *reinterpret_cast<const cv::_InputArray*>(responses));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<float> cv_ml_StatModel_calcError_const_const_Ptr_TrainData_X_bool_const__OutputArrayX(void* instance, void* data, bool test, void* resp) {
		try {
			float ret = reinterpret_cast<cv::ml::StatModel*>(instance)->calcError(*reinterpret_cast<const cv::Ptr<cv::ml::TrainData>*>(data), test, *reinterpret_cast<const cv::_OutputArray*>(resp));
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<float> cv_ml_StatModel_predict_const_const__InputArrayX_const__OutputArrayX_int(void* instance, void* samples, void* results, int flags) {
		try {
			float ret = reinterpret_cast<cv::ml::StatModel*>(instance)->predict(*reinterpret_cast<const cv::_InputArray*>(samples), *reinterpret_cast<const cv::_OutputArray*>(results), flags);
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<float> cv_ml_TrainData_missingValue() {
		try {
			float ret = cv::ml::TrainData::missingValue();
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result<int> cv_ml_TrainData_getLayout_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getLayout();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ml_TrainData_getNTrainSamples_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getNTrainSamples();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ml_TrainData_getNTestSamples_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getNTestSamples();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ml_TrainData_getNSamples_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getNSamples();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ml_TrainData_getNVars_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getNVars();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_ml_TrainData_getNAllVars_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getNAllVars();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_ml_TrainData_getSample_const_const__InputArrayX_int_floatX(void* instance, void* varIdx, int sidx, float* buf) {
		try {
			reinterpret_cast<cv::ml::TrainData*>(instance)->getSample(*reinterpret_cast<const cv::_InputArray*>(varIdx), sidx, buf);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ml_TrainData_getSamples_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getSamples();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_TrainData_getMissing_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getMissing();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_TrainData_getTrainSamples_const_int_bool_bool(void* instance, int layout, bool compressSamples, bool compressVars) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getTrainSamples(layout, compressSamples, compressVars);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_TrainData_getTrainResponses_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getTrainResponses();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_TrainData_getTrainNormCatResponses_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getTrainNormCatResponses();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_TrainData_getTestResponses_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getTestResponses();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_TrainData_getTestNormCatResponses_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getTestNormCatResponses();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_TrainData_getResponses_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getResponses();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_TrainData_getNormCatResponses_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getNormCatResponses();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_TrainData_getSampleWeights_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getSampleWeights();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_TrainData_getTrainSampleWeights_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getTrainSampleWeights();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_TrainData_getTestSampleWeights_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getTestSampleWeights();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_TrainData_getVarIdx_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getVarIdx();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_TrainData_getVarType_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getVarType();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_TrainData_getVarSymbolFlags_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getVarSymbolFlags();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_ml_TrainData_getResponseType_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getResponseType();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_ml_TrainData_getTrainSampleIdx_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getTrainSampleIdx();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_TrainData_getTestSampleIdx_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getTestSampleIdx();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ml_TrainData_getValues_const_int_const__InputArrayX_floatX(void* instance, int vi, void* sidx, float* values) {
		try {
			reinterpret_cast<cv::ml::TrainData*>(instance)->getValues(vi, *reinterpret_cast<const cv::_InputArray*>(sidx), values);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ml_TrainData_getNormCatValues_const_int_const__InputArrayX_intX(void* instance, int vi, void* sidx, int* values) {
		try {
			reinterpret_cast<cv::ml::TrainData*>(instance)->getNormCatValues(vi, *reinterpret_cast<const cv::_InputArray*>(sidx), values);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ml_TrainData_getDefaultSubstValues_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getDefaultSubstValues();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_ml_TrainData_getCatCount_const_int(void* instance, int vi) {
		try {
			int ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getCatCount(vi);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<void*> cv_ml_TrainData_getClassLabels_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getClassLabels();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_TrainData_getCatOfs_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getCatOfs();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_TrainData_getCatMap_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getCatMap();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ml_TrainData_setTrainTestSplit_int_bool(void* instance, int count, bool shuffle) {
		try {
			reinterpret_cast<cv::ml::TrainData*>(instance)->setTrainTestSplit(count, shuffle);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ml_TrainData_setTrainTestSplitRatio_double_bool(void* instance, double ratio, bool shuffle) {
		try {
			reinterpret_cast<cv::ml::TrainData*>(instance)->setTrainTestSplitRatio(ratio, shuffle);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_ml_TrainData_shuffleTrainTest(void* instance) {
		try {
			reinterpret_cast<cv::ml::TrainData*>(instance)->shuffleTrainTest();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ml_TrainData_getTestSamples_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::ml::TrainData*>(instance)->getTestSamples();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_ml_TrainData_getNames_const_vector_String_X(void* instance, void* names) {
		try {
			reinterpret_cast<cv::ml::TrainData*>(instance)->getNames(*reinterpret_cast<std::vector<cv::String>*>(names));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_ml_TrainData_getSubVector_const_MatX_const_MatX(void* vec, void* idx) {
		try {
			cv::Mat ret = cv::ml::TrainData::getSubVector(*reinterpret_cast<const cv::Mat*>(vec), *reinterpret_cast<const cv::Mat*>(idx));
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_TrainData_getSubMatrix_const_MatX_const_MatX_int(void* matrix, void* idx, int layout) {
		try {
			cv::Mat ret = cv::ml::TrainData::getSubMatrix(*reinterpret_cast<const cv::Mat*>(matrix), *reinterpret_cast<const cv::Mat*>(idx), layout);
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_TrainData_loadFromCSV_const_StringX_int_int_int_const_StringX_char_char(const char* filename, int headerLineCount, int responseStartIdx, int responseEndIdx, const char* varTypeSpec, char delimiter, char missch) {
		try {
			cv::Ptr<cv::ml::TrainData> ret = cv::ml::TrainData::loadFromCSV(std::string(filename), headerLineCount, responseStartIdx, responseEndIdx, std::string(varTypeSpec), delimiter, missch);
			return Ok<void*>(new cv::Ptr<cv::ml::TrainData>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_ml_TrainData_create_const__InputArrayX_int_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX(void* samples, int layout, void* responses, void* varIdx, void* sampleIdx, void* sampleWeights, void* varType) {
		try {
			cv::Ptr<cv::ml::TrainData> ret = cv::ml::TrainData::create(*reinterpret_cast<const cv::_InputArray*>(samples), layout, *reinterpret_cast<const cv::_InputArray*>(responses), *reinterpret_cast<const cv::_InputArray*>(varIdx), *reinterpret_cast<const cv::_InputArray*>(sampleIdx), *reinterpret_cast<const cv::_InputArray*>(sampleWeights), *reinterpret_cast<const cv::_InputArray*>(varType));
			return Ok<void*>(new cv::Ptr<cv::ml::TrainData>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
}
