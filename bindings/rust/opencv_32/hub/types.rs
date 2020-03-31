
#[cfg(feature = "contrib")]
mod aruco_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBoard = core::Ptr::<crate::aruco::Board>;
	
	ptr_extern! { crate::aruco::Board,
		cv_PtrOfBoard_delete, cv_PtrOfBoard_get_inner_ptr
	}
	
	impl PtrOfBoard {
		pub fn as_raw_PtrOfBoard(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::aruco::BoardTrait for PtrOfBoard {
		fn as_raw_Board(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Board(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCharucoBoard = core::Ptr::<crate::aruco::CharucoBoard>;
	
	ptr_extern! { crate::aruco::CharucoBoard,
		cv_PtrOfCharucoBoard_delete, cv_PtrOfCharucoBoard_get_inner_ptr
	}
	
	impl PtrOfCharucoBoard {
		pub fn as_raw_PtrOfCharucoBoard(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfCharucoBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::aruco::BoardTrait for PtrOfCharucoBoard {
		fn as_raw_Board(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Board(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::aruco::CharucoBoardTrait for PtrOfCharucoBoard {
		fn as_raw_CharucoBoard(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_CharucoBoard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetectorParameters = core::Ptr::<crate::aruco::DetectorParameters>;
	
	ptr_extern! { crate::aruco::DetectorParameters,
		cv_PtrOfDetectorParameters_delete, cv_PtrOfDetectorParameters_get_inner_ptr
	}
	
	impl PtrOfDetectorParameters {
		pub fn as_raw_PtrOfDetectorParameters(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfDetectorParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::aruco::DetectorParametersTrait for PtrOfDetectorParameters {
		fn as_raw_DetectorParameters(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_DetectorParameters(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDictionary = core::Ptr::<crate::aruco::Dictionary>;
	
	ptr_extern! { crate::aruco::Dictionary,
		cv_PtrOfDictionary_delete, cv_PtrOfDictionary_get_inner_ptr
	}
	
	impl PtrOfDictionary {
		pub fn as_raw_PtrOfDictionary(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfDictionary(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::aruco::DictionaryTrait for PtrOfDictionary {
		fn as_raw_Dictionary(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Dictionary(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGridBoard = core::Ptr::<crate::aruco::GridBoard>;
	
	ptr_extern! { crate::aruco::GridBoard,
		cv_PtrOfGridBoard_delete, cv_PtrOfGridBoard_get_inner_ptr
	}
	
	impl PtrOfGridBoard {
		pub fn as_raw_PtrOfGridBoard(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfGridBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::aruco::BoardTrait for PtrOfGridBoard {
		fn as_raw_Board(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Board(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::aruco::GridBoardTrait for PtrOfGridBoard {
		fn as_raw_GridBoard(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_GridBoard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use aruco_types::*;

#[cfg(feature = "contrib")]
mod bgsegm_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBackgroundSubtractorGMG = core::Ptr::<dyn crate::bgsegm::BackgroundSubtractorGMG>;
	
	ptr_extern! { dyn crate::bgsegm::BackgroundSubtractorGMG,
		cv_PtrOfBackgroundSubtractorGMG_delete, cv_PtrOfBackgroundSubtractorGMG_get_inner_ptr
	}
	
	impl PtrOfBackgroundSubtractorGMG {
		pub fn as_raw_PtrOfBackgroundSubtractorGMG(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfBackgroundSubtractorGMG(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBackgroundSubtractorGMG {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorGMG {
		fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorGMG for PtrOfBackgroundSubtractorGMG {
		fn as_raw_BackgroundSubtractorGMG(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BackgroundSubtractorGMG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackgroundSubtractorMOG = core::Ptr::<dyn crate::bgsegm::BackgroundSubtractorMOG>;
	
	ptr_extern! { dyn crate::bgsegm::BackgroundSubtractorMOG,
		cv_PtrOfBackgroundSubtractorMOG_delete, cv_PtrOfBackgroundSubtractorMOG_get_inner_ptr
	}
	
	impl PtrOfBackgroundSubtractorMOG {
		pub fn as_raw_PtrOfBackgroundSubtractorMOG(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfBackgroundSubtractorMOG(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBackgroundSubtractorMOG {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorMOG {
		fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorMOG for PtrOfBackgroundSubtractorMOG {
		fn as_raw_BackgroundSubtractorMOG(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BackgroundSubtractorMOG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use bgsegm_types::*;

#[cfg(feature = "contrib")]
mod bioinspired_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfRetina = core::Ptr::<dyn crate::bioinspired::Retina>;
	
	ptr_extern! { dyn crate::bioinspired::Retina,
		cv_PtrOfRetina_delete, cv_PtrOfRetina_get_inner_ptr
	}
	
	impl PtrOfRetina {
		pub fn as_raw_PtrOfRetina(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfRetina(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfRetina {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::bioinspired::Retina for PtrOfRetina {
		fn as_raw_Retina(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Retina(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRetinaFastToneMapping = core::Ptr::<dyn crate::bioinspired::RetinaFastToneMapping>;
	
	ptr_extern! { dyn crate::bioinspired::RetinaFastToneMapping,
		cv_PtrOfRetinaFastToneMapping_delete, cv_PtrOfRetinaFastToneMapping_get_inner_ptr
	}
	
	impl PtrOfRetinaFastToneMapping {
		pub fn as_raw_PtrOfRetinaFastToneMapping(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfRetinaFastToneMapping(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfRetinaFastToneMapping {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::bioinspired::RetinaFastToneMapping for PtrOfRetinaFastToneMapping {
		fn as_raw_RetinaFastToneMapping(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_RetinaFastToneMapping(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTransientAreasSegmentationModule = core::Ptr::<dyn crate::bioinspired::TransientAreasSegmentationModule>;
	
	ptr_extern! { dyn crate::bioinspired::TransientAreasSegmentationModule,
		cv_PtrOfTransientAreasSegmentationModule_delete, cv_PtrOfTransientAreasSegmentationModule_get_inner_ptr
	}
	
	impl PtrOfTransientAreasSegmentationModule {
		pub fn as_raw_PtrOfTransientAreasSegmentationModule(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfTransientAreasSegmentationModule(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfTransientAreasSegmentationModule {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::bioinspired::TransientAreasSegmentationModule for PtrOfTransientAreasSegmentationModule {
		fn as_raw_TransientAreasSegmentationModule(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_TransientAreasSegmentationModule(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use bioinspired_types::*;

mod calib3d_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfStereoBM = core::Ptr::<dyn crate::calib3d::StereoBM>;
	
	ptr_extern! { dyn crate::calib3d::StereoBM,
		cv_PtrOfStereoBM_delete, cv_PtrOfStereoBM_get_inner_ptr
	}
	
	impl PtrOfStereoBM {
		pub fn as_raw_PtrOfStereoBM(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfStereoBM(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfStereoBM {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoBM for PtrOfStereoBM {
		fn as_raw_StereoBM(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_StereoBM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcher for PtrOfStereoBM {
		fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStereoSGBM = core::Ptr::<dyn crate::calib3d::StereoSGBM>;
	
	ptr_extern! { dyn crate::calib3d::StereoSGBM,
		cv_PtrOfStereoSGBM_delete, cv_PtrOfStereoSGBM_get_inner_ptr
	}
	
	impl PtrOfStereoSGBM {
		pub fn as_raw_PtrOfStereoSGBM(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfStereoSGBM(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfStereoSGBM {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcher for PtrOfStereoSGBM {
		fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoSGBM for PtrOfStereoSGBM {
		fn as_raw_StereoSGBM(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_StereoSGBM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
pub use calib3d_types::*;

mod core_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfConjGradSolver = core::Ptr::<dyn core::ConjGradSolver>;
	
	ptr_extern! { dyn core::ConjGradSolver,
		cv_PtrOfConjGradSolver_delete, cv_PtrOfConjGradSolver_get_inner_ptr
	}
	
	impl PtrOfConjGradSolver {
		pub fn as_raw_PtrOfConjGradSolver(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfConjGradSolver(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfConjGradSolver {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::ConjGradSolver for PtrOfConjGradSolver {
		fn as_raw_ConjGradSolver(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_ConjGradSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::MinProblemSolver for PtrOfConjGradSolver {
		fn as_raw_MinProblemSolver(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDownhillSolver = core::Ptr::<dyn core::DownhillSolver>;
	
	ptr_extern! { dyn core::DownhillSolver,
		cv_PtrOfDownhillSolver_delete, cv_PtrOfDownhillSolver_get_inner_ptr
	}
	
	impl PtrOfDownhillSolver {
		pub fn as_raw_PtrOfDownhillSolver(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfDownhillSolver(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfDownhillSolver {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::DownhillSolver for PtrOfDownhillSolver {
		fn as_raw_DownhillSolver(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_DownhillSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::MinProblemSolver for PtrOfDownhillSolver {
		fn as_raw_MinProblemSolver(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFormatted = core::Ptr::<dyn core::Formatted>;
	
	ptr_extern! { dyn core::Formatted,
		cv_PtrOfFormatted_delete, cv_PtrOfFormatted_get_inner_ptr
	}
	
	impl PtrOfFormatted {
		pub fn as_raw_PtrOfFormatted(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfFormatted(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::Formatted for PtrOfFormatted {
		fn as_raw_Formatted(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Formatted(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFormatter = core::Ptr::<dyn core::Formatter>;
	
	ptr_extern! { dyn core::Formatter,
		cv_PtrOfFormatter_delete, cv_PtrOfFormatter_get_inner_ptr
	}
	
	impl PtrOfFormatter {
		pub fn as_raw_PtrOfFormatter(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfFormatter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::Formatter for PtrOfFormatter {
		fn as_raw_Formatter(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Formatter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMinProblemSolver_Function = core::Ptr::<dyn core::MinProblemSolver_Function>;
	
	ptr_extern! { dyn core::MinProblemSolver_Function,
		cv_PtrOfMinProblemSolver_Function_delete, cv_PtrOfMinProblemSolver_Function_get_inner_ptr
	}
	
	impl PtrOfMinProblemSolver_Function {
		pub fn as_raw_PtrOfMinProblemSolver_Function(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfMinProblemSolver_Function(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::MinProblemSolver_Function for PtrOfMinProblemSolver_Function {
		fn as_raw_MinProblemSolver_Function(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_MinProblemSolver_Function(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfDMatch = core::Vector::<core::DMatch>;
	
	impl VectorOfDMatch {
		pub fn as_raw_VectorOfDMatch(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfDMatch(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::DMatch, *const c_void, *mut c_void,
		cv_VectorOfDMatch_new, cv_VectorOfDMatch_delete,
		cv_VectorOfDMatch_len, cv_VectorOfDMatch_is_empty,
		cv_VectorOfDMatch_capacity, cv_VectorOfDMatch_shrink_to_fit,
		cv_VectorOfDMatch_reserve, cv_VectorOfDMatch_remove,
		cv_VectorOfDMatch_swap, cv_VectorOfDMatch_clear,
		cv_VectorOfDMatch_get -> sys::Result<core::DMatch>,
		ret_map: 
	}
	vector_copy_non_bool! { core::DMatch, *const c_void,
		cv_VectorOfDMatch_data
	}
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::DMatch> {
		type Arg = core::DMatch;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: core::DMatch) {
			extern "C" { fn cv_VectorOfDMatch_push(instance: *mut c_void, val: *const core::DMatch); }
			unsafe { cv_VectorOfDMatch_push(self.as_raw_mut_VectorOfDMatch(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: core::DMatch) -> Result<()> {
			extern "C" { fn cv_VectorOfDMatch_insert(instance: *mut c_void, index: size_t, val: *const core::DMatch); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfDMatch_insert(self.as_raw_mut_VectorOfDMatch(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: core::DMatch) -> Result<()> {
			extern "C" { fn cv_VectorOfDMatch_set(instance: *mut c_void, index: size_t, val: *const core::DMatch); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfDMatch_set(self.as_raw_mut_VectorOfDMatch(), index, &val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: core::DMatch) {
			extern "C" { fn cv_VectorOfDMatch_set(instance: *mut c_void, index: size_t, val: *const core::DMatch); }
			cv_VectorOfDMatch_set(self.as_raw_mut_VectorOfDMatch(), index, &val)
		}
	}
	
	unsafe impl Send for core::Vector::<core::DMatch> {}
	
	pub type VectorOfKeyPoint = core::Vector::<core::KeyPoint>;
	
	impl VectorOfKeyPoint {
		pub fn as_raw_VectorOfKeyPoint(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfKeyPoint(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::KeyPoint, *const c_void, *mut c_void,
		cv_VectorOfKeyPoint_new, cv_VectorOfKeyPoint_delete,
		cv_VectorOfKeyPoint_len, cv_VectorOfKeyPoint_is_empty,
		cv_VectorOfKeyPoint_capacity, cv_VectorOfKeyPoint_shrink_to_fit,
		cv_VectorOfKeyPoint_reserve, cv_VectorOfKeyPoint_remove,
		cv_VectorOfKeyPoint_swap, cv_VectorOfKeyPoint_clear,
		cv_VectorOfKeyPoint_get -> sys::Result<core::KeyPoint>,
		ret_map: 
	}
	vector_copy_non_bool! { core::KeyPoint, *const c_void,
		cv_VectorOfKeyPoint_data
	}
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::KeyPoint> {
		type Arg = core::KeyPoint;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: core::KeyPoint) {
			extern "C" { fn cv_VectorOfKeyPoint_push(instance: *mut c_void, val: *const core::KeyPoint); }
			unsafe { cv_VectorOfKeyPoint_push(self.as_raw_mut_VectorOfKeyPoint(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: core::KeyPoint) -> Result<()> {
			extern "C" { fn cv_VectorOfKeyPoint_insert(instance: *mut c_void, index: size_t, val: *const core::KeyPoint); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfKeyPoint_insert(self.as_raw_mut_VectorOfKeyPoint(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: core::KeyPoint) -> Result<()> {
			extern "C" { fn cv_VectorOfKeyPoint_set(instance: *mut c_void, index: size_t, val: *const core::KeyPoint); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfKeyPoint_set(self.as_raw_mut_VectorOfKeyPoint(), index, &val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: core::KeyPoint) {
			extern "C" { fn cv_VectorOfKeyPoint_set(instance: *mut c_void, index: size_t, val: *const core::KeyPoint); }
			cv_VectorOfKeyPoint_set(self.as_raw_mut_VectorOfKeyPoint(), index, &val)
		}
	}
	
	unsafe impl Send for core::Vector::<core::KeyPoint> {}
	
	pub type VectorOfMat = core::Vector::<core::Mat>;
	
	impl VectorOfMat {
		pub fn as_raw_VectorOfMat(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfMat(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Mat, *const c_void, *mut c_void,
		cv_VectorOfMat_new, cv_VectorOfMat_delete,
		cv_VectorOfMat_len, cv_VectorOfMat_is_empty,
		cv_VectorOfMat_capacity, cv_VectorOfMat_shrink_to_fit,
		cv_VectorOfMat_reserve, cv_VectorOfMat_remove,
		cv_VectorOfMat_swap, cv_VectorOfMat_clear,
		cv_VectorOfMat_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { core::Mat::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { core::Mat }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Mat> {
		type Arg = core::Mat;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: core::Mat) {
			extern "C" { fn cv_VectorOfMat_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfMat_push(self.as_raw_mut_VectorOfMat(), val.as_raw_mut_Mat()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: core::Mat) -> Result<()> {
			extern "C" { fn cv_VectorOfMat_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfMat_insert(self.as_raw_mut_VectorOfMat(), index, val.as_raw_mut_Mat()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: core::Mat) -> Result<()> {
			extern "C" { fn cv_VectorOfMat_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfMat_set(self.as_raw_mut_VectorOfMat(), index, val.as_raw_mut_Mat()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: core::Mat) {
			extern "C" { fn cv_VectorOfMat_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfMat_set(self.as_raw_mut_VectorOfMat(), index, val.as_raw_mut_Mat())
		}
	}
	
	unsafe impl Send for core::Vector::<core::Mat> {}
	
	pub type VectorOfPlatformInfo = core::Vector::<core::PlatformInfo>;
	
	impl VectorOfPlatformInfo {
		pub fn as_raw_VectorOfPlatformInfo(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPlatformInfo(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::PlatformInfo, *const c_void, *mut c_void,
		cv_VectorOfPlatformInfo_new, cv_VectorOfPlatformInfo_delete,
		cv_VectorOfPlatformInfo_len, cv_VectorOfPlatformInfo_is_empty,
		cv_VectorOfPlatformInfo_capacity, cv_VectorOfPlatformInfo_shrink_to_fit,
		cv_VectorOfPlatformInfo_reserve, cv_VectorOfPlatformInfo_remove,
		cv_VectorOfPlatformInfo_swap, cv_VectorOfPlatformInfo_clear,
		cv_VectorOfPlatformInfo_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { core::PlatformInfo::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { core::PlatformInfo }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::PlatformInfo> {
		type Arg = core::PlatformInfo;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: core::PlatformInfo) {
			extern "C" { fn cv_VectorOfPlatformInfo_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfPlatformInfo_push(self.as_raw_mut_VectorOfPlatformInfo(), val.as_raw_mut_PlatformInfo()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: core::PlatformInfo) -> Result<()> {
			extern "C" { fn cv_VectorOfPlatformInfo_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfPlatformInfo_insert(self.as_raw_mut_VectorOfPlatformInfo(), index, val.as_raw_mut_PlatformInfo()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: core::PlatformInfo) -> Result<()> {
			extern "C" { fn cv_VectorOfPlatformInfo_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfPlatformInfo_set(self.as_raw_mut_VectorOfPlatformInfo(), index, val.as_raw_mut_PlatformInfo()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: core::PlatformInfo) {
			extern "C" { fn cv_VectorOfPlatformInfo_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfPlatformInfo_set(self.as_raw_mut_VectorOfPlatformInfo(), index, val.as_raw_mut_PlatformInfo())
		}
	}
	
	unsafe impl Send for core::Vector::<core::PlatformInfo> {}
	
	pub type VectorOfPoint = core::Vector::<core::Point>;
	
	impl VectorOfPoint {
		pub fn as_raw_VectorOfPoint(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point, *const c_void, *mut c_void,
		cv_VectorOfPoint_new, cv_VectorOfPoint_delete,
		cv_VectorOfPoint_len, cv_VectorOfPoint_is_empty,
		cv_VectorOfPoint_capacity, cv_VectorOfPoint_shrink_to_fit,
		cv_VectorOfPoint_reserve, cv_VectorOfPoint_remove,
		cv_VectorOfPoint_swap, cv_VectorOfPoint_clear,
		cv_VectorOfPoint_get -> sys::Result<core::Point>,
		ret_map: 
	}
	vector_copy_non_bool! { core::Point, *const c_void,
		cv_VectorOfPoint_data
	}
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Point> {
		type Arg = core::Point;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: core::Point) {
			extern "C" { fn cv_VectorOfPoint_push(instance: *mut c_void, val: *const core::Point); }
			unsafe { cv_VectorOfPoint_push(self.as_raw_mut_VectorOfPoint(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: core::Point) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint_insert(instance: *mut c_void, index: size_t, val: *const core::Point); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfPoint_insert(self.as_raw_mut_VectorOfPoint(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: core::Point) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint_set(instance: *mut c_void, index: size_t, val: *const core::Point); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfPoint_set(self.as_raw_mut_VectorOfPoint(), index, &val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: core::Point) {
			extern "C" { fn cv_VectorOfPoint_set(instance: *mut c_void, index: size_t, val: *const core::Point); }
			cv_VectorOfPoint_set(self.as_raw_mut_VectorOfPoint(), index, &val)
		}
	}
	
	unsafe impl Send for core::Vector::<core::Point> {}
	
	impl core::ToInputArray for VectorOfPoint {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint_input_array(self.as_raw_VectorOfPoint()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfPoint {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfPoint {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfPoint_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint_output_array(self.as_raw_mut_VectorOfPoint()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfPoint {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfPoint {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfPoint_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint_input_output_array(self.as_raw_mut_VectorOfPoint()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfPoint {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfPoint2d = core::Vector::<core::Point2d>;
	
	impl VectorOfPoint2d {
		pub fn as_raw_VectorOfPoint2d(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint2d(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point2d, *const c_void, *mut c_void,
		cv_VectorOfPoint2d_new, cv_VectorOfPoint2d_delete,
		cv_VectorOfPoint2d_len, cv_VectorOfPoint2d_is_empty,
		cv_VectorOfPoint2d_capacity, cv_VectorOfPoint2d_shrink_to_fit,
		cv_VectorOfPoint2d_reserve, cv_VectorOfPoint2d_remove,
		cv_VectorOfPoint2d_swap, cv_VectorOfPoint2d_clear,
		cv_VectorOfPoint2d_get -> sys::Result<core::Point2d>,
		ret_map: 
	}
	vector_copy_non_bool! { core::Point2d, *const c_void,
		cv_VectorOfPoint2d_data
	}
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Point2d> {
		type Arg = core::Point2d;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: core::Point2d) {
			extern "C" { fn cv_VectorOfPoint2d_push(instance: *mut c_void, val: *const core::Point2d); }
			unsafe { cv_VectorOfPoint2d_push(self.as_raw_mut_VectorOfPoint2d(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: core::Point2d) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint2d_insert(instance: *mut c_void, index: size_t, val: *const core::Point2d); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfPoint2d_insert(self.as_raw_mut_VectorOfPoint2d(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: core::Point2d) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint2d_set(instance: *mut c_void, index: size_t, val: *const core::Point2d); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfPoint2d_set(self.as_raw_mut_VectorOfPoint2d(), index, &val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: core::Point2d) {
			extern "C" { fn cv_VectorOfPoint2d_set(instance: *mut c_void, index: size_t, val: *const core::Point2d); }
			cv_VectorOfPoint2d_set(self.as_raw_mut_VectorOfPoint2d(), index, &val)
		}
	}
	
	unsafe impl Send for core::Vector::<core::Point2d> {}
	
	impl core::ToInputArray for VectorOfPoint2d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint2d_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint2d_input_array(self.as_raw_VectorOfPoint2d()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfPoint2d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfPoint2d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfPoint2d_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint2d_output_array(self.as_raw_mut_VectorOfPoint2d()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfPoint2d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfPoint2d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfPoint2d_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint2d_input_output_array(self.as_raw_mut_VectorOfPoint2d()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfPoint2d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfPoint2f = core::Vector::<core::Point2f>;
	
	impl VectorOfPoint2f {
		pub fn as_raw_VectorOfPoint2f(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint2f(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point2f, *const c_void, *mut c_void,
		cv_VectorOfPoint2f_new, cv_VectorOfPoint2f_delete,
		cv_VectorOfPoint2f_len, cv_VectorOfPoint2f_is_empty,
		cv_VectorOfPoint2f_capacity, cv_VectorOfPoint2f_shrink_to_fit,
		cv_VectorOfPoint2f_reserve, cv_VectorOfPoint2f_remove,
		cv_VectorOfPoint2f_swap, cv_VectorOfPoint2f_clear,
		cv_VectorOfPoint2f_get -> sys::Result<core::Point2f>,
		ret_map: 
	}
	vector_copy_non_bool! { core::Point2f, *const c_void,
		cv_VectorOfPoint2f_data
	}
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Point2f> {
		type Arg = core::Point2f;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: core::Point2f) {
			extern "C" { fn cv_VectorOfPoint2f_push(instance: *mut c_void, val: *const core::Point2f); }
			unsafe { cv_VectorOfPoint2f_push(self.as_raw_mut_VectorOfPoint2f(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: core::Point2f) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint2f_insert(instance: *mut c_void, index: size_t, val: *const core::Point2f); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfPoint2f_insert(self.as_raw_mut_VectorOfPoint2f(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: core::Point2f) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint2f_set(instance: *mut c_void, index: size_t, val: *const core::Point2f); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfPoint2f_set(self.as_raw_mut_VectorOfPoint2f(), index, &val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: core::Point2f) {
			extern "C" { fn cv_VectorOfPoint2f_set(instance: *mut c_void, index: size_t, val: *const core::Point2f); }
			cv_VectorOfPoint2f_set(self.as_raw_mut_VectorOfPoint2f(), index, &val)
		}
	}
	
	unsafe impl Send for core::Vector::<core::Point2f> {}
	
	impl core::ToInputArray for VectorOfPoint2f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint2f_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint2f_input_array(self.as_raw_VectorOfPoint2f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfPoint2f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfPoint2f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfPoint2f_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint2f_output_array(self.as_raw_mut_VectorOfPoint2f()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfPoint2f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfPoint2f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfPoint2f_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint2f_input_output_array(self.as_raw_mut_VectorOfPoint2f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfPoint2f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfPoint3d = core::Vector::<core::Point3d>;
	
	impl VectorOfPoint3d {
		pub fn as_raw_VectorOfPoint3d(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint3d(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point3d, *const c_void, *mut c_void,
		cv_VectorOfPoint3d_new, cv_VectorOfPoint3d_delete,
		cv_VectorOfPoint3d_len, cv_VectorOfPoint3d_is_empty,
		cv_VectorOfPoint3d_capacity, cv_VectorOfPoint3d_shrink_to_fit,
		cv_VectorOfPoint3d_reserve, cv_VectorOfPoint3d_remove,
		cv_VectorOfPoint3d_swap, cv_VectorOfPoint3d_clear,
		cv_VectorOfPoint3d_get -> sys::Result<core::Point3d>,
		ret_map: 
	}
	vector_copy_non_bool! { core::Point3d, *const c_void,
		cv_VectorOfPoint3d_data
	}
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Point3d> {
		type Arg = core::Point3d;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: core::Point3d) {
			extern "C" { fn cv_VectorOfPoint3d_push(instance: *mut c_void, val: *const core::Point3d); }
			unsafe { cv_VectorOfPoint3d_push(self.as_raw_mut_VectorOfPoint3d(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: core::Point3d) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint3d_insert(instance: *mut c_void, index: size_t, val: *const core::Point3d); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfPoint3d_insert(self.as_raw_mut_VectorOfPoint3d(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: core::Point3d) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint3d_set(instance: *mut c_void, index: size_t, val: *const core::Point3d); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfPoint3d_set(self.as_raw_mut_VectorOfPoint3d(), index, &val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: core::Point3d) {
			extern "C" { fn cv_VectorOfPoint3d_set(instance: *mut c_void, index: size_t, val: *const core::Point3d); }
			cv_VectorOfPoint3d_set(self.as_raw_mut_VectorOfPoint3d(), index, &val)
		}
	}
	
	unsafe impl Send for core::Vector::<core::Point3d> {}
	
	impl core::ToInputArray for VectorOfPoint3d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint3d_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint3d_input_array(self.as_raw_VectorOfPoint3d()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfPoint3d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfPoint3d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfPoint3d_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint3d_output_array(self.as_raw_mut_VectorOfPoint3d()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfPoint3d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfPoint3d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfPoint3d_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint3d_input_output_array(self.as_raw_mut_VectorOfPoint3d()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfPoint3d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfPoint3f = core::Vector::<core::Point3f>;
	
	impl VectorOfPoint3f {
		pub fn as_raw_VectorOfPoint3f(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint3f(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point3f, *const c_void, *mut c_void,
		cv_VectorOfPoint3f_new, cv_VectorOfPoint3f_delete,
		cv_VectorOfPoint3f_len, cv_VectorOfPoint3f_is_empty,
		cv_VectorOfPoint3f_capacity, cv_VectorOfPoint3f_shrink_to_fit,
		cv_VectorOfPoint3f_reserve, cv_VectorOfPoint3f_remove,
		cv_VectorOfPoint3f_swap, cv_VectorOfPoint3f_clear,
		cv_VectorOfPoint3f_get -> sys::Result<core::Point3f>,
		ret_map: 
	}
	vector_copy_non_bool! { core::Point3f, *const c_void,
		cv_VectorOfPoint3f_data
	}
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Point3f> {
		type Arg = core::Point3f;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: core::Point3f) {
			extern "C" { fn cv_VectorOfPoint3f_push(instance: *mut c_void, val: *const core::Point3f); }
			unsafe { cv_VectorOfPoint3f_push(self.as_raw_mut_VectorOfPoint3f(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: core::Point3f) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint3f_insert(instance: *mut c_void, index: size_t, val: *const core::Point3f); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfPoint3f_insert(self.as_raw_mut_VectorOfPoint3f(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: core::Point3f) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint3f_set(instance: *mut c_void, index: size_t, val: *const core::Point3f); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfPoint3f_set(self.as_raw_mut_VectorOfPoint3f(), index, &val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: core::Point3f) {
			extern "C" { fn cv_VectorOfPoint3f_set(instance: *mut c_void, index: size_t, val: *const core::Point3f); }
			cv_VectorOfPoint3f_set(self.as_raw_mut_VectorOfPoint3f(), index, &val)
		}
	}
	
	unsafe impl Send for core::Vector::<core::Point3f> {}
	
	impl core::ToInputArray for VectorOfPoint3f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint3f_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint3f_input_array(self.as_raw_VectorOfPoint3f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfPoint3f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfPoint3f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfPoint3f_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint3f_output_array(self.as_raw_mut_VectorOfPoint3f()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfPoint3f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfPoint3f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfPoint3f_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint3f_input_output_array(self.as_raw_mut_VectorOfPoint3f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfPoint3f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfPoint3i = core::Vector::<core::Point3i>;
	
	impl VectorOfPoint3i {
		pub fn as_raw_VectorOfPoint3i(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint3i(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point3i, *const c_void, *mut c_void,
		cv_VectorOfPoint3i_new, cv_VectorOfPoint3i_delete,
		cv_VectorOfPoint3i_len, cv_VectorOfPoint3i_is_empty,
		cv_VectorOfPoint3i_capacity, cv_VectorOfPoint3i_shrink_to_fit,
		cv_VectorOfPoint3i_reserve, cv_VectorOfPoint3i_remove,
		cv_VectorOfPoint3i_swap, cv_VectorOfPoint3i_clear,
		cv_VectorOfPoint3i_get -> sys::Result<core::Point3i>,
		ret_map: 
	}
	vector_copy_non_bool! { core::Point3i, *const c_void,
		cv_VectorOfPoint3i_data
	}
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Point3i> {
		type Arg = core::Point3i;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: core::Point3i) {
			extern "C" { fn cv_VectorOfPoint3i_push(instance: *mut c_void, val: *const core::Point3i); }
			unsafe { cv_VectorOfPoint3i_push(self.as_raw_mut_VectorOfPoint3i(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: core::Point3i) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint3i_insert(instance: *mut c_void, index: size_t, val: *const core::Point3i); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfPoint3i_insert(self.as_raw_mut_VectorOfPoint3i(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: core::Point3i) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint3i_set(instance: *mut c_void, index: size_t, val: *const core::Point3i); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfPoint3i_set(self.as_raw_mut_VectorOfPoint3i(), index, &val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: core::Point3i) {
			extern "C" { fn cv_VectorOfPoint3i_set(instance: *mut c_void, index: size_t, val: *const core::Point3i); }
			cv_VectorOfPoint3i_set(self.as_raw_mut_VectorOfPoint3i(), index, &val)
		}
	}
	
	unsafe impl Send for core::Vector::<core::Point3i> {}
	
	impl core::ToInputArray for VectorOfPoint3i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint3i_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint3i_input_array(self.as_raw_VectorOfPoint3i()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfPoint3i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfPoint3i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfPoint3i_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint3i_output_array(self.as_raw_mut_VectorOfPoint3i()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfPoint3i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfPoint3i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfPoint3i_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint3i_input_output_array(self.as_raw_mut_VectorOfPoint3i()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfPoint3i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfRange = core::Vector::<core::Range>;
	
	impl VectorOfRange {
		pub fn as_raw_VectorOfRange(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfRange(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Range, *const c_void, *mut c_void,
		cv_VectorOfRange_new, cv_VectorOfRange_delete,
		cv_VectorOfRange_len, cv_VectorOfRange_is_empty,
		cv_VectorOfRange_capacity, cv_VectorOfRange_shrink_to_fit,
		cv_VectorOfRange_reserve, cv_VectorOfRange_remove,
		cv_VectorOfRange_swap, cv_VectorOfRange_clear,
		cv_VectorOfRange_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { core::Range::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { core::Range }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Range> {
		type Arg = core::Range;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: core::Range) {
			extern "C" { fn cv_VectorOfRange_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfRange_push(self.as_raw_mut_VectorOfRange(), val.as_raw_mut_Range()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: core::Range) -> Result<()> {
			extern "C" { fn cv_VectorOfRange_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfRange_insert(self.as_raw_mut_VectorOfRange(), index, val.as_raw_mut_Range()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: core::Range) -> Result<()> {
			extern "C" { fn cv_VectorOfRange_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfRange_set(self.as_raw_mut_VectorOfRange(), index, val.as_raw_mut_Range()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: core::Range) {
			extern "C" { fn cv_VectorOfRange_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfRange_set(self.as_raw_mut_VectorOfRange(), index, val.as_raw_mut_Range())
		}
	}
	
	unsafe impl Send for core::Vector::<core::Range> {}
	
	pub type VectorOfRect = core::Vector::<core::Rect>;
	
	impl VectorOfRect {
		pub fn as_raw_VectorOfRect(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfRect(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Rect, *const c_void, *mut c_void,
		cv_VectorOfRect_new, cv_VectorOfRect_delete,
		cv_VectorOfRect_len, cv_VectorOfRect_is_empty,
		cv_VectorOfRect_capacity, cv_VectorOfRect_shrink_to_fit,
		cv_VectorOfRect_reserve, cv_VectorOfRect_remove,
		cv_VectorOfRect_swap, cv_VectorOfRect_clear,
		cv_VectorOfRect_get -> sys::Result<core::Rect>,
		ret_map: 
	}
	vector_copy_non_bool! { core::Rect, *const c_void,
		cv_VectorOfRect_data
	}
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Rect> {
		type Arg = core::Rect;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: core::Rect) {
			extern "C" { fn cv_VectorOfRect_push(instance: *mut c_void, val: *const core::Rect); }
			unsafe { cv_VectorOfRect_push(self.as_raw_mut_VectorOfRect(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: core::Rect) -> Result<()> {
			extern "C" { fn cv_VectorOfRect_insert(instance: *mut c_void, index: size_t, val: *const core::Rect); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfRect_insert(self.as_raw_mut_VectorOfRect(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: core::Rect) -> Result<()> {
			extern "C" { fn cv_VectorOfRect_set(instance: *mut c_void, index: size_t, val: *const core::Rect); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfRect_set(self.as_raw_mut_VectorOfRect(), index, &val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: core::Rect) {
			extern "C" { fn cv_VectorOfRect_set(instance: *mut c_void, index: size_t, val: *const core::Rect); }
			cv_VectorOfRect_set(self.as_raw_mut_VectorOfRect(), index, &val)
		}
	}
	
	unsafe impl Send for core::Vector::<core::Rect> {}
	
	impl core::ToInputArray for VectorOfRect {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfRect_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfRect_input_array(self.as_raw_VectorOfRect()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfRect {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfRect {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfRect_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfRect_output_array(self.as_raw_mut_VectorOfRect()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfRect {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfRect {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfRect_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfRect_input_output_array(self.as_raw_mut_VectorOfRect()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfRect {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfSize = core::Vector::<core::Size>;
	
	impl VectorOfSize {
		pub fn as_raw_VectorOfSize(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfSize(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Size, *const c_void, *mut c_void,
		cv_VectorOfSize_new, cv_VectorOfSize_delete,
		cv_VectorOfSize_len, cv_VectorOfSize_is_empty,
		cv_VectorOfSize_capacity, cv_VectorOfSize_shrink_to_fit,
		cv_VectorOfSize_reserve, cv_VectorOfSize_remove,
		cv_VectorOfSize_swap, cv_VectorOfSize_clear,
		cv_VectorOfSize_get -> sys::Result<core::Size>,
		ret_map: 
	}
	vector_copy_non_bool! { core::Size, *const c_void,
		cv_VectorOfSize_data
	}
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Size> {
		type Arg = core::Size;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: core::Size) {
			extern "C" { fn cv_VectorOfSize_push(instance: *mut c_void, val: *const core::Size); }
			unsafe { cv_VectorOfSize_push(self.as_raw_mut_VectorOfSize(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: core::Size) -> Result<()> {
			extern "C" { fn cv_VectorOfSize_insert(instance: *mut c_void, index: size_t, val: *const core::Size); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfSize_insert(self.as_raw_mut_VectorOfSize(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: core::Size) -> Result<()> {
			extern "C" { fn cv_VectorOfSize_set(instance: *mut c_void, index: size_t, val: *const core::Size); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfSize_set(self.as_raw_mut_VectorOfSize(), index, &val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: core::Size) {
			extern "C" { fn cv_VectorOfSize_set(instance: *mut c_void, index: size_t, val: *const core::Size); }
			cv_VectorOfSize_set(self.as_raw_mut_VectorOfSize(), index, &val)
		}
	}
	
	unsafe impl Send for core::Vector::<core::Size> {}
	
	impl core::ToInputArray for VectorOfSize {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfSize_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfSize_input_array(self.as_raw_VectorOfSize()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfSize {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfSize {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfSize_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfSize_output_array(self.as_raw_mut_VectorOfSize()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfSize {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfSize {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfSize_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfSize_input_output_array(self.as_raw_mut_VectorOfSize()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfSize {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfString = core::Vector::<String>;
	
	impl VectorOfString {
		pub fn as_raw_VectorOfString(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfString(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { String, *const c_void, *mut c_void,
		cv_VectorOfString_new, cv_VectorOfString_delete,
		cv_VectorOfString_len, cv_VectorOfString_is_empty,
		cv_VectorOfString_capacity, cv_VectorOfString_shrink_to_fit,
		cv_VectorOfString_reserve, cv_VectorOfString_remove,
		cv_VectorOfString_swap, cv_VectorOfString_clear,
		cv_VectorOfString_get -> sys::Result<*mut c_void>,
		ret_map: .map(|s| { crate::templ::receive_string(s as *mut String) })
	}
	vector_non_copy_or_bool! { String }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<String> {
		type Arg = &'i str;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: &str) {
			extern "C" { fn cv_VectorOfString_push(instance: *mut c_void, val: *mut c_char); }
			string_arg_infallible!(val);
			unsafe { cv_VectorOfString_push(self.as_raw_mut_VectorOfString(), val.as_ptr() as _) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: &str) -> Result<()> {
			extern "C" { fn cv_VectorOfString_insert(instance: *mut c_void, index: size_t, val: *mut c_char); }
			core::vector_index_check(index, self.len() + 1)?;
			string_arg!(val);
			unsafe { cv_VectorOfString_insert(self.as_raw_mut_VectorOfString(), index, val.as_ptr() as _) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: &str) -> Result<()> {
			extern "C" { fn cv_VectorOfString_set(instance: *mut c_void, index: size_t, val: *mut c_char); }
			core::vector_index_check(index, self.len())?;
			string_arg!(val);
			unsafe { cv_VectorOfString_set(self.as_raw_mut_VectorOfString(), index, val.as_ptr() as _) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: &str) {
			extern "C" { fn cv_VectorOfString_set(instance: *mut c_void, index: size_t, val: *mut c_char); }
			string_arg_infallible!(val);
			cv_VectorOfString_set(self.as_raw_mut_VectorOfString(), index, val.as_ptr() as _)
		}
	}
	
	unsafe impl Send for core::Vector::<String> {}
	
	pub type VectorOfUMat = core::Vector::<core::UMat>;
	
	impl VectorOfUMat {
		pub fn as_raw_VectorOfUMat(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfUMat(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::UMat, *const c_void, *mut c_void,
		cv_VectorOfUMat_new, cv_VectorOfUMat_delete,
		cv_VectorOfUMat_len, cv_VectorOfUMat_is_empty,
		cv_VectorOfUMat_capacity, cv_VectorOfUMat_shrink_to_fit,
		cv_VectorOfUMat_reserve, cv_VectorOfUMat_remove,
		cv_VectorOfUMat_swap, cv_VectorOfUMat_clear,
		cv_VectorOfUMat_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { core::UMat::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { core::UMat }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::UMat> {
		type Arg = core::UMat;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: core::UMat) {
			extern "C" { fn cv_VectorOfUMat_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfUMat_push(self.as_raw_mut_VectorOfUMat(), val.as_raw_mut_UMat()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: core::UMat) -> Result<()> {
			extern "C" { fn cv_VectorOfUMat_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfUMat_insert(self.as_raw_mut_VectorOfUMat(), index, val.as_raw_mut_UMat()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: core::UMat) -> Result<()> {
			extern "C" { fn cv_VectorOfUMat_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfUMat_set(self.as_raw_mut_VectorOfUMat(), index, val.as_raw_mut_UMat()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: core::UMat) {
			extern "C" { fn cv_VectorOfUMat_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfUMat_set(self.as_raw_mut_VectorOfUMat(), index, val.as_raw_mut_UMat())
		}
	}
	
	unsafe impl Send for core::Vector::<core::UMat> {}
	
	pub type VectorOfVec2i = core::Vector::<core::Vec2i>;
	
	impl VectorOfVec2i {
		pub fn as_raw_VectorOfVec2i(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec2i(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec2i, *const c_void, *mut c_void,
		cv_VectorOfVec2i_new, cv_VectorOfVec2i_delete,
		cv_VectorOfVec2i_len, cv_VectorOfVec2i_is_empty,
		cv_VectorOfVec2i_capacity, cv_VectorOfVec2i_shrink_to_fit,
		cv_VectorOfVec2i_reserve, cv_VectorOfVec2i_remove,
		cv_VectorOfVec2i_swap, cv_VectorOfVec2i_clear,
		cv_VectorOfVec2i_get -> sys::Result<core::Vec2i>,
		ret_map: 
	}
	vector_copy_non_bool! { core::Vec2i, *const c_void,
		cv_VectorOfVec2i_data
	}
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Vec2i> {
		type Arg = core::Vec2i;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: core::Vec2i) {
			extern "C" { fn cv_VectorOfVec2i_push(instance: *mut c_void, val: *const core::Vec2i); }
			unsafe { cv_VectorOfVec2i_push(self.as_raw_mut_VectorOfVec2i(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: core::Vec2i) -> Result<()> {
			extern "C" { fn cv_VectorOfVec2i_insert(instance: *mut c_void, index: size_t, val: *const core::Vec2i); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVec2i_insert(self.as_raw_mut_VectorOfVec2i(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: core::Vec2i) -> Result<()> {
			extern "C" { fn cv_VectorOfVec2i_set(instance: *mut c_void, index: size_t, val: *const core::Vec2i); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfVec2i_set(self.as_raw_mut_VectorOfVec2i(), index, &val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: core::Vec2i) {
			extern "C" { fn cv_VectorOfVec2i_set(instance: *mut c_void, index: size_t, val: *const core::Vec2i); }
			cv_VectorOfVec2i_set(self.as_raw_mut_VectorOfVec2i(), index, &val)
		}
	}
	
	unsafe impl Send for core::Vector::<core::Vec2i> {}
	
	impl core::ToInputArray for VectorOfVec2i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec2i_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec2i_input_array(self.as_raw_VectorOfVec2i()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVec2i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVec2i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVec2i_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec2i_output_array(self.as_raw_mut_VectorOfVec2i()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVec2i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVec2i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVec2i_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec2i_input_output_array(self.as_raw_mut_VectorOfVec2i()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVec2i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVec3d = core::Vector::<core::Vec3d>;
	
	impl VectorOfVec3d {
		pub fn as_raw_VectorOfVec3d(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec3d(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec3d, *const c_void, *mut c_void,
		cv_VectorOfVec3d_new, cv_VectorOfVec3d_delete,
		cv_VectorOfVec3d_len, cv_VectorOfVec3d_is_empty,
		cv_VectorOfVec3d_capacity, cv_VectorOfVec3d_shrink_to_fit,
		cv_VectorOfVec3d_reserve, cv_VectorOfVec3d_remove,
		cv_VectorOfVec3d_swap, cv_VectorOfVec3d_clear,
		cv_VectorOfVec3d_get -> sys::Result<core::Vec3d>,
		ret_map: 
	}
	vector_copy_non_bool! { core::Vec3d, *const c_void,
		cv_VectorOfVec3d_data
	}
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Vec3d> {
		type Arg = core::Vec3d;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: core::Vec3d) {
			extern "C" { fn cv_VectorOfVec3d_push(instance: *mut c_void, val: *const core::Vec3d); }
			unsafe { cv_VectorOfVec3d_push(self.as_raw_mut_VectorOfVec3d(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: core::Vec3d) -> Result<()> {
			extern "C" { fn cv_VectorOfVec3d_insert(instance: *mut c_void, index: size_t, val: *const core::Vec3d); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVec3d_insert(self.as_raw_mut_VectorOfVec3d(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: core::Vec3d) -> Result<()> {
			extern "C" { fn cv_VectorOfVec3d_set(instance: *mut c_void, index: size_t, val: *const core::Vec3d); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfVec3d_set(self.as_raw_mut_VectorOfVec3d(), index, &val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: core::Vec3d) {
			extern "C" { fn cv_VectorOfVec3d_set(instance: *mut c_void, index: size_t, val: *const core::Vec3d); }
			cv_VectorOfVec3d_set(self.as_raw_mut_VectorOfVec3d(), index, &val)
		}
	}
	
	unsafe impl Send for core::Vector::<core::Vec3d> {}
	
	impl core::ToInputArray for VectorOfVec3d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec3d_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec3d_input_array(self.as_raw_VectorOfVec3d()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVec3d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVec3d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVec3d_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec3d_output_array(self.as_raw_mut_VectorOfVec3d()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVec3d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVec3d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVec3d_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec3d_input_output_array(self.as_raw_mut_VectorOfVec3d()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVec3d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVec3f = core::Vector::<core::Vec3f>;
	
	impl VectorOfVec3f {
		pub fn as_raw_VectorOfVec3f(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec3f(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec3f, *const c_void, *mut c_void,
		cv_VectorOfVec3f_new, cv_VectorOfVec3f_delete,
		cv_VectorOfVec3f_len, cv_VectorOfVec3f_is_empty,
		cv_VectorOfVec3f_capacity, cv_VectorOfVec3f_shrink_to_fit,
		cv_VectorOfVec3f_reserve, cv_VectorOfVec3f_remove,
		cv_VectorOfVec3f_swap, cv_VectorOfVec3f_clear,
		cv_VectorOfVec3f_get -> sys::Result<core::Vec3f>,
		ret_map: 
	}
	vector_copy_non_bool! { core::Vec3f, *const c_void,
		cv_VectorOfVec3f_data
	}
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Vec3f> {
		type Arg = core::Vec3f;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: core::Vec3f) {
			extern "C" { fn cv_VectorOfVec3f_push(instance: *mut c_void, val: *const core::Vec3f); }
			unsafe { cv_VectorOfVec3f_push(self.as_raw_mut_VectorOfVec3f(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: core::Vec3f) -> Result<()> {
			extern "C" { fn cv_VectorOfVec3f_insert(instance: *mut c_void, index: size_t, val: *const core::Vec3f); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVec3f_insert(self.as_raw_mut_VectorOfVec3f(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: core::Vec3f) -> Result<()> {
			extern "C" { fn cv_VectorOfVec3f_set(instance: *mut c_void, index: size_t, val: *const core::Vec3f); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfVec3f_set(self.as_raw_mut_VectorOfVec3f(), index, &val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: core::Vec3f) {
			extern "C" { fn cv_VectorOfVec3f_set(instance: *mut c_void, index: size_t, val: *const core::Vec3f); }
			cv_VectorOfVec3f_set(self.as_raw_mut_VectorOfVec3f(), index, &val)
		}
	}
	
	unsafe impl Send for core::Vector::<core::Vec3f> {}
	
	impl core::ToInputArray for VectorOfVec3f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec3f_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec3f_input_array(self.as_raw_VectorOfVec3f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVec3f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVec3f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVec3f_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec3f_output_array(self.as_raw_mut_VectorOfVec3f()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVec3f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVec3f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVec3f_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec3f_input_output_array(self.as_raw_mut_VectorOfVec3f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVec3f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVec4f = core::Vector::<core::Vec4f>;
	
	impl VectorOfVec4f {
		pub fn as_raw_VectorOfVec4f(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec4f(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec4f, *const c_void, *mut c_void,
		cv_VectorOfVec4f_new, cv_VectorOfVec4f_delete,
		cv_VectorOfVec4f_len, cv_VectorOfVec4f_is_empty,
		cv_VectorOfVec4f_capacity, cv_VectorOfVec4f_shrink_to_fit,
		cv_VectorOfVec4f_reserve, cv_VectorOfVec4f_remove,
		cv_VectorOfVec4f_swap, cv_VectorOfVec4f_clear,
		cv_VectorOfVec4f_get -> sys::Result<core::Vec4f>,
		ret_map: 
	}
	vector_copy_non_bool! { core::Vec4f, *const c_void,
		cv_VectorOfVec4f_data
	}
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Vec4f> {
		type Arg = core::Vec4f;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: core::Vec4f) {
			extern "C" { fn cv_VectorOfVec4f_push(instance: *mut c_void, val: *const core::Vec4f); }
			unsafe { cv_VectorOfVec4f_push(self.as_raw_mut_VectorOfVec4f(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: core::Vec4f) -> Result<()> {
			extern "C" { fn cv_VectorOfVec4f_insert(instance: *mut c_void, index: size_t, val: *const core::Vec4f); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVec4f_insert(self.as_raw_mut_VectorOfVec4f(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: core::Vec4f) -> Result<()> {
			extern "C" { fn cv_VectorOfVec4f_set(instance: *mut c_void, index: size_t, val: *const core::Vec4f); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfVec4f_set(self.as_raw_mut_VectorOfVec4f(), index, &val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: core::Vec4f) {
			extern "C" { fn cv_VectorOfVec4f_set(instance: *mut c_void, index: size_t, val: *const core::Vec4f); }
			cv_VectorOfVec4f_set(self.as_raw_mut_VectorOfVec4f(), index, &val)
		}
	}
	
	unsafe impl Send for core::Vector::<core::Vec4f> {}
	
	impl core::ToInputArray for VectorOfVec4f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec4f_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec4f_input_array(self.as_raw_VectorOfVec4f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVec4f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVec4f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVec4f_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec4f_output_array(self.as_raw_mut_VectorOfVec4f()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVec4f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVec4f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVec4f_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec4f_input_output_array(self.as_raw_mut_VectorOfVec4f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVec4f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVec4i = core::Vector::<core::Vec4i>;
	
	impl VectorOfVec4i {
		pub fn as_raw_VectorOfVec4i(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec4i(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec4i, *const c_void, *mut c_void,
		cv_VectorOfVec4i_new, cv_VectorOfVec4i_delete,
		cv_VectorOfVec4i_len, cv_VectorOfVec4i_is_empty,
		cv_VectorOfVec4i_capacity, cv_VectorOfVec4i_shrink_to_fit,
		cv_VectorOfVec4i_reserve, cv_VectorOfVec4i_remove,
		cv_VectorOfVec4i_swap, cv_VectorOfVec4i_clear,
		cv_VectorOfVec4i_get -> sys::Result<core::Vec4i>,
		ret_map: 
	}
	vector_copy_non_bool! { core::Vec4i, *const c_void,
		cv_VectorOfVec4i_data
	}
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Vec4i> {
		type Arg = core::Vec4i;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: core::Vec4i) {
			extern "C" { fn cv_VectorOfVec4i_push(instance: *mut c_void, val: *const core::Vec4i); }
			unsafe { cv_VectorOfVec4i_push(self.as_raw_mut_VectorOfVec4i(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: core::Vec4i) -> Result<()> {
			extern "C" { fn cv_VectorOfVec4i_insert(instance: *mut c_void, index: size_t, val: *const core::Vec4i); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVec4i_insert(self.as_raw_mut_VectorOfVec4i(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: core::Vec4i) -> Result<()> {
			extern "C" { fn cv_VectorOfVec4i_set(instance: *mut c_void, index: size_t, val: *const core::Vec4i); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfVec4i_set(self.as_raw_mut_VectorOfVec4i(), index, &val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: core::Vec4i) {
			extern "C" { fn cv_VectorOfVec4i_set(instance: *mut c_void, index: size_t, val: *const core::Vec4i); }
			cv_VectorOfVec4i_set(self.as_raw_mut_VectorOfVec4i(), index, &val)
		}
	}
	
	unsafe impl Send for core::Vector::<core::Vec4i> {}
	
	impl core::ToInputArray for VectorOfVec4i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec4i_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec4i_input_array(self.as_raw_VectorOfVec4i()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVec4i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVec4i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVec4i_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec4i_output_array(self.as_raw_mut_VectorOfVec4i()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVec4i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVec4i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVec4i_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec4i_input_output_array(self.as_raw_mut_VectorOfVec4i()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVec4i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVec6f = core::Vector::<core::Vec6f>;
	
	impl VectorOfVec6f {
		pub fn as_raw_VectorOfVec6f(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec6f(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec6f, *const c_void, *mut c_void,
		cv_VectorOfVec6f_new, cv_VectorOfVec6f_delete,
		cv_VectorOfVec6f_len, cv_VectorOfVec6f_is_empty,
		cv_VectorOfVec6f_capacity, cv_VectorOfVec6f_shrink_to_fit,
		cv_VectorOfVec6f_reserve, cv_VectorOfVec6f_remove,
		cv_VectorOfVec6f_swap, cv_VectorOfVec6f_clear,
		cv_VectorOfVec6f_get -> sys::Result<core::Vec6f>,
		ret_map: 
	}
	vector_copy_non_bool! { core::Vec6f, *const c_void,
		cv_VectorOfVec6f_data
	}
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Vec6f> {
		type Arg = core::Vec6f;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: core::Vec6f) {
			extern "C" { fn cv_VectorOfVec6f_push(instance: *mut c_void, val: *const core::Vec6f); }
			unsafe { cv_VectorOfVec6f_push(self.as_raw_mut_VectorOfVec6f(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: core::Vec6f) -> Result<()> {
			extern "C" { fn cv_VectorOfVec6f_insert(instance: *mut c_void, index: size_t, val: *const core::Vec6f); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVec6f_insert(self.as_raw_mut_VectorOfVec6f(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: core::Vec6f) -> Result<()> {
			extern "C" { fn cv_VectorOfVec6f_set(instance: *mut c_void, index: size_t, val: *const core::Vec6f); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfVec6f_set(self.as_raw_mut_VectorOfVec6f(), index, &val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: core::Vec6f) {
			extern "C" { fn cv_VectorOfVec6f_set(instance: *mut c_void, index: size_t, val: *const core::Vec6f); }
			cv_VectorOfVec6f_set(self.as_raw_mut_VectorOfVec6f(), index, &val)
		}
	}
	
	unsafe impl Send for core::Vector::<core::Vec6f> {}
	
	impl core::ToInputArray for VectorOfVec6f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec6f_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec6f_input_array(self.as_raw_VectorOfVec6f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVec6f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVec6f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVec6f_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec6f_output_array(self.as_raw_mut_VectorOfVec6f()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVec6f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVec6f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVec6f_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec6f_input_output_array(self.as_raw_mut_VectorOfVec6f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVec6f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVectorOfDMatch = core::Vector::<core::Vector::<core::DMatch>>;
	
	impl VectorOfVectorOfDMatch {
		pub fn as_raw_VectorOfVectorOfDMatch(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfDMatch(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<core::DMatch>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfDMatch_new, cv_VectorOfVectorOfDMatch_delete,
		cv_VectorOfVectorOfDMatch_len, cv_VectorOfVectorOfDMatch_is_empty,
		cv_VectorOfVectorOfDMatch_capacity, cv_VectorOfVectorOfDMatch_shrink_to_fit,
		cv_VectorOfVectorOfDMatch_reserve, cv_VectorOfVectorOfDMatch_remove,
		cv_VectorOfVectorOfDMatch_swap, cv_VectorOfVectorOfDMatch_clear,
		cv_VectorOfVectorOfDMatch_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { core::Vector::<core::DMatch>::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { core::Vector::<core::DMatch> }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Vector::<core::DMatch>> {
		type Arg = core::Vector::<core::DMatch>;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: core::Vector::<core::DMatch>) {
			extern "C" { fn cv_VectorOfVectorOfDMatch_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfDMatch_push(self.as_raw_mut_VectorOfVectorOfDMatch(), val.as_raw_mut_VectorOfDMatch()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: core::Vector::<core::DMatch>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfDMatch_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfDMatch_insert(self.as_raw_mut_VectorOfVectorOfDMatch(), index, val.as_raw_mut_VectorOfDMatch()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: core::Vector::<core::DMatch>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfDMatch_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfVectorOfDMatch_set(self.as_raw_mut_VectorOfVectorOfDMatch(), index, val.as_raw_mut_VectorOfDMatch()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: core::Vector::<core::DMatch>) {
			extern "C" { fn cv_VectorOfVectorOfDMatch_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfDMatch_set(self.as_raw_mut_VectorOfVectorOfDMatch(), index, val.as_raw_mut_VectorOfDMatch())
		}
	}
	
	unsafe impl Send for core::Vector::<core::Vector::<core::DMatch>> {}
	
	pub type VectorOfVectorOfKeyPoint = core::Vector::<core::Vector::<core::KeyPoint>>;
	
	impl VectorOfVectorOfKeyPoint {
		pub fn as_raw_VectorOfVectorOfKeyPoint(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfKeyPoint(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<core::KeyPoint>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfKeyPoint_new, cv_VectorOfVectorOfKeyPoint_delete,
		cv_VectorOfVectorOfKeyPoint_len, cv_VectorOfVectorOfKeyPoint_is_empty,
		cv_VectorOfVectorOfKeyPoint_capacity, cv_VectorOfVectorOfKeyPoint_shrink_to_fit,
		cv_VectorOfVectorOfKeyPoint_reserve, cv_VectorOfVectorOfKeyPoint_remove,
		cv_VectorOfVectorOfKeyPoint_swap, cv_VectorOfVectorOfKeyPoint_clear,
		cv_VectorOfVectorOfKeyPoint_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { core::Vector::<core::KeyPoint>::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { core::Vector::<core::KeyPoint> }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Vector::<core::KeyPoint>> {
		type Arg = core::Vector::<core::KeyPoint>;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: core::Vector::<core::KeyPoint>) {
			extern "C" { fn cv_VectorOfVectorOfKeyPoint_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfKeyPoint_push(self.as_raw_mut_VectorOfVectorOfKeyPoint(), val.as_raw_mut_VectorOfKeyPoint()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: core::Vector::<core::KeyPoint>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfKeyPoint_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfKeyPoint_insert(self.as_raw_mut_VectorOfVectorOfKeyPoint(), index, val.as_raw_mut_VectorOfKeyPoint()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: core::Vector::<core::KeyPoint>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfKeyPoint_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfVectorOfKeyPoint_set(self.as_raw_mut_VectorOfVectorOfKeyPoint(), index, val.as_raw_mut_VectorOfKeyPoint()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: core::Vector::<core::KeyPoint>) {
			extern "C" { fn cv_VectorOfVectorOfKeyPoint_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfKeyPoint_set(self.as_raw_mut_VectorOfVectorOfKeyPoint(), index, val.as_raw_mut_VectorOfKeyPoint())
		}
	}
	
	unsafe impl Send for core::Vector::<core::Vector::<core::KeyPoint>> {}
	
	pub type VectorOfVectorOfPoint = core::Vector::<core::Vector::<core::Point>>;
	
	impl VectorOfVectorOfPoint {
		pub fn as_raw_VectorOfVectorOfPoint(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<core::Point>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfPoint_new, cv_VectorOfVectorOfPoint_delete,
		cv_VectorOfVectorOfPoint_len, cv_VectorOfVectorOfPoint_is_empty,
		cv_VectorOfVectorOfPoint_capacity, cv_VectorOfVectorOfPoint_shrink_to_fit,
		cv_VectorOfVectorOfPoint_reserve, cv_VectorOfVectorOfPoint_remove,
		cv_VectorOfVectorOfPoint_swap, cv_VectorOfVectorOfPoint_clear,
		cv_VectorOfVectorOfPoint_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { core::Vector::<core::Point>::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { core::Vector::<core::Point> }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Vector::<core::Point>> {
		type Arg = core::Vector::<core::Point>;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: core::Vector::<core::Point>) {
			extern "C" { fn cv_VectorOfVectorOfPoint_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint_push(self.as_raw_mut_VectorOfVectorOfPoint(), val.as_raw_mut_VectorOfPoint()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: core::Vector::<core::Point>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfPoint_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfPoint_insert(self.as_raw_mut_VectorOfVectorOfPoint(), index, val.as_raw_mut_VectorOfPoint()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: core::Vector::<core::Point>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfPoint_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfVectorOfPoint_set(self.as_raw_mut_VectorOfVectorOfPoint(), index, val.as_raw_mut_VectorOfPoint()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: core::Vector::<core::Point>) {
			extern "C" { fn cv_VectorOfVectorOfPoint_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfPoint_set(self.as_raw_mut_VectorOfVectorOfPoint(), index, val.as_raw_mut_VectorOfPoint())
		}
	}
	
	unsafe impl Send for core::Vector::<core::Vector::<core::Point>> {}
	
	impl core::ToInputArray for VectorOfVectorOfPoint {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint_input_array(self.as_raw_VectorOfVectorOfPoint()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVectorOfPoint {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfPoint {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint_output_array(self.as_raw_mut_VectorOfVectorOfPoint()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVectorOfPoint {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfPoint {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfPoint {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVectorOfPoint2f = core::Vector::<core::Vector::<core::Point2f>>;
	
	impl VectorOfVectorOfPoint2f {
		pub fn as_raw_VectorOfVectorOfPoint2f(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint2f(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<core::Point2f>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfPoint2f_new, cv_VectorOfVectorOfPoint2f_delete,
		cv_VectorOfVectorOfPoint2f_len, cv_VectorOfVectorOfPoint2f_is_empty,
		cv_VectorOfVectorOfPoint2f_capacity, cv_VectorOfVectorOfPoint2f_shrink_to_fit,
		cv_VectorOfVectorOfPoint2f_reserve, cv_VectorOfVectorOfPoint2f_remove,
		cv_VectorOfVectorOfPoint2f_swap, cv_VectorOfVectorOfPoint2f_clear,
		cv_VectorOfVectorOfPoint2f_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { core::Vector::<core::Point2f>::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { core::Vector::<core::Point2f> }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Vector::<core::Point2f>> {
		type Arg = core::Vector::<core::Point2f>;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: core::Vector::<core::Point2f>) {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint2f_push(self.as_raw_mut_VectorOfVectorOfPoint2f(), val.as_raw_mut_VectorOfPoint2f()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: core::Vector::<core::Point2f>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfPoint2f_insert(self.as_raw_mut_VectorOfVectorOfPoint2f(), index, val.as_raw_mut_VectorOfPoint2f()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: core::Vector::<core::Point2f>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfVectorOfPoint2f_set(self.as_raw_mut_VectorOfVectorOfPoint2f(), index, val.as_raw_mut_VectorOfPoint2f()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: core::Vector::<core::Point2f>) {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfPoint2f_set(self.as_raw_mut_VectorOfVectorOfPoint2f(), index, val.as_raw_mut_VectorOfPoint2f())
		}
	}
	
	unsafe impl Send for core::Vector::<core::Vector::<core::Point2f>> {}
	
	impl core::ToInputArray for VectorOfVectorOfPoint2f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint2f_input_array(self.as_raw_VectorOfVectorOfPoint2f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVectorOfPoint2f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfPoint2f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint2f_output_array(self.as_raw_mut_VectorOfVectorOfPoint2f()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVectorOfPoint2f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfPoint2f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint2f_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint2f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfPoint2f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVectorOfPoint3d = core::Vector::<core::Vector::<core::Point3d>>;
	
	impl VectorOfVectorOfPoint3d {
		pub fn as_raw_VectorOfVectorOfPoint3d(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint3d(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<core::Point3d>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfPoint3d_new, cv_VectorOfVectorOfPoint3d_delete,
		cv_VectorOfVectorOfPoint3d_len, cv_VectorOfVectorOfPoint3d_is_empty,
		cv_VectorOfVectorOfPoint3d_capacity, cv_VectorOfVectorOfPoint3d_shrink_to_fit,
		cv_VectorOfVectorOfPoint3d_reserve, cv_VectorOfVectorOfPoint3d_remove,
		cv_VectorOfVectorOfPoint3d_swap, cv_VectorOfVectorOfPoint3d_clear,
		cv_VectorOfVectorOfPoint3d_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { core::Vector::<core::Point3d>::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { core::Vector::<core::Point3d> }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Vector::<core::Point3d>> {
		type Arg = core::Vector::<core::Point3d>;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: core::Vector::<core::Point3d>) {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint3d_push(self.as_raw_mut_VectorOfVectorOfPoint3d(), val.as_raw_mut_VectorOfPoint3d()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: core::Vector::<core::Point3d>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfPoint3d_insert(self.as_raw_mut_VectorOfVectorOfPoint3d(), index, val.as_raw_mut_VectorOfPoint3d()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: core::Vector::<core::Point3d>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfVectorOfPoint3d_set(self.as_raw_mut_VectorOfVectorOfPoint3d(), index, val.as_raw_mut_VectorOfPoint3d()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: core::Vector::<core::Point3d>) {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfPoint3d_set(self.as_raw_mut_VectorOfVectorOfPoint3d(), index, val.as_raw_mut_VectorOfPoint3d())
		}
	}
	
	unsafe impl Send for core::Vector::<core::Vector::<core::Point3d>> {}
	
	impl core::ToInputArray for VectorOfVectorOfPoint3d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3d_input_array(self.as_raw_VectorOfVectorOfPoint3d()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVectorOfPoint3d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfPoint3d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3d_output_array(self.as_raw_mut_VectorOfVectorOfPoint3d()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVectorOfPoint3d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfPoint3d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3d_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint3d()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfPoint3d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVectorOfPoint3f = core::Vector::<core::Vector::<core::Point3f>>;
	
	impl VectorOfVectorOfPoint3f {
		pub fn as_raw_VectorOfVectorOfPoint3f(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint3f(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<core::Point3f>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfPoint3f_new, cv_VectorOfVectorOfPoint3f_delete,
		cv_VectorOfVectorOfPoint3f_len, cv_VectorOfVectorOfPoint3f_is_empty,
		cv_VectorOfVectorOfPoint3f_capacity, cv_VectorOfVectorOfPoint3f_shrink_to_fit,
		cv_VectorOfVectorOfPoint3f_reserve, cv_VectorOfVectorOfPoint3f_remove,
		cv_VectorOfVectorOfPoint3f_swap, cv_VectorOfVectorOfPoint3f_clear,
		cv_VectorOfVectorOfPoint3f_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { core::Vector::<core::Point3f>::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { core::Vector::<core::Point3f> }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Vector::<core::Point3f>> {
		type Arg = core::Vector::<core::Point3f>;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: core::Vector::<core::Point3f>) {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint3f_push(self.as_raw_mut_VectorOfVectorOfPoint3f(), val.as_raw_mut_VectorOfPoint3f()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: core::Vector::<core::Point3f>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfPoint3f_insert(self.as_raw_mut_VectorOfVectorOfPoint3f(), index, val.as_raw_mut_VectorOfPoint3f()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: core::Vector::<core::Point3f>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfVectorOfPoint3f_set(self.as_raw_mut_VectorOfVectorOfPoint3f(), index, val.as_raw_mut_VectorOfPoint3f()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: core::Vector::<core::Point3f>) {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfPoint3f_set(self.as_raw_mut_VectorOfVectorOfPoint3f(), index, val.as_raw_mut_VectorOfPoint3f())
		}
	}
	
	unsafe impl Send for core::Vector::<core::Vector::<core::Point3f>> {}
	
	impl core::ToInputArray for VectorOfVectorOfPoint3f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3f_input_array(self.as_raw_VectorOfVectorOfPoint3f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVectorOfPoint3f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfPoint3f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3f_output_array(self.as_raw_mut_VectorOfVectorOfPoint3f()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVectorOfPoint3f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfPoint3f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3f_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint3f()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfPoint3f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVectorOfPoint3i = core::Vector::<core::Vector::<core::Point3i>>;
	
	impl VectorOfVectorOfPoint3i {
		pub fn as_raw_VectorOfVectorOfPoint3i(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint3i(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<core::Point3i>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfPoint3i_new, cv_VectorOfVectorOfPoint3i_delete,
		cv_VectorOfVectorOfPoint3i_len, cv_VectorOfVectorOfPoint3i_is_empty,
		cv_VectorOfVectorOfPoint3i_capacity, cv_VectorOfVectorOfPoint3i_shrink_to_fit,
		cv_VectorOfVectorOfPoint3i_reserve, cv_VectorOfVectorOfPoint3i_remove,
		cv_VectorOfVectorOfPoint3i_swap, cv_VectorOfVectorOfPoint3i_clear,
		cv_VectorOfVectorOfPoint3i_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { core::Vector::<core::Point3i>::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { core::Vector::<core::Point3i> }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Vector::<core::Point3i>> {
		type Arg = core::Vector::<core::Point3i>;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: core::Vector::<core::Point3i>) {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint3i_push(self.as_raw_mut_VectorOfVectorOfPoint3i(), val.as_raw_mut_VectorOfPoint3i()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: core::Vector::<core::Point3i>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfPoint3i_insert(self.as_raw_mut_VectorOfVectorOfPoint3i(), index, val.as_raw_mut_VectorOfPoint3i()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: core::Vector::<core::Point3i>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfVectorOfPoint3i_set(self.as_raw_mut_VectorOfVectorOfPoint3i(), index, val.as_raw_mut_VectorOfPoint3i()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: core::Vector::<core::Point3i>) {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfPoint3i_set(self.as_raw_mut_VectorOfVectorOfPoint3i(), index, val.as_raw_mut_VectorOfPoint3i())
		}
	}
	
	unsafe impl Send for core::Vector::<core::Vector::<core::Point3i>> {}
	
	impl core::ToInputArray for VectorOfVectorOfPoint3i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3i_input_array(self.as_raw_VectorOfVectorOfPoint3i()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVectorOfPoint3i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfPoint3i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3i_output_array(self.as_raw_mut_VectorOfVectorOfPoint3i()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVectorOfPoint3i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfPoint3i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3i_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint3i()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfPoint3i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVectorOfRect = core::Vector::<core::Vector::<core::Rect>>;
	
	impl VectorOfVectorOfRect {
		pub fn as_raw_VectorOfVectorOfRect(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfRect(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<core::Rect>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfRect_new, cv_VectorOfVectorOfRect_delete,
		cv_VectorOfVectorOfRect_len, cv_VectorOfVectorOfRect_is_empty,
		cv_VectorOfVectorOfRect_capacity, cv_VectorOfVectorOfRect_shrink_to_fit,
		cv_VectorOfVectorOfRect_reserve, cv_VectorOfVectorOfRect_remove,
		cv_VectorOfVectorOfRect_swap, cv_VectorOfVectorOfRect_clear,
		cv_VectorOfVectorOfRect_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { core::Vector::<core::Rect>::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { core::Vector::<core::Rect> }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Vector::<core::Rect>> {
		type Arg = core::Vector::<core::Rect>;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: core::Vector::<core::Rect>) {
			extern "C" { fn cv_VectorOfVectorOfRect_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfRect_push(self.as_raw_mut_VectorOfVectorOfRect(), val.as_raw_mut_VectorOfRect()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: core::Vector::<core::Rect>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfRect_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfRect_insert(self.as_raw_mut_VectorOfVectorOfRect(), index, val.as_raw_mut_VectorOfRect()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: core::Vector::<core::Rect>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfRect_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfVectorOfRect_set(self.as_raw_mut_VectorOfVectorOfRect(), index, val.as_raw_mut_VectorOfRect()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: core::Vector::<core::Rect>) {
			extern "C" { fn cv_VectorOfVectorOfRect_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfRect_set(self.as_raw_mut_VectorOfVectorOfRect(), index, val.as_raw_mut_VectorOfRect())
		}
	}
	
	unsafe impl Send for core::Vector::<core::Vector::<core::Rect>> {}
	
	impl core::ToInputArray for VectorOfVectorOfRect {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfRect_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfRect_input_array(self.as_raw_VectorOfVectorOfRect()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVectorOfRect {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfRect {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfRect_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfRect_output_array(self.as_raw_mut_VectorOfVectorOfRect()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVectorOfRect {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfRect {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfRect_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfRect_input_output_array(self.as_raw_mut_VectorOfVectorOfRect()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfRect {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVectorOfVec2i = core::Vector::<core::Vector::<core::Vec2i>>;
	
	impl VectorOfVectorOfVec2i {
		pub fn as_raw_VectorOfVectorOfVec2i(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfVec2i(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<core::Vec2i>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfVec2i_new, cv_VectorOfVectorOfVec2i_delete,
		cv_VectorOfVectorOfVec2i_len, cv_VectorOfVectorOfVec2i_is_empty,
		cv_VectorOfVectorOfVec2i_capacity, cv_VectorOfVectorOfVec2i_shrink_to_fit,
		cv_VectorOfVectorOfVec2i_reserve, cv_VectorOfVectorOfVec2i_remove,
		cv_VectorOfVectorOfVec2i_swap, cv_VectorOfVectorOfVec2i_clear,
		cv_VectorOfVectorOfVec2i_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { core::Vector::<core::Vec2i>::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { core::Vector::<core::Vec2i> }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Vector::<core::Vec2i>> {
		type Arg = core::Vector::<core::Vec2i>;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: core::Vector::<core::Vec2i>) {
			extern "C" { fn cv_VectorOfVectorOfVec2i_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfVec2i_push(self.as_raw_mut_VectorOfVectorOfVec2i(), val.as_raw_mut_VectorOfVec2i()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: core::Vector::<core::Vec2i>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfVec2i_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfVec2i_insert(self.as_raw_mut_VectorOfVectorOfVec2i(), index, val.as_raw_mut_VectorOfVec2i()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: core::Vector::<core::Vec2i>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfVec2i_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfVectorOfVec2i_set(self.as_raw_mut_VectorOfVectorOfVec2i(), index, val.as_raw_mut_VectorOfVec2i()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: core::Vector::<core::Vec2i>) {
			extern "C" { fn cv_VectorOfVectorOfVec2i_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfVec2i_set(self.as_raw_mut_VectorOfVectorOfVec2i(), index, val.as_raw_mut_VectorOfVec2i())
		}
	}
	
	unsafe impl Send for core::Vector::<core::Vector::<core::Vec2i>> {}
	
	impl core::ToInputArray for VectorOfVectorOfVec2i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfVec2i_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfVec2i_input_array(self.as_raw_VectorOfVectorOfVec2i()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVectorOfVec2i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfVec2i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfVec2i_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfVec2i_output_array(self.as_raw_mut_VectorOfVectorOfVec2i()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVectorOfVec2i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfVec2i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfVec2i_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfVec2i_input_output_array(self.as_raw_mut_VectorOfVectorOfVec2i()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfVec2i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVectorOff64 = core::Vector::<core::Vector::<f64>>;
	
	impl VectorOfVectorOff64 {
		pub fn as_raw_VectorOfVectorOff64(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOff64(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<f64>, *const c_void, *mut c_void,
		cv_VectorOfVectorOff64_new, cv_VectorOfVectorOff64_delete,
		cv_VectorOfVectorOff64_len, cv_VectorOfVectorOff64_is_empty,
		cv_VectorOfVectorOff64_capacity, cv_VectorOfVectorOff64_shrink_to_fit,
		cv_VectorOfVectorOff64_reserve, cv_VectorOfVectorOff64_remove,
		cv_VectorOfVectorOff64_swap, cv_VectorOfVectorOff64_clear,
		cv_VectorOfVectorOff64_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { core::Vector::<f64>::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { core::Vector::<f64> }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Vector::<f64>> {
		type Arg = core::Vector::<f64>;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: core::Vector::<f64>) {
			extern "C" { fn cv_VectorOfVectorOff64_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOff64_push(self.as_raw_mut_VectorOfVectorOff64(), val.as_raw_mut_VectorOff64()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: core::Vector::<f64>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOff64_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOff64_insert(self.as_raw_mut_VectorOfVectorOff64(), index, val.as_raw_mut_VectorOff64()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: core::Vector::<f64>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOff64_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfVectorOff64_set(self.as_raw_mut_VectorOfVectorOff64(), index, val.as_raw_mut_VectorOff64()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: core::Vector::<f64>) {
			extern "C" { fn cv_VectorOfVectorOff64_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOff64_set(self.as_raw_mut_VectorOfVectorOff64(), index, val.as_raw_mut_VectorOff64())
		}
	}
	
	unsafe impl Send for core::Vector::<core::Vector::<f64>> {}
	
	impl core::ToInputArray for VectorOfVectorOff64 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOff64_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOff64_input_array(self.as_raw_VectorOfVectorOff64()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVectorOff64 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOff64 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOff64_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOff64_output_array(self.as_raw_mut_VectorOfVectorOff64()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVectorOff64 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOff64 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOff64_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOff64_input_output_array(self.as_raw_mut_VectorOfVectorOff64()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOff64 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVectorOfi32 = core::Vector::<core::Vector::<i32>>;
	
	impl VectorOfVectorOfi32 {
		pub fn as_raw_VectorOfVectorOfi32(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfi32(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<i32>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfi32_new, cv_VectorOfVectorOfi32_delete,
		cv_VectorOfVectorOfi32_len, cv_VectorOfVectorOfi32_is_empty,
		cv_VectorOfVectorOfi32_capacity, cv_VectorOfVectorOfi32_shrink_to_fit,
		cv_VectorOfVectorOfi32_reserve, cv_VectorOfVectorOfi32_remove,
		cv_VectorOfVectorOfi32_swap, cv_VectorOfVectorOfi32_clear,
		cv_VectorOfVectorOfi32_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { core::Vector::<i32>::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { core::Vector::<i32> }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Vector::<i32>> {
		type Arg = core::Vector::<i32>;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: core::Vector::<i32>) {
			extern "C" { fn cv_VectorOfVectorOfi32_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfi32_push(self.as_raw_mut_VectorOfVectorOfi32(), val.as_raw_mut_VectorOfi32()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: core::Vector::<i32>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfi32_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfi32_insert(self.as_raw_mut_VectorOfVectorOfi32(), index, val.as_raw_mut_VectorOfi32()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: core::Vector::<i32>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfi32_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfVectorOfi32_set(self.as_raw_mut_VectorOfVectorOfi32(), index, val.as_raw_mut_VectorOfi32()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: core::Vector::<i32>) {
			extern "C" { fn cv_VectorOfVectorOfi32_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfi32_set(self.as_raw_mut_VectorOfVectorOfi32(), index, val.as_raw_mut_VectorOfi32())
		}
	}
	
	unsafe impl Send for core::Vector::<core::Vector::<i32>> {}
	
	impl core::ToInputArray for VectorOfVectorOfi32 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfi32_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfi32_input_array(self.as_raw_VectorOfVectorOfi32()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVectorOfi32 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfi32 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfi32_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfi32_output_array(self.as_raw_mut_VectorOfVectorOfi32()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVectorOfi32 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfi32 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfi32_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfi32_input_output_array(self.as_raw_mut_VectorOfVectorOfi32()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfi32 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVectorOfi8 = core::Vector::<core::Vector::<i8>>;
	
	impl VectorOfVectorOfi8 {
		pub fn as_raw_VectorOfVectorOfi8(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfi8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<i8>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfi8_new, cv_VectorOfVectorOfi8_delete,
		cv_VectorOfVectorOfi8_len, cv_VectorOfVectorOfi8_is_empty,
		cv_VectorOfVectorOfi8_capacity, cv_VectorOfVectorOfi8_shrink_to_fit,
		cv_VectorOfVectorOfi8_reserve, cv_VectorOfVectorOfi8_remove,
		cv_VectorOfVectorOfi8_swap, cv_VectorOfVectorOfi8_clear,
		cv_VectorOfVectorOfi8_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { core::Vector::<i8>::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { core::Vector::<i8> }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Vector::<i8>> {
		type Arg = core::Vector::<i8>;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: core::Vector::<i8>) {
			extern "C" { fn cv_VectorOfVectorOfi8_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfi8_push(self.as_raw_mut_VectorOfVectorOfi8(), val.as_raw_mut_VectorOfi8()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: core::Vector::<i8>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfi8_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfi8_insert(self.as_raw_mut_VectorOfVectorOfi8(), index, val.as_raw_mut_VectorOfi8()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: core::Vector::<i8>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfi8_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfVectorOfi8_set(self.as_raw_mut_VectorOfVectorOfi8(), index, val.as_raw_mut_VectorOfi8()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: core::Vector::<i8>) {
			extern "C" { fn cv_VectorOfVectorOfi8_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfi8_set(self.as_raw_mut_VectorOfVectorOfi8(), index, val.as_raw_mut_VectorOfi8())
		}
	}
	
	unsafe impl Send for core::Vector::<core::Vector::<i8>> {}
	
	impl core::ToInputArray for VectorOfVectorOfi8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfi8_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfi8_input_array(self.as_raw_VectorOfVectorOfi8()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVectorOfi8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfi8 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfi8_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfi8_output_array(self.as_raw_mut_VectorOfVectorOfi8()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVectorOfi8 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfi8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfi8_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfi8_input_output_array(self.as_raw_mut_VectorOfVectorOfi8()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfi8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfVectorOfu8 = core::Vector::<core::Vector::<u8>>;
	
	impl VectorOfVectorOfu8 {
		pub fn as_raw_VectorOfVectorOfu8(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfu8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<u8>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfu8_new, cv_VectorOfVectorOfu8_delete,
		cv_VectorOfVectorOfu8_len, cv_VectorOfVectorOfu8_is_empty,
		cv_VectorOfVectorOfu8_capacity, cv_VectorOfVectorOfu8_shrink_to_fit,
		cv_VectorOfVectorOfu8_reserve, cv_VectorOfVectorOfu8_remove,
		cv_VectorOfVectorOfu8_swap, cv_VectorOfVectorOfu8_clear,
		cv_VectorOfVectorOfu8_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { core::Vector::<u8>::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { core::Vector::<u8> }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Vector::<u8>> {
		type Arg = core::Vector::<u8>;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: core::Vector::<u8>) {
			extern "C" { fn cv_VectorOfVectorOfu8_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfu8_push(self.as_raw_mut_VectorOfVectorOfu8(), val.as_raw_mut_VectorOfu8()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: core::Vector::<u8>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfu8_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfu8_insert(self.as_raw_mut_VectorOfVectorOfu8(), index, val.as_raw_mut_VectorOfu8()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: core::Vector::<u8>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfu8_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfVectorOfu8_set(self.as_raw_mut_VectorOfVectorOfu8(), index, val.as_raw_mut_VectorOfu8()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: core::Vector::<u8>) {
			extern "C" { fn cv_VectorOfVectorOfu8_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfu8_set(self.as_raw_mut_VectorOfVectorOfu8(), index, val.as_raw_mut_VectorOfu8())
		}
	}
	
	unsafe impl Send for core::Vector::<core::Vector::<u8>> {}
	
	impl core::ToInputArray for VectorOfVectorOfu8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfu8_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfu8_input_array(self.as_raw_VectorOfVectorOfu8()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfVectorOfu8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfu8 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfu8_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfu8_output_array(self.as_raw_mut_VectorOfVectorOfu8()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfVectorOfu8 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfu8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfu8_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfu8_input_output_array(self.as_raw_mut_VectorOfVectorOfu8()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfu8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfbool = core::Vector::<bool>;
	
	impl VectorOfbool {
		pub fn as_raw_VectorOfbool(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfbool(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { bool, *const c_void, *mut c_void,
		cv_VectorOfbool_new, cv_VectorOfbool_delete,
		cv_VectorOfbool_len, cv_VectorOfbool_is_empty,
		cv_VectorOfbool_capacity, cv_VectorOfbool_shrink_to_fit,
		cv_VectorOfbool_reserve, cv_VectorOfbool_remove,
		cv_VectorOfbool_swap, cv_VectorOfbool_clear,
		cv_VectorOfbool_get -> sys::Result<bool>,
		ret_map: 
	}
	vector_non_copy_or_bool! { bool }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<bool> {
		type Arg = bool;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: bool) {
			extern "C" { fn cv_VectorOfbool_push(instance: *mut c_void, val: bool); }
			unsafe { cv_VectorOfbool_push(self.as_raw_mut_VectorOfbool(), val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: bool) -> Result<()> {
			extern "C" { fn cv_VectorOfbool_insert(instance: *mut c_void, index: size_t, val: bool); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfbool_insert(self.as_raw_mut_VectorOfbool(), index, val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: bool) -> Result<()> {
			extern "C" { fn cv_VectorOfbool_set(instance: *mut c_void, index: size_t, val: bool); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfbool_set(self.as_raw_mut_VectorOfbool(), index, val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: bool) {
			extern "C" { fn cv_VectorOfbool_set(instance: *mut c_void, index: size_t, val: bool); }
			cv_VectorOfbool_set(self.as_raw_mut_VectorOfbool(), index, val)
		}
	}
	
	unsafe impl Send for core::Vector::<bool> {}
	
	pub type VectorOff32 = core::Vector::<f32>;
	
	impl VectorOff32 {
		pub fn as_raw_VectorOff32(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOff32(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { f32, *const c_void, *mut c_void,
		cv_VectorOff32_new, cv_VectorOff32_delete,
		cv_VectorOff32_len, cv_VectorOff32_is_empty,
		cv_VectorOff32_capacity, cv_VectorOff32_shrink_to_fit,
		cv_VectorOff32_reserve, cv_VectorOff32_remove,
		cv_VectorOff32_swap, cv_VectorOff32_clear,
		cv_VectorOff32_get -> sys::Result<f32>,
		ret_map: 
	}
	vector_copy_non_bool! { f32, *const c_void,
		cv_VectorOff32_data
	}
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<f32> {
		type Arg = f32;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: f32) {
			extern "C" { fn cv_VectorOff32_push(instance: *mut c_void, val: f32); }
			unsafe { cv_VectorOff32_push(self.as_raw_mut_VectorOff32(), val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: f32) -> Result<()> {
			extern "C" { fn cv_VectorOff32_insert(instance: *mut c_void, index: size_t, val: f32); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOff32_insert(self.as_raw_mut_VectorOff32(), index, val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: f32) -> Result<()> {
			extern "C" { fn cv_VectorOff32_set(instance: *mut c_void, index: size_t, val: f32); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOff32_set(self.as_raw_mut_VectorOff32(), index, val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: f32) {
			extern "C" { fn cv_VectorOff32_set(instance: *mut c_void, index: size_t, val: f32); }
			cv_VectorOff32_set(self.as_raw_mut_VectorOff32(), index, val)
		}
	}
	
	unsafe impl Send for core::Vector::<f32> {}
	
	impl core::ToInputArray for VectorOff32 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOff32_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOff32_input_array(self.as_raw_VectorOff32()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOff32 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOff32 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOff32_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOff32_output_array(self.as_raw_mut_VectorOff32()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOff32 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOff32 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOff32_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOff32_input_output_array(self.as_raw_mut_VectorOff32()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOff32 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOff64 = core::Vector::<f64>;
	
	impl VectorOff64 {
		pub fn as_raw_VectorOff64(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOff64(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { f64, *const c_void, *mut c_void,
		cv_VectorOff64_new, cv_VectorOff64_delete,
		cv_VectorOff64_len, cv_VectorOff64_is_empty,
		cv_VectorOff64_capacity, cv_VectorOff64_shrink_to_fit,
		cv_VectorOff64_reserve, cv_VectorOff64_remove,
		cv_VectorOff64_swap, cv_VectorOff64_clear,
		cv_VectorOff64_get -> sys::Result<f64>,
		ret_map: 
	}
	vector_copy_non_bool! { f64, *const c_void,
		cv_VectorOff64_data
	}
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<f64> {
		type Arg = f64;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: f64) {
			extern "C" { fn cv_VectorOff64_push(instance: *mut c_void, val: f64); }
			unsafe { cv_VectorOff64_push(self.as_raw_mut_VectorOff64(), val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: f64) -> Result<()> {
			extern "C" { fn cv_VectorOff64_insert(instance: *mut c_void, index: size_t, val: f64); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOff64_insert(self.as_raw_mut_VectorOff64(), index, val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: f64) -> Result<()> {
			extern "C" { fn cv_VectorOff64_set(instance: *mut c_void, index: size_t, val: f64); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOff64_set(self.as_raw_mut_VectorOff64(), index, val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: f64) {
			extern "C" { fn cv_VectorOff64_set(instance: *mut c_void, index: size_t, val: f64); }
			cv_VectorOff64_set(self.as_raw_mut_VectorOff64(), index, val)
		}
	}
	
	unsafe impl Send for core::Vector::<f64> {}
	
	impl core::ToInputArray for VectorOff64 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOff64_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOff64_input_array(self.as_raw_VectorOff64()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOff64 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOff64 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOff64_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOff64_output_array(self.as_raw_mut_VectorOff64()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOff64 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOff64 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOff64_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOff64_input_output_array(self.as_raw_mut_VectorOff64()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOff64 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfi32 = core::Vector::<i32>;
	
	impl VectorOfi32 {
		pub fn as_raw_VectorOfi32(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfi32(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { i32, *const c_void, *mut c_void,
		cv_VectorOfi32_new, cv_VectorOfi32_delete,
		cv_VectorOfi32_len, cv_VectorOfi32_is_empty,
		cv_VectorOfi32_capacity, cv_VectorOfi32_shrink_to_fit,
		cv_VectorOfi32_reserve, cv_VectorOfi32_remove,
		cv_VectorOfi32_swap, cv_VectorOfi32_clear,
		cv_VectorOfi32_get -> sys::Result<i32>,
		ret_map: 
	}
	vector_copy_non_bool! { i32, *const c_void,
		cv_VectorOfi32_data
	}
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<i32> {
		type Arg = i32;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: i32) {
			extern "C" { fn cv_VectorOfi32_push(instance: *mut c_void, val: i32); }
			unsafe { cv_VectorOfi32_push(self.as_raw_mut_VectorOfi32(), val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: i32) -> Result<()> {
			extern "C" { fn cv_VectorOfi32_insert(instance: *mut c_void, index: size_t, val: i32); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfi32_insert(self.as_raw_mut_VectorOfi32(), index, val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: i32) -> Result<()> {
			extern "C" { fn cv_VectorOfi32_set(instance: *mut c_void, index: size_t, val: i32); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfi32_set(self.as_raw_mut_VectorOfi32(), index, val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: i32) {
			extern "C" { fn cv_VectorOfi32_set(instance: *mut c_void, index: size_t, val: i32); }
			cv_VectorOfi32_set(self.as_raw_mut_VectorOfi32(), index, val)
		}
	}
	
	unsafe impl Send for core::Vector::<i32> {}
	
	impl core::ToInputArray for VectorOfi32 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfi32_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfi32_input_array(self.as_raw_VectorOfi32()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfi32 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfi32 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfi32_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfi32_output_array(self.as_raw_mut_VectorOfi32()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfi32 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfi32 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfi32_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfi32_input_output_array(self.as_raw_mut_VectorOfi32()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfi32 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfi8 = core::Vector::<i8>;
	
	impl VectorOfi8 {
		pub fn as_raw_VectorOfi8(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfi8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { i8, *const c_void, *mut c_void,
		cv_VectorOfi8_new, cv_VectorOfi8_delete,
		cv_VectorOfi8_len, cv_VectorOfi8_is_empty,
		cv_VectorOfi8_capacity, cv_VectorOfi8_shrink_to_fit,
		cv_VectorOfi8_reserve, cv_VectorOfi8_remove,
		cv_VectorOfi8_swap, cv_VectorOfi8_clear,
		cv_VectorOfi8_get -> sys::Result<i8>,
		ret_map: 
	}
	vector_copy_non_bool! { i8, *const c_void,
		cv_VectorOfi8_data
	}
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<i8> {
		type Arg = i8;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: i8) {
			extern "C" { fn cv_VectorOfi8_push(instance: *mut c_void, val: i8); }
			unsafe { cv_VectorOfi8_push(self.as_raw_mut_VectorOfi8(), val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: i8) -> Result<()> {
			extern "C" { fn cv_VectorOfi8_insert(instance: *mut c_void, index: size_t, val: i8); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfi8_insert(self.as_raw_mut_VectorOfi8(), index, val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: i8) -> Result<()> {
			extern "C" { fn cv_VectorOfi8_set(instance: *mut c_void, index: size_t, val: i8); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfi8_set(self.as_raw_mut_VectorOfi8(), index, val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: i8) {
			extern "C" { fn cv_VectorOfi8_set(instance: *mut c_void, index: size_t, val: i8); }
			cv_VectorOfi8_set(self.as_raw_mut_VectorOfi8(), index, val)
		}
	}
	
	unsafe impl Send for core::Vector::<i8> {}
	
	impl core::ToInputArray for VectorOfi8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfi8_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfi8_input_array(self.as_raw_VectorOfi8()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfi8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfi8 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfi8_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfi8_output_array(self.as_raw_mut_VectorOfi8()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfi8 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfi8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfi8_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfi8_input_output_array(self.as_raw_mut_VectorOfi8()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfi8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub type VectorOfsize_t = core::Vector::<size_t>;
	
	impl VectorOfsize_t {
		pub fn as_raw_VectorOfsize_t(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfsize_t(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { size_t, *const c_void, *mut c_void,
		cv_VectorOfsize_t_new, cv_VectorOfsize_t_delete,
		cv_VectorOfsize_t_len, cv_VectorOfsize_t_is_empty,
		cv_VectorOfsize_t_capacity, cv_VectorOfsize_t_shrink_to_fit,
		cv_VectorOfsize_t_reserve, cv_VectorOfsize_t_remove,
		cv_VectorOfsize_t_swap, cv_VectorOfsize_t_clear,
		cv_VectorOfsize_t_get -> sys::Result<size_t>,
		ret_map: 
	}
	vector_copy_non_bool! { size_t, *const c_void,
		cv_VectorOfsize_t_data
	}
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<size_t> {
		type Arg = size_t;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: size_t) {
			extern "C" { fn cv_VectorOfsize_t_push(instance: *mut c_void, val: size_t); }
			unsafe { cv_VectorOfsize_t_push(self.as_raw_mut_VectorOfsize_t(), val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: size_t) -> Result<()> {
			extern "C" { fn cv_VectorOfsize_t_insert(instance: *mut c_void, index: size_t, val: size_t); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfsize_t_insert(self.as_raw_mut_VectorOfsize_t(), index, val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: size_t) -> Result<()> {
			extern "C" { fn cv_VectorOfsize_t_set(instance: *mut c_void, index: size_t, val: size_t); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfsize_t_set(self.as_raw_mut_VectorOfsize_t(), index, val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: size_t) {
			extern "C" { fn cv_VectorOfsize_t_set(instance: *mut c_void, index: size_t, val: size_t); }
			cv_VectorOfsize_t_set(self.as_raw_mut_VectorOfsize_t(), index, val)
		}
	}
	
	unsafe impl Send for core::Vector::<size_t> {}
	
	pub type VectorOfu8 = core::Vector::<u8>;
	
	impl VectorOfu8 {
		pub fn as_raw_VectorOfu8(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfu8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { u8, *const c_void, *mut c_void,
		cv_VectorOfu8_new, cv_VectorOfu8_delete,
		cv_VectorOfu8_len, cv_VectorOfu8_is_empty,
		cv_VectorOfu8_capacity, cv_VectorOfu8_shrink_to_fit,
		cv_VectorOfu8_reserve, cv_VectorOfu8_remove,
		cv_VectorOfu8_swap, cv_VectorOfu8_clear,
		cv_VectorOfu8_get -> sys::Result<u8>,
		ret_map: 
	}
	vector_copy_non_bool! { u8, *const c_void,
		cv_VectorOfu8_data
	}
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<u8> {
		type Arg = u8;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, val: u8) {
			extern "C" { fn cv_VectorOfu8_push(instance: *mut c_void, val: u8); }
			unsafe { cv_VectorOfu8_push(self.as_raw_mut_VectorOfu8(), val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: u8) -> Result<()> {
			extern "C" { fn cv_VectorOfu8_insert(instance: *mut c_void, index: size_t, val: u8); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfu8_insert(self.as_raw_mut_VectorOfu8(), index, val) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: u8) -> Result<()> {
			extern "C" { fn cv_VectorOfu8_set(instance: *mut c_void, index: size_t, val: u8); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfu8_set(self.as_raw_mut_VectorOfu8(), index, val) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: u8) {
			extern "C" { fn cv_VectorOfu8_set(instance: *mut c_void, index: size_t, val: u8); }
			cv_VectorOfu8_set(self.as_raw_mut_VectorOfu8(), index, val)
		}
	}
	
	unsafe impl Send for core::Vector::<u8> {}
	
	impl core::ToInputArray for VectorOfu8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfu8_input_array(instance: *const c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfu8_input_array(self.as_raw_VectorOfu8()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToInputArray for &VectorOfu8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			(*self).input_array()
		}
	}
	
	impl core::ToOutputArray for VectorOfu8 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfu8_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfu8_output_array(self.as_raw_mut_VectorOfu8()) }
				.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToOutputArray for &mut VectorOfu8 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			(*self).output_array()
		}
	}
	
	impl core::ToInputOutputArray for VectorOfu8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfu8_input_output_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfu8_input_output_array(self.as_raw_mut_VectorOfu8()) }
				.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfu8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
}
pub use core_types::*;

#[cfg(feature = "contrib")]
mod dnn_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAbsLayer = core::Ptr::<dyn crate::dnn::AbsLayer>;
	
	ptr_extern! { dyn crate::dnn::AbsLayer,
		cv_PtrOfAbsLayer_delete, cv_PtrOfAbsLayer_get_inner_ptr
	}
	
	impl PtrOfAbsLayer {
		pub fn as_raw_PtrOfAbsLayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfAbsLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::AbsLayer for PtrOfAbsLayer {
		fn as_raw_AbsLayer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_AbsLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::Layer for PtrOfAbsLayer {
		fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBNLLLayer = core::Ptr::<dyn crate::dnn::BNLLLayer>;
	
	ptr_extern! { dyn crate::dnn::BNLLLayer,
		cv_PtrOfBNLLLayer_delete, cv_PtrOfBNLLLayer_get_inner_ptr
	}
	
	impl PtrOfBNLLLayer {
		pub fn as_raw_PtrOfBNLLLayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfBNLLLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::BNLLLayer for PtrOfBNLLLayer {
		fn as_raw_BNLLLayer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BNLLLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::Layer for PtrOfBNLLLayer {
		fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBaseConvolutionLayer = core::Ptr::<dyn crate::dnn::BaseConvolutionLayer>;
	
	ptr_extern! { dyn crate::dnn::BaseConvolutionLayer,
		cv_PtrOfBaseConvolutionLayer_delete, cv_PtrOfBaseConvolutionLayer_get_inner_ptr
	}
	
	impl PtrOfBaseConvolutionLayer {
		pub fn as_raw_PtrOfBaseConvolutionLayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfBaseConvolutionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::BaseConvolutionLayer for PtrOfBaseConvolutionLayer {
		fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::Layer for PtrOfBaseConvolutionLayer {
		fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfConcatLayer = core::Ptr::<dyn crate::dnn::ConcatLayer>;
	
	ptr_extern! { dyn crate::dnn::ConcatLayer,
		cv_PtrOfConcatLayer_delete, cv_PtrOfConcatLayer_get_inner_ptr
	}
	
	impl PtrOfConcatLayer {
		pub fn as_raw_PtrOfConcatLayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfConcatLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ConcatLayer for PtrOfConcatLayer {
		fn as_raw_ConcatLayer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_ConcatLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::Layer for PtrOfConcatLayer {
		fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCropLayer = core::Ptr::<dyn crate::dnn::CropLayer>;
	
	ptr_extern! { dyn crate::dnn::CropLayer,
		cv_PtrOfCropLayer_delete, cv_PtrOfCropLayer_get_inner_ptr
	}
	
	impl PtrOfCropLayer {
		pub fn as_raw_PtrOfCropLayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfCropLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::CropLayer for PtrOfCropLayer {
		fn as_raw_CropLayer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_CropLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::Layer for PtrOfCropLayer {
		fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEltwiseLayer = core::Ptr::<dyn crate::dnn::EltwiseLayer>;
	
	ptr_extern! { dyn crate::dnn::EltwiseLayer,
		cv_PtrOfEltwiseLayer_delete, cv_PtrOfEltwiseLayer_get_inner_ptr
	}
	
	impl PtrOfEltwiseLayer {
		pub fn as_raw_PtrOfEltwiseLayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfEltwiseLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::EltwiseLayer for PtrOfEltwiseLayer {
		fn as_raw_EltwiseLayer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_EltwiseLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::Layer for PtrOfEltwiseLayer {
		fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfImporter = core::Ptr::<dyn crate::dnn::Importer>;
	
	ptr_extern! { dyn crate::dnn::Importer,
		cv_PtrOfImporter_delete, cv_PtrOfImporter_get_inner_ptr
	}
	
	impl PtrOfImporter {
		pub fn as_raw_PtrOfImporter(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfImporter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::Importer for PtrOfImporter {
		fn as_raw_Importer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Importer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfInnerProductLayer = core::Ptr::<dyn crate::dnn::InnerProductLayer>;
	
	ptr_extern! { dyn crate::dnn::InnerProductLayer,
		cv_PtrOfInnerProductLayer_delete, cv_PtrOfInnerProductLayer_get_inner_ptr
	}
	
	impl PtrOfInnerProductLayer {
		pub fn as_raw_PtrOfInnerProductLayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfInnerProductLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::InnerProductLayer for PtrOfInnerProductLayer {
		fn as_raw_InnerProductLayer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::Layer for PtrOfInnerProductLayer {
		fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLRNLayer = core::Ptr::<dyn crate::dnn::LRNLayer>;
	
	ptr_extern! { dyn crate::dnn::LRNLayer,
		cv_PtrOfLRNLayer_delete, cv_PtrOfLRNLayer_get_inner_ptr
	}
	
	impl PtrOfLRNLayer {
		pub fn as_raw_PtrOfLRNLayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfLRNLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LRNLayer for PtrOfLRNLayer {
		fn as_raw_LRNLayer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_LRNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::Layer for PtrOfLRNLayer {
		fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLSTMLayer = core::Ptr::<dyn crate::dnn::LSTMLayer>;
	
	ptr_extern! { dyn crate::dnn::LSTMLayer,
		cv_PtrOfLSTMLayer_delete, cv_PtrOfLSTMLayer_get_inner_ptr
	}
	
	impl PtrOfLSTMLayer {
		pub fn as_raw_PtrOfLSTMLayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfLSTMLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LSTMLayer for PtrOfLSTMLayer {
		fn as_raw_LSTMLayer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_LSTMLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::Layer for PtrOfLSTMLayer {
		fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLayer = core::Ptr::<dyn crate::dnn::Layer>;
	
	ptr_extern! { dyn crate::dnn::Layer,
		cv_PtrOfLayer_delete, cv_PtrOfLayer_get_inner_ptr
	}
	
	impl PtrOfLayer {
		pub fn as_raw_PtrOfLayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::Layer for PtrOfLayer {
		fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMVNLayer = core::Ptr::<dyn crate::dnn::MVNLayer>;
	
	ptr_extern! { dyn crate::dnn::MVNLayer,
		cv_PtrOfMVNLayer_delete, cv_PtrOfMVNLayer_get_inner_ptr
	}
	
	impl PtrOfMVNLayer {
		pub fn as_raw_PtrOfMVNLayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfMVNLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::Layer for PtrOfMVNLayer {
		fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::MVNLayer for PtrOfMVNLayer {
		fn as_raw_MVNLayer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_MVNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPoolingLayer = core::Ptr::<dyn crate::dnn::PoolingLayer>;
	
	ptr_extern! { dyn crate::dnn::PoolingLayer,
		cv_PtrOfPoolingLayer_delete, cv_PtrOfPoolingLayer_get_inner_ptr
	}
	
	impl PtrOfPoolingLayer {
		pub fn as_raw_PtrOfPoolingLayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfPoolingLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::Layer for PtrOfPoolingLayer {
		fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::PoolingLayer for PtrOfPoolingLayer {
		fn as_raw_PoolingLayer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPowerLayer = core::Ptr::<dyn crate::dnn::PowerLayer>;
	
	ptr_extern! { dyn crate::dnn::PowerLayer,
		cv_PtrOfPowerLayer_delete, cv_PtrOfPowerLayer_get_inner_ptr
	}
	
	impl PtrOfPowerLayer {
		pub fn as_raw_PtrOfPowerLayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfPowerLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::Layer for PtrOfPowerLayer {
		fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::PowerLayer for PtrOfPowerLayer {
		fn as_raw_PowerLayer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_PowerLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRNNLayer = core::Ptr::<dyn crate::dnn::RNNLayer>;
	
	ptr_extern! { dyn crate::dnn::RNNLayer,
		cv_PtrOfRNNLayer_delete, cv_PtrOfRNNLayer_get_inner_ptr
	}
	
	impl PtrOfRNNLayer {
		pub fn as_raw_PtrOfRNNLayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfRNNLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::Layer for PtrOfRNNLayer {
		fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::RNNLayer for PtrOfRNNLayer {
		fn as_raw_RNNLayer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_RNNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReLULayer = core::Ptr::<dyn crate::dnn::ReLULayer>;
	
	ptr_extern! { dyn crate::dnn::ReLULayer,
		cv_PtrOfReLULayer_delete, cv_PtrOfReLULayer_get_inner_ptr
	}
	
	impl PtrOfReLULayer {
		pub fn as_raw_PtrOfReLULayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfReLULayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::Layer for PtrOfReLULayer {
		fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ReLULayer for PtrOfReLULayer {
		fn as_raw_ReLULayer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_ReLULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReshapeLayer = core::Ptr::<dyn crate::dnn::ReshapeLayer>;
	
	ptr_extern! { dyn crate::dnn::ReshapeLayer,
		cv_PtrOfReshapeLayer_delete, cv_PtrOfReshapeLayer_get_inner_ptr
	}
	
	impl PtrOfReshapeLayer {
		pub fn as_raw_PtrOfReshapeLayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfReshapeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::Layer for PtrOfReshapeLayer {
		fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ReshapeLayer for PtrOfReshapeLayer {
		fn as_raw_ReshapeLayer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_ReshapeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSigmoidLayer = core::Ptr::<dyn crate::dnn::SigmoidLayer>;
	
	ptr_extern! { dyn crate::dnn::SigmoidLayer,
		cv_PtrOfSigmoidLayer_delete, cv_PtrOfSigmoidLayer_get_inner_ptr
	}
	
	impl PtrOfSigmoidLayer {
		pub fn as_raw_PtrOfSigmoidLayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfSigmoidLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::Layer for PtrOfSigmoidLayer {
		fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::SigmoidLayer for PtrOfSigmoidLayer {
		fn as_raw_SigmoidLayer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_SigmoidLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSliceLayer = core::Ptr::<dyn crate::dnn::SliceLayer>;
	
	ptr_extern! { dyn crate::dnn::SliceLayer,
		cv_PtrOfSliceLayer_delete, cv_PtrOfSliceLayer_get_inner_ptr
	}
	
	impl PtrOfSliceLayer {
		pub fn as_raw_PtrOfSliceLayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfSliceLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::Layer for PtrOfSliceLayer {
		fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::SliceLayer for PtrOfSliceLayer {
		fn as_raw_SliceLayer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_SliceLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSoftmaxLayer = core::Ptr::<dyn crate::dnn::SoftmaxLayer>;
	
	ptr_extern! { dyn crate::dnn::SoftmaxLayer,
		cv_PtrOfSoftmaxLayer_delete, cv_PtrOfSoftmaxLayer_get_inner_ptr
	}
	
	impl PtrOfSoftmaxLayer {
		pub fn as_raw_PtrOfSoftmaxLayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfSoftmaxLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::Layer for PtrOfSoftmaxLayer {
		fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::SoftmaxLayer for PtrOfSoftmaxLayer {
		fn as_raw_SoftmaxLayer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSplitLayer = core::Ptr::<dyn crate::dnn::SplitLayer>;
	
	ptr_extern! { dyn crate::dnn::SplitLayer,
		cv_PtrOfSplitLayer_delete, cv_PtrOfSplitLayer_get_inner_ptr
	}
	
	impl PtrOfSplitLayer {
		pub fn as_raw_PtrOfSplitLayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfSplitLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::Layer for PtrOfSplitLayer {
		fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::SplitLayer for PtrOfSplitLayer {
		fn as_raw_SplitLayer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_SplitLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTanHLayer = core::Ptr::<dyn crate::dnn::TanHLayer>;
	
	ptr_extern! { dyn crate::dnn::TanHLayer,
		cv_PtrOfTanHLayer_delete, cv_PtrOfTanHLayer_get_inner_ptr
	}
	
	impl PtrOfTanHLayer {
		pub fn as_raw_PtrOfTanHLayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfTanHLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::Layer for PtrOfTanHLayer {
		fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::TanHLayer for PtrOfTanHLayer {
		fn as_raw_TanHLayer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_TanHLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfBlob = core::Vector::<crate::dnn::Blob>;
	
	impl VectorOfBlob {
		pub fn as_raw_VectorOfBlob(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfBlob(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::dnn::Blob, *const c_void, *mut c_void,
		cv_VectorOfBlob_new, cv_VectorOfBlob_delete,
		cv_VectorOfBlob_len, cv_VectorOfBlob_is_empty,
		cv_VectorOfBlob_capacity, cv_VectorOfBlob_shrink_to_fit,
		cv_VectorOfBlob_reserve, cv_VectorOfBlob_remove,
		cv_VectorOfBlob_swap, cv_VectorOfBlob_clear,
		cv_VectorOfBlob_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { crate::dnn::Blob::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { crate::dnn::Blob }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<crate::dnn::Blob> {
		type Arg = crate::dnn::Blob;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: crate::dnn::Blob) {
			extern "C" { fn cv_VectorOfBlob_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfBlob_push(self.as_raw_mut_VectorOfBlob(), val.as_raw_mut_Blob()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: crate::dnn::Blob) -> Result<()> {
			extern "C" { fn cv_VectorOfBlob_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfBlob_insert(self.as_raw_mut_VectorOfBlob(), index, val.as_raw_mut_Blob()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: crate::dnn::Blob) -> Result<()> {
			extern "C" { fn cv_VectorOfBlob_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfBlob_set(self.as_raw_mut_VectorOfBlob(), index, val.as_raw_mut_Blob()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: crate::dnn::Blob) {
			extern "C" { fn cv_VectorOfBlob_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfBlob_set(self.as_raw_mut_VectorOfBlob(), index, val.as_raw_mut_Blob())
		}
	}
	
	unsafe impl Send for core::Vector::<crate::dnn::Blob> {}
	
	pub type VectorOfNet_LayerId = core::Vector::<crate::dnn::Net_LayerId>;
	
	impl VectorOfNet_LayerId {
		pub fn as_raw_VectorOfNet_LayerId(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfNet_LayerId(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::dnn::Net_LayerId, *const c_void, *mut c_void,
		cv_VectorOfNet_LayerId_new, cv_VectorOfNet_LayerId_delete,
		cv_VectorOfNet_LayerId_len, cv_VectorOfNet_LayerId_is_empty,
		cv_VectorOfNet_LayerId_capacity, cv_VectorOfNet_LayerId_shrink_to_fit,
		cv_VectorOfNet_LayerId_reserve, cv_VectorOfNet_LayerId_remove,
		cv_VectorOfNet_LayerId_swap, cv_VectorOfNet_LayerId_clear,
		cv_VectorOfNet_LayerId_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { crate::dnn::DictValue::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { crate::dnn::Net_LayerId }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<crate::dnn::Net_LayerId> {
		type Arg = crate::dnn::Net_LayerId;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: crate::dnn::Net_LayerId) {
			extern "C" { fn cv_VectorOfNet_LayerId_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfNet_LayerId_push(self.as_raw_mut_VectorOfNet_LayerId(), val.as_raw_mut_DictValue()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: crate::dnn::Net_LayerId) -> Result<()> {
			extern "C" { fn cv_VectorOfNet_LayerId_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfNet_LayerId_insert(self.as_raw_mut_VectorOfNet_LayerId(), index, val.as_raw_mut_DictValue()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: crate::dnn::Net_LayerId) -> Result<()> {
			extern "C" { fn cv_VectorOfNet_LayerId_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfNet_LayerId_set(self.as_raw_mut_VectorOfNet_LayerId(), index, val.as_raw_mut_DictValue()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: crate::dnn::Net_LayerId) {
			extern "C" { fn cv_VectorOfNet_LayerId_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfNet_LayerId_set(self.as_raw_mut_VectorOfNet_LayerId(), index, val.as_raw_mut_DictValue())
		}
	}
	
	unsafe impl Send for core::Vector::<crate::dnn::Net_LayerId> {}
	
}
#[cfg(feature = "contrib")]
pub use dnn_types::*;

#[cfg(feature = "contrib")]
mod dpm_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfDPMDetector = core::Ptr::<dyn crate::dpm::DPMDetector>;
	
	ptr_extern! { dyn crate::dpm::DPMDetector,
		cv_PtrOfDPMDetector_delete, cv_PtrOfDPMDetector_get_inner_ptr
	}
	
	impl PtrOfDPMDetector {
		pub fn as_raw_PtrOfDPMDetector(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfDPMDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dpm::DPMDetector for PtrOfDPMDetector {
		fn as_raw_DPMDetector(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_DPMDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfDPMDetector_ObjectDetection = core::Vector::<crate::dpm::DPMDetector_ObjectDetection>;
	
	impl VectorOfDPMDetector_ObjectDetection {
		pub fn as_raw_VectorOfDPMDetector_ObjectDetection(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfDPMDetector_ObjectDetection(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::dpm::DPMDetector_ObjectDetection, *const c_void, *mut c_void,
		cv_VectorOfDPMDetector_ObjectDetection_new, cv_VectorOfDPMDetector_ObjectDetection_delete,
		cv_VectorOfDPMDetector_ObjectDetection_len, cv_VectorOfDPMDetector_ObjectDetection_is_empty,
		cv_VectorOfDPMDetector_ObjectDetection_capacity, cv_VectorOfDPMDetector_ObjectDetection_shrink_to_fit,
		cv_VectorOfDPMDetector_ObjectDetection_reserve, cv_VectorOfDPMDetector_ObjectDetection_remove,
		cv_VectorOfDPMDetector_ObjectDetection_swap, cv_VectorOfDPMDetector_ObjectDetection_clear,
		cv_VectorOfDPMDetector_ObjectDetection_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { crate::dpm::DPMDetector_ObjectDetection::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { crate::dpm::DPMDetector_ObjectDetection }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<crate::dpm::DPMDetector_ObjectDetection> {
		type Arg = crate::dpm::DPMDetector_ObjectDetection;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: crate::dpm::DPMDetector_ObjectDetection) {
			extern "C" { fn cv_VectorOfDPMDetector_ObjectDetection_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfDPMDetector_ObjectDetection_push(self.as_raw_mut_VectorOfDPMDetector_ObjectDetection(), val.as_raw_mut_DPMDetector_ObjectDetection()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: crate::dpm::DPMDetector_ObjectDetection) -> Result<()> {
			extern "C" { fn cv_VectorOfDPMDetector_ObjectDetection_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfDPMDetector_ObjectDetection_insert(self.as_raw_mut_VectorOfDPMDetector_ObjectDetection(), index, val.as_raw_mut_DPMDetector_ObjectDetection()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: crate::dpm::DPMDetector_ObjectDetection) -> Result<()> {
			extern "C" { fn cv_VectorOfDPMDetector_ObjectDetection_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfDPMDetector_ObjectDetection_set(self.as_raw_mut_VectorOfDPMDetector_ObjectDetection(), index, val.as_raw_mut_DPMDetector_ObjectDetection()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: crate::dpm::DPMDetector_ObjectDetection) {
			extern "C" { fn cv_VectorOfDPMDetector_ObjectDetection_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfDPMDetector_ObjectDetection_set(self.as_raw_mut_VectorOfDPMDetector_ObjectDetection(), index, val.as_raw_mut_DPMDetector_ObjectDetection())
		}
	}
	
	unsafe impl Send for core::Vector::<crate::dpm::DPMDetector_ObjectDetection> {}
	
}
#[cfg(feature = "contrib")]
pub use dpm_types::*;

#[cfg(feature = "contrib")]
mod face_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBIF = core::Ptr::<dyn crate::face::BIF>;
	
	ptr_extern! { dyn crate::face::BIF,
		cv_PtrOfBIF_delete, cv_PtrOfBIF_get_inner_ptr
	}
	
	impl PtrOfBIF {
		pub fn as_raw_PtrOfBIF(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfBIF(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBIF {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::BIF for PtrOfBIF {
		fn as_raw_BIF(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BIF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBasicFaceRecognizer = core::Ptr::<dyn crate::face::BasicFaceRecognizer>;
	
	ptr_extern! { dyn crate::face::BasicFaceRecognizer,
		cv_PtrOfBasicFaceRecognizer_delete, cv_PtrOfBasicFaceRecognizer_get_inner_ptr
	}
	
	impl PtrOfBasicFaceRecognizer {
		pub fn as_raw_PtrOfBasicFaceRecognizer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfBasicFaceRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBasicFaceRecognizer {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::BasicFaceRecognizer for PtrOfBasicFaceRecognizer {
		fn as_raw_BasicFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BasicFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FaceRecognizer for PtrOfBasicFaceRecognizer {
		fn as_raw_FaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLBPHFaceRecognizer = core::Ptr::<dyn crate::face::LBPHFaceRecognizer>;
	
	ptr_extern! { dyn crate::face::LBPHFaceRecognizer,
		cv_PtrOfLBPHFaceRecognizer_delete, cv_PtrOfLBPHFaceRecognizer_get_inner_ptr
	}
	
	impl PtrOfLBPHFaceRecognizer {
		pub fn as_raw_PtrOfLBPHFaceRecognizer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfLBPHFaceRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfLBPHFaceRecognizer {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FaceRecognizer for PtrOfLBPHFaceRecognizer {
		fn as_raw_FaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::LBPHFaceRecognizer for PtrOfLBPHFaceRecognizer {
		fn as_raw_LBPHFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_LBPHFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPredictCollector = core::Ptr::<dyn crate::face::PredictCollector>;
	
	ptr_extern! { dyn crate::face::PredictCollector,
		cv_PtrOfPredictCollector_delete, cv_PtrOfPredictCollector_get_inner_ptr
	}
	
	impl PtrOfPredictCollector {
		pub fn as_raw_PtrOfPredictCollector(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfPredictCollector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::PredictCollector for PtrOfPredictCollector {
		fn as_raw_PredictCollector(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_PredictCollector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStandardCollector = core::Ptr::<crate::face::StandardCollector>;
	
	ptr_extern! { crate::face::StandardCollector,
		cv_PtrOfStandardCollector_delete, cv_PtrOfStandardCollector_get_inner_ptr
	}
	
	impl PtrOfStandardCollector {
		pub fn as_raw_PtrOfStandardCollector(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfStandardCollector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::PredictCollector for PtrOfStandardCollector {
		fn as_raw_PredictCollector(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_PredictCollector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::StandardCollectorTrait for PtrOfStandardCollector {
		fn as_raw_StandardCollector(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_StandardCollector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use face_types::*;

mod features2d_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAKAZE = core::Ptr::<dyn crate::features2d::AKAZE>;
	
	ptr_extern! { dyn crate::features2d::AKAZE,
		cv_PtrOfAKAZE_delete, cv_PtrOfAKAZE_get_inner_ptr
	}
	
	impl PtrOfAKAZE {
		pub fn as_raw_PtrOfAKAZE(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfAKAZE(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::AKAZE for PtrOfAKAZE {
		fn as_raw_AKAZE(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_AKAZE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfAKAZE {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfAKAZE {
		fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAgastFeatureDetector = core::Ptr::<dyn crate::features2d::AgastFeatureDetector>;
	
	ptr_extern! { dyn crate::features2d::AgastFeatureDetector,
		cv_PtrOfAgastFeatureDetector_delete, cv_PtrOfAgastFeatureDetector_get_inner_ptr
	}
	
	impl PtrOfAgastFeatureDetector {
		pub fn as_raw_PtrOfAgastFeatureDetector(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfAgastFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::AgastFeatureDetector for PtrOfAgastFeatureDetector {
		fn as_raw_AgastFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_AgastFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfAgastFeatureDetector {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfAgastFeatureDetector {
		fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBFMatcher = core::Ptr::<crate::features2d::BFMatcher>;
	
	ptr_extern! { crate::features2d::BFMatcher,
		cv_PtrOfBFMatcher_delete, cv_PtrOfBFMatcher_get_inner_ptr
	}
	
	impl PtrOfBFMatcher {
		pub fn as_raw_PtrOfBFMatcher(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfBFMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBFMatcher {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::BFMatcherTrait for PtrOfBFMatcher {
		fn as_raw_BFMatcher(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BFMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::DescriptorMatcher for PtrOfBFMatcher {
		fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBRISK = core::Ptr::<crate::features2d::BRISK>;
	
	ptr_extern! { crate::features2d::BRISK,
		cv_PtrOfBRISK_delete, cv_PtrOfBRISK_get_inner_ptr
	}
	
	impl PtrOfBRISK {
		pub fn as_raw_PtrOfBRISK(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfBRISK(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBRISK {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::BRISKTrait for PtrOfBRISK {
		fn as_raw_BRISK(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BRISK(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfBRISK {
		fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDescriptorMatcher = core::Ptr::<dyn crate::features2d::DescriptorMatcher>;
	
	ptr_extern! { dyn crate::features2d::DescriptorMatcher,
		cv_PtrOfDescriptorMatcher_delete, cv_PtrOfDescriptorMatcher_get_inner_ptr
	}
	
	impl PtrOfDescriptorMatcher {
		pub fn as_raw_PtrOfDescriptorMatcher(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfDescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfDescriptorMatcher {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::DescriptorMatcher for PtrOfDescriptorMatcher {
		fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFastFeatureDetector = core::Ptr::<dyn crate::features2d::FastFeatureDetector>;
	
	ptr_extern! { dyn crate::features2d::FastFeatureDetector,
		cv_PtrOfFastFeatureDetector_delete, cv_PtrOfFastFeatureDetector_get_inner_ptr
	}
	
	impl PtrOfFastFeatureDetector {
		pub fn as_raw_PtrOfFastFeatureDetector(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfFastFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFastFeatureDetector {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::FastFeatureDetector for PtrOfFastFeatureDetector {
		fn as_raw_FastFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_FastFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfFastFeatureDetector {
		fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFeature2D = core::Ptr::<crate::features2d::Feature2D>;
	
	ptr_extern! { crate::features2d::Feature2D,
		cv_PtrOfFeature2D_delete, cv_PtrOfFeature2D_get_inner_ptr
	}
	
	impl PtrOfFeature2D {
		pub fn as_raw_PtrOfFeature2D(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfFeature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFeature2D {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfFeature2D {
		fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFlannBasedMatcher = core::Ptr::<crate::features2d::FlannBasedMatcher>;
	
	ptr_extern! { crate::features2d::FlannBasedMatcher,
		cv_PtrOfFlannBasedMatcher_delete, cv_PtrOfFlannBasedMatcher_get_inner_ptr
	}
	
	impl PtrOfFlannBasedMatcher {
		pub fn as_raw_PtrOfFlannBasedMatcher(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfFlannBasedMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFlannBasedMatcher {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::DescriptorMatcher for PtrOfFlannBasedMatcher {
		fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::FlannBasedMatcherTrait for PtrOfFlannBasedMatcher {
		fn as_raw_FlannBasedMatcher(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_FlannBasedMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGFTTDetector = core::Ptr::<dyn crate::features2d::GFTTDetector>;
	
	ptr_extern! { dyn crate::features2d::GFTTDetector,
		cv_PtrOfGFTTDetector_delete, cv_PtrOfGFTTDetector_get_inner_ptr
	}
	
	impl PtrOfGFTTDetector {
		pub fn as_raw_PtrOfGFTTDetector(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfGFTTDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfGFTTDetector {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfGFTTDetector {
		fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::GFTTDetector for PtrOfGFTTDetector {
		fn as_raw_GFTTDetector(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_GFTTDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfKAZE = core::Ptr::<dyn crate::features2d::KAZE>;
	
	ptr_extern! { dyn crate::features2d::KAZE,
		cv_PtrOfKAZE_delete, cv_PtrOfKAZE_get_inner_ptr
	}
	
	impl PtrOfKAZE {
		pub fn as_raw_PtrOfKAZE(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfKAZE(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfKAZE {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfKAZE {
		fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::KAZE for PtrOfKAZE {
		fn as_raw_KAZE(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_KAZE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMSER = core::Ptr::<dyn crate::features2d::MSER>;
	
	ptr_extern! { dyn crate::features2d::MSER,
		cv_PtrOfMSER_delete, cv_PtrOfMSER_get_inner_ptr
	}
	
	impl PtrOfMSER {
		pub fn as_raw_PtrOfMSER(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfMSER(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfMSER {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfMSER {
		fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::MSER for PtrOfMSER {
		fn as_raw_MSER(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_MSER(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfORB = core::Ptr::<dyn crate::features2d::ORB>;
	
	ptr_extern! { dyn crate::features2d::ORB,
		cv_PtrOfORB_delete, cv_PtrOfORB_get_inner_ptr
	}
	
	impl PtrOfORB {
		pub fn as_raw_PtrOfORB(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfORB(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfORB {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfORB {
		fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::ORB for PtrOfORB {
		fn as_raw_ORB(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_ORB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSimpleBlobDetector = core::Ptr::<crate::features2d::SimpleBlobDetector>;
	
	ptr_extern! { crate::features2d::SimpleBlobDetector,
		cv_PtrOfSimpleBlobDetector_delete, cv_PtrOfSimpleBlobDetector_get_inner_ptr
	}
	
	impl PtrOfSimpleBlobDetector {
		pub fn as_raw_PtrOfSimpleBlobDetector(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfSimpleBlobDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSimpleBlobDetector {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfSimpleBlobDetector {
		fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::SimpleBlobDetectorTrait for PtrOfSimpleBlobDetector {
		fn as_raw_SimpleBlobDetector(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_SimpleBlobDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
pub use features2d_types::*;

mod flann_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfIndexParams = core::Ptr::<crate::flann::IndexParams>;
	
	ptr_extern! { crate::flann::IndexParams,
		cv_PtrOfIndexParams_delete, cv_PtrOfIndexParams_get_inner_ptr
	}
	
	impl PtrOfIndexParams {
		pub fn as_raw_PtrOfIndexParams(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::flann::IndexParamsTrait for PtrOfIndexParams {
		fn as_raw_IndexParams(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSearchParams = core::Ptr::<crate::flann::SearchParams>;
	
	ptr_extern! { crate::flann::SearchParams,
		cv_PtrOfSearchParams_delete, cv_PtrOfSearchParams_get_inner_ptr
	}
	
	impl PtrOfSearchParams {
		pub fn as_raw_PtrOfSearchParams(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfSearchParams(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::flann::IndexParamsTrait for PtrOfSearchParams {
		fn as_raw_IndexParams(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::flann::SearchParamsTrait for PtrOfSearchParams {
		fn as_raw_SearchParams(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_SearchParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
pub use flann_types::*;

#[cfg(feature = "contrib")]
mod freetype_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfFreeType2 = core::Ptr::<dyn crate::freetype::FreeType2>;
	
	ptr_extern! { dyn crate::freetype::FreeType2,
		cv_PtrOfFreeType2_delete, cv_PtrOfFreeType2_get_inner_ptr
	}
	
	impl PtrOfFreeType2 {
		pub fn as_raw_PtrOfFreeType2(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfFreeType2(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFreeType2 {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::freetype::FreeType2 for PtrOfFreeType2 {
		fn as_raw_FreeType2(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_FreeType2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use freetype_types::*;

#[cfg(feature = "contrib")]
mod hdf_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfHDF5 = core::Ptr::<dyn crate::hdf::HDF5>;
	
	ptr_extern! { dyn crate::hdf::HDF5,
		cv_PtrOfHDF5_delete, cv_PtrOfHDF5_get_inner_ptr
	}
	
	impl PtrOfHDF5 {
		pub fn as_raw_PtrOfHDF5(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfHDF5(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::hdf::HDF5 for PtrOfHDF5 {
		fn as_raw_HDF5(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_HDF5(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use hdf_types::*;

mod imgproc_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCLAHE = core::Ptr::<dyn crate::imgproc::CLAHE>;
	
	ptr_extern! { dyn crate::imgproc::CLAHE,
		cv_PtrOfCLAHE_delete, cv_PtrOfCLAHE_get_inner_ptr
	}
	
	impl PtrOfCLAHE {
		pub fn as_raw_PtrOfCLAHE(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfCLAHE(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCLAHE {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::imgproc::CLAHE for PtrOfCLAHE {
		fn as_raw_CLAHE(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_CLAHE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGeneralizedHoughBallard = core::Ptr::<dyn crate::imgproc::GeneralizedHoughBallard>;
	
	ptr_extern! { dyn crate::imgproc::GeneralizedHoughBallard,
		cv_PtrOfGeneralizedHoughBallard_delete, cv_PtrOfGeneralizedHoughBallard_get_inner_ptr
	}
	
	impl PtrOfGeneralizedHoughBallard {
		pub fn as_raw_PtrOfGeneralizedHoughBallard(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfGeneralizedHoughBallard(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfGeneralizedHoughBallard {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::imgproc::GeneralizedHough for PtrOfGeneralizedHoughBallard {
		fn as_raw_GeneralizedHough(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::imgproc::GeneralizedHoughBallard for PtrOfGeneralizedHoughBallard {
		fn as_raw_GeneralizedHoughBallard(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_GeneralizedHoughBallard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGeneralizedHoughGuil = core::Ptr::<dyn crate::imgproc::GeneralizedHoughGuil>;
	
	ptr_extern! { dyn crate::imgproc::GeneralizedHoughGuil,
		cv_PtrOfGeneralizedHoughGuil_delete, cv_PtrOfGeneralizedHoughGuil_get_inner_ptr
	}
	
	impl PtrOfGeneralizedHoughGuil {
		pub fn as_raw_PtrOfGeneralizedHoughGuil(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfGeneralizedHoughGuil(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfGeneralizedHoughGuil {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::imgproc::GeneralizedHough for PtrOfGeneralizedHoughGuil {
		fn as_raw_GeneralizedHough(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::imgproc::GeneralizedHoughGuil for PtrOfGeneralizedHoughGuil {
		fn as_raw_GeneralizedHoughGuil(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_GeneralizedHoughGuil(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLineSegmentDetector = core::Ptr::<dyn crate::imgproc::LineSegmentDetector>;
	
	ptr_extern! { dyn crate::imgproc::LineSegmentDetector,
		cv_PtrOfLineSegmentDetector_delete, cv_PtrOfLineSegmentDetector_get_inner_ptr
	}
	
	impl PtrOfLineSegmentDetector {
		pub fn as_raw_PtrOfLineSegmentDetector(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfLineSegmentDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfLineSegmentDetector {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::imgproc::LineSegmentDetector for PtrOfLineSegmentDetector {
		fn as_raw_LineSegmentDetector(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_LineSegmentDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
pub use imgproc_types::*;

#[cfg(feature = "contrib")]
mod line_descriptor_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBinaryDescriptor = core::Ptr::<crate::line_descriptor::BinaryDescriptor>;
	
	ptr_extern! { crate::line_descriptor::BinaryDescriptor,
		cv_PtrOfBinaryDescriptor_delete, cv_PtrOfBinaryDescriptor_get_inner_ptr
	}
	
	impl PtrOfBinaryDescriptor {
		pub fn as_raw_PtrOfBinaryDescriptor(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfBinaryDescriptor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBinaryDescriptor {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::line_descriptor::BinaryDescriptorTrait for PtrOfBinaryDescriptor {
		fn as_raw_BinaryDescriptor(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BinaryDescriptor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBinaryDescriptorMatcher = core::Ptr::<crate::line_descriptor::BinaryDescriptorMatcher>;
	
	ptr_extern! { crate::line_descriptor::BinaryDescriptorMatcher,
		cv_PtrOfBinaryDescriptorMatcher_delete, cv_PtrOfBinaryDescriptorMatcher_get_inner_ptr
	}
	
	impl PtrOfBinaryDescriptorMatcher {
		pub fn as_raw_PtrOfBinaryDescriptorMatcher(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfBinaryDescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBinaryDescriptorMatcher {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::line_descriptor::BinaryDescriptorMatcherTrait for PtrOfBinaryDescriptorMatcher {
		fn as_raw_BinaryDescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BinaryDescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLSDDetector = core::Ptr::<crate::line_descriptor::LSDDetector>;
	
	ptr_extern! { crate::line_descriptor::LSDDetector,
		cv_PtrOfLSDDetector_delete, cv_PtrOfLSDDetector_get_inner_ptr
	}
	
	impl PtrOfLSDDetector {
		pub fn as_raw_PtrOfLSDDetector(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfLSDDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfLSDDetector {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::line_descriptor::LSDDetectorTrait for PtrOfLSDDetector {
		fn as_raw_LSDDetector(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_LSDDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfKeyLine = core::Vector::<crate::line_descriptor::KeyLine>;
	
	impl VectorOfKeyLine {
		pub fn as_raw_VectorOfKeyLine(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfKeyLine(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::line_descriptor::KeyLine, *const c_void, *mut c_void,
		cv_VectorOfKeyLine_new, cv_VectorOfKeyLine_delete,
		cv_VectorOfKeyLine_len, cv_VectorOfKeyLine_is_empty,
		cv_VectorOfKeyLine_capacity, cv_VectorOfKeyLine_shrink_to_fit,
		cv_VectorOfKeyLine_reserve, cv_VectorOfKeyLine_remove,
		cv_VectorOfKeyLine_swap, cv_VectorOfKeyLine_clear,
		cv_VectorOfKeyLine_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { crate::line_descriptor::KeyLine::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { crate::line_descriptor::KeyLine }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<crate::line_descriptor::KeyLine> {
		type Arg = crate::line_descriptor::KeyLine;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: crate::line_descriptor::KeyLine) {
			extern "C" { fn cv_VectorOfKeyLine_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfKeyLine_push(self.as_raw_mut_VectorOfKeyLine(), val.as_raw_mut_KeyLine()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: crate::line_descriptor::KeyLine) -> Result<()> {
			extern "C" { fn cv_VectorOfKeyLine_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfKeyLine_insert(self.as_raw_mut_VectorOfKeyLine(), index, val.as_raw_mut_KeyLine()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: crate::line_descriptor::KeyLine) -> Result<()> {
			extern "C" { fn cv_VectorOfKeyLine_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfKeyLine_set(self.as_raw_mut_VectorOfKeyLine(), index, val.as_raw_mut_KeyLine()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: crate::line_descriptor::KeyLine) {
			extern "C" { fn cv_VectorOfKeyLine_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfKeyLine_set(self.as_raw_mut_VectorOfKeyLine(), index, val.as_raw_mut_KeyLine())
		}
	}
	
	unsafe impl Send for core::Vector::<crate::line_descriptor::KeyLine> {}
	
	pub type VectorOfVectorOfKeyLine = core::Vector::<core::Vector::<crate::line_descriptor::KeyLine>>;
	
	impl VectorOfVectorOfKeyLine {
		pub fn as_raw_VectorOfVectorOfKeyLine(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfKeyLine(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<crate::line_descriptor::KeyLine>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfKeyLine_new, cv_VectorOfVectorOfKeyLine_delete,
		cv_VectorOfVectorOfKeyLine_len, cv_VectorOfVectorOfKeyLine_is_empty,
		cv_VectorOfVectorOfKeyLine_capacity, cv_VectorOfVectorOfKeyLine_shrink_to_fit,
		cv_VectorOfVectorOfKeyLine_reserve, cv_VectorOfVectorOfKeyLine_remove,
		cv_VectorOfVectorOfKeyLine_swap, cv_VectorOfVectorOfKeyLine_clear,
		cv_VectorOfVectorOfKeyLine_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { core::Vector::<crate::line_descriptor::KeyLine>::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { core::Vector::<crate::line_descriptor::KeyLine> }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Vector::<crate::line_descriptor::KeyLine>> {
		type Arg = core::Vector::<crate::line_descriptor::KeyLine>;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: core::Vector::<crate::line_descriptor::KeyLine>) {
			extern "C" { fn cv_VectorOfVectorOfKeyLine_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfKeyLine_push(self.as_raw_mut_VectorOfVectorOfKeyLine(), val.as_raw_mut_VectorOfKeyLine()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: core::Vector::<crate::line_descriptor::KeyLine>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfKeyLine_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfKeyLine_insert(self.as_raw_mut_VectorOfVectorOfKeyLine(), index, val.as_raw_mut_VectorOfKeyLine()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: core::Vector::<crate::line_descriptor::KeyLine>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfKeyLine_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfVectorOfKeyLine_set(self.as_raw_mut_VectorOfVectorOfKeyLine(), index, val.as_raw_mut_VectorOfKeyLine()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: core::Vector::<crate::line_descriptor::KeyLine>) {
			extern "C" { fn cv_VectorOfVectorOfKeyLine_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfKeyLine_set(self.as_raw_mut_VectorOfVectorOfKeyLine(), index, val.as_raw_mut_VectorOfKeyLine())
		}
	}
	
	unsafe impl Send for core::Vector::<core::Vector::<crate::line_descriptor::KeyLine>> {}
	
}
#[cfg(feature = "contrib")]
pub use line_descriptor_types::*;

mod ml_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfANN_MLP = core::Ptr::<dyn crate::ml::ANN_MLP>;
	
	ptr_extern! { dyn crate::ml::ANN_MLP,
		cv_PtrOfANN_MLP_delete, cv_PtrOfANN_MLP_get_inner_ptr
	}
	
	impl PtrOfANN_MLP {
		pub fn as_raw_PtrOfANN_MLP(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfANN_MLP(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ml::ANN_MLP for PtrOfANN_MLP {
		fn as_raw_ANN_MLP(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_ANN_MLP(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfANN_MLP {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModel for PtrOfANN_MLP {
		fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBoost = core::Ptr::<dyn crate::ml::Boost>;
	
	ptr_extern! { dyn crate::ml::Boost,
		cv_PtrOfBoost_delete, cv_PtrOfBoost_get_inner_ptr
	}
	
	impl PtrOfBoost {
		pub fn as_raw_PtrOfBoost(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfBoost(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBoost {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::Boost for PtrOfBoost {
		fn as_raw_Boost(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Boost(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::DTrees for PtrOfBoost {
		fn as_raw_DTrees(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_DTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModel for PtrOfBoost {
		fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDTrees = core::Ptr::<dyn crate::ml::DTrees>;
	
	ptr_extern! { dyn crate::ml::DTrees,
		cv_PtrOfDTrees_delete, cv_PtrOfDTrees_get_inner_ptr
	}
	
	impl PtrOfDTrees {
		pub fn as_raw_PtrOfDTrees(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfDTrees(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfDTrees {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::DTrees for PtrOfDTrees {
		fn as_raw_DTrees(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_DTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModel for PtrOfDTrees {
		fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEM = core::Ptr::<dyn crate::ml::EM>;
	
	ptr_extern! { dyn crate::ml::EM,
		cv_PtrOfEM_delete, cv_PtrOfEM_get_inner_ptr
	}
	
	impl PtrOfEM {
		pub fn as_raw_PtrOfEM(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfEM(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfEM {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::EM for PtrOfEM {
		fn as_raw_EM(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_EM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModel for PtrOfEM {
		fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfKNearest = core::Ptr::<dyn crate::ml::KNearest>;
	
	ptr_extern! { dyn crate::ml::KNearest,
		cv_PtrOfKNearest_delete, cv_PtrOfKNearest_get_inner_ptr
	}
	
	impl PtrOfKNearest {
		pub fn as_raw_PtrOfKNearest(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfKNearest(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfKNearest {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::KNearest for PtrOfKNearest {
		fn as_raw_KNearest(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_KNearest(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModel for PtrOfKNearest {
		fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLogisticRegression = core::Ptr::<dyn crate::ml::LogisticRegression>;
	
	ptr_extern! { dyn crate::ml::LogisticRegression,
		cv_PtrOfLogisticRegression_delete, cv_PtrOfLogisticRegression_get_inner_ptr
	}
	
	impl PtrOfLogisticRegression {
		pub fn as_raw_PtrOfLogisticRegression(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfLogisticRegression(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfLogisticRegression {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::LogisticRegression for PtrOfLogisticRegression {
		fn as_raw_LogisticRegression(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_LogisticRegression(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModel for PtrOfLogisticRegression {
		fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfNormalBayesClassifier = core::Ptr::<dyn crate::ml::NormalBayesClassifier>;
	
	ptr_extern! { dyn crate::ml::NormalBayesClassifier,
		cv_PtrOfNormalBayesClassifier_delete, cv_PtrOfNormalBayesClassifier_get_inner_ptr
	}
	
	impl PtrOfNormalBayesClassifier {
		pub fn as_raw_PtrOfNormalBayesClassifier(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfNormalBayesClassifier(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfNormalBayesClassifier {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::NormalBayesClassifier for PtrOfNormalBayesClassifier {
		fn as_raw_NormalBayesClassifier(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_NormalBayesClassifier(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModel for PtrOfNormalBayesClassifier {
		fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRTrees = core::Ptr::<dyn crate::ml::RTrees>;
	
	ptr_extern! { dyn crate::ml::RTrees,
		cv_PtrOfRTrees_delete, cv_PtrOfRTrees_get_inner_ptr
	}
	
	impl PtrOfRTrees {
		pub fn as_raw_PtrOfRTrees(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfRTrees(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfRTrees {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::DTrees for PtrOfRTrees {
		fn as_raw_DTrees(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_DTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::RTrees for PtrOfRTrees {
		fn as_raw_RTrees(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_RTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModel for PtrOfRTrees {
		fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSVM = core::Ptr::<dyn crate::ml::SVM>;
	
	ptr_extern! { dyn crate::ml::SVM,
		cv_PtrOfSVM_delete, cv_PtrOfSVM_get_inner_ptr
	}
	
	impl PtrOfSVM {
		pub fn as_raw_PtrOfSVM(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfSVM(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSVM {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::SVM for PtrOfSVM {
		fn as_raw_SVM(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_SVM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModel for PtrOfSVM {
		fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSVMSGD = core::Ptr::<dyn crate::ml::SVMSGD>;
	
	ptr_extern! { dyn crate::ml::SVMSGD,
		cv_PtrOfSVMSGD_delete, cv_PtrOfSVMSGD_get_inner_ptr
	}
	
	impl PtrOfSVMSGD {
		pub fn as_raw_PtrOfSVMSGD(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfSVMSGD(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSVMSGD {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::SVMSGD for PtrOfSVMSGD {
		fn as_raw_SVMSGD(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_SVMSGD(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModel for PtrOfSVMSGD {
		fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSVM_Kernel = core::Ptr::<dyn crate::ml::SVM_Kernel>;
	
	ptr_extern! { dyn crate::ml::SVM_Kernel,
		cv_PtrOfSVM_Kernel_delete, cv_PtrOfSVM_Kernel_get_inner_ptr
	}
	
	impl PtrOfSVM_Kernel {
		pub fn as_raw_PtrOfSVM_Kernel(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfSVM_Kernel(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSVM_Kernel {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::SVM_Kernel for PtrOfSVM_Kernel {
		fn as_raw_SVM_Kernel(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_SVM_Kernel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrainData = core::Ptr::<dyn crate::ml::TrainData>;
	
	ptr_extern! { dyn crate::ml::TrainData,
		cv_PtrOfTrainData_delete, cv_PtrOfTrainData_get_inner_ptr
	}
	
	impl PtrOfTrainData {
		pub fn as_raw_PtrOfTrainData(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfTrainData(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ml::TrainData for PtrOfTrainData {
		fn as_raw_TrainData(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_TrainData(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfDTrees_Node = core::Vector::<crate::ml::DTrees_Node>;
	
	impl VectorOfDTrees_Node {
		pub fn as_raw_VectorOfDTrees_Node(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfDTrees_Node(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::ml::DTrees_Node, *const c_void, *mut c_void,
		cv_VectorOfDTrees_Node_new, cv_VectorOfDTrees_Node_delete,
		cv_VectorOfDTrees_Node_len, cv_VectorOfDTrees_Node_is_empty,
		cv_VectorOfDTrees_Node_capacity, cv_VectorOfDTrees_Node_shrink_to_fit,
		cv_VectorOfDTrees_Node_reserve, cv_VectorOfDTrees_Node_remove,
		cv_VectorOfDTrees_Node_swap, cv_VectorOfDTrees_Node_clear,
		cv_VectorOfDTrees_Node_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { crate::ml::DTrees_Node::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { crate::ml::DTrees_Node }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<crate::ml::DTrees_Node> {
		type Arg = crate::ml::DTrees_Node;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: crate::ml::DTrees_Node) {
			extern "C" { fn cv_VectorOfDTrees_Node_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfDTrees_Node_push(self.as_raw_mut_VectorOfDTrees_Node(), val.as_raw_mut_DTrees_Node()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: crate::ml::DTrees_Node) -> Result<()> {
			extern "C" { fn cv_VectorOfDTrees_Node_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfDTrees_Node_insert(self.as_raw_mut_VectorOfDTrees_Node(), index, val.as_raw_mut_DTrees_Node()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: crate::ml::DTrees_Node) -> Result<()> {
			extern "C" { fn cv_VectorOfDTrees_Node_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfDTrees_Node_set(self.as_raw_mut_VectorOfDTrees_Node(), index, val.as_raw_mut_DTrees_Node()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: crate::ml::DTrees_Node) {
			extern "C" { fn cv_VectorOfDTrees_Node_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfDTrees_Node_set(self.as_raw_mut_VectorOfDTrees_Node(), index, val.as_raw_mut_DTrees_Node())
		}
	}
	
	unsafe impl Send for core::Vector::<crate::ml::DTrees_Node> {}
	
	pub type VectorOfDTrees_Split = core::Vector::<crate::ml::DTrees_Split>;
	
	impl VectorOfDTrees_Split {
		pub fn as_raw_VectorOfDTrees_Split(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfDTrees_Split(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::ml::DTrees_Split, *const c_void, *mut c_void,
		cv_VectorOfDTrees_Split_new, cv_VectorOfDTrees_Split_delete,
		cv_VectorOfDTrees_Split_len, cv_VectorOfDTrees_Split_is_empty,
		cv_VectorOfDTrees_Split_capacity, cv_VectorOfDTrees_Split_shrink_to_fit,
		cv_VectorOfDTrees_Split_reserve, cv_VectorOfDTrees_Split_remove,
		cv_VectorOfDTrees_Split_swap, cv_VectorOfDTrees_Split_clear,
		cv_VectorOfDTrees_Split_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { crate::ml::DTrees_Split::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { crate::ml::DTrees_Split }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<crate::ml::DTrees_Split> {
		type Arg = crate::ml::DTrees_Split;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: crate::ml::DTrees_Split) {
			extern "C" { fn cv_VectorOfDTrees_Split_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfDTrees_Split_push(self.as_raw_mut_VectorOfDTrees_Split(), val.as_raw_mut_DTrees_Split()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: crate::ml::DTrees_Split) -> Result<()> {
			extern "C" { fn cv_VectorOfDTrees_Split_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfDTrees_Split_insert(self.as_raw_mut_VectorOfDTrees_Split(), index, val.as_raw_mut_DTrees_Split()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: crate::ml::DTrees_Split) -> Result<()> {
			extern "C" { fn cv_VectorOfDTrees_Split_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfDTrees_Split_set(self.as_raw_mut_VectorOfDTrees_Split(), index, val.as_raw_mut_DTrees_Split()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: crate::ml::DTrees_Split) {
			extern "C" { fn cv_VectorOfDTrees_Split_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfDTrees_Split_set(self.as_raw_mut_VectorOfDTrees_Split(), index, val.as_raw_mut_DTrees_Split())
		}
	}
	
	unsafe impl Send for core::Vector::<crate::ml::DTrees_Split> {}
	
}
pub use ml_types::*;

mod objdetect_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBaseCascadeClassifier = core::Ptr::<dyn crate::objdetect::BaseCascadeClassifier>;
	
	ptr_extern! { dyn crate::objdetect::BaseCascadeClassifier,
		cv_PtrOfBaseCascadeClassifier_delete, cv_PtrOfBaseCascadeClassifier_get_inner_ptr
	}
	
	impl PtrOfBaseCascadeClassifier {
		pub fn as_raw_PtrOfBaseCascadeClassifier(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfBaseCascadeClassifier(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBaseCascadeClassifier {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::objdetect::BaseCascadeClassifier for PtrOfBaseCascadeClassifier {
		fn as_raw_BaseCascadeClassifier(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BaseCascadeClassifier(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBaseCascadeClassifier_MaskGenerator = core::Ptr::<dyn crate::objdetect::BaseCascadeClassifier_MaskGenerator>;
	
	ptr_extern! { dyn crate::objdetect::BaseCascadeClassifier_MaskGenerator,
		cv_PtrOfBaseCascadeClassifier_MaskGenerator_delete, cv_PtrOfBaseCascadeClassifier_MaskGenerator_get_inner_ptr
	}
	
	impl PtrOfBaseCascadeClassifier_MaskGenerator {
		pub fn as_raw_PtrOfBaseCascadeClassifier_MaskGenerator(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfBaseCascadeClassifier_MaskGenerator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::objdetect::BaseCascadeClassifier_MaskGenerator for PtrOfBaseCascadeClassifier_MaskGenerator {
		fn as_raw_BaseCascadeClassifier_MaskGenerator(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BaseCascadeClassifier_MaskGenerator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetectionBasedTracker_IDetector = core::Ptr::<dyn crate::objdetect::DetectionBasedTracker_IDetector>;
	
	ptr_extern! { dyn crate::objdetect::DetectionBasedTracker_IDetector,
		cv_PtrOfDetectionBasedTracker_IDetector_delete, cv_PtrOfDetectionBasedTracker_IDetector_get_inner_ptr
	}
	
	impl PtrOfDetectionBasedTracker_IDetector {
		pub fn as_raw_PtrOfDetectionBasedTracker_IDetector(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfDetectionBasedTracker_IDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::objdetect::DetectionBasedTracker_IDetector for PtrOfDetectionBasedTracker_IDetector {
		fn as_raw_DetectionBasedTracker_IDetector(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_DetectionBasedTracker_IDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfDetectionBasedTracker_ExtObject = core::Vector::<crate::objdetect::DetectionBasedTracker_ExtObject>;
	
	impl VectorOfDetectionBasedTracker_ExtObject {
		pub fn as_raw_VectorOfDetectionBasedTracker_ExtObject(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetectionBasedTracker_ExtObject(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::objdetect::DetectionBasedTracker_ExtObject, *const c_void, *mut c_void,
		cv_VectorOfDetectionBasedTracker_ExtObject_new, cv_VectorOfDetectionBasedTracker_ExtObject_delete,
		cv_VectorOfDetectionBasedTracker_ExtObject_len, cv_VectorOfDetectionBasedTracker_ExtObject_is_empty,
		cv_VectorOfDetectionBasedTracker_ExtObject_capacity, cv_VectorOfDetectionBasedTracker_ExtObject_shrink_to_fit,
		cv_VectorOfDetectionBasedTracker_ExtObject_reserve, cv_VectorOfDetectionBasedTracker_ExtObject_remove,
		cv_VectorOfDetectionBasedTracker_ExtObject_swap, cv_VectorOfDetectionBasedTracker_ExtObject_clear,
		cv_VectorOfDetectionBasedTracker_ExtObject_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { crate::objdetect::DetectionBasedTracker_ExtObject::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { crate::objdetect::DetectionBasedTracker_ExtObject }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<crate::objdetect::DetectionBasedTracker_ExtObject> {
		type Arg = crate::objdetect::DetectionBasedTracker_ExtObject;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: crate::objdetect::DetectionBasedTracker_ExtObject) {
			extern "C" { fn cv_VectorOfDetectionBasedTracker_ExtObject_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfDetectionBasedTracker_ExtObject_push(self.as_raw_mut_VectorOfDetectionBasedTracker_ExtObject(), val.as_raw_mut_DetectionBasedTracker_ExtObject()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: crate::objdetect::DetectionBasedTracker_ExtObject) -> Result<()> {
			extern "C" { fn cv_VectorOfDetectionBasedTracker_ExtObject_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfDetectionBasedTracker_ExtObject_insert(self.as_raw_mut_VectorOfDetectionBasedTracker_ExtObject(), index, val.as_raw_mut_DetectionBasedTracker_ExtObject()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: crate::objdetect::DetectionBasedTracker_ExtObject) -> Result<()> {
			extern "C" { fn cv_VectorOfDetectionBasedTracker_ExtObject_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfDetectionBasedTracker_ExtObject_set(self.as_raw_mut_VectorOfDetectionBasedTracker_ExtObject(), index, val.as_raw_mut_DetectionBasedTracker_ExtObject()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: crate::objdetect::DetectionBasedTracker_ExtObject) {
			extern "C" { fn cv_VectorOfDetectionBasedTracker_ExtObject_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfDetectionBasedTracker_ExtObject_set(self.as_raw_mut_VectorOfDetectionBasedTracker_ExtObject(), index, val.as_raw_mut_DetectionBasedTracker_ExtObject())
		}
	}
	
	unsafe impl Send for core::Vector::<crate::objdetect::DetectionBasedTracker_ExtObject> {}
	
	pub type VectorOfDetectionROI = core::Vector::<crate::objdetect::DetectionROI>;
	
	impl VectorOfDetectionROI {
		pub fn as_raw_VectorOfDetectionROI(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetectionROI(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::objdetect::DetectionROI, *const c_void, *mut c_void,
		cv_VectorOfDetectionROI_new, cv_VectorOfDetectionROI_delete,
		cv_VectorOfDetectionROI_len, cv_VectorOfDetectionROI_is_empty,
		cv_VectorOfDetectionROI_capacity, cv_VectorOfDetectionROI_shrink_to_fit,
		cv_VectorOfDetectionROI_reserve, cv_VectorOfDetectionROI_remove,
		cv_VectorOfDetectionROI_swap, cv_VectorOfDetectionROI_clear,
		cv_VectorOfDetectionROI_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { crate::objdetect::DetectionROI::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { crate::objdetect::DetectionROI }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<crate::objdetect::DetectionROI> {
		type Arg = crate::objdetect::DetectionROI;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: crate::objdetect::DetectionROI) {
			extern "C" { fn cv_VectorOfDetectionROI_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfDetectionROI_push(self.as_raw_mut_VectorOfDetectionROI(), val.as_raw_mut_DetectionROI()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: crate::objdetect::DetectionROI) -> Result<()> {
			extern "C" { fn cv_VectorOfDetectionROI_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfDetectionROI_insert(self.as_raw_mut_VectorOfDetectionROI(), index, val.as_raw_mut_DetectionROI()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: crate::objdetect::DetectionROI) -> Result<()> {
			extern "C" { fn cv_VectorOfDetectionROI_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfDetectionROI_set(self.as_raw_mut_VectorOfDetectionROI(), index, val.as_raw_mut_DetectionROI()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: crate::objdetect::DetectionROI) {
			extern "C" { fn cv_VectorOfDetectionROI_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfDetectionROI_set(self.as_raw_mut_VectorOfDetectionROI(), index, val.as_raw_mut_DetectionROI())
		}
	}
	
	unsafe impl Send for core::Vector::<crate::objdetect::DetectionROI> {}
	
}
pub use objdetect_types::*;

#[cfg(feature = "contrib")]
mod phase_unwrapping_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfHistogramPhaseUnwrapping = core::Ptr::<dyn crate::phase_unwrapping::HistogramPhaseUnwrapping>;
	
	ptr_extern! { dyn crate::phase_unwrapping::HistogramPhaseUnwrapping,
		cv_PtrOfHistogramPhaseUnwrapping_delete, cv_PtrOfHistogramPhaseUnwrapping_get_inner_ptr
	}
	
	impl PtrOfHistogramPhaseUnwrapping {
		pub fn as_raw_PtrOfHistogramPhaseUnwrapping(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfHistogramPhaseUnwrapping(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfHistogramPhaseUnwrapping {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::phase_unwrapping::HistogramPhaseUnwrapping for PtrOfHistogramPhaseUnwrapping {
		fn as_raw_HistogramPhaseUnwrapping(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_HistogramPhaseUnwrapping(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::phase_unwrapping::PhaseUnwrapping for PtrOfHistogramPhaseUnwrapping {
		fn as_raw_PhaseUnwrapping(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_PhaseUnwrapping(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use phase_unwrapping_types::*;

mod photo_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAlignMTB = core::Ptr::<dyn crate::photo::AlignMTB>;
	
	ptr_extern! { dyn crate::photo::AlignMTB,
		cv_PtrOfAlignMTB_delete, cv_PtrOfAlignMTB_get_inner_ptr
	}
	
	impl PtrOfAlignMTB {
		pub fn as_raw_PtrOfAlignMTB(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfAlignMTB(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfAlignMTB {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::AlignExposures for PtrOfAlignMTB {
		fn as_raw_AlignExposures(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_AlignExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::AlignMTB for PtrOfAlignMTB {
		fn as_raw_AlignMTB(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_AlignMTB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCalibrateDebevec = core::Ptr::<dyn crate::photo::CalibrateDebevec>;
	
	ptr_extern! { dyn crate::photo::CalibrateDebevec,
		cv_PtrOfCalibrateDebevec_delete, cv_PtrOfCalibrateDebevec_get_inner_ptr
	}
	
	impl PtrOfCalibrateDebevec {
		pub fn as_raw_PtrOfCalibrateDebevec(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfCalibrateDebevec(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCalibrateDebevec {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::CalibrateCRF for PtrOfCalibrateDebevec {
		fn as_raw_CalibrateCRF(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_CalibrateCRF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::CalibrateDebevec for PtrOfCalibrateDebevec {
		fn as_raw_CalibrateDebevec(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_CalibrateDebevec(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCalibrateRobertson = core::Ptr::<dyn crate::photo::CalibrateRobertson>;
	
	ptr_extern! { dyn crate::photo::CalibrateRobertson,
		cv_PtrOfCalibrateRobertson_delete, cv_PtrOfCalibrateRobertson_get_inner_ptr
	}
	
	impl PtrOfCalibrateRobertson {
		pub fn as_raw_PtrOfCalibrateRobertson(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfCalibrateRobertson(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfCalibrateRobertson {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::CalibrateCRF for PtrOfCalibrateRobertson {
		fn as_raw_CalibrateCRF(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_CalibrateCRF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::CalibrateRobertson for PtrOfCalibrateRobertson {
		fn as_raw_CalibrateRobertson(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_CalibrateRobertson(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMergeDebevec = core::Ptr::<dyn crate::photo::MergeDebevec>;
	
	ptr_extern! { dyn crate::photo::MergeDebevec,
		cv_PtrOfMergeDebevec_delete, cv_PtrOfMergeDebevec_get_inner_ptr
	}
	
	impl PtrOfMergeDebevec {
		pub fn as_raw_PtrOfMergeDebevec(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfMergeDebevec(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfMergeDebevec {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::MergeDebevec for PtrOfMergeDebevec {
		fn as_raw_MergeDebevec(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_MergeDebevec(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::MergeExposures for PtrOfMergeDebevec {
		fn as_raw_MergeExposures(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_MergeExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMergeMertens = core::Ptr::<dyn crate::photo::MergeMertens>;
	
	ptr_extern! { dyn crate::photo::MergeMertens,
		cv_PtrOfMergeMertens_delete, cv_PtrOfMergeMertens_get_inner_ptr
	}
	
	impl PtrOfMergeMertens {
		pub fn as_raw_PtrOfMergeMertens(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfMergeMertens(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfMergeMertens {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::MergeExposures for PtrOfMergeMertens {
		fn as_raw_MergeExposures(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_MergeExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::MergeMertens for PtrOfMergeMertens {
		fn as_raw_MergeMertens(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_MergeMertens(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMergeRobertson = core::Ptr::<dyn crate::photo::MergeRobertson>;
	
	ptr_extern! { dyn crate::photo::MergeRobertson,
		cv_PtrOfMergeRobertson_delete, cv_PtrOfMergeRobertson_get_inner_ptr
	}
	
	impl PtrOfMergeRobertson {
		pub fn as_raw_PtrOfMergeRobertson(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfMergeRobertson(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfMergeRobertson {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::MergeExposures for PtrOfMergeRobertson {
		fn as_raw_MergeExposures(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_MergeExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::MergeRobertson for PtrOfMergeRobertson {
		fn as_raw_MergeRobertson(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_MergeRobertson(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemap = core::Ptr::<dyn crate::photo::Tonemap>;
	
	ptr_extern! { dyn crate::photo::Tonemap,
		cv_PtrOfTonemap_delete, cv_PtrOfTonemap_get_inner_ptr
	}
	
	impl PtrOfTonemap {
		pub fn as_raw_PtrOfTonemap(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfTonemap(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfTonemap {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::Tonemap for PtrOfTonemap {
		fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemapDrago = core::Ptr::<dyn crate::photo::TonemapDrago>;
	
	ptr_extern! { dyn crate::photo::TonemapDrago,
		cv_PtrOfTonemapDrago_delete, cv_PtrOfTonemapDrago_get_inner_ptr
	}
	
	impl PtrOfTonemapDrago {
		pub fn as_raw_PtrOfTonemapDrago(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfTonemapDrago(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfTonemapDrago {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::Tonemap for PtrOfTonemapDrago {
		fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::TonemapDrago for PtrOfTonemapDrago {
		fn as_raw_TonemapDrago(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_TonemapDrago(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemapDurand = core::Ptr::<dyn crate::photo::TonemapDurand>;
	
	ptr_extern! { dyn crate::photo::TonemapDurand,
		cv_PtrOfTonemapDurand_delete, cv_PtrOfTonemapDurand_get_inner_ptr
	}
	
	impl PtrOfTonemapDurand {
		pub fn as_raw_PtrOfTonemapDurand(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfTonemapDurand(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfTonemapDurand {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::Tonemap for PtrOfTonemapDurand {
		fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::TonemapDurand for PtrOfTonemapDurand {
		fn as_raw_TonemapDurand(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_TonemapDurand(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemapMantiuk = core::Ptr::<dyn crate::photo::TonemapMantiuk>;
	
	ptr_extern! { dyn crate::photo::TonemapMantiuk,
		cv_PtrOfTonemapMantiuk_delete, cv_PtrOfTonemapMantiuk_get_inner_ptr
	}
	
	impl PtrOfTonemapMantiuk {
		pub fn as_raw_PtrOfTonemapMantiuk(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfTonemapMantiuk(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfTonemapMantiuk {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::Tonemap for PtrOfTonemapMantiuk {
		fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::TonemapMantiuk for PtrOfTonemapMantiuk {
		fn as_raw_TonemapMantiuk(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_TonemapMantiuk(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemapReinhard = core::Ptr::<dyn crate::photo::TonemapReinhard>;
	
	ptr_extern! { dyn crate::photo::TonemapReinhard,
		cv_PtrOfTonemapReinhard_delete, cv_PtrOfTonemapReinhard_get_inner_ptr
	}
	
	impl PtrOfTonemapReinhard {
		pub fn as_raw_PtrOfTonemapReinhard(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfTonemapReinhard(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfTonemapReinhard {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::Tonemap for PtrOfTonemapReinhard {
		fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::TonemapReinhard for PtrOfTonemapReinhard {
		fn as_raw_TonemapReinhard(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_TonemapReinhard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
pub use photo_types::*;

#[cfg(feature = "contrib")]
mod plot_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfPlot2d = core::Ptr::<dyn crate::plot::Plot2d>;
	
	ptr_extern! { dyn crate::plot::Plot2d,
		cv_PtrOfPlot2d_delete, cv_PtrOfPlot2d_get_inner_ptr
	}
	
	impl PtrOfPlot2d {
		pub fn as_raw_PtrOfPlot2d(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfPlot2d(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfPlot2d {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::plot::Plot2d for PtrOfPlot2d {
		fn as_raw_Plot2d(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Plot2d(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use plot_types::*;

#[cfg(feature = "contrib")]
mod sfm_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfSFMLibmvEuclideanReconstruction = core::Ptr::<dyn crate::sfm::SFMLibmvEuclideanReconstruction>;
	
	ptr_extern! { dyn crate::sfm::SFMLibmvEuclideanReconstruction,
		cv_PtrOfSFMLibmvEuclideanReconstruction_delete, cv_PtrOfSFMLibmvEuclideanReconstruction_get_inner_ptr
	}
	
	impl PtrOfSFMLibmvEuclideanReconstruction {
		pub fn as_raw_PtrOfSFMLibmvEuclideanReconstruction(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfSFMLibmvEuclideanReconstruction(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::sfm::BaseSFM for PtrOfSFMLibmvEuclideanReconstruction {
		fn as_raw_BaseSFM(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BaseSFM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::sfm::SFMLibmvEuclideanReconstruction for PtrOfSFMLibmvEuclideanReconstruction {
		fn as_raw_SFMLibmvEuclideanReconstruction(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_SFMLibmvEuclideanReconstruction(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use sfm_types::*;

mod shape_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAffineTransformer = core::Ptr::<dyn crate::shape::AffineTransformer>;
	
	ptr_extern! { dyn crate::shape::AffineTransformer,
		cv_PtrOfAffineTransformer_delete, cv_PtrOfAffineTransformer_get_inner_ptr
	}
	
	impl PtrOfAffineTransformer {
		pub fn as_raw_PtrOfAffineTransformer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfAffineTransformer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::AffineTransformer for PtrOfAffineTransformer {
		fn as_raw_AffineTransformer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_AffineTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfAffineTransformer {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::ShapeTransformer for PtrOfAffineTransformer {
		fn as_raw_ShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfHausdorffDistanceExtractor = core::Ptr::<dyn crate::shape::HausdorffDistanceExtractor>;
	
	ptr_extern! { dyn crate::shape::HausdorffDistanceExtractor,
		cv_PtrOfHausdorffDistanceExtractor_delete, cv_PtrOfHausdorffDistanceExtractor_get_inner_ptr
	}
	
	impl PtrOfHausdorffDistanceExtractor {
		pub fn as_raw_PtrOfHausdorffDistanceExtractor(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfHausdorffDistanceExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfHausdorffDistanceExtractor {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::HausdorffDistanceExtractor for PtrOfHausdorffDistanceExtractor {
		fn as_raw_HausdorffDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_HausdorffDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::ShapeDistanceExtractor for PtrOfHausdorffDistanceExtractor {
		fn as_raw_ShapeDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_ShapeDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfHistogramCostExtractor = core::Ptr::<dyn crate::shape::HistogramCostExtractor>;
	
	ptr_extern! { dyn crate::shape::HistogramCostExtractor,
		cv_PtrOfHistogramCostExtractor_delete, cv_PtrOfHistogramCostExtractor_get_inner_ptr
	}
	
	impl PtrOfHistogramCostExtractor {
		pub fn as_raw_PtrOfHistogramCostExtractor(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfHistogramCostExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfHistogramCostExtractor {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::HistogramCostExtractor for PtrOfHistogramCostExtractor {
		fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfShapeContextDistanceExtractor = core::Ptr::<dyn crate::shape::ShapeContextDistanceExtractor>;
	
	ptr_extern! { dyn crate::shape::ShapeContextDistanceExtractor,
		cv_PtrOfShapeContextDistanceExtractor_delete, cv_PtrOfShapeContextDistanceExtractor_get_inner_ptr
	}
	
	impl PtrOfShapeContextDistanceExtractor {
		pub fn as_raw_PtrOfShapeContextDistanceExtractor(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfShapeContextDistanceExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfShapeContextDistanceExtractor {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::ShapeContextDistanceExtractor for PtrOfShapeContextDistanceExtractor {
		fn as_raw_ShapeContextDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_ShapeContextDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::ShapeDistanceExtractor for PtrOfShapeContextDistanceExtractor {
		fn as_raw_ShapeDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_ShapeDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfShapeTransformer = core::Ptr::<dyn crate::shape::ShapeTransformer>;
	
	ptr_extern! { dyn crate::shape::ShapeTransformer,
		cv_PtrOfShapeTransformer_delete, cv_PtrOfShapeTransformer_get_inner_ptr
	}
	
	impl PtrOfShapeTransformer {
		pub fn as_raw_PtrOfShapeTransformer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfShapeTransformer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfShapeTransformer {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::ShapeTransformer for PtrOfShapeTransformer {
		fn as_raw_ShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfThinPlateSplineShapeTransformer = core::Ptr::<dyn crate::shape::ThinPlateSplineShapeTransformer>;
	
	ptr_extern! { dyn crate::shape::ThinPlateSplineShapeTransformer,
		cv_PtrOfThinPlateSplineShapeTransformer_delete, cv_PtrOfThinPlateSplineShapeTransformer_get_inner_ptr
	}
	
	impl PtrOfThinPlateSplineShapeTransformer {
		pub fn as_raw_PtrOfThinPlateSplineShapeTransformer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfThinPlateSplineShapeTransformer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfThinPlateSplineShapeTransformer {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::ShapeTransformer for PtrOfThinPlateSplineShapeTransformer {
		fn as_raw_ShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::ThinPlateSplineShapeTransformer for PtrOfThinPlateSplineShapeTransformer {
		fn as_raw_ThinPlateSplineShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_ThinPlateSplineShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
pub use shape_types::*;

mod stitching_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfDetail_Blender = core::Ptr::<crate::stitching::Detail_Blender>;
	
	ptr_extern! { crate::stitching::Detail_Blender,
		cv_PtrOfDetail_Blender_delete, cv_PtrOfDetail_Blender_get_inner_ptr
	}
	
	impl PtrOfDetail_Blender {
		pub fn as_raw_PtrOfDetail_Blender(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfDetail_Blender(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlenderTrait for PtrOfDetail_Blender {
		fn as_raw_Detail_Blender(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_BundleAdjusterBase = core::Ptr::<dyn crate::stitching::Detail_BundleAdjusterBase>;
	
	ptr_extern! { dyn crate::stitching::Detail_BundleAdjusterBase,
		cv_PtrOfDetail_BundleAdjusterBase_delete, cv_PtrOfDetail_BundleAdjusterBase_get_inner_ptr
	}
	
	impl PtrOfDetail_BundleAdjusterBase {
		pub fn as_raw_PtrOfDetail_BundleAdjusterBase(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfDetail_BundleAdjusterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBase for PtrOfDetail_BundleAdjusterBase {
		fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_Estimator for PtrOfDetail_BundleAdjusterBase {
		fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_ExposureCompensator = core::Ptr::<dyn crate::stitching::Detail_ExposureCompensator>;
	
	ptr_extern! { dyn crate::stitching::Detail_ExposureCompensator,
		cv_PtrOfDetail_ExposureCompensator_delete, cv_PtrOfDetail_ExposureCompensator_get_inner_ptr
	}
	
	impl PtrOfDetail_ExposureCompensator {
		pub fn as_raw_PtrOfDetail_ExposureCompensator(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfDetail_ExposureCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ExposureCompensator for PtrOfDetail_ExposureCompensator {
		fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_FeaturesFinder = core::Ptr::<dyn crate::stitching::Detail_FeaturesFinder>;
	
	ptr_extern! { dyn crate::stitching::Detail_FeaturesFinder,
		cv_PtrOfDetail_FeaturesFinder_delete, cv_PtrOfDetail_FeaturesFinder_get_inner_ptr
	}
	
	impl PtrOfDetail_FeaturesFinder {
		pub fn as_raw_PtrOfDetail_FeaturesFinder(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfDetail_FeaturesFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_FeaturesFinder for PtrOfDetail_FeaturesFinder {
		fn as_raw_Detail_FeaturesFinder(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Detail_FeaturesFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_FeaturesMatcher = core::Ptr::<dyn crate::stitching::Detail_FeaturesMatcher>;
	
	ptr_extern! { dyn crate::stitching::Detail_FeaturesMatcher,
		cv_PtrOfDetail_FeaturesMatcher_delete, cv_PtrOfDetail_FeaturesMatcher_get_inner_ptr
	}
	
	impl PtrOfDetail_FeaturesMatcher {
		pub fn as_raw_PtrOfDetail_FeaturesMatcher(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfDetail_FeaturesMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_FeaturesMatcher for PtrOfDetail_FeaturesMatcher {
		fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_RotationWarper = core::Ptr::<dyn crate::stitching::Detail_RotationWarper>;
	
	ptr_extern! { dyn crate::stitching::Detail_RotationWarper,
		cv_PtrOfDetail_RotationWarper_delete, cv_PtrOfDetail_RotationWarper_get_inner_ptr
	}
	
	impl PtrOfDetail_RotationWarper {
		pub fn as_raw_PtrOfDetail_RotationWarper(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfDetail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_RotationWarper for PtrOfDetail_RotationWarper {
		fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_SeamFinder = core::Ptr::<dyn crate::stitching::Detail_SeamFinder>;
	
	ptr_extern! { dyn crate::stitching::Detail_SeamFinder,
		cv_PtrOfDetail_SeamFinder_delete, cv_PtrOfDetail_SeamFinder_get_inner_ptr
	}
	
	impl PtrOfDetail_SeamFinder {
		pub fn as_raw_PtrOfDetail_SeamFinder(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfDetail_SeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_SeamFinder for PtrOfDetail_SeamFinder {
		fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStitcher = core::Ptr::<crate::stitching::Stitcher>;
	
	ptr_extern! { crate::stitching::Stitcher,
		cv_PtrOfStitcher_delete, cv_PtrOfStitcher_get_inner_ptr
	}
	
	impl PtrOfStitcher {
		pub fn as_raw_PtrOfStitcher(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfStitcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::StitcherTrait for PtrOfStitcher {
		fn as_raw_Stitcher(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Stitcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfWarperCreator = core::Ptr::<dyn crate::stitching::WarperCreator>;
	
	ptr_extern! { dyn crate::stitching::WarperCreator,
		cv_PtrOfWarperCreator_delete, cv_PtrOfWarperCreator_get_inner_ptr
	}
	
	impl PtrOfWarperCreator {
		pub fn as_raw_PtrOfWarperCreator(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfWarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreator for PtrOfWarperCreator {
		fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfDetail_CameraParams = core::Vector::<crate::stitching::Detail_CameraParams>;
	
	impl VectorOfDetail_CameraParams {
		pub fn as_raw_VectorOfDetail_CameraParams(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetail_CameraParams(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::stitching::Detail_CameraParams, *const c_void, *mut c_void,
		cv_VectorOfDetail_CameraParams_new, cv_VectorOfDetail_CameraParams_delete,
		cv_VectorOfDetail_CameraParams_len, cv_VectorOfDetail_CameraParams_is_empty,
		cv_VectorOfDetail_CameraParams_capacity, cv_VectorOfDetail_CameraParams_shrink_to_fit,
		cv_VectorOfDetail_CameraParams_reserve, cv_VectorOfDetail_CameraParams_remove,
		cv_VectorOfDetail_CameraParams_swap, cv_VectorOfDetail_CameraParams_clear,
		cv_VectorOfDetail_CameraParams_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { crate::stitching::Detail_CameraParams::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { crate::stitching::Detail_CameraParams }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<crate::stitching::Detail_CameraParams> {
		type Arg = crate::stitching::Detail_CameraParams;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: crate::stitching::Detail_CameraParams) {
			extern "C" { fn cv_VectorOfDetail_CameraParams_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfDetail_CameraParams_push(self.as_raw_mut_VectorOfDetail_CameraParams(), val.as_raw_mut_Detail_CameraParams()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: crate::stitching::Detail_CameraParams) -> Result<()> {
			extern "C" { fn cv_VectorOfDetail_CameraParams_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfDetail_CameraParams_insert(self.as_raw_mut_VectorOfDetail_CameraParams(), index, val.as_raw_mut_Detail_CameraParams()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: crate::stitching::Detail_CameraParams) -> Result<()> {
			extern "C" { fn cv_VectorOfDetail_CameraParams_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfDetail_CameraParams_set(self.as_raw_mut_VectorOfDetail_CameraParams(), index, val.as_raw_mut_Detail_CameraParams()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: crate::stitching::Detail_CameraParams) {
			extern "C" { fn cv_VectorOfDetail_CameraParams_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfDetail_CameraParams_set(self.as_raw_mut_VectorOfDetail_CameraParams(), index, val.as_raw_mut_Detail_CameraParams())
		}
	}
	
	unsafe impl Send for core::Vector::<crate::stitching::Detail_CameraParams> {}
	
	pub type VectorOfDetail_ImageFeatures = core::Vector::<crate::stitching::Detail_ImageFeatures>;
	
	impl VectorOfDetail_ImageFeatures {
		pub fn as_raw_VectorOfDetail_ImageFeatures(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetail_ImageFeatures(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::stitching::Detail_ImageFeatures, *const c_void, *mut c_void,
		cv_VectorOfDetail_ImageFeatures_new, cv_VectorOfDetail_ImageFeatures_delete,
		cv_VectorOfDetail_ImageFeatures_len, cv_VectorOfDetail_ImageFeatures_is_empty,
		cv_VectorOfDetail_ImageFeatures_capacity, cv_VectorOfDetail_ImageFeatures_shrink_to_fit,
		cv_VectorOfDetail_ImageFeatures_reserve, cv_VectorOfDetail_ImageFeatures_remove,
		cv_VectorOfDetail_ImageFeatures_swap, cv_VectorOfDetail_ImageFeatures_clear,
		cv_VectorOfDetail_ImageFeatures_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { crate::stitching::Detail_ImageFeatures::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { crate::stitching::Detail_ImageFeatures }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<crate::stitching::Detail_ImageFeatures> {
		type Arg = crate::stitching::Detail_ImageFeatures;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: crate::stitching::Detail_ImageFeatures) {
			extern "C" { fn cv_VectorOfDetail_ImageFeatures_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfDetail_ImageFeatures_push(self.as_raw_mut_VectorOfDetail_ImageFeatures(), val.as_raw_mut_Detail_ImageFeatures()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: crate::stitching::Detail_ImageFeatures) -> Result<()> {
			extern "C" { fn cv_VectorOfDetail_ImageFeatures_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfDetail_ImageFeatures_insert(self.as_raw_mut_VectorOfDetail_ImageFeatures(), index, val.as_raw_mut_Detail_ImageFeatures()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: crate::stitching::Detail_ImageFeatures) -> Result<()> {
			extern "C" { fn cv_VectorOfDetail_ImageFeatures_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfDetail_ImageFeatures_set(self.as_raw_mut_VectorOfDetail_ImageFeatures(), index, val.as_raw_mut_Detail_ImageFeatures()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: crate::stitching::Detail_ImageFeatures) {
			extern "C" { fn cv_VectorOfDetail_ImageFeatures_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfDetail_ImageFeatures_set(self.as_raw_mut_VectorOfDetail_ImageFeatures(), index, val.as_raw_mut_Detail_ImageFeatures())
		}
	}
	
	unsafe impl Send for core::Vector::<crate::stitching::Detail_ImageFeatures> {}
	
	pub type VectorOfDetail_MatchesInfo = core::Vector::<crate::stitching::Detail_MatchesInfo>;
	
	impl VectorOfDetail_MatchesInfo {
		pub fn as_raw_VectorOfDetail_MatchesInfo(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetail_MatchesInfo(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::stitching::Detail_MatchesInfo, *const c_void, *mut c_void,
		cv_VectorOfDetail_MatchesInfo_new, cv_VectorOfDetail_MatchesInfo_delete,
		cv_VectorOfDetail_MatchesInfo_len, cv_VectorOfDetail_MatchesInfo_is_empty,
		cv_VectorOfDetail_MatchesInfo_capacity, cv_VectorOfDetail_MatchesInfo_shrink_to_fit,
		cv_VectorOfDetail_MatchesInfo_reserve, cv_VectorOfDetail_MatchesInfo_remove,
		cv_VectorOfDetail_MatchesInfo_swap, cv_VectorOfDetail_MatchesInfo_clear,
		cv_VectorOfDetail_MatchesInfo_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { crate::stitching::Detail_MatchesInfo::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { crate::stitching::Detail_MatchesInfo }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<crate::stitching::Detail_MatchesInfo> {
		type Arg = crate::stitching::Detail_MatchesInfo;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: crate::stitching::Detail_MatchesInfo) {
			extern "C" { fn cv_VectorOfDetail_MatchesInfo_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfDetail_MatchesInfo_push(self.as_raw_mut_VectorOfDetail_MatchesInfo(), val.as_raw_mut_Detail_MatchesInfo()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: crate::stitching::Detail_MatchesInfo) -> Result<()> {
			extern "C" { fn cv_VectorOfDetail_MatchesInfo_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfDetail_MatchesInfo_insert(self.as_raw_mut_VectorOfDetail_MatchesInfo(), index, val.as_raw_mut_Detail_MatchesInfo()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: crate::stitching::Detail_MatchesInfo) -> Result<()> {
			extern "C" { fn cv_VectorOfDetail_MatchesInfo_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfDetail_MatchesInfo_set(self.as_raw_mut_VectorOfDetail_MatchesInfo(), index, val.as_raw_mut_Detail_MatchesInfo()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: crate::stitching::Detail_MatchesInfo) {
			extern "C" { fn cv_VectorOfDetail_MatchesInfo_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfDetail_MatchesInfo_set(self.as_raw_mut_VectorOfDetail_MatchesInfo(), index, val.as_raw_mut_Detail_MatchesInfo())
		}
	}
	
	unsafe impl Send for core::Vector::<crate::stitching::Detail_MatchesInfo> {}
	
}
pub use stitching_types::*;

#[cfg(feature = "contrib")]
mod structured_light_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfGrayCodePattern = core::Ptr::<dyn crate::structured_light::GrayCodePattern>;
	
	ptr_extern! { dyn crate::structured_light::GrayCodePattern,
		cv_PtrOfGrayCodePattern_delete, cv_PtrOfGrayCodePattern_get_inner_ptr
	}
	
	impl PtrOfGrayCodePattern {
		pub fn as_raw_PtrOfGrayCodePattern(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfGrayCodePattern(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfGrayCodePattern {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::structured_light::GrayCodePattern for PtrOfGrayCodePattern {
		fn as_raw_GrayCodePattern(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_GrayCodePattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::structured_light::StructuredLightPattern for PtrOfGrayCodePattern {
		fn as_raw_StructuredLightPattern(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_StructuredLightPattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSinusoidalPattern = core::Ptr::<dyn crate::structured_light::SinusoidalPattern>;
	
	ptr_extern! { dyn crate::structured_light::SinusoidalPattern,
		cv_PtrOfSinusoidalPattern_delete, cv_PtrOfSinusoidalPattern_get_inner_ptr
	}
	
	impl PtrOfSinusoidalPattern {
		pub fn as_raw_PtrOfSinusoidalPattern(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfSinusoidalPattern(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSinusoidalPattern {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::structured_light::SinusoidalPattern for PtrOfSinusoidalPattern {
		fn as_raw_SinusoidalPattern(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_SinusoidalPattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::structured_light::StructuredLightPattern for PtrOfSinusoidalPattern {
		fn as_raw_StructuredLightPattern(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_StructuredLightPattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use structured_light_types::*;

mod superres_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfSuperres_BroxOpticalFlow = core::Ptr::<dyn crate::superres::Superres_BroxOpticalFlow>;
	
	ptr_extern! { dyn crate::superres::Superres_BroxOpticalFlow,
		cv_PtrOfSuperres_BroxOpticalFlow_delete, cv_PtrOfSuperres_BroxOpticalFlow_get_inner_ptr
	}
	
	impl PtrOfSuperres_BroxOpticalFlow {
		pub fn as_raw_PtrOfSuperres_BroxOpticalFlow(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfSuperres_BroxOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperres_BroxOpticalFlow {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_BroxOpticalFlow for PtrOfSuperres_BroxOpticalFlow {
		fn as_raw_Superres_BroxOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Superres_BroxOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_BroxOpticalFlow {
		fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_DenseOpticalFlowExt = core::Ptr::<dyn crate::superres::Superres_DenseOpticalFlowExt>;
	
	ptr_extern! { dyn crate::superres::Superres_DenseOpticalFlowExt,
		cv_PtrOfSuperres_DenseOpticalFlowExt_delete, cv_PtrOfSuperres_DenseOpticalFlowExt_get_inner_ptr
	}
	
	impl PtrOfSuperres_DenseOpticalFlowExt {
		pub fn as_raw_PtrOfSuperres_DenseOpticalFlowExt(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfSuperres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperres_DenseOpticalFlowExt {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_DenseOpticalFlowExt {
		fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_DualTVL1OpticalFlow = core::Ptr::<dyn crate::superres::Superres_DualTVL1OpticalFlow>;
	
	ptr_extern! { dyn crate::superres::Superres_DualTVL1OpticalFlow,
		cv_PtrOfSuperres_DualTVL1OpticalFlow_delete, cv_PtrOfSuperres_DualTVL1OpticalFlow_get_inner_ptr
	}
	
	impl PtrOfSuperres_DualTVL1OpticalFlow {
		pub fn as_raw_PtrOfSuperres_DualTVL1OpticalFlow(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfSuperres_DualTVL1OpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperres_DualTVL1OpticalFlow {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_DualTVL1OpticalFlow {
		fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_DualTVL1OpticalFlow for PtrOfSuperres_DualTVL1OpticalFlow {
		fn as_raw_Superres_DualTVL1OpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Superres_DualTVL1OpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_FarnebackOpticalFlow = core::Ptr::<dyn crate::superres::Superres_FarnebackOpticalFlow>;
	
	ptr_extern! { dyn crate::superres::Superres_FarnebackOpticalFlow,
		cv_PtrOfSuperres_FarnebackOpticalFlow_delete, cv_PtrOfSuperres_FarnebackOpticalFlow_get_inner_ptr
	}
	
	impl PtrOfSuperres_FarnebackOpticalFlow {
		pub fn as_raw_PtrOfSuperres_FarnebackOpticalFlow(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfSuperres_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperres_FarnebackOpticalFlow {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_FarnebackOpticalFlow {
		fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_FarnebackOpticalFlow for PtrOfSuperres_FarnebackOpticalFlow {
		fn as_raw_Superres_FarnebackOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Superres_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_FrameSource = core::Ptr::<dyn crate::superres::Superres_FrameSource>;
	
	ptr_extern! { dyn crate::superres::Superres_FrameSource,
		cv_PtrOfSuperres_FrameSource_delete, cv_PtrOfSuperres_FrameSource_get_inner_ptr
	}
	
	impl PtrOfSuperres_FrameSource {
		pub fn as_raw_PtrOfSuperres_FrameSource(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfSuperres_FrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::superres::Superres_FrameSource for PtrOfSuperres_FrameSource {
		fn as_raw_Superres_FrameSource(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Superres_FrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_PyrLKOpticalFlow = core::Ptr::<dyn crate::superres::Superres_PyrLKOpticalFlow>;
	
	ptr_extern! { dyn crate::superres::Superres_PyrLKOpticalFlow,
		cv_PtrOfSuperres_PyrLKOpticalFlow_delete, cv_PtrOfSuperres_PyrLKOpticalFlow_get_inner_ptr
	}
	
	impl PtrOfSuperres_PyrLKOpticalFlow {
		pub fn as_raw_PtrOfSuperres_PyrLKOpticalFlow(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfSuperres_PyrLKOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperres_PyrLKOpticalFlow {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_PyrLKOpticalFlow {
		fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_PyrLKOpticalFlow for PtrOfSuperres_PyrLKOpticalFlow {
		fn as_raw_Superres_PyrLKOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Superres_PyrLKOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_SuperResolution = core::Ptr::<dyn crate::superres::Superres_SuperResolution>;
	
	ptr_extern! { dyn crate::superres::Superres_SuperResolution,
		cv_PtrOfSuperres_SuperResolution_delete, cv_PtrOfSuperres_SuperResolution_get_inner_ptr
	}
	
	impl PtrOfSuperres_SuperResolution {
		pub fn as_raw_PtrOfSuperres_SuperResolution(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfSuperres_SuperResolution(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperres_SuperResolution {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_FrameSource for PtrOfSuperres_SuperResolution {
		fn as_raw_Superres_FrameSource(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Superres_FrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_SuperResolution for PtrOfSuperres_SuperResolution {
		fn as_raw_Superres_SuperResolution(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Superres_SuperResolution(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
pub use superres_types::*;

#[cfg(feature = "contrib")]
mod surface_matching_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfPose3D = core::Ptr::<crate::surface_matching::Pose3D>;
	
	ptr_extern! { crate::surface_matching::Pose3D,
		cv_PtrOfPose3D_delete, cv_PtrOfPose3D_get_inner_ptr
	}
	
	impl PtrOfPose3D {
		pub fn as_raw_PtrOfPose3D(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfPose3D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::surface_matching::Pose3DTrait for PtrOfPose3D {
		fn as_raw_Pose3D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Pose3D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPoseCluster3D = core::Ptr::<crate::surface_matching::PoseCluster3D>;
	
	ptr_extern! { crate::surface_matching::PoseCluster3D,
		cv_PtrOfPoseCluster3D_delete, cv_PtrOfPoseCluster3D_get_inner_ptr
	}
	
	impl PtrOfPoseCluster3D {
		pub fn as_raw_PtrOfPoseCluster3D(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfPoseCluster3D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::surface_matching::PoseCluster3DTrait for PtrOfPoseCluster3D {
		fn as_raw_PoseCluster3D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_PoseCluster3D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfPose3DPtr = core::Vector::<crate::surface_matching::Pose3DPtr>;
	
	impl VectorOfPose3DPtr {
		pub fn as_raw_VectorOfPose3DPtr(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPose3DPtr(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::surface_matching::Pose3DPtr, *const c_void, *mut c_void,
		cv_VectorOfPose3DPtr_new, cv_VectorOfPose3DPtr_delete,
		cv_VectorOfPose3DPtr_len, cv_VectorOfPose3DPtr_is_empty,
		cv_VectorOfPose3DPtr_capacity, cv_VectorOfPose3DPtr_shrink_to_fit,
		cv_VectorOfPose3DPtr_reserve, cv_VectorOfPose3DPtr_remove,
		cv_VectorOfPose3DPtr_swap, cv_VectorOfPose3DPtr_clear,
		cv_VectorOfPose3DPtr_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { core::Ptr::<crate::surface_matching::Pose3D>::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { crate::surface_matching::Pose3DPtr }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<crate::surface_matching::Pose3DPtr> {
		type Arg = crate::surface_matching::Pose3DPtr;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: crate::surface_matching::Pose3DPtr) {
			extern "C" { fn cv_VectorOfPose3DPtr_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfPose3DPtr_push(self.as_raw_mut_VectorOfPose3DPtr(), val.as_raw_mut_PtrOfPose3D()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: crate::surface_matching::Pose3DPtr) -> Result<()> {
			extern "C" { fn cv_VectorOfPose3DPtr_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfPose3DPtr_insert(self.as_raw_mut_VectorOfPose3DPtr(), index, val.as_raw_mut_PtrOfPose3D()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: crate::surface_matching::Pose3DPtr) -> Result<()> {
			extern "C" { fn cv_VectorOfPose3DPtr_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfPose3DPtr_set(self.as_raw_mut_VectorOfPose3DPtr(), index, val.as_raw_mut_PtrOfPose3D()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: crate::surface_matching::Pose3DPtr) {
			extern "C" { fn cv_VectorOfPose3DPtr_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfPose3DPtr_set(self.as_raw_mut_VectorOfPose3DPtr(), index, val.as_raw_mut_PtrOfPose3D())
		}
	}
	
	unsafe impl Send for core::Vector::<crate::surface_matching::Pose3DPtr> {}
	
}
#[cfg(feature = "contrib")]
pub use surface_matching_types::*;

#[cfg(feature = "contrib")]
mod text_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfERFilter = core::Ptr::<dyn crate::text::ERFilter>;
	
	ptr_extern! { dyn crate::text::ERFilter,
		cv_PtrOfERFilter_delete, cv_PtrOfERFilter_get_inner_ptr
	}
	
	impl PtrOfERFilter {
		pub fn as_raw_PtrOfERFilter(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfERFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfERFilter {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::text::ERFilter for PtrOfERFilter {
		fn as_raw_ERFilter(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_ERFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfERFilter_Callback = core::Ptr::<dyn crate::text::ERFilter_Callback>;
	
	ptr_extern! { dyn crate::text::ERFilter_Callback,
		cv_PtrOfERFilter_Callback_delete, cv_PtrOfERFilter_Callback_get_inner_ptr
	}
	
	impl PtrOfERFilter_Callback {
		pub fn as_raw_PtrOfERFilter_Callback(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfERFilter_Callback(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::ERFilter_Callback for PtrOfERFilter_Callback {
		fn as_raw_ERFilter_Callback(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_ERFilter_Callback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRBeamSearchDecoder = core::Ptr::<crate::text::OCRBeamSearchDecoder>;
	
	ptr_extern! { crate::text::OCRBeamSearchDecoder,
		cv_PtrOfOCRBeamSearchDecoder_delete, cv_PtrOfOCRBeamSearchDecoder_get_inner_ptr
	}
	
	impl PtrOfOCRBeamSearchDecoder {
		pub fn as_raw_PtrOfOCRBeamSearchDecoder(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfOCRBeamSearchDecoder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::BaseOCR for PtrOfOCRBeamSearchDecoder {
		fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::text::OCRBeamSearchDecoderTrait for PtrOfOCRBeamSearchDecoder {
		fn as_raw_OCRBeamSearchDecoder(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_OCRBeamSearchDecoder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRBeamSearchDecoder_ClassifierCallback = core::Ptr::<crate::text::OCRBeamSearchDecoder_ClassifierCallback>;
	
	ptr_extern! { crate::text::OCRBeamSearchDecoder_ClassifierCallback,
		cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_delete, cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_get_inner_ptr
	}
	
	impl PtrOfOCRBeamSearchDecoder_ClassifierCallback {
		pub fn as_raw_PtrOfOCRBeamSearchDecoder_ClassifierCallback(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfOCRBeamSearchDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::OCRBeamSearchDecoder_ClassifierCallbackTrait for PtrOfOCRBeamSearchDecoder_ClassifierCallback {
		fn as_raw_OCRBeamSearchDecoder_ClassifierCallback(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_OCRBeamSearchDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRHMMDecoder = core::Ptr::<crate::text::OCRHMMDecoder>;
	
	ptr_extern! { crate::text::OCRHMMDecoder,
		cv_PtrOfOCRHMMDecoder_delete, cv_PtrOfOCRHMMDecoder_get_inner_ptr
	}
	
	impl PtrOfOCRHMMDecoder {
		pub fn as_raw_PtrOfOCRHMMDecoder(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfOCRHMMDecoder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::BaseOCR for PtrOfOCRHMMDecoder {
		fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::text::OCRHMMDecoderTrait for PtrOfOCRHMMDecoder {
		fn as_raw_OCRHMMDecoder(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_OCRHMMDecoder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRHMMDecoder_ClassifierCallback = core::Ptr::<crate::text::OCRHMMDecoder_ClassifierCallback>;
	
	ptr_extern! { crate::text::OCRHMMDecoder_ClassifierCallback,
		cv_PtrOfOCRHMMDecoder_ClassifierCallback_delete, cv_PtrOfOCRHMMDecoder_ClassifierCallback_get_inner_ptr
	}
	
	impl PtrOfOCRHMMDecoder_ClassifierCallback {
		pub fn as_raw_PtrOfOCRHMMDecoder_ClassifierCallback(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfOCRHMMDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::OCRHMMDecoder_ClassifierCallbackTrait for PtrOfOCRHMMDecoder_ClassifierCallback {
		fn as_raw_OCRHMMDecoder_ClassifierCallback(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_OCRHMMDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRTesseract = core::Ptr::<dyn crate::text::OCRTesseract>;
	
	ptr_extern! { dyn crate::text::OCRTesseract,
		cv_PtrOfOCRTesseract_delete, cv_PtrOfOCRTesseract_get_inner_ptr
	}
	
	impl PtrOfOCRTesseract {
		pub fn as_raw_PtrOfOCRTesseract(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfOCRTesseract(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::BaseOCR for PtrOfOCRTesseract {
		fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::text::OCRTesseract for PtrOfOCRTesseract {
		fn as_raw_OCRTesseract(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_OCRTesseract(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfERStat = core::Vector::<crate::text::ERStat>;
	
	impl VectorOfERStat {
		pub fn as_raw_VectorOfERStat(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfERStat(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::text::ERStat, *const c_void, *mut c_void,
		cv_VectorOfERStat_new, cv_VectorOfERStat_delete,
		cv_VectorOfERStat_len, cv_VectorOfERStat_is_empty,
		cv_VectorOfERStat_capacity, cv_VectorOfERStat_shrink_to_fit,
		cv_VectorOfERStat_reserve, cv_VectorOfERStat_remove,
		cv_VectorOfERStat_swap, cv_VectorOfERStat_clear,
		cv_VectorOfERStat_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { crate::text::ERStat::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { crate::text::ERStat }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<crate::text::ERStat> {
		type Arg = crate::text::ERStat;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: crate::text::ERStat) {
			extern "C" { fn cv_VectorOfERStat_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfERStat_push(self.as_raw_mut_VectorOfERStat(), val.as_raw_mut_ERStat()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: crate::text::ERStat) -> Result<()> {
			extern "C" { fn cv_VectorOfERStat_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfERStat_insert(self.as_raw_mut_VectorOfERStat(), index, val.as_raw_mut_ERStat()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: crate::text::ERStat) -> Result<()> {
			extern "C" { fn cv_VectorOfERStat_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfERStat_set(self.as_raw_mut_VectorOfERStat(), index, val.as_raw_mut_ERStat()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: crate::text::ERStat) {
			extern "C" { fn cv_VectorOfERStat_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfERStat_set(self.as_raw_mut_VectorOfERStat(), index, val.as_raw_mut_ERStat())
		}
	}
	
	unsafe impl Send for core::Vector::<crate::text::ERStat> {}
	
	pub type VectorOfVectorOfERStat = core::Vector::<core::Vector::<crate::text::ERStat>>;
	
	impl VectorOfVectorOfERStat {
		pub fn as_raw_VectorOfVectorOfERStat(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfERStat(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector::<crate::text::ERStat>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfERStat_new, cv_VectorOfVectorOfERStat_delete,
		cv_VectorOfVectorOfERStat_len, cv_VectorOfVectorOfERStat_is_empty,
		cv_VectorOfVectorOfERStat_capacity, cv_VectorOfVectorOfERStat_shrink_to_fit,
		cv_VectorOfVectorOfERStat_reserve, cv_VectorOfVectorOfERStat_remove,
		cv_VectorOfVectorOfERStat_swap, cv_VectorOfVectorOfERStat_clear,
		cv_VectorOfVectorOfERStat_get -> sys::Result<*mut c_void>,
		ret_map: .map(|ptr| { core::Vector::<crate::text::ERStat>::from_raw(ptr) })
	}
	vector_non_copy_or_bool! { core::Vector::<crate::text::ERStat> }
	
	impl<'i> core::VectorTrait<'i> for core::Vector::<core::Vector::<crate::text::ERStat>> {
		type Arg = core::Vector::<crate::text::ERStat>;
	
		fn with_capacity(capacity: size_t) -> Self { Self::with_capacity(capacity) }
	
		#[inline]
		fn push(&mut self, mut val: core::Vector::<crate::text::ERStat>) {
			extern "C" { fn cv_VectorOfVectorOfERStat_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfERStat_push(self.as_raw_mut_VectorOfVectorOfERStat(), val.as_raw_mut_VectorOfERStat()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, mut val: core::Vector::<crate::text::ERStat>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfERStat_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfERStat_insert(self.as_raw_mut_VectorOfVectorOfERStat(), index, val.as_raw_mut_VectorOfERStat()) }
			Ok(())
		}
	
		#[inline]
		fn set(&mut self, index: size_t, mut val: core::Vector::<crate::text::ERStat>) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfERStat_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			core::vector_index_check(index, self.len())?;
			unsafe { cv_VectorOfVectorOfERStat_set(self.as_raw_mut_VectorOfVectorOfERStat(), index, val.as_raw_mut_VectorOfERStat()) }
			Ok(())
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, mut val: core::Vector::<crate::text::ERStat>) {
			extern "C" { fn cv_VectorOfVectorOfERStat_set(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfERStat_set(self.as_raw_mut_VectorOfVectorOfERStat(), index, val.as_raw_mut_VectorOfERStat())
		}
	}
	
	unsafe impl Send for core::Vector::<core::Vector::<crate::text::ERStat>> {}
	
}
#[cfg(feature = "contrib")]
pub use text_types::*;

mod video_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBackgroundSubtractorKNN = core::Ptr::<dyn crate::video::BackgroundSubtractorKNN>;
	
	ptr_extern! { dyn crate::video::BackgroundSubtractorKNN,
		cv_PtrOfBackgroundSubtractorKNN_delete, cv_PtrOfBackgroundSubtractorKNN_get_inner_ptr
	}
	
	impl PtrOfBackgroundSubtractorKNN {
		pub fn as_raw_PtrOfBackgroundSubtractorKNN(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfBackgroundSubtractorKNN(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBackgroundSubtractorKNN {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorKNN {
		fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorKNN for PtrOfBackgroundSubtractorKNN {
		fn as_raw_BackgroundSubtractorKNN(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BackgroundSubtractorKNN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackgroundSubtractorMOG2 = core::Ptr::<dyn crate::video::BackgroundSubtractorMOG2>;
	
	ptr_extern! { dyn crate::video::BackgroundSubtractorMOG2,
		cv_PtrOfBackgroundSubtractorMOG2_delete, cv_PtrOfBackgroundSubtractorMOG2_get_inner_ptr
	}
	
	impl PtrOfBackgroundSubtractorMOG2 {
		pub fn as_raw_PtrOfBackgroundSubtractorMOG2(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfBackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBackgroundSubtractorMOG2 {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorMOG2 {
		fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorMOG2 for PtrOfBackgroundSubtractorMOG2 {
		fn as_raw_BackgroundSubtractorMOG2(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDualTVL1OpticalFlow = core::Ptr::<dyn crate::video::DualTVL1OpticalFlow>;
	
	ptr_extern! { dyn crate::video::DualTVL1OpticalFlow,
		cv_PtrOfDualTVL1OpticalFlow_delete, cv_PtrOfDualTVL1OpticalFlow_get_inner_ptr
	}
	
	impl PtrOfDualTVL1OpticalFlow {
		pub fn as_raw_PtrOfDualTVL1OpticalFlow(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfDualTVL1OpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfDualTVL1OpticalFlow {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DenseOpticalFlow for PtrOfDualTVL1OpticalFlow {
		fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DualTVL1OpticalFlow for PtrOfDualTVL1OpticalFlow {
		fn as_raw_DualTVL1OpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_DualTVL1OpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFarnebackOpticalFlow = core::Ptr::<dyn crate::video::FarnebackOpticalFlow>;
	
	ptr_extern! { dyn crate::video::FarnebackOpticalFlow,
		cv_PtrOfFarnebackOpticalFlow_delete, cv_PtrOfFarnebackOpticalFlow_get_inner_ptr
	}
	
	impl PtrOfFarnebackOpticalFlow {
		pub fn as_raw_PtrOfFarnebackOpticalFlow(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfFarnebackOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFarnebackOpticalFlow {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DenseOpticalFlow for PtrOfFarnebackOpticalFlow {
		fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::FarnebackOpticalFlow for PtrOfFarnebackOpticalFlow {
		fn as_raw_FarnebackOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSparsePyrLKOpticalFlow = core::Ptr::<dyn crate::video::SparsePyrLKOpticalFlow>;
	
	ptr_extern! { dyn crate::video::SparsePyrLKOpticalFlow,
		cv_PtrOfSparsePyrLKOpticalFlow_delete, cv_PtrOfSparsePyrLKOpticalFlow_get_inner_ptr
	}
	
	impl PtrOfSparsePyrLKOpticalFlow {
		pub fn as_raw_PtrOfSparsePyrLKOpticalFlow(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfSparsePyrLKOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSparsePyrLKOpticalFlow {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::SparseOpticalFlow for PtrOfSparsePyrLKOpticalFlow {
		fn as_raw_SparseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_SparseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::SparsePyrLKOpticalFlow for PtrOfSparsePyrLKOpticalFlow {
		fn as_raw_SparsePyrLKOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_SparsePyrLKOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
pub use video_types::*;

mod videostab_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfDeblurerBase = core::Ptr::<dyn crate::videostab::DeblurerBase>;
	
	ptr_extern! { dyn crate::videostab::DeblurerBase,
		cv_PtrOfDeblurerBase_delete, cv_PtrOfDeblurerBase_get_inner_ptr
	}
	
	impl PtrOfDeblurerBase {
		pub fn as_raw_PtrOfDeblurerBase(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfDeblurerBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::DeblurerBase for PtrOfDeblurerBase {
		fn as_raw_DeblurerBase(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_DeblurerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfIDenseOptFlowEstimator = core::Ptr::<dyn crate::videostab::IDenseOptFlowEstimator>;
	
	ptr_extern! { dyn crate::videostab::IDenseOptFlowEstimator,
		cv_PtrOfIDenseOptFlowEstimator_delete, cv_PtrOfIDenseOptFlowEstimator_get_inner_ptr
	}
	
	impl PtrOfIDenseOptFlowEstimator {
		pub fn as_raw_PtrOfIDenseOptFlowEstimator(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfIDenseOptFlowEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::IDenseOptFlowEstimator for PtrOfIDenseOptFlowEstimator {
		fn as_raw_IDenseOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_IDenseOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfIFrameSource = core::Ptr::<dyn crate::videostab::IFrameSource>;
	
	ptr_extern! { dyn crate::videostab::IFrameSource,
		cv_PtrOfIFrameSource_delete, cv_PtrOfIFrameSource_get_inner_ptr
	}
	
	impl PtrOfIFrameSource {
		pub fn as_raw_PtrOfIFrameSource(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfIFrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::IFrameSource for PtrOfIFrameSource {
		fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfILog = core::Ptr::<dyn crate::videostab::ILog>;
	
	ptr_extern! { dyn crate::videostab::ILog,
		cv_PtrOfILog_delete, cv_PtrOfILog_get_inner_ptr
	}
	
	impl PtrOfILog {
		pub fn as_raw_PtrOfILog(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfILog(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::ILog for PtrOfILog {
		fn as_raw_ILog(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_ILog(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfIMotionStabilizer = core::Ptr::<dyn crate::videostab::IMotionStabilizer>;
	
	ptr_extern! { dyn crate::videostab::IMotionStabilizer,
		cv_PtrOfIMotionStabilizer_delete, cv_PtrOfIMotionStabilizer_get_inner_ptr
	}
	
	impl PtrOfIMotionStabilizer {
		pub fn as_raw_PtrOfIMotionStabilizer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfIMotionStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::IMotionStabilizer for PtrOfIMotionStabilizer {
		fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfIOutlierRejector = core::Ptr::<dyn crate::videostab::IOutlierRejector>;
	
	ptr_extern! { dyn crate::videostab::IOutlierRejector,
		cv_PtrOfIOutlierRejector_delete, cv_PtrOfIOutlierRejector_get_inner_ptr
	}
	
	impl PtrOfIOutlierRejector {
		pub fn as_raw_PtrOfIOutlierRejector(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfIOutlierRejector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::IOutlierRejector for PtrOfIOutlierRejector {
		fn as_raw_IOutlierRejector(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_IOutlierRejector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfISparseOptFlowEstimator = core::Ptr::<dyn crate::videostab::ISparseOptFlowEstimator>;
	
	ptr_extern! { dyn crate::videostab::ISparseOptFlowEstimator,
		cv_PtrOfISparseOptFlowEstimator_delete, cv_PtrOfISparseOptFlowEstimator_get_inner_ptr
	}
	
	impl PtrOfISparseOptFlowEstimator {
		pub fn as_raw_PtrOfISparseOptFlowEstimator(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfISparseOptFlowEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::ISparseOptFlowEstimator for PtrOfISparseOptFlowEstimator {
		fn as_raw_ISparseOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_ISparseOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfImageMotionEstimatorBase = core::Ptr::<dyn crate::videostab::ImageMotionEstimatorBase>;
	
	ptr_extern! { dyn crate::videostab::ImageMotionEstimatorBase,
		cv_PtrOfImageMotionEstimatorBase_delete, cv_PtrOfImageMotionEstimatorBase_get_inner_ptr
	}
	
	impl PtrOfImageMotionEstimatorBase {
		pub fn as_raw_PtrOfImageMotionEstimatorBase(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfImageMotionEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::ImageMotionEstimatorBase for PtrOfImageMotionEstimatorBase {
		fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfInpainterBase = core::Ptr::<dyn crate::videostab::InpainterBase>;
	
	ptr_extern! { dyn crate::videostab::InpainterBase,
		cv_PtrOfInpainterBase_delete, cv_PtrOfInpainterBase_get_inner_ptr
	}
	
	impl PtrOfInpainterBase {
		pub fn as_raw_PtrOfInpainterBase(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfInpainterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::InpainterBase for PtrOfInpainterBase {
		fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMotionEstimatorBase = core::Ptr::<dyn crate::videostab::MotionEstimatorBase>;
	
	ptr_extern! { dyn crate::videostab::MotionEstimatorBase,
		cv_PtrOfMotionEstimatorBase_delete, cv_PtrOfMotionEstimatorBase_get_inner_ptr
	}
	
	impl PtrOfMotionEstimatorBase {
		pub fn as_raw_PtrOfMotionEstimatorBase(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfMotionEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::MotionEstimatorBase for PtrOfMotionEstimatorBase {
		fn as_raw_MotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_MotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMotionFilterBase = core::Ptr::<dyn crate::videostab::MotionFilterBase>;
	
	ptr_extern! { dyn crate::videostab::MotionFilterBase,
		cv_PtrOfMotionFilterBase_delete, cv_PtrOfMotionFilterBase_get_inner_ptr
	}
	
	impl PtrOfMotionFilterBase {
		pub fn as_raw_PtrOfMotionFilterBase(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfMotionFilterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::IMotionStabilizer for PtrOfMotionFilterBase {
		fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::MotionFilterBase for PtrOfMotionFilterBase {
		fn as_raw_MotionFilterBase(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_MotionFilterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfWobbleSuppressorBase = core::Ptr::<dyn crate::videostab::WobbleSuppressorBase>;
	
	ptr_extern! { dyn crate::videostab::WobbleSuppressorBase,
		cv_PtrOfWobbleSuppressorBase_delete, cv_PtrOfWobbleSuppressorBase_get_inner_ptr
	}
	
	impl PtrOfWobbleSuppressorBase {
		pub fn as_raw_PtrOfWobbleSuppressorBase(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfWobbleSuppressorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::WobbleSuppressorBase for PtrOfWobbleSuppressorBase {
		fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
pub use videostab_types::*;

#[cfg(feature = "contrib")]
mod xfeatures2d_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBoostDesc = core::Ptr::<crate::xfeatures2d::BoostDesc>;
	
	ptr_extern! { crate::xfeatures2d::BoostDesc,
		cv_PtrOfBoostDesc_delete, cv_PtrOfBoostDesc_get_inner_ptr
	}
	
	impl PtrOfBoostDesc {
		pub fn as_raw_PtrOfBoostDesc(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfBoostDesc(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBoostDesc {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::BoostDescTrait for PtrOfBoostDesc {
		fn as_raw_BoostDesc(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BoostDesc(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfBoostDesc {
		fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBriefDescriptorExtractor = core::Ptr::<crate::xfeatures2d::BriefDescriptorExtractor>;
	
	ptr_extern! { crate::xfeatures2d::BriefDescriptorExtractor,
		cv_PtrOfBriefDescriptorExtractor_delete, cv_PtrOfBriefDescriptorExtractor_get_inner_ptr
	}
	
	impl PtrOfBriefDescriptorExtractor {
		pub fn as_raw_PtrOfBriefDescriptorExtractor(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfBriefDescriptorExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfBriefDescriptorExtractor {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::BriefDescriptorExtractorTrait for PtrOfBriefDescriptorExtractor {
		fn as_raw_BriefDescriptorExtractor(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_BriefDescriptorExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfBriefDescriptorExtractor {
		fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDAISY = core::Ptr::<dyn crate::xfeatures2d::DAISY>;
	
	ptr_extern! { dyn crate::xfeatures2d::DAISY,
		cv_PtrOfDAISY_delete, cv_PtrOfDAISY_get_inner_ptr
	}
	
	impl PtrOfDAISY {
		pub fn as_raw_PtrOfDAISY(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfDAISY(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfDAISY {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::DAISY for PtrOfDAISY {
		fn as_raw_DAISY(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_DAISY(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfDAISY {
		fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFREAK = core::Ptr::<crate::xfeatures2d::FREAK>;
	
	ptr_extern! { crate::xfeatures2d::FREAK,
		cv_PtrOfFREAK_delete, cv_PtrOfFREAK_get_inner_ptr
	}
	
	impl PtrOfFREAK {
		pub fn as_raw_PtrOfFREAK(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfFREAK(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfFREAK {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::FREAKTrait for PtrOfFREAK {
		fn as_raw_FREAK(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_FREAK(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfFREAK {
		fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLATCH = core::Ptr::<crate::xfeatures2d::LATCH>;
	
	ptr_extern! { crate::xfeatures2d::LATCH,
		cv_PtrOfLATCH_delete, cv_PtrOfLATCH_get_inner_ptr
	}
	
	impl PtrOfLATCH {
		pub fn as_raw_PtrOfLATCH(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfLATCH(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfLATCH {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfLATCH {
		fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::LATCHTrait for PtrOfLATCH {
		fn as_raw_LATCH(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_LATCH(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLUCID = core::Ptr::<crate::xfeatures2d::LUCID>;
	
	ptr_extern! { crate::xfeatures2d::LUCID,
		cv_PtrOfLUCID_delete, cv_PtrOfLUCID_get_inner_ptr
	}
	
	impl PtrOfLUCID {
		pub fn as_raw_PtrOfLUCID(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfLUCID(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfLUCID {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfLUCID {
		fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::LUCIDTrait for PtrOfLUCID {
		fn as_raw_LUCID(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_LUCID(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMSDDetector = core::Ptr::<crate::xfeatures2d::MSDDetector>;
	
	ptr_extern! { crate::xfeatures2d::MSDDetector,
		cv_PtrOfMSDDetector_delete, cv_PtrOfMSDDetector_get_inner_ptr
	}
	
	impl PtrOfMSDDetector {
		pub fn as_raw_PtrOfMSDDetector(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfMSDDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfMSDDetector {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfMSDDetector {
		fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::MSDDetectorTrait for PtrOfMSDDetector {
		fn as_raw_MSDDetector(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_MSDDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPCTSignatures = core::Ptr::<dyn crate::xfeatures2d::PCTSignatures>;
	
	ptr_extern! { dyn crate::xfeatures2d::PCTSignatures,
		cv_PtrOfPCTSignatures_delete, cv_PtrOfPCTSignatures_get_inner_ptr
	}
	
	impl PtrOfPCTSignatures {
		pub fn as_raw_PtrOfPCTSignatures(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfPCTSignatures(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfPCTSignatures {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::PCTSignatures for PtrOfPCTSignatures {
		fn as_raw_PCTSignatures(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_PCTSignatures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPCTSignaturesSQFD = core::Ptr::<dyn crate::xfeatures2d::PCTSignaturesSQFD>;
	
	ptr_extern! { dyn crate::xfeatures2d::PCTSignaturesSQFD,
		cv_PtrOfPCTSignaturesSQFD_delete, cv_PtrOfPCTSignaturesSQFD_get_inner_ptr
	}
	
	impl PtrOfPCTSignaturesSQFD {
		pub fn as_raw_PtrOfPCTSignaturesSQFD(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfPCTSignaturesSQFD(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfPCTSignaturesSQFD {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::PCTSignaturesSQFD for PtrOfPCTSignaturesSQFD {
		fn as_raw_PCTSignaturesSQFD(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_PCTSignaturesSQFD(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSIFT = core::Ptr::<crate::xfeatures2d::SIFT>;
	
	ptr_extern! { crate::xfeatures2d::SIFT,
		cv_PtrOfSIFT_delete, cv_PtrOfSIFT_get_inner_ptr
	}
	
	impl PtrOfSIFT {
		pub fn as_raw_PtrOfSIFT(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfSIFT(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSIFT {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfSIFT {
		fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::SIFTTrait for PtrOfSIFT {
		fn as_raw_SIFT(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_SIFT(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSURF = core::Ptr::<dyn crate::xfeatures2d::SURF>;
	
	ptr_extern! { dyn crate::xfeatures2d::SURF,
		cv_PtrOfSURF_delete, cv_PtrOfSURF_get_inner_ptr
	}
	
	impl PtrOfSURF {
		pub fn as_raw_PtrOfSURF(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfSURF(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSURF {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfSURF {
		fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::SURF for PtrOfSURF {
		fn as_raw_SURF(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_SURF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStarDetector = core::Ptr::<crate::xfeatures2d::StarDetector>;
	
	ptr_extern! { crate::xfeatures2d::StarDetector,
		cv_PtrOfStarDetector_delete, cv_PtrOfStarDetector_get_inner_ptr
	}
	
	impl PtrOfStarDetector {
		pub fn as_raw_PtrOfStarDetector(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfStarDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfStarDetector {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfStarDetector {
		fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::StarDetectorTrait for PtrOfStarDetector {
		fn as_raw_StarDetector(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_StarDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfVGG = core::Ptr::<dyn crate::xfeatures2d::VGG>;
	
	ptr_extern! { dyn crate::xfeatures2d::VGG,
		cv_PtrOfVGG_delete, cv_PtrOfVGG_get_inner_ptr
	}
	
	impl PtrOfVGG {
		pub fn as_raw_PtrOfVGG(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfVGG(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfVGG {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfVGG {
		fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xfeatures2d::VGG for PtrOfVGG {
		fn as_raw_VGG(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_VGG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use xfeatures2d_types::*;

#[cfg(feature = "contrib")]
mod xobjdetect_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfWBDetector = core::Ptr::<dyn crate::xobjdetect::WBDetector>;
	
	ptr_extern! { dyn crate::xobjdetect::WBDetector,
		cv_PtrOfWBDetector_delete, cv_PtrOfWBDetector_get_inner_ptr
	}
	
	impl PtrOfWBDetector {
		pub fn as_raw_PtrOfWBDetector(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfWBDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xobjdetect::WBDetector for PtrOfWBDetector {
		fn as_raw_WBDetector(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_WBDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use xobjdetect_types::*;

#[cfg(feature = "contrib")]
mod xphoto_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfGrayworldWB = core::Ptr::<dyn crate::xphoto::GrayworldWB>;
	
	ptr_extern! { dyn crate::xphoto::GrayworldWB,
		cv_PtrOfGrayworldWB_delete, cv_PtrOfGrayworldWB_get_inner_ptr
	}
	
	impl PtrOfGrayworldWB {
		pub fn as_raw_PtrOfGrayworldWB(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfGrayworldWB(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfGrayworldWB {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xphoto::GrayworldWB for PtrOfGrayworldWB {
		fn as_raw_GrayworldWB(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_GrayworldWB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xphoto::WhiteBalancer for PtrOfGrayworldWB {
		fn as_raw_WhiteBalancer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLearningBasedWB = core::Ptr::<dyn crate::xphoto::LearningBasedWB>;
	
	ptr_extern! { dyn crate::xphoto::LearningBasedWB,
		cv_PtrOfLearningBasedWB_delete, cv_PtrOfLearningBasedWB_get_inner_ptr
	}
	
	impl PtrOfLearningBasedWB {
		pub fn as_raw_PtrOfLearningBasedWB(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfLearningBasedWB(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfLearningBasedWB {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xphoto::LearningBasedWB for PtrOfLearningBasedWB {
		fn as_raw_LearningBasedWB(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_LearningBasedWB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xphoto::WhiteBalancer for PtrOfLearningBasedWB {
		fn as_raw_WhiteBalancer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSimpleWB = core::Ptr::<dyn crate::xphoto::SimpleWB>;
	
	ptr_extern! { dyn crate::xphoto::SimpleWB,
		cv_PtrOfSimpleWB_delete, cv_PtrOfSimpleWB_get_inner_ptr
	}
	
	impl PtrOfSimpleWB {
		pub fn as_raw_PtrOfSimpleWB(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_PtrOfSimpleWB(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::AlgorithmTrait for PtrOfSimpleWB {
		fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xphoto::SimpleWB for PtrOfSimpleWB {
		fn as_raw_SimpleWB(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_SimpleWB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xphoto::WhiteBalancer for PtrOfSimpleWB {
		fn as_raw_WhiteBalancer(&self) -> *const c_void { self.inner_as_raw() }
		fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(feature = "contrib")]
pub use xphoto_types::*;

pub use crate::manual::types::*;
