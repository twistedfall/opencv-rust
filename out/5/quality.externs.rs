// compute(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitybrisque.hpp:33
// ("cv::quality::QualityBRISQUE::compute", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
pub fn cv_quality_QualityBRISQUE_compute_const__InputArrayR(instance: *mut c_void, img: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
// create(const cv::String &, const cv::String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitybrisque.hpp:40
// ("cv::quality::QualityBRISQUE::create", vec![(pred!(mut, ["model_file_path", "range_file_path"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_quality_QualityBRISQUE_create_const_StringR_const_StringR(model_file_path: *const c_char, range_file_path: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// create(const cv::Ptr<cv::ml::SVM> &, const cv::Mat &)(CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitybrisque.hpp:47
// ("cv::quality::QualityBRISQUE::create", vec![(pred!(mut, ["model", "range"], ["const cv::Ptr<cv::ml::SVM>*", "const cv::Mat*"]), _)]),
pub fn cv_quality_QualityBRISQUE_create_const_PtrLSVMGR_const_MatR(model: *const c_void, range: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// compute(InputArray, const cv::String &, const cv::String &)(InputArray, InString, InString) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitybrisque.hpp:56
// ("cv::quality::QualityBRISQUE::compute", vec![(pred!(mut, ["img", "model_file_path", "range_file_path"], ["const cv::_InputArray*", "const cv::String*", "const cv::String*"]), _)]),
pub fn cv_quality_QualityBRISQUE_compute_const__InputArrayR_const_StringR_const_StringR(img: *const c_void, model_file_path: *const c_char, range_file_path: *const c_char, ocvrs_return: *mut Result<core::Scalar>);
// computeFeatures(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitybrisque.hpp:63
// ("cv::quality::QualityBRISQUE::computeFeatures", vec![(pred!(mut, ["img", "features"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_quality_QualityBRISQUE_computeFeatures_const__InputArrayR_const__OutputArrayR(img: *const c_void, features: *const c_void, ocvrs_return: *mut Result<()>);
// cv::quality::QualityBRISQUE::to_Algorithm() generated
// ("cv::quality::QualityBRISQUE::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityBRISQUE_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::quality::QualityBRISQUE::to_QualityBase() generated
// ("cv::quality::QualityBRISQUE::to_QualityBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityBRISQUE_to_QualityBase(instance: *mut c_void) -> *mut c_void;
// cv::quality::QualityBRISQUE::delete() generated
// ("cv::quality::QualityBRISQUE::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityBRISQUE_delete(instance: *mut c_void);
// compute(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitybase.hpp:35
// ("cv::quality::QualityBase::compute", vec![(pred!(mut, ["img"], ["const cv::_InputArray*"]), _)]),
pub fn cv_quality_QualityBase_compute_const__InputArrayR(instance: *mut c_void, img: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
// getQualityMap(OutputArray)(OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitybase.hpp:38
// ("cv::quality::QualityBase::getQualityMap", vec![(pred!(const, ["dst"], ["const cv::_OutputArray*"]), _)]),
pub fn cv_quality_QualityBase_getQualityMap_const_const__OutputArrayR(instance: *const c_void, dst: *const c_void, ocvrs_return: *mut Result<()>);
// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitybase.hpp:46
// ("cv::quality::QualityBase::clear", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityBase_clear(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitybase.hpp:49
// ("cv::quality::QualityBase::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_quality_QualityBase_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::quality::QualityBase::to_QualityBRISQUE() generated
// ("cv::quality::QualityBase::to_QualityBRISQUE", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityBase_to_QualityBRISQUE(instance: *mut c_void) -> *mut c_void;
// cv::quality::QualityBase::to_QualityGMSD() generated
// ("cv::quality::QualityBase::to_QualityGMSD", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityBase_to_QualityGMSD(instance: *mut c_void) -> *mut c_void;
// cv::quality::QualityBase::to_QualityMSE() generated
// ("cv::quality::QualityBase::to_QualityMSE", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityBase_to_QualityMSE(instance: *mut c_void) -> *mut c_void;
// cv::quality::QualityBase::to_QualityPSNR() generated
// ("cv::quality::QualityBase::to_QualityPSNR", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityBase_to_QualityPSNR(instance: *mut c_void) -> *mut c_void;
// cv::quality::QualityBase::to_QualitySSIM() generated
// ("cv::quality::QualityBase::to_QualitySSIM", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityBase_to_QualitySSIM(instance: *mut c_void) -> *mut c_void;
// cv::quality::QualityBase::to_Algorithm() generated
// ("cv::quality::QualityBase::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityBase_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::quality::QualityBase::delete() generated
// ("cv::quality::QualityBase::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityBase_delete(instance: *mut c_void);
// compute(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitygmsd.hpp:28
// ("cv::quality::QualityGMSD::compute", vec![(pred!(mut, ["cmp"], ["const cv::_InputArray*"]), _)]),
pub fn cv_quality_QualityGMSD_compute_const__InputArrayR(instance: *mut c_void, cmp: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitygmsd.hpp:31
// ("cv::quality::QualityGMSD::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_quality_QualityGMSD_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitygmsd.hpp:34
// ("cv::quality::QualityGMSD::clear", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityGMSD_clear(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// create(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitygmsd.hpp:40
// ("cv::quality::QualityGMSD::create", vec![(pred!(mut, ["ref"], ["const cv::_InputArray*"]), _)]),
pub fn cv_quality_QualityGMSD_create_const__InputArrayR(ref_: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// compute(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitygmsd.hpp:49
// ("cv::quality::QualityGMSD::compute", vec![(pred!(mut, ["ref", "cmp", "qualityMap"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_quality_QualityGMSD_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(ref_: *const c_void, cmp: *const c_void, quality_map: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
// cv::quality::QualityGMSD::to_Algorithm() generated
// ("cv::quality::QualityGMSD::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityGMSD_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::quality::QualityGMSD::to_QualityBase() generated
// ("cv::quality::QualityGMSD::to_QualityBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityGMSD_to_QualityBase(instance: *mut c_void) -> *mut c_void;
// cv::quality::QualityGMSD::delete() generated
// ("cv::quality::QualityGMSD::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityGMSD_delete(instance: *mut c_void);
// compute(InputArrayOfArrays)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitymse.hpp:25
// ("cv::quality::QualityMSE::compute", vec![(pred!(mut, ["cmpImgs"], ["const cv::_InputArray*"]), _)]),
pub fn cv_quality_QualityMSE_compute_const__InputArrayR(instance: *mut c_void, cmp_imgs: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitymse.hpp:28
// ("cv::quality::QualityMSE::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_quality_QualityMSE_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitymse.hpp:31
// ("cv::quality::QualityMSE::clear", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityMSE_clear(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// create(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitymse.hpp:37
// ("cv::quality::QualityMSE::create", vec![(pred!(mut, ["ref"], ["const cv::_InputArray*"]), _)]),
pub fn cv_quality_QualityMSE_create_const__InputArrayR(ref_: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// compute(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitymse.hpp:46
// ("cv::quality::QualityMSE::compute", vec![(pred!(mut, ["ref", "cmp", "qualityMap"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_quality_QualityMSE_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(ref_: *const c_void, cmp: *const c_void, quality_map: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
// cv::quality::QualityMSE::to_Algorithm() generated
// ("cv::quality::QualityMSE::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityMSE_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::quality::QualityMSE::to_QualityBase() generated
// ("cv::quality::QualityMSE::to_QualityBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityMSE_to_QualityBase(instance: *mut c_void) -> *mut c_void;
// cv::quality::QualityMSE::delete() generated
// ("cv::quality::QualityMSE::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityMSE_delete(instance: *mut c_void);
// create(InputArray, double)(InputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitypsnr.hpp:38
// ("cv::quality::QualityPSNR::create", vec![(pred!(mut, ["ref", "maxPixelValue"], ["const cv::_InputArray*", "double"]), _)]),
pub fn cv_quality_QualityPSNR_create_const__InputArrayR_double(ref_: *const c_void, max_pixel_value: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::quality::QualityPSNR::create(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitypsnr.hpp:38
// ("cv::quality::QualityPSNR::create", vec![(pred!(mut, ["ref"], ["const cv::_InputArray*"]), _)]),
pub fn cv_quality_QualityPSNR_create_const__InputArrayR(ref_: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// compute(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitypsnr.hpp:48
// ("cv::quality::QualityPSNR::compute", vec![(pred!(mut, ["cmp"], ["const cv::_InputArray*"]), _)]),
pub fn cv_quality_QualityPSNR_compute_const__InputArrayR(instance: *mut c_void, cmp: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitypsnr.hpp:59
// ("cv::quality::QualityPSNR::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_quality_QualityPSNR_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitypsnr.hpp:62
// ("cv::quality::QualityPSNR::clear", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityPSNR_clear(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// compute(InputArray, InputArray, OutputArray, double)(InputArray, InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitypsnr.hpp:72
// ("cv::quality::QualityPSNR::compute", vec![(pred!(mut, ["ref", "cmp", "qualityMap", "maxPixelValue"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "double"]), _)]),
pub fn cv_quality_QualityPSNR_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double(ref_: *const c_void, cmp: *const c_void, quality_map: *const c_void, max_pixel_value: f64, ocvrs_return: *mut Result<core::Scalar>);
// cv::quality::QualityPSNR::compute(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitypsnr.hpp:72
// ("cv::quality::QualityPSNR::compute", vec![(pred!(mut, ["ref", "cmp", "qualityMap"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_quality_QualityPSNR_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(ref_: *const c_void, cmp: *const c_void, quality_map: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
// getMaxPixelValue()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitypsnr.hpp:81
// ("cv::quality::QualityPSNR::getMaxPixelValue", vec![(pred!(const, [], []), _)]),
pub fn cv_quality_QualityPSNR_getMaxPixelValue_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setMaxPixelValue(double)(Primitive) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualitypsnr.hpp:87
// ("cv::quality::QualityPSNR::setMaxPixelValue", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_quality_QualityPSNR_setMaxPixelValue_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// cv::quality::QualityPSNR::to_Algorithm() generated
// ("cv::quality::QualityPSNR::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityPSNR_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::quality::QualityPSNR::to_QualityBase() generated
// ("cv::quality::QualityPSNR::to_QualityBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityPSNR_to_QualityBase(instance: *mut c_void) -> *mut c_void;
// cv::quality::QualityPSNR::delete() generated
// ("cv::quality::QualityPSNR::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualityPSNR_delete(instance: *mut c_void);
// compute(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualityssim.hpp:27
// ("cv::quality::QualitySSIM::compute", vec![(pred!(mut, ["cmp"], ["const cv::_InputArray*"]), _)]),
pub fn cv_quality_QualitySSIM_compute_const__InputArrayR(instance: *mut c_void, cmp: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
// empty()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualityssim.hpp:30
// ("cv::quality::QualitySSIM::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_quality_QualitySSIM_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// clear()() /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualityssim.hpp:33
// ("cv::quality::QualitySSIM::clear", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualitySSIM_clear(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// create(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualityssim.hpp:39
// ("cv::quality::QualitySSIM::create", vec![(pred!(mut, ["ref"], ["const cv::_InputArray*"]), _)]),
pub fn cv_quality_QualitySSIM_create_const__InputArrayR(ref_: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// compute(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-5/install/include/opencv5/opencv2/quality/qualityssim.hpp:48
// ("cv::quality::QualitySSIM::compute", vec![(pred!(mut, ["ref", "cmp", "qualityMap"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_quality_QualitySSIM_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(ref_: *const c_void, cmp: *const c_void, quality_map: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
// cv::quality::QualitySSIM::to_Algorithm() generated
// ("cv::quality::QualitySSIM::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualitySSIM_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::quality::QualitySSIM::to_QualityBase() generated
// ("cv::quality::QualitySSIM::to_QualityBase", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualitySSIM_to_QualityBase(instance: *mut c_void) -> *mut c_void;
// cv::quality::QualitySSIM::delete() generated
// ("cv::quality::QualitySSIM::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_quality_QualitySSIM_delete(instance: *mut c_void);
