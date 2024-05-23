#include "ocvrs_common.hpp"
#include <opencv2/xphoto.hpp>
#include "xphoto_types.hpp"

extern "C" {
	// applyChannelGains(InputArray, OutputArray, float, float, float)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:225
	// ("cv::xphoto::applyChannelGains", vec![(pred!(mut, ["src", "dst", "gainB", "gainG", "gainR"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "float", "float"]), _)]),
	void cv_xphoto_applyChannelGains_const__InputArrayR_const__OutputArrayR_float_float_float(const cv::_InputArray* src, const cv::_OutputArray* dst, float gainB, float gainG, float gainR, ResultVoid* ocvrs_return) {
		try {
			cv::xphoto::applyChannelGains(*src, *dst, gainB, gainG, gainR);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xphoto::bm3dDenoising(InputArray, InputOutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/bm3d_image_denoising.hpp:115
	// ("cv::xphoto::bm3dDenoising", vec![(pred!(mut, ["src", "dstStep1", "dstStep2"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_xphoto_bm3dDenoising_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_InputOutputArray* dstStep1, const cv::_OutputArray* dstStep2, ResultVoid* ocvrs_return) {
		try {
			cv::xphoto::bm3dDenoising(*src, *dstStep1, *dstStep2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bm3dDenoising(InputArray, InputOutputArray, OutputArray, float, int, int, int, int, int, int, float, int, int, int)(InputArray, InputOutputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/bm3d_image_denoising.hpp:115
	// ("cv::xphoto::bm3dDenoising", vec![(pred!(mut, ["src", "dstStep1", "dstStep2", "h", "templateWindowSize", "searchWindowSize", "blockMatchingStep1", "blockMatchingStep2", "groupSize", "slidingStep", "beta", "normType", "step", "transformType"], ["const cv::_InputArray*", "const cv::_InputOutputArray*", "const cv::_OutputArray*", "float", "int", "int", "int", "int", "int", "int", "float", "int", "int", "int"]), _)]),
	void cv_xphoto_bm3dDenoising_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_float_int_int_int_int_int_int_float_int_int_int(const cv::_InputArray* src, const cv::_InputOutputArray* dstStep1, const cv::_OutputArray* dstStep2, float h, int templateWindowSize, int searchWindowSize, int blockMatchingStep1, int blockMatchingStep2, int groupSize, int slidingStep, float beta, int normType, int step, int transformType, ResultVoid* ocvrs_return) {
		try {
			cv::xphoto::bm3dDenoising(*src, *dstStep1, *dstStep2, h, templateWindowSize, searchWindowSize, blockMatchingStep1, blockMatchingStep2, groupSize, slidingStep, beta, normType, step, transformType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xphoto::bm3dDenoising(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/bm3d_image_denoising.hpp:168
	// ("cv::xphoto::bm3dDenoising", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_xphoto_bm3dDenoising_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			cv::xphoto::bm3dDenoising(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// bm3dDenoising(InputArray, OutputArray, float, int, int, int, int, int, int, float, int, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/bm3d_image_denoising.hpp:168
	// ("cv::xphoto::bm3dDenoising", vec![(pred!(mut, ["src", "dst", "h", "templateWindowSize", "searchWindowSize", "blockMatchingStep1", "blockMatchingStep2", "groupSize", "slidingStep", "beta", "normType", "step", "transformType"], ["const cv::_InputArray*", "const cv::_OutputArray*", "float", "int", "int", "int", "int", "int", "int", "float", "int", "int", "int"]), _)]),
	void cv_xphoto_bm3dDenoising_const__InputArrayR_const__OutputArrayR_float_int_int_int_int_int_int_float_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, float h, int templateWindowSize, int searchWindowSize, int blockMatchingStep1, int blockMatchingStep2, int groupSize, int slidingStep, float beta, int normType, int step, int transformType, ResultVoid* ocvrs_return) {
		try {
			cv::xphoto::bm3dDenoising(*src, *dst, h, templateWindowSize, searchWindowSize, blockMatchingStep1, blockMatchingStep2, groupSize, slidingStep, beta, normType, step, transformType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createGrayworldWB()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:152
	// ("cv::xphoto::createGrayworldWB", vec![(pred!(mut, [], []), _)]),
	void cv_xphoto_createGrayworldWB(Result<cv::Ptr<cv::xphoto::GrayworldWB>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xphoto::GrayworldWB> ret = cv::xphoto::createGrayworldWB();
			Ok(new cv::Ptr<cv::xphoto::GrayworldWB>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xphoto::createLearningBasedWB() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:214
	// ("cv::xphoto::createLearningBasedWB", vec![(pred!(mut, [], []), _)]),
	void cv_xphoto_createLearningBasedWB(Result<cv::Ptr<cv::xphoto::LearningBasedWB>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xphoto::LearningBasedWB> ret = cv::xphoto::createLearningBasedWB();
			Ok(new cv::Ptr<cv::xphoto::LearningBasedWB>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createLearningBasedWB(const String &)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:214
	// ("cv::xphoto::createLearningBasedWB", vec![(pred!(mut, ["path_to_model"], ["const cv::String*"]), _)]),
	void cv_xphoto_createLearningBasedWB_const_StringR(const char* path_to_model, Result<cv::Ptr<cv::xphoto::LearningBasedWB>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xphoto::LearningBasedWB> ret = cv::xphoto::createLearningBasedWB(std::string(path_to_model));
			Ok(new cv::Ptr<cv::xphoto::LearningBasedWB>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createSimpleWB()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:115
	// ("cv::xphoto::createSimpleWB", vec![(pred!(mut, [], []), _)]),
	void cv_xphoto_createSimpleWB(Result<cv::Ptr<cv::xphoto::SimpleWB>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xphoto::SimpleWB> ret = cv::xphoto::createSimpleWB();
			Ok(new cv::Ptr<cv::xphoto::SimpleWB>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xphoto::createTonemapDurand() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/tonemap.hpp:53
	// ("cv::xphoto::createTonemapDurand", vec![(pred!(mut, [], []), _)]),
	void cv_xphoto_createTonemapDurand(Result<cv::Ptr<cv::xphoto::TonemapDurand>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xphoto::TonemapDurand> ret = cv::xphoto::createTonemapDurand();
			Ok(new cv::Ptr<cv::xphoto::TonemapDurand>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// createTonemapDurand(float, float, float, float, float)(Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/tonemap.hpp:53
	// ("cv::xphoto::createTonemapDurand", vec![(pred!(mut, ["gamma", "contrast", "saturation", "sigma_color", "sigma_space"], ["float", "float", "float", "float", "float"]), _)]),
	void cv_xphoto_createTonemapDurand_float_float_float_float_float(float gamma, float contrast, float saturation, float sigma_color, float sigma_space, Result<cv::Ptr<cv::xphoto::TonemapDurand>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xphoto::TonemapDurand> ret = cv::xphoto::createTonemapDurand(gamma, contrast, saturation, sigma_color, sigma_space);
			Ok(new cv::Ptr<cv::xphoto::TonemapDurand>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xphoto::dctDenoising(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/dct_image_denoising.hpp:72
	// ("cv::xphoto::dctDenoising", vec![(pred!(mut, ["src", "dst", "sigma"], ["const cv::Mat*", "cv::Mat*", "const double"]), _)]),
	void cv_xphoto_dctDenoising_const_MatR_MatR_const_double(const cv::Mat* src, cv::Mat* dst, const double sigma, ResultVoid* ocvrs_return) {
		try {
			cv::xphoto::dctDenoising(*src, *dst, sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// dctDenoising(const Mat &, Mat &, const double, const int)(TraitClass, TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/dct_image_denoising.hpp:72
	// ("cv::xphoto::dctDenoising", vec![(pred!(mut, ["src", "dst", "sigma", "psize"], ["const cv::Mat*", "cv::Mat*", "const double", "const int"]), _)]),
	void cv_xphoto_dctDenoising_const_MatR_MatR_const_double_const_int(const cv::Mat* src, cv::Mat* dst, const double sigma, const int psize, ResultVoid* ocvrs_return) {
		try {
			cv::xphoto::dctDenoising(*src, *dst, sigma, psize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// inpaint(const Mat &, const Mat &, Mat &, const int)(TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/inpainting.hpp:113
	// ("cv::xphoto::inpaint", vec![(pred!(mut, ["src", "mask", "dst", "algorithmType"], ["const cv::Mat*", "const cv::Mat*", "cv::Mat*", "const int"]), _)]),
	void cv_xphoto_inpaint_const_MatR_const_MatR_MatR_const_int(const cv::Mat* src, const cv::Mat* mask, cv::Mat* dst, const int algorithmType, ResultVoid* ocvrs_return) {
		try {
			cv::xphoto::inpaint(*src, *mask, *dst, algorithmType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// oilPainting(InputArray, OutputArray, int, int)(InputArray, OutputArray, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/oilpainting.hpp:36
	// ("cv::xphoto::oilPainting", vec![(pred!(mut, ["src", "dst", "size", "dynRatio"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int"]), _)]),
	void cv_xphoto_oilPainting_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int size, int dynRatio, ResultVoid* ocvrs_return) {
		try {
			cv::xphoto::oilPainting(*src, *dst, size, dynRatio);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// oilPainting(InputArray, OutputArray, int, int, int)(InputArray, OutputArray, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/oilpainting.hpp:28
	// ("cv::xphoto::oilPainting", vec![(pred!(mut, ["src", "dst", "size", "dynRatio", "code"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "int"]), _)]),
	void cv_xphoto_oilPainting_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int size, int dynRatio, int code, ResultVoid* ocvrs_return) {
		try {
			cv::xphoto::oilPainting(*src, *dst, size, dynRatio, code);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSaturationThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:145
	// ("cv::xphoto::GrayworldWB::getSaturationThreshold", vec![(pred!(const, [], []), _)]),
	void cv_xphoto_GrayworldWB_getSaturationThreshold_const(const cv::xphoto::GrayworldWB* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSaturationThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSaturationThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:147
	// ("cv::xphoto::GrayworldWB::setSaturationThreshold", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_xphoto_GrayworldWB_setSaturationThreshold_float(cv::xphoto::GrayworldWB* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setSaturationThreshold(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xphoto::GrayworldWB::to_Algorithm() generated
	// ("cv::xphoto::GrayworldWB::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xphoto_GrayworldWB_to_Algorithm(cv::xphoto::GrayworldWB* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xphoto::GrayworldWB::to_WhiteBalancer() generated
	// ("cv::xphoto::GrayworldWB::to_WhiteBalancer", vec![(pred!(mut, [], []), _)]),
	cv::xphoto::WhiteBalancer* cv_xphoto_GrayworldWB_to_WhiteBalancer(cv::xphoto::GrayworldWB* instance) {
			return dynamic_cast<cv::xphoto::WhiteBalancer*>(instance);
	}

	// cv::xphoto::GrayworldWB::delete() generated
	// ("cv::xphoto::GrayworldWB::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xphoto_GrayworldWB_delete(cv::xphoto::GrayworldWB* instance) {
			delete instance;
	}

	// extractSimpleFeatures(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:185
	// ("cv::xphoto::LearningBasedWB::extractSimpleFeatures", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_xphoto_LearningBasedWB_extractSimpleFeatures_const__InputArrayR_const__OutputArrayR(cv::xphoto::LearningBasedWB* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->extractSimpleFeatures(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getRangeMaxVal()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:190
	// ("cv::xphoto::LearningBasedWB::getRangeMaxVal", vec![(pred!(const, [], []), _)]),
	void cv_xphoto_LearningBasedWB_getRangeMaxVal_const(const cv::xphoto::LearningBasedWB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getRangeMaxVal();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setRangeMaxVal(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:192
	// ("cv::xphoto::LearningBasedWB::setRangeMaxVal", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_xphoto_LearningBasedWB_setRangeMaxVal_int(cv::xphoto::LearningBasedWB* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setRangeMaxVal(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSaturationThreshold()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:197
	// ("cv::xphoto::LearningBasedWB::getSaturationThreshold", vec![(pred!(const, [], []), _)]),
	void cv_xphoto_LearningBasedWB_getSaturationThreshold_const(const cv::xphoto::LearningBasedWB* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSaturationThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSaturationThreshold(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:199
	// ("cv::xphoto::LearningBasedWB::setSaturationThreshold", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_xphoto_LearningBasedWB_setSaturationThreshold_float(cv::xphoto::LearningBasedWB* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setSaturationThreshold(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getHistBinNum()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:205
	// ("cv::xphoto::LearningBasedWB::getHistBinNum", vec![(pred!(const, [], []), _)]),
	void cv_xphoto_LearningBasedWB_getHistBinNum_const(const cv::xphoto::LearningBasedWB* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getHistBinNum();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setHistBinNum(int)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:207
	// ("cv::xphoto::LearningBasedWB::setHistBinNum", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_xphoto_LearningBasedWB_setHistBinNum_int(cv::xphoto::LearningBasedWB* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setHistBinNum(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xphoto::LearningBasedWB::to_Algorithm() generated
	// ("cv::xphoto::LearningBasedWB::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xphoto_LearningBasedWB_to_Algorithm(cv::xphoto::LearningBasedWB* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xphoto::LearningBasedWB::to_WhiteBalancer() generated
	// ("cv::xphoto::LearningBasedWB::to_WhiteBalancer", vec![(pred!(mut, [], []), _)]),
	cv::xphoto::WhiteBalancer* cv_xphoto_LearningBasedWB_to_WhiteBalancer(cv::xphoto::LearningBasedWB* instance) {
			return dynamic_cast<cv::xphoto::WhiteBalancer*>(instance);
	}

	// cv::xphoto::LearningBasedWB::delete() generated
	// ("cv::xphoto::LearningBasedWB::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xphoto_LearningBasedWB_delete(cv::xphoto::LearningBasedWB* instance) {
			delete instance;
	}

	// getInputMin()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:84
	// ("cv::xphoto::SimpleWB::getInputMin", vec![(pred!(const, [], []), _)]),
	void cv_xphoto_SimpleWB_getInputMin_const(const cv::xphoto::SimpleWB* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getInputMin();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInputMin(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:86
	// ("cv::xphoto::SimpleWB::setInputMin", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_xphoto_SimpleWB_setInputMin_float(cv::xphoto::SimpleWB* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setInputMin(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getInputMax()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:90
	// ("cv::xphoto::SimpleWB::getInputMax", vec![(pred!(const, [], []), _)]),
	void cv_xphoto_SimpleWB_getInputMax_const(const cv::xphoto::SimpleWB* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getInputMax();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setInputMax(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:92
	// ("cv::xphoto::SimpleWB::setInputMax", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_xphoto_SimpleWB_setInputMax_float(cv::xphoto::SimpleWB* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setInputMax(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOutputMin()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:96
	// ("cv::xphoto::SimpleWB::getOutputMin", vec![(pred!(const, [], []), _)]),
	void cv_xphoto_SimpleWB_getOutputMin_const(const cv::xphoto::SimpleWB* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getOutputMin();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOutputMin(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:98
	// ("cv::xphoto::SimpleWB::setOutputMin", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_xphoto_SimpleWB_setOutputMin_float(cv::xphoto::SimpleWB* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setOutputMin(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOutputMax()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:102
	// ("cv::xphoto::SimpleWB::getOutputMax", vec![(pred!(const, [], []), _)]),
	void cv_xphoto_SimpleWB_getOutputMax_const(const cv::xphoto::SimpleWB* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getOutputMax();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setOutputMax(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:104
	// ("cv::xphoto::SimpleWB::setOutputMax", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_xphoto_SimpleWB_setOutputMax_float(cv::xphoto::SimpleWB* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setOutputMax(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getP()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:108
	// ("cv::xphoto::SimpleWB::getP", vec![(pred!(const, [], []), _)]),
	void cv_xphoto_SimpleWB_getP_const(const cv::xphoto::SimpleWB* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getP();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setP(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:110
	// ("cv::xphoto::SimpleWB::setP", vec![(pred!(mut, ["val"], ["float"]), _)]),
	void cv_xphoto_SimpleWB_setP_float(cv::xphoto::SimpleWB* instance, float val, ResultVoid* ocvrs_return) {
		try {
			instance->setP(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xphoto::SimpleWB::to_Algorithm() generated
	// ("cv::xphoto::SimpleWB::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xphoto_SimpleWB_to_Algorithm(cv::xphoto::SimpleWB* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xphoto::SimpleWB::to_WhiteBalancer() generated
	// ("cv::xphoto::SimpleWB::to_WhiteBalancer", vec![(pred!(mut, [], []), _)]),
	cv::xphoto::WhiteBalancer* cv_xphoto_SimpleWB_to_WhiteBalancer(cv::xphoto::SimpleWB* instance) {
			return dynamic_cast<cv::xphoto::WhiteBalancer*>(instance);
	}

	// cv::xphoto::SimpleWB::delete() generated
	// ("cv::xphoto::SimpleWB::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xphoto_SimpleWB_delete(cv::xphoto::SimpleWB* instance) {
			delete instance;
	}

	// getSaturation()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/tonemap.hpp:28
	// ("cv::xphoto::TonemapDurand::getSaturation", vec![(pred!(const, [], []), _)]),
	void cv_xphoto_TonemapDurand_getSaturation_const(const cv::xphoto::TonemapDurand* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSaturation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSaturation(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/tonemap.hpp:29
	// ("cv::xphoto::TonemapDurand::setSaturation", vec![(pred!(mut, ["saturation"], ["float"]), _)]),
	void cv_xphoto_TonemapDurand_setSaturation_float(cv::xphoto::TonemapDurand* instance, float saturation, ResultVoid* ocvrs_return) {
		try {
			instance->setSaturation(saturation);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getContrast()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/tonemap.hpp:31
	// ("cv::xphoto::TonemapDurand::getContrast", vec![(pred!(const, [], []), _)]),
	void cv_xphoto_TonemapDurand_getContrast_const(const cv::xphoto::TonemapDurand* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getContrast();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setContrast(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/tonemap.hpp:32
	// ("cv::xphoto::TonemapDurand::setContrast", vec![(pred!(mut, ["contrast"], ["float"]), _)]),
	void cv_xphoto_TonemapDurand_setContrast_float(cv::xphoto::TonemapDurand* instance, float contrast, ResultVoid* ocvrs_return) {
		try {
			instance->setContrast(contrast);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSigmaSpace()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/tonemap.hpp:34
	// ("cv::xphoto::TonemapDurand::getSigmaSpace", vec![(pred!(const, [], []), _)]),
	void cv_xphoto_TonemapDurand_getSigmaSpace_const(const cv::xphoto::TonemapDurand* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSigmaSpace();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSigmaSpace(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/tonemap.hpp:35
	// ("cv::xphoto::TonemapDurand::setSigmaSpace", vec![(pred!(mut, ["sigma_space"], ["float"]), _)]),
	void cv_xphoto_TonemapDurand_setSigmaSpace_float(cv::xphoto::TonemapDurand* instance, float sigma_space, ResultVoid* ocvrs_return) {
		try {
			instance->setSigmaSpace(sigma_space);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSigmaColor()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/tonemap.hpp:37
	// ("cv::xphoto::TonemapDurand::getSigmaColor", vec![(pred!(const, [], []), _)]),
	void cv_xphoto_TonemapDurand_getSigmaColor_const(const cv::xphoto::TonemapDurand* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSigmaColor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setSigmaColor(float)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/tonemap.hpp:38
	// ("cv::xphoto::TonemapDurand::setSigmaColor", vec![(pred!(mut, ["sigma_color"], ["float"]), _)]),
	void cv_xphoto_TonemapDurand_setSigmaColor_float(cv::xphoto::TonemapDurand* instance, float sigma_color, ResultVoid* ocvrs_return) {
		try {
			instance->setSigmaColor(sigma_color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xphoto::TonemapDurand::to_Algorithm() generated
	// ("cv::xphoto::TonemapDurand::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xphoto_TonemapDurand_to_Algorithm(cv::xphoto::TonemapDurand* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xphoto::TonemapDurand::to_Tonemap() generated
	// ("cv::xphoto::TonemapDurand::to_Tonemap", vec![(pred!(mut, [], []), _)]),
	cv::Tonemap* cv_xphoto_TonemapDurand_to_Tonemap(cv::xphoto::TonemapDurand* instance) {
			return dynamic_cast<cv::Tonemap*>(instance);
	}

	// cv::xphoto::TonemapDurand::delete() generated
	// ("cv::xphoto::TonemapDurand::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xphoto_TonemapDurand_delete(cv::xphoto::TonemapDurand* instance) {
			delete instance;
	}

	// balanceWhite(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/xphoto/white_balance.hpp:72
	// ("cv::xphoto::WhiteBalancer::balanceWhite", vec![(pred!(mut, ["src", "dst"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_xphoto_WhiteBalancer_balanceWhite_const__InputArrayR_const__OutputArrayR(cv::xphoto::WhiteBalancer* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->balanceWhite(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::xphoto::WhiteBalancer::to_GrayworldWB() generated
	// ("cv::xphoto::WhiteBalancer::to_GrayworldWB", vec![(pred!(mut, [], []), _)]),
	cv::xphoto::GrayworldWB* cv_xphoto_WhiteBalancer_to_GrayworldWB(cv::xphoto::WhiteBalancer* instance) {
			return dynamic_cast<cv::xphoto::GrayworldWB*>(instance);
	}

	// cv::xphoto::WhiteBalancer::to_LearningBasedWB() generated
	// ("cv::xphoto::WhiteBalancer::to_LearningBasedWB", vec![(pred!(mut, [], []), _)]),
	cv::xphoto::LearningBasedWB* cv_xphoto_WhiteBalancer_to_LearningBasedWB(cv::xphoto::WhiteBalancer* instance) {
			return dynamic_cast<cv::xphoto::LearningBasedWB*>(instance);
	}

	// cv::xphoto::WhiteBalancer::to_SimpleWB() generated
	// ("cv::xphoto::WhiteBalancer::to_SimpleWB", vec![(pred!(mut, [], []), _)]),
	cv::xphoto::SimpleWB* cv_xphoto_WhiteBalancer_to_SimpleWB(cv::xphoto::WhiteBalancer* instance) {
			return dynamic_cast<cv::xphoto::SimpleWB*>(instance);
	}

	// cv::xphoto::WhiteBalancer::to_Algorithm() generated
	// ("cv::xphoto::WhiteBalancer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_xphoto_WhiteBalancer_to_Algorithm(cv::xphoto::WhiteBalancer* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::xphoto::WhiteBalancer::delete() generated
	// ("cv::xphoto::WhiteBalancer::delete", vec![(pred!(mut, [], []), _)]),
	void cv_xphoto_WhiteBalancer_delete(cv::xphoto::WhiteBalancer* instance) {
			delete instance;
	}

}
