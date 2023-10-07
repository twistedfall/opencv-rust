pub mod text {
	//! # Scene Text Detection and Recognition
	//! 
	//! The opencv_text module provides different algorithms for text detection and recognition in natural
	//! scene images.
	//!    # Scene Text Detection
	//! 
	//! Class-specific Extremal Regions for Scene Text Detection
	//! --------------------------------------------------------
	//! 
	//! The scene text detection algorithm described below has been initially proposed by Lukás Neumann &
	//! Jiri Matas [Neumann11](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Neumann11). The main idea behind Class-specific Extremal Regions is similar to the MSER
	//! in that suitable Extremal Regions (ERs) are selected from the whole component tree of the image.
	//! However, this technique differs from MSER in that selection of suitable ERs is done by a sequential
	//! classifier trained for character detection, i.e. dropping the stability requirement of MSERs and
	//! selecting class-specific (not necessarily stable) regions.
	//! 
	//! The component tree of an image is constructed by thresholding by an increasing value step-by-step
	//! from 0 to 255 and then linking the obtained connected components from successive levels in a
	//! hierarchy by their inclusion relation:
	//! 
	//! ![image](https://docs.opencv.org/4.8.1/component_tree.png)
	//! 
	//! The component tree may contain a huge number of regions even for a very simple image as shown in
	//! the previous image. This number can easily reach the order of 1 x 10\^6 regions for an average 1
	//! Megapixel image. In order to efficiently select suitable regions among all the ERs the algorithm
	//! make use of a sequential classifier with two differentiated stages.
	//! 
	//! In the first stage incrementally computable descriptors (area, perimeter, bounding box, and Euler's
	//! number) are computed (in O(1)) for each region r and used as features for a classifier which
	//! estimates the class-conditional probability p(r|character). Only the ERs which correspond to local
	//! maximum of the probability p(r|character) are selected (if their probability is above a global limit
	//! p_min and the difference between local maximum and local minimum is greater than a delta_min
	//! value).
	//! 
	//! In the second stage, the ERs that passed the first stage are classified into character and
	//! non-character classes using more informative but also more computationally expensive features. (Hole
	//! area ratio, convex hull ratio, and the number of outer boundary inflexion points).
	//! 
	//! This ER filtering process is done in different single-channel projections of the input image in
	//! order to increase the character localization recall.
	//! 
	//! After the ER filtering is done on each input channel, character candidates must be grouped in
	//! high-level text blocks (i.e. words, text lines, paragraphs, ...). The opencv_text module implements
	//! two different grouping algorithms: the Exhaustive Search algorithm proposed in [Neumann12](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Neumann12) for
	//! grouping horizontally aligned text, and the method proposed by Lluis Gomez and Dimosthenis Karatzas
	//! in [Gomez13](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Gomez13) [Gomez14](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Gomez14) for grouping arbitrary oriented text (see erGrouping).
	//! 
	//! To see the text detector at work, have a look at the textdetection demo:
	//! <https://github.com/opencv/opencv_contrib/blob/master/modules/text/samples/textdetection.cpp>
	//! 
	//!    # Scene Text Recognition
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::ERStatTraitConst, super::ERStatTrait, super::ERFilter_CallbackTraitConst, super::ERFilter_CallbackTrait, super::ERFilterTraitConst, super::ERFilterTrait, super::BaseOCRTraitConst, super::BaseOCRTrait, super::OCRTesseractTraitConst, super::OCRTesseractTrait, super::OCRHMMDecoder_ClassifierCallbackTraitConst, super::OCRHMMDecoder_ClassifierCallbackTrait, super::OCRHMMDecoderTraitConst, super::OCRHMMDecoderTrait, super::OCRBeamSearchDecoder_ClassifierCallbackTraitConst, super::OCRBeamSearchDecoder_ClassifierCallbackTrait, super::OCRBeamSearchDecoderTraitConst, super::OCRBeamSearchDecoderTrait, super::OCRHolisticWordRecognizerTraitConst, super::OCRHolisticWordRecognizerTrait, super::TextDetectorTraitConst, super::TextDetectorTrait, super::TextDetectorCNNTraitConst, super::TextDetectorCNNTrait };
	}
	
	pub const ERFILTER_NM_IHSGrad: i32 = 1;
	pub const ERFILTER_NM_RGBLGrad: i32 = 0;
	/// Text grouping method proposed in [Gomez13](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Gomez13) [Gomez14](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Gomez14) for grouping arbitrary oriented text. Regions
	/// are agglomerated by Single Linkage Clustering in a weighted feature space that combines proximity
	/// (x,y coordinates) and similarity measures (color, size, gradient magnitude, stroke width, etc.).
	/// SLC provides a dendrogram where each node represents a text group hypothesis. Then the algorithm
	/// finds the branches corresponding to text groups by traversing this dendrogram with a stopping rule
	/// that combines the output of a rotation invariant text group classifier and a probabilistic measure
	/// for hierarchical clustering validity assessment.
	/// 
	/// 
	/// Note: This mode is not supported due NFA code removal ( <https://github.com/opencv/opencv_contrib/issues/2235> )
	pub const ERGROUPING_ORIENTATION_ANY: i32 = 1;
	/// Exhaustive Search algorithm proposed in [Neumann11](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Neumann11) for grouping horizontally aligned text.
	/// The algorithm models a verification function for all the possible ER sequences. The
	/// verification fuction for ER pairs consists in a set of threshold-based pairwise rules which
	/// compare measurements of two regions (height ratio, centroid angle, and region distance). The
	/// verification function for ER triplets creates a word text line estimate using Least
	/// Median-Squares fitting for a given triplet and then verifies that the estimate is valid (based
	/// on thresholds created during training). Verification functions for sequences larger than 3 are
	/// approximated by verifying that the text line parameters of all (sub)sequences of length 3 are
	/// consistent.
	pub const ERGROUPING_ORIENTATION_HORIZ: i32 = 0;
	pub const OCR_CNN_CLASSIFIER: i32 = 1;
	pub const OCR_DECODER_VITERBI: i32 = 0;
	pub const OCR_KNN_CLASSIFIER: i32 = 0;
	pub const OCR_LEVEL_TEXTLINE: i32 = 1;
	pub const OCR_LEVEL_WORD: i32 = 0;
	pub const OEM_CUBE_ONLY: i32 = 1;
	pub const OEM_DEFAULT: i32 = 3;
	pub const OEM_TESSERACT_CUBE_COMBINED: i32 = 2;
	pub const OEM_TESSERACT_ONLY: i32 = 0;
	pub const PSM_AUTO: i32 = 3;
	pub const PSM_AUTO_ONLY: i32 = 2;
	pub const PSM_AUTO_OSD: i32 = 1;
	pub const PSM_CIRCLE_WORD: i32 = 9;
	pub const PSM_OSD_ONLY: i32 = 0;
	pub const PSM_SINGLE_BLOCK: i32 = 6;
	pub const PSM_SINGLE_BLOCK_VERT_TEXT: i32 = 5;
	pub const PSM_SINGLE_CHAR: i32 = 10;
	pub const PSM_SINGLE_COLUMN: i32 = 4;
	pub const PSM_SINGLE_LINE: i32 = 7;
	pub const PSM_SINGLE_WORD: i32 = 8;
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum classifier_type {
		OCR_KNN_CLASSIFIER = 0,
		OCR_CNN_CLASSIFIER = 1,
	}
	
	opencv_type_enum! { crate::text::classifier_type }
	
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum decoder_mode {
		OCR_DECODER_VITERBI = 0,
	}
	
	opencv_type_enum! { crate::text::decoder_mode }
	
	/// text::erGrouping operation modes
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum erGrouping_Modes {
		/// Exhaustive Search algorithm proposed in [Neumann11](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Neumann11) for grouping horizontally aligned text.
		/// The algorithm models a verification function for all the possible ER sequences. The
		/// verification fuction for ER pairs consists in a set of threshold-based pairwise rules which
		/// compare measurements of two regions (height ratio, centroid angle, and region distance). The
		/// verification function for ER triplets creates a word text line estimate using Least
		/// Median-Squares fitting for a given triplet and then verifies that the estimate is valid (based
		/// on thresholds created during training). Verification functions for sequences larger than 3 are
		/// approximated by verifying that the text line parameters of all (sub)sequences of length 3 are
		/// consistent.
		ERGROUPING_ORIENTATION_HORIZ = 0,
		/// Text grouping method proposed in [Gomez13](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Gomez13) [Gomez14](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Gomez14) for grouping arbitrary oriented text. Regions
		/// are agglomerated by Single Linkage Clustering in a weighted feature space that combines proximity
		/// (x,y coordinates) and similarity measures (color, size, gradient magnitude, stroke width, etc.).
		/// SLC provides a dendrogram where each node represents a text group hypothesis. Then the algorithm
		/// finds the branches corresponding to text groups by traversing this dendrogram with a stopping rule
		/// that combines the output of a rotation invariant text group classifier and a probabilistic measure
		/// for hierarchical clustering validity assessment.
		/// 
		/// 
		/// Note: This mode is not supported due NFA code removal ( <https://github.com/opencv/opencv_contrib/issues/2235> )
		ERGROUPING_ORIENTATION_ANY = 1,
	}
	
	opencv_type_enum! { crate::text::erGrouping_Modes }
	
	/// Tesseract.OcrEngineMode Enumeration
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum ocr_engine_mode {
		OEM_TESSERACT_ONLY = 0,
		OEM_CUBE_ONLY = 1,
		OEM_TESSERACT_CUBE_COMBINED = 2,
		OEM_DEFAULT = 3,
	}
	
	opencv_type_enum! { crate::text::ocr_engine_mode }
	
	/// Tesseract.PageSegMode Enumeration
	#[repr(C)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum page_seg_mode {
		PSM_OSD_ONLY = 0,
		PSM_AUTO_OSD = 1,
		PSM_AUTO_ONLY = 2,
		PSM_AUTO = 3,
		PSM_SINGLE_COLUMN = 4,
		PSM_SINGLE_BLOCK_VERT_TEXT = 5,
		PSM_SINGLE_BLOCK = 6,
		PSM_SINGLE_LINE = 7,
		PSM_SINGLE_WORD = 8,
		PSM_CIRCLE_WORD = 9,
		PSM_SINGLE_CHAR = 10,
	}
	
	opencv_type_enum! { crate::text::page_seg_mode }
	
	/// Converts MSER contours (vector\<Point\>) to ERStat regions.
	/// 
	/// ## Parameters
	/// * image: Source image CV_8UC1 from which the MSERs where extracted.
	/// 
	/// * contours: Input vector with all the contours (vector\<Point\>).
	/// 
	/// * regions: Output where the ERStat regions are stored.
	/// 
	/// It takes as input the contours provided by the OpenCV MSER feature detector and returns as output
	/// two vectors of ERStats. This is because MSER() output contains both MSER+ and MSER- regions in a
	/// single vector\<Point\>, the function separates them in two different vectors (this is as if the
	/// ERStats where extracted from two different channels).
	/// 
	/// An example of MSERsToERStats in use can be found in the text detection webcam_demo:
	/// <https://github.com/opencv/opencv_contrib/blob/master/modules/text/samples/webcam_demo.cpp>
	#[inline]
	pub fn mse_rs_to_er_stats(image: &impl core::ToInputArray, contours: &mut core::Vector<core::Vector<core::Point>>, regions: &mut core::Vector<core::Vector<crate::text::ERStat>>) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_MSERsToERStats_const__InputArrayR_vectorLvectorLPointGGR_vectorLvectorLERStatGGR(image.as_raw__InputArray(), contours.as_raw_mut_VectorOfVectorOfPoint(), regions.as_raw_mut_VectorOfVectorOfERStat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Compute the different channels to be processed independently in the N&M algorithm [Neumann12](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Neumann12).
	/// 
	/// ## Parameters
	/// * _src: Source image. Must be RGB CV_8UC3.
	/// 
	/// * _channels: Output vector\<Mat\> where computed channels are stored.
	/// 
	/// * _mode: Mode of operation. Currently the only available options are:
	/// **ERFILTER_NM_RGBLGrad** (used by default) and **ERFILTER_NM_IHSGrad**.
	/// 
	/// In N&M algorithm, the combination of intensity (I), hue (H), saturation (S), and gradient magnitude
	/// channels (Grad) are used in order to obtain high localization recall. This implementation also
	/// provides an alternative combination of red (R), green (G), blue (B), lightness (L), and gradient
	/// magnitude (Grad).
	/// 
	/// ## Note
	/// This alternative version of [compute_nm_channels] function uses the following default values for its arguments:
	/// * _mode: ERFILTER_NM_RGBLGrad
	#[inline]
	pub fn compute_nm_channels_def(_src: &impl core::ToInputArray, _channels: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(_src);
		output_array_arg!(_channels);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_computeNMChannels_const__InputArrayR_const__OutputArrayR(_src.as_raw__InputArray(), _channels.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Compute the different channels to be processed independently in the N&M algorithm [Neumann12](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Neumann12).
	/// 
	/// ## Parameters
	/// * _src: Source image. Must be RGB CV_8UC3.
	/// 
	/// * _channels: Output vector\<Mat\> where computed channels are stored.
	/// 
	/// * _mode: Mode of operation. Currently the only available options are:
	/// **ERFILTER_NM_RGBLGrad** (used by default) and **ERFILTER_NM_IHSGrad**.
	/// 
	/// In N&M algorithm, the combination of intensity (I), hue (H), saturation (S), and gradient magnitude
	/// channels (Grad) are used in order to obtain high localization recall. This implementation also
	/// provides an alternative combination of red (R), green (G), blue (B), lightness (L), and gradient
	/// magnitude (Grad).
	/// 
	/// ## C++ default parameters
	/// * _mode: ERFILTER_NM_RGBLGrad
	#[inline]
	pub fn compute_nm_channels(_src: &impl core::ToInputArray, _channels: &mut impl core::ToOutputArray, _mode: i32) -> Result<()> {
		input_array_arg!(_src);
		output_array_arg!(_channels);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_computeNMChannels_const__InputArrayR_const__OutputArrayR_int(_src.as_raw__InputArray(), _channels.as_raw__OutputArray(), _mode, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Create an Extremal Region Filter for the 1st stage classifier of N&M algorithm [Neumann12](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Neumann12).
	/// 
	/// ## Parameters
	/// * cb: :   Callback with the classifier. Default classifier can be implicitly load with function
	/// loadClassifierNM1, e.g. from file in samples/cpp/trained_classifierNM1.xml
	/// * thresholdDelta: :   Threshold step in subsequent thresholds when extracting the component tree
	/// * minArea: :   The minimum area (% of image size) allowed for retreived ER's
	/// * maxArea: :   The maximum area (% of image size) allowed for retreived ER's
	/// * minProbability: :   The minimum probability P(er|character) allowed for retreived ER's
	/// * nonMaxSuppression: :   Whenever non-maximum suppression is done over the branch probabilities
	/// * minProbabilityDiff: :   The minimum probability difference between local maxima and local minima ERs
	/// 
	/// The component tree of the image is extracted by a threshold increased step by step from 0 to 255,
	/// incrementally computable descriptors (aspect_ratio, compactness, number of holes, and number of
	/// horizontal crossings) are computed for each ER and used as features for a classifier which estimates
	/// the class-conditional probability P(er|character). The value of P(er|character) is tracked using the
	/// inclusion relation of ER across all thresholds and only the ERs which correspond to local maximum of
	/// the probability P(er|character) are selected (if the local maximum of the probability is above a
	/// global limit pmin and the difference between local maximum and local minimum is greater than
	/// minProbabilityDiff).
	/// 
	/// ## Note
	/// This alternative version of [create_er_filter_nm1] function uses the following default values for its arguments:
	/// * threshold_delta: 1
	/// * min_area: (float)0.00025
	/// * max_area: (float)0.13
	/// * min_probability: (float)0.4
	/// * non_max_suppression: true
	/// * min_probability_diff: (float)0.1
	#[inline]
	pub fn create_er_filter_nm1_def(cb: &core::Ptr<crate::text::ERFilter_Callback>) -> Result<core::Ptr<crate::text::ERFilter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_createERFilterNM1_const_PtrLCallbackGR(cb.as_raw_PtrOfERFilter_Callback(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::text::ERFilter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Create an Extremal Region Filter for the 1st stage classifier of N&M algorithm [Neumann12](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Neumann12).
	/// 
	/// ## Parameters
	/// * cb: :   Callback with the classifier. Default classifier can be implicitly load with function
	/// loadClassifierNM1, e.g. from file in samples/cpp/trained_classifierNM1.xml
	/// * thresholdDelta: :   Threshold step in subsequent thresholds when extracting the component tree
	/// * minArea: :   The minimum area (% of image size) allowed for retreived ER's
	/// * maxArea: :   The maximum area (% of image size) allowed for retreived ER's
	/// * minProbability: :   The minimum probability P(er|character) allowed for retreived ER's
	/// * nonMaxSuppression: :   Whenever non-maximum suppression is done over the branch probabilities
	/// * minProbabilityDiff: :   The minimum probability difference between local maxima and local minima ERs
	/// 
	/// The component tree of the image is extracted by a threshold increased step by step from 0 to 255,
	/// incrementally computable descriptors (aspect_ratio, compactness, number of holes, and number of
	/// horizontal crossings) are computed for each ER and used as features for a classifier which estimates
	/// the class-conditional probability P(er|character). The value of P(er|character) is tracked using the
	/// inclusion relation of ER across all thresholds and only the ERs which correspond to local maximum of
	/// the probability P(er|character) are selected (if the local maximum of the probability is above a
	/// global limit pmin and the difference between local maximum and local minimum is greater than
	/// minProbabilityDiff).
	/// 
	/// ## C++ default parameters
	/// * threshold_delta: 1
	/// * min_area: (float)0.00025
	/// * max_area: (float)0.13
	/// * min_probability: (float)0.4
	/// * non_max_suppression: true
	/// * min_probability_diff: (float)0.1
	#[inline]
	pub fn create_er_filter_nm1(cb: &core::Ptr<crate::text::ERFilter_Callback>, threshold_delta: i32, min_area: f32, max_area: f32, min_probability: f32, non_max_suppression: bool, min_probability_diff: f32) -> Result<core::Ptr<crate::text::ERFilter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_createERFilterNM1_const_PtrLCallbackGR_int_float_float_float_bool_float(cb.as_raw_PtrOfERFilter_Callback(), threshold_delta, min_area, max_area, min_probability, non_max_suppression, min_probability_diff, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::text::ERFilter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Reads an Extremal Region Filter for the 1st stage classifier of N&M algorithm
	///    from the provided path e.g. /path/to/cpp/trained_classifierNM1.xml
	/// 
	/// @overload
	/// 
	/// ## Note
	/// This alternative version of [create_er_filter_nm1_from_file] function uses the following default values for its arguments:
	/// * threshold_delta: 1
	/// * min_area: (float)0.00025
	/// * max_area: (float)0.13
	/// * min_probability: (float)0.4
	/// * non_max_suppression: true
	/// * min_probability_diff: (float)0.1
	#[inline]
	pub fn create_er_filter_nm1_from_file_def(filename: &str) -> Result<core::Ptr<crate::text::ERFilter>> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_createERFilterNM1_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::text::ERFilter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Reads an Extremal Region Filter for the 1st stage classifier of N&M algorithm
	///    from the provided path e.g. /path/to/cpp/trained_classifierNM1.xml
	/// 
	/// Create an Extremal Region Filter for the 1st stage classifier of N&M algorithm [Neumann12](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Neumann12).
	/// 
	/// ## Parameters
	/// * cb: :   Callback with the classifier. Default classifier can be implicitly load with function
	/// loadClassifierNM1, e.g. from file in samples/cpp/trained_classifierNM1.xml
	/// * thresholdDelta: :   Threshold step in subsequent thresholds when extracting the component tree
	/// * minArea: :   The minimum area (% of image size) allowed for retreived ER's
	/// * maxArea: :   The maximum area (% of image size) allowed for retreived ER's
	/// * minProbability: :   The minimum probability P(er|character) allowed for retreived ER's
	/// * nonMaxSuppression: :   Whenever non-maximum suppression is done over the branch probabilities
	/// * minProbabilityDiff: :   The minimum probability difference between local maxima and local minima ERs
	/// 
	/// The component tree of the image is extracted by a threshold increased step by step from 0 to 255,
	/// incrementally computable descriptors (aspect_ratio, compactness, number of holes, and number of
	/// horizontal crossings) are computed for each ER and used as features for a classifier which estimates
	/// the class-conditional probability P(er|character). The value of P(er|character) is tracked using the
	/// inclusion relation of ER across all thresholds and only the ERs which correspond to local maximum of
	/// the probability P(er|character) are selected (if the local maximum of the probability is above a
	/// global limit pmin and the difference between local maximum and local minimum is greater than
	/// minProbabilityDiff).
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * threshold_delta: 1
	/// * min_area: (float)0.00025
	/// * max_area: (float)0.13
	/// * min_probability: (float)0.4
	/// * non_max_suppression: true
	/// * min_probability_diff: (float)0.1
	#[inline]
	pub fn create_er_filter_nm1_from_file(filename: &str, threshold_delta: i32, min_area: f32, max_area: f32, min_probability: f32, non_max_suppression: bool, min_probability_diff: f32) -> Result<core::Ptr<crate::text::ERFilter>> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_createERFilterNM1_const_StringR_int_float_float_float_bool_float(filename.opencv_as_extern(), threshold_delta, min_area, max_area, min_probability, non_max_suppression, min_probability_diff, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::text::ERFilter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Create an Extremal Region Filter for the 2nd stage classifier of N&M algorithm [Neumann12](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Neumann12).
	/// 
	/// ## Parameters
	/// * cb: :   Callback with the classifier. Default classifier can be implicitly load with function
	/// loadClassifierNM2, e.g. from file in samples/cpp/trained_classifierNM2.xml
	/// * minProbability: :   The minimum probability P(er|character) allowed for retreived ER's
	/// 
	/// In the second stage, the ERs that passed the first stage are classified into character and
	/// non-character classes using more informative but also more computationally expensive features. The
	/// classifier uses all the features calculated in the first stage and the following additional
	/// features: hole area ratio, convex hull ratio, and number of outer inflexion points.
	/// 
	/// ## Note
	/// This alternative version of [create_er_filter_nm2] function uses the following default values for its arguments:
	/// * min_probability: (float)0.3
	#[inline]
	pub fn create_er_filter_nm2_def(cb: &core::Ptr<crate::text::ERFilter_Callback>) -> Result<core::Ptr<crate::text::ERFilter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_createERFilterNM2_const_PtrLCallbackGR(cb.as_raw_PtrOfERFilter_Callback(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::text::ERFilter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Create an Extremal Region Filter for the 2nd stage classifier of N&M algorithm [Neumann12](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Neumann12).
	/// 
	/// ## Parameters
	/// * cb: :   Callback with the classifier. Default classifier can be implicitly load with function
	/// loadClassifierNM2, e.g. from file in samples/cpp/trained_classifierNM2.xml
	/// * minProbability: :   The minimum probability P(er|character) allowed for retreived ER's
	/// 
	/// In the second stage, the ERs that passed the first stage are classified into character and
	/// non-character classes using more informative but also more computationally expensive features. The
	/// classifier uses all the features calculated in the first stage and the following additional
	/// features: hole area ratio, convex hull ratio, and number of outer inflexion points.
	/// 
	/// ## C++ default parameters
	/// * min_probability: (float)0.3
	#[inline]
	pub fn create_er_filter_nm2(cb: &core::Ptr<crate::text::ERFilter_Callback>, min_probability: f32) -> Result<core::Ptr<crate::text::ERFilter>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_createERFilterNM2_const_PtrLCallbackGR_float(cb.as_raw_PtrOfERFilter_Callback(), min_probability, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::text::ERFilter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Reads an Extremal Region Filter for the 2nd stage classifier of N&M algorithm
	///    from the provided path e.g. /path/to/cpp/trained_classifierNM2.xml
	/// 
	/// @overload
	/// 
	/// ## Note
	/// This alternative version of [create_er_filter_nm2_from_file] function uses the following default values for its arguments:
	/// * min_probability: (float)0.3
	#[inline]
	pub fn create_er_filter_nm2_from_file_def(filename: &str) -> Result<core::Ptr<crate::text::ERFilter>> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_createERFilterNM2_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::text::ERFilter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Reads an Extremal Region Filter for the 2nd stage classifier of N&M algorithm
	///    from the provided path e.g. /path/to/cpp/trained_classifierNM2.xml
	/// 
	/// Create an Extremal Region Filter for the 2nd stage classifier of N&M algorithm [Neumann12](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Neumann12).
	/// 
	/// ## Parameters
	/// * cb: :   Callback with the classifier. Default classifier can be implicitly load with function
	/// loadClassifierNM2, e.g. from file in samples/cpp/trained_classifierNM2.xml
	/// * minProbability: :   The minimum probability P(er|character) allowed for retreived ER's
	/// 
	/// In the second stage, the ERs that passed the first stage are classified into character and
	/// non-character classes using more informative but also more computationally expensive features. The
	/// classifier uses all the features calculated in the first stage and the following additional
	/// features: hole area ratio, convex hull ratio, and number of outer inflexion points.
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## C++ default parameters
	/// * min_probability: (float)0.3
	#[inline]
	pub fn create_er_filter_nm2_from_file(filename: &str, min_probability: f32) -> Result<core::Ptr<crate::text::ERFilter>> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_createERFilterNM2_const_StringR_float(filename.opencv_as_extern(), min_probability, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::text::ERFilter>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn create_ocrhmm_transitions_table_1(vocabulary: &str, lexicon: &mut core::Vector<String>) -> Result<core::Mat> {
		extern_container_arg!(vocabulary);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_createOCRHMMTransitionsTable_const_StringR_vectorLStringGR(vocabulary.opencv_as_extern(), lexicon.as_raw_mut_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Utility function to create a tailored language model transitions table from a given list of words (lexicon).
	/// 
	/// ## Parameters
	/// * vocabulary: The language vocabulary (chars when ASCII English text).
	/// 
	/// * lexicon: The list of words that are expected to be found in a particular image.
	/// 
	/// * transition_probabilities_table: Output table with transition probabilities between character pairs. cols == rows == vocabulary.size().
	/// 
	/// The function calculate frequency statistics of character pairs from the given lexicon and fills the output transition_probabilities_table with them. The transition_probabilities_table can be used as input in the OCRHMMDecoder::create() and OCRBeamSearchDecoder::create() methods.
	/// 
	/// Note:
	///    *   (C++) An alternative would be to load the default generic language transition table provided in the text module samples folder (created from ispell 42869 english words list) :
	///            <https://github.com/opencv/opencv_contrib/blob/master/modules/text/samples/OCRHMM_transitions_table.xml>
	#[inline]
	pub fn create_ocrhmm_transitions_table(vocabulary: &mut String, lexicon: &mut core::Vector<String>, transition_probabilities_table: &mut impl core::ToOutputArray) -> Result<()> {
		string_arg_output_send!(via vocabulary_via);
		output_array_arg!(transition_probabilities_table);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_createOCRHMMTransitionsTable_stringR_vectorLstringGR_const__OutputArrayR(&mut vocabulary_via, lexicon.as_raw_mut_VectorOfString(), transition_probabilities_table.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		string_arg_output_receive!(vocabulary_via => vocabulary);
		Ok(ret)
	}
	
	/// Extracts text regions from image.
	/// 
	/// ## Parameters
	/// * image: Source image where text blocks needs to be extracted from.  Should be CV_8UC3 (color).
	/// * er_filter1: Extremal Region Filter for the 1st stage classifier of N&M algorithm [Neumann12](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Neumann12)
	/// * er_filter2: Extremal Region Filter for the 2nd stage classifier of N&M algorithm [Neumann12](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Neumann12)
	/// * groups_rects: Output list of rectangle blocks with text
	/// * method: Grouping method (see text::erGrouping_Modes). Can be one of ERGROUPING_ORIENTATION_HORIZ, ERGROUPING_ORIENTATION_ANY.
	/// * filename: The XML or YAML file with the classifier model (e.g. samples/trained_classifier_erGrouping.xml). Only to use when grouping method is ERGROUPING_ORIENTATION_ANY.
	/// * minProbability: The minimum probability for accepting a group. Only to use when grouping method is ERGROUPING_ORIENTATION_ANY.
	/// 
	/// ## Note
	/// This alternative version of [detect_regions_from_file] function uses the following default values for its arguments:
	/// * method: ERGROUPING_ORIENTATION_HORIZ
	/// * filename: String()
	/// * min_probability: (float)0.5
	#[inline]
	pub fn detect_regions_from_file_def(image: &impl core::ToInputArray, er_filter1: &core::Ptr<crate::text::ERFilter>, er_filter2: &core::Ptr<crate::text::ERFilter>, groups_rects: &mut core::Vector<core::Rect>) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_detectRegions_const__InputArrayR_const_PtrLERFilterGR_const_PtrLERFilterGR_vectorLRectGR(image.as_raw__InputArray(), er_filter1.as_raw_PtrOfERFilter(), er_filter2.as_raw_PtrOfERFilter(), groups_rects.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Extracts text regions from image.
	/// 
	/// ## Parameters
	/// * image: Source image where text blocks needs to be extracted from.  Should be CV_8UC3 (color).
	/// * er_filter1: Extremal Region Filter for the 1st stage classifier of N&M algorithm [Neumann12](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Neumann12)
	/// * er_filter2: Extremal Region Filter for the 2nd stage classifier of N&M algorithm [Neumann12](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Neumann12)
	/// * groups_rects: Output list of rectangle blocks with text
	/// * method: Grouping method (see text::erGrouping_Modes). Can be one of ERGROUPING_ORIENTATION_HORIZ, ERGROUPING_ORIENTATION_ANY.
	/// * filename: The XML or YAML file with the classifier model (e.g. samples/trained_classifier_erGrouping.xml). Only to use when grouping method is ERGROUPING_ORIENTATION_ANY.
	/// * minProbability: The minimum probability for accepting a group. Only to use when grouping method is ERGROUPING_ORIENTATION_ANY.
	/// 
	/// ## C++ default parameters
	/// * method: ERGROUPING_ORIENTATION_HORIZ
	/// * filename: String()
	/// * min_probability: (float)0.5
	#[inline]
	pub fn detect_regions_from_file(image: &impl core::ToInputArray, er_filter1: &core::Ptr<crate::text::ERFilter>, er_filter2: &core::Ptr<crate::text::ERFilter>, groups_rects: &mut core::Vector<core::Rect>, method: i32, filename: &str, min_probability: f32) -> Result<()> {
		input_array_arg!(image);
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_detectRegions_const__InputArrayR_const_PtrLERFilterGR_const_PtrLERFilterGR_vectorLRectGR_int_const_StringR_float(image.as_raw__InputArray(), er_filter1.as_raw_PtrOfERFilter(), er_filter2.as_raw_PtrOfERFilter(), groups_rects.as_raw_mut_VectorOfRect(), method, filename.opencv_as_extern(), min_probability, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn detect_regions(image: &impl core::ToInputArray, er_filter1: &core::Ptr<crate::text::ERFilter>, er_filter2: &core::Ptr<crate::text::ERFilter>, regions: &mut core::Vector<core::Vector<core::Point>>) -> Result<()> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_detectRegions_const__InputArrayR_const_PtrLERFilterGR_const_PtrLERFilterGR_vectorLvectorLPointGGR(image.as_raw__InputArray(), er_filter1.as_raw_PtrOfERFilter(), er_filter2.as_raw_PtrOfERFilter(), regions.as_raw_mut_VectorOfVectorOfPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Applies the Stroke Width Transform operator followed by filtering of connected components of similar Stroke Widths to return letter candidates. It also chain them by proximity and size, saving the result in chainBBs.
	/// ## Parameters
	/// * input: the input image with 3 channels.
	/// * result: a vector of resulting bounding boxes where probability of finding text is high
	/// * dark_on_light: a boolean value signifying whether the text is darker or lighter than the background, it is observed to reverse the gradient obtained from Scharr operator, and significantly affect the result.
	/// * draw: an optional Mat of type CV_8UC3 which visualises the detected letters using bounding boxes.
	/// * chainBBs: an optional parameter which chains the letter candidates according to heuristics in the paper and returns all possible regions where text is likely to occur.
	/// 
	/// ## Note
	/// This alternative version of [detect_text_swt] function uses the following default values for its arguments:
	/// * draw: noArray()
	/// * chain_b_bs: noArray()
	#[inline]
	pub fn detect_text_swt_def(input: &impl core::ToInputArray, result: &mut core::Vector<core::Rect>, dark_on_light: bool) -> Result<()> {
		input_array_arg!(input);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_detectTextSWT_const__InputArrayR_vectorLRectGR_bool(input.as_raw__InputArray(), result.as_raw_mut_VectorOfRect(), dark_on_light, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Applies the Stroke Width Transform operator followed by filtering of connected components of similar Stroke Widths to return letter candidates. It also chain them by proximity and size, saving the result in chainBBs.
	/// ## Parameters
	/// * input: the input image with 3 channels.
	/// * result: a vector of resulting bounding boxes where probability of finding text is high
	/// * dark_on_light: a boolean value signifying whether the text is darker or lighter than the background, it is observed to reverse the gradient obtained from Scharr operator, and significantly affect the result.
	/// * draw: an optional Mat of type CV_8UC3 which visualises the detected letters using bounding boxes.
	/// * chainBBs: an optional parameter which chains the letter candidates according to heuristics in the paper and returns all possible regions where text is likely to occur.
	/// 
	/// ## C++ default parameters
	/// * draw: noArray()
	/// * chain_b_bs: noArray()
	#[inline]
	pub fn detect_text_swt(input: &impl core::ToInputArray, result: &mut core::Vector<core::Rect>, dark_on_light: bool, draw: &mut impl core::ToOutputArray, chain_b_bs: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(input);
		output_array_arg!(draw);
		output_array_arg!(chain_b_bs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_detectTextSWT_const__InputArrayR_vectorLRectGR_bool_const__OutputArrayR_const__OutputArrayR(input.as_raw__InputArray(), result.as_raw_mut_VectorOfRect(), dark_on_light, draw.as_raw__OutputArray(), chain_b_bs.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Find groups of Extremal Regions that are organized as text blocks.
	/// 
	/// ## Parameters
	/// * img: Original RGB or Greyscale image from wich the regions were extracted.
	/// 
	/// * channels: Vector of single channel images CV_8UC1 from wich the regions were extracted.
	/// 
	/// * regions: Vector of ER's retrieved from the ERFilter algorithm from each channel.
	/// 
	/// * groups: The output of the algorithm is stored in this parameter as set of lists of indexes to
	/// provided regions.
	/// 
	/// * groups_rects: The output of the algorithm are stored in this parameter as list of rectangles.
	/// 
	/// * method: Grouping method (see text::erGrouping_Modes). Can be one of ERGROUPING_ORIENTATION_HORIZ,
	/// ERGROUPING_ORIENTATION_ANY.
	/// 
	/// * filename: The XML or YAML file with the classifier model (e.g.
	/// samples/trained_classifier_erGrouping.xml). Only to use when grouping method is
	/// ERGROUPING_ORIENTATION_ANY.
	/// 
	/// * minProbablity: The minimum probability for accepting a group. Only to use when grouping
	/// method is ERGROUPING_ORIENTATION_ANY.
	/// 
	/// ## Note
	/// This alternative version of [er_grouping] function uses the following default values for its arguments:
	/// * method: ERGROUPING_ORIENTATION_HORIZ
	/// * filename: std::string()
	/// * min_probablity: 0.5
	#[inline]
	pub fn er_grouping_def(img: &impl core::ToInputArray, channels: &impl core::ToInputArray, regions: &mut core::Vector<core::Vector<crate::text::ERStat>>, groups: &mut core::Vector<core::Vector<core::Vec2i>>, groups_rects: &mut core::Vector<core::Rect>) -> Result<()> {
		input_array_arg!(img);
		input_array_arg!(channels);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_erGrouping_const__InputArrayR_const__InputArrayR_vectorLvectorLERStatGGR_vectorLvectorLVec2iGGR_vectorLRectGR(img.as_raw__InputArray(), channels.as_raw__InputArray(), regions.as_raw_mut_VectorOfVectorOfERStat(), groups.as_raw_mut_VectorOfVectorOfVec2i(), groups_rects.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Find groups of Extremal Regions that are organized as text blocks.
	/// 
	/// ## Parameters
	/// * img: Original RGB or Greyscale image from wich the regions were extracted.
	/// 
	/// * channels: Vector of single channel images CV_8UC1 from wich the regions were extracted.
	/// 
	/// * regions: Vector of ER's retrieved from the ERFilter algorithm from each channel.
	/// 
	/// * groups: The output of the algorithm is stored in this parameter as set of lists of indexes to
	/// provided regions.
	/// 
	/// * groups_rects: The output of the algorithm are stored in this parameter as list of rectangles.
	/// 
	/// * method: Grouping method (see text::erGrouping_Modes). Can be one of ERGROUPING_ORIENTATION_HORIZ,
	/// ERGROUPING_ORIENTATION_ANY.
	/// 
	/// * filename: The XML or YAML file with the classifier model (e.g.
	/// samples/trained_classifier_erGrouping.xml). Only to use when grouping method is
	/// ERGROUPING_ORIENTATION_ANY.
	/// 
	/// * minProbablity: The minimum probability for accepting a group. Only to use when grouping
	/// method is ERGROUPING_ORIENTATION_ANY.
	/// 
	/// ## C++ default parameters
	/// * method: ERGROUPING_ORIENTATION_HORIZ
	/// * filename: std::string()
	/// * min_probablity: 0.5
	#[inline]
	pub fn er_grouping(img: &impl core::ToInputArray, channels: &impl core::ToInputArray, regions: &mut core::Vector<core::Vector<crate::text::ERStat>>, groups: &mut core::Vector<core::Vector<core::Vec2i>>, groups_rects: &mut core::Vector<core::Rect>, method: i32, filename: &str, min_probablity: f32) -> Result<()> {
		input_array_arg!(img);
		input_array_arg!(channels);
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_erGrouping_const__InputArrayR_const__InputArrayR_vectorLvectorLERStatGGR_vectorLvectorLVec2iGGR_vectorLRectGR_int_const_stringR_float(img.as_raw__InputArray(), channels.as_raw__InputArray(), regions.as_raw_mut_VectorOfVectorOfERStat(), groups.as_raw_mut_VectorOfVectorOfVec2i(), groups_rects.as_raw_mut_VectorOfRect(), method, filename.opencv_as_extern(), min_probablity, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [er_grouping_1] function uses the following default values for its arguments:
	/// * method: ERGROUPING_ORIENTATION_HORIZ
	/// * filename: String()
	/// * min_probablity: (float)0.5
	#[inline]
	pub fn er_grouping_1_def(image: &impl core::ToInputArray, channel: &impl core::ToInputArray, mut regions: core::Vector<core::Vector<core::Point>>, groups_rects: &mut core::Vector<core::Rect>) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(channel);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_erGrouping_const__InputArrayR_const__InputArrayR_vectorLvectorLPointGG_vectorLRectGR(image.as_raw__InputArray(), channel.as_raw__InputArray(), regions.as_raw_mut_VectorOfVectorOfPoint(), groups_rects.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * method: ERGROUPING_ORIENTATION_HORIZ
	/// * filename: String()
	/// * min_probablity: (float)0.5
	#[inline]
	pub fn er_grouping_1(image: &impl core::ToInputArray, channel: &impl core::ToInputArray, mut regions: core::Vector<core::Vector<core::Point>>, groups_rects: &mut core::Vector<core::Rect>, method: i32, filename: &str, min_probablity: f32) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(channel);
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_erGrouping_const__InputArrayR_const__InputArrayR_vectorLvectorLPointGG_vectorLRectGR_int_const_StringR_float(image.as_raw__InputArray(), channel.as_raw__InputArray(), regions.as_raw_mut_VectorOfVectorOfPoint(), groups_rects.as_raw_mut_VectorOfRect(), method, filename.opencv_as_extern(), min_probablity, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Allow to implicitly load the default classifier when creating an ERFilter object.
	/// 
	/// ## Parameters
	/// * filename: The XML or YAML file with the classifier model (e.g. trained_classifierNM1.xml)
	/// 
	/// returns a pointer to ERFilter::Callback.
	#[inline]
	pub fn load_classifier_nm1(filename: &str) -> Result<core::Ptr<crate::text::ERFilter_Callback>> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_loadClassifierNM1_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::text::ERFilter_Callback>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Allow to implicitly load the default classifier when creating an ERFilter object.
	/// 
	/// ## Parameters
	/// * filename: The XML or YAML file with the classifier model (e.g. trained_classifierNM2.xml)
	/// 
	/// returns a pointer to ERFilter::Callback.
	#[inline]
	pub fn load_classifier_nm2(filename: &str) -> Result<core::Ptr<crate::text::ERFilter_Callback>> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_loadClassifierNM2_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::text::ERFilter_Callback>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Allow to implicitly load the default character classifier when creating an OCRBeamSearchDecoder object.
	/// 
	/// ## Parameters
	/// * filename: The XML or YAML file with the classifier model (e.g. OCRBeamSearch_CNN_model_data.xml.gz)
	/// 
	/// The CNN default classifier is based in the scene text recognition method proposed by Adam Coates &
	/// Andrew NG in [Coates11a]. The character classifier consists in a Single Layer Convolutional Neural Network and
	/// a linear classifier. It is applied to the input image in a sliding window fashion, providing a set of recognitions
	/// at each window location.
	#[inline]
	pub fn load_ocr_beam_search_classifier_cnn(filename: &str) -> Result<core::Ptr<crate::text::OCRBeamSearchDecoder_ClassifierCallback>> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_loadOCRBeamSearchClassifierCNN_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::text::OCRBeamSearchDecoder_ClassifierCallback>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Allow to implicitly load the default character classifier when creating an OCRHMMDecoder object.
	/// 
	/// ## Parameters
	/// * filename: The XML or YAML file with the classifier model (e.g. OCRBeamSearch_CNN_model_data.xml.gz)
	/// 
	/// The CNN default classifier is based in the scene text recognition method proposed by Adam Coates &
	/// Andrew NG in [Coates11a]. The character classifier consists in a Single Layer Convolutional Neural Network and
	/// a linear classifier. It is applied to the input image in a sliding window fashion, providing a set of recognitions
	/// at each window location.
	/// 
	/// 
	/// **Deprecated**: use loadOCRHMMClassifier instead
	#[deprecated = "use loadOCRHMMClassifier instead"]
	#[inline]
	pub fn load_ocrhmm_classifier_cnn(filename: &str) -> Result<core::Ptr<crate::text::OCRHMMDecoder_ClassifierCallback>> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_loadOCRHMMClassifierCNN_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::text::OCRHMMDecoder_ClassifierCallback>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Allow to implicitly load the default character classifier when creating an OCRHMMDecoder object.
	/// 
	/// ## Parameters
	/// * filename: The XML or YAML file with the classifier model (e.g. OCRHMM_knn_model_data.xml)
	/// 
	/// The KNN default classifier is based in the scene text recognition method proposed by Lukás Neumann &
	/// Jiri Matas in [Neumann11b]. Basically, the region (contour) in the input image is normalized to a
	/// fixed size, while retaining the centroid and aspect ratio, in order to extract a feature vector
	/// based on gradient orientations along the chain-code of its perimeter. Then, the region is classified
	/// using a KNN model trained with synthetic data of rendered characters with different standard font
	/// types.
	/// 
	/// 
	/// **Deprecated**: loadOCRHMMClassifier instead
	#[deprecated = "loadOCRHMMClassifier instead"]
	#[inline]
	pub fn load_ocrhmm_classifier_nm(filename: &str) -> Result<core::Ptr<crate::text::OCRHMMDecoder_ClassifierCallback>> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_loadOCRHMMClassifierNM_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::text::OCRHMMDecoder_ClassifierCallback>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Allow to implicitly load the default character classifier when creating an OCRHMMDecoder object.
	/// 
	/// ## Parameters
	/// * filename: The XML or YAML file with the classifier model (e.g. OCRBeamSearch_CNN_model_data.xml.gz)
	/// 
	/// * classifier: Can be one of classifier_type enum values.
	#[inline]
	pub fn load_ocrhmm_classifier(filename: &str, classifier: i32) -> Result<core::Ptr<crate::text::OCRHMMDecoder_ClassifierCallback>> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_text_loadOCRHMMClassifier_const_StringR_int(filename.opencv_as_extern(), classifier, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::text::OCRHMMDecoder_ClassifierCallback>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Constant methods for [crate::text::BaseOCR]
	pub trait BaseOCRTraitConst {
		fn as_raw_BaseOCR(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::text::BaseOCR]
	pub trait BaseOCRTrait: crate::text::BaseOCRTraitConst {
		fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void;
	
		/// ## C++ default parameters
		/// * component_rects: NULL
		/// * component_texts: NULL
		/// * component_confidences: NULL
		/// * component_level: 0
		#[inline]
		fn run(&mut self, image: &mut core::Mat, output_text: &mut String, component_rects: &mut core::Vector<core::Rect>, component_texts: &mut core::Vector<String>, component_confidences: &mut core::Vector<f32>, component_level: i32) -> Result<()> {
			string_arg_output_send!(via output_text_via);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_BaseOCR_run_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(self.as_raw_mut_BaseOCR(), image.as_raw_mut_Mat(), &mut output_text_via, component_rects.as_raw_mut_VectorOfRect(), component_texts.as_raw_mut_VectorOfString(), component_confidences.as_raw_mut_VectorOff32(), component_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			string_arg_output_receive!(output_text_via => output_text);
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [run] function uses the following default values for its arguments:
		/// * component_rects: NULL
		/// * component_texts: NULL
		/// * component_confidences: NULL
		/// * component_level: 0
		#[inline]
		fn run_def(&mut self, image: &mut core::Mat, output_text: &mut String) -> Result<()> {
			string_arg_output_send!(via output_text_via);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_BaseOCR_run_MatR_stringR(self.as_raw_mut_BaseOCR(), image.as_raw_mut_Mat(), &mut output_text_via, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			string_arg_output_receive!(output_text_via => output_text);
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * component_rects: NULL
		/// * component_texts: NULL
		/// * component_confidences: NULL
		/// * component_level: 0
		#[inline]
		fn run_mask(&mut self, image: &mut core::Mat, mask: &mut core::Mat, output_text: &mut String, component_rects: &mut core::Vector<core::Rect>, component_texts: &mut core::Vector<String>, component_confidences: &mut core::Vector<f32>, component_level: i32) -> Result<()> {
			string_arg_output_send!(via output_text_via);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_BaseOCR_run_MatR_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(self.as_raw_mut_BaseOCR(), image.as_raw_mut_Mat(), mask.as_raw_mut_Mat(), &mut output_text_via, component_rects.as_raw_mut_VectorOfRect(), component_texts.as_raw_mut_VectorOfString(), component_confidences.as_raw_mut_VectorOff32(), component_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			string_arg_output_receive!(output_text_via => output_text);
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [run_mask] function uses the following default values for its arguments:
		/// * component_rects: NULL
		/// * component_texts: NULL
		/// * component_confidences: NULL
		/// * component_level: 0
		#[inline]
		fn run_mask_def(&mut self, image: &mut core::Mat, mask: &mut core::Mat, output_text: &mut String) -> Result<()> {
			string_arg_output_send!(via output_text_via);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_BaseOCR_run_MatR_MatR_stringR(self.as_raw_mut_BaseOCR(), image.as_raw_mut_Mat(), mask.as_raw_mut_Mat(), &mut output_text_via, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			string_arg_output_receive!(output_text_via => output_text);
			Ok(ret)
		}
		
	}
	
	pub struct BaseOCR {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { BaseOCR }
	
	impl Drop for BaseOCR {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_text_BaseOCR_delete(self.as_raw_mut_BaseOCR()) };
		}
	}
	
	unsafe impl Send for BaseOCR {}
	
	impl crate::text::BaseOCRTraitConst for BaseOCR {
		#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::text::BaseOCRTrait for BaseOCR {
		#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl BaseOCR {
	}
	
	boxed_cast_descendant! { BaseOCR, crate::text::OCRBeamSearchDecoder, cv_text_BaseOCR_to_OCRBeamSearchDecoder }
	
	boxed_cast_descendant! { BaseOCR, crate::text::OCRHMMDecoder, cv_text_BaseOCR_to_OCRHMMDecoder }
	
	boxed_cast_descendant! { BaseOCR, crate::text::OCRHolisticWordRecognizer, cv_text_BaseOCR_to_OCRHolisticWordRecognizer }
	
	boxed_cast_descendant! { BaseOCR, crate::text::OCRTesseract, cv_text_BaseOCR_to_OCRTesseract }
	
	impl std::fmt::Debug for BaseOCR {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("BaseOCR")
				.finish()
		}
	}
	
	/// Constant methods for [crate::text::ERFilter]
	pub trait ERFilterTraitConst: core::AlgorithmTraitConst {
		fn as_raw_ERFilter(&self) -> *const c_void;
	
		#[inline]
		fn get_num_rejected(&self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_ERFilter_getNumRejected_const(self.as_raw_ERFilter(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::text::ERFilter]
	pub trait ERFilterTrait: core::AlgorithmTrait + crate::text::ERFilterTraitConst {
		fn as_raw_mut_ERFilter(&mut self) -> *mut c_void;
	
		/// The key method of ERFilter algorithm.
		/// 
		/// Takes image on input and returns the selected regions in a vector of ERStat only distinctive
		/// ERs which correspond to characters are selected by a sequential classifier
		/// 
		/// ## Parameters
		/// * image: Single channel image CV_8UC1
		/// 
		/// * regions: Output for the 1st stage and Input/Output for the 2nd. The selected Extremal Regions
		/// are stored here.
		/// 
		/// Extracts the component tree (if needed) and filter the extremal regions (ER's) by using a given
		/// classifier.
		#[inline]
		fn run(&mut self, image: &impl core::ToInputArray, regions: &mut core::Vector<crate::text::ERStat>) -> Result<()> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_ERFilter_run_const__InputArrayR_vectorLERStatGR(self.as_raw_mut_ERFilter(), image.as_raw__InputArray(), regions.as_raw_mut_VectorOfERStat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		/// set/get methods to set the algorithm properties,
		#[inline]
		fn set_callback(&mut self, cb: &core::Ptr<crate::text::ERFilter_Callback>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_ERFilter_setCallback_const_PtrLCallbackGR(self.as_raw_mut_ERFilter(), cb.as_raw_PtrOfERFilter_Callback(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_threshold_delta(&mut self, threshold_delta: i32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_ERFilter_setThresholdDelta_int(self.as_raw_mut_ERFilter(), threshold_delta, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_min_area(&mut self, min_area: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_ERFilter_setMinArea_float(self.as_raw_mut_ERFilter(), min_area, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_max_area(&mut self, max_area: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_ERFilter_setMaxArea_float(self.as_raw_mut_ERFilter(), max_area, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_min_probability(&mut self, min_probability: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_ERFilter_setMinProbability_float(self.as_raw_mut_ERFilter(), min_probability, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_min_probability_diff(&mut self, min_probability_diff: f32) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_ERFilter_setMinProbabilityDiff_float(self.as_raw_mut_ERFilter(), min_probability_diff, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn set_non_max_suppression(&mut self, non_max_suppression: bool) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_ERFilter_setNonMaxSuppression_bool(self.as_raw_mut_ERFilter(), non_max_suppression, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Base class for 1st and 2nd stages of Neumann and Matas scene text detection algorithm [Neumann12](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Neumann12). :
	/// 
	/// Extracts the component tree (if needed) and filter the extremal regions (ER's) by using a given classifier.
	pub struct ERFilter {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ERFilter }
	
	impl Drop for ERFilter {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_text_ERFilter_delete(self.as_raw_mut_ERFilter()) };
		}
	}
	
	unsafe impl Send for ERFilter {}
	
	impl core::AlgorithmTraitConst for ERFilter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ERFilter {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::ERFilterTraitConst for ERFilter {
		#[inline] fn as_raw_ERFilter(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::text::ERFilterTrait for ERFilter {
		#[inline] fn as_raw_mut_ERFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ERFilter {
	}
	
	boxed_cast_base! { ERFilter, core::Algorithm, cv_text_ERFilter_to_Algorithm }
	
	impl std::fmt::Debug for ERFilter {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ERFilter")
				.finish()
		}
	}
	
	/// Constant methods for [crate::text::ERFilter_Callback]
	pub trait ERFilter_CallbackTraitConst {
		fn as_raw_ERFilter_Callback(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::text::ERFilter_Callback]
	pub trait ERFilter_CallbackTrait: crate::text::ERFilter_CallbackTraitConst {
		fn as_raw_mut_ERFilter_Callback(&mut self) -> *mut c_void;
	
		/// The classifier must return probability measure for the region.
		/// 
		/// ## Parameters
		/// * stat: :   The region to be classified
		#[inline]
		fn eval(&mut self, stat: &crate::text::ERStat) -> Result<f64> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_ERFilter_Callback_eval_const_ERStatR(self.as_raw_mut_ERFilter_Callback(), stat.as_raw_ERStat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Callback with the classifier is made a class.
	/// 
	/// By doing it we hide SVM, Boost etc. Developers can provide their own classifiers to the
	/// ERFilter algorithm.
	pub struct ERFilter_Callback {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ERFilter_Callback }
	
	impl Drop for ERFilter_Callback {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_text_ERFilter_Callback_delete(self.as_raw_mut_ERFilter_Callback()) };
		}
	}
	
	unsafe impl Send for ERFilter_Callback {}
	
	impl crate::text::ERFilter_CallbackTraitConst for ERFilter_Callback {
		#[inline] fn as_raw_ERFilter_Callback(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::text::ERFilter_CallbackTrait for ERFilter_Callback {
		#[inline] fn as_raw_mut_ERFilter_Callback(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ERFilter_Callback {
	}
	
	impl std::fmt::Debug for ERFilter_Callback {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ERFilter_Callback")
				.finish()
		}
	}
	
	/// Constant methods for [crate::text::ERStat]
	pub trait ERStatTraitConst {
		fn as_raw_ERStat(&self) -> *const c_void;
	
		/// seed point and the threshold (max grey-level value)
		#[inline]
		fn pixel(&self) -> i32 {
			let ret = unsafe { sys::cv_text_ERStat_propPixel_const(self.as_raw_ERStat()) };
			ret
		}
		
		#[inline]
		fn level(&self) -> i32 {
			let ret = unsafe { sys::cv_text_ERStat_propLevel_const(self.as_raw_ERStat()) };
			ret
		}
		
		/// incrementally computable features
		#[inline]
		fn area(&self) -> i32 {
			let ret = unsafe { sys::cv_text_ERStat_propArea_const(self.as_raw_ERStat()) };
			ret
		}
		
		#[inline]
		fn perimeter(&self) -> i32 {
			let ret = unsafe { sys::cv_text_ERStat_propPerimeter_const(self.as_raw_ERStat()) };
			ret
		}
		
		/// Euler's number
		#[inline]
		fn euler(&self) -> i32 {
			let ret = unsafe { sys::cv_text_ERStat_propEuler_const(self.as_raw_ERStat()) };
			ret
		}
		
		#[inline]
		fn rect(&self) -> core::Rect {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_ERStat_propRect_const(self.as_raw_ERStat(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			ret
		}
		
		/// median of the crossings at three different height levels
		#[inline]
		fn med_crossings(&self) -> f32 {
			let ret = unsafe { sys::cv_text_ERStat_propMed_crossings_const(self.as_raw_ERStat()) };
			ret
		}
		
		/// 2nd stage features
		#[inline]
		fn hole_area_ratio(&self) -> f32 {
			let ret = unsafe { sys::cv_text_ERStat_propHole_area_ratio_const(self.as_raw_ERStat()) };
			ret
		}
		
		#[inline]
		fn convex_hull_ratio(&self) -> f32 {
			let ret = unsafe { sys::cv_text_ERStat_propConvex_hull_ratio_const(self.as_raw_ERStat()) };
			ret
		}
		
		#[inline]
		fn num_inflexion_points(&self) -> f32 {
			let ret = unsafe { sys::cv_text_ERStat_propNum_inflexion_points_const(self.as_raw_ERStat()) };
			ret
		}
		
		/// probability that the ER belongs to the class we are looking for
		#[inline]
		fn probability(&self) -> f64 {
			let ret = unsafe { sys::cv_text_ERStat_propProbability_const(self.as_raw_ERStat()) };
			ret
		}
		
		/// whenever the regions is a local maxima of the probability
		#[inline]
		fn local_maxima(&self) -> bool {
			let ret = unsafe { sys::cv_text_ERStat_propLocal_maxima_const(self.as_raw_ERStat()) };
			ret
		}
		
	}
	
	/// Mutable methods for [crate::text::ERStat]
	pub trait ERStatTrait: crate::text::ERStatTraitConst {
		fn as_raw_mut_ERStat(&mut self) -> *mut c_void;
	
		/// seed point and the threshold (max grey-level value)
		#[inline]
		fn set_pixel(&mut self, val: i32) {
			let ret = unsafe { sys::cv_text_ERStat_propPixel_int(self.as_raw_mut_ERStat(), val) };
			ret
		}
		
		#[inline]
		fn set_level(&mut self, val: i32) {
			let ret = unsafe { sys::cv_text_ERStat_propLevel_int(self.as_raw_mut_ERStat(), val) };
			ret
		}
		
		/// incrementally computable features
		#[inline]
		fn set_area(&mut self, val: i32) {
			let ret = unsafe { sys::cv_text_ERStat_propArea_int(self.as_raw_mut_ERStat(), val) };
			ret
		}
		
		#[inline]
		fn set_perimeter(&mut self, val: i32) {
			let ret = unsafe { sys::cv_text_ERStat_propPerimeter_int(self.as_raw_mut_ERStat(), val) };
			ret
		}
		
		/// Euler's number
		#[inline]
		fn set_euler(&mut self, val: i32) {
			let ret = unsafe { sys::cv_text_ERStat_propEuler_int(self.as_raw_mut_ERStat(), val) };
			ret
		}
		
		#[inline]
		fn set_rect(&mut self, val: core::Rect) {
			let ret = unsafe { sys::cv_text_ERStat_propRect_Rect(self.as_raw_mut_ERStat(), val.opencv_as_extern()) };
			ret
		}
		
		/// order 1 raw moments to derive the centroid
		#[inline]
		fn raw_moments(&mut self) -> &mut [f64; 2] {
			let ret = unsafe { sys::cv_text_ERStat_propRaw_moments(self.as_raw_mut_ERStat()) };
			let ret = unsafe { ret.as_mut() }.expect("Function returned null pointer");
			ret
		}
		
		/// order 2 central moments to construct the covariance matrix
		#[inline]
		fn central_moments(&mut self) -> &mut [f64; 3] {
			let ret = unsafe { sys::cv_text_ERStat_propCentral_moments(self.as_raw_mut_ERStat()) };
			let ret = unsafe { ret.as_mut() }.expect("Function returned null pointer");
			ret
		}
		
		/// median of the crossings at three different height levels
		#[inline]
		fn set_med_crossings(&mut self, val: f32) {
			let ret = unsafe { sys::cv_text_ERStat_propMed_crossings_float(self.as_raw_mut_ERStat(), val) };
			ret
		}
		
		/// 2nd stage features
		#[inline]
		fn set_hole_area_ratio(&mut self, val: f32) {
			let ret = unsafe { sys::cv_text_ERStat_propHole_area_ratio_float(self.as_raw_mut_ERStat(), val) };
			ret
		}
		
		#[inline]
		fn set_convex_hull_ratio(&mut self, val: f32) {
			let ret = unsafe { sys::cv_text_ERStat_propConvex_hull_ratio_float(self.as_raw_mut_ERStat(), val) };
			ret
		}
		
		#[inline]
		fn set_num_inflexion_points(&mut self, val: f32) {
			let ret = unsafe { sys::cv_text_ERStat_propNum_inflexion_points_float(self.as_raw_mut_ERStat(), val) };
			ret
		}
		
		/// probability that the ER belongs to the class we are looking for
		#[inline]
		fn set_probability(&mut self, val: f64) {
			let ret = unsafe { sys::cv_text_ERStat_propProbability_double(self.as_raw_mut_ERStat(), val) };
			ret
		}
		
		/// pointers preserving the tree structure of the component tree
		#[inline]
		fn parent(&mut self) -> crate::text::ERStat {
			let ret = unsafe { sys::cv_text_ERStat_propParent(self.as_raw_mut_ERStat()) };
			let ret = unsafe { crate::text::ERStat::opencv_from_extern(ret) };
			ret
		}
		
		/// pointers preserving the tree structure of the component tree
		#[inline]
		fn set_parent(&mut self, val: &mut crate::text::ERStat) {
			let ret = unsafe { sys::cv_text_ERStat_propParent_ERStatX(self.as_raw_mut_ERStat(), val.as_raw_mut_ERStat()) };
			ret
		}
		
		#[inline]
		fn child(&mut self) -> crate::text::ERStat {
			let ret = unsafe { sys::cv_text_ERStat_propChild(self.as_raw_mut_ERStat()) };
			let ret = unsafe { crate::text::ERStat::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn set_child(&mut self, val: &mut crate::text::ERStat) {
			let ret = unsafe { sys::cv_text_ERStat_propChild_ERStatX(self.as_raw_mut_ERStat(), val.as_raw_mut_ERStat()) };
			ret
		}
		
		#[inline]
		fn next(&mut self) -> crate::text::ERStat {
			let ret = unsafe { sys::cv_text_ERStat_propNext(self.as_raw_mut_ERStat()) };
			let ret = unsafe { crate::text::ERStat::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn set_next(&mut self, val: &mut crate::text::ERStat) {
			let ret = unsafe { sys::cv_text_ERStat_propNext_ERStatX(self.as_raw_mut_ERStat(), val.as_raw_mut_ERStat()) };
			ret
		}
		
		#[inline]
		fn prev(&mut self) -> crate::text::ERStat {
			let ret = unsafe { sys::cv_text_ERStat_propPrev(self.as_raw_mut_ERStat()) };
			let ret = unsafe { crate::text::ERStat::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn set_prev(&mut self, val: &mut crate::text::ERStat) {
			let ret = unsafe { sys::cv_text_ERStat_propPrev_ERStatX(self.as_raw_mut_ERStat(), val.as_raw_mut_ERStat()) };
			ret
		}
		
		/// whenever the regions is a local maxima of the probability
		#[inline]
		fn set_local_maxima(&mut self, val: bool) {
			let ret = unsafe { sys::cv_text_ERStat_propLocal_maxima_bool(self.as_raw_mut_ERStat(), val) };
			ret
		}
		
		#[inline]
		fn max_probability_ancestor(&mut self) -> crate::text::ERStat {
			let ret = unsafe { sys::cv_text_ERStat_propMax_probability_ancestor(self.as_raw_mut_ERStat()) };
			let ret = unsafe { crate::text::ERStat::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn set_max_probability_ancestor(&mut self, val: &mut crate::text::ERStat) {
			let ret = unsafe { sys::cv_text_ERStat_propMax_probability_ancestor_ERStatX(self.as_raw_mut_ERStat(), val.as_raw_mut_ERStat()) };
			ret
		}
		
		#[inline]
		fn min_probability_ancestor(&mut self) -> crate::text::ERStat {
			let ret = unsafe { sys::cv_text_ERStat_propMin_probability_ancestor(self.as_raw_mut_ERStat()) };
			let ret = unsafe { crate::text::ERStat::opencv_from_extern(ret) };
			ret
		}
		
		#[inline]
		fn set_min_probability_ancestor(&mut self, val: &mut crate::text::ERStat) {
			let ret = unsafe { sys::cv_text_ERStat_propMin_probability_ancestor_ERStatX(self.as_raw_mut_ERStat(), val.as_raw_mut_ERStat()) };
			ret
		}
		
	}
	
	/// The ERStat structure represents a class-specific Extremal Region (ER).
	/// 
	/// An ER is a 4-connected set of pixels with all its grey-level values smaller than the values in its
	/// outer boundary. A class-specific ER is selected (using a classifier) from all the ER's in the
	/// component tree of the image. :
	pub struct ERStat {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ERStat }
	
	impl Drop for ERStat {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_text_ERStat_delete(self.as_raw_mut_ERStat()) };
		}
	}
	
	unsafe impl Send for ERStat {}
	
	impl crate::text::ERStatTraitConst for ERStat {
		#[inline] fn as_raw_ERStat(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::text::ERStatTrait for ERStat {
		#[inline] fn as_raw_mut_ERStat(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ERStat {
		/// Constructor
		/// 
		/// ## C++ default parameters
		/// * level: 256
		/// * pixel: 0
		/// * x: 0
		/// * y: 0
		#[inline]
		pub fn new(level: i32, pixel: i32, x: i32, y: i32) -> Result<crate::text::ERStat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_ERStat_ERStat_int_int_int_int(level, pixel, x, y, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::text::ERStat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Constructor
		/// 
		/// ## Note
		/// This alternative version of [new] function uses the following default values for its arguments:
		/// * level: 256
		/// * pixel: 0
		/// * x: 0
		/// * y: 0
		#[inline]
		pub fn new_def() -> Result<crate::text::ERStat> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_ERStat_ERStat(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { crate::text::ERStat::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	impl std::fmt::Debug for ERStat {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ERStat")
				.field("pixel", &crate::text::ERStatTraitConst::pixel(self))
				.field("level", &crate::text::ERStatTraitConst::level(self))
				.field("area", &crate::text::ERStatTraitConst::area(self))
				.field("perimeter", &crate::text::ERStatTraitConst::perimeter(self))
				.field("euler", &crate::text::ERStatTraitConst::euler(self))
				.field("rect", &crate::text::ERStatTraitConst::rect(self))
				.field("med_crossings", &crate::text::ERStatTraitConst::med_crossings(self))
				.field("hole_area_ratio", &crate::text::ERStatTraitConst::hole_area_ratio(self))
				.field("convex_hull_ratio", &crate::text::ERStatTraitConst::convex_hull_ratio(self))
				.field("num_inflexion_points", &crate::text::ERStatTraitConst::num_inflexion_points(self))
				.field("probability", &crate::text::ERStatTraitConst::probability(self))
				.field("local_maxima", &crate::text::ERStatTraitConst::local_maxima(self))
				.finish()
		}
	}
	
	/// Constant methods for [crate::text::OCRBeamSearchDecoder]
	pub trait OCRBeamSearchDecoderTraitConst: crate::text::BaseOCRTraitConst {
		fn as_raw_OCRBeamSearchDecoder(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::text::OCRBeamSearchDecoder]
	pub trait OCRBeamSearchDecoderTrait: crate::text::BaseOCRTrait + crate::text::OCRBeamSearchDecoderTraitConst {
		fn as_raw_mut_OCRBeamSearchDecoder(&mut self) -> *mut c_void;
	
		/// Recognize text using Beam Search.
		/// 
		/// Takes image on input and returns recognized text in the output_text parameter. Optionally
		/// provides also the Rects for individual text elements found (e.g. words), and the list of those
		/// text elements with their confidence values.
		/// 
		/// ## Parameters
		/// * image: Input binary image CV_8UC1 with a single text line (or word).
		/// 
		/// * output_text: Output text. Most likely character sequence found by the HMM decoder.
		/// 
		/// * component_rects: If provided the method will output a list of Rects for the individual
		/// text elements found (e.g. words).
		/// 
		/// * component_texts: If provided the method will output a list of text strings for the
		/// recognition of individual text elements found (e.g. words).
		/// 
		/// * component_confidences: If provided the method will output a list of confidence values
		/// for the recognition of individual text elements found (e.g. words).
		/// 
		/// * component_level: Only OCR_LEVEL_WORD is supported.
		/// 
		/// ## C++ default parameters
		/// * component_rects: NULL
		/// * component_texts: NULL
		/// * component_confidences: NULL
		/// * component_level: 0
		#[inline]
		fn run_multiple(&mut self, image: &mut core::Mat, output_text: &mut String, component_rects: &mut core::Vector<core::Rect>, component_texts: &mut core::Vector<String>, component_confidences: &mut core::Vector<f32>, component_level: i32) -> Result<()> {
			string_arg_output_send!(via output_text_via);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRBeamSearchDecoder_run_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(self.as_raw_mut_OCRBeamSearchDecoder(), image.as_raw_mut_Mat(), &mut output_text_via, component_rects.as_raw_mut_VectorOfRect(), component_texts.as_raw_mut_VectorOfString(), component_confidences.as_raw_mut_VectorOff32(), component_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			string_arg_output_receive!(output_text_via => output_text);
			Ok(ret)
		}
		
		/// Recognize text using Beam Search.
		/// 
		/// Takes image on input and returns recognized text in the output_text parameter. Optionally
		/// provides also the Rects for individual text elements found (e.g. words), and the list of those
		/// text elements with their confidence values.
		/// 
		/// ## Parameters
		/// * image: Input binary image CV_8UC1 with a single text line (or word).
		/// 
		/// * output_text: Output text. Most likely character sequence found by the HMM decoder.
		/// 
		/// * component_rects: If provided the method will output a list of Rects for the individual
		/// text elements found (e.g. words).
		/// 
		/// * component_texts: If provided the method will output a list of text strings for the
		/// recognition of individual text elements found (e.g. words).
		/// 
		/// * component_confidences: If provided the method will output a list of confidence values
		/// for the recognition of individual text elements found (e.g. words).
		/// 
		/// * component_level: Only OCR_LEVEL_WORD is supported.
		/// 
		/// ## Note
		/// This alternative version of [run_multiple] function uses the following default values for its arguments:
		/// * component_rects: NULL
		/// * component_texts: NULL
		/// * component_confidences: NULL
		/// * component_level: 0
		#[inline]
		fn run_multiple_def(&mut self, image: &mut core::Mat, output_text: &mut String) -> Result<()> {
			string_arg_output_send!(via output_text_via);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRBeamSearchDecoder_run_MatR_stringR(self.as_raw_mut_OCRBeamSearchDecoder(), image.as_raw_mut_Mat(), &mut output_text_via, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			string_arg_output_receive!(output_text_via => output_text);
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * component_rects: NULL
		/// * component_texts: NULL
		/// * component_confidences: NULL
		/// * component_level: 0
		#[inline]
		fn run_multiple_mask(&mut self, image: &mut core::Mat, mask: &mut core::Mat, output_text: &mut String, component_rects: &mut core::Vector<core::Rect>, component_texts: &mut core::Vector<String>, component_confidences: &mut core::Vector<f32>, component_level: i32) -> Result<()> {
			string_arg_output_send!(via output_text_via);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRBeamSearchDecoder_run_MatR_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(self.as_raw_mut_OCRBeamSearchDecoder(), image.as_raw_mut_Mat(), mask.as_raw_mut_Mat(), &mut output_text_via, component_rects.as_raw_mut_VectorOfRect(), component_texts.as_raw_mut_VectorOfString(), component_confidences.as_raw_mut_VectorOff32(), component_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			string_arg_output_receive!(output_text_via => output_text);
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [run_multiple_mask] function uses the following default values for its arguments:
		/// * component_rects: NULL
		/// * component_texts: NULL
		/// * component_confidences: NULL
		/// * component_level: 0
		#[inline]
		fn run_multiple_mask_def(&mut self, image: &mut core::Mat, mask: &mut core::Mat, output_text: &mut String) -> Result<()> {
			string_arg_output_send!(via output_text_via);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRBeamSearchDecoder_run_MatR_MatR_stringR(self.as_raw_mut_OCRBeamSearchDecoder(), image.as_raw_mut_Mat(), mask.as_raw_mut_Mat(), &mut output_text_via, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			string_arg_output_receive!(output_text_via => output_text);
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * component_level: 0
		#[inline]
		fn run(&mut self, image: &impl core::ToInputArray, min_confidence: i32, component_level: i32) -> Result<String> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRBeamSearchDecoder_run_const__InputArrayR_int_int(self.as_raw_mut_OCRBeamSearchDecoder(), image.as_raw__InputArray(), min_confidence, component_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [run] function uses the following default values for its arguments:
		/// * component_level: 0
		#[inline]
		fn run_def(&mut self, image: &impl core::ToInputArray, min_confidence: i32) -> Result<String> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRBeamSearchDecoder_run_const__InputArrayR_int(self.as_raw_mut_OCRBeamSearchDecoder(), image.as_raw__InputArray(), min_confidence, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * component_level: 0
		#[inline]
		fn run_mask(&mut self, image: &impl core::ToInputArray, mask: &impl core::ToInputArray, min_confidence: i32, component_level: i32) -> Result<String> {
			input_array_arg!(image);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRBeamSearchDecoder_run_const__InputArrayR_const__InputArrayR_int_int(self.as_raw_mut_OCRBeamSearchDecoder(), image.as_raw__InputArray(), mask.as_raw__InputArray(), min_confidence, component_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [run_mask] function uses the following default values for its arguments:
		/// * component_level: 0
		#[inline]
		fn run_mask_def(&mut self, image: &impl core::ToInputArray, mask: &impl core::ToInputArray, min_confidence: i32) -> Result<String> {
			input_array_arg!(image);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRBeamSearchDecoder_run_const__InputArrayR_const__InputArrayR_int(self.as_raw_mut_OCRBeamSearchDecoder(), image.as_raw__InputArray(), mask.as_raw__InputArray(), min_confidence, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// OCRBeamSearchDecoder class provides an interface for OCR using Beam Search algorithm.
	/// 
	/// 
	/// Note:
	///    *   (C++) An example on using OCRBeamSearchDecoder recognition combined with scene text detection can
	///        be found at the demo sample:
	///        <https://github.com/opencv/opencv_contrib/blob/master/modules/text/samples/word_recognition.cpp>
	pub struct OCRBeamSearchDecoder {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { OCRBeamSearchDecoder }
	
	impl Drop for OCRBeamSearchDecoder {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_text_OCRBeamSearchDecoder_delete(self.as_raw_mut_OCRBeamSearchDecoder()) };
		}
	}
	
	unsafe impl Send for OCRBeamSearchDecoder {}
	
	impl crate::text::BaseOCRTraitConst for OCRBeamSearchDecoder {
		#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::text::BaseOCRTrait for OCRBeamSearchDecoder {
		#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::OCRBeamSearchDecoderTraitConst for OCRBeamSearchDecoder {
		#[inline] fn as_raw_OCRBeamSearchDecoder(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::text::OCRBeamSearchDecoderTrait for OCRBeamSearchDecoder {
		#[inline] fn as_raw_mut_OCRBeamSearchDecoder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl OCRBeamSearchDecoder {
		/// Creates an instance of the OCRBeamSearchDecoder class. Initializes HMMDecoder.
		/// 
		/// ## Parameters
		/// * classifier: The character classifier with built in feature extractor.
		/// 
		/// * vocabulary: The language vocabulary (chars when ASCII English text). vocabulary.size()
		/// must be equal to the number of classes of the classifier.
		/// 
		/// * transition_probabilities_table: Table with transition probabilities between character
		/// pairs. cols == rows == vocabulary.size().
		/// 
		/// * emission_probabilities_table: Table with observation emission probabilities. cols ==
		/// rows == vocabulary.size().
		/// 
		/// * mode: HMM Decoding algorithm. Only OCR_DECODER_VITERBI is available for the moment
		/// (<http://en.wikipedia.org/wiki/Viterbi_algorithm>).
		/// 
		/// * beam_size: Size of the beam in Beam Search algorithm.
		/// 
		/// ## C++ default parameters
		/// * mode: OCR_DECODER_VITERBI
		/// * beam_size: 500
		#[inline]
		pub fn create(classifier: core::Ptr<crate::text::OCRBeamSearchDecoder_ClassifierCallback>, vocabulary: &str, transition_probabilities_table: &impl core::ToInputArray, emission_probabilities_table: &impl core::ToInputArray, mode: crate::text::decoder_mode, beam_size: i32) -> Result<core::Ptr<crate::text::OCRBeamSearchDecoder>> {
			extern_container_arg!(vocabulary);
			input_array_arg!(transition_probabilities_table);
			input_array_arg!(emission_probabilities_table);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRBeamSearchDecoder_create_const_PtrLClassifierCallbackG_const_stringR_const__InputArrayR_const__InputArrayR_decoder_mode_int(classifier.as_raw_PtrOfOCRBeamSearchDecoder_ClassifierCallback(), vocabulary.opencv_as_extern(), transition_probabilities_table.as_raw__InputArray(), emission_probabilities_table.as_raw__InputArray(), mode, beam_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::text::OCRBeamSearchDecoder>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Creates an instance of the OCRBeamSearchDecoder class. Initializes HMMDecoder.
		/// 
		/// ## Parameters
		/// * classifier: The character classifier with built in feature extractor.
		/// 
		/// * vocabulary: The language vocabulary (chars when ASCII English text). vocabulary.size()
		/// must be equal to the number of classes of the classifier.
		/// 
		/// * transition_probabilities_table: Table with transition probabilities between character
		/// pairs. cols == rows == vocabulary.size().
		/// 
		/// * emission_probabilities_table: Table with observation emission probabilities. cols ==
		/// rows == vocabulary.size().
		/// 
		/// * mode: HMM Decoding algorithm. Only OCR_DECODER_VITERBI is available for the moment
		/// (<http://en.wikipedia.org/wiki/Viterbi_algorithm>).
		/// 
		/// * beam_size: Size of the beam in Beam Search algorithm.
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * mode: OCR_DECODER_VITERBI
		/// * beam_size: 500
		#[inline]
		pub fn create_def(classifier: core::Ptr<crate::text::OCRBeamSearchDecoder_ClassifierCallback>, vocabulary: &str, transition_probabilities_table: &impl core::ToInputArray, emission_probabilities_table: &impl core::ToInputArray) -> Result<core::Ptr<crate::text::OCRBeamSearchDecoder>> {
			extern_container_arg!(vocabulary);
			input_array_arg!(transition_probabilities_table);
			input_array_arg!(emission_probabilities_table);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRBeamSearchDecoder_create_const_PtrLClassifierCallbackG_const_stringR_const__InputArrayR_const__InputArrayR(classifier.as_raw_PtrOfOCRBeamSearchDecoder_ClassifierCallback(), vocabulary.opencv_as_extern(), transition_probabilities_table.as_raw__InputArray(), emission_probabilities_table.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::text::OCRBeamSearchDecoder>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Creates an instance of the OCRBeamSearchDecoder class. Initializes HMMDecoder from the specified path.
		/// 
		///    Creates an instance of the OCRBeamSearchDecoder class. Initializes HMMDecoder.
		/// 
		/// ## Parameters
		/// * classifier: The character classifier with built in feature extractor.
		/// 
		/// * vocabulary: The language vocabulary (chars when ASCII English text). vocabulary.size()
		/// must be equal to the number of classes of the classifier.
		/// 
		/// * transition_probabilities_table: Table with transition probabilities between character
		/// pairs. cols == rows == vocabulary.size().
		/// 
		/// * emission_probabilities_table: Table with observation emission probabilities. cols ==
		/// rows == vocabulary.size().
		/// 
		/// * mode: HMM Decoding algorithm. Only OCR_DECODER_VITERBI is available for the moment
		/// (<http://en.wikipedia.org/wiki/Viterbi_algorithm>).
		/// 
		/// * beam_size: Size of the beam in Beam Search algorithm.
		/// 
		/// ## Overloaded parameters
		/// 
		/// ## C++ default parameters
		/// * mode: OCR_DECODER_VITERBI
		/// * beam_size: 500
		#[inline]
		pub fn create_from_file(filename: &str, vocabulary: &str, transition_probabilities_table: &impl core::ToInputArray, emission_probabilities_table: &impl core::ToInputArray, mode: crate::text::decoder_mode, beam_size: i32) -> Result<core::Ptr<crate::text::OCRBeamSearchDecoder>> {
			extern_container_arg!(filename);
			extern_container_arg!(vocabulary);
			input_array_arg!(transition_probabilities_table);
			input_array_arg!(emission_probabilities_table);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRBeamSearchDecoder_create_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR_decoder_mode_int(filename.opencv_as_extern(), vocabulary.opencv_as_extern(), transition_probabilities_table.as_raw__InputArray(), emission_probabilities_table.as_raw__InputArray(), mode, beam_size, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::text::OCRBeamSearchDecoder>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Creates an instance of the OCRBeamSearchDecoder class. Initializes HMMDecoder from the specified path.
		/// 
		/// @overload
		/// 
		/// ## Note
		/// This alternative version of [create_from_file] function uses the following default values for its arguments:
		/// * mode: OCR_DECODER_VITERBI
		/// * beam_size: 500
		#[inline]
		pub fn create_from_file_def(filename: &str, vocabulary: &str, transition_probabilities_table: &impl core::ToInputArray, emission_probabilities_table: &impl core::ToInputArray) -> Result<core::Ptr<crate::text::OCRBeamSearchDecoder>> {
			extern_container_arg!(filename);
			extern_container_arg!(vocabulary);
			input_array_arg!(transition_probabilities_table);
			input_array_arg!(emission_probabilities_table);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRBeamSearchDecoder_create_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR(filename.opencv_as_extern(), vocabulary.opencv_as_extern(), transition_probabilities_table.as_raw__InputArray(), emission_probabilities_table.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::text::OCRBeamSearchDecoder>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { OCRBeamSearchDecoder, crate::text::BaseOCR, cv_text_OCRBeamSearchDecoder_to_BaseOCR }
	
	impl std::fmt::Debug for OCRBeamSearchDecoder {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("OCRBeamSearchDecoder")
				.finish()
		}
	}
	
	/// Constant methods for [crate::text::OCRBeamSearchDecoder_ClassifierCallback]
	pub trait OCRBeamSearchDecoder_ClassifierCallbackTraitConst {
		fn as_raw_OCRBeamSearchDecoder_ClassifierCallback(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::text::OCRBeamSearchDecoder_ClassifierCallback]
	pub trait OCRBeamSearchDecoder_ClassifierCallbackTrait: crate::text::OCRBeamSearchDecoder_ClassifierCallbackTraitConst {
		fn as_raw_mut_OCRBeamSearchDecoder_ClassifierCallback(&mut self) -> *mut c_void;
	
		/// The character classifier must return a (ranked list of) class(es) id('s)
		/// 
		/// ## Parameters
		/// * image: Input image CV_8UC1 or CV_8UC3 with a single letter.
		/// * recognition_probabilities: For each of the N characters found the classifier returns a list with
		/// class probabilities for each class.
		/// * oversegmentation: The classifier returns a list of N+1 character locations' x-coordinates,
		/// including 0 as start-sequence location.
		#[inline]
		fn eval(&mut self, image: &impl core::ToInputArray, recognition_probabilities: &mut core::Vector<core::Vector<f64>>, oversegmentation: &mut core::Vector<i32>) -> Result<()> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRBeamSearchDecoder_ClassifierCallback_eval_const__InputArrayR_vectorLvectorLdoubleGGR_vectorLintGR(self.as_raw_mut_OCRBeamSearchDecoder_ClassifierCallback(), image.as_raw__InputArray(), recognition_probabilities.as_raw_mut_VectorOfVectorOff64(), oversegmentation.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_window_size(&mut self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRBeamSearchDecoder_ClassifierCallback_getWindowSize(self.as_raw_mut_OCRBeamSearchDecoder_ClassifierCallback(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn get_step_size(&mut self) -> Result<i32> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRBeamSearchDecoder_ClassifierCallback_getStepSize(self.as_raw_mut_OCRBeamSearchDecoder_ClassifierCallback(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Callback with the character classifier is made a class.
	/// 
	/// This way it hides the feature extractor and the classifier itself, so developers can write
	/// their own OCR code.
	/// 
	/// The default character classifier and feature extractor can be loaded using the utility function
	/// loadOCRBeamSearchClassifierCNN with all its parameters provided in
	/// <https://github.com/opencv/opencv_contrib/blob/master/modules/text/samples/OCRBeamSearch_CNN_model_data.xml.gz>.
	pub struct OCRBeamSearchDecoder_ClassifierCallback {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { OCRBeamSearchDecoder_ClassifierCallback }
	
	impl Drop for OCRBeamSearchDecoder_ClassifierCallback {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_text_OCRBeamSearchDecoder_ClassifierCallback_delete(self.as_raw_mut_OCRBeamSearchDecoder_ClassifierCallback()) };
		}
	}
	
	unsafe impl Send for OCRBeamSearchDecoder_ClassifierCallback {}
	
	impl crate::text::OCRBeamSearchDecoder_ClassifierCallbackTraitConst for OCRBeamSearchDecoder_ClassifierCallback {
		#[inline] fn as_raw_OCRBeamSearchDecoder_ClassifierCallback(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::text::OCRBeamSearchDecoder_ClassifierCallbackTrait for OCRBeamSearchDecoder_ClassifierCallback {
		#[inline] fn as_raw_mut_OCRBeamSearchDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl OCRBeamSearchDecoder_ClassifierCallback {
	}
	
	impl std::fmt::Debug for OCRBeamSearchDecoder_ClassifierCallback {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("OCRBeamSearchDecoder_ClassifierCallback")
				.finish()
		}
	}
	
	/// Constant methods for [crate::text::OCRHMMDecoder]
	pub trait OCRHMMDecoderTraitConst: crate::text::BaseOCRTraitConst {
		fn as_raw_OCRHMMDecoder(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::text::OCRHMMDecoder]
	pub trait OCRHMMDecoderTrait: crate::text::BaseOCRTrait + crate::text::OCRHMMDecoderTraitConst {
		fn as_raw_mut_OCRHMMDecoder(&mut self) -> *mut c_void;
	
		/// Recognize text using HMM.
		/// 
		/// Takes binary image on input and returns recognized text in the output_text parameter. Optionally
		/// provides also the Rects for individual text elements found (e.g. words), and the list of those
		/// text elements with their confidence values.
		/// 
		/// ## Parameters
		/// * image: Input binary image CV_8UC1 with a single text line (or word).
		/// 
		/// * output_text: Output text. Most likely character sequence found by the HMM decoder.
		/// 
		/// * component_rects: If provided the method will output a list of Rects for the individual
		/// text elements found (e.g. words).
		/// 
		/// * component_texts: If provided the method will output a list of text strings for the
		/// recognition of individual text elements found (e.g. words).
		/// 
		/// * component_confidences: If provided the method will output a list of confidence values
		/// for the recognition of individual text elements found (e.g. words).
		/// 
		/// * component_level: Only OCR_LEVEL_WORD is supported.
		/// 
		/// ## C++ default parameters
		/// * component_rects: NULL
		/// * component_texts: NULL
		/// * component_confidences: NULL
		/// * component_level: 0
		#[inline]
		fn run_multiple(&mut self, image: &mut core::Mat, output_text: &mut String, component_rects: &mut core::Vector<core::Rect>, component_texts: &mut core::Vector<String>, component_confidences: &mut core::Vector<f32>, component_level: i32) -> Result<()> {
			string_arg_output_send!(via output_text_via);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRHMMDecoder_run_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(self.as_raw_mut_OCRHMMDecoder(), image.as_raw_mut_Mat(), &mut output_text_via, component_rects.as_raw_mut_VectorOfRect(), component_texts.as_raw_mut_VectorOfString(), component_confidences.as_raw_mut_VectorOff32(), component_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			string_arg_output_receive!(output_text_via => output_text);
			Ok(ret)
		}
		
		/// Recognize text using HMM.
		/// 
		/// Takes binary image on input and returns recognized text in the output_text parameter. Optionally
		/// provides also the Rects for individual text elements found (e.g. words), and the list of those
		/// text elements with their confidence values.
		/// 
		/// ## Parameters
		/// * image: Input binary image CV_8UC1 with a single text line (or word).
		/// 
		/// * output_text: Output text. Most likely character sequence found by the HMM decoder.
		/// 
		/// * component_rects: If provided the method will output a list of Rects for the individual
		/// text elements found (e.g. words).
		/// 
		/// * component_texts: If provided the method will output a list of text strings for the
		/// recognition of individual text elements found (e.g. words).
		/// 
		/// * component_confidences: If provided the method will output a list of confidence values
		/// for the recognition of individual text elements found (e.g. words).
		/// 
		/// * component_level: Only OCR_LEVEL_WORD is supported.
		/// 
		/// ## Note
		/// This alternative version of [run_multiple] function uses the following default values for its arguments:
		/// * component_rects: NULL
		/// * component_texts: NULL
		/// * component_confidences: NULL
		/// * component_level: 0
		#[inline]
		fn run_multiple_def(&mut self, image: &mut core::Mat, output_text: &mut String) -> Result<()> {
			string_arg_output_send!(via output_text_via);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRHMMDecoder_run_MatR_stringR(self.as_raw_mut_OCRHMMDecoder(), image.as_raw_mut_Mat(), &mut output_text_via, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			string_arg_output_receive!(output_text_via => output_text);
			Ok(ret)
		}
		
		/// Recognize text using HMM.
		/// 
		/// Takes an image and a mask (where each connected component corresponds to a segmented character)
		/// on input and returns recognized text in the output_text parameter. Optionally
		/// provides also the Rects for individual text elements found (e.g. words), and the list of those
		/// text elements with their confidence values.
		/// 
		/// ## Parameters
		/// * image: Input image CV_8UC1 or CV_8UC3 with a single text line (or word).
		/// * mask: Input binary image CV_8UC1 same size as input image. Each connected component in mask corresponds to a segmented character in the input image.
		/// 
		/// * output_text: Output text. Most likely character sequence found by the HMM decoder.
		/// 
		/// * component_rects: If provided the method will output a list of Rects for the individual
		/// text elements found (e.g. words).
		/// 
		/// * component_texts: If provided the method will output a list of text strings for the
		/// recognition of individual text elements found (e.g. words).
		/// 
		/// * component_confidences: If provided the method will output a list of confidence values
		/// for the recognition of individual text elements found (e.g. words).
		/// 
		/// * component_level: Only OCR_LEVEL_WORD is supported.
		/// 
		/// ## C++ default parameters
		/// * component_rects: NULL
		/// * component_texts: NULL
		/// * component_confidences: NULL
		/// * component_level: 0
		#[inline]
		fn run_multiple_mask(&mut self, image: &mut core::Mat, mask: &mut core::Mat, output_text: &mut String, component_rects: &mut core::Vector<core::Rect>, component_texts: &mut core::Vector<String>, component_confidences: &mut core::Vector<f32>, component_level: i32) -> Result<()> {
			string_arg_output_send!(via output_text_via);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRHMMDecoder_run_MatR_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(self.as_raw_mut_OCRHMMDecoder(), image.as_raw_mut_Mat(), mask.as_raw_mut_Mat(), &mut output_text_via, component_rects.as_raw_mut_VectorOfRect(), component_texts.as_raw_mut_VectorOfString(), component_confidences.as_raw_mut_VectorOff32(), component_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			string_arg_output_receive!(output_text_via => output_text);
			Ok(ret)
		}
		
		/// Recognize text using HMM.
		/// 
		/// Takes an image and a mask (where each connected component corresponds to a segmented character)
		/// on input and returns recognized text in the output_text parameter. Optionally
		/// provides also the Rects for individual text elements found (e.g. words), and the list of those
		/// text elements with their confidence values.
		/// 
		/// ## Parameters
		/// * image: Input image CV_8UC1 or CV_8UC3 with a single text line (or word).
		/// * mask: Input binary image CV_8UC1 same size as input image. Each connected component in mask corresponds to a segmented character in the input image.
		/// 
		/// * output_text: Output text. Most likely character sequence found by the HMM decoder.
		/// 
		/// * component_rects: If provided the method will output a list of Rects for the individual
		/// text elements found (e.g. words).
		/// 
		/// * component_texts: If provided the method will output a list of text strings for the
		/// recognition of individual text elements found (e.g. words).
		/// 
		/// * component_confidences: If provided the method will output a list of confidence values
		/// for the recognition of individual text elements found (e.g. words).
		/// 
		/// * component_level: Only OCR_LEVEL_WORD is supported.
		/// 
		/// ## Note
		/// This alternative version of [run_multiple_mask] function uses the following default values for its arguments:
		/// * component_rects: NULL
		/// * component_texts: NULL
		/// * component_confidences: NULL
		/// * component_level: 0
		#[inline]
		fn run_multiple_mask_def(&mut self, image: &mut core::Mat, mask: &mut core::Mat, output_text: &mut String) -> Result<()> {
			string_arg_output_send!(via output_text_via);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRHMMDecoder_run_MatR_MatR_stringR(self.as_raw_mut_OCRHMMDecoder(), image.as_raw_mut_Mat(), mask.as_raw_mut_Mat(), &mut output_text_via, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			string_arg_output_receive!(output_text_via => output_text);
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * component_level: 0
		#[inline]
		fn run(&mut self, image: &impl core::ToInputArray, min_confidence: i32, component_level: i32) -> Result<String> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRHMMDecoder_run_const__InputArrayR_int_int(self.as_raw_mut_OCRHMMDecoder(), image.as_raw__InputArray(), min_confidence, component_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [run] function uses the following default values for its arguments:
		/// * component_level: 0
		#[inline]
		fn run_def(&mut self, image: &impl core::ToInputArray, min_confidence: i32) -> Result<String> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRHMMDecoder_run_const__InputArrayR_int(self.as_raw_mut_OCRHMMDecoder(), image.as_raw__InputArray(), min_confidence, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * component_level: 0
		#[inline]
		fn run_mask(&mut self, image: &impl core::ToInputArray, mask: &impl core::ToInputArray, min_confidence: i32, component_level: i32) -> Result<String> {
			input_array_arg!(image);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRHMMDecoder_run_const__InputArrayR_const__InputArrayR_int_int(self.as_raw_mut_OCRHMMDecoder(), image.as_raw__InputArray(), mask.as_raw__InputArray(), min_confidence, component_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [run_mask] function uses the following default values for its arguments:
		/// * component_level: 0
		#[inline]
		fn run_mask_def(&mut self, image: &impl core::ToInputArray, mask: &impl core::ToInputArray, min_confidence: i32) -> Result<String> {
			input_array_arg!(image);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRHMMDecoder_run_const__InputArrayR_const__InputArrayR_int(self.as_raw_mut_OCRHMMDecoder(), image.as_raw__InputArray(), mask.as_raw__InputArray(), min_confidence, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// OCRHMMDecoder class provides an interface for OCR using Hidden Markov Models.
	/// 
	/// 
	/// Note:
	///    *   (C++) An example on using OCRHMMDecoder recognition combined with scene text detection can
	///        be found at the webcam_demo sample:
	///        <https://github.com/opencv/opencv_contrib/blob/master/modules/text/samples/webcam_demo.cpp>
	pub struct OCRHMMDecoder {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { OCRHMMDecoder }
	
	impl Drop for OCRHMMDecoder {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_text_OCRHMMDecoder_delete(self.as_raw_mut_OCRHMMDecoder()) };
		}
	}
	
	unsafe impl Send for OCRHMMDecoder {}
	
	impl crate::text::BaseOCRTraitConst for OCRHMMDecoder {
		#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::text::BaseOCRTrait for OCRHMMDecoder {
		#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::OCRHMMDecoderTraitConst for OCRHMMDecoder {
		#[inline] fn as_raw_OCRHMMDecoder(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::text::OCRHMMDecoderTrait for OCRHMMDecoder {
		#[inline] fn as_raw_mut_OCRHMMDecoder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl OCRHMMDecoder {
		/// Creates an instance of the OCRHMMDecoder class. Initializes HMMDecoder.
		/// 
		/// ## Parameters
		/// * classifier: The character classifier with built in feature extractor.
		/// 
		/// * vocabulary: The language vocabulary (chars when ascii english text). vocabulary.size()
		/// must be equal to the number of classes of the classifier.
		/// 
		/// * transition_probabilities_table: Table with transition probabilities between character
		/// pairs. cols == rows == vocabulary.size().
		/// 
		/// * emission_probabilities_table: Table with observation emission probabilities. cols ==
		/// rows == vocabulary.size().
		/// 
		/// * mode: HMM Decoding algorithm. Only OCR_DECODER_VITERBI is available for the moment
		/// (<http://en.wikipedia.org/wiki/Viterbi_algorithm>).
		/// 
		/// ## C++ default parameters
		/// * mode: OCR_DECODER_VITERBI
		#[inline]
		pub fn create(classifier: core::Ptr<crate::text::OCRHMMDecoder_ClassifierCallback>, vocabulary: &str, transition_probabilities_table: &impl core::ToInputArray, emission_probabilities_table: &impl core::ToInputArray, mode: i32) -> Result<core::Ptr<crate::text::OCRHMMDecoder>> {
			extern_container_arg!(vocabulary);
			input_array_arg!(transition_probabilities_table);
			input_array_arg!(emission_probabilities_table);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRHMMDecoder_create_const_PtrLClassifierCallbackG_const_StringR_const__InputArrayR_const__InputArrayR_int(classifier.as_raw_PtrOfOCRHMMDecoder_ClassifierCallback(), vocabulary.opencv_as_extern(), transition_probabilities_table.as_raw__InputArray(), emission_probabilities_table.as_raw__InputArray(), mode, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::text::OCRHMMDecoder>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Creates an instance of the OCRHMMDecoder class. Initializes HMMDecoder.
		/// 
		/// ## Parameters
		/// * classifier: The character classifier with built in feature extractor.
		/// 
		/// * vocabulary: The language vocabulary (chars when ascii english text). vocabulary.size()
		/// must be equal to the number of classes of the classifier.
		/// 
		/// * transition_probabilities_table: Table with transition probabilities between character
		/// pairs. cols == rows == vocabulary.size().
		/// 
		/// * emission_probabilities_table: Table with observation emission probabilities. cols ==
		/// rows == vocabulary.size().
		/// 
		/// * mode: HMM Decoding algorithm. Only OCR_DECODER_VITERBI is available for the moment
		/// (<http://en.wikipedia.org/wiki/Viterbi_algorithm>).
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * mode: OCR_DECODER_VITERBI
		#[inline]
		pub fn create_def(classifier: core::Ptr<crate::text::OCRHMMDecoder_ClassifierCallback>, vocabulary: &str, transition_probabilities_table: &impl core::ToInputArray, emission_probabilities_table: &impl core::ToInputArray) -> Result<core::Ptr<crate::text::OCRHMMDecoder>> {
			extern_container_arg!(vocabulary);
			input_array_arg!(transition_probabilities_table);
			input_array_arg!(emission_probabilities_table);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRHMMDecoder_create_const_PtrLClassifierCallbackG_const_StringR_const__InputArrayR_const__InputArrayR(classifier.as_raw_PtrOfOCRHMMDecoder_ClassifierCallback(), vocabulary.opencv_as_extern(), transition_probabilities_table.as_raw__InputArray(), emission_probabilities_table.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::text::OCRHMMDecoder>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Creates an instance of the OCRHMMDecoder class. Loads and initializes HMMDecoder from the specified path
		/// 
		///      Creates an instance of the OCRHMMDecoder class. Initializes HMMDecoder.
		/// 
		/// ## Parameters
		/// * classifier: The character classifier with built in feature extractor.
		/// 
		/// * vocabulary: The language vocabulary (chars when ascii english text). vocabulary.size()
		/// must be equal to the number of classes of the classifier.
		/// 
		/// * transition_probabilities_table: Table with transition probabilities between character
		/// pairs. cols == rows == vocabulary.size().
		/// 
		/// * emission_probabilities_table: Table with observation emission probabilities. cols ==
		/// rows == vocabulary.size().
		/// 
		/// * mode: HMM Decoding algorithm. Only OCR_DECODER_VITERBI is available for the moment
		/// (<http://en.wikipedia.org/wiki/Viterbi_algorithm>).
		/// 
		/// ## Overloaded parameters
		/// 
		/// ## C++ default parameters
		/// * mode: OCR_DECODER_VITERBI
		/// * classifier: OCR_KNN_CLASSIFIER
		#[inline]
		pub fn create_from_file(filename: &str, vocabulary: &str, transition_probabilities_table: &impl core::ToInputArray, emission_probabilities_table: &impl core::ToInputArray, mode: i32, classifier: i32) -> Result<core::Ptr<crate::text::OCRHMMDecoder>> {
			extern_container_arg!(filename);
			extern_container_arg!(vocabulary);
			input_array_arg!(transition_probabilities_table);
			input_array_arg!(emission_probabilities_table);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRHMMDecoder_create_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR_int_int(filename.opencv_as_extern(), vocabulary.opencv_as_extern(), transition_probabilities_table.as_raw__InputArray(), emission_probabilities_table.as_raw__InputArray(), mode, classifier, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::text::OCRHMMDecoder>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Creates an instance of the OCRHMMDecoder class. Loads and initializes HMMDecoder from the specified path
		/// 
		/// @overload
		/// 
		/// ## Note
		/// This alternative version of [create_from_file] function uses the following default values for its arguments:
		/// * mode: OCR_DECODER_VITERBI
		/// * classifier: OCR_KNN_CLASSIFIER
		#[inline]
		pub fn create_from_file_def(filename: &str, vocabulary: &str, transition_probabilities_table: &impl core::ToInputArray, emission_probabilities_table: &impl core::ToInputArray) -> Result<core::Ptr<crate::text::OCRHMMDecoder>> {
			extern_container_arg!(filename);
			extern_container_arg!(vocabulary);
			input_array_arg!(transition_probabilities_table);
			input_array_arg!(emission_probabilities_table);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRHMMDecoder_create_const_StringR_const_StringR_const__InputArrayR_const__InputArrayR(filename.opencv_as_extern(), vocabulary.opencv_as_extern(), transition_probabilities_table.as_raw__InputArray(), emission_probabilities_table.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::text::OCRHMMDecoder>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { OCRHMMDecoder, crate::text::BaseOCR, cv_text_OCRHMMDecoder_to_BaseOCR }
	
	impl std::fmt::Debug for OCRHMMDecoder {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("OCRHMMDecoder")
				.finish()
		}
	}
	
	/// Constant methods for [crate::text::OCRHMMDecoder_ClassifierCallback]
	pub trait OCRHMMDecoder_ClassifierCallbackTraitConst {
		fn as_raw_OCRHMMDecoder_ClassifierCallback(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::text::OCRHMMDecoder_ClassifierCallback]
	pub trait OCRHMMDecoder_ClassifierCallbackTrait: crate::text::OCRHMMDecoder_ClassifierCallbackTraitConst {
		fn as_raw_mut_OCRHMMDecoder_ClassifierCallback(&mut self) -> *mut c_void;
	
		/// The character classifier must return a (ranked list of) class(es) id('s)
		/// 
		/// ## Parameters
		/// * image: Input image CV_8UC1 or CV_8UC3 with a single letter.
		/// * out_class: The classifier returns the character class categorical label, or list of
		/// class labels, to which the input image corresponds.
		/// * out_confidence: The classifier returns the probability of the input image
		/// corresponding to each classes in out_class.
		#[inline]
		fn eval(&mut self, image: &impl core::ToInputArray, out_class: &mut core::Vector<i32>, out_confidence: &mut core::Vector<f64>) -> Result<()> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRHMMDecoder_ClassifierCallback_eval_const__InputArrayR_vectorLintGR_vectorLdoubleGR(self.as_raw_mut_OCRHMMDecoder_ClassifierCallback(), image.as_raw__InputArray(), out_class.as_raw_mut_VectorOfi32(), out_confidence.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Callback with the character classifier is made a class.
	/// 
	/// This way it hides the feature extractor and the classifier itself, so developers can write
	/// their own OCR code.
	/// 
	/// The default character classifier and feature extractor can be loaded using the utility function
	/// loadOCRHMMClassifierNM and KNN model provided in
	/// <https://github.com/opencv/opencv_contrib/blob/master/modules/text/samples/OCRHMM_knn_model_data.xml.gz>.
	pub struct OCRHMMDecoder_ClassifierCallback {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { OCRHMMDecoder_ClassifierCallback }
	
	impl Drop for OCRHMMDecoder_ClassifierCallback {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_text_OCRHMMDecoder_ClassifierCallback_delete(self.as_raw_mut_OCRHMMDecoder_ClassifierCallback()) };
		}
	}
	
	unsafe impl Send for OCRHMMDecoder_ClassifierCallback {}
	
	impl crate::text::OCRHMMDecoder_ClassifierCallbackTraitConst for OCRHMMDecoder_ClassifierCallback {
		#[inline] fn as_raw_OCRHMMDecoder_ClassifierCallback(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::text::OCRHMMDecoder_ClassifierCallbackTrait for OCRHMMDecoder_ClassifierCallback {
		#[inline] fn as_raw_mut_OCRHMMDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl OCRHMMDecoder_ClassifierCallback {
	}
	
	impl std::fmt::Debug for OCRHMMDecoder_ClassifierCallback {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("OCRHMMDecoder_ClassifierCallback")
				.finish()
		}
	}
	
	/// Constant methods for [crate::text::OCRHolisticWordRecognizer]
	pub trait OCRHolisticWordRecognizerTraitConst: crate::text::BaseOCRTraitConst {
		fn as_raw_OCRHolisticWordRecognizer(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::text::OCRHolisticWordRecognizer]
	pub trait OCRHolisticWordRecognizerTrait: crate::text::BaseOCRTrait + crate::text::OCRHolisticWordRecognizerTraitConst {
		fn as_raw_mut_OCRHolisticWordRecognizer(&mut self) -> *mut c_void;
	
		/// ## C++ default parameters
		/// * component_rects: NULL
		/// * component_texts: NULL
		/// * component_confidences: NULL
		/// * component_level: OCR_LEVEL_WORD
		#[inline]
		fn run(&mut self, image: &mut core::Mat, output_text: &mut String, component_rects: &mut core::Vector<core::Rect>, component_texts: &mut core::Vector<String>, component_confidences: &mut core::Vector<f32>, component_level: i32) -> Result<()> {
			string_arg_output_send!(via output_text_via);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRHolisticWordRecognizer_run_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(self.as_raw_mut_OCRHolisticWordRecognizer(), image.as_raw_mut_Mat(), &mut output_text_via, component_rects.as_raw_mut_VectorOfRect(), component_texts.as_raw_mut_VectorOfString(), component_confidences.as_raw_mut_VectorOff32(), component_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			string_arg_output_receive!(output_text_via => output_text);
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [run] function uses the following default values for its arguments:
		/// * component_rects: NULL
		/// * component_texts: NULL
		/// * component_confidences: NULL
		/// * component_level: OCR_LEVEL_WORD
		#[inline]
		fn run_def(&mut self, image: &mut core::Mat, output_text: &mut String) -> Result<()> {
			string_arg_output_send!(via output_text_via);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRHolisticWordRecognizer_run_MatR_stringR(self.as_raw_mut_OCRHolisticWordRecognizer(), image.as_raw_mut_Mat(), &mut output_text_via, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			string_arg_output_receive!(output_text_via => output_text);
			Ok(ret)
		}
		
		/// Recognize text using a segmentation based word-spotting/classifier cnn.
		/// 
		/// Takes image on input and returns recognized text in the output_text parameter. Optionally
		/// provides also the Rects for individual text elements found (e.g. words), and the list of those
		/// text elements with their confidence values.
		/// 
		/// ## Parameters
		/// * image: Input image CV_8UC1 or CV_8UC3
		/// 
		/// * mask: is totally ignored and is only available for compatibillity reasons
		/// 
		/// * output_text: Output text of the the word spoting, always one that exists in the dictionary.
		/// 
		/// * component_rects: Not applicable for word spotting can be be NULL if not, a single elemnt will
		///    be put in the vector.
		/// 
		/// * component_texts: Not applicable for word spotting can be be NULL if not, a single elemnt will
		///    be put in the vector.
		/// 
		/// * component_confidences: Not applicable for word spotting can be be NULL if not, a single elemnt will
		///    be put in the vector.
		/// 
		/// * component_level: must be OCR_LEVEL_WORD.
		/// 
		/// ## C++ default parameters
		/// * component_rects: NULL
		/// * component_texts: NULL
		/// * component_confidences: NULL
		/// * component_level: OCR_LEVEL_WORD
		#[inline]
		fn run_mask(&mut self, image: &mut core::Mat, mask: &mut core::Mat, output_text: &mut String, component_rects: &mut core::Vector<core::Rect>, component_texts: &mut core::Vector<String>, component_confidences: &mut core::Vector<f32>, component_level: i32) -> Result<()> {
			string_arg_output_send!(via output_text_via);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRHolisticWordRecognizer_run_MatR_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(self.as_raw_mut_OCRHolisticWordRecognizer(), image.as_raw_mut_Mat(), mask.as_raw_mut_Mat(), &mut output_text_via, component_rects.as_raw_mut_VectorOfRect(), component_texts.as_raw_mut_VectorOfString(), component_confidences.as_raw_mut_VectorOff32(), component_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			string_arg_output_receive!(output_text_via => output_text);
			Ok(ret)
		}
		
		/// Recognize text using a segmentation based word-spotting/classifier cnn.
		/// 
		/// Takes image on input and returns recognized text in the output_text parameter. Optionally
		/// provides also the Rects for individual text elements found (e.g. words), and the list of those
		/// text elements with their confidence values.
		/// 
		/// ## Parameters
		/// * image: Input image CV_8UC1 or CV_8UC3
		/// 
		/// * mask: is totally ignored and is only available for compatibillity reasons
		/// 
		/// * output_text: Output text of the the word spoting, always one that exists in the dictionary.
		/// 
		/// * component_rects: Not applicable for word spotting can be be NULL if not, a single elemnt will
		///    be put in the vector.
		/// 
		/// * component_texts: Not applicable for word spotting can be be NULL if not, a single elemnt will
		///    be put in the vector.
		/// 
		/// * component_confidences: Not applicable for word spotting can be be NULL if not, a single elemnt will
		///    be put in the vector.
		/// 
		/// * component_level: must be OCR_LEVEL_WORD.
		/// 
		/// ## Note
		/// This alternative version of [run_mask] function uses the following default values for its arguments:
		/// * component_rects: NULL
		/// * component_texts: NULL
		/// * component_confidences: NULL
		/// * component_level: OCR_LEVEL_WORD
		#[inline]
		fn run_mask_def(&mut self, image: &mut core::Mat, mask: &mut core::Mat, output_text: &mut String) -> Result<()> {
			string_arg_output_send!(via output_text_via);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRHolisticWordRecognizer_run_MatR_MatR_stringR(self.as_raw_mut_OCRHolisticWordRecognizer(), image.as_raw_mut_Mat(), mask.as_raw_mut_Mat(), &mut output_text_via, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			string_arg_output_receive!(output_text_via => output_text);
			Ok(ret)
		}
		
	}
	
	/// OCRHolisticWordRecognizer class provides the functionallity of segmented wordspotting.
	/// Given a predefined vocabulary , a DictNet is employed to select the most probable
	/// word given an input image.
	/// 
	/// DictNet is described in detail in:
	/// Max Jaderberg et al.: Reading Text in the Wild with Convolutional Neural Networks, IJCV 2015
	/// <http://arxiv.org/abs/1412.1842>
	pub struct OCRHolisticWordRecognizer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { OCRHolisticWordRecognizer }
	
	impl Drop for OCRHolisticWordRecognizer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_text_OCRHolisticWordRecognizer_delete(self.as_raw_mut_OCRHolisticWordRecognizer()) };
		}
	}
	
	unsafe impl Send for OCRHolisticWordRecognizer {}
	
	impl crate::text::BaseOCRTraitConst for OCRHolisticWordRecognizer {
		#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::text::BaseOCRTrait for OCRHolisticWordRecognizer {
		#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::OCRHolisticWordRecognizerTraitConst for OCRHolisticWordRecognizer {
		#[inline] fn as_raw_OCRHolisticWordRecognizer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::text::OCRHolisticWordRecognizerTrait for OCRHolisticWordRecognizer {
		#[inline] fn as_raw_mut_OCRHolisticWordRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl OCRHolisticWordRecognizer {
		/// Creates an instance of the OCRHolisticWordRecognizer class.
		#[inline]
		pub fn create(arch_filename: &str, weights_filename: &str, words_filename: &str) -> Result<core::Ptr<crate::text::OCRHolisticWordRecognizer>> {
			extern_container_arg!(arch_filename);
			extern_container_arg!(weights_filename);
			extern_container_arg!(words_filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRHolisticWordRecognizer_create_const_stringR_const_stringR_const_stringR(arch_filename.opencv_as_extern(), weights_filename.opencv_as_extern(), words_filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::text::OCRHolisticWordRecognizer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { OCRHolisticWordRecognizer, crate::text::BaseOCR, cv_text_OCRHolisticWordRecognizer_to_BaseOCR }
	
	impl std::fmt::Debug for OCRHolisticWordRecognizer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("OCRHolisticWordRecognizer")
				.finish()
		}
	}
	
	/// Constant methods for [crate::text::OCRTesseract]
	pub trait OCRTesseractTraitConst: crate::text::BaseOCRTraitConst {
		fn as_raw_OCRTesseract(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::text::OCRTesseract]
	pub trait OCRTesseractTrait: crate::text::BaseOCRTrait + crate::text::OCRTesseractTraitConst {
		fn as_raw_mut_OCRTesseract(&mut self) -> *mut c_void;
	
		/// Recognize text using the tesseract-ocr API.
		/// 
		/// Takes image on input and returns recognized text in the output_text parameter. Optionally
		/// provides also the Rects for individual text elements found (e.g. words), and the list of those
		/// text elements with their confidence values.
		/// 
		/// ## Parameters
		/// * image: Input image CV_8UC1 or CV_8UC3
		/// * output_text: Output text of the tesseract-ocr.
		/// * component_rects: If provided the method will output a list of Rects for the individual
		/// text elements found (e.g. words or text lines).
		/// * component_texts: If provided the method will output a list of text strings for the
		/// recognition of individual text elements found (e.g. words or text lines).
		/// * component_confidences: If provided the method will output a list of confidence values
		/// for the recognition of individual text elements found (e.g. words or text lines).
		/// * component_level: OCR_LEVEL_WORD (by default), or OCR_LEVEL_TEXTLINE.
		/// 
		/// ## C++ default parameters
		/// * component_rects: NULL
		/// * component_texts: NULL
		/// * component_confidences: NULL
		/// * component_level: 0
		#[inline]
		fn run_multiple(&mut self, image: &mut core::Mat, output_text: &mut String, component_rects: &mut core::Vector<core::Rect>, component_texts: &mut core::Vector<String>, component_confidences: &mut core::Vector<f32>, component_level: i32) -> Result<()> {
			string_arg_output_send!(via output_text_via);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRTesseract_run_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(self.as_raw_mut_OCRTesseract(), image.as_raw_mut_Mat(), &mut output_text_via, component_rects.as_raw_mut_VectorOfRect(), component_texts.as_raw_mut_VectorOfString(), component_confidences.as_raw_mut_VectorOff32(), component_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			string_arg_output_receive!(output_text_via => output_text);
			Ok(ret)
		}
		
		/// Recognize text using the tesseract-ocr API.
		/// 
		/// Takes image on input and returns recognized text in the output_text parameter. Optionally
		/// provides also the Rects for individual text elements found (e.g. words), and the list of those
		/// text elements with their confidence values.
		/// 
		/// ## Parameters
		/// * image: Input image CV_8UC1 or CV_8UC3
		/// * output_text: Output text of the tesseract-ocr.
		/// * component_rects: If provided the method will output a list of Rects for the individual
		/// text elements found (e.g. words or text lines).
		/// * component_texts: If provided the method will output a list of text strings for the
		/// recognition of individual text elements found (e.g. words or text lines).
		/// * component_confidences: If provided the method will output a list of confidence values
		/// for the recognition of individual text elements found (e.g. words or text lines).
		/// * component_level: OCR_LEVEL_WORD (by default), or OCR_LEVEL_TEXTLINE.
		/// 
		/// ## Note
		/// This alternative version of [run_multiple] function uses the following default values for its arguments:
		/// * component_rects: NULL
		/// * component_texts: NULL
		/// * component_confidences: NULL
		/// * component_level: 0
		#[inline]
		fn run_multiple_def(&mut self, image: &mut core::Mat, output_text: &mut String) -> Result<()> {
			string_arg_output_send!(via output_text_via);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRTesseract_run_MatR_stringR(self.as_raw_mut_OCRTesseract(), image.as_raw_mut_Mat(), &mut output_text_via, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			string_arg_output_receive!(output_text_via => output_text);
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * component_rects: NULL
		/// * component_texts: NULL
		/// * component_confidences: NULL
		/// * component_level: 0
		#[inline]
		fn run_multiple_mask(&mut self, image: &mut core::Mat, mask: &mut core::Mat, output_text: &mut String, component_rects: &mut core::Vector<core::Rect>, component_texts: &mut core::Vector<String>, component_confidences: &mut core::Vector<f32>, component_level: i32) -> Result<()> {
			string_arg_output_send!(via output_text_via);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRTesseract_run_MatR_MatR_stringR_vectorLRectGX_vectorLstringGX_vectorLfloatGX_int(self.as_raw_mut_OCRTesseract(), image.as_raw_mut_Mat(), mask.as_raw_mut_Mat(), &mut output_text_via, component_rects.as_raw_mut_VectorOfRect(), component_texts.as_raw_mut_VectorOfString(), component_confidences.as_raw_mut_VectorOff32(), component_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			string_arg_output_receive!(output_text_via => output_text);
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [run_multiple_mask] function uses the following default values for its arguments:
		/// * component_rects: NULL
		/// * component_texts: NULL
		/// * component_confidences: NULL
		/// * component_level: 0
		#[inline]
		fn run_multiple_mask_def(&mut self, image: &mut core::Mat, mask: &mut core::Mat, output_text: &mut String) -> Result<()> {
			string_arg_output_send!(via output_text_via);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRTesseract_run_MatR_MatR_stringR(self.as_raw_mut_OCRTesseract(), image.as_raw_mut_Mat(), mask.as_raw_mut_Mat(), &mut output_text_via, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			string_arg_output_receive!(output_text_via => output_text);
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * component_level: 0
		#[inline]
		fn run(&mut self, image: &impl core::ToInputArray, min_confidence: i32, component_level: i32) -> Result<String> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRTesseract_run_const__InputArrayR_int_int(self.as_raw_mut_OCRTesseract(), image.as_raw__InputArray(), min_confidence, component_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [run] function uses the following default values for its arguments:
		/// * component_level: 0
		#[inline]
		fn run_def(&mut self, image: &impl core::ToInputArray, min_confidence: i32) -> Result<String> {
			input_array_arg!(image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRTesseract_run_const__InputArrayR_int(self.as_raw_mut_OCRTesseract(), image.as_raw__InputArray(), min_confidence, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## C++ default parameters
		/// * component_level: 0
		#[inline]
		fn run_mask(&mut self, image: &impl core::ToInputArray, mask: &impl core::ToInputArray, min_confidence: i32, component_level: i32) -> Result<String> {
			input_array_arg!(image);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRTesseract_run_const__InputArrayR_const__InputArrayR_int_int(self.as_raw_mut_OCRTesseract(), image.as_raw__InputArray(), mask.as_raw__InputArray(), min_confidence, component_level, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// ## Note
		/// This alternative version of [run_mask] function uses the following default values for its arguments:
		/// * component_level: 0
		#[inline]
		fn run_mask_def(&mut self, image: &impl core::ToInputArray, mask: &impl core::ToInputArray, min_confidence: i32) -> Result<String> {
			input_array_arg!(image);
			input_array_arg!(mask);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRTesseract_run_const__InputArrayR_const__InputArrayR_int(self.as_raw_mut_OCRTesseract(), image.as_raw__InputArray(), mask.as_raw__InputArray(), min_confidence, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { String::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		#[inline]
		fn set_white_list(&mut self, char_whitelist: &str) -> Result<()> {
			extern_container_arg!(char_whitelist);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRTesseract_setWhiteList_const_StringR(self.as_raw_mut_OCRTesseract(), char_whitelist.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// OCRTesseract class provides an interface with the tesseract-ocr API (v3.02.02) in C++.
	/// 
	/// Notice that it is compiled only when tesseract-ocr is correctly installed.
	/// 
	/// 
	/// Note:
	///    *   (C++) An example of OCRTesseract recognition combined with scene text detection can be found
	///        at the end_to_end_recognition demo:
	///        <https://github.com/opencv/opencv_contrib/blob/master/modules/text/samples/end_to_end_recognition.cpp>
	///    *   (C++) Another example of OCRTesseract recognition combined with scene text detection can be
	///        found at the webcam_demo:
	///        <https://github.com/opencv/opencv_contrib/blob/master/modules/text/samples/webcam_demo.cpp>
	pub struct OCRTesseract {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { OCRTesseract }
	
	impl Drop for OCRTesseract {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_text_OCRTesseract_delete(self.as_raw_mut_OCRTesseract()) };
		}
	}
	
	unsafe impl Send for OCRTesseract {}
	
	impl crate::text::BaseOCRTraitConst for OCRTesseract {
		#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::text::BaseOCRTrait for OCRTesseract {
		#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::OCRTesseractTraitConst for OCRTesseract {
		#[inline] fn as_raw_OCRTesseract(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::text::OCRTesseractTrait for OCRTesseract {
		#[inline] fn as_raw_mut_OCRTesseract(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl OCRTesseract {
		/// Creates an instance of the OCRTesseract class. Initializes Tesseract.
		/// 
		/// ## Parameters
		/// * datapath: the name of the parent directory of tessdata ended with "/", or NULL to use the
		/// system's default directory.
		/// * language: an ISO 639-3 code or NULL will default to "eng".
		/// * char_whitelist: specifies the list of characters used for recognition. NULL defaults to ""
		/// (All characters will be used for recognition).
		/// * oem: tesseract-ocr offers different OCR Engine Modes (OEM), by default
		/// tesseract::OEM_DEFAULT is used. See the tesseract-ocr API documentation for other possible
		/// values.
		/// * psmode: tesseract-ocr offers different Page Segmentation Modes (PSM) tesseract::PSM_AUTO
		/// (fully automatic layout analysis) is used. See the tesseract-ocr API documentation for other
		/// possible values.
		/// 
		/// 
		/// Note: The char_whitelist default is changed after OpenCV 4.7.0/3.19.0 from "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ" to "".
		/// 
		/// ## C++ default parameters
		/// * datapath: NULL
		/// * language: NULL
		/// * char_whitelist: NULL
		/// * oem: OEM_DEFAULT
		/// * psmode: PSM_AUTO
		#[inline]
		pub fn create(datapath: &str, language: &str, char_whitelist: &str, oem: i32, psmode: i32) -> Result<core::Ptr<crate::text::OCRTesseract>> {
			extern_container_arg!(datapath);
			extern_container_arg!(language);
			extern_container_arg!(char_whitelist);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRTesseract_create_const_charX_const_charX_const_charX_int_int(datapath.opencv_as_extern(), language.opencv_as_extern(), char_whitelist.opencv_as_extern(), oem, psmode, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::text::OCRTesseract>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Creates an instance of the OCRTesseract class. Initializes Tesseract.
		/// 
		/// ## Parameters
		/// * datapath: the name of the parent directory of tessdata ended with "/", or NULL to use the
		/// system's default directory.
		/// * language: an ISO 639-3 code or NULL will default to "eng".
		/// * char_whitelist: specifies the list of characters used for recognition. NULL defaults to ""
		/// (All characters will be used for recognition).
		/// * oem: tesseract-ocr offers different OCR Engine Modes (OEM), by default
		/// tesseract::OEM_DEFAULT is used. See the tesseract-ocr API documentation for other possible
		/// values.
		/// * psmode: tesseract-ocr offers different Page Segmentation Modes (PSM) tesseract::PSM_AUTO
		/// (fully automatic layout analysis) is used. See the tesseract-ocr API documentation for other
		/// possible values.
		/// 
		/// 
		/// Note: The char_whitelist default is changed after OpenCV 4.7.0/3.19.0 from "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ" to "".
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * datapath: NULL
		/// * language: NULL
		/// * char_whitelist: NULL
		/// * oem: OEM_DEFAULT
		/// * psmode: PSM_AUTO
		#[inline]
		pub fn create_def() -> Result<core::Ptr<crate::text::OCRTesseract>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_OCRTesseract_create(ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::text::OCRTesseract>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { OCRTesseract, crate::text::BaseOCR, cv_text_OCRTesseract_to_BaseOCR }
	
	impl std::fmt::Debug for OCRTesseract {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("OCRTesseract")
				.finish()
		}
	}
	
	/// Constant methods for [crate::text::TextDetector]
	pub trait TextDetectorTraitConst {
		fn as_raw_TextDetector(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::text::TextDetector]
	pub trait TextDetectorTrait: crate::text::TextDetectorTraitConst {
		fn as_raw_mut_TextDetector(&mut self) -> *mut c_void;
	
		/// Method that provides a quick and simple interface to detect text inside an image
		/// 
		/// ## Parameters
		/// * inputImage: an image to process
		/// * Bbox: a vector of Rect that will store the detected word bounding box
		/// * confidence: a vector of float that will be updated with the confidence the classifier has for the selected bounding box
		#[inline]
		fn detect(&mut self, input_image: &impl core::ToInputArray, bbox: &mut core::Vector<core::Rect>, confidence: &mut core::Vector<f32>) -> Result<()> {
			input_array_arg!(input_image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_TextDetector_detect_const__InputArrayR_vectorLRectGR_vectorLfloatGR(self.as_raw_mut_TextDetector(), input_image.as_raw__InputArray(), bbox.as_raw_mut_VectorOfRect(), confidence.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// An abstract class providing interface for text detection algorithms
	pub struct TextDetector {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { TextDetector }
	
	impl Drop for TextDetector {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_text_TextDetector_delete(self.as_raw_mut_TextDetector()) };
		}
	}
	
	unsafe impl Send for TextDetector {}
	
	impl crate::text::TextDetectorTraitConst for TextDetector {
		#[inline] fn as_raw_TextDetector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::text::TextDetectorTrait for TextDetector {
		#[inline] fn as_raw_mut_TextDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl TextDetector {
	}
	
	boxed_cast_descendant! { TextDetector, crate::text::TextDetectorCNN, cv_text_TextDetector_to_TextDetectorCNN }
	
	impl std::fmt::Debug for TextDetector {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("TextDetector")
				.finish()
		}
	}
	
	/// Constant methods for [crate::text::TextDetectorCNN]
	pub trait TextDetectorCNNTraitConst: crate::text::TextDetectorTraitConst {
		fn as_raw_TextDetectorCNN(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::text::TextDetectorCNN]
	pub trait TextDetectorCNNTrait: crate::text::TextDetectorCNNTraitConst + crate::text::TextDetectorTrait {
		fn as_raw_mut_TextDetectorCNN(&mut self) -> *mut c_void;
	
		/// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
		/// 
		/// ## Parameters
		/// * inputImage: an image expected to be a CV_U8C3 of any size
		/// * Bbox: a vector of Rect that will store the detected word bounding box
		/// * confidence: a vector of float that will be updated with the confidence the classifier has for the selected bounding box
		#[inline]
		fn detect(&mut self, input_image: &impl core::ToInputArray, bbox: &mut core::Vector<core::Rect>, confidence: &mut core::Vector<f32>) -> Result<()> {
			input_array_arg!(input_image);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_TextDetectorCNN_detect_const__InputArrayR_vectorLRectGR_vectorLfloatGR(self.as_raw_mut_TextDetectorCNN(), input_image.as_raw__InputArray(), bbox.as_raw_mut_VectorOfRect(), confidence.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// TextDetectorCNN class provides the functionallity of text bounding box detection.
	/// This class is representing to find bounding boxes of text words given an input image.
	/// This class uses OpenCV dnn module to load pre-trained model described in [LiaoSBWL17](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_LiaoSBWL17).
	/// The original repository with the modified SSD Caffe version: <https://github.com/MhLiao/TextBoxes>.
	/// Model can be downloaded from [DropBox](https://www.dropbox.com/s/g8pjzv2de9gty8g/TextBoxes_icdar13.caffemodel?dl=0).
	/// Modified .prototxt file with the model description can be found in `opencv_contrib/modules/text/samples/textbox.prototxt`.
	pub struct TextDetectorCNN {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { TextDetectorCNN }
	
	impl Drop for TextDetectorCNN {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_text_TextDetectorCNN_delete(self.as_raw_mut_TextDetectorCNN()) };
		}
	}
	
	unsafe impl Send for TextDetectorCNN {}
	
	impl crate::text::TextDetectorTraitConst for TextDetectorCNN {
		#[inline] fn as_raw_TextDetector(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::text::TextDetectorTrait for TextDetectorCNN {
		#[inline] fn as_raw_mut_TextDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::TextDetectorCNNTraitConst for TextDetectorCNN {
		#[inline] fn as_raw_TextDetectorCNN(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::text::TextDetectorCNNTrait for TextDetectorCNN {
		#[inline] fn as_raw_mut_TextDetectorCNN(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl TextDetectorCNN {
		/// Creates an instance of the TextDetectorCNN class using the provided parameters.
		/// 
		/// ## Parameters
		/// * modelArchFilename: the relative or absolute path to the prototxt file describing the classifiers architecture.
		/// * modelWeightsFilename: the relative or absolute path to the file containing the pretrained weights of the model in caffe-binary form.
		/// * detectionSizes: a list of sizes for multiscale detection. The values`[(300,300),(700,500),(700,300),(700,700),(1600,1600)]` are
		/// recommended in [LiaoSBWL17](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_LiaoSBWL17) to achieve the best quality.
		#[inline]
		pub fn create_with_sizes(model_arch_filename: &str, model_weights_filename: &str, mut detection_sizes: core::Vector<core::Size>) -> Result<core::Ptr<crate::text::TextDetectorCNN>> {
			extern_container_arg!(model_arch_filename);
			extern_container_arg!(model_weights_filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_TextDetectorCNN_create_const_StringR_const_StringR_vectorLSizeG(model_arch_filename.opencv_as_extern(), model_weights_filename.opencv_as_extern(), detection_sizes.as_raw_mut_VectorOfSize(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::text::TextDetectorCNN>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Creates an instance of the TextDetectorCNN class using the provided parameters.
		/// 
		/// ## Parameters
		/// * modelArchFilename: the relative or absolute path to the prototxt file describing the classifiers architecture.
		/// * modelWeightsFilename: the relative or absolute path to the file containing the pretrained weights of the model in caffe-binary form.
		/// * detectionSizes: a list of sizes for multiscale detection. The values`[(300,300),(700,500),(700,300),(700,700),(1600,1600)]` are
		/// recommended in [LiaoSBWL17](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_LiaoSBWL17) to achieve the best quality.
		/// 
		/// ## Overloaded parameters
		#[inline]
		pub fn create(model_arch_filename: &str, model_weights_filename: &str) -> Result<core::Ptr<crate::text::TextDetectorCNN>> {
			extern_container_arg!(model_arch_filename);
			extern_container_arg!(model_weights_filename);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_text_TextDetectorCNN_create_const_StringR_const_StringR(model_arch_filename.opencv_as_extern(), model_weights_filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::text::TextDetectorCNN>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { TextDetectorCNN, crate::text::TextDetector, cv_text_TextDetectorCNN_to_TextDetector }
	
	impl std::fmt::Debug for TextDetectorCNN {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("TextDetectorCNN")
				.finish()
		}
	}
}
