//! # Shape Distance and Matching
use std::os::raw::{c_char, c_void};
use libc::size_t;
use crate::{Error, Result, core, sys, types};


// identifier: cv_EMDL1_Mat_signature1_Mat_signature2
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
    unsafe { sys::cv_shape_cv_EMDL1_Mat_signature1_Mat_signature2(signature1.as_raw_Mat(), signature2.as_raw_Mat()) }.into_result()
}

// identifier: cv_createAffineTransformer_bool_fullAffine
/// Complete constructor
pub fn create_affine_transformer(full_affine: bool) -> Result<types::PtrOfAffineTransformer> {
    unsafe { sys::cv_shape_cv_createAffineTransformer_bool_fullAffine(full_affine) }.into_result().map(|x| types::PtrOfAffineTransformer { ptr: x })
}

// identifier: cv_createChiHistogramCostExtractor_int_nDummies_float_defaultCost
///
/// ## C++ default parameters:
/// * n_dummies: 25
/// * default_cost: 0.2f
pub fn create_chi_histogram_cost_extractor(n_dummies: i32, default_cost: f32) -> Result<types::PtrOfHistogramCostExtractor> {
    unsafe { sys::cv_shape_cv_createChiHistogramCostExtractor_int_nDummies_float_defaultCost(n_dummies, default_cost) }.into_result().map(|x| types::PtrOfHistogramCostExtractor { ptr: x })
}

// identifier: cv_createEMDHistogramCostExtractor_int_flag_int_nDummies_float_defaultCost
///
/// ## C++ default parameters:
/// * flag: DIST_L2
/// * n_dummies: 25
/// * default_cost: 0.2f
pub fn create_emd_histogram_cost_extractor(flag: i32, n_dummies: i32, default_cost: f32) -> Result<types::PtrOfHistogramCostExtractor> {
    unsafe { sys::cv_shape_cv_createEMDHistogramCostExtractor_int_flag_int_nDummies_float_defaultCost(flag, n_dummies, default_cost) }.into_result().map(|x| types::PtrOfHistogramCostExtractor { ptr: x })
}

// identifier: cv_createEMDL1HistogramCostExtractor_int_nDummies_float_defaultCost
///
/// ## C++ default parameters:
/// * n_dummies: 25
/// * default_cost: 0.2f
pub fn create_emdl1_histogram_cost_extractor(n_dummies: i32, default_cost: f32) -> Result<types::PtrOfHistogramCostExtractor> {
    unsafe { sys::cv_shape_cv_createEMDL1HistogramCostExtractor_int_nDummies_float_defaultCost(n_dummies, default_cost) }.into_result().map(|x| types::PtrOfHistogramCostExtractor { ptr: x })
}

// identifier: cv_createHausdorffDistanceExtractor_int_distanceFlag_float_rankProp
///
/// ## C++ default parameters:
/// * distance_flag: cv::NORM_L2
/// * rank_prop: 0.6f
pub fn create_hausdorff_distance_extractor(distance_flag: i32, rank_prop: f32) -> Result<types::PtrOfHausdorffDistanceExtractor> {
    unsafe { sys::cv_shape_cv_createHausdorffDistanceExtractor_int_distanceFlag_float_rankProp(distance_flag, rank_prop) }.into_result().map(|x| types::PtrOfHausdorffDistanceExtractor { ptr: x })
}

// identifier: cv_createNormHistogramCostExtractor_int_flag_int_nDummies_float_defaultCost
///
/// ## C++ default parameters:
/// * flag: DIST_L2
/// * n_dummies: 25
/// * default_cost: 0.2f
pub fn create_norm_histogram_cost_extractor(flag: i32, n_dummies: i32, default_cost: f32) -> Result<types::PtrOfHistogramCostExtractor> {
    unsafe { sys::cv_shape_cv_createNormHistogramCostExtractor_int_flag_int_nDummies_float_defaultCost(flag, n_dummies, default_cost) }.into_result().map(|x| types::PtrOfHistogramCostExtractor { ptr: x })
}

