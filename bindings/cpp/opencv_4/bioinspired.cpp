#include "bioinspired.hpp"
#include "bioinspired_types.hpp"

extern "C" {
	Result<cv::Size> cv_bioinspired_Retina_getInputSize(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::bioinspired::Retina*>(instance)->getInputSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result<cv::Size> cv_bioinspired_Retina_getOutputSize(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::bioinspired::Retina*>(instance)->getOutputSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_bioinspired_Retina_setup_String_bool(void* instance, char* retinaParameterFile, bool applyDefaultSetupOnFailure) {
		try {
			reinterpret_cast<cv::bioinspired::Retina*>(instance)->setup(std::string(retinaParameterFile), applyDefaultSetupOnFailure);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_setup_FileStorageX_bool(void* instance, void* fs, bool applyDefaultSetupOnFailure) {
		try {
			reinterpret_cast<cv::bioinspired::Retina*>(instance)->setup(*reinterpret_cast<cv::FileStorage*>(fs), applyDefaultSetupOnFailure);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_setup_RetinaParameters(void* instance, void* newParameters) {
		try {
			reinterpret_cast<cv::bioinspired::Retina*>(instance)->setup(*reinterpret_cast<cv::bioinspired::RetinaParameters*>(newParameters));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_bioinspired_Retina_getParameters(void* instance) {
		try {
			cv::bioinspired::RetinaParameters ret = reinterpret_cast<cv::bioinspired::Retina*>(instance)->getParameters();
			return Ok<void*>(new cv::bioinspired::RetinaParameters(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_bioinspired_Retina_printSetup(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::bioinspired::Retina*>(instance)->printSetup();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_bioinspired_Retina_write_const_String(void* instance, char* fs) {
		try {
			reinterpret_cast<cv::bioinspired::Retina*>(instance)->write(std::string(fs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_write_const_FileStorageX(void* instance, void* fs) {
		try {
			reinterpret_cast<cv::bioinspired::Retina*>(instance)->write(*reinterpret_cast<cv::FileStorage*>(fs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_setupOPLandIPLParvoChannel_bool_bool_float_float_float_float_float_float_float(void* instance, bool colorMode, bool normaliseOutput, float photoreceptorsLocalAdaptationSensitivity, float photoreceptorsTemporalConstant, float photoreceptorsSpatialConstant, float horizontalCellsGain, float HcellsTemporalConstant, float HcellsSpatialConstant, float ganglionCellsSensitivity) {
		try {
			reinterpret_cast<cv::bioinspired::Retina*>(instance)->setupOPLandIPLParvoChannel(colorMode, normaliseOutput, photoreceptorsLocalAdaptationSensitivity, photoreceptorsTemporalConstant, photoreceptorsSpatialConstant, horizontalCellsGain, HcellsTemporalConstant, HcellsSpatialConstant, ganglionCellsSensitivity);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_setupIPLMagnoChannel_bool_float_float_float_float_float_float_float(void* instance, bool normaliseOutput, float parasolCells_beta, float parasolCells_tau, float parasolCells_k, float amacrinCellsTemporalCutFrequency, float V0CompressionParameter, float localAdaptintegration_tau, float localAdaptintegration_k) {
		try {
			reinterpret_cast<cv::bioinspired::Retina*>(instance)->setupIPLMagnoChannel(normaliseOutput, parasolCells_beta, parasolCells_tau, parasolCells_k, amacrinCellsTemporalCutFrequency, V0CompressionParameter, localAdaptintegration_tau, localAdaptintegration_k);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_run_const__InputArrayX(void* instance, void* inputImage) {
		try {
			reinterpret_cast<cv::bioinspired::Retina*>(instance)->run(*reinterpret_cast<const cv::_InputArray*>(inputImage));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_applyFastToneMapping_const__InputArrayX_const__OutputArrayX(void* instance, void* inputImage, void* outputToneMappedImage) {
		try {
			reinterpret_cast<cv::bioinspired::Retina*>(instance)->applyFastToneMapping(*reinterpret_cast<const cv::_InputArray*>(inputImage), *reinterpret_cast<const cv::_OutputArray*>(outputToneMappedImage));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_getParvo_const__OutputArrayX(void* instance, void* retinaOutput_parvo) {
		try {
			reinterpret_cast<cv::bioinspired::Retina*>(instance)->getParvo(*reinterpret_cast<const cv::_OutputArray*>(retinaOutput_parvo));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_getParvoRAW_const__OutputArrayX(void* instance, void* retinaOutput_parvo) {
		try {
			reinterpret_cast<cv::bioinspired::Retina*>(instance)->getParvoRAW(*reinterpret_cast<const cv::_OutputArray*>(retinaOutput_parvo));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_getMagno_const__OutputArrayX(void* instance, void* retinaOutput_magno) {
		try {
			reinterpret_cast<cv::bioinspired::Retina*>(instance)->getMagno(*reinterpret_cast<const cv::_OutputArray*>(retinaOutput_magno));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_getMagnoRAW_const__OutputArrayX(void* instance, void* retinaOutput_magno) {
		try {
			reinterpret_cast<cv::bioinspired::Retina*>(instance)->getMagnoRAW(*reinterpret_cast<const cv::_OutputArray*>(retinaOutput_magno));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_bioinspired_Retina_getMagnoRAW_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::bioinspired::Retina*>(instance)->getMagnoRAW();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_bioinspired_Retina_getParvoRAW_const(void* instance) {
		try {
			cv::Mat ret = reinterpret_cast<cv::bioinspired::Retina*>(instance)->getParvoRAW();
			return Ok<void*>(new cv::Mat(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_bioinspired_Retina_setColorSaturation_bool_float(void* instance, bool saturateColors, float colorSaturationValue) {
		try {
			reinterpret_cast<cv::bioinspired::Retina*>(instance)->setColorSaturation(saturateColors, colorSaturationValue);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_clearBuffers(void* instance) {
		try {
			reinterpret_cast<cv::bioinspired::Retina*>(instance)->clearBuffers();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_activateMovingContoursProcessing_bool(void* instance, bool activate) {
		try {
			reinterpret_cast<cv::bioinspired::Retina*>(instance)->activateMovingContoursProcessing(activate);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_Retina_activateContoursProcessing_bool(void* instance, bool activate) {
		try {
			reinterpret_cast<cv::bioinspired::Retina*>(instance)->activateContoursProcessing(activate);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_bioinspired_Retina_create_Size(const cv::Size* inputSize) {
		try {
			cv::Ptr<cv::bioinspired::Retina> ret = cv::bioinspired::Retina::create(*inputSize);
			return Ok<void*>(new cv::Ptr<cv::bioinspired::Retina>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_bioinspired_Retina_create_Size_bool_int_bool_float_float(const cv::Size* inputSize, bool colorMode, int colorSamplingMethod, bool useRetinaLogSampling, float reductionFactor, float samplingStrenght) {
		try {
			cv::Ptr<cv::bioinspired::Retina> ret = cv::bioinspired::Retina::create(*inputSize, colorMode, colorSamplingMethod, useRetinaLogSampling, reductionFactor, samplingStrenght);
			return Ok<void*>(new cv::Ptr<cv::bioinspired::Retina>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_bioinspired_RetinaFastToneMapping_applyFastToneMapping_const__InputArrayX_const__OutputArrayX(void* instance, void* inputImage, void* outputToneMappedImage) {
		try {
			reinterpret_cast<cv::bioinspired::RetinaFastToneMapping*>(instance)->applyFastToneMapping(*reinterpret_cast<const cv::_InputArray*>(inputImage), *reinterpret_cast<const cv::_OutputArray*>(outputToneMappedImage));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_RetinaFastToneMapping_setup_float_float_float(void* instance, float photoreceptorsNeighborhoodRadius, float ganglioncellsNeighborhoodRadius, float meanLuminanceModulatorK) {
		try {
			reinterpret_cast<cv::bioinspired::RetinaFastToneMapping*>(instance)->setup(photoreceptorsNeighborhoodRadius, ganglioncellsNeighborhoodRadius, meanLuminanceModulatorK);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_bioinspired_RetinaFastToneMapping_create_Size(const cv::Size* inputSize) {
		try {
			cv::Ptr<cv::bioinspired::RetinaFastToneMapping> ret = cv::bioinspired::RetinaFastToneMapping::create(*inputSize);
			return Ok<void*>(new cv::Ptr<cv::bioinspired::RetinaFastToneMapping>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<cv::bioinspired::RetinaParameters::OPLandIplParvoParameters> cv_bioinspired_RetinaParameters_OPLandIplParvo_const(void* instance) {
		try {
			cv::bioinspired::RetinaParameters::OPLandIplParvoParameters ret = reinterpret_cast<cv::bioinspired::RetinaParameters*>(instance)->OPLandIplParvo;
			return Ok<cv::bioinspired::RetinaParameters::OPLandIplParvoParameters>(ret);
		} OCVRS_CATCH(Result<cv::bioinspired::RetinaParameters::OPLandIplParvoParameters>)
	}
	
	Result_void cv_bioinspired_RetinaParameters_setOPLandIplParvo_OPLandIplParvoParameters(void* instance, const cv::bioinspired::RetinaParameters::OPLandIplParvoParameters* val) {
		try {
			reinterpret_cast<cv::bioinspired::RetinaParameters*>(instance)->OPLandIplParvo = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::bioinspired::RetinaParameters::IplMagnoParameters> cv_bioinspired_RetinaParameters_IplMagno_const(void* instance) {
		try {
			cv::bioinspired::RetinaParameters::IplMagnoParameters ret = reinterpret_cast<cv::bioinspired::RetinaParameters*>(instance)->IplMagno;
			return Ok<cv::bioinspired::RetinaParameters::IplMagnoParameters>(ret);
		} OCVRS_CATCH(Result<cv::bioinspired::RetinaParameters::IplMagnoParameters>)
	}
	
	Result_void cv_bioinspired_RetinaParameters_setIplMagno_IplMagnoParameters(void* instance, const cv::bioinspired::RetinaParameters::IplMagnoParameters* val) {
		try {
			reinterpret_cast<cv::bioinspired::RetinaParameters*>(instance)->IplMagno = *val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_RetinaParameters_delete(cv::bioinspired::RetinaParameters* instance) {
		delete instance;
	}
	Result<cv::bioinspired::RetinaParameters::IplMagnoParameters> cv_bioinspired_RetinaParameters_IplMagnoParameters_IplMagnoParameters() {
		try {
			cv::bioinspired::RetinaParameters::IplMagnoParameters ret;
			return Ok<cv::bioinspired::RetinaParameters::IplMagnoParameters>(ret);
		} OCVRS_CATCH(Result<cv::bioinspired::RetinaParameters::IplMagnoParameters>)
	}
	
	Result<cv::bioinspired::RetinaParameters::OPLandIplParvoParameters> cv_bioinspired_RetinaParameters_OPLandIplParvoParameters_OPLandIplParvoParameters() {
		try {
			cv::bioinspired::RetinaParameters::OPLandIplParvoParameters ret;
			return Ok<cv::bioinspired::RetinaParameters::OPLandIplParvoParameters>(ret);
		} OCVRS_CATCH(Result<cv::bioinspired::RetinaParameters::OPLandIplParvoParameters>)
	}
	
	Result<cv::bioinspired::SegmentationParameters> cv_bioinspired_SegmentationParameters_SegmentationParameters() {
		try {
			cv::bioinspired::SegmentationParameters ret;
			return Ok<cv::bioinspired::SegmentationParameters>(ret);
		} OCVRS_CATCH(Result<cv::bioinspired::SegmentationParameters>)
	}
	
	Result<cv::Size> cv_bioinspired_TransientAreasSegmentationModule_getSize(void* instance) {
		try {
			cv::Size ret = reinterpret_cast<cv::bioinspired::TransientAreasSegmentationModule*>(instance)->getSize();
			return Ok<cv::Size>(ret);
		} OCVRS_CATCH(Result<cv::Size>)
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_setup_String_bool(void* instance, char* segmentationParameterFile, bool applyDefaultSetupOnFailure) {
		try {
			reinterpret_cast<cv::bioinspired::TransientAreasSegmentationModule*>(instance)->setup(std::string(segmentationParameterFile), applyDefaultSetupOnFailure);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_setup_FileStorageX_bool(void* instance, void* fs, bool applyDefaultSetupOnFailure) {
		try {
			reinterpret_cast<cv::bioinspired::TransientAreasSegmentationModule*>(instance)->setup(*reinterpret_cast<cv::FileStorage*>(fs), applyDefaultSetupOnFailure);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_setup_SegmentationParameters(void* instance, const cv::bioinspired::SegmentationParameters* newParameters) {
		try {
			reinterpret_cast<cv::bioinspired::TransientAreasSegmentationModule*>(instance)->setup(*newParameters);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::bioinspired::SegmentationParameters> cv_bioinspired_TransientAreasSegmentationModule_getParameters(void* instance) {
		try {
			cv::bioinspired::SegmentationParameters ret = reinterpret_cast<cv::bioinspired::TransientAreasSegmentationModule*>(instance)->getParameters();
			return Ok<cv::bioinspired::SegmentationParameters>(ret);
		} OCVRS_CATCH(Result<cv::bioinspired::SegmentationParameters>)
	}
	
	Result<void*> cv_bioinspired_TransientAreasSegmentationModule_printSetup(void* instance) {
		try {
			cv::String ret = reinterpret_cast<cv::bioinspired::TransientAreasSegmentationModule*>(instance)->printSetup();
			return Ok<void*>(ocvrs_create_string(ret.c_str()));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_write_const_String(void* instance, char* fs) {
		try {
			reinterpret_cast<cv::bioinspired::TransientAreasSegmentationModule*>(instance)->write(std::string(fs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_write_const_FileStorageX(void* instance, void* fs) {
		try {
			reinterpret_cast<cv::bioinspired::TransientAreasSegmentationModule*>(instance)->write(*reinterpret_cast<cv::FileStorage*>(fs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_run_const__InputArrayX_int(void* instance, void* inputToSegment, int channelIndex) {
		try {
			reinterpret_cast<cv::bioinspired::TransientAreasSegmentationModule*>(instance)->run(*reinterpret_cast<const cv::_InputArray*>(inputToSegment), channelIndex);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_getSegmentationPicture_const__OutputArrayX(void* instance, void* transientAreas) {
		try {
			reinterpret_cast<cv::bioinspired::TransientAreasSegmentationModule*>(instance)->getSegmentationPicture(*reinterpret_cast<const cv::_OutputArray*>(transientAreas));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_bioinspired_TransientAreasSegmentationModule_clearAllBuffers(void* instance) {
		try {
			reinterpret_cast<cv::bioinspired::TransientAreasSegmentationModule*>(instance)->clearAllBuffers();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_bioinspired_TransientAreasSegmentationModule_create_Size(const cv::Size* inputSize) {
		try {
			cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule> ret = cv::bioinspired::TransientAreasSegmentationModule::create(*inputSize);
			return Ok<void*>(new cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
}
