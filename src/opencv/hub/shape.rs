#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Shape Distance and Matching
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::ShapeTransformerConst, super::ShapeTransformer, super::ThinPlateSplineShapeTransformerConst, super::ThinPlateSplineShapeTransformer, super::AffineTransformerConst, super::AffineTransformer, super::HistogramCostExtractorConst, super::HistogramCostExtractor, super::NormHistogramCostExtractorConst, super::NormHistogramCostExtractor, super::EMDHistogramCostExtractorConst, super::EMDHistogramCostExtractor, super::ChiHistogramCostExtractorConst, super::ChiHistogramCostExtractor, super::EMDL1HistogramCostExtractorConst, super::EMDL1HistogramCostExtractor, super::ShapeDistanceExtractorConst, super::ShapeDistanceExtractor, super::ShapeContextDistanceExtractorConst, super::ShapeContextDistanceExtractor, super::HausdorffDistanceExtractorConst, super::HausdorffDistanceExtractor };
}

/// Computes the "minimal work" distance between two weighted point configurations base on the papers
/// "EMD-L1: An efficient and Robust Algorithm for comparing histogram-based descriptors", by Haibin
/// Ling and Kazunori Okuda; and "The Earth Mover's Distance is the Mallows Distance: Some Insights from
/// Statistics", by Elizaveta Levina and Peter Bickel.
/// 
/// ## Parameters
/// * signature1: First signature, a single column floating-point matrix. Each row is the value of
/// the histogram in each bin.
/// * signature2: Second signature of the same format and size as signature1.
#[inline]
pub fn emdl1(signature1: &dyn core::ToInputArray, signature2: &dyn core::ToInputArray) -> Result<f32> {
	input_array_arg!(signature1);
	input_array_arg!(signature2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_EMDL1_const__InputArrayR_const__InputArrayR(signature1.as_raw__InputArray(), signature2.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Complete constructor
#[inline]
pub fn create_affine_transformer(full_affine: bool) -> Result<core::Ptr<dyn crate::shape::AffineTransformer>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createAffineTransformer_bool(full_affine, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::shape::AffineTransformer>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * n_dummies: 25
/// * default_cost: 0.2f
#[inline]
pub fn create_chi_histogram_cost_extractor(n_dummies: i32, default_cost: f32) -> Result<core::Ptr<dyn crate::shape::HistogramCostExtractor>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createChiHistogramCostExtractor_int_float(n_dummies, default_cost, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::shape::HistogramCostExtractor>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * flag: DIST_L2
/// * n_dummies: 25
/// * default_cost: 0.2f
#[inline]
pub fn create_emd_histogram_cost_extractor(flag: i32, n_dummies: i32, default_cost: f32) -> Result<core::Ptr<dyn crate::shape::HistogramCostExtractor>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createEMDHistogramCostExtractor_int_int_float(flag, n_dummies, default_cost, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::shape::HistogramCostExtractor>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * n_dummies: 25
/// * default_cost: 0.2f
#[inline]
pub fn create_emdl1_histogram_cost_extractor(n_dummies: i32, default_cost: f32) -> Result<core::Ptr<dyn crate::shape::HistogramCostExtractor>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createEMDL1HistogramCostExtractor_int_float(n_dummies, default_cost, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::shape::HistogramCostExtractor>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * distance_flag: cv::NORM_L2
/// * rank_prop: 0.6f
#[inline]
pub fn create_hausdorff_distance_extractor(distance_flag: i32, rank_prop: f32) -> Result<core::Ptr<dyn crate::shape::HausdorffDistanceExtractor>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createHausdorffDistanceExtractor_int_float(distance_flag, rank_prop, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::shape::HausdorffDistanceExtractor>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * flag: DIST_L2
/// * n_dummies: 25
/// * default_cost: 0.2f
#[inline]
pub fn create_norm_histogram_cost_extractor(flag: i32, n_dummies: i32, default_cost: f32) -> Result<core::Ptr<dyn crate::shape::HistogramCostExtractor>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createNormHistogramCostExtractor_int_int_float(flag, n_dummies, default_cost, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::shape::HistogramCostExtractor>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * n_angular_bins: 12
/// * n_radial_bins: 4
/// * inner_radius: 0.2f
/// * outer_radius: 2
/// * iterations: 3
/// * comparer: createChiHistogramCostExtractor()
/// * transformer: createThinPlateSplineShapeTransformer()
#[inline]
pub fn create_shape_context_distance_extractor(n_angular_bins: i32, n_radial_bins: i32, inner_radius: f32, outer_radius: f32, iterations: i32, comparer: &core::Ptr<dyn crate::shape::HistogramCostExtractor>, transformer: &core::Ptr<dyn crate::shape::ShapeTransformer>) -> Result<core::Ptr<dyn crate::shape::ShapeContextDistanceExtractor>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createShapeContextDistanceExtractor_int_int_float_float_int_const_Ptr_HistogramCostExtractor_R_const_Ptr_ShapeTransformer_R(n_angular_bins, n_radial_bins, inner_radius, outer_radius, iterations, comparer.as_raw_PtrOfHistogramCostExtractor(), transformer.as_raw_PtrOfShapeTransformer(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::shape::ShapeContextDistanceExtractor>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Complete constructor
/// 
/// ## C++ default parameters
/// * regularization_parameter: 0
#[inline]
pub fn create_thin_plate_spline_shape_transformer(regularization_parameter: f64) -> Result<core::Ptr<dyn crate::shape::ThinPlateSplineShapeTransformer>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createThinPlateSplineShapeTransformer_double(regularization_parameter, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::shape::ThinPlateSplineShapeTransformer>::opencv_from_extern(ret) };
	Ok(ret)
}

/// Wrapper class for the OpenCV Affine Transformation algorithm. :
pub trait AffineTransformerConst: crate::shape::ShapeTransformerConst {
	fn as_raw_AffineTransformer(&self) -> *const c_void;

	#[inline]
	fn get_full_affine(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AffineTransformer_getFullAffine_const(self.as_raw_AffineTransformer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait AffineTransformer: crate::shape::AffineTransformerConst + crate::shape::ShapeTransformer {
	fn as_raw_mut_AffineTransformer(&mut self) -> *mut c_void;

	#[inline]
	fn set_full_affine(&mut self, full_affine: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AffineTransformer_setFullAffine_bool(self.as_raw_mut_AffineTransformer(), full_affine, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// An Chi based cost extraction. :
pub trait ChiHistogramCostExtractorConst: crate::shape::HistogramCostExtractorConst {
	fn as_raw_ChiHistogramCostExtractor(&self) -> *const c_void;

}

pub trait ChiHistogramCostExtractor: crate::shape::ChiHistogramCostExtractorConst + crate::shape::HistogramCostExtractor {
	fn as_raw_mut_ChiHistogramCostExtractor(&mut self) -> *mut c_void;

}

/// An EMD based cost extraction. :
pub trait EMDHistogramCostExtractorConst: crate::shape::HistogramCostExtractorConst {
	fn as_raw_EMDHistogramCostExtractor(&self) -> *const c_void;

	#[inline]
	fn get_norm_flag(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_EMDHistogramCostExtractor_getNormFlag_const(self.as_raw_EMDHistogramCostExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait EMDHistogramCostExtractor: crate::shape::EMDHistogramCostExtractorConst + crate::shape::HistogramCostExtractor {
	fn as_raw_mut_EMDHistogramCostExtractor(&mut self) -> *mut c_void;

	#[inline]
	fn set_norm_flag(&mut self, flag: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_EMDHistogramCostExtractor_setNormFlag_int(self.as_raw_mut_EMDHistogramCostExtractor(), flag, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// An EMD-L1 based cost extraction. :
pub trait EMDL1HistogramCostExtractorConst: crate::shape::HistogramCostExtractorConst {
	fn as_raw_EMDL1HistogramCostExtractor(&self) -> *const c_void;

}

pub trait EMDL1HistogramCostExtractor: crate::shape::EMDL1HistogramCostExtractorConst + crate::shape::HistogramCostExtractor {
	fn as_raw_mut_EMDL1HistogramCostExtractor(&mut self) -> *mut c_void;

}

/// ********************************************************************************
/// /
/// /
/// A simple Hausdorff distance measure between shapes defined by contours
/// 
/// according to the paper "Comparing Images using the Hausdorff distance." by D.P. Huttenlocher, G.A.
/// Klanderman, and W.J. Rucklidge. (PAMI 1993). :
pub trait HausdorffDistanceExtractorConst: crate::shape::ShapeDistanceExtractorConst {
	fn as_raw_HausdorffDistanceExtractor(&self) -> *const c_void;

	#[inline]
	fn get_distance_flag(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HausdorffDistanceExtractor_getDistanceFlag_const(self.as_raw_HausdorffDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_rank_proportion(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HausdorffDistanceExtractor_getRankProportion_const(self.as_raw_HausdorffDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait HausdorffDistanceExtractor: crate::shape::HausdorffDistanceExtractorConst + crate::shape::ShapeDistanceExtractor {
	fn as_raw_mut_HausdorffDistanceExtractor(&mut self) -> *mut c_void;

	/// Set the norm used to compute the Hausdorff value between two shapes. It can be L1 or L2 norm.
	/// 
	/// ## Parameters
	/// * distanceFlag: Flag indicating which norm is used to compute the Hausdorff distance
	/// (NORM_L1, NORM_L2).
	#[inline]
	fn set_distance_flag(&mut self, distance_flag: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HausdorffDistanceExtractor_setDistanceFlag_int(self.as_raw_mut_HausdorffDistanceExtractor(), distance_flag, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// This method sets the rank proportion (or fractional value) that establish the Kth ranked value of
	/// the partial Hausdorff distance. Experimentally had been shown that 0.6 is a good value to compare
	/// shapes.
	/// 
	/// ## Parameters
	/// * rankProportion: fractional value (between 0 and 1).
	#[inline]
	fn set_rank_proportion(&mut self, rank_proportion: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HausdorffDistanceExtractor_setRankProportion_float(self.as_raw_mut_HausdorffDistanceExtractor(), rank_proportion, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Abstract base class for histogram cost algorithms.
pub trait HistogramCostExtractorConst: core::AlgorithmTraitConst {
	fn as_raw_HistogramCostExtractor(&self) -> *const c_void;

	#[inline]
	fn get_n_dummies(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HistogramCostExtractor_getNDummies_const(self.as_raw_HistogramCostExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_default_cost(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HistogramCostExtractor_getDefaultCost_const(self.as_raw_HistogramCostExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait HistogramCostExtractor: core::AlgorithmTrait + crate::shape::HistogramCostExtractorConst {
	fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void;

	#[inline]
	fn build_cost_matrix(&mut self, descriptors1: &dyn core::ToInputArray, descriptors2: &dyn core::ToInputArray, cost_matrix: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(descriptors1);
		input_array_arg!(descriptors2);
		output_array_arg!(cost_matrix);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HistogramCostExtractor_buildCostMatrix_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_HistogramCostExtractor(), descriptors1.as_raw__InputArray(), descriptors2.as_raw__InputArray(), cost_matrix.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_n_dummies(&mut self, n_dummies: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HistogramCostExtractor_setNDummies_int(self.as_raw_mut_HistogramCostExtractor(), n_dummies, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_default_cost(&mut self, default_cost: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HistogramCostExtractor_setDefaultCost_float(self.as_raw_mut_HistogramCostExtractor(), default_cost, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// A norm based cost extraction. :
pub trait NormHistogramCostExtractorConst: crate::shape::HistogramCostExtractorConst {
	fn as_raw_NormHistogramCostExtractor(&self) -> *const c_void;

	#[inline]
	fn get_norm_flag(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_NormHistogramCostExtractor_getNormFlag_const(self.as_raw_NormHistogramCostExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait NormHistogramCostExtractor: crate::shape::HistogramCostExtractor + crate::shape::NormHistogramCostExtractorConst {
	fn as_raw_mut_NormHistogramCostExtractor(&mut self) -> *mut c_void;

	#[inline]
	fn set_norm_flag(&mut self, flag: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_NormHistogramCostExtractor_setNormFlag_int(self.as_raw_mut_NormHistogramCostExtractor(), flag, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// ********************************************************************************
/// /
/// /
/// Implementation of the Shape Context descriptor and matching algorithm
/// 
/// proposed by Belongie et al. in "Shape Matching and Object Recognition Using Shape Contexts" (PAMI
/// 2002). This implementation is packaged in a generic scheme, in order to allow you the
/// implementation of the common variations of the original pipeline.
pub trait ShapeContextDistanceExtractorConst: crate::shape::ShapeDistanceExtractorConst {
	fn as_raw_ShapeContextDistanceExtractor(&self) -> *const c_void;

	#[inline]
	fn get_angular_bins(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getAngularBins_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_radial_bins(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getRadialBins_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_inner_radius(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getInnerRadius_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_outer_radius(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getOuterRadius_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_rotation_invariant(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getRotationInvariant_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_shape_context_weight(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getShapeContextWeight_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_image_appearance_weight(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getImageAppearanceWeight_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_bending_energy_weight(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getBendingEnergyWeight_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_images(&self, image1: &mut dyn core::ToOutputArray, image2: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(image1);
		output_array_arg!(image2);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getImages_const_const__OutputArrayR_const__OutputArrayR(self.as_raw_ShapeContextDistanceExtractor(), image1.as_raw__OutputArray(), image2.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getIterations_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_cost_extractor(&self) -> Result<core::Ptr<dyn crate::shape::HistogramCostExtractor>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getCostExtractor_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::shape::HistogramCostExtractor>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn get_std_dev(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getStdDev_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_transform_algorithm(&self) -> Result<core::Ptr<dyn crate::shape::ShapeTransformer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getTransformAlgorithm_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::shape::ShapeTransformer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait ShapeContextDistanceExtractor: crate::shape::ShapeContextDistanceExtractorConst + crate::shape::ShapeDistanceExtractor {
	fn as_raw_mut_ShapeContextDistanceExtractor(&mut self) -> *mut c_void;

	/// Establish the number of angular bins for the Shape Context Descriptor used in the shape matching
	/// pipeline.
	/// 
	/// ## Parameters
	/// * nAngularBins: The number of angular bins in the shape context descriptor.
	#[inline]
	fn set_angular_bins(&mut self, n_angular_bins: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setAngularBins_int(self.as_raw_mut_ShapeContextDistanceExtractor(), n_angular_bins, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Establish the number of radial bins for the Shape Context Descriptor used in the shape matching
	/// pipeline.
	/// 
	/// ## Parameters
	/// * nRadialBins: The number of radial bins in the shape context descriptor.
	#[inline]
	fn set_radial_bins(&mut self, n_radial_bins: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setRadialBins_int(self.as_raw_mut_ShapeContextDistanceExtractor(), n_radial_bins, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Set the inner radius of the shape context descriptor.
	/// 
	/// ## Parameters
	/// * innerRadius: The value of the inner radius.
	#[inline]
	fn set_inner_radius(&mut self, inner_radius: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setInnerRadius_float(self.as_raw_mut_ShapeContextDistanceExtractor(), inner_radius, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Set the outer radius of the shape context descriptor.
	/// 
	/// ## Parameters
	/// * outerRadius: The value of the outer radius.
	#[inline]
	fn set_outer_radius(&mut self, outer_radius: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setOuterRadius_float(self.as_raw_mut_ShapeContextDistanceExtractor(), outer_radius, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_rotation_invariant(&mut self, rotation_invariant: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setRotationInvariant_bool(self.as_raw_mut_ShapeContextDistanceExtractor(), rotation_invariant, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Set the weight of the shape context distance in the final value of the shape distance. The shape
	/// context distance between two shapes is defined as the symmetric sum of shape context matching costs
	/// over best matching points. The final value of the shape distance is a user-defined linear
	/// combination of the shape context distance, an image appearance distance, and a bending energy.
	/// 
	/// ## Parameters
	/// * shapeContextWeight: The weight of the shape context distance in the final distance value.
	#[inline]
	fn set_shape_context_weight(&mut self, shape_context_weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setShapeContextWeight_float(self.as_raw_mut_ShapeContextDistanceExtractor(), shape_context_weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
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
	#[inline]
	fn set_image_appearance_weight(&mut self, image_appearance_weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setImageAppearanceWeight_float(self.as_raw_mut_ShapeContextDistanceExtractor(), image_appearance_weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Set the weight of the Bending Energy in the final value of the shape distance. The bending energy
	/// definition depends on what transformation is being used to align the shapes. The final value of the
	/// shape distance is a user-defined linear combination of the shape context distance, an image
	/// appearance distance, and a bending energy.
	/// 
	/// ## Parameters
	/// * bendingEnergyWeight: The weight of the Bending Energy in the final distance value.
	#[inline]
	fn set_bending_energy_weight(&mut self, bending_energy_weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setBendingEnergyWeight_float(self.as_raw_mut_ShapeContextDistanceExtractor(), bending_energy_weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Set the images that correspond to each shape. This images are used in the calculation of the Image
	/// Appearance cost.
	/// 
	/// ## Parameters
	/// * image1: Image corresponding to the shape defined by contours1.
	/// * image2: Image corresponding to the shape defined by contours2.
	#[inline]
	fn set_images(&mut self, image1: &dyn core::ToInputArray, image2: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(image1);
		input_array_arg!(image2);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setImages_const__InputArrayR_const__InputArrayR(self.as_raw_mut_ShapeContextDistanceExtractor(), image1.as_raw__InputArray(), image2.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_iterations(&mut self, iterations: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setIterations_int(self.as_raw_mut_ShapeContextDistanceExtractor(), iterations, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Set the algorithm used for building the shape context descriptor cost matrix.
	/// 
	/// ## Parameters
	/// * comparer: Smart pointer to a HistogramCostExtractor, an algorithm that defines the cost
	/// matrix between descriptors.
	#[inline]
	fn set_cost_extractor(&mut self, mut comparer: core::Ptr<dyn crate::shape::HistogramCostExtractor>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setCostExtractor_Ptr_HistogramCostExtractor_(self.as_raw_mut_ShapeContextDistanceExtractor(), comparer.as_raw_mut_PtrOfHistogramCostExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Set the value of the standard deviation for the Gaussian window for the image appearance cost.
	/// 
	/// ## Parameters
	/// * sigma: Standard Deviation.
	#[inline]
	fn set_std_dev(&mut self, sigma: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setStdDev_float(self.as_raw_mut_ShapeContextDistanceExtractor(), sigma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Set the algorithm used for aligning the shapes.
	/// 
	/// ## Parameters
	/// * transformer: Smart pointer to a ShapeTransformer, an algorithm that defines the aligning
	/// transformation.
	#[inline]
	fn set_transform_algorithm(&mut self, mut transformer: core::Ptr<dyn crate::shape::ShapeTransformer>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setTransformAlgorithm_Ptr_ShapeTransformer_(self.as_raw_mut_ShapeContextDistanceExtractor(), transformer.as_raw_mut_PtrOfShapeTransformer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// @example modules/shape/samples/shape_example.cpp
/// An example using shape distance algorithm
/// 
/// Abstract base class for shape distance algorithms.
pub trait ShapeDistanceExtractorConst: core::AlgorithmTraitConst {
	fn as_raw_ShapeDistanceExtractor(&self) -> *const c_void;

}

pub trait ShapeDistanceExtractor: core::AlgorithmTrait + crate::shape::ShapeDistanceExtractorConst {
	fn as_raw_mut_ShapeDistanceExtractor(&mut self) -> *mut c_void;

	/// Compute the shape distance between two shapes defined by its contours.
	/// 
	/// ## Parameters
	/// * contour1: Contour defining first shape.
	/// * contour2: Contour defining second shape.
	#[inline]
	fn compute_distance(&mut self, contour1: &dyn core::ToInputArray, contour2: &dyn core::ToInputArray) -> Result<f32> {
		input_array_arg!(contour1);
		input_array_arg!(contour2);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeDistanceExtractor_computeDistance_const__InputArrayR_const__InputArrayR(self.as_raw_mut_ShapeDistanceExtractor(), contour1.as_raw__InputArray(), contour2.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Abstract base class for shape transformation algorithms.
pub trait ShapeTransformerConst: core::AlgorithmTraitConst {
	fn as_raw_ShapeTransformer(&self) -> *const c_void;

	/// Apply a transformation, given a pre-estimated transformation parameters, to an Image.
	/// 
	/// ## Parameters
	/// * transformingImage: Input image.
	/// * output: Output image.
	/// * flags: Image interpolation method.
	/// * borderMode: border style.
	/// * borderValue: border value.
	/// 
	/// ## C++ default parameters
	/// * flags: INTER_LINEAR
	/// * border_mode: BORDER_CONSTANT
	/// * border_value: Scalar()
	#[inline]
	fn warp_image(&self, transforming_image: &dyn core::ToInputArray, output: &mut dyn core::ToOutputArray, flags: i32, border_mode: i32, border_value: core::Scalar) -> Result<()> {
		input_array_arg!(transforming_image);
		output_array_arg!(output);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeTransformer_warpImage_const_const__InputArrayR_const__OutputArrayR_int_int_const_ScalarR(self.as_raw_ShapeTransformer(), transforming_image.as_raw__InputArray(), output.as_raw__OutputArray(), flags, border_mode, &border_value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait ShapeTransformer: core::AlgorithmTrait + crate::shape::ShapeTransformerConst {
	fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void;

	/// Estimate the transformation parameters of the current transformer algorithm, based on point matches.
	/// 
	/// ## Parameters
	/// * transformingShape: Contour defining first shape.
	/// * targetShape: Contour defining second shape (Target).
	/// * matches: Standard vector of Matches between points.
	#[inline]
	fn estimate_transformation(&mut self, transforming_shape: &dyn core::ToInputArray, target_shape: &dyn core::ToInputArray, matches: &mut core::Vector<core::DMatch>) -> Result<()> {
		input_array_arg!(transforming_shape);
		input_array_arg!(target_shape);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeTransformer_estimateTransformation_const__InputArrayR_const__InputArrayR_vector_DMatch_R(self.as_raw_mut_ShapeTransformer(), transforming_shape.as_raw__InputArray(), target_shape.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Apply a transformation, given a pre-estimated transformation parameters.
	/// 
	/// ## Parameters
	/// * input: Contour (set of points) to apply the transformation.
	/// * output: Output contour.
	/// 
	/// ## C++ default parameters
	/// * output: noArray()
	#[inline]
	fn apply_transformation(&mut self, input: &dyn core::ToInputArray, output: &mut dyn core::ToOutputArray) -> Result<f32> {
		input_array_arg!(input);
		output_array_arg!(output);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeTransformer_applyTransformation_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_ShapeTransformer(), input.as_raw__InputArray(), output.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Definition of the transformation
/// 
/// occupied in the paper "Principal Warps: Thin-Plate Splines and Decomposition of Deformations", by
/// F.L. Bookstein (PAMI 1989). :
pub trait ThinPlateSplineShapeTransformerConst: crate::shape::ShapeTransformerConst {
	fn as_raw_ThinPlateSplineShapeTransformer(&self) -> *const c_void;

	#[inline]
	fn get_regularization_parameter(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ThinPlateSplineShapeTransformer_getRegularizationParameter_const(self.as_raw_ThinPlateSplineShapeTransformer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait ThinPlateSplineShapeTransformer: crate::shape::ShapeTransformer + crate::shape::ThinPlateSplineShapeTransformerConst {
	fn as_raw_mut_ThinPlateSplineShapeTransformer(&mut self) -> *mut c_void;

	/// Set the regularization parameter for relaxing the exact interpolation requirements of the TPS
	/// algorithm.
	/// 
	/// ## Parameters
	/// * beta: value of the regularization parameter.
	#[inline]
	fn set_regularization_parameter(&mut self, beta: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ThinPlateSplineShapeTransformer_setRegularizationParameter_double(self.as_raw_mut_ThinPlateSplineShapeTransformer(), beta, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}
