//! # Machine Learning
//! 
//! The Machine Learning Library (MLL) is a set of classes and functions for statistical
//! classification, regression, and clustering of data.
//! 
//! Most of the classification and regression algorithms are implemented as C++ classes. As the
//! algorithms have different sets of features (like an ability to handle missing measurements or
//! categorical input variables), there is a little common ground between the classes. This common
//! ground is defined by the class cv::ml::StatModel that all the other ML classes are derived from.
//! 
//! See detailed overview here: @ref ml_intro.
use std::os::raw::{c_char, c_void};
use libc::size_t;
use crate::{Error, Result, core, sys, types};

pub const ANN_MLP_ANNEAL: i32 = 2;
pub const ANN_MLP_BACKPROP: i32 = 0;
pub const ANN_MLP_GAUSSIAN: i32 = 2;
pub const ANN_MLP_IDENTITY: i32 = 0;
pub const ANN_MLP_LEAKYRELU: i32 = 4;
pub const ANN_MLP_NO_INPUT_SCALE: i32 = 2;
pub const ANN_MLP_NO_OUTPUT_SCALE: i32 = 4;
pub const ANN_MLP_RELU: i32 = 3;
pub const ANN_MLP_RPROP: i32 = 1;
pub const ANN_MLP_SIGMOID_SYM: i32 = 1;
pub const ANN_MLP_UPDATE_WEIGHTS: i32 = 1;
pub const Boost_DISCRETE: i32 = 0;
pub const Boost_GENTLE: i32 = 3;
pub const Boost_LOGIT: i32 = 2;
pub const Boost_REAL: i32 = 1;
pub const COL_SAMPLE: i32 = 1;
pub const DTrees_PREDICT_AUTO: i32 = 0;
pub const DTrees_PREDICT_MASK: i32 = (3<<8);
pub const DTrees_PREDICT_MAX_VOTE: i32 = (2<<8);
pub const DTrees_PREDICT_SUM: i32 = (1<<8);
pub const EM_COV_MAT_DEFAULT: i32 = 1;
pub const EM_COV_MAT_DIAGONAL: i32 = 1;
pub const EM_COV_MAT_GENERIC: i32 = 2;
pub const EM_COV_MAT_SPHERICAL: i32 = 0;
pub const EM_DEFAULT_MAX_ITERS: i32 = 100;
pub const EM_DEFAULT_NCLUSTERS: i32 = 5;
pub const EM_START_AUTO_STEP: i32 = 0;
pub const EM_START_E_STEP: i32 = 1;
pub const EM_START_M_STEP: i32 = 2;
pub const KNearest_BRUTE_FORCE: i32 = 1;
pub const KNearest_KDTREE: i32 = 2;
pub const LogisticRegression_BATCH: i32 = 0;
pub const LogisticRegression_MINI_BATCH: i32 = 1;
pub const LogisticRegression_REG_DISABLE: i32 = -1;
pub const LogisticRegression_REG_L1: i32 = 0;
pub const LogisticRegression_REG_L2: i32 = 1;
pub const ROW_SAMPLE: i32 = 0;
pub const SVMSGD_ASGD: i32 = 1;
pub const SVMSGD_HARD_MARGIN: i32 = 1;
pub const SVMSGD_SGD: i32 = 0;
pub const SVMSGD_SOFT_MARGIN: i32 = 0;
pub const SVM_C: i32 = 0;
pub const SVM_CHI2: i32 = 4;
pub const SVM_COEF: i32 = 4;
pub const SVM_CUSTOM: i32 = -1;
pub const SVM_C_SVC: i32 = 100;
pub const SVM_DEGREE: i32 = 5;
pub const SVM_EPS_SVR: i32 = 103;
pub const SVM_GAMMA: i32 = 1;
pub const SVM_INTER: i32 = 5;
pub const SVM_LINEAR: i32 = 0;
pub const SVM_NU: i32 = 3;
pub const SVM_NU_SVC: i32 = 101;
pub const SVM_NU_SVR: i32 = 104;
pub const SVM_ONE_CLASS: i32 = 102;
pub const SVM_P: i32 = 2;
pub const SVM_POLY: i32 = 1;
pub const SVM_RBF: i32 = 2;
pub const SVM_SIGMOID: i32 = 3;
pub const StatModel_COMPRESSED_INPUT: i32 = 2;
pub const StatModel_PREPROCESSED_INPUT: i32 = 4;
pub const StatModel_RAW_OUTPUT: i32 = 1;
pub const StatModel_UPDATE_MODEL: i32 = 1;
pub const TEST_ERROR: i32 = 0;
pub const TRAIN_ERROR: i32 = 1;
pub const VAR_CATEGORICAL: i32 = 1;
pub const VAR_NUMERICAL: i32 = 0;
pub const VAR_ORDERED: i32 = 0;

// identifier: cv_ml_createConcentricSpheresTestSet_int_nsamples_int_nfeatures_int_nclasses_Mat_samples_Mat_responses
/// Creates test set
pub fn create_concentric_spheres_test_set(nsamples: i32, nfeatures: i32, nclasses: i32, samples: &mut core::Mat, responses: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_ml_cv_ml_createConcentricSpheresTestSet_int_nsamples_int_nfeatures_int_nclasses_Mat_samples_Mat_responses(nsamples, nfeatures, nclasses, samples.as_raw_Mat(), responses.as_raw_Mat()) }.into_result()
}

// identifier: cv_ml_randMVNormal_Mat_mean_Mat_cov_int_nsamples_Mat_samples
/// Generates _sample_ from multivariate normal distribution
/// 
/// ## Parameters
/// * mean: an average row vector
/// * cov: symmetric covariation matrix
/// * nsamples: returned samples count
/// * samples: returned samples array
pub fn rand_mv_normal(mean: &core::Mat, cov: &core::Mat, nsamples: i32, samples: &mut core::Mat) -> Result<()> {
    unsafe { sys::cv_ml_cv_ml_randMVNormal_Mat_mean_Mat_cov_int_nsamples_Mat_samples(mean.as_raw_Mat(), cov.as_raw_Mat(), nsamples, samples.as_raw_Mat()) }.into_result()
}

