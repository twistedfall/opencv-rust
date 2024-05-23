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
//! ![Saliency diagram](https://docs.opencv.org/3.4.20/saliency.png)
//!
//! To see how API works, try tracker demo:
//! <https://github.com/fpuja/opencv_contrib/blob/saliencyModuleDevelop/modules/saliency/samples/computeSaliency.cpp>
//!
//!
//! Note: This API has been designed with PlantUML. If you modify this API please change UML.
use crate::mod_prelude::*;
use crate::{core, sys, types};
pub mod prelude {
	pub use super::{MotionSaliencyBinWangApr2014Trait, MotionSaliencyBinWangApr2014TraitConst, MotionSaliencyTrait, MotionSaliencyTraitConst, ObjectnessBINGTrait, ObjectnessBINGTraitConst, ObjectnessTrait, ObjectnessTraitConst, SaliencyTrait, SaliencyTraitConst, StaticSaliencyFineGrainedTrait, StaticSaliencyFineGrainedTraitConst, StaticSaliencySpectralResidualTrait, StaticSaliencySpectralResidualTraitConst, StaticSaliencyTrait, StaticSaliencyTraitConst};
}

/// Constant methods for [crate::saliency::MotionSaliency]
// MotionSaliency /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencyBaseClasses.hpp:111
pub trait MotionSaliencyTraitConst: crate::saliency::SaliencyTraitConst {
	fn as_raw_MotionSaliency(&self) -> *const c_void;

}

/// Mutable methods for [crate::saliency::MotionSaliency]
pub trait MotionSaliencyTrait: crate::saliency::MotionSaliencyTraitConst + crate::saliency::SaliencyTrait {
	fn as_raw_mut_MotionSaliency(&mut self) -> *mut c_void;

}

/// ********************************* Motion Saliency Base Class ***********************************
// MotionSaliency /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencyBaseClasses.hpp:111
pub struct MotionSaliency {
	ptr: *mut c_void,
}

opencv_type_boxed! { MotionSaliency }

impl Drop for MotionSaliency {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_saliency_MotionSaliency_delete(self.as_raw_mut_MotionSaliency()) };
	}
}

unsafe impl Send for MotionSaliency {}

impl core::AlgorithmTraitConst for MotionSaliency {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for MotionSaliency {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MotionSaliency, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::saliency::SaliencyTraitConst for MotionSaliency {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::SaliencyTrait for MotionSaliency {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MotionSaliency, crate::saliency::SaliencyTraitConst, as_raw_Saliency, crate::saliency::SaliencyTrait, as_raw_mut_Saliency }

impl crate::saliency::MotionSaliencyTraitConst for MotionSaliency {
	#[inline] fn as_raw_MotionSaliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::MotionSaliencyTrait for MotionSaliency {
	#[inline] fn as_raw_mut_MotionSaliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MotionSaliency, crate::saliency::MotionSaliencyTraitConst, as_raw_MotionSaliency, crate::saliency::MotionSaliencyTrait, as_raw_mut_MotionSaliency }

impl MotionSaliency {
}

boxed_cast_descendant! { MotionSaliency, crate::saliency::MotionSaliencyBinWangApr2014, cv_saliency_MotionSaliency_to_MotionSaliencyBinWangApr2014 }

boxed_cast_base! { MotionSaliency, core::Algorithm, cv_saliency_MotionSaliency_to_Algorithm }

boxed_cast_base! { MotionSaliency, crate::saliency::Saliency, cv_saliency_MotionSaliency_to_Saliency }

impl std::fmt::Debug for MotionSaliency {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MotionSaliency")
			.finish()
	}
}

/// Constant methods for [crate::saliency::MotionSaliencyBinWangApr2014]
// MotionSaliencyBinWangApr2014 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:169
pub trait MotionSaliencyBinWangApr2014TraitConst: crate::saliency::MotionSaliencyTraitConst {
	fn as_raw_MotionSaliencyBinWangApr2014(&self) -> *const c_void;

	// getImageWidth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:199
	// ("cv::saliency::MotionSaliencyBinWangApr2014::getImageWidth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_image_width(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_getImageWidth_const(self.as_raw_MotionSaliencyBinWangApr2014(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getImageHeight()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:207
	// ("cv::saliency::MotionSaliencyBinWangApr2014::getImageHeight", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_image_height(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_getImageHeight_const(self.as_raw_MotionSaliencyBinWangApr2014(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::saliency::MotionSaliencyBinWangApr2014]
pub trait MotionSaliencyBinWangApr2014Trait: crate::saliency::MotionSaliencyBinWangApr2014TraitConst + crate::saliency::MotionSaliencyTrait {
	fn as_raw_mut_MotionSaliencyBinWangApr2014(&mut self) -> *mut c_void;

