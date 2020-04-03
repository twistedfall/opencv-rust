
#[cfg(feature = "contrib")]
mod aruco_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfBoard {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfBoard {
		pub fn as_raw_PtrOfBoard(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfBoard_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfBoard_get_inner_ptr(self.as_raw_PtrOfBoard()) }
		}
	}
	
	impl Drop for PtrOfBoard {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfBoard_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfBoard_delete(self.as_raw_PtrOfBoard()) };
		}
	}
	
	unsafe impl Send for PtrOfBoard {}
	
	impl crate::aruco::BoardTrait for PtrOfBoard {
		fn as_raw_Board(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfCharucoBoard {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfCharucoBoard {
		pub fn as_raw_PtrOfCharucoBoard(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfCharucoBoard_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfCharucoBoard_get_inner_ptr(self.as_raw_PtrOfCharucoBoard()) }
		}
	}
	
	impl Drop for PtrOfCharucoBoard {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfCharucoBoard_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfCharucoBoard_delete(self.as_raw_PtrOfCharucoBoard()) };
		}
	}
	
	unsafe impl Send for PtrOfCharucoBoard {}
	
	impl crate::aruco::BoardTrait for PtrOfCharucoBoard {
		fn as_raw_Board(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::aruco::CharucoBoardTrait for PtrOfCharucoBoard {
		fn as_raw_CharucoBoard(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfDetectorParameters {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfDetectorParameters {
		pub fn as_raw_PtrOfDetectorParameters(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfDetectorParameters_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfDetectorParameters_get_inner_ptr(self.as_raw_PtrOfDetectorParameters()) }
		}
	}
	
	impl Drop for PtrOfDetectorParameters {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfDetectorParameters_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfDetectorParameters_delete(self.as_raw_PtrOfDetectorParameters()) };
		}
	}
	
	unsafe impl Send for PtrOfDetectorParameters {}
	
	impl crate::aruco::DetectorParametersTrait for PtrOfDetectorParameters {
		fn as_raw_DetectorParameters(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfDictionary {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfDictionary {
		pub fn as_raw_PtrOfDictionary(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfDictionary_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfDictionary_get_inner_ptr(self.as_raw_PtrOfDictionary()) }
		}
	}
	
	impl Drop for PtrOfDictionary {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfDictionary_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfDictionary_delete(self.as_raw_PtrOfDictionary()) };
		}
	}
	
	unsafe impl Send for PtrOfDictionary {}
	
	impl crate::aruco::DictionaryTrait for PtrOfDictionary {
		fn as_raw_Dictionary(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfGridBoard {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfGridBoard {
		pub fn as_raw_PtrOfGridBoard(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfGridBoard_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfGridBoard_get_inner_ptr(self.as_raw_PtrOfGridBoard()) }
		}
	}
	
	impl Drop for PtrOfGridBoard {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfGridBoard_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfGridBoard_delete(self.as_raw_PtrOfGridBoard()) };
		}
	}
	
	unsafe impl Send for PtrOfGridBoard {}
	
	impl crate::aruco::BoardTrait for PtrOfGridBoard {
		fn as_raw_Board(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::aruco::GridBoardTrait for PtrOfGridBoard {
		fn as_raw_GridBoard(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
}
#[cfg(feature = "contrib")]
pub use aruco_types::*;

#[cfg(feature = "contrib")]
mod bgsegm_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfBackgroundSubtractorGMG {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfBackgroundSubtractorGMG {
		pub fn as_raw_PtrOfBackgroundSubtractorGMG(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfBackgroundSubtractorGMG_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfBackgroundSubtractorGMG_get_inner_ptr(self.as_raw_PtrOfBackgroundSubtractorGMG()) }
		}
	}
	
	impl Drop for PtrOfBackgroundSubtractorGMG {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfBackgroundSubtractorGMG_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfBackgroundSubtractorGMG_delete(self.as_raw_PtrOfBackgroundSubtractorGMG()) };
		}
	}
	
	unsafe impl Send for PtrOfBackgroundSubtractorGMG {}
	
	impl core::AlgorithmTrait for PtrOfBackgroundSubtractorGMG {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorGMG {
		fn as_raw_BackgroundSubtractor(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::bgsegm::BackgroundSubtractorGMG for PtrOfBackgroundSubtractorGMG {
		fn as_raw_BackgroundSubtractorGMG(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfBackgroundSubtractorMOG {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfBackgroundSubtractorMOG {
		pub fn as_raw_PtrOfBackgroundSubtractorMOG(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfBackgroundSubtractorMOG_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfBackgroundSubtractorMOG_get_inner_ptr(self.as_raw_PtrOfBackgroundSubtractorMOG()) }
		}
	}
	
	impl Drop for PtrOfBackgroundSubtractorMOG {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfBackgroundSubtractorMOG_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfBackgroundSubtractorMOG_delete(self.as_raw_PtrOfBackgroundSubtractorMOG()) };
		}
	}
	
	unsafe impl Send for PtrOfBackgroundSubtractorMOG {}
	
	impl core::AlgorithmTrait for PtrOfBackgroundSubtractorMOG {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorMOG {
		fn as_raw_BackgroundSubtractor(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::bgsegm::BackgroundSubtractorMOG for PtrOfBackgroundSubtractorMOG {
		fn as_raw_BackgroundSubtractorMOG(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
}
#[cfg(feature = "contrib")]
pub use bgsegm_types::*;

#[cfg(feature = "contrib")]
mod bioinspired_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfRetina {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfRetina {
		pub fn as_raw_PtrOfRetina(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfRetina_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfRetina_get_inner_ptr(self.as_raw_PtrOfRetina()) }
		}
	}
	
	impl Drop for PtrOfRetina {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfRetina_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfRetina_delete(self.as_raw_PtrOfRetina()) };
		}
	}
	
	unsafe impl Send for PtrOfRetina {}
	
	impl core::AlgorithmTrait for PtrOfRetina {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::bioinspired::Retina for PtrOfRetina {
		fn as_raw_Retina(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfRetinaFastToneMapping {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfRetinaFastToneMapping {
		pub fn as_raw_PtrOfRetinaFastToneMapping(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfRetinaFastToneMapping_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfRetinaFastToneMapping_get_inner_ptr(self.as_raw_PtrOfRetinaFastToneMapping()) }
		}
	}
	
	impl Drop for PtrOfRetinaFastToneMapping {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfRetinaFastToneMapping_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfRetinaFastToneMapping_delete(self.as_raw_PtrOfRetinaFastToneMapping()) };
		}
	}
	
	unsafe impl Send for PtrOfRetinaFastToneMapping {}
	
	impl core::AlgorithmTrait for PtrOfRetinaFastToneMapping {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::bioinspired::RetinaFastToneMapping for PtrOfRetinaFastToneMapping {
		fn as_raw_RetinaFastToneMapping(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfTransientAreasSegmentationModule {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfTransientAreasSegmentationModule {
		pub fn as_raw_PtrOfTransientAreasSegmentationModule(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfTransientAreasSegmentationModule_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfTransientAreasSegmentationModule_get_inner_ptr(self.as_raw_PtrOfTransientAreasSegmentationModule()) }
		}
	}
	
	impl Drop for PtrOfTransientAreasSegmentationModule {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfTransientAreasSegmentationModule_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfTransientAreasSegmentationModule_delete(self.as_raw_PtrOfTransientAreasSegmentationModule()) };
		}
	}
	
	unsafe impl Send for PtrOfTransientAreasSegmentationModule {}
	
	impl core::AlgorithmTrait for PtrOfTransientAreasSegmentationModule {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::bioinspired::TransientAreasSegmentationModule for PtrOfTransientAreasSegmentationModule {
		fn as_raw_TransientAreasSegmentationModule(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
}
#[cfg(feature = "contrib")]
pub use bioinspired_types::*;

mod calib3d_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfStereoBM {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfStereoBM {
		pub fn as_raw_PtrOfStereoBM(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfStereoBM_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfStereoBM_get_inner_ptr(self.as_raw_PtrOfStereoBM()) }
		}
	}
	
	impl Drop for PtrOfStereoBM {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfStereoBM_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfStereoBM_delete(self.as_raw_PtrOfStereoBM()) };
		}
	}
	
	unsafe impl Send for PtrOfStereoBM {}
	
	impl core::AlgorithmTrait for PtrOfStereoBM {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::calib3d::StereoBM for PtrOfStereoBM {
		fn as_raw_StereoBM(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::calib3d::StereoMatcher for PtrOfStereoBM {
		fn as_raw_StereoMatcher(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfStereoSGBM {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfStereoSGBM {
		pub fn as_raw_PtrOfStereoSGBM(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfStereoSGBM_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfStereoSGBM_get_inner_ptr(self.as_raw_PtrOfStereoSGBM()) }
		}
	}
	
	impl Drop for PtrOfStereoSGBM {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfStereoSGBM_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfStereoSGBM_delete(self.as_raw_PtrOfStereoSGBM()) };
		}
	}
	
	unsafe impl Send for PtrOfStereoSGBM {}
	
	impl core::AlgorithmTrait for PtrOfStereoSGBM {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::calib3d::StereoMatcher for PtrOfStereoSGBM {
		fn as_raw_StereoMatcher(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::calib3d::StereoSGBM for PtrOfStereoSGBM {
		fn as_raw_StereoSGBM(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
}
pub use calib3d_types::*;

mod core_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfConjGradSolver {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfConjGradSolver {
		pub fn as_raw_PtrOfConjGradSolver(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfConjGradSolver_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfConjGradSolver_get_inner_ptr(self.as_raw_PtrOfConjGradSolver()) }
		}
	}
	
	impl Drop for PtrOfConjGradSolver {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfConjGradSolver_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfConjGradSolver_delete(self.as_raw_PtrOfConjGradSolver()) };
		}
	}
	
	unsafe impl Send for PtrOfConjGradSolver {}
	
	impl core::AlgorithmTrait for PtrOfConjGradSolver {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl core::ConjGradSolver for PtrOfConjGradSolver {
		fn as_raw_ConjGradSolver(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl core::MinProblemSolver for PtrOfConjGradSolver {
		fn as_raw_MinProblemSolver(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfDownhillSolver {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfDownhillSolver {
		pub fn as_raw_PtrOfDownhillSolver(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfDownhillSolver_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfDownhillSolver_get_inner_ptr(self.as_raw_PtrOfDownhillSolver()) }
		}
	}
	
	impl Drop for PtrOfDownhillSolver {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfDownhillSolver_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfDownhillSolver_delete(self.as_raw_PtrOfDownhillSolver()) };
		}
	}
	
	unsafe impl Send for PtrOfDownhillSolver {}
	
	impl core::AlgorithmTrait for PtrOfDownhillSolver {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl core::DownhillSolver for PtrOfDownhillSolver {
		fn as_raw_DownhillSolver(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl core::MinProblemSolver for PtrOfDownhillSolver {
		fn as_raw_MinProblemSolver(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfFormatted {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfFormatted {
		pub fn as_raw_PtrOfFormatted(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfFormatted_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfFormatted_get_inner_ptr(self.as_raw_PtrOfFormatted()) }
		}
	}
	
	impl Drop for PtrOfFormatted {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfFormatted_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfFormatted_delete(self.as_raw_PtrOfFormatted()) };
		}
	}
	
	unsafe impl Send for PtrOfFormatted {}
	
	impl core::Formatted for PtrOfFormatted {
		fn as_raw_Formatted(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfFormatter {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfFormatter {
		pub fn as_raw_PtrOfFormatter(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfFormatter_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfFormatter_get_inner_ptr(self.as_raw_PtrOfFormatter()) }
		}
	}
	
	impl Drop for PtrOfFormatter {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfFormatter_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfFormatter_delete(self.as_raw_PtrOfFormatter()) };
		}
	}
	
	unsafe impl Send for PtrOfFormatter {}
	
	impl core::Formatter for PtrOfFormatter {
		fn as_raw_Formatter(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfMinProblemSolver_Function {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfMinProblemSolver_Function {
		pub fn as_raw_PtrOfMinProblemSolver_Function(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfMinProblemSolver_Function_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfMinProblemSolver_Function_get_inner_ptr(self.as_raw_PtrOfMinProblemSolver_Function()) }
		}
	}
	
	impl Drop for PtrOfMinProblemSolver_Function {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfMinProblemSolver_Function_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfMinProblemSolver_Function_delete(self.as_raw_PtrOfMinProblemSolver_Function()) };
		}
	}
	
	unsafe impl Send for PtrOfMinProblemSolver_Function {}
	
	impl core::MinProblemSolver_Function for PtrOfMinProblemSolver_Function {
		fn as_raw_MinProblemSolver_Function(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct VectorOfDMatch {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfDMatch {
		pub fn as_raw_VectorOfDMatch(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
		
		pub fn to_slice(&self) -> &[core::DMatch] {
			extern "C" { fn cv_VectorOfDMatch_data(instance: *mut c_void) -> *const core::DMatch; }
			unsafe {
				let data = cv_VectorOfDMatch_data(self.as_raw_VectorOfDMatch());
				::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
			}
		}
	}
	
	impl Drop for VectorOfDMatch {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfDMatch_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfDMatch_delete(self.as_raw_VectorOfDMatch()) };
		}
	}
	
	impl IntoIterator for VectorOfDMatch {
		type Item = core::DMatch;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfDMatch {
		type Item = core::DMatch;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfDMatch>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfDMatch {
		type Storage = core::DMatch;
		type Arg = core::DMatch;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfDMatch_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfDMatch_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfDMatch_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfDMatch_len(self.as_raw_VectorOfDMatch()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfDMatch_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfDMatch_is_empty(self.as_raw_VectorOfDMatch()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfDMatch_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfDMatch_capacity(self.as_raw_VectorOfDMatch()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfDMatch_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfDMatch_shrink_to_fit(self.as_raw_VectorOfDMatch()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfDMatch_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfDMatch_reserve(self.as_raw_VectorOfDMatch(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfDMatch_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfDMatch_remove(self.as_raw_VectorOfDMatch(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfDMatch_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfDMatch_swap(self.as_raw_VectorOfDMatch(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfDMatch_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfDMatch_clear(self.as_raw_VectorOfDMatch()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfDMatch_push(instance: *mut c_void, val: *const core::DMatch); }
			unsafe { cv_VectorOfDMatch_push(self.as_raw_VectorOfDMatch(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfDMatch_insert(instance: *mut c_void, index: size_t, val: *const core::DMatch); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfDMatch_insert(self.as_raw_VectorOfDMatch(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfDMatch_get(instance: *mut c_void, index: size_t) -> sys::Result<core::DMatch>; }
			unsafe { cv_VectorOfDMatch_get(self.as_raw_VectorOfDMatch(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfDMatch_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<core::DMatch>; }
			cv_VectorOfDMatch_get_unchecked(self.as_raw_VectorOfDMatch(), index)
				.into_result()
				.expect("Infallible function failed: VectorOfDMatch::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfDMatch_set(instance: *mut c_void, index: size_t, val: *const core::DMatch) -> sys::Result_void; }
			unsafe { cv_VectorOfDMatch_set(self.as_raw_VectorOfDMatch(), index, &val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfDMatch_set_unchecked(instance: *mut c_void, index: size_t, val: *const core::DMatch); }
			cv_VectorOfDMatch_set_unchecked(self.as_raw_VectorOfDMatch(), index, &val)
		}
	
		
		#[inline]
		fn to_vec(&self) -> Vec<Self::Storage> {
			self.to_slice().to_vec()
		}
	}
	
	unsafe impl Send for VectorOfDMatch {}
	
	pub struct VectorOfKeyPoint {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfKeyPoint {
		pub fn as_raw_VectorOfKeyPoint(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
		
		pub fn to_slice(&self) -> &[core::KeyPoint] {
			extern "C" { fn cv_VectorOfKeyPoint_data(instance: *mut c_void) -> *const core::KeyPoint; }
			unsafe {
				let data = cv_VectorOfKeyPoint_data(self.as_raw_VectorOfKeyPoint());
				::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
			}
		}
	}
	
	impl Drop for VectorOfKeyPoint {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfKeyPoint_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfKeyPoint_delete(self.as_raw_VectorOfKeyPoint()) };
		}
	}
	
	impl IntoIterator for VectorOfKeyPoint {
		type Item = core::KeyPoint;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfKeyPoint {
		type Item = core::KeyPoint;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfKeyPoint>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfKeyPoint {
		type Storage = core::KeyPoint;
		type Arg = core::KeyPoint;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfKeyPoint_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfKeyPoint_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfKeyPoint_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfKeyPoint_len(self.as_raw_VectorOfKeyPoint()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfKeyPoint_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfKeyPoint_is_empty(self.as_raw_VectorOfKeyPoint()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfKeyPoint_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfKeyPoint_capacity(self.as_raw_VectorOfKeyPoint()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfKeyPoint_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfKeyPoint_shrink_to_fit(self.as_raw_VectorOfKeyPoint()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfKeyPoint_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfKeyPoint_reserve(self.as_raw_VectorOfKeyPoint(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfKeyPoint_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfKeyPoint_remove(self.as_raw_VectorOfKeyPoint(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfKeyPoint_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfKeyPoint_swap(self.as_raw_VectorOfKeyPoint(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfKeyPoint_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfKeyPoint_clear(self.as_raw_VectorOfKeyPoint()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfKeyPoint_push(instance: *mut c_void, val: *const core::KeyPoint); }
			unsafe { cv_VectorOfKeyPoint_push(self.as_raw_VectorOfKeyPoint(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfKeyPoint_insert(instance: *mut c_void, index: size_t, val: *const core::KeyPoint); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfKeyPoint_insert(self.as_raw_VectorOfKeyPoint(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfKeyPoint_get(instance: *mut c_void, index: size_t) -> sys::Result<core::KeyPoint>; }
			unsafe { cv_VectorOfKeyPoint_get(self.as_raw_VectorOfKeyPoint(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfKeyPoint_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<core::KeyPoint>; }
			cv_VectorOfKeyPoint_get_unchecked(self.as_raw_VectorOfKeyPoint(), index)
				.into_result()
				.expect("Infallible function failed: VectorOfKeyPoint::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfKeyPoint_set(instance: *mut c_void, index: size_t, val: *const core::KeyPoint) -> sys::Result_void; }
			unsafe { cv_VectorOfKeyPoint_set(self.as_raw_VectorOfKeyPoint(), index, &val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfKeyPoint_set_unchecked(instance: *mut c_void, index: size_t, val: *const core::KeyPoint); }
			cv_VectorOfKeyPoint_set_unchecked(self.as_raw_VectorOfKeyPoint(), index, &val)
		}
	
		
		#[inline]
		fn to_vec(&self) -> Vec<Self::Storage> {
			self.to_slice().to_vec()
		}
	}
	
	unsafe impl Send for VectorOfKeyPoint {}
	
	pub struct VectorOfMat {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfMat {
		pub fn as_raw_VectorOfMat(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfMat {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfMat_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfMat_delete(self.as_raw_VectorOfMat()) };
		}
	}
	
	impl IntoIterator for VectorOfMat {
		type Item = core::Mat;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfMat {
		type Item = core::Mat;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfMat>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfMat {
		type Storage = core::Mat;
		type Arg = core::Mat;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfMat_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfMat_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfMat_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfMat_len(self.as_raw_VectorOfMat()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfMat_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfMat_is_empty(self.as_raw_VectorOfMat()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfMat_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfMat_capacity(self.as_raw_VectorOfMat()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfMat_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfMat_shrink_to_fit(self.as_raw_VectorOfMat()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfMat_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfMat_reserve(self.as_raw_VectorOfMat(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfMat_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfMat_remove(self.as_raw_VectorOfMat(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfMat_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfMat_swap(self.as_raw_VectorOfMat(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfMat_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfMat_clear(self.as_raw_VectorOfMat()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfMat_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfMat_push(self.as_raw_VectorOfMat(), val.as_raw_Mat()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfMat_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfMat_insert(self.as_raw_VectorOfMat(), index, val.as_raw_Mat()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfMat_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfMat_get(self.as_raw_VectorOfMat(), index) }
				.into_result()
				.map(|ptr| core::Mat { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfMat_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfMat_get_unchecked(self.as_raw_VectorOfMat(), index)
				.into_result()
				.map(|ptr| core::Mat { ptr })
				.expect("Infallible function failed: VectorOfMat::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfMat_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfMat_set(self.as_raw_VectorOfMat(), index, val.as_raw_Mat()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfMat_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfMat_set_unchecked(self.as_raw_VectorOfMat(), index, val.as_raw_Mat())
		}
	
	}
	
	unsafe impl Send for VectorOfMat {}
	
	pub struct VectorOfPlatformInfo {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfPlatformInfo {
		pub fn as_raw_VectorOfPlatformInfo(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfPlatformInfo {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfPlatformInfo_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfPlatformInfo_delete(self.as_raw_VectorOfPlatformInfo()) };
		}
	}
	
	impl IntoIterator for VectorOfPlatformInfo {
		type Item = core::PlatformInfo;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfPlatformInfo {
		type Item = core::PlatformInfo;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfPlatformInfo>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfPlatformInfo {
		type Storage = core::PlatformInfo;
		type Arg = core::PlatformInfo;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfPlatformInfo_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfPlatformInfo_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfPlatformInfo_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfPlatformInfo_len(self.as_raw_VectorOfPlatformInfo()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfPlatformInfo_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfPlatformInfo_is_empty(self.as_raw_VectorOfPlatformInfo()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfPlatformInfo_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfPlatformInfo_capacity(self.as_raw_VectorOfPlatformInfo()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfPlatformInfo_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfPlatformInfo_shrink_to_fit(self.as_raw_VectorOfPlatformInfo()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfPlatformInfo_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfPlatformInfo_reserve(self.as_raw_VectorOfPlatformInfo(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfPlatformInfo_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfPlatformInfo_remove(self.as_raw_VectorOfPlatformInfo(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfPlatformInfo_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfPlatformInfo_swap(self.as_raw_VectorOfPlatformInfo(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfPlatformInfo_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfPlatformInfo_clear(self.as_raw_VectorOfPlatformInfo()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfPlatformInfo_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfPlatformInfo_push(self.as_raw_VectorOfPlatformInfo(), val.as_raw_PlatformInfo()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfPlatformInfo_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfPlatformInfo_insert(self.as_raw_VectorOfPlatformInfo(), index, val.as_raw_PlatformInfo()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfPlatformInfo_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPlatformInfo_get(self.as_raw_VectorOfPlatformInfo(), index) }
				.into_result()
				.map(|ptr| core::PlatformInfo { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfPlatformInfo_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfPlatformInfo_get_unchecked(self.as_raw_VectorOfPlatformInfo(), index)
				.into_result()
				.map(|ptr| core::PlatformInfo { ptr })
				.expect("Infallible function failed: VectorOfPlatformInfo::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfPlatformInfo_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfPlatformInfo_set(self.as_raw_VectorOfPlatformInfo(), index, val.as_raw_PlatformInfo()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfPlatformInfo_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfPlatformInfo_set_unchecked(self.as_raw_VectorOfPlatformInfo(), index, val.as_raw_PlatformInfo())
		}
	
	}
	
	unsafe impl Send for VectorOfPlatformInfo {}
	
	pub struct VectorOfPoint {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfPoint {
		pub fn as_raw_VectorOfPoint(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
		
		pub fn to_slice(&self) -> &[core::Point] {
			extern "C" { fn cv_VectorOfPoint_data(instance: *mut c_void) -> *const core::Point; }
			unsafe {
				let data = cv_VectorOfPoint_data(self.as_raw_VectorOfPoint());
				::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
			}
		}
	}
	
	impl Drop for VectorOfPoint {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfPoint_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfPoint_delete(self.as_raw_VectorOfPoint()) };
		}
	}
	
	impl IntoIterator for VectorOfPoint {
		type Item = core::Point;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfPoint {
		type Item = core::Point;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfPoint>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfPoint {
		type Storage = core::Point;
		type Arg = core::Point;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfPoint_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfPoint_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfPoint_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfPoint_len(self.as_raw_VectorOfPoint()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfPoint_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfPoint_is_empty(self.as_raw_VectorOfPoint()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfPoint_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfPoint_capacity(self.as_raw_VectorOfPoint()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfPoint_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfPoint_shrink_to_fit(self.as_raw_VectorOfPoint()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfPoint_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfPoint_reserve(self.as_raw_VectorOfPoint(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfPoint_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfPoint_remove(self.as_raw_VectorOfPoint(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfPoint_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfPoint_swap(self.as_raw_VectorOfPoint(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfPoint_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfPoint_clear(self.as_raw_VectorOfPoint()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfPoint_push(instance: *mut c_void, val: *const core::Point); }
			unsafe { cv_VectorOfPoint_push(self.as_raw_VectorOfPoint(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint_insert(instance: *mut c_void, index: size_t, val: *const core::Point); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfPoint_insert(self.as_raw_VectorOfPoint(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfPoint_get(instance: *mut c_void, index: size_t) -> sys::Result<core::Point>; }
			unsafe { cv_VectorOfPoint_get(self.as_raw_VectorOfPoint(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfPoint_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<core::Point>; }
			cv_VectorOfPoint_get_unchecked(self.as_raw_VectorOfPoint(), index)
				.into_result()
				.expect("Infallible function failed: VectorOfPoint::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint_set(instance: *mut c_void, index: size_t, val: *const core::Point) -> sys::Result_void; }
			unsafe { cv_VectorOfPoint_set(self.as_raw_VectorOfPoint(), index, &val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfPoint_set_unchecked(instance: *mut c_void, index: size_t, val: *const core::Point); }
			cv_VectorOfPoint_set_unchecked(self.as_raw_VectorOfPoint(), index, &val)
		}
	
		
		#[inline]
		fn to_vec(&self) -> Vec<Self::Storage> {
			self.to_slice().to_vec()
		}
	}
	
	unsafe impl Send for VectorOfPoint {}
	
	impl core::ToInputArray for VectorOfPoint {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint_input_array(self.as_raw_VectorOfPoint()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfPoint_output_array(self.as_raw_VectorOfPoint()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfPoint_input_output_array(self.as_raw_VectorOfPoint()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfPoint {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfPoint2d {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfPoint2d {
		pub fn as_raw_VectorOfPoint2d(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
		
		pub fn to_slice(&self) -> &[core::Point2d] {
			extern "C" { fn cv_VectorOfPoint2d_data(instance: *mut c_void) -> *const core::Point2d; }
			unsafe {
				let data = cv_VectorOfPoint2d_data(self.as_raw_VectorOfPoint2d());
				::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
			}
		}
	}
	
	impl Drop for VectorOfPoint2d {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfPoint2d_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfPoint2d_delete(self.as_raw_VectorOfPoint2d()) };
		}
	}
	
	impl IntoIterator for VectorOfPoint2d {
		type Item = core::Point2d;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfPoint2d {
		type Item = core::Point2d;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfPoint2d>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfPoint2d {
		type Storage = core::Point2d;
		type Arg = core::Point2d;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfPoint2d_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfPoint2d_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfPoint2d_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfPoint2d_len(self.as_raw_VectorOfPoint2d()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfPoint2d_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfPoint2d_is_empty(self.as_raw_VectorOfPoint2d()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfPoint2d_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfPoint2d_capacity(self.as_raw_VectorOfPoint2d()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfPoint2d_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfPoint2d_shrink_to_fit(self.as_raw_VectorOfPoint2d()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfPoint2d_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfPoint2d_reserve(self.as_raw_VectorOfPoint2d(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfPoint2d_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfPoint2d_remove(self.as_raw_VectorOfPoint2d(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfPoint2d_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfPoint2d_swap(self.as_raw_VectorOfPoint2d(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfPoint2d_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfPoint2d_clear(self.as_raw_VectorOfPoint2d()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfPoint2d_push(instance: *mut c_void, val: *const core::Point2d); }
			unsafe { cv_VectorOfPoint2d_push(self.as_raw_VectorOfPoint2d(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint2d_insert(instance: *mut c_void, index: size_t, val: *const core::Point2d); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfPoint2d_insert(self.as_raw_VectorOfPoint2d(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfPoint2d_get(instance: *mut c_void, index: size_t) -> sys::Result<core::Point2d>; }
			unsafe { cv_VectorOfPoint2d_get(self.as_raw_VectorOfPoint2d(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfPoint2d_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<core::Point2d>; }
			cv_VectorOfPoint2d_get_unchecked(self.as_raw_VectorOfPoint2d(), index)
				.into_result()
				.expect("Infallible function failed: VectorOfPoint2d::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint2d_set(instance: *mut c_void, index: size_t, val: *const core::Point2d) -> sys::Result_void; }
			unsafe { cv_VectorOfPoint2d_set(self.as_raw_VectorOfPoint2d(), index, &val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfPoint2d_set_unchecked(instance: *mut c_void, index: size_t, val: *const core::Point2d); }
			cv_VectorOfPoint2d_set_unchecked(self.as_raw_VectorOfPoint2d(), index, &val)
		}
	
		
		#[inline]
		fn to_vec(&self) -> Vec<Self::Storage> {
			self.to_slice().to_vec()
		}
	}
	
	unsafe impl Send for VectorOfPoint2d {}
	
	impl core::ToInputArray for VectorOfPoint2d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint2d_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint2d_input_array(self.as_raw_VectorOfPoint2d()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfPoint2d_output_array(self.as_raw_VectorOfPoint2d()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfPoint2d_input_output_array(self.as_raw_VectorOfPoint2d()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfPoint2d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfPoint2f {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfPoint2f {
		pub fn as_raw_VectorOfPoint2f(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
		
		pub fn to_slice(&self) -> &[core::Point2f] {
			extern "C" { fn cv_VectorOfPoint2f_data(instance: *mut c_void) -> *const core::Point2f; }
			unsafe {
				let data = cv_VectorOfPoint2f_data(self.as_raw_VectorOfPoint2f());
				::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
			}
		}
	}
	
	impl Drop for VectorOfPoint2f {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfPoint2f_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfPoint2f_delete(self.as_raw_VectorOfPoint2f()) };
		}
	}
	
	impl IntoIterator for VectorOfPoint2f {
		type Item = core::Point2f;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfPoint2f {
		type Item = core::Point2f;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfPoint2f>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfPoint2f {
		type Storage = core::Point2f;
		type Arg = core::Point2f;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfPoint2f_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfPoint2f_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfPoint2f_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfPoint2f_len(self.as_raw_VectorOfPoint2f()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfPoint2f_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfPoint2f_is_empty(self.as_raw_VectorOfPoint2f()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfPoint2f_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfPoint2f_capacity(self.as_raw_VectorOfPoint2f()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfPoint2f_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfPoint2f_shrink_to_fit(self.as_raw_VectorOfPoint2f()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfPoint2f_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfPoint2f_reserve(self.as_raw_VectorOfPoint2f(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfPoint2f_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfPoint2f_remove(self.as_raw_VectorOfPoint2f(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfPoint2f_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfPoint2f_swap(self.as_raw_VectorOfPoint2f(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfPoint2f_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfPoint2f_clear(self.as_raw_VectorOfPoint2f()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfPoint2f_push(instance: *mut c_void, val: *const core::Point2f); }
			unsafe { cv_VectorOfPoint2f_push(self.as_raw_VectorOfPoint2f(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint2f_insert(instance: *mut c_void, index: size_t, val: *const core::Point2f); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfPoint2f_insert(self.as_raw_VectorOfPoint2f(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfPoint2f_get(instance: *mut c_void, index: size_t) -> sys::Result<core::Point2f>; }
			unsafe { cv_VectorOfPoint2f_get(self.as_raw_VectorOfPoint2f(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfPoint2f_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<core::Point2f>; }
			cv_VectorOfPoint2f_get_unchecked(self.as_raw_VectorOfPoint2f(), index)
				.into_result()
				.expect("Infallible function failed: VectorOfPoint2f::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint2f_set(instance: *mut c_void, index: size_t, val: *const core::Point2f) -> sys::Result_void; }
			unsafe { cv_VectorOfPoint2f_set(self.as_raw_VectorOfPoint2f(), index, &val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfPoint2f_set_unchecked(instance: *mut c_void, index: size_t, val: *const core::Point2f); }
			cv_VectorOfPoint2f_set_unchecked(self.as_raw_VectorOfPoint2f(), index, &val)
		}
	
		
		#[inline]
		fn to_vec(&self) -> Vec<Self::Storage> {
			self.to_slice().to_vec()
		}
	}
	
	unsafe impl Send for VectorOfPoint2f {}
	
	impl core::ToInputArray for VectorOfPoint2f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint2f_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint2f_input_array(self.as_raw_VectorOfPoint2f()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfPoint2f_output_array(self.as_raw_VectorOfPoint2f()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfPoint2f_input_output_array(self.as_raw_VectorOfPoint2f()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfPoint2f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfPoint3d {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfPoint3d {
		pub fn as_raw_VectorOfPoint3d(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
		
		pub fn to_slice(&self) -> &[core::Point3d] {
			extern "C" { fn cv_VectorOfPoint3d_data(instance: *mut c_void) -> *const core::Point3d; }
			unsafe {
				let data = cv_VectorOfPoint3d_data(self.as_raw_VectorOfPoint3d());
				::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
			}
		}
	}
	
	impl Drop for VectorOfPoint3d {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfPoint3d_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfPoint3d_delete(self.as_raw_VectorOfPoint3d()) };
		}
	}
	
	impl IntoIterator for VectorOfPoint3d {
		type Item = core::Point3d;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfPoint3d {
		type Item = core::Point3d;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfPoint3d>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfPoint3d {
		type Storage = core::Point3d;
		type Arg = core::Point3d;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfPoint3d_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfPoint3d_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfPoint3d_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfPoint3d_len(self.as_raw_VectorOfPoint3d()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfPoint3d_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfPoint3d_is_empty(self.as_raw_VectorOfPoint3d()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfPoint3d_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfPoint3d_capacity(self.as_raw_VectorOfPoint3d()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfPoint3d_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfPoint3d_shrink_to_fit(self.as_raw_VectorOfPoint3d()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfPoint3d_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfPoint3d_reserve(self.as_raw_VectorOfPoint3d(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfPoint3d_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfPoint3d_remove(self.as_raw_VectorOfPoint3d(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfPoint3d_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfPoint3d_swap(self.as_raw_VectorOfPoint3d(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfPoint3d_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfPoint3d_clear(self.as_raw_VectorOfPoint3d()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfPoint3d_push(instance: *mut c_void, val: *const core::Point3d); }
			unsafe { cv_VectorOfPoint3d_push(self.as_raw_VectorOfPoint3d(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint3d_insert(instance: *mut c_void, index: size_t, val: *const core::Point3d); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfPoint3d_insert(self.as_raw_VectorOfPoint3d(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfPoint3d_get(instance: *mut c_void, index: size_t) -> sys::Result<core::Point3d>; }
			unsafe { cv_VectorOfPoint3d_get(self.as_raw_VectorOfPoint3d(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfPoint3d_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<core::Point3d>; }
			cv_VectorOfPoint3d_get_unchecked(self.as_raw_VectorOfPoint3d(), index)
				.into_result()
				.expect("Infallible function failed: VectorOfPoint3d::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint3d_set(instance: *mut c_void, index: size_t, val: *const core::Point3d) -> sys::Result_void; }
			unsafe { cv_VectorOfPoint3d_set(self.as_raw_VectorOfPoint3d(), index, &val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfPoint3d_set_unchecked(instance: *mut c_void, index: size_t, val: *const core::Point3d); }
			cv_VectorOfPoint3d_set_unchecked(self.as_raw_VectorOfPoint3d(), index, &val)
		}
	
		
		#[inline]
		fn to_vec(&self) -> Vec<Self::Storage> {
			self.to_slice().to_vec()
		}
	}
	
	unsafe impl Send for VectorOfPoint3d {}
	
	impl core::ToInputArray for VectorOfPoint3d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint3d_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint3d_input_array(self.as_raw_VectorOfPoint3d()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfPoint3d_output_array(self.as_raw_VectorOfPoint3d()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfPoint3d_input_output_array(self.as_raw_VectorOfPoint3d()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfPoint3d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfPoint3f {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfPoint3f {
		pub fn as_raw_VectorOfPoint3f(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
		
		pub fn to_slice(&self) -> &[core::Point3f] {
			extern "C" { fn cv_VectorOfPoint3f_data(instance: *mut c_void) -> *const core::Point3f; }
			unsafe {
				let data = cv_VectorOfPoint3f_data(self.as_raw_VectorOfPoint3f());
				::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
			}
		}
	}
	
	impl Drop for VectorOfPoint3f {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfPoint3f_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfPoint3f_delete(self.as_raw_VectorOfPoint3f()) };
		}
	}
	
	impl IntoIterator for VectorOfPoint3f {
		type Item = core::Point3f;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfPoint3f {
		type Item = core::Point3f;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfPoint3f>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfPoint3f {
		type Storage = core::Point3f;
		type Arg = core::Point3f;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfPoint3f_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfPoint3f_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfPoint3f_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfPoint3f_len(self.as_raw_VectorOfPoint3f()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfPoint3f_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfPoint3f_is_empty(self.as_raw_VectorOfPoint3f()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfPoint3f_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfPoint3f_capacity(self.as_raw_VectorOfPoint3f()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfPoint3f_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfPoint3f_shrink_to_fit(self.as_raw_VectorOfPoint3f()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfPoint3f_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfPoint3f_reserve(self.as_raw_VectorOfPoint3f(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfPoint3f_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfPoint3f_remove(self.as_raw_VectorOfPoint3f(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfPoint3f_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfPoint3f_swap(self.as_raw_VectorOfPoint3f(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfPoint3f_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfPoint3f_clear(self.as_raw_VectorOfPoint3f()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfPoint3f_push(instance: *mut c_void, val: *const core::Point3f); }
			unsafe { cv_VectorOfPoint3f_push(self.as_raw_VectorOfPoint3f(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint3f_insert(instance: *mut c_void, index: size_t, val: *const core::Point3f); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfPoint3f_insert(self.as_raw_VectorOfPoint3f(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfPoint3f_get(instance: *mut c_void, index: size_t) -> sys::Result<core::Point3f>; }
			unsafe { cv_VectorOfPoint3f_get(self.as_raw_VectorOfPoint3f(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfPoint3f_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<core::Point3f>; }
			cv_VectorOfPoint3f_get_unchecked(self.as_raw_VectorOfPoint3f(), index)
				.into_result()
				.expect("Infallible function failed: VectorOfPoint3f::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint3f_set(instance: *mut c_void, index: size_t, val: *const core::Point3f) -> sys::Result_void; }
			unsafe { cv_VectorOfPoint3f_set(self.as_raw_VectorOfPoint3f(), index, &val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfPoint3f_set_unchecked(instance: *mut c_void, index: size_t, val: *const core::Point3f); }
			cv_VectorOfPoint3f_set_unchecked(self.as_raw_VectorOfPoint3f(), index, &val)
		}
	
		
		#[inline]
		fn to_vec(&self) -> Vec<Self::Storage> {
			self.to_slice().to_vec()
		}
	}
	
	unsafe impl Send for VectorOfPoint3f {}
	
	impl core::ToInputArray for VectorOfPoint3f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint3f_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint3f_input_array(self.as_raw_VectorOfPoint3f()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfPoint3f_output_array(self.as_raw_VectorOfPoint3f()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfPoint3f_input_output_array(self.as_raw_VectorOfPoint3f()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfPoint3f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfPoint3i {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfPoint3i {
		pub fn as_raw_VectorOfPoint3i(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
		
		pub fn to_slice(&self) -> &[core::Point3i] {
			extern "C" { fn cv_VectorOfPoint3i_data(instance: *mut c_void) -> *const core::Point3i; }
			unsafe {
				let data = cv_VectorOfPoint3i_data(self.as_raw_VectorOfPoint3i());
				::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
			}
		}
	}
	
	impl Drop for VectorOfPoint3i {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfPoint3i_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfPoint3i_delete(self.as_raw_VectorOfPoint3i()) };
		}
	}
	
	impl IntoIterator for VectorOfPoint3i {
		type Item = core::Point3i;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfPoint3i {
		type Item = core::Point3i;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfPoint3i>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfPoint3i {
		type Storage = core::Point3i;
		type Arg = core::Point3i;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfPoint3i_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfPoint3i_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfPoint3i_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfPoint3i_len(self.as_raw_VectorOfPoint3i()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfPoint3i_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfPoint3i_is_empty(self.as_raw_VectorOfPoint3i()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfPoint3i_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfPoint3i_capacity(self.as_raw_VectorOfPoint3i()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfPoint3i_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfPoint3i_shrink_to_fit(self.as_raw_VectorOfPoint3i()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfPoint3i_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfPoint3i_reserve(self.as_raw_VectorOfPoint3i(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfPoint3i_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfPoint3i_remove(self.as_raw_VectorOfPoint3i(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfPoint3i_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfPoint3i_swap(self.as_raw_VectorOfPoint3i(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfPoint3i_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfPoint3i_clear(self.as_raw_VectorOfPoint3i()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfPoint3i_push(instance: *mut c_void, val: *const core::Point3i); }
			unsafe { cv_VectorOfPoint3i_push(self.as_raw_VectorOfPoint3i(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint3i_insert(instance: *mut c_void, index: size_t, val: *const core::Point3i); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfPoint3i_insert(self.as_raw_VectorOfPoint3i(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfPoint3i_get(instance: *mut c_void, index: size_t) -> sys::Result<core::Point3i>; }
			unsafe { cv_VectorOfPoint3i_get(self.as_raw_VectorOfPoint3i(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfPoint3i_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<core::Point3i>; }
			cv_VectorOfPoint3i_get_unchecked(self.as_raw_VectorOfPoint3i(), index)
				.into_result()
				.expect("Infallible function failed: VectorOfPoint3i::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfPoint3i_set(instance: *mut c_void, index: size_t, val: *const core::Point3i) -> sys::Result_void; }
			unsafe { cv_VectorOfPoint3i_set(self.as_raw_VectorOfPoint3i(), index, &val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfPoint3i_set_unchecked(instance: *mut c_void, index: size_t, val: *const core::Point3i); }
			cv_VectorOfPoint3i_set_unchecked(self.as_raw_VectorOfPoint3i(), index, &val)
		}
	
		
		#[inline]
		fn to_vec(&self) -> Vec<Self::Storage> {
			self.to_slice().to_vec()
		}
	}
	
	unsafe impl Send for VectorOfPoint3i {}
	
	impl core::ToInputArray for VectorOfPoint3i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint3i_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPoint3i_input_array(self.as_raw_VectorOfPoint3i()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfPoint3i_output_array(self.as_raw_VectorOfPoint3i()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfPoint3i_input_output_array(self.as_raw_VectorOfPoint3i()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfPoint3i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfRange {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfRange {
		pub fn as_raw_VectorOfRange(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfRange {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfRange_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfRange_delete(self.as_raw_VectorOfRange()) };
		}
	}
	
	impl IntoIterator for VectorOfRange {
		type Item = core::Range;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfRange {
		type Item = core::Range;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfRange>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfRange {
		type Storage = core::Range;
		type Arg = core::Range;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfRange_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfRange_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfRange_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfRange_len(self.as_raw_VectorOfRange()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfRange_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfRange_is_empty(self.as_raw_VectorOfRange()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfRange_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfRange_capacity(self.as_raw_VectorOfRange()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfRange_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfRange_shrink_to_fit(self.as_raw_VectorOfRange()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfRange_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfRange_reserve(self.as_raw_VectorOfRange(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfRange_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfRange_remove(self.as_raw_VectorOfRange(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfRange_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfRange_swap(self.as_raw_VectorOfRange(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfRange_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfRange_clear(self.as_raw_VectorOfRange()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfRange_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfRange_push(self.as_raw_VectorOfRange(), val.as_raw_Range()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfRange_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfRange_insert(self.as_raw_VectorOfRange(), index, val.as_raw_Range()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfRange_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfRange_get(self.as_raw_VectorOfRange(), index) }
				.into_result()
				.map(|ptr| core::Range { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfRange_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfRange_get_unchecked(self.as_raw_VectorOfRange(), index)
				.into_result()
				.map(|ptr| core::Range { ptr })
				.expect("Infallible function failed: VectorOfRange::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfRange_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfRange_set(self.as_raw_VectorOfRange(), index, val.as_raw_Range()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfRange_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfRange_set_unchecked(self.as_raw_VectorOfRange(), index, val.as_raw_Range())
		}
	
	}
	
	unsafe impl Send for VectorOfRange {}
	
	pub struct VectorOfRect {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfRect {
		pub fn as_raw_VectorOfRect(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
		
		pub fn to_slice(&self) -> &[core::Rect] {
			extern "C" { fn cv_VectorOfRect_data(instance: *mut c_void) -> *const core::Rect; }
			unsafe {
				let data = cv_VectorOfRect_data(self.as_raw_VectorOfRect());
				::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
			}
		}
	}
	
	impl Drop for VectorOfRect {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfRect_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfRect_delete(self.as_raw_VectorOfRect()) };
		}
	}
	
	impl IntoIterator for VectorOfRect {
		type Item = core::Rect;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfRect {
		type Item = core::Rect;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfRect>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfRect {
		type Storage = core::Rect;
		type Arg = core::Rect;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfRect_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfRect_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfRect_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfRect_len(self.as_raw_VectorOfRect()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfRect_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfRect_is_empty(self.as_raw_VectorOfRect()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfRect_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfRect_capacity(self.as_raw_VectorOfRect()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfRect_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfRect_shrink_to_fit(self.as_raw_VectorOfRect()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfRect_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfRect_reserve(self.as_raw_VectorOfRect(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfRect_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfRect_remove(self.as_raw_VectorOfRect(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfRect_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfRect_swap(self.as_raw_VectorOfRect(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfRect_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfRect_clear(self.as_raw_VectorOfRect()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfRect_push(instance: *mut c_void, val: *const core::Rect); }
			unsafe { cv_VectorOfRect_push(self.as_raw_VectorOfRect(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfRect_insert(instance: *mut c_void, index: size_t, val: *const core::Rect); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfRect_insert(self.as_raw_VectorOfRect(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfRect_get(instance: *mut c_void, index: size_t) -> sys::Result<core::Rect>; }
			unsafe { cv_VectorOfRect_get(self.as_raw_VectorOfRect(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfRect_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<core::Rect>; }
			cv_VectorOfRect_get_unchecked(self.as_raw_VectorOfRect(), index)
				.into_result()
				.expect("Infallible function failed: VectorOfRect::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfRect_set(instance: *mut c_void, index: size_t, val: *const core::Rect) -> sys::Result_void; }
			unsafe { cv_VectorOfRect_set(self.as_raw_VectorOfRect(), index, &val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfRect_set_unchecked(instance: *mut c_void, index: size_t, val: *const core::Rect); }
			cv_VectorOfRect_set_unchecked(self.as_raw_VectorOfRect(), index, &val)
		}
	
		
		#[inline]
		fn to_vec(&self) -> Vec<Self::Storage> {
			self.to_slice().to_vec()
		}
	}
	
	unsafe impl Send for VectorOfRect {}
	
	impl core::ToInputArray for VectorOfRect {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfRect_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfRect_input_array(self.as_raw_VectorOfRect()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfRect_output_array(self.as_raw_VectorOfRect()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfRect_input_output_array(self.as_raw_VectorOfRect()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfRect {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfSize {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfSize {
		pub fn as_raw_VectorOfSize(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
		
		pub fn to_slice(&self) -> &[core::Size] {
			extern "C" { fn cv_VectorOfSize_data(instance: *mut c_void) -> *const core::Size; }
			unsafe {
				let data = cv_VectorOfSize_data(self.as_raw_VectorOfSize());
				::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
			}
		}
	}
	
	impl Drop for VectorOfSize {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfSize_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfSize_delete(self.as_raw_VectorOfSize()) };
		}
	}
	
	impl IntoIterator for VectorOfSize {
		type Item = core::Size;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfSize {
		type Item = core::Size;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfSize>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfSize {
		type Storage = core::Size;
		type Arg = core::Size;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfSize_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfSize_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfSize_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfSize_len(self.as_raw_VectorOfSize()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfSize_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfSize_is_empty(self.as_raw_VectorOfSize()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfSize_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfSize_capacity(self.as_raw_VectorOfSize()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfSize_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfSize_shrink_to_fit(self.as_raw_VectorOfSize()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfSize_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfSize_reserve(self.as_raw_VectorOfSize(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfSize_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfSize_remove(self.as_raw_VectorOfSize(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfSize_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfSize_swap(self.as_raw_VectorOfSize(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfSize_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfSize_clear(self.as_raw_VectorOfSize()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfSize_push(instance: *mut c_void, val: *const core::Size); }
			unsafe { cv_VectorOfSize_push(self.as_raw_VectorOfSize(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfSize_insert(instance: *mut c_void, index: size_t, val: *const core::Size); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfSize_insert(self.as_raw_VectorOfSize(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfSize_get(instance: *mut c_void, index: size_t) -> sys::Result<core::Size>; }
			unsafe { cv_VectorOfSize_get(self.as_raw_VectorOfSize(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfSize_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<core::Size>; }
			cv_VectorOfSize_get_unchecked(self.as_raw_VectorOfSize(), index)
				.into_result()
				.expect("Infallible function failed: VectorOfSize::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfSize_set(instance: *mut c_void, index: size_t, val: *const core::Size) -> sys::Result_void; }
			unsafe { cv_VectorOfSize_set(self.as_raw_VectorOfSize(), index, &val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfSize_set_unchecked(instance: *mut c_void, index: size_t, val: *const core::Size); }
			cv_VectorOfSize_set_unchecked(self.as_raw_VectorOfSize(), index, &val)
		}
	
		
		#[inline]
		fn to_vec(&self) -> Vec<Self::Storage> {
			self.to_slice().to_vec()
		}
	}
	
	unsafe impl Send for VectorOfSize {}
	
	impl core::ToInputArray for VectorOfSize {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfSize_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfSize_input_array(self.as_raw_VectorOfSize()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfSize_output_array(self.as_raw_VectorOfSize()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfSize_input_output_array(self.as_raw_VectorOfSize()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfSize {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfString {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfString {
		pub fn as_raw_VectorOfString(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfString {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfString_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfString_delete(self.as_raw_VectorOfString()) };
		}
	}
	
	impl IntoIterator for VectorOfString {
		type Item = String;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfString {
		type Item = String;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfString>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfString {
		type Storage = String;
		type Arg = &'i str;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfString_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfString_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfString_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfString_len(self.as_raw_VectorOfString()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfString_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfString_is_empty(self.as_raw_VectorOfString()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfString_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfString_capacity(self.as_raw_VectorOfString()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfString_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfString_shrink_to_fit(self.as_raw_VectorOfString()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfString_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfString_reserve(self.as_raw_VectorOfString(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfString_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfString_remove(self.as_raw_VectorOfString(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfString_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfString_swap(self.as_raw_VectorOfString(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfString_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfString_clear(self.as_raw_VectorOfString()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfString_push(instance: *mut c_void, val: *mut c_char); }
			string_arg_infallible!(val);
			unsafe { cv_VectorOfString_push(self.as_raw_VectorOfString(), val.as_ptr() as _) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfString_insert(instance: *mut c_void, index: size_t, val: *mut c_char); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			string_arg!(val);
			unsafe { cv_VectorOfString_insert(self.as_raw_VectorOfString(), index, val.as_ptr() as _) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfString_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfString_get(self.as_raw_VectorOfString(), index) }
				.into_result()
				.map(|s| unsafe { crate::templ::receive_string(s as *mut String) })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfString_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfString_get_unchecked(self.as_raw_VectorOfString(), index)
				.into_result()
				.map(|s| { crate::templ::receive_string(s as *mut String) })
				.expect("Infallible function failed: VectorOfString::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfString_set(instance: *mut c_void, index: size_t, val: *mut c_char) -> sys::Result_void; }
			string_arg!(val);
			unsafe { cv_VectorOfString_set(self.as_raw_VectorOfString(), index, val.as_ptr() as _) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfString_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_char); }
			string_arg_infallible!(val);
			cv_VectorOfString_set_unchecked(self.as_raw_VectorOfString(), index, val.as_ptr() as _)
		}
	
	}
	
	unsafe impl Send for VectorOfString {}
	
	pub struct VectorOfUMat {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfUMat {
		pub fn as_raw_VectorOfUMat(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfUMat {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfUMat_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfUMat_delete(self.as_raw_VectorOfUMat()) };
		}
	}
	
	impl IntoIterator for VectorOfUMat {
		type Item = core::UMat;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfUMat {
		type Item = core::UMat;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfUMat>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfUMat {
		type Storage = core::UMat;
		type Arg = core::UMat;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfUMat_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfUMat_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfUMat_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfUMat_len(self.as_raw_VectorOfUMat()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfUMat_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfUMat_is_empty(self.as_raw_VectorOfUMat()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfUMat_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfUMat_capacity(self.as_raw_VectorOfUMat()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfUMat_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfUMat_shrink_to_fit(self.as_raw_VectorOfUMat()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfUMat_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfUMat_reserve(self.as_raw_VectorOfUMat(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfUMat_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfUMat_remove(self.as_raw_VectorOfUMat(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfUMat_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfUMat_swap(self.as_raw_VectorOfUMat(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfUMat_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfUMat_clear(self.as_raw_VectorOfUMat()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfUMat_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfUMat_push(self.as_raw_VectorOfUMat(), val.as_raw_UMat()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfUMat_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfUMat_insert(self.as_raw_VectorOfUMat(), index, val.as_raw_UMat()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfUMat_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfUMat_get(self.as_raw_VectorOfUMat(), index) }
				.into_result()
				.map(|ptr| core::UMat { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfUMat_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfUMat_get_unchecked(self.as_raw_VectorOfUMat(), index)
				.into_result()
				.map(|ptr| core::UMat { ptr })
				.expect("Infallible function failed: VectorOfUMat::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfUMat_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfUMat_set(self.as_raw_VectorOfUMat(), index, val.as_raw_UMat()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfUMat_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfUMat_set_unchecked(self.as_raw_VectorOfUMat(), index, val.as_raw_UMat())
		}
	
	}
	
	unsafe impl Send for VectorOfUMat {}
	
	pub struct VectorOfVec2i {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfVec2i {
		pub fn as_raw_VectorOfVec2i(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
		
		pub fn to_slice(&self) -> &[core::Vec2i] {
			extern "C" { fn cv_VectorOfVec2i_data(instance: *mut c_void) -> *const core::Vec2i; }
			unsafe {
				let data = cv_VectorOfVec2i_data(self.as_raw_VectorOfVec2i());
				::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
			}
		}
	}
	
	impl Drop for VectorOfVec2i {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfVec2i_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfVec2i_delete(self.as_raw_VectorOfVec2i()) };
		}
	}
	
	impl IntoIterator for VectorOfVec2i {
		type Item = core::Vec2i;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfVec2i {
		type Item = core::Vec2i;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVec2i>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfVec2i {
		type Storage = core::Vec2i;
		type Arg = core::Vec2i;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfVec2i_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfVec2i_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfVec2i_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVec2i_len(self.as_raw_VectorOfVec2i()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfVec2i_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfVec2i_is_empty(self.as_raw_VectorOfVec2i()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfVec2i_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVec2i_capacity(self.as_raw_VectorOfVec2i()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfVec2i_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfVec2i_shrink_to_fit(self.as_raw_VectorOfVec2i()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfVec2i_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfVec2i_reserve(self.as_raw_VectorOfVec2i(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfVec2i_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfVec2i_remove(self.as_raw_VectorOfVec2i(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfVec2i_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfVec2i_swap(self.as_raw_VectorOfVec2i(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfVec2i_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfVec2i_clear(self.as_raw_VectorOfVec2i()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVec2i_push(instance: *mut c_void, val: *const core::Vec2i); }
			unsafe { cv_VectorOfVec2i_push(self.as_raw_VectorOfVec2i(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVec2i_insert(instance: *mut c_void, index: size_t, val: *const core::Vec2i); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVec2i_insert(self.as_raw_VectorOfVec2i(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfVec2i_get(instance: *mut c_void, index: size_t) -> sys::Result<core::Vec2i>; }
			unsafe { cv_VectorOfVec2i_get(self.as_raw_VectorOfVec2i(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfVec2i_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<core::Vec2i>; }
			cv_VectorOfVec2i_get_unchecked(self.as_raw_VectorOfVec2i(), index)
				.into_result()
				.expect("Infallible function failed: VectorOfVec2i::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVec2i_set(instance: *mut c_void, index: size_t, val: *const core::Vec2i) -> sys::Result_void; }
			unsafe { cv_VectorOfVec2i_set(self.as_raw_VectorOfVec2i(), index, &val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVec2i_set_unchecked(instance: *mut c_void, index: size_t, val: *const core::Vec2i); }
			cv_VectorOfVec2i_set_unchecked(self.as_raw_VectorOfVec2i(), index, &val)
		}
	
		
		#[inline]
		fn to_vec(&self) -> Vec<Self::Storage> {
			self.to_slice().to_vec()
		}
	}
	
	unsafe impl Send for VectorOfVec2i {}
	
	impl core::ToInputArray for VectorOfVec2i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec2i_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec2i_input_array(self.as_raw_VectorOfVec2i()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfVec2i_output_array(self.as_raw_VectorOfVec2i()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfVec2i_input_output_array(self.as_raw_VectorOfVec2i()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVec2i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfVec3d {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfVec3d {
		pub fn as_raw_VectorOfVec3d(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
		
		pub fn to_slice(&self) -> &[core::Vec3d] {
			extern "C" { fn cv_VectorOfVec3d_data(instance: *mut c_void) -> *const core::Vec3d; }
			unsafe {
				let data = cv_VectorOfVec3d_data(self.as_raw_VectorOfVec3d());
				::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
			}
		}
	}
	
	impl Drop for VectorOfVec3d {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfVec3d_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfVec3d_delete(self.as_raw_VectorOfVec3d()) };
		}
	}
	
	impl IntoIterator for VectorOfVec3d {
		type Item = core::Vec3d;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfVec3d {
		type Item = core::Vec3d;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVec3d>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfVec3d {
		type Storage = core::Vec3d;
		type Arg = core::Vec3d;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfVec3d_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfVec3d_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfVec3d_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVec3d_len(self.as_raw_VectorOfVec3d()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfVec3d_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfVec3d_is_empty(self.as_raw_VectorOfVec3d()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfVec3d_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVec3d_capacity(self.as_raw_VectorOfVec3d()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfVec3d_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfVec3d_shrink_to_fit(self.as_raw_VectorOfVec3d()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfVec3d_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfVec3d_reserve(self.as_raw_VectorOfVec3d(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfVec3d_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfVec3d_remove(self.as_raw_VectorOfVec3d(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfVec3d_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfVec3d_swap(self.as_raw_VectorOfVec3d(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfVec3d_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfVec3d_clear(self.as_raw_VectorOfVec3d()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVec3d_push(instance: *mut c_void, val: *const core::Vec3d); }
			unsafe { cv_VectorOfVec3d_push(self.as_raw_VectorOfVec3d(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVec3d_insert(instance: *mut c_void, index: size_t, val: *const core::Vec3d); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVec3d_insert(self.as_raw_VectorOfVec3d(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfVec3d_get(instance: *mut c_void, index: size_t) -> sys::Result<core::Vec3d>; }
			unsafe { cv_VectorOfVec3d_get(self.as_raw_VectorOfVec3d(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfVec3d_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<core::Vec3d>; }
			cv_VectorOfVec3d_get_unchecked(self.as_raw_VectorOfVec3d(), index)
				.into_result()
				.expect("Infallible function failed: VectorOfVec3d::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVec3d_set(instance: *mut c_void, index: size_t, val: *const core::Vec3d) -> sys::Result_void; }
			unsafe { cv_VectorOfVec3d_set(self.as_raw_VectorOfVec3d(), index, &val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVec3d_set_unchecked(instance: *mut c_void, index: size_t, val: *const core::Vec3d); }
			cv_VectorOfVec3d_set_unchecked(self.as_raw_VectorOfVec3d(), index, &val)
		}
	
		
		#[inline]
		fn to_vec(&self) -> Vec<Self::Storage> {
			self.to_slice().to_vec()
		}
	}
	
	unsafe impl Send for VectorOfVec3d {}
	
	impl core::ToInputArray for VectorOfVec3d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec3d_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec3d_input_array(self.as_raw_VectorOfVec3d()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfVec3d_output_array(self.as_raw_VectorOfVec3d()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfVec3d_input_output_array(self.as_raw_VectorOfVec3d()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVec3d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfVec3f {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfVec3f {
		pub fn as_raw_VectorOfVec3f(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
		
		pub fn to_slice(&self) -> &[core::Vec3f] {
			extern "C" { fn cv_VectorOfVec3f_data(instance: *mut c_void) -> *const core::Vec3f; }
			unsafe {
				let data = cv_VectorOfVec3f_data(self.as_raw_VectorOfVec3f());
				::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
			}
		}
	}
	
	impl Drop for VectorOfVec3f {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfVec3f_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfVec3f_delete(self.as_raw_VectorOfVec3f()) };
		}
	}
	
	impl IntoIterator for VectorOfVec3f {
		type Item = core::Vec3f;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfVec3f {
		type Item = core::Vec3f;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVec3f>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfVec3f {
		type Storage = core::Vec3f;
		type Arg = core::Vec3f;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfVec3f_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfVec3f_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfVec3f_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVec3f_len(self.as_raw_VectorOfVec3f()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfVec3f_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfVec3f_is_empty(self.as_raw_VectorOfVec3f()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfVec3f_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVec3f_capacity(self.as_raw_VectorOfVec3f()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfVec3f_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfVec3f_shrink_to_fit(self.as_raw_VectorOfVec3f()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfVec3f_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfVec3f_reserve(self.as_raw_VectorOfVec3f(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfVec3f_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfVec3f_remove(self.as_raw_VectorOfVec3f(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfVec3f_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfVec3f_swap(self.as_raw_VectorOfVec3f(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfVec3f_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfVec3f_clear(self.as_raw_VectorOfVec3f()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVec3f_push(instance: *mut c_void, val: *const core::Vec3f); }
			unsafe { cv_VectorOfVec3f_push(self.as_raw_VectorOfVec3f(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVec3f_insert(instance: *mut c_void, index: size_t, val: *const core::Vec3f); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVec3f_insert(self.as_raw_VectorOfVec3f(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfVec3f_get(instance: *mut c_void, index: size_t) -> sys::Result<core::Vec3f>; }
			unsafe { cv_VectorOfVec3f_get(self.as_raw_VectorOfVec3f(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfVec3f_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<core::Vec3f>; }
			cv_VectorOfVec3f_get_unchecked(self.as_raw_VectorOfVec3f(), index)
				.into_result()
				.expect("Infallible function failed: VectorOfVec3f::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVec3f_set(instance: *mut c_void, index: size_t, val: *const core::Vec3f) -> sys::Result_void; }
			unsafe { cv_VectorOfVec3f_set(self.as_raw_VectorOfVec3f(), index, &val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVec3f_set_unchecked(instance: *mut c_void, index: size_t, val: *const core::Vec3f); }
			cv_VectorOfVec3f_set_unchecked(self.as_raw_VectorOfVec3f(), index, &val)
		}
	
		
		#[inline]
		fn to_vec(&self) -> Vec<Self::Storage> {
			self.to_slice().to_vec()
		}
	}
	
	unsafe impl Send for VectorOfVec3f {}
	
	impl core::ToInputArray for VectorOfVec3f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec3f_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec3f_input_array(self.as_raw_VectorOfVec3f()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfVec3f_output_array(self.as_raw_VectorOfVec3f()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfVec3f_input_output_array(self.as_raw_VectorOfVec3f()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVec3f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfVec4f {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfVec4f {
		pub fn as_raw_VectorOfVec4f(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
		
		pub fn to_slice(&self) -> &[core::Vec4f] {
			extern "C" { fn cv_VectorOfVec4f_data(instance: *mut c_void) -> *const core::Vec4f; }
			unsafe {
				let data = cv_VectorOfVec4f_data(self.as_raw_VectorOfVec4f());
				::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
			}
		}
	}
	
	impl Drop for VectorOfVec4f {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfVec4f_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfVec4f_delete(self.as_raw_VectorOfVec4f()) };
		}
	}
	
	impl IntoIterator for VectorOfVec4f {
		type Item = core::Vec4f;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfVec4f {
		type Item = core::Vec4f;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVec4f>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfVec4f {
		type Storage = core::Vec4f;
		type Arg = core::Vec4f;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfVec4f_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfVec4f_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfVec4f_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVec4f_len(self.as_raw_VectorOfVec4f()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfVec4f_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfVec4f_is_empty(self.as_raw_VectorOfVec4f()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfVec4f_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVec4f_capacity(self.as_raw_VectorOfVec4f()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfVec4f_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfVec4f_shrink_to_fit(self.as_raw_VectorOfVec4f()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfVec4f_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfVec4f_reserve(self.as_raw_VectorOfVec4f(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfVec4f_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfVec4f_remove(self.as_raw_VectorOfVec4f(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfVec4f_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfVec4f_swap(self.as_raw_VectorOfVec4f(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfVec4f_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfVec4f_clear(self.as_raw_VectorOfVec4f()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVec4f_push(instance: *mut c_void, val: *const core::Vec4f); }
			unsafe { cv_VectorOfVec4f_push(self.as_raw_VectorOfVec4f(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVec4f_insert(instance: *mut c_void, index: size_t, val: *const core::Vec4f); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVec4f_insert(self.as_raw_VectorOfVec4f(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfVec4f_get(instance: *mut c_void, index: size_t) -> sys::Result<core::Vec4f>; }
			unsafe { cv_VectorOfVec4f_get(self.as_raw_VectorOfVec4f(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfVec4f_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<core::Vec4f>; }
			cv_VectorOfVec4f_get_unchecked(self.as_raw_VectorOfVec4f(), index)
				.into_result()
				.expect("Infallible function failed: VectorOfVec4f::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVec4f_set(instance: *mut c_void, index: size_t, val: *const core::Vec4f) -> sys::Result_void; }
			unsafe { cv_VectorOfVec4f_set(self.as_raw_VectorOfVec4f(), index, &val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVec4f_set_unchecked(instance: *mut c_void, index: size_t, val: *const core::Vec4f); }
			cv_VectorOfVec4f_set_unchecked(self.as_raw_VectorOfVec4f(), index, &val)
		}
	
		
		#[inline]
		fn to_vec(&self) -> Vec<Self::Storage> {
			self.to_slice().to_vec()
		}
	}
	
	unsafe impl Send for VectorOfVec4f {}
	
	impl core::ToInputArray for VectorOfVec4f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec4f_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec4f_input_array(self.as_raw_VectorOfVec4f()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfVec4f_output_array(self.as_raw_VectorOfVec4f()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfVec4f_input_output_array(self.as_raw_VectorOfVec4f()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVec4f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfVec4i {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfVec4i {
		pub fn as_raw_VectorOfVec4i(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
		
		pub fn to_slice(&self) -> &[core::Vec4i] {
			extern "C" { fn cv_VectorOfVec4i_data(instance: *mut c_void) -> *const core::Vec4i; }
			unsafe {
				let data = cv_VectorOfVec4i_data(self.as_raw_VectorOfVec4i());
				::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
			}
		}
	}
	
	impl Drop for VectorOfVec4i {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfVec4i_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfVec4i_delete(self.as_raw_VectorOfVec4i()) };
		}
	}
	
	impl IntoIterator for VectorOfVec4i {
		type Item = core::Vec4i;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfVec4i {
		type Item = core::Vec4i;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVec4i>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfVec4i {
		type Storage = core::Vec4i;
		type Arg = core::Vec4i;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfVec4i_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfVec4i_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfVec4i_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVec4i_len(self.as_raw_VectorOfVec4i()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfVec4i_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfVec4i_is_empty(self.as_raw_VectorOfVec4i()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfVec4i_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVec4i_capacity(self.as_raw_VectorOfVec4i()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfVec4i_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfVec4i_shrink_to_fit(self.as_raw_VectorOfVec4i()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfVec4i_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfVec4i_reserve(self.as_raw_VectorOfVec4i(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfVec4i_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfVec4i_remove(self.as_raw_VectorOfVec4i(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfVec4i_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfVec4i_swap(self.as_raw_VectorOfVec4i(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfVec4i_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfVec4i_clear(self.as_raw_VectorOfVec4i()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVec4i_push(instance: *mut c_void, val: *const core::Vec4i); }
			unsafe { cv_VectorOfVec4i_push(self.as_raw_VectorOfVec4i(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVec4i_insert(instance: *mut c_void, index: size_t, val: *const core::Vec4i); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVec4i_insert(self.as_raw_VectorOfVec4i(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfVec4i_get(instance: *mut c_void, index: size_t) -> sys::Result<core::Vec4i>; }
			unsafe { cv_VectorOfVec4i_get(self.as_raw_VectorOfVec4i(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfVec4i_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<core::Vec4i>; }
			cv_VectorOfVec4i_get_unchecked(self.as_raw_VectorOfVec4i(), index)
				.into_result()
				.expect("Infallible function failed: VectorOfVec4i::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVec4i_set(instance: *mut c_void, index: size_t, val: *const core::Vec4i) -> sys::Result_void; }
			unsafe { cv_VectorOfVec4i_set(self.as_raw_VectorOfVec4i(), index, &val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVec4i_set_unchecked(instance: *mut c_void, index: size_t, val: *const core::Vec4i); }
			cv_VectorOfVec4i_set_unchecked(self.as_raw_VectorOfVec4i(), index, &val)
		}
	
		
		#[inline]
		fn to_vec(&self) -> Vec<Self::Storage> {
			self.to_slice().to_vec()
		}
	}
	
	unsafe impl Send for VectorOfVec4i {}
	
	impl core::ToInputArray for VectorOfVec4i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec4i_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec4i_input_array(self.as_raw_VectorOfVec4i()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfVec4i_output_array(self.as_raw_VectorOfVec4i()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfVec4i_input_output_array(self.as_raw_VectorOfVec4i()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVec4i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfVec6f {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfVec6f {
		pub fn as_raw_VectorOfVec6f(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
		
		pub fn to_slice(&self) -> &[core::Vec6f] {
			extern "C" { fn cv_VectorOfVec6f_data(instance: *mut c_void) -> *const core::Vec6f; }
			unsafe {
				let data = cv_VectorOfVec6f_data(self.as_raw_VectorOfVec6f());
				::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
			}
		}
	}
	
	impl Drop for VectorOfVec6f {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfVec6f_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfVec6f_delete(self.as_raw_VectorOfVec6f()) };
		}
	}
	
	impl IntoIterator for VectorOfVec6f {
		type Item = core::Vec6f;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfVec6f {
		type Item = core::Vec6f;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVec6f>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfVec6f {
		type Storage = core::Vec6f;
		type Arg = core::Vec6f;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfVec6f_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfVec6f_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfVec6f_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVec6f_len(self.as_raw_VectorOfVec6f()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfVec6f_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfVec6f_is_empty(self.as_raw_VectorOfVec6f()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfVec6f_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVec6f_capacity(self.as_raw_VectorOfVec6f()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfVec6f_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfVec6f_shrink_to_fit(self.as_raw_VectorOfVec6f()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfVec6f_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfVec6f_reserve(self.as_raw_VectorOfVec6f(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfVec6f_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfVec6f_remove(self.as_raw_VectorOfVec6f(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfVec6f_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfVec6f_swap(self.as_raw_VectorOfVec6f(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfVec6f_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfVec6f_clear(self.as_raw_VectorOfVec6f()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVec6f_push(instance: *mut c_void, val: *const core::Vec6f); }
			unsafe { cv_VectorOfVec6f_push(self.as_raw_VectorOfVec6f(), &val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVec6f_insert(instance: *mut c_void, index: size_t, val: *const core::Vec6f); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVec6f_insert(self.as_raw_VectorOfVec6f(), index, &val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfVec6f_get(instance: *mut c_void, index: size_t) -> sys::Result<core::Vec6f>; }
			unsafe { cv_VectorOfVec6f_get(self.as_raw_VectorOfVec6f(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfVec6f_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<core::Vec6f>; }
			cv_VectorOfVec6f_get_unchecked(self.as_raw_VectorOfVec6f(), index)
				.into_result()
				.expect("Infallible function failed: VectorOfVec6f::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVec6f_set(instance: *mut c_void, index: size_t, val: *const core::Vec6f) -> sys::Result_void; }
			unsafe { cv_VectorOfVec6f_set(self.as_raw_VectorOfVec6f(), index, &val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVec6f_set_unchecked(instance: *mut c_void, index: size_t, val: *const core::Vec6f); }
			cv_VectorOfVec6f_set_unchecked(self.as_raw_VectorOfVec6f(), index, &val)
		}
	
		
		#[inline]
		fn to_vec(&self) -> Vec<Self::Storage> {
			self.to_slice().to_vec()
		}
	}
	
	unsafe impl Send for VectorOfVec6f {}
	
	impl core::ToInputArray for VectorOfVec6f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec6f_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVec6f_input_array(self.as_raw_VectorOfVec6f()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfVec6f_output_array(self.as_raw_VectorOfVec6f()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfVec6f_input_output_array(self.as_raw_VectorOfVec6f()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVec6f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfVectorOfDMatch {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfVectorOfDMatch {
		pub fn as_raw_VectorOfVectorOfDMatch(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfVectorOfDMatch {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfDMatch_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfDMatch_delete(self.as_raw_VectorOfVectorOfDMatch()) };
		}
	}
	
	impl IntoIterator for VectorOfVectorOfDMatch {
		type Item = types::VectorOfDMatch;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfVectorOfDMatch {
		type Item = types::VectorOfDMatch;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfDMatch>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfDMatch {
		type Storage = types::VectorOfDMatch;
		type Arg = types::VectorOfDMatch;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfVectorOfDMatch_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfVectorOfDMatch_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfDMatch_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfDMatch_len(self.as_raw_VectorOfVectorOfDMatch()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfVectorOfDMatch_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfVectorOfDMatch_is_empty(self.as_raw_VectorOfVectorOfDMatch()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfDMatch_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfDMatch_capacity(self.as_raw_VectorOfVectorOfDMatch()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfDMatch_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfDMatch_shrink_to_fit(self.as_raw_VectorOfVectorOfDMatch()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfVectorOfDMatch_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfVectorOfDMatch_reserve(self.as_raw_VectorOfVectorOfDMatch(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfVectorOfDMatch_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfVectorOfDMatch_remove(self.as_raw_VectorOfVectorOfDMatch(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfVectorOfDMatch_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfVectorOfDMatch_swap(self.as_raw_VectorOfVectorOfDMatch(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfDMatch_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfDMatch_clear(self.as_raw_VectorOfVectorOfDMatch()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfDMatch_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfDMatch_push(self.as_raw_VectorOfVectorOfDMatch(), val.as_raw_VectorOfDMatch()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfDMatch_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfDMatch_insert(self.as_raw_VectorOfVectorOfDMatch(), index, val.as_raw_VectorOfDMatch()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfVectorOfDMatch_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfDMatch_get(self.as_raw_VectorOfVectorOfDMatch(), index) }
				.into_result()
				.map(|ptr| types::VectorOfDMatch { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfVectorOfDMatch_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfVectorOfDMatch_get_unchecked(self.as_raw_VectorOfVectorOfDMatch(), index)
				.into_result()
				.map(|ptr| types::VectorOfDMatch { ptr })
				.expect("Infallible function failed: VectorOfVectorOfDMatch::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfDMatch_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfVectorOfDMatch_set(self.as_raw_VectorOfVectorOfDMatch(), index, val.as_raw_VectorOfDMatch()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfDMatch_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfDMatch_set_unchecked(self.as_raw_VectorOfVectorOfDMatch(), index, val.as_raw_VectorOfDMatch())
		}
	
	}
	
	unsafe impl Send for VectorOfVectorOfDMatch {}
	
	pub struct VectorOfVectorOfKeyPoint {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfVectorOfKeyPoint {
		pub fn as_raw_VectorOfVectorOfKeyPoint(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfVectorOfKeyPoint {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfKeyPoint_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfKeyPoint_delete(self.as_raw_VectorOfVectorOfKeyPoint()) };
		}
	}
	
	impl IntoIterator for VectorOfVectorOfKeyPoint {
		type Item = types::VectorOfKeyPoint;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfVectorOfKeyPoint {
		type Item = types::VectorOfKeyPoint;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfKeyPoint>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfKeyPoint {
		type Storage = types::VectorOfKeyPoint;
		type Arg = types::VectorOfKeyPoint;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfVectorOfKeyPoint_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfVectorOfKeyPoint_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfKeyPoint_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfKeyPoint_len(self.as_raw_VectorOfVectorOfKeyPoint()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfVectorOfKeyPoint_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfVectorOfKeyPoint_is_empty(self.as_raw_VectorOfVectorOfKeyPoint()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfKeyPoint_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfKeyPoint_capacity(self.as_raw_VectorOfVectorOfKeyPoint()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfKeyPoint_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfKeyPoint_shrink_to_fit(self.as_raw_VectorOfVectorOfKeyPoint()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfVectorOfKeyPoint_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfVectorOfKeyPoint_reserve(self.as_raw_VectorOfVectorOfKeyPoint(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfVectorOfKeyPoint_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfVectorOfKeyPoint_remove(self.as_raw_VectorOfVectorOfKeyPoint(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfVectorOfKeyPoint_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfVectorOfKeyPoint_swap(self.as_raw_VectorOfVectorOfKeyPoint(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfKeyPoint_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfKeyPoint_clear(self.as_raw_VectorOfVectorOfKeyPoint()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfKeyPoint_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfKeyPoint_push(self.as_raw_VectorOfVectorOfKeyPoint(), val.as_raw_VectorOfKeyPoint()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfKeyPoint_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfKeyPoint_insert(self.as_raw_VectorOfVectorOfKeyPoint(), index, val.as_raw_VectorOfKeyPoint()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfVectorOfKeyPoint_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfKeyPoint_get(self.as_raw_VectorOfVectorOfKeyPoint(), index) }
				.into_result()
				.map(|ptr| types::VectorOfKeyPoint { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfVectorOfKeyPoint_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfVectorOfKeyPoint_get_unchecked(self.as_raw_VectorOfVectorOfKeyPoint(), index)
				.into_result()
				.map(|ptr| types::VectorOfKeyPoint { ptr })
				.expect("Infallible function failed: VectorOfVectorOfKeyPoint::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfKeyPoint_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfVectorOfKeyPoint_set(self.as_raw_VectorOfVectorOfKeyPoint(), index, val.as_raw_VectorOfKeyPoint()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfKeyPoint_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfKeyPoint_set_unchecked(self.as_raw_VectorOfVectorOfKeyPoint(), index, val.as_raw_VectorOfKeyPoint())
		}
	
	}
	
	unsafe impl Send for VectorOfVectorOfKeyPoint {}
	
	pub struct VectorOfVectorOfPoint {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfVectorOfPoint {
		pub fn as_raw_VectorOfVectorOfPoint(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfVectorOfPoint {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfPoint_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint_delete(self.as_raw_VectorOfVectorOfPoint()) };
		}
	}
	
	impl IntoIterator for VectorOfVectorOfPoint {
		type Item = types::VectorOfPoint;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfVectorOfPoint {
		type Item = types::VectorOfPoint;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfPoint>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfPoint {
		type Storage = types::VectorOfPoint;
		type Arg = types::VectorOfPoint;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfVectorOfPoint_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfVectorOfPoint_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfPoint_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfPoint_len(self.as_raw_VectorOfVectorOfPoint()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfVectorOfPoint_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfVectorOfPoint_is_empty(self.as_raw_VectorOfVectorOfPoint()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfPoint_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfPoint_capacity(self.as_raw_VectorOfVectorOfPoint()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfPoint_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint_shrink_to_fit(self.as_raw_VectorOfVectorOfPoint()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfVectorOfPoint_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfVectorOfPoint_reserve(self.as_raw_VectorOfVectorOfPoint(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfVectorOfPoint_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfVectorOfPoint_remove(self.as_raw_VectorOfVectorOfPoint(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfVectorOfPoint_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfVectorOfPoint_swap(self.as_raw_VectorOfVectorOfPoint(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfPoint_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint_clear(self.as_raw_VectorOfVectorOfPoint()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfPoint_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint_push(self.as_raw_VectorOfVectorOfPoint(), val.as_raw_VectorOfPoint()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfPoint_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfPoint_insert(self.as_raw_VectorOfVectorOfPoint(), index, val.as_raw_VectorOfPoint()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfVectorOfPoint_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint_get(self.as_raw_VectorOfVectorOfPoint(), index) }
				.into_result()
				.map(|ptr| types::VectorOfPoint { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfVectorOfPoint_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfVectorOfPoint_get_unchecked(self.as_raw_VectorOfVectorOfPoint(), index)
				.into_result()
				.map(|ptr| types::VectorOfPoint { ptr })
				.expect("Infallible function failed: VectorOfVectorOfPoint::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfPoint_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfVectorOfPoint_set(self.as_raw_VectorOfVectorOfPoint(), index, val.as_raw_VectorOfPoint()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfPoint_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfPoint_set_unchecked(self.as_raw_VectorOfVectorOfPoint(), index, val.as_raw_VectorOfPoint())
		}
	
	}
	
	unsafe impl Send for VectorOfVectorOfPoint {}
	
	impl core::ToInputArray for VectorOfVectorOfPoint {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint_input_array(self.as_raw_VectorOfVectorOfPoint()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfVectorOfPoint_output_array(self.as_raw_VectorOfVectorOfPoint()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfVectorOfPoint_input_output_array(self.as_raw_VectorOfVectorOfPoint()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfPoint {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfVectorOfPoint2f {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfVectorOfPoint2f {
		pub fn as_raw_VectorOfVectorOfPoint2f(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfVectorOfPoint2f {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint2f_delete(self.as_raw_VectorOfVectorOfPoint2f()) };
		}
	}
	
	impl IntoIterator for VectorOfVectorOfPoint2f {
		type Item = types::VectorOfPoint2f;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfVectorOfPoint2f {
		type Item = types::VectorOfPoint2f;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfPoint2f>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfPoint2f {
		type Storage = types::VectorOfPoint2f;
		type Arg = types::VectorOfPoint2f;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfVectorOfPoint2f_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfPoint2f_len(self.as_raw_VectorOfVectorOfPoint2f()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfVectorOfPoint2f_is_empty(self.as_raw_VectorOfVectorOfPoint2f()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfPoint2f_capacity(self.as_raw_VectorOfVectorOfPoint2f()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint2f_shrink_to_fit(self.as_raw_VectorOfVectorOfPoint2f()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfVectorOfPoint2f_reserve(self.as_raw_VectorOfVectorOfPoint2f(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfVectorOfPoint2f_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfVectorOfPoint2f_remove(self.as_raw_VectorOfVectorOfPoint2f(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfVectorOfPoint2f_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfVectorOfPoint2f_swap(self.as_raw_VectorOfVectorOfPoint2f(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint2f_clear(self.as_raw_VectorOfVectorOfPoint2f()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint2f_push(self.as_raw_VectorOfVectorOfPoint2f(), val.as_raw_VectorOfPoint2f()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfPoint2f_insert(self.as_raw_VectorOfVectorOfPoint2f(), index, val.as_raw_VectorOfPoint2f()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint2f_get(self.as_raw_VectorOfVectorOfPoint2f(), index) }
				.into_result()
				.map(|ptr| types::VectorOfPoint2f { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfVectorOfPoint2f_get_unchecked(self.as_raw_VectorOfVectorOfPoint2f(), index)
				.into_result()
				.map(|ptr| types::VectorOfPoint2f { ptr })
				.expect("Infallible function failed: VectorOfVectorOfPoint2f::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfVectorOfPoint2f_set(self.as_raw_VectorOfVectorOfPoint2f(), index, val.as_raw_VectorOfPoint2f()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfPoint2f_set_unchecked(self.as_raw_VectorOfVectorOfPoint2f(), index, val.as_raw_VectorOfPoint2f())
		}
	
	}
	
	unsafe impl Send for VectorOfVectorOfPoint2f {}
	
	impl core::ToInputArray for VectorOfVectorOfPoint2f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint2f_input_array(self.as_raw_VectorOfVectorOfPoint2f()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfVectorOfPoint2f_output_array(self.as_raw_VectorOfVectorOfPoint2f()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfVectorOfPoint2f_input_output_array(self.as_raw_VectorOfVectorOfPoint2f()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfPoint2f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfVectorOfPoint3d {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfVectorOfPoint3d {
		pub fn as_raw_VectorOfVectorOfPoint3d(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfVectorOfPoint3d {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint3d_delete(self.as_raw_VectorOfVectorOfPoint3d()) };
		}
	}
	
	impl IntoIterator for VectorOfVectorOfPoint3d {
		type Item = types::VectorOfPoint3d;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfVectorOfPoint3d {
		type Item = types::VectorOfPoint3d;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfPoint3d>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfPoint3d {
		type Storage = types::VectorOfPoint3d;
		type Arg = types::VectorOfPoint3d;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfVectorOfPoint3d_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfPoint3d_len(self.as_raw_VectorOfVectorOfPoint3d()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfVectorOfPoint3d_is_empty(self.as_raw_VectorOfVectorOfPoint3d()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfPoint3d_capacity(self.as_raw_VectorOfVectorOfPoint3d()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint3d_shrink_to_fit(self.as_raw_VectorOfVectorOfPoint3d()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfVectorOfPoint3d_reserve(self.as_raw_VectorOfVectorOfPoint3d(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfVectorOfPoint3d_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfVectorOfPoint3d_remove(self.as_raw_VectorOfVectorOfPoint3d(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfVectorOfPoint3d_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfVectorOfPoint3d_swap(self.as_raw_VectorOfVectorOfPoint3d(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint3d_clear(self.as_raw_VectorOfVectorOfPoint3d()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint3d_push(self.as_raw_VectorOfVectorOfPoint3d(), val.as_raw_VectorOfPoint3d()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfPoint3d_insert(self.as_raw_VectorOfVectorOfPoint3d(), index, val.as_raw_VectorOfPoint3d()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3d_get(self.as_raw_VectorOfVectorOfPoint3d(), index) }
				.into_result()
				.map(|ptr| types::VectorOfPoint3d { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfVectorOfPoint3d_get_unchecked(self.as_raw_VectorOfVectorOfPoint3d(), index)
				.into_result()
				.map(|ptr| types::VectorOfPoint3d { ptr })
				.expect("Infallible function failed: VectorOfVectorOfPoint3d::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfVectorOfPoint3d_set(self.as_raw_VectorOfVectorOfPoint3d(), index, val.as_raw_VectorOfPoint3d()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfPoint3d_set_unchecked(self.as_raw_VectorOfVectorOfPoint3d(), index, val.as_raw_VectorOfPoint3d())
		}
	
	}
	
	unsafe impl Send for VectorOfVectorOfPoint3d {}
	
	impl core::ToInputArray for VectorOfVectorOfPoint3d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3d_input_array(self.as_raw_VectorOfVectorOfPoint3d()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfVectorOfPoint3d_output_array(self.as_raw_VectorOfVectorOfPoint3d()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfVectorOfPoint3d_input_output_array(self.as_raw_VectorOfVectorOfPoint3d()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfPoint3d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfVectorOfPoint3f {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfVectorOfPoint3f {
		pub fn as_raw_VectorOfVectorOfPoint3f(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfVectorOfPoint3f {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint3f_delete(self.as_raw_VectorOfVectorOfPoint3f()) };
		}
	}
	
	impl IntoIterator for VectorOfVectorOfPoint3f {
		type Item = types::VectorOfPoint3f;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfVectorOfPoint3f {
		type Item = types::VectorOfPoint3f;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfPoint3f>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfPoint3f {
		type Storage = types::VectorOfPoint3f;
		type Arg = types::VectorOfPoint3f;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfVectorOfPoint3f_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfPoint3f_len(self.as_raw_VectorOfVectorOfPoint3f()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfVectorOfPoint3f_is_empty(self.as_raw_VectorOfVectorOfPoint3f()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfPoint3f_capacity(self.as_raw_VectorOfVectorOfPoint3f()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint3f_shrink_to_fit(self.as_raw_VectorOfVectorOfPoint3f()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfVectorOfPoint3f_reserve(self.as_raw_VectorOfVectorOfPoint3f(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfVectorOfPoint3f_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfVectorOfPoint3f_remove(self.as_raw_VectorOfVectorOfPoint3f(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfVectorOfPoint3f_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfVectorOfPoint3f_swap(self.as_raw_VectorOfVectorOfPoint3f(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint3f_clear(self.as_raw_VectorOfVectorOfPoint3f()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint3f_push(self.as_raw_VectorOfVectorOfPoint3f(), val.as_raw_VectorOfPoint3f()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfPoint3f_insert(self.as_raw_VectorOfVectorOfPoint3f(), index, val.as_raw_VectorOfPoint3f()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3f_get(self.as_raw_VectorOfVectorOfPoint3f(), index) }
				.into_result()
				.map(|ptr| types::VectorOfPoint3f { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfVectorOfPoint3f_get_unchecked(self.as_raw_VectorOfVectorOfPoint3f(), index)
				.into_result()
				.map(|ptr| types::VectorOfPoint3f { ptr })
				.expect("Infallible function failed: VectorOfVectorOfPoint3f::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfVectorOfPoint3f_set(self.as_raw_VectorOfVectorOfPoint3f(), index, val.as_raw_VectorOfPoint3f()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfPoint3f_set_unchecked(self.as_raw_VectorOfVectorOfPoint3f(), index, val.as_raw_VectorOfPoint3f())
		}
	
	}
	
	unsafe impl Send for VectorOfVectorOfPoint3f {}
	
	impl core::ToInputArray for VectorOfVectorOfPoint3f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3f_input_array(self.as_raw_VectorOfVectorOfPoint3f()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfVectorOfPoint3f_output_array(self.as_raw_VectorOfVectorOfPoint3f()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfVectorOfPoint3f_input_output_array(self.as_raw_VectorOfVectorOfPoint3f()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfPoint3f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfVectorOfPoint3i {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfVectorOfPoint3i {
		pub fn as_raw_VectorOfVectorOfPoint3i(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfVectorOfPoint3i {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint3i_delete(self.as_raw_VectorOfVectorOfPoint3i()) };
		}
	}
	
	impl IntoIterator for VectorOfVectorOfPoint3i {
		type Item = types::VectorOfPoint3i;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfVectorOfPoint3i {
		type Item = types::VectorOfPoint3i;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfPoint3i>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfPoint3i {
		type Storage = types::VectorOfPoint3i;
		type Arg = types::VectorOfPoint3i;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfVectorOfPoint3i_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfPoint3i_len(self.as_raw_VectorOfVectorOfPoint3i()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfVectorOfPoint3i_is_empty(self.as_raw_VectorOfVectorOfPoint3i()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfPoint3i_capacity(self.as_raw_VectorOfVectorOfPoint3i()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint3i_shrink_to_fit(self.as_raw_VectorOfVectorOfPoint3i()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfVectorOfPoint3i_reserve(self.as_raw_VectorOfVectorOfPoint3i(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfVectorOfPoint3i_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfVectorOfPoint3i_remove(self.as_raw_VectorOfVectorOfPoint3i(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfVectorOfPoint3i_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfVectorOfPoint3i_swap(self.as_raw_VectorOfVectorOfPoint3i(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint3i_clear(self.as_raw_VectorOfVectorOfPoint3i()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfPoint3i_push(self.as_raw_VectorOfVectorOfPoint3i(), val.as_raw_VectorOfPoint3i()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfPoint3i_insert(self.as_raw_VectorOfVectorOfPoint3i(), index, val.as_raw_VectorOfPoint3i()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3i_get(self.as_raw_VectorOfVectorOfPoint3i(), index) }
				.into_result()
				.map(|ptr| types::VectorOfPoint3i { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfVectorOfPoint3i_get_unchecked(self.as_raw_VectorOfVectorOfPoint3i(), index)
				.into_result()
				.map(|ptr| types::VectorOfPoint3i { ptr })
				.expect("Infallible function failed: VectorOfVectorOfPoint3i::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfVectorOfPoint3i_set(self.as_raw_VectorOfVectorOfPoint3i(), index, val.as_raw_VectorOfPoint3i()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfPoint3i_set_unchecked(self.as_raw_VectorOfVectorOfPoint3i(), index, val.as_raw_VectorOfPoint3i())
		}
	
	}
	
	unsafe impl Send for VectorOfVectorOfPoint3i {}
	
	impl core::ToInputArray for VectorOfVectorOfPoint3i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfPoint3i_input_array(self.as_raw_VectorOfVectorOfPoint3i()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfVectorOfPoint3i_output_array(self.as_raw_VectorOfVectorOfPoint3i()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfVectorOfPoint3i_input_output_array(self.as_raw_VectorOfVectorOfPoint3i()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfPoint3i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfVectorOfRect {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfVectorOfRect {
		pub fn as_raw_VectorOfVectorOfRect(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfVectorOfRect {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfRect_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfRect_delete(self.as_raw_VectorOfVectorOfRect()) };
		}
	}
	
	impl IntoIterator for VectorOfVectorOfRect {
		type Item = types::VectorOfRect;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfVectorOfRect {
		type Item = types::VectorOfRect;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfRect>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfRect {
		type Storage = types::VectorOfRect;
		type Arg = types::VectorOfRect;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfVectorOfRect_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfVectorOfRect_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfRect_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfRect_len(self.as_raw_VectorOfVectorOfRect()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfVectorOfRect_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfVectorOfRect_is_empty(self.as_raw_VectorOfVectorOfRect()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfRect_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfRect_capacity(self.as_raw_VectorOfVectorOfRect()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfRect_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfRect_shrink_to_fit(self.as_raw_VectorOfVectorOfRect()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfVectorOfRect_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfVectorOfRect_reserve(self.as_raw_VectorOfVectorOfRect(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfVectorOfRect_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfVectorOfRect_remove(self.as_raw_VectorOfVectorOfRect(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfVectorOfRect_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfVectorOfRect_swap(self.as_raw_VectorOfVectorOfRect(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfRect_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfRect_clear(self.as_raw_VectorOfVectorOfRect()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfRect_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfRect_push(self.as_raw_VectorOfVectorOfRect(), val.as_raw_VectorOfRect()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfRect_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfRect_insert(self.as_raw_VectorOfVectorOfRect(), index, val.as_raw_VectorOfRect()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfVectorOfRect_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfRect_get(self.as_raw_VectorOfVectorOfRect(), index) }
				.into_result()
				.map(|ptr| types::VectorOfRect { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfVectorOfRect_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfVectorOfRect_get_unchecked(self.as_raw_VectorOfVectorOfRect(), index)
				.into_result()
				.map(|ptr| types::VectorOfRect { ptr })
				.expect("Infallible function failed: VectorOfVectorOfRect::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfRect_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfVectorOfRect_set(self.as_raw_VectorOfVectorOfRect(), index, val.as_raw_VectorOfRect()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfRect_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfRect_set_unchecked(self.as_raw_VectorOfVectorOfRect(), index, val.as_raw_VectorOfRect())
		}
	
	}
	
	unsafe impl Send for VectorOfVectorOfRect {}
	
	impl core::ToInputArray for VectorOfVectorOfRect {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfRect_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfRect_input_array(self.as_raw_VectorOfVectorOfRect()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfVectorOfRect_output_array(self.as_raw_VectorOfVectorOfRect()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfVectorOfRect_input_output_array(self.as_raw_VectorOfVectorOfRect()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfRect {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfVectorOfVec2i {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfVectorOfVec2i {
		pub fn as_raw_VectorOfVectorOfVec2i(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfVectorOfVec2i {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfVec2i_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfVec2i_delete(self.as_raw_VectorOfVectorOfVec2i()) };
		}
	}
	
	impl IntoIterator for VectorOfVectorOfVec2i {
		type Item = types::VectorOfVec2i;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfVectorOfVec2i {
		type Item = types::VectorOfVec2i;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfVec2i>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfVec2i {
		type Storage = types::VectorOfVec2i;
		type Arg = types::VectorOfVec2i;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfVectorOfVec2i_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfVectorOfVec2i_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfVec2i_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfVec2i_len(self.as_raw_VectorOfVectorOfVec2i()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfVectorOfVec2i_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfVectorOfVec2i_is_empty(self.as_raw_VectorOfVectorOfVec2i()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfVec2i_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfVec2i_capacity(self.as_raw_VectorOfVectorOfVec2i()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfVec2i_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfVec2i_shrink_to_fit(self.as_raw_VectorOfVectorOfVec2i()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfVectorOfVec2i_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfVectorOfVec2i_reserve(self.as_raw_VectorOfVectorOfVec2i(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfVectorOfVec2i_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfVectorOfVec2i_remove(self.as_raw_VectorOfVectorOfVec2i(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfVectorOfVec2i_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfVectorOfVec2i_swap(self.as_raw_VectorOfVectorOfVec2i(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfVec2i_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfVec2i_clear(self.as_raw_VectorOfVectorOfVec2i()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfVec2i_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfVec2i_push(self.as_raw_VectorOfVectorOfVec2i(), val.as_raw_VectorOfVec2i()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfVec2i_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfVec2i_insert(self.as_raw_VectorOfVectorOfVec2i(), index, val.as_raw_VectorOfVec2i()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfVectorOfVec2i_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfVec2i_get(self.as_raw_VectorOfVectorOfVec2i(), index) }
				.into_result()
				.map(|ptr| types::VectorOfVec2i { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfVectorOfVec2i_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfVectorOfVec2i_get_unchecked(self.as_raw_VectorOfVectorOfVec2i(), index)
				.into_result()
				.map(|ptr| types::VectorOfVec2i { ptr })
				.expect("Infallible function failed: VectorOfVectorOfVec2i::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfVec2i_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfVectorOfVec2i_set(self.as_raw_VectorOfVectorOfVec2i(), index, val.as_raw_VectorOfVec2i()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfVec2i_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfVec2i_set_unchecked(self.as_raw_VectorOfVectorOfVec2i(), index, val.as_raw_VectorOfVec2i())
		}
	
	}
	
	unsafe impl Send for VectorOfVectorOfVec2i {}
	
	impl core::ToInputArray for VectorOfVectorOfVec2i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfVec2i_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfVec2i_input_array(self.as_raw_VectorOfVectorOfVec2i()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfVectorOfVec2i_output_array(self.as_raw_VectorOfVectorOfVec2i()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfVectorOfVec2i_input_output_array(self.as_raw_VectorOfVectorOfVec2i()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfVec2i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfVectorOff64 {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfVectorOff64 {
		pub fn as_raw_VectorOfVectorOff64(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfVectorOff64 {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfVectorOff64_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOff64_delete(self.as_raw_VectorOfVectorOff64()) };
		}
	}
	
	impl IntoIterator for VectorOfVectorOff64 {
		type Item = types::VectorOff64;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfVectorOff64 {
		type Item = types::VectorOff64;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOff64>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfVectorOff64 {
		type Storage = types::VectorOff64;
		type Arg = types::VectorOff64;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfVectorOff64_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfVectorOff64_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOff64_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOff64_len(self.as_raw_VectorOfVectorOff64()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfVectorOff64_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfVectorOff64_is_empty(self.as_raw_VectorOfVectorOff64()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOff64_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOff64_capacity(self.as_raw_VectorOfVectorOff64()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfVectorOff64_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOff64_shrink_to_fit(self.as_raw_VectorOfVectorOff64()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfVectorOff64_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfVectorOff64_reserve(self.as_raw_VectorOfVectorOff64(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfVectorOff64_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfVectorOff64_remove(self.as_raw_VectorOfVectorOff64(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfVectorOff64_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfVectorOff64_swap(self.as_raw_VectorOfVectorOff64(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfVectorOff64_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOff64_clear(self.as_raw_VectorOfVectorOff64()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOff64_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOff64_push(self.as_raw_VectorOfVectorOff64(), val.as_raw_VectorOff64()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOff64_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOff64_insert(self.as_raw_VectorOfVectorOff64(), index, val.as_raw_VectorOff64()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfVectorOff64_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOff64_get(self.as_raw_VectorOfVectorOff64(), index) }
				.into_result()
				.map(|ptr| types::VectorOff64 { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfVectorOff64_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfVectorOff64_get_unchecked(self.as_raw_VectorOfVectorOff64(), index)
				.into_result()
				.map(|ptr| types::VectorOff64 { ptr })
				.expect("Infallible function failed: VectorOfVectorOff64::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOff64_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfVectorOff64_set(self.as_raw_VectorOfVectorOff64(), index, val.as_raw_VectorOff64()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOff64_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOff64_set_unchecked(self.as_raw_VectorOfVectorOff64(), index, val.as_raw_VectorOff64())
		}
	
	}
	
	unsafe impl Send for VectorOfVectorOff64 {}
	
	impl core::ToInputArray for VectorOfVectorOff64 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOff64_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOff64_input_array(self.as_raw_VectorOfVectorOff64()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfVectorOff64_output_array(self.as_raw_VectorOfVectorOff64()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfVectorOff64_input_output_array(self.as_raw_VectorOfVectorOff64()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOff64 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfVectorOfi32 {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfVectorOfi32 {
		pub fn as_raw_VectorOfVectorOfi32(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfVectorOfi32 {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfi32_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfi32_delete(self.as_raw_VectorOfVectorOfi32()) };
		}
	}
	
	impl IntoIterator for VectorOfVectorOfi32 {
		type Item = types::VectorOfi32;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfVectorOfi32 {
		type Item = types::VectorOfi32;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfi32>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfi32 {
		type Storage = types::VectorOfi32;
		type Arg = types::VectorOfi32;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfVectorOfi32_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfVectorOfi32_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfi32_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfi32_len(self.as_raw_VectorOfVectorOfi32()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfVectorOfi32_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfVectorOfi32_is_empty(self.as_raw_VectorOfVectorOfi32()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfi32_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfi32_capacity(self.as_raw_VectorOfVectorOfi32()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfi32_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfi32_shrink_to_fit(self.as_raw_VectorOfVectorOfi32()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfVectorOfi32_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfVectorOfi32_reserve(self.as_raw_VectorOfVectorOfi32(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfVectorOfi32_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfVectorOfi32_remove(self.as_raw_VectorOfVectorOfi32(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfVectorOfi32_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfVectorOfi32_swap(self.as_raw_VectorOfVectorOfi32(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfi32_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfi32_clear(self.as_raw_VectorOfVectorOfi32()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfi32_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfi32_push(self.as_raw_VectorOfVectorOfi32(), val.as_raw_VectorOfi32()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfi32_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfi32_insert(self.as_raw_VectorOfVectorOfi32(), index, val.as_raw_VectorOfi32()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfVectorOfi32_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfi32_get(self.as_raw_VectorOfVectorOfi32(), index) }
				.into_result()
				.map(|ptr| types::VectorOfi32 { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfVectorOfi32_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfVectorOfi32_get_unchecked(self.as_raw_VectorOfVectorOfi32(), index)
				.into_result()
				.map(|ptr| types::VectorOfi32 { ptr })
				.expect("Infallible function failed: VectorOfVectorOfi32::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfi32_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfVectorOfi32_set(self.as_raw_VectorOfVectorOfi32(), index, val.as_raw_VectorOfi32()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfi32_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfi32_set_unchecked(self.as_raw_VectorOfVectorOfi32(), index, val.as_raw_VectorOfi32())
		}
	
	}
	
	unsafe impl Send for VectorOfVectorOfi32 {}
	
	impl core::ToInputArray for VectorOfVectorOfi32 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfi32_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfi32_input_array(self.as_raw_VectorOfVectorOfi32()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfVectorOfi32_output_array(self.as_raw_VectorOfVectorOfi32()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfVectorOfi32_input_output_array(self.as_raw_VectorOfVectorOfi32()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfi32 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfVectorOfi8 {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfVectorOfi8 {
		pub fn as_raw_VectorOfVectorOfi8(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfVectorOfi8 {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfi8_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfi8_delete(self.as_raw_VectorOfVectorOfi8()) };
		}
	}
	
	impl IntoIterator for VectorOfVectorOfi8 {
		type Item = types::VectorOfi8;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfVectorOfi8 {
		type Item = types::VectorOfi8;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfi8>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfi8 {
		type Storage = types::VectorOfi8;
		type Arg = types::VectorOfi8;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfVectorOfi8_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfVectorOfi8_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfi8_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfi8_len(self.as_raw_VectorOfVectorOfi8()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfVectorOfi8_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfVectorOfi8_is_empty(self.as_raw_VectorOfVectorOfi8()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfi8_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfi8_capacity(self.as_raw_VectorOfVectorOfi8()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfi8_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfi8_shrink_to_fit(self.as_raw_VectorOfVectorOfi8()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfVectorOfi8_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfVectorOfi8_reserve(self.as_raw_VectorOfVectorOfi8(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfVectorOfi8_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfVectorOfi8_remove(self.as_raw_VectorOfVectorOfi8(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfVectorOfi8_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfVectorOfi8_swap(self.as_raw_VectorOfVectorOfi8(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfi8_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfi8_clear(self.as_raw_VectorOfVectorOfi8()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfi8_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfi8_push(self.as_raw_VectorOfVectorOfi8(), val.as_raw_VectorOfi8()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfi8_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfi8_insert(self.as_raw_VectorOfVectorOfi8(), index, val.as_raw_VectorOfi8()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfVectorOfi8_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfi8_get(self.as_raw_VectorOfVectorOfi8(), index) }
				.into_result()
				.map(|ptr| types::VectorOfi8 { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfVectorOfi8_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfVectorOfi8_get_unchecked(self.as_raw_VectorOfVectorOfi8(), index)
				.into_result()
				.map(|ptr| types::VectorOfi8 { ptr })
				.expect("Infallible function failed: VectorOfVectorOfi8::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfi8_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfVectorOfi8_set(self.as_raw_VectorOfVectorOfi8(), index, val.as_raw_VectorOfi8()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfi8_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfi8_set_unchecked(self.as_raw_VectorOfVectorOfi8(), index, val.as_raw_VectorOfi8())
		}
	
	}
	
	unsafe impl Send for VectorOfVectorOfi8 {}
	
	impl core::ToInputArray for VectorOfVectorOfi8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfi8_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfi8_input_array(self.as_raw_VectorOfVectorOfi8()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfVectorOfi8_output_array(self.as_raw_VectorOfVectorOfi8()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfVectorOfi8_input_output_array(self.as_raw_VectorOfVectorOfi8()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfi8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfVectorOfu8 {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfVectorOfu8 {
		pub fn as_raw_VectorOfVectorOfu8(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfVectorOfu8 {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfu8_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfu8_delete(self.as_raw_VectorOfVectorOfu8()) };
		}
	}
	
	impl IntoIterator for VectorOfVectorOfu8 {
		type Item = types::VectorOfu8;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfVectorOfu8 {
		type Item = types::VectorOfu8;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfu8>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfu8 {
		type Storage = types::VectorOfu8;
		type Arg = types::VectorOfu8;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfVectorOfu8_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfVectorOfu8_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfu8_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfu8_len(self.as_raw_VectorOfVectorOfu8()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfVectorOfu8_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfVectorOfu8_is_empty(self.as_raw_VectorOfVectorOfu8()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfu8_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfu8_capacity(self.as_raw_VectorOfVectorOfu8()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfu8_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfu8_shrink_to_fit(self.as_raw_VectorOfVectorOfu8()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfVectorOfu8_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfVectorOfu8_reserve(self.as_raw_VectorOfVectorOfu8(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfVectorOfu8_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfVectorOfu8_remove(self.as_raw_VectorOfVectorOfu8(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfVectorOfu8_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfVectorOfu8_swap(self.as_raw_VectorOfVectorOfu8(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfu8_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfu8_clear(self.as_raw_VectorOfVectorOfu8()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfu8_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfu8_push(self.as_raw_VectorOfVectorOfu8(), val.as_raw_VectorOfu8()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfu8_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfu8_insert(self.as_raw_VectorOfVectorOfu8(), index, val.as_raw_VectorOfu8()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfVectorOfu8_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfu8_get(self.as_raw_VectorOfVectorOfu8(), index) }
				.into_result()
				.map(|ptr| types::VectorOfu8 { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfVectorOfu8_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfVectorOfu8_get_unchecked(self.as_raw_VectorOfVectorOfu8(), index)
				.into_result()
				.map(|ptr| types::VectorOfu8 { ptr })
				.expect("Infallible function failed: VectorOfVectorOfu8::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfu8_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfVectorOfu8_set(self.as_raw_VectorOfVectorOfu8(), index, val.as_raw_VectorOfu8()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfu8_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfu8_set_unchecked(self.as_raw_VectorOfVectorOfu8(), index, val.as_raw_VectorOfu8())
		}
	
	}
	
	unsafe impl Send for VectorOfVectorOfu8 {}
	
	impl core::ToInputArray for VectorOfVectorOfu8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfu8_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfu8_input_array(self.as_raw_VectorOfVectorOfu8()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfVectorOfu8_output_array(self.as_raw_VectorOfVectorOfu8()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfVectorOfu8_input_output_array(self.as_raw_VectorOfVectorOfu8()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfVectorOfu8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfbool {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfbool {
		pub fn as_raw_VectorOfbool(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfbool {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfbool_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfbool_delete(self.as_raw_VectorOfbool()) };
		}
	}
	
	impl IntoIterator for VectorOfbool {
		type Item = bool;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfbool {
		type Item = bool;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfbool>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfbool {
		type Storage = bool;
		type Arg = bool;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfbool_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfbool_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfbool_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfbool_len(self.as_raw_VectorOfbool()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfbool_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfbool_is_empty(self.as_raw_VectorOfbool()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfbool_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfbool_capacity(self.as_raw_VectorOfbool()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfbool_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfbool_shrink_to_fit(self.as_raw_VectorOfbool()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfbool_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfbool_reserve(self.as_raw_VectorOfbool(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfbool_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfbool_remove(self.as_raw_VectorOfbool(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfbool_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfbool_swap(self.as_raw_VectorOfbool(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfbool_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfbool_clear(self.as_raw_VectorOfbool()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfbool_push(instance: *mut c_void, val: bool); }
			unsafe { cv_VectorOfbool_push(self.as_raw_VectorOfbool(), val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfbool_insert(instance: *mut c_void, index: size_t, val: bool); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfbool_insert(self.as_raw_VectorOfbool(), index, val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfbool_get(instance: *mut c_void, index: size_t) -> sys::Result<bool>; }
			unsafe { cv_VectorOfbool_get(self.as_raw_VectorOfbool(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfbool_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<bool>; }
			cv_VectorOfbool_get_unchecked(self.as_raw_VectorOfbool(), index)
				.into_result()
				.expect("Infallible function failed: VectorOfbool::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfbool_set(instance: *mut c_void, index: size_t, val: bool) -> sys::Result_void; }
			unsafe { cv_VectorOfbool_set(self.as_raw_VectorOfbool(), index, val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfbool_set_unchecked(instance: *mut c_void, index: size_t, val: bool); }
			cv_VectorOfbool_set_unchecked(self.as_raw_VectorOfbool(), index, val)
		}
	
	}
	
	unsafe impl Send for VectorOfbool {}
	
	pub struct VectorOff32 {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOff32 {
		pub fn as_raw_VectorOff32(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
		
		pub fn to_slice(&self) -> &[f32] {
			extern "C" { fn cv_VectorOff32_data(instance: *mut c_void) -> *const f32; }
			unsafe {
				let data = cv_VectorOff32_data(self.as_raw_VectorOff32());
				::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
			}
		}
	}
	
	impl Drop for VectorOff32 {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOff32_delete(instance: *mut c_void); }
			unsafe { cv_VectorOff32_delete(self.as_raw_VectorOff32()) };
		}
	}
	
	impl IntoIterator for VectorOff32 {
		type Item = f32;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOff32 {
		type Item = f32;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOff32>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOff32 {
		type Storage = f32;
		type Arg = f32;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOff32_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOff32_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOff32_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOff32_len(self.as_raw_VectorOff32()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOff32_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOff32_is_empty(self.as_raw_VectorOff32()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOff32_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOff32_capacity(self.as_raw_VectorOff32()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOff32_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOff32_shrink_to_fit(self.as_raw_VectorOff32()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOff32_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOff32_reserve(self.as_raw_VectorOff32(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOff32_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOff32_remove(self.as_raw_VectorOff32(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOff32_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOff32_swap(self.as_raw_VectorOff32(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOff32_clear(instance: *mut c_void); }
			unsafe { cv_VectorOff32_clear(self.as_raw_VectorOff32()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOff32_push(instance: *mut c_void, val: f32); }
			unsafe { cv_VectorOff32_push(self.as_raw_VectorOff32(), val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOff32_insert(instance: *mut c_void, index: size_t, val: f32); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOff32_insert(self.as_raw_VectorOff32(), index, val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOff32_get(instance: *mut c_void, index: size_t) -> sys::Result<f32>; }
			unsafe { cv_VectorOff32_get(self.as_raw_VectorOff32(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOff32_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<f32>; }
			cv_VectorOff32_get_unchecked(self.as_raw_VectorOff32(), index)
				.into_result()
				.expect("Infallible function failed: VectorOff32::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOff32_set(instance: *mut c_void, index: size_t, val: f32) -> sys::Result_void; }
			unsafe { cv_VectorOff32_set(self.as_raw_VectorOff32(), index, val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOff32_set_unchecked(instance: *mut c_void, index: size_t, val: f32); }
			cv_VectorOff32_set_unchecked(self.as_raw_VectorOff32(), index, val)
		}
	
		
		#[inline]
		fn to_vec(&self) -> Vec<Self::Storage> {
			self.to_slice().to_vec()
		}
	}
	
	unsafe impl Send for VectorOff32 {}
	
	impl core::ToInputArray for VectorOff32 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOff32_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOff32_input_array(self.as_raw_VectorOff32()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOff32_output_array(self.as_raw_VectorOff32()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOff32_input_output_array(self.as_raw_VectorOff32()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOff32 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOff64 {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOff64 {
		pub fn as_raw_VectorOff64(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
		
		pub fn to_slice(&self) -> &[f64] {
			extern "C" { fn cv_VectorOff64_data(instance: *mut c_void) -> *const f64; }
			unsafe {
				let data = cv_VectorOff64_data(self.as_raw_VectorOff64());
				::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
			}
		}
	}
	
	impl Drop for VectorOff64 {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOff64_delete(instance: *mut c_void); }
			unsafe { cv_VectorOff64_delete(self.as_raw_VectorOff64()) };
		}
	}
	
	impl IntoIterator for VectorOff64 {
		type Item = f64;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOff64 {
		type Item = f64;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOff64>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOff64 {
		type Storage = f64;
		type Arg = f64;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOff64_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOff64_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOff64_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOff64_len(self.as_raw_VectorOff64()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOff64_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOff64_is_empty(self.as_raw_VectorOff64()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOff64_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOff64_capacity(self.as_raw_VectorOff64()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOff64_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOff64_shrink_to_fit(self.as_raw_VectorOff64()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOff64_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOff64_reserve(self.as_raw_VectorOff64(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOff64_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOff64_remove(self.as_raw_VectorOff64(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOff64_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOff64_swap(self.as_raw_VectorOff64(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOff64_clear(instance: *mut c_void); }
			unsafe { cv_VectorOff64_clear(self.as_raw_VectorOff64()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOff64_push(instance: *mut c_void, val: f64); }
			unsafe { cv_VectorOff64_push(self.as_raw_VectorOff64(), val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOff64_insert(instance: *mut c_void, index: size_t, val: f64); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOff64_insert(self.as_raw_VectorOff64(), index, val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOff64_get(instance: *mut c_void, index: size_t) -> sys::Result<f64>; }
			unsafe { cv_VectorOff64_get(self.as_raw_VectorOff64(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOff64_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<f64>; }
			cv_VectorOff64_get_unchecked(self.as_raw_VectorOff64(), index)
				.into_result()
				.expect("Infallible function failed: VectorOff64::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOff64_set(instance: *mut c_void, index: size_t, val: f64) -> sys::Result_void; }
			unsafe { cv_VectorOff64_set(self.as_raw_VectorOff64(), index, val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOff64_set_unchecked(instance: *mut c_void, index: size_t, val: f64); }
			cv_VectorOff64_set_unchecked(self.as_raw_VectorOff64(), index, val)
		}
	
		
		#[inline]
		fn to_vec(&self) -> Vec<Self::Storage> {
			self.to_slice().to_vec()
		}
	}
	
	unsafe impl Send for VectorOff64 {}
	
	impl core::ToInputArray for VectorOff64 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOff64_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOff64_input_array(self.as_raw_VectorOff64()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOff64_output_array(self.as_raw_VectorOff64()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOff64_input_output_array(self.as_raw_VectorOff64()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOff64 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfi32 {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfi32 {
		pub fn as_raw_VectorOfi32(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
		
		pub fn to_slice(&self) -> &[i32] {
			extern "C" { fn cv_VectorOfi32_data(instance: *mut c_void) -> *const i32; }
			unsafe {
				let data = cv_VectorOfi32_data(self.as_raw_VectorOfi32());
				::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
			}
		}
	}
	
	impl Drop for VectorOfi32 {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfi32_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfi32_delete(self.as_raw_VectorOfi32()) };
		}
	}
	
	impl IntoIterator for VectorOfi32 {
		type Item = i32;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfi32 {
		type Item = i32;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfi32>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfi32 {
		type Storage = i32;
		type Arg = i32;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfi32_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfi32_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfi32_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfi32_len(self.as_raw_VectorOfi32()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfi32_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfi32_is_empty(self.as_raw_VectorOfi32()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfi32_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfi32_capacity(self.as_raw_VectorOfi32()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfi32_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfi32_shrink_to_fit(self.as_raw_VectorOfi32()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfi32_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfi32_reserve(self.as_raw_VectorOfi32(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfi32_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfi32_remove(self.as_raw_VectorOfi32(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfi32_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfi32_swap(self.as_raw_VectorOfi32(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfi32_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfi32_clear(self.as_raw_VectorOfi32()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfi32_push(instance: *mut c_void, val: i32); }
			unsafe { cv_VectorOfi32_push(self.as_raw_VectorOfi32(), val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfi32_insert(instance: *mut c_void, index: size_t, val: i32); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfi32_insert(self.as_raw_VectorOfi32(), index, val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfi32_get(instance: *mut c_void, index: size_t) -> sys::Result<i32>; }
			unsafe { cv_VectorOfi32_get(self.as_raw_VectorOfi32(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfi32_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<i32>; }
			cv_VectorOfi32_get_unchecked(self.as_raw_VectorOfi32(), index)
				.into_result()
				.expect("Infallible function failed: VectorOfi32::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfi32_set(instance: *mut c_void, index: size_t, val: i32) -> sys::Result_void; }
			unsafe { cv_VectorOfi32_set(self.as_raw_VectorOfi32(), index, val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfi32_set_unchecked(instance: *mut c_void, index: size_t, val: i32); }
			cv_VectorOfi32_set_unchecked(self.as_raw_VectorOfi32(), index, val)
		}
	
		
		#[inline]
		fn to_vec(&self) -> Vec<Self::Storage> {
			self.to_slice().to_vec()
		}
	}
	
	unsafe impl Send for VectorOfi32 {}
	
	impl core::ToInputArray for VectorOfi32 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfi32_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfi32_input_array(self.as_raw_VectorOfi32()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfi32_output_array(self.as_raw_VectorOfi32()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfi32_input_output_array(self.as_raw_VectorOfi32()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfi32 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfi8 {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfi8 {
		pub fn as_raw_VectorOfi8(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
		
		pub fn to_slice(&self) -> &[i8] {
			extern "C" { fn cv_VectorOfi8_data(instance: *mut c_void) -> *const i8; }
			unsafe {
				let data = cv_VectorOfi8_data(self.as_raw_VectorOfi8());
				::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
			}
		}
	}
	
	impl Drop for VectorOfi8 {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfi8_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfi8_delete(self.as_raw_VectorOfi8()) };
		}
	}
	
	impl IntoIterator for VectorOfi8 {
		type Item = i8;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfi8 {
		type Item = i8;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfi8>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfi8 {
		type Storage = i8;
		type Arg = i8;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfi8_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfi8_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfi8_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfi8_len(self.as_raw_VectorOfi8()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfi8_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfi8_is_empty(self.as_raw_VectorOfi8()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfi8_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfi8_capacity(self.as_raw_VectorOfi8()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfi8_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfi8_shrink_to_fit(self.as_raw_VectorOfi8()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfi8_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfi8_reserve(self.as_raw_VectorOfi8(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfi8_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfi8_remove(self.as_raw_VectorOfi8(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfi8_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfi8_swap(self.as_raw_VectorOfi8(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfi8_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfi8_clear(self.as_raw_VectorOfi8()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfi8_push(instance: *mut c_void, val: i8); }
			unsafe { cv_VectorOfi8_push(self.as_raw_VectorOfi8(), val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfi8_insert(instance: *mut c_void, index: size_t, val: i8); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfi8_insert(self.as_raw_VectorOfi8(), index, val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfi8_get(instance: *mut c_void, index: size_t) -> sys::Result<i8>; }
			unsafe { cv_VectorOfi8_get(self.as_raw_VectorOfi8(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfi8_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<i8>; }
			cv_VectorOfi8_get_unchecked(self.as_raw_VectorOfi8(), index)
				.into_result()
				.expect("Infallible function failed: VectorOfi8::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfi8_set(instance: *mut c_void, index: size_t, val: i8) -> sys::Result_void; }
			unsafe { cv_VectorOfi8_set(self.as_raw_VectorOfi8(), index, val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfi8_set_unchecked(instance: *mut c_void, index: size_t, val: i8); }
			cv_VectorOfi8_set_unchecked(self.as_raw_VectorOfi8(), index, val)
		}
	
		
		#[inline]
		fn to_vec(&self) -> Vec<Self::Storage> {
			self.to_slice().to_vec()
		}
	}
	
	unsafe impl Send for VectorOfi8 {}
	
	impl core::ToInputArray for VectorOfi8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfi8_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfi8_input_array(self.as_raw_VectorOfi8()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfi8_output_array(self.as_raw_VectorOfi8()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfi8_input_output_array(self.as_raw_VectorOfi8()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
		}
	}
	
	impl core::ToInputOutputArray for &mut VectorOfi8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			(*self).input_output_array()
		}
	}
	
	pub struct VectorOfsize_t {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfsize_t {
		pub fn as_raw_VectorOfsize_t(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
		
		pub fn to_slice(&self) -> &[size_t] {
			extern "C" { fn cv_VectorOfsize_t_data(instance: *mut c_void) -> *const size_t; }
			unsafe {
				let data = cv_VectorOfsize_t_data(self.as_raw_VectorOfsize_t());
				::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
			}
		}
	}
	
	impl Drop for VectorOfsize_t {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfsize_t_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfsize_t_delete(self.as_raw_VectorOfsize_t()) };
		}
	}
	
	impl IntoIterator for VectorOfsize_t {
		type Item = size_t;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfsize_t {
		type Item = size_t;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfsize_t>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfsize_t {
		type Storage = size_t;
		type Arg = size_t;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfsize_t_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfsize_t_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfsize_t_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfsize_t_len(self.as_raw_VectorOfsize_t()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfsize_t_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfsize_t_is_empty(self.as_raw_VectorOfsize_t()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfsize_t_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfsize_t_capacity(self.as_raw_VectorOfsize_t()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfsize_t_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfsize_t_shrink_to_fit(self.as_raw_VectorOfsize_t()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfsize_t_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfsize_t_reserve(self.as_raw_VectorOfsize_t(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfsize_t_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfsize_t_remove(self.as_raw_VectorOfsize_t(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfsize_t_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfsize_t_swap(self.as_raw_VectorOfsize_t(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfsize_t_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfsize_t_clear(self.as_raw_VectorOfsize_t()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfsize_t_push(instance: *mut c_void, val: size_t); }
			unsafe { cv_VectorOfsize_t_push(self.as_raw_VectorOfsize_t(), val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfsize_t_insert(instance: *mut c_void, index: size_t, val: size_t); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfsize_t_insert(self.as_raw_VectorOfsize_t(), index, val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfsize_t_get(instance: *mut c_void, index: size_t) -> sys::Result<size_t>; }
			unsafe { cv_VectorOfsize_t_get(self.as_raw_VectorOfsize_t(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfsize_t_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<size_t>; }
			cv_VectorOfsize_t_get_unchecked(self.as_raw_VectorOfsize_t(), index)
				.into_result()
				.expect("Infallible function failed: VectorOfsize_t::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfsize_t_set(instance: *mut c_void, index: size_t, val: size_t) -> sys::Result_void; }
			unsafe { cv_VectorOfsize_t_set(self.as_raw_VectorOfsize_t(), index, val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfsize_t_set_unchecked(instance: *mut c_void, index: size_t, val: size_t); }
			cv_VectorOfsize_t_set_unchecked(self.as_raw_VectorOfsize_t(), index, val)
		}
	
		
		#[inline]
		fn to_vec(&self) -> Vec<Self::Storage> {
			self.to_slice().to_vec()
		}
	}
	
	unsafe impl Send for VectorOfsize_t {}
	
	pub struct VectorOfu8 {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfu8 {
		pub fn as_raw_VectorOfu8(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
		
		pub fn to_slice(&self) -> &[u8] {
			extern "C" { fn cv_VectorOfu8_data(instance: *mut c_void) -> *const u8; }
			unsafe {
				let data = cv_VectorOfu8_data(self.as_raw_VectorOfu8());
				::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
			}
		}
	}
	
	impl Drop for VectorOfu8 {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfu8_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfu8_delete(self.as_raw_VectorOfu8()) };
		}
	}
	
	impl IntoIterator for VectorOfu8 {
		type Item = u8;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfu8 {
		type Item = u8;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfu8>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfu8 {
		type Storage = u8;
		type Arg = u8;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfu8_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfu8_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfu8_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfu8_len(self.as_raw_VectorOfu8()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfu8_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfu8_is_empty(self.as_raw_VectorOfu8()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfu8_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfu8_capacity(self.as_raw_VectorOfu8()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfu8_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfu8_shrink_to_fit(self.as_raw_VectorOfu8()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfu8_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfu8_reserve(self.as_raw_VectorOfu8(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfu8_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfu8_remove(self.as_raw_VectorOfu8(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfu8_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfu8_swap(self.as_raw_VectorOfu8(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfu8_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfu8_clear(self.as_raw_VectorOfu8()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfu8_push(instance: *mut c_void, val: u8); }
			unsafe { cv_VectorOfu8_push(self.as_raw_VectorOfu8(), val) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfu8_insert(instance: *mut c_void, index: size_t, val: u8); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfu8_insert(self.as_raw_VectorOfu8(), index, val) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfu8_get(instance: *mut c_void, index: size_t) -> sys::Result<u8>; }
			unsafe { cv_VectorOfu8_get(self.as_raw_VectorOfu8(), index) }
				.into_result()
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfu8_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<u8>; }
			cv_VectorOfu8_get_unchecked(self.as_raw_VectorOfu8(), index)
				.into_result()
				.expect("Infallible function failed: VectorOfu8::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfu8_set(instance: *mut c_void, index: size_t, val: u8) -> sys::Result_void; }
			unsafe { cv_VectorOfu8_set(self.as_raw_VectorOfu8(), index, val) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfu8_set_unchecked(instance: *mut c_void, index: size_t, val: u8); }
			cv_VectorOfu8_set_unchecked(self.as_raw_VectorOfu8(), index, val)
		}
	
		
		#[inline]
		fn to_vec(&self) -> Vec<Self::Storage> {
			self.to_slice().to_vec()
		}
	}
	
	unsafe impl Send for VectorOfu8 {}
	
	impl core::ToInputArray for VectorOfu8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfu8_input_array(instance: *mut c_void) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfu8_input_array(self.as_raw_VectorOfu8()) }
				.into_result()
				.map(|ptr| core::_InputArray { ptr })
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
			unsafe { cv_VectorOfu8_output_array(self.as_raw_VectorOfu8()) }
				.into_result()
				.map(|ptr| core::_OutputArray { ptr })
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
			unsafe { cv_VectorOfu8_input_output_array(self.as_raw_VectorOfu8()) }
				.into_result()
				.map(|ptr| core::_InputOutputArray { ptr })
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

	pub struct PtrOfAbsLayer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfAbsLayer {
		pub fn as_raw_PtrOfAbsLayer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfAbsLayer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfAbsLayer_get_inner_ptr(self.as_raw_PtrOfAbsLayer()) }
		}
	}
	
	impl Drop for PtrOfAbsLayer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfAbsLayer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfAbsLayer_delete(self.as_raw_PtrOfAbsLayer()) };
		}
	}
	
	unsafe impl Send for PtrOfAbsLayer {}
	
	impl crate::dnn::AbsLayer for PtrOfAbsLayer {
		fn as_raw_AbsLayer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::dnn::Layer for PtrOfAbsLayer {
		fn as_raw_Layer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfBNLLLayer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfBNLLLayer {
		pub fn as_raw_PtrOfBNLLLayer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfBNLLLayer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfBNLLLayer_get_inner_ptr(self.as_raw_PtrOfBNLLLayer()) }
		}
	}
	
	impl Drop for PtrOfBNLLLayer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfBNLLLayer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfBNLLLayer_delete(self.as_raw_PtrOfBNLLLayer()) };
		}
	}
	
	unsafe impl Send for PtrOfBNLLLayer {}
	
	impl crate::dnn::BNLLLayer for PtrOfBNLLLayer {
		fn as_raw_BNLLLayer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::dnn::Layer for PtrOfBNLLLayer {
		fn as_raw_Layer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfBaseConvolutionLayer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfBaseConvolutionLayer {
		pub fn as_raw_PtrOfBaseConvolutionLayer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfBaseConvolutionLayer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfBaseConvolutionLayer_get_inner_ptr(self.as_raw_PtrOfBaseConvolutionLayer()) }
		}
	}
	
	impl Drop for PtrOfBaseConvolutionLayer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfBaseConvolutionLayer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfBaseConvolutionLayer_delete(self.as_raw_PtrOfBaseConvolutionLayer()) };
		}
	}
	
	unsafe impl Send for PtrOfBaseConvolutionLayer {}
	
	impl crate::dnn::BaseConvolutionLayer for PtrOfBaseConvolutionLayer {
		fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::dnn::Layer for PtrOfBaseConvolutionLayer {
		fn as_raw_Layer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfConcatLayer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfConcatLayer {
		pub fn as_raw_PtrOfConcatLayer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfConcatLayer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfConcatLayer_get_inner_ptr(self.as_raw_PtrOfConcatLayer()) }
		}
	}
	
	impl Drop for PtrOfConcatLayer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfConcatLayer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfConcatLayer_delete(self.as_raw_PtrOfConcatLayer()) };
		}
	}
	
	unsafe impl Send for PtrOfConcatLayer {}
	
	impl crate::dnn::ConcatLayer for PtrOfConcatLayer {
		fn as_raw_ConcatLayer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::dnn::Layer for PtrOfConcatLayer {
		fn as_raw_Layer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfCropLayer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfCropLayer {
		pub fn as_raw_PtrOfCropLayer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfCropLayer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfCropLayer_get_inner_ptr(self.as_raw_PtrOfCropLayer()) }
		}
	}
	
	impl Drop for PtrOfCropLayer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfCropLayer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfCropLayer_delete(self.as_raw_PtrOfCropLayer()) };
		}
	}
	
	unsafe impl Send for PtrOfCropLayer {}
	
	impl crate::dnn::CropLayer for PtrOfCropLayer {
		fn as_raw_CropLayer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::dnn::Layer for PtrOfCropLayer {
		fn as_raw_Layer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfEltwiseLayer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfEltwiseLayer {
		pub fn as_raw_PtrOfEltwiseLayer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfEltwiseLayer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfEltwiseLayer_get_inner_ptr(self.as_raw_PtrOfEltwiseLayer()) }
		}
	}
	
	impl Drop for PtrOfEltwiseLayer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfEltwiseLayer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfEltwiseLayer_delete(self.as_raw_PtrOfEltwiseLayer()) };
		}
	}
	
	unsafe impl Send for PtrOfEltwiseLayer {}
	
	impl crate::dnn::EltwiseLayer for PtrOfEltwiseLayer {
		fn as_raw_EltwiseLayer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::dnn::Layer for PtrOfEltwiseLayer {
		fn as_raw_Layer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfImporter {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfImporter {
		pub fn as_raw_PtrOfImporter(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfImporter_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfImporter_get_inner_ptr(self.as_raw_PtrOfImporter()) }
		}
	}
	
	impl Drop for PtrOfImporter {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfImporter_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfImporter_delete(self.as_raw_PtrOfImporter()) };
		}
	}
	
	unsafe impl Send for PtrOfImporter {}
	
	impl crate::dnn::Importer for PtrOfImporter {
		fn as_raw_Importer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfInnerProductLayer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfInnerProductLayer {
		pub fn as_raw_PtrOfInnerProductLayer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfInnerProductLayer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfInnerProductLayer_get_inner_ptr(self.as_raw_PtrOfInnerProductLayer()) }
		}
	}
	
	impl Drop for PtrOfInnerProductLayer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfInnerProductLayer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfInnerProductLayer_delete(self.as_raw_PtrOfInnerProductLayer()) };
		}
	}
	
	unsafe impl Send for PtrOfInnerProductLayer {}
	
	impl crate::dnn::InnerProductLayer for PtrOfInnerProductLayer {
		fn as_raw_InnerProductLayer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::dnn::Layer for PtrOfInnerProductLayer {
		fn as_raw_Layer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfLRNLayer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfLRNLayer {
		pub fn as_raw_PtrOfLRNLayer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfLRNLayer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfLRNLayer_get_inner_ptr(self.as_raw_PtrOfLRNLayer()) }
		}
	}
	
	impl Drop for PtrOfLRNLayer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfLRNLayer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfLRNLayer_delete(self.as_raw_PtrOfLRNLayer()) };
		}
	}
	
	unsafe impl Send for PtrOfLRNLayer {}
	
	impl crate::dnn::LRNLayer for PtrOfLRNLayer {
		fn as_raw_LRNLayer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::dnn::Layer for PtrOfLRNLayer {
		fn as_raw_Layer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfLSTMLayer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfLSTMLayer {
		pub fn as_raw_PtrOfLSTMLayer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfLSTMLayer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfLSTMLayer_get_inner_ptr(self.as_raw_PtrOfLSTMLayer()) }
		}
	}
	
	impl Drop for PtrOfLSTMLayer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfLSTMLayer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfLSTMLayer_delete(self.as_raw_PtrOfLSTMLayer()) };
		}
	}
	
	unsafe impl Send for PtrOfLSTMLayer {}
	
	impl crate::dnn::LSTMLayer for PtrOfLSTMLayer {
		fn as_raw_LSTMLayer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::dnn::Layer for PtrOfLSTMLayer {
		fn as_raw_Layer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfLayer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfLayer {
		pub fn as_raw_PtrOfLayer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfLayer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfLayer_get_inner_ptr(self.as_raw_PtrOfLayer()) }
		}
	}
	
	impl Drop for PtrOfLayer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfLayer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfLayer_delete(self.as_raw_PtrOfLayer()) };
		}
	}
	
	unsafe impl Send for PtrOfLayer {}
	
	impl crate::dnn::Layer for PtrOfLayer {
		fn as_raw_Layer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfMVNLayer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfMVNLayer {
		pub fn as_raw_PtrOfMVNLayer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfMVNLayer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfMVNLayer_get_inner_ptr(self.as_raw_PtrOfMVNLayer()) }
		}
	}
	
	impl Drop for PtrOfMVNLayer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfMVNLayer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfMVNLayer_delete(self.as_raw_PtrOfMVNLayer()) };
		}
	}
	
	unsafe impl Send for PtrOfMVNLayer {}
	
	impl crate::dnn::Layer for PtrOfMVNLayer {
		fn as_raw_Layer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::dnn::MVNLayer for PtrOfMVNLayer {
		fn as_raw_MVNLayer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfPoolingLayer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfPoolingLayer {
		pub fn as_raw_PtrOfPoolingLayer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfPoolingLayer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfPoolingLayer_get_inner_ptr(self.as_raw_PtrOfPoolingLayer()) }
		}
	}
	
	impl Drop for PtrOfPoolingLayer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfPoolingLayer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfPoolingLayer_delete(self.as_raw_PtrOfPoolingLayer()) };
		}
	}
	
	unsafe impl Send for PtrOfPoolingLayer {}
	
	impl crate::dnn::Layer for PtrOfPoolingLayer {
		fn as_raw_Layer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::dnn::PoolingLayer for PtrOfPoolingLayer {
		fn as_raw_PoolingLayer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfPowerLayer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfPowerLayer {
		pub fn as_raw_PtrOfPowerLayer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfPowerLayer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfPowerLayer_get_inner_ptr(self.as_raw_PtrOfPowerLayer()) }
		}
	}
	
	impl Drop for PtrOfPowerLayer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfPowerLayer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfPowerLayer_delete(self.as_raw_PtrOfPowerLayer()) };
		}
	}
	
	unsafe impl Send for PtrOfPowerLayer {}
	
	impl crate::dnn::Layer for PtrOfPowerLayer {
		fn as_raw_Layer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::dnn::PowerLayer for PtrOfPowerLayer {
		fn as_raw_PowerLayer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfRNNLayer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfRNNLayer {
		pub fn as_raw_PtrOfRNNLayer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfRNNLayer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfRNNLayer_get_inner_ptr(self.as_raw_PtrOfRNNLayer()) }
		}
	}
	
	impl Drop for PtrOfRNNLayer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfRNNLayer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfRNNLayer_delete(self.as_raw_PtrOfRNNLayer()) };
		}
	}
	
	unsafe impl Send for PtrOfRNNLayer {}
	
	impl crate::dnn::Layer for PtrOfRNNLayer {
		fn as_raw_Layer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::dnn::RNNLayer for PtrOfRNNLayer {
		fn as_raw_RNNLayer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfReLULayer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfReLULayer {
		pub fn as_raw_PtrOfReLULayer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfReLULayer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfReLULayer_get_inner_ptr(self.as_raw_PtrOfReLULayer()) }
		}
	}
	
	impl Drop for PtrOfReLULayer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfReLULayer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfReLULayer_delete(self.as_raw_PtrOfReLULayer()) };
		}
	}
	
	unsafe impl Send for PtrOfReLULayer {}
	
	impl crate::dnn::Layer for PtrOfReLULayer {
		fn as_raw_Layer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::dnn::ReLULayer for PtrOfReLULayer {
		fn as_raw_ReLULayer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfReshapeLayer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfReshapeLayer {
		pub fn as_raw_PtrOfReshapeLayer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfReshapeLayer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfReshapeLayer_get_inner_ptr(self.as_raw_PtrOfReshapeLayer()) }
		}
	}
	
	impl Drop for PtrOfReshapeLayer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfReshapeLayer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfReshapeLayer_delete(self.as_raw_PtrOfReshapeLayer()) };
		}
	}
	
	unsafe impl Send for PtrOfReshapeLayer {}
	
	impl crate::dnn::Layer for PtrOfReshapeLayer {
		fn as_raw_Layer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::dnn::ReshapeLayer for PtrOfReshapeLayer {
		fn as_raw_ReshapeLayer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfSigmoidLayer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfSigmoidLayer {
		pub fn as_raw_PtrOfSigmoidLayer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfSigmoidLayer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfSigmoidLayer_get_inner_ptr(self.as_raw_PtrOfSigmoidLayer()) }
		}
	}
	
	impl Drop for PtrOfSigmoidLayer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfSigmoidLayer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfSigmoidLayer_delete(self.as_raw_PtrOfSigmoidLayer()) };
		}
	}
	
	unsafe impl Send for PtrOfSigmoidLayer {}
	
	impl crate::dnn::Layer for PtrOfSigmoidLayer {
		fn as_raw_Layer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::dnn::SigmoidLayer for PtrOfSigmoidLayer {
		fn as_raw_SigmoidLayer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfSliceLayer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfSliceLayer {
		pub fn as_raw_PtrOfSliceLayer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfSliceLayer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfSliceLayer_get_inner_ptr(self.as_raw_PtrOfSliceLayer()) }
		}
	}
	
	impl Drop for PtrOfSliceLayer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfSliceLayer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfSliceLayer_delete(self.as_raw_PtrOfSliceLayer()) };
		}
	}
	
	unsafe impl Send for PtrOfSliceLayer {}
	
	impl crate::dnn::Layer for PtrOfSliceLayer {
		fn as_raw_Layer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::dnn::SliceLayer for PtrOfSliceLayer {
		fn as_raw_SliceLayer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfSoftmaxLayer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfSoftmaxLayer {
		pub fn as_raw_PtrOfSoftmaxLayer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfSoftmaxLayer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfSoftmaxLayer_get_inner_ptr(self.as_raw_PtrOfSoftmaxLayer()) }
		}
	}
	
	impl Drop for PtrOfSoftmaxLayer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfSoftmaxLayer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfSoftmaxLayer_delete(self.as_raw_PtrOfSoftmaxLayer()) };
		}
	}
	
	unsafe impl Send for PtrOfSoftmaxLayer {}
	
	impl crate::dnn::Layer for PtrOfSoftmaxLayer {
		fn as_raw_Layer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::dnn::SoftmaxLayer for PtrOfSoftmaxLayer {
		fn as_raw_SoftmaxLayer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfSplitLayer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfSplitLayer {
		pub fn as_raw_PtrOfSplitLayer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfSplitLayer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfSplitLayer_get_inner_ptr(self.as_raw_PtrOfSplitLayer()) }
		}
	}
	
	impl Drop for PtrOfSplitLayer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfSplitLayer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfSplitLayer_delete(self.as_raw_PtrOfSplitLayer()) };
		}
	}
	
	unsafe impl Send for PtrOfSplitLayer {}
	
	impl crate::dnn::Layer for PtrOfSplitLayer {
		fn as_raw_Layer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::dnn::SplitLayer for PtrOfSplitLayer {
		fn as_raw_SplitLayer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfTanHLayer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfTanHLayer {
		pub fn as_raw_PtrOfTanHLayer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfTanHLayer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfTanHLayer_get_inner_ptr(self.as_raw_PtrOfTanHLayer()) }
		}
	}
	
	impl Drop for PtrOfTanHLayer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfTanHLayer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfTanHLayer_delete(self.as_raw_PtrOfTanHLayer()) };
		}
	}
	
	unsafe impl Send for PtrOfTanHLayer {}
	
	impl crate::dnn::Layer for PtrOfTanHLayer {
		fn as_raw_Layer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::dnn::TanHLayer for PtrOfTanHLayer {
		fn as_raw_TanHLayer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct VectorOfBlob {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfBlob {
		pub fn as_raw_VectorOfBlob(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfBlob {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfBlob_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfBlob_delete(self.as_raw_VectorOfBlob()) };
		}
	}
	
	impl IntoIterator for VectorOfBlob {
		type Item = crate::dnn::Blob;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfBlob {
		type Item = crate::dnn::Blob;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfBlob>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfBlob {
		type Storage = crate::dnn::Blob;
		type Arg = crate::dnn::Blob;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfBlob_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfBlob_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfBlob_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfBlob_len(self.as_raw_VectorOfBlob()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfBlob_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfBlob_is_empty(self.as_raw_VectorOfBlob()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfBlob_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfBlob_capacity(self.as_raw_VectorOfBlob()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfBlob_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfBlob_shrink_to_fit(self.as_raw_VectorOfBlob()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfBlob_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfBlob_reserve(self.as_raw_VectorOfBlob(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfBlob_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfBlob_remove(self.as_raw_VectorOfBlob(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfBlob_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfBlob_swap(self.as_raw_VectorOfBlob(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfBlob_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfBlob_clear(self.as_raw_VectorOfBlob()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfBlob_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfBlob_push(self.as_raw_VectorOfBlob(), val.as_raw_Blob()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfBlob_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfBlob_insert(self.as_raw_VectorOfBlob(), index, val.as_raw_Blob()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfBlob_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfBlob_get(self.as_raw_VectorOfBlob(), index) }
				.into_result()
				.map(|ptr| crate::dnn::Blob { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfBlob_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfBlob_get_unchecked(self.as_raw_VectorOfBlob(), index)
				.into_result()
				.map(|ptr| crate::dnn::Blob { ptr })
				.expect("Infallible function failed: VectorOfBlob::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfBlob_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfBlob_set(self.as_raw_VectorOfBlob(), index, val.as_raw_Blob()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfBlob_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfBlob_set_unchecked(self.as_raw_VectorOfBlob(), index, val.as_raw_Blob())
		}
	
	}
	
	unsafe impl Send for VectorOfBlob {}
	
	pub struct VectorOfNet_LayerId {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfNet_LayerId {
		pub fn as_raw_VectorOfNet_LayerId(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfNet_LayerId {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfNet_LayerId_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfNet_LayerId_delete(self.as_raw_VectorOfNet_LayerId()) };
		}
	}
	
	impl IntoIterator for VectorOfNet_LayerId {
		type Item = crate::dnn::Net_LayerId;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfNet_LayerId {
		type Item = crate::dnn::Net_LayerId;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfNet_LayerId>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfNet_LayerId {
		type Storage = crate::dnn::Net_LayerId;
		type Arg = crate::dnn::Net_LayerId;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfNet_LayerId_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfNet_LayerId_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfNet_LayerId_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfNet_LayerId_len(self.as_raw_VectorOfNet_LayerId()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfNet_LayerId_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfNet_LayerId_is_empty(self.as_raw_VectorOfNet_LayerId()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfNet_LayerId_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfNet_LayerId_capacity(self.as_raw_VectorOfNet_LayerId()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfNet_LayerId_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfNet_LayerId_shrink_to_fit(self.as_raw_VectorOfNet_LayerId()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfNet_LayerId_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfNet_LayerId_reserve(self.as_raw_VectorOfNet_LayerId(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfNet_LayerId_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfNet_LayerId_remove(self.as_raw_VectorOfNet_LayerId(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfNet_LayerId_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfNet_LayerId_swap(self.as_raw_VectorOfNet_LayerId(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfNet_LayerId_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfNet_LayerId_clear(self.as_raw_VectorOfNet_LayerId()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfNet_LayerId_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfNet_LayerId_push(self.as_raw_VectorOfNet_LayerId(), val.as_raw_DictValue()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfNet_LayerId_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfNet_LayerId_insert(self.as_raw_VectorOfNet_LayerId(), index, val.as_raw_DictValue()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfNet_LayerId_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfNet_LayerId_get(self.as_raw_VectorOfNet_LayerId(), index) }
				.into_result()
				.map(|ptr| crate::dnn::DictValue { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfNet_LayerId_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfNet_LayerId_get_unchecked(self.as_raw_VectorOfNet_LayerId(), index)
				.into_result()
				.map(|ptr| crate::dnn::DictValue { ptr })
				.expect("Infallible function failed: VectorOfNet_LayerId::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfNet_LayerId_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfNet_LayerId_set(self.as_raw_VectorOfNet_LayerId(), index, val.as_raw_DictValue()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfNet_LayerId_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfNet_LayerId_set_unchecked(self.as_raw_VectorOfNet_LayerId(), index, val.as_raw_DictValue())
		}
	
	}
	
	unsafe impl Send for VectorOfNet_LayerId {}
	
}
#[cfg(feature = "contrib")]
pub use dnn_types::*;

#[cfg(feature = "contrib")]
mod dpm_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfDPMDetector {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfDPMDetector {
		pub fn as_raw_PtrOfDPMDetector(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfDPMDetector_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfDPMDetector_get_inner_ptr(self.as_raw_PtrOfDPMDetector()) }
		}
	}
	
	impl Drop for PtrOfDPMDetector {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfDPMDetector_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfDPMDetector_delete(self.as_raw_PtrOfDPMDetector()) };
		}
	}
	
	unsafe impl Send for PtrOfDPMDetector {}
	
	impl crate::dpm::DPMDetector for PtrOfDPMDetector {
		fn as_raw_DPMDetector(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct VectorOfDPMDetector_ObjectDetection {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfDPMDetector_ObjectDetection {
		pub fn as_raw_VectorOfDPMDetector_ObjectDetection(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfDPMDetector_ObjectDetection {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfDPMDetector_ObjectDetection_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfDPMDetector_ObjectDetection_delete(self.as_raw_VectorOfDPMDetector_ObjectDetection()) };
		}
	}
	
	impl IntoIterator for VectorOfDPMDetector_ObjectDetection {
		type Item = crate::dpm::DPMDetector_ObjectDetection;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfDPMDetector_ObjectDetection {
		type Item = crate::dpm::DPMDetector_ObjectDetection;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfDPMDetector_ObjectDetection>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfDPMDetector_ObjectDetection {
		type Storage = crate::dpm::DPMDetector_ObjectDetection;
		type Arg = crate::dpm::DPMDetector_ObjectDetection;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfDPMDetector_ObjectDetection_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfDPMDetector_ObjectDetection_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfDPMDetector_ObjectDetection_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfDPMDetector_ObjectDetection_len(self.as_raw_VectorOfDPMDetector_ObjectDetection()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfDPMDetector_ObjectDetection_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfDPMDetector_ObjectDetection_is_empty(self.as_raw_VectorOfDPMDetector_ObjectDetection()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfDPMDetector_ObjectDetection_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfDPMDetector_ObjectDetection_capacity(self.as_raw_VectorOfDPMDetector_ObjectDetection()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfDPMDetector_ObjectDetection_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfDPMDetector_ObjectDetection_shrink_to_fit(self.as_raw_VectorOfDPMDetector_ObjectDetection()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfDPMDetector_ObjectDetection_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfDPMDetector_ObjectDetection_reserve(self.as_raw_VectorOfDPMDetector_ObjectDetection(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfDPMDetector_ObjectDetection_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfDPMDetector_ObjectDetection_remove(self.as_raw_VectorOfDPMDetector_ObjectDetection(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfDPMDetector_ObjectDetection_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfDPMDetector_ObjectDetection_swap(self.as_raw_VectorOfDPMDetector_ObjectDetection(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfDPMDetector_ObjectDetection_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfDPMDetector_ObjectDetection_clear(self.as_raw_VectorOfDPMDetector_ObjectDetection()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfDPMDetector_ObjectDetection_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfDPMDetector_ObjectDetection_push(self.as_raw_VectorOfDPMDetector_ObjectDetection(), val.as_raw_DPMDetector_ObjectDetection()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfDPMDetector_ObjectDetection_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfDPMDetector_ObjectDetection_insert(self.as_raw_VectorOfDPMDetector_ObjectDetection(), index, val.as_raw_DPMDetector_ObjectDetection()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfDPMDetector_ObjectDetection_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfDPMDetector_ObjectDetection_get(self.as_raw_VectorOfDPMDetector_ObjectDetection(), index) }
				.into_result()
				.map(|ptr| crate::dpm::DPMDetector_ObjectDetection { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfDPMDetector_ObjectDetection_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfDPMDetector_ObjectDetection_get_unchecked(self.as_raw_VectorOfDPMDetector_ObjectDetection(), index)
				.into_result()
				.map(|ptr| crate::dpm::DPMDetector_ObjectDetection { ptr })
				.expect("Infallible function failed: VectorOfDPMDetector_ObjectDetection::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfDPMDetector_ObjectDetection_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfDPMDetector_ObjectDetection_set(self.as_raw_VectorOfDPMDetector_ObjectDetection(), index, val.as_raw_DPMDetector_ObjectDetection()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfDPMDetector_ObjectDetection_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfDPMDetector_ObjectDetection_set_unchecked(self.as_raw_VectorOfDPMDetector_ObjectDetection(), index, val.as_raw_DPMDetector_ObjectDetection())
		}
	
	}
	
	unsafe impl Send for VectorOfDPMDetector_ObjectDetection {}
	
}
#[cfg(feature = "contrib")]
pub use dpm_types::*;

#[cfg(feature = "contrib")]
mod face_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfBIF {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfBIF {
		pub fn as_raw_PtrOfBIF(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfBIF_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfBIF_get_inner_ptr(self.as_raw_PtrOfBIF()) }
		}
	}
	
	impl Drop for PtrOfBIF {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfBIF_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfBIF_delete(self.as_raw_PtrOfBIF()) };
		}
	}
	
	unsafe impl Send for PtrOfBIF {}
	
	impl core::AlgorithmTrait for PtrOfBIF {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::face::BIF for PtrOfBIF {
		fn as_raw_BIF(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfBasicFaceRecognizer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfBasicFaceRecognizer {
		pub fn as_raw_PtrOfBasicFaceRecognizer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfBasicFaceRecognizer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfBasicFaceRecognizer_get_inner_ptr(self.as_raw_PtrOfBasicFaceRecognizer()) }
		}
	}
	
	impl Drop for PtrOfBasicFaceRecognizer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfBasicFaceRecognizer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfBasicFaceRecognizer_delete(self.as_raw_PtrOfBasicFaceRecognizer()) };
		}
	}
	
	unsafe impl Send for PtrOfBasicFaceRecognizer {}
	
	impl core::AlgorithmTrait for PtrOfBasicFaceRecognizer {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::face::BasicFaceRecognizer for PtrOfBasicFaceRecognizer {
		fn as_raw_BasicFaceRecognizer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::face::FaceRecognizer for PtrOfBasicFaceRecognizer {
		fn as_raw_FaceRecognizer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfLBPHFaceRecognizer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfLBPHFaceRecognizer {
		pub fn as_raw_PtrOfLBPHFaceRecognizer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfLBPHFaceRecognizer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfLBPHFaceRecognizer_get_inner_ptr(self.as_raw_PtrOfLBPHFaceRecognizer()) }
		}
	}
	
	impl Drop for PtrOfLBPHFaceRecognizer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfLBPHFaceRecognizer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfLBPHFaceRecognizer_delete(self.as_raw_PtrOfLBPHFaceRecognizer()) };
		}
	}
	
	unsafe impl Send for PtrOfLBPHFaceRecognizer {}
	
	impl core::AlgorithmTrait for PtrOfLBPHFaceRecognizer {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::face::FaceRecognizer for PtrOfLBPHFaceRecognizer {
		fn as_raw_FaceRecognizer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::face::LBPHFaceRecognizer for PtrOfLBPHFaceRecognizer {
		fn as_raw_LBPHFaceRecognizer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfPredictCollector {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfPredictCollector {
		pub fn as_raw_PtrOfPredictCollector(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfPredictCollector_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfPredictCollector_get_inner_ptr(self.as_raw_PtrOfPredictCollector()) }
		}
	}
	
	impl Drop for PtrOfPredictCollector {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfPredictCollector_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfPredictCollector_delete(self.as_raw_PtrOfPredictCollector()) };
		}
	}
	
	unsafe impl Send for PtrOfPredictCollector {}
	
	impl crate::face::PredictCollector for PtrOfPredictCollector {
		fn as_raw_PredictCollector(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfStandardCollector {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfStandardCollector {
		pub fn as_raw_PtrOfStandardCollector(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfStandardCollector_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfStandardCollector_get_inner_ptr(self.as_raw_PtrOfStandardCollector()) }
		}
	}
	
	impl Drop for PtrOfStandardCollector {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfStandardCollector_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfStandardCollector_delete(self.as_raw_PtrOfStandardCollector()) };
		}
	}
	
	unsafe impl Send for PtrOfStandardCollector {}
	
	impl crate::face::PredictCollector for PtrOfStandardCollector {
		fn as_raw_PredictCollector(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::face::StandardCollectorTrait for PtrOfStandardCollector {
		fn as_raw_StandardCollector(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
}
#[cfg(feature = "contrib")]
pub use face_types::*;

mod features2d_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfAKAZE {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfAKAZE {
		pub fn as_raw_PtrOfAKAZE(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfAKAZE_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfAKAZE_get_inner_ptr(self.as_raw_PtrOfAKAZE()) }
		}
	}
	
	impl Drop for PtrOfAKAZE {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfAKAZE_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfAKAZE_delete(self.as_raw_PtrOfAKAZE()) };
		}
	}
	
	unsafe impl Send for PtrOfAKAZE {}
	
	impl crate::features2d::AKAZE for PtrOfAKAZE {
		fn as_raw_AKAZE(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl core::AlgorithmTrait for PtrOfAKAZE {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfAKAZE {
		fn as_raw_Feature2D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfAgastFeatureDetector {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfAgastFeatureDetector {
		pub fn as_raw_PtrOfAgastFeatureDetector(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfAgastFeatureDetector_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfAgastFeatureDetector_get_inner_ptr(self.as_raw_PtrOfAgastFeatureDetector()) }
		}
	}
	
	impl Drop for PtrOfAgastFeatureDetector {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfAgastFeatureDetector_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfAgastFeatureDetector_delete(self.as_raw_PtrOfAgastFeatureDetector()) };
		}
	}
	
	unsafe impl Send for PtrOfAgastFeatureDetector {}
	
	impl crate::features2d::AgastFeatureDetector for PtrOfAgastFeatureDetector {
		fn as_raw_AgastFeatureDetector(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl core::AlgorithmTrait for PtrOfAgastFeatureDetector {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfAgastFeatureDetector {
		fn as_raw_Feature2D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfBFMatcher {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfBFMatcher {
		pub fn as_raw_PtrOfBFMatcher(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfBFMatcher_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfBFMatcher_get_inner_ptr(self.as_raw_PtrOfBFMatcher()) }
		}
	}
	
	impl Drop for PtrOfBFMatcher {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfBFMatcher_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfBFMatcher_delete(self.as_raw_PtrOfBFMatcher()) };
		}
	}
	
	unsafe impl Send for PtrOfBFMatcher {}
	
	impl core::AlgorithmTrait for PtrOfBFMatcher {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::BFMatcherTrait for PtrOfBFMatcher {
		fn as_raw_BFMatcher(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::DescriptorMatcher for PtrOfBFMatcher {
		fn as_raw_DescriptorMatcher(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfBRISK {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfBRISK {
		pub fn as_raw_PtrOfBRISK(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfBRISK_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfBRISK_get_inner_ptr(self.as_raw_PtrOfBRISK()) }
		}
	}
	
	impl Drop for PtrOfBRISK {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfBRISK_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfBRISK_delete(self.as_raw_PtrOfBRISK()) };
		}
	}
	
	unsafe impl Send for PtrOfBRISK {}
	
	impl core::AlgorithmTrait for PtrOfBRISK {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::BRISKTrait for PtrOfBRISK {
		fn as_raw_BRISK(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfBRISK {
		fn as_raw_Feature2D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfDescriptorExtractor {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfDescriptorExtractor {
		pub fn as_raw_PtrOfDescriptorExtractor(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfDescriptorExtractor_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfDescriptorExtractor_get_inner_ptr(self.as_raw_PtrOfDescriptorExtractor()) }
		}
	}
	
	impl Drop for PtrOfDescriptorExtractor {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfDescriptorExtractor_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfDescriptorExtractor_delete(self.as_raw_PtrOfDescriptorExtractor()) };
		}
	}
	
	unsafe impl Send for PtrOfDescriptorExtractor {}
	
	impl core::AlgorithmTrait for PtrOfDescriptorExtractor {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfDescriptorExtractor {
		fn as_raw_Feature2D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfDescriptorMatcher {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfDescriptorMatcher {
		pub fn as_raw_PtrOfDescriptorMatcher(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfDescriptorMatcher_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfDescriptorMatcher_get_inner_ptr(self.as_raw_PtrOfDescriptorMatcher()) }
		}
	}
	
	impl Drop for PtrOfDescriptorMatcher {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfDescriptorMatcher_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfDescriptorMatcher_delete(self.as_raw_PtrOfDescriptorMatcher()) };
		}
	}
	
	unsafe impl Send for PtrOfDescriptorMatcher {}
	
	impl core::AlgorithmTrait for PtrOfDescriptorMatcher {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::DescriptorMatcher for PtrOfDescriptorMatcher {
		fn as_raw_DescriptorMatcher(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfFastFeatureDetector {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfFastFeatureDetector {
		pub fn as_raw_PtrOfFastFeatureDetector(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfFastFeatureDetector_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfFastFeatureDetector_get_inner_ptr(self.as_raw_PtrOfFastFeatureDetector()) }
		}
	}
	
	impl Drop for PtrOfFastFeatureDetector {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfFastFeatureDetector_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfFastFeatureDetector_delete(self.as_raw_PtrOfFastFeatureDetector()) };
		}
	}
	
	unsafe impl Send for PtrOfFastFeatureDetector {}
	
	impl core::AlgorithmTrait for PtrOfFastFeatureDetector {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::FastFeatureDetector for PtrOfFastFeatureDetector {
		fn as_raw_FastFeatureDetector(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfFastFeatureDetector {
		fn as_raw_Feature2D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfFeatureDetector {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfFeatureDetector {
		pub fn as_raw_PtrOfFeatureDetector(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfFeatureDetector_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfFeatureDetector_get_inner_ptr(self.as_raw_PtrOfFeatureDetector()) }
		}
	}
	
	impl Drop for PtrOfFeatureDetector {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfFeatureDetector_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfFeatureDetector_delete(self.as_raw_PtrOfFeatureDetector()) };
		}
	}
	
	unsafe impl Send for PtrOfFeatureDetector {}
	
	impl core::AlgorithmTrait for PtrOfFeatureDetector {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfFeatureDetector {
		fn as_raw_Feature2D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfFlannBasedMatcher {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfFlannBasedMatcher {
		pub fn as_raw_PtrOfFlannBasedMatcher(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfFlannBasedMatcher_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfFlannBasedMatcher_get_inner_ptr(self.as_raw_PtrOfFlannBasedMatcher()) }
		}
	}
	
	impl Drop for PtrOfFlannBasedMatcher {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfFlannBasedMatcher_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfFlannBasedMatcher_delete(self.as_raw_PtrOfFlannBasedMatcher()) };
		}
	}
	
	unsafe impl Send for PtrOfFlannBasedMatcher {}
	
	impl core::AlgorithmTrait for PtrOfFlannBasedMatcher {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::DescriptorMatcher for PtrOfFlannBasedMatcher {
		fn as_raw_DescriptorMatcher(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::FlannBasedMatcherTrait for PtrOfFlannBasedMatcher {
		fn as_raw_FlannBasedMatcher(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfGFTTDetector {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfGFTTDetector {
		pub fn as_raw_PtrOfGFTTDetector(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfGFTTDetector_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfGFTTDetector_get_inner_ptr(self.as_raw_PtrOfGFTTDetector()) }
		}
	}
	
	impl Drop for PtrOfGFTTDetector {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfGFTTDetector_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfGFTTDetector_delete(self.as_raw_PtrOfGFTTDetector()) };
		}
	}
	
	unsafe impl Send for PtrOfGFTTDetector {}
	
	impl core::AlgorithmTrait for PtrOfGFTTDetector {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfGFTTDetector {
		fn as_raw_Feature2D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::GFTTDetector for PtrOfGFTTDetector {
		fn as_raw_GFTTDetector(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfKAZE {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfKAZE {
		pub fn as_raw_PtrOfKAZE(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfKAZE_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfKAZE_get_inner_ptr(self.as_raw_PtrOfKAZE()) }
		}
	}
	
	impl Drop for PtrOfKAZE {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfKAZE_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfKAZE_delete(self.as_raw_PtrOfKAZE()) };
		}
	}
	
	unsafe impl Send for PtrOfKAZE {}
	
	impl core::AlgorithmTrait for PtrOfKAZE {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfKAZE {
		fn as_raw_Feature2D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::KAZE for PtrOfKAZE {
		fn as_raw_KAZE(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfMSER {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfMSER {
		pub fn as_raw_PtrOfMSER(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfMSER_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfMSER_get_inner_ptr(self.as_raw_PtrOfMSER()) }
		}
	}
	
	impl Drop for PtrOfMSER {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfMSER_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfMSER_delete(self.as_raw_PtrOfMSER()) };
		}
	}
	
	unsafe impl Send for PtrOfMSER {}
	
	impl core::AlgorithmTrait for PtrOfMSER {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfMSER {
		fn as_raw_Feature2D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::MSER for PtrOfMSER {
		fn as_raw_MSER(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfORB {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfORB {
		pub fn as_raw_PtrOfORB(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfORB_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfORB_get_inner_ptr(self.as_raw_PtrOfORB()) }
		}
	}
	
	impl Drop for PtrOfORB {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfORB_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfORB_delete(self.as_raw_PtrOfORB()) };
		}
	}
	
	unsafe impl Send for PtrOfORB {}
	
	impl core::AlgorithmTrait for PtrOfORB {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfORB {
		fn as_raw_Feature2D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::ORB for PtrOfORB {
		fn as_raw_ORB(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfSimpleBlobDetector {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfSimpleBlobDetector {
		pub fn as_raw_PtrOfSimpleBlobDetector(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfSimpleBlobDetector_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfSimpleBlobDetector_get_inner_ptr(self.as_raw_PtrOfSimpleBlobDetector()) }
		}
	}
	
	impl Drop for PtrOfSimpleBlobDetector {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfSimpleBlobDetector_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfSimpleBlobDetector_delete(self.as_raw_PtrOfSimpleBlobDetector()) };
		}
	}
	
	unsafe impl Send for PtrOfSimpleBlobDetector {}
	
	impl core::AlgorithmTrait for PtrOfSimpleBlobDetector {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfSimpleBlobDetector {
		fn as_raw_Feature2D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::SimpleBlobDetectorTrait for PtrOfSimpleBlobDetector {
		fn as_raw_SimpleBlobDetector(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
}
pub use features2d_types::*;

mod flann_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfIndexParams {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfIndexParams {
		pub fn as_raw_PtrOfIndexParams(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfIndexParams_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfIndexParams_get_inner_ptr(self.as_raw_PtrOfIndexParams()) }
		}
	}
	
	impl Drop for PtrOfIndexParams {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfIndexParams_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfIndexParams_delete(self.as_raw_PtrOfIndexParams()) };
		}
	}
	
	unsafe impl Send for PtrOfIndexParams {}
	
	impl crate::flann::IndexParamsTrait for PtrOfIndexParams {
		fn as_raw_IndexParams(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfSearchParams {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfSearchParams {
		pub fn as_raw_PtrOfSearchParams(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfSearchParams_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfSearchParams_get_inner_ptr(self.as_raw_PtrOfSearchParams()) }
		}
	}
	
	impl Drop for PtrOfSearchParams {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfSearchParams_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfSearchParams_delete(self.as_raw_PtrOfSearchParams()) };
		}
	}
	
	unsafe impl Send for PtrOfSearchParams {}
	
	impl crate::flann::IndexParamsTrait for PtrOfSearchParams {
		fn as_raw_IndexParams(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::flann::SearchParamsTrait for PtrOfSearchParams {
		fn as_raw_SearchParams(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
}
pub use flann_types::*;

#[cfg(feature = "contrib")]
mod freetype_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfFreeType2 {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfFreeType2 {
		pub fn as_raw_PtrOfFreeType2(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfFreeType2_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfFreeType2_get_inner_ptr(self.as_raw_PtrOfFreeType2()) }
		}
	}
	
	impl Drop for PtrOfFreeType2 {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfFreeType2_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfFreeType2_delete(self.as_raw_PtrOfFreeType2()) };
		}
	}
	
	unsafe impl Send for PtrOfFreeType2 {}
	
	impl core::AlgorithmTrait for PtrOfFreeType2 {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::freetype::FreeType2 for PtrOfFreeType2 {
		fn as_raw_FreeType2(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
}
#[cfg(feature = "contrib")]
pub use freetype_types::*;

#[cfg(feature = "contrib")]
mod hdf_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfHDF5 {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfHDF5 {
		pub fn as_raw_PtrOfHDF5(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfHDF5_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfHDF5_get_inner_ptr(self.as_raw_PtrOfHDF5()) }
		}
	}
	
	impl Drop for PtrOfHDF5 {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfHDF5_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfHDF5_delete(self.as_raw_PtrOfHDF5()) };
		}
	}
	
	unsafe impl Send for PtrOfHDF5 {}
	
	impl crate::hdf::HDF5 for PtrOfHDF5 {
		fn as_raw_HDF5(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
}
#[cfg(feature = "contrib")]
pub use hdf_types::*;

mod imgproc_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfCLAHE {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfCLAHE {
		pub fn as_raw_PtrOfCLAHE(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfCLAHE_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfCLAHE_get_inner_ptr(self.as_raw_PtrOfCLAHE()) }
		}
	}
	
	impl Drop for PtrOfCLAHE {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfCLAHE_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfCLAHE_delete(self.as_raw_PtrOfCLAHE()) };
		}
	}
	
	unsafe impl Send for PtrOfCLAHE {}
	
	impl core::AlgorithmTrait for PtrOfCLAHE {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::imgproc::CLAHE for PtrOfCLAHE {
		fn as_raw_CLAHE(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfGeneralizedHoughBallard {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfGeneralizedHoughBallard {
		pub fn as_raw_PtrOfGeneralizedHoughBallard(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfGeneralizedHoughBallard_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfGeneralizedHoughBallard_get_inner_ptr(self.as_raw_PtrOfGeneralizedHoughBallard()) }
		}
	}
	
	impl Drop for PtrOfGeneralizedHoughBallard {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfGeneralizedHoughBallard_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfGeneralizedHoughBallard_delete(self.as_raw_PtrOfGeneralizedHoughBallard()) };
		}
	}
	
	unsafe impl Send for PtrOfGeneralizedHoughBallard {}
	
	impl core::AlgorithmTrait for PtrOfGeneralizedHoughBallard {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::imgproc::GeneralizedHough for PtrOfGeneralizedHoughBallard {
		fn as_raw_GeneralizedHough(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::imgproc::GeneralizedHoughBallard for PtrOfGeneralizedHoughBallard {
		fn as_raw_GeneralizedHoughBallard(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfGeneralizedHoughGuil {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfGeneralizedHoughGuil {
		pub fn as_raw_PtrOfGeneralizedHoughGuil(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfGeneralizedHoughGuil_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfGeneralizedHoughGuil_get_inner_ptr(self.as_raw_PtrOfGeneralizedHoughGuil()) }
		}
	}
	
	impl Drop for PtrOfGeneralizedHoughGuil {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfGeneralizedHoughGuil_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfGeneralizedHoughGuil_delete(self.as_raw_PtrOfGeneralizedHoughGuil()) };
		}
	}
	
	unsafe impl Send for PtrOfGeneralizedHoughGuil {}
	
	impl core::AlgorithmTrait for PtrOfGeneralizedHoughGuil {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::imgproc::GeneralizedHough for PtrOfGeneralizedHoughGuil {
		fn as_raw_GeneralizedHough(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::imgproc::GeneralizedHoughGuil for PtrOfGeneralizedHoughGuil {
		fn as_raw_GeneralizedHoughGuil(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfLineSegmentDetector {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfLineSegmentDetector {
		pub fn as_raw_PtrOfLineSegmentDetector(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfLineSegmentDetector_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfLineSegmentDetector_get_inner_ptr(self.as_raw_PtrOfLineSegmentDetector()) }
		}
	}
	
	impl Drop for PtrOfLineSegmentDetector {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfLineSegmentDetector_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfLineSegmentDetector_delete(self.as_raw_PtrOfLineSegmentDetector()) };
		}
	}
	
	unsafe impl Send for PtrOfLineSegmentDetector {}
	
	impl core::AlgorithmTrait for PtrOfLineSegmentDetector {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::imgproc::LineSegmentDetector for PtrOfLineSegmentDetector {
		fn as_raw_LineSegmentDetector(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
}
pub use imgproc_types::*;

#[cfg(feature = "contrib")]
mod line_descriptor_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfBinaryDescriptor {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfBinaryDescriptor {
		pub fn as_raw_PtrOfBinaryDescriptor(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfBinaryDescriptor_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfBinaryDescriptor_get_inner_ptr(self.as_raw_PtrOfBinaryDescriptor()) }
		}
	}
	
	impl Drop for PtrOfBinaryDescriptor {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfBinaryDescriptor_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfBinaryDescriptor_delete(self.as_raw_PtrOfBinaryDescriptor()) };
		}
	}
	
	unsafe impl Send for PtrOfBinaryDescriptor {}
	
	impl core::AlgorithmTrait for PtrOfBinaryDescriptor {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::line_descriptor::BinaryDescriptorTrait for PtrOfBinaryDescriptor {
		fn as_raw_BinaryDescriptor(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfBinaryDescriptorMatcher {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfBinaryDescriptorMatcher {
		pub fn as_raw_PtrOfBinaryDescriptorMatcher(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfBinaryDescriptorMatcher_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfBinaryDescriptorMatcher_get_inner_ptr(self.as_raw_PtrOfBinaryDescriptorMatcher()) }
		}
	}
	
	impl Drop for PtrOfBinaryDescriptorMatcher {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfBinaryDescriptorMatcher_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfBinaryDescriptorMatcher_delete(self.as_raw_PtrOfBinaryDescriptorMatcher()) };
		}
	}
	
	unsafe impl Send for PtrOfBinaryDescriptorMatcher {}
	
	impl core::AlgorithmTrait for PtrOfBinaryDescriptorMatcher {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::line_descriptor::BinaryDescriptorMatcherTrait for PtrOfBinaryDescriptorMatcher {
		fn as_raw_BinaryDescriptorMatcher(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfLSDDetector {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfLSDDetector {
		pub fn as_raw_PtrOfLSDDetector(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfLSDDetector_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfLSDDetector_get_inner_ptr(self.as_raw_PtrOfLSDDetector()) }
		}
	}
	
	impl Drop for PtrOfLSDDetector {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfLSDDetector_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfLSDDetector_delete(self.as_raw_PtrOfLSDDetector()) };
		}
	}
	
	unsafe impl Send for PtrOfLSDDetector {}
	
	impl core::AlgorithmTrait for PtrOfLSDDetector {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::line_descriptor::LSDDetectorTrait for PtrOfLSDDetector {
		fn as_raw_LSDDetector(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct VectorOfKeyLine {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfKeyLine {
		pub fn as_raw_VectorOfKeyLine(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfKeyLine {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfKeyLine_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfKeyLine_delete(self.as_raw_VectorOfKeyLine()) };
		}
	}
	
	impl IntoIterator for VectorOfKeyLine {
		type Item = crate::line_descriptor::KeyLine;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfKeyLine {
		type Item = crate::line_descriptor::KeyLine;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfKeyLine>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfKeyLine {
		type Storage = crate::line_descriptor::KeyLine;
		type Arg = crate::line_descriptor::KeyLine;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfKeyLine_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfKeyLine_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfKeyLine_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfKeyLine_len(self.as_raw_VectorOfKeyLine()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfKeyLine_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfKeyLine_is_empty(self.as_raw_VectorOfKeyLine()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfKeyLine_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfKeyLine_capacity(self.as_raw_VectorOfKeyLine()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfKeyLine_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfKeyLine_shrink_to_fit(self.as_raw_VectorOfKeyLine()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfKeyLine_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfKeyLine_reserve(self.as_raw_VectorOfKeyLine(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfKeyLine_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfKeyLine_remove(self.as_raw_VectorOfKeyLine(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfKeyLine_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfKeyLine_swap(self.as_raw_VectorOfKeyLine(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfKeyLine_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfKeyLine_clear(self.as_raw_VectorOfKeyLine()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfKeyLine_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfKeyLine_push(self.as_raw_VectorOfKeyLine(), val.as_raw_KeyLine()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfKeyLine_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfKeyLine_insert(self.as_raw_VectorOfKeyLine(), index, val.as_raw_KeyLine()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfKeyLine_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfKeyLine_get(self.as_raw_VectorOfKeyLine(), index) }
				.into_result()
				.map(|ptr| crate::line_descriptor::KeyLine { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfKeyLine_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfKeyLine_get_unchecked(self.as_raw_VectorOfKeyLine(), index)
				.into_result()
				.map(|ptr| crate::line_descriptor::KeyLine { ptr })
				.expect("Infallible function failed: VectorOfKeyLine::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfKeyLine_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfKeyLine_set(self.as_raw_VectorOfKeyLine(), index, val.as_raw_KeyLine()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfKeyLine_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfKeyLine_set_unchecked(self.as_raw_VectorOfKeyLine(), index, val.as_raw_KeyLine())
		}
	
	}
	
	unsafe impl Send for VectorOfKeyLine {}
	
	pub struct VectorOfVectorOfKeyLine {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfVectorOfKeyLine {
		pub fn as_raw_VectorOfVectorOfKeyLine(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfVectorOfKeyLine {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfKeyLine_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfKeyLine_delete(self.as_raw_VectorOfVectorOfKeyLine()) };
		}
	}
	
	impl IntoIterator for VectorOfVectorOfKeyLine {
		type Item = types::VectorOfKeyLine;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfVectorOfKeyLine {
		type Item = types::VectorOfKeyLine;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfKeyLine>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfKeyLine {
		type Storage = types::VectorOfKeyLine;
		type Arg = types::VectorOfKeyLine;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfVectorOfKeyLine_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfVectorOfKeyLine_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfKeyLine_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfKeyLine_len(self.as_raw_VectorOfVectorOfKeyLine()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfVectorOfKeyLine_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfVectorOfKeyLine_is_empty(self.as_raw_VectorOfVectorOfKeyLine()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfKeyLine_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfKeyLine_capacity(self.as_raw_VectorOfVectorOfKeyLine()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfKeyLine_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfKeyLine_shrink_to_fit(self.as_raw_VectorOfVectorOfKeyLine()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfVectorOfKeyLine_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfVectorOfKeyLine_reserve(self.as_raw_VectorOfVectorOfKeyLine(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfVectorOfKeyLine_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfVectorOfKeyLine_remove(self.as_raw_VectorOfVectorOfKeyLine(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfVectorOfKeyLine_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfVectorOfKeyLine_swap(self.as_raw_VectorOfVectorOfKeyLine(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfKeyLine_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfKeyLine_clear(self.as_raw_VectorOfVectorOfKeyLine()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfKeyLine_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfKeyLine_push(self.as_raw_VectorOfVectorOfKeyLine(), val.as_raw_VectorOfKeyLine()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfKeyLine_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfKeyLine_insert(self.as_raw_VectorOfVectorOfKeyLine(), index, val.as_raw_VectorOfKeyLine()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfVectorOfKeyLine_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfKeyLine_get(self.as_raw_VectorOfVectorOfKeyLine(), index) }
				.into_result()
				.map(|ptr| types::VectorOfKeyLine { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfVectorOfKeyLine_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfVectorOfKeyLine_get_unchecked(self.as_raw_VectorOfVectorOfKeyLine(), index)
				.into_result()
				.map(|ptr| types::VectorOfKeyLine { ptr })
				.expect("Infallible function failed: VectorOfVectorOfKeyLine::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfKeyLine_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfVectorOfKeyLine_set(self.as_raw_VectorOfVectorOfKeyLine(), index, val.as_raw_VectorOfKeyLine()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfKeyLine_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfKeyLine_set_unchecked(self.as_raw_VectorOfVectorOfKeyLine(), index, val.as_raw_VectorOfKeyLine())
		}
	
	}
	
	unsafe impl Send for VectorOfVectorOfKeyLine {}
	
}
#[cfg(feature = "contrib")]
pub use line_descriptor_types::*;

mod ml_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfANN_MLP {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfANN_MLP {
		pub fn as_raw_PtrOfANN_MLP(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfANN_MLP_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfANN_MLP_get_inner_ptr(self.as_raw_PtrOfANN_MLP()) }
		}
	}
	
	impl Drop for PtrOfANN_MLP {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfANN_MLP_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfANN_MLP_delete(self.as_raw_PtrOfANN_MLP()) };
		}
	}
	
	unsafe impl Send for PtrOfANN_MLP {}
	
	impl crate::ml::ANN_MLP for PtrOfANN_MLP {
		fn as_raw_ANN_MLP(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl core::AlgorithmTrait for PtrOfANN_MLP {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::ml::StatModel for PtrOfANN_MLP {
		fn as_raw_StatModel(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfBoost {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfBoost {
		pub fn as_raw_PtrOfBoost(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfBoost_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfBoost_get_inner_ptr(self.as_raw_PtrOfBoost()) }
		}
	}
	
	impl Drop for PtrOfBoost {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfBoost_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfBoost_delete(self.as_raw_PtrOfBoost()) };
		}
	}
	
	unsafe impl Send for PtrOfBoost {}
	
	impl core::AlgorithmTrait for PtrOfBoost {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::ml::Boost for PtrOfBoost {
		fn as_raw_Boost(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::ml::DTrees for PtrOfBoost {
		fn as_raw_DTrees(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::ml::StatModel for PtrOfBoost {
		fn as_raw_StatModel(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfDTrees {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfDTrees {
		pub fn as_raw_PtrOfDTrees(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfDTrees_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfDTrees_get_inner_ptr(self.as_raw_PtrOfDTrees()) }
		}
	}
	
	impl Drop for PtrOfDTrees {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfDTrees_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfDTrees_delete(self.as_raw_PtrOfDTrees()) };
		}
	}
	
	unsafe impl Send for PtrOfDTrees {}
	
	impl core::AlgorithmTrait for PtrOfDTrees {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::ml::DTrees for PtrOfDTrees {
		fn as_raw_DTrees(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::ml::StatModel for PtrOfDTrees {
		fn as_raw_StatModel(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfEM {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfEM {
		pub fn as_raw_PtrOfEM(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfEM_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfEM_get_inner_ptr(self.as_raw_PtrOfEM()) }
		}
	}
	
	impl Drop for PtrOfEM {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfEM_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfEM_delete(self.as_raw_PtrOfEM()) };
		}
	}
	
	unsafe impl Send for PtrOfEM {}
	
	impl core::AlgorithmTrait for PtrOfEM {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::ml::EM for PtrOfEM {
		fn as_raw_EM(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::ml::StatModel for PtrOfEM {
		fn as_raw_StatModel(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfKNearest {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfKNearest {
		pub fn as_raw_PtrOfKNearest(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfKNearest_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfKNearest_get_inner_ptr(self.as_raw_PtrOfKNearest()) }
		}
	}
	
	impl Drop for PtrOfKNearest {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfKNearest_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfKNearest_delete(self.as_raw_PtrOfKNearest()) };
		}
	}
	
	unsafe impl Send for PtrOfKNearest {}
	
	impl core::AlgorithmTrait for PtrOfKNearest {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::ml::KNearest for PtrOfKNearest {
		fn as_raw_KNearest(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::ml::StatModel for PtrOfKNearest {
		fn as_raw_StatModel(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfLogisticRegression {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfLogisticRegression {
		pub fn as_raw_PtrOfLogisticRegression(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfLogisticRegression_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfLogisticRegression_get_inner_ptr(self.as_raw_PtrOfLogisticRegression()) }
		}
	}
	
	impl Drop for PtrOfLogisticRegression {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfLogisticRegression_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfLogisticRegression_delete(self.as_raw_PtrOfLogisticRegression()) };
		}
	}
	
	unsafe impl Send for PtrOfLogisticRegression {}
	
	impl core::AlgorithmTrait for PtrOfLogisticRegression {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::ml::LogisticRegression for PtrOfLogisticRegression {
		fn as_raw_LogisticRegression(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::ml::StatModel for PtrOfLogisticRegression {
		fn as_raw_StatModel(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfNormalBayesClassifier {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfNormalBayesClassifier {
		pub fn as_raw_PtrOfNormalBayesClassifier(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfNormalBayesClassifier_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfNormalBayesClassifier_get_inner_ptr(self.as_raw_PtrOfNormalBayesClassifier()) }
		}
	}
	
	impl Drop for PtrOfNormalBayesClassifier {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfNormalBayesClassifier_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfNormalBayesClassifier_delete(self.as_raw_PtrOfNormalBayesClassifier()) };
		}
	}
	
	unsafe impl Send for PtrOfNormalBayesClassifier {}
	
	impl core::AlgorithmTrait for PtrOfNormalBayesClassifier {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::ml::NormalBayesClassifier for PtrOfNormalBayesClassifier {
		fn as_raw_NormalBayesClassifier(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::ml::StatModel for PtrOfNormalBayesClassifier {
		fn as_raw_StatModel(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfRTrees {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfRTrees {
		pub fn as_raw_PtrOfRTrees(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfRTrees_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfRTrees_get_inner_ptr(self.as_raw_PtrOfRTrees()) }
		}
	}
	
	impl Drop for PtrOfRTrees {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfRTrees_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfRTrees_delete(self.as_raw_PtrOfRTrees()) };
		}
	}
	
	unsafe impl Send for PtrOfRTrees {}
	
	impl core::AlgorithmTrait for PtrOfRTrees {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::ml::DTrees for PtrOfRTrees {
		fn as_raw_DTrees(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::ml::RTrees for PtrOfRTrees {
		fn as_raw_RTrees(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::ml::StatModel for PtrOfRTrees {
		fn as_raw_StatModel(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfSVM {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfSVM {
		pub fn as_raw_PtrOfSVM(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfSVM_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfSVM_get_inner_ptr(self.as_raw_PtrOfSVM()) }
		}
	}
	
	impl Drop for PtrOfSVM {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfSVM_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfSVM_delete(self.as_raw_PtrOfSVM()) };
		}
	}
	
	unsafe impl Send for PtrOfSVM {}
	
	impl core::AlgorithmTrait for PtrOfSVM {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::ml::SVM for PtrOfSVM {
		fn as_raw_SVM(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::ml::StatModel for PtrOfSVM {
		fn as_raw_StatModel(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfSVMSGD {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfSVMSGD {
		pub fn as_raw_PtrOfSVMSGD(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfSVMSGD_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfSVMSGD_get_inner_ptr(self.as_raw_PtrOfSVMSGD()) }
		}
	}
	
	impl Drop for PtrOfSVMSGD {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfSVMSGD_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfSVMSGD_delete(self.as_raw_PtrOfSVMSGD()) };
		}
	}
	
	unsafe impl Send for PtrOfSVMSGD {}
	
	impl core::AlgorithmTrait for PtrOfSVMSGD {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::ml::SVMSGD for PtrOfSVMSGD {
		fn as_raw_SVMSGD(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::ml::StatModel for PtrOfSVMSGD {
		fn as_raw_StatModel(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfSVM_Kernel {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfSVM_Kernel {
		pub fn as_raw_PtrOfSVM_Kernel(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfSVM_Kernel_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfSVM_Kernel_get_inner_ptr(self.as_raw_PtrOfSVM_Kernel()) }
		}
	}
	
	impl Drop for PtrOfSVM_Kernel {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfSVM_Kernel_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfSVM_Kernel_delete(self.as_raw_PtrOfSVM_Kernel()) };
		}
	}
	
	unsafe impl Send for PtrOfSVM_Kernel {}
	
	impl core::AlgorithmTrait for PtrOfSVM_Kernel {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::ml::SVM_Kernel for PtrOfSVM_Kernel {
		fn as_raw_SVM_Kernel(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfTrainData {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfTrainData {
		pub fn as_raw_PtrOfTrainData(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfTrainData_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfTrainData_get_inner_ptr(self.as_raw_PtrOfTrainData()) }
		}
	}
	
	impl Drop for PtrOfTrainData {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfTrainData_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfTrainData_delete(self.as_raw_PtrOfTrainData()) };
		}
	}
	
	unsafe impl Send for PtrOfTrainData {}
	
	impl crate::ml::TrainData for PtrOfTrainData {
		fn as_raw_TrainData(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct VectorOfDTrees_Node {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfDTrees_Node {
		pub fn as_raw_VectorOfDTrees_Node(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfDTrees_Node {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfDTrees_Node_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfDTrees_Node_delete(self.as_raw_VectorOfDTrees_Node()) };
		}
	}
	
	impl IntoIterator for VectorOfDTrees_Node {
		type Item = crate::ml::DTrees_Node;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfDTrees_Node {
		type Item = crate::ml::DTrees_Node;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfDTrees_Node>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfDTrees_Node {
		type Storage = crate::ml::DTrees_Node;
		type Arg = crate::ml::DTrees_Node;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfDTrees_Node_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfDTrees_Node_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfDTrees_Node_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfDTrees_Node_len(self.as_raw_VectorOfDTrees_Node()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfDTrees_Node_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfDTrees_Node_is_empty(self.as_raw_VectorOfDTrees_Node()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfDTrees_Node_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfDTrees_Node_capacity(self.as_raw_VectorOfDTrees_Node()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfDTrees_Node_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfDTrees_Node_shrink_to_fit(self.as_raw_VectorOfDTrees_Node()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfDTrees_Node_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfDTrees_Node_reserve(self.as_raw_VectorOfDTrees_Node(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfDTrees_Node_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfDTrees_Node_remove(self.as_raw_VectorOfDTrees_Node(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfDTrees_Node_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfDTrees_Node_swap(self.as_raw_VectorOfDTrees_Node(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfDTrees_Node_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfDTrees_Node_clear(self.as_raw_VectorOfDTrees_Node()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfDTrees_Node_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfDTrees_Node_push(self.as_raw_VectorOfDTrees_Node(), val.as_raw_DTrees_Node()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfDTrees_Node_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfDTrees_Node_insert(self.as_raw_VectorOfDTrees_Node(), index, val.as_raw_DTrees_Node()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfDTrees_Node_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfDTrees_Node_get(self.as_raw_VectorOfDTrees_Node(), index) }
				.into_result()
				.map(|ptr| crate::ml::DTrees_Node { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfDTrees_Node_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfDTrees_Node_get_unchecked(self.as_raw_VectorOfDTrees_Node(), index)
				.into_result()
				.map(|ptr| crate::ml::DTrees_Node { ptr })
				.expect("Infallible function failed: VectorOfDTrees_Node::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfDTrees_Node_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfDTrees_Node_set(self.as_raw_VectorOfDTrees_Node(), index, val.as_raw_DTrees_Node()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfDTrees_Node_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfDTrees_Node_set_unchecked(self.as_raw_VectorOfDTrees_Node(), index, val.as_raw_DTrees_Node())
		}
	
	}
	
	unsafe impl Send for VectorOfDTrees_Node {}
	
	pub struct VectorOfDTrees_Split {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfDTrees_Split {
		pub fn as_raw_VectorOfDTrees_Split(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfDTrees_Split {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfDTrees_Split_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfDTrees_Split_delete(self.as_raw_VectorOfDTrees_Split()) };
		}
	}
	
	impl IntoIterator for VectorOfDTrees_Split {
		type Item = crate::ml::DTrees_Split;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfDTrees_Split {
		type Item = crate::ml::DTrees_Split;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfDTrees_Split>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfDTrees_Split {
		type Storage = crate::ml::DTrees_Split;
		type Arg = crate::ml::DTrees_Split;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfDTrees_Split_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfDTrees_Split_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfDTrees_Split_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfDTrees_Split_len(self.as_raw_VectorOfDTrees_Split()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfDTrees_Split_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfDTrees_Split_is_empty(self.as_raw_VectorOfDTrees_Split()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfDTrees_Split_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfDTrees_Split_capacity(self.as_raw_VectorOfDTrees_Split()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfDTrees_Split_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfDTrees_Split_shrink_to_fit(self.as_raw_VectorOfDTrees_Split()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfDTrees_Split_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfDTrees_Split_reserve(self.as_raw_VectorOfDTrees_Split(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfDTrees_Split_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfDTrees_Split_remove(self.as_raw_VectorOfDTrees_Split(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfDTrees_Split_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfDTrees_Split_swap(self.as_raw_VectorOfDTrees_Split(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfDTrees_Split_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfDTrees_Split_clear(self.as_raw_VectorOfDTrees_Split()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfDTrees_Split_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfDTrees_Split_push(self.as_raw_VectorOfDTrees_Split(), val.as_raw_DTrees_Split()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfDTrees_Split_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfDTrees_Split_insert(self.as_raw_VectorOfDTrees_Split(), index, val.as_raw_DTrees_Split()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfDTrees_Split_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfDTrees_Split_get(self.as_raw_VectorOfDTrees_Split(), index) }
				.into_result()
				.map(|ptr| crate::ml::DTrees_Split { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfDTrees_Split_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfDTrees_Split_get_unchecked(self.as_raw_VectorOfDTrees_Split(), index)
				.into_result()
				.map(|ptr| crate::ml::DTrees_Split { ptr })
				.expect("Infallible function failed: VectorOfDTrees_Split::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfDTrees_Split_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfDTrees_Split_set(self.as_raw_VectorOfDTrees_Split(), index, val.as_raw_DTrees_Split()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfDTrees_Split_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfDTrees_Split_set_unchecked(self.as_raw_VectorOfDTrees_Split(), index, val.as_raw_DTrees_Split())
		}
	
	}
	
	unsafe impl Send for VectorOfDTrees_Split {}
	
}
pub use ml_types::*;

mod objdetect_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfBaseCascadeClassifier {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfBaseCascadeClassifier {
		pub fn as_raw_PtrOfBaseCascadeClassifier(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfBaseCascadeClassifier_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfBaseCascadeClassifier_get_inner_ptr(self.as_raw_PtrOfBaseCascadeClassifier()) }
		}
	}
	
	impl Drop for PtrOfBaseCascadeClassifier {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfBaseCascadeClassifier_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfBaseCascadeClassifier_delete(self.as_raw_PtrOfBaseCascadeClassifier()) };
		}
	}
	
	unsafe impl Send for PtrOfBaseCascadeClassifier {}
	
	impl core::AlgorithmTrait for PtrOfBaseCascadeClassifier {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::objdetect::BaseCascadeClassifier for PtrOfBaseCascadeClassifier {
		fn as_raw_BaseCascadeClassifier(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfBaseCascadeClassifier_MaskGenerator {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfBaseCascadeClassifier_MaskGenerator {
		pub fn as_raw_PtrOfBaseCascadeClassifier_MaskGenerator(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfBaseCascadeClassifier_MaskGenerator_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfBaseCascadeClassifier_MaskGenerator_get_inner_ptr(self.as_raw_PtrOfBaseCascadeClassifier_MaskGenerator()) }
		}
	}
	
	impl Drop for PtrOfBaseCascadeClassifier_MaskGenerator {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfBaseCascadeClassifier_MaskGenerator_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfBaseCascadeClassifier_MaskGenerator_delete(self.as_raw_PtrOfBaseCascadeClassifier_MaskGenerator()) };
		}
	}
	
	unsafe impl Send for PtrOfBaseCascadeClassifier_MaskGenerator {}
	
	impl crate::objdetect::BaseCascadeClassifier_MaskGenerator for PtrOfBaseCascadeClassifier_MaskGenerator {
		fn as_raw_BaseCascadeClassifier_MaskGenerator(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfDetectionBasedTracker_IDetector {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfDetectionBasedTracker_IDetector {
		pub fn as_raw_PtrOfDetectionBasedTracker_IDetector(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfDetectionBasedTracker_IDetector_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfDetectionBasedTracker_IDetector_get_inner_ptr(self.as_raw_PtrOfDetectionBasedTracker_IDetector()) }
		}
	}
	
	impl Drop for PtrOfDetectionBasedTracker_IDetector {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfDetectionBasedTracker_IDetector_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfDetectionBasedTracker_IDetector_delete(self.as_raw_PtrOfDetectionBasedTracker_IDetector()) };
		}
	}
	
	unsafe impl Send for PtrOfDetectionBasedTracker_IDetector {}
	
	impl crate::objdetect::DetectionBasedTracker_IDetector for PtrOfDetectionBasedTracker_IDetector {
		fn as_raw_DetectionBasedTracker_IDetector(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct VectorOfDetectionBasedTracker_ExtObject {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfDetectionBasedTracker_ExtObject {
		pub fn as_raw_VectorOfDetectionBasedTracker_ExtObject(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfDetectionBasedTracker_ExtObject {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfDetectionBasedTracker_ExtObject_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfDetectionBasedTracker_ExtObject_delete(self.as_raw_VectorOfDetectionBasedTracker_ExtObject()) };
		}
	}
	
	impl IntoIterator for VectorOfDetectionBasedTracker_ExtObject {
		type Item = crate::objdetect::DetectionBasedTracker_ExtObject;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfDetectionBasedTracker_ExtObject {
		type Item = crate::objdetect::DetectionBasedTracker_ExtObject;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfDetectionBasedTracker_ExtObject>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfDetectionBasedTracker_ExtObject {
		type Storage = crate::objdetect::DetectionBasedTracker_ExtObject;
		type Arg = crate::objdetect::DetectionBasedTracker_ExtObject;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfDetectionBasedTracker_ExtObject_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfDetectionBasedTracker_ExtObject_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfDetectionBasedTracker_ExtObject_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfDetectionBasedTracker_ExtObject_len(self.as_raw_VectorOfDetectionBasedTracker_ExtObject()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfDetectionBasedTracker_ExtObject_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfDetectionBasedTracker_ExtObject_is_empty(self.as_raw_VectorOfDetectionBasedTracker_ExtObject()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfDetectionBasedTracker_ExtObject_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfDetectionBasedTracker_ExtObject_capacity(self.as_raw_VectorOfDetectionBasedTracker_ExtObject()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfDetectionBasedTracker_ExtObject_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfDetectionBasedTracker_ExtObject_shrink_to_fit(self.as_raw_VectorOfDetectionBasedTracker_ExtObject()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfDetectionBasedTracker_ExtObject_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfDetectionBasedTracker_ExtObject_reserve(self.as_raw_VectorOfDetectionBasedTracker_ExtObject(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfDetectionBasedTracker_ExtObject_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfDetectionBasedTracker_ExtObject_remove(self.as_raw_VectorOfDetectionBasedTracker_ExtObject(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfDetectionBasedTracker_ExtObject_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfDetectionBasedTracker_ExtObject_swap(self.as_raw_VectorOfDetectionBasedTracker_ExtObject(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfDetectionBasedTracker_ExtObject_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfDetectionBasedTracker_ExtObject_clear(self.as_raw_VectorOfDetectionBasedTracker_ExtObject()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfDetectionBasedTracker_ExtObject_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfDetectionBasedTracker_ExtObject_push(self.as_raw_VectorOfDetectionBasedTracker_ExtObject(), val.as_raw_DetectionBasedTracker_ExtObject()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfDetectionBasedTracker_ExtObject_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfDetectionBasedTracker_ExtObject_insert(self.as_raw_VectorOfDetectionBasedTracker_ExtObject(), index, val.as_raw_DetectionBasedTracker_ExtObject()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfDetectionBasedTracker_ExtObject_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfDetectionBasedTracker_ExtObject_get(self.as_raw_VectorOfDetectionBasedTracker_ExtObject(), index) }
				.into_result()
				.map(|ptr| crate::objdetect::DetectionBasedTracker_ExtObject { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfDetectionBasedTracker_ExtObject_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfDetectionBasedTracker_ExtObject_get_unchecked(self.as_raw_VectorOfDetectionBasedTracker_ExtObject(), index)
				.into_result()
				.map(|ptr| crate::objdetect::DetectionBasedTracker_ExtObject { ptr })
				.expect("Infallible function failed: VectorOfDetectionBasedTracker_ExtObject::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfDetectionBasedTracker_ExtObject_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfDetectionBasedTracker_ExtObject_set(self.as_raw_VectorOfDetectionBasedTracker_ExtObject(), index, val.as_raw_DetectionBasedTracker_ExtObject()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfDetectionBasedTracker_ExtObject_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfDetectionBasedTracker_ExtObject_set_unchecked(self.as_raw_VectorOfDetectionBasedTracker_ExtObject(), index, val.as_raw_DetectionBasedTracker_ExtObject())
		}
	
	}
	
	unsafe impl Send for VectorOfDetectionBasedTracker_ExtObject {}
	
	pub struct VectorOfDetectionROI {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfDetectionROI {
		pub fn as_raw_VectorOfDetectionROI(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfDetectionROI {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfDetectionROI_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfDetectionROI_delete(self.as_raw_VectorOfDetectionROI()) };
		}
	}
	
	impl IntoIterator for VectorOfDetectionROI {
		type Item = crate::objdetect::DetectionROI;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfDetectionROI {
		type Item = crate::objdetect::DetectionROI;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfDetectionROI>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfDetectionROI {
		type Storage = crate::objdetect::DetectionROI;
		type Arg = crate::objdetect::DetectionROI;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfDetectionROI_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfDetectionROI_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfDetectionROI_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfDetectionROI_len(self.as_raw_VectorOfDetectionROI()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfDetectionROI_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfDetectionROI_is_empty(self.as_raw_VectorOfDetectionROI()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfDetectionROI_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfDetectionROI_capacity(self.as_raw_VectorOfDetectionROI()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfDetectionROI_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfDetectionROI_shrink_to_fit(self.as_raw_VectorOfDetectionROI()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfDetectionROI_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfDetectionROI_reserve(self.as_raw_VectorOfDetectionROI(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfDetectionROI_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfDetectionROI_remove(self.as_raw_VectorOfDetectionROI(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfDetectionROI_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfDetectionROI_swap(self.as_raw_VectorOfDetectionROI(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfDetectionROI_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfDetectionROI_clear(self.as_raw_VectorOfDetectionROI()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfDetectionROI_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfDetectionROI_push(self.as_raw_VectorOfDetectionROI(), val.as_raw_DetectionROI()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfDetectionROI_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfDetectionROI_insert(self.as_raw_VectorOfDetectionROI(), index, val.as_raw_DetectionROI()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfDetectionROI_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfDetectionROI_get(self.as_raw_VectorOfDetectionROI(), index) }
				.into_result()
				.map(|ptr| crate::objdetect::DetectionROI { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfDetectionROI_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfDetectionROI_get_unchecked(self.as_raw_VectorOfDetectionROI(), index)
				.into_result()
				.map(|ptr| crate::objdetect::DetectionROI { ptr })
				.expect("Infallible function failed: VectorOfDetectionROI::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfDetectionROI_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfDetectionROI_set(self.as_raw_VectorOfDetectionROI(), index, val.as_raw_DetectionROI()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfDetectionROI_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfDetectionROI_set_unchecked(self.as_raw_VectorOfDetectionROI(), index, val.as_raw_DetectionROI())
		}
	
	}
	
	unsafe impl Send for VectorOfDetectionROI {}
	
}
pub use objdetect_types::*;

#[cfg(feature = "contrib")]
mod phase_unwrapping_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfHistogramPhaseUnwrapping {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfHistogramPhaseUnwrapping {
		pub fn as_raw_PtrOfHistogramPhaseUnwrapping(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfHistogramPhaseUnwrapping_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfHistogramPhaseUnwrapping_get_inner_ptr(self.as_raw_PtrOfHistogramPhaseUnwrapping()) }
		}
	}
	
	impl Drop for PtrOfHistogramPhaseUnwrapping {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfHistogramPhaseUnwrapping_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfHistogramPhaseUnwrapping_delete(self.as_raw_PtrOfHistogramPhaseUnwrapping()) };
		}
	}
	
	unsafe impl Send for PtrOfHistogramPhaseUnwrapping {}
	
	impl core::AlgorithmTrait for PtrOfHistogramPhaseUnwrapping {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::phase_unwrapping::HistogramPhaseUnwrapping for PtrOfHistogramPhaseUnwrapping {
		fn as_raw_HistogramPhaseUnwrapping(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::phase_unwrapping::PhaseUnwrapping for PtrOfHistogramPhaseUnwrapping {
		fn as_raw_PhaseUnwrapping(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
}
#[cfg(feature = "contrib")]
pub use phase_unwrapping_types::*;

mod photo_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfAlignMTB {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfAlignMTB {
		pub fn as_raw_PtrOfAlignMTB(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfAlignMTB_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfAlignMTB_get_inner_ptr(self.as_raw_PtrOfAlignMTB()) }
		}
	}
	
	impl Drop for PtrOfAlignMTB {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfAlignMTB_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfAlignMTB_delete(self.as_raw_PtrOfAlignMTB()) };
		}
	}
	
	unsafe impl Send for PtrOfAlignMTB {}
	
	impl core::AlgorithmTrait for PtrOfAlignMTB {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::photo::AlignExposures for PtrOfAlignMTB {
		fn as_raw_AlignExposures(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::photo::AlignMTB for PtrOfAlignMTB {
		fn as_raw_AlignMTB(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfCalibrateDebevec {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfCalibrateDebevec {
		pub fn as_raw_PtrOfCalibrateDebevec(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfCalibrateDebevec_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfCalibrateDebevec_get_inner_ptr(self.as_raw_PtrOfCalibrateDebevec()) }
		}
	}
	
	impl Drop for PtrOfCalibrateDebevec {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfCalibrateDebevec_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfCalibrateDebevec_delete(self.as_raw_PtrOfCalibrateDebevec()) };
		}
	}
	
	unsafe impl Send for PtrOfCalibrateDebevec {}
	
	impl core::AlgorithmTrait for PtrOfCalibrateDebevec {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::photo::CalibrateCRF for PtrOfCalibrateDebevec {
		fn as_raw_CalibrateCRF(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::photo::CalibrateDebevec for PtrOfCalibrateDebevec {
		fn as_raw_CalibrateDebevec(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfCalibrateRobertson {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfCalibrateRobertson {
		pub fn as_raw_PtrOfCalibrateRobertson(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfCalibrateRobertson_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfCalibrateRobertson_get_inner_ptr(self.as_raw_PtrOfCalibrateRobertson()) }
		}
	}
	
	impl Drop for PtrOfCalibrateRobertson {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfCalibrateRobertson_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfCalibrateRobertson_delete(self.as_raw_PtrOfCalibrateRobertson()) };
		}
	}
	
	unsafe impl Send for PtrOfCalibrateRobertson {}
	
	impl core::AlgorithmTrait for PtrOfCalibrateRobertson {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::photo::CalibrateCRF for PtrOfCalibrateRobertson {
		fn as_raw_CalibrateCRF(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::photo::CalibrateRobertson for PtrOfCalibrateRobertson {
		fn as_raw_CalibrateRobertson(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfMergeDebevec {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfMergeDebevec {
		pub fn as_raw_PtrOfMergeDebevec(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfMergeDebevec_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfMergeDebevec_get_inner_ptr(self.as_raw_PtrOfMergeDebevec()) }
		}
	}
	
	impl Drop for PtrOfMergeDebevec {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfMergeDebevec_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfMergeDebevec_delete(self.as_raw_PtrOfMergeDebevec()) };
		}
	}
	
	unsafe impl Send for PtrOfMergeDebevec {}
	
	impl core::AlgorithmTrait for PtrOfMergeDebevec {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::photo::MergeDebevec for PtrOfMergeDebevec {
		fn as_raw_MergeDebevec(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::photo::MergeExposures for PtrOfMergeDebevec {
		fn as_raw_MergeExposures(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfMergeMertens {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfMergeMertens {
		pub fn as_raw_PtrOfMergeMertens(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfMergeMertens_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfMergeMertens_get_inner_ptr(self.as_raw_PtrOfMergeMertens()) }
		}
	}
	
	impl Drop for PtrOfMergeMertens {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfMergeMertens_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfMergeMertens_delete(self.as_raw_PtrOfMergeMertens()) };
		}
	}
	
	unsafe impl Send for PtrOfMergeMertens {}
	
	impl core::AlgorithmTrait for PtrOfMergeMertens {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::photo::MergeExposures for PtrOfMergeMertens {
		fn as_raw_MergeExposures(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::photo::MergeMertens for PtrOfMergeMertens {
		fn as_raw_MergeMertens(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfMergeRobertson {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfMergeRobertson {
		pub fn as_raw_PtrOfMergeRobertson(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfMergeRobertson_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfMergeRobertson_get_inner_ptr(self.as_raw_PtrOfMergeRobertson()) }
		}
	}
	
	impl Drop for PtrOfMergeRobertson {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfMergeRobertson_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfMergeRobertson_delete(self.as_raw_PtrOfMergeRobertson()) };
		}
	}
	
	unsafe impl Send for PtrOfMergeRobertson {}
	
	impl core::AlgorithmTrait for PtrOfMergeRobertson {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::photo::MergeExposures for PtrOfMergeRobertson {
		fn as_raw_MergeExposures(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::photo::MergeRobertson for PtrOfMergeRobertson {
		fn as_raw_MergeRobertson(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfTonemap {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfTonemap {
		pub fn as_raw_PtrOfTonemap(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfTonemap_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfTonemap_get_inner_ptr(self.as_raw_PtrOfTonemap()) }
		}
	}
	
	impl Drop for PtrOfTonemap {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfTonemap_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfTonemap_delete(self.as_raw_PtrOfTonemap()) };
		}
	}
	
	unsafe impl Send for PtrOfTonemap {}
	
	impl core::AlgorithmTrait for PtrOfTonemap {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::photo::Tonemap for PtrOfTonemap {
		fn as_raw_Tonemap(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfTonemapDrago {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfTonemapDrago {
		pub fn as_raw_PtrOfTonemapDrago(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfTonemapDrago_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfTonemapDrago_get_inner_ptr(self.as_raw_PtrOfTonemapDrago()) }
		}
	}
	
	impl Drop for PtrOfTonemapDrago {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfTonemapDrago_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfTonemapDrago_delete(self.as_raw_PtrOfTonemapDrago()) };
		}
	}
	
	unsafe impl Send for PtrOfTonemapDrago {}
	
	impl core::AlgorithmTrait for PtrOfTonemapDrago {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::photo::Tonemap for PtrOfTonemapDrago {
		fn as_raw_Tonemap(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::photo::TonemapDrago for PtrOfTonemapDrago {
		fn as_raw_TonemapDrago(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfTonemapDurand {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfTonemapDurand {
		pub fn as_raw_PtrOfTonemapDurand(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfTonemapDurand_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfTonemapDurand_get_inner_ptr(self.as_raw_PtrOfTonemapDurand()) }
		}
	}
	
	impl Drop for PtrOfTonemapDurand {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfTonemapDurand_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfTonemapDurand_delete(self.as_raw_PtrOfTonemapDurand()) };
		}
	}
	
	unsafe impl Send for PtrOfTonemapDurand {}
	
	impl core::AlgorithmTrait for PtrOfTonemapDurand {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::photo::Tonemap for PtrOfTonemapDurand {
		fn as_raw_Tonemap(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::photo::TonemapDurand for PtrOfTonemapDurand {
		fn as_raw_TonemapDurand(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfTonemapMantiuk {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfTonemapMantiuk {
		pub fn as_raw_PtrOfTonemapMantiuk(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfTonemapMantiuk_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfTonemapMantiuk_get_inner_ptr(self.as_raw_PtrOfTonemapMantiuk()) }
		}
	}
	
	impl Drop for PtrOfTonemapMantiuk {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfTonemapMantiuk_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfTonemapMantiuk_delete(self.as_raw_PtrOfTonemapMantiuk()) };
		}
	}
	
	unsafe impl Send for PtrOfTonemapMantiuk {}
	
	impl core::AlgorithmTrait for PtrOfTonemapMantiuk {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::photo::Tonemap for PtrOfTonemapMantiuk {
		fn as_raw_Tonemap(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::photo::TonemapMantiuk for PtrOfTonemapMantiuk {
		fn as_raw_TonemapMantiuk(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfTonemapReinhard {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfTonemapReinhard {
		pub fn as_raw_PtrOfTonemapReinhard(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfTonemapReinhard_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfTonemapReinhard_get_inner_ptr(self.as_raw_PtrOfTonemapReinhard()) }
		}
	}
	
	impl Drop for PtrOfTonemapReinhard {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfTonemapReinhard_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfTonemapReinhard_delete(self.as_raw_PtrOfTonemapReinhard()) };
		}
	}
	
	unsafe impl Send for PtrOfTonemapReinhard {}
	
	impl core::AlgorithmTrait for PtrOfTonemapReinhard {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::photo::Tonemap for PtrOfTonemapReinhard {
		fn as_raw_Tonemap(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::photo::TonemapReinhard for PtrOfTonemapReinhard {
		fn as_raw_TonemapReinhard(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
}
pub use photo_types::*;

#[cfg(feature = "contrib")]
mod plot_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfPlot2d {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfPlot2d {
		pub fn as_raw_PtrOfPlot2d(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfPlot2d_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfPlot2d_get_inner_ptr(self.as_raw_PtrOfPlot2d()) }
		}
	}
	
	impl Drop for PtrOfPlot2d {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfPlot2d_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfPlot2d_delete(self.as_raw_PtrOfPlot2d()) };
		}
	}
	
	unsafe impl Send for PtrOfPlot2d {}
	
	impl core::AlgorithmTrait for PtrOfPlot2d {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::plot::Plot2d for PtrOfPlot2d {
		fn as_raw_Plot2d(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
}
#[cfg(feature = "contrib")]
pub use plot_types::*;

#[cfg(feature = "contrib")]
mod sfm_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfSFMLibmvEuclideanReconstruction {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfSFMLibmvEuclideanReconstruction {
		pub fn as_raw_PtrOfSFMLibmvEuclideanReconstruction(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfSFMLibmvEuclideanReconstruction_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfSFMLibmvEuclideanReconstruction_get_inner_ptr(self.as_raw_PtrOfSFMLibmvEuclideanReconstruction()) }
		}
	}
	
	impl Drop for PtrOfSFMLibmvEuclideanReconstruction {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfSFMLibmvEuclideanReconstruction_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfSFMLibmvEuclideanReconstruction_delete(self.as_raw_PtrOfSFMLibmvEuclideanReconstruction()) };
		}
	}
	
	unsafe impl Send for PtrOfSFMLibmvEuclideanReconstruction {}
	
	impl crate::sfm::BaseSFM for PtrOfSFMLibmvEuclideanReconstruction {
		fn as_raw_BaseSFM(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::sfm::SFMLibmvEuclideanReconstruction for PtrOfSFMLibmvEuclideanReconstruction {
		fn as_raw_SFMLibmvEuclideanReconstruction(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
}
#[cfg(feature = "contrib")]
pub use sfm_types::*;

mod shape_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfAffineTransformer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfAffineTransformer {
		pub fn as_raw_PtrOfAffineTransformer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfAffineTransformer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfAffineTransformer_get_inner_ptr(self.as_raw_PtrOfAffineTransformer()) }
		}
	}
	
	impl Drop for PtrOfAffineTransformer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfAffineTransformer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfAffineTransformer_delete(self.as_raw_PtrOfAffineTransformer()) };
		}
	}
	
	unsafe impl Send for PtrOfAffineTransformer {}
	
	impl crate::shape::AffineTransformer for PtrOfAffineTransformer {
		fn as_raw_AffineTransformer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl core::AlgorithmTrait for PtrOfAffineTransformer {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::shape::ShapeTransformer for PtrOfAffineTransformer {
		fn as_raw_ShapeTransformer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfHausdorffDistanceExtractor {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfHausdorffDistanceExtractor {
		pub fn as_raw_PtrOfHausdorffDistanceExtractor(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfHausdorffDistanceExtractor_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfHausdorffDistanceExtractor_get_inner_ptr(self.as_raw_PtrOfHausdorffDistanceExtractor()) }
		}
	}
	
	impl Drop for PtrOfHausdorffDistanceExtractor {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfHausdorffDistanceExtractor_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfHausdorffDistanceExtractor_delete(self.as_raw_PtrOfHausdorffDistanceExtractor()) };
		}
	}
	
	unsafe impl Send for PtrOfHausdorffDistanceExtractor {}
	
	impl core::AlgorithmTrait for PtrOfHausdorffDistanceExtractor {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::shape::HausdorffDistanceExtractor for PtrOfHausdorffDistanceExtractor {
		fn as_raw_HausdorffDistanceExtractor(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::shape::ShapeDistanceExtractor for PtrOfHausdorffDistanceExtractor {
		fn as_raw_ShapeDistanceExtractor(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfHistogramCostExtractor {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfHistogramCostExtractor {
		pub fn as_raw_PtrOfHistogramCostExtractor(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfHistogramCostExtractor_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfHistogramCostExtractor_get_inner_ptr(self.as_raw_PtrOfHistogramCostExtractor()) }
		}
	}
	
	impl Drop for PtrOfHistogramCostExtractor {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfHistogramCostExtractor_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfHistogramCostExtractor_delete(self.as_raw_PtrOfHistogramCostExtractor()) };
		}
	}
	
	unsafe impl Send for PtrOfHistogramCostExtractor {}
	
	impl core::AlgorithmTrait for PtrOfHistogramCostExtractor {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::shape::HistogramCostExtractor for PtrOfHistogramCostExtractor {
		fn as_raw_HistogramCostExtractor(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfShapeContextDistanceExtractor {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfShapeContextDistanceExtractor {
		pub fn as_raw_PtrOfShapeContextDistanceExtractor(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfShapeContextDistanceExtractor_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfShapeContextDistanceExtractor_get_inner_ptr(self.as_raw_PtrOfShapeContextDistanceExtractor()) }
		}
	}
	
	impl Drop for PtrOfShapeContextDistanceExtractor {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfShapeContextDistanceExtractor_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfShapeContextDistanceExtractor_delete(self.as_raw_PtrOfShapeContextDistanceExtractor()) };
		}
	}
	
	unsafe impl Send for PtrOfShapeContextDistanceExtractor {}
	
	impl core::AlgorithmTrait for PtrOfShapeContextDistanceExtractor {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::shape::ShapeContextDistanceExtractor for PtrOfShapeContextDistanceExtractor {
		fn as_raw_ShapeContextDistanceExtractor(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::shape::ShapeDistanceExtractor for PtrOfShapeContextDistanceExtractor {
		fn as_raw_ShapeDistanceExtractor(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfShapeTransformer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfShapeTransformer {
		pub fn as_raw_PtrOfShapeTransformer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfShapeTransformer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfShapeTransformer_get_inner_ptr(self.as_raw_PtrOfShapeTransformer()) }
		}
	}
	
	impl Drop for PtrOfShapeTransformer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfShapeTransformer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfShapeTransformer_delete(self.as_raw_PtrOfShapeTransformer()) };
		}
	}
	
	unsafe impl Send for PtrOfShapeTransformer {}
	
	impl core::AlgorithmTrait for PtrOfShapeTransformer {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::shape::ShapeTransformer for PtrOfShapeTransformer {
		fn as_raw_ShapeTransformer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfThinPlateSplineShapeTransformer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfThinPlateSplineShapeTransformer {
		pub fn as_raw_PtrOfThinPlateSplineShapeTransformer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfThinPlateSplineShapeTransformer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfThinPlateSplineShapeTransformer_get_inner_ptr(self.as_raw_PtrOfThinPlateSplineShapeTransformer()) }
		}
	}
	
	impl Drop for PtrOfThinPlateSplineShapeTransformer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfThinPlateSplineShapeTransformer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfThinPlateSplineShapeTransformer_delete(self.as_raw_PtrOfThinPlateSplineShapeTransformer()) };
		}
	}
	
	unsafe impl Send for PtrOfThinPlateSplineShapeTransformer {}
	
	impl core::AlgorithmTrait for PtrOfThinPlateSplineShapeTransformer {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::shape::ShapeTransformer for PtrOfThinPlateSplineShapeTransformer {
		fn as_raw_ShapeTransformer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::shape::ThinPlateSplineShapeTransformer for PtrOfThinPlateSplineShapeTransformer {
		fn as_raw_ThinPlateSplineShapeTransformer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
}
pub use shape_types::*;

mod stitching_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfDetail_Blender {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfDetail_Blender {
		pub fn as_raw_PtrOfDetail_Blender(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfDetail_Blender_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfDetail_Blender_get_inner_ptr(self.as_raw_PtrOfDetail_Blender()) }
		}
	}
	
	impl Drop for PtrOfDetail_Blender {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfDetail_Blender_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfDetail_Blender_delete(self.as_raw_PtrOfDetail_Blender()) };
		}
	}
	
	unsafe impl Send for PtrOfDetail_Blender {}
	
	impl crate::stitching::Detail_BlenderTrait for PtrOfDetail_Blender {
		fn as_raw_Detail_Blender(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfDetail_BundleAdjusterBase {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfDetail_BundleAdjusterBase {
		pub fn as_raw_PtrOfDetail_BundleAdjusterBase(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfDetail_BundleAdjusterBase_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfDetail_BundleAdjusterBase_get_inner_ptr(self.as_raw_PtrOfDetail_BundleAdjusterBase()) }
		}
	}
	
	impl Drop for PtrOfDetail_BundleAdjusterBase {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfDetail_BundleAdjusterBase_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfDetail_BundleAdjusterBase_delete(self.as_raw_PtrOfDetail_BundleAdjusterBase()) };
		}
	}
	
	unsafe impl Send for PtrOfDetail_BundleAdjusterBase {}
	
	impl crate::stitching::Detail_BundleAdjusterBase for PtrOfDetail_BundleAdjusterBase {
		fn as_raw_Detail_BundleAdjusterBase(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::stitching::Detail_Estimator for PtrOfDetail_BundleAdjusterBase {
		fn as_raw_Detail_Estimator(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfDetail_ExposureCompensator {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfDetail_ExposureCompensator {
		pub fn as_raw_PtrOfDetail_ExposureCompensator(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfDetail_ExposureCompensator_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfDetail_ExposureCompensator_get_inner_ptr(self.as_raw_PtrOfDetail_ExposureCompensator()) }
		}
	}
	
	impl Drop for PtrOfDetail_ExposureCompensator {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfDetail_ExposureCompensator_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfDetail_ExposureCompensator_delete(self.as_raw_PtrOfDetail_ExposureCompensator()) };
		}
	}
	
	unsafe impl Send for PtrOfDetail_ExposureCompensator {}
	
	impl crate::stitching::Detail_ExposureCompensator for PtrOfDetail_ExposureCompensator {
		fn as_raw_Detail_ExposureCompensator(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfDetail_FeaturesFinder {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfDetail_FeaturesFinder {
		pub fn as_raw_PtrOfDetail_FeaturesFinder(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfDetail_FeaturesFinder_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfDetail_FeaturesFinder_get_inner_ptr(self.as_raw_PtrOfDetail_FeaturesFinder()) }
		}
	}
	
	impl Drop for PtrOfDetail_FeaturesFinder {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfDetail_FeaturesFinder_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfDetail_FeaturesFinder_delete(self.as_raw_PtrOfDetail_FeaturesFinder()) };
		}
	}
	
	unsafe impl Send for PtrOfDetail_FeaturesFinder {}
	
	impl crate::stitching::Detail_FeaturesFinder for PtrOfDetail_FeaturesFinder {
		fn as_raw_Detail_FeaturesFinder(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfDetail_FeaturesMatcher {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfDetail_FeaturesMatcher {
		pub fn as_raw_PtrOfDetail_FeaturesMatcher(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfDetail_FeaturesMatcher_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfDetail_FeaturesMatcher_get_inner_ptr(self.as_raw_PtrOfDetail_FeaturesMatcher()) }
		}
	}
	
	impl Drop for PtrOfDetail_FeaturesMatcher {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfDetail_FeaturesMatcher_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfDetail_FeaturesMatcher_delete(self.as_raw_PtrOfDetail_FeaturesMatcher()) };
		}
	}
	
	unsafe impl Send for PtrOfDetail_FeaturesMatcher {}
	
	impl crate::stitching::Detail_FeaturesMatcher for PtrOfDetail_FeaturesMatcher {
		fn as_raw_Detail_FeaturesMatcher(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfDetail_RotationWarper {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfDetail_RotationWarper {
		pub fn as_raw_PtrOfDetail_RotationWarper(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfDetail_RotationWarper_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfDetail_RotationWarper_get_inner_ptr(self.as_raw_PtrOfDetail_RotationWarper()) }
		}
	}
	
	impl Drop for PtrOfDetail_RotationWarper {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfDetail_RotationWarper_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfDetail_RotationWarper_delete(self.as_raw_PtrOfDetail_RotationWarper()) };
		}
	}
	
	unsafe impl Send for PtrOfDetail_RotationWarper {}
	
	impl crate::stitching::Detail_RotationWarper for PtrOfDetail_RotationWarper {
		fn as_raw_Detail_RotationWarper(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfDetail_SeamFinder {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfDetail_SeamFinder {
		pub fn as_raw_PtrOfDetail_SeamFinder(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfDetail_SeamFinder_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfDetail_SeamFinder_get_inner_ptr(self.as_raw_PtrOfDetail_SeamFinder()) }
		}
	}
	
	impl Drop for PtrOfDetail_SeamFinder {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfDetail_SeamFinder_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfDetail_SeamFinder_delete(self.as_raw_PtrOfDetail_SeamFinder()) };
		}
	}
	
	unsafe impl Send for PtrOfDetail_SeamFinder {}
	
	impl crate::stitching::Detail_SeamFinder for PtrOfDetail_SeamFinder {
		fn as_raw_Detail_SeamFinder(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfStitcher {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfStitcher {
		pub fn as_raw_PtrOfStitcher(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfStitcher_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfStitcher_get_inner_ptr(self.as_raw_PtrOfStitcher()) }
		}
	}
	
	impl Drop for PtrOfStitcher {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfStitcher_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfStitcher_delete(self.as_raw_PtrOfStitcher()) };
		}
	}
	
	unsafe impl Send for PtrOfStitcher {}
	
	impl crate::stitching::StitcherTrait for PtrOfStitcher {
		fn as_raw_Stitcher(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfWarperCreator {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfWarperCreator {
		pub fn as_raw_PtrOfWarperCreator(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfWarperCreator_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfWarperCreator_get_inner_ptr(self.as_raw_PtrOfWarperCreator()) }
		}
	}
	
	impl Drop for PtrOfWarperCreator {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfWarperCreator_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfWarperCreator_delete(self.as_raw_PtrOfWarperCreator()) };
		}
	}
	
	unsafe impl Send for PtrOfWarperCreator {}
	
	impl crate::stitching::WarperCreator for PtrOfWarperCreator {
		fn as_raw_WarperCreator(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct VectorOfDetail_CameraParams {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfDetail_CameraParams {
		pub fn as_raw_VectorOfDetail_CameraParams(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfDetail_CameraParams {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfDetail_CameraParams_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfDetail_CameraParams_delete(self.as_raw_VectorOfDetail_CameraParams()) };
		}
	}
	
	impl IntoIterator for VectorOfDetail_CameraParams {
		type Item = crate::stitching::Detail_CameraParams;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfDetail_CameraParams {
		type Item = crate::stitching::Detail_CameraParams;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfDetail_CameraParams>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfDetail_CameraParams {
		type Storage = crate::stitching::Detail_CameraParams;
		type Arg = crate::stitching::Detail_CameraParams;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfDetail_CameraParams_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfDetail_CameraParams_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfDetail_CameraParams_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfDetail_CameraParams_len(self.as_raw_VectorOfDetail_CameraParams()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfDetail_CameraParams_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfDetail_CameraParams_is_empty(self.as_raw_VectorOfDetail_CameraParams()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfDetail_CameraParams_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfDetail_CameraParams_capacity(self.as_raw_VectorOfDetail_CameraParams()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfDetail_CameraParams_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfDetail_CameraParams_shrink_to_fit(self.as_raw_VectorOfDetail_CameraParams()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfDetail_CameraParams_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfDetail_CameraParams_reserve(self.as_raw_VectorOfDetail_CameraParams(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfDetail_CameraParams_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfDetail_CameraParams_remove(self.as_raw_VectorOfDetail_CameraParams(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfDetail_CameraParams_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfDetail_CameraParams_swap(self.as_raw_VectorOfDetail_CameraParams(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfDetail_CameraParams_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfDetail_CameraParams_clear(self.as_raw_VectorOfDetail_CameraParams()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfDetail_CameraParams_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfDetail_CameraParams_push(self.as_raw_VectorOfDetail_CameraParams(), val.as_raw_Detail_CameraParams()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfDetail_CameraParams_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfDetail_CameraParams_insert(self.as_raw_VectorOfDetail_CameraParams(), index, val.as_raw_Detail_CameraParams()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfDetail_CameraParams_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfDetail_CameraParams_get(self.as_raw_VectorOfDetail_CameraParams(), index) }
				.into_result()
				.map(|ptr| crate::stitching::Detail_CameraParams { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfDetail_CameraParams_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfDetail_CameraParams_get_unchecked(self.as_raw_VectorOfDetail_CameraParams(), index)
				.into_result()
				.map(|ptr| crate::stitching::Detail_CameraParams { ptr })
				.expect("Infallible function failed: VectorOfDetail_CameraParams::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfDetail_CameraParams_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfDetail_CameraParams_set(self.as_raw_VectorOfDetail_CameraParams(), index, val.as_raw_Detail_CameraParams()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfDetail_CameraParams_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfDetail_CameraParams_set_unchecked(self.as_raw_VectorOfDetail_CameraParams(), index, val.as_raw_Detail_CameraParams())
		}
	
	}
	
	unsafe impl Send for VectorOfDetail_CameraParams {}
	
	pub struct VectorOfDetail_ImageFeatures {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfDetail_ImageFeatures {
		pub fn as_raw_VectorOfDetail_ImageFeatures(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfDetail_ImageFeatures {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfDetail_ImageFeatures_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfDetail_ImageFeatures_delete(self.as_raw_VectorOfDetail_ImageFeatures()) };
		}
	}
	
	impl IntoIterator for VectorOfDetail_ImageFeatures {
		type Item = crate::stitching::Detail_ImageFeatures;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfDetail_ImageFeatures {
		type Item = crate::stitching::Detail_ImageFeatures;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfDetail_ImageFeatures>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfDetail_ImageFeatures {
		type Storage = crate::stitching::Detail_ImageFeatures;
		type Arg = crate::stitching::Detail_ImageFeatures;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfDetail_ImageFeatures_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfDetail_ImageFeatures_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfDetail_ImageFeatures_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfDetail_ImageFeatures_len(self.as_raw_VectorOfDetail_ImageFeatures()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfDetail_ImageFeatures_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfDetail_ImageFeatures_is_empty(self.as_raw_VectorOfDetail_ImageFeatures()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfDetail_ImageFeatures_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfDetail_ImageFeatures_capacity(self.as_raw_VectorOfDetail_ImageFeatures()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfDetail_ImageFeatures_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfDetail_ImageFeatures_shrink_to_fit(self.as_raw_VectorOfDetail_ImageFeatures()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfDetail_ImageFeatures_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfDetail_ImageFeatures_reserve(self.as_raw_VectorOfDetail_ImageFeatures(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfDetail_ImageFeatures_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfDetail_ImageFeatures_remove(self.as_raw_VectorOfDetail_ImageFeatures(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfDetail_ImageFeatures_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfDetail_ImageFeatures_swap(self.as_raw_VectorOfDetail_ImageFeatures(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfDetail_ImageFeatures_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfDetail_ImageFeatures_clear(self.as_raw_VectorOfDetail_ImageFeatures()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfDetail_ImageFeatures_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfDetail_ImageFeatures_push(self.as_raw_VectorOfDetail_ImageFeatures(), val.as_raw_Detail_ImageFeatures()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfDetail_ImageFeatures_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfDetail_ImageFeatures_insert(self.as_raw_VectorOfDetail_ImageFeatures(), index, val.as_raw_Detail_ImageFeatures()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfDetail_ImageFeatures_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfDetail_ImageFeatures_get(self.as_raw_VectorOfDetail_ImageFeatures(), index) }
				.into_result()
				.map(|ptr| crate::stitching::Detail_ImageFeatures { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfDetail_ImageFeatures_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfDetail_ImageFeatures_get_unchecked(self.as_raw_VectorOfDetail_ImageFeatures(), index)
				.into_result()
				.map(|ptr| crate::stitching::Detail_ImageFeatures { ptr })
				.expect("Infallible function failed: VectorOfDetail_ImageFeatures::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfDetail_ImageFeatures_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfDetail_ImageFeatures_set(self.as_raw_VectorOfDetail_ImageFeatures(), index, val.as_raw_Detail_ImageFeatures()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfDetail_ImageFeatures_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfDetail_ImageFeatures_set_unchecked(self.as_raw_VectorOfDetail_ImageFeatures(), index, val.as_raw_Detail_ImageFeatures())
		}
	
	}
	
	unsafe impl Send for VectorOfDetail_ImageFeatures {}
	
	pub struct VectorOfDetail_MatchesInfo {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfDetail_MatchesInfo {
		pub fn as_raw_VectorOfDetail_MatchesInfo(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfDetail_MatchesInfo {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfDetail_MatchesInfo_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfDetail_MatchesInfo_delete(self.as_raw_VectorOfDetail_MatchesInfo()) };
		}
	}
	
	impl IntoIterator for VectorOfDetail_MatchesInfo {
		type Item = crate::stitching::Detail_MatchesInfo;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfDetail_MatchesInfo {
		type Item = crate::stitching::Detail_MatchesInfo;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfDetail_MatchesInfo>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfDetail_MatchesInfo {
		type Storage = crate::stitching::Detail_MatchesInfo;
		type Arg = crate::stitching::Detail_MatchesInfo;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfDetail_MatchesInfo_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfDetail_MatchesInfo_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfDetail_MatchesInfo_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfDetail_MatchesInfo_len(self.as_raw_VectorOfDetail_MatchesInfo()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfDetail_MatchesInfo_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfDetail_MatchesInfo_is_empty(self.as_raw_VectorOfDetail_MatchesInfo()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfDetail_MatchesInfo_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfDetail_MatchesInfo_capacity(self.as_raw_VectorOfDetail_MatchesInfo()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfDetail_MatchesInfo_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfDetail_MatchesInfo_shrink_to_fit(self.as_raw_VectorOfDetail_MatchesInfo()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfDetail_MatchesInfo_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfDetail_MatchesInfo_reserve(self.as_raw_VectorOfDetail_MatchesInfo(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfDetail_MatchesInfo_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfDetail_MatchesInfo_remove(self.as_raw_VectorOfDetail_MatchesInfo(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfDetail_MatchesInfo_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfDetail_MatchesInfo_swap(self.as_raw_VectorOfDetail_MatchesInfo(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfDetail_MatchesInfo_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfDetail_MatchesInfo_clear(self.as_raw_VectorOfDetail_MatchesInfo()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfDetail_MatchesInfo_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfDetail_MatchesInfo_push(self.as_raw_VectorOfDetail_MatchesInfo(), val.as_raw_Detail_MatchesInfo()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfDetail_MatchesInfo_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfDetail_MatchesInfo_insert(self.as_raw_VectorOfDetail_MatchesInfo(), index, val.as_raw_Detail_MatchesInfo()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfDetail_MatchesInfo_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfDetail_MatchesInfo_get(self.as_raw_VectorOfDetail_MatchesInfo(), index) }
				.into_result()
				.map(|ptr| crate::stitching::Detail_MatchesInfo { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfDetail_MatchesInfo_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfDetail_MatchesInfo_get_unchecked(self.as_raw_VectorOfDetail_MatchesInfo(), index)
				.into_result()
				.map(|ptr| crate::stitching::Detail_MatchesInfo { ptr })
				.expect("Infallible function failed: VectorOfDetail_MatchesInfo::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfDetail_MatchesInfo_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfDetail_MatchesInfo_set(self.as_raw_VectorOfDetail_MatchesInfo(), index, val.as_raw_Detail_MatchesInfo()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfDetail_MatchesInfo_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfDetail_MatchesInfo_set_unchecked(self.as_raw_VectorOfDetail_MatchesInfo(), index, val.as_raw_Detail_MatchesInfo())
		}
	
	}
	
	unsafe impl Send for VectorOfDetail_MatchesInfo {}
	
}
pub use stitching_types::*;

#[cfg(feature = "contrib")]
mod structured_light_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfGrayCodePattern {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfGrayCodePattern {
		pub fn as_raw_PtrOfGrayCodePattern(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfGrayCodePattern_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfGrayCodePattern_get_inner_ptr(self.as_raw_PtrOfGrayCodePattern()) }
		}
	}
	
	impl Drop for PtrOfGrayCodePattern {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfGrayCodePattern_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfGrayCodePattern_delete(self.as_raw_PtrOfGrayCodePattern()) };
		}
	}
	
	unsafe impl Send for PtrOfGrayCodePattern {}
	
	impl core::AlgorithmTrait for PtrOfGrayCodePattern {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::structured_light::GrayCodePattern for PtrOfGrayCodePattern {
		fn as_raw_GrayCodePattern(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::structured_light::StructuredLightPattern for PtrOfGrayCodePattern {
		fn as_raw_StructuredLightPattern(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfSinusoidalPattern {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfSinusoidalPattern {
		pub fn as_raw_PtrOfSinusoidalPattern(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfSinusoidalPattern_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfSinusoidalPattern_get_inner_ptr(self.as_raw_PtrOfSinusoidalPattern()) }
		}
	}
	
	impl Drop for PtrOfSinusoidalPattern {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfSinusoidalPattern_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfSinusoidalPattern_delete(self.as_raw_PtrOfSinusoidalPattern()) };
		}
	}
	
	unsafe impl Send for PtrOfSinusoidalPattern {}
	
	impl core::AlgorithmTrait for PtrOfSinusoidalPattern {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::structured_light::SinusoidalPattern for PtrOfSinusoidalPattern {
		fn as_raw_SinusoidalPattern(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::structured_light::StructuredLightPattern for PtrOfSinusoidalPattern {
		fn as_raw_StructuredLightPattern(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
}
#[cfg(feature = "contrib")]
pub use structured_light_types::*;

mod superres_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfSuperres_BroxOpticalFlow {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfSuperres_BroxOpticalFlow {
		pub fn as_raw_PtrOfSuperres_BroxOpticalFlow(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfSuperres_BroxOpticalFlow_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfSuperres_BroxOpticalFlow_get_inner_ptr(self.as_raw_PtrOfSuperres_BroxOpticalFlow()) }
		}
	}
	
	impl Drop for PtrOfSuperres_BroxOpticalFlow {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfSuperres_BroxOpticalFlow_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfSuperres_BroxOpticalFlow_delete(self.as_raw_PtrOfSuperres_BroxOpticalFlow()) };
		}
	}
	
	unsafe impl Send for PtrOfSuperres_BroxOpticalFlow {}
	
	impl core::AlgorithmTrait for PtrOfSuperres_BroxOpticalFlow {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::superres::Superres_BroxOpticalFlow for PtrOfSuperres_BroxOpticalFlow {
		fn as_raw_Superres_BroxOpticalFlow(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_BroxOpticalFlow {
		fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfSuperres_DenseOpticalFlowExt {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfSuperres_DenseOpticalFlowExt {
		pub fn as_raw_PtrOfSuperres_DenseOpticalFlowExt(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfSuperres_DenseOpticalFlowExt_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfSuperres_DenseOpticalFlowExt_get_inner_ptr(self.as_raw_PtrOfSuperres_DenseOpticalFlowExt()) }
		}
	}
	
	impl Drop for PtrOfSuperres_DenseOpticalFlowExt {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfSuperres_DenseOpticalFlowExt_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfSuperres_DenseOpticalFlowExt_delete(self.as_raw_PtrOfSuperres_DenseOpticalFlowExt()) };
		}
	}
	
	unsafe impl Send for PtrOfSuperres_DenseOpticalFlowExt {}
	
	impl core::AlgorithmTrait for PtrOfSuperres_DenseOpticalFlowExt {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_DenseOpticalFlowExt {
		fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfSuperres_DualTVL1OpticalFlow {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfSuperres_DualTVL1OpticalFlow {
		pub fn as_raw_PtrOfSuperres_DualTVL1OpticalFlow(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfSuperres_DualTVL1OpticalFlow_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfSuperres_DualTVL1OpticalFlow_get_inner_ptr(self.as_raw_PtrOfSuperres_DualTVL1OpticalFlow()) }
		}
	}
	
	impl Drop for PtrOfSuperres_DualTVL1OpticalFlow {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfSuperres_DualTVL1OpticalFlow_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfSuperres_DualTVL1OpticalFlow_delete(self.as_raw_PtrOfSuperres_DualTVL1OpticalFlow()) };
		}
	}
	
	unsafe impl Send for PtrOfSuperres_DualTVL1OpticalFlow {}
	
	impl core::AlgorithmTrait for PtrOfSuperres_DualTVL1OpticalFlow {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_DualTVL1OpticalFlow {
		fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::superres::Superres_DualTVL1OpticalFlow for PtrOfSuperres_DualTVL1OpticalFlow {
		fn as_raw_Superres_DualTVL1OpticalFlow(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfSuperres_FarnebackOpticalFlow {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfSuperres_FarnebackOpticalFlow {
		pub fn as_raw_PtrOfSuperres_FarnebackOpticalFlow(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfSuperres_FarnebackOpticalFlow_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfSuperres_FarnebackOpticalFlow_get_inner_ptr(self.as_raw_PtrOfSuperres_FarnebackOpticalFlow()) }
		}
	}
	
	impl Drop for PtrOfSuperres_FarnebackOpticalFlow {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfSuperres_FarnebackOpticalFlow_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfSuperres_FarnebackOpticalFlow_delete(self.as_raw_PtrOfSuperres_FarnebackOpticalFlow()) };
		}
	}
	
	unsafe impl Send for PtrOfSuperres_FarnebackOpticalFlow {}
	
	impl core::AlgorithmTrait for PtrOfSuperres_FarnebackOpticalFlow {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_FarnebackOpticalFlow {
		fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::superres::Superres_FarnebackOpticalFlow for PtrOfSuperres_FarnebackOpticalFlow {
		fn as_raw_Superres_FarnebackOpticalFlow(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfSuperres_FrameSource {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfSuperres_FrameSource {
		pub fn as_raw_PtrOfSuperres_FrameSource(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfSuperres_FrameSource_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfSuperres_FrameSource_get_inner_ptr(self.as_raw_PtrOfSuperres_FrameSource()) }
		}
	}
	
	impl Drop for PtrOfSuperres_FrameSource {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfSuperres_FrameSource_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfSuperres_FrameSource_delete(self.as_raw_PtrOfSuperres_FrameSource()) };
		}
	}
	
	unsafe impl Send for PtrOfSuperres_FrameSource {}
	
	impl crate::superres::Superres_FrameSource for PtrOfSuperres_FrameSource {
		fn as_raw_Superres_FrameSource(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfSuperres_PyrLKOpticalFlow {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfSuperres_PyrLKOpticalFlow {
		pub fn as_raw_PtrOfSuperres_PyrLKOpticalFlow(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfSuperres_PyrLKOpticalFlow_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfSuperres_PyrLKOpticalFlow_get_inner_ptr(self.as_raw_PtrOfSuperres_PyrLKOpticalFlow()) }
		}
	}
	
	impl Drop for PtrOfSuperres_PyrLKOpticalFlow {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfSuperres_PyrLKOpticalFlow_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfSuperres_PyrLKOpticalFlow_delete(self.as_raw_PtrOfSuperres_PyrLKOpticalFlow()) };
		}
	}
	
	unsafe impl Send for PtrOfSuperres_PyrLKOpticalFlow {}
	
	impl core::AlgorithmTrait for PtrOfSuperres_PyrLKOpticalFlow {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_PyrLKOpticalFlow {
		fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::superres::Superres_PyrLKOpticalFlow for PtrOfSuperres_PyrLKOpticalFlow {
		fn as_raw_Superres_PyrLKOpticalFlow(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfSuperres_SuperResolution {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfSuperres_SuperResolution {
		pub fn as_raw_PtrOfSuperres_SuperResolution(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfSuperres_SuperResolution_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfSuperres_SuperResolution_get_inner_ptr(self.as_raw_PtrOfSuperres_SuperResolution()) }
		}
	}
	
	impl Drop for PtrOfSuperres_SuperResolution {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfSuperres_SuperResolution_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfSuperres_SuperResolution_delete(self.as_raw_PtrOfSuperres_SuperResolution()) };
		}
	}
	
	unsafe impl Send for PtrOfSuperres_SuperResolution {}
	
	impl core::AlgorithmTrait for PtrOfSuperres_SuperResolution {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::superres::Superres_FrameSource for PtrOfSuperres_SuperResolution {
		fn as_raw_Superres_FrameSource(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::superres::Superres_SuperResolution for PtrOfSuperres_SuperResolution {
		fn as_raw_Superres_SuperResolution(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
}
pub use superres_types::*;

#[cfg(feature = "contrib")]
mod surface_matching_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfPose3D {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfPose3D {
		pub fn as_raw_PtrOfPose3D(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfPose3D_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfPose3D_get_inner_ptr(self.as_raw_PtrOfPose3D()) }
		}
	}
	
	impl Drop for PtrOfPose3D {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfPose3D_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfPose3D_delete(self.as_raw_PtrOfPose3D()) };
		}
	}
	
	unsafe impl Send for PtrOfPose3D {}
	
	impl crate::surface_matching::Pose3DTrait for PtrOfPose3D {
		fn as_raw_Pose3D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct VectorOfPose3DPtr {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfPose3DPtr {
		pub fn as_raw_VectorOfPose3DPtr(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfPose3DPtr {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfPose3DPtr_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfPose3DPtr_delete(self.as_raw_VectorOfPose3DPtr()) };
		}
	}
	
	impl IntoIterator for VectorOfPose3DPtr {
		type Item = crate::surface_matching::Pose3DPtr;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfPose3DPtr {
		type Item = crate::surface_matching::Pose3DPtr;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfPose3DPtr>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfPose3DPtr {
		type Storage = crate::surface_matching::Pose3DPtr;
		type Arg = crate::surface_matching::Pose3DPtr;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfPose3DPtr_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfPose3DPtr_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfPose3DPtr_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfPose3DPtr_len(self.as_raw_VectorOfPose3DPtr()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfPose3DPtr_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfPose3DPtr_is_empty(self.as_raw_VectorOfPose3DPtr()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfPose3DPtr_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfPose3DPtr_capacity(self.as_raw_VectorOfPose3DPtr()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfPose3DPtr_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfPose3DPtr_shrink_to_fit(self.as_raw_VectorOfPose3DPtr()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfPose3DPtr_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfPose3DPtr_reserve(self.as_raw_VectorOfPose3DPtr(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfPose3DPtr_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfPose3DPtr_remove(self.as_raw_VectorOfPose3DPtr(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfPose3DPtr_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfPose3DPtr_swap(self.as_raw_VectorOfPose3DPtr(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfPose3DPtr_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfPose3DPtr_clear(self.as_raw_VectorOfPose3DPtr()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfPose3DPtr_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfPose3DPtr_push(self.as_raw_VectorOfPose3DPtr(), val.as_raw_PtrOfPose3D()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfPose3DPtr_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfPose3DPtr_insert(self.as_raw_VectorOfPose3DPtr(), index, val.as_raw_PtrOfPose3D()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfPose3DPtr_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfPose3DPtr_get(self.as_raw_VectorOfPose3DPtr(), index) }
				.into_result()
				.map(|ptr| types::PtrOfPose3D { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfPose3DPtr_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfPose3DPtr_get_unchecked(self.as_raw_VectorOfPose3DPtr(), index)
				.into_result()
				.map(|ptr| types::PtrOfPose3D { ptr })
				.expect("Infallible function failed: VectorOfPose3DPtr::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfPose3DPtr_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfPose3DPtr_set(self.as_raw_VectorOfPose3DPtr(), index, val.as_raw_PtrOfPose3D()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfPose3DPtr_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfPose3DPtr_set_unchecked(self.as_raw_VectorOfPose3DPtr(), index, val.as_raw_PtrOfPose3D())
		}
	
	}
	
	unsafe impl Send for VectorOfPose3DPtr {}
	
}
#[cfg(feature = "contrib")]
pub use surface_matching_types::*;

#[cfg(feature = "contrib")]
mod text_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfERFilter {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfERFilter {
		pub fn as_raw_PtrOfERFilter(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfERFilter_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfERFilter_get_inner_ptr(self.as_raw_PtrOfERFilter()) }
		}
	}
	
	impl Drop for PtrOfERFilter {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfERFilter_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfERFilter_delete(self.as_raw_PtrOfERFilter()) };
		}
	}
	
	unsafe impl Send for PtrOfERFilter {}
	
	impl core::AlgorithmTrait for PtrOfERFilter {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::text::ERFilter for PtrOfERFilter {
		fn as_raw_ERFilter(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfERFilter_Callback {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfERFilter_Callback {
		pub fn as_raw_PtrOfERFilter_Callback(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfERFilter_Callback_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfERFilter_Callback_get_inner_ptr(self.as_raw_PtrOfERFilter_Callback()) }
		}
	}
	
	impl Drop for PtrOfERFilter_Callback {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfERFilter_Callback_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfERFilter_Callback_delete(self.as_raw_PtrOfERFilter_Callback()) };
		}
	}
	
	unsafe impl Send for PtrOfERFilter_Callback {}
	
	impl crate::text::ERFilter_Callback for PtrOfERFilter_Callback {
		fn as_raw_ERFilter_Callback(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfOCRBeamSearchDecoder {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfOCRBeamSearchDecoder {
		pub fn as_raw_PtrOfOCRBeamSearchDecoder(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfOCRBeamSearchDecoder_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfOCRBeamSearchDecoder_get_inner_ptr(self.as_raw_PtrOfOCRBeamSearchDecoder()) }
		}
	}
	
	impl Drop for PtrOfOCRBeamSearchDecoder {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfOCRBeamSearchDecoder_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfOCRBeamSearchDecoder_delete(self.as_raw_PtrOfOCRBeamSearchDecoder()) };
		}
	}
	
	unsafe impl Send for PtrOfOCRBeamSearchDecoder {}
	
	impl crate::text::BaseOCR for PtrOfOCRBeamSearchDecoder {
		fn as_raw_BaseOCR(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::text::OCRBeamSearchDecoderTrait for PtrOfOCRBeamSearchDecoder {
		fn as_raw_OCRBeamSearchDecoder(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfOCRBeamSearchDecoder_ClassifierCallback {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfOCRBeamSearchDecoder_ClassifierCallback {
		pub fn as_raw_PtrOfOCRBeamSearchDecoder_ClassifierCallback(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_get_inner_ptr(self.as_raw_PtrOfOCRBeamSearchDecoder_ClassifierCallback()) }
		}
	}
	
	impl Drop for PtrOfOCRBeamSearchDecoder_ClassifierCallback {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_delete(self.as_raw_PtrOfOCRBeamSearchDecoder_ClassifierCallback()) };
		}
	}
	
	unsafe impl Send for PtrOfOCRBeamSearchDecoder_ClassifierCallback {}
	
	impl crate::text::OCRBeamSearchDecoder_ClassifierCallbackTrait for PtrOfOCRBeamSearchDecoder_ClassifierCallback {
		fn as_raw_OCRBeamSearchDecoder_ClassifierCallback(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfOCRHMMDecoder {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfOCRHMMDecoder {
		pub fn as_raw_PtrOfOCRHMMDecoder(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfOCRHMMDecoder_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfOCRHMMDecoder_get_inner_ptr(self.as_raw_PtrOfOCRHMMDecoder()) }
		}
	}
	
	impl Drop for PtrOfOCRHMMDecoder {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfOCRHMMDecoder_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfOCRHMMDecoder_delete(self.as_raw_PtrOfOCRHMMDecoder()) };
		}
	}
	
	unsafe impl Send for PtrOfOCRHMMDecoder {}
	
	impl crate::text::BaseOCR for PtrOfOCRHMMDecoder {
		fn as_raw_BaseOCR(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::text::OCRHMMDecoderTrait for PtrOfOCRHMMDecoder {
		fn as_raw_OCRHMMDecoder(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfOCRHMMDecoder_ClassifierCallback {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfOCRHMMDecoder_ClassifierCallback {
		pub fn as_raw_PtrOfOCRHMMDecoder_ClassifierCallback(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfOCRHMMDecoder_ClassifierCallback_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfOCRHMMDecoder_ClassifierCallback_get_inner_ptr(self.as_raw_PtrOfOCRHMMDecoder_ClassifierCallback()) }
		}
	}
	
	impl Drop for PtrOfOCRHMMDecoder_ClassifierCallback {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfOCRHMMDecoder_ClassifierCallback_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfOCRHMMDecoder_ClassifierCallback_delete(self.as_raw_PtrOfOCRHMMDecoder_ClassifierCallback()) };
		}
	}
	
	unsafe impl Send for PtrOfOCRHMMDecoder_ClassifierCallback {}
	
	impl crate::text::OCRHMMDecoder_ClassifierCallbackTrait for PtrOfOCRHMMDecoder_ClassifierCallback {
		fn as_raw_OCRHMMDecoder_ClassifierCallback(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfOCRTesseract {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfOCRTesseract {
		pub fn as_raw_PtrOfOCRTesseract(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfOCRTesseract_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfOCRTesseract_get_inner_ptr(self.as_raw_PtrOfOCRTesseract()) }
		}
	}
	
	impl Drop for PtrOfOCRTesseract {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfOCRTesseract_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfOCRTesseract_delete(self.as_raw_PtrOfOCRTesseract()) };
		}
	}
	
	unsafe impl Send for PtrOfOCRTesseract {}
	
	impl crate::text::BaseOCR for PtrOfOCRTesseract {
		fn as_raw_BaseOCR(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::text::OCRTesseract for PtrOfOCRTesseract {
		fn as_raw_OCRTesseract(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct VectorOfERStat {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfERStat {
		pub fn as_raw_VectorOfERStat(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfERStat {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfERStat_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfERStat_delete(self.as_raw_VectorOfERStat()) };
		}
	}
	
	impl IntoIterator for VectorOfERStat {
		type Item = crate::text::ERStat;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfERStat {
		type Item = crate::text::ERStat;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfERStat>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfERStat {
		type Storage = crate::text::ERStat;
		type Arg = crate::text::ERStat;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfERStat_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfERStat_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfERStat_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfERStat_len(self.as_raw_VectorOfERStat()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfERStat_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfERStat_is_empty(self.as_raw_VectorOfERStat()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfERStat_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfERStat_capacity(self.as_raw_VectorOfERStat()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfERStat_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfERStat_shrink_to_fit(self.as_raw_VectorOfERStat()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfERStat_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfERStat_reserve(self.as_raw_VectorOfERStat(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfERStat_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfERStat_remove(self.as_raw_VectorOfERStat(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfERStat_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfERStat_swap(self.as_raw_VectorOfERStat(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfERStat_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfERStat_clear(self.as_raw_VectorOfERStat()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfERStat_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfERStat_push(self.as_raw_VectorOfERStat(), val.as_raw_ERStat()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfERStat_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfERStat_insert(self.as_raw_VectorOfERStat(), index, val.as_raw_ERStat()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfERStat_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfERStat_get(self.as_raw_VectorOfERStat(), index) }
				.into_result()
				.map(|ptr| crate::text::ERStat { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfERStat_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfERStat_get_unchecked(self.as_raw_VectorOfERStat(), index)
				.into_result()
				.map(|ptr| crate::text::ERStat { ptr })
				.expect("Infallible function failed: VectorOfERStat::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfERStat_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfERStat_set(self.as_raw_VectorOfERStat(), index, val.as_raw_ERStat()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfERStat_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfERStat_set_unchecked(self.as_raw_VectorOfERStat(), index, val.as_raw_ERStat())
		}
	
	}
	
	unsafe impl Send for VectorOfERStat {}
	
	pub struct VectorOfVectorOfERStat {
		pub(crate) ptr: *mut c_void
	}
	
	impl VectorOfVectorOfERStat {
		pub fn as_raw_VectorOfVectorOfERStat(&self) -> *mut c_void { self.ptr }
	
		#[inline]
		pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
			crate::templ::VectorRefIterator::new(self)
		}
	}
	
	impl Drop for VectorOfVectorOfERStat {
		#[inline]
		fn drop(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfERStat_delete(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfERStat_delete(self.as_raw_VectorOfVectorOfERStat()) };
		}
	}
	
	impl IntoIterator for VectorOfVectorOfERStat {
		type Item = types::VectorOfERStat;
		type IntoIter = crate::templ::VectorIterator<Self>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self)
		}
	}
	
	impl<'i> IntoIterator for &'i VectorOfVectorOfERStat {
		type Item = types::VectorOfERStat;
		type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfERStat>;
	
		#[inline]
		fn into_iter(self) -> Self::IntoIter {
			self.iter()
		}
	}
	
	impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfERStat {
		type Storage = types::VectorOfERStat;
		type Arg = types::VectorOfERStat;
	
		#[inline]
		fn new() -> Self {
			extern "C" { fn cv_VectorOfVectorOfERStat_new() -> *mut c_void; }
			Self { ptr: unsafe { cv_VectorOfVectorOfERStat_new() } }
		}
	
		#[inline]
		fn len(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfERStat_len(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfERStat_len(self.as_raw_VectorOfVectorOfERStat()) }
		}
	
		#[inline]
		fn is_empty(&self) -> bool {
			extern "C" { fn cv_VectorOfVectorOfERStat_is_empty(instance: *mut c_void) -> bool; }
			unsafe { cv_VectorOfVectorOfERStat_is_empty(self.as_raw_VectorOfVectorOfERStat()) }
		}
	
		#[inline]
		fn capacity(&self) -> size_t {
			extern "C" { fn cv_VectorOfVectorOfERStat_capacity(instance: *mut c_void) -> size_t; }
			unsafe { cv_VectorOfVectorOfERStat_capacity(self.as_raw_VectorOfVectorOfERStat()) }
		}
	
		#[inline]
		fn shrink_to_fit(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfERStat_shrink_to_fit(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfERStat_shrink_to_fit(self.as_raw_VectorOfVectorOfERStat()) }
		}
	
		#[inline]
		fn reserve(&mut self, additional: size_t) {
			extern "C" { fn cv_VectorOfVectorOfERStat_reserve(instance: *mut c_void, additional: size_t); }
			unsafe { cv_VectorOfVectorOfERStat_reserve(self.as_raw_VectorOfVectorOfERStat(), additional) }
		}
	
		#[inline]
		fn remove(&mut self, index: size_t) -> Result<()> {
			crate::templ::vector_index_check(index, self.len())?;
			extern "C" { fn cv_VectorOfVectorOfERStat_remove(instance: *mut c_void, index: size_t); }
			unsafe { cv_VectorOfVectorOfERStat_remove(self.as_raw_VectorOfVectorOfERStat(), index) };
			Ok(())
		}
	
		#[inline]
		fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
			let len = self.len();
			crate::templ::vector_index_check(index1, len)?;
			crate::templ::vector_index_check(index2, len)?;
			if index1 != index2 {
				extern "C" { fn cv_VectorOfVectorOfERStat_swap(instance: *mut c_void, index1: size_t, index2: size_t); }
				unsafe { cv_VectorOfVectorOfERStat_swap(self.as_raw_VectorOfVectorOfERStat(), index1, index2) };
			}
			Ok(())
		}
	
		#[inline]
		fn clear(&mut self) {
			extern "C" { fn cv_VectorOfVectorOfERStat_clear(instance: *mut c_void); }
			unsafe { cv_VectorOfVectorOfERStat_clear(self.as_raw_VectorOfVectorOfERStat()) }
		}
	
		#[inline]
		fn push(&mut self, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfERStat_push(instance: *mut c_void, val: *mut c_void); }
			unsafe { cv_VectorOfVectorOfERStat_push(self.as_raw_VectorOfVectorOfERStat(), val.as_raw_VectorOfERStat()) }
		}
	
		#[inline]
		fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfERStat_insert(instance: *mut c_void, index: size_t, val: *mut c_void); }
			crate::templ::vector_index_check(index, self.len() + 1)?;
			unsafe { cv_VectorOfVectorOfERStat_insert(self.as_raw_VectorOfVectorOfERStat(), index, val.as_raw_VectorOfERStat()) }
			Ok(())
		}
	
		#[inline]
		fn get(&self, index: size_t) -> Result<Self::Storage> {
			extern "C" { fn cv_VectorOfVectorOfERStat_get(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			unsafe { cv_VectorOfVectorOfERStat_get(self.as_raw_VectorOfVectorOfERStat(), index) }
				.into_result()
				.map(|ptr| types::VectorOfERStat { ptr })
		}
	
		#[inline]
		unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
			extern "C" { fn cv_VectorOfVectorOfERStat_get_unchecked(instance: *mut c_void, index: size_t) -> sys::Result<*mut c_void>; }
			cv_VectorOfVectorOfERStat_get_unchecked(self.as_raw_VectorOfVectorOfERStat(), index)
				.into_result()
				.map(|ptr| types::VectorOfERStat { ptr })
				.expect("Infallible function failed: VectorOfVectorOfERStat::get_unchecked")
		}
	
		#[inline]
		fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
			extern "C" { fn cv_VectorOfVectorOfERStat_set(instance: *mut c_void, index: size_t, val: *mut c_void) -> sys::Result_void; }
			unsafe { cv_VectorOfVectorOfERStat_set(self.as_raw_VectorOfVectorOfERStat(), index, val.as_raw_VectorOfERStat()) }
				.into_result()
		}
	
		#[inline]
		unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
			extern "C" { fn cv_VectorOfVectorOfERStat_set_unchecked(instance: *mut c_void, index: size_t, val: *mut c_void); }
			cv_VectorOfVectorOfERStat_set_unchecked(self.as_raw_VectorOfVectorOfERStat(), index, val.as_raw_VectorOfERStat())
		}
	
	}
	
	unsafe impl Send for VectorOfVectorOfERStat {}
	
}
#[cfg(feature = "contrib")]
pub use text_types::*;

mod video_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfBackgroundSubtractorKNN {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfBackgroundSubtractorKNN {
		pub fn as_raw_PtrOfBackgroundSubtractorKNN(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfBackgroundSubtractorKNN_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfBackgroundSubtractorKNN_get_inner_ptr(self.as_raw_PtrOfBackgroundSubtractorKNN()) }
		}
	}
	
	impl Drop for PtrOfBackgroundSubtractorKNN {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfBackgroundSubtractorKNN_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfBackgroundSubtractorKNN_delete(self.as_raw_PtrOfBackgroundSubtractorKNN()) };
		}
	}
	
	unsafe impl Send for PtrOfBackgroundSubtractorKNN {}
	
	impl core::AlgorithmTrait for PtrOfBackgroundSubtractorKNN {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorKNN {
		fn as_raw_BackgroundSubtractor(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::video::BackgroundSubtractorKNN for PtrOfBackgroundSubtractorKNN {
		fn as_raw_BackgroundSubtractorKNN(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfBackgroundSubtractorMOG2 {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfBackgroundSubtractorMOG2 {
		pub fn as_raw_PtrOfBackgroundSubtractorMOG2(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfBackgroundSubtractorMOG2_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfBackgroundSubtractorMOG2_get_inner_ptr(self.as_raw_PtrOfBackgroundSubtractorMOG2()) }
		}
	}
	
	impl Drop for PtrOfBackgroundSubtractorMOG2 {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfBackgroundSubtractorMOG2_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfBackgroundSubtractorMOG2_delete(self.as_raw_PtrOfBackgroundSubtractorMOG2()) };
		}
	}
	
	unsafe impl Send for PtrOfBackgroundSubtractorMOG2 {}
	
	impl core::AlgorithmTrait for PtrOfBackgroundSubtractorMOG2 {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorMOG2 {
		fn as_raw_BackgroundSubtractor(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::video::BackgroundSubtractorMOG2 for PtrOfBackgroundSubtractorMOG2 {
		fn as_raw_BackgroundSubtractorMOG2(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfDualTVL1OpticalFlow {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfDualTVL1OpticalFlow {
		pub fn as_raw_PtrOfDualTVL1OpticalFlow(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfDualTVL1OpticalFlow_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfDualTVL1OpticalFlow_get_inner_ptr(self.as_raw_PtrOfDualTVL1OpticalFlow()) }
		}
	}
	
	impl Drop for PtrOfDualTVL1OpticalFlow {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfDualTVL1OpticalFlow_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfDualTVL1OpticalFlow_delete(self.as_raw_PtrOfDualTVL1OpticalFlow()) };
		}
	}
	
	unsafe impl Send for PtrOfDualTVL1OpticalFlow {}
	
	impl core::AlgorithmTrait for PtrOfDualTVL1OpticalFlow {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::video::DenseOpticalFlow for PtrOfDualTVL1OpticalFlow {
		fn as_raw_DenseOpticalFlow(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::video::DualTVL1OpticalFlow for PtrOfDualTVL1OpticalFlow {
		fn as_raw_DualTVL1OpticalFlow(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfFarnebackOpticalFlow {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfFarnebackOpticalFlow {
		pub fn as_raw_PtrOfFarnebackOpticalFlow(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfFarnebackOpticalFlow_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfFarnebackOpticalFlow_get_inner_ptr(self.as_raw_PtrOfFarnebackOpticalFlow()) }
		}
	}
	
	impl Drop for PtrOfFarnebackOpticalFlow {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfFarnebackOpticalFlow_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfFarnebackOpticalFlow_delete(self.as_raw_PtrOfFarnebackOpticalFlow()) };
		}
	}
	
	unsafe impl Send for PtrOfFarnebackOpticalFlow {}
	
	impl core::AlgorithmTrait for PtrOfFarnebackOpticalFlow {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::video::DenseOpticalFlow for PtrOfFarnebackOpticalFlow {
		fn as_raw_DenseOpticalFlow(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::video::FarnebackOpticalFlow for PtrOfFarnebackOpticalFlow {
		fn as_raw_FarnebackOpticalFlow(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfSparsePyrLKOpticalFlow {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfSparsePyrLKOpticalFlow {
		pub fn as_raw_PtrOfSparsePyrLKOpticalFlow(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfSparsePyrLKOpticalFlow_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfSparsePyrLKOpticalFlow_get_inner_ptr(self.as_raw_PtrOfSparsePyrLKOpticalFlow()) }
		}
	}
	
	impl Drop for PtrOfSparsePyrLKOpticalFlow {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfSparsePyrLKOpticalFlow_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfSparsePyrLKOpticalFlow_delete(self.as_raw_PtrOfSparsePyrLKOpticalFlow()) };
		}
	}
	
	unsafe impl Send for PtrOfSparsePyrLKOpticalFlow {}
	
	impl core::AlgorithmTrait for PtrOfSparsePyrLKOpticalFlow {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::video::SparseOpticalFlow for PtrOfSparsePyrLKOpticalFlow {
		fn as_raw_SparseOpticalFlow(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::video::SparsePyrLKOpticalFlow for PtrOfSparsePyrLKOpticalFlow {
		fn as_raw_SparsePyrLKOpticalFlow(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
}
pub use video_types::*;

mod videostab_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfDeblurerBase {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfDeblurerBase {
		pub fn as_raw_PtrOfDeblurerBase(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfDeblurerBase_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfDeblurerBase_get_inner_ptr(self.as_raw_PtrOfDeblurerBase()) }
		}
	}
	
	impl Drop for PtrOfDeblurerBase {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfDeblurerBase_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfDeblurerBase_delete(self.as_raw_PtrOfDeblurerBase()) };
		}
	}
	
	unsafe impl Send for PtrOfDeblurerBase {}
	
	impl crate::videostab::DeblurerBase for PtrOfDeblurerBase {
		fn as_raw_DeblurerBase(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfIDenseOptFlowEstimator {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfIDenseOptFlowEstimator {
		pub fn as_raw_PtrOfIDenseOptFlowEstimator(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfIDenseOptFlowEstimator_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfIDenseOptFlowEstimator_get_inner_ptr(self.as_raw_PtrOfIDenseOptFlowEstimator()) }
		}
	}
	
	impl Drop for PtrOfIDenseOptFlowEstimator {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfIDenseOptFlowEstimator_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfIDenseOptFlowEstimator_delete(self.as_raw_PtrOfIDenseOptFlowEstimator()) };
		}
	}
	
	unsafe impl Send for PtrOfIDenseOptFlowEstimator {}
	
	impl crate::videostab::IDenseOptFlowEstimator for PtrOfIDenseOptFlowEstimator {
		fn as_raw_IDenseOptFlowEstimator(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfIFrameSource {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfIFrameSource {
		pub fn as_raw_PtrOfIFrameSource(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfIFrameSource_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfIFrameSource_get_inner_ptr(self.as_raw_PtrOfIFrameSource()) }
		}
	}
	
	impl Drop for PtrOfIFrameSource {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfIFrameSource_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfIFrameSource_delete(self.as_raw_PtrOfIFrameSource()) };
		}
	}
	
	unsafe impl Send for PtrOfIFrameSource {}
	
	impl crate::videostab::IFrameSource for PtrOfIFrameSource {
		fn as_raw_IFrameSource(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfILog {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfILog {
		pub fn as_raw_PtrOfILog(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfILog_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfILog_get_inner_ptr(self.as_raw_PtrOfILog()) }
		}
	}
	
	impl Drop for PtrOfILog {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfILog_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfILog_delete(self.as_raw_PtrOfILog()) };
		}
	}
	
	unsafe impl Send for PtrOfILog {}
	
	impl crate::videostab::ILog for PtrOfILog {
		fn as_raw_ILog(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfIMotionStabilizer {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfIMotionStabilizer {
		pub fn as_raw_PtrOfIMotionStabilizer(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfIMotionStabilizer_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfIMotionStabilizer_get_inner_ptr(self.as_raw_PtrOfIMotionStabilizer()) }
		}
	}
	
	impl Drop for PtrOfIMotionStabilizer {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfIMotionStabilizer_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfIMotionStabilizer_delete(self.as_raw_PtrOfIMotionStabilizer()) };
		}
	}
	
	unsafe impl Send for PtrOfIMotionStabilizer {}
	
	impl crate::videostab::IMotionStabilizer for PtrOfIMotionStabilizer {
		fn as_raw_IMotionStabilizer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfIOutlierRejector {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfIOutlierRejector {
		pub fn as_raw_PtrOfIOutlierRejector(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfIOutlierRejector_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfIOutlierRejector_get_inner_ptr(self.as_raw_PtrOfIOutlierRejector()) }
		}
	}
	
	impl Drop for PtrOfIOutlierRejector {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfIOutlierRejector_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfIOutlierRejector_delete(self.as_raw_PtrOfIOutlierRejector()) };
		}
	}
	
	unsafe impl Send for PtrOfIOutlierRejector {}
	
	impl crate::videostab::IOutlierRejector for PtrOfIOutlierRejector {
		fn as_raw_IOutlierRejector(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfISparseOptFlowEstimator {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfISparseOptFlowEstimator {
		pub fn as_raw_PtrOfISparseOptFlowEstimator(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfISparseOptFlowEstimator_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfISparseOptFlowEstimator_get_inner_ptr(self.as_raw_PtrOfISparseOptFlowEstimator()) }
		}
	}
	
	impl Drop for PtrOfISparseOptFlowEstimator {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfISparseOptFlowEstimator_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfISparseOptFlowEstimator_delete(self.as_raw_PtrOfISparseOptFlowEstimator()) };
		}
	}
	
	unsafe impl Send for PtrOfISparseOptFlowEstimator {}
	
	impl crate::videostab::ISparseOptFlowEstimator for PtrOfISparseOptFlowEstimator {
		fn as_raw_ISparseOptFlowEstimator(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfImageMotionEstimatorBase {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfImageMotionEstimatorBase {
		pub fn as_raw_PtrOfImageMotionEstimatorBase(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfImageMotionEstimatorBase_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfImageMotionEstimatorBase_get_inner_ptr(self.as_raw_PtrOfImageMotionEstimatorBase()) }
		}
	}
	
	impl Drop for PtrOfImageMotionEstimatorBase {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfImageMotionEstimatorBase_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfImageMotionEstimatorBase_delete(self.as_raw_PtrOfImageMotionEstimatorBase()) };
		}
	}
	
	unsafe impl Send for PtrOfImageMotionEstimatorBase {}
	
	impl crate::videostab::ImageMotionEstimatorBase for PtrOfImageMotionEstimatorBase {
		fn as_raw_ImageMotionEstimatorBase(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfInpainterBase {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfInpainterBase {
		pub fn as_raw_PtrOfInpainterBase(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfInpainterBase_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfInpainterBase_get_inner_ptr(self.as_raw_PtrOfInpainterBase()) }
		}
	}
	
	impl Drop for PtrOfInpainterBase {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfInpainterBase_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfInpainterBase_delete(self.as_raw_PtrOfInpainterBase()) };
		}
	}
	
	unsafe impl Send for PtrOfInpainterBase {}
	
	impl crate::videostab::InpainterBase for PtrOfInpainterBase {
		fn as_raw_InpainterBase(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfMotionEstimatorBase {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfMotionEstimatorBase {
		pub fn as_raw_PtrOfMotionEstimatorBase(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfMotionEstimatorBase_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfMotionEstimatorBase_get_inner_ptr(self.as_raw_PtrOfMotionEstimatorBase()) }
		}
	}
	
	impl Drop for PtrOfMotionEstimatorBase {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfMotionEstimatorBase_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfMotionEstimatorBase_delete(self.as_raw_PtrOfMotionEstimatorBase()) };
		}
	}
	
	unsafe impl Send for PtrOfMotionEstimatorBase {}
	
	impl crate::videostab::MotionEstimatorBase for PtrOfMotionEstimatorBase {
		fn as_raw_MotionEstimatorBase(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfMotionFilterBase {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfMotionFilterBase {
		pub fn as_raw_PtrOfMotionFilterBase(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfMotionFilterBase_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfMotionFilterBase_get_inner_ptr(self.as_raw_PtrOfMotionFilterBase()) }
		}
	}
	
	impl Drop for PtrOfMotionFilterBase {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfMotionFilterBase_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfMotionFilterBase_delete(self.as_raw_PtrOfMotionFilterBase()) };
		}
	}
	
	unsafe impl Send for PtrOfMotionFilterBase {}
	
	impl crate::videostab::IMotionStabilizer for PtrOfMotionFilterBase {
		fn as_raw_IMotionStabilizer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::videostab::MotionFilterBase for PtrOfMotionFilterBase {
		fn as_raw_MotionFilterBase(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfWobbleSuppressorBase {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfWobbleSuppressorBase {
		pub fn as_raw_PtrOfWobbleSuppressorBase(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfWobbleSuppressorBase_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfWobbleSuppressorBase_get_inner_ptr(self.as_raw_PtrOfWobbleSuppressorBase()) }
		}
	}
	
	impl Drop for PtrOfWobbleSuppressorBase {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfWobbleSuppressorBase_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfWobbleSuppressorBase_delete(self.as_raw_PtrOfWobbleSuppressorBase()) };
		}
	}
	
	unsafe impl Send for PtrOfWobbleSuppressorBase {}
	
	impl crate::videostab::WobbleSuppressorBase for PtrOfWobbleSuppressorBase {
		fn as_raw_WobbleSuppressorBase(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
}
pub use videostab_types::*;

#[cfg(feature = "contrib")]
mod xfeatures2d_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfBoostDesc {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfBoostDesc {
		pub fn as_raw_PtrOfBoostDesc(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfBoostDesc_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfBoostDesc_get_inner_ptr(self.as_raw_PtrOfBoostDesc()) }
		}
	}
	
	impl Drop for PtrOfBoostDesc {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfBoostDesc_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfBoostDesc_delete(self.as_raw_PtrOfBoostDesc()) };
		}
	}
	
	unsafe impl Send for PtrOfBoostDesc {}
	
	impl core::AlgorithmTrait for PtrOfBoostDesc {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::xfeatures2d::BoostDescTrait for PtrOfBoostDesc {
		fn as_raw_BoostDesc(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfBoostDesc {
		fn as_raw_Feature2D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfBriefDescriptorExtractor {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfBriefDescriptorExtractor {
		pub fn as_raw_PtrOfBriefDescriptorExtractor(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfBriefDescriptorExtractor_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfBriefDescriptorExtractor_get_inner_ptr(self.as_raw_PtrOfBriefDescriptorExtractor()) }
		}
	}
	
	impl Drop for PtrOfBriefDescriptorExtractor {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfBriefDescriptorExtractor_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfBriefDescriptorExtractor_delete(self.as_raw_PtrOfBriefDescriptorExtractor()) };
		}
	}
	
	unsafe impl Send for PtrOfBriefDescriptorExtractor {}
	
	impl core::AlgorithmTrait for PtrOfBriefDescriptorExtractor {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::xfeatures2d::BriefDescriptorExtractorTrait for PtrOfBriefDescriptorExtractor {
		fn as_raw_BriefDescriptorExtractor(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfBriefDescriptorExtractor {
		fn as_raw_Feature2D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfDAISY {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfDAISY {
		pub fn as_raw_PtrOfDAISY(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfDAISY_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfDAISY_get_inner_ptr(self.as_raw_PtrOfDAISY()) }
		}
	}
	
	impl Drop for PtrOfDAISY {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfDAISY_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfDAISY_delete(self.as_raw_PtrOfDAISY()) };
		}
	}
	
	unsafe impl Send for PtrOfDAISY {}
	
	impl core::AlgorithmTrait for PtrOfDAISY {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::xfeatures2d::DAISY for PtrOfDAISY {
		fn as_raw_DAISY(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfDAISY {
		fn as_raw_Feature2D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfFREAK {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfFREAK {
		pub fn as_raw_PtrOfFREAK(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfFREAK_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfFREAK_get_inner_ptr(self.as_raw_PtrOfFREAK()) }
		}
	}
	
	impl Drop for PtrOfFREAK {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfFREAK_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfFREAK_delete(self.as_raw_PtrOfFREAK()) };
		}
	}
	
	unsafe impl Send for PtrOfFREAK {}
	
	impl core::AlgorithmTrait for PtrOfFREAK {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::xfeatures2d::FREAKTrait for PtrOfFREAK {
		fn as_raw_FREAK(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfFREAK {
		fn as_raw_Feature2D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfLATCH {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfLATCH {
		pub fn as_raw_PtrOfLATCH(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfLATCH_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfLATCH_get_inner_ptr(self.as_raw_PtrOfLATCH()) }
		}
	}
	
	impl Drop for PtrOfLATCH {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfLATCH_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfLATCH_delete(self.as_raw_PtrOfLATCH()) };
		}
	}
	
	unsafe impl Send for PtrOfLATCH {}
	
	impl core::AlgorithmTrait for PtrOfLATCH {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfLATCH {
		fn as_raw_Feature2D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::xfeatures2d::LATCHTrait for PtrOfLATCH {
		fn as_raw_LATCH(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfLUCID {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfLUCID {
		pub fn as_raw_PtrOfLUCID(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfLUCID_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfLUCID_get_inner_ptr(self.as_raw_PtrOfLUCID()) }
		}
	}
	
	impl Drop for PtrOfLUCID {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfLUCID_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfLUCID_delete(self.as_raw_PtrOfLUCID()) };
		}
	}
	
	unsafe impl Send for PtrOfLUCID {}
	
	impl core::AlgorithmTrait for PtrOfLUCID {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfLUCID {
		fn as_raw_Feature2D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::xfeatures2d::LUCIDTrait for PtrOfLUCID {
		fn as_raw_LUCID(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfMSDDetector {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfMSDDetector {
		pub fn as_raw_PtrOfMSDDetector(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfMSDDetector_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfMSDDetector_get_inner_ptr(self.as_raw_PtrOfMSDDetector()) }
		}
	}
	
	impl Drop for PtrOfMSDDetector {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfMSDDetector_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfMSDDetector_delete(self.as_raw_PtrOfMSDDetector()) };
		}
	}
	
	unsafe impl Send for PtrOfMSDDetector {}
	
	impl core::AlgorithmTrait for PtrOfMSDDetector {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfMSDDetector {
		fn as_raw_Feature2D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::xfeatures2d::MSDDetectorTrait for PtrOfMSDDetector {
		fn as_raw_MSDDetector(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfPCTSignatures {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfPCTSignatures {
		pub fn as_raw_PtrOfPCTSignatures(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfPCTSignatures_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfPCTSignatures_get_inner_ptr(self.as_raw_PtrOfPCTSignatures()) }
		}
	}
	
	impl Drop for PtrOfPCTSignatures {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfPCTSignatures_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfPCTSignatures_delete(self.as_raw_PtrOfPCTSignatures()) };
		}
	}
	
	unsafe impl Send for PtrOfPCTSignatures {}
	
	impl core::AlgorithmTrait for PtrOfPCTSignatures {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::xfeatures2d::PCTSignatures for PtrOfPCTSignatures {
		fn as_raw_PCTSignatures(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfPCTSignaturesSQFD {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfPCTSignaturesSQFD {
		pub fn as_raw_PtrOfPCTSignaturesSQFD(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfPCTSignaturesSQFD_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfPCTSignaturesSQFD_get_inner_ptr(self.as_raw_PtrOfPCTSignaturesSQFD()) }
		}
	}
	
	impl Drop for PtrOfPCTSignaturesSQFD {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfPCTSignaturesSQFD_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfPCTSignaturesSQFD_delete(self.as_raw_PtrOfPCTSignaturesSQFD()) };
		}
	}
	
	unsafe impl Send for PtrOfPCTSignaturesSQFD {}
	
	impl core::AlgorithmTrait for PtrOfPCTSignaturesSQFD {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::xfeatures2d::PCTSignaturesSQFD for PtrOfPCTSignaturesSQFD {
		fn as_raw_PCTSignaturesSQFD(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfSIFT {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfSIFT {
		pub fn as_raw_PtrOfSIFT(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfSIFT_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfSIFT_get_inner_ptr(self.as_raw_PtrOfSIFT()) }
		}
	}
	
	impl Drop for PtrOfSIFT {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfSIFT_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfSIFT_delete(self.as_raw_PtrOfSIFT()) };
		}
	}
	
	unsafe impl Send for PtrOfSIFT {}
	
	impl core::AlgorithmTrait for PtrOfSIFT {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfSIFT {
		fn as_raw_Feature2D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::xfeatures2d::SIFTTrait for PtrOfSIFT {
		fn as_raw_SIFT(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfSURF {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfSURF {
		pub fn as_raw_PtrOfSURF(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfSURF_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfSURF_get_inner_ptr(self.as_raw_PtrOfSURF()) }
		}
	}
	
	impl Drop for PtrOfSURF {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfSURF_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfSURF_delete(self.as_raw_PtrOfSURF()) };
		}
	}
	
	unsafe impl Send for PtrOfSURF {}
	
	impl core::AlgorithmTrait for PtrOfSURF {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfSURF {
		fn as_raw_Feature2D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::xfeatures2d::SURF for PtrOfSURF {
		fn as_raw_SURF(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfStarDetector {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfStarDetector {
		pub fn as_raw_PtrOfStarDetector(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfStarDetector_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfStarDetector_get_inner_ptr(self.as_raw_PtrOfStarDetector()) }
		}
	}
	
	impl Drop for PtrOfStarDetector {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfStarDetector_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfStarDetector_delete(self.as_raw_PtrOfStarDetector()) };
		}
	}
	
	unsafe impl Send for PtrOfStarDetector {}
	
	impl core::AlgorithmTrait for PtrOfStarDetector {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfStarDetector {
		fn as_raw_Feature2D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::xfeatures2d::StarDetectorTrait for PtrOfStarDetector {
		fn as_raw_StarDetector(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfVGG {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfVGG {
		pub fn as_raw_PtrOfVGG(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfVGG_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfVGG_get_inner_ptr(self.as_raw_PtrOfVGG()) }
		}
	}
	
	impl Drop for PtrOfVGG {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfVGG_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfVGG_delete(self.as_raw_PtrOfVGG()) };
		}
	}
	
	unsafe impl Send for PtrOfVGG {}
	
	impl core::AlgorithmTrait for PtrOfVGG {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfVGG {
		fn as_raw_Feature2D(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::xfeatures2d::VGG for PtrOfVGG {
		fn as_raw_VGG(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
}
#[cfg(feature = "contrib")]
pub use xfeatures2d_types::*;

#[cfg(feature = "contrib")]
mod xobjdetect_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfWBDetector {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfWBDetector {
		pub fn as_raw_PtrOfWBDetector(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfWBDetector_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfWBDetector_get_inner_ptr(self.as_raw_PtrOfWBDetector()) }
		}
	}
	
	impl Drop for PtrOfWBDetector {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfWBDetector_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfWBDetector_delete(self.as_raw_PtrOfWBDetector()) };
		}
	}
	
	unsafe impl Send for PtrOfWBDetector {}
	
	impl crate::xobjdetect::WBDetector for PtrOfWBDetector {
		fn as_raw_WBDetector(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
}
#[cfg(feature = "contrib")]
pub use xobjdetect_types::*;

#[cfg(feature = "contrib")]
mod xphoto_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub struct PtrOfGrayworldWB {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfGrayworldWB {
		pub fn as_raw_PtrOfGrayworldWB(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfGrayworldWB_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfGrayworldWB_get_inner_ptr(self.as_raw_PtrOfGrayworldWB()) }
		}
	}
	
	impl Drop for PtrOfGrayworldWB {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfGrayworldWB_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfGrayworldWB_delete(self.as_raw_PtrOfGrayworldWB()) };
		}
	}
	
	unsafe impl Send for PtrOfGrayworldWB {}
	
	impl core::AlgorithmTrait for PtrOfGrayworldWB {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::xphoto::GrayworldWB for PtrOfGrayworldWB {
		fn as_raw_GrayworldWB(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::xphoto::WhiteBalancer for PtrOfGrayworldWB {
		fn as_raw_WhiteBalancer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfLearningBasedWB {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfLearningBasedWB {
		pub fn as_raw_PtrOfLearningBasedWB(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfLearningBasedWB_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfLearningBasedWB_get_inner_ptr(self.as_raw_PtrOfLearningBasedWB()) }
		}
	}
	
	impl Drop for PtrOfLearningBasedWB {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfLearningBasedWB_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfLearningBasedWB_delete(self.as_raw_PtrOfLearningBasedWB()) };
		}
	}
	
	unsafe impl Send for PtrOfLearningBasedWB {}
	
	impl core::AlgorithmTrait for PtrOfLearningBasedWB {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::xphoto::LearningBasedWB for PtrOfLearningBasedWB {
		fn as_raw_LearningBasedWB(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::xphoto::WhiteBalancer for PtrOfLearningBasedWB {
		fn as_raw_WhiteBalancer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	pub struct PtrOfSimpleWB {
		pub(crate) ptr: *mut c_void
	}
	
	impl PtrOfSimpleWB {
		pub fn as_raw_PtrOfSimpleWB(&self) -> *mut c_void { self.ptr }
	
		pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
			Self { ptr }
		}
	
		/// Get raw pointer to the inner object
		pub fn get_inner_ptr(&self) -> *mut c_void {
			extern "C" { fn cv_PtrOfSimpleWB_get_inner_ptr(instance: *mut c_void) -> *mut c_void; }
			unsafe { cv_PtrOfSimpleWB_get_inner_ptr(self.as_raw_PtrOfSimpleWB()) }
		}
	}
	
	impl Drop for PtrOfSimpleWB {
		fn drop(&mut self) {
			extern "C" { fn cv_PtrOfSimpleWB_delete(instance: *mut c_void); }
			unsafe { cv_PtrOfSimpleWB_delete(self.as_raw_PtrOfSimpleWB()) };
		}
	}
	
	unsafe impl Send for PtrOfSimpleWB {}
	
	impl core::AlgorithmTrait for PtrOfSimpleWB {
		fn as_raw_Algorithm(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::xphoto::SimpleWB for PtrOfSimpleWB {
		fn as_raw_SimpleWB(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
	impl crate::xphoto::WhiteBalancer for PtrOfSimpleWB {
		fn as_raw_WhiteBalancer(&self) -> *mut c_void {
			self.get_inner_ptr()
		}
	}
	
}
#[cfg(feature = "contrib")]
pub use xphoto_types::*;

pub use crate::manual::types::*;