// Generating impl for trait cv::ml::ANN_MLP (trait)
/// Artificial Neural Networks - Multi-Layer Perceptrons.
/// 
/// Unlike many other models in ML that are constructed and trained at once, in the MLP model these
/// steps are separated. First, a network with the specified topology is created using the non-default
/// constructor or the method ANN_MLP::create. All the weights are set to zeros. Then, the network is
/// trained using a set of input and output vectors. The training procedure can be repeated more than
/// once, that is, the weights can be adjusted based on the new training data.
/// 
/// Additional flags for StatModel::train are available: ANN_MLP::TrainFlags.
/// 
/// ## See also
/// @ref ml_intro_ann
pub trait ANN_MLP : crate::ml::StatModel {
    #[doc(hidden)] fn as_raw_ANN_MLP(&self) -> *mut c_void;
    // identifier: cv_ml_ANN_MLP_setTrainMethod_int_method_double_param1_double_param2
    /// Sets training method and common parameters.
    /// ## Parameters
    /// * method: Default value is ANN_MLP::RPROP. See ANN_MLP::TrainingMethods.
    /// * param1: passed to setRpropDW0 for ANN_MLP::RPROP and to setBackpropWeightScale for ANN_MLP::BACKPROP and to initialT for ANN_MLP::ANNEAL.
    /// * param2: passed to setRpropDWMin for ANN_MLP::RPROP and to setBackpropMomentumScale for ANN_MLP::BACKPROP and to finalT for ANN_MLP::ANNEAL.
    ///
    /// ## C++ default parameters:
    /// * param1: 0
    /// * param2: 0
    fn set_train_method(&mut self, method: i32, param1: f64, param2: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_setTrainMethod_int_method_double_param1_double_param2(self.as_raw_ANN_MLP(), method, param1, param2) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_getTrainMethod_const
    /// Returns current training method
    fn get_train_method(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_getTrainMethod_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_setActivationFunction_int_type_double_param1_double_param2
    /// <script type="text/javascript" src="https://latex.codecogs.com/latexit.js"></script>
    /// Initialize the activation function for each neuron.
    /// Currently the default and the only fully supported activation function is ANN_MLP::SIGMOID_SYM.
    /// ## Parameters
    /// * type: The type of activation function. See ANN_MLP::ActivationFunctions.
    /// * param1: The first parameter of the activation function, <span lang='latex'>\alpha</span>. Default value is 0.
    /// * param2: The second parameter of the activation function, <span lang='latex'>\beta</span>. Default value is 0.
    ///
    /// ## C++ default parameters:
    /// * param1: 0
    /// * param2: 0
    fn set_activation_function(&mut self, _type: i32, param1: f64, param2: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_setActivationFunction_int_type_double_param1_double_param2(self.as_raw_ANN_MLP(), _type, param1, param2) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_setLayerSizes_Mat__layer_sizes
    /// Integer vector specifying the number of neurons in each layer including the input and output layers.
    /// The very first element specifies the number of elements in the input layer.
    /// The last element - number of elements in the output layer. Default value is empty Mat.
    /// ## See also
    /// getLayerSizes
    fn set_layer_sizes(&mut self, _layer_sizes: &core::Mat) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_setLayerSizes_Mat__layer_sizes(self.as_raw_ANN_MLP(), _layer_sizes.as_raw_Mat()) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_getLayerSizes_const
    /// Integer vector specifying the number of neurons in each layer including the input and output layers.
    /// The very first element specifies the number of elements in the input layer.
    /// The last element - number of elements in the output layer.
    /// ## See also
    /// setLayerSizes
    fn get_layer_sizes(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_getLayerSizes_const(self.as_raw_ANN_MLP()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_ANN_MLP_getTermCriteria_const
    /// @see setTermCriteria
    fn get_term_criteria(&self) -> Result<core::TermCriteria> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_getTermCriteria_const(self.as_raw_ANN_MLP()) }.into_result().map(|x| core::TermCriteria { ptr: x })
    }
    
    // identifier: cv_ml_ANN_MLP_setTermCriteria_TermCriteria_val
    /// @copybrief getTermCriteria @see getTermCriteria
    fn set_term_criteria(&mut self, val: &core::TermCriteria) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_setTermCriteria_TermCriteria_val(self.as_raw_ANN_MLP(), val.as_raw_TermCriteria()) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_getBackpropWeightScale_const
    /// @see setBackpropWeightScale
    fn get_backprop_weight_scale(&self) -> Result<f64> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_getBackpropWeightScale_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_setBackpropWeightScale_double_val
    /// @copybrief getBackpropWeightScale @see getBackpropWeightScale
    fn set_backprop_weight_scale(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_setBackpropWeightScale_double_val(self.as_raw_ANN_MLP(), val) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_getBackpropMomentumScale_const
    /// @see setBackpropMomentumScale
    fn get_backprop_momentum_scale(&self) -> Result<f64> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_getBackpropMomentumScale_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_setBackpropMomentumScale_double_val
    /// @copybrief getBackpropMomentumScale @see getBackpropMomentumScale
    fn set_backprop_momentum_scale(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_setBackpropMomentumScale_double_val(self.as_raw_ANN_MLP(), val) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_getRpropDW0_const
    /// @see setRpropDW0
    fn get_rprop_dw0(&self) -> Result<f64> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_getRpropDW0_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_setRpropDW0_double_val
    /// @copybrief getRpropDW0 @see getRpropDW0
    fn set_rprop_dw0(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_setRpropDW0_double_val(self.as_raw_ANN_MLP(), val) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_getRpropDWPlus_const
    /// @see setRpropDWPlus
    fn get_rprop_dw_plus(&self) -> Result<f64> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_getRpropDWPlus_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_setRpropDWPlus_double_val
    /// @copybrief getRpropDWPlus @see getRpropDWPlus
    fn set_rprop_dw_plus(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_setRpropDWPlus_double_val(self.as_raw_ANN_MLP(), val) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_getRpropDWMinus_const
    /// @see setRpropDWMinus
    fn get_rprop_dw_minus(&self) -> Result<f64> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_getRpropDWMinus_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_setRpropDWMinus_double_val
    /// @copybrief getRpropDWMinus @see getRpropDWMinus
    fn set_rprop_dw_minus(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_setRpropDWMinus_double_val(self.as_raw_ANN_MLP(), val) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_getRpropDWMin_const
    /// @see setRpropDWMin
    fn get_rprop_dw_min(&self) -> Result<f64> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_getRpropDWMin_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_setRpropDWMin_double_val
    /// @copybrief getRpropDWMin @see getRpropDWMin
    fn set_rprop_dw_min(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_setRpropDWMin_double_val(self.as_raw_ANN_MLP(), val) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_getRpropDWMax_const
    /// @see setRpropDWMax
    fn get_rprop_dw_max(&self) -> Result<f64> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_getRpropDWMax_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_setRpropDWMax_double_val
    /// @copybrief getRpropDWMax @see getRpropDWMax
    fn set_rprop_dw_max(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_setRpropDWMax_double_val(self.as_raw_ANN_MLP(), val) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_getAnnealInitialT_const
    /// @see setAnnealInitialT
    fn get_anneal_initial_t(&self) -> Result<f64> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_getAnnealInitialT_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_setAnnealInitialT_double_val
    /// @copybrief getAnnealInitialT @see getAnnealInitialT
    fn set_anneal_initial_t(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_setAnnealInitialT_double_val(self.as_raw_ANN_MLP(), val) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_getAnnealFinalT_const
    /// @see setAnnealFinalT
    fn get_anneal_final_t(&self) -> Result<f64> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_getAnnealFinalT_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_setAnnealFinalT_double_val
    /// @copybrief getAnnealFinalT @see getAnnealFinalT
    fn set_anneal_final_t(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_setAnnealFinalT_double_val(self.as_raw_ANN_MLP(), val) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_getAnnealCoolingRatio_const
    /// @see setAnnealCoolingRatio
    fn get_anneal_cooling_ratio(&self) -> Result<f64> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_getAnnealCoolingRatio_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_setAnnealCoolingRatio_double_val
    /// @copybrief getAnnealCoolingRatio @see getAnnealCoolingRatio
    fn set_anneal_cooling_ratio(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_setAnnealCoolingRatio_double_val(self.as_raw_ANN_MLP(), val) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_getAnnealItePerStep_const
    /// @see setAnnealItePerStep
    fn get_anneal_ite_per_step(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_getAnnealItePerStep_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_setAnnealItePerStep_int_val
    /// @copybrief getAnnealItePerStep @see getAnnealItePerStep
    fn set_anneal_ite_per_step(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_setAnnealItePerStep_int_val(self.as_raw_ANN_MLP(), val) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_getWeights_const_int_layerIdx
    fn get_weights(&self, layer_idx: i32) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_getWeights_const_int_layerIdx(self.as_raw_ANN_MLP(), layer_idx) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
}

impl<'a> ANN_MLP + 'a {

    // identifier: cv_ml_ANN_MLP_create
    /// Creates empty model
    /// 
    /// Use StatModel::train to train the model, Algorithm::load\<ANN_MLP\>(filename) to load the pre-trained model.
    /// Note that the train method has optional flags: ANN_MLP::TrainFlags.
    pub fn create() -> Result<types::PtrOfANN_MLP> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_create() }.into_result().map(|x| types::PtrOfANN_MLP { ptr: x })
    }
    
    // identifier: cv_ml_ANN_MLP_load_String_filepath
    /// Loads and creates a serialized ANN from a file
    /// 
    /// Use ANN::save to serialize and store an ANN to disk.
    /// Load the ANN from this file again, by calling this function with the path to the file.
    /// 
    /// ## Parameters
    /// * filepath: path to serialized ANN
    pub fn load(filepath: &str) -> Result<types::PtrOfANN_MLP> {
        string_arg!(filepath);
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_load_String_filepath(filepath.as_ptr()) }.into_result().map(|x| types::PtrOfANN_MLP { ptr: x })
    }
    
}

// Generating impl for trait cv::ml::ANN_MLP_ANNEAL (trait)
/// Artificial Neural Networks - Multi-Layer Perceptrons.
/// 
/// ## See also
/// @ref ml_intro_ann
pub trait ANN_MLP_ANNEAL : crate::ml::ANN_MLP {
    #[doc(hidden)] fn as_raw_ANN_MLP_ANNEAL(&self) -> *mut c_void;
    // identifier: cv_ml_ANN_MLP_ANNEAL_getAnnealInitialT_const
    /// @see setAnnealInitialT
    fn get_anneal_initial_t(&self) -> Result<f64> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_ANNEAL_getAnnealInitialT_const(self.as_raw_ANN_MLP_ANNEAL()) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_ANNEAL_setAnnealInitialT_double_val
    /// @copybrief getAnnealInitialT @see getAnnealInitialT
    fn set_anneal_initial_t(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_ANNEAL_setAnnealInitialT_double_val(self.as_raw_ANN_MLP_ANNEAL(), val) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_ANNEAL_getAnnealFinalT_const
    /// @see setAnnealFinalT
    fn get_anneal_final_t(&self) -> Result<f64> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_ANNEAL_getAnnealFinalT_const(self.as_raw_ANN_MLP_ANNEAL()) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_ANNEAL_setAnnealFinalT_double_val
    /// @copybrief getAnnealFinalT @see getAnnealFinalT
    fn set_anneal_final_t(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_ANNEAL_setAnnealFinalT_double_val(self.as_raw_ANN_MLP_ANNEAL(), val) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_ANNEAL_getAnnealCoolingRatio_const
    /// @see setAnnealCoolingRatio
    fn get_anneal_cooling_ratio(&self) -> Result<f64> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_ANNEAL_getAnnealCoolingRatio_const(self.as_raw_ANN_MLP_ANNEAL()) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_ANNEAL_setAnnealCoolingRatio_double_val
    /// @copybrief getAnnealCoolingRatio @see getAnnealCoolingRatio
    fn set_anneal_cooling_ratio(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_ANNEAL_setAnnealCoolingRatio_double_val(self.as_raw_ANN_MLP_ANNEAL(), val) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_ANNEAL_getAnnealItePerStep_const
    /// @see setAnnealItePerStep
    fn get_anneal_ite_per_step(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_ANNEAL_getAnnealItePerStep_const(self.as_raw_ANN_MLP_ANNEAL()) }.into_result()
    }
    
    // identifier: cv_ml_ANN_MLP_ANNEAL_setAnnealItePerStep_int_val
    /// @copybrief getAnnealItePerStep @see getAnnealItePerStep
    fn set_anneal_ite_per_step(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_ANN_MLP_ANNEAL_setAnnealItePerStep_int_val(self.as_raw_ANN_MLP_ANNEAL(), val) }.into_result()
    }
    
}

impl<'a> ANN_MLP_ANNEAL + 'a {

}

// Generating impl for trait cv::ml::Boost (trait)
/// Boosted tree classifier derived from DTrees
/// 
/// ## See also
/// @ref ml_intro_boost
pub trait Boost : crate::ml::DTrees {
    #[doc(hidden)] fn as_raw_Boost(&self) -> *mut c_void;
    // identifier: cv_ml_Boost_getBoostType_const
    /// @see setBoostType
    fn get_boost_type(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_Boost_getBoostType_const(self.as_raw_Boost()) }.into_result()
    }
    
    // identifier: cv_ml_Boost_setBoostType_int_val
    /// @copybrief getBoostType @see getBoostType
    fn set_boost_type(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_Boost_setBoostType_int_val(self.as_raw_Boost(), val) }.into_result()
    }
    
    // identifier: cv_ml_Boost_getWeakCount_const
    /// @see setWeakCount
    fn get_weak_count(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_Boost_getWeakCount_const(self.as_raw_Boost()) }.into_result()
    }
    
    // identifier: cv_ml_Boost_setWeakCount_int_val
    /// @copybrief getWeakCount @see getWeakCount
    fn set_weak_count(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_Boost_setWeakCount_int_val(self.as_raw_Boost(), val) }.into_result()
    }
    
    // identifier: cv_ml_Boost_getWeightTrimRate_const
    /// @see setWeightTrimRate
    fn get_weight_trim_rate(&self) -> Result<f64> {
        unsafe { sys::cv_ml_cv_ml_Boost_getWeightTrimRate_const(self.as_raw_Boost()) }.into_result()
    }
    
    // identifier: cv_ml_Boost_setWeightTrimRate_double_val
    /// @copybrief getWeightTrimRate @see getWeightTrimRate
    fn set_weight_trim_rate(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_Boost_setWeightTrimRate_double_val(self.as_raw_Boost(), val) }.into_result()
    }
    
}

impl<'a> Boost + 'a {

    // identifier: cv_ml_Boost_create
    /// Creates the empty model.
    /// Use StatModel::train to train the model, Algorithm::load\<Boost\>(filename) to load the pre-trained model.
    pub fn create() -> Result<types::PtrOfBoost> {
        unsafe { sys::cv_ml_cv_ml_Boost_create() }.into_result().map(|x| types::PtrOfBoost { ptr: x })
    }
    
    // identifier: cv_ml_Boost_load_String_filepath_String_nodeName
    /// Loads and creates a serialized Boost from a file
    /// 
    /// Use Boost::save to serialize and store an RTree to disk.
    /// Load the Boost from this file again, by calling this function with the path to the file.
    /// Optionally specify the node for the file containing the classifier
    /// 
    /// ## Parameters
    /// * filepath: path to serialized Boost
    /// * nodeName: name of node containing the classifier
    ///
    /// ## C++ default parameters:
    /// * node_name: String()
    pub fn load(filepath: &str, node_name: &str) -> Result<types::PtrOfBoost> {
        string_arg!(filepath);
        string_arg!(node_name);
        unsafe { sys::cv_ml_cv_ml_Boost_load_String_filepath_String_nodeName(filepath.as_ptr(), node_name.as_ptr()) }.into_result().map(|x| types::PtrOfBoost { ptr: x })
    }
    
}

// Generating impl for trait cv::ml::DTrees (trait)
/// The class represents a single decision tree or a collection of decision trees.
/// 
/// The current public interface of the class allows user to train only a single decision tree, however
/// the class is capable of storing multiple decision trees and using them for prediction (by summing
/// responses or using a voting schemes), and the derived from DTrees classes (such as RTrees and Boost)
/// use this capability to implement decision tree ensembles.
/// 
/// ## See also
/// @ref ml_intro_trees
pub trait DTrees : crate::ml::StatModel {
    #[doc(hidden)] fn as_raw_DTrees(&self) -> *mut c_void;
    // identifier: cv_ml_DTrees_getMaxCategories_const
    /// @see setMaxCategories
    fn get_max_categories(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_DTrees_getMaxCategories_const(self.as_raw_DTrees()) }.into_result()
    }
    
    // identifier: cv_ml_DTrees_setMaxCategories_int_val
    /// @copybrief getMaxCategories @see getMaxCategories
    fn set_max_categories(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_DTrees_setMaxCategories_int_val(self.as_raw_DTrees(), val) }.into_result()
    }
    
    // identifier: cv_ml_DTrees_getMaxDepth_const
    /// @see setMaxDepth
    fn get_max_depth(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_DTrees_getMaxDepth_const(self.as_raw_DTrees()) }.into_result()
    }
    
    // identifier: cv_ml_DTrees_setMaxDepth_int_val
    /// @copybrief getMaxDepth @see getMaxDepth
    fn set_max_depth(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_DTrees_setMaxDepth_int_val(self.as_raw_DTrees(), val) }.into_result()
    }
    
    // identifier: cv_ml_DTrees_getMinSampleCount_const
    /// @see setMinSampleCount
    fn get_min_sample_count(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_DTrees_getMinSampleCount_const(self.as_raw_DTrees()) }.into_result()
    }
    
    // identifier: cv_ml_DTrees_setMinSampleCount_int_val
    /// @copybrief getMinSampleCount @see getMinSampleCount
    fn set_min_sample_count(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_DTrees_setMinSampleCount_int_val(self.as_raw_DTrees(), val) }.into_result()
    }
    
    // identifier: cv_ml_DTrees_getCVFolds_const
    /// @see setCVFolds
    fn get_cv_folds(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_DTrees_getCVFolds_const(self.as_raw_DTrees()) }.into_result()
    }
    
    // identifier: cv_ml_DTrees_setCVFolds_int_val
    /// @copybrief getCVFolds @see getCVFolds
    fn set_cv_folds(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_DTrees_setCVFolds_int_val(self.as_raw_DTrees(), val) }.into_result()
    }
    
    // identifier: cv_ml_DTrees_getUseSurrogates_const
    /// @see setUseSurrogates
    fn get_use_surrogates(&self) -> Result<bool> {
        unsafe { sys::cv_ml_cv_ml_DTrees_getUseSurrogates_const(self.as_raw_DTrees()) }.into_result()
    }
    
    // identifier: cv_ml_DTrees_setUseSurrogates_bool_val
    /// @copybrief getUseSurrogates @see getUseSurrogates
    fn set_use_surrogates(&mut self, val: bool) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_DTrees_setUseSurrogates_bool_val(self.as_raw_DTrees(), val) }.into_result()
    }
    
    // identifier: cv_ml_DTrees_getUse1SERule_const
    /// @see setUse1SERule
    fn get_use1_se_rule(&self) -> Result<bool> {
        unsafe { sys::cv_ml_cv_ml_DTrees_getUse1SERule_const(self.as_raw_DTrees()) }.into_result()
    }
    
    // identifier: cv_ml_DTrees_setUse1SERule_bool_val
    /// @copybrief getUse1SERule @see getUse1SERule
    fn set_use1_se_rule(&mut self, val: bool) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_DTrees_setUse1SERule_bool_val(self.as_raw_DTrees(), val) }.into_result()
    }
    
    // identifier: cv_ml_DTrees_getTruncatePrunedTree_const
    /// @see setTruncatePrunedTree
    fn get_truncate_pruned_tree(&self) -> Result<bool> {
        unsafe { sys::cv_ml_cv_ml_DTrees_getTruncatePrunedTree_const(self.as_raw_DTrees()) }.into_result()
    }
    
    // identifier: cv_ml_DTrees_setTruncatePrunedTree_bool_val
    /// @copybrief getTruncatePrunedTree @see getTruncatePrunedTree
    fn set_truncate_pruned_tree(&mut self, val: bool) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_DTrees_setTruncatePrunedTree_bool_val(self.as_raw_DTrees(), val) }.into_result()
    }
    
    // identifier: cv_ml_DTrees_getRegressionAccuracy_const
    /// @see setRegressionAccuracy
    fn get_regression_accuracy(&self) -> Result<f32> {
        unsafe { sys::cv_ml_cv_ml_DTrees_getRegressionAccuracy_const(self.as_raw_DTrees()) }.into_result()
    }
    
    // identifier: cv_ml_DTrees_setRegressionAccuracy_float_val
    /// @copybrief getRegressionAccuracy @see getRegressionAccuracy
    fn set_regression_accuracy(&mut self, val: f32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_DTrees_setRegressionAccuracy_float_val(self.as_raw_DTrees(), val) }.into_result()
    }
    
    // identifier: cv_ml_DTrees_getPriors_const
    /// @see setPriors
    fn get_priors(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_DTrees_getPriors_const(self.as_raw_DTrees()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_DTrees_setPriors_Mat_val
    /// @copybrief getPriors @see getPriors
    fn set_priors(&mut self, val: &core::Mat) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_DTrees_setPriors_Mat_val(self.as_raw_DTrees(), val.as_raw_Mat()) }.into_result()
    }
    
    // identifier: cv_ml_DTrees_getRoots_const
    /// Returns indices of root nodes
    fn get_roots(&self) -> Result<types::VectorOfint> {
        unsafe { sys::cv_ml_cv_ml_DTrees_getRoots_const(self.as_raw_DTrees()) }.into_result().map(|x| types::VectorOfint { ptr: x })
    }
    
    // identifier: cv_ml_DTrees_getNodes_const
    /// Returns all the nodes
    /// 
    /// all the node indices are indices in the returned vector
    fn get_nodes(&self) -> Result<types::VectorOfNode> {
        unsafe { sys::cv_ml_cv_ml_DTrees_getNodes_const(self.as_raw_DTrees()) }.into_result().map(|x| types::VectorOfNode { ptr: x })
    }
    
    // identifier: cv_ml_DTrees_getSplits_const
    /// Returns all the splits
    /// 
    /// all the split indices are indices in the returned vector
    fn get_splits(&self) -> Result<types::VectorOfSplit> {
        unsafe { sys::cv_ml_cv_ml_DTrees_getSplits_const(self.as_raw_DTrees()) }.into_result().map(|x| types::VectorOfSplit { ptr: x })
    }
    
    // identifier: cv_ml_DTrees_getSubsets_const
    /// Returns all the bitsets for categorical splits
    /// 
    /// Split::subsetOfs is an offset in the returned vector
    fn get_subsets(&self) -> Result<types::VectorOfint> {
        unsafe { sys::cv_ml_cv_ml_DTrees_getSubsets_const(self.as_raw_DTrees()) }.into_result().map(|x| types::VectorOfint { ptr: x })
    }
    
}