	// computeSaliency(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:180
	// ("cv::saliency::MotionSaliencyBinWangApr2014::computeSaliency", vec![(pred!(mut, ["image", "saliencyMap"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute_saliency(&mut self, image: &impl ToInputArray, saliency_map: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(saliency_map);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_computeSaliency_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_MotionSaliencyBinWangApr2014(), image.as_raw__InputArray(), saliency_map.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// This is a utility function that allows to set the correct size (taken from the input image) in the
	/// corresponding variables that will be used to size the data structures of the algorithm.
	/// ## Parameters
	/// * W: width of input image
	/// * H: height of input image
	// setImagesize(int, int)(Primitive, Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:193
	// ("cv::saliency::MotionSaliencyBinWangApr2014::setImagesize", vec![(pred!(mut, ["W", "H"], ["int", "int"]), _)]),
	#[inline]
	fn set_imagesize(&mut self, w: i32, h: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_setImagesize_int_int(self.as_raw_mut_MotionSaliencyBinWangApr2014(), w, h, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// This function allows the correct initialization of all data structures that will be used by the
	/// algorithm.
	// init()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:197
	// ("cv::saliency::MotionSaliencyBinWangApr2014::init", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn init(&mut self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_init(self.as_raw_mut_MotionSaliencyBinWangApr2014(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setImageWidth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:203
	// ("cv::saliency::MotionSaliencyBinWangApr2014::setImageWidth", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_image_width(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_setImageWidth_int(self.as_raw_mut_MotionSaliencyBinWangApr2014(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setImageHeight(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:211
	// ("cv::saliency::MotionSaliencyBinWangApr2014::setImageHeight", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_image_height(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_setImageHeight_int(self.as_raw_mut_MotionSaliencyBinWangApr2014(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// !
///  * A Fast Self-tuning Background Subtraction Algorithm.
///  *
///  * This background subtraction algorithm is inspired to the work of B. Wang and P. Dudek [2]
///  * [2]  B. Wang and P. Dudek "A Fast Self-tuning Background Subtraction Algorithm", in proc of IEEE Workshop on Change Detection, 2014
///  *
///
/// the Fast Self-tuning Background Subtraction Algorithm from [BinWangApr2014](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_BinWangApr2014)
// MotionSaliencyBinWangApr2014 /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:169
pub struct MotionSaliencyBinWangApr2014 {
	ptr: *mut c_void,
}

opencv_type_boxed! { MotionSaliencyBinWangApr2014 }

impl Drop for MotionSaliencyBinWangApr2014 {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_delete(self.as_raw_mut_MotionSaliencyBinWangApr2014()) };
	}
}

unsafe impl Send for MotionSaliencyBinWangApr2014 {}

impl core::AlgorithmTraitConst for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MotionSaliencyBinWangApr2014, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::saliency::MotionSaliencyTraitConst for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_MotionSaliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::MotionSaliencyTrait for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_mut_MotionSaliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MotionSaliencyBinWangApr2014, crate::saliency::MotionSaliencyTraitConst, as_raw_MotionSaliency, crate::saliency::MotionSaliencyTrait, as_raw_mut_MotionSaliency }

impl crate::saliency::SaliencyTraitConst for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::SaliencyTrait for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MotionSaliencyBinWangApr2014, crate::saliency::SaliencyTraitConst, as_raw_Saliency, crate::saliency::SaliencyTrait, as_raw_mut_Saliency }

impl crate::saliency::MotionSaliencyBinWangApr2014TraitConst for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_MotionSaliencyBinWangApr2014(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::MotionSaliencyBinWangApr2014Trait for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_mut_MotionSaliencyBinWangApr2014(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { MotionSaliencyBinWangApr2014, crate::saliency::MotionSaliencyBinWangApr2014TraitConst, as_raw_MotionSaliencyBinWangApr2014, crate::saliency::MotionSaliencyBinWangApr2014Trait, as_raw_mut_MotionSaliencyBinWangApr2014 }

impl MotionSaliencyBinWangApr2014 {
	// MotionSaliencyBinWangApr2014()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:172
	// ("cv::saliency::MotionSaliencyBinWangApr2014::MotionSaliencyBinWangApr2014", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::saliency::MotionSaliencyBinWangApr2014> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_MotionSaliencyBinWangApr2014(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::saliency::MotionSaliencyBinWangApr2014::opencv_from_extern(ret) };
		Ok(ret)
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:175
	// ("cv::saliency::MotionSaliencyBinWangApr2014::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::saliency::MotionSaliencyBinWangApr2014>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { MotionSaliencyBinWangApr2014, core::Algorithm, cv_saliency_MotionSaliencyBinWangApr2014_to_Algorithm }

boxed_cast_base! { MotionSaliencyBinWangApr2014, crate::saliency::MotionSaliency, cv_saliency_MotionSaliencyBinWangApr2014_to_MotionSaliency }

boxed_cast_base! { MotionSaliencyBinWangApr2014, crate::saliency::Saliency, cv_saliency_MotionSaliencyBinWangApr2014_to_Saliency }

impl std::fmt::Debug for MotionSaliencyBinWangApr2014 {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("MotionSaliencyBinWangApr2014")
			.finish()
	}
}

/// Constant methods for [crate::saliency::Objectness]
// Objectness /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencyBaseClasses.hpp:120
pub trait ObjectnessTraitConst: crate::saliency::SaliencyTraitConst {
	fn as_raw_Objectness(&self) -> *const c_void;

}

/// Mutable methods for [crate::saliency::Objectness]
pub trait ObjectnessTrait: crate::saliency::ObjectnessTraitConst + crate::saliency::SaliencyTrait {
	fn as_raw_mut_Objectness(&mut self) -> *mut c_void;

}

/// ********************************* Objectness Base Class ***********************************
// Objectness /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencyBaseClasses.hpp:120
pub struct Objectness {
	ptr: *mut c_void,
}

opencv_type_boxed! { Objectness }

impl Drop for Objectness {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_saliency_Objectness_delete(self.as_raw_mut_Objectness()) };
	}
}

unsafe impl Send for Objectness {}

impl core::AlgorithmTraitConst for Objectness {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for Objectness {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Objectness, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::saliency::SaliencyTraitConst for Objectness {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::SaliencyTrait for Objectness {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Objectness, crate::saliency::SaliencyTraitConst, as_raw_Saliency, crate::saliency::SaliencyTrait, as_raw_mut_Saliency }

impl crate::saliency::ObjectnessTraitConst for Objectness {
	#[inline] fn as_raw_Objectness(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::ObjectnessTrait for Objectness {
	#[inline] fn as_raw_mut_Objectness(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Objectness, crate::saliency::ObjectnessTraitConst, as_raw_Objectness, crate::saliency::ObjectnessTrait, as_raw_mut_Objectness }

impl Objectness {
}

boxed_cast_descendant! { Objectness, crate::saliency::ObjectnessBING, cv_saliency_Objectness_to_ObjectnessBING }

boxed_cast_base! { Objectness, core::Algorithm, cv_saliency_Objectness_to_Algorithm }

boxed_cast_base! { Objectness, crate::saliency::Saliency, cv_saliency_Objectness_to_Saliency }

impl std::fmt::Debug for Objectness {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Objectness")
			.finish()
	}
}

/// Constant methods for [crate::saliency::ObjectnessBING]
// ObjectnessBING /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:291
pub trait ObjectnessBINGTraitConst: crate::saliency::ObjectnessTraitConst {
	fn as_raw_ObjectnessBING(&self) -> *const c_void;

