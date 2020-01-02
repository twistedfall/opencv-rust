//! # Improved Background-Foreground Segmentation Methods
use crate::{mod_prelude::*, core, sys, types};
use crate::core::{_InputArrayTrait, _OutputArrayTrait};

pub const LSBP_CAMERA_MOTION_COMPENSATION_LK: i32 = 0+1;
pub const LSBP_CAMERA_MOTION_COMPENSATION_NONE: i32 = 0;

/// Creates a CNT Background Subtractor
///
/// ## Parameters
/// * minPixelStability: number of frames with same pixel color to consider stable
/// * useHistory: determines if we're giving a pixel credit for being stable for a long time
/// * maxPixelStability: maximum allowed credit for a pixel in history
/// * isParallel: determines if we're parallelizing the algorithm
///
/// ## C++ default parameters
/// * min_pixel_stability: 15
/// * use_history: true
/// * max_pixel_stability: 15*60
/// * is_parallel: true
pub fn create_background_subtractor_cnt(min_pixel_stability: i32, use_history: bool, max_pixel_stability: i32, is_parallel: bool) -> Result<types::PtrOfBackgroundSubtractorCNT> {
    unsafe { sys::cv_bgsegm_createBackgroundSubtractorCNT_int_bool_int_bool(min_pixel_stability, use_history, max_pixel_stability, is_parallel) }.into_result().map(|ptr| types::PtrOfBackgroundSubtractorCNT { ptr })
}

/// Creates a GMG Background Subtractor
///
/// ## Parameters
/// * initializationFrames: number of frames used to initialize the background models.
/// * decisionThreshold: Threshold value, above which it is marked foreground, else background.
///
/// ## C++ default parameters
/// * initialization_frames: 120
/// * decision_threshold: 0.8
pub fn create_background_subtractor_gmg(initialization_frames: i32, decision_threshold: f64) -> Result<types::PtrOfBackgroundSubtractorGMG> {
    unsafe { sys::cv_bgsegm_createBackgroundSubtractorGMG_int_double(initialization_frames, decision_threshold) }.into_result().map(|ptr| types::PtrOfBackgroundSubtractorGMG { ptr })
}

/// Creates an instance of BackgroundSubtractorGSOC algorithm.
///
/// Implementation of the different yet better algorithm which is called GSOC, as it was implemented during GSOC and was not originated from any paper.
///
/// ## Parameters
/// * mc: Whether to use camera motion compensation.
/// * nSamples: Number of samples to maintain at each point of the frame.
/// * replaceRate: Probability of replacing the old sample - how fast the model will update itself.
/// * propagationRate: Probability of propagating to neighbors.
/// * hitsThreshold: How many positives the sample must get before it will be considered as a possible replacement.
/// * alpha: Scale coefficient for threshold.
/// * beta: Bias coefficient for threshold.
/// * blinkingSupressionDecay: Blinking supression decay factor.
/// * blinkingSupressionMultiplier: Blinking supression multiplier.
/// * noiseRemovalThresholdFacBG: Strength of the noise removal for background points.
/// * noiseRemovalThresholdFacFG: Strength of the noise removal for foreground points.
///
/// ## C++ default parameters
/// * mc: LSBP_CAMERA_MOTION_COMPENSATION_NONE
/// * n_samples: 20
/// * replace_rate: 0.003f
/// * propagation_rate: 0.01f
/// * hits_threshold: 32
/// * alpha: 0.01f
/// * beta: 0.0022f
/// * blinking_supression_decay: 0.1f
/// * blinking_supression_multiplier: 0.1f
/// * noise_removal_threshold_fac_bg: 0.0004f
/// * noise_removal_threshold_fac_fg: 0.0008f
pub fn create_background_subtractor_gsoc(mc: i32, n_samples: i32, replace_rate: f32, propagation_rate: f32, hits_threshold: i32, alpha: f32, beta: f32, blinking_supression_decay: f32, blinking_supression_multiplier: f32, noise_removal_threshold_fac_bg: f32, noise_removal_threshold_fac_fg: f32) -> Result<types::PtrOfBackgroundSubtractorGSOC> {
    unsafe { sys::cv_bgsegm_createBackgroundSubtractorGSOC_int_int_float_float_int_float_float_float_float_float_float(mc, n_samples, replace_rate, propagation_rate, hits_threshold, alpha, beta, blinking_supression_decay, blinking_supression_multiplier, noise_removal_threshold_fac_bg, noise_removal_threshold_fac_fg) }.into_result().map(|ptr| types::PtrOfBackgroundSubtractorGSOC { ptr })
}