// identifier: cv_createThinPlateSplineShapeTransformer_double_regularizationParameter
/// Complete constructor
///
/// ## C++ default parameters:
/// * regularization_parameter: 0
pub fn create_thin_plate_spline_shape_transformer(regularization_parameter: f64) -> Result<types::PtrOfThinPlateSplineShapeTransformer> {
    unsafe { sys::cv_shape_cv_createThinPlateSplineShapeTransformer_double_regularizationParameter(regularization_parameter) }.into_result().map(|x| types::PtrOfThinPlateSplineShapeTransformer { ptr: x })
}

// Generating impl for trait cv::AffineTransformer (trait)
/// Wrapper class for the OpenCV Affine Transformation algorithm. :
pub trait AffineTransformer : crate::shape::ShapeTransformer {
    #[doc(hidden)] fn as_raw_AffineTransformer(&self) -> *mut c_void;
    // identifier: cv_AffineTransformer_setFullAffine_bool_fullAffine
    fn set_full_affine(&mut self, full_affine: bool) -> Result<()> {
        unsafe { sys::cv_shape_cv_AffineTransformer_setFullAffine_bool_fullAffine(self.as_raw_AffineTransformer(), full_affine) }.into_result()
    }
    
    // identifier: cv_AffineTransformer_getFullAffine_const
    fn get_full_affine(&self) -> Result<bool> {
        unsafe { sys::cv_shape_cv_AffineTransformer_getFullAffine_const(self.as_raw_AffineTransformer()) }.into_result()
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
impl Drop for crate::shape::ChiHistogramCostExtractor {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_ChiHistogramCostExtractor(self.ptr) };
    }
}
impl crate::shape::ChiHistogramCostExtractor {
    #[doc(hidden)] pub fn as_raw_ChiHistogramCostExtractor(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for ChiHistogramCostExtractor {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::shape::HistogramCostExtractor for ChiHistogramCostExtractor {
    #[doc(hidden)] fn as_raw_HistogramCostExtractor(&self) -> *mut c_void { self.ptr }
}

// Generating impl for trait cv::EMDHistogramCostExtractor (trait)
/// An EMD based cost extraction. :
pub trait EMDHistogramCostExtractor : crate::shape::HistogramCostExtractor {
    #[doc(hidden)] fn as_raw_EMDHistogramCostExtractor(&self) -> *mut c_void;
    // identifier: cv_EMDHistogramCostExtractor_setNormFlag_int_flag
    fn set_norm_flag(&mut self, flag: i32) -> Result<()> {
        unsafe { sys::cv_shape_cv_EMDHistogramCostExtractor_setNormFlag_int_flag(self.as_raw_EMDHistogramCostExtractor(), flag) }.into_result()
    }
    
