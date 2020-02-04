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
use crate::{mod_prelude::*, core, sys, types};
use crate::core::{_InputArrayTrait, _OutputArrayTrait};

/// The simulated annealing algorithm. See [Kirkpatrick83](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_Kirkpatrick83) for details.
pub const ANN_MLP_ANNEAL: i32 = 2;
/// The back-propagation algorithm.
pub const ANN_MLP_BACKPROP: i32 = 0;
pub const ANN_MLP_GAUSSIAN: i32 = 2;
pub const ANN_MLP_IDENTITY: i32 = 0;
pub const ANN_MLP_LEAKYRELU: i32 = 4;
pub const ANN_MLP_NO_INPUT_SCALE: i32 = 2;
pub const ANN_MLP_NO_OUTPUT_SCALE: i32 = 4;
pub const ANN_MLP_RELU: i32 = 3;
/// The RPROP algorithm. See [RPROP93](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_RPROP93) for details.
pub const ANN_MLP_RPROP: i32 = 1;
pub const ANN_MLP_SIGMOID_SYM: i32 = 1;
pub const ANN_MLP_UPDATE_WEIGHTS: i32 = 1;
/// Discrete AdaBoost.
pub const Boost_DISCRETE: i32 = 0;
/// Gentle AdaBoost. It puts less weight on outlier data points and for that
pub const Boost_GENTLE: i32 = 3;
/// LogitBoost. It can produce good regression fits.
pub const Boost_LOGIT: i32 = 2;
/// Real AdaBoost. It is a technique that utilizes confidence-rated predictions
pub const Boost_REAL: i32 = 1;
/// each training sample occupies a column of samples
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
/// Set MiniBatchSize to a positive integer when using this method.
pub const LogisticRegression_MINI_BATCH: i32 = 1;
/// Regularization disabled
pub const LogisticRegression_REG_DISABLE: i32 = -1;
/// %L1 norm
pub const LogisticRegression_REG_L1: i32 = 0;
/// %L2 norm
pub const LogisticRegression_REG_L2: i32 = 1;
/// each training sample is a row of samples
pub const ROW_SAMPLE: i32 = 0;
/// Average Stochastic Gradient Descent
pub const SVMSGD_ASGD: i32 = 1;
/// More accurate for the case of linearly separable sets.
pub const SVMSGD_HARD_MARGIN: i32 = 1;
/// Stochastic Gradient Descent
pub const SVMSGD_SGD: i32 = 0;
/// General case, suits to the case of non-linearly separable sets, allows outliers.
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
/// makes the method return the raw results (the sum), not the class label
pub const StatModel_RAW_OUTPUT: i32 = 1;
pub const StatModel_UPDATE_MODEL: i32 = 1;
pub const TEST_ERROR: i32 = 0;
pub const TRAIN_ERROR: i32 = 1;
/// categorical variables
pub const VAR_CATEGORICAL: i32 = 1;
/// same as VAR_ORDERED
pub const VAR_NUMERICAL: i32 = 0;
/// ordered variables
pub const VAR_ORDERED: i32 = 0;

/// Creates test set
pub fn create_concentric_spheres_test_set(nsamples: i32, nfeatures: i32, nclasses: i32, samples: &mut dyn core::ToOutputArray, responses: &mut dyn core::ToOutputArray) -> Result<()> {
    output_array_arg!(samples);
    output_array_arg!(responses);
    unsafe { sys::cv_ml_createConcentricSpheresTestSet_int_int_int__OutputArray__OutputArray(nsamples, nfeatures, nclasses, samples.as_raw__OutputArray(), responses.as_raw__OutputArray()) }.into_result()
}

/// Generates _sample_ from multivariate normal distribution
///
/// ## Parameters
/// * mean: an average row vector
/// * cov: symmetric covariation matrix
/// * nsamples: returned samples count
/// * samples: returned samples array
pub fn rand_mv_normal(mean: &dyn core::ToInputArray, cov: &dyn core::ToInputArray, nsamples: i32, samples: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(mean);
    input_array_arg!(cov);
    output_array_arg!(samples);
    unsafe { sys::cv_ml_randMVNormal__InputArray__InputArray_int__OutputArray(mean.as_raw__InputArray(), cov.as_raw__InputArray(), nsamples, samples.as_raw__OutputArray()) }.into_result()
}

// Generating impl for trait crate::ml::ANN_MLP
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
pub trait ANN_MLP: crate::ml::StatModel {
    fn as_raw_ANN_MLP(&self) -> *mut c_void;
    /// Sets training method and common parameters.
    /// ## Parameters
    /// * method: Default value is ANN_MLP::RPROP. See ANN_MLP::TrainingMethods.
    /// * param1: passed to setRpropDW0 for ANN_MLP::RPROP and to setBackpropWeightScale for ANN_MLP::BACKPROP and to initialT for ANN_MLP::ANNEAL.
    /// * param2: passed to setRpropDWMin for ANN_MLP::RPROP and to setBackpropMomentumScale for ANN_MLP::BACKPROP and to finalT for ANN_MLP::ANNEAL.
    ///
    /// ## C++ default parameters
    /// * param1: 0
    /// * param2: 0
    fn set_train_method(&mut self, method: i32, param1: f64, param2: f64) -> Result<()> {
        unsafe { sys::cv_ml_ANN_MLP_setTrainMethod_int_double_double(self.as_raw_ANN_MLP(), method, param1, param2) }.into_result()
    }
    
