#![allow(unused_parens)]
//! # Deformable Part-based Models
//! 
//! Discriminatively Trained Part Based Models for Object Detection
//! ---------------------------------------------------------------
//! 
//! The object detector described below has been initially proposed by P.F. Felzenszwalb in
//! [Felzenszwalb2010a](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Felzenszwalb2010a) . It is based on a Dalal-Triggs detector that uses a single filter on histogram
//! of oriented gradients (HOG) features to represent an object category. This detector uses a sliding
//! window approach, where a filter is applied at all positions and scales of an image. The first
//! innovation is enriching the Dalal-Triggs model using a star-structured part-based model defined by a
//! "root" filter (analogous to the Dalal-Triggs filter) plus a set of parts filters and associated
//! deformation models. The score of one of star models at a particular position and scale within an
//! image is the score of the root filter at the given location plus the sum over parts of the maximum,
//! over placements of that part, of the part filter score on its location minus a deformation cost
//! easuring the deviation of the part from its ideal location relative to the root. Both root and part
//! filter scores are defined by the dot product between a filter (a set of weights) and a subwindow of
//! a feature pyramid computed from the input image. Another improvement is a representation of the
//! class of models by a mixture of star models. The score of a mixture model at a particular position
//! and scale is the maximum over components, of the score of that component model at the given
//! location.
//! 
//! The detector was dramatically speeded-up with cascade algorithm proposed by P.F. Felzenszwalb in
//! [Felzenszwalb2010b](https://docs.opencv.org/3.2.0/d0/de3/citelist.html#CITEREF_Felzenszwalb2010b) . The algorithm prunes partial hypotheses using thresholds on their scores.The
//! basic idea of the algorithm is to use a hierarchy of models defined by an ordering of the original
//! model's parts. For a model with (n+1) parts, including the root, a sequence of (n+1) models is
//! obtained. The i-th model in this sequence is defined by the first i parts from the original model.
//! Using this hierarchy, low scoring hypotheses can be pruned after looking at the best configuration
//! of a subset of the parts. Hypotheses that score high under a weak model are evaluated further using
//! a richer model.
//! 
//! In OpenCV there is an C++ implementation of DPM cascade detector.
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::DPMDetector_ObjectDetectionTrait, super::DPMDetector };
}

/// This is a C++ abstract class, it provides external user API to work with DPM.
pub trait DPMDetector {
	fn as_raw_DPMDetector(&self) -> *const c_void;
	fn as_raw_mut_DPMDetector(&mut self) -> *mut c_void;

	fn is_empty(&self) -> Result<bool> {
		unsafe { sys::cv_dpm_DPMDetector_isEmpty_const(self.as_raw_DPMDetector()) }.into_result()
	}
	
	/// Find rectangular regions in the given image that are likely to contain objects of loaded classes
	/// (models) and corresponding confidence levels.
	/// ## Parameters
	/// * image: An image.
	/// * objects: The detections: rectangulars, scores and class IDs.
	fn detect(&mut self, image: &mut core::Mat, objects: &mut core::Vector::<crate::dpm::DPMDetector_ObjectDetection>) -> Result<()> {
		unsafe { sys::cv_dpm_DPMDetector_detect_MatX_vector_ObjectDetection_X(self.as_raw_mut_DPMDetector(), image.as_raw_mut_Mat(), objects.as_raw_mut_VectorOfDPMDetector_ObjectDetection()) }.into_result()
	}
	
	/// Return the class (model) names that were passed in constructor or method load or extracted from
	/// models filenames in those methods.
	fn get_class_names(&self) -> Result<core::Vector::<String>> {
		unsafe { sys::cv_dpm_DPMDetector_getClassNames_const(self.as_raw_DPMDetector()) }.into_result().map(|ptr| unsafe { core::Vector::<String>::from_raw(ptr) })
	}
	
	/// Return a count of loaded models (classes).
	fn get_class_count(&self) -> Result<size_t> {
		unsafe { sys::cv_dpm_DPMDetector_getClassCount_const(self.as_raw_DPMDetector()) }.into_result()
	}
	
}