/// Creates an instance of BackgroundSubtractorLSBP algorithm.
///
/// Background Subtraction using Local SVD Binary Pattern. More details about the algorithm can be found at [LGuo2016](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_LGuo2016)
///
/// ## Parameters
/// * mc: Whether to use camera motion compensation.
/// * nSamples: Number of samples to maintain at each point of the frame.
/// * LSBPRadius: LSBP descriptor radius.
/// * Tlower: Lower bound for T-values. See [LGuo2016](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_LGuo2016) for details.
/// * Tupper: Upper bound for T-values. See [LGuo2016](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_LGuo2016) for details.
/// * Tinc: Increase step for T-values. See [LGuo2016](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_LGuo2016) for details.
/// * Tdec: Decrease step for T-values. See [LGuo2016](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_LGuo2016) for details.
/// * Rscale: Scale coefficient for threshold values.
/// * Rincdec: Increase/Decrease step for threshold values.
/// * noiseRemovalThresholdFacBG: Strength of the noise removal for background points.
/// * noiseRemovalThresholdFacFG: Strength of the noise removal for foreground points.
/// * LSBPthreshold: Threshold for LSBP binary string.
/// * minCount: Minimal number of matches for sample to be considered as foreground.
///
/// ## C++ default parameters
/// * mc: LSBP_CAMERA_MOTION_COMPENSATION_NONE
/// * n_samples: 20
/// * lsbp_radius: 16
/// * tlower: 2.0f
/// * tupper: 32.0f
/// * tinc: 1.0f
/// * tdec: 0.05f
/// * rscale: 10.0f
/// * rincdec: 0.005f
/// * noise_removal_threshold_fac_bg: 0.0004f
/// * noise_removal_threshold_fac_fg: 0.0008f
/// * lsb_pthreshold: 8
/// * min_count: 2
pub fn create_background_subtractor_lsbp(mc: i32, n_samples: i32, lsbp_radius: i32, tlower: f32, tupper: f32, tinc: f32, tdec: f32, rscale: f32, rincdec: f32, noise_removal_threshold_fac_bg: f32, noise_removal_threshold_fac_fg: f32, lsb_pthreshold: i32, min_count: i32) -> Result<types::PtrOfBackgroundSubtractorLSBP> {
    unsafe { sys::cv_bgsegm_createBackgroundSubtractorLSBP_int_int_int_float_float_float_float_float_float_float_float_int_int(mc, n_samples, lsbp_radius, tlower, tupper, tinc, tdec, rscale, rincdec, noise_removal_threshold_fac_bg, noise_removal_threshold_fac_fg, lsb_pthreshold, min_count) }.into_result().map(|ptr| types::PtrOfBackgroundSubtractorLSBP { ptr })
}

/// Creates mixture-of-gaussian background subtractor
///
/// ## Parameters
/// * history: Length of the history.
/// * nmixtures: Number of Gaussian mixtures.
/// * backgroundRatio: Background ratio.
/// * noiseSigma: Noise strength (standard deviation of the brightness or each color channel). 0
/// means some automatic value.
///
/// ## C++ default parameters
/// * history: 200
/// * nmixtures: 5
/// * background_ratio: 0.7
/// * noise_sigma: 0
pub fn create_background_subtractor_mog(history: i32, nmixtures: i32, background_ratio: f64, noise_sigma: f64) -> Result<types::PtrOfBackgroundSubtractorMOG> {
    unsafe { sys::cv_bgsegm_createBackgroundSubtractorMOG_int_int_double_double(history, nmixtures, background_ratio, noise_sigma) }.into_result().map(|ptr| types::PtrOfBackgroundSubtractorMOG { ptr })
}

