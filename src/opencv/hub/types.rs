
#[cfg(ocvrs_has_module_aruco)]
mod aruco_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBoard = core::Ptr<crate::aruco::Board>;
	
	ptr_extern! { crate::aruco::Board,
		cv_PtrOfBoard_delete, cv_PtrOfBoard_get_inner_ptr, cv_PtrOfBoard_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::aruco::Board, cv_PtrOfBoard_new }
	
	impl PtrOfBoard {
		#[inline] pub fn as_raw_PtrOfBoard(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::aruco::BoardTraitConst for PtrOfBoard {
		#[inline] fn as_raw_Board(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::aruco::BoardTrait for PtrOfBoard {
		#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCharucoBoard = core::Ptr<crate::aruco::CharucoBoard>;
	
	ptr_extern! { crate::aruco::CharucoBoard,
		cv_PtrOfCharucoBoard_delete, cv_PtrOfCharucoBoard_get_inner_ptr, cv_PtrOfCharucoBoard_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::aruco::CharucoBoard, cv_PtrOfCharucoBoard_new }
	
	impl PtrOfCharucoBoard {
		#[inline] pub fn as_raw_PtrOfCharucoBoard(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCharucoBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::aruco::CharucoBoardTraitConst for PtrOfCharucoBoard {
		#[inline] fn as_raw_CharucoBoard(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::aruco::CharucoBoardTrait for PtrOfCharucoBoard {
		#[inline] fn as_raw_mut_CharucoBoard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::aruco::BoardTraitConst for PtrOfCharucoBoard {
		#[inline] fn as_raw_Board(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::aruco::BoardTrait for PtrOfCharucoBoard {
		#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetectorParameters = core::Ptr<crate::aruco::DetectorParameters>;
	
	ptr_extern! { crate::aruco::DetectorParameters,
		cv_PtrOfDetectorParameters_delete, cv_PtrOfDetectorParameters_get_inner_ptr, cv_PtrOfDetectorParameters_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::aruco::DetectorParameters, cv_PtrOfDetectorParameters_new }
	
	impl PtrOfDetectorParameters {
		#[inline] pub fn as_raw_PtrOfDetectorParameters(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetectorParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::aruco::DetectorParametersTraitConst for PtrOfDetectorParameters {
		#[inline] fn as_raw_DetectorParameters(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::aruco::DetectorParametersTrait for PtrOfDetectorParameters {
		#[inline] fn as_raw_mut_DetectorParameters(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDictionary = core::Ptr<crate::aruco::Dictionary>;
	
	ptr_extern! { crate::aruco::Dictionary,
		cv_PtrOfDictionary_delete, cv_PtrOfDictionary_get_inner_ptr, cv_PtrOfDictionary_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::aruco::Dictionary, cv_PtrOfDictionary_new }
	
	impl PtrOfDictionary {
		#[inline] pub fn as_raw_PtrOfDictionary(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDictionary(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::aruco::DictionaryTraitConst for PtrOfDictionary {
		#[inline] fn as_raw_Dictionary(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::aruco::DictionaryTrait for PtrOfDictionary {
		#[inline] fn as_raw_mut_Dictionary(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEstimateParameters = core::Ptr<crate::aruco::EstimateParameters>;
	
	ptr_extern! { crate::aruco::EstimateParameters,
		cv_PtrOfEstimateParameters_delete, cv_PtrOfEstimateParameters_get_inner_ptr, cv_PtrOfEstimateParameters_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::aruco::EstimateParameters, cv_PtrOfEstimateParameters_new }
	
	impl PtrOfEstimateParameters {
		#[inline] pub fn as_raw_PtrOfEstimateParameters(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEstimateParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::aruco::EstimateParametersTraitConst for PtrOfEstimateParameters {
		#[inline] fn as_raw_EstimateParameters(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::aruco::EstimateParametersTrait for PtrOfEstimateParameters {
		#[inline] fn as_raw_mut_EstimateParameters(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGridBoard = core::Ptr<crate::aruco::GridBoard>;
	
	ptr_extern! { crate::aruco::GridBoard,
		cv_PtrOfGridBoard_delete, cv_PtrOfGridBoard_get_inner_ptr, cv_PtrOfGridBoard_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::aruco::GridBoard, cv_PtrOfGridBoard_new }
	
	impl PtrOfGridBoard {
		#[inline] pub fn as_raw_PtrOfGridBoard(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGridBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::aruco::GridBoardTraitConst for PtrOfGridBoard {
		#[inline] fn as_raw_GridBoard(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::aruco::GridBoardTrait for PtrOfGridBoard {
		#[inline] fn as_raw_mut_GridBoard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::aruco::BoardTraitConst for PtrOfGridBoard {
		#[inline] fn as_raw_Board(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::aruco::BoardTrait for PtrOfGridBoard {
		#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_aruco)]
pub use aruco_types::*;

#[cfg(ocvrs_has_module_barcode)]
mod barcode_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type VectorOfBarcodeType = core::Vector<crate::barcode::BarcodeType>;
	
	impl VectorOfBarcodeType {
		pub fn as_raw_VectorOfBarcodeType(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfBarcodeType(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::barcode::BarcodeType, *const c_void, *mut c_void,
		cv_VectorOfBarcodeType_new, cv_VectorOfBarcodeType_delete,
		cv_VectorOfBarcodeType_len, cv_VectorOfBarcodeType_is_empty,
		cv_VectorOfBarcodeType_capacity, cv_VectorOfBarcodeType_shrink_to_fit,
		cv_VectorOfBarcodeType_reserve, cv_VectorOfBarcodeType_remove,
		cv_VectorOfBarcodeType_swap, cv_VectorOfBarcodeType_clear,
		cv_VectorOfBarcodeType_get, cv_VectorOfBarcodeType_set,
		cv_VectorOfBarcodeType_push, cv_VectorOfBarcodeType_insert,
	}
	vector_copy_non_bool! { crate::barcode::BarcodeType, *const c_void, *mut c_void,
		cv_VectorOfBarcodeType_data, cv_VectorOfBarcodeType_data_mut, cv_VectorOfBarcodeType_from_slice,
		cv_VectorOfBarcodeType_clone,
	}
	
	unsafe impl Send for core::Vector<crate::barcode::BarcodeType> {}
	
}
#[cfg(ocvrs_has_module_barcode)]
pub use barcode_types::*;

#[cfg(ocvrs_has_module_bgsegm)]
mod bgsegm_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBackgroundSubtractorCNT = core::Ptr<dyn crate::bgsegm::BackgroundSubtractorCNT>;
	
	ptr_extern! { dyn crate::bgsegm::BackgroundSubtractorCNT,
		cv_PtrOfBackgroundSubtractorCNT_delete, cv_PtrOfBackgroundSubtractorCNT_get_inner_ptr, cv_PtrOfBackgroundSubtractorCNT_get_inner_ptr_mut
	}
	
	impl PtrOfBackgroundSubtractorCNT {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorCNT(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorCNT(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorCNTConst for PtrOfBackgroundSubtractorCNT {
		#[inline] fn as_raw_BackgroundSubtractorCNT(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorCNT for PtrOfBackgroundSubtractorCNT {
		#[inline] fn as_raw_mut_BackgroundSubtractorCNT(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfBackgroundSubtractorCNT {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfBackgroundSubtractorCNT {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorConst for PtrOfBackgroundSubtractorCNT {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorCNT {
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackgroundSubtractorGMG = core::Ptr<dyn crate::bgsegm::BackgroundSubtractorGMG>;
	
	ptr_extern! { dyn crate::bgsegm::BackgroundSubtractorGMG,
		cv_PtrOfBackgroundSubtractorGMG_delete, cv_PtrOfBackgroundSubtractorGMG_get_inner_ptr, cv_PtrOfBackgroundSubtractorGMG_get_inner_ptr_mut
	}
	
	impl PtrOfBackgroundSubtractorGMG {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorGMG(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorGMG(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorGMGConst for PtrOfBackgroundSubtractorGMG {
		#[inline] fn as_raw_BackgroundSubtractorGMG(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorGMG for PtrOfBackgroundSubtractorGMG {
		#[inline] fn as_raw_mut_BackgroundSubtractorGMG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfBackgroundSubtractorGMG {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfBackgroundSubtractorGMG {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorConst for PtrOfBackgroundSubtractorGMG {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorGMG {
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackgroundSubtractorGSOC = core::Ptr<dyn crate::bgsegm::BackgroundSubtractorGSOC>;
	
	ptr_extern! { dyn crate::bgsegm::BackgroundSubtractorGSOC,
		cv_PtrOfBackgroundSubtractorGSOC_delete, cv_PtrOfBackgroundSubtractorGSOC_get_inner_ptr, cv_PtrOfBackgroundSubtractorGSOC_get_inner_ptr_mut
	}
	
	impl PtrOfBackgroundSubtractorGSOC {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorGSOC(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorGSOC(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorGSOCConst for PtrOfBackgroundSubtractorGSOC {
		#[inline] fn as_raw_BackgroundSubtractorGSOC(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorGSOC for PtrOfBackgroundSubtractorGSOC {
		#[inline] fn as_raw_mut_BackgroundSubtractorGSOC(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfBackgroundSubtractorGSOC {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfBackgroundSubtractorGSOC {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorConst for PtrOfBackgroundSubtractorGSOC {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorGSOC {
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackgroundSubtractorLSBP = core::Ptr<dyn crate::bgsegm::BackgroundSubtractorLSBP>;
	
	ptr_extern! { dyn crate::bgsegm::BackgroundSubtractorLSBP,
		cv_PtrOfBackgroundSubtractorLSBP_delete, cv_PtrOfBackgroundSubtractorLSBP_get_inner_ptr, cv_PtrOfBackgroundSubtractorLSBP_get_inner_ptr_mut
	}
	
	impl PtrOfBackgroundSubtractorLSBP {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorLSBP(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorLSBP(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorLSBPConst for PtrOfBackgroundSubtractorLSBP {
		#[inline] fn as_raw_BackgroundSubtractorLSBP(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorLSBP for PtrOfBackgroundSubtractorLSBP {
		#[inline] fn as_raw_mut_BackgroundSubtractorLSBP(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfBackgroundSubtractorLSBP {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfBackgroundSubtractorLSBP {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorConst for PtrOfBackgroundSubtractorLSBP {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorLSBP {
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackgroundSubtractorMOG = core::Ptr<dyn crate::bgsegm::BackgroundSubtractorMOG>;
	
	ptr_extern! { dyn crate::bgsegm::BackgroundSubtractorMOG,
		cv_PtrOfBackgroundSubtractorMOG_delete, cv_PtrOfBackgroundSubtractorMOG_get_inner_ptr, cv_PtrOfBackgroundSubtractorMOG_get_inner_ptr_mut
	}
	
	impl PtrOfBackgroundSubtractorMOG {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorMOG(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorMOG(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorMOGConst for PtrOfBackgroundSubtractorMOG {
		#[inline] fn as_raw_BackgroundSubtractorMOG(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorMOG for PtrOfBackgroundSubtractorMOG {
		#[inline] fn as_raw_mut_BackgroundSubtractorMOG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfBackgroundSubtractorMOG {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfBackgroundSubtractorMOG {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorConst for PtrOfBackgroundSubtractorMOG {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorMOG {
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSyntheticSequenceGenerator = core::Ptr<crate::bgsegm::SyntheticSequenceGenerator>;
	
	ptr_extern! { crate::bgsegm::SyntheticSequenceGenerator,
		cv_PtrOfSyntheticSequenceGenerator_delete, cv_PtrOfSyntheticSequenceGenerator_get_inner_ptr, cv_PtrOfSyntheticSequenceGenerator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::bgsegm::SyntheticSequenceGenerator, cv_PtrOfSyntheticSequenceGenerator_new }
	
	impl PtrOfSyntheticSequenceGenerator {
		#[inline] pub fn as_raw_PtrOfSyntheticSequenceGenerator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSyntheticSequenceGenerator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::bgsegm::SyntheticSequenceGeneratorTraitConst for PtrOfSyntheticSequenceGenerator {
		#[inline] fn as_raw_SyntheticSequenceGenerator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::bgsegm::SyntheticSequenceGeneratorTrait for PtrOfSyntheticSequenceGenerator {
		#[inline] fn as_raw_mut_SyntheticSequenceGenerator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSyntheticSequenceGenerator {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSyntheticSequenceGenerator {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_bgsegm)]
pub use bgsegm_types::*;

#[cfg(ocvrs_has_module_bioinspired)]
mod bioinspired_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfRetina = core::Ptr<dyn crate::bioinspired::Retina>;
	
	ptr_extern! { dyn crate::bioinspired::Retina,
		cv_PtrOfRetina_delete, cv_PtrOfRetina_get_inner_ptr, cv_PtrOfRetina_get_inner_ptr_mut
	}
	
	impl PtrOfRetina {
		#[inline] pub fn as_raw_PtrOfRetina(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRetina(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::bioinspired::RetinaConst for PtrOfRetina {
		#[inline] fn as_raw_Retina(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::bioinspired::Retina for PtrOfRetina {
		#[inline] fn as_raw_mut_Retina(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfRetina {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfRetina {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRetinaFastToneMapping = core::Ptr<dyn crate::bioinspired::RetinaFastToneMapping>;
	
	ptr_extern! { dyn crate::bioinspired::RetinaFastToneMapping,
		cv_PtrOfRetinaFastToneMapping_delete, cv_PtrOfRetinaFastToneMapping_get_inner_ptr, cv_PtrOfRetinaFastToneMapping_get_inner_ptr_mut
	}
	
	impl PtrOfRetinaFastToneMapping {
		#[inline] pub fn as_raw_PtrOfRetinaFastToneMapping(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRetinaFastToneMapping(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::bioinspired::RetinaFastToneMappingConst for PtrOfRetinaFastToneMapping {
		#[inline] fn as_raw_RetinaFastToneMapping(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::bioinspired::RetinaFastToneMapping for PtrOfRetinaFastToneMapping {
		#[inline] fn as_raw_mut_RetinaFastToneMapping(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfRetinaFastToneMapping {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfRetinaFastToneMapping {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTransientAreasSegmentationModule = core::Ptr<dyn crate::bioinspired::TransientAreasSegmentationModule>;
	
	ptr_extern! { dyn crate::bioinspired::TransientAreasSegmentationModule,
		cv_PtrOfTransientAreasSegmentationModule_delete, cv_PtrOfTransientAreasSegmentationModule_get_inner_ptr, cv_PtrOfTransientAreasSegmentationModule_get_inner_ptr_mut
	}
	
	impl PtrOfTransientAreasSegmentationModule {
		#[inline] pub fn as_raw_PtrOfTransientAreasSegmentationModule(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTransientAreasSegmentationModule(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::bioinspired::TransientAreasSegmentationModuleConst for PtrOfTransientAreasSegmentationModule {
		#[inline] fn as_raw_TransientAreasSegmentationModule(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::bioinspired::TransientAreasSegmentationModule for PtrOfTransientAreasSegmentationModule {
		#[inline] fn as_raw_mut_TransientAreasSegmentationModule(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfTransientAreasSegmentationModule {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfTransientAreasSegmentationModule {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_bioinspired)]
pub use bioinspired_types::*;

#[cfg(ocvrs_has_module_calib3d)]
mod calib3d_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfLMSolver = core::Ptr<dyn crate::calib3d::LMSolver>;
	
	ptr_extern! { dyn crate::calib3d::LMSolver,
		cv_PtrOfLMSolver_delete, cv_PtrOfLMSolver_get_inner_ptr, cv_PtrOfLMSolver_get_inner_ptr_mut
	}
	
	impl PtrOfLMSolver {
		#[inline] pub fn as_raw_PtrOfLMSolver(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLMSolver(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::calib3d::LMSolverConst for PtrOfLMSolver {
		#[inline] fn as_raw_LMSolver(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::LMSolver for PtrOfLMSolver {
		#[inline] fn as_raw_mut_LMSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfLMSolver {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfLMSolver {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLMSolver_Callback = core::Ptr<dyn crate::calib3d::LMSolver_Callback>;
	
	ptr_extern! { dyn crate::calib3d::LMSolver_Callback,
		cv_PtrOfLMSolver_Callback_delete, cv_PtrOfLMSolver_Callback_get_inner_ptr, cv_PtrOfLMSolver_Callback_get_inner_ptr_mut
	}
	
	impl PtrOfLMSolver_Callback {
		#[inline] pub fn as_raw_PtrOfLMSolver_Callback(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLMSolver_Callback(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::calib3d::LMSolver_CallbackConst for PtrOfLMSolver_Callback {
		#[inline] fn as_raw_LMSolver_Callback(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::LMSolver_Callback for PtrOfLMSolver_Callback {
		#[inline] fn as_raw_mut_LMSolver_Callback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStereoBM = core::Ptr<dyn crate::calib3d::StereoBM>;
	
	ptr_extern! { dyn crate::calib3d::StereoBM,
		cv_PtrOfStereoBM_delete, cv_PtrOfStereoBM_get_inner_ptr, cv_PtrOfStereoBM_get_inner_ptr_mut
	}
	
	impl PtrOfStereoBM {
		#[inline] pub fn as_raw_PtrOfStereoBM(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStereoBM(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::calib3d::StereoBMConst for PtrOfStereoBM {
		#[inline] fn as_raw_StereoBM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoBM for PtrOfStereoBM {
		#[inline] fn as_raw_mut_StereoBM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfStereoBM {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfStereoBM {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcherConst for PtrOfStereoBM {
		#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoMatcher for PtrOfStereoBM {
		#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStereoMatcher = core::Ptr<dyn crate::calib3d::StereoMatcher>;
	
	ptr_extern! { dyn crate::calib3d::StereoMatcher,
		cv_PtrOfStereoMatcher_delete, cv_PtrOfStereoMatcher_get_inner_ptr, cv_PtrOfStereoMatcher_get_inner_ptr_mut
	}
	
	impl PtrOfStereoMatcher {
		#[inline] pub fn as_raw_PtrOfStereoMatcher(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStereoMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcherConst for PtrOfStereoMatcher {
		#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoMatcher for PtrOfStereoMatcher {
		#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfStereoMatcher {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfStereoMatcher {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStereoSGBM = core::Ptr<dyn crate::calib3d::StereoSGBM>;
	
	ptr_extern! { dyn crate::calib3d::StereoSGBM,
		cv_PtrOfStereoSGBM_delete, cv_PtrOfStereoSGBM_get_inner_ptr, cv_PtrOfStereoSGBM_get_inner_ptr_mut
	}
	
	impl PtrOfStereoSGBM {
		#[inline] pub fn as_raw_PtrOfStereoSGBM(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStereoSGBM(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::calib3d::StereoSGBMConst for PtrOfStereoSGBM {
		#[inline] fn as_raw_StereoSGBM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoSGBM for PtrOfStereoSGBM {
		#[inline] fn as_raw_mut_StereoSGBM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfStereoSGBM {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfStereoSGBM {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcherConst for PtrOfStereoSGBM {
		#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoMatcher for PtrOfStereoSGBM {
		#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_calib3d)]
pub use calib3d_types::*;

#[cfg(ocvrs_has_module_core)]
mod core_types {
	use crate::{mod_prelude::*, core, types, sys};

	impl core::GpuMat_AllocatorConst for types::AbstractRefMut<'static, dyn core::GpuMat_Allocator> {
		#[inline] fn as_raw_GpuMat_Allocator(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::GpuMat_Allocator for types::AbstractRefMut<'static, dyn core::GpuMat_Allocator> {
		#[inline] fn as_raw_mut_GpuMat_Allocator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::MatOpConst for types::AbstractRefMut<'static, dyn core::MatOp> {
		#[inline] fn as_raw_MatOp(&self) -> *const c_void { self.as_raw() }
	}
	
	impl core::MatOp for types::AbstractRefMut<'static, dyn core::MatOp> {
		#[inline] fn as_raw_mut_MatOp(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	pub type PtrOfConjGradSolver = core::Ptr<dyn core::ConjGradSolver>;
	
	ptr_extern! { dyn core::ConjGradSolver,
		cv_PtrOfConjGradSolver_delete, cv_PtrOfConjGradSolver_get_inner_ptr, cv_PtrOfConjGradSolver_get_inner_ptr_mut
	}
	
	impl PtrOfConjGradSolver {
		#[inline] pub fn as_raw_PtrOfConjGradSolver(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConjGradSolver(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::ConjGradSolverConst for PtrOfConjGradSolver {
		#[inline] fn as_raw_ConjGradSolver(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::ConjGradSolver for PtrOfConjGradSolver {
		#[inline] fn as_raw_mut_ConjGradSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfConjGradSolver {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfConjGradSolver {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::MinProblemSolverConst for PtrOfConjGradSolver {
		#[inline] fn as_raw_MinProblemSolver(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::MinProblemSolver for PtrOfConjGradSolver {
		#[inline] fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDownhillSolver = core::Ptr<dyn core::DownhillSolver>;
	
	ptr_extern! { dyn core::DownhillSolver,
		cv_PtrOfDownhillSolver_delete, cv_PtrOfDownhillSolver_get_inner_ptr, cv_PtrOfDownhillSolver_get_inner_ptr_mut
	}
	
	impl PtrOfDownhillSolver {
		#[inline] pub fn as_raw_PtrOfDownhillSolver(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDownhillSolver(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::DownhillSolverConst for PtrOfDownhillSolver {
		#[inline] fn as_raw_DownhillSolver(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::DownhillSolver for PtrOfDownhillSolver {
		#[inline] fn as_raw_mut_DownhillSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfDownhillSolver {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfDownhillSolver {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::MinProblemSolverConst for PtrOfDownhillSolver {
		#[inline] fn as_raw_MinProblemSolver(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::MinProblemSolver for PtrOfDownhillSolver {
		#[inline] fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFileStorage = core::Ptr<core::FileStorage>;
	
	ptr_extern! { core::FileStorage,
		cv_PtrOfFileStorage_delete, cv_PtrOfFileStorage_get_inner_ptr, cv_PtrOfFileStorage_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { core::FileStorage, cv_PtrOfFileStorage_new }
	
	impl PtrOfFileStorage {
		#[inline] pub fn as_raw_PtrOfFileStorage(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFileStorage(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::FileStorageTraitConst for PtrOfFileStorage {
		#[inline] fn as_raw_FileStorage(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::FileStorageTrait for PtrOfFileStorage {
		#[inline] fn as_raw_mut_FileStorage(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFormatted = core::Ptr<dyn core::Formatted>;
	
	ptr_extern! { dyn core::Formatted,
		cv_PtrOfFormatted_delete, cv_PtrOfFormatted_get_inner_ptr, cv_PtrOfFormatted_get_inner_ptr_mut
	}
	
	impl PtrOfFormatted {
		#[inline] pub fn as_raw_PtrOfFormatted(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFormatted(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::FormattedConst for PtrOfFormatted {
		#[inline] fn as_raw_Formatted(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::Formatted for PtrOfFormatted {
		#[inline] fn as_raw_mut_Formatted(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFormatter = core::Ptr<dyn core::Formatter>;
	
	ptr_extern! { dyn core::Formatter,
		cv_PtrOfFormatter_delete, cv_PtrOfFormatter_get_inner_ptr, cv_PtrOfFormatter_get_inner_ptr_mut
	}
	
	impl PtrOfFormatter {
		#[inline] pub fn as_raw_PtrOfFormatter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFormatter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::FormatterConst for PtrOfFormatter {
		#[inline] fn as_raw_Formatter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::Formatter for PtrOfFormatter {
		#[inline] fn as_raw_mut_Formatter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGpuMat_Allocator = core::Ptr<dyn core::GpuMat_Allocator>;
	
	ptr_extern! { dyn core::GpuMat_Allocator,
		cv_PtrOfGpuMat_Allocator_delete, cv_PtrOfGpuMat_Allocator_get_inner_ptr, cv_PtrOfGpuMat_Allocator_get_inner_ptr_mut
	}
	
	impl PtrOfGpuMat_Allocator {
		#[inline] pub fn as_raw_PtrOfGpuMat_Allocator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGpuMat_Allocator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::GpuMat_AllocatorConst for PtrOfGpuMat_Allocator {
		#[inline] fn as_raw_GpuMat_Allocator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::GpuMat_Allocator for PtrOfGpuMat_Allocator {
		#[inline] fn as_raw_mut_GpuMat_Allocator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMinProblemSolver_Function = core::Ptr<dyn core::MinProblemSolver_Function>;
	
	ptr_extern! { dyn core::MinProblemSolver_Function,
		cv_PtrOfMinProblemSolver_Function_delete, cv_PtrOfMinProblemSolver_Function_get_inner_ptr, cv_PtrOfMinProblemSolver_Function_get_inner_ptr_mut
	}
	
	impl PtrOfMinProblemSolver_Function {
		#[inline] pub fn as_raw_PtrOfMinProblemSolver_Function(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMinProblemSolver_Function(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::MinProblemSolver_FunctionConst for PtrOfMinProblemSolver_Function {
		#[inline] fn as_raw_MinProblemSolver_Function(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::MinProblemSolver_Function for PtrOfMinProblemSolver_Function {
		#[inline] fn as_raw_mut_MinProblemSolver_Function(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOriginalClassName = core::Ptr<core::OriginalClassName>;
	
	ptr_extern! { core::OriginalClassName,
		cv_PtrOfOriginalClassName_delete, cv_PtrOfOriginalClassName_get_inner_ptr, cv_PtrOfOriginalClassName_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { core::OriginalClassName, cv_PtrOfOriginalClassName_new }
	
	impl PtrOfOriginalClassName {
		#[inline] pub fn as_raw_PtrOfOriginalClassName(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOriginalClassName(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl core::OriginalClassNameTraitConst for PtrOfOriginalClassName {
		#[inline] fn as_raw_OriginalClassName(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::OriginalClassNameTrait for PtrOfOriginalClassName {
		#[inline] fn as_raw_mut_OriginalClassName(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOff32 = core::Ptr<f32>;
	
	ptr_extern! { f32,
		cv_PtrOff32_delete, cv_PtrOff32_get_inner_ptr, cv_PtrOff32_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { f32, cv_PtrOff32_new }
	
	impl PtrOff32 {
		#[inline] pub fn as_raw_PtrOff32(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOff32(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	pub type VectorOfDMatch = core::Vector<core::DMatch>;
	
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
		cv_VectorOfDMatch_get, cv_VectorOfDMatch_set,
		cv_VectorOfDMatch_push, cv_VectorOfDMatch_insert,
	}
	vector_copy_non_bool! { core::DMatch, *const c_void, *mut c_void,
		cv_VectorOfDMatch_data, cv_VectorOfDMatch_data_mut, cv_VectorOfDMatch_from_slice,
		cv_VectorOfDMatch_clone,
	}
	
	unsafe impl Send for core::Vector<core::DMatch> {}
	
	pub type VectorOfGpuMat = core::Vector<core::GpuMat>;
	
	impl VectorOfGpuMat {
		pub fn as_raw_VectorOfGpuMat(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfGpuMat(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::GpuMat, *const c_void, *mut c_void,
		cv_VectorOfGpuMat_new, cv_VectorOfGpuMat_delete,
		cv_VectorOfGpuMat_len, cv_VectorOfGpuMat_is_empty,
		cv_VectorOfGpuMat_capacity, cv_VectorOfGpuMat_shrink_to_fit,
		cv_VectorOfGpuMat_reserve, cv_VectorOfGpuMat_remove,
		cv_VectorOfGpuMat_swap, cv_VectorOfGpuMat_clear,
		cv_VectorOfGpuMat_get, cv_VectorOfGpuMat_set,
		cv_VectorOfGpuMat_push, cv_VectorOfGpuMat_insert,
	}
	vector_non_copy_or_bool! { clone core::GpuMat }
	
	unsafe impl Send for core::Vector<core::GpuMat> {}
	
	pub type VectorOfKeyPoint = core::Vector<core::KeyPoint>;
	
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
		cv_VectorOfKeyPoint_get, cv_VectorOfKeyPoint_set,
		cv_VectorOfKeyPoint_push, cv_VectorOfKeyPoint_insert,
	}
	vector_copy_non_bool! { core::KeyPoint, *const c_void, *mut c_void,
		cv_VectorOfKeyPoint_data, cv_VectorOfKeyPoint_data_mut, cv_VectorOfKeyPoint_from_slice,
		cv_VectorOfKeyPoint_clone,
	}
	
	unsafe impl Send for core::Vector<core::KeyPoint> {}
	
	pub type VectorOfMat = core::Vector<core::Mat>;
	
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
		cv_VectorOfMat_get, cv_VectorOfMat_set,
		cv_VectorOfMat_push, cv_VectorOfMat_insert,
	}
	vector_non_copy_or_bool! { clone core::Mat }
	
	unsafe impl Send for core::Vector<core::Mat> {}
	
	pub type VectorOfPlatformInfo = core::Vector<core::PlatformInfo>;
	
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
		cv_VectorOfPlatformInfo_get, cv_VectorOfPlatformInfo_set,
		cv_VectorOfPlatformInfo_push, cv_VectorOfPlatformInfo_insert,
	}
	vector_non_copy_or_bool! { core::PlatformInfo }
	
	unsafe impl Send for core::Vector<core::PlatformInfo> {}
	
	pub type VectorOfPoint = core::Vector<core::Point>;
	
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
		cv_VectorOfPoint_get, cv_VectorOfPoint_set,
		cv_VectorOfPoint_push, cv_VectorOfPoint_insert,
	}
	vector_copy_non_bool! { core::Point, *const c_void, *mut c_void,
		cv_VectorOfPoint_data, cv_VectorOfPoint_data_mut, cv_VectorOfPoint_from_slice,
		cv_VectorOfPoint_clone,
	}
	
	unsafe impl Send for core::Vector<core::Point> {}
	
	impl core::ToInputArray for VectorOfPoint {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint_input_array(self.as_raw_VectorOfPoint(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfPoint {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfPoint_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint_output_array(self.as_raw_mut_VectorOfPoint(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfPoint {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfPoint_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint_input_output_array(self.as_raw_mut_VectorOfPoint(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfPoint }
	
	pub type VectorOfPoint2d = core::Vector<core::Point2d>;
	
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
		cv_VectorOfPoint2d_get, cv_VectorOfPoint2d_set,
		cv_VectorOfPoint2d_push, cv_VectorOfPoint2d_insert,
	}
	vector_copy_non_bool! { core::Point2d, *const c_void, *mut c_void,
		cv_VectorOfPoint2d_data, cv_VectorOfPoint2d_data_mut, cv_VectorOfPoint2d_from_slice,
		cv_VectorOfPoint2d_clone,
	}
	
	unsafe impl Send for core::Vector<core::Point2d> {}
	
	impl core::ToInputArray for VectorOfPoint2d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint2d_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint2d_input_array(self.as_raw_VectorOfPoint2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfPoint2d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfPoint2d_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint2d_output_array(self.as_raw_mut_VectorOfPoint2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfPoint2d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfPoint2d_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint2d_input_output_array(self.as_raw_mut_VectorOfPoint2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfPoint2d }
	
	pub type VectorOfPoint2f = core::Vector<core::Point2f>;
	
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
		cv_VectorOfPoint2f_get, cv_VectorOfPoint2f_set,
		cv_VectorOfPoint2f_push, cv_VectorOfPoint2f_insert,
	}
	vector_copy_non_bool! { core::Point2f, *const c_void, *mut c_void,
		cv_VectorOfPoint2f_data, cv_VectorOfPoint2f_data_mut, cv_VectorOfPoint2f_from_slice,
		cv_VectorOfPoint2f_clone,
	}
	
	unsafe impl Send for core::Vector<core::Point2f> {}
	
	impl core::ToInputArray for VectorOfPoint2f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint2f_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint2f_input_array(self.as_raw_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfPoint2f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfPoint2f_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint2f_output_array(self.as_raw_mut_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfPoint2f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfPoint2f_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint2f_input_output_array(self.as_raw_mut_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfPoint2f }
	
	pub type VectorOfPoint3d = core::Vector<core::Point3d>;
	
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
		cv_VectorOfPoint3d_get, cv_VectorOfPoint3d_set,
		cv_VectorOfPoint3d_push, cv_VectorOfPoint3d_insert,
	}
	vector_copy_non_bool! { core::Point3d, *const c_void, *mut c_void,
		cv_VectorOfPoint3d_data, cv_VectorOfPoint3d_data_mut, cv_VectorOfPoint3d_from_slice,
		cv_VectorOfPoint3d_clone,
	}
	
	unsafe impl Send for core::Vector<core::Point3d> {}
	
	impl core::ToInputArray for VectorOfPoint3d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint3d_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3d_input_array(self.as_raw_VectorOfPoint3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfPoint3d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfPoint3d_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3d_output_array(self.as_raw_mut_VectorOfPoint3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfPoint3d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfPoint3d_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3d_input_output_array(self.as_raw_mut_VectorOfPoint3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfPoint3d }
	
	pub type VectorOfPoint3f = core::Vector<core::Point3f>;
	
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
		cv_VectorOfPoint3f_get, cv_VectorOfPoint3f_set,
		cv_VectorOfPoint3f_push, cv_VectorOfPoint3f_insert,
	}
	vector_copy_non_bool! { core::Point3f, *const c_void, *mut c_void,
		cv_VectorOfPoint3f_data, cv_VectorOfPoint3f_data_mut, cv_VectorOfPoint3f_from_slice,
		cv_VectorOfPoint3f_clone,
	}
	
	unsafe impl Send for core::Vector<core::Point3f> {}
	
	impl core::ToInputArray for VectorOfPoint3f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint3f_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3f_input_array(self.as_raw_VectorOfPoint3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfPoint3f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfPoint3f_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3f_output_array(self.as_raw_mut_VectorOfPoint3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfPoint3f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfPoint3f_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3f_input_output_array(self.as_raw_mut_VectorOfPoint3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfPoint3f }
	
	pub type VectorOfPoint3i = core::Vector<core::Point3i>;
	
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
		cv_VectorOfPoint3i_get, cv_VectorOfPoint3i_set,
		cv_VectorOfPoint3i_push, cv_VectorOfPoint3i_insert,
	}
	vector_copy_non_bool! { core::Point3i, *const c_void, *mut c_void,
		cv_VectorOfPoint3i_data, cv_VectorOfPoint3i_data_mut, cv_VectorOfPoint3i_from_slice,
		cv_VectorOfPoint3i_clone,
	}
	
	unsafe impl Send for core::Vector<core::Point3i> {}
	
	impl core::ToInputArray for VectorOfPoint3i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfPoint3i_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3i_input_array(self.as_raw_VectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfPoint3i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfPoint3i_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3i_output_array(self.as_raw_mut_VectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfPoint3i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfPoint3i_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3i_input_output_array(self.as_raw_mut_VectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfPoint3i }
	
	pub type VectorOfRange = core::Vector<core::Range>;
	
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
		cv_VectorOfRange_get, cv_VectorOfRange_set,
		cv_VectorOfRange_push, cv_VectorOfRange_insert,
	}
	vector_non_copy_or_bool! { core::Range }
	
	unsafe impl Send for core::Vector<core::Range> {}
	
	pub type VectorOfRect = core::Vector<core::Rect>;
	
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
		cv_VectorOfRect_get, cv_VectorOfRect_set,
		cv_VectorOfRect_push, cv_VectorOfRect_insert,
	}
	vector_copy_non_bool! { core::Rect, *const c_void, *mut c_void,
		cv_VectorOfRect_data, cv_VectorOfRect_data_mut, cv_VectorOfRect_from_slice,
		cv_VectorOfRect_clone,
	}
	
	unsafe impl Send for core::Vector<core::Rect> {}
	
	impl core::ToInputArray for VectorOfRect {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfRect_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfRect_input_array(self.as_raw_VectorOfRect(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfRect {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfRect_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfRect_output_array(self.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfRect {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfRect_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfRect_input_output_array(self.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfRect }
	
	pub type VectorOfRect2d = core::Vector<core::Rect2d>;
	
	impl VectorOfRect2d {
		pub fn as_raw_VectorOfRect2d(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfRect2d(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Rect2d, *const c_void, *mut c_void,
		cv_VectorOfRect2d_new, cv_VectorOfRect2d_delete,
		cv_VectorOfRect2d_len, cv_VectorOfRect2d_is_empty,
		cv_VectorOfRect2d_capacity, cv_VectorOfRect2d_shrink_to_fit,
		cv_VectorOfRect2d_reserve, cv_VectorOfRect2d_remove,
		cv_VectorOfRect2d_swap, cv_VectorOfRect2d_clear,
		cv_VectorOfRect2d_get, cv_VectorOfRect2d_set,
		cv_VectorOfRect2d_push, cv_VectorOfRect2d_insert,
	}
	vector_copy_non_bool! { core::Rect2d, *const c_void, *mut c_void,
		cv_VectorOfRect2d_data, cv_VectorOfRect2d_data_mut, cv_VectorOfRect2d_from_slice,
		cv_VectorOfRect2d_clone,
	}
	
	unsafe impl Send for core::Vector<core::Rect2d> {}
	
	impl core::ToInputArray for VectorOfRect2d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfRect2d_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfRect2d_input_array(self.as_raw_VectorOfRect2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfRect2d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfRect2d_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfRect2d_output_array(self.as_raw_mut_VectorOfRect2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfRect2d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfRect2d_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfRect2d_input_output_array(self.as_raw_mut_VectorOfRect2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfRect2d }
	
	pub type VectorOfRotatedRect = core::Vector<core::RotatedRect>;
	
	impl VectorOfRotatedRect {
		pub fn as_raw_VectorOfRotatedRect(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfRotatedRect(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::RotatedRect, *const c_void, *mut c_void,
		cv_VectorOfRotatedRect_new, cv_VectorOfRotatedRect_delete,
		cv_VectorOfRotatedRect_len, cv_VectorOfRotatedRect_is_empty,
		cv_VectorOfRotatedRect_capacity, cv_VectorOfRotatedRect_shrink_to_fit,
		cv_VectorOfRotatedRect_reserve, cv_VectorOfRotatedRect_remove,
		cv_VectorOfRotatedRect_swap, cv_VectorOfRotatedRect_clear,
		cv_VectorOfRotatedRect_get, cv_VectorOfRotatedRect_set,
		cv_VectorOfRotatedRect_push, cv_VectorOfRotatedRect_insert,
	}
	vector_non_copy_or_bool! { core::RotatedRect }
	
	unsafe impl Send for core::Vector<core::RotatedRect> {}
	
	pub type VectorOfScalar = core::Vector<core::Scalar>;
	
	impl VectorOfScalar {
		pub fn as_raw_VectorOfScalar(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfScalar(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Scalar, *const c_void, *mut c_void,
		cv_VectorOfScalar_new, cv_VectorOfScalar_delete,
		cv_VectorOfScalar_len, cv_VectorOfScalar_is_empty,
		cv_VectorOfScalar_capacity, cv_VectorOfScalar_shrink_to_fit,
		cv_VectorOfScalar_reserve, cv_VectorOfScalar_remove,
		cv_VectorOfScalar_swap, cv_VectorOfScalar_clear,
		cv_VectorOfScalar_get, cv_VectorOfScalar_set,
		cv_VectorOfScalar_push, cv_VectorOfScalar_insert,
	}
	vector_copy_non_bool! { core::Scalar, *const c_void, *mut c_void,
		cv_VectorOfScalar_data, cv_VectorOfScalar_data_mut, cv_VectorOfScalar_from_slice,
		cv_VectorOfScalar_clone,
	}
	
	unsafe impl Send for core::Vector<core::Scalar> {}
	
	impl core::ToInputArray for VectorOfScalar {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfScalar_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfScalar_input_array(self.as_raw_VectorOfScalar(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfScalar {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfScalar_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfScalar_output_array(self.as_raw_mut_VectorOfScalar(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfScalar {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfScalar_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfScalar_input_output_array(self.as_raw_mut_VectorOfScalar(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfScalar }
	
	pub type VectorOfSize = core::Vector<core::Size>;
	
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
		cv_VectorOfSize_get, cv_VectorOfSize_set,
		cv_VectorOfSize_push, cv_VectorOfSize_insert,
	}
	vector_copy_non_bool! { core::Size, *const c_void, *mut c_void,
		cv_VectorOfSize_data, cv_VectorOfSize_data_mut, cv_VectorOfSize_from_slice,
		cv_VectorOfSize_clone,
	}
	
	unsafe impl Send for core::Vector<core::Size> {}
	
	impl core::ToInputArray for VectorOfSize {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfSize_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfSize_input_array(self.as_raw_VectorOfSize(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfSize {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfSize_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfSize_output_array(self.as_raw_mut_VectorOfSize(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfSize {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfSize_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfSize_input_output_array(self.as_raw_mut_VectorOfSize(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfSize }
	
	pub type VectorOfString = core::Vector<String>;
	
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
		cv_VectorOfString_get, cv_VectorOfString_set,
		cv_VectorOfString_push, cv_VectorOfString_insert,
	}
	vector_non_copy_or_bool! { String }
	
	unsafe impl Send for core::Vector<String> {}
	
	pub type VectorOfUMat = core::Vector<core::UMat>;
	
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
		cv_VectorOfUMat_get, cv_VectorOfUMat_set,
		cv_VectorOfUMat_push, cv_VectorOfUMat_insert,
	}
	vector_non_copy_or_bool! { clone core::UMat }
	
	unsafe impl Send for core::Vector<core::UMat> {}
	
	pub type VectorOfVec2d = core::Vector<core::Vec2d>;
	
	impl VectorOfVec2d {
		pub fn as_raw_VectorOfVec2d(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec2d(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec2d, *const c_void, *mut c_void,
		cv_VectorOfVec2d_new, cv_VectorOfVec2d_delete,
		cv_VectorOfVec2d_len, cv_VectorOfVec2d_is_empty,
		cv_VectorOfVec2d_capacity, cv_VectorOfVec2d_shrink_to_fit,
		cv_VectorOfVec2d_reserve, cv_VectorOfVec2d_remove,
		cv_VectorOfVec2d_swap, cv_VectorOfVec2d_clear,
		cv_VectorOfVec2d_get, cv_VectorOfVec2d_set,
		cv_VectorOfVec2d_push, cv_VectorOfVec2d_insert,
	}
	vector_copy_non_bool! { core::Vec2d, *const c_void, *mut c_void,
		cv_VectorOfVec2d_data, cv_VectorOfVec2d_data_mut, cv_VectorOfVec2d_from_slice,
		cv_VectorOfVec2d_clone,
	}
	
	unsafe impl Send for core::Vector<core::Vec2d> {}
	
	impl core::ToInputArray for VectorOfVec2d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec2d_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2d_input_array(self.as_raw_VectorOfVec2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfVec2d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVec2d_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2d_output_array(self.as_raw_mut_VectorOfVec2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVec2d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVec2d_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2d_input_output_array(self.as_raw_mut_VectorOfVec2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfVec2d }
	
	pub type VectorOfVec2f = core::Vector<core::Vec2f>;
	
	impl VectorOfVec2f {
		pub fn as_raw_VectorOfVec2f(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec2f(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec2f, *const c_void, *mut c_void,
		cv_VectorOfVec2f_new, cv_VectorOfVec2f_delete,
		cv_VectorOfVec2f_len, cv_VectorOfVec2f_is_empty,
		cv_VectorOfVec2f_capacity, cv_VectorOfVec2f_shrink_to_fit,
		cv_VectorOfVec2f_reserve, cv_VectorOfVec2f_remove,
		cv_VectorOfVec2f_swap, cv_VectorOfVec2f_clear,
		cv_VectorOfVec2f_get, cv_VectorOfVec2f_set,
		cv_VectorOfVec2f_push, cv_VectorOfVec2f_insert,
	}
	vector_copy_non_bool! { core::Vec2f, *const c_void, *mut c_void,
		cv_VectorOfVec2f_data, cv_VectorOfVec2f_data_mut, cv_VectorOfVec2f_from_slice,
		cv_VectorOfVec2f_clone,
	}
	
	unsafe impl Send for core::Vector<core::Vec2f> {}
	
	impl core::ToInputArray for VectorOfVec2f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec2f_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2f_input_array(self.as_raw_VectorOfVec2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfVec2f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVec2f_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2f_output_array(self.as_raw_mut_VectorOfVec2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVec2f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVec2f_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2f_input_output_array(self.as_raw_mut_VectorOfVec2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfVec2f }
	
	pub type VectorOfVec2i = core::Vector<core::Vec2i>;
	
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
		cv_VectorOfVec2i_get, cv_VectorOfVec2i_set,
		cv_VectorOfVec2i_push, cv_VectorOfVec2i_insert,
	}
	vector_copy_non_bool! { core::Vec2i, *const c_void, *mut c_void,
		cv_VectorOfVec2i_data, cv_VectorOfVec2i_data_mut, cv_VectorOfVec2i_from_slice,
		cv_VectorOfVec2i_clone,
	}
	
	unsafe impl Send for core::Vector<core::Vec2i> {}
	
	impl core::ToInputArray for VectorOfVec2i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec2i_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2i_input_array(self.as_raw_VectorOfVec2i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfVec2i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVec2i_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2i_output_array(self.as_raw_mut_VectorOfVec2i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVec2i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVec2i_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2i_input_output_array(self.as_raw_mut_VectorOfVec2i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfVec2i }
	
	pub type VectorOfVec3d = core::Vector<core::Vec3d>;
	
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
		cv_VectorOfVec3d_get, cv_VectorOfVec3d_set,
		cv_VectorOfVec3d_push, cv_VectorOfVec3d_insert,
	}
	vector_copy_non_bool! { core::Vec3d, *const c_void, *mut c_void,
		cv_VectorOfVec3d_data, cv_VectorOfVec3d_data_mut, cv_VectorOfVec3d_from_slice,
		cv_VectorOfVec3d_clone,
	}
	
	unsafe impl Send for core::Vector<core::Vec3d> {}
	
	impl core::ToInputArray for VectorOfVec3d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec3d_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3d_input_array(self.as_raw_VectorOfVec3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfVec3d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVec3d_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3d_output_array(self.as_raw_mut_VectorOfVec3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVec3d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVec3d_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3d_input_output_array(self.as_raw_mut_VectorOfVec3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfVec3d }
	
	pub type VectorOfVec3f = core::Vector<core::Vec3f>;
	
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
		cv_VectorOfVec3f_get, cv_VectorOfVec3f_set,
		cv_VectorOfVec3f_push, cv_VectorOfVec3f_insert,
	}
	vector_copy_non_bool! { core::Vec3f, *const c_void, *mut c_void,
		cv_VectorOfVec3f_data, cv_VectorOfVec3f_data_mut, cv_VectorOfVec3f_from_slice,
		cv_VectorOfVec3f_clone,
	}
	
	unsafe impl Send for core::Vector<core::Vec3f> {}
	
	impl core::ToInputArray for VectorOfVec3f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec3f_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3f_input_array(self.as_raw_VectorOfVec3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfVec3f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVec3f_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3f_output_array(self.as_raw_mut_VectorOfVec3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVec3f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVec3f_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3f_input_output_array(self.as_raw_mut_VectorOfVec3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfVec3f }
	
	pub type VectorOfVec3i = core::Vector<core::Vec3i>;
	
	impl VectorOfVec3i {
		pub fn as_raw_VectorOfVec3i(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec3i(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec3i, *const c_void, *mut c_void,
		cv_VectorOfVec3i_new, cv_VectorOfVec3i_delete,
		cv_VectorOfVec3i_len, cv_VectorOfVec3i_is_empty,
		cv_VectorOfVec3i_capacity, cv_VectorOfVec3i_shrink_to_fit,
		cv_VectorOfVec3i_reserve, cv_VectorOfVec3i_remove,
		cv_VectorOfVec3i_swap, cv_VectorOfVec3i_clear,
		cv_VectorOfVec3i_get, cv_VectorOfVec3i_set,
		cv_VectorOfVec3i_push, cv_VectorOfVec3i_insert,
	}
	vector_copy_non_bool! { core::Vec3i, *const c_void, *mut c_void,
		cv_VectorOfVec3i_data, cv_VectorOfVec3i_data_mut, cv_VectorOfVec3i_from_slice,
		cv_VectorOfVec3i_clone,
	}
	
	unsafe impl Send for core::Vector<core::Vec3i> {}
	
	impl core::ToInputArray for VectorOfVec3i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec3i_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3i_input_array(self.as_raw_VectorOfVec3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfVec3i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVec3i_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3i_output_array(self.as_raw_mut_VectorOfVec3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVec3i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVec3i_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3i_input_output_array(self.as_raw_mut_VectorOfVec3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfVec3i }
	
	pub type VectorOfVec4f = core::Vector<core::Vec4f>;
	
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
		cv_VectorOfVec4f_get, cv_VectorOfVec4f_set,
		cv_VectorOfVec4f_push, cv_VectorOfVec4f_insert,
	}
	vector_copy_non_bool! { core::Vec4f, *const c_void, *mut c_void,
		cv_VectorOfVec4f_data, cv_VectorOfVec4f_data_mut, cv_VectorOfVec4f_from_slice,
		cv_VectorOfVec4f_clone,
	}
	
	unsafe impl Send for core::Vector<core::Vec4f> {}
	
	impl core::ToInputArray for VectorOfVec4f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec4f_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec4f_input_array(self.as_raw_VectorOfVec4f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfVec4f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVec4f_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec4f_output_array(self.as_raw_mut_VectorOfVec4f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVec4f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVec4f_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec4f_input_output_array(self.as_raw_mut_VectorOfVec4f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfVec4f }
	
	pub type VectorOfVec4i = core::Vector<core::Vec4i>;
	
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
		cv_VectorOfVec4i_get, cv_VectorOfVec4i_set,
		cv_VectorOfVec4i_push, cv_VectorOfVec4i_insert,
	}
	vector_copy_non_bool! { core::Vec4i, *const c_void, *mut c_void,
		cv_VectorOfVec4i_data, cv_VectorOfVec4i_data_mut, cv_VectorOfVec4i_from_slice,
		cv_VectorOfVec4i_clone,
	}
	
	unsafe impl Send for core::Vector<core::Vec4i> {}
	
	impl core::ToInputArray for VectorOfVec4i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec4i_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec4i_input_array(self.as_raw_VectorOfVec4i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfVec4i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVec4i_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec4i_output_array(self.as_raw_mut_VectorOfVec4i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVec4i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVec4i_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec4i_input_output_array(self.as_raw_mut_VectorOfVec4i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfVec4i }
	
	pub type VectorOfVec6f = core::Vector<core::Vec6f>;
	
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
		cv_VectorOfVec6f_get, cv_VectorOfVec6f_set,
		cv_VectorOfVec6f_push, cv_VectorOfVec6f_insert,
	}
	vector_copy_non_bool! { core::Vec6f, *const c_void, *mut c_void,
		cv_VectorOfVec6f_data, cv_VectorOfVec6f_data_mut, cv_VectorOfVec6f_from_slice,
		cv_VectorOfVec6f_clone,
	}
	
	unsafe impl Send for core::Vector<core::Vec6f> {}
	
	impl core::ToInputArray for VectorOfVec6f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVec6f_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec6f_input_array(self.as_raw_VectorOfVec6f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfVec6f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVec6f_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec6f_output_array(self.as_raw_mut_VectorOfVec6f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVec6f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVec6f_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec6f_input_output_array(self.as_raw_mut_VectorOfVec6f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfVec6f }
	
	pub type VectorOfVectorOfDMatch = core::Vector<core::Vector<core::DMatch>>;
	
	impl VectorOfVectorOfDMatch {
		pub fn as_raw_VectorOfVectorOfDMatch(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfDMatch(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::DMatch>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfDMatch_new, cv_VectorOfVectorOfDMatch_delete,
		cv_VectorOfVectorOfDMatch_len, cv_VectorOfVectorOfDMatch_is_empty,
		cv_VectorOfVectorOfDMatch_capacity, cv_VectorOfVectorOfDMatch_shrink_to_fit,
		cv_VectorOfVectorOfDMatch_reserve, cv_VectorOfVectorOfDMatch_remove,
		cv_VectorOfVectorOfDMatch_swap, cv_VectorOfVectorOfDMatch_clear,
		cv_VectorOfVectorOfDMatch_get, cv_VectorOfVectorOfDMatch_set,
		cv_VectorOfVectorOfDMatch_push, cv_VectorOfVectorOfDMatch_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::DMatch> }
	
	unsafe impl Send for core::Vector<core::Vector<core::DMatch>> {}
	
	pub type VectorOfVectorOfKeyPoint = core::Vector<core::Vector<core::KeyPoint>>;
	
	impl VectorOfVectorOfKeyPoint {
		pub fn as_raw_VectorOfVectorOfKeyPoint(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfKeyPoint(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::KeyPoint>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfKeyPoint_new, cv_VectorOfVectorOfKeyPoint_delete,
		cv_VectorOfVectorOfKeyPoint_len, cv_VectorOfVectorOfKeyPoint_is_empty,
		cv_VectorOfVectorOfKeyPoint_capacity, cv_VectorOfVectorOfKeyPoint_shrink_to_fit,
		cv_VectorOfVectorOfKeyPoint_reserve, cv_VectorOfVectorOfKeyPoint_remove,
		cv_VectorOfVectorOfKeyPoint_swap, cv_VectorOfVectorOfKeyPoint_clear,
		cv_VectorOfVectorOfKeyPoint_get, cv_VectorOfVectorOfKeyPoint_set,
		cv_VectorOfVectorOfKeyPoint_push, cv_VectorOfVectorOfKeyPoint_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::KeyPoint> }
	
	unsafe impl Send for core::Vector<core::Vector<core::KeyPoint>> {}
	
	pub type VectorOfVectorOfMat = core::Vector<core::Vector<core::Mat>>;
	
	impl VectorOfVectorOfMat {
		pub fn as_raw_VectorOfVectorOfMat(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfMat(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Mat>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfMat_new, cv_VectorOfVectorOfMat_delete,
		cv_VectorOfVectorOfMat_len, cv_VectorOfVectorOfMat_is_empty,
		cv_VectorOfVectorOfMat_capacity, cv_VectorOfVectorOfMat_shrink_to_fit,
		cv_VectorOfVectorOfMat_reserve, cv_VectorOfVectorOfMat_remove,
		cv_VectorOfVectorOfMat_swap, cv_VectorOfVectorOfMat_clear,
		cv_VectorOfVectorOfMat_get, cv_VectorOfVectorOfMat_set,
		cv_VectorOfVectorOfMat_push, cv_VectorOfVectorOfMat_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Mat> }
	
	unsafe impl Send for core::Vector<core::Vector<core::Mat>> {}
	
	pub type VectorOfVectorOfPoint = core::Vector<core::Vector<core::Point>>;
	
	impl VectorOfVectorOfPoint {
		pub fn as_raw_VectorOfVectorOfPoint(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Point>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfPoint_new, cv_VectorOfVectorOfPoint_delete,
		cv_VectorOfVectorOfPoint_len, cv_VectorOfVectorOfPoint_is_empty,
		cv_VectorOfVectorOfPoint_capacity, cv_VectorOfVectorOfPoint_shrink_to_fit,
		cv_VectorOfVectorOfPoint_reserve, cv_VectorOfVectorOfPoint_remove,
		cv_VectorOfVectorOfPoint_swap, cv_VectorOfVectorOfPoint_clear,
		cv_VectorOfVectorOfPoint_get, cv_VectorOfVectorOfPoint_set,
		cv_VectorOfVectorOfPoint_push, cv_VectorOfVectorOfPoint_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Point> }
	
	unsafe impl Send for core::Vector<core::Vector<core::Point>> {}
	
	impl core::ToInputArray for VectorOfVectorOfPoint {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint_input_array(self.as_raw_VectorOfVectorOfPoint(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfPoint {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint_output_array(self.as_raw_mut_VectorOfVectorOfPoint(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfPoint {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfVectorOfPoint }
	
	pub type VectorOfVectorOfPoint2f = core::Vector<core::Vector<core::Point2f>>;
	
	impl VectorOfVectorOfPoint2f {
		pub fn as_raw_VectorOfVectorOfPoint2f(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint2f(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Point2f>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfPoint2f_new, cv_VectorOfVectorOfPoint2f_delete,
		cv_VectorOfVectorOfPoint2f_len, cv_VectorOfVectorOfPoint2f_is_empty,
		cv_VectorOfVectorOfPoint2f_capacity, cv_VectorOfVectorOfPoint2f_shrink_to_fit,
		cv_VectorOfVectorOfPoint2f_reserve, cv_VectorOfVectorOfPoint2f_remove,
		cv_VectorOfVectorOfPoint2f_swap, cv_VectorOfVectorOfPoint2f_clear,
		cv_VectorOfVectorOfPoint2f_get, cv_VectorOfVectorOfPoint2f_set,
		cv_VectorOfVectorOfPoint2f_push, cv_VectorOfVectorOfPoint2f_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Point2f> }
	
	unsafe impl Send for core::Vector<core::Vector<core::Point2f>> {}
	
	impl core::ToInputArray for VectorOfVectorOfPoint2f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint2f_input_array(self.as_raw_VectorOfVectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfPoint2f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint2f_output_array(self.as_raw_mut_VectorOfVectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfPoint2f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint2f_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint2f_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfVectorOfPoint2f }
	
	pub type VectorOfVectorOfPoint3d = core::Vector<core::Vector<core::Point3d>>;
	
	impl VectorOfVectorOfPoint3d {
		pub fn as_raw_VectorOfVectorOfPoint3d(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint3d(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Point3d>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfPoint3d_new, cv_VectorOfVectorOfPoint3d_delete,
		cv_VectorOfVectorOfPoint3d_len, cv_VectorOfVectorOfPoint3d_is_empty,
		cv_VectorOfVectorOfPoint3d_capacity, cv_VectorOfVectorOfPoint3d_shrink_to_fit,
		cv_VectorOfVectorOfPoint3d_reserve, cv_VectorOfVectorOfPoint3d_remove,
		cv_VectorOfVectorOfPoint3d_swap, cv_VectorOfVectorOfPoint3d_clear,
		cv_VectorOfVectorOfPoint3d_get, cv_VectorOfVectorOfPoint3d_set,
		cv_VectorOfVectorOfPoint3d_push, cv_VectorOfVectorOfPoint3d_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Point3d> }
	
	unsafe impl Send for core::Vector<core::Vector<core::Point3d>> {}
	
	impl core::ToInputArray for VectorOfVectorOfPoint3d {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3d_input_array(self.as_raw_VectorOfVectorOfPoint3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfPoint3d {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3d_output_array(self.as_raw_mut_VectorOfVectorOfPoint3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfPoint3d {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3d_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3d_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfVectorOfPoint3d }
	
	pub type VectorOfVectorOfPoint3f = core::Vector<core::Vector<core::Point3f>>;
	
	impl VectorOfVectorOfPoint3f {
		pub fn as_raw_VectorOfVectorOfPoint3f(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint3f(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Point3f>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfPoint3f_new, cv_VectorOfVectorOfPoint3f_delete,
		cv_VectorOfVectorOfPoint3f_len, cv_VectorOfVectorOfPoint3f_is_empty,
		cv_VectorOfVectorOfPoint3f_capacity, cv_VectorOfVectorOfPoint3f_shrink_to_fit,
		cv_VectorOfVectorOfPoint3f_reserve, cv_VectorOfVectorOfPoint3f_remove,
		cv_VectorOfVectorOfPoint3f_swap, cv_VectorOfVectorOfPoint3f_clear,
		cv_VectorOfVectorOfPoint3f_get, cv_VectorOfVectorOfPoint3f_set,
		cv_VectorOfVectorOfPoint3f_push, cv_VectorOfVectorOfPoint3f_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Point3f> }
	
	unsafe impl Send for core::Vector<core::Vector<core::Point3f>> {}
	
	impl core::ToInputArray for VectorOfVectorOfPoint3f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3f_input_array(self.as_raw_VectorOfVectorOfPoint3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfPoint3f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3f_output_array(self.as_raw_mut_VectorOfVectorOfPoint3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfPoint3f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3f_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3f_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfVectorOfPoint3f }
	
	pub type VectorOfVectorOfPoint3i = core::Vector<core::Vector<core::Point3i>>;
	
	impl VectorOfVectorOfPoint3i {
		pub fn as_raw_VectorOfVectorOfPoint3i(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint3i(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Point3i>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfPoint3i_new, cv_VectorOfVectorOfPoint3i_delete,
		cv_VectorOfVectorOfPoint3i_len, cv_VectorOfVectorOfPoint3i_is_empty,
		cv_VectorOfVectorOfPoint3i_capacity, cv_VectorOfVectorOfPoint3i_shrink_to_fit,
		cv_VectorOfVectorOfPoint3i_reserve, cv_VectorOfVectorOfPoint3i_remove,
		cv_VectorOfVectorOfPoint3i_swap, cv_VectorOfVectorOfPoint3i_clear,
		cv_VectorOfVectorOfPoint3i_get, cv_VectorOfVectorOfPoint3i_set,
		cv_VectorOfVectorOfPoint3i_push, cv_VectorOfVectorOfPoint3i_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Point3i> }
	
	unsafe impl Send for core::Vector<core::Vector<core::Point3i>> {}
	
	impl core::ToInputArray for VectorOfVectorOfPoint3i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3i_input_array(self.as_raw_VectorOfVectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfPoint3i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3i_output_array(self.as_raw_mut_VectorOfVectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfPoint3i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfPoint3i_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3i_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfVectorOfPoint3i }
	
	pub type VectorOfVectorOfRange = core::Vector<core::Vector<core::Range>>;
	
	impl VectorOfVectorOfRange {
		pub fn as_raw_VectorOfVectorOfRange(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfRange(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Range>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfRange_new, cv_VectorOfVectorOfRange_delete,
		cv_VectorOfVectorOfRange_len, cv_VectorOfVectorOfRange_is_empty,
		cv_VectorOfVectorOfRange_capacity, cv_VectorOfVectorOfRange_shrink_to_fit,
		cv_VectorOfVectorOfRange_reserve, cv_VectorOfVectorOfRange_remove,
		cv_VectorOfVectorOfRange_swap, cv_VectorOfVectorOfRange_clear,
		cv_VectorOfVectorOfRange_get, cv_VectorOfVectorOfRange_set,
		cv_VectorOfVectorOfRange_push, cv_VectorOfVectorOfRange_insert,
	}
	vector_non_copy_or_bool! { core::Vector<core::Range> }
	
	unsafe impl Send for core::Vector<core::Vector<core::Range>> {}
	
	pub type VectorOfVectorOfVec2i = core::Vector<core::Vector<core::Vec2i>>;
	
	impl VectorOfVectorOfVec2i {
		pub fn as_raw_VectorOfVectorOfVec2i(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfVec2i(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Vec2i>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfVec2i_new, cv_VectorOfVectorOfVec2i_delete,
		cv_VectorOfVectorOfVec2i_len, cv_VectorOfVectorOfVec2i_is_empty,
		cv_VectorOfVectorOfVec2i_capacity, cv_VectorOfVectorOfVec2i_shrink_to_fit,
		cv_VectorOfVectorOfVec2i_reserve, cv_VectorOfVectorOfVec2i_remove,
		cv_VectorOfVectorOfVec2i_swap, cv_VectorOfVectorOfVec2i_clear,
		cv_VectorOfVectorOfVec2i_get, cv_VectorOfVectorOfVec2i_set,
		cv_VectorOfVectorOfVec2i_push, cv_VectorOfVectorOfVec2i_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Vec2i> }
	
	unsafe impl Send for core::Vector<core::Vector<core::Vec2i>> {}
	
	impl core::ToInputArray for VectorOfVectorOfVec2i {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfVec2i_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfVec2i_input_array(self.as_raw_VectorOfVectorOfVec2i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfVec2i {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfVec2i_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfVec2i_output_array(self.as_raw_mut_VectorOfVectorOfVec2i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfVec2i {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfVec2i_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfVec2i_input_output_array(self.as_raw_mut_VectorOfVectorOfVec2i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfVectorOfVec2i }
	
	pub type VectorOfVectorOfVec3f = core::Vector<core::Vector<core::Vec3f>>;
	
	impl VectorOfVectorOfVec3f {
		pub fn as_raw_VectorOfVectorOfVec3f(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfVec3f(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Vec3f>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfVec3f_new, cv_VectorOfVectorOfVec3f_delete,
		cv_VectorOfVectorOfVec3f_len, cv_VectorOfVectorOfVec3f_is_empty,
		cv_VectorOfVectorOfVec3f_capacity, cv_VectorOfVectorOfVec3f_shrink_to_fit,
		cv_VectorOfVectorOfVec3f_reserve, cv_VectorOfVectorOfVec3f_remove,
		cv_VectorOfVectorOfVec3f_swap, cv_VectorOfVectorOfVec3f_clear,
		cv_VectorOfVectorOfVec3f_get, cv_VectorOfVectorOfVec3f_set,
		cv_VectorOfVectorOfVec3f_push, cv_VectorOfVectorOfVec3f_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Vec3f> }
	
	unsafe impl Send for core::Vector<core::Vector<core::Vec3f>> {}
	
	impl core::ToInputArray for VectorOfVectorOfVec3f {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfVec3f_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfVec3f_input_array(self.as_raw_VectorOfVectorOfVec3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfVec3f {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfVec3f_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfVec3f_output_array(self.as_raw_mut_VectorOfVectorOfVec3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfVec3f {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfVec3f_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfVec3f_input_output_array(self.as_raw_mut_VectorOfVectorOfVec3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfVectorOfVec3f }
	
	pub type VectorOfVectorOff32 = core::Vector<core::Vector<f32>>;
	
	impl VectorOfVectorOff32 {
		pub fn as_raw_VectorOfVectorOff32(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOff32(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<f32>, *const c_void, *mut c_void,
		cv_VectorOfVectorOff32_new, cv_VectorOfVectorOff32_delete,
		cv_VectorOfVectorOff32_len, cv_VectorOfVectorOff32_is_empty,
		cv_VectorOfVectorOff32_capacity, cv_VectorOfVectorOff32_shrink_to_fit,
		cv_VectorOfVectorOff32_reserve, cv_VectorOfVectorOff32_remove,
		cv_VectorOfVectorOff32_swap, cv_VectorOfVectorOff32_clear,
		cv_VectorOfVectorOff32_get, cv_VectorOfVectorOff32_set,
		cv_VectorOfVectorOff32_push, cv_VectorOfVectorOff32_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<f32> }
	
	unsafe impl Send for core::Vector<core::Vector<f32>> {}
	
	impl core::ToInputArray for VectorOfVectorOff32 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOff32_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOff32_input_array(self.as_raw_VectorOfVectorOff32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOff32 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOff32_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOff32_output_array(self.as_raw_mut_VectorOfVectorOff32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOff32 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOff32_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOff32_input_output_array(self.as_raw_mut_VectorOfVectorOff32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfVectorOff32 }
	
	pub type VectorOfVectorOff64 = core::Vector<core::Vector<f64>>;
	
	impl VectorOfVectorOff64 {
		pub fn as_raw_VectorOfVectorOff64(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOff64(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<f64>, *const c_void, *mut c_void,
		cv_VectorOfVectorOff64_new, cv_VectorOfVectorOff64_delete,
		cv_VectorOfVectorOff64_len, cv_VectorOfVectorOff64_is_empty,
		cv_VectorOfVectorOff64_capacity, cv_VectorOfVectorOff64_shrink_to_fit,
		cv_VectorOfVectorOff64_reserve, cv_VectorOfVectorOff64_remove,
		cv_VectorOfVectorOff64_swap, cv_VectorOfVectorOff64_clear,
		cv_VectorOfVectorOff64_get, cv_VectorOfVectorOff64_set,
		cv_VectorOfVectorOff64_push, cv_VectorOfVectorOff64_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<f64> }
	
	unsafe impl Send for core::Vector<core::Vector<f64>> {}
	
	impl core::ToInputArray for VectorOfVectorOff64 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOff64_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOff64_input_array(self.as_raw_VectorOfVectorOff64(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOff64 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOff64_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOff64_output_array(self.as_raw_mut_VectorOfVectorOff64(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOff64 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOff64_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOff64_input_output_array(self.as_raw_mut_VectorOfVectorOff64(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfVectorOff64 }
	
	pub type VectorOfVectorOfi32 = core::Vector<core::Vector<i32>>;
	
	impl VectorOfVectorOfi32 {
		pub fn as_raw_VectorOfVectorOfi32(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfi32(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<i32>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfi32_new, cv_VectorOfVectorOfi32_delete,
		cv_VectorOfVectorOfi32_len, cv_VectorOfVectorOfi32_is_empty,
		cv_VectorOfVectorOfi32_capacity, cv_VectorOfVectorOfi32_shrink_to_fit,
		cv_VectorOfVectorOfi32_reserve, cv_VectorOfVectorOfi32_remove,
		cv_VectorOfVectorOfi32_swap, cv_VectorOfVectorOfi32_clear,
		cv_VectorOfVectorOfi32_get, cv_VectorOfVectorOfi32_set,
		cv_VectorOfVectorOfi32_push, cv_VectorOfVectorOfi32_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<i32> }
	
	unsafe impl Send for core::Vector<core::Vector<i32>> {}
	
	impl core::ToInputArray for VectorOfVectorOfi32 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfi32_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfi32_input_array(self.as_raw_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfi32 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfi32_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfi32_output_array(self.as_raw_mut_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfi32 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfi32_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfi32_input_output_array(self.as_raw_mut_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfVectorOfi32 }
	
	pub type VectorOfVectorOfi8 = core::Vector<core::Vector<i8>>;
	
	impl VectorOfVectorOfi8 {
		pub fn as_raw_VectorOfVectorOfi8(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfi8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<i8>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfi8_new, cv_VectorOfVectorOfi8_delete,
		cv_VectorOfVectorOfi8_len, cv_VectorOfVectorOfi8_is_empty,
		cv_VectorOfVectorOfi8_capacity, cv_VectorOfVectorOfi8_shrink_to_fit,
		cv_VectorOfVectorOfi8_reserve, cv_VectorOfVectorOfi8_remove,
		cv_VectorOfVectorOfi8_swap, cv_VectorOfVectorOfi8_clear,
		cv_VectorOfVectorOfi8_get, cv_VectorOfVectorOfi8_set,
		cv_VectorOfVectorOfi8_push, cv_VectorOfVectorOfi8_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<i8> }
	
	unsafe impl Send for core::Vector<core::Vector<i8>> {}
	
	impl core::ToInputArray for VectorOfVectorOfi8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfi8_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfi8_input_array(self.as_raw_VectorOfVectorOfi8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfi8 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfi8_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfi8_output_array(self.as_raw_mut_VectorOfVectorOfi8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfi8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfi8_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfi8_input_output_array(self.as_raw_mut_VectorOfVectorOfi8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfVectorOfi8 }
	
	pub type VectorOfVectorOfu8 = core::Vector<core::Vector<u8>>;
	
	impl VectorOfVectorOfu8 {
		pub fn as_raw_VectorOfVectorOfu8(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfu8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<u8>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfu8_new, cv_VectorOfVectorOfu8_delete,
		cv_VectorOfVectorOfu8_len, cv_VectorOfVectorOfu8_is_empty,
		cv_VectorOfVectorOfu8_capacity, cv_VectorOfVectorOfu8_shrink_to_fit,
		cv_VectorOfVectorOfu8_reserve, cv_VectorOfVectorOfu8_remove,
		cv_VectorOfVectorOfu8_swap, cv_VectorOfVectorOfu8_clear,
		cv_VectorOfVectorOfu8_get, cv_VectorOfVectorOfu8_set,
		cv_VectorOfVectorOfu8_push, cv_VectorOfVectorOfu8_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<u8> }
	
	unsafe impl Send for core::Vector<core::Vector<u8>> {}
	
	impl core::ToInputArray for VectorOfVectorOfu8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfVectorOfu8_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfu8_input_array(self.as_raw_VectorOfVectorOfu8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfVectorOfu8 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfVectorOfu8_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfu8_output_array(self.as_raw_mut_VectorOfVectorOfu8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfVectorOfu8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfVectorOfu8_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfu8_input_output_array(self.as_raw_mut_VectorOfVectorOfu8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfVectorOfu8 }
	
	pub type VectorOfbool = core::Vector<bool>;
	
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
		cv_VectorOfbool_get, cv_VectorOfbool_set,
		cv_VectorOfbool_push, cv_VectorOfbool_insert,
	}
	vector_non_copy_or_bool! { clone bool }
	
	unsafe impl Send for core::Vector<bool> {}
	
	pub type VectorOff32 = core::Vector<f32>;
	
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
		cv_VectorOff32_get, cv_VectorOff32_set,
		cv_VectorOff32_push, cv_VectorOff32_insert,
	}
	vector_copy_non_bool! { f32, *const c_void, *mut c_void,
		cv_VectorOff32_data, cv_VectorOff32_data_mut, cv_VectorOff32_from_slice,
		cv_VectorOff32_clone,
	}
	
	unsafe impl Send for core::Vector<f32> {}
	
	impl core::ToInputArray for VectorOff32 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOff32_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOff32_input_array(self.as_raw_VectorOff32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOff32 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOff32_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOff32_output_array(self.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOff32 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOff32_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOff32_input_output_array(self.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOff32 }
	
	pub type VectorOff64 = core::Vector<f64>;
	
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
		cv_VectorOff64_get, cv_VectorOff64_set,
		cv_VectorOff64_push, cv_VectorOff64_insert,
	}
	vector_copy_non_bool! { f64, *const c_void, *mut c_void,
		cv_VectorOff64_data, cv_VectorOff64_data_mut, cv_VectorOff64_from_slice,
		cv_VectorOff64_clone,
	}
	
	unsafe impl Send for core::Vector<f64> {}
	
	impl core::ToInputArray for VectorOff64 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOff64_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOff64_input_array(self.as_raw_VectorOff64(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOff64 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOff64_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOff64_output_array(self.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOff64 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOff64_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOff64_input_output_array(self.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOff64 }
	
	pub type VectorOfi32 = core::Vector<i32>;
	
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
		cv_VectorOfi32_get, cv_VectorOfi32_set,
		cv_VectorOfi32_push, cv_VectorOfi32_insert,
	}
	vector_copy_non_bool! { i32, *const c_void, *mut c_void,
		cv_VectorOfi32_data, cv_VectorOfi32_data_mut, cv_VectorOfi32_from_slice,
		cv_VectorOfi32_clone,
	}
	
	unsafe impl Send for core::Vector<i32> {}
	
	impl core::ToInputArray for VectorOfi32 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfi32_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfi32_input_array(self.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfi32 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfi32_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfi32_output_array(self.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfi32 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfi32_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfi32_input_output_array(self.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfi32 }
	
	pub type VectorOfi8 = core::Vector<i8>;
	
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
		cv_VectorOfi8_get, cv_VectorOfi8_set,
		cv_VectorOfi8_push, cv_VectorOfi8_insert,
	}
	vector_copy_non_bool! { i8, *const c_void, *mut c_void,
		cv_VectorOfi8_data, cv_VectorOfi8_data_mut, cv_VectorOfi8_from_slice,
		cv_VectorOfi8_clone,
	}
	
	unsafe impl Send for core::Vector<i8> {}
	
	impl core::ToInputArray for VectorOfi8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfi8_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfi8_input_array(self.as_raw_VectorOfi8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfi8 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfi8_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfi8_output_array(self.as_raw_mut_VectorOfi8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfi8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfi8_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfi8_input_output_array(self.as_raw_mut_VectorOfi8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfi8 }
	
	pub type VectorOfsize_t = core::Vector<size_t>;
	
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
		cv_VectorOfsize_t_get, cv_VectorOfsize_t_set,
		cv_VectorOfsize_t_push, cv_VectorOfsize_t_insert,
	}
	vector_copy_non_bool! { size_t, *const c_void, *mut c_void,
		cv_VectorOfsize_t_data, cv_VectorOfsize_t_data_mut, cv_VectorOfsize_t_from_slice,
		cv_VectorOfsize_t_clone,
	}
	
	unsafe impl Send for core::Vector<size_t> {}
	
	pub type VectorOfu8 = core::Vector<u8>;
	
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
		cv_VectorOfu8_get, cv_VectorOfu8_set,
		cv_VectorOfu8_push, cv_VectorOfu8_insert,
	}
	vector_copy_non_bool! { u8, *const c_void, *mut c_void,
		cv_VectorOfu8_data, cv_VectorOfu8_data_mut, cv_VectorOfu8_from_slice,
		cv_VectorOfu8_clone,
	}
	
	unsafe impl Send for core::Vector<u8> {}
	
	impl core::ToInputArray for VectorOfu8 {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			extern "C" { fn cv_VectorOfu8_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfu8_input_array(self.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for VectorOfu8 {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			extern "C" { fn cv_VectorOfu8_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfu8_output_array(self.as_raw_mut_VectorOfu8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for VectorOfu8 {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			extern "C" { fn cv_VectorOfu8_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfu8_input_output_array(self.as_raw_mut_VectorOfu8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { VectorOfu8 }
	
}
#[cfg(ocvrs_has_module_core)]
pub use core_types::*;

#[cfg(ocvrs_has_module_cudaarithm)]
mod cudaarithm_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfConvolution = core::Ptr<dyn crate::cudaarithm::Convolution>;
	
	ptr_extern! { dyn crate::cudaarithm::Convolution,
		cv_PtrOfConvolution_delete, cv_PtrOfConvolution_get_inner_ptr, cv_PtrOfConvolution_get_inner_ptr_mut
	}
	
	impl PtrOfConvolution {
		#[inline] pub fn as_raw_PtrOfConvolution(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConvolution(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaarithm::ConvolutionConst for PtrOfConvolution {
		#[inline] fn as_raw_Convolution(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaarithm::Convolution for PtrOfConvolution {
		#[inline] fn as_raw_mut_Convolution(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfConvolution {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfConvolution {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDFT = core::Ptr<dyn crate::cudaarithm::DFT>;
	
	ptr_extern! { dyn crate::cudaarithm::DFT,
		cv_PtrOfDFT_delete, cv_PtrOfDFT_get_inner_ptr, cv_PtrOfDFT_get_inner_ptr_mut
	}
	
	impl PtrOfDFT {
		#[inline] pub fn as_raw_PtrOfDFT(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDFT(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaarithm::DFTConst for PtrOfDFT {
		#[inline] fn as_raw_DFT(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaarithm::DFT for PtrOfDFT {
		#[inline] fn as_raw_mut_DFT(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfDFT {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfDFT {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLookUpTable = core::Ptr<dyn crate::cudaarithm::LookUpTable>;
	
	ptr_extern! { dyn crate::cudaarithm::LookUpTable,
		cv_PtrOfLookUpTable_delete, cv_PtrOfLookUpTable_get_inner_ptr, cv_PtrOfLookUpTable_get_inner_ptr_mut
	}
	
	impl PtrOfLookUpTable {
		#[inline] pub fn as_raw_PtrOfLookUpTable(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLookUpTable(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaarithm::LookUpTableConst for PtrOfLookUpTable {
		#[inline] fn as_raw_LookUpTable(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaarithm::LookUpTable for PtrOfLookUpTable {
		#[inline] fn as_raw_mut_LookUpTable(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfLookUpTable {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfLookUpTable {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_cudaarithm)]
pub use cudaarithm_types::*;

#[cfg(ocvrs_has_module_cudabgsegm)]
mod cudabgsegm_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCUDA_BackgroundSubtractorMOG = core::Ptr<dyn crate::cudabgsegm::CUDA_BackgroundSubtractorMOG>;
	
	ptr_extern! { dyn crate::cudabgsegm::CUDA_BackgroundSubtractorMOG,
		cv_PtrOfCUDA_BackgroundSubtractorMOG_delete, cv_PtrOfCUDA_BackgroundSubtractorMOG_get_inner_ptr, cv_PtrOfCUDA_BackgroundSubtractorMOG_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_BackgroundSubtractorMOG {
		#[inline] pub fn as_raw_PtrOfCUDA_BackgroundSubtractorMOG(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_BackgroundSubtractorMOG(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudabgsegm::CUDA_BackgroundSubtractorMOGConst for PtrOfCUDA_BackgroundSubtractorMOG {
		#[inline] fn as_raw_CUDA_BackgroundSubtractorMOG(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudabgsegm::CUDA_BackgroundSubtractorMOG for PtrOfCUDA_BackgroundSubtractorMOG {
		#[inline] fn as_raw_mut_CUDA_BackgroundSubtractorMOG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_BackgroundSubtractorMOG {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_BackgroundSubtractorMOG {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorConst for PtrOfCUDA_BackgroundSubtractorMOG {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfCUDA_BackgroundSubtractorMOG {
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_BackgroundSubtractorMOG2 = core::Ptr<dyn crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2>;
	
	ptr_extern! { dyn crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2,
		cv_PtrOfCUDA_BackgroundSubtractorMOG2_delete, cv_PtrOfCUDA_BackgroundSubtractorMOG2_get_inner_ptr, cv_PtrOfCUDA_BackgroundSubtractorMOG2_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_BackgroundSubtractorMOG2 {
		#[inline] pub fn as_raw_PtrOfCUDA_BackgroundSubtractorMOG2(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_BackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2Const for PtrOfCUDA_BackgroundSubtractorMOG2 {
		#[inline] fn as_raw_CUDA_BackgroundSubtractorMOG2(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2 for PtrOfCUDA_BackgroundSubtractorMOG2 {
		#[inline] fn as_raw_mut_CUDA_BackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_BackgroundSubtractorMOG2 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_BackgroundSubtractorMOG2 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorConst for PtrOfCUDA_BackgroundSubtractorMOG2 {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfCUDA_BackgroundSubtractorMOG2 {
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorMOG2Const for PtrOfCUDA_BackgroundSubtractorMOG2 {
		#[inline] fn as_raw_BackgroundSubtractorMOG2(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractorMOG2 for PtrOfCUDA_BackgroundSubtractorMOG2 {
		#[inline] fn as_raw_mut_BackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_cudabgsegm)]
pub use cudabgsegm_types::*;

#[cfg(ocvrs_has_module_cudacodec)]
mod cudacodec_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfEncoderCallBack = core::Ptr<dyn crate::cudacodec::EncoderCallBack>;
	
	ptr_extern! { dyn crate::cudacodec::EncoderCallBack,
		cv_PtrOfEncoderCallBack_delete, cv_PtrOfEncoderCallBack_get_inner_ptr, cv_PtrOfEncoderCallBack_get_inner_ptr_mut
	}
	
	impl PtrOfEncoderCallBack {
		#[inline] pub fn as_raw_PtrOfEncoderCallBack(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEncoderCallBack(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudacodec::EncoderCallBackConst for PtrOfEncoderCallBack {
		#[inline] fn as_raw_EncoderCallBack(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudacodec::EncoderCallBack for PtrOfEncoderCallBack {
		#[inline] fn as_raw_mut_EncoderCallBack(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRawVideoSource = core::Ptr<dyn crate::cudacodec::RawVideoSource>;
	
	ptr_extern! { dyn crate::cudacodec::RawVideoSource,
		cv_PtrOfRawVideoSource_delete, cv_PtrOfRawVideoSource_get_inner_ptr, cv_PtrOfRawVideoSource_get_inner_ptr_mut
	}
	
	impl PtrOfRawVideoSource {
		#[inline] pub fn as_raw_PtrOfRawVideoSource(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRawVideoSource(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudacodec::RawVideoSourceConst for PtrOfRawVideoSource {
		#[inline] fn as_raw_RawVideoSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudacodec::RawVideoSource for PtrOfRawVideoSource {
		#[inline] fn as_raw_mut_RawVideoSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfVideoReader = core::Ptr<dyn crate::cudacodec::VideoReader>;
	
	ptr_extern! { dyn crate::cudacodec::VideoReader,
		cv_PtrOfVideoReader_delete, cv_PtrOfVideoReader_get_inner_ptr, cv_PtrOfVideoReader_get_inner_ptr_mut
	}
	
	impl PtrOfVideoReader {
		#[inline] pub fn as_raw_PtrOfVideoReader(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfVideoReader(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudacodec::VideoReaderConst for PtrOfVideoReader {
		#[inline] fn as_raw_VideoReader(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudacodec::VideoReader for PtrOfVideoReader {
		#[inline] fn as_raw_mut_VideoReader(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfVideoWriter = core::Ptr<dyn crate::cudacodec::VideoWriter>;
	
	ptr_extern! { dyn crate::cudacodec::VideoWriter,
		cv_PtrOfVideoWriter_delete, cv_PtrOfVideoWriter_get_inner_ptr, cv_PtrOfVideoWriter_get_inner_ptr_mut
	}
	
	impl PtrOfVideoWriter {
		#[inline] pub fn as_raw_PtrOfVideoWriter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfVideoWriter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudacodec::VideoWriterConst for PtrOfVideoWriter {
		#[inline] fn as_raw_VideoWriter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudacodec::VideoWriter for PtrOfVideoWriter {
		#[inline] fn as_raw_mut_VideoWriter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_cudacodec)]
pub use cudacodec_types::*;

#[cfg(ocvrs_has_module_cudafeatures2d)]
mod cudafeatures2d_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCUDA_DescriptorMatcher = core::Ptr<dyn crate::cudafeatures2d::CUDA_DescriptorMatcher>;
	
	ptr_extern! { dyn crate::cudafeatures2d::CUDA_DescriptorMatcher,
		cv_PtrOfCUDA_DescriptorMatcher_delete, cv_PtrOfCUDA_DescriptorMatcher_get_inner_ptr, cv_PtrOfCUDA_DescriptorMatcher_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_DescriptorMatcher {
		#[inline] pub fn as_raw_PtrOfCUDA_DescriptorMatcher(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_DescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudafeatures2d::CUDA_DescriptorMatcherConst for PtrOfCUDA_DescriptorMatcher {
		#[inline] fn as_raw_CUDA_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudafeatures2d::CUDA_DescriptorMatcher for PtrOfCUDA_DescriptorMatcher {
		#[inline] fn as_raw_mut_CUDA_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_DescriptorMatcher {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_DescriptorMatcher {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_FastFeatureDetector = core::Ptr<dyn crate::cudafeatures2d::CUDA_FastFeatureDetector>;
	
	ptr_extern! { dyn crate::cudafeatures2d::CUDA_FastFeatureDetector,
		cv_PtrOfCUDA_FastFeatureDetector_delete, cv_PtrOfCUDA_FastFeatureDetector_get_inner_ptr, cv_PtrOfCUDA_FastFeatureDetector_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_FastFeatureDetector {
		#[inline] pub fn as_raw_PtrOfCUDA_FastFeatureDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_FastFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudafeatures2d::CUDA_FastFeatureDetectorConst for PtrOfCUDA_FastFeatureDetector {
		#[inline] fn as_raw_CUDA_FastFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudafeatures2d::CUDA_FastFeatureDetector for PtrOfCUDA_FastFeatureDetector {
		#[inline] fn as_raw_mut_CUDA_FastFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_FastFeatureDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_FastFeatureDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfCUDA_FastFeatureDetector {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfCUDA_FastFeatureDetector {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudafeatures2d::CUDA_Feature2DAsyncConst for PtrOfCUDA_FastFeatureDetector {
		#[inline] fn as_raw_CUDA_Feature2DAsync(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudafeatures2d::CUDA_Feature2DAsync for PtrOfCUDA_FastFeatureDetector {
		#[inline] fn as_raw_mut_CUDA_Feature2DAsync(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_ORB = core::Ptr<dyn crate::cudafeatures2d::CUDA_ORB>;
	
	ptr_extern! { dyn crate::cudafeatures2d::CUDA_ORB,
		cv_PtrOfCUDA_ORB_delete, cv_PtrOfCUDA_ORB_get_inner_ptr, cv_PtrOfCUDA_ORB_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_ORB {
		#[inline] pub fn as_raw_PtrOfCUDA_ORB(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_ORB(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudafeatures2d::CUDA_ORBConst for PtrOfCUDA_ORB {
		#[inline] fn as_raw_CUDA_ORB(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudafeatures2d::CUDA_ORB for PtrOfCUDA_ORB {
		#[inline] fn as_raw_mut_CUDA_ORB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_ORB {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_ORB {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfCUDA_ORB {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfCUDA_ORB {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudafeatures2d::CUDA_Feature2DAsyncConst for PtrOfCUDA_ORB {
		#[inline] fn as_raw_CUDA_Feature2DAsync(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudafeatures2d::CUDA_Feature2DAsync for PtrOfCUDA_ORB {
		#[inline] fn as_raw_mut_CUDA_Feature2DAsync(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_cudafeatures2d)]
pub use cudafeatures2d_types::*;

#[cfg(ocvrs_has_module_cudafilters)]
mod cudafilters_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfFilter = core::Ptr<dyn crate::cudafilters::Filter>;
	
	ptr_extern! { dyn crate::cudafilters::Filter,
		cv_PtrOfFilter_delete, cv_PtrOfFilter_get_inner_ptr, cv_PtrOfFilter_get_inner_ptr_mut
	}
	
	impl PtrOfFilter {
		#[inline] pub fn as_raw_PtrOfFilter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudafilters::FilterConst for PtrOfFilter {
		#[inline] fn as_raw_Filter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudafilters::Filter for PtrOfFilter {
		#[inline] fn as_raw_mut_Filter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfFilter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfFilter {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_cudafilters)]
pub use cudafilters_types::*;

#[cfg(ocvrs_has_module_cudaimgproc)]
mod cudaimgproc_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCUDA_CLAHE = core::Ptr<dyn crate::cudaimgproc::CUDA_CLAHE>;
	
	ptr_extern! { dyn crate::cudaimgproc::CUDA_CLAHE,
		cv_PtrOfCUDA_CLAHE_delete, cv_PtrOfCUDA_CLAHE_get_inner_ptr, cv_PtrOfCUDA_CLAHE_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_CLAHE {
		#[inline] pub fn as_raw_PtrOfCUDA_CLAHE(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_CLAHE(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_CLAHEConst for PtrOfCUDA_CLAHE {
		#[inline] fn as_raw_CUDA_CLAHE(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaimgproc::CUDA_CLAHE for PtrOfCUDA_CLAHE {
		#[inline] fn as_raw_mut_CUDA_CLAHE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_CLAHE {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_CLAHE {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::imgproc::CLAHEConst for PtrOfCUDA_CLAHE {
		#[inline] fn as_raw_CLAHE(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::imgproc::CLAHE for PtrOfCUDA_CLAHE {
		#[inline] fn as_raw_mut_CLAHE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_CannyEdgeDetector = core::Ptr<dyn crate::cudaimgproc::CUDA_CannyEdgeDetector>;
	
	ptr_extern! { dyn crate::cudaimgproc::CUDA_CannyEdgeDetector,
		cv_PtrOfCUDA_CannyEdgeDetector_delete, cv_PtrOfCUDA_CannyEdgeDetector_get_inner_ptr, cv_PtrOfCUDA_CannyEdgeDetector_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_CannyEdgeDetector {
		#[inline] pub fn as_raw_PtrOfCUDA_CannyEdgeDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_CannyEdgeDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_CannyEdgeDetectorConst for PtrOfCUDA_CannyEdgeDetector {
		#[inline] fn as_raw_CUDA_CannyEdgeDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaimgproc::CUDA_CannyEdgeDetector for PtrOfCUDA_CannyEdgeDetector {
		#[inline] fn as_raw_mut_CUDA_CannyEdgeDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_CannyEdgeDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_CannyEdgeDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_CornernessCriteria = core::Ptr<dyn crate::cudaimgproc::CUDA_CornernessCriteria>;
	
	ptr_extern! { dyn crate::cudaimgproc::CUDA_CornernessCriteria,
		cv_PtrOfCUDA_CornernessCriteria_delete, cv_PtrOfCUDA_CornernessCriteria_get_inner_ptr, cv_PtrOfCUDA_CornernessCriteria_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_CornernessCriteria {
		#[inline] pub fn as_raw_PtrOfCUDA_CornernessCriteria(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_CornernessCriteria(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_CornernessCriteriaConst for PtrOfCUDA_CornernessCriteria {
		#[inline] fn as_raw_CUDA_CornernessCriteria(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaimgproc::CUDA_CornernessCriteria for PtrOfCUDA_CornernessCriteria {
		#[inline] fn as_raw_mut_CUDA_CornernessCriteria(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_CornernessCriteria {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_CornernessCriteria {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_CornersDetector = core::Ptr<dyn crate::cudaimgproc::CUDA_CornersDetector>;
	
	ptr_extern! { dyn crate::cudaimgproc::CUDA_CornersDetector,
		cv_PtrOfCUDA_CornersDetector_delete, cv_PtrOfCUDA_CornersDetector_get_inner_ptr, cv_PtrOfCUDA_CornersDetector_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_CornersDetector {
		#[inline] pub fn as_raw_PtrOfCUDA_CornersDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_CornersDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_CornersDetectorConst for PtrOfCUDA_CornersDetector {
		#[inline] fn as_raw_CUDA_CornersDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaimgproc::CUDA_CornersDetector for PtrOfCUDA_CornersDetector {
		#[inline] fn as_raw_mut_CUDA_CornersDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_CornersDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_CornersDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_HoughCirclesDetector = core::Ptr<dyn crate::cudaimgproc::CUDA_HoughCirclesDetector>;
	
	ptr_extern! { dyn crate::cudaimgproc::CUDA_HoughCirclesDetector,
		cv_PtrOfCUDA_HoughCirclesDetector_delete, cv_PtrOfCUDA_HoughCirclesDetector_get_inner_ptr, cv_PtrOfCUDA_HoughCirclesDetector_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_HoughCirclesDetector {
		#[inline] pub fn as_raw_PtrOfCUDA_HoughCirclesDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_HoughCirclesDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_HoughCirclesDetectorConst for PtrOfCUDA_HoughCirclesDetector {
		#[inline] fn as_raw_CUDA_HoughCirclesDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaimgproc::CUDA_HoughCirclesDetector for PtrOfCUDA_HoughCirclesDetector {
		#[inline] fn as_raw_mut_CUDA_HoughCirclesDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_HoughCirclesDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_HoughCirclesDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_HoughLinesDetector = core::Ptr<dyn crate::cudaimgproc::CUDA_HoughLinesDetector>;
	
	ptr_extern! { dyn crate::cudaimgproc::CUDA_HoughLinesDetector,
		cv_PtrOfCUDA_HoughLinesDetector_delete, cv_PtrOfCUDA_HoughLinesDetector_get_inner_ptr, cv_PtrOfCUDA_HoughLinesDetector_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_HoughLinesDetector {
		#[inline] pub fn as_raw_PtrOfCUDA_HoughLinesDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_HoughLinesDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_HoughLinesDetectorConst for PtrOfCUDA_HoughLinesDetector {
		#[inline] fn as_raw_CUDA_HoughLinesDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaimgproc::CUDA_HoughLinesDetector for PtrOfCUDA_HoughLinesDetector {
		#[inline] fn as_raw_mut_CUDA_HoughLinesDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_HoughLinesDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_HoughLinesDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_HoughSegmentDetector = core::Ptr<dyn crate::cudaimgproc::CUDA_HoughSegmentDetector>;
	
	ptr_extern! { dyn crate::cudaimgproc::CUDA_HoughSegmentDetector,
		cv_PtrOfCUDA_HoughSegmentDetector_delete, cv_PtrOfCUDA_HoughSegmentDetector_get_inner_ptr, cv_PtrOfCUDA_HoughSegmentDetector_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_HoughSegmentDetector {
		#[inline] pub fn as_raw_PtrOfCUDA_HoughSegmentDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_HoughSegmentDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_HoughSegmentDetectorConst for PtrOfCUDA_HoughSegmentDetector {
		#[inline] fn as_raw_CUDA_HoughSegmentDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaimgproc::CUDA_HoughSegmentDetector for PtrOfCUDA_HoughSegmentDetector {
		#[inline] fn as_raw_mut_CUDA_HoughSegmentDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_HoughSegmentDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_HoughSegmentDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_TemplateMatching = core::Ptr<dyn crate::cudaimgproc::CUDA_TemplateMatching>;
	
	ptr_extern! { dyn crate::cudaimgproc::CUDA_TemplateMatching,
		cv_PtrOfCUDA_TemplateMatching_delete, cv_PtrOfCUDA_TemplateMatching_get_inner_ptr, cv_PtrOfCUDA_TemplateMatching_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_TemplateMatching {
		#[inline] pub fn as_raw_PtrOfCUDA_TemplateMatching(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_TemplateMatching(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_TemplateMatchingConst for PtrOfCUDA_TemplateMatching {
		#[inline] fn as_raw_CUDA_TemplateMatching(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaimgproc::CUDA_TemplateMatching for PtrOfCUDA_TemplateMatching {
		#[inline] fn as_raw_mut_CUDA_TemplateMatching(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_TemplateMatching {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_TemplateMatching {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_cudaimgproc)]
pub use cudaimgproc_types::*;

#[cfg(ocvrs_has_module_cudaobjdetect)]
mod cudaobjdetect_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCascadeClassifier = core::Ptr<dyn crate::cudaobjdetect::CascadeClassifier>;
	
	ptr_extern! { dyn crate::cudaobjdetect::CascadeClassifier,
		cv_PtrOfCascadeClassifier_delete, cv_PtrOfCascadeClassifier_get_inner_ptr, cv_PtrOfCascadeClassifier_get_inner_ptr_mut
	}
	
	impl PtrOfCascadeClassifier {
		#[inline] pub fn as_raw_PtrOfCascadeClassifier(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCascadeClassifier(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaobjdetect::CascadeClassifierConst for PtrOfCascadeClassifier {
		#[inline] fn as_raw_CascadeClassifier(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaobjdetect::CascadeClassifier for PtrOfCascadeClassifier {
		#[inline] fn as_raw_mut_CascadeClassifier(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCascadeClassifier {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCascadeClassifier {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfHOG = core::Ptr<dyn crate::cudaobjdetect::HOG>;
	
	ptr_extern! { dyn crate::cudaobjdetect::HOG,
		cv_PtrOfHOG_delete, cv_PtrOfHOG_get_inner_ptr, cv_PtrOfHOG_get_inner_ptr_mut
	}
	
	impl PtrOfHOG {
		#[inline] pub fn as_raw_PtrOfHOG(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHOG(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaobjdetect::HOGConst for PtrOfHOG {
		#[inline] fn as_raw_HOG(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaobjdetect::HOG for PtrOfHOG {
		#[inline] fn as_raw_mut_HOG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfHOG {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfHOG {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_cudaobjdetect)]
pub use cudaobjdetect_types::*;

#[cfg(ocvrs_has_module_cudaoptflow)]
mod cudaoptflow_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCUDA_BroxOpticalFlow = core::Ptr<dyn crate::cudaoptflow::CUDA_BroxOpticalFlow>;
	
	ptr_extern! { dyn crate::cudaoptflow::CUDA_BroxOpticalFlow,
		cv_PtrOfCUDA_BroxOpticalFlow_delete, cv_PtrOfCUDA_BroxOpticalFlow_get_inner_ptr, cv_PtrOfCUDA_BroxOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_BroxOpticalFlow {
		#[inline] pub fn as_raw_PtrOfCUDA_BroxOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_BroxOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_BroxOpticalFlowConst for PtrOfCUDA_BroxOpticalFlow {
		#[inline] fn as_raw_CUDA_BroxOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_BroxOpticalFlow for PtrOfCUDA_BroxOpticalFlow {
		#[inline] fn as_raw_mut_CUDA_BroxOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_BroxOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_BroxOpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlowConst for PtrOfCUDA_BroxOpticalFlow {
		#[inline] fn as_raw_CUDA_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlow for PtrOfCUDA_BroxOpticalFlow {
		#[inline] fn as_raw_mut_CUDA_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_DensePyrLKOpticalFlow = core::Ptr<dyn crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow>;
	
	ptr_extern! { dyn crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow,
		cv_PtrOfCUDA_DensePyrLKOpticalFlow_delete, cv_PtrOfCUDA_DensePyrLKOpticalFlow_get_inner_ptr, cv_PtrOfCUDA_DensePyrLKOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_DensePyrLKOpticalFlow {
		#[inline] pub fn as_raw_PtrOfCUDA_DensePyrLKOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_DensePyrLKOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_DensePyrLKOpticalFlowConst for PtrOfCUDA_DensePyrLKOpticalFlow {
		#[inline] fn as_raw_CUDA_DensePyrLKOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow for PtrOfCUDA_DensePyrLKOpticalFlow {
		#[inline] fn as_raw_mut_CUDA_DensePyrLKOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_DensePyrLKOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_DensePyrLKOpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlowConst for PtrOfCUDA_DensePyrLKOpticalFlow {
		#[inline] fn as_raw_CUDA_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlow for PtrOfCUDA_DensePyrLKOpticalFlow {
		#[inline] fn as_raw_mut_CUDA_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_FarnebackOpticalFlow = core::Ptr<dyn crate::cudaoptflow::CUDA_FarnebackOpticalFlow>;
	
	ptr_extern! { dyn crate::cudaoptflow::CUDA_FarnebackOpticalFlow,
		cv_PtrOfCUDA_FarnebackOpticalFlow_delete, cv_PtrOfCUDA_FarnebackOpticalFlow_get_inner_ptr, cv_PtrOfCUDA_FarnebackOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_FarnebackOpticalFlow {
		#[inline] pub fn as_raw_PtrOfCUDA_FarnebackOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_FarnebackOpticalFlowConst for PtrOfCUDA_FarnebackOpticalFlow {
		#[inline] fn as_raw_CUDA_FarnebackOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_FarnebackOpticalFlow for PtrOfCUDA_FarnebackOpticalFlow {
		#[inline] fn as_raw_mut_CUDA_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_FarnebackOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_FarnebackOpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlowConst for PtrOfCUDA_FarnebackOpticalFlow {
		#[inline] fn as_raw_CUDA_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlow for PtrOfCUDA_FarnebackOpticalFlow {
		#[inline] fn as_raw_mut_CUDA_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_NvidiaOpticalFlow_1_0 = core::Ptr<dyn crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0>;
	
	ptr_extern! { dyn crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0,
		cv_PtrOfCUDA_NvidiaOpticalFlow_1_0_delete, cv_PtrOfCUDA_NvidiaOpticalFlow_1_0_get_inner_ptr, cv_PtrOfCUDA_NvidiaOpticalFlow_1_0_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_NvidiaOpticalFlow_1_0 {
		#[inline] pub fn as_raw_PtrOfCUDA_NvidiaOpticalFlow_1_0(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_NvidiaOpticalFlow_1_0(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0Const for PtrOfCUDA_NvidiaOpticalFlow_1_0 {
		#[inline] fn as_raw_CUDA_NvidiaOpticalFlow_1_0(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0 for PtrOfCUDA_NvidiaOpticalFlow_1_0 {
		#[inline] fn as_raw_mut_CUDA_NvidiaOpticalFlow_1_0(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_NvidiaOpticalFlow_1_0 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_NvidiaOpticalFlow_1_0 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaHWOpticalFlowConst for PtrOfCUDA_NvidiaOpticalFlow_1_0 {
		#[inline] fn as_raw_CUDA_NvidiaHWOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaHWOpticalFlow for PtrOfCUDA_NvidiaOpticalFlow_1_0 {
		#[inline] fn as_raw_mut_CUDA_NvidiaHWOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_NvidiaOpticalFlow_2_0 = core::Ptr<dyn crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0>;
	
	ptr_extern! { dyn crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0,
		cv_PtrOfCUDA_NvidiaOpticalFlow_2_0_delete, cv_PtrOfCUDA_NvidiaOpticalFlow_2_0_get_inner_ptr, cv_PtrOfCUDA_NvidiaOpticalFlow_2_0_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_NvidiaOpticalFlow_2_0 {
		#[inline] pub fn as_raw_PtrOfCUDA_NvidiaOpticalFlow_2_0(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_NvidiaOpticalFlow_2_0(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0Const for PtrOfCUDA_NvidiaOpticalFlow_2_0 {
		#[inline] fn as_raw_CUDA_NvidiaOpticalFlow_2_0(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0 for PtrOfCUDA_NvidiaOpticalFlow_2_0 {
		#[inline] fn as_raw_mut_CUDA_NvidiaOpticalFlow_2_0(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_NvidiaOpticalFlow_2_0 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_NvidiaOpticalFlow_2_0 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaHWOpticalFlowConst for PtrOfCUDA_NvidiaOpticalFlow_2_0 {
		#[inline] fn as_raw_CUDA_NvidiaHWOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaHWOpticalFlow for PtrOfCUDA_NvidiaOpticalFlow_2_0 {
		#[inline] fn as_raw_mut_CUDA_NvidiaHWOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_OpticalFlowDual_TVL1 = core::Ptr<dyn crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1>;
	
	ptr_extern! { dyn crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1,
		cv_PtrOfCUDA_OpticalFlowDual_TVL1_delete, cv_PtrOfCUDA_OpticalFlowDual_TVL1_get_inner_ptr, cv_PtrOfCUDA_OpticalFlowDual_TVL1_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_OpticalFlowDual_TVL1 {
		#[inline] pub fn as_raw_PtrOfCUDA_OpticalFlowDual_TVL1(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_OpticalFlowDual_TVL1(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1Const for PtrOfCUDA_OpticalFlowDual_TVL1 {
		#[inline] fn as_raw_CUDA_OpticalFlowDual_TVL1(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1 for PtrOfCUDA_OpticalFlowDual_TVL1 {
		#[inline] fn as_raw_mut_CUDA_OpticalFlowDual_TVL1(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_OpticalFlowDual_TVL1 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_OpticalFlowDual_TVL1 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlowConst for PtrOfCUDA_OpticalFlowDual_TVL1 {
		#[inline] fn as_raw_CUDA_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlow for PtrOfCUDA_OpticalFlowDual_TVL1 {
		#[inline] fn as_raw_mut_CUDA_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_SparsePyrLKOpticalFlow = core::Ptr<dyn crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow>;
	
	ptr_extern! { dyn crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow,
		cv_PtrOfCUDA_SparsePyrLKOpticalFlow_delete, cv_PtrOfCUDA_SparsePyrLKOpticalFlow_get_inner_ptr, cv_PtrOfCUDA_SparsePyrLKOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_SparsePyrLKOpticalFlow {
		#[inline] pub fn as_raw_PtrOfCUDA_SparsePyrLKOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_SparsePyrLKOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlowConst for PtrOfCUDA_SparsePyrLKOpticalFlow {
		#[inline] fn as_raw_CUDA_SparsePyrLKOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow for PtrOfCUDA_SparsePyrLKOpticalFlow {
		#[inline] fn as_raw_mut_CUDA_SparsePyrLKOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_SparsePyrLKOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_SparsePyrLKOpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_SparseOpticalFlowConst for PtrOfCUDA_SparsePyrLKOpticalFlow {
		#[inline] fn as_raw_CUDA_SparseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_SparseOpticalFlow for PtrOfCUDA_SparsePyrLKOpticalFlow {
		#[inline] fn as_raw_mut_CUDA_SparseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_cudaoptflow)]
pub use cudaoptflow_types::*;

#[cfg(ocvrs_has_module_cudastereo)]
mod cudastereo_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCUDA_DisparityBilateralFilter = core::Ptr<dyn crate::cudastereo::CUDA_DisparityBilateralFilter>;
	
	ptr_extern! { dyn crate::cudastereo::CUDA_DisparityBilateralFilter,
		cv_PtrOfCUDA_DisparityBilateralFilter_delete, cv_PtrOfCUDA_DisparityBilateralFilter_get_inner_ptr, cv_PtrOfCUDA_DisparityBilateralFilter_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_DisparityBilateralFilter {
		#[inline] pub fn as_raw_PtrOfCUDA_DisparityBilateralFilter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_DisparityBilateralFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudastereo::CUDA_DisparityBilateralFilterConst for PtrOfCUDA_DisparityBilateralFilter {
		#[inline] fn as_raw_CUDA_DisparityBilateralFilter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudastereo::CUDA_DisparityBilateralFilter for PtrOfCUDA_DisparityBilateralFilter {
		#[inline] fn as_raw_mut_CUDA_DisparityBilateralFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_DisparityBilateralFilter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_DisparityBilateralFilter {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_StereoBM = core::Ptr<dyn crate::cudastereo::CUDA_StereoBM>;
	
	ptr_extern! { dyn crate::cudastereo::CUDA_StereoBM,
		cv_PtrOfCUDA_StereoBM_delete, cv_PtrOfCUDA_StereoBM_get_inner_ptr, cv_PtrOfCUDA_StereoBM_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_StereoBM {
		#[inline] pub fn as_raw_PtrOfCUDA_StereoBM(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_StereoBM(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudastereo::CUDA_StereoBMConst for PtrOfCUDA_StereoBM {
		#[inline] fn as_raw_CUDA_StereoBM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudastereo::CUDA_StereoBM for PtrOfCUDA_StereoBM {
		#[inline] fn as_raw_mut_CUDA_StereoBM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_StereoBM {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_StereoBM {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoBMConst for PtrOfCUDA_StereoBM {
		#[inline] fn as_raw_StereoBM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoBM for PtrOfCUDA_StereoBM {
		#[inline] fn as_raw_mut_StereoBM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcherConst for PtrOfCUDA_StereoBM {
		#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoMatcher for PtrOfCUDA_StereoBM {
		#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_StereoBeliefPropagation = core::Ptr<dyn crate::cudastereo::CUDA_StereoBeliefPropagation>;
	
	ptr_extern! { dyn crate::cudastereo::CUDA_StereoBeliefPropagation,
		cv_PtrOfCUDA_StereoBeliefPropagation_delete, cv_PtrOfCUDA_StereoBeliefPropagation_get_inner_ptr, cv_PtrOfCUDA_StereoBeliefPropagation_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_StereoBeliefPropagation {
		#[inline] pub fn as_raw_PtrOfCUDA_StereoBeliefPropagation(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_StereoBeliefPropagation(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudastereo::CUDA_StereoBeliefPropagationConst for PtrOfCUDA_StereoBeliefPropagation {
		#[inline] fn as_raw_CUDA_StereoBeliefPropagation(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudastereo::CUDA_StereoBeliefPropagation for PtrOfCUDA_StereoBeliefPropagation {
		#[inline] fn as_raw_mut_CUDA_StereoBeliefPropagation(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_StereoBeliefPropagation {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_StereoBeliefPropagation {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcherConst for PtrOfCUDA_StereoBeliefPropagation {
		#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoMatcher for PtrOfCUDA_StereoBeliefPropagation {
		#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_StereoConstantSpaceBP = core::Ptr<dyn crate::cudastereo::CUDA_StereoConstantSpaceBP>;
	
	ptr_extern! { dyn crate::cudastereo::CUDA_StereoConstantSpaceBP,
		cv_PtrOfCUDA_StereoConstantSpaceBP_delete, cv_PtrOfCUDA_StereoConstantSpaceBP_get_inner_ptr, cv_PtrOfCUDA_StereoConstantSpaceBP_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_StereoConstantSpaceBP {
		#[inline] pub fn as_raw_PtrOfCUDA_StereoConstantSpaceBP(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_StereoConstantSpaceBP(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudastereo::CUDA_StereoConstantSpaceBPConst for PtrOfCUDA_StereoConstantSpaceBP {
		#[inline] fn as_raw_CUDA_StereoConstantSpaceBP(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudastereo::CUDA_StereoConstantSpaceBP for PtrOfCUDA_StereoConstantSpaceBP {
		#[inline] fn as_raw_mut_CUDA_StereoConstantSpaceBP(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_StereoConstantSpaceBP {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_StereoConstantSpaceBP {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcherConst for PtrOfCUDA_StereoConstantSpaceBP {
		#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoMatcher for PtrOfCUDA_StereoConstantSpaceBP {
		#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudastereo::CUDA_StereoBeliefPropagationConst for PtrOfCUDA_StereoConstantSpaceBP {
		#[inline] fn as_raw_CUDA_StereoBeliefPropagation(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudastereo::CUDA_StereoBeliefPropagation for PtrOfCUDA_StereoConstantSpaceBP {
		#[inline] fn as_raw_mut_CUDA_StereoBeliefPropagation(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_StereoSGM = core::Ptr<dyn crate::cudastereo::CUDA_StereoSGM>;
	
	ptr_extern! { dyn crate::cudastereo::CUDA_StereoSGM,
		cv_PtrOfCUDA_StereoSGM_delete, cv_PtrOfCUDA_StereoSGM_get_inner_ptr, cv_PtrOfCUDA_StereoSGM_get_inner_ptr_mut
	}
	
	impl PtrOfCUDA_StereoSGM {
		#[inline] pub fn as_raw_PtrOfCUDA_StereoSGM(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_StereoSGM(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::cudastereo::CUDA_StereoSGMConst for PtrOfCUDA_StereoSGM {
		#[inline] fn as_raw_CUDA_StereoSGM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudastereo::CUDA_StereoSGM for PtrOfCUDA_StereoSGM {
		#[inline] fn as_raw_mut_CUDA_StereoSGM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCUDA_StereoSGM {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCUDA_StereoSGM {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcherConst for PtrOfCUDA_StereoSGM {
		#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoMatcher for PtrOfCUDA_StereoSGM {
		#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoSGBMConst for PtrOfCUDA_StereoSGM {
		#[inline] fn as_raw_StereoSGBM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoSGBM for PtrOfCUDA_StereoSGM {
		#[inline] fn as_raw_mut_StereoSGBM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_cudastereo)]
pub use cudastereo_types::*;

#[cfg(ocvrs_has_module_dnn)]
mod dnn_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAbsLayer = core::Ptr<crate::dnn::AbsLayer>;
	
	ptr_extern! { crate::dnn::AbsLayer,
		cv_PtrOfAbsLayer_delete, cv_PtrOfAbsLayer_get_inner_ptr, cv_PtrOfAbsLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AbsLayer, cv_PtrOfAbsLayer_new }
	
	impl PtrOfAbsLayer {
		#[inline] pub fn as_raw_PtrOfAbsLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAbsLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::AbsLayerTraitConst for PtrOfAbsLayer {
		#[inline] fn as_raw_AbsLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AbsLayerTrait for PtrOfAbsLayer {
		#[inline] fn as_raw_mut_AbsLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfAbsLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfAbsLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfAbsLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfAbsLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfAbsLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfAbsLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAccumLayer = core::Ptr<crate::dnn::AccumLayer>;
	
	ptr_extern! { crate::dnn::AccumLayer,
		cv_PtrOfAccumLayer_delete, cv_PtrOfAccumLayer_get_inner_ptr, cv_PtrOfAccumLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AccumLayer, cv_PtrOfAccumLayer_new }
	
	impl PtrOfAccumLayer {
		#[inline] pub fn as_raw_PtrOfAccumLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAccumLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::AccumLayerTraitConst for PtrOfAccumLayer {
		#[inline] fn as_raw_AccumLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AccumLayerTrait for PtrOfAccumLayer {
		#[inline] fn as_raw_mut_AccumLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfAccumLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfAccumLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfAccumLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfAccumLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAcosLayer = core::Ptr<crate::dnn::AcosLayer>;
	
	ptr_extern! { crate::dnn::AcosLayer,
		cv_PtrOfAcosLayer_delete, cv_PtrOfAcosLayer_get_inner_ptr, cv_PtrOfAcosLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AcosLayer, cv_PtrOfAcosLayer_new }
	
	impl PtrOfAcosLayer {
		#[inline] pub fn as_raw_PtrOfAcosLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAcosLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::AcosLayerTraitConst for PtrOfAcosLayer {
		#[inline] fn as_raw_AcosLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AcosLayerTrait for PtrOfAcosLayer {
		#[inline] fn as_raw_mut_AcosLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfAcosLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfAcosLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfAcosLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfAcosLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfAcosLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfAcosLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAcoshLayer = core::Ptr<crate::dnn::AcoshLayer>;
	
	ptr_extern! { crate::dnn::AcoshLayer,
		cv_PtrOfAcoshLayer_delete, cv_PtrOfAcoshLayer_get_inner_ptr, cv_PtrOfAcoshLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AcoshLayer, cv_PtrOfAcoshLayer_new }
	
	impl PtrOfAcoshLayer {
		#[inline] pub fn as_raw_PtrOfAcoshLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAcoshLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::AcoshLayerTraitConst for PtrOfAcoshLayer {
		#[inline] fn as_raw_AcoshLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AcoshLayerTrait for PtrOfAcoshLayer {
		#[inline] fn as_raw_mut_AcoshLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfAcoshLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfAcoshLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfAcoshLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfAcoshLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfAcoshLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfAcoshLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfActivationLayer = core::Ptr<crate::dnn::ActivationLayer>;
	
	ptr_extern! { crate::dnn::ActivationLayer,
		cv_PtrOfActivationLayer_delete, cv_PtrOfActivationLayer_get_inner_ptr, cv_PtrOfActivationLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ActivationLayer, cv_PtrOfActivationLayer_new }
	
	impl PtrOfActivationLayer {
		#[inline] pub fn as_raw_PtrOfActivationLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfActivationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfActivationLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfActivationLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfActivationLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfActivationLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfActivationLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfActivationLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfActivationLayerInt8 = core::Ptr<crate::dnn::ActivationLayerInt8>;
	
	ptr_extern! { crate::dnn::ActivationLayerInt8,
		cv_PtrOfActivationLayerInt8_delete, cv_PtrOfActivationLayerInt8_get_inner_ptr, cv_PtrOfActivationLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ActivationLayerInt8, cv_PtrOfActivationLayerInt8_new }
	
	impl PtrOfActivationLayerInt8 {
		#[inline] pub fn as_raw_PtrOfActivationLayerInt8(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfActivationLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerInt8TraitConst for PtrOfActivationLayerInt8 {
		#[inline] fn as_raw_ActivationLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerInt8Trait for PtrOfActivationLayerInt8 {
		#[inline] fn as_raw_mut_ActivationLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfActivationLayerInt8 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfActivationLayerInt8 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfActivationLayerInt8 {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfActivationLayerInt8 {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfActivationLayerInt8 {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfActivationLayerInt8 {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfArgLayer = core::Ptr<crate::dnn::ArgLayer>;
	
	ptr_extern! { crate::dnn::ArgLayer,
		cv_PtrOfArgLayer_delete, cv_PtrOfArgLayer_get_inner_ptr, cv_PtrOfArgLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ArgLayer, cv_PtrOfArgLayer_new }
	
	impl PtrOfArgLayer {
		#[inline] pub fn as_raw_PtrOfArgLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfArgLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ArgLayerTraitConst for PtrOfArgLayer {
		#[inline] fn as_raw_ArgLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ArgLayerTrait for PtrOfArgLayer {
		#[inline] fn as_raw_mut_ArgLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfArgLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfArgLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfArgLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfArgLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAsinLayer = core::Ptr<crate::dnn::AsinLayer>;
	
	ptr_extern! { crate::dnn::AsinLayer,
		cv_PtrOfAsinLayer_delete, cv_PtrOfAsinLayer_get_inner_ptr, cv_PtrOfAsinLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AsinLayer, cv_PtrOfAsinLayer_new }
	
	impl PtrOfAsinLayer {
		#[inline] pub fn as_raw_PtrOfAsinLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAsinLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::AsinLayerTraitConst for PtrOfAsinLayer {
		#[inline] fn as_raw_AsinLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AsinLayerTrait for PtrOfAsinLayer {
		#[inline] fn as_raw_mut_AsinLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfAsinLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfAsinLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfAsinLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfAsinLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfAsinLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfAsinLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAsinhLayer = core::Ptr<crate::dnn::AsinhLayer>;
	
	ptr_extern! { crate::dnn::AsinhLayer,
		cv_PtrOfAsinhLayer_delete, cv_PtrOfAsinhLayer_get_inner_ptr, cv_PtrOfAsinhLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AsinhLayer, cv_PtrOfAsinhLayer_new }
	
	impl PtrOfAsinhLayer {
		#[inline] pub fn as_raw_PtrOfAsinhLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAsinhLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::AsinhLayerTraitConst for PtrOfAsinhLayer {
		#[inline] fn as_raw_AsinhLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AsinhLayerTrait for PtrOfAsinhLayer {
		#[inline] fn as_raw_mut_AsinhLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfAsinhLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfAsinhLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfAsinhLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfAsinhLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfAsinhLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfAsinhLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAtanLayer = core::Ptr<crate::dnn::AtanLayer>;
	
	ptr_extern! { crate::dnn::AtanLayer,
		cv_PtrOfAtanLayer_delete, cv_PtrOfAtanLayer_get_inner_ptr, cv_PtrOfAtanLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AtanLayer, cv_PtrOfAtanLayer_new }
	
	impl PtrOfAtanLayer {
		#[inline] pub fn as_raw_PtrOfAtanLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAtanLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::AtanLayerTraitConst for PtrOfAtanLayer {
		#[inline] fn as_raw_AtanLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AtanLayerTrait for PtrOfAtanLayer {
		#[inline] fn as_raw_mut_AtanLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfAtanLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfAtanLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfAtanLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfAtanLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfAtanLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfAtanLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAtanhLayer = core::Ptr<crate::dnn::AtanhLayer>;
	
	ptr_extern! { crate::dnn::AtanhLayer,
		cv_PtrOfAtanhLayer_delete, cv_PtrOfAtanhLayer_get_inner_ptr, cv_PtrOfAtanhLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AtanhLayer, cv_PtrOfAtanhLayer_new }
	
	impl PtrOfAtanhLayer {
		#[inline] pub fn as_raw_PtrOfAtanhLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAtanhLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::AtanhLayerTraitConst for PtrOfAtanhLayer {
		#[inline] fn as_raw_AtanhLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AtanhLayerTrait for PtrOfAtanhLayer {
		#[inline] fn as_raw_mut_AtanhLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfAtanhLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfAtanhLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfAtanhLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfAtanhLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfAtanhLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfAtanhLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBNLLLayer = core::Ptr<crate::dnn::BNLLLayer>;
	
	ptr_extern! { crate::dnn::BNLLLayer,
		cv_PtrOfBNLLLayer_delete, cv_PtrOfBNLLLayer_get_inner_ptr, cv_PtrOfBNLLLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::BNLLLayer, cv_PtrOfBNLLLayer_new }
	
	impl PtrOfBNLLLayer {
		#[inline] pub fn as_raw_PtrOfBNLLLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBNLLLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::BNLLLayerTraitConst for PtrOfBNLLLayer {
		#[inline] fn as_raw_BNLLLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::BNLLLayerTrait for PtrOfBNLLLayer {
		#[inline] fn as_raw_mut_BNLLLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfBNLLLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfBNLLLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfBNLLLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfBNLLLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfBNLLLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfBNLLLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackendNode = core::Ptr<crate::dnn::BackendNode>;
	
	ptr_extern! { crate::dnn::BackendNode,
		cv_PtrOfBackendNode_delete, cv_PtrOfBackendNode_get_inner_ptr, cv_PtrOfBackendNode_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::BackendNode, cv_PtrOfBackendNode_new }
	
	impl PtrOfBackendNode {
		#[inline] pub fn as_raw_PtrOfBackendNode(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackendNode(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::BackendNodeTraitConst for PtrOfBackendNode {
		#[inline] fn as_raw_BackendNode(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::BackendNodeTrait for PtrOfBackendNode {
		#[inline] fn as_raw_mut_BackendNode(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackendWrapper = core::Ptr<dyn crate::dnn::BackendWrapper>;
	
	ptr_extern! { dyn crate::dnn::BackendWrapper,
		cv_PtrOfBackendWrapper_delete, cv_PtrOfBackendWrapper_get_inner_ptr, cv_PtrOfBackendWrapper_get_inner_ptr_mut
	}
	
	impl PtrOfBackendWrapper {
		#[inline] pub fn as_raw_PtrOfBackendWrapper(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackendWrapper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::BackendWrapperConst for PtrOfBackendWrapper {
		#[inline] fn as_raw_BackendWrapper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::BackendWrapper for PtrOfBackendWrapper {
		#[inline] fn as_raw_mut_BackendWrapper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBaseConvolutionLayer = core::Ptr<crate::dnn::BaseConvolutionLayer>;
	
	ptr_extern! { crate::dnn::BaseConvolutionLayer,
		cv_PtrOfBaseConvolutionLayer_delete, cv_PtrOfBaseConvolutionLayer_get_inner_ptr, cv_PtrOfBaseConvolutionLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::BaseConvolutionLayer, cv_PtrOfBaseConvolutionLayer_new }
	
	impl PtrOfBaseConvolutionLayer {
		#[inline] pub fn as_raw_PtrOfBaseConvolutionLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBaseConvolutionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::BaseConvolutionLayerTraitConst for PtrOfBaseConvolutionLayer {
		#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::BaseConvolutionLayerTrait for PtrOfBaseConvolutionLayer {
		#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfBaseConvolutionLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfBaseConvolutionLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfBaseConvolutionLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfBaseConvolutionLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBatchNormLayer = core::Ptr<crate::dnn::BatchNormLayer>;
	
	ptr_extern! { crate::dnn::BatchNormLayer,
		cv_PtrOfBatchNormLayer_delete, cv_PtrOfBatchNormLayer_get_inner_ptr, cv_PtrOfBatchNormLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::BatchNormLayer, cv_PtrOfBatchNormLayer_new }
	
	impl PtrOfBatchNormLayer {
		#[inline] pub fn as_raw_PtrOfBatchNormLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBatchNormLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::BatchNormLayerTraitConst for PtrOfBatchNormLayer {
		#[inline] fn as_raw_BatchNormLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::BatchNormLayerTrait for PtrOfBatchNormLayer {
		#[inline] fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfBatchNormLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfBatchNormLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfBatchNormLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfBatchNormLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfBatchNormLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfBatchNormLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBatchNormLayerInt8 = core::Ptr<crate::dnn::BatchNormLayerInt8>;
	
	ptr_extern! { crate::dnn::BatchNormLayerInt8,
		cv_PtrOfBatchNormLayerInt8_delete, cv_PtrOfBatchNormLayerInt8_get_inner_ptr, cv_PtrOfBatchNormLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::BatchNormLayerInt8, cv_PtrOfBatchNormLayerInt8_new }
	
	impl PtrOfBatchNormLayerInt8 {
		#[inline] pub fn as_raw_PtrOfBatchNormLayerInt8(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBatchNormLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::BatchNormLayerInt8TraitConst for PtrOfBatchNormLayerInt8 {
		#[inline] fn as_raw_BatchNormLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::BatchNormLayerInt8Trait for PtrOfBatchNormLayerInt8 {
		#[inline] fn as_raw_mut_BatchNormLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfBatchNormLayerInt8 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfBatchNormLayerInt8 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfBatchNormLayerInt8 {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfBatchNormLayerInt8 {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::BatchNormLayerTraitConst for PtrOfBatchNormLayerInt8 {
		#[inline] fn as_raw_BatchNormLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::BatchNormLayerTrait for PtrOfBatchNormLayerInt8 {
		#[inline] fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfBatchNormLayerInt8 {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfBatchNormLayerInt8 {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCeilLayer = core::Ptr<crate::dnn::CeilLayer>;
	
	ptr_extern! { crate::dnn::CeilLayer,
		cv_PtrOfCeilLayer_delete, cv_PtrOfCeilLayer_get_inner_ptr, cv_PtrOfCeilLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::CeilLayer, cv_PtrOfCeilLayer_new }
	
	impl PtrOfCeilLayer {
		#[inline] pub fn as_raw_PtrOfCeilLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCeilLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::CeilLayerTraitConst for PtrOfCeilLayer {
		#[inline] fn as_raw_CeilLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::CeilLayerTrait for PtrOfCeilLayer {
		#[inline] fn as_raw_mut_CeilLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCeilLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCeilLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfCeilLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfCeilLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfCeilLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfCeilLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCeluLayer = core::Ptr<crate::dnn::CeluLayer>;
	
	ptr_extern! { crate::dnn::CeluLayer,
		cv_PtrOfCeluLayer_delete, cv_PtrOfCeluLayer_get_inner_ptr, cv_PtrOfCeluLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::CeluLayer, cv_PtrOfCeluLayer_new }
	
	impl PtrOfCeluLayer {
		#[inline] pub fn as_raw_PtrOfCeluLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCeluLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::CeluLayerTraitConst for PtrOfCeluLayer {
		#[inline] fn as_raw_CeluLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::CeluLayerTrait for PtrOfCeluLayer {
		#[inline] fn as_raw_mut_CeluLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCeluLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCeluLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfCeluLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfCeluLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfCeluLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfCeluLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfConcatLayer = core::Ptr<crate::dnn::ConcatLayer>;
	
	ptr_extern! { crate::dnn::ConcatLayer,
		cv_PtrOfConcatLayer_delete, cv_PtrOfConcatLayer_get_inner_ptr, cv_PtrOfConcatLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ConcatLayer, cv_PtrOfConcatLayer_new }
	
	impl PtrOfConcatLayer {
		#[inline] pub fn as_raw_PtrOfConcatLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConcatLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ConcatLayerTraitConst for PtrOfConcatLayer {
		#[inline] fn as_raw_ConcatLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ConcatLayerTrait for PtrOfConcatLayer {
		#[inline] fn as_raw_mut_ConcatLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfConcatLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfConcatLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfConcatLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfConcatLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCorrelationLayer = core::Ptr<crate::dnn::CorrelationLayer>;
	
	ptr_extern! { crate::dnn::CorrelationLayer,
		cv_PtrOfCorrelationLayer_delete, cv_PtrOfCorrelationLayer_get_inner_ptr, cv_PtrOfCorrelationLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::CorrelationLayer, cv_PtrOfCorrelationLayer_new }
	
	impl PtrOfCorrelationLayer {
		#[inline] pub fn as_raw_PtrOfCorrelationLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCorrelationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::CorrelationLayerTraitConst for PtrOfCorrelationLayer {
		#[inline] fn as_raw_CorrelationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::CorrelationLayerTrait for PtrOfCorrelationLayer {
		#[inline] fn as_raw_mut_CorrelationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCorrelationLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCorrelationLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfCorrelationLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfCorrelationLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCosLayer = core::Ptr<crate::dnn::CosLayer>;
	
	ptr_extern! { crate::dnn::CosLayer,
		cv_PtrOfCosLayer_delete, cv_PtrOfCosLayer_get_inner_ptr, cv_PtrOfCosLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::CosLayer, cv_PtrOfCosLayer_new }
	
	impl PtrOfCosLayer {
		#[inline] pub fn as_raw_PtrOfCosLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCosLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::CosLayerTraitConst for PtrOfCosLayer {
		#[inline] fn as_raw_CosLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::CosLayerTrait for PtrOfCosLayer {
		#[inline] fn as_raw_mut_CosLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCosLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCosLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfCosLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfCosLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfCosLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfCosLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCoshLayer = core::Ptr<crate::dnn::CoshLayer>;
	
	ptr_extern! { crate::dnn::CoshLayer,
		cv_PtrOfCoshLayer_delete, cv_PtrOfCoshLayer_get_inner_ptr, cv_PtrOfCoshLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::CoshLayer, cv_PtrOfCoshLayer_new }
	
	impl PtrOfCoshLayer {
		#[inline] pub fn as_raw_PtrOfCoshLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCoshLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::CoshLayerTraitConst for PtrOfCoshLayer {
		#[inline] fn as_raw_CoshLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::CoshLayerTrait for PtrOfCoshLayer {
		#[inline] fn as_raw_mut_CoshLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCoshLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCoshLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfCoshLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfCoshLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfCoshLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfCoshLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCumSumLayer = core::Ptr<crate::dnn::CumSumLayer>;
	
	ptr_extern! { crate::dnn::CumSumLayer,
		cv_PtrOfCumSumLayer_delete, cv_PtrOfCumSumLayer_get_inner_ptr, cv_PtrOfCumSumLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::CumSumLayer, cv_PtrOfCumSumLayer_new }
	
	impl PtrOfCumSumLayer {
		#[inline] pub fn as_raw_PtrOfCumSumLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCumSumLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::CumSumLayerTraitConst for PtrOfCumSumLayer {
		#[inline] fn as_raw_CumSumLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::CumSumLayerTrait for PtrOfCumSumLayer {
		#[inline] fn as_raw_mut_CumSumLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCumSumLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCumSumLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfCumSumLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfCumSumLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDataAugmentationLayer = core::Ptr<crate::dnn::DataAugmentationLayer>;
	
	ptr_extern! { crate::dnn::DataAugmentationLayer,
		cv_PtrOfDataAugmentationLayer_delete, cv_PtrOfDataAugmentationLayer_get_inner_ptr, cv_PtrOfDataAugmentationLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::DataAugmentationLayer, cv_PtrOfDataAugmentationLayer_new }
	
	impl PtrOfDataAugmentationLayer {
		#[inline] pub fn as_raw_PtrOfDataAugmentationLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDataAugmentationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::DataAugmentationLayerTraitConst for PtrOfDataAugmentationLayer {
		#[inline] fn as_raw_DataAugmentationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::DataAugmentationLayerTrait for PtrOfDataAugmentationLayer {
		#[inline] fn as_raw_mut_DataAugmentationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfDataAugmentationLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfDataAugmentationLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfDataAugmentationLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfDataAugmentationLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDequantizeLayer = core::Ptr<crate::dnn::DequantizeLayer>;
	
	ptr_extern! { crate::dnn::DequantizeLayer,
		cv_PtrOfDequantizeLayer_delete, cv_PtrOfDequantizeLayer_get_inner_ptr, cv_PtrOfDequantizeLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::DequantizeLayer, cv_PtrOfDequantizeLayer_new }
	
	impl PtrOfDequantizeLayer {
		#[inline] pub fn as_raw_PtrOfDequantizeLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDequantizeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::DequantizeLayerTraitConst for PtrOfDequantizeLayer {
		#[inline] fn as_raw_DequantizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::DequantizeLayerTrait for PtrOfDequantizeLayer {
		#[inline] fn as_raw_mut_DequantizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfDequantizeLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfDequantizeLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfDequantizeLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfDequantizeLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetectionOutputLayer = core::Ptr<crate::dnn::DetectionOutputLayer>;
	
	ptr_extern! { crate::dnn::DetectionOutputLayer,
		cv_PtrOfDetectionOutputLayer_delete, cv_PtrOfDetectionOutputLayer_get_inner_ptr, cv_PtrOfDetectionOutputLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::DetectionOutputLayer, cv_PtrOfDetectionOutputLayer_new }
	
	impl PtrOfDetectionOutputLayer {
		#[inline] pub fn as_raw_PtrOfDetectionOutputLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetectionOutputLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::DetectionOutputLayerTraitConst for PtrOfDetectionOutputLayer {
		#[inline] fn as_raw_DetectionOutputLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::DetectionOutputLayerTrait for PtrOfDetectionOutputLayer {
		#[inline] fn as_raw_mut_DetectionOutputLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfDetectionOutputLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfDetectionOutputLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfDetectionOutputLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfDetectionOutputLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfELULayer = core::Ptr<crate::dnn::ELULayer>;
	
	ptr_extern! { crate::dnn::ELULayer,
		cv_PtrOfELULayer_delete, cv_PtrOfELULayer_get_inner_ptr, cv_PtrOfELULayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ELULayer, cv_PtrOfELULayer_new }
	
	impl PtrOfELULayer {
		#[inline] pub fn as_raw_PtrOfELULayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfELULayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ELULayerTraitConst for PtrOfELULayer {
		#[inline] fn as_raw_ELULayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ELULayerTrait for PtrOfELULayer {
		#[inline] fn as_raw_mut_ELULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfELULayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfELULayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfELULayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfELULayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfELULayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfELULayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEltwiseLayer = core::Ptr<crate::dnn::EltwiseLayer>;
	
	ptr_extern! { crate::dnn::EltwiseLayer,
		cv_PtrOfEltwiseLayer_delete, cv_PtrOfEltwiseLayer_get_inner_ptr, cv_PtrOfEltwiseLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::EltwiseLayer, cv_PtrOfEltwiseLayer_new }
	
	impl PtrOfEltwiseLayer {
		#[inline] pub fn as_raw_PtrOfEltwiseLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEltwiseLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::EltwiseLayerTraitConst for PtrOfEltwiseLayer {
		#[inline] fn as_raw_EltwiseLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::EltwiseLayerTrait for PtrOfEltwiseLayer {
		#[inline] fn as_raw_mut_EltwiseLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfEltwiseLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfEltwiseLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfEltwiseLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfEltwiseLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEltwiseLayerInt8 = core::Ptr<crate::dnn::EltwiseLayerInt8>;
	
	ptr_extern! { crate::dnn::EltwiseLayerInt8,
		cv_PtrOfEltwiseLayerInt8_delete, cv_PtrOfEltwiseLayerInt8_get_inner_ptr, cv_PtrOfEltwiseLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::EltwiseLayerInt8, cv_PtrOfEltwiseLayerInt8_new }
	
	impl PtrOfEltwiseLayerInt8 {
		#[inline] pub fn as_raw_PtrOfEltwiseLayerInt8(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEltwiseLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::EltwiseLayerInt8TraitConst for PtrOfEltwiseLayerInt8 {
		#[inline] fn as_raw_EltwiseLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::EltwiseLayerInt8Trait for PtrOfEltwiseLayerInt8 {
		#[inline] fn as_raw_mut_EltwiseLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfEltwiseLayerInt8 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfEltwiseLayerInt8 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfEltwiseLayerInt8 {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfEltwiseLayerInt8 {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfErfLayer = core::Ptr<crate::dnn::ErfLayer>;
	
	ptr_extern! { crate::dnn::ErfLayer,
		cv_PtrOfErfLayer_delete, cv_PtrOfErfLayer_get_inner_ptr, cv_PtrOfErfLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ErfLayer, cv_PtrOfErfLayer_new }
	
	impl PtrOfErfLayer {
		#[inline] pub fn as_raw_PtrOfErfLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfErfLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ErfLayerTraitConst for PtrOfErfLayer {
		#[inline] fn as_raw_ErfLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ErfLayerTrait for PtrOfErfLayer {
		#[inline] fn as_raw_mut_ErfLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfErfLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfErfLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfErfLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfErfLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfErfLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfErfLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfExpLayer = core::Ptr<crate::dnn::ExpLayer>;
	
	ptr_extern! { crate::dnn::ExpLayer,
		cv_PtrOfExpLayer_delete, cv_PtrOfExpLayer_get_inner_ptr, cv_PtrOfExpLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ExpLayer, cv_PtrOfExpLayer_new }
	
	impl PtrOfExpLayer {
		#[inline] pub fn as_raw_PtrOfExpLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfExpLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ExpLayerTraitConst for PtrOfExpLayer {
		#[inline] fn as_raw_ExpLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ExpLayerTrait for PtrOfExpLayer {
		#[inline] fn as_raw_mut_ExpLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfExpLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfExpLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfExpLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfExpLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfExpLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfExpLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFlattenLayer = core::Ptr<crate::dnn::FlattenLayer>;
	
	ptr_extern! { crate::dnn::FlattenLayer,
		cv_PtrOfFlattenLayer_delete, cv_PtrOfFlattenLayer_get_inner_ptr, cv_PtrOfFlattenLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::FlattenLayer, cv_PtrOfFlattenLayer_new }
	
	impl PtrOfFlattenLayer {
		#[inline] pub fn as_raw_PtrOfFlattenLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFlattenLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::FlattenLayerTraitConst for PtrOfFlattenLayer {
		#[inline] fn as_raw_FlattenLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::FlattenLayerTrait for PtrOfFlattenLayer {
		#[inline] fn as_raw_mut_FlattenLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfFlattenLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfFlattenLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfFlattenLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfFlattenLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFloorLayer = core::Ptr<crate::dnn::FloorLayer>;
	
	ptr_extern! { crate::dnn::FloorLayer,
		cv_PtrOfFloorLayer_delete, cv_PtrOfFloorLayer_get_inner_ptr, cv_PtrOfFloorLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::FloorLayer, cv_PtrOfFloorLayer_new }
	
	impl PtrOfFloorLayer {
		#[inline] pub fn as_raw_PtrOfFloorLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFloorLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::FloorLayerTraitConst for PtrOfFloorLayer {
		#[inline] fn as_raw_FloorLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::FloorLayerTrait for PtrOfFloorLayer {
		#[inline] fn as_raw_mut_FloorLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfFloorLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfFloorLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfFloorLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfFloorLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfFloorLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfFloorLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFlowWarpLayer = core::Ptr<crate::dnn::FlowWarpLayer>;
	
	ptr_extern! { crate::dnn::FlowWarpLayer,
		cv_PtrOfFlowWarpLayer_delete, cv_PtrOfFlowWarpLayer_get_inner_ptr, cv_PtrOfFlowWarpLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::FlowWarpLayer, cv_PtrOfFlowWarpLayer_new }
	
	impl PtrOfFlowWarpLayer {
		#[inline] pub fn as_raw_PtrOfFlowWarpLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFlowWarpLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::FlowWarpLayerTraitConst for PtrOfFlowWarpLayer {
		#[inline] fn as_raw_FlowWarpLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::FlowWarpLayerTrait for PtrOfFlowWarpLayer {
		#[inline] fn as_raw_mut_FlowWarpLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfFlowWarpLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfFlowWarpLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfFlowWarpLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfFlowWarpLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGRULayer = core::Ptr<crate::dnn::GRULayer>;
	
	ptr_extern! { crate::dnn::GRULayer,
		cv_PtrOfGRULayer_delete, cv_PtrOfGRULayer_get_inner_ptr, cv_PtrOfGRULayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::GRULayer, cv_PtrOfGRULayer_new }
	
	impl PtrOfGRULayer {
		#[inline] pub fn as_raw_PtrOfGRULayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGRULayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::GRULayerTraitConst for PtrOfGRULayer {
		#[inline] fn as_raw_GRULayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::GRULayerTrait for PtrOfGRULayer {
		#[inline] fn as_raw_mut_GRULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfGRULayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfGRULayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfGRULayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfGRULayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfHardSigmoidLayer = core::Ptr<crate::dnn::HardSigmoidLayer>;
	
	ptr_extern! { crate::dnn::HardSigmoidLayer,
		cv_PtrOfHardSigmoidLayer_delete, cv_PtrOfHardSigmoidLayer_get_inner_ptr, cv_PtrOfHardSigmoidLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::HardSigmoidLayer, cv_PtrOfHardSigmoidLayer_new }
	
	impl PtrOfHardSigmoidLayer {
		#[inline] pub fn as_raw_PtrOfHardSigmoidLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHardSigmoidLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::HardSigmoidLayerTraitConst for PtrOfHardSigmoidLayer {
		#[inline] fn as_raw_HardSigmoidLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::HardSigmoidLayerTrait for PtrOfHardSigmoidLayer {
		#[inline] fn as_raw_mut_HardSigmoidLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfHardSigmoidLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfHardSigmoidLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfHardSigmoidLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfHardSigmoidLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfHardSigmoidLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfHardSigmoidLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfHardSwishLayer = core::Ptr<crate::dnn::HardSwishLayer>;
	
	ptr_extern! { crate::dnn::HardSwishLayer,
		cv_PtrOfHardSwishLayer_delete, cv_PtrOfHardSwishLayer_get_inner_ptr, cv_PtrOfHardSwishLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::HardSwishLayer, cv_PtrOfHardSwishLayer_new }
	
	impl PtrOfHardSwishLayer {
		#[inline] pub fn as_raw_PtrOfHardSwishLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHardSwishLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::HardSwishLayerTraitConst for PtrOfHardSwishLayer {
		#[inline] fn as_raw_HardSwishLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::HardSwishLayerTrait for PtrOfHardSwishLayer {
		#[inline] fn as_raw_mut_HardSwishLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfHardSwishLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfHardSwishLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfHardSwishLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfHardSwishLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfHardSwishLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfHardSwishLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfInnerProductLayer = core::Ptr<crate::dnn::InnerProductLayer>;
	
	ptr_extern! { crate::dnn::InnerProductLayer,
		cv_PtrOfInnerProductLayer_delete, cv_PtrOfInnerProductLayer_get_inner_ptr, cv_PtrOfInnerProductLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::InnerProductLayer, cv_PtrOfInnerProductLayer_new }
	
	impl PtrOfInnerProductLayer {
		#[inline] pub fn as_raw_PtrOfInnerProductLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfInnerProductLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::InnerProductLayerTraitConst for PtrOfInnerProductLayer {
		#[inline] fn as_raw_InnerProductLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::InnerProductLayerTrait for PtrOfInnerProductLayer {
		#[inline] fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfInnerProductLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfInnerProductLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfInnerProductLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfInnerProductLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfInnerProductLayerInt8 = core::Ptr<crate::dnn::InnerProductLayerInt8>;
	
	ptr_extern! { crate::dnn::InnerProductLayerInt8,
		cv_PtrOfInnerProductLayerInt8_delete, cv_PtrOfInnerProductLayerInt8_get_inner_ptr, cv_PtrOfInnerProductLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::InnerProductLayerInt8, cv_PtrOfInnerProductLayerInt8_new }
	
	impl PtrOfInnerProductLayerInt8 {
		#[inline] pub fn as_raw_PtrOfInnerProductLayerInt8(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfInnerProductLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::InnerProductLayerInt8TraitConst for PtrOfInnerProductLayerInt8 {
		#[inline] fn as_raw_InnerProductLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::InnerProductLayerInt8Trait for PtrOfInnerProductLayerInt8 {
		#[inline] fn as_raw_mut_InnerProductLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfInnerProductLayerInt8 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfInnerProductLayerInt8 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::InnerProductLayerTraitConst for PtrOfInnerProductLayerInt8 {
		#[inline] fn as_raw_InnerProductLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::InnerProductLayerTrait for PtrOfInnerProductLayerInt8 {
		#[inline] fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfInnerProductLayerInt8 {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfInnerProductLayerInt8 {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLRNLayer = core::Ptr<crate::dnn::LRNLayer>;
	
	ptr_extern! { crate::dnn::LRNLayer,
		cv_PtrOfLRNLayer_delete, cv_PtrOfLRNLayer_get_inner_ptr, cv_PtrOfLRNLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::LRNLayer, cv_PtrOfLRNLayer_new }
	
	impl PtrOfLRNLayer {
		#[inline] pub fn as_raw_PtrOfLRNLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLRNLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LRNLayerTraitConst for PtrOfLRNLayer {
		#[inline] fn as_raw_LRNLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LRNLayerTrait for PtrOfLRNLayer {
		#[inline] fn as_raw_mut_LRNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfLRNLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfLRNLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfLRNLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfLRNLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLSTMLayer = core::Ptr<dyn crate::dnn::LSTMLayer>;
	
	ptr_extern! { dyn crate::dnn::LSTMLayer,
		cv_PtrOfLSTMLayer_delete, cv_PtrOfLSTMLayer_get_inner_ptr, cv_PtrOfLSTMLayer_get_inner_ptr_mut
	}
	
	impl PtrOfLSTMLayer {
		#[inline] pub fn as_raw_PtrOfLSTMLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLSTMLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LSTMLayerConst for PtrOfLSTMLayer {
		#[inline] fn as_raw_LSTMLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LSTMLayer for PtrOfLSTMLayer {
		#[inline] fn as_raw_mut_LSTMLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfLSTMLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfLSTMLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfLSTMLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfLSTMLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLayer = core::Ptr<crate::dnn::Layer>;
	
	ptr_extern! { crate::dnn::Layer,
		cv_PtrOfLayer_delete, cv_PtrOfLayer_get_inner_ptr, cv_PtrOfLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::Layer, cv_PtrOfLayer_new }
	
	impl PtrOfLayer {
		#[inline] pub fn as_raw_PtrOfLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLogLayer = core::Ptr<crate::dnn::LogLayer>;
	
	ptr_extern! { crate::dnn::LogLayer,
		cv_PtrOfLogLayer_delete, cv_PtrOfLogLayer_get_inner_ptr, cv_PtrOfLogLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::LogLayer, cv_PtrOfLogLayer_new }
	
	impl PtrOfLogLayer {
		#[inline] pub fn as_raw_PtrOfLogLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLogLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::LogLayerTraitConst for PtrOfLogLayer {
		#[inline] fn as_raw_LogLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LogLayerTrait for PtrOfLogLayer {
		#[inline] fn as_raw_mut_LogLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfLogLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfLogLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfLogLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfLogLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfLogLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfLogLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMVNLayer = core::Ptr<crate::dnn::MVNLayer>;
	
	ptr_extern! { crate::dnn::MVNLayer,
		cv_PtrOfMVNLayer_delete, cv_PtrOfMVNLayer_get_inner_ptr, cv_PtrOfMVNLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::MVNLayer, cv_PtrOfMVNLayer_new }
	
	impl PtrOfMVNLayer {
		#[inline] pub fn as_raw_PtrOfMVNLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMVNLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::MVNLayerTraitConst for PtrOfMVNLayer {
		#[inline] fn as_raw_MVNLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::MVNLayerTrait for PtrOfMVNLayer {
		#[inline] fn as_raw_mut_MVNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfMVNLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfMVNLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfMVNLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfMVNLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMaxUnpoolLayer = core::Ptr<crate::dnn::MaxUnpoolLayer>;
	
	ptr_extern! { crate::dnn::MaxUnpoolLayer,
		cv_PtrOfMaxUnpoolLayer_delete, cv_PtrOfMaxUnpoolLayer_get_inner_ptr, cv_PtrOfMaxUnpoolLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::MaxUnpoolLayer, cv_PtrOfMaxUnpoolLayer_new }
	
	impl PtrOfMaxUnpoolLayer {
		#[inline] pub fn as_raw_PtrOfMaxUnpoolLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMaxUnpoolLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::MaxUnpoolLayerTraitConst for PtrOfMaxUnpoolLayer {
		#[inline] fn as_raw_MaxUnpoolLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::MaxUnpoolLayerTrait for PtrOfMaxUnpoolLayer {
		#[inline] fn as_raw_mut_MaxUnpoolLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfMaxUnpoolLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfMaxUnpoolLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfMaxUnpoolLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfMaxUnpoolLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMishLayer = core::Ptr<crate::dnn::MishLayer>;
	
	ptr_extern! { crate::dnn::MishLayer,
		cv_PtrOfMishLayer_delete, cv_PtrOfMishLayer_get_inner_ptr, cv_PtrOfMishLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::MishLayer, cv_PtrOfMishLayer_new }
	
	impl PtrOfMishLayer {
		#[inline] pub fn as_raw_PtrOfMishLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMishLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::MishLayerTraitConst for PtrOfMishLayer {
		#[inline] fn as_raw_MishLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::MishLayerTrait for PtrOfMishLayer {
		#[inline] fn as_raw_mut_MishLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfMishLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfMishLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfMishLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfMishLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfMishLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfMishLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfNormalizeBBoxLayer = core::Ptr<crate::dnn::NormalizeBBoxLayer>;
	
	ptr_extern! { crate::dnn::NormalizeBBoxLayer,
		cv_PtrOfNormalizeBBoxLayer_delete, cv_PtrOfNormalizeBBoxLayer_get_inner_ptr, cv_PtrOfNormalizeBBoxLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::NormalizeBBoxLayer, cv_PtrOfNormalizeBBoxLayer_new }
	
	impl PtrOfNormalizeBBoxLayer {
		#[inline] pub fn as_raw_PtrOfNormalizeBBoxLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNormalizeBBoxLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::NormalizeBBoxLayerTraitConst for PtrOfNormalizeBBoxLayer {
		#[inline] fn as_raw_NormalizeBBoxLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::NormalizeBBoxLayerTrait for PtrOfNormalizeBBoxLayer {
		#[inline] fn as_raw_mut_NormalizeBBoxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfNormalizeBBoxLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfNormalizeBBoxLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfNormalizeBBoxLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfNormalizeBBoxLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfNotLayer = core::Ptr<crate::dnn::NotLayer>;
	
	ptr_extern! { crate::dnn::NotLayer,
		cv_PtrOfNotLayer_delete, cv_PtrOfNotLayer_get_inner_ptr, cv_PtrOfNotLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::NotLayer, cv_PtrOfNotLayer_new }
	
	impl PtrOfNotLayer {
		#[inline] pub fn as_raw_PtrOfNotLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNotLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::NotLayerTraitConst for PtrOfNotLayer {
		#[inline] fn as_raw_NotLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::NotLayerTrait for PtrOfNotLayer {
		#[inline] fn as_raw_mut_NotLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfNotLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfNotLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfNotLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfNotLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfNotLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfNotLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPaddingLayer = core::Ptr<crate::dnn::PaddingLayer>;
	
	ptr_extern! { crate::dnn::PaddingLayer,
		cv_PtrOfPaddingLayer_delete, cv_PtrOfPaddingLayer_get_inner_ptr, cv_PtrOfPaddingLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::PaddingLayer, cv_PtrOfPaddingLayer_new }
	
	impl PtrOfPaddingLayer {
		#[inline] pub fn as_raw_PtrOfPaddingLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPaddingLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::PaddingLayerTraitConst for PtrOfPaddingLayer {
		#[inline] fn as_raw_PaddingLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::PaddingLayerTrait for PtrOfPaddingLayer {
		#[inline] fn as_raw_mut_PaddingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfPaddingLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfPaddingLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfPaddingLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfPaddingLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPermuteLayer = core::Ptr<crate::dnn::PermuteLayer>;
	
	ptr_extern! { crate::dnn::PermuteLayer,
		cv_PtrOfPermuteLayer_delete, cv_PtrOfPermuteLayer_get_inner_ptr, cv_PtrOfPermuteLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::PermuteLayer, cv_PtrOfPermuteLayer_new }
	
	impl PtrOfPermuteLayer {
		#[inline] pub fn as_raw_PtrOfPermuteLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPermuteLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::PermuteLayerTraitConst for PtrOfPermuteLayer {
		#[inline] fn as_raw_PermuteLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::PermuteLayerTrait for PtrOfPermuteLayer {
		#[inline] fn as_raw_mut_PermuteLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfPermuteLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfPermuteLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfPermuteLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfPermuteLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPoolingLayer = core::Ptr<crate::dnn::PoolingLayer>;
	
	ptr_extern! { crate::dnn::PoolingLayer,
		cv_PtrOfPoolingLayer_delete, cv_PtrOfPoolingLayer_get_inner_ptr, cv_PtrOfPoolingLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::PoolingLayer, cv_PtrOfPoolingLayer_new }
	
	impl PtrOfPoolingLayer {
		#[inline] pub fn as_raw_PtrOfPoolingLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPoolingLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::PoolingLayerTraitConst for PtrOfPoolingLayer {
		#[inline] fn as_raw_PoolingLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::PoolingLayerTrait for PtrOfPoolingLayer {
		#[inline] fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfPoolingLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfPoolingLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfPoolingLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfPoolingLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPoolingLayerInt8 = core::Ptr<crate::dnn::PoolingLayerInt8>;
	
	ptr_extern! { crate::dnn::PoolingLayerInt8,
		cv_PtrOfPoolingLayerInt8_delete, cv_PtrOfPoolingLayerInt8_get_inner_ptr, cv_PtrOfPoolingLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::PoolingLayerInt8, cv_PtrOfPoolingLayerInt8_new }
	
	impl PtrOfPoolingLayerInt8 {
		#[inline] pub fn as_raw_PtrOfPoolingLayerInt8(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPoolingLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::PoolingLayerInt8TraitConst for PtrOfPoolingLayerInt8 {
		#[inline] fn as_raw_PoolingLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::PoolingLayerInt8Trait for PtrOfPoolingLayerInt8 {
		#[inline] fn as_raw_mut_PoolingLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfPoolingLayerInt8 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfPoolingLayerInt8 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfPoolingLayerInt8 {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfPoolingLayerInt8 {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::PoolingLayerTraitConst for PtrOfPoolingLayerInt8 {
		#[inline] fn as_raw_PoolingLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::PoolingLayerTrait for PtrOfPoolingLayerInt8 {
		#[inline] fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPowerLayer = core::Ptr<crate::dnn::PowerLayer>;
	
	ptr_extern! { crate::dnn::PowerLayer,
		cv_PtrOfPowerLayer_delete, cv_PtrOfPowerLayer_get_inner_ptr, cv_PtrOfPowerLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::PowerLayer, cv_PtrOfPowerLayer_new }
	
	impl PtrOfPowerLayer {
		#[inline] pub fn as_raw_PtrOfPowerLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPowerLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::PowerLayerTraitConst for PtrOfPowerLayer {
		#[inline] fn as_raw_PowerLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::PowerLayerTrait for PtrOfPowerLayer {
		#[inline] fn as_raw_mut_PowerLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfPowerLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfPowerLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfPowerLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfPowerLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfPowerLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfPowerLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPriorBoxLayer = core::Ptr<crate::dnn::PriorBoxLayer>;
	
	ptr_extern! { crate::dnn::PriorBoxLayer,
		cv_PtrOfPriorBoxLayer_delete, cv_PtrOfPriorBoxLayer_get_inner_ptr, cv_PtrOfPriorBoxLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::PriorBoxLayer, cv_PtrOfPriorBoxLayer_new }
	
	impl PtrOfPriorBoxLayer {
		#[inline] pub fn as_raw_PtrOfPriorBoxLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPriorBoxLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::PriorBoxLayerTraitConst for PtrOfPriorBoxLayer {
		#[inline] fn as_raw_PriorBoxLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::PriorBoxLayerTrait for PtrOfPriorBoxLayer {
		#[inline] fn as_raw_mut_PriorBoxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfPriorBoxLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfPriorBoxLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfPriorBoxLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfPriorBoxLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfProposalLayer = core::Ptr<crate::dnn::ProposalLayer>;
	
	ptr_extern! { crate::dnn::ProposalLayer,
		cv_PtrOfProposalLayer_delete, cv_PtrOfProposalLayer_get_inner_ptr, cv_PtrOfProposalLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ProposalLayer, cv_PtrOfProposalLayer_new }
	
	impl PtrOfProposalLayer {
		#[inline] pub fn as_raw_PtrOfProposalLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfProposalLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ProposalLayerTraitConst for PtrOfProposalLayer {
		#[inline] fn as_raw_ProposalLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ProposalLayerTrait for PtrOfProposalLayer {
		#[inline] fn as_raw_mut_ProposalLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfProposalLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfProposalLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfProposalLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfProposalLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfQuantizeLayer = core::Ptr<crate::dnn::QuantizeLayer>;
	
	ptr_extern! { crate::dnn::QuantizeLayer,
		cv_PtrOfQuantizeLayer_delete, cv_PtrOfQuantizeLayer_get_inner_ptr, cv_PtrOfQuantizeLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::QuantizeLayer, cv_PtrOfQuantizeLayer_new }
	
	impl PtrOfQuantizeLayer {
		#[inline] pub fn as_raw_PtrOfQuantizeLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfQuantizeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::QuantizeLayerTraitConst for PtrOfQuantizeLayer {
		#[inline] fn as_raw_QuantizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::QuantizeLayerTrait for PtrOfQuantizeLayer {
		#[inline] fn as_raw_mut_QuantizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfQuantizeLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfQuantizeLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfQuantizeLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfQuantizeLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRNNLayer = core::Ptr<dyn crate::dnn::RNNLayer>;
	
	ptr_extern! { dyn crate::dnn::RNNLayer,
		cv_PtrOfRNNLayer_delete, cv_PtrOfRNNLayer_get_inner_ptr, cv_PtrOfRNNLayer_get_inner_ptr_mut
	}
	
	impl PtrOfRNNLayer {
		#[inline] pub fn as_raw_PtrOfRNNLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRNNLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::RNNLayerConst for PtrOfRNNLayer {
		#[inline] fn as_raw_RNNLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::RNNLayer for PtrOfRNNLayer {
		#[inline] fn as_raw_mut_RNNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfRNNLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfRNNLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfRNNLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfRNNLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReLU6Layer = core::Ptr<crate::dnn::ReLU6Layer>;
	
	ptr_extern! { crate::dnn::ReLU6Layer,
		cv_PtrOfReLU6Layer_delete, cv_PtrOfReLU6Layer_get_inner_ptr, cv_PtrOfReLU6Layer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReLU6Layer, cv_PtrOfReLU6Layer_new }
	
	impl PtrOfReLU6Layer {
		#[inline] pub fn as_raw_PtrOfReLU6Layer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReLU6Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReLU6LayerTraitConst for PtrOfReLU6Layer {
		#[inline] fn as_raw_ReLU6Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReLU6LayerTrait for PtrOfReLU6Layer {
		#[inline] fn as_raw_mut_ReLU6Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfReLU6Layer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfReLU6Layer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfReLU6Layer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfReLU6Layer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfReLU6Layer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfReLU6Layer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReLULayer = core::Ptr<crate::dnn::ReLULayer>;
	
	ptr_extern! { crate::dnn::ReLULayer,
		cv_PtrOfReLULayer_delete, cv_PtrOfReLULayer_get_inner_ptr, cv_PtrOfReLULayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReLULayer, cv_PtrOfReLULayer_new }
	
	impl PtrOfReLULayer {
		#[inline] pub fn as_raw_PtrOfReLULayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReLULayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReLULayerTraitConst for PtrOfReLULayer {
		#[inline] fn as_raw_ReLULayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReLULayerTrait for PtrOfReLULayer {
		#[inline] fn as_raw_mut_ReLULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfReLULayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfReLULayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfReLULayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfReLULayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfReLULayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfReLULayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReciprocalLayer = core::Ptr<crate::dnn::ReciprocalLayer>;
	
	ptr_extern! { crate::dnn::ReciprocalLayer,
		cv_PtrOfReciprocalLayer_delete, cv_PtrOfReciprocalLayer_get_inner_ptr, cv_PtrOfReciprocalLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReciprocalLayer, cv_PtrOfReciprocalLayer_new }
	
	impl PtrOfReciprocalLayer {
		#[inline] pub fn as_raw_PtrOfReciprocalLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReciprocalLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReciprocalLayerTraitConst for PtrOfReciprocalLayer {
		#[inline] fn as_raw_ReciprocalLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReciprocalLayerTrait for PtrOfReciprocalLayer {
		#[inline] fn as_raw_mut_ReciprocalLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfReciprocalLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfReciprocalLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfReciprocalLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfReciprocalLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfReciprocalLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfReciprocalLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReduceLayer = core::Ptr<crate::dnn::ReduceLayer>;
	
	ptr_extern! { crate::dnn::ReduceLayer,
		cv_PtrOfReduceLayer_delete, cv_PtrOfReduceLayer_get_inner_ptr, cv_PtrOfReduceLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReduceLayer, cv_PtrOfReduceLayer_new }
	
	impl PtrOfReduceLayer {
		#[inline] pub fn as_raw_PtrOfReduceLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReduceLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReduceLayerTraitConst for PtrOfReduceLayer {
		#[inline] fn as_raw_ReduceLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReduceLayerTrait for PtrOfReduceLayer {
		#[inline] fn as_raw_mut_ReduceLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfReduceLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfReduceLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfReduceLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfReduceLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReduceLayerInt8 = core::Ptr<crate::dnn::ReduceLayerInt8>;
	
	ptr_extern! { crate::dnn::ReduceLayerInt8,
		cv_PtrOfReduceLayerInt8_delete, cv_PtrOfReduceLayerInt8_get_inner_ptr, cv_PtrOfReduceLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReduceLayerInt8, cv_PtrOfReduceLayerInt8_new }
	
	impl PtrOfReduceLayerInt8 {
		#[inline] pub fn as_raw_PtrOfReduceLayerInt8(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReduceLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReduceLayerInt8TraitConst for PtrOfReduceLayerInt8 {
		#[inline] fn as_raw_ReduceLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReduceLayerInt8Trait for PtrOfReduceLayerInt8 {
		#[inline] fn as_raw_mut_ReduceLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfReduceLayerInt8 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfReduceLayerInt8 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfReduceLayerInt8 {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfReduceLayerInt8 {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ReduceLayerTraitConst for PtrOfReduceLayerInt8 {
		#[inline] fn as_raw_ReduceLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReduceLayerTrait for PtrOfReduceLayerInt8 {
		#[inline] fn as_raw_mut_ReduceLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRegionLayer = core::Ptr<crate::dnn::RegionLayer>;
	
	ptr_extern! { crate::dnn::RegionLayer,
		cv_PtrOfRegionLayer_delete, cv_PtrOfRegionLayer_get_inner_ptr, cv_PtrOfRegionLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::RegionLayer, cv_PtrOfRegionLayer_new }
	
	impl PtrOfRegionLayer {
		#[inline] pub fn as_raw_PtrOfRegionLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRegionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::RegionLayerTraitConst for PtrOfRegionLayer {
		#[inline] fn as_raw_RegionLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::RegionLayerTrait for PtrOfRegionLayer {
		#[inline] fn as_raw_mut_RegionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfRegionLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfRegionLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfRegionLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfRegionLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReorgLayer = core::Ptr<crate::dnn::ReorgLayer>;
	
	ptr_extern! { crate::dnn::ReorgLayer,
		cv_PtrOfReorgLayer_delete, cv_PtrOfReorgLayer_get_inner_ptr, cv_PtrOfReorgLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReorgLayer, cv_PtrOfReorgLayer_new }
	
	impl PtrOfReorgLayer {
		#[inline] pub fn as_raw_PtrOfReorgLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReorgLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReorgLayerTraitConst for PtrOfReorgLayer {
		#[inline] fn as_raw_ReorgLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReorgLayerTrait for PtrOfReorgLayer {
		#[inline] fn as_raw_mut_ReorgLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfReorgLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfReorgLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfReorgLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfReorgLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRequantizeLayer = core::Ptr<crate::dnn::RequantizeLayer>;
	
	ptr_extern! { crate::dnn::RequantizeLayer,
		cv_PtrOfRequantizeLayer_delete, cv_PtrOfRequantizeLayer_get_inner_ptr, cv_PtrOfRequantizeLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::RequantizeLayer, cv_PtrOfRequantizeLayer_new }
	
	impl PtrOfRequantizeLayer {
		#[inline] pub fn as_raw_PtrOfRequantizeLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRequantizeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::RequantizeLayerTraitConst for PtrOfRequantizeLayer {
		#[inline] fn as_raw_RequantizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::RequantizeLayerTrait for PtrOfRequantizeLayer {
		#[inline] fn as_raw_mut_RequantizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfRequantizeLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfRequantizeLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfRequantizeLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfRequantizeLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReshapeLayer = core::Ptr<crate::dnn::ReshapeLayer>;
	
	ptr_extern! { crate::dnn::ReshapeLayer,
		cv_PtrOfReshapeLayer_delete, cv_PtrOfReshapeLayer_get_inner_ptr, cv_PtrOfReshapeLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReshapeLayer, cv_PtrOfReshapeLayer_new }
	
	impl PtrOfReshapeLayer {
		#[inline] pub fn as_raw_PtrOfReshapeLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReshapeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReshapeLayerTraitConst for PtrOfReshapeLayer {
		#[inline] fn as_raw_ReshapeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReshapeLayerTrait for PtrOfReshapeLayer {
		#[inline] fn as_raw_mut_ReshapeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfReshapeLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfReshapeLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfReshapeLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfReshapeLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfResizeLayer = core::Ptr<crate::dnn::ResizeLayer>;
	
	ptr_extern! { crate::dnn::ResizeLayer,
		cv_PtrOfResizeLayer_delete, cv_PtrOfResizeLayer_get_inner_ptr, cv_PtrOfResizeLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ResizeLayer, cv_PtrOfResizeLayer_new }
	
	impl PtrOfResizeLayer {
		#[inline] pub fn as_raw_PtrOfResizeLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfResizeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ResizeLayerTraitConst for PtrOfResizeLayer {
		#[inline] fn as_raw_ResizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ResizeLayerTrait for PtrOfResizeLayer {
		#[inline] fn as_raw_mut_ResizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfResizeLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfResizeLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfResizeLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfResizeLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRoundLayer = core::Ptr<crate::dnn::RoundLayer>;
	
	ptr_extern! { crate::dnn::RoundLayer,
		cv_PtrOfRoundLayer_delete, cv_PtrOfRoundLayer_get_inner_ptr, cv_PtrOfRoundLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::RoundLayer, cv_PtrOfRoundLayer_new }
	
	impl PtrOfRoundLayer {
		#[inline] pub fn as_raw_PtrOfRoundLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRoundLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::RoundLayerTraitConst for PtrOfRoundLayer {
		#[inline] fn as_raw_RoundLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::RoundLayerTrait for PtrOfRoundLayer {
		#[inline] fn as_raw_mut_RoundLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfRoundLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfRoundLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfRoundLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfRoundLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfRoundLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfRoundLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfScaleLayer = core::Ptr<crate::dnn::ScaleLayer>;
	
	ptr_extern! { crate::dnn::ScaleLayer,
		cv_PtrOfScaleLayer_delete, cv_PtrOfScaleLayer_get_inner_ptr, cv_PtrOfScaleLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ScaleLayer, cv_PtrOfScaleLayer_new }
	
	impl PtrOfScaleLayer {
		#[inline] pub fn as_raw_PtrOfScaleLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfScaleLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ScaleLayerTraitConst for PtrOfScaleLayer {
		#[inline] fn as_raw_ScaleLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ScaleLayerTrait for PtrOfScaleLayer {
		#[inline] fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfScaleLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfScaleLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfScaleLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfScaleLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfScaleLayerInt8 = core::Ptr<crate::dnn::ScaleLayerInt8>;
	
	ptr_extern! { crate::dnn::ScaleLayerInt8,
		cv_PtrOfScaleLayerInt8_delete, cv_PtrOfScaleLayerInt8_get_inner_ptr, cv_PtrOfScaleLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ScaleLayerInt8, cv_PtrOfScaleLayerInt8_new }
	
	impl PtrOfScaleLayerInt8 {
		#[inline] pub fn as_raw_PtrOfScaleLayerInt8(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfScaleLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ScaleLayerInt8TraitConst for PtrOfScaleLayerInt8 {
		#[inline] fn as_raw_ScaleLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ScaleLayerInt8Trait for PtrOfScaleLayerInt8 {
		#[inline] fn as_raw_mut_ScaleLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfScaleLayerInt8 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfScaleLayerInt8 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfScaleLayerInt8 {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfScaleLayerInt8 {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ScaleLayerTraitConst for PtrOfScaleLayerInt8 {
		#[inline] fn as_raw_ScaleLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ScaleLayerTrait for PtrOfScaleLayerInt8 {
		#[inline] fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSeluLayer = core::Ptr<crate::dnn::SeluLayer>;
	
	ptr_extern! { crate::dnn::SeluLayer,
		cv_PtrOfSeluLayer_delete, cv_PtrOfSeluLayer_get_inner_ptr, cv_PtrOfSeluLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SeluLayer, cv_PtrOfSeluLayer_new }
	
	impl PtrOfSeluLayer {
		#[inline] pub fn as_raw_PtrOfSeluLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSeluLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SeluLayerTraitConst for PtrOfSeluLayer {
		#[inline] fn as_raw_SeluLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SeluLayerTrait for PtrOfSeluLayer {
		#[inline] fn as_raw_mut_SeluLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSeluLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSeluLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfSeluLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfSeluLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfSeluLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfSeluLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfShrinkLayer = core::Ptr<crate::dnn::ShrinkLayer>;
	
	ptr_extern! { crate::dnn::ShrinkLayer,
		cv_PtrOfShrinkLayer_delete, cv_PtrOfShrinkLayer_get_inner_ptr, cv_PtrOfShrinkLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ShrinkLayer, cv_PtrOfShrinkLayer_new }
	
	impl PtrOfShrinkLayer {
		#[inline] pub fn as_raw_PtrOfShrinkLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfShrinkLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ShrinkLayerTraitConst for PtrOfShrinkLayer {
		#[inline] fn as_raw_ShrinkLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ShrinkLayerTrait for PtrOfShrinkLayer {
		#[inline] fn as_raw_mut_ShrinkLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfShrinkLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfShrinkLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfShrinkLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfShrinkLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfShrinkLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfShrinkLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSigmoidLayer = core::Ptr<crate::dnn::SigmoidLayer>;
	
	ptr_extern! { crate::dnn::SigmoidLayer,
		cv_PtrOfSigmoidLayer_delete, cv_PtrOfSigmoidLayer_get_inner_ptr, cv_PtrOfSigmoidLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SigmoidLayer, cv_PtrOfSigmoidLayer_new }
	
	impl PtrOfSigmoidLayer {
		#[inline] pub fn as_raw_PtrOfSigmoidLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSigmoidLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SigmoidLayerTraitConst for PtrOfSigmoidLayer {
		#[inline] fn as_raw_SigmoidLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SigmoidLayerTrait for PtrOfSigmoidLayer {
		#[inline] fn as_raw_mut_SigmoidLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSigmoidLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSigmoidLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfSigmoidLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfSigmoidLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfSigmoidLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfSigmoidLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSignLayer = core::Ptr<crate::dnn::SignLayer>;
	
	ptr_extern! { crate::dnn::SignLayer,
		cv_PtrOfSignLayer_delete, cv_PtrOfSignLayer_get_inner_ptr, cv_PtrOfSignLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SignLayer, cv_PtrOfSignLayer_new }
	
	impl PtrOfSignLayer {
		#[inline] pub fn as_raw_PtrOfSignLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSignLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SignLayerTraitConst for PtrOfSignLayer {
		#[inline] fn as_raw_SignLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SignLayerTrait for PtrOfSignLayer {
		#[inline] fn as_raw_mut_SignLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSignLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSignLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfSignLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfSignLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfSignLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfSignLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSinLayer = core::Ptr<crate::dnn::SinLayer>;
	
	ptr_extern! { crate::dnn::SinLayer,
		cv_PtrOfSinLayer_delete, cv_PtrOfSinLayer_get_inner_ptr, cv_PtrOfSinLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SinLayer, cv_PtrOfSinLayer_new }
	
	impl PtrOfSinLayer {
		#[inline] pub fn as_raw_PtrOfSinLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSinLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SinLayerTraitConst for PtrOfSinLayer {
		#[inline] fn as_raw_SinLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SinLayerTrait for PtrOfSinLayer {
		#[inline] fn as_raw_mut_SinLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSinLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSinLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfSinLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfSinLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfSinLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfSinLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSinhLayer = core::Ptr<crate::dnn::SinhLayer>;
	
	ptr_extern! { crate::dnn::SinhLayer,
		cv_PtrOfSinhLayer_delete, cv_PtrOfSinhLayer_get_inner_ptr, cv_PtrOfSinhLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SinhLayer, cv_PtrOfSinhLayer_new }
	
	impl PtrOfSinhLayer {
		#[inline] pub fn as_raw_PtrOfSinhLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSinhLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SinhLayerTraitConst for PtrOfSinhLayer {
		#[inline] fn as_raw_SinhLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SinhLayerTrait for PtrOfSinhLayer {
		#[inline] fn as_raw_mut_SinhLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSinhLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSinhLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfSinhLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfSinhLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfSinhLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfSinhLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSliceLayer = core::Ptr<crate::dnn::SliceLayer>;
	
	ptr_extern! { crate::dnn::SliceLayer,
		cv_PtrOfSliceLayer_delete, cv_PtrOfSliceLayer_get_inner_ptr, cv_PtrOfSliceLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SliceLayer, cv_PtrOfSliceLayer_new }
	
	impl PtrOfSliceLayer {
		#[inline] pub fn as_raw_PtrOfSliceLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSliceLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SliceLayerTraitConst for PtrOfSliceLayer {
		#[inline] fn as_raw_SliceLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SliceLayerTrait for PtrOfSliceLayer {
		#[inline] fn as_raw_mut_SliceLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSliceLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSliceLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfSliceLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfSliceLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSoftmaxLayer = core::Ptr<crate::dnn::SoftmaxLayer>;
	
	ptr_extern! { crate::dnn::SoftmaxLayer,
		cv_PtrOfSoftmaxLayer_delete, cv_PtrOfSoftmaxLayer_get_inner_ptr, cv_PtrOfSoftmaxLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SoftmaxLayer, cv_PtrOfSoftmaxLayer_new }
	
	impl PtrOfSoftmaxLayer {
		#[inline] pub fn as_raw_PtrOfSoftmaxLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSoftmaxLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SoftmaxLayerTraitConst for PtrOfSoftmaxLayer {
		#[inline] fn as_raw_SoftmaxLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SoftmaxLayerTrait for PtrOfSoftmaxLayer {
		#[inline] fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSoftmaxLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSoftmaxLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfSoftmaxLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfSoftmaxLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSoftmaxLayerInt8 = core::Ptr<crate::dnn::SoftmaxLayerInt8>;
	
	ptr_extern! { crate::dnn::SoftmaxLayerInt8,
		cv_PtrOfSoftmaxLayerInt8_delete, cv_PtrOfSoftmaxLayerInt8_get_inner_ptr, cv_PtrOfSoftmaxLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SoftmaxLayerInt8, cv_PtrOfSoftmaxLayerInt8_new }
	
	impl PtrOfSoftmaxLayerInt8 {
		#[inline] pub fn as_raw_PtrOfSoftmaxLayerInt8(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSoftmaxLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SoftmaxLayerInt8TraitConst for PtrOfSoftmaxLayerInt8 {
		#[inline] fn as_raw_SoftmaxLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SoftmaxLayerInt8Trait for PtrOfSoftmaxLayerInt8 {
		#[inline] fn as_raw_mut_SoftmaxLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSoftmaxLayerInt8 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSoftmaxLayerInt8 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfSoftmaxLayerInt8 {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfSoftmaxLayerInt8 {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::SoftmaxLayerTraitConst for PtrOfSoftmaxLayerInt8 {
		#[inline] fn as_raw_SoftmaxLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SoftmaxLayerTrait for PtrOfSoftmaxLayerInt8 {
		#[inline] fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSoftplusLayer = core::Ptr<crate::dnn::SoftplusLayer>;
	
	ptr_extern! { crate::dnn::SoftplusLayer,
		cv_PtrOfSoftplusLayer_delete, cv_PtrOfSoftplusLayer_get_inner_ptr, cv_PtrOfSoftplusLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SoftplusLayer, cv_PtrOfSoftplusLayer_new }
	
	impl PtrOfSoftplusLayer {
		#[inline] pub fn as_raw_PtrOfSoftplusLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSoftplusLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SoftplusLayerTraitConst for PtrOfSoftplusLayer {
		#[inline] fn as_raw_SoftplusLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SoftplusLayerTrait for PtrOfSoftplusLayer {
		#[inline] fn as_raw_mut_SoftplusLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSoftplusLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSoftplusLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfSoftplusLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfSoftplusLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfSoftplusLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfSoftplusLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSoftsignLayer = core::Ptr<crate::dnn::SoftsignLayer>;
	
	ptr_extern! { crate::dnn::SoftsignLayer,
		cv_PtrOfSoftsignLayer_delete, cv_PtrOfSoftsignLayer_get_inner_ptr, cv_PtrOfSoftsignLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SoftsignLayer, cv_PtrOfSoftsignLayer_new }
	
	impl PtrOfSoftsignLayer {
		#[inline] pub fn as_raw_PtrOfSoftsignLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSoftsignLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SoftsignLayerTraitConst for PtrOfSoftsignLayer {
		#[inline] fn as_raw_SoftsignLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SoftsignLayerTrait for PtrOfSoftsignLayer {
		#[inline] fn as_raw_mut_SoftsignLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSoftsignLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSoftsignLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfSoftsignLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfSoftsignLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfSoftsignLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfSoftsignLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSplitLayer = core::Ptr<crate::dnn::SplitLayer>;
	
	ptr_extern! { crate::dnn::SplitLayer,
		cv_PtrOfSplitLayer_delete, cv_PtrOfSplitLayer_get_inner_ptr, cv_PtrOfSplitLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SplitLayer, cv_PtrOfSplitLayer_new }
	
	impl PtrOfSplitLayer {
		#[inline] pub fn as_raw_PtrOfSplitLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSplitLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SplitLayerTraitConst for PtrOfSplitLayer {
		#[inline] fn as_raw_SplitLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SplitLayerTrait for PtrOfSplitLayer {
		#[inline] fn as_raw_mut_SplitLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSplitLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSplitLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfSplitLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfSplitLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSqrtLayer = core::Ptr<crate::dnn::SqrtLayer>;
	
	ptr_extern! { crate::dnn::SqrtLayer,
		cv_PtrOfSqrtLayer_delete, cv_PtrOfSqrtLayer_get_inner_ptr, cv_PtrOfSqrtLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SqrtLayer, cv_PtrOfSqrtLayer_new }
	
	impl PtrOfSqrtLayer {
		#[inline] pub fn as_raw_PtrOfSqrtLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSqrtLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SqrtLayerTraitConst for PtrOfSqrtLayer {
		#[inline] fn as_raw_SqrtLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SqrtLayerTrait for PtrOfSqrtLayer {
		#[inline] fn as_raw_mut_SqrtLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSqrtLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSqrtLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfSqrtLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfSqrtLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfSqrtLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfSqrtLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSwishLayer = core::Ptr<crate::dnn::SwishLayer>;
	
	ptr_extern! { crate::dnn::SwishLayer,
		cv_PtrOfSwishLayer_delete, cv_PtrOfSwishLayer_get_inner_ptr, cv_PtrOfSwishLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SwishLayer, cv_PtrOfSwishLayer_new }
	
	impl PtrOfSwishLayer {
		#[inline] pub fn as_raw_PtrOfSwishLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSwishLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::SwishLayerTraitConst for PtrOfSwishLayer {
		#[inline] fn as_raw_SwishLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SwishLayerTrait for PtrOfSwishLayer {
		#[inline] fn as_raw_mut_SwishLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSwishLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSwishLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfSwishLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfSwishLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfSwishLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfSwishLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTanHLayer = core::Ptr<crate::dnn::TanHLayer>;
	
	ptr_extern! { crate::dnn::TanHLayer,
		cv_PtrOfTanHLayer_delete, cv_PtrOfTanHLayer_get_inner_ptr, cv_PtrOfTanHLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::TanHLayer, cv_PtrOfTanHLayer_new }
	
	impl PtrOfTanHLayer {
		#[inline] pub fn as_raw_PtrOfTanHLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTanHLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::TanHLayerTraitConst for PtrOfTanHLayer {
		#[inline] fn as_raw_TanHLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::TanHLayerTrait for PtrOfTanHLayer {
		#[inline] fn as_raw_mut_TanHLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfTanHLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfTanHLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfTanHLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfTanHLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfTanHLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfTanHLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTanLayer = core::Ptr<crate::dnn::TanLayer>;
	
	ptr_extern! { crate::dnn::TanLayer,
		cv_PtrOfTanLayer_delete, cv_PtrOfTanLayer_get_inner_ptr, cv_PtrOfTanLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::TanLayer, cv_PtrOfTanLayer_new }
	
	impl PtrOfTanLayer {
		#[inline] pub fn as_raw_PtrOfTanLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTanLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::TanLayerTraitConst for PtrOfTanLayer {
		#[inline] fn as_raw_TanLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::TanLayerTrait for PtrOfTanLayer {
		#[inline] fn as_raw_mut_TanLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfTanLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfTanLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfTanLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfTanLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfTanLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfTanLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfThresholdedReluLayer = core::Ptr<crate::dnn::ThresholdedReluLayer>;
	
	ptr_extern! { crate::dnn::ThresholdedReluLayer,
		cv_PtrOfThresholdedReluLayer_delete, cv_PtrOfThresholdedReluLayer_get_inner_ptr, cv_PtrOfThresholdedReluLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ThresholdedReluLayer, cv_PtrOfThresholdedReluLayer_new }
	
	impl PtrOfThresholdedReluLayer {
		#[inline] pub fn as_raw_PtrOfThresholdedReluLayer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfThresholdedReluLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn::ThresholdedReluLayerTraitConst for PtrOfThresholdedReluLayer {
		#[inline] fn as_raw_ThresholdedReluLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ThresholdedReluLayerTrait for PtrOfThresholdedReluLayer {
		#[inline] fn as_raw_mut_ThresholdedReluLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfThresholdedReluLayer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfThresholdedReluLayer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for PtrOfThresholdedReluLayer {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for PtrOfThresholdedReluLayer {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for PtrOfThresholdedReluLayer {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for PtrOfThresholdedReluLayer {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfMatShape = core::Vector<crate::dnn::MatShape>;
	
	impl VectorOfMatShape {
		pub fn as_raw_VectorOfMatShape(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfMatShape(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	pub type VectorOfPtrOfBackendNode = core::Vector<core::Ptr<crate::dnn::BackendNode>>;
	
	impl VectorOfPtrOfBackendNode {
		pub fn as_raw_VectorOfPtrOfBackendNode(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfBackendNode(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Ptr<crate::dnn::BackendNode>, *const c_void, *mut c_void,
		cv_VectorOfPtrOfBackendNode_new, cv_VectorOfPtrOfBackendNode_delete,
		cv_VectorOfPtrOfBackendNode_len, cv_VectorOfPtrOfBackendNode_is_empty,
		cv_VectorOfPtrOfBackendNode_capacity, cv_VectorOfPtrOfBackendNode_shrink_to_fit,
		cv_VectorOfPtrOfBackendNode_reserve, cv_VectorOfPtrOfBackendNode_remove,
		cv_VectorOfPtrOfBackendNode_swap, cv_VectorOfPtrOfBackendNode_clear,
		cv_VectorOfPtrOfBackendNode_get, cv_VectorOfPtrOfBackendNode_set,
		cv_VectorOfPtrOfBackendNode_push, cv_VectorOfPtrOfBackendNode_insert,
	}
	vector_non_copy_or_bool! { core::Ptr<crate::dnn::BackendNode> }
	
	unsafe impl Send for core::Vector<core::Ptr<crate::dnn::BackendNode>> {}
	
	pub type VectorOfPtrOfBackendWrapper = core::Vector<core::Ptr<dyn crate::dnn::BackendWrapper>>;
	
	impl VectorOfPtrOfBackendWrapper {
		pub fn as_raw_VectorOfPtrOfBackendWrapper(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfBackendWrapper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Ptr<dyn crate::dnn::BackendWrapper>, *const c_void, *mut c_void,
		cv_VectorOfPtrOfBackendWrapper_new, cv_VectorOfPtrOfBackendWrapper_delete,
		cv_VectorOfPtrOfBackendWrapper_len, cv_VectorOfPtrOfBackendWrapper_is_empty,
		cv_VectorOfPtrOfBackendWrapper_capacity, cv_VectorOfPtrOfBackendWrapper_shrink_to_fit,
		cv_VectorOfPtrOfBackendWrapper_reserve, cv_VectorOfPtrOfBackendWrapper_remove,
		cv_VectorOfPtrOfBackendWrapper_swap, cv_VectorOfPtrOfBackendWrapper_clear,
		cv_VectorOfPtrOfBackendWrapper_get, cv_VectorOfPtrOfBackendWrapper_set,
		cv_VectorOfPtrOfBackendWrapper_push, cv_VectorOfPtrOfBackendWrapper_insert,
	}
	vector_non_copy_or_bool! { core::Ptr<dyn crate::dnn::BackendWrapper> }
	
	unsafe impl Send for core::Vector<core::Ptr<dyn crate::dnn::BackendWrapper>> {}
	
	pub type VectorOfPtrOfLayer = core::Vector<core::Ptr<crate::dnn::Layer>>;
	
	impl VectorOfPtrOfLayer {
		pub fn as_raw_VectorOfPtrOfLayer(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Ptr<crate::dnn::Layer>, *const c_void, *mut c_void,
		cv_VectorOfPtrOfLayer_new, cv_VectorOfPtrOfLayer_delete,
		cv_VectorOfPtrOfLayer_len, cv_VectorOfPtrOfLayer_is_empty,
		cv_VectorOfPtrOfLayer_capacity, cv_VectorOfPtrOfLayer_shrink_to_fit,
		cv_VectorOfPtrOfLayer_reserve, cv_VectorOfPtrOfLayer_remove,
		cv_VectorOfPtrOfLayer_swap, cv_VectorOfPtrOfLayer_clear,
		cv_VectorOfPtrOfLayer_get, cv_VectorOfPtrOfLayer_set,
		cv_VectorOfPtrOfLayer_push, cv_VectorOfPtrOfLayer_insert,
	}
	vector_non_copy_or_bool! { core::Ptr<crate::dnn::Layer> }
	
	unsafe impl Send for core::Vector<core::Ptr<crate::dnn::Layer>> {}
	
	pub type VectorOfTarget = core::Vector<crate::dnn::Target>;
	
	impl VectorOfTarget {
		pub fn as_raw_VectorOfTarget(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfTarget(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::dnn::Target, *const c_void, *mut c_void,
		cv_VectorOfTarget_new, cv_VectorOfTarget_delete,
		cv_VectorOfTarget_len, cv_VectorOfTarget_is_empty,
		cv_VectorOfTarget_capacity, cv_VectorOfTarget_shrink_to_fit,
		cv_VectorOfTarget_reserve, cv_VectorOfTarget_remove,
		cv_VectorOfTarget_swap, cv_VectorOfTarget_clear,
		cv_VectorOfTarget_get, cv_VectorOfTarget_set,
		cv_VectorOfTarget_push, cv_VectorOfTarget_insert,
	}
	vector_copy_non_bool! { crate::dnn::Target, *const c_void, *mut c_void,
		cv_VectorOfTarget_data, cv_VectorOfTarget_data_mut, cv_VectorOfTarget_from_slice,
		cv_VectorOfTarget_clone,
	}
	
	unsafe impl Send for core::Vector<crate::dnn::Target> {}
	
	pub type VectorOfVectorOfMatShape = core::Vector<core::Vector<crate::dnn::MatShape>>;
	
	impl VectorOfVectorOfMatShape {
		pub fn as_raw_VectorOfVectorOfMatShape(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfMatShape(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<crate::dnn::MatShape>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfMatShape_new, cv_VectorOfVectorOfMatShape_delete,
		cv_VectorOfVectorOfMatShape_len, cv_VectorOfVectorOfMatShape_is_empty,
		cv_VectorOfVectorOfMatShape_capacity, cv_VectorOfVectorOfMatShape_shrink_to_fit,
		cv_VectorOfVectorOfMatShape_reserve, cv_VectorOfVectorOfMatShape_remove,
		cv_VectorOfVectorOfMatShape_swap, cv_VectorOfVectorOfMatShape_clear,
		cv_VectorOfVectorOfMatShape_get, cv_VectorOfVectorOfMatShape_set,
		cv_VectorOfVectorOfMatShape_push, cv_VectorOfVectorOfMatShape_insert,
	}
	vector_non_copy_or_bool! { core::Vector<crate::dnn::MatShape> }
	
	unsafe impl Send for core::Vector<core::Vector<crate::dnn::MatShape>> {}
	
}
#[cfg(ocvrs_has_module_dnn)]
pub use dnn_types::*;

#[cfg(ocvrs_has_module_dnn_superres)]
mod dnn_superres_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfDnnSuperResImpl = core::Ptr<crate::dnn_superres::DnnSuperResImpl>;
	
	ptr_extern! { crate::dnn_superres::DnnSuperResImpl,
		cv_PtrOfDnnSuperResImpl_delete, cv_PtrOfDnnSuperResImpl_get_inner_ptr, cv_PtrOfDnnSuperResImpl_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn_superres::DnnSuperResImpl, cv_PtrOfDnnSuperResImpl_new }
	
	impl PtrOfDnnSuperResImpl {
		#[inline] pub fn as_raw_PtrOfDnnSuperResImpl(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDnnSuperResImpl(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dnn_superres::DnnSuperResImplTraitConst for PtrOfDnnSuperResImpl {
		#[inline] fn as_raw_DnnSuperResImpl(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn_superres::DnnSuperResImplTrait for PtrOfDnnSuperResImpl {
		#[inline] fn as_raw_mut_DnnSuperResImpl(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_dnn_superres)]
pub use dnn_superres_types::*;

#[cfg(ocvrs_has_module_dpm)]
mod dpm_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfDPMDetector = core::Ptr<dyn crate::dpm::DPMDetector>;
	
	ptr_extern! { dyn crate::dpm::DPMDetector,
		cv_PtrOfDPMDetector_delete, cv_PtrOfDPMDetector_get_inner_ptr, cv_PtrOfDPMDetector_get_inner_ptr_mut
	}
	
	impl PtrOfDPMDetector {
		#[inline] pub fn as_raw_PtrOfDPMDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDPMDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::dpm::DPMDetectorConst for PtrOfDPMDetector {
		#[inline] fn as_raw_DPMDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dpm::DPMDetector for PtrOfDPMDetector {
		#[inline] fn as_raw_mut_DPMDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfDPMDetector_ObjectDetection = core::Vector<crate::dpm::DPMDetector_ObjectDetection>;
	
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
		cv_VectorOfDPMDetector_ObjectDetection_get, cv_VectorOfDPMDetector_ObjectDetection_set,
		cv_VectorOfDPMDetector_ObjectDetection_push, cv_VectorOfDPMDetector_ObjectDetection_insert,
	}
	vector_non_copy_or_bool! { crate::dpm::DPMDetector_ObjectDetection }
	
	unsafe impl Send for core::Vector<crate::dpm::DPMDetector_ObjectDetection> {}
	
}
#[cfg(ocvrs_has_module_dpm)]
pub use dpm_types::*;

#[cfg(ocvrs_has_module_face)]
mod face_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBIF = core::Ptr<dyn crate::face::BIF>;
	
	ptr_extern! { dyn crate::face::BIF,
		cv_PtrOfBIF_delete, cv_PtrOfBIF_get_inner_ptr, cv_PtrOfBIF_get_inner_ptr_mut
	}
	
	impl PtrOfBIF {
		#[inline] pub fn as_raw_PtrOfBIF(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBIF(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::BIFConst for PtrOfBIF {
		#[inline] fn as_raw_BIF(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::BIF for PtrOfBIF {
		#[inline] fn as_raw_mut_BIF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfBIF {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfBIF {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEigenFaceRecognizer = core::Ptr<dyn crate::face::EigenFaceRecognizer>;
	
	ptr_extern! { dyn crate::face::EigenFaceRecognizer,
		cv_PtrOfEigenFaceRecognizer_delete, cv_PtrOfEigenFaceRecognizer_get_inner_ptr, cv_PtrOfEigenFaceRecognizer_get_inner_ptr_mut
	}
	
	impl PtrOfEigenFaceRecognizer {
		#[inline] pub fn as_raw_PtrOfEigenFaceRecognizer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEigenFaceRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::EigenFaceRecognizerConst for PtrOfEigenFaceRecognizer {
		#[inline] fn as_raw_EigenFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::EigenFaceRecognizer for PtrOfEigenFaceRecognizer {
		#[inline] fn as_raw_mut_EigenFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfEigenFaceRecognizer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfEigenFaceRecognizer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::BasicFaceRecognizerConst for PtrOfEigenFaceRecognizer {
		#[inline] fn as_raw_BasicFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::BasicFaceRecognizer for PtrOfEigenFaceRecognizer {
		#[inline] fn as_raw_mut_BasicFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FaceRecognizerConst for PtrOfEigenFaceRecognizer {
		#[inline] fn as_raw_FaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::FaceRecognizer for PtrOfEigenFaceRecognizer {
		#[inline] fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFacemark = core::Ptr<dyn crate::face::Facemark>;
	
	ptr_extern! { dyn crate::face::Facemark,
		cv_PtrOfFacemark_delete, cv_PtrOfFacemark_get_inner_ptr, cv_PtrOfFacemark_get_inner_ptr_mut
	}
	
	impl PtrOfFacemark {
		#[inline] pub fn as_raw_PtrOfFacemark(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFacemark(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::FacemarkConst for PtrOfFacemark {
		#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::Facemark for PtrOfFacemark {
		#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfFacemark {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfFacemark {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFacemarkAAM = core::Ptr<dyn crate::face::FacemarkAAM>;
	
	ptr_extern! { dyn crate::face::FacemarkAAM,
		cv_PtrOfFacemarkAAM_delete, cv_PtrOfFacemarkAAM_get_inner_ptr, cv_PtrOfFacemarkAAM_get_inner_ptr_mut
	}
	
	impl PtrOfFacemarkAAM {
		#[inline] pub fn as_raw_PtrOfFacemarkAAM(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFacemarkAAM(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::FacemarkAAMConst for PtrOfFacemarkAAM {
		#[inline] fn as_raw_FacemarkAAM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::FacemarkAAM for PtrOfFacemarkAAM {
		#[inline] fn as_raw_mut_FacemarkAAM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfFacemarkAAM {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfFacemarkAAM {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FacemarkConst for PtrOfFacemarkAAM {
		#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::Facemark for PtrOfFacemarkAAM {
		#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FacemarkTrainConst for PtrOfFacemarkAAM {
		#[inline] fn as_raw_FacemarkTrain(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::FacemarkTrain for PtrOfFacemarkAAM {
		#[inline] fn as_raw_mut_FacemarkTrain(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFacemarkKazemi = core::Ptr<dyn crate::face::FacemarkKazemi>;
	
	ptr_extern! { dyn crate::face::FacemarkKazemi,
		cv_PtrOfFacemarkKazemi_delete, cv_PtrOfFacemarkKazemi_get_inner_ptr, cv_PtrOfFacemarkKazemi_get_inner_ptr_mut
	}
	
	impl PtrOfFacemarkKazemi {
		#[inline] pub fn as_raw_PtrOfFacemarkKazemi(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFacemarkKazemi(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::FacemarkKazemiConst for PtrOfFacemarkKazemi {
		#[inline] fn as_raw_FacemarkKazemi(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::FacemarkKazemi for PtrOfFacemarkKazemi {
		#[inline] fn as_raw_mut_FacemarkKazemi(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfFacemarkKazemi {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfFacemarkKazemi {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FacemarkConst for PtrOfFacemarkKazemi {
		#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::Facemark for PtrOfFacemarkKazemi {
		#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFacemarkLBF = core::Ptr<dyn crate::face::FacemarkLBF>;
	
	ptr_extern! { dyn crate::face::FacemarkLBF,
		cv_PtrOfFacemarkLBF_delete, cv_PtrOfFacemarkLBF_get_inner_ptr, cv_PtrOfFacemarkLBF_get_inner_ptr_mut
	}
	
	impl PtrOfFacemarkLBF {
		#[inline] pub fn as_raw_PtrOfFacemarkLBF(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFacemarkLBF(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::FacemarkLBFConst for PtrOfFacemarkLBF {
		#[inline] fn as_raw_FacemarkLBF(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::FacemarkLBF for PtrOfFacemarkLBF {
		#[inline] fn as_raw_mut_FacemarkLBF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfFacemarkLBF {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfFacemarkLBF {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FacemarkConst for PtrOfFacemarkLBF {
		#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::Facemark for PtrOfFacemarkLBF {
		#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FacemarkTrainConst for PtrOfFacemarkLBF {
		#[inline] fn as_raw_FacemarkTrain(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::FacemarkTrain for PtrOfFacemarkLBF {
		#[inline] fn as_raw_mut_FacemarkTrain(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFisherFaceRecognizer = core::Ptr<dyn crate::face::FisherFaceRecognizer>;
	
	ptr_extern! { dyn crate::face::FisherFaceRecognizer,
		cv_PtrOfFisherFaceRecognizer_delete, cv_PtrOfFisherFaceRecognizer_get_inner_ptr, cv_PtrOfFisherFaceRecognizer_get_inner_ptr_mut
	}
	
	impl PtrOfFisherFaceRecognizer {
		#[inline] pub fn as_raw_PtrOfFisherFaceRecognizer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFisherFaceRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::FisherFaceRecognizerConst for PtrOfFisherFaceRecognizer {
		#[inline] fn as_raw_FisherFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::FisherFaceRecognizer for PtrOfFisherFaceRecognizer {
		#[inline] fn as_raw_mut_FisherFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfFisherFaceRecognizer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfFisherFaceRecognizer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::BasicFaceRecognizerConst for PtrOfFisherFaceRecognizer {
		#[inline] fn as_raw_BasicFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::BasicFaceRecognizer for PtrOfFisherFaceRecognizer {
		#[inline] fn as_raw_mut_BasicFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FaceRecognizerConst for PtrOfFisherFaceRecognizer {
		#[inline] fn as_raw_FaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::FaceRecognizer for PtrOfFisherFaceRecognizer {
		#[inline] fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLBPHFaceRecognizer = core::Ptr<dyn crate::face::LBPHFaceRecognizer>;
	
	ptr_extern! { dyn crate::face::LBPHFaceRecognizer,
		cv_PtrOfLBPHFaceRecognizer_delete, cv_PtrOfLBPHFaceRecognizer_get_inner_ptr, cv_PtrOfLBPHFaceRecognizer_get_inner_ptr_mut
	}
	
	impl PtrOfLBPHFaceRecognizer {
		#[inline] pub fn as_raw_PtrOfLBPHFaceRecognizer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLBPHFaceRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::LBPHFaceRecognizerConst for PtrOfLBPHFaceRecognizer {
		#[inline] fn as_raw_LBPHFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::LBPHFaceRecognizer for PtrOfLBPHFaceRecognizer {
		#[inline] fn as_raw_mut_LBPHFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfLBPHFaceRecognizer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfLBPHFaceRecognizer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FaceRecognizerConst for PtrOfLBPHFaceRecognizer {
		#[inline] fn as_raw_FaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::FaceRecognizer for PtrOfLBPHFaceRecognizer {
		#[inline] fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMACE = core::Ptr<dyn crate::face::MACE>;
	
	ptr_extern! { dyn crate::face::MACE,
		cv_PtrOfMACE_delete, cv_PtrOfMACE_get_inner_ptr, cv_PtrOfMACE_get_inner_ptr_mut
	}
	
	impl PtrOfMACE {
		#[inline] pub fn as_raw_PtrOfMACE(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMACE(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::MACEConst for PtrOfMACE {
		#[inline] fn as_raw_MACE(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::MACE for PtrOfMACE {
		#[inline] fn as_raw_mut_MACE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfMACE {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfMACE {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPredictCollector = core::Ptr<dyn crate::face::PredictCollector>;
	
	ptr_extern! { dyn crate::face::PredictCollector,
		cv_PtrOfPredictCollector_delete, cv_PtrOfPredictCollector_get_inner_ptr, cv_PtrOfPredictCollector_get_inner_ptr_mut
	}
	
	impl PtrOfPredictCollector {
		#[inline] pub fn as_raw_PtrOfPredictCollector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPredictCollector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::PredictCollectorConst for PtrOfPredictCollector {
		#[inline] fn as_raw_PredictCollector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::PredictCollector for PtrOfPredictCollector {
		#[inline] fn as_raw_mut_PredictCollector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStandardCollector = core::Ptr<crate::face::StandardCollector>;
	
	ptr_extern! { crate::face::StandardCollector,
		cv_PtrOfStandardCollector_delete, cv_PtrOfStandardCollector_get_inner_ptr, cv_PtrOfStandardCollector_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::face::StandardCollector, cv_PtrOfStandardCollector_new }
	
	impl PtrOfStandardCollector {
		#[inline] pub fn as_raw_PtrOfStandardCollector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStandardCollector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::face::StandardCollectorTraitConst for PtrOfStandardCollector {
		#[inline] fn as_raw_StandardCollector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::StandardCollectorTrait for PtrOfStandardCollector {
		#[inline] fn as_raw_mut_StandardCollector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::PredictCollectorConst for PtrOfStandardCollector {
		#[inline] fn as_raw_PredictCollector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::PredictCollector for PtrOfStandardCollector {
		#[inline] fn as_raw_mut_PredictCollector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfStandardCollector, core::Ptr<dyn crate::face::PredictCollector>,
		cv_PtrOfStandardCollector_to_PtrOfPredictCollector,
	}
	
	pub type VectorOfFacemarkAAM_Config = core::Vector<crate::face::FacemarkAAM_Config>;
	
	impl VectorOfFacemarkAAM_Config {
		pub fn as_raw_VectorOfFacemarkAAM_Config(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfFacemarkAAM_Config(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::face::FacemarkAAM_Config, *const c_void, *mut c_void,
		cv_VectorOfFacemarkAAM_Config_new, cv_VectorOfFacemarkAAM_Config_delete,
		cv_VectorOfFacemarkAAM_Config_len, cv_VectorOfFacemarkAAM_Config_is_empty,
		cv_VectorOfFacemarkAAM_Config_capacity, cv_VectorOfFacemarkAAM_Config_shrink_to_fit,
		cv_VectorOfFacemarkAAM_Config_reserve, cv_VectorOfFacemarkAAM_Config_remove,
		cv_VectorOfFacemarkAAM_Config_swap, cv_VectorOfFacemarkAAM_Config_clear,
		cv_VectorOfFacemarkAAM_Config_get, cv_VectorOfFacemarkAAM_Config_set,
		cv_VectorOfFacemarkAAM_Config_push, cv_VectorOfFacemarkAAM_Config_insert,
	}
	vector_non_copy_or_bool! { crate::face::FacemarkAAM_Config }
	
	unsafe impl Send for core::Vector<crate::face::FacemarkAAM_Config> {}
	
	pub type VectorOfFacemarkAAM_Model_Texture = core::Vector<crate::face::FacemarkAAM_Model_Texture>;
	
	impl VectorOfFacemarkAAM_Model_Texture {
		pub fn as_raw_VectorOfFacemarkAAM_Model_Texture(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfFacemarkAAM_Model_Texture(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::face::FacemarkAAM_Model_Texture, *const c_void, *mut c_void,
		cv_VectorOfFacemarkAAM_Model_Texture_new, cv_VectorOfFacemarkAAM_Model_Texture_delete,
		cv_VectorOfFacemarkAAM_Model_Texture_len, cv_VectorOfFacemarkAAM_Model_Texture_is_empty,
		cv_VectorOfFacemarkAAM_Model_Texture_capacity, cv_VectorOfFacemarkAAM_Model_Texture_shrink_to_fit,
		cv_VectorOfFacemarkAAM_Model_Texture_reserve, cv_VectorOfFacemarkAAM_Model_Texture_remove,
		cv_VectorOfFacemarkAAM_Model_Texture_swap, cv_VectorOfFacemarkAAM_Model_Texture_clear,
		cv_VectorOfFacemarkAAM_Model_Texture_get, cv_VectorOfFacemarkAAM_Model_Texture_set,
		cv_VectorOfFacemarkAAM_Model_Texture_push, cv_VectorOfFacemarkAAM_Model_Texture_insert,
	}
	vector_non_copy_or_bool! { crate::face::FacemarkAAM_Model_Texture }
	
	unsafe impl Send for core::Vector<crate::face::FacemarkAAM_Model_Texture> {}
	
}
#[cfg(ocvrs_has_module_face)]
pub use face_types::*;

#[cfg(ocvrs_has_module_features2d)]
mod features2d_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAKAZE = core::Ptr<dyn crate::features2d::AKAZE>;
	
	ptr_extern! { dyn crate::features2d::AKAZE,
		cv_PtrOfAKAZE_delete, cv_PtrOfAKAZE_get_inner_ptr, cv_PtrOfAKAZE_get_inner_ptr_mut
	}
	
	impl PtrOfAKAZE {
		#[inline] pub fn as_raw_PtrOfAKAZE(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAKAZE(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::AKAZEConst for PtrOfAKAZE {
		#[inline] fn as_raw_AKAZE(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::AKAZE for PtrOfAKAZE {
		#[inline] fn as_raw_mut_AKAZE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfAKAZE {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfAKAZE {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfAKAZE {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfAKAZE {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfAKAZE, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfAKAZE_to_PtrOfFeature2D,
	}
	
	pub type PtrOfAffineFeature = core::Ptr<dyn crate::features2d::AffineFeature>;
	
	ptr_extern! { dyn crate::features2d::AffineFeature,
		cv_PtrOfAffineFeature_delete, cv_PtrOfAffineFeature_get_inner_ptr, cv_PtrOfAffineFeature_get_inner_ptr_mut
	}
	
	impl PtrOfAffineFeature {
		#[inline] pub fn as_raw_PtrOfAffineFeature(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAffineFeature(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::AffineFeatureConst for PtrOfAffineFeature {
		#[inline] fn as_raw_AffineFeature(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::AffineFeature for PtrOfAffineFeature {
		#[inline] fn as_raw_mut_AffineFeature(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfAffineFeature {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfAffineFeature {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfAffineFeature {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfAffineFeature {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfAffineFeature, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfAffineFeature_to_PtrOfFeature2D,
	}
	
	pub type PtrOfAgastFeatureDetector = core::Ptr<dyn crate::features2d::AgastFeatureDetector>;
	
	ptr_extern! { dyn crate::features2d::AgastFeatureDetector,
		cv_PtrOfAgastFeatureDetector_delete, cv_PtrOfAgastFeatureDetector_get_inner_ptr, cv_PtrOfAgastFeatureDetector_get_inner_ptr_mut
	}
	
	impl PtrOfAgastFeatureDetector {
		#[inline] pub fn as_raw_PtrOfAgastFeatureDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAgastFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::AgastFeatureDetectorConst for PtrOfAgastFeatureDetector {
		#[inline] fn as_raw_AgastFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::AgastFeatureDetector for PtrOfAgastFeatureDetector {
		#[inline] fn as_raw_mut_AgastFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfAgastFeatureDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfAgastFeatureDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfAgastFeatureDetector {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfAgastFeatureDetector {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfAgastFeatureDetector, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfAgastFeatureDetector_to_PtrOfFeature2D,
	}
	
	pub type PtrOfBFMatcher = core::Ptr<crate::features2d::BFMatcher>;
	
	ptr_extern! { crate::features2d::BFMatcher,
		cv_PtrOfBFMatcher_delete, cv_PtrOfBFMatcher_get_inner_ptr, cv_PtrOfBFMatcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::features2d::BFMatcher, cv_PtrOfBFMatcher_new }
	
	impl PtrOfBFMatcher {
		#[inline] pub fn as_raw_PtrOfBFMatcher(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBFMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::BFMatcherTraitConst for PtrOfBFMatcher {
		#[inline] fn as_raw_BFMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::BFMatcherTrait for PtrOfBFMatcher {
		#[inline] fn as_raw_mut_BFMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfBFMatcher {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfBFMatcher {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::DescriptorMatcherConst for PtrOfBFMatcher {
		#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::DescriptorMatcher for PtrOfBFMatcher {
		#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBRISK = core::Ptr<crate::features2d::BRISK>;
	
	ptr_extern! { crate::features2d::BRISK,
		cv_PtrOfBRISK_delete, cv_PtrOfBRISK_get_inner_ptr, cv_PtrOfBRISK_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::features2d::BRISK, cv_PtrOfBRISK_new }
	
	impl PtrOfBRISK {
		#[inline] pub fn as_raw_PtrOfBRISK(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBRISK(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::BRISKTraitConst for PtrOfBRISK {
		#[inline] fn as_raw_BRISK(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::BRISKTrait for PtrOfBRISK {
		#[inline] fn as_raw_mut_BRISK(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfBRISK {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfBRISK {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfBRISK {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfBRISK {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfBRISK, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfBRISK_to_PtrOfFeature2D,
	}
	
	pub type PtrOfDescriptorMatcher = core::Ptr<dyn crate::features2d::DescriptorMatcher>;
	
	ptr_extern! { dyn crate::features2d::DescriptorMatcher,
		cv_PtrOfDescriptorMatcher_delete, cv_PtrOfDescriptorMatcher_get_inner_ptr, cv_PtrOfDescriptorMatcher_get_inner_ptr_mut
	}
	
	impl PtrOfDescriptorMatcher {
		#[inline] pub fn as_raw_PtrOfDescriptorMatcher(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::DescriptorMatcherConst for PtrOfDescriptorMatcher {
		#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::DescriptorMatcher for PtrOfDescriptorMatcher {
		#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfDescriptorMatcher {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfDescriptorMatcher {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFastFeatureDetector = core::Ptr<dyn crate::features2d::FastFeatureDetector>;
	
	ptr_extern! { dyn crate::features2d::FastFeatureDetector,
		cv_PtrOfFastFeatureDetector_delete, cv_PtrOfFastFeatureDetector_get_inner_ptr, cv_PtrOfFastFeatureDetector_get_inner_ptr_mut
	}
	
	impl PtrOfFastFeatureDetector {
		#[inline] pub fn as_raw_PtrOfFastFeatureDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFastFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::FastFeatureDetectorConst for PtrOfFastFeatureDetector {
		#[inline] fn as_raw_FastFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::FastFeatureDetector for PtrOfFastFeatureDetector {
		#[inline] fn as_raw_mut_FastFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfFastFeatureDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfFastFeatureDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfFastFeatureDetector {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfFastFeatureDetector {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfFastFeatureDetector, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfFastFeatureDetector_to_PtrOfFeature2D,
	}
	
	pub type PtrOfFeature2D = core::Ptr<crate::features2d::Feature2D>;
	
	ptr_extern! { crate::features2d::Feature2D,
		cv_PtrOfFeature2D_delete, cv_PtrOfFeature2D_get_inner_ptr, cv_PtrOfFeature2D_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::features2d::Feature2D, cv_PtrOfFeature2D_new }
	
	impl PtrOfFeature2D {
		#[inline] pub fn as_raw_PtrOfFeature2D(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFeature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfFeature2D {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfFeature2D {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfFeature2D {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfFeature2D {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFlannBasedMatcher = core::Ptr<crate::features2d::FlannBasedMatcher>;
	
	ptr_extern! { crate::features2d::FlannBasedMatcher,
		cv_PtrOfFlannBasedMatcher_delete, cv_PtrOfFlannBasedMatcher_get_inner_ptr, cv_PtrOfFlannBasedMatcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::features2d::FlannBasedMatcher, cv_PtrOfFlannBasedMatcher_new }
	
	impl PtrOfFlannBasedMatcher {
		#[inline] pub fn as_raw_PtrOfFlannBasedMatcher(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFlannBasedMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::FlannBasedMatcherTraitConst for PtrOfFlannBasedMatcher {
		#[inline] fn as_raw_FlannBasedMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::FlannBasedMatcherTrait for PtrOfFlannBasedMatcher {
		#[inline] fn as_raw_mut_FlannBasedMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfFlannBasedMatcher {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfFlannBasedMatcher {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::DescriptorMatcherConst for PtrOfFlannBasedMatcher {
		#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::DescriptorMatcher for PtrOfFlannBasedMatcher {
		#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGFTTDetector = core::Ptr<dyn crate::features2d::GFTTDetector>;
	
	ptr_extern! { dyn crate::features2d::GFTTDetector,
		cv_PtrOfGFTTDetector_delete, cv_PtrOfGFTTDetector_get_inner_ptr, cv_PtrOfGFTTDetector_get_inner_ptr_mut
	}
	
	impl PtrOfGFTTDetector {
		#[inline] pub fn as_raw_PtrOfGFTTDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGFTTDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::GFTTDetectorConst for PtrOfGFTTDetector {
		#[inline] fn as_raw_GFTTDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::GFTTDetector for PtrOfGFTTDetector {
		#[inline] fn as_raw_mut_GFTTDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfGFTTDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfGFTTDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfGFTTDetector {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfGFTTDetector {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfGFTTDetector, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfGFTTDetector_to_PtrOfFeature2D,
	}
	
	pub type PtrOfKAZE = core::Ptr<dyn crate::features2d::KAZE>;
	
	ptr_extern! { dyn crate::features2d::KAZE,
		cv_PtrOfKAZE_delete, cv_PtrOfKAZE_get_inner_ptr, cv_PtrOfKAZE_get_inner_ptr_mut
	}
	
	impl PtrOfKAZE {
		#[inline] pub fn as_raw_PtrOfKAZE(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKAZE(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::KAZEConst for PtrOfKAZE {
		#[inline] fn as_raw_KAZE(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::KAZE for PtrOfKAZE {
		#[inline] fn as_raw_mut_KAZE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfKAZE {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfKAZE {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfKAZE {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfKAZE {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfKAZE, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfKAZE_to_PtrOfFeature2D,
	}
	
	pub type PtrOfMSER = core::Ptr<dyn crate::features2d::MSER>;
	
	ptr_extern! { dyn crate::features2d::MSER,
		cv_PtrOfMSER_delete, cv_PtrOfMSER_get_inner_ptr, cv_PtrOfMSER_get_inner_ptr_mut
	}
	
	impl PtrOfMSER {
		#[inline] pub fn as_raw_PtrOfMSER(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMSER(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::MSERConst for PtrOfMSER {
		#[inline] fn as_raw_MSER(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::MSER for PtrOfMSER {
		#[inline] fn as_raw_mut_MSER(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfMSER {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfMSER {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfMSER {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfMSER {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMSER, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfMSER_to_PtrOfFeature2D,
	}
	
	pub type PtrOfORB = core::Ptr<dyn crate::features2d::ORB>;
	
	ptr_extern! { dyn crate::features2d::ORB,
		cv_PtrOfORB_delete, cv_PtrOfORB_get_inner_ptr, cv_PtrOfORB_get_inner_ptr_mut
	}
	
	impl PtrOfORB {
		#[inline] pub fn as_raw_PtrOfORB(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfORB(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::ORBConst for PtrOfORB {
		#[inline] fn as_raw_ORB(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::ORB for PtrOfORB {
		#[inline] fn as_raw_mut_ORB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfORB {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfORB {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfORB {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfORB {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfORB, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfORB_to_PtrOfFeature2D,
	}
	
	pub type PtrOfSIFT = core::Ptr<crate::features2d::SIFT>;
	
	ptr_extern! { crate::features2d::SIFT,
		cv_PtrOfSIFT_delete, cv_PtrOfSIFT_get_inner_ptr, cv_PtrOfSIFT_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::features2d::SIFT, cv_PtrOfSIFT_new }
	
	impl PtrOfSIFT {
		#[inline] pub fn as_raw_PtrOfSIFT(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSIFT(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::SIFTTraitConst for PtrOfSIFT {
		#[inline] fn as_raw_SIFT(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::SIFTTrait for PtrOfSIFT {
		#[inline] fn as_raw_mut_SIFT(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSIFT {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSIFT {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfSIFT {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfSIFT {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSIFT, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfSIFT_to_PtrOfFeature2D,
	}
	
	pub type PtrOfSimpleBlobDetector = core::Ptr<crate::features2d::SimpleBlobDetector>;
	
	ptr_extern! { crate::features2d::SimpleBlobDetector,
		cv_PtrOfSimpleBlobDetector_delete, cv_PtrOfSimpleBlobDetector_get_inner_ptr, cv_PtrOfSimpleBlobDetector_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::features2d::SimpleBlobDetector, cv_PtrOfSimpleBlobDetector_new }
	
	impl PtrOfSimpleBlobDetector {
		#[inline] pub fn as_raw_PtrOfSimpleBlobDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSimpleBlobDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::features2d::SimpleBlobDetectorTraitConst for PtrOfSimpleBlobDetector {
		#[inline] fn as_raw_SimpleBlobDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::SimpleBlobDetectorTrait for PtrOfSimpleBlobDetector {
		#[inline] fn as_raw_mut_SimpleBlobDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSimpleBlobDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSimpleBlobDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfSimpleBlobDetector {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfSimpleBlobDetector {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSimpleBlobDetector, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfSimpleBlobDetector_to_PtrOfFeature2D,
	}
	
}
#[cfg(ocvrs_has_module_features2d)]
pub use features2d_types::*;

#[cfg(ocvrs_has_module_flann)]
mod flann_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfIndexParams = core::Ptr<crate::flann::IndexParams>;
	
	ptr_extern! { crate::flann::IndexParams,
		cv_PtrOfIndexParams_delete, cv_PtrOfIndexParams_get_inner_ptr, cv_PtrOfIndexParams_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::flann::IndexParams, cv_PtrOfIndexParams_new }
	
	impl PtrOfIndexParams {
		#[inline] pub fn as_raw_PtrOfIndexParams(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::flann::IndexParamsTraitConst for PtrOfIndexParams {
		#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::flann::IndexParamsTrait for PtrOfIndexParams {
		#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSearchParams = core::Ptr<crate::flann::SearchParams>;
	
	ptr_extern! { crate::flann::SearchParams,
		cv_PtrOfSearchParams_delete, cv_PtrOfSearchParams_get_inner_ptr, cv_PtrOfSearchParams_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::flann::SearchParams, cv_PtrOfSearchParams_new }
	
	impl PtrOfSearchParams {
		#[inline] pub fn as_raw_PtrOfSearchParams(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSearchParams(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::flann::SearchParamsTraitConst for PtrOfSearchParams {
		#[inline] fn as_raw_SearchParams(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::flann::SearchParamsTrait for PtrOfSearchParams {
		#[inline] fn as_raw_mut_SearchParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::flann::IndexParamsTraitConst for PtrOfSearchParams {
		#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::flann::IndexParamsTrait for PtrOfSearchParams {
		#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfFlannIndexType = core::Vector<crate::flann::FlannIndexType>;
	
	impl VectorOfFlannIndexType {
		pub fn as_raw_VectorOfFlannIndexType(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfFlannIndexType(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::flann::FlannIndexType, *const c_void, *mut c_void,
		cv_VectorOfFlannIndexType_new, cv_VectorOfFlannIndexType_delete,
		cv_VectorOfFlannIndexType_len, cv_VectorOfFlannIndexType_is_empty,
		cv_VectorOfFlannIndexType_capacity, cv_VectorOfFlannIndexType_shrink_to_fit,
		cv_VectorOfFlannIndexType_reserve, cv_VectorOfFlannIndexType_remove,
		cv_VectorOfFlannIndexType_swap, cv_VectorOfFlannIndexType_clear,
		cv_VectorOfFlannIndexType_get, cv_VectorOfFlannIndexType_set,
		cv_VectorOfFlannIndexType_push, cv_VectorOfFlannIndexType_insert,
	}
	vector_copy_non_bool! { crate::flann::FlannIndexType, *const c_void, *mut c_void,
		cv_VectorOfFlannIndexType_data, cv_VectorOfFlannIndexType_data_mut, cv_VectorOfFlannIndexType_from_slice,
		cv_VectorOfFlannIndexType_clone,
	}
	
	unsafe impl Send for core::Vector<crate::flann::FlannIndexType> {}
	
}
#[cfg(ocvrs_has_module_flann)]
pub use flann_types::*;

#[cfg(ocvrs_has_module_freetype)]
mod freetype_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfFreeType2 = core::Ptr<dyn crate::freetype::FreeType2>;
	
	ptr_extern! { dyn crate::freetype::FreeType2,
		cv_PtrOfFreeType2_delete, cv_PtrOfFreeType2_get_inner_ptr, cv_PtrOfFreeType2_get_inner_ptr_mut
	}
	
	impl PtrOfFreeType2 {
		#[inline] pub fn as_raw_PtrOfFreeType2(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFreeType2(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::freetype::FreeType2Const for PtrOfFreeType2 {
		#[inline] fn as_raw_FreeType2(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::freetype::FreeType2 for PtrOfFreeType2 {
		#[inline] fn as_raw_mut_FreeType2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfFreeType2 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfFreeType2 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_freetype)]
pub use freetype_types::*;

#[cfg(ocvrs_has_module_hdf)]
mod hdf_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfHDF5 = core::Ptr<dyn crate::hdf::HDF5>;
	
	ptr_extern! { dyn crate::hdf::HDF5,
		cv_PtrOfHDF5_delete, cv_PtrOfHDF5_get_inner_ptr, cv_PtrOfHDF5_get_inner_ptr_mut
	}
	
	impl PtrOfHDF5 {
		#[inline] pub fn as_raw_PtrOfHDF5(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHDF5(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::hdf::HDF5Const for PtrOfHDF5 {
		#[inline] fn as_raw_HDF5(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::hdf::HDF5 for PtrOfHDF5 {
		#[inline] fn as_raw_mut_HDF5(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_hdf)]
pub use hdf_types::*;

#[cfg(ocvrs_has_module_hfs)]
mod hfs_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfHfsSegment = core::Ptr<dyn crate::hfs::HfsSegment>;
	
	ptr_extern! { dyn crate::hfs::HfsSegment,
		cv_PtrOfHfsSegment_delete, cv_PtrOfHfsSegment_get_inner_ptr, cv_PtrOfHfsSegment_get_inner_ptr_mut
	}
	
	impl PtrOfHfsSegment {
		#[inline] pub fn as_raw_PtrOfHfsSegment(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHfsSegment(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::hfs::HfsSegmentConst for PtrOfHfsSegment {
		#[inline] fn as_raw_HfsSegment(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::hfs::HfsSegment for PtrOfHfsSegment {
		#[inline] fn as_raw_mut_HfsSegment(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfHfsSegment {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfHfsSegment {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_hfs)]
pub use hfs_types::*;

#[cfg(ocvrs_has_module_img_hash)]
mod img_hash_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAverageHash = core::Ptr<crate::img_hash::AverageHash>;
	
	ptr_extern! { crate::img_hash::AverageHash,
		cv_PtrOfAverageHash_delete, cv_PtrOfAverageHash_get_inner_ptr, cv_PtrOfAverageHash_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::img_hash::AverageHash, cv_PtrOfAverageHash_new }
	
	impl PtrOfAverageHash {
		#[inline] pub fn as_raw_PtrOfAverageHash(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAverageHash(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::img_hash::AverageHashTraitConst for PtrOfAverageHash {
		#[inline] fn as_raw_AverageHash(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::AverageHashTrait for PtrOfAverageHash {
		#[inline] fn as_raw_mut_AverageHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfAverageHash {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfAverageHash {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTraitConst for PtrOfAverageHash {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for PtrOfAverageHash {
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBlockMeanHash = core::Ptr<crate::img_hash::BlockMeanHash>;
	
	ptr_extern! { crate::img_hash::BlockMeanHash,
		cv_PtrOfBlockMeanHash_delete, cv_PtrOfBlockMeanHash_get_inner_ptr, cv_PtrOfBlockMeanHash_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::img_hash::BlockMeanHash, cv_PtrOfBlockMeanHash_new }
	
	impl PtrOfBlockMeanHash {
		#[inline] pub fn as_raw_PtrOfBlockMeanHash(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBlockMeanHash(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::img_hash::BlockMeanHashTraitConst for PtrOfBlockMeanHash {
		#[inline] fn as_raw_BlockMeanHash(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::BlockMeanHashTrait for PtrOfBlockMeanHash {
		#[inline] fn as_raw_mut_BlockMeanHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfBlockMeanHash {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfBlockMeanHash {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTraitConst for PtrOfBlockMeanHash {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for PtrOfBlockMeanHash {
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfColorMomentHash = core::Ptr<crate::img_hash::ColorMomentHash>;
	
	ptr_extern! { crate::img_hash::ColorMomentHash,
		cv_PtrOfColorMomentHash_delete, cv_PtrOfColorMomentHash_get_inner_ptr, cv_PtrOfColorMomentHash_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::img_hash::ColorMomentHash, cv_PtrOfColorMomentHash_new }
	
	impl PtrOfColorMomentHash {
		#[inline] pub fn as_raw_PtrOfColorMomentHash(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfColorMomentHash(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::img_hash::ColorMomentHashTraitConst for PtrOfColorMomentHash {
		#[inline] fn as_raw_ColorMomentHash(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::ColorMomentHashTrait for PtrOfColorMomentHash {
		#[inline] fn as_raw_mut_ColorMomentHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfColorMomentHash {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfColorMomentHash {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTraitConst for PtrOfColorMomentHash {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for PtrOfColorMomentHash {
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMarrHildrethHash = core::Ptr<crate::img_hash::MarrHildrethHash>;
	
	ptr_extern! { crate::img_hash::MarrHildrethHash,
		cv_PtrOfMarrHildrethHash_delete, cv_PtrOfMarrHildrethHash_get_inner_ptr, cv_PtrOfMarrHildrethHash_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::img_hash::MarrHildrethHash, cv_PtrOfMarrHildrethHash_new }
	
	impl PtrOfMarrHildrethHash {
		#[inline] pub fn as_raw_PtrOfMarrHildrethHash(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMarrHildrethHash(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::img_hash::MarrHildrethHashTraitConst for PtrOfMarrHildrethHash {
		#[inline] fn as_raw_MarrHildrethHash(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::MarrHildrethHashTrait for PtrOfMarrHildrethHash {
		#[inline] fn as_raw_mut_MarrHildrethHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfMarrHildrethHash {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfMarrHildrethHash {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTraitConst for PtrOfMarrHildrethHash {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for PtrOfMarrHildrethHash {
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPHash = core::Ptr<crate::img_hash::PHash>;
	
	ptr_extern! { crate::img_hash::PHash,
		cv_PtrOfPHash_delete, cv_PtrOfPHash_get_inner_ptr, cv_PtrOfPHash_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::img_hash::PHash, cv_PtrOfPHash_new }
	
	impl PtrOfPHash {
		#[inline] pub fn as_raw_PtrOfPHash(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPHash(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::img_hash::PHashTraitConst for PtrOfPHash {
		#[inline] fn as_raw_PHash(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::PHashTrait for PtrOfPHash {
		#[inline] fn as_raw_mut_PHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfPHash {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfPHash {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTraitConst for PtrOfPHash {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for PtrOfPHash {
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRadialVarianceHash = core::Ptr<crate::img_hash::RadialVarianceHash>;
	
	ptr_extern! { crate::img_hash::RadialVarianceHash,
		cv_PtrOfRadialVarianceHash_delete, cv_PtrOfRadialVarianceHash_get_inner_ptr, cv_PtrOfRadialVarianceHash_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::img_hash::RadialVarianceHash, cv_PtrOfRadialVarianceHash_new }
	
	impl PtrOfRadialVarianceHash {
		#[inline] pub fn as_raw_PtrOfRadialVarianceHash(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRadialVarianceHash(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::img_hash::RadialVarianceHashTraitConst for PtrOfRadialVarianceHash {
		#[inline] fn as_raw_RadialVarianceHash(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::RadialVarianceHashTrait for PtrOfRadialVarianceHash {
		#[inline] fn as_raw_mut_RadialVarianceHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfRadialVarianceHash {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfRadialVarianceHash {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTraitConst for PtrOfRadialVarianceHash {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for PtrOfRadialVarianceHash {
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_img_hash)]
pub use img_hash_types::*;

#[cfg(ocvrs_has_module_imgproc)]
mod imgproc_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCLAHE = core::Ptr<dyn crate::imgproc::CLAHE>;
	
	ptr_extern! { dyn crate::imgproc::CLAHE,
		cv_PtrOfCLAHE_delete, cv_PtrOfCLAHE_get_inner_ptr, cv_PtrOfCLAHE_get_inner_ptr_mut
	}
	
	impl PtrOfCLAHE {
		#[inline] pub fn as_raw_PtrOfCLAHE(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCLAHE(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::imgproc::CLAHEConst for PtrOfCLAHE {
		#[inline] fn as_raw_CLAHE(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::imgproc::CLAHE for PtrOfCLAHE {
		#[inline] fn as_raw_mut_CLAHE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCLAHE {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCLAHE {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGeneralizedHoughBallard = core::Ptr<dyn crate::imgproc::GeneralizedHoughBallard>;
	
	ptr_extern! { dyn crate::imgproc::GeneralizedHoughBallard,
		cv_PtrOfGeneralizedHoughBallard_delete, cv_PtrOfGeneralizedHoughBallard_get_inner_ptr, cv_PtrOfGeneralizedHoughBallard_get_inner_ptr_mut
	}
	
	impl PtrOfGeneralizedHoughBallard {
		#[inline] pub fn as_raw_PtrOfGeneralizedHoughBallard(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGeneralizedHoughBallard(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::imgproc::GeneralizedHoughBallardConst for PtrOfGeneralizedHoughBallard {
		#[inline] fn as_raw_GeneralizedHoughBallard(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::imgproc::GeneralizedHoughBallard for PtrOfGeneralizedHoughBallard {
		#[inline] fn as_raw_mut_GeneralizedHoughBallard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfGeneralizedHoughBallard {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfGeneralizedHoughBallard {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::imgproc::GeneralizedHoughConst for PtrOfGeneralizedHoughBallard {
		#[inline] fn as_raw_GeneralizedHough(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::imgproc::GeneralizedHough for PtrOfGeneralizedHoughBallard {
		#[inline] fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGeneralizedHoughGuil = core::Ptr<dyn crate::imgproc::GeneralizedHoughGuil>;
	
	ptr_extern! { dyn crate::imgproc::GeneralizedHoughGuil,
		cv_PtrOfGeneralizedHoughGuil_delete, cv_PtrOfGeneralizedHoughGuil_get_inner_ptr, cv_PtrOfGeneralizedHoughGuil_get_inner_ptr_mut
	}
	
	impl PtrOfGeneralizedHoughGuil {
		#[inline] pub fn as_raw_PtrOfGeneralizedHoughGuil(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGeneralizedHoughGuil(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::imgproc::GeneralizedHoughGuilConst for PtrOfGeneralizedHoughGuil {
		#[inline] fn as_raw_GeneralizedHoughGuil(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::imgproc::GeneralizedHoughGuil for PtrOfGeneralizedHoughGuil {
		#[inline] fn as_raw_mut_GeneralizedHoughGuil(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfGeneralizedHoughGuil {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfGeneralizedHoughGuil {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::imgproc::GeneralizedHoughConst for PtrOfGeneralizedHoughGuil {
		#[inline] fn as_raw_GeneralizedHough(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::imgproc::GeneralizedHough for PtrOfGeneralizedHoughGuil {
		#[inline] fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLineSegmentDetector = core::Ptr<dyn crate::imgproc::LineSegmentDetector>;
	
	ptr_extern! { dyn crate::imgproc::LineSegmentDetector,
		cv_PtrOfLineSegmentDetector_delete, cv_PtrOfLineSegmentDetector_get_inner_ptr, cv_PtrOfLineSegmentDetector_get_inner_ptr_mut
	}
	
	impl PtrOfLineSegmentDetector {
		#[inline] pub fn as_raw_PtrOfLineSegmentDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLineSegmentDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::imgproc::LineSegmentDetectorConst for PtrOfLineSegmentDetector {
		#[inline] fn as_raw_LineSegmentDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::imgproc::LineSegmentDetector for PtrOfLineSegmentDetector {
		#[inline] fn as_raw_mut_LineSegmentDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfLineSegmentDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfLineSegmentDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_imgproc)]
pub use imgproc_types::*;

#[cfg(ocvrs_has_module_line_descriptor)]
mod line_descriptor_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBinaryDescriptor = core::Ptr<crate::line_descriptor::BinaryDescriptor>;
	
	ptr_extern! { crate::line_descriptor::BinaryDescriptor,
		cv_PtrOfBinaryDescriptor_delete, cv_PtrOfBinaryDescriptor_get_inner_ptr, cv_PtrOfBinaryDescriptor_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::line_descriptor::BinaryDescriptor, cv_PtrOfBinaryDescriptor_new }
	
	impl PtrOfBinaryDescriptor {
		#[inline] pub fn as_raw_PtrOfBinaryDescriptor(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBinaryDescriptor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::line_descriptor::BinaryDescriptorTraitConst for PtrOfBinaryDescriptor {
		#[inline] fn as_raw_BinaryDescriptor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::line_descriptor::BinaryDescriptorTrait for PtrOfBinaryDescriptor {
		#[inline] fn as_raw_mut_BinaryDescriptor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfBinaryDescriptor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfBinaryDescriptor {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBinaryDescriptorMatcher = core::Ptr<crate::line_descriptor::BinaryDescriptorMatcher>;
	
	ptr_extern! { crate::line_descriptor::BinaryDescriptorMatcher,
		cv_PtrOfBinaryDescriptorMatcher_delete, cv_PtrOfBinaryDescriptorMatcher_get_inner_ptr, cv_PtrOfBinaryDescriptorMatcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::line_descriptor::BinaryDescriptorMatcher, cv_PtrOfBinaryDescriptorMatcher_new }
	
	impl PtrOfBinaryDescriptorMatcher {
		#[inline] pub fn as_raw_PtrOfBinaryDescriptorMatcher(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBinaryDescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::line_descriptor::BinaryDescriptorMatcherTraitConst for PtrOfBinaryDescriptorMatcher {
		#[inline] fn as_raw_BinaryDescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::line_descriptor::BinaryDescriptorMatcherTrait for PtrOfBinaryDescriptorMatcher {
		#[inline] fn as_raw_mut_BinaryDescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfBinaryDescriptorMatcher {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfBinaryDescriptorMatcher {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLSDDetector = core::Ptr<crate::line_descriptor::LSDDetector>;
	
	ptr_extern! { crate::line_descriptor::LSDDetector,
		cv_PtrOfLSDDetector_delete, cv_PtrOfLSDDetector_get_inner_ptr, cv_PtrOfLSDDetector_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::line_descriptor::LSDDetector, cv_PtrOfLSDDetector_new }
	
	impl PtrOfLSDDetector {
		#[inline] pub fn as_raw_PtrOfLSDDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLSDDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::line_descriptor::LSDDetectorTraitConst for PtrOfLSDDetector {
		#[inline] fn as_raw_LSDDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::line_descriptor::LSDDetectorTrait for PtrOfLSDDetector {
		#[inline] fn as_raw_mut_LSDDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfLSDDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfLSDDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfKeyLine = core::Vector<crate::line_descriptor::KeyLine>;
	
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
		cv_VectorOfKeyLine_get, cv_VectorOfKeyLine_set,
		cv_VectorOfKeyLine_push, cv_VectorOfKeyLine_insert,
	}
	vector_copy_non_bool! { crate::line_descriptor::KeyLine, *const c_void, *mut c_void,
		cv_VectorOfKeyLine_data, cv_VectorOfKeyLine_data_mut, cv_VectorOfKeyLine_from_slice,
		cv_VectorOfKeyLine_clone,
	}
	
	unsafe impl Send for core::Vector<crate::line_descriptor::KeyLine> {}
	
	pub type VectorOfVectorOfKeyLine = core::Vector<core::Vector<crate::line_descriptor::KeyLine>>;
	
	impl VectorOfVectorOfKeyLine {
		pub fn as_raw_VectorOfVectorOfKeyLine(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfKeyLine(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<crate::line_descriptor::KeyLine>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfKeyLine_new, cv_VectorOfVectorOfKeyLine_delete,
		cv_VectorOfVectorOfKeyLine_len, cv_VectorOfVectorOfKeyLine_is_empty,
		cv_VectorOfVectorOfKeyLine_capacity, cv_VectorOfVectorOfKeyLine_shrink_to_fit,
		cv_VectorOfVectorOfKeyLine_reserve, cv_VectorOfVectorOfKeyLine_remove,
		cv_VectorOfVectorOfKeyLine_swap, cv_VectorOfVectorOfKeyLine_clear,
		cv_VectorOfVectorOfKeyLine_get, cv_VectorOfVectorOfKeyLine_set,
		cv_VectorOfVectorOfKeyLine_push, cv_VectorOfVectorOfKeyLine_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<crate::line_descriptor::KeyLine> }
	
	unsafe impl Send for core::Vector<core::Vector<crate::line_descriptor::KeyLine>> {}
	
}
#[cfg(ocvrs_has_module_line_descriptor)]
pub use line_descriptor_types::*;

#[cfg(ocvrs_has_module_mcc)]
mod mcc_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfMCC_CChecker = core::Ptr<dyn crate::mcc::MCC_CChecker>;
	
	ptr_extern! { dyn crate::mcc::MCC_CChecker,
		cv_PtrOfMCC_CChecker_delete, cv_PtrOfMCC_CChecker_get_inner_ptr, cv_PtrOfMCC_CChecker_get_inner_ptr_mut
	}
	
	impl PtrOfMCC_CChecker {
		#[inline] pub fn as_raw_PtrOfMCC_CChecker(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMCC_CChecker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::mcc::MCC_CCheckerConst for PtrOfMCC_CChecker {
		#[inline] fn as_raw_MCC_CChecker(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::mcc::MCC_CChecker for PtrOfMCC_CChecker {
		#[inline] fn as_raw_mut_MCC_CChecker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMCC_CCheckerDetector = core::Ptr<dyn crate::mcc::MCC_CCheckerDetector>;
	
	ptr_extern! { dyn crate::mcc::MCC_CCheckerDetector,
		cv_PtrOfMCC_CCheckerDetector_delete, cv_PtrOfMCC_CCheckerDetector_get_inner_ptr, cv_PtrOfMCC_CCheckerDetector_get_inner_ptr_mut
	}
	
	impl PtrOfMCC_CCheckerDetector {
		#[inline] pub fn as_raw_PtrOfMCC_CCheckerDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMCC_CCheckerDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::mcc::MCC_CCheckerDetectorConst for PtrOfMCC_CCheckerDetector {
		#[inline] fn as_raw_MCC_CCheckerDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::mcc::MCC_CCheckerDetector for PtrOfMCC_CCheckerDetector {
		#[inline] fn as_raw_mut_MCC_CCheckerDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfMCC_CCheckerDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfMCC_CCheckerDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMCC_CCheckerDraw = core::Ptr<dyn crate::mcc::MCC_CCheckerDraw>;
	
	ptr_extern! { dyn crate::mcc::MCC_CCheckerDraw,
		cv_PtrOfMCC_CCheckerDraw_delete, cv_PtrOfMCC_CCheckerDraw_get_inner_ptr, cv_PtrOfMCC_CCheckerDraw_get_inner_ptr_mut
	}
	
	impl PtrOfMCC_CCheckerDraw {
		#[inline] pub fn as_raw_PtrOfMCC_CCheckerDraw(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMCC_CCheckerDraw(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::mcc::MCC_CCheckerDrawConst for PtrOfMCC_CCheckerDraw {
		#[inline] fn as_raw_MCC_CCheckerDraw(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::mcc::MCC_CCheckerDraw for PtrOfMCC_CCheckerDraw {
		#[inline] fn as_raw_mut_MCC_CCheckerDraw(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMCC_DetectorParameters = core::Ptr<crate::mcc::MCC_DetectorParameters>;
	
	ptr_extern! { crate::mcc::MCC_DetectorParameters,
		cv_PtrOfMCC_DetectorParameters_delete, cv_PtrOfMCC_DetectorParameters_get_inner_ptr, cv_PtrOfMCC_DetectorParameters_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::mcc::MCC_DetectorParameters, cv_PtrOfMCC_DetectorParameters_new }
	
	impl PtrOfMCC_DetectorParameters {
		#[inline] pub fn as_raw_PtrOfMCC_DetectorParameters(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMCC_DetectorParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::mcc::MCC_DetectorParametersTraitConst for PtrOfMCC_DetectorParameters {
		#[inline] fn as_raw_MCC_DetectorParameters(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::mcc::MCC_DetectorParametersTrait for PtrOfMCC_DetectorParameters {
		#[inline] fn as_raw_mut_MCC_DetectorParameters(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfPtrOfMCC_CChecker = core::Vector<core::Ptr<dyn crate::mcc::MCC_CChecker>>;
	
	impl VectorOfPtrOfMCC_CChecker {
		pub fn as_raw_VectorOfPtrOfMCC_CChecker(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfMCC_CChecker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Ptr<dyn crate::mcc::MCC_CChecker>, *const c_void, *mut c_void,
		cv_VectorOfPtrOfMCC_CChecker_new, cv_VectorOfPtrOfMCC_CChecker_delete,
		cv_VectorOfPtrOfMCC_CChecker_len, cv_VectorOfPtrOfMCC_CChecker_is_empty,
		cv_VectorOfPtrOfMCC_CChecker_capacity, cv_VectorOfPtrOfMCC_CChecker_shrink_to_fit,
		cv_VectorOfPtrOfMCC_CChecker_reserve, cv_VectorOfPtrOfMCC_CChecker_remove,
		cv_VectorOfPtrOfMCC_CChecker_swap, cv_VectorOfPtrOfMCC_CChecker_clear,
		cv_VectorOfPtrOfMCC_CChecker_get, cv_VectorOfPtrOfMCC_CChecker_set,
		cv_VectorOfPtrOfMCC_CChecker_push, cv_VectorOfPtrOfMCC_CChecker_insert,
	}
	vector_non_copy_or_bool! { core::Ptr<dyn crate::mcc::MCC_CChecker> }
	
	unsafe impl Send for core::Vector<core::Ptr<dyn crate::mcc::MCC_CChecker>> {}
	
}
#[cfg(ocvrs_has_module_mcc)]
pub use mcc_types::*;

#[cfg(ocvrs_has_module_ml)]
mod ml_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfANN_MLP = core::Ptr<dyn crate::ml::ANN_MLP>;
	
	ptr_extern! { dyn crate::ml::ANN_MLP,
		cv_PtrOfANN_MLP_delete, cv_PtrOfANN_MLP_get_inner_ptr, cv_PtrOfANN_MLP_get_inner_ptr_mut
	}
	
	impl PtrOfANN_MLP {
		#[inline] pub fn as_raw_PtrOfANN_MLP(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfANN_MLP(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ml::ANN_MLPConst for PtrOfANN_MLP {
		#[inline] fn as_raw_ANN_MLP(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::ANN_MLP for PtrOfANN_MLP {
		#[inline] fn as_raw_mut_ANN_MLP(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfANN_MLP {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfANN_MLP {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelConst for PtrOfANN_MLP {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModel for PtrOfANN_MLP {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBoost = core::Ptr<dyn crate::ml::Boost>;
	
	ptr_extern! { dyn crate::ml::Boost,
		cv_PtrOfBoost_delete, cv_PtrOfBoost_get_inner_ptr, cv_PtrOfBoost_get_inner_ptr_mut
	}
	
	impl PtrOfBoost {
		#[inline] pub fn as_raw_PtrOfBoost(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBoost(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ml::BoostConst for PtrOfBoost {
		#[inline] fn as_raw_Boost(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::Boost for PtrOfBoost {
		#[inline] fn as_raw_mut_Boost(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfBoost {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfBoost {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::DTreesConst for PtrOfBoost {
		#[inline] fn as_raw_DTrees(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::DTrees for PtrOfBoost {
		#[inline] fn as_raw_mut_DTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelConst for PtrOfBoost {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModel for PtrOfBoost {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDTrees = core::Ptr<dyn crate::ml::DTrees>;
	
	ptr_extern! { dyn crate::ml::DTrees,
		cv_PtrOfDTrees_delete, cv_PtrOfDTrees_get_inner_ptr, cv_PtrOfDTrees_get_inner_ptr_mut
	}
	
	impl PtrOfDTrees {
		#[inline] pub fn as_raw_PtrOfDTrees(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDTrees(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ml::DTreesConst for PtrOfDTrees {
		#[inline] fn as_raw_DTrees(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::DTrees for PtrOfDTrees {
		#[inline] fn as_raw_mut_DTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfDTrees {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfDTrees {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelConst for PtrOfDTrees {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModel for PtrOfDTrees {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEM = core::Ptr<dyn crate::ml::EM>;
	
	ptr_extern! { dyn crate::ml::EM,
		cv_PtrOfEM_delete, cv_PtrOfEM_get_inner_ptr, cv_PtrOfEM_get_inner_ptr_mut
	}
	
	impl PtrOfEM {
		#[inline] pub fn as_raw_PtrOfEM(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEM(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ml::EMConst for PtrOfEM {
		#[inline] fn as_raw_EM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::EM for PtrOfEM {
		#[inline] fn as_raw_mut_EM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfEM {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfEM {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelConst for PtrOfEM {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModel for PtrOfEM {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfKNearest = core::Ptr<dyn crate::ml::KNearest>;
	
	ptr_extern! { dyn crate::ml::KNearest,
		cv_PtrOfKNearest_delete, cv_PtrOfKNearest_get_inner_ptr, cv_PtrOfKNearest_get_inner_ptr_mut
	}
	
	impl PtrOfKNearest {
		#[inline] pub fn as_raw_PtrOfKNearest(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKNearest(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ml::KNearestConst for PtrOfKNearest {
		#[inline] fn as_raw_KNearest(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::KNearest for PtrOfKNearest {
		#[inline] fn as_raw_mut_KNearest(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfKNearest {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfKNearest {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelConst for PtrOfKNearest {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModel for PtrOfKNearest {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLogisticRegression = core::Ptr<dyn crate::ml::LogisticRegression>;
	
	ptr_extern! { dyn crate::ml::LogisticRegression,
		cv_PtrOfLogisticRegression_delete, cv_PtrOfLogisticRegression_get_inner_ptr, cv_PtrOfLogisticRegression_get_inner_ptr_mut
	}
	
	impl PtrOfLogisticRegression {
		#[inline] pub fn as_raw_PtrOfLogisticRegression(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLogisticRegression(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ml::LogisticRegressionConst for PtrOfLogisticRegression {
		#[inline] fn as_raw_LogisticRegression(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::LogisticRegression for PtrOfLogisticRegression {
		#[inline] fn as_raw_mut_LogisticRegression(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfLogisticRegression {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfLogisticRegression {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelConst for PtrOfLogisticRegression {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModel for PtrOfLogisticRegression {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfNormalBayesClassifier = core::Ptr<dyn crate::ml::NormalBayesClassifier>;
	
	ptr_extern! { dyn crate::ml::NormalBayesClassifier,
		cv_PtrOfNormalBayesClassifier_delete, cv_PtrOfNormalBayesClassifier_get_inner_ptr, cv_PtrOfNormalBayesClassifier_get_inner_ptr_mut
	}
	
	impl PtrOfNormalBayesClassifier {
		#[inline] pub fn as_raw_PtrOfNormalBayesClassifier(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNormalBayesClassifier(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ml::NormalBayesClassifierConst for PtrOfNormalBayesClassifier {
		#[inline] fn as_raw_NormalBayesClassifier(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::NormalBayesClassifier for PtrOfNormalBayesClassifier {
		#[inline] fn as_raw_mut_NormalBayesClassifier(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfNormalBayesClassifier {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfNormalBayesClassifier {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelConst for PtrOfNormalBayesClassifier {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModel for PtrOfNormalBayesClassifier {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfParamGrid = core::Ptr<crate::ml::ParamGrid>;
	
	ptr_extern! { crate::ml::ParamGrid,
		cv_PtrOfParamGrid_delete, cv_PtrOfParamGrid_get_inner_ptr, cv_PtrOfParamGrid_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::ml::ParamGrid, cv_PtrOfParamGrid_new }
	
	impl PtrOfParamGrid {
		#[inline] pub fn as_raw_PtrOfParamGrid(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfParamGrid(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ml::ParamGridTraitConst for PtrOfParamGrid {
		#[inline] fn as_raw_ParamGrid(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::ParamGridTrait for PtrOfParamGrid {
		#[inline] fn as_raw_mut_ParamGrid(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRTrees = core::Ptr<dyn crate::ml::RTrees>;
	
	ptr_extern! { dyn crate::ml::RTrees,
		cv_PtrOfRTrees_delete, cv_PtrOfRTrees_get_inner_ptr, cv_PtrOfRTrees_get_inner_ptr_mut
	}
	
	impl PtrOfRTrees {
		#[inline] pub fn as_raw_PtrOfRTrees(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRTrees(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ml::RTreesConst for PtrOfRTrees {
		#[inline] fn as_raw_RTrees(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::RTrees for PtrOfRTrees {
		#[inline] fn as_raw_mut_RTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfRTrees {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfRTrees {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::DTreesConst for PtrOfRTrees {
		#[inline] fn as_raw_DTrees(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::DTrees for PtrOfRTrees {
		#[inline] fn as_raw_mut_DTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelConst for PtrOfRTrees {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModel for PtrOfRTrees {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSVM = core::Ptr<dyn crate::ml::SVM>;
	
	ptr_extern! { dyn crate::ml::SVM,
		cv_PtrOfSVM_delete, cv_PtrOfSVM_get_inner_ptr, cv_PtrOfSVM_get_inner_ptr_mut
	}
	
	impl PtrOfSVM {
		#[inline] pub fn as_raw_PtrOfSVM(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSVM(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ml::SVMConst for PtrOfSVM {
		#[inline] fn as_raw_SVM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::SVM for PtrOfSVM {
		#[inline] fn as_raw_mut_SVM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSVM {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSVM {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelConst for PtrOfSVM {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModel for PtrOfSVM {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSVMSGD = core::Ptr<dyn crate::ml::SVMSGD>;
	
	ptr_extern! { dyn crate::ml::SVMSGD,
		cv_PtrOfSVMSGD_delete, cv_PtrOfSVMSGD_get_inner_ptr, cv_PtrOfSVMSGD_get_inner_ptr_mut
	}
	
	impl PtrOfSVMSGD {
		#[inline] pub fn as_raw_PtrOfSVMSGD(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSVMSGD(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ml::SVMSGDConst for PtrOfSVMSGD {
		#[inline] fn as_raw_SVMSGD(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::SVMSGD for PtrOfSVMSGD {
		#[inline] fn as_raw_mut_SVMSGD(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSVMSGD {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSVMSGD {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelConst for PtrOfSVMSGD {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModel for PtrOfSVMSGD {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSVM_Kernel = core::Ptr<dyn crate::ml::SVM_Kernel>;
	
	ptr_extern! { dyn crate::ml::SVM_Kernel,
		cv_PtrOfSVM_Kernel_delete, cv_PtrOfSVM_Kernel_get_inner_ptr, cv_PtrOfSVM_Kernel_get_inner_ptr_mut
	}
	
	impl PtrOfSVM_Kernel {
		#[inline] pub fn as_raw_PtrOfSVM_Kernel(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSVM_Kernel(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ml::SVM_KernelConst for PtrOfSVM_Kernel {
		#[inline] fn as_raw_SVM_Kernel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::SVM_Kernel for PtrOfSVM_Kernel {
		#[inline] fn as_raw_mut_SVM_Kernel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSVM_Kernel {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSVM_Kernel {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrainData = core::Ptr<dyn crate::ml::TrainData>;
	
	ptr_extern! { dyn crate::ml::TrainData,
		cv_PtrOfTrainData_delete, cv_PtrOfTrainData_get_inner_ptr, cv_PtrOfTrainData_get_inner_ptr_mut
	}
	
	impl PtrOfTrainData {
		#[inline] pub fn as_raw_PtrOfTrainData(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrainData(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ml::TrainDataConst for PtrOfTrainData {
		#[inline] fn as_raw_TrainData(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::TrainData for PtrOfTrainData {
		#[inline] fn as_raw_mut_TrainData(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfDTrees_Node = core::Vector<crate::ml::DTrees_Node>;
	
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
		cv_VectorOfDTrees_Node_get, cv_VectorOfDTrees_Node_set,
		cv_VectorOfDTrees_Node_push, cv_VectorOfDTrees_Node_insert,
	}
	vector_non_copy_or_bool! { crate::ml::DTrees_Node }
	
	unsafe impl Send for core::Vector<crate::ml::DTrees_Node> {}
	
	pub type VectorOfDTrees_Split = core::Vector<crate::ml::DTrees_Split>;
	
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
		cv_VectorOfDTrees_Split_get, cv_VectorOfDTrees_Split_set,
		cv_VectorOfDTrees_Split_push, cv_VectorOfDTrees_Split_insert,
	}
	vector_non_copy_or_bool! { crate::ml::DTrees_Split }
	
	unsafe impl Send for core::Vector<crate::ml::DTrees_Split> {}
	
}
#[cfg(ocvrs_has_module_ml)]
pub use ml_types::*;

#[cfg(ocvrs_has_module_objdetect)]
mod objdetect_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBaseCascadeClassifier = core::Ptr<dyn crate::objdetect::BaseCascadeClassifier>;
	
	ptr_extern! { dyn crate::objdetect::BaseCascadeClassifier,
		cv_PtrOfBaseCascadeClassifier_delete, cv_PtrOfBaseCascadeClassifier_get_inner_ptr, cv_PtrOfBaseCascadeClassifier_get_inner_ptr_mut
	}
	
	impl PtrOfBaseCascadeClassifier {
		#[inline] pub fn as_raw_PtrOfBaseCascadeClassifier(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBaseCascadeClassifier(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::objdetect::BaseCascadeClassifierConst for PtrOfBaseCascadeClassifier {
		#[inline] fn as_raw_BaseCascadeClassifier(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::BaseCascadeClassifier for PtrOfBaseCascadeClassifier {
		#[inline] fn as_raw_mut_BaseCascadeClassifier(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfBaseCascadeClassifier {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfBaseCascadeClassifier {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBaseCascadeClassifier_MaskGenerator = core::Ptr<dyn crate::objdetect::BaseCascadeClassifier_MaskGenerator>;
	
	ptr_extern! { dyn crate::objdetect::BaseCascadeClassifier_MaskGenerator,
		cv_PtrOfBaseCascadeClassifier_MaskGenerator_delete, cv_PtrOfBaseCascadeClassifier_MaskGenerator_get_inner_ptr, cv_PtrOfBaseCascadeClassifier_MaskGenerator_get_inner_ptr_mut
	}
	
	impl PtrOfBaseCascadeClassifier_MaskGenerator {
		#[inline] pub fn as_raw_PtrOfBaseCascadeClassifier_MaskGenerator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBaseCascadeClassifier_MaskGenerator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::objdetect::BaseCascadeClassifier_MaskGeneratorConst for PtrOfBaseCascadeClassifier_MaskGenerator {
		#[inline] fn as_raw_BaseCascadeClassifier_MaskGenerator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::BaseCascadeClassifier_MaskGenerator for PtrOfBaseCascadeClassifier_MaskGenerator {
		#[inline] fn as_raw_mut_BaseCascadeClassifier_MaskGenerator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetectionBasedTracker_IDetector = core::Ptr<dyn crate::objdetect::DetectionBasedTracker_IDetector>;
	
	ptr_extern! { dyn crate::objdetect::DetectionBasedTracker_IDetector,
		cv_PtrOfDetectionBasedTracker_IDetector_delete, cv_PtrOfDetectionBasedTracker_IDetector_get_inner_ptr, cv_PtrOfDetectionBasedTracker_IDetector_get_inner_ptr_mut
	}
	
	impl PtrOfDetectionBasedTracker_IDetector {
		#[inline] pub fn as_raw_PtrOfDetectionBasedTracker_IDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetectionBasedTracker_IDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::objdetect::DetectionBasedTracker_IDetectorConst for PtrOfDetectionBasedTracker_IDetector {
		#[inline] fn as_raw_DetectionBasedTracker_IDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::DetectionBasedTracker_IDetector for PtrOfDetectionBasedTracker_IDetector {
		#[inline] fn as_raw_mut_DetectionBasedTracker_IDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFaceDetectorYN = core::Ptr<dyn crate::objdetect::FaceDetectorYN>;
	
	ptr_extern! { dyn crate::objdetect::FaceDetectorYN,
		cv_PtrOfFaceDetectorYN_delete, cv_PtrOfFaceDetectorYN_get_inner_ptr, cv_PtrOfFaceDetectorYN_get_inner_ptr_mut
	}
	
	impl PtrOfFaceDetectorYN {
		#[inline] pub fn as_raw_PtrOfFaceDetectorYN(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFaceDetectorYN(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::objdetect::FaceDetectorYNConst for PtrOfFaceDetectorYN {
		#[inline] fn as_raw_FaceDetectorYN(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::FaceDetectorYN for PtrOfFaceDetectorYN {
		#[inline] fn as_raw_mut_FaceDetectorYN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFaceRecognizerSF = core::Ptr<dyn crate::objdetect::FaceRecognizerSF>;
	
	ptr_extern! { dyn crate::objdetect::FaceRecognizerSF,
		cv_PtrOfFaceRecognizerSF_delete, cv_PtrOfFaceRecognizerSF_get_inner_ptr, cv_PtrOfFaceRecognizerSF_get_inner_ptr_mut
	}
	
	impl PtrOfFaceRecognizerSF {
		#[inline] pub fn as_raw_PtrOfFaceRecognizerSF(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFaceRecognizerSF(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::objdetect::FaceRecognizerSFConst for PtrOfFaceRecognizerSF {
		#[inline] fn as_raw_FaceRecognizerSF(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::FaceRecognizerSF for PtrOfFaceRecognizerSF {
		#[inline] fn as_raw_mut_FaceRecognizerSF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfQRCodeEncoder = core::Ptr<dyn crate::objdetect::QRCodeEncoder>;
	
	ptr_extern! { dyn crate::objdetect::QRCodeEncoder,
		cv_PtrOfQRCodeEncoder_delete, cv_PtrOfQRCodeEncoder_get_inner_ptr, cv_PtrOfQRCodeEncoder_get_inner_ptr_mut
	}
	
	impl PtrOfQRCodeEncoder {
		#[inline] pub fn as_raw_PtrOfQRCodeEncoder(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfQRCodeEncoder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::objdetect::QRCodeEncoderConst for PtrOfQRCodeEncoder {
		#[inline] fn as_raw_QRCodeEncoder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::QRCodeEncoder for PtrOfQRCodeEncoder {
		#[inline] fn as_raw_mut_QRCodeEncoder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfDetectionBasedTracker_ExtObject = core::Vector<crate::objdetect::DetectionBasedTracker_ExtObject>;
	
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
		cv_VectorOfDetectionBasedTracker_ExtObject_get, cv_VectorOfDetectionBasedTracker_ExtObject_set,
		cv_VectorOfDetectionBasedTracker_ExtObject_push, cv_VectorOfDetectionBasedTracker_ExtObject_insert,
	}
	vector_non_copy_or_bool! { crate::objdetect::DetectionBasedTracker_ExtObject }
	
	unsafe impl Send for core::Vector<crate::objdetect::DetectionBasedTracker_ExtObject> {}
	
	pub type VectorOfDetectionROI = core::Vector<crate::objdetect::DetectionROI>;
	
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
		cv_VectorOfDetectionROI_get, cv_VectorOfDetectionROI_set,
		cv_VectorOfDetectionROI_push, cv_VectorOfDetectionROI_insert,
	}
	vector_non_copy_or_bool! { crate::objdetect::DetectionROI }
	
	unsafe impl Send for core::Vector<crate::objdetect::DetectionROI> {}
	
}
#[cfg(ocvrs_has_module_objdetect)]
pub use objdetect_types::*;

#[cfg(ocvrs_has_module_optflow)]
mod optflow_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfDenseRLOFOpticalFlow = core::Ptr<dyn crate::optflow::DenseRLOFOpticalFlow>;
	
	ptr_extern! { dyn crate::optflow::DenseRLOFOpticalFlow,
		cv_PtrOfDenseRLOFOpticalFlow_delete, cv_PtrOfDenseRLOFOpticalFlow_get_inner_ptr, cv_PtrOfDenseRLOFOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfDenseRLOFOpticalFlow {
		#[inline] pub fn as_raw_PtrOfDenseRLOFOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDenseRLOFOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::optflow::DenseRLOFOpticalFlowConst for PtrOfDenseRLOFOpticalFlow {
		#[inline] fn as_raw_DenseRLOFOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::optflow::DenseRLOFOpticalFlow for PtrOfDenseRLOFOpticalFlow {
		#[inline] fn as_raw_mut_DenseRLOFOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfDenseRLOFOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfDenseRLOFOpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DenseOpticalFlowConst for PtrOfDenseRLOFOpticalFlow {
		#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::DenseOpticalFlow for PtrOfDenseRLOFOpticalFlow {
		#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDualTVL1OpticalFlow = core::Ptr<dyn crate::optflow::DualTVL1OpticalFlow>;
	
	ptr_extern! { dyn crate::optflow::DualTVL1OpticalFlow,
		cv_PtrOfDualTVL1OpticalFlow_delete, cv_PtrOfDualTVL1OpticalFlow_get_inner_ptr, cv_PtrOfDualTVL1OpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfDualTVL1OpticalFlow {
		#[inline] pub fn as_raw_PtrOfDualTVL1OpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDualTVL1OpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::optflow::DualTVL1OpticalFlowConst for PtrOfDualTVL1OpticalFlow {
		#[inline] fn as_raw_DualTVL1OpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::optflow::DualTVL1OpticalFlow for PtrOfDualTVL1OpticalFlow {
		#[inline] fn as_raw_mut_DualTVL1OpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfDualTVL1OpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfDualTVL1OpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DenseOpticalFlowConst for PtrOfDualTVL1OpticalFlow {
		#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::DenseOpticalFlow for PtrOfDualTVL1OpticalFlow {
		#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGPCTrainingSamples = core::Ptr<crate::optflow::GPCTrainingSamples>;
	
	ptr_extern! { crate::optflow::GPCTrainingSamples,
		cv_PtrOfGPCTrainingSamples_delete, cv_PtrOfGPCTrainingSamples_get_inner_ptr, cv_PtrOfGPCTrainingSamples_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::optflow::GPCTrainingSamples, cv_PtrOfGPCTrainingSamples_new }
	
	impl PtrOfGPCTrainingSamples {
		#[inline] pub fn as_raw_PtrOfGPCTrainingSamples(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGPCTrainingSamples(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::optflow::GPCTrainingSamplesTraitConst for PtrOfGPCTrainingSamples {
		#[inline] fn as_raw_GPCTrainingSamples(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::optflow::GPCTrainingSamplesTrait for PtrOfGPCTrainingSamples {
		#[inline] fn as_raw_mut_GPCTrainingSamples(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGPCTree = core::Ptr<crate::optflow::GPCTree>;
	
	ptr_extern! { crate::optflow::GPCTree,
		cv_PtrOfGPCTree_delete, cv_PtrOfGPCTree_get_inner_ptr, cv_PtrOfGPCTree_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::optflow::GPCTree, cv_PtrOfGPCTree_new }
	
	impl PtrOfGPCTree {
		#[inline] pub fn as_raw_PtrOfGPCTree(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGPCTree(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::optflow::GPCTreeTraitConst for PtrOfGPCTree {
		#[inline] fn as_raw_GPCTree(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::optflow::GPCTreeTrait for PtrOfGPCTree {
		#[inline] fn as_raw_mut_GPCTree(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfGPCTree {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfGPCTree {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPCAPrior = core::Ptr<crate::optflow::PCAPrior>;
	
	ptr_extern! { crate::optflow::PCAPrior,
		cv_PtrOfPCAPrior_delete, cv_PtrOfPCAPrior_get_inner_ptr, cv_PtrOfPCAPrior_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::optflow::PCAPrior, cv_PtrOfPCAPrior_new }
	
	impl PtrOfPCAPrior {
		#[inline] pub fn as_raw_PtrOfPCAPrior(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPCAPrior(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::optflow::PCAPriorTraitConst for PtrOfPCAPrior {
		#[inline] fn as_raw_PCAPrior(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::optflow::PCAPriorTrait for PtrOfPCAPrior {
		#[inline] fn as_raw_mut_PCAPrior(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRLOFOpticalFlowParameter = core::Ptr<crate::optflow::RLOFOpticalFlowParameter>;
	
	ptr_extern! { crate::optflow::RLOFOpticalFlowParameter,
		cv_PtrOfRLOFOpticalFlowParameter_delete, cv_PtrOfRLOFOpticalFlowParameter_get_inner_ptr, cv_PtrOfRLOFOpticalFlowParameter_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::optflow::RLOFOpticalFlowParameter, cv_PtrOfRLOFOpticalFlowParameter_new }
	
	impl PtrOfRLOFOpticalFlowParameter {
		#[inline] pub fn as_raw_PtrOfRLOFOpticalFlowParameter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRLOFOpticalFlowParameter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::optflow::RLOFOpticalFlowParameterTraitConst for PtrOfRLOFOpticalFlowParameter {
		#[inline] fn as_raw_RLOFOpticalFlowParameter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::optflow::RLOFOpticalFlowParameterTrait for PtrOfRLOFOpticalFlowParameter {
		#[inline] fn as_raw_mut_RLOFOpticalFlowParameter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSparseRLOFOpticalFlow = core::Ptr<dyn crate::optflow::SparseRLOFOpticalFlow>;
	
	ptr_extern! { dyn crate::optflow::SparseRLOFOpticalFlow,
		cv_PtrOfSparseRLOFOpticalFlow_delete, cv_PtrOfSparseRLOFOpticalFlow_get_inner_ptr, cv_PtrOfSparseRLOFOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfSparseRLOFOpticalFlow {
		#[inline] pub fn as_raw_PtrOfSparseRLOFOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSparseRLOFOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::optflow::SparseRLOFOpticalFlowConst for PtrOfSparseRLOFOpticalFlow {
		#[inline] fn as_raw_SparseRLOFOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::optflow::SparseRLOFOpticalFlow for PtrOfSparseRLOFOpticalFlow {
		#[inline] fn as_raw_mut_SparseRLOFOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSparseRLOFOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSparseRLOFOpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::SparseOpticalFlowConst for PtrOfSparseRLOFOpticalFlow {
		#[inline] fn as_raw_SparseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::SparseOpticalFlow for PtrOfSparseRLOFOpticalFlow {
		#[inline] fn as_raw_mut_SparseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfGPCPatchDescriptor = core::Vector<crate::optflow::GPCPatchDescriptor>;
	
	impl VectorOfGPCPatchDescriptor {
		pub fn as_raw_VectorOfGPCPatchDescriptor(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfGPCPatchDescriptor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::optflow::GPCPatchDescriptor, *const c_void, *mut c_void,
		cv_VectorOfGPCPatchDescriptor_new, cv_VectorOfGPCPatchDescriptor_delete,
		cv_VectorOfGPCPatchDescriptor_len, cv_VectorOfGPCPatchDescriptor_is_empty,
		cv_VectorOfGPCPatchDescriptor_capacity, cv_VectorOfGPCPatchDescriptor_shrink_to_fit,
		cv_VectorOfGPCPatchDescriptor_reserve, cv_VectorOfGPCPatchDescriptor_remove,
		cv_VectorOfGPCPatchDescriptor_swap, cv_VectorOfGPCPatchDescriptor_clear,
		cv_VectorOfGPCPatchDescriptor_get, cv_VectorOfGPCPatchDescriptor_set,
		cv_VectorOfGPCPatchDescriptor_push, cv_VectorOfGPCPatchDescriptor_insert,
	}
	vector_non_copy_or_bool! { crate::optflow::GPCPatchDescriptor }
	
	unsafe impl Send for core::Vector<crate::optflow::GPCPatchDescriptor> {}
	
}
#[cfg(ocvrs_has_module_optflow)]
pub use optflow_types::*;

#[cfg(ocvrs_has_module_ovis)]
mod ovis_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfWindowScene = core::Ptr<dyn crate::ovis::WindowScene>;
	
	ptr_extern! { dyn crate::ovis::WindowScene,
		cv_PtrOfWindowScene_delete, cv_PtrOfWindowScene_get_inner_ptr, cv_PtrOfWindowScene_get_inner_ptr_mut
	}
	
	impl PtrOfWindowScene {
		#[inline] pub fn as_raw_PtrOfWindowScene(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfWindowScene(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ovis::WindowSceneConst for PtrOfWindowScene {
		#[inline] fn as_raw_WindowScene(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ovis::WindowScene for PtrOfWindowScene {
		#[inline] fn as_raw_mut_WindowScene(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_ovis)]
pub use ovis_types::*;

#[cfg(ocvrs_has_module_phase_unwrapping)]
mod phase_unwrapping_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfHistogramPhaseUnwrapping = core::Ptr<dyn crate::phase_unwrapping::HistogramPhaseUnwrapping>;
	
	ptr_extern! { dyn crate::phase_unwrapping::HistogramPhaseUnwrapping,
		cv_PtrOfHistogramPhaseUnwrapping_delete, cv_PtrOfHistogramPhaseUnwrapping_get_inner_ptr, cv_PtrOfHistogramPhaseUnwrapping_get_inner_ptr_mut
	}
	
	impl PtrOfHistogramPhaseUnwrapping {
		#[inline] pub fn as_raw_PtrOfHistogramPhaseUnwrapping(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHistogramPhaseUnwrapping(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::phase_unwrapping::HistogramPhaseUnwrappingConst for PtrOfHistogramPhaseUnwrapping {
		#[inline] fn as_raw_HistogramPhaseUnwrapping(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::phase_unwrapping::HistogramPhaseUnwrapping for PtrOfHistogramPhaseUnwrapping {
		#[inline] fn as_raw_mut_HistogramPhaseUnwrapping(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfHistogramPhaseUnwrapping {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfHistogramPhaseUnwrapping {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::phase_unwrapping::PhaseUnwrappingConst for PtrOfHistogramPhaseUnwrapping {
		#[inline] fn as_raw_PhaseUnwrapping(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::phase_unwrapping::PhaseUnwrapping for PtrOfHistogramPhaseUnwrapping {
		#[inline] fn as_raw_mut_PhaseUnwrapping(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_phase_unwrapping)]
pub use phase_unwrapping_types::*;

#[cfg(ocvrs_has_module_photo)]
mod photo_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAlignMTB = core::Ptr<dyn crate::photo::AlignMTB>;
	
	ptr_extern! { dyn crate::photo::AlignMTB,
		cv_PtrOfAlignMTB_delete, cv_PtrOfAlignMTB_get_inner_ptr, cv_PtrOfAlignMTB_get_inner_ptr_mut
	}
	
	impl PtrOfAlignMTB {
		#[inline] pub fn as_raw_PtrOfAlignMTB(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAlignMTB(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::photo::AlignMTBConst for PtrOfAlignMTB {
		#[inline] fn as_raw_AlignMTB(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::AlignMTB for PtrOfAlignMTB {
		#[inline] fn as_raw_mut_AlignMTB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfAlignMTB {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfAlignMTB {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::AlignExposuresConst for PtrOfAlignMTB {
		#[inline] fn as_raw_AlignExposures(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::AlignExposures for PtrOfAlignMTB {
		#[inline] fn as_raw_mut_AlignExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCalibrateDebevec = core::Ptr<dyn crate::photo::CalibrateDebevec>;
	
	ptr_extern! { dyn crate::photo::CalibrateDebevec,
		cv_PtrOfCalibrateDebevec_delete, cv_PtrOfCalibrateDebevec_get_inner_ptr, cv_PtrOfCalibrateDebevec_get_inner_ptr_mut
	}
	
	impl PtrOfCalibrateDebevec {
		#[inline] pub fn as_raw_PtrOfCalibrateDebevec(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCalibrateDebevec(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::photo::CalibrateDebevecConst for PtrOfCalibrateDebevec {
		#[inline] fn as_raw_CalibrateDebevec(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::CalibrateDebevec for PtrOfCalibrateDebevec {
		#[inline] fn as_raw_mut_CalibrateDebevec(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCalibrateDebevec {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCalibrateDebevec {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::CalibrateCRFConst for PtrOfCalibrateDebevec {
		#[inline] fn as_raw_CalibrateCRF(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::CalibrateCRF for PtrOfCalibrateDebevec {
		#[inline] fn as_raw_mut_CalibrateCRF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCalibrateRobertson = core::Ptr<dyn crate::photo::CalibrateRobertson>;
	
	ptr_extern! { dyn crate::photo::CalibrateRobertson,
		cv_PtrOfCalibrateRobertson_delete, cv_PtrOfCalibrateRobertson_get_inner_ptr, cv_PtrOfCalibrateRobertson_get_inner_ptr_mut
	}
	
	impl PtrOfCalibrateRobertson {
		#[inline] pub fn as_raw_PtrOfCalibrateRobertson(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCalibrateRobertson(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::photo::CalibrateRobertsonConst for PtrOfCalibrateRobertson {
		#[inline] fn as_raw_CalibrateRobertson(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::CalibrateRobertson for PtrOfCalibrateRobertson {
		#[inline] fn as_raw_mut_CalibrateRobertson(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfCalibrateRobertson {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfCalibrateRobertson {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::CalibrateCRFConst for PtrOfCalibrateRobertson {
		#[inline] fn as_raw_CalibrateCRF(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::CalibrateCRF for PtrOfCalibrateRobertson {
		#[inline] fn as_raw_mut_CalibrateCRF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMergeDebevec = core::Ptr<dyn crate::photo::MergeDebevec>;
	
	ptr_extern! { dyn crate::photo::MergeDebevec,
		cv_PtrOfMergeDebevec_delete, cv_PtrOfMergeDebevec_get_inner_ptr, cv_PtrOfMergeDebevec_get_inner_ptr_mut
	}
	
	impl PtrOfMergeDebevec {
		#[inline] pub fn as_raw_PtrOfMergeDebevec(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMergeDebevec(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::photo::MergeDebevecConst for PtrOfMergeDebevec {
		#[inline] fn as_raw_MergeDebevec(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::MergeDebevec for PtrOfMergeDebevec {
		#[inline] fn as_raw_mut_MergeDebevec(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfMergeDebevec {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfMergeDebevec {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::MergeExposuresConst for PtrOfMergeDebevec {
		#[inline] fn as_raw_MergeExposures(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::MergeExposures for PtrOfMergeDebevec {
		#[inline] fn as_raw_mut_MergeExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMergeMertens = core::Ptr<dyn crate::photo::MergeMertens>;
	
	ptr_extern! { dyn crate::photo::MergeMertens,
		cv_PtrOfMergeMertens_delete, cv_PtrOfMergeMertens_get_inner_ptr, cv_PtrOfMergeMertens_get_inner_ptr_mut
	}
	
	impl PtrOfMergeMertens {
		#[inline] pub fn as_raw_PtrOfMergeMertens(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMergeMertens(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::photo::MergeMertensConst for PtrOfMergeMertens {
		#[inline] fn as_raw_MergeMertens(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::MergeMertens for PtrOfMergeMertens {
		#[inline] fn as_raw_mut_MergeMertens(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfMergeMertens {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfMergeMertens {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::MergeExposuresConst for PtrOfMergeMertens {
		#[inline] fn as_raw_MergeExposures(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::MergeExposures for PtrOfMergeMertens {
		#[inline] fn as_raw_mut_MergeExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMergeRobertson = core::Ptr<dyn crate::photo::MergeRobertson>;
	
	ptr_extern! { dyn crate::photo::MergeRobertson,
		cv_PtrOfMergeRobertson_delete, cv_PtrOfMergeRobertson_get_inner_ptr, cv_PtrOfMergeRobertson_get_inner_ptr_mut
	}
	
	impl PtrOfMergeRobertson {
		#[inline] pub fn as_raw_PtrOfMergeRobertson(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMergeRobertson(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::photo::MergeRobertsonConst for PtrOfMergeRobertson {
		#[inline] fn as_raw_MergeRobertson(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::MergeRobertson for PtrOfMergeRobertson {
		#[inline] fn as_raw_mut_MergeRobertson(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfMergeRobertson {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfMergeRobertson {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::MergeExposuresConst for PtrOfMergeRobertson {
		#[inline] fn as_raw_MergeExposures(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::MergeExposures for PtrOfMergeRobertson {
		#[inline] fn as_raw_mut_MergeExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemap = core::Ptr<dyn crate::photo::Tonemap>;
	
	ptr_extern! { dyn crate::photo::Tonemap,
		cv_PtrOfTonemap_delete, cv_PtrOfTonemap_get_inner_ptr, cv_PtrOfTonemap_get_inner_ptr_mut
	}
	
	impl PtrOfTonemap {
		#[inline] pub fn as_raw_PtrOfTonemap(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTonemap(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::photo::TonemapConst for PtrOfTonemap {
		#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::Tonemap for PtrOfTonemap {
		#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfTonemap {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfTonemap {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemapDrago = core::Ptr<dyn crate::photo::TonemapDrago>;
	
	ptr_extern! { dyn crate::photo::TonemapDrago,
		cv_PtrOfTonemapDrago_delete, cv_PtrOfTonemapDrago_get_inner_ptr, cv_PtrOfTonemapDrago_get_inner_ptr_mut
	}
	
	impl PtrOfTonemapDrago {
		#[inline] pub fn as_raw_PtrOfTonemapDrago(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTonemapDrago(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::photo::TonemapDragoConst for PtrOfTonemapDrago {
		#[inline] fn as_raw_TonemapDrago(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::TonemapDrago for PtrOfTonemapDrago {
		#[inline] fn as_raw_mut_TonemapDrago(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfTonemapDrago {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfTonemapDrago {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::TonemapConst for PtrOfTonemapDrago {
		#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::Tonemap for PtrOfTonemapDrago {
		#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemapMantiuk = core::Ptr<dyn crate::photo::TonemapMantiuk>;
	
	ptr_extern! { dyn crate::photo::TonemapMantiuk,
		cv_PtrOfTonemapMantiuk_delete, cv_PtrOfTonemapMantiuk_get_inner_ptr, cv_PtrOfTonemapMantiuk_get_inner_ptr_mut
	}
	
	impl PtrOfTonemapMantiuk {
		#[inline] pub fn as_raw_PtrOfTonemapMantiuk(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTonemapMantiuk(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::photo::TonemapMantiukConst for PtrOfTonemapMantiuk {
		#[inline] fn as_raw_TonemapMantiuk(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::TonemapMantiuk for PtrOfTonemapMantiuk {
		#[inline] fn as_raw_mut_TonemapMantiuk(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfTonemapMantiuk {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfTonemapMantiuk {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::TonemapConst for PtrOfTonemapMantiuk {
		#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::Tonemap for PtrOfTonemapMantiuk {
		#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemapReinhard = core::Ptr<dyn crate::photo::TonemapReinhard>;
	
	ptr_extern! { dyn crate::photo::TonemapReinhard,
		cv_PtrOfTonemapReinhard_delete, cv_PtrOfTonemapReinhard_get_inner_ptr, cv_PtrOfTonemapReinhard_get_inner_ptr_mut
	}
	
	impl PtrOfTonemapReinhard {
		#[inline] pub fn as_raw_PtrOfTonemapReinhard(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTonemapReinhard(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::photo::TonemapReinhardConst for PtrOfTonemapReinhard {
		#[inline] fn as_raw_TonemapReinhard(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::TonemapReinhard for PtrOfTonemapReinhard {
		#[inline] fn as_raw_mut_TonemapReinhard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfTonemapReinhard {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfTonemapReinhard {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::TonemapConst for PtrOfTonemapReinhard {
		#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::Tonemap for PtrOfTonemapReinhard {
		#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_photo)]
pub use photo_types::*;

#[cfg(ocvrs_has_module_plot)]
mod plot_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfPlot2d = core::Ptr<dyn crate::plot::Plot2d>;
	
	ptr_extern! { dyn crate::plot::Plot2d,
		cv_PtrOfPlot2d_delete, cv_PtrOfPlot2d_get_inner_ptr, cv_PtrOfPlot2d_get_inner_ptr_mut
	}
	
	impl PtrOfPlot2d {
		#[inline] pub fn as_raw_PtrOfPlot2d(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPlot2d(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::plot::Plot2dConst for PtrOfPlot2d {
		#[inline] fn as_raw_Plot2d(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::plot::Plot2d for PtrOfPlot2d {
		#[inline] fn as_raw_mut_Plot2d(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfPlot2d {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfPlot2d {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_plot)]
pub use plot_types::*;

#[cfg(ocvrs_has_module_quality)]
mod quality_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfQualityBRISQUE = core::Ptr<crate::quality::QualityBRISQUE>;
	
	ptr_extern! { crate::quality::QualityBRISQUE,
		cv_PtrOfQualityBRISQUE_delete, cv_PtrOfQualityBRISQUE_get_inner_ptr, cv_PtrOfQualityBRISQUE_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::quality::QualityBRISQUE, cv_PtrOfQualityBRISQUE_new }
	
	impl PtrOfQualityBRISQUE {
		#[inline] pub fn as_raw_PtrOfQualityBRISQUE(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfQualityBRISQUE(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::quality::QualityBRISQUETraitConst for PtrOfQualityBRISQUE {
		#[inline] fn as_raw_QualityBRISQUE(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::quality::QualityBRISQUETrait for PtrOfQualityBRISQUE {
		#[inline] fn as_raw_mut_QualityBRISQUE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfQualityBRISQUE {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfQualityBRISQUE {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::quality::QualityBaseConst for PtrOfQualityBRISQUE {
		#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::quality::QualityBase for PtrOfQualityBRISQUE {
		#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfQualityGMSD = core::Ptr<crate::quality::QualityGMSD>;
	
	ptr_extern! { crate::quality::QualityGMSD,
		cv_PtrOfQualityGMSD_delete, cv_PtrOfQualityGMSD_get_inner_ptr, cv_PtrOfQualityGMSD_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::quality::QualityGMSD, cv_PtrOfQualityGMSD_new }
	
	impl PtrOfQualityGMSD {
		#[inline] pub fn as_raw_PtrOfQualityGMSD(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfQualityGMSD(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::quality::QualityGMSDTraitConst for PtrOfQualityGMSD {
		#[inline] fn as_raw_QualityGMSD(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::quality::QualityGMSDTrait for PtrOfQualityGMSD {
		#[inline] fn as_raw_mut_QualityGMSD(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfQualityGMSD {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfQualityGMSD {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::quality::QualityBaseConst for PtrOfQualityGMSD {
		#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::quality::QualityBase for PtrOfQualityGMSD {
		#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfQualityMSE = core::Ptr<crate::quality::QualityMSE>;
	
	ptr_extern! { crate::quality::QualityMSE,
		cv_PtrOfQualityMSE_delete, cv_PtrOfQualityMSE_get_inner_ptr, cv_PtrOfQualityMSE_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::quality::QualityMSE, cv_PtrOfQualityMSE_new }
	
	impl PtrOfQualityMSE {
		#[inline] pub fn as_raw_PtrOfQualityMSE(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfQualityMSE(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::quality::QualityMSETraitConst for PtrOfQualityMSE {
		#[inline] fn as_raw_QualityMSE(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::quality::QualityMSETrait for PtrOfQualityMSE {
		#[inline] fn as_raw_mut_QualityMSE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfQualityMSE {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfQualityMSE {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::quality::QualityBaseConst for PtrOfQualityMSE {
		#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::quality::QualityBase for PtrOfQualityMSE {
		#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfQualityPSNR = core::Ptr<crate::quality::QualityPSNR>;
	
	ptr_extern! { crate::quality::QualityPSNR,
		cv_PtrOfQualityPSNR_delete, cv_PtrOfQualityPSNR_get_inner_ptr, cv_PtrOfQualityPSNR_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::quality::QualityPSNR, cv_PtrOfQualityPSNR_new }
	
	impl PtrOfQualityPSNR {
		#[inline] pub fn as_raw_PtrOfQualityPSNR(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfQualityPSNR(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::quality::QualityPSNRTraitConst for PtrOfQualityPSNR {
		#[inline] fn as_raw_QualityPSNR(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::quality::QualityPSNRTrait for PtrOfQualityPSNR {
		#[inline] fn as_raw_mut_QualityPSNR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfQualityPSNR {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfQualityPSNR {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::quality::QualityBaseConst for PtrOfQualityPSNR {
		#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::quality::QualityBase for PtrOfQualityPSNR {
		#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfQualitySSIM = core::Ptr<crate::quality::QualitySSIM>;
	
	ptr_extern! { crate::quality::QualitySSIM,
		cv_PtrOfQualitySSIM_delete, cv_PtrOfQualitySSIM_get_inner_ptr, cv_PtrOfQualitySSIM_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::quality::QualitySSIM, cv_PtrOfQualitySSIM_new }
	
	impl PtrOfQualitySSIM {
		#[inline] pub fn as_raw_PtrOfQualitySSIM(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfQualitySSIM(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::quality::QualitySSIMTraitConst for PtrOfQualitySSIM {
		#[inline] fn as_raw_QualitySSIM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::quality::QualitySSIMTrait for PtrOfQualitySSIM {
		#[inline] fn as_raw_mut_QualitySSIM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfQualitySSIM {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfQualitySSIM {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::quality::QualityBaseConst for PtrOfQualitySSIM {
		#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::quality::QualityBase for PtrOfQualitySSIM {
		#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_quality)]
pub use quality_types::*;

#[cfg(ocvrs_has_module_rapid)]
mod rapid_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfOLSTracker = core::Ptr<dyn crate::rapid::OLSTracker>;
	
	ptr_extern! { dyn crate::rapid::OLSTracker,
		cv_PtrOfOLSTracker_delete, cv_PtrOfOLSTracker_get_inner_ptr, cv_PtrOfOLSTracker_get_inner_ptr_mut
	}
	
	impl PtrOfOLSTracker {
		#[inline] pub fn as_raw_PtrOfOLSTracker(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOLSTracker(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rapid::OLSTrackerConst for PtrOfOLSTracker {
		#[inline] fn as_raw_OLSTracker(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rapid::OLSTracker for PtrOfOLSTracker {
		#[inline] fn as_raw_mut_OLSTracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfOLSTracker {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfOLSTracker {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rapid::TrackerConst for PtrOfOLSTracker {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rapid::Tracker for PtrOfOLSTracker {
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRapid = core::Ptr<dyn crate::rapid::Rapid>;
	
	ptr_extern! { dyn crate::rapid::Rapid,
		cv_PtrOfRapid_delete, cv_PtrOfRapid_get_inner_ptr, cv_PtrOfRapid_get_inner_ptr_mut
	}
	
	impl PtrOfRapid {
		#[inline] pub fn as_raw_PtrOfRapid(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRapid(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rapid::RapidConst for PtrOfRapid {
		#[inline] fn as_raw_Rapid(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rapid::Rapid for PtrOfRapid {
		#[inline] fn as_raw_mut_Rapid(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfRapid {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfRapid {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rapid::TrackerConst for PtrOfRapid {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rapid::Tracker for PtrOfRapid {
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_rapid)]
pub use rapid_types::*;

#[cfg(ocvrs_has_module_rgbd)]
mod rgbd_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfColoredKinfu_ColoredKinFu = core::Ptr<dyn crate::rgbd::ColoredKinfu_ColoredKinFu>;
	
	ptr_extern! { dyn crate::rgbd::ColoredKinfu_ColoredKinFu,
		cv_PtrOfColoredKinfu_ColoredKinFu_delete, cv_PtrOfColoredKinfu_ColoredKinFu_get_inner_ptr, cv_PtrOfColoredKinfu_ColoredKinFu_get_inner_ptr_mut
	}
	
	impl PtrOfColoredKinfu_ColoredKinFu {
		#[inline] pub fn as_raw_PtrOfColoredKinfu_ColoredKinFu(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfColoredKinfu_ColoredKinFu(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::ColoredKinfu_ColoredKinFuConst for PtrOfColoredKinfu_ColoredKinFu {
		#[inline] fn as_raw_ColoredKinfu_ColoredKinFu(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::ColoredKinfu_ColoredKinFu for PtrOfColoredKinfu_ColoredKinFu {
		#[inline] fn as_raw_mut_ColoredKinfu_ColoredKinFu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfColoredKinfu_Params = core::Ptr<crate::rgbd::ColoredKinfu_Params>;
	
	ptr_extern! { crate::rgbd::ColoredKinfu_Params,
		cv_PtrOfColoredKinfu_Params_delete, cv_PtrOfColoredKinfu_Params_get_inner_ptr, cv_PtrOfColoredKinfu_Params_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::ColoredKinfu_Params, cv_PtrOfColoredKinfu_Params_new }
	
	impl PtrOfColoredKinfu_Params {
		#[inline] pub fn as_raw_PtrOfColoredKinfu_Params(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfColoredKinfu_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::ColoredKinfu_ParamsTraitConst for PtrOfColoredKinfu_Params {
		#[inline] fn as_raw_ColoredKinfu_Params(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::ColoredKinfu_ParamsTrait for PtrOfColoredKinfu_Params {
		#[inline] fn as_raw_mut_ColoredKinfu_Params(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDepthCleaner = core::Ptr<crate::rgbd::DepthCleaner>;
	
	ptr_extern! { crate::rgbd::DepthCleaner,
		cv_PtrOfDepthCleaner_delete, cv_PtrOfDepthCleaner_get_inner_ptr, cv_PtrOfDepthCleaner_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::DepthCleaner, cv_PtrOfDepthCleaner_new }
	
	impl PtrOfDepthCleaner {
		#[inline] pub fn as_raw_PtrOfDepthCleaner(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDepthCleaner(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::DepthCleanerTraitConst for PtrOfDepthCleaner {
		#[inline] fn as_raw_DepthCleaner(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::DepthCleanerTrait for PtrOfDepthCleaner {
		#[inline] fn as_raw_mut_DepthCleaner(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfDepthCleaner {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfDepthCleaner {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDynafu_DynaFu = core::Ptr<dyn crate::rgbd::Dynafu_DynaFu>;
	
	ptr_extern! { dyn crate::rgbd::Dynafu_DynaFu,
		cv_PtrOfDynafu_DynaFu_delete, cv_PtrOfDynafu_DynaFu_get_inner_ptr, cv_PtrOfDynafu_DynaFu_get_inner_ptr_mut
	}
	
	impl PtrOfDynafu_DynaFu {
		#[inline] pub fn as_raw_PtrOfDynafu_DynaFu(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDynafu_DynaFu(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Dynafu_DynaFuConst for PtrOfDynafu_DynaFu {
		#[inline] fn as_raw_Dynafu_DynaFu(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Dynafu_DynaFu for PtrOfDynafu_DynaFu {
		#[inline] fn as_raw_mut_Dynafu_DynaFu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFastICPOdometry = core::Ptr<crate::rgbd::FastICPOdometry>;
	
	ptr_extern! { crate::rgbd::FastICPOdometry,
		cv_PtrOfFastICPOdometry_delete, cv_PtrOfFastICPOdometry_get_inner_ptr, cv_PtrOfFastICPOdometry_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::FastICPOdometry, cv_PtrOfFastICPOdometry_new }
	
	impl PtrOfFastICPOdometry {
		#[inline] pub fn as_raw_PtrOfFastICPOdometry(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFastICPOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::FastICPOdometryTraitConst for PtrOfFastICPOdometry {
		#[inline] fn as_raw_FastICPOdometry(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::FastICPOdometryTrait for PtrOfFastICPOdometry {
		#[inline] fn as_raw_mut_FastICPOdometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfFastICPOdometry {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfFastICPOdometry {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::OdometryConst for PtrOfFastICPOdometry {
		#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Odometry for PtrOfFastICPOdometry {
		#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfICPOdometry = core::Ptr<crate::rgbd::ICPOdometry>;
	
	ptr_extern! { crate::rgbd::ICPOdometry,
		cv_PtrOfICPOdometry_delete, cv_PtrOfICPOdometry_get_inner_ptr, cv_PtrOfICPOdometry_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::ICPOdometry, cv_PtrOfICPOdometry_new }
	
	impl PtrOfICPOdometry {
		#[inline] pub fn as_raw_PtrOfICPOdometry(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfICPOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::ICPOdometryTraitConst for PtrOfICPOdometry {
		#[inline] fn as_raw_ICPOdometry(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::ICPOdometryTrait for PtrOfICPOdometry {
		#[inline] fn as_raw_mut_ICPOdometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfICPOdometry {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfICPOdometry {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::OdometryConst for PtrOfICPOdometry {
		#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Odometry for PtrOfICPOdometry {
		#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfKinfu_Detail_PoseGraph = core::Ptr<dyn crate::rgbd::Kinfu_Detail_PoseGraph>;
	
	ptr_extern! { dyn crate::rgbd::Kinfu_Detail_PoseGraph,
		cv_PtrOfKinfu_Detail_PoseGraph_delete, cv_PtrOfKinfu_Detail_PoseGraph_get_inner_ptr, cv_PtrOfKinfu_Detail_PoseGraph_get_inner_ptr_mut
	}
	
	impl PtrOfKinfu_Detail_PoseGraph {
		#[inline] pub fn as_raw_PtrOfKinfu_Detail_PoseGraph(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKinfu_Detail_PoseGraph(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Kinfu_Detail_PoseGraphConst for PtrOfKinfu_Detail_PoseGraph {
		#[inline] fn as_raw_Kinfu_Detail_PoseGraph(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Kinfu_Detail_PoseGraph for PtrOfKinfu_Detail_PoseGraph {
		#[inline] fn as_raw_mut_Kinfu_Detail_PoseGraph(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfKinfu_KinFu = core::Ptr<dyn crate::rgbd::Kinfu_KinFu>;
	
	ptr_extern! { dyn crate::rgbd::Kinfu_KinFu,
		cv_PtrOfKinfu_KinFu_delete, cv_PtrOfKinfu_KinFu_get_inner_ptr, cv_PtrOfKinfu_KinFu_get_inner_ptr_mut
	}
	
	impl PtrOfKinfu_KinFu {
		#[inline] pub fn as_raw_PtrOfKinfu_KinFu(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKinfu_KinFu(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Kinfu_KinFuConst for PtrOfKinfu_KinFu {
		#[inline] fn as_raw_Kinfu_KinFu(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Kinfu_KinFu for PtrOfKinfu_KinFu {
		#[inline] fn as_raw_mut_Kinfu_KinFu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfKinfu_Params = core::Ptr<crate::rgbd::Kinfu_Params>;
	
	ptr_extern! { crate::rgbd::Kinfu_Params,
		cv_PtrOfKinfu_Params_delete, cv_PtrOfKinfu_Params_get_inner_ptr, cv_PtrOfKinfu_Params_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::Kinfu_Params, cv_PtrOfKinfu_Params_new }
	
	impl PtrOfKinfu_Params {
		#[inline] pub fn as_raw_PtrOfKinfu_Params(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKinfu_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Kinfu_ParamsTraitConst for PtrOfKinfu_Params {
		#[inline] fn as_raw_Kinfu_Params(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Kinfu_ParamsTrait for PtrOfKinfu_Params {
		#[inline] fn as_raw_mut_Kinfu_Params(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfKinfu_Volume = core::Ptr<dyn crate::rgbd::Kinfu_Volume>;
	
	ptr_extern! { dyn crate::rgbd::Kinfu_Volume,
		cv_PtrOfKinfu_Volume_delete, cv_PtrOfKinfu_Volume_get_inner_ptr, cv_PtrOfKinfu_Volume_get_inner_ptr_mut
	}
	
	impl PtrOfKinfu_Volume {
		#[inline] pub fn as_raw_PtrOfKinfu_Volume(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKinfu_Volume(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Kinfu_VolumeConst for PtrOfKinfu_Volume {
		#[inline] fn as_raw_Kinfu_Volume(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Kinfu_Volume for PtrOfKinfu_Volume {
		#[inline] fn as_raw_mut_Kinfu_Volume(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfKinfu_VolumeParams = core::Ptr<crate::rgbd::Kinfu_VolumeParams>;
	
	ptr_extern! { crate::rgbd::Kinfu_VolumeParams,
		cv_PtrOfKinfu_VolumeParams_delete, cv_PtrOfKinfu_VolumeParams_get_inner_ptr, cv_PtrOfKinfu_VolumeParams_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::Kinfu_VolumeParams, cv_PtrOfKinfu_VolumeParams_new }
	
	impl PtrOfKinfu_VolumeParams {
		#[inline] pub fn as_raw_PtrOfKinfu_VolumeParams(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKinfu_VolumeParams(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Kinfu_VolumeParamsTraitConst for PtrOfKinfu_VolumeParams {
		#[inline] fn as_raw_Kinfu_VolumeParams(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Kinfu_VolumeParamsTrait for PtrOfKinfu_VolumeParams {
		#[inline] fn as_raw_mut_Kinfu_VolumeParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLargeKinfu = core::Ptr<dyn crate::rgbd::LargeKinfu>;
	
	ptr_extern! { dyn crate::rgbd::LargeKinfu,
		cv_PtrOfLargeKinfu_delete, cv_PtrOfLargeKinfu_get_inner_ptr, cv_PtrOfLargeKinfu_get_inner_ptr_mut
	}
	
	impl PtrOfLargeKinfu {
		#[inline] pub fn as_raw_PtrOfLargeKinfu(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLargeKinfu(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::LargeKinfuConst for PtrOfLargeKinfu {
		#[inline] fn as_raw_LargeKinfu(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::LargeKinfu for PtrOfLargeKinfu {
		#[inline] fn as_raw_mut_LargeKinfu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLinemod_ColorGradient = core::Ptr<crate::rgbd::Linemod_ColorGradient>;
	
	ptr_extern! { crate::rgbd::Linemod_ColorGradient,
		cv_PtrOfLinemod_ColorGradient_delete, cv_PtrOfLinemod_ColorGradient_get_inner_ptr, cv_PtrOfLinemod_ColorGradient_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::Linemod_ColorGradient, cv_PtrOfLinemod_ColorGradient_new }
	
	impl PtrOfLinemod_ColorGradient {
		#[inline] pub fn as_raw_PtrOfLinemod_ColorGradient(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLinemod_ColorGradient(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Linemod_ColorGradientTraitConst for PtrOfLinemod_ColorGradient {
		#[inline] fn as_raw_Linemod_ColorGradient(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Linemod_ColorGradientTrait for PtrOfLinemod_ColorGradient {
		#[inline] fn as_raw_mut_Linemod_ColorGradient(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::Linemod_ModalityConst for PtrOfLinemod_ColorGradient {
		#[inline] fn as_raw_Linemod_Modality(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Linemod_Modality for PtrOfLinemod_ColorGradient {
		#[inline] fn as_raw_mut_Linemod_Modality(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLinemod_DepthNormal = core::Ptr<crate::rgbd::Linemod_DepthNormal>;
	
	ptr_extern! { crate::rgbd::Linemod_DepthNormal,
		cv_PtrOfLinemod_DepthNormal_delete, cv_PtrOfLinemod_DepthNormal_get_inner_ptr, cv_PtrOfLinemod_DepthNormal_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::Linemod_DepthNormal, cv_PtrOfLinemod_DepthNormal_new }
	
	impl PtrOfLinemod_DepthNormal {
		#[inline] pub fn as_raw_PtrOfLinemod_DepthNormal(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLinemod_DepthNormal(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Linemod_DepthNormalTraitConst for PtrOfLinemod_DepthNormal {
		#[inline] fn as_raw_Linemod_DepthNormal(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Linemod_DepthNormalTrait for PtrOfLinemod_DepthNormal {
		#[inline] fn as_raw_mut_Linemod_DepthNormal(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::Linemod_ModalityConst for PtrOfLinemod_DepthNormal {
		#[inline] fn as_raw_Linemod_Modality(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Linemod_Modality for PtrOfLinemod_DepthNormal {
		#[inline] fn as_raw_mut_Linemod_Modality(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLinemod_Detector = core::Ptr<crate::rgbd::Linemod_Detector>;
	
	ptr_extern! { crate::rgbd::Linemod_Detector,
		cv_PtrOfLinemod_Detector_delete, cv_PtrOfLinemod_Detector_get_inner_ptr, cv_PtrOfLinemod_Detector_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::Linemod_Detector, cv_PtrOfLinemod_Detector_new }
	
	impl PtrOfLinemod_Detector {
		#[inline] pub fn as_raw_PtrOfLinemod_Detector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLinemod_Detector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Linemod_DetectorTraitConst for PtrOfLinemod_Detector {
		#[inline] fn as_raw_Linemod_Detector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Linemod_DetectorTrait for PtrOfLinemod_Detector {
		#[inline] fn as_raw_mut_Linemod_Detector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLinemod_Modality = core::Ptr<dyn crate::rgbd::Linemod_Modality>;
	
	ptr_extern! { dyn crate::rgbd::Linemod_Modality,
		cv_PtrOfLinemod_Modality_delete, cv_PtrOfLinemod_Modality_get_inner_ptr, cv_PtrOfLinemod_Modality_get_inner_ptr_mut
	}
	
	impl PtrOfLinemod_Modality {
		#[inline] pub fn as_raw_PtrOfLinemod_Modality(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLinemod_Modality(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Linemod_ModalityConst for PtrOfLinemod_Modality {
		#[inline] fn as_raw_Linemod_Modality(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Linemod_Modality for PtrOfLinemod_Modality {
		#[inline] fn as_raw_mut_Linemod_Modality(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLinemod_QuantizedPyramid = core::Ptr<dyn crate::rgbd::Linemod_QuantizedPyramid>;
	
	ptr_extern! { dyn crate::rgbd::Linemod_QuantizedPyramid,
		cv_PtrOfLinemod_QuantizedPyramid_delete, cv_PtrOfLinemod_QuantizedPyramid_get_inner_ptr, cv_PtrOfLinemod_QuantizedPyramid_get_inner_ptr_mut
	}
	
	impl PtrOfLinemod_QuantizedPyramid {
		#[inline] pub fn as_raw_PtrOfLinemod_QuantizedPyramid(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLinemod_QuantizedPyramid(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Linemod_QuantizedPyramidConst for PtrOfLinemod_QuantizedPyramid {
		#[inline] fn as_raw_Linemod_QuantizedPyramid(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Linemod_QuantizedPyramid for PtrOfLinemod_QuantizedPyramid {
		#[inline] fn as_raw_mut_Linemod_QuantizedPyramid(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOdometry = core::Ptr<dyn crate::rgbd::Odometry>;
	
	ptr_extern! { dyn crate::rgbd::Odometry,
		cv_PtrOfOdometry_delete, cv_PtrOfOdometry_get_inner_ptr, cv_PtrOfOdometry_get_inner_ptr_mut
	}
	
	impl PtrOfOdometry {
		#[inline] pub fn as_raw_PtrOfOdometry(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::OdometryConst for PtrOfOdometry {
		#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Odometry for PtrOfOdometry {
		#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfOdometry {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfOdometry {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOdometryFrame = core::Ptr<crate::rgbd::OdometryFrame>;
	
	ptr_extern! { crate::rgbd::OdometryFrame,
		cv_PtrOfOdometryFrame_delete, cv_PtrOfOdometryFrame_get_inner_ptr, cv_PtrOfOdometryFrame_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::OdometryFrame, cv_PtrOfOdometryFrame_new }
	
	impl PtrOfOdometryFrame {
		#[inline] pub fn as_raw_PtrOfOdometryFrame(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOdometryFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::OdometryFrameTraitConst for PtrOfOdometryFrame {
		#[inline] fn as_raw_OdometryFrame(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::OdometryFrameTrait for PtrOfOdometryFrame {
		#[inline] fn as_raw_mut_OdometryFrame(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::RgbdFrameTraitConst for PtrOfOdometryFrame {
		#[inline] fn as_raw_RgbdFrame(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::RgbdFrameTrait for PtrOfOdometryFrame {
		#[inline] fn as_raw_mut_RgbdFrame(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfParams = core::Ptr<crate::rgbd::Params>;
	
	ptr_extern! { crate::rgbd::Params,
		cv_PtrOfParams_delete, cv_PtrOfParams_get_inner_ptr, cv_PtrOfParams_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::Params, cv_PtrOfParams_new }
	
	impl PtrOfParams {
		#[inline] pub fn as_raw_PtrOfParams(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfParams(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::ParamsTraitConst for PtrOfParams {
		#[inline] fn as_raw_Params(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::ParamsTrait for PtrOfParams {
		#[inline] fn as_raw_mut_Params(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRgbdFrame = core::Ptr<crate::rgbd::RgbdFrame>;
	
	ptr_extern! { crate::rgbd::RgbdFrame,
		cv_PtrOfRgbdFrame_delete, cv_PtrOfRgbdFrame_get_inner_ptr, cv_PtrOfRgbdFrame_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::RgbdFrame, cv_PtrOfRgbdFrame_new }
	
	impl PtrOfRgbdFrame {
		#[inline] pub fn as_raw_PtrOfRgbdFrame(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRgbdFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::RgbdFrameTraitConst for PtrOfRgbdFrame {
		#[inline] fn as_raw_RgbdFrame(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::RgbdFrameTrait for PtrOfRgbdFrame {
		#[inline] fn as_raw_mut_RgbdFrame(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRgbdICPOdometry = core::Ptr<crate::rgbd::RgbdICPOdometry>;
	
	ptr_extern! { crate::rgbd::RgbdICPOdometry,
		cv_PtrOfRgbdICPOdometry_delete, cv_PtrOfRgbdICPOdometry_get_inner_ptr, cv_PtrOfRgbdICPOdometry_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::RgbdICPOdometry, cv_PtrOfRgbdICPOdometry_new }
	
	impl PtrOfRgbdICPOdometry {
		#[inline] pub fn as_raw_PtrOfRgbdICPOdometry(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRgbdICPOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::RgbdICPOdometryTraitConst for PtrOfRgbdICPOdometry {
		#[inline] fn as_raw_RgbdICPOdometry(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::RgbdICPOdometryTrait for PtrOfRgbdICPOdometry {
		#[inline] fn as_raw_mut_RgbdICPOdometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfRgbdICPOdometry {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfRgbdICPOdometry {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::OdometryConst for PtrOfRgbdICPOdometry {
		#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Odometry for PtrOfRgbdICPOdometry {
		#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRgbdNormals = core::Ptr<crate::rgbd::RgbdNormals>;
	
	ptr_extern! { crate::rgbd::RgbdNormals,
		cv_PtrOfRgbdNormals_delete, cv_PtrOfRgbdNormals_get_inner_ptr, cv_PtrOfRgbdNormals_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::RgbdNormals, cv_PtrOfRgbdNormals_new }
	
	impl PtrOfRgbdNormals {
		#[inline] pub fn as_raw_PtrOfRgbdNormals(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRgbdNormals(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::RgbdNormalsTraitConst for PtrOfRgbdNormals {
		#[inline] fn as_raw_RgbdNormals(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::RgbdNormalsTrait for PtrOfRgbdNormals {
		#[inline] fn as_raw_mut_RgbdNormals(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfRgbdNormals {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfRgbdNormals {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRgbdOdometry = core::Ptr<crate::rgbd::RgbdOdometry>;
	
	ptr_extern! { crate::rgbd::RgbdOdometry,
		cv_PtrOfRgbdOdometry_delete, cv_PtrOfRgbdOdometry_get_inner_ptr, cv_PtrOfRgbdOdometry_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::RgbdOdometry, cv_PtrOfRgbdOdometry_new }
	
	impl PtrOfRgbdOdometry {
		#[inline] pub fn as_raw_PtrOfRgbdOdometry(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRgbdOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::RgbdOdometryTraitConst for PtrOfRgbdOdometry {
		#[inline] fn as_raw_RgbdOdometry(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::RgbdOdometryTrait for PtrOfRgbdOdometry {
		#[inline] fn as_raw_mut_RgbdOdometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfRgbdOdometry {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfRgbdOdometry {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::OdometryConst for PtrOfRgbdOdometry {
		#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Odometry for PtrOfRgbdOdometry {
		#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRgbdPlane = core::Ptr<crate::rgbd::RgbdPlane>;
	
	ptr_extern! { crate::rgbd::RgbdPlane,
		cv_PtrOfRgbdPlane_delete, cv_PtrOfRgbdPlane_get_inner_ptr, cv_PtrOfRgbdPlane_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::RgbdPlane, cv_PtrOfRgbdPlane_new }
	
	impl PtrOfRgbdPlane {
		#[inline] pub fn as_raw_PtrOfRgbdPlane(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRgbdPlane(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::rgbd::RgbdPlaneTraitConst for PtrOfRgbdPlane {
		#[inline] fn as_raw_RgbdPlane(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::RgbdPlaneTrait for PtrOfRgbdPlane {
		#[inline] fn as_raw_mut_RgbdPlane(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfRgbdPlane {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfRgbdPlane {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfLinemod_Feature = core::Vector<crate::rgbd::Linemod_Feature>;
	
	impl VectorOfLinemod_Feature {
		pub fn as_raw_VectorOfLinemod_Feature(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfLinemod_Feature(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::rgbd::Linemod_Feature, *const c_void, *mut c_void,
		cv_VectorOfLinemod_Feature_new, cv_VectorOfLinemod_Feature_delete,
		cv_VectorOfLinemod_Feature_len, cv_VectorOfLinemod_Feature_is_empty,
		cv_VectorOfLinemod_Feature_capacity, cv_VectorOfLinemod_Feature_shrink_to_fit,
		cv_VectorOfLinemod_Feature_reserve, cv_VectorOfLinemod_Feature_remove,
		cv_VectorOfLinemod_Feature_swap, cv_VectorOfLinemod_Feature_clear,
		cv_VectorOfLinemod_Feature_get, cv_VectorOfLinemod_Feature_set,
		cv_VectorOfLinemod_Feature_push, cv_VectorOfLinemod_Feature_insert,
	}
	vector_copy_non_bool! { crate::rgbd::Linemod_Feature, *const c_void, *mut c_void,
		cv_VectorOfLinemod_Feature_data, cv_VectorOfLinemod_Feature_data_mut, cv_VectorOfLinemod_Feature_from_slice,
		cv_VectorOfLinemod_Feature_clone,
	}
	
	unsafe impl Send for core::Vector<crate::rgbd::Linemod_Feature> {}
	
	pub type VectorOfLinemod_Match = core::Vector<crate::rgbd::Linemod_Match>;
	
	impl VectorOfLinemod_Match {
		pub fn as_raw_VectorOfLinemod_Match(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfLinemod_Match(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::rgbd::Linemod_Match, *const c_void, *mut c_void,
		cv_VectorOfLinemod_Match_new, cv_VectorOfLinemod_Match_delete,
		cv_VectorOfLinemod_Match_len, cv_VectorOfLinemod_Match_is_empty,
		cv_VectorOfLinemod_Match_capacity, cv_VectorOfLinemod_Match_shrink_to_fit,
		cv_VectorOfLinemod_Match_reserve, cv_VectorOfLinemod_Match_remove,
		cv_VectorOfLinemod_Match_swap, cv_VectorOfLinemod_Match_clear,
		cv_VectorOfLinemod_Match_get, cv_VectorOfLinemod_Match_set,
		cv_VectorOfLinemod_Match_push, cv_VectorOfLinemod_Match_insert,
	}
	vector_non_copy_or_bool! { crate::rgbd::Linemod_Match }
	
	unsafe impl Send for core::Vector<crate::rgbd::Linemod_Match> {}
	
	pub type VectorOfLinemod_Template = core::Vector<crate::rgbd::Linemod_Template>;
	
	impl VectorOfLinemod_Template {
		pub fn as_raw_VectorOfLinemod_Template(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfLinemod_Template(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::rgbd::Linemod_Template, *const c_void, *mut c_void,
		cv_VectorOfLinemod_Template_new, cv_VectorOfLinemod_Template_delete,
		cv_VectorOfLinemod_Template_len, cv_VectorOfLinemod_Template_is_empty,
		cv_VectorOfLinemod_Template_capacity, cv_VectorOfLinemod_Template_shrink_to_fit,
		cv_VectorOfLinemod_Template_reserve, cv_VectorOfLinemod_Template_remove,
		cv_VectorOfLinemod_Template_swap, cv_VectorOfLinemod_Template_clear,
		cv_VectorOfLinemod_Template_get, cv_VectorOfLinemod_Template_set,
		cv_VectorOfLinemod_Template_push, cv_VectorOfLinemod_Template_insert,
	}
	vector_non_copy_or_bool! { crate::rgbd::Linemod_Template }
	
	unsafe impl Send for core::Vector<crate::rgbd::Linemod_Template> {}
	
	pub type VectorOfPtrOfLinemod_Modality = core::Vector<core::Ptr<dyn crate::rgbd::Linemod_Modality>>;
	
	impl VectorOfPtrOfLinemod_Modality {
		pub fn as_raw_VectorOfPtrOfLinemod_Modality(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfLinemod_Modality(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Ptr<dyn crate::rgbd::Linemod_Modality>, *const c_void, *mut c_void,
		cv_VectorOfPtrOfLinemod_Modality_new, cv_VectorOfPtrOfLinemod_Modality_delete,
		cv_VectorOfPtrOfLinemod_Modality_len, cv_VectorOfPtrOfLinemod_Modality_is_empty,
		cv_VectorOfPtrOfLinemod_Modality_capacity, cv_VectorOfPtrOfLinemod_Modality_shrink_to_fit,
		cv_VectorOfPtrOfLinemod_Modality_reserve, cv_VectorOfPtrOfLinemod_Modality_remove,
		cv_VectorOfPtrOfLinemod_Modality_swap, cv_VectorOfPtrOfLinemod_Modality_clear,
		cv_VectorOfPtrOfLinemod_Modality_get, cv_VectorOfPtrOfLinemod_Modality_set,
		cv_VectorOfPtrOfLinemod_Modality_push, cv_VectorOfPtrOfLinemod_Modality_insert,
	}
	vector_non_copy_or_bool! { core::Ptr<dyn crate::rgbd::Linemod_Modality> }
	
	unsafe impl Send for core::Vector<core::Ptr<dyn crate::rgbd::Linemod_Modality>> {}
	
}
#[cfg(ocvrs_has_module_rgbd)]
pub use rgbd_types::*;

#[cfg(ocvrs_has_module_saliency)]
mod saliency_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfMotionSaliencyBinWangApr2014 = core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014>;
	
	ptr_extern! { crate::saliency::MotionSaliencyBinWangApr2014,
		cv_PtrOfMotionSaliencyBinWangApr2014_delete, cv_PtrOfMotionSaliencyBinWangApr2014_get_inner_ptr, cv_PtrOfMotionSaliencyBinWangApr2014_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::saliency::MotionSaliencyBinWangApr2014, cv_PtrOfMotionSaliencyBinWangApr2014_new }
	
	impl PtrOfMotionSaliencyBinWangApr2014 {
		#[inline] pub fn as_raw_PtrOfMotionSaliencyBinWangApr2014(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMotionSaliencyBinWangApr2014(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::saliency::MotionSaliencyBinWangApr2014TraitConst for PtrOfMotionSaliencyBinWangApr2014 {
		#[inline] fn as_raw_MotionSaliencyBinWangApr2014(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::MotionSaliencyBinWangApr2014Trait for PtrOfMotionSaliencyBinWangApr2014 {
		#[inline] fn as_raw_mut_MotionSaliencyBinWangApr2014(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfMotionSaliencyBinWangApr2014 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfMotionSaliencyBinWangApr2014 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::MotionSaliencyConst for PtrOfMotionSaliencyBinWangApr2014 {
		#[inline] fn as_raw_MotionSaliency(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::MotionSaliency for PtrOfMotionSaliencyBinWangApr2014 {
		#[inline] fn as_raw_mut_MotionSaliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::SaliencyConst for PtrOfMotionSaliencyBinWangApr2014 {
		#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::Saliency for PtrOfMotionSaliencyBinWangApr2014 {
		#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfObjectnessBING = core::Ptr<crate::saliency::ObjectnessBING>;
	
	ptr_extern! { crate::saliency::ObjectnessBING,
		cv_PtrOfObjectnessBING_delete, cv_PtrOfObjectnessBING_get_inner_ptr, cv_PtrOfObjectnessBING_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::saliency::ObjectnessBING, cv_PtrOfObjectnessBING_new }
	
	impl PtrOfObjectnessBING {
		#[inline] pub fn as_raw_PtrOfObjectnessBING(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfObjectnessBING(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::saliency::ObjectnessBINGTraitConst for PtrOfObjectnessBING {
		#[inline] fn as_raw_ObjectnessBING(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::ObjectnessBINGTrait for PtrOfObjectnessBING {
		#[inline] fn as_raw_mut_ObjectnessBING(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfObjectnessBING {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfObjectnessBING {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::ObjectnessConst for PtrOfObjectnessBING {
		#[inline] fn as_raw_Objectness(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::Objectness for PtrOfObjectnessBING {
		#[inline] fn as_raw_mut_Objectness(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::SaliencyConst for PtrOfObjectnessBING {
		#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::Saliency for PtrOfObjectnessBING {
		#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStaticSaliencyFineGrained = core::Ptr<crate::saliency::StaticSaliencyFineGrained>;
	
	ptr_extern! { crate::saliency::StaticSaliencyFineGrained,
		cv_PtrOfStaticSaliencyFineGrained_delete, cv_PtrOfStaticSaliencyFineGrained_get_inner_ptr, cv_PtrOfStaticSaliencyFineGrained_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::saliency::StaticSaliencyFineGrained, cv_PtrOfStaticSaliencyFineGrained_new }
	
	impl PtrOfStaticSaliencyFineGrained {
		#[inline] pub fn as_raw_PtrOfStaticSaliencyFineGrained(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStaticSaliencyFineGrained(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::saliency::StaticSaliencyFineGrainedTraitConst for PtrOfStaticSaliencyFineGrained {
		#[inline] fn as_raw_StaticSaliencyFineGrained(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::StaticSaliencyFineGrainedTrait for PtrOfStaticSaliencyFineGrained {
		#[inline] fn as_raw_mut_StaticSaliencyFineGrained(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfStaticSaliencyFineGrained {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfStaticSaliencyFineGrained {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::SaliencyConst for PtrOfStaticSaliencyFineGrained {
		#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::Saliency for PtrOfStaticSaliencyFineGrained {
		#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::StaticSaliencyConst for PtrOfStaticSaliencyFineGrained {
		#[inline] fn as_raw_StaticSaliency(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::StaticSaliency for PtrOfStaticSaliencyFineGrained {
		#[inline] fn as_raw_mut_StaticSaliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStaticSaliencySpectralResidual = core::Ptr<crate::saliency::StaticSaliencySpectralResidual>;
	
	ptr_extern! { crate::saliency::StaticSaliencySpectralResidual,
		cv_PtrOfStaticSaliencySpectralResidual_delete, cv_PtrOfStaticSaliencySpectralResidual_get_inner_ptr, cv_PtrOfStaticSaliencySpectralResidual_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::saliency::StaticSaliencySpectralResidual, cv_PtrOfStaticSaliencySpectralResidual_new }
	
	impl PtrOfStaticSaliencySpectralResidual {
		#[inline] pub fn as_raw_PtrOfStaticSaliencySpectralResidual(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStaticSaliencySpectralResidual(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::saliency::StaticSaliencySpectralResidualTraitConst for PtrOfStaticSaliencySpectralResidual {
		#[inline] fn as_raw_StaticSaliencySpectralResidual(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::StaticSaliencySpectralResidualTrait for PtrOfStaticSaliencySpectralResidual {
		#[inline] fn as_raw_mut_StaticSaliencySpectralResidual(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfStaticSaliencySpectralResidual {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfStaticSaliencySpectralResidual {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::SaliencyConst for PtrOfStaticSaliencySpectralResidual {
		#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::Saliency for PtrOfStaticSaliencySpectralResidual {
		#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::StaticSaliencyConst for PtrOfStaticSaliencySpectralResidual {
		#[inline] fn as_raw_StaticSaliency(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::StaticSaliency for PtrOfStaticSaliencySpectralResidual {
		#[inline] fn as_raw_mut_StaticSaliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_saliency)]
pub use saliency_types::*;

#[cfg(ocvrs_has_module_sfm)]
mod sfm_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfSFMLibmvEuclideanReconstruction = core::Ptr<dyn crate::sfm::SFMLibmvEuclideanReconstruction>;
	
	ptr_extern! { dyn crate::sfm::SFMLibmvEuclideanReconstruction,
		cv_PtrOfSFMLibmvEuclideanReconstruction_delete, cv_PtrOfSFMLibmvEuclideanReconstruction_get_inner_ptr, cv_PtrOfSFMLibmvEuclideanReconstruction_get_inner_ptr_mut
	}
	
	impl PtrOfSFMLibmvEuclideanReconstruction {
		#[inline] pub fn as_raw_PtrOfSFMLibmvEuclideanReconstruction(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSFMLibmvEuclideanReconstruction(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::sfm::SFMLibmvEuclideanReconstructionConst for PtrOfSFMLibmvEuclideanReconstruction {
		#[inline] fn as_raw_SFMLibmvEuclideanReconstruction(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::sfm::SFMLibmvEuclideanReconstruction for PtrOfSFMLibmvEuclideanReconstruction {
		#[inline] fn as_raw_mut_SFMLibmvEuclideanReconstruction(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::sfm::BaseSFMConst for PtrOfSFMLibmvEuclideanReconstruction {
		#[inline] fn as_raw_BaseSFM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::sfm::BaseSFM for PtrOfSFMLibmvEuclideanReconstruction {
		#[inline] fn as_raw_mut_BaseSFM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_sfm)]
pub use sfm_types::*;

#[cfg(ocvrs_has_module_shape)]
mod shape_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAffineTransformer = core::Ptr<dyn crate::shape::AffineTransformer>;
	
	ptr_extern! { dyn crate::shape::AffineTransformer,
		cv_PtrOfAffineTransformer_delete, cv_PtrOfAffineTransformer_get_inner_ptr, cv_PtrOfAffineTransformer_get_inner_ptr_mut
	}
	
	impl PtrOfAffineTransformer {
		#[inline] pub fn as_raw_PtrOfAffineTransformer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAffineTransformer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::AffineTransformerConst for PtrOfAffineTransformer {
		#[inline] fn as_raw_AffineTransformer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::AffineTransformer for PtrOfAffineTransformer {
		#[inline] fn as_raw_mut_AffineTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfAffineTransformer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfAffineTransformer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::ShapeTransformerConst for PtrOfAffineTransformer {
		#[inline] fn as_raw_ShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::ShapeTransformer for PtrOfAffineTransformer {
		#[inline] fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfAffineTransformer, core::Ptr<dyn crate::shape::ShapeTransformer>,
		cv_PtrOfAffineTransformer_to_PtrOfShapeTransformer,
	}
	
	pub type PtrOfChiHistogramCostExtractor = core::Ptr<dyn crate::shape::ChiHistogramCostExtractor>;
	
	ptr_extern! { dyn crate::shape::ChiHistogramCostExtractor,
		cv_PtrOfChiHistogramCostExtractor_delete, cv_PtrOfChiHistogramCostExtractor_get_inner_ptr, cv_PtrOfChiHistogramCostExtractor_get_inner_ptr_mut
	}
	
	impl PtrOfChiHistogramCostExtractor {
		#[inline] pub fn as_raw_PtrOfChiHistogramCostExtractor(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfChiHistogramCostExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::ChiHistogramCostExtractorConst for PtrOfChiHistogramCostExtractor {
		#[inline] fn as_raw_ChiHistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::ChiHistogramCostExtractor for PtrOfChiHistogramCostExtractor {
		#[inline] fn as_raw_mut_ChiHistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfChiHistogramCostExtractor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfChiHistogramCostExtractor {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::HistogramCostExtractorConst for PtrOfChiHistogramCostExtractor {
		#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::HistogramCostExtractor for PtrOfChiHistogramCostExtractor {
		#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfChiHistogramCostExtractor, core::Ptr<dyn crate::shape::HistogramCostExtractor>,
		cv_PtrOfChiHistogramCostExtractor_to_PtrOfHistogramCostExtractor,
	}
	
	pub type PtrOfEMDHistogramCostExtractor = core::Ptr<dyn crate::shape::EMDHistogramCostExtractor>;
	
	ptr_extern! { dyn crate::shape::EMDHistogramCostExtractor,
		cv_PtrOfEMDHistogramCostExtractor_delete, cv_PtrOfEMDHistogramCostExtractor_get_inner_ptr, cv_PtrOfEMDHistogramCostExtractor_get_inner_ptr_mut
	}
	
	impl PtrOfEMDHistogramCostExtractor {
		#[inline] pub fn as_raw_PtrOfEMDHistogramCostExtractor(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEMDHistogramCostExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::EMDHistogramCostExtractorConst for PtrOfEMDHistogramCostExtractor {
		#[inline] fn as_raw_EMDHistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::EMDHistogramCostExtractor for PtrOfEMDHistogramCostExtractor {
		#[inline] fn as_raw_mut_EMDHistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfEMDHistogramCostExtractor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfEMDHistogramCostExtractor {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::HistogramCostExtractorConst for PtrOfEMDHistogramCostExtractor {
		#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::HistogramCostExtractor for PtrOfEMDHistogramCostExtractor {
		#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfEMDHistogramCostExtractor, core::Ptr<dyn crate::shape::HistogramCostExtractor>,
		cv_PtrOfEMDHistogramCostExtractor_to_PtrOfHistogramCostExtractor,
	}
	
	pub type PtrOfEMDL1HistogramCostExtractor = core::Ptr<dyn crate::shape::EMDL1HistogramCostExtractor>;
	
	ptr_extern! { dyn crate::shape::EMDL1HistogramCostExtractor,
		cv_PtrOfEMDL1HistogramCostExtractor_delete, cv_PtrOfEMDL1HistogramCostExtractor_get_inner_ptr, cv_PtrOfEMDL1HistogramCostExtractor_get_inner_ptr_mut
	}
	
	impl PtrOfEMDL1HistogramCostExtractor {
		#[inline] pub fn as_raw_PtrOfEMDL1HistogramCostExtractor(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEMDL1HistogramCostExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::EMDL1HistogramCostExtractorConst for PtrOfEMDL1HistogramCostExtractor {
		#[inline] fn as_raw_EMDL1HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::EMDL1HistogramCostExtractor for PtrOfEMDL1HistogramCostExtractor {
		#[inline] fn as_raw_mut_EMDL1HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfEMDL1HistogramCostExtractor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfEMDL1HistogramCostExtractor {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::HistogramCostExtractorConst for PtrOfEMDL1HistogramCostExtractor {
		#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::HistogramCostExtractor for PtrOfEMDL1HistogramCostExtractor {
		#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfEMDL1HistogramCostExtractor, core::Ptr<dyn crate::shape::HistogramCostExtractor>,
		cv_PtrOfEMDL1HistogramCostExtractor_to_PtrOfHistogramCostExtractor,
	}
	
	pub type PtrOfHausdorffDistanceExtractor = core::Ptr<dyn crate::shape::HausdorffDistanceExtractor>;
	
	ptr_extern! { dyn crate::shape::HausdorffDistanceExtractor,
		cv_PtrOfHausdorffDistanceExtractor_delete, cv_PtrOfHausdorffDistanceExtractor_get_inner_ptr, cv_PtrOfHausdorffDistanceExtractor_get_inner_ptr_mut
	}
	
	impl PtrOfHausdorffDistanceExtractor {
		#[inline] pub fn as_raw_PtrOfHausdorffDistanceExtractor(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHausdorffDistanceExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::HausdorffDistanceExtractorConst for PtrOfHausdorffDistanceExtractor {
		#[inline] fn as_raw_HausdorffDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::HausdorffDistanceExtractor for PtrOfHausdorffDistanceExtractor {
		#[inline] fn as_raw_mut_HausdorffDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfHausdorffDistanceExtractor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfHausdorffDistanceExtractor {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::ShapeDistanceExtractorConst for PtrOfHausdorffDistanceExtractor {
		#[inline] fn as_raw_ShapeDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::ShapeDistanceExtractor for PtrOfHausdorffDistanceExtractor {
		#[inline] fn as_raw_mut_ShapeDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfHistogramCostExtractor = core::Ptr<dyn crate::shape::HistogramCostExtractor>;
	
	ptr_extern! { dyn crate::shape::HistogramCostExtractor,
		cv_PtrOfHistogramCostExtractor_delete, cv_PtrOfHistogramCostExtractor_get_inner_ptr, cv_PtrOfHistogramCostExtractor_get_inner_ptr_mut
	}
	
	impl PtrOfHistogramCostExtractor {
		#[inline] pub fn as_raw_PtrOfHistogramCostExtractor(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHistogramCostExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::HistogramCostExtractorConst for PtrOfHistogramCostExtractor {
		#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::HistogramCostExtractor for PtrOfHistogramCostExtractor {
		#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfHistogramCostExtractor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfHistogramCostExtractor {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfNormHistogramCostExtractor = core::Ptr<dyn crate::shape::NormHistogramCostExtractor>;
	
	ptr_extern! { dyn crate::shape::NormHistogramCostExtractor,
		cv_PtrOfNormHistogramCostExtractor_delete, cv_PtrOfNormHistogramCostExtractor_get_inner_ptr, cv_PtrOfNormHistogramCostExtractor_get_inner_ptr_mut
	}
	
	impl PtrOfNormHistogramCostExtractor {
		#[inline] pub fn as_raw_PtrOfNormHistogramCostExtractor(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNormHistogramCostExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::NormHistogramCostExtractorConst for PtrOfNormHistogramCostExtractor {
		#[inline] fn as_raw_NormHistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::NormHistogramCostExtractor for PtrOfNormHistogramCostExtractor {
		#[inline] fn as_raw_mut_NormHistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfNormHistogramCostExtractor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfNormHistogramCostExtractor {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::HistogramCostExtractorConst for PtrOfNormHistogramCostExtractor {
		#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::HistogramCostExtractor for PtrOfNormHistogramCostExtractor {
		#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfNormHistogramCostExtractor, core::Ptr<dyn crate::shape::HistogramCostExtractor>,
		cv_PtrOfNormHistogramCostExtractor_to_PtrOfHistogramCostExtractor,
	}
	
	pub type PtrOfShapeContextDistanceExtractor = core::Ptr<dyn crate::shape::ShapeContextDistanceExtractor>;
	
	ptr_extern! { dyn crate::shape::ShapeContextDistanceExtractor,
		cv_PtrOfShapeContextDistanceExtractor_delete, cv_PtrOfShapeContextDistanceExtractor_get_inner_ptr, cv_PtrOfShapeContextDistanceExtractor_get_inner_ptr_mut
	}
	
	impl PtrOfShapeContextDistanceExtractor {
		#[inline] pub fn as_raw_PtrOfShapeContextDistanceExtractor(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfShapeContextDistanceExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::ShapeContextDistanceExtractorConst for PtrOfShapeContextDistanceExtractor {
		#[inline] fn as_raw_ShapeContextDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::ShapeContextDistanceExtractor for PtrOfShapeContextDistanceExtractor {
		#[inline] fn as_raw_mut_ShapeContextDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfShapeContextDistanceExtractor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfShapeContextDistanceExtractor {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::ShapeDistanceExtractorConst for PtrOfShapeContextDistanceExtractor {
		#[inline] fn as_raw_ShapeDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::ShapeDistanceExtractor for PtrOfShapeContextDistanceExtractor {
		#[inline] fn as_raw_mut_ShapeDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfShapeTransformer = core::Ptr<dyn crate::shape::ShapeTransformer>;
	
	ptr_extern! { dyn crate::shape::ShapeTransformer,
		cv_PtrOfShapeTransformer_delete, cv_PtrOfShapeTransformer_get_inner_ptr, cv_PtrOfShapeTransformer_get_inner_ptr_mut
	}
	
	impl PtrOfShapeTransformer {
		#[inline] pub fn as_raw_PtrOfShapeTransformer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfShapeTransformer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::ShapeTransformerConst for PtrOfShapeTransformer {
		#[inline] fn as_raw_ShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::ShapeTransformer for PtrOfShapeTransformer {
		#[inline] fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfShapeTransformer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfShapeTransformer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfThinPlateSplineShapeTransformer = core::Ptr<dyn crate::shape::ThinPlateSplineShapeTransformer>;
	
	ptr_extern! { dyn crate::shape::ThinPlateSplineShapeTransformer,
		cv_PtrOfThinPlateSplineShapeTransformer_delete, cv_PtrOfThinPlateSplineShapeTransformer_get_inner_ptr, cv_PtrOfThinPlateSplineShapeTransformer_get_inner_ptr_mut
	}
	
	impl PtrOfThinPlateSplineShapeTransformer {
		#[inline] pub fn as_raw_PtrOfThinPlateSplineShapeTransformer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfThinPlateSplineShapeTransformer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::shape::ThinPlateSplineShapeTransformerConst for PtrOfThinPlateSplineShapeTransformer {
		#[inline] fn as_raw_ThinPlateSplineShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::ThinPlateSplineShapeTransformer for PtrOfThinPlateSplineShapeTransformer {
		#[inline] fn as_raw_mut_ThinPlateSplineShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfThinPlateSplineShapeTransformer {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfThinPlateSplineShapeTransformer {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::ShapeTransformerConst for PtrOfThinPlateSplineShapeTransformer {
		#[inline] fn as_raw_ShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::ShapeTransformer for PtrOfThinPlateSplineShapeTransformer {
		#[inline] fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfThinPlateSplineShapeTransformer, core::Ptr<dyn crate::shape::ShapeTransformer>,
		cv_PtrOfThinPlateSplineShapeTransformer_to_PtrOfShapeTransformer,
	}
	
}
#[cfg(ocvrs_has_module_shape)]
pub use shape_types::*;

#[cfg(ocvrs_has_module_stereo)]
mod stereo_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfQuasiDenseStereo = core::Ptr<dyn crate::stereo::QuasiDenseStereo>;
	
	ptr_extern! { dyn crate::stereo::QuasiDenseStereo,
		cv_PtrOfQuasiDenseStereo_delete, cv_PtrOfQuasiDenseStereo_get_inner_ptr, cv_PtrOfQuasiDenseStereo_get_inner_ptr_mut
	}
	
	impl PtrOfQuasiDenseStereo {
		#[inline] pub fn as_raw_PtrOfQuasiDenseStereo(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfQuasiDenseStereo(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stereo::QuasiDenseStereoConst for PtrOfQuasiDenseStereo {
		#[inline] fn as_raw_QuasiDenseStereo(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stereo::QuasiDenseStereo for PtrOfQuasiDenseStereo {
		#[inline] fn as_raw_mut_QuasiDenseStereo(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfMatchQuasiDense = core::Vector<crate::stereo::MatchQuasiDense>;
	
	impl VectorOfMatchQuasiDense {
		pub fn as_raw_VectorOfMatchQuasiDense(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfMatchQuasiDense(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::stereo::MatchQuasiDense, *const c_void, *mut c_void,
		cv_VectorOfMatchQuasiDense_new, cv_VectorOfMatchQuasiDense_delete,
		cv_VectorOfMatchQuasiDense_len, cv_VectorOfMatchQuasiDense_is_empty,
		cv_VectorOfMatchQuasiDense_capacity, cv_VectorOfMatchQuasiDense_shrink_to_fit,
		cv_VectorOfMatchQuasiDense_reserve, cv_VectorOfMatchQuasiDense_remove,
		cv_VectorOfMatchQuasiDense_swap, cv_VectorOfMatchQuasiDense_clear,
		cv_VectorOfMatchQuasiDense_get, cv_VectorOfMatchQuasiDense_set,
		cv_VectorOfMatchQuasiDense_push, cv_VectorOfMatchQuasiDense_insert,
	}
	vector_copy_non_bool! { crate::stereo::MatchQuasiDense, *const c_void, *mut c_void,
		cv_VectorOfMatchQuasiDense_data, cv_VectorOfMatchQuasiDense_data_mut, cv_VectorOfMatchQuasiDense_from_slice,
		cv_VectorOfMatchQuasiDense_clone,
	}
	
	unsafe impl Send for core::Vector<crate::stereo::MatchQuasiDense> {}
	
}
#[cfg(ocvrs_has_module_stereo)]
pub use stereo_types::*;

#[cfg(ocvrs_has_module_stitching)]
mod stitching_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAffineWarper = core::Ptr<crate::stitching::AffineWarper>;
	
	ptr_extern! { crate::stitching::AffineWarper,
		cv_PtrOfAffineWarper_delete, cv_PtrOfAffineWarper_get_inner_ptr, cv_PtrOfAffineWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::AffineWarper, cv_PtrOfAffineWarper_new }
	
	impl PtrOfAffineWarper {
		#[inline] pub fn as_raw_PtrOfAffineWarper(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAffineWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::AffineWarperTraitConst for PtrOfAffineWarper {
		#[inline] fn as_raw_AffineWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::AffineWarperTrait for PtrOfAffineWarper {
		#[inline] fn as_raw_mut_AffineWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for PtrOfAffineWarper {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for PtrOfAffineWarper {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfAffineWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
		cv_PtrOfAffineWarper_to_PtrOfWarperCreator,
	}
	
	pub type PtrOfCompressedRectilinearPortraitWarper = core::Ptr<crate::stitching::CompressedRectilinearPortraitWarper>;
	
	ptr_extern! { crate::stitching::CompressedRectilinearPortraitWarper,
		cv_PtrOfCompressedRectilinearPortraitWarper_delete, cv_PtrOfCompressedRectilinearPortraitWarper_get_inner_ptr, cv_PtrOfCompressedRectilinearPortraitWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::CompressedRectilinearPortraitWarper, cv_PtrOfCompressedRectilinearPortraitWarper_new }
	
	impl PtrOfCompressedRectilinearPortraitWarper {
		#[inline] pub fn as_raw_PtrOfCompressedRectilinearPortraitWarper(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCompressedRectilinearPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::CompressedRectilinearPortraitWarperTraitConst for PtrOfCompressedRectilinearPortraitWarper {
		#[inline] fn as_raw_CompressedRectilinearPortraitWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::CompressedRectilinearPortraitWarperTrait for PtrOfCompressedRectilinearPortraitWarper {
		#[inline] fn as_raw_mut_CompressedRectilinearPortraitWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for PtrOfCompressedRectilinearPortraitWarper {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for PtrOfCompressedRectilinearPortraitWarper {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfCompressedRectilinearPortraitWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
		cv_PtrOfCompressedRectilinearPortraitWarper_to_PtrOfWarperCreator,
	}
	
	pub type PtrOfCompressedRectilinearWarper = core::Ptr<crate::stitching::CompressedRectilinearWarper>;
	
	ptr_extern! { crate::stitching::CompressedRectilinearWarper,
		cv_PtrOfCompressedRectilinearWarper_delete, cv_PtrOfCompressedRectilinearWarper_get_inner_ptr, cv_PtrOfCompressedRectilinearWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::CompressedRectilinearWarper, cv_PtrOfCompressedRectilinearWarper_new }
	
	impl PtrOfCompressedRectilinearWarper {
		#[inline] pub fn as_raw_PtrOfCompressedRectilinearWarper(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCompressedRectilinearWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::CompressedRectilinearWarperTraitConst for PtrOfCompressedRectilinearWarper {
		#[inline] fn as_raw_CompressedRectilinearWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::CompressedRectilinearWarperTrait for PtrOfCompressedRectilinearWarper {
		#[inline] fn as_raw_mut_CompressedRectilinearWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for PtrOfCompressedRectilinearWarper {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for PtrOfCompressedRectilinearWarper {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfCompressedRectilinearWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
		cv_PtrOfCompressedRectilinearWarper_to_PtrOfWarperCreator,
	}
	
	pub type PtrOfCylindricalWarper = core::Ptr<crate::stitching::CylindricalWarper>;
	
	ptr_extern! { crate::stitching::CylindricalWarper,
		cv_PtrOfCylindricalWarper_delete, cv_PtrOfCylindricalWarper_get_inner_ptr, cv_PtrOfCylindricalWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::CylindricalWarper, cv_PtrOfCylindricalWarper_new }
	
	impl PtrOfCylindricalWarper {
		#[inline] pub fn as_raw_PtrOfCylindricalWarper(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCylindricalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::CylindricalWarperTraitConst for PtrOfCylindricalWarper {
		#[inline] fn as_raw_CylindricalWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::CylindricalWarperTrait for PtrOfCylindricalWarper {
		#[inline] fn as_raw_mut_CylindricalWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for PtrOfCylindricalWarper {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for PtrOfCylindricalWarper {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfCylindricalWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
		cv_PtrOfCylindricalWarper_to_PtrOfWarperCreator,
	}
	
	pub type PtrOfCylindricalWarperGpu = core::Ptr<crate::stitching::CylindricalWarperGpu>;
	
	ptr_extern! { crate::stitching::CylindricalWarperGpu,
		cv_PtrOfCylindricalWarperGpu_delete, cv_PtrOfCylindricalWarperGpu_get_inner_ptr, cv_PtrOfCylindricalWarperGpu_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::CylindricalWarperGpu, cv_PtrOfCylindricalWarperGpu_new }
	
	impl PtrOfCylindricalWarperGpu {
		#[inline] pub fn as_raw_PtrOfCylindricalWarperGpu(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCylindricalWarperGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::CylindricalWarperGpuTraitConst for PtrOfCylindricalWarperGpu {
		#[inline] fn as_raw_CylindricalWarperGpu(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::CylindricalWarperGpuTrait for PtrOfCylindricalWarperGpu {
		#[inline] fn as_raw_mut_CylindricalWarperGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for PtrOfCylindricalWarperGpu {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for PtrOfCylindricalWarperGpu {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfCylindricalWarperGpu, core::Ptr<dyn crate::stitching::WarperCreator>,
		cv_PtrOfCylindricalWarperGpu_to_PtrOfWarperCreator,
	}
	
	pub type PtrOfDetail_AffineBasedEstimator = core::Ptr<crate::stitching::Detail_AffineBasedEstimator>;
	
	ptr_extern! { crate::stitching::Detail_AffineBasedEstimator,
		cv_PtrOfDetail_AffineBasedEstimator_delete, cv_PtrOfDetail_AffineBasedEstimator_get_inner_ptr, cv_PtrOfDetail_AffineBasedEstimator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_AffineBasedEstimator, cv_PtrOfDetail_AffineBasedEstimator_new }
	
	impl PtrOfDetail_AffineBasedEstimator {
		#[inline] pub fn as_raw_PtrOfDetail_AffineBasedEstimator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_AffineBasedEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_AffineBasedEstimatorTraitConst for PtrOfDetail_AffineBasedEstimator {
		#[inline] fn as_raw_Detail_AffineBasedEstimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_AffineBasedEstimatorTrait for PtrOfDetail_AffineBasedEstimator {
		#[inline] fn as_raw_mut_Detail_AffineBasedEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_EstimatorConst for PtrOfDetail_AffineBasedEstimator {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_Estimator for PtrOfDetail_AffineBasedEstimator {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_AffineBasedEstimator, core::Ptr<dyn crate::stitching::Detail_Estimator>,
		cv_PtrOfDetail_AffineBasedEstimator_to_PtrOfDetail_Estimator,
	}
	
	pub type PtrOfDetail_BestOf2NearestMatcher = core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher>;
	
	ptr_extern! { crate::stitching::Detail_BestOf2NearestMatcher,
		cv_PtrOfDetail_BestOf2NearestMatcher_delete, cv_PtrOfDetail_BestOf2NearestMatcher_get_inner_ptr, cv_PtrOfDetail_BestOf2NearestMatcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_BestOf2NearestMatcher, cv_PtrOfDetail_BestOf2NearestMatcher_new }
	
	impl PtrOfDetail_BestOf2NearestMatcher {
		#[inline] pub fn as_raw_PtrOfDetail_BestOf2NearestMatcher(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BestOf2NearestMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BestOf2NearestMatcherTraitConst for PtrOfDetail_BestOf2NearestMatcher {
		#[inline] fn as_raw_Detail_BestOf2NearestMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BestOf2NearestMatcherTrait for PtrOfDetail_BestOf2NearestMatcher {
		#[inline] fn as_raw_mut_Detail_BestOf2NearestMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_FeaturesMatcherConst for PtrOfDetail_BestOf2NearestMatcher {
		#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_FeaturesMatcher for PtrOfDetail_BestOf2NearestMatcher {
		#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BestOf2NearestMatcher, core::Ptr<dyn crate::stitching::Detail_FeaturesMatcher>,
		cv_PtrOfDetail_BestOf2NearestMatcher_to_PtrOfDetail_FeaturesMatcher,
	}
	
	pub type PtrOfDetail_Blender = core::Ptr<crate::stitching::Detail_Blender>;
	
	ptr_extern! { crate::stitching::Detail_Blender,
		cv_PtrOfDetail_Blender_delete, cv_PtrOfDetail_Blender_get_inner_ptr, cv_PtrOfDetail_Blender_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_Blender, cv_PtrOfDetail_Blender_new }
	
	impl PtrOfDetail_Blender {
		#[inline] pub fn as_raw_PtrOfDetail_Blender(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_Blender(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlenderTraitConst for PtrOfDetail_Blender {
		#[inline] fn as_raw_Detail_Blender(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BlenderTrait for PtrOfDetail_Blender {
		#[inline] fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_BlocksCompensator = core::Ptr<dyn crate::stitching::Detail_BlocksCompensator>;
	
	ptr_extern! { dyn crate::stitching::Detail_BlocksCompensator,
		cv_PtrOfDetail_BlocksCompensator_delete, cv_PtrOfDetail_BlocksCompensator_get_inner_ptr, cv_PtrOfDetail_BlocksCompensator_get_inner_ptr_mut
	}
	
	impl PtrOfDetail_BlocksCompensator {
		#[inline] pub fn as_raw_PtrOfDetail_BlocksCompensator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BlocksCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlocksCompensatorConst for PtrOfDetail_BlocksCompensator {
		#[inline] fn as_raw_Detail_BlocksCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BlocksCompensator for PtrOfDetail_BlocksCompensator {
		#[inline] fn as_raw_mut_Detail_BlocksCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorConst for PtrOfDetail_BlocksCompensator {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensator for PtrOfDetail_BlocksCompensator {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BlocksCompensator, core::Ptr<dyn crate::stitching::Detail_ExposureCompensator>,
		cv_PtrOfDetail_BlocksCompensator_to_PtrOfDetail_ExposureCompensator,
	}
	
	pub type PtrOfDetail_BundleAdjusterAffine = core::Ptr<crate::stitching::Detail_BundleAdjusterAffine>;
	
	ptr_extern! { crate::stitching::Detail_BundleAdjusterAffine,
		cv_PtrOfDetail_BundleAdjusterAffine_delete, cv_PtrOfDetail_BundleAdjusterAffine_get_inner_ptr, cv_PtrOfDetail_BundleAdjusterAffine_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_BundleAdjusterAffine, cv_PtrOfDetail_BundleAdjusterAffine_new }
	
	impl PtrOfDetail_BundleAdjusterAffine {
		#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterAffine(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterAffine(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterAffineTraitConst for PtrOfDetail_BundleAdjusterAffine {
		#[inline] fn as_raw_Detail_BundleAdjusterAffine(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterAffineTrait for PtrOfDetail_BundleAdjusterAffine {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterAffine(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseConst for PtrOfDetail_BundleAdjusterAffine {
		#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBase for PtrOfDetail_BundleAdjusterAffine {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterAffine, core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase>,
		cv_PtrOfDetail_BundleAdjusterAffine_to_PtrOfDetail_BundleAdjusterBase,
	}
	
	impl crate::stitching::Detail_EstimatorConst for PtrOfDetail_BundleAdjusterAffine {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_Estimator for PtrOfDetail_BundleAdjusterAffine {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterAffine, core::Ptr<dyn crate::stitching::Detail_Estimator>,
		cv_PtrOfDetail_BundleAdjusterAffine_to_PtrOfDetail_Estimator,
	}
	
	pub type PtrOfDetail_BundleAdjusterAffinePartial = core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial>;
	
	ptr_extern! { crate::stitching::Detail_BundleAdjusterAffinePartial,
		cv_PtrOfDetail_BundleAdjusterAffinePartial_delete, cv_PtrOfDetail_BundleAdjusterAffinePartial_get_inner_ptr, cv_PtrOfDetail_BundleAdjusterAffinePartial_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_BundleAdjusterAffinePartial, cv_PtrOfDetail_BundleAdjusterAffinePartial_new }
	
	impl PtrOfDetail_BundleAdjusterAffinePartial {
		#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterAffinePartial(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterAffinePartial(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterAffinePartialTraitConst for PtrOfDetail_BundleAdjusterAffinePartial {
		#[inline] fn as_raw_Detail_BundleAdjusterAffinePartial(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterAffinePartialTrait for PtrOfDetail_BundleAdjusterAffinePartial {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterAffinePartial(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseConst for PtrOfDetail_BundleAdjusterAffinePartial {
		#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBase for PtrOfDetail_BundleAdjusterAffinePartial {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterAffinePartial, core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase>,
		cv_PtrOfDetail_BundleAdjusterAffinePartial_to_PtrOfDetail_BundleAdjusterBase,
	}
	
	impl crate::stitching::Detail_EstimatorConst for PtrOfDetail_BundleAdjusterAffinePartial {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_Estimator for PtrOfDetail_BundleAdjusterAffinePartial {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterAffinePartial, core::Ptr<dyn crate::stitching::Detail_Estimator>,
		cv_PtrOfDetail_BundleAdjusterAffinePartial_to_PtrOfDetail_Estimator,
	}
	
	pub type PtrOfDetail_BundleAdjusterBase = core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase>;
	
	ptr_extern! { dyn crate::stitching::Detail_BundleAdjusterBase,
		cv_PtrOfDetail_BundleAdjusterBase_delete, cv_PtrOfDetail_BundleAdjusterBase_get_inner_ptr, cv_PtrOfDetail_BundleAdjusterBase_get_inner_ptr_mut
	}
	
	impl PtrOfDetail_BundleAdjusterBase {
		#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterBase(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseConst for PtrOfDetail_BundleAdjusterBase {
		#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBase for PtrOfDetail_BundleAdjusterBase {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_EstimatorConst for PtrOfDetail_BundleAdjusterBase {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_Estimator for PtrOfDetail_BundleAdjusterBase {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterBase, core::Ptr<dyn crate::stitching::Detail_Estimator>,
		cv_PtrOfDetail_BundleAdjusterBase_to_PtrOfDetail_Estimator,
	}
	
	pub type PtrOfDetail_BundleAdjusterRay = core::Ptr<crate::stitching::Detail_BundleAdjusterRay>;
	
	ptr_extern! { crate::stitching::Detail_BundleAdjusterRay,
		cv_PtrOfDetail_BundleAdjusterRay_delete, cv_PtrOfDetail_BundleAdjusterRay_get_inner_ptr, cv_PtrOfDetail_BundleAdjusterRay_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_BundleAdjusterRay, cv_PtrOfDetail_BundleAdjusterRay_new }
	
	impl PtrOfDetail_BundleAdjusterRay {
		#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterRay(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterRay(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterRayTraitConst for PtrOfDetail_BundleAdjusterRay {
		#[inline] fn as_raw_Detail_BundleAdjusterRay(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterRayTrait for PtrOfDetail_BundleAdjusterRay {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterRay(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseConst for PtrOfDetail_BundleAdjusterRay {
		#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBase for PtrOfDetail_BundleAdjusterRay {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterRay, core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase>,
		cv_PtrOfDetail_BundleAdjusterRay_to_PtrOfDetail_BundleAdjusterBase,
	}
	
	impl crate::stitching::Detail_EstimatorConst for PtrOfDetail_BundleAdjusterRay {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_Estimator for PtrOfDetail_BundleAdjusterRay {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterRay, core::Ptr<dyn crate::stitching::Detail_Estimator>,
		cv_PtrOfDetail_BundleAdjusterRay_to_PtrOfDetail_Estimator,
	}
	
	pub type PtrOfDetail_BundleAdjusterReproj = core::Ptr<crate::stitching::Detail_BundleAdjusterReproj>;
	
	ptr_extern! { crate::stitching::Detail_BundleAdjusterReproj,
		cv_PtrOfDetail_BundleAdjusterReproj_delete, cv_PtrOfDetail_BundleAdjusterReproj_get_inner_ptr, cv_PtrOfDetail_BundleAdjusterReproj_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_BundleAdjusterReproj, cv_PtrOfDetail_BundleAdjusterReproj_new }
	
	impl PtrOfDetail_BundleAdjusterReproj {
		#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterReproj(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterReproj(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterReprojTraitConst for PtrOfDetail_BundleAdjusterReproj {
		#[inline] fn as_raw_Detail_BundleAdjusterReproj(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterReprojTrait for PtrOfDetail_BundleAdjusterReproj {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterReproj(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseConst for PtrOfDetail_BundleAdjusterReproj {
		#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBase for PtrOfDetail_BundleAdjusterReproj {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterReproj, core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase>,
		cv_PtrOfDetail_BundleAdjusterReproj_to_PtrOfDetail_BundleAdjusterBase,
	}
	
	impl crate::stitching::Detail_EstimatorConst for PtrOfDetail_BundleAdjusterReproj {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_Estimator for PtrOfDetail_BundleAdjusterReproj {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterReproj, core::Ptr<dyn crate::stitching::Detail_Estimator>,
		cv_PtrOfDetail_BundleAdjusterReproj_to_PtrOfDetail_Estimator,
	}
	
	pub type PtrOfDetail_ChannelsCompensator = core::Ptr<crate::stitching::Detail_ChannelsCompensator>;
	
	ptr_extern! { crate::stitching::Detail_ChannelsCompensator,
		cv_PtrOfDetail_ChannelsCompensator_delete, cv_PtrOfDetail_ChannelsCompensator_get_inner_ptr, cv_PtrOfDetail_ChannelsCompensator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_ChannelsCompensator, cv_PtrOfDetail_ChannelsCompensator_new }
	
	impl PtrOfDetail_ChannelsCompensator {
		#[inline] pub fn as_raw_PtrOfDetail_ChannelsCompensator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_ChannelsCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ChannelsCompensatorTraitConst for PtrOfDetail_ChannelsCompensator {
		#[inline] fn as_raw_Detail_ChannelsCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_ChannelsCompensatorTrait for PtrOfDetail_ChannelsCompensator {
		#[inline] fn as_raw_mut_Detail_ChannelsCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorConst for PtrOfDetail_ChannelsCompensator {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensator for PtrOfDetail_ChannelsCompensator {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_ChannelsCompensator, core::Ptr<dyn crate::stitching::Detail_ExposureCompensator>,
		cv_PtrOfDetail_ChannelsCompensator_to_PtrOfDetail_ExposureCompensator,
	}
	
	pub type PtrOfDetail_DpSeamFinder = core::Ptr<crate::stitching::Detail_DpSeamFinder>;
	
	ptr_extern! { crate::stitching::Detail_DpSeamFinder,
		cv_PtrOfDetail_DpSeamFinder_delete, cv_PtrOfDetail_DpSeamFinder_get_inner_ptr, cv_PtrOfDetail_DpSeamFinder_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_DpSeamFinder, cv_PtrOfDetail_DpSeamFinder_new }
	
	impl PtrOfDetail_DpSeamFinder {
		#[inline] pub fn as_raw_PtrOfDetail_DpSeamFinder(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_DpSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_DpSeamFinderTraitConst for PtrOfDetail_DpSeamFinder {
		#[inline] fn as_raw_Detail_DpSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_DpSeamFinderTrait for PtrOfDetail_DpSeamFinder {
		#[inline] fn as_raw_mut_Detail_DpSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_SeamFinderConst for PtrOfDetail_DpSeamFinder {
		#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_SeamFinder for PtrOfDetail_DpSeamFinder {
		#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_DpSeamFinder, core::Ptr<dyn crate::stitching::Detail_SeamFinder>,
		cv_PtrOfDetail_DpSeamFinder_to_PtrOfDetail_SeamFinder,
	}
	
	pub type PtrOfDetail_Estimator = core::Ptr<dyn crate::stitching::Detail_Estimator>;
	
	ptr_extern! { dyn crate::stitching::Detail_Estimator,
		cv_PtrOfDetail_Estimator_delete, cv_PtrOfDetail_Estimator_get_inner_ptr, cv_PtrOfDetail_Estimator_get_inner_ptr_mut
	}
	
	impl PtrOfDetail_Estimator {
		#[inline] pub fn as_raw_PtrOfDetail_Estimator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_Estimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_EstimatorConst for PtrOfDetail_Estimator {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_Estimator for PtrOfDetail_Estimator {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_ExposureCompensator = core::Ptr<dyn crate::stitching::Detail_ExposureCompensator>;
	
	ptr_extern! { dyn crate::stitching::Detail_ExposureCompensator,
		cv_PtrOfDetail_ExposureCompensator_delete, cv_PtrOfDetail_ExposureCompensator_get_inner_ptr, cv_PtrOfDetail_ExposureCompensator_get_inner_ptr_mut
	}
	
	impl PtrOfDetail_ExposureCompensator {
		#[inline] pub fn as_raw_PtrOfDetail_ExposureCompensator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_ExposureCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorConst for PtrOfDetail_ExposureCompensator {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensator for PtrOfDetail_ExposureCompensator {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_FeatherBlender = core::Ptr<crate::stitching::Detail_FeatherBlender>;
	
	ptr_extern! { crate::stitching::Detail_FeatherBlender,
		cv_PtrOfDetail_FeatherBlender_delete, cv_PtrOfDetail_FeatherBlender_get_inner_ptr, cv_PtrOfDetail_FeatherBlender_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_FeatherBlender, cv_PtrOfDetail_FeatherBlender_new }
	
	impl PtrOfDetail_FeatherBlender {
		#[inline] pub fn as_raw_PtrOfDetail_FeatherBlender(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_FeatherBlender(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_FeatherBlenderTraitConst for PtrOfDetail_FeatherBlender {
		#[inline] fn as_raw_Detail_FeatherBlender(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_FeatherBlenderTrait for PtrOfDetail_FeatherBlender {
		#[inline] fn as_raw_mut_Detail_FeatherBlender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlenderTraitConst for PtrOfDetail_FeatherBlender {
		#[inline] fn as_raw_Detail_Blender(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BlenderTrait for PtrOfDetail_FeatherBlender {
		#[inline] fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_FeatherBlender, core::Ptr<crate::stitching::Detail_Blender>,
		cv_PtrOfDetail_FeatherBlender_to_PtrOfDetail_Blender,
	}
	
	pub type PtrOfDetail_FeaturesMatcher = core::Ptr<dyn crate::stitching::Detail_FeaturesMatcher>;
	
	ptr_extern! { dyn crate::stitching::Detail_FeaturesMatcher,
		cv_PtrOfDetail_FeaturesMatcher_delete, cv_PtrOfDetail_FeaturesMatcher_get_inner_ptr, cv_PtrOfDetail_FeaturesMatcher_get_inner_ptr_mut
	}
	
	impl PtrOfDetail_FeaturesMatcher {
		#[inline] pub fn as_raw_PtrOfDetail_FeaturesMatcher(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_FeaturesMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_FeaturesMatcherConst for PtrOfDetail_FeaturesMatcher {
		#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_FeaturesMatcher for PtrOfDetail_FeaturesMatcher {
		#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_GainCompensator = core::Ptr<crate::stitching::Detail_GainCompensator>;
	
	ptr_extern! { crate::stitching::Detail_GainCompensator,
		cv_PtrOfDetail_GainCompensator_delete, cv_PtrOfDetail_GainCompensator_get_inner_ptr, cv_PtrOfDetail_GainCompensator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_GainCompensator, cv_PtrOfDetail_GainCompensator_new }
	
	impl PtrOfDetail_GainCompensator {
		#[inline] pub fn as_raw_PtrOfDetail_GainCompensator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_GainCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_GainCompensatorTraitConst for PtrOfDetail_GainCompensator {
		#[inline] fn as_raw_Detail_GainCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_GainCompensatorTrait for PtrOfDetail_GainCompensator {
		#[inline] fn as_raw_mut_Detail_GainCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorConst for PtrOfDetail_GainCompensator {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensator for PtrOfDetail_GainCompensator {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_GainCompensator, core::Ptr<dyn crate::stitching::Detail_ExposureCompensator>,
		cv_PtrOfDetail_GainCompensator_to_PtrOfDetail_ExposureCompensator,
	}
	
	pub type PtrOfDetail_GraphCutSeamFinder = core::Ptr<crate::stitching::Detail_GraphCutSeamFinder>;
	
	ptr_extern! { crate::stitching::Detail_GraphCutSeamFinder,
		cv_PtrOfDetail_GraphCutSeamFinder_delete, cv_PtrOfDetail_GraphCutSeamFinder_get_inner_ptr, cv_PtrOfDetail_GraphCutSeamFinder_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_GraphCutSeamFinder, cv_PtrOfDetail_GraphCutSeamFinder_new }
	
	impl PtrOfDetail_GraphCutSeamFinder {
		#[inline] pub fn as_raw_PtrOfDetail_GraphCutSeamFinder(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_GraphCutSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_GraphCutSeamFinderTraitConst for PtrOfDetail_GraphCutSeamFinder {
		#[inline] fn as_raw_Detail_GraphCutSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_GraphCutSeamFinderTrait for PtrOfDetail_GraphCutSeamFinder {
		#[inline] fn as_raw_mut_Detail_GraphCutSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_GraphCutSeamFinderBaseTraitConst for PtrOfDetail_GraphCutSeamFinder {
		#[inline] fn as_raw_Detail_GraphCutSeamFinderBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_GraphCutSeamFinderBaseTrait for PtrOfDetail_GraphCutSeamFinder {
		#[inline] fn as_raw_mut_Detail_GraphCutSeamFinderBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_SeamFinderConst for PtrOfDetail_GraphCutSeamFinder {
		#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_SeamFinder for PtrOfDetail_GraphCutSeamFinder {
		#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_GraphCutSeamFinder, core::Ptr<dyn crate::stitching::Detail_SeamFinder>,
		cv_PtrOfDetail_GraphCutSeamFinder_to_PtrOfDetail_SeamFinder,
	}
	
	pub type PtrOfDetail_HomographyBasedEstimator = core::Ptr<crate::stitching::Detail_HomographyBasedEstimator>;
	
	ptr_extern! { crate::stitching::Detail_HomographyBasedEstimator,
		cv_PtrOfDetail_HomographyBasedEstimator_delete, cv_PtrOfDetail_HomographyBasedEstimator_get_inner_ptr, cv_PtrOfDetail_HomographyBasedEstimator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_HomographyBasedEstimator, cv_PtrOfDetail_HomographyBasedEstimator_new }
	
	impl PtrOfDetail_HomographyBasedEstimator {
		#[inline] pub fn as_raw_PtrOfDetail_HomographyBasedEstimator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_HomographyBasedEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_HomographyBasedEstimatorTraitConst for PtrOfDetail_HomographyBasedEstimator {
		#[inline] fn as_raw_Detail_HomographyBasedEstimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_HomographyBasedEstimatorTrait for PtrOfDetail_HomographyBasedEstimator {
		#[inline] fn as_raw_mut_Detail_HomographyBasedEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_EstimatorConst for PtrOfDetail_HomographyBasedEstimator {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_Estimator for PtrOfDetail_HomographyBasedEstimator {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_HomographyBasedEstimator, core::Ptr<dyn crate::stitching::Detail_Estimator>,
		cv_PtrOfDetail_HomographyBasedEstimator_to_PtrOfDetail_Estimator,
	}
	
	pub type PtrOfDetail_MultiBandBlender = core::Ptr<crate::stitching::Detail_MultiBandBlender>;
	
	ptr_extern! { crate::stitching::Detail_MultiBandBlender,
		cv_PtrOfDetail_MultiBandBlender_delete, cv_PtrOfDetail_MultiBandBlender_get_inner_ptr, cv_PtrOfDetail_MultiBandBlender_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_MultiBandBlender, cv_PtrOfDetail_MultiBandBlender_new }
	
	impl PtrOfDetail_MultiBandBlender {
		#[inline] pub fn as_raw_PtrOfDetail_MultiBandBlender(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_MultiBandBlender(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_MultiBandBlenderTraitConst for PtrOfDetail_MultiBandBlender {
		#[inline] fn as_raw_Detail_MultiBandBlender(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_MultiBandBlenderTrait for PtrOfDetail_MultiBandBlender {
		#[inline] fn as_raw_mut_Detail_MultiBandBlender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlenderTraitConst for PtrOfDetail_MultiBandBlender {
		#[inline] fn as_raw_Detail_Blender(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BlenderTrait for PtrOfDetail_MultiBandBlender {
		#[inline] fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_MultiBandBlender, core::Ptr<crate::stitching::Detail_Blender>,
		cv_PtrOfDetail_MultiBandBlender_to_PtrOfDetail_Blender,
	}
	
	pub type PtrOfDetail_NoBundleAdjuster = core::Ptr<crate::stitching::Detail_NoBundleAdjuster>;
	
	ptr_extern! { crate::stitching::Detail_NoBundleAdjuster,
		cv_PtrOfDetail_NoBundleAdjuster_delete, cv_PtrOfDetail_NoBundleAdjuster_get_inner_ptr, cv_PtrOfDetail_NoBundleAdjuster_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_NoBundleAdjuster, cv_PtrOfDetail_NoBundleAdjuster_new }
	
	impl PtrOfDetail_NoBundleAdjuster {
		#[inline] pub fn as_raw_PtrOfDetail_NoBundleAdjuster(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_NoBundleAdjuster(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_NoBundleAdjusterTraitConst for PtrOfDetail_NoBundleAdjuster {
		#[inline] fn as_raw_Detail_NoBundleAdjuster(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_NoBundleAdjusterTrait for PtrOfDetail_NoBundleAdjuster {
		#[inline] fn as_raw_mut_Detail_NoBundleAdjuster(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseConst for PtrOfDetail_NoBundleAdjuster {
		#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBase for PtrOfDetail_NoBundleAdjuster {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_NoBundleAdjuster, core::Ptr<dyn crate::stitching::Detail_BundleAdjusterBase>,
		cv_PtrOfDetail_NoBundleAdjuster_to_PtrOfDetail_BundleAdjusterBase,
	}
	
	impl crate::stitching::Detail_EstimatorConst for PtrOfDetail_NoBundleAdjuster {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_Estimator for PtrOfDetail_NoBundleAdjuster {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_NoBundleAdjuster, core::Ptr<dyn crate::stitching::Detail_Estimator>,
		cv_PtrOfDetail_NoBundleAdjuster_to_PtrOfDetail_Estimator,
	}
	
	pub type PtrOfDetail_NoExposureCompensator = core::Ptr<crate::stitching::Detail_NoExposureCompensator>;
	
	ptr_extern! { crate::stitching::Detail_NoExposureCompensator,
		cv_PtrOfDetail_NoExposureCompensator_delete, cv_PtrOfDetail_NoExposureCompensator_get_inner_ptr, cv_PtrOfDetail_NoExposureCompensator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_NoExposureCompensator, cv_PtrOfDetail_NoExposureCompensator_new }
	
	impl PtrOfDetail_NoExposureCompensator {
		#[inline] pub fn as_raw_PtrOfDetail_NoExposureCompensator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_NoExposureCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_NoExposureCompensatorTraitConst for PtrOfDetail_NoExposureCompensator {
		#[inline] fn as_raw_Detail_NoExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_NoExposureCompensatorTrait for PtrOfDetail_NoExposureCompensator {
		#[inline] fn as_raw_mut_Detail_NoExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorConst for PtrOfDetail_NoExposureCompensator {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensator for PtrOfDetail_NoExposureCompensator {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_NoExposureCompensator, core::Ptr<dyn crate::stitching::Detail_ExposureCompensator>,
		cv_PtrOfDetail_NoExposureCompensator_to_PtrOfDetail_ExposureCompensator,
	}
	
	pub type PtrOfDetail_NoSeamFinder = core::Ptr<crate::stitching::Detail_NoSeamFinder>;
	
	ptr_extern! { crate::stitching::Detail_NoSeamFinder,
		cv_PtrOfDetail_NoSeamFinder_delete, cv_PtrOfDetail_NoSeamFinder_get_inner_ptr, cv_PtrOfDetail_NoSeamFinder_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_NoSeamFinder, cv_PtrOfDetail_NoSeamFinder_new }
	
	impl PtrOfDetail_NoSeamFinder {
		#[inline] pub fn as_raw_PtrOfDetail_NoSeamFinder(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_NoSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_NoSeamFinderTraitConst for PtrOfDetail_NoSeamFinder {
		#[inline] fn as_raw_Detail_NoSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_NoSeamFinderTrait for PtrOfDetail_NoSeamFinder {
		#[inline] fn as_raw_mut_Detail_NoSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_SeamFinderConst for PtrOfDetail_NoSeamFinder {
		#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_SeamFinder for PtrOfDetail_NoSeamFinder {
		#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_NoSeamFinder, core::Ptr<dyn crate::stitching::Detail_SeamFinder>,
		cv_PtrOfDetail_NoSeamFinder_to_PtrOfDetail_SeamFinder,
	}
	
	pub type PtrOfDetail_PairwiseSeamFinder = core::Ptr<dyn crate::stitching::Detail_PairwiseSeamFinder>;
	
	ptr_extern! { dyn crate::stitching::Detail_PairwiseSeamFinder,
		cv_PtrOfDetail_PairwiseSeamFinder_delete, cv_PtrOfDetail_PairwiseSeamFinder_get_inner_ptr, cv_PtrOfDetail_PairwiseSeamFinder_get_inner_ptr_mut
	}
	
	impl PtrOfDetail_PairwiseSeamFinder {
		#[inline] pub fn as_raw_PtrOfDetail_PairwiseSeamFinder(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_PairwiseSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_PairwiseSeamFinderConst for PtrOfDetail_PairwiseSeamFinder {
		#[inline] fn as_raw_Detail_PairwiseSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_PairwiseSeamFinder for PtrOfDetail_PairwiseSeamFinder {
		#[inline] fn as_raw_mut_Detail_PairwiseSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_SeamFinderConst for PtrOfDetail_PairwiseSeamFinder {
		#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_SeamFinder for PtrOfDetail_PairwiseSeamFinder {
		#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_PairwiseSeamFinder, core::Ptr<dyn crate::stitching::Detail_SeamFinder>,
		cv_PtrOfDetail_PairwiseSeamFinder_to_PtrOfDetail_SeamFinder,
	}
	
	pub type PtrOfDetail_RotationWarper = core::Ptr<dyn crate::stitching::Detail_RotationWarper>;
	
	ptr_extern! { dyn crate::stitching::Detail_RotationWarper,
		cv_PtrOfDetail_RotationWarper_delete, cv_PtrOfDetail_RotationWarper_get_inner_ptr, cv_PtrOfDetail_RotationWarper_get_inner_ptr_mut
	}
	
	impl PtrOfDetail_RotationWarper {
		#[inline] pub fn as_raw_PtrOfDetail_RotationWarper(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_RotationWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_RotationWarperConst for PtrOfDetail_RotationWarper {
		#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_RotationWarper for PtrOfDetail_RotationWarper {
		#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_SeamFinder = core::Ptr<dyn crate::stitching::Detail_SeamFinder>;
	
	ptr_extern! { dyn crate::stitching::Detail_SeamFinder,
		cv_PtrOfDetail_SeamFinder_delete, cv_PtrOfDetail_SeamFinder_get_inner_ptr, cv_PtrOfDetail_SeamFinder_get_inner_ptr_mut
	}
	
	impl PtrOfDetail_SeamFinder {
		#[inline] pub fn as_raw_PtrOfDetail_SeamFinder(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_SeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_SeamFinderConst for PtrOfDetail_SeamFinder {
		#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_SeamFinder for PtrOfDetail_SeamFinder {
		#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFisheyeWarper = core::Ptr<crate::stitching::FisheyeWarper>;
	
	ptr_extern! { crate::stitching::FisheyeWarper,
		cv_PtrOfFisheyeWarper_delete, cv_PtrOfFisheyeWarper_get_inner_ptr, cv_PtrOfFisheyeWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::FisheyeWarper, cv_PtrOfFisheyeWarper_new }
	
	impl PtrOfFisheyeWarper {
		#[inline] pub fn as_raw_PtrOfFisheyeWarper(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFisheyeWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::FisheyeWarperTraitConst for PtrOfFisheyeWarper {
		#[inline] fn as_raw_FisheyeWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::FisheyeWarperTrait for PtrOfFisheyeWarper {
		#[inline] fn as_raw_mut_FisheyeWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for PtrOfFisheyeWarper {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for PtrOfFisheyeWarper {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfFisheyeWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
		cv_PtrOfFisheyeWarper_to_PtrOfWarperCreator,
	}
	
	pub type PtrOfMercatorWarper = core::Ptr<crate::stitching::MercatorWarper>;
	
	ptr_extern! { crate::stitching::MercatorWarper,
		cv_PtrOfMercatorWarper_delete, cv_PtrOfMercatorWarper_get_inner_ptr, cv_PtrOfMercatorWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::MercatorWarper, cv_PtrOfMercatorWarper_new }
	
	impl PtrOfMercatorWarper {
		#[inline] pub fn as_raw_PtrOfMercatorWarper(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMercatorWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::MercatorWarperTraitConst for PtrOfMercatorWarper {
		#[inline] fn as_raw_MercatorWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::MercatorWarperTrait for PtrOfMercatorWarper {
		#[inline] fn as_raw_mut_MercatorWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for PtrOfMercatorWarper {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for PtrOfMercatorWarper {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMercatorWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
		cv_PtrOfMercatorWarper_to_PtrOfWarperCreator,
	}
	
	pub type PtrOfPaniniPortraitWarper = core::Ptr<crate::stitching::PaniniPortraitWarper>;
	
	ptr_extern! { crate::stitching::PaniniPortraitWarper,
		cv_PtrOfPaniniPortraitWarper_delete, cv_PtrOfPaniniPortraitWarper_get_inner_ptr, cv_PtrOfPaniniPortraitWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::PaniniPortraitWarper, cv_PtrOfPaniniPortraitWarper_new }
	
	impl PtrOfPaniniPortraitWarper {
		#[inline] pub fn as_raw_PtrOfPaniniPortraitWarper(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPaniniPortraitWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::PaniniPortraitWarperTraitConst for PtrOfPaniniPortraitWarper {
		#[inline] fn as_raw_PaniniPortraitWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::PaniniPortraitWarperTrait for PtrOfPaniniPortraitWarper {
		#[inline] fn as_raw_mut_PaniniPortraitWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for PtrOfPaniniPortraitWarper {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for PtrOfPaniniPortraitWarper {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfPaniniPortraitWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
		cv_PtrOfPaniniPortraitWarper_to_PtrOfWarperCreator,
	}
	
	pub type PtrOfPaniniWarper = core::Ptr<crate::stitching::PaniniWarper>;
	
	ptr_extern! { crate::stitching::PaniniWarper,
		cv_PtrOfPaniniWarper_delete, cv_PtrOfPaniniWarper_get_inner_ptr, cv_PtrOfPaniniWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::PaniniWarper, cv_PtrOfPaniniWarper_new }
	
	impl PtrOfPaniniWarper {
		#[inline] pub fn as_raw_PtrOfPaniniWarper(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPaniniWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::PaniniWarperTraitConst for PtrOfPaniniWarper {
		#[inline] fn as_raw_PaniniWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::PaniniWarperTrait for PtrOfPaniniWarper {
		#[inline] fn as_raw_mut_PaniniWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for PtrOfPaniniWarper {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for PtrOfPaniniWarper {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfPaniniWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
		cv_PtrOfPaniniWarper_to_PtrOfWarperCreator,
	}
	
	pub type PtrOfPlaneWarper = core::Ptr<crate::stitching::PlaneWarper>;
	
	ptr_extern! { crate::stitching::PlaneWarper,
		cv_PtrOfPlaneWarper_delete, cv_PtrOfPlaneWarper_get_inner_ptr, cv_PtrOfPlaneWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::PlaneWarper, cv_PtrOfPlaneWarper_new }
	
	impl PtrOfPlaneWarper {
		#[inline] pub fn as_raw_PtrOfPlaneWarper(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPlaneWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::PlaneWarperTraitConst for PtrOfPlaneWarper {
		#[inline] fn as_raw_PlaneWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::PlaneWarperTrait for PtrOfPlaneWarper {
		#[inline] fn as_raw_mut_PlaneWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for PtrOfPlaneWarper {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for PtrOfPlaneWarper {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfPlaneWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
		cv_PtrOfPlaneWarper_to_PtrOfWarperCreator,
	}
	
	pub type PtrOfPlaneWarperGpu = core::Ptr<crate::stitching::PlaneWarperGpu>;
	
	ptr_extern! { crate::stitching::PlaneWarperGpu,
		cv_PtrOfPlaneWarperGpu_delete, cv_PtrOfPlaneWarperGpu_get_inner_ptr, cv_PtrOfPlaneWarperGpu_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::PlaneWarperGpu, cv_PtrOfPlaneWarperGpu_new }
	
	impl PtrOfPlaneWarperGpu {
		#[inline] pub fn as_raw_PtrOfPlaneWarperGpu(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPlaneWarperGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::PlaneWarperGpuTraitConst for PtrOfPlaneWarperGpu {
		#[inline] fn as_raw_PlaneWarperGpu(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::PlaneWarperGpuTrait for PtrOfPlaneWarperGpu {
		#[inline] fn as_raw_mut_PlaneWarperGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for PtrOfPlaneWarperGpu {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for PtrOfPlaneWarperGpu {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfPlaneWarperGpu, core::Ptr<dyn crate::stitching::WarperCreator>,
		cv_PtrOfPlaneWarperGpu_to_PtrOfWarperCreator,
	}
	
	pub type PtrOfSphericalWarper = core::Ptr<crate::stitching::SphericalWarper>;
	
	ptr_extern! { crate::stitching::SphericalWarper,
		cv_PtrOfSphericalWarper_delete, cv_PtrOfSphericalWarper_get_inner_ptr, cv_PtrOfSphericalWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::SphericalWarper, cv_PtrOfSphericalWarper_new }
	
	impl PtrOfSphericalWarper {
		#[inline] pub fn as_raw_PtrOfSphericalWarper(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSphericalWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::SphericalWarperTraitConst for PtrOfSphericalWarper {
		#[inline] fn as_raw_SphericalWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::SphericalWarperTrait for PtrOfSphericalWarper {
		#[inline] fn as_raw_mut_SphericalWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for PtrOfSphericalWarper {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for PtrOfSphericalWarper {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSphericalWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
		cv_PtrOfSphericalWarper_to_PtrOfWarperCreator,
	}
	
	pub type PtrOfSphericalWarperGpu = core::Ptr<crate::stitching::SphericalWarperGpu>;
	
	ptr_extern! { crate::stitching::SphericalWarperGpu,
		cv_PtrOfSphericalWarperGpu_delete, cv_PtrOfSphericalWarperGpu_get_inner_ptr, cv_PtrOfSphericalWarperGpu_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::SphericalWarperGpu, cv_PtrOfSphericalWarperGpu_new }
	
	impl PtrOfSphericalWarperGpu {
		#[inline] pub fn as_raw_PtrOfSphericalWarperGpu(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSphericalWarperGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::SphericalWarperGpuTraitConst for PtrOfSphericalWarperGpu {
		#[inline] fn as_raw_SphericalWarperGpu(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::SphericalWarperGpuTrait for PtrOfSphericalWarperGpu {
		#[inline] fn as_raw_mut_SphericalWarperGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for PtrOfSphericalWarperGpu {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for PtrOfSphericalWarperGpu {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSphericalWarperGpu, core::Ptr<dyn crate::stitching::WarperCreator>,
		cv_PtrOfSphericalWarperGpu_to_PtrOfWarperCreator,
	}
	
	pub type PtrOfStereographicWarper = core::Ptr<crate::stitching::StereographicWarper>;
	
	ptr_extern! { crate::stitching::StereographicWarper,
		cv_PtrOfStereographicWarper_delete, cv_PtrOfStereographicWarper_get_inner_ptr, cv_PtrOfStereographicWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::StereographicWarper, cv_PtrOfStereographicWarper_new }
	
	impl PtrOfStereographicWarper {
		#[inline] pub fn as_raw_PtrOfStereographicWarper(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStereographicWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::StereographicWarperTraitConst for PtrOfStereographicWarper {
		#[inline] fn as_raw_StereographicWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::StereographicWarperTrait for PtrOfStereographicWarper {
		#[inline] fn as_raw_mut_StereographicWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for PtrOfStereographicWarper {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for PtrOfStereographicWarper {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfStereographicWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
		cv_PtrOfStereographicWarper_to_PtrOfWarperCreator,
	}
	
	pub type PtrOfStitcher = core::Ptr<crate::stitching::Stitcher>;
	
	ptr_extern! { crate::stitching::Stitcher,
		cv_PtrOfStitcher_delete, cv_PtrOfStitcher_get_inner_ptr, cv_PtrOfStitcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Stitcher, cv_PtrOfStitcher_new }
	
	impl PtrOfStitcher {
		#[inline] pub fn as_raw_PtrOfStitcher(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStitcher(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::StitcherTraitConst for PtrOfStitcher {
		#[inline] fn as_raw_Stitcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::StitcherTrait for PtrOfStitcher {
		#[inline] fn as_raw_mut_Stitcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTransverseMercatorWarper = core::Ptr<crate::stitching::TransverseMercatorWarper>;
	
	ptr_extern! { crate::stitching::TransverseMercatorWarper,
		cv_PtrOfTransverseMercatorWarper_delete, cv_PtrOfTransverseMercatorWarper_get_inner_ptr, cv_PtrOfTransverseMercatorWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::TransverseMercatorWarper, cv_PtrOfTransverseMercatorWarper_new }
	
	impl PtrOfTransverseMercatorWarper {
		#[inline] pub fn as_raw_PtrOfTransverseMercatorWarper(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTransverseMercatorWarper(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::TransverseMercatorWarperTraitConst for PtrOfTransverseMercatorWarper {
		#[inline] fn as_raw_TransverseMercatorWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::TransverseMercatorWarperTrait for PtrOfTransverseMercatorWarper {
		#[inline] fn as_raw_mut_TransverseMercatorWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for PtrOfTransverseMercatorWarper {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for PtrOfTransverseMercatorWarper {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfTransverseMercatorWarper, core::Ptr<dyn crate::stitching::WarperCreator>,
		cv_PtrOfTransverseMercatorWarper_to_PtrOfWarperCreator,
	}
	
	pub type PtrOfWarperCreator = core::Ptr<dyn crate::stitching::WarperCreator>;
	
	ptr_extern! { dyn crate::stitching::WarperCreator,
		cv_PtrOfWarperCreator_delete, cv_PtrOfWarperCreator_get_inner_ptr, cv_PtrOfWarperCreator_get_inner_ptr_mut
	}
	
	impl PtrOfWarperCreator {
		#[inline] pub fn as_raw_PtrOfWarperCreator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfWarperCreator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorConst for PtrOfWarperCreator {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreator for PtrOfWarperCreator {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfDetail_CameraParams = core::Vector<crate::stitching::Detail_CameraParams>;
	
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
		cv_VectorOfDetail_CameraParams_get, cv_VectorOfDetail_CameraParams_set,
		cv_VectorOfDetail_CameraParams_push, cv_VectorOfDetail_CameraParams_insert,
	}
	vector_non_copy_or_bool! { crate::stitching::Detail_CameraParams }
	
	unsafe impl Send for core::Vector<crate::stitching::Detail_CameraParams> {}
	
	pub type VectorOfDetail_ImageFeatures = core::Vector<crate::stitching::Detail_ImageFeatures>;
	
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
		cv_VectorOfDetail_ImageFeatures_get, cv_VectorOfDetail_ImageFeatures_set,
		cv_VectorOfDetail_ImageFeatures_push, cv_VectorOfDetail_ImageFeatures_insert,
	}
	vector_non_copy_or_bool! { crate::stitching::Detail_ImageFeatures }
	
	unsafe impl Send for core::Vector<crate::stitching::Detail_ImageFeatures> {}
	
	pub type VectorOfDetail_MatchesInfo = core::Vector<crate::stitching::Detail_MatchesInfo>;
	
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
		cv_VectorOfDetail_MatchesInfo_get, cv_VectorOfDetail_MatchesInfo_set,
		cv_VectorOfDetail_MatchesInfo_push, cv_VectorOfDetail_MatchesInfo_insert,
	}
	vector_non_copy_or_bool! { crate::stitching::Detail_MatchesInfo }
	
	unsafe impl Send for core::Vector<crate::stitching::Detail_MatchesInfo> {}
	
}
#[cfg(ocvrs_has_module_stitching)]
pub use stitching_types::*;

#[cfg(ocvrs_has_module_structured_light)]
mod structured_light_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfGrayCodePattern = core::Ptr<dyn crate::structured_light::GrayCodePattern>;
	
	ptr_extern! { dyn crate::structured_light::GrayCodePattern,
		cv_PtrOfGrayCodePattern_delete, cv_PtrOfGrayCodePattern_get_inner_ptr, cv_PtrOfGrayCodePattern_get_inner_ptr_mut
	}
	
	impl PtrOfGrayCodePattern {
		#[inline] pub fn as_raw_PtrOfGrayCodePattern(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGrayCodePattern(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::structured_light::GrayCodePatternConst for PtrOfGrayCodePattern {
		#[inline] fn as_raw_GrayCodePattern(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::structured_light::GrayCodePattern for PtrOfGrayCodePattern {
		#[inline] fn as_raw_mut_GrayCodePattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfGrayCodePattern {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfGrayCodePattern {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::structured_light::StructuredLightPatternConst for PtrOfGrayCodePattern {
		#[inline] fn as_raw_StructuredLightPattern(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::structured_light::StructuredLightPattern for PtrOfGrayCodePattern {
		#[inline] fn as_raw_mut_StructuredLightPattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSinusoidalPattern = core::Ptr<dyn crate::structured_light::SinusoidalPattern>;
	
	ptr_extern! { dyn crate::structured_light::SinusoidalPattern,
		cv_PtrOfSinusoidalPattern_delete, cv_PtrOfSinusoidalPattern_get_inner_ptr, cv_PtrOfSinusoidalPattern_get_inner_ptr_mut
	}
	
	impl PtrOfSinusoidalPattern {
		#[inline] pub fn as_raw_PtrOfSinusoidalPattern(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSinusoidalPattern(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::structured_light::SinusoidalPatternConst for PtrOfSinusoidalPattern {
		#[inline] fn as_raw_SinusoidalPattern(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::structured_light::SinusoidalPattern for PtrOfSinusoidalPattern {
		#[inline] fn as_raw_mut_SinusoidalPattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSinusoidalPattern {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSinusoidalPattern {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::structured_light::StructuredLightPatternConst for PtrOfSinusoidalPattern {
		#[inline] fn as_raw_StructuredLightPattern(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::structured_light::StructuredLightPattern for PtrOfSinusoidalPattern {
		#[inline] fn as_raw_mut_StructuredLightPattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSinusoidalPattern_Params = core::Ptr<crate::structured_light::SinusoidalPattern_Params>;
	
	ptr_extern! { crate::structured_light::SinusoidalPattern_Params,
		cv_PtrOfSinusoidalPattern_Params_delete, cv_PtrOfSinusoidalPattern_Params_get_inner_ptr, cv_PtrOfSinusoidalPattern_Params_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::structured_light::SinusoidalPattern_Params, cv_PtrOfSinusoidalPattern_Params_new }
	
	impl PtrOfSinusoidalPattern_Params {
		#[inline] pub fn as_raw_PtrOfSinusoidalPattern_Params(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSinusoidalPattern_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::structured_light::SinusoidalPattern_ParamsTraitConst for PtrOfSinusoidalPattern_Params {
		#[inline] fn as_raw_SinusoidalPattern_Params(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::structured_light::SinusoidalPattern_ParamsTrait for PtrOfSinusoidalPattern_Params {
		#[inline] fn as_raw_mut_SinusoidalPattern_Params(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_structured_light)]
pub use structured_light_types::*;

#[cfg(ocvrs_has_module_superres)]
mod superres_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfSuperres_BroxOpticalFlow = core::Ptr<dyn crate::superres::Superres_BroxOpticalFlow>;
	
	ptr_extern! { dyn crate::superres::Superres_BroxOpticalFlow,
		cv_PtrOfSuperres_BroxOpticalFlow_delete, cv_PtrOfSuperres_BroxOpticalFlow_get_inner_ptr, cv_PtrOfSuperres_BroxOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfSuperres_BroxOpticalFlow {
		#[inline] pub fn as_raw_PtrOfSuperres_BroxOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperres_BroxOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::superres::Superres_BroxOpticalFlowConst for PtrOfSuperres_BroxOpticalFlow {
		#[inline] fn as_raw_Superres_BroxOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_BroxOpticalFlow for PtrOfSuperres_BroxOpticalFlow {
		#[inline] fn as_raw_mut_Superres_BroxOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSuperres_BroxOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperres_BroxOpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExtConst for PtrOfSuperres_BroxOpticalFlow {
		#[inline] fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_BroxOpticalFlow {
		#[inline] fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_DenseOpticalFlowExt = core::Ptr<dyn crate::superres::Superres_DenseOpticalFlowExt>;
	
	ptr_extern! { dyn crate::superres::Superres_DenseOpticalFlowExt,
		cv_PtrOfSuperres_DenseOpticalFlowExt_delete, cv_PtrOfSuperres_DenseOpticalFlowExt_get_inner_ptr, cv_PtrOfSuperres_DenseOpticalFlowExt_get_inner_ptr_mut
	}
	
	impl PtrOfSuperres_DenseOpticalFlowExt {
		#[inline] pub fn as_raw_PtrOfSuperres_DenseOpticalFlowExt(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExtConst for PtrOfSuperres_DenseOpticalFlowExt {
		#[inline] fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_DenseOpticalFlowExt {
		#[inline] fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSuperres_DenseOpticalFlowExt {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperres_DenseOpticalFlowExt {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_DualTVL1OpticalFlow = core::Ptr<dyn crate::superres::Superres_DualTVL1OpticalFlow>;
	
	ptr_extern! { dyn crate::superres::Superres_DualTVL1OpticalFlow,
		cv_PtrOfSuperres_DualTVL1OpticalFlow_delete, cv_PtrOfSuperres_DualTVL1OpticalFlow_get_inner_ptr, cv_PtrOfSuperres_DualTVL1OpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfSuperres_DualTVL1OpticalFlow {
		#[inline] pub fn as_raw_PtrOfSuperres_DualTVL1OpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperres_DualTVL1OpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::superres::Superres_DualTVL1OpticalFlowConst for PtrOfSuperres_DualTVL1OpticalFlow {
		#[inline] fn as_raw_Superres_DualTVL1OpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_DualTVL1OpticalFlow for PtrOfSuperres_DualTVL1OpticalFlow {
		#[inline] fn as_raw_mut_Superres_DualTVL1OpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSuperres_DualTVL1OpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperres_DualTVL1OpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExtConst for PtrOfSuperres_DualTVL1OpticalFlow {
		#[inline] fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_DualTVL1OpticalFlow {
		#[inline] fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_FarnebackOpticalFlow = core::Ptr<dyn crate::superres::Superres_FarnebackOpticalFlow>;
	
	ptr_extern! { dyn crate::superres::Superres_FarnebackOpticalFlow,
		cv_PtrOfSuperres_FarnebackOpticalFlow_delete, cv_PtrOfSuperres_FarnebackOpticalFlow_get_inner_ptr, cv_PtrOfSuperres_FarnebackOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfSuperres_FarnebackOpticalFlow {
		#[inline] pub fn as_raw_PtrOfSuperres_FarnebackOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperres_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::superres::Superres_FarnebackOpticalFlowConst for PtrOfSuperres_FarnebackOpticalFlow {
		#[inline] fn as_raw_Superres_FarnebackOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_FarnebackOpticalFlow for PtrOfSuperres_FarnebackOpticalFlow {
		#[inline] fn as_raw_mut_Superres_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSuperres_FarnebackOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperres_FarnebackOpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExtConst for PtrOfSuperres_FarnebackOpticalFlow {
		#[inline] fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_FarnebackOpticalFlow {
		#[inline] fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_FrameSource = core::Ptr<dyn crate::superres::Superres_FrameSource>;
	
	ptr_extern! { dyn crate::superres::Superres_FrameSource,
		cv_PtrOfSuperres_FrameSource_delete, cv_PtrOfSuperres_FrameSource_get_inner_ptr, cv_PtrOfSuperres_FrameSource_get_inner_ptr_mut
	}
	
	impl PtrOfSuperres_FrameSource {
		#[inline] pub fn as_raw_PtrOfSuperres_FrameSource(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperres_FrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::superres::Superres_FrameSourceConst for PtrOfSuperres_FrameSource {
		#[inline] fn as_raw_Superres_FrameSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_FrameSource for PtrOfSuperres_FrameSource {
		#[inline] fn as_raw_mut_Superres_FrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_PyrLKOpticalFlow = core::Ptr<dyn crate::superres::Superres_PyrLKOpticalFlow>;
	
	ptr_extern! { dyn crate::superres::Superres_PyrLKOpticalFlow,
		cv_PtrOfSuperres_PyrLKOpticalFlow_delete, cv_PtrOfSuperres_PyrLKOpticalFlow_get_inner_ptr, cv_PtrOfSuperres_PyrLKOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfSuperres_PyrLKOpticalFlow {
		#[inline] pub fn as_raw_PtrOfSuperres_PyrLKOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperres_PyrLKOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::superres::Superres_PyrLKOpticalFlowConst for PtrOfSuperres_PyrLKOpticalFlow {
		#[inline] fn as_raw_Superres_PyrLKOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_PyrLKOpticalFlow for PtrOfSuperres_PyrLKOpticalFlow {
		#[inline] fn as_raw_mut_Superres_PyrLKOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSuperres_PyrLKOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperres_PyrLKOpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExtConst for PtrOfSuperres_PyrLKOpticalFlow {
		#[inline] fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExt for PtrOfSuperres_PyrLKOpticalFlow {
		#[inline] fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_SuperResolution = core::Ptr<dyn crate::superres::Superres_SuperResolution>;
	
	ptr_extern! { dyn crate::superres::Superres_SuperResolution,
		cv_PtrOfSuperres_SuperResolution_delete, cv_PtrOfSuperres_SuperResolution_get_inner_ptr, cv_PtrOfSuperres_SuperResolution_get_inner_ptr_mut
	}
	
	impl PtrOfSuperres_SuperResolution {
		#[inline] pub fn as_raw_PtrOfSuperres_SuperResolution(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperres_SuperResolution(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::superres::Superres_SuperResolutionConst for PtrOfSuperres_SuperResolution {
		#[inline] fn as_raw_Superres_SuperResolution(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_SuperResolution for PtrOfSuperres_SuperResolution {
		#[inline] fn as_raw_mut_Superres_SuperResolution(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSuperres_SuperResolution {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperres_SuperResolution {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_FrameSourceConst for PtrOfSuperres_SuperResolution {
		#[inline] fn as_raw_Superres_FrameSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_FrameSource for PtrOfSuperres_SuperResolution {
		#[inline] fn as_raw_mut_Superres_FrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_superres)]
pub use superres_types::*;

#[cfg(ocvrs_has_module_surface_matching)]
mod surface_matching_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfPose3D = core::Ptr<crate::surface_matching::Pose3D>;
	
	ptr_extern! { crate::surface_matching::Pose3D,
		cv_PtrOfPose3D_delete, cv_PtrOfPose3D_get_inner_ptr, cv_PtrOfPose3D_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::surface_matching::Pose3D, cv_PtrOfPose3D_new }
	
	impl PtrOfPose3D {
		#[inline] pub fn as_raw_PtrOfPose3D(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPose3D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::surface_matching::Pose3DTraitConst for PtrOfPose3D {
		#[inline] fn as_raw_Pose3D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::surface_matching::Pose3DTrait for PtrOfPose3D {
		#[inline] fn as_raw_mut_Pose3D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPoseCluster3D = core::Ptr<crate::surface_matching::PoseCluster3D>;
	
	ptr_extern! { crate::surface_matching::PoseCluster3D,
		cv_PtrOfPoseCluster3D_delete, cv_PtrOfPoseCluster3D_get_inner_ptr, cv_PtrOfPoseCluster3D_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::surface_matching::PoseCluster3D, cv_PtrOfPoseCluster3D_new }
	
	impl PtrOfPoseCluster3D {
		#[inline] pub fn as_raw_PtrOfPoseCluster3D(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPoseCluster3D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::surface_matching::PoseCluster3DTraitConst for PtrOfPoseCluster3D {
		#[inline] fn as_raw_PoseCluster3D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::surface_matching::PoseCluster3DTrait for PtrOfPoseCluster3D {
		#[inline] fn as_raw_mut_PoseCluster3D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfPose3DPtr = core::Vector<crate::surface_matching::Pose3DPtr>;
	
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
		cv_VectorOfPose3DPtr_get, cv_VectorOfPose3DPtr_set,
		cv_VectorOfPose3DPtr_push, cv_VectorOfPose3DPtr_insert,
	}
	vector_non_copy_or_bool! { crate::surface_matching::Pose3DPtr }
	
	unsafe impl Send for core::Vector<crate::surface_matching::Pose3DPtr> {}
	
}
#[cfg(ocvrs_has_module_surface_matching)]
pub use surface_matching_types::*;

#[cfg(ocvrs_has_module_text)]
mod text_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfERFilter = core::Ptr<dyn crate::text::ERFilter>;
	
	ptr_extern! { dyn crate::text::ERFilter,
		cv_PtrOfERFilter_delete, cv_PtrOfERFilter_get_inner_ptr, cv_PtrOfERFilter_get_inner_ptr_mut
	}
	
	impl PtrOfERFilter {
		#[inline] pub fn as_raw_PtrOfERFilter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfERFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::ERFilterConst for PtrOfERFilter {
		#[inline] fn as_raw_ERFilter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::ERFilter for PtrOfERFilter {
		#[inline] fn as_raw_mut_ERFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfERFilter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfERFilter {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfERFilter_Callback = core::Ptr<dyn crate::text::ERFilter_Callback>;
	
	ptr_extern! { dyn crate::text::ERFilter_Callback,
		cv_PtrOfERFilter_Callback_delete, cv_PtrOfERFilter_Callback_get_inner_ptr, cv_PtrOfERFilter_Callback_get_inner_ptr_mut
	}
	
	impl PtrOfERFilter_Callback {
		#[inline] pub fn as_raw_PtrOfERFilter_Callback(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfERFilter_Callback(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::ERFilter_CallbackConst for PtrOfERFilter_Callback {
		#[inline] fn as_raw_ERFilter_Callback(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::ERFilter_Callback for PtrOfERFilter_Callback {
		#[inline] fn as_raw_mut_ERFilter_Callback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRBeamSearchDecoder = core::Ptr<crate::text::OCRBeamSearchDecoder>;
	
	ptr_extern! { crate::text::OCRBeamSearchDecoder,
		cv_PtrOfOCRBeamSearchDecoder_delete, cv_PtrOfOCRBeamSearchDecoder_get_inner_ptr, cv_PtrOfOCRBeamSearchDecoder_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::text::OCRBeamSearchDecoder, cv_PtrOfOCRBeamSearchDecoder_new }
	
	impl PtrOfOCRBeamSearchDecoder {
		#[inline] pub fn as_raw_PtrOfOCRBeamSearchDecoder(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOCRBeamSearchDecoder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::OCRBeamSearchDecoderTraitConst for PtrOfOCRBeamSearchDecoder {
		#[inline] fn as_raw_OCRBeamSearchDecoder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::OCRBeamSearchDecoderTrait for PtrOfOCRBeamSearchDecoder {
		#[inline] fn as_raw_mut_OCRBeamSearchDecoder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::text::BaseOCRConst for PtrOfOCRBeamSearchDecoder {
		#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::BaseOCR for PtrOfOCRBeamSearchDecoder {
		#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRBeamSearchDecoder_ClassifierCallback = core::Ptr<crate::text::OCRBeamSearchDecoder_ClassifierCallback>;
	
	ptr_extern! { crate::text::OCRBeamSearchDecoder_ClassifierCallback,
		cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_delete, cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_get_inner_ptr, cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::text::OCRBeamSearchDecoder_ClassifierCallback, cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_new }
	
	impl PtrOfOCRBeamSearchDecoder_ClassifierCallback {
		#[inline] pub fn as_raw_PtrOfOCRBeamSearchDecoder_ClassifierCallback(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOCRBeamSearchDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::OCRBeamSearchDecoder_ClassifierCallbackTraitConst for PtrOfOCRBeamSearchDecoder_ClassifierCallback {
		#[inline] fn as_raw_OCRBeamSearchDecoder_ClassifierCallback(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::OCRBeamSearchDecoder_ClassifierCallbackTrait for PtrOfOCRBeamSearchDecoder_ClassifierCallback {
		#[inline] fn as_raw_mut_OCRBeamSearchDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRHMMDecoder = core::Ptr<crate::text::OCRHMMDecoder>;
	
	ptr_extern! { crate::text::OCRHMMDecoder,
		cv_PtrOfOCRHMMDecoder_delete, cv_PtrOfOCRHMMDecoder_get_inner_ptr, cv_PtrOfOCRHMMDecoder_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::text::OCRHMMDecoder, cv_PtrOfOCRHMMDecoder_new }
	
	impl PtrOfOCRHMMDecoder {
		#[inline] pub fn as_raw_PtrOfOCRHMMDecoder(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOCRHMMDecoder(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::OCRHMMDecoderTraitConst for PtrOfOCRHMMDecoder {
		#[inline] fn as_raw_OCRHMMDecoder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::OCRHMMDecoderTrait for PtrOfOCRHMMDecoder {
		#[inline] fn as_raw_mut_OCRHMMDecoder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::text::BaseOCRConst for PtrOfOCRHMMDecoder {
		#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::BaseOCR for PtrOfOCRHMMDecoder {
		#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRHMMDecoder_ClassifierCallback = core::Ptr<crate::text::OCRHMMDecoder_ClassifierCallback>;
	
	ptr_extern! { crate::text::OCRHMMDecoder_ClassifierCallback,
		cv_PtrOfOCRHMMDecoder_ClassifierCallback_delete, cv_PtrOfOCRHMMDecoder_ClassifierCallback_get_inner_ptr, cv_PtrOfOCRHMMDecoder_ClassifierCallback_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::text::OCRHMMDecoder_ClassifierCallback, cv_PtrOfOCRHMMDecoder_ClassifierCallback_new }
	
	impl PtrOfOCRHMMDecoder_ClassifierCallback {
		#[inline] pub fn as_raw_PtrOfOCRHMMDecoder_ClassifierCallback(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOCRHMMDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::OCRHMMDecoder_ClassifierCallbackTraitConst for PtrOfOCRHMMDecoder_ClassifierCallback {
		#[inline] fn as_raw_OCRHMMDecoder_ClassifierCallback(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::OCRHMMDecoder_ClassifierCallbackTrait for PtrOfOCRHMMDecoder_ClassifierCallback {
		#[inline] fn as_raw_mut_OCRHMMDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRHolisticWordRecognizer = core::Ptr<dyn crate::text::OCRHolisticWordRecognizer>;
	
	ptr_extern! { dyn crate::text::OCRHolisticWordRecognizer,
		cv_PtrOfOCRHolisticWordRecognizer_delete, cv_PtrOfOCRHolisticWordRecognizer_get_inner_ptr, cv_PtrOfOCRHolisticWordRecognizer_get_inner_ptr_mut
	}
	
	impl PtrOfOCRHolisticWordRecognizer {
		#[inline] pub fn as_raw_PtrOfOCRHolisticWordRecognizer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOCRHolisticWordRecognizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::OCRHolisticWordRecognizerConst for PtrOfOCRHolisticWordRecognizer {
		#[inline] fn as_raw_OCRHolisticWordRecognizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::OCRHolisticWordRecognizer for PtrOfOCRHolisticWordRecognizer {
		#[inline] fn as_raw_mut_OCRHolisticWordRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::text::BaseOCRConst for PtrOfOCRHolisticWordRecognizer {
		#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::BaseOCR for PtrOfOCRHolisticWordRecognizer {
		#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRTesseract = core::Ptr<dyn crate::text::OCRTesseract>;
	
	ptr_extern! { dyn crate::text::OCRTesseract,
		cv_PtrOfOCRTesseract_delete, cv_PtrOfOCRTesseract_get_inner_ptr, cv_PtrOfOCRTesseract_get_inner_ptr_mut
	}
	
	impl PtrOfOCRTesseract {
		#[inline] pub fn as_raw_PtrOfOCRTesseract(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOCRTesseract(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::OCRTesseractConst for PtrOfOCRTesseract {
		#[inline] fn as_raw_OCRTesseract(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::OCRTesseract for PtrOfOCRTesseract {
		#[inline] fn as_raw_mut_OCRTesseract(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::text::BaseOCRConst for PtrOfOCRTesseract {
		#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::BaseOCR for PtrOfOCRTesseract {
		#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTextDetectorCNN = core::Ptr<dyn crate::text::TextDetectorCNN>;
	
	ptr_extern! { dyn crate::text::TextDetectorCNN,
		cv_PtrOfTextDetectorCNN_delete, cv_PtrOfTextDetectorCNN_get_inner_ptr, cv_PtrOfTextDetectorCNN_get_inner_ptr_mut
	}
	
	impl PtrOfTextDetectorCNN {
		#[inline] pub fn as_raw_PtrOfTextDetectorCNN(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTextDetectorCNN(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::text::TextDetectorCNNConst for PtrOfTextDetectorCNN {
		#[inline] fn as_raw_TextDetectorCNN(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::TextDetectorCNN for PtrOfTextDetectorCNN {
		#[inline] fn as_raw_mut_TextDetectorCNN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::text::TextDetectorConst for PtrOfTextDetectorCNN {
		#[inline] fn as_raw_TextDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::TextDetector for PtrOfTextDetectorCNN {
		#[inline] fn as_raw_mut_TextDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfERStat = core::Vector<crate::text::ERStat>;
	
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
		cv_VectorOfERStat_get, cv_VectorOfERStat_set,
		cv_VectorOfERStat_push, cv_VectorOfERStat_insert,
	}
	vector_non_copy_or_bool! { crate::text::ERStat }
	
	unsafe impl Send for core::Vector<crate::text::ERStat> {}
	
	pub type VectorOfVectorOfERStat = core::Vector<core::Vector<crate::text::ERStat>>;
	
	impl VectorOfVectorOfERStat {
		pub fn as_raw_VectorOfVectorOfERStat(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfERStat(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<crate::text::ERStat>, *const c_void, *mut c_void,
		cv_VectorOfVectorOfERStat_new, cv_VectorOfVectorOfERStat_delete,
		cv_VectorOfVectorOfERStat_len, cv_VectorOfVectorOfERStat_is_empty,
		cv_VectorOfVectorOfERStat_capacity, cv_VectorOfVectorOfERStat_shrink_to_fit,
		cv_VectorOfVectorOfERStat_reserve, cv_VectorOfVectorOfERStat_remove,
		cv_VectorOfVectorOfERStat_swap, cv_VectorOfVectorOfERStat_clear,
		cv_VectorOfVectorOfERStat_get, cv_VectorOfVectorOfERStat_set,
		cv_VectorOfVectorOfERStat_push, cv_VectorOfVectorOfERStat_insert,
	}
	vector_non_copy_or_bool! { core::Vector<crate::text::ERStat> }
	
	unsafe impl Send for core::Vector<core::Vector<crate::text::ERStat>> {}
	
}
#[cfg(ocvrs_has_module_text)]
pub use text_types::*;

#[cfg(ocvrs_has_module_tracking)]
mod tracking_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfTrackerCSRT = core::Ptr<dyn crate::tracking::TrackerCSRT>;
	
	ptr_extern! { dyn crate::tracking::TrackerCSRT,
		cv_PtrOfTrackerCSRT_delete, cv_PtrOfTrackerCSRT_get_inner_ptr, cv_PtrOfTrackerCSRT_get_inner_ptr_mut
	}
	
	impl PtrOfTrackerCSRT {
		#[inline] pub fn as_raw_PtrOfTrackerCSRT(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerCSRT(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::tracking::TrackerCSRTConst for PtrOfTrackerCSRT {
		#[inline] fn as_raw_TrackerCSRT(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::tracking::TrackerCSRT for PtrOfTrackerCSRT {
		#[inline] fn as_raw_mut_TrackerCSRT(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::TrackerConst for PtrOfTrackerCSRT {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::Tracker for PtrOfTrackerCSRT {
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerKCF = core::Ptr<dyn crate::tracking::TrackerKCF>;
	
	ptr_extern! { dyn crate::tracking::TrackerKCF,
		cv_PtrOfTrackerKCF_delete, cv_PtrOfTrackerKCF_get_inner_ptr, cv_PtrOfTrackerKCF_get_inner_ptr_mut
	}
	
	impl PtrOfTrackerKCF {
		#[inline] pub fn as_raw_PtrOfTrackerKCF(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerKCF(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::tracking::TrackerKCFConst for PtrOfTrackerKCF {
		#[inline] fn as_raw_TrackerKCF(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::tracking::TrackerKCF for PtrOfTrackerKCF {
		#[inline] fn as_raw_mut_TrackerKCF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::TrackerConst for PtrOfTrackerKCF {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::Tracker for PtrOfTrackerKCF {
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_tracking)]
pub use tracking_types::*;

#[cfg(ocvrs_has_module_video)]
mod video_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBackgroundSubtractorKNN = core::Ptr<dyn crate::video::BackgroundSubtractorKNN>;
	
	ptr_extern! { dyn crate::video::BackgroundSubtractorKNN,
		cv_PtrOfBackgroundSubtractorKNN_delete, cv_PtrOfBackgroundSubtractorKNN_get_inner_ptr, cv_PtrOfBackgroundSubtractorKNN_get_inner_ptr_mut
	}
	
	impl PtrOfBackgroundSubtractorKNN {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorKNN(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorKNN(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorKNNConst for PtrOfBackgroundSubtractorKNN {
		#[inline] fn as_raw_BackgroundSubtractorKNN(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractorKNN for PtrOfBackgroundSubtractorKNN {
		#[inline] fn as_raw_mut_BackgroundSubtractorKNN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfBackgroundSubtractorKNN {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfBackgroundSubtractorKNN {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorConst for PtrOfBackgroundSubtractorKNN {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorKNN {
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackgroundSubtractorMOG2 = core::Ptr<dyn crate::video::BackgroundSubtractorMOG2>;
	
	ptr_extern! { dyn crate::video::BackgroundSubtractorMOG2,
		cv_PtrOfBackgroundSubtractorMOG2_delete, cv_PtrOfBackgroundSubtractorMOG2_get_inner_ptr, cv_PtrOfBackgroundSubtractorMOG2_get_inner_ptr_mut
	}
	
	impl PtrOfBackgroundSubtractorMOG2 {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorMOG2(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorMOG2Const for PtrOfBackgroundSubtractorMOG2 {
		#[inline] fn as_raw_BackgroundSubtractorMOG2(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractorMOG2 for PtrOfBackgroundSubtractorMOG2 {
		#[inline] fn as_raw_mut_BackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfBackgroundSubtractorMOG2 {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfBackgroundSubtractorMOG2 {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorConst for PtrOfBackgroundSubtractorMOG2 {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorMOG2 {
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDISOpticalFlow = core::Ptr<dyn crate::video::DISOpticalFlow>;
	
	ptr_extern! { dyn crate::video::DISOpticalFlow,
		cv_PtrOfDISOpticalFlow_delete, cv_PtrOfDISOpticalFlow_get_inner_ptr, cv_PtrOfDISOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfDISOpticalFlow {
		#[inline] pub fn as_raw_PtrOfDISOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDISOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::video::DISOpticalFlowConst for PtrOfDISOpticalFlow {
		#[inline] fn as_raw_DISOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::DISOpticalFlow for PtrOfDISOpticalFlow {
		#[inline] fn as_raw_mut_DISOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfDISOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfDISOpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DenseOpticalFlowConst for PtrOfDISOpticalFlow {
		#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::DenseOpticalFlow for PtrOfDISOpticalFlow {
		#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDenseOpticalFlow = core::Ptr<dyn crate::video::DenseOpticalFlow>;
	
	ptr_extern! { dyn crate::video::DenseOpticalFlow,
		cv_PtrOfDenseOpticalFlow_delete, cv_PtrOfDenseOpticalFlow_get_inner_ptr, cv_PtrOfDenseOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfDenseOpticalFlow {
		#[inline] pub fn as_raw_PtrOfDenseOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDenseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::video::DenseOpticalFlowConst for PtrOfDenseOpticalFlow {
		#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::DenseOpticalFlow for PtrOfDenseOpticalFlow {
		#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfDenseOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfDenseOpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFarnebackOpticalFlow = core::Ptr<dyn crate::video::FarnebackOpticalFlow>;
	
	ptr_extern! { dyn crate::video::FarnebackOpticalFlow,
		cv_PtrOfFarnebackOpticalFlow_delete, cv_PtrOfFarnebackOpticalFlow_get_inner_ptr, cv_PtrOfFarnebackOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfFarnebackOpticalFlow {
		#[inline] pub fn as_raw_PtrOfFarnebackOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFarnebackOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::video::FarnebackOpticalFlowConst for PtrOfFarnebackOpticalFlow {
		#[inline] fn as_raw_FarnebackOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::FarnebackOpticalFlow for PtrOfFarnebackOpticalFlow {
		#[inline] fn as_raw_mut_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfFarnebackOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfFarnebackOpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DenseOpticalFlowConst for PtrOfFarnebackOpticalFlow {
		#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::DenseOpticalFlow for PtrOfFarnebackOpticalFlow {
		#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSparseOpticalFlow = core::Ptr<dyn crate::video::SparseOpticalFlow>;
	
	ptr_extern! { dyn crate::video::SparseOpticalFlow,
		cv_PtrOfSparseOpticalFlow_delete, cv_PtrOfSparseOpticalFlow_get_inner_ptr, cv_PtrOfSparseOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfSparseOpticalFlow {
		#[inline] pub fn as_raw_PtrOfSparseOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSparseOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::video::SparseOpticalFlowConst for PtrOfSparseOpticalFlow {
		#[inline] fn as_raw_SparseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::SparseOpticalFlow for PtrOfSparseOpticalFlow {
		#[inline] fn as_raw_mut_SparseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSparseOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSparseOpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSparsePyrLKOpticalFlow = core::Ptr<dyn crate::video::SparsePyrLKOpticalFlow>;
	
	ptr_extern! { dyn crate::video::SparsePyrLKOpticalFlow,
		cv_PtrOfSparsePyrLKOpticalFlow_delete, cv_PtrOfSparsePyrLKOpticalFlow_get_inner_ptr, cv_PtrOfSparsePyrLKOpticalFlow_get_inner_ptr_mut
	}
	
	impl PtrOfSparsePyrLKOpticalFlow {
		#[inline] pub fn as_raw_PtrOfSparsePyrLKOpticalFlow(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSparsePyrLKOpticalFlow(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::video::SparsePyrLKOpticalFlowConst for PtrOfSparsePyrLKOpticalFlow {
		#[inline] fn as_raw_SparsePyrLKOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::SparsePyrLKOpticalFlow for PtrOfSparsePyrLKOpticalFlow {
		#[inline] fn as_raw_mut_SparsePyrLKOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSparsePyrLKOpticalFlow {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSparsePyrLKOpticalFlow {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::SparseOpticalFlowConst for PtrOfSparsePyrLKOpticalFlow {
		#[inline] fn as_raw_SparseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::SparseOpticalFlow for PtrOfSparsePyrLKOpticalFlow {
		#[inline] fn as_raw_mut_SparseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerDaSiamRPN = core::Ptr<dyn crate::video::TrackerDaSiamRPN>;
	
	ptr_extern! { dyn crate::video::TrackerDaSiamRPN,
		cv_PtrOfTrackerDaSiamRPN_delete, cv_PtrOfTrackerDaSiamRPN_get_inner_ptr, cv_PtrOfTrackerDaSiamRPN_get_inner_ptr_mut
	}
	
	impl PtrOfTrackerDaSiamRPN {
		#[inline] pub fn as_raw_PtrOfTrackerDaSiamRPN(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerDaSiamRPN(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::video::TrackerDaSiamRPNConst for PtrOfTrackerDaSiamRPN {
		#[inline] fn as_raw_TrackerDaSiamRPN(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::TrackerDaSiamRPN for PtrOfTrackerDaSiamRPN {
		#[inline] fn as_raw_mut_TrackerDaSiamRPN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::TrackerConst for PtrOfTrackerDaSiamRPN {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::Tracker for PtrOfTrackerDaSiamRPN {
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerGOTURN = core::Ptr<dyn crate::video::TrackerGOTURN>;
	
	ptr_extern! { dyn crate::video::TrackerGOTURN,
		cv_PtrOfTrackerGOTURN_delete, cv_PtrOfTrackerGOTURN_get_inner_ptr, cv_PtrOfTrackerGOTURN_get_inner_ptr_mut
	}
	
	impl PtrOfTrackerGOTURN {
		#[inline] pub fn as_raw_PtrOfTrackerGOTURN(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerGOTURN(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::video::TrackerGOTURNConst for PtrOfTrackerGOTURN {
		#[inline] fn as_raw_TrackerGOTURN(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::TrackerGOTURN for PtrOfTrackerGOTURN {
		#[inline] fn as_raw_mut_TrackerGOTURN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::TrackerConst for PtrOfTrackerGOTURN {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::Tracker for PtrOfTrackerGOTURN {
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerMIL = core::Ptr<dyn crate::video::TrackerMIL>;
	
	ptr_extern! { dyn crate::video::TrackerMIL,
		cv_PtrOfTrackerMIL_delete, cv_PtrOfTrackerMIL_get_inner_ptr, cv_PtrOfTrackerMIL_get_inner_ptr_mut
	}
	
	impl PtrOfTrackerMIL {
		#[inline] pub fn as_raw_PtrOfTrackerMIL(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerMIL(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::video::TrackerMILConst for PtrOfTrackerMIL {
		#[inline] fn as_raw_TrackerMIL(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::TrackerMIL for PtrOfTrackerMIL {
		#[inline] fn as_raw_mut_TrackerMIL(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::TrackerConst for PtrOfTrackerMIL {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::Tracker for PtrOfTrackerMIL {
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfVariationalRefinement = core::Ptr<dyn crate::video::VariationalRefinement>;
	
	ptr_extern! { dyn crate::video::VariationalRefinement,
		cv_PtrOfVariationalRefinement_delete, cv_PtrOfVariationalRefinement_get_inner_ptr, cv_PtrOfVariationalRefinement_get_inner_ptr_mut
	}
	
	impl PtrOfVariationalRefinement {
		#[inline] pub fn as_raw_PtrOfVariationalRefinement(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfVariationalRefinement(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::video::VariationalRefinementConst for PtrOfVariationalRefinement {
		#[inline] fn as_raw_VariationalRefinement(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::VariationalRefinement for PtrOfVariationalRefinement {
		#[inline] fn as_raw_mut_VariationalRefinement(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfVariationalRefinement {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfVariationalRefinement {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DenseOpticalFlowConst for PtrOfVariationalRefinement {
		#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::DenseOpticalFlow for PtrOfVariationalRefinement {
		#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_video)]
pub use video_types::*;

#[cfg(ocvrs_has_module_videoio)]
mod videoio_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type VectorOfVideoCapture = core::Vector<crate::videoio::VideoCapture>;
	
	impl VectorOfVideoCapture {
		pub fn as_raw_VectorOfVideoCapture(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVideoCapture(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::videoio::VideoCapture, *const c_void, *mut c_void,
		cv_VectorOfVideoCapture_new, cv_VectorOfVideoCapture_delete,
		cv_VectorOfVideoCapture_len, cv_VectorOfVideoCapture_is_empty,
		cv_VectorOfVideoCapture_capacity, cv_VectorOfVideoCapture_shrink_to_fit,
		cv_VectorOfVideoCapture_reserve, cv_VectorOfVideoCapture_remove,
		cv_VectorOfVideoCapture_swap, cv_VectorOfVideoCapture_clear,
		cv_VectorOfVideoCapture_get, cv_VectorOfVideoCapture_set,
		cv_VectorOfVideoCapture_push, cv_VectorOfVideoCapture_insert,
	}
	vector_non_copy_or_bool! { crate::videoio::VideoCapture }
	
	unsafe impl Send for core::Vector<crate::videoio::VideoCapture> {}
	
	pub type VectorOfVideoCaptureAPIs = core::Vector<crate::videoio::VideoCaptureAPIs>;
	
	impl VectorOfVideoCaptureAPIs {
		pub fn as_raw_VectorOfVideoCaptureAPIs(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfVideoCaptureAPIs(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::videoio::VideoCaptureAPIs, *const c_void, *mut c_void,
		cv_VectorOfVideoCaptureAPIs_new, cv_VectorOfVideoCaptureAPIs_delete,
		cv_VectorOfVideoCaptureAPIs_len, cv_VectorOfVideoCaptureAPIs_is_empty,
		cv_VectorOfVideoCaptureAPIs_capacity, cv_VectorOfVideoCaptureAPIs_shrink_to_fit,
		cv_VectorOfVideoCaptureAPIs_reserve, cv_VectorOfVideoCaptureAPIs_remove,
		cv_VectorOfVideoCaptureAPIs_swap, cv_VectorOfVideoCaptureAPIs_clear,
		cv_VectorOfVideoCaptureAPIs_get, cv_VectorOfVideoCaptureAPIs_set,
		cv_VectorOfVideoCaptureAPIs_push, cv_VectorOfVideoCaptureAPIs_insert,
	}
	vector_copy_non_bool! { crate::videoio::VideoCaptureAPIs, *const c_void, *mut c_void,
		cv_VectorOfVideoCaptureAPIs_data, cv_VectorOfVideoCaptureAPIs_data_mut, cv_VectorOfVideoCaptureAPIs_from_slice,
		cv_VectorOfVideoCaptureAPIs_clone,
	}
	
	unsafe impl Send for core::Vector<crate::videoio::VideoCaptureAPIs> {}
	
}
#[cfg(ocvrs_has_module_videoio)]
pub use videoio_types::*;

#[cfg(ocvrs_has_module_videostab)]
mod videostab_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfColorAverageInpainter = core::Ptr<crate::videostab::ColorAverageInpainter>;
	
	ptr_extern! { crate::videostab::ColorAverageInpainter,
		cv_PtrOfColorAverageInpainter_delete, cv_PtrOfColorAverageInpainter_get_inner_ptr, cv_PtrOfColorAverageInpainter_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::ColorAverageInpainter, cv_PtrOfColorAverageInpainter_new }
	
	impl PtrOfColorAverageInpainter {
		#[inline] pub fn as_raw_PtrOfColorAverageInpainter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfColorAverageInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::ColorAverageInpainterTraitConst for PtrOfColorAverageInpainter {
		#[inline] fn as_raw_ColorAverageInpainter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ColorAverageInpainterTrait for PtrOfColorAverageInpainter {
		#[inline] fn as_raw_mut_ColorAverageInpainter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::InpainterBaseConst for PtrOfColorAverageInpainter {
		#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::InpainterBase for PtrOfColorAverageInpainter {
		#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfColorAverageInpainter, core::Ptr<dyn crate::videostab::InpainterBase>,
		cv_PtrOfColorAverageInpainter_to_PtrOfInpainterBase,
	}
	
	pub type PtrOfColorInpainter = core::Ptr<crate::videostab::ColorInpainter>;
	
	ptr_extern! { crate::videostab::ColorInpainter,
		cv_PtrOfColorInpainter_delete, cv_PtrOfColorInpainter_get_inner_ptr, cv_PtrOfColorInpainter_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::ColorInpainter, cv_PtrOfColorInpainter_new }
	
	impl PtrOfColorInpainter {
		#[inline] pub fn as_raw_PtrOfColorInpainter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfColorInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::ColorInpainterTraitConst for PtrOfColorInpainter {
		#[inline] fn as_raw_ColorInpainter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ColorInpainterTrait for PtrOfColorInpainter {
		#[inline] fn as_raw_mut_ColorInpainter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::InpainterBaseConst for PtrOfColorInpainter {
		#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::InpainterBase for PtrOfColorInpainter {
		#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfColorInpainter, core::Ptr<dyn crate::videostab::InpainterBase>,
		cv_PtrOfColorInpainter_to_PtrOfInpainterBase,
	}
	
	pub type PtrOfConsistentMosaicInpainter = core::Ptr<crate::videostab::ConsistentMosaicInpainter>;
	
	ptr_extern! { crate::videostab::ConsistentMosaicInpainter,
		cv_PtrOfConsistentMosaicInpainter_delete, cv_PtrOfConsistentMosaicInpainter_get_inner_ptr, cv_PtrOfConsistentMosaicInpainter_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::ConsistentMosaicInpainter, cv_PtrOfConsistentMosaicInpainter_new }
	
	impl PtrOfConsistentMosaicInpainter {
		#[inline] pub fn as_raw_PtrOfConsistentMosaicInpainter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConsistentMosaicInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::ConsistentMosaicInpainterTraitConst for PtrOfConsistentMosaicInpainter {
		#[inline] fn as_raw_ConsistentMosaicInpainter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ConsistentMosaicInpainterTrait for PtrOfConsistentMosaicInpainter {
		#[inline] fn as_raw_mut_ConsistentMosaicInpainter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::InpainterBaseConst for PtrOfConsistentMosaicInpainter {
		#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::InpainterBase for PtrOfConsistentMosaicInpainter {
		#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfConsistentMosaicInpainter, core::Ptr<dyn crate::videostab::InpainterBase>,
		cv_PtrOfConsistentMosaicInpainter_to_PtrOfInpainterBase,
	}
	
	pub type PtrOfDeblurerBase = core::Ptr<dyn crate::videostab::DeblurerBase>;
	
	ptr_extern! { dyn crate::videostab::DeblurerBase,
		cv_PtrOfDeblurerBase_delete, cv_PtrOfDeblurerBase_get_inner_ptr, cv_PtrOfDeblurerBase_get_inner_ptr_mut
	}
	
	impl PtrOfDeblurerBase {
		#[inline] pub fn as_raw_PtrOfDeblurerBase(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDeblurerBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::DeblurerBaseConst for PtrOfDeblurerBase {
		#[inline] fn as_raw_DeblurerBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::DeblurerBase for PtrOfDeblurerBase {
		#[inline] fn as_raw_mut_DeblurerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDensePyrLkOptFlowEstimatorGpu = core::Ptr<crate::videostab::DensePyrLkOptFlowEstimatorGpu>;
	
	ptr_extern! { crate::videostab::DensePyrLkOptFlowEstimatorGpu,
		cv_PtrOfDensePyrLkOptFlowEstimatorGpu_delete, cv_PtrOfDensePyrLkOptFlowEstimatorGpu_get_inner_ptr, cv_PtrOfDensePyrLkOptFlowEstimatorGpu_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::DensePyrLkOptFlowEstimatorGpu, cv_PtrOfDensePyrLkOptFlowEstimatorGpu_new }
	
	impl PtrOfDensePyrLkOptFlowEstimatorGpu {
		#[inline] pub fn as_raw_PtrOfDensePyrLkOptFlowEstimatorGpu(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDensePyrLkOptFlowEstimatorGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::DensePyrLkOptFlowEstimatorGpuTraitConst for PtrOfDensePyrLkOptFlowEstimatorGpu {
		#[inline] fn as_raw_DensePyrLkOptFlowEstimatorGpu(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::DensePyrLkOptFlowEstimatorGpuTrait for PtrOfDensePyrLkOptFlowEstimatorGpu {
		#[inline] fn as_raw_mut_DensePyrLkOptFlowEstimatorGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IDenseOptFlowEstimatorConst for PtrOfDensePyrLkOptFlowEstimatorGpu {
		#[inline] fn as_raw_IDenseOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IDenseOptFlowEstimator for PtrOfDensePyrLkOptFlowEstimatorGpu {
		#[inline] fn as_raw_mut_IDenseOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDensePyrLkOptFlowEstimatorGpu, core::Ptr<dyn crate::videostab::IDenseOptFlowEstimator>,
		cv_PtrOfDensePyrLkOptFlowEstimatorGpu_to_PtrOfIDenseOptFlowEstimator,
	}
	
	impl crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst for PtrOfDensePyrLkOptFlowEstimatorGpu {
		#[inline] fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::PyrLkOptFlowEstimatorBaseTrait for PtrOfDensePyrLkOptFlowEstimatorGpu {
		#[inline] fn as_raw_mut_PyrLkOptFlowEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFromFileMotionReader = core::Ptr<crate::videostab::FromFileMotionReader>;
	
	ptr_extern! { crate::videostab::FromFileMotionReader,
		cv_PtrOfFromFileMotionReader_delete, cv_PtrOfFromFileMotionReader_get_inner_ptr, cv_PtrOfFromFileMotionReader_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::FromFileMotionReader, cv_PtrOfFromFileMotionReader_new }
	
	impl PtrOfFromFileMotionReader {
		#[inline] pub fn as_raw_PtrOfFromFileMotionReader(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFromFileMotionReader(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::FromFileMotionReaderTraitConst for PtrOfFromFileMotionReader {
		#[inline] fn as_raw_FromFileMotionReader(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::FromFileMotionReaderTrait for PtrOfFromFileMotionReader {
		#[inline] fn as_raw_mut_FromFileMotionReader(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::ImageMotionEstimatorBaseConst for PtrOfFromFileMotionReader {
		#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ImageMotionEstimatorBase for PtrOfFromFileMotionReader {
		#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfFromFileMotionReader, core::Ptr<dyn crate::videostab::ImageMotionEstimatorBase>,
		cv_PtrOfFromFileMotionReader_to_PtrOfImageMotionEstimatorBase,
	}
	
	pub type PtrOfGaussianMotionFilter = core::Ptr<crate::videostab::GaussianMotionFilter>;
	
	ptr_extern! { crate::videostab::GaussianMotionFilter,
		cv_PtrOfGaussianMotionFilter_delete, cv_PtrOfGaussianMotionFilter_get_inner_ptr, cv_PtrOfGaussianMotionFilter_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::GaussianMotionFilter, cv_PtrOfGaussianMotionFilter_new }
	
	impl PtrOfGaussianMotionFilter {
		#[inline] pub fn as_raw_PtrOfGaussianMotionFilter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGaussianMotionFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::GaussianMotionFilterTraitConst for PtrOfGaussianMotionFilter {
		#[inline] fn as_raw_GaussianMotionFilter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::GaussianMotionFilterTrait for PtrOfGaussianMotionFilter {
		#[inline] fn as_raw_mut_GaussianMotionFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IMotionStabilizerConst for PtrOfGaussianMotionFilter {
		#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IMotionStabilizer for PtrOfGaussianMotionFilter {
		#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfGaussianMotionFilter, core::Ptr<dyn crate::videostab::IMotionStabilizer>,
		cv_PtrOfGaussianMotionFilter_to_PtrOfIMotionStabilizer,
	}
	
	impl crate::videostab::MotionFilterBaseConst for PtrOfGaussianMotionFilter {
		#[inline] fn as_raw_MotionFilterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MotionFilterBase for PtrOfGaussianMotionFilter {
		#[inline] fn as_raw_mut_MotionFilterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfGaussianMotionFilter, core::Ptr<dyn crate::videostab::MotionFilterBase>,
		cv_PtrOfGaussianMotionFilter_to_PtrOfMotionFilterBase,
	}
	
	pub type PtrOfIDenseOptFlowEstimator = core::Ptr<dyn crate::videostab::IDenseOptFlowEstimator>;
	
	ptr_extern! { dyn crate::videostab::IDenseOptFlowEstimator,
		cv_PtrOfIDenseOptFlowEstimator_delete, cv_PtrOfIDenseOptFlowEstimator_get_inner_ptr, cv_PtrOfIDenseOptFlowEstimator_get_inner_ptr_mut
	}
	
	impl PtrOfIDenseOptFlowEstimator {
		#[inline] pub fn as_raw_PtrOfIDenseOptFlowEstimator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfIDenseOptFlowEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::IDenseOptFlowEstimatorConst for PtrOfIDenseOptFlowEstimator {
		#[inline] fn as_raw_IDenseOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IDenseOptFlowEstimator for PtrOfIDenseOptFlowEstimator {
		#[inline] fn as_raw_mut_IDenseOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfIFrameSource = core::Ptr<dyn crate::videostab::IFrameSource>;
	
	ptr_extern! { dyn crate::videostab::IFrameSource,
		cv_PtrOfIFrameSource_delete, cv_PtrOfIFrameSource_get_inner_ptr, cv_PtrOfIFrameSource_get_inner_ptr_mut
	}
	
	impl PtrOfIFrameSource {
		#[inline] pub fn as_raw_PtrOfIFrameSource(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfIFrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::IFrameSourceConst for PtrOfIFrameSource {
		#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IFrameSource for PtrOfIFrameSource {
		#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfILog = core::Ptr<dyn crate::videostab::ILog>;
	
	ptr_extern! { dyn crate::videostab::ILog,
		cv_PtrOfILog_delete, cv_PtrOfILog_get_inner_ptr, cv_PtrOfILog_get_inner_ptr_mut
	}
	
	impl PtrOfILog {
		#[inline] pub fn as_raw_PtrOfILog(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfILog(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::ILogConst for PtrOfILog {
		#[inline] fn as_raw_ILog(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ILog for PtrOfILog {
		#[inline] fn as_raw_mut_ILog(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfIMotionStabilizer = core::Ptr<dyn crate::videostab::IMotionStabilizer>;
	
	ptr_extern! { dyn crate::videostab::IMotionStabilizer,
		cv_PtrOfIMotionStabilizer_delete, cv_PtrOfIMotionStabilizer_get_inner_ptr, cv_PtrOfIMotionStabilizer_get_inner_ptr_mut
	}
	
	impl PtrOfIMotionStabilizer {
		#[inline] pub fn as_raw_PtrOfIMotionStabilizer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfIMotionStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::IMotionStabilizerConst for PtrOfIMotionStabilizer {
		#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IMotionStabilizer for PtrOfIMotionStabilizer {
		#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfIOutlierRejector = core::Ptr<dyn crate::videostab::IOutlierRejector>;
	
	ptr_extern! { dyn crate::videostab::IOutlierRejector,
		cv_PtrOfIOutlierRejector_delete, cv_PtrOfIOutlierRejector_get_inner_ptr, cv_PtrOfIOutlierRejector_get_inner_ptr_mut
	}
	
	impl PtrOfIOutlierRejector {
		#[inline] pub fn as_raw_PtrOfIOutlierRejector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfIOutlierRejector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::IOutlierRejectorConst for PtrOfIOutlierRejector {
		#[inline] fn as_raw_IOutlierRejector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IOutlierRejector for PtrOfIOutlierRejector {
		#[inline] fn as_raw_mut_IOutlierRejector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfISparseOptFlowEstimator = core::Ptr<dyn crate::videostab::ISparseOptFlowEstimator>;
	
	ptr_extern! { dyn crate::videostab::ISparseOptFlowEstimator,
		cv_PtrOfISparseOptFlowEstimator_delete, cv_PtrOfISparseOptFlowEstimator_get_inner_ptr, cv_PtrOfISparseOptFlowEstimator_get_inner_ptr_mut
	}
	
	impl PtrOfISparseOptFlowEstimator {
		#[inline] pub fn as_raw_PtrOfISparseOptFlowEstimator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfISparseOptFlowEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::ISparseOptFlowEstimatorConst for PtrOfISparseOptFlowEstimator {
		#[inline] fn as_raw_ISparseOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ISparseOptFlowEstimator for PtrOfISparseOptFlowEstimator {
		#[inline] fn as_raw_mut_ISparseOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfImageMotionEstimatorBase = core::Ptr<dyn crate::videostab::ImageMotionEstimatorBase>;
	
	ptr_extern! { dyn crate::videostab::ImageMotionEstimatorBase,
		cv_PtrOfImageMotionEstimatorBase_delete, cv_PtrOfImageMotionEstimatorBase_get_inner_ptr, cv_PtrOfImageMotionEstimatorBase_get_inner_ptr_mut
	}
	
	impl PtrOfImageMotionEstimatorBase {
		#[inline] pub fn as_raw_PtrOfImageMotionEstimatorBase(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfImageMotionEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::ImageMotionEstimatorBaseConst for PtrOfImageMotionEstimatorBase {
		#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ImageMotionEstimatorBase for PtrOfImageMotionEstimatorBase {
		#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfInpainterBase = core::Ptr<dyn crate::videostab::InpainterBase>;
	
	ptr_extern! { dyn crate::videostab::InpainterBase,
		cv_PtrOfInpainterBase_delete, cv_PtrOfInpainterBase_get_inner_ptr, cv_PtrOfInpainterBase_get_inner_ptr_mut
	}
	
	impl PtrOfInpainterBase {
		#[inline] pub fn as_raw_PtrOfInpainterBase(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfInpainterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::InpainterBaseConst for PtrOfInpainterBase {
		#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::InpainterBase for PtrOfInpainterBase {
		#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfInpaintingPipeline = core::Ptr<crate::videostab::InpaintingPipeline>;
	
	ptr_extern! { crate::videostab::InpaintingPipeline,
		cv_PtrOfInpaintingPipeline_delete, cv_PtrOfInpaintingPipeline_get_inner_ptr, cv_PtrOfInpaintingPipeline_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::InpaintingPipeline, cv_PtrOfInpaintingPipeline_new }
	
	impl PtrOfInpaintingPipeline {
		#[inline] pub fn as_raw_PtrOfInpaintingPipeline(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfInpaintingPipeline(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::InpaintingPipelineTraitConst for PtrOfInpaintingPipeline {
		#[inline] fn as_raw_InpaintingPipeline(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::InpaintingPipelineTrait for PtrOfInpaintingPipeline {
		#[inline] fn as_raw_mut_InpaintingPipeline(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::InpainterBaseConst for PtrOfInpaintingPipeline {
		#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::InpainterBase for PtrOfInpaintingPipeline {
		#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfInpaintingPipeline, core::Ptr<dyn crate::videostab::InpainterBase>,
		cv_PtrOfInpaintingPipeline_to_PtrOfInpainterBase,
	}
	
	pub type PtrOfKeypointBasedMotionEstimator = core::Ptr<crate::videostab::KeypointBasedMotionEstimator>;
	
	ptr_extern! { crate::videostab::KeypointBasedMotionEstimator,
		cv_PtrOfKeypointBasedMotionEstimator_delete, cv_PtrOfKeypointBasedMotionEstimator_get_inner_ptr, cv_PtrOfKeypointBasedMotionEstimator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::KeypointBasedMotionEstimator, cv_PtrOfKeypointBasedMotionEstimator_new }
	
	impl PtrOfKeypointBasedMotionEstimator {
		#[inline] pub fn as_raw_PtrOfKeypointBasedMotionEstimator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKeypointBasedMotionEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::KeypointBasedMotionEstimatorTraitConst for PtrOfKeypointBasedMotionEstimator {
		#[inline] fn as_raw_KeypointBasedMotionEstimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::KeypointBasedMotionEstimatorTrait for PtrOfKeypointBasedMotionEstimator {
		#[inline] fn as_raw_mut_KeypointBasedMotionEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::ImageMotionEstimatorBaseConst for PtrOfKeypointBasedMotionEstimator {
		#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ImageMotionEstimatorBase for PtrOfKeypointBasedMotionEstimator {
		#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfKeypointBasedMotionEstimator, core::Ptr<dyn crate::videostab::ImageMotionEstimatorBase>,
		cv_PtrOfKeypointBasedMotionEstimator_to_PtrOfImageMotionEstimatorBase,
	}
	
	pub type PtrOfKeypointBasedMotionEstimatorGpu = core::Ptr<crate::videostab::KeypointBasedMotionEstimatorGpu>;
	
	ptr_extern! { crate::videostab::KeypointBasedMotionEstimatorGpu,
		cv_PtrOfKeypointBasedMotionEstimatorGpu_delete, cv_PtrOfKeypointBasedMotionEstimatorGpu_get_inner_ptr, cv_PtrOfKeypointBasedMotionEstimatorGpu_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::KeypointBasedMotionEstimatorGpu, cv_PtrOfKeypointBasedMotionEstimatorGpu_new }
	
	impl PtrOfKeypointBasedMotionEstimatorGpu {
		#[inline] pub fn as_raw_PtrOfKeypointBasedMotionEstimatorGpu(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKeypointBasedMotionEstimatorGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::KeypointBasedMotionEstimatorGpuTraitConst for PtrOfKeypointBasedMotionEstimatorGpu {
		#[inline] fn as_raw_KeypointBasedMotionEstimatorGpu(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::KeypointBasedMotionEstimatorGpuTrait for PtrOfKeypointBasedMotionEstimatorGpu {
		#[inline] fn as_raw_mut_KeypointBasedMotionEstimatorGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::ImageMotionEstimatorBaseConst for PtrOfKeypointBasedMotionEstimatorGpu {
		#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ImageMotionEstimatorBase for PtrOfKeypointBasedMotionEstimatorGpu {
		#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfKeypointBasedMotionEstimatorGpu, core::Ptr<dyn crate::videostab::ImageMotionEstimatorBase>,
		cv_PtrOfKeypointBasedMotionEstimatorGpu_to_PtrOfImageMotionEstimatorBase,
	}
	
	pub type PtrOfLogToStdout = core::Ptr<crate::videostab::LogToStdout>;
	
	ptr_extern! { crate::videostab::LogToStdout,
		cv_PtrOfLogToStdout_delete, cv_PtrOfLogToStdout_get_inner_ptr, cv_PtrOfLogToStdout_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::LogToStdout, cv_PtrOfLogToStdout_new }
	
	impl PtrOfLogToStdout {
		#[inline] pub fn as_raw_PtrOfLogToStdout(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLogToStdout(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::LogToStdoutTraitConst for PtrOfLogToStdout {
		#[inline] fn as_raw_LogToStdout(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::LogToStdoutTrait for PtrOfLogToStdout {
		#[inline] fn as_raw_mut_LogToStdout(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::ILogConst for PtrOfLogToStdout {
		#[inline] fn as_raw_ILog(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ILog for PtrOfLogToStdout {
		#[inline] fn as_raw_mut_ILog(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfLogToStdout, core::Ptr<dyn crate::videostab::ILog>,
		cv_PtrOfLogToStdout_to_PtrOfILog,
	}
	
	pub type PtrOfLpMotionStabilizer = core::Ptr<crate::videostab::LpMotionStabilizer>;
	
	ptr_extern! { crate::videostab::LpMotionStabilizer,
		cv_PtrOfLpMotionStabilizer_delete, cv_PtrOfLpMotionStabilizer_get_inner_ptr, cv_PtrOfLpMotionStabilizer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::LpMotionStabilizer, cv_PtrOfLpMotionStabilizer_new }
	
	impl PtrOfLpMotionStabilizer {
		#[inline] pub fn as_raw_PtrOfLpMotionStabilizer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLpMotionStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::LpMotionStabilizerTraitConst for PtrOfLpMotionStabilizer {
		#[inline] fn as_raw_LpMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::LpMotionStabilizerTrait for PtrOfLpMotionStabilizer {
		#[inline] fn as_raw_mut_LpMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IMotionStabilizerConst for PtrOfLpMotionStabilizer {
		#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IMotionStabilizer for PtrOfLpMotionStabilizer {
		#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfLpMotionStabilizer, core::Ptr<dyn crate::videostab::IMotionStabilizer>,
		cv_PtrOfLpMotionStabilizer_to_PtrOfIMotionStabilizer,
	}
	
	pub type PtrOfMaskFrameSource = core::Ptr<crate::videostab::MaskFrameSource>;
	
	ptr_extern! { crate::videostab::MaskFrameSource,
		cv_PtrOfMaskFrameSource_delete, cv_PtrOfMaskFrameSource_get_inner_ptr, cv_PtrOfMaskFrameSource_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::MaskFrameSource, cv_PtrOfMaskFrameSource_new }
	
	impl PtrOfMaskFrameSource {
		#[inline] pub fn as_raw_PtrOfMaskFrameSource(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMaskFrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::MaskFrameSourceTraitConst for PtrOfMaskFrameSource {
		#[inline] fn as_raw_MaskFrameSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MaskFrameSourceTrait for PtrOfMaskFrameSource {
		#[inline] fn as_raw_mut_MaskFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IFrameSourceConst for PtrOfMaskFrameSource {
		#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IFrameSource for PtrOfMaskFrameSource {
		#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMaskFrameSource, core::Ptr<dyn crate::videostab::IFrameSource>,
		cv_PtrOfMaskFrameSource_to_PtrOfIFrameSource,
	}
	
	pub type PtrOfMoreAccurateMotionWobbleSuppressorBase = core::Ptr<dyn crate::videostab::MoreAccurateMotionWobbleSuppressorBase>;
	
	ptr_extern! { dyn crate::videostab::MoreAccurateMotionWobbleSuppressorBase,
		cv_PtrOfMoreAccurateMotionWobbleSuppressorBase_delete, cv_PtrOfMoreAccurateMotionWobbleSuppressorBase_get_inner_ptr, cv_PtrOfMoreAccurateMotionWobbleSuppressorBase_get_inner_ptr_mut
	}
	
	impl PtrOfMoreAccurateMotionWobbleSuppressorBase {
		#[inline] pub fn as_raw_PtrOfMoreAccurateMotionWobbleSuppressorBase(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMoreAccurateMotionWobbleSuppressorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::MoreAccurateMotionWobbleSuppressorBaseConst for PtrOfMoreAccurateMotionWobbleSuppressorBase {
		#[inline] fn as_raw_MoreAccurateMotionWobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MoreAccurateMotionWobbleSuppressorBase for PtrOfMoreAccurateMotionWobbleSuppressorBase {
		#[inline] fn as_raw_mut_MoreAccurateMotionWobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::WobbleSuppressorBaseConst for PtrOfMoreAccurateMotionWobbleSuppressorBase {
		#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::WobbleSuppressorBase for PtrOfMoreAccurateMotionWobbleSuppressorBase {
		#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMoreAccurateMotionWobbleSuppressorBase, core::Ptr<dyn crate::videostab::WobbleSuppressorBase>,
		cv_PtrOfMoreAccurateMotionWobbleSuppressorBase_to_PtrOfWobbleSuppressorBase,
	}
	
	pub type PtrOfMotionEstimatorBase = core::Ptr<dyn crate::videostab::MotionEstimatorBase>;
	
	ptr_extern! { dyn crate::videostab::MotionEstimatorBase,
		cv_PtrOfMotionEstimatorBase_delete, cv_PtrOfMotionEstimatorBase_get_inner_ptr, cv_PtrOfMotionEstimatorBase_get_inner_ptr_mut
	}
	
	impl PtrOfMotionEstimatorBase {
		#[inline] pub fn as_raw_PtrOfMotionEstimatorBase(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMotionEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::MotionEstimatorBaseConst for PtrOfMotionEstimatorBase {
		#[inline] fn as_raw_MotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MotionEstimatorBase for PtrOfMotionEstimatorBase {
		#[inline] fn as_raw_mut_MotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMotionEstimatorL1 = core::Ptr<crate::videostab::MotionEstimatorL1>;
	
	ptr_extern! { crate::videostab::MotionEstimatorL1,
		cv_PtrOfMotionEstimatorL1_delete, cv_PtrOfMotionEstimatorL1_get_inner_ptr, cv_PtrOfMotionEstimatorL1_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::MotionEstimatorL1, cv_PtrOfMotionEstimatorL1_new }
	
	impl PtrOfMotionEstimatorL1 {
		#[inline] pub fn as_raw_PtrOfMotionEstimatorL1(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMotionEstimatorL1(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::MotionEstimatorL1TraitConst for PtrOfMotionEstimatorL1 {
		#[inline] fn as_raw_MotionEstimatorL1(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MotionEstimatorL1Trait for PtrOfMotionEstimatorL1 {
		#[inline] fn as_raw_mut_MotionEstimatorL1(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::MotionEstimatorBaseConst for PtrOfMotionEstimatorL1 {
		#[inline] fn as_raw_MotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MotionEstimatorBase for PtrOfMotionEstimatorL1 {
		#[inline] fn as_raw_mut_MotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMotionEstimatorL1, core::Ptr<dyn crate::videostab::MotionEstimatorBase>,
		cv_PtrOfMotionEstimatorL1_to_PtrOfMotionEstimatorBase,
	}
	
	pub type PtrOfMotionEstimatorRansacL2 = core::Ptr<crate::videostab::MotionEstimatorRansacL2>;
	
	ptr_extern! { crate::videostab::MotionEstimatorRansacL2,
		cv_PtrOfMotionEstimatorRansacL2_delete, cv_PtrOfMotionEstimatorRansacL2_get_inner_ptr, cv_PtrOfMotionEstimatorRansacL2_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::MotionEstimatorRansacL2, cv_PtrOfMotionEstimatorRansacL2_new }
	
	impl PtrOfMotionEstimatorRansacL2 {
		#[inline] pub fn as_raw_PtrOfMotionEstimatorRansacL2(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMotionEstimatorRansacL2(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::MotionEstimatorRansacL2TraitConst for PtrOfMotionEstimatorRansacL2 {
		#[inline] fn as_raw_MotionEstimatorRansacL2(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MotionEstimatorRansacL2Trait for PtrOfMotionEstimatorRansacL2 {
		#[inline] fn as_raw_mut_MotionEstimatorRansacL2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::MotionEstimatorBaseConst for PtrOfMotionEstimatorRansacL2 {
		#[inline] fn as_raw_MotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MotionEstimatorBase for PtrOfMotionEstimatorRansacL2 {
		#[inline] fn as_raw_mut_MotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMotionEstimatorRansacL2, core::Ptr<dyn crate::videostab::MotionEstimatorBase>,
		cv_PtrOfMotionEstimatorRansacL2_to_PtrOfMotionEstimatorBase,
	}
	
	pub type PtrOfMotionFilterBase = core::Ptr<dyn crate::videostab::MotionFilterBase>;
	
	ptr_extern! { dyn crate::videostab::MotionFilterBase,
		cv_PtrOfMotionFilterBase_delete, cv_PtrOfMotionFilterBase_get_inner_ptr, cv_PtrOfMotionFilterBase_get_inner_ptr_mut
	}
	
	impl PtrOfMotionFilterBase {
		#[inline] pub fn as_raw_PtrOfMotionFilterBase(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMotionFilterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::MotionFilterBaseConst for PtrOfMotionFilterBase {
		#[inline] fn as_raw_MotionFilterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MotionFilterBase for PtrOfMotionFilterBase {
		#[inline] fn as_raw_mut_MotionFilterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IMotionStabilizerConst for PtrOfMotionFilterBase {
		#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IMotionStabilizer for PtrOfMotionFilterBase {
		#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMotionFilterBase, core::Ptr<dyn crate::videostab::IMotionStabilizer>,
		cv_PtrOfMotionFilterBase_to_PtrOfIMotionStabilizer,
	}
	
	pub type PtrOfMotionInpainter = core::Ptr<crate::videostab::MotionInpainter>;
	
	ptr_extern! { crate::videostab::MotionInpainter,
		cv_PtrOfMotionInpainter_delete, cv_PtrOfMotionInpainter_get_inner_ptr, cv_PtrOfMotionInpainter_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::MotionInpainter, cv_PtrOfMotionInpainter_new }
	
	impl PtrOfMotionInpainter {
		#[inline] pub fn as_raw_PtrOfMotionInpainter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMotionInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::MotionInpainterTraitConst for PtrOfMotionInpainter {
		#[inline] fn as_raw_MotionInpainter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MotionInpainterTrait for PtrOfMotionInpainter {
		#[inline] fn as_raw_mut_MotionInpainter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::InpainterBaseConst for PtrOfMotionInpainter {
		#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::InpainterBase for PtrOfMotionInpainter {
		#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMotionInpainter, core::Ptr<dyn crate::videostab::InpainterBase>,
		cv_PtrOfMotionInpainter_to_PtrOfInpainterBase,
	}
	
	pub type PtrOfMotionStabilizationPipeline = core::Ptr<crate::videostab::MotionStabilizationPipeline>;
	
	ptr_extern! { crate::videostab::MotionStabilizationPipeline,
		cv_PtrOfMotionStabilizationPipeline_delete, cv_PtrOfMotionStabilizationPipeline_get_inner_ptr, cv_PtrOfMotionStabilizationPipeline_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::MotionStabilizationPipeline, cv_PtrOfMotionStabilizationPipeline_new }
	
	impl PtrOfMotionStabilizationPipeline {
		#[inline] pub fn as_raw_PtrOfMotionStabilizationPipeline(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMotionStabilizationPipeline(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::MotionStabilizationPipelineTraitConst for PtrOfMotionStabilizationPipeline {
		#[inline] fn as_raw_MotionStabilizationPipeline(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MotionStabilizationPipelineTrait for PtrOfMotionStabilizationPipeline {
		#[inline] fn as_raw_mut_MotionStabilizationPipeline(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IMotionStabilizerConst for PtrOfMotionStabilizationPipeline {
		#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IMotionStabilizer for PtrOfMotionStabilizationPipeline {
		#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMotionStabilizationPipeline, core::Ptr<dyn crate::videostab::IMotionStabilizer>,
		cv_PtrOfMotionStabilizationPipeline_to_PtrOfIMotionStabilizer,
	}
	
	pub type PtrOfNullDeblurer = core::Ptr<crate::videostab::NullDeblurer>;
	
	ptr_extern! { crate::videostab::NullDeblurer,
		cv_PtrOfNullDeblurer_delete, cv_PtrOfNullDeblurer_get_inner_ptr, cv_PtrOfNullDeblurer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::NullDeblurer, cv_PtrOfNullDeblurer_new }
	
	impl PtrOfNullDeblurer {
		#[inline] pub fn as_raw_PtrOfNullDeblurer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNullDeblurer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::NullDeblurerTraitConst for PtrOfNullDeblurer {
		#[inline] fn as_raw_NullDeblurer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::NullDeblurerTrait for PtrOfNullDeblurer {
		#[inline] fn as_raw_mut_NullDeblurer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::DeblurerBaseConst for PtrOfNullDeblurer {
		#[inline] fn as_raw_DeblurerBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::DeblurerBase for PtrOfNullDeblurer {
		#[inline] fn as_raw_mut_DeblurerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfNullDeblurer, core::Ptr<dyn crate::videostab::DeblurerBase>,
		cv_PtrOfNullDeblurer_to_PtrOfDeblurerBase,
	}
	
	pub type PtrOfNullFrameSource = core::Ptr<crate::videostab::NullFrameSource>;
	
	ptr_extern! { crate::videostab::NullFrameSource,
		cv_PtrOfNullFrameSource_delete, cv_PtrOfNullFrameSource_get_inner_ptr, cv_PtrOfNullFrameSource_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::NullFrameSource, cv_PtrOfNullFrameSource_new }
	
	impl PtrOfNullFrameSource {
		#[inline] pub fn as_raw_PtrOfNullFrameSource(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNullFrameSource(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::NullFrameSourceTraitConst for PtrOfNullFrameSource {
		#[inline] fn as_raw_NullFrameSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::NullFrameSourceTrait for PtrOfNullFrameSource {
		#[inline] fn as_raw_mut_NullFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IFrameSourceConst for PtrOfNullFrameSource {
		#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IFrameSource for PtrOfNullFrameSource {
		#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfNullFrameSource, core::Ptr<dyn crate::videostab::IFrameSource>,
		cv_PtrOfNullFrameSource_to_PtrOfIFrameSource,
	}
	
	pub type PtrOfNullInpainter = core::Ptr<crate::videostab::NullInpainter>;
	
	ptr_extern! { crate::videostab::NullInpainter,
		cv_PtrOfNullInpainter_delete, cv_PtrOfNullInpainter_get_inner_ptr, cv_PtrOfNullInpainter_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::NullInpainter, cv_PtrOfNullInpainter_new }
	
	impl PtrOfNullInpainter {
		#[inline] pub fn as_raw_PtrOfNullInpainter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNullInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::NullInpainterTraitConst for PtrOfNullInpainter {
		#[inline] fn as_raw_NullInpainter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::NullInpainterTrait for PtrOfNullInpainter {
		#[inline] fn as_raw_mut_NullInpainter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::InpainterBaseConst for PtrOfNullInpainter {
		#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::InpainterBase for PtrOfNullInpainter {
		#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfNullInpainter, core::Ptr<dyn crate::videostab::InpainterBase>,
		cv_PtrOfNullInpainter_to_PtrOfInpainterBase,
	}
	
	pub type PtrOfNullLog = core::Ptr<crate::videostab::NullLog>;
	
	ptr_extern! { crate::videostab::NullLog,
		cv_PtrOfNullLog_delete, cv_PtrOfNullLog_get_inner_ptr, cv_PtrOfNullLog_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::NullLog, cv_PtrOfNullLog_new }
	
	impl PtrOfNullLog {
		#[inline] pub fn as_raw_PtrOfNullLog(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNullLog(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::NullLogTraitConst for PtrOfNullLog {
		#[inline] fn as_raw_NullLog(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::NullLogTrait for PtrOfNullLog {
		#[inline] fn as_raw_mut_NullLog(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::ILogConst for PtrOfNullLog {
		#[inline] fn as_raw_ILog(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ILog for PtrOfNullLog {
		#[inline] fn as_raw_mut_ILog(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfNullLog, core::Ptr<dyn crate::videostab::ILog>,
		cv_PtrOfNullLog_to_PtrOfILog,
	}
	
	pub type PtrOfNullOutlierRejector = core::Ptr<crate::videostab::NullOutlierRejector>;
	
	ptr_extern! { crate::videostab::NullOutlierRejector,
		cv_PtrOfNullOutlierRejector_delete, cv_PtrOfNullOutlierRejector_get_inner_ptr, cv_PtrOfNullOutlierRejector_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::NullOutlierRejector, cv_PtrOfNullOutlierRejector_new }
	
	impl PtrOfNullOutlierRejector {
		#[inline] pub fn as_raw_PtrOfNullOutlierRejector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNullOutlierRejector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::NullOutlierRejectorTraitConst for PtrOfNullOutlierRejector {
		#[inline] fn as_raw_NullOutlierRejector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::NullOutlierRejectorTrait for PtrOfNullOutlierRejector {
		#[inline] fn as_raw_mut_NullOutlierRejector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IOutlierRejectorConst for PtrOfNullOutlierRejector {
		#[inline] fn as_raw_IOutlierRejector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IOutlierRejector for PtrOfNullOutlierRejector {
		#[inline] fn as_raw_mut_IOutlierRejector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfNullOutlierRejector, core::Ptr<dyn crate::videostab::IOutlierRejector>,
		cv_PtrOfNullOutlierRejector_to_PtrOfIOutlierRejector,
	}
	
	pub type PtrOfNullWobbleSuppressor = core::Ptr<crate::videostab::NullWobbleSuppressor>;
	
	ptr_extern! { crate::videostab::NullWobbleSuppressor,
		cv_PtrOfNullWobbleSuppressor_delete, cv_PtrOfNullWobbleSuppressor_get_inner_ptr, cv_PtrOfNullWobbleSuppressor_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::NullWobbleSuppressor, cv_PtrOfNullWobbleSuppressor_new }
	
	impl PtrOfNullWobbleSuppressor {
		#[inline] pub fn as_raw_PtrOfNullWobbleSuppressor(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNullWobbleSuppressor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::NullWobbleSuppressorTraitConst for PtrOfNullWobbleSuppressor {
		#[inline] fn as_raw_NullWobbleSuppressor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::NullWobbleSuppressorTrait for PtrOfNullWobbleSuppressor {
		#[inline] fn as_raw_mut_NullWobbleSuppressor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::WobbleSuppressorBaseConst for PtrOfNullWobbleSuppressor {
		#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::WobbleSuppressorBase for PtrOfNullWobbleSuppressor {
		#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfNullWobbleSuppressor, core::Ptr<dyn crate::videostab::WobbleSuppressorBase>,
		cv_PtrOfNullWobbleSuppressor_to_PtrOfWobbleSuppressorBase,
	}
	
	pub type PtrOfOnePassStabilizer = core::Ptr<crate::videostab::OnePassStabilizer>;
	
	ptr_extern! { crate::videostab::OnePassStabilizer,
		cv_PtrOfOnePassStabilizer_delete, cv_PtrOfOnePassStabilizer_get_inner_ptr, cv_PtrOfOnePassStabilizer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::OnePassStabilizer, cv_PtrOfOnePassStabilizer_new }
	
	impl PtrOfOnePassStabilizer {
		#[inline] pub fn as_raw_PtrOfOnePassStabilizer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOnePassStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::OnePassStabilizerTraitConst for PtrOfOnePassStabilizer {
		#[inline] fn as_raw_OnePassStabilizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::OnePassStabilizerTrait for PtrOfOnePassStabilizer {
		#[inline] fn as_raw_mut_OnePassStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IFrameSourceConst for PtrOfOnePassStabilizer {
		#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IFrameSource for PtrOfOnePassStabilizer {
		#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfOnePassStabilizer, core::Ptr<dyn crate::videostab::IFrameSource>,
		cv_PtrOfOnePassStabilizer_to_PtrOfIFrameSource,
	}
	
	impl crate::videostab::StabilizerBaseConst for PtrOfOnePassStabilizer {
		#[inline] fn as_raw_StabilizerBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::StabilizerBase for PtrOfOnePassStabilizer {
		#[inline] fn as_raw_mut_StabilizerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSparsePyrLkOptFlowEstimator = core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimator>;
	
	ptr_extern! { crate::videostab::SparsePyrLkOptFlowEstimator,
		cv_PtrOfSparsePyrLkOptFlowEstimator_delete, cv_PtrOfSparsePyrLkOptFlowEstimator_get_inner_ptr, cv_PtrOfSparsePyrLkOptFlowEstimator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::SparsePyrLkOptFlowEstimator, cv_PtrOfSparsePyrLkOptFlowEstimator_new }
	
	impl PtrOfSparsePyrLkOptFlowEstimator {
		#[inline] pub fn as_raw_PtrOfSparsePyrLkOptFlowEstimator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSparsePyrLkOptFlowEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::SparsePyrLkOptFlowEstimatorTraitConst for PtrOfSparsePyrLkOptFlowEstimator {
		#[inline] fn as_raw_SparsePyrLkOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::SparsePyrLkOptFlowEstimatorTrait for PtrOfSparsePyrLkOptFlowEstimator {
		#[inline] fn as_raw_mut_SparsePyrLkOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::ISparseOptFlowEstimatorConst for PtrOfSparsePyrLkOptFlowEstimator {
		#[inline] fn as_raw_ISparseOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ISparseOptFlowEstimator for PtrOfSparsePyrLkOptFlowEstimator {
		#[inline] fn as_raw_mut_ISparseOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSparsePyrLkOptFlowEstimator, core::Ptr<dyn crate::videostab::ISparseOptFlowEstimator>,
		cv_PtrOfSparsePyrLkOptFlowEstimator_to_PtrOfISparseOptFlowEstimator,
	}
	
	impl crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst for PtrOfSparsePyrLkOptFlowEstimator {
		#[inline] fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::PyrLkOptFlowEstimatorBaseTrait for PtrOfSparsePyrLkOptFlowEstimator {
		#[inline] fn as_raw_mut_PyrLkOptFlowEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSparsePyrLkOptFlowEstimatorGpu = core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimatorGpu>;
	
	ptr_extern! { crate::videostab::SparsePyrLkOptFlowEstimatorGpu,
		cv_PtrOfSparsePyrLkOptFlowEstimatorGpu_delete, cv_PtrOfSparsePyrLkOptFlowEstimatorGpu_get_inner_ptr, cv_PtrOfSparsePyrLkOptFlowEstimatorGpu_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::SparsePyrLkOptFlowEstimatorGpu, cv_PtrOfSparsePyrLkOptFlowEstimatorGpu_new }
	
	impl PtrOfSparsePyrLkOptFlowEstimatorGpu {
		#[inline] pub fn as_raw_PtrOfSparsePyrLkOptFlowEstimatorGpu(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSparsePyrLkOptFlowEstimatorGpu(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::SparsePyrLkOptFlowEstimatorGpuTraitConst for PtrOfSparsePyrLkOptFlowEstimatorGpu {
		#[inline] fn as_raw_SparsePyrLkOptFlowEstimatorGpu(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::SparsePyrLkOptFlowEstimatorGpuTrait for PtrOfSparsePyrLkOptFlowEstimatorGpu {
		#[inline] fn as_raw_mut_SparsePyrLkOptFlowEstimatorGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::ISparseOptFlowEstimatorConst for PtrOfSparsePyrLkOptFlowEstimatorGpu {
		#[inline] fn as_raw_ISparseOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ISparseOptFlowEstimator for PtrOfSparsePyrLkOptFlowEstimatorGpu {
		#[inline] fn as_raw_mut_ISparseOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSparsePyrLkOptFlowEstimatorGpu, core::Ptr<dyn crate::videostab::ISparseOptFlowEstimator>,
		cv_PtrOfSparsePyrLkOptFlowEstimatorGpu_to_PtrOfISparseOptFlowEstimator,
	}
	
	impl crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst for PtrOfSparsePyrLkOptFlowEstimatorGpu {
		#[inline] fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::PyrLkOptFlowEstimatorBaseTrait for PtrOfSparsePyrLkOptFlowEstimatorGpu {
		#[inline] fn as_raw_mut_PyrLkOptFlowEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfToFileMotionWriter = core::Ptr<crate::videostab::ToFileMotionWriter>;
	
	ptr_extern! { crate::videostab::ToFileMotionWriter,
		cv_PtrOfToFileMotionWriter_delete, cv_PtrOfToFileMotionWriter_get_inner_ptr, cv_PtrOfToFileMotionWriter_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::ToFileMotionWriter, cv_PtrOfToFileMotionWriter_new }
	
	impl PtrOfToFileMotionWriter {
		#[inline] pub fn as_raw_PtrOfToFileMotionWriter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfToFileMotionWriter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::ToFileMotionWriterTraitConst for PtrOfToFileMotionWriter {
		#[inline] fn as_raw_ToFileMotionWriter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ToFileMotionWriterTrait for PtrOfToFileMotionWriter {
		#[inline] fn as_raw_mut_ToFileMotionWriter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::ImageMotionEstimatorBaseConst for PtrOfToFileMotionWriter {
		#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ImageMotionEstimatorBase for PtrOfToFileMotionWriter {
		#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfToFileMotionWriter, core::Ptr<dyn crate::videostab::ImageMotionEstimatorBase>,
		cv_PtrOfToFileMotionWriter_to_PtrOfImageMotionEstimatorBase,
	}
	
	pub type PtrOfTranslationBasedLocalOutlierRejector = core::Ptr<crate::videostab::TranslationBasedLocalOutlierRejector>;
	
	ptr_extern! { crate::videostab::TranslationBasedLocalOutlierRejector,
		cv_PtrOfTranslationBasedLocalOutlierRejector_delete, cv_PtrOfTranslationBasedLocalOutlierRejector_get_inner_ptr, cv_PtrOfTranslationBasedLocalOutlierRejector_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::TranslationBasedLocalOutlierRejector, cv_PtrOfTranslationBasedLocalOutlierRejector_new }
	
	impl PtrOfTranslationBasedLocalOutlierRejector {
		#[inline] pub fn as_raw_PtrOfTranslationBasedLocalOutlierRejector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTranslationBasedLocalOutlierRejector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::TranslationBasedLocalOutlierRejectorTraitConst for PtrOfTranslationBasedLocalOutlierRejector {
		#[inline] fn as_raw_TranslationBasedLocalOutlierRejector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::TranslationBasedLocalOutlierRejectorTrait for PtrOfTranslationBasedLocalOutlierRejector {
		#[inline] fn as_raw_mut_TranslationBasedLocalOutlierRejector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IOutlierRejectorConst for PtrOfTranslationBasedLocalOutlierRejector {
		#[inline] fn as_raw_IOutlierRejector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IOutlierRejector for PtrOfTranslationBasedLocalOutlierRejector {
		#[inline] fn as_raw_mut_IOutlierRejector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfTranslationBasedLocalOutlierRejector, core::Ptr<dyn crate::videostab::IOutlierRejector>,
		cv_PtrOfTranslationBasedLocalOutlierRejector_to_PtrOfIOutlierRejector,
	}
	
	pub type PtrOfTwoPassStabilizer = core::Ptr<crate::videostab::TwoPassStabilizer>;
	
	ptr_extern! { crate::videostab::TwoPassStabilizer,
		cv_PtrOfTwoPassStabilizer_delete, cv_PtrOfTwoPassStabilizer_get_inner_ptr, cv_PtrOfTwoPassStabilizer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::TwoPassStabilizer, cv_PtrOfTwoPassStabilizer_new }
	
	impl PtrOfTwoPassStabilizer {
		#[inline] pub fn as_raw_PtrOfTwoPassStabilizer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTwoPassStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::TwoPassStabilizerTraitConst for PtrOfTwoPassStabilizer {
		#[inline] fn as_raw_TwoPassStabilizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::TwoPassStabilizerTrait for PtrOfTwoPassStabilizer {
		#[inline] fn as_raw_mut_TwoPassStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IFrameSourceConst for PtrOfTwoPassStabilizer {
		#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IFrameSource for PtrOfTwoPassStabilizer {
		#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfTwoPassStabilizer, core::Ptr<dyn crate::videostab::IFrameSource>,
		cv_PtrOfTwoPassStabilizer_to_PtrOfIFrameSource,
	}
	
	impl crate::videostab::StabilizerBaseConst for PtrOfTwoPassStabilizer {
		#[inline] fn as_raw_StabilizerBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::StabilizerBase for PtrOfTwoPassStabilizer {
		#[inline] fn as_raw_mut_StabilizerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfVideoFileSource = core::Ptr<crate::videostab::VideoFileSource>;
	
	ptr_extern! { crate::videostab::VideoFileSource,
		cv_PtrOfVideoFileSource_delete, cv_PtrOfVideoFileSource_get_inner_ptr, cv_PtrOfVideoFileSource_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::VideoFileSource, cv_PtrOfVideoFileSource_new }
	
	impl PtrOfVideoFileSource {
		#[inline] pub fn as_raw_PtrOfVideoFileSource(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfVideoFileSource(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::VideoFileSourceTraitConst for PtrOfVideoFileSource {
		#[inline] fn as_raw_VideoFileSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::VideoFileSourceTrait for PtrOfVideoFileSource {
		#[inline] fn as_raw_mut_VideoFileSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IFrameSourceConst for PtrOfVideoFileSource {
		#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IFrameSource for PtrOfVideoFileSource {
		#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfVideoFileSource, core::Ptr<dyn crate::videostab::IFrameSource>,
		cv_PtrOfVideoFileSource_to_PtrOfIFrameSource,
	}
	
	pub type PtrOfWeightingDeblurer = core::Ptr<crate::videostab::WeightingDeblurer>;
	
	ptr_extern! { crate::videostab::WeightingDeblurer,
		cv_PtrOfWeightingDeblurer_delete, cv_PtrOfWeightingDeblurer_get_inner_ptr, cv_PtrOfWeightingDeblurer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::WeightingDeblurer, cv_PtrOfWeightingDeblurer_new }
	
	impl PtrOfWeightingDeblurer {
		#[inline] pub fn as_raw_PtrOfWeightingDeblurer(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfWeightingDeblurer(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::WeightingDeblurerTraitConst for PtrOfWeightingDeblurer {
		#[inline] fn as_raw_WeightingDeblurer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::WeightingDeblurerTrait for PtrOfWeightingDeblurer {
		#[inline] fn as_raw_mut_WeightingDeblurer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::DeblurerBaseConst for PtrOfWeightingDeblurer {
		#[inline] fn as_raw_DeblurerBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::DeblurerBase for PtrOfWeightingDeblurer {
		#[inline] fn as_raw_mut_DeblurerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfWeightingDeblurer, core::Ptr<dyn crate::videostab::DeblurerBase>,
		cv_PtrOfWeightingDeblurer_to_PtrOfDeblurerBase,
	}
	
	pub type PtrOfWobbleSuppressorBase = core::Ptr<dyn crate::videostab::WobbleSuppressorBase>;
	
	ptr_extern! { dyn crate::videostab::WobbleSuppressorBase,
		cv_PtrOfWobbleSuppressorBase_delete, cv_PtrOfWobbleSuppressorBase_get_inner_ptr, cv_PtrOfWobbleSuppressorBase_get_inner_ptr_mut
	}
	
	impl PtrOfWobbleSuppressorBase {
		#[inline] pub fn as_raw_PtrOfWobbleSuppressorBase(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfWobbleSuppressorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::videostab::WobbleSuppressorBaseConst for PtrOfWobbleSuppressorBase {
		#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::WobbleSuppressorBase for PtrOfWobbleSuppressorBase {
		#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_videostab)]
pub use videostab_types::*;

#[cfg(ocvrs_has_module_xfeatures2d)]
mod xfeatures2d_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAffineFeature2D = core::Ptr<dyn crate::xfeatures2d::AffineFeature2D>;
	
	ptr_extern! { dyn crate::xfeatures2d::AffineFeature2D,
		cv_PtrOfAffineFeature2D_delete, cv_PtrOfAffineFeature2D_get_inner_ptr, cv_PtrOfAffineFeature2D_get_inner_ptr_mut
	}
	
	impl PtrOfAffineFeature2D {
		#[inline] pub fn as_raw_PtrOfAffineFeature2D(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAffineFeature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::AffineFeature2DConst for PtrOfAffineFeature2D {
		#[inline] fn as_raw_AffineFeature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::AffineFeature2D for PtrOfAffineFeature2D {
		#[inline] fn as_raw_mut_AffineFeature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfAffineFeature2D {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfAffineFeature2D {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfAffineFeature2D {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfAffineFeature2D {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfAffineFeature2D, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfAffineFeature2D_to_PtrOfFeature2D,
	}
	
	pub type PtrOfBEBLID = core::Ptr<crate::xfeatures2d::BEBLID>;
	
	ptr_extern! { crate::xfeatures2d::BEBLID,
		cv_PtrOfBEBLID_delete, cv_PtrOfBEBLID_get_inner_ptr, cv_PtrOfBEBLID_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::xfeatures2d::BEBLID, cv_PtrOfBEBLID_new }
	
	impl PtrOfBEBLID {
		#[inline] pub fn as_raw_PtrOfBEBLID(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBEBLID(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::BEBLIDTraitConst for PtrOfBEBLID {
		#[inline] fn as_raw_BEBLID(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::BEBLIDTrait for PtrOfBEBLID {
		#[inline] fn as_raw_mut_BEBLID(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfBEBLID {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfBEBLID {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfBEBLID {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfBEBLID {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfBEBLID, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfBEBLID_to_PtrOfFeature2D,
	}
	
	pub type PtrOfBoostDesc = core::Ptr<dyn crate::xfeatures2d::BoostDesc>;
	
	ptr_extern! { dyn crate::xfeatures2d::BoostDesc,
		cv_PtrOfBoostDesc_delete, cv_PtrOfBoostDesc_get_inner_ptr, cv_PtrOfBoostDesc_get_inner_ptr_mut
	}
	
	impl PtrOfBoostDesc {
		#[inline] pub fn as_raw_PtrOfBoostDesc(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBoostDesc(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::BoostDescConst for PtrOfBoostDesc {
		#[inline] fn as_raw_BoostDesc(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::BoostDesc for PtrOfBoostDesc {
		#[inline] fn as_raw_mut_BoostDesc(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfBoostDesc {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfBoostDesc {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfBoostDesc {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfBoostDesc {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfBoostDesc, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfBoostDesc_to_PtrOfFeature2D,
	}
	
	pub type PtrOfBriefDescriptorExtractor = core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor>;
	
	ptr_extern! { crate::xfeatures2d::BriefDescriptorExtractor,
		cv_PtrOfBriefDescriptorExtractor_delete, cv_PtrOfBriefDescriptorExtractor_get_inner_ptr, cv_PtrOfBriefDescriptorExtractor_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::xfeatures2d::BriefDescriptorExtractor, cv_PtrOfBriefDescriptorExtractor_new }
	
	impl PtrOfBriefDescriptorExtractor {
		#[inline] pub fn as_raw_PtrOfBriefDescriptorExtractor(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBriefDescriptorExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::BriefDescriptorExtractorTraitConst for PtrOfBriefDescriptorExtractor {
		#[inline] fn as_raw_BriefDescriptorExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::BriefDescriptorExtractorTrait for PtrOfBriefDescriptorExtractor {
		#[inline] fn as_raw_mut_BriefDescriptorExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfBriefDescriptorExtractor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfBriefDescriptorExtractor {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfBriefDescriptorExtractor {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfBriefDescriptorExtractor {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfBriefDescriptorExtractor, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfBriefDescriptorExtractor_to_PtrOfFeature2D,
	}
	
	pub type PtrOfDAISY = core::Ptr<dyn crate::xfeatures2d::DAISY>;
	
	ptr_extern! { dyn crate::xfeatures2d::DAISY,
		cv_PtrOfDAISY_delete, cv_PtrOfDAISY_get_inner_ptr, cv_PtrOfDAISY_get_inner_ptr_mut
	}
	
	impl PtrOfDAISY {
		#[inline] pub fn as_raw_PtrOfDAISY(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDAISY(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::DAISYConst for PtrOfDAISY {
		#[inline] fn as_raw_DAISY(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::DAISY for PtrOfDAISY {
		#[inline] fn as_raw_mut_DAISY(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfDAISY {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfDAISY {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfDAISY {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfDAISY {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDAISY, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfDAISY_to_PtrOfFeature2D,
	}
	
	pub type PtrOfFREAK = core::Ptr<crate::xfeatures2d::FREAK>;
	
	ptr_extern! { crate::xfeatures2d::FREAK,
		cv_PtrOfFREAK_delete, cv_PtrOfFREAK_get_inner_ptr, cv_PtrOfFREAK_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::xfeatures2d::FREAK, cv_PtrOfFREAK_new }
	
	impl PtrOfFREAK {
		#[inline] pub fn as_raw_PtrOfFREAK(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFREAK(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::FREAKTraitConst for PtrOfFREAK {
		#[inline] fn as_raw_FREAK(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::FREAKTrait for PtrOfFREAK {
		#[inline] fn as_raw_mut_FREAK(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfFREAK {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfFREAK {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfFREAK {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfFREAK {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfFREAK, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfFREAK_to_PtrOfFeature2D,
	}
	
	pub type PtrOfHarrisLaplaceFeatureDetector = core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector>;
	
	ptr_extern! { crate::xfeatures2d::HarrisLaplaceFeatureDetector,
		cv_PtrOfHarrisLaplaceFeatureDetector_delete, cv_PtrOfHarrisLaplaceFeatureDetector_get_inner_ptr, cv_PtrOfHarrisLaplaceFeatureDetector_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::xfeatures2d::HarrisLaplaceFeatureDetector, cv_PtrOfHarrisLaplaceFeatureDetector_new }
	
	impl PtrOfHarrisLaplaceFeatureDetector {
		#[inline] pub fn as_raw_PtrOfHarrisLaplaceFeatureDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHarrisLaplaceFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::HarrisLaplaceFeatureDetectorTraitConst for PtrOfHarrisLaplaceFeatureDetector {
		#[inline] fn as_raw_HarrisLaplaceFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::HarrisLaplaceFeatureDetectorTrait for PtrOfHarrisLaplaceFeatureDetector {
		#[inline] fn as_raw_mut_HarrisLaplaceFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfHarrisLaplaceFeatureDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfHarrisLaplaceFeatureDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfHarrisLaplaceFeatureDetector {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfHarrisLaplaceFeatureDetector {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfHarrisLaplaceFeatureDetector, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfHarrisLaplaceFeatureDetector_to_PtrOfFeature2D,
	}
	
	pub type PtrOfLATCH = core::Ptr<crate::xfeatures2d::LATCH>;
	
	ptr_extern! { crate::xfeatures2d::LATCH,
		cv_PtrOfLATCH_delete, cv_PtrOfLATCH_get_inner_ptr, cv_PtrOfLATCH_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::xfeatures2d::LATCH, cv_PtrOfLATCH_new }
	
	impl PtrOfLATCH {
		#[inline] pub fn as_raw_PtrOfLATCH(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLATCH(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::LATCHTraitConst for PtrOfLATCH {
		#[inline] fn as_raw_LATCH(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::LATCHTrait for PtrOfLATCH {
		#[inline] fn as_raw_mut_LATCH(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfLATCH {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfLATCH {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfLATCH {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfLATCH {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfLATCH, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfLATCH_to_PtrOfFeature2D,
	}
	
	pub type PtrOfLUCID = core::Ptr<crate::xfeatures2d::LUCID>;
	
	ptr_extern! { crate::xfeatures2d::LUCID,
		cv_PtrOfLUCID_delete, cv_PtrOfLUCID_get_inner_ptr, cv_PtrOfLUCID_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::xfeatures2d::LUCID, cv_PtrOfLUCID_new }
	
	impl PtrOfLUCID {
		#[inline] pub fn as_raw_PtrOfLUCID(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLUCID(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::LUCIDTraitConst for PtrOfLUCID {
		#[inline] fn as_raw_LUCID(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::LUCIDTrait for PtrOfLUCID {
		#[inline] fn as_raw_mut_LUCID(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfLUCID {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfLUCID {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfLUCID {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfLUCID {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfLUCID, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfLUCID_to_PtrOfFeature2D,
	}
	
	pub type PtrOfMSDDetector = core::Ptr<crate::xfeatures2d::MSDDetector>;
	
	ptr_extern! { crate::xfeatures2d::MSDDetector,
		cv_PtrOfMSDDetector_delete, cv_PtrOfMSDDetector_get_inner_ptr, cv_PtrOfMSDDetector_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::xfeatures2d::MSDDetector, cv_PtrOfMSDDetector_new }
	
	impl PtrOfMSDDetector {
		#[inline] pub fn as_raw_PtrOfMSDDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMSDDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::MSDDetectorTraitConst for PtrOfMSDDetector {
		#[inline] fn as_raw_MSDDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::MSDDetectorTrait for PtrOfMSDDetector {
		#[inline] fn as_raw_mut_MSDDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfMSDDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfMSDDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfMSDDetector {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfMSDDetector {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMSDDetector, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfMSDDetector_to_PtrOfFeature2D,
	}
	
	pub type PtrOfPCTSignatures = core::Ptr<dyn crate::xfeatures2d::PCTSignatures>;
	
	ptr_extern! { dyn crate::xfeatures2d::PCTSignatures,
		cv_PtrOfPCTSignatures_delete, cv_PtrOfPCTSignatures_get_inner_ptr, cv_PtrOfPCTSignatures_get_inner_ptr_mut
	}
	
	impl PtrOfPCTSignatures {
		#[inline] pub fn as_raw_PtrOfPCTSignatures(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPCTSignatures(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::PCTSignaturesConst for PtrOfPCTSignatures {
		#[inline] fn as_raw_PCTSignatures(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::PCTSignatures for PtrOfPCTSignatures {
		#[inline] fn as_raw_mut_PCTSignatures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfPCTSignatures {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfPCTSignatures {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPCTSignaturesSQFD = core::Ptr<dyn crate::xfeatures2d::PCTSignaturesSQFD>;
	
	ptr_extern! { dyn crate::xfeatures2d::PCTSignaturesSQFD,
		cv_PtrOfPCTSignaturesSQFD_delete, cv_PtrOfPCTSignaturesSQFD_get_inner_ptr, cv_PtrOfPCTSignaturesSQFD_get_inner_ptr_mut
	}
	
	impl PtrOfPCTSignaturesSQFD {
		#[inline] pub fn as_raw_PtrOfPCTSignaturesSQFD(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPCTSignaturesSQFD(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::PCTSignaturesSQFDConst for PtrOfPCTSignaturesSQFD {
		#[inline] fn as_raw_PCTSignaturesSQFD(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::PCTSignaturesSQFD for PtrOfPCTSignaturesSQFD {
		#[inline] fn as_raw_mut_PCTSignaturesSQFD(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfPCTSignaturesSQFD {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfPCTSignaturesSQFD {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSURF = core::Ptr<dyn crate::xfeatures2d::SURF>;
	
	ptr_extern! { dyn crate::xfeatures2d::SURF,
		cv_PtrOfSURF_delete, cv_PtrOfSURF_get_inner_ptr, cv_PtrOfSURF_get_inner_ptr_mut
	}
	
	impl PtrOfSURF {
		#[inline] pub fn as_raw_PtrOfSURF(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSURF(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::SURFConst for PtrOfSURF {
		#[inline] fn as_raw_SURF(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::SURF for PtrOfSURF {
		#[inline] fn as_raw_mut_SURF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSURF {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSURF {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfSURF {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfSURF {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSURF, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfSURF_to_PtrOfFeature2D,
	}
	
	pub type PtrOfSURF_CUDA = core::Ptr<crate::xfeatures2d::SURF_CUDA>;
	
	ptr_extern! { crate::xfeatures2d::SURF_CUDA,
		cv_PtrOfSURF_CUDA_delete, cv_PtrOfSURF_CUDA_get_inner_ptr, cv_PtrOfSURF_CUDA_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::xfeatures2d::SURF_CUDA, cv_PtrOfSURF_CUDA_new }
	
	impl PtrOfSURF_CUDA {
		#[inline] pub fn as_raw_PtrOfSURF_CUDA(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSURF_CUDA(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::SURF_CUDATraitConst for PtrOfSURF_CUDA {
		#[inline] fn as_raw_SURF_CUDA(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::SURF_CUDATrait for PtrOfSURF_CUDA {
		#[inline] fn as_raw_mut_SURF_CUDA(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStarDetector = core::Ptr<crate::xfeatures2d::StarDetector>;
	
	ptr_extern! { crate::xfeatures2d::StarDetector,
		cv_PtrOfStarDetector_delete, cv_PtrOfStarDetector_get_inner_ptr, cv_PtrOfStarDetector_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::xfeatures2d::StarDetector, cv_PtrOfStarDetector_new }
	
	impl PtrOfStarDetector {
		#[inline] pub fn as_raw_PtrOfStarDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStarDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::StarDetectorTraitConst for PtrOfStarDetector {
		#[inline] fn as_raw_StarDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::StarDetectorTrait for PtrOfStarDetector {
		#[inline] fn as_raw_mut_StarDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfStarDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfStarDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfStarDetector {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfStarDetector {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfStarDetector, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfStarDetector_to_PtrOfFeature2D,
	}
	
	pub type PtrOfTBMR = core::Ptr<dyn crate::xfeatures2d::TBMR>;
	
	ptr_extern! { dyn crate::xfeatures2d::TBMR,
		cv_PtrOfTBMR_delete, cv_PtrOfTBMR_get_inner_ptr, cv_PtrOfTBMR_get_inner_ptr_mut
	}
	
	impl PtrOfTBMR {
		#[inline] pub fn as_raw_PtrOfTBMR(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTBMR(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::TBMRConst for PtrOfTBMR {
		#[inline] fn as_raw_TBMR(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::TBMR for PtrOfTBMR {
		#[inline] fn as_raw_mut_TBMR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfTBMR {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfTBMR {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfTBMR {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfTBMR {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfTBMR, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfTBMR_to_PtrOfFeature2D,
	}
	
	impl crate::xfeatures2d::AffineFeature2DConst for PtrOfTBMR {
		#[inline] fn as_raw_AffineFeature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::AffineFeature2D for PtrOfTBMR {
		#[inline] fn as_raw_mut_AffineFeature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfVGG = core::Ptr<dyn crate::xfeatures2d::VGG>;
	
	ptr_extern! { dyn crate::xfeatures2d::VGG,
		cv_PtrOfVGG_delete, cv_PtrOfVGG_get_inner_ptr, cv_PtrOfVGG_get_inner_ptr_mut
	}
	
	impl PtrOfVGG {
		#[inline] pub fn as_raw_PtrOfVGG(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfVGG(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::VGGConst for PtrOfVGG {
		#[inline] fn as_raw_VGG(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::VGG for PtrOfVGG {
		#[inline] fn as_raw_mut_VGG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfVGG {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfVGG {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for PtrOfVGG {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for PtrOfVGG {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfVGG, core::Ptr<crate::features2d::Feature2D>,
		cv_PtrOfVGG_to_PtrOfFeature2D,
	}
	
	pub type VectorOfElliptic_KeyPoint = core::Vector<crate::xfeatures2d::Elliptic_KeyPoint>;
	
	impl VectorOfElliptic_KeyPoint {
		pub fn as_raw_VectorOfElliptic_KeyPoint(&self) -> *const c_void { self.as_raw() }
		pub fn as_raw_mut_VectorOfElliptic_KeyPoint(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	vector_extern! { crate::xfeatures2d::Elliptic_KeyPoint, *const c_void, *mut c_void,
		cv_VectorOfElliptic_KeyPoint_new, cv_VectorOfElliptic_KeyPoint_delete,
		cv_VectorOfElliptic_KeyPoint_len, cv_VectorOfElliptic_KeyPoint_is_empty,
		cv_VectorOfElliptic_KeyPoint_capacity, cv_VectorOfElliptic_KeyPoint_shrink_to_fit,
		cv_VectorOfElliptic_KeyPoint_reserve, cv_VectorOfElliptic_KeyPoint_remove,
		cv_VectorOfElliptic_KeyPoint_swap, cv_VectorOfElliptic_KeyPoint_clear,
		cv_VectorOfElliptic_KeyPoint_get, cv_VectorOfElliptic_KeyPoint_set,
		cv_VectorOfElliptic_KeyPoint_push, cv_VectorOfElliptic_KeyPoint_insert,
	}
	vector_non_copy_or_bool! { crate::xfeatures2d::Elliptic_KeyPoint }
	
	unsafe impl Send for core::Vector<crate::xfeatures2d::Elliptic_KeyPoint> {}
	
}
#[cfg(ocvrs_has_module_xfeatures2d)]
pub use xfeatures2d_types::*;

#[cfg(ocvrs_has_module_ximgproc)]
mod ximgproc_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAdaptiveManifoldFilter = core::Ptr<dyn crate::ximgproc::AdaptiveManifoldFilter>;
	
	ptr_extern! { dyn crate::ximgproc::AdaptiveManifoldFilter,
		cv_PtrOfAdaptiveManifoldFilter_delete, cv_PtrOfAdaptiveManifoldFilter_get_inner_ptr, cv_PtrOfAdaptiveManifoldFilter_get_inner_ptr_mut
	}
	
	impl PtrOfAdaptiveManifoldFilter {
		#[inline] pub fn as_raw_PtrOfAdaptiveManifoldFilter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAdaptiveManifoldFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::AdaptiveManifoldFilterConst for PtrOfAdaptiveManifoldFilter {
		#[inline] fn as_raw_AdaptiveManifoldFilter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::AdaptiveManifoldFilter for PtrOfAdaptiveManifoldFilter {
		#[inline] fn as_raw_mut_AdaptiveManifoldFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfAdaptiveManifoldFilter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfAdaptiveManifoldFilter {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfContourFitting = core::Ptr<crate::ximgproc::ContourFitting>;
	
	ptr_extern! { crate::ximgproc::ContourFitting,
		cv_PtrOfContourFitting_delete, cv_PtrOfContourFitting_get_inner_ptr, cv_PtrOfContourFitting_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::ximgproc::ContourFitting, cv_PtrOfContourFitting_new }
	
	impl PtrOfContourFitting {
		#[inline] pub fn as_raw_PtrOfContourFitting(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfContourFitting(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::ContourFittingTraitConst for PtrOfContourFitting {
		#[inline] fn as_raw_ContourFitting(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::ContourFittingTrait for PtrOfContourFitting {
		#[inline] fn as_raw_mut_ContourFitting(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfContourFitting {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfContourFitting {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDTFilter = core::Ptr<dyn crate::ximgproc::DTFilter>;
	
	ptr_extern! { dyn crate::ximgproc::DTFilter,
		cv_PtrOfDTFilter_delete, cv_PtrOfDTFilter_get_inner_ptr, cv_PtrOfDTFilter_get_inner_ptr_mut
	}
	
	impl PtrOfDTFilter {
		#[inline] pub fn as_raw_PtrOfDTFilter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDTFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::DTFilterConst for PtrOfDTFilter {
		#[inline] fn as_raw_DTFilter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::DTFilter for PtrOfDTFilter {
		#[inline] fn as_raw_mut_DTFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfDTFilter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfDTFilter {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDisparityWLSFilter = core::Ptr<dyn crate::ximgproc::DisparityWLSFilter>;
	
	ptr_extern! { dyn crate::ximgproc::DisparityWLSFilter,
		cv_PtrOfDisparityWLSFilter_delete, cv_PtrOfDisparityWLSFilter_get_inner_ptr, cv_PtrOfDisparityWLSFilter_get_inner_ptr_mut
	}
	
	impl PtrOfDisparityWLSFilter {
		#[inline] pub fn as_raw_PtrOfDisparityWLSFilter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDisparityWLSFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::DisparityWLSFilterConst for PtrOfDisparityWLSFilter {
		#[inline] fn as_raw_DisparityWLSFilter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::DisparityWLSFilter for PtrOfDisparityWLSFilter {
		#[inline] fn as_raw_mut_DisparityWLSFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfDisparityWLSFilter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfDisparityWLSFilter {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::DisparityFilterConst for PtrOfDisparityWLSFilter {
		#[inline] fn as_raw_DisparityFilter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::DisparityFilter for PtrOfDisparityWLSFilter {
		#[inline] fn as_raw_mut_DisparityFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEdgeAwareInterpolator = core::Ptr<dyn crate::ximgproc::EdgeAwareInterpolator>;
	
	ptr_extern! { dyn crate::ximgproc::EdgeAwareInterpolator,
		cv_PtrOfEdgeAwareInterpolator_delete, cv_PtrOfEdgeAwareInterpolator_get_inner_ptr, cv_PtrOfEdgeAwareInterpolator_get_inner_ptr_mut
	}
	
	impl PtrOfEdgeAwareInterpolator {
		#[inline] pub fn as_raw_PtrOfEdgeAwareInterpolator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEdgeAwareInterpolator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::EdgeAwareInterpolatorConst for PtrOfEdgeAwareInterpolator {
		#[inline] fn as_raw_EdgeAwareInterpolator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::EdgeAwareInterpolator for PtrOfEdgeAwareInterpolator {
		#[inline] fn as_raw_mut_EdgeAwareInterpolator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfEdgeAwareInterpolator {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfEdgeAwareInterpolator {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SparseMatchInterpolatorConst for PtrOfEdgeAwareInterpolator {
		#[inline] fn as_raw_SparseMatchInterpolator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SparseMatchInterpolator for PtrOfEdgeAwareInterpolator {
		#[inline] fn as_raw_mut_SparseMatchInterpolator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEdgeBoxes = core::Ptr<dyn crate::ximgproc::EdgeBoxes>;
	
	ptr_extern! { dyn crate::ximgproc::EdgeBoxes,
		cv_PtrOfEdgeBoxes_delete, cv_PtrOfEdgeBoxes_get_inner_ptr, cv_PtrOfEdgeBoxes_get_inner_ptr_mut
	}
	
	impl PtrOfEdgeBoxes {
		#[inline] pub fn as_raw_PtrOfEdgeBoxes(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEdgeBoxes(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::EdgeBoxesConst for PtrOfEdgeBoxes {
		#[inline] fn as_raw_EdgeBoxes(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::EdgeBoxes for PtrOfEdgeBoxes {
		#[inline] fn as_raw_mut_EdgeBoxes(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfEdgeBoxes {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfEdgeBoxes {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEdgeDrawing = core::Ptr<dyn crate::ximgproc::EdgeDrawing>;
	
	ptr_extern! { dyn crate::ximgproc::EdgeDrawing,
		cv_PtrOfEdgeDrawing_delete, cv_PtrOfEdgeDrawing_get_inner_ptr, cv_PtrOfEdgeDrawing_get_inner_ptr_mut
	}
	
	impl PtrOfEdgeDrawing {
		#[inline] pub fn as_raw_PtrOfEdgeDrawing(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEdgeDrawing(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::EdgeDrawingConst for PtrOfEdgeDrawing {
		#[inline] fn as_raw_EdgeDrawing(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::EdgeDrawing for PtrOfEdgeDrawing {
		#[inline] fn as_raw_mut_EdgeDrawing(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfEdgeDrawing {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfEdgeDrawing {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFastBilateralSolverFilter = core::Ptr<dyn crate::ximgproc::FastBilateralSolverFilter>;
	
	ptr_extern! { dyn crate::ximgproc::FastBilateralSolverFilter,
		cv_PtrOfFastBilateralSolverFilter_delete, cv_PtrOfFastBilateralSolverFilter_get_inner_ptr, cv_PtrOfFastBilateralSolverFilter_get_inner_ptr_mut
	}
	
	impl PtrOfFastBilateralSolverFilter {
		#[inline] pub fn as_raw_PtrOfFastBilateralSolverFilter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFastBilateralSolverFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::FastBilateralSolverFilterConst for PtrOfFastBilateralSolverFilter {
		#[inline] fn as_raw_FastBilateralSolverFilter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::FastBilateralSolverFilter for PtrOfFastBilateralSolverFilter {
		#[inline] fn as_raw_mut_FastBilateralSolverFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfFastBilateralSolverFilter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfFastBilateralSolverFilter {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFastGlobalSmootherFilter = core::Ptr<dyn crate::ximgproc::FastGlobalSmootherFilter>;
	
	ptr_extern! { dyn crate::ximgproc::FastGlobalSmootherFilter,
		cv_PtrOfFastGlobalSmootherFilter_delete, cv_PtrOfFastGlobalSmootherFilter_get_inner_ptr, cv_PtrOfFastGlobalSmootherFilter_get_inner_ptr_mut
	}
	
	impl PtrOfFastGlobalSmootherFilter {
		#[inline] pub fn as_raw_PtrOfFastGlobalSmootherFilter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFastGlobalSmootherFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::FastGlobalSmootherFilterConst for PtrOfFastGlobalSmootherFilter {
		#[inline] fn as_raw_FastGlobalSmootherFilter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::FastGlobalSmootherFilter for PtrOfFastGlobalSmootherFilter {
		#[inline] fn as_raw_mut_FastGlobalSmootherFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfFastGlobalSmootherFilter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfFastGlobalSmootherFilter {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFastLineDetector = core::Ptr<dyn crate::ximgproc::FastLineDetector>;
	
	ptr_extern! { dyn crate::ximgproc::FastLineDetector,
		cv_PtrOfFastLineDetector_delete, cv_PtrOfFastLineDetector_get_inner_ptr, cv_PtrOfFastLineDetector_get_inner_ptr_mut
	}
	
	impl PtrOfFastLineDetector {
		#[inline] pub fn as_raw_PtrOfFastLineDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFastLineDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::FastLineDetectorConst for PtrOfFastLineDetector {
		#[inline] fn as_raw_FastLineDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::FastLineDetector for PtrOfFastLineDetector {
		#[inline] fn as_raw_mut_FastLineDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfFastLineDetector {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfFastLineDetector {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGraphSegmentation = core::Ptr<dyn crate::ximgproc::GraphSegmentation>;
	
	ptr_extern! { dyn crate::ximgproc::GraphSegmentation,
		cv_PtrOfGraphSegmentation_delete, cv_PtrOfGraphSegmentation_get_inner_ptr, cv_PtrOfGraphSegmentation_get_inner_ptr_mut
	}
	
	impl PtrOfGraphSegmentation {
		#[inline] pub fn as_raw_PtrOfGraphSegmentation(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGraphSegmentation(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::GraphSegmentationConst for PtrOfGraphSegmentation {
		#[inline] fn as_raw_GraphSegmentation(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::GraphSegmentation for PtrOfGraphSegmentation {
		#[inline] fn as_raw_mut_GraphSegmentation(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfGraphSegmentation {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfGraphSegmentation {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGuidedFilter = core::Ptr<dyn crate::ximgproc::GuidedFilter>;
	
	ptr_extern! { dyn crate::ximgproc::GuidedFilter,
		cv_PtrOfGuidedFilter_delete, cv_PtrOfGuidedFilter_get_inner_ptr, cv_PtrOfGuidedFilter_get_inner_ptr_mut
	}
	
	impl PtrOfGuidedFilter {
		#[inline] pub fn as_raw_PtrOfGuidedFilter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGuidedFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::GuidedFilterConst for PtrOfGuidedFilter {
		#[inline] fn as_raw_GuidedFilter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::GuidedFilter for PtrOfGuidedFilter {
		#[inline] fn as_raw_mut_GuidedFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfGuidedFilter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfGuidedFilter {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRFFeatureGetter = core::Ptr<dyn crate::ximgproc::RFFeatureGetter>;
	
	ptr_extern! { dyn crate::ximgproc::RFFeatureGetter,
		cv_PtrOfRFFeatureGetter_delete, cv_PtrOfRFFeatureGetter_get_inner_ptr, cv_PtrOfRFFeatureGetter_get_inner_ptr_mut
	}
	
	impl PtrOfRFFeatureGetter {
		#[inline] pub fn as_raw_PtrOfRFFeatureGetter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRFFeatureGetter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::RFFeatureGetterConst for PtrOfRFFeatureGetter {
		#[inline] fn as_raw_RFFeatureGetter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::RFFeatureGetter for PtrOfRFFeatureGetter {
		#[inline] fn as_raw_mut_RFFeatureGetter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfRFFeatureGetter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfRFFeatureGetter {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRICInterpolator = core::Ptr<dyn crate::ximgproc::RICInterpolator>;
	
	ptr_extern! { dyn crate::ximgproc::RICInterpolator,
		cv_PtrOfRICInterpolator_delete, cv_PtrOfRICInterpolator_get_inner_ptr, cv_PtrOfRICInterpolator_get_inner_ptr_mut
	}
	
	impl PtrOfRICInterpolator {
		#[inline] pub fn as_raw_PtrOfRICInterpolator(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRICInterpolator(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::RICInterpolatorConst for PtrOfRICInterpolator {
		#[inline] fn as_raw_RICInterpolator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::RICInterpolator for PtrOfRICInterpolator {
		#[inline] fn as_raw_mut_RICInterpolator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfRICInterpolator {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfRICInterpolator {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SparseMatchInterpolatorConst for PtrOfRICInterpolator {
		#[inline] fn as_raw_SparseMatchInterpolator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SparseMatchInterpolator for PtrOfRICInterpolator {
		#[inline] fn as_raw_mut_SparseMatchInterpolator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRidgeDetectionFilter = core::Ptr<dyn crate::ximgproc::RidgeDetectionFilter>;
	
	ptr_extern! { dyn crate::ximgproc::RidgeDetectionFilter,
		cv_PtrOfRidgeDetectionFilter_delete, cv_PtrOfRidgeDetectionFilter_get_inner_ptr, cv_PtrOfRidgeDetectionFilter_get_inner_ptr_mut
	}
	
	impl PtrOfRidgeDetectionFilter {
		#[inline] pub fn as_raw_PtrOfRidgeDetectionFilter(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRidgeDetectionFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::RidgeDetectionFilterConst for PtrOfRidgeDetectionFilter {
		#[inline] fn as_raw_RidgeDetectionFilter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::RidgeDetectionFilter for PtrOfRidgeDetectionFilter {
		#[inline] fn as_raw_mut_RidgeDetectionFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfRidgeDetectionFilter {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfRidgeDetectionFilter {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfScanSegment = core::Ptr<dyn crate::ximgproc::ScanSegment>;
	
	ptr_extern! { dyn crate::ximgproc::ScanSegment,
		cv_PtrOfScanSegment_delete, cv_PtrOfScanSegment_get_inner_ptr, cv_PtrOfScanSegment_get_inner_ptr_mut
	}
	
	impl PtrOfScanSegment {
		#[inline] pub fn as_raw_PtrOfScanSegment(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfScanSegment(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::ScanSegmentConst for PtrOfScanSegment {
		#[inline] fn as_raw_ScanSegment(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::ScanSegment for PtrOfScanSegment {
		#[inline] fn as_raw_mut_ScanSegment(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfScanSegment {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfScanSegment {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSelectiveSearchSegmentation = core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentation>;
	
	ptr_extern! { dyn crate::ximgproc::SelectiveSearchSegmentation,
		cv_PtrOfSelectiveSearchSegmentation_delete, cv_PtrOfSelectiveSearchSegmentation_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentation_get_inner_ptr_mut
	}
	
	impl PtrOfSelectiveSearchSegmentation {
		#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentation(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentation(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationConst for PtrOfSelectiveSearchSegmentation {
		#[inline] fn as_raw_SelectiveSearchSegmentation(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentation for PtrOfSelectiveSearchSegmentation {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentation(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSelectiveSearchSegmentation {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSelectiveSearchSegmentation {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSelectiveSearchSegmentationStrategy = core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>;
	
	ptr_extern! { dyn crate::ximgproc::SelectiveSearchSegmentationStrategy,
		cv_PtrOfSelectiveSearchSegmentationStrategy_delete, cv_PtrOfSelectiveSearchSegmentationStrategy_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategy_get_inner_ptr_mut
	}
	
	impl PtrOfSelectiveSearchSegmentationStrategy {
		#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyConst for PtrOfSelectiveSearchSegmentationStrategy {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategy for PtrOfSelectiveSearchSegmentationStrategy {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSelectiveSearchSegmentationStrategy {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSelectiveSearchSegmentationStrategy {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSelectiveSearchSegmentationStrategyColor = core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyColor>;
	
	ptr_extern! { dyn crate::ximgproc::SelectiveSearchSegmentationStrategyColor,
		cv_PtrOfSelectiveSearchSegmentationStrategyColor_delete, cv_PtrOfSelectiveSearchSegmentationStrategyColor_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategyColor_get_inner_ptr_mut
	}
	
	impl PtrOfSelectiveSearchSegmentationStrategyColor {
		#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategyColor(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategyColor(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyColorConst for PtrOfSelectiveSearchSegmentationStrategyColor {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategyColor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyColor for PtrOfSelectiveSearchSegmentationStrategyColor {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategyColor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSelectiveSearchSegmentationStrategyColor {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSelectiveSearchSegmentationStrategyColor {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyConst for PtrOfSelectiveSearchSegmentationStrategyColor {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategy for PtrOfSelectiveSearchSegmentationStrategyColor {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSelectiveSearchSegmentationStrategyColor, core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>,
		cv_PtrOfSelectiveSearchSegmentationStrategyColor_to_PtrOfSelectiveSearchSegmentationStrategy,
	}
	
	pub type PtrOfSelectiveSearchSegmentationStrategyFill = core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyFill>;
	
	ptr_extern! { dyn crate::ximgproc::SelectiveSearchSegmentationStrategyFill,
		cv_PtrOfSelectiveSearchSegmentationStrategyFill_delete, cv_PtrOfSelectiveSearchSegmentationStrategyFill_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategyFill_get_inner_ptr_mut
	}
	
	impl PtrOfSelectiveSearchSegmentationStrategyFill {
		#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategyFill(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategyFill(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyFillConst for PtrOfSelectiveSearchSegmentationStrategyFill {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategyFill(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyFill for PtrOfSelectiveSearchSegmentationStrategyFill {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategyFill(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSelectiveSearchSegmentationStrategyFill {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSelectiveSearchSegmentationStrategyFill {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyConst for PtrOfSelectiveSearchSegmentationStrategyFill {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategy for PtrOfSelectiveSearchSegmentationStrategyFill {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSelectiveSearchSegmentationStrategyFill, core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>,
		cv_PtrOfSelectiveSearchSegmentationStrategyFill_to_PtrOfSelectiveSearchSegmentationStrategy,
	}
	
	pub type PtrOfSelectiveSearchSegmentationStrategyMultiple = core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>;
	
	ptr_extern! { dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple,
		cv_PtrOfSelectiveSearchSegmentationStrategyMultiple_delete, cv_PtrOfSelectiveSearchSegmentationStrategyMultiple_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategyMultiple_get_inner_ptr_mut
	}
	
	impl PtrOfSelectiveSearchSegmentationStrategyMultiple {
		#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategyMultiple(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategyMultiple(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyMultipleConst for PtrOfSelectiveSearchSegmentationStrategyMultiple {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategyMultiple(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple for PtrOfSelectiveSearchSegmentationStrategyMultiple {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategyMultiple(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSelectiveSearchSegmentationStrategyMultiple {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSelectiveSearchSegmentationStrategyMultiple {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyConst for PtrOfSelectiveSearchSegmentationStrategyMultiple {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategy for PtrOfSelectiveSearchSegmentationStrategyMultiple {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSelectiveSearchSegmentationStrategyMultiple, core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>,
		cv_PtrOfSelectiveSearchSegmentationStrategyMultiple_to_PtrOfSelectiveSearchSegmentationStrategy,
	}
	
	pub type PtrOfSelectiveSearchSegmentationStrategySize = core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategySize>;
	
	ptr_extern! { dyn crate::ximgproc::SelectiveSearchSegmentationStrategySize,
		cv_PtrOfSelectiveSearchSegmentationStrategySize_delete, cv_PtrOfSelectiveSearchSegmentationStrategySize_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategySize_get_inner_ptr_mut
	}
	
	impl PtrOfSelectiveSearchSegmentationStrategySize {
		#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategySize(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategySize(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategySizeConst for PtrOfSelectiveSearchSegmentationStrategySize {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategySize(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategySize for PtrOfSelectiveSearchSegmentationStrategySize {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategySize(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSelectiveSearchSegmentationStrategySize {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSelectiveSearchSegmentationStrategySize {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyConst for PtrOfSelectiveSearchSegmentationStrategySize {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategy for PtrOfSelectiveSearchSegmentationStrategySize {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSelectiveSearchSegmentationStrategySize, core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>,
		cv_PtrOfSelectiveSearchSegmentationStrategySize_to_PtrOfSelectiveSearchSegmentationStrategy,
	}
	
	pub type PtrOfSelectiveSearchSegmentationStrategyTexture = core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyTexture>;
	
	ptr_extern! { dyn crate::ximgproc::SelectiveSearchSegmentationStrategyTexture,
		cv_PtrOfSelectiveSearchSegmentationStrategyTexture_delete, cv_PtrOfSelectiveSearchSegmentationStrategyTexture_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategyTexture_get_inner_ptr_mut
	}
	
	impl PtrOfSelectiveSearchSegmentationStrategyTexture {
		#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategyTexture(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategyTexture(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyTextureConst for PtrOfSelectiveSearchSegmentationStrategyTexture {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategyTexture(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyTexture for PtrOfSelectiveSearchSegmentationStrategyTexture {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategyTexture(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSelectiveSearchSegmentationStrategyTexture {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSelectiveSearchSegmentationStrategyTexture {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyConst for PtrOfSelectiveSearchSegmentationStrategyTexture {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategy for PtrOfSelectiveSearchSegmentationStrategyTexture {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSelectiveSearchSegmentationStrategyTexture, core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>,
		cv_PtrOfSelectiveSearchSegmentationStrategyTexture_to_PtrOfSelectiveSearchSegmentationStrategy,
	}
	
	pub type PtrOfStructuredEdgeDetection = core::Ptr<dyn crate::ximgproc::StructuredEdgeDetection>;
	
	ptr_extern! { dyn crate::ximgproc::StructuredEdgeDetection,
		cv_PtrOfStructuredEdgeDetection_delete, cv_PtrOfStructuredEdgeDetection_get_inner_ptr, cv_PtrOfStructuredEdgeDetection_get_inner_ptr_mut
	}
	
	impl PtrOfStructuredEdgeDetection {
		#[inline] pub fn as_raw_PtrOfStructuredEdgeDetection(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStructuredEdgeDetection(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::StructuredEdgeDetectionConst for PtrOfStructuredEdgeDetection {
		#[inline] fn as_raw_StructuredEdgeDetection(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::StructuredEdgeDetection for PtrOfStructuredEdgeDetection {
		#[inline] fn as_raw_mut_StructuredEdgeDetection(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfStructuredEdgeDetection {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfStructuredEdgeDetection {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperpixelLSC = core::Ptr<dyn crate::ximgproc::SuperpixelLSC>;
	
	ptr_extern! { dyn crate::ximgproc::SuperpixelLSC,
		cv_PtrOfSuperpixelLSC_delete, cv_PtrOfSuperpixelLSC_get_inner_ptr, cv_PtrOfSuperpixelLSC_get_inner_ptr_mut
	}
	
	impl PtrOfSuperpixelLSC {
		#[inline] pub fn as_raw_PtrOfSuperpixelLSC(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperpixelLSC(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::SuperpixelLSCConst for PtrOfSuperpixelLSC {
		#[inline] fn as_raw_SuperpixelLSC(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SuperpixelLSC for PtrOfSuperpixelLSC {
		#[inline] fn as_raw_mut_SuperpixelLSC(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSuperpixelLSC {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperpixelLSC {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperpixelSEEDS = core::Ptr<dyn crate::ximgproc::SuperpixelSEEDS>;
	
	ptr_extern! { dyn crate::ximgproc::SuperpixelSEEDS,
		cv_PtrOfSuperpixelSEEDS_delete, cv_PtrOfSuperpixelSEEDS_get_inner_ptr, cv_PtrOfSuperpixelSEEDS_get_inner_ptr_mut
	}
	
	impl PtrOfSuperpixelSEEDS {
		#[inline] pub fn as_raw_PtrOfSuperpixelSEEDS(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperpixelSEEDS(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::SuperpixelSEEDSConst for PtrOfSuperpixelSEEDS {
		#[inline] fn as_raw_SuperpixelSEEDS(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SuperpixelSEEDS for PtrOfSuperpixelSEEDS {
		#[inline] fn as_raw_mut_SuperpixelSEEDS(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSuperpixelSEEDS {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperpixelSEEDS {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperpixelSLIC = core::Ptr<dyn crate::ximgproc::SuperpixelSLIC>;
	
	ptr_extern! { dyn crate::ximgproc::SuperpixelSLIC,
		cv_PtrOfSuperpixelSLIC_delete, cv_PtrOfSuperpixelSLIC_get_inner_ptr, cv_PtrOfSuperpixelSLIC_get_inner_ptr_mut
	}
	
	impl PtrOfSuperpixelSLIC {
		#[inline] pub fn as_raw_PtrOfSuperpixelSLIC(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperpixelSLIC(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::SuperpixelSLICConst for PtrOfSuperpixelSLIC {
		#[inline] fn as_raw_SuperpixelSLIC(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SuperpixelSLIC for PtrOfSuperpixelSLIC {
		#[inline] fn as_raw_mut_SuperpixelSLIC(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSuperpixelSLIC {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSuperpixelSLIC {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_ximgproc)]
pub use ximgproc_types::*;

#[cfg(ocvrs_has_module_xobjdetect)]
mod xobjdetect_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfWBDetector = core::Ptr<dyn crate::xobjdetect::WBDetector>;
	
	ptr_extern! { dyn crate::xobjdetect::WBDetector,
		cv_PtrOfWBDetector_delete, cv_PtrOfWBDetector_get_inner_ptr, cv_PtrOfWBDetector_get_inner_ptr_mut
	}
	
	impl PtrOfWBDetector {
		#[inline] pub fn as_raw_PtrOfWBDetector(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfWBDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xobjdetect::WBDetectorConst for PtrOfWBDetector {
		#[inline] fn as_raw_WBDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xobjdetect::WBDetector for PtrOfWBDetector {
		#[inline] fn as_raw_mut_WBDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_xobjdetect)]
pub use xobjdetect_types::*;

#[cfg(ocvrs_has_module_xphoto)]
mod xphoto_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfGrayworldWB = core::Ptr<dyn crate::xphoto::GrayworldWB>;
	
	ptr_extern! { dyn crate::xphoto::GrayworldWB,
		cv_PtrOfGrayworldWB_delete, cv_PtrOfGrayworldWB_get_inner_ptr, cv_PtrOfGrayworldWB_get_inner_ptr_mut
	}
	
	impl PtrOfGrayworldWB {
		#[inline] pub fn as_raw_PtrOfGrayworldWB(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGrayworldWB(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xphoto::GrayworldWBConst for PtrOfGrayworldWB {
		#[inline] fn as_raw_GrayworldWB(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xphoto::GrayworldWB for PtrOfGrayworldWB {
		#[inline] fn as_raw_mut_GrayworldWB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfGrayworldWB {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfGrayworldWB {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xphoto::WhiteBalancerConst for PtrOfGrayworldWB {
		#[inline] fn as_raw_WhiteBalancer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xphoto::WhiteBalancer for PtrOfGrayworldWB {
		#[inline] fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLearningBasedWB = core::Ptr<dyn crate::xphoto::LearningBasedWB>;
	
	ptr_extern! { dyn crate::xphoto::LearningBasedWB,
		cv_PtrOfLearningBasedWB_delete, cv_PtrOfLearningBasedWB_get_inner_ptr, cv_PtrOfLearningBasedWB_get_inner_ptr_mut
	}
	
	impl PtrOfLearningBasedWB {
		#[inline] pub fn as_raw_PtrOfLearningBasedWB(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLearningBasedWB(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xphoto::LearningBasedWBConst for PtrOfLearningBasedWB {
		#[inline] fn as_raw_LearningBasedWB(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xphoto::LearningBasedWB for PtrOfLearningBasedWB {
		#[inline] fn as_raw_mut_LearningBasedWB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfLearningBasedWB {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfLearningBasedWB {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xphoto::WhiteBalancerConst for PtrOfLearningBasedWB {
		#[inline] fn as_raw_WhiteBalancer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xphoto::WhiteBalancer for PtrOfLearningBasedWB {
		#[inline] fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSimpleWB = core::Ptr<dyn crate::xphoto::SimpleWB>;
	
	ptr_extern! { dyn crate::xphoto::SimpleWB,
		cv_PtrOfSimpleWB_delete, cv_PtrOfSimpleWB_get_inner_ptr, cv_PtrOfSimpleWB_get_inner_ptr_mut
	}
	
	impl PtrOfSimpleWB {
		#[inline] pub fn as_raw_PtrOfSimpleWB(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSimpleWB(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xphoto::SimpleWBConst for PtrOfSimpleWB {
		#[inline] fn as_raw_SimpleWB(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xphoto::SimpleWB for PtrOfSimpleWB {
		#[inline] fn as_raw_mut_SimpleWB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfSimpleWB {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfSimpleWB {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xphoto::WhiteBalancerConst for PtrOfSimpleWB {
		#[inline] fn as_raw_WhiteBalancer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xphoto::WhiteBalancer for PtrOfSimpleWB {
		#[inline] fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemapDurand = core::Ptr<dyn crate::xphoto::TonemapDurand>;
	
	ptr_extern! { dyn crate::xphoto::TonemapDurand,
		cv_PtrOfTonemapDurand_delete, cv_PtrOfTonemapDurand_get_inner_ptr, cv_PtrOfTonemapDurand_get_inner_ptr_mut
	}
	
	impl PtrOfTonemapDurand {
		#[inline] pub fn as_raw_PtrOfTonemapDurand(&self) -> *const c_void { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTonemapDurand(&mut self) -> *mut c_void { self.as_raw_mut() }
	}
	
	impl crate::xphoto::TonemapDurandConst for PtrOfTonemapDurand {
		#[inline] fn as_raw_TonemapDurand(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xphoto::TonemapDurand for PtrOfTonemapDurand {
		#[inline] fn as_raw_mut_TonemapDurand(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for PtrOfTonemapDurand {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for PtrOfTonemapDurand {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::TonemapConst for PtrOfTonemapDurand {
		#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::Tonemap for PtrOfTonemapDurand {
		#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_xphoto)]
pub use xphoto_types::*;

pub use crate::manual::types::*;
