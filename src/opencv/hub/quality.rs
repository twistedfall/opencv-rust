#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::QualityBaseConst, super::QualityBase, super::QualityMSETraitConst, super::QualityMSETrait, super::QualityPSNRTraitConst, super::QualityPSNRTrait, super::QualitySSIMTraitConst, super::QualitySSIMTrait, super::QualityGMSDTraitConst, super::QualityGMSDTrait, super::QualityBRISQUETraitConst, super::QualityBRISQUETrait };
}

/// BRISQUE (Blind/Referenceless Image Spatial Quality Evaluator) is a No Reference Image Quality Assessment (NR-IQA) algorithm.
/// 
/// BRISQUE computes a score based on extracting Natural Scene Statistics (https://en.wikipedia.org/wiki/Scene_statistics)
/// and calculating feature vectors. See Mittal et al. [Mittal2](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Mittal2) for original paper and original implementation [Mittal2_software](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Mittal2_software) .
/// 
/// A trained model is provided in the /samples/ directory and is trained on the LIVE-R2 database [Sheikh](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Sheikh) as in the original implementation.
/// When evaluated against the TID2008 database [Ponomarenko](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Ponomarenko) , the SROCC is -0.8424 versus the SROCC of -0.8354 in the original implementation.
/// C++ code for the BRISQUE LIVE-R2 trainer and TID2008 evaluator are also provided in the /samples/ directory.
pub trait QualityBRISQUETraitConst: crate::quality::QualityBaseConst {
	fn as_raw_QualityBRISQUE(&self) -> *const c_void;

}

pub trait QualityBRISQUETrait: crate::quality::QualityBRISQUETraitConst + crate::quality::QualityBase {
	fn as_raw_mut_QualityBRISQUE(&mut self) -> *mut c_void;

