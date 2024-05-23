// createConcentricSpheresTestSet(int, int, int, OutputArray, OutputArray)(Primitive, Primitive, Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1898
// ("cv::ml::createConcentricSpheresTestSet", vec![(pred!(mut, ["nsamples", "nfeatures", "nclasses", "samples", "responses"], ["int", "int", "int", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ml_createConcentricSpheresTestSet_int_int_int_const__OutputArrayR_const__OutputArrayR(nsamples: i32, nfeatures: i32, nclasses: i32, samples: *const c_void, responses: *const c_void, ocvrs_return: *mut Result<()>);
// randMVNormal(InputArray, InputArray, int, OutputArray)(InputArray, InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1895
// ("cv::ml::randMVNormal", vec![(pred!(mut, ["mean", "cov", "nsamples", "samples"], ["const cv::_InputArray*", "const cv::_InputArray*", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_ml_randMVNormal_const__InputArrayR_const__InputArrayR_int_const__OutputArrayR(mean: *const c_void, cov: *const c_void, nsamples: i32, samples: *const c_void, ocvrs_return: *mut Result<()>);
// setTrainMethod(int, double, double)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1446
// ("cv::ml::ANN_MLP::setTrainMethod", vec![(pred!(mut, ["method", "param1", "param2"], ["int", "double", "double"]), _)]),
pub fn cv_ml_ANN_MLP_setTrainMethod_int_double_double(instance: *mut c_void, method: i32, param1: f64, param2: f64, ocvrs_return: *mut Result<()>);
// cv::ml::ANN_MLP::setTrainMethod(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1446
// ("cv::ml::ANN_MLP::setTrainMethod", vec![(pred!(mut, ["method"], ["int"]), _)]),
pub fn cv_ml_ANN_MLP_setTrainMethod_int(instance: *mut c_void, method: i32, ocvrs_return: *mut Result<()>);
// getTrainMethod()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1449
// ("cv::ml::ANN_MLP::getTrainMethod", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_ANN_MLP_getTrainMethod_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setActivationFunction(int, double, double)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1457
// ("cv::ml::ANN_MLP::setActivationFunction", vec![(pred!(mut, ["type", "param1", "param2"], ["int", "double", "double"]), _)]),
pub fn cv_ml_ANN_MLP_setActivationFunction_int_double_double(instance: *mut c_void, typ: i32, param1: f64, param2: f64, ocvrs_return: *mut Result<()>);
// cv::ml::ANN_MLP::setActivationFunction(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1457
// ("cv::ml::ANN_MLP::setActivationFunction", vec![(pred!(mut, ["type"], ["int"]), _)]),
pub fn cv_ml_ANN_MLP_setActivationFunction_int(instance: *mut c_void, typ: i32, ocvrs_return: *mut Result<()>);
// setLayerSizes(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1463
// ("cv::ml::ANN_MLP::setLayerSizes", vec![(pred!(mut, ["_layer_sizes"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ml_ANN_MLP_setLayerSizes_const__InputArrayR(instance: *mut c_void, _layer_sizes: *const c_void, ocvrs_return: *mut Result<()>);
// getLayerSizes()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1469
// ("cv::ml::ANN_MLP::getLayerSizes", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_ANN_MLP_getLayerSizes_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getTermCriteria()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1476
// ("cv::ml::ANN_MLP::getTermCriteria", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_ANN_MLP_getTermCriteria_const(instance: *const c_void, ocvrs_return: *mut Result<core::TermCriteria>);
// setTermCriteria(TermCriteria)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1478
// ("cv::ml::ANN_MLP::setTermCriteria", vec![(pred!(mut, ["val"], ["cv::TermCriteria"]), _)]),
pub fn cv_ml_ANN_MLP_setTermCriteria_TermCriteria(instance: *mut c_void, val: *const core::TermCriteria, ocvrs_return: *mut Result<()>);
// getBackpropWeightScale()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1483
// ("cv::ml::ANN_MLP::getBackpropWeightScale", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_ANN_MLP_getBackpropWeightScale_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setBackpropWeightScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1485
// ("cv::ml::ANN_MLP::setBackpropWeightScale", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ml_ANN_MLP_setBackpropWeightScale_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getBackpropMomentumScale()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1492
// ("cv::ml::ANN_MLP::getBackpropMomentumScale", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_ANN_MLP_getBackpropMomentumScale_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setBackpropMomentumScale(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1494
// ("cv::ml::ANN_MLP::setBackpropMomentumScale", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ml_ANN_MLP_setBackpropMomentumScale_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getRpropDW0()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1499
// ("cv::ml::ANN_MLP::getRpropDW0", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_ANN_MLP_getRpropDW0_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setRpropDW0(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1501
// ("cv::ml::ANN_MLP::setRpropDW0", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ml_ANN_MLP_setRpropDW0_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getRpropDWPlus()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1506
// ("cv::ml::ANN_MLP::getRpropDWPlus", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_ANN_MLP_getRpropDWPlus_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setRpropDWPlus(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1508
// ("cv::ml::ANN_MLP::setRpropDWPlus", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ml_ANN_MLP_setRpropDWPlus_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getRpropDWMinus()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1513
// ("cv::ml::ANN_MLP::getRpropDWMinus", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_ANN_MLP_getRpropDWMinus_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setRpropDWMinus(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1515
// ("cv::ml::ANN_MLP::setRpropDWMinus", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ml_ANN_MLP_setRpropDWMinus_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getRpropDWMin()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1520
// ("cv::ml::ANN_MLP::getRpropDWMin", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_ANN_MLP_getRpropDWMin_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setRpropDWMin(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1522
// ("cv::ml::ANN_MLP::setRpropDWMin", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ml_ANN_MLP_setRpropDWMin_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getRpropDWMax()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1527
// ("cv::ml::ANN_MLP::getRpropDWMax", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_ANN_MLP_getRpropDWMax_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setRpropDWMax(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1529
// ("cv::ml::ANN_MLP::setRpropDWMax", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ml_ANN_MLP_setRpropDWMax_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getAnnealInitialT()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1534
// ("cv::ml::ANN_MLP::getAnnealInitialT", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_ANN_MLP_getAnnealInitialT_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setAnnealInitialT(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1536
// ("cv::ml::ANN_MLP::setAnnealInitialT", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ml_ANN_MLP_setAnnealInitialT_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getAnnealFinalT()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1541
// ("cv::ml::ANN_MLP::getAnnealFinalT", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_ANN_MLP_getAnnealFinalT_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setAnnealFinalT(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1543
// ("cv::ml::ANN_MLP::setAnnealFinalT", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ml_ANN_MLP_setAnnealFinalT_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getAnnealCoolingRatio()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1548
// ("cv::ml::ANN_MLP::getAnnealCoolingRatio", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_ANN_MLP_getAnnealCoolingRatio_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setAnnealCoolingRatio(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1550
// ("cv::ml::ANN_MLP::setAnnealCoolingRatio", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ml_ANN_MLP_setAnnealCoolingRatio_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getAnnealItePerStep()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1555
// ("cv::ml::ANN_MLP::getAnnealItePerStep", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_ANN_MLP_getAnnealItePerStep_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setAnnealItePerStep(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1557
// ("cv::ml::ANN_MLP::setAnnealItePerStep", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_ml_ANN_MLP_setAnnealItePerStep_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// setAnnealEnergyRNG(const RNG &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1560
// ("cv::ml::ANN_MLP::setAnnealEnergyRNG", vec![(pred!(mut, ["rng"], ["const cv::RNG*"]), _)]),
pub fn cv_ml_ANN_MLP_setAnnealEnergyRNG_const_RNGR(instance: *mut c_void, rng: *const c_void, ocvrs_return: *mut Result<()>);
// getWeights(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1597
// ("cv::ml::ANN_MLP::getWeights", vec![(pred!(const, ["layerIdx"], ["int"]), _)]),
pub fn cv_ml_ANN_MLP_getWeights_const_int(instance: *const c_void, layer_idx: i32, ocvrs_return: *mut Result<*mut c_void>);
// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1604
// ("cv::ml::ANN_MLP::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_ANN_MLP_create(ocvrs_return: *mut Result<*mut c_void>);
// load(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1613
// ("cv::ml::ANN_MLP::load", vec![(pred!(mut, ["filepath"], ["const cv::String*"]), _)]),
pub fn cv_ml_ANN_MLP_load_const_StringR(filepath: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::ANN_MLP::to_ANN_MLP_ANNEAL() generated
// ("cv::ml::ANN_MLP::to_ANN_MLP_ANNEAL", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_ANN_MLP_to_ANN_MLP_ANNEAL(instance: *mut c_void) -> *mut c_void;
// cv::ml::ANN_MLP::to_Algorithm() generated
// ("cv::ml::ANN_MLP::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_ANN_MLP_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ml::ANN_MLP::to_StatModel() generated
// ("cv::ml::ANN_MLP::to_StatModel", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_ANN_MLP_to_StatModel(instance: *mut c_void) -> *mut c_void;
// cv::ml::ANN_MLP::delete() generated
// ("cv::ml::ANN_MLP::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_ANN_MLP_delete(instance: *mut c_void);
// getAnnealInitialT()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1909
// ("cv::ml::ANN_MLP_ANNEAL::getAnnealInitialT", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_ANN_MLP_ANNEAL_getAnnealInitialT_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setAnnealInitialT(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1911
// ("cv::ml::ANN_MLP_ANNEAL::setAnnealInitialT", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ml_ANN_MLP_ANNEAL_setAnnealInitialT_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getAnnealFinalT()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1916
// ("cv::ml::ANN_MLP_ANNEAL::getAnnealFinalT", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_ANN_MLP_ANNEAL_getAnnealFinalT_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setAnnealFinalT(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1918
// ("cv::ml::ANN_MLP_ANNEAL::setAnnealFinalT", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ml_ANN_MLP_ANNEAL_setAnnealFinalT_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getAnnealCoolingRatio()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1923
// ("cv::ml::ANN_MLP_ANNEAL::getAnnealCoolingRatio", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_ANN_MLP_ANNEAL_getAnnealCoolingRatio_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setAnnealCoolingRatio(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1925
// ("cv::ml::ANN_MLP_ANNEAL::setAnnealCoolingRatio", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ml_ANN_MLP_ANNEAL_setAnnealCoolingRatio_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getAnnealItePerStep()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1930
// ("cv::ml::ANN_MLP_ANNEAL::getAnnealItePerStep", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_ANN_MLP_ANNEAL_getAnnealItePerStep_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setAnnealItePerStep(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1932
// ("cv::ml::ANN_MLP_ANNEAL::setAnnealItePerStep", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_ml_ANN_MLP_ANNEAL_setAnnealItePerStep_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// setAnnealEnergyRNG(const RNG &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1935
// ("cv::ml::ANN_MLP_ANNEAL::setAnnealEnergyRNG", vec![(pred!(mut, ["rng"], ["const cv::RNG*"]), _)]),
pub fn cv_ml_ANN_MLP_ANNEAL_setAnnealEnergyRNG_const_RNGR(instance: *mut c_void, rng: *const c_void, ocvrs_return: *mut Result<()>);
// cv::ml::ANN_MLP_ANNEAL::to_ANN_MLP() generated
// ("cv::ml::ANN_MLP_ANNEAL::to_ANN_MLP", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_ANN_MLP_ANNEAL_to_ANN_MLP(instance: *mut c_void) -> *mut c_void;
// cv::ml::ANN_MLP_ANNEAL::to_Algorithm() generated
// ("cv::ml::ANN_MLP_ANNEAL::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_ANN_MLP_ANNEAL_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ml::ANN_MLP_ANNEAL::to_StatModel() generated
// ("cv::ml::ANN_MLP_ANNEAL::to_StatModel", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_ANN_MLP_ANNEAL_to_StatModel(instance: *mut c_void) -> *mut c_void;
// cv::ml::ANN_MLP_ANNEAL::delete() generated
// ("cv::ml::ANN_MLP_ANNEAL::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_ANN_MLP_ANNEAL_delete(instance: *mut c_void);
// getBoostType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1338
// ("cv::ml::Boost::getBoostType", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_Boost_getBoostType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setBoostType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1340
// ("cv::ml::Boost::setBoostType", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_ml_Boost_setBoostType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getWeakCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1345
// ("cv::ml::Boost::getWeakCount", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_Boost_getWeakCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setWeakCount(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1347
// ("cv::ml::Boost::setWeakCount", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_ml_Boost_setWeakCount_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getWeightTrimRate()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1353
// ("cv::ml::Boost::getWeightTrimRate", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_Boost_getWeightTrimRate_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setWeightTrimRate(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1355
// ("cv::ml::Boost::setWeightTrimRate", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ml_Boost_setWeightTrimRate_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1370
// ("cv::ml::Boost::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_Boost_create(ocvrs_return: *mut Result<*mut c_void>);
// load(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1381
// ("cv::ml::Boost::load", vec![(pred!(mut, ["filepath", "nodeName"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_ml_Boost_load_const_StringR_const_StringR(filepath: *const c_char, node_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::Boost::load(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1381
// ("cv::ml::Boost::load", vec![(pred!(mut, ["filepath"], ["const cv::String*"]), _)]),
pub fn cv_ml_Boost_load_const_StringR(filepath: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::Boost::to_Algorithm() generated
// ("cv::ml::Boost::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_Boost_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ml::Boost::to_DTrees() generated
// ("cv::ml::Boost::to_DTrees", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_Boost_to_DTrees(instance: *mut c_void) -> *mut c_void;
// cv::ml::Boost::to_StatModel() generated
// ("cv::ml::Boost::to_StatModel", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_Boost_to_StatModel(instance: *mut c_void) -> *mut c_void;
// cv::ml::Boost::delete() generated
// ("cv::ml::Boost::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_Boost_delete(instance: *mut c_void);
// getMaxCategories()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1071
// ("cv::ml::DTrees::getMaxCategories", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_getMaxCategories_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMaxCategories(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1073
// ("cv::ml::DTrees::setMaxCategories", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_ml_DTrees_setMaxCategories_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getMaxDepth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1081
// ("cv::ml::DTrees::getMaxDepth", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_getMaxDepth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMaxDepth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1083
// ("cv::ml::DTrees::setMaxDepth", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_ml_DTrees_setMaxDepth_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getMinSampleCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1089
// ("cv::ml::DTrees::getMinSampleCount", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_getMinSampleCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMinSampleCount(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1091
// ("cv::ml::DTrees::setMinSampleCount", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_ml_DTrees_setMinSampleCount_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getCVFolds()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1097
// ("cv::ml::DTrees::getCVFolds", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_getCVFolds_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setCVFolds(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1099
// ("cv::ml::DTrees::setCVFolds", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_ml_DTrees_setCVFolds_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getUseSurrogates()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1106
// ("cv::ml::DTrees::getUseSurrogates", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_getUseSurrogates_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUseSurrogates(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1108
// ("cv::ml::DTrees::setUseSurrogates", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_ml_DTrees_setUseSurrogates_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// getUse1SERule()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1114
// ("cv::ml::DTrees::getUse1SERule", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_getUse1SERule_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setUse1SERule(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1116
// ("cv::ml::DTrees::setUse1SERule", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_ml_DTrees_setUse1SERule_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// getTruncatePrunedTree()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1122
// ("cv::ml::DTrees::getTruncatePrunedTree", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_getTruncatePrunedTree_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setTruncatePrunedTree(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1124
// ("cv::ml::DTrees::setTruncatePrunedTree", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_ml_DTrees_setTruncatePrunedTree_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// getRegressionAccuracy()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1131
// ("cv::ml::DTrees::getRegressionAccuracy", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_getRegressionAccuracy_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setRegressionAccuracy(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1133
// ("cv::ml::DTrees::setRegressionAccuracy", vec![(pred!(mut, ["val"], ["float"]), _)]),
pub fn cv_ml_DTrees_setRegressionAccuracy_float(instance: *mut c_void, val: f32, ocvrs_return: *mut Result<()>);
// getPriors()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1151
// ("cv::ml::DTrees::getPriors", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_getPriors_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setPriors(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1153
// ("cv::ml::DTrees::setPriors", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_ml_DTrees_setPriors_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getRoots()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1202
// ("cv::ml::DTrees::getRoots", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_getRoots_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getNodes()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1207
// ("cv::ml::DTrees::getNodes", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_getNodes_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getSplits()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1212
// ("cv::ml::DTrees::getSplits", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_getSplits_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getSubsets()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1217
// ("cv::ml::DTrees::getSubsets", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_getSubsets_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1225
// ("cv::ml::DTrees::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_DTrees_create(ocvrs_return: *mut Result<*mut c_void>);
// load(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1236
// ("cv::ml::DTrees::load", vec![(pred!(mut, ["filepath", "nodeName"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_ml_DTrees_load_const_StringR_const_StringR(filepath: *const c_char, node_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::DTrees::load(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1236
// ("cv::ml::DTrees::load", vec![(pred!(mut, ["filepath"], ["const cv::String*"]), _)]),
pub fn cv_ml_DTrees_load_const_StringR(filepath: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::DTrees::to_Boost() generated
// ("cv::ml::DTrees::to_Boost", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_DTrees_to_Boost(instance: *mut c_void) -> *mut c_void;
// cv::ml::DTrees::to_RTrees() generated
// ("cv::ml::DTrees::to_RTrees", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_DTrees_to_RTrees(instance: *mut c_void) -> *mut c_void;
// cv::ml::DTrees::to_Algorithm() generated
// ("cv::ml::DTrees::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_DTrees_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ml::DTrees::to_StatModel() generated
// ("cv::ml::DTrees::to_StatModel", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_DTrees_to_StatModel(instance: *mut c_void) -> *mut c_void;
// cv::ml::DTrees::delete() generated
// ("cv::ml::DTrees::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_DTrees_delete(instance: *mut c_void);
// Node()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1160
// ("cv::ml::DTrees::Node::Node", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_DTrees_Node_Node(ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::DTrees::Node::value() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1161
// ("cv::ml::DTrees::Node::value", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_Node_propValue_const(instance: *const c_void) -> f64;
// cv::ml::DTrees::Node::setValue(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1161
// ("cv::ml::DTrees::Node::setValue", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_ml_DTrees_Node_propValue_const_double(instance: *mut c_void, val: f64);
// cv::ml::DTrees::Node::classIdx() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1163
// ("cv::ml::DTrees::Node::classIdx", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_Node_propClassIdx_const(instance: *const c_void) -> i32;
// cv::ml::DTrees::Node::setClassIdx(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1163
// ("cv::ml::DTrees::Node::setClassIdx", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_ml_DTrees_Node_propClassIdx_const_int(instance: *mut c_void, val: i32);
// cv::ml::DTrees::Node::parent() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1165
// ("cv::ml::DTrees::Node::parent", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_Node_propParent_const(instance: *const c_void) -> i32;
// cv::ml::DTrees::Node::setParent(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1165
// ("cv::ml::DTrees::Node::setParent", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_ml_DTrees_Node_propParent_const_int(instance: *mut c_void, val: i32);
// cv::ml::DTrees::Node::left() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1166
// ("cv::ml::DTrees::Node::left", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_Node_propLeft_const(instance: *const c_void) -> i32;
// cv::ml::DTrees::Node::setLeft(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1166
// ("cv::ml::DTrees::Node::setLeft", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_ml_DTrees_Node_propLeft_const_int(instance: *mut c_void, val: i32);
// cv::ml::DTrees::Node::right() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1167
// ("cv::ml::DTrees::Node::right", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_Node_propRight_const(instance: *const c_void) -> i32;
// cv::ml::DTrees::Node::setRight(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1167
// ("cv::ml::DTrees::Node::setRight", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_ml_DTrees_Node_propRight_const_int(instance: *mut c_void, val: i32);
// cv::ml::DTrees::Node::defaultDir() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1168
// ("cv::ml::DTrees::Node::defaultDir", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_Node_propDefaultDir_const(instance: *const c_void) -> i32;
// cv::ml::DTrees::Node::setDefaultDir(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1168
// ("cv::ml::DTrees::Node::setDefaultDir", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_ml_DTrees_Node_propDefaultDir_const_int(instance: *mut c_void, val: i32);
// cv::ml::DTrees::Node::split() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1170
// ("cv::ml::DTrees::Node::split", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_Node_propSplit_const(instance: *const c_void) -> i32;
// cv::ml::DTrees::Node::setSplit(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1170
// ("cv::ml::DTrees::Node::setSplit", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_ml_DTrees_Node_propSplit_const_int(instance: *mut c_void, val: i32);
// cv::ml::DTrees::Node::delete() generated
// ("cv::ml::DTrees::Node::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_DTrees_Node_delete(instance: *mut c_void);
// Split()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1178
// ("cv::ml::DTrees::Split::Split", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_DTrees_Split_Split(ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::DTrees::Split::varIdx() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1179
// ("cv::ml::DTrees::Split::varIdx", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_Split_propVarIdx_const(instance: *const c_void) -> i32;
// cv::ml::DTrees::Split::setVarIdx(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1179
// ("cv::ml::DTrees::Split::setVarIdx", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_ml_DTrees_Split_propVarIdx_const_int(instance: *mut c_void, val: i32);
// cv::ml::DTrees::Split::inversed() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1180
// ("cv::ml::DTrees::Split::inversed", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_Split_propInversed_const(instance: *const c_void) -> bool;
// cv::ml::DTrees::Split::setInversed(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1180
// ("cv::ml::DTrees::Split::setInversed", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
pub fn cv_ml_DTrees_Split_propInversed_const_bool(instance: *mut c_void, val: bool);
// cv::ml::DTrees::Split::quality() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1182
// ("cv::ml::DTrees::Split::quality", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_Split_propQuality_const(instance: *const c_void) -> f32;
// cv::ml::DTrees::Split::setQuality(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1182
// ("cv::ml::DTrees::Split::setQuality", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_ml_DTrees_Split_propQuality_const_float(instance: *mut c_void, val: f32);
// cv::ml::DTrees::Split::next() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1183
// ("cv::ml::DTrees::Split::next", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_Split_propNext_const(instance: *const c_void) -> i32;
// cv::ml::DTrees::Split::setNext(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1183
// ("cv::ml::DTrees::Split::setNext", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_ml_DTrees_Split_propNext_const_int(instance: *mut c_void, val: i32);
// cv::ml::DTrees::Split::c() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1184
// ("cv::ml::DTrees::Split::c", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_Split_propC_const(instance: *const c_void) -> f32;
// cv::ml::DTrees::Split::setC(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1184
// ("cv::ml::DTrees::Split::setC", vec![(pred!(mut, ["val"], ["const float"]), _)]),
pub fn cv_ml_DTrees_Split_propC_const_float(instance: *mut c_void, val: f32);
// cv::ml::DTrees::Split::subsetOfs() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1191
// ("cv::ml::DTrees::Split::subsetOfs", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_DTrees_Split_propSubsetOfs_const(instance: *const c_void) -> i32;
// cv::ml::DTrees::Split::setSubsetOfs(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1191
// ("cv::ml::DTrees::Split::setSubsetOfs", vec![(pred!(mut, ["val"], ["const int"]), _)]),
pub fn cv_ml_DTrees_Split_propSubsetOfs_const_int(instance: *mut c_void, val: i32);
// cv::ml::DTrees::Split::delete() generated
// ("cv::ml::DTrees::Split::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_DTrees_Split_delete(instance: *mut c_void);
// getClustersNumber()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:871
// ("cv::ml::EM::getClustersNumber", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_EM_getClustersNumber_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setClustersNumber(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:873
// ("cv::ml::EM::setClustersNumber", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_ml_EM_setClustersNumber_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getCovarianceMatrixType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:878
// ("cv::ml::EM::getCovarianceMatrixType", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_EM_getCovarianceMatrixType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setCovarianceMatrixType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:880
// ("cv::ml::EM::setCovarianceMatrixType", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_ml_EM_setCovarianceMatrixType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getTermCriteria()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:887
// ("cv::ml::EM::getTermCriteria", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_EM_getTermCriteria_const(instance: *const c_void, ocvrs_return: *mut Result<core::TermCriteria>);
// setTermCriteria(const TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:889
// ("cv::ml::EM::setTermCriteria", vec![(pred!(mut, ["val"], ["const cv::TermCriteria*"]), _)]),
pub fn cv_ml_EM_setTermCriteria_const_TermCriteriaR(instance: *mut c_void, val: *const core::TermCriteria, ocvrs_return: *mut Result<()>);
// getWeights()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:895
// ("cv::ml::EM::getWeights", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_EM_getWeights_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getMeans()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:901
// ("cv::ml::EM::getMeans", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_EM_getMeans_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getCovs(std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:907
// ("cv::ml::EM::getCovs", vec![(pred!(const, ["covs"], ["std::vector<cv::Mat>*"]), _)]),
pub fn cv_ml_EM_getCovs_const_vectorLMatGR(instance: *const c_void, covs: *mut c_void, ocvrs_return: *mut Result<()>);
// predict(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:916
// ("cv::ml::EM::predict", vec![(pred!(const, ["samples", "results", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_ml_EM_predict_const_const__InputArrayR_const__OutputArrayR_int(instance: *const c_void, samples: *const c_void, results: *const c_void, flags: i32, ocvrs_return: *mut Result<f32>);
// cv::ml::EM::predict(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:916
// ("cv::ml::EM::predict", vec![(pred!(const, ["samples"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ml_EM_predict_const_const__InputArrayR(instance: *const c_void, samples: *const c_void, ocvrs_return: *mut Result<f32>);
// predict2(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:930
// ("cv::ml::EM::predict2", vec![(pred!(const, ["sample", "probs"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ml_EM_predict2_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, sample: *const c_void, probs: *const c_void, ocvrs_return: *mut Result<core::Vec2d>);
// trainEM(InputArray, OutputArray, OutputArray, OutputArray)(InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:960
// ("cv::ml::EM::trainEM", vec![(pred!(mut, ["samples", "logLikelihoods", "labels", "probs"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ml_EM_trainEM_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, samples: *const c_void, log_likelihoods: *const c_void, labels: *const c_void, probs: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::ml::EM::trainEM(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:960
// ("cv::ml::EM::trainEM", vec![(pred!(mut, ["samples"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ml_EM_trainEM_const__InputArrayR(instance: *mut c_void, samples: *const c_void, ocvrs_return: *mut Result<bool>);
// trainE(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray)(InputArray, InputArray, InputArray, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:992
// ("cv::ml::EM::trainE", vec![(pred!(mut, ["samples", "means0", "covs0", "weights0", "logLikelihoods", "labels", "probs"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ml_EM_trainE_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, samples: *const c_void, means0: *const c_void, covs0: *const c_void, weights0: *const c_void, log_likelihoods: *const c_void, labels: *const c_void, probs: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::ml::EM::trainE(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:992
// ("cv::ml::EM::trainE", vec![(pred!(mut, ["samples", "means0"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_ml_EM_trainE_const__InputArrayR_const__InputArrayR(instance: *mut c_void, samples: *const c_void, means0: *const c_void, ocvrs_return: *mut Result<bool>);
// trainM(InputArray, InputArray, OutputArray, OutputArray, OutputArray)(InputArray, InputArray, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1017
// ("cv::ml::EM::trainM", vec![(pred!(mut, ["samples", "probs0", "logLikelihoods", "labels", "probs"], ["const cv::_InputArray*", "const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ml_EM_trainM_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *mut c_void, samples: *const c_void, probs0: *const c_void, log_likelihoods: *const c_void, labels: *const c_void, probs: *const c_void, ocvrs_return: *mut Result<bool>);
// cv::ml::EM::trainM(InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1017
// ("cv::ml::EM::trainM", vec![(pred!(mut, ["samples", "probs0"], ["const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_ml_EM_trainM_const__InputArrayR_const__InputArrayR(instance: *mut c_void, samples: *const c_void, probs0: *const c_void, ocvrs_return: *mut Result<bool>);
// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1026
// ("cv::ml::EM::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_EM_create(ocvrs_return: *mut Result<*mut c_void>);
// load(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1037
// ("cv::ml::EM::load", vec![(pred!(mut, ["filepath", "nodeName"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_ml_EM_load_const_StringR_const_StringR(filepath: *const c_char, node_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::EM::load(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1037
// ("cv::ml::EM::load", vec![(pred!(mut, ["filepath"], ["const cv::String*"]), _)]),
pub fn cv_ml_EM_load_const_StringR(filepath: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::EM::to_Algorithm() generated
// ("cv::ml::EM::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_EM_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ml::EM::to_StatModel() generated
// ("cv::ml::EM::to_StatModel", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_EM_to_StatModel(instance: *mut c_void) -> *mut c_void;
// cv::ml::EM::delete() generated
// ("cv::ml::EM::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_EM_delete(instance: *mut c_void);
// getDefaultK()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:442
// ("cv::ml::KNearest::getDefaultK", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_KNearest_getDefaultK_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setDefaultK(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:444
// ("cv::ml::KNearest::setDefaultK", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_ml_KNearest_setDefaultK_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getIsClassifier()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:448
// ("cv::ml::KNearest::getIsClassifier", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_KNearest_getIsClassifier_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setIsClassifier(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:450
// ("cv::ml::KNearest::setIsClassifier", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_ml_KNearest_setIsClassifier_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// getEmax()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:454
// ("cv::ml::KNearest::getEmax", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_KNearest_getEmax_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setEmax(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:456
// ("cv::ml::KNearest::setEmax", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_ml_KNearest_setEmax_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getAlgorithmType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:460
// ("cv::ml::KNearest::getAlgorithmType", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_KNearest_getAlgorithmType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setAlgorithmType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:462
// ("cv::ml::KNearest::setAlgorithmType", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_ml_KNearest_setAlgorithmType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// findNearest(InputArray, int, OutputArray, OutputArray, OutputArray)(InputArray, Primitive, OutputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:490
// ("cv::ml::KNearest::findNearest", vec![(pred!(const, ["samples", "k", "results", "neighborResponses", "dist"], ["const cv::_InputArray*", "int", "const cv::_OutputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ml_KNearest_findNearest_const_const__InputArrayR_int_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, samples: *const c_void, k: i32, results: *const c_void, neighbor_responses: *const c_void, dist: *const c_void, ocvrs_return: *mut Result<f32>);
// cv::ml::KNearest::findNearest(InputArray, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:490
// ("cv::ml::KNearest::findNearest", vec![(pred!(const, ["samples", "k", "results"], ["const cv::_InputArray*", "int", "const cv::_OutputArray*"]), _)]),
pub fn cv_ml_KNearest_findNearest_const_const__InputArrayR_int_const__OutputArrayR(instance: *const c_void, samples: *const c_void, k: i32, results: *const c_void, ocvrs_return: *mut Result<f32>);
// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:507
// ("cv::ml::KNearest::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_KNearest_create(ocvrs_return: *mut Result<*mut c_void>);
// load(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:515
// ("cv::ml::KNearest::load", vec![(pred!(mut, ["filepath"], ["const cv::String*"]), _)]),
pub fn cv_ml_KNearest_load_const_StringR(filepath: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::KNearest::to_Algorithm() generated
// ("cv::ml::KNearest::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_KNearest_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ml::KNearest::to_StatModel() generated
// ("cv::ml::KNearest::to_StatModel", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_KNearest_to_StatModel(instance: *mut c_void) -> *mut c_void;
// cv::ml::KNearest::delete() generated
// ("cv::ml::KNearest::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_KNearest_delete(instance: *mut c_void);
// getLearningRate()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1631
// ("cv::ml::LogisticRegression::getLearningRate", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_LogisticRegression_getLearningRate_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setLearningRate(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1633
// ("cv::ml::LogisticRegression::setLearningRate", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ml_LogisticRegression_setLearningRate_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getIterations()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1637
// ("cv::ml::LogisticRegression::getIterations", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_LogisticRegression_getIterations_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setIterations(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1639
// ("cv::ml::LogisticRegression::setIterations", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_ml_LogisticRegression_setIterations_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getRegularization()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1643
// ("cv::ml::LogisticRegression::getRegularization", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_LogisticRegression_getRegularization_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setRegularization(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1645
// ("cv::ml::LogisticRegression::setRegularization", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_ml_LogisticRegression_setRegularization_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getTrainMethod()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1649
// ("cv::ml::LogisticRegression::getTrainMethod", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_LogisticRegression_getTrainMethod_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setTrainMethod(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1651
// ("cv::ml::LogisticRegression::setTrainMethod", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_ml_LogisticRegression_setTrainMethod_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getMiniBatchSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1657
// ("cv::ml::LogisticRegression::getMiniBatchSize", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_LogisticRegression_getMiniBatchSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMiniBatchSize(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1659
// ("cv::ml::LogisticRegression::setMiniBatchSize", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_ml_LogisticRegression_setMiniBatchSize_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getTermCriteria()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1663
// ("cv::ml::LogisticRegression::getTermCriteria", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_LogisticRegression_getTermCriteria_const(instance: *const c_void, ocvrs_return: *mut Result<core::TermCriteria>);
// setTermCriteria(TermCriteria)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1665
// ("cv::ml::LogisticRegression::setTermCriteria", vec![(pred!(mut, ["val"], ["cv::TermCriteria"]), _)]),
pub fn cv_ml_LogisticRegression_setTermCriteria_TermCriteria(instance: *mut c_void, val: *const core::TermCriteria, ocvrs_return: *mut Result<()>);
// predict(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1687
// ("cv::ml::LogisticRegression::predict", vec![(pred!(const, ["samples", "results", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_ml_LogisticRegression_predict_const_const__InputArrayR_const__OutputArrayR_int(instance: *const c_void, samples: *const c_void, results: *const c_void, flags: i32, ocvrs_return: *mut Result<f32>);
// cv::ml::LogisticRegression::predict(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1687
// ("cv::ml::LogisticRegression::predict", vec![(pred!(const, ["samples"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ml_LogisticRegression_predict_const_const__InputArrayR(instance: *const c_void, samples: *const c_void, ocvrs_return: *mut Result<f32>);
// get_learnt_thetas()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1694
// ("cv::ml::LogisticRegression::get_learnt_thetas", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_LogisticRegression_get_learnt_thetas_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1700
// ("cv::ml::LogisticRegression::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_LogisticRegression_create(ocvrs_return: *mut Result<*mut c_void>);
// load(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1711
// ("cv::ml::LogisticRegression::load", vec![(pred!(mut, ["filepath", "nodeName"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_ml_LogisticRegression_load_const_StringR_const_StringR(filepath: *const c_char, node_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::LogisticRegression::load(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1711
// ("cv::ml::LogisticRegression::load", vec![(pred!(mut, ["filepath"], ["const cv::String*"]), _)]),
pub fn cv_ml_LogisticRegression_load_const_StringR(filepath: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::LogisticRegression::to_Algorithm() generated
// ("cv::ml::LogisticRegression::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_LogisticRegression_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ml::LogisticRegression::to_StatModel() generated
// ("cv::ml::LogisticRegression::to_StatModel", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_LogisticRegression_to_StatModel(instance: *mut c_void) -> *mut c_void;
// cv::ml::LogisticRegression::delete() generated
// ("cv::ml::LogisticRegression::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_LogisticRegression_delete(instance: *mut c_void);
// predictProb(InputArray, OutputArray, OutputArray, int)(InputArray, OutputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:409
// ("cv::ml::NormalBayesClassifier::predictProb", vec![(pred!(const, ["inputs", "outputs", "outputProbs", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_ml_NormalBayesClassifier_predictProb_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int(instance: *const c_void, inputs: *const c_void, outputs: *const c_void, output_probs: *const c_void, flags: i32, ocvrs_return: *mut Result<f32>);
// cv::ml::NormalBayesClassifier::predictProb(InputArray, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:409
// ("cv::ml::NormalBayesClassifier::predictProb", vec![(pred!(const, ["inputs", "outputs", "outputProbs"], ["const cv::_InputArray*", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ml_NormalBayesClassifier_predictProb_const_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, inputs: *const c_void, outputs: *const c_void, output_probs: *const c_void, ocvrs_return: *mut Result<f32>);
// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:414
// ("cv::ml::NormalBayesClassifier::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_NormalBayesClassifier_create(ocvrs_return: *mut Result<*mut c_void>);
// load(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:425
// ("cv::ml::NormalBayesClassifier::load", vec![(pred!(mut, ["filepath", "nodeName"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_ml_NormalBayesClassifier_load_const_StringR_const_StringR(filepath: *const c_char, node_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::NormalBayesClassifier::load(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:425
// ("cv::ml::NormalBayesClassifier::load", vec![(pred!(mut, ["filepath"], ["const cv::String*"]), _)]),
pub fn cv_ml_NormalBayesClassifier_load_const_StringR(filepath: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::NormalBayesClassifier::to_Algorithm() generated
// ("cv::ml::NormalBayesClassifier::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_NormalBayesClassifier_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ml::NormalBayesClassifier::to_StatModel() generated
// ("cv::ml::NormalBayesClassifier::to_StatModel", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_NormalBayesClassifier_to_StatModel(instance: *mut c_void) -> *mut c_void;
// cv::ml::NormalBayesClassifier::delete() generated
// ("cv::ml::NormalBayesClassifier::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_NormalBayesClassifier_delete(instance: *mut c_void);
// ParamGrid()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:111
// ("cv::ml::ParamGrid::ParamGrid", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_ParamGrid_ParamGrid(ocvrs_return: *mut Result<*mut c_void>);
// ParamGrid(double, double, double)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:113
// ("cv::ml::ParamGrid::ParamGrid", vec![(pred!(mut, ["_minVal", "_maxVal", "_logStep"], ["double", "double", "double"]), _)]),
pub fn cv_ml_ParamGrid_ParamGrid_double_double_double(_min_val: f64, _max_val: f64, _log_step: f64, ocvrs_return: *mut Result<*mut c_void>);
// create(double, double, double)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:133
// ("cv::ml::ParamGrid::create", vec![(pred!(mut, ["minVal", "maxVal", "logstep"], ["double", "double", "double"]), _)]),
pub fn cv_ml_ParamGrid_create_double_double_double(min_val: f64, max_val: f64, logstep: f64, ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::ParamGrid::create() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:133
// ("cv::ml::ParamGrid::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_ParamGrid_create(ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::ParamGrid::minVal() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:115
// ("cv::ml::ParamGrid::minVal", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_ParamGrid_propMinVal_const(instance: *const c_void) -> f64;
// cv::ml::ParamGrid::setMinVal(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:115
// ("cv::ml::ParamGrid::setMinVal", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_ml_ParamGrid_propMinVal_const_double(instance: *mut c_void, val: f64);
// cv::ml::ParamGrid::maxVal() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:116
// ("cv::ml::ParamGrid::maxVal", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_ParamGrid_propMaxVal_const(instance: *const c_void) -> f64;
// cv::ml::ParamGrid::setMaxVal(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:116
// ("cv::ml::ParamGrid::setMaxVal", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_ml_ParamGrid_propMaxVal_const_double(instance: *mut c_void, val: f64);
// cv::ml::ParamGrid::logStep() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:125
// ("cv::ml::ParamGrid::logStep", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_ParamGrid_propLogStep_const(instance: *const c_void) -> f64;
// cv::ml::ParamGrid::setLogStep(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:125
// ("cv::ml::ParamGrid::setLogStep", vec![(pred!(mut, ["val"], ["const double"]), _)]),
pub fn cv_ml_ParamGrid_propLogStep_const_double(instance: *mut c_void, val: f64);
// cv::ml::ParamGrid::delete() generated
// ("cv::ml::ParamGrid::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_ParamGrid_delete(instance: *mut c_void);
// getCalculateVarImportance()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1254
// ("cv::ml::RTrees::getCalculateVarImportance", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_RTrees_getCalculateVarImportance_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// setCalculateVarImportance(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1256
// ("cv::ml::RTrees::setCalculateVarImportance", vec![(pred!(mut, ["val"], ["bool"]), _)]),
pub fn cv_ml_RTrees_setCalculateVarImportance_bool(instance: *mut c_void, val: bool, ocvrs_return: *mut Result<()>);
// getActiveVarCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1263
// ("cv::ml::RTrees::getActiveVarCount", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_RTrees_getActiveVarCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setActiveVarCount(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1265
// ("cv::ml::RTrees::setActiveVarCount", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_ml_RTrees_setActiveVarCount_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getTermCriteria()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1275
// ("cv::ml::RTrees::getTermCriteria", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_RTrees_getTermCriteria_const(instance: *const c_void, ocvrs_return: *mut Result<core::TermCriteria>);
// setTermCriteria(const TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1277
// ("cv::ml::RTrees::setTermCriteria", vec![(pred!(mut, ["val"], ["const cv::TermCriteria*"]), _)]),
pub fn cv_ml_RTrees_setTermCriteria_const_TermCriteriaR(instance: *mut c_void, val: *const core::TermCriteria, ocvrs_return: *mut Result<()>);
// getVarImportance()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1284
// ("cv::ml::RTrees::getVarImportance", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_RTrees_getVarImportance_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getVotes(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1295
// ("cv::ml::RTrees::getVotes", vec![(pred!(const, ["samples", "results", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_ml_RTrees_getVotes_const_const__InputArrayR_const__OutputArrayR_int(instance: *const c_void, samples: *const c_void, results: *const c_void, flags: i32, ocvrs_return: *mut Result<()>);
// getOOBError()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1301
// ("cv::ml::RTrees::getOOBError", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_RTrees_getOOBError_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1310
// ("cv::ml::RTrees::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_RTrees_create(ocvrs_return: *mut Result<*mut c_void>);
// load(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1321
// ("cv::ml::RTrees::load", vec![(pred!(mut, ["filepath", "nodeName"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_ml_RTrees_load_const_StringR_const_StringR(filepath: *const c_char, node_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::RTrees::load(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1321
// ("cv::ml::RTrees::load", vec![(pred!(mut, ["filepath"], ["const cv::String*"]), _)]),
pub fn cv_ml_RTrees_load_const_StringR(filepath: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::RTrees::to_Algorithm() generated
// ("cv::ml::RTrees::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_RTrees_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ml::RTrees::to_DTrees() generated
// ("cv::ml::RTrees::to_DTrees", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_RTrees_to_DTrees(instance: *mut c_void) -> *mut c_void;
// cv::ml::RTrees::to_StatModel() generated
// ("cv::ml::RTrees::to_StatModel", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_RTrees_to_StatModel(instance: *mut c_void) -> *mut c_void;
// cv::ml::RTrees::delete() generated
// ("cv::ml::RTrees::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_RTrees_delete(instance: *mut c_void);
// getType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:540
// ("cv::ml::SVM::getType", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_SVM_getType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:542
// ("cv::ml::SVM::setType", vec![(pred!(mut, ["val"], ["int"]), _)]),
pub fn cv_ml_SVM_setType_int(instance: *mut c_void, val: i32, ocvrs_return: *mut Result<()>);
// getGamma()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:547
// ("cv::ml::SVM::getGamma", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_SVM_getGamma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setGamma(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:549
// ("cv::ml::SVM::setGamma", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ml_SVM_setGamma_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getCoef0()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:554
// ("cv::ml::SVM::getCoef0", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_SVM_getCoef0_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setCoef0(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:556
// ("cv::ml::SVM::setCoef0", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ml_SVM_setCoef0_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getDegree()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:561
// ("cv::ml::SVM::getDegree", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_SVM_getDegree_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setDegree(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:563
// ("cv::ml::SVM::setDegree", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ml_SVM_setDegree_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getC()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:568
// ("cv::ml::SVM::getC", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_SVM_getC_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setC(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:570
// ("cv::ml::SVM::setC", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ml_SVM_setC_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getNu()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:575
// ("cv::ml::SVM::getNu", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_SVM_getNu_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setNu(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:577
// ("cv::ml::SVM::setNu", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ml_SVM_setNu_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getP()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:582
// ("cv::ml::SVM::getP", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_SVM_getP_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
// setP(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:584
// ("cv::ml::SVM::setP", vec![(pred!(mut, ["val"], ["double"]), _)]),
pub fn cv_ml_SVM_setP_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result<()>);
// getClassWeights()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:592
// ("cv::ml::SVM::getClassWeights", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_SVM_getClassWeights_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setClassWeights(const cv::Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:594
// ("cv::ml::SVM::setClassWeights", vec![(pred!(mut, ["val"], ["const cv::Mat*"]), _)]),
pub fn cv_ml_SVM_setClassWeights_const_MatR(instance: *mut c_void, val: *const c_void, ocvrs_return: *mut Result<()>);
// getTermCriteria()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:601
// ("cv::ml::SVM::getTermCriteria", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_SVM_getTermCriteria_const(instance: *const c_void, ocvrs_return: *mut Result<core::TermCriteria>);
// setTermCriteria(const cv::TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:603
// ("cv::ml::SVM::setTermCriteria", vec![(pred!(mut, ["val"], ["const cv::TermCriteria*"]), _)]),
pub fn cv_ml_SVM_setTermCriteria_const_TermCriteriaR(instance: *mut c_void, val: *const core::TermCriteria, ocvrs_return: *mut Result<()>);
// getKernelType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:607
// ("cv::ml::SVM::getKernelType", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_SVM_getKernelType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setKernel(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:611
// ("cv::ml::SVM::setKernel", vec![(pred!(mut, ["kernelType"], ["int"]), _)]),
pub fn cv_ml_SVM_setKernel_int(instance: *mut c_void, kernel_type: i32, ocvrs_return: *mut Result<()>);
// setCustomKernel(const Ptr<Kernel> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:615
// ("cv::ml::SVM::setCustomKernel", vec![(pred!(mut, ["_kernel"], ["const cv::Ptr<cv::ml::SVM::Kernel>*"]), _)]),
pub fn cv_ml_SVM_setCustomKernel_const_PtrLKernelGR(instance: *mut c_void, _kernel: *const c_void, ocvrs_return: *mut Result<()>);
// trainAuto(const Ptr<TrainData> &, int, ParamGrid, ParamGrid, ParamGrid, ParamGrid, ParamGrid, ParamGrid, bool)(CppPassByVoidPtr, Primitive, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:712
// ("cv::ml::SVM::trainAuto", vec![(pred!(mut, ["data", "kFold", "Cgrid", "gammaGrid", "pGrid", "nuGrid", "coeffGrid", "degreeGrid", "balanced"], ["const cv::Ptr<cv::ml::TrainData>*", "int", "cv::ml::ParamGrid", "cv::ml::ParamGrid", "cv::ml::ParamGrid", "cv::ml::ParamGrid", "cv::ml::ParamGrid", "cv::ml::ParamGrid", "bool"]), _)]),
pub fn cv_ml_SVM_trainAuto_const_PtrLTrainDataGR_int_ParamGrid_ParamGrid_ParamGrid_ParamGrid_ParamGrid_ParamGrid_bool(instance: *mut c_void, data: *const c_void, k_fold: i32, cgrid: *mut c_void, gamma_grid: *mut c_void, p_grid: *mut c_void, nu_grid: *mut c_void, coeff_grid: *mut c_void, degree_grid: *mut c_void, balanced: bool, ocvrs_return: *mut Result<bool>);
// cv::ml::SVM::trainAuto(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:712
// ("cv::ml::SVM::trainAuto", vec![(pred!(mut, ["data"], ["const cv::Ptr<cv::ml::TrainData>*"]), _)]),
pub fn cv_ml_SVM_trainAuto_const_PtrLTrainDataGR(instance: *mut c_void, data: *const c_void, ocvrs_return: *mut Result<bool>);
// trainAuto(InputArray, int, InputArray, int, Ptr<ParamGrid>, Ptr<ParamGrid>, Ptr<ParamGrid>, Ptr<ParamGrid>, Ptr<ParamGrid>, Ptr<ParamGrid>, bool)(InputArray, Primitive, InputArray, Primitive, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:749
// ("cv::ml::SVM::trainAuto", vec![(pred!(mut, ["samples", "layout", "responses", "kFold", "Cgrid", "gammaGrid", "pGrid", "nuGrid", "coeffGrid", "degreeGrid", "balanced"], ["const cv::_InputArray*", "int", "const cv::_InputArray*", "int", "cv::Ptr<cv::ml::ParamGrid>", "cv::Ptr<cv::ml::ParamGrid>", "cv::Ptr<cv::ml::ParamGrid>", "cv::Ptr<cv::ml::ParamGrid>", "cv::Ptr<cv::ml::ParamGrid>", "cv::Ptr<cv::ml::ParamGrid>", "bool"]), _)]),
pub fn cv_ml_SVM_trainAuto_const__InputArrayR_int_const__InputArrayR_int_PtrLParamGridG_PtrLParamGridG_PtrLParamGridG_PtrLParamGridG_PtrLParamGridG_PtrLParamGridG_bool(instance: *mut c_void, samples: *const c_void, layout: i32, responses: *const c_void, k_fold: i32, cgrid: *mut c_void, gamma_grid: *mut c_void, p_grid: *mut c_void, nu_grid: *mut c_void, coeff_grid: *mut c_void, degree_grid: *mut c_void, balanced: bool, ocvrs_return: *mut Result<bool>);
// cv::ml::SVM::trainAuto(InputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:749
// ("cv::ml::SVM::trainAuto", vec![(pred!(mut, ["samples", "layout", "responses"], ["const cv::_InputArray*", "int", "const cv::_InputArray*"]), _)]),
pub fn cv_ml_SVM_trainAuto_const__InputArrayR_int_const__InputArrayR(instance: *mut c_void, samples: *const c_void, layout: i32, responses: *const c_void, ocvrs_return: *mut Result<bool>);
// getSupportVectors()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:766
// ("cv::ml::SVM::getSupportVectors", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_SVM_getSupportVectors_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getUncompressedSupportVectors()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:774
// ("cv::ml::SVM::getUncompressedSupportVectors", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_SVM_getUncompressedSupportVectors_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getDecisionFunction(int, OutputArray, OutputArray)(Primitive, OutputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:791
// ("cv::ml::SVM::getDecisionFunction", vec![(pred!(const, ["i", "alpha", "svidx"], ["int", "const cv::_OutputArray*", "const cv::_OutputArray*"]), _)]),
pub fn cv_ml_SVM_getDecisionFunction_const_int_const__OutputArrayR_const__OutputArrayR(instance: *const c_void, i: i32, alpha: *const c_void, svidx: *const c_void, ocvrs_return: *mut Result<f64>);
// getDefaultGrid(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:801
// ("cv::ml::SVM::getDefaultGrid", vec![(pred!(mut, ["param_id"], ["int"]), _)]),
pub fn cv_ml_SVM_getDefaultGrid_int(param_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// getDefaultGridPtr(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:811
// ("cv::ml::SVM::getDefaultGridPtr", vec![(pred!(mut, ["param_id"], ["int"]), _)]),
pub fn cv_ml_SVM_getDefaultGridPtr_int(param_id: i32, ocvrs_return: *mut Result<*mut c_void>);
// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:816
// ("cv::ml::SVM::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_SVM_create(ocvrs_return: *mut Result<*mut c_void>);
// load(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:825
// ("cv::ml::SVM::load", vec![(pred!(mut, ["filepath"], ["const cv::String*"]), _)]),
pub fn cv_ml_SVM_load_const_StringR(filepath: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::SVM::to_Algorithm() generated
// ("cv::ml::SVM::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_SVM_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ml::SVM::to_StatModel() generated
// ("cv::ml::SVM::to_StatModel", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_SVM_to_StatModel(instance: *mut c_void) -> *mut c_void;
// cv::ml::SVM::delete() generated
// ("cv::ml::SVM::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_SVM_delete(instance: *mut c_void);
// getType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:533
// ("cv::ml::SVM::Kernel::getType", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_SVM_Kernel_getType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// calc(int, int, const float *, const float *, float *)(Primitive, Primitive, Indirect, Indirect, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:534
// ("cv::ml::SVM::Kernel::calc", vec![(pred!(mut, ["vcount", "n", "vecs", "another", "results"], ["int", "int", "const float*", "const float*", "float*"]), _)]),
pub fn cv_ml_SVM_Kernel_calc_int_int_const_floatX_const_floatX_floatX(instance: *mut c_void, vcount: i32, n: i32, vecs: *const f32, another: *const f32, results: *mut f32, ocvrs_return: *mut Result<()>);
// cv::ml::SVM::Kernel::to_Algorithm() generated
// ("cv::ml::SVM::Kernel::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_SVM_Kernel_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ml::SVM::Kernel::delete() generated
// ("cv::ml::SVM::Kernel::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_SVM_Kernel_delete(instance: *mut c_void);
// getWeights()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1814
// ("cv::ml::SVMSGD::getWeights", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_SVMSGD_getWeights(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
// getShift()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1819
// ("cv::ml::SVMSGD::getShift", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_SVMSGD_getShift(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1825
// ("cv::ml::SVMSGD::create", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_SVMSGD_create(ocvrs_return: *mut Result<*mut c_void>);
// load(const String &, const String &)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1836
// ("cv::ml::SVMSGD::load", vec![(pred!(mut, ["filepath", "nodeName"], ["const cv::String*", "const cv::String*"]), _)]),
pub fn cv_ml_SVMSGD_load_const_StringR_const_StringR(filepath: *const c_char, node_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::SVMSGD::load(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1836
// ("cv::ml::SVMSGD::load", vec![(pred!(mut, ["filepath"], ["const cv::String*"]), _)]),
pub fn cv_ml_SVMSGD_load_const_StringR(filepath: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
// setOptimalParameters(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1842
// ("cv::ml::SVMSGD::setOptimalParameters", vec![(pred!(mut, ["svmsgdType", "marginType"], ["int", "int"]), _)]),
pub fn cv_ml_SVMSGD_setOptimalParameters_int_int(instance: *mut c_void, svmsgd_type: i32, margin_type: i32, ocvrs_return: *mut Result<()>);
// cv::ml::SVMSGD::setOptimalParameters() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1842
// ("cv::ml::SVMSGD::setOptimalParameters", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_SVMSGD_setOptimalParameters(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// getSvmsgdType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1846
// ("cv::ml::SVMSGD::getSvmsgdType", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_SVMSGD_getSvmsgdType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setSvmsgdType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1848
// ("cv::ml::SVMSGD::setSvmsgdType", vec![(pred!(mut, ["svmsgdType"], ["int"]), _)]),
pub fn cv_ml_SVMSGD_setSvmsgdType_int(instance: *mut c_void, svmsgd_type: i32, ocvrs_return: *mut Result<()>);
// getMarginType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1852
// ("cv::ml::SVMSGD::getMarginType", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_SVMSGD_getMarginType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// setMarginType(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1854
// ("cv::ml::SVMSGD::setMarginType", vec![(pred!(mut, ["marginType"], ["int"]), _)]),
pub fn cv_ml_SVMSGD_setMarginType_int(instance: *mut c_void, margin_type: i32, ocvrs_return: *mut Result<()>);
// getMarginRegularization()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1858
// ("cv::ml::SVMSGD::getMarginRegularization", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_SVMSGD_getMarginRegularization_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setMarginRegularization(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1860
// ("cv::ml::SVMSGD::setMarginRegularization", vec![(pred!(mut, ["marginRegularization"], ["float"]), _)]),
pub fn cv_ml_SVMSGD_setMarginRegularization_float(instance: *mut c_void, margin_regularization: f32, ocvrs_return: *mut Result<()>);
// getInitialStepSize()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1864
// ("cv::ml::SVMSGD::getInitialStepSize", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_SVMSGD_getInitialStepSize_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setInitialStepSize(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1866
// ("cv::ml::SVMSGD::setInitialStepSize", vec![(pred!(mut, ["InitialStepSize"], ["float"]), _)]),
pub fn cv_ml_SVMSGD_setInitialStepSize_float(instance: *mut c_void, initial_step_size: f32, ocvrs_return: *mut Result<()>);
// getStepDecreasingPower()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1870
// ("cv::ml::SVMSGD::getStepDecreasingPower", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_SVMSGD_getStepDecreasingPower_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
// setStepDecreasingPower(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1872
// ("cv::ml::SVMSGD::setStepDecreasingPower", vec![(pred!(mut, ["stepDecreasingPower"], ["float"]), _)]),
pub fn cv_ml_SVMSGD_setStepDecreasingPower_float(instance: *mut c_void, step_decreasing_power: f32, ocvrs_return: *mut Result<()>);
// getTermCriteria()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1878
// ("cv::ml::SVMSGD::getTermCriteria", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_SVMSGD_getTermCriteria_const(instance: *const c_void, ocvrs_return: *mut Result<core::TermCriteria>);
// setTermCriteria(const cv::TermCriteria &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:1880
// ("cv::ml::SVMSGD::setTermCriteria", vec![(pred!(mut, ["val"], ["const cv::TermCriteria*"]), _)]),
pub fn cv_ml_SVMSGD_setTermCriteria_const_TermCriteriaR(instance: *mut c_void, val: *const core::TermCriteria, ocvrs_return: *mut Result<()>);
// cv::ml::SVMSGD::to_Algorithm() generated
// ("cv::ml::SVMSGD::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_SVMSGD_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ml::SVMSGD::to_StatModel() generated
// ("cv::ml::SVMSGD::to_StatModel", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_SVMSGD_to_StatModel(instance: *mut c_void) -> *mut c_void;
// cv::ml::SVMSGD::delete() generated
// ("cv::ml::SVMSGD::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_SVMSGD_delete(instance: *mut c_void);
// getVarCount()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:330
// ("cv::ml::StatModel::getVarCount", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_StatModel_getVarCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// empty()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:332
// ("cv::ml::StatModel::empty", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_StatModel_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isTrained()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:335
// ("cv::ml::StatModel::isTrained", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_StatModel_isTrained_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// isClassifier()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:337
// ("cv::ml::StatModel::isClassifier", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_StatModel_isClassifier_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
// train(const Ptr<TrainData> &, int)(CppPassByVoidPtr, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:346
// ("cv::ml::StatModel::train", vec![(pred!(mut, ["trainData", "flags"], ["const cv::Ptr<cv::ml::TrainData>*", "int"]), _)]),
pub fn cv_ml_StatModel_train_const_PtrLTrainDataGR_int(instance: *mut c_void, train_data: *const c_void, flags: i32, ocvrs_return: *mut Result<bool>);
// cv::ml::StatModel::train(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:346
// ("cv::ml::StatModel::train", vec![(pred!(mut, ["trainData"], ["const cv::Ptr<cv::ml::TrainData>*"]), _)]),
pub fn cv_ml_StatModel_train_const_PtrLTrainDataGR(instance: *mut c_void, train_data: *const c_void, ocvrs_return: *mut Result<bool>);
// train(InputArray, int, InputArray)(InputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:354
// ("cv::ml::StatModel::train", vec![(pred!(mut, ["samples", "layout", "responses"], ["const cv::_InputArray*", "int", "const cv::_InputArray*"]), _)]),
pub fn cv_ml_StatModel_train_const__InputArrayR_int_const__InputArrayR(instance: *mut c_void, samples: *const c_void, layout: i32, responses: *const c_void, ocvrs_return: *mut Result<bool>);
// calcError(const Ptr<TrainData> &, bool, OutputArray)(CppPassByVoidPtr, Primitive, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:369
// ("cv::ml::StatModel::calcError", vec![(pred!(const, ["data", "test", "resp"], ["const cv::Ptr<cv::ml::TrainData>*", "bool", "const cv::_OutputArray*"]), _)]),
pub fn cv_ml_StatModel_calcError_const_const_PtrLTrainDataGR_bool_const__OutputArrayR(instance: *const c_void, data: *const c_void, test: bool, resp: *const c_void, ocvrs_return: *mut Result<f32>);
// predict(InputArray, OutputArray, int)(InputArray, OutputArray, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:377
// ("cv::ml::StatModel::predict", vec![(pred!(const, ["samples", "results", "flags"], ["const cv::_InputArray*", "const cv::_OutputArray*", "int"]), _)]),
pub fn cv_ml_StatModel_predict_const_const__InputArrayR_const__OutputArrayR_int(instance: *const c_void, samples: *const c_void, results: *const c_void, flags: i32, ocvrs_return: *mut Result<f32>);
// cv::ml::StatModel::predict(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:377
// ("cv::ml::StatModel::predict", vec![(pred!(const, ["samples"], ["const cv::_InputArray*"]), _)]),
pub fn cv_ml_StatModel_predict_const_const__InputArrayR(instance: *const c_void, samples: *const c_void, ocvrs_return: *mut Result<f32>);
// cv::ml::StatModel::to_ANN_MLP() generated
// ("cv::ml::StatModel::to_ANN_MLP", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_StatModel_to_ANN_MLP(instance: *mut c_void) -> *mut c_void;
// cv::ml::StatModel::to_ANN_MLP_ANNEAL() generated
// ("cv::ml::StatModel::to_ANN_MLP_ANNEAL", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_StatModel_to_ANN_MLP_ANNEAL(instance: *mut c_void) -> *mut c_void;
// cv::ml::StatModel::to_Boost() generated
// ("cv::ml::StatModel::to_Boost", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_StatModel_to_Boost(instance: *mut c_void) -> *mut c_void;
// cv::ml::StatModel::to_DTrees() generated
// ("cv::ml::StatModel::to_DTrees", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_StatModel_to_DTrees(instance: *mut c_void) -> *mut c_void;
// cv::ml::StatModel::to_EM() generated
// ("cv::ml::StatModel::to_EM", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_StatModel_to_EM(instance: *mut c_void) -> *mut c_void;
// cv::ml::StatModel::to_KNearest() generated
// ("cv::ml::StatModel::to_KNearest", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_StatModel_to_KNearest(instance: *mut c_void) -> *mut c_void;
// cv::ml::StatModel::to_LogisticRegression() generated
// ("cv::ml::StatModel::to_LogisticRegression", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_StatModel_to_LogisticRegression(instance: *mut c_void) -> *mut c_void;
// cv::ml::StatModel::to_NormalBayesClassifier() generated
// ("cv::ml::StatModel::to_NormalBayesClassifier", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_StatModel_to_NormalBayesClassifier(instance: *mut c_void) -> *mut c_void;
// cv::ml::StatModel::to_RTrees() generated
// ("cv::ml::StatModel::to_RTrees", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_StatModel_to_RTrees(instance: *mut c_void) -> *mut c_void;
// cv::ml::StatModel::to_SVM() generated
// ("cv::ml::StatModel::to_SVM", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_StatModel_to_SVM(instance: *mut c_void) -> *mut c_void;
// cv::ml::StatModel::to_SVMSGD() generated
// ("cv::ml::StatModel::to_SVMSGD", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_StatModel_to_SVMSGD(instance: *mut c_void) -> *mut c_void;
// cv::ml::StatModel::to_Algorithm() generated
// ("cv::ml::StatModel::to_Algorithm", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_StatModel_to_Algorithm(instance: *mut c_void) -> *mut c_void;
// cv::ml::StatModel::delete() generated
// ("cv::ml::StatModel::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_StatModel_delete(instance: *mut c_void);
// missingValue()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:148
// ("cv::ml::TrainData::missingValue", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_TrainData_missingValue(ocvrs_return: *mut Result<f32>);
// getLayout()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:151
// ("cv::ml::TrainData::getLayout", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getLayout_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getNTrainSamples()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:152
// ("cv::ml::TrainData::getNTrainSamples", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getNTrainSamples_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getNTestSamples()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:153
// ("cv::ml::TrainData::getNTestSamples", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getNTestSamples_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getNSamples()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:154
// ("cv::ml::TrainData::getNSamples", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getNSamples_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getNVars()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:155
// ("cv::ml::TrainData::getNVars", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getNVars_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getNAllVars()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:156
// ("cv::ml::TrainData::getNAllVars", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getNAllVars_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getSample(InputArray, int, float *)(InputArray, Primitive, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:158
// ("cv::ml::TrainData::getSample", vec![(pred!(const, ["varIdx", "sidx", "buf"], ["const cv::_InputArray*", "int", "float*"]), _)]),
pub fn cv_ml_TrainData_getSample_const_const__InputArrayR_int_floatX(instance: *const c_void, var_idx: *const c_void, sidx: i32, buf: *mut f32, ocvrs_return: *mut Result<()>);
// getSamples()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:159
// ("cv::ml::TrainData::getSamples", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getSamples_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getMissing()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:160
// ("cv::ml::TrainData::getMissing", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getMissing_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getTrainSamples(int, bool, bool)(Primitive, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:174
// ("cv::ml::TrainData::getTrainSamples", vec![(pred!(const, ["layout", "compressSamples", "compressVars"], ["int", "bool", "bool"]), _)]),
pub fn cv_ml_TrainData_getTrainSamples_const_int_bool_bool(instance: *const c_void, layout: i32, compress_samples: bool, compress_vars: bool, ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::TrainData::getTrainSamples() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:174
// ("cv::ml::TrainData::getTrainSamples", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getTrainSamples_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getTrainResponses()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:183
// ("cv::ml::TrainData::getTrainResponses", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getTrainResponses_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getTrainNormCatResponses()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:191
// ("cv::ml::TrainData::getTrainNormCatResponses", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getTrainNormCatResponses_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getTestResponses()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:192
// ("cv::ml::TrainData::getTestResponses", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getTestResponses_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getTestNormCatResponses()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:193
// ("cv::ml::TrainData::getTestNormCatResponses", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getTestNormCatResponses_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getResponses()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:194
// ("cv::ml::TrainData::getResponses", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getResponses_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getNormCatResponses()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:195
// ("cv::ml::TrainData::getNormCatResponses", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getNormCatResponses_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getSampleWeights()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:196
// ("cv::ml::TrainData::getSampleWeights", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getSampleWeights_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getTrainSampleWeights()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:197
// ("cv::ml::TrainData::getTrainSampleWeights", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getTrainSampleWeights_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getTestSampleWeights()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:198
// ("cv::ml::TrainData::getTestSampleWeights", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getTestSampleWeights_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getVarIdx()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:199
// ("cv::ml::TrainData::getVarIdx", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getVarIdx_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getVarType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:200
// ("cv::ml::TrainData::getVarType", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getVarType_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getVarSymbolFlags()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:201
// ("cv::ml::TrainData::getVarSymbolFlags", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getVarSymbolFlags_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getResponseType()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:202
// ("cv::ml::TrainData::getResponseType", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getResponseType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
// getTrainSampleIdx()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:203
// ("cv::ml::TrainData::getTrainSampleIdx", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getTrainSampleIdx_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getTestSampleIdx()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:204
// ("cv::ml::TrainData::getTestSampleIdx", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getTestSampleIdx_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getValues(int, InputArray, float *)(Primitive, InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:205
// ("cv::ml::TrainData::getValues", vec![(pred!(const, ["vi", "sidx", "values"], ["int", "const cv::_InputArray*", "float*"]), _)]),
pub fn cv_ml_TrainData_getValues_const_int_const__InputArrayR_floatX(instance: *const c_void, vi: i32, sidx: *const c_void, values: *mut f32, ocvrs_return: *mut Result<()>);
// getNormCatValues(int, InputArray, int *)(Primitive, InputArray, Indirect) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:206
// ("cv::ml::TrainData::getNormCatValues", vec![(pred!(const, ["vi", "sidx", "values"], ["int", "const cv::_InputArray*", "int*"]), _)]),
pub fn cv_ml_TrainData_getNormCatValues_const_int_const__InputArrayR_intX(instance: *const c_void, vi: i32, sidx: *const c_void, values: *mut i32, ocvrs_return: *mut Result<()>);
// getDefaultSubstValues()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:207
// ("cv::ml::TrainData::getDefaultSubstValues", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getDefaultSubstValues_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getCatCount(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:209
// ("cv::ml::TrainData::getCatCount", vec![(pred!(const, ["vi"], ["int"]), _)]),
pub fn cv_ml_TrainData_getCatCount_const_int(instance: *const c_void, vi: i32, ocvrs_return: *mut Result<i32>);
// getClassLabels()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:215
// ("cv::ml::TrainData::getClassLabels", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getClassLabels_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getCatOfs()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:217
// ("cv::ml::TrainData::getCatOfs", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getCatOfs_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getCatMap()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:218
// ("cv::ml::TrainData::getCatMap", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getCatMap_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// setTrainTestSplit(int, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:223
// ("cv::ml::TrainData::setTrainTestSplit", vec![(pred!(mut, ["count", "shuffle"], ["int", "bool"]), _)]),
pub fn cv_ml_TrainData_setTrainTestSplit_int_bool(instance: *mut c_void, count: i32, shuffle: bool, ocvrs_return: *mut Result<()>);
// cv::ml::TrainData::setTrainTestSplit(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:223
// ("cv::ml::TrainData::setTrainTestSplit", vec![(pred!(mut, ["count"], ["int"]), _)]),
pub fn cv_ml_TrainData_setTrainTestSplit_int(instance: *mut c_void, count: i32, ocvrs_return: *mut Result<()>);
// setTrainTestSplitRatio(double, bool)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:233
// ("cv::ml::TrainData::setTrainTestSplitRatio", vec![(pred!(mut, ["ratio", "shuffle"], ["double", "bool"]), _)]),
pub fn cv_ml_TrainData_setTrainTestSplitRatio_double_bool(instance: *mut c_void, ratio: f64, shuffle: bool, ocvrs_return: *mut Result<()>);
// cv::ml::TrainData::setTrainTestSplitRatio(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:233
// ("cv::ml::TrainData::setTrainTestSplitRatio", vec![(pred!(mut, ["ratio"], ["double"]), _)]),
pub fn cv_ml_TrainData_setTrainTestSplitRatio_double(instance: *mut c_void, ratio: f64, ocvrs_return: *mut Result<()>);
// shuffleTrainTest()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:234
// ("cv::ml::TrainData::shuffleTrainTest", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_TrainData_shuffleTrainTest(instance: *mut c_void, ocvrs_return: *mut Result<()>);
// getTestSamples()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:237
// ("cv::ml::TrainData::getTestSamples", vec![(pred!(const, [], []), _)]),
pub fn cv_ml_TrainData_getTestSamples_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getNames(std::vector<String> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:240
// ("cv::ml::TrainData::getNames", vec![(pred!(const, ["names"], ["std::vector<cv::String>*"]), _)]),
pub fn cv_ml_TrainData_getNames_const_vectorLStringGR(instance: *const c_void, names: *mut c_void, ocvrs_return: *mut Result<()>);
// getSubVector(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:246
// ("cv::ml::TrainData::getSubVector", vec![(pred!(mut, ["vec", "idx"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
pub fn cv_ml_TrainData_getSubVector_const_MatR_const_MatR(vec: *const c_void, idx: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// getSubMatrix(const Mat &, const Mat &, int)(TraitClass, TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:253
// ("cv::ml::TrainData::getSubMatrix", vec![(pred!(mut, ["matrix", "idx", "layout"], ["const cv::Mat*", "const cv::Mat*", "int"]), _)]),
pub fn cv_ml_TrainData_getSubMatrix_const_MatR_const_MatR_int(matrix: *const c_void, idx: *const c_void, layout: i32, ocvrs_return: *mut Result<*mut c_void>);
// loadFromCSV(const String &, int, int, int, const String &, char, char)(InString, Primitive, Primitive, Primitive, InString, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:284
// ("cv::ml::TrainData::loadFromCSV", vec![(pred!(mut, ["filename", "headerLineCount", "responseStartIdx", "responseEndIdx", "varTypeSpec", "delimiter", "missch"], ["const cv::String*", "int", "int", "int", "const cv::String*", "char", "char"]), _)]),
pub fn cv_ml_TrainData_loadFromCSV_const_StringR_int_int_int_const_StringR_char_char(filename: *const c_char, header_line_count: i32, response_start_idx: i32, response_end_idx: i32, var_type_spec: *const c_char, delimiter: c_char, missch: c_char, ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::TrainData::loadFromCSV(InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:284
// ("cv::ml::TrainData::loadFromCSV", vec![(pred!(mut, ["filename", "headerLineCount"], ["const cv::String*", "int"]), _)]),
pub fn cv_ml_TrainData_loadFromCSV_const_StringR_int(filename: *const c_char, header_line_count: i32, ocvrs_return: *mut Result<*mut c_void>);
// create(InputArray, int, InputArray, InputArray, InputArray, InputArray, InputArray)(InputArray, Primitive, InputArray, InputArray, InputArray, InputArray, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:311
// ("cv::ml::TrainData::create", vec![(pred!(mut, ["samples", "layout", "responses", "varIdx", "sampleIdx", "sampleWeights", "varType"], ["const cv::_InputArray*", "int", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*", "const cv::_InputArray*"]), _)]),
pub fn cv_ml_TrainData_create_const__InputArrayR_int_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(samples: *const c_void, layout: i32, responses: *const c_void, var_idx: *const c_void, sample_idx: *const c_void, sample_weights: *const c_void, var_type: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::TrainData::create(InputArray, Primitive, InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/ml.hpp:311
// ("cv::ml::TrainData::create", vec![(pred!(mut, ["samples", "layout", "responses"], ["const cv::_InputArray*", "int", "const cv::_InputArray*"]), _)]),
pub fn cv_ml_TrainData_create_const__InputArrayR_int_const__InputArrayR(samples: *const c_void, layout: i32, responses: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
// cv::ml::TrainData::delete() generated
// ("cv::ml::TrainData::delete", vec![(pred!(mut, [], []), _)]),
pub fn cv_ml_TrainData_delete(instance: *mut c_void);
