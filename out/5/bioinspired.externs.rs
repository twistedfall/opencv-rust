// getInputSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:170
// ("cv::bioinspired::Retina::getInputSize", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_Retina_getInputSize(instance: *mut c_void, ocvrs_return: *mut Result<core::Size>);
// getOutputSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:176
// ("cv::bioinspired::Retina::getOutputSize", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_Retina_getOutputSize(instance: *mut c_void, ocvrs_return: *mut Result<core::Size>);
// setup(String, const bool)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:188
// ("cv::bioinspired::Retina::setup", vec![(pred!(mut, ["retinaParameterFile", "applyDefaultSetupOnFailure"], ["cv::String", "const bool"]), _)]),
pub fn cv_bioinspired_Retina_setup_String_const_bool(instance: *mut c_void, retina_parameter_file: *const c_char, apply_default_setup_on_failure: bool, ocvrs_return: *mut Result<()>);
// cv::bioinspired::Retina::setup() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:188
// ("cv::bioinspired::Retina::setup", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_Retina_setup(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// setup(cv::FileStorage &, const bool)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:194
// ("cv::bioinspired::Retina::setup", vec![(pred!(mut, ["fs", "applyDefaultSetupOnFailure"], ["cv::FileStorage*", "const bool"]), _)]),
pub fn cv_bioinspired_Retina_setup_FileStorageR_const_bool(instance: *mut c_void, fs: *mut c_void, apply_default_setup_on_failure: bool, ocvrs_return: *mut Result<()>);
// cv::bioinspired::Retina::setup(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:194
// ("cv::bioinspired::Retina::setup", vec![(pred!(mut, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_bioinspired_Retina_setup_FileStorageR(instance: *mut c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// setup(RetinaParameters)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:199
// ("cv::bioinspired::Retina::setup", vec![(pred!(mut, ["newParameters"], ["cv::bioinspired::RetinaParameters"]), _)]),
pub fn cv_bioinspired_Retina_setup_RetinaParameters(instance: *mut c_void, new_parameters: *mut c_void, ocvrs_return: *mut Result<()>);
// getParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:204
// ("cv::bioinspired::Retina::getParameters", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_Retina_getParameters(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// printSetup()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:209
// ("cv::bioinspired::Retina::printSetup", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_Retina_printSetup(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// write(String)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:215
// ("cv::bioinspired::Retina::write", vec![(pred!(const, ["fs"], ["cv::String"]), _)]),
pub fn cv_bioinspired_Retina_write_const_String(instance: *const c_void, fs: *const c_char, ocvrs_return: *mut Result<()>);
// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:218
// ("cv::bioinspired::Retina::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_bioinspired_Retina_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// setupOPLandIPLParvoChannel(const bool, const bool, const float, const float, const float, const float, const float, const float, const float)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:254
// ("cv::bioinspired::Retina::setupOPLandIPLParvoChannel", vec![(pred!(mut, ["colorMode", "normaliseOutput", "photoreceptorsLocalAdaptationSensitivity", "photoreceptorsTemporalConstant", "photoreceptorsSpatialConstant", "horizontalCellsGain", "HcellsTemporalConstant", "HcellsSpatialConstant", "ganglionCellsSensitivity"], ["const bool", "const bool", "const float", "const float", "const float", "const float", "const float", "const float", "const float"]), _)]),
pub fn cv_bioinspired_Retina_setupOPLandIPLParvoChannel_const_bool_const_bool_const_float_const_float_const_float_const_float_const_float_const_float_const_float(instance: *mut c_void, color_mode: bool, normalise_output: bool, photoreceptors_local_adaptation_sensitivity: f32, photoreceptors_temporal_constant: f32, photoreceptors_spatial_constant: f32, horizontal_cells_gain: f32, hcells_temporal_constant: f32, hcells_spatial_constant: f32, ganglion_cells_sensitivity: f32, ocvrs_return: *mut Result<()>);
// cv::bioinspired::Retina::setupOPLandIPLParvoChannel() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:254
// ("cv::bioinspired::Retina::setupOPLandIPLParvoChannel", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_Retina_setupOPLandIPLParvoChannel(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// setupIPLMagnoChannel(const bool, const float, const float, const float, const float, const float, const float, const float)(Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:281
// ("cv::bioinspired::Retina::setupIPLMagnoChannel", vec![(pred!(mut, ["normaliseOutput", "parasolCells_beta", "parasolCells_tau", "parasolCells_k", "amacrinCellsTemporalCutFrequency", "V0CompressionParameter", "localAdaptintegration_tau", "localAdaptintegration_k"], ["const bool", "const float", "const float", "const float", "const float", "const float", "const float", "const float"]), _)]),
pub fn cv_bioinspired_Retina_setupIPLMagnoChannel_const_bool_const_float_const_float_const_float_const_float_const_float_const_float_const_float(instance: *mut c_void, normalise_output: bool, parasol_cells_beta: f32, parasol_cells_tau: f32, parasol_cells_k: f32, amacrin_cells_temporal_cut_frequency: f32, v0_compression_parameter: f32, local_adaptintegration_tau: f32, local_adaptintegration_k: f32, ocvrs_return: *mut Result<()>);
// cv::bioinspired::Retina::setupIPLMagnoChannel() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:281
// ("cv::bioinspired::Retina::setupIPLMagnoChannel", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_Retina_setupIPLMagnoChannel(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// run(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:290
// ("cv::bioinspired::Retina::run", vec![(pred!(mut, ["inputImage"], ["const cv::_InputArray*"]), _)]),
pub fn cv_bioinspired_Retina_run_const__InputArrayR(instance: *mut c_void, input_image: *const c_void, ocvrs_return: *mut Result<()>);
// applyFastToneMapping(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:308
// ("cv::bioinspired::Retina::applyFastToneMapping", vec![(pred!(mut, ["inputImage", "outputToneMappedImage"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_bioinspired_Retina_applyFastToneMapping_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, input_image: *const c_void, output_tone_mapped_image: *const c_void, ocvrs_return: *mut Result<()>);
// getParvo(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:322
// ("cv::bioinspired::Retina::getParvo", vec![(pred!(mut, ["retinaOutput_parvo"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_bioinspired_Retina_getParvo_const__OutputArrayR(instance: *mut c_void, retina_output_parvo: *const c_void, ocvrs_return: *mut Result<()>);
// getParvoRAW(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:327
// ("cv::bioinspired::Retina::getParvoRAW", vec![(pred!(mut, ["retinaOutput_parvo"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_bioinspired_Retina_getParvoRAW_const__OutputArrayR(instance: *mut c_void, retina_output_parvo: *const c_void, ocvrs_return: *mut Result<()>);
// getMagno(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:339
// ("cv::bioinspired::Retina::getMagno", vec![(pred!(mut, ["retinaOutput_magno"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_bioinspired_Retina_getMagno_const__OutputArrayR(instance: *mut c_void, retina_output_magno: *const c_void, ocvrs_return: *mut Result<()>);
// getMagnoRAW(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:344
// ("cv::bioinspired::Retina::getMagnoRAW", vec![(pred!(mut, ["retinaOutput_magno"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_bioinspired_Retina_getMagnoRAW_const__OutputArrayR(instance: *mut c_void, retina_output_magno: *const c_void, ocvrs_return: *mut Result<()>);
// getMagnoRAW()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:347
// ("cv::bioinspired::Retina::getMagnoRAW", vec![(pred!(const, [], []), _)]),
pub fn cv_bioinspired_Retina_getMagnoRAW_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getParvoRAW()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:349
// ("cv::bioinspired::Retina::getParvoRAW", vec![(pred!(const, [], []), _)]),
pub fn cv_bioinspired_Retina_getParvoRAW_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setColorSaturation(const bool, const float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:357
// ("cv::bioinspired::Retina::setColorSaturation", vec![(pred!(mut, ["saturateColors", "colorSaturationValue"], ["const bool", "const float"]), _)]),
pub fn cv_bioinspired_Retina_setColorSaturation_const_bool_const_float(instance: *mut c_void, saturate_colors: bool, color_saturation_value: f32, ocvrs_return: *mut Result<()>);
// cv::bioinspired::Retina::setColorSaturation() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:357
// ("cv::bioinspired::Retina::setColorSaturation", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_Retina_setColorSaturation(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// clearBuffers()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:364
// ("cv::bioinspired::Retina::clearBuffers", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_Retina_clearBuffers(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// activateMovingContoursProcessing(const bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:371
// ("cv::bioinspired::Retina::activateMovingContoursProcessing", vec![(pred!(mut, ["activate"], ["const bool"]), _)]),
pub fn cv_bioinspired_Retina_activateMovingContoursProcessing_const_bool(instance: *mut c_void, activate: bool, ocvrs_return: *mut Result<()>);
// activateContoursProcessing(const bool)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:379
// ("cv::bioinspired::Retina::activateContoursProcessing", vec![(pred!(mut, ["activate"], ["const bool"]), _)]),
pub fn cv_bioinspired_Retina_activateContoursProcessing_const_bool(instance: *mut c_void, activate: bool, ocvrs_return: *mut Result<()>);
// create(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:382
// ("cv::bioinspired::Retina::create", vec![(pred!(mut, ["inputSize"], ["cv::Size"]), _)]),
pub fn cv_bioinspired_Retina_create_Size(input_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// create(Size, const bool, int, const bool, const float, const float)(SimpleClass, Primitive, Primitive, Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:399
// ("cv::bioinspired::Retina::create", vec![(pred!(mut, ["inputSize", "colorMode", "colorSamplingMethod", "useRetinaLogSampling", "reductionFactor", "samplingStrength"], ["cv::Size", "const bool", "int", "const bool", "const float", "const float"]), _)]),
pub fn cv_bioinspired_Retina_create_Size_const_bool_int_const_bool_const_float_const_float(input_size: *const core::Size, color_mode: bool, color_sampling_method: i32, use_retina_log_sampling: bool, reduction_factor: f32, sampling_strength: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::bioinspired::Retina::create(SimpleClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:399
// ("cv::bioinspired::Retina::create", vec![(pred!(mut, ["inputSize", "colorMode"], ["cv::Size", "const bool"]), _)]),
pub fn cv_bioinspired_Retina_create_Size_const_bool(input_size: *const core::Size, color_mode: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::bioinspired::Retina::to_Algorithm() generated
// ("cv::bioinspired::Retina::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_Retina_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::bioinspired::Retina::delete() generated
// ("cv::bioinspired::Retina::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_Retina_delete(instance: *mut c_void);
// applyFastToneMapping(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retinafasttonemapping.hpp:119
// ("cv::bioinspired::RetinaFastToneMapping::applyFastToneMapping", vec![(pred!(mut, ["inputImage", "outputToneMappedImage"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_bioinspired_RetinaFastToneMapping_applyFastToneMapping_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, input_image: *const c_void, output_tone_mapped_image: *const c_void, ocvrs_return: *mut Result<()>);
// setup(const float, const float, const float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retinafasttonemapping.hpp:128
// ("cv::bioinspired::RetinaFastToneMapping::setup", vec![(pred!(mut, ["photoreceptorsNeighborhoodRadius", "ganglioncellsNeighborhoodRadius", "meanLuminanceModulatorK"], ["const float", "const float", "const float"]), _)]),
pub fn cv_bioinspired_RetinaFastToneMapping_setup_const_float_const_float_const_float(instance: *mut c_void, photoreceptors_neighborhood_radius: f32, ganglioncells_neighborhood_radius: f32, mean_luminance_modulator_k: f32, ocvrs_return: *mut Result<()>);
// cv::bioinspired::RetinaFastToneMapping::setup() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retinafasttonemapping.hpp:128
// ("cv::bioinspired::RetinaFastToneMapping::setup", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_RetinaFastToneMapping_setup(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// create(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retinafasttonemapping.hpp:130
// ("cv::bioinspired::RetinaFastToneMapping::create", vec![(pred!(mut, ["inputSize"], ["cv::Size"]), _)]),
pub fn cv_bioinspired_RetinaFastToneMapping_create_Size(input_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// cv::bioinspired::RetinaFastToneMapping::to_Algorithm() generated
// ("cv::bioinspired::RetinaFastToneMapping::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_RetinaFastToneMapping_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::bioinspired::RetinaFastToneMapping::delete() generated
// ("cv::bioinspired::RetinaFastToneMapping::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_RetinaFastToneMapping_delete(instance: *mut c_void);
// cv::bioinspired::RetinaParameters::implicitClone() generated
// ("cv::bioinspired::RetinaParameters::implicitClone", vec![(pred!(const, [], []), _)]),
pub fn cv_bioinspired_RetinaParameters_implicitClone_const(instance: *const c_void) -> *mut c_void;
// cv::bioinspired::RetinaParameters::defaultNew() generated
// ("cv::bioinspired::RetinaParameters::defaultNew", vec![(pred!(const, [], []), _)]),
pub fn cv_bioinspired_RetinaParameters_defaultNew_const() -> *mut c_void;
// cv::bioinspired::RetinaParameters::OPLandIplParvo() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:133
// ("cv::bioinspired::RetinaParameters::OPLandIplParvo", vec![(pred!(const, [], []), _)]),
pub fn cv_bioinspired_RetinaParameters_propOPLandIplParvo_const(instance: *const c_void, ocvrs_return: *mut crate::bioinspired::RetinaParameters_OPLandIplParvoParameters);
// cv::bioinspired::RetinaParameters::setOPLandIplParvo(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:133
// ("cv::bioinspired::RetinaParameters::setOPLandIplParvo", vec![(pred!(mut, ["val"], ["const cv::bioinspired::RetinaParameters::OPLandIplParvoParameters"]), _)]),
pub fn cv_bioinspired_RetinaParameters_propOPLandIplParvo_const_OPLandIplParvoParameters(instance: *mut c_void, val: *const crate::bioinspired::RetinaParameters_OPLandIplParvoParameters);
// cv::bioinspired::RetinaParameters::IplMagno() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:134
// ("cv::bioinspired::RetinaParameters::IplMagno", vec![(pred!(const, [], []), _)]),
pub fn cv_bioinspired_RetinaParameters_propIplMagno_const(instance: *const c_void, ocvrs_return: *mut crate::bioinspired::RetinaParameters_IplMagnoParameters);
// cv::bioinspired::RetinaParameters::setIplMagno(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:134
// ("cv::bioinspired::RetinaParameters::setIplMagno", vec![(pred!(mut, ["val"], ["const cv::bioinspired::RetinaParameters::IplMagnoParameters"]), _)]),
pub fn cv_bioinspired_RetinaParameters_propIplMagno_const_IplMagnoParameters(instance: *mut c_void, val: *const crate::bioinspired::RetinaParameters_IplMagnoParameters);
// cv::bioinspired::RetinaParameters::delete() generated
// ("cv::bioinspired::RetinaParameters::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_RetinaParameters_delete(instance: *mut c_void);
// IplMagnoParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:121
// ("cv::bioinspired::RetinaParameters::IplMagnoParameters::IplMagnoParameters", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_RetinaParameters_IplMagnoParameters_IplMagnoParameters(ocvrs_return: *mut Result<crate::bioinspired::RetinaParameters_IplMagnoParameters>);
// OPLandIplParvoParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/retina.hpp:107
// ("cv::bioinspired::RetinaParameters::OPLandIplParvoParameters::OPLandIplParvoParameters", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_RetinaParameters_OPLandIplParvoParameters_OPLandIplParvoParameters(ocvrs_return: *mut Result<crate::bioinspired::RetinaParameters_OPLandIplParvoParameters>);
// SegmentationParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/transientareassegmentationmodule.hpp:84
// ("cv::bioinspired::SegmentationParameters::SegmentationParameters", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_SegmentationParameters_SegmentationParameters(ocvrs_return: *mut Result<crate::bioinspired::SegmentationParameters>);
// getSize()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/transientareassegmentationmodule.hpp:129
// ("cv::bioinspired::TransientAreasSegmentationModule::getSize", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_TransientAreasSegmentationModule_getSize(instance: *mut c_void, ocvrs_return: *mut Result<core::Size>);
// setup(String, const bool)(InString, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/transientareassegmentationmodule.hpp:138
// ("cv::bioinspired::TransientAreasSegmentationModule::setup", vec![(pred!(mut, ["segmentationParameterFile", "applyDefaultSetupOnFailure"], ["cv::String", "const bool"]), _)]),
pub fn cv_bioinspired_TransientAreasSegmentationModule_setup_String_const_bool(instance: *mut c_void, segmentation_parameter_file: *const c_char, apply_default_setup_on_failure: bool, ocvrs_return: *mut Result<()>);
// cv::bioinspired::TransientAreasSegmentationModule::setup() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/transientareassegmentationmodule.hpp:138
// ("cv::bioinspired::TransientAreasSegmentationModule::setup", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_TransientAreasSegmentationModule_setup(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// setup(cv::FileStorage &, const bool)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/transientareassegmentationmodule.hpp:147
// ("cv::bioinspired::TransientAreasSegmentationModule::setup", vec![(pred!(mut, ["fs", "applyDefaultSetupOnFailure"], ["cv::FileStorage*", "const bool"]), _)]),
pub fn cv_bioinspired_TransientAreasSegmentationModule_setup_FileStorageR_const_bool(instance: *mut c_void, fs: *mut c_void, apply_default_setup_on_failure: bool, ocvrs_return: *mut Result<()>);
// cv::bioinspired::TransientAreasSegmentationModule::setup(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/transientareassegmentationmodule.hpp:147
// ("cv::bioinspired::TransientAreasSegmentationModule::setup", vec![(pred!(mut, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_bioinspired_TransientAreasSegmentationModule_setup_FileStorageR(instance: *mut c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// setup(SegmentationParameters)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/transientareassegmentationmodule.hpp:155
// ("cv::bioinspired::TransientAreasSegmentationModule::setup", vec![(pred!(mut, ["newParameters"], ["cv::bioinspired::SegmentationParameters"]), _)]),
pub fn cv_bioinspired_TransientAreasSegmentationModule_setup_SegmentationParameters(instance: *mut c_void, new_parameters: *const crate::bioinspired::SegmentationParameters, ocvrs_return: *mut Result<()>);
// getParameters()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/transientareassegmentationmodule.hpp:159
// ("cv::bioinspired::TransientAreasSegmentationModule::getParameters", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_TransientAreasSegmentationModule_getParameters(instance: *mut c_void, ocvrs_return: *mut Result<crate::bioinspired::SegmentationParameters>);
// printSetup()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/transientareassegmentationmodule.hpp:164
// ("cv::bioinspired::TransientAreasSegmentationModule::printSetup", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_TransientAreasSegmentationModule_printSetup(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// write(String)(InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/transientareassegmentationmodule.hpp:169
// ("cv::bioinspired::TransientAreasSegmentationModule::write", vec![(pred!(const, ["fs"], ["cv::String"]), _)]),
pub fn cv_bioinspired_TransientAreasSegmentationModule_write_const_String(instance: *const c_void, fs: *const c_char, ocvrs_return: *mut Result<()>);
// write(cv::FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/transientareassegmentationmodule.hpp:174
// ("cv::bioinspired::TransientAreasSegmentationModule::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
pub fn cv_bioinspired_TransientAreasSegmentationModule_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result<()>);
// run(InputArray, const int)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/transientareassegmentationmodule.hpp:180
// ("cv::bioinspired::TransientAreasSegmentationModule::run", vec![(pred!(mut, ["inputToSegment", "channelIndex"], ["const cv::_InputArray*", "const int"]), _)]),
pub fn cv_bioinspired_TransientAreasSegmentationModule_run_const__InputArrayR_const_int(instance: *mut c_void, input_to_segment: *const c_void, channel_index: i32, ocvrs_return: *mut Result<()>);
// cv::bioinspired::TransientAreasSegmentationModule::run(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/transientareassegmentationmodule.hpp:180
// ("cv::bioinspired::TransientAreasSegmentationModule::run", vec![(pred!(mut, ["inputToSegment"], ["const cv::_InputArray*"]), _)]),
pub fn cv_bioinspired_TransientAreasSegmentationModule_run_const__InputArrayR(instance: *mut c_void, input_to_segment: *const c_void, ocvrs_return: *mut Result<()>);
// getSegmentationPicture(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/transientareassegmentationmodule.hpp:185
// ("cv::bioinspired::TransientAreasSegmentationModule::getSegmentationPicture", vec![(pred!(mut, ["transientAreas"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_bioinspired_TransientAreasSegmentationModule_getSegmentationPicture_const__OutputArrayR(instance: *mut c_void, transient_areas: *const c_void, ocvrs_return: *mut Result<()>);
// clearAllBuffers()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/transientareassegmentationmodule.hpp:189
// ("cv::bioinspired::TransientAreasSegmentationModule::clearAllBuffers", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_TransientAreasSegmentationModule_clearAllBuffers(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// create(Size)(SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/bioinspired/transientareassegmentationmodule.hpp:194
// ("cv::bioinspired::TransientAreasSegmentationModule::create", vec![(pred!(mut, ["inputSize"], ["cv::Size"]), _)]),
pub fn cv_bioinspired_TransientAreasSegmentationModule_create_Size(input_size: *const core::Size, ocvrs_return: *mut Result<*mut c_void>);
// cv::bioinspired::TransientAreasSegmentationModule::to_Algorithm() generated
// ("cv::bioinspired::TransientAreasSegmentationModule::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_TransientAreasSegmentationModule_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::bioinspired::TransientAreasSegmentationModule::delete() generated
// ("cv::bioinspired::TransientAreasSegmentationModule::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_bioinspired_TransientAreasSegmentationModule_delete(instance: *mut c_void);
