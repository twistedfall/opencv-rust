//! <script type="text/javascript" src="http://latex.codecogs.com/latexit.js"></script>

use libc::{c_void, c_char, size_t};
use std::ffi::{CStr, CString};
use crate::{core, sys, types};
use crate::{Error, Result};

/// Computes the "minimal work" distance between two weighted point configurations base on the papers
/// "EMD-L1: An efficient and Robust Algorithm for comparing histogram-based descriptors", by Haibin
/// Ling and Kazunori Okuda; and "The Earth Mover's Distance is the Mallows Distance: Some Insights from
/// Statistics", by Elizaveta Levina and Peter Bickel.
/// 
/// ## Parameters
/// * signature1: First signature, a single column floating-point matrix. Each row is the value of
/// the histogram in each bin.
/// * signature2: Second signature of the same format and size as signature1.
pub fn emdl1(signature1: &core::Mat, signature2: &core::Mat) -> Result<f32> {
// identifier: cv_EMDL1_Mat_signature1_Mat_signature2
  unsafe {
    let rv = sys::cv_shape_cv_EMDL1_Mat_signature1_Mat_signature2(signature1.as_raw_Mat(), signature2.as_raw_Mat());
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(rv.result)
    }
  }
}

/// Complete constructor
pub fn create_affine_transformer(full_affine: bool) -> Result<types::PtrOfAffineTransformer> {
// identifier: cv_createAffineTransformer_bool_fullAffine
  unsafe {
    let rv = sys::cv_shape_cv_createAffineTransformer_bool_fullAffine(full_affine);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(types::PtrOfAffineTransformer { ptr: rv.result })
    }
  }
}

///
/// ## C++ default parameters:
/// * n_dummies: 25
/// * default_cost: 0.2f
pub fn create_chi_histogram_cost_extractor(n_dummies: i32, default_cost: f32) -> Result<types::PtrOfHistogramCostExtractor> {
// identifier: cv_createChiHistogramCostExtractor_int_nDummies_float_defaultCost
  unsafe {
    let rv = sys::cv_shape_cv_createChiHistogramCostExtractor_int_nDummies_float_defaultCost(n_dummies, default_cost);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(types::PtrOfHistogramCostExtractor { ptr: rv.result })
    }
  }
}

///
/// ## C++ default parameters:
/// * flag: DIST_L2
/// * n_dummies: 25
/// * default_cost: 0.2f
pub fn create_emd_histogram_cost_extractor(flag: i32, n_dummies: i32, default_cost: f32) -> Result<types::PtrOfHistogramCostExtractor> {
// identifier: cv_createEMDHistogramCostExtractor_int_flag_int_nDummies_float_defaultCost
  unsafe {
    let rv = sys::cv_shape_cv_createEMDHistogramCostExtractor_int_flag_int_nDummies_float_defaultCost(flag, n_dummies, default_cost);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(types::PtrOfHistogramCostExtractor { ptr: rv.result })
    }
  }
}

///
/// ## C++ default parameters:
/// * n_dummies: 25
/// * default_cost: 0.2f
pub fn create_emdl1_histogram_cost_extractor(n_dummies: i32, default_cost: f32) -> Result<types::PtrOfHistogramCostExtractor> {
// identifier: cv_createEMDL1HistogramCostExtractor_int_nDummies_float_defaultCost
  unsafe {
    let rv = sys::cv_shape_cv_createEMDL1HistogramCostExtractor_int_nDummies_float_defaultCost(n_dummies, default_cost);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(types::PtrOfHistogramCostExtractor { ptr: rv.result })
    }
  }
}

///
/// ## C++ default parameters:
/// * distance_flag: cv::NORM_L2
/// * rank_prop: 0.6f
pub fn create_hausdorff_distance_extractor(distance_flag: i32, rank_prop: f32) -> Result<types::PtrOfHausdorffDistanceExtractor> {
// identifier: cv_createHausdorffDistanceExtractor_int_distanceFlag_float_rankProp
  unsafe {
    let rv = sys::cv_shape_cv_createHausdorffDistanceExtractor_int_distanceFlag_float_rankProp(distance_flag, rank_prop);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(types::PtrOfHausdorffDistanceExtractor { ptr: rv.result })
    }
  }
}