impl<'a> DTrees + 'a {

    // identifier: cv_ml_DTrees_create
    /// Creates the empty model
    /// 
    /// The static method creates empty decision tree with the specified parameters. It should be then
    /// trained using train method (see StatModel::train). Alternatively, you can load the model from
    /// file using Algorithm::load\<DTrees\>(filename).
    pub fn create() -> Result<types::PtrOfDTrees> {
        unsafe { sys::cv_ml_cv_ml_DTrees_create() }.into_result().map(|x| types::PtrOfDTrees { ptr: x })
    }
    
    // identifier: cv_ml_DTrees_load_String_filepath_String_nodeName
    /// Loads and creates a serialized DTrees from a file
    /// 
    /// Use DTree::save to serialize and store an DTree to disk.
    /// Load the DTree from this file again, by calling this function with the path to the file.
    /// Optionally specify the node for the file containing the classifier
    /// 
    /// ## Parameters
    /// * filepath: path to serialized DTree
    /// * nodeName: name of node containing the classifier
    ///
    /// ## C++ default parameters:
    /// * node_name: String()
    pub fn load(filepath: &str, node_name: &str) -> Result<types::PtrOfDTrees> {
        string_arg!(filepath);
        string_arg!(node_name);
        unsafe { sys::cv_ml_cv_ml_DTrees_load_String_filepath_String_nodeName(filepath.as_ptr(), node_name.as_ptr()) }.into_result().map(|x| types::PtrOfDTrees { ptr: x })
    }
    
}

// boxed class cv::ml::DTrees::Node
/// The class represents a decision tree node.
#[allow(dead_code)]
pub struct DTrees_Node {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for crate::ml::DTrees_Node {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_DTrees_Node(self.ptr) };
    }
}
impl crate::ml::DTrees_Node {
    #[doc(hidden)] pub fn as_raw_DTrees_Node(&self) -> *mut c_void { self.ptr }
}

impl DTrees_Node {

    // identifier: cv_ml_DTrees_Node_Node
    pub fn new() -> Result<crate::ml::DTrees_Node> {
        unsafe { sys::cv_ml_cv_ml_DTrees_Node_Node() }.into_result().map(|x| crate::ml::DTrees_Node { ptr: x })
    }
    
}

// boxed class cv::ml::DTrees::Split
/// The class represents split in a decision tree.
#[allow(dead_code)]
pub struct DTrees_Split {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for crate::ml::DTrees_Split {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_DTrees_Split(self.ptr) };
    }
}
impl crate::ml::DTrees_Split {
    #[doc(hidden)] pub fn as_raw_DTrees_Split(&self) -> *mut c_void { self.ptr }
}

impl DTrees_Split {

    // identifier: cv_ml_DTrees_Split_Split
    pub fn new() -> Result<crate::ml::DTrees_Split> {
        unsafe { sys::cv_ml_cv_ml_DTrees_Split_Split() }.into_result().map(|x| crate::ml::DTrees_Split { ptr: x })
    }
    
}

// Generating impl for trait cv::ml::EM (trait)
/// The class implements the Expectation Maximization algorithm.
/// 
/// ## See also
/// @ref ml_intro_em
pub trait EM : crate::ml::StatModel {
    #[doc(hidden)] fn as_raw_EM(&self) -> *mut c_void;
    // identifier: cv_ml_EM_getClustersNumber_const
    /// @see setClustersNumber
    fn get_clusters_number(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_EM_getClustersNumber_const(self.as_raw_EM()) }.into_result()
    }
    
    // identifier: cv_ml_EM_setClustersNumber_int_val
    /// @copybrief getClustersNumber @see getClustersNumber
    fn set_clusters_number(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_EM_setClustersNumber_int_val(self.as_raw_EM(), val) }.into_result()
    }
    
    // identifier: cv_ml_EM_getCovarianceMatrixType_const
    /// @see setCovarianceMatrixType
    fn get_covariance_matrix_type(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_EM_getCovarianceMatrixType_const(self.as_raw_EM()) }.into_result()
    }
    
    // identifier: cv_ml_EM_setCovarianceMatrixType_int_val
    /// @copybrief getCovarianceMatrixType @see getCovarianceMatrixType
    fn set_covariance_matrix_type(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_EM_setCovarianceMatrixType_int_val(self.as_raw_EM(), val) }.into_result()
    }
    
    // identifier: cv_ml_EM_getTermCriteria_const
    /// @see setTermCriteria
    fn get_term_criteria(&self) -> Result<core::TermCriteria> {
        unsafe { sys::cv_ml_cv_ml_EM_getTermCriteria_const(self.as_raw_EM()) }.into_result().map(|x| core::TermCriteria { ptr: x })
    }
    
    // identifier: cv_ml_EM_setTermCriteria_TermCriteria_val
    /// @copybrief getTermCriteria @see getTermCriteria
    fn set_term_criteria(&mut self, val: &core::TermCriteria) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_EM_setTermCriteria_TermCriteria_val(self.as_raw_EM(), val.as_raw_TermCriteria()) }.into_result()
    }
    
    // identifier: cv_ml_EM_getWeights_const
    /// Returns weights of the mixtures
    /// 
    /// Returns vector with the number of elements equal to the number of mixtures.
    fn get_weights(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_EM_getWeights_const(self.as_raw_EM()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_EM_getMeans_const
    /// Returns the cluster centers (means of the Gaussian mixture)
    /// 
    /// Returns matrix with the number of rows equal to the number of mixtures and number of columns
    /// equal to the space dimensionality.
    fn get_means(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_EM_getMeans_const(self.as_raw_EM()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_EM_getCovs_const_VectorOfMat_covs
    /// Returns covariation matrices
    /// 
    /// Returns vector of covariation matrices. Number of matrices is the number of gaussian mixtures,
    /// each matrix is a square floating-point matrix NxN, where N is the space dimensionality.
    fn get_covs(&self, covs: &types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_EM_getCovs_const_VectorOfMat_covs(self.as_raw_EM(), covs.as_raw_VectorOfMat()) }.into_result()
    }
    
    // identifier: cv_ml_EM_predict_const_Mat_samples_Mat_results_int_flags
    /// <script type="text/javascript" src="https://latex.codecogs.com/latexit.js"></script>
    ///  Returns posterior probabilities for the provided samples
    /// 
    /// ## Parameters
    /// * samples: The input samples, floating-point matrix
    /// * results: The optional output <span lang='latex'> nSamples \times nClusters</span> matrix of results. It contains
    /// posterior probabilities for each sample from the input
    /// * flags: This parameter will be ignored
    ///
    /// ## C++ default parameters:
    /// * results: noArray()
    /// * flags: 0
    fn predict(&self, samples: &core::Mat, results: &mut core::Mat, flags: i32) -> Result<f32> {
        unsafe { sys::cv_ml_cv_ml_EM_predict_const_Mat_samples_Mat_results_int_flags(self.as_raw_EM(), samples.as_raw_Mat(), results.as_raw_Mat(), flags) }.into_result()
    }
    
    // identifier: cv_ml_EM_predict2_const_Mat_sample_Mat_probs
    /// <script type="text/javascript" src="https://latex.codecogs.com/latexit.js"></script>
    ///  Returns a likelihood logarithm value and an index of the most probable mixture component
    /// for the given sample.
    /// 
    /// ## Parameters
    /// * sample: A sample for classification. It should be a one-channel matrix of
    /// <span lang='latex'>1 \times dims</span> or <span lang='latex'>dims \times 1</span> size.
    /// * probs: Optional output matrix that contains posterior probabilities of each component
    /// given the sample. It has <span lang='latex'>1 \times nclusters</span> size and CV_64FC1 type.
    /// 
    /// The method returns a two-element double vector. Zero element is a likelihood logarithm value for
    /// the sample. First element is an index of the most probable mixture component for the given
    /// sample.
    fn predict2(&self, sample: &core::Mat, probs: &mut core::Mat) -> Result<core::Vec2d> {
        unsafe { sys::cv_ml_cv_ml_EM_predict2_const_Mat_sample_Mat_probs(self.as_raw_EM(), sample.as_raw_Mat(), probs.as_raw_Mat()) }.into_result()
    }
    
    // identifier: cv_ml_EM_trainEM_Mat_samples_Mat_logLikelihoods_Mat_labels_Mat_probs
    /// <script type="text/javascript" src="https://latex.codecogs.com/latexit.js"></script>
    ///  Estimate the Gaussian mixture parameters from a samples set.
    /// 
    /// This variation starts with Expectation step. Initial values of the model parameters will be
    /// estimated by the k-means algorithm.
    /// 
    /// Unlike many of the ML models, %EM is an unsupervised learning algorithm and it does not take
    /// responses (class labels or function values) as input. Instead, it computes the *Maximum
    /// Likelihood Estimate* of the Gaussian mixture parameters from an input sample set, stores all the
    /// parameters inside the structure: <span lang='latex'>p_{i,k}</span> in probs, <span lang='latex'>a_k</span> in means , <span lang='latex'>S_k</span> in
    /// covs[k], <span lang='latex'>\pi_k</span> in weights , and optionally computes the output "class label" for each
    /// sample: <span lang='latex'>\texttt{labels}_i=\texttt{arg max}_k(p_{i,k}), i=1..N</span> (indices of the most
    /// probable mixture component for each sample).
    /// 
    /// The trained model can be used further for prediction, just like any other classifier. The
    /// trained model is similar to the NormalBayesClassifier.
    /// 
    /// ## Parameters
    /// * samples: Samples from which the Gaussian mixture model will be estimated. It should be a
    /// one-channel matrix, each row of which is a sample. If the matrix does not have CV_64F type
    /// it will be converted to the inner matrix of such type for the further computing.
    /// * logLikelihoods: The optional output matrix that contains a likelihood logarithm value for
    /// each sample. It has <span lang='latex'>nsamples \times 1</span> size and CV_64FC1 type.
    /// * labels: The optional output "class label" for each sample:
    /// <span lang='latex'>\texttt{labels}_i=\texttt{arg max}_k(p_{i,k}), i=1..N</span> (indices of the most probable
    /// mixture component for each sample). It has <span lang='latex'>nsamples \times 1</span> size and CV_32SC1 type.
    /// * probs: The optional output matrix that contains posterior probabilities of each Gaussian
    /// mixture component given the each sample. It has <span lang='latex'>nsamples \times nclusters</span> size and
    /// CV_64FC1 type.
    ///
    /// ## C++ default parameters:
    /// * log_likelihoods: noArray()
    /// * labels: noArray()
    /// * probs: noArray()
    fn train_em(&mut self, samples: &core::Mat, log_likelihoods: &mut core::Mat, labels: &mut core::Mat, probs: &mut core::Mat) -> Result<bool> {
        unsafe { sys::cv_ml_cv_ml_EM_trainEM_Mat_samples_Mat_logLikelihoods_Mat_labels_Mat_probs(self.as_raw_EM(), samples.as_raw_Mat(), log_likelihoods.as_raw_Mat(), labels.as_raw_Mat(), probs.as_raw_Mat()) }.into_result()
    }
    
    // identifier: cv_ml_EM_trainE_Mat_samples_Mat_means0_Mat_covs0_Mat_weights0_Mat_logLikelihoods_Mat_labels_Mat_probs
    /// <script type="text/javascript" src="https://latex.codecogs.com/latexit.js"></script>
    ///  Estimate the Gaussian mixture parameters from a samples set.
    /// 
    /// This variation starts with Expectation step. You need to provide initial means <span lang='latex'>a_k</span> of
    /// mixture components. Optionally you can pass initial weights <span lang='latex'>\pi_k</span> and covariance matrices
    /// <span lang='latex'>S_k</span> of mixture components.
    /// 
    /// ## Parameters
    /// * samples: Samples from which the Gaussian mixture model will be estimated. It should be a
    /// one-channel matrix, each row of which is a sample. If the matrix does not have CV_64F type
    /// it will be converted to the inner matrix of such type for the further computing.
    /// * means0: Initial means <span lang='latex'>a_k</span> of mixture components. It is a one-channel matrix of
    /// <span lang='latex'>nclusters \times dims</span> size. If the matrix does not have CV_64F type it will be
    /// converted to the inner matrix of such type for the further computing.
    /// * covs0: The vector of initial covariance matrices <span lang='latex'>S_k</span> of mixture components. Each of
    /// covariance matrices is a one-channel matrix of <span lang='latex'>dims \times dims</span> size. If the matrices
    /// do not have CV_64F type they will be converted to the inner matrices of such type for the
    /// further computing.
    /// * weights0: Initial weights <span lang='latex'>\pi_k</span> of mixture components. It should be a one-channel
    /// floating-point matrix with <span lang='latex'>1 \times nclusters</span> or <span lang='latex'>nclusters \times 1</span> size.
    /// * logLikelihoods: The optional output matrix that contains a likelihood logarithm value for
    /// each sample. It has <span lang='latex'>nsamples \times 1</span> size and CV_64FC1 type.
    /// * labels: The optional output "class label" for each sample:
    /// <span lang='latex'>\texttt{labels}_i=\texttt{arg max}_k(p_{i,k}), i=1..N</span> (indices of the most probable
    /// mixture component for each sample). It has <span lang='latex'>nsamples \times 1</span> size and CV_32SC1 type.
    /// * probs: The optional output matrix that contains posterior probabilities of each Gaussian
    /// mixture component given the each sample. It has <span lang='latex'>nsamples \times nclusters</span> size and
    /// CV_64FC1 type.
    ///
    /// ## C++ default parameters:
    /// * covs0: noArray()
    /// * weights0: noArray()
    /// * log_likelihoods: noArray()
    /// * labels: noArray()
    /// * probs: noArray()
    fn train_e(&mut self, samples: &core::Mat, means0: &core::Mat, covs0: &core::Mat, weights0: &core::Mat, log_likelihoods: &mut core::Mat, labels: &mut core::Mat, probs: &mut core::Mat) -> Result<bool> {
        unsafe { sys::cv_ml_cv_ml_EM_trainE_Mat_samples_Mat_means0_Mat_covs0_Mat_weights0_Mat_logLikelihoods_Mat_labels_Mat_probs(self.as_raw_EM(), samples.as_raw_Mat(), means0.as_raw_Mat(), covs0.as_raw_Mat(), weights0.as_raw_Mat(), log_likelihoods.as_raw_Mat(), labels.as_raw_Mat(), probs.as_raw_Mat()) }.into_result()
    }
    
    // identifier: cv_ml_EM_trainM_Mat_samples_Mat_probs0_Mat_logLikelihoods_Mat_labels_Mat_probs
    /// <script type="text/javascript" src="https://latex.codecogs.com/latexit.js"></script>
    ///  Estimate the Gaussian mixture parameters from a samples set.
    /// 
    /// This variation starts with Maximization step. You need to provide initial probabilities
    /// <span lang='latex'>p_{i,k}</span> to use this option.
    /// 
    /// ## Parameters
    /// * samples: Samples from which the Gaussian mixture model will be estimated. It should be a
    /// one-channel matrix, each row of which is a sample. If the matrix does not have CV_64F type
    /// it will be converted to the inner matrix of such type for the further computing.
    /// * probs0
    /// @param: logLikelihoods The optional output matrix that contains a likelihood logarithm value for
    /// each sample. It has <span lang='latex'>nsamples \times 1</span> size and CV_64FC1 type.
    /// * labels: The optional output "class label" for each sample:
    /// <span lang='latex'>\texttt{labels}_i=\texttt{arg max}_k(p_{i,k}), i=1..N</span> (indices of the most probable
    /// mixture component for each sample). It has <span lang='latex'>nsamples \times 1</span> size and CV_32SC1 type.
    /// * probs: The optional output matrix that contains posterior probabilities of each Gaussian
    /// mixture component given the each sample. It has <span lang='latex'>nsamples \times nclusters</span> size and
    /// CV_64FC1 type.
    ///
    /// ## C++ default parameters:
    /// * log_likelihoods: noArray()
    /// * labels: noArray()
    /// * probs: noArray()
    fn train_m(&mut self, samples: &core::Mat, probs0: &core::Mat, log_likelihoods: &mut core::Mat, labels: &mut core::Mat, probs: &mut core::Mat) -> Result<bool> {
        unsafe { sys::cv_ml_cv_ml_EM_trainM_Mat_samples_Mat_probs0_Mat_logLikelihoods_Mat_labels_Mat_probs(self.as_raw_EM(), samples.as_raw_Mat(), probs0.as_raw_Mat(), log_likelihoods.as_raw_Mat(), labels.as_raw_Mat(), probs.as_raw_Mat()) }.into_result()
    }
    
}

impl<'a> EM + 'a {

    // identifier: cv_ml_EM_create
    /// Creates empty %EM model.
    /// The model should be trained then using StatModel::train(traindata, flags) method. Alternatively, you
    /// can use one of the EM::train\* methods or load it from file using Algorithm::load\<EM\>(filename).
    pub fn create() -> Result<types::PtrOfEM> {
        unsafe { sys::cv_ml_cv_ml_EM_create() }.into_result().map(|x| types::PtrOfEM { ptr: x })
    }
    
    // identifier: cv_ml_EM_load_String_filepath_String_nodeName
    /// Loads and creates a serialized EM from a file
    /// 
    /// Use EM::save to serialize and store an EM to disk.
    /// Load the EM from this file again, by calling this function with the path to the file.
    /// Optionally specify the node for the file containing the classifier
    /// 
    /// ## Parameters
    /// * filepath: path to serialized EM
    /// * nodeName: name of node containing the classifier
    ///
    /// ## C++ default parameters:
    /// * node_name: String()
    pub fn load(filepath: &str, node_name: &str) -> Result<types::PtrOfEM> {
        string_arg!(filepath);
        string_arg!(node_name);
        unsafe { sys::cv_ml_cv_ml_EM_load_String_filepath_String_nodeName(filepath.as_ptr(), node_name.as_ptr()) }.into_result().map(|x| types::PtrOfEM { ptr: x })
    }
    
}

// Generating impl for trait cv::ml::KNearest (trait)
/// The class implements K-Nearest Neighbors model
/// 
/// ## See also
/// @ref ml_intro_knn
pub trait KNearest : crate::ml::StatModel {
    #[doc(hidden)] fn as_raw_KNearest(&self) -> *mut c_void;
    // identifier: cv_ml_KNearest_getDefaultK_const
    /// @see setDefaultK
    fn get_default_k(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_KNearest_getDefaultK_const(self.as_raw_KNearest()) }.into_result()
    }
    
    // identifier: cv_ml_KNearest_setDefaultK_int_val
    /// @copybrief getDefaultK @see getDefaultK
    fn set_default_k(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_KNearest_setDefaultK_int_val(self.as_raw_KNearest(), val) }.into_result()
    }
    
    // identifier: cv_ml_KNearest_getIsClassifier_const
    /// @see setIsClassifier
    fn get_is_classifier(&self) -> Result<bool> {
        unsafe { sys::cv_ml_cv_ml_KNearest_getIsClassifier_const(self.as_raw_KNearest()) }.into_result()
    }
    
    // identifier: cv_ml_KNearest_setIsClassifier_bool_val
    /// @copybrief getIsClassifier @see getIsClassifier
    fn set_is_classifier(&mut self, val: bool) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_KNearest_setIsClassifier_bool_val(self.as_raw_KNearest(), val) }.into_result()
    }
    
    // identifier: cv_ml_KNearest_getEmax_const
    /// @see setEmax
    fn get_emax(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_KNearest_getEmax_const(self.as_raw_KNearest()) }.into_result()
    }
    
    // identifier: cv_ml_KNearest_setEmax_int_val
    /// @copybrief getEmax @see getEmax
    fn set_emax(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_KNearest_setEmax_int_val(self.as_raw_KNearest(), val) }.into_result()
    }
    
    // identifier: cv_ml_KNearest_getAlgorithmType_const
    /// @see setAlgorithmType
    fn get_algorithm_type(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_KNearest_getAlgorithmType_const(self.as_raw_KNearest()) }.into_result()
    }
    
    // identifier: cv_ml_KNearest_setAlgorithmType_int_val
    /// @copybrief getAlgorithmType @see getAlgorithmType
    fn set_algorithm_type(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_KNearest_setAlgorithmType_int_val(self.as_raw_KNearest(), val) }.into_result()
    }
    
    // identifier: cv_ml_KNearest_findNearest_const_Mat_samples_int_k_Mat_results_Mat_neighborResponses_Mat_dist
    /// Finds the neighbors and predicts responses for input vectors.
    /// 
    /// ## Parameters
    /// * samples: Input samples stored by rows. It is a single-precision floating-point matrix of
    /// `<number_of_samples> * k` size.
    /// * k: Number of used nearest neighbors. Should be greater than 1.
    /// * results: Vector with results of prediction (regression or classification) for each input
    /// sample. It is a single-precision floating-point vector with `<number_of_samples>` elements.
    /// * neighborResponses: Optional output values for corresponding neighbors. It is a single-
    /// precision floating-point matrix of `<number_of_samples> * k` size.
    /// * dist: Optional output distances from the input vectors to the corresponding neighbors. It
    /// is a single-precision floating-point matrix of `<number_of_samples> * k` size.
    /// 
    /// For each input vector (a row of the matrix samples), the method finds the k nearest neighbors.
    /// In case of regression, the predicted result is a mean value of the particular vector's neighbor
    /// responses. In case of classification, the class is determined by voting.
    /// 
    /// For each input vector, the neighbors are sorted by their distances to the vector.
    /// 
    /// In case of C++ interface you can use output pointers to empty matrices and the function will
    /// allocate memory itself.
    /// 
    /// If only a single input vector is passed, all output matrices are optional and the predicted
    /// value is returned by the method.
    /// 
    /// The function is parallelized with the TBB library.
    ///
    /// ## C++ default parameters:
    /// * neighbor_responses: noArray()
    /// * dist: noArray()
    fn find_nearest(&self, samples: &core::Mat, k: i32, results: &mut core::Mat, neighbor_responses: &mut core::Mat, dist: &mut core::Mat) -> Result<f32> {
        unsafe { sys::cv_ml_cv_ml_KNearest_findNearest_const_Mat_samples_int_k_Mat_results_Mat_neighborResponses_Mat_dist(self.as_raw_KNearest(), samples.as_raw_Mat(), k, results.as_raw_Mat(), neighbor_responses.as_raw_Mat(), dist.as_raw_Mat()) }.into_result()
    }
    
}

