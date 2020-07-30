#include "bioinspired.hpp"
#include "bioinspired_types.hpp"

extern "C" {
	Result<cv::Size> cv_bioinspired_Retina_getInputSize(cv::bioinspired::Retina* instance) {
		try {
			cv::Size ret = instance->getInputSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result<cv::Size> cv_bioinspired_Retina_getOutputSize(cv::bioinspired::Retina* instance) {
		try {
			cv::Size ret = instance->getOutputSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_bioinspired_Retina_setup_String_const_bool(cv::bioinspired::Retina* instance, char* retinaParameterFile, const bool applyDefaultSetupOnFailure) {
		try {
			instance->setup(cv::String(retinaParameterFile), applyDefaultSetupOnFailure);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bioinspired_Retina_setup_FileStorageR_const_bool(cv::bioinspired::Retina* instance, cv::FileStorage* fs, const bool applyDefaultSetupOnFailure) {
		try {
			instance->setup(*fs, applyDefaultSetupOnFailure);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bioinspired_Retina_setup_RetinaParameters(cv::bioinspired::Retina* instance, cv::bioinspired::RetinaParameters* newParameters) {
		try {
			instance->setup(*newParameters);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::bioinspired::RetinaParameters*> cv_bioinspired_Retina_getParameters(cv::bioinspired::Retina* instance) {
		try {
			cv::bioinspired::RetinaParameters ret = instance->getParameters();
			return Ok(new cv::bioinspired::RetinaParameters(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::bioinspired::RetinaParameters*>))
	}
	
	Result<void*> cv_bioinspired_Retina_printSetup(cv::bioinspired::Retina* instance) {
		try {
			const cv::String ret = instance->printSetup();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_bioinspired_Retina_write_const_String(const cv::bioinspired::Retina* instance, char* fs) {
		try {
			instance->write(cv::String(fs));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bioinspired_Retina_write_const_FileStorageR(const cv::bioinspired::Retina* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bioinspired_Retina_setupOPLandIPLParvoChannel_const_bool_const_bool_const_float_const_float_const_float_const_float_const_float_const_float_const_float(cv::bioinspired::Retina* instance, const bool colorMode, const bool normaliseOutput, const float photoreceptorsLocalAdaptationSensitivity, const float photoreceptorsTemporalConstant, const float photoreceptorsSpatialConstant, const float horizontalCellsGain, const float HcellsTemporalConstant, const float HcellsSpatialConstant, const float ganglionCellsSensitivity) {
		try {
			instance->setupOPLandIPLParvoChannel(colorMode, normaliseOutput, photoreceptorsLocalAdaptationSensitivity, photoreceptorsTemporalConstant, photoreceptorsSpatialConstant, horizontalCellsGain, HcellsTemporalConstant, HcellsSpatialConstant, ganglionCellsSensitivity);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bioinspired_Retina_setupIPLMagnoChannel_const_bool_const_float_const_float_const_float_const_float_const_float_const_float_const_float(cv::bioinspired::Retina* instance, const bool normaliseOutput, const float parasolCells_beta, const float parasolCells_tau, const float parasolCells_k, const float amacrinCellsTemporalCutFrequency, const float V0CompressionParameter, const float localAdaptintegration_tau, const float localAdaptintegration_k) {
		try {
			instance->setupIPLMagnoChannel(normaliseOutput, parasolCells_beta, parasolCells_tau, parasolCells_k, amacrinCellsTemporalCutFrequency, V0CompressionParameter, localAdaptintegration_tau, localAdaptintegration_k);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bioinspired_Retina_run_const__InputArrayR(cv::bioinspired::Retina* instance, const cv::_InputArray* inputImage) {
		try {
			instance->run(*inputImage);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bioinspired_Retina_applyFastToneMapping_const__InputArrayR_const__OutputArrayR(cv::bioinspired::Retina* instance, const cv::_InputArray* inputImage, const cv::_OutputArray* outputToneMappedImage) {
		try {
			instance->applyFastToneMapping(*inputImage, *outputToneMappedImage);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bioinspired_Retina_getParvo_const__OutputArrayR(cv::bioinspired::Retina* instance, const cv::_OutputArray* retinaOutput_parvo) {
		try {
			instance->getParvo(*retinaOutput_parvo);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bioinspired_Retina_getParvoRAW_const__OutputArrayR(cv::bioinspired::Retina* instance, const cv::_OutputArray* retinaOutput_parvo) {
		try {
			instance->getParvoRAW(*retinaOutput_parvo);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bioinspired_Retina_getMagno_const__OutputArrayR(cv::bioinspired::Retina* instance, const cv::_OutputArray* retinaOutput_magno) {
		try {
			instance->getMagno(*retinaOutput_magno);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bioinspired_Retina_getMagnoRAW_const__OutputArrayR(cv::bioinspired::Retina* instance, const cv::_OutputArray* retinaOutput_magno) {
		try {
			instance->getMagnoRAW(*retinaOutput_magno);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<const cv::Mat*> cv_bioinspired_Retina_getMagnoRAW_const(const cv::bioinspired::Retina* instance) {
		try {
			const cv::Mat ret = instance->getMagnoRAW();
			return Ok(new const cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::Mat*>))
	}
	
	Result<const cv::Mat*> cv_bioinspired_Retina_getParvoRAW_const(const cv::bioinspired::Retina* instance) {
		try {
			const cv::Mat ret = instance->getParvoRAW();
			return Ok(new const cv::Mat(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::Mat*>))
	}
	
	Result_void cv_bioinspired_Retina_setColorSaturation_const_bool_const_float(cv::bioinspired::Retina* instance, const bool saturateColors, const float colorSaturationValue) {
		try {
			instance->setColorSaturation(saturateColors, colorSaturationValue);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bioinspired_Retina_clearBuffers(cv::bioinspired::Retina* instance) {
		try {
			instance->clearBuffers();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bioinspired_Retina_activateMovingContoursProcessing_const_bool(cv::bioinspired::Retina* instance, const bool activate) {
		try {
			instance->activateMovingContoursProcessing(activate);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bioinspired_Retina_activateContoursProcessing_const_bool(cv::bioinspired::Retina* instance, const bool activate) {
		try {
			instance->activateContoursProcessing(activate);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::bioinspired::Retina>*> cv_bioinspired_Retina_create_Size(cv::Size* inputSize) {
		try {
			cv::Ptr<cv::bioinspired::Retina> ret = cv::bioinspired::Retina::create(*inputSize);
			return Ok(new cv::Ptr<cv::bioinspired::Retina>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::bioinspired::Retina>*>))
	}
	
	Result<cv::Ptr<cv::bioinspired::Retina>*> cv_bioinspired_Retina_create_Size_const_bool_int_const_bool_const_float_const_float(cv::Size* inputSize, const bool colorMode, int colorSamplingMethod, const bool useRetinaLogSampling, const float reductionFactor, const float samplingStrength) {
		try {
			cv::Ptr<cv::bioinspired::Retina> ret = cv::bioinspired::Retina::create(*inputSize, colorMode, colorSamplingMethod, useRetinaLogSampling, reductionFactor, samplingStrength);
			return Ok(new cv::Ptr<cv::bioinspired::Retina>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::bioinspired::Retina>*>))
	}
	
	Result_void cv_bioinspired_RetinaFastToneMapping_applyFastToneMapping_const__InputArrayR_const__OutputArrayR(cv::bioinspired::RetinaFastToneMapping* instance, const cv::_InputArray* inputImage, const cv::_OutputArray* outputToneMappedImage) {
		try {
			instance->applyFastToneMapping(*inputImage, *outputToneMappedImage);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bioinspired_RetinaFastToneMapping_setup_const_float_const_float_const_float(cv::bioinspired::RetinaFastToneMapping* instance, const float photoreceptorsNeighborhoodRadius, const float ganglioncellsNeighborhoodRadius, const float meanLuminanceModulatorK) {
		try {
			instance->setup(photoreceptorsNeighborhoodRadius, ganglioncellsNeighborhoodRadius, meanLuminanceModulatorK);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::bioinspired::RetinaFastToneMapping>*> cv_bioinspired_RetinaFastToneMapping_create_Size(cv::Size* inputSize) {
		try {
			cv::Ptr<cv::bioinspired::RetinaFastToneMapping> ret = cv::bioinspired::RetinaFastToneMapping::create(*inputSize);
			return Ok(new cv::Ptr<cv::bioinspired::RetinaFastToneMapping>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::bioinspired::RetinaFastToneMapping>*>))
	}
	
	Result<cv::bioinspired::RetinaParameters::OPLandIplParvoParameters> cv_bioinspired_RetinaParameters_getPropOPLandIplParvo_const(const cv::bioinspired::RetinaParameters* instance) {
		try {
			cv::bioinspired::RetinaParameters::OPLandIplParvoParameters ret = instance->OPLandIplParvo;
			return Ok<cv::bioinspired::RetinaParameters::OPLandIplParvoParameters>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::bioinspired::RetinaParameters::OPLandIplParvoParameters>))
	}
	
	Result_void cv_bioinspired_RetinaParameters_setPropOPLandIplParvo_OPLandIplParvoParameters(cv::bioinspired::RetinaParameters* instance, cv::bioinspired::RetinaParameters::OPLandIplParvoParameters* val) {
		try {
			instance->OPLandIplParvo = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::bioinspired::RetinaParameters::IplMagnoParameters> cv_bioinspired_RetinaParameters_getPropIplMagno_const(const cv::bioinspired::RetinaParameters* instance) {
		try {
			cv::bioinspired::RetinaParameters::IplMagnoParameters ret = instance->IplMagno;
			return Ok<cv::bioinspired::RetinaParameters::IplMagnoParameters>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::bioinspired::RetinaParameters::IplMagnoParameters>))
	}
	
	Result_void cv_bioinspired_RetinaParameters_setPropIplMagno_IplMagnoParameters(cv::bioinspired::RetinaParameters* instance, cv::bioinspired::RetinaParameters::IplMagnoParameters* val) {
		try {
			instance->IplMagno = *val;
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_RetinaParameters_delete(cv::bioinspired::RetinaParameters* instance) {
		delete instance;
	}
	Result<cv::bioinspired::RetinaParameters::IplMagnoParameters> cv_bioinspired_RetinaParameters_IplMagnoParameters_IplMagnoParameters() {
		try {
			cv::bioinspired::RetinaParameters::IplMagnoParameters ret;
			return Ok<cv::bioinspired::RetinaParameters::IplMagnoParameters>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::bioinspired::RetinaParameters::IplMagnoParameters>))
	}
	
	Result<cv::bioinspired::RetinaParameters::OPLandIplParvoParameters> cv_bioinspired_RetinaParameters_OPLandIplParvoParameters_OPLandIplParvoParameters() {
		try {
			cv::bioinspired::RetinaParameters::OPLandIplParvoParameters ret;
			return Ok<cv::bioinspired::RetinaParameters::OPLandIplParvoParameters>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::bioinspired::RetinaParameters::OPLandIplParvoParameters>))
	}
	
	Result<cv::bioinspired::SegmentationParameters> cv_bioinspired_SegmentationParameters_SegmentationParameters() {
		try {
			cv::bioinspired::SegmentationParameters ret;
			return Ok<cv::bioinspired::SegmentationParameters>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::bioinspired::SegmentationParameters>))
	}
	
	Result<cv::Size> cv_bioinspired_TransientAreasSegmentationModule_getSize(cv::bioinspired::TransientAreasSegmentationModule* instance) {
		try {
			cv::Size ret = instance->getSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_setup_String_const_bool(cv::bioinspired::TransientAreasSegmentationModule* instance, char* segmentationParameterFile, const bool applyDefaultSetupOnFailure) {
		try {
			instance->setup(cv::String(segmentationParameterFile), applyDefaultSetupOnFailure);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_setup_FileStorageR_const_bool(cv::bioinspired::TransientAreasSegmentationModule* instance, cv::FileStorage* fs, const bool applyDefaultSetupOnFailure) {
		try {
			instance->setup(*fs, applyDefaultSetupOnFailure);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_setup_SegmentationParameters(cv::bioinspired::TransientAreasSegmentationModule* instance, cv::bioinspired::SegmentationParameters* newParameters) {
		try {
			instance->setup(*newParameters);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::bioinspired::SegmentationParameters> cv_bioinspired_TransientAreasSegmentationModule_getParameters(cv::bioinspired::TransientAreasSegmentationModule* instance) {
		try {
			cv::bioinspired::SegmentationParameters ret = instance->getParameters();
			return Ok<cv::bioinspired::SegmentationParameters>(ret);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::bioinspired::SegmentationParameters>))
	}
	
	Result<void*> cv_bioinspired_TransientAreasSegmentationModule_printSetup(cv::bioinspired::TransientAreasSegmentationModule* instance) {
		try {
			const cv::String ret = instance->printSetup();
			return Ok(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_write_const_String(const cv::bioinspired::TransientAreasSegmentationModule* instance, char* fs) {
		try {
			instance->write(cv::String(fs));
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_write_const_FileStorageR(const cv::bioinspired::TransientAreasSegmentationModule* instance, cv::FileStorage* fs) {
		try {
			instance->write(*fs);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_run_const__InputArrayR_const_int(cv::bioinspired::TransientAreasSegmentationModule* instance, const cv::_InputArray* inputToSegment, const int channelIndex) {
		try {
			instance->run(*inputToSegment, channelIndex);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_getSegmentationPicture_const__OutputArrayR(cv::bioinspired::TransientAreasSegmentationModule* instance, const cv::_OutputArray* transientAreas) {
		try {
			instance->getSegmentationPicture(*transientAreas);
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_clearAllBuffers(cv::bioinspired::TransientAreasSegmentationModule* instance) {
		try {
			instance->clearAllBuffers();
			return Ok();
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	Result<cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>*> cv_bioinspired_TransientAreasSegmentationModule_create_Size(cv::Size* inputSize) {
		try {
			cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule> ret = cv::bioinspired::TransientAreasSegmentationModule::create(*inputSize);
			return Ok(new cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>(ret));
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>*>))
	}
	
}