    // identifier: cv_EMDHistogramCostExtractor_getNormFlag_const
    fn get_norm_flag(&self) -> Result<i32> {
        unsafe { sys::cv_shape_cv_EMDHistogramCostExtractor_getNormFlag_const(self.as_raw_EMDHistogramCostExtractor()) }.into_result()
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
impl Drop for crate::shape::EMDL1HistogramCostExtractor {
    fn drop(&mut self) {
        unsafe { sys::cv_delete_EMDL1HistogramCostExtractor(self.ptr) };
    }
}
impl crate::shape::EMDL1HistogramCostExtractor {
    #[doc(hidden)] pub fn as_raw_EMDL1HistogramCostExtractor(&self) -> *mut c_void { self.ptr }
}

impl core::Algorithm for EMDL1HistogramCostExtractor {
    #[doc(hidden)] fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::shape::HistogramCostExtractor for EMDL1HistogramCostExtractor {
    #[doc(hidden)] fn as_raw_HistogramCostExtractor(&self) -> *mut c_void { self.ptr }
}

// Generating impl for trait cv::HausdorffDistanceExtractor (trait)
/// A simple Hausdorff distance measure between shapes defined by contours
/// 
/// according to the paper "Comparing Images using the Hausdorff distance." by D.P. Huttenlocher, G.A.
/// Klanderman, and W.J. Rucklidge. (PAMI 1993). :
pub trait HausdorffDistanceExtractor : crate::shape::ShapeDistanceExtractor {
    #[doc(hidden)] fn as_raw_HausdorffDistanceExtractor(&self) -> *mut c_void;
    // identifier: cv_HausdorffDistanceExtractor_setDistanceFlag_int_distanceFlag
    /// Set the norm used to compute the Hausdorff value between two shapes. It can be L1 or L2 norm.
    /// 
    /// ## Parameters
    /// * distanceFlag: Flag indicating which norm is used to compute the Hausdorff distance
    /// (NORM_L1, NORM_L2).
    fn set_distance_flag(&mut self, distance_flag: i32) -> Result<()> {
        unsafe { sys::cv_shape_cv_HausdorffDistanceExtractor_setDistanceFlag_int_distanceFlag(self.as_raw_HausdorffDistanceExtractor(), distance_flag) }.into_result()
    }
    
    // identifier: cv_HausdorffDistanceExtractor_getDistanceFlag_const
    fn get_distance_flag(&self) -> Result<i32> {
        unsafe { sys::cv_shape_cv_HausdorffDistanceExtractor_getDistanceFlag_const(self.as_raw_HausdorffDistanceExtractor()) }.into_result()
    }
    
    // identifier: cv_HausdorffDistanceExtractor_setRankProportion_float_rankProportion
    /// This method sets the rank proportion (or fractional value) that establish the Kth ranked value of
    /// the partial Hausdorff distance. Experimentally had been shown that 0.6 is a good value to compare
    /// shapes.
    /// 
    /// ## Parameters
    /// * rankProportion: fractional value (between 0 and 1).
    fn set_rank_proportion(&mut self, rank_proportion: f32) -> Result<()> {
        unsafe { sys::cv_shape_cv_HausdorffDistanceExtractor_setRankProportion_float_rankProportion(self.as_raw_HausdorffDistanceExtractor(), rank_proportion) }.into_result()
    }
    
    // identifier: cv_HausdorffDistanceExtractor_getRankProportion_const
    fn get_rank_proportion(&self) -> Result<f32> {
        unsafe { sys::cv_shape_cv_HausdorffDistanceExtractor_getRankProportion_const(self.as_raw_HausdorffDistanceExtractor()) }.into_result()
    }
    
}

impl<'a> HausdorffDistanceExtractor + 'a {

}

// Generating impl for trait cv::HistogramCostExtractor (trait)
/// Abstract base class for histogram cost algorithms.
pub trait HistogramCostExtractor : core::Algorithm {
    #[doc(hidden)] fn as_raw_HistogramCostExtractor(&self) -> *mut c_void;
    // identifier: cv_HistogramCostExtractor_buildCostMatrix_Mat_descriptors1_Mat_descriptors2_Mat_costMatrix
    fn build_cost_matrix(&mut self, descriptors1: &core::Mat, descriptors2: &core::Mat, cost_matrix: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_shape_cv_HistogramCostExtractor_buildCostMatrix_Mat_descriptors1_Mat_descriptors2_Mat_costMatrix(self.as_raw_HistogramCostExtractor(), descriptors1.as_raw_Mat(), descriptors2.as_raw_Mat(), cost_matrix.as_raw_Mat()) }.into_result()
    }
    
    // identifier: cv_HistogramCostExtractor_setNDummies_int_nDummies
    fn set_n_dummies(&mut self, n_dummies: i32) -> Result<()> {
        unsafe { sys::cv_shape_cv_HistogramCostExtractor_setNDummies_int_nDummies(self.as_raw_HistogramCostExtractor(), n_dummies) }.into_result()
    }
    
    // identifier: cv_HistogramCostExtractor_getNDummies_const
    fn get_n_dummies(&self) -> Result<i32> {
        unsafe { sys::cv_shape_cv_HistogramCostExtractor_getNDummies_const(self.as_raw_HistogramCostExtractor()) }.into_result()
    }
    
    // identifier: cv_HistogramCostExtractor_setDefaultCost_float_defaultCost
    fn set_default_cost(&mut self, default_cost: f32) -> Result<()> {
        unsafe { sys::cv_shape_cv_HistogramCostExtractor_setDefaultCost_float_defaultCost(self.as_raw_HistogramCostExtractor(), default_cost) }.into_result()
    }
    
    // identifier: cv_HistogramCostExtractor_getDefaultCost_const
    fn get_default_cost(&self) -> Result<f32> {
        unsafe { sys::cv_shape_cv_HistogramCostExtractor_getDefaultCost_const(self.as_raw_HistogramCostExtractor()) }.into_result()
    }
    
}

impl<'a> HistogramCostExtractor + 'a {

}

// Generating impl for trait cv::NormHistogramCostExtractor (trait)
/// A norm based cost extraction. :
pub trait NormHistogramCostExtractor : crate::shape::HistogramCostExtractor {
    #[doc(hidden)] fn as_raw_NormHistogramCostExtractor(&self) -> *mut c_void;
    // identifier: cv_NormHistogramCostExtractor_setNormFlag_int_flag
    fn set_norm_flag(&mut self, flag: i32) -> Result<()> {
        unsafe { sys::cv_shape_cv_NormHistogramCostExtractor_setNormFlag_int_flag(self.as_raw_NormHistogramCostExtractor(), flag) }.into_result()
    }
    