    /// Returns current training method
    fn get_train_method(&self) -> Result<i32> {
        unsafe { sys::cv_ml_ANN_MLP_getTrainMethod_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    /// Initialize the activation function for each neuron.
    /// Currently the default and the only fully supported activation function is ANN_MLP::SIGMOID_SYM.
    /// ## Parameters
    /// * type: The type of activation function. See ANN_MLP::ActivationFunctions.
    /// * param1: The first parameter of the activation function, ![inline formula](https://latex.codecogs.com/png.latex?%5Calpha). Default value is 0.
    /// * param2: The second parameter of the activation function, ![inline formula](https://latex.codecogs.com/png.latex?%5Cbeta). Default value is 0.
    ///
    /// ## C++ default parameters
    /// * param1: 0
    /// * param2: 0
    fn set_activation_function(&mut self, _type: i32, param1: f64, param2: f64) -> Result<()> {
        unsafe { sys::cv_ml_ANN_MLP_setActivationFunction_int_double_double(self.as_raw_ANN_MLP(), _type, param1, param2) }.into_result()
    }
    
    /// Integer vector specifying the number of neurons in each layer including the input and output layers.
    /// The very first element specifies the number of elements in the input layer.
    /// The last element - number of elements in the output layer. Default value is empty Mat.
    /// ## See also
    /// getLayerSizes
    fn set_layer_sizes(&mut self, _layer_sizes: &dyn core::ToInputArray) -> Result<()> {
        input_array_arg!(_layer_sizes);
        unsafe { sys::cv_ml_ANN_MLP_setLayerSizes__InputArray(self.as_raw_ANN_MLP(), _layer_sizes.as_raw__InputArray()) }.into_result()
    }
    
    /// Integer vector specifying the number of neurons in each layer including the input and output layers.
    /// The very first element specifies the number of elements in the input layer.
    /// The last element - number of elements in the output layer.
    /// ## See also
    /// setLayerSizes
    fn get_layer_sizes(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_ANN_MLP_getLayerSizes_const(self.as_raw_ANN_MLP()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// @see setTermCriteria
    fn get_term_criteria(&self) -> Result<core::TermCriteria> {
        unsafe { sys::cv_ml_ANN_MLP_getTermCriteria_const(self.as_raw_ANN_MLP()) }.into_result().map(|ptr| core::TermCriteria { ptr })
    }
    
    /// @copybrief getTermCriteria @see getTermCriteria
    fn set_term_criteria(&mut self, val: &core::TermCriteria) -> Result<()> {
        unsafe { sys::cv_ml_ANN_MLP_setTermCriteria_TermCriteria(self.as_raw_ANN_MLP(), val.as_raw_TermCriteria()) }.into_result()
    }
    
    /// @see setBackpropWeightScale
    fn get_backprop_weight_scale(&self) -> Result<f64> {
        unsafe { sys::cv_ml_ANN_MLP_getBackpropWeightScale_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    /// @copybrief getBackpropWeightScale @see getBackpropWeightScale
    fn set_backprop_weight_scale(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_ANN_MLP_setBackpropWeightScale_double(self.as_raw_ANN_MLP(), val) }.into_result()
    }
    
    /// @see setBackpropMomentumScale
    fn get_backprop_momentum_scale(&self) -> Result<f64> {
        unsafe { sys::cv_ml_ANN_MLP_getBackpropMomentumScale_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    /// @copybrief getBackpropMomentumScale @see getBackpropMomentumScale
    fn set_backprop_momentum_scale(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_ANN_MLP_setBackpropMomentumScale_double(self.as_raw_ANN_MLP(), val) }.into_result()
    }
    
    /// @see setRpropDW0
    fn get_rprop_dw0(&self) -> Result<f64> {
        unsafe { sys::cv_ml_ANN_MLP_getRpropDW0_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    /// @copybrief getRpropDW0 @see getRpropDW0
    fn set_rprop_dw0(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_ANN_MLP_setRpropDW0_double(self.as_raw_ANN_MLP(), val) }.into_result()
    }
    
    /// @see setRpropDWPlus
    fn get_rprop_dw_plus(&self) -> Result<f64> {
        unsafe { sys::cv_ml_ANN_MLP_getRpropDWPlus_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    /// @copybrief getRpropDWPlus @see getRpropDWPlus
    fn set_rprop_dw_plus(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_ANN_MLP_setRpropDWPlus_double(self.as_raw_ANN_MLP(), val) }.into_result()
    }
    
    /// @see setRpropDWMinus
    fn get_rprop_dw_minus(&self) -> Result<f64> {
        unsafe { sys::cv_ml_ANN_MLP_getRpropDWMinus_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    /// @copybrief getRpropDWMinus @see getRpropDWMinus
    fn set_rprop_dw_minus(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_ANN_MLP_setRpropDWMinus_double(self.as_raw_ANN_MLP(), val) }.into_result()
    }
    
    /// @see setRpropDWMin
    fn get_rprop_dw_min(&self) -> Result<f64> {
        unsafe { sys::cv_ml_ANN_MLP_getRpropDWMin_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    /// @copybrief getRpropDWMin @see getRpropDWMin
    fn set_rprop_dw_min(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_ANN_MLP_setRpropDWMin_double(self.as_raw_ANN_MLP(), val) }.into_result()
    }
    
    /// @see setRpropDWMax
    fn get_rprop_dw_max(&self) -> Result<f64> {
        unsafe { sys::cv_ml_ANN_MLP_getRpropDWMax_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    /// @copybrief getRpropDWMax @see getRpropDWMax
    fn set_rprop_dw_max(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_ANN_MLP_setRpropDWMax_double(self.as_raw_ANN_MLP(), val) }.into_result()
    }
    
    /// @see setAnnealInitialT
    fn get_anneal_initial_t(&self) -> Result<f64> {
        unsafe { sys::cv_ml_ANN_MLP_getAnnealInitialT_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    /// @copybrief getAnnealInitialT @see getAnnealInitialT
    fn set_anneal_initial_t(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_ANN_MLP_setAnnealInitialT_double(self.as_raw_ANN_MLP(), val) }.into_result()
    }
    
    /// @see setAnnealFinalT
    fn get_anneal_final_t(&self) -> Result<f64> {
        unsafe { sys::cv_ml_ANN_MLP_getAnnealFinalT_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    /// @copybrief getAnnealFinalT @see getAnnealFinalT
    fn set_anneal_final_t(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_ANN_MLP_setAnnealFinalT_double(self.as_raw_ANN_MLP(), val) }.into_result()
    }
    
    /// @see setAnnealCoolingRatio
    fn get_anneal_cooling_ratio(&self) -> Result<f64> {
        unsafe { sys::cv_ml_ANN_MLP_getAnnealCoolingRatio_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    /// @copybrief getAnnealCoolingRatio @see getAnnealCoolingRatio
    fn set_anneal_cooling_ratio(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_ANN_MLP_setAnnealCoolingRatio_double(self.as_raw_ANN_MLP(), val) }.into_result()
    }
    
    /// @see setAnnealItePerStep
    fn get_anneal_ite_per_step(&self) -> Result<i32> {
        unsafe { sys::cv_ml_ANN_MLP_getAnnealItePerStep_const(self.as_raw_ANN_MLP()) }.into_result()
    }
    
    /// @copybrief getAnnealItePerStep @see getAnnealItePerStep
    fn set_anneal_ite_per_step(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_ANN_MLP_setAnnealItePerStep_int(self.as_raw_ANN_MLP(), val) }.into_result()
    }
    
    fn get_weights(&self, layer_idx: i32) -> Result<core::Mat> {
        unsafe { sys::cv_ml_ANN_MLP_getWeights_const_int(self.as_raw_ANN_MLP(), layer_idx) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
}

impl dyn ANN_MLP + '_ {
    /// Creates empty model
    ///
    /// Use StatModel::train to train the model, Algorithm::load\<ANN_MLP\>(filename) to load the pre-trained model.
    /// Note that the train method has optional flags: ANN_MLP::TrainFlags.
    pub fn create() -> Result<types::PtrOfANN_MLP> {
        unsafe { sys::cv_ml_ANN_MLP_create() }.into_result().map(|ptr| types::PtrOfANN_MLP { ptr })
    }
    
    /// Loads and creates a serialized ANN from a file
    ///
    /// Use ANN::save to serialize and store an ANN to disk.
    /// Load the ANN from this file again, by calling this function with the path to the file.
    ///
    /// ## Parameters
    /// * filepath: path to serialized ANN
    pub fn load(filepath: &str) -> Result<types::PtrOfANN_MLP> {
        string_arg!(filepath);
        unsafe { sys::cv_ml_ANN_MLP_load_String(filepath.as_ptr()) }.into_result().map(|ptr| types::PtrOfANN_MLP { ptr })
    }
    
}

// Generating impl for trait crate::ml::Boost
/// Boosted tree classifier derived from DTrees
///
/// ## See also
/// @ref ml_intro_boost
pub trait Boost: crate::ml::DTrees {
    fn as_raw_Boost(&self) -> *mut c_void;
    /// @see setBoostType
    fn get_boost_type(&self) -> Result<i32> {
        unsafe { sys::cv_ml_Boost_getBoostType_const(self.as_raw_Boost()) }.into_result()
    }
    
    /// @copybrief getBoostType @see getBoostType
    fn set_boost_type(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_Boost_setBoostType_int(self.as_raw_Boost(), val) }.into_result()
    }
    
    /// @see setWeakCount
    fn get_weak_count(&self) -> Result<i32> {
        unsafe { sys::cv_ml_Boost_getWeakCount_const(self.as_raw_Boost()) }.into_result()
    }
    
    /// @copybrief getWeakCount @see getWeakCount
    fn set_weak_count(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_Boost_setWeakCount_int(self.as_raw_Boost(), val) }.into_result()
    }
    
    /// @see setWeightTrimRate
    fn get_weight_trim_rate(&self) -> Result<f64> {
        unsafe { sys::cv_ml_Boost_getWeightTrimRate_const(self.as_raw_Boost()) }.into_result()
    }
    
    /// @copybrief getWeightTrimRate @see getWeightTrimRate
    fn set_weight_trim_rate(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_Boost_setWeightTrimRate_double(self.as_raw_Boost(), val) }.into_result()
    }
    
}

impl dyn Boost + '_ {
    /// Creates the empty model.
    /// Use StatModel::train to train the model, Algorithm::load\<Boost\>(filename) to load the pre-trained model.
    pub fn create() -> Result<types::PtrOfBoost> {
        unsafe { sys::cv_ml_Boost_create() }.into_result().map(|ptr| types::PtrOfBoost { ptr })
    }
    
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
    /// ## C++ default parameters
    /// * node_name: String()
    pub fn load(filepath: &str, node_name: &str) -> Result<types::PtrOfBoost> {
        string_arg!(filepath);
        string_arg!(node_name);
        unsafe { sys::cv_ml_Boost_load_String_String(filepath.as_ptr(), node_name.as_ptr()) }.into_result().map(|ptr| types::PtrOfBoost { ptr })
    }
    
}

// Generating impl for trait crate::ml::DTrees
/// The class represents a single decision tree or a collection of decision trees.
///
/// The current public interface of the class allows user to train only a single decision tree, however
/// the class is capable of storing multiple decision trees and using them for prediction (by summing
/// responses or using a voting schemes), and the derived from DTrees classes (such as RTrees and Boost)
/// use this capability to implement decision tree ensembles.
///
/// ## See also
/// @ref ml_intro_trees
pub trait DTrees: crate::ml::StatModel {
    fn as_raw_DTrees(&self) -> *mut c_void;
    /// @see setMaxCategories
    fn get_max_categories(&self) -> Result<i32> {
        unsafe { sys::cv_ml_DTrees_getMaxCategories_const(self.as_raw_DTrees()) }.into_result()
    }
    
    /// @copybrief getMaxCategories @see getMaxCategories
    fn set_max_categories(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_DTrees_setMaxCategories_int(self.as_raw_DTrees(), val) }.into_result()
    }
    
    /// @see setMaxDepth
    fn get_max_depth(&self) -> Result<i32> {
        unsafe { sys::cv_ml_DTrees_getMaxDepth_const(self.as_raw_DTrees()) }.into_result()
    }
    
    /// @copybrief getMaxDepth @see getMaxDepth
    fn set_max_depth(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_DTrees_setMaxDepth_int(self.as_raw_DTrees(), val) }.into_result()
    }
    
    /// @see setMinSampleCount
    fn get_min_sample_count(&self) -> Result<i32> {
        unsafe { sys::cv_ml_DTrees_getMinSampleCount_const(self.as_raw_DTrees()) }.into_result()
    }
    
    /// @copybrief getMinSampleCount @see getMinSampleCount
    fn set_min_sample_count(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_DTrees_setMinSampleCount_int(self.as_raw_DTrees(), val) }.into_result()
    }
    
    /// @see setCVFolds
    fn get_cv_folds(&self) -> Result<i32> {
        unsafe { sys::cv_ml_DTrees_getCVFolds_const(self.as_raw_DTrees()) }.into_result()
    }
    
    /// @copybrief getCVFolds @see getCVFolds
    fn set_cv_folds(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_DTrees_setCVFolds_int(self.as_raw_DTrees(), val) }.into_result()
    }
    
    /// @see setUseSurrogates
    fn get_use_surrogates(&self) -> Result<bool> {
        unsafe { sys::cv_ml_DTrees_getUseSurrogates_const(self.as_raw_DTrees()) }.into_result()
    }
    
    /// @copybrief getUseSurrogates @see getUseSurrogates
    fn set_use_surrogates(&mut self, val: bool) -> Result<()> {
        unsafe { sys::cv_ml_DTrees_setUseSurrogates_bool(self.as_raw_DTrees(), val) }.into_result()
    }
    
    /// @see setUse1SERule
    fn get_use1_se_rule(&self) -> Result<bool> {
        unsafe { sys::cv_ml_DTrees_getUse1SERule_const(self.as_raw_DTrees()) }.into_result()
    }
    
    /// @copybrief getUse1SERule @see getUse1SERule
    fn set_use1_se_rule(&mut self, val: bool) -> Result<()> {
        unsafe { sys::cv_ml_DTrees_setUse1SERule_bool(self.as_raw_DTrees(), val) }.into_result()
    }
    
    /// @see setTruncatePrunedTree
    fn get_truncate_pruned_tree(&self) -> Result<bool> {
        unsafe { sys::cv_ml_DTrees_getTruncatePrunedTree_const(self.as_raw_DTrees()) }.into_result()
    }
    
    /// @copybrief getTruncatePrunedTree @see getTruncatePrunedTree
    fn set_truncate_pruned_tree(&mut self, val: bool) -> Result<()> {
        unsafe { sys::cv_ml_DTrees_setTruncatePrunedTree_bool(self.as_raw_DTrees(), val) }.into_result()
    }
    
    /// @see setRegressionAccuracy
    fn get_regression_accuracy(&self) -> Result<f32> {
        unsafe { sys::cv_ml_DTrees_getRegressionAccuracy_const(self.as_raw_DTrees()) }.into_result()
    }
    
    /// @copybrief getRegressionAccuracy @see getRegressionAccuracy
    fn set_regression_accuracy(&mut self, val: f32) -> Result<()> {
        unsafe { sys::cv_ml_DTrees_setRegressionAccuracy_float(self.as_raw_DTrees(), val) }.into_result()
    }
    
    /// @see setPriors
    fn get_priors(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_DTrees_getPriors_const(self.as_raw_DTrees()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// @copybrief getPriors @see getPriors
    fn set_priors(&mut self, val: &core::Mat) -> Result<()> {
        unsafe { sys::cv_ml_DTrees_setPriors_Mat(self.as_raw_DTrees(), val.as_raw_Mat()) }.into_result()
    }
    
    /// Returns indices of root nodes
    fn get_roots(&self) -> Result<types::VectorOfint> {
        unsafe { sys::cv_ml_DTrees_getRoots_const(self.as_raw_DTrees()) }.into_result().map(|ptr| types::VectorOfint { ptr })
    }
    
    /// Returns all the nodes
    ///
    /// all the node indices are indices in the returned vector
    fn get_nodes(&self) -> Result<types::VectorOfNode> {
        unsafe { sys::cv_ml_DTrees_getNodes_const(self.as_raw_DTrees()) }.into_result().map(|ptr| types::VectorOfNode { ptr })
    }
    
    /// Returns all the splits
    ///
    /// all the split indices are indices in the returned vector
    fn get_splits(&self) -> Result<types::VectorOfSplit> {
        unsafe { sys::cv_ml_DTrees_getSplits_const(self.as_raw_DTrees()) }.into_result().map(|ptr| types::VectorOfSplit { ptr })
    }
    
    /// Returns all the bitsets for categorical splits
    ///
    /// Split::subsetOfs is an offset in the returned vector
    fn get_subsets(&self) -> Result<types::VectorOfint> {
        unsafe { sys::cv_ml_DTrees_getSubsets_const(self.as_raw_DTrees()) }.into_result().map(|ptr| types::VectorOfint { ptr })
    }
    
}

impl dyn DTrees + '_ {
    /// Creates the empty model
    ///
    /// The static method creates empty decision tree with the specified parameters. It should be then
    /// trained using train method (see StatModel::train). Alternatively, you can load the model from
    /// file using Algorithm::load\<DTrees\>(filename).
    pub fn create() -> Result<types::PtrOfDTrees> {
        unsafe { sys::cv_ml_DTrees_create() }.into_result().map(|ptr| types::PtrOfDTrees { ptr })
    }
    
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
    /// ## C++ default parameters
    /// * node_name: String()
    pub fn load(filepath: &str, node_name: &str) -> Result<types::PtrOfDTrees> {
        string_arg!(filepath);
        string_arg!(node_name);
        unsafe { sys::cv_ml_DTrees_load_String_String(filepath.as_ptr(), node_name.as_ptr()) }.into_result().map(|ptr| types::PtrOfDTrees { ptr })
    }
    
}

// boxed class cv::ml::DTrees::Node
/// The class represents a decision tree node.
pub struct DTrees_Node {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for DTrees_Node {
    fn drop(&mut self) {
        unsafe { sys::cv_DTrees_Node_delete(self.ptr) };
    }
}

impl DTrees_Node {
    #[inline(always)] pub fn as_raw_DTrees_Node(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for DTrees_Node {}

impl DTrees_Node {
    pub fn default() -> Result<crate::ml::DTrees_Node> {
        unsafe { sys::cv_ml_DTrees_Node_Node() }.into_result().map(|ptr| crate::ml::DTrees_Node { ptr })
    }
    
}

// boxed class cv::ml::DTrees::Split
/// The class represents split in a decision tree.
pub struct DTrees_Split {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for DTrees_Split {
    fn drop(&mut self) {
        unsafe { sys::cv_DTrees_Split_delete(self.ptr) };
    }
}

impl DTrees_Split {
    #[inline(always)] pub fn as_raw_DTrees_Split(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for DTrees_Split {}

impl DTrees_Split {
    pub fn default() -> Result<crate::ml::DTrees_Split> {
        unsafe { sys::cv_ml_DTrees_Split_Split() }.into_result().map(|ptr| crate::ml::DTrees_Split { ptr })
    }
    
}

// Generating impl for trait crate::ml::EM
/// The class implements the Expectation Maximization algorithm.
///
/// ## See also
/// @ref ml_intro_em
pub trait EM: crate::ml::StatModel {
    fn as_raw_EM(&self) -> *mut c_void;
    /// @see setClustersNumber
    fn get_clusters_number(&self) -> Result<i32> {
        unsafe { sys::cv_ml_EM_getClustersNumber_const(self.as_raw_EM()) }.into_result()
    }
    
    /// @copybrief getClustersNumber @see getClustersNumber
    fn set_clusters_number(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_EM_setClustersNumber_int(self.as_raw_EM(), val) }.into_result()
    }
    
    /// @see setCovarianceMatrixType
    fn get_covariance_matrix_type(&self) -> Result<i32> {
        unsafe { sys::cv_ml_EM_getCovarianceMatrixType_const(self.as_raw_EM()) }.into_result()
    }
    
    /// @copybrief getCovarianceMatrixType @see getCovarianceMatrixType
    fn set_covariance_matrix_type(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_EM_setCovarianceMatrixType_int(self.as_raw_EM(), val) }.into_result()
    }
    
    /// @see setTermCriteria
    fn get_term_criteria(&self) -> Result<core::TermCriteria> {
        unsafe { sys::cv_ml_EM_getTermCriteria_const(self.as_raw_EM()) }.into_result().map(|ptr| core::TermCriteria { ptr })
    }
    
    /// @copybrief getTermCriteria @see getTermCriteria
    fn set_term_criteria(&mut self, val: &core::TermCriteria) -> Result<()> {
        unsafe { sys::cv_ml_EM_setTermCriteria_TermCriteria(self.as_raw_EM(), val.as_raw_TermCriteria()) }.into_result()
    }
    
    /// Returns weights of the mixtures
    ///
    /// Returns vector with the number of elements equal to the number of mixtures.
    fn get_weights(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_EM_getWeights_const(self.as_raw_EM()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Returns the cluster centers (means of the Gaussian mixture)
    ///
    /// Returns matrix with the number of rows equal to the number of mixtures and number of columns
    /// equal to the space dimensionality.
    fn get_means(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_EM_getMeans_const(self.as_raw_EM()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Returns covariation matrices
    ///
    /// Returns vector of covariation matrices. Number of matrices is the number of gaussian mixtures,
    /// each matrix is a square floating-point matrix NxN, where N is the space dimensionality.
    fn get_covs(&self, covs: &mut types::VectorOfMat) -> Result<()> {
        unsafe { sys::cv_ml_EM_getCovs_const_VectorOfMat(self.as_raw_EM(), covs.as_raw_VectorOfMat()) }.into_result()
    }
    
    /// Returns posterior probabilities for the provided samples
    ///
    /// ## Parameters
    /// * samples: The input samples, floating-point matrix
    /// * results: The optional output ![inline formula](https://latex.codecogs.com/png.latex?%20nSamples%20%5Ctimes%20nClusters) matrix of results. It contains
    /// posterior probabilities for each sample from the input
    /// * flags: This parameter will be ignored
    ///
    /// ## C++ default parameters
    /// * results: noArray()
    /// * flags: 0
    fn predict(&self, samples: &dyn core::ToInputArray, results: &mut dyn core::ToOutputArray, flags: i32) -> Result<f32> {
        input_array_arg!(samples);
        output_array_arg!(results);
        unsafe { sys::cv_ml_EM_predict_const__InputArray__OutputArray_int(self.as_raw_EM(), samples.as_raw__InputArray(), results.as_raw__OutputArray(), flags) }.into_result()
    }
    
    /// Returns a likelihood logarithm value and an index of the most probable mixture component
    /// for the given sample.
    ///
    /// ## Parameters
    /// * sample: A sample for classification. It should be a one-channel matrix of
    /// ![inline formula](https://latex.codecogs.com/png.latex?1%20%5Ctimes%20dims) or ![inline formula](https://latex.codecogs.com/png.latex?dims%20%5Ctimes%201) size.
    /// * probs: Optional output matrix that contains posterior probabilities of each component
    /// given the sample. It has ![inline formula](https://latex.codecogs.com/png.latex?1%20%5Ctimes%20nclusters) size and CV_64FC1 type.
    ///
    /// The method returns a two-element double vector. Zero element is a likelihood logarithm value for
    /// the sample. First element is an index of the most probable mixture component for the given
    /// sample.
    fn predict2(&self, sample: &dyn core::ToInputArray, probs: &mut dyn core::ToOutputArray) -> Result<core::Vec2d> {
        input_array_arg!(sample);
        output_array_arg!(probs);
        unsafe { sys::cv_ml_EM_predict2_const__InputArray__OutputArray(self.as_raw_EM(), sample.as_raw__InputArray(), probs.as_raw__OutputArray()) }.into_result()
    }
    
    /// Estimate the Gaussian mixture parameters from a samples set.
    ///
    /// This variation starts with Expectation step. Initial values of the model parameters will be
    /// estimated by the k-means algorithm.
    ///
    /// Unlike many of the ML models, %EM is an unsupervised learning algorithm and it does not take
    /// responses (class labels or function values) as input. Instead, it computes the *Maximum
    /// Likelihood Estimate* of the Gaussian mixture parameters from an input sample set, stores all the
    /// parameters inside the structure: ![inline formula](https://latex.codecogs.com/png.latex?p_%7Bi%2Ck%7D) in probs, ![inline formula](https://latex.codecogs.com/png.latex?a_k) in means , ![inline formula](https://latex.codecogs.com/png.latex?S_k) in
    /// covs[k], ![inline formula](https://latex.codecogs.com/png.latex?%5Cpi_k) in weights , and optionally computes the output "class label" for each
    /// sample: ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Blabels%7D_i%3D%5Ctexttt%7Barg%20max%7D_k%28p_%7Bi%2Ck%7D%29%2C%20i%3D1..N) (indices of the most
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
    /// each sample. It has ![inline formula](https://latex.codecogs.com/png.latex?nsamples%20%5Ctimes%201) size and CV_64FC1 type.
    /// * labels: The optional output "class label" for each sample:
    /// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Blabels%7D_i%3D%5Ctexttt%7Barg%20max%7D_k%28p_%7Bi%2Ck%7D%29%2C%20i%3D1..N) (indices of the most probable
    /// mixture component for each sample). It has ![inline formula](https://latex.codecogs.com/png.latex?nsamples%20%5Ctimes%201) size and CV_32SC1 type.
    /// * probs: The optional output matrix that contains posterior probabilities of each Gaussian
    /// mixture component given the each sample. It has ![inline formula](https://latex.codecogs.com/png.latex?nsamples%20%5Ctimes%20nclusters) size and
    /// CV_64FC1 type.
    ///
    /// ## C++ default parameters
    /// * log_likelihoods: noArray()
    /// * labels: noArray()
    /// * probs: noArray()
    fn train_em(&mut self, samples: &dyn core::ToInputArray, log_likelihoods: &mut dyn core::ToOutputArray, labels: &mut dyn core::ToOutputArray, probs: &mut dyn core::ToOutputArray) -> Result<bool> {
        input_array_arg!(samples);
        output_array_arg!(log_likelihoods);
        output_array_arg!(labels);
        output_array_arg!(probs);
        unsafe { sys::cv_ml_EM_trainEM__InputArray__OutputArray__OutputArray__OutputArray(self.as_raw_EM(), samples.as_raw__InputArray(), log_likelihoods.as_raw__OutputArray(), labels.as_raw__OutputArray(), probs.as_raw__OutputArray()) }.into_result()
    }
    
    /// Estimate the Gaussian mixture parameters from a samples set.
    ///
    /// This variation starts with Expectation step. You need to provide initial means ![inline formula](https://latex.codecogs.com/png.latex?a_k) of
    /// mixture components. Optionally you can pass initial weights ![inline formula](https://latex.codecogs.com/png.latex?%5Cpi_k) and covariance matrices
    /// ![inline formula](https://latex.codecogs.com/png.latex?S_k) of mixture components.
    ///
    /// ## Parameters
    /// * samples: Samples from which the Gaussian mixture model will be estimated. It should be a
    /// one-channel matrix, each row of which is a sample. If the matrix does not have CV_64F type
    /// it will be converted to the inner matrix of such type for the further computing.
    /// * means0: Initial means ![inline formula](https://latex.codecogs.com/png.latex?a_k) of mixture components. It is a one-channel matrix of
    /// ![inline formula](https://latex.codecogs.com/png.latex?nclusters%20%5Ctimes%20dims) size. If the matrix does not have CV_64F type it will be
    /// converted to the inner matrix of such type for the further computing.
    /// * covs0: The vector of initial covariance matrices ![inline formula](https://latex.codecogs.com/png.latex?S_k) of mixture components. Each of
    /// covariance matrices is a one-channel matrix of ![inline formula](https://latex.codecogs.com/png.latex?dims%20%5Ctimes%20dims) size. If the matrices
    /// do not have CV_64F type they will be converted to the inner matrices of such type for the
    /// further computing.
    /// * weights0: Initial weights ![inline formula](https://latex.codecogs.com/png.latex?%5Cpi_k) of mixture components. It should be a one-channel
    /// floating-point matrix with ![inline formula](https://latex.codecogs.com/png.latex?1%20%5Ctimes%20nclusters) or ![inline formula](https://latex.codecogs.com/png.latex?nclusters%20%5Ctimes%201) size.
    /// * logLikelihoods: The optional output matrix that contains a likelihood logarithm value for
    /// each sample. It has ![inline formula](https://latex.codecogs.com/png.latex?nsamples%20%5Ctimes%201) size and CV_64FC1 type.
    /// * labels: The optional output "class label" for each sample:
    /// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Blabels%7D_i%3D%5Ctexttt%7Barg%20max%7D_k%28p_%7Bi%2Ck%7D%29%2C%20i%3D1..N) (indices of the most probable
    /// mixture component for each sample). It has ![inline formula](https://latex.codecogs.com/png.latex?nsamples%20%5Ctimes%201) size and CV_32SC1 type.
    /// * probs: The optional output matrix that contains posterior probabilities of each Gaussian
    /// mixture component given the each sample. It has ![inline formula](https://latex.codecogs.com/png.latex?nsamples%20%5Ctimes%20nclusters) size and
    /// CV_64FC1 type.
    ///
    /// ## C++ default parameters
    /// * covs0: noArray()
    /// * weights0: noArray()
    /// * log_likelihoods: noArray()
    /// * labels: noArray()
    /// * probs: noArray()
    fn train_e(&mut self, samples: &dyn core::ToInputArray, means0: &dyn core::ToInputArray, covs0: &dyn core::ToInputArray, weights0: &dyn core::ToInputArray, log_likelihoods: &mut dyn core::ToOutputArray, labels: &mut dyn core::ToOutputArray, probs: &mut dyn core::ToOutputArray) -> Result<bool> {
        input_array_arg!(samples);
        input_array_arg!(means0);
        input_array_arg!(covs0);
        input_array_arg!(weights0);
        output_array_arg!(log_likelihoods);
        output_array_arg!(labels);
        output_array_arg!(probs);
        unsafe { sys::cv_ml_EM_trainE__InputArray__InputArray__InputArray__InputArray__OutputArray__OutputArray__OutputArray(self.as_raw_EM(), samples.as_raw__InputArray(), means0.as_raw__InputArray(), covs0.as_raw__InputArray(), weights0.as_raw__InputArray(), log_likelihoods.as_raw__OutputArray(), labels.as_raw__OutputArray(), probs.as_raw__OutputArray()) }.into_result()
    }
    
    /// Estimate the Gaussian mixture parameters from a samples set.
    ///
    /// This variation starts with Maximization step. You need to provide initial probabilities
    /// ![inline formula](https://latex.codecogs.com/png.latex?p_%7Bi%2Ck%7D) to use this option.
    ///
    /// ## Parameters
    /// * samples: Samples from which the Gaussian mixture model will be estimated. It should be a
    /// one-channel matrix, each row of which is a sample. If the matrix does not have CV_64F type
    /// it will be converted to the inner matrix of such type for the further computing.
    /// * probs0: the probabilities
    /// * logLikelihoods: The optional output matrix that contains a likelihood logarithm value for
    /// each sample. It has ![inline formula](https://latex.codecogs.com/png.latex?nsamples%20%5Ctimes%201) size and CV_64FC1 type.
    /// * labels: The optional output "class label" for each sample:
    /// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Blabels%7D_i%3D%5Ctexttt%7Barg%20max%7D_k%28p_%7Bi%2Ck%7D%29%2C%20i%3D1..N) (indices of the most probable
    /// mixture component for each sample). It has ![inline formula](https://latex.codecogs.com/png.latex?nsamples%20%5Ctimes%201) size and CV_32SC1 type.
    /// * probs: The optional output matrix that contains posterior probabilities of each Gaussian
    /// mixture component given the each sample. It has ![inline formula](https://latex.codecogs.com/png.latex?nsamples%20%5Ctimes%20nclusters) size and
    /// CV_64FC1 type.
    ///
    /// ## C++ default parameters
    /// * log_likelihoods: noArray()
    /// * labels: noArray()
    /// * probs: noArray()
    fn train_m(&mut self, samples: &dyn core::ToInputArray, probs0: &dyn core::ToInputArray, log_likelihoods: &mut dyn core::ToOutputArray, labels: &mut dyn core::ToOutputArray, probs: &mut dyn core::ToOutputArray) -> Result<bool> {
        input_array_arg!(samples);
        input_array_arg!(probs0);
        output_array_arg!(log_likelihoods);
        output_array_arg!(labels);
        output_array_arg!(probs);
        unsafe { sys::cv_ml_EM_trainM__InputArray__InputArray__OutputArray__OutputArray__OutputArray(self.as_raw_EM(), samples.as_raw__InputArray(), probs0.as_raw__InputArray(), log_likelihoods.as_raw__OutputArray(), labels.as_raw__OutputArray(), probs.as_raw__OutputArray()) }.into_result()
    }
    
}

impl dyn EM + '_ {
    /// Creates empty %EM model.
    /// The model should be trained then using StatModel::train(traindata, flags) method. Alternatively, you
    /// can use one of the EM::train\* methods or load it from file using Algorithm::load\<EM\>(filename).
    pub fn create() -> Result<types::PtrOfEM> {
        unsafe { sys::cv_ml_EM_create() }.into_result().map(|ptr| types::PtrOfEM { ptr })
    }
    
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
    /// ## C++ default parameters
    /// * node_name: String()
    pub fn load(filepath: &str, node_name: &str) -> Result<types::PtrOfEM> {
        string_arg!(filepath);
        string_arg!(node_name);
        unsafe { sys::cv_ml_EM_load_String_String(filepath.as_ptr(), node_name.as_ptr()) }.into_result().map(|ptr| types::PtrOfEM { ptr })
    }
    
}

// Generating impl for trait crate::ml::KNearest
/// The class implements K-Nearest Neighbors model
///
/// ## See also
/// @ref ml_intro_knn
pub trait KNearest: crate::ml::StatModel {
    fn as_raw_KNearest(&self) -> *mut c_void;
    /// @see setDefaultK
    fn get_default_k(&self) -> Result<i32> {
        unsafe { sys::cv_ml_KNearest_getDefaultK_const(self.as_raw_KNearest()) }.into_result()
    }
    
    /// @copybrief getDefaultK @see getDefaultK
    fn set_default_k(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_KNearest_setDefaultK_int(self.as_raw_KNearest(), val) }.into_result()
    }
    
    /// @see setIsClassifier
    fn get_is_classifier(&self) -> Result<bool> {
        unsafe { sys::cv_ml_KNearest_getIsClassifier_const(self.as_raw_KNearest()) }.into_result()
    }
    
    /// @copybrief getIsClassifier @see getIsClassifier
    fn set_is_classifier(&mut self, val: bool) -> Result<()> {
        unsafe { sys::cv_ml_KNearest_setIsClassifier_bool(self.as_raw_KNearest(), val) }.into_result()
    }
    
    /// @see setEmax
    fn get_emax(&self) -> Result<i32> {
        unsafe { sys::cv_ml_KNearest_getEmax_const(self.as_raw_KNearest()) }.into_result()
    }
    
    /// @copybrief getEmax @see getEmax
    fn set_emax(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_KNearest_setEmax_int(self.as_raw_KNearest(), val) }.into_result()
    }
    
    /// @see setAlgorithmType
    fn get_algorithm_type(&self) -> Result<i32> {
        unsafe { sys::cv_ml_KNearest_getAlgorithmType_const(self.as_raw_KNearest()) }.into_result()
    }
    
    /// @copybrief getAlgorithmType @see getAlgorithmType
    fn set_algorithm_type(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_KNearest_setAlgorithmType_int(self.as_raw_KNearest(), val) }.into_result()
    }
    
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
    /// ## C++ default parameters
    /// * neighbor_responses: noArray()
    /// * dist: noArray()
    fn find_nearest(&self, samples: &dyn core::ToInputArray, k: i32, results: &mut dyn core::ToOutputArray, neighbor_responses: &mut dyn core::ToOutputArray, dist: &mut dyn core::ToOutputArray) -> Result<f32> {
        input_array_arg!(samples);
        output_array_arg!(results);
        output_array_arg!(neighbor_responses);
        output_array_arg!(dist);
        unsafe { sys::cv_ml_KNearest_findNearest_const__InputArray_int__OutputArray__OutputArray__OutputArray(self.as_raw_KNearest(), samples.as_raw__InputArray(), k, results.as_raw__OutputArray(), neighbor_responses.as_raw__OutputArray(), dist.as_raw__OutputArray()) }.into_result()
    }
    
}

impl dyn KNearest + '_ {
    /// Creates the empty model
    ///
    /// The static method creates empty %KNearest classifier. It should be then trained using StatModel::train method.
    pub fn create() -> Result<types::PtrOfKNearest> {
        unsafe { sys::cv_ml_KNearest_create() }.into_result().map(|ptr| types::PtrOfKNearest { ptr })
    }
    
    /// Loads and creates a serialized knearest from a file
    ///
    /// Use KNearest::save to serialize and store an KNearest to disk.
    /// Load the KNearest from this file again, by calling this function with the path to the file.
    ///
    /// ## Parameters
    /// * filepath: path to serialized KNearest
    pub fn load(filepath: &str) -> Result<types::PtrOfKNearest> {
        string_arg!(filepath);
        unsafe { sys::cv_ml_KNearest_load_String(filepath.as_ptr()) }.into_result().map(|ptr| types::PtrOfKNearest { ptr })
    }
    
}

// Generating impl for trait crate::ml::LogisticRegression
/// Implements Logistic Regression classifier.
///
/// ## See also
/// @ref ml_intro_lr
pub trait LogisticRegression: crate::ml::StatModel {
    fn as_raw_LogisticRegression(&self) -> *mut c_void;
    /// @see setLearningRate
    fn get_learning_rate(&self) -> Result<f64> {
        unsafe { sys::cv_ml_LogisticRegression_getLearningRate_const(self.as_raw_LogisticRegression()) }.into_result()
    }
    
    /// @copybrief getLearningRate @see getLearningRate
    fn set_learning_rate(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_LogisticRegression_setLearningRate_double(self.as_raw_LogisticRegression(), val) }.into_result()
    }
    
    /// @see setIterations
    fn get_iterations(&self) -> Result<i32> {
        unsafe { sys::cv_ml_LogisticRegression_getIterations_const(self.as_raw_LogisticRegression()) }.into_result()
    }
    
    /// @copybrief getIterations @see getIterations
    fn set_iterations(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_LogisticRegression_setIterations_int(self.as_raw_LogisticRegression(), val) }.into_result()
    }
    
    /// @see setRegularization
    fn get_regularization(&self) -> Result<i32> {
        unsafe { sys::cv_ml_LogisticRegression_getRegularization_const(self.as_raw_LogisticRegression()) }.into_result()
    }
    
    /// @copybrief getRegularization @see getRegularization
    fn set_regularization(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_LogisticRegression_setRegularization_int(self.as_raw_LogisticRegression(), val) }.into_result()
    }
    
    /// @see setTrainMethod
    fn get_train_method(&self) -> Result<i32> {
        unsafe { sys::cv_ml_LogisticRegression_getTrainMethod_const(self.as_raw_LogisticRegression()) }.into_result()
    }
    
    /// @copybrief getTrainMethod @see getTrainMethod
    fn set_train_method(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_LogisticRegression_setTrainMethod_int(self.as_raw_LogisticRegression(), val) }.into_result()
    }
    
    /// @see setMiniBatchSize
    fn get_mini_batch_size(&self) -> Result<i32> {
        unsafe { sys::cv_ml_LogisticRegression_getMiniBatchSize_const(self.as_raw_LogisticRegression()) }.into_result()
    }
    
    /// @copybrief getMiniBatchSize @see getMiniBatchSize
    fn set_mini_batch_size(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_LogisticRegression_setMiniBatchSize_int(self.as_raw_LogisticRegression(), val) }.into_result()
    }
    
    /// @see setTermCriteria
    fn get_term_criteria(&self) -> Result<core::TermCriteria> {
        unsafe { sys::cv_ml_LogisticRegression_getTermCriteria_const(self.as_raw_LogisticRegression()) }.into_result().map(|ptr| core::TermCriteria { ptr })
    }
    
    /// @copybrief getTermCriteria @see getTermCriteria
    fn set_term_criteria(&mut self, val: &core::TermCriteria) -> Result<()> {
        unsafe { sys::cv_ml_LogisticRegression_setTermCriteria_TermCriteria(self.as_raw_LogisticRegression(), val.as_raw_TermCriteria()) }.into_result()
    }
    
    /// Predicts responses for input samples and returns a float type.
    ///
    /// ## Parameters
    /// * samples: The input data for the prediction algorithm. Matrix [m x n], where each row
    /// contains variables (features) of one object being classified. Should have data type CV_32F.
    /// * results: Predicted labels as a column matrix of type CV_32S.
    /// * flags: Not used.
    ///
    /// ## C++ default parameters
    /// * results: noArray()
    /// * flags: 0
    fn predict(&self, samples: &dyn core::ToInputArray, results: &mut dyn core::ToOutputArray, flags: i32) -> Result<f32> {
        input_array_arg!(samples);
        output_array_arg!(results);
        unsafe { sys::cv_ml_LogisticRegression_predict_const__InputArray__OutputArray_int(self.as_raw_LogisticRegression(), samples.as_raw__InputArray(), results.as_raw__OutputArray(), flags) }.into_result()
    }
    
    /// This function returns the trained parameters arranged across rows.
    ///
    /// For a two class classifcation problem, it returns a row matrix. It returns learnt parameters of
    /// the Logistic Regression as a matrix of type CV_32F.
    fn get_learnt_thetas(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_LogisticRegression_get_learnt_thetas_const(self.as_raw_LogisticRegression()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
}

impl dyn LogisticRegression + '_ {
    /// Creates empty model.
    ///
    /// Creates Logistic Regression model with parameters given.
    pub fn create() -> Result<types::PtrOfLogisticRegression> {
        unsafe { sys::cv_ml_LogisticRegression_create() }.into_result().map(|ptr| types::PtrOfLogisticRegression { ptr })
    }
    
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
    /// ## C++ default parameters
    /// * node_name: String()
    pub fn load(filepath: &str, node_name: &str) -> Result<types::PtrOfLogisticRegression> {
        string_arg!(filepath);
        string_arg!(node_name);
        unsafe { sys::cv_ml_LogisticRegression_load_String_String(filepath.as_ptr(), node_name.as_ptr()) }.into_result().map(|ptr| types::PtrOfLogisticRegression { ptr })
    }
    
}

// Generating impl for trait crate::ml::NormalBayesClassifier
/// Bayes classifier for normally distributed data.
///
/// ## See also
/// @ref ml_intro_bayes
pub trait NormalBayesClassifier: crate::ml::StatModel {
    fn as_raw_NormalBayesClassifier(&self) -> *mut c_void;
    /// Predicts the response for sample(s).
    ///
    /// The method estimates the most probable classes for input vectors. Input vectors (one or more)
    /// are stored as rows of the matrix inputs. In case of multiple input vectors, there should be one
    /// output vector outputs. The predicted class for a single input vector is returned by the method.
    /// The vector outputProbs contains the output probabilities corresponding to each element of
    /// result.
    ///
    /// ## C++ default parameters
    /// * flags: 0
    fn predict_prob(&self, inputs: &dyn core::ToInputArray, outputs: &mut dyn core::ToOutputArray, output_probs: &mut dyn core::ToOutputArray, flags: i32) -> Result<f32> {
        input_array_arg!(inputs);
        output_array_arg!(outputs);
        output_array_arg!(output_probs);
        unsafe { sys::cv_ml_NormalBayesClassifier_predictProb_const__InputArray__OutputArray__OutputArray_int(self.as_raw_NormalBayesClassifier(), inputs.as_raw__InputArray(), outputs.as_raw__OutputArray(), output_probs.as_raw__OutputArray(), flags) }.into_result()
    }
    
}

impl dyn NormalBayesClassifier + '_ {
    /// Creates empty model
    /// Use StatModel::train to train the model after creation.
    pub fn create() -> Result<types::PtrOfNormalBayesClassifier> {
        unsafe { sys::cv_ml_NormalBayesClassifier_create() }.into_result().map(|ptr| types::PtrOfNormalBayesClassifier { ptr })
    }
    
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
    /// ## C++ default parameters
    /// * node_name: String()
    pub fn load(filepath: &str, node_name: &str) -> Result<types::PtrOfNormalBayesClassifier> {
        string_arg!(filepath);
        string_arg!(node_name);
        unsafe { sys::cv_ml_NormalBayesClassifier_load_String_String(filepath.as_ptr(), node_name.as_ptr()) }.into_result().map(|ptr| types::PtrOfNormalBayesClassifier { ptr })
    }
    
}

// boxed class cv::ml::ParamGrid
/// The structure represents the logarithmic grid range of statmodel parameters.
///
/// It is used for optimizing statmodel accuracy by varying model parameters, the accuracy estimate
/// being computed by cross-validation.
pub struct ParamGrid {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for ParamGrid {
    fn drop(&mut self) {
        unsafe { sys::cv_ParamGrid_delete(self.ptr) };
    }
}

impl ParamGrid {
    #[inline(always)] pub fn as_raw_ParamGrid(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for ParamGrid {}

impl ParamGrid {
    /// Minimum value of the statmodel parameter. Default value is 0.
    pub fn min_val(&self) -> Result<f64> {
        unsafe { sys::cv_ml_ParamGrid_minVal_const(self.as_raw_ParamGrid()) }.into_result()
    }
    
    /// Minimum value of the statmodel parameter. Default value is 0.
    pub fn set_min_val(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_ParamGrid_set_minVal_double(self.as_raw_ParamGrid(), val) }.into_result()
    }
    
    /// Maximum value of the statmodel parameter. Default value is 0.
    pub fn max_val(&self) -> Result<f64> {
        unsafe { sys::cv_ml_ParamGrid_maxVal_const(self.as_raw_ParamGrid()) }.into_result()
    }
    
    /// Maximum value of the statmodel parameter. Default value is 0.
    pub fn set_max_val(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_ParamGrid_set_maxVal_double(self.as_raw_ParamGrid(), val) }.into_result()
    }
    
    /// Logarithmic step for iterating the statmodel parameter.
    ///
    /// The grid determines the following iteration sequence of the statmodel parameter values:
    /// ![block formula](https://latex.codecogs.com/png.latex?%28minVal%2C%20minVal%2Astep%2C%20minVal%2A%7Bstep%7D%5E2%2C%20%5Cdots%2C%20%20minVal%2A%7BlogStep%7D%5En%29%2C)
    /// where ![inline formula](https://latex.codecogs.com/png.latex?n) is the maximal index satisfying
    /// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BminVal%7D%20%2A%20%5Ctexttt%7BlogStep%7D%20%5En%20%3C%20%20%5Ctexttt%7BmaxVal%7D)
    /// The grid is logarithmic, so logStep must always be greater than 1. Default value is 1.
    pub fn log_step(&self) -> Result<f64> {
        unsafe { sys::cv_ml_ParamGrid_logStep_const(self.as_raw_ParamGrid()) }.into_result()
    }
    
    /// Logarithmic step for iterating the statmodel parameter.
    ///
    /// The grid determines the following iteration sequence of the statmodel parameter values:
    /// ![block formula](https://latex.codecogs.com/png.latex?%28minVal%2C%20minVal%2Astep%2C%20minVal%2A%7Bstep%7D%5E2%2C%20%5Cdots%2C%20%20minVal%2A%7BlogStep%7D%5En%29%2C)
    /// where ![inline formula](https://latex.codecogs.com/png.latex?n) is the maximal index satisfying
    /// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BminVal%7D%20%2A%20%5Ctexttt%7BlogStep%7D%20%5En%20%3C%20%20%5Ctexttt%7BmaxVal%7D)
    /// The grid is logarithmic, so logStep must always be greater than 1. Default value is 1.
    pub fn set_log_step(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_ParamGrid_set_logStep_double(self.as_raw_ParamGrid(), val) }.into_result()
    }
    
    /// Default constructor
    pub fn default() -> Result<crate::ml::ParamGrid> {
        unsafe { sys::cv_ml_ParamGrid_ParamGrid() }.into_result().map(|ptr| crate::ml::ParamGrid { ptr })
    }
    
    /// Constructor with parameters
    pub fn for_range(_min_val: f64, _max_val: f64, _log_step: f64) -> Result<crate::ml::ParamGrid> {
        unsafe { sys::cv_ml_ParamGrid_ParamGrid_double_double_double(_min_val, _max_val, _log_step) }.into_result().map(|ptr| crate::ml::ParamGrid { ptr })
    }
    
    /// Creates a ParamGrid Ptr that can be given to the %SVM::trainAuto method
    ///
    /// ## Parameters
    /// * minVal: minimum value of the parameter grid
    /// * maxVal: maximum value of the parameter grid
    /// * logstep: Logarithmic step for iterating the statmodel parameter
    ///
    /// ## C++ default parameters
    /// * min_val: 0.
    /// * max_val: 0.
    /// * logstep: 1.
    pub fn create(min_val: f64, max_val: f64, logstep: f64) -> Result<types::PtrOfParamGrid> {
        unsafe { sys::cv_ml_ParamGrid_create_double_double_double(min_val, max_val, logstep) }.into_result().map(|ptr| types::PtrOfParamGrid { ptr })
    }
    
}

// Generating impl for trait crate::ml::RTrees
/// The class implements the random forest predictor.
///
/// ## See also
/// @ref ml_intro_rtrees
pub trait RTrees: crate::ml::DTrees {
    fn as_raw_RTrees(&self) -> *mut c_void;
    /// @see setCalculateVarImportance
    fn get_calculate_var_importance(&self) -> Result<bool> {
        unsafe { sys::cv_ml_RTrees_getCalculateVarImportance_const(self.as_raw_RTrees()) }.into_result()
    }
    
    /// @copybrief getCalculateVarImportance @see getCalculateVarImportance
    fn set_calculate_var_importance(&mut self, val: bool) -> Result<()> {
        unsafe { sys::cv_ml_RTrees_setCalculateVarImportance_bool(self.as_raw_RTrees(), val) }.into_result()
    }
    
    /// @see setActiveVarCount
    fn get_active_var_count(&self) -> Result<i32> {
        unsafe { sys::cv_ml_RTrees_getActiveVarCount_const(self.as_raw_RTrees()) }.into_result()
    }
    
    /// @copybrief getActiveVarCount @see getActiveVarCount
    fn set_active_var_count(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_RTrees_setActiveVarCount_int(self.as_raw_RTrees(), val) }.into_result()
    }
    
    /// @see setTermCriteria
    fn get_term_criteria(&self) -> Result<core::TermCriteria> {
        unsafe { sys::cv_ml_RTrees_getTermCriteria_const(self.as_raw_RTrees()) }.into_result().map(|ptr| core::TermCriteria { ptr })
    }
    
    /// @copybrief getTermCriteria @see getTermCriteria
    fn set_term_criteria(&mut self, val: &core::TermCriteria) -> Result<()> {
        unsafe { sys::cv_ml_RTrees_setTermCriteria_TermCriteria(self.as_raw_RTrees(), val.as_raw_TermCriteria()) }.into_result()
    }
    
    /// Returns the variable importance array.
    /// The method returns the variable importance vector, computed at the training stage when
    /// CalculateVarImportance is set to true. If this flag was set to false, the empty matrix is
    /// returned.
    fn get_var_importance(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_RTrees_getVarImportance_const(self.as_raw_RTrees()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Returns the result of each individual tree in the forest.
    /// In case the model is a regression problem, the method will return each of the trees'
    /// results for each of the sample cases. If the model is a classifier, it will return
    /// a Mat with samples + 1 rows, where the first row gives the class number and the
    /// following rows return the votes each class had for each sample.
    /// ## Parameters
    /// * samples: Array containing the samples for which votes will be calculated.
    /// * results: Array where the result of the calculation will be written.
    /// * flags: Flags for defining the type of RTrees.
    fn get_votes(&self, samples: &dyn core::ToInputArray, results: &mut dyn core::ToOutputArray, flags: i32) -> Result<()> {
        input_array_arg!(samples);
        output_array_arg!(results);
        unsafe { sys::cv_ml_RTrees_getVotes_const__InputArray__OutputArray_int(self.as_raw_RTrees(), samples.as_raw__InputArray(), results.as_raw__OutputArray(), flags) }.into_result()
    }
    
}

impl dyn RTrees + '_ {
    /// Creates the empty model.
    /// Use StatModel::train to train the model, StatModel::train to create and train the model,
    /// Algorithm::load to load the pre-trained model.
    pub fn create() -> Result<types::PtrOfRTrees> {
        unsafe { sys::cv_ml_RTrees_create() }.into_result().map(|ptr| types::PtrOfRTrees { ptr })
    }
    
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
    /// ## C++ default parameters
    /// * node_name: String()
    pub fn load(filepath: &str, node_name: &str) -> Result<types::PtrOfRTrees> {
        string_arg!(filepath);
        string_arg!(node_name);
        unsafe { sys::cv_ml_RTrees_load_String_String(filepath.as_ptr(), node_name.as_ptr()) }.into_result().map(|ptr| types::PtrOfRTrees { ptr })
    }
    
}

// Generating impl for trait crate::ml::SVM
/// Support Vector Machines.
///
/// ## See also
/// @ref ml_intro_svm
pub trait SVM: crate::ml::StatModel {
    fn as_raw_SVM(&self) -> *mut c_void;
    /// @see setType
    fn get_type(&self) -> Result<i32> {
        unsafe { sys::cv_ml_SVM_getType_const(self.as_raw_SVM()) }.into_result()
    }
    
    /// @copybrief getType @see getType
    fn set_type(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_ml_SVM_setType_int(self.as_raw_SVM(), val) }.into_result()
    }
    
    /// @see setGamma
    fn get_gamma(&self) -> Result<f64> {
        unsafe { sys::cv_ml_SVM_getGamma_const(self.as_raw_SVM()) }.into_result()
    }
    
    /// @copybrief getGamma @see getGamma
    fn set_gamma(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_SVM_setGamma_double(self.as_raw_SVM(), val) }.into_result()
    }
    
