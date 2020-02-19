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
//! Jiri Matas [Neumann11](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Neumann11). The main idea behind Class-specific Extremal Regions is similar to the MSER
//! in that suitable Extremal Regions (ERs) are selected from the whole component tree of the image.
//! However, this technique differs from MSER in that selection of suitable ERs is done by a sequential
//! classifier trained for character detection, i.e. dropping the stability requirement of MSERs and
//! selecting class-specific (not necessarily stable) regions.
//! 
//! The component tree of an image is constructed by thresholding by an increasing value step-by-step
//! from 0 to 255 and then linking the obtained connected components from successive levels in a
//! hierarchy by their inclusion relation:
//! 
//! ![image](https://docs.opencv.org/3.4.9/component_tree.png)
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
//! two different grouping algorithms: the Exhaustive Search algorithm proposed in [Neumann12](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Neumann12) for
//! grouping horizontally aligned text, and the method proposed by Lluis Gomez and Dimosthenis Karatzas
//! in [Gomez13](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Gomez13) [Gomez14](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Gomez14) for grouping arbitrary oriented text (see erGrouping).
//! 
//! To see the text detector at work, have a look at the textdetection demo:
//! <https://github.com/opencv/opencv_contrib/blob/master/modules/text/samples/textdetection.cpp>
//! 
//!    # Scene Text Recognition
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::ERStatTrait, super::ERFilter_Callback, super::ERFilter, super::BaseOCR, super::OCRTesseract, super::OCRHMMDecoder_ClassifierCallbackTrait, super::OCRHMMDecoderTrait, super::OCRBeamSearchDecoder_ClassifierCallbackTrait, super::OCRBeamSearchDecoderTrait, super::OCRHolisticWordRecognizer, super::TextDetector, super::TextDetectorCNN };
}

