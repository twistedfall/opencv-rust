#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Saliency API
//! 
//! Many computer vision applications may benefit from understanding where humans focus given a scene.
//! Other than cognitively understanding the way human perceive images and scenes, finding salient
//! regions and objects in the images helps various tasks such as speeding up object detection, object
//! recognition, object tracking and content-aware image editing.
//! 
//! About the saliency, there is a rich literature but the development is very fragmented. The principal
//! purpose of this API is to give a unique interface, a unique framework for use and plug sever
//! saliency algorithms, also with very different nature and methodology, but they share the same
//! purpose, organizing algorithms into three main categories:
//! 
//! **Static Saliency**: algorithms belonging to this category, exploit different image features that
//! allow to detect salient objects in a non dynamic scenarios.
//! 
//! **Motion Saliency**: algorithms belonging to this category, are particularly focused to detect
//! salient objects over time (hence also over frame), then there is a temporal component sealing
//! cosider that allows to detect "moving" objects as salient, meaning therefore also the more general
//! sense of detection the changes in the scene.
//! 
//! **Objectness**: Objectness is usually represented as a value which reflects how likely an image
//! window covers an object of any category. Algorithms belonging to this category, avoid making
//! decisions early on, by proposing a small number of category-independent proposals, that are expected
//! to cover all objects in an image. Being able to perceive objects before identifying them is closely
//! related to bottom up visual attention (saliency).
//! 
//! ![Saliency diagram](https://docs.opencv.org/4.5.4/saliency.png)
//! 
//! To see how API works, try tracker demo:
//! <https://github.com/fpuja/opencv_contrib/blob/saliencyModuleDevelop/modules/saliency/samples/computeSaliency.cpp>
//! 
//! 
//! Note: This API has been designed with PlantUML. If you modify this API please change UML.
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::SaliencyConst, super::Saliency, super::StaticSaliencyConst, super::StaticSaliency, super::MotionSaliencyConst, super::MotionSaliency, super::ObjectnessConst, super::Objectness, super::StaticSaliencySpectralResidualTraitConst, super::StaticSaliencySpectralResidualTrait, super::StaticSaliencyFineGrainedTraitConst, super::StaticSaliencyFineGrainedTrait, super::MotionSaliencyBinWangApr2014TraitConst, super::MotionSaliencyBinWangApr2014Trait, super::ObjectnessBINGTraitConst, super::ObjectnessBINGTrait };
}

/// ********************************* Motion Saliency Base Class ***********************************
pub trait MotionSaliencyConst: crate::saliency::SaliencyConst {
	fn as_raw_MotionSaliency(&self) -> *const c_void;

}

pub trait MotionSaliency: crate::saliency::MotionSaliencyConst + crate::saliency::Saliency {
	fn as_raw_mut_MotionSaliency(&mut self) -> *mut c_void;

}

/// !
///  * A Fast Self-tuning Background Subtraction Algorithm.
///  *
///  * This background subtraction algorithm is inspired to the work of B. Wang and P. Dudek [2]
///  * [2]  B. Wang and P. Dudek "A Fast Self-tuning Background Subtraction Algorithm", in proc of IEEE Workshop on Change Detection, 2014
///  *
/// 
/// the Fast Self-tuning Background Subtraction Algorithm from [BinWangApr2014](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_BinWangApr2014)
pub trait MotionSaliencyBinWangApr2014TraitConst: crate::saliency::MotionSaliencyConst {
	fn as_raw_MotionSaliencyBinWangApr2014(&self) -> *const c_void;

	#[inline]
	fn get_image_width(&self) -> Result<i32> {
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_getImageWidth_const(self.as_raw_MotionSaliencyBinWangApr2014()) }.into_result()
	}
	
	#[inline]
	fn get_image_height(&self) -> Result<i32> {
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_getImageHeight_const(self.as_raw_MotionSaliencyBinWangApr2014()) }.into_result()
	}
	
}

pub trait MotionSaliencyBinWangApr2014Trait: crate::saliency::MotionSaliency + crate::saliency::MotionSaliencyBinWangApr2014TraitConst {
	fn as_raw_mut_MotionSaliencyBinWangApr2014(&mut self) -> *mut c_void;