/// Creates an instance of SyntheticSequenceGenerator.
///
/// ## Parameters
/// * background: Background image for object.
/// * object: Object image which will move slowly over the background.
/// * amplitude: Amplitude of wave distortion applied to background.
/// * wavelength: Length of waves in distortion applied to background.
/// * wavespeed: How fast waves will move.
/// * objspeed: How fast object will fly over background.
///
/// ## C++ default parameters
/// * amplitude: 2.0
/// * wavelength: 20.0
/// * wavespeed: 0.2
/// * objspeed: 6.0
pub fn create_synthetic_sequence_generator(background: &dyn core::ToInputArray, object: &dyn core::ToInputArray, amplitude: f64, wavelength: f64, wavespeed: f64, objspeed: f64) -> Result<types::PtrOfSyntheticSequenceGenerator> {
    input_array_arg!(background);
    input_array_arg!(object);
    unsafe { sys::cv_bgsegm_createSyntheticSequenceGenerator__InputArray__InputArray_double_double_double_double(background.as_raw__InputArray(), object.as_raw__InputArray(), amplitude, wavelength, wavespeed, objspeed) }.into_result().map(|ptr| types::PtrOfSyntheticSequenceGenerator { ptr })
}

// Generating impl for trait crate::bgsegm::BackgroundSubtractorCNT
/// Background subtraction based on counting.
///
/// About as fast as MOG2 on a high end system.
/// More than twice faster than MOG2 on cheap hardware (benchmarked on Raspberry Pi3).
///
/// %Algorithm by Sagi Zeevi ( https://github.com/sagi-z/BackgroundSubtractorCNT )
pub trait BackgroundSubtractorCNT {
    fn as_raw_BackgroundSubtractorCNT(&self) -> *mut c_void;
    ///
    /// ## C++ default parameters
    /// * learning_rate: -1
    fn apply(&mut self, image: &dyn core::ToInputArray, fgmask: &mut dyn core::ToOutputArray, learning_rate: f64) -> Result<()> {
        input_array_arg!(image);
        output_array_arg!(fgmask);
        unsafe { sys::cv_bgsegm_BackgroundSubtractorCNT_apply__InputArray__OutputArray_double(self.as_raw_BackgroundSubtractorCNT(), image.as_raw__InputArray(), fgmask.as_raw__OutputArray(), learning_rate) }.into_result()
    }
    
    fn get_background_image(&self, background_image: &mut dyn core::ToOutputArray) -> Result<()> {
        output_array_arg!(background_image);
        unsafe { sys::cv_bgsegm_BackgroundSubtractorCNT_getBackgroundImage_const__OutputArray(self.as_raw_BackgroundSubtractorCNT(), background_image.as_raw__OutputArray()) }.into_result()
    }
    
    /// Returns number of frames with same pixel color to consider stable.
    fn get_min_pixel_stability(&self) -> Result<i32> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorCNT_getMinPixelStability_const(self.as_raw_BackgroundSubtractorCNT()) }.into_result()
    }
    
    /// Sets the number of frames with same pixel color to consider stable.
    fn set_min_pixel_stability(&mut self, value: i32) -> Result<()> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorCNT_setMinPixelStability_int(self.as_raw_BackgroundSubtractorCNT(), value) }.into_result()
    }
    
    /// Returns maximum allowed credit for a pixel in history.
    fn get_max_pixel_stability(&self) -> Result<i32> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorCNT_getMaxPixelStability_const(self.as_raw_BackgroundSubtractorCNT()) }.into_result()
    }
    
    /// Sets the maximum allowed credit for a pixel in history.
    fn set_max_pixel_stability(&mut self, value: i32) -> Result<()> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorCNT_setMaxPixelStability_int(self.as_raw_BackgroundSubtractorCNT(), value) }.into_result()
    }
    
    /// Returns if we're giving a pixel credit for being stable for a long time.
    fn get_use_history(&self) -> Result<bool> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorCNT_getUseHistory_const(self.as_raw_BackgroundSubtractorCNT()) }.into_result()
    }
    
    /// Sets if we're giving a pixel credit for being stable for a long time.
    fn set_use_history(&mut self, value: bool) -> Result<()> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorCNT_setUseHistory_bool(self.as_raw_BackgroundSubtractorCNT(), value) }.into_result()
    }
    
    /// Returns if we're parallelizing the algorithm.
    fn get_is_parallel(&self) -> Result<bool> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorCNT_getIsParallel_const(self.as_raw_BackgroundSubtractorCNT()) }.into_result()
    }
    
    /// Sets if we're parallelizing the algorithm.
    fn set_is_parallel(&mut self, value: bool) -> Result<()> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorCNT_setIsParallel_bool(self.as_raw_BackgroundSubtractorCNT(), value) }.into_result()
    }
    
}

