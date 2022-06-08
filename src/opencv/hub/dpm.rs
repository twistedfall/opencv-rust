#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Deformable Part-based Models
//! 
//! Discriminatively Trained Part Based Models for Object Detection
//! ---------------------------------------------------------------
//! 
//! The object detector described below has been initially proposed by P.F. Felzenszwalb in
//! [Felzenszwalb2010a](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Felzenszwalb2010a) . It is based on a Dalal-Triggs detector that uses a single filter on histogram
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
//! [Felzenszwalb2010b](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Felzenszwalb2010b) . The algorithm prunes partial hypotheses using thresholds on their scores.The
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
	pub use { super::DPMDetector_ObjectDetectionTraitConst, super::DPMDetector_ObjectDetectionTrait, super::DPMDetectorConst, super::DPMDetector };
}

/// This is a C++ abstract class, it provides external user API to work with DPM.
pub trait DPMDetectorConst {
	fn as_raw_DPMDetector(&self) -> *const c_void;

	#[inline]
	fn is_empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dpm_DPMDetector_isEmpty_const(self.as_raw_DPMDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Return the class (model) names that were passed in constructor or method load or extracted from
	/// models filenames in those methods.
	#[inline]
	fn get_class_names(&self) -> Result<core::Vector<String>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dpm_DPMDetector_getClassNames_const(self.as_raw_DPMDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<String>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Return a count of loaded models (classes).
	#[inline]
	fn get_class_count(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dpm_DPMDetector_getClassCount_const(self.as_raw_DPMDetector(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait DPMDetector: crate::dpm::DPMDetectorConst {
	fn as_raw_mut_DPMDetector(&mut self) -> *mut c_void;

	/// Find rectangular regions in the given image that are likely to contain objects of loaded classes
	/// (models) and corresponding confidence levels.
	/// ## Parameters
	/// * image: An image.
	/// * objects: The detections: rectangulars, scores and class IDs.
	#[inline]
	fn detect(&mut self, image: &mut core::Mat, objects: &mut core::Vector<crate::dpm::DPMDetector_ObjectDetection>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dpm_DPMDetector_detect_MatR_vector_ObjectDetection_R(self.as_raw_mut_DPMDetector(), image.as_raw_mut_Mat(), objects.as_raw_mut_VectorOfDPMDetector_ObjectDetection(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	#[inline]
	pub fn create(filenames: &core::Vector<String>, class_names: &core::Vector<String>) -> Result<core::Ptr<dyn crate::dpm::DPMDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dpm_DPMDetector_create_const_vector_string_R_const_vector_string_R(filenames.as_raw_VectorOfString(), class_names.as_raw_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::dpm::DPMDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
pub trait DPMDetector_ObjectDetectionTraitConst {
	fn as_raw_DPMDetector_ObjectDetection(&self) -> *const c_void;

	#[inline]
	fn rect(&self) -> core::Rect {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dpm_DPMDetector_ObjectDetection_getPropRect_const(self.as_raw_DPMDetector_ObjectDetection(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn score(&self) -> f32 {
		let ret = unsafe { sys::cv_dpm_DPMDetector_ObjectDetection_getPropScore_const(self.as_raw_DPMDetector_ObjectDetection()) };
		ret
	}
	
	#[inline]
	fn class_id(&self) -> i32 {
		let ret = unsafe { sys::cv_dpm_DPMDetector_ObjectDetection_getPropClassID_const(self.as_raw_DPMDetector_ObjectDetection()) };
		ret
	}
	
}

pub trait DPMDetector_ObjectDetectionTrait: crate::dpm::DPMDetector_ObjectDetectionTraitConst {
	fn as_raw_mut_DPMDetector_ObjectDetection(&mut self) -> *mut c_void;

	#[inline]
	fn set_rect(&mut self, val: core::Rect) {
		let ret = unsafe { sys::cv_dpm_DPMDetector_ObjectDetection_setPropRect_Rect(self.as_raw_mut_DPMDetector_ObjectDetection(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_score(&mut self, val: f32) {
		let ret = unsafe { sys::cv_dpm_DPMDetector_ObjectDetection_setPropScore_float(self.as_raw_mut_DPMDetector_ObjectDetection(), val) };
		ret
	}
	
	#[inline]
	fn set_class_id(&mut self, val: i32) {
		let ret = unsafe { sys::cv_dpm_DPMDetector_ObjectDetection_setPropClassID_int(self.as_raw_mut_DPMDetector_ObjectDetection(), val) };
		ret
	}
	
}

pub struct DPMDetector_ObjectDetection {
	ptr: *mut c_void
}

opencv_type_boxed! { DPMDetector_ObjectDetection }

impl Drop for DPMDetector_ObjectDetection {
	fn drop(&mut self) {
		extern "C" { fn cv_DPMDetector_ObjectDetection_delete(instance: *mut c_void); }
		unsafe { cv_DPMDetector_ObjectDetection_delete(self.as_raw_mut_DPMDetector_ObjectDetection()) };
	}
}

unsafe impl Send for DPMDetector_ObjectDetection {}

impl crate::dpm::DPMDetector_ObjectDetectionTraitConst for DPMDetector_ObjectDetection {
	#[inline] fn as_raw_DPMDetector_ObjectDetection(&self) -> *const c_void { self.as_raw() }
}

impl crate::dpm::DPMDetector_ObjectDetectionTrait for DPMDetector_ObjectDetection {
	#[inline] fn as_raw_mut_DPMDetector_ObjectDetection(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl DPMDetector_ObjectDetection {
	#[inline]
	pub fn default() -> Result<crate::dpm::DPMDetector_ObjectDetection> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dpm_DPMDetector_ObjectDetection_ObjectDetection(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dpm::DPMDetector_ObjectDetection::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * class_id: -1
	#[inline]
	pub fn new(rect: core::Rect, score: f32, class_id: i32) -> Result<crate::dpm::DPMDetector_ObjectDetection> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_dpm_DPMDetector_ObjectDetection_ObjectDetection_const_RectR_float_int(&rect, score, class_id, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::dpm::DPMDetector_ObjectDetection::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