impl<'a> KNearest + 'a {

    // identifier: cv_ml_KNearest_create
    /// Creates the empty model
    /// 
    /// The static method creates empty %KNearest classifier. It should be then trained using StatModel::train method.
    pub fn create() -> Result<types::PtrOfKNearest> {
        unsafe { sys::cv_ml_cv_ml_KNearest_create() }.into_result().map(|x| types::PtrOfKNearest { ptr: x })
    }
    
}

// Generating impl for trait cv::ml::LogisticRegression (trait)
/// Implements Logistic Regression classifier.
/// 
/// ## See also
/// @ref ml_intro_lr
pub trait LogisticRegression : crate::ml::StatModel {
    #[doc(hidden)] fn as_raw_LogisticRegression(&self) -> *mut c_void;
    // identifier: cv_ml_LogisticRegression_getLearningRate_const
    /// @see setLearningRate
    fn get_learning_rate(&self) -> Result<f64> {
        unsafe { sys::cv_ml_cv_ml_LogisticRegression_getLearningRate_const(self.as_raw_LogisticRegression()) }.into_result()
    }
    
    // identifier: cv_ml_LogisticRegression_setLearningRate_double_val
    /// @copybrief getLearningRate @see getLearningRate
    fn set_learning_rate(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_LogisticRegression_setLearningRate_double_val(self.as_raw_LogisticRegression(), val) }.into_result()
    }
    
    // identifier: cv_ml_LogisticRegression_getIterations_const
    /// @see setIterations
    fn get_iterations(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_LogisticRegression_getIterations_const(self.as_raw_LogisticRegression()) }.into_result()
    }
    
    // identifier: cv_ml_LogisticRegression_setIterations_int_val
    /// @copybrief getIterations @see getIterations
    fn set_iterations(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_LogisticRegression_setIterations_int_val(self.as_raw_LogisticRegression(), val) }.into_result()
    }
    
    // identifier: cv_ml_LogisticRegression_getRegularization_const
    /// @see setRegularization
    fn get_regularization(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_LogisticRegression_getRegularization_const(self.as_raw_LogisticRegression()) }.into_result()
    }
    
    // identifier: cv_ml_LogisticRegression_setRegularization_int_val
    /// @copybrief getRegularization @see getRegularization
    fn set_regularization(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_LogisticRegression_setRegularization_int_val(self.as_raw_LogisticRegression(), val) }.into_result()
    }
    
    // identifier: cv_ml_LogisticRegression_getTrainMethod_const
    /// @see setTrainMethod
    fn get_train_method(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_LogisticRegression_getTrainMethod_const(self.as_raw_LogisticRegression()) }.into_result()
    }
    
    // identifier: cv_ml_LogisticRegression_setTrainMethod_int_val
    /// @copybrief getTrainMethod @see getTrainMethod
    fn set_train_method(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_LogisticRegression_setTrainMethod_int_val(self.as_raw_LogisticRegression(), val) }.into_result()
    }
    
    // identifier: cv_ml_LogisticRegression_getMiniBatchSize_const
    /// @see setMiniBatchSize
    fn get_mini_batch_size(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_LogisticRegression_getMiniBatchSize_const(self.as_raw_LogisticRegression()) }.into_result()
    }
    
    // identifier: cv_ml_LogisticRegression_setMiniBatchSize_int_val
    /// @copybrief getMiniBatchSize @see getMiniBatchSize
    fn set_mini_batch_size(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_LogisticRegression_setMiniBatchSize_int_val(self.as_raw_LogisticRegression(), val) }.into_result()
    }
    
    // identifier: cv_ml_LogisticRegression_getTermCriteria_const
    /// @see setTermCriteria
    fn get_term_criteria(&self) -> Result<core::TermCriteria> {
        unsafe { sys::cv_ml_cv_ml_LogisticRegression_getTermCriteria_const(self.as_raw_LogisticRegression()) }.into_result().map(|x| core::TermCriteria { ptr: x })
    }
    
    // identifier: cv_ml_LogisticRegression_setTermCriteria_TermCriteria_val
    /// @copybrief getTermCriteria @see getTermCriteria
    fn set_term_criteria(&mut self, val: &core::TermCriteria) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_LogisticRegression_setTermCriteria_TermCriteria_val(self.as_raw_LogisticRegression(), val.as_raw_TermCriteria()) }.into_result()
    }
    
    // identifier: cv_ml_LogisticRegression_predict_const_Mat_samples_Mat_results_int_flags
    /// Predicts responses for input samples and returns a float type.
    /// 
    /// ## Parameters
    /// * samples: The input data for the prediction algorithm. Matrix [m x n], where each row
    /// contains variables (features) of one object being classified. Should have data type CV_32F.
    /// * results: Predicted labels as a column matrix of type CV_32S.
    /// * flags: Not used.
    ///
    /// ## C++ default parameters:
    /// * results: noArray()
    /// * flags: 0
    fn predict(&self, samples: &core::Mat, results: &mut core::Mat, flags: i32) -> Result<f32> {
        unsafe { sys::cv_ml_cv_ml_LogisticRegression_predict_const_Mat_samples_Mat_results_int_flags(self.as_raw_LogisticRegression(), samples.as_raw_Mat(), results.as_raw_Mat(), flags) }.into_result()
    }
    
    // identifier: cv_ml_LogisticRegression_get_learnt_thetas_const
    /// This function returns the trained parameters arranged across rows.
    /// 
    /// For a two class classifcation problem, it returns a row matrix. It returns learnt parameters of
    /// the Logistic Regression as a matrix of type CV_32F.
    fn get_learnt_thetas(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_LogisticRegression_get_learnt_thetas_const(self.as_raw_LogisticRegression()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
}

impl<'a> LogisticRegression + 'a {

    // identifier: cv_ml_LogisticRegression_create
    /// Creates empty model.
    /// 
    /// Creates Logistic Regression model with parameters given.
    pub fn create() -> Result<types::PtrOfLogisticRegression> {
        unsafe { sys::cv_ml_cv_ml_LogisticRegression_create() }.into_result().map(|x| types::PtrOfLogisticRegression { ptr: x })
    }
    
    // identifier: cv_ml_LogisticRegression_load_String_filepath_String_nodeName
    /// Loads and creates a serialized LogisticRegression from a file
    /// 
    /// Use LogisticRegression::save to serialize and store an LogisticRegression to disk.
    /// Load the LogisticRegression from this file again, by calling this function with the path to the file.
    /// Optionally specify the node for the file containing the classifier
    /// 
    /// ## Parameters
    /// * filepath: path to serialized LogisticRegression
    /// * nodeName: name of node containing the classifier
    ///
    /// ## C++ default parameters:
    /// * node_name: String()
    pub fn load(filepath: &str, node_name: &str) -> Result<types::PtrOfLogisticRegression> {
        string_arg!(filepath);
        string_arg!(node_name);
        unsafe { sys::cv_ml_cv_ml_LogisticRegression_load_String_filepath_String_nodeName(filepath.as_ptr(), node_name.as_ptr()) }.into_result().map(|x| types::PtrOfLogisticRegression { ptr: x })
    }
    
}

// Generating impl for trait cv::ml::NormalBayesClassifier (trait)
/// Bayes classifier for normally distributed data.
/// 
/// ## See also
/// @ref ml_intro_bayes
pub trait NormalBayesClassifier : crate::ml::StatModel {
    #[doc(hidden)] fn as_raw_NormalBayesClassifier(&self) -> *mut c_void;
    // identifier: cv_ml_NormalBayesClassifier_predictProb_const_Mat_inputs_Mat_outputs_Mat_outputProbs_int_flags
    /// Predicts the response for sample(s).
    /// 
    /// The method estimates the most probable classes for input vectors. Input vectors (one or more)
    /// are stored as rows of the matrix inputs. In case of multiple input vectors, there should be one
    /// output vector outputs. The predicted class for a single input vector is returned by the method.
    /// The vector outputProbs contains the output probabilities corresponding to each element of
    /// result.
    ///
    /// ## C++ default parameters:
    /// * flags: 0
    fn predict_prob(&self, inputs: &core::Mat, outputs: &mut core::Mat, output_probs: &mut core::Mat, flags: i32) -> Result<f32> {
        unsafe { sys::cv_ml_cv_ml_NormalBayesClassifier_predictProb_const_Mat_inputs_Mat_outputs_Mat_outputProbs_int_flags(self.as_raw_NormalBayesClassifier(), inputs.as_raw_Mat(), outputs.as_raw_Mat(), output_probs.as_raw_Mat(), flags) }.into_result()
    }
    
}

impl<'a> NormalBayesClassifier + 'a {

    // identifier: cv_ml_NormalBayesClassifier_create
    /// Creates empty model
    /// Use StatModel::train to train the model after creation.
    pub fn create() -> Result<types::PtrOfNormalBayesClassifier> {
        unsafe { sys::cv_ml_cv_ml_NormalBayesClassifier_create() }.into_result().map(|x| types::PtrOfNormalBayesClassifier { ptr: x })
    }
    
    // identifier: cv_ml_NormalBayesClassifier_load_String_filepath_String_nodeName
    /// Loads and creates a serialized NormalBayesClassifier from a file
    /// 
    /// Use NormalBayesClassifier::save to serialize and store an NormalBayesClassifier to disk.
    /// Load the NormalBayesClassifier from this file again, by calling this function with the path to the file.
    /// Optionally specify the node for the file containing the classifier
    /// 
    /// ## Parameters
    /// * filepath: path to serialized NormalBayesClassifier
    /// * nodeName: name of node containing the classifier
    ///
    /// ## C++ default parameters:
    /// * node_name: String()
    pub fn load(filepath: &str, node_name: &str) -> Result<types::PtrOfNormalBayesClassifier> {
        string_arg!(filepath);
        string_arg!(node_name);
        unsafe { sys::cv_ml_cv_ml_NormalBayesClassifier_load_String_filepath_String_nodeName(filepath.as_ptr(), node_name.as_ptr()) }.into_result().map(|x| types::PtrOfNormalBayesClassifier { ptr: x })
    }
    
}

// boxed class cv::ml::ParamGrid
/// The structure represents the logarithmic grid range of statmodel parameters.
/// 
/// It is used for optimizing statmodel accuracy by varying model parameters, the accuracy estimate
/// being computed by cross-validation.
#[allow(dead_code)]
pub struct ParamGrid {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for crate::ml::ParamGrid {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_ParamGrid(self.ptr) };
    }
}
impl crate::ml::ParamGrid {
    #[doc(hidden)] pub fn as_raw_ParamGrid(&self) -> *mut c_void { self.ptr }
}