// Generating impl for trait crate::bgsegm::BackgroundSubtractorGMG
/// Background Subtractor module based on the algorithm given in [Gold2012](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Gold2012) .
///
/// Takes a series of images and returns a sequence of mask (8UC1)
/// images of the same size, where 255 indicates Foreground and 0 represents Background.
/// This class implements an algorithm described in "Visual Tracking of Human Visitors under
/// Variable-Lighting Conditions for a Responsive Audio Art Installation," A. Godbehere,
/// A. Matsukawa, K. Goldberg, American Control Conference, Montreal, June 2012.
pub trait BackgroundSubtractorGMG {
    fn as_raw_BackgroundSubtractorGMG(&self) -> *mut c_void;
    /// Returns total number of distinct colors to maintain in histogram.
    fn get_max_features(&self) -> Result<i32> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorGMG_getMaxFeatures_const(self.as_raw_BackgroundSubtractorGMG()) }.into_result()
    }
    
    /// Sets total number of distinct colors to maintain in histogram.
    fn set_max_features(&mut self, max_features: i32) -> Result<()> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorGMG_setMaxFeatures_int(self.as_raw_BackgroundSubtractorGMG(), max_features) }.into_result()
    }
    
    /// Returns the learning rate of the algorithm.
    ///
    /// It lies between 0.0 and 1.0. It determines how quickly features are "forgotten" from
    /// histograms.
    fn get_default_learning_rate(&self) -> Result<f64> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorGMG_getDefaultLearningRate_const(self.as_raw_BackgroundSubtractorGMG()) }.into_result()
    }
    
    /// Sets the learning rate of the algorithm.
    fn set_default_learning_rate(&mut self, lr: f64) -> Result<()> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorGMG_setDefaultLearningRate_double(self.as_raw_BackgroundSubtractorGMG(), lr) }.into_result()
    }
    
    /// Returns the number of frames used to initialize background model.
    fn get_num_frames(&self) -> Result<i32> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorGMG_getNumFrames_const(self.as_raw_BackgroundSubtractorGMG()) }.into_result()
    }
    
    /// Sets the number of frames used to initialize background model.
    fn set_num_frames(&mut self, nframes: i32) -> Result<()> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorGMG_setNumFrames_int(self.as_raw_BackgroundSubtractorGMG(), nframes) }.into_result()
    }
    
    /// Returns the parameter used for quantization of color-space.
    ///
    /// It is the number of discrete levels in each channel to be used in histograms.
    fn get_quantization_levels(&self) -> Result<i32> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorGMG_getQuantizationLevels_const(self.as_raw_BackgroundSubtractorGMG()) }.into_result()
    }
    
    /// Sets the parameter used for quantization of color-space
    fn set_quantization_levels(&mut self, nlevels: i32) -> Result<()> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorGMG_setQuantizationLevels_int(self.as_raw_BackgroundSubtractorGMG(), nlevels) }.into_result()
    }
    
    /// Returns the prior probability that each individual pixel is a background pixel.
    fn get_background_prior(&self) -> Result<f64> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorGMG_getBackgroundPrior_const(self.as_raw_BackgroundSubtractorGMG()) }.into_result()
    }
    
    /// Sets the prior probability that each individual pixel is a background pixel.
    fn set_background_prior(&mut self, bgprior: f64) -> Result<()> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorGMG_setBackgroundPrior_double(self.as_raw_BackgroundSubtractorGMG(), bgprior) }.into_result()
    }
    
    /// Returns the kernel radius used for morphological operations
    fn get_smoothing_radius(&self) -> Result<i32> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorGMG_getSmoothingRadius_const(self.as_raw_BackgroundSubtractorGMG()) }.into_result()
    }
    
    /// Sets the kernel radius used for morphological operations
    fn set_smoothing_radius(&mut self, radius: i32) -> Result<()> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorGMG_setSmoothingRadius_int(self.as_raw_BackgroundSubtractorGMG(), radius) }.into_result()
    }
    
    /// Returns the value of decision threshold.
    ///
    /// Decision value is the value above which pixel is determined to be FG.
    fn get_decision_threshold(&self) -> Result<f64> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorGMG_getDecisionThreshold_const(self.as_raw_BackgroundSubtractorGMG()) }.into_result()
    }
    
    /// Sets the value of decision threshold.
    fn set_decision_threshold(&mut self, thresh: f64) -> Result<()> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorGMG_setDecisionThreshold_double(self.as_raw_BackgroundSubtractorGMG(), thresh) }.into_result()
    }
    
    /// Returns the status of background model update
    fn get_update_background_model(&self) -> Result<bool> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorGMG_getUpdateBackgroundModel_const(self.as_raw_BackgroundSubtractorGMG()) }.into_result()
    }
    
    /// Sets the status of background model update
    fn set_update_background_model(&mut self, update: bool) -> Result<()> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorGMG_setUpdateBackgroundModel_bool(self.as_raw_BackgroundSubtractorGMG(), update) }.into_result()
    }
    
    /// Returns the minimum value taken on by pixels in image sequence. Usually 0.
    fn get_min_val(&self) -> Result<f64> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorGMG_getMinVal_const(self.as_raw_BackgroundSubtractorGMG()) }.into_result()
    }
    
    /// Sets the minimum value taken on by pixels in image sequence.
    fn set_min_val(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorGMG_setMinVal_double(self.as_raw_BackgroundSubtractorGMG(), val) }.into_result()
    }
    
    /// Returns the maximum value taken on by pixels in image sequence. e.g. 1.0 or 255.
    fn get_max_val(&self) -> Result<f64> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorGMG_getMaxVal_const(self.as_raw_BackgroundSubtractorGMG()) }.into_result()
    }
    
    /// Sets the maximum value taken on by pixels in image sequence.
    fn set_max_val(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorGMG_setMaxVal_double(self.as_raw_BackgroundSubtractorGMG(), val) }.into_result()
    }
    
}

