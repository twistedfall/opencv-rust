//! # Improved Background-Foreground Segmentation Methods
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::BackgroundSubtractorMOG, super::BackgroundSubtractorGMG };
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

/// Background Subtractor module based on the algorithm given in [Gold2012](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Gold2012) .
/// 
/// Takes a series of images and returns a sequence of mask (8UC1)
/// images of the same size, where 255 indicates Foreground and 0 represents Background.
/// This class implements an algorithm described in "Visual Tracking of Human Visitors under
/// Variable-Lighting Conditions for a Responsive Audio Art Installation," A. Godbehere,
/// A. Matsukawa, K. Goldberg, American Control Conference, Montreal, June 2012.
pub trait BackgroundSubtractorGMG: core::AlgorithmTrait + crate::video::BackgroundSubtractor {
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

/// Gaussian Mixture-based Background/Foreground Segmentation Algorithm.
/// 
/// The class implements the algorithm described in [KB2001](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_KB2001) .
pub trait BackgroundSubtractorMOG: core::AlgorithmTrait + crate::video::BackgroundSubtractor {
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