pub const ERFILTER_NM_IHSGrad: i32 = 1;
pub const ERFILTER_NM_RGBLGrad: i32 = 0;
/// Text grouping method proposed in [Gomez13](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Gomez13) [Gomez14](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Gomez14) for grouping arbitrary oriented text. Regions
/// are agglomerated by Single Linkage Clustering in a weighted feature space that combines proximity
/// (x,y coordinates) and similarity measures (color, size, gradient magnitude, stroke width, etc.).
/// SLC provides a dendrogram where each node represents a text group hypothesis. Then the algorithm
/// finds the branches corresponding to text groups by traversing this dendrogram with a stopping rule
/// that combines the output of a rotation invariant text group classifier and a probabilistic measure
/// for hierarchical clustering validity assessment.
/// 
/// 
/// Note: This mode is not supported due NFA code removal ( https://github.com/opencv/opencv_contrib/issues/2235 )
pub const ERGROUPING_ORIENTATION_ANY: i32 = 1;
/// Exhaustive Search algorithm proposed in [Neumann11](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Neumann11) for grouping horizontally aligned text.
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
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum classifier_type {
	OCR_KNN_CLASSIFIER = 0 as isize,
	OCR_CNN_CLASSIFIER = 1 as isize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum decoder_mode {
	OCR_DECODER_VITERBI = 0 as isize,
}

/// text::erGrouping operation modes
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum erGrouping_Modes {
	/// Exhaustive Search algorithm proposed in [Neumann11](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Neumann11) for grouping horizontally aligned text.
	/// The algorithm models a verification function for all the possible ER sequences. The
	/// verification fuction for ER pairs consists in a set of threshold-based pairwise rules which
	/// compare measurements of two regions (height ratio, centroid angle, and region distance). The
	/// verification function for ER triplets creates a word text line estimate using Least
	/// Median-Squares fitting for a given triplet and then verifies that the estimate is valid (based
	/// on thresholds created during training). Verification functions for sequences larger than 3 are
	/// approximated by verifying that the text line parameters of all (sub)sequences of length 3 are
	/// consistent.
	ERGROUPING_ORIENTATION_HORIZ = 0 as isize,
	/// Text grouping method proposed in [Gomez13](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Gomez13) [Gomez14](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Gomez14) for grouping arbitrary oriented text. Regions
	/// are agglomerated by Single Linkage Clustering in a weighted feature space that combines proximity
	/// (x,y coordinates) and similarity measures (color, size, gradient magnitude, stroke width, etc.).
	/// SLC provides a dendrogram where each node represents a text group hypothesis. Then the algorithm
	/// finds the branches corresponding to text groups by traversing this dendrogram with a stopping rule
	/// that combines the output of a rotation invariant text group classifier and a probabilistic measure
	/// for hierarchical clustering validity assessment.
	/// 
	/// 
	/// Note: This mode is not supported due NFA code removal ( https://github.com/opencv/opencv_contrib/issues/2235 )
	ERGROUPING_ORIENTATION_ANY = 1 as isize,
}

/// Tesseract.OcrEngineMode Enumeration
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ocr_engine_mode {
	OEM_TESSERACT_ONLY = 0 as isize,
	OEM_CUBE_ONLY = 1 as isize,
	OEM_TESSERACT_CUBE_COMBINED = 2 as isize,
	OEM_DEFAULT = 3 as isize,
}

/// Tesseract.PageSegMode Enumeration
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum page_seg_mode {
	PSM_OSD_ONLY = 0 as isize,
	PSM_AUTO_OSD = 1 as isize,
	PSM_AUTO_ONLY = 2 as isize,
	PSM_AUTO = 3 as isize,
	PSM_SINGLE_COLUMN = 4 as isize,
	PSM_SINGLE_BLOCK_VERT_TEXT = 5 as isize,
	PSM_SINGLE_BLOCK = 6 as isize,
	PSM_SINGLE_LINE = 7 as isize,
	PSM_SINGLE_WORD = 8 as isize,
	PSM_CIRCLE_WORD = 9 as isize,
	PSM_SINGLE_CHAR = 10 as isize,
}

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
pub fn mse_rs_to_er_stats(image: &dyn core::ToInputArray, contours: &mut types::VectorOfVectorOfPoint, regions: &mut types::VectorOfVectorOfERStat) -> Result<()> {
	input_array_arg!(image);
	unsafe { sys::cv_text_MSERsToERStats_const__InputArrayX_vector_vector_Point__X_vector_vector_ERStat__X(image.as_raw__InputArray(), contours.as_raw_VectorOfVectorOfPoint(), regions.as_raw_VectorOfVectorOfERStat()) }.into_result()
}

/// Compute the different channels to be processed independently in the N&M algorithm [Neumann12](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Neumann12).
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
pub fn compute_nm_channels(_src: &dyn core::ToInputArray, _channels: &mut dyn core::ToOutputArray, _mode: i32) -> Result<()> {
	input_array_arg!(_src);
	output_array_arg!(_channels);
	unsafe { sys::cv_text_computeNMChannels_const__InputArrayX_const__OutputArrayX_int(_src.as_raw__InputArray(), _channels.as_raw__OutputArray(), _mode) }.into_result()
}

/// Create an Extremal Region Filter for the 1st stage classifier of N&M algorithm [Neumann12](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Neumann12).
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
pub fn create_er_filter_nm1(cb: &types::PtrOfERFilter_Callback, threshold_delta: i32, min_area: f32, max_area: f32, min_probability: f32, non_max_suppression: bool, min_probability_diff: f32) -> Result<types::PtrOfERFilter> {
	unsafe { sys::cv_text_createERFilterNM1_const_Ptr_Callback_X_int_float_float_float_bool_float(cb.as_raw_PtrOfERFilter_Callback(), threshold_delta, min_area, max_area, min_probability, non_max_suppression, min_probability_diff) }.into_result().map(|ptr| types::PtrOfERFilter { ptr })
}

/// Reads an Extremal Region Filter for the 1st stage classifier of N&M algorithm
///    from the provided path e.g. /path/to/cpp/trained_classifierNM1.xml
/// 
/// Create an Extremal Region Filter for the 1st stage classifier of N&M algorithm [Neumann12](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Neumann12).
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
pub fn create_er_filter_nm1_1(filename: &str, threshold_delta: i32, min_area: f32, max_area: f32, min_probability: f32, non_max_suppression: bool, min_probability_diff: f32) -> Result<types::PtrOfERFilter> {
	string_arg!(filename);
	unsafe { sys::cv_text_createERFilterNM1_const_StringX_int_float_float_float_bool_float(filename.as_ptr(), threshold_delta, min_area, max_area, min_probability, non_max_suppression, min_probability_diff) }.into_result().map(|ptr| types::PtrOfERFilter { ptr })
}

/// Create an Extremal Region Filter for the 2nd stage classifier of N&M algorithm [Neumann12](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Neumann12).
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
pub fn create_er_filter_nm2(cb: &types::PtrOfERFilter_Callback, min_probability: f32) -> Result<types::PtrOfERFilter> {
	unsafe { sys::cv_text_createERFilterNM2_const_Ptr_Callback_X_float(cb.as_raw_PtrOfERFilter_Callback(), min_probability) }.into_result().map(|ptr| types::PtrOfERFilter { ptr })
}

/// Reads an Extremal Region Filter for the 2nd stage classifier of N&M algorithm
///    from the provided path e.g. /path/to/cpp/trained_classifierNM2.xml
/// 
/// Create an Extremal Region Filter for the 2nd stage classifier of N&M algorithm [Neumann12](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Neumann12).
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
pub fn create_er_filter_nm2_1(filename: &str, min_probability: f32) -> Result<types::PtrOfERFilter> {
	string_arg!(filename);
	unsafe { sys::cv_text_createERFilterNM2_const_StringX_float(filename.as_ptr(), min_probability) }.into_result().map(|ptr| types::PtrOfERFilter { ptr })
}

pub fn create_ocrhmm_transitions_table_1(vocabulary: &str, lexicon: &mut types::VectorOfString) -> Result<core::Mat> {
	string_arg!(vocabulary);
	unsafe { sys::cv_text_createOCRHMMTransitionsTable_const_StringX_vector_String_X(vocabulary.as_ptr(), lexicon.as_raw_VectorOfString()) }.into_result().map(|ptr| core::Mat { ptr })
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
pub fn create_ocrhmm_transitions_table(vocabulary: &mut String, lexicon: &mut types::VectorOfString, transition_probabilities_table: &mut dyn core::ToOutputArray) -> Result<()> {
	string_arg_output_send!(via vocabulary_via);
	output_array_arg!(transition_probabilities_table);
	let out = unsafe { sys::cv_text_createOCRHMMTransitionsTable_stringX_vector_string_X_const__OutputArrayX(&mut vocabulary_via, lexicon.as_raw_VectorOfString(), transition_probabilities_table.as_raw__OutputArray()) }.into_result();
	string_arg_output_receive!(out, vocabulary_via => vocabulary);
	out
}

/// Extracts text regions from image.
/// 
/// ## Parameters
/// * image: Source image where text blocks needs to be extracted from.  Should be CV_8UC3 (color).
/// * er_filter1: Extremal Region Filter for the 1st stage classifier of N&M algorithm [Neumann12](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Neumann12)
/// * er_filter2: Extremal Region Filter for the 2nd stage classifier of N&M algorithm [Neumann12](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Neumann12)
/// * groups_rects: Output list of rectangle blocks with text
/// * method: Grouping method (see text::erGrouping_Modes). Can be one of ERGROUPING_ORIENTATION_HORIZ, ERGROUPING_ORIENTATION_ANY.
/// * filename: The XML or YAML file with the classifier model (e.g. samples/trained_classifier_erGrouping.xml). Only to use when grouping method is ERGROUPING_ORIENTATION_ANY.
/// * minProbability: The minimum probability for accepting a group. Only to use when grouping method is ERGROUPING_ORIENTATION_ANY.
/// 
/// ## C++ default parameters
/// * method: ERGROUPING_ORIENTATION_HORIZ
/// * filename: String()
/// * min_probability: (float)0.5
pub fn detect_regions_1(image: &dyn core::ToInputArray, er_filter1: &types::PtrOfERFilter, er_filter2: &types::PtrOfERFilter, groups_rects: &mut types::VectorOfRect, method: i32, filename: &str, min_probability: f32) -> Result<()> {
	input_array_arg!(image);
	string_arg!(filename);
	unsafe { sys::cv_text_detectRegions_const__InputArrayX_const_Ptr_ERFilter_X_const_Ptr_ERFilter_X_vector_Rect_X_int_const_StringX_float(image.as_raw__InputArray(), er_filter1.as_raw_PtrOfERFilter(), er_filter2.as_raw_PtrOfERFilter(), groups_rects.as_raw_VectorOfRect(), method, filename.as_ptr(), min_probability) }.into_result()
}

pub fn detect_regions(image: &dyn core::ToInputArray, er_filter1: &types::PtrOfERFilter, er_filter2: &types::PtrOfERFilter, regions: &mut types::VectorOfVectorOfPoint) -> Result<()> {
	input_array_arg!(image);
	unsafe { sys::cv_text_detectRegions_const__InputArrayX_const_Ptr_ERFilter_X_const_Ptr_ERFilter_X_vector_vector_Point__X(image.as_raw__InputArray(), er_filter1.as_raw_PtrOfERFilter(), er_filter2.as_raw_PtrOfERFilter(), regions.as_raw_VectorOfVectorOfPoint()) }.into_result()
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
pub fn er_grouping(img: &dyn core::ToInputArray, channels: &dyn core::ToInputArray, regions: &mut types::VectorOfVectorOfERStat, groups: &mut types::VectorOfVectorOfVec2i, groups_rects: &mut types::VectorOfRect, method: i32, filename: &str, min_probablity: f32) -> Result<()> {
	input_array_arg!(img);
	input_array_arg!(channels);
	string_arg!(filename);
	unsafe { sys::cv_text_erGrouping_const__InputArrayX_const__InputArrayX_vector_vector_ERStat__X_vector_vector_Vec2i__X_vector_Rect_X_int_const_stringX_float(img.as_raw__InputArray(), channels.as_raw__InputArray(), regions.as_raw_VectorOfVectorOfERStat(), groups.as_raw_VectorOfVectorOfVec2i(), groups_rects.as_raw_VectorOfRect(), method, filename.as_ptr(), min_probablity) }.into_result()
}

/// ## C++ default parameters
/// * method: ERGROUPING_ORIENTATION_HORIZ
/// * filename: String()
/// * min_probablity: (float)0.5
pub fn er_grouping_1(image: &dyn core::ToInputArray, channel: &dyn core::ToInputArray, regions: types::VectorOfVectorOfPoint, groups_rects: &mut types::VectorOfRect, method: i32, filename: &str, min_probablity: f32) -> Result<()> {
	input_array_arg!(image);
	input_array_arg!(channel);
	string_arg!(filename);
	unsafe { sys::cv_text_erGrouping_const__InputArrayX_const__InputArrayX_vector_vector_Point___vector_Rect_X_int_const_StringX_float(image.as_raw__InputArray(), channel.as_raw__InputArray(), regions.as_raw_VectorOfVectorOfPoint(), groups_rects.as_raw_VectorOfRect(), method, filename.as_ptr(), min_probablity) }.into_result()
}

/// Allow to implicitly load the default classifier when creating an ERFilter object.
/// 
/// ## Parameters
/// * filename: The XML or YAML file with the classifier model (e.g. trained_classifierNM1.xml)
/// 
/// returns a pointer to ERFilter::Callback.
pub fn load_classifier_nm1(filename: &str) -> Result<types::PtrOfERFilter_Callback> {
	string_arg!(filename);
	unsafe { sys::cv_text_loadClassifierNM1_const_StringX(filename.as_ptr()) }.into_result().map(|ptr| types::PtrOfERFilter_Callback { ptr })
}

/// Allow to implicitly load the default classifier when creating an ERFilter object.
/// 
/// ## Parameters
/// * filename: The XML or YAML file with the classifier model (e.g. trained_classifierNM2.xml)
/// 
/// returns a pointer to ERFilter::Callback.
pub fn load_classifier_nm2(filename: &str) -> Result<types::PtrOfERFilter_Callback> {
	string_arg!(filename);
	unsafe { sys::cv_text_loadClassifierNM2_const_StringX(filename.as_ptr()) }.into_result().map(|ptr| types::PtrOfERFilter_Callback { ptr })
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
pub fn load_ocr_beam_search_classifier_cnn(filename: &str) -> Result<types::PtrOfOCRBeamSearchDecoder_ClassifierCallback> {
	string_arg!(filename);
	unsafe { sys::cv_text_loadOCRBeamSearchClassifierCNN_const_StringX(filename.as_ptr()) }.into_result().map(|ptr| types::PtrOfOCRBeamSearchDecoder_ClassifierCallback { ptr })
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
pub fn load_ocrhmm_classifier_cnn(filename: &str) -> Result<types::PtrOfOCRHMMDecoder_ClassifierCallback> {
	string_arg!(filename);
	unsafe { sys::cv_text_loadOCRHMMClassifierCNN_const_StringX(filename.as_ptr()) }.into_result().map(|ptr| types::PtrOfOCRHMMDecoder_ClassifierCallback { ptr })
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
pub fn load_ocrhmm_classifier_nm(filename: &str) -> Result<types::PtrOfOCRHMMDecoder_ClassifierCallback> {
	string_arg!(filename);
	unsafe { sys::cv_text_loadOCRHMMClassifierNM_const_StringX(filename.as_ptr()) }.into_result().map(|ptr| types::PtrOfOCRHMMDecoder_ClassifierCallback { ptr })
}

/// Allow to implicitly load the default character classifier when creating an OCRHMMDecoder object.
/// 
/// ## Parameters
/// * filename: The XML or YAML file with the classifier model (e.g. OCRBeamSearch_CNN_model_data.xml.gz)
/// 
/// * classifier: Can be one of classifier_type enum values.
pub fn load_ocrhmm_classifier(filename: &str, classifier: i32) -> Result<types::PtrOfOCRHMMDecoder_ClassifierCallback> {
	string_arg!(filename);
	unsafe { sys::cv_text_loadOCRHMMClassifier_const_StringX_int(filename.as_ptr(), classifier) }.into_result().map(|ptr| types::PtrOfOCRHMMDecoder_ClassifierCallback { ptr })
}

pub trait BaseOCR {
	fn as_raw_BaseOCR(&self) -> *mut c_void;
	/// ## C++ default parameters
	/// * component_rects: NULL
	/// * component_texts: NULL
	/// * component_confidences: NULL
	/// * component_level: 0
	fn run(&mut self, image: &mut core::Mat, output_text: &mut String, component_rects: &mut types::VectorOfRect, component_texts: &mut types::VectorOfString, component_confidences: &mut types::VectorOff32, component_level: i32) -> Result<()> {
		string_arg_output_send!(via output_text_via);
		let out = unsafe { sys::cv_text_BaseOCR_run_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(self.as_raw_BaseOCR(), image.as_raw_Mat(), &mut output_text_via, component_rects.as_raw_VectorOfRect(), component_texts.as_raw_VectorOfString(), component_confidences.as_raw_VectorOff32(), component_level) }.into_result();
		string_arg_output_receive!(out, output_text_via => output_text);
		out
	}
	
	/// ## C++ default parameters
	/// * component_rects: NULL
	/// * component_texts: NULL
	/// * component_confidences: NULL
	/// * component_level: 0
	fn run_1(&mut self, image: &mut core::Mat, mask: &mut core::Mat, output_text: &mut String, component_rects: &mut types::VectorOfRect, component_texts: &mut types::VectorOfString, component_confidences: &mut types::VectorOff32, component_level: i32) -> Result<()> {
		string_arg_output_send!(via output_text_via);
		let out = unsafe { sys::cv_text_BaseOCR_run_MatX_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(self.as_raw_BaseOCR(), image.as_raw_Mat(), mask.as_raw_Mat(), &mut output_text_via, component_rects.as_raw_VectorOfRect(), component_texts.as_raw_VectorOfString(), component_confidences.as_raw_VectorOff32(), component_level) }.into_result();
		string_arg_output_receive!(out, output_text_via => output_text);
		out
	}
	
}

/// Base class for 1st and 2nd stages of Neumann and Matas scene text detection algorithm [Neumann12](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Neumann12). :
/// 
/// Extracts the component tree (if needed) and filter the extremal regions (ER's) by using a given classifier.
pub trait ERFilter: core::AlgorithmTrait {
	fn as_raw_ERFilter(&self) -> *mut c_void;
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
	fn run(&mut self, image: &dyn core::ToInputArray, regions: &mut types::VectorOfERStat) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_text_ERFilter_run_const__InputArrayX_vector_ERStat_X(self.as_raw_ERFilter(), image.as_raw__InputArray(), regions.as_raw_VectorOfERStat()) }.into_result()
	}
	
	/// set/get methods to set the algorithm properties,
	fn set_callback(&mut self, cb: &types::PtrOfERFilter_Callback) -> Result<()> {
		unsafe { sys::cv_text_ERFilter_setCallback_const_Ptr_Callback_X(self.as_raw_ERFilter(), cb.as_raw_PtrOfERFilter_Callback()) }.into_result()
	}
	
	fn set_threshold_delta(&mut self, threshold_delta: i32) -> Result<()> {
		unsafe { sys::cv_text_ERFilter_setThresholdDelta_int(self.as_raw_ERFilter(), threshold_delta) }.into_result()
	}
	
	fn set_min_area(&mut self, min_area: f32) -> Result<()> {
		unsafe { sys::cv_text_ERFilter_setMinArea_float(self.as_raw_ERFilter(), min_area) }.into_result()
	}
	
	fn set_max_area(&mut self, max_area: f32) -> Result<()> {
		unsafe { sys::cv_text_ERFilter_setMaxArea_float(self.as_raw_ERFilter(), max_area) }.into_result()
	}
	
	fn set_min_probability(&mut self, min_probability: f32) -> Result<()> {
		unsafe { sys::cv_text_ERFilter_setMinProbability_float(self.as_raw_ERFilter(), min_probability) }.into_result()
	}
	
	fn set_min_probability_diff(&mut self, min_probability_diff: f32) -> Result<()> {
		unsafe { sys::cv_text_ERFilter_setMinProbabilityDiff_float(self.as_raw_ERFilter(), min_probability_diff) }.into_result()
	}
	
	fn set_non_max_suppression(&mut self, non_max_suppression: bool) -> Result<()> {
		unsafe { sys::cv_text_ERFilter_setNonMaxSuppression_bool(self.as_raw_ERFilter(), non_max_suppression) }.into_result()
	}
	
	fn get_num_rejected(&self) -> Result<i32> {
		unsafe { sys::cv_text_ERFilter_getNumRejected_const(self.as_raw_ERFilter()) }.into_result()
	}
	
}

/// Callback with the classifier is made a class.
/// 
/// By doing it we hide SVM, Boost etc. Developers can provide their own classifiers to the
/// ERFilter algorithm.
pub trait ERFilter_Callback {
	fn as_raw_ERFilter_Callback(&self) -> *mut c_void;
	/// The classifier must return probability measure for the region.
	/// 
	/// ## Parameters
	/// * stat: :   The region to be classified
	fn eval(&mut self, stat: &crate::text::ERStat) -> Result<f64> {
		unsafe { sys::cv_text_ERFilter_Callback_eval_const_ERStatX(self.as_raw_ERFilter_Callback(), stat.as_raw_ERStat()) }.into_result()
	}
	
}

/// The ERStat structure represents a class-specific Extremal Region (ER).
/// 
/// An ER is a 4-connected set of pixels with all its grey-level values smaller than the values in its
/// outer boundary. A class-specific ER is selected (using a classifier) from all the ER's in the
/// component tree of the image. :
pub trait ERStatTrait {
	fn as_raw_ERStat(&self) -> *mut c_void;
	/// seed point and the threshold (max grey-level value)
	fn pixel(&self) -> i32 {
		unsafe { sys::cv_text_ERStat_pixel_const(self.as_raw_ERStat()) }.into_result().expect("Infallible function failed: pixel")
	}
	
	/// seed point and the threshold (max grey-level value)
	fn set_pixel(&mut self, val: i32) -> () {
		unsafe { sys::cv_text_ERStat_setPixel_int(self.as_raw_ERStat(), val) }.into_result().expect("Infallible function failed: set_pixel")
	}
	
	fn level(&self) -> i32 {
		unsafe { sys::cv_text_ERStat_level_const(self.as_raw_ERStat()) }.into_result().expect("Infallible function failed: level")
	}
	
	fn set_level(&mut self, val: i32) -> () {
		unsafe { sys::cv_text_ERStat_setLevel_int(self.as_raw_ERStat(), val) }.into_result().expect("Infallible function failed: set_level")
	}
	
	/// incrementally computable features
	fn area(&self) -> i32 {
		unsafe { sys::cv_text_ERStat_area_const(self.as_raw_ERStat()) }.into_result().expect("Infallible function failed: area")
	}
	
	/// incrementally computable features
	fn set_area(&mut self, val: i32) -> () {
		unsafe { sys::cv_text_ERStat_setArea_int(self.as_raw_ERStat(), val) }.into_result().expect("Infallible function failed: set_area")
	}
	
	fn perimeter(&self) -> i32 {
		unsafe { sys::cv_text_ERStat_perimeter_const(self.as_raw_ERStat()) }.into_result().expect("Infallible function failed: perimeter")
	}
	
	fn set_perimeter(&mut self, val: i32) -> () {
		unsafe { sys::cv_text_ERStat_setPerimeter_int(self.as_raw_ERStat(), val) }.into_result().expect("Infallible function failed: set_perimeter")
	}
	
	/// Euler's number
	fn euler(&self) -> i32 {
		unsafe { sys::cv_text_ERStat_euler_const(self.as_raw_ERStat()) }.into_result().expect("Infallible function failed: euler")
	}
	
	/// Euler's number
	fn set_euler(&mut self, val: i32) -> () {
		unsafe { sys::cv_text_ERStat_setEuler_int(self.as_raw_ERStat(), val) }.into_result().expect("Infallible function failed: set_euler")
	}
	
	fn rect(&self) -> core::Rect {
		unsafe { sys::cv_text_ERStat_rect_const(self.as_raw_ERStat()) }.into_result().expect("Infallible function failed: rect")
	}
	
	fn set_rect(&mut self, val: core::Rect) -> () {
		unsafe { sys::cv_text_ERStat_setRect_Rect(self.as_raw_ERStat(), &val) }.into_result().expect("Infallible function failed: set_rect")
	}
	
	/// order 1 raw moments to derive the centroid
	fn raw_moments(&mut self) -> &mut [f64; 2] {
		unsafe { sys::cv_text_ERStat_raw_moments(self.as_raw_ERStat()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: raw_moments")
	}
	
	/// order 2 central moments to construct the covariance matrix
	fn central_moments(&mut self) -> &mut [f64; 3] {
		unsafe { sys::cv_text_ERStat_central_moments(self.as_raw_ERStat()) }.into_result().and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, "Function returned Null pointer".to_string()))).expect("Infallible function failed: central_moments")
	}
	
	/// median of the crossings at three different height levels
	fn med_crossings(&self) -> f32 {
		unsafe { sys::cv_text_ERStat_med_crossings_const(self.as_raw_ERStat()) }.into_result().expect("Infallible function failed: med_crossings")
	}
	
	/// median of the crossings at three different height levels
	fn set_med_crossings(&mut self, val: f32) -> () {
		unsafe { sys::cv_text_ERStat_setMed_crossings_float(self.as_raw_ERStat(), val) }.into_result().expect("Infallible function failed: set_med_crossings")
	}
	
	/// 2nd stage features
	fn hole_area_ratio(&self) -> f32 {
		unsafe { sys::cv_text_ERStat_hole_area_ratio_const(self.as_raw_ERStat()) }.into_result().expect("Infallible function failed: hole_area_ratio")
	}
	
	/// 2nd stage features
	fn set_hole_area_ratio(&mut self, val: f32) -> () {
		unsafe { sys::cv_text_ERStat_setHole_area_ratio_float(self.as_raw_ERStat(), val) }.into_result().expect("Infallible function failed: set_hole_area_ratio")
	}
	
	fn convex_hull_ratio(&self) -> f32 {
		unsafe { sys::cv_text_ERStat_convex_hull_ratio_const(self.as_raw_ERStat()) }.into_result().expect("Infallible function failed: convex_hull_ratio")
	}
	
	fn set_convex_hull_ratio(&mut self, val: f32) -> () {
		unsafe { sys::cv_text_ERStat_setConvex_hull_ratio_float(self.as_raw_ERStat(), val) }.into_result().expect("Infallible function failed: set_convex_hull_ratio")
	}
	
	fn num_inflexion_points(&self) -> f32 {
		unsafe { sys::cv_text_ERStat_num_inflexion_points_const(self.as_raw_ERStat()) }.into_result().expect("Infallible function failed: num_inflexion_points")
	}
	
	fn set_num_inflexion_points(&mut self, val: f32) -> () {
		unsafe { sys::cv_text_ERStat_setNum_inflexion_points_float(self.as_raw_ERStat(), val) }.into_result().expect("Infallible function failed: set_num_inflexion_points")
	}
	
	fn pixels(&mut self) -> types::VectorOfi32 {
		unsafe { sys::cv_text_ERStat_pixels(self.as_raw_ERStat()) }.into_result().map(|ptr| types::VectorOfi32 { ptr }).expect("Infallible function failed: pixels")
	}
	
	fn set_pixels(&mut self, val: &mut types::VectorOfi32) -> () {
		unsafe { sys::cv_text_ERStat_setPixels_vector_int_X(self.as_raw_ERStat(), val.as_raw_VectorOfi32()) }.into_result().expect("Infallible function failed: set_pixels")
	}
	
	/// probability that the ER belongs to the class we are looking for
	fn probability(&self) -> f64 {
		unsafe { sys::cv_text_ERStat_probability_const(self.as_raw_ERStat()) }.into_result().expect("Infallible function failed: probability")
	}
	
	/// probability that the ER belongs to the class we are looking for
	fn set_probability(&mut self, val: f64) -> () {
		unsafe { sys::cv_text_ERStat_setProbability_double(self.as_raw_ERStat(), val) }.into_result().expect("Infallible function failed: set_probability")
	}
	
	/// pointers preserving the tree structure of the component tree
	fn parent(&mut self) -> crate::text::ERStat {
		unsafe { sys::cv_text_ERStat_parent(self.as_raw_ERStat()) }.into_result().map(|ptr| crate::text::ERStat { ptr }).expect("Infallible function failed: parent")
	}
	
	/// pointers preserving the tree structure of the component tree
	fn set_parent(&mut self, val: &mut crate::text::ERStat) -> () {
		unsafe { sys::cv_text_ERStat_setParent_ERStatX(self.as_raw_ERStat(), val.as_raw_ERStat()) }.into_result().expect("Infallible function failed: set_parent")
	}
	
	fn child(&mut self) -> crate::text::ERStat {
		unsafe { sys::cv_text_ERStat_child(self.as_raw_ERStat()) }.into_result().map(|ptr| crate::text::ERStat { ptr }).expect("Infallible function failed: child")
	}
	
	fn set_child(&mut self, val: &mut crate::text::ERStat) -> () {
		unsafe { sys::cv_text_ERStat_setChild_ERStatX(self.as_raw_ERStat(), val.as_raw_ERStat()) }.into_result().expect("Infallible function failed: set_child")
	}
	
	fn next(&mut self) -> crate::text::ERStat {
		unsafe { sys::cv_text_ERStat_next(self.as_raw_ERStat()) }.into_result().map(|ptr| crate::text::ERStat { ptr }).expect("Infallible function failed: next")
	}
	
	fn set_next(&mut self, val: &mut crate::text::ERStat) -> () {
		unsafe { sys::cv_text_ERStat_setNext_ERStatX(self.as_raw_ERStat(), val.as_raw_ERStat()) }.into_result().expect("Infallible function failed: set_next")
	}
	
	fn prev(&mut self) -> crate::text::ERStat {
		unsafe { sys::cv_text_ERStat_prev(self.as_raw_ERStat()) }.into_result().map(|ptr| crate::text::ERStat { ptr }).expect("Infallible function failed: prev")
	}
	
	fn set_prev(&mut self, val: &mut crate::text::ERStat) -> () {
		unsafe { sys::cv_text_ERStat_setPrev_ERStatX(self.as_raw_ERStat(), val.as_raw_ERStat()) }.into_result().expect("Infallible function failed: set_prev")
	}
	
	/// whenever the regions is a local maxima of the probability
	fn local_maxima(&self) -> bool {
		unsafe { sys::cv_text_ERStat_local_maxima_const(self.as_raw_ERStat()) }.into_result().expect("Infallible function failed: local_maxima")
	}
	
	/// whenever the regions is a local maxima of the probability
	fn set_local_maxima(&mut self, val: bool) -> () {
		unsafe { sys::cv_text_ERStat_setLocal_maxima_bool(self.as_raw_ERStat(), val) }.into_result().expect("Infallible function failed: set_local_maxima")
	}
	
	fn max_probability_ancestor(&mut self) -> crate::text::ERStat {
		unsafe { sys::cv_text_ERStat_max_probability_ancestor(self.as_raw_ERStat()) }.into_result().map(|ptr| crate::text::ERStat { ptr }).expect("Infallible function failed: max_probability_ancestor")
	}
	
	fn set_max_probability_ancestor(&mut self, val: &mut crate::text::ERStat) -> () {
		unsafe { sys::cv_text_ERStat_setMax_probability_ancestor_ERStatX(self.as_raw_ERStat(), val.as_raw_ERStat()) }.into_result().expect("Infallible function failed: set_max_probability_ancestor")
	}
	
	fn min_probability_ancestor(&mut self) -> crate::text::ERStat {
		unsafe { sys::cv_text_ERStat_min_probability_ancestor(self.as_raw_ERStat()) }.into_result().map(|ptr| crate::text::ERStat { ptr }).expect("Infallible function failed: min_probability_ancestor")
	}
	
	fn set_min_probability_ancestor(&mut self, val: &mut crate::text::ERStat) -> () {
		unsafe { sys::cv_text_ERStat_setMin_probability_ancestor_ERStatX(self.as_raw_ERStat(), val.as_raw_ERStat()) }.into_result().expect("Infallible function failed: set_min_probability_ancestor")
	}
	
}

/// The ERStat structure represents a class-specific Extremal Region (ER).
/// 
/// An ER is a 4-connected set of pixels with all its grey-level values smaller than the values in its
/// outer boundary. A class-specific ER is selected (using a classifier) from all the ER's in the
/// component tree of the image. :
pub struct ERStat {
	pub(crate) ptr: *mut c_void
}

impl Drop for ERStat {
	fn drop(&mut self) {
		extern "C" { fn cv_ERStat_delete(instance: *mut c_void); }
		unsafe { cv_ERStat_delete(self.as_raw_ERStat()) };
	}
}

impl ERStat {
	pub fn as_raw_ERStat(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for ERStat {}

impl crate::text::ERStatTrait for ERStat {
	fn as_raw_ERStat(&self) -> *mut c_void { self.ptr }
}

impl ERStat {
	/// Constructor
	/// 
	/// ## C++ default parameters
	/// * level: 256
	/// * pixel: 0
	/// * x: 0
	/// * y: 0
	pub fn new(level: i32, pixel: i32, x: i32, y: i32) -> Result<crate::text::ERStat> {
		unsafe { sys::cv_text_ERStat_ERStat_int_int_int_int(level, pixel, x, y) }.into_result().map(|ptr| crate::text::ERStat { ptr })
	}
	
}

/// OCRBeamSearchDecoder class provides an interface for OCR using Beam Search algorithm.
/// 
/// 
/// Note:
///    *   (C++) An example on using OCRBeamSearchDecoder recognition combined with scene text detection can
///        be found at the demo sample:
///        <https://github.com/opencv/opencv_contrib/blob/master/modules/text/samples/word_recognition.cpp>
pub trait OCRBeamSearchDecoderTrait: crate::text::BaseOCR {
	fn as_raw_OCRBeamSearchDecoder(&self) -> *mut c_void;
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
	fn run(&mut self, image: &mut core::Mat, output_text: &mut String, component_rects: &mut types::VectorOfRect, component_texts: &mut types::VectorOfString, component_confidences: &mut types::VectorOff32, component_level: i32) -> Result<()> {
		string_arg_output_send!(via output_text_via);
		let out = unsafe { sys::cv_text_OCRBeamSearchDecoder_run_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(self.as_raw_OCRBeamSearchDecoder(), image.as_raw_Mat(), &mut output_text_via, component_rects.as_raw_VectorOfRect(), component_texts.as_raw_VectorOfString(), component_confidences.as_raw_VectorOff32(), component_level) }.into_result();
		string_arg_output_receive!(out, output_text_via => output_text);
		out
	}
	
	/// ## C++ default parameters
	/// * component_rects: NULL
	/// * component_texts: NULL
	/// * component_confidences: NULL
	/// * component_level: 0
	fn run_1(&mut self, image: &mut core::Mat, mask: &mut core::Mat, output_text: &mut String, component_rects: &mut types::VectorOfRect, component_texts: &mut types::VectorOfString, component_confidences: &mut types::VectorOff32, component_level: i32) -> Result<()> {
		string_arg_output_send!(via output_text_via);
		let out = unsafe { sys::cv_text_OCRBeamSearchDecoder_run_MatX_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(self.as_raw_OCRBeamSearchDecoder(), image.as_raw_Mat(), mask.as_raw_Mat(), &mut output_text_via, component_rects.as_raw_VectorOfRect(), component_texts.as_raw_VectorOfString(), component_confidences.as_raw_VectorOff32(), component_level) }.into_result();
		string_arg_output_receive!(out, output_text_via => output_text);
		out
	}
	
	/// ## C++ default parameters
	/// * component_level: 0
	fn run_2(&mut self, image: &dyn core::ToInputArray, min_confidence: i32, component_level: i32) -> Result<String> {
		input_array_arg!(image);
		unsafe { sys::cv_text_OCRBeamSearchDecoder_run_const__InputArrayX_int_int(self.as_raw_OCRBeamSearchDecoder(), image.as_raw__InputArray(), min_confidence, component_level) }.into_result().map(crate::templ::receive_string)
	}
	
	/// ## C++ default parameters
	/// * component_level: 0
	fn run_3(&mut self, image: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, min_confidence: i32, component_level: i32) -> Result<String> {
		input_array_arg!(image);
		input_array_arg!(mask);
		unsafe { sys::cv_text_OCRBeamSearchDecoder_run_const__InputArrayX_const__InputArrayX_int_int(self.as_raw_OCRBeamSearchDecoder(), image.as_raw__InputArray(), mask.as_raw__InputArray(), min_confidence, component_level) }.into_result().map(crate::templ::receive_string)
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
	pub(crate) ptr: *mut c_void
}

impl Drop for OCRBeamSearchDecoder {
	fn drop(&mut self) {
		extern "C" { fn cv_OCRBeamSearchDecoder_delete(instance: *mut c_void); }
		unsafe { cv_OCRBeamSearchDecoder_delete(self.as_raw_OCRBeamSearchDecoder()) };
	}
}

impl OCRBeamSearchDecoder {
	pub fn as_raw_OCRBeamSearchDecoder(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for OCRBeamSearchDecoder {}

impl crate::text::BaseOCR for OCRBeamSearchDecoder {
	fn as_raw_BaseOCR(&self) -> *mut c_void { self.ptr }
}

impl crate::text::OCRBeamSearchDecoderTrait for OCRBeamSearchDecoder {
	fn as_raw_OCRBeamSearchDecoder(&self) -> *mut c_void { self.ptr }
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
	pub fn create(classifier: types::PtrOfOCRBeamSearchDecoder_ClassifierCallback, vocabulary: &str, transition_probabilities_table: &dyn core::ToInputArray, emission_probabilities_table: &dyn core::ToInputArray, mode: crate::text::decoder_mode, beam_size: i32) -> Result<types::PtrOfOCRBeamSearchDecoder> {
		string_arg!(vocabulary);
		input_array_arg!(transition_probabilities_table);
		input_array_arg!(emission_probabilities_table);
		unsafe { sys::cv_text_OCRBeamSearchDecoder_create_Ptr_ClassifierCallback__const_stringX_const__InputArrayX_const__InputArrayX_decoder_mode_int(classifier.as_raw_PtrOfOCRBeamSearchDecoder_ClassifierCallback(), vocabulary.as_ptr(), transition_probabilities_table.as_raw__InputArray(), emission_probabilities_table.as_raw__InputArray(), mode, beam_size) }.into_result().map(|ptr| types::PtrOfOCRBeamSearchDecoder { ptr })
	}
	
	/// ## C++ default parameters
	/// * mode: OCR_DECODER_VITERBI
	/// * beam_size: 500
	pub fn create_1(classifier: types::PtrOfOCRBeamSearchDecoder_ClassifierCallback, vocabulary: &str, transition_probabilities_table: &dyn core::ToInputArray, emission_probabilities_table: &dyn core::ToInputArray, mode: i32, beam_size: i32) -> Result<types::PtrOfOCRBeamSearchDecoder> {
		string_arg!(vocabulary);
		input_array_arg!(transition_probabilities_table);
		input_array_arg!(emission_probabilities_table);
		unsafe { sys::cv_text_OCRBeamSearchDecoder_create_Ptr_ClassifierCallback__const_StringX_const__InputArrayX_const__InputArrayX_int_int(classifier.as_raw_PtrOfOCRBeamSearchDecoder_ClassifierCallback(), vocabulary.as_ptr(), transition_probabilities_table.as_raw__InputArray(), emission_probabilities_table.as_raw__InputArray(), mode, beam_size) }.into_result().map(|ptr| types::PtrOfOCRBeamSearchDecoder { ptr })
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
	pub fn create_2(filename: &str, vocabulary: &str, transition_probabilities_table: &dyn core::ToInputArray, emission_probabilities_table: &dyn core::ToInputArray, mode: i32, beam_size: i32) -> Result<types::PtrOfOCRBeamSearchDecoder> {
		string_arg!(filename);
		string_arg!(vocabulary);
		input_array_arg!(transition_probabilities_table);
		input_array_arg!(emission_probabilities_table);
		unsafe { sys::cv_text_OCRBeamSearchDecoder_create_const_StringX_const_StringX_const__InputArrayX_const__InputArrayX_int_int(filename.as_ptr(), vocabulary.as_ptr(), transition_probabilities_table.as_raw__InputArray(), emission_probabilities_table.as_raw__InputArray(), mode, beam_size) }.into_result().map(|ptr| types::PtrOfOCRBeamSearchDecoder { ptr })
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
pub trait OCRBeamSearchDecoder_ClassifierCallbackTrait {
	fn as_raw_OCRBeamSearchDecoder_ClassifierCallback(&self) -> *mut c_void;
	/// The character classifier must return a (ranked list of) class(es) id('s)
	/// 
	/// ## Parameters
	/// * image: Input image CV_8UC1 or CV_8UC3 with a single letter.
	/// * recognition_probabilities: For each of the N characters found the classifier returns a list with
	/// class probabilities for each class.
	/// * oversegmentation: The classifier returns a list of N+1 character locations' x-coordinates,
	/// including 0 as start-sequence location.
	fn eval(&mut self, image: &dyn core::ToInputArray, recognition_probabilities: &mut types::VectorOfVectorOff64, oversegmentation: &mut types::VectorOfi32) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_text_OCRBeamSearchDecoder_ClassifierCallback_eval_const__InputArrayX_vector_vector_double__X_vector_int_X(self.as_raw_OCRBeamSearchDecoder_ClassifierCallback(), image.as_raw__InputArray(), recognition_probabilities.as_raw_VectorOfVectorOff64(), oversegmentation.as_raw_VectorOfi32()) }.into_result()
	}
	
	fn get_window_size(&mut self) -> Result<i32> {
		unsafe { sys::cv_text_OCRBeamSearchDecoder_ClassifierCallback_getWindowSize(self.as_raw_OCRBeamSearchDecoder_ClassifierCallback()) }.into_result()
	}
	
	fn get_step_size(&mut self) -> Result<i32> {
		unsafe { sys::cv_text_OCRBeamSearchDecoder_ClassifierCallback_getStepSize(self.as_raw_OCRBeamSearchDecoder_ClassifierCallback()) }.into_result()
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
	pub(crate) ptr: *mut c_void
}

impl Drop for OCRBeamSearchDecoder_ClassifierCallback {
	fn drop(&mut self) {
		extern "C" { fn cv_OCRBeamSearchDecoder_ClassifierCallback_delete(instance: *mut c_void); }
		unsafe { cv_OCRBeamSearchDecoder_ClassifierCallback_delete(self.as_raw_OCRBeamSearchDecoder_ClassifierCallback()) };
	}
}

impl OCRBeamSearchDecoder_ClassifierCallback {
	pub fn as_raw_OCRBeamSearchDecoder_ClassifierCallback(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for OCRBeamSearchDecoder_ClassifierCallback {}

impl crate::text::OCRBeamSearchDecoder_ClassifierCallbackTrait for OCRBeamSearchDecoder_ClassifierCallback {
	fn as_raw_OCRBeamSearchDecoder_ClassifierCallback(&self) -> *mut c_void { self.ptr }
}

impl OCRBeamSearchDecoder_ClassifierCallback {
}

/// OCRHMMDecoder class provides an interface for OCR using Hidden Markov Models.
/// 
/// 
/// Note:
///    *   (C++) An example on using OCRHMMDecoder recognition combined with scene text detection can
///        be found at the webcam_demo sample:
///        <https://github.com/opencv/opencv_contrib/blob/master/modules/text/samples/webcam_demo.cpp>
pub trait OCRHMMDecoderTrait: crate::text::BaseOCR {
	fn as_raw_OCRHMMDecoder(&self) -> *mut c_void;
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
	fn run(&mut self, image: &mut core::Mat, output_text: &mut String, component_rects: &mut types::VectorOfRect, component_texts: &mut types::VectorOfString, component_confidences: &mut types::VectorOff32, component_level: i32) -> Result<()> {
		string_arg_output_send!(via output_text_via);
		let out = unsafe { sys::cv_text_OCRHMMDecoder_run_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(self.as_raw_OCRHMMDecoder(), image.as_raw_Mat(), &mut output_text_via, component_rects.as_raw_VectorOfRect(), component_texts.as_raw_VectorOfString(), component_confidences.as_raw_VectorOff32(), component_level) }.into_result();
		string_arg_output_receive!(out, output_text_via => output_text);
		out
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
	fn run_1(&mut self, image: &mut core::Mat, mask: &mut core::Mat, output_text: &mut String, component_rects: &mut types::VectorOfRect, component_texts: &mut types::VectorOfString, component_confidences: &mut types::VectorOff32, component_level: i32) -> Result<()> {
		string_arg_output_send!(via output_text_via);
		let out = unsafe { sys::cv_text_OCRHMMDecoder_run_MatX_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(self.as_raw_OCRHMMDecoder(), image.as_raw_Mat(), mask.as_raw_Mat(), &mut output_text_via, component_rects.as_raw_VectorOfRect(), component_texts.as_raw_VectorOfString(), component_confidences.as_raw_VectorOff32(), component_level) }.into_result();
		string_arg_output_receive!(out, output_text_via => output_text);
		out
	}
	
	/// ## C++ default parameters
	/// * component_level: 0
	fn run_2(&mut self, image: &dyn core::ToInputArray, min_confidence: i32, component_level: i32) -> Result<String> {
		input_array_arg!(image);
		unsafe { sys::cv_text_OCRHMMDecoder_run_const__InputArrayX_int_int(self.as_raw_OCRHMMDecoder(), image.as_raw__InputArray(), min_confidence, component_level) }.into_result().map(crate::templ::receive_string)
	}
	
	/// ## C++ default parameters
	/// * component_level: 0
	fn run_3(&mut self, image: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, min_confidence: i32, component_level: i32) -> Result<String> {
		input_array_arg!(image);
		input_array_arg!(mask);
		unsafe { sys::cv_text_OCRHMMDecoder_run_const__InputArrayX_const__InputArrayX_int_int(self.as_raw_OCRHMMDecoder(), image.as_raw__InputArray(), mask.as_raw__InputArray(), min_confidence, component_level) }.into_result().map(crate::templ::receive_string)
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
	pub(crate) ptr: *mut c_void
}

impl Drop for OCRHMMDecoder {
	fn drop(&mut self) {
		extern "C" { fn cv_OCRHMMDecoder_delete(instance: *mut c_void); }
		unsafe { cv_OCRHMMDecoder_delete(self.as_raw_OCRHMMDecoder()) };
	}
}

impl OCRHMMDecoder {
	pub fn as_raw_OCRHMMDecoder(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for OCRHMMDecoder {}

impl crate::text::BaseOCR for OCRHMMDecoder {
	fn as_raw_BaseOCR(&self) -> *mut c_void { self.ptr }
}

impl crate::text::OCRHMMDecoderTrait for OCRHMMDecoder {
	fn as_raw_OCRHMMDecoder(&self) -> *mut c_void { self.ptr }
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
	pub fn create(classifier: types::PtrOfOCRHMMDecoder_ClassifierCallback, vocabulary: &str, transition_probabilities_table: &dyn core::ToInputArray, emission_probabilities_table: &dyn core::ToInputArray, mode: crate::text::decoder_mode) -> Result<types::PtrOfOCRHMMDecoder> {
		string_arg!(vocabulary);
		input_array_arg!(transition_probabilities_table);
		input_array_arg!(emission_probabilities_table);
		unsafe { sys::cv_text_OCRHMMDecoder_create_Ptr_ClassifierCallback__const_stringX_const__InputArrayX_const__InputArrayX_decoder_mode(classifier.as_raw_PtrOfOCRHMMDecoder_ClassifierCallback(), vocabulary.as_ptr(), transition_probabilities_table.as_raw__InputArray(), emission_probabilities_table.as_raw__InputArray(), mode) }.into_result().map(|ptr| types::PtrOfOCRHMMDecoder { ptr })
	}
	
	/// ## C++ default parameters
	/// * mode: OCR_DECODER_VITERBI
	pub fn create_1(classifier: types::PtrOfOCRHMMDecoder_ClassifierCallback, vocabulary: &str, transition_probabilities_table: &dyn core::ToInputArray, emission_probabilities_table: &dyn core::ToInputArray, mode: i32) -> Result<types::PtrOfOCRHMMDecoder> {
		string_arg!(vocabulary);
		input_array_arg!(transition_probabilities_table);
		input_array_arg!(emission_probabilities_table);
		unsafe { sys::cv_text_OCRHMMDecoder_create_Ptr_ClassifierCallback__const_StringX_const__InputArrayX_const__InputArrayX_int(classifier.as_raw_PtrOfOCRHMMDecoder_ClassifierCallback(), vocabulary.as_ptr(), transition_probabilities_table.as_raw__InputArray(), emission_probabilities_table.as_raw__InputArray(), mode) }.into_result().map(|ptr| types::PtrOfOCRHMMDecoder { ptr })
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
	pub fn create_2(filename: &str, vocabulary: &str, transition_probabilities_table: &dyn core::ToInputArray, emission_probabilities_table: &dyn core::ToInputArray, mode: i32, classifier: i32) -> Result<types::PtrOfOCRHMMDecoder> {
		string_arg!(filename);
		string_arg!(vocabulary);
		input_array_arg!(transition_probabilities_table);
		input_array_arg!(emission_probabilities_table);
		unsafe { sys::cv_text_OCRHMMDecoder_create_const_StringX_const_StringX_const__InputArrayX_const__InputArrayX_int_int(filename.as_ptr(), vocabulary.as_ptr(), transition_probabilities_table.as_raw__InputArray(), emission_probabilities_table.as_raw__InputArray(), mode, classifier) }.into_result().map(|ptr| types::PtrOfOCRHMMDecoder { ptr })
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
pub trait OCRHMMDecoder_ClassifierCallbackTrait {
	fn as_raw_OCRHMMDecoder_ClassifierCallback(&self) -> *mut c_void;
	/// The character classifier must return a (ranked list of) class(es) id('s)
	/// 
	/// ## Parameters
	/// * image: Input image CV_8UC1 or CV_8UC3 with a single letter.
	/// * out_class: The classifier returns the character class categorical label, or list of
	/// class labels, to which the input image corresponds.
	/// * out_confidence: The classifier returns the probability of the input image
	/// corresponding to each classes in out_class.
	fn eval(&mut self, image: &dyn core::ToInputArray, out_class: &mut types::VectorOfi32, out_confidence: &mut types::VectorOff64) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_text_OCRHMMDecoder_ClassifierCallback_eval_const__InputArrayX_vector_int_X_vector_double_X(self.as_raw_OCRHMMDecoder_ClassifierCallback(), image.as_raw__InputArray(), out_class.as_raw_VectorOfi32(), out_confidence.as_raw_VectorOff64()) }.into_result()
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
	pub(crate) ptr: *mut c_void
}

impl Drop for OCRHMMDecoder_ClassifierCallback {
	fn drop(&mut self) {
		extern "C" { fn cv_OCRHMMDecoder_ClassifierCallback_delete(instance: *mut c_void); }
		unsafe { cv_OCRHMMDecoder_ClassifierCallback_delete(self.as_raw_OCRHMMDecoder_ClassifierCallback()) };
	}
}

impl OCRHMMDecoder_ClassifierCallback {
	pub fn as_raw_OCRHMMDecoder_ClassifierCallback(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for OCRHMMDecoder_ClassifierCallback {}

impl crate::text::OCRHMMDecoder_ClassifierCallbackTrait for OCRHMMDecoder_ClassifierCallback {
	fn as_raw_OCRHMMDecoder_ClassifierCallback(&self) -> *mut c_void { self.ptr }
}

impl OCRHMMDecoder_ClassifierCallback {
}

/// OCRHolisticWordRecognizer class provides the functionallity of segmented wordspotting.
/// Given a predefined vocabulary , a DictNet is employed to select the most probable
/// word given an input image.
/// 
/// DictNet is described in detail in:
/// Max Jaderberg et al.: Reading Text in the Wild with Convolutional Neural Networks, IJCV 2015
/// http://arxiv.org/abs/1412.1842
pub trait OCRHolisticWordRecognizer: crate::text::BaseOCR {
	fn as_raw_OCRHolisticWordRecognizer(&self) -> *mut c_void;
	/// ## C++ default parameters
	/// * component_rects: NULL
	/// * component_texts: NULL
	/// * component_confidences: NULL
	/// * component_level: OCR_LEVEL_WORD
	fn run(&mut self, image: &mut core::Mat, output_text: &mut String, component_rects: &mut types::VectorOfRect, component_texts: &mut types::VectorOfString, component_confidences: &mut types::VectorOff32, component_level: i32) -> Result<()> {
		string_arg_output_send!(via output_text_via);
		let out = unsafe { sys::cv_text_OCRHolisticWordRecognizer_run_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(self.as_raw_OCRHolisticWordRecognizer(), image.as_raw_Mat(), &mut output_text_via, component_rects.as_raw_VectorOfRect(), component_texts.as_raw_VectorOfString(), component_confidences.as_raw_VectorOff32(), component_level) }.into_result();
		string_arg_output_receive!(out, output_text_via => output_text);
		out
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
	fn run_1(&mut self, image: &mut core::Mat, mask: &mut core::Mat, output_text: &mut String, component_rects: &mut types::VectorOfRect, component_texts: &mut types::VectorOfString, component_confidences: &mut types::VectorOff32, component_level: i32) -> Result<()> {
		string_arg_output_send!(via output_text_via);
		let out = unsafe { sys::cv_text_OCRHolisticWordRecognizer_run_MatX_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(self.as_raw_OCRHolisticWordRecognizer(), image.as_raw_Mat(), mask.as_raw_Mat(), &mut output_text_via, component_rects.as_raw_VectorOfRect(), component_texts.as_raw_VectorOfString(), component_confidences.as_raw_VectorOff32(), component_level) }.into_result();
		string_arg_output_receive!(out, output_text_via => output_text);
		out
	}
	
}

impl dyn OCRHolisticWordRecognizer + '_ {
	/// Creates an instance of the OCRHolisticWordRecognizer class.
	pub fn create(arch_filename: &str, weights_filename: &str, words_filename: &str) -> Result<types::PtrOfOCRHolisticWordRecognizer> {
		string_arg!(arch_filename);
		string_arg!(weights_filename);
		string_arg!(words_filename);
		unsafe { sys::cv_text_OCRHolisticWordRecognizer_create_const_stringX_const_stringX_const_stringX(arch_filename.as_ptr(), weights_filename.as_ptr(), words_filename.as_ptr()) }.into_result().map(|ptr| types::PtrOfOCRHolisticWordRecognizer { ptr })
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
pub trait OCRTesseract: crate::text::BaseOCR {
	fn as_raw_OCRTesseract(&self) -> *mut c_void;
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
	fn run(&mut self, image: &mut core::Mat, output_text: &mut String, component_rects: &mut types::VectorOfRect, component_texts: &mut types::VectorOfString, component_confidences: &mut types::VectorOff32, component_level: i32) -> Result<()> {
		string_arg_output_send!(via output_text_via);
		let out = unsafe { sys::cv_text_OCRTesseract_run_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(self.as_raw_OCRTesseract(), image.as_raw_Mat(), &mut output_text_via, component_rects.as_raw_VectorOfRect(), component_texts.as_raw_VectorOfString(), component_confidences.as_raw_VectorOff32(), component_level) }.into_result();
		string_arg_output_receive!(out, output_text_via => output_text);
		out
	}
	
	/// ## C++ default parameters
	/// * component_rects: NULL
	/// * component_texts: NULL
	/// * component_confidences: NULL
	/// * component_level: 0
	fn run_1(&mut self, image: &mut core::Mat, mask: &mut core::Mat, output_text: &mut String, component_rects: &mut types::VectorOfRect, component_texts: &mut types::VectorOfString, component_confidences: &mut types::VectorOff32, component_level: i32) -> Result<()> {
		string_arg_output_send!(via output_text_via);
		let out = unsafe { sys::cv_text_OCRTesseract_run_MatX_MatX_stringX_vector_Rect_X_vector_string_X_vector_float_X_int(self.as_raw_OCRTesseract(), image.as_raw_Mat(), mask.as_raw_Mat(), &mut output_text_via, component_rects.as_raw_VectorOfRect(), component_texts.as_raw_VectorOfString(), component_confidences.as_raw_VectorOff32(), component_level) }.into_result();
		string_arg_output_receive!(out, output_text_via => output_text);
		out
	}
	
	/// ## C++ default parameters
	/// * component_level: 0
	fn run_2(&mut self, image: &dyn core::ToInputArray, min_confidence: i32, component_level: i32) -> Result<String> {
		input_array_arg!(image);
		unsafe { sys::cv_text_OCRTesseract_run_const__InputArrayX_int_int(self.as_raw_OCRTesseract(), image.as_raw__InputArray(), min_confidence, component_level) }.into_result().map(crate::templ::receive_string)
	}
	
	/// ## C++ default parameters
	/// * component_level: 0
	fn run_3(&mut self, image: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, min_confidence: i32, component_level: i32) -> Result<String> {
		input_array_arg!(image);
		input_array_arg!(mask);
		unsafe { sys::cv_text_OCRTesseract_run_const__InputArrayX_const__InputArrayX_int_int(self.as_raw_OCRTesseract(), image.as_raw__InputArray(), mask.as_raw__InputArray(), min_confidence, component_level) }.into_result().map(crate::templ::receive_string)
	}
	
	fn set_white_list(&mut self, char_whitelist: &str) -> Result<()> {
		string_arg!(char_whitelist);
		unsafe { sys::cv_text_OCRTesseract_setWhiteList_const_StringX(self.as_raw_OCRTesseract(), char_whitelist.as_ptr()) }.into_result()
	}
	
}

impl dyn OCRTesseract + '_ {
	/// Creates an instance of the OCRTesseract class. Initializes Tesseract.
	/// 
	/// ## Parameters
	/// * datapath: the name of the parent directory of tessdata ended with "/", or NULL to use the
	/// system's default directory.
	/// * language: an ISO 639-3 code or NULL will default to "eng".
	/// * char_whitelist: specifies the list of characters used for recognition. NULL defaults to
	/// "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".
	/// * oem: tesseract-ocr offers different OCR Engine Modes (OEM), by default
	/// tesseract::OEM_DEFAULT is used. See the tesseract-ocr API documentation for other possible
	/// values.
	/// * psmode: tesseract-ocr offers different Page Segmentation Modes (PSM) tesseract::PSM_AUTO
	/// (fully automatic layout analysis) is used. See the tesseract-ocr API documentation for other
	/// possible values.
	/// 
	/// ## C++ default parameters
	/// * datapath: NULL
	/// * language: NULL
	/// * char_whitelist: NULL
	/// * oem: OEM_DEFAULT
	/// * psmode: PSM_AUTO
	pub fn create(datapath: &str, language: &str, char_whitelist: &str, oem: i32, psmode: i32) -> Result<types::PtrOfOCRTesseract> {
		string_arg!(datapath);
		string_arg!(language);
		string_arg!(char_whitelist);
		unsafe { sys::cv_text_OCRTesseract_create_const_charX_const_charX_const_charX_int_int(datapath.as_ptr(), language.as_ptr(), char_whitelist.as_ptr(), oem, psmode) }.into_result().map(|ptr| types::PtrOfOCRTesseract { ptr })
	}
	
}
/// An abstract class providing interface for text detection algorithms
pub trait TextDetector {
	fn as_raw_TextDetector(&self) -> *mut c_void;
	/// Method that provides a quick and simple interface to detect text inside an image
	/// 
	/// ## Parameters
	/// * inputImage: an image to process
	/// * Bbox: a vector of Rect that will store the detected word bounding box
	/// * confidence: a vector of float that will be updated with the confidence the classifier has for the selected bounding box
	fn detect(&mut self, input_image: &dyn core::ToInputArray, bbox: &mut types::VectorOfRect, confidence: &mut types::VectorOff32) -> Result<()> {
		input_array_arg!(input_image);
		unsafe { sys::cv_text_TextDetector_detect_const__InputArrayX_vector_Rect_X_vector_float_X(self.as_raw_TextDetector(), input_image.as_raw__InputArray(), bbox.as_raw_VectorOfRect(), confidence.as_raw_VectorOff32()) }.into_result()
	}
	
}

/// TextDetectorCNN class provides the functionallity of text bounding box detection.
/// This class is representing to find bounding boxes of text words given an input image.
/// This class uses OpenCV dnn module to load pre-trained model described in [LiaoSBWL17](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_LiaoSBWL17).
/// The original repository with the modified SSD Caffe version: https://github.com/MhLiao/TextBoxes.
/// Model can be downloaded from [DropBox](https://www.dropbox.com/s/g8pjzv2de9gty8g/TextBoxes_icdar13.caffemodel?dl=0).
/// Modified .prototxt file with the model description can be found in `opencv_contrib/modules/text/samples/textbox.prototxt`.
pub trait TextDetectorCNN: crate::text::TextDetector {
	fn as_raw_TextDetectorCNN(&self) -> *mut c_void;
	/// ## Parameters
	/// * inputImage: an image expected to be a CV_U8C3 of any size
	/// * Bbox: a vector of Rect that will store the detected word bounding box
	/// * confidence: a vector of float that will be updated with the confidence the classifier has for the selected bounding box
	fn detect(&mut self, input_image: &dyn core::ToInputArray, bbox: &mut types::VectorOfRect, confidence: &mut types::VectorOff32) -> Result<()> {
		input_array_arg!(input_image);
		unsafe { sys::cv_text_TextDetectorCNN_detect_const__InputArrayX_vector_Rect_X_vector_float_X(self.as_raw_TextDetectorCNN(), input_image.as_raw__InputArray(), bbox.as_raw_VectorOfRect(), confidence.as_raw_VectorOff32()) }.into_result()
	}
	
}

impl dyn TextDetectorCNN + '_ {
	/// Creates an instance of the TextDetectorCNN class using the provided parameters.
	/// 
	/// ## Parameters
	/// * modelArchFilename: the relative or absolute path to the prototxt file describing the classifiers architecture.
	/// * modelWeightsFilename: the relative or absolute path to the file containing the pretrained weights of the model in caffe-binary form.
	/// * detectionSizes: a list of sizes for multiscale detection. The values`[(300,300),(700,500),(700,300),(700,700),(1600,1600)]` are
	/// recommended in [LiaoSBWL17](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_LiaoSBWL17) to achieve the best quality.
	pub fn create(model_arch_filename: &str, model_weights_filename: &str, detection_sizes: types::VectorOfSize) -> Result<types::PtrOfTextDetectorCNN> {
		string_arg!(model_arch_filename);
		string_arg!(model_weights_filename);
		unsafe { sys::cv_text_TextDetectorCNN_create_const_StringX_const_StringX_vector_Size_(model_arch_filename.as_ptr(), model_weights_filename.as_ptr(), detection_sizes.as_raw_VectorOfSize()) }.into_result().map(|ptr| types::PtrOfTextDetectorCNN { ptr })
	}
	
	/// Creates an instance of the TextDetectorCNN class using the provided parameters.
	/// 
	/// ## Parameters
	/// * modelArchFilename: the relative or absolute path to the prototxt file describing the classifiers architecture.
	/// * modelWeightsFilename: the relative or absolute path to the file containing the pretrained weights of the model in caffe-binary form.
	/// * detectionSizes: a list of sizes for multiscale detection. The values`[(300,300),(700,500),(700,300),(700,700),(1600,1600)]` are
	/// recommended in [LiaoSBWL17](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_LiaoSBWL17) to achieve the best quality.
	/// 
	/// ## Overloaded parameters
	pub fn create_1(model_arch_filename: &str, model_weights_filename: &str) -> Result<types::PtrOfTextDetectorCNN> {
		string_arg!(model_arch_filename);
		string_arg!(model_weights_filename);
		unsafe { sys::cv_text_TextDetectorCNN_create_const_StringX_const_StringX(model_arch_filename.as_ptr(), model_weights_filename.as_ptr()) }.into_result().map(|ptr| types::PtrOfTextDetectorCNN { ptr })
	}
	
}