pub mod quality {
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::QualityBaseTraitConst, super::QualityBaseTrait, super::QualityMSETraitConst, super::QualityMSETrait, super::QualityPSNRTraitConst, super::QualityPSNRTrait, super::QualitySSIMTraitConst, super::QualitySSIMTrait, super::QualityGMSDTraitConst, super::QualityGMSDTrait, super::QualityBRISQUETraitConst, super::QualityBRISQUETrait };
	}
	
	/// Constant methods for [crate::quality::QualityBRISQUE]
	pub trait QualityBRISQUETraitConst: crate::quality::QualityBaseTraitConst {
		fn as_raw_QualityBRISQUE(&self) -> *const c_void;
	
	}
	
	/// Mutable methods for [crate::quality::QualityBRISQUE]
	pub trait QualityBRISQUETrait: crate::quality::QualityBRISQUETraitConst + crate::quality::QualityBaseTrait {
		fn as_raw_mut_QualityBRISQUE(&mut self) -> *mut c_void;
	
		/// Computes BRISQUE quality score for input image
		/// ## Parameters
		/// * img: Image for which to compute quality
		/// ## Returns
		/// cv::Scalar with the score in the first element.  The score ranges from 0 (best quality) to 100 (worst quality)
		#[inline]
		fn compute(&mut self, img: &impl core::ToInputArray) -> Result<core::Scalar> {
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
	/// BRISQUE computes a score based on extracting Natural Scene Statistics (<https://en.wikipedia.org/wiki/Scene_statistics>)
	/// and calculating feature vectors. See Mittal et al. [Mittal2](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Mittal2) for original paper and original implementation [Mittal2_software](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Mittal2_software) .
	/// 
	/// A trained model is provided in the /samples/ directory and is trained on the LIVE-R2 database [Sheikh](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Sheikh) as in the original implementation.
	/// When evaluated against the TID2008 database [Ponomarenko](https://docs.opencv.org/4.8.1/d0/de3/citelist.html#CITEREF_Ponomarenko) , the SROCC is -0.8424 versus the SROCC of -0.8354 in the original implementation.
	/// C++ code for the BRISQUE LIVE-R2 trainer and TID2008 evaluator are also provided in the /samples/ directory.
	pub struct QualityBRISQUE {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { QualityBRISQUE }
	
	impl Drop for QualityBRISQUE {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_quality_QualityBRISQUE_delete(self.as_raw_mut_QualityBRISQUE()) };
		}
	}
	
	unsafe impl Send for QualityBRISQUE {}
	
	impl core::AlgorithmTraitConst for QualityBRISQUE {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for QualityBRISQUE {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::quality::QualityBaseTraitConst for QualityBRISQUE {
		#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::quality::QualityBaseTrait for QualityBRISQUE {
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
		pub fn create_1(model: &core::Ptr<crate::ml::SVM>, range: &core::Mat) -> Result<core::Ptr<crate::quality::QualityBRISQUE>> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_quality_QualityBRISQUE_create_const_PtrLSVMGR_const_MatR(model.as_raw_PtrOfSVM(), range.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
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
		pub fn compute(img: &impl core::ToInputArray, model_file_path: &str, range_file_path: &str) -> Result<core::Scalar> {
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
		pub fn compute_features(img: &impl core::ToInputArray, features: &mut impl core::ToOutputArray) -> Result<()> {
			input_array_arg!(img);
			output_array_arg!(features);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_quality_QualityBRISQUE_computeFeatures_const__InputArrayR_const__OutputArrayR(img.as_raw__InputArray(), features.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { QualityBRISQUE, core::Algorithm, cv_quality_QualityBRISQUE_to_Algorithm }
	
	boxed_cast_base! { QualityBRISQUE, crate::quality::QualityBase, cv_quality_QualityBRISQUE_to_QualityBase }
	
	impl std::fmt::Debug for QualityBRISQUE {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("QualityBRISQUE")
				.finish()
		}
	}
	
	/// Constant methods for [crate::quality::QualityBase]
	pub trait QualityBaseTraitConst: core::AlgorithmTraitConst {
		fn as_raw_QualityBase(&self) -> *const c_void;
	
		/// Returns output quality map that was generated during computation, if supported by the algorithm
		#[inline]
		fn get_quality_map(&self, dst: &mut impl core::ToOutputArray) -> Result<()> {
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
	
	/// Mutable methods for [crate::quality::QualityBase]
	pub trait QualityBaseTrait: core::AlgorithmTrait + crate::quality::QualityBaseTraitConst {
		fn as_raw_mut_QualityBase(&mut self) -> *mut c_void;
	
		/// Compute quality score per channel with the per-channel score in each element of the resulting cv::Scalar.  See specific algorithm for interpreting result scores
		/// ## Parameters
		/// * img: comparison image, or image to evalute for no-reference quality algorithms
		#[inline]
		fn compute(&mut self, img: &impl core::ToInputArray) -> Result<core::Scalar> {
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
	
	/// ********************************* Quality Base Class ***********************************
	pub struct QualityBase {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { QualityBase }
	
	impl Drop for QualityBase {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_quality_QualityBase_delete(self.as_raw_mut_QualityBase()) };
		}
	}
	
	unsafe impl Send for QualityBase {}
	
	impl core::AlgorithmTraitConst for QualityBase {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for QualityBase {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::quality::QualityBaseTraitConst for QualityBase {
		#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::quality::QualityBaseTrait for QualityBase {
		#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl QualityBase {
	}
	
	boxed_cast_descendant! { QualityBase, crate::quality::QualityBRISQUE, cv_quality_QualityBase_to_QualityBRISQUE }
	
	boxed_cast_descendant! { QualityBase, crate::quality::QualityGMSD, cv_quality_QualityBase_to_QualityGMSD }
	
	boxed_cast_descendant! { QualityBase, crate::quality::QualityMSE, cv_quality_QualityBase_to_QualityMSE }
	
	boxed_cast_descendant! { QualityBase, crate::quality::QualityPSNR, cv_quality_QualityBase_to_QualityPSNR }
	
	boxed_cast_descendant! { QualityBase, crate::quality::QualitySSIM, cv_quality_QualityBase_to_QualitySSIM }
	
	boxed_cast_base! { QualityBase, core::Algorithm, cv_quality_QualityBase_to_Algorithm }
	
	impl std::fmt::Debug for QualityBase {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("QualityBase")
				.finish()
		}
	}
	
	/// Constant methods for [crate::quality::QualityGMSD]
	pub trait QualityGMSDTraitConst: crate::quality::QualityBaseTraitConst {
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
	
	/// Mutable methods for [crate::quality::QualityGMSD]
	pub trait QualityGMSDTrait: crate::quality::QualityBaseTrait + crate::quality::QualityGMSDTraitConst {
		fn as_raw_mut_QualityGMSD(&mut self) -> *mut c_void;
	
		/// Compute GMSD
		/// ## Parameters
		/// * cmp: comparison image
		/// ## Returns
		/// cv::Scalar with per-channel quality value.  Values range from 0 (worst) to 1 (best)
		#[inline]
		fn compute(&mut self, cmp: &impl core::ToInputArray) -> Result<core::Scalar> {
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
	/// <http://www4.comp.polyu.edu.hk/~cslzhang/IQA/GMSD/GMSD.htm>
	pub struct QualityGMSD {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { QualityGMSD }
	
	impl Drop for QualityGMSD {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_quality_QualityGMSD_delete(self.as_raw_mut_QualityGMSD()) };
		}
	}
	
	unsafe impl Send for QualityGMSD {}
	
	impl core::AlgorithmTraitConst for QualityGMSD {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for QualityGMSD {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::quality::QualityBaseTraitConst for QualityGMSD {
		#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::quality::QualityBaseTrait for QualityGMSD {
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
		pub fn create(ref_: &impl core::ToInputArray) -> Result<core::Ptr<crate::quality::QualityGMSD>> {
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
		pub fn compute(ref_: &impl core::ToInputArray, cmp: &impl core::ToInputArray, quality_map: &mut impl core::ToOutputArray) -> Result<core::Scalar> {
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
	
	boxed_cast_base! { QualityGMSD, core::Algorithm, cv_quality_QualityGMSD_to_Algorithm }
	
	boxed_cast_base! { QualityGMSD, crate::quality::QualityBase, cv_quality_QualityGMSD_to_QualityBase }
	
	impl std::fmt::Debug for QualityGMSD {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("QualityGMSD")
				.finish()
		}
	}
	
	/// Constant methods for [crate::quality::QualityMSE]
	pub trait QualityMSETraitConst: crate::quality::QualityBaseTraitConst {
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
	
	/// Mutable methods for [crate::quality::QualityMSE]
	pub trait QualityMSETrait: crate::quality::QualityBaseTrait + crate::quality::QualityMSETraitConst {
		fn as_raw_mut_QualityMSE(&mut self) -> *mut c_void;
	
		/// Computes MSE for reference images supplied in class constructor and provided comparison images
		/// ## Parameters
		/// * cmpImgs: Comparison image(s)
		/// ## Returns
		/// cv::Scalar with per-channel quality values.  Values range from 0 (best) to potentially max float (worst)
		#[inline]
		fn compute(&mut self, cmp_imgs: &impl core::ToInputArray) -> Result<core::Scalar> {
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
	
	/// Full reference mean square error algorithm  <https://en.wikipedia.org/wiki/Mean_squared_error>
	pub struct QualityMSE {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { QualityMSE }
	
	impl Drop for QualityMSE {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_quality_QualityMSE_delete(self.as_raw_mut_QualityMSE()) };
		}
	}
	
	unsafe impl Send for QualityMSE {}
	
	impl core::AlgorithmTraitConst for QualityMSE {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for QualityMSE {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::quality::QualityBaseTraitConst for QualityMSE {
		#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::quality::QualityBaseTrait for QualityMSE {
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
		pub fn create(ref_: &impl core::ToInputArray) -> Result<core::Ptr<crate::quality::QualityMSE>> {
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
		pub fn compute(ref_: &impl core::ToInputArray, cmp: &impl core::ToInputArray, quality_map: &mut impl core::ToOutputArray) -> Result<core::Scalar> {
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
	
	boxed_cast_base! { QualityMSE, core::Algorithm, cv_quality_QualityMSE_to_Algorithm }
	
	boxed_cast_base! { QualityMSE, crate::quality::QualityBase, cv_quality_QualityMSE_to_QualityBase }
	
	impl std::fmt::Debug for QualityMSE {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("QualityMSE")
				.finish()
		}
	}
	
	/// Constant methods for [crate::quality::QualityPSNR]
	pub trait QualityPSNRTraitConst: crate::quality::QualityBaseTraitConst {
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
	
	/// Mutable methods for [crate::quality::QualityPSNR]
	pub trait QualityPSNRTrait: crate::quality::QualityBaseTrait + crate::quality::QualityPSNRTraitConst {
		fn as_raw_mut_QualityPSNR(&mut self) -> *mut c_void;
	
		/// Compute the PSNR
		/// ## Parameters
		/// * cmp: Comparison image
		/// ## Returns
		/// Per-channel PSNR value, or std::numeric_limits<double>::infinity() if the MSE between the two images == 0
		#[inline]
		fn compute(&mut self, cmp: &impl core::ToInputArray) -> Result<core::Scalar> {
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
	
	/// Full reference peak signal to noise ratio (PSNR) algorithm  <https://en.wikipedia.org/wiki/Peak_signal-to-noise_ratio>
	pub struct QualityPSNR {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { QualityPSNR }
	
	impl Drop for QualityPSNR {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_quality_QualityPSNR_delete(self.as_raw_mut_QualityPSNR()) };
		}
	}
	
	unsafe impl Send for QualityPSNR {}
	
	impl core::AlgorithmTraitConst for QualityPSNR {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for QualityPSNR {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::quality::QualityBaseTraitConst for QualityPSNR {
		#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::quality::QualityBaseTrait for QualityPSNR {
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
		pub fn create(ref_: &impl core::ToInputArray, max_pixel_value: f64) -> Result<core::Ptr<crate::quality::QualityPSNR>> {
			input_array_arg!(ref_);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_quality_QualityPSNR_create_const__InputArrayR_double(ref_.as_raw__InputArray(), max_pixel_value, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<crate::quality::QualityPSNR>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
		/// Create an object which calculates quality
		/// ## Parameters
		/// * ref: input image to use as the source for comparison
		/// * maxPixelValue: maximum per-channel value for any individual pixel; eg 255 for uint8 image
		/// 
		/// ## Note
		/// This alternative version of [create] function uses the following default values for its arguments:
		/// * max_pixel_value: QualityPSNR::MAX_PIXEL_VALUE_DEFAULT
		#[inline]
		pub fn create_def(ref_: &impl core::ToInputArray) -> Result<core::Ptr<crate::quality::QualityPSNR>> {
			input_array_arg!(ref_);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_quality_QualityPSNR_create_const__InputArrayR(ref_.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
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
		pub fn compute(ref_: &impl core::ToInputArray, cmp: &impl core::ToInputArray, quality_map: &mut impl core::ToOutputArray, max_pixel_value: f64) -> Result<core::Scalar> {
			input_array_arg!(ref_);
			input_array_arg!(cmp);
			output_array_arg!(quality_map);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_quality_QualityPSNR_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double(ref_.as_raw__InputArray(), cmp.as_raw__InputArray(), quality_map.as_raw__OutputArray(), max_pixel_value, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
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
		/// ## Note
		/// This alternative version of [compute] function uses the following default values for its arguments:
		/// * max_pixel_value: QualityPSNR::MAX_PIXEL_VALUE_DEFAULT
		#[inline]
		pub fn compute_def(ref_: &impl core::ToInputArray, cmp: &impl core::ToInputArray, quality_map: &mut impl core::ToOutputArray) -> Result<core::Scalar> {
			input_array_arg!(ref_);
			input_array_arg!(cmp);
			output_array_arg!(quality_map);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_quality_QualityPSNR_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(ref_.as_raw__InputArray(), cmp.as_raw__InputArray(), quality_map.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
	
	boxed_cast_base! { QualityPSNR, core::Algorithm, cv_quality_QualityPSNR_to_Algorithm }
	
	boxed_cast_base! { QualityPSNR, crate::quality::QualityBase, cv_quality_QualityPSNR_to_QualityBase }
	
	impl std::fmt::Debug for QualityPSNR {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("QualityPSNR")
				.finish()
		}
	}
	
	/// Constant methods for [crate::quality::QualitySSIM]
	pub trait QualitySSIMTraitConst: crate::quality::QualityBaseTraitConst {
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
	
	/// Mutable methods for [crate::quality::QualitySSIM]
	pub trait QualitySSIMTrait: crate::quality::QualityBaseTrait + crate::quality::QualitySSIMTraitConst {
		fn as_raw_mut_QualitySSIM(&mut self) -> *mut c_void;
	
		/// Computes SSIM
		/// ## Parameters
		/// * cmp: Comparison image
		/// ## Returns
		/// cv::Scalar with per-channel quality values.  Values range from 0 (worst) to 1 (best)
		#[inline]
		fn compute(&mut self, cmp: &impl core::ToInputArray) -> Result<core::Scalar> {
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
	
	/// Full reference structural similarity algorithm  <https://en.wikipedia.org/wiki/Structural_similarity>
	pub struct QualitySSIM {
		ptr: *mut c_void
	}
	
	opencv_type_boxed! { QualitySSIM }
	
	impl Drop for QualitySSIM {
		#[inline]
		fn drop(&mut self) {
			unsafe { sys::cv_quality_QualitySSIM_delete(self.as_raw_mut_QualitySSIM()) };
		}
	}
	
	unsafe impl Send for QualitySSIM {}
	
	impl core::AlgorithmTraitConst for QualitySSIM {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::AlgorithmTrait for QualitySSIM {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::quality::QualityBaseTraitConst for QualitySSIM {
		#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.as_raw() }
	}
	
	impl crate::quality::QualityBaseTrait for QualitySSIM {
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
		pub fn create(ref_: &impl core::ToInputArray) -> Result<core::Ptr<crate::quality::QualitySSIM>> {
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
		pub fn compute(ref_: &impl core::ToInputArray, cmp: &impl core::ToInputArray, quality_map: &mut impl core::ToOutputArray) -> Result<core::Scalar> {
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
	
	boxed_cast_base! { QualitySSIM, core::Algorithm, cv_quality_QualitySSIM_to_Algorithm }
	
	boxed_cast_base! { QualitySSIM, crate::quality::QualityBase, cv_quality_QualitySSIM_to_QualityBase }
	
	impl std::fmt::Debug for QualitySSIM {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("QualitySSIM")
				.finish()
		}
	}
}