impl ParamGrid {

    // identifier: cv_ml_ParamGrid_ParamGrid
    /// Default constructor
    pub fn new() -> Result<crate::ml::ParamGrid> {
        unsafe { sys::cv_ml_cv_ml_ParamGrid_ParamGrid() }.into_result().map(|x| crate::ml::ParamGrid { ptr: x })
    }
    
    // identifier: cv_ml_ParamGrid_ParamGrid_double__minVal_double__maxVal_double__logStep
    /// Constructor with parameters
    pub fn for_range(_min_val: f64, _max_val: f64, _log_step: f64) -> Result<crate::ml::ParamGrid> {
        unsafe { sys::cv_ml_cv_ml_ParamGrid_ParamGrid_double__minVal_double__maxVal_double__logStep(_min_val, _max_val, _log_step) }.into_result().map(|x| crate::ml::ParamGrid { ptr: x })
    }
    
    // identifier: cv_ml_ParamGrid_create_double_minVal_double_maxVal_double_logstep
    /// Creates a ParamGrid Ptr that can be given to the %SVM::trainAuto method
    /// 
    /// ## Parameters
    /// * minVal: minimum value of the parameter grid
    /// * maxVal: maximum value of the parameter grid
    /// * logstep: Logarithmic step for iterating the statmodel parameter
    ///
    /// ## C++ default parameters:
    /// * min_val: 0.
    /// * max_val: 0.
    /// * logstep: 1.
    pub fn create(min_val: f64, max_val: f64, logstep: f64) -> Result<types::PtrOfParamGrid> {
        unsafe { sys::cv_ml_cv_ml_ParamGrid_create_double_minVal_double_maxVal_double_logstep(min_val, max_val, logstep) }.into_result().map(|x| types::PtrOfParamGrid { ptr: x })
    }
    
}

// Generating impl for trait cv::ml::RTrees (trait)
/// The class implements the random forest predictor.
/// 
/// ## See also
/// @ref ml_intro_rtrees
pub trait RTrees : crate::ml::DTrees {
    #[doc(hidden)] fn as_raw_RTrees(&self) -> *mut c_void;
    // identifier: cv_ml_RTrees_getCalculateVarImportance_const
    /// @see setCalculateVarImportance
    fn get_calculate_var_importance(&self) -> Result<bool> {
        unsafe { sys::cv_ml_cv_ml_RTrees_getCalculateVarImportance_const(self.as_raw_RTrees()) }.into_result()
    }
    
    // identifier: cv_ml_RTrees_setCalculateVarImportance_bool_val
    /// @copybrief getCalculateVarImportance @see getCalculateVarImportance
    fn set_calculate_var_importance(&mut self, val: bool) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_RTrees_setCalculateVarImportance_bool_val(self.as_raw_RTrees(), val) }.into_result()
    }
    
    // identifier: cv_ml_RTrees_getActiveVarCount_const
    /// @see setActiveVarCount
    fn get_active_var_count(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_RTrees_getActiveVarCount_const(self.as_raw_RTrees()) }.into_result()
    }
    
    // identifier: cv_ml_RTrees_setActiveVarCount_int_val
    /// @copybrief getActiveVarCount @see getActiveVarCount
    fn set_active_var_count(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_RTrees_setActiveVarCount_int_val(self.as_raw_RTrees(), val) }.into_result()
    }
    
    // identifier: cv_ml_RTrees_getTermCriteria_const
    /// @see setTermCriteria
    fn get_term_criteria(&self) -> Result<core::TermCriteria> {
        unsafe { sys::cv_ml_cv_ml_RTrees_getTermCriteria_const(self.as_raw_RTrees()) }.into_result().map(|x| core::TermCriteria { ptr: x })
    }
    
    // identifier: cv_ml_RTrees_setTermCriteria_TermCriteria_val
    /// @copybrief getTermCriteria @see getTermCriteria
    fn set_term_criteria(&mut self, val: &core::TermCriteria) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_RTrees_setTermCriteria_TermCriteria_val(self.as_raw_RTrees(), val.as_raw_TermCriteria()) }.into_result()
    }
    
    // identifier: cv_ml_RTrees_getVarImportance_const
    /// Returns the variable importance array.
    /// The method returns the variable importance vector, computed at the training stage when
    /// CalculateVarImportance is set to true. If this flag was set to false, the empty matrix is
    /// returned.
    fn get_var_importance(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_RTrees_getVarImportance_const(self.as_raw_RTrees()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_RTrees_getVotes_const_Mat_samples_Mat_results_int_flags
    /// Returns the result of each individual tree in the forest.
    /// In case the model is a regression problem, the method will return each of the trees'
    /// results for each of the sample cases. If the model is a classifier, it will return
    /// a Mat with samples + 1 rows, where the first row gives the class number and the
    /// following rows return the votes each class had for each sample.
    /// ## Parameters
    /// * samples: Array containing the samples for which votes will be calculated.
    /// * results: Array where the result of the calculation will be written.
    /// * flags: Flags for defining the type of RTrees.
    fn get_votes(&self, samples: &core::Mat, results: &mut core::Mat, flags: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_RTrees_getVotes_const_Mat_samples_Mat_results_int_flags(self.as_raw_RTrees(), samples.as_raw_Mat(), results.as_raw_Mat(), flags) }.into_result()
    }
    
}

impl<'a> RTrees + 'a {

    // identifier: cv_ml_RTrees_create
    /// Creates the empty model.
    /// Use StatModel::train to train the model, StatModel::train to create and train the model,
    /// Algorithm::load to load the pre-trained model.
    pub fn create() -> Result<types::PtrOfRTrees> {
        unsafe { sys::cv_ml_cv_ml_RTrees_create() }.into_result().map(|x| types::PtrOfRTrees { ptr: x })
    }
    
    // identifier: cv_ml_RTrees_load_String_filepath_String_nodeName
    /// Loads and creates a serialized RTree from a file
    /// 
    /// Use RTree::save to serialize and store an RTree to disk.
    /// Load the RTree from this file again, by calling this function with the path to the file.
    /// Optionally specify the node for the file containing the classifier
    /// 
    /// ## Parameters
    /// * filepath: path to serialized RTree
    /// * nodeName: name of node containing the classifier
    ///
    /// ## C++ default parameters:
    /// * node_name: String()
    pub fn load(filepath: &str, node_name: &str) -> Result<types::PtrOfRTrees> {
        string_arg!(filepath);
        string_arg!(node_name);
        unsafe { sys::cv_ml_cv_ml_RTrees_load_String_filepath_String_nodeName(filepath.as_ptr(), node_name.as_ptr()) }.into_result().map(|x| types::PtrOfRTrees { ptr: x })
    }
    
}