// boxed class cv::bgsegm::BackgroundSubtractorGSOC
/// Implementation of the different yet better algorithm which is called GSOC, as it was implemented during GSOC and was not originated from any paper.
///
/// This algorithm demonstrates better performance on CDNET 2014 dataset compared to other algorithms in OpenCV.
pub struct BackgroundSubtractorGSOC {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for BackgroundSubtractorGSOC {
    fn drop(&mut self) {
        unsafe { sys::cv_BackgroundSubtractorGSOC_delete(self.ptr) };
    }
}

impl BackgroundSubtractorGSOC {
    #[inline(always)] pub fn as_raw_BackgroundSubtractorGSOC(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for BackgroundSubtractorGSOC {}

impl BackgroundSubtractorGSOC {
    ///
    /// ## C++ default parameters
    /// * learning_rate: -1
    pub fn apply(&mut self, image: &dyn core::ToInputArray, fgmask: &mut dyn core::ToOutputArray, learning_rate: f64) -> Result<()> {
        input_array_arg!(image);
        output_array_arg!(fgmask);
        unsafe { sys::cv_bgsegm_BackgroundSubtractorGSOC_apply__InputArray__OutputArray_double(self.as_raw_BackgroundSubtractorGSOC(), image.as_raw__InputArray(), fgmask.as_raw__OutputArray(), learning_rate) }.into_result()
    }
    