	// write()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:312
	// ("cv::saliency::ObjectnessBING::write", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn write(&self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_write_const(self.as_raw_ObjectnessBING(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getBase()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:337
	// ("cv::saliency::ObjectnessBING::getBase", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_base(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_getBase_const(self.as_raw_ObjectnessBING(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getNSS()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:345
	// ("cv::saliency::ObjectnessBING::getNSS", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_nss(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_getNSS_const(self.as_raw_ObjectnessBING(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getW()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:353
	// ("cv::saliency::ObjectnessBING::getW", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_w(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_getW_const(self.as_raw_ObjectnessBING(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::saliency::ObjectnessBING]
pub trait ObjectnessBINGTrait: crate::saliency::ObjectnessBINGTraitConst + crate::saliency::ObjectnessTrait {
	fn as_raw_mut_ObjectnessBING(&mut self) -> *mut c_void;

	// computeSaliency(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:303
	// ("cv::saliency::ObjectnessBING::computeSaliency", vec![(pred!(mut, ["image", "saliencyMap"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute_saliency(&mut self, image: &impl ToInputArray, saliency_map: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(saliency_map);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_computeSaliency_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_ObjectnessBING(), image.as_raw__InputArray(), saliency_map.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// read()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:311
	// ("cv::saliency::ObjectnessBING::read", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn read(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_read(self.as_raw_mut_ObjectnessBING(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// Return the list of the rectangles' objectness value,
	///
	/// in the same order as the *vector\<Vec4i\> objectnessBoundingBox* returned by the algorithm (in
	/// computeSaliencyImpl function). The bigger value these scores are, it is more likely to be an
	/// object window.
	// getobjectnessValues()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:320
	// ("cv::saliency::ObjectnessBING::getobjectnessValues", vec![(pred!(mut, [], []), _)]),
	#[inline]
	fn getobjectness_values(&mut self) -> Result<core::Vector<f32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_getobjectnessValues(self.as_raw_mut_ObjectnessBING(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<f32>::opencv_from_extern(ret) };
		Ok(ret)
	}

