#include "ocvrs_common.hpp"
#include <opencv2/bgsegm.hpp>
#include "bgsegm_types.hpp"

extern "C" {
	// cv::bgsegm::createBackgroundSubtractorCNT() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:253
	// ("cv::bgsegm::createBackgroundSubtractorCNT", vec![(pred!(mut, [], []), _)]),
	void cv_bgsegm_createBackgroundSubtractorCNT(Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT> ret = cv::bgsegm::createBackgroundSubtractorCNT();
			Ok(new cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createBackgroundSubtractorCNT(int, bool, int, bool)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:253
	// ("cv::bgsegm::createBackgroundSubtractorCNT", vec![(pred!(mut, ["minPixelStability", "useHistory", "maxPixelStability", "isParallel"], ["int", "bool", "int", "bool"]), _)]),
	void cv_bgsegm_createBackgroundSubtractorCNT_int_bool_int_bool(int minPixelStability, bool useHistory, int maxPixelStability, bool isParallel, Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT> ret = cv::bgsegm::createBackgroundSubtractorCNT(minPixelStability, useHistory, maxPixelStability, isParallel);
			Ok(new cv::Ptr<cv::bgsegm::BackgroundSubtractorCNT>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bgsegm::createBackgroundSubtractorGMG() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:198
	// ("cv::bgsegm::createBackgroundSubtractorGMG", vec![(pred!(mut, [], []), _)]),
	void cv_bgsegm_createBackgroundSubtractorGMG(Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG> ret = cv::bgsegm::createBackgroundSubtractorGMG();
			Ok(new cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createBackgroundSubtractorGMG(int, double)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:198
	// ("cv::bgsegm::createBackgroundSubtractorGMG", vec![(pred!(mut, ["initializationFrames", "decisionThreshold"], ["int", "double"]), _)]),
	void cv_bgsegm_createBackgroundSubtractorGMG_int_double(int initializationFrames, double decisionThreshold, Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG> ret = cv::bgsegm::createBackgroundSubtractorGMG(initializationFrames, decisionThreshold);
			Ok(new cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bgsegm::createBackgroundSubtractorGSOC() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:315
	// ("cv::bgsegm::createBackgroundSubtractorGSOC", vec![(pred!(mut, [], []), _)]),
	void cv_bgsegm_createBackgroundSubtractorGSOC(Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC> ret = cv::bgsegm::createBackgroundSubtractorGSOC();
			Ok(new cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createBackgroundSubtractorGSOC(int, int, float, float, int, float, float, float, float, float, float)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:315
	// ("cv::bgsegm::createBackgroundSubtractorGSOC", vec![(pred!(mut, ["mc", "nSamples", "replaceRate", "propagationRate", "hitsThreshold", "alpha", "beta", "blinkingSupressionDecay", "blinkingSupressionMultiplier", "noiseRemovalThresholdFacBG", "noiseRemovalThresholdFacFG"], ["int", "int", "float", "float", "int", "float", "float", "float", "float", "float", "float"]), _)]),
	void cv_bgsegm_createBackgroundSubtractorGSOC_int_int_float_float_int_float_float_float_float_float_float(int mc, int nSamples, float replaceRate, float propagationRate, int hitsThreshold, float alpha, float beta, float blinkingSupressionDecay, float blinkingSupressionMultiplier, float noiseRemovalThresholdFacBG, float noiseRemovalThresholdFacFG, Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC> ret = cv::bgsegm::createBackgroundSubtractorGSOC(mc, nSamples, replaceRate, propagationRate, hitsThreshold, alpha, beta, blinkingSupressionDecay, blinkingSupressionMultiplier, noiseRemovalThresholdFacBG, noiseRemovalThresholdFacFG);
			Ok(new cv::Ptr<cv::bgsegm::BackgroundSubtractorGSOC>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bgsegm::createBackgroundSubtractorLSBP() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:335
	// ("cv::bgsegm::createBackgroundSubtractorLSBP", vec![(pred!(mut, [], []), _)]),
	void cv_bgsegm_createBackgroundSubtractorLSBP(Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP> ret = cv::bgsegm::createBackgroundSubtractorLSBP();
			Ok(new cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createBackgroundSubtractorLSBP(int, int, int, float, float, float, float, float, float, float, float, int, int)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:335
	// ("cv::bgsegm::createBackgroundSubtractorLSBP", vec![(pred!(mut, ["mc", "nSamples", "LSBPRadius", "Tlower", "Tupper", "Tinc", "Tdec", "Rscale", "Rincdec", "noiseRemovalThresholdFacBG", "noiseRemovalThresholdFacFG", "LSBPthreshold", "minCount"], ["int", "int", "int", "float", "float", "float", "float", "float", "float", "float", "float", "int", "int"]), _)]),
	void cv_bgsegm_createBackgroundSubtractorLSBP_int_int_int_float_float_float_float_float_float_float_float_int_int(int mc, int nSamples, int LSBPRadius, float Tlower, float Tupper, float Tinc, float Tdec, float Rscale, float Rincdec, float noiseRemovalThresholdFacBG, float noiseRemovalThresholdFacFG, int LSBPthreshold, int minCount, Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP> ret = cv::bgsegm::createBackgroundSubtractorLSBP(mc, nSamples, LSBPRadius, Tlower, Tupper, Tinc, Tdec, Rscale, Rincdec, noiseRemovalThresholdFacBG, noiseRemovalThresholdFacFG, LSBPthreshold, minCount);
			Ok(new cv::Ptr<cv::bgsegm::BackgroundSubtractorLSBP>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bgsegm::createBackgroundSubtractorMOG() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:87
	// ("cv::bgsegm::createBackgroundSubtractorMOG", vec![(pred!(mut, [], []), _)]),
	void cv_bgsegm_createBackgroundSubtractorMOG(Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG> ret = cv::bgsegm::createBackgroundSubtractorMOG();
			Ok(new cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createBackgroundSubtractorMOG(int, int, double, double)(Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:87
	// ("cv::bgsegm::createBackgroundSubtractorMOG", vec![(pred!(mut, ["history", "nmixtures", "backgroundRatio", "noiseSigma"], ["int", "int", "double", "double"]), _)]),
	void cv_bgsegm_createBackgroundSubtractorMOG_int_int_double_double(int history, int nmixtures, double backgroundRatio, double noiseSigma, Result<cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG> ret = cv::bgsegm::createBackgroundSubtractorMOG(history, nmixtures, backgroundRatio, noiseSigma);
			Ok(new cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bgsegm::createSyntheticSequenceGenerator(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:385
	// ("cv::bgsegm::createSyntheticSequenceGenerator", vec![(pred!(mut, ["background", "object"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
	void cv_bgsegm_createSyntheticSequenceGenerator_const__InputArrayR_const__InputArrayR(const cv::_InputArray* background, const cv::_InputArray* object, Result<cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator> ret = cv::bgsegm::createSyntheticSequenceGenerator(*background, *object);
			Ok(new cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createSyntheticSequenceGenerator(InputArray, InputArray, double, double, double, double)(InputArray, InputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:385
	// ("cv::bgsegm::createSyntheticSequenceGenerator", vec![(pred!(mut, ["background", "object", "amplitude", "wavelength", "wavespeed", "objspeed"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "double", "double", "double"]), _)]),
	void cv_bgsegm_createSyntheticSequenceGenerator_const__InputArrayR_const__InputArrayR_double_double_double_double(const cv::_InputArray* background, const cv::_InputArray* object, double amplitude, double wavelength, double wavespeed, double objspeed, Result<cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator> ret = cv::bgsegm::createSyntheticSequenceGenerator(*background, *object, amplitude, wavelength, wavespeed, objspeed);
			Ok(new cv::Ptr<cv::bgsegm::SyntheticSequenceGenerator>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// apply(InputArray, OutputArray, double)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:212
	// ("cv::bgsegm::BackgroundSubtractorCNT::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double"]), _)]),
	void cv_bgsegm_BackgroundSubtractorCNT_apply_const__InputArrayR_const__OutputArrayR_double(cv::bgsegm::BackgroundSubtractorCNT* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, double learningRate, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask, learningRate);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bgsegm::BackgroundSubtractorCNT::apply(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:212
	// ("cv::bgsegm::BackgroundSubtractorCNT::apply", vec![(pred!(mut, ["image", "fgmask"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_bgsegm_BackgroundSubtractorCNT_apply_const__InputArrayR_const__OutputArrayR(cv::bgsegm::BackgroundSubtractorCNT* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackgroundImage(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:213
	// ("cv::bgsegm::BackgroundSubtractorCNT::getBackgroundImage", vec![(pred!(const, ["backgroundImage"], ["const cv::_OutputArray*"]), _)]),
	void cv_bgsegm_BackgroundSubtractorCNT_getBackgroundImage_const_const__OutputArrayR(const cv::bgsegm::BackgroundSubtractorCNT* instance, const cv::_OutputArray* backgroundImage, ResultVoid* ocvrs_return) {
		try {
			instance->getBackgroundImage(*backgroundImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinPixelStability()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:217
	// ("cv::bgsegm::BackgroundSubtractorCNT::getMinPixelStability", vec![(pred!(const, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorCNT_getMinPixelStability_const(const cv::bgsegm::BackgroundSubtractorCNT* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinPixelStability();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinPixelStability(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:220
	// ("cv::bgsegm::BackgroundSubtractorCNT::setMinPixelStability", vec![(pred!(mut, ["value"], ["int"]), _)]),
	void cv_bgsegm_BackgroundSubtractorCNT_setMinPixelStability_int(cv::bgsegm::BackgroundSubtractorCNT* instance, int value, ResultVoid* ocvrs_return) {
		try {
			instance->setMinPixelStability(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxPixelStability()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:224
	// ("cv::bgsegm::BackgroundSubtractorCNT::getMaxPixelStability", vec![(pred!(const, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorCNT_getMaxPixelStability_const(const cv::bgsegm::BackgroundSubtractorCNT* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxPixelStability();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxPixelStability(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:227
	// ("cv::bgsegm::BackgroundSubtractorCNT::setMaxPixelStability", vec![(pred!(mut, ["value"], ["int"]), _)]),
	void cv_bgsegm_BackgroundSubtractorCNT_setMaxPixelStability_int(cv::bgsegm::BackgroundSubtractorCNT* instance, int value, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxPixelStability(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUseHistory()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:231
	// ("cv::bgsegm::BackgroundSubtractorCNT::getUseHistory", vec![(pred!(const, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorCNT_getUseHistory_const(const cv::bgsegm::BackgroundSubtractorCNT* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseHistory();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUseHistory(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:234
	// ("cv::bgsegm::BackgroundSubtractorCNT::setUseHistory", vec![(pred!(mut, ["value"], ["bool"]), _)]),
	void cv_bgsegm_BackgroundSubtractorCNT_setUseHistory_bool(cv::bgsegm::BackgroundSubtractorCNT* instance, bool value, ResultVoid* ocvrs_return) {
		try {
			instance->setUseHistory(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getIsParallel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:238
	// ("cv::bgsegm::BackgroundSubtractorCNT::getIsParallel", vec![(pred!(const, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorCNT_getIsParallel_const(const cv::bgsegm::BackgroundSubtractorCNT* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getIsParallel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setIsParallel(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:241
	// ("cv::bgsegm::BackgroundSubtractorCNT::setIsParallel", vec![(pred!(mut, ["value"], ["bool"]), _)]),
	void cv_bgsegm_BackgroundSubtractorCNT_setIsParallel_bool(cv::bgsegm::BackgroundSubtractorCNT* instance, bool value, ResultVoid* ocvrs_return) {
		try {
			instance->setIsParallel(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bgsegm::BackgroundSubtractorCNT::to_Algorithm() generated
	// ("cv::bgsegm::BackgroundSubtractorCNT::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_bgsegm_BackgroundSubtractorCNT_to_Algorithm(cv::bgsegm::BackgroundSubtractorCNT* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::bgsegm::BackgroundSubtractorCNT::to_BackgroundSubtractor() generated
	// ("cv::bgsegm::BackgroundSubtractorCNT::to_BackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
	cv::BackgroundSubtractor* cv_bgsegm_BackgroundSubtractorCNT_to_BackgroundSubtractor(cv::bgsegm::BackgroundSubtractorCNT* instance) {
			return dynamic_cast<cv::BackgroundSubtractor*>(instance);
	}

	// cv::bgsegm::BackgroundSubtractorCNT::delete() generated
	// ("cv::bgsegm::BackgroundSubtractorCNT::delete", vec![(pred!(mut, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorCNT_delete(cv::bgsegm::BackgroundSubtractorCNT* instance) {
			delete instance;
	}

	// apply(InputArray, OutputArray, double)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:112
	// ("cv::bgsegm::BackgroundSubtractorGMG::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double"]), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_apply_const__InputArrayR_const__OutputArrayR_double(cv::bgsegm::BackgroundSubtractorGMG* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, double learningRate, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask, learningRate);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bgsegm::BackgroundSubtractorGMG::apply(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:112
	// ("cv::bgsegm::BackgroundSubtractorGMG::apply", vec![(pred!(mut, ["image", "fgmask"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_apply_const__InputArrayR_const__OutputArrayR(cv::bgsegm::BackgroundSubtractorGMG* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackgroundImage(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:113
	// ("cv::bgsegm::BackgroundSubtractorGMG::getBackgroundImage", vec![(pred!(const, ["backgroundImage"], ["const cv::_OutputArray*"]), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_getBackgroundImage_const_const__OutputArrayR(const cv::bgsegm::BackgroundSubtractorGMG* instance, const cv::_OutputArray* backgroundImage, ResultVoid* ocvrs_return) {
		try {
			instance->getBackgroundImage(*backgroundImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxFeatures()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:117
	// ("cv::bgsegm::BackgroundSubtractorGMG::getMaxFeatures", vec![(pred!(const, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_getMaxFeatures_const(const cv::bgsegm::BackgroundSubtractorGMG* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxFeatures();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:120
	// ("cv::bgsegm::BackgroundSubtractorGMG::setMaxFeatures", vec![(pred!(mut, ["maxFeatures"], ["int"]), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_setMaxFeatures_int(cv::bgsegm::BackgroundSubtractorGMG* instance, int maxFeatures, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxFeatures(maxFeatures);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDefaultLearningRate()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:127
	// ("cv::bgsegm::BackgroundSubtractorGMG::getDefaultLearningRate", vec![(pred!(const, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_getDefaultLearningRate_const(const cv::bgsegm::BackgroundSubtractorGMG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDefaultLearningRate();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDefaultLearningRate(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:130
	// ("cv::bgsegm::BackgroundSubtractorGMG::setDefaultLearningRate", vec![(pred!(mut, ["lr"], ["double"]), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_setDefaultLearningRate_double(cv::bgsegm::BackgroundSubtractorGMG* instance, double lr, ResultVoid* ocvrs_return) {
		try {
			instance->setDefaultLearningRate(lr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNumFrames()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:134
	// ("cv::bgsegm::BackgroundSubtractorGMG::getNumFrames", vec![(pred!(const, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_getNumFrames_const(const cv::bgsegm::BackgroundSubtractorGMG* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumFrames();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNumFrames(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:137
	// ("cv::bgsegm::BackgroundSubtractorGMG::setNumFrames", vec![(pred!(mut, ["nframes"], ["int"]), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_setNumFrames_int(cv::bgsegm::BackgroundSubtractorGMG* instance, int nframes, ResultVoid* ocvrs_return) {
		try {
			instance->setNumFrames(nframes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getQuantizationLevels()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:143
	// ("cv::bgsegm::BackgroundSubtractorGMG::getQuantizationLevels", vec![(pred!(const, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_getQuantizationLevels_const(const cv::bgsegm::BackgroundSubtractorGMG* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getQuantizationLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setQuantizationLevels(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:146
	// ("cv::bgsegm::BackgroundSubtractorGMG::setQuantizationLevels", vec![(pred!(mut, ["nlevels"], ["int"]), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_setQuantizationLevels_int(cv::bgsegm::BackgroundSubtractorGMG* instance, int nlevels, ResultVoid* ocvrs_return) {
		try {
			instance->setQuantizationLevels(nlevels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackgroundPrior()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:150
	// ("cv::bgsegm::BackgroundSubtractorGMG::getBackgroundPrior", vec![(pred!(const, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_getBackgroundPrior_const(const cv::bgsegm::BackgroundSubtractorGMG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getBackgroundPrior();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBackgroundPrior(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:153
	// ("cv::bgsegm::BackgroundSubtractorGMG::setBackgroundPrior", vec![(pred!(mut, ["bgprior"], ["double"]), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_setBackgroundPrior_double(cv::bgsegm::BackgroundSubtractorGMG* instance, double bgprior, ResultVoid* ocvrs_return) {
		try {
			instance->setBackgroundPrior(bgprior);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSmoothingRadius()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:157
	// ("cv::bgsegm::BackgroundSubtractorGMG::getSmoothingRadius", vec![(pred!(const, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_getSmoothingRadius_const(const cv::bgsegm::BackgroundSubtractorGMG* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSmoothingRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSmoothingRadius(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:160
	// ("cv::bgsegm::BackgroundSubtractorGMG::setSmoothingRadius", vec![(pred!(mut, ["radius"], ["int"]), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_setSmoothingRadius_int(cv::bgsegm::BackgroundSubtractorGMG* instance, int radius, ResultVoid* ocvrs_return) {
		try {
			instance->setSmoothingRadius(radius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getDecisionThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:166
	// ("cv::bgsegm::BackgroundSubtractorGMG::getDecisionThreshold", vec![(pred!(const, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_getDecisionThreshold_const(const cv::bgsegm::BackgroundSubtractorGMG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDecisionThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setDecisionThreshold(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:169
	// ("cv::bgsegm::BackgroundSubtractorGMG::setDecisionThreshold", vec![(pred!(mut, ["thresh"], ["double"]), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_setDecisionThreshold_double(cv::bgsegm::BackgroundSubtractorGMG* instance, double thresh, ResultVoid* ocvrs_return) {
		try {
			instance->setDecisionThreshold(thresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getUpdateBackgroundModel()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:173
	// ("cv::bgsegm::BackgroundSubtractorGMG::getUpdateBackgroundModel", vec![(pred!(const, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_getUpdateBackgroundModel_const(const cv::bgsegm::BackgroundSubtractorGMG* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUpdateBackgroundModel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setUpdateBackgroundModel(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:176
	// ("cv::bgsegm::BackgroundSubtractorGMG::setUpdateBackgroundModel", vec![(pred!(mut, ["update"], ["bool"]), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_setUpdateBackgroundModel_bool(cv::bgsegm::BackgroundSubtractorGMG* instance, bool update, ResultVoid* ocvrs_return) {
		try {
			instance->setUpdateBackgroundModel(update);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMinVal()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:180
	// ("cv::bgsegm::BackgroundSubtractorGMG::getMinVal", vec![(pred!(const, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_getMinVal_const(const cv::bgsegm::BackgroundSubtractorGMG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinVal();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMinVal(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:183
	// ("cv::bgsegm::BackgroundSubtractorGMG::setMinVal", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_setMinVal_double(cv::bgsegm::BackgroundSubtractorGMG* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMinVal(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMaxVal()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:187
	// ("cv::bgsegm::BackgroundSubtractorGMG::getMaxVal", vec![(pred!(const, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_getMaxVal_const(const cv::bgsegm::BackgroundSubtractorGMG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxVal();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setMaxVal(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:190
	// ("cv::bgsegm::BackgroundSubtractorGMG::setMaxVal", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_setMaxVal_double(cv::bgsegm::BackgroundSubtractorGMG* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxVal(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bgsegm::BackgroundSubtractorGMG::to_Algorithm() generated
	// ("cv::bgsegm::BackgroundSubtractorGMG::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_bgsegm_BackgroundSubtractorGMG_to_Algorithm(cv::bgsegm::BackgroundSubtractorGMG* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::bgsegm::BackgroundSubtractorGMG::to_BackgroundSubtractor() generated
	// ("cv::bgsegm::BackgroundSubtractorGMG::to_BackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
	cv::BackgroundSubtractor* cv_bgsegm_BackgroundSubtractorGMG_to_BackgroundSubtractor(cv::bgsegm::BackgroundSubtractorGMG* instance) {
			return dynamic_cast<cv::BackgroundSubtractor*>(instance);
	}

	// cv::bgsegm::BackgroundSubtractorGMG::delete() generated
	// ("cv::bgsegm::BackgroundSubtractorGMG::delete", vec![(pred!(mut, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorGMG_delete(cv::bgsegm::BackgroundSubtractorGMG* instance) {
			delete instance;
	}

	// apply(InputArray, OutputArray, double)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:271
	// ("cv::bgsegm::BackgroundSubtractorGSOC::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double"]), _)]),
	void cv_bgsegm_BackgroundSubtractorGSOC_apply_const__InputArrayR_const__OutputArrayR_double(cv::bgsegm::BackgroundSubtractorGSOC* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, double learningRate, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask, learningRate);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bgsegm::BackgroundSubtractorGSOC::apply(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:271
	// ("cv::bgsegm::BackgroundSubtractorGSOC::apply", vec![(pred!(mut, ["image", "fgmask"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_bgsegm_BackgroundSubtractorGSOC_apply_const__InputArrayR_const__OutputArrayR(cv::bgsegm::BackgroundSubtractorGSOC* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackgroundImage(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:273
	// ("cv::bgsegm::BackgroundSubtractorGSOC::getBackgroundImage", vec![(pred!(const, ["backgroundImage"], ["const cv::_OutputArray*"]), _)]),
	void cv_bgsegm_BackgroundSubtractorGSOC_getBackgroundImage_const_const__OutputArrayR(const cv::bgsegm::BackgroundSubtractorGSOC* instance, const cv::_OutputArray* backgroundImage, ResultVoid* ocvrs_return) {
		try {
			instance->getBackgroundImage(*backgroundImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bgsegm::BackgroundSubtractorGSOC::to_Algorithm() generated
	// ("cv::bgsegm::BackgroundSubtractorGSOC::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_bgsegm_BackgroundSubtractorGSOC_to_Algorithm(cv::bgsegm::BackgroundSubtractorGSOC* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::bgsegm::BackgroundSubtractorGSOC::to_BackgroundSubtractor() generated
	// ("cv::bgsegm::BackgroundSubtractorGSOC::to_BackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
	cv::BackgroundSubtractor* cv_bgsegm_BackgroundSubtractorGSOC_to_BackgroundSubtractor(cv::bgsegm::BackgroundSubtractorGSOC* instance) {
			return dynamic_cast<cv::BackgroundSubtractor*>(instance);
	}

	// cv::bgsegm::BackgroundSubtractorGSOC::delete() generated
	// ("cv::bgsegm::BackgroundSubtractorGSOC::delete", vec![(pred!(mut, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorGSOC_delete(cv::bgsegm::BackgroundSubtractorGSOC* instance) {
			delete instance;
	}

	// apply(InputArray, OutputArray, double)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:282
	// ("cv::bgsegm::BackgroundSubtractorLSBP::apply", vec![(pred!(mut, ["image", "fgmask", "learningRate"], ["const cv::_InputArray*", "const cv::_OutputArray*", "double"]), _)]),
	void cv_bgsegm_BackgroundSubtractorLSBP_apply_const__InputArrayR_const__OutputArrayR_double(cv::bgsegm::BackgroundSubtractorLSBP* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, double learningRate, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask, learningRate);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bgsegm::BackgroundSubtractorLSBP::apply(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:282
	// ("cv::bgsegm::BackgroundSubtractorLSBP::apply", vec![(pred!(mut, ["image", "fgmask"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_bgsegm_BackgroundSubtractorLSBP_apply_const__InputArrayR_const__OutputArrayR(cv::bgsegm::BackgroundSubtractorLSBP* instance, const cv::_InputArray* image, const cv::_OutputArray* fgmask, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*image, *fgmask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackgroundImage(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:284
	// ("cv::bgsegm::BackgroundSubtractorLSBP::getBackgroundImage", vec![(pred!(const, ["backgroundImage"], ["const cv::_OutputArray*"]), _)]),
	void cv_bgsegm_BackgroundSubtractorLSBP_getBackgroundImage_const_const__OutputArrayR(const cv::bgsegm::BackgroundSubtractorLSBP* instance, const cv::_OutputArray* backgroundImage, ResultVoid* ocvrs_return) {
		try {
			instance->getBackgroundImage(*backgroundImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bgsegm::BackgroundSubtractorLSBP::to_Algorithm() generated
	// ("cv::bgsegm::BackgroundSubtractorLSBP::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_bgsegm_BackgroundSubtractorLSBP_to_Algorithm(cv::bgsegm::BackgroundSubtractorLSBP* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::bgsegm::BackgroundSubtractorLSBP::to_BackgroundSubtractor() generated
	// ("cv::bgsegm::BackgroundSubtractorLSBP::to_BackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
	cv::BackgroundSubtractor* cv_bgsegm_BackgroundSubtractorLSBP_to_BackgroundSubtractor(cv::bgsegm::BackgroundSubtractorLSBP* instance) {
			return dynamic_cast<cv::BackgroundSubtractor*>(instance);
	}

	// cv::bgsegm::BackgroundSubtractorLSBP::delete() generated
	// ("cv::bgsegm::BackgroundSubtractorLSBP::delete", vec![(pred!(mut, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorLSBP_delete(cv::bgsegm::BackgroundSubtractorLSBP* instance) {
			delete instance;
	}

	// calcLocalSVDValues(OutputArray, const Mat &)(OutputArray, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:292
	// ("cv::bgsegm::BackgroundSubtractorLSBPDesc::calcLocalSVDValues", vec![(pred!(mut, ["localSVDValues", "frame"], ["const cv::_OutputArray*", "const cv::Mat*"]), _)]),
	void cv_bgsegm_BackgroundSubtractorLSBPDesc_calcLocalSVDValues_const__OutputArrayR_const_MatR(const cv::_OutputArray* localSVDValues, const cv::Mat* frame, ResultVoid* ocvrs_return) {
		try {
			cv::bgsegm::BackgroundSubtractorLSBPDesc::calcLocalSVDValues(*localSVDValues, *frame);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeFromLocalSVDValues(OutputArray, const Mat &, const Point2i *)(OutputArray, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:294
	// ("cv::bgsegm::BackgroundSubtractorLSBPDesc::computeFromLocalSVDValues", vec![(pred!(mut, ["desc", "localSVDValues", "LSBPSamplePoints"], ["const cv::_OutputArray*", "const cv::Mat*", "const cv::Point2i*"]), _)]),
	void cv_bgsegm_BackgroundSubtractorLSBPDesc_computeFromLocalSVDValues_const__OutputArrayR_const_MatR_const_Point2iX(const cv::_OutputArray* desc, const cv::Mat* localSVDValues, const cv::Point2i* LSBPSamplePoints, ResultVoid* ocvrs_return) {
		try {
			cv::bgsegm::BackgroundSubtractorLSBPDesc::computeFromLocalSVDValues(*desc, *localSVDValues, LSBPSamplePoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// compute(OutputArray, const Mat &, const Point2i *)(OutputArray, TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:296
	// ("cv::bgsegm::BackgroundSubtractorLSBPDesc::compute", vec![(pred!(mut, ["desc", "frame", "LSBPSamplePoints"], ["const cv::_OutputArray*", "const cv::Mat*", "const cv::Point2i*"]), _)]),
	void cv_bgsegm_BackgroundSubtractorLSBPDesc_compute_const__OutputArrayR_const_MatR_const_Point2iX(const cv::_OutputArray* desc, const cv::Mat* frame, const cv::Point2i* LSBPSamplePoints, ResultVoid* ocvrs_return) {
		try {
			cv::bgsegm::BackgroundSubtractorLSBPDesc::compute(*desc, *frame, LSBPSamplePoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bgsegm::BackgroundSubtractorLSBPDesc::defaultNew() generated
	// ("cv::bgsegm::BackgroundSubtractorLSBPDesc::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::bgsegm::BackgroundSubtractorLSBPDesc* cv_bgsegm_BackgroundSubtractorLSBPDesc_defaultNew_const() {
			cv::bgsegm::BackgroundSubtractorLSBPDesc* ret = new cv::bgsegm::BackgroundSubtractorLSBPDesc();
			return ret;
	}

	// cv::bgsegm::BackgroundSubtractorLSBPDesc::delete() generated
	// ("cv::bgsegm::BackgroundSubtractorLSBPDesc::delete", vec![(pred!(mut, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorLSBPDesc_delete(cv::bgsegm::BackgroundSubtractorLSBPDesc* instance) {
			delete instance;
	}

	// getHistory()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:65
	// ("cv::bgsegm::BackgroundSubtractorMOG::getHistory", vec![(pred!(const, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorMOG_getHistory_const(const cv::bgsegm::BackgroundSubtractorMOG* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getHistory();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setHistory(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:66
	// ("cv::bgsegm::BackgroundSubtractorMOG::setHistory", vec![(pred!(mut, ["nframes"], ["int"]), _)]),
	void cv_bgsegm_BackgroundSubtractorMOG_setHistory_int(cv::bgsegm::BackgroundSubtractorMOG* instance, int nframes, ResultVoid* ocvrs_return) {
		try {
			instance->setHistory(nframes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNMixtures()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:68
	// ("cv::bgsegm::BackgroundSubtractorMOG::getNMixtures", vec![(pred!(const, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorMOG_getNMixtures_const(const cv::bgsegm::BackgroundSubtractorMOG* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNMixtures();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNMixtures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:69
	// ("cv::bgsegm::BackgroundSubtractorMOG::setNMixtures", vec![(pred!(mut, ["nmix"], ["int"]), _)]),
	void cv_bgsegm_BackgroundSubtractorMOG_setNMixtures_int(cv::bgsegm::BackgroundSubtractorMOG* instance, int nmix, ResultVoid* ocvrs_return) {
		try {
			instance->setNMixtures(nmix);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBackgroundRatio()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:71
	// ("cv::bgsegm::BackgroundSubtractorMOG::getBackgroundRatio", vec![(pred!(const, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorMOG_getBackgroundRatio_const(const cv::bgsegm::BackgroundSubtractorMOG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getBackgroundRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBackgroundRatio(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:72
	// ("cv::bgsegm::BackgroundSubtractorMOG::setBackgroundRatio", vec![(pred!(mut, ["backgroundRatio"], ["double"]), _)]),
	void cv_bgsegm_BackgroundSubtractorMOG_setBackgroundRatio_double(cv::bgsegm::BackgroundSubtractorMOG* instance, double backgroundRatio, ResultVoid* ocvrs_return) {
		try {
			instance->setBackgroundRatio(backgroundRatio);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNoiseSigma()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:74
	// ("cv::bgsegm::BackgroundSubtractorMOG::getNoiseSigma", vec![(pred!(const, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorMOG_getNoiseSigma_const(const cv::bgsegm::BackgroundSubtractorMOG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getNoiseSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNoiseSigma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:75
	// ("cv::bgsegm::BackgroundSubtractorMOG::setNoiseSigma", vec![(pred!(mut, ["noiseSigma"], ["double"]), _)]),
	void cv_bgsegm_BackgroundSubtractorMOG_setNoiseSigma_double(cv::bgsegm::BackgroundSubtractorMOG* instance, double noiseSigma, ResultVoid* ocvrs_return) {
		try {
			instance->setNoiseSigma(noiseSigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bgsegm::BackgroundSubtractorMOG::to_Algorithm() generated
	// ("cv::bgsegm::BackgroundSubtractorMOG::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_bgsegm_BackgroundSubtractorMOG_to_Algorithm(cv::bgsegm::BackgroundSubtractorMOG* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::bgsegm::BackgroundSubtractorMOG::to_BackgroundSubtractor() generated
	// ("cv::bgsegm::BackgroundSubtractorMOG::to_BackgroundSubtractor", vec![(pred!(mut, [], []), _)]),
	cv::BackgroundSubtractor* cv_bgsegm_BackgroundSubtractorMOG_to_BackgroundSubtractor(cv::bgsegm::BackgroundSubtractorMOG* instance) {
			return dynamic_cast<cv::BackgroundSubtractor*>(instance);
	}

	// cv::bgsegm::BackgroundSubtractorMOG::delete() generated
	// ("cv::bgsegm::BackgroundSubtractorMOG::delete", vec![(pred!(mut, [], []), _)]),
	void cv_bgsegm_BackgroundSubtractorMOG_delete(cv::bgsegm::BackgroundSubtractorMOG* instance) {
			delete instance;
	}

	// SyntheticSequenceGenerator(InputArray, InputArray, double, double, double, double)(InputArray, InputArray, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:366
	// ("cv::bgsegm::SyntheticSequenceGenerator::SyntheticSequenceGenerator", vec![(pred!(mut, ["background", "object", "amplitude", "wavelength", "wavespeed", "objspeed"], ["const cv::_InputArray*", "const cv::_InputArray*", "double", "double", "double", "double"]), _)]),
	void cv_bgsegm_SyntheticSequenceGenerator_SyntheticSequenceGenerator_const__InputArrayR_const__InputArrayR_double_double_double_double(const cv::_InputArray* background, const cv::_InputArray* object, double amplitude, double wavelength, double wavespeed, double objspeed, Result<cv::bgsegm::SyntheticSequenceGenerator*>* ocvrs_return) {
		try {
			cv::bgsegm::SyntheticSequenceGenerator* ret = new cv::bgsegm::SyntheticSequenceGenerator(*background, *object, amplitude, wavelength, wavespeed, objspeed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNextFrame(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bgsegm.hpp:373
	// ("cv::bgsegm::SyntheticSequenceGenerator::getNextFrame", vec![(pred!(mut, ["frame", "gtMask"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_bgsegm_SyntheticSequenceGenerator_getNextFrame_const__OutputArrayR_const__OutputArrayR(cv::bgsegm::SyntheticSequenceGenerator* instance, const cv::_OutputArray* frame, const cv::_OutputArray* gtMask, ResultVoid* ocvrs_return) {
		try {
			instance->getNextFrame(*frame, *gtMask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bgsegm::SyntheticSequenceGenerator::to_Algorithm() generated
	// ("cv::bgsegm::SyntheticSequenceGenerator::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_bgsegm_SyntheticSequenceGenerator_to_Algorithm(cv::bgsegm::SyntheticSequenceGenerator* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::bgsegm::SyntheticSequenceGenerator::delete() generated
	// ("cv::bgsegm::SyntheticSequenceGenerator::delete", vec![(pred!(mut, [], []), _)]),
	void cv_bgsegm_SyntheticSequenceGenerator_delete(cv::bgsegm::SyntheticSequenceGenerator* instance) {
			delete instance;
	}

}