// Generating impl for trait cv::ml::SVM (trait)
/// Support Vector Machines.
/// 
/// ## See also
/// @ref ml_intro_svm
pub trait SVM : crate::ml::StatModel {
    #[doc(hidden)] fn as_raw_SVM(&self) -> *mut c_void;
    // identifier: cv_ml_SVM_getType_const
    /// @see setType
    fn get_type(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_SVM_getType_const(self.as_raw_SVM()) }.into_result()
    }
    
    // identifier: cv_ml_SVM_setType_int_val
    /// @copybrief getType @see getType
    fn set_type(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_SVM_setType_int_val(self.as_raw_SVM(), val) }.into_result()
    }
    
    // identifier: cv_ml_SVM_getGamma_const
    /// @see setGamma
    fn get_gamma(&self) -> Result<f64> {
        unsafe { sys::cv_ml_cv_ml_SVM_getGamma_const(self.as_raw_SVM()) }.into_result()
    }
    
    // identifier: cv_ml_SVM_setGamma_double_val
    /// @copybrief getGamma @see getGamma
    fn set_gamma(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_SVM_setGamma_double_val(self.as_raw_SVM(), val) }.into_result()
    }
    
    // identifier: cv_ml_SVM_getCoef0_const
    /// @see setCoef0
    fn get_coef0(&self) -> Result<f64> {
        unsafe { sys::cv_ml_cv_ml_SVM_getCoef0_const(self.as_raw_SVM()) }.into_result()
    }
    
    // identifier: cv_ml_SVM_setCoef0_double_val
    /// @copybrief getCoef0 @see getCoef0
    fn set_coef0(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_SVM_setCoef0_double_val(self.as_raw_SVM(), val) }.into_result()
    }
    
    // identifier: cv_ml_SVM_getDegree_const
    /// @see setDegree
    fn get_degree(&self) -> Result<f64> {
        unsafe { sys::cv_ml_cv_ml_SVM_getDegree_const(self.as_raw_SVM()) }.into_result()
    }
    
    // identifier: cv_ml_SVM_setDegree_double_val
    /// @copybrief getDegree @see getDegree
    fn set_degree(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_SVM_setDegree_double_val(self.as_raw_SVM(), val) }.into_result()
    }
    
    // identifier: cv_ml_SVM_getC_const
    /// @see setC
    fn get_c(&self) -> Result<f64> {
        unsafe { sys::cv_ml_cv_ml_SVM_getC_const(self.as_raw_SVM()) }.into_result()
    }
    
    // identifier: cv_ml_SVM_setC_double_val
    /// @copybrief getC @see getC
    fn set_c(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_SVM_setC_double_val(self.as_raw_SVM(), val) }.into_result()
    }
    
    // identifier: cv_ml_SVM_getNu_const
    /// @see setNu
    fn get_nu(&self) -> Result<f64> {
        unsafe { sys::cv_ml_cv_ml_SVM_getNu_const(self.as_raw_SVM()) }.into_result()
    }
    
    // identifier: cv_ml_SVM_setNu_double_val
    /// @copybrief getNu @see getNu
    fn set_nu(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_SVM_setNu_double_val(self.as_raw_SVM(), val) }.into_result()
    }
    
    // identifier: cv_ml_SVM_getP_const
    /// @see setP
    fn get_p(&self) -> Result<f64> {
        unsafe { sys::cv_ml_cv_ml_SVM_getP_const(self.as_raw_SVM()) }.into_result()
    }
    
    // identifier: cv_ml_SVM_setP_double_val
    /// @copybrief getP @see getP
    fn set_p(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_SVM_setP_double_val(self.as_raw_SVM(), val) }.into_result()
    }
    
    // identifier: cv_ml_SVM_getClassWeights_const
    /// @see setClassWeights
    fn get_class_weights(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_SVM_getClassWeights_const(self.as_raw_SVM()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_SVM_setClassWeights_Mat_val
    /// @copybrief getClassWeights @see getClassWeights
    fn set_class_weights(&mut self, val: &core::Mat) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_SVM_setClassWeights_Mat_val(self.as_raw_SVM(), val.as_raw_Mat()) }.into_result()
    }
    
    // identifier: cv_ml_SVM_getTermCriteria_const
    /// @see setTermCriteria
    fn get_term_criteria(&self) -> Result<core::TermCriteria> {
        unsafe { sys::cv_ml_cv_ml_SVM_getTermCriteria_const(self.as_raw_SVM()) }.into_result().map(|x| core::TermCriteria { ptr: x })
    }
    
    // identifier: cv_ml_SVM_setTermCriteria_TermCriteria_val
    /// @copybrief getTermCriteria @see getTermCriteria
    fn set_term_criteria(&mut self, val: &core::TermCriteria) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_SVM_setTermCriteria_TermCriteria_val(self.as_raw_SVM(), val.as_raw_TermCriteria()) }.into_result()
    }
    
    // identifier: cv_ml_SVM_getKernelType_const
    /// Type of a %SVM kernel.
    /// See SVM::KernelTypes. Default value is SVM::RBF.
    fn get_kernel_type(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_SVM_getKernelType_const(self.as_raw_SVM()) }.into_result()
    }
    
    // identifier: cv_ml_SVM_setKernel_int_kernelType
    /// Initialize with one of predefined kernels.
    /// See SVM::KernelTypes.
    fn set_kernel(&mut self, kernel_type: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_SVM_setKernel_int_kernelType(self.as_raw_SVM(), kernel_type) }.into_result()
    }
    
    // identifier: cv_ml_SVM_setCustomKernel_PtrOfKernel__kernel
    /// Initialize with custom kernel.
    /// See SVM::Kernel class for implementation details
    fn set_custom_kernel(&mut self, _kernel: &types::PtrOfKernel) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_SVM_setCustomKernel_PtrOfKernel__kernel(self.as_raw_SVM(), _kernel.as_raw_PtrOfKernel()) }.into_result()
    }
    
    // identifier: cv_ml_SVM_trainAuto_PtrOfTrainData_data_int_kFold_ParamGrid_Cgrid_ParamGrid_gammaGrid_ParamGrid_pGrid_ParamGrid_nuGrid_ParamGrid_coeffGrid_ParamGrid_degreeGrid_bool_balanced
    /// Trains an %SVM with optimal parameters.
    /// 
    /// ## Parameters
    /// * data: the training data that can be constructed using TrainData::create or
    /// TrainData::loadFromCSV.
    /// * kFold: Cross-validation parameter. The training set is divided into kFold subsets. One
    /// subset is used to test the model, the others form the train set. So, the %SVM algorithm is
    /// executed kFold times.
    /// * Cgrid: grid for C
    /// * gammaGrid: grid for gamma
    /// * pGrid: grid for p
    /// * nuGrid: grid for nu
    /// * coeffGrid: grid for coeff
    /// * degreeGrid: grid for degree
    /// * balanced: If true and the problem is 2-class classification then the method creates more
    /// balanced cross-validation subsets that is proportions between classes in subsets are close
    /// to such proportion in the whole train dataset.
    /// 
    /// The method trains the %SVM model automatically by choosing the optimal parameters C, gamma, p,
    /// nu, coef0, degree. Parameters are considered optimal when the cross-validation
    /// estimate of the test set error is minimal.
    /// 
    /// If there is no need to optimize a parameter, the corresponding grid step should be set to any
    /// value less than or equal to 1. For example, to avoid optimization in gamma, set `gammaGrid.step
    /// = 0`, `gammaGrid.minVal`, `gamma_grid.maxVal` as arbitrary numbers. In this case, the value
    /// `Gamma` is taken for gamma.
    /// 
    /// And, finally, if the optimization in a parameter is required but the corresponding grid is
    /// unknown, you may call the function SVM::getDefaultGrid. To generate a grid, for example, for
    /// gamma, call `SVM::getDefaultGrid(SVM::GAMMA)`.
    /// 
    /// This function works for the classification (SVM::C_SVC or SVM::NU_SVC) as well as for the
    /// regression (SVM::EPS_SVR or SVM::NU_SVR). If it is SVM::ONE_CLASS, no optimization is made and
    /// the usual %SVM with parameters specified in params is executed.
    ///
    /// ## C++ default parameters:
    /// * k_fold: 10
    /// * cgrid: getDefaultGrid(C)
    /// * gamma_grid: getDefaultGrid(GAMMA)
    /// * p_grid: getDefaultGrid(P)
    /// * nu_grid: getDefaultGrid(NU)
    /// * coeff_grid: getDefaultGrid(COEF)
    /// * degree_grid: getDefaultGrid(DEGREE)
    /// * balanced: false
    fn train_auto(&mut self, data: &types::PtrOfTrainData, k_fold: i32, cgrid: &crate::ml::ParamGrid, gamma_grid: &crate::ml::ParamGrid, p_grid: &crate::ml::ParamGrid, nu_grid: &crate::ml::ParamGrid, coeff_grid: &crate::ml::ParamGrid, degree_grid: &crate::ml::ParamGrid, balanced: bool) -> Result<bool> {
        unsafe { sys::cv_ml_cv_ml_SVM_trainAuto_PtrOfTrainData_data_int_kFold_ParamGrid_Cgrid_ParamGrid_gammaGrid_ParamGrid_pGrid_ParamGrid_nuGrid_ParamGrid_coeffGrid_ParamGrid_degreeGrid_bool_balanced(self.as_raw_SVM(), data.as_raw_PtrOfTrainData(), k_fold, cgrid.as_raw_ParamGrid(), gamma_grid.as_raw_ParamGrid(), p_grid.as_raw_ParamGrid(), nu_grid.as_raw_ParamGrid(), coeff_grid.as_raw_ParamGrid(), degree_grid.as_raw_ParamGrid(), balanced) }.into_result()
    }
    
    // identifier: cv_ml_SVM_trainAuto_Mat_samples_int_layout_Mat_responses_int_kFold_PtrOfParamGrid_Cgrid_PtrOfParamGrid_gammaGrid_PtrOfParamGrid_pGrid_PtrOfParamGrid_nuGrid_PtrOfParamGrid_coeffGrid_PtrOfParamGrid_degreeGrid_bool_balanced
    /// Trains an %SVM with optimal parameters
    /// 
    /// ## Parameters
    /// * samples: training samples
    /// * layout: See ml::SampleTypes.
    /// * responses: vector of responses associated with the training samples.
    /// * kFold: Cross-validation parameter. The training set is divided into kFold subsets. One
    /// subset is used to test the model, the others form the train set. So, the %SVM algorithm is
    /// * Cgrid: grid for C
    /// * gammaGrid: grid for gamma
    /// * pGrid: grid for p
    /// * nuGrid: grid for nu
    /// * coeffGrid: grid for coeff
    /// * degreeGrid: grid for degree
    /// * balanced: If true and the problem is 2-class classification then the method creates more
    /// balanced cross-validation subsets that is proportions between classes in subsets are close
    /// to such proportion in the whole train dataset.
    /// 
    /// The method trains the %SVM model automatically by choosing the optimal parameters C, gamma, p,
    /// nu, coef0, degree. Parameters are considered optimal when the cross-validation
    /// estimate of the test set error is minimal.
    /// 
    /// This function only makes use of SVM::getDefaultGrid for parameter optimization and thus only
    /// offers rudimentary parameter options.
    /// 
    /// This function works for the classification (SVM::C_SVC or SVM::NU_SVC) as well as for the
    /// regression (SVM::EPS_SVR or SVM::NU_SVR). If it is SVM::ONE_CLASS, no optimization is made and
    /// the usual %SVM with parameters specified in params is executed.
    ///
    /// ## C++ default parameters:
    /// * k_fold: 10
    /// * cgrid: SVM::getDefaultGridPtr(SVM::C)
    /// * gamma_grid: SVM::getDefaultGridPtr(SVM::GAMMA)
    /// * p_grid: SVM::getDefaultGridPtr(SVM::P)
    /// * nu_grid: SVM::getDefaultGridPtr(SVM::NU)
    /// * coeff_grid: SVM::getDefaultGridPtr(SVM::COEF)
    /// * degree_grid: SVM::getDefaultGridPtr(SVM::DEGREE)
    /// * balanced: false
    fn train_auto_1(&mut self, samples: &core::Mat, layout: i32, responses: &core::Mat, k_fold: i32, cgrid: &types::PtrOfParamGrid, gamma_grid: &types::PtrOfParamGrid, p_grid: &types::PtrOfParamGrid, nu_grid: &types::PtrOfParamGrid, coeff_grid: &types::PtrOfParamGrid, degree_grid: &types::PtrOfParamGrid, balanced: bool) -> Result<bool> {
        unsafe { sys::cv_ml_cv_ml_SVM_trainAuto_Mat_samples_int_layout_Mat_responses_int_kFold_PtrOfParamGrid_Cgrid_PtrOfParamGrid_gammaGrid_PtrOfParamGrid_pGrid_PtrOfParamGrid_nuGrid_PtrOfParamGrid_coeffGrid_PtrOfParamGrid_degreeGrid_bool_balanced(self.as_raw_SVM(), samples.as_raw_Mat(), layout, responses.as_raw_Mat(), k_fold, cgrid.as_raw_PtrOfParamGrid(), gamma_grid.as_raw_PtrOfParamGrid(), p_grid.as_raw_PtrOfParamGrid(), nu_grid.as_raw_PtrOfParamGrid(), coeff_grid.as_raw_PtrOfParamGrid(), degree_grid.as_raw_PtrOfParamGrid(), balanced) }.into_result()
    }
    
    // identifier: cv_ml_SVM_getSupportVectors_const
    /// Retrieves all the support vectors
    /// 
    /// The method returns all the support vectors as a floating-point matrix, where support vectors are
    /// stored as matrix rows.
    fn get_support_vectors(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_SVM_getSupportVectors_const(self.as_raw_SVM()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_SVM_getUncompressedSupportVectors_const
    /// Retrieves all the uncompressed support vectors of a linear %SVM
    /// 
    /// The method returns all the uncompressed support vectors of a linear %SVM that the compressed
    /// support vector, used for prediction, was derived from. They are returned in a floating-point
    /// matrix, where the support vectors are stored as matrix rows.
    fn get_uncompressed_support_vectors(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_SVM_getUncompressedSupportVectors_const(self.as_raw_SVM()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_SVM_getDecisionFunction_const_int_i_Mat_alpha_Mat_svidx
    /// <script type="text/javascript" src="https://latex.codecogs.com/latexit.js"></script>
    ///  Retrieves the decision function
    /// 
    /// ## Parameters
    /// * i: the index of the decision function. If the problem solved is regression, 1-class or
    /// 2-class classification, then there will be just one decision function and the index should
    /// always be 0. Otherwise, in the case of N-class classification, there will be <span lang='latex'>N(N-1)/2</span>
    /// decision functions.
    /// * alpha: the optional output vector for weights, corresponding to different support vectors.
    /// In the case of linear %SVM all the alpha's will be 1's.
    /// * svidx: the optional output vector of indices of support vectors within the matrix of
    /// support vectors (which can be retrieved by SVM::getSupportVectors). In the case of linear
    /// %SVM each decision function consists of a single "compressed" support vector.
    /// 
    /// The method returns rho parameter of the decision function, a scalar subtracted from the weighted
    /// sum of kernel responses.
    fn get_decision_function(&self, i: i32, alpha: &mut core::Mat, svidx: &mut core::Mat) -> Result<f64> {
        unsafe { sys::cv_ml_cv_ml_SVM_getDecisionFunction_const_int_i_Mat_alpha_Mat_svidx(self.as_raw_SVM(), i, alpha.as_raw_Mat(), svidx.as_raw_Mat()) }.into_result()
    }
    
}

impl<'a> SVM + 'a {

    // identifier: cv_ml_SVM_getDefaultGrid_int_param_id
    /// Generates a grid for %SVM parameters.
    /// 
    /// ## Parameters
    /// * param_id: %SVM parameters IDs that must be one of the SVM::ParamTypes. The grid is
    /// generated for the parameter with this ID.
    /// 
    /// The function generates a grid for the specified parameter of the %SVM algorithm. The grid may be
    /// passed to the function SVM::trainAuto.
    pub fn get_default_grid(param_id: i32) -> Result<crate::ml::ParamGrid> {
        unsafe { sys::cv_ml_cv_ml_SVM_getDefaultGrid_int_param_id(param_id) }.into_result().map(|x| crate::ml::ParamGrid { ptr: x })
    }
    
    // identifier: cv_ml_SVM_getDefaultGridPtr_int_param_id
    /// Generates a grid for %SVM parameters.
    /// 
    /// ## Parameters
    /// * param_id: %SVM parameters IDs that must be one of the SVM::ParamTypes. The grid is
    /// generated for the parameter with this ID.
    /// 
    /// The function generates a grid pointer for the specified parameter of the %SVM algorithm.
    /// The grid may be passed to the function SVM::trainAuto.
    pub fn get_default_grid_ptr(param_id: i32) -> Result<types::PtrOfParamGrid> {
        unsafe { sys::cv_ml_cv_ml_SVM_getDefaultGridPtr_int_param_id(param_id) }.into_result().map(|x| types::PtrOfParamGrid { ptr: x })
    }
    
    // identifier: cv_ml_SVM_create
    /// Creates empty model.
    /// Use StatModel::train to train the model. Since %SVM has several parameters, you may want to
    /// find the best parameters for your problem, it can be done with SVM::trainAuto.
    pub fn create() -> Result<types::PtrOfSVM> {
        unsafe { sys::cv_ml_cv_ml_SVM_create() }.into_result().map(|x| types::PtrOfSVM { ptr: x })
    }
    
    // identifier: cv_ml_SVM_load_String_filepath
    /// Loads and creates a serialized svm from a file
    /// 
    /// Use SVM::save to serialize and store an SVM to disk.
    /// Load the SVM from this file again, by calling this function with the path to the file.
    /// 
    /// ## Parameters
    /// * filepath: path to serialized svm
    pub fn load(filepath: &str) -> Result<types::PtrOfSVM> {
        string_arg!(filepath);
        unsafe { sys::cv_ml_cv_ml_SVM_load_String_filepath(filepath.as_ptr()) }.into_result().map(|x| types::PtrOfSVM { ptr: x })
    }
    
}

// Generating impl for trait cv::ml::SVM::Kernel (trait)
pub trait SVM_Kernel : core::Algorithm {
    #[doc(hidden)] fn as_raw_SVM_Kernel(&self) -> *mut c_void;
    // identifier: cv_ml_SVM_Kernel_getType_const
    fn get_type(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_SVM_Kernel_getType_const(self.as_raw_SVM_Kernel()) }.into_result()
    }
    
    // identifier: cv_ml_SVM_Kernel_calc_int_vcount_int_n_const_float_X_vecs_const_float_X_another_float_X_results
    fn calc(&mut self, vcount: i32, n: i32, vecs: &f32, another: &f32, results: &mut f32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_SVM_Kernel_calc_int_vcount_int_n_const_float_X_vecs_const_float_X_another_float_X_results(self.as_raw_SVM_Kernel(), vcount, n, vecs, another, results) }.into_result()
    }
    
}

impl<'a> SVM_Kernel + 'a {

}

// Generating impl for trait cv::ml::SVMSGD (trait)
/// \
///                        Stochastic Gradient Descent SVM Classifier                      *
pub trait SVMSGD : crate::ml::StatModel {
    #[doc(hidden)] fn as_raw_SVMSGD(&self) -> *mut c_void;
    // identifier: cv_ml_SVMSGD_getWeights
    /// @return the weights of the trained model (decision function f(x) = weights * x + shift).
    fn get_weights(&mut self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_SVMSGD_getWeights(self.as_raw_SVMSGD()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_SVMSGD_getShift
    /// @return the shift of the trained model (decision function f(x) = weights * x + shift).
    fn get_shift(&mut self) -> Result<f32> {
        unsafe { sys::cv_ml_cv_ml_SVMSGD_getShift(self.as_raw_SVMSGD()) }.into_result()
    }
    
    // identifier: cv_ml_SVMSGD_setOptimalParameters_int_svmsgdType_int_marginType
    /// Function sets optimal parameters values for chosen SVM SGD model.
    /// ## Parameters
    /// * svmsgdType: is the type of SVMSGD classifier.
    /// * marginType: is the type of margin constraint.
    ///
    /// ## C++ default parameters:
    /// * svmsgd_type: SVMSGD::ASGD
    /// * margin_type: SVMSGD::SOFT_MARGIN
    fn set_optimal_parameters(&mut self, svmsgd_type: i32, margin_type: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_SVMSGD_setOptimalParameters_int_svmsgdType_int_marginType(self.as_raw_SVMSGD(), svmsgd_type, margin_type) }.into_result()
    }
    
    // identifier: cv_ml_SVMSGD_getSvmsgdType_const
    /// @see setSvmsgdType
    fn get_svmsgd_type(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_SVMSGD_getSvmsgdType_const(self.as_raw_SVMSGD()) }.into_result()
    }
    
    // identifier: cv_ml_SVMSGD_setSvmsgdType_int_svmsgdType
    /// @copybrief getSvmsgdType @see getSvmsgdType
    fn set_svmsgd_type(&mut self, svmsgd_type: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_SVMSGD_setSvmsgdType_int_svmsgdType(self.as_raw_SVMSGD(), svmsgd_type) }.into_result()
    }
    
    // identifier: cv_ml_SVMSGD_getMarginType_const
    /// @see setMarginType
    fn get_margin_type(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_SVMSGD_getMarginType_const(self.as_raw_SVMSGD()) }.into_result()
    }
    
    // identifier: cv_ml_SVMSGD_setMarginType_int_marginType
    /// @copybrief getMarginType @see getMarginType
    fn set_margin_type(&mut self, margin_type: i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_SVMSGD_setMarginType_int_marginType(self.as_raw_SVMSGD(), margin_type) }.into_result()
    }
    
    // identifier: cv_ml_SVMSGD_getMarginRegularization_const
    /// @see setMarginRegularization
    fn get_margin_regularization(&self) -> Result<f32> {
        unsafe { sys::cv_ml_cv_ml_SVMSGD_getMarginRegularization_const(self.as_raw_SVMSGD()) }.into_result()
    }
    
    // identifier: cv_ml_SVMSGD_setMarginRegularization_float_marginRegularization
    /// @copybrief getMarginRegularization @see getMarginRegularization
    fn set_margin_regularization(&mut self, margin_regularization: f32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_SVMSGD_setMarginRegularization_float_marginRegularization(self.as_raw_SVMSGD(), margin_regularization) }.into_result()
    }
    
    // identifier: cv_ml_SVMSGD_getInitialStepSize_const
    /// @see setInitialStepSize
    fn get_initial_step_size(&self) -> Result<f32> {
        unsafe { sys::cv_ml_cv_ml_SVMSGD_getInitialStepSize_const(self.as_raw_SVMSGD()) }.into_result()
    }
    
    // identifier: cv_ml_SVMSGD_setInitialStepSize_float_InitialStepSize
    /// @copybrief getInitialStepSize @see getInitialStepSize
    fn set_initial_step_size(&mut self, initial_step_size: f32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_SVMSGD_setInitialStepSize_float_InitialStepSize(self.as_raw_SVMSGD(), initial_step_size) }.into_result()
    }
    
    // identifier: cv_ml_SVMSGD_getStepDecreasingPower_const
    /// @see setStepDecreasingPower
    fn get_step_decreasing_power(&self) -> Result<f32> {
        unsafe { sys::cv_ml_cv_ml_SVMSGD_getStepDecreasingPower_const(self.as_raw_SVMSGD()) }.into_result()
    }
    
    // identifier: cv_ml_SVMSGD_setStepDecreasingPower_float_stepDecreasingPower
    /// @copybrief getStepDecreasingPower @see getStepDecreasingPower
    fn set_step_decreasing_power(&mut self, step_decreasing_power: f32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_SVMSGD_setStepDecreasingPower_float_stepDecreasingPower(self.as_raw_SVMSGD(), step_decreasing_power) }.into_result()
    }
    
    // identifier: cv_ml_SVMSGD_getTermCriteria_const
    /// @see setTermCriteria
    fn get_term_criteria(&self) -> Result<core::TermCriteria> {
        unsafe { sys::cv_ml_cv_ml_SVMSGD_getTermCriteria_const(self.as_raw_SVMSGD()) }.into_result().map(|x| core::TermCriteria { ptr: x })
    }
    
    // identifier: cv_ml_SVMSGD_setTermCriteria_TermCriteria_val
    /// @copybrief getTermCriteria @see getTermCriteria
    fn set_term_criteria(&mut self, val: &core::TermCriteria) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_SVMSGD_setTermCriteria_TermCriteria_val(self.as_raw_SVMSGD(), val.as_raw_TermCriteria()) }.into_result()
    }
    
}