	/// This is a utility function that allows to set the correct path from which the algorithm will load
	/// the trained model.
	/// ## Parameters
	/// * trainingPath: trained model path
	// setTrainingPath(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:326
	// ("cv::saliency::ObjectnessBING::setTrainingPath", vec![(pred!(mut, ["trainingPath"], ["const cv::String*"]), _)]),
	#[inline]
	fn set_training_path(&mut self, training_path: &str) -> Result<()> {
		extern_container_arg!(training_path);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_setTrainingPath_const_StringR(self.as_raw_mut_ObjectnessBING(), training_path.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	/// This is a utility function that allows to set an arbitrary path in which the algorithm will save the
	/// optional results
	///
	/// (ie writing on file the total number and the list of rectangles returned by objectess, one for
	/// each row).
	/// ## Parameters
	/// * resultsDir: results' folder path
	// setBBResDir(const String &)(InString) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:335
	// ("cv::saliency::ObjectnessBING::setBBResDir", vec![(pred!(mut, ["resultsDir"], ["const cv::String*"]), _)]),
	#[inline]
	fn set_bb_res_dir(&mut self, results_dir: &str) -> Result<()> {
		extern_container_arg!(results_dir);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_setBBResDir_const_StringR(self.as_raw_mut_ObjectnessBING(), results_dir.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setBase(double)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:341
	// ("cv::saliency::ObjectnessBING::setBase", vec![(pred!(mut, ["val"], ["double"]), _)]),
	#[inline]
	fn set_base(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_setBase_double(self.as_raw_mut_ObjectnessBING(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setNSS(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:349
	// ("cv::saliency::ObjectnessBING::setNSS", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_nss(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_setNSS_int(self.as_raw_mut_ObjectnessBING(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setW(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:357
	// ("cv::saliency::ObjectnessBING::setW", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_w(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_setW_int(self.as_raw_mut_ObjectnessBING(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// the Binarized normed gradients algorithm from [BING](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_BING)
// ObjectnessBING /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:291
pub struct ObjectnessBING {
	ptr: *mut c_void,
}

opencv_type_boxed! { ObjectnessBING }

impl Drop for ObjectnessBING {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_saliency_ObjectnessBING_delete(self.as_raw_mut_ObjectnessBING()) };
	}
}

unsafe impl Send for ObjectnessBING {}

impl core::AlgorithmTraitConst for ObjectnessBING {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ObjectnessBING {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ObjectnessBING, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::saliency::ObjectnessTraitConst for ObjectnessBING {
	#[inline] fn as_raw_Objectness(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::ObjectnessTrait for ObjectnessBING {
	#[inline] fn as_raw_mut_Objectness(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ObjectnessBING, crate::saliency::ObjectnessTraitConst, as_raw_Objectness, crate::saliency::ObjectnessTrait, as_raw_mut_Objectness }

impl crate::saliency::SaliencyTraitConst for ObjectnessBING {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::SaliencyTrait for ObjectnessBING {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ObjectnessBING, crate::saliency::SaliencyTraitConst, as_raw_Saliency, crate::saliency::SaliencyTrait, as_raw_mut_Saliency }

impl crate::saliency::ObjectnessBINGTraitConst for ObjectnessBING {
	#[inline] fn as_raw_ObjectnessBING(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::ObjectnessBINGTrait for ObjectnessBING {
	#[inline] fn as_raw_mut_ObjectnessBING(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { ObjectnessBING, crate::saliency::ObjectnessBINGTraitConst, as_raw_ObjectnessBING, crate::saliency::ObjectnessBINGTrait, as_raw_mut_ObjectnessBING }

impl ObjectnessBING {
	// ObjectnessBING()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:295
	// ("cv::saliency::ObjectnessBING::ObjectnessBING", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::saliency::ObjectnessBING> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_ObjectnessBING(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::saliency::ObjectnessBING::opencv_from_extern(ret) };
		Ok(ret)
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:298
	// ("cv::saliency::ObjectnessBING::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::saliency::ObjectnessBING>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::saliency::ObjectnessBING>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { ObjectnessBING, core::Algorithm, cv_saliency_ObjectnessBING_to_Algorithm }

boxed_cast_base! { ObjectnessBING, crate::saliency::Objectness, cv_saliency_ObjectnessBING_to_Objectness }

boxed_cast_base! { ObjectnessBING, crate::saliency::Saliency, cv_saliency_ObjectnessBING_to_Saliency }

impl std::fmt::Debug for ObjectnessBING {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("ObjectnessBING")
			.finish()
	}
}

/// Constant methods for [crate::saliency::Saliency]
// Saliency /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencyBaseClasses.hpp:62
pub trait SaliencyTraitConst: core::AlgorithmTraitConst {
	fn as_raw_Saliency(&self) -> *const c_void;

}

/// Mutable methods for [crate::saliency::Saliency]
pub trait SaliencyTrait: core::AlgorithmTrait + crate::saliency::SaliencyTraitConst {
	fn as_raw_mut_Saliency(&mut self) -> *mut c_void;

	/// \brief Compute the saliency
	/// \param image        The image.
	/// \param saliencyMap      The computed saliency map.
	/// \return true if the saliency map is computed, false otherwise
	// computeSaliency(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencyBaseClasses.hpp:76
	// ("cv::saliency::Saliency::computeSaliency", vec![(pred!(mut, ["image", "saliencyMap"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute_saliency(&mut self, image: &impl ToInputArray, saliency_map: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(saliency_map);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_Saliency_computeSaliency_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_Saliency(), image.as_raw__InputArray(), saliency_map.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// ********************************* Saliency Base Class ***********************************
// Saliency /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencyBaseClasses.hpp:62
pub struct Saliency {
	ptr: *mut c_void,
}

opencv_type_boxed! { Saliency }

impl Drop for Saliency {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_saliency_Saliency_delete(self.as_raw_mut_Saliency()) };
	}
}

unsafe impl Send for Saliency {}

impl core::AlgorithmTraitConst for Saliency {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for Saliency {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Saliency, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::saliency::SaliencyTraitConst for Saliency {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::SaliencyTrait for Saliency {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { Saliency, crate::saliency::SaliencyTraitConst, as_raw_Saliency, crate::saliency::SaliencyTrait, as_raw_mut_Saliency }

impl Saliency {
}

boxed_cast_descendant! { Saliency, crate::saliency::MotionSaliency, cv_saliency_Saliency_to_MotionSaliency }

boxed_cast_descendant! { Saliency, crate::saliency::MotionSaliencyBinWangApr2014, cv_saliency_Saliency_to_MotionSaliencyBinWangApr2014 }

boxed_cast_descendant! { Saliency, crate::saliency::Objectness, cv_saliency_Saliency_to_Objectness }

boxed_cast_descendant! { Saliency, crate::saliency::ObjectnessBING, cv_saliency_Saliency_to_ObjectnessBING }

boxed_cast_descendant! { Saliency, crate::saliency::StaticSaliency, cv_saliency_Saliency_to_StaticSaliency }

boxed_cast_descendant! { Saliency, crate::saliency::StaticSaliencyFineGrained, cv_saliency_Saliency_to_StaticSaliencyFineGrained }

boxed_cast_descendant! { Saliency, crate::saliency::StaticSaliencySpectralResidual, cv_saliency_Saliency_to_StaticSaliencySpectralResidual }

boxed_cast_base! { Saliency, core::Algorithm, cv_saliency_Saliency_to_Algorithm }

impl std::fmt::Debug for Saliency {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Saliency")
			.finish()
	}
}

/// Constant methods for [crate::saliency::StaticSaliency]
// StaticSaliency /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencyBaseClasses.hpp:85
pub trait StaticSaliencyTraitConst: crate::saliency::SaliencyTraitConst {
	fn as_raw_StaticSaliency(&self) -> *const c_void;

}

/// Mutable methods for [crate::saliency::StaticSaliency]
pub trait StaticSaliencyTrait: crate::saliency::SaliencyTrait + crate::saliency::StaticSaliencyTraitConst {
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
	// computeBinaryMap(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencyBaseClasses.hpp:104
	// ("cv::saliency::StaticSaliency::computeBinaryMap", vec![(pred!(mut, ["_saliencyMap", "_binaryMap"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute_binary_map(&mut self, _saliency_map: &impl ToInputArray, _binary_map: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(_saliency_map);
		output_array_arg!(_binary_map);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliency_computeBinaryMap_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_StaticSaliency(), _saliency_map.as_raw__InputArray(), _binary_map.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// ********************************* Static Saliency Base Class ***********************************
// StaticSaliency /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencyBaseClasses.hpp:85
pub struct StaticSaliency {
	ptr: *mut c_void,
}

opencv_type_boxed! { StaticSaliency }

impl Drop for StaticSaliency {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_saliency_StaticSaliency_delete(self.as_raw_mut_StaticSaliency()) };
	}
}

unsafe impl Send for StaticSaliency {}

impl core::AlgorithmTraitConst for StaticSaliency {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for StaticSaliency {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StaticSaliency, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::saliency::SaliencyTraitConst for StaticSaliency {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::SaliencyTrait for StaticSaliency {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StaticSaliency, crate::saliency::SaliencyTraitConst, as_raw_Saliency, crate::saliency::SaliencyTrait, as_raw_mut_Saliency }

impl crate::saliency::StaticSaliencyTraitConst for StaticSaliency {
	#[inline] fn as_raw_StaticSaliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::StaticSaliencyTrait for StaticSaliency {
	#[inline] fn as_raw_mut_StaticSaliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StaticSaliency, crate::saliency::StaticSaliencyTraitConst, as_raw_StaticSaliency, crate::saliency::StaticSaliencyTrait, as_raw_mut_StaticSaliency }

impl StaticSaliency {
}

boxed_cast_descendant! { StaticSaliency, crate::saliency::StaticSaliencyFineGrained, cv_saliency_StaticSaliency_to_StaticSaliencyFineGrained }

boxed_cast_descendant! { StaticSaliency, crate::saliency::StaticSaliencySpectralResidual, cv_saliency_StaticSaliency_to_StaticSaliencySpectralResidual }

boxed_cast_base! { StaticSaliency, core::Algorithm, cv_saliency_StaticSaliency_to_Algorithm }

boxed_cast_base! { StaticSaliency, crate::saliency::Saliency, cv_saliency_StaticSaliency_to_Saliency }

impl std::fmt::Debug for StaticSaliency {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("StaticSaliency")
			.finish()
	}
}

/// Constant methods for [crate::saliency::StaticSaliencyFineGrained]
// StaticSaliencyFineGrained /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:122
pub trait StaticSaliencyFineGrainedTraitConst: crate::saliency::StaticSaliencyTraitConst {
	fn as_raw_StaticSaliencyFineGrained(&self) -> *const c_void;

}

/// Mutable methods for [crate::saliency::StaticSaliencyFineGrained]
pub trait StaticSaliencyFineGrainedTrait: crate::saliency::StaticSaliencyFineGrainedTraitConst + crate::saliency::StaticSaliencyTrait {
	fn as_raw_mut_StaticSaliencyFineGrained(&mut self) -> *mut c_void;

	// computeSaliency(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:133
	// ("cv::saliency::StaticSaliencyFineGrained::computeSaliency", vec![(pred!(mut, ["image", "saliencyMap"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute_saliency(&mut self, image: &impl ToInputArray, saliency_map: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(saliency_map);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencyFineGrained_computeSaliency_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_StaticSaliencyFineGrained(), image.as_raw__InputArray(), saliency_map.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// the Fine Grained Saliency approach from [FGS](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_FGS)
///
/// This method calculates saliency based on center-surround differences.
/// High resolution saliency maps are generated in real time by using integral images.
// StaticSaliencyFineGrained /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:122
pub struct StaticSaliencyFineGrained {
	ptr: *mut c_void,
}

opencv_type_boxed! { StaticSaliencyFineGrained }

impl Drop for StaticSaliencyFineGrained {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_saliency_StaticSaliencyFineGrained_delete(self.as_raw_mut_StaticSaliencyFineGrained()) };
	}
}

unsafe impl Send for StaticSaliencyFineGrained {}

impl core::AlgorithmTraitConst for StaticSaliencyFineGrained {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for StaticSaliencyFineGrained {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StaticSaliencyFineGrained, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::saliency::SaliencyTraitConst for StaticSaliencyFineGrained {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::SaliencyTrait for StaticSaliencyFineGrained {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StaticSaliencyFineGrained, crate::saliency::SaliencyTraitConst, as_raw_Saliency, crate::saliency::SaliencyTrait, as_raw_mut_Saliency }

impl crate::saliency::StaticSaliencyTraitConst for StaticSaliencyFineGrained {
	#[inline] fn as_raw_StaticSaliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::StaticSaliencyTrait for StaticSaliencyFineGrained {
	#[inline] fn as_raw_mut_StaticSaliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StaticSaliencyFineGrained, crate::saliency::StaticSaliencyTraitConst, as_raw_StaticSaliency, crate::saliency::StaticSaliencyTrait, as_raw_mut_StaticSaliency }

impl crate::saliency::StaticSaliencyFineGrainedTraitConst for StaticSaliencyFineGrained {
	#[inline] fn as_raw_StaticSaliencyFineGrained(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::StaticSaliencyFineGrainedTrait for StaticSaliencyFineGrained {
	#[inline] fn as_raw_mut_StaticSaliencyFineGrained(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StaticSaliencyFineGrained, crate::saliency::StaticSaliencyFineGrainedTraitConst, as_raw_StaticSaliencyFineGrained, crate::saliency::StaticSaliencyFineGrainedTrait, as_raw_mut_StaticSaliencyFineGrained }

impl StaticSaliencyFineGrained {
	// StaticSaliencyFineGrained()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:126
	// ("cv::saliency::StaticSaliencyFineGrained::StaticSaliencyFineGrained", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::saliency::StaticSaliencyFineGrained> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencyFineGrained_StaticSaliencyFineGrained(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::saliency::StaticSaliencyFineGrained::opencv_from_extern(ret) };
		Ok(ret)
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:128
	// ("cv::saliency::StaticSaliencyFineGrained::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::saliency::StaticSaliencyFineGrained>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencyFineGrained_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::saliency::StaticSaliencyFineGrained>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { StaticSaliencyFineGrained, core::Algorithm, cv_saliency_StaticSaliencyFineGrained_to_Algorithm }

boxed_cast_base! { StaticSaliencyFineGrained, crate::saliency::Saliency, cv_saliency_StaticSaliencyFineGrained_to_Saliency }

boxed_cast_base! { StaticSaliencyFineGrained, crate::saliency::StaticSaliency, cv_saliency_StaticSaliencyFineGrained_to_StaticSaliency }

impl std::fmt::Debug for StaticSaliencyFineGrained {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("StaticSaliencyFineGrained")
			.finish()
	}
}

/// Constant methods for [crate::saliency::StaticSaliencySpectralResidual]
// StaticSaliencySpectralResidual /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:69
pub trait StaticSaliencySpectralResidualTraitConst: crate::saliency::StaticSaliencyTraitConst {
	fn as_raw_StaticSaliencySpectralResidual(&self) -> *const c_void;

	// write(FileStorage &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:90
	// ("cv::saliency::StaticSaliencySpectralResidual::write", vec![(pred!(const, ["fs"], ["cv::FileStorage*"]), _)]),
	#[inline]
	fn write(&self, fs: &mut impl core::FileStorageTrait) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_write_const_FileStorageR(self.as_raw_StaticSaliencySpectralResidual(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getImageWidth()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:92
	// ("cv::saliency::StaticSaliencySpectralResidual::getImageWidth", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_image_width(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_getImageWidth_const(self.as_raw_StaticSaliencySpectralResidual(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// getImageHeight()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:100
	// ("cv::saliency::StaticSaliencySpectralResidual::getImageHeight", vec![(pred!(const, [], []), _)]),
	#[inline]
	fn get_image_height(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_getImageHeight_const(self.as_raw_StaticSaliencySpectralResidual(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// Mutable methods for [crate::saliency::StaticSaliencySpectralResidual]
pub trait StaticSaliencySpectralResidualTrait: crate::saliency::StaticSaliencySpectralResidualTraitConst + crate::saliency::StaticSaliencyTrait {
	fn as_raw_mut_StaticSaliencySpectralResidual(&mut self) -> *mut c_void;

	// computeSaliency(InputArray, OutputArray)(InputArray, OutputArray) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:81
	// ("cv::saliency::StaticSaliencySpectralResidual::computeSaliency", vec![(pred!(mut, ["image", "saliencyMap"], ["const cv::_InputArray*", "const cv::_OutputArray*"]), _)]),
	#[inline]
	fn compute_saliency(&mut self, image: &impl ToInputArray, saliency_map: &mut impl ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(saliency_map);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_computeSaliency_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_StaticSaliencySpectralResidual(), image.as_raw__InputArray(), saliency_map.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// read(const FileNode &)(TraitClass) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:89
	// ("cv::saliency::StaticSaliencySpectralResidual::read", vec![(pred!(mut, ["fn"], ["const cv::FileNode*"]), _)]),
	#[inline]
	fn read(&mut self, fn_: &impl core::FileNodeTraitConst) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_read_const_FileNodeR(self.as_raw_mut_StaticSaliencySpectralResidual(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setImageWidth(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:96
	// ("cv::saliency::StaticSaliencySpectralResidual::setImageWidth", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_image_width(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_setImageWidth_int(self.as_raw_mut_StaticSaliencySpectralResidual(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

	// setImageHeight(int)(Primitive) /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:104
	// ("cv::saliency::StaticSaliencySpectralResidual::setImageHeight", vec![(pred!(mut, ["val"], ["int"]), _)]),
	#[inline]
	fn set_image_height(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_setImageHeight_int(self.as_raw_mut_StaticSaliencySpectralResidual(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}

}

/// the Spectral Residual approach from  [SR](https://docs.opencv.org/3.4.20/d0/de3/citelist.html#CITEREF_SR)
///
/// Starting from the principle of natural image statistics, this method simulate the behavior of
/// pre-attentive visual search. The algorithm analyze the log spectrum of each image and obtain the
/// spectral residual. Then transform the spectral residual to spatial domain to obtain the saliency
/// map, which suggests the positions of proto-objects.
// StaticSaliencySpectralResidual /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:69
pub struct StaticSaliencySpectralResidual {
	ptr: *mut c_void,
}

opencv_type_boxed! { StaticSaliencySpectralResidual }

impl Drop for StaticSaliencySpectralResidual {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_delete(self.as_raw_mut_StaticSaliencySpectralResidual()) };
	}
}

unsafe impl Send for StaticSaliencySpectralResidual {}

impl core::AlgorithmTraitConst for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StaticSaliencySpectralResidual, core::AlgorithmTraitConst, as_raw_Algorithm, core::AlgorithmTrait, as_raw_mut_Algorithm }

impl crate::saliency::SaliencyTraitConst for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::SaliencyTrait for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StaticSaliencySpectralResidual, crate::saliency::SaliencyTraitConst, as_raw_Saliency, crate::saliency::SaliencyTrait, as_raw_mut_Saliency }

impl crate::saliency::StaticSaliencyTraitConst for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_StaticSaliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::StaticSaliencyTrait for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_mut_StaticSaliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StaticSaliencySpectralResidual, crate::saliency::StaticSaliencyTraitConst, as_raw_StaticSaliency, crate::saliency::StaticSaliencyTrait, as_raw_mut_StaticSaliency }

impl crate::saliency::StaticSaliencySpectralResidualTraitConst for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_StaticSaliencySpectralResidual(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::StaticSaliencySpectralResidualTrait for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_mut_StaticSaliencySpectralResidual(&mut self) -> *mut c_void { self.as_raw_mut() }
}

boxed_ref! { StaticSaliencySpectralResidual, crate::saliency::StaticSaliencySpectralResidualTraitConst, as_raw_StaticSaliencySpectralResidual, crate::saliency::StaticSaliencySpectralResidualTrait, as_raw_mut_StaticSaliencySpectralResidual }

impl StaticSaliencySpectralResidual {
	// StaticSaliencySpectralResidual()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:73
	// ("cv::saliency::StaticSaliencySpectralResidual::StaticSaliencySpectralResidual", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn default() -> Result<crate::saliency::StaticSaliencySpectralResidual> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_StaticSaliencySpectralResidual(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::saliency::StaticSaliencySpectralResidual::opencv_from_extern(ret) };
		Ok(ret)
	}

	// create()() /home/pro/projects/opencv-lib/opencv-3.4/install/include/opencv2/saliency/saliencySpecializedClasses.hpp:76
	// ("cv::saliency::StaticSaliencySpectralResidual::create", vec![(pred!(mut, [], []), _)]),
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::saliency::StaticSaliencySpectralResidual>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::saliency::StaticSaliencySpectralResidual>::opencv_from_extern(ret) };
		Ok(ret)
	}

}

boxed_cast_base! { StaticSaliencySpectralResidual, core::Algorithm, cv_saliency_StaticSaliencySpectralResidual_to_Algorithm }

boxed_cast_base! { StaticSaliencySpectralResidual, crate::saliency::Saliency, cv_saliency_StaticSaliencySpectralResidual_to_Saliency }

boxed_cast_base! { StaticSaliencySpectralResidual, crate::saliency::StaticSaliency, cv_saliency_StaticSaliencySpectralResidual_to_StaticSaliency }

impl std::fmt::Debug for StaticSaliencySpectralResidual {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("StaticSaliencySpectralResidual")
			.finish()
	}
}