    // identifier: cv_NormHistogramCostExtractor_getNormFlag_const
    fn get_norm_flag(&self) -> Result<i32> {
        unsafe { sys::cv_shape_cv_NormHistogramCostExtractor_getNormFlag_const(self.as_raw_NormHistogramCostExtractor()) }.into_result()
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
pub trait ShapeContextDistanceExtractor : crate::shape::ShapeDistanceExtractor {
    #[doc(hidden)] fn as_raw_ShapeContextDistanceExtractor(&self) -> *mut c_void;
    // identifier: cv_ShapeContextDistanceExtractor_setAngularBins_int_nAngularBins
    /// Establish the number of angular bins for the Shape Context Descriptor used in the shape matching
    /// pipeline.
    /// 
    /// ## Parameters
    /// * nAngularBins: The number of angular bins in the shape context descriptor.
    fn set_angular_bins(&mut self, n_angular_bins: i32) -> Result<()> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_setAngularBins_int_nAngularBins(self.as_raw_ShapeContextDistanceExtractor(), n_angular_bins) }.into_result()
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_getAngularBins_const
    fn get_angular_bins(&self) -> Result<i32> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_getAngularBins_const(self.as_raw_ShapeContextDistanceExtractor()) }.into_result()
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_setRadialBins_int_nRadialBins
    /// Establish the number of radial bins for the Shape Context Descriptor used in the shape matching
    /// pipeline.
    /// 
    /// ## Parameters
    /// * nRadialBins: The number of radial bins in the shape context descriptor.
    fn set_radial_bins(&mut self, n_radial_bins: i32) -> Result<()> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_setRadialBins_int_nRadialBins(self.as_raw_ShapeContextDistanceExtractor(), n_radial_bins) }.into_result()
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_getRadialBins_const
    fn get_radial_bins(&self) -> Result<i32> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_getRadialBins_const(self.as_raw_ShapeContextDistanceExtractor()) }.into_result()
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_setInnerRadius_float_innerRadius
    /// Set the inner radius of the shape context descriptor.
    /// 
    /// ## Parameters
    /// * innerRadius: The value of the inner radius.
    fn set_inner_radius(&mut self, inner_radius: f32) -> Result<()> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_setInnerRadius_float_innerRadius(self.as_raw_ShapeContextDistanceExtractor(), inner_radius) }.into_result()
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_getInnerRadius_const
    fn get_inner_radius(&self) -> Result<f32> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_getInnerRadius_const(self.as_raw_ShapeContextDistanceExtractor()) }.into_result()
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_setOuterRadius_float_outerRadius
    /// Set the outer radius of the shape context descriptor.
    /// 
    /// ## Parameters
    /// * outerRadius: The value of the outer radius.
    fn set_outer_radius(&mut self, outer_radius: f32) -> Result<()> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_setOuterRadius_float_outerRadius(self.as_raw_ShapeContextDistanceExtractor(), outer_radius) }.into_result()
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_getOuterRadius_const
    fn get_outer_radius(&self) -> Result<f32> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_getOuterRadius_const(self.as_raw_ShapeContextDistanceExtractor()) }.into_result()
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_setRotationInvariant_bool_rotationInvariant
    fn set_rotation_invariant(&mut self, rotation_invariant: bool) -> Result<()> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_setRotationInvariant_bool_rotationInvariant(self.as_raw_ShapeContextDistanceExtractor(), rotation_invariant) }.into_result()
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_getRotationInvariant_const
    fn get_rotation_invariant(&self) -> Result<bool> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_getRotationInvariant_const(self.as_raw_ShapeContextDistanceExtractor()) }.into_result()
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_setShapeContextWeight_float_shapeContextWeight
    /// Set the weight of the shape context distance in the final value of the shape distance. The shape
    /// context distance between two shapes is defined as the symmetric sum of shape context matching costs
    /// over best matching points. The final value of the shape distance is a user-defined linear
    /// combination of the shape context distance, an image appearance distance, and a bending energy.
    /// 
    /// ## Parameters
    /// * shapeContextWeight: The weight of the shape context distance in the final distance value.
    fn set_shape_context_weight(&mut self, shape_context_weight: f32) -> Result<()> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_setShapeContextWeight_float_shapeContextWeight(self.as_raw_ShapeContextDistanceExtractor(), shape_context_weight) }.into_result()
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_getShapeContextWeight_const
    fn get_shape_context_weight(&self) -> Result<f32> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_getShapeContextWeight_const(self.as_raw_ShapeContextDistanceExtractor()) }.into_result()
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_setImageAppearanceWeight_float_imageAppearanceWeight
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
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_setImageAppearanceWeight_float_imageAppearanceWeight(self.as_raw_ShapeContextDistanceExtractor(), image_appearance_weight) }.into_result()
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_getImageAppearanceWeight_const
    fn get_image_appearance_weight(&self) -> Result<f32> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_getImageAppearanceWeight_const(self.as_raw_ShapeContextDistanceExtractor()) }.into_result()
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_setBendingEnergyWeight_float_bendingEnergyWeight
    /// Set the weight of the Bending Energy in the final value of the shape distance. The bending energy
    /// definition depends on what transformation is being used to align the shapes. The final value of the
    /// shape distance is a user-defined linear combination of the shape context distance, an image
    /// appearance distance, and a bending energy.
    /// 
    /// ## Parameters
    /// * bendingEnergyWeight: The weight of the Bending Energy in the final distance value.
    fn set_bending_energy_weight(&mut self, bending_energy_weight: f32) -> Result<()> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_setBendingEnergyWeight_float_bendingEnergyWeight(self.as_raw_ShapeContextDistanceExtractor(), bending_energy_weight) }.into_result()
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_getBendingEnergyWeight_const
    fn get_bending_energy_weight(&self) -> Result<f32> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_getBendingEnergyWeight_const(self.as_raw_ShapeContextDistanceExtractor()) }.into_result()
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_setImages_Mat_image1_Mat_image2
    /// Set the images that correspond to each shape. This images are used in the calculation of the Image
    /// Appearance cost.
    /// 
    /// ## Parameters
    /// * image1: Image corresponding to the shape defined by contours1.
    /// * image2: Image corresponding to the shape defined by contours2.
    fn set_images(&mut self, image1: &core::Mat, image2: &core::Mat) -> Result<()> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_setImages_Mat_image1_Mat_image2(self.as_raw_ShapeContextDistanceExtractor(), image1.as_raw_Mat(), image2.as_raw_Mat()) }.into_result()
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_getImages_const_Mat_image1_Mat_image2
    fn get_images(&self, image1: &mut core::Mat, image2: &mut core::Mat) -> Result<()> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_getImages_const_Mat_image1_Mat_image2(self.as_raw_ShapeContextDistanceExtractor(), image1.as_raw_Mat(), image2.as_raw_Mat()) }.into_result()
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_setIterations_int_iterations
    fn set_iterations(&mut self, iterations: i32) -> Result<()> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_setIterations_int_iterations(self.as_raw_ShapeContextDistanceExtractor(), iterations) }.into_result()
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_getIterations_const
    fn get_iterations(&self) -> Result<i32> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_getIterations_const(self.as_raw_ShapeContextDistanceExtractor()) }.into_result()
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_setCostExtractor_PtrOfHistogramCostExtractor_comparer
    /// Set the algorithm used for building the shape context descriptor cost matrix.
    /// 
    /// ## Parameters
    /// * comparer: Smart pointer to a HistogramCostExtractor, an algorithm that defines the cost
    /// matrix between descriptors.
    fn set_cost_extractor(&mut self, comparer: &types::PtrOfHistogramCostExtractor) -> Result<()> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_setCostExtractor_PtrOfHistogramCostExtractor_comparer(self.as_raw_ShapeContextDistanceExtractor(), comparer.as_raw_PtrOfHistogramCostExtractor()) }.into_result()
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_getCostExtractor_const
    fn get_cost_extractor(&self) -> Result<types::PtrOfHistogramCostExtractor> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_getCostExtractor_const(self.as_raw_ShapeContextDistanceExtractor()) }.into_result().map(|x| types::PtrOfHistogramCostExtractor { ptr: x })
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_setStdDev_float_sigma
    /// Set the value of the standard deviation for the Gaussian window for the image appearance cost.
    /// 
    /// ## Parameters
    /// * sigma: Standard Deviation.
    fn set_std_dev(&mut self, sigma: f32) -> Result<()> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_setStdDev_float_sigma(self.as_raw_ShapeContextDistanceExtractor(), sigma) }.into_result()
    }
    
    // identifier: cv_ShapeContextDistanceExtractor_getStdDev_const
    fn get_std_dev(&self) -> Result<f32> {
        unsafe { sys::cv_shape_cv_ShapeContextDistanceExtractor_getStdDev_const(self.as_raw_ShapeContextDistanceExtractor()) }.into_result()
    }
    
}

