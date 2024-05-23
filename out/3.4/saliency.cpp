#include "ocvrs_common.hpp"
#include <opencv2/saliency.hpp>
#include "saliency_types.hpp"

extern "C" {
	// cv::saliency::MotionSaliency::to_MotionSaliencyBinWangApr2014() generated
	// ("cv::saliency::MotionSaliency::to_MotionSaliencyBinWangApr2014", vec![(pred!(mut, [], []), _)]),
	cv::saliency::MotionSaliencyBinWangApr2014* cv_saliency_MotionSaliency_to_MotionSaliencyBinWangApr2014(cv::saliency::MotionSaliency* instance) {
			return dynamic_cast<cv::saliency::MotionSaliencyBinWangApr2014*>(instance);
	}

	// cv::saliency::MotionSaliency::to_Algorithm() generated
	// ("cv::saliency::MotionSaliency::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_saliency_MotionSaliency_to_Algorithm(cv::saliency::MotionSaliency* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::saliency::MotionSaliency::to_Saliency() generated
	// ("cv::saliency::MotionSaliency::to_Saliency", vec![(pred!(mut, [], []), _)]),
	cv::saliency::Saliency* cv_saliency_MotionSaliency_to_Saliency(cv::saliency::MotionSaliency* instance) {
			return dynamic_cast<cv::saliency::Saliency*>(instance);
	}

	// cv::saliency::MotionSaliency::delete() generated
	// ("cv::saliency::MotionSaliency::delete", vec![(pred!(mut, [], []), _)]),
	void cv_saliency_MotionSaliency_delete(cv::saliency::MotionSaliency* instance) {
			delete instance;
	}

	// MotionSaliencyBinWangApr2014()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:172
	// ("cv::saliency::MotionSaliencyBinWangApr2014::MotionSaliencyBinWangApr2014", vec![(pred!(mut, [], []), _)]),
	void cv_saliency_MotionSaliencyBinWangApr2014_MotionSaliencyBinWangApr2014(Result<cv::saliency::MotionSaliencyBinWangApr2014*>* ocvrs_return) {
		try {
			cv::saliency::MotionSaliencyBinWangApr2014* ret = new cv::saliency::MotionSaliencyBinWangApr2014();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:175
	// ("cv::saliency::MotionSaliencyBinWangApr2014::create", vec![(pred!(mut, [], []), _)]),
	void cv_saliency_MotionSaliencyBinWangApr2014_create(Result<cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014> ret = cv::saliency::MotionSaliencyBinWangApr2014::create();
			Ok(new cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeSaliency(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:180
	// ("cv::saliency::MotionSaliencyBinWangApr2014::computeSaliency", vec![(pred!(mut, ["image", "saliencyMap"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_saliency_MotionSaliencyBinWangApr2014_computeSaliency_const__InputArrayR_const__OutputArrayR(cv::saliency::MotionSaliencyBinWangApr2014* instance, const cv::_InputArray* image, const cv::_OutputArray* saliencyMap, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->computeSaliency(*image, *saliencyMap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setImagesize(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:193
	// ("cv::saliency::MotionSaliencyBinWangApr2014::setImagesize", vec![(pred!(mut, ["W", "H"], ["int", "int"]), _)]),
	void cv_saliency_MotionSaliencyBinWangApr2014_setImagesize_int_int(cv::saliency::MotionSaliencyBinWangApr2014* instance, int W, int H, ResultVoid* ocvrs_return) {
		try {
			instance->setImagesize(W, H);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// init()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:197
	// ("cv::saliency::MotionSaliencyBinWangApr2014::init", vec![(pred!(mut, [], []), _)]),
	void cv_saliency_MotionSaliencyBinWangApr2014_init(cv::saliency::MotionSaliencyBinWangApr2014* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->init();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getImageWidth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:199
	// ("cv::saliency::MotionSaliencyBinWangApr2014::getImageWidth", vec![(pred!(const, [], []), _)]),
	void cv_saliency_MotionSaliencyBinWangApr2014_getImageWidth_const(const cv::saliency::MotionSaliencyBinWangApr2014* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getImageWidth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setImageWidth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:203
	// ("cv::saliency::MotionSaliencyBinWangApr2014::setImageWidth", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_saliency_MotionSaliencyBinWangApr2014_setImageWidth_int(cv::saliency::MotionSaliencyBinWangApr2014* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setImageWidth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getImageHeight()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:207
	// ("cv::saliency::MotionSaliencyBinWangApr2014::getImageHeight", vec![(pred!(const, [], []), _)]),
	void cv_saliency_MotionSaliencyBinWangApr2014_getImageHeight_const(const cv::saliency::MotionSaliencyBinWangApr2014* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getImageHeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setImageHeight(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:211
	// ("cv::saliency::MotionSaliencyBinWangApr2014::setImageHeight", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_saliency_MotionSaliencyBinWangApr2014_setImageHeight_int(cv::saliency::MotionSaliencyBinWangApr2014* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setImageHeight(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::saliency::MotionSaliencyBinWangApr2014::to_Algorithm() generated
	// ("cv::saliency::MotionSaliencyBinWangApr2014::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_saliency_MotionSaliencyBinWangApr2014_to_Algorithm(cv::saliency::MotionSaliencyBinWangApr2014* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::saliency::MotionSaliencyBinWangApr2014::to_MotionSaliency() generated
	// ("cv::saliency::MotionSaliencyBinWangApr2014::to_MotionSaliency", vec![(pred!(mut, [], []), _)]),
	cv::saliency::MotionSaliency* cv_saliency_MotionSaliencyBinWangApr2014_to_MotionSaliency(cv::saliency::MotionSaliencyBinWangApr2014* instance) {
			return dynamic_cast<cv::saliency::MotionSaliency*>(instance);
	}

	// cv::saliency::MotionSaliencyBinWangApr2014::to_Saliency() generated
	// ("cv::saliency::MotionSaliencyBinWangApr2014::to_Saliency", vec![(pred!(mut, [], []), _)]),
	cv::saliency::Saliency* cv_saliency_MotionSaliencyBinWangApr2014_to_Saliency(cv::saliency::MotionSaliencyBinWangApr2014* instance) {
			return dynamic_cast<cv::saliency::Saliency*>(instance);
	}

	// cv::saliency::MotionSaliencyBinWangApr2014::delete() generated
	// ("cv::saliency::MotionSaliencyBinWangApr2014::delete", vec![(pred!(mut, [], []), _)]),
	void cv_saliency_MotionSaliencyBinWangApr2014_delete(cv::saliency::MotionSaliencyBinWangApr2014* instance) {
			delete instance;
	}

	// cv::saliency::Objectness::to_ObjectnessBING() generated
	// ("cv::saliency::Objectness::to_ObjectnessBING", vec![(pred!(mut, [], []), _)]),
	cv::saliency::ObjectnessBING* cv_saliency_Objectness_to_ObjectnessBING(cv::saliency::Objectness* instance) {
			return dynamic_cast<cv::saliency::ObjectnessBING*>(instance);
	}

	// cv::saliency::Objectness::to_Algorithm() generated
	// ("cv::saliency::Objectness::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_saliency_Objectness_to_Algorithm(cv::saliency::Objectness* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::saliency::Objectness::to_Saliency() generated
	// ("cv::saliency::Objectness::to_Saliency", vec![(pred!(mut, [], []), _)]),
	cv::saliency::Saliency* cv_saliency_Objectness_to_Saliency(cv::saliency::Objectness* instance) {
			return dynamic_cast<cv::saliency::Saliency*>(instance);
	}

	// cv::saliency::Objectness::delete() generated
	// ("cv::saliency::Objectness::delete", vec![(pred!(mut, [], []), _)]),
	void cv_saliency_Objectness_delete(cv::saliency::Objectness* instance) {
			delete instance;
	}

	// ObjectnessBING()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:295
	// ("cv::saliency::ObjectnessBING::ObjectnessBING", vec![(pred!(mut, [], []), _)]),
	void cv_saliency_ObjectnessBING_ObjectnessBING(Result<cv::saliency::ObjectnessBING*>* ocvrs_return) {
		try {
			cv::saliency::ObjectnessBING* ret = new cv::saliency::ObjectnessBING();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:298
	// ("cv::saliency::ObjectnessBING::create", vec![(pred!(mut, [], []), _)]),
	void cv_saliency_ObjectnessBING_create(Result<cv::Ptr<cv::saliency::ObjectnessBING>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::saliency::ObjectnessBING> ret = cv::saliency::ObjectnessBING::create();
			Ok(new cv::Ptr<cv::saliency::ObjectnessBING>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeSaliency(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:303
	// ("cv::saliency::ObjectnessBING::computeSaliency", vec![(pred!(mut, ["image", "saliencyMap"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_saliency_ObjectnessBING_computeSaliency_const__InputArrayR_const__OutputArrayR(cv::saliency::ObjectnessBING* instance, const cv::_InputArray* image, const cv::_OutputArray* saliencyMap, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->computeSaliency(*image, *saliencyMap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:311
	// ("cv::saliency::ObjectnessBING::read", vec![(pred!(mut, [], []), _)]),
	void cv_saliency_ObjectnessBING_read(cv::saliency::ObjectnessBING* instance, ResultVoid* ocvrs_return) {
		try {
			instance->read();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:312
	// ("cv::saliency::ObjectnessBING::write", vec![(pred!(const, [], []), _)]),
	void cv_saliency_ObjectnessBING_write_const(const cv::saliency::ObjectnessBING* instance, ResultVoid* ocvrs_return) {
		try {
			instance->write();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getobjectnessValues()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:320
	// ("cv::saliency::ObjectnessBING::getobjectnessValues", vec![(pred!(mut, [], []), _)]),
	void cv_saliency_ObjectnessBING_getobjectnessValues(cv::saliency::ObjectnessBING* instance, Result<std::vector<float>*>* ocvrs_return) {
		try {
			std::vector<float> ret = instance->getobjectnessValues();
			Ok(new std::vector<float>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setTrainingPath(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:326
	// ("cv::saliency::ObjectnessBING::setTrainingPath", vec![(pred!(mut, ["trainingPath"], ["const cv::String*"]), _)]),
	void cv_saliency_ObjectnessBING_setTrainingPath_const_StringR(cv::saliency::ObjectnessBING* instance, const char* trainingPath, ResultVoid* ocvrs_return) {
		try {
			instance->setTrainingPath(cv::String(trainingPath));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBBResDir(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:335
	// ("cv::saliency::ObjectnessBING::setBBResDir", vec![(pred!(mut, ["resultsDir"], ["const cv::String*"]), _)]),
	void cv_saliency_ObjectnessBING_setBBResDir_const_StringR(cv::saliency::ObjectnessBING* instance, const char* resultsDir, ResultVoid* ocvrs_return) {
		try {
			instance->setBBResDir(cv::String(resultsDir));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getBase()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:337
	// ("cv::saliency::ObjectnessBING::getBase", vec![(pred!(const, [], []), _)]),
	void cv_saliency_ObjectnessBING_getBase_const(const cv::saliency::ObjectnessBING* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getBase();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setBase(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:341
	// ("cv::saliency::ObjectnessBING::setBase", vec![(pred!(mut, ["val"], ["double"]), _)]),
	void cv_saliency_ObjectnessBING_setBase_double(cv::saliency::ObjectnessBING* instance, double val, ResultVoid* ocvrs_return) {
		try {
			instance->setBase(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getNSS()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:345
	// ("cv::saliency::ObjectnessBING::getNSS", vec![(pred!(const, [], []), _)]),
	void cv_saliency_ObjectnessBING_getNSS_const(const cv::saliency::ObjectnessBING* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNSS();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setNSS(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:349
	// ("cv::saliency::ObjectnessBING::setNSS", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_saliency_ObjectnessBING_setNSS_int(cv::saliency::ObjectnessBING* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setNSS(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getW()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:353
	// ("cv::saliency::ObjectnessBING::getW", vec![(pred!(const, [], []), _)]),
	void cv_saliency_ObjectnessBING_getW_const(const cv::saliency::ObjectnessBING* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getW();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setW(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:357
	// ("cv::saliency::ObjectnessBING::setW", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_saliency_ObjectnessBING_setW_int(cv::saliency::ObjectnessBING* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setW(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::saliency::ObjectnessBING::to_Algorithm() generated
	// ("cv::saliency::ObjectnessBING::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_saliency_ObjectnessBING_to_Algorithm(cv::saliency::ObjectnessBING* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::saliency::ObjectnessBING::to_Objectness() generated
	// ("cv::saliency::ObjectnessBING::to_Objectness", vec![(pred!(mut, [], []), _)]),
	cv::saliency::Objectness* cv_saliency_ObjectnessBING_to_Objectness(cv::saliency::ObjectnessBING* instance) {
			return dynamic_cast<cv::saliency::Objectness*>(instance);
	}

	// cv::saliency::ObjectnessBING::to_Saliency() generated
	// ("cv::saliency::ObjectnessBING::to_Saliency", vec![(pred!(mut, [], []), _)]),
	cv::saliency::Saliency* cv_saliency_ObjectnessBING_to_Saliency(cv::saliency::ObjectnessBING* instance) {
			return dynamic_cast<cv::saliency::Saliency*>(instance);
	}

	// cv::saliency::ObjectnessBING::delete() generated
	// ("cv::saliency::ObjectnessBING::delete", vec![(pred!(mut, [], []), _)]),
	void cv_saliency_ObjectnessBING_delete(cv::saliency::ObjectnessBING* instance) {
			delete instance;
	}

	// computeSaliency(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencyBaseClasses.hpp:76
	// ("cv::saliency::Saliency::computeSaliency", vec![(pred!(mut, ["image", "saliencyMap"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_saliency_Saliency_computeSaliency_const__InputArrayR_const__OutputArrayR(cv::saliency::Saliency* instance, const cv::_InputArray* image, const cv::_OutputArray* saliencyMap, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->computeSaliency(*image, *saliencyMap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::saliency::Saliency::to_MotionSaliency() generated
	// ("cv::saliency::Saliency::to_MotionSaliency", vec![(pred!(mut, [], []), _)]),
	cv::saliency::MotionSaliency* cv_saliency_Saliency_to_MotionSaliency(cv::saliency::Saliency* instance) {
			return dynamic_cast<cv::saliency::MotionSaliency*>(instance);
	}

	// cv::saliency::Saliency::to_MotionSaliencyBinWangApr2014() generated
	// ("cv::saliency::Saliency::to_MotionSaliencyBinWangApr2014", vec![(pred!(mut, [], []), _)]),
	cv::saliency::MotionSaliencyBinWangApr2014* cv_saliency_Saliency_to_MotionSaliencyBinWangApr2014(cv::saliency::Saliency* instance) {
			return dynamic_cast<cv::saliency::MotionSaliencyBinWangApr2014*>(instance);
	}

	// cv::saliency::Saliency::to_Objectness() generated
	// ("cv::saliency::Saliency::to_Objectness", vec![(pred!(mut, [], []), _)]),
	cv::saliency::Objectness* cv_saliency_Saliency_to_Objectness(cv::saliency::Saliency* instance) {
			return dynamic_cast<cv::saliency::Objectness*>(instance);
	}

	// cv::saliency::Saliency::to_ObjectnessBING() generated
	// ("cv::saliency::Saliency::to_ObjectnessBING", vec![(pred!(mut, [], []), _)]),
	cv::saliency::ObjectnessBING* cv_saliency_Saliency_to_ObjectnessBING(cv::saliency::Saliency* instance) {
			return dynamic_cast<cv::saliency::ObjectnessBING*>(instance);
	}

	// cv::saliency::Saliency::to_StaticSaliency() generated
	// ("cv::saliency::Saliency::to_StaticSaliency", vec![(pred!(mut, [], []), _)]),
	cv::saliency::StaticSaliency* cv_saliency_Saliency_to_StaticSaliency(cv::saliency::Saliency* instance) {
			return dynamic_cast<cv::saliency::StaticSaliency*>(instance);
	}

	// cv::saliency::Saliency::to_StaticSaliencyFineGrained() generated
	// ("cv::saliency::Saliency::to_StaticSaliencyFineGrained", vec![(pred!(mut, [], []), _)]),
	cv::saliency::StaticSaliencyFineGrained* cv_saliency_Saliency_to_StaticSaliencyFineGrained(cv::saliency::Saliency* instance) {
			return dynamic_cast<cv::saliency::StaticSaliencyFineGrained*>(instance);
	}

	// cv::saliency::Saliency::to_StaticSaliencySpectralResidual() generated
	// ("cv::saliency::Saliency::to_StaticSaliencySpectralResidual", vec![(pred!(mut, [], []), _)]),
	cv::saliency::StaticSaliencySpectralResidual* cv_saliency_Saliency_to_StaticSaliencySpectralResidual(cv::saliency::Saliency* instance) {
			return dynamic_cast<cv::saliency::StaticSaliencySpectralResidual*>(instance);
	}

	// cv::saliency::Saliency::to_Algorithm() generated
	// ("cv::saliency::Saliency::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_saliency_Saliency_to_Algorithm(cv::saliency::Saliency* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::saliency::Saliency::delete() generated
	// ("cv::saliency::Saliency::delete", vec![(pred!(mut, [], []), _)]),
	void cv_saliency_Saliency_delete(cv::saliency::Saliency* instance) {
			delete instance;
	}

	// computeBinaryMap(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencyBaseClasses.hpp:104
	// ("cv::saliency::StaticSaliency::computeBinaryMap", vec![(pred!(mut, ["_saliencyMap", "_binaryMap"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_saliency_StaticSaliency_computeBinaryMap_const__InputArrayR_const__OutputArrayR(cv::saliency::StaticSaliency* instance, const cv::_InputArray* _saliencyMap, const cv::_OutputArray* _binaryMap, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->computeBinaryMap(*_saliencyMap, *_binaryMap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::saliency::StaticSaliency::to_StaticSaliencyFineGrained() generated
	// ("cv::saliency::StaticSaliency::to_StaticSaliencyFineGrained", vec![(pred!(mut, [], []), _)]),
	cv::saliency::StaticSaliencyFineGrained* cv_saliency_StaticSaliency_to_StaticSaliencyFineGrained(cv::saliency::StaticSaliency* instance) {
			return dynamic_cast<cv::saliency::StaticSaliencyFineGrained*>(instance);
	}

	// cv::saliency::StaticSaliency::to_StaticSaliencySpectralResidual() generated
	// ("cv::saliency::StaticSaliency::to_StaticSaliencySpectralResidual", vec![(pred!(mut, [], []), _)]),
	cv::saliency::StaticSaliencySpectralResidual* cv_saliency_StaticSaliency_to_StaticSaliencySpectralResidual(cv::saliency::StaticSaliency* instance) {
			return dynamic_cast<cv::saliency::StaticSaliencySpectralResidual*>(instance);
	}

	// cv::saliency::StaticSaliency::to_Algorithm() generated
	// ("cv::saliency::StaticSaliency::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_saliency_StaticSaliency_to_Algorithm(cv::saliency::StaticSaliency* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::saliency::StaticSaliency::to_Saliency() generated
	// ("cv::saliency::StaticSaliency::to_Saliency", vec![(pred!(mut, [], []), _)]),
	cv::saliency::Saliency* cv_saliency_StaticSaliency_to_Saliency(cv::saliency::StaticSaliency* instance) {
			return dynamic_cast<cv::saliency::Saliency*>(instance);
	}

	// cv::saliency::StaticSaliency::delete() generated
	// ("cv::saliency::StaticSaliency::delete", vec![(pred!(mut, [], []), _)]),
	void cv_saliency_StaticSaliency_delete(cv::saliency::StaticSaliency* instance) {
			delete instance;
	}

	// StaticSaliencyFineGrained()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:126
	// ("cv::saliency::StaticSaliencyFineGrained::StaticSaliencyFineGrained", vec![(pred!(mut, [], []), _)]),
	void cv_saliency_StaticSaliencyFineGrained_StaticSaliencyFineGrained(Result<cv::saliency::StaticSaliencyFineGrained*>* ocvrs_return) {
		try {
			cv::saliency::StaticSaliencyFineGrained* ret = new cv::saliency::StaticSaliencyFineGrained();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:128
	// ("cv::saliency::StaticSaliencyFineGrained::create", vec![(pred!(mut, [], []), _)]),
	void cv_saliency_StaticSaliencyFineGrained_create(Result<cv::Ptr<cv::saliency::StaticSaliencyFineGrained>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::saliency::StaticSaliencyFineGrained> ret = cv::saliency::StaticSaliencyFineGrained::create();
			Ok(new cv::Ptr<cv::saliency::StaticSaliencyFineGrained>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeSaliency(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:133
	// ("cv::saliency::StaticSaliencyFineGrained::computeSaliency", vec![(pred!(mut, ["image", "saliencyMap"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_saliency_StaticSaliencyFineGrained_computeSaliency_const__InputArrayR_const__OutputArrayR(cv::saliency::StaticSaliencyFineGrained* instance, const cv::_InputArray* image, const cv::_OutputArray* saliencyMap, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->computeSaliency(*image, *saliencyMap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::saliency::StaticSaliencyFineGrained::to_Algorithm() generated
	// ("cv::saliency::StaticSaliencyFineGrained::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_saliency_StaticSaliencyFineGrained_to_Algorithm(cv::saliency::StaticSaliencyFineGrained* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::saliency::StaticSaliencyFineGrained::to_Saliency() generated
	// ("cv::saliency::StaticSaliencyFineGrained::to_Saliency", vec![(pred!(mut, [], []), _)]),
	cv::saliency::Saliency* cv_saliency_StaticSaliencyFineGrained_to_Saliency(cv::saliency::StaticSaliencyFineGrained* instance) {
			return dynamic_cast<cv::saliency::Saliency*>(instance);
	}

	// cv::saliency::StaticSaliencyFineGrained::to_StaticSaliency() generated
	// ("cv::saliency::StaticSaliencyFineGrained::to_StaticSaliency", vec![(pred!(mut, [], []), _)]),
	cv::saliency::StaticSaliency* cv_saliency_StaticSaliencyFineGrained_to_StaticSaliency(cv::saliency::StaticSaliencyFineGrained* instance) {
			return dynamic_cast<cv::saliency::StaticSaliency*>(instance);
	}

	// cv::saliency::StaticSaliencyFineGrained::delete() generated
	// ("cv::saliency::StaticSaliencyFineGrained::delete", vec![(pred!(mut, [], []), _)]),
	void cv_saliency_StaticSaliencyFineGrained_delete(cv::saliency::StaticSaliencyFineGrained* instance) {
			delete instance;
	}

	// StaticSaliencySpectralResidual()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:73
	// ("cv::saliency::StaticSaliencySpectralResidual::StaticSaliencySpectralResidual", vec![(pred!(mut, [], []), _)]),
	void cv_saliency_StaticSaliencySpectralResidual_StaticSaliencySpectralResidual(Result<cv::saliency::StaticSaliencySpectralResidual*>* ocvrs_return) {
		try {
			cv::saliency::StaticSaliencySpectralResidual* ret = new cv::saliency::StaticSaliencySpectralResidual();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:76
	// ("cv::saliency::StaticSaliencySpectralResidual::create", vec![(pred!(mut, [], []), _)]),
	void cv_saliency_StaticSaliencySpectralResidual_create(Result<cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::saliency::StaticSaliencySpectralResidual> ret = cv::saliency::StaticSaliencySpectralResidual::create();
			Ok(new cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// computeSaliency(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:81
	// ("cv::saliency::StaticSaliencySpectralResidual::computeSaliency", vec![(pred!(mut, ["image", "saliencyMap"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_saliency_StaticSaliencySpectralResidual_computeSaliency_const__InputArrayR_const__OutputArrayR(cv::saliency::StaticSaliencySpectralResidual* instance, const cv::_InputArray* image, const cv::_OutputArray* saliencyMap, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->computeSaliency(*image, *saliencyMap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:89
	// ("cv::saliency::StaticSaliencySpectralResidual::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	void cv_saliency_StaticSaliencySpectralResidual_read_const_FileNodeR(cv::saliency::StaticSaliencySpectralResidual* instance, const cv::FileNode* fn, ResultVoid* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:90
	// ("cv::saliency::StaticSaliencySpectralResidual::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_saliency_StaticSaliencySpectralResidual_write_const_FileStorageR(const cv::saliency::StaticSaliencySpectralResidual* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getImageWidth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:92
	// ("cv::saliency::StaticSaliencySpectralResidual::getImageWidth", vec![(pred!(const, [], []), _)]),
	void cv_saliency_StaticSaliencySpectralResidual_getImageWidth_const(const cv::saliency::StaticSaliencySpectralResidual* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getImageWidth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setImageWidth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:96
	// ("cv::saliency::StaticSaliencySpectralResidual::setImageWidth", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_saliency_StaticSaliencySpectralResidual_setImageWidth_int(cv::saliency::StaticSaliencySpectralResidual* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setImageWidth(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getImageHeight()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:100
	// ("cv::saliency::StaticSaliencySpectralResidual::getImageHeight", vec![(pred!(const, [], []), _)]),
	void cv_saliency_StaticSaliencySpectralResidual_getImageHeight_const(const cv::saliency::StaticSaliencySpectralResidual* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getImageHeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setImageHeight(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:104
	// ("cv::saliency::StaticSaliencySpectralResidual::setImageHeight", vec![(pred!(mut, ["val"], ["int"]), _)]),
	void cv_saliency_StaticSaliencySpectralResidual_setImageHeight_int(cv::saliency::StaticSaliencySpectralResidual* instance, int val, ResultVoid* ocvrs_return) {
		try {
			instance->setImageHeight(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::saliency::StaticSaliencySpectralResidual::to_Algorithm() generated
	// ("cv::saliency::StaticSaliencySpectralResidual::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_saliency_StaticSaliencySpectralResidual_to_Algorithm(cv::saliency::StaticSaliencySpectralResidual* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::saliency::StaticSaliencySpectralResidual::to_Saliency() generated
	// ("cv::saliency::StaticSaliencySpectralResidual::to_Saliency", vec![(pred!(mut, [], []), _)]),
	cv::saliency::Saliency* cv_saliency_StaticSaliencySpectralResidual_to_Saliency(cv::saliency::StaticSaliencySpectralResidual* instance) {
			return dynamic_cast<cv::saliency::Saliency*>(instance);
	}

	// cv::saliency::StaticSaliencySpectralResidual::to_StaticSaliency() generated
	// ("cv::saliency::StaticSaliencySpectralResidual::to_StaticSaliency", vec![(pred!(mut, [], []), _)]),
	cv::saliency::StaticSaliency* cv_saliency_StaticSaliencySpectralResidual_to_StaticSaliency(cv::saliency::StaticSaliencySpectralResidual* instance) {
			return dynamic_cast<cv::saliency::StaticSaliency*>(instance);
	}

	// cv::saliency::StaticSaliencySpectralResidual::delete() generated
	// ("cv::saliency::StaticSaliencySpectralResidual::delete", vec![(pred!(mut, [], []), _)]),
	void cv_saliency_StaticSaliencySpectralResidual_delete(cv::saliency::StaticSaliencySpectralResidual* instance) {
			delete instance;
	}

}