///
/// ## C++ default parameters:
/// * flag: DIST_L2
/// * n_dummies: 25
/// * default_cost: 0.2f
pub fn create_norm_histogram_cost_extractor(flag: i32, n_dummies: i32, default_cost: f32) -> Result<types::PtrOfHistogramCostExtractor> {
// identifier: cv_createNormHistogramCostExtractor_int_flag_int_nDummies_float_defaultCost
  unsafe {
    let rv = sys::cv_shape_cv_createNormHistogramCostExtractor_int_flag_int_nDummies_float_defaultCost(flag, n_dummies, default_cost);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(types::PtrOfHistogramCostExtractor { ptr: rv.result })
    }
  }
}

/// Complete constructor
///
/// ## C++ default parameters:
/// * regularization_parameter: 0
pub fn create_thin_plate_spline_shape_transformer(regularization_parameter: f64) -> Result<types::PtrOfThinPlateSplineShapeTransformer> {
// identifier: cv_createThinPlateSplineShapeTransformer_double_regularizationParameter
  unsafe {
    let rv = sys::cv_shape_cv_createThinPlateSplineShapeTransformer_double_regularizationParameter(regularization_parameter);
    if !rv.error_msg.is_null() {
      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
      ::libc::free(rv.error_msg as _);
      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
    } else {
      Ok(types::PtrOfThinPlateSplineShapeTransformer { ptr: rv.result })
    }
  }
}

