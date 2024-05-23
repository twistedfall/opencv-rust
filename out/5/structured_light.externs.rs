// create(const GrayCodePattern::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:86
// ("cv::structured_light::GrayCodePattern::create", vec![(pred!(mut, ["parameters"], ["const cv::structured_light::GrayCodePattern::Params*"]), _)]),
pub fn cv_structured_light_GrayCodePattern_create_const_ParamsR(parameters: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::structured_light::GrayCodePattern::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:86
// ("cv::structured_light::GrayCodePattern::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_structured_light_GrayCodePattern_create(ocvrs_return: *mut Result<*mut c_void>);
// create(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:90
// ("cv::structured_light::GrayCodePattern::create", vec![(pred!(mut, ["width", "height"], ["int", "int"]), _)]),
pub fn cv_structured_light_GrayCodePattern_create_int_int(width: i32, height: i32, ocvrs_return: *mut Result<*mut c_void>);
// getNumberOfPatternImages()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:98
// ("cv::structured_light::GrayCodePattern::getNumberOfPatternImages", vec![(pred!(const, [], []), _)]),
pub fn cv_structured_light_GrayCodePattern_getNumberOfPatternImages_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
// setWhiteThreshold(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:108
// ("cv::structured_light::GrayCodePattern::setWhiteThreshold", vec![(pred!(mut, ["value"], ["size_t"]), _)]),
pub fn cv_structured_light_GrayCodePattern_setWhiteThreshold_size_t(instance: *mut c_void, value: size_t, ocvrs_return: *mut Result<()>);
// setBlackThreshold(size_t)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:118
// ("cv::structured_light::GrayCodePattern::setBlackThreshold", vec![(pred!(mut, ["value"], ["size_t"]), _)]),
pub fn cv_structured_light_GrayCodePattern_setBlackThreshold_size_t(instance: *mut c_void, value: size_t, ocvrs_return: *mut Result<()>);
// getImagesForShadowMasks(InputOutputArray, InputOutputArray)(InputOutputArray, InputOutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:130
// ("cv::structured_light::GrayCodePattern::getImagesForShadowMasks", vec![(pred!(const, ["blackImage", "whiteImage"], ["const cv::_InputOutputArray*", "const cv::_InputOutputArray*"]), _)]),
pub fn cv_structured_light_GrayCodePattern_getImagesForShadowMasks_const_const__InputOutputArrayR_const__InputOutputArrayR(instance: *const c_void, black_image: *const c_void, white_image: *const c_void, ocvrs_return: *mut Result<()>);
// getProjPixel(InputArrayOfArrays, int, int, Point &)(InputArray, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:143
// ("cv::structured_light::GrayCodePattern::getProjPixel", vec![(pred!(const, ["patternImages", "x", "y", "projPix"], ["const cv::_InputArray*", "int", "int", "cv::Point*"]), _)]),
pub fn cv_structured_light_GrayCodePattern_getProjPixel_const_const__InputArrayR_int_int_PointR(instance: *const c_void, pattern_images: *const c_void, x: i32, y: i32, proj_pix: *mut core::Point, ocvrs_return: *mut Result<bool>);
// cv::structured_light::GrayCodePattern::to_Algorithm() generated
// ("cv::structured_light::GrayCodePattern::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_structured_light_GrayCodePattern_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::structured_light::GrayCodePattern::to_StructuredLightPattern() generated
// ("cv::structured_light::GrayCodePattern::to_StructuredLightPattern", vec![(pred!(mut, [], []), _)]),
pub fn cv_structured_light_GrayCodePattern_to_StructuredLightPattern(instance: *mut c_void) -> *mut c_void;
// cv::structured_light::GrayCodePattern::delete() generated
// ("cv::structured_light::GrayCodePattern::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_structured_light_GrayCodePattern_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:78
// ("cv::structured_light::GrayCodePattern::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_structured_light_GrayCodePattern_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// cv::structured_light::GrayCodePattern::Params::width() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:79
// ("cv::structured_light::GrayCodePattern::Params::width", vec![(pred!(const, [], []), _)]),
pub fn cv_structured_light_GrayCodePattern_Params_propWidth_const(instance: *const c_void) -> i32;
// cv::structured_light::GrayCodePattern::Params::setWidth(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:79
// ("cv::structured_light::GrayCodePattern::Params::setWidth", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_structured_light_GrayCodePattern_Params_propWidth_const_int(instance: *mut c_void, val: i32);
// cv::structured_light::GrayCodePattern::Params::height() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:80
// ("cv::structured_light::GrayCodePattern::Params::height", vec![(pred!(const, [], []), _)]),
pub fn cv_structured_light_GrayCodePattern_Params_propHeight_const(instance: *const c_void) -> i32;
// cv::structured_light::GrayCodePattern::Params::setHeight(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/graycodepattern.hpp:80
// ("cv::structured_light::GrayCodePattern::Params::setHeight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_structured_light_GrayCodePattern_Params_propHeight_const_int(instance: *mut c_void, val: i32);
// cv::structured_light::GrayCodePattern::Params::delete() generated
// ("cv::structured_light::GrayCodePattern::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_structured_light_GrayCodePattern_Params_delete(instance: *mut c_void);
// create(Ptr<SinusoidalPattern::Params>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:100
// ("cv::structured_light::SinusoidalPattern::create", vec![(pred!(mut, ["parameters"], ["cv::Ptr<cv::structured_light::SinusoidalPattern::Params>"]), _)]),
pub fn cv_structured_light_SinusoidalPattern_create_PtrLParamsG(parameters: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::structured_light::SinusoidalPattern::create() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:100
// ("cv::structured_light::SinusoidalPattern::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_structured_light_SinusoidalPattern_create(ocvrs_return: *mut Result<*mut c_void>);
// computePhaseMap(InputArrayOfArrays, OutputArray, OutputArray, InputArray)(InputArray, OutputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:110
// ("cv::structured_light::SinusoidalPattern::computePhaseMap", vec![(pred!(mut, ["patternImages", "wrappedPhaseMap", "shadowMask", "fundamental"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_structured_light_SinusoidalPattern_computePhaseMap_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR(instance: *mut c_void, pattern_images: *const c_void, wrapped_phase_map: *const c_void, shadow_mask: *const c_void, fundamental: *const c_void, ocvrs_return: *mut Result<()>);
// cv::structured_light::SinusoidalPattern::computePhaseMap(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:110
// ("cv::structured_light::SinusoidalPattern::computePhaseMap", vec![(pred!(mut, ["patternImages", "wrappedPhaseMap"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_structured_light_SinusoidalPattern_computePhaseMap_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, pattern_images: *const c_void, wrapped_phase_map: *const c_void, ocvrs_return: *mut Result<()>);
// unwrapPhaseMap(InputArray, OutputArray, cv::Size, InputArray)(InputArray, OutputArray, SimpleClass, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:122
// ("cv::structured_light::SinusoidalPattern::unwrapPhaseMap", vec![(pred!(mut, ["wrappedPhaseMap", "unwrappedPhaseMap", "camSize", "shadowMask"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size", "const cv::_InputArray*"]), _)]),
pub fn cv_structured_light_SinusoidalPattern_unwrapPhaseMap_const__InputArrayR_const__OutputArrayR_Size_const__InputArrayR(instance: *mut c_void, wrapped_phase_map: *const c_void, unwrapped_phase_map: *const c_void, cam_size: *const core::Size, shadow_mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::structured_light::SinusoidalPattern::unwrapPhaseMap(InputArray, OutputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:122
// ("cv::structured_light::SinusoidalPattern::unwrapPhaseMap", vec![(pred!(mut, ["wrappedPhaseMap", "unwrappedPhaseMap", "camSize"], ["const cv::_InputArray*", "const cv::_OutputArray*", "cv::Size"]), _)]),
pub fn cv_structured_light_SinusoidalPattern_unwrapPhaseMap_const__InputArrayR_const__OutputArrayR_Size(instance: *mut c_void, wrapped_phase_map: *const c_void, unwrapped_phase_map: *const c_void, cam_size: *const core::Size, ocvrs_return: *mut Result<()>);
// findProCamMatches(InputArray, InputArray, OutputArrayOfArrays)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:133
// ("cv::structured_light::SinusoidalPattern::findProCamMatches", vec![(pred!(mut, ["projUnwrappedPhaseMap", "camUnwrappedPhaseMap", "matches"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_structured_light_SinusoidalPattern_findProCamMatches_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, proj_unwrapped_phase_map: *const c_void, cam_unwrapped_phase_map: *const c_void, matches: *const c_void, ocvrs_return: *mut Result<()>);
// computeDataModulationTerm(InputArrayOfArrays, OutputArray, InputArray)(InputArray, OutputArray, InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:143
// ("cv::structured_light::SinusoidalPattern::computeDataModulationTerm", vec![(pred!(mut, ["patternImages", "dataModulationTerm", "shadowMask"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_structured_light_SinusoidalPattern_computeDataModulationTerm_const__InputArrayR_const__OutputArrayR_const__InputArrayR(instance: *mut c_void, pattern_images: *const c_void, data_modulation_term: *const c_void, shadow_mask: *const c_void, ocvrs_return: *mut Result<()>);
// cv::structured_light::SinusoidalPattern::to_Algorithm() generated
// ("cv::structured_light::SinusoidalPattern::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_structured_light_SinusoidalPattern_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::structured_light::SinusoidalPattern::to_StructuredLightPattern() generated
// ("cv::structured_light::SinusoidalPattern::to_StructuredLightPattern", vec![(pred!(mut, [], []), _)]),
pub fn cv_structured_light_SinusoidalPattern_to_StructuredLightPattern(instance: *mut c_void) -> *mut c_void;
// cv::structured_light::SinusoidalPattern::delete() generated
// ("cv::structured_light::SinusoidalPattern::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_structured_light_SinusoidalPattern_delete(instance: *mut c_void);
// Params()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:84
// ("cv::structured_light::SinusoidalPattern::Params::Params", vec![(pred!(mut, [], []), _)]),
pub fn cv_structured_light_SinusoidalPattern_Params_Params(ocvrs_return: *mut Result<*mut c_void>);
// cv::structured_light::SinusoidalPattern::Params::width() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:85
// ("cv::structured_light::SinusoidalPattern::Params::width", vec![(pred!(const, [], []), _)]),
pub fn cv_structured_light_SinusoidalPattern_Params_propWidth_const(instance: *const c_void) -> i32;
// cv::structured_light::SinusoidalPattern::Params::setWidth(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:85
// ("cv::structured_light::SinusoidalPattern::Params::setWidth", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_structured_light_SinusoidalPattern_Params_propWidth_const_int(instance: *mut c_void, val: i32);
// cv::structured_light::SinusoidalPattern::Params::height() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:86
// ("cv::structured_light::SinusoidalPattern::Params::height", vec![(pred!(const, [], []), _)]),
pub fn cv_structured_light_SinusoidalPattern_Params_propHeight_const(instance: *const c_void) -> i32;
// cv::structured_light::SinusoidalPattern::Params::setHeight(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:86
// ("cv::structured_light::SinusoidalPattern::Params::setHeight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_structured_light_SinusoidalPattern_Params_propHeight_const_int(instance: *mut c_void, val: i32);
// cv::structured_light::SinusoidalPattern::Params::nbrOfPeriods() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:87
// ("cv::structured_light::SinusoidalPattern::Params::nbrOfPeriods", vec![(pred!(const, [], []), _)]),
pub fn cv_structured_light_SinusoidalPattern_Params_propNbrOfPeriods_const(instance: *const c_void) -> i32;
// cv::structured_light::SinusoidalPattern::Params::setNbrOfPeriods(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:87
// ("cv::structured_light::SinusoidalPattern::Params::setNbrOfPeriods", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_structured_light_SinusoidalPattern_Params_propNbrOfPeriods_const_int(instance: *mut c_void, val: i32);
// cv::structured_light::SinusoidalPattern::Params::shiftValue() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:88
// ("cv::structured_light::SinusoidalPattern::Params::shiftValue", vec![(pred!(const, [], []), _)]),
pub fn cv_structured_light_SinusoidalPattern_Params_propShiftValue_const(instance: *const c_void) -> f32;
// cv::structured_light::SinusoidalPattern::Params::setShiftValue(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:88
// ("cv::structured_light::SinusoidalPattern::Params::setShiftValue", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_structured_light_SinusoidalPattern_Params_propShiftValue_const_float(instance: *mut c_void, val: f32);
// cv::structured_light::SinusoidalPattern::Params::methodId() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:89
// ("cv::structured_light::SinusoidalPattern::Params::methodId", vec![(pred!(const, [], []), _)]),
pub fn cv_structured_light_SinusoidalPattern_Params_propMethodId_const(instance: *const c_void) -> i32;
// cv::structured_light::SinusoidalPattern::Params::setMethodId(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:89
// ("cv::structured_light::SinusoidalPattern::Params::setMethodId", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_structured_light_SinusoidalPattern_Params_propMethodId_const_int(instance: *mut c_void, val: i32);
// cv::structured_light::SinusoidalPattern::Params::nbrOfPixelsBetweenMarkers() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:90
// ("cv::structured_light::SinusoidalPattern::Params::nbrOfPixelsBetweenMarkers", vec![(pred!(const, [], []), _)]),
pub fn cv_structured_light_SinusoidalPattern_Params_propNbrOfPixelsBetweenMarkers_const(instance: *const c_void) -> i32;
// cv::structured_light::SinusoidalPattern::Params::setNbrOfPixelsBetweenMarkers(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:90
// ("cv::structured_light::SinusoidalPattern::Params::setNbrOfPixelsBetweenMarkers", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_structured_light_SinusoidalPattern_Params_propNbrOfPixelsBetweenMarkers_const_int(instance: *mut c_void, val: i32);
// cv::structured_light::SinusoidalPattern::Params::horizontal() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:91
// ("cv::structured_light::SinusoidalPattern::Params::horizontal", vec![(pred!(const, [], []), _)]),
pub fn cv_structured_light_SinusoidalPattern_Params_propHorizontal_const(instance: *const c_void) -> bool;
// cv::structured_light::SinusoidalPattern::Params::setHorizontal(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:91
// ("cv::structured_light::SinusoidalPattern::Params::setHorizontal", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_structured_light_SinusoidalPattern_Params_propHorizontal_const_bool(instance: *mut c_void, val: bool);
// cv::structured_light::SinusoidalPattern::Params::setMarkers() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:92
// ("cv::structured_light::SinusoidalPattern::Params::setMarkers", vec![(pred!(const, [], []), _)]),
pub fn cv_structured_light_SinusoidalPattern_Params_propSetMarkers_const(instance: *const c_void) -> bool;
// cv::structured_light::SinusoidalPattern::Params::setSetMarkers(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:92
// ("cv::structured_light::SinusoidalPattern::Params::setSetMarkers", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_structured_light_SinusoidalPattern_Params_propSetMarkers_const_bool(instance: *mut c_void, val: bool);
// cv::structured_light::SinusoidalPattern::Params::markersLocation() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:93
// ("cv::structured_light::SinusoidalPattern::Params::markersLocation", vec![(pred!(const, [], []), _)]),
pub fn cv_structured_light_SinusoidalPattern_Params_propMarkersLocation_const(instance: *const c_void) -> *mut c_void;
// cv::structured_light::SinusoidalPattern::Params::setMarkersLocation(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/sinusoidalpattern.hpp:93
// ("cv::structured_light::SinusoidalPattern::Params::setMarkersLocation", vec![(pred!(mut, ["val"], ["const std::vector<cv::Point2f>"]), _)]),
pub fn cv_structured_light_SinusoidalPattern_Params_propMarkersLocation_const_vectorLPoint2fG(instance: *mut c_void, val: *const c_void);
// cv::structured_light::SinusoidalPattern::Params::delete() generated
// ("cv::structured_light::SinusoidalPattern::Params::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_structured_light_SinusoidalPattern_Params_delete(instance: *mut c_void);
// generate(OutputArrayOfArrays)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/structured_light.hpp:69
// ("cv::structured_light::StructuredLightPattern::generate", vec![(pred!(mut, ["patternImages"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_structured_light_StructuredLightPattern_generate_const__OutputArrayR(instance: *mut c_void, pattern_images: *const c_void, ocvrs_return: *mut Result<bool>);
// decode(const std::vector<std::vector<Mat>> &, OutputArray, InputArrayOfArrays, InputArrayOfArrays, int)(CppPassByVoidPtr, OutputArray, InputArray, InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/structured_light.hpp:81
// ("cv::structured_light::StructuredLightPattern::decode", vec![(pred!(const, ["patternImages", "disparityMap", "blackImages", "whiteImages", "flags"], ["const std::vector<std::vector<cv::Mat>>*", "const cv::_OutputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "int"]), _)]),
pub fn cv_structured_light_StructuredLightPattern_decode_const_const_vectorLvectorLMatGGR_const__OutputArrayR_const__InputArrayR_const__InputArrayR_int(instance: *const c_void, pattern_images: *const c_void, disparity_map: *const c_void, black_images: *const c_void, white_images: *const c_void, flags: i32, ocvrs_return: *mut Result<bool>);
// cv::structured_light::StructuredLightPattern::decode(CppPassByVoidPtr, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/structured_light/structured_light.hpp:81
// ("cv::structured_light::StructuredLightPattern::decode", vec![(pred!(const, ["patternImages", "disparityMap"], ["const std::vector<std::vector<cv::Mat>>*", "const cv::_OutputArray*"]), _)]),
pub fn cv_structured_light_StructuredLightPattern_decode_const_const_vectorLvectorLMatGGR_const__OutputArrayR(instance: *const c_void, pattern_images: *const c_void, disparity_map: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::structured_light::StructuredLightPattern::to_GrayCodePattern() generated
// ("cv::structured_light::StructuredLightPattern::to_GrayCodePattern", vec![(pred!(mut, [], []), _)]),
pub fn cv_structured_light_StructuredLightPattern_to_GrayCodePattern(instance: *mut c_void) -> *mut c_void;
// cv::structured_light::StructuredLightPattern::to_SinusoidalPattern() generated
// ("cv::structured_light::StructuredLightPattern::to_SinusoidalPattern", vec![(pred!(mut, [], []), _)]),
pub fn cv_structured_light_StructuredLightPattern_to_SinusoidalPattern(instance: *mut c_void) -> *mut c_void;
// cv::structured_light::StructuredLightPattern::to_Algorithm() generated
// ("cv::structured_light::StructuredLightPattern::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_structured_light_StructuredLightPattern_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::structured_light::StructuredLightPattern::delete() generated
// ("cv::structured_light::StructuredLightPattern::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_structured_light_StructuredLightPattern_delete(instance: *mut c_void);