	/// Computes BRISQUE quality score for input image
	/// ## Parameters
	/// * img: Image for which to compute quality
	/// ## Returns
	/// cv::Scalar with the score in the first element.  The score ranges from 0 (best quality) to 100 (worst quality)
	#[inline]
	fn compute(&mut self, img: &dyn core::ToInputArray) -> Result<core::Scalar> {
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityBRISQUE_compute_const__InputArrayR(self.as_raw_mut_QualityBRISQUE(), img.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// BRISQUE (Blind/Referenceless Image Spatial Quality Evaluator) is a No Reference Image Quality Assessment (NR-IQA) algorithm.
/// 
/// BRISQUE computes a score based on extracting Natural Scene Statistics (https://en.wikipedia.org/wiki/Scene_statistics)
/// and calculating feature vectors. See Mittal et al. [Mittal2](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Mittal2) for original paper and original implementation [Mittal2_software](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Mittal2_software) .
/// 
/// A trained model is provided in the /samples/ directory and is trained on the LIVE-R2 database [Sheikh](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Sheikh) as in the original implementation.
/// When evaluated against the TID2008 database [Ponomarenko](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Ponomarenko) , the SROCC is -0.8424 versus the SROCC of -0.8354 in the original implementation.
/// C++ code for the BRISQUE LIVE-R2 trainer and TID2008 evaluator are also provided in the /samples/ directory.
pub struct QualityBRISQUE {
	ptr: *mut c_void
}

opencv_type_boxed! { QualityBRISQUE }

impl Drop for QualityBRISQUE {
	fn drop(&mut self) {
		extern "C" { fn cv_QualityBRISQUE_delete(instance: *mut c_void); }
		unsafe { cv_QualityBRISQUE_delete(self.as_raw_mut_QualityBRISQUE()) };
	}
}

unsafe impl Send for QualityBRISQUE {}

impl core::AlgorithmTraitConst for QualityBRISQUE {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for QualityBRISQUE {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::quality::QualityBaseConst for QualityBRISQUE {
	#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::quality::QualityBase for QualityBRISQUE {
	#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::quality::QualityBRISQUETraitConst for QualityBRISQUE {
	#[inline] fn as_raw_QualityBRISQUE(&self) -> *const c_void { self.as_raw() }
}

impl crate::quality::QualityBRISQUETrait for QualityBRISQUE {
	#[inline] fn as_raw_mut_QualityBRISQUE(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl QualityBRISQUE {
	/// Create an object which calculates quality
	/// ## Parameters
	/// * model_file_path: cv::String which contains a path to the BRISQUE model data, eg. /path/to/brisque_model_live.yml
	/// * range_file_path: cv::String which contains a path to the BRISQUE range data, eg. /path/to/brisque_range_live.yml
	#[inline]
	pub fn create(model_file_path: &str, range_file_path: &str) -> Result<core::Ptr<crate::quality::QualityBRISQUE>> {
		extern_container_arg!(model_file_path);
		extern_container_arg!(range_file_path);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityBRISQUE_create_const_StringR_const_StringR(model_file_path.opencv_as_extern(), range_file_path.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::quality::QualityBRISQUE>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Create an object which calculates quality
	/// ## Parameters
	/// * model: cv::Ptr<cv::ml::SVM> which contains a loaded BRISQUE model
	/// * range: cv::Mat which contains BRISQUE range data
	#[inline]
	pub fn create_1(model: &core::Ptr<dyn crate::ml::SVM>, range: &core::Mat) -> Result<core::Ptr<crate::quality::QualityBRISQUE>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityBRISQUE_create_const_Ptr_SVM_R_const_MatR(model.as_raw_PtrOfSVM(), range.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::quality::QualityBRISQUE>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// static method for computing quality
	/// ## Parameters
	/// * img: image for which to compute quality
	/// * model_file_path: cv::String which contains a path to the BRISQUE model data, eg. /path/to/brisque_model_live.yml
	/// * range_file_path: cv::String which contains a path to the BRISQUE range data, eg. /path/to/brisque_range_live.yml
	/// ## Returns
	/// cv::Scalar with the score in the first element.  The score ranges from 0 (best quality) to 100 (worst quality)
	#[inline]
	pub fn compute(img: &dyn core::ToInputArray, model_file_path: &str, range_file_path: &str) -> Result<core::Scalar> {
		input_array_arg!(img);
		extern_container_arg!(model_file_path);
		extern_container_arg!(range_file_path);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityBRISQUE_compute_const__InputArrayR_const_StringR_const_StringR(img.as_raw__InputArray(), model_file_path.opencv_as_extern(), range_file_path.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// static method for computing image features used by the BRISQUE algorithm
	/// ## Parameters
	/// * img: image (BGR(A) or grayscale) for which to compute features
	/// * features: output row vector of features to cv::Mat or cv::UMat
	#[inline]
	pub fn compute_features(img: &dyn core::ToInputArray, features: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(img);
		output_array_arg!(features);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityBRISQUE_computeFeatures_const__InputArrayR_const__OutputArrayR(img.as_raw__InputArray(), features.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

boxed_cast_base! { QualityBRISQUE, core::Algorithm, cv_QualityBRISQUE_to_Algorithm }

/// ********************************* Quality Base Class ***********************************
pub trait QualityBaseConst: core::AlgorithmTraitConst {
	fn as_raw_QualityBase(&self) -> *const c_void;

	/// Returns output quality map that was generated during computation, if supported by the algorithm
	#[inline]
	fn get_quality_map(&self, dst: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityBase_getQualityMap_const_const__OutputArrayR(self.as_raw_QualityBase(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Implements Algorithm::empty()
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityBase_empty_const(self.as_raw_QualityBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait QualityBase: core::AlgorithmTrait + crate::quality::QualityBaseConst {
	fn as_raw_mut_QualityBase(&mut self) -> *mut c_void;

	/// Compute quality score per channel with the per-channel score in each element of the resulting cv::Scalar.  See specific algorithm for interpreting result scores
	/// ## Parameters
	/// * img: comparison image, or image to evalute for no-reference quality algorithms
	#[inline]
	fn compute(&mut self, img: &dyn core::ToInputArray) -> Result<core::Scalar> {
		input_array_arg!(img);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityBase_compute_const__InputArrayR(self.as_raw_mut_QualityBase(), img.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Implements Algorithm::clear()
	#[inline]
	fn clear(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityBase_clear(self.as_raw_mut_QualityBase(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Full reference GMSD algorithm
/// http://www4.comp.polyu.edu.hk/~cslzhang/IQA/GMSD/GMSD.htm
pub trait QualityGMSDTraitConst: crate::quality::QualityBaseConst {
	fn as_raw_QualityGMSD(&self) -> *const c_void;

	/// Implements Algorithm::empty()
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityGMSD_empty_const(self.as_raw_QualityGMSD(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait QualityGMSDTrait: crate::quality::QualityBase + crate::quality::QualityGMSDTraitConst {
	fn as_raw_mut_QualityGMSD(&mut self) -> *mut c_void;

	/// Compute GMSD
	/// ## Parameters
	/// * cmp: comparison image
	/// ## Returns
	/// cv::Scalar with per-channel quality value.  Values range from 0 (worst) to 1 (best)
	#[inline]
	fn compute(&mut self, cmp: &dyn core::ToInputArray) -> Result<core::Scalar> {
		input_array_arg!(cmp);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityGMSD_compute_const__InputArrayR(self.as_raw_mut_QualityGMSD(), cmp.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Implements Algorithm::clear()
	#[inline]
	fn clear(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityGMSD_clear(self.as_raw_mut_QualityGMSD(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Full reference GMSD algorithm
/// http://www4.comp.polyu.edu.hk/~cslzhang/IQA/GMSD/GMSD.htm
pub struct QualityGMSD {
	ptr: *mut c_void
}

opencv_type_boxed! { QualityGMSD }

impl Drop for QualityGMSD {
	fn drop(&mut self) {
		extern "C" { fn cv_QualityGMSD_delete(instance: *mut c_void); }
		unsafe { cv_QualityGMSD_delete(self.as_raw_mut_QualityGMSD()) };
	}
}

unsafe impl Send for QualityGMSD {}

impl core::AlgorithmTraitConst for QualityGMSD {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for QualityGMSD {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::quality::QualityBaseConst for QualityGMSD {
	#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::quality::QualityBase for QualityGMSD {
	#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::quality::QualityGMSDTraitConst for QualityGMSD {
	#[inline] fn as_raw_QualityGMSD(&self) -> *const c_void { self.as_raw() }
}

impl crate::quality::QualityGMSDTrait for QualityGMSD {
	#[inline] fn as_raw_mut_QualityGMSD(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl QualityGMSD {
	/// Create an object which calculates image quality
	/// ## Parameters
	/// * ref: reference image
	#[inline]
	pub fn create(ref_: &dyn core::ToInputArray) -> Result<core::Ptr<crate::quality::QualityGMSD>> {
		input_array_arg!(ref_);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityGMSD_create_const__InputArrayR(ref_.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::quality::QualityGMSD>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// static method for computing quality
	/// ## Parameters
	/// * ref: reference image
	/// * cmp: comparison image
	/// * qualityMap: output quality map, or cv::noArray()
	/// ## Returns
	/// cv::Scalar with per-channel quality value.  Values range from 0 (worst) to 1 (best)
	#[inline]
	pub fn compute(ref_: &dyn core::ToInputArray, cmp: &dyn core::ToInputArray, quality_map: &mut dyn core::ToOutputArray) -> Result<core::Scalar> {
		input_array_arg!(ref_);
		input_array_arg!(cmp);
		output_array_arg!(quality_map);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityGMSD_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(ref_.as_raw__InputArray(), cmp.as_raw__InputArray(), quality_map.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

boxed_cast_base! { QualityGMSD, core::Algorithm, cv_QualityGMSD_to_Algorithm }

/// Full reference mean square error algorithm  https://en.wikipedia.org/wiki/Mean_squared_error
pub trait QualityMSETraitConst: crate::quality::QualityBaseConst {
	fn as_raw_QualityMSE(&self) -> *const c_void;

	/// Implements Algorithm::empty()
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityMSE_empty_const(self.as_raw_QualityMSE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait QualityMSETrait: crate::quality::QualityBase + crate::quality::QualityMSETraitConst {
	fn as_raw_mut_QualityMSE(&mut self) -> *mut c_void;

	/// Computes MSE for reference images supplied in class constructor and provided comparison images
	/// ## Parameters
	/// * cmpImgs: Comparison image(s)
	/// ## Returns
	/// cv::Scalar with per-channel quality values.  Values range from 0 (best) to potentially max float (worst)
	#[inline]
	fn compute(&mut self, cmp_imgs: &dyn core::ToInputArray) -> Result<core::Scalar> {
		input_array_arg!(cmp_imgs);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityMSE_compute_const__InputArrayR(self.as_raw_mut_QualityMSE(), cmp_imgs.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Implements Algorithm::clear()
	#[inline]
	fn clear(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityMSE_clear(self.as_raw_mut_QualityMSE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Full reference mean square error algorithm  https://en.wikipedia.org/wiki/Mean_squared_error
pub struct QualityMSE {
	ptr: *mut c_void
}

opencv_type_boxed! { QualityMSE }

impl Drop for QualityMSE {
	fn drop(&mut self) {
		extern "C" { fn cv_QualityMSE_delete(instance: *mut c_void); }
		unsafe { cv_QualityMSE_delete(self.as_raw_mut_QualityMSE()) };
	}
}

unsafe impl Send for QualityMSE {}

impl core::AlgorithmTraitConst for QualityMSE {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for QualityMSE {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::quality::QualityBaseConst for QualityMSE {
	#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::quality::QualityBase for QualityMSE {
	#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::quality::QualityMSETraitConst for QualityMSE {
	#[inline] fn as_raw_QualityMSE(&self) -> *const c_void { self.as_raw() }
}

impl crate::quality::QualityMSETrait for QualityMSE {
	#[inline] fn as_raw_mut_QualityMSE(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl QualityMSE {
	/// Create an object which calculates quality
	/// ## Parameters
	/// * ref: input image to use as the reference for comparison
	#[inline]
	pub fn create(ref_: &dyn core::ToInputArray) -> Result<core::Ptr<crate::quality::QualityMSE>> {
		input_array_arg!(ref_);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityMSE_create_const__InputArrayR(ref_.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::quality::QualityMSE>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// static method for computing quality
	/// ## Parameters
	/// * ref: reference image
	/// * cmp: comparison image=
	/// * qualityMap: output quality map, or cv::noArray()
	/// ## Returns
	/// cv::Scalar with per-channel quality values.  Values range from 0 (best) to max float (worst)
	#[inline]
	pub fn compute(ref_: &dyn core::ToInputArray, cmp: &dyn core::ToInputArray, quality_map: &mut dyn core::ToOutputArray) -> Result<core::Scalar> {
		input_array_arg!(ref_);
		input_array_arg!(cmp);
		output_array_arg!(quality_map);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityMSE_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(ref_.as_raw__InputArray(), cmp.as_raw__InputArray(), quality_map.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

boxed_cast_base! { QualityMSE, core::Algorithm, cv_QualityMSE_to_Algorithm }

/// Full reference peak signal to noise ratio (PSNR) algorithm  https://en.wikipedia.org/wiki/Peak_signal-to-noise_ratio
pub trait QualityPSNRTraitConst: crate::quality::QualityBaseConst {
	fn as_raw_QualityPSNR(&self) -> *const c_void;

	/// Implements Algorithm::empty()
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityPSNR_empty_const(self.as_raw_QualityPSNR(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// return the maximum pixel value used for PSNR computation
	#[inline]
	fn get_max_pixel_value(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityPSNR_getMaxPixelValue_const(self.as_raw_QualityPSNR(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait QualityPSNRTrait: crate::quality::QualityBase + crate::quality::QualityPSNRTraitConst {
	fn as_raw_mut_QualityPSNR(&mut self) -> *mut c_void;

	/// Compute the PSNR
	/// ## Parameters
	/// * cmp: Comparison image
	/// ## Returns
	/// Per-channel PSNR value, or std::numeric_limits<double>::infinity() if the MSE between the two images == 0
	#[inline]
	fn compute(&mut self, cmp: &dyn core::ToInputArray) -> Result<core::Scalar> {
		input_array_arg!(cmp);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityPSNR_compute_const__InputArrayR(self.as_raw_mut_QualityPSNR(), cmp.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Implements Algorithm::clear()
	#[inline]
	fn clear(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityPSNR_clear(self.as_raw_mut_QualityPSNR(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// sets the maximum pixel value used for PSNR computation
	/// ## Parameters
	/// * val: Maximum pixel value
	#[inline]
	fn set_max_pixel_value(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityPSNR_setMaxPixelValue_double(self.as_raw_mut_QualityPSNR(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Full reference peak signal to noise ratio (PSNR) algorithm  https://en.wikipedia.org/wiki/Peak_signal-to-noise_ratio
pub struct QualityPSNR {
	ptr: *mut c_void
}

opencv_type_boxed! { QualityPSNR }

impl Drop for QualityPSNR {
	fn drop(&mut self) {
		extern "C" { fn cv_QualityPSNR_delete(instance: *mut c_void); }
		unsafe { cv_QualityPSNR_delete(self.as_raw_mut_QualityPSNR()) };
	}
}

unsafe impl Send for QualityPSNR {}

impl core::AlgorithmTraitConst for QualityPSNR {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for QualityPSNR {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::quality::QualityBaseConst for QualityPSNR {
	#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::quality::QualityBase for QualityPSNR {
	#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::quality::QualityPSNRTraitConst for QualityPSNR {
	#[inline] fn as_raw_QualityPSNR(&self) -> *const c_void { self.as_raw() }
}

impl crate::quality::QualityPSNRTrait for QualityPSNR {
	#[inline] fn as_raw_mut_QualityPSNR(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl QualityPSNR {
	pub const MAX_PIXEL_VALUE_DEFAULT: f64 = 255.;
	/// Create an object which calculates quality
	/// ## Parameters
	/// * ref: input image to use as the source for comparison
	/// * maxPixelValue: maximum per-channel value for any individual pixel; eg 255 for uint8 image
	/// 
	/// ## C++ default parameters
	/// * max_pixel_value: QualityPSNR::MAX_PIXEL_VALUE_DEFAULT
	#[inline]
	pub fn create(ref_: &dyn core::ToInputArray, max_pixel_value: f64) -> Result<core::Ptr<crate::quality::QualityPSNR>> {
		input_array_arg!(ref_);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityPSNR_create_const__InputArrayR_double(ref_.as_raw__InputArray(), max_pixel_value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::quality::QualityPSNR>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// static method for computing quality
	/// ## Parameters
	/// * ref: reference image
	/// * cmp: comparison image
	/// * qualityMap: output quality map, or cv::noArray()
	/// * maxPixelValue: maximum per-channel value for any individual pixel; eg 255 for uint8 image
	/// ## Returns
	/// PSNR value, or std::numeric_limits<double>::infinity() if the MSE between the two images == 0
	/// 
	/// ## C++ default parameters
	/// * max_pixel_value: QualityPSNR::MAX_PIXEL_VALUE_DEFAULT
	#[inline]
	pub fn compute(ref_: &dyn core::ToInputArray, cmp: &dyn core::ToInputArray, quality_map: &mut dyn core::ToOutputArray, max_pixel_value: f64) -> Result<core::Scalar> {
		input_array_arg!(ref_);
		input_array_arg!(cmp);
		output_array_arg!(quality_map);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualityPSNR_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double(ref_.as_raw__InputArray(), cmp.as_raw__InputArray(), quality_map.as_raw__OutputArray(), max_pixel_value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

boxed_cast_base! { QualityPSNR, core::Algorithm, cv_QualityPSNR_to_Algorithm }

/// Full reference structural similarity algorithm  https://en.wikipedia.org/wiki/Structural_similarity
pub trait QualitySSIMTraitConst: crate::quality::QualityBaseConst {
	fn as_raw_QualitySSIM(&self) -> *const c_void;

	/// Implements Algorithm::empty()
	#[inline]
	fn empty(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualitySSIM_empty_const(self.as_raw_QualitySSIM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait QualitySSIMTrait: crate::quality::QualityBase + crate::quality::QualitySSIMTraitConst {
	fn as_raw_mut_QualitySSIM(&mut self) -> *mut c_void;

	/// Computes SSIM
	/// ## Parameters
	/// * cmp: Comparison image
	/// ## Returns
	/// cv::Scalar with per-channel quality values.  Values range from 0 (worst) to 1 (best)
	#[inline]
	fn compute(&mut self, cmp: &dyn core::ToInputArray) -> Result<core::Scalar> {
		input_array_arg!(cmp);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualitySSIM_compute_const__InputArrayR(self.as_raw_mut_QualitySSIM(), cmp.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Implements Algorithm::clear()
	#[inline]
	fn clear(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualitySSIM_clear(self.as_raw_mut_QualitySSIM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Full reference structural similarity algorithm  https://en.wikipedia.org/wiki/Structural_similarity
pub struct QualitySSIM {
	ptr: *mut c_void
}

opencv_type_boxed! { QualitySSIM }

impl Drop for QualitySSIM {
	fn drop(&mut self) {
		extern "C" { fn cv_QualitySSIM_delete(instance: *mut c_void); }
		unsafe { cv_QualitySSIM_delete(self.as_raw_mut_QualitySSIM()) };
	}
}

unsafe impl Send for QualitySSIM {}

impl core::AlgorithmTraitConst for QualitySSIM {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for QualitySSIM {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::quality::QualityBaseConst for QualitySSIM {
	#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.as_raw() }
}

impl crate::quality::QualityBase for QualitySSIM {
	#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::quality::QualitySSIMTraitConst for QualitySSIM {
	#[inline] fn as_raw_QualitySSIM(&self) -> *const c_void { self.as_raw() }
}

impl crate::quality::QualitySSIMTrait for QualitySSIM {
	#[inline] fn as_raw_mut_QualitySSIM(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl QualitySSIM {
	/// Create an object which calculates quality
	/// ## Parameters
	/// * ref: input image to use as the reference image for comparison
	#[inline]
	pub fn create(ref_: &dyn core::ToInputArray) -> Result<core::Ptr<crate::quality::QualitySSIM>> {
		input_array_arg!(ref_);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualitySSIM_create_const__InputArrayR(ref_.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::quality::QualitySSIM>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// static method for computing quality
	/// ## Parameters
	/// * ref: reference image
	/// * cmp: comparison image
	/// * qualityMap: output quality map, or cv::noArray()
	/// ## Returns
	/// cv::Scalar with per-channel quality values.  Values range from 0 (worst) to 1 (best)
	#[inline]
	pub fn compute(ref_: &dyn core::ToInputArray, cmp: &dyn core::ToInputArray, quality_map: &mut dyn core::ToOutputArray) -> Result<core::Scalar> {
		input_array_arg!(ref_);
		input_array_arg!(cmp);
		output_array_arg!(quality_map);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_quality_QualitySSIM_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(ref_.as_raw__InputArray(), cmp.as_raw__InputArray(), quality_map.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

boxed_cast_base! { QualitySSIM, core::Algorithm, cv_QualitySSIM_to_Algorithm }