impl<'a> ShapeContextDistanceExtractor + 'a {

}

// Generating impl for trait cv::ShapeDistanceExtractor (trait)
/// Abstract base class for shape distance algorithms.
pub trait ShapeDistanceExtractor : core::Algorithm {
    #[doc(hidden)] fn as_raw_ShapeDistanceExtractor(&self) -> *mut c_void;
    // identifier: cv_ShapeDistanceExtractor_computeDistance_Mat_contour1_Mat_contour2
    /// Compute the shape distance between two shapes defined by its contours.
    /// 
    /// ## Parameters
    /// * contour1: Contour defining first shape.
    /// * contour2: Contour defining second shape.
    fn compute_distance(&mut self, contour1: &core::Mat, contour2: &core::Mat) -> Result<f32> {
        unsafe { sys::cv_shape_cv_ShapeDistanceExtractor_computeDistance_Mat_contour1_Mat_contour2(self.as_raw_ShapeDistanceExtractor(), contour1.as_raw_Mat(), contour2.as_raw_Mat()) }.into_result()
    }
    
}

impl<'a> ShapeDistanceExtractor + 'a {

}

// Generating impl for trait cv::ShapeTransformer (trait)
/// Abstract base class for shape transformation algorithms.
pub trait ShapeTransformer : core::Algorithm {
    #[doc(hidden)] fn as_raw_ShapeTransformer(&self) -> *mut c_void;
    // identifier: cv_ShapeTransformer_estimateTransformation_Mat_transformingShape_Mat_targetShape_VectorOfDMatch_matches
    /// Estimate the transformation parameters of the current transformer algorithm, based on point matches.
    /// 
    /// ## Parameters
    /// * transformingShape: Contour defining first shape.
    /// * targetShape: Contour defining second shape (Target).
    /// * matches: Standard vector of Matches between points.
    fn estimate_transformation(&mut self, transforming_shape: &core::Mat, target_shape: &core::Mat, matches: &types::VectorOfDMatch) -> Result<()> {
        unsafe { sys::cv_shape_cv_ShapeTransformer_estimateTransformation_Mat_transformingShape_Mat_targetShape_VectorOfDMatch_matches(self.as_raw_ShapeTransformer(), transforming_shape.as_raw_Mat(), target_shape.as_raw_Mat(), matches.as_raw_VectorOfDMatch()) }.into_result()
    }
    
