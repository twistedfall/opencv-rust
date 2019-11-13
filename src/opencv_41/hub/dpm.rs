//! # Deformable Part-based Models
//!
//! Discriminatively Trained Part Based Models for Object Detection
//! ---------------------------------------------------------------
//!
//! The object detector described below has been initially proposed by P.F. Felzenszwalb in
//! [Felzenszwalb2010a](https://docs.opencv.org/4.1.2/d0/de3/citelist.html#CITEREF_Felzenszwalb2010a) . It is based on a Dalal-Triggs detector that uses a single filter on histogram
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
//! [Felzenszwalb2010b](https://docs.opencv.org/4.1.2/d0/de3/citelist.html#CITEREF_Felzenszwalb2010b) . The algorithm prunes partial hypotheses using thresholds on their scores.The
//! basic idea of the algorithm is to use a hierarchy of models defined by an ordering of the original
//! model's parts. For a model with (n+1) parts, including the root, a sequence of (n+1) models is
//! obtained. The i-th model in this sequence is defined by the first i parts from the original model.
//! Using this hierarchy, low scoring hypotheses can be pruned after looking at the best configuration
//! of a subset of the parts. Hypotheses that score high under a weak model are evaluated further using
//! a richer model.
//!
//! In OpenCV there is an C++ implementation of DPM cascade detector.
use std::os::raw::{c_char, c_void};
use libc::{ptrdiff_t, size_t};
use crate::{Error, Result, core, sys, types};
use crate::core::{_InputArrayTrait, _OutputArrayTrait};


// Generating impl for trait crate::dpm::DPMDetector
/// This is a C++ abstract class, it provides external user API to work with DPM.
pub trait DPMDetector {
    fn as_raw_DPMDetector(&self) -> *mut c_void;
    fn is_empty(&self) -> Result<bool> {
        unsafe { sys::cv_dpm_DPMDetector_isEmpty_const(self.as_raw_DPMDetector()) }.into_result()
    }
    
    /// Find rectangular regions in the given image that are likely to contain objects of loaded classes
    /// (models) and corresponding confidence levels.
    /// ## Parameters
    /// * image: An image.
    /// * objects: The detections: rectangulars, scores and class IDs.
    fn detect(&mut self, image: &mut core::Mat, objects: &mut types::VectorOfObjectDetection) -> Result<()> {
        unsafe { sys::cv_dpm_DPMDetector_detect_Mat_VectorOfObjectDetection(self.as_raw_DPMDetector(), image.as_raw_Mat(), objects.as_raw_VectorOfObjectDetection()) }.into_result()
    }
    
    /// Return a count of loaded models (classes).
    fn get_class_count(&self) -> Result<size_t> {
        unsafe { sys::cv_dpm_DPMDetector_getClassCount_const(self.as_raw_DPMDetector()) }.into_result()
    }
    
}

// boxed class cv::dpm::DPMDetector::ObjectDetection
pub struct DPMDetector_ObjectDetection {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for DPMDetector_ObjectDetection {
    fn drop(&mut self) {
        unsafe { sys::cv_DPMDetector_ObjectDetection_delete(self.ptr) };
    }
}

impl DPMDetector_ObjectDetection {
    #[inline(always)] pub fn as_raw_DPMDetector_ObjectDetection(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for DPMDetector_ObjectDetection {}

impl DPMDetector_ObjectDetection {
    pub fn default() -> Result<crate::dpm::DPMDetector_ObjectDetection> {
        unsafe { sys::cv_dpm_DPMDetector_ObjectDetection_ObjectDetection() }.into_result().map(|ptr| crate::dpm::DPMDetector_ObjectDetection { ptr })
    }
    
    ///
    /// ## C++ default parameters
    /// * class_id: -1
    pub fn new(rect: core::Rect, score: f32, class_id: i32) -> Result<crate::dpm::DPMDetector_ObjectDetection> {
        unsafe { sys::cv_dpm_DPMDetector_ObjectDetection_ObjectDetection_Rect_float_int(rect, score, class_id) }.into_result().map(|ptr| crate::dpm::DPMDetector_ObjectDetection { ptr })
    }
    
}