impl<'a> SVMSGD + 'a {

    // identifier: cv_ml_SVMSGD_create
    /// Creates empty model.
    /// Use StatModel::train to train the model. Since %SVMSGD has several parameters, you may want to
    /// find the best parameters for your problem or use setOptimalParameters() to set some default parameters.
    pub fn create() -> Result<types::PtrOfSVMSGD> {
        unsafe { sys::cv_ml_cv_ml_SVMSGD_create() }.into_result().map(|x| types::PtrOfSVMSGD { ptr: x })
    }
    
    // identifier: cv_ml_SVMSGD_load_String_filepath_String_nodeName
    /// Loads and creates a serialized SVMSGD from a file
    /// 
    /// Use SVMSGD::save to serialize and store an SVMSGD to disk.
    /// Load the SVMSGD from this file again, by calling this function with the path to the file.
    /// Optionally specify the node for the file containing the classifier
    /// 
    /// ## Parameters
    /// * filepath: path to serialized SVMSGD
    /// * nodeName: name of node containing the classifier
    ///
    /// ## C++ default parameters:
    /// * node_name: String()
    pub fn load(filepath: &str, node_name: &str) -> Result<types::PtrOfSVMSGD> {
        string_arg!(filepath);
        string_arg!(node_name);
        unsafe { sys::cv_ml_cv_ml_SVMSGD_load_String_filepath_String_nodeName(filepath.as_ptr(), node_name.as_ptr()) }.into_result().map(|x| types::PtrOfSVMSGD { ptr: x })
    }
    
}

// Generating impl for trait cv::ml::StatModel (trait)
/// Base class for statistical models in OpenCV ML.
pub trait StatModel : core::Algorithm {
    #[doc(hidden)] fn as_raw_StatModel(&self) -> *mut c_void;
    // identifier: cv_ml_StatModel_getVarCount_const
    /// Returns the number of variables in training samples
    fn get_var_count(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_StatModel_getVarCount_const(self.as_raw_StatModel()) }.into_result()
    }
    
    // identifier: cv_ml_StatModel_empty_const
    fn empty(&self) -> Result<bool> {
        unsafe { sys::cv_ml_cv_ml_StatModel_empty_const(self.as_raw_StatModel()) }.into_result()
    }
    
    // identifier: cv_ml_StatModel_isTrained_const
    /// Returns true if the model is trained
    fn is_trained(&self) -> Result<bool> {
        unsafe { sys::cv_ml_cv_ml_StatModel_isTrained_const(self.as_raw_StatModel()) }.into_result()
    }
    
    // identifier: cv_ml_StatModel_isClassifier_const
    /// Returns true if the model is classifier
    fn is_classifier(&self) -> Result<bool> {
        unsafe { sys::cv_ml_cv_ml_StatModel_isClassifier_const(self.as_raw_StatModel()) }.into_result()
    }
    
    // identifier: cv_ml_StatModel_train_PtrOfTrainData_trainData_int_flags
    /// Trains the statistical model
    /// 
    /// ## Parameters
    /// * trainData: training data that can be loaded from file using TrainData::loadFromCSV or
    /// created with TrainData::create.
    /// * flags: optional flags, depending on the model. Some of the models can be updated with the
    /// new training samples, not completely overwritten (such as NormalBayesClassifier or ANN_MLP).
    ///
    /// ## C++ default parameters:
    /// * flags: 0
    fn train(&mut self, train_data: &types::PtrOfTrainData, flags: i32) -> Result<bool> {
        unsafe { sys::cv_ml_cv_ml_StatModel_train_PtrOfTrainData_trainData_int_flags(self.as_raw_StatModel(), train_data.as_raw_PtrOfTrainData(), flags) }.into_result()
    }
    
    // identifier: cv_ml_StatModel_train_Mat_samples_int_layout_Mat_responses
    /// Trains the statistical model
    /// 
    /// ## Parameters
    /// * samples: training samples
    /// * layout: See ml::SampleTypes.
    /// * responses: vector of responses associated with the training samples.
    fn train_1(&mut self, samples: &core::Mat, layout: i32, responses: &core::Mat) -> Result<bool> {
        unsafe { sys::cv_ml_cv_ml_StatModel_train_Mat_samples_int_layout_Mat_responses(self.as_raw_StatModel(), samples.as_raw_Mat(), layout, responses.as_raw_Mat()) }.into_result()
    }
    
    // identifier: cv_ml_StatModel_calcError_const_PtrOfTrainData_data_bool_test_Mat_resp
    /// Computes error on the training or test dataset
    /// 
    /// ## Parameters
    /// * data: the training data
    /// * test: if true, the error is computed over the test subset of the data, otherwise it's
    /// computed over the training subset of the data. Please note that if you loaded a completely
    /// different dataset to evaluate already trained classifier, you will probably want not to set
    /// the test subset at all with TrainData::setTrainTestSplitRatio and specify test=false, so
    /// that the error is computed for the whole new set. Yes, this sounds a bit confusing.
    /// * resp: the optional output responses.
    /// 
    /// The method uses StatModel::predict to compute the error. For regression models the error is
    /// computed as RMS, for classifiers - as a percent of missclassified samples (0%-100%).
    fn calc_error(&self, data: &types::PtrOfTrainData, test: bool, resp: &mut core::Mat) -> Result<f32> {
        unsafe { sys::cv_ml_cv_ml_StatModel_calcError_const_PtrOfTrainData_data_bool_test_Mat_resp(self.as_raw_StatModel(), data.as_raw_PtrOfTrainData(), test, resp.as_raw_Mat()) }.into_result()
    }
    
    // identifier: cv_ml_StatModel_predict_const_Mat_samples_Mat_results_int_flags
    /// Predicts response(s) for the provided sample(s)
    /// 
    /// ## Parameters
    /// * samples: The input samples, floating-point matrix
    /// * results: The optional output matrix of results.
    /// * flags: The optional flags, model-dependent. See cv::ml::StatModel::Flags.
    ///
    /// ## C++ default parameters:
    /// * results: noArray()
    /// * flags: 0
    fn predict(&self, samples: &core::Mat, results: &mut core::Mat, flags: i32) -> Result<f32> {
        unsafe { sys::cv_ml_cv_ml_StatModel_predict_const_Mat_samples_Mat_results_int_flags(self.as_raw_StatModel(), samples.as_raw_Mat(), results.as_raw_Mat(), flags) }.into_result()
    }
    
}

