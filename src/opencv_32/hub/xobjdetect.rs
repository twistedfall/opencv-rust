//! # Extended object detection
use std::os::raw::{c_char, c_void};
use libc::{ptrdiff_t, size_t};
use crate::{Error, Result, core, sys, types};
use crate::core::{_InputArray, _OutputArray};


// Generating impl for trait cv::xobjdetect::WBDetector (trait)
/// WaldBoost detector
pub trait WBDetector {
    #[inline(always)] fn as_raw_WBDetector(&self) -> *mut c_void;
    /// Read detector from FileNode.
    /// ## Parameters
    /// * node: FileNode for input
    fn read(&mut self, node: &core::FileNode) -> Result<()> {
        unsafe { sys::cv_xobjdetect_WBDetector_read_FileNode(self.as_raw_WBDetector(), node.as_raw_FileNode()) }.into_result()
    }
    
    /// Write detector to FileStorage.
    /// ## Parameters
    /// * fs: FileStorage for output
    fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
        unsafe { sys::cv_xobjdetect_WBDetector_write_const_FileStorage(self.as_raw_WBDetector(), fs.as_raw_FileStorage()) }.into_result()
    }
    
    /// Train WaldBoost detector
    /// ## Parameters
    /// * pos_samples: Path to directory with cropped positive samples
    /// * neg_imgs: Path to directory with negative (background) images
    fn train(&mut self, pos_samples: &str, neg_imgs: &str) -> Result<()> {
        string_arg!(pos_samples);
        string_arg!(neg_imgs);
        unsafe { sys::cv_xobjdetect_WBDetector_train_std_string_std_string(self.as_raw_WBDetector(), pos_samples.as_ptr(), neg_imgs.as_ptr()) }.into_result()
    }
    
    /// Detect objects on image using WaldBoost detector
    /// ## Parameters
    /// * img: Input image for detection
    /// * bboxes: Bounding boxes coordinates output vector
    /// * confidences: Confidence values for bounding boxes output vector
    fn detect(&mut self, img: &core::Mat, bboxes: &mut types::VectorOfRect, confidences: &mut types::VectorOfdouble) -> Result<()> {
        unsafe { sys::cv_xobjdetect_WBDetector_detect_Mat_VectorOfRect_VectorOfdouble(self.as_raw_WBDetector(), img.as_raw_Mat(), bboxes.as_raw_VectorOfRect(), confidences.as_raw_VectorOfdouble()) }.into_result()
    }
    
}

impl dyn WBDetector + '_ {

    /// Create instance of WBDetector
    pub fn create() -> Result<types::PtrOfWBDetector> {
        unsafe { sys::cv_xobjdetect_WBDetector_create() }.into_result().map(|ptr| types::PtrOfWBDetector { ptr })
    }
    
}