    // identifier: cv_ShapeTransformer_applyTransformation_Mat_input_Mat_output
    /// Apply a transformation, given a pre-estimated transformation parameters.
    /// 
    /// ## Parameters
    /// * input: Contour (set of points) to apply the transformation.
    /// * output: Output contour.
    ///
    /// ## C++ default parameters:
    /// * output: noArray()
    fn apply_transformation(&mut self, input: &core::Mat, output: &mut core::Mat) -> Result<f32> {
        unsafe { sys::cv_shape_cv_ShapeTransformer_applyTransformation_Mat_input_Mat_output(self.as_raw_ShapeTransformer(), input.as_raw_Mat(), output.as_raw_Mat()) }.into_result()
    }
    
    // identifier: cv_ShapeTransformer_warpImage_const_Mat_transformingImage_Mat_output_int_flags_int_borderMode_Scalar_borderValue
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
        unsafe { sys::cv_shape_cv_ShapeTransformer_warpImage_const_Mat_transformingImage_Mat_output_int_flags_int_borderMode_Scalar_borderValue(self.as_raw_ShapeTransformer(), transforming_image.as_raw_Mat(), output.as_raw_Mat(), flags, border_mode, border_value) }.into_result()
    }
    
}

impl<'a> ShapeTransformer + 'a {

}

