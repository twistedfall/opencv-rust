#![allow(unused_parens)]
//! # Tracking API
//! 
//! Long-term optical tracking API
//! ------------------------------
//! 
//! Long-term optical tracking is one of most important issue for many computer vision applications in
//! real world scenario. The development in this area is very fragmented and this API is an unique
//! interface useful for plug several algorithms and compare them. This work is partially based on
//! [AAM](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_AAM) and [AMVOT](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_AMVOT) .
//! 
//! This algorithms start from a bounding box of the target and with their internal representation they
//! avoid the drift during the tracking. These long-term trackers are able to evaluate online the
//! quality of the location of the target in the new frame, without ground truth.
//! 
//! There are three main components: the TrackerSampler, the TrackerFeatureSet and the TrackerModel. The
//! first component is the object that computes the patches over the frame based on the last target
//! location. The TrackerFeatureSet is the class that manages the Features, is possible plug many kind
//! of these (HAAR, HOG, LBP, Feature2D, etc). The last component is the internal representation of the
//! target, it is the appearence model. It stores all state candidates and compute the trajectory (the
//! most likely target states). The class TrackerTargetState represents a possible state of the target.
//! The TrackerSampler and the TrackerFeatureSet are the visual representation of the target, instead
//! the TrackerModel is the statistical model.
//! 
//! A recent benchmark between these algorithms can be found in [OOT](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_OOT)
//! 
//! To see how API works, try tracker demo:
//! <https://github.com/lenlen/opencv/blob/tracking_api/samples/cpp/tracker.cpp>
//! 
//! Creating Own Tracker
//! --------------------
//! 
//! If you want create a new tracker, here's what you have to do. First, decide on the name of the class
//! for the tracker (to meet the existing style, we suggest something with prefix "tracker", e.g.
//! trackerMIL, trackerBoosting) -- we shall refer to this choice as to "classname" in subsequent. Also,
//! you should decide upon the name of the tracker, is it will be known to user (the current style
//! suggests using all capitals, say MIL or BOOSTING) --we'll call it a "name".
//! 
//! *   Declare your tracker in include/opencv2/tracking/tracker.hpp. Your tracker should inherit from
//!    Tracker (please, see the example below). You should declare the specialized Param structure,
//!    where you probably will want to put the data, needed to initialize your tracker. Also don't
//!    forget to put the BOILERPLATE_CODE(name,classname) macro inside the class declaration. That
//!    macro will generate static createTracker() function, which we'll talk about later. You should
//!    get something similar to :
//! ```ignore
//!        class CV_EXPORTS_W TrackerMIL : public Tracker
//!        {
//!          public:
//!           struct CV_EXPORTS Params
//!           {
//!            Params();
//!            //parameters for sampler
//!            float samplerInitInRadius;  // radius for gathering positive instances during init
//!            int samplerInitMaxNegNum;  // # negative samples to use during init
//!            float samplerSearchWinSize;  // size of search window
//!            float samplerTrackInRadius;  // radius for gathering positive instances during tracking
//!            int samplerTrackMaxPosNum;  // # positive samples to use during tracking
//!            int samplerTrackMaxNegNum;  // # negative samples to use during tracking
//!            int featureSetNumFeatures;  // #features
//! 
//!            void read( const FileNode& fn );
//!            void write( FileStorage& fs ) const;
//!           };
//! ```
//! 
//!    of course, you can also add any additional methods of your choice. It should be pointed out,
//!    however, that it is not expected to have a constructor declared, as creation should be done via
//!    the corresponding createTracker() method.
//! *   In src/tracker.cpp file add BOILERPLATE_CODE(name,classname) line to the body of
//!    Tracker::create() method you will find there, like :
//! ```ignore
//!        Ptr<Tracker> Tracker::create( const String& trackerType )
//!        {
//!           BOILERPLATE_CODE("BOOSTING",TrackerBoosting);
//!           BOILERPLATE_CODE("MIL",TrackerMIL);
//!           return Ptr<Tracker>();
//!        }
//! ```
//! 
//! *   Finally, you should implement the function with signature :
//! ```ignore
//!        Ptr<classname> classname::createTracker(const classname::Params &parameters){
//!            ...
//!        }
//! ```
//! 
//!    That function can (and probably will) return a pointer to some derived class of "classname",
//!    which will probably have a real constructor.
//! 
//! Every tracker has three component TrackerSampler, TrackerFeatureSet and TrackerModel. The first two
//! are instantiated from Tracker base class, instead the last component is abstract, so you must
//! implement your TrackerModel.
//! 
//! ### TrackerSampler
//! 
//! TrackerSampler is already instantiated, but you should define the sampling algorithm and add the
//! classes (or single class) to TrackerSampler. You can choose one of the ready implementation as
//! TrackerSamplerCSC or you can implement your sampling method, in this case the class must inherit
//! TrackerSamplerAlgorithm. Fill the samplingImpl method that writes the result in "sample" output
//! argument.
//! 
//! Example of creating specialized TrackerSamplerAlgorithm TrackerSamplerCSC : :
//! ```ignore
//!    class CV_EXPORTS_W TrackerSamplerCSC : public TrackerSamplerAlgorithm
//!    {
//!      public:
//!       TrackerSamplerCSC( const TrackerSamplerCSC::Params &parameters = TrackerSamplerCSC::Params() );
//!       ~TrackerSamplerCSC();
//!       ...
//! 
//!      protected:
//!       bool samplingImpl( const Mat& image, Rect boundingBox, std::vector<Mat>& sample );
//!       ...
//! 
//!    };
//! ```
//! 
//! 
//! Example of adding TrackerSamplerAlgorithm to TrackerSampler : :
//! ```ignore
//!    //sampler is the TrackerSampler
//!    Ptr<TrackerSamplerAlgorithm> CSCSampler = new TrackerSamplerCSC( CSCparameters );
//!    if( !sampler->addTrackerSamplerAlgorithm( CSCSampler ) )
//!      return false;
//! 
//!    //or add CSC sampler with default parameters
//!    //sampler->addTrackerSamplerAlgorithm( "CSC" );
//! ```
//! ## See also
//! TrackerSamplerCSC, TrackerSamplerAlgorithm
//! 
//! ### TrackerFeatureSet
//! 
//! TrackerFeatureSet is already instantiated (as first) , but you should define what kinds of features
//! you'll use in your tracker. You can use multiple feature types, so you can add a ready
//! implementation as TrackerFeatureHAAR in your TrackerFeatureSet or develop your own implementation.
//! In this case, in the computeImpl method put the code that extract the features and in the selection
//! method optionally put the code for the refinement and selection of the features.
//! 
//! Example of creating specialized TrackerFeature TrackerFeatureHAAR : :
//! ```ignore
//!    class CV_EXPORTS_W TrackerFeatureHAAR : public TrackerFeature
//!    {
//!      public:
//!       TrackerFeatureHAAR( const TrackerFeatureHAAR::Params &parameters = TrackerFeatureHAAR::Params() );
//!       ~TrackerFeatureHAAR();
//!       void selection( Mat& response, int npoints );
//!       ...
//! 
//!      protected:
//!       bool computeImpl( const std::vector<Mat>& images, Mat& response );
//!       ...
//! 
//!    };
//! ```
//! 
//! Example of adding TrackerFeature to TrackerFeatureSet : :
//! ```ignore
//!    //featureSet is the TrackerFeatureSet
//!    Ptr<TrackerFeature> trackerFeature = new TrackerFeatureHAAR( HAARparameters );
//!    featureSet->addTrackerFeature( trackerFeature );
//! ```
//! 
//! TrackerFeatureHAAR, TrackerFeatureSet
//! 
//! ### TrackerModel
//! 
//! TrackerModel is abstract, so in your implementation you must develop your TrackerModel that inherit
//! from TrackerModel. Fill the method for the estimation of the state "modelEstimationImpl", that
//! estimates the most likely target location, see [AAM](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_AAM) table I (ME) for further information. Fill
//! "modelUpdateImpl" in order to update the model, see [AAM](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_AAM) table I (MU). In this class you can use
//! the :cConfidenceMap and :cTrajectory to storing the model. The first represents the model on the all
//! possible candidate states and the second represents the list of all estimated states.
//! 
//! Example of creating specialized TrackerModel TrackerMILModel : :
//! ```ignore
//!    class TrackerMILModel : public TrackerModel
//!    {
//!      public:
//!       TrackerMILModel( const Rect& boundingBox );
//!       ~TrackerMILModel();
//!       ...
//! 
//!      protected:
//!       void modelEstimationImpl( const std::vector<Mat>& responses );
//!       void modelUpdateImpl();
//!       ...
//! 
//!    };
//! ```
//! 
//! And add it in your Tracker : :
//! ```ignore
//!    bool TrackerMIL::initImpl( const Mat& image, const Rect2d& boundingBox )
//!    {
//!       ...
//!       //model is the general TrackerModel field of the general Tracker
//!       model = new TrackerMILModel( boundingBox );
//!       ...
//!    }
//! ```
//! 
//! In the last step you should define the TrackerStateEstimator based on your implementation or you can
//! use one of ready class as TrackerStateEstimatorMILBoosting. It represent the statistical part of the
//! model that estimates the most likely target state.
//! 
//! Example of creating specialized TrackerStateEstimator TrackerStateEstimatorMILBoosting : :
//! ```ignore
//!    class CV_EXPORTS_W TrackerStateEstimatorMILBoosting : public TrackerStateEstimator
//!    {
//!      class TrackerMILTargetState : public TrackerTargetState
//!      {
//!      ...
//!      };
//! 
//!      public:
//!       TrackerStateEstimatorMILBoosting( int nFeatures = 250 );
//!       ~TrackerStateEstimatorMILBoosting();
//!       ...
//! 
//!      protected:
//!       Ptr<TrackerTargetState> estimateImpl( const std::vector<ConfidenceMap>& confidenceMaps );
//!       void updateImpl( std::vector<ConfidenceMap>& confidenceMaps );
//!       ...
//! 
//!    };
//! ```
//! 
//! And add it in your TrackerModel : :
//! ```ignore
//!    //model is the TrackerModel of your Tracker
//!    Ptr<TrackerStateEstimatorMILBoosting> stateEstimator = new TrackerStateEstimatorMILBoosting( params.featureSetNumFeatures );
//!    model->setTrackerStateEstimator( stateEstimator );
//! ```
//! 
//! TrackerModel, TrackerStateEstimatorMILBoosting, TrackerTargetState
//! 
//! During this step, you should define your TrackerTargetState based on your implementation.
//! TrackerTargetState base class has only the bounding box (upper-left position, width and height), you
//! can enrich it adding scale factor, target rotation, etc.
//! 
//! Example of creating specialized TrackerTargetState TrackerMILTargetState : :
//! ```ignore
//!    class TrackerMILTargetState : public TrackerTargetState
//!    {
//!      public:
//!       TrackerMILTargetState( const Point2f& position, int targetWidth, int targetHeight, bool foreground, const Mat& features );
//!       ~TrackerMILTargetState();
//!       ...
//! 
//!      private:
//!       bool isTarget;
//!       Mat targetFeatures;
//!       ...
//! 
//!    };
//! ```
//! 
//! ### Try it
//! 
//! To try your tracker you can use the demo at
//! <https://github.com/lenlen/opencv/blob/tracking_api/samples/cpp/tracker.cpp>.
//! 
//! The first argument is the name of the tracker and the second is a video source.
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::CvFeatureParamsTrait, super::CvHaarEvaluator_FeatureHaarTrait, super::CvHaarEvaluatorTrait, super::ClfMilBoost_ParamsTrait, super::ClfMilBoostTrait, super::TrackerFeature, super::TrackerFeatureSetTrait, super::TrackerSamplerAlgorithm, super::TrackerSamplerTrait, super::TrackerTargetStateTrait, super::TrackerStateEstimator, super::TrackerModel, super::Tracker, super::TrackerStateEstimatorMILBoosting_TrackerMILTargetStateTrait, super::TrackerStateEstimatorMILBoostingTrait, super::TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetStateTrait, super::TrackerStateEstimatorAdaBoostingTrait, super::TrackerStateEstimatorSVMTrait, super::TrackerSamplerCSC_ParamsTrait, super::TrackerSamplerCSCTrait, super::TrackerSamplerCS_ParamsTrait, super::TrackerSamplerCSTrait, super::TrackerSamplerPF_ParamsTrait, super::TrackerSamplerPFTrait, super::TrackerFeatureFeature2dTrait, super::TrackerFeatureHOGTrait, super::TrackerFeatureHAAR_ParamsTrait, super::TrackerFeatureHAARTrait, super::TrackerFeatureLBPTrait, super::TrackerMIL_ParamsTrait, super::TrackerMIL, super::TrackerBoosting_ParamsTrait, super::TrackerBoosting, super::TrackerMedianFlow_ParamsTrait, super::TrackerMedianFlow, super::TrackerTLD_ParamsTrait, super::TrackerTLD, super::TrackerKCF_ParamsTrait, super::TrackerKCF, super::TrackerGOTURN_ParamsTrait, super::TrackerGOTURN, super::MultiTrackerTrait, super::MultiTracker_AltTrait, super::MultiTrackerTLDTrait };
}

pub const CC_FEATURE_PARAMS: &'static str = "featureParams";
pub const CC_FEATURE_SIZE: &'static str = "featSize";
pub const CC_ISINTEGRAL: &'static str = "isIntegral";
pub const CC_MAX_CAT_COUNT: &'static str = "maxCatCount";
pub const CC_NUM_FEATURES: &'static str = "numFeat";
pub const CC_RECT: &'static str = "rect";
pub const CC_RECTS: &'static str = "rects";
pub const CC_TILTED: &'static str = "tilted";
pub const CV_HAAR_FEATURE_MAX: i32 = 3;
pub const CvFeatureParams_HAAR: i32 = 0;
pub const CvFeatureParams_HOG: i32 = 2;
pub const CvFeatureParams_LBP: i32 = 1;
pub const FEATURES: &'static str = "features";
pub const HFP_NAME: &'static str = "haarFeatureParams";
pub const HOGF_NAME: &'static str = "HOGFeatureParams";
pub const LBPF_NAME: &'static str = "lbpFeatureParams";
pub const N_BINS: i32 = 9;
pub const N_CELLS: i32 = 4;
/// mode for detect samples
pub const TrackerSamplerCSC_MODE_DETECT: i32 = 5;
/// mode for init negative samples
pub const TrackerSamplerCSC_MODE_INIT_NEG: i32 = 2;
/// mode for init positive samples
pub const TrackerSamplerCSC_MODE_INIT_POS: i32 = 1;
/// mode for update negative samples
pub const TrackerSamplerCSC_MODE_TRACK_NEG: i32 = 4;
/// mode for update positive samples
pub const TrackerSamplerCSC_MODE_TRACK_POS: i32 = 3;
/// mode for classify samples
pub const TrackerSamplerCS_MODE_CLASSIFY: i32 = 3;
/// mode for negative samples
pub const TrackerSamplerCS_MODE_NEGATIVE: i32 = 2;
/// mode for positive samples
pub const TrackerSamplerCS_MODE_POSITIVE: i32 = 1;
/// \brief Feature type to be used in the tracking grayscale, colornames, compressed color-names
///  The modes available now:
/// *   "GRAY" -- Use grayscale values as the feature
/// *   "CN" -- Color-names feature
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TrackerKCF_MODE {
	GRAY = 1 as isize,
	CN = 2 as isize,
	CUSTOM = 4 as isize,
}

opencv_type_enum! { crate::tracking::TrackerKCF_MODE }

/// ## C++ default parameters
/// * from_center: true
pub fn select_roi(mut img: core::Mat, from_center: bool) -> Result<core::Rect2d> {
	unsafe { sys::cv_selectROI_Mat_bool(img.as_raw_mut_Mat(), from_center) }.into_result()
}

/// ## C++ default parameters
/// * show_crossair: true
/// * from_center: true
pub fn select_roi_1(window_name: &str, mut img: core::Mat, show_crossair: bool, from_center: bool) -> Result<core::Rect2d> {
	extern_container_arg!(window_name);
	unsafe { sys::cv_selectROI_const_StringX_Mat_bool_bool(window_name.opencv_to_extern(), img.as_raw_mut_Mat(), show_crossair, from_center) }.into_result()
}

/// ## C++ default parameters
/// * from_center: true
pub fn select_roi_2(window_name: &str, mut img: core::Mat, bounding_box: &mut core::Vector::<core::Rect2d>, from_center: bool) -> Result<()> {
	extern_container_arg!(window_name);
	unsafe { sys::cv_selectROI_const_StringX_Mat_vector_Rect2d_X_bool(window_name.opencv_to_extern(), img.as_raw_mut_Mat(), bounding_box.as_raw_mut_VectorOfRect2d(), from_center) }.into_result()
}

/// ## C++ default parameters
/// * root_path: "TLD_dataset"
/// * dataset_ind: 0
pub fn tld_init_dataset(video_ind: i32, root_path: &str, dataset_ind: i32) -> Result<core::Rect2d> {
	extern_container_arg!(root_path);
	unsafe { sys::cv_tld_tld_InitDataset_int_const_charX_int(video_ind, root_path.opencv_to_extern(), dataset_ind) }.into_result()
}

pub fn tld_get_next_dataset_frame() -> Result<core::Mat> {
	unsafe { sys::cv_tld_tld_getNextDatasetFrame() }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
}

pub trait ClfMilBoostTrait {
	fn as_raw_ClfMilBoost(&self) -> *const c_void;
	fn as_raw_mut_ClfMilBoost(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * parameters: ClfMilBoost::Params()
	fn init(&mut self, parameters: &crate::tracking::ClfMilBoost_Params) -> Result<()> {
		unsafe { sys::cv_ClfMilBoost_init_const_ParamsX(self.as_raw_mut_ClfMilBoost(), parameters.as_raw_ClfMilBoost_Params()) }.into_result()
	}
	
	fn update(&mut self, posx: &core::Mat, negx: &core::Mat) -> Result<()> {
		unsafe { sys::cv_ClfMilBoost_update_const_MatX_const_MatX(self.as_raw_mut_ClfMilBoost(), posx.as_raw_Mat(), negx.as_raw_Mat()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * log_r: true
	fn classify(&mut self, x: &core::Mat, log_r: bool) -> Result<core::Vector::<f32>> {
		unsafe { sys::cv_ClfMilBoost_classify_const_MatX_bool(self.as_raw_mut_ClfMilBoost(), x.as_raw_Mat(), log_r) }.into_result().map(|r| unsafe { core::Vector::<f32>::opencv_from_extern(r) } )
	}
	
	fn sigmoid(&mut self, x: f32) -> Result<f32> {
		unsafe { sys::cv_ClfMilBoost_sigmoid_float(self.as_raw_mut_ClfMilBoost(), x) }.into_result()
	}
	
}

pub struct ClfMilBoost {
	ptr: *mut c_void
}

opencv_type_boxed! { ClfMilBoost }

impl Drop for ClfMilBoost {
	fn drop(&mut self) {
		extern "C" { fn cv_ClfMilBoost_delete(instance: *mut c_void); }
		unsafe { cv_ClfMilBoost_delete(self.as_raw_mut_ClfMilBoost()) };
	}
}

impl ClfMilBoost {
	pub fn as_raw_ClfMilBoost(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_ClfMilBoost(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for ClfMilBoost {}

impl crate::tracking::ClfMilBoostTrait for ClfMilBoost {
	fn as_raw_ClfMilBoost(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_ClfMilBoost(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ClfMilBoost {
	pub fn default() -> Result<crate::tracking::ClfMilBoost> {
		unsafe { sys::cv_ClfMilBoost_ClfMilBoost() }.into_result().map(|r| unsafe { crate::tracking::ClfMilBoost::opencv_from_extern(r) } )
	}
	
}

pub trait ClfMilBoost_ParamsTrait {
	fn as_raw_ClfMilBoost_Params(&self) -> *const c_void;
	fn as_raw_mut_ClfMilBoost_Params(&mut self) -> *mut c_void;

	fn _num_sel(&self) -> i32 {
		unsafe { sys::cv_ClfMilBoost_Params_getProp_numSel_const(self.as_raw_ClfMilBoost_Params()) }.into_result().expect("Infallible function failed: _num_sel")
	}
	
	fn set_num_sel(&mut self, val: i32) -> () {
		unsafe { sys::cv_ClfMilBoost_Params_setProp_numSel_int(self.as_raw_mut_ClfMilBoost_Params(), val) }.into_result().expect("Infallible function failed: set_num_sel")
	}
	
	fn _num_feat(&self) -> i32 {
		unsafe { sys::cv_ClfMilBoost_Params_getProp_numFeat_const(self.as_raw_ClfMilBoost_Params()) }.into_result().expect("Infallible function failed: _num_feat")
	}
	
	fn set_num_feat(&mut self, val: i32) -> () {
		unsafe { sys::cv_ClfMilBoost_Params_setProp_numFeat_int(self.as_raw_mut_ClfMilBoost_Params(), val) }.into_result().expect("Infallible function failed: set_num_feat")
	}
	
	fn _l_rate(&self) -> f32 {
		unsafe { sys::cv_ClfMilBoost_Params_getProp_lRate_const(self.as_raw_ClfMilBoost_Params()) }.into_result().expect("Infallible function failed: _l_rate")
	}
	
	fn set_l_rate(&mut self, val: f32) -> () {
		unsafe { sys::cv_ClfMilBoost_Params_setProp_lRate_float(self.as_raw_mut_ClfMilBoost_Params(), val) }.into_result().expect("Infallible function failed: set_l_rate")
	}
	
}

pub struct ClfMilBoost_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { ClfMilBoost_Params }

impl Drop for ClfMilBoost_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_ClfMilBoost_Params_delete(instance: *mut c_void); }
		unsafe { cv_ClfMilBoost_Params_delete(self.as_raw_mut_ClfMilBoost_Params()) };
	}
}

impl ClfMilBoost_Params {
	pub fn as_raw_ClfMilBoost_Params(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_ClfMilBoost_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for ClfMilBoost_Params {}

impl crate::tracking::ClfMilBoost_ParamsTrait for ClfMilBoost_Params {
	fn as_raw_ClfMilBoost_Params(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_ClfMilBoost_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ClfMilBoost_Params {
	pub fn default() -> Result<crate::tracking::ClfMilBoost_Params> {
		unsafe { sys::cv_ClfMilBoost_Params_Params() }.into_result().map(|r| unsafe { crate::tracking::ClfMilBoost_Params::opencv_from_extern(r) } )
	}
	
}

pub trait CvFeatureParamsTrait {
	fn as_raw_CvFeatureParams(&self) -> *const c_void;
	fn as_raw_mut_CvFeatureParams(&mut self) -> *mut c_void;

	fn max_cat_count(&self) -> i32 {
		unsafe { sys::cv_CvFeatureParams_getPropMaxCatCount_const(self.as_raw_CvFeatureParams()) }.into_result().expect("Infallible function failed: max_cat_count")
	}
	
	fn set_max_cat_count(&mut self, val: i32) -> () {
		unsafe { sys::cv_CvFeatureParams_setPropMaxCatCount_int(self.as_raw_mut_CvFeatureParams(), val) }.into_result().expect("Infallible function failed: set_max_cat_count")
	}
	
	fn feat_size(&self) -> i32 {
		unsafe { sys::cv_CvFeatureParams_getPropFeatSize_const(self.as_raw_CvFeatureParams()) }.into_result().expect("Infallible function failed: feat_size")
	}
	
	fn set_feat_size(&mut self, val: i32) -> () {
		unsafe { sys::cv_CvFeatureParams_setPropFeatSize_int(self.as_raw_mut_CvFeatureParams(), val) }.into_result().expect("Infallible function failed: set_feat_size")
	}
	
	fn num_features(&self) -> i32 {
		unsafe { sys::cv_CvFeatureParams_getPropNumFeatures_const(self.as_raw_CvFeatureParams()) }.into_result().expect("Infallible function failed: num_features")
	}
	
	fn set_num_features(&mut self, val: i32) -> () {
		unsafe { sys::cv_CvFeatureParams_setPropNumFeatures_int(self.as_raw_mut_CvFeatureParams(), val) }.into_result().expect("Infallible function failed: set_num_features")
	}
	
	fn init(&mut self, fp: &crate::tracking::CvFeatureParams) -> Result<()> {
		unsafe { sys::cv_CvFeatureParams_init_const_CvFeatureParamsX(self.as_raw_mut_CvFeatureParams(), fp.as_raw_CvFeatureParams()) }.into_result()
	}
	
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_CvFeatureParams_write_const_FileStorageX(self.as_raw_CvFeatureParams(), fs.as_raw_mut_FileStorage()) }.into_result()
	}
	
	fn read(&mut self, node: &core::FileNode) -> Result<bool> {
		unsafe { sys::cv_CvFeatureParams_read_const_FileNodeX(self.as_raw_mut_CvFeatureParams(), node.as_raw_FileNode()) }.into_result()
	}
	
}

pub struct CvFeatureParams {
	ptr: *mut c_void
}

opencv_type_boxed! { CvFeatureParams }

impl Drop for CvFeatureParams {
	fn drop(&mut self) {
		extern "C" { fn cv_CvFeatureParams_delete(instance: *mut c_void); }
		unsafe { cv_CvFeatureParams_delete(self.as_raw_mut_CvFeatureParams()) };
	}
}

impl CvFeatureParams {
	pub fn as_raw_CvFeatureParams(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_CvFeatureParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for CvFeatureParams {}

impl crate::tracking::CvFeatureParamsTrait for CvFeatureParams {
	fn as_raw_CvFeatureParams(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_CvFeatureParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CvFeatureParams {
	#[cfg(not(target_os = "windows"))]
	pub fn default() -> Result<crate::tracking::CvFeatureParams> {
		unsafe { sys::cv_CvFeatureParams_CvFeatureParams() }.into_result().map(|r| unsafe { crate::tracking::CvFeatureParams::opencv_from_extern(r) } )
	}
	
	#[cfg(not(target_os = "windows"))]
	pub fn create(feature_type: i32) -> Result<core::Ptr::<crate::tracking::CvFeatureParams>> {
		unsafe { sys::cv_CvFeatureParams_create_int(feature_type) }.into_result().map(|r| unsafe { core::Ptr::<crate::tracking::CvFeatureParams>::opencv_from_extern(r) } )
	}
	
}

pub trait CvHaarEvaluatorTrait {
	fn as_raw_CvHaarEvaluator(&self) -> *const c_void;
	fn as_raw_mut_CvHaarEvaluator(&mut self) -> *mut c_void;

	fn init(&mut self, _feature_params: &crate::tracking::CvFeatureParams, _max_sample_count: i32, _win_size: core::Size) -> Result<()> {
		unsafe { sys::cv_CvHaarEvaluator_init_const_CvFeatureParamsX_int_Size(self.as_raw_mut_CvHaarEvaluator(), _feature_params.as_raw_CvFeatureParams(), _max_sample_count, _win_size.opencv_to_extern()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * cls_label: 0
	/// * idx: 1
	fn set_image(&mut self, img: &core::Mat, cls_label: u8, idx: i32) -> Result<()> {
		unsafe { sys::cv_CvHaarEvaluator_setImage_const_MatX_unsigned_char_int(self.as_raw_mut_CvHaarEvaluator(), img.as_raw_Mat(), cls_label, idx) }.into_result()
	}
	
	fn write_features(&self, fs: &mut core::FileStorage, feature_map: &core::Mat) -> Result<()> {
		unsafe { sys::cv_CvHaarEvaluator_writeFeatures_const_FileStorageX_const_MatX(self.as_raw_CvHaarEvaluator(), fs.as_raw_mut_FileStorage(), feature_map.as_raw_Mat()) }.into_result()
	}
	
	#[cfg(not(target_os = "windows"))]
	fn write_feature(&self, fs: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_CvHaarEvaluator_writeFeature_const_FileStorageX(self.as_raw_CvHaarEvaluator(), fs.as_raw_mut_FileStorage()) }.into_result()
	}
	
	#[cfg(not(target_os = "windows"))]
	fn get_features(&self) -> Result<core::Vector::<crate::tracking::CvHaarEvaluator_FeatureHaar>> {
		unsafe { sys::cv_CvHaarEvaluator_getFeatures_const(self.as_raw_CvHaarEvaluator()) }.into_result().map(|r| unsafe { core::Vector::<crate::tracking::CvHaarEvaluator_FeatureHaar>::opencv_from_extern(r) } )
	}
	
	fn get_features_1(&mut self, idx: i32) -> Result<crate::tracking::CvHaarEvaluator_FeatureHaar> {
		unsafe { sys::cv_CvHaarEvaluator_getFeatures_int(self.as_raw_mut_CvHaarEvaluator(), idx) }.into_result().map(|r| unsafe { crate::tracking::CvHaarEvaluator_FeatureHaar::opencv_from_extern(r) } )
	}
	
	#[cfg(not(target_os = "windows"))]
	fn set_win_size(&mut self, patch_size: core::Size) -> Result<()> {
		unsafe { sys::cv_CvHaarEvaluator_setWinSize_Size(self.as_raw_mut_CvHaarEvaluator(), patch_size.opencv_to_extern()) }.into_result()
	}
	
	#[cfg(not(target_os = "windows"))]
	fn set_win_size_1(&self) -> Result<core::Size> {
		unsafe { sys::cv_CvHaarEvaluator_setWinSize_const(self.as_raw_CvHaarEvaluator()) }.into_result()
	}
	
	fn generate_features(&mut self) -> Result<()> {
		unsafe { sys::cv_CvHaarEvaluator_generateFeatures(self.as_raw_mut_CvHaarEvaluator()) }.into_result()
	}
	
	/// TODO new method
	/// \brief Overload the original generateFeatures in order to limit the number of the features
	/// ## Parameters
	/// * numFeatures: Number of the features
	fn generate_features_1(&mut self, num_features: i32) -> Result<()> {
		unsafe { sys::cv_CvHaarEvaluator_generateFeatures_int(self.as_raw_mut_CvHaarEvaluator(), num_features) }.into_result()
	}
	
}

pub struct CvHaarEvaluator {
	ptr: *mut c_void
}

opencv_type_boxed! { CvHaarEvaluator }

impl Drop for CvHaarEvaluator {
	fn drop(&mut self) {
		extern "C" { fn cv_CvHaarEvaluator_delete(instance: *mut c_void); }
		unsafe { cv_CvHaarEvaluator_delete(self.as_raw_mut_CvHaarEvaluator()) };
	}
}

impl CvHaarEvaluator {
	pub fn as_raw_CvHaarEvaluator(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_CvHaarEvaluator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for CvHaarEvaluator {}

impl crate::tracking::CvHaarEvaluatorTrait for CvHaarEvaluator {
	fn as_raw_CvHaarEvaluator(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_CvHaarEvaluator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CvHaarEvaluator {
}

pub trait CvHaarEvaluator_FeatureHaarTrait {
	fn as_raw_CvHaarEvaluator_FeatureHaar(&self) -> *const c_void;
	fn as_raw_mut_CvHaarEvaluator_FeatureHaar(&mut self) -> *mut c_void;

	#[cfg(not(target_os = "windows"))]
	fn eval(&self, image: &core::Mat, roi: core::Rect, result: &mut f32) -> Result<bool> {
		unsafe { sys::cv_CvHaarEvaluator_FeatureHaar_eval_const_const_MatX_Rect_floatX(self.as_raw_CvHaarEvaluator_FeatureHaar(), image.as_raw_Mat(), roi.opencv_to_extern(), result) }.into_result()
	}
	
	#[cfg(not(target_os = "windows"))]
	fn get_num_areas(&mut self) -> Result<i32> {
		unsafe { sys::cv_CvHaarEvaluator_FeatureHaar_getNumAreas(self.as_raw_mut_CvHaarEvaluator_FeatureHaar()) }.into_result()
	}
	
	#[cfg(not(target_os = "windows"))]
	fn get_weights(&self) -> Result<core::Vector::<f32>> {
		unsafe { sys::cv_CvHaarEvaluator_FeatureHaar_getWeights_const(self.as_raw_CvHaarEvaluator_FeatureHaar()) }.into_result().map(|r| unsafe { core::Vector::<f32>::opencv_from_extern(r) } )
	}
	
	#[cfg(not(target_os = "windows"))]
	fn get_areas(&self) -> Result<core::Vector::<core::Rect>> {
		unsafe { sys::cv_CvHaarEvaluator_FeatureHaar_getAreas_const(self.as_raw_CvHaarEvaluator_FeatureHaar()) }.into_result().map(|r| unsafe { core::Vector::<core::Rect>::opencv_from_extern(r) } )
	}
	
	fn write(&self, mut unnamed: core::FileStorage) -> Result<()> {
		unsafe { sys::cv_CvHaarEvaluator_FeatureHaar_write_const_FileStorage(self.as_raw_CvHaarEvaluator_FeatureHaar(), unnamed.as_raw_mut_FileStorage()) }.into_result()
	}
	
	#[cfg(not(target_os = "windows"))]
	fn get_init_mean(&self) -> Result<f32> {
		unsafe { sys::cv_CvHaarEvaluator_FeatureHaar_getInitMean_const(self.as_raw_CvHaarEvaluator_FeatureHaar()) }.into_result()
	}
	
	#[cfg(not(target_os = "windows"))]
	fn get_init_sigma(&self) -> Result<f32> {
		unsafe { sys::cv_CvHaarEvaluator_FeatureHaar_getInitSigma_const(self.as_raw_CvHaarEvaluator_FeatureHaar()) }.into_result()
	}
	
}

pub struct CvHaarEvaluator_FeatureHaar {
	ptr: *mut c_void
}

opencv_type_boxed! { CvHaarEvaluator_FeatureHaar }

impl Drop for CvHaarEvaluator_FeatureHaar {
	fn drop(&mut self) {
		extern "C" { fn cv_CvHaarEvaluator_FeatureHaar_delete(instance: *mut c_void); }
		unsafe { cv_CvHaarEvaluator_FeatureHaar_delete(self.as_raw_mut_CvHaarEvaluator_FeatureHaar()) };
	}
}

impl CvHaarEvaluator_FeatureHaar {
	pub fn as_raw_CvHaarEvaluator_FeatureHaar(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_CvHaarEvaluator_FeatureHaar(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for CvHaarEvaluator_FeatureHaar {}

impl crate::tracking::CvHaarEvaluator_FeatureHaarTrait for CvHaarEvaluator_FeatureHaar {
	fn as_raw_CvHaarEvaluator_FeatureHaar(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_CvHaarEvaluator_FeatureHaar(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CvHaarEvaluator_FeatureHaar {
	#[cfg(not(target_os = "windows"))]
	pub fn new(patch_size: core::Size) -> Result<crate::tracking::CvHaarEvaluator_FeatureHaar> {
		unsafe { sys::cv_CvHaarEvaluator_FeatureHaar_FeatureHaar_Size(patch_size.opencv_to_extern()) }.into_result().map(|r| unsafe { crate::tracking::CvHaarEvaluator_FeatureHaar::opencv_from_extern(r) } )
	}
	
}

/// ********************************** MultiTracker Class ---By Laksono Kurnianggoro---) ***********************************
/// This class is used to track multiple objects using the specified tracker algorithm.
/// * The MultiTracker is naive implementation of multiple object tracking.
/// * It process the tracked objects independently without any optimization accross the tracked objects.
pub trait MultiTrackerTrait {
	fn as_raw_MultiTracker(&self) -> *const c_void;
	fn as_raw_mut_MultiTracker(&mut self) -> *mut c_void;

	fn objects(&mut self) -> core::Vector::<core::Rect2d> {
		unsafe { sys::cv_MultiTracker_getPropObjects(self.as_raw_mut_MultiTracker()) }.into_result().map(|r| unsafe { core::Vector::<core::Rect2d>::opencv_from_extern(r) } ).expect("Infallible function failed: objects")
	}
	
	fn set_objects(&mut self, mut val: core::Vector::<core::Rect2d>) -> () {
		unsafe { sys::cv_MultiTracker_setPropObjects_vector_Rect2d_(self.as_raw_mut_MultiTracker(), val.as_raw_mut_VectorOfRect2d()) }.into_result().expect("Infallible function failed: set_objects")
	}
	
	/// \brief Add a new object to be tracked.
	/// The defaultAlgorithm will be used the newly added tracker.
	/// ## Parameters
	/// * image: input image
	/// * boundingBox: a rectangle represents ROI of the tracked object
	fn add(&mut self, image: &core::Mat, bounding_box: core::Rect2d) -> Result<bool> {
		unsafe { sys::cv_MultiTracker_add_const_MatX_const_Rect2dX(self.as_raw_mut_MultiTracker(), image.as_raw_Mat(), &bounding_box) }.into_result()
	}
	
	/// \brief Add a new object to be tracked.
	/// ## Parameters
	/// * trackerType: the name of the tracker algorithm to be used
	/// * image: input image
	/// * boundingBox: a rectangle represents ROI of the tracked object
	fn add_1(&mut self, tracker_type: &str, image: &core::Mat, bounding_box: core::Rect2d) -> Result<bool> {
		extern_container_arg!(tracker_type);
		unsafe { sys::cv_MultiTracker_add_const_StringX_const_MatX_const_Rect2dX(self.as_raw_mut_MultiTracker(), tracker_type.opencv_to_extern(), image.as_raw_Mat(), &bounding_box) }.into_result()
	}
	
	/// \brief Add a set of objects to be tracked.
	/// ## Parameters
	/// * trackerType: the name of the tracker algorithm to be used
	/// * image: input image
	/// * boundingBox: list of the tracked objects
	fn add_2(&mut self, tracker_type: &str, image: &core::Mat, mut bounding_box: core::Vector::<core::Rect2d>) -> Result<bool> {
		extern_container_arg!(tracker_type);
		unsafe { sys::cv_MultiTracker_add_const_StringX_const_MatX_vector_Rect2d_(self.as_raw_mut_MultiTracker(), tracker_type.opencv_to_extern(), image.as_raw_Mat(), bounding_box.as_raw_mut_VectorOfRect2d()) }.into_result()
	}
	
	/// \brief Add a set of objects to be tracked using the defaultAlgorithm tracker.
	/// ## Parameters
	/// * image: input image
	/// * boundingBox: list of the tracked objects
	fn add_3(&mut self, image: &core::Mat, mut bounding_box: core::Vector::<core::Rect2d>) -> Result<bool> {
		unsafe { sys::cv_MultiTracker_add_const_MatX_vector_Rect2d_(self.as_raw_mut_MultiTracker(), image.as_raw_Mat(), bounding_box.as_raw_mut_VectorOfRect2d()) }.into_result()
	}
	
	/// \brief Update the current tracking status.
	/// The result will be saved in the internal storage.
	/// ## Parameters
	/// * image: input image
	fn update(&mut self, image: &core::Mat) -> Result<bool> {
		unsafe { sys::cv_MultiTracker_update_const_MatX(self.as_raw_mut_MultiTracker(), image.as_raw_Mat()) }.into_result()
	}
	
	/// \brief Update the current tracking status.
	/// ## Parameters
	/// * image: input image
	/// * boundingBox: the tracking result, represent a list of ROIs of the tracked objects.
	fn update_1(&mut self, image: &core::Mat, bounding_box: &mut core::Vector::<core::Rect2d>) -> Result<bool> {
		unsafe { sys::cv_MultiTracker_update_const_MatX_vector_Rect2d_X(self.as_raw_mut_MultiTracker(), image.as_raw_Mat(), bounding_box.as_raw_mut_VectorOfRect2d()) }.into_result()
	}
	
}

/// ********************************** MultiTracker Class ---By Laksono Kurnianggoro---) ***********************************
/// This class is used to track multiple objects using the specified tracker algorithm.
/// * The MultiTracker is naive implementation of multiple object tracking.
/// * It process the tracked objects independently without any optimization accross the tracked objects.
pub struct MultiTracker {
	ptr: *mut c_void
}

opencv_type_boxed! { MultiTracker }

impl Drop for MultiTracker {
	fn drop(&mut self) {
		extern "C" { fn cv_MultiTracker_delete(instance: *mut c_void); }
		unsafe { cv_MultiTracker_delete(self.as_raw_mut_MultiTracker()) };
	}
}

impl MultiTracker {
	pub fn as_raw_MultiTracker(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_MultiTracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for MultiTracker {}

impl crate::tracking::MultiTrackerTrait for MultiTracker {
	fn as_raw_MultiTracker(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_MultiTracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MultiTracker {
	/// \brief Constructor.
	/// In the case of trackerType is given, it will be set as the default algorithm for all trackers.
	/// ## Parameters
	/// * trackerType: the name of the tracker algorithm to be used
	/// 
	/// ## C++ default parameters
	/// * tracker_type: ""
	pub fn new(tracker_type: &str) -> Result<crate::tracking::MultiTracker> {
		extern_container_arg!(tracker_type);
		unsafe { sys::cv_MultiTracker_MultiTracker_const_StringX(tracker_type.opencv_to_extern()) }.into_result().map(|r| unsafe { crate::tracking::MultiTracker::opencv_from_extern(r) } )
	}
	
}

/// Multi Object Tracker for TLD. TLD is a novel tracking framework that explicitly decomposes
/// the long-term tracking task into tracking, learning and detection.
/// 
/// The tracker follows the object from frame to frame. The detector localizes all appearances that
/// have been observed so far and corrects the tracker if necessary. The learning estimates detector’s
/// errors and updates it to avoid these errors in the future. The implementation is based on [TLD](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_TLD) .
/// 
/// The Median Flow algorithm (see cv::TrackerMedianFlow) was chosen as a tracking component in this
/// implementation, following authors. Tracker is supposed to be able to handle rapid motions, partial
/// occlusions, object absence etc.
/// ## See also
/// Tracker, MultiTracker, TrackerTLD
pub trait MultiTrackerTLDTrait: crate::tracking::MultiTracker_AltTrait {
	fn as_raw_MultiTrackerTLD(&self) -> *const c_void;
	fn as_raw_mut_MultiTrackerTLD(&mut self) -> *mut c_void;

	/// Update all trackers from the tracking-list, find a new most likely bounding boxes for the targets by
	/// optimized update method using some techniques to speedup calculations specifically for MO TLD. The only limitation
	/// is that all target bounding boxes should have approximately same aspect ratios. Speed boost is around 20%
	/// 
	/// ## Parameters
	/// * image: The current frame.
	/// 
	/// ## Returns
	/// True means that all targets were located and false means that tracker couldn't locate one of the targets in
	/// current frame. Note, that latter *does not* imply that tracker has failed, maybe target is indeed
	/// missing from the frame (say, out of sight)
	fn update_opt(&mut self, image: &core::Mat) -> Result<bool> {
		unsafe { sys::cv_MultiTrackerTLD_update_opt_const_MatX(self.as_raw_mut_MultiTrackerTLD(), image.as_raw_Mat()) }.into_result()
	}
	
}

/// Multi Object Tracker for TLD. TLD is a novel tracking framework that explicitly decomposes
/// the long-term tracking task into tracking, learning and detection.
/// 
/// The tracker follows the object from frame to frame. The detector localizes all appearances that
/// have been observed so far and corrects the tracker if necessary. The learning estimates detector’s
/// errors and updates it to avoid these errors in the future. The implementation is based on [TLD](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_TLD) .
/// 
/// The Median Flow algorithm (see cv::TrackerMedianFlow) was chosen as a tracking component in this
/// implementation, following authors. Tracker is supposed to be able to handle rapid motions, partial
/// occlusions, object absence etc.
/// ## See also
/// Tracker, MultiTracker, TrackerTLD
pub struct MultiTrackerTLD {
	ptr: *mut c_void
}

opencv_type_boxed! { MultiTrackerTLD }

impl Drop for MultiTrackerTLD {
	fn drop(&mut self) {
		extern "C" { fn cv_MultiTrackerTLD_delete(instance: *mut c_void); }
		unsafe { cv_MultiTrackerTLD_delete(self.as_raw_mut_MultiTrackerTLD()) };
	}
}

impl MultiTrackerTLD {
	pub fn as_raw_MultiTrackerTLD(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_MultiTrackerTLD(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for MultiTrackerTLD {}

impl crate::tracking::MultiTrackerTLDTrait for MultiTrackerTLD {
	fn as_raw_MultiTrackerTLD(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_MultiTrackerTLD(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::tracking::MultiTracker_AltTrait for MultiTrackerTLD {
	fn as_raw_MultiTracker_Alt(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_MultiTracker_Alt(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MultiTrackerTLD {
}

/// Base abstract class for the long-term Multi Object Trackers:
/// ## See also
/// Tracker, MultiTrackerTLD
pub trait MultiTracker_AltTrait {
	fn as_raw_MultiTracker_Alt(&self) -> *const c_void;
	fn as_raw_mut_MultiTracker_Alt(&mut self) -> *mut c_void;

	/// Current number of targets in tracking-list
	fn target_num(&self) -> i32 {
		unsafe { sys::cv_MultiTracker_Alt_getPropTargetNum_const(self.as_raw_MultiTracker_Alt()) }.into_result().expect("Infallible function failed: target_num")
	}
	
	/// Current number of targets in tracking-list
	fn set_target_num(&mut self, val: i32) -> () {
		unsafe { sys::cv_MultiTracker_Alt_setPropTargetNum_int(self.as_raw_mut_MultiTracker_Alt(), val) }.into_result().expect("Infallible function failed: set_target_num")
	}
	
	/// Trackers list for Multi-Object-Tracker
	fn trackers(&mut self) -> core::Vector::<core::Ptr::<dyn crate::tracking::Tracker>> {
		unsafe { sys::cv_MultiTracker_Alt_getPropTrackers(self.as_raw_mut_MultiTracker_Alt()) }.into_result().map(|r| unsafe { core::Vector::<core::Ptr::<dyn crate::tracking::Tracker>>::opencv_from_extern(r) } ).expect("Infallible function failed: trackers")
	}
	
	/// Trackers list for Multi-Object-Tracker
	fn set_trackers(&mut self, mut val: core::Vector::<core::Ptr::<dyn crate::tracking::Tracker>>) -> () {
		unsafe { sys::cv_MultiTracker_Alt_setPropTrackers_vector_Ptr_Tracker__(self.as_raw_mut_MultiTracker_Alt(), val.as_raw_mut_VectorOfPtrOfTracker()) }.into_result().expect("Infallible function failed: set_trackers")
	}
	
	/// Bounding Boxes list for Multi-Object-Tracker
	fn bounding_boxes(&mut self) -> core::Vector::<core::Rect2d> {
		unsafe { sys::cv_MultiTracker_Alt_getPropBoundingBoxes(self.as_raw_mut_MultiTracker_Alt()) }.into_result().map(|r| unsafe { core::Vector::<core::Rect2d>::opencv_from_extern(r) } ).expect("Infallible function failed: bounding_boxes")
	}
	
	/// Bounding Boxes list for Multi-Object-Tracker
	fn set_bounding_boxes(&mut self, mut val: core::Vector::<core::Rect2d>) -> () {
		unsafe { sys::cv_MultiTracker_Alt_setPropBoundingBoxes_vector_Rect2d_(self.as_raw_mut_MultiTracker_Alt(), val.as_raw_mut_VectorOfRect2d()) }.into_result().expect("Infallible function failed: set_bounding_boxes")
	}
	
	/// List of randomly generated colors for bounding boxes display
	fn colors(&mut self) -> core::Vector::<core::Scalar> {
		unsafe { sys::cv_MultiTracker_Alt_getPropColors(self.as_raw_mut_MultiTracker_Alt()) }.into_result().map(|r| unsafe { core::Vector::<core::Scalar>::opencv_from_extern(r) } ).expect("Infallible function failed: colors")
	}
	
	/// List of randomly generated colors for bounding boxes display
	fn set_colors(&mut self, mut val: core::Vector::<core::Scalar>) -> () {
		unsafe { sys::cv_MultiTracker_Alt_setPropColors_vector_Scalar_(self.as_raw_mut_MultiTracker_Alt(), val.as_raw_mut_VectorOfScalar()) }.into_result().expect("Infallible function failed: set_colors")
	}
	
	/// Add a new target to a tracking-list and initialize the tracker with a know bounding box that surrounding the target
	/// ## Parameters
	/// * image: The initial frame
	/// * boundingBox: The initial boundig box of target
	/// * tracker_algorithm_name: Multi-tracker algorithm name
	/// 
	/// ## Returns
	/// True if new target initialization went succesfully, false otherwise
	fn add_target(&mut self, image: &core::Mat, bounding_box: core::Rect2d, tracker_algorithm_name: &str) -> Result<bool> {
		extern_container_arg!(mut tracker_algorithm_name);
		unsafe { sys::cv_MultiTracker_Alt_addTarget_const_MatX_const_Rect2dX_String(self.as_raw_mut_MultiTracker_Alt(), image.as_raw_Mat(), &bounding_box, tracker_algorithm_name.opencv_to_extern_mut()) }.into_result()
	}
	
	/// Update all trackers from the tracking-list, find a new most likely bounding boxes for the targets
	/// ## Parameters
	/// * image: The current frame
	/// 
	/// ## Returns
	/// True means that all targets were located and false means that tracker couldn't locate one of the targets in
	/// current frame. Note, that latter *does not* imply that tracker has failed, maybe target is indeed
	/// missing from the frame (say, out of sight)
	fn update(&mut self, image: &core::Mat) -> Result<bool> {
		unsafe { sys::cv_MultiTracker_Alt_update_const_MatX(self.as_raw_mut_MultiTracker_Alt(), image.as_raw_Mat()) }.into_result()
	}
	
}

/// Base abstract class for the long-term Multi Object Trackers:
/// ## See also
/// Tracker, MultiTrackerTLD
pub struct MultiTracker_Alt {
	ptr: *mut c_void
}

opencv_type_boxed! { MultiTracker_Alt }

impl Drop for MultiTracker_Alt {
	fn drop(&mut self) {
		extern "C" { fn cv_MultiTracker_Alt_delete(instance: *mut c_void); }
		unsafe { cv_MultiTracker_Alt_delete(self.as_raw_mut_MultiTracker_Alt()) };
	}
}

impl MultiTracker_Alt {
	pub fn as_raw_MultiTracker_Alt(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_MultiTracker_Alt(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for MultiTracker_Alt {}

impl crate::tracking::MultiTracker_AltTrait for MultiTracker_Alt {
	fn as_raw_MultiTracker_Alt(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_MultiTracker_Alt(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MultiTracker_Alt {
	/// Constructor for Multitracker
	pub fn default() -> Result<crate::tracking::MultiTracker_Alt> {
		unsafe { sys::cv_MultiTracker_Alt_MultiTracker_Alt() }.into_result().map(|r| unsafe { crate::tracking::MultiTracker_Alt::opencv_from_extern(r) } )
	}
	
}

/// Base abstract class for the long-term tracker:
pub trait Tracker: core::AlgorithmTrait {
	fn as_raw_Tracker(&self) -> *const c_void;
	fn as_raw_mut_Tracker(&mut self) -> *mut c_void;

	/// Initialize the tracker with a know bounding box that surrounding the target
	/// ## Parameters
	/// * image: The initial frame
	/// * boundingBox: The initial boundig box
	/// 
	/// ## Returns
	/// True if initialization went succesfully, false otherwise
	fn init(&mut self, image: &core::Mat, bounding_box: core::Rect2d) -> Result<bool> {
		unsafe { sys::cv_Tracker_init_const_MatX_const_Rect2dX(self.as_raw_mut_Tracker(), image.as_raw_Mat(), &bounding_box) }.into_result()
	}
	
	/// Update the tracker, find the new most likely bounding box for the target
	/// ## Parameters
	/// * image: The current frame
	/// * boundingBox: The boundig box that represent the new target location, if true was returned, not
	/// modified otherwise
	/// 
	/// ## Returns
	/// True means that target was located and false means that tracker cannot locate target in
	/// current frame. Note, that latter *does not* imply that tracker has failed, maybe target is indeed
	/// missing from the frame (say, out of sight)
	fn update(&mut self, image: &core::Mat, bounding_box: &mut core::Rect2d) -> Result<bool> {
		unsafe { sys::cv_Tracker_update_const_MatX_Rect2dX(self.as_raw_mut_Tracker(), image.as_raw_Mat(), bounding_box) }.into_result()
	}
	
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_Tracker_read_const_FileNodeX(self.as_raw_mut_Tracker(), fn_.as_raw_FileNode()) }.into_result()
	}
	
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_Tracker_write_const_FileStorageX(self.as_raw_Tracker(), fs.as_raw_mut_FileStorage()) }.into_result()
	}
	
	fn get_model(&mut self) -> Result<core::Ptr::<dyn crate::tracking::TrackerModel>> {
		unsafe { sys::cv_Tracker_getModel(self.as_raw_mut_Tracker()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::tracking::TrackerModel>::opencv_from_extern(r) } )
	}
	
}

impl dyn Tracker + '_ {
	/// Creates a tracker by its name.
	/// ## Parameters
	/// * trackerType: Tracker type
	/// 
	/// The following detector types are supported:
	/// 
	/// *   "MIL" -- TrackerMIL
	/// *   "BOOSTING" -- TrackerBoosting
	pub fn create(tracker_type: &str) -> Result<core::Ptr::<dyn crate::tracking::Tracker>> {
		extern_container_arg!(tracker_type);
		unsafe { sys::cv_Tracker_create_const_StringX(tracker_type.opencv_to_extern()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::tracking::Tracker>::opencv_from_extern(r) } )
	}
	
}
/// This is a real-time object tracking based on a novel on-line version of the AdaBoost algorithm.
/// 
/// The classifier uses the surrounding background as negative examples in update step to avoid the
/// drifting problem. The implementation is based on [OLB](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_OLB) .
pub trait TrackerBoosting: crate::tracking::Tracker {
	fn as_raw_TrackerBoosting(&self) -> *const c_void;
	fn as_raw_mut_TrackerBoosting(&mut self) -> *mut c_void;

}

impl dyn TrackerBoosting + '_ {
	/// ## C++ default parameters
	/// * parameters: classname::Params());virtual~classname(){};/*
	///  * Partially based on:
	///  * ====================================================================================================================
	///  *   - [AAM] S. Salti, A. Cavallaro, L. Di Stefano, Adaptive Appearance Modeling for Video Tracking: Survey and Evaluation
	///  *  - [AMVOT] X. Li, W. Hu, C. Shen, Z. Zhang, A. Dick, A. van den Hengel, A Survey of Appearance Models in Visual Object Tracking
	///  *
	///  * This Tracking API has been designed with PlantUML. If you modify this API please change UML files under modules/tracking/doc/uml
	///  *
	///  */namespacecv{//! @addtogroup tracking//! @{/************************************ TrackerFeature Base Classes ************************************//** @brief Abstract base class for TrackerFeature that represents the feature.
	///  */classCV_EXPORTSTrackerFeature{public:virtual~TrackerFeature();/** @brief Compute the features in the images collection
	///     @param images The images
	///     @param response The output response
	///      */voidcompute(conststd::vector<Mat>&images,Mat&response);/** @brief Create TrackerFeature by tracker feature type
	///     @param trackerFeatureType The TrackerFeature name
	/// 
	///     The modes available now:
	/// 
	///     -   "HAAR" -- Haar Feature-based
	/// 
	///     The modes that will be available soon:
	/// 
	///     -   "HOG" -- Histogram of Oriented Gradients features
	///     -   "LBP" -- Local Binary Pattern features
	///     -   "FEATURE2D" -- All types of Feature2D
	///      */staticPtr<TrackerFeature>create(constString&trackerFeatureType);/** @brief Identify most effective features
	///     @param response Collection of response for the specific TrackerFeature
	///     @param npoints Max number of features
	/// 
	///     @note This method modifies the response parameter
	///      */virtualvoidselection(Mat&response,intnpoints)=0;/** @brief Get the name of the specific TrackerFeature
	///      */StringgetClassName()const;protected:virtualboolcomputeImpl(conststd::vector<Mat>&images,Mat&response)=0;StringclassName;};/** @brief Class that manages the extraction and selection of features
	/// 
	/// @cite AAM Feature Extraction and Feature Set Refinement (Feature Processing and Feature Selection).
	/// See table I and section III C @cite AMVOT Appearance modelling -\> Visual representation (Table II,
	/// section 3.1 - 3.2)
	/// 
	/// TrackerFeatureSet is an aggregation of TrackerFeature
	/// 
	/// @sa
	///    TrackerFeature
	/// 
	///  */classCV_EXPORTSTrackerFeatureSet{public:TrackerFeatureSet();~TrackerFeatureSet();/** @brief Extract features from the images collection
	///     @param images The input images
	///      */voidextraction(conststd::vector<Mat>&images);/** @brief Identify most effective features for all feature types (optional)
	///      */voidselection();/** @brief Remove outliers for all feature types (optional)
	///      */voidremoveOutliers();/** @brief Add TrackerFeature in the collection. Return true if TrackerFeature is added, false otherwise
	///     @param trackerFeatureType The TrackerFeature name
	/// 
	///     The modes available now:
	/// 
	///     -   "HAAR" -- Haar Feature-based
	/// 
	///     The modes that will be available soon:
	/// 
	///     -   "HOG" -- Histogram of Oriented Gradients features
	///     -   "LBP" -- Local Binary Pattern features
	///     -   "FEATURE2D" -- All types of Feature2D
	/// 
	///     Example TrackerFeatureSet::addTrackerFeature : :
	///     @code
	///         //sample usage:
	/// 
	///         Ptr<TrackerFeature> trackerFeature = new TrackerFeatureHAAR( HAARparameters );
	///         featureSet->addTrackerFeature( trackerFeature );
	/// 
	///         //or add CSC sampler with default parameters
	///         //featureSet->addTrackerFeature( "HAAR" );
	///     @endcode
	///     @note If you use the second method, you must initialize the TrackerFeature
	///      */booladdTrackerFeature(StringtrackerFeatureType);/** @overload
	///     @param feature The TrackerFeature class
	///     */booladdTrackerFeature(Ptr<TrackerFeature>&feature);/** @brief Get the TrackerFeature collection (TrackerFeature name, TrackerFeature pointer)
	///      */conststd::vector<std::pair<String,Ptr<TrackerFeature>>>&getTrackerFeature()const;/** @brief Get the responses
	/// 
	///     @note Be sure to call extraction before getResponses Example TrackerFeatureSet::getResponses : :
	///      */conststd::vector<Mat>&getResponses()const;private:voidclearResponses();boolblockAddTrackerFeature;std::vector<std::pair<String,Ptr<TrackerFeature>>>features;//list of featuresstd::vector<Mat>responses;//list of response after compute};/************************************ TrackerSampler Base Classes ************************************//** @brief Abstract base class for TrackerSamplerAlgorithm that represents the algorithm for the specific
	/// sampler.
	///  */classCV_EXPORTSTrackerSamplerAlgorithm{public:/**
	///    * \brief Destructor
	///    */virtual~TrackerSamplerAlgorithm();/** @brief Create TrackerSamplerAlgorithm by tracker sampler type.
	///     @param trackerSamplerType The trackerSamplerType name
	/// 
	///     The modes available now:
	/// 
	///     -   "CSC" -- Current State Center
	///     -   "CS" -- Current State
	///      */staticPtr<TrackerSamplerAlgorithm>create(constString&trackerSamplerType);/** @brief Computes the regions starting from a position in an image.
	/// 
	///     Return true if samples are computed, false otherwise
	/// 
	///     @param image The current frame
	///     @param boundingBox The bounding box from which regions can be calculated
	/// 
	///     @param sample The computed samples @cite AAM Fig. 1 variable Sk
	///      */boolsampling(constMat&image,RectboundingBox,std::vector<Mat>&sample);/** @brief Get the name of the specific TrackerSamplerAlgorithm
	///     */StringgetClassName()const;protected:StringclassName;virtualboolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample)=0;};/**
	///  * \brief Class that manages the sampler in order to select regions for the update the model of the tracker
	///  * [AAM] Sampling e Labeling. See table I and section III B
	///  *//** @brief Class that manages the sampler in order to select regions for the update the model of the tracker
	/// 
	/// @cite AAM Sampling e Labeling. See table I and section III B
	/// 
	/// TrackerSampler is an aggregation of TrackerSamplerAlgorithm
	/// @sa
	///    TrackerSamplerAlgorithm
	///  */classCV_EXPORTSTrackerSampler{public:/**
	///    * \brief Constructor
	///    */TrackerSampler();/**
	///    * \brief Destructor
	///    */~TrackerSampler();/** @brief Computes the regions starting from a position in an image
	///     @param image The current frame
	///     @param boundingBox The bounding box from which regions can be calculated
	///      */voidsampling(constMat&image,RectboundingBox);/** @brief Return the collection of the TrackerSamplerAlgorithm
	///     */conststd::vector<std::pair<String,Ptr<TrackerSamplerAlgorithm>>>&getSamplers()const;/** @brief Return the samples from all TrackerSamplerAlgorithm, @cite AAM Fig. 1 variable Sk
	///     */conststd::vector<Mat>&getSamples()const;/** @brief Add TrackerSamplerAlgorithm in the collection. Return true if sampler is added, false otherwise
	///     @param trackerSamplerAlgorithmType The TrackerSamplerAlgorithm name
	/// 
	///     The modes available now:
	///     -   "CSC" -- Current State Center
	///     -   "CS" -- Current State
	///     -   "PF" -- Particle Filtering
	/// 
	///     Example TrackerSamplerAlgorithm::addTrackerSamplerAlgorithm : :
	///     @code
	///          TrackerSamplerCSC::Params CSCparameters;
	///          Ptr<TrackerSamplerAlgorithm> CSCSampler = new TrackerSamplerCSC( CSCparameters );
	/// 
	///          if( !sampler->addTrackerSamplerAlgorithm( CSCSampler ) )
	///            return false;
	/// 
	///          //or add CSC sampler with default parameters
	///          //sampler->addTrackerSamplerAlgorithm( "CSC" );
	///     @endcode
	///     @note If you use the second method, you must initialize the TrackerSamplerAlgorithm
	///      */booladdTrackerSamplerAlgorithm(StringtrackerSamplerAlgorithmType);/** @overload
	///     @param sampler The TrackerSamplerAlgorithm
	///     */booladdTrackerSamplerAlgorithm(Ptr<TrackerSamplerAlgorithm>&sampler);private:std::vector<std::pair<String,Ptr<TrackerSamplerAlgorithm>>>samplers;std::vector<Mat>samples;boolblockAddTrackerSampler;voidclearSamples();};/************************************ TrackerModel Base Classes ************************************//** @brief Abstract base class for TrackerTargetState that represents a possible state of the target.
	/// 
	/// See @cite AAM \f$\hat{x}^{i}_{k}\f$ all the states candidates.
	/// 
	/// Inherits this class with your Target state, In own implementation you can add scale variation,
	/// width, height, orientation, etc.
	///  */classCV_EXPORTSTrackerTargetState{public:virtual~TrackerTargetState(){};/**
	///    * \brief Get the position
	///    * \return The position
	///    */Point2fgetTargetPosition()const;/**
	///    * \brief Set the position
	///    * \param position The position
	///    */voidsetTargetPosition(constPoint2f&position);/**
	///    * \brief Get the width of the target
	///    * \return The width of the target
	///    */intgetTargetWidth()const;/**
	///    * \brief Set the width of the target
	///    * \param width The width of the target
	///    */voidsetTargetWidth(intwidth);/**
	///    * \brief Get the height of the target
	///    * \return The height of the target
	///    */intgetTargetHeight()const;/**
	///    * \brief Set the height of the target
	///    * \param height The height of the target
	///    */voidsetTargetHeight(intheight);protected:Point2ftargetPosition;inttargetWidth;inttargetHeight;};/** @brief Represents the model of the target at frame \f$k\f$ (all states and scores)
	/// 
	/// See @cite AAM The set of the pair \f$\langle \hat{x}^{i}_{k}, C^{i}_{k} \rangle\f$
	/// @sa TrackerTargetState
	///  */typedefstd::vector<std::pair<Ptr<TrackerTargetState>,float>>ConfidenceMap;/** @brief Represents the estimate states for all frames
	/// 
	/// @cite AAM \f$x_{k}\f$ is the trajectory of the target up to time \f$k\f$
	/// 
	/// @sa TrackerTargetState
	///  */typedefstd::vector<Ptr<TrackerTargetState>>Trajectory;/** @brief Abstract base class for TrackerStateEstimator that estimates the most likely target state.
	/// 
	/// See @cite AAM State estimator
	/// 
	/// See @cite AMVOT Statistical modeling (Fig. 3), Table III (generative) - IV (discriminative) - V (hybrid)
	///  */classCV_EXPORTSTrackerStateEstimator{public:virtual~TrackerStateEstimator();/** @brief Estimate the most likely target state, return the estimated state
	///     @param confidenceMaps The overall appearance model as a list of :cConfidenceMap
	///      */Ptr<TrackerTargetState>estimate(conststd::vector<ConfidenceMap>&confidenceMaps);/** @brief Update the ConfidenceMap with the scores
	///     @param confidenceMaps The overall appearance model as a list of :cConfidenceMap
	///      */voidupdate(std::vector<ConfidenceMap>&confidenceMaps);/** @brief Create TrackerStateEstimator by tracker state estimator type
	///     @param trackeStateEstimatorType The TrackerStateEstimator name
	/// 
	///     The modes available now:
	/// 
	///     -   "BOOSTING" -- Boosting-based discriminative appearance models. See @cite AMVOT section 4.4
	/// 
	///     The modes available soon:
	/// 
	///     -   "SVM" -- SVM-based discriminative appearance models. See @cite AMVOT section 4.5
	///      */staticPtr<TrackerStateEstimator>create(constString&trackeStateEstimatorType);/** @brief Get the name of the specific TrackerStateEstimator
	///      */StringgetClassName()const;protected:virtualPtr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps)=0;virtualvoidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps)=0;StringclassName;};/** @brief Abstract class that represents the model of the target. It must be instantiated by specialized
	/// tracker
	/// 
	/// See @cite AAM Ak
	/// 
	/// Inherits this with your TrackerModel
	///  */classCV_EXPORTSTrackerModel{public:/**
	///    * \brief Constructor
	///    */TrackerModel();/**
	///    * \brief Destructor
	///    */virtual~TrackerModel();/** @brief Set TrackerEstimator, return true if the tracker state estimator is added, false otherwise
	///     @param trackerStateEstimator The TrackerStateEstimator
	///     @note You can add only one TrackerStateEstimator
	///      */boolsetTrackerStateEstimator(Ptr<TrackerStateEstimator>trackerStateEstimator);/** @brief Estimate the most likely target location
	/// 
	///     @cite AAM ME, Model Estimation table I
	///     @param responses Features extracted from TrackerFeatureSet
	///      */voidmodelEstimation(conststd::vector<Mat>&responses);/** @brief Update the model
	/// 
	///     @cite AAM MU, Model Update table I
	///      */voidmodelUpdate();/** @brief Run the TrackerStateEstimator, return true if is possible to estimate a new state, false otherwise
	///     */boolrunStateEstimator();/** @brief Set the current TrackerTargetState in the Trajectory
	///     @param lastTargetState The current TrackerTargetState
	///      */voidsetLastTargetState(constPtr<TrackerTargetState>&lastTargetState);/** @brief Get the last TrackerTargetState from Trajectory
	///     */Ptr<TrackerTargetState>getLastTargetState()const;/** @brief Get the list of the ConfidenceMap
	///     */conststd::vector<ConfidenceMap>&getConfidenceMaps()const;/** @brief Get the last ConfidenceMap for the current frame
	///      */constConfidenceMap&getLastConfidenceMap()const;/** @brief Get the TrackerStateEstimator
	///     */Ptr<TrackerStateEstimator>getTrackerStateEstimator()const;private:voidclearCurrentConfidenceMap();protected:std::vector<ConfidenceMap>confidenceMaps;Ptr<TrackerStateEstimator>stateEstimator;ConfidenceMapcurrentConfidenceMap;Trajectorytrajectory;intmaxCMLength;virtualvoidmodelEstimationImpl(conststd::vector<Mat>&responses)=0;virtualvoidmodelUpdateImpl()=0;};/************************************ Tracker Base Class ************************************//** @brief Base abstract class for the long-term tracker:
	///  */classCV_EXPORTS_WTracker:publicvirtualAlgorithm{public:virtual~Tracker();/** @brief Initialize the tracker with a know bounding box that surrounding the target
	///     @param image The initial frame
	///     @param boundingBox The initial boundig box
	/// 
	///     @return True if initialization went succesfully, false otherwise
	///      */CV_WRAPboolinit(constMat&image,constRect2d&boundingBox);/** @brief Update the tracker, find the new most likely bounding box for the target
	///     @param image The current frame
	///     @param boundingBox The boundig box that represent the new target location, if true was returned, not
	///     modified otherwise
	/// 
	///     @return True means that target was located and false means that tracker cannot locate target in
	///     current frame. Note, that latter *does not* imply that tracker has failed, maybe target is indeed
	///     missing from the frame (say, out of sight)
	///      */CV_WRAPboolupdate(constMat&image,CV_OUTRect2d&boundingBox);/** @brief Creates a tracker by its name.
	///     @param trackerType Tracker type
	/// 
	///     The following detector types are supported:
	/// 
	///     -   "MIL" -- TrackerMIL
	///     -   "BOOSTING" -- TrackerBoosting
	///      */CV_WRAPstaticPtr<Tracker>create(constString&trackerType);virtualvoidread(constFileNode&fn)=0;virtualvoidwrite(FileStorage&fs)const=0;Ptr<TrackerModel>getModel(){returnmodel;}protected:virtualboolinitImpl(constMat&image,constRect2d&boundingBox)=0;virtualboolupdateImpl(constMat&image,Rect2d&boundingBox)=0;boolisInit;Ptr<TrackerFeatureSet>featureSet;Ptr<TrackerSampler>sampler;Ptr<TrackerModel>model;};/************************************ Specific TrackerStateEstimator Classes ************************************//** @brief TrackerStateEstimator based on Boosting
	///     */classCV_EXPORTSTrackerStateEstimatorMILBoosting:publicTrackerStateEstimator{public:/**
	///    * Implementation of the target state for TrackerStateEstimatorMILBoosting
	///    */classTrackerMILTargetState:publicTrackerTargetState{public:/**
	///      * \brief Constructor
	///      * \param position Top left corner of the bounding box
	///      * \param width Width of the bounding box
	///      * \param height Height of the bounding box
	///      * \param foreground label for target or background
	///      * \param features features extracted
	///      */TrackerMILTargetState(constPoint2f&position,intwidth,intheight,boolforeground,constMat&features);/**
	///      * \brief Destructor
	///      */~TrackerMILTargetState(){};/** @brief Set label: true for target foreground, false for background
	///     @param foreground Label for background/foreground
	///      */voidsetTargetFg(boolforeground);/** @brief Set the features extracted from TrackerFeatureSet
	///     @param features The features extracted
	///      */voidsetFeatures(constMat&features);/** @brief Get the label. Return true for target foreground, false for background
	///      */boolisTargetFg()const;/** @brief Get the features extracted
	///      */MatgetFeatures()const;private:boolisTarget;MattargetFeatures;};/** @brief Constructor
	///     @param nFeatures Number of features for each sample
	///      */TrackerStateEstimatorMILBoosting(intnFeatures=250);~TrackerStateEstimatorMILBoosting();/** @brief Set the current confidenceMap
	///     @param confidenceMap The current :cConfidenceMap
	///      */voidsetCurrentConfidenceMap(ConfidenceMap&confidenceMap);protected:Ptr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps);voidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps);private:uintmax_idx(conststd::vector<float>&v);voidprepareData(constConfidenceMap&confidenceMap,Mat&positive,Mat&negative);ClfMilBoostboostMILModel;booltrained;intnumFeatures;ConfidenceMapcurrentConfidenceMap;};/** @brief TrackerStateEstimatorAdaBoosting based on ADA-Boosting
	///  */classCV_EXPORTSTrackerStateEstimatorAdaBoosting:publicTrackerStateEstimator{public:/** @brief Implementation of the target state for TrackerAdaBoostingTargetState
	///     */classTrackerAdaBoostingTargetState:publicTrackerTargetState{public:/**
	///      * \brief Constructor
	///      * \param position Top left corner of the bounding box
	///      * \param width Width of the bounding box
	///      * \param height Height of the bounding box
	///      * \param foreground label for target or background
	///      * \param responses list of features
	///      */TrackerAdaBoostingTargetState(constPoint2f&position,intwidth,intheight,boolforeground,constMat&responses);/**
	///      * \brief Destructor
	///      */~TrackerAdaBoostingTargetState(){};/** @brief Set the features extracted from TrackerFeatureSet
	///     @param responses The features extracted
	///      */voidsetTargetResponses(constMat&responses);/** @brief Set label: true for target foreground, false for background
	///     @param foreground Label for background/foreground
	///      */voidsetTargetFg(boolforeground);/** @brief Get the features extracted
	///      */MatgetTargetResponses()const;/** @brief Get the label. Return true for target foreground, false for background
	///     */boolisTargetFg()const;private:boolisTarget;MattargetResponses;};/** @brief Constructor
	///     @param numClassifer Number of base classifiers
	///     @param initIterations Number of iterations in the initialization
	///     @param nFeatures Number of features/weak classifiers
	///     @param patchSize tracking rect
	///     @param ROI initial ROI
	///      */TrackerStateEstimatorAdaBoosting(intnumClassifer,intinitIterations,intnFeatures,SizepatchSize,constRect&ROI);/**
	///    * \brief Destructor
	///    */~TrackerStateEstimatorAdaBoosting();/** @brief Get the sampling ROI
	///      */RectgetSampleROI()const;/** @brief Set the sampling ROI
	///     @param ROI the sampling ROI
	///      */voidsetSampleROI(constRect&ROI);/** @brief Set the current confidenceMap
	///     @param confidenceMap The current :cConfidenceMap
	///      */voidsetCurrentConfidenceMap(ConfidenceMap&confidenceMap);/** @brief Get the list of the selected weak classifiers for the classification step
	///      */std::vector<int>computeSelectedWeakClassifier();/** @brief Get the list of the weak classifiers that should be replaced
	///      */std::vector<int>computeReplacedClassifier();/** @brief Get the list of the weak classifiers that replace those to be replaced
	///      */std::vector<int>computeSwappedClassifier();protected:Ptr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps);voidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps);Ptr<StrongClassifierDirectSelection>boostClassifier;private:intnumBaseClassifier;intiterationInit;intnumFeatures;booltrained;SizeinitPatchSize;RectsampleROI;std::vector<int>replacedClassifier;std::vector<int>swappedClassifier;ConfidenceMapcurrentConfidenceMap;};/**
	///  * \brief TrackerStateEstimator based on SVM
	///  */classCV_EXPORTSTrackerStateEstimatorSVM:publicTrackerStateEstimator{public:TrackerStateEstimatorSVM();~TrackerStateEstimatorSVM();protected:Ptr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps);voidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps);};/************************************ Specific TrackerSamplerAlgorithm Classes ************************************//** @brief TrackerSampler based on CSC (current state centered), used by MIL algorithm TrackerMIL
	///  */classCV_EXPORTSTrackerSamplerCSC:publicTrackerSamplerAlgorithm{public:enum{MODE_INIT_POS=1,//!< mode for init positive samplesMODE_INIT_NEG=2,//!< mode for init negative samplesMODE_TRACK_POS=3,//!< mode for update positive samplesMODE_TRACK_NEG=4,//!< mode for update negative samplesMODE_DETECT=5//!< mode for detect samples};structCV_EXPORTSParams{Params();floatinitInRad;//!< radius for gathering positive instances during initfloattrackInPosRad;//!< radius for gathering positive instances during trackingfloatsearchWinSize;//!< size of search windowintinitMaxNegNum;//!< # negative samples to use during initinttrackMaxPosNum;//!< # positive samples to use during traininginttrackMaxNegNum;//!< # negative samples to use during training};/** @brief Constructor
	///     @param parameters TrackerSamplerCSC parameters TrackerSamplerCSC::Params
	///      */TrackerSamplerCSC(constTrackerSamplerCSC::Params&parameters=TrackerSamplerCSC::Params());/** @brief Set the sampling mode of TrackerSamplerCSC
	///     @param samplingMode The sampling mode
	/// 
	///     The modes are:
	/// 
	///     -   "MODE_INIT_POS = 1" -- for the positive sampling in initialization step
	///     -   "MODE_INIT_NEG = 2" -- for the negative sampling in initialization step
	///     -   "MODE_TRACK_POS = 3" -- for the positive sampling in update step
	///     -   "MODE_TRACK_NEG = 4" -- for the negative sampling in update step
	///     -   "MODE_DETECT = 5" -- for the sampling in detection step
	///      */voidsetMode(intsamplingMode);~TrackerSamplerCSC();protected:boolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample);private:Paramsparams;intmode;RNGrng;std::vector<Mat>sampleImage(constMat&img,intx,inty,intw,inth,floatinrad,floatoutrad=0,intmaxnum=1000000);};/** @brief TrackerSampler based on CS (current state), used by algorithm TrackerBoosting
	///  */classCV_EXPORTSTrackerSamplerCS:publicTrackerSamplerAlgorithm{public:enum{MODE_POSITIVE=1,//!< mode for positive samplesMODE_NEGATIVE=2,//!< mode for negative samplesMODE_CLASSIFY=3//!< mode for classify samples};structCV_EXPORTSParams{Params();floatoverlap;//!<overlapping for the search windowsfloatsearchFactor;//!<search region parameter};/** @brief Constructor
	///     @param parameters TrackerSamplerCS parameters TrackerSamplerCS::Params
	///      */TrackerSamplerCS(constTrackerSamplerCS::Params&parameters=TrackerSamplerCS::Params());/** @brief Set the sampling mode of TrackerSamplerCS
	///     @param samplingMode The sampling mode
	/// 
	///     The modes are:
	/// 
	///     -   "MODE_POSITIVE = 1" -- for the positive sampling
	///     -   "MODE_NEGATIVE = 2" -- for the negative sampling
	///     -   "MODE_CLASSIFY = 3" -- for the sampling in classification step
	///      */voidsetMode(intsamplingMode);~TrackerSamplerCS();boolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample);RectgetROI()const;private:RectgetTrackingROI(floatsearchFactor);RectRectMultiply(constRect&rect,floatf);std::vector<Mat>patchesRegularScan(constMat&image,RecttrackingROI,SizepatchSize);voidsetCheckedROI(RectimageROI);Paramsparams;intmode;RecttrackedPatch;RectvalidROI;RectROI;};/** @brief This sampler is based on particle filtering.
	/// 
	/// In principle, it can be thought of as performing some sort of optimization (and indeed, this
	/// tracker uses opencv's optim module), where tracker seeks to find the rectangle in given frame,
	/// which is the most *"similar"* to the initial rectangle (the one, given through the constructor).
	/// 
	/// The optimization performed is stochastic and somehow resembles genetic algorithms, where on each new
	/// image received (submitted via TrackerSamplerPF::sampling()) we start with the region bounded by
	/// boundingBox, then generate several "perturbed" boxes, take the ones most similar to the original.
	/// This selection round is repeated several times. At the end, we hope that only the most promising box
	/// remaining, and these are combined to produce the subrectangle of image, which is put as a sole
	/// element in array sample.
	/// 
	/// It should be noted, that the definition of "similarity" between two rectangles is based on comparing
	/// their histograms. As experiments show, tracker is *not* very succesfull if target is assumed to
	/// strongly change its dimensions.
	///  */classCV_EXPORTSTrackerSamplerPF:publicTrackerSamplerAlgorithm{public:/** @brief This structure contains all the parameters that can be varied during the course of sampling
	///     algorithm. Below is the structure exposed, together with its members briefly explained with
	///     reference to the above discussion on algorithm's working.
	///  */structCV_EXPORTSParams{Params();intiterationNum;//!< number of selection roundsintparticlesNum;//!< number of "perturbed" boxes on each rounddoublealpha;//!< with each new round we exponentially decrease the amount of "perturbing" we allow (like in simulated annealing)//!< and this very alpha controls how fast annealing happens, ie. how fast perturbing decreasesMat_<double>std;//!< initial values for perturbing (1-by-4 array, as each rectangle is given by 4 values -- coordinates of opposite vertices,//!< hence we have 4 values to perturb)};/** @brief Constructor
	///     @param chosenRect Initial rectangle, that is supposed to contain target we'd like to track.
	///     @param parameters
	///      */TrackerSamplerPF(constMat&chosenRect,constTrackerSamplerPF::Params&parameters=TrackerSamplerPF::Params());protected:boolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample);private:Paramsparams;Ptr<MinProblemSolver>_solver;Ptr<MinProblemSolver::Function>_function;};/************************************ Specific TrackerFeature Classes ************************************//**
	///  * \brief TrackerFeature based on Feature2D
	///  */classCV_EXPORTSTrackerFeatureFeature2d:publicTrackerFeature{public:/**
	///    * \brief Constructor
	///    * \param detectorType string of FeatureDetector
	///    * \param descriptorType string of DescriptorExtractor
	///    */TrackerFeatureFeature2d(StringdetectorType,StringdescriptorType);~TrackerFeatureFeature2d();voidselection(Mat&response,intnpoints);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);private:std::vector<KeyPoint>keypoints;};/**
	///  * \brief TrackerFeature based on HOG
	///  */classCV_EXPORTSTrackerFeatureHOG:publicTrackerFeature{public:TrackerFeatureHOG();~TrackerFeatureHOG();voidselection(Mat&response,intnpoints);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);};/** @brief TrackerFeature based on HAAR features, used by TrackerMIL and many others algorithms
	/// @note HAAR features implementation is copied from apps/traincascade and modified according to MIL
	///  */classCV_EXPORTSTrackerFeatureHAAR:publicTrackerFeature{public:structCV_EXPORTSParams{Params();intnumFeatures;//!< # of rectsSizerectSize;//!< rect sizeboolisIntegral;//!< true if input images are integral, false otherwise};/** @brief Constructor
	///     @param parameters TrackerFeatureHAAR parameters TrackerFeatureHAAR::Params
	///      */TrackerFeatureHAAR(constTrackerFeatureHAAR::Params&parameters=TrackerFeatureHAAR::Params());~TrackerFeatureHAAR();/** @brief Compute the features only for the selected indices in the images collection
	///     @param selFeatures indices of selected features
	///     @param images The images
	///     @param response Collection of response for the specific TrackerFeature
	///      */boolextractSelected(conststd::vector<int>selFeatures,conststd::vector<Mat>&images,Mat&response);/** @brief Identify most effective features
	///     @param response Collection of response for the specific TrackerFeature
	///     @param npoints Max number of features
	/// 
	///     @note This method modifies the response parameter
	///      */voidselection(Mat&response,intnpoints);/** @brief Swap the feature in position source with the feature in position target
	///   @param source The source position
	///   @param target The target position
	///  */boolswapFeature(intsource,inttarget);/** @brief   Swap the feature in position id with the feature input
	///   @param id The position
	///   @param feature The feature
	///  */boolswapFeature(intid,CvHaarEvaluator::FeatureHaar&feature);/** @brief Get the feature in position id
	///     @param id The position
	///      */CvHaarEvaluator::FeatureHaar&getFeatureAt(intid);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);private:Paramsparams;Ptr<CvHaarEvaluator>featureEvaluator;};/**
	///  * \brief TrackerFeature based on LBP
	///  */classCV_EXPORTSTrackerFeatureLBP:publicTrackerFeature{public:TrackerFeatureLBP();~TrackerFeatureLBP();voidselection(Mat&response,intnpoints);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);};/************************************ Specific Tracker Classes ************************************//** @brief The MIL algorithm trains a classifier in an online manner to separate the object from the
	/// background.
	/// 
	/// Multiple Instance Learning avoids the drift problem for a robust tracking. The implementation is
	/// based on @cite MIL .
	/// 
	/// Original code can be found here <http://vision.ucsd.edu/~bbabenko/project_miltrack.shtml>
	///  */classCV_EXPORTSTrackerMIL:publicTracker{public:structCV_EXPORTSParams{Params();//parameters for samplerfloatsamplerInitInRadius;//!< radius for gathering positive instances during initintsamplerInitMaxNegNum;//!< # negative samples to use during initfloatsamplerSearchWinSize;//!< size of search windowfloatsamplerTrackInRadius;//!< radius for gathering positive instances during trackingintsamplerTrackMaxPosNum;//!< # positive samples to use during trackingintsamplerTrackMaxNegNum;//!< # negative samples to use during trackingintfeatureSetNumFeatures;//!< # featuresvoidread(constFileNode&fn);voidwrite(FileStorage&fs)const;};/** @brief Constructor
	///     @param parameters MIL parameters TrackerMIL::Params
	///      */BOILERPLATE_CODE("MIL",TrackerMIL);};/** @brief This is a real-time object tracking based on a novel on-line version of the AdaBoost algorithm.
	/// 
	/// The classifier uses the surrounding background as negative examples in update step to avoid the
	/// drifting problem. The implementation is based on @cite OLB .
	///  */classCV_EXPORTSTrackerBoosting:publicTracker{public:structCV_EXPORTSParams{Params();intnumClassifiers;//!<the number of classifiers to use in a OnlineBoosting algorithmfloatsamplerOverlap;//!<search region parameters to use in a OnlineBoosting algorithmfloatsamplerSearchFactor;//!< search region parameters to use in a OnlineBoosting algorithmintiterationInit;//!<the initial iterationsintfeatureSetNumFeatures;//!< # features/**
	///      * \brief Read parameters from file
	///      */voidread(constFileNode&fn);/**
	///      * \brief Write parameters in a file
	///      */voidwrite(FileStorage&fs)const;};/** @brief Constructor
	///     @param parameters BOOSTING parameters TrackerBoosting::Params
	///      */BOILERPLATE_CODE("BOOSTING",TrackerBoosting)
	pub fn create_tracker(parameters: &crate::tracking::TrackerBoosting_Params) -> Result<core::Ptr::<dyn crate::tracking::TrackerBoosting>> {
		unsafe { sys::cv_TrackerBoosting_createTracker_const_ParamsX(parameters.as_raw_TrackerBoosting_Params()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::tracking::TrackerBoosting>::opencv_from_extern(r) } )
	}
	
}
pub trait TrackerBoosting_ParamsTrait {
	fn as_raw_TrackerBoosting_Params(&self) -> *const c_void;
	fn as_raw_mut_TrackerBoosting_Params(&mut self) -> *mut c_void;

	/// the number of classifiers to use in a OnlineBoosting algorithm
	fn num_classifiers(&self) -> i32 {
		unsafe { sys::cv_TrackerBoosting_Params_getPropNumClassifiers_const(self.as_raw_TrackerBoosting_Params()) }.into_result().expect("Infallible function failed: num_classifiers")
	}
	
	/// the number of classifiers to use in a OnlineBoosting algorithm
	fn set_num_classifiers(&mut self, val: i32) -> () {
		unsafe { sys::cv_TrackerBoosting_Params_setPropNumClassifiers_int(self.as_raw_mut_TrackerBoosting_Params(), val) }.into_result().expect("Infallible function failed: set_num_classifiers")
	}
	
	/// search region parameters to use in a OnlineBoosting algorithm
	fn sampler_overlap(&self) -> f32 {
		unsafe { sys::cv_TrackerBoosting_Params_getPropSamplerOverlap_const(self.as_raw_TrackerBoosting_Params()) }.into_result().expect("Infallible function failed: sampler_overlap")
	}
	
	/// search region parameters to use in a OnlineBoosting algorithm
	fn set_sampler_overlap(&mut self, val: f32) -> () {
		unsafe { sys::cv_TrackerBoosting_Params_setPropSamplerOverlap_float(self.as_raw_mut_TrackerBoosting_Params(), val) }.into_result().expect("Infallible function failed: set_sampler_overlap")
	}
	
	/// search region parameters to use in a OnlineBoosting algorithm
	fn sampler_search_factor(&self) -> f32 {
		unsafe { sys::cv_TrackerBoosting_Params_getPropSamplerSearchFactor_const(self.as_raw_TrackerBoosting_Params()) }.into_result().expect("Infallible function failed: sampler_search_factor")
	}
	
	/// search region parameters to use in a OnlineBoosting algorithm
	fn set_sampler_search_factor(&mut self, val: f32) -> () {
		unsafe { sys::cv_TrackerBoosting_Params_setPropSamplerSearchFactor_float(self.as_raw_mut_TrackerBoosting_Params(), val) }.into_result().expect("Infallible function failed: set_sampler_search_factor")
	}
	
	/// the initial iterations
	fn iteration_init(&self) -> i32 {
		unsafe { sys::cv_TrackerBoosting_Params_getPropIterationInit_const(self.as_raw_TrackerBoosting_Params()) }.into_result().expect("Infallible function failed: iteration_init")
	}
	
	/// the initial iterations
	fn set_iteration_init(&mut self, val: i32) -> () {
		unsafe { sys::cv_TrackerBoosting_Params_setPropIterationInit_int(self.as_raw_mut_TrackerBoosting_Params(), val) }.into_result().expect("Infallible function failed: set_iteration_init")
	}
	
	/// # features
	fn feature_set_num_features(&self) -> i32 {
		unsafe { sys::cv_TrackerBoosting_Params_getPropFeatureSetNumFeatures_const(self.as_raw_TrackerBoosting_Params()) }.into_result().expect("Infallible function failed: feature_set_num_features")
	}
	
	/// # features
	fn set_feature_set_num_features(&mut self, val: i32) -> () {
		unsafe { sys::cv_TrackerBoosting_Params_setPropFeatureSetNumFeatures_int(self.as_raw_mut_TrackerBoosting_Params(), val) }.into_result().expect("Infallible function failed: set_feature_set_num_features")
	}
	
	/// \brief Read parameters from file
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_TrackerBoosting_Params_read_const_FileNodeX(self.as_raw_mut_TrackerBoosting_Params(), fn_.as_raw_FileNode()) }.into_result()
	}
	
	/// \brief Write parameters in a file
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_TrackerBoosting_Params_write_const_FileStorageX(self.as_raw_TrackerBoosting_Params(), fs.as_raw_mut_FileStorage()) }.into_result()
	}
	
}

pub struct TrackerBoosting_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerBoosting_Params }

impl Drop for TrackerBoosting_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerBoosting_Params_delete(instance: *mut c_void); }
		unsafe { cv_TrackerBoosting_Params_delete(self.as_raw_mut_TrackerBoosting_Params()) };
	}
}

impl TrackerBoosting_Params {
	pub fn as_raw_TrackerBoosting_Params(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerBoosting_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerBoosting_Params {}

impl crate::tracking::TrackerBoosting_ParamsTrait for TrackerBoosting_Params {
	fn as_raw_TrackerBoosting_Params(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerBoosting_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerBoosting_Params {
	pub fn default() -> Result<crate::tracking::TrackerBoosting_Params> {
		unsafe { sys::cv_TrackerBoosting_Params_Params() }.into_result().map(|r| unsafe { crate::tracking::TrackerBoosting_Params::opencv_from_extern(r) } )
	}
	
}

/// Abstract base class for TrackerFeature that represents the feature.
pub trait TrackerFeature {
	fn as_raw_TrackerFeature(&self) -> *const c_void;
	fn as_raw_mut_TrackerFeature(&mut self) -> *mut c_void;

	/// Compute the features in the images collection
	/// ## Parameters
	/// * images: The images
	/// * response: The output response
	fn compute(&mut self, images: &core::Vector::<core::Mat>, response: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_TrackerFeature_compute_const_vector_Mat_X_MatX(self.as_raw_mut_TrackerFeature(), images.as_raw_VectorOfMat(), response.as_raw_mut_Mat()) }.into_result()
	}
	
	/// Identify most effective features
	/// ## Parameters
	/// * response: Collection of response for the specific TrackerFeature
	/// * npoints: Max number of features
	/// 
	/// 
	/// Note: This method modifies the response parameter
	fn selection(&mut self, response: &mut core::Mat, npoints: i32) -> Result<()> {
		unsafe { sys::cv_TrackerFeature_selection_MatX_int(self.as_raw_mut_TrackerFeature(), response.as_raw_mut_Mat(), npoints) }.into_result()
	}
	
	/// Get the name of the specific TrackerFeature
	fn get_class_name(&self) -> Result<String> {
		unsafe { sys::cv_TrackerFeature_getClassName_const(self.as_raw_TrackerFeature()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
}

impl dyn TrackerFeature + '_ {
	/// Create TrackerFeature by tracker feature type
	/// ## Parameters
	/// * trackerFeatureType: The TrackerFeature name
	/// 
	/// The modes available now:
	/// 
	/// *   "HAAR" -- Haar Feature-based
	/// 
	/// The modes that will be available soon:
	/// 
	/// *   "HOG" -- Histogram of Oriented Gradients features
	/// *   "LBP" -- Local Binary Pattern features
	/// *   "FEATURE2D" -- All types of Feature2D
	pub fn create(tracker_feature_type: &str) -> Result<core::Ptr::<dyn crate::tracking::TrackerFeature>> {
		extern_container_arg!(tracker_feature_type);
		unsafe { sys::cv_TrackerFeature_create_const_StringX(tracker_feature_type.opencv_to_extern()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::tracking::TrackerFeature>::opencv_from_extern(r) } )
	}
	
}
/// \brief TrackerFeature based on Feature2D
pub trait TrackerFeatureFeature2dTrait: crate::tracking::TrackerFeature {
	fn as_raw_TrackerFeatureFeature2d(&self) -> *const c_void;
	fn as_raw_mut_TrackerFeatureFeature2d(&mut self) -> *mut c_void;

	fn selection(&mut self, response: &mut core::Mat, npoints: i32) -> Result<()> {
		unsafe { sys::cv_TrackerFeatureFeature2d_selection_MatX_int(self.as_raw_mut_TrackerFeatureFeature2d(), response.as_raw_mut_Mat(), npoints) }.into_result()
	}
	
}

/// \brief TrackerFeature based on Feature2D
pub struct TrackerFeatureFeature2d {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerFeatureFeature2d }

impl Drop for TrackerFeatureFeature2d {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerFeatureFeature2d_delete(instance: *mut c_void); }
		unsafe { cv_TrackerFeatureFeature2d_delete(self.as_raw_mut_TrackerFeatureFeature2d()) };
	}
}

impl TrackerFeatureFeature2d {
	pub fn as_raw_TrackerFeatureFeature2d(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerFeatureFeature2d(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerFeatureFeature2d {}

impl crate::tracking::TrackerFeature for TrackerFeatureFeature2d {
	fn as_raw_TrackerFeature(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::tracking::TrackerFeatureFeature2dTrait for TrackerFeatureFeature2d {
	fn as_raw_TrackerFeatureFeature2d(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerFeatureFeature2d(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerFeatureFeature2d {
	/// \brief Constructor
	/// \param detectorType string of FeatureDetector
	/// \param descriptorType string of DescriptorExtractor
	pub fn new(detector_type: &str, descriptor_type: &str) -> Result<crate::tracking::TrackerFeatureFeature2d> {
		extern_container_arg!(mut detector_type);
		extern_container_arg!(mut descriptor_type);
		unsafe { sys::cv_TrackerFeatureFeature2d_TrackerFeatureFeature2d_String_String(detector_type.opencv_to_extern_mut(), descriptor_type.opencv_to_extern_mut()) }.into_result().map(|r| unsafe { crate::tracking::TrackerFeatureFeature2d::opencv_from_extern(r) } )
	}
	
}

/// TrackerFeature based on HAAR features, used by TrackerMIL and many others algorithms
/// 
/// Note: HAAR features implementation is copied from apps/traincascade and modified according to MIL
pub trait TrackerFeatureHAARTrait: crate::tracking::TrackerFeature {
	fn as_raw_TrackerFeatureHAAR(&self) -> *const c_void;
	fn as_raw_mut_TrackerFeatureHAAR(&mut self) -> *mut c_void;

	/// Compute the features only for the selected indices in the images collection
	/// ## Parameters
	/// * selFeatures: indices of selected features
	/// * images: The images
	/// * response: Collection of response for the specific TrackerFeature
	fn extract_selected(&mut self, sel_features: core::Vector::<i32>, images: &core::Vector::<core::Mat>, response: &mut core::Mat) -> Result<bool> {
		unsafe { sys::cv_TrackerFeatureHAAR_extractSelected_vector_int__const_vector_Mat_X_MatX(self.as_raw_mut_TrackerFeatureHAAR(), sel_features.as_raw_VectorOfi32(), images.as_raw_VectorOfMat(), response.as_raw_mut_Mat()) }.into_result()
	}
	
	/// Identify most effective features
	/// ## Parameters
	/// * response: Collection of response for the specific TrackerFeature
	/// * npoints: Max number of features
	/// 
	/// 
	/// Note: This method modifies the response parameter
	fn selection(&mut self, response: &mut core::Mat, npoints: i32) -> Result<()> {
		unsafe { sys::cv_TrackerFeatureHAAR_selection_MatX_int(self.as_raw_mut_TrackerFeatureHAAR(), response.as_raw_mut_Mat(), npoints) }.into_result()
	}
	
	/// Swap the feature in position source with the feature in position target
	/// ## Parameters
	/// * source: The source position
	/// * target: The target position
	fn swap_feature(&mut self, source: i32, target: i32) -> Result<bool> {
		unsafe { sys::cv_TrackerFeatureHAAR_swapFeature_int_int(self.as_raw_mut_TrackerFeatureHAAR(), source, target) }.into_result()
	}
	
	///   Swap the feature in position id with the feature input
	/// ## Parameters
	/// * id: The position
	/// * feature: The feature
	fn swap_feature_1(&mut self, id: i32, feature: &mut crate::tracking::CvHaarEvaluator_FeatureHaar) -> Result<bool> {
		unsafe { sys::cv_TrackerFeatureHAAR_swapFeature_int_FeatureHaarX(self.as_raw_mut_TrackerFeatureHAAR(), id, feature.as_raw_mut_CvHaarEvaluator_FeatureHaar()) }.into_result()
	}
	
	/// Get the feature in position id
	/// ## Parameters
	/// * id: The position
	fn get_feature_at(&mut self, id: i32) -> Result<crate::tracking::CvHaarEvaluator_FeatureHaar> {
		unsafe { sys::cv_TrackerFeatureHAAR_getFeatureAt_int(self.as_raw_mut_TrackerFeatureHAAR(), id) }.into_result().map(|r| unsafe { crate::tracking::CvHaarEvaluator_FeatureHaar::opencv_from_extern(r) } )
	}
	
}

/// TrackerFeature based on HAAR features, used by TrackerMIL and many others algorithms
/// 
/// Note: HAAR features implementation is copied from apps/traincascade and modified according to MIL
pub struct TrackerFeatureHAAR {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerFeatureHAAR }

impl Drop for TrackerFeatureHAAR {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerFeatureHAAR_delete(instance: *mut c_void); }
		unsafe { cv_TrackerFeatureHAAR_delete(self.as_raw_mut_TrackerFeatureHAAR()) };
	}
}

impl TrackerFeatureHAAR {
	pub fn as_raw_TrackerFeatureHAAR(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerFeatureHAAR(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerFeatureHAAR {}

impl crate::tracking::TrackerFeature for TrackerFeatureHAAR {
	fn as_raw_TrackerFeature(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::tracking::TrackerFeatureHAARTrait for TrackerFeatureHAAR {
	fn as_raw_TrackerFeatureHAAR(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerFeatureHAAR(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerFeatureHAAR {
	/// Constructor
	/// ## Parameters
	/// * parameters: TrackerFeatureHAAR parameters TrackerFeatureHAAR::Params
	/// 
	/// ## C++ default parameters
	/// * parameters: TrackerFeatureHAAR::Params()
	pub fn new(parameters: &crate::tracking::TrackerFeatureHAAR_Params) -> Result<crate::tracking::TrackerFeatureHAAR> {
		unsafe { sys::cv_TrackerFeatureHAAR_TrackerFeatureHAAR_const_ParamsX(parameters.as_raw_TrackerFeatureHAAR_Params()) }.into_result().map(|r| unsafe { crate::tracking::TrackerFeatureHAAR::opencv_from_extern(r) } )
	}
	
}

pub trait TrackerFeatureHAAR_ParamsTrait {
	fn as_raw_TrackerFeatureHAAR_Params(&self) -> *const c_void;
	fn as_raw_mut_TrackerFeatureHAAR_Params(&mut self) -> *mut c_void;

	/// # of rects
	fn num_features(&self) -> i32 {
		unsafe { sys::cv_TrackerFeatureHAAR_Params_getPropNumFeatures_const(self.as_raw_TrackerFeatureHAAR_Params()) }.into_result().expect("Infallible function failed: num_features")
	}
	
	/// # of rects
	fn set_num_features(&mut self, val: i32) -> () {
		unsafe { sys::cv_TrackerFeatureHAAR_Params_setPropNumFeatures_int(self.as_raw_mut_TrackerFeatureHAAR_Params(), val) }.into_result().expect("Infallible function failed: set_num_features")
	}
	
	/// rect size
	fn rect_size(&self) -> core::Size {
		unsafe { sys::cv_TrackerFeatureHAAR_Params_getPropRectSize_const(self.as_raw_TrackerFeatureHAAR_Params()) }.into_result().expect("Infallible function failed: rect_size")
	}
	
	/// rect size
	fn set_rect_size(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_TrackerFeatureHAAR_Params_setPropRectSize_Size(self.as_raw_mut_TrackerFeatureHAAR_Params(), val.opencv_to_extern()) }.into_result().expect("Infallible function failed: set_rect_size")
	}
	
	/// true if input images are integral, false otherwise
	fn is_integral(&self) -> bool {
		unsafe { sys::cv_TrackerFeatureHAAR_Params_getPropIsIntegral_const(self.as_raw_TrackerFeatureHAAR_Params()) }.into_result().expect("Infallible function failed: is_integral")
	}
	
	/// true if input images are integral, false otherwise
	fn set_is_integral(&mut self, val: bool) -> () {
		unsafe { sys::cv_TrackerFeatureHAAR_Params_setPropIsIntegral_bool(self.as_raw_mut_TrackerFeatureHAAR_Params(), val) }.into_result().expect("Infallible function failed: set_is_integral")
	}
	
}

pub struct TrackerFeatureHAAR_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerFeatureHAAR_Params }

impl Drop for TrackerFeatureHAAR_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerFeatureHAAR_Params_delete(instance: *mut c_void); }
		unsafe { cv_TrackerFeatureHAAR_Params_delete(self.as_raw_mut_TrackerFeatureHAAR_Params()) };
	}
}

impl TrackerFeatureHAAR_Params {
	pub fn as_raw_TrackerFeatureHAAR_Params(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerFeatureHAAR_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerFeatureHAAR_Params {}

impl crate::tracking::TrackerFeatureHAAR_ParamsTrait for TrackerFeatureHAAR_Params {
	fn as_raw_TrackerFeatureHAAR_Params(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerFeatureHAAR_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerFeatureHAAR_Params {
	pub fn default() -> Result<crate::tracking::TrackerFeatureHAAR_Params> {
		unsafe { sys::cv_TrackerFeatureHAAR_Params_Params() }.into_result().map(|r| unsafe { crate::tracking::TrackerFeatureHAAR_Params::opencv_from_extern(r) } )
	}
	
}

/// \brief TrackerFeature based on HOG
pub trait TrackerFeatureHOGTrait: crate::tracking::TrackerFeature {
	fn as_raw_TrackerFeatureHOG(&self) -> *const c_void;
	fn as_raw_mut_TrackerFeatureHOG(&mut self) -> *mut c_void;

	fn selection(&mut self, response: &mut core::Mat, npoints: i32) -> Result<()> {
		unsafe { sys::cv_TrackerFeatureHOG_selection_MatX_int(self.as_raw_mut_TrackerFeatureHOG(), response.as_raw_mut_Mat(), npoints) }.into_result()
	}
	
}

/// \brief TrackerFeature based on HOG
pub struct TrackerFeatureHOG {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerFeatureHOG }

impl Drop for TrackerFeatureHOG {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerFeatureHOG_delete(instance: *mut c_void); }
		unsafe { cv_TrackerFeatureHOG_delete(self.as_raw_mut_TrackerFeatureHOG()) };
	}
}

impl TrackerFeatureHOG {
	pub fn as_raw_TrackerFeatureHOG(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerFeatureHOG(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerFeatureHOG {}

impl crate::tracking::TrackerFeature for TrackerFeatureHOG {
	fn as_raw_TrackerFeature(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::tracking::TrackerFeatureHOGTrait for TrackerFeatureHOG {
	fn as_raw_TrackerFeatureHOG(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerFeatureHOG(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerFeatureHOG {
	pub fn default() -> Result<crate::tracking::TrackerFeatureHOG> {
		unsafe { sys::cv_TrackerFeatureHOG_TrackerFeatureHOG() }.into_result().map(|r| unsafe { crate::tracking::TrackerFeatureHOG::opencv_from_extern(r) } )
	}
	
}

/// \brief TrackerFeature based on LBP
pub trait TrackerFeatureLBPTrait: crate::tracking::TrackerFeature {
	fn as_raw_TrackerFeatureLBP(&self) -> *const c_void;
	fn as_raw_mut_TrackerFeatureLBP(&mut self) -> *mut c_void;

	fn selection(&mut self, response: &mut core::Mat, npoints: i32) -> Result<()> {
		unsafe { sys::cv_TrackerFeatureLBP_selection_MatX_int(self.as_raw_mut_TrackerFeatureLBP(), response.as_raw_mut_Mat(), npoints) }.into_result()
	}
	
}

/// \brief TrackerFeature based on LBP
pub struct TrackerFeatureLBP {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerFeatureLBP }

impl Drop for TrackerFeatureLBP {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerFeatureLBP_delete(instance: *mut c_void); }
		unsafe { cv_TrackerFeatureLBP_delete(self.as_raw_mut_TrackerFeatureLBP()) };
	}
}

impl TrackerFeatureLBP {
	pub fn as_raw_TrackerFeatureLBP(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerFeatureLBP(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerFeatureLBP {}

impl crate::tracking::TrackerFeature for TrackerFeatureLBP {
	fn as_raw_TrackerFeature(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::tracking::TrackerFeatureLBPTrait for TrackerFeatureLBP {
	fn as_raw_TrackerFeatureLBP(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerFeatureLBP(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerFeatureLBP {
	pub fn default() -> Result<crate::tracking::TrackerFeatureLBP> {
		unsafe { sys::cv_TrackerFeatureLBP_TrackerFeatureLBP() }.into_result().map(|r| unsafe { crate::tracking::TrackerFeatureLBP::opencv_from_extern(r) } )
	}
	
}

/// Class that manages the extraction and selection of features
/// 
/// [AAM](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_AAM) Feature Extraction and Feature Set Refinement (Feature Processing and Feature Selection).
/// See table I and section III C [AMVOT](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_AMVOT) Appearance modelling -\> Visual representation (Table II,
/// section 3.1 - 3.2)
/// 
/// TrackerFeatureSet is an aggregation of TrackerFeature
/// ## See also
/// TrackerFeature
pub trait TrackerFeatureSetTrait {
	fn as_raw_TrackerFeatureSet(&self) -> *const c_void;
	fn as_raw_mut_TrackerFeatureSet(&mut self) -> *mut c_void;

	/// Extract features from the images collection
	/// ## Parameters
	/// * images: The input images
	fn extraction(&mut self, images: &core::Vector::<core::Mat>) -> Result<()> {
		unsafe { sys::cv_TrackerFeatureSet_extraction_const_vector_Mat_X(self.as_raw_mut_TrackerFeatureSet(), images.as_raw_VectorOfMat()) }.into_result()
	}
	
	/// Identify most effective features for all feature types (optional)
	fn selection(&mut self) -> Result<()> {
		unsafe { sys::cv_TrackerFeatureSet_selection(self.as_raw_mut_TrackerFeatureSet()) }.into_result()
	}
	
	/// Remove outliers for all feature types (optional)
	fn remove_outliers(&mut self) -> Result<()> {
		unsafe { sys::cv_TrackerFeatureSet_removeOutliers(self.as_raw_mut_TrackerFeatureSet()) }.into_result()
	}
	
	/// Add TrackerFeature in the collection. Return true if TrackerFeature is added, false otherwise
	/// ## Parameters
	/// * trackerFeatureType: The TrackerFeature name
	/// 
	/// The modes available now:
	/// 
	/// *   "HAAR" -- Haar Feature-based
	/// 
	/// The modes that will be available soon:
	/// 
	/// *   "HOG" -- Histogram of Oriented Gradients features
	/// *   "LBP" -- Local Binary Pattern features
	/// *   "FEATURE2D" -- All types of Feature2D
	/// 
	/// Example TrackerFeatureSet::addTrackerFeature : :
	/// ```ignore
	///    //sample usage:
	/// 
	///    Ptr<TrackerFeature> trackerFeature = new TrackerFeatureHAAR( HAARparameters );
	///    featureSet->addTrackerFeature( trackerFeature );
	/// 
	///    //or add CSC sampler with default parameters
	///    //featureSet->addTrackerFeature( "HAAR" );
	/// ```
	/// 
	/// 
	/// Note: If you use the second method, you must initialize the TrackerFeature
	fn add_tracker_feature(&mut self, tracker_feature_type: &str) -> Result<bool> {
		extern_container_arg!(mut tracker_feature_type);
		unsafe { sys::cv_TrackerFeatureSet_addTrackerFeature_String(self.as_raw_mut_TrackerFeatureSet(), tracker_feature_type.opencv_to_extern_mut()) }.into_result()
	}
	
	/// Add TrackerFeature in the collection. Return true if TrackerFeature is added, false otherwise
	/// ## Parameters
	/// * trackerFeatureType: The TrackerFeature name
	/// 
	/// The modes available now:
	/// 
	/// *   "HAAR" -- Haar Feature-based
	/// 
	/// The modes that will be available soon:
	/// 
	/// *   "HOG" -- Histogram of Oriented Gradients features
	/// *   "LBP" -- Local Binary Pattern features
	/// *   "FEATURE2D" -- All types of Feature2D
	/// 
	/// Example TrackerFeatureSet::addTrackerFeature : :
	/// ```ignore
	///    //sample usage:
	/// 
	///    Ptr<TrackerFeature> trackerFeature = new TrackerFeatureHAAR( HAARparameters );
	///    featureSet->addTrackerFeature( trackerFeature );
	/// 
	///    //or add CSC sampler with default parameters
	///    //featureSet->addTrackerFeature( "HAAR" );
	/// ```
	/// 
	/// 
	/// Note: If you use the second method, you must initialize the TrackerFeature
	/// 
	/// ## Overloaded parameters
	/// 
	/// * feature: The TrackerFeature class
	fn add_tracker_feature_1(&mut self, feature: &mut core::Ptr::<dyn crate::tracking::TrackerFeature>) -> Result<bool> {
		unsafe { sys::cv_TrackerFeatureSet_addTrackerFeature_Ptr_TrackerFeature_X(self.as_raw_mut_TrackerFeatureSet(), feature.as_raw_mut_PtrOfTrackerFeature()) }.into_result()
	}
	
	/// Get the responses
	/// 
	/// 
	/// Note: Be sure to call extraction before getResponses Example TrackerFeatureSet::getResponses : :
	fn get_responses(&self) -> Result<core::Vector::<core::Mat>> {
		unsafe { sys::cv_TrackerFeatureSet_getResponses_const(self.as_raw_TrackerFeatureSet()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } )
	}
	
}

/// Class that manages the extraction and selection of features
/// 
/// [AAM](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_AAM) Feature Extraction and Feature Set Refinement (Feature Processing and Feature Selection).
/// See table I and section III C [AMVOT](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_AMVOT) Appearance modelling -\> Visual representation (Table II,
/// section 3.1 - 3.2)
/// 
/// TrackerFeatureSet is an aggregation of TrackerFeature
/// ## See also
/// TrackerFeature
pub struct TrackerFeatureSet {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerFeatureSet }

impl Drop for TrackerFeatureSet {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerFeatureSet_delete(instance: *mut c_void); }
		unsafe { cv_TrackerFeatureSet_delete(self.as_raw_mut_TrackerFeatureSet()) };
	}
}

impl TrackerFeatureSet {
	pub fn as_raw_TrackerFeatureSet(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerFeatureSet(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerFeatureSet {}

impl crate::tracking::TrackerFeatureSetTrait for TrackerFeatureSet {
	fn as_raw_TrackerFeatureSet(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerFeatureSet(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerFeatureSet {
	pub fn default() -> Result<crate::tracking::TrackerFeatureSet> {
		unsafe { sys::cv_TrackerFeatureSet_TrackerFeatureSet() }.into_result().map(|r| unsafe { crate::tracking::TrackerFeatureSet::opencv_from_extern(r) } )
	}
	
}

/// GOTURN ([GOTURN](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_GOTURN)) is kind of trackers based on Convolutional Neural Networks (CNN). While taking all advantages of CNN trackers,
/// GOTURN is much faster due to offline training without online fine-tuning nature.
/// GOTURN tracker addresses the problem of single target tracking: given a bounding box label of an object in the first frame of the video,
/// we track that object through the rest of the video. NOTE: Current method of GOTURN does not handle occlusions; however, it is fairly
/// robust to viewpoint changes, lighting changes, and deformations.
/// Inputs of GOTURN are two RGB patches representing Target and Search patches resized to 227x227.
/// Outputs of GOTURN are predicted bounding box coordinates, relative to Search patch coordinate system, in format X1,Y1,X2,Y2.
/// Original paper is here: <http://davheld.github.io/GOTURN/GOTURN.pdf>
/// As long as original authors implementation: <https://github.com/davheld/GOTURN#train-the-tracker>
/// Implementation of training algorithm is placed in separately here due to 3d-party dependencies:
/// <https://github.com/Auron-X/GOTURN_Training_Toolkit>
/// GOTURN architecture goturn.prototxt and trained model goturn.caffemodel are accessible on opencv_extra GitHub repository.
pub trait TrackerGOTURN: crate::tracking::Tracker {
	fn as_raw_TrackerGOTURN(&self) -> *const c_void;
	fn as_raw_mut_TrackerGOTURN(&mut self) -> *mut c_void;

}

impl dyn TrackerGOTURN + '_ {
	/// ## C++ default parameters
	/// * parameters: classname::Params());virtual~classname(){};/*
	///  * Partially based on:
	///  * ====================================================================================================================
	///  *   - [AAM] S. Salti, A. Cavallaro, L. Di Stefano, Adaptive Appearance Modeling for Video Tracking: Survey and Evaluation
	///  *  - [AMVOT] X. Li, W. Hu, C. Shen, Z. Zhang, A. Dick, A. van den Hengel, A Survey of Appearance Models in Visual Object Tracking
	///  *
	///  * This Tracking API has been designed with PlantUML. If you modify this API please change UML files under modules/tracking/doc/uml
	///  *
	///  */namespacecv{//! @addtogroup tracking//! @{/************************************ TrackerFeature Base Classes ************************************//** @brief Abstract base class for TrackerFeature that represents the feature.
	///  */classCV_EXPORTSTrackerFeature{public:virtual~TrackerFeature();/** @brief Compute the features in the images collection
	///     @param images The images
	///     @param response The output response
	///      */voidcompute(conststd::vector<Mat>&images,Mat&response);/** @brief Create TrackerFeature by tracker feature type
	///     @param trackerFeatureType The TrackerFeature name
	/// 
	///     The modes available now:
	/// 
	///     -   "HAAR" -- Haar Feature-based
	/// 
	///     The modes that will be available soon:
	/// 
	///     -   "HOG" -- Histogram of Oriented Gradients features
	///     -   "LBP" -- Local Binary Pattern features
	///     -   "FEATURE2D" -- All types of Feature2D
	///      */staticPtr<TrackerFeature>create(constString&trackerFeatureType);/** @brief Identify most effective features
	///     @param response Collection of response for the specific TrackerFeature
	///     @param npoints Max number of features
	/// 
	///     @note This method modifies the response parameter
	///      */virtualvoidselection(Mat&response,intnpoints)=0;/** @brief Get the name of the specific TrackerFeature
	///      */StringgetClassName()const;protected:virtualboolcomputeImpl(conststd::vector<Mat>&images,Mat&response)=0;StringclassName;};/** @brief Class that manages the extraction and selection of features
	/// 
	/// @cite AAM Feature Extraction and Feature Set Refinement (Feature Processing and Feature Selection).
	/// See table I and section III C @cite AMVOT Appearance modelling -\> Visual representation (Table II,
	/// section 3.1 - 3.2)
	/// 
	/// TrackerFeatureSet is an aggregation of TrackerFeature
	/// 
	/// @sa
	///    TrackerFeature
	/// 
	///  */classCV_EXPORTSTrackerFeatureSet{public:TrackerFeatureSet();~TrackerFeatureSet();/** @brief Extract features from the images collection
	///     @param images The input images
	///      */voidextraction(conststd::vector<Mat>&images);/** @brief Identify most effective features for all feature types (optional)
	///      */voidselection();/** @brief Remove outliers for all feature types (optional)
	///      */voidremoveOutliers();/** @brief Add TrackerFeature in the collection. Return true if TrackerFeature is added, false otherwise
	///     @param trackerFeatureType The TrackerFeature name
	/// 
	///     The modes available now:
	/// 
	///     -   "HAAR" -- Haar Feature-based
	/// 
	///     The modes that will be available soon:
	/// 
	///     -   "HOG" -- Histogram of Oriented Gradients features
	///     -   "LBP" -- Local Binary Pattern features
	///     -   "FEATURE2D" -- All types of Feature2D
	/// 
	///     Example TrackerFeatureSet::addTrackerFeature : :
	///     @code
	///         //sample usage:
	/// 
	///         Ptr<TrackerFeature> trackerFeature = new TrackerFeatureHAAR( HAARparameters );
	///         featureSet->addTrackerFeature( trackerFeature );
	/// 
	///         //or add CSC sampler with default parameters
	///         //featureSet->addTrackerFeature( "HAAR" );
	///     @endcode
	///     @note If you use the second method, you must initialize the TrackerFeature
	///      */booladdTrackerFeature(StringtrackerFeatureType);/** @overload
	///     @param feature The TrackerFeature class
	///     */booladdTrackerFeature(Ptr<TrackerFeature>&feature);/** @brief Get the TrackerFeature collection (TrackerFeature name, TrackerFeature pointer)
	///      */conststd::vector<std::pair<String,Ptr<TrackerFeature>>>&getTrackerFeature()const;/** @brief Get the responses
	/// 
	///     @note Be sure to call extraction before getResponses Example TrackerFeatureSet::getResponses : :
	///      */conststd::vector<Mat>&getResponses()const;private:voidclearResponses();boolblockAddTrackerFeature;std::vector<std::pair<String,Ptr<TrackerFeature>>>features;//list of featuresstd::vector<Mat>responses;//list of response after compute};/************************************ TrackerSampler Base Classes ************************************//** @brief Abstract base class for TrackerSamplerAlgorithm that represents the algorithm for the specific
	/// sampler.
	///  */classCV_EXPORTSTrackerSamplerAlgorithm{public:/**
	///    * \brief Destructor
	///    */virtual~TrackerSamplerAlgorithm();/** @brief Create TrackerSamplerAlgorithm by tracker sampler type.
	///     @param trackerSamplerType The trackerSamplerType name
	/// 
	///     The modes available now:
	/// 
	///     -   "CSC" -- Current State Center
	///     -   "CS" -- Current State
	///      */staticPtr<TrackerSamplerAlgorithm>create(constString&trackerSamplerType);/** @brief Computes the regions starting from a position in an image.
	/// 
	///     Return true if samples are computed, false otherwise
	/// 
	///     @param image The current frame
	///     @param boundingBox The bounding box from which regions can be calculated
	/// 
	///     @param sample The computed samples @cite AAM Fig. 1 variable Sk
	///      */boolsampling(constMat&image,RectboundingBox,std::vector<Mat>&sample);/** @brief Get the name of the specific TrackerSamplerAlgorithm
	///     */StringgetClassName()const;protected:StringclassName;virtualboolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample)=0;};/**
	///  * \brief Class that manages the sampler in order to select regions for the update the model of the tracker
	///  * [AAM] Sampling e Labeling. See table I and section III B
	///  *//** @brief Class that manages the sampler in order to select regions for the update the model of the tracker
	/// 
	/// @cite AAM Sampling e Labeling. See table I and section III B
	/// 
	/// TrackerSampler is an aggregation of TrackerSamplerAlgorithm
	/// @sa
	///    TrackerSamplerAlgorithm
	///  */classCV_EXPORTSTrackerSampler{public:/**
	///    * \brief Constructor
	///    */TrackerSampler();/**
	///    * \brief Destructor
	///    */~TrackerSampler();/** @brief Computes the regions starting from a position in an image
	///     @param image The current frame
	///     @param boundingBox The bounding box from which regions can be calculated
	///      */voidsampling(constMat&image,RectboundingBox);/** @brief Return the collection of the TrackerSamplerAlgorithm
	///     */conststd::vector<std::pair<String,Ptr<TrackerSamplerAlgorithm>>>&getSamplers()const;/** @brief Return the samples from all TrackerSamplerAlgorithm, @cite AAM Fig. 1 variable Sk
	///     */conststd::vector<Mat>&getSamples()const;/** @brief Add TrackerSamplerAlgorithm in the collection. Return true if sampler is added, false otherwise
	///     @param trackerSamplerAlgorithmType The TrackerSamplerAlgorithm name
	/// 
	///     The modes available now:
	///     -   "CSC" -- Current State Center
	///     -   "CS" -- Current State
	///     -   "PF" -- Particle Filtering
	/// 
	///     Example TrackerSamplerAlgorithm::addTrackerSamplerAlgorithm : :
	///     @code
	///          TrackerSamplerCSC::Params CSCparameters;
	///          Ptr<TrackerSamplerAlgorithm> CSCSampler = new TrackerSamplerCSC( CSCparameters );
	/// 
	///          if( !sampler->addTrackerSamplerAlgorithm( CSCSampler ) )
	///            return false;
	/// 
	///          //or add CSC sampler with default parameters
	///          //sampler->addTrackerSamplerAlgorithm( "CSC" );
	///     @endcode
	///     @note If you use the second method, you must initialize the TrackerSamplerAlgorithm
	///      */booladdTrackerSamplerAlgorithm(StringtrackerSamplerAlgorithmType);/** @overload
	///     @param sampler The TrackerSamplerAlgorithm
	///     */booladdTrackerSamplerAlgorithm(Ptr<TrackerSamplerAlgorithm>&sampler);private:std::vector<std::pair<String,Ptr<TrackerSamplerAlgorithm>>>samplers;std::vector<Mat>samples;boolblockAddTrackerSampler;voidclearSamples();};/************************************ TrackerModel Base Classes ************************************//** @brief Abstract base class for TrackerTargetState that represents a possible state of the target.
	/// 
	/// See @cite AAM \f$\hat{x}^{i}_{k}\f$ all the states candidates.
	/// 
	/// Inherits this class with your Target state, In own implementation you can add scale variation,
	/// width, height, orientation, etc.
	///  */classCV_EXPORTSTrackerTargetState{public:virtual~TrackerTargetState(){};/**
	///    * \brief Get the position
	///    * \return The position
	///    */Point2fgetTargetPosition()const;/**
	///    * \brief Set the position
	///    * \param position The position
	///    */voidsetTargetPosition(constPoint2f&position);/**
	///    * \brief Get the width of the target
	///    * \return The width of the target
	///    */intgetTargetWidth()const;/**
	///    * \brief Set the width of the target
	///    * \param width The width of the target
	///    */voidsetTargetWidth(intwidth);/**
	///    * \brief Get the height of the target
	///    * \return The height of the target
	///    */intgetTargetHeight()const;/**
	///    * \brief Set the height of the target
	///    * \param height The height of the target
	///    */voidsetTargetHeight(intheight);protected:Point2ftargetPosition;inttargetWidth;inttargetHeight;};/** @brief Represents the model of the target at frame \f$k\f$ (all states and scores)
	/// 
	/// See @cite AAM The set of the pair \f$\langle \hat{x}^{i}_{k}, C^{i}_{k} \rangle\f$
	/// @sa TrackerTargetState
	///  */typedefstd::vector<std::pair<Ptr<TrackerTargetState>,float>>ConfidenceMap;/** @brief Represents the estimate states for all frames
	/// 
	/// @cite AAM \f$x_{k}\f$ is the trajectory of the target up to time \f$k\f$
	/// 
	/// @sa TrackerTargetState
	///  */typedefstd::vector<Ptr<TrackerTargetState>>Trajectory;/** @brief Abstract base class for TrackerStateEstimator that estimates the most likely target state.
	/// 
	/// See @cite AAM State estimator
	/// 
	/// See @cite AMVOT Statistical modeling (Fig. 3), Table III (generative) - IV (discriminative) - V (hybrid)
	///  */classCV_EXPORTSTrackerStateEstimator{public:virtual~TrackerStateEstimator();/** @brief Estimate the most likely target state, return the estimated state
	///     @param confidenceMaps The overall appearance model as a list of :cConfidenceMap
	///      */Ptr<TrackerTargetState>estimate(conststd::vector<ConfidenceMap>&confidenceMaps);/** @brief Update the ConfidenceMap with the scores
	///     @param confidenceMaps The overall appearance model as a list of :cConfidenceMap
	///      */voidupdate(std::vector<ConfidenceMap>&confidenceMaps);/** @brief Create TrackerStateEstimator by tracker state estimator type
	///     @param trackeStateEstimatorType The TrackerStateEstimator name
	/// 
	///     The modes available now:
	/// 
	///     -   "BOOSTING" -- Boosting-based discriminative appearance models. See @cite AMVOT section 4.4
	/// 
	///     The modes available soon:
	/// 
	///     -   "SVM" -- SVM-based discriminative appearance models. See @cite AMVOT section 4.5
	///      */staticPtr<TrackerStateEstimator>create(constString&trackeStateEstimatorType);/** @brief Get the name of the specific TrackerStateEstimator
	///      */StringgetClassName()const;protected:virtualPtr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps)=0;virtualvoidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps)=0;StringclassName;};/** @brief Abstract class that represents the model of the target. It must be instantiated by specialized
	/// tracker
	/// 
	/// See @cite AAM Ak
	/// 
	/// Inherits this with your TrackerModel
	///  */classCV_EXPORTSTrackerModel{public:/**
	///    * \brief Constructor
	///    */TrackerModel();/**
	///    * \brief Destructor
	///    */virtual~TrackerModel();/** @brief Set TrackerEstimator, return true if the tracker state estimator is added, false otherwise
	///     @param trackerStateEstimator The TrackerStateEstimator
	///     @note You can add only one TrackerStateEstimator
	///      */boolsetTrackerStateEstimator(Ptr<TrackerStateEstimator>trackerStateEstimator);/** @brief Estimate the most likely target location
	/// 
	///     @cite AAM ME, Model Estimation table I
	///     @param responses Features extracted from TrackerFeatureSet
	///      */voidmodelEstimation(conststd::vector<Mat>&responses);/** @brief Update the model
	/// 
	///     @cite AAM MU, Model Update table I
	///      */voidmodelUpdate();/** @brief Run the TrackerStateEstimator, return true if is possible to estimate a new state, false otherwise
	///     */boolrunStateEstimator();/** @brief Set the current TrackerTargetState in the Trajectory
	///     @param lastTargetState The current TrackerTargetState
	///      */voidsetLastTargetState(constPtr<TrackerTargetState>&lastTargetState);/** @brief Get the last TrackerTargetState from Trajectory
	///     */Ptr<TrackerTargetState>getLastTargetState()const;/** @brief Get the list of the ConfidenceMap
	///     */conststd::vector<ConfidenceMap>&getConfidenceMaps()const;/** @brief Get the last ConfidenceMap for the current frame
	///      */constConfidenceMap&getLastConfidenceMap()const;/** @brief Get the TrackerStateEstimator
	///     */Ptr<TrackerStateEstimator>getTrackerStateEstimator()const;private:voidclearCurrentConfidenceMap();protected:std::vector<ConfidenceMap>confidenceMaps;Ptr<TrackerStateEstimator>stateEstimator;ConfidenceMapcurrentConfidenceMap;Trajectorytrajectory;intmaxCMLength;virtualvoidmodelEstimationImpl(conststd::vector<Mat>&responses)=0;virtualvoidmodelUpdateImpl()=0;};/************************************ Tracker Base Class ************************************//** @brief Base abstract class for the long-term tracker:
	///  */classCV_EXPORTS_WTracker:publicvirtualAlgorithm{public:virtual~Tracker();/** @brief Initialize the tracker with a know bounding box that surrounding the target
	///     @param image The initial frame
	///     @param boundingBox The initial boundig box
	/// 
	///     @return True if initialization went succesfully, false otherwise
	///      */CV_WRAPboolinit(constMat&image,constRect2d&boundingBox);/** @brief Update the tracker, find the new most likely bounding box for the target
	///     @param image The current frame
	///     @param boundingBox The boundig box that represent the new target location, if true was returned, not
	///     modified otherwise
	/// 
	///     @return True means that target was located and false means that tracker cannot locate target in
	///     current frame. Note, that latter *does not* imply that tracker has failed, maybe target is indeed
	///     missing from the frame (say, out of sight)
	///      */CV_WRAPboolupdate(constMat&image,CV_OUTRect2d&boundingBox);/** @brief Creates a tracker by its name.
	///     @param trackerType Tracker type
	/// 
	///     The following detector types are supported:
	/// 
	///     -   "MIL" -- TrackerMIL
	///     -   "BOOSTING" -- TrackerBoosting
	///      */CV_WRAPstaticPtr<Tracker>create(constString&trackerType);virtualvoidread(constFileNode&fn)=0;virtualvoidwrite(FileStorage&fs)const=0;Ptr<TrackerModel>getModel(){returnmodel;}protected:virtualboolinitImpl(constMat&image,constRect2d&boundingBox)=0;virtualboolupdateImpl(constMat&image,Rect2d&boundingBox)=0;boolisInit;Ptr<TrackerFeatureSet>featureSet;Ptr<TrackerSampler>sampler;Ptr<TrackerModel>model;};/************************************ Specific TrackerStateEstimator Classes ************************************//** @brief TrackerStateEstimator based on Boosting
	///     */classCV_EXPORTSTrackerStateEstimatorMILBoosting:publicTrackerStateEstimator{public:/**
	///    * Implementation of the target state for TrackerStateEstimatorMILBoosting
	///    */classTrackerMILTargetState:publicTrackerTargetState{public:/**
	///      * \brief Constructor
	///      * \param position Top left corner of the bounding box
	///      * \param width Width of the bounding box
	///      * \param height Height of the bounding box
	///      * \param foreground label for target or background
	///      * \param features features extracted
	///      */TrackerMILTargetState(constPoint2f&position,intwidth,intheight,boolforeground,constMat&features);/**
	///      * \brief Destructor
	///      */~TrackerMILTargetState(){};/** @brief Set label: true for target foreground, false for background
	///     @param foreground Label for background/foreground
	///      */voidsetTargetFg(boolforeground);/** @brief Set the features extracted from TrackerFeatureSet
	///     @param features The features extracted
	///      */voidsetFeatures(constMat&features);/** @brief Get the label. Return true for target foreground, false for background
	///      */boolisTargetFg()const;/** @brief Get the features extracted
	///      */MatgetFeatures()const;private:boolisTarget;MattargetFeatures;};/** @brief Constructor
	///     @param nFeatures Number of features for each sample
	///      */TrackerStateEstimatorMILBoosting(intnFeatures=250);~TrackerStateEstimatorMILBoosting();/** @brief Set the current confidenceMap
	///     @param confidenceMap The current :cConfidenceMap
	///      */voidsetCurrentConfidenceMap(ConfidenceMap&confidenceMap);protected:Ptr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps);voidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps);private:uintmax_idx(conststd::vector<float>&v);voidprepareData(constConfidenceMap&confidenceMap,Mat&positive,Mat&negative);ClfMilBoostboostMILModel;booltrained;intnumFeatures;ConfidenceMapcurrentConfidenceMap;};/** @brief TrackerStateEstimatorAdaBoosting based on ADA-Boosting
	///  */classCV_EXPORTSTrackerStateEstimatorAdaBoosting:publicTrackerStateEstimator{public:/** @brief Implementation of the target state for TrackerAdaBoostingTargetState
	///     */classTrackerAdaBoostingTargetState:publicTrackerTargetState{public:/**
	///      * \brief Constructor
	///      * \param position Top left corner of the bounding box
	///      * \param width Width of the bounding box
	///      * \param height Height of the bounding box
	///      * \param foreground label for target or background
	///      * \param responses list of features
	///      */TrackerAdaBoostingTargetState(constPoint2f&position,intwidth,intheight,boolforeground,constMat&responses);/**
	///      * \brief Destructor
	///      */~TrackerAdaBoostingTargetState(){};/** @brief Set the features extracted from TrackerFeatureSet
	///     @param responses The features extracted
	///      */voidsetTargetResponses(constMat&responses);/** @brief Set label: true for target foreground, false for background
	///     @param foreground Label for background/foreground
	///      */voidsetTargetFg(boolforeground);/** @brief Get the features extracted
	///      */MatgetTargetResponses()const;/** @brief Get the label. Return true for target foreground, false for background
	///     */boolisTargetFg()const;private:boolisTarget;MattargetResponses;};/** @brief Constructor
	///     @param numClassifer Number of base classifiers
	///     @param initIterations Number of iterations in the initialization
	///     @param nFeatures Number of features/weak classifiers
	///     @param patchSize tracking rect
	///     @param ROI initial ROI
	///      */TrackerStateEstimatorAdaBoosting(intnumClassifer,intinitIterations,intnFeatures,SizepatchSize,constRect&ROI);/**
	///    * \brief Destructor
	///    */~TrackerStateEstimatorAdaBoosting();/** @brief Get the sampling ROI
	///      */RectgetSampleROI()const;/** @brief Set the sampling ROI
	///     @param ROI the sampling ROI
	///      */voidsetSampleROI(constRect&ROI);/** @brief Set the current confidenceMap
	///     @param confidenceMap The current :cConfidenceMap
	///      */voidsetCurrentConfidenceMap(ConfidenceMap&confidenceMap);/** @brief Get the list of the selected weak classifiers for the classification step
	///      */std::vector<int>computeSelectedWeakClassifier();/** @brief Get the list of the weak classifiers that should be replaced
	///      */std::vector<int>computeReplacedClassifier();/** @brief Get the list of the weak classifiers that replace those to be replaced
	///      */std::vector<int>computeSwappedClassifier();protected:Ptr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps);voidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps);Ptr<StrongClassifierDirectSelection>boostClassifier;private:intnumBaseClassifier;intiterationInit;intnumFeatures;booltrained;SizeinitPatchSize;RectsampleROI;std::vector<int>replacedClassifier;std::vector<int>swappedClassifier;ConfidenceMapcurrentConfidenceMap;};/**
	///  * \brief TrackerStateEstimator based on SVM
	///  */classCV_EXPORTSTrackerStateEstimatorSVM:publicTrackerStateEstimator{public:TrackerStateEstimatorSVM();~TrackerStateEstimatorSVM();protected:Ptr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps);voidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps);};/************************************ Specific TrackerSamplerAlgorithm Classes ************************************//** @brief TrackerSampler based on CSC (current state centered), used by MIL algorithm TrackerMIL
	///  */classCV_EXPORTSTrackerSamplerCSC:publicTrackerSamplerAlgorithm{public:enum{MODE_INIT_POS=1,//!< mode for init positive samplesMODE_INIT_NEG=2,//!< mode for init negative samplesMODE_TRACK_POS=3,//!< mode for update positive samplesMODE_TRACK_NEG=4,//!< mode for update negative samplesMODE_DETECT=5//!< mode for detect samples};structCV_EXPORTSParams{Params();floatinitInRad;//!< radius for gathering positive instances during initfloattrackInPosRad;//!< radius for gathering positive instances during trackingfloatsearchWinSize;//!< size of search windowintinitMaxNegNum;//!< # negative samples to use during initinttrackMaxPosNum;//!< # positive samples to use during traininginttrackMaxNegNum;//!< # negative samples to use during training};/** @brief Constructor
	///     @param parameters TrackerSamplerCSC parameters TrackerSamplerCSC::Params
	///      */TrackerSamplerCSC(constTrackerSamplerCSC::Params&parameters=TrackerSamplerCSC::Params());/** @brief Set the sampling mode of TrackerSamplerCSC
	///     @param samplingMode The sampling mode
	/// 
	///     The modes are:
	/// 
	///     -   "MODE_INIT_POS = 1" -- for the positive sampling in initialization step
	///     -   "MODE_INIT_NEG = 2" -- for the negative sampling in initialization step
	///     -   "MODE_TRACK_POS = 3" -- for the positive sampling in update step
	///     -   "MODE_TRACK_NEG = 4" -- for the negative sampling in update step
	///     -   "MODE_DETECT = 5" -- for the sampling in detection step
	///      */voidsetMode(intsamplingMode);~TrackerSamplerCSC();protected:boolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample);private:Paramsparams;intmode;RNGrng;std::vector<Mat>sampleImage(constMat&img,intx,inty,intw,inth,floatinrad,floatoutrad=0,intmaxnum=1000000);};/** @brief TrackerSampler based on CS (current state), used by algorithm TrackerBoosting
	///  */classCV_EXPORTSTrackerSamplerCS:publicTrackerSamplerAlgorithm{public:enum{MODE_POSITIVE=1,//!< mode for positive samplesMODE_NEGATIVE=2,//!< mode for negative samplesMODE_CLASSIFY=3//!< mode for classify samples};structCV_EXPORTSParams{Params();floatoverlap;//!<overlapping for the search windowsfloatsearchFactor;//!<search region parameter};/** @brief Constructor
	///     @param parameters TrackerSamplerCS parameters TrackerSamplerCS::Params
	///      */TrackerSamplerCS(constTrackerSamplerCS::Params&parameters=TrackerSamplerCS::Params());/** @brief Set the sampling mode of TrackerSamplerCS
	///     @param samplingMode The sampling mode
	/// 
	///     The modes are:
	/// 
	///     -   "MODE_POSITIVE = 1" -- for the positive sampling
	///     -   "MODE_NEGATIVE = 2" -- for the negative sampling
	///     -   "MODE_CLASSIFY = 3" -- for the sampling in classification step
	///      */voidsetMode(intsamplingMode);~TrackerSamplerCS();boolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample);RectgetROI()const;private:RectgetTrackingROI(floatsearchFactor);RectRectMultiply(constRect&rect,floatf);std::vector<Mat>patchesRegularScan(constMat&image,RecttrackingROI,SizepatchSize);voidsetCheckedROI(RectimageROI);Paramsparams;intmode;RecttrackedPatch;RectvalidROI;RectROI;};/** @brief This sampler is based on particle filtering.
	/// 
	/// In principle, it can be thought of as performing some sort of optimization (and indeed, this
	/// tracker uses opencv's optim module), where tracker seeks to find the rectangle in given frame,
	/// which is the most *"similar"* to the initial rectangle (the one, given through the constructor).
	/// 
	/// The optimization performed is stochastic and somehow resembles genetic algorithms, where on each new
	/// image received (submitted via TrackerSamplerPF::sampling()) we start with the region bounded by
	/// boundingBox, then generate several "perturbed" boxes, take the ones most similar to the original.
	/// This selection round is repeated several times. At the end, we hope that only the most promising box
	/// remaining, and these are combined to produce the subrectangle of image, which is put as a sole
	/// element in array sample.
	/// 
	/// It should be noted, that the definition of "similarity" between two rectangles is based on comparing
	/// their histograms. As experiments show, tracker is *not* very succesfull if target is assumed to
	/// strongly change its dimensions.
	///  */classCV_EXPORTSTrackerSamplerPF:publicTrackerSamplerAlgorithm{public:/** @brief This structure contains all the parameters that can be varied during the course of sampling
	///     algorithm. Below is the structure exposed, together with its members briefly explained with
	///     reference to the above discussion on algorithm's working.
	///  */structCV_EXPORTSParams{Params();intiterationNum;//!< number of selection roundsintparticlesNum;//!< number of "perturbed" boxes on each rounddoublealpha;//!< with each new round we exponentially decrease the amount of "perturbing" we allow (like in simulated annealing)//!< and this very alpha controls how fast annealing happens, ie. how fast perturbing decreasesMat_<double>std;//!< initial values for perturbing (1-by-4 array, as each rectangle is given by 4 values -- coordinates of opposite vertices,//!< hence we have 4 values to perturb)};/** @brief Constructor
	///     @param chosenRect Initial rectangle, that is supposed to contain target we'd like to track.
	///     @param parameters
	///      */TrackerSamplerPF(constMat&chosenRect,constTrackerSamplerPF::Params&parameters=TrackerSamplerPF::Params());protected:boolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample);private:Paramsparams;Ptr<MinProblemSolver>_solver;Ptr<MinProblemSolver::Function>_function;};/************************************ Specific TrackerFeature Classes ************************************//**
	///  * \brief TrackerFeature based on Feature2D
	///  */classCV_EXPORTSTrackerFeatureFeature2d:publicTrackerFeature{public:/**
	///    * \brief Constructor
	///    * \param detectorType string of FeatureDetector
	///    * \param descriptorType string of DescriptorExtractor
	///    */TrackerFeatureFeature2d(StringdetectorType,StringdescriptorType);~TrackerFeatureFeature2d();voidselection(Mat&response,intnpoints);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);private:std::vector<KeyPoint>keypoints;};/**
	///  * \brief TrackerFeature based on HOG
	///  */classCV_EXPORTSTrackerFeatureHOG:publicTrackerFeature{public:TrackerFeatureHOG();~TrackerFeatureHOG();voidselection(Mat&response,intnpoints);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);};/** @brief TrackerFeature based on HAAR features, used by TrackerMIL and many others algorithms
	/// @note HAAR features implementation is copied from apps/traincascade and modified according to MIL
	///  */classCV_EXPORTSTrackerFeatureHAAR:publicTrackerFeature{public:structCV_EXPORTSParams{Params();intnumFeatures;//!< # of rectsSizerectSize;//!< rect sizeboolisIntegral;//!< true if input images are integral, false otherwise};/** @brief Constructor
	///     @param parameters TrackerFeatureHAAR parameters TrackerFeatureHAAR::Params
	///      */TrackerFeatureHAAR(constTrackerFeatureHAAR::Params&parameters=TrackerFeatureHAAR::Params());~TrackerFeatureHAAR();/** @brief Compute the features only for the selected indices in the images collection
	///     @param selFeatures indices of selected features
	///     @param images The images
	///     @param response Collection of response for the specific TrackerFeature
	///      */boolextractSelected(conststd::vector<int>selFeatures,conststd::vector<Mat>&images,Mat&response);/** @brief Identify most effective features
	///     @param response Collection of response for the specific TrackerFeature
	///     @param npoints Max number of features
	/// 
	///     @note This method modifies the response parameter
	///      */voidselection(Mat&response,intnpoints);/** @brief Swap the feature in position source with the feature in position target
	///   @param source The source position
	///   @param target The target position
	///  */boolswapFeature(intsource,inttarget);/** @brief   Swap the feature in position id with the feature input
	///   @param id The position
	///   @param feature The feature
	///  */boolswapFeature(intid,CvHaarEvaluator::FeatureHaar&feature);/** @brief Get the feature in position id
	///     @param id The position
	///      */CvHaarEvaluator::FeatureHaar&getFeatureAt(intid);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);private:Paramsparams;Ptr<CvHaarEvaluator>featureEvaluator;};/**
	///  * \brief TrackerFeature based on LBP
	///  */classCV_EXPORTSTrackerFeatureLBP:publicTrackerFeature{public:TrackerFeatureLBP();~TrackerFeatureLBP();voidselection(Mat&response,intnpoints);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);};/************************************ Specific Tracker Classes ************************************//** @brief The MIL algorithm trains a classifier in an online manner to separate the object from the
	/// background.
	/// 
	/// Multiple Instance Learning avoids the drift problem for a robust tracking. The implementation is
	/// based on @cite MIL .
	/// 
	/// Original code can be found here <http://vision.ucsd.edu/~bbabenko/project_miltrack.shtml>
	///  */classCV_EXPORTSTrackerMIL:publicTracker{public:structCV_EXPORTSParams{Params();//parameters for samplerfloatsamplerInitInRadius;//!< radius for gathering positive instances during initintsamplerInitMaxNegNum;//!< # negative samples to use during initfloatsamplerSearchWinSize;//!< size of search windowfloatsamplerTrackInRadius;//!< radius for gathering positive instances during trackingintsamplerTrackMaxPosNum;//!< # positive samples to use during trackingintsamplerTrackMaxNegNum;//!< # negative samples to use during trackingintfeatureSetNumFeatures;//!< # featuresvoidread(constFileNode&fn);voidwrite(FileStorage&fs)const;};/** @brief Constructor
	///     @param parameters MIL parameters TrackerMIL::Params
	///      */BOILERPLATE_CODE("MIL",TrackerMIL);};/** @brief This is a real-time object tracking based on a novel on-line version of the AdaBoost algorithm.
	/// 
	/// The classifier uses the surrounding background as negative examples in update step to avoid the
	/// drifting problem. The implementation is based on @cite OLB .
	///  */classCV_EXPORTSTrackerBoosting:publicTracker{public:structCV_EXPORTSParams{Params();intnumClassifiers;//!<the number of classifiers to use in a OnlineBoosting algorithmfloatsamplerOverlap;//!<search region parameters to use in a OnlineBoosting algorithmfloatsamplerSearchFactor;//!< search region parameters to use in a OnlineBoosting algorithmintiterationInit;//!<the initial iterationsintfeatureSetNumFeatures;//!< # features/**
	///      * \brief Read parameters from file
	///      */voidread(constFileNode&fn);/**
	///      * \brief Write parameters in a file
	///      */voidwrite(FileStorage&fs)const;};/** @brief Constructor
	///     @param parameters BOOSTING parameters TrackerBoosting::Params
	///      */BOILERPLATE_CODE("BOOSTING",TrackerBoosting);};/** @brief Median Flow tracker implementation.
	/// 
	/// Implementation of a paper @cite MedianFlow .
	/// 
	/// The tracker is suitable for very smooth and predictable movements when object is visible throughout
	/// the whole sequence. It's quite and accurate for this type of problems (in particular, it was shown
	/// by authors to outperform MIL). During the implementation period the code at
	/// <http://www.aonsquared.co.uk/node/5>, the courtesy of the author Arthur Amarra, was used for the
	/// reference purpose.
	///  */classCV_EXPORTSTrackerMedianFlow:publicTracker{public:structCV_EXPORTSParams{Params();intpointsInGrid;//!<square root of number of keypoints used; increase it to trade//!<accurateness for speed; default value is sensible and recommendedvoidread(constFileNode&/*fn*/);voidwrite(FileStorage&/*fs*/)const;};/** @brief Constructor
	///     @param parameters Median Flow parameters TrackerMedianFlow::Params
	///     */BOILERPLATE_CODE("MEDIANFLOW",TrackerMedianFlow);};/** @brief TLD is a novel tracking framework that explicitly decomposes the long-term tracking task into
	/// tracking, learning and detection.
	/// 
	/// The tracker follows the object from frame to frame. The detector localizes all appearances that
	/// have been observed so far and corrects the tracker if necessary. The learning estimates detector’s
	/// errors and updates it to avoid these errors in the future. The implementation is based on @cite TLD .
	/// 
	/// The Median Flow algorithm (see cv::TrackerMedianFlow) was chosen as a tracking component in this
	/// implementation, following authors. Tracker is supposed to be able to handle rapid motions, partial
	/// occlusions, object absence etc.
	///  */classCV_EXPORTSTrackerTLD:publicTracker{public:structCV_EXPORTSParams{Params();voidread(constFileNode&/*fn*/);voidwrite(FileStorage&/*fs*/)const;};/** @brief Constructor
	///     @param parameters TLD parameters TrackerTLD::Params
	///      */BOILERPLATE_CODE("TLD",TrackerTLD);};/** @brief KCF is a novel tracking framework that utilizes properties of circulant matrix to enhance the processing speed.
	///  * This tracking method is an implementation of @cite KCF_ECCV which is extended to KFC with color-names features (@cite KCF_CN).
	///  * The original paper of KCF is available at <http://home.isr.uc.pt/~henriques/circulant/index.html>
	///  * as well as the matlab implementation. For more information about KCF with color-names features, please refer to
	///  * <http://www.cvl.isy.liu.se/research/objrec/visualtracking/colvistrack/index.html>.
	///  */classCV_EXPORTSTrackerKCF:publicTracker{public:/**
	///   * \brief Feature type to be used in the tracking grayscale, colornames, compressed color-names
	///   * The modes available now:
	///   -   "GRAY" -- Use grayscale values as the feature
	///   -   "CN" -- Color-names feature
	///   */enumMODE{GRAY=(1u<<0),CN=(1u<<1),CUSTOM=(1u<<2)};structCV_EXPORTSParams{/**
	///     * \brief Constructor
	///     */Params();/**
	///     * \brief Read parameters from file, currently unused
	///     */voidread(constFileNode&/*fn*/);/**
	///     * \brief Read parameters from file, currently unused
	///     */voidwrite(FileStorage&/*fs*/)const;doublesigma;//!<  gaussian kernel bandwidthdoublelambda;//!<  regularizationdoubleinterp_factor;//!<  linear interpolation factor for adaptationdoubleoutput_sigma_factor;//!<  spatial bandwidth (proportional to target)doublepca_learning_rate;//!<  compression learning rateboolresize;//!<  activate the resize feature to improve the processing speedboolsplit_coeff;//!<  split the training coefficients into two matricesboolwrap_kernel;//!<  wrap around the kernel valuesboolcompress_feature;//!<  activate the pca method to compress the featuresintmax_patch_size;//!<  threshold for the ROI sizeintcompressed_size;//!<  feature size after compressionunsignedintdesc_pca;//!<  compressed descriptors of TrackerKCF::MODEunsignedintdesc_npca;//!<  non-compressed descriptors of TrackerKCF::MODE};virtualvoidsetFeatureExtractor(void(*)(constMat,constRect,Mat&),boolpca_func=false);/** @brief Constructor
	///   @param parameters KCF parameters TrackerKCF::Params
	///   */BOILERPLATE_CODE("KCF",TrackerKCF);};/** @brief GOTURN (@cite GOTURN) is kind of trackers based on Convolutional Neural Networks (CNN). While taking all advantages of CNN trackers,
	///  *  GOTURN is much faster due to offline training without online fine-tuning nature.
	///  *  GOTURN tracker addresses the problem of single target tracking: given a bounding box label of an object in the first frame of the video,
	///  *  we track that object through the rest of the video. NOTE: Current method of GOTURN does not handle occlusions; however, it is fairly
	///  *  robust to viewpoint changes, lighting changes, and deformations.
	///  *  Inputs of GOTURN are two RGB patches representing Target and Search patches resized to 227x227.
	///  *  Outputs of GOTURN are predicted bounding box coordinates, relative to Search patch coordinate system, in format X1,Y1,X2,Y2.
	///  *  Original paper is here: <http://davheld.github.io/GOTURN/GOTURN.pdf>
	///  *  As long as original authors implementation: <https://github.com/davheld/GOTURN#train-the-tracker>
	///  *  Implementation of training algorithm is placed in separately here due to 3d-party dependencies:
	///  *  <https://github.com/Auron-X/GOTURN_Training_Toolkit>
	///  *  GOTURN architecture goturn.prototxt and trained model goturn.caffemodel are accessible on opencv_extra GitHub repository.
	/// */classCV_EXPORTSTrackerGOTURN:publicTracker{public:structCV_EXPORTSParams{Params();voidread(constFileNode&/*fn*/);voidwrite(FileStorage&/*fs*/)const;};/** @brief Constructor
	///   @param parameters GOTURN parameters TrackerGOTURN::Params
	///   */BOILERPLATE_CODE("GOTURN",TrackerGOTURN)
	pub fn create_tracker(parameters: &crate::tracking::TrackerGOTURN_Params) -> Result<core::Ptr::<dyn crate::tracking::TrackerGOTURN>> {
		unsafe { sys::cv_TrackerGOTURN_createTracker_const_ParamsX(parameters.as_raw_TrackerGOTURN_Params()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::tracking::TrackerGOTURN>::opencv_from_extern(r) } )
	}
	
}
pub trait TrackerGOTURN_ParamsTrait {
	fn as_raw_TrackerGOTURN_Params(&self) -> *const c_void;
	fn as_raw_mut_TrackerGOTURN_Params(&mut self) -> *mut c_void;

	fn read(&mut self, unnamed: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_TrackerGOTURN_Params_read_const_FileNodeX(self.as_raw_mut_TrackerGOTURN_Params(), unnamed.as_raw_FileNode()) }.into_result()
	}
	
	fn write(&self, unnamed: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_TrackerGOTURN_Params_write_const_FileStorageX(self.as_raw_TrackerGOTURN_Params(), unnamed.as_raw_mut_FileStorage()) }.into_result()
	}
	
}

pub struct TrackerGOTURN_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerGOTURN_Params }

impl Drop for TrackerGOTURN_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerGOTURN_Params_delete(instance: *mut c_void); }
		unsafe { cv_TrackerGOTURN_Params_delete(self.as_raw_mut_TrackerGOTURN_Params()) };
	}
}

impl TrackerGOTURN_Params {
	pub fn as_raw_TrackerGOTURN_Params(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerGOTURN_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerGOTURN_Params {}

impl crate::tracking::TrackerGOTURN_ParamsTrait for TrackerGOTURN_Params {
	fn as_raw_TrackerGOTURN_Params(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerGOTURN_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerGOTURN_Params {
	pub fn default() -> Result<crate::tracking::TrackerGOTURN_Params> {
		unsafe { sys::cv_TrackerGOTURN_Params_Params() }.into_result().map(|r| unsafe { crate::tracking::TrackerGOTURN_Params::opencv_from_extern(r) } )
	}
	
}

/// KCF is a novel tracking framework that utilizes properties of circulant matrix to enhance the processing speed.
/// This tracking method is an implementation of [KCF_ECCV](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_KCF_ECCV) which is extended to KFC with color-names features ([KCF_CN](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_KCF_CN)).
/// The original paper of KCF is available at <http://home.isr.uc.pt/~henriques/circulant/index.html>
/// as well as the matlab implementation. For more information about KCF with color-names features, please refer to
/// <http://www.cvl.isy.liu.se/research/objrec/visualtracking/colvistrack/index.html>.
pub trait TrackerKCF: crate::tracking::Tracker {
	fn as_raw_TrackerKCF(&self) -> *const c_void;
	fn as_raw_mut_TrackerKCF(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * pca_func: false
	fn set_feature_extractor(&mut self, unnamed: Option<unsafe extern "C" fn(*const c_void, core::Rect, *mut c_void) -> ()>, pca_func: bool) -> Result<()> {
		unsafe { sys::cv_TrackerKCF_setFeatureExtractor_void__X__cv_Mat__cv_Rect__cv_Mat__bool(self.as_raw_mut_TrackerKCF(), unnamed, pca_func) }.into_result()
	}
	
}

impl dyn TrackerKCF + '_ {
	/// ## C++ default parameters
	/// * parameters: classname::Params());virtual~classname(){};/*
	///  * Partially based on:
	///  * ====================================================================================================================
	///  *   - [AAM] S. Salti, A. Cavallaro, L. Di Stefano, Adaptive Appearance Modeling for Video Tracking: Survey and Evaluation
	///  *  - [AMVOT] X. Li, W. Hu, C. Shen, Z. Zhang, A. Dick, A. van den Hengel, A Survey of Appearance Models in Visual Object Tracking
	///  *
	///  * This Tracking API has been designed with PlantUML. If you modify this API please change UML files under modules/tracking/doc/uml
	///  *
	///  */namespacecv{//! @addtogroup tracking//! @{/************************************ TrackerFeature Base Classes ************************************//** @brief Abstract base class for TrackerFeature that represents the feature.
	///  */classCV_EXPORTSTrackerFeature{public:virtual~TrackerFeature();/** @brief Compute the features in the images collection
	///     @param images The images
	///     @param response The output response
	///      */voidcompute(conststd::vector<Mat>&images,Mat&response);/** @brief Create TrackerFeature by tracker feature type
	///     @param trackerFeatureType The TrackerFeature name
	/// 
	///     The modes available now:
	/// 
	///     -   "HAAR" -- Haar Feature-based
	/// 
	///     The modes that will be available soon:
	/// 
	///     -   "HOG" -- Histogram of Oriented Gradients features
	///     -   "LBP" -- Local Binary Pattern features
	///     -   "FEATURE2D" -- All types of Feature2D
	///      */staticPtr<TrackerFeature>create(constString&trackerFeatureType);/** @brief Identify most effective features
	///     @param response Collection of response for the specific TrackerFeature
	///     @param npoints Max number of features
	/// 
	///     @note This method modifies the response parameter
	///      */virtualvoidselection(Mat&response,intnpoints)=0;/** @brief Get the name of the specific TrackerFeature
	///      */StringgetClassName()const;protected:virtualboolcomputeImpl(conststd::vector<Mat>&images,Mat&response)=0;StringclassName;};/** @brief Class that manages the extraction and selection of features
	/// 
	/// @cite AAM Feature Extraction and Feature Set Refinement (Feature Processing and Feature Selection).
	/// See table I and section III C @cite AMVOT Appearance modelling -\> Visual representation (Table II,
	/// section 3.1 - 3.2)
	/// 
	/// TrackerFeatureSet is an aggregation of TrackerFeature
	/// 
	/// @sa
	///    TrackerFeature
	/// 
	///  */classCV_EXPORTSTrackerFeatureSet{public:TrackerFeatureSet();~TrackerFeatureSet();/** @brief Extract features from the images collection
	///     @param images The input images
	///      */voidextraction(conststd::vector<Mat>&images);/** @brief Identify most effective features for all feature types (optional)
	///      */voidselection();/** @brief Remove outliers for all feature types (optional)
	///      */voidremoveOutliers();/** @brief Add TrackerFeature in the collection. Return true if TrackerFeature is added, false otherwise
	///     @param trackerFeatureType The TrackerFeature name
	/// 
	///     The modes available now:
	/// 
	///     -   "HAAR" -- Haar Feature-based
	/// 
	///     The modes that will be available soon:
	/// 
	///     -   "HOG" -- Histogram of Oriented Gradients features
	///     -   "LBP" -- Local Binary Pattern features
	///     -   "FEATURE2D" -- All types of Feature2D
	/// 
	///     Example TrackerFeatureSet::addTrackerFeature : :
	///     @code
	///         //sample usage:
	/// 
	///         Ptr<TrackerFeature> trackerFeature = new TrackerFeatureHAAR( HAARparameters );
	///         featureSet->addTrackerFeature( trackerFeature );
	/// 
	///         //or add CSC sampler with default parameters
	///         //featureSet->addTrackerFeature( "HAAR" );
	///     @endcode
	///     @note If you use the second method, you must initialize the TrackerFeature
	///      */booladdTrackerFeature(StringtrackerFeatureType);/** @overload
	///     @param feature The TrackerFeature class
	///     */booladdTrackerFeature(Ptr<TrackerFeature>&feature);/** @brief Get the TrackerFeature collection (TrackerFeature name, TrackerFeature pointer)
	///      */conststd::vector<std::pair<String,Ptr<TrackerFeature>>>&getTrackerFeature()const;/** @brief Get the responses
	/// 
	///     @note Be sure to call extraction before getResponses Example TrackerFeatureSet::getResponses : :
	///      */conststd::vector<Mat>&getResponses()const;private:voidclearResponses();boolblockAddTrackerFeature;std::vector<std::pair<String,Ptr<TrackerFeature>>>features;//list of featuresstd::vector<Mat>responses;//list of response after compute};/************************************ TrackerSampler Base Classes ************************************//** @brief Abstract base class for TrackerSamplerAlgorithm that represents the algorithm for the specific
	/// sampler.
	///  */classCV_EXPORTSTrackerSamplerAlgorithm{public:/**
	///    * \brief Destructor
	///    */virtual~TrackerSamplerAlgorithm();/** @brief Create TrackerSamplerAlgorithm by tracker sampler type.
	///     @param trackerSamplerType The trackerSamplerType name
	/// 
	///     The modes available now:
	/// 
	///     -   "CSC" -- Current State Center
	///     -   "CS" -- Current State
	///      */staticPtr<TrackerSamplerAlgorithm>create(constString&trackerSamplerType);/** @brief Computes the regions starting from a position in an image.
	/// 
	///     Return true if samples are computed, false otherwise
	/// 
	///     @param image The current frame
	///     @param boundingBox The bounding box from which regions can be calculated
	/// 
	///     @param sample The computed samples @cite AAM Fig. 1 variable Sk
	///      */boolsampling(constMat&image,RectboundingBox,std::vector<Mat>&sample);/** @brief Get the name of the specific TrackerSamplerAlgorithm
	///     */StringgetClassName()const;protected:StringclassName;virtualboolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample)=0;};/**
	///  * \brief Class that manages the sampler in order to select regions for the update the model of the tracker
	///  * [AAM] Sampling e Labeling. See table I and section III B
	///  *//** @brief Class that manages the sampler in order to select regions for the update the model of the tracker
	/// 
	/// @cite AAM Sampling e Labeling. See table I and section III B
	/// 
	/// TrackerSampler is an aggregation of TrackerSamplerAlgorithm
	/// @sa
	///    TrackerSamplerAlgorithm
	///  */classCV_EXPORTSTrackerSampler{public:/**
	///    * \brief Constructor
	///    */TrackerSampler();/**
	///    * \brief Destructor
	///    */~TrackerSampler();/** @brief Computes the regions starting from a position in an image
	///     @param image The current frame
	///     @param boundingBox The bounding box from which regions can be calculated
	///      */voidsampling(constMat&image,RectboundingBox);/** @brief Return the collection of the TrackerSamplerAlgorithm
	///     */conststd::vector<std::pair<String,Ptr<TrackerSamplerAlgorithm>>>&getSamplers()const;/** @brief Return the samples from all TrackerSamplerAlgorithm, @cite AAM Fig. 1 variable Sk
	///     */conststd::vector<Mat>&getSamples()const;/** @brief Add TrackerSamplerAlgorithm in the collection. Return true if sampler is added, false otherwise
	///     @param trackerSamplerAlgorithmType The TrackerSamplerAlgorithm name
	/// 
	///     The modes available now:
	///     -   "CSC" -- Current State Center
	///     -   "CS" -- Current State
	///     -   "PF" -- Particle Filtering
	/// 
	///     Example TrackerSamplerAlgorithm::addTrackerSamplerAlgorithm : :
	///     @code
	///          TrackerSamplerCSC::Params CSCparameters;
	///          Ptr<TrackerSamplerAlgorithm> CSCSampler = new TrackerSamplerCSC( CSCparameters );
	/// 
	///          if( !sampler->addTrackerSamplerAlgorithm( CSCSampler ) )
	///            return false;
	/// 
	///          //or add CSC sampler with default parameters
	///          //sampler->addTrackerSamplerAlgorithm( "CSC" );
	///     @endcode
	///     @note If you use the second method, you must initialize the TrackerSamplerAlgorithm
	///      */booladdTrackerSamplerAlgorithm(StringtrackerSamplerAlgorithmType);/** @overload
	///     @param sampler The TrackerSamplerAlgorithm
	///     */booladdTrackerSamplerAlgorithm(Ptr<TrackerSamplerAlgorithm>&sampler);private:std::vector<std::pair<String,Ptr<TrackerSamplerAlgorithm>>>samplers;std::vector<Mat>samples;boolblockAddTrackerSampler;voidclearSamples();};/************************************ TrackerModel Base Classes ************************************//** @brief Abstract base class for TrackerTargetState that represents a possible state of the target.
	/// 
	/// See @cite AAM \f$\hat{x}^{i}_{k}\f$ all the states candidates.
	/// 
	/// Inherits this class with your Target state, In own implementation you can add scale variation,
	/// width, height, orientation, etc.
	///  */classCV_EXPORTSTrackerTargetState{public:virtual~TrackerTargetState(){};/**
	///    * \brief Get the position
	///    * \return The position
	///    */Point2fgetTargetPosition()const;/**
	///    * \brief Set the position
	///    * \param position The position
	///    */voidsetTargetPosition(constPoint2f&position);/**
	///    * \brief Get the width of the target
	///    * \return The width of the target
	///    */intgetTargetWidth()const;/**
	///    * \brief Set the width of the target
	///    * \param width The width of the target
	///    */voidsetTargetWidth(intwidth);/**
	///    * \brief Get the height of the target
	///    * \return The height of the target
	///    */intgetTargetHeight()const;/**
	///    * \brief Set the height of the target
	///    * \param height The height of the target
	///    */voidsetTargetHeight(intheight);protected:Point2ftargetPosition;inttargetWidth;inttargetHeight;};/** @brief Represents the model of the target at frame \f$k\f$ (all states and scores)
	/// 
	/// See @cite AAM The set of the pair \f$\langle \hat{x}^{i}_{k}, C^{i}_{k} \rangle\f$
	/// @sa TrackerTargetState
	///  */typedefstd::vector<std::pair<Ptr<TrackerTargetState>,float>>ConfidenceMap;/** @brief Represents the estimate states for all frames
	/// 
	/// @cite AAM \f$x_{k}\f$ is the trajectory of the target up to time \f$k\f$
	/// 
	/// @sa TrackerTargetState
	///  */typedefstd::vector<Ptr<TrackerTargetState>>Trajectory;/** @brief Abstract base class for TrackerStateEstimator that estimates the most likely target state.
	/// 
	/// See @cite AAM State estimator
	/// 
	/// See @cite AMVOT Statistical modeling (Fig. 3), Table III (generative) - IV (discriminative) - V (hybrid)
	///  */classCV_EXPORTSTrackerStateEstimator{public:virtual~TrackerStateEstimator();/** @brief Estimate the most likely target state, return the estimated state
	///     @param confidenceMaps The overall appearance model as a list of :cConfidenceMap
	///      */Ptr<TrackerTargetState>estimate(conststd::vector<ConfidenceMap>&confidenceMaps);/** @brief Update the ConfidenceMap with the scores
	///     @param confidenceMaps The overall appearance model as a list of :cConfidenceMap
	///      */voidupdate(std::vector<ConfidenceMap>&confidenceMaps);/** @brief Create TrackerStateEstimator by tracker state estimator type
	///     @param trackeStateEstimatorType The TrackerStateEstimator name
	/// 
	///     The modes available now:
	/// 
	///     -   "BOOSTING" -- Boosting-based discriminative appearance models. See @cite AMVOT section 4.4
	/// 
	///     The modes available soon:
	/// 
	///     -   "SVM" -- SVM-based discriminative appearance models. See @cite AMVOT section 4.5
	///      */staticPtr<TrackerStateEstimator>create(constString&trackeStateEstimatorType);/** @brief Get the name of the specific TrackerStateEstimator
	///      */StringgetClassName()const;protected:virtualPtr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps)=0;virtualvoidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps)=0;StringclassName;};/** @brief Abstract class that represents the model of the target. It must be instantiated by specialized
	/// tracker
	/// 
	/// See @cite AAM Ak
	/// 
	/// Inherits this with your TrackerModel
	///  */classCV_EXPORTSTrackerModel{public:/**
	///    * \brief Constructor
	///    */TrackerModel();/**
	///    * \brief Destructor
	///    */virtual~TrackerModel();/** @brief Set TrackerEstimator, return true if the tracker state estimator is added, false otherwise
	///     @param trackerStateEstimator The TrackerStateEstimator
	///     @note You can add only one TrackerStateEstimator
	///      */boolsetTrackerStateEstimator(Ptr<TrackerStateEstimator>trackerStateEstimator);/** @brief Estimate the most likely target location
	/// 
	///     @cite AAM ME, Model Estimation table I
	///     @param responses Features extracted from TrackerFeatureSet
	///      */voidmodelEstimation(conststd::vector<Mat>&responses);/** @brief Update the model
	/// 
	///     @cite AAM MU, Model Update table I
	///      */voidmodelUpdate();/** @brief Run the TrackerStateEstimator, return true if is possible to estimate a new state, false otherwise
	///     */boolrunStateEstimator();/** @brief Set the current TrackerTargetState in the Trajectory
	///     @param lastTargetState The current TrackerTargetState
	///      */voidsetLastTargetState(constPtr<TrackerTargetState>&lastTargetState);/** @brief Get the last TrackerTargetState from Trajectory
	///     */Ptr<TrackerTargetState>getLastTargetState()const;/** @brief Get the list of the ConfidenceMap
	///     */conststd::vector<ConfidenceMap>&getConfidenceMaps()const;/** @brief Get the last ConfidenceMap for the current frame
	///      */constConfidenceMap&getLastConfidenceMap()const;/** @brief Get the TrackerStateEstimator
	///     */Ptr<TrackerStateEstimator>getTrackerStateEstimator()const;private:voidclearCurrentConfidenceMap();protected:std::vector<ConfidenceMap>confidenceMaps;Ptr<TrackerStateEstimator>stateEstimator;ConfidenceMapcurrentConfidenceMap;Trajectorytrajectory;intmaxCMLength;virtualvoidmodelEstimationImpl(conststd::vector<Mat>&responses)=0;virtualvoidmodelUpdateImpl()=0;};/************************************ Tracker Base Class ************************************//** @brief Base abstract class for the long-term tracker:
	///  */classCV_EXPORTS_WTracker:publicvirtualAlgorithm{public:virtual~Tracker();/** @brief Initialize the tracker with a know bounding box that surrounding the target
	///     @param image The initial frame
	///     @param boundingBox The initial boundig box
	/// 
	///     @return True if initialization went succesfully, false otherwise
	///      */CV_WRAPboolinit(constMat&image,constRect2d&boundingBox);/** @brief Update the tracker, find the new most likely bounding box for the target
	///     @param image The current frame
	///     @param boundingBox The boundig box that represent the new target location, if true was returned, not
	///     modified otherwise
	/// 
	///     @return True means that target was located and false means that tracker cannot locate target in
	///     current frame. Note, that latter *does not* imply that tracker has failed, maybe target is indeed
	///     missing from the frame (say, out of sight)
	///      */CV_WRAPboolupdate(constMat&image,CV_OUTRect2d&boundingBox);/** @brief Creates a tracker by its name.
	///     @param trackerType Tracker type
	/// 
	///     The following detector types are supported:
	/// 
	///     -   "MIL" -- TrackerMIL
	///     -   "BOOSTING" -- TrackerBoosting
	///      */CV_WRAPstaticPtr<Tracker>create(constString&trackerType);virtualvoidread(constFileNode&fn)=0;virtualvoidwrite(FileStorage&fs)const=0;Ptr<TrackerModel>getModel(){returnmodel;}protected:virtualboolinitImpl(constMat&image,constRect2d&boundingBox)=0;virtualboolupdateImpl(constMat&image,Rect2d&boundingBox)=0;boolisInit;Ptr<TrackerFeatureSet>featureSet;Ptr<TrackerSampler>sampler;Ptr<TrackerModel>model;};/************************************ Specific TrackerStateEstimator Classes ************************************//** @brief TrackerStateEstimator based on Boosting
	///     */classCV_EXPORTSTrackerStateEstimatorMILBoosting:publicTrackerStateEstimator{public:/**
	///    * Implementation of the target state for TrackerStateEstimatorMILBoosting
	///    */classTrackerMILTargetState:publicTrackerTargetState{public:/**
	///      * \brief Constructor
	///      * \param position Top left corner of the bounding box
	///      * \param width Width of the bounding box
	///      * \param height Height of the bounding box
	///      * \param foreground label for target or background
	///      * \param features features extracted
	///      */TrackerMILTargetState(constPoint2f&position,intwidth,intheight,boolforeground,constMat&features);/**
	///      * \brief Destructor
	///      */~TrackerMILTargetState(){};/** @brief Set label: true for target foreground, false for background
	///     @param foreground Label for background/foreground
	///      */voidsetTargetFg(boolforeground);/** @brief Set the features extracted from TrackerFeatureSet
	///     @param features The features extracted
	///      */voidsetFeatures(constMat&features);/** @brief Get the label. Return true for target foreground, false for background
	///      */boolisTargetFg()const;/** @brief Get the features extracted
	///      */MatgetFeatures()const;private:boolisTarget;MattargetFeatures;};/** @brief Constructor
	///     @param nFeatures Number of features for each sample
	///      */TrackerStateEstimatorMILBoosting(intnFeatures=250);~TrackerStateEstimatorMILBoosting();/** @brief Set the current confidenceMap
	///     @param confidenceMap The current :cConfidenceMap
	///      */voidsetCurrentConfidenceMap(ConfidenceMap&confidenceMap);protected:Ptr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps);voidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps);private:uintmax_idx(conststd::vector<float>&v);voidprepareData(constConfidenceMap&confidenceMap,Mat&positive,Mat&negative);ClfMilBoostboostMILModel;booltrained;intnumFeatures;ConfidenceMapcurrentConfidenceMap;};/** @brief TrackerStateEstimatorAdaBoosting based on ADA-Boosting
	///  */classCV_EXPORTSTrackerStateEstimatorAdaBoosting:publicTrackerStateEstimator{public:/** @brief Implementation of the target state for TrackerAdaBoostingTargetState
	///     */classTrackerAdaBoostingTargetState:publicTrackerTargetState{public:/**
	///      * \brief Constructor
	///      * \param position Top left corner of the bounding box
	///      * \param width Width of the bounding box
	///      * \param height Height of the bounding box
	///      * \param foreground label for target or background
	///      * \param responses list of features
	///      */TrackerAdaBoostingTargetState(constPoint2f&position,intwidth,intheight,boolforeground,constMat&responses);/**
	///      * \brief Destructor
	///      */~TrackerAdaBoostingTargetState(){};/** @brief Set the features extracted from TrackerFeatureSet
	///     @param responses The features extracted
	///      */voidsetTargetResponses(constMat&responses);/** @brief Set label: true for target foreground, false for background
	///     @param foreground Label for background/foreground
	///      */voidsetTargetFg(boolforeground);/** @brief Get the features extracted
	///      */MatgetTargetResponses()const;/** @brief Get the label. Return true for target foreground, false for background
	///     */boolisTargetFg()const;private:boolisTarget;MattargetResponses;};/** @brief Constructor
	///     @param numClassifer Number of base classifiers
	///     @param initIterations Number of iterations in the initialization
	///     @param nFeatures Number of features/weak classifiers
	///     @param patchSize tracking rect
	///     @param ROI initial ROI
	///      */TrackerStateEstimatorAdaBoosting(intnumClassifer,intinitIterations,intnFeatures,SizepatchSize,constRect&ROI);/**
	///    * \brief Destructor
	///    */~TrackerStateEstimatorAdaBoosting();/** @brief Get the sampling ROI
	///      */RectgetSampleROI()const;/** @brief Set the sampling ROI
	///     @param ROI the sampling ROI
	///      */voidsetSampleROI(constRect&ROI);/** @brief Set the current confidenceMap
	///     @param confidenceMap The current :cConfidenceMap
	///      */voidsetCurrentConfidenceMap(ConfidenceMap&confidenceMap);/** @brief Get the list of the selected weak classifiers for the classification step
	///      */std::vector<int>computeSelectedWeakClassifier();/** @brief Get the list of the weak classifiers that should be replaced
	///      */std::vector<int>computeReplacedClassifier();/** @brief Get the list of the weak classifiers that replace those to be replaced
	///      */std::vector<int>computeSwappedClassifier();protected:Ptr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps);voidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps);Ptr<StrongClassifierDirectSelection>boostClassifier;private:intnumBaseClassifier;intiterationInit;intnumFeatures;booltrained;SizeinitPatchSize;RectsampleROI;std::vector<int>replacedClassifier;std::vector<int>swappedClassifier;ConfidenceMapcurrentConfidenceMap;};/**
	///  * \brief TrackerStateEstimator based on SVM
	///  */classCV_EXPORTSTrackerStateEstimatorSVM:publicTrackerStateEstimator{public:TrackerStateEstimatorSVM();~TrackerStateEstimatorSVM();protected:Ptr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps);voidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps);};/************************************ Specific TrackerSamplerAlgorithm Classes ************************************//** @brief TrackerSampler based on CSC (current state centered), used by MIL algorithm TrackerMIL
	///  */classCV_EXPORTSTrackerSamplerCSC:publicTrackerSamplerAlgorithm{public:enum{MODE_INIT_POS=1,//!< mode for init positive samplesMODE_INIT_NEG=2,//!< mode for init negative samplesMODE_TRACK_POS=3,//!< mode for update positive samplesMODE_TRACK_NEG=4,//!< mode for update negative samplesMODE_DETECT=5//!< mode for detect samples};structCV_EXPORTSParams{Params();floatinitInRad;//!< radius for gathering positive instances during initfloattrackInPosRad;//!< radius for gathering positive instances during trackingfloatsearchWinSize;//!< size of search windowintinitMaxNegNum;//!< # negative samples to use during initinttrackMaxPosNum;//!< # positive samples to use during traininginttrackMaxNegNum;//!< # negative samples to use during training};/** @brief Constructor
	///     @param parameters TrackerSamplerCSC parameters TrackerSamplerCSC::Params
	///      */TrackerSamplerCSC(constTrackerSamplerCSC::Params&parameters=TrackerSamplerCSC::Params());/** @brief Set the sampling mode of TrackerSamplerCSC
	///     @param samplingMode The sampling mode
	/// 
	///     The modes are:
	/// 
	///     -   "MODE_INIT_POS = 1" -- for the positive sampling in initialization step
	///     -   "MODE_INIT_NEG = 2" -- for the negative sampling in initialization step
	///     -   "MODE_TRACK_POS = 3" -- for the positive sampling in update step
	///     -   "MODE_TRACK_NEG = 4" -- for the negative sampling in update step
	///     -   "MODE_DETECT = 5" -- for the sampling in detection step
	///      */voidsetMode(intsamplingMode);~TrackerSamplerCSC();protected:boolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample);private:Paramsparams;intmode;RNGrng;std::vector<Mat>sampleImage(constMat&img,intx,inty,intw,inth,floatinrad,floatoutrad=0,intmaxnum=1000000);};/** @brief TrackerSampler based on CS (current state), used by algorithm TrackerBoosting
	///  */classCV_EXPORTSTrackerSamplerCS:publicTrackerSamplerAlgorithm{public:enum{MODE_POSITIVE=1,//!< mode for positive samplesMODE_NEGATIVE=2,//!< mode for negative samplesMODE_CLASSIFY=3//!< mode for classify samples};structCV_EXPORTSParams{Params();floatoverlap;//!<overlapping for the search windowsfloatsearchFactor;//!<search region parameter};/** @brief Constructor
	///     @param parameters TrackerSamplerCS parameters TrackerSamplerCS::Params
	///      */TrackerSamplerCS(constTrackerSamplerCS::Params&parameters=TrackerSamplerCS::Params());/** @brief Set the sampling mode of TrackerSamplerCS
	///     @param samplingMode The sampling mode
	/// 
	///     The modes are:
	/// 
	///     -   "MODE_POSITIVE = 1" -- for the positive sampling
	///     -   "MODE_NEGATIVE = 2" -- for the negative sampling
	///     -   "MODE_CLASSIFY = 3" -- for the sampling in classification step
	///      */voidsetMode(intsamplingMode);~TrackerSamplerCS();boolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample);RectgetROI()const;private:RectgetTrackingROI(floatsearchFactor);RectRectMultiply(constRect&rect,floatf);std::vector<Mat>patchesRegularScan(constMat&image,RecttrackingROI,SizepatchSize);voidsetCheckedROI(RectimageROI);Paramsparams;intmode;RecttrackedPatch;RectvalidROI;RectROI;};/** @brief This sampler is based on particle filtering.
	/// 
	/// In principle, it can be thought of as performing some sort of optimization (and indeed, this
	/// tracker uses opencv's optim module), where tracker seeks to find the rectangle in given frame,
	/// which is the most *"similar"* to the initial rectangle (the one, given through the constructor).
	/// 
	/// The optimization performed is stochastic and somehow resembles genetic algorithms, where on each new
	/// image received (submitted via TrackerSamplerPF::sampling()) we start with the region bounded by
	/// boundingBox, then generate several "perturbed" boxes, take the ones most similar to the original.
	/// This selection round is repeated several times. At the end, we hope that only the most promising box
	/// remaining, and these are combined to produce the subrectangle of image, which is put as a sole
	/// element in array sample.
	/// 
	/// It should be noted, that the definition of "similarity" between two rectangles is based on comparing
	/// their histograms. As experiments show, tracker is *not* very succesfull if target is assumed to
	/// strongly change its dimensions.
	///  */classCV_EXPORTSTrackerSamplerPF:publicTrackerSamplerAlgorithm{public:/** @brief This structure contains all the parameters that can be varied during the course of sampling
	///     algorithm. Below is the structure exposed, together with its members briefly explained with
	///     reference to the above discussion on algorithm's working.
	///  */structCV_EXPORTSParams{Params();intiterationNum;//!< number of selection roundsintparticlesNum;//!< number of "perturbed" boxes on each rounddoublealpha;//!< with each new round we exponentially decrease the amount of "perturbing" we allow (like in simulated annealing)//!< and this very alpha controls how fast annealing happens, ie. how fast perturbing decreasesMat_<double>std;//!< initial values for perturbing (1-by-4 array, as each rectangle is given by 4 values -- coordinates of opposite vertices,//!< hence we have 4 values to perturb)};/** @brief Constructor
	///     @param chosenRect Initial rectangle, that is supposed to contain target we'd like to track.
	///     @param parameters
	///      */TrackerSamplerPF(constMat&chosenRect,constTrackerSamplerPF::Params&parameters=TrackerSamplerPF::Params());protected:boolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample);private:Paramsparams;Ptr<MinProblemSolver>_solver;Ptr<MinProblemSolver::Function>_function;};/************************************ Specific TrackerFeature Classes ************************************//**
	///  * \brief TrackerFeature based on Feature2D
	///  */classCV_EXPORTSTrackerFeatureFeature2d:publicTrackerFeature{public:/**
	///    * \brief Constructor
	///    * \param detectorType string of FeatureDetector
	///    * \param descriptorType string of DescriptorExtractor
	///    */TrackerFeatureFeature2d(StringdetectorType,StringdescriptorType);~TrackerFeatureFeature2d();voidselection(Mat&response,intnpoints);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);private:std::vector<KeyPoint>keypoints;};/**
	///  * \brief TrackerFeature based on HOG
	///  */classCV_EXPORTSTrackerFeatureHOG:publicTrackerFeature{public:TrackerFeatureHOG();~TrackerFeatureHOG();voidselection(Mat&response,intnpoints);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);};/** @brief TrackerFeature based on HAAR features, used by TrackerMIL and many others algorithms
	/// @note HAAR features implementation is copied from apps/traincascade and modified according to MIL
	///  */classCV_EXPORTSTrackerFeatureHAAR:publicTrackerFeature{public:structCV_EXPORTSParams{Params();intnumFeatures;//!< # of rectsSizerectSize;//!< rect sizeboolisIntegral;//!< true if input images are integral, false otherwise};/** @brief Constructor
	///     @param parameters TrackerFeatureHAAR parameters TrackerFeatureHAAR::Params
	///      */TrackerFeatureHAAR(constTrackerFeatureHAAR::Params&parameters=TrackerFeatureHAAR::Params());~TrackerFeatureHAAR();/** @brief Compute the features only for the selected indices in the images collection
	///     @param selFeatures indices of selected features
	///     @param images The images
	///     @param response Collection of response for the specific TrackerFeature
	///      */boolextractSelected(conststd::vector<int>selFeatures,conststd::vector<Mat>&images,Mat&response);/** @brief Identify most effective features
	///     @param response Collection of response for the specific TrackerFeature
	///     @param npoints Max number of features
	/// 
	///     @note This method modifies the response parameter
	///      */voidselection(Mat&response,intnpoints);/** @brief Swap the feature in position source with the feature in position target
	///   @param source The source position
	///   @param target The target position
	///  */boolswapFeature(intsource,inttarget);/** @brief   Swap the feature in position id with the feature input
	///   @param id The position
	///   @param feature The feature
	///  */boolswapFeature(intid,CvHaarEvaluator::FeatureHaar&feature);/** @brief Get the feature in position id
	///     @param id The position
	///      */CvHaarEvaluator::FeatureHaar&getFeatureAt(intid);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);private:Paramsparams;Ptr<CvHaarEvaluator>featureEvaluator;};/**
	///  * \brief TrackerFeature based on LBP
	///  */classCV_EXPORTSTrackerFeatureLBP:publicTrackerFeature{public:TrackerFeatureLBP();~TrackerFeatureLBP();voidselection(Mat&response,intnpoints);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);};/************************************ Specific Tracker Classes ************************************//** @brief The MIL algorithm trains a classifier in an online manner to separate the object from the
	/// background.
	/// 
	/// Multiple Instance Learning avoids the drift problem for a robust tracking. The implementation is
	/// based on @cite MIL .
	/// 
	/// Original code can be found here <http://vision.ucsd.edu/~bbabenko/project_miltrack.shtml>
	///  */classCV_EXPORTSTrackerMIL:publicTracker{public:structCV_EXPORTSParams{Params();//parameters for samplerfloatsamplerInitInRadius;//!< radius for gathering positive instances during initintsamplerInitMaxNegNum;//!< # negative samples to use during initfloatsamplerSearchWinSize;//!< size of search windowfloatsamplerTrackInRadius;//!< radius for gathering positive instances during trackingintsamplerTrackMaxPosNum;//!< # positive samples to use during trackingintsamplerTrackMaxNegNum;//!< # negative samples to use during trackingintfeatureSetNumFeatures;//!< # featuresvoidread(constFileNode&fn);voidwrite(FileStorage&fs)const;};/** @brief Constructor
	///     @param parameters MIL parameters TrackerMIL::Params
	///      */BOILERPLATE_CODE("MIL",TrackerMIL);};/** @brief This is a real-time object tracking based on a novel on-line version of the AdaBoost algorithm.
	/// 
	/// The classifier uses the surrounding background as negative examples in update step to avoid the
	/// drifting problem. The implementation is based on @cite OLB .
	///  */classCV_EXPORTSTrackerBoosting:publicTracker{public:structCV_EXPORTSParams{Params();intnumClassifiers;//!<the number of classifiers to use in a OnlineBoosting algorithmfloatsamplerOverlap;//!<search region parameters to use in a OnlineBoosting algorithmfloatsamplerSearchFactor;//!< search region parameters to use in a OnlineBoosting algorithmintiterationInit;//!<the initial iterationsintfeatureSetNumFeatures;//!< # features/**
	///      * \brief Read parameters from file
	///      */voidread(constFileNode&fn);/**
	///      * \brief Write parameters in a file
	///      */voidwrite(FileStorage&fs)const;};/** @brief Constructor
	///     @param parameters BOOSTING parameters TrackerBoosting::Params
	///      */BOILERPLATE_CODE("BOOSTING",TrackerBoosting);};/** @brief Median Flow tracker implementation.
	/// 
	/// Implementation of a paper @cite MedianFlow .
	/// 
	/// The tracker is suitable for very smooth and predictable movements when object is visible throughout
	/// the whole sequence. It's quite and accurate for this type of problems (in particular, it was shown
	/// by authors to outperform MIL). During the implementation period the code at
	/// <http://www.aonsquared.co.uk/node/5>, the courtesy of the author Arthur Amarra, was used for the
	/// reference purpose.
	///  */classCV_EXPORTSTrackerMedianFlow:publicTracker{public:structCV_EXPORTSParams{Params();intpointsInGrid;//!<square root of number of keypoints used; increase it to trade//!<accurateness for speed; default value is sensible and recommendedvoidread(constFileNode&/*fn*/);voidwrite(FileStorage&/*fs*/)const;};/** @brief Constructor
	///     @param parameters Median Flow parameters TrackerMedianFlow::Params
	///     */BOILERPLATE_CODE("MEDIANFLOW",TrackerMedianFlow);};/** @brief TLD is a novel tracking framework that explicitly decomposes the long-term tracking task into
	/// tracking, learning and detection.
	/// 
	/// The tracker follows the object from frame to frame. The detector localizes all appearances that
	/// have been observed so far and corrects the tracker if necessary. The learning estimates detector’s
	/// errors and updates it to avoid these errors in the future. The implementation is based on @cite TLD .
	/// 
	/// The Median Flow algorithm (see cv::TrackerMedianFlow) was chosen as a tracking component in this
	/// implementation, following authors. Tracker is supposed to be able to handle rapid motions, partial
	/// occlusions, object absence etc.
	///  */classCV_EXPORTSTrackerTLD:publicTracker{public:structCV_EXPORTSParams{Params();voidread(constFileNode&/*fn*/);voidwrite(FileStorage&/*fs*/)const;};/** @brief Constructor
	///     @param parameters TLD parameters TrackerTLD::Params
	///      */BOILERPLATE_CODE("TLD",TrackerTLD);};/** @brief KCF is a novel tracking framework that utilizes properties of circulant matrix to enhance the processing speed.
	///  * This tracking method is an implementation of @cite KCF_ECCV which is extended to KFC with color-names features (@cite KCF_CN).
	///  * The original paper of KCF is available at <http://home.isr.uc.pt/~henriques/circulant/index.html>
	///  * as well as the matlab implementation. For more information about KCF with color-names features, please refer to
	///  * <http://www.cvl.isy.liu.se/research/objrec/visualtracking/colvistrack/index.html>.
	///  */classCV_EXPORTSTrackerKCF:publicTracker{public:/**
	///   * \brief Feature type to be used in the tracking grayscale, colornames, compressed color-names
	///   * The modes available now:
	///   -   "GRAY" -- Use grayscale values as the feature
	///   -   "CN" -- Color-names feature
	///   */enumMODE{GRAY=(1u<<0),CN=(1u<<1),CUSTOM=(1u<<2)};structCV_EXPORTSParams{/**
	///     * \brief Constructor
	///     */Params();/**
	///     * \brief Read parameters from file, currently unused
	///     */voidread(constFileNode&/*fn*/);/**
	///     * \brief Read parameters from file, currently unused
	///     */voidwrite(FileStorage&/*fs*/)const;doublesigma;//!<  gaussian kernel bandwidthdoublelambda;//!<  regularizationdoubleinterp_factor;//!<  linear interpolation factor for adaptationdoubleoutput_sigma_factor;//!<  spatial bandwidth (proportional to target)doublepca_learning_rate;//!<  compression learning rateboolresize;//!<  activate the resize feature to improve the processing speedboolsplit_coeff;//!<  split the training coefficients into two matricesboolwrap_kernel;//!<  wrap around the kernel valuesboolcompress_feature;//!<  activate the pca method to compress the featuresintmax_patch_size;//!<  threshold for the ROI sizeintcompressed_size;//!<  feature size after compressionunsignedintdesc_pca;//!<  compressed descriptors of TrackerKCF::MODEunsignedintdesc_npca;//!<  non-compressed descriptors of TrackerKCF::MODE};virtualvoidsetFeatureExtractor(void(*)(constMat,constRect,Mat&),boolpca_func=false);/** @brief Constructor
	///   @param parameters KCF parameters TrackerKCF::Params
	///   */BOILERPLATE_CODE("KCF",TrackerKCF)
	pub fn create_tracker(parameters: &crate::tracking::TrackerKCF_Params) -> Result<core::Ptr::<dyn crate::tracking::TrackerKCF>> {
		unsafe { sys::cv_TrackerKCF_createTracker_const_ParamsX(parameters.as_raw_TrackerKCF_Params()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::tracking::TrackerKCF>::opencv_from_extern(r) } )
	}
	
}
pub trait TrackerKCF_ParamsTrait {
	fn as_raw_TrackerKCF_Params(&self) -> *const c_void;
	fn as_raw_mut_TrackerKCF_Params(&mut self) -> *mut c_void;

	/// gaussian kernel bandwidth
	fn sigma(&self) -> f64 {
		unsafe { sys::cv_TrackerKCF_Params_getPropSigma_const(self.as_raw_TrackerKCF_Params()) }.into_result().expect("Infallible function failed: sigma")
	}
	
	/// gaussian kernel bandwidth
	fn set_sigma(&mut self, val: f64) -> () {
		unsafe { sys::cv_TrackerKCF_Params_setPropSigma_double(self.as_raw_mut_TrackerKCF_Params(), val) }.into_result().expect("Infallible function failed: set_sigma")
	}
	
	/// regularization
	fn lambda(&self) -> f64 {
		unsafe { sys::cv_TrackerKCF_Params_getPropLambda_const(self.as_raw_TrackerKCF_Params()) }.into_result().expect("Infallible function failed: lambda")
	}
	
	/// regularization
	fn set_lambda(&mut self, val: f64) -> () {
		unsafe { sys::cv_TrackerKCF_Params_setPropLambda_double(self.as_raw_mut_TrackerKCF_Params(), val) }.into_result().expect("Infallible function failed: set_lambda")
	}
	
	/// linear interpolation factor for adaptation
	fn interp_factor(&self) -> f64 {
		unsafe { sys::cv_TrackerKCF_Params_getPropInterp_factor_const(self.as_raw_TrackerKCF_Params()) }.into_result().expect("Infallible function failed: interp_factor")
	}
	
	/// linear interpolation factor for adaptation
	fn set_interp_factor(&mut self, val: f64) -> () {
		unsafe { sys::cv_TrackerKCF_Params_setPropInterp_factor_double(self.as_raw_mut_TrackerKCF_Params(), val) }.into_result().expect("Infallible function failed: set_interp_factor")
	}
	
	/// spatial bandwidth (proportional to target)
	fn output_sigma_factor(&self) -> f64 {
		unsafe { sys::cv_TrackerKCF_Params_getPropOutput_sigma_factor_const(self.as_raw_TrackerKCF_Params()) }.into_result().expect("Infallible function failed: output_sigma_factor")
	}
	
	/// spatial bandwidth (proportional to target)
	fn set_output_sigma_factor(&mut self, val: f64) -> () {
		unsafe { sys::cv_TrackerKCF_Params_setPropOutput_sigma_factor_double(self.as_raw_mut_TrackerKCF_Params(), val) }.into_result().expect("Infallible function failed: set_output_sigma_factor")
	}
	
	/// compression learning rate
	fn pca_learning_rate(&self) -> f64 {
		unsafe { sys::cv_TrackerKCF_Params_getPropPca_learning_rate_const(self.as_raw_TrackerKCF_Params()) }.into_result().expect("Infallible function failed: pca_learning_rate")
	}
	
	/// compression learning rate
	fn set_pca_learning_rate(&mut self, val: f64) -> () {
		unsafe { sys::cv_TrackerKCF_Params_setPropPca_learning_rate_double(self.as_raw_mut_TrackerKCF_Params(), val) }.into_result().expect("Infallible function failed: set_pca_learning_rate")
	}
	
	/// activate the resize feature to improve the processing speed
	fn resize(&self) -> bool {
		unsafe { sys::cv_TrackerKCF_Params_getPropResize_const(self.as_raw_TrackerKCF_Params()) }.into_result().expect("Infallible function failed: resize")
	}
	
	/// activate the resize feature to improve the processing speed
	fn set_resize(&mut self, val: bool) -> () {
		unsafe { sys::cv_TrackerKCF_Params_setPropResize_bool(self.as_raw_mut_TrackerKCF_Params(), val) }.into_result().expect("Infallible function failed: set_resize")
	}
	
	/// split the training coefficients into two matrices
	fn split_coeff(&self) -> bool {
		unsafe { sys::cv_TrackerKCF_Params_getPropSplit_coeff_const(self.as_raw_TrackerKCF_Params()) }.into_result().expect("Infallible function failed: split_coeff")
	}
	
	/// split the training coefficients into two matrices
	fn set_split_coeff(&mut self, val: bool) -> () {
		unsafe { sys::cv_TrackerKCF_Params_setPropSplit_coeff_bool(self.as_raw_mut_TrackerKCF_Params(), val) }.into_result().expect("Infallible function failed: set_split_coeff")
	}
	
	/// wrap around the kernel values
	fn wrap_kernel(&self) -> bool {
		unsafe { sys::cv_TrackerKCF_Params_getPropWrap_kernel_const(self.as_raw_TrackerKCF_Params()) }.into_result().expect("Infallible function failed: wrap_kernel")
	}
	
	/// wrap around the kernel values
	fn set_wrap_kernel(&mut self, val: bool) -> () {
		unsafe { sys::cv_TrackerKCF_Params_setPropWrap_kernel_bool(self.as_raw_mut_TrackerKCF_Params(), val) }.into_result().expect("Infallible function failed: set_wrap_kernel")
	}
	
	/// activate the pca method to compress the features
	fn compress_feature(&self) -> bool {
		unsafe { sys::cv_TrackerKCF_Params_getPropCompress_feature_const(self.as_raw_TrackerKCF_Params()) }.into_result().expect("Infallible function failed: compress_feature")
	}
	
	/// activate the pca method to compress the features
	fn set_compress_feature(&mut self, val: bool) -> () {
		unsafe { sys::cv_TrackerKCF_Params_setPropCompress_feature_bool(self.as_raw_mut_TrackerKCF_Params(), val) }.into_result().expect("Infallible function failed: set_compress_feature")
	}
	
	/// threshold for the ROI size
	fn max_patch_size(&self) -> i32 {
		unsafe { sys::cv_TrackerKCF_Params_getPropMax_patch_size_const(self.as_raw_TrackerKCF_Params()) }.into_result().expect("Infallible function failed: max_patch_size")
	}
	
	/// threshold for the ROI size
	fn set_max_patch_size(&mut self, val: i32) -> () {
		unsafe { sys::cv_TrackerKCF_Params_setPropMax_patch_size_int(self.as_raw_mut_TrackerKCF_Params(), val) }.into_result().expect("Infallible function failed: set_max_patch_size")
	}
	
	/// feature size after compression
	fn compressed_size(&self) -> i32 {
		unsafe { sys::cv_TrackerKCF_Params_getPropCompressed_size_const(self.as_raw_TrackerKCF_Params()) }.into_result().expect("Infallible function failed: compressed_size")
	}
	
	/// feature size after compression
	fn set_compressed_size(&mut self, val: i32) -> () {
		unsafe { sys::cv_TrackerKCF_Params_setPropCompressed_size_int(self.as_raw_mut_TrackerKCF_Params(), val) }.into_result().expect("Infallible function failed: set_compressed_size")
	}
	
	/// compressed descriptors of TrackerKCF::MODE
	fn desc_pca(&self) -> u32 {
		unsafe { sys::cv_TrackerKCF_Params_getPropDesc_pca_const(self.as_raw_TrackerKCF_Params()) }.into_result().expect("Infallible function failed: desc_pca")
	}
	
	/// compressed descriptors of TrackerKCF::MODE
	fn set_desc_pca(&mut self, val: u32) -> () {
		unsafe { sys::cv_TrackerKCF_Params_setPropDesc_pca_unsigned_int(self.as_raw_mut_TrackerKCF_Params(), val) }.into_result().expect("Infallible function failed: set_desc_pca")
	}
	
	/// non-compressed descriptors of TrackerKCF::MODE
	fn desc_npca(&self) -> u32 {
		unsafe { sys::cv_TrackerKCF_Params_getPropDesc_npca_const(self.as_raw_TrackerKCF_Params()) }.into_result().expect("Infallible function failed: desc_npca")
	}
	
	/// non-compressed descriptors of TrackerKCF::MODE
	fn set_desc_npca(&mut self, val: u32) -> () {
		unsafe { sys::cv_TrackerKCF_Params_setPropDesc_npca_unsigned_int(self.as_raw_mut_TrackerKCF_Params(), val) }.into_result().expect("Infallible function failed: set_desc_npca")
	}
	
	/// \brief Read parameters from file, currently unused
	fn read(&mut self, unnamed: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_TrackerKCF_Params_read_const_FileNodeX(self.as_raw_mut_TrackerKCF_Params(), unnamed.as_raw_FileNode()) }.into_result()
	}
	
	/// \brief Read parameters from file, currently unused
	fn write(&self, unnamed: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_TrackerKCF_Params_write_const_FileStorageX(self.as_raw_TrackerKCF_Params(), unnamed.as_raw_mut_FileStorage()) }.into_result()
	}
	
}

pub struct TrackerKCF_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerKCF_Params }

impl Drop for TrackerKCF_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerKCF_Params_delete(instance: *mut c_void); }
		unsafe { cv_TrackerKCF_Params_delete(self.as_raw_mut_TrackerKCF_Params()) };
	}
}

impl TrackerKCF_Params {
	pub fn as_raw_TrackerKCF_Params(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerKCF_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerKCF_Params {}

impl crate::tracking::TrackerKCF_ParamsTrait for TrackerKCF_Params {
	fn as_raw_TrackerKCF_Params(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerKCF_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerKCF_Params {
	/// \brief Constructor
	pub fn default() -> Result<crate::tracking::TrackerKCF_Params> {
		unsafe { sys::cv_TrackerKCF_Params_Params() }.into_result().map(|r| unsafe { crate::tracking::TrackerKCF_Params::opencv_from_extern(r) } )
	}
	
}

/// The MIL algorithm trains a classifier in an online manner to separate the object from the
/// background.
/// 
/// Multiple Instance Learning avoids the drift problem for a robust tracking. The implementation is
/// based on [MIL](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_MIL) .
/// 
/// Original code can be found here <http://vision.ucsd.edu/~bbabenko/project_miltrack.shtml>
pub trait TrackerMIL: crate::tracking::Tracker {
	fn as_raw_TrackerMIL(&self) -> *const c_void;
	fn as_raw_mut_TrackerMIL(&mut self) -> *mut c_void;

}

impl dyn TrackerMIL + '_ {
	/// ## C++ default parameters
	/// * parameters: classname::Params());virtual~classname(){};/*
	///  * Partially based on:
	///  * ====================================================================================================================
	///  *   - [AAM] S. Salti, A. Cavallaro, L. Di Stefano, Adaptive Appearance Modeling for Video Tracking: Survey and Evaluation
	///  *  - [AMVOT] X. Li, W. Hu, C. Shen, Z. Zhang, A. Dick, A. van den Hengel, A Survey of Appearance Models in Visual Object Tracking
	///  *
	///  * This Tracking API has been designed with PlantUML. If you modify this API please change UML files under modules/tracking/doc/uml
	///  *
	///  */namespacecv{//! @addtogroup tracking//! @{/************************************ TrackerFeature Base Classes ************************************//** @brief Abstract base class for TrackerFeature that represents the feature.
	///  */classCV_EXPORTSTrackerFeature{public:virtual~TrackerFeature();/** @brief Compute the features in the images collection
	///     @param images The images
	///     @param response The output response
	///      */voidcompute(conststd::vector<Mat>&images,Mat&response);/** @brief Create TrackerFeature by tracker feature type
	///     @param trackerFeatureType The TrackerFeature name
	/// 
	///     The modes available now:
	/// 
	///     -   "HAAR" -- Haar Feature-based
	/// 
	///     The modes that will be available soon:
	/// 
	///     -   "HOG" -- Histogram of Oriented Gradients features
	///     -   "LBP" -- Local Binary Pattern features
	///     -   "FEATURE2D" -- All types of Feature2D
	///      */staticPtr<TrackerFeature>create(constString&trackerFeatureType);/** @brief Identify most effective features
	///     @param response Collection of response for the specific TrackerFeature
	///     @param npoints Max number of features
	/// 
	///     @note This method modifies the response parameter
	///      */virtualvoidselection(Mat&response,intnpoints)=0;/** @brief Get the name of the specific TrackerFeature
	///      */StringgetClassName()const;protected:virtualboolcomputeImpl(conststd::vector<Mat>&images,Mat&response)=0;StringclassName;};/** @brief Class that manages the extraction and selection of features
	/// 
	/// @cite AAM Feature Extraction and Feature Set Refinement (Feature Processing and Feature Selection).
	/// See table I and section III C @cite AMVOT Appearance modelling -\> Visual representation (Table II,
	/// section 3.1 - 3.2)
	/// 
	/// TrackerFeatureSet is an aggregation of TrackerFeature
	/// 
	/// @sa
	///    TrackerFeature
	/// 
	///  */classCV_EXPORTSTrackerFeatureSet{public:TrackerFeatureSet();~TrackerFeatureSet();/** @brief Extract features from the images collection
	///     @param images The input images
	///      */voidextraction(conststd::vector<Mat>&images);/** @brief Identify most effective features for all feature types (optional)
	///      */voidselection();/** @brief Remove outliers for all feature types (optional)
	///      */voidremoveOutliers();/** @brief Add TrackerFeature in the collection. Return true if TrackerFeature is added, false otherwise
	///     @param trackerFeatureType The TrackerFeature name
	/// 
	///     The modes available now:
	/// 
	///     -   "HAAR" -- Haar Feature-based
	/// 
	///     The modes that will be available soon:
	/// 
	///     -   "HOG" -- Histogram of Oriented Gradients features
	///     -   "LBP" -- Local Binary Pattern features
	///     -   "FEATURE2D" -- All types of Feature2D
	/// 
	///     Example TrackerFeatureSet::addTrackerFeature : :
	///     @code
	///         //sample usage:
	/// 
	///         Ptr<TrackerFeature> trackerFeature = new TrackerFeatureHAAR( HAARparameters );
	///         featureSet->addTrackerFeature( trackerFeature );
	/// 
	///         //or add CSC sampler with default parameters
	///         //featureSet->addTrackerFeature( "HAAR" );
	///     @endcode
	///     @note If you use the second method, you must initialize the TrackerFeature
	///      */booladdTrackerFeature(StringtrackerFeatureType);/** @overload
	///     @param feature The TrackerFeature class
	///     */booladdTrackerFeature(Ptr<TrackerFeature>&feature);/** @brief Get the TrackerFeature collection (TrackerFeature name, TrackerFeature pointer)
	///      */conststd::vector<std::pair<String,Ptr<TrackerFeature>>>&getTrackerFeature()const;/** @brief Get the responses
	/// 
	///     @note Be sure to call extraction before getResponses Example TrackerFeatureSet::getResponses : :
	///      */conststd::vector<Mat>&getResponses()const;private:voidclearResponses();boolblockAddTrackerFeature;std::vector<std::pair<String,Ptr<TrackerFeature>>>features;//list of featuresstd::vector<Mat>responses;//list of response after compute};/************************************ TrackerSampler Base Classes ************************************//** @brief Abstract base class for TrackerSamplerAlgorithm that represents the algorithm for the specific
	/// sampler.
	///  */classCV_EXPORTSTrackerSamplerAlgorithm{public:/**
	///    * \brief Destructor
	///    */virtual~TrackerSamplerAlgorithm();/** @brief Create TrackerSamplerAlgorithm by tracker sampler type.
	///     @param trackerSamplerType The trackerSamplerType name
	/// 
	///     The modes available now:
	/// 
	///     -   "CSC" -- Current State Center
	///     -   "CS" -- Current State
	///      */staticPtr<TrackerSamplerAlgorithm>create(constString&trackerSamplerType);/** @brief Computes the regions starting from a position in an image.
	/// 
	///     Return true if samples are computed, false otherwise
	/// 
	///     @param image The current frame
	///     @param boundingBox The bounding box from which regions can be calculated
	/// 
	///     @param sample The computed samples @cite AAM Fig. 1 variable Sk
	///      */boolsampling(constMat&image,RectboundingBox,std::vector<Mat>&sample);/** @brief Get the name of the specific TrackerSamplerAlgorithm
	///     */StringgetClassName()const;protected:StringclassName;virtualboolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample)=0;};/**
	///  * \brief Class that manages the sampler in order to select regions for the update the model of the tracker
	///  * [AAM] Sampling e Labeling. See table I and section III B
	///  *//** @brief Class that manages the sampler in order to select regions for the update the model of the tracker
	/// 
	/// @cite AAM Sampling e Labeling. See table I and section III B
	/// 
	/// TrackerSampler is an aggregation of TrackerSamplerAlgorithm
	/// @sa
	///    TrackerSamplerAlgorithm
	///  */classCV_EXPORTSTrackerSampler{public:/**
	///    * \brief Constructor
	///    */TrackerSampler();/**
	///    * \brief Destructor
	///    */~TrackerSampler();/** @brief Computes the regions starting from a position in an image
	///     @param image The current frame
	///     @param boundingBox The bounding box from which regions can be calculated
	///      */voidsampling(constMat&image,RectboundingBox);/** @brief Return the collection of the TrackerSamplerAlgorithm
	///     */conststd::vector<std::pair<String,Ptr<TrackerSamplerAlgorithm>>>&getSamplers()const;/** @brief Return the samples from all TrackerSamplerAlgorithm, @cite AAM Fig. 1 variable Sk
	///     */conststd::vector<Mat>&getSamples()const;/** @brief Add TrackerSamplerAlgorithm in the collection. Return true if sampler is added, false otherwise
	///     @param trackerSamplerAlgorithmType The TrackerSamplerAlgorithm name
	/// 
	///     The modes available now:
	///     -   "CSC" -- Current State Center
	///     -   "CS" -- Current State
	///     -   "PF" -- Particle Filtering
	/// 
	///     Example TrackerSamplerAlgorithm::addTrackerSamplerAlgorithm : :
	///     @code
	///          TrackerSamplerCSC::Params CSCparameters;
	///          Ptr<TrackerSamplerAlgorithm> CSCSampler = new TrackerSamplerCSC( CSCparameters );
	/// 
	///          if( !sampler->addTrackerSamplerAlgorithm( CSCSampler ) )
	///            return false;
	/// 
	///          //or add CSC sampler with default parameters
	///          //sampler->addTrackerSamplerAlgorithm( "CSC" );
	///     @endcode
	///     @note If you use the second method, you must initialize the TrackerSamplerAlgorithm
	///      */booladdTrackerSamplerAlgorithm(StringtrackerSamplerAlgorithmType);/** @overload
	///     @param sampler The TrackerSamplerAlgorithm
	///     */booladdTrackerSamplerAlgorithm(Ptr<TrackerSamplerAlgorithm>&sampler);private:std::vector<std::pair<String,Ptr<TrackerSamplerAlgorithm>>>samplers;std::vector<Mat>samples;boolblockAddTrackerSampler;voidclearSamples();};/************************************ TrackerModel Base Classes ************************************//** @brief Abstract base class for TrackerTargetState that represents a possible state of the target.
	/// 
	/// See @cite AAM \f$\hat{x}^{i}_{k}\f$ all the states candidates.
	/// 
	/// Inherits this class with your Target state, In own implementation you can add scale variation,
	/// width, height, orientation, etc.
	///  */classCV_EXPORTSTrackerTargetState{public:virtual~TrackerTargetState(){};/**
	///    * \brief Get the position
	///    * \return The position
	///    */Point2fgetTargetPosition()const;/**
	///    * \brief Set the position
	///    * \param position The position
	///    */voidsetTargetPosition(constPoint2f&position);/**
	///    * \brief Get the width of the target
	///    * \return The width of the target
	///    */intgetTargetWidth()const;/**
	///    * \brief Set the width of the target
	///    * \param width The width of the target
	///    */voidsetTargetWidth(intwidth);/**
	///    * \brief Get the height of the target
	///    * \return The height of the target
	///    */intgetTargetHeight()const;/**
	///    * \brief Set the height of the target
	///    * \param height The height of the target
	///    */voidsetTargetHeight(intheight);protected:Point2ftargetPosition;inttargetWidth;inttargetHeight;};/** @brief Represents the model of the target at frame \f$k\f$ (all states and scores)
	/// 
	/// See @cite AAM The set of the pair \f$\langle \hat{x}^{i}_{k}, C^{i}_{k} \rangle\f$
	/// @sa TrackerTargetState
	///  */typedefstd::vector<std::pair<Ptr<TrackerTargetState>,float>>ConfidenceMap;/** @brief Represents the estimate states for all frames
	/// 
	/// @cite AAM \f$x_{k}\f$ is the trajectory of the target up to time \f$k\f$
	/// 
	/// @sa TrackerTargetState
	///  */typedefstd::vector<Ptr<TrackerTargetState>>Trajectory;/** @brief Abstract base class for TrackerStateEstimator that estimates the most likely target state.
	/// 
	/// See @cite AAM State estimator
	/// 
	/// See @cite AMVOT Statistical modeling (Fig. 3), Table III (generative) - IV (discriminative) - V (hybrid)
	///  */classCV_EXPORTSTrackerStateEstimator{public:virtual~TrackerStateEstimator();/** @brief Estimate the most likely target state, return the estimated state
	///     @param confidenceMaps The overall appearance model as a list of :cConfidenceMap
	///      */Ptr<TrackerTargetState>estimate(conststd::vector<ConfidenceMap>&confidenceMaps);/** @brief Update the ConfidenceMap with the scores
	///     @param confidenceMaps The overall appearance model as a list of :cConfidenceMap
	///      */voidupdate(std::vector<ConfidenceMap>&confidenceMaps);/** @brief Create TrackerStateEstimator by tracker state estimator type
	///     @param trackeStateEstimatorType The TrackerStateEstimator name
	/// 
	///     The modes available now:
	/// 
	///     -   "BOOSTING" -- Boosting-based discriminative appearance models. See @cite AMVOT section 4.4
	/// 
	///     The modes available soon:
	/// 
	///     -   "SVM" -- SVM-based discriminative appearance models. See @cite AMVOT section 4.5
	///      */staticPtr<TrackerStateEstimator>create(constString&trackeStateEstimatorType);/** @brief Get the name of the specific TrackerStateEstimator
	///      */StringgetClassName()const;protected:virtualPtr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps)=0;virtualvoidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps)=0;StringclassName;};/** @brief Abstract class that represents the model of the target. It must be instantiated by specialized
	/// tracker
	/// 
	/// See @cite AAM Ak
	/// 
	/// Inherits this with your TrackerModel
	///  */classCV_EXPORTSTrackerModel{public:/**
	///    * \brief Constructor
	///    */TrackerModel();/**
	///    * \brief Destructor
	///    */virtual~TrackerModel();/** @brief Set TrackerEstimator, return true if the tracker state estimator is added, false otherwise
	///     @param trackerStateEstimator The TrackerStateEstimator
	///     @note You can add only one TrackerStateEstimator
	///      */boolsetTrackerStateEstimator(Ptr<TrackerStateEstimator>trackerStateEstimator);/** @brief Estimate the most likely target location
	/// 
	///     @cite AAM ME, Model Estimation table I
	///     @param responses Features extracted from TrackerFeatureSet
	///      */voidmodelEstimation(conststd::vector<Mat>&responses);/** @brief Update the model
	/// 
	///     @cite AAM MU, Model Update table I
	///      */voidmodelUpdate();/** @brief Run the TrackerStateEstimator, return true if is possible to estimate a new state, false otherwise
	///     */boolrunStateEstimator();/** @brief Set the current TrackerTargetState in the Trajectory
	///     @param lastTargetState The current TrackerTargetState
	///      */voidsetLastTargetState(constPtr<TrackerTargetState>&lastTargetState);/** @brief Get the last TrackerTargetState from Trajectory
	///     */Ptr<TrackerTargetState>getLastTargetState()const;/** @brief Get the list of the ConfidenceMap
	///     */conststd::vector<ConfidenceMap>&getConfidenceMaps()const;/** @brief Get the last ConfidenceMap for the current frame
	///      */constConfidenceMap&getLastConfidenceMap()const;/** @brief Get the TrackerStateEstimator
	///     */Ptr<TrackerStateEstimator>getTrackerStateEstimator()const;private:voidclearCurrentConfidenceMap();protected:std::vector<ConfidenceMap>confidenceMaps;Ptr<TrackerStateEstimator>stateEstimator;ConfidenceMapcurrentConfidenceMap;Trajectorytrajectory;intmaxCMLength;virtualvoidmodelEstimationImpl(conststd::vector<Mat>&responses)=0;virtualvoidmodelUpdateImpl()=0;};/************************************ Tracker Base Class ************************************//** @brief Base abstract class for the long-term tracker:
	///  */classCV_EXPORTS_WTracker:publicvirtualAlgorithm{public:virtual~Tracker();/** @brief Initialize the tracker with a know bounding box that surrounding the target
	///     @param image The initial frame
	///     @param boundingBox The initial boundig box
	/// 
	///     @return True if initialization went succesfully, false otherwise
	///      */CV_WRAPboolinit(constMat&image,constRect2d&boundingBox);/** @brief Update the tracker, find the new most likely bounding box for the target
	///     @param image The current frame
	///     @param boundingBox The boundig box that represent the new target location, if true was returned, not
	///     modified otherwise
	/// 
	///     @return True means that target was located and false means that tracker cannot locate target in
	///     current frame. Note, that latter *does not* imply that tracker has failed, maybe target is indeed
	///     missing from the frame (say, out of sight)
	///      */CV_WRAPboolupdate(constMat&image,CV_OUTRect2d&boundingBox);/** @brief Creates a tracker by its name.
	///     @param trackerType Tracker type
	/// 
	///     The following detector types are supported:
	/// 
	///     -   "MIL" -- TrackerMIL
	///     -   "BOOSTING" -- TrackerBoosting
	///      */CV_WRAPstaticPtr<Tracker>create(constString&trackerType);virtualvoidread(constFileNode&fn)=0;virtualvoidwrite(FileStorage&fs)const=0;Ptr<TrackerModel>getModel(){returnmodel;}protected:virtualboolinitImpl(constMat&image,constRect2d&boundingBox)=0;virtualboolupdateImpl(constMat&image,Rect2d&boundingBox)=0;boolisInit;Ptr<TrackerFeatureSet>featureSet;Ptr<TrackerSampler>sampler;Ptr<TrackerModel>model;};/************************************ Specific TrackerStateEstimator Classes ************************************//** @brief TrackerStateEstimator based on Boosting
	///     */classCV_EXPORTSTrackerStateEstimatorMILBoosting:publicTrackerStateEstimator{public:/**
	///    * Implementation of the target state for TrackerStateEstimatorMILBoosting
	///    */classTrackerMILTargetState:publicTrackerTargetState{public:/**
	///      * \brief Constructor
	///      * \param position Top left corner of the bounding box
	///      * \param width Width of the bounding box
	///      * \param height Height of the bounding box
	///      * \param foreground label for target or background
	///      * \param features features extracted
	///      */TrackerMILTargetState(constPoint2f&position,intwidth,intheight,boolforeground,constMat&features);/**
	///      * \brief Destructor
	///      */~TrackerMILTargetState(){};/** @brief Set label: true for target foreground, false for background
	///     @param foreground Label for background/foreground
	///      */voidsetTargetFg(boolforeground);/** @brief Set the features extracted from TrackerFeatureSet
	///     @param features The features extracted
	///      */voidsetFeatures(constMat&features);/** @brief Get the label. Return true for target foreground, false for background
	///      */boolisTargetFg()const;/** @brief Get the features extracted
	///      */MatgetFeatures()const;private:boolisTarget;MattargetFeatures;};/** @brief Constructor
	///     @param nFeatures Number of features for each sample
	///      */TrackerStateEstimatorMILBoosting(intnFeatures=250);~TrackerStateEstimatorMILBoosting();/** @brief Set the current confidenceMap
	///     @param confidenceMap The current :cConfidenceMap
	///      */voidsetCurrentConfidenceMap(ConfidenceMap&confidenceMap);protected:Ptr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps);voidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps);private:uintmax_idx(conststd::vector<float>&v);voidprepareData(constConfidenceMap&confidenceMap,Mat&positive,Mat&negative);ClfMilBoostboostMILModel;booltrained;intnumFeatures;ConfidenceMapcurrentConfidenceMap;};/** @brief TrackerStateEstimatorAdaBoosting based on ADA-Boosting
	///  */classCV_EXPORTSTrackerStateEstimatorAdaBoosting:publicTrackerStateEstimator{public:/** @brief Implementation of the target state for TrackerAdaBoostingTargetState
	///     */classTrackerAdaBoostingTargetState:publicTrackerTargetState{public:/**
	///      * \brief Constructor
	///      * \param position Top left corner of the bounding box
	///      * \param width Width of the bounding box
	///      * \param height Height of the bounding box
	///      * \param foreground label for target or background
	///      * \param responses list of features
	///      */TrackerAdaBoostingTargetState(constPoint2f&position,intwidth,intheight,boolforeground,constMat&responses);/**
	///      * \brief Destructor
	///      */~TrackerAdaBoostingTargetState(){};/** @brief Set the features extracted from TrackerFeatureSet
	///     @param responses The features extracted
	///      */voidsetTargetResponses(constMat&responses);/** @brief Set label: true for target foreground, false for background
	///     @param foreground Label for background/foreground
	///      */voidsetTargetFg(boolforeground);/** @brief Get the features extracted
	///      */MatgetTargetResponses()const;/** @brief Get the label. Return true for target foreground, false for background
	///     */boolisTargetFg()const;private:boolisTarget;MattargetResponses;};/** @brief Constructor
	///     @param numClassifer Number of base classifiers
	///     @param initIterations Number of iterations in the initialization
	///     @param nFeatures Number of features/weak classifiers
	///     @param patchSize tracking rect
	///     @param ROI initial ROI
	///      */TrackerStateEstimatorAdaBoosting(intnumClassifer,intinitIterations,intnFeatures,SizepatchSize,constRect&ROI);/**
	///    * \brief Destructor
	///    */~TrackerStateEstimatorAdaBoosting();/** @brief Get the sampling ROI
	///      */RectgetSampleROI()const;/** @brief Set the sampling ROI
	///     @param ROI the sampling ROI
	///      */voidsetSampleROI(constRect&ROI);/** @brief Set the current confidenceMap
	///     @param confidenceMap The current :cConfidenceMap
	///      */voidsetCurrentConfidenceMap(ConfidenceMap&confidenceMap);/** @brief Get the list of the selected weak classifiers for the classification step
	///      */std::vector<int>computeSelectedWeakClassifier();/** @brief Get the list of the weak classifiers that should be replaced
	///      */std::vector<int>computeReplacedClassifier();/** @brief Get the list of the weak classifiers that replace those to be replaced
	///      */std::vector<int>computeSwappedClassifier();protected:Ptr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps);voidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps);Ptr<StrongClassifierDirectSelection>boostClassifier;private:intnumBaseClassifier;intiterationInit;intnumFeatures;booltrained;SizeinitPatchSize;RectsampleROI;std::vector<int>replacedClassifier;std::vector<int>swappedClassifier;ConfidenceMapcurrentConfidenceMap;};/**
	///  * \brief TrackerStateEstimator based on SVM
	///  */classCV_EXPORTSTrackerStateEstimatorSVM:publicTrackerStateEstimator{public:TrackerStateEstimatorSVM();~TrackerStateEstimatorSVM();protected:Ptr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps);voidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps);};/************************************ Specific TrackerSamplerAlgorithm Classes ************************************//** @brief TrackerSampler based on CSC (current state centered), used by MIL algorithm TrackerMIL
	///  */classCV_EXPORTSTrackerSamplerCSC:publicTrackerSamplerAlgorithm{public:enum{MODE_INIT_POS=1,//!< mode for init positive samplesMODE_INIT_NEG=2,//!< mode for init negative samplesMODE_TRACK_POS=3,//!< mode for update positive samplesMODE_TRACK_NEG=4,//!< mode for update negative samplesMODE_DETECT=5//!< mode for detect samples};structCV_EXPORTSParams{Params();floatinitInRad;//!< radius for gathering positive instances during initfloattrackInPosRad;//!< radius for gathering positive instances during trackingfloatsearchWinSize;//!< size of search windowintinitMaxNegNum;//!< # negative samples to use during initinttrackMaxPosNum;//!< # positive samples to use during traininginttrackMaxNegNum;//!< # negative samples to use during training};/** @brief Constructor
	///     @param parameters TrackerSamplerCSC parameters TrackerSamplerCSC::Params
	///      */TrackerSamplerCSC(constTrackerSamplerCSC::Params&parameters=TrackerSamplerCSC::Params());/** @brief Set the sampling mode of TrackerSamplerCSC
	///     @param samplingMode The sampling mode
	/// 
	///     The modes are:
	/// 
	///     -   "MODE_INIT_POS = 1" -- for the positive sampling in initialization step
	///     -   "MODE_INIT_NEG = 2" -- for the negative sampling in initialization step
	///     -   "MODE_TRACK_POS = 3" -- for the positive sampling in update step
	///     -   "MODE_TRACK_NEG = 4" -- for the negative sampling in update step
	///     -   "MODE_DETECT = 5" -- for the sampling in detection step
	///      */voidsetMode(intsamplingMode);~TrackerSamplerCSC();protected:boolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample);private:Paramsparams;intmode;RNGrng;std::vector<Mat>sampleImage(constMat&img,intx,inty,intw,inth,floatinrad,floatoutrad=0,intmaxnum=1000000);};/** @brief TrackerSampler based on CS (current state), used by algorithm TrackerBoosting
	///  */classCV_EXPORTSTrackerSamplerCS:publicTrackerSamplerAlgorithm{public:enum{MODE_POSITIVE=1,//!< mode for positive samplesMODE_NEGATIVE=2,//!< mode for negative samplesMODE_CLASSIFY=3//!< mode for classify samples};structCV_EXPORTSParams{Params();floatoverlap;//!<overlapping for the search windowsfloatsearchFactor;//!<search region parameter};/** @brief Constructor
	///     @param parameters TrackerSamplerCS parameters TrackerSamplerCS::Params
	///      */TrackerSamplerCS(constTrackerSamplerCS::Params&parameters=TrackerSamplerCS::Params());/** @brief Set the sampling mode of TrackerSamplerCS
	///     @param samplingMode The sampling mode
	/// 
	///     The modes are:
	/// 
	///     -   "MODE_POSITIVE = 1" -- for the positive sampling
	///     -   "MODE_NEGATIVE = 2" -- for the negative sampling
	///     -   "MODE_CLASSIFY = 3" -- for the sampling in classification step
	///      */voidsetMode(intsamplingMode);~TrackerSamplerCS();boolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample);RectgetROI()const;private:RectgetTrackingROI(floatsearchFactor);RectRectMultiply(constRect&rect,floatf);std::vector<Mat>patchesRegularScan(constMat&image,RecttrackingROI,SizepatchSize);voidsetCheckedROI(RectimageROI);Paramsparams;intmode;RecttrackedPatch;RectvalidROI;RectROI;};/** @brief This sampler is based on particle filtering.
	/// 
	/// In principle, it can be thought of as performing some sort of optimization (and indeed, this
	/// tracker uses opencv's optim module), where tracker seeks to find the rectangle in given frame,
	/// which is the most *"similar"* to the initial rectangle (the one, given through the constructor).
	/// 
	/// The optimization performed is stochastic and somehow resembles genetic algorithms, where on each new
	/// image received (submitted via TrackerSamplerPF::sampling()) we start with the region bounded by
	/// boundingBox, then generate several "perturbed" boxes, take the ones most similar to the original.
	/// This selection round is repeated several times. At the end, we hope that only the most promising box
	/// remaining, and these are combined to produce the subrectangle of image, which is put as a sole
	/// element in array sample.
	/// 
	/// It should be noted, that the definition of "similarity" between two rectangles is based on comparing
	/// their histograms. As experiments show, tracker is *not* very succesfull if target is assumed to
	/// strongly change its dimensions.
	///  */classCV_EXPORTSTrackerSamplerPF:publicTrackerSamplerAlgorithm{public:/** @brief This structure contains all the parameters that can be varied during the course of sampling
	///     algorithm. Below is the structure exposed, together with its members briefly explained with
	///     reference to the above discussion on algorithm's working.
	///  */structCV_EXPORTSParams{Params();intiterationNum;//!< number of selection roundsintparticlesNum;//!< number of "perturbed" boxes on each rounddoublealpha;//!< with each new round we exponentially decrease the amount of "perturbing" we allow (like in simulated annealing)//!< and this very alpha controls how fast annealing happens, ie. how fast perturbing decreasesMat_<double>std;//!< initial values for perturbing (1-by-4 array, as each rectangle is given by 4 values -- coordinates of opposite vertices,//!< hence we have 4 values to perturb)};/** @brief Constructor
	///     @param chosenRect Initial rectangle, that is supposed to contain target we'd like to track.
	///     @param parameters
	///      */TrackerSamplerPF(constMat&chosenRect,constTrackerSamplerPF::Params&parameters=TrackerSamplerPF::Params());protected:boolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample);private:Paramsparams;Ptr<MinProblemSolver>_solver;Ptr<MinProblemSolver::Function>_function;};/************************************ Specific TrackerFeature Classes ************************************//**
	///  * \brief TrackerFeature based on Feature2D
	///  */classCV_EXPORTSTrackerFeatureFeature2d:publicTrackerFeature{public:/**
	///    * \brief Constructor
	///    * \param detectorType string of FeatureDetector
	///    * \param descriptorType string of DescriptorExtractor
	///    */TrackerFeatureFeature2d(StringdetectorType,StringdescriptorType);~TrackerFeatureFeature2d();voidselection(Mat&response,intnpoints);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);private:std::vector<KeyPoint>keypoints;};/**
	///  * \brief TrackerFeature based on HOG
	///  */classCV_EXPORTSTrackerFeatureHOG:publicTrackerFeature{public:TrackerFeatureHOG();~TrackerFeatureHOG();voidselection(Mat&response,intnpoints);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);};/** @brief TrackerFeature based on HAAR features, used by TrackerMIL and many others algorithms
	/// @note HAAR features implementation is copied from apps/traincascade and modified according to MIL
	///  */classCV_EXPORTSTrackerFeatureHAAR:publicTrackerFeature{public:structCV_EXPORTSParams{Params();intnumFeatures;//!< # of rectsSizerectSize;//!< rect sizeboolisIntegral;//!< true if input images are integral, false otherwise};/** @brief Constructor
	///     @param parameters TrackerFeatureHAAR parameters TrackerFeatureHAAR::Params
	///      */TrackerFeatureHAAR(constTrackerFeatureHAAR::Params&parameters=TrackerFeatureHAAR::Params());~TrackerFeatureHAAR();/** @brief Compute the features only for the selected indices in the images collection
	///     @param selFeatures indices of selected features
	///     @param images The images
	///     @param response Collection of response for the specific TrackerFeature
	///      */boolextractSelected(conststd::vector<int>selFeatures,conststd::vector<Mat>&images,Mat&response);/** @brief Identify most effective features
	///     @param response Collection of response for the specific TrackerFeature
	///     @param npoints Max number of features
	/// 
	///     @note This method modifies the response parameter
	///      */voidselection(Mat&response,intnpoints);/** @brief Swap the feature in position source with the feature in position target
	///   @param source The source position
	///   @param target The target position
	///  */boolswapFeature(intsource,inttarget);/** @brief   Swap the feature in position id with the feature input
	///   @param id The position
	///   @param feature The feature
	///  */boolswapFeature(intid,CvHaarEvaluator::FeatureHaar&feature);/** @brief Get the feature in position id
	///     @param id The position
	///      */CvHaarEvaluator::FeatureHaar&getFeatureAt(intid);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);private:Paramsparams;Ptr<CvHaarEvaluator>featureEvaluator;};/**
	///  * \brief TrackerFeature based on LBP
	///  */classCV_EXPORTSTrackerFeatureLBP:publicTrackerFeature{public:TrackerFeatureLBP();~TrackerFeatureLBP();voidselection(Mat&response,intnpoints);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);};/************************************ Specific Tracker Classes ************************************//** @brief The MIL algorithm trains a classifier in an online manner to separate the object from the
	/// background.
	/// 
	/// Multiple Instance Learning avoids the drift problem for a robust tracking. The implementation is
	/// based on @cite MIL .
	/// 
	/// Original code can be found here <http://vision.ucsd.edu/~bbabenko/project_miltrack.shtml>
	///  */classCV_EXPORTSTrackerMIL:publicTracker{public:structCV_EXPORTSParams{Params();//parameters for samplerfloatsamplerInitInRadius;//!< radius for gathering positive instances during initintsamplerInitMaxNegNum;//!< # negative samples to use during initfloatsamplerSearchWinSize;//!< size of search windowfloatsamplerTrackInRadius;//!< radius for gathering positive instances during trackingintsamplerTrackMaxPosNum;//!< # positive samples to use during trackingintsamplerTrackMaxNegNum;//!< # negative samples to use during trackingintfeatureSetNumFeatures;//!< # featuresvoidread(constFileNode&fn);voidwrite(FileStorage&fs)const;};/** @brief Constructor
	///     @param parameters MIL parameters TrackerMIL::Params
	///      */BOILERPLATE_CODE("MIL",TrackerMIL)
	pub fn create_tracker(parameters: &crate::tracking::TrackerMIL_Params) -> Result<core::Ptr::<dyn crate::tracking::TrackerMIL>> {
		unsafe { sys::cv_TrackerMIL_createTracker_const_ParamsX(parameters.as_raw_TrackerMIL_Params()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::tracking::TrackerMIL>::opencv_from_extern(r) } )
	}
	
}
pub trait TrackerMIL_ParamsTrait {
	fn as_raw_TrackerMIL_Params(&self) -> *const c_void;
	fn as_raw_mut_TrackerMIL_Params(&mut self) -> *mut c_void;

	/// radius for gathering positive instances during init
	fn sampler_init_in_radius(&self) -> f32 {
		unsafe { sys::cv_TrackerMIL_Params_getPropSamplerInitInRadius_const(self.as_raw_TrackerMIL_Params()) }.into_result().expect("Infallible function failed: sampler_init_in_radius")
	}
	
	/// radius for gathering positive instances during init
	fn set_sampler_init_in_radius(&mut self, val: f32) -> () {
		unsafe { sys::cv_TrackerMIL_Params_setPropSamplerInitInRadius_float(self.as_raw_mut_TrackerMIL_Params(), val) }.into_result().expect("Infallible function failed: set_sampler_init_in_radius")
	}
	
	/// # negative samples to use during init
	fn sampler_init_max_neg_num(&self) -> i32 {
		unsafe { sys::cv_TrackerMIL_Params_getPropSamplerInitMaxNegNum_const(self.as_raw_TrackerMIL_Params()) }.into_result().expect("Infallible function failed: sampler_init_max_neg_num")
	}
	
	/// # negative samples to use during init
	fn set_sampler_init_max_neg_num(&mut self, val: i32) -> () {
		unsafe { sys::cv_TrackerMIL_Params_setPropSamplerInitMaxNegNum_int(self.as_raw_mut_TrackerMIL_Params(), val) }.into_result().expect("Infallible function failed: set_sampler_init_max_neg_num")
	}
	
	/// size of search window
	fn sampler_search_win_size(&self) -> f32 {
		unsafe { sys::cv_TrackerMIL_Params_getPropSamplerSearchWinSize_const(self.as_raw_TrackerMIL_Params()) }.into_result().expect("Infallible function failed: sampler_search_win_size")
	}
	
	/// size of search window
	fn set_sampler_search_win_size(&mut self, val: f32) -> () {
		unsafe { sys::cv_TrackerMIL_Params_setPropSamplerSearchWinSize_float(self.as_raw_mut_TrackerMIL_Params(), val) }.into_result().expect("Infallible function failed: set_sampler_search_win_size")
	}
	
	/// radius for gathering positive instances during tracking
	fn sampler_track_in_radius(&self) -> f32 {
		unsafe { sys::cv_TrackerMIL_Params_getPropSamplerTrackInRadius_const(self.as_raw_TrackerMIL_Params()) }.into_result().expect("Infallible function failed: sampler_track_in_radius")
	}
	
	/// radius for gathering positive instances during tracking
	fn set_sampler_track_in_radius(&mut self, val: f32) -> () {
		unsafe { sys::cv_TrackerMIL_Params_setPropSamplerTrackInRadius_float(self.as_raw_mut_TrackerMIL_Params(), val) }.into_result().expect("Infallible function failed: set_sampler_track_in_radius")
	}
	
	/// # positive samples to use during tracking
	fn sampler_track_max_pos_num(&self) -> i32 {
		unsafe { sys::cv_TrackerMIL_Params_getPropSamplerTrackMaxPosNum_const(self.as_raw_TrackerMIL_Params()) }.into_result().expect("Infallible function failed: sampler_track_max_pos_num")
	}
	
	/// # positive samples to use during tracking
	fn set_sampler_track_max_pos_num(&mut self, val: i32) -> () {
		unsafe { sys::cv_TrackerMIL_Params_setPropSamplerTrackMaxPosNum_int(self.as_raw_mut_TrackerMIL_Params(), val) }.into_result().expect("Infallible function failed: set_sampler_track_max_pos_num")
	}
	
	/// # negative samples to use during tracking
	fn sampler_track_max_neg_num(&self) -> i32 {
		unsafe { sys::cv_TrackerMIL_Params_getPropSamplerTrackMaxNegNum_const(self.as_raw_TrackerMIL_Params()) }.into_result().expect("Infallible function failed: sampler_track_max_neg_num")
	}
	
	/// # negative samples to use during tracking
	fn set_sampler_track_max_neg_num(&mut self, val: i32) -> () {
		unsafe { sys::cv_TrackerMIL_Params_setPropSamplerTrackMaxNegNum_int(self.as_raw_mut_TrackerMIL_Params(), val) }.into_result().expect("Infallible function failed: set_sampler_track_max_neg_num")
	}
	
	/// # features
	fn feature_set_num_features(&self) -> i32 {
		unsafe { sys::cv_TrackerMIL_Params_getPropFeatureSetNumFeatures_const(self.as_raw_TrackerMIL_Params()) }.into_result().expect("Infallible function failed: feature_set_num_features")
	}
	
	/// # features
	fn set_feature_set_num_features(&mut self, val: i32) -> () {
		unsafe { sys::cv_TrackerMIL_Params_setPropFeatureSetNumFeatures_int(self.as_raw_mut_TrackerMIL_Params(), val) }.into_result().expect("Infallible function failed: set_feature_set_num_features")
	}
	
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_TrackerMIL_Params_read_const_FileNodeX(self.as_raw_mut_TrackerMIL_Params(), fn_.as_raw_FileNode()) }.into_result()
	}
	
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_TrackerMIL_Params_write_const_FileStorageX(self.as_raw_TrackerMIL_Params(), fs.as_raw_mut_FileStorage()) }.into_result()
	}
	
}

pub struct TrackerMIL_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerMIL_Params }

impl Drop for TrackerMIL_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerMIL_Params_delete(instance: *mut c_void); }
		unsafe { cv_TrackerMIL_Params_delete(self.as_raw_mut_TrackerMIL_Params()) };
	}
}

impl TrackerMIL_Params {
	pub fn as_raw_TrackerMIL_Params(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerMIL_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerMIL_Params {}

impl crate::tracking::TrackerMIL_ParamsTrait for TrackerMIL_Params {
	fn as_raw_TrackerMIL_Params(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerMIL_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerMIL_Params {
	pub fn default() -> Result<crate::tracking::TrackerMIL_Params> {
		unsafe { sys::cv_TrackerMIL_Params_Params() }.into_result().map(|r| unsafe { crate::tracking::TrackerMIL_Params::opencv_from_extern(r) } )
	}
	
}

/// Median Flow tracker implementation.
/// 
/// Implementation of a paper [MedianFlow](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_MedianFlow) .
/// 
/// The tracker is suitable for very smooth and predictable movements when object is visible throughout
/// the whole sequence. It's quite and accurate for this type of problems (in particular, it was shown
/// by authors to outperform MIL). During the implementation period the code at
/// <http://www.aonsquared.co.uk/node/5>, the courtesy of the author Arthur Amarra, was used for the
/// reference purpose.
pub trait TrackerMedianFlow: crate::tracking::Tracker {
	fn as_raw_TrackerMedianFlow(&self) -> *const c_void;
	fn as_raw_mut_TrackerMedianFlow(&mut self) -> *mut c_void;

}

impl dyn TrackerMedianFlow + '_ {
	/// ## C++ default parameters
	/// * parameters: classname::Params());virtual~classname(){};/*
	///  * Partially based on:
	///  * ====================================================================================================================
	///  *   - [AAM] S. Salti, A. Cavallaro, L. Di Stefano, Adaptive Appearance Modeling for Video Tracking: Survey and Evaluation
	///  *  - [AMVOT] X. Li, W. Hu, C. Shen, Z. Zhang, A. Dick, A. van den Hengel, A Survey of Appearance Models in Visual Object Tracking
	///  *
	///  * This Tracking API has been designed with PlantUML. If you modify this API please change UML files under modules/tracking/doc/uml
	///  *
	///  */namespacecv{//! @addtogroup tracking//! @{/************************************ TrackerFeature Base Classes ************************************//** @brief Abstract base class for TrackerFeature that represents the feature.
	///  */classCV_EXPORTSTrackerFeature{public:virtual~TrackerFeature();/** @brief Compute the features in the images collection
	///     @param images The images
	///     @param response The output response
	///      */voidcompute(conststd::vector<Mat>&images,Mat&response);/** @brief Create TrackerFeature by tracker feature type
	///     @param trackerFeatureType The TrackerFeature name
	/// 
	///     The modes available now:
	/// 
	///     -   "HAAR" -- Haar Feature-based
	/// 
	///     The modes that will be available soon:
	/// 
	///     -   "HOG" -- Histogram of Oriented Gradients features
	///     -   "LBP" -- Local Binary Pattern features
	///     -   "FEATURE2D" -- All types of Feature2D
	///      */staticPtr<TrackerFeature>create(constString&trackerFeatureType);/** @brief Identify most effective features
	///     @param response Collection of response for the specific TrackerFeature
	///     @param npoints Max number of features
	/// 
	///     @note This method modifies the response parameter
	///      */virtualvoidselection(Mat&response,intnpoints)=0;/** @brief Get the name of the specific TrackerFeature
	///      */StringgetClassName()const;protected:virtualboolcomputeImpl(conststd::vector<Mat>&images,Mat&response)=0;StringclassName;};/** @brief Class that manages the extraction and selection of features
	/// 
	/// @cite AAM Feature Extraction and Feature Set Refinement (Feature Processing and Feature Selection).
	/// See table I and section III C @cite AMVOT Appearance modelling -\> Visual representation (Table II,
	/// section 3.1 - 3.2)
	/// 
	/// TrackerFeatureSet is an aggregation of TrackerFeature
	/// 
	/// @sa
	///    TrackerFeature
	/// 
	///  */classCV_EXPORTSTrackerFeatureSet{public:TrackerFeatureSet();~TrackerFeatureSet();/** @brief Extract features from the images collection
	///     @param images The input images
	///      */voidextraction(conststd::vector<Mat>&images);/** @brief Identify most effective features for all feature types (optional)
	///      */voidselection();/** @brief Remove outliers for all feature types (optional)
	///      */voidremoveOutliers();/** @brief Add TrackerFeature in the collection. Return true if TrackerFeature is added, false otherwise
	///     @param trackerFeatureType The TrackerFeature name
	/// 
	///     The modes available now:
	/// 
	///     -   "HAAR" -- Haar Feature-based
	/// 
	///     The modes that will be available soon:
	/// 
	///     -   "HOG" -- Histogram of Oriented Gradients features
	///     -   "LBP" -- Local Binary Pattern features
	///     -   "FEATURE2D" -- All types of Feature2D
	/// 
	///     Example TrackerFeatureSet::addTrackerFeature : :
	///     @code
	///         //sample usage:
	/// 
	///         Ptr<TrackerFeature> trackerFeature = new TrackerFeatureHAAR( HAARparameters );
	///         featureSet->addTrackerFeature( trackerFeature );
	/// 
	///         //or add CSC sampler with default parameters
	///         //featureSet->addTrackerFeature( "HAAR" );
	///     @endcode
	///     @note If you use the second method, you must initialize the TrackerFeature
	///      */booladdTrackerFeature(StringtrackerFeatureType);/** @overload
	///     @param feature The TrackerFeature class
	///     */booladdTrackerFeature(Ptr<TrackerFeature>&feature);/** @brief Get the TrackerFeature collection (TrackerFeature name, TrackerFeature pointer)
	///      */conststd::vector<std::pair<String,Ptr<TrackerFeature>>>&getTrackerFeature()const;/** @brief Get the responses
	/// 
	///     @note Be sure to call extraction before getResponses Example TrackerFeatureSet::getResponses : :
	///      */conststd::vector<Mat>&getResponses()const;private:voidclearResponses();boolblockAddTrackerFeature;std::vector<std::pair<String,Ptr<TrackerFeature>>>features;//list of featuresstd::vector<Mat>responses;//list of response after compute};/************************************ TrackerSampler Base Classes ************************************//** @brief Abstract base class for TrackerSamplerAlgorithm that represents the algorithm for the specific
	/// sampler.
	///  */classCV_EXPORTSTrackerSamplerAlgorithm{public:/**
	///    * \brief Destructor
	///    */virtual~TrackerSamplerAlgorithm();/** @brief Create TrackerSamplerAlgorithm by tracker sampler type.
	///     @param trackerSamplerType The trackerSamplerType name
	/// 
	///     The modes available now:
	/// 
	///     -   "CSC" -- Current State Center
	///     -   "CS" -- Current State
	///      */staticPtr<TrackerSamplerAlgorithm>create(constString&trackerSamplerType);/** @brief Computes the regions starting from a position in an image.
	/// 
	///     Return true if samples are computed, false otherwise
	/// 
	///     @param image The current frame
	///     @param boundingBox The bounding box from which regions can be calculated
	/// 
	///     @param sample The computed samples @cite AAM Fig. 1 variable Sk
	///      */boolsampling(constMat&image,RectboundingBox,std::vector<Mat>&sample);/** @brief Get the name of the specific TrackerSamplerAlgorithm
	///     */StringgetClassName()const;protected:StringclassName;virtualboolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample)=0;};/**
	///  * \brief Class that manages the sampler in order to select regions for the update the model of the tracker
	///  * [AAM] Sampling e Labeling. See table I and section III B
	///  *//** @brief Class that manages the sampler in order to select regions for the update the model of the tracker
	/// 
	/// @cite AAM Sampling e Labeling. See table I and section III B
	/// 
	/// TrackerSampler is an aggregation of TrackerSamplerAlgorithm
	/// @sa
	///    TrackerSamplerAlgorithm
	///  */classCV_EXPORTSTrackerSampler{public:/**
	///    * \brief Constructor
	///    */TrackerSampler();/**
	///    * \brief Destructor
	///    */~TrackerSampler();/** @brief Computes the regions starting from a position in an image
	///     @param image The current frame
	///     @param boundingBox The bounding box from which regions can be calculated
	///      */voidsampling(constMat&image,RectboundingBox);/** @brief Return the collection of the TrackerSamplerAlgorithm
	///     */conststd::vector<std::pair<String,Ptr<TrackerSamplerAlgorithm>>>&getSamplers()const;/** @brief Return the samples from all TrackerSamplerAlgorithm, @cite AAM Fig. 1 variable Sk
	///     */conststd::vector<Mat>&getSamples()const;/** @brief Add TrackerSamplerAlgorithm in the collection. Return true if sampler is added, false otherwise
	///     @param trackerSamplerAlgorithmType The TrackerSamplerAlgorithm name
	/// 
	///     The modes available now:
	///     -   "CSC" -- Current State Center
	///     -   "CS" -- Current State
	///     -   "PF" -- Particle Filtering
	/// 
	///     Example TrackerSamplerAlgorithm::addTrackerSamplerAlgorithm : :
	///     @code
	///          TrackerSamplerCSC::Params CSCparameters;
	///          Ptr<TrackerSamplerAlgorithm> CSCSampler = new TrackerSamplerCSC( CSCparameters );
	/// 
	///          if( !sampler->addTrackerSamplerAlgorithm( CSCSampler ) )
	///            return false;
	/// 
	///          //or add CSC sampler with default parameters
	///          //sampler->addTrackerSamplerAlgorithm( "CSC" );
	///     @endcode
	///     @note If you use the second method, you must initialize the TrackerSamplerAlgorithm
	///      */booladdTrackerSamplerAlgorithm(StringtrackerSamplerAlgorithmType);/** @overload
	///     @param sampler The TrackerSamplerAlgorithm
	///     */booladdTrackerSamplerAlgorithm(Ptr<TrackerSamplerAlgorithm>&sampler);private:std::vector<std::pair<String,Ptr<TrackerSamplerAlgorithm>>>samplers;std::vector<Mat>samples;boolblockAddTrackerSampler;voidclearSamples();};/************************************ TrackerModel Base Classes ************************************//** @brief Abstract base class for TrackerTargetState that represents a possible state of the target.
	/// 
	/// See @cite AAM \f$\hat{x}^{i}_{k}\f$ all the states candidates.
	/// 
	/// Inherits this class with your Target state, In own implementation you can add scale variation,
	/// width, height, orientation, etc.
	///  */classCV_EXPORTSTrackerTargetState{public:virtual~TrackerTargetState(){};/**
	///    * \brief Get the position
	///    * \return The position
	///    */Point2fgetTargetPosition()const;/**
	///    * \brief Set the position
	///    * \param position The position
	///    */voidsetTargetPosition(constPoint2f&position);/**
	///    * \brief Get the width of the target
	///    * \return The width of the target
	///    */intgetTargetWidth()const;/**
	///    * \brief Set the width of the target
	///    * \param width The width of the target
	///    */voidsetTargetWidth(intwidth);/**
	///    * \brief Get the height of the target
	///    * \return The height of the target
	///    */intgetTargetHeight()const;/**
	///    * \brief Set the height of the target
	///    * \param height The height of the target
	///    */voidsetTargetHeight(intheight);protected:Point2ftargetPosition;inttargetWidth;inttargetHeight;};/** @brief Represents the model of the target at frame \f$k\f$ (all states and scores)
	/// 
	/// See @cite AAM The set of the pair \f$\langle \hat{x}^{i}_{k}, C^{i}_{k} \rangle\f$
	/// @sa TrackerTargetState
	///  */typedefstd::vector<std::pair<Ptr<TrackerTargetState>,float>>ConfidenceMap;/** @brief Represents the estimate states for all frames
	/// 
	/// @cite AAM \f$x_{k}\f$ is the trajectory of the target up to time \f$k\f$
	/// 
	/// @sa TrackerTargetState
	///  */typedefstd::vector<Ptr<TrackerTargetState>>Trajectory;/** @brief Abstract base class for TrackerStateEstimator that estimates the most likely target state.
	/// 
	/// See @cite AAM State estimator
	/// 
	/// See @cite AMVOT Statistical modeling (Fig. 3), Table III (generative) - IV (discriminative) - V (hybrid)
	///  */classCV_EXPORTSTrackerStateEstimator{public:virtual~TrackerStateEstimator();/** @brief Estimate the most likely target state, return the estimated state
	///     @param confidenceMaps The overall appearance model as a list of :cConfidenceMap
	///      */Ptr<TrackerTargetState>estimate(conststd::vector<ConfidenceMap>&confidenceMaps);/** @brief Update the ConfidenceMap with the scores
	///     @param confidenceMaps The overall appearance model as a list of :cConfidenceMap
	///      */voidupdate(std::vector<ConfidenceMap>&confidenceMaps);/** @brief Create TrackerStateEstimator by tracker state estimator type
	///     @param trackeStateEstimatorType The TrackerStateEstimator name
	/// 
	///     The modes available now:
	/// 
	///     -   "BOOSTING" -- Boosting-based discriminative appearance models. See @cite AMVOT section 4.4
	/// 
	///     The modes available soon:
	/// 
	///     -   "SVM" -- SVM-based discriminative appearance models. See @cite AMVOT section 4.5
	///      */staticPtr<TrackerStateEstimator>create(constString&trackeStateEstimatorType);/** @brief Get the name of the specific TrackerStateEstimator
	///      */StringgetClassName()const;protected:virtualPtr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps)=0;virtualvoidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps)=0;StringclassName;};/** @brief Abstract class that represents the model of the target. It must be instantiated by specialized
	/// tracker
	/// 
	/// See @cite AAM Ak
	/// 
	/// Inherits this with your TrackerModel
	///  */classCV_EXPORTSTrackerModel{public:/**
	///    * \brief Constructor
	///    */TrackerModel();/**
	///    * \brief Destructor
	///    */virtual~TrackerModel();/** @brief Set TrackerEstimator, return true if the tracker state estimator is added, false otherwise
	///     @param trackerStateEstimator The TrackerStateEstimator
	///     @note You can add only one TrackerStateEstimator
	///      */boolsetTrackerStateEstimator(Ptr<TrackerStateEstimator>trackerStateEstimator);/** @brief Estimate the most likely target location
	/// 
	///     @cite AAM ME, Model Estimation table I
	///     @param responses Features extracted from TrackerFeatureSet
	///      */voidmodelEstimation(conststd::vector<Mat>&responses);/** @brief Update the model
	/// 
	///     @cite AAM MU, Model Update table I
	///      */voidmodelUpdate();/** @brief Run the TrackerStateEstimator, return true if is possible to estimate a new state, false otherwise
	///     */boolrunStateEstimator();/** @brief Set the current TrackerTargetState in the Trajectory
	///     @param lastTargetState The current TrackerTargetState
	///      */voidsetLastTargetState(constPtr<TrackerTargetState>&lastTargetState);/** @brief Get the last TrackerTargetState from Trajectory
	///     */Ptr<TrackerTargetState>getLastTargetState()const;/** @brief Get the list of the ConfidenceMap
	///     */conststd::vector<ConfidenceMap>&getConfidenceMaps()const;/** @brief Get the last ConfidenceMap for the current frame
	///      */constConfidenceMap&getLastConfidenceMap()const;/** @brief Get the TrackerStateEstimator
	///     */Ptr<TrackerStateEstimator>getTrackerStateEstimator()const;private:voidclearCurrentConfidenceMap();protected:std::vector<ConfidenceMap>confidenceMaps;Ptr<TrackerStateEstimator>stateEstimator;ConfidenceMapcurrentConfidenceMap;Trajectorytrajectory;intmaxCMLength;virtualvoidmodelEstimationImpl(conststd::vector<Mat>&responses)=0;virtualvoidmodelUpdateImpl()=0;};/************************************ Tracker Base Class ************************************//** @brief Base abstract class for the long-term tracker:
	///  */classCV_EXPORTS_WTracker:publicvirtualAlgorithm{public:virtual~Tracker();/** @brief Initialize the tracker with a know bounding box that surrounding the target
	///     @param image The initial frame
	///     @param boundingBox The initial boundig box
	/// 
	///     @return True if initialization went succesfully, false otherwise
	///      */CV_WRAPboolinit(constMat&image,constRect2d&boundingBox);/** @brief Update the tracker, find the new most likely bounding box for the target
	///     @param image The current frame
	///     @param boundingBox The boundig box that represent the new target location, if true was returned, not
	///     modified otherwise
	/// 
	///     @return True means that target was located and false means that tracker cannot locate target in
	///     current frame. Note, that latter *does not* imply that tracker has failed, maybe target is indeed
	///     missing from the frame (say, out of sight)
	///      */CV_WRAPboolupdate(constMat&image,CV_OUTRect2d&boundingBox);/** @brief Creates a tracker by its name.
	///     @param trackerType Tracker type
	/// 
	///     The following detector types are supported:
	/// 
	///     -   "MIL" -- TrackerMIL
	///     -   "BOOSTING" -- TrackerBoosting
	///      */CV_WRAPstaticPtr<Tracker>create(constString&trackerType);virtualvoidread(constFileNode&fn)=0;virtualvoidwrite(FileStorage&fs)const=0;Ptr<TrackerModel>getModel(){returnmodel;}protected:virtualboolinitImpl(constMat&image,constRect2d&boundingBox)=0;virtualboolupdateImpl(constMat&image,Rect2d&boundingBox)=0;boolisInit;Ptr<TrackerFeatureSet>featureSet;Ptr<TrackerSampler>sampler;Ptr<TrackerModel>model;};/************************************ Specific TrackerStateEstimator Classes ************************************//** @brief TrackerStateEstimator based on Boosting
	///     */classCV_EXPORTSTrackerStateEstimatorMILBoosting:publicTrackerStateEstimator{public:/**
	///    * Implementation of the target state for TrackerStateEstimatorMILBoosting
	///    */classTrackerMILTargetState:publicTrackerTargetState{public:/**
	///      * \brief Constructor
	///      * \param position Top left corner of the bounding box
	///      * \param width Width of the bounding box
	///      * \param height Height of the bounding box
	///      * \param foreground label for target or background
	///      * \param features features extracted
	///      */TrackerMILTargetState(constPoint2f&position,intwidth,intheight,boolforeground,constMat&features);/**
	///      * \brief Destructor
	///      */~TrackerMILTargetState(){};/** @brief Set label: true for target foreground, false for background
	///     @param foreground Label for background/foreground
	///      */voidsetTargetFg(boolforeground);/** @brief Set the features extracted from TrackerFeatureSet
	///     @param features The features extracted
	///      */voidsetFeatures(constMat&features);/** @brief Get the label. Return true for target foreground, false for background
	///      */boolisTargetFg()const;/** @brief Get the features extracted
	///      */MatgetFeatures()const;private:boolisTarget;MattargetFeatures;};/** @brief Constructor
	///     @param nFeatures Number of features for each sample
	///      */TrackerStateEstimatorMILBoosting(intnFeatures=250);~TrackerStateEstimatorMILBoosting();/** @brief Set the current confidenceMap
	///     @param confidenceMap The current :cConfidenceMap
	///      */voidsetCurrentConfidenceMap(ConfidenceMap&confidenceMap);protected:Ptr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps);voidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps);private:uintmax_idx(conststd::vector<float>&v);voidprepareData(constConfidenceMap&confidenceMap,Mat&positive,Mat&negative);ClfMilBoostboostMILModel;booltrained;intnumFeatures;ConfidenceMapcurrentConfidenceMap;};/** @brief TrackerStateEstimatorAdaBoosting based on ADA-Boosting
	///  */classCV_EXPORTSTrackerStateEstimatorAdaBoosting:publicTrackerStateEstimator{public:/** @brief Implementation of the target state for TrackerAdaBoostingTargetState
	///     */classTrackerAdaBoostingTargetState:publicTrackerTargetState{public:/**
	///      * \brief Constructor
	///      * \param position Top left corner of the bounding box
	///      * \param width Width of the bounding box
	///      * \param height Height of the bounding box
	///      * \param foreground label for target or background
	///      * \param responses list of features
	///      */TrackerAdaBoostingTargetState(constPoint2f&position,intwidth,intheight,boolforeground,constMat&responses);/**
	///      * \brief Destructor
	///      */~TrackerAdaBoostingTargetState(){};/** @brief Set the features extracted from TrackerFeatureSet
	///     @param responses The features extracted
	///      */voidsetTargetResponses(constMat&responses);/** @brief Set label: true for target foreground, false for background
	///     @param foreground Label for background/foreground
	///      */voidsetTargetFg(boolforeground);/** @brief Get the features extracted
	///      */MatgetTargetResponses()const;/** @brief Get the label. Return true for target foreground, false for background
	///     */boolisTargetFg()const;private:boolisTarget;MattargetResponses;};/** @brief Constructor
	///     @param numClassifer Number of base classifiers
	///     @param initIterations Number of iterations in the initialization
	///     @param nFeatures Number of features/weak classifiers
	///     @param patchSize tracking rect
	///     @param ROI initial ROI
	///      */TrackerStateEstimatorAdaBoosting(intnumClassifer,intinitIterations,intnFeatures,SizepatchSize,constRect&ROI);/**
	///    * \brief Destructor
	///    */~TrackerStateEstimatorAdaBoosting();/** @brief Get the sampling ROI
	///      */RectgetSampleROI()const;/** @brief Set the sampling ROI
	///     @param ROI the sampling ROI
	///      */voidsetSampleROI(constRect&ROI);/** @brief Set the current confidenceMap
	///     @param confidenceMap The current :cConfidenceMap
	///      */voidsetCurrentConfidenceMap(ConfidenceMap&confidenceMap);/** @brief Get the list of the selected weak classifiers for the classification step
	///      */std::vector<int>computeSelectedWeakClassifier();/** @brief Get the list of the weak classifiers that should be replaced
	///      */std::vector<int>computeReplacedClassifier();/** @brief Get the list of the weak classifiers that replace those to be replaced
	///      */std::vector<int>computeSwappedClassifier();protected:Ptr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps);voidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps);Ptr<StrongClassifierDirectSelection>boostClassifier;private:intnumBaseClassifier;intiterationInit;intnumFeatures;booltrained;SizeinitPatchSize;RectsampleROI;std::vector<int>replacedClassifier;std::vector<int>swappedClassifier;ConfidenceMapcurrentConfidenceMap;};/**
	///  * \brief TrackerStateEstimator based on SVM
	///  */classCV_EXPORTSTrackerStateEstimatorSVM:publicTrackerStateEstimator{public:TrackerStateEstimatorSVM();~TrackerStateEstimatorSVM();protected:Ptr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps);voidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps);};/************************************ Specific TrackerSamplerAlgorithm Classes ************************************//** @brief TrackerSampler based on CSC (current state centered), used by MIL algorithm TrackerMIL
	///  */classCV_EXPORTSTrackerSamplerCSC:publicTrackerSamplerAlgorithm{public:enum{MODE_INIT_POS=1,//!< mode for init positive samplesMODE_INIT_NEG=2,//!< mode for init negative samplesMODE_TRACK_POS=3,//!< mode for update positive samplesMODE_TRACK_NEG=4,//!< mode for update negative samplesMODE_DETECT=5//!< mode for detect samples};structCV_EXPORTSParams{Params();floatinitInRad;//!< radius for gathering positive instances during initfloattrackInPosRad;//!< radius for gathering positive instances during trackingfloatsearchWinSize;//!< size of search windowintinitMaxNegNum;//!< # negative samples to use during initinttrackMaxPosNum;//!< # positive samples to use during traininginttrackMaxNegNum;//!< # negative samples to use during training};/** @brief Constructor
	///     @param parameters TrackerSamplerCSC parameters TrackerSamplerCSC::Params
	///      */TrackerSamplerCSC(constTrackerSamplerCSC::Params&parameters=TrackerSamplerCSC::Params());/** @brief Set the sampling mode of TrackerSamplerCSC
	///     @param samplingMode The sampling mode
	/// 
	///     The modes are:
	/// 
	///     -   "MODE_INIT_POS = 1" -- for the positive sampling in initialization step
	///     -   "MODE_INIT_NEG = 2" -- for the negative sampling in initialization step
	///     -   "MODE_TRACK_POS = 3" -- for the positive sampling in update step
	///     -   "MODE_TRACK_NEG = 4" -- for the negative sampling in update step
	///     -   "MODE_DETECT = 5" -- for the sampling in detection step
	///      */voidsetMode(intsamplingMode);~TrackerSamplerCSC();protected:boolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample);private:Paramsparams;intmode;RNGrng;std::vector<Mat>sampleImage(constMat&img,intx,inty,intw,inth,floatinrad,floatoutrad=0,intmaxnum=1000000);};/** @brief TrackerSampler based on CS (current state), used by algorithm TrackerBoosting
	///  */classCV_EXPORTSTrackerSamplerCS:publicTrackerSamplerAlgorithm{public:enum{MODE_POSITIVE=1,//!< mode for positive samplesMODE_NEGATIVE=2,//!< mode for negative samplesMODE_CLASSIFY=3//!< mode for classify samples};structCV_EXPORTSParams{Params();floatoverlap;//!<overlapping for the search windowsfloatsearchFactor;//!<search region parameter};/** @brief Constructor
	///     @param parameters TrackerSamplerCS parameters TrackerSamplerCS::Params
	///      */TrackerSamplerCS(constTrackerSamplerCS::Params&parameters=TrackerSamplerCS::Params());/** @brief Set the sampling mode of TrackerSamplerCS
	///     @param samplingMode The sampling mode
	/// 
	///     The modes are:
	/// 
	///     -   "MODE_POSITIVE = 1" -- for the positive sampling
	///     -   "MODE_NEGATIVE = 2" -- for the negative sampling
	///     -   "MODE_CLASSIFY = 3" -- for the sampling in classification step
	///      */voidsetMode(intsamplingMode);~TrackerSamplerCS();boolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample);RectgetROI()const;private:RectgetTrackingROI(floatsearchFactor);RectRectMultiply(constRect&rect,floatf);std::vector<Mat>patchesRegularScan(constMat&image,RecttrackingROI,SizepatchSize);voidsetCheckedROI(RectimageROI);Paramsparams;intmode;RecttrackedPatch;RectvalidROI;RectROI;};/** @brief This sampler is based on particle filtering.
	/// 
	/// In principle, it can be thought of as performing some sort of optimization (and indeed, this
	/// tracker uses opencv's optim module), where tracker seeks to find the rectangle in given frame,
	/// which is the most *"similar"* to the initial rectangle (the one, given through the constructor).
	/// 
	/// The optimization performed is stochastic and somehow resembles genetic algorithms, where on each new
	/// image received (submitted via TrackerSamplerPF::sampling()) we start with the region bounded by
	/// boundingBox, then generate several "perturbed" boxes, take the ones most similar to the original.
	/// This selection round is repeated several times. At the end, we hope that only the most promising box
	/// remaining, and these are combined to produce the subrectangle of image, which is put as a sole
	/// element in array sample.
	/// 
	/// It should be noted, that the definition of "similarity" between two rectangles is based on comparing
	/// their histograms. As experiments show, tracker is *not* very succesfull if target is assumed to
	/// strongly change its dimensions.
	///  */classCV_EXPORTSTrackerSamplerPF:publicTrackerSamplerAlgorithm{public:/** @brief This structure contains all the parameters that can be varied during the course of sampling
	///     algorithm. Below is the structure exposed, together with its members briefly explained with
	///     reference to the above discussion on algorithm's working.
	///  */structCV_EXPORTSParams{Params();intiterationNum;//!< number of selection roundsintparticlesNum;//!< number of "perturbed" boxes on each rounddoublealpha;//!< with each new round we exponentially decrease the amount of "perturbing" we allow (like in simulated annealing)//!< and this very alpha controls how fast annealing happens, ie. how fast perturbing decreasesMat_<double>std;//!< initial values for perturbing (1-by-4 array, as each rectangle is given by 4 values -- coordinates of opposite vertices,//!< hence we have 4 values to perturb)};/** @brief Constructor
	///     @param chosenRect Initial rectangle, that is supposed to contain target we'd like to track.
	///     @param parameters
	///      */TrackerSamplerPF(constMat&chosenRect,constTrackerSamplerPF::Params&parameters=TrackerSamplerPF::Params());protected:boolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample);private:Paramsparams;Ptr<MinProblemSolver>_solver;Ptr<MinProblemSolver::Function>_function;};/************************************ Specific TrackerFeature Classes ************************************//**
	///  * \brief TrackerFeature based on Feature2D
	///  */classCV_EXPORTSTrackerFeatureFeature2d:publicTrackerFeature{public:/**
	///    * \brief Constructor
	///    * \param detectorType string of FeatureDetector
	///    * \param descriptorType string of DescriptorExtractor
	///    */TrackerFeatureFeature2d(StringdetectorType,StringdescriptorType);~TrackerFeatureFeature2d();voidselection(Mat&response,intnpoints);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);private:std::vector<KeyPoint>keypoints;};/**
	///  * \brief TrackerFeature based on HOG
	///  */classCV_EXPORTSTrackerFeatureHOG:publicTrackerFeature{public:TrackerFeatureHOG();~TrackerFeatureHOG();voidselection(Mat&response,intnpoints);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);};/** @brief TrackerFeature based on HAAR features, used by TrackerMIL and many others algorithms
	/// @note HAAR features implementation is copied from apps/traincascade and modified according to MIL
	///  */classCV_EXPORTSTrackerFeatureHAAR:publicTrackerFeature{public:structCV_EXPORTSParams{Params();intnumFeatures;//!< # of rectsSizerectSize;//!< rect sizeboolisIntegral;//!< true if input images are integral, false otherwise};/** @brief Constructor
	///     @param parameters TrackerFeatureHAAR parameters TrackerFeatureHAAR::Params
	///      */TrackerFeatureHAAR(constTrackerFeatureHAAR::Params&parameters=TrackerFeatureHAAR::Params());~TrackerFeatureHAAR();/** @brief Compute the features only for the selected indices in the images collection
	///     @param selFeatures indices of selected features
	///     @param images The images
	///     @param response Collection of response for the specific TrackerFeature
	///      */boolextractSelected(conststd::vector<int>selFeatures,conststd::vector<Mat>&images,Mat&response);/** @brief Identify most effective features
	///     @param response Collection of response for the specific TrackerFeature
	///     @param npoints Max number of features
	/// 
	///     @note This method modifies the response parameter
	///      */voidselection(Mat&response,intnpoints);/** @brief Swap the feature in position source with the feature in position target
	///   @param source The source position
	///   @param target The target position
	///  */boolswapFeature(intsource,inttarget);/** @brief   Swap the feature in position id with the feature input
	///   @param id The position
	///   @param feature The feature
	///  */boolswapFeature(intid,CvHaarEvaluator::FeatureHaar&feature);/** @brief Get the feature in position id
	///     @param id The position
	///      */CvHaarEvaluator::FeatureHaar&getFeatureAt(intid);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);private:Paramsparams;Ptr<CvHaarEvaluator>featureEvaluator;};/**
	///  * \brief TrackerFeature based on LBP
	///  */classCV_EXPORTSTrackerFeatureLBP:publicTrackerFeature{public:TrackerFeatureLBP();~TrackerFeatureLBP();voidselection(Mat&response,intnpoints);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);};/************************************ Specific Tracker Classes ************************************//** @brief The MIL algorithm trains a classifier in an online manner to separate the object from the
	/// background.
	/// 
	/// Multiple Instance Learning avoids the drift problem for a robust tracking. The implementation is
	/// based on @cite MIL .
	/// 
	/// Original code can be found here <http://vision.ucsd.edu/~bbabenko/project_miltrack.shtml>
	///  */classCV_EXPORTSTrackerMIL:publicTracker{public:structCV_EXPORTSParams{Params();//parameters for samplerfloatsamplerInitInRadius;//!< radius for gathering positive instances during initintsamplerInitMaxNegNum;//!< # negative samples to use during initfloatsamplerSearchWinSize;//!< size of search windowfloatsamplerTrackInRadius;//!< radius for gathering positive instances during trackingintsamplerTrackMaxPosNum;//!< # positive samples to use during trackingintsamplerTrackMaxNegNum;//!< # negative samples to use during trackingintfeatureSetNumFeatures;//!< # featuresvoidread(constFileNode&fn);voidwrite(FileStorage&fs)const;};/** @brief Constructor
	///     @param parameters MIL parameters TrackerMIL::Params
	///      */BOILERPLATE_CODE("MIL",TrackerMIL);};/** @brief This is a real-time object tracking based on a novel on-line version of the AdaBoost algorithm.
	/// 
	/// The classifier uses the surrounding background as negative examples in update step to avoid the
	/// drifting problem. The implementation is based on @cite OLB .
	///  */classCV_EXPORTSTrackerBoosting:publicTracker{public:structCV_EXPORTSParams{Params();intnumClassifiers;//!<the number of classifiers to use in a OnlineBoosting algorithmfloatsamplerOverlap;//!<search region parameters to use in a OnlineBoosting algorithmfloatsamplerSearchFactor;//!< search region parameters to use in a OnlineBoosting algorithmintiterationInit;//!<the initial iterationsintfeatureSetNumFeatures;//!< # features/**
	///      * \brief Read parameters from file
	///      */voidread(constFileNode&fn);/**
	///      * \brief Write parameters in a file
	///      */voidwrite(FileStorage&fs)const;};/** @brief Constructor
	///     @param parameters BOOSTING parameters TrackerBoosting::Params
	///      */BOILERPLATE_CODE("BOOSTING",TrackerBoosting);};/** @brief Median Flow tracker implementation.
	/// 
	/// Implementation of a paper @cite MedianFlow .
	/// 
	/// The tracker is suitable for very smooth and predictable movements when object is visible throughout
	/// the whole sequence. It's quite and accurate for this type of problems (in particular, it was shown
	/// by authors to outperform MIL). During the implementation period the code at
	/// <http://www.aonsquared.co.uk/node/5>, the courtesy of the author Arthur Amarra, was used for the
	/// reference purpose.
	///  */classCV_EXPORTSTrackerMedianFlow:publicTracker{public:structCV_EXPORTSParams{Params();intpointsInGrid;//!<square root of number of keypoints used; increase it to trade//!<accurateness for speed; default value is sensible and recommendedvoidread(constFileNode&/*fn*/);voidwrite(FileStorage&/*fs*/)const;};/** @brief Constructor
	///     @param parameters Median Flow parameters TrackerMedianFlow::Params
	///     */BOILERPLATE_CODE("MEDIANFLOW",TrackerMedianFlow)
	pub fn create_tracker(parameters: &crate::tracking::TrackerMedianFlow_Params) -> Result<core::Ptr::<dyn crate::tracking::TrackerMedianFlow>> {
		unsafe { sys::cv_TrackerMedianFlow_createTracker_const_ParamsX(parameters.as_raw_TrackerMedianFlow_Params()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::tracking::TrackerMedianFlow>::opencv_from_extern(r) } )
	}
	
}
pub trait TrackerMedianFlow_ParamsTrait {
	fn as_raw_TrackerMedianFlow_Params(&self) -> *const c_void;
	fn as_raw_mut_TrackerMedianFlow_Params(&mut self) -> *mut c_void;

	/// square root of number of keypoints used; increase it to trade
	/// accurateness for speed; default value is sensible and recommended
	fn points_in_grid(&self) -> i32 {
		unsafe { sys::cv_TrackerMedianFlow_Params_getPropPointsInGrid_const(self.as_raw_TrackerMedianFlow_Params()) }.into_result().expect("Infallible function failed: points_in_grid")
	}
	
	/// square root of number of keypoints used; increase it to trade
	/// accurateness for speed; default value is sensible and recommended
	fn set_points_in_grid(&mut self, val: i32) -> () {
		unsafe { sys::cv_TrackerMedianFlow_Params_setPropPointsInGrid_int(self.as_raw_mut_TrackerMedianFlow_Params(), val) }.into_result().expect("Infallible function failed: set_points_in_grid")
	}
	
	fn read(&mut self, unnamed: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_TrackerMedianFlow_Params_read_const_FileNodeX(self.as_raw_mut_TrackerMedianFlow_Params(), unnamed.as_raw_FileNode()) }.into_result()
	}
	
	fn write(&self, unnamed: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_TrackerMedianFlow_Params_write_const_FileStorageX(self.as_raw_TrackerMedianFlow_Params(), unnamed.as_raw_mut_FileStorage()) }.into_result()
	}
	
}

pub struct TrackerMedianFlow_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerMedianFlow_Params }

impl Drop for TrackerMedianFlow_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerMedianFlow_Params_delete(instance: *mut c_void); }
		unsafe { cv_TrackerMedianFlow_Params_delete(self.as_raw_mut_TrackerMedianFlow_Params()) };
	}
}

impl TrackerMedianFlow_Params {
	pub fn as_raw_TrackerMedianFlow_Params(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerMedianFlow_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerMedianFlow_Params {}

impl crate::tracking::TrackerMedianFlow_ParamsTrait for TrackerMedianFlow_Params {
	fn as_raw_TrackerMedianFlow_Params(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerMedianFlow_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerMedianFlow_Params {
	pub fn default() -> Result<crate::tracking::TrackerMedianFlow_Params> {
		unsafe { sys::cv_TrackerMedianFlow_Params_Params() }.into_result().map(|r| unsafe { crate::tracking::TrackerMedianFlow_Params::opencv_from_extern(r) } )
	}
	
}

/// Abstract class that represents the model of the target. It must be instantiated by specialized
/// tracker
/// 
/// See [AAM](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_AAM) Ak
/// 
/// Inherits this with your TrackerModel
pub trait TrackerModel {
	fn as_raw_TrackerModel(&self) -> *const c_void;
	fn as_raw_mut_TrackerModel(&mut self) -> *mut c_void;

	/// Set TrackerEstimator, return true if the tracker state estimator is added, false otherwise
	/// ## Parameters
	/// * trackerStateEstimator: The TrackerStateEstimator
	/// 
	/// Note: You can add only one TrackerStateEstimator
	fn set_tracker_state_estimator(&mut self, mut tracker_state_estimator: core::Ptr::<dyn crate::tracking::TrackerStateEstimator>) -> Result<bool> {
		unsafe { sys::cv_TrackerModel_setTrackerStateEstimator_Ptr_TrackerStateEstimator_(self.as_raw_mut_TrackerModel(), tracker_state_estimator.as_raw_mut_PtrOfTrackerStateEstimator()) }.into_result()
	}
	
	/// Estimate the most likely target location
	/// 
	/// [AAM](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_AAM) ME, Model Estimation table I
	/// ## Parameters
	/// * responses: Features extracted from TrackerFeatureSet
	fn model_estimation(&mut self, responses: &core::Vector::<core::Mat>) -> Result<()> {
		unsafe { sys::cv_TrackerModel_modelEstimation_const_vector_Mat_X(self.as_raw_mut_TrackerModel(), responses.as_raw_VectorOfMat()) }.into_result()
	}
	
	/// Update the model
	/// 
	/// [AAM](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_AAM) MU, Model Update table I
	fn model_update(&mut self) -> Result<()> {
		unsafe { sys::cv_TrackerModel_modelUpdate(self.as_raw_mut_TrackerModel()) }.into_result()
	}
	
	/// Run the TrackerStateEstimator, return true if is possible to estimate a new state, false otherwise
	fn run_state_estimator(&mut self) -> Result<bool> {
		unsafe { sys::cv_TrackerModel_runStateEstimator(self.as_raw_mut_TrackerModel()) }.into_result()
	}
	
	/// Set the current TrackerTargetState in the Trajectory
	/// ## Parameters
	/// * lastTargetState: The current TrackerTargetState
	fn set_last_target_state(&mut self, last_target_state: &core::Ptr::<crate::tracking::TrackerTargetState>) -> Result<()> {
		unsafe { sys::cv_TrackerModel_setLastTargetState_const_Ptr_TrackerTargetState_X(self.as_raw_mut_TrackerModel(), last_target_state.as_raw_PtrOfTrackerTargetState()) }.into_result()
	}
	
	/// Get the last TrackerTargetState from Trajectory
	fn get_last_target_state(&self) -> Result<core::Ptr::<crate::tracking::TrackerTargetState>> {
		unsafe { sys::cv_TrackerModel_getLastTargetState_const(self.as_raw_TrackerModel()) }.into_result().map(|r| unsafe { core::Ptr::<crate::tracking::TrackerTargetState>::opencv_from_extern(r) } )
	}
	
	/// Get the TrackerStateEstimator
	fn get_tracker_state_estimator(&self) -> Result<core::Ptr::<dyn crate::tracking::TrackerStateEstimator>> {
		unsafe { sys::cv_TrackerModel_getTrackerStateEstimator_const(self.as_raw_TrackerModel()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::tracking::TrackerStateEstimator>::opencv_from_extern(r) } )
	}
	
}

/// Class that manages the sampler in order to select regions for the update the model of the tracker
/// 
/// [AAM](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_AAM) Sampling e Labeling. See table I and section III B
/// 
/// TrackerSampler is an aggregation of TrackerSamplerAlgorithm
/// ## See also
/// TrackerSamplerAlgorithm
pub trait TrackerSamplerTrait {
	fn as_raw_TrackerSampler(&self) -> *const c_void;
	fn as_raw_mut_TrackerSampler(&mut self) -> *mut c_void;

	/// Computes the regions starting from a position in an image
	/// ## Parameters
	/// * image: The current frame
	/// * boundingBox: The bounding box from which regions can be calculated
	fn sampling(&mut self, image: &core::Mat, bounding_box: core::Rect) -> Result<()> {
		unsafe { sys::cv_TrackerSampler_sampling_const_MatX_Rect(self.as_raw_mut_TrackerSampler(), image.as_raw_Mat(), bounding_box.opencv_to_extern()) }.into_result()
	}
	
	/// Return the samples from all TrackerSamplerAlgorithm, [AAM](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_AAM) Fig. 1 variable Sk
	fn get_samples(&self) -> Result<core::Vector::<core::Mat>> {
		unsafe { sys::cv_TrackerSampler_getSamples_const(self.as_raw_TrackerSampler()) }.into_result().map(|r| unsafe { core::Vector::<core::Mat>::opencv_from_extern(r) } )
	}
	
	/// Add TrackerSamplerAlgorithm in the collection. Return true if sampler is added, false otherwise
	/// ## Parameters
	/// * trackerSamplerAlgorithmType: The TrackerSamplerAlgorithm name
	/// 
	/// The modes available now:
	/// *   "CSC" -- Current State Center
	/// *   "CS" -- Current State
	/// *   "PF" -- Particle Filtering
	/// 
	/// Example TrackerSamplerAlgorithm::addTrackerSamplerAlgorithm : :
	/// ```ignore
	///      TrackerSamplerCSC::Params CSCparameters;
	///      Ptr<TrackerSamplerAlgorithm> CSCSampler = new TrackerSamplerCSC( CSCparameters );
	/// 
	///      if( !sampler->addTrackerSamplerAlgorithm( CSCSampler ) )
	///        return false;
	/// 
	///      //or add CSC sampler with default parameters
	///      //sampler->addTrackerSamplerAlgorithm( "CSC" );
	/// ```
	/// 
	/// 
	/// Note: If you use the second method, you must initialize the TrackerSamplerAlgorithm
	fn add_tracker_sampler_algorithm(&mut self, tracker_sampler_algorithm_type: &str) -> Result<bool> {
		extern_container_arg!(mut tracker_sampler_algorithm_type);
		unsafe { sys::cv_TrackerSampler_addTrackerSamplerAlgorithm_String(self.as_raw_mut_TrackerSampler(), tracker_sampler_algorithm_type.opencv_to_extern_mut()) }.into_result()
	}
	
	/// Add TrackerSamplerAlgorithm in the collection. Return true if sampler is added, false otherwise
	/// ## Parameters
	/// * trackerSamplerAlgorithmType: The TrackerSamplerAlgorithm name
	/// 
	/// The modes available now:
	/// *   "CSC" -- Current State Center
	/// *   "CS" -- Current State
	/// *   "PF" -- Particle Filtering
	/// 
	/// Example TrackerSamplerAlgorithm::addTrackerSamplerAlgorithm : :
	/// ```ignore
	///      TrackerSamplerCSC::Params CSCparameters;
	///      Ptr<TrackerSamplerAlgorithm> CSCSampler = new TrackerSamplerCSC( CSCparameters );
	/// 
	///      if( !sampler->addTrackerSamplerAlgorithm( CSCSampler ) )
	///        return false;
	/// 
	///      //or add CSC sampler with default parameters
	///      //sampler->addTrackerSamplerAlgorithm( "CSC" );
	/// ```
	/// 
	/// 
	/// Note: If you use the second method, you must initialize the TrackerSamplerAlgorithm
	/// 
	/// ## Overloaded parameters
	/// 
	/// * sampler: The TrackerSamplerAlgorithm
	fn add_tracker_sampler_algorithm_1(&mut self, sampler: &mut core::Ptr::<dyn crate::tracking::TrackerSamplerAlgorithm>) -> Result<bool> {
		unsafe { sys::cv_TrackerSampler_addTrackerSamplerAlgorithm_Ptr_TrackerSamplerAlgorithm_X(self.as_raw_mut_TrackerSampler(), sampler.as_raw_mut_PtrOfTrackerSamplerAlgorithm()) }.into_result()
	}
	
}

/// Class that manages the sampler in order to select regions for the update the model of the tracker
/// 
/// [AAM](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_AAM) Sampling e Labeling. See table I and section III B
/// 
/// TrackerSampler is an aggregation of TrackerSamplerAlgorithm
/// ## See also
/// TrackerSamplerAlgorithm
pub struct TrackerSampler {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerSampler }

impl Drop for TrackerSampler {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerSampler_delete(instance: *mut c_void); }
		unsafe { cv_TrackerSampler_delete(self.as_raw_mut_TrackerSampler()) };
	}
}

impl TrackerSampler {
	pub fn as_raw_TrackerSampler(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerSampler(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerSampler {}

impl crate::tracking::TrackerSamplerTrait for TrackerSampler {
	fn as_raw_TrackerSampler(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerSampler(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerSampler {
	/// \brief Constructor
	pub fn default() -> Result<crate::tracking::TrackerSampler> {
		unsafe { sys::cv_TrackerSampler_TrackerSampler() }.into_result().map(|r| unsafe { crate::tracking::TrackerSampler::opencv_from_extern(r) } )
	}
	
}

/// Abstract base class for TrackerSamplerAlgorithm that represents the algorithm for the specific
/// sampler.
pub trait TrackerSamplerAlgorithm {
	fn as_raw_TrackerSamplerAlgorithm(&self) -> *const c_void;
	fn as_raw_mut_TrackerSamplerAlgorithm(&mut self) -> *mut c_void;

	/// Computes the regions starting from a position in an image.
	/// 
	/// Return true if samples are computed, false otherwise
	/// 
	/// ## Parameters
	/// * image: The current frame
	/// * boundingBox: The bounding box from which regions can be calculated
	/// 
	/// * sample: The computed samples [AAM](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_AAM) Fig. 1 variable Sk
	fn sampling(&mut self, image: &core::Mat, bounding_box: core::Rect, sample: &mut core::Vector::<core::Mat>) -> Result<bool> {
		unsafe { sys::cv_TrackerSamplerAlgorithm_sampling_const_MatX_Rect_vector_Mat_X(self.as_raw_mut_TrackerSamplerAlgorithm(), image.as_raw_Mat(), bounding_box.opencv_to_extern(), sample.as_raw_mut_VectorOfMat()) }.into_result()
	}
	
	/// Get the name of the specific TrackerSamplerAlgorithm
	fn get_class_name(&self) -> Result<String> {
		unsafe { sys::cv_TrackerSamplerAlgorithm_getClassName_const(self.as_raw_TrackerSamplerAlgorithm()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
}

impl dyn TrackerSamplerAlgorithm + '_ {
	/// Create TrackerSamplerAlgorithm by tracker sampler type.
	/// ## Parameters
	/// * trackerSamplerType: The trackerSamplerType name
	/// 
	/// The modes available now:
	/// 
	/// *   "CSC" -- Current State Center
	/// *   "CS" -- Current State
	pub fn create(tracker_sampler_type: &str) -> Result<core::Ptr::<dyn crate::tracking::TrackerSamplerAlgorithm>> {
		extern_container_arg!(tracker_sampler_type);
		unsafe { sys::cv_TrackerSamplerAlgorithm_create_const_StringX(tracker_sampler_type.opencv_to_extern()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::tracking::TrackerSamplerAlgorithm>::opencv_from_extern(r) } )
	}
	
}
/// TrackerSampler based on CS (current state), used by algorithm TrackerBoosting
pub trait TrackerSamplerCSTrait: crate::tracking::TrackerSamplerAlgorithm {
	fn as_raw_TrackerSamplerCS(&self) -> *const c_void;
	fn as_raw_mut_TrackerSamplerCS(&mut self) -> *mut c_void;

	/// Set the sampling mode of TrackerSamplerCS
	/// ## Parameters
	/// * samplingMode: The sampling mode
	/// 
	/// The modes are:
	/// 
	/// *   "MODE_POSITIVE = 1" -- for the positive sampling
	/// *   "MODE_NEGATIVE = 2" -- for the negative sampling
	/// *   "MODE_CLASSIFY = 3" -- for the sampling in classification step
	fn set_mode(&mut self, sampling_mode: i32) -> Result<()> {
		unsafe { sys::cv_TrackerSamplerCS_setMode_int(self.as_raw_mut_TrackerSamplerCS(), sampling_mode) }.into_result()
	}
	
	fn sampling_impl(&mut self, image: &core::Mat, bounding_box: core::Rect, sample: &mut core::Vector::<core::Mat>) -> Result<bool> {
		unsafe { sys::cv_TrackerSamplerCS_samplingImpl_const_MatX_Rect_vector_Mat_X(self.as_raw_mut_TrackerSamplerCS(), image.as_raw_Mat(), bounding_box.opencv_to_extern(), sample.as_raw_mut_VectorOfMat()) }.into_result()
	}
	
	fn get_roi(&self) -> Result<core::Rect> {
		unsafe { sys::cv_TrackerSamplerCS_getROI_const(self.as_raw_TrackerSamplerCS()) }.into_result()
	}
	
}

/// TrackerSampler based on CS (current state), used by algorithm TrackerBoosting
pub struct TrackerSamplerCS {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerSamplerCS }

impl Drop for TrackerSamplerCS {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerSamplerCS_delete(instance: *mut c_void); }
		unsafe { cv_TrackerSamplerCS_delete(self.as_raw_mut_TrackerSamplerCS()) };
	}
}

impl TrackerSamplerCS {
	pub fn as_raw_TrackerSamplerCS(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerSamplerCS(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerSamplerCS {}

impl crate::tracking::TrackerSamplerAlgorithm for TrackerSamplerCS {
	fn as_raw_TrackerSamplerAlgorithm(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerSamplerAlgorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::tracking::TrackerSamplerCSTrait for TrackerSamplerCS {
	fn as_raw_TrackerSamplerCS(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerSamplerCS(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerSamplerCS {
	/// Constructor
	/// ## Parameters
	/// * parameters: TrackerSamplerCS parameters TrackerSamplerCS::Params
	/// 
	/// ## C++ default parameters
	/// * parameters: TrackerSamplerCS::Params()
	pub fn new(parameters: &crate::tracking::TrackerSamplerCS_Params) -> Result<crate::tracking::TrackerSamplerCS> {
		unsafe { sys::cv_TrackerSamplerCS_TrackerSamplerCS_const_ParamsX(parameters.as_raw_TrackerSamplerCS_Params()) }.into_result().map(|r| unsafe { crate::tracking::TrackerSamplerCS::opencv_from_extern(r) } )
	}
	
}

pub trait TrackerSamplerCS_ParamsTrait {
	fn as_raw_TrackerSamplerCS_Params(&self) -> *const c_void;
	fn as_raw_mut_TrackerSamplerCS_Params(&mut self) -> *mut c_void;

	/// overlapping for the search windows
	fn overlap(&self) -> f32 {
		unsafe { sys::cv_TrackerSamplerCS_Params_getPropOverlap_const(self.as_raw_TrackerSamplerCS_Params()) }.into_result().expect("Infallible function failed: overlap")
	}
	
	/// overlapping for the search windows
	fn set_overlap(&mut self, val: f32) -> () {
		unsafe { sys::cv_TrackerSamplerCS_Params_setPropOverlap_float(self.as_raw_mut_TrackerSamplerCS_Params(), val) }.into_result().expect("Infallible function failed: set_overlap")
	}
	
	/// search region parameter
	fn search_factor(&self) -> f32 {
		unsafe { sys::cv_TrackerSamplerCS_Params_getPropSearchFactor_const(self.as_raw_TrackerSamplerCS_Params()) }.into_result().expect("Infallible function failed: search_factor")
	}
	
	/// search region parameter
	fn set_search_factor(&mut self, val: f32) -> () {
		unsafe { sys::cv_TrackerSamplerCS_Params_setPropSearchFactor_float(self.as_raw_mut_TrackerSamplerCS_Params(), val) }.into_result().expect("Infallible function failed: set_search_factor")
	}
	
}

pub struct TrackerSamplerCS_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerSamplerCS_Params }

impl Drop for TrackerSamplerCS_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerSamplerCS_Params_delete(instance: *mut c_void); }
		unsafe { cv_TrackerSamplerCS_Params_delete(self.as_raw_mut_TrackerSamplerCS_Params()) };
	}
}

impl TrackerSamplerCS_Params {
	pub fn as_raw_TrackerSamplerCS_Params(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerSamplerCS_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerSamplerCS_Params {}

impl crate::tracking::TrackerSamplerCS_ParamsTrait for TrackerSamplerCS_Params {
	fn as_raw_TrackerSamplerCS_Params(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerSamplerCS_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerSamplerCS_Params {
	pub fn default() -> Result<crate::tracking::TrackerSamplerCS_Params> {
		unsafe { sys::cv_TrackerSamplerCS_Params_Params() }.into_result().map(|r| unsafe { crate::tracking::TrackerSamplerCS_Params::opencv_from_extern(r) } )
	}
	
}

/// TrackerSampler based on CSC (current state centered), used by MIL algorithm TrackerMIL
pub trait TrackerSamplerCSCTrait: crate::tracking::TrackerSamplerAlgorithm {
	fn as_raw_TrackerSamplerCSC(&self) -> *const c_void;
	fn as_raw_mut_TrackerSamplerCSC(&mut self) -> *mut c_void;

	/// Set the sampling mode of TrackerSamplerCSC
	/// ## Parameters
	/// * samplingMode: The sampling mode
	/// 
	/// The modes are:
	/// 
	/// *   "MODE_INIT_POS = 1" -- for the positive sampling in initialization step
	/// *   "MODE_INIT_NEG = 2" -- for the negative sampling in initialization step
	/// *   "MODE_TRACK_POS = 3" -- for the positive sampling in update step
	/// *   "MODE_TRACK_NEG = 4" -- for the negative sampling in update step
	/// *   "MODE_DETECT = 5" -- for the sampling in detection step
	fn set_mode(&mut self, sampling_mode: i32) -> Result<()> {
		unsafe { sys::cv_TrackerSamplerCSC_setMode_int(self.as_raw_mut_TrackerSamplerCSC(), sampling_mode) }.into_result()
	}
	
}

/// TrackerSampler based on CSC (current state centered), used by MIL algorithm TrackerMIL
pub struct TrackerSamplerCSC {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerSamplerCSC }

impl Drop for TrackerSamplerCSC {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerSamplerCSC_delete(instance: *mut c_void); }
		unsafe { cv_TrackerSamplerCSC_delete(self.as_raw_mut_TrackerSamplerCSC()) };
	}
}

impl TrackerSamplerCSC {
	pub fn as_raw_TrackerSamplerCSC(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerSamplerCSC(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerSamplerCSC {}

impl crate::tracking::TrackerSamplerAlgorithm for TrackerSamplerCSC {
	fn as_raw_TrackerSamplerAlgorithm(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerSamplerAlgorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::tracking::TrackerSamplerCSCTrait for TrackerSamplerCSC {
	fn as_raw_TrackerSamplerCSC(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerSamplerCSC(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerSamplerCSC {
	/// Constructor
	/// ## Parameters
	/// * parameters: TrackerSamplerCSC parameters TrackerSamplerCSC::Params
	/// 
	/// ## C++ default parameters
	/// * parameters: TrackerSamplerCSC::Params()
	pub fn new(parameters: &crate::tracking::TrackerSamplerCSC_Params) -> Result<crate::tracking::TrackerSamplerCSC> {
		unsafe { sys::cv_TrackerSamplerCSC_TrackerSamplerCSC_const_ParamsX(parameters.as_raw_TrackerSamplerCSC_Params()) }.into_result().map(|r| unsafe { crate::tracking::TrackerSamplerCSC::opencv_from_extern(r) } )
	}
	
}

pub trait TrackerSamplerCSC_ParamsTrait {
	fn as_raw_TrackerSamplerCSC_Params(&self) -> *const c_void;
	fn as_raw_mut_TrackerSamplerCSC_Params(&mut self) -> *mut c_void;

	/// radius for gathering positive instances during init
	fn init_in_rad(&self) -> f32 {
		unsafe { sys::cv_TrackerSamplerCSC_Params_getPropInitInRad_const(self.as_raw_TrackerSamplerCSC_Params()) }.into_result().expect("Infallible function failed: init_in_rad")
	}
	
	/// radius for gathering positive instances during init
	fn set_init_in_rad(&mut self, val: f32) -> () {
		unsafe { sys::cv_TrackerSamplerCSC_Params_setPropInitInRad_float(self.as_raw_mut_TrackerSamplerCSC_Params(), val) }.into_result().expect("Infallible function failed: set_init_in_rad")
	}
	
	/// radius for gathering positive instances during tracking
	fn track_in_pos_rad(&self) -> f32 {
		unsafe { sys::cv_TrackerSamplerCSC_Params_getPropTrackInPosRad_const(self.as_raw_TrackerSamplerCSC_Params()) }.into_result().expect("Infallible function failed: track_in_pos_rad")
	}
	
	/// radius for gathering positive instances during tracking
	fn set_track_in_pos_rad(&mut self, val: f32) -> () {
		unsafe { sys::cv_TrackerSamplerCSC_Params_setPropTrackInPosRad_float(self.as_raw_mut_TrackerSamplerCSC_Params(), val) }.into_result().expect("Infallible function failed: set_track_in_pos_rad")
	}
	
	/// size of search window
	fn search_win_size(&self) -> f32 {
		unsafe { sys::cv_TrackerSamplerCSC_Params_getPropSearchWinSize_const(self.as_raw_TrackerSamplerCSC_Params()) }.into_result().expect("Infallible function failed: search_win_size")
	}
	
	/// size of search window
	fn set_search_win_size(&mut self, val: f32) -> () {
		unsafe { sys::cv_TrackerSamplerCSC_Params_setPropSearchWinSize_float(self.as_raw_mut_TrackerSamplerCSC_Params(), val) }.into_result().expect("Infallible function failed: set_search_win_size")
	}
	
	/// # negative samples to use during init
	fn init_max_neg_num(&self) -> i32 {
		unsafe { sys::cv_TrackerSamplerCSC_Params_getPropInitMaxNegNum_const(self.as_raw_TrackerSamplerCSC_Params()) }.into_result().expect("Infallible function failed: init_max_neg_num")
	}
	
	/// # negative samples to use during init
	fn set_init_max_neg_num(&mut self, val: i32) -> () {
		unsafe { sys::cv_TrackerSamplerCSC_Params_setPropInitMaxNegNum_int(self.as_raw_mut_TrackerSamplerCSC_Params(), val) }.into_result().expect("Infallible function failed: set_init_max_neg_num")
	}
	
	/// # positive samples to use during training
	fn track_max_pos_num(&self) -> i32 {
		unsafe { sys::cv_TrackerSamplerCSC_Params_getPropTrackMaxPosNum_const(self.as_raw_TrackerSamplerCSC_Params()) }.into_result().expect("Infallible function failed: track_max_pos_num")
	}
	
	/// # positive samples to use during training
	fn set_track_max_pos_num(&mut self, val: i32) -> () {
		unsafe { sys::cv_TrackerSamplerCSC_Params_setPropTrackMaxPosNum_int(self.as_raw_mut_TrackerSamplerCSC_Params(), val) }.into_result().expect("Infallible function failed: set_track_max_pos_num")
	}
	
	/// # negative samples to use during training
	fn track_max_neg_num(&self) -> i32 {
		unsafe { sys::cv_TrackerSamplerCSC_Params_getPropTrackMaxNegNum_const(self.as_raw_TrackerSamplerCSC_Params()) }.into_result().expect("Infallible function failed: track_max_neg_num")
	}
	
	/// # negative samples to use during training
	fn set_track_max_neg_num(&mut self, val: i32) -> () {
		unsafe { sys::cv_TrackerSamplerCSC_Params_setPropTrackMaxNegNum_int(self.as_raw_mut_TrackerSamplerCSC_Params(), val) }.into_result().expect("Infallible function failed: set_track_max_neg_num")
	}
	
}

pub struct TrackerSamplerCSC_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerSamplerCSC_Params }

impl Drop for TrackerSamplerCSC_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerSamplerCSC_Params_delete(instance: *mut c_void); }
		unsafe { cv_TrackerSamplerCSC_Params_delete(self.as_raw_mut_TrackerSamplerCSC_Params()) };
	}
}

impl TrackerSamplerCSC_Params {
	pub fn as_raw_TrackerSamplerCSC_Params(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerSamplerCSC_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerSamplerCSC_Params {}

impl crate::tracking::TrackerSamplerCSC_ParamsTrait for TrackerSamplerCSC_Params {
	fn as_raw_TrackerSamplerCSC_Params(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerSamplerCSC_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerSamplerCSC_Params {
	pub fn default() -> Result<crate::tracking::TrackerSamplerCSC_Params> {
		unsafe { sys::cv_TrackerSamplerCSC_Params_Params() }.into_result().map(|r| unsafe { crate::tracking::TrackerSamplerCSC_Params::opencv_from_extern(r) } )
	}
	
}

/// This sampler is based on particle filtering.
/// 
/// In principle, it can be thought of as performing some sort of optimization (and indeed, this
/// tracker uses opencv's optim module), where tracker seeks to find the rectangle in given frame,
/// which is the most *"similar"* to the initial rectangle (the one, given through the constructor).
/// 
/// The optimization performed is stochastic and somehow resembles genetic algorithms, where on each new
/// image received (submitted via TrackerSamplerPF::sampling()) we start with the region bounded by
/// boundingBox, then generate several "perturbed" boxes, take the ones most similar to the original.
/// This selection round is repeated several times. At the end, we hope that only the most promising box
/// remaining, and these are combined to produce the subrectangle of image, which is put as a sole
/// element in array sample.
/// 
/// It should be noted, that the definition of "similarity" between two rectangles is based on comparing
/// their histograms. As experiments show, tracker is *not* very succesfull if target is assumed to
/// strongly change its dimensions.
pub trait TrackerSamplerPFTrait: crate::tracking::TrackerSamplerAlgorithm {
	fn as_raw_TrackerSamplerPF(&self) -> *const c_void;
	fn as_raw_mut_TrackerSamplerPF(&mut self) -> *mut c_void;

}

/// This sampler is based on particle filtering.
/// 
/// In principle, it can be thought of as performing some sort of optimization (and indeed, this
/// tracker uses opencv's optim module), where tracker seeks to find the rectangle in given frame,
/// which is the most *"similar"* to the initial rectangle (the one, given through the constructor).
/// 
/// The optimization performed is stochastic and somehow resembles genetic algorithms, where on each new
/// image received (submitted via TrackerSamplerPF::sampling()) we start with the region bounded by
/// boundingBox, then generate several "perturbed" boxes, take the ones most similar to the original.
/// This selection round is repeated several times. At the end, we hope that only the most promising box
/// remaining, and these are combined to produce the subrectangle of image, which is put as a sole
/// element in array sample.
/// 
/// It should be noted, that the definition of "similarity" between two rectangles is based on comparing
/// their histograms. As experiments show, tracker is *not* very succesfull if target is assumed to
/// strongly change its dimensions.
pub struct TrackerSamplerPF {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerSamplerPF }

impl Drop for TrackerSamplerPF {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerSamplerPF_delete(instance: *mut c_void); }
		unsafe { cv_TrackerSamplerPF_delete(self.as_raw_mut_TrackerSamplerPF()) };
	}
}

impl TrackerSamplerPF {
	pub fn as_raw_TrackerSamplerPF(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerSamplerPF(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerSamplerPF {}

impl crate::tracking::TrackerSamplerAlgorithm for TrackerSamplerPF {
	fn as_raw_TrackerSamplerAlgorithm(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerSamplerAlgorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::tracking::TrackerSamplerPFTrait for TrackerSamplerPF {
	fn as_raw_TrackerSamplerPF(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerSamplerPF(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerSamplerPF {
	/// Constructor
	/// ## Parameters
	/// * chosenRect: Initial rectangle, that is supposed to contain target we'd like to track.
	/// * parameters: 
	/// 
	/// ## C++ default parameters
	/// * parameters: TrackerSamplerPF::Params()
	pub fn new(chosen_rect: &core::Mat, parameters: &crate::tracking::TrackerSamplerPF_Params) -> Result<crate::tracking::TrackerSamplerPF> {
		unsafe { sys::cv_TrackerSamplerPF_TrackerSamplerPF_const_MatX_const_ParamsX(chosen_rect.as_raw_Mat(), parameters.as_raw_TrackerSamplerPF_Params()) }.into_result().map(|r| unsafe { crate::tracking::TrackerSamplerPF::opencv_from_extern(r) } )
	}
	
}

/// This structure contains all the parameters that can be varied during the course of sampling
/// algorithm. Below is the structure exposed, together with its members briefly explained with
/// reference to the above discussion on algorithm's working.
pub trait TrackerSamplerPF_ParamsTrait {
	fn as_raw_TrackerSamplerPF_Params(&self) -> *const c_void;
	fn as_raw_mut_TrackerSamplerPF_Params(&mut self) -> *mut c_void;

	/// number of selection rounds
	fn iteration_num(&self) -> i32 {
		unsafe { sys::cv_TrackerSamplerPF_Params_getPropIterationNum_const(self.as_raw_TrackerSamplerPF_Params()) }.into_result().expect("Infallible function failed: iteration_num")
	}
	
	/// number of selection rounds
	fn set_iteration_num(&mut self, val: i32) -> () {
		unsafe { sys::cv_TrackerSamplerPF_Params_setPropIterationNum_int(self.as_raw_mut_TrackerSamplerPF_Params(), val) }.into_result().expect("Infallible function failed: set_iteration_num")
	}
	
	/// number of "perturbed" boxes on each round
	fn particles_num(&self) -> i32 {
		unsafe { sys::cv_TrackerSamplerPF_Params_getPropParticlesNum_const(self.as_raw_TrackerSamplerPF_Params()) }.into_result().expect("Infallible function failed: particles_num")
	}
	
	/// number of "perturbed" boxes on each round
	fn set_particles_num(&mut self, val: i32) -> () {
		unsafe { sys::cv_TrackerSamplerPF_Params_setPropParticlesNum_int(self.as_raw_mut_TrackerSamplerPF_Params(), val) }.into_result().expect("Infallible function failed: set_particles_num")
	}
	
	/// with each new round we exponentially decrease the amount of "perturbing" we allow (like in simulated annealing)
	/// and this very alpha controls how fast annealing happens, ie. how fast perturbing decreases
	fn alpha(&self) -> f64 {
		unsafe { sys::cv_TrackerSamplerPF_Params_getPropAlpha_const(self.as_raw_TrackerSamplerPF_Params()) }.into_result().expect("Infallible function failed: alpha")
	}
	
	/// with each new round we exponentially decrease the amount of "perturbing" we allow (like in simulated annealing)
	/// and this very alpha controls how fast annealing happens, ie. how fast perturbing decreases
	fn set_alpha(&mut self, val: f64) -> () {
		unsafe { sys::cv_TrackerSamplerPF_Params_setPropAlpha_double(self.as_raw_mut_TrackerSamplerPF_Params(), val) }.into_result().expect("Infallible function failed: set_alpha")
	}
	
	/// initial values for perturbing (1-by-4 array, as each rectangle is given by 4 values -- coordinates of opposite vertices,
	/// hence we have 4 values to perturb)
	fn std(&mut self) -> core::Mat_<f64> {
		unsafe { sys::cv_TrackerSamplerPF_Params_getPropStd(self.as_raw_mut_TrackerSamplerPF_Params()) }.into_result().map(|r| unsafe { core::Mat_::<f64>::opencv_from_extern(r) } ).expect("Infallible function failed: std")
	}
	
	/// initial values for perturbing (1-by-4 array, as each rectangle is given by 4 values -- coordinates of opposite vertices,
	/// hence we have 4 values to perturb)
	fn set_std(&mut self, mut val: core::Mat_<f64>) -> () {
		unsafe { sys::cv_TrackerSamplerPF_Params_setPropStd_Mat__double_(self.as_raw_mut_TrackerSamplerPF_Params(), val.as_raw_mut_Mat_()) }.into_result().expect("Infallible function failed: set_std")
	}
	
}

/// This structure contains all the parameters that can be varied during the course of sampling
/// algorithm. Below is the structure exposed, together with its members briefly explained with
/// reference to the above discussion on algorithm's working.
pub struct TrackerSamplerPF_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerSamplerPF_Params }

impl Drop for TrackerSamplerPF_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerSamplerPF_Params_delete(instance: *mut c_void); }
		unsafe { cv_TrackerSamplerPF_Params_delete(self.as_raw_mut_TrackerSamplerPF_Params()) };
	}
}

impl TrackerSamplerPF_Params {
	pub fn as_raw_TrackerSamplerPF_Params(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerSamplerPF_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerSamplerPF_Params {}

impl crate::tracking::TrackerSamplerPF_ParamsTrait for TrackerSamplerPF_Params {
	fn as_raw_TrackerSamplerPF_Params(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerSamplerPF_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerSamplerPF_Params {
	pub fn default() -> Result<crate::tracking::TrackerSamplerPF_Params> {
		unsafe { sys::cv_TrackerSamplerPF_Params_Params() }.into_result().map(|r| unsafe { crate::tracking::TrackerSamplerPF_Params::opencv_from_extern(r) } )
	}
	
}

/// Abstract base class for TrackerStateEstimator that estimates the most likely target state.
/// 
/// See [AAM](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_AAM) State estimator
/// 
/// See [AMVOT](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_AMVOT) Statistical modeling (Fig. 3), Table III (generative) - IV (discriminative) - V (hybrid)
pub trait TrackerStateEstimator {
	fn as_raw_TrackerStateEstimator(&self) -> *const c_void;
	fn as_raw_mut_TrackerStateEstimator(&mut self) -> *mut c_void;

	/// Get the name of the specific TrackerStateEstimator
	fn get_class_name(&self) -> Result<String> {
		unsafe { sys::cv_TrackerStateEstimator_getClassName_const(self.as_raw_TrackerStateEstimator()) }.into_result().map(|r| unsafe { String::opencv_from_extern(r) } )
	}
	
}

impl dyn TrackerStateEstimator + '_ {
	/// Create TrackerStateEstimator by tracker state estimator type
	/// ## Parameters
	/// * trackeStateEstimatorType: The TrackerStateEstimator name
	/// 
	/// The modes available now:
	/// 
	/// *   "BOOSTING" -- Boosting-based discriminative appearance models. See [AMVOT](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_AMVOT) section 4.4
	/// 
	/// The modes available soon:
	/// 
	/// *   "SVM" -- SVM-based discriminative appearance models. See [AMVOT](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_AMVOT) section 4.5
	pub fn create(tracke_state_estimator_type: &str) -> Result<core::Ptr::<dyn crate::tracking::TrackerStateEstimator>> {
		extern_container_arg!(tracke_state_estimator_type);
		unsafe { sys::cv_TrackerStateEstimator_create_const_StringX(tracke_state_estimator_type.opencv_to_extern()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::tracking::TrackerStateEstimator>::opencv_from_extern(r) } )
	}
	
}
/// TrackerStateEstimatorAdaBoosting based on ADA-Boosting
pub trait TrackerStateEstimatorAdaBoostingTrait: crate::tracking::TrackerStateEstimator {
	fn as_raw_TrackerStateEstimatorAdaBoosting(&self) -> *const c_void;
	fn as_raw_mut_TrackerStateEstimatorAdaBoosting(&mut self) -> *mut c_void;

	/// Get the sampling ROI
	fn get_sample_roi(&self) -> Result<core::Rect> {
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_getSampleROI_const(self.as_raw_TrackerStateEstimatorAdaBoosting()) }.into_result()
	}
	
	/// Set the sampling ROI
	/// ## Parameters
	/// * ROI: the sampling ROI
	fn set_sample_roi(&mut self, roi: core::Rect) -> Result<()> {
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_setSampleROI_const_RectX(self.as_raw_mut_TrackerStateEstimatorAdaBoosting(), &roi) }.into_result()
	}
	
	/// Get the list of the selected weak classifiers for the classification step
	fn compute_selected_weak_classifier(&mut self) -> Result<core::Vector::<i32>> {
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_computeSelectedWeakClassifier(self.as_raw_mut_TrackerStateEstimatorAdaBoosting()) }.into_result().map(|r| unsafe { core::Vector::<i32>::opencv_from_extern(r) } )
	}
	
	/// Get the list of the weak classifiers that should be replaced
	fn compute_replaced_classifier(&mut self) -> Result<core::Vector::<i32>> {
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_computeReplacedClassifier(self.as_raw_mut_TrackerStateEstimatorAdaBoosting()) }.into_result().map(|r| unsafe { core::Vector::<i32>::opencv_from_extern(r) } )
	}
	
	/// Get the list of the weak classifiers that replace those to be replaced
	fn compute_swapped_classifier(&mut self) -> Result<core::Vector::<i32>> {
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_computeSwappedClassifier(self.as_raw_mut_TrackerStateEstimatorAdaBoosting()) }.into_result().map(|r| unsafe { core::Vector::<i32>::opencv_from_extern(r) } )
	}
	
}

/// TrackerStateEstimatorAdaBoosting based on ADA-Boosting
pub struct TrackerStateEstimatorAdaBoosting {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerStateEstimatorAdaBoosting }

impl Drop for TrackerStateEstimatorAdaBoosting {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerStateEstimatorAdaBoosting_delete(instance: *mut c_void); }
		unsafe { cv_TrackerStateEstimatorAdaBoosting_delete(self.as_raw_mut_TrackerStateEstimatorAdaBoosting()) };
	}
}

impl TrackerStateEstimatorAdaBoosting {
	pub fn as_raw_TrackerStateEstimatorAdaBoosting(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerStateEstimatorAdaBoosting(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerStateEstimatorAdaBoosting {}

impl crate::tracking::TrackerStateEstimator for TrackerStateEstimatorAdaBoosting {
	fn as_raw_TrackerStateEstimator(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerStateEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::tracking::TrackerStateEstimatorAdaBoostingTrait for TrackerStateEstimatorAdaBoosting {
	fn as_raw_TrackerStateEstimatorAdaBoosting(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerStateEstimatorAdaBoosting(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerStateEstimatorAdaBoosting {
	/// Constructor
	/// ## Parameters
	/// * numClassifer: Number of base classifiers
	/// * initIterations: Number of iterations in the initialization
	/// * nFeatures: Number of features/weak classifiers
	/// * patchSize: tracking rect
	/// * ROI: initial ROI
	pub fn new(num_classifer: i32, init_iterations: i32, n_features: i32, patch_size: core::Size, roi: core::Rect) -> Result<crate::tracking::TrackerStateEstimatorAdaBoosting> {
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_TrackerStateEstimatorAdaBoosting_int_int_int_Size_const_RectX(num_classifer, init_iterations, n_features, patch_size.opencv_to_extern(), &roi) }.into_result().map(|r| unsafe { crate::tracking::TrackerStateEstimatorAdaBoosting::opencv_from_extern(r) } )
	}
	
}

/// Implementation of the target state for TrackerAdaBoostingTargetState
pub trait TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetStateTrait: crate::tracking::TrackerTargetStateTrait {
	fn as_raw_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(&self) -> *const c_void;
	fn as_raw_mut_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(&mut self) -> *mut c_void;

	#[cfg(not(target_os = "windows"))]
	/// Set the features extracted from TrackerFeatureSet
	/// ## Parameters
	/// * responses: The features extracted
	fn set_target_responses(&mut self, responses: &core::Mat) -> Result<()> {
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_setTargetResponses_const_MatX(self.as_raw_mut_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(), responses.as_raw_Mat()) }.into_result()
	}
	
	#[cfg(not(target_os = "windows"))]
	/// Set label: true for target foreground, false for background
	/// ## Parameters
	/// * foreground: Label for background/foreground
	fn set_target_fg(&mut self, foreground: bool) -> Result<()> {
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_setTargetFg_bool(self.as_raw_mut_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(), foreground) }.into_result()
	}
	
	#[cfg(not(target_os = "windows"))]
	/// Get the features extracted
	fn get_target_responses(&self) -> Result<core::Mat> {
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_getTargetResponses_const(self.as_raw_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
	#[cfg(not(target_os = "windows"))]
	/// Get the label. Return true for target foreground, false for background
	fn is_target_fg(&self) -> Result<bool> {
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_isTargetFg_const(self.as_raw_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState()) }.into_result()
	}
	
}

/// Implementation of the target state for TrackerAdaBoostingTargetState
pub struct TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState }

impl Drop for TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_delete(instance: *mut c_void); }
		unsafe { cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_delete(self.as_raw_mut_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState()) };
	}
}

impl TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {
	pub fn as_raw_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {}

impl crate::tracking::TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetStateTrait for TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {
	fn as_raw_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::tracking::TrackerTargetStateTrait for TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {
	fn as_raw_TrackerTargetState(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerTargetState(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {
	#[cfg(not(target_os = "windows"))]
	/// \brief Constructor
	/// \param position Top left corner of the bounding box
	/// \param width Width of the bounding box
	/// \param height Height of the bounding box
	/// \param foreground label for target or background
	/// \param responses list of features
	pub fn new(position: core::Point2f, width: i32, height: i32, foreground: bool, responses: &core::Mat) -> Result<crate::tracking::TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState> {
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_TrackerAdaBoostingTargetState_const_Point2fX_int_int_bool_const_MatX(&position, width, height, foreground, responses.as_raw_Mat()) }.into_result().map(|r| unsafe { crate::tracking::TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState::opencv_from_extern(r) } )
	}
	
}

/// TrackerStateEstimator based on Boosting
pub trait TrackerStateEstimatorMILBoostingTrait: crate::tracking::TrackerStateEstimator {
	fn as_raw_TrackerStateEstimatorMILBoosting(&self) -> *const c_void;
	fn as_raw_mut_TrackerStateEstimatorMILBoosting(&mut self) -> *mut c_void;

}

/// TrackerStateEstimator based on Boosting
pub struct TrackerStateEstimatorMILBoosting {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerStateEstimatorMILBoosting }

impl Drop for TrackerStateEstimatorMILBoosting {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerStateEstimatorMILBoosting_delete(instance: *mut c_void); }
		unsafe { cv_TrackerStateEstimatorMILBoosting_delete(self.as_raw_mut_TrackerStateEstimatorMILBoosting()) };
	}
}

impl TrackerStateEstimatorMILBoosting {
	pub fn as_raw_TrackerStateEstimatorMILBoosting(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerStateEstimatorMILBoosting(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerStateEstimatorMILBoosting {}

impl crate::tracking::TrackerStateEstimator for TrackerStateEstimatorMILBoosting {
	fn as_raw_TrackerStateEstimator(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerStateEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::tracking::TrackerStateEstimatorMILBoostingTrait for TrackerStateEstimatorMILBoosting {
	fn as_raw_TrackerStateEstimatorMILBoosting(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerStateEstimatorMILBoosting(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerStateEstimatorMILBoosting {
	/// Constructor
	/// ## Parameters
	/// * nFeatures: Number of features for each sample
	/// 
	/// ## C++ default parameters
	/// * n_features: 250
	pub fn new(n_features: i32) -> Result<crate::tracking::TrackerStateEstimatorMILBoosting> {
		unsafe { sys::cv_TrackerStateEstimatorMILBoosting_TrackerStateEstimatorMILBoosting_int(n_features) }.into_result().map(|r| unsafe { crate::tracking::TrackerStateEstimatorMILBoosting::opencv_from_extern(r) } )
	}
	
}

/// Implementation of the target state for TrackerStateEstimatorMILBoosting
pub trait TrackerStateEstimatorMILBoosting_TrackerMILTargetStateTrait: crate::tracking::TrackerTargetStateTrait {
	fn as_raw_TrackerStateEstimatorMILBoosting_TrackerMILTargetState(&self) -> *const c_void;
	fn as_raw_mut_TrackerStateEstimatorMILBoosting_TrackerMILTargetState(&mut self) -> *mut c_void;

	#[cfg(not(target_os = "windows"))]
	/// Set label: true for target foreground, false for background
	/// ## Parameters
	/// * foreground: Label for background/foreground
	fn set_target_fg(&mut self, foreground: bool) -> Result<()> {
		unsafe { sys::cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_setTargetFg_bool(self.as_raw_mut_TrackerStateEstimatorMILBoosting_TrackerMILTargetState(), foreground) }.into_result()
	}
	
	#[cfg(not(target_os = "windows"))]
	/// Set the features extracted from TrackerFeatureSet
	/// ## Parameters
	/// * features: The features extracted
	fn set_features(&mut self, features: &core::Mat) -> Result<()> {
		unsafe { sys::cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_setFeatures_const_MatX(self.as_raw_mut_TrackerStateEstimatorMILBoosting_TrackerMILTargetState(), features.as_raw_Mat()) }.into_result()
	}
	
	#[cfg(not(target_os = "windows"))]
	/// Get the label. Return true for target foreground, false for background
	fn is_target_fg(&self) -> Result<bool> {
		unsafe { sys::cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_isTargetFg_const(self.as_raw_TrackerStateEstimatorMILBoosting_TrackerMILTargetState()) }.into_result()
	}
	
	#[cfg(not(target_os = "windows"))]
	/// Get the features extracted
	fn get_features(&self) -> Result<core::Mat> {
		unsafe { sys::cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_getFeatures_const(self.as_raw_TrackerStateEstimatorMILBoosting_TrackerMILTargetState()) }.into_result().map(|r| unsafe { core::Mat::opencv_from_extern(r) } )
	}
	
}

/// Implementation of the target state for TrackerStateEstimatorMILBoosting
pub struct TrackerStateEstimatorMILBoosting_TrackerMILTargetState {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerStateEstimatorMILBoosting_TrackerMILTargetState }

impl Drop for TrackerStateEstimatorMILBoosting_TrackerMILTargetState {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_delete(instance: *mut c_void); }
		unsafe { cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_delete(self.as_raw_mut_TrackerStateEstimatorMILBoosting_TrackerMILTargetState()) };
	}
}

impl TrackerStateEstimatorMILBoosting_TrackerMILTargetState {
	pub fn as_raw_TrackerStateEstimatorMILBoosting_TrackerMILTargetState(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerStateEstimatorMILBoosting_TrackerMILTargetState(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerStateEstimatorMILBoosting_TrackerMILTargetState {}

impl crate::tracking::TrackerStateEstimatorMILBoosting_TrackerMILTargetStateTrait for TrackerStateEstimatorMILBoosting_TrackerMILTargetState {
	fn as_raw_TrackerStateEstimatorMILBoosting_TrackerMILTargetState(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerStateEstimatorMILBoosting_TrackerMILTargetState(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::tracking::TrackerTargetStateTrait for TrackerStateEstimatorMILBoosting_TrackerMILTargetState {
	fn as_raw_TrackerTargetState(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerTargetState(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerStateEstimatorMILBoosting_TrackerMILTargetState {
	#[cfg(not(target_os = "windows"))]
	/// \brief Constructor
	/// \param position Top left corner of the bounding box
	/// \param width Width of the bounding box
	/// \param height Height of the bounding box
	/// \param foreground label for target or background
	/// \param features features extracted
	pub fn new(position: core::Point2f, width: i32, height: i32, foreground: bool, features: &core::Mat) -> Result<crate::tracking::TrackerStateEstimatorMILBoosting_TrackerMILTargetState> {
		unsafe { sys::cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_TrackerMILTargetState_const_Point2fX_int_int_bool_const_MatX(&position, width, height, foreground, features.as_raw_Mat()) }.into_result().map(|r| unsafe { crate::tracking::TrackerStateEstimatorMILBoosting_TrackerMILTargetState::opencv_from_extern(r) } )
	}
	
}

/// \brief TrackerStateEstimator based on SVM
pub trait TrackerStateEstimatorSVMTrait: crate::tracking::TrackerStateEstimator {
	fn as_raw_TrackerStateEstimatorSVM(&self) -> *const c_void;
	fn as_raw_mut_TrackerStateEstimatorSVM(&mut self) -> *mut c_void;

}

/// \brief TrackerStateEstimator based on SVM
pub struct TrackerStateEstimatorSVM {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerStateEstimatorSVM }

impl Drop for TrackerStateEstimatorSVM {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerStateEstimatorSVM_delete(instance: *mut c_void); }
		unsafe { cv_TrackerStateEstimatorSVM_delete(self.as_raw_mut_TrackerStateEstimatorSVM()) };
	}
}

impl TrackerStateEstimatorSVM {
	pub fn as_raw_TrackerStateEstimatorSVM(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerStateEstimatorSVM(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerStateEstimatorSVM {}

impl crate::tracking::TrackerStateEstimator for TrackerStateEstimatorSVM {
	fn as_raw_TrackerStateEstimator(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerStateEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::tracking::TrackerStateEstimatorSVMTrait for TrackerStateEstimatorSVM {
	fn as_raw_TrackerStateEstimatorSVM(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerStateEstimatorSVM(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerStateEstimatorSVM {
	pub fn default() -> Result<crate::tracking::TrackerStateEstimatorSVM> {
		unsafe { sys::cv_TrackerStateEstimatorSVM_TrackerStateEstimatorSVM() }.into_result().map(|r| unsafe { crate::tracking::TrackerStateEstimatorSVM::opencv_from_extern(r) } )
	}
	
}

/// TLD is a novel tracking framework that explicitly decomposes the long-term tracking task into
/// tracking, learning and detection.
/// 
/// The tracker follows the object from frame to frame. The detector localizes all appearances that
/// have been observed so far and corrects the tracker if necessary. The learning estimates detector’s
/// errors and updates it to avoid these errors in the future. The implementation is based on [TLD](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_TLD) .
/// 
/// The Median Flow algorithm (see cv::TrackerMedianFlow) was chosen as a tracking component in this
/// implementation, following authors. Tracker is supposed to be able to handle rapid motions, partial
/// occlusions, object absence etc.
pub trait TrackerTLD: crate::tracking::Tracker {
	fn as_raw_TrackerTLD(&self) -> *const c_void;
	fn as_raw_mut_TrackerTLD(&mut self) -> *mut c_void;

}

impl dyn TrackerTLD + '_ {
	/// ## C++ default parameters
	/// * parameters: classname::Params());virtual~classname(){};/*
	///  * Partially based on:
	///  * ====================================================================================================================
	///  *   - [AAM] S. Salti, A. Cavallaro, L. Di Stefano, Adaptive Appearance Modeling for Video Tracking: Survey and Evaluation
	///  *  - [AMVOT] X. Li, W. Hu, C. Shen, Z. Zhang, A. Dick, A. van den Hengel, A Survey of Appearance Models in Visual Object Tracking
	///  *
	///  * This Tracking API has been designed with PlantUML. If you modify this API please change UML files under modules/tracking/doc/uml
	///  *
	///  */namespacecv{//! @addtogroup tracking//! @{/************************************ TrackerFeature Base Classes ************************************//** @brief Abstract base class for TrackerFeature that represents the feature.
	///  */classCV_EXPORTSTrackerFeature{public:virtual~TrackerFeature();/** @brief Compute the features in the images collection
	///     @param images The images
	///     @param response The output response
	///      */voidcompute(conststd::vector<Mat>&images,Mat&response);/** @brief Create TrackerFeature by tracker feature type
	///     @param trackerFeatureType The TrackerFeature name
	/// 
	///     The modes available now:
	/// 
	///     -   "HAAR" -- Haar Feature-based
	/// 
	///     The modes that will be available soon:
	/// 
	///     -   "HOG" -- Histogram of Oriented Gradients features
	///     -   "LBP" -- Local Binary Pattern features
	///     -   "FEATURE2D" -- All types of Feature2D
	///      */staticPtr<TrackerFeature>create(constString&trackerFeatureType);/** @brief Identify most effective features
	///     @param response Collection of response for the specific TrackerFeature
	///     @param npoints Max number of features
	/// 
	///     @note This method modifies the response parameter
	///      */virtualvoidselection(Mat&response,intnpoints)=0;/** @brief Get the name of the specific TrackerFeature
	///      */StringgetClassName()const;protected:virtualboolcomputeImpl(conststd::vector<Mat>&images,Mat&response)=0;StringclassName;};/** @brief Class that manages the extraction and selection of features
	/// 
	/// @cite AAM Feature Extraction and Feature Set Refinement (Feature Processing and Feature Selection).
	/// See table I and section III C @cite AMVOT Appearance modelling -\> Visual representation (Table II,
	/// section 3.1 - 3.2)
	/// 
	/// TrackerFeatureSet is an aggregation of TrackerFeature
	/// 
	/// @sa
	///    TrackerFeature
	/// 
	///  */classCV_EXPORTSTrackerFeatureSet{public:TrackerFeatureSet();~TrackerFeatureSet();/** @brief Extract features from the images collection
	///     @param images The input images
	///      */voidextraction(conststd::vector<Mat>&images);/** @brief Identify most effective features for all feature types (optional)
	///      */voidselection();/** @brief Remove outliers for all feature types (optional)
	///      */voidremoveOutliers();/** @brief Add TrackerFeature in the collection. Return true if TrackerFeature is added, false otherwise
	///     @param trackerFeatureType The TrackerFeature name
	/// 
	///     The modes available now:
	/// 
	///     -   "HAAR" -- Haar Feature-based
	/// 
	///     The modes that will be available soon:
	/// 
	///     -   "HOG" -- Histogram of Oriented Gradients features
	///     -   "LBP" -- Local Binary Pattern features
	///     -   "FEATURE2D" -- All types of Feature2D
	/// 
	///     Example TrackerFeatureSet::addTrackerFeature : :
	///     @code
	///         //sample usage:
	/// 
	///         Ptr<TrackerFeature> trackerFeature = new TrackerFeatureHAAR( HAARparameters );
	///         featureSet->addTrackerFeature( trackerFeature );
	/// 
	///         //or add CSC sampler with default parameters
	///         //featureSet->addTrackerFeature( "HAAR" );
	///     @endcode
	///     @note If you use the second method, you must initialize the TrackerFeature
	///      */booladdTrackerFeature(StringtrackerFeatureType);/** @overload
	///     @param feature The TrackerFeature class
	///     */booladdTrackerFeature(Ptr<TrackerFeature>&feature);/** @brief Get the TrackerFeature collection (TrackerFeature name, TrackerFeature pointer)
	///      */conststd::vector<std::pair<String,Ptr<TrackerFeature>>>&getTrackerFeature()const;/** @brief Get the responses
	/// 
	///     @note Be sure to call extraction before getResponses Example TrackerFeatureSet::getResponses : :
	///      */conststd::vector<Mat>&getResponses()const;private:voidclearResponses();boolblockAddTrackerFeature;std::vector<std::pair<String,Ptr<TrackerFeature>>>features;//list of featuresstd::vector<Mat>responses;//list of response after compute};/************************************ TrackerSampler Base Classes ************************************//** @brief Abstract base class for TrackerSamplerAlgorithm that represents the algorithm for the specific
	/// sampler.
	///  */classCV_EXPORTSTrackerSamplerAlgorithm{public:/**
	///    * \brief Destructor
	///    */virtual~TrackerSamplerAlgorithm();/** @brief Create TrackerSamplerAlgorithm by tracker sampler type.
	///     @param trackerSamplerType The trackerSamplerType name
	/// 
	///     The modes available now:
	/// 
	///     -   "CSC" -- Current State Center
	///     -   "CS" -- Current State
	///      */staticPtr<TrackerSamplerAlgorithm>create(constString&trackerSamplerType);/** @brief Computes the regions starting from a position in an image.
	/// 
	///     Return true if samples are computed, false otherwise
	/// 
	///     @param image The current frame
	///     @param boundingBox The bounding box from which regions can be calculated
	/// 
	///     @param sample The computed samples @cite AAM Fig. 1 variable Sk
	///      */boolsampling(constMat&image,RectboundingBox,std::vector<Mat>&sample);/** @brief Get the name of the specific TrackerSamplerAlgorithm
	///     */StringgetClassName()const;protected:StringclassName;virtualboolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample)=0;};/**
	///  * \brief Class that manages the sampler in order to select regions for the update the model of the tracker
	///  * [AAM] Sampling e Labeling. See table I and section III B
	///  *//** @brief Class that manages the sampler in order to select regions for the update the model of the tracker
	/// 
	/// @cite AAM Sampling e Labeling. See table I and section III B
	/// 
	/// TrackerSampler is an aggregation of TrackerSamplerAlgorithm
	/// @sa
	///    TrackerSamplerAlgorithm
	///  */classCV_EXPORTSTrackerSampler{public:/**
	///    * \brief Constructor
	///    */TrackerSampler();/**
	///    * \brief Destructor
	///    */~TrackerSampler();/** @brief Computes the regions starting from a position in an image
	///     @param image The current frame
	///     @param boundingBox The bounding box from which regions can be calculated
	///      */voidsampling(constMat&image,RectboundingBox);/** @brief Return the collection of the TrackerSamplerAlgorithm
	///     */conststd::vector<std::pair<String,Ptr<TrackerSamplerAlgorithm>>>&getSamplers()const;/** @brief Return the samples from all TrackerSamplerAlgorithm, @cite AAM Fig. 1 variable Sk
	///     */conststd::vector<Mat>&getSamples()const;/** @brief Add TrackerSamplerAlgorithm in the collection. Return true if sampler is added, false otherwise
	///     @param trackerSamplerAlgorithmType The TrackerSamplerAlgorithm name
	/// 
	///     The modes available now:
	///     -   "CSC" -- Current State Center
	///     -   "CS" -- Current State
	///     -   "PF" -- Particle Filtering
	/// 
	///     Example TrackerSamplerAlgorithm::addTrackerSamplerAlgorithm : :
	///     @code
	///          TrackerSamplerCSC::Params CSCparameters;
	///          Ptr<TrackerSamplerAlgorithm> CSCSampler = new TrackerSamplerCSC( CSCparameters );
	/// 
	///          if( !sampler->addTrackerSamplerAlgorithm( CSCSampler ) )
	///            return false;
	/// 
	///          //or add CSC sampler with default parameters
	///          //sampler->addTrackerSamplerAlgorithm( "CSC" );
	///     @endcode
	///     @note If you use the second method, you must initialize the TrackerSamplerAlgorithm
	///      */booladdTrackerSamplerAlgorithm(StringtrackerSamplerAlgorithmType);/** @overload
	///     @param sampler The TrackerSamplerAlgorithm
	///     */booladdTrackerSamplerAlgorithm(Ptr<TrackerSamplerAlgorithm>&sampler);private:std::vector<std::pair<String,Ptr<TrackerSamplerAlgorithm>>>samplers;std::vector<Mat>samples;boolblockAddTrackerSampler;voidclearSamples();};/************************************ TrackerModel Base Classes ************************************//** @brief Abstract base class for TrackerTargetState that represents a possible state of the target.
	/// 
	/// See @cite AAM \f$\hat{x}^{i}_{k}\f$ all the states candidates.
	/// 
	/// Inherits this class with your Target state, In own implementation you can add scale variation,
	/// width, height, orientation, etc.
	///  */classCV_EXPORTSTrackerTargetState{public:virtual~TrackerTargetState(){};/**
	///    * \brief Get the position
	///    * \return The position
	///    */Point2fgetTargetPosition()const;/**
	///    * \brief Set the position
	///    * \param position The position
	///    */voidsetTargetPosition(constPoint2f&position);/**
	///    * \brief Get the width of the target
	///    * \return The width of the target
	///    */intgetTargetWidth()const;/**
	///    * \brief Set the width of the target
	///    * \param width The width of the target
	///    */voidsetTargetWidth(intwidth);/**
	///    * \brief Get the height of the target
	///    * \return The height of the target
	///    */intgetTargetHeight()const;/**
	///    * \brief Set the height of the target
	///    * \param height The height of the target
	///    */voidsetTargetHeight(intheight);protected:Point2ftargetPosition;inttargetWidth;inttargetHeight;};/** @brief Represents the model of the target at frame \f$k\f$ (all states and scores)
	/// 
	/// See @cite AAM The set of the pair \f$\langle \hat{x}^{i}_{k}, C^{i}_{k} \rangle\f$
	/// @sa TrackerTargetState
	///  */typedefstd::vector<std::pair<Ptr<TrackerTargetState>,float>>ConfidenceMap;/** @brief Represents the estimate states for all frames
	/// 
	/// @cite AAM \f$x_{k}\f$ is the trajectory of the target up to time \f$k\f$
	/// 
	/// @sa TrackerTargetState
	///  */typedefstd::vector<Ptr<TrackerTargetState>>Trajectory;/** @brief Abstract base class for TrackerStateEstimator that estimates the most likely target state.
	/// 
	/// See @cite AAM State estimator
	/// 
	/// See @cite AMVOT Statistical modeling (Fig. 3), Table III (generative) - IV (discriminative) - V (hybrid)
	///  */classCV_EXPORTSTrackerStateEstimator{public:virtual~TrackerStateEstimator();/** @brief Estimate the most likely target state, return the estimated state
	///     @param confidenceMaps The overall appearance model as a list of :cConfidenceMap
	///      */Ptr<TrackerTargetState>estimate(conststd::vector<ConfidenceMap>&confidenceMaps);/** @brief Update the ConfidenceMap with the scores
	///     @param confidenceMaps The overall appearance model as a list of :cConfidenceMap
	///      */voidupdate(std::vector<ConfidenceMap>&confidenceMaps);/** @brief Create TrackerStateEstimator by tracker state estimator type
	///     @param trackeStateEstimatorType The TrackerStateEstimator name
	/// 
	///     The modes available now:
	/// 
	///     -   "BOOSTING" -- Boosting-based discriminative appearance models. See @cite AMVOT section 4.4
	/// 
	///     The modes available soon:
	/// 
	///     -   "SVM" -- SVM-based discriminative appearance models. See @cite AMVOT section 4.5
	///      */staticPtr<TrackerStateEstimator>create(constString&trackeStateEstimatorType);/** @brief Get the name of the specific TrackerStateEstimator
	///      */StringgetClassName()const;protected:virtualPtr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps)=0;virtualvoidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps)=0;StringclassName;};/** @brief Abstract class that represents the model of the target. It must be instantiated by specialized
	/// tracker
	/// 
	/// See @cite AAM Ak
	/// 
	/// Inherits this with your TrackerModel
	///  */classCV_EXPORTSTrackerModel{public:/**
	///    * \brief Constructor
	///    */TrackerModel();/**
	///    * \brief Destructor
	///    */virtual~TrackerModel();/** @brief Set TrackerEstimator, return true if the tracker state estimator is added, false otherwise
	///     @param trackerStateEstimator The TrackerStateEstimator
	///     @note You can add only one TrackerStateEstimator
	///      */boolsetTrackerStateEstimator(Ptr<TrackerStateEstimator>trackerStateEstimator);/** @brief Estimate the most likely target location
	/// 
	///     @cite AAM ME, Model Estimation table I
	///     @param responses Features extracted from TrackerFeatureSet
	///      */voidmodelEstimation(conststd::vector<Mat>&responses);/** @brief Update the model
	/// 
	///     @cite AAM MU, Model Update table I
	///      */voidmodelUpdate();/** @brief Run the TrackerStateEstimator, return true if is possible to estimate a new state, false otherwise
	///     */boolrunStateEstimator();/** @brief Set the current TrackerTargetState in the Trajectory
	///     @param lastTargetState The current TrackerTargetState
	///      */voidsetLastTargetState(constPtr<TrackerTargetState>&lastTargetState);/** @brief Get the last TrackerTargetState from Trajectory
	///     */Ptr<TrackerTargetState>getLastTargetState()const;/** @brief Get the list of the ConfidenceMap
	///     */conststd::vector<ConfidenceMap>&getConfidenceMaps()const;/** @brief Get the last ConfidenceMap for the current frame
	///      */constConfidenceMap&getLastConfidenceMap()const;/** @brief Get the TrackerStateEstimator
	///     */Ptr<TrackerStateEstimator>getTrackerStateEstimator()const;private:voidclearCurrentConfidenceMap();protected:std::vector<ConfidenceMap>confidenceMaps;Ptr<TrackerStateEstimator>stateEstimator;ConfidenceMapcurrentConfidenceMap;Trajectorytrajectory;intmaxCMLength;virtualvoidmodelEstimationImpl(conststd::vector<Mat>&responses)=0;virtualvoidmodelUpdateImpl()=0;};/************************************ Tracker Base Class ************************************//** @brief Base abstract class for the long-term tracker:
	///  */classCV_EXPORTS_WTracker:publicvirtualAlgorithm{public:virtual~Tracker();/** @brief Initialize the tracker with a know bounding box that surrounding the target
	///     @param image The initial frame
	///     @param boundingBox The initial boundig box
	/// 
	///     @return True if initialization went succesfully, false otherwise
	///      */CV_WRAPboolinit(constMat&image,constRect2d&boundingBox);/** @brief Update the tracker, find the new most likely bounding box for the target
	///     @param image The current frame
	///     @param boundingBox The boundig box that represent the new target location, if true was returned, not
	///     modified otherwise
	/// 
	///     @return True means that target was located and false means that tracker cannot locate target in
	///     current frame. Note, that latter *does not* imply that tracker has failed, maybe target is indeed
	///     missing from the frame (say, out of sight)
	///      */CV_WRAPboolupdate(constMat&image,CV_OUTRect2d&boundingBox);/** @brief Creates a tracker by its name.
	///     @param trackerType Tracker type
	/// 
	///     The following detector types are supported:
	/// 
	///     -   "MIL" -- TrackerMIL
	///     -   "BOOSTING" -- TrackerBoosting
	///      */CV_WRAPstaticPtr<Tracker>create(constString&trackerType);virtualvoidread(constFileNode&fn)=0;virtualvoidwrite(FileStorage&fs)const=0;Ptr<TrackerModel>getModel(){returnmodel;}protected:virtualboolinitImpl(constMat&image,constRect2d&boundingBox)=0;virtualboolupdateImpl(constMat&image,Rect2d&boundingBox)=0;boolisInit;Ptr<TrackerFeatureSet>featureSet;Ptr<TrackerSampler>sampler;Ptr<TrackerModel>model;};/************************************ Specific TrackerStateEstimator Classes ************************************//** @brief TrackerStateEstimator based on Boosting
	///     */classCV_EXPORTSTrackerStateEstimatorMILBoosting:publicTrackerStateEstimator{public:/**
	///    * Implementation of the target state for TrackerStateEstimatorMILBoosting
	///    */classTrackerMILTargetState:publicTrackerTargetState{public:/**
	///      * \brief Constructor
	///      * \param position Top left corner of the bounding box
	///      * \param width Width of the bounding box
	///      * \param height Height of the bounding box
	///      * \param foreground label for target or background
	///      * \param features features extracted
	///      */TrackerMILTargetState(constPoint2f&position,intwidth,intheight,boolforeground,constMat&features);/**
	///      * \brief Destructor
	///      */~TrackerMILTargetState(){};/** @brief Set label: true for target foreground, false for background
	///     @param foreground Label for background/foreground
	///      */voidsetTargetFg(boolforeground);/** @brief Set the features extracted from TrackerFeatureSet
	///     @param features The features extracted
	///      */voidsetFeatures(constMat&features);/** @brief Get the label. Return true for target foreground, false for background
	///      */boolisTargetFg()const;/** @brief Get the features extracted
	///      */MatgetFeatures()const;private:boolisTarget;MattargetFeatures;};/** @brief Constructor
	///     @param nFeatures Number of features for each sample
	///      */TrackerStateEstimatorMILBoosting(intnFeatures=250);~TrackerStateEstimatorMILBoosting();/** @brief Set the current confidenceMap
	///     @param confidenceMap The current :cConfidenceMap
	///      */voidsetCurrentConfidenceMap(ConfidenceMap&confidenceMap);protected:Ptr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps);voidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps);private:uintmax_idx(conststd::vector<float>&v);voidprepareData(constConfidenceMap&confidenceMap,Mat&positive,Mat&negative);ClfMilBoostboostMILModel;booltrained;intnumFeatures;ConfidenceMapcurrentConfidenceMap;};/** @brief TrackerStateEstimatorAdaBoosting based on ADA-Boosting
	///  */classCV_EXPORTSTrackerStateEstimatorAdaBoosting:publicTrackerStateEstimator{public:/** @brief Implementation of the target state for TrackerAdaBoostingTargetState
	///     */classTrackerAdaBoostingTargetState:publicTrackerTargetState{public:/**
	///      * \brief Constructor
	///      * \param position Top left corner of the bounding box
	///      * \param width Width of the bounding box
	///      * \param height Height of the bounding box
	///      * \param foreground label for target or background
	///      * \param responses list of features
	///      */TrackerAdaBoostingTargetState(constPoint2f&position,intwidth,intheight,boolforeground,constMat&responses);/**
	///      * \brief Destructor
	///      */~TrackerAdaBoostingTargetState(){};/** @brief Set the features extracted from TrackerFeatureSet
	///     @param responses The features extracted
	///      */voidsetTargetResponses(constMat&responses);/** @brief Set label: true for target foreground, false for background
	///     @param foreground Label for background/foreground
	///      */voidsetTargetFg(boolforeground);/** @brief Get the features extracted
	///      */MatgetTargetResponses()const;/** @brief Get the label. Return true for target foreground, false for background
	///     */boolisTargetFg()const;private:boolisTarget;MattargetResponses;};/** @brief Constructor
	///     @param numClassifer Number of base classifiers
	///     @param initIterations Number of iterations in the initialization
	///     @param nFeatures Number of features/weak classifiers
	///     @param patchSize tracking rect
	///     @param ROI initial ROI
	///      */TrackerStateEstimatorAdaBoosting(intnumClassifer,intinitIterations,intnFeatures,SizepatchSize,constRect&ROI);/**
	///    * \brief Destructor
	///    */~TrackerStateEstimatorAdaBoosting();/** @brief Get the sampling ROI
	///      */RectgetSampleROI()const;/** @brief Set the sampling ROI
	///     @param ROI the sampling ROI
	///      */voidsetSampleROI(constRect&ROI);/** @brief Set the current confidenceMap
	///     @param confidenceMap The current :cConfidenceMap
	///      */voidsetCurrentConfidenceMap(ConfidenceMap&confidenceMap);/** @brief Get the list of the selected weak classifiers for the classification step
	///      */std::vector<int>computeSelectedWeakClassifier();/** @brief Get the list of the weak classifiers that should be replaced
	///      */std::vector<int>computeReplacedClassifier();/** @brief Get the list of the weak classifiers that replace those to be replaced
	///      */std::vector<int>computeSwappedClassifier();protected:Ptr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps);voidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps);Ptr<StrongClassifierDirectSelection>boostClassifier;private:intnumBaseClassifier;intiterationInit;intnumFeatures;booltrained;SizeinitPatchSize;RectsampleROI;std::vector<int>replacedClassifier;std::vector<int>swappedClassifier;ConfidenceMapcurrentConfidenceMap;};/**
	///  * \brief TrackerStateEstimator based on SVM
	///  */classCV_EXPORTSTrackerStateEstimatorSVM:publicTrackerStateEstimator{public:TrackerStateEstimatorSVM();~TrackerStateEstimatorSVM();protected:Ptr<TrackerTargetState>estimateImpl(conststd::vector<ConfidenceMap>&confidenceMaps);voidupdateImpl(std::vector<ConfidenceMap>&confidenceMaps);};/************************************ Specific TrackerSamplerAlgorithm Classes ************************************//** @brief TrackerSampler based on CSC (current state centered), used by MIL algorithm TrackerMIL
	///  */classCV_EXPORTSTrackerSamplerCSC:publicTrackerSamplerAlgorithm{public:enum{MODE_INIT_POS=1,//!< mode for init positive samplesMODE_INIT_NEG=2,//!< mode for init negative samplesMODE_TRACK_POS=3,//!< mode for update positive samplesMODE_TRACK_NEG=4,//!< mode for update negative samplesMODE_DETECT=5//!< mode for detect samples};structCV_EXPORTSParams{Params();floatinitInRad;//!< radius for gathering positive instances during initfloattrackInPosRad;//!< radius for gathering positive instances during trackingfloatsearchWinSize;//!< size of search windowintinitMaxNegNum;//!< # negative samples to use during initinttrackMaxPosNum;//!< # positive samples to use during traininginttrackMaxNegNum;//!< # negative samples to use during training};/** @brief Constructor
	///     @param parameters TrackerSamplerCSC parameters TrackerSamplerCSC::Params
	///      */TrackerSamplerCSC(constTrackerSamplerCSC::Params&parameters=TrackerSamplerCSC::Params());/** @brief Set the sampling mode of TrackerSamplerCSC
	///     @param samplingMode The sampling mode
	/// 
	///     The modes are:
	/// 
	///     -   "MODE_INIT_POS = 1" -- for the positive sampling in initialization step
	///     -   "MODE_INIT_NEG = 2" -- for the negative sampling in initialization step
	///     -   "MODE_TRACK_POS = 3" -- for the positive sampling in update step
	///     -   "MODE_TRACK_NEG = 4" -- for the negative sampling in update step
	///     -   "MODE_DETECT = 5" -- for the sampling in detection step
	///      */voidsetMode(intsamplingMode);~TrackerSamplerCSC();protected:boolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample);private:Paramsparams;intmode;RNGrng;std::vector<Mat>sampleImage(constMat&img,intx,inty,intw,inth,floatinrad,floatoutrad=0,intmaxnum=1000000);};/** @brief TrackerSampler based on CS (current state), used by algorithm TrackerBoosting
	///  */classCV_EXPORTSTrackerSamplerCS:publicTrackerSamplerAlgorithm{public:enum{MODE_POSITIVE=1,//!< mode for positive samplesMODE_NEGATIVE=2,//!< mode for negative samplesMODE_CLASSIFY=3//!< mode for classify samples};structCV_EXPORTSParams{Params();floatoverlap;//!<overlapping for the search windowsfloatsearchFactor;//!<search region parameter};/** @brief Constructor
	///     @param parameters TrackerSamplerCS parameters TrackerSamplerCS::Params
	///      */TrackerSamplerCS(constTrackerSamplerCS::Params&parameters=TrackerSamplerCS::Params());/** @brief Set the sampling mode of TrackerSamplerCS
	///     @param samplingMode The sampling mode
	/// 
	///     The modes are:
	/// 
	///     -   "MODE_POSITIVE = 1" -- for the positive sampling
	///     -   "MODE_NEGATIVE = 2" -- for the negative sampling
	///     -   "MODE_CLASSIFY = 3" -- for the sampling in classification step
	///      */voidsetMode(intsamplingMode);~TrackerSamplerCS();boolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample);RectgetROI()const;private:RectgetTrackingROI(floatsearchFactor);RectRectMultiply(constRect&rect,floatf);std::vector<Mat>patchesRegularScan(constMat&image,RecttrackingROI,SizepatchSize);voidsetCheckedROI(RectimageROI);Paramsparams;intmode;RecttrackedPatch;RectvalidROI;RectROI;};/** @brief This sampler is based on particle filtering.
	/// 
	/// In principle, it can be thought of as performing some sort of optimization (and indeed, this
	/// tracker uses opencv's optim module), where tracker seeks to find the rectangle in given frame,
	/// which is the most *"similar"* to the initial rectangle (the one, given through the constructor).
	/// 
	/// The optimization performed is stochastic and somehow resembles genetic algorithms, where on each new
	/// image received (submitted via TrackerSamplerPF::sampling()) we start with the region bounded by
	/// boundingBox, then generate several "perturbed" boxes, take the ones most similar to the original.
	/// This selection round is repeated several times. At the end, we hope that only the most promising box
	/// remaining, and these are combined to produce the subrectangle of image, which is put as a sole
	/// element in array sample.
	/// 
	/// It should be noted, that the definition of "similarity" between two rectangles is based on comparing
	/// their histograms. As experiments show, tracker is *not* very succesfull if target is assumed to
	/// strongly change its dimensions.
	///  */classCV_EXPORTSTrackerSamplerPF:publicTrackerSamplerAlgorithm{public:/** @brief This structure contains all the parameters that can be varied during the course of sampling
	///     algorithm. Below is the structure exposed, together with its members briefly explained with
	///     reference to the above discussion on algorithm's working.
	///  */structCV_EXPORTSParams{Params();intiterationNum;//!< number of selection roundsintparticlesNum;//!< number of "perturbed" boxes on each rounddoublealpha;//!< with each new round we exponentially decrease the amount of "perturbing" we allow (like in simulated annealing)//!< and this very alpha controls how fast annealing happens, ie. how fast perturbing decreasesMat_<double>std;//!< initial values for perturbing (1-by-4 array, as each rectangle is given by 4 values -- coordinates of opposite vertices,//!< hence we have 4 values to perturb)};/** @brief Constructor
	///     @param chosenRect Initial rectangle, that is supposed to contain target we'd like to track.
	///     @param parameters
	///      */TrackerSamplerPF(constMat&chosenRect,constTrackerSamplerPF::Params&parameters=TrackerSamplerPF::Params());protected:boolsamplingImpl(constMat&image,RectboundingBox,std::vector<Mat>&sample);private:Paramsparams;Ptr<MinProblemSolver>_solver;Ptr<MinProblemSolver::Function>_function;};/************************************ Specific TrackerFeature Classes ************************************//**
	///  * \brief TrackerFeature based on Feature2D
	///  */classCV_EXPORTSTrackerFeatureFeature2d:publicTrackerFeature{public:/**
	///    * \brief Constructor
	///    * \param detectorType string of FeatureDetector
	///    * \param descriptorType string of DescriptorExtractor
	///    */TrackerFeatureFeature2d(StringdetectorType,StringdescriptorType);~TrackerFeatureFeature2d();voidselection(Mat&response,intnpoints);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);private:std::vector<KeyPoint>keypoints;};/**
	///  * \brief TrackerFeature based on HOG
	///  */classCV_EXPORTSTrackerFeatureHOG:publicTrackerFeature{public:TrackerFeatureHOG();~TrackerFeatureHOG();voidselection(Mat&response,intnpoints);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);};/** @brief TrackerFeature based on HAAR features, used by TrackerMIL and many others algorithms
	/// @note HAAR features implementation is copied from apps/traincascade and modified according to MIL
	///  */classCV_EXPORTSTrackerFeatureHAAR:publicTrackerFeature{public:structCV_EXPORTSParams{Params();intnumFeatures;//!< # of rectsSizerectSize;//!< rect sizeboolisIntegral;//!< true if input images are integral, false otherwise};/** @brief Constructor
	///     @param parameters TrackerFeatureHAAR parameters TrackerFeatureHAAR::Params
	///      */TrackerFeatureHAAR(constTrackerFeatureHAAR::Params&parameters=TrackerFeatureHAAR::Params());~TrackerFeatureHAAR();/** @brief Compute the features only for the selected indices in the images collection
	///     @param selFeatures indices of selected features
	///     @param images The images
	///     @param response Collection of response for the specific TrackerFeature
	///      */boolextractSelected(conststd::vector<int>selFeatures,conststd::vector<Mat>&images,Mat&response);/** @brief Identify most effective features
	///     @param response Collection of response for the specific TrackerFeature
	///     @param npoints Max number of features
	/// 
	///     @note This method modifies the response parameter
	///      */voidselection(Mat&response,intnpoints);/** @brief Swap the feature in position source with the feature in position target
	///   @param source The source position
	///   @param target The target position
	///  */boolswapFeature(intsource,inttarget);/** @brief   Swap the feature in position id with the feature input
	///   @param id The position
	///   @param feature The feature
	///  */boolswapFeature(intid,CvHaarEvaluator::FeatureHaar&feature);/** @brief Get the feature in position id
	///     @param id The position
	///      */CvHaarEvaluator::FeatureHaar&getFeatureAt(intid);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);private:Paramsparams;Ptr<CvHaarEvaluator>featureEvaluator;};/**
	///  * \brief TrackerFeature based on LBP
	///  */classCV_EXPORTSTrackerFeatureLBP:publicTrackerFeature{public:TrackerFeatureLBP();~TrackerFeatureLBP();voidselection(Mat&response,intnpoints);protected:boolcomputeImpl(conststd::vector<Mat>&images,Mat&response);};/************************************ Specific Tracker Classes ************************************//** @brief The MIL algorithm trains a classifier in an online manner to separate the object from the
	/// background.
	/// 
	/// Multiple Instance Learning avoids the drift problem for a robust tracking. The implementation is
	/// based on @cite MIL .
	/// 
	/// Original code can be found here <http://vision.ucsd.edu/~bbabenko/project_miltrack.shtml>
	///  */classCV_EXPORTSTrackerMIL:publicTracker{public:structCV_EXPORTSParams{Params();//parameters for samplerfloatsamplerInitInRadius;//!< radius for gathering positive instances during initintsamplerInitMaxNegNum;//!< # negative samples to use during initfloatsamplerSearchWinSize;//!< size of search windowfloatsamplerTrackInRadius;//!< radius for gathering positive instances during trackingintsamplerTrackMaxPosNum;//!< # positive samples to use during trackingintsamplerTrackMaxNegNum;//!< # negative samples to use during trackingintfeatureSetNumFeatures;//!< # featuresvoidread(constFileNode&fn);voidwrite(FileStorage&fs)const;};/** @brief Constructor
	///     @param parameters MIL parameters TrackerMIL::Params
	///      */BOILERPLATE_CODE("MIL",TrackerMIL);};/** @brief This is a real-time object tracking based on a novel on-line version of the AdaBoost algorithm.
	/// 
	/// The classifier uses the surrounding background as negative examples in update step to avoid the
	/// drifting problem. The implementation is based on @cite OLB .
	///  */classCV_EXPORTSTrackerBoosting:publicTracker{public:structCV_EXPORTSParams{Params();intnumClassifiers;//!<the number of classifiers to use in a OnlineBoosting algorithmfloatsamplerOverlap;//!<search region parameters to use in a OnlineBoosting algorithmfloatsamplerSearchFactor;//!< search region parameters to use in a OnlineBoosting algorithmintiterationInit;//!<the initial iterationsintfeatureSetNumFeatures;//!< # features/**
	///      * \brief Read parameters from file
	///      */voidread(constFileNode&fn);/**
	///      * \brief Write parameters in a file
	///      */voidwrite(FileStorage&fs)const;};/** @brief Constructor
	///     @param parameters BOOSTING parameters TrackerBoosting::Params
	///      */BOILERPLATE_CODE("BOOSTING",TrackerBoosting);};/** @brief Median Flow tracker implementation.
	/// 
	/// Implementation of a paper @cite MedianFlow .
	/// 
	/// The tracker is suitable for very smooth and predictable movements when object is visible throughout
	/// the whole sequence. It's quite and accurate for this type of problems (in particular, it was shown
	/// by authors to outperform MIL). During the implementation period the code at
	/// <http://www.aonsquared.co.uk/node/5>, the courtesy of the author Arthur Amarra, was used for the
	/// reference purpose.
	///  */classCV_EXPORTSTrackerMedianFlow:publicTracker{public:structCV_EXPORTSParams{Params();intpointsInGrid;//!<square root of number of keypoints used; increase it to trade//!<accurateness for speed; default value is sensible and recommendedvoidread(constFileNode&/*fn*/);voidwrite(FileStorage&/*fs*/)const;};/** @brief Constructor
	///     @param parameters Median Flow parameters TrackerMedianFlow::Params
	///     */BOILERPLATE_CODE("MEDIANFLOW",TrackerMedianFlow);};/** @brief TLD is a novel tracking framework that explicitly decomposes the long-term tracking task into
	/// tracking, learning and detection.
	/// 
	/// The tracker follows the object from frame to frame. The detector localizes all appearances that
	/// have been observed so far and corrects the tracker if necessary. The learning estimates detector’s
	/// errors and updates it to avoid these errors in the future. The implementation is based on @cite TLD .
	/// 
	/// The Median Flow algorithm (see cv::TrackerMedianFlow) was chosen as a tracking component in this
	/// implementation, following authors. Tracker is supposed to be able to handle rapid motions, partial
	/// occlusions, object absence etc.
	///  */classCV_EXPORTSTrackerTLD:publicTracker{public:structCV_EXPORTSParams{Params();voidread(constFileNode&/*fn*/);voidwrite(FileStorage&/*fs*/)const;};/** @brief Constructor
	///     @param parameters TLD parameters TrackerTLD::Params
	///      */BOILERPLATE_CODE("TLD",TrackerTLD)
	pub fn create_tracker(parameters: &crate::tracking::TrackerTLD_Params) -> Result<core::Ptr::<dyn crate::tracking::TrackerTLD>> {
		unsafe { sys::cv_TrackerTLD_createTracker_const_ParamsX(parameters.as_raw_TrackerTLD_Params()) }.into_result().map(|r| unsafe { core::Ptr::<dyn crate::tracking::TrackerTLD>::opencv_from_extern(r) } )
	}
	
}
pub trait TrackerTLD_ParamsTrait {
	fn as_raw_TrackerTLD_Params(&self) -> *const c_void;
	fn as_raw_mut_TrackerTLD_Params(&mut self) -> *mut c_void;

	fn read(&mut self, unnamed: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_TrackerTLD_Params_read_const_FileNodeX(self.as_raw_mut_TrackerTLD_Params(), unnamed.as_raw_FileNode()) }.into_result()
	}
	
	fn write(&self, unnamed: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_TrackerTLD_Params_write_const_FileStorageX(self.as_raw_TrackerTLD_Params(), unnamed.as_raw_mut_FileStorage()) }.into_result()
	}
	
}

pub struct TrackerTLD_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerTLD_Params }

impl Drop for TrackerTLD_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerTLD_Params_delete(instance: *mut c_void); }
		unsafe { cv_TrackerTLD_Params_delete(self.as_raw_mut_TrackerTLD_Params()) };
	}
}

impl TrackerTLD_Params {
	pub fn as_raw_TrackerTLD_Params(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerTLD_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerTLD_Params {}

impl crate::tracking::TrackerTLD_ParamsTrait for TrackerTLD_Params {
	fn as_raw_TrackerTLD_Params(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerTLD_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerTLD_Params {
	pub fn default() -> Result<crate::tracking::TrackerTLD_Params> {
		unsafe { sys::cv_TrackerTLD_Params_Params() }.into_result().map(|r| unsafe { crate::tracking::TrackerTLD_Params::opencv_from_extern(r) } )
	}
	
}

/// Abstract base class for TrackerTargetState that represents a possible state of the target.
/// 
/// See [AAM](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_AAM) ![inline formula](https://latex.codecogs.com/png.latex?%5Chat%7Bx%7D%5E%7Bi%7D%5F%7Bk%7D) all the states candidates.
/// 
/// Inherits this class with your Target state, In own implementation you can add scale variation,
/// width, height, orientation, etc.
pub trait TrackerTargetStateTrait {
	fn as_raw_TrackerTargetState(&self) -> *const c_void;
	fn as_raw_mut_TrackerTargetState(&mut self) -> *mut c_void;

	/// \brief Get the position
	/// \return The position
	fn get_target_position(&self) -> Result<core::Point2f> {
		unsafe { sys::cv_TrackerTargetState_getTargetPosition_const(self.as_raw_TrackerTargetState()) }.into_result()
	}
	
	/// \brief Set the position
	/// \param position The position
	fn set_target_position(&mut self, position: core::Point2f) -> Result<()> {
		unsafe { sys::cv_TrackerTargetState_setTargetPosition_const_Point2fX(self.as_raw_mut_TrackerTargetState(), &position) }.into_result()
	}
	
	/// \brief Get the width of the target
	/// \return The width of the target
	fn get_target_width(&self) -> Result<i32> {
		unsafe { sys::cv_TrackerTargetState_getTargetWidth_const(self.as_raw_TrackerTargetState()) }.into_result()
	}
	
	/// \brief Set the width of the target
	/// \param width The width of the target
	fn set_target_width(&mut self, width: i32) -> Result<()> {
		unsafe { sys::cv_TrackerTargetState_setTargetWidth_int(self.as_raw_mut_TrackerTargetState(), width) }.into_result()
	}
	
	/// \brief Get the height of the target
	/// \return The height of the target
	fn get_target_height(&self) -> Result<i32> {
		unsafe { sys::cv_TrackerTargetState_getTargetHeight_const(self.as_raw_TrackerTargetState()) }.into_result()
	}
	
	/// \brief Set the height of the target
	/// \param height The height of the target
	fn set_target_height(&mut self, height: i32) -> Result<()> {
		unsafe { sys::cv_TrackerTargetState_setTargetHeight_int(self.as_raw_mut_TrackerTargetState(), height) }.into_result()
	}
	
}

/// Abstract base class for TrackerTargetState that represents a possible state of the target.
/// 
/// See [AAM](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_AAM) ![inline formula](https://latex.codecogs.com/png.latex?%5Chat%7Bx%7D%5E%7Bi%7D%5F%7Bk%7D) all the states candidates.
/// 
/// Inherits this class with your Target state, In own implementation you can add scale variation,
/// width, height, orientation, etc.
pub struct TrackerTargetState {
	ptr: *mut c_void
}

opencv_type_boxed! { TrackerTargetState }

impl Drop for TrackerTargetState {
	fn drop(&mut self) {
		extern "C" { fn cv_TrackerTargetState_delete(instance: *mut c_void); }
		unsafe { cv_TrackerTargetState_delete(self.as_raw_mut_TrackerTargetState()) };
	}
}

impl TrackerTargetState {
	pub fn as_raw_TrackerTargetState(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_TrackerTargetState(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for TrackerTargetState {}

impl crate::tracking::TrackerTargetStateTrait for TrackerTargetState {
	fn as_raw_TrackerTargetState(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_TrackerTargetState(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TrackerTargetState {
}