// Generating impl for trait cv::AffineTransformer (trait)
/// Wrapper class for the OpenCV Affine Transformation algorithm. :
pub trait AffineTransformer : super::shape::ShapeTransformer {
  #[doc(hidden)] fn as_raw_AffineTransformer(&self) -> *mut c_void;
  fn set_full_affine(&mut self, full_affine: bool) -> Result<()> {
  // identifier: cv_AffineTransformer_setFullAffine_bool_fullAffine
    unsafe {
      let rv = sys::cv_shape_cv_AffineTransformer_setFullAffine_bool_fullAffine(self.as_raw_AffineTransformer(), full_affine);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_full_affine(&self) -> Result<bool> {
  // identifier: cv_AffineTransformer_getFullAffine
    unsafe {
      let rv = sys::cv_shape_cv_AffineTransformer_getFullAffine(self.as_raw_AffineTransformer());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
impl<'a> AffineTransformer + 'a {

}

// boxed class cv::ChiHistogramCostExtractor
/// An Chi based cost extraction. :

#[allow(dead_code)]
pub struct ChiHistogramCostExtractor {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::shape::ChiHistogramCostExtractor {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_ChiHistogramCostExtractor(self.ptr) };
    }
}
impl super::shape::ChiHistogramCostExtractor {
    #[doc(hidden)] pub fn as_raw_ChiHistogramCostExtractor(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for ChiHistogramCostExtractor {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl super::shape::HistogramCostExtractor for ChiHistogramCostExtractor {
    #[doc(hidden)] fn as_raw_HistogramCostExtractor(&self) -> *mut c_void { self.ptr }
}
impl ChiHistogramCostExtractor {

}
// Generating impl for trait cv::EMDHistogramCostExtractor (trait)
/// An EMD based cost extraction. :
pub trait EMDHistogramCostExtractor : super::shape::HistogramCostExtractor {
  #[doc(hidden)] fn as_raw_EMDHistogramCostExtractor(&self) -> *mut c_void;
  fn set_norm_flag(&mut self, flag: i32) -> Result<()> {
  // identifier: cv_EMDHistogramCostExtractor_setNormFlag_int_flag
    unsafe {
      let rv = sys::cv_shape_cv_EMDHistogramCostExtractor_setNormFlag_int_flag(self.as_raw_EMDHistogramCostExtractor(), flag);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_norm_flag(&self) -> Result<i32> {
  // identifier: cv_EMDHistogramCostExtractor_getNormFlag
    unsafe {
      let rv = sys::cv_shape_cv_EMDHistogramCostExtractor_getNormFlag(self.as_raw_EMDHistogramCostExtractor());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
impl<'a> EMDHistogramCostExtractor + 'a {

}

// boxed class cv::EMDL1HistogramCostExtractor
/// An EMD-L1 based cost extraction. :

#[allow(dead_code)]
pub struct EMDL1HistogramCostExtractor {
    #[doc(hidden)] pub ptr: *mut c_void
}
impl Drop for super::shape::EMDL1HistogramCostExtractor {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_EMDL1HistogramCostExtractor(self.ptr) };
    }
}
impl super::shape::EMDL1HistogramCostExtractor {
    #[doc(hidden)] pub fn as_raw_EMDL1HistogramCostExtractor(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for EMDL1HistogramCostExtractor {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl super::shape::HistogramCostExtractor for EMDL1HistogramCostExtractor {
    #[doc(hidden)] fn as_raw_HistogramCostExtractor(&self) -> *mut c_void { self.ptr }
}
impl EMDL1HistogramCostExtractor {

}
// Generating impl for trait cv::HausdorffDistanceExtractor (trait)
/// A simple Hausdorff distance measure between shapes defined by contours
/// 
/// according to the paper "Comparing Images using the Hausdorff distance." by D.P. Huttenlocher, G.A.
/// Klanderman, and W.J. Rucklidge. (PAMI 1993). :
pub trait HausdorffDistanceExtractor : super::shape::ShapeDistanceExtractor {
  #[doc(hidden)] fn as_raw_HausdorffDistanceExtractor(&self) -> *mut c_void;
  /// Set the norm used to compute the Hausdorff value between two shapes. It can be L1 or L2 norm.
  /// 
  /// ## Parameters
  /// * distanceFlag: Flag indicating which norm is used to compute the Hausdorff distance
  /// (NORM_L1, NORM_L2).
  fn set_distance_flag(&mut self, distance_flag: i32) -> Result<()> {
  // identifier: cv_HausdorffDistanceExtractor_setDistanceFlag_int_distanceFlag
    unsafe {
      let rv = sys::cv_shape_cv_HausdorffDistanceExtractor_setDistanceFlag_int_distanceFlag(self.as_raw_HausdorffDistanceExtractor(), distance_flag);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_distance_flag(&self) -> Result<i32> {
  // identifier: cv_HausdorffDistanceExtractor_getDistanceFlag
    unsafe {
      let rv = sys::cv_shape_cv_HausdorffDistanceExtractor_getDistanceFlag(self.as_raw_HausdorffDistanceExtractor());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// This method sets the rank proportion (or fractional value) that establish the Kth ranked value of
  /// the partial Hausdorff distance. Experimentally had been shown that 0.6 is a good value to compare
  /// shapes.
  /// 
  /// ## Parameters
  /// * rankProportion: fractional value (between 0 and 1).
  fn set_rank_proportion(&mut self, rank_proportion: f32) -> Result<()> {
  // identifier: cv_HausdorffDistanceExtractor_setRankProportion_float_rankProportion
    unsafe {
      let rv = sys::cv_shape_cv_HausdorffDistanceExtractor_setRankProportion_float_rankProportion(self.as_raw_HausdorffDistanceExtractor(), rank_proportion);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_rank_proportion(&self) -> Result<f32> {
  // identifier: cv_HausdorffDistanceExtractor_getRankProportion
    unsafe {
      let rv = sys::cv_shape_cv_HausdorffDistanceExtractor_getRankProportion(self.as_raw_HausdorffDistanceExtractor());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
impl<'a> HausdorffDistanceExtractor + 'a {

}

// Generating impl for trait cv::HistogramCostExtractor (trait)
/// Abstract base class for histogram cost algorithms.
pub trait HistogramCostExtractor : core::Algorithm {
  #[doc(hidden)] fn as_raw_HistogramCostExtractor(&self) -> *mut c_void;
  fn build_cost_matrix(&mut self, descriptors1: &core::Mat, descriptors2: &core::Mat, cost_matrix: &mut core::Mat) -> Result<()> {
  // identifier: cv_HistogramCostExtractor_buildCostMatrix_Mat_descriptors1_Mat_descriptors2_Mat_costMatrix
    unsafe {
      let rv = sys::cv_shape_cv_HistogramCostExtractor_buildCostMatrix_Mat_descriptors1_Mat_descriptors2_Mat_costMatrix(self.as_raw_HistogramCostExtractor(), descriptors1.as_raw_Mat(), descriptors2.as_raw_Mat(), cost_matrix.as_raw_Mat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn set_n_dummies(&mut self, n_dummies: i32) -> Result<()> {
  // identifier: cv_HistogramCostExtractor_setNDummies_int_nDummies
    unsafe {
      let rv = sys::cv_shape_cv_HistogramCostExtractor_setNDummies_int_nDummies(self.as_raw_HistogramCostExtractor(), n_dummies);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_n_dummies(&self) -> Result<i32> {
  // identifier: cv_HistogramCostExtractor_getNDummies
    unsafe {
      let rv = sys::cv_shape_cv_HistogramCostExtractor_getNDummies(self.as_raw_HistogramCostExtractor());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_default_cost(&mut self, default_cost: f32) -> Result<()> {
  // identifier: cv_HistogramCostExtractor_setDefaultCost_float_defaultCost
    unsafe {
      let rv = sys::cv_shape_cv_HistogramCostExtractor_setDefaultCost_float_defaultCost(self.as_raw_HistogramCostExtractor(), default_cost);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_default_cost(&self) -> Result<f32> {
  // identifier: cv_HistogramCostExtractor_getDefaultCost
    unsafe {
      let rv = sys::cv_shape_cv_HistogramCostExtractor_getDefaultCost(self.as_raw_HistogramCostExtractor());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
impl<'a> HistogramCostExtractor + 'a {

}

// Generating impl for trait cv::NormHistogramCostExtractor (trait)
/// A norm based cost extraction. :
pub trait NormHistogramCostExtractor : super::shape::HistogramCostExtractor {
  #[doc(hidden)] fn as_raw_NormHistogramCostExtractor(&self) -> *mut c_void;
  fn set_norm_flag(&mut self, flag: i32) -> Result<()> {
  // identifier: cv_NormHistogramCostExtractor_setNormFlag_int_flag
    unsafe {
      let rv = sys::cv_shape_cv_NormHistogramCostExtractor_setNormFlag_int_flag(self.as_raw_NormHistogramCostExtractor(), flag);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_norm_flag(&self) -> Result<i32> {
  // identifier: cv_NormHistogramCostExtractor_getNormFlag
    unsafe {
      let rv = sys::cv_shape_cv_NormHistogramCostExtractor_getNormFlag(self.as_raw_NormHistogramCostExtractor());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
impl<'a> NormHistogramCostExtractor + 'a {

}

// Generating impl for trait cv::ShapeContextDistanceExtractor (trait)
/// Implementation of the Shape Context descriptor and matching algorithm
/// 
/// proposed by Belongie et al. in "Shape Matching and Object Recognition Using Shape Contexts" (PAMI
/// 2002). This implementation is packaged in a generic scheme, in order to allow you the
/// implementation of the common variations of the original pipeline.
pub trait ShapeContextDistanceExtractor : super::shape::ShapeDistanceExtractor {
  #[doc(hidden)] fn as_raw_ShapeContextDistanceExtractor(&self) -> *mut c_void;
  /// Establish the number of angular bins for the Shape Context Descriptor used in the shape matching
  /// pipeline.
  /// 
  /// ## Parameters
  /// * nAngularBins: The number of angular bins in the shape context descriptor.
  fn set_angular_bins(&mut self, n_angular_bins: i32) -> Result<()> {
  // identifier: cv_ShapeContextDistanceExtractor_setAngularBins_int_nAngularBins
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_setAngularBins_int_nAngularBins(self.as_raw_ShapeContextDistanceExtractor(), n_angular_bins);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_angular_bins(&self) -> Result<i32> {
  // identifier: cv_ShapeContextDistanceExtractor_getAngularBins
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_getAngularBins(self.as_raw_ShapeContextDistanceExtractor());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Establish the number of radial bins for the Shape Context Descriptor used in the shape matching
  /// pipeline.
  /// 
  /// ## Parameters
  /// * nRadialBins: The number of radial bins in the shape context descriptor.
  fn set_radial_bins(&mut self, n_radial_bins: i32) -> Result<()> {
  // identifier: cv_ShapeContextDistanceExtractor_setRadialBins_int_nRadialBins
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_setRadialBins_int_nRadialBins(self.as_raw_ShapeContextDistanceExtractor(), n_radial_bins);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_radial_bins(&self) -> Result<i32> {
  // identifier: cv_ShapeContextDistanceExtractor_getRadialBins
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_getRadialBins(self.as_raw_ShapeContextDistanceExtractor());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Set the inner radius of the shape context descriptor.
  /// 
  /// ## Parameters
  /// * innerRadius: The value of the inner radius.
  fn set_inner_radius(&mut self, inner_radius: f32) -> Result<()> {
  // identifier: cv_ShapeContextDistanceExtractor_setInnerRadius_float_innerRadius
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_setInnerRadius_float_innerRadius(self.as_raw_ShapeContextDistanceExtractor(), inner_radius);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_inner_radius(&self) -> Result<f32> {
  // identifier: cv_ShapeContextDistanceExtractor_getInnerRadius
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_getInnerRadius(self.as_raw_ShapeContextDistanceExtractor());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Set the outer radius of the shape context descriptor.
  /// 
  /// ## Parameters
  /// * outerRadius: The value of the outer radius.
  fn set_outer_radius(&mut self, outer_radius: f32) -> Result<()> {
  // identifier: cv_ShapeContextDistanceExtractor_setOuterRadius_float_outerRadius
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_setOuterRadius_float_outerRadius(self.as_raw_ShapeContextDistanceExtractor(), outer_radius);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_outer_radius(&self) -> Result<f32> {
  // identifier: cv_ShapeContextDistanceExtractor_getOuterRadius
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_getOuterRadius(self.as_raw_ShapeContextDistanceExtractor());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  fn set_rotation_invariant(&mut self, rotation_invariant: bool) -> Result<()> {
  // identifier: cv_ShapeContextDistanceExtractor_setRotationInvariant_bool_rotationInvariant
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_setRotationInvariant_bool_rotationInvariant(self.as_raw_ShapeContextDistanceExtractor(), rotation_invariant);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_rotation_invariant(&self) -> Result<bool> {
  // identifier: cv_ShapeContextDistanceExtractor_getRotationInvariant
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_getRotationInvariant(self.as_raw_ShapeContextDistanceExtractor());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Set the weight of the shape context distance in the final value of the shape distance. The shape
  /// context distance between two shapes is defined as the symmetric sum of shape context matching costs
  /// over best matching points. The final value of the shape distance is a user-defined linear
  /// combination of the shape context distance, an image appearance distance, and a bending energy.
  /// 
  /// ## Parameters
  /// * shapeContextWeight: The weight of the shape context distance in the final distance value.
  fn set_shape_context_weight(&mut self, shape_context_weight: f32) -> Result<()> {
  // identifier: cv_ShapeContextDistanceExtractor_setShapeContextWeight_float_shapeContextWeight
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_setShapeContextWeight_float_shapeContextWeight(self.as_raw_ShapeContextDistanceExtractor(), shape_context_weight);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_shape_context_weight(&self) -> Result<f32> {
  // identifier: cv_ShapeContextDistanceExtractor_getShapeContextWeight
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_getShapeContextWeight(self.as_raw_ShapeContextDistanceExtractor());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Set the weight of the Image Appearance cost in the final value of the shape distance. The image
  /// appearance cost is defined as the sum of squared brightness differences in Gaussian windows around
  /// corresponding image points. The final value of the shape distance is a user-defined linear
  /// combination of the shape context distance, an image appearance distance, and a bending energy. If
  /// this value is set to a number different from 0, is mandatory to set the images that correspond to
  /// each shape.
  /// 
  /// ## Parameters
  /// * imageAppearanceWeight: The weight of the appearance cost in the final distance value.
  fn set_image_appearance_weight(&mut self, image_appearance_weight: f32) -> Result<()> {
  // identifier: cv_ShapeContextDistanceExtractor_setImageAppearanceWeight_float_imageAppearanceWeight
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_setImageAppearanceWeight_float_imageAppearanceWeight(self.as_raw_ShapeContextDistanceExtractor(), image_appearance_weight);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_image_appearance_weight(&self) -> Result<f32> {
  // identifier: cv_ShapeContextDistanceExtractor_getImageAppearanceWeight
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_getImageAppearanceWeight(self.as_raw_ShapeContextDistanceExtractor());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Set the weight of the Bending Energy in the final value of the shape distance. The bending energy
  /// definition depends on what transformation is being used to align the shapes. The final value of the
  /// shape distance is a user-defined linear combination of the shape context distance, an image
  /// appearance distance, and a bending energy.
  /// 
  /// ## Parameters
  /// * bendingEnergyWeight: The weight of the Bending Energy in the final distance value.
  fn set_bending_energy_weight(&mut self, bending_energy_weight: f32) -> Result<()> {
  // identifier: cv_ShapeContextDistanceExtractor_setBendingEnergyWeight_float_bendingEnergyWeight
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_setBendingEnergyWeight_float_bendingEnergyWeight(self.as_raw_ShapeContextDistanceExtractor(), bending_energy_weight);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_bending_energy_weight(&self) -> Result<f32> {
  // identifier: cv_ShapeContextDistanceExtractor_getBendingEnergyWeight
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_getBendingEnergyWeight(self.as_raw_ShapeContextDistanceExtractor());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Set the images that correspond to each shape. This images are used in the calculation of the Image
  /// Appearance cost.
  /// 
  /// ## Parameters
  /// * image1: Image corresponding to the shape defined by contours1.
  /// * image2: Image corresponding to the shape defined by contours2.
  fn set_images(&mut self, image1: &core::Mat, image2: &core::Mat) -> Result<()> {
  // identifier: cv_ShapeContextDistanceExtractor_setImages_Mat_image1_Mat_image2
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_setImages_Mat_image1_Mat_image2(self.as_raw_ShapeContextDistanceExtractor(), image1.as_raw_Mat(), image2.as_raw_Mat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_images(&self, image1: &mut core::Mat, image2: &mut core::Mat) -> Result<()> {
  // identifier: cv_ShapeContextDistanceExtractor_getImages_Mat_image1_Mat_image2
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_getImages_Mat_image1_Mat_image2(self.as_raw_ShapeContextDistanceExtractor(), image1.as_raw_Mat(), image2.as_raw_Mat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn set_iterations(&mut self, iterations: i32) -> Result<()> {
  // identifier: cv_ShapeContextDistanceExtractor_setIterations_int_iterations
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_setIterations_int_iterations(self.as_raw_ShapeContextDistanceExtractor(), iterations);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_iterations(&self) -> Result<i32> {
  // identifier: cv_ShapeContextDistanceExtractor_getIterations
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_getIterations(self.as_raw_ShapeContextDistanceExtractor());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Set the algorithm used for building the shape context descriptor cost matrix.
  /// 
  /// ## Parameters
  /// * comparer: Smart pointer to a HistogramCostExtractor, an algorithm that defines the cost
  /// matrix between descriptors.
  fn set_cost_extractor(&mut self, comparer: &types::PtrOfHistogramCostExtractor) -> Result<()> {
  // identifier: cv_ShapeContextDistanceExtractor_setCostExtractor_PtrOfHistogramCostExtractor_comparer
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_setCostExtractor_PtrOfHistogramCostExtractor_comparer(self.as_raw_ShapeContextDistanceExtractor(), comparer.as_raw_PtrOfHistogramCostExtractor());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_cost_extractor(&self) -> Result<types::PtrOfHistogramCostExtractor> {
  // identifier: cv_ShapeContextDistanceExtractor_getCostExtractor
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_getCostExtractor(self.as_raw_ShapeContextDistanceExtractor());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(types::PtrOfHistogramCostExtractor { ptr: rv.result })
      }
    }
  }

  /// Set the value of the standard deviation for the Gaussian window for the image appearance cost.
  /// 
  /// ## Parameters
  /// * sigma: Standard Deviation.
  fn set_std_dev(&mut self, sigma: f32) -> Result<()> {
  // identifier: cv_ShapeContextDistanceExtractor_setStdDev_float_sigma
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_setStdDev_float_sigma(self.as_raw_ShapeContextDistanceExtractor(), sigma);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_std_dev(&self) -> Result<f32> {
  // identifier: cv_ShapeContextDistanceExtractor_getStdDev
    unsafe {
      let rv = sys::cv_shape_cv_ShapeContextDistanceExtractor_getStdDev(self.as_raw_ShapeContextDistanceExtractor());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
impl<'a> ShapeContextDistanceExtractor + 'a {

}

// Generating impl for trait cv::ShapeDistanceExtractor (trait)
/// Abstract base class for shape distance algorithms.
pub trait ShapeDistanceExtractor : core::Algorithm {
  #[doc(hidden)] fn as_raw_ShapeDistanceExtractor(&self) -> *mut c_void;
  /// Compute the shape distance between two shapes defined by its contours.
  /// 
  /// ## Parameters
  /// * contour1: Contour defining first shape.
  /// * contour2: Contour defining second shape.
  fn compute_distance(&mut self, contour1: &core::Mat, contour2: &core::Mat) -> Result<f32> {
  // identifier: cv_ShapeDistanceExtractor_computeDistance_Mat_contour1_Mat_contour2
    unsafe {
      let rv = sys::cv_shape_cv_ShapeDistanceExtractor_computeDistance_Mat_contour1_Mat_contour2(self.as_raw_ShapeDistanceExtractor(), contour1.as_raw_Mat(), contour2.as_raw_Mat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
impl<'a> ShapeDistanceExtractor + 'a {

}

// Generating impl for trait cv::ShapeTransformer (trait)
/// Abstract base class for shape transformation algorithms.
pub trait ShapeTransformer : core::Algorithm {
  #[doc(hidden)] fn as_raw_ShapeTransformer(&self) -> *mut c_void;
  /// Estimate the transformation parameters of the current transformer algorithm, based on point matches.
  /// 
  /// ## Parameters
  /// * transformingShape: Contour defining first shape.
  /// * targetShape: Contour defining second shape (Target).
  /// * matches: Standard vector of Matches between points.
  fn estimate_transformation(&mut self, transforming_shape: &core::Mat, target_shape: &core::Mat, matches: &types::VectorOfDMatch) -> Result<()> {
  // identifier: cv_ShapeTransformer_estimateTransformation_Mat_transformingShape_Mat_targetShape_VectorOfDMatch_matches
    unsafe {
      let rv = sys::cv_shape_cv_ShapeTransformer_estimateTransformation_Mat_transformingShape_Mat_targetShape_VectorOfDMatch_matches(self.as_raw_ShapeTransformer(), transforming_shape.as_raw_Mat(), target_shape.as_raw_Mat(), matches.as_raw_VectorOfDMatch());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  /// Apply a transformation, given a pre-estimated transformation parameters.
  /// 
  /// ## Parameters
  /// * input: Contour (set of points) to apply the transformation.
  /// * output: Output contour.
  ///
  /// ## C++ default parameters:
  /// * output: noArray()
  fn apply_transformation(&mut self, input: &core::Mat, output: &mut core::Mat) -> Result<f32> {
  // identifier: cv_ShapeTransformer_applyTransformation_Mat_input_Mat_output
    unsafe {
      let rv = sys::cv_shape_cv_ShapeTransformer_applyTransformation_Mat_input_Mat_output(self.as_raw_ShapeTransformer(), input.as_raw_Mat(), output.as_raw_Mat());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

  /// Apply a transformation, given a pre-estimated transformation parameters, to an Image.
  /// 
  /// ## Parameters
  /// * transformingImage: Input image.
  /// * output: Output image.
  /// * flags: Image interpolation method.
  /// * borderMode: border style.
  /// * borderValue: border value.
  ///
  /// ## C++ default parameters:
  /// * flags: INTER_LINEAR
  /// * border_mode: BORDER_CONSTANT
  /// * border_value: Scalar()
  fn warp_image(&self, transforming_image: &core::Mat, output: &mut core::Mat, flags: i32, border_mode: i32, border_value: core::Scalar) -> Result<()> {
  // identifier: cv_ShapeTransformer_warpImage_Mat_transformingImage_Mat_output_int_flags_int_borderMode_Scalar_borderValue
    unsafe {
      let rv = sys::cv_shape_cv_ShapeTransformer_warpImage_Mat_transformingImage_Mat_output_int_flags_int_borderMode_Scalar_borderValue(self.as_raw_ShapeTransformer(), transforming_image.as_raw_Mat(), output.as_raw_Mat(), flags, border_mode, border_value);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

}
impl<'a> ShapeTransformer + 'a {

}

// Generating impl for trait cv::ThinPlateSplineShapeTransformer (trait)
/// Definition of the transformation
/// 
/// occupied in the paper "Principal Warps: Thin-Plate Splines and Decomposition of Deformations", by
/// F.L. Bookstein (PAMI 1989). :
pub trait ThinPlateSplineShapeTransformer : super::shape::ShapeTransformer {
  #[doc(hidden)] fn as_raw_ThinPlateSplineShapeTransformer(&self) -> *mut c_void;
  /// Set the regularization parameter for relaxing the exact interpolation requirements of the TPS
  /// algorithm.
  /// 
  /// ## Parameters
  /// * beta: value of the regularization parameter.
  fn set_regularization_parameter(&mut self, beta: f64) -> Result<()> {
  // identifier: cv_ThinPlateSplineShapeTransformer_setRegularizationParameter_double_beta
    unsafe {
      let rv = sys::cv_shape_cv_ThinPlateSplineShapeTransformer_setRegularizationParameter_double_beta(self.as_raw_ThinPlateSplineShapeTransformer(), beta);
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(())
      }
    }
  }

  fn get_regularization_parameter(&self) -> Result<f64> {
  // identifier: cv_ThinPlateSplineShapeTransformer_getRegularizationParameter
    unsafe {
      let rv = sys::cv_shape_cv_ThinPlateSplineShapeTransformer_getRegularizationParameter(self.as_raw_ThinPlateSplineShapeTransformer());
      if !rv.error_msg.is_null() {
        let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();
        ::libc::free(rv.error_msg as _);
        Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })
      } else {
        Ok(rv.result)
      }
    }
  }

}
impl<'a> ThinPlateSplineShapeTransformer + 'a {

}