    /// @see setCoef0
    fn get_coef0(&self) -> Result<f64> {
        unsafe { sys::cv_ml_SVM_getCoef0_const(self.as_raw_SVM()) }.into_result()
    }
    
    /// @copybrief getCoef0 @see getCoef0
    fn set_coef0(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_SVM_setCoef0_double(self.as_raw_SVM(), val) }.into_result()
    }
    
    /// @see setDegree
    fn get_degree(&self) -> Result<f64> {
        unsafe { sys::cv_ml_SVM_getDegree_const(self.as_raw_SVM()) }.into_result()
    }
    
    /// @copybrief getDegree @see getDegree
    fn set_degree(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_SVM_setDegree_double(self.as_raw_SVM(), val) }.into_result()
    }
    
    /// @see setC
    fn get_c(&self) -> Result<f64> {
        unsafe { sys::cv_ml_SVM_getC_const(self.as_raw_SVM()) }.into_result()
    }
    
    /// @copybrief getC @see getC
    fn set_c(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_SVM_setC_double(self.as_raw_SVM(), val) }.into_result()
    }
    
    /// @see setNu
    fn get_nu(&self) -> Result<f64> {
        unsafe { sys::cv_ml_SVM_getNu_const(self.as_raw_SVM()) }.into_result()
    }
    
    /// @copybrief getNu @see getNu
    fn set_nu(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_SVM_setNu_double(self.as_raw_SVM(), val) }.into_result()
    }
    
    /// @see setP
    fn get_p(&self) -> Result<f64> {
        unsafe { sys::cv_ml_SVM_getP_const(self.as_raw_SVM()) }.into_result()
    }
    
    /// @copybrief getP @see getP
    fn set_p(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_ml_SVM_setP_double(self.as_raw_SVM(), val) }.into_result()
    }
    
    /// @see setClassWeights
    fn get_class_weights(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_SVM_getClassWeights_const(self.as_raw_SVM()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// @copybrief getClassWeights @see getClassWeights
    fn set_class_weights(&mut self, val: &core::Mat) -> Result<()> {
        unsafe { sys::cv_ml_SVM_setClassWeights_Mat(self.as_raw_SVM(), val.as_raw_Mat()) }.into_result()
    }
    
    /// @see setTermCriteria
    fn get_term_criteria(&self) -> Result<core::TermCriteria> {
        unsafe { sys::cv_ml_SVM_getTermCriteria_const(self.as_raw_SVM()) }.into_result().map(|ptr| core::TermCriteria { ptr })
    }
    
    /// @copybrief getTermCriteria @see getTermCriteria
    fn set_term_criteria(&mut self, val: &core::TermCriteria) -> Result<()> {
        unsafe { sys::cv_ml_SVM_setTermCriteria_TermCriteria(self.as_raw_SVM(), val.as_raw_TermCriteria()) }.into_result()
    }
    
    /// Type of a %SVM kernel.
    /// See SVM::KernelTypes. Default value is SVM::RBF.
    fn get_kernel_type(&self) -> Result<i32> {
        unsafe { sys::cv_ml_SVM_getKernelType_const(self.as_raw_SVM()) }.into_result()
    }
    
    /// Initialize with one of predefined kernels.
    /// See SVM::KernelTypes.
    fn set_kernel(&mut self, kernel_type: i32) -> Result<()> {
        unsafe { sys::cv_ml_SVM_setKernel_int(self.as_raw_SVM(), kernel_type) }.into_result()
    }
    
    /// Initialize with custom kernel.
    /// See SVM::Kernel class for implementation details
    fn set_custom_kernel(&mut self, _kernel: &types::PtrOfKernel) -> Result<()> {
        unsafe { sys::cv_ml_SVM_setCustomKernel_PtrOfKernel(self.as_raw_SVM(), _kernel.as_raw_PtrOfKernel()) }.into_result()
    }
    
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
    /// ## C++ default parameters
    /// * k_fold: 10
    /// * cgrid: getDefaultGrid(C)
    /// * gamma_grid: getDefaultGrid(GAMMA)
    /// * p_grid: getDefaultGrid(P)
    /// * nu_grid: getDefaultGrid(NU)
    /// * coeff_grid: getDefaultGrid(COEF)
    /// * degree_grid: getDefaultGrid(DEGREE)
    /// * balanced: false
    fn train_auto_with_data(&mut self, data: &types::PtrOfTrainData, k_fold: i32, cgrid: &crate::ml::ParamGrid, gamma_grid: &crate::ml::ParamGrid, p_grid: &crate::ml::ParamGrid, nu_grid: &crate::ml::ParamGrid, coeff_grid: &crate::ml::ParamGrid, degree_grid: &crate::ml::ParamGrid, balanced: bool) -> Result<bool> {
        unsafe { sys::cv_ml_SVM_trainAuto_PtrOfTrainData_int_ParamGrid_ParamGrid_ParamGrid_ParamGrid_ParamGrid_ParamGrid_bool(self.as_raw_SVM(), data.as_raw_PtrOfTrainData(), k_fold, cgrid.as_raw_ParamGrid(), gamma_grid.as_raw_ParamGrid(), p_grid.as_raw_ParamGrid(), nu_grid.as_raw_ParamGrid(), coeff_grid.as_raw_ParamGrid(), degree_grid.as_raw_ParamGrid(), balanced) }.into_result()
    }
    
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
    /// ## C++ default parameters
    /// * k_fold: 10
    /// * cgrid: SVM::getDefaultGridPtr(SVM::C)
    /// * gamma_grid: SVM::getDefaultGridPtr(SVM::GAMMA)
    /// * p_grid: SVM::getDefaultGridPtr(SVM::P)
    /// * nu_grid: SVM::getDefaultGridPtr(SVM::NU)
    /// * coeff_grid: SVM::getDefaultGridPtr(SVM::COEF)
    /// * degree_grid: SVM::getDefaultGridPtr(SVM::DEGREE)
    /// * balanced: false
    fn train_auto(&mut self, samples: &dyn core::ToInputArray, layout: i32, responses: &dyn core::ToInputArray, k_fold: i32, cgrid: &types::PtrOfParamGrid, gamma_grid: &types::PtrOfParamGrid, p_grid: &types::PtrOfParamGrid, nu_grid: &types::PtrOfParamGrid, coeff_grid: &types::PtrOfParamGrid, degree_grid: &types::PtrOfParamGrid, balanced: bool) -> Result<bool> {
        input_array_arg!(samples);
        input_array_arg!(responses);
        unsafe { sys::cv_ml_SVM_trainAuto__InputArray_int__InputArray_int_PtrOfParamGrid_PtrOfParamGrid_PtrOfParamGrid_PtrOfParamGrid_PtrOfParamGrid_PtrOfParamGrid_bool(self.as_raw_SVM(), samples.as_raw__InputArray(), layout, responses.as_raw__InputArray(), k_fold, cgrid.as_raw_PtrOfParamGrid(), gamma_grid.as_raw_PtrOfParamGrid(), p_grid.as_raw_PtrOfParamGrid(), nu_grid.as_raw_PtrOfParamGrid(), coeff_grid.as_raw_PtrOfParamGrid(), degree_grid.as_raw_PtrOfParamGrid(), balanced) }.into_result()
    }
    
    /// Retrieves all the support vectors
    ///
    /// The method returns all the support vectors as a floating-point matrix, where support vectors are
    /// stored as matrix rows.
    fn get_support_vectors(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_SVM_getSupportVectors_const(self.as_raw_SVM()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Retrieves all the uncompressed support vectors of a linear %SVM
    ///
    /// The method returns all the uncompressed support vectors of a linear %SVM that the compressed
    /// support vector, used for prediction, was derived from. They are returned in a floating-point
    /// matrix, where the support vectors are stored as matrix rows.
    fn get_uncompressed_support_vectors(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_SVM_getUncompressedSupportVectors_const(self.as_raw_SVM()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Retrieves the decision function
    ///
    /// ## Parameters
    /// * i: the index of the decision function. If the problem solved is regression, 1-class or
    /// 2-class classification, then there will be just one decision function and the index should
    /// always be 0. Otherwise, in the case of N-class classification, there will be ![inline formula](https://latex.codecogs.com/png.latex?N%28N-1%29%2F2)
    /// decision functions.
    /// * alpha: the optional output vector for weights, corresponding to different support vectors.
    /// In the case of linear %SVM all the alpha's will be 1's.
    /// * svidx: the optional output vector of indices of support vectors within the matrix of
    /// support vectors (which can be retrieved by SVM::getSupportVectors). In the case of linear
    /// %SVM each decision function consists of a single "compressed" support vector.
    ///
    /// The method returns rho parameter of the decision function, a scalar subtracted from the weighted
    /// sum of kernel responses.
    fn get_decision_function(&self, i: i32, alpha: &mut dyn core::ToOutputArray, svidx: &mut dyn core::ToOutputArray) -> Result<f64> {
        output_array_arg!(alpha);
        output_array_arg!(svidx);
        unsafe { sys::cv_ml_SVM_getDecisionFunction_const_int__OutputArray__OutputArray(self.as_raw_SVM(), i, alpha.as_raw__OutputArray(), svidx.as_raw__OutputArray()) }.into_result()
    }
    
}

impl dyn SVM + '_ {
    /// Generates a grid for %SVM parameters.
    ///
    /// ## Parameters
    /// * param_id: %SVM parameters IDs that must be one of the SVM::ParamTypes. The grid is
    /// generated for the parameter with this ID.
    ///
    /// The function generates a grid for the specified parameter of the %SVM algorithm. The grid may be
    /// passed to the function SVM::trainAuto.
    pub fn get_default_grid(param_id: i32) -> Result<crate::ml::ParamGrid> {
        unsafe { sys::cv_ml_SVM_getDefaultGrid_int(param_id) }.into_result().map(|ptr| crate::ml::ParamGrid { ptr })
    }
    
    /// Generates a grid for %SVM parameters.
    ///
    /// ## Parameters
    /// * param_id: %SVM parameters IDs that must be one of the SVM::ParamTypes. The grid is
    /// generated for the parameter with this ID.
    ///
    /// The function generates a grid pointer for the specified parameter of the %SVM algorithm.
    /// The grid may be passed to the function SVM::trainAuto.
    pub fn get_default_grid_ptr(param_id: i32) -> Result<types::PtrOfParamGrid> {
        unsafe { sys::cv_ml_SVM_getDefaultGridPtr_int(param_id) }.into_result().map(|ptr| types::PtrOfParamGrid { ptr })
    }
    
    /// Creates empty model.
    /// Use StatModel::train to train the model. Since %SVM has several parameters, you may want to
    /// find the best parameters for your problem, it can be done with SVM::trainAuto.
    pub fn create() -> Result<types::PtrOfSVM> {
        unsafe { sys::cv_ml_SVM_create() }.into_result().map(|ptr| types::PtrOfSVM { ptr })
    }
    
    /// Loads and creates a serialized svm from a file
    ///
    /// Use SVM::save to serialize and store an SVM to disk.
    /// Load the SVM from this file again, by calling this function with the path to the file.
    ///
    /// ## Parameters
    /// * filepath: path to serialized svm
    pub fn load(filepath: &str) -> Result<types::PtrOfSVM> {
        string_arg!(filepath);
        unsafe { sys::cv_ml_SVM_load_String(filepath.as_ptr()) }.into_result().map(|ptr| types::PtrOfSVM { ptr })
    }
    
}

// Generating impl for trait crate::ml::SVM_Kernel
pub trait SVM_Kernel: core::AlgorithmTrait {
    fn as_raw_SVM_Kernel(&self) -> *mut c_void;
    fn get_type(&self) -> Result<i32> {
        unsafe { sys::cv_ml_SVM_Kernel_getType_const(self.as_raw_SVM_Kernel()) }.into_result()
    }
    
    fn calc(&mut self, vcount: i32, n: i32, vecs: &f32, another: &f32, results: &mut f32) -> Result<()> {
        unsafe { sys::cv_ml_SVM_Kernel_calc_int_int_const_float_X_const_float_X_float_X(self.as_raw_SVM_Kernel(), vcount, n, vecs, another, results) }.into_result()
    }
    
}

// Generating impl for trait crate::ml::SVMSGD
/// \
///                        Stochastic Gradient Descent SVM Classifier                      *
pub trait SVMSGD: crate::ml::StatModel {
    fn as_raw_SVMSGD(&self) -> *mut c_void;
    /// ## Returns
    /// the weights of the trained model (decision function f(x) = weights * x + shift).
    fn get_weights(&mut self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_SVMSGD_getWeights(self.as_raw_SVMSGD()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// ## Returns
    /// the shift of the trained model (decision function f(x) = weights * x + shift).
    fn get_shift(&mut self) -> Result<f32> {
        unsafe { sys::cv_ml_SVMSGD_getShift(self.as_raw_SVMSGD()) }.into_result()
    }
    
    /// Function sets optimal parameters values for chosen SVM SGD model.
    /// ## Parameters
    /// * svmsgdType: is the type of SVMSGD classifier.
    /// * marginType: is the type of margin constraint.
    ///
    /// ## C++ default parameters
    /// * svmsgd_type: SVMSGD::ASGD
    /// * margin_type: SVMSGD::SOFT_MARGIN
    fn set_optimal_parameters(&mut self, svmsgd_type: i32, margin_type: i32) -> Result<()> {
        unsafe { sys::cv_ml_SVMSGD_setOptimalParameters_int_int(self.as_raw_SVMSGD(), svmsgd_type, margin_type) }.into_result()
    }
    
    /// @see setSvmsgdType
    fn get_svmsgd_type(&self) -> Result<i32> {
        unsafe { sys::cv_ml_SVMSGD_getSvmsgdType_const(self.as_raw_SVMSGD()) }.into_result()
    }
    
    /// @copybrief getSvmsgdType @see getSvmsgdType
    fn set_svmsgd_type(&mut self, svmsgd_type: i32) -> Result<()> {
        unsafe { sys::cv_ml_SVMSGD_setSvmsgdType_int(self.as_raw_SVMSGD(), svmsgd_type) }.into_result()
    }
    
    /// @see setMarginType
    fn get_margin_type(&self) -> Result<i32> {
        unsafe { sys::cv_ml_SVMSGD_getMarginType_const(self.as_raw_SVMSGD()) }.into_result()
    }
    
    /// @copybrief getMarginType @see getMarginType
    fn set_margin_type(&mut self, margin_type: i32) -> Result<()> {
        unsafe { sys::cv_ml_SVMSGD_setMarginType_int(self.as_raw_SVMSGD(), margin_type) }.into_result()
    }
    
    /// @see setMarginRegularization
    fn get_margin_regularization(&self) -> Result<f32> {
        unsafe { sys::cv_ml_SVMSGD_getMarginRegularization_const(self.as_raw_SVMSGD()) }.into_result()
    }
    
    /// @copybrief getMarginRegularization @see getMarginRegularization
    fn set_margin_regularization(&mut self, margin_regularization: f32) -> Result<()> {
        unsafe { sys::cv_ml_SVMSGD_setMarginRegularization_float(self.as_raw_SVMSGD(), margin_regularization) }.into_result()
    }
    
    /// @see setInitialStepSize
    fn get_initial_step_size(&self) -> Result<f32> {
        unsafe { sys::cv_ml_SVMSGD_getInitialStepSize_const(self.as_raw_SVMSGD()) }.into_result()
    }
    
    /// @copybrief getInitialStepSize @see getInitialStepSize
    fn set_initial_step_size(&mut self, initial_step_size: f32) -> Result<()> {
        unsafe { sys::cv_ml_SVMSGD_setInitialStepSize_float(self.as_raw_SVMSGD(), initial_step_size) }.into_result()
    }
    
    /// @see setStepDecreasingPower
    fn get_step_decreasing_power(&self) -> Result<f32> {
        unsafe { sys::cv_ml_SVMSGD_getStepDecreasingPower_const(self.as_raw_SVMSGD()) }.into_result()
    }
    
    /// @copybrief getStepDecreasingPower @see getStepDecreasingPower
    fn set_step_decreasing_power(&mut self, step_decreasing_power: f32) -> Result<()> {
        unsafe { sys::cv_ml_SVMSGD_setStepDecreasingPower_float(self.as_raw_SVMSGD(), step_decreasing_power) }.into_result()
    }
    
    /// @see setTermCriteria
    fn get_term_criteria(&self) -> Result<core::TermCriteria> {
        unsafe { sys::cv_ml_SVMSGD_getTermCriteria_const(self.as_raw_SVMSGD()) }.into_result().map(|ptr| core::TermCriteria { ptr })
    }
    
    /// @copybrief getTermCriteria @see getTermCriteria
    fn set_term_criteria(&mut self, val: &core::TermCriteria) -> Result<()> {
        unsafe { sys::cv_ml_SVMSGD_setTermCriteria_TermCriteria(self.as_raw_SVMSGD(), val.as_raw_TermCriteria()) }.into_result()
    }
    
}

impl dyn SVMSGD + '_ {
    /// Creates empty model.
    /// Use StatModel::train to train the model. Since %SVMSGD has several parameters, you may want to
    /// find the best parameters for your problem or use setOptimalParameters() to set some default parameters.
    pub fn create() -> Result<types::PtrOfSVMSGD> {
        unsafe { sys::cv_ml_SVMSGD_create() }.into_result().map(|ptr| types::PtrOfSVMSGD { ptr })
    }
    
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
    /// ## C++ default parameters
    /// * node_name: String()
    pub fn load(filepath: &str, node_name: &str) -> Result<types::PtrOfSVMSGD> {
        string_arg!(filepath);
        string_arg!(node_name);
        unsafe { sys::cv_ml_SVMSGD_load_String_String(filepath.as_ptr(), node_name.as_ptr()) }.into_result().map(|ptr| types::PtrOfSVMSGD { ptr })
    }
    
}

// Generating impl for trait crate::ml::StatModel
/// Base class for statistical models in OpenCV ML.
pub trait StatModel: core::AlgorithmTrait {
    fn as_raw_StatModel(&self) -> *mut c_void;
    /// Returns the number of variables in training samples
    fn get_var_count(&self) -> Result<i32> {
        unsafe { sys::cv_ml_StatModel_getVarCount_const(self.as_raw_StatModel()) }.into_result()
    }
    
    fn empty(&self) -> Result<bool> {
        unsafe { sys::cv_ml_StatModel_empty_const(self.as_raw_StatModel()) }.into_result()
    }
    
    /// Returns true if the model is trained
    fn is_trained(&self) -> Result<bool> {
        unsafe { sys::cv_ml_StatModel_isTrained_const(self.as_raw_StatModel()) }.into_result()
    }
    
    /// Returns true if the model is classifier
    fn is_classifier(&self) -> Result<bool> {
        unsafe { sys::cv_ml_StatModel_isClassifier_const(self.as_raw_StatModel()) }.into_result()
    }
    
    /// Trains the statistical model
    ///
    /// ## Parameters
    /// * trainData: training data that can be loaded from file using TrainData::loadFromCSV or
    /// created with TrainData::create.
    /// * flags: optional flags, depending on the model. Some of the models can be updated with the
    /// new training samples, not completely overwritten (such as NormalBayesClassifier or ANN_MLP).
    ///
    /// ## C++ default parameters
    /// * flags: 0
    fn train_with_data(&mut self, train_data: &types::PtrOfTrainData, flags: i32) -> Result<bool> {
        unsafe { sys::cv_ml_StatModel_train_PtrOfTrainData_int(self.as_raw_StatModel(), train_data.as_raw_PtrOfTrainData(), flags) }.into_result()
    }
    
    /// Trains the statistical model
    ///
    /// ## Parameters
    /// * samples: training samples
    /// * layout: See ml::SampleTypes.
    /// * responses: vector of responses associated with the training samples.
    fn train(&mut self, samples: &dyn core::ToInputArray, layout: i32, responses: &dyn core::ToInputArray) -> Result<bool> {
        input_array_arg!(samples);
        input_array_arg!(responses);
        unsafe { sys::cv_ml_StatModel_train__InputArray_int__InputArray(self.as_raw_StatModel(), samples.as_raw__InputArray(), layout, responses.as_raw__InputArray()) }.into_result()
    }
    
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
    fn calc_error(&self, data: &types::PtrOfTrainData, test: bool, resp: &mut dyn core::ToOutputArray) -> Result<f32> {
        output_array_arg!(resp);
        unsafe { sys::cv_ml_StatModel_calcError_const_PtrOfTrainData_bool__OutputArray(self.as_raw_StatModel(), data.as_raw_PtrOfTrainData(), test, resp.as_raw__OutputArray()) }.into_result()
    }
    
    /// Predicts response(s) for the provided sample(s)
    ///
    /// ## Parameters
    /// * samples: The input samples, floating-point matrix
    /// * results: The optional output matrix of results.
    /// * flags: The optional flags, model-dependent. See cv::ml::StatModel::Flags.
    ///
    /// ## C++ default parameters
    /// * results: noArray()
    /// * flags: 0
    fn predict(&self, samples: &dyn core::ToInputArray, results: &mut dyn core::ToOutputArray, flags: i32) -> Result<f32> {
        input_array_arg!(samples);
        output_array_arg!(results);
        unsafe { sys::cv_ml_StatModel_predict_const__InputArray__OutputArray_int(self.as_raw_StatModel(), samples.as_raw__InputArray(), results.as_raw__OutputArray(), flags) }.into_result()
    }
    
}

// Generating impl for trait crate::ml::TrainData
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
    fn as_raw_TrainData(&self) -> *mut c_void;
    fn missing_value(&mut self) -> Result<f32> {
        unsafe { sys::cv_ml_TrainData_missingValue(self.as_raw_TrainData()) }.into_result()
    }
    
    fn get_layout(&self) -> Result<i32> {
        unsafe { sys::cv_ml_TrainData_getLayout_const(self.as_raw_TrainData()) }.into_result()
    }
    
    fn get_n_train_samples(&self) -> Result<i32> {
        unsafe { sys::cv_ml_TrainData_getNTrainSamples_const(self.as_raw_TrainData()) }.into_result()
    }
    
    fn get_n_test_samples(&self) -> Result<i32> {
        unsafe { sys::cv_ml_TrainData_getNTestSamples_const(self.as_raw_TrainData()) }.into_result()
    }
    
    fn get_n_samples(&self) -> Result<i32> {
        unsafe { sys::cv_ml_TrainData_getNSamples_const(self.as_raw_TrainData()) }.into_result()
    }
    
    fn get_n_vars(&self) -> Result<i32> {
        unsafe { sys::cv_ml_TrainData_getNVars_const(self.as_raw_TrainData()) }.into_result()
    }
    
    fn get_n_all_vars(&self) -> Result<i32> {
        unsafe { sys::cv_ml_TrainData_getNAllVars_const(self.as_raw_TrainData()) }.into_result()
    }
    
    fn get_sample(&self, var_idx: &dyn core::ToInputArray, sidx: i32, buf: &mut f32) -> Result<()> {
        input_array_arg!(var_idx);
        unsafe { sys::cv_ml_TrainData_getSample_const__InputArray_int_float_X(self.as_raw_TrainData(), var_idx.as_raw__InputArray(), sidx, buf) }.into_result()
    }
    
    fn get_samples(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getSamples_const(self.as_raw_TrainData()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_missing(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getMissing_const(self.as_raw_TrainData()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
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
    /// ## C++ default parameters
    /// * layout: ROW_SAMPLE
    /// * compress_samples: true
    /// * compress_vars: true
    fn get_train_samples(&self, layout: i32, compress_samples: bool, compress_vars: bool) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getTrainSamples_const_int_bool_bool(self.as_raw_TrainData(), layout, compress_samples, compress_vars) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Returns the vector of responses
    ///
    /// The function returns ordered or the original categorical responses. Usually it's used in
    /// regression algorithms.
    fn get_train_responses(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getTrainResponses_const(self.as_raw_TrainData()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Returns the vector of normalized categorical responses
    ///
    /// The function returns vector of responses. Each response is integer from `0` to `<number of
    /// classes>-1`. The actual label value can be retrieved then from the class label vector, see
    /// TrainData::getClassLabels.
    fn get_train_norm_cat_responses(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getTrainNormCatResponses_const(self.as_raw_TrainData()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_test_responses(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getTestResponses_const(self.as_raw_TrainData()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_test_norm_cat_responses(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getTestNormCatResponses_const(self.as_raw_TrainData()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_responses(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getResponses_const(self.as_raw_TrainData()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_norm_cat_responses(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getNormCatResponses_const(self.as_raw_TrainData()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_sample_weights(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getSampleWeights_const(self.as_raw_TrainData()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_train_sample_weights(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getTrainSampleWeights_const(self.as_raw_TrainData()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_test_sample_weights(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getTestSampleWeights_const(self.as_raw_TrainData()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_var_idx(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getVarIdx_const(self.as_raw_TrainData()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_var_type(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getVarType_const(self.as_raw_TrainData()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_var_symbol_flags(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getVarSymbolFlags_const(self.as_raw_TrainData()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_response_type(&self) -> Result<i32> {
        unsafe { sys::cv_ml_TrainData_getResponseType_const(self.as_raw_TrainData()) }.into_result()
    }
    
    fn get_train_sample_idx(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getTrainSampleIdx_const(self.as_raw_TrainData()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_test_sample_idx(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getTestSampleIdx_const(self.as_raw_TrainData()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_values(&self, vi: i32, sidx: &dyn core::ToInputArray, values: &mut f32) -> Result<()> {
        input_array_arg!(sidx);
        unsafe { sys::cv_ml_TrainData_getValues_const_int__InputArray_float_X(self.as_raw_TrainData(), vi, sidx.as_raw__InputArray(), values) }.into_result()
    }
    
    fn get_norm_cat_values(&self, vi: i32, sidx: &dyn core::ToInputArray, values: &mut i32) -> Result<()> {
        input_array_arg!(sidx);
        unsafe { sys::cv_ml_TrainData_getNormCatValues_const_int__InputArray_int_X(self.as_raw_TrainData(), vi, sidx.as_raw__InputArray(), values) }.into_result()
    }
    
    fn get_default_subst_values(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getDefaultSubstValues_const(self.as_raw_TrainData()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_cat_count(&self, vi: i32) -> Result<i32> {
        unsafe { sys::cv_ml_TrainData_getCatCount_const_int(self.as_raw_TrainData(), vi) }.into_result()
    }
    
    /// Returns the vector of class labels
    ///
    /// The function returns vector of unique labels occurred in the responses.
    fn get_class_labels(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getClassLabels_const(self.as_raw_TrainData()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_cat_ofs(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getCatOfs_const(self.as_raw_TrainData()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_cat_map(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getCatMap_const(self.as_raw_TrainData()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Splits the training data into the training and test parts
    /// ## See also
    /// TrainData::setTrainTestSplitRatio
    ///
    /// ## C++ default parameters
    /// * shuffle: true
    fn set_train_test_split(&mut self, count: i32, shuffle: bool) -> Result<()> {
        unsafe { sys::cv_ml_TrainData_setTrainTestSplit_int_bool(self.as_raw_TrainData(), count, shuffle) }.into_result()
    }
    
    /// Splits the training data into the training and test parts
    ///
    /// The function selects a subset of specified relative size and then returns it as the training
    /// set. If the function is not called, all the data is used for training. Please, note that for
    /// each of TrainData::getTrain\* there is corresponding TrainData::getTest\*, so that the test
    /// subset can be retrieved and processed as well.
    /// ## See also
    /// TrainData::setTrainTestSplit
    ///
    /// ## C++ default parameters
    /// * shuffle: true
    fn set_train_test_split_ratio(&mut self, ratio: f64, shuffle: bool) -> Result<()> {
        unsafe { sys::cv_ml_TrainData_setTrainTestSplitRatio_double_bool(self.as_raw_TrainData(), ratio, shuffle) }.into_result()
    }
    
    fn shuffle_train_test(&mut self) -> Result<()> {
        unsafe { sys::cv_ml_TrainData_shuffleTrainTest(self.as_raw_TrainData()) }.into_result()
    }
    
    /// Returns matrix of test samples
    fn get_test_samples(&self) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getTestSamples_const(self.as_raw_TrainData()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Returns vector of symbolic names captured in loadFromCSV()
    fn get_names(&self, names: &mut types::VectorOfString) -> Result<()> {
        unsafe { sys::cv_ml_TrainData_getNames_const_VectorOfString(self.as_raw_TrainData(), names.as_raw_VectorOfString()) }.into_result()
    }
    
}

impl dyn TrainData + '_ {
    /// Extract from 1D vector elements specified by passed indexes.
    /// ## Parameters
    /// * vec: input vector (supported types: CV_32S, CV_32F, CV_64F)
    /// * idx: 1D index vector
    pub fn get_sub_vector(vec: &core::Mat, idx: &core::Mat) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getSubVector_Mat_Mat(vec.as_raw_Mat(), idx.as_raw_Mat()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Extract from matrix rows/cols specified by passed indexes.
    /// ## Parameters
    /// * matrix: input matrix (supported types: CV_32S, CV_32F, CV_64F)
    /// * idx: 1D index vector
    /// * layout: specifies to extract rows (cv::ml::ROW_SAMPLES) or to extract columns (cv::ml::COL_SAMPLES)
    pub fn get_sub_matrix(matrix: &core::Mat, idx: &core::Mat, layout: i32) -> Result<core::Mat> {
        unsafe { sys::cv_ml_TrainData_getSubMatrix_Mat_Mat_int(matrix.as_raw_Mat(), idx.as_raw_Mat(), layout) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Reads the dataset from a .csv file and returns the ready-to-use training data.
    ///
    /// ## Parameters
    /// * filename: The input file name
    /// * headerLineCount: The number of lines in the beginning to skip; besides the header, the
    /// function also skips empty lines and lines staring with `#`
    /// * responseStartIdx: Index of the first output variable. If -1, the function considers the
    /// last variable as the response
    /// * responseEndIdx: Index of the last output variable + 1. If -1, then there is single
    /// response variable at responseStartIdx.
    /// * varTypeSpec: The optional text string that specifies the variables' types. It has the
    /// format `ord[n1-n2,n3,n4-n5,...]cat[n6,n7-n8,...]`. That is, variables from `n1 to n2`
    /// (inclusive range), `n3`, `n4 to n5` ... are considered ordered and `n6`, `n7 to n8` ... are
    /// considered as categorical. The range `[n1..n2] + [n3] + [n4..n5] + ... + [n6] + [n7..n8]`
    /// should cover all the variables. If varTypeSpec is not specified, then algorithm uses the
    /// following rules:
    /// - all input variables are considered ordered by default. If some column contains has non-
    /// numerical values, e.g. 'apple', 'pear', 'apple', 'apple', 'mango', the corresponding
    /// variable is considered categorical.
    /// - if there are several output variables, they are all considered as ordered. Error is
    /// reported when non-numerical values are used.
    /// - if there is a single output variable, then if its values are non-numerical or are all
    /// integers, then it's considered categorical. Otherwise, it's considered ordered.
    /// * delimiter: The character used to separate values in each line.
    /// * missch: The character used to specify missing measurements. It should not be a digit.
    /// Although it's a non-numerical value, it surely does not affect the decision of whether the
    /// variable ordered or categorical.
    ///
    /// Note: If the dataset only contains input variables and no responses, use responseStartIdx = -2
    /// and responseEndIdx = 0. The output variables vector will just contain zeros.
    ///
    /// ## C++ default parameters
    /// * response_start_idx: -1
    /// * response_end_idx: -1
    /// * var_type_spec: String()
    /// * delimiter: ','
    /// * missch: '?'
    pub fn load_from_csv(filename: &str, header_line_count: i32, response_start_idx: i32, response_end_idx: i32, var_type_spec: &str, delimiter: i8, missch: i8) -> Result<types::PtrOfTrainData> {
        string_arg!(filename);
        string_arg!(var_type_spec);
        unsafe { sys::cv_ml_TrainData_loadFromCSV_String_int_int_int_String_char_char(filename.as_ptr(), header_line_count, response_start_idx, response_end_idx, var_type_spec.as_ptr(), delimiter, missch) }.into_result().map(|ptr| types::PtrOfTrainData { ptr })
    }
    
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
    /// ## C++ default parameters
    /// * var_idx: noArray()
    /// * sample_idx: noArray()
    /// * sample_weights: noArray()
    /// * var_type: noArray()
    pub fn create(samples: &dyn core::ToInputArray, layout: i32, responses: &dyn core::ToInputArray, var_idx: &dyn core::ToInputArray, sample_idx: &dyn core::ToInputArray, sample_weights: &dyn core::ToInputArray, var_type: &dyn core::ToInputArray) -> Result<types::PtrOfTrainData> {
        input_array_arg!(samples);
        input_array_arg!(responses);
        input_array_arg!(var_idx);
        input_array_arg!(sample_idx);
        input_array_arg!(sample_weights);
        input_array_arg!(var_type);
        unsafe { sys::cv_ml_TrainData_create__InputArray_int__InputArray__InputArray__InputArray__InputArray__InputArray(samples.as_raw__InputArray(), layout, responses.as_raw__InputArray(), var_idx.as_raw__InputArray(), sample_idx.as_raw__InputArray(), sample_weights.as_raw__InputArray(), var_type.as_raw__InputArray()) }.into_result().map(|ptr| types::PtrOfTrainData { ptr })
    }
    
}

