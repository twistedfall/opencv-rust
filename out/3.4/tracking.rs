//! # Tracking API
//!
//! Long-term optical tracking API
//! ------------------------------
//!
//! Long-term optical tracking is an important issue for many computer vision applications in
//! real world scenario. The development in this area is very fragmented and this API is an unique
//! interface useful for plug several algorithms and compare them. This work is partially based on
//! [AAM](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_AAM) and [AMVOT](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_AMVOT) .
//!
//! These algorithms start from a bounding box of the target and with their internal representation they
//! avoid the drift during the tracking. These long-term trackers are able to evaluate online the
//! quality of the location of the target in the new frame, without ground truth.
//!
//! There are three main components: the TrackerSampler, the TrackerFeatureSet and the TrackerModel. The
//! first component is the object that computes the patches over the frame based on the last target
//! location. The TrackerFeatureSet is the class that manages the Features, is possible plug many kind
//! of these (HAAR, HOG, LBP, Feature2D, etc). The last component is the internal representation of the
//! target, it is the appearance model. It stores all state candidates and compute the trajectory (the
//! most likely target states). The class TrackerTargetState represents a possible state of the target.
//! The TrackerSampler and the TrackerFeatureSet are the visual representation of the target, instead
//! the TrackerModel is the statistical model.
//!
//! A recent benchmark between these algorithms can be found in [OOT](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_OOT)
//!
//! Creating Your Own %Tracker
//! --------------------
//!
//! If you want to create a new tracker, here's what you have to do. First, decide on the name of the class
//! for the tracker (to meet the existing style, we suggest something with prefix "tracker", e.g.
//! trackerMIL, trackerBoosting) -- we shall refer to this choice as to "classname" in subsequent.
//!
//! *   Declare your tracker in modules/tracking/include/opencv2/tracking/tracker.hpp. Your tracker should inherit from
//!    Tracker (please, see the example below). You should declare the specialized Param structure,
//!    where you probably will want to put the data, needed to initialize your tracker. You should
//!    get something similar to :
//! ```C++
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
//!            int featureSetNumFeatures;  // [features]
//!
//!            void read( const FileNode& fn );
//!            void write( FileStorage& fs ) const;
//!           };
//! ```
//!
//!    of course, you can also add any additional methods of your choice. It should be pointed out,
//!    however, that it is not expected to have a constructor declared, as creation should be done via
//!    the corresponding create() method.
//! *   Finally, you should implement the function with signature :
//! ```C++
//!        Ptr<classname> classname::create(const classname::Params &parameters){
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
//! ```C++
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
//! ```C++
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
//! ```C++
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
//! ```C++
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
//! estimates the most likely target location, see [AAM](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_AAM) table I (ME) for further information. Fill
//! "modelUpdateImpl" in order to update the model, see [AAM](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_AAM) table I (MU). In this class you can use
//! the :cConfidenceMap and :cTrajectory to storing the model. The first represents the model on the all
//! possible candidate states and the second represents the list of all estimated states.
//!
//! Example of creating specialized TrackerModel TrackerMILModel : :
//! ```C++
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
//! ```C++
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
//! ```C++
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
//! ```C++
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
//! ```C++
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
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{ClfMilBoostTrait, ClfMilBoostTraitConst, ClfMilBoost_ParamsTrait, ClfMilBoost_ParamsTraitConst, CvFeatureParamsTrait, CvFeatureParamsTraitConst, CvHaarEvaluatorTrait, CvHaarEvaluatorTraitConst, CvHaarEvaluator_FeatureHaarTrait, CvHaarEvaluator_FeatureHaarTraitConst, MultiTrackerTLDTrait, MultiTrackerTLDTraitConst, MultiTrackerTrait, MultiTrackerTraitConst, MultiTracker_AltTrait, MultiTracker_AltTraitConst, TrackerBoostingTrait, TrackerBoostingTraitConst, TrackerBoosting_ParamsTrait, TrackerBoosting_ParamsTraitConst, TrackerCSRTTrait, TrackerCSRTTraitConst, TrackerCSRT_ParamsTrait, TrackerCSRT_ParamsTraitConst, TrackerFeatureFeature2dTrait, TrackerFeatureFeature2dTraitConst, TrackerFeatureHAARTrait, TrackerFeatureHAARTraitConst, TrackerFeatureHAAR_ParamsTrait, TrackerFeatureHAAR_ParamsTraitConst, TrackerFeatureHOGTrait, TrackerFeatureHOGTraitConst, TrackerFeatureLBPTrait, TrackerFeatureLBPTraitConst, TrackerFeatureSetTrait, TrackerFeatureSetTraitConst, TrackerFeatureTrait, TrackerFeatureTraitConst, TrackerGOTURNTrait, TrackerGOTURNTraitConst, TrackerGOTURN_ParamsTrait, TrackerGOTURN_ParamsTraitConst, TrackerKCFTrait, TrackerKCFTraitConst, TrackerKCF_ParamsTrait, TrackerKCF_ParamsTraitConst, TrackerMILTrait, TrackerMILTraitConst, TrackerMIL_ParamsTrait, TrackerMIL_ParamsTraitConst, TrackerMOSSETrait, TrackerMOSSETraitConst, TrackerMedianFlowTrait, TrackerMedianFlowTraitConst, TrackerMedianFlow_ParamsTrait, TrackerMedianFlow_ParamsTraitConst, TrackerModelTrait, TrackerModelTraitConst, TrackerSamplerAlgorithmTrait, TrackerSamplerAlgorithmTraitConst, TrackerSamplerCSCTrait, TrackerSamplerCSCTraitConst, TrackerSamplerCSC_ParamsTrait, TrackerSamplerCSC_ParamsTraitConst, TrackerSamplerCSTrait, TrackerSamplerCSTraitConst, TrackerSamplerCS_ParamsTrait, TrackerSamplerCS_ParamsTraitConst, TrackerSamplerPFTrait, TrackerSamplerPFTraitConst, TrackerSamplerPF_ParamsTrait, TrackerSamplerPF_ParamsTraitConst, TrackerSamplerTrait, TrackerSamplerTraitConst, TrackerStateEstimatorAdaBoostingTrait, TrackerStateEstimatorAdaBoostingTraitConst, TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetStateTrait, TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetStateTraitConst, TrackerStateEstimatorMILBoostingTrait, TrackerStateEstimatorMILBoostingTraitConst, TrackerStateEstimatorMILBoosting_TrackerMILTargetStateTrait, TrackerStateEstimatorMILBoosting_TrackerMILTargetStateTraitConst, TrackerStateEstimatorSVMTrait, TrackerStateEstimatorSVMTraitConst, TrackerStateEstimatorTrait, TrackerStateEstimatorTraitConst, TrackerTLDTrait, TrackerTLDTraitConst, TrackerTLD_ParamsTrait, TrackerTLD_ParamsTraitConst, TrackerTargetStateTrait, TrackerTargetStateTraitConst, TrackerTrait, TrackerTraitConst};
}

// CC_FEATURE_PARAMS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:65
pub const CC_FEATURE_PARAMS: &str = "featureParams";
// CC_FEATURE_SIZE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:67
pub const CC_FEATURE_SIZE: &str = "featSize";
// CC_ISINTEGRAL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:69
pub const CC_ISINTEGRAL: &str = "isIntegral";
// CC_MAX_CAT_COUNT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:66
pub const CC_MAX_CAT_COUNT: &str = "maxCatCount";
// CC_NUM_FEATURES /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:68
pub const CC_NUM_FEATURES: &str = "numFeat";
// CC_RECT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:72
pub const CC_RECT: &str = "rect";
// CC_RECTS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:70
pub const CC_RECTS: &str = "rects";
// CC_TILTED /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:71
pub const CC_TILTED: &str = "tilted";
// CV_HAAR_FEATURE_MAX /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:78
pub const CV_HAAR_FEATURE_MAX: i32 = 3;
// HAAR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:142
pub const CvFeatureParams_HAAR: i32 = 0;
// HOG /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:144
pub const CvFeatureParams_HOG: i32 = 2;
// LBP /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:143
pub const CvFeatureParams_LBP: i32 = 1;
// FEATURES /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:62
pub const FEATURES: &str = "features";
// HFP_NAME /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:76
pub const HFP_NAME: &str = "haarFeatureParams";
// HOGF_NAME /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:75
pub const HOGF_NAME: &str = "HOGFeatureParams";
// LBPF_NAME /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:74
pub const LBPF_NAME: &str = "lbpFeatureParams";
// N_BINS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:79
pub const N_BINS: i32 = 9;
// N_CELLS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:80
pub const N_CELLS: i32 = 4;
// CN /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1223
pub const TrackerKCF_CN: i32 = 2;
// CUSTOM /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1224
pub const TrackerKCF_CUSTOM: i32 = 4;
// GRAY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1222
pub const TrackerKCF_GRAY: i32 = 1;
/// mode for detect samples
// MODE_DETECT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:780
pub const TrackerSamplerCSC_MODE_DETECT: i32 = 5;
/// mode for init negative samples
// MODE_INIT_NEG /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:777
pub const TrackerSamplerCSC_MODE_INIT_NEG: i32 = 2;
/// mode for init positive samples
// MODE_INIT_POS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:776
pub const TrackerSamplerCSC_MODE_INIT_POS: i32 = 1;
/// mode for update negative samples
// MODE_TRACK_NEG /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:779
pub const TrackerSamplerCSC_MODE_TRACK_NEG: i32 = 4;
/// mode for update positive samples
// MODE_TRACK_POS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:778
pub const TrackerSamplerCSC_MODE_TRACK_POS: i32 = 3;
/// mode for classify samples
// MODE_CLASSIFY /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:836
pub const TrackerSamplerCS_MODE_CLASSIFY: i32 = 3;
/// mode for negative samples
// MODE_NEGATIVE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:835
pub const TrackerSamplerCS_MODE_NEGATIVE: i32 = 2;
/// mode for positive samples
// MODE_POSITIVE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:834
pub const TrackerSamplerCS_MODE_POSITIVE: i32 = 1;
/// \brief Feature type to be used in the tracking grayscale, colornames, compressed color-names
///  The modes available now:
/// *   "GRAY" -- Use grayscale values as the feature
/// *   "CN" -- Color-names feature
// MODE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1221
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TrackerKCF_MODE {
	GRAY = 1,
	CN = 2,
	CUSTOM = 4,
}

impl TryFrom<i32> for TrackerKCF_MODE {
	type Error = crate::Error;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		match value {
			1 => Ok(Self::GRAY),
			2 => Ok(Self::CN),
			4 => Ok(Self::CUSTOM),
			_ => Err(crate::Error::new(crate::core::StsBadArg, format!("Value: {value} is not valid for enum: crate::tracking::TrackerKCF_MODE"))),
		}
	}
}

opencv_type_enum! { crate::tracking::TrackerKCF_MODE }

/// Represents the model of the target at frame ![inline formula](https://latex.codecogs.com/png.latex?k) (all states and scores)
///
/// See [AAM](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_AAM) The set of the pair ![inline formula](https://latex.codecogs.com/png.latex?%5Clangle%20%5Chat%7Bx%7D%5E%7Bi%7D%5F%7Bk%7D%2C%20C%5E%7Bi%7D%5F%7Bk%7D%20%5Crangle)
/// ## See also
/// TrackerTargetState
// ConfidenceMap /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:384
pub type ConfidenceMap = core::Vector<core::Tuple<(core::Ptr<crate::tracking::TrackerTargetState>, f32)>>;
/// Represents the estimate states for all frames
///
/// [AAM](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_AAM) ![inline formula](https://latex.codecogs.com/png.latex?x%5F%7Bk%7D) is the trajectory of the target up to time ![inline formula](https://latex.codecogs.com/png.latex?k)
/// ## See also
/// TrackerTargetState
// Trajectory /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:392
pub type Trajectory = core::Vector<core::Ptr<crate::tracking::TrackerTargetState>>;
/// ## Note
/// This alternative version of [tld_init_dataset] function uses the following default values for its arguments:
/// * root_path: "TLD_dataset"
/// * dataset_ind: 0
// cv::tld::tld_InitDataset(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tldDataset.hpp:51
// ("cv::tld::tld_InitDataset", vec![(pred!(mut, ["videoInd"], ["int"]), _)]),
#[inline]
pub fn tld_init_dataset_def(video_ind: i32) -> Result<core::Rect2d> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_tld_tld_InitDataset_int(video_ind, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * root_path: "TLD_dataset"
/// * dataset_ind: 0
// tld_InitDataset(int, const char *, int)(Primitive, InString, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tldDataset.hpp:51
// ("cv::tld::tld_InitDataset", vec![(pred!(mut, ["videoInd", "rootPath", "datasetInd"], ["int", "const char*", "int"]), _)]),
#[inline]
pub fn tld_init_dataset(video_ind: i32, root_path: &str, dataset_ind: i32) -> Result<core::Rect2d> {
	extern_container_arg!(root_path);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_tld_tld_InitDataset_int_const_charX_int(video_ind, root_path.opencv_as_extern(), dataset_ind, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// tld_getNextDatasetFrame()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tldDataset.hpp:52
// ("cv::tld::tld_getNextDatasetFrame", vec![(pred!(mut, [], []), _)]),
#[inline]
pub fn tld_get_next_dataset_frame() -> Result<String> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_tld_tld_getNextDatasetFrame(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { String::opencv_from_extern(ret) };
	Ok(ret)
}

/// Constant methods for [crate::tracking::ClfMilBoost]
// ClfMilBoost /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:59
pub trait ClfMilBoostTraitConst {
	fn as_raw_ClfMilBoost(&self) -> *const c_void;

}

/// Mutable methods for [crate::tracking::ClfMilBoost]
pub trait ClfMilBoostTrait: crate::tracking::ClfMilBoostTraitConst {
	fn as_raw_mut_ClfMilBoost(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * parameters: ClfMilBoost::Params()
	// init(const ClfMilBoost::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:72
	// ("cv::ClfMilBoost::init", vec![(pred!(mut, ["parameters"], ["const cv::ClfMilBoost::Params*"]), _)]),
	#[inline]
	fn init(&mut self, parameters: &impl crate::tracking::ClfMilBoost_ParamsTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ClfMilBoost_init_const_ParamsR(self.as_raw_mut_ClfMilBoost(), parameters.as_raw_ClfMilBoost_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [ClfMilBoostTrait::init] function uses the following default values for its arguments:
	/// * parameters: ClfMilBoost::Params()
	// cv::ClfMilBoost::init() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:72
	// ("cv::ClfMilBoost::init", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn init_def(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ClfMilBoost_init(self.as_raw_mut_ClfMilBoost(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// update(const Mat &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:73
	// ("cv::ClfMilBoost::update", vec![(pred!(mut, ["posx", "negx"], ["const cv::Mat*", "const cv::Mat*"]), _)]),
	#[inline]
	fn update(&mut self, posx: &impl core::MatTraitConst, negx: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ClfMilBoost_update_const_MatR_const_MatR(self.as_raw_mut_ClfMilBoost(), posx.as_raw_Mat(), negx.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * log_r: true
	// classify(const Mat &, bool)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:74
	// ("cv::ClfMilBoost::classify", vec![(pred!(mut, ["x", "logR"], ["const cv::Mat*", "bool"]), _)]),
	#[inline]
	fn classify(&mut self, x: &impl core::MatTraitConst, log_r: bool) -> Result<core::Vector<f32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ClfMilBoost_classify_const_MatR_bool(self.as_raw_mut_ClfMilBoost(), x.as_raw_Mat(), log_r, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<f32>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [ClfMilBoostTrait::classify] function uses the following default values for its arguments:
	/// * log_r: true
	// cv::ClfMilBoost::classify(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:74
	// ("cv::ClfMilBoost::classify", vec![(pred!(mut, ["x"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn classify_def(&mut self, x: &impl core::MatTraitConst) -> Result<core::Vector<f32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ClfMilBoost_classify_const_MatR(self.as_raw_mut_ClfMilBoost(), x.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<f32>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// sigmoid(float)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:76
	// ("cv::ClfMilBoost::sigmoid", vec![(pred!(mut, ["x"], ["float"]), _)]),
	#[inline]
	fn sigmoid(&mut self, x: f32) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ClfMilBoost_sigmoid_float(self.as_raw_mut_ClfMilBoost(), x, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// ClfMilBoost /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:59
pub struct ClfMilBoost {
	ptr: *mut c_void,
}

opencv_type_boxed! { ClfMilBoost }

impl Drop for ClfMilBoost {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_ClfMilBoost_delete(self.as_raw_mut_ClfMilBoost()) };
	}
}

unsafe impl Send for ClfMilBoost {}

impl crate::tracking::ClfMilBoostTraitConst for ClfMilBoost {
	#[inline] fn as_raw_ClfMilBoost(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::ClfMilBoostTrait for ClfMilBoost {
	#[inline] fn as_raw_mut_ClfMilBoost(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ClfMilBoost, crate::tracking::ClfMilBoostTraitConst, as_raw_ClfMilBoost, crate::tracking::ClfMilBoostTrait, as_raw_mut_ClfMilBoost }

impl ClfMilBoost {
	// ClfMilBoost()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:70
	// ("cv::ClfMilBoost::ClfMilBoost", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::tracking::ClfMilBoost> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ClfMilBoost_ClfMilBoost(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::ClfMilBoost::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for ClfMilBoost {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ClfMilBoost")
			.finish()
	}
}

/// Constant methods for [crate::tracking::ClfMilBoost_Params]
// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:62
pub trait ClfMilBoost_ParamsTraitConst {
	fn as_raw_ClfMilBoost_Params(&self) -> *const c_void;

	// cv::ClfMilBoost::Params::_numSel() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:65
	// ("cv::ClfMilBoost::Params::_numSel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn _num_sel(&self) -> i32 {
		let ret = unsafe { sys::cv_ClfMilBoost_Params_prop_numSel_const(self.as_raw_ClfMilBoost_Params()) };
		ret
	}

	// cv::ClfMilBoost::Params::_numFeat() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:66
	// ("cv::ClfMilBoost::Params::_numFeat", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn _num_feat(&self) -> i32 {
		let ret = unsafe { sys::cv_ClfMilBoost_Params_prop_numFeat_const(self.as_raw_ClfMilBoost_Params()) };
		ret
	}

	// cv::ClfMilBoost::Params::_lRate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:67
	// ("cv::ClfMilBoost::Params::_lRate", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn _l_rate(&self) -> f32 {
		let ret = unsafe { sys::cv_ClfMilBoost_Params_prop_lRate_const(self.as_raw_ClfMilBoost_Params()) };
		ret
	}

}

/// Mutable methods for [crate::tracking::ClfMilBoost_Params]
pub trait ClfMilBoost_ParamsTrait: crate::tracking::ClfMilBoost_ParamsTraitConst {
	fn as_raw_mut_ClfMilBoost_Params(&mut self) -> *mut c_void;

	// cv::ClfMilBoost::Params::set_numSel(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:65
	// ("cv::ClfMilBoost::Params::set_numSel", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_num_sel(&mut self, val: i32) {
		let ret = unsafe { sys::cv_ClfMilBoost_Params_prop_numSel_const_int(self.as_raw_mut_ClfMilBoost_Params(), val) };
		ret
	}

	// cv::ClfMilBoost::Params::set_numFeat(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:66
	// ("cv::ClfMilBoost::Params::set_numFeat", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_num_feat(&mut self, val: i32) {
		let ret = unsafe { sys::cv_ClfMilBoost_Params_prop_numFeat_const_int(self.as_raw_mut_ClfMilBoost_Params(), val) };
		ret
	}

	// cv::ClfMilBoost::Params::set_lRate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:67
	// ("cv::ClfMilBoost::Params::set_lRate", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_l_rate(&mut self, val: f32) {
		let ret = unsafe { sys::cv_ClfMilBoost_Params_prop_lRate_const_float(self.as_raw_mut_ClfMilBoost_Params(), val) };
		ret
	}

}

// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:62
pub struct ClfMilBoost_Params {
	ptr: *mut c_void,
}

opencv_type_boxed! { ClfMilBoost_Params }

impl Drop for ClfMilBoost_Params {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_ClfMilBoost_Params_delete(self.as_raw_mut_ClfMilBoost_Params()) };
	}
}

unsafe impl Send for ClfMilBoost_Params {}

impl crate::tracking::ClfMilBoost_ParamsTraitConst for ClfMilBoost_Params {
	#[inline] fn as_raw_ClfMilBoost_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::ClfMilBoost_ParamsTrait for ClfMilBoost_Params {
	#[inline] fn as_raw_mut_ClfMilBoost_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ClfMilBoost_Params, crate::tracking::ClfMilBoost_ParamsTraitConst, as_raw_ClfMilBoost_Params, crate::tracking::ClfMilBoost_ParamsTrait, as_raw_mut_ClfMilBoost_Params }

impl ClfMilBoost_Params {
	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/onlineMIL.hpp:64
	// ("cv::ClfMilBoost::Params::Params", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::tracking::ClfMilBoost_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ClfMilBoost_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::ClfMilBoost_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for ClfMilBoost_Params {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ClfMilBoost_Params")
			.field("_num_sel", &crate::tracking::ClfMilBoost_ParamsTraitConst::_num_sel(self))
			.field("_num_feat", &crate::tracking::ClfMilBoost_ParamsTraitConst::_num_feat(self))
			.field("_l_rate", &crate::tracking::ClfMilBoost_ParamsTraitConst::_l_rate(self))
			.finish()
	}
}

/// Constant methods for [crate::tracking::CvFeatureParams]
// CvFeatureParams /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:137
pub trait CvFeatureParamsTraitConst {
	fn as_raw_CvFeatureParams(&self) -> *const c_void;

	// cv::CvFeatureParams::maxCatCount() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:151
	// ("cv::CvFeatureParams::maxCatCount", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn max_cat_count(&self) -> i32 {
		let ret = unsafe { sys::cv_CvFeatureParams_propMaxCatCount_const(self.as_raw_CvFeatureParams()) };
		ret
	}

	// cv::CvFeatureParams::featSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:152
	// ("cv::CvFeatureParams::featSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn feat_size(&self) -> i32 {
		let ret = unsafe { sys::cv_CvFeatureParams_propFeatSize_const(self.as_raw_CvFeatureParams()) };
		ret
	}

	// cv::CvFeatureParams::numFeatures() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:153
	// ("cv::CvFeatureParams::numFeatures", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_features(&self) -> i32 {
		let ret = unsafe { sys::cv_CvFeatureParams_propNumFeatures_const(self.as_raw_CvFeatureParams()) };
		ret
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:148
	// ("cv::CvFeatureParams::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CvFeatureParams_write_const_FileStorageR(self.as_raw_CvFeatureParams(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::CvFeatureParams]
pub trait CvFeatureParamsTrait: crate::tracking::CvFeatureParamsTraitConst {
	fn as_raw_mut_CvFeatureParams(&mut self) -> *mut c_void;

	// cv::CvFeatureParams::setMaxCatCount(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:151
	// ("cv::CvFeatureParams::setMaxCatCount", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_max_cat_count(&mut self, val: i32) {
		let ret = unsafe { sys::cv_CvFeatureParams_propMaxCatCount_const_int(self.as_raw_mut_CvFeatureParams(), val) };
		ret
	}

	// cv::CvFeatureParams::setFeatSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:152
	// ("cv::CvFeatureParams::setFeatSize", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_feat_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_CvFeatureParams_propFeatSize_const_int(self.as_raw_mut_CvFeatureParams(), val) };
		ret
	}

	// cv::CvFeatureParams::setNumFeatures(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:153
	// ("cv::CvFeatureParams::setNumFeatures", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_num_features(&mut self, val: i32) {
		let ret = unsafe { sys::cv_CvFeatureParams_propNumFeatures_const_int(self.as_raw_mut_CvFeatureParams(), val) };
		ret
	}

	// init(const CvFeatureParams &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:147
	// ("cv::CvFeatureParams::init", vec![(pred!(mut, ["fp"], ["const cv::CvFeatureParams*"]), _)]),
	#[inline]
	fn init(&mut self, fp: &impl crate::tracking::CvFeatureParamsTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CvFeatureParams_init_const_CvFeatureParamsR(self.as_raw_mut_CvFeatureParams(), fp.as_raw_CvFeatureParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:149
	// ("cv::CvFeatureParams::read", vec![(pred!(mut, ["node"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read(&mut self, node: &impl core::FileNodeTraitConst) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CvFeatureParams_read_const_FileNodeR(self.as_raw_mut_CvFeatureParams(), node.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// CvFeatureParams /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:137
pub struct CvFeatureParams {
	ptr: *mut c_void,
}

opencv_type_boxed! { CvFeatureParams }

impl Drop for CvFeatureParams {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_CvFeatureParams_delete(self.as_raw_mut_CvFeatureParams()) };
	}
}

unsafe impl Send for CvFeatureParams {}

impl crate::tracking::CvFeatureParamsTraitConst for CvFeatureParams {
	#[inline] fn as_raw_CvFeatureParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::CvFeatureParamsTrait for CvFeatureParams {
	#[inline] fn as_raw_mut_CvFeatureParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CvFeatureParams, crate::tracking::CvFeatureParamsTraitConst, as_raw_CvFeatureParams, crate::tracking::CvFeatureParamsTrait, as_raw_mut_CvFeatureParams }

impl CvFeatureParams {
}

impl std::fmt::Debug for CvFeatureParams {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("CvFeatureParams")
			.field("max_cat_count", &crate::tracking::CvFeatureParamsTraitConst::max_cat_count(self))
			.field("feat_size", &crate::tracking::CvFeatureParamsTraitConst::feat_size(self))
			.field("num_features", &crate::tracking::CvFeatureParamsTraitConst::num_features(self))
			.finish()
	}
}

/// Constant methods for [crate::tracking::CvHaarEvaluator]
// CvHaarEvaluator /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:215
pub trait CvHaarEvaluatorTraitConst {
	fn as_raw_CvHaarEvaluator(&self) -> *const c_void;

