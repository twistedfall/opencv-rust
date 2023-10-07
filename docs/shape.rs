pub mod shape {
	//! # Shape Distance and Matching
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::ShapeTransformerTraitConst, super::ShapeTransformerTrait, super::ThinPlateSplineShapeTransformerTraitConst, super::ThinPlateSplineShapeTransformerTrait, super::AffineTransformerTraitConst, super::AffineTransformerTrait, super::HistogramCostExtractorTraitConst, super::HistogramCostExtractorTrait, super::NormHistogramCostExtractorTraitConst, super::NormHistogramCostExtractorTrait, super::EMDHistogramCostExtractorTraitConst, super::EMDHistogramCostExtractorTrait, super::ChiHistogramCostExtractorTraitConst, super::ChiHistogramCostExtractorTrait, super::EMDL1HistogramCostExtractorTraitConst, super::EMDL1HistogramCostExtractorTrait, super::ShapeDistanceExtractorTraitConst, super::ShapeDistanceExtractorTrait, super::ShapeContextDistanceExtractorTraitConst, super::ShapeContextDistanceExtractorTrait, super::HausdorffDistanceExtractorTraitConst, super::HausdorffDistanceExtractorTrait };
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
	pub fn emdl1(signature1: &impl core::ToInputArray, signature2: &impl core::ToInputArray) -> Result<f32> {
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
	pub fn create_affine_transformer(full_affine: bool) -> Result<core::Ptr<crate::shape::AffineTransformer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_createAffineTransformer_bool(full_affine, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::shape::AffineTransformer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [create_chi_histogram_cost_extractor] function uses the following default values for its arguments:
	/// * n_dummies: 25
	/// * default_cost: 0.2f
	#[inline]
	pub fn create_chi_histogram_cost_extractor_def() -> Result<core::Ptr<crate::shape::HistogramCostExtractor>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_createChiHistogramCostExtractor(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::shape::HistogramCostExtractor>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * n_dummies: 25
	/// * default_cost: 0.2f
	#[inline]
	pub fn create_chi_histogram_cost_extractor(n_dummies: i32, default_cost: f32) -> Result<core::Ptr<crate::shape::HistogramCostExtractor>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_createChiHistogramCostExtractor_int_float(n_dummies, default_cost, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::shape::HistogramCostExtractor>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [create_emd_histogram_cost_extractor] function uses the following default values for its arguments:
	/// * flag: DIST_L2
	/// * n_dummies: 25
	/// * default_cost: 0.2f
	#[inline]
	pub fn create_emd_histogram_cost_extractor_def() -> Result<core::Ptr<crate::shape::HistogramCostExtractor>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_createEMDHistogramCostExtractor(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::shape::HistogramCostExtractor>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * flag: DIST_L2
	/// * n_dummies: 25
	/// * default_cost: 0.2f
	#[inline]
	pub fn create_emd_histogram_cost_extractor(flag: i32, n_dummies: i32, default_cost: f32) -> Result<core::Ptr<crate::shape::HistogramCostExtractor>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_createEMDHistogramCostExtractor_int_int_float(flag, n_dummies, default_cost, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::shape::HistogramCostExtractor>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [create_emdl1_histogram_cost_extractor] function uses the following default values for its arguments:
	/// * n_dummies: 25
	/// * default_cost: 0.2f
	#[inline]
	pub fn create_emdl1_histogram_cost_extractor_def() -> Result<core::Ptr<crate::shape::HistogramCostExtractor>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_createEMDL1HistogramCostExtractor(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::shape::HistogramCostExtractor>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * n_dummies: 25
	/// * default_cost: 0.2f
	#[inline]
	pub fn create_emdl1_histogram_cost_extractor(n_dummies: i32, default_cost: f32) -> Result<core::Ptr<crate::shape::HistogramCostExtractor>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_createEMDL1HistogramCostExtractor_int_float(n_dummies, default_cost, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::shape::HistogramCostExtractor>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [create_hausdorff_distance_extractor] function uses the following default values for its arguments:
	/// * distance_flag: cv::NORM_L2
	/// * rank_prop: 0.6f
	#[inline]
	pub fn create_hausdorff_distance_extractor_def() -> Result<core::Ptr<crate::shape::HausdorffDistanceExtractor>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_createHausdorffDistanceExtractor(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::shape::HausdorffDistanceExtractor>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * distance_flag: cv::NORM_L2
	/// * rank_prop: 0.6f
	#[inline]
	pub fn create_hausdorff_distance_extractor(distance_flag: i32, rank_prop: f32) -> Result<core::Ptr<crate::shape::HausdorffDistanceExtractor>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_createHausdorffDistanceExtractor_int_float(distance_flag, rank_prop, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::shape::HausdorffDistanceExtractor>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [create_norm_histogram_cost_extractor] function uses the following default values for its arguments:
	/// * flag: DIST_L2
	/// * n_dummies: 25
	/// * default_cost: 0.2f
	#[inline]
	pub fn create_norm_histogram_cost_extractor_def() -> Result<core::Ptr<crate::shape::HistogramCostExtractor>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_createNormHistogramCostExtractor(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::shape::HistogramCostExtractor>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * flag: DIST_L2
	/// * n_dummies: 25
	/// * default_cost: 0.2f
	#[inline]
	pub fn create_norm_histogram_cost_extractor(flag: i32, n_dummies: i32, default_cost: f32) -> Result<core::Ptr<crate::shape::HistogramCostExtractor>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_createNormHistogramCostExtractor_int_int_float(flag, n_dummies, default_cost, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::shape::HistogramCostExtractor>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [create_shape_context_distance_extractor] function uses the following default values for its arguments:
	/// * n_angular_bins: 12
	/// * n_radial_bins: 4
	/// * inner_radius: 0.2f
	/// * outer_radius: 2
	/// * iterations: 3
	/// * comparer: createChiHistogramCostExtractor()
	/// * transformer: createThinPlateSplineShapeTransformer()
	#[inline]
	pub fn create_shape_context_distance_extractor_def() -> Result<core::Ptr<crate::shape::ShapeContextDistanceExtractor>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_createShapeContextDistanceExtractor(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::shape::ShapeContextDistanceExtractor>::opencv_from_extern(ret) };
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
	pub fn create_shape_context_distance_extractor(n_angular_bins: i32, n_radial_bins: i32, inner_radius: f32, outer_radius: f32, iterations: i32, comparer: &core::Ptr<crate::shape::HistogramCostExtractor>, transformer: &core::Ptr<crate::shape::ShapeTransformer>) -> Result<core::Ptr<crate::shape::ShapeContextDistanceExtractor>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_createShapeContextDistanceExtractor_int_int_float_float_int_const_PtrLHistogramCostExtractorGR_const_PtrLShapeTransformerGR(n_angular_bins, n_radial_bins, inner_radius, outer_radius, iterations, comparer.as_raw_PtrOfHistogramCostExtractor(), transformer.as_raw_PtrOfShapeTransformer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::shape::ShapeContextDistanceExtractor>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Complete constructor
	/// 
	/// ## Note
	/// This alternative version of [create_thin_plate_spline_shape_transformer] function uses the following default values for its arguments:
	/// * regularization_parameter: 0
	#[inline]
	pub fn create_thin_plate_spline_shape_transformer_def() -> Result<core::Ptr<crate::shape::ThinPlateSplineShapeTransformer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_createThinPlateSplineShapeTransformer(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::shape::ThinPlateSplineShapeTransformer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Complete constructor
	/// 
	/// ## C++ default parameters
	/// * regularization_parameter: 0
	#[inline]
	pub fn create_thin_plate_spline_shape_transformer(regularization_parameter: f64) -> Result<core::Ptr<crate::shape::ThinPlateSplineShapeTransformer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_createThinPlateSplineShapeTransformer_double(regularization_parameter, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::shape::ThinPlateSplineShapeTransformer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Constant methods for [crate::shape::AffineTransformer]
	pub trait AffineTransformerTraitConst: crate::shape::ShapeTransformerTraitConst {
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
	
	/// Mutable methods for [crate::shape::AffineTransformer]
	pub trait AffineTransformerTrait: crate::shape::AffineTransformerTraitConst + crate::shape::ShapeTransformerTrait {
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
	
	/// Wrapper class for the OpenCV Affine Transformation algorithm. :
	pub struct AffineTransformer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { AffineTransformer }
	
	impl Drop for AffineTransformer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_AffineTransformer_delete(self.as_raw_mut_AffineTransformer()) };
		}
	}
	
	unsafe impl Send for AffineTransformer {}
	
	impl core::AlgorithmTraitConst for AffineTransformer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for AffineTransformer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::ShapeTransformerTraitConst for AffineTransformer {
		#[inline] fn as_raw_ShapeTransformer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::shape::ShapeTransformerTrait for AffineTransformer {
		#[inline] fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::AffineTransformerTraitConst for AffineTransformer {
		#[inline] fn as_raw_AffineTransformer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::shape::AffineTransformerTrait for AffineTransformer {
		#[inline] fn as_raw_mut_AffineTransformer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl AffineTransformer {
	}
	
	boxed_cast_base! { AffineTransformer, core::Algorithm, cv_AffineTransformer_to_Algorithm }
	
	boxed_cast_base! { AffineTransformer, crate::shape::ShapeTransformer, cv_AffineTransformer_to_ShapeTransformer }
	
	impl std::fmt::Debug for AffineTransformer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("AffineTransformer")
				.finish()
		}
	}
	
	/// Constant methods for [crate::shape::ChiHistogramCostExtractor]
	pub trait ChiHistogramCostExtractorTraitConst: crate::shape::HistogramCostExtractorTraitConst {
		fn as_raw_ChiHistogramCostExtractor(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::shape::ChiHistogramCostExtractor]
	pub trait ChiHistogramCostExtractorTrait: crate::shape::ChiHistogramCostExtractorTraitConst + crate::shape::HistogramCostExtractorTrait {
		fn as_raw_mut_ChiHistogramCostExtractor(&mut self) -> *mut c_void;
	
	}
	
	/// An Chi based cost extraction. :
	pub struct ChiHistogramCostExtractor {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ChiHistogramCostExtractor }
	
	impl Drop for ChiHistogramCostExtractor {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_ChiHistogramCostExtractor_delete(self.as_raw_mut_ChiHistogramCostExtractor()) };
		}
	}
	
	unsafe impl Send for ChiHistogramCostExtractor {}
	
	impl core::AlgorithmTraitConst for ChiHistogramCostExtractor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ChiHistogramCostExtractor {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::HistogramCostExtractorTraitConst for ChiHistogramCostExtractor {
		#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::shape::HistogramCostExtractorTrait for ChiHistogramCostExtractor {
		#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::ChiHistogramCostExtractorTraitConst for ChiHistogramCostExtractor {
		#[inline] fn as_raw_ChiHistogramCostExtractor(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::shape::ChiHistogramCostExtractorTrait for ChiHistogramCostExtractor {
		#[inline] fn as_raw_mut_ChiHistogramCostExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ChiHistogramCostExtractor {
	}
	
	boxed_cast_base! { ChiHistogramCostExtractor, core::Algorithm, cv_ChiHistogramCostExtractor_to_Algorithm }
	
	boxed_cast_base! { ChiHistogramCostExtractor, crate::shape::HistogramCostExtractor, cv_ChiHistogramCostExtractor_to_HistogramCostExtractor }
	
	impl std::fmt::Debug for ChiHistogramCostExtractor {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ChiHistogramCostExtractor")
				.finish()
		}
	}
	
	/// Constant methods for [crate::shape::EMDHistogramCostExtractor]
	pub trait EMDHistogramCostExtractorTraitConst: crate::shape::HistogramCostExtractorTraitConst {
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
	
	/// Mutable methods for [crate::shape::EMDHistogramCostExtractor]
	pub trait EMDHistogramCostExtractorTrait: crate::shape::EMDHistogramCostExtractorTraitConst + crate::shape::HistogramCostExtractorTrait {
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
	
	/// An EMD based cost extraction. :
	pub struct EMDHistogramCostExtractor {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { EMDHistogramCostExtractor }
	
	impl Drop for EMDHistogramCostExtractor {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_EMDHistogramCostExtractor_delete(self.as_raw_mut_EMDHistogramCostExtractor()) };
		}
	}
	
	unsafe impl Send for EMDHistogramCostExtractor {}
	
	impl core::AlgorithmTraitConst for EMDHistogramCostExtractor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for EMDHistogramCostExtractor {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::HistogramCostExtractorTraitConst for EMDHistogramCostExtractor {
		#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::shape::HistogramCostExtractorTrait for EMDHistogramCostExtractor {
		#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::EMDHistogramCostExtractorTraitConst for EMDHistogramCostExtractor {
		#[inline] fn as_raw_EMDHistogramCostExtractor(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::shape::EMDHistogramCostExtractorTrait for EMDHistogramCostExtractor {
		#[inline] fn as_raw_mut_EMDHistogramCostExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl EMDHistogramCostExtractor {
	}
	
	boxed_cast_base! { EMDHistogramCostExtractor, core::Algorithm, cv_EMDHistogramCostExtractor_to_Algorithm }
	
	boxed_cast_base! { EMDHistogramCostExtractor, crate::shape::HistogramCostExtractor, cv_EMDHistogramCostExtractor_to_HistogramCostExtractor }
	
	impl std::fmt::Debug for EMDHistogramCostExtractor {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("EMDHistogramCostExtractor")
				.finish()
		}
	}
	
	/// Constant methods for [crate::shape::EMDL1HistogramCostExtractor]
	pub trait EMDL1HistogramCostExtractorTraitConst: crate::shape::HistogramCostExtractorTraitConst {
		fn as_raw_EMDL1HistogramCostExtractor(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::shape::EMDL1HistogramCostExtractor]
	pub trait EMDL1HistogramCostExtractorTrait: crate::shape::EMDL1HistogramCostExtractorTraitConst + crate::shape::HistogramCostExtractorTrait {
		fn as_raw_mut_EMDL1HistogramCostExtractor(&mut self) -> *mut c_void;
	
	}
	
	/// An EMD-L1 based cost extraction. :
	pub struct EMDL1HistogramCostExtractor {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { EMDL1HistogramCostExtractor }
	
	impl Drop for EMDL1HistogramCostExtractor {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_EMDL1HistogramCostExtractor_delete(self.as_raw_mut_EMDL1HistogramCostExtractor()) };
		}
	}
	
	unsafe impl Send for EMDL1HistogramCostExtractor {}
	
	impl core::AlgorithmTraitConst for EMDL1HistogramCostExtractor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for EMDL1HistogramCostExtractor {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::HistogramCostExtractorTraitConst for EMDL1HistogramCostExtractor {
		#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::shape::HistogramCostExtractorTrait for EMDL1HistogramCostExtractor {
		#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::EMDL1HistogramCostExtractorTraitConst for EMDL1HistogramCostExtractor {
		#[inline] fn as_raw_EMDL1HistogramCostExtractor(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::shape::EMDL1HistogramCostExtractorTrait for EMDL1HistogramCostExtractor {
		#[inline] fn as_raw_mut_EMDL1HistogramCostExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl EMDL1HistogramCostExtractor {
	}
	
	boxed_cast_base! { EMDL1HistogramCostExtractor, core::Algorithm, cv_EMDL1HistogramCostExtractor_to_Algorithm }
	
	boxed_cast_base! { EMDL1HistogramCostExtractor, crate::shape::HistogramCostExtractor, cv_EMDL1HistogramCostExtractor_to_HistogramCostExtractor }
	
	impl std::fmt::Debug for EMDL1HistogramCostExtractor {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("EMDL1HistogramCostExtractor")
				.finish()
		}
	}
	
	/// Constant methods for [crate::shape::HausdorffDistanceExtractor]
	pub trait HausdorffDistanceExtractorTraitConst: crate::shape::ShapeDistanceExtractorTraitConst {
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
	
	/// Mutable methods for [crate::shape::HausdorffDistanceExtractor]
	pub trait HausdorffDistanceExtractorTrait: crate::shape::HausdorffDistanceExtractorTraitConst + crate::shape::ShapeDistanceExtractorTrait {
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
	
	/// ********************************************************************************
	/// /
	/// /
	/// A simple Hausdorff distance measure between shapes defined by contours
	/// 
	/// according to the paper "Comparing Images using the Hausdorff distance." by D.P. Huttenlocher, G.A.
	/// Klanderman, and W.J. Rucklidge. (PAMI 1993). :
	pub struct HausdorffDistanceExtractor {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { HausdorffDistanceExtractor }
	
	impl Drop for HausdorffDistanceExtractor {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_HausdorffDistanceExtractor_delete(self.as_raw_mut_HausdorffDistanceExtractor()) };
		}
	}
	
	unsafe impl Send for HausdorffDistanceExtractor {}
	
	impl core::AlgorithmTraitConst for HausdorffDistanceExtractor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for HausdorffDistanceExtractor {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::ShapeDistanceExtractorTraitConst for HausdorffDistanceExtractor {
		#[inline] fn as_raw_ShapeDistanceExtractor(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::shape::ShapeDistanceExtractorTrait for HausdorffDistanceExtractor {
		#[inline] fn as_raw_mut_ShapeDistanceExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::HausdorffDistanceExtractorTraitConst for HausdorffDistanceExtractor {
		#[inline] fn as_raw_HausdorffDistanceExtractor(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::shape::HausdorffDistanceExtractorTrait for HausdorffDistanceExtractor {
		#[inline] fn as_raw_mut_HausdorffDistanceExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl HausdorffDistanceExtractor {
	}
	
	boxed_cast_base! { HausdorffDistanceExtractor, core::Algorithm, cv_HausdorffDistanceExtractor_to_Algorithm }
	
	boxed_cast_base! { HausdorffDistanceExtractor, crate::shape::ShapeDistanceExtractor, cv_HausdorffDistanceExtractor_to_ShapeDistanceExtractor }
	
	impl std::fmt::Debug for HausdorffDistanceExtractor {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("HausdorffDistanceExtractor")
				.finish()
		}
	}
	
	/// Constant methods for [crate::shape::HistogramCostExtractor]
	pub trait HistogramCostExtractorTraitConst: core::AlgorithmTraitConst {
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
	
	/// Mutable methods for [crate::shape::HistogramCostExtractor]
	pub trait HistogramCostExtractorTrait: core::AlgorithmTrait + crate::shape::HistogramCostExtractorTraitConst {
		fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void;
	
		#[inline]
		fn build_cost_matrix(&mut self, descriptors1: &impl core::ToInputArray, descriptors2: &impl core::ToInputArray, cost_matrix: &mut impl core::ToOutputArray) -> Result<()> {
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
	
	/// Abstract base class for histogram cost algorithms.
	pub struct HistogramCostExtractor {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { HistogramCostExtractor }
	
	impl Drop for HistogramCostExtractor {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_HistogramCostExtractor_delete(self.as_raw_mut_HistogramCostExtractor()) };
		}
	}
	
	unsafe impl Send for HistogramCostExtractor {}
	
	impl core::AlgorithmTraitConst for HistogramCostExtractor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for HistogramCostExtractor {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::HistogramCostExtractorTraitConst for HistogramCostExtractor {
		#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::shape::HistogramCostExtractorTrait for HistogramCostExtractor {
		#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl HistogramCostExtractor {
	}
	
	boxed_cast_descendant! { HistogramCostExtractor, crate::shape::ChiHistogramCostExtractor, cv_HistogramCostExtractor_to_ChiHistogramCostExtractor }
	
	boxed_cast_descendant! { HistogramCostExtractor, crate::shape::EMDHistogramCostExtractor, cv_HistogramCostExtractor_to_EMDHistogramCostExtractor }
	
	boxed_cast_descendant! { HistogramCostExtractor, crate::shape::EMDL1HistogramCostExtractor, cv_HistogramCostExtractor_to_EMDL1HistogramCostExtractor }
	
	boxed_cast_descendant! { HistogramCostExtractor, crate::shape::NormHistogramCostExtractor, cv_HistogramCostExtractor_to_NormHistogramCostExtractor }
	
	boxed_cast_base! { HistogramCostExtractor, core::Algorithm, cv_HistogramCostExtractor_to_Algorithm }
	
	impl std::fmt::Debug for HistogramCostExtractor {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("HistogramCostExtractor")
				.finish()
		}
	}
	
	/// Constant methods for [crate::shape::NormHistogramCostExtractor]
	pub trait NormHistogramCostExtractorTraitConst: crate::shape::HistogramCostExtractorTraitConst {
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
	
	/// Mutable methods for [crate::shape::NormHistogramCostExtractor]
	pub trait NormHistogramCostExtractorTrait: crate::shape::HistogramCostExtractorTrait + crate::shape::NormHistogramCostExtractorTraitConst {
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
	
	/// A norm based cost extraction. :
	pub struct NormHistogramCostExtractor {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { NormHistogramCostExtractor }
	
	impl Drop for NormHistogramCostExtractor {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_NormHistogramCostExtractor_delete(self.as_raw_mut_NormHistogramCostExtractor()) };
		}
	}
	
	unsafe impl Send for NormHistogramCostExtractor {}
	
	impl core::AlgorithmTraitConst for NormHistogramCostExtractor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for NormHistogramCostExtractor {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::HistogramCostExtractorTraitConst for NormHistogramCostExtractor {
		#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::shape::HistogramCostExtractorTrait for NormHistogramCostExtractor {
		#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::NormHistogramCostExtractorTraitConst for NormHistogramCostExtractor {
		#[inline] fn as_raw_NormHistogramCostExtractor(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::shape::NormHistogramCostExtractorTrait for NormHistogramCostExtractor {
		#[inline] fn as_raw_mut_NormHistogramCostExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl NormHistogramCostExtractor {
	}
	
	boxed_cast_base! { NormHistogramCostExtractor, core::Algorithm, cv_NormHistogramCostExtractor_to_Algorithm }
	
	boxed_cast_base! { NormHistogramCostExtractor, crate::shape::HistogramCostExtractor, cv_NormHistogramCostExtractor_to_HistogramCostExtractor }
	
	impl std::fmt::Debug for NormHistogramCostExtractor {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("NormHistogramCostExtractor")
				.finish()
		}
	}
	
	/// Constant methods for [crate::shape::ShapeContextDistanceExtractor]
	pub trait ShapeContextDistanceExtractorTraitConst: crate::shape::ShapeDistanceExtractorTraitConst {
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
		fn get_images(&self, image1: &mut impl core::ToOutputArray, image2: &mut impl core::ToOutputArray) -> Result<()> {
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
		fn get_cost_extractor(&self) -> Result<core::Ptr<crate::shape::HistogramCostExtractor>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ShapeContextDistanceExtractor_getCostExtractor_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::shape::HistogramCostExtractor>::opencv_from_extern(ret) };
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
		fn get_transform_algorithm(&self) -> Result<core::Ptr<crate::shape::ShapeTransformer>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ShapeContextDistanceExtractor_getTransformAlgorithm_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::shape::ShapeTransformer>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::shape::ShapeContextDistanceExtractor]
	pub trait ShapeContextDistanceExtractorTrait: crate::shape::ShapeContextDistanceExtractorTraitConst + crate::shape::ShapeDistanceExtractorTrait {
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
		fn set_images(&mut self, image1: &impl core::ToInputArray, image2: &impl core::ToInputArray) -> Result<()> {
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
		fn set_cost_extractor(&mut self, mut comparer: core::Ptr<crate::shape::HistogramCostExtractor>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ShapeContextDistanceExtractor_setCostExtractor_PtrLHistogramCostExtractorG(self.as_raw_mut_ShapeContextDistanceExtractor(), comparer.as_raw_mut_PtrOfHistogramCostExtractor(), ocvrs_return.as_mut_ptr()) };
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
		fn set_transform_algorithm(&mut self, mut transformer: core::Ptr<crate::shape::ShapeTransformer>) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ShapeContextDistanceExtractor_setTransformAlgorithm_PtrLShapeTransformerG(self.as_raw_mut_ShapeContextDistanceExtractor(), transformer.as_raw_mut_PtrOfShapeTransformer(), ocvrs_return.as_mut_ptr()) };
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
	pub struct ShapeContextDistanceExtractor {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ShapeContextDistanceExtractor }
	
	impl Drop for ShapeContextDistanceExtractor {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_ShapeContextDistanceExtractor_delete(self.as_raw_mut_ShapeContextDistanceExtractor()) };
		}
	}
	
	unsafe impl Send for ShapeContextDistanceExtractor {}
	
	impl core::AlgorithmTraitConst for ShapeContextDistanceExtractor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ShapeContextDistanceExtractor {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::ShapeDistanceExtractorTraitConst for ShapeContextDistanceExtractor {
		#[inline] fn as_raw_ShapeDistanceExtractor(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::shape::ShapeDistanceExtractorTrait for ShapeContextDistanceExtractor {
		#[inline] fn as_raw_mut_ShapeDistanceExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::ShapeContextDistanceExtractorTraitConst for ShapeContextDistanceExtractor {
		#[inline] fn as_raw_ShapeContextDistanceExtractor(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::shape::ShapeContextDistanceExtractorTrait for ShapeContextDistanceExtractor {
		#[inline] fn as_raw_mut_ShapeContextDistanceExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ShapeContextDistanceExtractor {
	}
	
	boxed_cast_base! { ShapeContextDistanceExtractor, core::Algorithm, cv_ShapeContextDistanceExtractor_to_Algorithm }
	
	boxed_cast_base! { ShapeContextDistanceExtractor, crate::shape::ShapeDistanceExtractor, cv_ShapeContextDistanceExtractor_to_ShapeDistanceExtractor }
	
	impl std::fmt::Debug for ShapeContextDistanceExtractor {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ShapeContextDistanceExtractor")
				.finish()
		}
	}
	
	/// Constant methods for [crate::shape::ShapeDistanceExtractor]
	pub trait ShapeDistanceExtractorTraitConst: core::AlgorithmTraitConst {
		fn as_raw_ShapeDistanceExtractor(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::shape::ShapeDistanceExtractor]
	pub trait ShapeDistanceExtractorTrait: core::AlgorithmTrait + crate::shape::ShapeDistanceExtractorTraitConst {
		fn as_raw_mut_ShapeDistanceExtractor(&mut self) -> *mut c_void;
	
		/// Compute the shape distance between two shapes defined by its contours.
		/// 
		/// ## Parameters
		/// * contour1: Contour defining first shape.
		/// * contour2: Contour defining second shape.
		#[inline]
		fn compute_distance(&mut self, contour1: &impl core::ToInputArray, contour2: &impl core::ToInputArray) -> Result<f32> {
			input_array_arg!(contour1);
			input_array_arg!(contour2);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ShapeDistanceExtractor_computeDistance_const__InputArrayR_const__InputArrayR(self.as_raw_mut_ShapeDistanceExtractor(), contour1.as_raw__InputArray(), contour2.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// @example modules/shape/samples/shape_example.cpp
	/// An example using shape distance algorithm
	/// 
	/// Abstract base class for shape distance algorithms.
	pub struct ShapeDistanceExtractor {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ShapeDistanceExtractor }
	
	impl Drop for ShapeDistanceExtractor {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_ShapeDistanceExtractor_delete(self.as_raw_mut_ShapeDistanceExtractor()) };
		}
	}
	
	unsafe impl Send for ShapeDistanceExtractor {}
	
	impl core::AlgorithmTraitConst for ShapeDistanceExtractor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ShapeDistanceExtractor {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::ShapeDistanceExtractorTraitConst for ShapeDistanceExtractor {
		#[inline] fn as_raw_ShapeDistanceExtractor(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::shape::ShapeDistanceExtractorTrait for ShapeDistanceExtractor {
		#[inline] fn as_raw_mut_ShapeDistanceExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ShapeDistanceExtractor {
	}
	
	boxed_cast_descendant! { ShapeDistanceExtractor, crate::shape::HausdorffDistanceExtractor, cv_ShapeDistanceExtractor_to_HausdorffDistanceExtractor }
	
	boxed_cast_descendant! { ShapeDistanceExtractor, crate::shape::ShapeContextDistanceExtractor, cv_ShapeDistanceExtractor_to_ShapeContextDistanceExtractor }
	
	boxed_cast_base! { ShapeDistanceExtractor, core::Algorithm, cv_ShapeDistanceExtractor_to_Algorithm }
	
	impl std::fmt::Debug for ShapeDistanceExtractor {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ShapeDistanceExtractor")
				.finish()
		}
	}
	
	/// Constant methods for [crate::shape::ShapeTransformer]
	pub trait ShapeTransformerTraitConst: core::AlgorithmTraitConst {
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
		fn warp_image(&self, transforming_image: &impl core::ToInputArray, output: &mut impl core::ToOutputArray, flags: i32, border_mode: i32, border_value: core::Scalar) -> Result<()> {
			input_array_arg!(transforming_image);
			output_array_arg!(output);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ShapeTransformer_warpImage_const_const__InputArrayR_const__OutputArrayR_int_int_const_ScalarR(self.as_raw_ShapeTransformer(), transforming_image.as_raw__InputArray(), output.as_raw__OutputArray(), flags, border_mode, &border_value, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
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
		/// ## Note
		/// This alternative version of [warp_image] function uses the following default values for its arguments:
		/// * flags: INTER_LINEAR
		/// * border_mode: BORDER_CONSTANT
		/// * border_value: Scalar()
		#[inline]
		fn warp_image_def(&self, transforming_image: &impl core::ToInputArray, output: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(transforming_image);
			output_array_arg!(output);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ShapeTransformer_warpImage_const_const__InputArrayR_const__OutputArrayR(self.as_raw_ShapeTransformer(), transforming_image.as_raw__InputArray(), output.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Mutable methods for [crate::shape::ShapeTransformer]
	pub trait ShapeTransformerTrait: core::AlgorithmTrait + crate::shape::ShapeTransformerTraitConst {
		fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void;
	
		/// Estimate the transformation parameters of the current transformer algorithm, based on point matches.
		/// 
		/// ## Parameters
		/// * transformingShape: Contour defining first shape.
		/// * targetShape: Contour defining second shape (Target).
		/// * matches: Standard vector of Matches between points.
		#[inline]
		fn estimate_transformation(&mut self, transforming_shape: &impl core::ToInputArray, target_shape: &impl core::ToInputArray, matches: &mut core::Vector<core::DMatch>) -> Result<()> {
			input_array_arg!(transforming_shape);
			input_array_arg!(target_shape);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ShapeTransformer_estimateTransformation_const__InputArrayR_const__InputArrayR_vectorLDMatchGR(self.as_raw_mut_ShapeTransformer(), transforming_shape.as_raw__InputArray(), target_shape.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch(), ocvrs_return.as_mut_ptr()) };
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
		fn apply_transformation(&mut self, input: &impl core::ToInputArray, output: &mut impl core::ToOutputArray) -> Result<f32> {
			input_array_arg!(input);
			output_array_arg!(output);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ShapeTransformer_applyTransformation_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_ShapeTransformer(), input.as_raw__InputArray(), output.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
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
		/// ## Note
		/// This alternative version of [apply_transformation] function uses the following default values for its arguments:
		/// * output: noArray()
		#[inline]
		fn apply_transformation_def(&mut self, input: &impl core::ToInputArray) -> Result<f32> {
			input_array_arg!(input);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_ShapeTransformer_applyTransformation_const__InputArrayR(self.as_raw_mut_ShapeTransformer(), input.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	/// Abstract base class for shape transformation algorithms.
	pub struct ShapeTransformer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ShapeTransformer }
	
	impl Drop for ShapeTransformer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_ShapeTransformer_delete(self.as_raw_mut_ShapeTransformer()) };
		}
	}
	
	unsafe impl Send for ShapeTransformer {}
	
	impl core::AlgorithmTraitConst for ShapeTransformer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ShapeTransformer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::ShapeTransformerTraitConst for ShapeTransformer {
		#[inline] fn as_raw_ShapeTransformer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::shape::ShapeTransformerTrait for ShapeTransformer {
		#[inline] fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ShapeTransformer {
	}
	
	boxed_cast_descendant! { ShapeTransformer, crate::shape::AffineTransformer, cv_ShapeTransformer_to_AffineTransformer }
	
	boxed_cast_descendant! { ShapeTransformer, crate::shape::ThinPlateSplineShapeTransformer, cv_ShapeTransformer_to_ThinPlateSplineShapeTransformer }
	
	boxed_cast_base! { ShapeTransformer, core::Algorithm, cv_ShapeTransformer_to_Algorithm }
	
	impl std::fmt::Debug for ShapeTransformer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ShapeTransformer")
				.finish()
		}
	}
	
	/// Constant methods for [crate::shape::ThinPlateSplineShapeTransformer]
	pub trait ThinPlateSplineShapeTransformerTraitConst: crate::shape::ShapeTransformerTraitConst {
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
	
	/// Mutable methods for [crate::shape::ThinPlateSplineShapeTransformer]
	pub trait ThinPlateSplineShapeTransformerTrait: crate::shape::ShapeTransformerTrait + crate::shape::ThinPlateSplineShapeTransformerTraitConst {
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
	
	/// Definition of the transformation
	/// 
	/// occupied in the paper "Principal Warps: Thin-Plate Splines and Decomposition of Deformations", by
	/// F.L. Bookstein (PAMI 1989). :
	pub struct ThinPlateSplineShapeTransformer {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { ThinPlateSplineShapeTransformer }
	
	impl Drop for ThinPlateSplineShapeTransformer {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_ThinPlateSplineShapeTransformer_delete(self.as_raw_mut_ThinPlateSplineShapeTransformer()) };
		}
	}
	
	unsafe impl Send for ThinPlateSplineShapeTransformer {}
	
	impl core::AlgorithmTraitConst for ThinPlateSplineShapeTransformer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for ThinPlateSplineShapeTransformer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::ShapeTransformerTraitConst for ThinPlateSplineShapeTransformer {
		#[inline] fn as_raw_ShapeTransformer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::shape::ShapeTransformerTrait for ThinPlateSplineShapeTransformer {
		#[inline] fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::ThinPlateSplineShapeTransformerTraitConst for ThinPlateSplineShapeTransformer {
		#[inline] fn as_raw_ThinPlateSplineShapeTransformer(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::shape::ThinPlateSplineShapeTransformerTrait for ThinPlateSplineShapeTransformer {
		#[inline] fn as_raw_mut_ThinPlateSplineShapeTransformer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl ThinPlateSplineShapeTransformer {
	}
	
	boxed_cast_base! { ThinPlateSplineShapeTransformer, core::Algorithm, cv_ThinPlateSplineShapeTransformer_to_Algorithm }
	
	boxed_cast_base! { ThinPlateSplineShapeTransformer, crate::shape::ShapeTransformer, cv_ThinPlateSplineShapeTransformer_to_ShapeTransformer }
	
	impl std::fmt::Debug for ThinPlateSplineShapeTransformer {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("ThinPlateSplineShapeTransformer")
				.finish()
		}
	}
}