	#[inline]
	fn compute_saliency(&mut self, image: &dyn core::ToInputArray, saliency_map: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(saliency_map);
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_computeSaliency_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_MotionSaliencyBinWangApr2014(), image.as_raw__InputArray(), saliency_map.as_raw__OutputArray()) }.into_result()
	}
	
	/// This is a utility function that allows to set the correct size (taken from the input image) in the
	/// corresponding variables that will be used to size the data structures of the algorithm.
	/// ## Parameters
	/// * W: width of input image
	/// * H: height of input image
	#[inline]
	fn set_imagesize(&mut self, w: i32, h: i32) -> Result<()> {
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_setImagesize_int_int(self.as_raw_mut_MotionSaliencyBinWangApr2014(), w, h) }.into_result()
	}
	
	/// This function allows the correct initialization of all data structures that will be used by the
	/// algorithm.
	#[inline]
	fn init(&mut self) -> Result<bool> {
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_init(self.as_raw_mut_MotionSaliencyBinWangApr2014()) }.into_result()
	}
	
	#[inline]
	fn set_image_width(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_setImageWidth_int(self.as_raw_mut_MotionSaliencyBinWangApr2014(), val) }.into_result()
	}
	
	#[inline]
	fn set_image_height(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_setImageHeight_int(self.as_raw_mut_MotionSaliencyBinWangApr2014(), val) }.into_result()
	}
	
}

/// !
///  * A Fast Self-tuning Background Subtraction Algorithm.
///  *
///  * This background subtraction algorithm is inspired to the work of B. Wang and P. Dudek [2]
///  * [2]  B. Wang and P. Dudek "A Fast Self-tuning Background Subtraction Algorithm", in proc of IEEE Workshop on Change Detection, 2014
///  *
/// 
/// the Fast Self-tuning Background Subtraction Algorithm from [BinWangApr2014](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_BinWangApr2014)
pub struct MotionSaliencyBinWangApr2014 {
	ptr: *mut c_void
}

opencv_type_boxed! { MotionSaliencyBinWangApr2014 }

impl Drop for MotionSaliencyBinWangApr2014 {
	fn drop(&mut self) {
		extern "C" { fn cv_MotionSaliencyBinWangApr2014_delete(instance: *mut c_void); }
		unsafe { cv_MotionSaliencyBinWangApr2014_delete(self.as_raw_mut_MotionSaliencyBinWangApr2014()) };
	}
}

unsafe impl Send for MotionSaliencyBinWangApr2014 {}

