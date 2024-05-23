#include "bioinspired.hpp"
#include "bioinspired_types.hpp"

extern "C" {
	// getInputSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:170
	// ("cv::bioinspired::Retina::getInputSize", vec![(pred!(mut, [], []), _)]),
	void cv_bioinspired_Retina_getInputSize(cv::bioinspired::Retina* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getInputSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getOutputSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:176
	// ("cv::bioinspired::Retina::getOutputSize", vec![(pred!(mut, [], []), _)]),
	void cv_bioinspired_Retina_getOutputSize(cv::bioinspired::Retina* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getOutputSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setup(String, const bool)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:188
	// ("cv::bioinspired::Retina::setup", vec![(pred!(mut, ["retinaParameterFile", "applyDefaultSetupOnFailure"], ["cv::String", "const bool"]), _)]),
	void cv_bioinspired_Retina_setup_String_const_bool(cv::bioinspired::Retina* instance, const char* retinaParameterFile, const bool applyDefaultSetupOnFailure, ResultVoid* ocvrs_return) {
		try {
			instance->setup(std::string(retinaParameterFile), applyDefaultSetupOnFailure);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bioinspired::Retina::setup() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:188
	// ("cv::bioinspired::Retina::setup", vec![(pred!(mut, [], []), _)]),
	void cv_bioinspired_Retina_setup(cv::bioinspired::Retina* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setup();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setup(cv::FileStorage &, const bool)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:194
	// ("cv::bioinspired::Retina::setup", vec![(pred!(mut, ["fs", "applyDefaultSetupOnFailure"], ["cv::FileStorage*", "const bool"]), _)]),
	void cv_bioinspired_Retina_setup_FileStorageR_const_bool(cv::bioinspired::Retina* instance, cv::FileStorage* fs, const bool applyDefaultSetupOnFailure, ResultVoid* ocvrs_return) {
		try {
			instance->setup(*fs, applyDefaultSetupOnFailure);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bioinspired::Retina::setup(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:194
	// ("cv::bioinspired::Retina::setup", vec![(pred!(mut, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_bioinspired_Retina_setup_FileStorageR(cv::bioinspired::Retina* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->setup(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setup(RetinaParameters)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:199
	// ("cv::bioinspired::Retina::setup", vec![(pred!(mut, ["newParameters"], ["cv::bioinspired::RetinaParameters"]), _)]),
	void cv_bioinspired_Retina_setup_RetinaParameters(cv::bioinspired::Retina* instance, cv::bioinspired::RetinaParameters* newParameters, ResultVoid* ocvrs_return) {
		try {
			instance->setup(*newParameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:204
	// ("cv::bioinspired::Retina::getParameters", vec![(pred!(mut, [], []), _)]),
	void cv_bioinspired_Retina_getParameters(cv::bioinspired::Retina* instance, Result<cv::bioinspired::RetinaParameters*>* ocvrs_return) {
		try {
			cv::bioinspired::RetinaParameters ret = instance->getParameters();
			Ok(new cv::bioinspired::RetinaParameters(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// printSetup()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:209
	// ("cv::bioinspired::Retina::printSetup", vec![(pred!(mut, [], []), _)]),
	void cv_bioinspired_Retina_printSetup(cv::bioinspired::Retina* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->printSetup();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(String)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:215
	// ("cv::bioinspired::Retina::write", vec![(pred!(const, ["fs"], ["cv::String"]), _)]),
	void cv_bioinspired_Retina_write_const_String(const cv::bioinspired::Retina* instance, const char* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(std::string(fs));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:218
	// ("cv::bioinspired::Retina::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_bioinspired_Retina_write_const_FileStorageR(const cv::bioinspired::Retina* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setupOPLandIPLParvoChannel(const bool, const bool, const float, const float, const float, const float, const float, const float, const float)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:254
	// ("cv::bioinspired::Retina::setupOPLandIPLParvoChannel", vec![(pred!(mut, ["colorMode", "normaliseOutput", "photoreceptorsLocalAdaptationSensitivity", "photoreceptorsTemporalConstant", "photoreceptorsSpatialConstant", "horizontalCellsGain", "HcellsTemporalConstant", "HcellsSpatialConstant", "ganglionCellsSensitivity"], ["const bool", "const bool", "const float", "const float", "const float", "const float", "const float", "const float", "const float"]), _)]),
	void cv_bioinspired_Retina_setupOPLandIPLParvoChannel_const_bool_const_bool_const_float_const_float_const_float_const_float_const_float_const_float_const_float(cv::bioinspired::Retina* instance, const bool colorMode, const bool normaliseOutput, const float photoreceptorsLocalAdaptationSensitivity, const float photoreceptorsTemporalConstant, const float photoreceptorsSpatialConstant, const float horizontalCellsGain, const float HcellsTemporalConstant, const float HcellsSpatialConstant, const float ganglionCellsSensitivity, ResultVoid* ocvrs_return) {
		try {
			instance->setupOPLandIPLParvoChannel(colorMode, normaliseOutput, photoreceptorsLocalAdaptationSensitivity, photoreceptorsTemporalConstant, photoreceptorsSpatialConstant, horizontalCellsGain, HcellsTemporalConstant, HcellsSpatialConstant, ganglionCellsSensitivity);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bioinspired::Retina::setupOPLandIPLParvoChannel() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:254
	// ("cv::bioinspired::Retina::setupOPLandIPLParvoChannel", vec![(pred!(mut, [], []), _)]),
	void cv_bioinspired_Retina_setupOPLandIPLParvoChannel(cv::bioinspired::Retina* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setupOPLandIPLParvoChannel();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setupIPLMagnoChannel(const bool, const float, const float, const float, const float, const float, const float, const float)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:281
	// ("cv::bioinspired::Retina::setupIPLMagnoChannel", vec![(pred!(mut, ["normaliseOutput", "parasolCells_beta", "parasolCells_tau", "parasolCells_k", "amacrinCellsTemporalCutFrequency", "V0CompressionParameter", "localAdaptintegration_tau", "localAdaptintegration_k"], ["const bool", "const float", "const float", "const float", "const float", "const float", "const float", "const float"]), _)]),
	void cv_bioinspired_Retina_setupIPLMagnoChannel_const_bool_const_float_const_float_const_float_const_float_const_float_const_float_const_float(cv::bioinspired::Retina* instance, const bool normaliseOutput, const float parasolCells_beta, const float parasolCells_tau, const float parasolCells_k, const float amacrinCellsTemporalCutFrequency, const float V0CompressionParameter, const float localAdaptintegration_tau, const float localAdaptintegration_k, ResultVoid* ocvrs_return) {
		try {
			instance->setupIPLMagnoChannel(normaliseOutput, parasolCells_beta, parasolCells_tau, parasolCells_k, amacrinCellsTemporalCutFrequency, V0CompressionParameter, localAdaptintegration_tau, localAdaptintegration_k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bioinspired::Retina::setupIPLMagnoChannel() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:281
	// ("cv::bioinspired::Retina::setupIPLMagnoChannel", vec![(pred!(mut, [], []), _)]),
	void cv_bioinspired_Retina_setupIPLMagnoChannel(cv::bioinspired::Retina* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setupIPLMagnoChannel();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:290
	// ("cv::bioinspired::Retina::run", vec![(pred!(mut, ["inputImage"], ["const cv::_InputArray*"]), _)]),
	void cv_bioinspired_Retina_run_const__InputArrayR(cv::bioinspired::Retina* instance, const cv::_InputArray* inputImage, ResultVoid* ocvrs_return) {
		try {
			instance->run(*inputImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// applyFastToneMapping(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:308
	// ("cv::bioinspired::Retina::applyFastToneMapping", vec![(pred!(mut, ["inputImage", "outputToneMappedImage"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_bioinspired_Retina_applyFastToneMapping_const__InputArrayR_const__OutputArrayR(cv::bioinspired::Retina* instance, const cv::_InputArray* inputImage, const cv::_OutputArray* outputToneMappedImage, ResultVoid* ocvrs_return) {
		try {
			instance->applyFastToneMapping(*inputImage, *outputToneMappedImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getParvo(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:322
	// ("cv::bioinspired::Retina::getParvo", vec![(pred!(mut, ["retinaOutput_parvo"], ["const cv::_OutputArray*"]), _)]),
	void cv_bioinspired_Retina_getParvo_const__OutputArrayR(cv::bioinspired::Retina* instance, const cv::_OutputArray* retinaOutput_parvo, ResultVoid* ocvrs_return) {
		try {
			instance->getParvo(*retinaOutput_parvo);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getParvoRAW(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:327
	// ("cv::bioinspired::Retina::getParvoRAW", vec![(pred!(mut, ["retinaOutput_parvo"], ["const cv::_OutputArray*"]), _)]),
	void cv_bioinspired_Retina_getParvoRAW_const__OutputArrayR(cv::bioinspired::Retina* instance, const cv::_OutputArray* retinaOutput_parvo, ResultVoid* ocvrs_return) {
		try {
			instance->getParvoRAW(*retinaOutput_parvo);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMagno(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:339
	// ("cv::bioinspired::Retina::getMagno", vec![(pred!(mut, ["retinaOutput_magno"], ["const cv::_OutputArray*"]), _)]),
	void cv_bioinspired_Retina_getMagno_const__OutputArrayR(cv::bioinspired::Retina* instance, const cv::_OutputArray* retinaOutput_magno, ResultVoid* ocvrs_return) {
		try {
			instance->getMagno(*retinaOutput_magno);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMagnoRAW(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:344
	// ("cv::bioinspired::Retina::getMagnoRAW", vec![(pred!(mut, ["retinaOutput_magno"], ["const cv::_OutputArray*"]), _)]),
	void cv_bioinspired_Retina_getMagnoRAW_const__OutputArrayR(cv::bioinspired::Retina* instance, const cv::_OutputArray* retinaOutput_magno, ResultVoid* ocvrs_return) {
		try {
			instance->getMagnoRAW(*retinaOutput_magno);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getMagnoRAW()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:347
	// ("cv::bioinspired::Retina::getMagnoRAW", vec![(pred!(const, [], []), _)]),
	void cv_bioinspired_Retina_getMagnoRAW_const(const cv::bioinspired::Retina* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMagnoRAW();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getParvoRAW()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:349
	// ("cv::bioinspired::Retina::getParvoRAW", vec![(pred!(const, [], []), _)]),
	void cv_bioinspired_Retina_getParvoRAW_const(const cv::bioinspired::Retina* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getParvoRAW();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setColorSaturation(const bool, const float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:357
	// ("cv::bioinspired::Retina::setColorSaturation", vec![(pred!(mut, ["saturateColors", "colorSaturationValue"], ["const bool", "const float"]), _)]),
	void cv_bioinspired_Retina_setColorSaturation_const_bool_const_float(cv::bioinspired::Retina* instance, const bool saturateColors, const float colorSaturationValue, ResultVoid* ocvrs_return) {
		try {
			instance->setColorSaturation(saturateColors, colorSaturationValue);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bioinspired::Retina::setColorSaturation() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:357
	// ("cv::bioinspired::Retina::setColorSaturation", vec![(pred!(mut, [], []), _)]),
	void cv_bioinspired_Retina_setColorSaturation(cv::bioinspired::Retina* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setColorSaturation();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clearBuffers()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:364
	// ("cv::bioinspired::Retina::clearBuffers", vec![(pred!(mut, [], []), _)]),
	void cv_bioinspired_Retina_clearBuffers(cv::bioinspired::Retina* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clearBuffers();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// activateMovingContoursProcessing(const bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:371
	// ("cv::bioinspired::Retina::activateMovingContoursProcessing", vec![(pred!(mut, ["activate"], ["const bool"]), _)]),
	void cv_bioinspired_Retina_activateMovingContoursProcessing_const_bool(cv::bioinspired::Retina* instance, const bool activate, ResultVoid* ocvrs_return) {
		try {
			instance->activateMovingContoursProcessing(activate);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// activateContoursProcessing(const bool)(Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:379
	// ("cv::bioinspired::Retina::activateContoursProcessing", vec![(pred!(mut, ["activate"], ["const bool"]), _)]),
	void cv_bioinspired_Retina_activateContoursProcessing_const_bool(cv::bioinspired::Retina* instance, const bool activate, ResultVoid* ocvrs_return) {
		try {
			instance->activateContoursProcessing(activate);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:382
	// ("cv::bioinspired::Retina::create", vec![(pred!(mut, ["inputSize"], ["cv::Size"]), _)]),
	void cv_bioinspired_Retina_create_Size(cv::Size* inputSize, Result<cv::Ptr<cv::bioinspired::Retina>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bioinspired::Retina> ret = cv::bioinspired::Retina::create(*inputSize);
			Ok(new cv::Ptr<cv::bioinspired::Retina>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(Size, const bool, int, const bool, const float, const float)(SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:399
	// ("cv::bioinspired::Retina::create", vec![(pred!(mut, ["inputSize", "colorMode", "colorSamplingMethod", "useRetinaLogSampling", "reductionFactor", "samplingStrength"], ["cv::Size", "const bool", "int", "const bool", "const float", "const float"]), _)]),
	void cv_bioinspired_Retina_create_Size_const_bool_int_const_bool_const_float_const_float(cv::Size* inputSize, const bool colorMode, int colorSamplingMethod, const bool useRetinaLogSampling, const float reductionFactor, const float samplingStrength, Result<cv::Ptr<cv::bioinspired::Retina>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bioinspired::Retina> ret = cv::bioinspired::Retina::create(*inputSize, colorMode, colorSamplingMethod, useRetinaLogSampling, reductionFactor, samplingStrength);
			Ok(new cv::Ptr<cv::bioinspired::Retina>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bioinspired::Retina::create(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:399
	// ("cv::bioinspired::Retina::create", vec![(pred!(mut, ["inputSize", "colorMode"], ["cv::Size", "const bool"]), _)]),
	void cv_bioinspired_Retina_create_Size_const_bool(cv::Size* inputSize, const bool colorMode, Result<cv::Ptr<cv::bioinspired::Retina>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bioinspired::Retina> ret = cv::bioinspired::Retina::create(*inputSize, colorMode);
			Ok(new cv::Ptr<cv::bioinspired::Retina>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bioinspired::Retina::to_Algorithm() generated
	// ("cv::bioinspired::Retina::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_bioinspired_Retina_to_Algorithm(cv::bioinspired::Retina* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::bioinspired::Retina::delete() generated
	// ("cv::bioinspired::Retina::delete", vec![(pred!(mut, [], []), _)]),
	void cv_bioinspired_Retina_delete(cv::bioinspired::Retina* instance) {
			delete instance;
	}

	// applyFastToneMapping(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retinafasttonemapping.hpp:119
	// ("cv::bioinspired::RetinaFastToneMapping::applyFastToneMapping", vec![(pred!(mut, ["inputImage", "outputToneMappedImage"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	void cv_bioinspired_RetinaFastToneMapping_applyFastToneMapping_const__InputArrayR_const__OutputArrayR(cv::bioinspired::RetinaFastToneMapping* instance, const cv::_InputArray* inputImage, const cv::_OutputArray* outputToneMappedImage, ResultVoid* ocvrs_return) {
		try {
			instance->applyFastToneMapping(*inputImage, *outputToneMappedImage);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setup(const float, const float, const float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retinafasttonemapping.hpp:128
	// ("cv::bioinspired::RetinaFastToneMapping::setup", vec![(pred!(mut, ["photoreceptorsNeighborhoodRadius", "ganglioncellsNeighborhoodRadius", "meanLuminanceModulatorK"], ["const float", "const float", "const float"]), _)]),
	void cv_bioinspired_RetinaFastToneMapping_setup_const_float_const_float_const_float(cv::bioinspired::RetinaFastToneMapping* instance, const float photoreceptorsNeighborhoodRadius, const float ganglioncellsNeighborhoodRadius, const float meanLuminanceModulatorK, ResultVoid* ocvrs_return) {
		try {
			instance->setup(photoreceptorsNeighborhoodRadius, ganglioncellsNeighborhoodRadius, meanLuminanceModulatorK);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bioinspired::RetinaFastToneMapping::setup() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retinafasttonemapping.hpp:128
	// ("cv::bioinspired::RetinaFastToneMapping::setup", vec![(pred!(mut, [], []), _)]),
	void cv_bioinspired_RetinaFastToneMapping_setup(cv::bioinspired::RetinaFastToneMapping* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setup();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retinafasttonemapping.hpp:130
	// ("cv::bioinspired::RetinaFastToneMapping::create", vec![(pred!(mut, ["inputSize"], ["cv::Size"]), _)]),
	void cv_bioinspired_RetinaFastToneMapping_create_Size(cv::Size* inputSize, Result<cv::Ptr<cv::bioinspired::RetinaFastToneMapping>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bioinspired::RetinaFastToneMapping> ret = cv::bioinspired::RetinaFastToneMapping::create(*inputSize);
			Ok(new cv::Ptr<cv::bioinspired::RetinaFastToneMapping>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bioinspired::RetinaFastToneMapping::to_Algorithm() generated
	// ("cv::bioinspired::RetinaFastToneMapping::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_bioinspired_RetinaFastToneMapping_to_Algorithm(cv::bioinspired::RetinaFastToneMapping* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::bioinspired::RetinaFastToneMapping::delete() generated
	// ("cv::bioinspired::RetinaFastToneMapping::delete", vec![(pred!(mut, [], []), _)]),
	void cv_bioinspired_RetinaFastToneMapping_delete(cv::bioinspired::RetinaFastToneMapping* instance) {
			delete instance;
	}

	// cv::bioinspired::RetinaParameters::implicitClone() generated
	// ("cv::bioinspired::RetinaParameters::implicitClone", vec![(pred!(const, [], []), _)]),
	cv::bioinspired::RetinaParameters* cv_bioinspired_RetinaParameters_implicitClone_const(const cv::bioinspired::RetinaParameters* instance) {
			return new cv::bioinspired::RetinaParameters(*instance);
	}

	// cv::bioinspired::RetinaParameters::defaultNew() generated
	// ("cv::bioinspired::RetinaParameters::defaultNew", vec![(pred!(const, [], []), _)]),
	cv::bioinspired::RetinaParameters* cv_bioinspired_RetinaParameters_defaultNew_const() {
			cv::bioinspired::RetinaParameters* ret = new cv::bioinspired::RetinaParameters();
			return ret;
	}

	// cv::bioinspired::RetinaParameters::OPLandIplParvo() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:133
	// ("cv::bioinspired::RetinaParameters::OPLandIplParvo", vec![(pred!(const, [], []), _)]),
	void cv_bioinspired_RetinaParameters_propOPLandIplParvo_const(const cv::bioinspired::RetinaParameters* instance, cv::bioinspired::RetinaParameters::OPLandIplParvoParameters* ocvrs_return) {
			cv::bioinspired::RetinaParameters::OPLandIplParvoParameters ret = instance->OPLandIplParvo;
			*ocvrs_return = ret;
	}

	// cv::bioinspired::RetinaParameters::setOPLandIplParvo(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:133
	// ("cv::bioinspired::RetinaParameters::setOPLandIplParvo", vec![(pred!(mut, ["val"], ["const cv::bioinspired::RetinaParameters::OPLandIplParvoParameters"]), _)]),
	void cv_bioinspired_RetinaParameters_propOPLandIplParvo_const_OPLandIplParvoParameters(cv::bioinspired::RetinaParameters* instance, const cv::bioinspired::RetinaParameters::OPLandIplParvoParameters* val) {
			instance->OPLandIplParvo = *val;
	}

	// cv::bioinspired::RetinaParameters::IplMagno() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:134
	// ("cv::bioinspired::RetinaParameters::IplMagno", vec![(pred!(const, [], []), _)]),
	void cv_bioinspired_RetinaParameters_propIplMagno_const(const cv::bioinspired::RetinaParameters* instance, cv::bioinspired::RetinaParameters::IplMagnoParameters* ocvrs_return) {
			cv::bioinspired::RetinaParameters::IplMagnoParameters ret = instance->IplMagno;
			*ocvrs_return = ret;
	}

	// cv::bioinspired::RetinaParameters::setIplMagno(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:134
	// ("cv::bioinspired::RetinaParameters::setIplMagno", vec![(pred!(mut, ["val"], ["const cv::bioinspired::RetinaParameters::IplMagnoParameters"]), _)]),
	void cv_bioinspired_RetinaParameters_propIplMagno_const_IplMagnoParameters(cv::bioinspired::RetinaParameters* instance, const cv::bioinspired::RetinaParameters::IplMagnoParameters* val) {
			instance->IplMagno = *val;
	}

	// cv::bioinspired::RetinaParameters::delete() generated
	// ("cv::bioinspired::RetinaParameters::delete", vec![(pred!(mut, [], []), _)]),
	void cv_bioinspired_RetinaParameters_delete(cv::bioinspired::RetinaParameters* instance) {
			delete instance;
	}

	// IplMagnoParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:121
	// ("cv::bioinspired::RetinaParameters::IplMagnoParameters::IplMagnoParameters", vec![(pred!(mut, [], []), _)]),
	void cv_bioinspired_RetinaParameters_IplMagnoParameters_IplMagnoParameters(Result<cv::bioinspired::RetinaParameters::IplMagnoParameters>* ocvrs_return) {
		try {
			cv::bioinspired::RetinaParameters::IplMagnoParameters ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// OPLandIplParvoParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/retina.hpp:107
	// ("cv::bioinspired::RetinaParameters::OPLandIplParvoParameters::OPLandIplParvoParameters", vec![(pred!(mut, [], []), _)]),
	void cv_bioinspired_RetinaParameters_OPLandIplParvoParameters_OPLandIplParvoParameters(Result<cv::bioinspired::RetinaParameters::OPLandIplParvoParameters>* ocvrs_return) {
		try {
			cv::bioinspired::RetinaParameters::OPLandIplParvoParameters ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// SegmentationParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/transientareassegmentationmodule.hpp:84
	// ("cv::bioinspired::SegmentationParameters::SegmentationParameters", vec![(pred!(mut, [], []), _)]),
	void cv_bioinspired_SegmentationParameters_SegmentationParameters(Result<cv::bioinspired::SegmentationParameters>* ocvrs_return) {
		try {
			cv::bioinspired::SegmentationParameters ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSize()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/transientareassegmentationmodule.hpp:129
	// ("cv::bioinspired::TransientAreasSegmentationModule::getSize", vec![(pred!(mut, [], []), _)]),
	void cv_bioinspired_TransientAreasSegmentationModule_getSize(cv::bioinspired::TransientAreasSegmentationModule* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setup(String, const bool)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/transientareassegmentationmodule.hpp:138
	// ("cv::bioinspired::TransientAreasSegmentationModule::setup", vec![(pred!(mut, ["segmentationParameterFile", "applyDefaultSetupOnFailure"], ["cv::String", "const bool"]), _)]),
	void cv_bioinspired_TransientAreasSegmentationModule_setup_String_const_bool(cv::bioinspired::TransientAreasSegmentationModule* instance, const char* segmentationParameterFile, const bool applyDefaultSetupOnFailure, ResultVoid* ocvrs_return) {
		try {
			instance->setup(std::string(segmentationParameterFile), applyDefaultSetupOnFailure);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bioinspired::TransientAreasSegmentationModule::setup() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/transientareassegmentationmodule.hpp:138
	// ("cv::bioinspired::TransientAreasSegmentationModule::setup", vec![(pred!(mut, [], []), _)]),
	void cv_bioinspired_TransientAreasSegmentationModule_setup(cv::bioinspired::TransientAreasSegmentationModule* instance, ResultVoid* ocvrs_return) {
		try {
			instance->setup();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setup(cv::FileStorage &, const bool)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/transientareassegmentationmodule.hpp:147
	// ("cv::bioinspired::TransientAreasSegmentationModule::setup", vec![(pred!(mut, ["fs", "applyDefaultSetupOnFailure"], ["cv::FileStorage*", "const bool"]), _)]),
	void cv_bioinspired_TransientAreasSegmentationModule_setup_FileStorageR_const_bool(cv::bioinspired::TransientAreasSegmentationModule* instance, cv::FileStorage* fs, const bool applyDefaultSetupOnFailure, ResultVoid* ocvrs_return) {
		try {
			instance->setup(*fs, applyDefaultSetupOnFailure);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bioinspired::TransientAreasSegmentationModule::setup(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/transientareassegmentationmodule.hpp:147
	// ("cv::bioinspired::TransientAreasSegmentationModule::setup", vec![(pred!(mut, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_bioinspired_TransientAreasSegmentationModule_setup_FileStorageR(cv::bioinspired::TransientAreasSegmentationModule* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->setup(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// setup(SegmentationParameters)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/transientareassegmentationmodule.hpp:155
	// ("cv::bioinspired::TransientAreasSegmentationModule::setup", vec![(pred!(mut, ["newParameters"], ["cv::bioinspired::SegmentationParameters"]), _)]),
	void cv_bioinspired_TransientAreasSegmentationModule_setup_SegmentationParameters(cv::bioinspired::TransientAreasSegmentationModule* instance, cv::bioinspired::SegmentationParameters* newParameters, ResultVoid* ocvrs_return) {
		try {
			instance->setup(*newParameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getParameters()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/transientareassegmentationmodule.hpp:159
	// ("cv::bioinspired::TransientAreasSegmentationModule::getParameters", vec![(pred!(mut, [], []), _)]),
	void cv_bioinspired_TransientAreasSegmentationModule_getParameters(cv::bioinspired::TransientAreasSegmentationModule* instance, Result<cv::bioinspired::SegmentationParameters>* ocvrs_return) {
		try {
			cv::bioinspired::SegmentationParameters ret = instance->getParameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// printSetup()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/transientareassegmentationmodule.hpp:164
	// ("cv::bioinspired::TransientAreasSegmentationModule::printSetup", vec![(pred!(mut, [], []), _)]),
	void cv_bioinspired_TransientAreasSegmentationModule_printSetup(cv::bioinspired::TransientAreasSegmentationModule* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->printSetup();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(String)(InString) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/transientareassegmentationmodule.hpp:169
	// ("cv::bioinspired::TransientAreasSegmentationModule::write", vec![(pred!(const, ["fs"], ["cv::String"]), _)]),
	void cv_bioinspired_TransientAreasSegmentationModule_write_const_String(const cv::bioinspired::TransientAreasSegmentationModule* instance, const char* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(std::string(fs));
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// write(cv::FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/transientareassegmentationmodule.hpp:174
	// ("cv::bioinspired::TransientAreasSegmentationModule::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	void cv_bioinspired_TransientAreasSegmentationModule_write_const_FileStorageR(const cv::bioinspired::TransientAreasSegmentationModule* instance, cv::FileStorage* fs, ResultVoid* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// run(InputArray, const int)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/transientareassegmentationmodule.hpp:180
	// ("cv::bioinspired::TransientAreasSegmentationModule::run", vec![(pred!(mut, ["inputToSegment", "channelIndex"], ["const cv::_InputArray*", "const int"]), _)]),
	void cv_bioinspired_TransientAreasSegmentationModule_run_const__InputArrayR_const_int(cv::bioinspired::TransientAreasSegmentationModule* instance, const cv::_InputArray* inputToSegment, const int channelIndex, ResultVoid* ocvrs_return) {
		try {
			instance->run(*inputToSegment, channelIndex);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bioinspired::TransientAreasSegmentationModule::run(InputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/transientareassegmentationmodule.hpp:180
	// ("cv::bioinspired::TransientAreasSegmentationModule::run", vec![(pred!(mut, ["inputToSegment"], ["const cv::_InputArray*"]), _)]),
	void cv_bioinspired_TransientAreasSegmentationModule_run_const__InputArrayR(cv::bioinspired::TransientAreasSegmentationModule* instance, const cv::_InputArray* inputToSegment, ResultVoid* ocvrs_return) {
		try {
			instance->run(*inputToSegment);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// getSegmentationPicture(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/transientareassegmentationmodule.hpp:185
	// ("cv::bioinspired::TransientAreasSegmentationModule::getSegmentationPicture", vec![(pred!(mut, ["transientAreas"], ["const cv::_OutputArray*"]), _)]),
	void cv_bioinspired_TransientAreasSegmentationModule_getSegmentationPicture_const__OutputArrayR(cv::bioinspired::TransientAreasSegmentationModule* instance, const cv::_OutputArray* transientAreas, ResultVoid* ocvrs_return) {
		try {
			instance->getSegmentationPicture(*transientAreas);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// clearAllBuffers()() /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/transientareassegmentationmodule.hpp:189
	// ("cv::bioinspired::TransientAreasSegmentationModule::clearAllBuffers", vec![(pred!(mut, [], []), _)]),
	void cv_bioinspired_TransientAreasSegmentationModule_clearAllBuffers(cv::bioinspired::TransientAreasSegmentationModule* instance, ResultVoid* ocvrs_return) {
		try {
			instance->clearAllBuffers();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// create(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-4/install/include/opencv4/opencv2/bioinspired/transientareassegmentationmodule.hpp:194
	// ("cv::bioinspired::TransientAreasSegmentationModule::create", vec![(pred!(mut, ["inputSize"], ["cv::Size"]), _)]),
	void cv_bioinspired_TransientAreasSegmentationModule_create_Size(cv::Size* inputSize, Result<cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule> ret = cv::bioinspired::TransientAreasSegmentationModule::create(*inputSize);
			Ok(new cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}

	// cv::bioinspired::TransientAreasSegmentationModule::to_Algorithm() generated
	// ("cv::bioinspired::TransientAreasSegmentationModule::to_Algorithm", vec![(pred!(mut, [], []), _)]),
	cv::Algorithm* cv_bioinspired_TransientAreasSegmentationModule_to_Algorithm(cv::bioinspired::TransientAreasSegmentationModule* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}

	// cv::bioinspired::TransientAreasSegmentationModule::delete() generated
	// ("cv::bioinspired::TransientAreasSegmentationModule::delete", vec![(pred!(mut, [], []), _)]),
	void cv_bioinspired_TransientAreasSegmentationModule_delete(cv::bioinspired::TransientAreasSegmentationModule* instance) {
			delete instance;
	}

}