	// writeFeatures(FileStorage &, const Mat &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:257
	// ("cv::CvHaarEvaluator::writeFeatures", vec![(pred!(const, ["fs", "featureMap"], ["cv::FileStorage*", "const cv::Mat*"]), _)]),
	#[inline]
	fn write_features(&self, fs: &mut impl core::FileStorageTrait, feature_map: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CvHaarEvaluator_writeFeatures_const_FileStorageR_const_MatR(self.as_raw_CvHaarEvaluator(), fs.as_raw_mut_FileStorage(), feature_map.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::CvHaarEvaluator]
pub trait CvHaarEvaluatorTrait: crate::tracking::CvHaarEvaluatorTraitConst {
	fn as_raw_mut_CvHaarEvaluator(&mut self) -> *mut c_void;

	// init(const CvFeatureParams *, int, Size)(TraitClass, Primitive, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:254
	// ("cv::CvHaarEvaluator::init", vec![(pred!(mut, ["_featureParams", "_maxSampleCount", "_winSize"], ["const cv::CvFeatureParams*", "int", "cv::Size"]), _)]),
	#[inline]
	fn init(&mut self, _feature_params: &impl crate::tracking::CvFeatureParamsTraitConst, _max_sample_count: i32, _win_size: core::Size) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CvHaarEvaluator_init_const_CvFeatureParamsX_int_Size(self.as_raw_mut_CvHaarEvaluator(), _feature_params.as_raw_CvFeatureParams(), _max_sample_count, &_win_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## C++ default parameters
	/// * cls_label: 0
	/// * idx: 1
	// setImage(const Mat &, uchar, int)(TraitClass, Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:255
	// ("cv::CvHaarEvaluator::setImage", vec![(pred!(mut, ["img", "clsLabel", "idx"], ["const cv::Mat*", "unsigned char", "int"]), _)]),
	#[inline]
	fn set_image(&mut self, img: &impl core::MatTraitConst, cls_label: u8, idx: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CvHaarEvaluator_setImage_const_MatR_unsigned_char_int(self.as_raw_mut_CvHaarEvaluator(), img.as_raw_Mat(), cls_label, idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [CvHaarEvaluatorTrait::set_image] function uses the following default values for its arguments:
	/// * cls_label: 0
	/// * idx: 1
	// cv::CvHaarEvaluator::setImage(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:255
	// ("cv::CvHaarEvaluator::setImage", vec![(pred!(mut, ["img"], ["const cv::Mat*"]), _)]),
	#[inline]
	fn set_image_def(&mut self, img: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CvHaarEvaluator_setImage_const_MatR(self.as_raw_mut_CvHaarEvaluator(), img.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// operator()(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:256
	// ("cv::CvHaarEvaluator::operator()", vec![(pred!(mut, ["featureIdx", "sampleIdx"], ["int", "int"]), _)]),
	#[inline]
	fn apply(&mut self, feature_idx: i32, sample_idx: i32) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CvHaarEvaluator_operator___int_int(self.as_raw_mut_CvHaarEvaluator(), feature_idx, sample_idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:260
	// ("cv::CvHaarEvaluator::getFeatures", vec![(pred!(mut, ["idx"], ["int"]), _)]),
	#[inline]
	fn get_features(&mut self, idx: i32) -> Result<crate::tracking::CvHaarEvaluator_FeatureHaar> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CvHaarEvaluator_getFeatures_int(self.as_raw_mut_CvHaarEvaluator(), idx, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::CvHaarEvaluator_FeatureHaar::opencv_from_extern(ret) };
		Ok(ret)
	}

	// generateFeatures()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:266
	// ("cv::CvHaarEvaluator::generateFeatures", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn generate_features(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CvHaarEvaluator_generateFeatures(self.as_raw_mut_CvHaarEvaluator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// TODO new method
	/// \brief Overload the original generateFeatures in order to limit the number of the features
	/// ## Parameters
	/// * numFeatures: Number of the features
	// generateFeatures(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:274
	// ("cv::CvHaarEvaluator::generateFeatures", vec![(pred!(mut, ["numFeatures"], ["int"]), _)]),
	#[inline]
	fn generate_features_1(&mut self, num_features: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CvHaarEvaluator_generateFeatures_int(self.as_raw_mut_CvHaarEvaluator(), num_features, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// CvHaarEvaluator /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:215
pub struct CvHaarEvaluator {
	ptr: *mut c_void,
}

opencv_type_boxed! { CvHaarEvaluator }

impl Drop for CvHaarEvaluator {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_CvHaarEvaluator_delete(self.as_raw_mut_CvHaarEvaluator()) };
	}
}

unsafe impl Send for CvHaarEvaluator {}

impl crate::tracking::CvHaarEvaluatorTraitConst for CvHaarEvaluator {
	#[inline] fn as_raw_CvHaarEvaluator(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::CvHaarEvaluatorTrait for CvHaarEvaluator {
	#[inline] fn as_raw_mut_CvHaarEvaluator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CvHaarEvaluator, crate::tracking::CvHaarEvaluatorTraitConst, as_raw_CvHaarEvaluator, crate::tracking::CvHaarEvaluatorTrait, as_raw_mut_CvHaarEvaluator }

impl CvHaarEvaluator {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_CvHaarEvaluator_defaultNew_const()) }
	}

}

impl std::fmt::Debug for CvHaarEvaluator {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("CvHaarEvaluator")
			.finish()
	}
}

impl Default for CvHaarEvaluator {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::tracking::CvHaarEvaluator_FeatureHaar]
// FeatureHaar /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:219
pub trait CvHaarEvaluator_FeatureHaarTraitConst {
	fn as_raw_CvHaarEvaluator_FeatureHaar(&self) -> *const c_void;

	// write(FileStorage)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:229
	// ("cv::CvHaarEvaluator::FeatureHaar::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage"]), _)]),
	#[inline]
	fn write(&self, mut unnamed: impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CvHaarEvaluator_FeatureHaar_write_const_FileStorage(self.as_raw_CvHaarEvaluator_FeatureHaar(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::CvHaarEvaluator_FeatureHaar]
pub trait CvHaarEvaluator_FeatureHaarTrait: crate::tracking::CvHaarEvaluator_FeatureHaarTraitConst {
	fn as_raw_mut_CvHaarEvaluator_FeatureHaar(&mut self) -> *mut c_void;

}

// FeatureHaar /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/feature.hpp:219
pub struct CvHaarEvaluator_FeatureHaar {
	ptr: *mut c_void,
}

opencv_type_boxed! { CvHaarEvaluator_FeatureHaar }

impl Drop for CvHaarEvaluator_FeatureHaar {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_CvHaarEvaluator_FeatureHaar_delete(self.as_raw_mut_CvHaarEvaluator_FeatureHaar()) };
	}
}

unsafe impl Send for CvHaarEvaluator_FeatureHaar {}

impl crate::tracking::CvHaarEvaluator_FeatureHaarTraitConst for CvHaarEvaluator_FeatureHaar {
	#[inline] fn as_raw_CvHaarEvaluator_FeatureHaar(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::CvHaarEvaluator_FeatureHaarTrait for CvHaarEvaluator_FeatureHaar {
	#[inline] fn as_raw_mut_CvHaarEvaluator_FeatureHaar(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { CvHaarEvaluator_FeatureHaar, crate::tracking::CvHaarEvaluator_FeatureHaarTraitConst, as_raw_CvHaarEvaluator_FeatureHaar, crate::tracking::CvHaarEvaluator_FeatureHaarTrait, as_raw_mut_CvHaarEvaluator_FeatureHaar }

impl CvHaarEvaluator_FeatureHaar {
}

impl std::fmt::Debug for CvHaarEvaluator_FeatureHaar {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("CvHaarEvaluator_FeatureHaar")
			.finish()
	}
}

/// Constant methods for [crate::tracking::MultiTracker]
// MultiTracker /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1332
pub trait MultiTrackerTraitConst: core::AlgorithmTraitConst {
	fn as_raw_MultiTracker(&self) -> *const c_void;

	/// \brief Returns a reference to a storage for the tracked objects, each object corresponds to one tracker algorithm
	// getObjects()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1380
	// ("cv::MultiTracker::getObjects", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_objects(&self) -> Result<core::Vector<core::Rect2d>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MultiTracker_getObjects_const(self.as_raw_MultiTracker(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Rect2d>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::MultiTracker]
pub trait MultiTrackerTrait: core::AlgorithmTrait + crate::tracking::MultiTrackerTraitConst {
	fn as_raw_mut_MultiTracker(&mut self) -> *mut c_void;

	/// \brief Add a new object to be tracked.
	///
	/// ## Parameters
	/// * newTracker: tracking algorithm to be used
	/// * image: input image
	/// * boundingBox: a rectangle represents ROI of the tracked object
	// add(Ptr<Tracker>, InputArray, const Rect2d &)(CppPassByVoidPtr, InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1353
	// ("cv::MultiTracker::add", vec![(pred!(mut, ["newTracker", "image", "boundingBox"], ["cv::Ptr<cv::Tracker>", "const cv::_InputArray*", "const cv::Rect2d*"]), _)]),
	#[inline]
	fn add(&mut self, mut new_tracker: core::Ptr<crate::tracking::Tracker>, image: &impl ToInputArray, bounding_box: core::Rect2d) -> Result<bool> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MultiTracker_add_PtrLTrackerG_const__InputArrayR_const_Rect2dR(self.as_raw_mut_MultiTracker(), new_tracker.as_raw_mut_PtrOfTracker(), image.as_raw__InputArray(), &bounding_box, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// \brief Add a set of objects to be tracked.
	/// ## Parameters
	/// * newTrackers: list of tracking algorithms to be used
	/// * image: input image
	/// * boundingBox: list of the tracked objects
	// add(std::vector<Ptr<Tracker>>, InputArray, std::vector<Rect2d>)(CppPassByVoidPtr, InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1361
	// ("cv::MultiTracker::add", vec![(pred!(mut, ["newTrackers", "image", "boundingBox"], ["std::vector<cv::Ptr<cv::Tracker>>", "const cv::_InputArray*", "std::vector<cv::Rect2d>"]), _)]),
	#[inline]
	fn add_1(&mut self, mut new_trackers: core::Vector<core::Ptr<crate::tracking::Tracker>>, image: &impl ToInputArray, mut bounding_box: core::Vector<core::Rect2d>) -> Result<bool> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MultiTracker_add_vectorLPtrLTrackerGG_const__InputArrayR_vectorLRect2dG(self.as_raw_mut_MultiTracker(), new_trackers.as_raw_mut_VectorOfPtrOfTracker(), image.as_raw__InputArray(), bounding_box.as_raw_mut_VectorOfRect2d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// \brief Update the current tracking status.
	/// The result will be saved in the internal storage.
	/// ## Parameters
	/// * image: input image
	// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1368
	// ("cv::MultiTracker::update", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn update(&mut self, image: &impl ToInputArray) -> Result<bool> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MultiTracker_update_const__InputArrayR(self.as_raw_mut_MultiTracker(), image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// \brief Update the current tracking status.
	/// ## Parameters
	/// * image: input image
	/// * boundingBox: the tracking result, represent a list of ROIs of the tracked objects.
	// update(InputArray, std::vector<Rect2d> &)(InputArray, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1375
	// ("cv::MultiTracker::update", vec![(pred!(mut, ["image", "boundingBox"], ["const cv::_InputArray*", "std::vector<cv::Rect2d>*"]), _)]),
	#[inline]
	fn update_1(&mut self, image: &impl ToInputArray, bounding_box: &mut core::Vector<core::Rect2d>) -> Result<bool> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MultiTracker_update_const__InputArrayR_vectorLRect2dGR(self.as_raw_mut_MultiTracker(), image.as_raw__InputArray(), bounding_box.as_raw_mut_VectorOfRect2d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// ********************************** MultiTracker Class ---By Laksono Kurnianggoro---) ***********************************
/// This class is used to track multiple objects using the specified tracker algorithm.
///
/// * The %MultiTracker is naive implementation of multiple object tracking.
/// * It process the tracked objects independently without any optimization accross the tracked objects.
// MultiTracker /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1332
pub struct MultiTracker {
	ptr: *mut c_void,
}

opencv_type_boxed! { MultiTracker }

impl Drop for MultiTracker {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_MultiTracker_delete(self.as_raw_mut_MultiTracker()) };
	}
}

unsafe impl Send for MultiTracker {}

impl core::AlgorithmTraitConst for MultiTracker {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for MultiTracker {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MultiTracker, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::tracking::MultiTrackerTraitConst for MultiTracker {
	#[inline] fn as_raw_MultiTracker(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::MultiTrackerTrait for MultiTracker {
	#[inline] fn as_raw_mut_MultiTracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MultiTracker, crate::tracking::MultiTrackerTraitConst, as_raw_MultiTracker, crate::tracking::MultiTrackerTrait, as_raw_mut_MultiTracker }

impl MultiTracker {
	/// \brief Constructor.
	// MultiTracker()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1339
	// ("cv::MultiTracker::MultiTracker", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::tracking::MultiTracker> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MultiTracker_MultiTracker(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::MultiTracker::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// \brief Returns a pointer to a new instance of MultiTracker
	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1385
	// ("cv::MultiTracker::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::tracking::MultiTracker>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MultiTracker_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::MultiTracker>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { MultiTracker, core::Algorithm, cv_MultiTracker_to_Algorithm }

impl std::fmt::Debug for MultiTracker {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MultiTracker")
			.finish()
	}
}

/// Constant methods for [crate::tracking::MultiTrackerTLD]
// MultiTrackerTLD /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1460
pub trait MultiTrackerTLDTraitConst: crate::tracking::MultiTracker_AltTraitConst {
	fn as_raw_MultiTrackerTLD(&self) -> *const c_void;

}

/// Mutable methods for [crate::tracking::MultiTrackerTLD]
pub trait MultiTrackerTLDTrait: crate::tracking::MultiTrackerTLDTraitConst + crate::tracking::MultiTracker_AltTrait {
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
	// update_opt(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1473
	// ("cv::MultiTrackerTLD::update_opt", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn update_opt(&mut self, image: &impl ToInputArray) -> Result<bool> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MultiTrackerTLD_update_opt_const__InputArrayR(self.as_raw_mut_MultiTrackerTLD(), image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Multi Object %Tracker for TLD.
///
/// TLD is a novel tracking framework that explicitly decomposes
/// the long-term tracking task into tracking, learning and detection.
///
/// The tracker follows the object from frame to frame. The detector localizes all appearances that
/// have been observed so far and corrects the tracker if necessary. The learning estimates detector's
/// errors and updates it to avoid these errors in the future. The implementation is based on [TLD](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_TLD) .
///
/// The Median Flow algorithm (see cv::TrackerMedianFlow) was chosen as a tracking component in this
/// implementation, following authors. The tracker is supposed to be able to handle rapid motions, partial
/// occlusions, object absence etc.
/// ## See also
/// Tracker, MultiTracker, TrackerTLD
// MultiTrackerTLD /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1460
pub struct MultiTrackerTLD {
	ptr: *mut c_void,
}

opencv_type_boxed! { MultiTrackerTLD }

impl Drop for MultiTrackerTLD {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_MultiTrackerTLD_delete(self.as_raw_mut_MultiTrackerTLD()) };
	}
}

unsafe impl Send for MultiTrackerTLD {}

impl crate::tracking::MultiTracker_AltTraitConst for MultiTrackerTLD {
	#[inline] fn as_raw_MultiTracker_Alt(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::MultiTracker_AltTrait for MultiTrackerTLD {
	#[inline] fn as_raw_mut_MultiTracker_Alt(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MultiTrackerTLD, crate::tracking::MultiTracker_AltTraitConst, as_raw_MultiTracker_Alt, crate::tracking::MultiTracker_AltTrait, as_raw_mut_MultiTracker_Alt }

impl crate::tracking::MultiTrackerTLDTraitConst for MultiTrackerTLD {
	#[inline] fn as_raw_MultiTrackerTLD(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::MultiTrackerTLDTrait for MultiTrackerTLD {
	#[inline] fn as_raw_mut_MultiTrackerTLD(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MultiTrackerTLD, crate::tracking::MultiTrackerTLDTraitConst, as_raw_MultiTrackerTLD, crate::tracking::MultiTrackerTLDTrait, as_raw_mut_MultiTrackerTLD }

impl MultiTrackerTLD {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_MultiTrackerTLD_defaultNew_const()) }
	}

}

boxed_cast_base! { MultiTrackerTLD, crate::tracking::MultiTracker_Alt, cv_MultiTrackerTLD_to_MultiTracker_Alt }

impl std::fmt::Debug for MultiTrackerTLD {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MultiTrackerTLD")
			.field("target_num", &crate::tracking::MultiTracker_AltTraitConst::target_num(self))
			.field("trackers", &crate::tracking::MultiTracker_AltTraitConst::trackers(self))
			.field("bounding_boxes", &crate::tracking::MultiTracker_AltTraitConst::bounding_boxes(self))
			.field("colors", &crate::tracking::MultiTracker_AltTraitConst::colors(self))
			.finish()
	}
}

impl Default for MultiTrackerTLD {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

/// Constant methods for [crate::tracking::MultiTracker_Alt]
// MultiTracker_Alt /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1401
pub trait MultiTracker_AltTraitConst {
	fn as_raw_MultiTracker_Alt(&self) -> *const c_void;

	/// Current number of targets in tracking-list
	// cv::MultiTracker_Alt::targetNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1431
	// ("cv::MultiTracker_Alt::targetNum", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn target_num(&self) -> i32 {
		let ret = unsafe { sys::cv_MultiTracker_Alt_propTargetNum_const(self.as_raw_MultiTracker_Alt()) };
		ret
	}

	/// Trackers list for Multi-Object-Tracker
	// cv::MultiTracker_Alt::trackers() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1435
	// ("cv::MultiTracker_Alt::trackers", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn trackers(&self) -> core::Vector<core::Ptr<crate::tracking::Tracker>> {
		let ret = unsafe { sys::cv_MultiTracker_Alt_propTrackers_const(self.as_raw_MultiTracker_Alt()) };
		let ret = unsafe { core::Vector::<core::Ptr<crate::tracking::Tracker>>::opencv_from_extern(ret) };
		ret
	}

	/// Bounding Boxes list for Multi-Object-Tracker
	// cv::MultiTracker_Alt::boundingBoxes() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1439
	// ("cv::MultiTracker_Alt::boundingBoxes", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn bounding_boxes(&self) -> core::Vector<core::Rect2d> {
		let ret = unsafe { sys::cv_MultiTracker_Alt_propBoundingBoxes_const(self.as_raw_MultiTracker_Alt()) };
		let ret = unsafe { core::Vector::<core::Rect2d>::opencv_from_extern(ret) };
		ret
	}

	/// List of randomly generated colors for bounding boxes display
	// cv::MultiTracker_Alt::colors() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1442
	// ("cv::MultiTracker_Alt::colors", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn colors(&self) -> core::Vector<core::Scalar> {
		let ret = unsafe { sys::cv_MultiTracker_Alt_propColors_const(self.as_raw_MultiTracker_Alt()) };
		let ret = unsafe { core::Vector::<core::Scalar>::opencv_from_extern(ret) };
		ret
	}

}

/// Mutable methods for [crate::tracking::MultiTracker_Alt]
pub trait MultiTracker_AltTrait: crate::tracking::MultiTracker_AltTraitConst {
	fn as_raw_mut_MultiTracker_Alt(&mut self) -> *mut c_void;

	/// Current number of targets in tracking-list
	// cv::MultiTracker_Alt::setTargetNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1431
	// ("cv::MultiTracker_Alt::setTargetNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_target_num(&mut self, val: i32) {
		let ret = unsafe { sys::cv_MultiTracker_Alt_propTargetNum_const_int(self.as_raw_mut_MultiTracker_Alt(), val) };
		ret
	}

	/// Trackers list for Multi-Object-Tracker
	// cv::MultiTracker_Alt::setTrackers(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1435
	// ("cv::MultiTracker_Alt::setTrackers", vec![(pred!(mut, ["val"], ["const std::vector<cv::Ptr<cv::Tracker>>"]), _)]),
	#[inline]
	fn set_trackers(&mut self, val: core::Vector<core::Ptr<crate::tracking::Tracker>>) {
		let ret = unsafe { sys::cv_MultiTracker_Alt_propTrackers_const_vectorLPtrLTrackerGG(self.as_raw_mut_MultiTracker_Alt(), val.as_raw_VectorOfPtrOfTracker()) };
		ret
	}

	/// Bounding Boxes list for Multi-Object-Tracker
	// cv::MultiTracker_Alt::setBoundingBoxes(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1439
	// ("cv::MultiTracker_Alt::setBoundingBoxes", vec![(pred!(mut, ["val"], ["const std::vector<cv::Rect2d>"]), _)]),
	#[inline]
	fn set_bounding_boxes(&mut self, val: core::Vector<core::Rect2d>) {
		let ret = unsafe { sys::cv_MultiTracker_Alt_propBoundingBoxes_const_vectorLRect2dG(self.as_raw_mut_MultiTracker_Alt(), val.as_raw_VectorOfRect2d()) };
		ret
	}

	/// List of randomly generated colors for bounding boxes display
	// cv::MultiTracker_Alt::setColors(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1442
	// ("cv::MultiTracker_Alt::setColors", vec![(pred!(mut, ["val"], ["const std::vector<cv::Scalar>"]), _)]),
	#[inline]
	fn set_colors(&mut self, val: core::Vector<core::Scalar>) {
		let ret = unsafe { sys::cv_MultiTracker_Alt_propColors_const_vectorLScalarG(self.as_raw_mut_MultiTracker_Alt(), val.as_raw_VectorOfScalar()) };
		ret
	}

	/// Add a new target to a tracking-list and initialize the tracker with a known bounding box that surrounded the target
	/// ## Parameters
	/// * image: The initial frame
	/// * boundingBox: The initial bounding box of target
	/// * tracker_algorithm: Multi-tracker algorithm
	///
	/// ## Returns
	/// True if new target initialization went succesfully, false otherwise
	// addTarget(InputArray, const Rect2d &, Ptr<Tracker>)(InputArray, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1418
	// ("cv::MultiTracker_Alt::addTarget", vec![(pred!(mut, ["image", "boundingBox", "tracker_algorithm"], ["const cv::_InputArray*", "const cv::Rect2d*", "cv::Ptr<cv::Tracker>"]), _)]),
	#[inline]
	fn add_target(&mut self, image: &impl ToInputArray, bounding_box: core::Rect2d, mut tracker_algorithm: core::Ptr<crate::tracking::Tracker>) -> Result<bool> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MultiTracker_Alt_addTarget_const__InputArrayR_const_Rect2dR_PtrLTrackerG(self.as_raw_mut_MultiTracker_Alt(), image.as_raw__InputArray(), &bounding_box, tracker_algorithm.as_raw_mut_PtrOfTracker(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Update all trackers from the tracking-list, find a new most likely bounding boxes for the targets
	/// ## Parameters
	/// * image: The current frame
	///
	/// ## Returns
	/// True means that all targets were located and false means that tracker couldn't locate one of the targets in
	/// current frame. Note, that latter *does not* imply that tracker has failed, maybe target is indeed
	/// missing from the frame (say, out of sight)
	// update(InputArray)(InputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1427
	// ("cv::MultiTracker_Alt::update", vec![(pred!(mut, ["image"], ["const cv::_InputArray*"]), _)]),
	#[inline]
	fn update(&mut self, image: &impl ToInputArray) -> Result<bool> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MultiTracker_Alt_update_const__InputArrayR(self.as_raw_mut_MultiTracker_Alt(), image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Base abstract class for the long-term Multi Object Trackers:
/// ## See also
/// Tracker, MultiTrackerTLD
// MultiTracker_Alt /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1401
pub struct MultiTracker_Alt {
	ptr: *mut c_void,
}

opencv_type_boxed! { MultiTracker_Alt }

impl Drop for MultiTracker_Alt {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_MultiTracker_Alt_delete(self.as_raw_mut_MultiTracker_Alt()) };
	}
}

unsafe impl Send for MultiTracker_Alt {}

impl crate::tracking::MultiTracker_AltTraitConst for MultiTracker_Alt {
	#[inline] fn as_raw_MultiTracker_Alt(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::MultiTracker_AltTrait for MultiTracker_Alt {
	#[inline] fn as_raw_mut_MultiTracker_Alt(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MultiTracker_Alt, crate::tracking::MultiTracker_AltTraitConst, as_raw_MultiTracker_Alt, crate::tracking::MultiTracker_AltTrait, as_raw_mut_MultiTracker_Alt }

impl MultiTracker_Alt {
	/// Constructor for Multitracker
	// MultiTracker_Alt()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1406
	// ("cv::MultiTracker_Alt::MultiTracker_Alt", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::tracking::MultiTracker_Alt> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MultiTracker_Alt_MultiTracker_Alt(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::MultiTracker_Alt::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for MultiTracker_Alt {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MultiTracker_Alt")
			.field("target_num", &crate::tracking::MultiTracker_AltTraitConst::target_num(self))
			.field("trackers", &crate::tracking::MultiTracker_AltTraitConst::trackers(self))
			.field("bounding_boxes", &crate::tracking::MultiTracker_AltTraitConst::bounding_boxes(self))
			.field("colors", &crate::tracking::MultiTracker_AltTraitConst::colors(self))
			.finish()
	}
}

/// Constant methods for [crate::tracking::Tracker]
// Tracker /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:524
pub trait TrackerTraitConst: core::AlgorithmTraitConst {
	fn as_raw_Tracker(&self) -> *const c_void;

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:550
	// ("cv::Tracker::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Tracker_write_const_FileStorageR(self.as_raw_Tracker(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::Tracker]
pub trait TrackerTrait: core::AlgorithmTrait + crate::tracking::TrackerTraitConst {
	fn as_raw_mut_Tracker(&mut self) -> *mut c_void;

	/// Initialize the tracker with a known bounding box that surrounded the target
	/// ## Parameters
	/// * image: The initial frame
	/// * boundingBox: The initial bounding box
	///
	/// ## Returns
	/// True if initialization went succesfully, false otherwise
	// init(InputArray, const Rect2d &)(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:536
	// ("cv::Tracker::init", vec![(pred!(mut, ["image", "boundingBox"], ["const cv::_InputArray*", "const cv::Rect2d*"]), _)]),
	#[inline]
	fn init(&mut self, image: &impl ToInputArray, bounding_box: core::Rect2d) -> Result<bool> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Tracker_init_const__InputArrayR_const_Rect2dR(self.as_raw_mut_Tracker(), image.as_raw__InputArray(), &bounding_box, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Update the tracker, find the new most likely bounding box for the target
	/// ## Parameters
	/// * image: The current frame
	/// * boundingBox: The bounding box that represent the new target location, if true was returned, not
	/// modified otherwise
	///
	/// ## Returns
	/// True means that target was located and false means that tracker cannot locate target in
	/// current frame. Note, that latter *does not* imply that tracker has failed, maybe target is indeed
	/// missing from the frame (say, out of sight)
	// update(InputArray, Rect2d &)(InputArray, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:547
	// ("cv::Tracker::update", vec![(pred!(mut, ["image", "boundingBox"], ["const cv::_InputArray*", "cv::Rect2d*"]), _)]),
	#[inline]
	fn update(&mut self, image: &impl ToInputArray, bounding_box: &mut core::Rect2d) -> Result<bool> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Tracker_update_const__InputArrayR_Rect2dR(self.as_raw_mut_Tracker(), image.as_raw__InputArray(), bounding_box, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:549
	// ("cv::Tracker::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Tracker_read_const_FileNodeR(self.as_raw_mut_Tracker(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Base abstract class for the long-term tracker:
// Tracker /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:524
pub struct Tracker {
	ptr: *mut c_void,
}

opencv_type_boxed! { Tracker }

impl Drop for Tracker {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_Tracker_delete(self.as_raw_mut_Tracker()) };
	}
}

unsafe impl Send for Tracker {}

impl core::AlgorithmTraitConst for Tracker {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for Tracker {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Tracker, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::tracking::TrackerTraitConst for Tracker {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerTrait for Tracker {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Tracker, crate::tracking::TrackerTraitConst, as_raw_Tracker, crate::tracking::TrackerTrait, as_raw_mut_Tracker }

impl Tracker {
}

boxed_cast_descendant! { Tracker, crate::tracking::TrackerBoosting, cv_Tracker_to_TrackerBoosting }

boxed_cast_descendant! { Tracker, crate::tracking::TrackerCSRT, cv_Tracker_to_TrackerCSRT }

boxed_cast_descendant! { Tracker, crate::tracking::TrackerGOTURN, cv_Tracker_to_TrackerGOTURN }

boxed_cast_descendant! { Tracker, crate::tracking::TrackerKCF, cv_Tracker_to_TrackerKCF }

boxed_cast_descendant! { Tracker, crate::tracking::TrackerMIL, cv_Tracker_to_TrackerMIL }

boxed_cast_descendant! { Tracker, crate::tracking::TrackerMOSSE, cv_Tracker_to_TrackerMOSSE }

boxed_cast_descendant! { Tracker, crate::tracking::TrackerMedianFlow, cv_Tracker_to_TrackerMedianFlow }

boxed_cast_descendant! { Tracker, crate::tracking::TrackerTLD, cv_Tracker_to_TrackerTLD }

boxed_cast_base! { Tracker, core::Algorithm, cv_Tracker_to_Algorithm }

impl std::fmt::Debug for Tracker {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Tracker")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerBoosting]
// TrackerBoosting /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1100
pub trait TrackerBoostingTraitConst: crate::tracking::TrackerTraitConst {
	fn as_raw_TrackerBoosting(&self) -> *const c_void;

}

/// Mutable methods for [crate::tracking::TrackerBoosting]
pub trait TrackerBoostingTrait: crate::tracking::TrackerBoostingTraitConst + crate::tracking::TrackerTrait {
	fn as_raw_mut_TrackerBoosting(&mut self) -> *mut c_void;

}

/// the Boosting tracker
///
/// This is a real-time object tracking based on a novel on-line version of the AdaBoost algorithm.
/// The classifier uses the surrounding background as negative examples in update step to avoid the
/// drifting problem. The implementation is based on [OLB](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_OLB) .
// TrackerBoosting /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1100
pub struct TrackerBoosting {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerBoosting }

impl Drop for TrackerBoosting {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerBoosting_delete(self.as_raw_mut_TrackerBoosting()) };
	}
}

unsafe impl Send for TrackerBoosting {}

impl core::AlgorithmTraitConst for TrackerBoosting {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for TrackerBoosting {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerBoosting, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::tracking::TrackerTraitConst for TrackerBoosting {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerTrait for TrackerBoosting {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerBoosting, crate::tracking::TrackerTraitConst, as_raw_Tracker, crate::tracking::TrackerTrait, as_raw_mut_Tracker }

impl crate::tracking::TrackerBoostingTraitConst for TrackerBoosting {
	#[inline] fn as_raw_TrackerBoosting(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerBoostingTrait for TrackerBoosting {
	#[inline] fn as_raw_mut_TrackerBoosting(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerBoosting, crate::tracking::TrackerBoostingTraitConst, as_raw_TrackerBoosting, crate::tracking::TrackerBoostingTrait, as_raw_mut_TrackerBoosting }

impl TrackerBoosting {
	/// Constructor
	/// ## Parameters
	/// * parameters: BOOSTING parameters TrackerBoosting::Params
	// create(const TrackerBoosting::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1125
	// ("cv::TrackerBoosting::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerBoosting::Params*"]), _)]),
	#[inline]
	pub fn create(parameters: &impl crate::tracking::TrackerBoosting_ParamsTraitConst) -> Result<core::Ptr<crate::tracking::TrackerBoosting>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerBoosting_create_const_ParamsR(parameters.as_raw_TrackerBoosting_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerBoosting>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1127
	// ("cv::TrackerBoosting::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_1() -> Result<core::Ptr<crate::tracking::TrackerBoosting>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerBoosting_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerBoosting>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TrackerBoosting, core::Algorithm, cv_TrackerBoosting_to_Algorithm }

boxed_cast_base! { TrackerBoosting, crate::tracking::Tracker, cv_TrackerBoosting_to_Tracker }

impl std::fmt::Debug for TrackerBoosting {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerBoosting")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerBoosting_Params]
// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1103
pub trait TrackerBoosting_ParamsTraitConst {
	fn as_raw_TrackerBoosting_Params(&self) -> *const c_void;

	/// the number of classifiers to use in a OnlineBoosting algorithm
	// cv::TrackerBoosting::Params::numClassifiers() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1106
	// ("cv::TrackerBoosting::Params::numClassifiers", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_classifiers(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerBoosting_Params_propNumClassifiers_const(self.as_raw_TrackerBoosting_Params()) };
		ret
	}

	/// search region parameters to use in a OnlineBoosting algorithm
	// cv::TrackerBoosting::Params::samplerOverlap() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1107
	// ("cv::TrackerBoosting::Params::samplerOverlap", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn sampler_overlap(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerBoosting_Params_propSamplerOverlap_const(self.as_raw_TrackerBoosting_Params()) };
		ret
	}

	/// search region parameters to use in a OnlineBoosting algorithm
	// cv::TrackerBoosting::Params::samplerSearchFactor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1108
	// ("cv::TrackerBoosting::Params::samplerSearchFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn sampler_search_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerBoosting_Params_propSamplerSearchFactor_const(self.as_raw_TrackerBoosting_Params()) };
		ret
	}

	/// the initial iterations
	// cv::TrackerBoosting::Params::iterationInit() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1109
	// ("cv::TrackerBoosting::Params::iterationInit", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn iteration_init(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerBoosting_Params_propIterationInit_const(self.as_raw_TrackerBoosting_Params()) };
		ret
	}

	/// # features
	// cv::TrackerBoosting::Params::featureSetNumFeatures() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1110
	// ("cv::TrackerBoosting::Params::featureSetNumFeatures", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn feature_set_num_features(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerBoosting_Params_propFeatureSetNumFeatures_const(self.as_raw_TrackerBoosting_Params()) };
		ret
	}

	/// \brief Write parameters to a file
	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1119
	// ("cv::TrackerBoosting::Params::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerBoosting_Params_write_const_FileStorageR(self.as_raw_TrackerBoosting_Params(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::TrackerBoosting_Params]
pub trait TrackerBoosting_ParamsTrait: crate::tracking::TrackerBoosting_ParamsTraitConst {
	fn as_raw_mut_TrackerBoosting_Params(&mut self) -> *mut c_void;

	/// the number of classifiers to use in a OnlineBoosting algorithm
	// cv::TrackerBoosting::Params::setNumClassifiers(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1106
	// ("cv::TrackerBoosting::Params::setNumClassifiers", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_num_classifiers(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerBoosting_Params_propNumClassifiers_const_int(self.as_raw_mut_TrackerBoosting_Params(), val) };
		ret
	}

	/// search region parameters to use in a OnlineBoosting algorithm
	// cv::TrackerBoosting::Params::setSamplerOverlap(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1107
	// ("cv::TrackerBoosting::Params::setSamplerOverlap", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_sampler_overlap(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerBoosting_Params_propSamplerOverlap_const_float(self.as_raw_mut_TrackerBoosting_Params(), val) };
		ret
	}

	/// search region parameters to use in a OnlineBoosting algorithm
	// cv::TrackerBoosting::Params::setSamplerSearchFactor(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1108
	// ("cv::TrackerBoosting::Params::setSamplerSearchFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_sampler_search_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerBoosting_Params_propSamplerSearchFactor_const_float(self.as_raw_mut_TrackerBoosting_Params(), val) };
		ret
	}

	/// the initial iterations
	// cv::TrackerBoosting::Params::setIterationInit(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1109
	// ("cv::TrackerBoosting::Params::setIterationInit", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_iteration_init(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerBoosting_Params_propIterationInit_const_int(self.as_raw_mut_TrackerBoosting_Params(), val) };
		ret
	}

	/// # features
	// cv::TrackerBoosting::Params::setFeatureSetNumFeatures(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1110
	// ("cv::TrackerBoosting::Params::setFeatureSetNumFeatures", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_feature_set_num_features(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerBoosting_Params_propFeatureSetNumFeatures_const_int(self.as_raw_mut_TrackerBoosting_Params(), val) };
		ret
	}

	/// \brief Read parameters from a file
	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1114
	// ("cv::TrackerBoosting::Params::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerBoosting_Params_read_const_FileNodeR(self.as_raw_mut_TrackerBoosting_Params(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1103
pub struct TrackerBoosting_Params {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerBoosting_Params }

impl Drop for TrackerBoosting_Params {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerBoosting_Params_delete(self.as_raw_mut_TrackerBoosting_Params()) };
	}
}

unsafe impl Send for TrackerBoosting_Params {}

impl crate::tracking::TrackerBoosting_ParamsTraitConst for TrackerBoosting_Params {
	#[inline] fn as_raw_TrackerBoosting_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerBoosting_ParamsTrait for TrackerBoosting_Params {
	#[inline] fn as_raw_mut_TrackerBoosting_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerBoosting_Params, crate::tracking::TrackerBoosting_ParamsTraitConst, as_raw_TrackerBoosting_Params, crate::tracking::TrackerBoosting_ParamsTrait, as_raw_mut_TrackerBoosting_Params }

impl TrackerBoosting_Params {
	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1105
	// ("cv::TrackerBoosting::Params::Params", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::tracking::TrackerBoosting_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerBoosting_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerBoosting_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for TrackerBoosting_Params {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerBoosting_Params")
			.field("num_classifiers", &crate::tracking::TrackerBoosting_ParamsTraitConst::num_classifiers(self))
			.field("sampler_overlap", &crate::tracking::TrackerBoosting_ParamsTraitConst::sampler_overlap(self))
			.field("sampler_search_factor", &crate::tracking::TrackerBoosting_ParamsTraitConst::sampler_search_factor(self))
			.field("iteration_init", &crate::tracking::TrackerBoosting_ParamsTraitConst::iteration_init(self))
			.field("feature_set_num_features", &crate::tracking::TrackerBoosting_ParamsTraitConst::feature_set_num_features(self))
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerCSRT]
// TrackerCSRT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1481
pub trait TrackerCSRTTraitConst: crate::tracking::TrackerTraitConst {
	fn as_raw_TrackerCSRT(&self) -> *const c_void;

}

/// Mutable methods for [crate::tracking::TrackerCSRT]
pub trait TrackerCSRTTrait: crate::tracking::TrackerCSRTTraitConst + crate::tracking::TrackerTrait {
	fn as_raw_mut_TrackerCSRT(&mut self) -> *mut c_void;

	// setInitialMask(const Mat)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1540
	// ("cv::TrackerCSRT::setInitialMask", vec![(pred!(mut, ["mask"], ["const cv::Mat"]), _)]),
	#[inline]
	fn set_initial_mask(&mut self, mask: impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerCSRT_setInitialMask_const_Mat(self.as_raw_mut_TrackerCSRT(), mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// ********************************* CSRT ***********************************
/// the CSRT tracker
///
/// The implementation is based on [Lukezic_IJCV2018](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_Lukezic_IJCV2018) Discriminative Correlation Filter with Channel and Spatial Reliability
// TrackerCSRT /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1481
pub struct TrackerCSRT {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerCSRT }

impl Drop for TrackerCSRT {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerCSRT_delete(self.as_raw_mut_TrackerCSRT()) };
	}
}

unsafe impl Send for TrackerCSRT {}

impl core::AlgorithmTraitConst for TrackerCSRT {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for TrackerCSRT {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerCSRT, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::tracking::TrackerTraitConst for TrackerCSRT {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerTrait for TrackerCSRT {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerCSRT, crate::tracking::TrackerTraitConst, as_raw_Tracker, crate::tracking::TrackerTrait, as_raw_mut_Tracker }

impl crate::tracking::TrackerCSRTTraitConst for TrackerCSRT {
	#[inline] fn as_raw_TrackerCSRT(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerCSRTTrait for TrackerCSRT {
	#[inline] fn as_raw_mut_TrackerCSRT(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerCSRT, crate::tracking::TrackerCSRTTraitConst, as_raw_TrackerCSRT, crate::tracking::TrackerCSRTTrait, as_raw_mut_TrackerCSRT }

impl TrackerCSRT {
	/// Constructor
	/// ## Parameters
	/// * parameters: CSRT parameters TrackerCSRT::Params
	// create(const TrackerCSRT::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1536
	// ("cv::TrackerCSRT::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerCSRT::Params*"]), _)]),
	#[inline]
	pub fn create(parameters: &impl crate::tracking::TrackerCSRT_ParamsTraitConst) -> Result<core::Ptr<crate::tracking::TrackerCSRT>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerCSRT_create_const_ParamsR(parameters.as_raw_TrackerCSRT_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerCSRT>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1538
	// ("cv::TrackerCSRT::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_1() -> Result<core::Ptr<crate::tracking::TrackerCSRT>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerCSRT_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerCSRT>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TrackerCSRT, core::Algorithm, cv_TrackerCSRT_to_Algorithm }

boxed_cast_base! { TrackerCSRT, crate::tracking::Tracker, cv_TrackerCSRT_to_Tracker }

impl std::fmt::Debug for TrackerCSRT {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerCSRT")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerCSRT_Params]
// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1484
pub trait TrackerCSRT_ParamsTraitConst {
	fn as_raw_TrackerCSRT_Params(&self) -> *const c_void;

	// cv::TrackerCSRT::Params::use_hog() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1501
	// ("cv::TrackerCSRT::Params::use_hog", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn use_hog(&self) -> bool {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_hog_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::use_color_names() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1502
	// ("cv::TrackerCSRT::Params::use_color_names", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn use_color_names(&self) -> bool {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_color_names_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::use_gray() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1503
	// ("cv::TrackerCSRT::Params::use_gray", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn use_gray(&self) -> bool {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_gray_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::use_rgb() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1504
	// ("cv::TrackerCSRT::Params::use_rgb", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn use_rgb(&self) -> bool {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_rgb_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::use_channel_weights() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1505
	// ("cv::TrackerCSRT::Params::use_channel_weights", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn use_channel_weights(&self) -> bool {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_channel_weights_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::use_segmentation() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1506
	// ("cv::TrackerCSRT::Params::use_segmentation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn use_segmentation(&self) -> bool {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_segmentation_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	/// Window function: "hann", "cheb", "kaiser"
	// cv::TrackerCSRT::Params::window_function() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1508
	// ("cv::TrackerCSRT::Params::window_function", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn window_function(&self) -> String {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propWindow_function_const(self.as_raw_TrackerCSRT_Params()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}

	// cv::TrackerCSRT::Params::kaiser_alpha() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1509
	// ("cv::TrackerCSRT::Params::kaiser_alpha", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn kaiser_alpha(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propKaiser_alpha_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::cheb_attenuation() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1510
	// ("cv::TrackerCSRT::Params::cheb_attenuation", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn cheb_attenuation(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propCheb_attenuation_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::template_size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1512
	// ("cv::TrackerCSRT::Params::template_size", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn template_size(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propTemplate_size_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::gsl_sigma() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1513
	// ("cv::TrackerCSRT::Params::gsl_sigma", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn gsl_sigma(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propGsl_sigma_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::hog_orientations() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1514
	// ("cv::TrackerCSRT::Params::hog_orientations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn hog_orientations(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propHog_orientations_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::hog_clip() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1515
	// ("cv::TrackerCSRT::Params::hog_clip", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn hog_clip(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propHog_clip_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::padding() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1516
	// ("cv::TrackerCSRT::Params::padding", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn padding(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propPadding_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::filter_lr() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1517
	// ("cv::TrackerCSRT::Params::filter_lr", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn filter_lr(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propFilter_lr_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::weights_lr() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1518
	// ("cv::TrackerCSRT::Params::weights_lr", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn weights_lr(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propWeights_lr_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::num_hog_channels_used() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1519
	// ("cv::TrackerCSRT::Params::num_hog_channels_used", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_hog_channels_used(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propNum_hog_channels_used_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::admm_iterations() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1520
	// ("cv::TrackerCSRT::Params::admm_iterations", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn admm_iterations(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propAdmm_iterations_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::histogram_bins() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1521
	// ("cv::TrackerCSRT::Params::histogram_bins", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn histogram_bins(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propHistogram_bins_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::histogram_lr() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1522
	// ("cv::TrackerCSRT::Params::histogram_lr", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn histogram_lr(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propHistogram_lr_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::background_ratio() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1523
	// ("cv::TrackerCSRT::Params::background_ratio", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn background_ratio(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propBackground_ratio_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::number_of_scales() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1524
	// ("cv::TrackerCSRT::Params::number_of_scales", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn number_of_scales(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propNumber_of_scales_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::scale_sigma_factor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1525
	// ("cv::TrackerCSRT::Params::scale_sigma_factor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn scale_sigma_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propScale_sigma_factor_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::scale_model_max_area() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1526
	// ("cv::TrackerCSRT::Params::scale_model_max_area", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn scale_model_max_area(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propScale_model_max_area_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::scale_lr() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1527
	// ("cv::TrackerCSRT::Params::scale_lr", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn scale_lr(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propScale_lr_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	// cv::TrackerCSRT::Params::scale_step() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1528
	// ("cv::TrackerCSRT::Params::scale_step", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn scale_step(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propScale_step_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	/// we lost the target, if the psr is lower than this.
	// cv::TrackerCSRT::Params::psr_threshold() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1530
	// ("cv::TrackerCSRT::Params::psr_threshold", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn psr_threshold(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propPsr_threshold_const(self.as_raw_TrackerCSRT_Params()) };
		ret
	}

	/// \brief Write parameters to a file
	// write(cv::FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1499
	// ("cv::TrackerCSRT::Params::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerCSRT_Params_write_const_FileStorageR(self.as_raw_TrackerCSRT_Params(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::TrackerCSRT_Params]
pub trait TrackerCSRT_ParamsTrait: crate::tracking::TrackerCSRT_ParamsTraitConst {
	fn as_raw_mut_TrackerCSRT_Params(&mut self) -> *mut c_void;

	// cv::TrackerCSRT::Params::setUse_hog(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1501
	// ("cv::TrackerCSRT::Params::setUse_hog", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_use_hog(&mut self, val: bool) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_hog_const_bool(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setUse_color_names(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1502
	// ("cv::TrackerCSRT::Params::setUse_color_names", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_use_color_names(&mut self, val: bool) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_color_names_const_bool(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setUse_gray(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1503
	// ("cv::TrackerCSRT::Params::setUse_gray", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_use_gray(&mut self, val: bool) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_gray_const_bool(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setUse_rgb(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1504
	// ("cv::TrackerCSRT::Params::setUse_rgb", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_use_rgb(&mut self, val: bool) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_rgb_const_bool(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setUse_channel_weights(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1505
	// ("cv::TrackerCSRT::Params::setUse_channel_weights", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_use_channel_weights(&mut self, val: bool) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_channel_weights_const_bool(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setUse_segmentation(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1506
	// ("cv::TrackerCSRT::Params::setUse_segmentation", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_use_segmentation(&mut self, val: bool) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propUse_segmentation_const_bool(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	/// Window function: "hann", "cheb", "kaiser"
	// cv::TrackerCSRT::Params::setWindow_function(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1508
	// ("cv::TrackerCSRT::Params::setWindow_function", vec![(pred!(mut, ["val"], ["const std::string"]), _)]),
	#[inline]
	fn set_window_function(&mut self, val: &str) {
		extern_container_arg!(nofail val);
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propWindow_function_const_string(self.as_raw_mut_TrackerCSRT_Params(), val.opencv_as_extern()) };
		ret
	}

	// cv::TrackerCSRT::Params::setKaiser_alpha(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1509
	// ("cv::TrackerCSRT::Params::setKaiser_alpha", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_kaiser_alpha(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propKaiser_alpha_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setCheb_attenuation(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1510
	// ("cv::TrackerCSRT::Params::setCheb_attenuation", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_cheb_attenuation(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propCheb_attenuation_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setTemplate_size(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1512
	// ("cv::TrackerCSRT::Params::setTemplate_size", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_template_size(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propTemplate_size_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setGsl_sigma(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1513
	// ("cv::TrackerCSRT::Params::setGsl_sigma", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_gsl_sigma(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propGsl_sigma_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setHog_orientations(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1514
	// ("cv::TrackerCSRT::Params::setHog_orientations", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_hog_orientations(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propHog_orientations_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setHog_clip(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1515
	// ("cv::TrackerCSRT::Params::setHog_clip", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_hog_clip(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propHog_clip_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setPadding(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1516
	// ("cv::TrackerCSRT::Params::setPadding", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_padding(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propPadding_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setFilter_lr(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1517
	// ("cv::TrackerCSRT::Params::setFilter_lr", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_filter_lr(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propFilter_lr_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setWeights_lr(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1518
	// ("cv::TrackerCSRT::Params::setWeights_lr", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_weights_lr(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propWeights_lr_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setNum_hog_channels_used(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1519
	// ("cv::TrackerCSRT::Params::setNum_hog_channels_used", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_num_hog_channels_used(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propNum_hog_channels_used_const_int(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setAdmm_iterations(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1520
	// ("cv::TrackerCSRT::Params::setAdmm_iterations", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_admm_iterations(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propAdmm_iterations_const_int(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setHistogram_bins(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1521
	// ("cv::TrackerCSRT::Params::setHistogram_bins", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_histogram_bins(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propHistogram_bins_const_int(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setHistogram_lr(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1522
	// ("cv::TrackerCSRT::Params::setHistogram_lr", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_histogram_lr(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propHistogram_lr_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setBackground_ratio(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1523
	// ("cv::TrackerCSRT::Params::setBackground_ratio", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_background_ratio(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propBackground_ratio_const_int(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setNumber_of_scales(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1524
	// ("cv::TrackerCSRT::Params::setNumber_of_scales", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_number_of_scales(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propNumber_of_scales_const_int(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setScale_sigma_factor(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1525
	// ("cv::TrackerCSRT::Params::setScale_sigma_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_scale_sigma_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propScale_sigma_factor_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setScale_model_max_area(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1526
	// ("cv::TrackerCSRT::Params::setScale_model_max_area", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_scale_model_max_area(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propScale_model_max_area_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setScale_lr(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1527
	// ("cv::TrackerCSRT::Params::setScale_lr", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_scale_lr(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propScale_lr_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	// cv::TrackerCSRT::Params::setScale_step(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1528
	// ("cv::TrackerCSRT::Params::setScale_step", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_scale_step(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propScale_step_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	/// we lost the target, if the psr is lower than this.
	// cv::TrackerCSRT::Params::setPsr_threshold(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1530
	// ("cv::TrackerCSRT::Params::setPsr_threshold", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_psr_threshold(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerCSRT_Params_propPsr_threshold_const_float(self.as_raw_mut_TrackerCSRT_Params(), val) };
		ret
	}

	/// \brief Read parameters from a file
	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1494
	// ("cv::TrackerCSRT::Params::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read(&mut self, unnamed: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerCSRT_Params_read_const_FileNodeR(self.as_raw_mut_TrackerCSRT_Params(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1484
pub struct TrackerCSRT_Params {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerCSRT_Params }

impl Drop for TrackerCSRT_Params {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerCSRT_Params_delete(self.as_raw_mut_TrackerCSRT_Params()) };
	}
}

unsafe impl Send for TrackerCSRT_Params {}

impl crate::tracking::TrackerCSRT_ParamsTraitConst for TrackerCSRT_Params {
	#[inline] fn as_raw_TrackerCSRT_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerCSRT_ParamsTrait for TrackerCSRT_Params {
	#[inline] fn as_raw_mut_TrackerCSRT_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerCSRT_Params, crate::tracking::TrackerCSRT_ParamsTraitConst, as_raw_TrackerCSRT_Params, crate::tracking::TrackerCSRT_ParamsTrait, as_raw_mut_TrackerCSRT_Params }

impl TrackerCSRT_Params {
	/// \brief Constructor
	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1489
	// ("cv::TrackerCSRT::Params::Params", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::tracking::TrackerCSRT_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerCSRT_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerCSRT_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for TrackerCSRT_Params {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerCSRT_Params")
			.field("use_hog", &crate::tracking::TrackerCSRT_ParamsTraitConst::use_hog(self))
			.field("use_color_names", &crate::tracking::TrackerCSRT_ParamsTraitConst::use_color_names(self))
			.field("use_gray", &crate::tracking::TrackerCSRT_ParamsTraitConst::use_gray(self))
			.field("use_rgb", &crate::tracking::TrackerCSRT_ParamsTraitConst::use_rgb(self))
			.field("use_channel_weights", &crate::tracking::TrackerCSRT_ParamsTraitConst::use_channel_weights(self))
			.field("use_segmentation", &crate::tracking::TrackerCSRT_ParamsTraitConst::use_segmentation(self))
			.field("window_function", &crate::tracking::TrackerCSRT_ParamsTraitConst::window_function(self))
			.field("kaiser_alpha", &crate::tracking::TrackerCSRT_ParamsTraitConst::kaiser_alpha(self))
			.field("cheb_attenuation", &crate::tracking::TrackerCSRT_ParamsTraitConst::cheb_attenuation(self))
			.field("template_size", &crate::tracking::TrackerCSRT_ParamsTraitConst::template_size(self))
			.field("gsl_sigma", &crate::tracking::TrackerCSRT_ParamsTraitConst::gsl_sigma(self))
			.field("hog_orientations", &crate::tracking::TrackerCSRT_ParamsTraitConst::hog_orientations(self))
			.field("hog_clip", &crate::tracking::TrackerCSRT_ParamsTraitConst::hog_clip(self))
			.field("padding", &crate::tracking::TrackerCSRT_ParamsTraitConst::padding(self))
			.field("filter_lr", &crate::tracking::TrackerCSRT_ParamsTraitConst::filter_lr(self))
			.field("weights_lr", &crate::tracking::TrackerCSRT_ParamsTraitConst::weights_lr(self))
			.field("num_hog_channels_used", &crate::tracking::TrackerCSRT_ParamsTraitConst::num_hog_channels_used(self))
			.field("admm_iterations", &crate::tracking::TrackerCSRT_ParamsTraitConst::admm_iterations(self))
			.field("histogram_bins", &crate::tracking::TrackerCSRT_ParamsTraitConst::histogram_bins(self))
			.field("histogram_lr", &crate::tracking::TrackerCSRT_ParamsTraitConst::histogram_lr(self))
			.field("background_ratio", &crate::tracking::TrackerCSRT_ParamsTraitConst::background_ratio(self))
			.field("number_of_scales", &crate::tracking::TrackerCSRT_ParamsTraitConst::number_of_scales(self))
			.field("scale_sigma_factor", &crate::tracking::TrackerCSRT_ParamsTraitConst::scale_sigma_factor(self))
			.field("scale_model_max_area", &crate::tracking::TrackerCSRT_ParamsTraitConst::scale_model_max_area(self))
			.field("scale_lr", &crate::tracking::TrackerCSRT_ParamsTraitConst::scale_lr(self))
			.field("scale_step", &crate::tracking::TrackerCSRT_ParamsTraitConst::scale_step(self))
			.field("psr_threshold", &crate::tracking::TrackerCSRT_ParamsTraitConst::psr_threshold(self))
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerFeature]
// TrackerFeature /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:71
pub trait TrackerFeatureTraitConst {
	fn as_raw_TrackerFeature(&self) -> *const c_void;

	/// Get the name of the specific TrackerFeature
	// getClassName()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:107
	// ("cv::TrackerFeature::getClassName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_class_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeature_getClassName_const(self.as_raw_TrackerFeature(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::TrackerFeature]
pub trait TrackerFeatureTrait: crate::tracking::TrackerFeatureTraitConst {
	fn as_raw_mut_TrackerFeature(&mut self) -> *mut c_void;

	/// Compute the features in the images collection
	/// ## Parameters
	/// * images: The images
	/// * response: The output response
	// compute(const std::vector<Mat> &, Mat &)(CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:80
	// ("cv::TrackerFeature::compute", vec![(pred!(mut, ["images", "response"], ["const std::vector<cv::Mat>*", "cv::Mat*"]), _)]),
	#[inline]
	fn compute(&mut self, images: &core::Vector<core::Mat>, response: &mut impl core::MatTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeature_compute_const_vectorLMatGR_MatR(self.as_raw_mut_TrackerFeature(), images.as_raw_VectorOfMat(), response.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Identify most effective features
	/// ## Parameters
	/// * response: Collection of response for the specific TrackerFeature
	/// * npoints: Max number of features
	///
	///
	/// Note: This method modifies the response parameter
	// selection(Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:103
	// ("cv::TrackerFeature::selection", vec![(pred!(mut, ["response", "npoints"], ["cv::Mat*", "int"]), _)]),
	#[inline]
	fn selection(&mut self, response: &mut impl core::MatTrait, npoints: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeature_selection_MatR_int(self.as_raw_mut_TrackerFeature(), response.as_raw_mut_Mat(), npoints, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Abstract base class for TrackerFeature that represents the feature.
// TrackerFeature /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:71
pub struct TrackerFeature {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerFeature }

impl Drop for TrackerFeature {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerFeature_delete(self.as_raw_mut_TrackerFeature()) };
	}
}

unsafe impl Send for TrackerFeature {}

impl crate::tracking::TrackerFeatureTraitConst for TrackerFeature {
	#[inline] fn as_raw_TrackerFeature(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerFeatureTrait for TrackerFeature {
	#[inline] fn as_raw_mut_TrackerFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerFeature, crate::tracking::TrackerFeatureTraitConst, as_raw_TrackerFeature, crate::tracking::TrackerFeatureTrait, as_raw_mut_TrackerFeature }

impl TrackerFeature {
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
	// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:95
	// ("cv::TrackerFeature::create", vec![(pred!(mut, ["trackerFeatureType"], ["const cv::String*"]), _)]),
	#[inline]
	pub fn create(tracker_feature_type: &str) -> Result<core::Ptr<crate::tracking::TrackerFeature>> {
		extern_container_arg!(tracker_feature_type);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeature_create_const_StringR(tracker_feature_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerFeature>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_descendant! { TrackerFeature, crate::tracking::TrackerFeatureFeature2d, cv_TrackerFeature_to_TrackerFeatureFeature2d }

boxed_cast_descendant! { TrackerFeature, crate::tracking::TrackerFeatureHAAR, cv_TrackerFeature_to_TrackerFeatureHAAR }

boxed_cast_descendant! { TrackerFeature, crate::tracking::TrackerFeatureHOG, cv_TrackerFeature_to_TrackerFeatureHOG }

boxed_cast_descendant! { TrackerFeature, crate::tracking::TrackerFeatureLBP, cv_TrackerFeature_to_TrackerFeatureLBP }

impl std::fmt::Debug for TrackerFeature {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerFeature")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerFeatureFeature2d]
// TrackerFeatureFeature2d /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:931
pub trait TrackerFeatureFeature2dTraitConst: crate::tracking::TrackerFeatureTraitConst {
	fn as_raw_TrackerFeatureFeature2d(&self) -> *const c_void;

}

/// Mutable methods for [crate::tracking::TrackerFeatureFeature2d]
pub trait TrackerFeatureFeature2dTrait: crate::tracking::TrackerFeatureFeature2dTraitConst + crate::tracking::TrackerFeatureTrait {
	fn as_raw_mut_TrackerFeatureFeature2d(&mut self) -> *mut c_void;

	// selection(Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:944
	// ("cv::TrackerFeatureFeature2d::selection", vec![(pred!(mut, ["response", "npoints"], ["cv::Mat*", "int"]), _)]),
	#[inline]
	fn selection(&mut self, response: &mut impl core::MatTrait, npoints: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureFeature2d_selection_MatR_int(self.as_raw_mut_TrackerFeatureFeature2d(), response.as_raw_mut_Mat(), npoints, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// \brief TrackerFeature based on Feature2D
// TrackerFeatureFeature2d /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:931
pub struct TrackerFeatureFeature2d {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerFeatureFeature2d }

impl Drop for TrackerFeatureFeature2d {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerFeatureFeature2d_delete(self.as_raw_mut_TrackerFeatureFeature2d()) };
	}
}

unsafe impl Send for TrackerFeatureFeature2d {}

impl crate::tracking::TrackerFeatureTraitConst for TrackerFeatureFeature2d {
	#[inline] fn as_raw_TrackerFeature(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerFeatureTrait for TrackerFeatureFeature2d {
	#[inline] fn as_raw_mut_TrackerFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerFeatureFeature2d, crate::tracking::TrackerFeatureTraitConst, as_raw_TrackerFeature, crate::tracking::TrackerFeatureTrait, as_raw_mut_TrackerFeature }

impl crate::tracking::TrackerFeatureFeature2dTraitConst for TrackerFeatureFeature2d {
	#[inline] fn as_raw_TrackerFeatureFeature2d(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerFeatureFeature2dTrait for TrackerFeatureFeature2d {
	#[inline] fn as_raw_mut_TrackerFeatureFeature2d(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerFeatureFeature2d, crate::tracking::TrackerFeatureFeature2dTraitConst, as_raw_TrackerFeatureFeature2d, crate::tracking::TrackerFeatureFeature2dTrait, as_raw_mut_TrackerFeatureFeature2d }

impl TrackerFeatureFeature2d {
	/// \brief Constructor
	/// \param detectorType string of FeatureDetector
	/// \param descriptorType string of DescriptorExtractor
	// TrackerFeatureFeature2d(String, String)(InString, InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:940
	// ("cv::TrackerFeatureFeature2d::TrackerFeatureFeature2d", vec![(pred!(mut, ["detectorType", "descriptorType"], ["cv::String", "cv::String"]), _)]),
	#[inline]
	pub fn new(detector_type: &str, descriptor_type: &str) -> Result<crate::tracking::TrackerFeatureFeature2d> {
		extern_container_arg!(detector_type);
		extern_container_arg!(descriptor_type);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureFeature2d_TrackerFeatureFeature2d_String_String(detector_type.opencv_as_extern(), descriptor_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerFeatureFeature2d::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TrackerFeatureFeature2d, crate::tracking::TrackerFeature, cv_TrackerFeatureFeature2d_to_TrackerFeature }

impl std::fmt::Debug for TrackerFeatureFeature2d {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerFeatureFeature2d")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerFeatureHAAR]
// TrackerFeatureHAAR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:977
pub trait TrackerFeatureHAARTraitConst: crate::tracking::TrackerFeatureTraitConst {
	fn as_raw_TrackerFeatureHAAR(&self) -> *const c_void;

}

/// Mutable methods for [crate::tracking::TrackerFeatureHAAR]
pub trait TrackerFeatureHAARTrait: crate::tracking::TrackerFeatureHAARTraitConst + crate::tracking::TrackerFeatureTrait {
	fn as_raw_mut_TrackerFeatureHAAR(&mut self) -> *mut c_void;

	/// Compute the features only for the selected indices in the images collection
	/// ## Parameters
	/// * selFeatures: indices of selected features
	/// * images: The images
	/// * response: Collection of response for the specific TrackerFeature
	// extractSelected(const std::vector<int>, const std::vector<Mat> &, Mat &)(CppPassByVoidPtr, CppPassByVoidPtr, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1000
	// ("cv::TrackerFeatureHAAR::extractSelected", vec![(pred!(mut, ["selFeatures", "images", "response"], ["const std::vector<int>", "const std::vector<cv::Mat>*", "cv::Mat*"]), _)]),
	#[inline]
	fn extract_selected(&mut self, sel_features: core::Vector<i32>, images: &core::Vector<core::Mat>, response: &mut impl core::MatTrait) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureHAAR_extractSelected_const_vectorLintG_const_vectorLMatGR_MatR(self.as_raw_mut_TrackerFeatureHAAR(), sel_features.as_raw_VectorOfi32(), images.as_raw_VectorOfMat(), response.as_raw_mut_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Identify most effective features
	/// ## Parameters
	/// * response: Collection of response for the specific TrackerFeature
	/// * npoints: Max number of features
	///
	///
	/// Note: This method modifies the response parameter
	// selection(Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1008
	// ("cv::TrackerFeatureHAAR::selection", vec![(pred!(mut, ["response", "npoints"], ["cv::Mat*", "int"]), _)]),
	#[inline]
	fn selection(&mut self, response: &mut impl core::MatTrait, npoints: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureHAAR_selection_MatR_int(self.as_raw_mut_TrackerFeatureHAAR(), response.as_raw_mut_Mat(), npoints, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Swap the feature in position source with the feature in position target
	/// ## Parameters
	/// * source: The source position
	/// * target: The target position
	// swapFeature(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1014
	// ("cv::TrackerFeatureHAAR::swapFeature", vec![(pred!(mut, ["source", "target"], ["int", "int"]), _)]),
	#[inline]
	fn swap_feature(&mut self, source: i32, target: i32) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureHAAR_swapFeature_int_int(self.as_raw_mut_TrackerFeatureHAAR(), source, target, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Swap the feature in position id with the feature input
	/// ## Parameters
	/// * id: The position
	/// * feature: The feature
	// swapFeature(int, CvHaarEvaluator::FeatureHaar &)(Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1020
	// ("cv::TrackerFeatureHAAR::swapFeature", vec![(pred!(mut, ["id", "feature"], ["int", "cv::CvHaarEvaluator::FeatureHaar*"]), _)]),
	#[inline]
	fn swap_feature_1(&mut self, id: i32, feature: &mut impl crate::tracking::CvHaarEvaluator_FeatureHaarTrait) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureHAAR_swapFeature_int_FeatureHaarR(self.as_raw_mut_TrackerFeatureHAAR(), id, feature.as_raw_mut_CvHaarEvaluator_FeatureHaar(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get the feature in position id
	/// ## Parameters
	/// * id: The position
	// getFeatureAt(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1025
	// ("cv::TrackerFeatureHAAR::getFeatureAt", vec![(pred!(mut, ["id"], ["int"]), _)]),
	#[inline]
	fn get_feature_at(&mut self, id: i32) -> Result<crate::tracking::CvHaarEvaluator_FeatureHaar> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureHAAR_getFeatureAt_int(self.as_raw_mut_TrackerFeatureHAAR(), id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::CvHaarEvaluator_FeatureHaar::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// TrackerFeature based on HAAR features, used by TrackerMIL and many others algorithms
///
/// Note: HAAR features implementation is copied from apps/traincascade and modified according to MIL
// TrackerFeatureHAAR /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:977
pub struct TrackerFeatureHAAR {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerFeatureHAAR }

impl Drop for TrackerFeatureHAAR {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerFeatureHAAR_delete(self.as_raw_mut_TrackerFeatureHAAR()) };
	}
}

unsafe impl Send for TrackerFeatureHAAR {}

impl crate::tracking::TrackerFeatureTraitConst for TrackerFeatureHAAR {
	#[inline] fn as_raw_TrackerFeature(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerFeatureTrait for TrackerFeatureHAAR {
	#[inline] fn as_raw_mut_TrackerFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerFeatureHAAR, crate::tracking::TrackerFeatureTraitConst, as_raw_TrackerFeature, crate::tracking::TrackerFeatureTrait, as_raw_mut_TrackerFeature }

impl crate::tracking::TrackerFeatureHAARTraitConst for TrackerFeatureHAAR {
	#[inline] fn as_raw_TrackerFeatureHAAR(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerFeatureHAARTrait for TrackerFeatureHAAR {
	#[inline] fn as_raw_mut_TrackerFeatureHAAR(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerFeatureHAAR, crate::tracking::TrackerFeatureHAARTraitConst, as_raw_TrackerFeatureHAAR, crate::tracking::TrackerFeatureHAARTrait, as_raw_mut_TrackerFeatureHAAR }

impl TrackerFeatureHAAR {
	/// Constructor
	/// ## Parameters
	/// * parameters: TrackerFeatureHAAR parameters TrackerFeatureHAAR::Params
	///
	/// ## C++ default parameters
	/// * parameters: TrackerFeatureHAAR::Params()
	// TrackerFeatureHAAR(const TrackerFeatureHAAR::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:991
	// ("cv::TrackerFeatureHAAR::TrackerFeatureHAAR", vec![(pred!(mut, ["parameters"], ["const cv::TrackerFeatureHAAR::Params*"]), _)]),
	#[inline]
	pub fn new(parameters: &impl crate::tracking::TrackerFeatureHAAR_ParamsTraitConst) -> Result<crate::tracking::TrackerFeatureHAAR> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureHAAR_TrackerFeatureHAAR_const_ParamsR(parameters.as_raw_TrackerFeatureHAAR_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerFeatureHAAR::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor
	/// ## Parameters
	/// * parameters: TrackerFeatureHAAR parameters TrackerFeatureHAAR::Params
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * parameters: TrackerFeatureHAAR::Params()
	// cv::TrackerFeatureHAAR::TrackerFeatureHAAR() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:991
	// ("cv::TrackerFeatureHAAR::TrackerFeatureHAAR", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::tracking::TrackerFeatureHAAR> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureHAAR_TrackerFeatureHAAR(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerFeatureHAAR::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TrackerFeatureHAAR, crate::tracking::TrackerFeature, cv_TrackerFeatureHAAR_to_TrackerFeature }

impl std::fmt::Debug for TrackerFeatureHAAR {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerFeatureHAAR")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerFeatureHAAR_Params]
// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:980
pub trait TrackerFeatureHAAR_ParamsTraitConst {
	fn as_raw_TrackerFeatureHAAR_Params(&self) -> *const c_void;

	/// # of rects
	// cv::TrackerFeatureHAAR::Params::numFeatures() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:983
	// ("cv::TrackerFeatureHAAR::Params::numFeatures", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn num_features(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerFeatureHAAR_Params_propNumFeatures_const(self.as_raw_TrackerFeatureHAAR_Params()) };
		ret
	}

	/// rect size
	// cv::TrackerFeatureHAAR::Params::rectSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:984
	// ("cv::TrackerFeatureHAAR::Params::rectSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn rect_size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureHAAR_Params_propRectSize_const(self.as_raw_TrackerFeatureHAAR_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// true if input images are integral, false otherwise
	// cv::TrackerFeatureHAAR::Params::isIntegral() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:985
	// ("cv::TrackerFeatureHAAR::Params::isIntegral", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn is_integral(&self) -> bool {
		let ret = unsafe { sys::cv_TrackerFeatureHAAR_Params_propIsIntegral_const(self.as_raw_TrackerFeatureHAAR_Params()) };
		ret
	}

}

/// Mutable methods for [crate::tracking::TrackerFeatureHAAR_Params]
pub trait TrackerFeatureHAAR_ParamsTrait: crate::tracking::TrackerFeatureHAAR_ParamsTraitConst {
	fn as_raw_mut_TrackerFeatureHAAR_Params(&mut self) -> *mut c_void;

	/// # of rects
	// cv::TrackerFeatureHAAR::Params::setNumFeatures(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:983
	// ("cv::TrackerFeatureHAAR::Params::setNumFeatures", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_num_features(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerFeatureHAAR_Params_propNumFeatures_const_int(self.as_raw_mut_TrackerFeatureHAAR_Params(), val) };
		ret
	}

	/// rect size
	// cv::TrackerFeatureHAAR::Params::setRectSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:984
	// ("cv::TrackerFeatureHAAR::Params::setRectSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_rect_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_TrackerFeatureHAAR_Params_propRectSize_const_Size(self.as_raw_mut_TrackerFeatureHAAR_Params(), &val) };
		ret
	}

	/// true if input images are integral, false otherwise
	// cv::TrackerFeatureHAAR::Params::setIsIntegral(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:985
	// ("cv::TrackerFeatureHAAR::Params::setIsIntegral", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_is_integral(&mut self, val: bool) {
		let ret = unsafe { sys::cv_TrackerFeatureHAAR_Params_propIsIntegral_const_bool(self.as_raw_mut_TrackerFeatureHAAR_Params(), val) };
		ret
	}

}

// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:980
pub struct TrackerFeatureHAAR_Params {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerFeatureHAAR_Params }

impl Drop for TrackerFeatureHAAR_Params {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerFeatureHAAR_Params_delete(self.as_raw_mut_TrackerFeatureHAAR_Params()) };
	}
}

unsafe impl Send for TrackerFeatureHAAR_Params {}

impl crate::tracking::TrackerFeatureHAAR_ParamsTraitConst for TrackerFeatureHAAR_Params {
	#[inline] fn as_raw_TrackerFeatureHAAR_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerFeatureHAAR_ParamsTrait for TrackerFeatureHAAR_Params {
	#[inline] fn as_raw_mut_TrackerFeatureHAAR_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerFeatureHAAR_Params, crate::tracking::TrackerFeatureHAAR_ParamsTraitConst, as_raw_TrackerFeatureHAAR_Params, crate::tracking::TrackerFeatureHAAR_ParamsTrait, as_raw_mut_TrackerFeatureHAAR_Params }

impl TrackerFeatureHAAR_Params {
	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:982
	// ("cv::TrackerFeatureHAAR::Params::Params", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::tracking::TrackerFeatureHAAR_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureHAAR_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerFeatureHAAR_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for TrackerFeatureHAAR_Params {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerFeatureHAAR_Params")
			.field("num_features", &crate::tracking::TrackerFeatureHAAR_ParamsTraitConst::num_features(self))
			.field("rect_size", &crate::tracking::TrackerFeatureHAAR_ParamsTraitConst::rect_size(self))
			.field("is_integral", &crate::tracking::TrackerFeatureHAAR_ParamsTraitConst::is_integral(self))
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerFeatureHOG]
// TrackerFeatureHOG /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:958
pub trait TrackerFeatureHOGTraitConst: crate::tracking::TrackerFeatureTraitConst {
	fn as_raw_TrackerFeatureHOG(&self) -> *const c_void;

}

/// Mutable methods for [crate::tracking::TrackerFeatureHOG]
pub trait TrackerFeatureHOGTrait: crate::tracking::TrackerFeatureHOGTraitConst + crate::tracking::TrackerFeatureTrait {
	fn as_raw_mut_TrackerFeatureHOG(&mut self) -> *mut c_void;

	// selection(Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:966
	// ("cv::TrackerFeatureHOG::selection", vec![(pred!(mut, ["response", "npoints"], ["cv::Mat*", "int"]), _)]),
	#[inline]
	fn selection(&mut self, response: &mut impl core::MatTrait, npoints: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureHOG_selection_MatR_int(self.as_raw_mut_TrackerFeatureHOG(), response.as_raw_mut_Mat(), npoints, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// \brief TrackerFeature based on HOG
// TrackerFeatureHOG /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:958
pub struct TrackerFeatureHOG {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerFeatureHOG }

impl Drop for TrackerFeatureHOG {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerFeatureHOG_delete(self.as_raw_mut_TrackerFeatureHOG()) };
	}
}

unsafe impl Send for TrackerFeatureHOG {}

impl crate::tracking::TrackerFeatureTraitConst for TrackerFeatureHOG {
	#[inline] fn as_raw_TrackerFeature(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerFeatureTrait for TrackerFeatureHOG {
	#[inline] fn as_raw_mut_TrackerFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerFeatureHOG, crate::tracking::TrackerFeatureTraitConst, as_raw_TrackerFeature, crate::tracking::TrackerFeatureTrait, as_raw_mut_TrackerFeature }

impl crate::tracking::TrackerFeatureHOGTraitConst for TrackerFeatureHOG {
	#[inline] fn as_raw_TrackerFeatureHOG(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerFeatureHOGTrait for TrackerFeatureHOG {
	#[inline] fn as_raw_mut_TrackerFeatureHOG(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerFeatureHOG, crate::tracking::TrackerFeatureHOGTraitConst, as_raw_TrackerFeatureHOG, crate::tracking::TrackerFeatureHOGTrait, as_raw_mut_TrackerFeatureHOG }

impl TrackerFeatureHOG {
	// TrackerFeatureHOG()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:962
	// ("cv::TrackerFeatureHOG::TrackerFeatureHOG", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::tracking::TrackerFeatureHOG> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureHOG_TrackerFeatureHOG(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerFeatureHOG::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TrackerFeatureHOG, crate::tracking::TrackerFeature, cv_TrackerFeatureHOG_to_TrackerFeature }

impl std::fmt::Debug for TrackerFeatureHOG {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerFeatureHOG")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerFeatureLBP]
// TrackerFeatureLBP /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1039
pub trait TrackerFeatureLBPTraitConst: crate::tracking::TrackerFeatureTraitConst {
	fn as_raw_TrackerFeatureLBP(&self) -> *const c_void;

}

/// Mutable methods for [crate::tracking::TrackerFeatureLBP]
pub trait TrackerFeatureLBPTrait: crate::tracking::TrackerFeatureLBPTraitConst + crate::tracking::TrackerFeatureTrait {
	fn as_raw_mut_TrackerFeatureLBP(&mut self) -> *mut c_void;

	// selection(Mat &, int)(TraitClass, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1047
	// ("cv::TrackerFeatureLBP::selection", vec![(pred!(mut, ["response", "npoints"], ["cv::Mat*", "int"]), _)]),
	#[inline]
	fn selection(&mut self, response: &mut impl core::MatTrait, npoints: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureLBP_selection_MatR_int(self.as_raw_mut_TrackerFeatureLBP(), response.as_raw_mut_Mat(), npoints, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// \brief TrackerFeature based on LBP
// TrackerFeatureLBP /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1039
pub struct TrackerFeatureLBP {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerFeatureLBP }

impl Drop for TrackerFeatureLBP {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerFeatureLBP_delete(self.as_raw_mut_TrackerFeatureLBP()) };
	}
}

unsafe impl Send for TrackerFeatureLBP {}

impl crate::tracking::TrackerFeatureTraitConst for TrackerFeatureLBP {
	#[inline] fn as_raw_TrackerFeature(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerFeatureTrait for TrackerFeatureLBP {
	#[inline] fn as_raw_mut_TrackerFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerFeatureLBP, crate::tracking::TrackerFeatureTraitConst, as_raw_TrackerFeature, crate::tracking::TrackerFeatureTrait, as_raw_mut_TrackerFeature }

impl crate::tracking::TrackerFeatureLBPTraitConst for TrackerFeatureLBP {
	#[inline] fn as_raw_TrackerFeatureLBP(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerFeatureLBPTrait for TrackerFeatureLBP {
	#[inline] fn as_raw_mut_TrackerFeatureLBP(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerFeatureLBP, crate::tracking::TrackerFeatureLBPTraitConst, as_raw_TrackerFeatureLBP, crate::tracking::TrackerFeatureLBPTrait, as_raw_mut_TrackerFeatureLBP }

impl TrackerFeatureLBP {
	// TrackerFeatureLBP()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1043
	// ("cv::TrackerFeatureLBP::TrackerFeatureLBP", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::tracking::TrackerFeatureLBP> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureLBP_TrackerFeatureLBP(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerFeatureLBP::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TrackerFeatureLBP, crate::tracking::TrackerFeature, cv_TrackerFeatureLBP_to_TrackerFeature }

impl std::fmt::Debug for TrackerFeatureLBP {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerFeatureLBP")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerFeatureSet]
// TrackerFeatureSet /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:128
pub trait TrackerFeatureSetTraitConst {
	fn as_raw_TrackerFeatureSet(&self) -> *const c_void;

	/// Get the TrackerFeature collection (TrackerFeature name, TrackerFeature pointer)
	// getTrackerFeature()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:183
	// ("cv::TrackerFeatureSet::getTrackerFeature", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_tracker_feature(&self) -> Result<core::Vector<core::Tuple<(String, core::Ptr<crate::tracking::TrackerFeature>)>>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureSet_getTrackerFeature_const(self.as_raw_TrackerFeatureSet(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Tuple<(String, core::Ptr<crate::tracking::TrackerFeature>)>>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Get the responses
	///
	///
	/// Note: Be sure to call extraction before getResponses Example TrackerFeatureSet::getResponses : :
	// getResponses()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:189
	// ("cv::TrackerFeatureSet::getResponses", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_responses(&self) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureSet_getResponses_const(self.as_raw_TrackerFeatureSet(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::TrackerFeatureSet]
pub trait TrackerFeatureSetTrait: crate::tracking::TrackerFeatureSetTraitConst {
	fn as_raw_mut_TrackerFeatureSet(&mut self) -> *mut c_void;

	/// Extract features from the images collection
	/// ## Parameters
	/// * images: The input images
	// extraction(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:139
	// ("cv::TrackerFeatureSet::extraction", vec![(pred!(mut, ["images"], ["const std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn extraction(&mut self, images: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureSet_extraction_const_vectorLMatGR(self.as_raw_mut_TrackerFeatureSet(), images.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Identify most effective features for all feature types (optional)
	// selection()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:143
	// ("cv::TrackerFeatureSet::selection", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn selection(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureSet_selection(self.as_raw_mut_TrackerFeatureSet(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Remove outliers for all feature types (optional)
	// removeOutliers()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:147
	// ("cv::TrackerFeatureSet::removeOutliers", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn remove_outliers(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureSet_removeOutliers(self.as_raw_mut_TrackerFeatureSet(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ```C++
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
	// addTrackerFeature(String)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:174
	// ("cv::TrackerFeatureSet::addTrackerFeature", vec![(pred!(mut, ["trackerFeatureType"], ["cv::String"]), _)]),
	#[inline]
	fn add_tracker_feature(&mut self, tracker_feature_type: &str) -> Result<bool> {
		extern_container_arg!(tracker_feature_type);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureSet_addTrackerFeature_String(self.as_raw_mut_TrackerFeatureSet(), tracker_feature_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ```C++
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
	// addTrackerFeature(Ptr<TrackerFeature> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:179
	// ("cv::TrackerFeatureSet::addTrackerFeature", vec![(pred!(mut, ["feature"], ["cv::Ptr<cv::TrackerFeature>*"]), _)]),
	#[inline]
	fn add_tracker_feature_1(&mut self, feature: &mut core::Ptr<crate::tracking::TrackerFeature>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureSet_addTrackerFeature_PtrLTrackerFeatureGR(self.as_raw_mut_TrackerFeatureSet(), feature.as_raw_mut_PtrOfTrackerFeature(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class that manages the extraction and selection of features
///
/// [AAM](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_AAM) Feature Extraction and Feature Set Refinement (Feature Processing and Feature Selection).
/// See table I and section III C [AMVOT](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_AMVOT) Appearance modelling -\> Visual representation (Table II,
/// section 3.1 - 3.2)
///
/// TrackerFeatureSet is an aggregation of TrackerFeature
/// ## See also
/// TrackerFeature
// TrackerFeatureSet /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:128
pub struct TrackerFeatureSet {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerFeatureSet }

impl Drop for TrackerFeatureSet {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerFeatureSet_delete(self.as_raw_mut_TrackerFeatureSet()) };
	}
}

unsafe impl Send for TrackerFeatureSet {}

impl crate::tracking::TrackerFeatureSetTraitConst for TrackerFeatureSet {
	#[inline] fn as_raw_TrackerFeatureSet(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerFeatureSetTrait for TrackerFeatureSet {
	#[inline] fn as_raw_mut_TrackerFeatureSet(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerFeatureSet, crate::tracking::TrackerFeatureSetTraitConst, as_raw_TrackerFeatureSet, crate::tracking::TrackerFeatureSetTrait, as_raw_mut_TrackerFeatureSet }

impl TrackerFeatureSet {
	// TrackerFeatureSet()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:132
	// ("cv::TrackerFeatureSet::TrackerFeatureSet", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::tracking::TrackerFeatureSet> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerFeatureSet_TrackerFeatureSet(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerFeatureSet::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for TrackerFeatureSet {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerFeatureSet")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerGOTURN]
// TrackerGOTURN /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1287
pub trait TrackerGOTURNTraitConst: crate::tracking::TrackerTraitConst {
	fn as_raw_TrackerGOTURN(&self) -> *const c_void;

}

/// Mutable methods for [crate::tracking::TrackerGOTURN]
pub trait TrackerGOTURNTrait: crate::tracking::TrackerGOTURNTraitConst + crate::tracking::TrackerTrait {
	fn as_raw_mut_TrackerGOTURN(&mut self) -> *mut c_void;

}

/// the GOTURN (Generic Object Tracking Using Regression Networks) tracker
///
/// *  GOTURN ([GOTURN](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_GOTURN)) is kind of trackers based on Convolutional Neural Networks (CNN). While taking all advantages of CNN trackers,
/// *  GOTURN is much faster due to offline training without online fine-tuning nature.
/// *  GOTURN tracker addresses the problem of single target tracking: given a bounding box label of an object in the first frame of the video,
/// *  we track that object through the rest of the video. NOTE: Current method of GOTURN does not handle occlusions; however, it is fairly
/// *  robust to viewpoint changes, lighting changes, and deformations.
/// *  Inputs of GOTURN are two RGB patches representing Target and Search patches resized to 227x227.
/// *  Outputs of GOTURN are predicted bounding box coordinates, relative to Search patch coordinate system, in format X1,Y1,X2,Y2.
/// *  Original paper is here: <http://davheld.github.io/GOTURN/GOTURN.pdf>
/// *  As long as original authors implementation: <https://github.com/davheld/GOTURN#train-the-tracker>
/// *  Implementation of training algorithm is placed in separately here due to 3d-party dependencies:
/// *  <https://github.com/Auron-X/GOTURN_Training_Toolkit>
/// *  GOTURN architecture goturn.prototxt and trained model goturn.caffemodel are accessible on opencv_extra GitHub repository.
// TrackerGOTURN /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1287
pub struct TrackerGOTURN {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerGOTURN }

impl Drop for TrackerGOTURN {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerGOTURN_delete(self.as_raw_mut_TrackerGOTURN()) };
	}
}

unsafe impl Send for TrackerGOTURN {}

impl core::AlgorithmTraitConst for TrackerGOTURN {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for TrackerGOTURN {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerGOTURN, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::tracking::TrackerTraitConst for TrackerGOTURN {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerTrait for TrackerGOTURN {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerGOTURN, crate::tracking::TrackerTraitConst, as_raw_Tracker, crate::tracking::TrackerTrait, as_raw_mut_Tracker }

impl crate::tracking::TrackerGOTURNTraitConst for TrackerGOTURN {
	#[inline] fn as_raw_TrackerGOTURN(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerGOTURNTrait for TrackerGOTURN {
	#[inline] fn as_raw_mut_TrackerGOTURN(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerGOTURN, crate::tracking::TrackerGOTURNTraitConst, as_raw_TrackerGOTURN, crate::tracking::TrackerGOTURNTrait, as_raw_mut_TrackerGOTURN }

impl TrackerGOTURN {
	/// Constructor
	/// ## Parameters
	/// * parameters: GOTURN parameters TrackerGOTURN::Params
	// create(const TrackerGOTURN::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1302
	// ("cv::TrackerGOTURN::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerGOTURN::Params*"]), _)]),
	#[inline]
	pub fn create(parameters: &impl crate::tracking::TrackerGOTURN_ParamsTraitConst) -> Result<core::Ptr<crate::tracking::TrackerGOTURN>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerGOTURN_create_const_ParamsR(parameters.as_raw_TrackerGOTURN_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerGOTURN>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1304
	// ("cv::TrackerGOTURN::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_1() -> Result<core::Ptr<crate::tracking::TrackerGOTURN>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerGOTURN_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerGOTURN>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TrackerGOTURN, core::Algorithm, cv_TrackerGOTURN_to_Algorithm }

boxed_cast_base! { TrackerGOTURN, crate::tracking::Tracker, cv_TrackerGOTURN_to_Tracker }

impl std::fmt::Debug for TrackerGOTURN {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerGOTURN")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerGOTURN_Params]
// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1290
pub trait TrackerGOTURN_ParamsTraitConst {
	fn as_raw_TrackerGOTURN_Params(&self) -> *const c_void;

	// cv::TrackerGOTURN::Params::modelTxt() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1295
	// ("cv::TrackerGOTURN::Params::modelTxt", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn model_txt(&self) -> String {
		let ret = unsafe { sys::cv_TrackerGOTURN_Params_propModelTxt_const(self.as_raw_TrackerGOTURN_Params()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}

	// cv::TrackerGOTURN::Params::modelBin() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1296
	// ("cv::TrackerGOTURN::Params::modelBin", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn model_bin(&self) -> String {
		let ret = unsafe { sys::cv_TrackerGOTURN_Params_propModelBin_const(self.as_raw_TrackerGOTURN_Params()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1294
	// ("cv::TrackerGOTURN::Params::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, unnamed: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerGOTURN_Params_write_const_FileStorageR(self.as_raw_TrackerGOTURN_Params(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::TrackerGOTURN_Params]
pub trait TrackerGOTURN_ParamsTrait: crate::tracking::TrackerGOTURN_ParamsTraitConst {
	fn as_raw_mut_TrackerGOTURN_Params(&mut self) -> *mut c_void;

	// cv::TrackerGOTURN::Params::setModelTxt(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1295
	// ("cv::TrackerGOTURN::Params::setModelTxt", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	#[inline]
	fn set_model_txt(&mut self, val: &str) {
		extern_container_arg!(nofail val);
		let ret = unsafe { sys::cv_TrackerGOTURN_Params_propModelTxt_const_String(self.as_raw_mut_TrackerGOTURN_Params(), val.opencv_as_extern()) };
		ret
	}

	// cv::TrackerGOTURN::Params::setModelBin(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1296
	// ("cv::TrackerGOTURN::Params::setModelBin", vec![(pred!(mut, ["val"], ["const cv::String"]), _)]),
	#[inline]
	fn set_model_bin(&mut self, val: &str) {
		extern_container_arg!(nofail val);
		let ret = unsafe { sys::cv_TrackerGOTURN_Params_propModelBin_const_String(self.as_raw_mut_TrackerGOTURN_Params(), val.opencv_as_extern()) };
		ret
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1293
	// ("cv::TrackerGOTURN::Params::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read(&mut self, unnamed: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerGOTURN_Params_read_const_FileNodeR(self.as_raw_mut_TrackerGOTURN_Params(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1290
pub struct TrackerGOTURN_Params {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerGOTURN_Params }

impl Drop for TrackerGOTURN_Params {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerGOTURN_Params_delete(self.as_raw_mut_TrackerGOTURN_Params()) };
	}
}

unsafe impl Send for TrackerGOTURN_Params {}

impl crate::tracking::TrackerGOTURN_ParamsTraitConst for TrackerGOTURN_Params {
	#[inline] fn as_raw_TrackerGOTURN_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerGOTURN_ParamsTrait for TrackerGOTURN_Params {
	#[inline] fn as_raw_mut_TrackerGOTURN_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerGOTURN_Params, crate::tracking::TrackerGOTURN_ParamsTraitConst, as_raw_TrackerGOTURN_Params, crate::tracking::TrackerGOTURN_ParamsTrait, as_raw_mut_TrackerGOTURN_Params }

impl TrackerGOTURN_Params {
	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1292
	// ("cv::TrackerGOTURN::Params::Params", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::tracking::TrackerGOTURN_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerGOTURN_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerGOTURN_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for TrackerGOTURN_Params {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerGOTURN_Params")
			.field("model_txt", &crate::tracking::TrackerGOTURN_ParamsTraitConst::model_txt(self))
			.field("model_bin", &crate::tracking::TrackerGOTURN_ParamsTraitConst::model_bin(self))
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerKCF]
// TrackerKCF /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1212
pub trait TrackerKCFTraitConst: crate::tracking::TrackerTraitConst {
	fn as_raw_TrackerKCF(&self) -> *const c_void;

}

/// Mutable methods for [crate::tracking::TrackerKCF]
pub trait TrackerKCFTrait: crate::tracking::TrackerKCFTraitConst + crate::tracking::TrackerTrait {
	fn as_raw_mut_TrackerKCF(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * pca_func: false
	// setFeatureExtractor(void (*)(const Mat, const Rect, Mat &), bool)(Function, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1260
	// ("cv::TrackerKCF::setFeatureExtractor", vec![(pred!(mut, ["unnamed", "pca_func"], ["void (*)(const cv::Mat, const cv::Rect, cv::Mat&)", "bool"]), _)]),
	#[inline]
	fn set_feature_extractor(&mut self, unnamed: Option<unsafe extern "C" fn(*const c_void, core::Rect, *mut c_void) -> ()>, pca_func: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerKCF_setFeatureExtractor_void__X__const_cv_Mat__const_cv_Rect__cv_MatR__bool(self.as_raw_mut_TrackerKCF(), unnamed, pca_func, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// ## Note
	/// This alternative version of [TrackerKCFTrait::set_feature_extractor] function uses the following default values for its arguments:
	/// * pca_func: false
	// cv::TrackerKCF::setFeatureExtractor(Function) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1260
	// ("cv::TrackerKCF::setFeatureExtractor", vec![(pred!(mut, ["unnamed"], ["void (*)(const cv::Mat, const cv::Rect, cv::Mat&)"]), _)]),
	#[inline]
	fn set_feature_extractor_def(&mut self, unnamed: Option<unsafe extern "C" fn(*const c_void, core::Rect, *mut c_void) -> ()>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerKCF_setFeatureExtractor_void__X__const_cv_Mat__const_cv_Rect__cv_MatR_(self.as_raw_mut_TrackerKCF(), unnamed, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// the KCF (Kernelized Correlation Filter) tracker
///
/// * KCF is a novel tracking framework that utilizes properties of circulant matrix to enhance the processing speed.
/// * This tracking method is an implementation of [KCF_ECCV](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_KCF_ECCV) which is extended to KCF with color-names features ([KCF_CN](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_KCF_CN)).
/// * The original paper of KCF is available at <http://www.robots.ox.ac.uk/~joao/publications/henriques_tpami2015.pdf>
/// * as well as the matlab implementation. For more information about KCF with color-names features, please refer to
/// * <http://www.cvl.isy.liu.se/research/objrec/visualtracking/colvistrack/index.html>.
// TrackerKCF /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1212
pub struct TrackerKCF {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerKCF }

impl Drop for TrackerKCF {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerKCF_delete(self.as_raw_mut_TrackerKCF()) };
	}
}

unsafe impl Send for TrackerKCF {}

impl core::AlgorithmTraitConst for TrackerKCF {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for TrackerKCF {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerKCF, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::tracking::TrackerTraitConst for TrackerKCF {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerTrait for TrackerKCF {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerKCF, crate::tracking::TrackerTraitConst, as_raw_Tracker, crate::tracking::TrackerTrait, as_raw_mut_Tracker }

impl crate::tracking::TrackerKCFTraitConst for TrackerKCF {
	#[inline] fn as_raw_TrackerKCF(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerKCFTrait for TrackerKCF {
	#[inline] fn as_raw_mut_TrackerKCF(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerKCF, crate::tracking::TrackerKCFTraitConst, as_raw_TrackerKCF, crate::tracking::TrackerKCFTrait, as_raw_mut_TrackerKCF }

impl TrackerKCF {
	/// Constructor
	/// ## Parameters
	/// * parameters: KCF parameters TrackerKCF::Params
	// create(const TrackerKCF::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1265
	// ("cv::TrackerKCF::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerKCF::Params*"]), _)]),
	#[inline]
	pub fn create(parameters: &impl crate::tracking::TrackerKCF_ParamsTraitConst) -> Result<core::Ptr<crate::tracking::TrackerKCF>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerKCF_create_const_ParamsR(parameters.as_raw_TrackerKCF_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerKCF>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1267
	// ("cv::TrackerKCF::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_1() -> Result<core::Ptr<crate::tracking::TrackerKCF>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerKCF_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerKCF>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TrackerKCF, core::Algorithm, cv_TrackerKCF_to_Algorithm }

boxed_cast_base! { TrackerKCF, crate::tracking::Tracker, cv_TrackerKCF_to_Tracker }

impl std::fmt::Debug for TrackerKCF {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerKCF")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerKCF_Params]
// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1227
pub trait TrackerKCF_ParamsTraitConst {
	fn as_raw_TrackerKCF_Params(&self) -> *const c_void;

	/// detection confidence threshold
	// cv::TrackerKCF::Params::detect_thresh() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1244
	// ("cv::TrackerKCF::Params::detect_thresh", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn detect_thresh(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propDetect_thresh_const(self.as_raw_TrackerKCF_Params()) };
		ret
	}

	/// gaussian kernel bandwidth
	// cv::TrackerKCF::Params::sigma() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1245
	// ("cv::TrackerKCF::Params::sigma", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn sigma(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propSigma_const(self.as_raw_TrackerKCF_Params()) };
		ret
	}

	/// regularization
	// cv::TrackerKCF::Params::lambda() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1246
	// ("cv::TrackerKCF::Params::lambda", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn lambda(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propLambda_const(self.as_raw_TrackerKCF_Params()) };
		ret
	}

	/// linear interpolation factor for adaptation
	// cv::TrackerKCF::Params::interp_factor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1247
	// ("cv::TrackerKCF::Params::interp_factor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn interp_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propInterp_factor_const(self.as_raw_TrackerKCF_Params()) };
		ret
	}

	/// spatial bandwidth (proportional to target)
	// cv::TrackerKCF::Params::output_sigma_factor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1248
	// ("cv::TrackerKCF::Params::output_sigma_factor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn output_sigma_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propOutput_sigma_factor_const(self.as_raw_TrackerKCF_Params()) };
		ret
	}

	/// compression learning rate
	// cv::TrackerKCF::Params::pca_learning_rate() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1249
	// ("cv::TrackerKCF::Params::pca_learning_rate", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn pca_learning_rate(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propPca_learning_rate_const(self.as_raw_TrackerKCF_Params()) };
		ret
	}

	/// activate the resize feature to improve the processing speed
	// cv::TrackerKCF::Params::resize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1250
	// ("cv::TrackerKCF::Params::resize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn resize(&self) -> bool {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propResize_const(self.as_raw_TrackerKCF_Params()) };
		ret
	}

	/// split the training coefficients into two matrices
	// cv::TrackerKCF::Params::split_coeff() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1251
	// ("cv::TrackerKCF::Params::split_coeff", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn split_coeff(&self) -> bool {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propSplit_coeff_const(self.as_raw_TrackerKCF_Params()) };
		ret
	}

	/// wrap around the kernel values
	// cv::TrackerKCF::Params::wrap_kernel() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1252
	// ("cv::TrackerKCF::Params::wrap_kernel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn wrap_kernel(&self) -> bool {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propWrap_kernel_const(self.as_raw_TrackerKCF_Params()) };
		ret
	}

	/// activate the pca method to compress the features
	// cv::TrackerKCF::Params::compress_feature() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1253
	// ("cv::TrackerKCF::Params::compress_feature", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn compress_feature(&self) -> bool {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propCompress_feature_const(self.as_raw_TrackerKCF_Params()) };
		ret
	}

	/// threshold for the ROI size
	// cv::TrackerKCF::Params::max_patch_size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1254
	// ("cv::TrackerKCF::Params::max_patch_size", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn max_patch_size(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propMax_patch_size_const(self.as_raw_TrackerKCF_Params()) };
		ret
	}

	/// feature size after compression
	// cv::TrackerKCF::Params::compressed_size() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1255
	// ("cv::TrackerKCF::Params::compressed_size", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn compressed_size(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propCompressed_size_const(self.as_raw_TrackerKCF_Params()) };
		ret
	}

	/// compressed descriptors of TrackerKCF::MODE
	// cv::TrackerKCF::Params::desc_pca() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1256
	// ("cv::TrackerKCF::Params::desc_pca", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn desc_pca(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propDesc_pca_const(self.as_raw_TrackerKCF_Params()) };
		ret
	}

	/// non-compressed descriptors of TrackerKCF::MODE
	// cv::TrackerKCF::Params::desc_npca() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1257
	// ("cv::TrackerKCF::Params::desc_npca", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn desc_npca(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propDesc_npca_const(self.as_raw_TrackerKCF_Params()) };
		ret
	}

	/// \brief Write parameters to a file
	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1242
	// ("cv::TrackerKCF::Params::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, unnamed: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerKCF_Params_write_const_FileStorageR(self.as_raw_TrackerKCF_Params(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::TrackerKCF_Params]
pub trait TrackerKCF_ParamsTrait: crate::tracking::TrackerKCF_ParamsTraitConst {
	fn as_raw_mut_TrackerKCF_Params(&mut self) -> *mut c_void;

	/// detection confidence threshold
	// cv::TrackerKCF::Params::setDetect_thresh(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1244
	// ("cv::TrackerKCF::Params::setDetect_thresh", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_detect_thresh(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propDetect_thresh_const_float(self.as_raw_mut_TrackerKCF_Params(), val) };
		ret
	}

	/// gaussian kernel bandwidth
	// cv::TrackerKCF::Params::setSigma(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1245
	// ("cv::TrackerKCF::Params::setSigma", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_sigma(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propSigma_const_float(self.as_raw_mut_TrackerKCF_Params(), val) };
		ret
	}

	/// regularization
	// cv::TrackerKCF::Params::setLambda(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1246
	// ("cv::TrackerKCF::Params::setLambda", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_lambda(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propLambda_const_float(self.as_raw_mut_TrackerKCF_Params(), val) };
		ret
	}

	/// linear interpolation factor for adaptation
	// cv::TrackerKCF::Params::setInterp_factor(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1247
	// ("cv::TrackerKCF::Params::setInterp_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_interp_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propInterp_factor_const_float(self.as_raw_mut_TrackerKCF_Params(), val) };
		ret
	}

	/// spatial bandwidth (proportional to target)
	// cv::TrackerKCF::Params::setOutput_sigma_factor(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1248
	// ("cv::TrackerKCF::Params::setOutput_sigma_factor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_output_sigma_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propOutput_sigma_factor_const_float(self.as_raw_mut_TrackerKCF_Params(), val) };
		ret
	}

	/// compression learning rate
	// cv::TrackerKCF::Params::setPca_learning_rate(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1249
	// ("cv::TrackerKCF::Params::setPca_learning_rate", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_pca_learning_rate(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propPca_learning_rate_const_float(self.as_raw_mut_TrackerKCF_Params(), val) };
		ret
	}

	/// activate the resize feature to improve the processing speed
	// cv::TrackerKCF::Params::setResize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1250
	// ("cv::TrackerKCF::Params::setResize", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_resize(&mut self, val: bool) {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propResize_const_bool(self.as_raw_mut_TrackerKCF_Params(), val) };
		ret
	}

	/// split the training coefficients into two matrices
	// cv::TrackerKCF::Params::setSplit_coeff(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1251
	// ("cv::TrackerKCF::Params::setSplit_coeff", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_split_coeff(&mut self, val: bool) {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propSplit_coeff_const_bool(self.as_raw_mut_TrackerKCF_Params(), val) };
		ret
	}

	/// wrap around the kernel values
	// cv::TrackerKCF::Params::setWrap_kernel(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1252
	// ("cv::TrackerKCF::Params::setWrap_kernel", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_wrap_kernel(&mut self, val: bool) {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propWrap_kernel_const_bool(self.as_raw_mut_TrackerKCF_Params(), val) };
		ret
	}

	/// activate the pca method to compress the features
	// cv::TrackerKCF::Params::setCompress_feature(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1253
	// ("cv::TrackerKCF::Params::setCompress_feature", vec![(pred!(mut, ["val"], ["const bool"]), _)]),
	#[inline]
	fn set_compress_feature(&mut self, val: bool) {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propCompress_feature_const_bool(self.as_raw_mut_TrackerKCF_Params(), val) };
		ret
	}

	/// threshold for the ROI size
	// cv::TrackerKCF::Params::setMax_patch_size(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1254
	// ("cv::TrackerKCF::Params::setMax_patch_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_max_patch_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propMax_patch_size_const_int(self.as_raw_mut_TrackerKCF_Params(), val) };
		ret
	}

	/// feature size after compression
	// cv::TrackerKCF::Params::setCompressed_size(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1255
	// ("cv::TrackerKCF::Params::setCompressed_size", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_compressed_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propCompressed_size_const_int(self.as_raw_mut_TrackerKCF_Params(), val) };
		ret
	}

	/// compressed descriptors of TrackerKCF::MODE
	// cv::TrackerKCF::Params::setDesc_pca(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1256
	// ("cv::TrackerKCF::Params::setDesc_pca", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_desc_pca(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propDesc_pca_const_int(self.as_raw_mut_TrackerKCF_Params(), val) };
		ret
	}

	/// non-compressed descriptors of TrackerKCF::MODE
	// cv::TrackerKCF::Params::setDesc_npca(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1257
	// ("cv::TrackerKCF::Params::setDesc_npca", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_desc_npca(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerKCF_Params_propDesc_npca_const_int(self.as_raw_mut_TrackerKCF_Params(), val) };
		ret
	}

	/// \brief Read parameters from a file
	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1237
	// ("cv::TrackerKCF::Params::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read(&mut self, unnamed: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerKCF_Params_read_const_FileNodeR(self.as_raw_mut_TrackerKCF_Params(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1227
pub struct TrackerKCF_Params {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerKCF_Params }

impl Drop for TrackerKCF_Params {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerKCF_Params_delete(self.as_raw_mut_TrackerKCF_Params()) };
	}
}

unsafe impl Send for TrackerKCF_Params {}

impl crate::tracking::TrackerKCF_ParamsTraitConst for TrackerKCF_Params {
	#[inline] fn as_raw_TrackerKCF_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerKCF_ParamsTrait for TrackerKCF_Params {
	#[inline] fn as_raw_mut_TrackerKCF_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerKCF_Params, crate::tracking::TrackerKCF_ParamsTraitConst, as_raw_TrackerKCF_Params, crate::tracking::TrackerKCF_ParamsTrait, as_raw_mut_TrackerKCF_Params }

impl TrackerKCF_Params {
	/// \brief Constructor
	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1232
	// ("cv::TrackerKCF::Params::Params", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::tracking::TrackerKCF_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerKCF_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerKCF_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for TrackerKCF_Params {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerKCF_Params")
			.field("detect_thresh", &crate::tracking::TrackerKCF_ParamsTraitConst::detect_thresh(self))
			.field("sigma", &crate::tracking::TrackerKCF_ParamsTraitConst::sigma(self))
			.field("lambda", &crate::tracking::TrackerKCF_ParamsTraitConst::lambda(self))
			.field("interp_factor", &crate::tracking::TrackerKCF_ParamsTraitConst::interp_factor(self))
			.field("output_sigma_factor", &crate::tracking::TrackerKCF_ParamsTraitConst::output_sigma_factor(self))
			.field("pca_learning_rate", &crate::tracking::TrackerKCF_ParamsTraitConst::pca_learning_rate(self))
			.field("resize", &crate::tracking::TrackerKCF_ParamsTraitConst::resize(self))
			.field("split_coeff", &crate::tracking::TrackerKCF_ParamsTraitConst::split_coeff(self))
			.field("wrap_kernel", &crate::tracking::TrackerKCF_ParamsTraitConst::wrap_kernel(self))
			.field("compress_feature", &crate::tracking::TrackerKCF_ParamsTraitConst::compress_feature(self))
			.field("max_patch_size", &crate::tracking::TrackerKCF_ParamsTraitConst::max_patch_size(self))
			.field("compressed_size", &crate::tracking::TrackerKCF_ParamsTraitConst::compressed_size(self))
			.field("desc_pca", &crate::tracking::TrackerKCF_ParamsTraitConst::desc_pca(self))
			.field("desc_npca", &crate::tracking::TrackerKCF_ParamsTraitConst::desc_npca(self))
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerMIL]
// TrackerMIL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1065
pub trait TrackerMILTraitConst: crate::tracking::TrackerTraitConst {
	fn as_raw_TrackerMIL(&self) -> *const c_void;

}

/// Mutable methods for [crate::tracking::TrackerMIL]
pub trait TrackerMILTrait: crate::tracking::TrackerMILTraitConst + crate::tracking::TrackerTrait {
	fn as_raw_mut_TrackerMIL(&mut self) -> *mut c_void;

}

/// The MIL algorithm trains a classifier in an online manner to separate the object from the
/// background.
///
/// Multiple Instance Learning avoids the drift problem for a robust tracking. The implementation is
/// based on [MIL](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_MIL) .
///
/// Original code can be found here <http://vision.ucsd.edu/~bbabenko/project_miltrack.shtml>
// TrackerMIL /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1065
pub struct TrackerMIL {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerMIL }

impl Drop for TrackerMIL {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerMIL_delete(self.as_raw_mut_TrackerMIL()) };
	}
}

unsafe impl Send for TrackerMIL {}

impl core::AlgorithmTraitConst for TrackerMIL {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for TrackerMIL {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerMIL, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::tracking::TrackerTraitConst for TrackerMIL {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerTrait for TrackerMIL {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerMIL, crate::tracking::TrackerTraitConst, as_raw_Tracker, crate::tracking::TrackerTrait, as_raw_mut_Tracker }

impl crate::tracking::TrackerMILTraitConst for TrackerMIL {
	#[inline] fn as_raw_TrackerMIL(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerMILTrait for TrackerMIL {
	#[inline] fn as_raw_mut_TrackerMIL(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerMIL, crate::tracking::TrackerMILTraitConst, as_raw_TrackerMIL, crate::tracking::TrackerMILTrait, as_raw_mut_TrackerMIL }

impl TrackerMIL {
	/// Constructor
	/// ## Parameters
	/// * parameters: MIL parameters TrackerMIL::Params
	// create(const TrackerMIL::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1087
	// ("cv::TrackerMIL::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerMIL::Params*"]), _)]),
	#[inline]
	pub fn create(parameters: &impl crate::tracking::TrackerMIL_ParamsTraitConst) -> Result<core::Ptr<crate::tracking::TrackerMIL>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerMIL_create_const_ParamsR(parameters.as_raw_TrackerMIL_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerMIL>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1089
	// ("cv::TrackerMIL::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_1() -> Result<core::Ptr<crate::tracking::TrackerMIL>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerMIL_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerMIL>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TrackerMIL, core::Algorithm, cv_TrackerMIL_to_Algorithm }

boxed_cast_base! { TrackerMIL, crate::tracking::Tracker, cv_TrackerMIL_to_Tracker }

impl std::fmt::Debug for TrackerMIL {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerMIL")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerMIL_Params]
// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1068
pub trait TrackerMIL_ParamsTraitConst {
	fn as_raw_TrackerMIL_Params(&self) -> *const c_void;

	/// radius for gathering positive instances during init
	// cv::TrackerMIL::Params::samplerInitInRadius() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1072
	// ("cv::TrackerMIL::Params::samplerInitInRadius", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn sampler_init_in_radius(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerMIL_Params_propSamplerInitInRadius_const(self.as_raw_TrackerMIL_Params()) };
		ret
	}

	/// # negative samples to use during init
	// cv::TrackerMIL::Params::samplerInitMaxNegNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1073
	// ("cv::TrackerMIL::Params::samplerInitMaxNegNum", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn sampler_init_max_neg_num(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerMIL_Params_propSamplerInitMaxNegNum_const(self.as_raw_TrackerMIL_Params()) };
		ret
	}

	/// size of search window
	// cv::TrackerMIL::Params::samplerSearchWinSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1074
	// ("cv::TrackerMIL::Params::samplerSearchWinSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn sampler_search_win_size(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerMIL_Params_propSamplerSearchWinSize_const(self.as_raw_TrackerMIL_Params()) };
		ret
	}

	/// radius for gathering positive instances during tracking
	// cv::TrackerMIL::Params::samplerTrackInRadius() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1075
	// ("cv::TrackerMIL::Params::samplerTrackInRadius", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn sampler_track_in_radius(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerMIL_Params_propSamplerTrackInRadius_const(self.as_raw_TrackerMIL_Params()) };
		ret
	}

	/// # positive samples to use during tracking
	// cv::TrackerMIL::Params::samplerTrackMaxPosNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1076
	// ("cv::TrackerMIL::Params::samplerTrackMaxPosNum", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn sampler_track_max_pos_num(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerMIL_Params_propSamplerTrackMaxPosNum_const(self.as_raw_TrackerMIL_Params()) };
		ret
	}

	/// # negative samples to use during tracking
	// cv::TrackerMIL::Params::samplerTrackMaxNegNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1077
	// ("cv::TrackerMIL::Params::samplerTrackMaxNegNum", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn sampler_track_max_neg_num(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerMIL_Params_propSamplerTrackMaxNegNum_const(self.as_raw_TrackerMIL_Params()) };
		ret
	}

	/// # features
	// cv::TrackerMIL::Params::featureSetNumFeatures() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1078
	// ("cv::TrackerMIL::Params::featureSetNumFeatures", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn feature_set_num_features(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerMIL_Params_propFeatureSetNumFeatures_const(self.as_raw_TrackerMIL_Params()) };
		ret
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1081
	// ("cv::TrackerMIL::Params::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerMIL_Params_write_const_FileStorageR(self.as_raw_TrackerMIL_Params(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::TrackerMIL_Params]
pub trait TrackerMIL_ParamsTrait: crate::tracking::TrackerMIL_ParamsTraitConst {
	fn as_raw_mut_TrackerMIL_Params(&mut self) -> *mut c_void;

	/// radius for gathering positive instances during init
	// cv::TrackerMIL::Params::setSamplerInitInRadius(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1072
	// ("cv::TrackerMIL::Params::setSamplerInitInRadius", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_sampler_init_in_radius(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerMIL_Params_propSamplerInitInRadius_const_float(self.as_raw_mut_TrackerMIL_Params(), val) };
		ret
	}

	/// # negative samples to use during init
	// cv::TrackerMIL::Params::setSamplerInitMaxNegNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1073
	// ("cv::TrackerMIL::Params::setSamplerInitMaxNegNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_sampler_init_max_neg_num(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerMIL_Params_propSamplerInitMaxNegNum_const_int(self.as_raw_mut_TrackerMIL_Params(), val) };
		ret
	}

	/// size of search window
	// cv::TrackerMIL::Params::setSamplerSearchWinSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1074
	// ("cv::TrackerMIL::Params::setSamplerSearchWinSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_sampler_search_win_size(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerMIL_Params_propSamplerSearchWinSize_const_float(self.as_raw_mut_TrackerMIL_Params(), val) };
		ret
	}

	/// radius for gathering positive instances during tracking
	// cv::TrackerMIL::Params::setSamplerTrackInRadius(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1075
	// ("cv::TrackerMIL::Params::setSamplerTrackInRadius", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_sampler_track_in_radius(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerMIL_Params_propSamplerTrackInRadius_const_float(self.as_raw_mut_TrackerMIL_Params(), val) };
		ret
	}

	/// # positive samples to use during tracking
	// cv::TrackerMIL::Params::setSamplerTrackMaxPosNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1076
	// ("cv::TrackerMIL::Params::setSamplerTrackMaxPosNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_sampler_track_max_pos_num(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerMIL_Params_propSamplerTrackMaxPosNum_const_int(self.as_raw_mut_TrackerMIL_Params(), val) };
		ret
	}

	/// # negative samples to use during tracking
	// cv::TrackerMIL::Params::setSamplerTrackMaxNegNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1077
	// ("cv::TrackerMIL::Params::setSamplerTrackMaxNegNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_sampler_track_max_neg_num(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerMIL_Params_propSamplerTrackMaxNegNum_const_int(self.as_raw_mut_TrackerMIL_Params(), val) };
		ret
	}

	/// # features
	// cv::TrackerMIL::Params::setFeatureSetNumFeatures(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1078
	// ("cv::TrackerMIL::Params::setFeatureSetNumFeatures", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_feature_set_num_features(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerMIL_Params_propFeatureSetNumFeatures_const_int(self.as_raw_mut_TrackerMIL_Params(), val) };
		ret
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1080
	// ("cv::TrackerMIL::Params::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerMIL_Params_read_const_FileNodeR(self.as_raw_mut_TrackerMIL_Params(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1068
pub struct TrackerMIL_Params {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerMIL_Params }

impl Drop for TrackerMIL_Params {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerMIL_Params_delete(self.as_raw_mut_TrackerMIL_Params()) };
	}
}

unsafe impl Send for TrackerMIL_Params {}

impl crate::tracking::TrackerMIL_ParamsTraitConst for TrackerMIL_Params {
	#[inline] fn as_raw_TrackerMIL_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerMIL_ParamsTrait for TrackerMIL_Params {
	#[inline] fn as_raw_mut_TrackerMIL_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerMIL_Params, crate::tracking::TrackerMIL_ParamsTraitConst, as_raw_TrackerMIL_Params, crate::tracking::TrackerMIL_ParamsTrait, as_raw_mut_TrackerMIL_Params }

impl TrackerMIL_Params {
	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1070
	// ("cv::TrackerMIL::Params::Params", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::tracking::TrackerMIL_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerMIL_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerMIL_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for TrackerMIL_Params {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerMIL_Params")
			.field("sampler_init_in_radius", &crate::tracking::TrackerMIL_ParamsTraitConst::sampler_init_in_radius(self))
			.field("sampler_init_max_neg_num", &crate::tracking::TrackerMIL_ParamsTraitConst::sampler_init_max_neg_num(self))
			.field("sampler_search_win_size", &crate::tracking::TrackerMIL_ParamsTraitConst::sampler_search_win_size(self))
			.field("sampler_track_in_radius", &crate::tracking::TrackerMIL_ParamsTraitConst::sampler_track_in_radius(self))
			.field("sampler_track_max_pos_num", &crate::tracking::TrackerMIL_ParamsTraitConst::sampler_track_max_pos_num(self))
			.field("sampler_track_max_neg_num", &crate::tracking::TrackerMIL_ParamsTraitConst::sampler_track_max_neg_num(self))
			.field("feature_set_num_features", &crate::tracking::TrackerMIL_ParamsTraitConst::feature_set_num_features(self))
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerMOSSE]
// TrackerMOSSE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1315
pub trait TrackerMOSSETraitConst: crate::tracking::TrackerTraitConst {
	fn as_raw_TrackerMOSSE(&self) -> *const c_void;

}

/// Mutable methods for [crate::tracking::TrackerMOSSE]
pub trait TrackerMOSSETrait: crate::tracking::TrackerMOSSETraitConst + crate::tracking::TrackerTrait {
	fn as_raw_mut_TrackerMOSSE(&mut self) -> *mut c_void;

}

/// the MOSSE (Minimum Output Sum of Squared %Error) tracker
///
/// The implementation is based on [MOSSE](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_MOSSE) Visual Object Tracking using Adaptive Correlation Filters
///
/// Note: this tracker works with grayscale images, if passed bgr ones, they will get converted internally.
// TrackerMOSSE /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1315
pub struct TrackerMOSSE {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerMOSSE }

impl Drop for TrackerMOSSE {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerMOSSE_delete(self.as_raw_mut_TrackerMOSSE()) };
	}
}

unsafe impl Send for TrackerMOSSE {}

impl core::AlgorithmTraitConst for TrackerMOSSE {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for TrackerMOSSE {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerMOSSE, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::tracking::TrackerTraitConst for TrackerMOSSE {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerTrait for TrackerMOSSE {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerMOSSE, crate::tracking::TrackerTraitConst, as_raw_Tracker, crate::tracking::TrackerTrait, as_raw_mut_Tracker }

impl crate::tracking::TrackerMOSSETraitConst for TrackerMOSSE {
	#[inline] fn as_raw_TrackerMOSSE(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerMOSSETrait for TrackerMOSSE {
	#[inline] fn as_raw_mut_TrackerMOSSE(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerMOSSE, crate::tracking::TrackerMOSSETraitConst, as_raw_TrackerMOSSE, crate::tracking::TrackerMOSSETrait, as_raw_mut_TrackerMOSSE }

impl TrackerMOSSE {
	/// Constructor
	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1320
	// ("cv::TrackerMOSSE::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::tracking::TrackerMOSSE>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerMOSSE_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerMOSSE>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TrackerMOSSE, core::Algorithm, cv_TrackerMOSSE_to_Algorithm }

boxed_cast_base! { TrackerMOSSE, crate::tracking::Tracker, cv_TrackerMOSSE_to_Tracker }

impl std::fmt::Debug for TrackerMOSSE {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerMOSSE")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerMedianFlow]
// TrackerMedianFlow /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1142
pub trait TrackerMedianFlowTraitConst: crate::tracking::TrackerTraitConst {
	fn as_raw_TrackerMedianFlow(&self) -> *const c_void;

}

/// Mutable methods for [crate::tracking::TrackerMedianFlow]
pub trait TrackerMedianFlowTrait: crate::tracking::TrackerMedianFlowTraitConst + crate::tracking::TrackerTrait {
	fn as_raw_mut_TrackerMedianFlow(&mut self) -> *mut c_void;

}

/// the Median Flow tracker
///
/// Implementation of a paper [MedianFlow](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_MedianFlow) .
///
/// The tracker is suitable for very smooth and predictable movements when object is visible throughout
/// the whole sequence. It's quite and accurate for this type of problems (in particular, it was shown
/// by authors to outperform MIL). During the implementation period the code at
/// <http://www.aonsquared.co.uk/node/5>, the courtesy of the author Arthur Amarra, was used for the
/// reference purpose.
// TrackerMedianFlow /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1142
pub struct TrackerMedianFlow {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerMedianFlow }

impl Drop for TrackerMedianFlow {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerMedianFlow_delete(self.as_raw_mut_TrackerMedianFlow()) };
	}
}

unsafe impl Send for TrackerMedianFlow {}

impl core::AlgorithmTraitConst for TrackerMedianFlow {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for TrackerMedianFlow {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerMedianFlow, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::tracking::TrackerTraitConst for TrackerMedianFlow {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerTrait for TrackerMedianFlow {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerMedianFlow, crate::tracking::TrackerTraitConst, as_raw_Tracker, crate::tracking::TrackerTrait, as_raw_mut_Tracker }

impl crate::tracking::TrackerMedianFlowTraitConst for TrackerMedianFlow {
	#[inline] fn as_raw_TrackerMedianFlow(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerMedianFlowTrait for TrackerMedianFlow {
	#[inline] fn as_raw_mut_TrackerMedianFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerMedianFlow, crate::tracking::TrackerMedianFlowTraitConst, as_raw_TrackerMedianFlow, crate::tracking::TrackerMedianFlowTrait, as_raw_mut_TrackerMedianFlow }

impl TrackerMedianFlow {
	/// Constructor
	/// ## Parameters
	/// * parameters: Median Flow parameters TrackerMedianFlow::Params
	// create(const TrackerMedianFlow::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1164
	// ("cv::TrackerMedianFlow::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerMedianFlow::Params*"]), _)]),
	#[inline]
	pub fn create(parameters: &impl crate::tracking::TrackerMedianFlow_ParamsTraitConst) -> Result<core::Ptr<crate::tracking::TrackerMedianFlow>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerMedianFlow_create_const_ParamsR(parameters.as_raw_TrackerMedianFlow_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerMedianFlow>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1166
	// ("cv::TrackerMedianFlow::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_1() -> Result<core::Ptr<crate::tracking::TrackerMedianFlow>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerMedianFlow_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerMedianFlow>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TrackerMedianFlow, core::Algorithm, cv_TrackerMedianFlow_to_Algorithm }

boxed_cast_base! { TrackerMedianFlow, crate::tracking::Tracker, cv_TrackerMedianFlow_to_Tracker }

impl std::fmt::Debug for TrackerMedianFlow {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerMedianFlow")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerMedianFlow_Params]
// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1145
pub trait TrackerMedianFlow_ParamsTraitConst {
	fn as_raw_TrackerMedianFlow_Params(&self) -> *const c_void;

	/// square root of number of keypoints used; increase it to trade
	/// accurateness for speed
	// cv::TrackerMedianFlow::Params::pointsInGrid() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1149
	// ("cv::TrackerMedianFlow::Params::pointsInGrid", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn points_in_grid(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerMedianFlow_Params_propPointsInGrid_const(self.as_raw_TrackerMedianFlow_Params()) };
		ret
	}

	/// window size parameter for Lucas-Kanade optical flow
	// cv::TrackerMedianFlow::Params::winSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1151
	// ("cv::TrackerMedianFlow::Params::winSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn win_size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerMedianFlow_Params_propWinSize_const(self.as_raw_TrackerMedianFlow_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// maximal pyramid level number for Lucas-Kanade optical flow
	// cv::TrackerMedianFlow::Params::maxLevel() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1152
	// ("cv::TrackerMedianFlow::Params::maxLevel", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn max_level(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerMedianFlow_Params_propMaxLevel_const(self.as_raw_TrackerMedianFlow_Params()) };
		ret
	}

	/// termination criteria for Lucas-Kanade optical flow
	// cv::TrackerMedianFlow::Params::termCriteria() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1153
	// ("cv::TrackerMedianFlow::Params::termCriteria", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn term_criteria(&self) -> core::TermCriteria {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerMedianFlow_Params_propTermCriteria_const(self.as_raw_TrackerMedianFlow_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// window size around a point for normalized cross-correlation check
	// cv::TrackerMedianFlow::Params::winSizeNCC() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1154
	// ("cv::TrackerMedianFlow::Params::winSizeNCC", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn win_size_ncc(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerMedianFlow_Params_propWinSizeNCC_const(self.as_raw_TrackerMedianFlow_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}

	/// criterion for loosing the tracked object
	// cv::TrackerMedianFlow::Params::maxMedianLengthOfDisplacementDifference() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1155
	// ("cv::TrackerMedianFlow::Params::maxMedianLengthOfDisplacementDifference", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn max_median_length_of_displacement_difference(&self) -> f64 {
		let ret = unsafe { sys::cv_TrackerMedianFlow_Params_propMaxMedianLengthOfDisplacementDifference_const(self.as_raw_TrackerMedianFlow_Params()) };
		ret
	}

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1158
	// ("cv::TrackerMedianFlow::Params::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, unnamed: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerMedianFlow_Params_write_const_FileStorageR(self.as_raw_TrackerMedianFlow_Params(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::TrackerMedianFlow_Params]
pub trait TrackerMedianFlow_ParamsTrait: crate::tracking::TrackerMedianFlow_ParamsTraitConst {
	fn as_raw_mut_TrackerMedianFlow_Params(&mut self) -> *mut c_void;

	/// square root of number of keypoints used; increase it to trade
	/// accurateness for speed
	// cv::TrackerMedianFlow::Params::setPointsInGrid(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1149
	// ("cv::TrackerMedianFlow::Params::setPointsInGrid", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_points_in_grid(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerMedianFlow_Params_propPointsInGrid_const_int(self.as_raw_mut_TrackerMedianFlow_Params(), val) };
		ret
	}

	/// window size parameter for Lucas-Kanade optical flow
	// cv::TrackerMedianFlow::Params::setWinSize(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1151
	// ("cv::TrackerMedianFlow::Params::setWinSize", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_win_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_TrackerMedianFlow_Params_propWinSize_const_Size(self.as_raw_mut_TrackerMedianFlow_Params(), &val) };
		ret
	}

	/// maximal pyramid level number for Lucas-Kanade optical flow
	// cv::TrackerMedianFlow::Params::setMaxLevel(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1152
	// ("cv::TrackerMedianFlow::Params::setMaxLevel", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_max_level(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerMedianFlow_Params_propMaxLevel_const_int(self.as_raw_mut_TrackerMedianFlow_Params(), val) };
		ret
	}

	/// termination criteria for Lucas-Kanade optical flow
	// cv::TrackerMedianFlow::Params::setTermCriteria(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1153
	// ("cv::TrackerMedianFlow::Params::setTermCriteria", vec![(pred!(mut, ["val"], ["const cv::TermCriteria"]), _)]),
	#[inline]
	fn set_term_criteria(&mut self, val: core::TermCriteria) {
		let ret = unsafe { sys::cv_TrackerMedianFlow_Params_propTermCriteria_const_TermCriteria(self.as_raw_mut_TrackerMedianFlow_Params(), &val) };
		ret
	}

	/// window size around a point for normalized cross-correlation check
	// cv::TrackerMedianFlow::Params::setWinSizeNCC(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1154
	// ("cv::TrackerMedianFlow::Params::setWinSizeNCC", vec![(pred!(mut, ["val"], ["const cv::Size"]), _)]),
	#[inline]
	fn set_win_size_ncc(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_TrackerMedianFlow_Params_propWinSizeNCC_const_Size(self.as_raw_mut_TrackerMedianFlow_Params(), &val) };
		ret
	}

	/// criterion for loosing the tracked object
	// cv::TrackerMedianFlow::Params::setMaxMedianLengthOfDisplacementDifference(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1155
	// ("cv::TrackerMedianFlow::Params::setMaxMedianLengthOfDisplacementDifference", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_max_median_length_of_displacement_difference(&mut self, val: f64) {
		let ret = unsafe { sys::cv_TrackerMedianFlow_Params_propMaxMedianLengthOfDisplacementDifference_const_double(self.as_raw_mut_TrackerMedianFlow_Params(), val) };
		ret
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1157
	// ("cv::TrackerMedianFlow::Params::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read(&mut self, unnamed: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerMedianFlow_Params_read_const_FileNodeR(self.as_raw_mut_TrackerMedianFlow_Params(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1145
pub struct TrackerMedianFlow_Params {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerMedianFlow_Params }

impl Drop for TrackerMedianFlow_Params {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerMedianFlow_Params_delete(self.as_raw_mut_TrackerMedianFlow_Params()) };
	}
}

unsafe impl Send for TrackerMedianFlow_Params {}

impl crate::tracking::TrackerMedianFlow_ParamsTraitConst for TrackerMedianFlow_Params {
	#[inline] fn as_raw_TrackerMedianFlow_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerMedianFlow_ParamsTrait for TrackerMedianFlow_Params {
	#[inline] fn as_raw_mut_TrackerMedianFlow_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerMedianFlow_Params, crate::tracking::TrackerMedianFlow_ParamsTraitConst, as_raw_TrackerMedianFlow_Params, crate::tracking::TrackerMedianFlow_ParamsTrait, as_raw_mut_TrackerMedianFlow_Params }

impl TrackerMedianFlow_Params {
	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1147
	// ("cv::TrackerMedianFlow::Params::Params", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::tracking::TrackerMedianFlow_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerMedianFlow_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerMedianFlow_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for TrackerMedianFlow_Params {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerMedianFlow_Params")
			.field("points_in_grid", &crate::tracking::TrackerMedianFlow_ParamsTraitConst::points_in_grid(self))
			.field("win_size", &crate::tracking::TrackerMedianFlow_ParamsTraitConst::win_size(self))
			.field("max_level", &crate::tracking::TrackerMedianFlow_ParamsTraitConst::max_level(self))
			.field("term_criteria", &crate::tracking::TrackerMedianFlow_ParamsTraitConst::term_criteria(self))
			.field("win_size_ncc", &crate::tracking::TrackerMedianFlow_ParamsTraitConst::win_size_ncc(self))
			.field("max_median_length_of_displacement_difference", &crate::tracking::TrackerMedianFlow_ParamsTraitConst::max_median_length_of_displacement_difference(self))
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerModel]
// TrackerModel /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:446
pub trait TrackerModelTraitConst {
	fn as_raw_TrackerModel(&self) -> *const c_void;

	/// Get the last TrackerTargetState from Trajectory
	// getLastTargetState()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:490
	// ("cv::TrackerModel::getLastTargetState", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_last_target_state(&self) -> Result<core::Ptr<crate::tracking::TrackerTargetState>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerModel_getLastTargetState_const(self.as_raw_TrackerModel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerTargetState>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Get the list of the ConfidenceMap
	// getConfidenceMaps()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:494
	// ("cv::TrackerModel::getConfidenceMaps", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_confidence_maps(&self) -> Result<core::Vector<crate::tracking::ConfidenceMap>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerModel_getConfidenceMaps_const(self.as_raw_TrackerModel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<crate::tracking::ConfidenceMap>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Get the last ConfidenceMap for the current frame
	// getLastConfidenceMap()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:498
	// ("cv::TrackerModel::getLastConfidenceMap", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_last_confidence_map(&self) -> Result<crate::tracking::ConfidenceMap> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerModel_getLastConfidenceMap_const(self.as_raw_TrackerModel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::ConfidenceMap::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Get the TrackerStateEstimator
	// getTrackerStateEstimator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:502
	// ("cv::TrackerModel::getTrackerStateEstimator", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_tracker_state_estimator(&self) -> Result<core::Ptr<crate::tracking::TrackerStateEstimator>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerModel_getTrackerStateEstimator_const(self.as_raw_TrackerModel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerStateEstimator>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::TrackerModel]
pub trait TrackerModelTrait: crate::tracking::TrackerModelTraitConst {
	fn as_raw_mut_TrackerModel(&mut self) -> *mut c_void;

	/// Set TrackerEstimator, return true if the tracker state estimator is added, false otherwise
	/// ## Parameters
	/// * trackerStateEstimator: The TrackerStateEstimator
	///
	/// Note: You can add only one TrackerStateEstimator
	// setTrackerStateEstimator(Ptr<TrackerStateEstimator>)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:464
	// ("cv::TrackerModel::setTrackerStateEstimator", vec![(pred!(mut, ["trackerStateEstimator"], ["cv::Ptr<cv::TrackerStateEstimator>"]), _)]),
	#[inline]
	fn set_tracker_state_estimator(&mut self, mut tracker_state_estimator: core::Ptr<crate::tracking::TrackerStateEstimator>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerModel_setTrackerStateEstimator_PtrLTrackerStateEstimatorG(self.as_raw_mut_TrackerModel(), tracker_state_estimator.as_raw_mut_PtrOfTrackerStateEstimator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Estimate the most likely target location
	///
	/// [AAM](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_AAM) ME, Model Estimation table I
	/// ## Parameters
	/// * responses: Features extracted from TrackerFeatureSet
	// modelEstimation(const std::vector<Mat> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:471
	// ("cv::TrackerModel::modelEstimation", vec![(pred!(mut, ["responses"], ["const std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn model_estimation(&mut self, responses: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerModel_modelEstimation_const_vectorLMatGR(self.as_raw_mut_TrackerModel(), responses.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Update the model
	///
	/// [AAM](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_AAM) MU, Model Update table I
	// modelUpdate()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:477
	// ("cv::TrackerModel::modelUpdate", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn model_update(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerModel_modelUpdate(self.as_raw_mut_TrackerModel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Run the TrackerStateEstimator, return true if is possible to estimate a new state, false otherwise
	// runStateEstimator()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:481
	// ("cv::TrackerModel::runStateEstimator", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn run_state_estimator(&mut self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerModel_runStateEstimator(self.as_raw_mut_TrackerModel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set the current TrackerTargetState in the Trajectory
	/// ## Parameters
	/// * lastTargetState: The current TrackerTargetState
	// setLastTargetState(const Ptr<TrackerTargetState> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:486
	// ("cv::TrackerModel::setLastTargetState", vec![(pred!(mut, ["lastTargetState"], ["const cv::Ptr<cv::TrackerTargetState>*"]), _)]),
	#[inline]
	fn set_last_target_state(&mut self, last_target_state: &core::Ptr<crate::tracking::TrackerTargetState>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerModel_setLastTargetState_const_PtrLTrackerTargetStateGR(self.as_raw_mut_TrackerModel(), last_target_state.as_raw_PtrOfTrackerTargetState(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Abstract class that represents the model of the target. It must be instantiated by specialized
/// tracker
///
/// See [AAM](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_AAM) Ak
///
/// Inherits this with your TrackerModel
// TrackerModel /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:446
pub struct TrackerModel {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerModel }

impl Drop for TrackerModel {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerModel_delete(self.as_raw_mut_TrackerModel()) };
	}
}

unsafe impl Send for TrackerModel {}

impl crate::tracking::TrackerModelTraitConst for TrackerModel {
	#[inline] fn as_raw_TrackerModel(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerModelTrait for TrackerModel {
	#[inline] fn as_raw_mut_TrackerModel(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerModel, crate::tracking::TrackerModelTraitConst, as_raw_TrackerModel, crate::tracking::TrackerModelTrait, as_raw_mut_TrackerModel }

impl TrackerModel {
}

impl std::fmt::Debug for TrackerModel {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerModel")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerSampler]
// TrackerSampler /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:258
pub trait TrackerSamplerTraitConst {
	fn as_raw_TrackerSampler(&self) -> *const c_void;

	/// Return the collection of the TrackerSamplerAlgorithm
	// getSamplers()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:280
	// ("cv::TrackerSampler::getSamplers", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_samplers(&self) -> Result<core::Vector<core::Tuple<(String, core::Ptr<crate::tracking::TrackerSamplerAlgorithm>)>>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerSampler_getSamplers_const(self.as_raw_TrackerSampler(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Tuple<(String, core::Ptr<crate::tracking::TrackerSamplerAlgorithm>)>>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Return the samples from all TrackerSamplerAlgorithm, [AAM](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_AAM) Fig. 1 variable Sk
	// getSamples()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:284
	// ("cv::TrackerSampler::getSamples", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_samples(&self) -> Result<core::Vector<core::Mat>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerSampler_getSamples_const(self.as_raw_TrackerSampler(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Mat>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::TrackerSampler]
pub trait TrackerSamplerTrait: crate::tracking::TrackerSamplerTraitConst {
	fn as_raw_mut_TrackerSampler(&mut self) -> *mut c_void;

	/// Computes the regions starting from a position in an image
	/// ## Parameters
	/// * image: The current frame
	/// * boundingBox: The bounding box from which regions can be calculated
	// sampling(const Mat &, Rect)(TraitClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:276
	// ("cv::TrackerSampler::sampling", vec![(pred!(mut, ["image", "boundingBox"], ["const cv::Mat*", "cv::Rect"]), _)]),
	#[inline]
	fn sampling(&mut self, image: &impl core::MatTraitConst, bounding_box: core::Rect) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerSampler_sampling_const_MatR_Rect(self.as_raw_mut_TrackerSampler(), image.as_raw_Mat(), &bounding_box, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ```C++
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
	// addTrackerSamplerAlgorithm(String)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:307
	// ("cv::TrackerSampler::addTrackerSamplerAlgorithm", vec![(pred!(mut, ["trackerSamplerAlgorithmType"], ["cv::String"]), _)]),
	#[inline]
	fn add_tracker_sampler_algorithm(&mut self, tracker_sampler_algorithm_type: &str) -> Result<bool> {
		extern_container_arg!(tracker_sampler_algorithm_type);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerSampler_addTrackerSamplerAlgorithm_String(self.as_raw_mut_TrackerSampler(), tracker_sampler_algorithm_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	/// ```C++
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
	// addTrackerSamplerAlgorithm(Ptr<TrackerSamplerAlgorithm> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:312
	// ("cv::TrackerSampler::addTrackerSamplerAlgorithm", vec![(pred!(mut, ["sampler"], ["cv::Ptr<cv::TrackerSamplerAlgorithm>*"]), _)]),
	#[inline]
	fn add_tracker_sampler_algorithm_1(&mut self, sampler: &mut core::Ptr<crate::tracking::TrackerSamplerAlgorithm>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerSampler_addTrackerSamplerAlgorithm_PtrLTrackerSamplerAlgorithmGR(self.as_raw_mut_TrackerSampler(), sampler.as_raw_mut_PtrOfTrackerSamplerAlgorithm(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Class that manages the sampler in order to select regions for the update the model of the tracker
///
/// [AAM](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_AAM) Sampling e Labeling. See table I and section III B
///
/// TrackerSampler is an aggregation of TrackerSamplerAlgorithm
/// ## See also
/// TrackerSamplerAlgorithm
// TrackerSampler /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:258
pub struct TrackerSampler {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerSampler }

impl Drop for TrackerSampler {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerSampler_delete(self.as_raw_mut_TrackerSampler()) };
	}
}

unsafe impl Send for TrackerSampler {}

impl crate::tracking::TrackerSamplerTraitConst for TrackerSampler {
	#[inline] fn as_raw_TrackerSampler(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerSamplerTrait for TrackerSampler {
	#[inline] fn as_raw_mut_TrackerSampler(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerSampler, crate::tracking::TrackerSamplerTraitConst, as_raw_TrackerSampler, crate::tracking::TrackerSamplerTrait, as_raw_mut_TrackerSampler }

impl TrackerSampler {
	/// \brief Constructor
	// TrackerSampler()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:265
	// ("cv::TrackerSampler::TrackerSampler", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::tracking::TrackerSampler> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerSampler_TrackerSampler(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerSampler::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for TrackerSampler {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerSampler")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerSamplerAlgorithm]
// TrackerSamplerAlgorithm /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:206
pub trait TrackerSamplerAlgorithmTraitConst {
	fn as_raw_TrackerSamplerAlgorithm(&self) -> *const c_void;

	/// Get the name of the specific TrackerSamplerAlgorithm
	// getClassName()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:237
	// ("cv::TrackerSamplerAlgorithm::getClassName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_class_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerSamplerAlgorithm_getClassName_const(self.as_raw_TrackerSamplerAlgorithm(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::TrackerSamplerAlgorithm]
pub trait TrackerSamplerAlgorithmTrait: crate::tracking::TrackerSamplerAlgorithmTraitConst {
	fn as_raw_mut_TrackerSamplerAlgorithm(&mut self) -> *mut c_void;

	/// Computes the regions starting from a position in an image.
	///
	/// Return true if samples are computed, false otherwise
	///
	/// ## Parameters
	/// * image: The current frame
	/// * boundingBox: The bounding box from which regions can be calculated
	///
	/// * sample: The computed samples [AAM](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_AAM) Fig. 1 variable Sk
	// sampling(const Mat &, Rect, std::vector<Mat> &)(TraitClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:233
	// ("cv::TrackerSamplerAlgorithm::sampling", vec![(pred!(mut, ["image", "boundingBox", "sample"], ["const cv::Mat*", "cv::Rect", "std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn sampling(&mut self, image: &impl core::MatTraitConst, bounding_box: core::Rect, sample: &mut core::Vector<core::Mat>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerSamplerAlgorithm_sampling_const_MatR_Rect_vectorLMatGR(self.as_raw_mut_TrackerSamplerAlgorithm(), image.as_raw_Mat(), &bounding_box, sample.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Abstract base class for TrackerSamplerAlgorithm that represents the algorithm for the specific
/// sampler.
// TrackerSamplerAlgorithm /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:206
pub struct TrackerSamplerAlgorithm {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerSamplerAlgorithm }

impl Drop for TrackerSamplerAlgorithm {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerSamplerAlgorithm_delete(self.as_raw_mut_TrackerSamplerAlgorithm()) };
	}
}

unsafe impl Send for TrackerSamplerAlgorithm {}

impl crate::tracking::TrackerSamplerAlgorithmTraitConst for TrackerSamplerAlgorithm {
	#[inline] fn as_raw_TrackerSamplerAlgorithm(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerSamplerAlgorithmTrait for TrackerSamplerAlgorithm {
	#[inline] fn as_raw_mut_TrackerSamplerAlgorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerSamplerAlgorithm, crate::tracking::TrackerSamplerAlgorithmTraitConst, as_raw_TrackerSamplerAlgorithm, crate::tracking::TrackerSamplerAlgorithmTrait, as_raw_mut_TrackerSamplerAlgorithm }

impl TrackerSamplerAlgorithm {
	/// Create TrackerSamplerAlgorithm by tracker sampler type.
	/// ## Parameters
	/// * trackerSamplerType: The trackerSamplerType name
	///
	/// The modes available now:
	///
	/// *   "CSC" -- Current State Center
	/// *   "CS" -- Current State
	// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:222
	// ("cv::TrackerSamplerAlgorithm::create", vec![(pred!(mut, ["trackerSamplerType"], ["const cv::String*"]), _)]),
	#[inline]
	pub fn create(tracker_sampler_type: &str) -> Result<core::Ptr<crate::tracking::TrackerSamplerAlgorithm>> {
		extern_container_arg!(tracker_sampler_type);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerSamplerAlgorithm_create_const_StringR(tracker_sampler_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerSamplerAlgorithm>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_descendant! { TrackerSamplerAlgorithm, crate::tracking::TrackerSamplerCS, cv_TrackerSamplerAlgorithm_to_TrackerSamplerCS }

boxed_cast_descendant! { TrackerSamplerAlgorithm, crate::tracking::TrackerSamplerCSC, cv_TrackerSamplerAlgorithm_to_TrackerSamplerCSC }

boxed_cast_descendant! { TrackerSamplerAlgorithm, crate::tracking::TrackerSamplerPF, cv_TrackerSamplerAlgorithm_to_TrackerSamplerPF }

impl std::fmt::Debug for TrackerSamplerAlgorithm {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerSamplerAlgorithm")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerSamplerCS]
// TrackerSamplerCS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:829
pub trait TrackerSamplerCSTraitConst: crate::tracking::TrackerSamplerAlgorithmTraitConst {
	fn as_raw_TrackerSamplerCS(&self) -> *const c_void;

	// getROI()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:864
	// ("cv::TrackerSamplerCS::getROI", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_roi(&self) -> Result<core::Rect> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerSamplerCS_getROI_const(self.as_raw_TrackerSamplerCS(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::TrackerSamplerCS]
pub trait TrackerSamplerCSTrait: crate::tracking::TrackerSamplerAlgorithmTrait + crate::tracking::TrackerSamplerCSTraitConst {
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
	// setMode(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:859
	// ("cv::TrackerSamplerCS::setMode", vec![(pred!(mut, ["samplingMode"], ["int"]), _)]),
	#[inline]
	fn set_mode(&mut self, sampling_mode: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerSamplerCS_setMode_int(self.as_raw_mut_TrackerSamplerCS(), sampling_mode, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// samplingImpl(const Mat &, Rect, std::vector<Mat> &)(TraitClass, SimpleClass, CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:863
	// ("cv::TrackerSamplerCS::samplingImpl", vec![(pred!(mut, ["image", "boundingBox", "sample"], ["const cv::Mat*", "cv::Rect", "std::vector<cv::Mat>*"]), _)]),
	#[inline]
	fn sampling_impl(&mut self, image: &impl core::MatTraitConst, bounding_box: core::Rect, sample: &mut core::Vector<core::Mat>) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerSamplerCS_samplingImpl_const_MatR_Rect_vectorLMatGR(self.as_raw_mut_TrackerSamplerCS(), image.as_raw_Mat(), &bounding_box, sample.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// TrackerSampler based on CS (current state), used by algorithm TrackerBoosting
// TrackerSamplerCS /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:829
pub struct TrackerSamplerCS {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerSamplerCS }

impl Drop for TrackerSamplerCS {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerSamplerCS_delete(self.as_raw_mut_TrackerSamplerCS()) };
	}
}

unsafe impl Send for TrackerSamplerCS {}

impl crate::tracking::TrackerSamplerAlgorithmTraitConst for TrackerSamplerCS {
	#[inline] fn as_raw_TrackerSamplerAlgorithm(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerSamplerAlgorithmTrait for TrackerSamplerCS {
	#[inline] fn as_raw_mut_TrackerSamplerAlgorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerSamplerCS, crate::tracking::TrackerSamplerAlgorithmTraitConst, as_raw_TrackerSamplerAlgorithm, crate::tracking::TrackerSamplerAlgorithmTrait, as_raw_mut_TrackerSamplerAlgorithm }

impl crate::tracking::TrackerSamplerCSTraitConst for TrackerSamplerCS {
	#[inline] fn as_raw_TrackerSamplerCS(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerSamplerCSTrait for TrackerSamplerCS {
	#[inline] fn as_raw_mut_TrackerSamplerCS(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerSamplerCS, crate::tracking::TrackerSamplerCSTraitConst, as_raw_TrackerSamplerCS, crate::tracking::TrackerSamplerCSTrait, as_raw_mut_TrackerSamplerCS }

impl TrackerSamplerCS {
	/// Constructor
	/// ## Parameters
	/// * parameters: TrackerSamplerCS parameters TrackerSamplerCS::Params
	///
	/// ## C++ default parameters
	/// * parameters: TrackerSamplerCS::Params()
	// TrackerSamplerCS(const TrackerSamplerCS::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:848
	// ("cv::TrackerSamplerCS::TrackerSamplerCS", vec![(pred!(mut, ["parameters"], ["const cv::TrackerSamplerCS::Params*"]), _)]),
	#[inline]
	pub fn new(parameters: &impl crate::tracking::TrackerSamplerCS_ParamsTraitConst) -> Result<crate::tracking::TrackerSamplerCS> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerSamplerCS_TrackerSamplerCS_const_ParamsR(parameters.as_raw_TrackerSamplerCS_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerSamplerCS::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor
	/// ## Parameters
	/// * parameters: TrackerSamplerCS parameters TrackerSamplerCS::Params
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * parameters: TrackerSamplerCS::Params()
	// cv::TrackerSamplerCS::TrackerSamplerCS() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:848
	// ("cv::TrackerSamplerCS::TrackerSamplerCS", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::tracking::TrackerSamplerCS> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerSamplerCS_TrackerSamplerCS(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerSamplerCS::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TrackerSamplerCS, crate::tracking::TrackerSamplerAlgorithm, cv_TrackerSamplerCS_to_TrackerSamplerAlgorithm }

impl std::fmt::Debug for TrackerSamplerCS {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerSamplerCS")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerSamplerCS_Params]
// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:839
pub trait TrackerSamplerCS_ParamsTraitConst {
	fn as_raw_TrackerSamplerCS_Params(&self) -> *const c_void;

	/// overlapping for the search windows
	// cv::TrackerSamplerCS::Params::overlap() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:842
	// ("cv::TrackerSamplerCS::Params::overlap", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn overlap(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerSamplerCS_Params_propOverlap_const(self.as_raw_TrackerSamplerCS_Params()) };
		ret
	}

	/// search region parameter
	// cv::TrackerSamplerCS::Params::searchFactor() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:843
	// ("cv::TrackerSamplerCS::Params::searchFactor", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn search_factor(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerSamplerCS_Params_propSearchFactor_const(self.as_raw_TrackerSamplerCS_Params()) };
		ret
	}

}

/// Mutable methods for [crate::tracking::TrackerSamplerCS_Params]
pub trait TrackerSamplerCS_ParamsTrait: crate::tracking::TrackerSamplerCS_ParamsTraitConst {
	fn as_raw_mut_TrackerSamplerCS_Params(&mut self) -> *mut c_void;

	/// overlapping for the search windows
	// cv::TrackerSamplerCS::Params::setOverlap(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:842
	// ("cv::TrackerSamplerCS::Params::setOverlap", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_overlap(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerSamplerCS_Params_propOverlap_const_float(self.as_raw_mut_TrackerSamplerCS_Params(), val) };
		ret
	}

	/// search region parameter
	// cv::TrackerSamplerCS::Params::setSearchFactor(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:843
	// ("cv::TrackerSamplerCS::Params::setSearchFactor", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_search_factor(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerSamplerCS_Params_propSearchFactor_const_float(self.as_raw_mut_TrackerSamplerCS_Params(), val) };
		ret
	}

}

// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:839
pub struct TrackerSamplerCS_Params {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerSamplerCS_Params }

impl Drop for TrackerSamplerCS_Params {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerSamplerCS_Params_delete(self.as_raw_mut_TrackerSamplerCS_Params()) };
	}
}

unsafe impl Send for TrackerSamplerCS_Params {}

impl crate::tracking::TrackerSamplerCS_ParamsTraitConst for TrackerSamplerCS_Params {
	#[inline] fn as_raw_TrackerSamplerCS_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerSamplerCS_ParamsTrait for TrackerSamplerCS_Params {
	#[inline] fn as_raw_mut_TrackerSamplerCS_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerSamplerCS_Params, crate::tracking::TrackerSamplerCS_ParamsTraitConst, as_raw_TrackerSamplerCS_Params, crate::tracking::TrackerSamplerCS_ParamsTrait, as_raw_mut_TrackerSamplerCS_Params }

impl TrackerSamplerCS_Params {
	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:841
	// ("cv::TrackerSamplerCS::Params::Params", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::tracking::TrackerSamplerCS_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerSamplerCS_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerSamplerCS_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for TrackerSamplerCS_Params {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerSamplerCS_Params")
			.field("overlap", &crate::tracking::TrackerSamplerCS_ParamsTraitConst::overlap(self))
			.field("search_factor", &crate::tracking::TrackerSamplerCS_ParamsTraitConst::search_factor(self))
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerSamplerCSC]
// TrackerSamplerCSC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:771
pub trait TrackerSamplerCSCTraitConst: crate::tracking::TrackerSamplerAlgorithmTraitConst {
	fn as_raw_TrackerSamplerCSC(&self) -> *const c_void;

}

/// Mutable methods for [crate::tracking::TrackerSamplerCSC]
pub trait TrackerSamplerCSCTrait: crate::tracking::TrackerSamplerAlgorithmTrait + crate::tracking::TrackerSamplerCSCTraitConst {
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
	// setMode(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:810
	// ("cv::TrackerSamplerCSC::setMode", vec![(pred!(mut, ["samplingMode"], ["int"]), _)]),
	#[inline]
	fn set_mode(&mut self, sampling_mode: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerSamplerCSC_setMode_int(self.as_raw_mut_TrackerSamplerCSC(), sampling_mode, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// TrackerSampler based on CSC (current state centered), used by MIL algorithm TrackerMIL
// TrackerSamplerCSC /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:771
pub struct TrackerSamplerCSC {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerSamplerCSC }

impl Drop for TrackerSamplerCSC {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerSamplerCSC_delete(self.as_raw_mut_TrackerSamplerCSC()) };
	}
}

unsafe impl Send for TrackerSamplerCSC {}

impl crate::tracking::TrackerSamplerAlgorithmTraitConst for TrackerSamplerCSC {
	#[inline] fn as_raw_TrackerSamplerAlgorithm(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerSamplerAlgorithmTrait for TrackerSamplerCSC {
	#[inline] fn as_raw_mut_TrackerSamplerAlgorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerSamplerCSC, crate::tracking::TrackerSamplerAlgorithmTraitConst, as_raw_TrackerSamplerAlgorithm, crate::tracking::TrackerSamplerAlgorithmTrait, as_raw_mut_TrackerSamplerAlgorithm }

impl crate::tracking::TrackerSamplerCSCTraitConst for TrackerSamplerCSC {
	#[inline] fn as_raw_TrackerSamplerCSC(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerSamplerCSCTrait for TrackerSamplerCSC {
	#[inline] fn as_raw_mut_TrackerSamplerCSC(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerSamplerCSC, crate::tracking::TrackerSamplerCSCTraitConst, as_raw_TrackerSamplerCSC, crate::tracking::TrackerSamplerCSCTrait, as_raw_mut_TrackerSamplerCSC }

impl TrackerSamplerCSC {
	/// Constructor
	/// ## Parameters
	/// * parameters: TrackerSamplerCSC parameters TrackerSamplerCSC::Params
	///
	/// ## C++ default parameters
	/// * parameters: TrackerSamplerCSC::Params()
	// TrackerSamplerCSC(const TrackerSamplerCSC::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:797
	// ("cv::TrackerSamplerCSC::TrackerSamplerCSC", vec![(pred!(mut, ["parameters"], ["const cv::TrackerSamplerCSC::Params*"]), _)]),
	#[inline]
	pub fn new(parameters: &impl crate::tracking::TrackerSamplerCSC_ParamsTraitConst) -> Result<crate::tracking::TrackerSamplerCSC> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerSamplerCSC_TrackerSamplerCSC_const_ParamsR(parameters.as_raw_TrackerSamplerCSC_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerSamplerCSC::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor
	/// ## Parameters
	/// * parameters: TrackerSamplerCSC parameters TrackerSamplerCSC::Params
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * parameters: TrackerSamplerCSC::Params()
	// cv::TrackerSamplerCSC::TrackerSamplerCSC() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:797
	// ("cv::TrackerSamplerCSC::TrackerSamplerCSC", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::tracking::TrackerSamplerCSC> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerSamplerCSC_TrackerSamplerCSC(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerSamplerCSC::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TrackerSamplerCSC, crate::tracking::TrackerSamplerAlgorithm, cv_TrackerSamplerCSC_to_TrackerSamplerAlgorithm }

impl std::fmt::Debug for TrackerSamplerCSC {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerSamplerCSC")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerSamplerCSC_Params]
// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:783
pub trait TrackerSamplerCSC_ParamsTraitConst {
	fn as_raw_TrackerSamplerCSC_Params(&self) -> *const c_void;

	/// radius for gathering positive instances during init
	// cv::TrackerSamplerCSC::Params::initInRad() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:786
	// ("cv::TrackerSamplerCSC::Params::initInRad", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn init_in_rad(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerSamplerCSC_Params_propInitInRad_const(self.as_raw_TrackerSamplerCSC_Params()) };
		ret
	}

	/// radius for gathering positive instances during tracking
	// cv::TrackerSamplerCSC::Params::trackInPosRad() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:787
	// ("cv::TrackerSamplerCSC::Params::trackInPosRad", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn track_in_pos_rad(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerSamplerCSC_Params_propTrackInPosRad_const(self.as_raw_TrackerSamplerCSC_Params()) };
		ret
	}

	/// size of search window
	// cv::TrackerSamplerCSC::Params::searchWinSize() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:788
	// ("cv::TrackerSamplerCSC::Params::searchWinSize", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn search_win_size(&self) -> f32 {
		let ret = unsafe { sys::cv_TrackerSamplerCSC_Params_propSearchWinSize_const(self.as_raw_TrackerSamplerCSC_Params()) };
		ret
	}

	/// # negative samples to use during init
	// cv::TrackerSamplerCSC::Params::initMaxNegNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:789
	// ("cv::TrackerSamplerCSC::Params::initMaxNegNum", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn init_max_neg_num(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerSamplerCSC_Params_propInitMaxNegNum_const(self.as_raw_TrackerSamplerCSC_Params()) };
		ret
	}

	/// # positive samples to use during training
	// cv::TrackerSamplerCSC::Params::trackMaxPosNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:790
	// ("cv::TrackerSamplerCSC::Params::trackMaxPosNum", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn track_max_pos_num(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerSamplerCSC_Params_propTrackMaxPosNum_const(self.as_raw_TrackerSamplerCSC_Params()) };
		ret
	}

	/// # negative samples to use during training
	// cv::TrackerSamplerCSC::Params::trackMaxNegNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:791
	// ("cv::TrackerSamplerCSC::Params::trackMaxNegNum", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn track_max_neg_num(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerSamplerCSC_Params_propTrackMaxNegNum_const(self.as_raw_TrackerSamplerCSC_Params()) };
		ret
	}

}

/// Mutable methods for [crate::tracking::TrackerSamplerCSC_Params]
pub trait TrackerSamplerCSC_ParamsTrait: crate::tracking::TrackerSamplerCSC_ParamsTraitConst {
	fn as_raw_mut_TrackerSamplerCSC_Params(&mut self) -> *mut c_void;

	/// radius for gathering positive instances during init
	// cv::TrackerSamplerCSC::Params::setInitInRad(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:786
	// ("cv::TrackerSamplerCSC::Params::setInitInRad", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_init_in_rad(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerSamplerCSC_Params_propInitInRad_const_float(self.as_raw_mut_TrackerSamplerCSC_Params(), val) };
		ret
	}

	/// radius for gathering positive instances during tracking
	// cv::TrackerSamplerCSC::Params::setTrackInPosRad(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:787
	// ("cv::TrackerSamplerCSC::Params::setTrackInPosRad", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_track_in_pos_rad(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerSamplerCSC_Params_propTrackInPosRad_const_float(self.as_raw_mut_TrackerSamplerCSC_Params(), val) };
		ret
	}

	/// size of search window
	// cv::TrackerSamplerCSC::Params::setSearchWinSize(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:788
	// ("cv::TrackerSamplerCSC::Params::setSearchWinSize", vec![(pred!(mut, ["val"], ["const float"]), _)]),
	#[inline]
	fn set_search_win_size(&mut self, val: f32) {
		let ret = unsafe { sys::cv_TrackerSamplerCSC_Params_propSearchWinSize_const_float(self.as_raw_mut_TrackerSamplerCSC_Params(), val) };
		ret
	}

	/// # negative samples to use during init
	// cv::TrackerSamplerCSC::Params::setInitMaxNegNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:789
	// ("cv::TrackerSamplerCSC::Params::setInitMaxNegNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_init_max_neg_num(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerSamplerCSC_Params_propInitMaxNegNum_const_int(self.as_raw_mut_TrackerSamplerCSC_Params(), val) };
		ret
	}

	/// # positive samples to use during training
	// cv::TrackerSamplerCSC::Params::setTrackMaxPosNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:790
	// ("cv::TrackerSamplerCSC::Params::setTrackMaxPosNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_track_max_pos_num(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerSamplerCSC_Params_propTrackMaxPosNum_const_int(self.as_raw_mut_TrackerSamplerCSC_Params(), val) };
		ret
	}

	/// # negative samples to use during training
	// cv::TrackerSamplerCSC::Params::setTrackMaxNegNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:791
	// ("cv::TrackerSamplerCSC::Params::setTrackMaxNegNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_track_max_neg_num(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerSamplerCSC_Params_propTrackMaxNegNum_const_int(self.as_raw_mut_TrackerSamplerCSC_Params(), val) };
		ret
	}

}

// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:783
pub struct TrackerSamplerCSC_Params {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerSamplerCSC_Params }

impl Drop for TrackerSamplerCSC_Params {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerSamplerCSC_Params_delete(self.as_raw_mut_TrackerSamplerCSC_Params()) };
	}
}

unsafe impl Send for TrackerSamplerCSC_Params {}

impl crate::tracking::TrackerSamplerCSC_ParamsTraitConst for TrackerSamplerCSC_Params {
	#[inline] fn as_raw_TrackerSamplerCSC_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerSamplerCSC_ParamsTrait for TrackerSamplerCSC_Params {
	#[inline] fn as_raw_mut_TrackerSamplerCSC_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerSamplerCSC_Params, crate::tracking::TrackerSamplerCSC_ParamsTraitConst, as_raw_TrackerSamplerCSC_Params, crate::tracking::TrackerSamplerCSC_ParamsTrait, as_raw_mut_TrackerSamplerCSC_Params }

impl TrackerSamplerCSC_Params {
	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:785
	// ("cv::TrackerSamplerCSC::Params::Params", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::tracking::TrackerSamplerCSC_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerSamplerCSC_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerSamplerCSC_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for TrackerSamplerCSC_Params {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerSamplerCSC_Params")
			.field("init_in_rad", &crate::tracking::TrackerSamplerCSC_ParamsTraitConst::init_in_rad(self))
			.field("track_in_pos_rad", &crate::tracking::TrackerSamplerCSC_ParamsTraitConst::track_in_pos_rad(self))
			.field("search_win_size", &crate::tracking::TrackerSamplerCSC_ParamsTraitConst::search_win_size(self))
			.field("init_max_neg_num", &crate::tracking::TrackerSamplerCSC_ParamsTraitConst::init_max_neg_num(self))
			.field("track_max_pos_num", &crate::tracking::TrackerSamplerCSC_ParamsTraitConst::track_max_pos_num(self))
			.field("track_max_neg_num", &crate::tracking::TrackerSamplerCSC_ParamsTraitConst::track_max_neg_num(self))
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerSamplerPF]
// TrackerSamplerPF /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:896
pub trait TrackerSamplerPFTraitConst: crate::tracking::TrackerSamplerAlgorithmTraitConst {
	fn as_raw_TrackerSamplerPF(&self) -> *const c_void;

}

/// Mutable methods for [crate::tracking::TrackerSamplerPF]
pub trait TrackerSamplerPFTrait: crate::tracking::TrackerSamplerAlgorithmTrait + crate::tracking::TrackerSamplerPFTraitConst {
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
// TrackerSamplerPF /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:896
pub struct TrackerSamplerPF {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerSamplerPF }

impl Drop for TrackerSamplerPF {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerSamplerPF_delete(self.as_raw_mut_TrackerSamplerPF()) };
	}
}

unsafe impl Send for TrackerSamplerPF {}

impl crate::tracking::TrackerSamplerAlgorithmTraitConst for TrackerSamplerPF {
	#[inline] fn as_raw_TrackerSamplerAlgorithm(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerSamplerAlgorithmTrait for TrackerSamplerPF {
	#[inline] fn as_raw_mut_TrackerSamplerAlgorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerSamplerPF, crate::tracking::TrackerSamplerAlgorithmTraitConst, as_raw_TrackerSamplerAlgorithm, crate::tracking::TrackerSamplerAlgorithmTrait, as_raw_mut_TrackerSamplerAlgorithm }

impl crate::tracking::TrackerSamplerPFTraitConst for TrackerSamplerPF {
	#[inline] fn as_raw_TrackerSamplerPF(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerSamplerPFTrait for TrackerSamplerPF {
	#[inline] fn as_raw_mut_TrackerSamplerPF(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerSamplerPF, crate::tracking::TrackerSamplerPFTraitConst, as_raw_TrackerSamplerPF, crate::tracking::TrackerSamplerPFTrait, as_raw_mut_TrackerSamplerPF }

impl TrackerSamplerPF {
	/// Constructor
	/// ## Parameters
	/// * chosenRect: Initial rectangle, that is supposed to contain target we'd like to track.
	/// * parameters: 
	///
	/// ## C++ default parameters
	/// * parameters: TrackerSamplerPF::Params()
	// TrackerSamplerPF(const Mat &, const TrackerSamplerPF::Params &)(TraitClass, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:917
	// ("cv::TrackerSamplerPF::TrackerSamplerPF", vec![(pred!(mut, ["chosenRect", "parameters"], ["const cv::Mat*", "const cv::TrackerSamplerPF::Params*"]), _)]),
	#[inline]
	pub fn new(chosen_rect: &impl core::MatTraitConst, parameters: &impl crate::tracking::TrackerSamplerPF_ParamsTraitConst) -> Result<crate::tracking::TrackerSamplerPF> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerSamplerPF_TrackerSamplerPF_const_MatR_const_ParamsR(chosen_rect.as_raw_Mat(), parameters.as_raw_TrackerSamplerPF_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerSamplerPF::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor
	/// ## Parameters
	/// * chosenRect: Initial rectangle, that is supposed to contain target we'd like to track.
	/// * parameters: 
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * parameters: TrackerSamplerPF::Params()
	// cv::TrackerSamplerPF::TrackerSamplerPF(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:917
	// ("cv::TrackerSamplerPF::TrackerSamplerPF", vec![(pred!(mut, ["chosenRect"], ["const cv::Mat*"]), _)]),
	#[inline]
	pub fn new_def(chosen_rect: &impl core::MatTraitConst) -> Result<crate::tracking::TrackerSamplerPF> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerSamplerPF_TrackerSamplerPF_const_MatR(chosen_rect.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerSamplerPF::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TrackerSamplerPF, crate::tracking::TrackerSamplerAlgorithm, cv_TrackerSamplerPF_to_TrackerSamplerAlgorithm }

impl std::fmt::Debug for TrackerSamplerPF {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerSamplerPF")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerSamplerPF_Params]
// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:903
pub trait TrackerSamplerPF_ParamsTraitConst {
	fn as_raw_TrackerSamplerPF_Params(&self) -> *const c_void;

	/// number of selection rounds
	// cv::TrackerSamplerPF::Params::iterationNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:906
	// ("cv::TrackerSamplerPF::Params::iterationNum", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn iteration_num(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerSamplerPF_Params_propIterationNum_const(self.as_raw_TrackerSamplerPF_Params()) };
		ret
	}

	/// number of "perturbed" boxes on each round
	// cv::TrackerSamplerPF::Params::particlesNum() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:907
	// ("cv::TrackerSamplerPF::Params::particlesNum", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn particles_num(&self) -> i32 {
		let ret = unsafe { sys::cv_TrackerSamplerPF_Params_propParticlesNum_const(self.as_raw_TrackerSamplerPF_Params()) };
		ret
	}

	/// with each new round we exponentially decrease the amount of "perturbing" we allow (like in simulated annealing)
	/// and this very alpha controls how fast annealing happens, ie. how fast perturbing decreases
	// cv::TrackerSamplerPF::Params::alpha() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:908
	// ("cv::TrackerSamplerPF::Params::alpha", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn alpha(&self) -> f64 {
		let ret = unsafe { sys::cv_TrackerSamplerPF_Params_propAlpha_const(self.as_raw_TrackerSamplerPF_Params()) };
		ret
	}

	/// initial values for perturbing (1-by-4 array, as each rectangle is given by 4 values -- coordinates of opposite vertices,
	/// hence we have 4 values to perturb)
	// cv::TrackerSamplerPF::Params::std() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:910
	// ("cv::TrackerSamplerPF::Params::std", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn std(&self) -> core::Mat_<f64> {
		let ret = unsafe { sys::cv_TrackerSamplerPF_Params_propStd_const(self.as_raw_TrackerSamplerPF_Params()) };
		let ret = unsafe { core::Mat_::<f64>::opencv_from_extern(ret) };
		ret
	}

}

/// Mutable methods for [crate::tracking::TrackerSamplerPF_Params]
pub trait TrackerSamplerPF_ParamsTrait: crate::tracking::TrackerSamplerPF_ParamsTraitConst {
	fn as_raw_mut_TrackerSamplerPF_Params(&mut self) -> *mut c_void;

	/// number of selection rounds
	// cv::TrackerSamplerPF::Params::setIterationNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:906
	// ("cv::TrackerSamplerPF::Params::setIterationNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_iteration_num(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerSamplerPF_Params_propIterationNum_const_int(self.as_raw_mut_TrackerSamplerPF_Params(), val) };
		ret
	}

	/// number of "perturbed" boxes on each round
	// cv::TrackerSamplerPF::Params::setParticlesNum(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:907
	// ("cv::TrackerSamplerPF::Params::setParticlesNum", vec![(pred!(mut, ["val"], ["const int"]), _)]),
	#[inline]
	fn set_particles_num(&mut self, val: i32) {
		let ret = unsafe { sys::cv_TrackerSamplerPF_Params_propParticlesNum_const_int(self.as_raw_mut_TrackerSamplerPF_Params(), val) };
		ret
	}

	/// with each new round we exponentially decrease the amount of "perturbing" we allow (like in simulated annealing)
	/// and this very alpha controls how fast annealing happens, ie. how fast perturbing decreases
	// cv::TrackerSamplerPF::Params::setAlpha(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:908
	// ("cv::TrackerSamplerPF::Params::setAlpha", vec![(pred!(mut, ["val"], ["const double"]), _)]),
	#[inline]
	fn set_alpha(&mut self, val: f64) {
		let ret = unsafe { sys::cv_TrackerSamplerPF_Params_propAlpha_const_double(self.as_raw_mut_TrackerSamplerPF_Params(), val) };
		ret
	}

	/// initial values for perturbing (1-by-4 array, as each rectangle is given by 4 values -- coordinates of opposite vertices,
	/// hence we have 4 values to perturb)
	// cv::TrackerSamplerPF::Params::setStd(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:910
	// ("cv::TrackerSamplerPF::Params::setStd", vec![(pred!(mut, ["val"], ["const cv::Mat_<double>"]), _)]),
	#[inline]
	fn set_std(&mut self, val: core::Mat_<f64>) {
		let ret = unsafe { sys::cv_TrackerSamplerPF_Params_propStd_const_Mat_LdoubleG(self.as_raw_mut_TrackerSamplerPF_Params(), val.as_raw_Mat_()) };
		ret
	}

}

/// This structure contains all the parameters that can be varied during the course of sampling
/// algorithm. Below is the structure exposed, together with its members briefly explained with
/// reference to the above discussion on algorithm's working.
// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:903
pub struct TrackerSamplerPF_Params {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerSamplerPF_Params }

impl Drop for TrackerSamplerPF_Params {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerSamplerPF_Params_delete(self.as_raw_mut_TrackerSamplerPF_Params()) };
	}
}

unsafe impl Send for TrackerSamplerPF_Params {}

impl crate::tracking::TrackerSamplerPF_ParamsTraitConst for TrackerSamplerPF_Params {
	#[inline] fn as_raw_TrackerSamplerPF_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerSamplerPF_ParamsTrait for TrackerSamplerPF_Params {
	#[inline] fn as_raw_mut_TrackerSamplerPF_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerSamplerPF_Params, crate::tracking::TrackerSamplerPF_ParamsTraitConst, as_raw_TrackerSamplerPF_Params, crate::tracking::TrackerSamplerPF_ParamsTrait, as_raw_mut_TrackerSamplerPF_Params }

impl TrackerSamplerPF_Params {
	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:905
	// ("cv::TrackerSamplerPF::Params::Params", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::tracking::TrackerSamplerPF_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerSamplerPF_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerSamplerPF_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for TrackerSamplerPF_Params {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerSamplerPF_Params")
			.field("iteration_num", &crate::tracking::TrackerSamplerPF_ParamsTraitConst::iteration_num(self))
			.field("particles_num", &crate::tracking::TrackerSamplerPF_ParamsTraitConst::particles_num(self))
			.field("alpha", &crate::tracking::TrackerSamplerPF_ParamsTraitConst::alpha(self))
			.field("std", &crate::tracking::TrackerSamplerPF_ParamsTraitConst::std(self))
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerStateEstimator]
// TrackerStateEstimator /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:400
pub trait TrackerStateEstimatorTraitConst {
	fn as_raw_TrackerStateEstimator(&self) -> *const c_void;

	/// Get the name of the specific TrackerStateEstimator
	// getClassName()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:430
	// ("cv::TrackerStateEstimator::getClassName", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_class_name(&self) -> Result<String> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimator_getClassName_const(self.as_raw_TrackerStateEstimator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::TrackerStateEstimator]
pub trait TrackerStateEstimatorTrait: crate::tracking::TrackerStateEstimatorTraitConst {
	fn as_raw_mut_TrackerStateEstimator(&mut self) -> *mut c_void;

	/// Estimate the most likely target state, return the estimated state
	/// ## Parameters
	/// * confidenceMaps: The overall appearance model as a list of :cConfidenceMap
	// estimate(const std::vector<ConfidenceMap> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:408
	// ("cv::TrackerStateEstimator::estimate", vec![(pred!(mut, ["confidenceMaps"], ["const std::vector<cv::ConfidenceMap>*"]), _)]),
	#[inline]
	fn estimate(&mut self, confidence_maps: &core::Vector<crate::tracking::ConfidenceMap>) -> Result<core::Ptr<crate::tracking::TrackerTargetState>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimator_estimate_const_vectorLConfidenceMapGR(self.as_raw_mut_TrackerStateEstimator(), confidence_maps.as_raw_VectorOfConfidenceMap(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerTargetState>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Update the ConfidenceMap with the scores
	/// ## Parameters
	/// * confidenceMaps: The overall appearance model as a list of :cConfidenceMap
	// update(std::vector<ConfidenceMap> &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:413
	// ("cv::TrackerStateEstimator::update", vec![(pred!(mut, ["confidenceMaps"], ["std::vector<cv::ConfidenceMap>*"]), _)]),
	#[inline]
	fn update(&mut self, confidence_maps: &mut core::Vector<crate::tracking::ConfidenceMap>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimator_update_vectorLConfidenceMapGR(self.as_raw_mut_TrackerStateEstimator(), confidence_maps.as_raw_mut_VectorOfConfidenceMap(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Abstract base class for TrackerStateEstimator that estimates the most likely target state.
///
/// See [AAM](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_AAM) State estimator
///
/// See [AMVOT](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_AMVOT) Statistical modeling (Fig. 3), Table III (generative) - IV (discriminative) - V (hybrid)
// TrackerStateEstimator /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:400
pub struct TrackerStateEstimator {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerStateEstimator }

impl Drop for TrackerStateEstimator {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerStateEstimator_delete(self.as_raw_mut_TrackerStateEstimator()) };
	}
}

unsafe impl Send for TrackerStateEstimator {}

impl crate::tracking::TrackerStateEstimatorTraitConst for TrackerStateEstimator {
	#[inline] fn as_raw_TrackerStateEstimator(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerStateEstimatorTrait for TrackerStateEstimator {
	#[inline] fn as_raw_mut_TrackerStateEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerStateEstimator, crate::tracking::TrackerStateEstimatorTraitConst, as_raw_TrackerStateEstimator, crate::tracking::TrackerStateEstimatorTrait, as_raw_mut_TrackerStateEstimator }

impl TrackerStateEstimator {
	/// Create TrackerStateEstimator by tracker state estimator type
	/// ## Parameters
	/// * trackeStateEstimatorType: The TrackerStateEstimator name
	///
	/// The modes available now:
	///
	/// *   "BOOSTING" -- Boosting-based discriminative appearance models. See [AMVOT](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_AMVOT) section 4.4
	///
	/// The modes available soon:
	///
	/// *   "SVM" -- SVM-based discriminative appearance models. See [AMVOT](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_AMVOT) section 4.5
	// create(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:426
	// ("cv::TrackerStateEstimator::create", vec![(pred!(mut, ["trackeStateEstimatorType"], ["const cv::String*"]), _)]),
	#[inline]
	pub fn create(tracke_state_estimator_type: &str) -> Result<core::Ptr<crate::tracking::TrackerStateEstimator>> {
		extern_container_arg!(tracke_state_estimator_type);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimator_create_const_StringR(tracke_state_estimator_type.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerStateEstimator>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_descendant! { TrackerStateEstimator, crate::tracking::TrackerStateEstimatorAdaBoosting, cv_TrackerStateEstimator_to_TrackerStateEstimatorAdaBoosting }

boxed_cast_descendant! { TrackerStateEstimator, crate::tracking::TrackerStateEstimatorMILBoosting, cv_TrackerStateEstimator_to_TrackerStateEstimatorMILBoosting }

boxed_cast_descendant! { TrackerStateEstimator, crate::tracking::TrackerStateEstimatorSVM, cv_TrackerStateEstimator_to_TrackerStateEstimatorSVM }

impl std::fmt::Debug for TrackerStateEstimator {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerStateEstimator")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerStateEstimatorAdaBoosting]
// TrackerStateEstimatorAdaBoosting /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:646
pub trait TrackerStateEstimatorAdaBoostingTraitConst: crate::tracking::TrackerStateEstimatorTraitConst {
	fn as_raw_TrackerStateEstimatorAdaBoosting(&self) -> *const c_void;

	/// Get the sampling ROI
	// getSampleROI()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:710
	// ("cv::TrackerStateEstimatorAdaBoosting::getSampleROI", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_sample_roi(&self) -> Result<core::Rect> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_getSampleROI_const(self.as_raw_TrackerStateEstimatorAdaBoosting(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::TrackerStateEstimatorAdaBoosting]
pub trait TrackerStateEstimatorAdaBoostingTrait: crate::tracking::TrackerStateEstimatorAdaBoostingTraitConst + crate::tracking::TrackerStateEstimatorTrait {
	fn as_raw_mut_TrackerStateEstimatorAdaBoosting(&mut self) -> *mut c_void;

	/// Set the sampling ROI
	/// ## Parameters
	/// * ROI: the sampling ROI
	// setSampleROI(const Rect &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:715
	// ("cv::TrackerStateEstimatorAdaBoosting::setSampleROI", vec![(pred!(mut, ["ROI"], ["const cv::Rect*"]), _)]),
	#[inline]
	fn set_sample_roi(&mut self, roi: core::Rect) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_setSampleROI_const_RectR(self.as_raw_mut_TrackerStateEstimatorAdaBoosting(), &roi, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set the current confidenceMap
	/// ## Parameters
	/// * confidenceMap: The current :cConfidenceMap
	// setCurrentConfidenceMap(ConfidenceMap &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:720
	// ("cv::TrackerStateEstimatorAdaBoosting::setCurrentConfidenceMap", vec![(pred!(mut, ["confidenceMap"], ["cv::ConfidenceMap*"]), _)]),
	#[inline]
	fn set_current_confidence_map(&mut self, confidence_map: &mut crate::tracking::ConfidenceMap) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_setCurrentConfidenceMap_ConfidenceMapR(self.as_raw_mut_TrackerStateEstimatorAdaBoosting(), confidence_map.as_raw_mut_VectorOfTupleOfPtrOfTrackerTargetState_f32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get the list of the selected weak classifiers for the classification step
	// computeSelectedWeakClassifier()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:724
	// ("cv::TrackerStateEstimatorAdaBoosting::computeSelectedWeakClassifier", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn compute_selected_weak_classifier(&mut self) -> Result<core::Vector<i32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_computeSelectedWeakClassifier(self.as_raw_mut_TrackerStateEstimatorAdaBoosting(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Get the list of the weak classifiers that should be replaced
	// computeReplacedClassifier()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:728
	// ("cv::TrackerStateEstimatorAdaBoosting::computeReplacedClassifier", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn compute_replaced_classifier(&mut self) -> Result<core::Vector<i32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_computeReplacedClassifier(self.as_raw_mut_TrackerStateEstimatorAdaBoosting(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Get the list of the weak classifiers that replace those to be replaced
	// computeSwappedClassifier()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:732
	// ("cv::TrackerStateEstimatorAdaBoosting::computeSwappedClassifier", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn compute_swapped_classifier(&mut self) -> Result<core::Vector<i32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_computeSwappedClassifier(self.as_raw_mut_TrackerStateEstimatorAdaBoosting(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// TrackerStateEstimatorAdaBoosting based on ADA-Boosting
// TrackerStateEstimatorAdaBoosting /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:646
pub struct TrackerStateEstimatorAdaBoosting {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerStateEstimatorAdaBoosting }

impl Drop for TrackerStateEstimatorAdaBoosting {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_delete(self.as_raw_mut_TrackerStateEstimatorAdaBoosting()) };
	}
}

unsafe impl Send for TrackerStateEstimatorAdaBoosting {}

impl crate::tracking::TrackerStateEstimatorTraitConst for TrackerStateEstimatorAdaBoosting {
	#[inline] fn as_raw_TrackerStateEstimator(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerStateEstimatorTrait for TrackerStateEstimatorAdaBoosting {
	#[inline] fn as_raw_mut_TrackerStateEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerStateEstimatorAdaBoosting, crate::tracking::TrackerStateEstimatorTraitConst, as_raw_TrackerStateEstimator, crate::tracking::TrackerStateEstimatorTrait, as_raw_mut_TrackerStateEstimator }

impl crate::tracking::TrackerStateEstimatorAdaBoostingTraitConst for TrackerStateEstimatorAdaBoosting {
	#[inline] fn as_raw_TrackerStateEstimatorAdaBoosting(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerStateEstimatorAdaBoostingTrait for TrackerStateEstimatorAdaBoosting {
	#[inline] fn as_raw_mut_TrackerStateEstimatorAdaBoosting(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerStateEstimatorAdaBoosting, crate::tracking::TrackerStateEstimatorAdaBoostingTraitConst, as_raw_TrackerStateEstimatorAdaBoosting, crate::tracking::TrackerStateEstimatorAdaBoostingTrait, as_raw_mut_TrackerStateEstimatorAdaBoosting }

impl TrackerStateEstimatorAdaBoosting {
	/// Constructor
	/// ## Parameters
	/// * numClassifer: Number of base classifiers
	/// * initIterations: Number of iterations in the initialization
	/// * nFeatures: Number of features/weak classifiers
	/// * patchSize: tracking rect
	/// * ROI: initial ROI
	// TrackerStateEstimatorAdaBoosting(int, int, int, Size, const Rect &)(Primitive, Primitive, Primitive, SimpleClass, SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:701
	// ("cv::TrackerStateEstimatorAdaBoosting::TrackerStateEstimatorAdaBoosting", vec![(pred!(mut, ["numClassifer", "initIterations", "nFeatures", "patchSize", "ROI"], ["int", "int", "int", "cv::Size", "const cv::Rect*"]), _)]),
	#[inline]
	pub fn new(num_classifer: i32, init_iterations: i32, n_features: i32, patch_size: core::Size, roi: core::Rect) -> Result<crate::tracking::TrackerStateEstimatorAdaBoosting> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_TrackerStateEstimatorAdaBoosting_int_int_int_Size_const_RectR(num_classifer, init_iterations, n_features, &patch_size, &roi, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerStateEstimatorAdaBoosting::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TrackerStateEstimatorAdaBoosting, crate::tracking::TrackerStateEstimator, cv_TrackerStateEstimatorAdaBoosting_to_TrackerStateEstimator }

impl std::fmt::Debug for TrackerStateEstimatorAdaBoosting {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerStateEstimatorAdaBoosting")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState]
// TrackerAdaBoostingTargetState /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:651
pub trait TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetStateTraitConst: crate::tracking::TrackerTargetStateTraitConst {
	fn as_raw_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(&self) -> *const c_void;

	/// Get the features extracted
	// getTargetResponses()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:683
	// ("cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::getTargetResponses", vec![(pred!(const, [], []), _)]),
	#[inline]
	#[cfg(not(target_os = "windows"))]
	fn get_target_responses(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_getTargetResponses_const(self.as_raw_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Get the label. Return true for target foreground, false for background
	// isTargetFg()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:686
	// ("cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::isTargetFg", vec![(pred!(const, [], []), _)]),
	#[inline]
	#[cfg(not(target_os = "windows"))]
	fn is_target_fg(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_isTargetFg_const(self.as_raw_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState]
pub trait TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetStateTrait: crate::tracking::TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetStateTraitConst + crate::tracking::TrackerTargetStateTrait {
	fn as_raw_mut_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(&mut self) -> *mut c_void;

	/// Set the features extracted from TrackerFeatureSet
	/// ## Parameters
	/// * responses: The features extracted
	// setTargetResponses(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:676
	// ("cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::setTargetResponses", vec![(pred!(mut, ["responses"], ["const cv::Mat*"]), _)]),
	#[inline]
	#[cfg(not(target_os = "windows"))]
	fn set_target_responses(&mut self, responses: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_setTargetResponses_const_MatR(self.as_raw_mut_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(), responses.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set label: true for target foreground, false for background
	/// ## Parameters
	/// * foreground: Label for background/foreground
	// setTargetFg(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:680
	// ("cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::setTargetFg", vec![(pred!(mut, ["foreground"], ["bool"]), _)]),
	#[inline]
	#[cfg(not(target_os = "windows"))]
	fn set_target_fg(&mut self, foreground: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_setTargetFg_bool(self.as_raw_mut_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(), foreground, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Implementation of the target state for TrackerAdaBoostingTargetState
// TrackerAdaBoostingTargetState /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:651
pub struct TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState }

impl Drop for TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_delete(self.as_raw_mut_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState()) };
	}
}

unsafe impl Send for TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {}

impl crate::tracking::TrackerTargetStateTraitConst for TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {
	#[inline] fn as_raw_TrackerTargetState(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerTargetStateTrait for TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {
	#[inline] fn as_raw_mut_TrackerTargetState(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState, crate::tracking::TrackerTargetStateTraitConst, as_raw_TrackerTargetState, crate::tracking::TrackerTargetStateTrait, as_raw_mut_TrackerTargetState }

impl crate::tracking::TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetStateTraitConst for TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {
	#[inline] fn as_raw_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetStateTrait for TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {
	#[inline] fn as_raw_mut_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState, crate::tracking::TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetStateTraitConst, as_raw_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState, crate::tracking::TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetStateTrait, as_raw_mut_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState }

impl TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {
	/// \brief Constructor
	/// \param position Top left corner of the bounding box
	/// \param width Width of the bounding box
	/// \param height Height of the bounding box
	/// \param foreground label for target or background
	/// \param responses list of features
	// TrackerAdaBoostingTargetState(const Point2f &, int, int, bool, const Mat &)(SimpleClass, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:663
	// ("cv::TrackerStateEstimatorAdaBoosting::TrackerAdaBoostingTargetState::TrackerAdaBoostingTargetState", vec![(pred!(mut, ["position", "width", "height", "foreground", "responses"], ["const cv::Point2f*", "int", "int", "bool", "const cv::Mat*"]), _)]),
	#[inline]
	#[cfg(not(target_os = "windows"))]
	pub fn new(position: core::Point2f, width: i32, height: i32, foreground: bool, responses: &impl core::MatTraitConst) -> Result<crate::tracking::TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_TrackerAdaBoostingTargetState_const_Point2fR_int_int_bool_const_MatR(&position, width, height, foreground, responses.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState, crate::tracking::TrackerTargetState, cv_TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState_to_TrackerTargetState }

impl std::fmt::Debug for TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerStateEstimatorAdaBoosting_TrackerAdaBoostingTargetState")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerStateEstimatorMILBoosting]
// TrackerStateEstimatorMILBoosting /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:569
pub trait TrackerStateEstimatorMILBoostingTraitConst: crate::tracking::TrackerStateEstimatorTraitConst {
	fn as_raw_TrackerStateEstimatorMILBoosting(&self) -> *const c_void;

}

/// Mutable methods for [crate::tracking::TrackerStateEstimatorMILBoosting]
pub trait TrackerStateEstimatorMILBoostingTrait: crate::tracking::TrackerStateEstimatorMILBoostingTraitConst + crate::tracking::TrackerStateEstimatorTrait {
	fn as_raw_mut_TrackerStateEstimatorMILBoosting(&mut self) -> *mut c_void;

	/// Set the current confidenceMap
	/// ## Parameters
	/// * confidenceMap: The current :cConfidenceMap
	// setCurrentConfidenceMap(ConfidenceMap &)(CppPassByVoidPtr) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:627
	// ("cv::TrackerStateEstimatorMILBoosting::setCurrentConfidenceMap", vec![(pred!(mut, ["confidenceMap"], ["cv::ConfidenceMap*"]), _)]),
	#[inline]
	fn set_current_confidence_map(&mut self, confidence_map: &mut crate::tracking::ConfidenceMap) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimatorMILBoosting_setCurrentConfidenceMap_ConfidenceMapR(self.as_raw_mut_TrackerStateEstimatorMILBoosting(), confidence_map.as_raw_mut_VectorOfTupleOfPtrOfTrackerTargetState_f32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// TrackerStateEstimator based on Boosting
// TrackerStateEstimatorMILBoosting /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:569
pub struct TrackerStateEstimatorMILBoosting {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerStateEstimatorMILBoosting }

impl Drop for TrackerStateEstimatorMILBoosting {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerStateEstimatorMILBoosting_delete(self.as_raw_mut_TrackerStateEstimatorMILBoosting()) };
	}
}

unsafe impl Send for TrackerStateEstimatorMILBoosting {}

impl crate::tracking::TrackerStateEstimatorTraitConst for TrackerStateEstimatorMILBoosting {
	#[inline] fn as_raw_TrackerStateEstimator(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerStateEstimatorTrait for TrackerStateEstimatorMILBoosting {
	#[inline] fn as_raw_mut_TrackerStateEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerStateEstimatorMILBoosting, crate::tracking::TrackerStateEstimatorTraitConst, as_raw_TrackerStateEstimator, crate::tracking::TrackerStateEstimatorTrait, as_raw_mut_TrackerStateEstimator }

impl crate::tracking::TrackerStateEstimatorMILBoostingTraitConst for TrackerStateEstimatorMILBoosting {
	#[inline] fn as_raw_TrackerStateEstimatorMILBoosting(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerStateEstimatorMILBoostingTrait for TrackerStateEstimatorMILBoosting {
	#[inline] fn as_raw_mut_TrackerStateEstimatorMILBoosting(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerStateEstimatorMILBoosting, crate::tracking::TrackerStateEstimatorMILBoostingTraitConst, as_raw_TrackerStateEstimatorMILBoosting, crate::tracking::TrackerStateEstimatorMILBoostingTrait, as_raw_mut_TrackerStateEstimatorMILBoosting }

impl TrackerStateEstimatorMILBoosting {
	/// Constructor
	/// ## Parameters
	/// * nFeatures: Number of features for each sample
	///
	/// ## C++ default parameters
	/// * n_features: 250
	// TrackerStateEstimatorMILBoosting(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:621
	// ("cv::TrackerStateEstimatorMILBoosting::TrackerStateEstimatorMILBoosting", vec![(pred!(mut, ["nFeatures"], ["int"]), _)]),
	#[inline]
	pub fn new(n_features: i32) -> Result<crate::tracking::TrackerStateEstimatorMILBoosting> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimatorMILBoosting_TrackerStateEstimatorMILBoosting_int(n_features, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerStateEstimatorMILBoosting::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// Constructor
	/// ## Parameters
	/// * nFeatures: Number of features for each sample
	///
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * n_features: 250
	// cv::TrackerStateEstimatorMILBoosting::TrackerStateEstimatorMILBoosting() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:621
	// ("cv::TrackerStateEstimatorMILBoosting::TrackerStateEstimatorMILBoosting", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn new_def() -> Result<crate::tracking::TrackerStateEstimatorMILBoosting> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimatorMILBoosting_TrackerStateEstimatorMILBoosting(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerStateEstimatorMILBoosting::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TrackerStateEstimatorMILBoosting, crate::tracking::TrackerStateEstimator, cv_TrackerStateEstimatorMILBoosting_to_TrackerStateEstimator }

impl std::fmt::Debug for TrackerStateEstimatorMILBoosting {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerStateEstimatorMILBoosting")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerStateEstimatorMILBoosting_TrackerMILTargetState]
// TrackerMILTargetState /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:576
pub trait TrackerStateEstimatorMILBoosting_TrackerMILTargetStateTraitConst: crate::tracking::TrackerTargetStateTraitConst {
	fn as_raw_TrackerStateEstimatorMILBoosting_TrackerMILTargetState(&self) -> *const c_void;

	/// Get the label. Return true for target foreground, false for background
	// isTargetFg()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:608
	// ("cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::isTargetFg", vec![(pred!(const, [], []), _)]),
	#[inline]
	#[cfg(not(target_os = "windows"))]
	fn is_target_fg(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_isTargetFg_const(self.as_raw_TrackerStateEstimatorMILBoosting_TrackerMILTargetState(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Get the features extracted
	// getFeatures()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:611
	// ("cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::getFeatures", vec![(pred!(const, [], []), _)]),
	#[inline]
	#[cfg(not(target_os = "windows"))]
	fn get_features(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_getFeatures_const(self.as_raw_TrackerStateEstimatorMILBoosting_TrackerMILTargetState(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::TrackerStateEstimatorMILBoosting_TrackerMILTargetState]
pub trait TrackerStateEstimatorMILBoosting_TrackerMILTargetStateTrait: crate::tracking::TrackerStateEstimatorMILBoosting_TrackerMILTargetStateTraitConst + crate::tracking::TrackerTargetStateTrait {
	fn as_raw_mut_TrackerStateEstimatorMILBoosting_TrackerMILTargetState(&mut self) -> *mut c_void;

	/// Set label: true for target foreground, false for background
	/// ## Parameters
	/// * foreground: Label for background/foreground
	// setTargetFg(bool)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:601
	// ("cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::setTargetFg", vec![(pred!(mut, ["foreground"], ["bool"]), _)]),
	#[inline]
	#[cfg(not(target_os = "windows"))]
	fn set_target_fg(&mut self, foreground: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_setTargetFg_bool(self.as_raw_mut_TrackerStateEstimatorMILBoosting_TrackerMILTargetState(), foreground, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Set the features extracted from TrackerFeatureSet
	/// ## Parameters
	/// * features: The features extracted
	// setFeatures(const Mat &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:605
	// ("cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::setFeatures", vec![(pred!(mut, ["features"], ["const cv::Mat*"]), _)]),
	#[inline]
	#[cfg(not(target_os = "windows"))]
	fn set_features(&mut self, features: &impl core::MatTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_setFeatures_const_MatR(self.as_raw_mut_TrackerStateEstimatorMILBoosting_TrackerMILTargetState(), features.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Implementation of the target state for TrackerStateEstimatorMILBoosting
// TrackerMILTargetState /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:576
pub struct TrackerStateEstimatorMILBoosting_TrackerMILTargetState {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerStateEstimatorMILBoosting_TrackerMILTargetState }

impl Drop for TrackerStateEstimatorMILBoosting_TrackerMILTargetState {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_delete(self.as_raw_mut_TrackerStateEstimatorMILBoosting_TrackerMILTargetState()) };
	}
}

unsafe impl Send for TrackerStateEstimatorMILBoosting_TrackerMILTargetState {}

impl crate::tracking::TrackerTargetStateTraitConst for TrackerStateEstimatorMILBoosting_TrackerMILTargetState {
	#[inline] fn as_raw_TrackerTargetState(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerTargetStateTrait for TrackerStateEstimatorMILBoosting_TrackerMILTargetState {
	#[inline] fn as_raw_mut_TrackerTargetState(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerStateEstimatorMILBoosting_TrackerMILTargetState, crate::tracking::TrackerTargetStateTraitConst, as_raw_TrackerTargetState, crate::tracking::TrackerTargetStateTrait, as_raw_mut_TrackerTargetState }

impl crate::tracking::TrackerStateEstimatorMILBoosting_TrackerMILTargetStateTraitConst for TrackerStateEstimatorMILBoosting_TrackerMILTargetState {
	#[inline] fn as_raw_TrackerStateEstimatorMILBoosting_TrackerMILTargetState(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerStateEstimatorMILBoosting_TrackerMILTargetStateTrait for TrackerStateEstimatorMILBoosting_TrackerMILTargetState {
	#[inline] fn as_raw_mut_TrackerStateEstimatorMILBoosting_TrackerMILTargetState(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerStateEstimatorMILBoosting_TrackerMILTargetState, crate::tracking::TrackerStateEstimatorMILBoosting_TrackerMILTargetStateTraitConst, as_raw_TrackerStateEstimatorMILBoosting_TrackerMILTargetState, crate::tracking::TrackerStateEstimatorMILBoosting_TrackerMILTargetStateTrait, as_raw_mut_TrackerStateEstimatorMILBoosting_TrackerMILTargetState }

impl TrackerStateEstimatorMILBoosting_TrackerMILTargetState {
	/// \brief Constructor
	/// \param position Top left corner of the bounding box
	/// \param width Width of the bounding box
	/// \param height Height of the bounding box
	/// \param foreground label for target or background
	/// \param features features extracted
	// TrackerMILTargetState(const Point2f &, int, int, bool, const Mat &)(SimpleClass, Primitive, Primitive, Primitive, TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:588
	// ("cv::TrackerStateEstimatorMILBoosting::TrackerMILTargetState::TrackerMILTargetState", vec![(pred!(mut, ["position", "width", "height", "foreground", "features"], ["const cv::Point2f*", "int", "int", "bool", "const cv::Mat*"]), _)]),
	#[inline]
	#[cfg(not(target_os = "windows"))]
	pub fn new(position: core::Point2f, width: i32, height: i32, foreground: bool, features: &impl core::MatTraitConst) -> Result<crate::tracking::TrackerStateEstimatorMILBoosting_TrackerMILTargetState> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_TrackerMILTargetState_const_Point2fR_int_int_bool_const_MatR(&position, width, height, foreground, features.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerStateEstimatorMILBoosting_TrackerMILTargetState::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TrackerStateEstimatorMILBoosting_TrackerMILTargetState, crate::tracking::TrackerTargetState, cv_TrackerStateEstimatorMILBoosting_TrackerMILTargetState_to_TrackerTargetState }

impl std::fmt::Debug for TrackerStateEstimatorMILBoosting_TrackerMILTargetState {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerStateEstimatorMILBoosting_TrackerMILTargetState")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerStateEstimatorSVM]
// TrackerStateEstimatorSVM /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:756
pub trait TrackerStateEstimatorSVMTraitConst: crate::tracking::TrackerStateEstimatorTraitConst {
	fn as_raw_TrackerStateEstimatorSVM(&self) -> *const c_void;

}

/// Mutable methods for [crate::tracking::TrackerStateEstimatorSVM]
pub trait TrackerStateEstimatorSVMTrait: crate::tracking::TrackerStateEstimatorSVMTraitConst + crate::tracking::TrackerStateEstimatorTrait {
	fn as_raw_mut_TrackerStateEstimatorSVM(&mut self) -> *mut c_void;

}

/// \brief TrackerStateEstimator based on SVM
// TrackerStateEstimatorSVM /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:756
pub struct TrackerStateEstimatorSVM {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerStateEstimatorSVM }

impl Drop for TrackerStateEstimatorSVM {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerStateEstimatorSVM_delete(self.as_raw_mut_TrackerStateEstimatorSVM()) };
	}
}

unsafe impl Send for TrackerStateEstimatorSVM {}

impl crate::tracking::TrackerStateEstimatorTraitConst for TrackerStateEstimatorSVM {
	#[inline] fn as_raw_TrackerStateEstimator(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerStateEstimatorTrait for TrackerStateEstimatorSVM {
	#[inline] fn as_raw_mut_TrackerStateEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerStateEstimatorSVM, crate::tracking::TrackerStateEstimatorTraitConst, as_raw_TrackerStateEstimator, crate::tracking::TrackerStateEstimatorTrait, as_raw_mut_TrackerStateEstimator }

impl crate::tracking::TrackerStateEstimatorSVMTraitConst for TrackerStateEstimatorSVM {
	#[inline] fn as_raw_TrackerStateEstimatorSVM(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerStateEstimatorSVMTrait for TrackerStateEstimatorSVM {
	#[inline] fn as_raw_mut_TrackerStateEstimatorSVM(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerStateEstimatorSVM, crate::tracking::TrackerStateEstimatorSVMTraitConst, as_raw_TrackerStateEstimatorSVM, crate::tracking::TrackerStateEstimatorSVMTrait, as_raw_mut_TrackerStateEstimatorSVM }

impl TrackerStateEstimatorSVM {
	// TrackerStateEstimatorSVM()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:759
	// ("cv::TrackerStateEstimatorSVM::TrackerStateEstimatorSVM", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::tracking::TrackerStateEstimatorSVM> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerStateEstimatorSVM_TrackerStateEstimatorSVM(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerStateEstimatorSVM::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TrackerStateEstimatorSVM, crate::tracking::TrackerStateEstimator, cv_TrackerStateEstimatorSVM_to_TrackerStateEstimator }

impl std::fmt::Debug for TrackerStateEstimatorSVM {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerStateEstimatorSVM")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerTLD]
// TrackerTLD /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1184
pub trait TrackerTLDTraitConst: crate::tracking::TrackerTraitConst {
	fn as_raw_TrackerTLD(&self) -> *const c_void;

}

/// Mutable methods for [crate::tracking::TrackerTLD]
pub trait TrackerTLDTrait: crate::tracking::TrackerTLDTraitConst + crate::tracking::TrackerTrait {
	fn as_raw_mut_TrackerTLD(&mut self) -> *mut c_void;

}

/// the TLD (Tracking, learning and detection) tracker
///
/// TLD is a novel tracking framework that explicitly decomposes the long-term tracking task into
/// tracking, learning and detection.
///
/// The tracker follows the object from frame to frame. The detector localizes all appearances that
/// have been observed so far and corrects the tracker if necessary. The learning estimates detector's
/// errors and updates it to avoid these errors in the future. The implementation is based on [TLD](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_TLD) .
///
/// The Median Flow algorithm (see cv::TrackerMedianFlow) was chosen as a tracking component in this
/// implementation, following authors. The tracker is supposed to be able to handle rapid motions, partial
/// occlusions, object absence etc.
// TrackerTLD /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1184
pub struct TrackerTLD {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerTLD }

impl Drop for TrackerTLD {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerTLD_delete(self.as_raw_mut_TrackerTLD()) };
	}
}

unsafe impl Send for TrackerTLD {}

impl core::AlgorithmTraitConst for TrackerTLD {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for TrackerTLD {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerTLD, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::tracking::TrackerTraitConst for TrackerTLD {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerTrait for TrackerTLD {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerTLD, crate::tracking::TrackerTraitConst, as_raw_Tracker, crate::tracking::TrackerTrait, as_raw_mut_Tracker }

impl crate::tracking::TrackerTLDTraitConst for TrackerTLD {
	#[inline] fn as_raw_TrackerTLD(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerTLDTrait for TrackerTLD {
	#[inline] fn as_raw_mut_TrackerTLD(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerTLD, crate::tracking::TrackerTLDTraitConst, as_raw_TrackerTLD, crate::tracking::TrackerTLDTrait, as_raw_mut_TrackerTLD }

impl TrackerTLD {
	/// Constructor
	/// ## Parameters
	/// * parameters: TLD parameters TrackerTLD::Params
	// create(const TrackerTLD::Params &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1197
	// ("cv::TrackerTLD::create", vec![(pred!(mut, ["parameters"], ["const cv::TrackerTLD::Params*"]), _)]),
	#[inline]
	pub fn create(parameters: &impl crate::tracking::TrackerTLD_ParamsTraitConst) -> Result<core::Ptr<crate::tracking::TrackerTLD>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerTLD_create_const_ParamsR(parameters.as_raw_TrackerTLD_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerTLD>::opencv_from_extern(ret) };
		Ok(ret)
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1199
	// ("cv::TrackerTLD::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create_1() -> Result<core::Ptr<crate::tracking::TrackerTLD>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerTLD_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::tracking::TrackerTLD>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { TrackerTLD, core::Algorithm, cv_TrackerTLD_to_Algorithm }

boxed_cast_base! { TrackerTLD, crate::tracking::Tracker, cv_TrackerTLD_to_Tracker }

impl std::fmt::Debug for TrackerTLD {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerTLD")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerTLD_Params]
// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1187
pub trait TrackerTLD_ParamsTraitConst {
	fn as_raw_TrackerTLD_Params(&self) -> *const c_void;

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1191
	// ("cv::TrackerTLD::Params::write", vec![(pred!(const, ["unnamed"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, unnamed: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerTLD_Params_write_const_FileStorageR(self.as_raw_TrackerTLD_Params(), unnamed.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::TrackerTLD_Params]
pub trait TrackerTLD_ParamsTrait: crate::tracking::TrackerTLD_ParamsTraitConst {
	fn as_raw_mut_TrackerTLD_Params(&mut self) -> *mut c_void;

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1190
	// ("cv::TrackerTLD::Params::read", vec![(pred!(mut, ["unnamed"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read(&mut self, unnamed: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerTLD_Params_read_const_FileNodeR(self.as_raw_mut_TrackerTLD_Params(), unnamed.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

// Params /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1187
pub struct TrackerTLD_Params {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerTLD_Params }

impl Drop for TrackerTLD_Params {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerTLD_Params_delete(self.as_raw_mut_TrackerTLD_Params()) };
	}
}

unsafe impl Send for TrackerTLD_Params {}

impl crate::tracking::TrackerTLD_ParamsTraitConst for TrackerTLD_Params {
	#[inline] fn as_raw_TrackerTLD_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerTLD_ParamsTrait for TrackerTLD_Params {
	#[inline] fn as_raw_mut_TrackerTLD_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerTLD_Params, crate::tracking::TrackerTLD_ParamsTraitConst, as_raw_TrackerTLD_Params, crate::tracking::TrackerTLD_ParamsTrait, as_raw_mut_TrackerTLD_Params }

impl TrackerTLD_Params {
	// Params()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:1189
	// ("cv::TrackerTLD::Params::Params", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::tracking::TrackerTLD_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerTLD_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::tracking::TrackerTLD_Params::opencv_from_extern(ret) };
		Ok(ret)
	}

}

impl std::fmt::Debug for TrackerTLD_Params {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerTLD_Params")
			.finish()
	}
}

/// Constant methods for [crate::tracking::TrackerTargetState]
// TrackerTargetState /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:331
pub trait TrackerTargetStateTraitConst {
	fn as_raw_TrackerTargetState(&self) -> *const c_void;

	/// \brief Get the position
	/// \return The position
	// getTargetPosition()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:342
	// ("cv::TrackerTargetState::getTargetPosition", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_target_position(&self) -> Result<core::Point2f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerTargetState_getTargetPosition_const(self.as_raw_TrackerTargetState(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// \brief Get the width of the target
	/// \return The width of the target
	// getTargetWidth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:353
	// ("cv::TrackerTargetState::getTargetWidth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_target_width(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerTargetState_getTargetWidth_const(self.as_raw_TrackerTargetState(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// \brief Get the height of the target
	/// \return The height of the target
	// getTargetHeight()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:364
	// ("cv::TrackerTargetState::getTargetHeight", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_target_height(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerTargetState_getTargetHeight_const(self.as_raw_TrackerTargetState(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::tracking::TrackerTargetState]
pub trait TrackerTargetStateTrait: crate::tracking::TrackerTargetStateTraitConst {
	fn as_raw_mut_TrackerTargetState(&mut self) -> *mut c_void;

	/// \brief Set the position
	/// \param position The position
	// setTargetPosition(const Point2f &)(SimpleClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:348
	// ("cv::TrackerTargetState::setTargetPosition", vec![(pred!(mut, ["position"], ["const cv::Point2f*"]), _)]),
	#[inline]
	fn set_target_position(&mut self, position: core::Point2f) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerTargetState_setTargetPosition_const_Point2fR(self.as_raw_mut_TrackerTargetState(), &position, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// \brief Set the width of the target
	/// \param width The width of the target
	// setTargetWidth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:359
	// ("cv::TrackerTargetState::setTargetWidth", vec![(pred!(mut, ["width"], ["int"]), _)]),
	#[inline]
	fn set_target_width(&mut self, width: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerTargetState_setTargetWidth_int(self.as_raw_mut_TrackerTargetState(), width, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// \brief Set the height of the target
	/// \param height The height of the target
	// setTargetHeight(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:370
	// ("cv::TrackerTargetState::setTargetHeight", vec![(pred!(mut, ["height"], ["int"]), _)]),
	#[inline]
	fn set_target_height(&mut self, height: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_TrackerTargetState_setTargetHeight_int(self.as_raw_mut_TrackerTargetState(), height, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Abstract base class for TrackerTargetState that represents a possible state of the target.
///
/// See [AAM](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_AAM) ![inline formula](https://latex.codecogs.com/png.latex?%5Chat%7Bx%7D%5E%7Bi%7D%5F%7Bk%7D) all the states candidates.
///
/// Inherits this class with your Target state, In own implementation you can add scale variation,
/// width, height, orientation, etc.
// TrackerTargetState /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/tracking/tracker.hpp:331
pub struct TrackerTargetState {
	ptr: *mut c_void,
}

opencv_type_boxed! { TrackerTargetState }

impl Drop for TrackerTargetState {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_TrackerTargetState_delete(self.as_raw_mut_TrackerTargetState()) };
	}
}

unsafe impl Send for TrackerTargetState {}

impl crate::tracking::TrackerTargetStateTraitConst for TrackerTargetState {
	#[inline] fn as_raw_TrackerTargetState(&self) -> *const c_void { self.as_raw() }
}

impl crate::tracking::TrackerTargetStateTrait for TrackerTargetState {
	#[inline] fn as_raw_mut_TrackerTargetState(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { TrackerTargetState, crate::tracking::TrackerTargetStateTraitConst, as_raw_TrackerTargetState, crate::tracking::TrackerTargetStateTrait, as_raw_mut_TrackerTargetState }

impl TrackerTargetState {
	/// Creates a default instance of the class by calling the default constructor
	#[inline]
	fn default() -> Self {
		unsafe { Self::from_raw(sys::cv_TrackerTargetState_defaultNew_const()) }
	}

}

impl std::fmt::Debug for TrackerTargetState {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TrackerTargetState")
			.finish()
	}
}

impl Default for TrackerTargetState {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}