// Generating impl for trait cv::ThinPlateSplineShapeTransformer (trait)
/// Definition of the transformation
/// 
/// occupied in the paper "Principal Warps: Thin-Plate Splines and Decomposition of Deformations", by
/// F.L. Bookstein (PAMI 1989). :
pub trait ThinPlateSplineShapeTransformer : crate::shape::ShapeTransformer {
    #[doc(hidden)] fn as_raw_ThinPlateSplineShapeTransformer(&self) -> *mut c_void;
    // identifier: cv_ThinPlateSplineShapeTransformer_setRegularizationParameter_double_beta
    /// Set the regularization parameter for relaxing the exact interpolation requirements of the TPS
    /// algorithm.
    /// 
    /// ## Parameters
    /// * beta: value of the regularization parameter.
    fn set_regularization_parameter(&mut self, beta: f64) -> Result<()> {
        unsafe { sys::cv_shape_cv_ThinPlateSplineShapeTransformer_setRegularizationParameter_double_beta(self.as_raw_ThinPlateSplineShapeTransformer(), beta) }.into_result()
    }
    
    // identifier: cv_ThinPlateSplineShapeTransformer_getRegularizationParameter_const
    fn get_regularization_parameter(&self) -> Result<f64> {
        unsafe { sys::cv_shape_cv_ThinPlateSplineShapeTransformer_getRegularizationParameter_const(self.as_raw_ThinPlateSplineShapeTransformer()) }.into_result()
    }
    
}

impl<'a> ThinPlateSplineShapeTransformer + 'a {

}