impl dyn DPMDetector + '_ {
	/// Load the trained models from given .xml files and return cv::Ptr\<DPMDetector\>.
	/// ## Parameters
	/// * filenames: A set of filenames storing the trained detectors (models). Each file contains one
	/// model. See examples of such files here `/opencv_extra/testdata/cv/dpm/VOC2007_Cascade/`.
	/// * classNames: A set of trained models names. If it's empty then the name of each model will be
	/// constructed from the name of file containing the model. E.g. the model stored in
	/// "/home/user/cat.xml" will get the name "cat".
	/// 
	/// ## C++ default parameters
	/// * class_names: std::vector<std::string>()
	pub fn create(filenames: &core::Vector::<String>, class_names: &core::Vector::<String>) -> Result<core::Ptr::<dyn crate::dpm::DPMDetector>> {
		unsafe { sys::cv_dpm_DPMDetector_create_const_vector_string_X_const_vector_string_X(filenames.as_raw_VectorOfString(), class_names.as_raw_VectorOfString()) }.into_result().map(|ptr| unsafe { core::Ptr::<dyn crate::dpm::DPMDetector>::from_raw(ptr) })
	}
	
}
pub trait DPMDetector_ObjectDetectionTrait {
	fn as_raw_DPMDetector_ObjectDetection(&self) -> *const c_void;
	fn as_raw_mut_DPMDetector_ObjectDetection(&mut self) -> *mut c_void;

	fn rect(&self) -> core::Rect {
		unsafe { sys::cv_dpm_DPMDetector_ObjectDetection_rect_const(self.as_raw_DPMDetector_ObjectDetection()) }.into_result().expect("Infallible function failed: rect")
	}
	
	fn set_rect(&mut self, val: core::Rect) -> () {
		unsafe { sys::cv_dpm_DPMDetector_ObjectDetection_setRect_Rect(self.as_raw_mut_DPMDetector_ObjectDetection(), &val) }.into_result().expect("Infallible function failed: set_rect")
	}
	
	fn score(&self) -> f32 {
		unsafe { sys::cv_dpm_DPMDetector_ObjectDetection_score_const(self.as_raw_DPMDetector_ObjectDetection()) }.into_result().expect("Infallible function failed: score")
	}
	
	fn set_score(&mut self, val: f32) -> () {
		unsafe { sys::cv_dpm_DPMDetector_ObjectDetection_setScore_float(self.as_raw_mut_DPMDetector_ObjectDetection(), val) }.into_result().expect("Infallible function failed: set_score")
	}
	
	fn class_id(&self) -> i32 {
		unsafe { sys::cv_dpm_DPMDetector_ObjectDetection_classID_const(self.as_raw_DPMDetector_ObjectDetection()) }.into_result().expect("Infallible function failed: class_id")
	}
	
	fn set_class_id(&mut self, val: i32) -> () {
		unsafe { sys::cv_dpm_DPMDetector_ObjectDetection_setClassID_int(self.as_raw_mut_DPMDetector_ObjectDetection(), val) }.into_result().expect("Infallible function failed: set_class_id")
	}
	
}

pub struct DPMDetector_ObjectDetection {
	ptr: *mut c_void
}

boxed_ptr! { DPMDetector_ObjectDetection }

impl Drop for DPMDetector_ObjectDetection {
	fn drop(&mut self) {
		extern "C" { fn cv_DPMDetector_ObjectDetection_delete(instance: *mut c_void); }
		unsafe { cv_DPMDetector_ObjectDetection_delete(self.as_raw_mut_DPMDetector_ObjectDetection()) };
	}
}

impl DPMDetector_ObjectDetection {
	pub fn as_raw_DPMDetector_ObjectDetection(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_DPMDetector_ObjectDetection(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for DPMDetector_ObjectDetection {}

impl crate::dpm::DPMDetector_ObjectDetectionTrait for DPMDetector_ObjectDetection {
	fn as_raw_DPMDetector_ObjectDetection(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_DPMDetector_ObjectDetection(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl DPMDetector_ObjectDetection {
	pub fn default() -> Result<crate::dpm::DPMDetector_ObjectDetection> {
		unsafe { sys::cv_dpm_DPMDetector_ObjectDetection_ObjectDetection() }.into_result().map(|ptr| unsafe { crate::dpm::DPMDetector_ObjectDetection::from_raw(ptr) })
	}
	
	/// ## C++ default parameters
	/// * class_id: -1
	pub fn new(rect: core::Rect, score: f32, class_id: i32) -> Result<crate::dpm::DPMDetector_ObjectDetection> {
		unsafe { sys::cv_dpm_DPMDetector_ObjectDetection_ObjectDetection_const_RectX_float_int(&rect, score, class_id) }.into_result().map(|ptr| unsafe { crate::dpm::DPMDetector_ObjectDetection::from_raw(ptr) })
	}
	
}
