// EMDL1(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/emdL1.hpp:66
// ("cv::EMDL1", vec![(pred!(mut, ["signature1", "signature2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_EMDL1_const__InputArrayR_const__InputArrayR(signature1: *const c_void, signature2: *const c_void, ocvrs_return: *mut Result<f32>);
// createAffineTransformer(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_transformer.hpp:127
// ("cv::createAffineTransformer", vec![(pred!(mut, ["fullAffine"], ["bool"]), _)]),
pub fn cv_createAffineTransformer_bool(full_affine: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::createChiHistogramCostExtractor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/hist_cost.hpp:98
// ("cv::createChiHistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_createChiHistogramCostExtractor(ocvrs_return: *mut Result<*mut c_void>);
// createChiHistogramCostExtractor(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/hist_cost.hpp:98
// ("cv::createChiHistogramCostExtractor", vec![(pred!(mut, ["nDummies", "defaultCost"], ["int", "float"]), _)]),
pub fn cv_createChiHistogramCostExtractor_int_float(n_dummies: i32, default_cost: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::createEMDHistogramCostExtractor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/hist_cost.hpp:91
// ("cv::createEMDHistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_createEMDHistogramCostExtractor(ocvrs_return: *mut Result<*mut c_void>);
// createEMDHistogramCostExtractor(int, int, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/hist_cost.hpp:91
// ("cv::createEMDHistogramCostExtractor", vec![(pred!(mut, ["flag", "nDummies", "defaultCost"], ["int", "int", "float"]), _)]),
pub fn cv_createEMDHistogramCostExtractor_int_int_float(flag: i32, n_dummies: i32, default_cost: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::createEMDL1HistogramCostExtractor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/hist_cost.hpp:106
// ("cv::createEMDL1HistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_createEMDL1HistogramCostExtractor(ocvrs_return: *mut Result<*mut c_void>);
// createEMDL1HistogramCostExtractor(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/hist_cost.hpp:106
// ("cv::createEMDL1HistogramCostExtractor", vec![(pred!(mut, ["nDummies", "defaultCost"], ["int", "float"]), _)]),
pub fn cv_createEMDL1HistogramCostExtractor_int_float(n_dummies: i32, default_cost: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::createHausdorffDistanceExtractor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:222
// ("cv::createHausdorffDistanceExtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_createHausdorffDistanceExtractor(ocvrs_return: *mut Result<*mut c_void>);
// createHausdorffDistanceExtractor(int, float)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:222
// ("cv::createHausdorffDistanceExtractor", vec![(pred!(mut, ["distanceFlag", "rankProp"], ["int", "float"]), _)]),
pub fn cv_createHausdorffDistanceExtractor_int_float(distance_flag: i32, rank_prop: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::createNormHistogramCostExtractor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/hist_cost.hpp:79
// ("cv::createNormHistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_createNormHistogramCostExtractor(ocvrs_return: *mut Result<*mut c_void>);
// createNormHistogramCostExtractor(int, int, float)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/hist_cost.hpp:79
// ("cv::createNormHistogramCostExtractor", vec![(pred!(mut, ["flag", "nDummies", "defaultCost"], ["int", "int", "float"]), _)]),
pub fn cv_createNormHistogramCostExtractor_int_int_float(flag: i32, n_dummies: i32, default_cost: f32, ocvrs_return: *mut Result<*mut c_void>);
// cv::createShapeContextDistanceExtractor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:187
// ("cv::createShapeContextDistanceExtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_createShapeContextDistanceExtractor(ocvrs_return: *mut Result<*mut c_void>);
// createShapeContextDistanceExtractor(int, int, float, float, int, const Ptr<HistogramCostExtractor> &, const Ptr<ShapeTransformer> &)(Primitive, Primitive, Primitive, Primitive, Primitive, CppPassByVoidPtr, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:187
// ("cv::createShapeContextDistanceExtractor", vec![(pred!(mut, ["nAngularBins", "nRadialBins", "innerRadius", "outerRadius", "iterations", "comparer", "transformer"], ["int", "int", "float", "float", "int", "const cv::Ptr<cv::HistogramCostExtractor>*", "const cv::Ptr<cv::ShapeTransformer>*"]), _)]),
pub fn cv_createShapeContextDistanceExtractor_int_int_float_float_int_const_PtrLHistogramCostExtractorGR_const_PtrLShapeTransformerGR(n_angular_bins: i32, n_radial_bins: i32, inner_radius: f32, outer_radius: f32, iterations: i32, comparer: *const c_void, transformer: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::createThinPlateSplineShapeTransformer() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_transformer.hpp:112
// ("cv::createThinPlateSplineShapeTransformer", vec![(pred!(mut, [], []), _)]),
pub fn cv_createThinPlateSplineShapeTransformer(ocvrs_return: *mut Result<*mut c_void>);
// createThinPlateSplineShapeTransformer(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_transformer.hpp:112
// ("cv::createThinPlateSplineShapeTransformer", vec![(pred!(mut, ["regularizationParameter"], ["double"]), _)]),
pub fn cv_createThinPlateSplineShapeTransformer_double(regularization_parameter: f64, ocvrs_return: *mut Result<*mut c_void>);
// setFullAffine(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_transformer.hpp:122
// ("cv::AffineTransformer::setFullAffine", vec![(pred!(mut, ["fullAffine"], ["bool"]), _)]),
pub fn cv_AffineTransformer_setFullAffine_bool(instance: *mut c_void, full_affine: bool, ocvrs_return: *mut Result<()>);
// getFullAffine()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_transformer.hpp:123
// ("cv::AffineTransformer::getFullAffine", vec![(pred!(const, [], []), _)]),
pub fn cv_AffineTransformer_getFullAffine_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::AffineTransformer::to_Algorithm() generated
// ("cv::AffineTransformer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_AffineTransformer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::AffineTransformer::to_ShapeTransformer() generated
// ("cv::AffineTransformer::to_ShapeTransformer", vec![(pred!(mut, [], []), _)]),
pub fn cv_AffineTransformer_to_ShapeTransformer(instance: *mut c_void) -> *mut c_void;
// cv::AffineTransformer::delete() generated
// ("cv::AffineTransformer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_AffineTransformer_delete(instance: *mut c_void);
// cv::ChiHistogramCostExtractor::to_Algorithm() generated
// ("cv::ChiHistogramCostExtractor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ChiHistogramCostExtractor_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ChiHistogramCostExtractor::to_HistogramCostExtractor() generated
// ("cv::ChiHistogramCostExtractor::to_HistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_ChiHistogramCostExtractor_to_HistogramCostExtractor(instance: *mut c_void) -> *mut c_void;
// cv::ChiHistogramCostExtractor::delete() generated
// ("cv::ChiHistogramCostExtractor::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ChiHistogramCostExtractor_delete(instance: *mut c_void);
// setNormFlag(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/hist_cost.hpp:86
// ("cv::EMDHistogramCostExtractor::setNormFlag", vec![(pred!(mut, ["flag"], ["int"]), _)]),
pub fn cv_EMDHistogramCostExtractor_setNormFlag_int(instance: *mut c_void, flag: i32, ocvrs_return: *mut Result<()>);
// getNormFlag()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/hist_cost.hpp:87
// ("cv::EMDHistogramCostExtractor::getNormFlag", vec![(pred!(const, [], []), _)]),
pub fn cv_EMDHistogramCostExtractor_getNormFlag_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::EMDHistogramCostExtractor::to_Algorithm() generated
// ("cv::EMDHistogramCostExtractor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_EMDHistogramCostExtractor_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::EMDHistogramCostExtractor::to_HistogramCostExtractor() generated
// ("cv::EMDHistogramCostExtractor::to_HistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_EMDHistogramCostExtractor_to_HistogramCostExtractor(instance: *mut c_void) -> *mut c_void;
// cv::EMDHistogramCostExtractor::delete() generated
// ("cv::EMDHistogramCostExtractor::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_EMDHistogramCostExtractor_delete(instance: *mut c_void);
// cv::EMDL1HistogramCostExtractor::to_Algorithm() generated
// ("cv::EMDL1HistogramCostExtractor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_EMDL1HistogramCostExtractor_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::EMDL1HistogramCostExtractor::to_HistogramCostExtractor() generated
// ("cv::EMDL1HistogramCostExtractor::to_HistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_EMDL1HistogramCostExtractor_to_HistogramCostExtractor(instance: *mut c_void) -> *mut c_void;
// cv::EMDL1HistogramCostExtractor::delete() generated
// ("cv::EMDL1HistogramCostExtractor::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_EMDL1HistogramCostExtractor_delete(instance: *mut c_void);
// setDistanceFlag(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:208
// ("cv::HausdorffDistanceExtractor::setDistanceFlag", vec![(pred!(mut, ["distanceFlag"], ["int"]), _)]),
pub fn cv_HausdorffDistanceExtractor_setDistanceFlag_int(instance: *mut c_void, distance_flag: i32, ocvrs_return: *mut Result<()>);
// getDistanceFlag()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:209
// ("cv::HausdorffDistanceExtractor::getDistanceFlag", vec![(pred!(const, [], []), _)]),
pub fn cv_HausdorffDistanceExtractor_getDistanceFlag_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setRankProportion(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:217
// ("cv::HausdorffDistanceExtractor::setRankProportion", vec![(pred!(mut, ["rankProportion"], ["float"]), _)]),
pub fn cv_HausdorffDistanceExtractor_setRankProportion_float(instance: *mut c_void, rank_proportion: f32, ocvrs_return: *mut Result<()>);
// getRankProportion()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:218
// ("cv::HausdorffDistanceExtractor::getRankProportion", vec![(pred!(const, [], []), _)]),
pub fn cv_HausdorffDistanceExtractor_getRankProportion_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// cv::HausdorffDistanceExtractor::to_Algorithm() generated
// ("cv::HausdorffDistanceExtractor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_HausdorffDistanceExtractor_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::HausdorffDistanceExtractor::to_ShapeDistanceExtractor() generated
// ("cv::HausdorffDistanceExtractor::to_ShapeDistanceExtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_HausdorffDistanceExtractor_to_ShapeDistanceExtractor(instance: *mut c_void) -> *mut c_void;
// cv::HausdorffDistanceExtractor::delete() generated
// ("cv::HausdorffDistanceExtractor::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_HausdorffDistanceExtractor_delete(instance: *mut c_void);
// buildCostMatrix(InputArray, InputArray, OutputArray)(InputArray, InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/hist_cost.hpp:60
// ("cv::HistogramCostExtractor::buildCostMatrix", vec![(pred!(mut, ["descriptors1", "descriptors2", "costMatrix"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_HistogramCostExtractor_buildCostMatrix_const__InputArrayR_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, descriptors1: *const c_void, descriptors2: *const c_void, cost_matrix: *const c_void, ocvrs_return: *mut Result<()>);
// setNDummies(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/hist_cost.hpp:62
// ("cv::HistogramCostExtractor::setNDummies", vec![(pred!(mut, ["nDummies"], ["int"]), _)]),
pub fn cv_HistogramCostExtractor_setNDummies_int(instance: *mut c_void, n_dummies: i32, ocvrs_return: *mut Result<()>);
// getNDummies()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/hist_cost.hpp:63
// ("cv::HistogramCostExtractor::getNDummies", vec![(pred!(const, [], []), _)]),
pub fn cv_HistogramCostExtractor_getNDummies_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setDefaultCost(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/hist_cost.hpp:65
// ("cv::HistogramCostExtractor::setDefaultCost", vec![(pred!(mut, ["defaultCost"], ["float"]), _)]),
pub fn cv_HistogramCostExtractor_setDefaultCost_float(instance: *mut c_void, default_cost: f32, ocvrs_return: *mut Result<()>);
// getDefaultCost()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/hist_cost.hpp:66
// ("cv::HistogramCostExtractor::getDefaultCost", vec![(pred!(const, [], []), _)]),
pub fn cv_HistogramCostExtractor_getDefaultCost_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// cv::HistogramCostExtractor::to_ChiHistogramCostExtractor() generated
// ("cv::HistogramCostExtractor::to_ChiHistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_HistogramCostExtractor_to_ChiHistogramCostExtractor(instance: *mut c_void) -> *mut c_void;
// cv::HistogramCostExtractor::to_EMDHistogramCostExtractor() generated
// ("cv::HistogramCostExtractor::to_EMDHistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_HistogramCostExtractor_to_EMDHistogramCostExtractor(instance: *mut c_void) -> *mut c_void;
// cv::HistogramCostExtractor::to_EMDL1HistogramCostExtractor() generated
// ("cv::HistogramCostExtractor::to_EMDL1HistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_HistogramCostExtractor_to_EMDL1HistogramCostExtractor(instance: *mut c_void) -> *mut c_void;
// cv::HistogramCostExtractor::to_NormHistogramCostExtractor() generated
// ("cv::HistogramCostExtractor::to_NormHistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_HistogramCostExtractor_to_NormHistogramCostExtractor(instance: *mut c_void) -> *mut c_void;
// cv::HistogramCostExtractor::to_Algorithm() generated
// ("cv::HistogramCostExtractor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_HistogramCostExtractor_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::HistogramCostExtractor::delete() generated
// ("cv::HistogramCostExtractor::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_HistogramCostExtractor_delete(instance: *mut c_void);
// setNormFlag(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/hist_cost.hpp:74
// ("cv::NormHistogramCostExtractor::setNormFlag", vec![(pred!(mut, ["flag"], ["int"]), _)]),
pub fn cv_NormHistogramCostExtractor_setNormFlag_int(instance: *mut c_void, flag: i32, ocvrs_return: *mut Result<()>);
// getNormFlag()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/hist_cost.hpp:75
// ("cv::NormHistogramCostExtractor::getNormFlag", vec![(pred!(const, [], []), _)]),
pub fn cv_NormHistogramCostExtractor_getNormFlag_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// cv::NormHistogramCostExtractor::to_Algorithm() generated
// ("cv::NormHistogramCostExtractor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_NormHistogramCostExtractor_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::NormHistogramCostExtractor::to_HistogramCostExtractor() generated
// ("cv::NormHistogramCostExtractor::to_HistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_NormHistogramCostExtractor_to_HistogramCostExtractor(instance: *mut c_void) -> *mut c_void;
// cv::NormHistogramCostExtractor::delete() generated
// ("cv::NormHistogramCostExtractor::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_NormHistogramCostExtractor_delete(instance: *mut c_void);
// setAngularBins(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:89
// ("cv::ShapeContextDistanceExtractor::setAngularBins", vec![(pred!(mut, ["nAngularBins"], ["int"]), _)]),
pub fn cv_ShapeContextDistanceExtractor_setAngularBins_int(instance: *mut c_void, n_angular_bins: i32, ocvrs_return: *mut Result<()>);
// getAngularBins()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:90
// ("cv::ShapeContextDistanceExtractor::getAngularBins", vec![(pred!(const, [], []), _)]),
pub fn cv_ShapeContextDistanceExtractor_getAngularBins_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setRadialBins(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:97
// ("cv::ShapeContextDistanceExtractor::setRadialBins", vec![(pred!(mut, ["nRadialBins"], ["int"]), _)]),
pub fn cv_ShapeContextDistanceExtractor_setRadialBins_int(instance: *mut c_void, n_radial_bins: i32, ocvrs_return: *mut Result<()>);
// getRadialBins()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:98
// ("cv::ShapeContextDistanceExtractor::getRadialBins", vec![(pred!(const, [], []), _)]),
pub fn cv_ShapeContextDistanceExtractor_getRadialBins_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setInnerRadius(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:104
// ("cv::ShapeContextDistanceExtractor::setInnerRadius", vec![(pred!(mut, ["innerRadius"], ["float"]), _)]),
pub fn cv_ShapeContextDistanceExtractor_setInnerRadius_float(instance: *mut c_void, inner_radius: f32, ocvrs_return: *mut Result<()>);
// getInnerRadius()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:105
// ("cv::ShapeContextDistanceExtractor::getInnerRadius", vec![(pred!(const, [], []), _)]),
pub fn cv_ShapeContextDistanceExtractor_getInnerRadius_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setOuterRadius(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:111
// ("cv::ShapeContextDistanceExtractor::setOuterRadius", vec![(pred!(mut, ["outerRadius"], ["float"]), _)]),
pub fn cv_ShapeContextDistanceExtractor_setOuterRadius_float(instance: *mut c_void, outer_radius: f32, ocvrs_return: *mut Result<()>);
// getOuterRadius()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:112
// ("cv::ShapeContextDistanceExtractor::getOuterRadius", vec![(pred!(const, [], []), _)]),
pub fn cv_ShapeContextDistanceExtractor_getOuterRadius_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setRotationInvariant(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:114
// ("cv::ShapeContextDistanceExtractor::setRotationInvariant", vec![(pred!(mut, ["rotationInvariant"], ["bool"]), _)]),
pub fn cv_ShapeContextDistanceExtractor_setRotationInvariant_bool(instance: *mut c_void, rotation_invariant: bool, ocvrs_return: *mut Result<()>);
// getRotationInvariant()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:115
// ("cv::ShapeContextDistanceExtractor::getRotationInvariant", vec![(pred!(const, [], []), _)]),
pub fn cv_ShapeContextDistanceExtractor_getRotationInvariant_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setShapeContextWeight(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:124
// ("cv::ShapeContextDistanceExtractor::setShapeContextWeight", vec![(pred!(mut, ["shapeContextWeight"], ["float"]), _)]),
pub fn cv_ShapeContextDistanceExtractor_setShapeContextWeight_float(instance: *mut c_void, shape_context_weight: f32, ocvrs_return: *mut Result<()>);
// getShapeContextWeight()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:125
// ("cv::ShapeContextDistanceExtractor::getShapeContextWeight", vec![(pred!(const, [], []), _)]),
pub fn cv_ShapeContextDistanceExtractor_getShapeContextWeight_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setImageAppearanceWeight(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:136
// ("cv::ShapeContextDistanceExtractor::setImageAppearanceWeight", vec![(pred!(mut, ["imageAppearanceWeight"], ["float"]), _)]),
pub fn cv_ShapeContextDistanceExtractor_setImageAppearanceWeight_float(instance: *mut c_void, image_appearance_weight: f32, ocvrs_return: *mut Result<()>);
// getImageAppearanceWeight()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:137
// ("cv::ShapeContextDistanceExtractor::getImageAppearanceWeight", vec![(pred!(const, [], []), _)]),
pub fn cv_ShapeContextDistanceExtractor_getImageAppearanceWeight_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setBendingEnergyWeight(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:146
// ("cv::ShapeContextDistanceExtractor::setBendingEnergyWeight", vec![(pred!(mut, ["bendingEnergyWeight"], ["float"]), _)]),
pub fn cv_ShapeContextDistanceExtractor_setBendingEnergyWeight_float(instance: *mut c_void, bending_energy_weight: f32, ocvrs_return: *mut Result<()>);
// getBendingEnergyWeight()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:147
// ("cv::ShapeContextDistanceExtractor::getBendingEnergyWeight", vec![(pred!(const, [], []), _)]),
pub fn cv_ShapeContextDistanceExtractor_getBendingEnergyWeight_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setImages(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:155
// ("cv::ShapeContextDistanceExtractor::setImages", vec![(pred!(mut, ["image1", "image2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_ShapeContextDistanceExtractor_setImages_const__InputArrayR_const__InputArrayR(instance: *mut c_void, image1: *const c_void, image2: *const c_void, ocvrs_return: *mut Result<()>);
// getImages(OutputArray, OutputArray)(OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:156
// ("cv::ShapeContextDistanceExtractor::getImages", vec![(pred!(const, ["image1", "image2"], ["const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ShapeContextDistanceExtractor_getImages_const_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, image1: *const c_void, image2: *const c_void, ocvrs_return: *mut Result<()>);
// setIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:158
// ("cv::ShapeContextDistanceExtractor::setIterations", vec![(pred!(mut, ["iterations"], ["int"]), _)]),
pub fn cv_ShapeContextDistanceExtractor_setIterations_int(instance: *mut c_void, iterations: i32, ocvrs_return: *mut Result<()>);
// getIterations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:159
// ("cv::ShapeContextDistanceExtractor::getIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_ShapeContextDistanceExtractor_getIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setCostExtractor(Ptr<HistogramCostExtractor>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:166
// ("cv::ShapeContextDistanceExtractor::setCostExtractor", vec![(pred!(mut, ["comparer"], ["cv::Ptr<cv::HistogramCostExtractor>"]), _)]),
pub fn cv_ShapeContextDistanceExtractor_setCostExtractor_PtrLHistogramCostExtractorG(instance: *mut c_void, comparer: *mut c_void, ocvrs_return: *mut Result<()>);
// getCostExtractor()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:167
// ("cv::ShapeContextDistanceExtractor::getCostExtractor", vec![(pred!(const, [], []), _)]),
pub fn cv_ShapeContextDistanceExtractor_getCostExtractor_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setStdDev(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:173
// ("cv::ShapeContextDistanceExtractor::setStdDev", vec![(pred!(mut, ["sigma"], ["float"]), _)]),
pub fn cv_ShapeContextDistanceExtractor_setStdDev_float(instance: *mut c_void, sigma: f32, ocvrs_return: *mut Result<()>);
// getStdDev()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:174
// ("cv::ShapeContextDistanceExtractor::getStdDev", vec![(pred!(const, [], []), _)]),
pub fn cv_ShapeContextDistanceExtractor_getStdDev_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setTransformAlgorithm(Ptr<ShapeTransformer>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:181
// ("cv::ShapeContextDistanceExtractor::setTransformAlgorithm", vec![(pred!(mut, ["transformer"], ["cv::Ptr<cv::ShapeTransformer>"]), _)]),
pub fn cv_ShapeContextDistanceExtractor_setTransformAlgorithm_PtrLShapeTransformerG(instance: *mut c_void, transformer: *mut c_void, ocvrs_return: *mut Result<()>);
// getTransformAlgorithm()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:182
// ("cv::ShapeContextDistanceExtractor::getTransformAlgorithm", vec![(pred!(const, [], []), _)]),
pub fn cv_ShapeContextDistanceExtractor_getTransformAlgorithm_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::ShapeContextDistanceExtractor::to_Algorithm() generated
// ("cv::ShapeContextDistanceExtractor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ShapeContextDistanceExtractor_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ShapeContextDistanceExtractor::to_ShapeDistanceExtractor() generated
// ("cv::ShapeContextDistanceExtractor::to_ShapeDistanceExtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_ShapeContextDistanceExtractor_to_ShapeDistanceExtractor(instance: *mut c_void) -> *mut c_void;
// cv::ShapeContextDistanceExtractor::delete() generated
// ("cv::ShapeContextDistanceExtractor::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ShapeContextDistanceExtractor_delete(instance: *mut c_void);
// computeDistance(InputArray, InputArray)(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_distance.hpp:69
// ("cv::ShapeDistanceExtractor::computeDistance", vec![(pred!(mut, ["contour1", "contour2"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_ShapeDistanceExtractor_computeDistance_const__InputArrayR_const__InputArrayR(instance: *mut c_void, contour1: *const c_void, contour2: *const c_void, ocvrs_return: *mut Result<f32>);
// cv::ShapeDistanceExtractor::to_HausdorffDistanceExtractor() generated
// ("cv::ShapeDistanceExtractor::to_HausdorffDistanceExtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_ShapeDistanceExtractor_to_HausdorffDistanceExtractor(instance: *mut c_void) -> *mut c_void;
// cv::ShapeDistanceExtractor::to_ShapeContextDistanceExtractor() generated
// ("cv::ShapeDistanceExtractor::to_ShapeContextDistanceExtractor", vec![(pred!(mut, [], []), _)]),
pub fn cv_ShapeDistanceExtractor_to_ShapeContextDistanceExtractor(instance: *mut c_void) -> *mut c_void;
// cv::ShapeDistanceExtractor::to_Algorithm() generated
// ("cv::ShapeDistanceExtractor::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ShapeDistanceExtractor_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ShapeDistanceExtractor::delete() generated
// ("cv::ShapeDistanceExtractor::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ShapeDistanceExtractor_delete(instance: *mut c_void);
// estimateTransformation(InputArray, InputArray, std::vector<DMatch> &)(InputArray, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_transformer.hpp:67
// ("cv::ShapeTransformer::estimateTransformation", vec![(pred!(mut, ["transformingShape", "targetShape", "matches"], ["const cv::_InputArray*", "const cv::_InputArray*", "std::vector<cv::DMatch>*"]), _)]),
pub fn cv_ShapeTransformer_estimateTransformation_const__InputArrayR_const__InputArrayR_vectorLDMatchGR(instance: *mut c_void, transforming_shape: *const c_void, target_shape: *const c_void, matches: *mut c_void, ocvrs_return: *mut Result<()>);
// applyTransformation(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_transformer.hpp:75
// ("cv::ShapeTransformer::applyTransformation", vec![(pred!(mut, ["input", "output"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ShapeTransformer_applyTransformation_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, input: *const c_void, output: *const c_void, ocvrs_return: *mut Result<f32>);
// cv::ShapeTransformer::applyTransformation(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_transformer.hpp:75
// ("cv::ShapeTransformer::applyTransformation", vec![(pred!(mut, ["input"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ShapeTransformer_applyTransformation_const__InputArrayR(instance: *mut c_void, input: *const c_void, ocvrs_return: *mut Result<f32>);
// warpImage(InputArray, OutputArray, int, int, const Scalar &)(InputArray, OutputArray, Primitive, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_transformer.hpp:85
// ("cv::ShapeTransformer::warpImage", vec![(pred!(const, ["transformingImage", "output", "flags", "borderMode", "borderValue"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int", "int", "const cv::Scalar*"]), _)]),
pub fn cv_ShapeTransformer_warpImage_const_const__InputArrayR_const__OutputArrayR_int_int_const_ScalarR(instance: *const c_void, transforming_image: *const c_void, output: *const c_void, flags: i32, border_mode: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<()>);
// cv::ShapeTransformer::warpImage(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_transformer.hpp:85
// ("cv::ShapeTransformer::warpImage", vec![(pred!(const, ["transformingImage", "output"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ShapeTransformer_warpImage_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, transforming_image: *const c_void, output: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ShapeTransformer::to_AffineTransformer() generated
// ("cv::ShapeTransformer::to_AffineTransformer", vec![(pred!(mut, [], []), _)]),
pub fn cv_ShapeTransformer_to_AffineTransformer(instance: *mut c_void) -> *mut c_void;
// cv::ShapeTransformer::to_ThinPlateSplineShapeTransformer() generated
// ("cv::ShapeTransformer::to_ThinPlateSplineShapeTransformer", vec![(pred!(mut, [], []), _)]),
pub fn cv_ShapeTransformer_to_ThinPlateSplineShapeTransformer(instance: *mut c_void) -> *mut c_void;
// cv::ShapeTransformer::to_Algorithm() generated
// ("cv::ShapeTransformer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ShapeTransformer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ShapeTransformer::delete() generated
// ("cv::ShapeTransformer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ShapeTransformer_delete(instance: *mut c_void);
// setRegularizationParameter(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_transformer.hpp:106
// ("cv::ThinPlateSplineShapeTransformer::setRegularizationParameter", vec![(pred!(mut, ["beta"], ["double"]), _)]),
pub fn cv_ThinPlateSplineShapeTransformer_setRegularizationParameter_double(instance: *mut c_void, beta: f64, ocvrs_return: *mut Result<()>);
// getRegularizationParameter()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/shape/shape_transformer.hpp:107
// ("cv::ThinPlateSplineShapeTransformer::getRegularizationParameter", vec![(pred!(const, [], []), _)]),
pub fn cv_ThinPlateSplineShapeTransformer_getRegularizationParameter_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// cv::ThinPlateSplineShapeTransformer::to_Algorithm() generated
// ("cv::ThinPlateSplineShapeTransformer::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ThinPlateSplineShapeTransformer_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ThinPlateSplineShapeTransformer::to_ShapeTransformer() generated
// ("cv::ThinPlateSplineShapeTransformer::to_ShapeTransformer", vec![(pred!(mut, [], []), _)]),
pub fn cv_ThinPlateSplineShapeTransformer_to_ShapeTransformer(instance: *mut c_void) -> *mut c_void;
// cv::ThinPlateSplineShapeTransformer::delete() generated
// ("cv::ThinPlateSplineShapeTransformer::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ThinPlateSplineShapeTransformer_delete(instance: *mut c_void);