    pub fn get_background_image(&self, background_image: &mut dyn core::ToOutputArray) -> Result<()> {
        output_array_arg!(background_image);
        unsafe { sys::cv_bgsegm_BackgroundSubtractorGSOC_getBackgroundImage_const__OutputArray(self.as_raw_BackgroundSubtractorGSOC(), background_image.as_raw__OutputArray()) }.into_result()
    }
    
}

// boxed class cv::bgsegm::BackgroundSubtractorLSBP
/// Background Subtraction using Local SVD Binary Pattern. More details about the algorithm can be found at [LGuo2016](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_LGuo2016)
pub struct BackgroundSubtractorLSBP {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for BackgroundSubtractorLSBP {
    fn drop(&mut self) {
        unsafe { sys::cv_BackgroundSubtractorLSBP_delete(self.ptr) };
    }
}

impl BackgroundSubtractorLSBP {
    #[inline(always)] pub fn as_raw_BackgroundSubtractorLSBP(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for BackgroundSubtractorLSBP {}

impl BackgroundSubtractorLSBP {
    ///
    /// ## C++ default parameters
    /// * learning_rate: -1
    pub fn apply(&mut self, image: &dyn core::ToInputArray, fgmask: &mut dyn core::ToOutputArray, learning_rate: f64) -> Result<()> {
        input_array_arg!(image);
        output_array_arg!(fgmask);
        unsafe { sys::cv_bgsegm_BackgroundSubtractorLSBP_apply__InputArray__OutputArray_double(self.as_raw_BackgroundSubtractorLSBP(), image.as_raw__InputArray(), fgmask.as_raw__OutputArray(), learning_rate) }.into_result()
    }
    
    pub fn get_background_image(&self, background_image: &mut dyn core::ToOutputArray) -> Result<()> {
        output_array_arg!(background_image);
        unsafe { sys::cv_bgsegm_BackgroundSubtractorLSBP_getBackgroundImage_const__OutputArray(self.as_raw_BackgroundSubtractorLSBP(), background_image.as_raw__OutputArray()) }.into_result()
    }
    
}

// boxed class cv::bgsegm::BackgroundSubtractorLSBPDesc
/// This is for calculation of the LSBP descriptors.
pub struct BackgroundSubtractorLSBPDesc {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for BackgroundSubtractorLSBPDesc {
    fn drop(&mut self) {
        unsafe { sys::cv_BackgroundSubtractorLSBPDesc_delete(self.ptr) };
    }
}

impl BackgroundSubtractorLSBPDesc {
    #[inline(always)] pub fn as_raw_BackgroundSubtractorLSBPDesc(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for BackgroundSubtractorLSBPDesc {}

impl BackgroundSubtractorLSBPDesc {
    pub fn calc_local_svd_values(local_svd_values: &mut dyn core::ToOutputArray, frame: &core::Mat) -> Result<()> {
        output_array_arg!(local_svd_values);
        unsafe { sys::cv_bgsegm_BackgroundSubtractorLSBPDesc_calcLocalSVDValues__OutputArray_Mat(local_svd_values.as_raw__OutputArray(), frame.as_raw_Mat()) }.into_result()
    }
    
    pub fn compute_from_local_svd_values(desc: &mut dyn core::ToOutputArray, local_svd_values: &core::Mat, lsbp_sample_points: &core::Point2i) -> Result<()> {
        output_array_arg!(desc);
        unsafe { sys::cv_bgsegm_BackgroundSubtractorLSBPDesc_computeFromLocalSVDValues__OutputArray_Mat_const_Point2i_X(desc.as_raw__OutputArray(), local_svd_values.as_raw_Mat(), lsbp_sample_points) }.into_result()
    }
    
    pub fn compute(desc: &mut dyn core::ToOutputArray, frame: &core::Mat, lsbp_sample_points: &core::Point2i) -> Result<()> {
        output_array_arg!(desc);
        unsafe { sys::cv_bgsegm_BackgroundSubtractorLSBPDesc_compute__OutputArray_Mat_const_Point2i_X(desc.as_raw__OutputArray(), frame.as_raw_Mat(), lsbp_sample_points) }.into_result()
    }
    
}

// Generating impl for trait crate::bgsegm::BackgroundSubtractorMOG
/// Gaussian Mixture-based Background/Foreground Segmentation Algorithm.
///
/// The class implements the algorithm described in [KB2001](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_KB2001) .
pub trait BackgroundSubtractorMOG {
    fn as_raw_BackgroundSubtractorMOG(&self) -> *mut c_void;
    fn get_history(&self) -> Result<i32> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorMOG_getHistory_const(self.as_raw_BackgroundSubtractorMOG()) }.into_result()
    }
    