impl core::AlgorithmTraitConst for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::MotionSaliencyConst for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_MotionSaliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::MotionSaliency for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_mut_MotionSaliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::SaliencyConst for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::Saliency for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::MotionSaliencyBinWangApr2014TraitConst for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_MotionSaliencyBinWangApr2014(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::MotionSaliencyBinWangApr2014Trait for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_mut_MotionSaliencyBinWangApr2014(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MotionSaliencyBinWangApr2014 {
	#[inline]
	pub fn default() -> Result<crate::saliency::MotionSaliencyBinWangApr2014> {
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_MotionSaliencyBinWangApr2014() }.into_result().map(|r| unsafe { crate::saliency::MotionSaliencyBinWangApr2014::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014>> {
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_create() }.into_result().map(|r| unsafe { core::Ptr::<crate::saliency::MotionSaliencyBinWangApr2014>::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { MotionSaliencyBinWangApr2014, core::Algorithm, cv_MotionSaliencyBinWangApr2014_to_Algorithm }

/// ********************************* Objectness Base Class ***********************************
pub trait ObjectnessConst: crate::saliency::SaliencyConst {
	fn as_raw_Objectness(&self) -> *const c_void;

}

pub trait Objectness: crate::saliency::ObjectnessConst + crate::saliency::Saliency {
	fn as_raw_mut_Objectness(&mut self) -> *mut c_void;

}

/// the Binarized normed gradients algorithm from [BING](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_BING)
pub trait ObjectnessBINGTraitConst: crate::saliency::ObjectnessConst {
	fn as_raw_ObjectnessBING(&self) -> *const c_void;

	#[inline]
	fn write(&self) -> Result<()> {
		unsafe { sys::cv_saliency_ObjectnessBING_write_const(self.as_raw_ObjectnessBING()) }.into_result()
	}
	
	#[inline]
	fn get_base(&self) -> Result<f64> {
		unsafe { sys::cv_saliency_ObjectnessBING_getBase_const(self.as_raw_ObjectnessBING()) }.into_result()
	}
	
	#[inline]
	fn get_nss(&self) -> Result<i32> {
		unsafe { sys::cv_saliency_ObjectnessBING_getNSS_const(self.as_raw_ObjectnessBING()) }.into_result()
	}
	
	#[inline]
	fn get_w(&self) -> Result<i32> {
		unsafe { sys::cv_saliency_ObjectnessBING_getW_const(self.as_raw_ObjectnessBING()) }.into_result()
	}
	
}

pub trait ObjectnessBINGTrait: crate::saliency::Objectness + crate::saliency::ObjectnessBINGTraitConst {
	fn as_raw_mut_ObjectnessBING(&mut self) -> *mut c_void;

	#[inline]
	fn compute_saliency(&mut self, image: &dyn core::ToInputArray, saliency_map: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(saliency_map);
		unsafe { sys::cv_saliency_ObjectnessBING_computeSaliency_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_ObjectnessBING(), image.as_raw__InputArray(), saliency_map.as_raw__OutputArray()) }.into_result()
	}
	
	#[inline]
	fn read(&mut self) -> Result<()> {
		unsafe { sys::cv_saliency_ObjectnessBING_read(self.as_raw_mut_ObjectnessBING()) }.into_result()
	}
	
	/// Return the list of the rectangles' objectness value,
	/// 
	/// in the same order as the *vector\<Vec4i\> objectnessBoundingBox* returned by the algorithm (in
	/// computeSaliencyImpl function). The bigger value these scores are, it is more likely to be an
	/// object window.
	#[inline]
	fn getobjectness_values(&mut self) -> Result<core::Vector<f32>> {
		unsafe { sys::cv_saliency_ObjectnessBING_getobjectnessValues(self.as_raw_mut_ObjectnessBING()) }.into_result().map(|r| unsafe { core::Vector::<f32>::opencv_from_extern(r) } )
	}
	
	/// This is a utility function that allows to set the correct path from which the algorithm will load
	/// the trained model.
	/// ## Parameters
	/// * trainingPath: trained model path
	#[inline]
	fn set_training_path(&mut self, training_path: &str) -> Result<()> {
		extern_container_arg!(training_path);
		unsafe { sys::cv_saliency_ObjectnessBING_setTrainingPath_const_StringR(self.as_raw_mut_ObjectnessBING(), training_path.opencv_as_extern()) }.into_result()
	}
	
	/// This is a utility function that allows to set an arbitrary path in which the algorithm will save the
	/// optional results
	/// 
	/// (ie writing on file the total number and the list of rectangles returned by objectess, one for
	/// each row).
	/// ## Parameters
	/// * resultsDir: results' folder path
	#[inline]
	fn set_bb_res_dir(&mut self, results_dir: &str) -> Result<()> {
		extern_container_arg!(results_dir);
		unsafe { sys::cv_saliency_ObjectnessBING_setBBResDir_const_StringR(self.as_raw_mut_ObjectnessBING(), results_dir.opencv_as_extern()) }.into_result()
	}
	
	#[inline]
	fn set_base(&mut self, val: f64) -> Result<()> {
		unsafe { sys::cv_saliency_ObjectnessBING_setBase_double(self.as_raw_mut_ObjectnessBING(), val) }.into_result()
	}
	
	#[inline]
	fn set_nss(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_saliency_ObjectnessBING_setNSS_int(self.as_raw_mut_ObjectnessBING(), val) }.into_result()
	}
	
	#[inline]
	fn set_w(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_saliency_ObjectnessBING_setW_int(self.as_raw_mut_ObjectnessBING(), val) }.into_result()
	}
	
}

/// the Binarized normed gradients algorithm from [BING](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_BING)
pub struct ObjectnessBING {
	ptr: *mut c_void
}

opencv_type_boxed! { ObjectnessBING }

impl Drop for ObjectnessBING {
	fn drop(&mut self) {
		extern "C" { fn cv_ObjectnessBING_delete(instance: *mut c_void); }
		unsafe { cv_ObjectnessBING_delete(self.as_raw_mut_ObjectnessBING()) };
	}
}

unsafe impl Send for ObjectnessBING {}

impl core::AlgorithmTraitConst for ObjectnessBING {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ObjectnessBING {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::ObjectnessConst for ObjectnessBING {
	#[inline] fn as_raw_Objectness(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::Objectness for ObjectnessBING {
	#[inline] fn as_raw_mut_Objectness(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::SaliencyConst for ObjectnessBING {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::Saliency for ObjectnessBING {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::ObjectnessBINGTraitConst for ObjectnessBING {
	#[inline] fn as_raw_ObjectnessBING(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::ObjectnessBINGTrait for ObjectnessBING {
	#[inline] fn as_raw_mut_ObjectnessBING(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ObjectnessBING {
	#[inline]
	pub fn default() -> Result<crate::saliency::ObjectnessBING> {
		unsafe { sys::cv_saliency_ObjectnessBING_ObjectnessBING() }.into_result().map(|r| unsafe { crate::saliency::ObjectnessBING::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::saliency::ObjectnessBING>> {
		unsafe { sys::cv_saliency_ObjectnessBING_create() }.into_result().map(|r| unsafe { core::Ptr::<crate::saliency::ObjectnessBING>::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { ObjectnessBING, core::Algorithm, cv_ObjectnessBING_to_Algorithm }

/// ********************************* Saliency Base Class ***********************************
pub trait SaliencyConst: core::AlgorithmTraitConst {
	fn as_raw_Saliency(&self) -> *const c_void;

}

pub trait Saliency: core::AlgorithmTrait + crate::saliency::SaliencyConst {
	fn as_raw_mut_Saliency(&mut self) -> *mut c_void;

	/// \brief Compute the saliency
	/// \param image        The image.
	/// \param saliencyMap      The computed saliency map.
	/// \return true if the saliency map is computed, false otherwise
	#[inline]
	fn compute_saliency(&mut self, image: &dyn core::ToInputArray, saliency_map: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(saliency_map);
		unsafe { sys::cv_saliency_Saliency_computeSaliency_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_Saliency(), image.as_raw__InputArray(), saliency_map.as_raw__OutputArray()) }.into_result()
	}
	
}

/// ********************************* Static Saliency Base Class ***********************************
pub trait StaticSaliencyConst: crate::saliency::SaliencyConst {
	fn as_raw_StaticSaliency(&self) -> *const c_void;

}

pub trait StaticSaliency: crate::saliency::Saliency + crate::saliency::StaticSaliencyConst {
	fn as_raw_mut_StaticSaliency(&mut self) -> *mut c_void;

	/// This function perform a binary map of given saliency map. This is obtained in this
	/// way:
	/// 
	/// In a first step, to improve the definition of interest areas and facilitate identification of
	/// targets, a segmentation by clustering is performed, using *K-means algorithm*. Then, to gain a
	/// binary representation of clustered saliency map, since values of the map can vary according to
	/// the characteristics of frame under analysis, it is not convenient to use a fixed threshold. So,
	/// *Otsu's algorithm* is used, which assumes that the image to be thresholded contains two classes
	/// of pixels or bi-modal histograms (e.g. foreground and back-ground pixels); later on, the
	/// algorithm calculates the optimal threshold separating those two classes, so that their
	/// intra-class variance is minimal.
	/// 
	/// ## Parameters
	/// * _saliencyMap: the saliency map obtained through one of the specialized algorithms
	/// * _binaryMap: the binary map
	#[inline]
	fn compute_binary_map(&mut self, _saliency_map: &dyn core::ToInputArray, _binary_map: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(_saliency_map);
		output_array_arg!(_binary_map);
		unsafe { sys::cv_saliency_StaticSaliency_computeBinaryMap_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_StaticSaliency(), _saliency_map.as_raw__InputArray(), _binary_map.as_raw__OutputArray()) }.into_result()
	}
	
}

/// the Fine Grained Saliency approach from [FGS](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_FGS)
/// 
/// This method calculates saliency based on center-surround differences.
/// High resolution saliency maps are generated in real time by using integral images.
pub trait StaticSaliencyFineGrainedTraitConst: crate::saliency::StaticSaliencyConst {
	fn as_raw_StaticSaliencyFineGrained(&self) -> *const c_void;

}

pub trait StaticSaliencyFineGrainedTrait: crate::saliency::StaticSaliency + crate::saliency::StaticSaliencyFineGrainedTraitConst {
	fn as_raw_mut_StaticSaliencyFineGrained(&mut self) -> *mut c_void;

	#[inline]
	fn compute_saliency(&mut self, image: &dyn core::ToInputArray, saliency_map: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(saliency_map);
		unsafe { sys::cv_saliency_StaticSaliencyFineGrained_computeSaliency_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_StaticSaliencyFineGrained(), image.as_raw__InputArray(), saliency_map.as_raw__OutputArray()) }.into_result()
	}
	
}

/// the Fine Grained Saliency approach from [FGS](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_FGS)
/// 
/// This method calculates saliency based on center-surround differences.
/// High resolution saliency maps are generated in real time by using integral images.
pub struct StaticSaliencyFineGrained {
	ptr: *mut c_void
}

opencv_type_boxed! { StaticSaliencyFineGrained }

impl Drop for StaticSaliencyFineGrained {
	fn drop(&mut self) {
		extern "C" { fn cv_StaticSaliencyFineGrained_delete(instance: *mut c_void); }
		unsafe { cv_StaticSaliencyFineGrained_delete(self.as_raw_mut_StaticSaliencyFineGrained()) };
	}
}

unsafe impl Send for StaticSaliencyFineGrained {}

impl core::AlgorithmTraitConst for StaticSaliencyFineGrained {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for StaticSaliencyFineGrained {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::SaliencyConst for StaticSaliencyFineGrained {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::Saliency for StaticSaliencyFineGrained {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::StaticSaliencyConst for StaticSaliencyFineGrained {
	#[inline] fn as_raw_StaticSaliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::StaticSaliency for StaticSaliencyFineGrained {
	#[inline] fn as_raw_mut_StaticSaliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::StaticSaliencyFineGrainedTraitConst for StaticSaliencyFineGrained {
	#[inline] fn as_raw_StaticSaliencyFineGrained(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::StaticSaliencyFineGrainedTrait for StaticSaliencyFineGrained {
	#[inline] fn as_raw_mut_StaticSaliencyFineGrained(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl StaticSaliencyFineGrained {
	#[inline]
	pub fn default() -> Result<crate::saliency::StaticSaliencyFineGrained> {
		unsafe { sys::cv_saliency_StaticSaliencyFineGrained_StaticSaliencyFineGrained() }.into_result().map(|r| unsafe { crate::saliency::StaticSaliencyFineGrained::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::saliency::StaticSaliencyFineGrained>> {
		unsafe { sys::cv_saliency_StaticSaliencyFineGrained_create() }.into_result().map(|r| unsafe { core::Ptr::<crate::saliency::StaticSaliencyFineGrained>::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { StaticSaliencyFineGrained, core::Algorithm, cv_StaticSaliencyFineGrained_to_Algorithm }

/// the Spectral Residual approach from  [SR](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_SR)
/// 
/// Starting from the principle of natural image statistics, this method simulate the behavior of
/// pre-attentive visual search. The algorithm analyze the log spectrum of each image and obtain the
/// spectral residual. Then transform the spectral residual to spatial domain to obtain the saliency
/// map, which suggests the positions of proto-objects.
pub trait StaticSaliencySpectralResidualTraitConst: crate::saliency::StaticSaliencyConst {
	fn as_raw_StaticSaliencySpectralResidual(&self) -> *const c_void;

	#[inline]
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_write_const_FileStorageR(self.as_raw_StaticSaliencySpectralResidual(), fs.as_raw_mut_FileStorage()) }.into_result()
	}
	
	#[inline]
	fn get_image_width(&self) -> Result<i32> {
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_getImageWidth_const(self.as_raw_StaticSaliencySpectralResidual()) }.into_result()
	}
	
	#[inline]
	fn get_image_height(&self) -> Result<i32> {
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_getImageHeight_const(self.as_raw_StaticSaliencySpectralResidual()) }.into_result()
	}
	
}

pub trait StaticSaliencySpectralResidualTrait: crate::saliency::StaticSaliency + crate::saliency::StaticSaliencySpectralResidualTraitConst {
	fn as_raw_mut_StaticSaliencySpectralResidual(&mut self) -> *mut c_void;

	#[inline]
	fn compute_saliency(&mut self, image: &dyn core::ToInputArray, saliency_map: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(saliency_map);
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_computeSaliency_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_StaticSaliencySpectralResidual(), image.as_raw__InputArray(), saliency_map.as_raw__OutputArray()) }.into_result()
	}
	
	#[inline]
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_read_const_FileNodeR(self.as_raw_mut_StaticSaliencySpectralResidual(), fn_.as_raw_FileNode()) }.into_result()
	}
	
	#[inline]
	fn set_image_width(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_setImageWidth_int(self.as_raw_mut_StaticSaliencySpectralResidual(), val) }.into_result()
	}
	
	#[inline]
	fn set_image_height(&mut self, val: i32) -> Result<()> {
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_setImageHeight_int(self.as_raw_mut_StaticSaliencySpectralResidual(), val) }.into_result()
	}
	
}

/// the Spectral Residual approach from  [SR](https://docs.opencv.org/4.5.4/d0/de3/citelist.html#CITEREF_SR)
/// 
/// Starting from the principle of natural image statistics, this method simulate the behavior of
/// pre-attentive visual search. The algorithm analyze the log spectrum of each image and obtain the
/// spectral residual. Then transform the spectral residual to spatial domain to obtain the saliency
/// map, which suggests the positions of proto-objects.
pub struct StaticSaliencySpectralResidual {
	ptr: *mut c_void
}

opencv_type_boxed! { StaticSaliencySpectralResidual }

impl Drop for StaticSaliencySpectralResidual {
	fn drop(&mut self) {
		extern "C" { fn cv_StaticSaliencySpectralResidual_delete(instance: *mut c_void); }
		unsafe { cv_StaticSaliencySpectralResidual_delete(self.as_raw_mut_StaticSaliencySpectralResidual()) };
	}
}

unsafe impl Send for StaticSaliencySpectralResidual {}

impl core::AlgorithmTraitConst for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::SaliencyConst for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::Saliency for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::StaticSaliencyConst for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_StaticSaliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::StaticSaliency for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_mut_StaticSaliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::StaticSaliencySpectralResidualTraitConst for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_StaticSaliencySpectralResidual(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::StaticSaliencySpectralResidualTrait for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_mut_StaticSaliencySpectralResidual(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl StaticSaliencySpectralResidual {
	#[inline]
	pub fn default() -> Result<crate::saliency::StaticSaliencySpectralResidual> {
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_StaticSaliencySpectralResidual() }.into_result().map(|r| unsafe { crate::saliency::StaticSaliencySpectralResidual::opencv_from_extern(r) } )
	}
	
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::saliency::StaticSaliencySpectralResidual>> {
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_create() }.into_result().map(|r| unsafe { core::Ptr::<crate::saliency::StaticSaliencySpectralResidual>::opencv_from_extern(r) } )
	}
	
}

boxed_cast_base! { StaticSaliencySpectralResidual, core::Algorithm, cv_StaticSaliencySpectralResidual_to_Algorithm }