impl<'a> StatModel + 'a {

}

// Generating impl for trait cv::ml::TrainData (trait)
/// Class encapsulating training data.
/// 
/// Please note that the class only specifies the interface of training data, but not implementation.
/// All the statistical model classes in _ml_ module accepts Ptr\<TrainData\> as parameter. In other
/// words, you can create your own class derived from TrainData and pass smart pointer to the instance
/// of this class into StatModel::train.
/// 
/// ## See also
/// @ref ml_intro_data
pub trait TrainData {
    #[doc(hidden)] fn as_raw_TrainData(&self) -> *mut c_void;
    // identifier: cv_ml_TrainData_missingValue
    fn missing_value(&mut self) -> Result<f32> {
        unsafe { sys::cv_ml_cv_ml_TrainData_missingValue(self.as_raw_TrainData()) }.into_result()
    }
    
    // identifier: cv_ml_TrainData_getLayout_const
    fn get_layout(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getLayout_const(self.as_raw_TrainData()) }.into_result()
    }
    
    // identifier: cv_ml_TrainData_getNTrainSamples_const
    fn get_n_train_samples(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getNTrainSamples_const(self.as_raw_TrainData()) }.into_result()
    }
    
    // identifier: cv_ml_TrainData_getNTestSamples_const
    fn get_n_test_samples(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getNTestSamples_const(self.as_raw_TrainData()) }.into_result()
    }
    
    // identifier: cv_ml_TrainData_getNSamples_const
    fn get_n_samples(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getNSamples_const(self.as_raw_TrainData()) }.into_result()
    }
    
    // identifier: cv_ml_TrainData_getNVars_const
    fn get_n_vars(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getNVars_const(self.as_raw_TrainData()) }.into_result()
    }
    
    // identifier: cv_ml_TrainData_getNAllVars_const
    fn get_n_all_vars(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getNAllVars_const(self.as_raw_TrainData()) }.into_result()
    }
    
    // identifier: cv_ml_TrainData_getSample_const_Mat_varIdx_int_sidx_float_X_buf
    fn get_sample(&self, var_idx: &core::Mat, sidx: i32, buf: &mut f32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getSample_const_Mat_varIdx_int_sidx_float_X_buf(self.as_raw_TrainData(), var_idx.as_raw_Mat(), sidx, buf) }.into_result()
    }
    
    // identifier: cv_ml_TrainData_getSamples_const
    fn get_samples(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getSamples_const(self.as_raw_TrainData()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_getMissing_const
    fn get_missing(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getMissing_const(self.as_raw_TrainData()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_getTrainSamples_const_int_layout_bool_compressSamples_bool_compressVars
    /// Returns matrix of train samples
    /// 
    /// ## Parameters
    /// * layout: The requested layout. If it's different from the initial one, the matrix is
    /// transposed. See ml::SampleTypes.
    /// * compressSamples: if true, the function returns only the training samples (specified by
    /// sampleIdx)
    /// * compressVars: if true, the function returns the shorter training samples, containing only
    /// the active variables.
    /// 
    /// In current implementation the function tries to avoid physical data copying and returns the
    /// matrix stored inside TrainData (unless the transposition or compression is needed).
    ///
    /// ## C++ default parameters:
    /// * layout: ROW_SAMPLE
    /// * compress_samples: true
    /// * compress_vars: true
    fn get_train_samples(&self, layout: i32, compress_samples: bool, compress_vars: bool) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getTrainSamples_const_int_layout_bool_compressSamples_bool_compressVars(self.as_raw_TrainData(), layout, compress_samples, compress_vars) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_getTrainResponses_const
    /// Returns the vector of responses
    /// 
    /// The function returns ordered or the original categorical responses. Usually it's used in
    /// regression algorithms.
    fn get_train_responses(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getTrainResponses_const(self.as_raw_TrainData()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_getTrainNormCatResponses_const
    /// Returns the vector of normalized categorical responses
    /// 
    /// The function returns vector of responses. Each response is integer from `0` to `<number of
    /// classes>-1`. The actual label value can be retrieved then from the class label vector, see
    /// TrainData::getClassLabels.
    fn get_train_norm_cat_responses(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getTrainNormCatResponses_const(self.as_raw_TrainData()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_getTestResponses_const
    fn get_test_responses(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getTestResponses_const(self.as_raw_TrainData()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_getTestNormCatResponses_const
    fn get_test_norm_cat_responses(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getTestNormCatResponses_const(self.as_raw_TrainData()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_getResponses_const
    fn get_responses(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getResponses_const(self.as_raw_TrainData()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_getNormCatResponses_const
    fn get_norm_cat_responses(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getNormCatResponses_const(self.as_raw_TrainData()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_getSampleWeights_const
    fn get_sample_weights(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getSampleWeights_const(self.as_raw_TrainData()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_getTrainSampleWeights_const
    fn get_train_sample_weights(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getTrainSampleWeights_const(self.as_raw_TrainData()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_getTestSampleWeights_const
    fn get_test_sample_weights(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getTestSampleWeights_const(self.as_raw_TrainData()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_getVarIdx_const
    fn get_var_idx(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getVarIdx_const(self.as_raw_TrainData()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_getVarType_const
    fn get_var_type(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getVarType_const(self.as_raw_TrainData()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_getVarSymbolFlags_const
    fn get_var_symbol_flags(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getVarSymbolFlags_const(self.as_raw_TrainData()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_getResponseType_const
    fn get_response_type(&self) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getResponseType_const(self.as_raw_TrainData()) }.into_result()
    }
    
    // identifier: cv_ml_TrainData_getTrainSampleIdx_const
    fn get_train_sample_idx(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getTrainSampleIdx_const(self.as_raw_TrainData()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_getTestSampleIdx_const
    fn get_test_sample_idx(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getTestSampleIdx_const(self.as_raw_TrainData()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_getValues_const_int_vi_Mat_sidx_float_X_values
    fn get_values(&self, vi: i32, sidx: &core::Mat, values: &mut f32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getValues_const_int_vi_Mat_sidx_float_X_values(self.as_raw_TrainData(), vi, sidx.as_raw_Mat(), values) }.into_result()
    }
    
    // identifier: cv_ml_TrainData_getNormCatValues_const_int_vi_Mat_sidx_int_X_values
    fn get_norm_cat_values(&self, vi: i32, sidx: &core::Mat, values: &mut i32) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getNormCatValues_const_int_vi_Mat_sidx_int_X_values(self.as_raw_TrainData(), vi, sidx.as_raw_Mat(), values) }.into_result()
    }
    
    // identifier: cv_ml_TrainData_getDefaultSubstValues_const
    fn get_default_subst_values(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getDefaultSubstValues_const(self.as_raw_TrainData()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_getCatCount_const_int_vi
    fn get_cat_count(&self, vi: i32) -> Result<i32> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getCatCount_const_int_vi(self.as_raw_TrainData(), vi) }.into_result()
    }
    
    // identifier: cv_ml_TrainData_getClassLabels_const
    /// Returns the vector of class labels
    /// 
    /// The function returns vector of unique labels occurred in the responses.
    fn get_class_labels(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getClassLabels_const(self.as_raw_TrainData()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_getCatOfs_const
    fn get_cat_ofs(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getCatOfs_const(self.as_raw_TrainData()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_getCatMap_const
    fn get_cat_map(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getCatMap_const(self.as_raw_TrainData()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_setTrainTestSplit_int_count_bool_shuffle
    /// Splits the training data into the training and test parts
    /// ## See also
    /// TrainData::setTrainTestSplitRatio
    ///
    /// ## C++ default parameters:
    /// * shuffle: true
    fn set_train_test_split(&mut self, count: i32, shuffle: bool) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_TrainData_setTrainTestSplit_int_count_bool_shuffle(self.as_raw_TrainData(), count, shuffle) }.into_result()
    }
    
    // identifier: cv_ml_TrainData_setTrainTestSplitRatio_double_ratio_bool_shuffle
    /// Splits the training data into the training and test parts
    /// 
    /// The function selects a subset of specified relative size and then returns it as the training
    /// set. If the function is not called, all the data is used for training. Please, note that for
    /// each of TrainData::getTrain\* there is corresponding TrainData::getTest\*, so that the test
    /// subset can be retrieved and processed as well.
    /// ## See also
    /// TrainData::setTrainTestSplit
    ///
    /// ## C++ default parameters:
    /// * shuffle: true
    fn set_train_test_split_ratio(&mut self, ratio: f64, shuffle: bool) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_TrainData_setTrainTestSplitRatio_double_ratio_bool_shuffle(self.as_raw_TrainData(), ratio, shuffle) }.into_result()
    }
    
    // identifier: cv_ml_TrainData_shuffleTrainTest
    fn shuffle_train_test(&mut self) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_TrainData_shuffleTrainTest(self.as_raw_TrainData()) }.into_result()
    }
    
    // identifier: cv_ml_TrainData_getTestSamples_const
    /// Returns matrix of test samples
    fn get_test_samples(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getTestSamples_const(self.as_raw_TrainData()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_getNames_const_VectorOfString_names
    /// Returns vector of symbolic names captured in loadFromCSV()
    fn get_names(&self, names: &types::VectorOfString) -> Result<()> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getNames_const_VectorOfString_names(self.as_raw_TrainData(), names.as_raw_VectorOfString()) }.into_result()
    }
    
}

impl<'a> TrainData + 'a {

    // identifier: cv_ml_TrainData_getSubVector_Mat_vec_Mat_idx
    /// Extract from 1D vector elements specified by passed indexes.
    /// ## Parameters
    /// * vec: input vector (supported types: CV_32S, CV_32F, CV_64F)
    /// * idx: 1D index vector
    pub fn get_sub_vector(vec: &core::Mat, idx: &core::Mat) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getSubVector_Mat_vec_Mat_idx(vec.as_raw_Mat(), idx.as_raw_Mat()) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_getSubMatrix_Mat_matrix_Mat_idx_int_layout
    /// Extract from matrix rows/cols specified by passed indexes.
    /// ## Parameters
    /// * matrix: input matrix (supported types: CV_32S, CV_32F, CV_64F)
    /// * idx: 1D index vector
    /// * layout: specifies to extract rows (cv::ml::ROW_SAMPLES) or to extract columns (cv::ml::COL_SAMPLES)
    pub fn get_sub_matrix(matrix: &core::Mat, idx: &core::Mat, layout: i32) -> Result<core::Mat> {
        unsafe { sys::cv_ml_cv_ml_TrainData_getSubMatrix_Mat_matrix_Mat_idx_int_layout(matrix.as_raw_Mat(), idx.as_raw_Mat(), layout) }.into_result().map(|x| core::Mat { ptr: x })
    }
    
    // identifier: cv_ml_TrainData_create_Mat_samples_int_layout_Mat_responses_Mat_varIdx_Mat_sampleIdx_Mat_sampleWeights_Mat_varType
    /// Creates training data from in-memory arrays.
    /// 
    /// ## Parameters
    /// * samples: matrix of samples. It should have CV_32F type.
    /// * layout: see ml::SampleTypes.
    /// * responses: matrix of responses. If the responses are scalar, they should be stored as a
    /// single row or as a single column. The matrix should have type CV_32F or CV_32S (in the
    /// former case the responses are considered as ordered by default; in the latter case - as
    /// categorical)
    /// * varIdx: vector specifying which variables to use for training. It can be an integer vector
    /// (CV_32S) containing 0-based variable indices or byte vector (CV_8U) containing a mask of
    /// active variables.
    /// * sampleIdx: vector specifying which samples to use for training. It can be an integer
    /// vector (CV_32S) containing 0-based sample indices or byte vector (CV_8U) containing a mask
    /// of training samples.
    /// * sampleWeights: optional vector with weights for each sample. It should have CV_32F type.
    /// * varType: optional vector of type CV_8U and size `<number_of_variables_in_samples> +
    /// <number_of_variables_in_responses>`, containing types of each input and output variable. See
    /// ml::VariableTypes.
    ///
    /// ## C++ default parameters:
    /// * var_idx: noArray()
    /// * sample_idx: noArray()
    /// * sample_weights: noArray()
    /// * var_type: noArray()
    pub fn create(samples: &core::Mat, layout: i32, responses: &core::Mat, var_idx: &core::Mat, sample_idx: &core::Mat, sample_weights: &core::Mat, var_type: &core::Mat) -> Result<types::PtrOfTrainData> {
        unsafe { sys::cv_ml_cv_ml_TrainData_create_Mat_samples_int_layout_Mat_responses_Mat_varIdx_Mat_sampleIdx_Mat_sampleWeights_Mat_varType(samples.as_raw_Mat(), layout, responses.as_raw_Mat(), var_idx.as_raw_Mat(), sample_idx.as_raw_Mat(), sample_weights.as_raw_Mat(), var_type.as_raw_Mat()) }.into_result().map(|x| types::PtrOfTrainData { ptr: x })
    }
    
}