    fn set_history(&mut self, nframes: i32) -> Result<()> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorMOG_setHistory_int(self.as_raw_BackgroundSubtractorMOG(), nframes) }.into_result()
    }
    
    fn get_n_mixtures(&self) -> Result<i32> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorMOG_getNMixtures_const(self.as_raw_BackgroundSubtractorMOG()) }.into_result()
    }
    
    fn set_n_mixtures(&mut self, nmix: i32) -> Result<()> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorMOG_setNMixtures_int(self.as_raw_BackgroundSubtractorMOG(), nmix) }.into_result()
    }
    
    fn get_background_ratio(&self) -> Result<f64> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorMOG_getBackgroundRatio_const(self.as_raw_BackgroundSubtractorMOG()) }.into_result()
    }
    
    fn set_background_ratio(&mut self, background_ratio: f64) -> Result<()> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorMOG_setBackgroundRatio_double(self.as_raw_BackgroundSubtractorMOG(), background_ratio) }.into_result()
    }
    
    fn get_noise_sigma(&self) -> Result<f64> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorMOG_getNoiseSigma_const(self.as_raw_BackgroundSubtractorMOG()) }.into_result()
    }
    
    fn set_noise_sigma(&mut self, noise_sigma: f64) -> Result<()> {
        unsafe { sys::cv_bgsegm_BackgroundSubtractorMOG_setNoiseSigma_double(self.as_raw_BackgroundSubtractorMOG(), noise_sigma) }.into_result()
    }
    
}

// boxed class cv::bgsegm::SyntheticSequenceGenerator
/// Synthetic frame sequence generator for testing background subtraction algorithms.
///
/// It will generate the moving object on top of the background.
/// It will apply some distortion to the background to make the test more complex.
pub struct SyntheticSequenceGenerator {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for SyntheticSequenceGenerator {
    fn drop(&mut self) {
        unsafe { sys::cv_SyntheticSequenceGenerator_delete(self.ptr) };
    }
}

impl SyntheticSequenceGenerator {
    #[inline(always)] pub fn as_raw_SyntheticSequenceGenerator(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for SyntheticSequenceGenerator {}

impl core::AlgorithmTrait for SyntheticSequenceGenerator {
    #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl SyntheticSequenceGenerator {
    /// Creates an instance of SyntheticSequenceGenerator.
    ///
    /// ## Parameters
    /// * background: Background image for object.
    /// * object: Object image which will move slowly over the background.
    /// * amplitude: Amplitude of wave distortion applied to background.
    /// * wavelength: Length of waves in distortion applied to background.
    /// * wavespeed: How fast waves will move.
    /// * objspeed: How fast object will fly over background.
    pub fn new(background: &dyn core::ToInputArray, object: &dyn core::ToInputArray, amplitude: f64, wavelength: f64, wavespeed: f64, objspeed: f64) -> Result<crate::bgsegm::SyntheticSequenceGenerator> {
        input_array_arg!(background);
        input_array_arg!(object);
        unsafe { sys::cv_bgsegm_SyntheticSequenceGenerator_SyntheticSequenceGenerator__InputArray__InputArray_double_double_double_double(background.as_raw__InputArray(), object.as_raw__InputArray(), amplitude, wavelength, wavespeed, objspeed) }.into_result().map(|ptr| crate::bgsegm::SyntheticSequenceGenerator { ptr })
    }
    
    /// Obtain the next frame in the sequence.
    ///
    /// ## Parameters
    /// * frame: Output frame.
    /// * gtMask: Output ground-truth (reference) segmentation mask object/background.
    pub fn get_next_frame(&mut self, frame: &mut dyn core::ToOutputArray, gt_mask: &mut dyn core::ToOutputArray) -> Result<()> {
        output_array_arg!(frame);
        output_array_arg!(gt_mask);
        unsafe { sys::cv_bgsegm_SyntheticSequenceGenerator_getNextFrame__OutputArray__OutputArray(self.as_raw_SyntheticSequenceGenerator(), frame.as_raw__OutputArray(), gt_mask.as_raw__OutputArray()) }.into_result()
    }
    
}

