#include "bioinspired.hpp"
#include "bioinspired_types.hpp"

extern "C" {
	Result<cv::Size> cv_bioinspired_Retina_getInputSize(cv::bioinspired::Retina* instance) {
		try {
			cv::Size ret = instance->getInputSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result<cv::Size> cv_bioinspired_Retina_getOutputSize(cv::bioinspired::Retina* instance) {
		try {
			cv::Size ret = instance->getOutputSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_bioinspired_Retina_setup_String_bool(cv::bioinspired::Retina* instance, char* retinaParameterFile, bool applyDefaultSetupOnFailure) {
		try {
			instance->setup(std::string(retinaParameterFile), applyDefaultSetupOnFailure);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_setup_FileStorageX_bool(cv::bioinspired::Retina* instance, cv::FileStorage* fs, bool applyDefaultSetupOnFailure) {
		try {
			instance->setup(*fs, applyDefaultSetupOnFailure);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_setup_RetinaParameters(cv::bioinspired::Retina* instance, cv::bioinspired::RetinaParameters* newParameters) {
		try {
			instance->setup(*newParameters);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::bioinspired::RetinaParameters*> cv_bioinspired_Retina_getParameters(cv::bioinspired::Retina* instance) {
		try {
			cv::bioinspired::RetinaParameters ret = instance->getParameters();
			return Ok(new cv::bioinspired::RetinaParameters(ret));
		} OCVRS_CATCH(Result<cv::bioinspired::RetinaParameters*>)
	}
	
	Result<void*> cv_bioinspired_Retina_printSetup(cv::bioinspired::Retina* instance) {
		try {
			cv::String ret = instance->printSetup();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_bioinspired_Retina_write_const_String(const cv::bioinspired::Retina* instance, char* fs) {
		try {
			instance->write(std::string(fs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_write_const_FileStorageX(const cv::bioinspired::Retina* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_setupOPLandIPLParvoChannel_bool_bool_float_float_float_float_float_float_float(cv::bioinspired::Retina* instance, bool colorMode, bool normaliseOutput, float photoreceptorsLocalAdaptationSensitivity, float photoreceptorsTemporalConstant, float photoreceptorsSpatialConstant, float horizontalCellsGain, float HcellsTemporalConstant, float HcellsSpatialConstant, float ganglionCellsSensitivity) {
		try {
			instance->setupOPLandIPLParvoChannel(colorMode, normaliseOutput, photoreceptorsLocalAdaptationSensitivity, photoreceptorsTemporalConstant, photoreceptorsSpatialConstant, horizontalCellsGain, HcellsTemporalConstant, HcellsSpatialConstant, ganglionCellsSensitivity);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_setupIPLMagnoChannel_bool_float_float_float_float_float_float_float(cv::bioinspired::Retina* instance, bool normaliseOutput, float parasolCells_beta, float parasolCells_tau, float parasolCells_k, float amacrinCellsTemporalCutFrequency, float V0CompressionParameter, float localAdaptintegration_tau, float localAdaptintegration_k) {
		try {
			instance->setupIPLMagnoChannel(normaliseOutput, parasolCells_beta, parasolCells_tau, parasolCells_k, amacrinCellsTemporalCutFrequency, V0CompressionParameter, localAdaptintegration_tau, localAdaptintegration_k);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_run_const__InputArrayX(cv::bioinspired::Retina* instance, const cv::_InputArray* inputImage) {
		try {
			instance->run(*inputImage);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_applyFastToneMapping_const__InputArrayX_const__OutputArrayX(cv::bioinspired::Retina* instance, const cv::_InputArray* inputImage, const cv::_OutputArray* outputToneMappedImage) {
		try {
			instance->applyFastToneMapping(*inputImage, *outputToneMappedImage);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_getParvo_const__OutputArrayX(cv::bioinspired::Retina* instance, const cv::_OutputArray* retinaOutput_parvo) {
		try {
			instance->getParvo(*retinaOutput_parvo);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_getParvoRAW_const__OutputArrayX(cv::bioinspired::Retina* instance, const cv::_OutputArray* retinaOutput_parvo) {
		try {
			instance->getParvoRAW(*retinaOutput_parvo);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_getMagno_const__OutputArrayX(cv::bioinspired::Retina* instance, const cv::_OutputArray* retinaOutput_magno) {
		try {
			instance->getMagno(*retinaOutput_magno);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_getMagnoRAW_const__OutputArrayX(cv::bioinspired::Retina* instance, const cv::_OutputArray* retinaOutput_magno) {
		try {
			instance->getMagnoRAW(*retinaOutput_magno);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Mat*> cv_bioinspired_Retina_getMagnoRAW_const(const cv::bioinspired::Retina* instance) {
		try {
			cv::Mat ret = instance->getMagnoRAW();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result<cv::Mat*> cv_bioinspired_Retina_getParvoRAW_const(const cv::bioinspired::Retina* instance) {
		try {
			cv::Mat ret = instance->getParvoRAW();
			return Ok(new cv::Mat(ret));
		} OCVRS_CATCH(Result<cv::Mat*>)
	}
	
	Result_void cv_bioinspired_Retina_setColorSaturation_bool_float(cv::bioinspired::Retina* instance, bool saturateColors, float colorSaturationValue) {
		try {
			instance->setColorSaturation(saturateColors, colorSaturationValue);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_clearBuffers(cv::bioinspired::Retina* instance) {
		try {
			instance->clearBuffers();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_activateMovingContoursProcessing_bool(cv::bioinspired::Retina* instance, bool activate) {
		try {
			instance->activateMovingContoursProcessing(activate);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_activateContoursProcessing_bool(cv::bioinspired::Retina* instance, bool activate) {
		try {
			instance->activateContoursProcessing(activate);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::bioinspired::Retina>*> cv_bioinspired_Retina_create_Size(const cv::Size* inputSize) {
		try {
			cv::Ptr<cv::bioinspired::Retina> ret = cv::bioinspired::Retina::create(*inputSize);
			return Ok(new cv::Ptr<cv::bioinspired::Retina>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::bioinspired::Retina>*>)
	}
	
	Result<cv::Ptr<cv::bioinspired::Retina>*> cv_bioinspired_Retina_create_Size_bool_int_bool_float_float(const cv::Size* inputSize, bool colorMode, int colorSamplingMethod, bool useRetinaLogSampling, float reductionFactor, float samplingStrenght) {
		try {
			cv::Ptr<cv::bioinspired::Retina> ret = cv::bioinspired::Retina::create(*inputSize, colorMode, colorSamplingMethod, useRetinaLogSampling, reductionFactor, samplingStrenght);
			return Ok(new cv::Ptr<cv::bioinspired::Retina>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::bioinspired::Retina>*>)
	}
	
	Result_void cv_bioinspired_RetinaFastToneMapping_applyFastToneMapping_const__InputArrayX_const__OutputArrayX(cv::bioinspired::RetinaFastToneMapping* instance, const cv::_InputArray* inputImage, const cv::_OutputArray* outputToneMappedImage) {
		try {
			instance->applyFastToneMapping(*inputImage, *outputToneMappedImage);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_RetinaFastToneMapping_setup_float_float_float(cv::bioinspired::RetinaFastToneMapping* instance, float photoreceptorsNeighborhoodRadius, float ganglioncellsNeighborhoodRadius, float meanLuminanceModulatorK) {
		try {
			instance->setup(photoreceptorsNeighborhoodRadius, ganglioncellsNeighborhoodRadius, meanLuminanceModulatorK);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::bioinspired::RetinaFastToneMapping>*> cv_bioinspired_RetinaFastToneMapping_create_Size(const cv::Size* inputSize) {
		try {
			cv::Ptr<cv::bioinspired::RetinaFastToneMapping> ret = cv::bioinspired::RetinaFastToneMapping::create(*inputSize);
			return Ok(new cv::Ptr<cv::bioinspired::RetinaFastToneMapping>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::bioinspired::RetinaFastToneMapping>*>)
	}
	
	Result<cv::bioinspired::RetinaParameters::OPLandIplParvoParameters> cv_bioinspired_RetinaParameters_OPLandIplParvo_const(const cv::bioinspired::RetinaParameters* instance) {
		try {
			cv::bioinspired::RetinaParameters::OPLandIplParvoParameters ret = instance->OPLandIplParvo;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::bioinspired::RetinaParameters::OPLandIplParvoParameters>)
	}
	
	Result_void cv_bioinspired_RetinaParameters_setOPLandIplParvo_OPLandIplParvoParameters(cv::bioinspired::RetinaParameters* instance, const cv::bioinspired::RetinaParameters::OPLandIplParvoParameters* val) {
		try {
			instance->OPLandIplParvo = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::bioinspired::RetinaParameters::IplMagnoParameters> cv_bioinspired_RetinaParameters_IplMagno_const(const cv::bioinspired::RetinaParameters* instance) {
		try {
			cv::bioinspired::RetinaParameters::IplMagnoParameters ret = instance->IplMagno;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::bioinspired::RetinaParameters::IplMagnoParameters>)
	}
	
	Result_void cv_bioinspired_RetinaParameters_setIplMagno_IplMagnoParameters(cv::bioinspired::RetinaParameters* instance, const cv::bioinspired::RetinaParameters::IplMagnoParameters* val) {
		try {
			instance->IplMagno = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_RetinaParameters_delete(cv::bioinspired::RetinaParameters* instance) {
		delete instance;
	}
	Result<cv::bioinspired::RetinaParameters::IplMagnoParameters> cv_bioinspired_RetinaParameters_IplMagnoParameters_IplMagnoParameters() {
		try {
			cv::bioinspired::RetinaParameters::IplMagnoParameters ret;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::bioinspired::RetinaParameters::IplMagnoParameters>)
	}
	
	Result<cv::bioinspired::RetinaParameters::OPLandIplParvoParameters> cv_bioinspired_RetinaParameters_OPLandIplParvoParameters_OPLandIplParvoParameters() {
		try {
			cv::bioinspired::RetinaParameters::OPLandIplParvoParameters ret;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::bioinspired::RetinaParameters::OPLandIplParvoParameters>)
	}
	
	Result<cv::bioinspired::SegmentationParameters> cv_bioinspired_SegmentationParameters_SegmentationParameters() {
		try {
			cv::bioinspired::SegmentationParameters ret;
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::bioinspired::SegmentationParameters>)
	}
	
	Result<cv::Size> cv_bioinspired_TransientAreasSegmentationModule_getSize(cv::bioinspired::TransientAreasSegmentationModule* instance) {
		try {
			cv::Size ret = instance->getSize();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_setup_String_bool(cv::bioinspired::TransientAreasSegmentationModule* instance, char* segmentationParameterFile, bool applyDefaultSetupOnFailure) {
		try {
			instance->setup(std::string(segmentationParameterFile), applyDefaultSetupOnFailure);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_setup_FileStorageX_bool(cv::bioinspired::TransientAreasSegmentationModule* instance, cv::FileStorage* fs, bool applyDefaultSetupOnFailure) {
		try {
			instance->setup(*fs, applyDefaultSetupOnFailure);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_setup_SegmentationParameters(cv::bioinspired::TransientAreasSegmentationModule* instance, const cv::bioinspired::SegmentationParameters* newParameters) {
		try {
			instance->setup(*newParameters);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::bioinspired::SegmentationParameters> cv_bioinspired_TransientAreasSegmentationModule_getParameters(cv::bioinspired::TransientAreasSegmentationModule* instance) {
		try {
			cv::bioinspired::SegmentationParameters ret = instance->getParameters();
			return Ok(ret);
		} OCVRS_CATCH(Result<cv::bioinspired::SegmentationParameters>)
	}
	
	Result<void*> cv_bioinspired_TransientAreasSegmentationModule_printSetup(cv::bioinspired::TransientAreasSegmentationModule* instance) {
		try {
			cv::String ret = instance->printSetup();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_write_const_String(const cv::bioinspired::TransientAreasSegmentationModule* instance, char* fs) {
		try {
			instance->write(std::string(fs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_write_const_FileStorageX(const cv::bioinspired::TransientAreasSegmentationModule* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_run_const__InputArrayX_int(cv::bioinspired::TransientAreasSegmentationModule* instance, const cv::_InputArray* inputToSegment, int channelIndex) {
		try {
			instance->run(*inputToSegment, channelIndex);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_getSegmentationPicture_const__OutputArrayX(cv::bioinspired::TransientAreasSegmentationModule* instance, const cv::_OutputArray* transientAreas) {
		try {
			instance->getSegmentationPicture(*transientAreas);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_clearAllBuffers(cv::bioinspired::TransientAreasSegmentationModule* instance) {
		try {
			instance->clearAllBuffers();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>*> cv_bioinspired_TransientAreasSegmentationModule_create_Size(const cv::Size* inputSize) {
		try {
			cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule> ret = cv::bioinspired::TransientAreasSegmentationModule::create(*inputSize);
			return Ok(new cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>(ret));
		} OCVRS_CATCH(Result<cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>*>)
	}
	
}
