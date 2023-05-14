
#[cfg(ocvrs_has_module_aruco)]
mod aruco_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfEstimateParameters = core::Ptr<crate::aruco::EstimateParameters>;
	
	ptr_extern! { crate::aruco::EstimateParameters,
		cv_PtrOfEstimateParameters_delete, cv_PtrOfEstimateParameters_get_inner_ptr, cv_PtrOfEstimateParameters_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::aruco::EstimateParameters, cv_PtrOfEstimateParameters_new }
	
	impl core::Ptr<crate::aruco::EstimateParameters> {
		#[inline] pub fn as_raw_PtrOfEstimateParameters(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEstimateParameters(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::aruco::EstimateParametersTraitConst for core::Ptr<crate::aruco::EstimateParameters> {
		#[inline] fn as_raw_EstimateParameters(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::aruco::EstimateParametersTrait for core::Ptr<crate::aruco::EstimateParameters> {
		#[inline] fn as_raw_mut_EstimateParameters(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_aruco)]
pub use aruco_types::*;

#[cfg(ocvrs_has_module_barcode)]
mod barcode_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type VectorOfBarcodeType = core::Vector<crate::barcode::BarcodeType>;
	
	impl core::Vector<crate::barcode::BarcodeType> {
		pub fn as_raw_VectorOfBarcodeType(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfBarcodeType(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::barcode::BarcodeType,
		cv_VectorOfBarcodeType_new, cv_VectorOfBarcodeType_delete,
		cv_VectorOfBarcodeType_len, cv_VectorOfBarcodeType_is_empty,
		cv_VectorOfBarcodeType_capacity, cv_VectorOfBarcodeType_shrink_to_fit,
		cv_VectorOfBarcodeType_reserve, cv_VectorOfBarcodeType_remove,
		cv_VectorOfBarcodeType_swap, cv_VectorOfBarcodeType_clear,
		cv_VectorOfBarcodeType_get, cv_VectorOfBarcodeType_set,
		cv_VectorOfBarcodeType_push, cv_VectorOfBarcodeType_insert,
	}
	vector_copy_non_bool! { crate::barcode::BarcodeType,
		cv_VectorOfBarcodeType_data, cv_VectorOfBarcodeType_data_mut, cv_VectorOfBarcodeType_from_slice,
		cv_VectorOfBarcodeType_clone,
	}
	
}
#[cfg(ocvrs_has_module_barcode)]
pub use barcode_types::*;

#[cfg(ocvrs_has_module_bgsegm)]
mod bgsegm_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBackgroundSubtractorCNT = core::Ptr<crate::bgsegm::BackgroundSubtractorCNT>;
	
	ptr_extern! { crate::bgsegm::BackgroundSubtractorCNT,
		cv_PtrOfBackgroundSubtractorCNT_delete, cv_PtrOfBackgroundSubtractorCNT_get_inner_ptr, cv_PtrOfBackgroundSubtractorCNT_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::bgsegm::BackgroundSubtractorCNT> {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorCNT(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorCNT(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorCNTTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorCNT> {
		#[inline] fn as_raw_BackgroundSubtractorCNT(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorCNTTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorCNT> {
		#[inline] fn as_raw_mut_BackgroundSubtractorCNT(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorCNT> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorCNT> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorCNT> {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractorTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorCNT> {
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackgroundSubtractorGMG = core::Ptr<crate::bgsegm::BackgroundSubtractorGMG>;
	
	ptr_extern! { crate::bgsegm::BackgroundSubtractorGMG,
		cv_PtrOfBackgroundSubtractorGMG_delete, cv_PtrOfBackgroundSubtractorGMG_get_inner_ptr, cv_PtrOfBackgroundSubtractorGMG_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::bgsegm::BackgroundSubtractorGMG> {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorGMG(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorGMG(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorGMGTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorGMG> {
		#[inline] fn as_raw_BackgroundSubtractorGMG(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorGMGTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorGMG> {
		#[inline] fn as_raw_mut_BackgroundSubtractorGMG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorGMG> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorGMG> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorGMG> {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractorTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorGMG> {
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackgroundSubtractorGSOC = core::Ptr<crate::bgsegm::BackgroundSubtractorGSOC>;
	
	ptr_extern! { crate::bgsegm::BackgroundSubtractorGSOC,
		cv_PtrOfBackgroundSubtractorGSOC_delete, cv_PtrOfBackgroundSubtractorGSOC_get_inner_ptr, cv_PtrOfBackgroundSubtractorGSOC_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::bgsegm::BackgroundSubtractorGSOC> {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorGSOC(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorGSOC(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorGSOCTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorGSOC> {
		#[inline] fn as_raw_BackgroundSubtractorGSOC(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorGSOCTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorGSOC> {
		#[inline] fn as_raw_mut_BackgroundSubtractorGSOC(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorGSOC> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorGSOC> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorGSOC> {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractorTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorGSOC> {
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackgroundSubtractorLSBP = core::Ptr<crate::bgsegm::BackgroundSubtractorLSBP>;
	
	ptr_extern! { crate::bgsegm::BackgroundSubtractorLSBP,
		cv_PtrOfBackgroundSubtractorLSBP_delete, cv_PtrOfBackgroundSubtractorLSBP_get_inner_ptr, cv_PtrOfBackgroundSubtractorLSBP_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::bgsegm::BackgroundSubtractorLSBP> {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorLSBP(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorLSBP(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorLSBPTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorLSBP> {
		#[inline] fn as_raw_BackgroundSubtractorLSBP(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorLSBPTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorLSBP> {
		#[inline] fn as_raw_mut_BackgroundSubtractorLSBP(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorLSBP> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorLSBP> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorLSBP> {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractorTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorLSBP> {
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackgroundSubtractorMOG = core::Ptr<crate::bgsegm::BackgroundSubtractorMOG>;
	
	ptr_extern! { crate::bgsegm::BackgroundSubtractorMOG,
		cv_PtrOfBackgroundSubtractorMOG_delete, cv_PtrOfBackgroundSubtractorMOG_get_inner_ptr, cv_PtrOfBackgroundSubtractorMOG_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::bgsegm::BackgroundSubtractorMOG> {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorMOG(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorMOG(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorMOGTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorMOG> {
		#[inline] fn as_raw_BackgroundSubtractorMOG(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::bgsegm::BackgroundSubtractorMOGTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorMOG> {
		#[inline] fn as_raw_mut_BackgroundSubtractorMOG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorMOG> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorMOG> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorTraitConst for core::Ptr<crate::bgsegm::BackgroundSubtractorMOG> {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractorTrait for core::Ptr<crate::bgsegm::BackgroundSubtractorMOG> {
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSyntheticSequenceGenerator = core::Ptr<crate::bgsegm::SyntheticSequenceGenerator>;
	
	ptr_extern! { crate::bgsegm::SyntheticSequenceGenerator,
		cv_PtrOfSyntheticSequenceGenerator_delete, cv_PtrOfSyntheticSequenceGenerator_get_inner_ptr, cv_PtrOfSyntheticSequenceGenerator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::bgsegm::SyntheticSequenceGenerator, cv_PtrOfSyntheticSequenceGenerator_new }
	
	impl core::Ptr<crate::bgsegm::SyntheticSequenceGenerator> {
		#[inline] pub fn as_raw_PtrOfSyntheticSequenceGenerator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSyntheticSequenceGenerator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::bgsegm::SyntheticSequenceGeneratorTraitConst for core::Ptr<crate::bgsegm::SyntheticSequenceGenerator> {
		#[inline] fn as_raw_SyntheticSequenceGenerator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::bgsegm::SyntheticSequenceGeneratorTrait for core::Ptr<crate::bgsegm::SyntheticSequenceGenerator> {
		#[inline] fn as_raw_mut_SyntheticSequenceGenerator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::bgsegm::SyntheticSequenceGenerator> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::bgsegm::SyntheticSequenceGenerator> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_bgsegm)]
pub use bgsegm_types::*;

#[cfg(ocvrs_has_module_bioinspired)]
mod bioinspired_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfRetina = core::Ptr<crate::bioinspired::Retina>;
	
	ptr_extern! { crate::bioinspired::Retina,
		cv_PtrOfRetina_delete, cv_PtrOfRetina_get_inner_ptr, cv_PtrOfRetina_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::bioinspired::Retina> {
		#[inline] pub fn as_raw_PtrOfRetina(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRetina(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::bioinspired::RetinaTraitConst for core::Ptr<crate::bioinspired::Retina> {
		#[inline] fn as_raw_Retina(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::bioinspired::RetinaTrait for core::Ptr<crate::bioinspired::Retina> {
		#[inline] fn as_raw_mut_Retina(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::bioinspired::Retina> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::bioinspired::Retina> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRetinaFastToneMapping = core::Ptr<crate::bioinspired::RetinaFastToneMapping>;
	
	ptr_extern! { crate::bioinspired::RetinaFastToneMapping,
		cv_PtrOfRetinaFastToneMapping_delete, cv_PtrOfRetinaFastToneMapping_get_inner_ptr, cv_PtrOfRetinaFastToneMapping_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::bioinspired::RetinaFastToneMapping> {
		#[inline] pub fn as_raw_PtrOfRetinaFastToneMapping(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRetinaFastToneMapping(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::bioinspired::RetinaFastToneMappingTraitConst for core::Ptr<crate::bioinspired::RetinaFastToneMapping> {
		#[inline] fn as_raw_RetinaFastToneMapping(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::bioinspired::RetinaFastToneMappingTrait for core::Ptr<crate::bioinspired::RetinaFastToneMapping> {
		#[inline] fn as_raw_mut_RetinaFastToneMapping(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::bioinspired::RetinaFastToneMapping> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::bioinspired::RetinaFastToneMapping> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTransientAreasSegmentationModule = core::Ptr<crate::bioinspired::TransientAreasSegmentationModule>;
	
	ptr_extern! { crate::bioinspired::TransientAreasSegmentationModule,
		cv_PtrOfTransientAreasSegmentationModule_delete, cv_PtrOfTransientAreasSegmentationModule_get_inner_ptr, cv_PtrOfTransientAreasSegmentationModule_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::bioinspired::TransientAreasSegmentationModule> {
		#[inline] pub fn as_raw_PtrOfTransientAreasSegmentationModule(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTransientAreasSegmentationModule(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::bioinspired::TransientAreasSegmentationModuleTraitConst for core::Ptr<crate::bioinspired::TransientAreasSegmentationModule> {
		#[inline] fn as_raw_TransientAreasSegmentationModule(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::bioinspired::TransientAreasSegmentationModuleTrait for core::Ptr<crate::bioinspired::TransientAreasSegmentationModule> {
		#[inline] fn as_raw_mut_TransientAreasSegmentationModule(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::bioinspired::TransientAreasSegmentationModule> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::bioinspired::TransientAreasSegmentationModule> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_bioinspired)]
pub use bioinspired_types::*;

#[cfg(ocvrs_has_module_calib3d)]
mod calib3d_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfLMSolver = core::Ptr<crate::calib3d::LMSolver>;
	
	ptr_extern! { crate::calib3d::LMSolver,
		cv_PtrOfLMSolver_delete, cv_PtrOfLMSolver_get_inner_ptr, cv_PtrOfLMSolver_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::calib3d::LMSolver> {
		#[inline] pub fn as_raw_PtrOfLMSolver(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLMSolver(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::calib3d::LMSolverTraitConst for core::Ptr<crate::calib3d::LMSolver> {
		#[inline] fn as_raw_LMSolver(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::LMSolverTrait for core::Ptr<crate::calib3d::LMSolver> {
		#[inline] fn as_raw_mut_LMSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::calib3d::LMSolver> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::calib3d::LMSolver> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLMSolver_Callback = core::Ptr<crate::calib3d::LMSolver_Callback>;
	
	ptr_extern! { crate::calib3d::LMSolver_Callback,
		cv_PtrOfLMSolver_Callback_delete, cv_PtrOfLMSolver_Callback_get_inner_ptr, cv_PtrOfLMSolver_Callback_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::calib3d::LMSolver_Callback> {
		#[inline] pub fn as_raw_PtrOfLMSolver_Callback(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLMSolver_Callback(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::calib3d::LMSolver_CallbackTraitConst for core::Ptr<crate::calib3d::LMSolver_Callback> {
		#[inline] fn as_raw_LMSolver_Callback(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::LMSolver_CallbackTrait for core::Ptr<crate::calib3d::LMSolver_Callback> {
		#[inline] fn as_raw_mut_LMSolver_Callback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStereoBM = core::Ptr<crate::calib3d::StereoBM>;
	
	ptr_extern! { crate::calib3d::StereoBM,
		cv_PtrOfStereoBM_delete, cv_PtrOfStereoBM_get_inner_ptr, cv_PtrOfStereoBM_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::calib3d::StereoBM> {
		#[inline] pub fn as_raw_PtrOfStereoBM(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStereoBM(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::calib3d::StereoBMTraitConst for core::Ptr<crate::calib3d::StereoBM> {
		#[inline] fn as_raw_StereoBM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoBMTrait for core::Ptr<crate::calib3d::StereoBM> {
		#[inline] fn as_raw_mut_StereoBM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::calib3d::StereoBM> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::calib3d::StereoBM> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcherTraitConst for core::Ptr<crate::calib3d::StereoBM> {
		#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoMatcherTrait for core::Ptr<crate::calib3d::StereoBM> {
		#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStereoMatcher = core::Ptr<crate::calib3d::StereoMatcher>;
	
	ptr_extern! { crate::calib3d::StereoMatcher,
		cv_PtrOfStereoMatcher_delete, cv_PtrOfStereoMatcher_get_inner_ptr, cv_PtrOfStereoMatcher_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::calib3d::StereoMatcher> {
		#[inline] pub fn as_raw_PtrOfStereoMatcher(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStereoMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcherTraitConst for core::Ptr<crate::calib3d::StereoMatcher> {
		#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoMatcherTrait for core::Ptr<crate::calib3d::StereoMatcher> {
		#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::calib3d::StereoMatcher> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::calib3d::StereoMatcher> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStereoSGBM = core::Ptr<crate::calib3d::StereoSGBM>;
	
	ptr_extern! { crate::calib3d::StereoSGBM,
		cv_PtrOfStereoSGBM_delete, cv_PtrOfStereoSGBM_get_inner_ptr, cv_PtrOfStereoSGBM_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::calib3d::StereoSGBM> {
		#[inline] pub fn as_raw_PtrOfStereoSGBM(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStereoSGBM(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::calib3d::StereoSGBMTraitConst for core::Ptr<crate::calib3d::StereoSGBM> {
		#[inline] fn as_raw_StereoSGBM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoSGBMTrait for core::Ptr<crate::calib3d::StereoSGBM> {
		#[inline] fn as_raw_mut_StereoSGBM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::calib3d::StereoSGBM> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::calib3d::StereoSGBM> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcherTraitConst for core::Ptr<crate::calib3d::StereoSGBM> {
		#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoMatcherTrait for core::Ptr<crate::calib3d::StereoSGBM> {
		#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_calib3d)]
pub use calib3d_types::*;

#[cfg(ocvrs_has_module_core)]
mod core_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfConjGradSolver = core::Ptr<core::ConjGradSolver>;
	
	ptr_extern! { core::ConjGradSolver,
		cv_PtrOfConjGradSolver_delete, cv_PtrOfConjGradSolver_get_inner_ptr, cv_PtrOfConjGradSolver_get_inner_ptr_mut
	}
	
	impl core::Ptr<core::ConjGradSolver> {
		#[inline] pub fn as_raw_PtrOfConjGradSolver(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConjGradSolver(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl core::ConjGradSolverTraitConst for core::Ptr<core::ConjGradSolver> {
		#[inline] fn as_raw_ConjGradSolver(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::ConjGradSolverTrait for core::Ptr<core::ConjGradSolver> {
		#[inline] fn as_raw_mut_ConjGradSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<core::ConjGradSolver> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<core::ConjGradSolver> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::MinProblemSolverTraitConst for core::Ptr<core::ConjGradSolver> {
		#[inline] fn as_raw_MinProblemSolver(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::MinProblemSolverTrait for core::Ptr<core::ConjGradSolver> {
		#[inline] fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDownhillSolver = core::Ptr<core::DownhillSolver>;
	
	ptr_extern! { core::DownhillSolver,
		cv_PtrOfDownhillSolver_delete, cv_PtrOfDownhillSolver_get_inner_ptr, cv_PtrOfDownhillSolver_get_inner_ptr_mut
	}
	
	impl core::Ptr<core::DownhillSolver> {
		#[inline] pub fn as_raw_PtrOfDownhillSolver(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDownhillSolver(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl core::DownhillSolverTraitConst for core::Ptr<core::DownhillSolver> {
		#[inline] fn as_raw_DownhillSolver(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::DownhillSolverTrait for core::Ptr<core::DownhillSolver> {
		#[inline] fn as_raw_mut_DownhillSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<core::DownhillSolver> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<core::DownhillSolver> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::MinProblemSolverTraitConst for core::Ptr<core::DownhillSolver> {
		#[inline] fn as_raw_MinProblemSolver(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::MinProblemSolverTrait for core::Ptr<core::DownhillSolver> {
		#[inline] fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFileStorage = core::Ptr<core::FileStorage>;
	
	ptr_extern! { core::FileStorage,
		cv_PtrOfFileStorage_delete, cv_PtrOfFileStorage_get_inner_ptr, cv_PtrOfFileStorage_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { core::FileStorage, cv_PtrOfFileStorage_new }
	
	impl core::Ptr<core::FileStorage> {
		#[inline] pub fn as_raw_PtrOfFileStorage(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFileStorage(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl core::FileStorageTraitConst for core::Ptr<core::FileStorage> {
		#[inline] fn as_raw_FileStorage(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::FileStorageTrait for core::Ptr<core::FileStorage> {
		#[inline] fn as_raw_mut_FileStorage(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFormatted = core::Ptr<core::Formatted>;
	
	ptr_extern! { core::Formatted,
		cv_PtrOfFormatted_delete, cv_PtrOfFormatted_get_inner_ptr, cv_PtrOfFormatted_get_inner_ptr_mut
	}
	
	impl core::Ptr<core::Formatted> {
		#[inline] pub fn as_raw_PtrOfFormatted(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFormatted(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl core::FormattedTraitConst for core::Ptr<core::Formatted> {
		#[inline] fn as_raw_Formatted(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::FormattedTrait for core::Ptr<core::Formatted> {
		#[inline] fn as_raw_mut_Formatted(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFormatter = core::Ptr<core::Formatter>;
	
	ptr_extern! { core::Formatter,
		cv_PtrOfFormatter_delete, cv_PtrOfFormatter_get_inner_ptr, cv_PtrOfFormatter_get_inner_ptr_mut
	}
	
	impl core::Ptr<core::Formatter> {
		#[inline] pub fn as_raw_PtrOfFormatter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFormatter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl core::FormatterTraitConst for core::Ptr<core::Formatter> {
		#[inline] fn as_raw_Formatter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::FormatterTrait for core::Ptr<core::Formatter> {
		#[inline] fn as_raw_mut_Formatter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGpuMat_Allocator = core::Ptr<core::GpuMat_Allocator>;
	
	ptr_extern! { core::GpuMat_Allocator,
		cv_PtrOfGpuMat_Allocator_delete, cv_PtrOfGpuMat_Allocator_get_inner_ptr, cv_PtrOfGpuMat_Allocator_get_inner_ptr_mut
	}
	
	impl core::Ptr<core::GpuMat_Allocator> {
		#[inline] pub fn as_raw_PtrOfGpuMat_Allocator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGpuMat_Allocator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl core::GpuMat_AllocatorTraitConst for core::Ptr<core::GpuMat_Allocator> {
		#[inline] fn as_raw_GpuMat_Allocator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::GpuMat_AllocatorTrait for core::Ptr<core::GpuMat_Allocator> {
		#[inline] fn as_raw_mut_GpuMat_Allocator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMinProblemSolver_Function = core::Ptr<core::MinProblemSolver_Function>;
	
	ptr_extern! { core::MinProblemSolver_Function,
		cv_PtrOfMinProblemSolver_Function_delete, cv_PtrOfMinProblemSolver_Function_get_inner_ptr, cv_PtrOfMinProblemSolver_Function_get_inner_ptr_mut
	}
	
	impl core::Ptr<core::MinProblemSolver_Function> {
		#[inline] pub fn as_raw_PtrOfMinProblemSolver_Function(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMinProblemSolver_Function(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl core::MinProblemSolver_FunctionTraitConst for core::Ptr<core::MinProblemSolver_Function> {
		#[inline] fn as_raw_MinProblemSolver_Function(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::MinProblemSolver_FunctionTrait for core::Ptr<core::MinProblemSolver_Function> {
		#[inline] fn as_raw_mut_MinProblemSolver_Function(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOriginalClassName = core::Ptr<core::OriginalClassName>;
	
	ptr_extern! { core::OriginalClassName,
		cv_PtrOfOriginalClassName_delete, cv_PtrOfOriginalClassName_get_inner_ptr, cv_PtrOfOriginalClassName_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { core::OriginalClassName, cv_PtrOfOriginalClassName_new }
	
	impl core::Ptr<core::OriginalClassName> {
		#[inline] pub fn as_raw_PtrOfOriginalClassName(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOriginalClassName(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl core::OriginalClassNameTraitConst for core::Ptr<core::OriginalClassName> {
		#[inline] fn as_raw_OriginalClassName(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::OriginalClassNameTrait for core::Ptr<core::OriginalClassName> {
		#[inline] fn as_raw_mut_OriginalClassName(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOff32 = core::Ptr<f32>;
	
	ptr_extern! { f32,
		cv_PtrOff32_delete, cv_PtrOff32_get_inner_ptr, cv_PtrOff32_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { f32, cv_PtrOff32_new }
	
	impl core::Ptr<f32> {
		#[inline] pub fn as_raw_PtrOff32(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOff32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	pub type TupleOfPoint2i_Point2i = core::Tuple<(core::Point2i, core::Point2i)>;
	
	impl core::Tuple<(core::Point2i, core::Point2i)> {
		pub fn as_raw_TupleOfPoint2i_Point2i(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfPoint2i_Point2i(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	tuple_extern! { (core::Point2i, core::Point2i),
		cv_TupleOfPoint2i_Point2i_new, cv_TupleOfPoint2i_Point2i_delete,
		0 = arg: core::Point2i, get_0 via cv_TupleOfPoint2i_Point2i_get_0,
		1 = arg_1: core::Point2i, get_1 via cv_TupleOfPoint2i_Point2i_get_1
	}
	
	pub type TupleOfRect_i32 = core::Tuple<(core::Rect, i32)>;
	
	impl core::Tuple<(core::Rect, i32)> {
		pub fn as_raw_TupleOfRect_i32(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfRect_i32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	tuple_extern! { (core::Rect, i32),
		cv_TupleOfRect_i32_new, cv_TupleOfRect_i32_delete,
		0 = arg: core::Rect, get_0 via cv_TupleOfRect_i32_get_0,
		1 = arg_1: i32, get_1 via cv_TupleOfRect_i32_get_1
	}
	
	pub type TupleOfUMat_u8 = core::Tuple<(core::UMat, u8)>;
	
	impl core::Tuple<(core::UMat, u8)> {
		pub fn as_raw_TupleOfUMat_u8(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfUMat_u8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	tuple_extern! { (core::UMat, u8),
		cv_TupleOfUMat_u8_new, cv_TupleOfUMat_u8_delete,
		0 = arg: core::UMat, get_0 via cv_TupleOfUMat_u8_get_0,
		1 = arg_1: u8, get_1 via cv_TupleOfUMat_u8_get_1
	}
	
	pub type TupleOfi32_f32 = core::Tuple<(i32, f32)>;
	
	impl core::Tuple<(i32, f32)> {
		pub fn as_raw_TupleOfi32_f32(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfi32_f32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	tuple_extern! { (i32, f32),
		cv_TupleOfi32_f32_new, cv_TupleOfi32_f32_delete,
		0 = arg: i32, get_0 via cv_TupleOfi32_f32_get_0,
		1 = arg_1: f32, get_1 via cv_TupleOfi32_f32_get_1
	}
	
	pub type TupleOfi32_f64 = core::Tuple<(i32, f64)>;
	
	impl core::Tuple<(i32, f64)> {
		pub fn as_raw_TupleOfi32_f64(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfi32_f64(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	tuple_extern! { (i32, f64),
		cv_TupleOfi32_f64_new, cv_TupleOfi32_f64_delete,
		0 = arg: i32, get_0 via cv_TupleOfi32_f64_get_0,
		1 = arg_1: f64, get_1 via cv_TupleOfi32_f64_get_1
	}
	
	pub type VectorOfDMatch = core::Vector<core::DMatch>;
	
	impl core::Vector<core::DMatch> {
		pub fn as_raw_VectorOfDMatch(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDMatch(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::DMatch,
		cv_VectorOfDMatch_new, cv_VectorOfDMatch_delete,
		cv_VectorOfDMatch_len, cv_VectorOfDMatch_is_empty,
		cv_VectorOfDMatch_capacity, cv_VectorOfDMatch_shrink_to_fit,
		cv_VectorOfDMatch_reserve, cv_VectorOfDMatch_remove,
		cv_VectorOfDMatch_swap, cv_VectorOfDMatch_clear,
		cv_VectorOfDMatch_get, cv_VectorOfDMatch_set,
		cv_VectorOfDMatch_push, cv_VectorOfDMatch_insert,
	}
	vector_copy_non_bool! { core::DMatch,
		cv_VectorOfDMatch_data, cv_VectorOfDMatch_data_mut, cv_VectorOfDMatch_from_slice,
		cv_VectorOfDMatch_clone,
	}
	
	pub type VectorOfGpuMat = core::Vector<core::GpuMat>;
	
	impl core::Vector<core::GpuMat> {
		pub fn as_raw_VectorOfGpuMat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfGpuMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::GpuMat,
		cv_VectorOfGpuMat_new, cv_VectorOfGpuMat_delete,
		cv_VectorOfGpuMat_len, cv_VectorOfGpuMat_is_empty,
		cv_VectorOfGpuMat_capacity, cv_VectorOfGpuMat_shrink_to_fit,
		cv_VectorOfGpuMat_reserve, cv_VectorOfGpuMat_remove,
		cv_VectorOfGpuMat_swap, cv_VectorOfGpuMat_clear,
		cv_VectorOfGpuMat_get, cv_VectorOfGpuMat_set,
		cv_VectorOfGpuMat_push, cv_VectorOfGpuMat_insert,
	}
	vector_non_copy_or_bool! { clone core::GpuMat }
	
	pub type VectorOfKeyPoint = core::Vector<core::KeyPoint>;
	
	impl core::Vector<core::KeyPoint> {
		pub fn as_raw_VectorOfKeyPoint(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfKeyPoint(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::KeyPoint,
		cv_VectorOfKeyPoint_new, cv_VectorOfKeyPoint_delete,
		cv_VectorOfKeyPoint_len, cv_VectorOfKeyPoint_is_empty,
		cv_VectorOfKeyPoint_capacity, cv_VectorOfKeyPoint_shrink_to_fit,
		cv_VectorOfKeyPoint_reserve, cv_VectorOfKeyPoint_remove,
		cv_VectorOfKeyPoint_swap, cv_VectorOfKeyPoint_clear,
		cv_VectorOfKeyPoint_get, cv_VectorOfKeyPoint_set,
		cv_VectorOfKeyPoint_push, cv_VectorOfKeyPoint_insert,
	}
	vector_non_copy_or_bool! { clone core::KeyPoint }
	
	pub type VectorOfMat = core::Vector<core::Mat>;
	
	impl core::Vector<core::Mat> {
		pub fn as_raw_VectorOfMat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Mat,
		cv_VectorOfMat_new, cv_VectorOfMat_delete,
		cv_VectorOfMat_len, cv_VectorOfMat_is_empty,
		cv_VectorOfMat_capacity, cv_VectorOfMat_shrink_to_fit,
		cv_VectorOfMat_reserve, cv_VectorOfMat_remove,
		cv_VectorOfMat_swap, cv_VectorOfMat_clear,
		cv_VectorOfMat_get, cv_VectorOfMat_set,
		cv_VectorOfMat_push, cv_VectorOfMat_insert,
	}
	vector_non_copy_or_bool! { clone core::Mat }
	
	pub type VectorOfPlatformInfo = core::Vector<core::PlatformInfo>;
	
	impl core::Vector<core::PlatformInfo> {
		pub fn as_raw_VectorOfPlatformInfo(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPlatformInfo(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::PlatformInfo,
		cv_VectorOfPlatformInfo_new, cv_VectorOfPlatformInfo_delete,
		cv_VectorOfPlatformInfo_len, cv_VectorOfPlatformInfo_is_empty,
		cv_VectorOfPlatformInfo_capacity, cv_VectorOfPlatformInfo_shrink_to_fit,
		cv_VectorOfPlatformInfo_reserve, cv_VectorOfPlatformInfo_remove,
		cv_VectorOfPlatformInfo_swap, cv_VectorOfPlatformInfo_clear,
		cv_VectorOfPlatformInfo_get, cv_VectorOfPlatformInfo_set,
		cv_VectorOfPlatformInfo_push, cv_VectorOfPlatformInfo_insert,
	}
	vector_non_copy_or_bool! { core::PlatformInfo }
	
	pub type VectorOfPoint = core::Vector<core::Point>;
	
	impl core::Vector<core::Point> {
		pub fn as_raw_VectorOfPoint(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point,
		cv_VectorOfPoint_new, cv_VectorOfPoint_delete,
		cv_VectorOfPoint_len, cv_VectorOfPoint_is_empty,
		cv_VectorOfPoint_capacity, cv_VectorOfPoint_shrink_to_fit,
		cv_VectorOfPoint_reserve, cv_VectorOfPoint_remove,
		cv_VectorOfPoint_swap, cv_VectorOfPoint_clear,
		cv_VectorOfPoint_get, cv_VectorOfPoint_set,
		cv_VectorOfPoint_push, cv_VectorOfPoint_insert,
	}
	vector_copy_non_bool! { core::Point,
		cv_VectorOfPoint_data, cv_VectorOfPoint_data_mut, cv_VectorOfPoint_from_slice,
		cv_VectorOfPoint_clone,
	}
	
	extern "C" {
		fn cv_VectorOfPoint_input_array(instance: extern_send!(core::Vector<core::Point>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfPoint_output_array(instance: extern_send!(mut core::Vector<core::Point>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfPoint_input_output_array(instance: extern_send!(mut core::Vector<core::Point>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Point> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint_input_array(self.as_raw_VectorOfPoint(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Point> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint_output_array(self.as_raw_mut_VectorOfPoint(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Point> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint_input_output_array(self.as_raw_mut_VectorOfPoint(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Point> }
	
	pub type VectorOfPoint2d = core::Vector<core::Point2d>;
	
	impl core::Vector<core::Point2d> {
		pub fn as_raw_VectorOfPoint2d(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint2d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point2d,
		cv_VectorOfPoint2d_new, cv_VectorOfPoint2d_delete,
		cv_VectorOfPoint2d_len, cv_VectorOfPoint2d_is_empty,
		cv_VectorOfPoint2d_capacity, cv_VectorOfPoint2d_shrink_to_fit,
		cv_VectorOfPoint2d_reserve, cv_VectorOfPoint2d_remove,
		cv_VectorOfPoint2d_swap, cv_VectorOfPoint2d_clear,
		cv_VectorOfPoint2d_get, cv_VectorOfPoint2d_set,
		cv_VectorOfPoint2d_push, cv_VectorOfPoint2d_insert,
	}
	vector_copy_non_bool! { core::Point2d,
		cv_VectorOfPoint2d_data, cv_VectorOfPoint2d_data_mut, cv_VectorOfPoint2d_from_slice,
		cv_VectorOfPoint2d_clone,
	}
	
	extern "C" {
		fn cv_VectorOfPoint2d_input_array(instance: extern_send!(core::Vector<core::Point2d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfPoint2d_output_array(instance: extern_send!(mut core::Vector<core::Point2d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfPoint2d_input_output_array(instance: extern_send!(mut core::Vector<core::Point2d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Point2d> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint2d_input_array(self.as_raw_VectorOfPoint2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Point2d> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint2d_output_array(self.as_raw_mut_VectorOfPoint2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Point2d> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint2d_input_output_array(self.as_raw_mut_VectorOfPoint2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Point2d> }
	
	pub type VectorOfPoint2f = core::Vector<core::Point2f>;
	
	impl core::Vector<core::Point2f> {
		pub fn as_raw_VectorOfPoint2f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint2f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point2f,
		cv_VectorOfPoint2f_new, cv_VectorOfPoint2f_delete,
		cv_VectorOfPoint2f_len, cv_VectorOfPoint2f_is_empty,
		cv_VectorOfPoint2f_capacity, cv_VectorOfPoint2f_shrink_to_fit,
		cv_VectorOfPoint2f_reserve, cv_VectorOfPoint2f_remove,
		cv_VectorOfPoint2f_swap, cv_VectorOfPoint2f_clear,
		cv_VectorOfPoint2f_get, cv_VectorOfPoint2f_set,
		cv_VectorOfPoint2f_push, cv_VectorOfPoint2f_insert,
	}
	vector_copy_non_bool! { core::Point2f,
		cv_VectorOfPoint2f_data, cv_VectorOfPoint2f_data_mut, cv_VectorOfPoint2f_from_slice,
		cv_VectorOfPoint2f_clone,
	}
	
	extern "C" {
		fn cv_VectorOfPoint2f_input_array(instance: extern_send!(core::Vector<core::Point2f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfPoint2f_output_array(instance: extern_send!(mut core::Vector<core::Point2f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfPoint2f_input_output_array(instance: extern_send!(mut core::Vector<core::Point2f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Point2f> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint2f_input_array(self.as_raw_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Point2f> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint2f_output_array(self.as_raw_mut_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Point2f> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint2f_input_output_array(self.as_raw_mut_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Point2f> }
	
	pub type VectorOfPoint3d = core::Vector<core::Point3d>;
	
	impl core::Vector<core::Point3d> {
		pub fn as_raw_VectorOfPoint3d(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint3d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point3d,
		cv_VectorOfPoint3d_new, cv_VectorOfPoint3d_delete,
		cv_VectorOfPoint3d_len, cv_VectorOfPoint3d_is_empty,
		cv_VectorOfPoint3d_capacity, cv_VectorOfPoint3d_shrink_to_fit,
		cv_VectorOfPoint3d_reserve, cv_VectorOfPoint3d_remove,
		cv_VectorOfPoint3d_swap, cv_VectorOfPoint3d_clear,
		cv_VectorOfPoint3d_get, cv_VectorOfPoint3d_set,
		cv_VectorOfPoint3d_push, cv_VectorOfPoint3d_insert,
	}
	vector_copy_non_bool! { core::Point3d,
		cv_VectorOfPoint3d_data, cv_VectorOfPoint3d_data_mut, cv_VectorOfPoint3d_from_slice,
		cv_VectorOfPoint3d_clone,
	}
	
	extern "C" {
		fn cv_VectorOfPoint3d_input_array(instance: extern_send!(core::Vector<core::Point3d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfPoint3d_output_array(instance: extern_send!(mut core::Vector<core::Point3d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfPoint3d_input_output_array(instance: extern_send!(mut core::Vector<core::Point3d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Point3d> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3d_input_array(self.as_raw_VectorOfPoint3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Point3d> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3d_output_array(self.as_raw_mut_VectorOfPoint3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Point3d> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3d_input_output_array(self.as_raw_mut_VectorOfPoint3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Point3d> }
	
	pub type VectorOfPoint3f = core::Vector<core::Point3f>;
	
	impl core::Vector<core::Point3f> {
		pub fn as_raw_VectorOfPoint3f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint3f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point3f,
		cv_VectorOfPoint3f_new, cv_VectorOfPoint3f_delete,
		cv_VectorOfPoint3f_len, cv_VectorOfPoint3f_is_empty,
		cv_VectorOfPoint3f_capacity, cv_VectorOfPoint3f_shrink_to_fit,
		cv_VectorOfPoint3f_reserve, cv_VectorOfPoint3f_remove,
		cv_VectorOfPoint3f_swap, cv_VectorOfPoint3f_clear,
		cv_VectorOfPoint3f_get, cv_VectorOfPoint3f_set,
		cv_VectorOfPoint3f_push, cv_VectorOfPoint3f_insert,
	}
	vector_copy_non_bool! { core::Point3f,
		cv_VectorOfPoint3f_data, cv_VectorOfPoint3f_data_mut, cv_VectorOfPoint3f_from_slice,
		cv_VectorOfPoint3f_clone,
	}
	
	extern "C" {
		fn cv_VectorOfPoint3f_input_array(instance: extern_send!(core::Vector<core::Point3f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfPoint3f_output_array(instance: extern_send!(mut core::Vector<core::Point3f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfPoint3f_input_output_array(instance: extern_send!(mut core::Vector<core::Point3f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Point3f> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3f_input_array(self.as_raw_VectorOfPoint3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Point3f> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3f_output_array(self.as_raw_mut_VectorOfPoint3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Point3f> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3f_input_output_array(self.as_raw_mut_VectorOfPoint3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Point3f> }
	
	pub type VectorOfPoint3i = core::Vector<core::Point3i>;
	
	impl core::Vector<core::Point3i> {
		pub fn as_raw_VectorOfPoint3i(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint3i(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Point3i,
		cv_VectorOfPoint3i_new, cv_VectorOfPoint3i_delete,
		cv_VectorOfPoint3i_len, cv_VectorOfPoint3i_is_empty,
		cv_VectorOfPoint3i_capacity, cv_VectorOfPoint3i_shrink_to_fit,
		cv_VectorOfPoint3i_reserve, cv_VectorOfPoint3i_remove,
		cv_VectorOfPoint3i_swap, cv_VectorOfPoint3i_clear,
		cv_VectorOfPoint3i_get, cv_VectorOfPoint3i_set,
		cv_VectorOfPoint3i_push, cv_VectorOfPoint3i_insert,
	}
	vector_copy_non_bool! { core::Point3i,
		cv_VectorOfPoint3i_data, cv_VectorOfPoint3i_data_mut, cv_VectorOfPoint3i_from_slice,
		cv_VectorOfPoint3i_clone,
	}
	
	extern "C" {
		fn cv_VectorOfPoint3i_input_array(instance: extern_send!(core::Vector<core::Point3i>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfPoint3i_output_array(instance: extern_send!(mut core::Vector<core::Point3i>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfPoint3i_input_output_array(instance: extern_send!(mut core::Vector<core::Point3i>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Point3i> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3i_input_array(self.as_raw_VectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Point3i> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3i_output_array(self.as_raw_mut_VectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Point3i> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfPoint3i_input_output_array(self.as_raw_mut_VectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Point3i> }
	
	pub type VectorOfRange = core::Vector<core::Range>;
	
	impl core::Vector<core::Range> {
		pub fn as_raw_VectorOfRange(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfRange(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Range,
		cv_VectorOfRange_new, cv_VectorOfRange_delete,
		cv_VectorOfRange_len, cv_VectorOfRange_is_empty,
		cv_VectorOfRange_capacity, cv_VectorOfRange_shrink_to_fit,
		cv_VectorOfRange_reserve, cv_VectorOfRange_remove,
		cv_VectorOfRange_swap, cv_VectorOfRange_clear,
		cv_VectorOfRange_get, cv_VectorOfRange_set,
		cv_VectorOfRange_push, cv_VectorOfRange_insert,
	}
	vector_non_copy_or_bool! { core::Range }
	
	pub type VectorOfRect = core::Vector<core::Rect>;
	
	impl core::Vector<core::Rect> {
		pub fn as_raw_VectorOfRect(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfRect(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Rect,
		cv_VectorOfRect_new, cv_VectorOfRect_delete,
		cv_VectorOfRect_len, cv_VectorOfRect_is_empty,
		cv_VectorOfRect_capacity, cv_VectorOfRect_shrink_to_fit,
		cv_VectorOfRect_reserve, cv_VectorOfRect_remove,
		cv_VectorOfRect_swap, cv_VectorOfRect_clear,
		cv_VectorOfRect_get, cv_VectorOfRect_set,
		cv_VectorOfRect_push, cv_VectorOfRect_insert,
	}
	vector_copy_non_bool! { core::Rect,
		cv_VectorOfRect_data, cv_VectorOfRect_data_mut, cv_VectorOfRect_from_slice,
		cv_VectorOfRect_clone,
	}
	
	extern "C" {
		fn cv_VectorOfRect_input_array(instance: extern_send!(core::Vector<core::Rect>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfRect_output_array(instance: extern_send!(mut core::Vector<core::Rect>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfRect_input_output_array(instance: extern_send!(mut core::Vector<core::Rect>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Rect> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfRect_input_array(self.as_raw_VectorOfRect(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Rect> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfRect_output_array(self.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Rect> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfRect_input_output_array(self.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Rect> }
	
	pub type VectorOfRect2d = core::Vector<core::Rect2d>;
	
	impl core::Vector<core::Rect2d> {
		pub fn as_raw_VectorOfRect2d(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfRect2d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Rect2d,
		cv_VectorOfRect2d_new, cv_VectorOfRect2d_delete,
		cv_VectorOfRect2d_len, cv_VectorOfRect2d_is_empty,
		cv_VectorOfRect2d_capacity, cv_VectorOfRect2d_shrink_to_fit,
		cv_VectorOfRect2d_reserve, cv_VectorOfRect2d_remove,
		cv_VectorOfRect2d_swap, cv_VectorOfRect2d_clear,
		cv_VectorOfRect2d_get, cv_VectorOfRect2d_set,
		cv_VectorOfRect2d_push, cv_VectorOfRect2d_insert,
	}
	vector_copy_non_bool! { core::Rect2d,
		cv_VectorOfRect2d_data, cv_VectorOfRect2d_data_mut, cv_VectorOfRect2d_from_slice,
		cv_VectorOfRect2d_clone,
	}
	
	extern "C" {
		fn cv_VectorOfRect2d_input_array(instance: extern_send!(core::Vector<core::Rect2d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfRect2d_output_array(instance: extern_send!(mut core::Vector<core::Rect2d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfRect2d_input_output_array(instance: extern_send!(mut core::Vector<core::Rect2d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Rect2d> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfRect2d_input_array(self.as_raw_VectorOfRect2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Rect2d> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfRect2d_output_array(self.as_raw_mut_VectorOfRect2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Rect2d> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfRect2d_input_output_array(self.as_raw_mut_VectorOfRect2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Rect2d> }
	
	pub type VectorOfRotatedRect = core::Vector<core::RotatedRect>;
	
	impl core::Vector<core::RotatedRect> {
		pub fn as_raw_VectorOfRotatedRect(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfRotatedRect(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::RotatedRect,
		cv_VectorOfRotatedRect_new, cv_VectorOfRotatedRect_delete,
		cv_VectorOfRotatedRect_len, cv_VectorOfRotatedRect_is_empty,
		cv_VectorOfRotatedRect_capacity, cv_VectorOfRotatedRect_shrink_to_fit,
		cv_VectorOfRotatedRect_reserve, cv_VectorOfRotatedRect_remove,
		cv_VectorOfRotatedRect_swap, cv_VectorOfRotatedRect_clear,
		cv_VectorOfRotatedRect_get, cv_VectorOfRotatedRect_set,
		cv_VectorOfRotatedRect_push, cv_VectorOfRotatedRect_insert,
	}
	vector_non_copy_or_bool! { core::RotatedRect }
	
	pub type VectorOfScalar = core::Vector<core::Scalar>;
	
	impl core::Vector<core::Scalar> {
		pub fn as_raw_VectorOfScalar(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfScalar(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Scalar,
		cv_VectorOfScalar_new, cv_VectorOfScalar_delete,
		cv_VectorOfScalar_len, cv_VectorOfScalar_is_empty,
		cv_VectorOfScalar_capacity, cv_VectorOfScalar_shrink_to_fit,
		cv_VectorOfScalar_reserve, cv_VectorOfScalar_remove,
		cv_VectorOfScalar_swap, cv_VectorOfScalar_clear,
		cv_VectorOfScalar_get, cv_VectorOfScalar_set,
		cv_VectorOfScalar_push, cv_VectorOfScalar_insert,
	}
	vector_copy_non_bool! { core::Scalar,
		cv_VectorOfScalar_data, cv_VectorOfScalar_data_mut, cv_VectorOfScalar_from_slice,
		cv_VectorOfScalar_clone,
	}
	
	extern "C" {
		fn cv_VectorOfScalar_input_array(instance: extern_send!(core::Vector<core::Scalar>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfScalar_output_array(instance: extern_send!(mut core::Vector<core::Scalar>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfScalar_input_output_array(instance: extern_send!(mut core::Vector<core::Scalar>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Scalar> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfScalar_input_array(self.as_raw_VectorOfScalar(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Scalar> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfScalar_output_array(self.as_raw_mut_VectorOfScalar(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Scalar> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfScalar_input_output_array(self.as_raw_mut_VectorOfScalar(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Scalar> }
	
	pub type VectorOfSize = core::Vector<core::Size>;
	
	impl core::Vector<core::Size> {
		pub fn as_raw_VectorOfSize(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfSize(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Size,
		cv_VectorOfSize_new, cv_VectorOfSize_delete,
		cv_VectorOfSize_len, cv_VectorOfSize_is_empty,
		cv_VectorOfSize_capacity, cv_VectorOfSize_shrink_to_fit,
		cv_VectorOfSize_reserve, cv_VectorOfSize_remove,
		cv_VectorOfSize_swap, cv_VectorOfSize_clear,
		cv_VectorOfSize_get, cv_VectorOfSize_set,
		cv_VectorOfSize_push, cv_VectorOfSize_insert,
	}
	vector_copy_non_bool! { core::Size,
		cv_VectorOfSize_data, cv_VectorOfSize_data_mut, cv_VectorOfSize_from_slice,
		cv_VectorOfSize_clone,
	}
	
	extern "C" {
		fn cv_VectorOfSize_input_array(instance: extern_send!(core::Vector<core::Size>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfSize_output_array(instance: extern_send!(mut core::Vector<core::Size>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfSize_input_output_array(instance: extern_send!(mut core::Vector<core::Size>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Size> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfSize_input_array(self.as_raw_VectorOfSize(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Size> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfSize_output_array(self.as_raw_mut_VectorOfSize(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Size> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfSize_input_output_array(self.as_raw_mut_VectorOfSize(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Size> }
	
	pub type VectorOfString = core::Vector<String>;
	
	impl core::Vector<String> {
		pub fn as_raw_VectorOfString(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfString(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { String,
		cv_VectorOfString_new, cv_VectorOfString_delete,
		cv_VectorOfString_len, cv_VectorOfString_is_empty,
		cv_VectorOfString_capacity, cv_VectorOfString_shrink_to_fit,
		cv_VectorOfString_reserve, cv_VectorOfString_remove,
		cv_VectorOfString_swap, cv_VectorOfString_clear,
		cv_VectorOfString_get, cv_VectorOfString_set,
		cv_VectorOfString_push, cv_VectorOfString_insert,
	}
	vector_non_copy_or_bool! { String }
	
	pub type VectorOfTupleOfPoint2i_Point2i = core::Vector<core::Tuple<(core::Point2i, core::Point2i)>>;
	
	impl core::Vector<core::Tuple<(core::Point2i, core::Point2i)>> {
		pub fn as_raw_VectorOfTupleOfPoint2i_Point2i(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfTupleOfPoint2i_Point2i(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Tuple<(core::Point2i, core::Point2i)>,
		cv_VectorOfTupleOfPoint2i_Point2i_new, cv_VectorOfTupleOfPoint2i_Point2i_delete,
		cv_VectorOfTupleOfPoint2i_Point2i_len, cv_VectorOfTupleOfPoint2i_Point2i_is_empty,
		cv_VectorOfTupleOfPoint2i_Point2i_capacity, cv_VectorOfTupleOfPoint2i_Point2i_shrink_to_fit,
		cv_VectorOfTupleOfPoint2i_Point2i_reserve, cv_VectorOfTupleOfPoint2i_Point2i_remove,
		cv_VectorOfTupleOfPoint2i_Point2i_swap, cv_VectorOfTupleOfPoint2i_Point2i_clear,
		cv_VectorOfTupleOfPoint2i_Point2i_get, cv_VectorOfTupleOfPoint2i_Point2i_set,
		cv_VectorOfTupleOfPoint2i_Point2i_push, cv_VectorOfTupleOfPoint2i_Point2i_insert,
	}
	vector_non_copy_or_bool! { core::Tuple<(core::Point2i, core::Point2i)> }
	
	pub type VectorOfTupleOfUMat_u8 = core::Vector<core::Tuple<(core::UMat, u8)>>;
	
	impl core::Vector<core::Tuple<(core::UMat, u8)>> {
		pub fn as_raw_VectorOfTupleOfUMat_u8(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfTupleOfUMat_u8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Tuple<(core::UMat, u8)>,
		cv_VectorOfTupleOfUMat_u8_new, cv_VectorOfTupleOfUMat_u8_delete,
		cv_VectorOfTupleOfUMat_u8_len, cv_VectorOfTupleOfUMat_u8_is_empty,
		cv_VectorOfTupleOfUMat_u8_capacity, cv_VectorOfTupleOfUMat_u8_shrink_to_fit,
		cv_VectorOfTupleOfUMat_u8_reserve, cv_VectorOfTupleOfUMat_u8_remove,
		cv_VectorOfTupleOfUMat_u8_swap, cv_VectorOfTupleOfUMat_u8_clear,
		cv_VectorOfTupleOfUMat_u8_get, cv_VectorOfTupleOfUMat_u8_set,
		cv_VectorOfTupleOfUMat_u8_push, cv_VectorOfTupleOfUMat_u8_insert,
	}
	vector_non_copy_or_bool! { core::Tuple<(core::UMat, u8)> }
	
	pub type VectorOfTupleOfi32_f64 = core::Vector<core::Tuple<(i32, f64)>>;
	
	impl core::Vector<core::Tuple<(i32, f64)>> {
		pub fn as_raw_VectorOfTupleOfi32_f64(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfTupleOfi32_f64(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Tuple<(i32, f64)>,
		cv_VectorOfTupleOfi32_f64_new, cv_VectorOfTupleOfi32_f64_delete,
		cv_VectorOfTupleOfi32_f64_len, cv_VectorOfTupleOfi32_f64_is_empty,
		cv_VectorOfTupleOfi32_f64_capacity, cv_VectorOfTupleOfi32_f64_shrink_to_fit,
		cv_VectorOfTupleOfi32_f64_reserve, cv_VectorOfTupleOfi32_f64_remove,
		cv_VectorOfTupleOfi32_f64_swap, cv_VectorOfTupleOfi32_f64_clear,
		cv_VectorOfTupleOfi32_f64_get, cv_VectorOfTupleOfi32_f64_set,
		cv_VectorOfTupleOfi32_f64_push, cv_VectorOfTupleOfi32_f64_insert,
	}
	vector_non_copy_or_bool! { core::Tuple<(i32, f64)> }
	
	pub type VectorOfUMat = core::Vector<core::UMat>;
	
	impl core::Vector<core::UMat> {
		pub fn as_raw_VectorOfUMat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfUMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::UMat,
		cv_VectorOfUMat_new, cv_VectorOfUMat_delete,
		cv_VectorOfUMat_len, cv_VectorOfUMat_is_empty,
		cv_VectorOfUMat_capacity, cv_VectorOfUMat_shrink_to_fit,
		cv_VectorOfUMat_reserve, cv_VectorOfUMat_remove,
		cv_VectorOfUMat_swap, cv_VectorOfUMat_clear,
		cv_VectorOfUMat_get, cv_VectorOfUMat_set,
		cv_VectorOfUMat_push, cv_VectorOfUMat_insert,
	}
	vector_non_copy_or_bool! { clone core::UMat }
	
	pub type VectorOfVec2d = core::Vector<core::Vec2d>;
	
	impl core::Vector<core::Vec2d> {
		pub fn as_raw_VectorOfVec2d(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec2d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec2d,
		cv_VectorOfVec2d_new, cv_VectorOfVec2d_delete,
		cv_VectorOfVec2d_len, cv_VectorOfVec2d_is_empty,
		cv_VectorOfVec2d_capacity, cv_VectorOfVec2d_shrink_to_fit,
		cv_VectorOfVec2d_reserve, cv_VectorOfVec2d_remove,
		cv_VectorOfVec2d_swap, cv_VectorOfVec2d_clear,
		cv_VectorOfVec2d_get, cv_VectorOfVec2d_set,
		cv_VectorOfVec2d_push, cv_VectorOfVec2d_insert,
	}
	vector_copy_non_bool! { core::Vec2d,
		cv_VectorOfVec2d_data, cv_VectorOfVec2d_data_mut, cv_VectorOfVec2d_from_slice,
		cv_VectorOfVec2d_clone,
	}
	
	extern "C" {
		fn cv_VectorOfVec2d_input_array(instance: extern_send!(core::Vector<core::Vec2d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVec2d_output_array(instance: extern_send!(mut core::Vector<core::Vec2d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVec2d_input_output_array(instance: extern_send!(mut core::Vector<core::Vec2d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vec2d> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2d_input_array(self.as_raw_VectorOfVec2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vec2d> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2d_output_array(self.as_raw_mut_VectorOfVec2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vec2d> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2d_input_output_array(self.as_raw_mut_VectorOfVec2d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vec2d> }
	
	pub type VectorOfVec2f = core::Vector<core::Vec2f>;
	
	impl core::Vector<core::Vec2f> {
		pub fn as_raw_VectorOfVec2f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec2f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec2f,
		cv_VectorOfVec2f_new, cv_VectorOfVec2f_delete,
		cv_VectorOfVec2f_len, cv_VectorOfVec2f_is_empty,
		cv_VectorOfVec2f_capacity, cv_VectorOfVec2f_shrink_to_fit,
		cv_VectorOfVec2f_reserve, cv_VectorOfVec2f_remove,
		cv_VectorOfVec2f_swap, cv_VectorOfVec2f_clear,
		cv_VectorOfVec2f_get, cv_VectorOfVec2f_set,
		cv_VectorOfVec2f_push, cv_VectorOfVec2f_insert,
	}
	vector_copy_non_bool! { core::Vec2f,
		cv_VectorOfVec2f_data, cv_VectorOfVec2f_data_mut, cv_VectorOfVec2f_from_slice,
		cv_VectorOfVec2f_clone,
	}
	
	extern "C" {
		fn cv_VectorOfVec2f_input_array(instance: extern_send!(core::Vector<core::Vec2f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVec2f_output_array(instance: extern_send!(mut core::Vector<core::Vec2f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVec2f_input_output_array(instance: extern_send!(mut core::Vector<core::Vec2f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vec2f> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2f_input_array(self.as_raw_VectorOfVec2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vec2f> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2f_output_array(self.as_raw_mut_VectorOfVec2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vec2f> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2f_input_output_array(self.as_raw_mut_VectorOfVec2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vec2f> }
	
	pub type VectorOfVec2i = core::Vector<core::Vec2i>;
	
	impl core::Vector<core::Vec2i> {
		pub fn as_raw_VectorOfVec2i(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec2i(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec2i,
		cv_VectorOfVec2i_new, cv_VectorOfVec2i_delete,
		cv_VectorOfVec2i_len, cv_VectorOfVec2i_is_empty,
		cv_VectorOfVec2i_capacity, cv_VectorOfVec2i_shrink_to_fit,
		cv_VectorOfVec2i_reserve, cv_VectorOfVec2i_remove,
		cv_VectorOfVec2i_swap, cv_VectorOfVec2i_clear,
		cv_VectorOfVec2i_get, cv_VectorOfVec2i_set,
		cv_VectorOfVec2i_push, cv_VectorOfVec2i_insert,
	}
	vector_copy_non_bool! { core::Vec2i,
		cv_VectorOfVec2i_data, cv_VectorOfVec2i_data_mut, cv_VectorOfVec2i_from_slice,
		cv_VectorOfVec2i_clone,
	}
	
	extern "C" {
		fn cv_VectorOfVec2i_input_array(instance: extern_send!(core::Vector<core::Vec2i>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVec2i_output_array(instance: extern_send!(mut core::Vector<core::Vec2i>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVec2i_input_output_array(instance: extern_send!(mut core::Vector<core::Vec2i>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vec2i> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2i_input_array(self.as_raw_VectorOfVec2i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vec2i> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2i_output_array(self.as_raw_mut_VectorOfVec2i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vec2i> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec2i_input_output_array(self.as_raw_mut_VectorOfVec2i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vec2i> }
	
	pub type VectorOfVec3d = core::Vector<core::Vec3d>;
	
	impl core::Vector<core::Vec3d> {
		pub fn as_raw_VectorOfVec3d(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec3d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec3d,
		cv_VectorOfVec3d_new, cv_VectorOfVec3d_delete,
		cv_VectorOfVec3d_len, cv_VectorOfVec3d_is_empty,
		cv_VectorOfVec3d_capacity, cv_VectorOfVec3d_shrink_to_fit,
		cv_VectorOfVec3d_reserve, cv_VectorOfVec3d_remove,
		cv_VectorOfVec3d_swap, cv_VectorOfVec3d_clear,
		cv_VectorOfVec3d_get, cv_VectorOfVec3d_set,
		cv_VectorOfVec3d_push, cv_VectorOfVec3d_insert,
	}
	vector_copy_non_bool! { core::Vec3d,
		cv_VectorOfVec3d_data, cv_VectorOfVec3d_data_mut, cv_VectorOfVec3d_from_slice,
		cv_VectorOfVec3d_clone,
	}
	
	extern "C" {
		fn cv_VectorOfVec3d_input_array(instance: extern_send!(core::Vector<core::Vec3d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVec3d_output_array(instance: extern_send!(mut core::Vector<core::Vec3d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVec3d_input_output_array(instance: extern_send!(mut core::Vector<core::Vec3d>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vec3d> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3d_input_array(self.as_raw_VectorOfVec3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vec3d> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3d_output_array(self.as_raw_mut_VectorOfVec3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vec3d> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3d_input_output_array(self.as_raw_mut_VectorOfVec3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vec3d> }
	
	pub type VectorOfVec3f = core::Vector<core::Vec3f>;
	
	impl core::Vector<core::Vec3f> {
		pub fn as_raw_VectorOfVec3f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec3f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec3f,
		cv_VectorOfVec3f_new, cv_VectorOfVec3f_delete,
		cv_VectorOfVec3f_len, cv_VectorOfVec3f_is_empty,
		cv_VectorOfVec3f_capacity, cv_VectorOfVec3f_shrink_to_fit,
		cv_VectorOfVec3f_reserve, cv_VectorOfVec3f_remove,
		cv_VectorOfVec3f_swap, cv_VectorOfVec3f_clear,
		cv_VectorOfVec3f_get, cv_VectorOfVec3f_set,
		cv_VectorOfVec3f_push, cv_VectorOfVec3f_insert,
	}
	vector_copy_non_bool! { core::Vec3f,
		cv_VectorOfVec3f_data, cv_VectorOfVec3f_data_mut, cv_VectorOfVec3f_from_slice,
		cv_VectorOfVec3f_clone,
	}
	
	extern "C" {
		fn cv_VectorOfVec3f_input_array(instance: extern_send!(core::Vector<core::Vec3f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVec3f_output_array(instance: extern_send!(mut core::Vector<core::Vec3f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVec3f_input_output_array(instance: extern_send!(mut core::Vector<core::Vec3f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vec3f> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3f_input_array(self.as_raw_VectorOfVec3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vec3f> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3f_output_array(self.as_raw_mut_VectorOfVec3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vec3f> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3f_input_output_array(self.as_raw_mut_VectorOfVec3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vec3f> }
	
	pub type VectorOfVec3i = core::Vector<core::Vec3i>;
	
	impl core::Vector<core::Vec3i> {
		pub fn as_raw_VectorOfVec3i(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec3i(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec3i,
		cv_VectorOfVec3i_new, cv_VectorOfVec3i_delete,
		cv_VectorOfVec3i_len, cv_VectorOfVec3i_is_empty,
		cv_VectorOfVec3i_capacity, cv_VectorOfVec3i_shrink_to_fit,
		cv_VectorOfVec3i_reserve, cv_VectorOfVec3i_remove,
		cv_VectorOfVec3i_swap, cv_VectorOfVec3i_clear,
		cv_VectorOfVec3i_get, cv_VectorOfVec3i_set,
		cv_VectorOfVec3i_push, cv_VectorOfVec3i_insert,
	}
	vector_copy_non_bool! { core::Vec3i,
		cv_VectorOfVec3i_data, cv_VectorOfVec3i_data_mut, cv_VectorOfVec3i_from_slice,
		cv_VectorOfVec3i_clone,
	}
	
	extern "C" {
		fn cv_VectorOfVec3i_input_array(instance: extern_send!(core::Vector<core::Vec3i>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVec3i_output_array(instance: extern_send!(mut core::Vector<core::Vec3i>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVec3i_input_output_array(instance: extern_send!(mut core::Vector<core::Vec3i>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vec3i> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3i_input_array(self.as_raw_VectorOfVec3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vec3i> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3i_output_array(self.as_raw_mut_VectorOfVec3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vec3i> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec3i_input_output_array(self.as_raw_mut_VectorOfVec3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vec3i> }
	
	pub type VectorOfVec4f = core::Vector<core::Vec4f>;
	
	impl core::Vector<core::Vec4f> {
		pub fn as_raw_VectorOfVec4f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec4f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec4f,
		cv_VectorOfVec4f_new, cv_VectorOfVec4f_delete,
		cv_VectorOfVec4f_len, cv_VectorOfVec4f_is_empty,
		cv_VectorOfVec4f_capacity, cv_VectorOfVec4f_shrink_to_fit,
		cv_VectorOfVec4f_reserve, cv_VectorOfVec4f_remove,
		cv_VectorOfVec4f_swap, cv_VectorOfVec4f_clear,
		cv_VectorOfVec4f_get, cv_VectorOfVec4f_set,
		cv_VectorOfVec4f_push, cv_VectorOfVec4f_insert,
	}
	vector_copy_non_bool! { core::Vec4f,
		cv_VectorOfVec4f_data, cv_VectorOfVec4f_data_mut, cv_VectorOfVec4f_from_slice,
		cv_VectorOfVec4f_clone,
	}
	
	extern "C" {
		fn cv_VectorOfVec4f_input_array(instance: extern_send!(core::Vector<core::Vec4f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVec4f_output_array(instance: extern_send!(mut core::Vector<core::Vec4f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVec4f_input_output_array(instance: extern_send!(mut core::Vector<core::Vec4f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vec4f> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec4f_input_array(self.as_raw_VectorOfVec4f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vec4f> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec4f_output_array(self.as_raw_mut_VectorOfVec4f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vec4f> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec4f_input_output_array(self.as_raw_mut_VectorOfVec4f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vec4f> }
	
	pub type VectorOfVec4i = core::Vector<core::Vec4i>;
	
	impl core::Vector<core::Vec4i> {
		pub fn as_raw_VectorOfVec4i(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec4i(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec4i,
		cv_VectorOfVec4i_new, cv_VectorOfVec4i_delete,
		cv_VectorOfVec4i_len, cv_VectorOfVec4i_is_empty,
		cv_VectorOfVec4i_capacity, cv_VectorOfVec4i_shrink_to_fit,
		cv_VectorOfVec4i_reserve, cv_VectorOfVec4i_remove,
		cv_VectorOfVec4i_swap, cv_VectorOfVec4i_clear,
		cv_VectorOfVec4i_get, cv_VectorOfVec4i_set,
		cv_VectorOfVec4i_push, cv_VectorOfVec4i_insert,
	}
	vector_copy_non_bool! { core::Vec4i,
		cv_VectorOfVec4i_data, cv_VectorOfVec4i_data_mut, cv_VectorOfVec4i_from_slice,
		cv_VectorOfVec4i_clone,
	}
	
	extern "C" {
		fn cv_VectorOfVec4i_input_array(instance: extern_send!(core::Vector<core::Vec4i>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVec4i_output_array(instance: extern_send!(mut core::Vector<core::Vec4i>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVec4i_input_output_array(instance: extern_send!(mut core::Vector<core::Vec4i>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vec4i> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec4i_input_array(self.as_raw_VectorOfVec4i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vec4i> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec4i_output_array(self.as_raw_mut_VectorOfVec4i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vec4i> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec4i_input_output_array(self.as_raw_mut_VectorOfVec4i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vec4i> }
	
	pub type VectorOfVec6f = core::Vector<core::Vec6f>;
	
	impl core::Vector<core::Vec6f> {
		pub fn as_raw_VectorOfVec6f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec6f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vec6f,
		cv_VectorOfVec6f_new, cv_VectorOfVec6f_delete,
		cv_VectorOfVec6f_len, cv_VectorOfVec6f_is_empty,
		cv_VectorOfVec6f_capacity, cv_VectorOfVec6f_shrink_to_fit,
		cv_VectorOfVec6f_reserve, cv_VectorOfVec6f_remove,
		cv_VectorOfVec6f_swap, cv_VectorOfVec6f_clear,
		cv_VectorOfVec6f_get, cv_VectorOfVec6f_set,
		cv_VectorOfVec6f_push, cv_VectorOfVec6f_insert,
	}
	vector_copy_non_bool! { core::Vec6f,
		cv_VectorOfVec6f_data, cv_VectorOfVec6f_data_mut, cv_VectorOfVec6f_from_slice,
		cv_VectorOfVec6f_clone,
	}
	
	extern "C" {
		fn cv_VectorOfVec6f_input_array(instance: extern_send!(core::Vector<core::Vec6f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVec6f_output_array(instance: extern_send!(mut core::Vector<core::Vec6f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVec6f_input_output_array(instance: extern_send!(mut core::Vector<core::Vec6f>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vec6f> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec6f_input_array(self.as_raw_VectorOfVec6f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vec6f> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec6f_output_array(self.as_raw_mut_VectorOfVec6f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vec6f> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVec6f_input_output_array(self.as_raw_mut_VectorOfVec6f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vec6f> }
	
	pub type VectorOfVectorOfDMatch = core::Vector<core::Vector<core::DMatch>>;
	
	impl core::Vector<core::Vector<core::DMatch>> {
		pub fn as_raw_VectorOfVectorOfDMatch(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfDMatch(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::DMatch>,
		cv_VectorOfVectorOfDMatch_new, cv_VectorOfVectorOfDMatch_delete,
		cv_VectorOfVectorOfDMatch_len, cv_VectorOfVectorOfDMatch_is_empty,
		cv_VectorOfVectorOfDMatch_capacity, cv_VectorOfVectorOfDMatch_shrink_to_fit,
		cv_VectorOfVectorOfDMatch_reserve, cv_VectorOfVectorOfDMatch_remove,
		cv_VectorOfVectorOfDMatch_swap, cv_VectorOfVectorOfDMatch_clear,
		cv_VectorOfVectorOfDMatch_get, cv_VectorOfVectorOfDMatch_set,
		cv_VectorOfVectorOfDMatch_push, cv_VectorOfVectorOfDMatch_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::DMatch> }
	
	pub type VectorOfVectorOfKeyPoint = core::Vector<core::Vector<core::KeyPoint>>;
	
	impl core::Vector<core::Vector<core::KeyPoint>> {
		pub fn as_raw_VectorOfVectorOfKeyPoint(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfKeyPoint(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::KeyPoint>,
		cv_VectorOfVectorOfKeyPoint_new, cv_VectorOfVectorOfKeyPoint_delete,
		cv_VectorOfVectorOfKeyPoint_len, cv_VectorOfVectorOfKeyPoint_is_empty,
		cv_VectorOfVectorOfKeyPoint_capacity, cv_VectorOfVectorOfKeyPoint_shrink_to_fit,
		cv_VectorOfVectorOfKeyPoint_reserve, cv_VectorOfVectorOfKeyPoint_remove,
		cv_VectorOfVectorOfKeyPoint_swap, cv_VectorOfVectorOfKeyPoint_clear,
		cv_VectorOfVectorOfKeyPoint_get, cv_VectorOfVectorOfKeyPoint_set,
		cv_VectorOfVectorOfKeyPoint_push, cv_VectorOfVectorOfKeyPoint_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::KeyPoint> }
	
	pub type VectorOfVectorOfMat = core::Vector<core::Vector<core::Mat>>;
	
	impl core::Vector<core::Vector<core::Mat>> {
		pub fn as_raw_VectorOfVectorOfMat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Mat>,
		cv_VectorOfVectorOfMat_new, cv_VectorOfVectorOfMat_delete,
		cv_VectorOfVectorOfMat_len, cv_VectorOfVectorOfMat_is_empty,
		cv_VectorOfVectorOfMat_capacity, cv_VectorOfVectorOfMat_shrink_to_fit,
		cv_VectorOfVectorOfMat_reserve, cv_VectorOfVectorOfMat_remove,
		cv_VectorOfVectorOfMat_swap, cv_VectorOfVectorOfMat_clear,
		cv_VectorOfVectorOfMat_get, cv_VectorOfVectorOfMat_set,
		cv_VectorOfVectorOfMat_push, cv_VectorOfVectorOfMat_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Mat> }
	
	pub type VectorOfVectorOfPoint = core::Vector<core::Vector<core::Point>>;
	
	impl core::Vector<core::Vector<core::Point>> {
		pub fn as_raw_VectorOfVectorOfPoint(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Point>,
		cv_VectorOfVectorOfPoint_new, cv_VectorOfVectorOfPoint_delete,
		cv_VectorOfVectorOfPoint_len, cv_VectorOfVectorOfPoint_is_empty,
		cv_VectorOfVectorOfPoint_capacity, cv_VectorOfVectorOfPoint_shrink_to_fit,
		cv_VectorOfVectorOfPoint_reserve, cv_VectorOfVectorOfPoint_remove,
		cv_VectorOfVectorOfPoint_swap, cv_VectorOfVectorOfPoint_clear,
		cv_VectorOfVectorOfPoint_get, cv_VectorOfVectorOfPoint_set,
		cv_VectorOfVectorOfPoint_push, cv_VectorOfVectorOfPoint_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Point> }
	
	extern "C" {
		fn cv_VectorOfVectorOfPoint_input_array(instance: extern_send!(core::Vector<core::Vector<core::Point>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOfPoint_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Point>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOfPoint_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Point>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<core::Point>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint_input_array(self.as_raw_VectorOfVectorOfPoint(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<core::Point>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint_output_array(self.as_raw_mut_VectorOfVectorOfPoint(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<core::Point>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<core::Point>> }
	
	pub type VectorOfVectorOfPoint2f = core::Vector<core::Vector<core::Point2f>>;
	
	impl core::Vector<core::Vector<core::Point2f>> {
		pub fn as_raw_VectorOfVectorOfPoint2f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint2f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Point2f>,
		cv_VectorOfVectorOfPoint2f_new, cv_VectorOfVectorOfPoint2f_delete,
		cv_VectorOfVectorOfPoint2f_len, cv_VectorOfVectorOfPoint2f_is_empty,
		cv_VectorOfVectorOfPoint2f_capacity, cv_VectorOfVectorOfPoint2f_shrink_to_fit,
		cv_VectorOfVectorOfPoint2f_reserve, cv_VectorOfVectorOfPoint2f_remove,
		cv_VectorOfVectorOfPoint2f_swap, cv_VectorOfVectorOfPoint2f_clear,
		cv_VectorOfVectorOfPoint2f_get, cv_VectorOfVectorOfPoint2f_set,
		cv_VectorOfVectorOfPoint2f_push, cv_VectorOfVectorOfPoint2f_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Point2f> }
	
	extern "C" {
		fn cv_VectorOfVectorOfPoint2f_input_array(instance: extern_send!(core::Vector<core::Vector<core::Point2f>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOfPoint2f_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Point2f>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOfPoint2f_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Point2f>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<core::Point2f>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint2f_input_array(self.as_raw_VectorOfVectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<core::Point2f>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint2f_output_array(self.as_raw_mut_VectorOfVectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<core::Point2f>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint2f_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<core::Point2f>> }
	
	pub type VectorOfVectorOfPoint3d = core::Vector<core::Vector<core::Point3d>>;
	
	impl core::Vector<core::Vector<core::Point3d>> {
		pub fn as_raw_VectorOfVectorOfPoint3d(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint3d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Point3d>,
		cv_VectorOfVectorOfPoint3d_new, cv_VectorOfVectorOfPoint3d_delete,
		cv_VectorOfVectorOfPoint3d_len, cv_VectorOfVectorOfPoint3d_is_empty,
		cv_VectorOfVectorOfPoint3d_capacity, cv_VectorOfVectorOfPoint3d_shrink_to_fit,
		cv_VectorOfVectorOfPoint3d_reserve, cv_VectorOfVectorOfPoint3d_remove,
		cv_VectorOfVectorOfPoint3d_swap, cv_VectorOfVectorOfPoint3d_clear,
		cv_VectorOfVectorOfPoint3d_get, cv_VectorOfVectorOfPoint3d_set,
		cv_VectorOfVectorOfPoint3d_push, cv_VectorOfVectorOfPoint3d_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Point3d> }
	
	extern "C" {
		fn cv_VectorOfVectorOfPoint3d_input_array(instance: extern_send!(core::Vector<core::Vector<core::Point3d>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOfPoint3d_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Point3d>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOfPoint3d_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Point3d>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<core::Point3d>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3d_input_array(self.as_raw_VectorOfVectorOfPoint3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<core::Point3d>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3d_output_array(self.as_raw_mut_VectorOfVectorOfPoint3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<core::Point3d>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3d_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint3d(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<core::Point3d>> }
	
	pub type VectorOfVectorOfPoint3f = core::Vector<core::Vector<core::Point3f>>;
	
	impl core::Vector<core::Vector<core::Point3f>> {
		pub fn as_raw_VectorOfVectorOfPoint3f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint3f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Point3f>,
		cv_VectorOfVectorOfPoint3f_new, cv_VectorOfVectorOfPoint3f_delete,
		cv_VectorOfVectorOfPoint3f_len, cv_VectorOfVectorOfPoint3f_is_empty,
		cv_VectorOfVectorOfPoint3f_capacity, cv_VectorOfVectorOfPoint3f_shrink_to_fit,
		cv_VectorOfVectorOfPoint3f_reserve, cv_VectorOfVectorOfPoint3f_remove,
		cv_VectorOfVectorOfPoint3f_swap, cv_VectorOfVectorOfPoint3f_clear,
		cv_VectorOfVectorOfPoint3f_get, cv_VectorOfVectorOfPoint3f_set,
		cv_VectorOfVectorOfPoint3f_push, cv_VectorOfVectorOfPoint3f_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Point3f> }
	
	extern "C" {
		fn cv_VectorOfVectorOfPoint3f_input_array(instance: extern_send!(core::Vector<core::Vector<core::Point3f>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOfPoint3f_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Point3f>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOfPoint3f_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Point3f>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<core::Point3f>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3f_input_array(self.as_raw_VectorOfVectorOfPoint3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<core::Point3f>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3f_output_array(self.as_raw_mut_VectorOfVectorOfPoint3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<core::Point3f>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3f_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<core::Point3f>> }
	
	pub type VectorOfVectorOfPoint3i = core::Vector<core::Vector<core::Point3i>>;
	
	impl core::Vector<core::Vector<core::Point3i>> {
		pub fn as_raw_VectorOfVectorOfPoint3i(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint3i(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Point3i>,
		cv_VectorOfVectorOfPoint3i_new, cv_VectorOfVectorOfPoint3i_delete,
		cv_VectorOfVectorOfPoint3i_len, cv_VectorOfVectorOfPoint3i_is_empty,
		cv_VectorOfVectorOfPoint3i_capacity, cv_VectorOfVectorOfPoint3i_shrink_to_fit,
		cv_VectorOfVectorOfPoint3i_reserve, cv_VectorOfVectorOfPoint3i_remove,
		cv_VectorOfVectorOfPoint3i_swap, cv_VectorOfVectorOfPoint3i_clear,
		cv_VectorOfVectorOfPoint3i_get, cv_VectorOfVectorOfPoint3i_set,
		cv_VectorOfVectorOfPoint3i_push, cv_VectorOfVectorOfPoint3i_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Point3i> }
	
	extern "C" {
		fn cv_VectorOfVectorOfPoint3i_input_array(instance: extern_send!(core::Vector<core::Vector<core::Point3i>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOfPoint3i_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Point3i>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOfPoint3i_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Point3i>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<core::Point3i>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3i_input_array(self.as_raw_VectorOfVectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<core::Point3i>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3i_output_array(self.as_raw_mut_VectorOfVectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<core::Point3i>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfPoint3i_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<core::Point3i>> }
	
	pub type VectorOfVectorOfRange = core::Vector<core::Vector<core::Range>>;
	
	impl core::Vector<core::Vector<core::Range>> {
		pub fn as_raw_VectorOfVectorOfRange(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfRange(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Range>,
		cv_VectorOfVectorOfRange_new, cv_VectorOfVectorOfRange_delete,
		cv_VectorOfVectorOfRange_len, cv_VectorOfVectorOfRange_is_empty,
		cv_VectorOfVectorOfRange_capacity, cv_VectorOfVectorOfRange_shrink_to_fit,
		cv_VectorOfVectorOfRange_reserve, cv_VectorOfVectorOfRange_remove,
		cv_VectorOfVectorOfRange_swap, cv_VectorOfVectorOfRange_clear,
		cv_VectorOfVectorOfRange_get, cv_VectorOfVectorOfRange_set,
		cv_VectorOfVectorOfRange_push, cv_VectorOfVectorOfRange_insert,
	}
	vector_non_copy_or_bool! { core::Vector<core::Range> }
	
	pub type VectorOfVectorOfVec2i = core::Vector<core::Vector<core::Vec2i>>;
	
	impl core::Vector<core::Vector<core::Vec2i>> {
		pub fn as_raw_VectorOfVectorOfVec2i(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfVec2i(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Vec2i>,
		cv_VectorOfVectorOfVec2i_new, cv_VectorOfVectorOfVec2i_delete,
		cv_VectorOfVectorOfVec2i_len, cv_VectorOfVectorOfVec2i_is_empty,
		cv_VectorOfVectorOfVec2i_capacity, cv_VectorOfVectorOfVec2i_shrink_to_fit,
		cv_VectorOfVectorOfVec2i_reserve, cv_VectorOfVectorOfVec2i_remove,
		cv_VectorOfVectorOfVec2i_swap, cv_VectorOfVectorOfVec2i_clear,
		cv_VectorOfVectorOfVec2i_get, cv_VectorOfVectorOfVec2i_set,
		cv_VectorOfVectorOfVec2i_push, cv_VectorOfVectorOfVec2i_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Vec2i> }
	
	extern "C" {
		fn cv_VectorOfVectorOfVec2i_input_array(instance: extern_send!(core::Vector<core::Vector<core::Vec2i>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOfVec2i_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Vec2i>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOfVec2i_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Vec2i>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<core::Vec2i>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfVec2i_input_array(self.as_raw_VectorOfVectorOfVec2i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<core::Vec2i>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfVec2i_output_array(self.as_raw_mut_VectorOfVectorOfVec2i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<core::Vec2i>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfVec2i_input_output_array(self.as_raw_mut_VectorOfVectorOfVec2i(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<core::Vec2i>> }
	
	pub type VectorOfVectorOfVec3f = core::Vector<core::Vector<core::Vec3f>>;
	
	impl core::Vector<core::Vector<core::Vec3f>> {
		pub fn as_raw_VectorOfVectorOfVec3f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfVec3f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<core::Vec3f>,
		cv_VectorOfVectorOfVec3f_new, cv_VectorOfVectorOfVec3f_delete,
		cv_VectorOfVectorOfVec3f_len, cv_VectorOfVectorOfVec3f_is_empty,
		cv_VectorOfVectorOfVec3f_capacity, cv_VectorOfVectorOfVec3f_shrink_to_fit,
		cv_VectorOfVectorOfVec3f_reserve, cv_VectorOfVectorOfVec3f_remove,
		cv_VectorOfVectorOfVec3f_swap, cv_VectorOfVectorOfVec3f_clear,
		cv_VectorOfVectorOfVec3f_get, cv_VectorOfVectorOfVec3f_set,
		cv_VectorOfVectorOfVec3f_push, cv_VectorOfVectorOfVec3f_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<core::Vec3f> }
	
	extern "C" {
		fn cv_VectorOfVectorOfVec3f_input_array(instance: extern_send!(core::Vector<core::Vector<core::Vec3f>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOfVec3f_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Vec3f>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOfVec3f_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<core::Vec3f>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<core::Vec3f>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfVec3f_input_array(self.as_raw_VectorOfVectorOfVec3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<core::Vec3f>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfVec3f_output_array(self.as_raw_mut_VectorOfVectorOfVec3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<core::Vec3f>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfVec3f_input_output_array(self.as_raw_mut_VectorOfVectorOfVec3f(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<core::Vec3f>> }
	
	pub type VectorOfVectorOff32 = core::Vector<core::Vector<f32>>;
	
	impl core::Vector<core::Vector<f32>> {
		pub fn as_raw_VectorOfVectorOff32(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOff32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<f32>,
		cv_VectorOfVectorOff32_new, cv_VectorOfVectorOff32_delete,
		cv_VectorOfVectorOff32_len, cv_VectorOfVectorOff32_is_empty,
		cv_VectorOfVectorOff32_capacity, cv_VectorOfVectorOff32_shrink_to_fit,
		cv_VectorOfVectorOff32_reserve, cv_VectorOfVectorOff32_remove,
		cv_VectorOfVectorOff32_swap, cv_VectorOfVectorOff32_clear,
		cv_VectorOfVectorOff32_get, cv_VectorOfVectorOff32_set,
		cv_VectorOfVectorOff32_push, cv_VectorOfVectorOff32_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<f32> }
	
	extern "C" {
		fn cv_VectorOfVectorOff32_input_array(instance: extern_send!(core::Vector<core::Vector<f32>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOff32_output_array(instance: extern_send!(mut core::Vector<core::Vector<f32>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOff32_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<f32>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<f32>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOff32_input_array(self.as_raw_VectorOfVectorOff32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<f32>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOff32_output_array(self.as_raw_mut_VectorOfVectorOff32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<f32>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOff32_input_output_array(self.as_raw_mut_VectorOfVectorOff32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<f32>> }
	
	pub type VectorOfVectorOff64 = core::Vector<core::Vector<f64>>;
	
	impl core::Vector<core::Vector<f64>> {
		pub fn as_raw_VectorOfVectorOff64(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOff64(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<f64>,
		cv_VectorOfVectorOff64_new, cv_VectorOfVectorOff64_delete,
		cv_VectorOfVectorOff64_len, cv_VectorOfVectorOff64_is_empty,
		cv_VectorOfVectorOff64_capacity, cv_VectorOfVectorOff64_shrink_to_fit,
		cv_VectorOfVectorOff64_reserve, cv_VectorOfVectorOff64_remove,
		cv_VectorOfVectorOff64_swap, cv_VectorOfVectorOff64_clear,
		cv_VectorOfVectorOff64_get, cv_VectorOfVectorOff64_set,
		cv_VectorOfVectorOff64_push, cv_VectorOfVectorOff64_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<f64> }
	
	extern "C" {
		fn cv_VectorOfVectorOff64_input_array(instance: extern_send!(core::Vector<core::Vector<f64>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOff64_output_array(instance: extern_send!(mut core::Vector<core::Vector<f64>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOff64_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<f64>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<f64>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOff64_input_array(self.as_raw_VectorOfVectorOff64(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<f64>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOff64_output_array(self.as_raw_mut_VectorOfVectorOff64(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<f64>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOff64_input_output_array(self.as_raw_mut_VectorOfVectorOff64(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<f64>> }
	
	pub type VectorOfVectorOfi32 = core::Vector<core::Vector<i32>>;
	
	impl core::Vector<core::Vector<i32>> {
		pub fn as_raw_VectorOfVectorOfi32(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfi32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<i32>,
		cv_VectorOfVectorOfi32_new, cv_VectorOfVectorOfi32_delete,
		cv_VectorOfVectorOfi32_len, cv_VectorOfVectorOfi32_is_empty,
		cv_VectorOfVectorOfi32_capacity, cv_VectorOfVectorOfi32_shrink_to_fit,
		cv_VectorOfVectorOfi32_reserve, cv_VectorOfVectorOfi32_remove,
		cv_VectorOfVectorOfi32_swap, cv_VectorOfVectorOfi32_clear,
		cv_VectorOfVectorOfi32_get, cv_VectorOfVectorOfi32_set,
		cv_VectorOfVectorOfi32_push, cv_VectorOfVectorOfi32_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<i32> }
	
	extern "C" {
		fn cv_VectorOfVectorOfi32_input_array(instance: extern_send!(core::Vector<core::Vector<i32>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOfi32_output_array(instance: extern_send!(mut core::Vector<core::Vector<i32>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOfi32_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<i32>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<i32>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfi32_input_array(self.as_raw_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<i32>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfi32_output_array(self.as_raw_mut_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<i32>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfi32_input_output_array(self.as_raw_mut_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<i32>> }
	
	pub type VectorOfVectorOfi8 = core::Vector<core::Vector<i8>>;
	
	impl core::Vector<core::Vector<i8>> {
		pub fn as_raw_VectorOfVectorOfi8(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfi8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<i8>,
		cv_VectorOfVectorOfi8_new, cv_VectorOfVectorOfi8_delete,
		cv_VectorOfVectorOfi8_len, cv_VectorOfVectorOfi8_is_empty,
		cv_VectorOfVectorOfi8_capacity, cv_VectorOfVectorOfi8_shrink_to_fit,
		cv_VectorOfVectorOfi8_reserve, cv_VectorOfVectorOfi8_remove,
		cv_VectorOfVectorOfi8_swap, cv_VectorOfVectorOfi8_clear,
		cv_VectorOfVectorOfi8_get, cv_VectorOfVectorOfi8_set,
		cv_VectorOfVectorOfi8_push, cv_VectorOfVectorOfi8_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<i8> }
	
	extern "C" {
		fn cv_VectorOfVectorOfi8_input_array(instance: extern_send!(core::Vector<core::Vector<i8>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOfi8_output_array(instance: extern_send!(mut core::Vector<core::Vector<i8>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOfi8_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<i8>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<i8>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfi8_input_array(self.as_raw_VectorOfVectorOfi8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<i8>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfi8_output_array(self.as_raw_mut_VectorOfVectorOfi8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<i8>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfi8_input_output_array(self.as_raw_mut_VectorOfVectorOfi8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<i8>> }
	
	pub type VectorOfVectorOfu8 = core::Vector<core::Vector<u8>>;
	
	impl core::Vector<core::Vector<u8>> {
		pub fn as_raw_VectorOfVectorOfu8(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfu8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<u8>,
		cv_VectorOfVectorOfu8_new, cv_VectorOfVectorOfu8_delete,
		cv_VectorOfVectorOfu8_len, cv_VectorOfVectorOfu8_is_empty,
		cv_VectorOfVectorOfu8_capacity, cv_VectorOfVectorOfu8_shrink_to_fit,
		cv_VectorOfVectorOfu8_reserve, cv_VectorOfVectorOfu8_remove,
		cv_VectorOfVectorOfu8_swap, cv_VectorOfVectorOfu8_clear,
		cv_VectorOfVectorOfu8_get, cv_VectorOfVectorOfu8_set,
		cv_VectorOfVectorOfu8_push, cv_VectorOfVectorOfu8_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<u8> }
	
	extern "C" {
		fn cv_VectorOfVectorOfu8_input_array(instance: extern_send!(core::Vector<core::Vector<u8>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfVectorOfu8_output_array(instance: extern_send!(mut core::Vector<core::Vector<u8>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfVectorOfu8_input_output_array(instance: extern_send!(mut core::Vector<core::Vector<u8>>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<core::Vector<u8>> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfu8_input_array(self.as_raw_VectorOfVectorOfu8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<core::Vector<u8>> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfu8_output_array(self.as_raw_mut_VectorOfVectorOfu8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<core::Vector<u8>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfVectorOfu8_input_output_array(self.as_raw_mut_VectorOfVectorOfu8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<core::Vector<u8>> }
	
	pub type VectorOfbool = core::Vector<bool>;
	
	impl core::Vector<bool> {
		pub fn as_raw_VectorOfbool(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfbool(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { bool,
		cv_VectorOfbool_new, cv_VectorOfbool_delete,
		cv_VectorOfbool_len, cv_VectorOfbool_is_empty,
		cv_VectorOfbool_capacity, cv_VectorOfbool_shrink_to_fit,
		cv_VectorOfbool_reserve, cv_VectorOfbool_remove,
		cv_VectorOfbool_swap, cv_VectorOfbool_clear,
		cv_VectorOfbool_get, cv_VectorOfbool_set,
		cv_VectorOfbool_push, cv_VectorOfbool_insert,
	}
	vector_non_copy_or_bool! { clone bool }
	
	pub type VectorOff32 = core::Vector<f32>;
	
	impl core::Vector<f32> {
		pub fn as_raw_VectorOff32(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOff32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { f32,
		cv_VectorOff32_new, cv_VectorOff32_delete,
		cv_VectorOff32_len, cv_VectorOff32_is_empty,
		cv_VectorOff32_capacity, cv_VectorOff32_shrink_to_fit,
		cv_VectorOff32_reserve, cv_VectorOff32_remove,
		cv_VectorOff32_swap, cv_VectorOff32_clear,
		cv_VectorOff32_get, cv_VectorOff32_set,
		cv_VectorOff32_push, cv_VectorOff32_insert,
	}
	vector_copy_non_bool! { f32,
		cv_VectorOff32_data, cv_VectorOff32_data_mut, cv_VectorOff32_from_slice,
		cv_VectorOff32_clone,
	}
	
	extern "C" {
		fn cv_VectorOff32_input_array(instance: extern_send!(core::Vector<f32>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOff32_output_array(instance: extern_send!(mut core::Vector<f32>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOff32_input_output_array(instance: extern_send!(mut core::Vector<f32>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<f32> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOff32_input_array(self.as_raw_VectorOff32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<f32> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOff32_output_array(self.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<f32> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOff32_input_output_array(self.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<f32> }
	
	pub type VectorOff64 = core::Vector<f64>;
	
	impl core::Vector<f64> {
		pub fn as_raw_VectorOff64(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOff64(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { f64,
		cv_VectorOff64_new, cv_VectorOff64_delete,
		cv_VectorOff64_len, cv_VectorOff64_is_empty,
		cv_VectorOff64_capacity, cv_VectorOff64_shrink_to_fit,
		cv_VectorOff64_reserve, cv_VectorOff64_remove,
		cv_VectorOff64_swap, cv_VectorOff64_clear,
		cv_VectorOff64_get, cv_VectorOff64_set,
		cv_VectorOff64_push, cv_VectorOff64_insert,
	}
	vector_copy_non_bool! { f64,
		cv_VectorOff64_data, cv_VectorOff64_data_mut, cv_VectorOff64_from_slice,
		cv_VectorOff64_clone,
	}
	
	extern "C" {
		fn cv_VectorOff64_input_array(instance: extern_send!(core::Vector<f64>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOff64_output_array(instance: extern_send!(mut core::Vector<f64>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOff64_input_output_array(instance: extern_send!(mut core::Vector<f64>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<f64> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOff64_input_array(self.as_raw_VectorOff64(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<f64> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOff64_output_array(self.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<f64> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOff64_input_output_array(self.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<f64> }
	
	pub type VectorOfi32 = core::Vector<i32>;
	
	impl core::Vector<i32> {
		pub fn as_raw_VectorOfi32(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfi32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { i32,
		cv_VectorOfi32_new, cv_VectorOfi32_delete,
		cv_VectorOfi32_len, cv_VectorOfi32_is_empty,
		cv_VectorOfi32_capacity, cv_VectorOfi32_shrink_to_fit,
		cv_VectorOfi32_reserve, cv_VectorOfi32_remove,
		cv_VectorOfi32_swap, cv_VectorOfi32_clear,
		cv_VectorOfi32_get, cv_VectorOfi32_set,
		cv_VectorOfi32_push, cv_VectorOfi32_insert,
	}
	vector_copy_non_bool! { i32,
		cv_VectorOfi32_data, cv_VectorOfi32_data_mut, cv_VectorOfi32_from_slice,
		cv_VectorOfi32_clone,
	}
	
	extern "C" {
		fn cv_VectorOfi32_input_array(instance: extern_send!(core::Vector<i32>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfi32_output_array(instance: extern_send!(mut core::Vector<i32>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfi32_input_output_array(instance: extern_send!(mut core::Vector<i32>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<i32> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfi32_input_array(self.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<i32> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfi32_output_array(self.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<i32> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfi32_input_output_array(self.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<i32> }
	
	pub type VectorOfi8 = core::Vector<i8>;
	
	impl core::Vector<i8> {
		pub fn as_raw_VectorOfi8(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfi8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { i8,
		cv_VectorOfi8_new, cv_VectorOfi8_delete,
		cv_VectorOfi8_len, cv_VectorOfi8_is_empty,
		cv_VectorOfi8_capacity, cv_VectorOfi8_shrink_to_fit,
		cv_VectorOfi8_reserve, cv_VectorOfi8_remove,
		cv_VectorOfi8_swap, cv_VectorOfi8_clear,
		cv_VectorOfi8_get, cv_VectorOfi8_set,
		cv_VectorOfi8_push, cv_VectorOfi8_insert,
	}
	vector_copy_non_bool! { i8,
		cv_VectorOfi8_data, cv_VectorOfi8_data_mut, cv_VectorOfi8_from_slice,
		cv_VectorOfi8_clone,
	}
	
	extern "C" {
		fn cv_VectorOfi8_input_array(instance: extern_send!(core::Vector<i8>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfi8_output_array(instance: extern_send!(mut core::Vector<i8>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfi8_input_output_array(instance: extern_send!(mut core::Vector<i8>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<i8> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfi8_input_array(self.as_raw_VectorOfi8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<i8> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfi8_output_array(self.as_raw_mut_VectorOfi8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<i8> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfi8_input_output_array(self.as_raw_mut_VectorOfi8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<i8> }
	
	pub type VectorOfsize_t = core::Vector<size_t>;
	
	impl core::Vector<size_t> {
		pub fn as_raw_VectorOfsize_t(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfsize_t(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { size_t,
		cv_VectorOfsize_t_new, cv_VectorOfsize_t_delete,
		cv_VectorOfsize_t_len, cv_VectorOfsize_t_is_empty,
		cv_VectorOfsize_t_capacity, cv_VectorOfsize_t_shrink_to_fit,
		cv_VectorOfsize_t_reserve, cv_VectorOfsize_t_remove,
		cv_VectorOfsize_t_swap, cv_VectorOfsize_t_clear,
		cv_VectorOfsize_t_get, cv_VectorOfsize_t_set,
		cv_VectorOfsize_t_push, cv_VectorOfsize_t_insert,
	}
	vector_copy_non_bool! { size_t,
		cv_VectorOfsize_t_data, cv_VectorOfsize_t_data_mut, cv_VectorOfsize_t_from_slice,
		cv_VectorOfsize_t_clone,
	}
	
	pub type VectorOfu8 = core::Vector<u8>;
	
	impl core::Vector<u8> {
		pub fn as_raw_VectorOfu8(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfu8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { u8,
		cv_VectorOfu8_new, cv_VectorOfu8_delete,
		cv_VectorOfu8_len, cv_VectorOfu8_is_empty,
		cv_VectorOfu8_capacity, cv_VectorOfu8_shrink_to_fit,
		cv_VectorOfu8_reserve, cv_VectorOfu8_remove,
		cv_VectorOfu8_swap, cv_VectorOfu8_clear,
		cv_VectorOfu8_get, cv_VectorOfu8_set,
		cv_VectorOfu8_push, cv_VectorOfu8_insert,
	}
	vector_copy_non_bool! { u8,
		cv_VectorOfu8_data, cv_VectorOfu8_data_mut, cv_VectorOfu8_from_slice,
		cv_VectorOfu8_clone,
	}
	
	extern "C" {
		fn cv_VectorOfu8_input_array(instance: extern_send!(core::Vector<u8>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputArray)>);
		fn cv_VectorOfu8_output_array(instance: extern_send!(mut core::Vector<u8>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_OutputArray)>);
		fn cv_VectorOfu8_input_output_array(instance: extern_send!(mut core::Vector<u8>), ocvrs_return: *mut sys::Result<extern_receive!(crate::core::_InputOutputArray)>);
	}
	
	impl core::ToInputArray for core::Vector<u8> {
		#[inline]
		fn input_array(&self) -> Result<core::_InputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfu8_input_array(self.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
		}
	}
	
	impl core::ToOutputArray for core::Vector<u8> {
		#[inline]
		fn output_array(&mut self) -> Result<core::_OutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfu8_output_array(self.as_raw_mut_VectorOfu8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
		}
	}
	
	impl core::ToInputOutputArray for core::Vector<u8> {
		#[inline]
		fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
			return_send!(via ocvrs_return);
			unsafe { cv_VectorOfu8_input_output_array(self.as_raw_mut_VectorOfu8(), ocvrs_return.as_mut_ptr()) }
			return_receive!(unsafe ocvrs_return => ret);
			ret.into_result()
				.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
		}
	}
	
	input_output_array_ref_forward! { core::Vector<u8> }
	
}
#[cfg(ocvrs_has_module_core)]
pub use core_types::*;

#[cfg(ocvrs_has_module_cudaarithm)]
mod cudaarithm_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfConvolution = core::Ptr<crate::cudaarithm::Convolution>;
	
	ptr_extern! { crate::cudaarithm::Convolution,
		cv_PtrOfConvolution_delete, cv_PtrOfConvolution_get_inner_ptr, cv_PtrOfConvolution_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudaarithm::Convolution> {
		#[inline] pub fn as_raw_PtrOfConvolution(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConvolution(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudaarithm::ConvolutionTraitConst for core::Ptr<crate::cudaarithm::Convolution> {
		#[inline] fn as_raw_Convolution(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaarithm::ConvolutionTrait for core::Ptr<crate::cudaarithm::Convolution> {
		#[inline] fn as_raw_mut_Convolution(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudaarithm::Convolution> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudaarithm::Convolution> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDFT = core::Ptr<crate::cudaarithm::DFT>;
	
	ptr_extern! { crate::cudaarithm::DFT,
		cv_PtrOfDFT_delete, cv_PtrOfDFT_get_inner_ptr, cv_PtrOfDFT_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudaarithm::DFT> {
		#[inline] pub fn as_raw_PtrOfDFT(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDFT(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudaarithm::DFTTraitConst for core::Ptr<crate::cudaarithm::DFT> {
		#[inline] fn as_raw_DFT(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaarithm::DFTTrait for core::Ptr<crate::cudaarithm::DFT> {
		#[inline] fn as_raw_mut_DFT(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudaarithm::DFT> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudaarithm::DFT> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLookUpTable = core::Ptr<crate::cudaarithm::LookUpTable>;
	
	ptr_extern! { crate::cudaarithm::LookUpTable,
		cv_PtrOfLookUpTable_delete, cv_PtrOfLookUpTable_get_inner_ptr, cv_PtrOfLookUpTable_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudaarithm::LookUpTable> {
		#[inline] pub fn as_raw_PtrOfLookUpTable(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLookUpTable(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudaarithm::LookUpTableTraitConst for core::Ptr<crate::cudaarithm::LookUpTable> {
		#[inline] fn as_raw_LookUpTable(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaarithm::LookUpTableTrait for core::Ptr<crate::cudaarithm::LookUpTable> {
		#[inline] fn as_raw_mut_LookUpTable(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudaarithm::LookUpTable> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudaarithm::LookUpTable> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_cudaarithm)]
pub use cudaarithm_types::*;

#[cfg(ocvrs_has_module_cudabgsegm)]
mod cudabgsegm_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCUDA_BackgroundSubtractorMOG = core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG>;
	
	ptr_extern! { crate::cudabgsegm::CUDA_BackgroundSubtractorMOG,
		cv_PtrOfCUDA_BackgroundSubtractorMOG_delete, cv_PtrOfCUDA_BackgroundSubtractorMOG_get_inner_ptr, cv_PtrOfCUDA_BackgroundSubtractorMOG_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG> {
		#[inline] pub fn as_raw_PtrOfCUDA_BackgroundSubtractorMOG(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_BackgroundSubtractorMOG(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudabgsegm::CUDA_BackgroundSubtractorMOGTraitConst for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG> {
		#[inline] fn as_raw_CUDA_BackgroundSubtractorMOG(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudabgsegm::CUDA_BackgroundSubtractorMOGTrait for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG> {
		#[inline] fn as_raw_mut_CUDA_BackgroundSubtractorMOG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorTraitConst for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG> {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractorTrait for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG> {
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_BackgroundSubtractorMOG2 = core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2>;
	
	ptr_extern! { crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2,
		cv_PtrOfCUDA_BackgroundSubtractorMOG2_delete, cv_PtrOfCUDA_BackgroundSubtractorMOG2_get_inner_ptr, cv_PtrOfCUDA_BackgroundSubtractorMOG2_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2> {
		#[inline] pub fn as_raw_PtrOfCUDA_BackgroundSubtractorMOG2(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_BackgroundSubtractorMOG2(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2TraitConst for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2> {
		#[inline] fn as_raw_CUDA_BackgroundSubtractorMOG2(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2Trait for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2> {
		#[inline] fn as_raw_mut_CUDA_BackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorTraitConst for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2> {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractorTrait for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2> {
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorMOG2TraitConst for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2> {
		#[inline] fn as_raw_BackgroundSubtractorMOG2(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractorMOG2Trait for core::Ptr<crate::cudabgsegm::CUDA_BackgroundSubtractorMOG2> {
		#[inline] fn as_raw_mut_BackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_cudabgsegm)]
pub use cudabgsegm_types::*;

#[cfg(ocvrs_has_module_cudacodec)]
mod cudacodec_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfEncoderCallback = core::Ptr<crate::cudacodec::EncoderCallback>;
	
	ptr_extern! { crate::cudacodec::EncoderCallback,
		cv_PtrOfEncoderCallback_delete, cv_PtrOfEncoderCallback_get_inner_ptr, cv_PtrOfEncoderCallback_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudacodec::EncoderCallback> {
		#[inline] pub fn as_raw_PtrOfEncoderCallback(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEncoderCallback(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudacodec::EncoderCallbackTraitConst for core::Ptr<crate::cudacodec::EncoderCallback> {
		#[inline] fn as_raw_EncoderCallback(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudacodec::EncoderCallbackTrait for core::Ptr<crate::cudacodec::EncoderCallback> {
		#[inline] fn as_raw_mut_EncoderCallback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRawVideoSource = core::Ptr<crate::cudacodec::RawVideoSource>;
	
	ptr_extern! { crate::cudacodec::RawVideoSource,
		cv_PtrOfRawVideoSource_delete, cv_PtrOfRawVideoSource_get_inner_ptr, cv_PtrOfRawVideoSource_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudacodec::RawVideoSource> {
		#[inline] pub fn as_raw_PtrOfRawVideoSource(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRawVideoSource(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudacodec::RawVideoSourceTraitConst for core::Ptr<crate::cudacodec::RawVideoSource> {
		#[inline] fn as_raw_RawVideoSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudacodec::RawVideoSourceTrait for core::Ptr<crate::cudacodec::RawVideoSource> {
		#[inline] fn as_raw_mut_RawVideoSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfVideoReader = core::Ptr<crate::cudacodec::VideoReader>;
	
	ptr_extern! { crate::cudacodec::VideoReader,
		cv_PtrOfVideoReader_delete, cv_PtrOfVideoReader_get_inner_ptr, cv_PtrOfVideoReader_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudacodec::VideoReader> {
		#[inline] pub fn as_raw_PtrOfVideoReader(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfVideoReader(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudacodec::VideoReaderTraitConst for core::Ptr<crate::cudacodec::VideoReader> {
		#[inline] fn as_raw_VideoReader(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudacodec::VideoReaderTrait for core::Ptr<crate::cudacodec::VideoReader> {
		#[inline] fn as_raw_mut_VideoReader(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfVideoWriter = core::Ptr<crate::cudacodec::VideoWriter>;
	
	ptr_extern! { crate::cudacodec::VideoWriter,
		cv_PtrOfVideoWriter_delete, cv_PtrOfVideoWriter_get_inner_ptr, cv_PtrOfVideoWriter_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudacodec::VideoWriter> {
		#[inline] pub fn as_raw_PtrOfVideoWriter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfVideoWriter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudacodec::VideoWriterTraitConst for core::Ptr<crate::cudacodec::VideoWriter> {
		#[inline] fn as_raw_VideoWriter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudacodec::VideoWriterTrait for core::Ptr<crate::cudacodec::VideoWriter> {
		#[inline] fn as_raw_mut_VideoWriter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_cudacodec)]
pub use cudacodec_types::*;

#[cfg(ocvrs_has_module_cudafeatures2d)]
mod cudafeatures2d_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCUDA_DescriptorMatcher = core::Ptr<crate::cudafeatures2d::CUDA_DescriptorMatcher>;
	
	ptr_extern! { crate::cudafeatures2d::CUDA_DescriptorMatcher,
		cv_PtrOfCUDA_DescriptorMatcher_delete, cv_PtrOfCUDA_DescriptorMatcher_get_inner_ptr, cv_PtrOfCUDA_DescriptorMatcher_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudafeatures2d::CUDA_DescriptorMatcher> {
		#[inline] pub fn as_raw_PtrOfCUDA_DescriptorMatcher(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_DescriptorMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudafeatures2d::CUDA_DescriptorMatcherTraitConst for core::Ptr<crate::cudafeatures2d::CUDA_DescriptorMatcher> {
		#[inline] fn as_raw_CUDA_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudafeatures2d::CUDA_DescriptorMatcherTrait for core::Ptr<crate::cudafeatures2d::CUDA_DescriptorMatcher> {
		#[inline] fn as_raw_mut_CUDA_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudafeatures2d::CUDA_DescriptorMatcher> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudafeatures2d::CUDA_DescriptorMatcher> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_FastFeatureDetector = core::Ptr<crate::cudafeatures2d::CUDA_FastFeatureDetector>;
	
	ptr_extern! { crate::cudafeatures2d::CUDA_FastFeatureDetector,
		cv_PtrOfCUDA_FastFeatureDetector_delete, cv_PtrOfCUDA_FastFeatureDetector_get_inner_ptr, cv_PtrOfCUDA_FastFeatureDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudafeatures2d::CUDA_FastFeatureDetector> {
		#[inline] pub fn as_raw_PtrOfCUDA_FastFeatureDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_FastFeatureDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudafeatures2d::CUDA_FastFeatureDetectorTraitConst for core::Ptr<crate::cudafeatures2d::CUDA_FastFeatureDetector> {
		#[inline] fn as_raw_CUDA_FastFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudafeatures2d::CUDA_FastFeatureDetectorTrait for core::Ptr<crate::cudafeatures2d::CUDA_FastFeatureDetector> {
		#[inline] fn as_raw_mut_CUDA_FastFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudafeatures2d::CUDA_FastFeatureDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudafeatures2d::CUDA_FastFeatureDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::cudafeatures2d::CUDA_FastFeatureDetector> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::cudafeatures2d::CUDA_FastFeatureDetector> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudafeatures2d::CUDA_Feature2DAsyncTraitConst for core::Ptr<crate::cudafeatures2d::CUDA_FastFeatureDetector> {
		#[inline] fn as_raw_CUDA_Feature2DAsync(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudafeatures2d::CUDA_Feature2DAsyncTrait for core::Ptr<crate::cudafeatures2d::CUDA_FastFeatureDetector> {
		#[inline] fn as_raw_mut_CUDA_Feature2DAsync(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_ORB = core::Ptr<crate::cudafeatures2d::CUDA_ORB>;
	
	ptr_extern! { crate::cudafeatures2d::CUDA_ORB,
		cv_PtrOfCUDA_ORB_delete, cv_PtrOfCUDA_ORB_get_inner_ptr, cv_PtrOfCUDA_ORB_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudafeatures2d::CUDA_ORB> {
		#[inline] pub fn as_raw_PtrOfCUDA_ORB(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_ORB(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudafeatures2d::CUDA_ORBTraitConst for core::Ptr<crate::cudafeatures2d::CUDA_ORB> {
		#[inline] fn as_raw_CUDA_ORB(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudafeatures2d::CUDA_ORBTrait for core::Ptr<crate::cudafeatures2d::CUDA_ORB> {
		#[inline] fn as_raw_mut_CUDA_ORB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudafeatures2d::CUDA_ORB> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudafeatures2d::CUDA_ORB> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::cudafeatures2d::CUDA_ORB> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::cudafeatures2d::CUDA_ORB> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudafeatures2d::CUDA_Feature2DAsyncTraitConst for core::Ptr<crate::cudafeatures2d::CUDA_ORB> {
		#[inline] fn as_raw_CUDA_Feature2DAsync(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudafeatures2d::CUDA_Feature2DAsyncTrait for core::Ptr<crate::cudafeatures2d::CUDA_ORB> {
		#[inline] fn as_raw_mut_CUDA_Feature2DAsync(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_cudafeatures2d)]
pub use cudafeatures2d_types::*;

#[cfg(ocvrs_has_module_cudafilters)]
mod cudafilters_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfFilter = core::Ptr<crate::cudafilters::Filter>;
	
	ptr_extern! { crate::cudafilters::Filter,
		cv_PtrOfFilter_delete, cv_PtrOfFilter_get_inner_ptr, cv_PtrOfFilter_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudafilters::Filter> {
		#[inline] pub fn as_raw_PtrOfFilter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudafilters::FilterTraitConst for core::Ptr<crate::cudafilters::Filter> {
		#[inline] fn as_raw_Filter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudafilters::FilterTrait for core::Ptr<crate::cudafilters::Filter> {
		#[inline] fn as_raw_mut_Filter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudafilters::Filter> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudafilters::Filter> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_cudafilters)]
pub use cudafilters_types::*;

#[cfg(ocvrs_has_module_cudaimgproc)]
mod cudaimgproc_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCUDA_CLAHE = core::Ptr<crate::cudaimgproc::CUDA_CLAHE>;
	
	ptr_extern! { crate::cudaimgproc::CUDA_CLAHE,
		cv_PtrOfCUDA_CLAHE_delete, cv_PtrOfCUDA_CLAHE_get_inner_ptr, cv_PtrOfCUDA_CLAHE_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudaimgproc::CUDA_CLAHE> {
		#[inline] pub fn as_raw_PtrOfCUDA_CLAHE(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_CLAHE(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_CLAHETraitConst for core::Ptr<crate::cudaimgproc::CUDA_CLAHE> {
		#[inline] fn as_raw_CUDA_CLAHE(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaimgproc::CUDA_CLAHETrait for core::Ptr<crate::cudaimgproc::CUDA_CLAHE> {
		#[inline] fn as_raw_mut_CUDA_CLAHE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudaimgproc::CUDA_CLAHE> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudaimgproc::CUDA_CLAHE> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::imgproc::CLAHETraitConst for core::Ptr<crate::cudaimgproc::CUDA_CLAHE> {
		#[inline] fn as_raw_CLAHE(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::imgproc::CLAHETrait for core::Ptr<crate::cudaimgproc::CUDA_CLAHE> {
		#[inline] fn as_raw_mut_CLAHE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_CannyEdgeDetector = core::Ptr<crate::cudaimgproc::CUDA_CannyEdgeDetector>;
	
	ptr_extern! { crate::cudaimgproc::CUDA_CannyEdgeDetector,
		cv_PtrOfCUDA_CannyEdgeDetector_delete, cv_PtrOfCUDA_CannyEdgeDetector_get_inner_ptr, cv_PtrOfCUDA_CannyEdgeDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudaimgproc::CUDA_CannyEdgeDetector> {
		#[inline] pub fn as_raw_PtrOfCUDA_CannyEdgeDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_CannyEdgeDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_CannyEdgeDetectorTraitConst for core::Ptr<crate::cudaimgproc::CUDA_CannyEdgeDetector> {
		#[inline] fn as_raw_CUDA_CannyEdgeDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaimgproc::CUDA_CannyEdgeDetectorTrait for core::Ptr<crate::cudaimgproc::CUDA_CannyEdgeDetector> {
		#[inline] fn as_raw_mut_CUDA_CannyEdgeDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudaimgproc::CUDA_CannyEdgeDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudaimgproc::CUDA_CannyEdgeDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_CornernessCriteria = core::Ptr<crate::cudaimgproc::CUDA_CornernessCriteria>;
	
	ptr_extern! { crate::cudaimgproc::CUDA_CornernessCriteria,
		cv_PtrOfCUDA_CornernessCriteria_delete, cv_PtrOfCUDA_CornernessCriteria_get_inner_ptr, cv_PtrOfCUDA_CornernessCriteria_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudaimgproc::CUDA_CornernessCriteria> {
		#[inline] pub fn as_raw_PtrOfCUDA_CornernessCriteria(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_CornernessCriteria(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_CornernessCriteriaTraitConst for core::Ptr<crate::cudaimgproc::CUDA_CornernessCriteria> {
		#[inline] fn as_raw_CUDA_CornernessCriteria(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaimgproc::CUDA_CornernessCriteriaTrait for core::Ptr<crate::cudaimgproc::CUDA_CornernessCriteria> {
		#[inline] fn as_raw_mut_CUDA_CornernessCriteria(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudaimgproc::CUDA_CornernessCriteria> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudaimgproc::CUDA_CornernessCriteria> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_CornersDetector = core::Ptr<crate::cudaimgproc::CUDA_CornersDetector>;
	
	ptr_extern! { crate::cudaimgproc::CUDA_CornersDetector,
		cv_PtrOfCUDA_CornersDetector_delete, cv_PtrOfCUDA_CornersDetector_get_inner_ptr, cv_PtrOfCUDA_CornersDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudaimgproc::CUDA_CornersDetector> {
		#[inline] pub fn as_raw_PtrOfCUDA_CornersDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_CornersDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_CornersDetectorTraitConst for core::Ptr<crate::cudaimgproc::CUDA_CornersDetector> {
		#[inline] fn as_raw_CUDA_CornersDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaimgproc::CUDA_CornersDetectorTrait for core::Ptr<crate::cudaimgproc::CUDA_CornersDetector> {
		#[inline] fn as_raw_mut_CUDA_CornersDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudaimgproc::CUDA_CornersDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudaimgproc::CUDA_CornersDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_HoughCirclesDetector = core::Ptr<crate::cudaimgproc::CUDA_HoughCirclesDetector>;
	
	ptr_extern! { crate::cudaimgproc::CUDA_HoughCirclesDetector,
		cv_PtrOfCUDA_HoughCirclesDetector_delete, cv_PtrOfCUDA_HoughCirclesDetector_get_inner_ptr, cv_PtrOfCUDA_HoughCirclesDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudaimgproc::CUDA_HoughCirclesDetector> {
		#[inline] pub fn as_raw_PtrOfCUDA_HoughCirclesDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_HoughCirclesDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_HoughCirclesDetectorTraitConst for core::Ptr<crate::cudaimgproc::CUDA_HoughCirclesDetector> {
		#[inline] fn as_raw_CUDA_HoughCirclesDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaimgproc::CUDA_HoughCirclesDetectorTrait for core::Ptr<crate::cudaimgproc::CUDA_HoughCirclesDetector> {
		#[inline] fn as_raw_mut_CUDA_HoughCirclesDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudaimgproc::CUDA_HoughCirclesDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudaimgproc::CUDA_HoughCirclesDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_HoughLinesDetector = core::Ptr<crate::cudaimgproc::CUDA_HoughLinesDetector>;
	
	ptr_extern! { crate::cudaimgproc::CUDA_HoughLinesDetector,
		cv_PtrOfCUDA_HoughLinesDetector_delete, cv_PtrOfCUDA_HoughLinesDetector_get_inner_ptr, cv_PtrOfCUDA_HoughLinesDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudaimgproc::CUDA_HoughLinesDetector> {
		#[inline] pub fn as_raw_PtrOfCUDA_HoughLinesDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_HoughLinesDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_HoughLinesDetectorTraitConst for core::Ptr<crate::cudaimgproc::CUDA_HoughLinesDetector> {
		#[inline] fn as_raw_CUDA_HoughLinesDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaimgproc::CUDA_HoughLinesDetectorTrait for core::Ptr<crate::cudaimgproc::CUDA_HoughLinesDetector> {
		#[inline] fn as_raw_mut_CUDA_HoughLinesDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudaimgproc::CUDA_HoughLinesDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudaimgproc::CUDA_HoughLinesDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_HoughSegmentDetector = core::Ptr<crate::cudaimgproc::CUDA_HoughSegmentDetector>;
	
	ptr_extern! { crate::cudaimgproc::CUDA_HoughSegmentDetector,
		cv_PtrOfCUDA_HoughSegmentDetector_delete, cv_PtrOfCUDA_HoughSegmentDetector_get_inner_ptr, cv_PtrOfCUDA_HoughSegmentDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudaimgproc::CUDA_HoughSegmentDetector> {
		#[inline] pub fn as_raw_PtrOfCUDA_HoughSegmentDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_HoughSegmentDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_HoughSegmentDetectorTraitConst for core::Ptr<crate::cudaimgproc::CUDA_HoughSegmentDetector> {
		#[inline] fn as_raw_CUDA_HoughSegmentDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaimgproc::CUDA_HoughSegmentDetectorTrait for core::Ptr<crate::cudaimgproc::CUDA_HoughSegmentDetector> {
		#[inline] fn as_raw_mut_CUDA_HoughSegmentDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudaimgproc::CUDA_HoughSegmentDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudaimgproc::CUDA_HoughSegmentDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_TemplateMatching = core::Ptr<crate::cudaimgproc::CUDA_TemplateMatching>;
	
	ptr_extern! { crate::cudaimgproc::CUDA_TemplateMatching,
		cv_PtrOfCUDA_TemplateMatching_delete, cv_PtrOfCUDA_TemplateMatching_get_inner_ptr, cv_PtrOfCUDA_TemplateMatching_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudaimgproc::CUDA_TemplateMatching> {
		#[inline] pub fn as_raw_PtrOfCUDA_TemplateMatching(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_TemplateMatching(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudaimgproc::CUDA_TemplateMatchingTraitConst for core::Ptr<crate::cudaimgproc::CUDA_TemplateMatching> {
		#[inline] fn as_raw_CUDA_TemplateMatching(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaimgproc::CUDA_TemplateMatchingTrait for core::Ptr<crate::cudaimgproc::CUDA_TemplateMatching> {
		#[inline] fn as_raw_mut_CUDA_TemplateMatching(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudaimgproc::CUDA_TemplateMatching> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudaimgproc::CUDA_TemplateMatching> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_cudaimgproc)]
pub use cudaimgproc_types::*;

#[cfg(ocvrs_has_module_cudaobjdetect)]
mod cudaobjdetect_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCUDA_CascadeClassifier = core::Ptr<crate::cudaobjdetect::CUDA_CascadeClassifier>;
	
	ptr_extern! { crate::cudaobjdetect::CUDA_CascadeClassifier,
		cv_PtrOfCUDA_CascadeClassifier_delete, cv_PtrOfCUDA_CascadeClassifier_get_inner_ptr, cv_PtrOfCUDA_CascadeClassifier_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudaobjdetect::CUDA_CascadeClassifier> {
		#[inline] pub fn as_raw_PtrOfCUDA_CascadeClassifier(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_CascadeClassifier(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudaobjdetect::CUDA_CascadeClassifierTraitConst for core::Ptr<crate::cudaobjdetect::CUDA_CascadeClassifier> {
		#[inline] fn as_raw_CUDA_CascadeClassifier(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaobjdetect::CUDA_CascadeClassifierTrait for core::Ptr<crate::cudaobjdetect::CUDA_CascadeClassifier> {
		#[inline] fn as_raw_mut_CUDA_CascadeClassifier(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudaobjdetect::CUDA_CascadeClassifier> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudaobjdetect::CUDA_CascadeClassifier> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_HOG = core::Ptr<crate::cudaobjdetect::CUDA_HOG>;
	
	ptr_extern! { crate::cudaobjdetect::CUDA_HOG,
		cv_PtrOfCUDA_HOG_delete, cv_PtrOfCUDA_HOG_get_inner_ptr, cv_PtrOfCUDA_HOG_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudaobjdetect::CUDA_HOG> {
		#[inline] pub fn as_raw_PtrOfCUDA_HOG(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_HOG(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudaobjdetect::CUDA_HOGTraitConst for core::Ptr<crate::cudaobjdetect::CUDA_HOG> {
		#[inline] fn as_raw_CUDA_HOG(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaobjdetect::CUDA_HOGTrait for core::Ptr<crate::cudaobjdetect::CUDA_HOG> {
		#[inline] fn as_raw_mut_CUDA_HOG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudaobjdetect::CUDA_HOG> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudaobjdetect::CUDA_HOG> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_cudaobjdetect)]
pub use cudaobjdetect_types::*;

#[cfg(ocvrs_has_module_cudaoptflow)]
mod cudaoptflow_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCUDA_BroxOpticalFlow = core::Ptr<crate::cudaoptflow::CUDA_BroxOpticalFlow>;
	
	ptr_extern! { crate::cudaoptflow::CUDA_BroxOpticalFlow,
		cv_PtrOfCUDA_BroxOpticalFlow_delete, cv_PtrOfCUDA_BroxOpticalFlow_get_inner_ptr, cv_PtrOfCUDA_BroxOpticalFlow_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudaoptflow::CUDA_BroxOpticalFlow> {
		#[inline] pub fn as_raw_PtrOfCUDA_BroxOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_BroxOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_BroxOpticalFlowTraitConst for core::Ptr<crate::cudaoptflow::CUDA_BroxOpticalFlow> {
		#[inline] fn as_raw_CUDA_BroxOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_BroxOpticalFlowTrait for core::Ptr<crate::cudaoptflow::CUDA_BroxOpticalFlow> {
		#[inline] fn as_raw_mut_CUDA_BroxOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudaoptflow::CUDA_BroxOpticalFlow> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudaoptflow::CUDA_BroxOpticalFlow> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlowTraitConst for core::Ptr<crate::cudaoptflow::CUDA_BroxOpticalFlow> {
		#[inline] fn as_raw_CUDA_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlowTrait for core::Ptr<crate::cudaoptflow::CUDA_BroxOpticalFlow> {
		#[inline] fn as_raw_mut_CUDA_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_DensePyrLKOpticalFlow = core::Ptr<crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow>;
	
	ptr_extern! { crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow,
		cv_PtrOfCUDA_DensePyrLKOpticalFlow_delete, cv_PtrOfCUDA_DensePyrLKOpticalFlow_get_inner_ptr, cv_PtrOfCUDA_DensePyrLKOpticalFlow_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow> {
		#[inline] pub fn as_raw_PtrOfCUDA_DensePyrLKOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_DensePyrLKOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_DensePyrLKOpticalFlowTraitConst for core::Ptr<crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow> {
		#[inline] fn as_raw_CUDA_DensePyrLKOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_DensePyrLKOpticalFlowTrait for core::Ptr<crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow> {
		#[inline] fn as_raw_mut_CUDA_DensePyrLKOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlowTraitConst for core::Ptr<crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow> {
		#[inline] fn as_raw_CUDA_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlowTrait for core::Ptr<crate::cudaoptflow::CUDA_DensePyrLKOpticalFlow> {
		#[inline] fn as_raw_mut_CUDA_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_FarnebackOpticalFlow = core::Ptr<crate::cudaoptflow::CUDA_FarnebackOpticalFlow>;
	
	ptr_extern! { crate::cudaoptflow::CUDA_FarnebackOpticalFlow,
		cv_PtrOfCUDA_FarnebackOpticalFlow_delete, cv_PtrOfCUDA_FarnebackOpticalFlow_get_inner_ptr, cv_PtrOfCUDA_FarnebackOpticalFlow_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudaoptflow::CUDA_FarnebackOpticalFlow> {
		#[inline] pub fn as_raw_PtrOfCUDA_FarnebackOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_FarnebackOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_FarnebackOpticalFlowTraitConst for core::Ptr<crate::cudaoptflow::CUDA_FarnebackOpticalFlow> {
		#[inline] fn as_raw_CUDA_FarnebackOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_FarnebackOpticalFlowTrait for core::Ptr<crate::cudaoptflow::CUDA_FarnebackOpticalFlow> {
		#[inline] fn as_raw_mut_CUDA_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudaoptflow::CUDA_FarnebackOpticalFlow> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudaoptflow::CUDA_FarnebackOpticalFlow> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlowTraitConst for core::Ptr<crate::cudaoptflow::CUDA_FarnebackOpticalFlow> {
		#[inline] fn as_raw_CUDA_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlowTrait for core::Ptr<crate::cudaoptflow::CUDA_FarnebackOpticalFlow> {
		#[inline] fn as_raw_mut_CUDA_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_NvidiaOpticalFlow_1_0 = core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0>;
	
	ptr_extern! { crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0,
		cv_PtrOfCUDA_NvidiaOpticalFlow_1_0_delete, cv_PtrOfCUDA_NvidiaOpticalFlow_1_0_get_inner_ptr, cv_PtrOfCUDA_NvidiaOpticalFlow_1_0_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0> {
		#[inline] pub fn as_raw_PtrOfCUDA_NvidiaOpticalFlow_1_0(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_NvidiaOpticalFlow_1_0(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0TraitConst for core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0> {
		#[inline] fn as_raw_CUDA_NvidiaOpticalFlow_1_0(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0Trait for core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0> {
		#[inline] fn as_raw_mut_CUDA_NvidiaOpticalFlow_1_0(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaHWOpticalFlowTraitConst for core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0> {
		#[inline] fn as_raw_CUDA_NvidiaHWOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaHWOpticalFlowTrait for core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_1_0> {
		#[inline] fn as_raw_mut_CUDA_NvidiaHWOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_NvidiaOpticalFlow_2_0 = core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0>;
	
	ptr_extern! { crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0,
		cv_PtrOfCUDA_NvidiaOpticalFlow_2_0_delete, cv_PtrOfCUDA_NvidiaOpticalFlow_2_0_get_inner_ptr, cv_PtrOfCUDA_NvidiaOpticalFlow_2_0_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0> {
		#[inline] pub fn as_raw_PtrOfCUDA_NvidiaOpticalFlow_2_0(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_NvidiaOpticalFlow_2_0(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0TraitConst for core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0> {
		#[inline] fn as_raw_CUDA_NvidiaOpticalFlow_2_0(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0Trait for core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0> {
		#[inline] fn as_raw_mut_CUDA_NvidiaOpticalFlow_2_0(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaHWOpticalFlowTraitConst for core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0> {
		#[inline] fn as_raw_CUDA_NvidiaHWOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_NvidiaHWOpticalFlowTrait for core::Ptr<crate::cudaoptflow::CUDA_NvidiaOpticalFlow_2_0> {
		#[inline] fn as_raw_mut_CUDA_NvidiaHWOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_OpticalFlowDual_TVL1 = core::Ptr<crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1>;
	
	ptr_extern! { crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1,
		cv_PtrOfCUDA_OpticalFlowDual_TVL1_delete, cv_PtrOfCUDA_OpticalFlowDual_TVL1_get_inner_ptr, cv_PtrOfCUDA_OpticalFlowDual_TVL1_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1> {
		#[inline] pub fn as_raw_PtrOfCUDA_OpticalFlowDual_TVL1(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_OpticalFlowDual_TVL1(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1TraitConst for core::Ptr<crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1> {
		#[inline] fn as_raw_CUDA_OpticalFlowDual_TVL1(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1Trait for core::Ptr<crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1> {
		#[inline] fn as_raw_mut_CUDA_OpticalFlowDual_TVL1(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlowTraitConst for core::Ptr<crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1> {
		#[inline] fn as_raw_CUDA_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_DenseOpticalFlowTrait for core::Ptr<crate::cudaoptflow::CUDA_OpticalFlowDual_TVL1> {
		#[inline] fn as_raw_mut_CUDA_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_SparsePyrLKOpticalFlow = core::Ptr<crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow>;
	
	ptr_extern! { crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow,
		cv_PtrOfCUDA_SparsePyrLKOpticalFlow_delete, cv_PtrOfCUDA_SparsePyrLKOpticalFlow_get_inner_ptr, cv_PtrOfCUDA_SparsePyrLKOpticalFlow_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow> {
		#[inline] pub fn as_raw_PtrOfCUDA_SparsePyrLKOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_SparsePyrLKOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlowTraitConst for core::Ptr<crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow> {
		#[inline] fn as_raw_CUDA_SparsePyrLKOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlowTrait for core::Ptr<crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow> {
		#[inline] fn as_raw_mut_CUDA_SparsePyrLKOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudaoptflow::CUDA_SparseOpticalFlowTraitConst for core::Ptr<crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow> {
		#[inline] fn as_raw_CUDA_SparseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudaoptflow::CUDA_SparseOpticalFlowTrait for core::Ptr<crate::cudaoptflow::CUDA_SparsePyrLKOpticalFlow> {
		#[inline] fn as_raw_mut_CUDA_SparseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_cudaoptflow)]
pub use cudaoptflow_types::*;

#[cfg(ocvrs_has_module_cudastereo)]
mod cudastereo_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCUDA_DisparityBilateralFilter = core::Ptr<crate::cudastereo::CUDA_DisparityBilateralFilter>;
	
	ptr_extern! { crate::cudastereo::CUDA_DisparityBilateralFilter,
		cv_PtrOfCUDA_DisparityBilateralFilter_delete, cv_PtrOfCUDA_DisparityBilateralFilter_get_inner_ptr, cv_PtrOfCUDA_DisparityBilateralFilter_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudastereo::CUDA_DisparityBilateralFilter> {
		#[inline] pub fn as_raw_PtrOfCUDA_DisparityBilateralFilter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_DisparityBilateralFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudastereo::CUDA_DisparityBilateralFilterTraitConst for core::Ptr<crate::cudastereo::CUDA_DisparityBilateralFilter> {
		#[inline] fn as_raw_CUDA_DisparityBilateralFilter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudastereo::CUDA_DisparityBilateralFilterTrait for core::Ptr<crate::cudastereo::CUDA_DisparityBilateralFilter> {
		#[inline] fn as_raw_mut_CUDA_DisparityBilateralFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudastereo::CUDA_DisparityBilateralFilter> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudastereo::CUDA_DisparityBilateralFilter> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_StereoBM = core::Ptr<crate::cudastereo::CUDA_StereoBM>;
	
	ptr_extern! { crate::cudastereo::CUDA_StereoBM,
		cv_PtrOfCUDA_StereoBM_delete, cv_PtrOfCUDA_StereoBM_get_inner_ptr, cv_PtrOfCUDA_StereoBM_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudastereo::CUDA_StereoBM> {
		#[inline] pub fn as_raw_PtrOfCUDA_StereoBM(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_StereoBM(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudastereo::CUDA_StereoBMTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoBM> {
		#[inline] fn as_raw_CUDA_StereoBM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudastereo::CUDA_StereoBMTrait for core::Ptr<crate::cudastereo::CUDA_StereoBM> {
		#[inline] fn as_raw_mut_CUDA_StereoBM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoBM> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudastereo::CUDA_StereoBM> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoBMTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoBM> {
		#[inline] fn as_raw_StereoBM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoBMTrait for core::Ptr<crate::cudastereo::CUDA_StereoBM> {
		#[inline] fn as_raw_mut_StereoBM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcherTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoBM> {
		#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoMatcherTrait for core::Ptr<crate::cudastereo::CUDA_StereoBM> {
		#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_StereoBeliefPropagation = core::Ptr<crate::cudastereo::CUDA_StereoBeliefPropagation>;
	
	ptr_extern! { crate::cudastereo::CUDA_StereoBeliefPropagation,
		cv_PtrOfCUDA_StereoBeliefPropagation_delete, cv_PtrOfCUDA_StereoBeliefPropagation_get_inner_ptr, cv_PtrOfCUDA_StereoBeliefPropagation_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudastereo::CUDA_StereoBeliefPropagation> {
		#[inline] pub fn as_raw_PtrOfCUDA_StereoBeliefPropagation(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_StereoBeliefPropagation(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudastereo::CUDA_StereoBeliefPropagationTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoBeliefPropagation> {
		#[inline] fn as_raw_CUDA_StereoBeliefPropagation(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudastereo::CUDA_StereoBeliefPropagationTrait for core::Ptr<crate::cudastereo::CUDA_StereoBeliefPropagation> {
		#[inline] fn as_raw_mut_CUDA_StereoBeliefPropagation(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoBeliefPropagation> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudastereo::CUDA_StereoBeliefPropagation> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcherTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoBeliefPropagation> {
		#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoMatcherTrait for core::Ptr<crate::cudastereo::CUDA_StereoBeliefPropagation> {
		#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_StereoConstantSpaceBP = core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP>;
	
	ptr_extern! { crate::cudastereo::CUDA_StereoConstantSpaceBP,
		cv_PtrOfCUDA_StereoConstantSpaceBP_delete, cv_PtrOfCUDA_StereoConstantSpaceBP_get_inner_ptr, cv_PtrOfCUDA_StereoConstantSpaceBP_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP> {
		#[inline] pub fn as_raw_PtrOfCUDA_StereoConstantSpaceBP(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_StereoConstantSpaceBP(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudastereo::CUDA_StereoConstantSpaceBPTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP> {
		#[inline] fn as_raw_CUDA_StereoConstantSpaceBP(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudastereo::CUDA_StereoConstantSpaceBPTrait for core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP> {
		#[inline] fn as_raw_mut_CUDA_StereoConstantSpaceBP(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcherTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP> {
		#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoMatcherTrait for core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP> {
		#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::cudastereo::CUDA_StereoBeliefPropagationTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP> {
		#[inline] fn as_raw_CUDA_StereoBeliefPropagation(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudastereo::CUDA_StereoBeliefPropagationTrait for core::Ptr<crate::cudastereo::CUDA_StereoConstantSpaceBP> {
		#[inline] fn as_raw_mut_CUDA_StereoBeliefPropagation(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCUDA_StereoSGM = core::Ptr<crate::cudastereo::CUDA_StereoSGM>;
	
	ptr_extern! { crate::cudastereo::CUDA_StereoSGM,
		cv_PtrOfCUDA_StereoSGM_delete, cv_PtrOfCUDA_StereoSGM_get_inner_ptr, cv_PtrOfCUDA_StereoSGM_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::cudastereo::CUDA_StereoSGM> {
		#[inline] pub fn as_raw_PtrOfCUDA_StereoSGM(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCUDA_StereoSGM(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::cudastereo::CUDA_StereoSGMTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoSGM> {
		#[inline] fn as_raw_CUDA_StereoSGM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::cudastereo::CUDA_StereoSGMTrait for core::Ptr<crate::cudastereo::CUDA_StereoSGM> {
		#[inline] fn as_raw_mut_CUDA_StereoSGM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoSGM> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::cudastereo::CUDA_StereoSGM> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoMatcherTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoSGM> {
		#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoMatcherTrait for core::Ptr<crate::cudastereo::CUDA_StereoSGM> {
		#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::calib3d::StereoSGBMTraitConst for core::Ptr<crate::cudastereo::CUDA_StereoSGM> {
		#[inline] fn as_raw_StereoSGBM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::calib3d::StereoSGBMTrait for core::Ptr<crate::cudastereo::CUDA_StereoSGM> {
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
	
	impl core::Ptr<crate::dnn::AbsLayer> {
		#[inline] pub fn as_raw_PtrOfAbsLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAbsLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::AbsLayerTraitConst for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_AbsLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AbsLayerTrait for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_mut_AbsLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAccumLayer = core::Ptr<crate::dnn::AccumLayer>;
	
	ptr_extern! { crate::dnn::AccumLayer,
		cv_PtrOfAccumLayer_delete, cv_PtrOfAccumLayer_get_inner_ptr, cv_PtrOfAccumLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AccumLayer, cv_PtrOfAccumLayer_new }
	
	impl core::Ptr<crate::dnn::AccumLayer> {
		#[inline] pub fn as_raw_PtrOfAccumLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAccumLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::AccumLayerTraitConst for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_AccumLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AccumLayerTrait for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_mut_AccumLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAcosLayer = core::Ptr<crate::dnn::AcosLayer>;
	
	ptr_extern! { crate::dnn::AcosLayer,
		cv_PtrOfAcosLayer_delete, cv_PtrOfAcosLayer_get_inner_ptr, cv_PtrOfAcosLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AcosLayer, cv_PtrOfAcosLayer_new }
	
	impl core::Ptr<crate::dnn::AcosLayer> {
		#[inline] pub fn as_raw_PtrOfAcosLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAcosLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::AcosLayerTraitConst for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_AcosLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AcosLayerTrait for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_mut_AcosLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAcoshLayer = core::Ptr<crate::dnn::AcoshLayer>;
	
	ptr_extern! { crate::dnn::AcoshLayer,
		cv_PtrOfAcoshLayer_delete, cv_PtrOfAcoshLayer_get_inner_ptr, cv_PtrOfAcoshLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AcoshLayer, cv_PtrOfAcoshLayer_new }
	
	impl core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] pub fn as_raw_PtrOfAcoshLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAcoshLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::AcoshLayerTraitConst for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_AcoshLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AcoshLayerTrait for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_mut_AcoshLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfActivationLayer = core::Ptr<crate::dnn::ActivationLayer>;
	
	ptr_extern! { crate::dnn::ActivationLayer,
		cv_PtrOfActivationLayer_delete, cv_PtrOfActivationLayer_get_inner_ptr, cv_PtrOfActivationLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ActivationLayer, cv_PtrOfActivationLayer_new }
	
	impl core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] pub fn as_raw_PtrOfActivationLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfActivationLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfActivationLayerInt8 = core::Ptr<crate::dnn::ActivationLayerInt8>;
	
	ptr_extern! { crate::dnn::ActivationLayerInt8,
		cv_PtrOfActivationLayerInt8_delete, cv_PtrOfActivationLayerInt8_get_inner_ptr, cv_PtrOfActivationLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ActivationLayerInt8, cv_PtrOfActivationLayerInt8_new }
	
	impl core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] pub fn as_raw_PtrOfActivationLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfActivationLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerInt8TraitConst for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_ActivationLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerInt8Trait for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_mut_ActivationLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfArgLayer = core::Ptr<crate::dnn::ArgLayer>;
	
	ptr_extern! { crate::dnn::ArgLayer,
		cv_PtrOfArgLayer_delete, cv_PtrOfArgLayer_get_inner_ptr, cv_PtrOfArgLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ArgLayer, cv_PtrOfArgLayer_new }
	
	impl core::Ptr<crate::dnn::ArgLayer> {
		#[inline] pub fn as_raw_PtrOfArgLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfArgLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ArgLayerTraitConst for core::Ptr<crate::dnn::ArgLayer> {
		#[inline] fn as_raw_ArgLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ArgLayerTrait for core::Ptr<crate::dnn::ArgLayer> {
		#[inline] fn as_raw_mut_ArgLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ArgLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ArgLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ArgLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ArgLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAsinLayer = core::Ptr<crate::dnn::AsinLayer>;
	
	ptr_extern! { crate::dnn::AsinLayer,
		cv_PtrOfAsinLayer_delete, cv_PtrOfAsinLayer_get_inner_ptr, cv_PtrOfAsinLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AsinLayer, cv_PtrOfAsinLayer_new }
	
	impl core::Ptr<crate::dnn::AsinLayer> {
		#[inline] pub fn as_raw_PtrOfAsinLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAsinLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::AsinLayerTraitConst for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_AsinLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AsinLayerTrait for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_mut_AsinLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAsinhLayer = core::Ptr<crate::dnn::AsinhLayer>;
	
	ptr_extern! { crate::dnn::AsinhLayer,
		cv_PtrOfAsinhLayer_delete, cv_PtrOfAsinhLayer_get_inner_ptr, cv_PtrOfAsinhLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AsinhLayer, cv_PtrOfAsinhLayer_new }
	
	impl core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] pub fn as_raw_PtrOfAsinhLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAsinhLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::AsinhLayerTraitConst for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_AsinhLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AsinhLayerTrait for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_mut_AsinhLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAtanLayer = core::Ptr<crate::dnn::AtanLayer>;
	
	ptr_extern! { crate::dnn::AtanLayer,
		cv_PtrOfAtanLayer_delete, cv_PtrOfAtanLayer_get_inner_ptr, cv_PtrOfAtanLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AtanLayer, cv_PtrOfAtanLayer_new }
	
	impl core::Ptr<crate::dnn::AtanLayer> {
		#[inline] pub fn as_raw_PtrOfAtanLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAtanLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::AtanLayerTraitConst for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_AtanLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AtanLayerTrait for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_mut_AtanLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfAtanhLayer = core::Ptr<crate::dnn::AtanhLayer>;
	
	ptr_extern! { crate::dnn::AtanhLayer,
		cv_PtrOfAtanhLayer_delete, cv_PtrOfAtanhLayer_get_inner_ptr, cv_PtrOfAtanhLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::AtanhLayer, cv_PtrOfAtanhLayer_new }
	
	impl core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] pub fn as_raw_PtrOfAtanhLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAtanhLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::AtanhLayerTraitConst for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_AtanhLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::AtanhLayerTrait for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_mut_AtanhLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBNLLLayer = core::Ptr<crate::dnn::BNLLLayer>;
	
	ptr_extern! { crate::dnn::BNLLLayer,
		cv_PtrOfBNLLLayer_delete, cv_PtrOfBNLLLayer_get_inner_ptr, cv_PtrOfBNLLLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::BNLLLayer, cv_PtrOfBNLLLayer_new }
	
	impl core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] pub fn as_raw_PtrOfBNLLLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBNLLLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::BNLLLayerTraitConst for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_BNLLLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::BNLLLayerTrait for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_mut_BNLLLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackendNode = core::Ptr<crate::dnn::BackendNode>;
	
	ptr_extern! { crate::dnn::BackendNode,
		cv_PtrOfBackendNode_delete, cv_PtrOfBackendNode_get_inner_ptr, cv_PtrOfBackendNode_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::BackendNode, cv_PtrOfBackendNode_new }
	
	impl core::Ptr<crate::dnn::BackendNode> {
		#[inline] pub fn as_raw_PtrOfBackendNode(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackendNode(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::BackendNodeTraitConst for core::Ptr<crate::dnn::BackendNode> {
		#[inline] fn as_raw_BackendNode(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::BackendNodeTrait for core::Ptr<crate::dnn::BackendNode> {
		#[inline] fn as_raw_mut_BackendNode(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackendWrapper = core::Ptr<crate::dnn::BackendWrapper>;
	
	ptr_extern! { crate::dnn::BackendWrapper,
		cv_PtrOfBackendWrapper_delete, cv_PtrOfBackendWrapper_get_inner_ptr, cv_PtrOfBackendWrapper_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::dnn::BackendWrapper> {
		#[inline] pub fn as_raw_PtrOfBackendWrapper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackendWrapper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::BackendWrapperTraitConst for core::Ptr<crate::dnn::BackendWrapper> {
		#[inline] fn as_raw_BackendWrapper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::BackendWrapperTrait for core::Ptr<crate::dnn::BackendWrapper> {
		#[inline] fn as_raw_mut_BackendWrapper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBaseConvolutionLayer = core::Ptr<crate::dnn::BaseConvolutionLayer>;
	
	ptr_extern! { crate::dnn::BaseConvolutionLayer,
		cv_PtrOfBaseConvolutionLayer_delete, cv_PtrOfBaseConvolutionLayer_get_inner_ptr, cv_PtrOfBaseConvolutionLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::BaseConvolutionLayer, cv_PtrOfBaseConvolutionLayer_new }
	
	impl core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] pub fn as_raw_PtrOfBaseConvolutionLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBaseConvolutionLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::BaseConvolutionLayerTraitConst for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::BaseConvolutionLayerTrait for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBatchNormLayer = core::Ptr<crate::dnn::BatchNormLayer>;
	
	ptr_extern! { crate::dnn::BatchNormLayer,
		cv_PtrOfBatchNormLayer_delete, cv_PtrOfBatchNormLayer_get_inner_ptr, cv_PtrOfBatchNormLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::BatchNormLayer, cv_PtrOfBatchNormLayer_new }
	
	impl core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] pub fn as_raw_PtrOfBatchNormLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBatchNormLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::BatchNormLayerTraitConst for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_BatchNormLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::BatchNormLayerTrait for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBatchNormLayerInt8 = core::Ptr<crate::dnn::BatchNormLayerInt8>;
	
	ptr_extern! { crate::dnn::BatchNormLayerInt8,
		cv_PtrOfBatchNormLayerInt8_delete, cv_PtrOfBatchNormLayerInt8_get_inner_ptr, cv_PtrOfBatchNormLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::BatchNormLayerInt8, cv_PtrOfBatchNormLayerInt8_new }
	
	impl core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] pub fn as_raw_PtrOfBatchNormLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBatchNormLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::BatchNormLayerInt8TraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_BatchNormLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::BatchNormLayerInt8Trait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_mut_BatchNormLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::BatchNormLayerTraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_BatchNormLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::BatchNormLayerTrait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCeilLayer = core::Ptr<crate::dnn::CeilLayer>;
	
	ptr_extern! { crate::dnn::CeilLayer,
		cv_PtrOfCeilLayer_delete, cv_PtrOfCeilLayer_get_inner_ptr, cv_PtrOfCeilLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::CeilLayer, cv_PtrOfCeilLayer_new }
	
	impl core::Ptr<crate::dnn::CeilLayer> {
		#[inline] pub fn as_raw_PtrOfCeilLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCeilLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::CeilLayerTraitConst for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_CeilLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::CeilLayerTrait for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_mut_CeilLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCeluLayer = core::Ptr<crate::dnn::CeluLayer>;
	
	ptr_extern! { crate::dnn::CeluLayer,
		cv_PtrOfCeluLayer_delete, cv_PtrOfCeluLayer_get_inner_ptr, cv_PtrOfCeluLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::CeluLayer, cv_PtrOfCeluLayer_new }
	
	impl core::Ptr<crate::dnn::CeluLayer> {
		#[inline] pub fn as_raw_PtrOfCeluLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCeluLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::CeluLayerTraitConst for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_CeluLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::CeluLayerTrait for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_mut_CeluLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfConcatLayer = core::Ptr<crate::dnn::ConcatLayer>;
	
	ptr_extern! { crate::dnn::ConcatLayer,
		cv_PtrOfConcatLayer_delete, cv_PtrOfConcatLayer_get_inner_ptr, cv_PtrOfConcatLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ConcatLayer, cv_PtrOfConcatLayer_new }
	
	impl core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] pub fn as_raw_PtrOfConcatLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConcatLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ConcatLayerTraitConst for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_ConcatLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ConcatLayerTrait for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_mut_ConcatLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCorrelationLayer = core::Ptr<crate::dnn::CorrelationLayer>;
	
	ptr_extern! { crate::dnn::CorrelationLayer,
		cv_PtrOfCorrelationLayer_delete, cv_PtrOfCorrelationLayer_get_inner_ptr, cv_PtrOfCorrelationLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::CorrelationLayer, cv_PtrOfCorrelationLayer_new }
	
	impl core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] pub fn as_raw_PtrOfCorrelationLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCorrelationLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::CorrelationLayerTraitConst for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_CorrelationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::CorrelationLayerTrait for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_mut_CorrelationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCosLayer = core::Ptr<crate::dnn::CosLayer>;
	
	ptr_extern! { crate::dnn::CosLayer,
		cv_PtrOfCosLayer_delete, cv_PtrOfCosLayer_get_inner_ptr, cv_PtrOfCosLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::CosLayer, cv_PtrOfCosLayer_new }
	
	impl core::Ptr<crate::dnn::CosLayer> {
		#[inline] pub fn as_raw_PtrOfCosLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCosLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::CosLayerTraitConst for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_CosLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::CosLayerTrait for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_mut_CosLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCoshLayer = core::Ptr<crate::dnn::CoshLayer>;
	
	ptr_extern! { crate::dnn::CoshLayer,
		cv_PtrOfCoshLayer_delete, cv_PtrOfCoshLayer_get_inner_ptr, cv_PtrOfCoshLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::CoshLayer, cv_PtrOfCoshLayer_new }
	
	impl core::Ptr<crate::dnn::CoshLayer> {
		#[inline] pub fn as_raw_PtrOfCoshLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCoshLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::CoshLayerTraitConst for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_CoshLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::CoshLayerTrait for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_mut_CoshLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCumSumLayer = core::Ptr<crate::dnn::CumSumLayer>;
	
	ptr_extern! { crate::dnn::CumSumLayer,
		cv_PtrOfCumSumLayer_delete, cv_PtrOfCumSumLayer_get_inner_ptr, cv_PtrOfCumSumLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::CumSumLayer, cv_PtrOfCumSumLayer_new }
	
	impl core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] pub fn as_raw_PtrOfCumSumLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCumSumLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::CumSumLayerTraitConst for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_CumSumLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::CumSumLayerTrait for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_mut_CumSumLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDataAugmentationLayer = core::Ptr<crate::dnn::DataAugmentationLayer>;
	
	ptr_extern! { crate::dnn::DataAugmentationLayer,
		cv_PtrOfDataAugmentationLayer_delete, cv_PtrOfDataAugmentationLayer_get_inner_ptr, cv_PtrOfDataAugmentationLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::DataAugmentationLayer, cv_PtrOfDataAugmentationLayer_new }
	
	impl core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] pub fn as_raw_PtrOfDataAugmentationLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDataAugmentationLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::DataAugmentationLayerTraitConst for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_DataAugmentationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::DataAugmentationLayerTrait for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_mut_DataAugmentationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDequantizeLayer = core::Ptr<crate::dnn::DequantizeLayer>;
	
	ptr_extern! { crate::dnn::DequantizeLayer,
		cv_PtrOfDequantizeLayer_delete, cv_PtrOfDequantizeLayer_get_inner_ptr, cv_PtrOfDequantizeLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::DequantizeLayer, cv_PtrOfDequantizeLayer_new }
	
	impl core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] pub fn as_raw_PtrOfDequantizeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDequantizeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::DequantizeLayerTraitConst for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_DequantizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::DequantizeLayerTrait for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_mut_DequantizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetectionOutputLayer = core::Ptr<crate::dnn::DetectionOutputLayer>;
	
	ptr_extern! { crate::dnn::DetectionOutputLayer,
		cv_PtrOfDetectionOutputLayer_delete, cv_PtrOfDetectionOutputLayer_get_inner_ptr, cv_PtrOfDetectionOutputLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::DetectionOutputLayer, cv_PtrOfDetectionOutputLayer_new }
	
	impl core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] pub fn as_raw_PtrOfDetectionOutputLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetectionOutputLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::DetectionOutputLayerTraitConst for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_DetectionOutputLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::DetectionOutputLayerTrait for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_mut_DetectionOutputLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfELULayer = core::Ptr<crate::dnn::ELULayer>;
	
	ptr_extern! { crate::dnn::ELULayer,
		cv_PtrOfELULayer_delete, cv_PtrOfELULayer_get_inner_ptr, cv_PtrOfELULayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ELULayer, cv_PtrOfELULayer_new }
	
	impl core::Ptr<crate::dnn::ELULayer> {
		#[inline] pub fn as_raw_PtrOfELULayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfELULayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ELULayerTraitConst for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_ELULayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ELULayerTrait for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_mut_ELULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEltwiseLayer = core::Ptr<crate::dnn::EltwiseLayer>;
	
	ptr_extern! { crate::dnn::EltwiseLayer,
		cv_PtrOfEltwiseLayer_delete, cv_PtrOfEltwiseLayer_get_inner_ptr, cv_PtrOfEltwiseLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::EltwiseLayer, cv_PtrOfEltwiseLayer_new }
	
	impl core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] pub fn as_raw_PtrOfEltwiseLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEltwiseLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::EltwiseLayerTraitConst for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_EltwiseLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::EltwiseLayerTrait for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_mut_EltwiseLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEltwiseLayerInt8 = core::Ptr<crate::dnn::EltwiseLayerInt8>;
	
	ptr_extern! { crate::dnn::EltwiseLayerInt8,
		cv_PtrOfEltwiseLayerInt8_delete, cv_PtrOfEltwiseLayerInt8_get_inner_ptr, cv_PtrOfEltwiseLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::EltwiseLayerInt8, cv_PtrOfEltwiseLayerInt8_new }
	
	impl core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] pub fn as_raw_PtrOfEltwiseLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEltwiseLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::EltwiseLayerInt8TraitConst for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_EltwiseLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::EltwiseLayerInt8Trait for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_mut_EltwiseLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfErfLayer = core::Ptr<crate::dnn::ErfLayer>;
	
	ptr_extern! { crate::dnn::ErfLayer,
		cv_PtrOfErfLayer_delete, cv_PtrOfErfLayer_get_inner_ptr, cv_PtrOfErfLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ErfLayer, cv_PtrOfErfLayer_new }
	
	impl core::Ptr<crate::dnn::ErfLayer> {
		#[inline] pub fn as_raw_PtrOfErfLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfErfLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ErfLayerTraitConst for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_ErfLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ErfLayerTrait for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_mut_ErfLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfExpLayer = core::Ptr<crate::dnn::ExpLayer>;
	
	ptr_extern! { crate::dnn::ExpLayer,
		cv_PtrOfExpLayer_delete, cv_PtrOfExpLayer_get_inner_ptr, cv_PtrOfExpLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ExpLayer, cv_PtrOfExpLayer_new }
	
	impl core::Ptr<crate::dnn::ExpLayer> {
		#[inline] pub fn as_raw_PtrOfExpLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfExpLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ExpLayerTraitConst for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_ExpLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ExpLayerTrait for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_mut_ExpLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFlattenLayer = core::Ptr<crate::dnn::FlattenLayer>;
	
	ptr_extern! { crate::dnn::FlattenLayer,
		cv_PtrOfFlattenLayer_delete, cv_PtrOfFlattenLayer_get_inner_ptr, cv_PtrOfFlattenLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::FlattenLayer, cv_PtrOfFlattenLayer_new }
	
	impl core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] pub fn as_raw_PtrOfFlattenLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFlattenLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::FlattenLayerTraitConst for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_FlattenLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::FlattenLayerTrait for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_mut_FlattenLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFloorLayer = core::Ptr<crate::dnn::FloorLayer>;
	
	ptr_extern! { crate::dnn::FloorLayer,
		cv_PtrOfFloorLayer_delete, cv_PtrOfFloorLayer_get_inner_ptr, cv_PtrOfFloorLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::FloorLayer, cv_PtrOfFloorLayer_new }
	
	impl core::Ptr<crate::dnn::FloorLayer> {
		#[inline] pub fn as_raw_PtrOfFloorLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFloorLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::FloorLayerTraitConst for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_FloorLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::FloorLayerTrait for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_mut_FloorLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFlowWarpLayer = core::Ptr<crate::dnn::FlowWarpLayer>;
	
	ptr_extern! { crate::dnn::FlowWarpLayer,
		cv_PtrOfFlowWarpLayer_delete, cv_PtrOfFlowWarpLayer_get_inner_ptr, cv_PtrOfFlowWarpLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::FlowWarpLayer, cv_PtrOfFlowWarpLayer_new }
	
	impl core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] pub fn as_raw_PtrOfFlowWarpLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFlowWarpLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::FlowWarpLayerTraitConst for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_FlowWarpLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::FlowWarpLayerTrait for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_mut_FlowWarpLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGRULayer = core::Ptr<crate::dnn::GRULayer>;
	
	ptr_extern! { crate::dnn::GRULayer,
		cv_PtrOfGRULayer_delete, cv_PtrOfGRULayer_get_inner_ptr, cv_PtrOfGRULayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::GRULayer, cv_PtrOfGRULayer_new }
	
	impl core::Ptr<crate::dnn::GRULayer> {
		#[inline] pub fn as_raw_PtrOfGRULayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGRULayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::GRULayerTraitConst for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_GRULayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::GRULayerTrait for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_mut_GRULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGatherLayer = core::Ptr<crate::dnn::GatherLayer>;
	
	ptr_extern! { crate::dnn::GatherLayer,
		cv_PtrOfGatherLayer_delete, cv_PtrOfGatherLayer_get_inner_ptr, cv_PtrOfGatherLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::GatherLayer, cv_PtrOfGatherLayer_new }
	
	impl core::Ptr<crate::dnn::GatherLayer> {
		#[inline] pub fn as_raw_PtrOfGatherLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGatherLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::GatherLayerTraitConst for core::Ptr<crate::dnn::GatherLayer> {
		#[inline] fn as_raw_GatherLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::GatherLayerTrait for core::Ptr<crate::dnn::GatherLayer> {
		#[inline] fn as_raw_mut_GatherLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::GatherLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::GatherLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::GatherLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::GatherLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfHardSigmoidLayer = core::Ptr<crate::dnn::HardSigmoidLayer>;
	
	ptr_extern! { crate::dnn::HardSigmoidLayer,
		cv_PtrOfHardSigmoidLayer_delete, cv_PtrOfHardSigmoidLayer_get_inner_ptr, cv_PtrOfHardSigmoidLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::HardSigmoidLayer, cv_PtrOfHardSigmoidLayer_new }
	
	impl core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] pub fn as_raw_PtrOfHardSigmoidLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHardSigmoidLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::HardSigmoidLayerTraitConst for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_HardSigmoidLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::HardSigmoidLayerTrait for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_mut_HardSigmoidLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfHardSwishLayer = core::Ptr<crate::dnn::HardSwishLayer>;
	
	ptr_extern! { crate::dnn::HardSwishLayer,
		cv_PtrOfHardSwishLayer_delete, cv_PtrOfHardSwishLayer_get_inner_ptr, cv_PtrOfHardSwishLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::HardSwishLayer, cv_PtrOfHardSwishLayer_new }
	
	impl core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] pub fn as_raw_PtrOfHardSwishLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHardSwishLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::HardSwishLayerTraitConst for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_HardSwishLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::HardSwishLayerTrait for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_mut_HardSwishLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfInnerProductLayer = core::Ptr<crate::dnn::InnerProductLayer>;
	
	ptr_extern! { crate::dnn::InnerProductLayer,
		cv_PtrOfInnerProductLayer_delete, cv_PtrOfInnerProductLayer_get_inner_ptr, cv_PtrOfInnerProductLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::InnerProductLayer, cv_PtrOfInnerProductLayer_new }
	
	impl core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] pub fn as_raw_PtrOfInnerProductLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfInnerProductLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::InnerProductLayerTraitConst for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_InnerProductLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::InnerProductLayerTrait for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfInnerProductLayerInt8 = core::Ptr<crate::dnn::InnerProductLayerInt8>;
	
	ptr_extern! { crate::dnn::InnerProductLayerInt8,
		cv_PtrOfInnerProductLayerInt8_delete, cv_PtrOfInnerProductLayerInt8_get_inner_ptr, cv_PtrOfInnerProductLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::InnerProductLayerInt8, cv_PtrOfInnerProductLayerInt8_new }
	
	impl core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] pub fn as_raw_PtrOfInnerProductLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfInnerProductLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::InnerProductLayerInt8TraitConst for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_InnerProductLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::InnerProductLayerInt8Trait for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_mut_InnerProductLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::InnerProductLayerTraitConst for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_InnerProductLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::InnerProductLayerTrait for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLRNLayer = core::Ptr<crate::dnn::LRNLayer>;
	
	ptr_extern! { crate::dnn::LRNLayer,
		cv_PtrOfLRNLayer_delete, cv_PtrOfLRNLayer_get_inner_ptr, cv_PtrOfLRNLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::LRNLayer, cv_PtrOfLRNLayer_new }
	
	impl core::Ptr<crate::dnn::LRNLayer> {
		#[inline] pub fn as_raw_PtrOfLRNLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLRNLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::LRNLayerTraitConst for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_LRNLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LRNLayerTrait for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_mut_LRNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLSTMLayer = core::Ptr<crate::dnn::LSTMLayer>;
	
	ptr_extern! { crate::dnn::LSTMLayer,
		cv_PtrOfLSTMLayer_delete, cv_PtrOfLSTMLayer_get_inner_ptr, cv_PtrOfLSTMLayer_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::dnn::LSTMLayer> {
		#[inline] pub fn as_raw_PtrOfLSTMLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLSTMLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::LSTMLayerTraitConst for core::Ptr<crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_LSTMLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LSTMLayerTrait for core::Ptr<crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_mut_LSTMLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLayer = core::Ptr<crate::dnn::Layer>;
	
	ptr_extern! { crate::dnn::Layer,
		cv_PtrOfLayer_delete, cv_PtrOfLayer_get_inner_ptr, cv_PtrOfLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::Layer, cv_PtrOfLayer_new }
	
	impl core::Ptr<crate::dnn::Layer> {
		#[inline] pub fn as_raw_PtrOfLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::Layer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::Layer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::Layer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::Layer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLogLayer = core::Ptr<crate::dnn::LogLayer>;
	
	ptr_extern! { crate::dnn::LogLayer,
		cv_PtrOfLogLayer_delete, cv_PtrOfLogLayer_get_inner_ptr, cv_PtrOfLogLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::LogLayer, cv_PtrOfLogLayer_new }
	
	impl core::Ptr<crate::dnn::LogLayer> {
		#[inline] pub fn as_raw_PtrOfLogLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLogLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::LogLayerTraitConst for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_LogLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LogLayerTrait for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_mut_LogLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMVNLayer = core::Ptr<crate::dnn::MVNLayer>;
	
	ptr_extern! { crate::dnn::MVNLayer,
		cv_PtrOfMVNLayer_delete, cv_PtrOfMVNLayer_get_inner_ptr, cv_PtrOfMVNLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::MVNLayer, cv_PtrOfMVNLayer_new }
	
	impl core::Ptr<crate::dnn::MVNLayer> {
		#[inline] pub fn as_raw_PtrOfMVNLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMVNLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::MVNLayerTraitConst for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_MVNLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::MVNLayerTrait for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_mut_MVNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMaxUnpoolLayer = core::Ptr<crate::dnn::MaxUnpoolLayer>;
	
	ptr_extern! { crate::dnn::MaxUnpoolLayer,
		cv_PtrOfMaxUnpoolLayer_delete, cv_PtrOfMaxUnpoolLayer_get_inner_ptr, cv_PtrOfMaxUnpoolLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::MaxUnpoolLayer, cv_PtrOfMaxUnpoolLayer_new }
	
	impl core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] pub fn as_raw_PtrOfMaxUnpoolLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMaxUnpoolLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::MaxUnpoolLayerTraitConst for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_MaxUnpoolLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::MaxUnpoolLayerTrait for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_mut_MaxUnpoolLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMishLayer = core::Ptr<crate::dnn::MishLayer>;
	
	ptr_extern! { crate::dnn::MishLayer,
		cv_PtrOfMishLayer_delete, cv_PtrOfMishLayer_get_inner_ptr, cv_PtrOfMishLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::MishLayer, cv_PtrOfMishLayer_new }
	
	impl core::Ptr<crate::dnn::MishLayer> {
		#[inline] pub fn as_raw_PtrOfMishLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMishLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::MishLayerTraitConst for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_MishLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::MishLayerTrait for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_mut_MishLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfNaryEltwiseLayer = core::Ptr<crate::dnn::NaryEltwiseLayer>;
	
	ptr_extern! { crate::dnn::NaryEltwiseLayer,
		cv_PtrOfNaryEltwiseLayer_delete, cv_PtrOfNaryEltwiseLayer_get_inner_ptr, cv_PtrOfNaryEltwiseLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::NaryEltwiseLayer, cv_PtrOfNaryEltwiseLayer_new }
	
	impl core::Ptr<crate::dnn::NaryEltwiseLayer> {
		#[inline] pub fn as_raw_PtrOfNaryEltwiseLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNaryEltwiseLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::NaryEltwiseLayerTraitConst for core::Ptr<crate::dnn::NaryEltwiseLayer> {
		#[inline] fn as_raw_NaryEltwiseLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::NaryEltwiseLayerTrait for core::Ptr<crate::dnn::NaryEltwiseLayer> {
		#[inline] fn as_raw_mut_NaryEltwiseLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::NaryEltwiseLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::NaryEltwiseLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::NaryEltwiseLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::NaryEltwiseLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfNormalizeBBoxLayer = core::Ptr<crate::dnn::NormalizeBBoxLayer>;
	
	ptr_extern! { crate::dnn::NormalizeBBoxLayer,
		cv_PtrOfNormalizeBBoxLayer_delete, cv_PtrOfNormalizeBBoxLayer_get_inner_ptr, cv_PtrOfNormalizeBBoxLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::NormalizeBBoxLayer, cv_PtrOfNormalizeBBoxLayer_new }
	
	impl core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] pub fn as_raw_PtrOfNormalizeBBoxLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNormalizeBBoxLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::NormalizeBBoxLayerTraitConst for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_NormalizeBBoxLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::NormalizeBBoxLayerTrait for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_mut_NormalizeBBoxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfNotLayer = core::Ptr<crate::dnn::NotLayer>;
	
	ptr_extern! { crate::dnn::NotLayer,
		cv_PtrOfNotLayer_delete, cv_PtrOfNotLayer_get_inner_ptr, cv_PtrOfNotLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::NotLayer, cv_PtrOfNotLayer_new }
	
	impl core::Ptr<crate::dnn::NotLayer> {
		#[inline] pub fn as_raw_PtrOfNotLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNotLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::NotLayerTraitConst for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_NotLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::NotLayerTrait for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_mut_NotLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPaddingLayer = core::Ptr<crate::dnn::PaddingLayer>;
	
	ptr_extern! { crate::dnn::PaddingLayer,
		cv_PtrOfPaddingLayer_delete, cv_PtrOfPaddingLayer_get_inner_ptr, cv_PtrOfPaddingLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::PaddingLayer, cv_PtrOfPaddingLayer_new }
	
	impl core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] pub fn as_raw_PtrOfPaddingLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPaddingLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::PaddingLayerTraitConst for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_PaddingLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::PaddingLayerTrait for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_mut_PaddingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPermuteLayer = core::Ptr<crate::dnn::PermuteLayer>;
	
	ptr_extern! { crate::dnn::PermuteLayer,
		cv_PtrOfPermuteLayer_delete, cv_PtrOfPermuteLayer_get_inner_ptr, cv_PtrOfPermuteLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::PermuteLayer, cv_PtrOfPermuteLayer_new }
	
	impl core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] pub fn as_raw_PtrOfPermuteLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPermuteLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::PermuteLayerTraitConst for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_PermuteLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::PermuteLayerTrait for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_mut_PermuteLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPoolingLayer = core::Ptr<crate::dnn::PoolingLayer>;
	
	ptr_extern! { crate::dnn::PoolingLayer,
		cv_PtrOfPoolingLayer_delete, cv_PtrOfPoolingLayer_get_inner_ptr, cv_PtrOfPoolingLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::PoolingLayer, cv_PtrOfPoolingLayer_new }
	
	impl core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] pub fn as_raw_PtrOfPoolingLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPoolingLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::PoolingLayerTraitConst for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_PoolingLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::PoolingLayerTrait for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPoolingLayerInt8 = core::Ptr<crate::dnn::PoolingLayerInt8>;
	
	ptr_extern! { crate::dnn::PoolingLayerInt8,
		cv_PtrOfPoolingLayerInt8_delete, cv_PtrOfPoolingLayerInt8_get_inner_ptr, cv_PtrOfPoolingLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::PoolingLayerInt8, cv_PtrOfPoolingLayerInt8_new }
	
	impl core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] pub fn as_raw_PtrOfPoolingLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPoolingLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::PoolingLayerInt8TraitConst for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_PoolingLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::PoolingLayerInt8Trait for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_mut_PoolingLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::PoolingLayerTraitConst for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_PoolingLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::PoolingLayerTrait for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPowerLayer = core::Ptr<crate::dnn::PowerLayer>;
	
	ptr_extern! { crate::dnn::PowerLayer,
		cv_PtrOfPowerLayer_delete, cv_PtrOfPowerLayer_get_inner_ptr, cv_PtrOfPowerLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::PowerLayer, cv_PtrOfPowerLayer_new }
	
	impl core::Ptr<crate::dnn::PowerLayer> {
		#[inline] pub fn as_raw_PtrOfPowerLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPowerLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::PowerLayerTraitConst for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_PowerLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::PowerLayerTrait for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_mut_PowerLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPriorBoxLayer = core::Ptr<crate::dnn::PriorBoxLayer>;
	
	ptr_extern! { crate::dnn::PriorBoxLayer,
		cv_PtrOfPriorBoxLayer_delete, cv_PtrOfPriorBoxLayer_get_inner_ptr, cv_PtrOfPriorBoxLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::PriorBoxLayer, cv_PtrOfPriorBoxLayer_new }
	
	impl core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] pub fn as_raw_PtrOfPriorBoxLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPriorBoxLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::PriorBoxLayerTraitConst for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_PriorBoxLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::PriorBoxLayerTrait for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_mut_PriorBoxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfProposalLayer = core::Ptr<crate::dnn::ProposalLayer>;
	
	ptr_extern! { crate::dnn::ProposalLayer,
		cv_PtrOfProposalLayer_delete, cv_PtrOfProposalLayer_get_inner_ptr, cv_PtrOfProposalLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ProposalLayer, cv_PtrOfProposalLayer_new }
	
	impl core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] pub fn as_raw_PtrOfProposalLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfProposalLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ProposalLayerTraitConst for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_ProposalLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ProposalLayerTrait for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_mut_ProposalLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfQuantizeLayer = core::Ptr<crate::dnn::QuantizeLayer>;
	
	ptr_extern! { crate::dnn::QuantizeLayer,
		cv_PtrOfQuantizeLayer_delete, cv_PtrOfQuantizeLayer_get_inner_ptr, cv_PtrOfQuantizeLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::QuantizeLayer, cv_PtrOfQuantizeLayer_new }
	
	impl core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] pub fn as_raw_PtrOfQuantizeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfQuantizeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::QuantizeLayerTraitConst for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_QuantizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::QuantizeLayerTrait for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_mut_QuantizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRNNLayer = core::Ptr<crate::dnn::RNNLayer>;
	
	ptr_extern! { crate::dnn::RNNLayer,
		cv_PtrOfRNNLayer_delete, cv_PtrOfRNNLayer_get_inner_ptr, cv_PtrOfRNNLayer_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::dnn::RNNLayer> {
		#[inline] pub fn as_raw_PtrOfRNNLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRNNLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::RNNLayerTraitConst for core::Ptr<crate::dnn::RNNLayer> {
		#[inline] fn as_raw_RNNLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::RNNLayerTrait for core::Ptr<crate::dnn::RNNLayer> {
		#[inline] fn as_raw_mut_RNNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::RNNLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::RNNLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::RNNLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::RNNLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReLU6Layer = core::Ptr<crate::dnn::ReLU6Layer>;
	
	ptr_extern! { crate::dnn::ReLU6Layer,
		cv_PtrOfReLU6Layer_delete, cv_PtrOfReLU6Layer_get_inner_ptr, cv_PtrOfReLU6Layer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReLU6Layer, cv_PtrOfReLU6Layer_new }
	
	impl core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] pub fn as_raw_PtrOfReLU6Layer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReLU6Layer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReLU6LayerTraitConst for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_ReLU6Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReLU6LayerTrait for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_mut_ReLU6Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReLULayer = core::Ptr<crate::dnn::ReLULayer>;
	
	ptr_extern! { crate::dnn::ReLULayer,
		cv_PtrOfReLULayer_delete, cv_PtrOfReLULayer_get_inner_ptr, cv_PtrOfReLULayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReLULayer, cv_PtrOfReLULayer_new }
	
	impl core::Ptr<crate::dnn::ReLULayer> {
		#[inline] pub fn as_raw_PtrOfReLULayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReLULayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReLULayerTraitConst for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_ReLULayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReLULayerTrait for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_mut_ReLULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReciprocalLayer = core::Ptr<crate::dnn::ReciprocalLayer>;
	
	ptr_extern! { crate::dnn::ReciprocalLayer,
		cv_PtrOfReciprocalLayer_delete, cv_PtrOfReciprocalLayer_get_inner_ptr, cv_PtrOfReciprocalLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReciprocalLayer, cv_PtrOfReciprocalLayer_new }
	
	impl core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] pub fn as_raw_PtrOfReciprocalLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReciprocalLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReciprocalLayerTraitConst for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_ReciprocalLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReciprocalLayerTrait for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_mut_ReciprocalLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReduceLayer = core::Ptr<crate::dnn::ReduceLayer>;
	
	ptr_extern! { crate::dnn::ReduceLayer,
		cv_PtrOfReduceLayer_delete, cv_PtrOfReduceLayer_get_inner_ptr, cv_PtrOfReduceLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReduceLayer, cv_PtrOfReduceLayer_new }
	
	impl core::Ptr<crate::dnn::ReduceLayer> {
		#[inline] pub fn as_raw_PtrOfReduceLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReduceLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReduceLayerTraitConst for core::Ptr<crate::dnn::ReduceLayer> {
		#[inline] fn as_raw_ReduceLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReduceLayerTrait for core::Ptr<crate::dnn::ReduceLayer> {
		#[inline] fn as_raw_mut_ReduceLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReduceLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReduceLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReduceLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReduceLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReduceLayerInt8 = core::Ptr<crate::dnn::ReduceLayerInt8>;
	
	ptr_extern! { crate::dnn::ReduceLayerInt8,
		cv_PtrOfReduceLayerInt8_delete, cv_PtrOfReduceLayerInt8_get_inner_ptr, cv_PtrOfReduceLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReduceLayerInt8, cv_PtrOfReduceLayerInt8_new }
	
	impl core::Ptr<crate::dnn::ReduceLayerInt8> {
		#[inline] pub fn as_raw_PtrOfReduceLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReduceLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReduceLayerInt8TraitConst for core::Ptr<crate::dnn::ReduceLayerInt8> {
		#[inline] fn as_raw_ReduceLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReduceLayerInt8Trait for core::Ptr<crate::dnn::ReduceLayerInt8> {
		#[inline] fn as_raw_mut_ReduceLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReduceLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReduceLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReduceLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReduceLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ReduceLayerTraitConst for core::Ptr<crate::dnn::ReduceLayerInt8> {
		#[inline] fn as_raw_ReduceLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReduceLayerTrait for core::Ptr<crate::dnn::ReduceLayerInt8> {
		#[inline] fn as_raw_mut_ReduceLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRegionLayer = core::Ptr<crate::dnn::RegionLayer>;
	
	ptr_extern! { crate::dnn::RegionLayer,
		cv_PtrOfRegionLayer_delete, cv_PtrOfRegionLayer_get_inner_ptr, cv_PtrOfRegionLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::RegionLayer, cv_PtrOfRegionLayer_new }
	
	impl core::Ptr<crate::dnn::RegionLayer> {
		#[inline] pub fn as_raw_PtrOfRegionLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRegionLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::RegionLayerTraitConst for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_RegionLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::RegionLayerTrait for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_mut_RegionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReorgLayer = core::Ptr<crate::dnn::ReorgLayer>;
	
	ptr_extern! { crate::dnn::ReorgLayer,
		cv_PtrOfReorgLayer_delete, cv_PtrOfReorgLayer_get_inner_ptr, cv_PtrOfReorgLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReorgLayer, cv_PtrOfReorgLayer_new }
	
	impl core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] pub fn as_raw_PtrOfReorgLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReorgLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReorgLayerTraitConst for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_ReorgLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReorgLayerTrait for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_mut_ReorgLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRequantizeLayer = core::Ptr<crate::dnn::RequantizeLayer>;
	
	ptr_extern! { crate::dnn::RequantizeLayer,
		cv_PtrOfRequantizeLayer_delete, cv_PtrOfRequantizeLayer_get_inner_ptr, cv_PtrOfRequantizeLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::RequantizeLayer, cv_PtrOfRequantizeLayer_new }
	
	impl core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] pub fn as_raw_PtrOfRequantizeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRequantizeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::RequantizeLayerTraitConst for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_RequantizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::RequantizeLayerTrait for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_mut_RequantizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfReshapeLayer = core::Ptr<crate::dnn::ReshapeLayer>;
	
	ptr_extern! { crate::dnn::ReshapeLayer,
		cv_PtrOfReshapeLayer_delete, cv_PtrOfReshapeLayer_get_inner_ptr, cv_PtrOfReshapeLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ReshapeLayer, cv_PtrOfReshapeLayer_new }
	
	impl core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] pub fn as_raw_PtrOfReshapeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReshapeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ReshapeLayerTraitConst for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_ReshapeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ReshapeLayerTrait for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_mut_ReshapeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfResizeLayer = core::Ptr<crate::dnn::ResizeLayer>;
	
	ptr_extern! { crate::dnn::ResizeLayer,
		cv_PtrOfResizeLayer_delete, cv_PtrOfResizeLayer_get_inner_ptr, cv_PtrOfResizeLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ResizeLayer, cv_PtrOfResizeLayer_new }
	
	impl core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] pub fn as_raw_PtrOfResizeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfResizeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ResizeLayerTraitConst for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_ResizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ResizeLayerTrait for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_mut_ResizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRoundLayer = core::Ptr<crate::dnn::RoundLayer>;
	
	ptr_extern! { crate::dnn::RoundLayer,
		cv_PtrOfRoundLayer_delete, cv_PtrOfRoundLayer_get_inner_ptr, cv_PtrOfRoundLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::RoundLayer, cv_PtrOfRoundLayer_new }
	
	impl core::Ptr<crate::dnn::RoundLayer> {
		#[inline] pub fn as_raw_PtrOfRoundLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRoundLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::RoundLayerTraitConst for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_RoundLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::RoundLayerTrait for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_mut_RoundLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfScaleLayer = core::Ptr<crate::dnn::ScaleLayer>;
	
	ptr_extern! { crate::dnn::ScaleLayer,
		cv_PtrOfScaleLayer_delete, cv_PtrOfScaleLayer_get_inner_ptr, cv_PtrOfScaleLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ScaleLayer, cv_PtrOfScaleLayer_new }
	
	impl core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] pub fn as_raw_PtrOfScaleLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfScaleLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ScaleLayerTraitConst for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_ScaleLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ScaleLayerTrait for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfScaleLayerInt8 = core::Ptr<crate::dnn::ScaleLayerInt8>;
	
	ptr_extern! { crate::dnn::ScaleLayerInt8,
		cv_PtrOfScaleLayerInt8_delete, cv_PtrOfScaleLayerInt8_get_inner_ptr, cv_PtrOfScaleLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ScaleLayerInt8, cv_PtrOfScaleLayerInt8_new }
	
	impl core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] pub fn as_raw_PtrOfScaleLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfScaleLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ScaleLayerInt8TraitConst for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_ScaleLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ScaleLayerInt8Trait for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_mut_ScaleLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ScaleLayerTraitConst for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_ScaleLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ScaleLayerTrait for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfScatterLayer = core::Ptr<crate::dnn::ScatterLayer>;
	
	ptr_extern! { crate::dnn::ScatterLayer,
		cv_PtrOfScatterLayer_delete, cv_PtrOfScatterLayer_get_inner_ptr, cv_PtrOfScatterLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ScatterLayer, cv_PtrOfScatterLayer_new }
	
	impl core::Ptr<crate::dnn::ScatterLayer> {
		#[inline] pub fn as_raw_PtrOfScatterLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfScatterLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ScatterLayerTraitConst for core::Ptr<crate::dnn::ScatterLayer> {
		#[inline] fn as_raw_ScatterLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ScatterLayerTrait for core::Ptr<crate::dnn::ScatterLayer> {
		#[inline] fn as_raw_mut_ScatterLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ScatterLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ScatterLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ScatterLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ScatterLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfScatterNDLayer = core::Ptr<crate::dnn::ScatterNDLayer>;
	
	ptr_extern! { crate::dnn::ScatterNDLayer,
		cv_PtrOfScatterNDLayer_delete, cv_PtrOfScatterNDLayer_get_inner_ptr, cv_PtrOfScatterNDLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ScatterNDLayer, cv_PtrOfScatterNDLayer_new }
	
	impl core::Ptr<crate::dnn::ScatterNDLayer> {
		#[inline] pub fn as_raw_PtrOfScatterNDLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfScatterNDLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ScatterNDLayerTraitConst for core::Ptr<crate::dnn::ScatterNDLayer> {
		#[inline] fn as_raw_ScatterNDLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ScatterNDLayerTrait for core::Ptr<crate::dnn::ScatterNDLayer> {
		#[inline] fn as_raw_mut_ScatterNDLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ScatterNDLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ScatterNDLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ScatterNDLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ScatterNDLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSeluLayer = core::Ptr<crate::dnn::SeluLayer>;
	
	ptr_extern! { crate::dnn::SeluLayer,
		cv_PtrOfSeluLayer_delete, cv_PtrOfSeluLayer_get_inner_ptr, cv_PtrOfSeluLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SeluLayer, cv_PtrOfSeluLayer_new }
	
	impl core::Ptr<crate::dnn::SeluLayer> {
		#[inline] pub fn as_raw_PtrOfSeluLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSeluLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SeluLayerTraitConst for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_SeluLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SeluLayerTrait for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_mut_SeluLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfShrinkLayer = core::Ptr<crate::dnn::ShrinkLayer>;
	
	ptr_extern! { crate::dnn::ShrinkLayer,
		cv_PtrOfShrinkLayer_delete, cv_PtrOfShrinkLayer_get_inner_ptr, cv_PtrOfShrinkLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ShrinkLayer, cv_PtrOfShrinkLayer_new }
	
	impl core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] pub fn as_raw_PtrOfShrinkLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfShrinkLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ShrinkLayerTraitConst for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_ShrinkLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ShrinkLayerTrait for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_mut_ShrinkLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSigmoidLayer = core::Ptr<crate::dnn::SigmoidLayer>;
	
	ptr_extern! { crate::dnn::SigmoidLayer,
		cv_PtrOfSigmoidLayer_delete, cv_PtrOfSigmoidLayer_get_inner_ptr, cv_PtrOfSigmoidLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SigmoidLayer, cv_PtrOfSigmoidLayer_new }
	
	impl core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] pub fn as_raw_PtrOfSigmoidLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSigmoidLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SigmoidLayerTraitConst for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_SigmoidLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SigmoidLayerTrait for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_mut_SigmoidLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSignLayer = core::Ptr<crate::dnn::SignLayer>;
	
	ptr_extern! { crate::dnn::SignLayer,
		cv_PtrOfSignLayer_delete, cv_PtrOfSignLayer_get_inner_ptr, cv_PtrOfSignLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SignLayer, cv_PtrOfSignLayer_new }
	
	impl core::Ptr<crate::dnn::SignLayer> {
		#[inline] pub fn as_raw_PtrOfSignLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSignLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SignLayerTraitConst for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_SignLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SignLayerTrait for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_mut_SignLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSinLayer = core::Ptr<crate::dnn::SinLayer>;
	
	ptr_extern! { crate::dnn::SinLayer,
		cv_PtrOfSinLayer_delete, cv_PtrOfSinLayer_get_inner_ptr, cv_PtrOfSinLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SinLayer, cv_PtrOfSinLayer_new }
	
	impl core::Ptr<crate::dnn::SinLayer> {
		#[inline] pub fn as_raw_PtrOfSinLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSinLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SinLayerTraitConst for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_SinLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SinLayerTrait for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_mut_SinLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSinhLayer = core::Ptr<crate::dnn::SinhLayer>;
	
	ptr_extern! { crate::dnn::SinhLayer,
		cv_PtrOfSinhLayer_delete, cv_PtrOfSinhLayer_get_inner_ptr, cv_PtrOfSinhLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SinhLayer, cv_PtrOfSinhLayer_new }
	
	impl core::Ptr<crate::dnn::SinhLayer> {
		#[inline] pub fn as_raw_PtrOfSinhLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSinhLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SinhLayerTraitConst for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_SinhLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SinhLayerTrait for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_mut_SinhLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSliceLayer = core::Ptr<crate::dnn::SliceLayer>;
	
	ptr_extern! { crate::dnn::SliceLayer,
		cv_PtrOfSliceLayer_delete, cv_PtrOfSliceLayer_get_inner_ptr, cv_PtrOfSliceLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SliceLayer, cv_PtrOfSliceLayer_new }
	
	impl core::Ptr<crate::dnn::SliceLayer> {
		#[inline] pub fn as_raw_PtrOfSliceLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSliceLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SliceLayerTraitConst for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_SliceLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SliceLayerTrait for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_mut_SliceLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSoftmaxLayer = core::Ptr<crate::dnn::SoftmaxLayer>;
	
	ptr_extern! { crate::dnn::SoftmaxLayer,
		cv_PtrOfSoftmaxLayer_delete, cv_PtrOfSoftmaxLayer_get_inner_ptr, cv_PtrOfSoftmaxLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SoftmaxLayer, cv_PtrOfSoftmaxLayer_new }
	
	impl core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] pub fn as_raw_PtrOfSoftmaxLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSoftmaxLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SoftmaxLayerTraitConst for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_SoftmaxLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SoftmaxLayerTrait for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSoftmaxLayerInt8 = core::Ptr<crate::dnn::SoftmaxLayerInt8>;
	
	ptr_extern! { crate::dnn::SoftmaxLayerInt8,
		cv_PtrOfSoftmaxLayerInt8_delete, cv_PtrOfSoftmaxLayerInt8_get_inner_ptr, cv_PtrOfSoftmaxLayerInt8_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SoftmaxLayerInt8, cv_PtrOfSoftmaxLayerInt8_new }
	
	impl core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] pub fn as_raw_PtrOfSoftmaxLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSoftmaxLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SoftmaxLayerInt8TraitConst for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_SoftmaxLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SoftmaxLayerInt8Trait for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_mut_SoftmaxLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::SoftmaxLayerTraitConst for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_SoftmaxLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SoftmaxLayerTrait for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSoftplusLayer = core::Ptr<crate::dnn::SoftplusLayer>;
	
	ptr_extern! { crate::dnn::SoftplusLayer,
		cv_PtrOfSoftplusLayer_delete, cv_PtrOfSoftplusLayer_get_inner_ptr, cv_PtrOfSoftplusLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SoftplusLayer, cv_PtrOfSoftplusLayer_new }
	
	impl core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] pub fn as_raw_PtrOfSoftplusLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSoftplusLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SoftplusLayerTraitConst for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_SoftplusLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SoftplusLayerTrait for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_mut_SoftplusLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSoftsignLayer = core::Ptr<crate::dnn::SoftsignLayer>;
	
	ptr_extern! { crate::dnn::SoftsignLayer,
		cv_PtrOfSoftsignLayer_delete, cv_PtrOfSoftsignLayer_get_inner_ptr, cv_PtrOfSoftsignLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SoftsignLayer, cv_PtrOfSoftsignLayer_new }
	
	impl core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] pub fn as_raw_PtrOfSoftsignLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSoftsignLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SoftsignLayerTraitConst for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_SoftsignLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SoftsignLayerTrait for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_mut_SoftsignLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSplitLayer = core::Ptr<crate::dnn::SplitLayer>;
	
	ptr_extern! { crate::dnn::SplitLayer,
		cv_PtrOfSplitLayer_delete, cv_PtrOfSplitLayer_get_inner_ptr, cv_PtrOfSplitLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SplitLayer, cv_PtrOfSplitLayer_new }
	
	impl core::Ptr<crate::dnn::SplitLayer> {
		#[inline] pub fn as_raw_PtrOfSplitLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSplitLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SplitLayerTraitConst for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_SplitLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SplitLayerTrait for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_mut_SplitLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSqrtLayer = core::Ptr<crate::dnn::SqrtLayer>;
	
	ptr_extern! { crate::dnn::SqrtLayer,
		cv_PtrOfSqrtLayer_delete, cv_PtrOfSqrtLayer_get_inner_ptr, cv_PtrOfSqrtLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SqrtLayer, cv_PtrOfSqrtLayer_new }
	
	impl core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] pub fn as_raw_PtrOfSqrtLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSqrtLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SqrtLayerTraitConst for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_SqrtLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SqrtLayerTrait for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_mut_SqrtLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSwishLayer = core::Ptr<crate::dnn::SwishLayer>;
	
	ptr_extern! { crate::dnn::SwishLayer,
		cv_PtrOfSwishLayer_delete, cv_PtrOfSwishLayer_get_inner_ptr, cv_PtrOfSwishLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::SwishLayer, cv_PtrOfSwishLayer_new }
	
	impl core::Ptr<crate::dnn::SwishLayer> {
		#[inline] pub fn as_raw_PtrOfSwishLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSwishLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::SwishLayerTraitConst for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_SwishLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::SwishLayerTrait for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_mut_SwishLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTanHLayer = core::Ptr<crate::dnn::TanHLayer>;
	
	ptr_extern! { crate::dnn::TanHLayer,
		cv_PtrOfTanHLayer_delete, cv_PtrOfTanHLayer_get_inner_ptr, cv_PtrOfTanHLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::TanHLayer, cv_PtrOfTanHLayer_new }
	
	impl core::Ptr<crate::dnn::TanHLayer> {
		#[inline] pub fn as_raw_PtrOfTanHLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTanHLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::TanHLayerTraitConst for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_TanHLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::TanHLayerTrait for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_mut_TanHLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTanLayer = core::Ptr<crate::dnn::TanLayer>;
	
	ptr_extern! { crate::dnn::TanLayer,
		cv_PtrOfTanLayer_delete, cv_PtrOfTanLayer_get_inner_ptr, cv_PtrOfTanLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::TanLayer, cv_PtrOfTanLayer_new }
	
	impl core::Ptr<crate::dnn::TanLayer> {
		#[inline] pub fn as_raw_PtrOfTanLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTanLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::TanLayerTraitConst for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_TanLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::TanLayerTrait for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_mut_TanLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfThresholdedReluLayer = core::Ptr<crate::dnn::ThresholdedReluLayer>;
	
	ptr_extern! { crate::dnn::ThresholdedReluLayer,
		cv_PtrOfThresholdedReluLayer_delete, cv_PtrOfThresholdedReluLayer_get_inner_ptr, cv_PtrOfThresholdedReluLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::ThresholdedReluLayer, cv_PtrOfThresholdedReluLayer_new }
	
	impl core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] pub fn as_raw_PtrOfThresholdedReluLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfThresholdedReluLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::ThresholdedReluLayerTraitConst for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_ThresholdedReluLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ThresholdedReluLayerTrait for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_mut_ThresholdedReluLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTileLayer = core::Ptr<crate::dnn::TileLayer>;
	
	ptr_extern! { crate::dnn::TileLayer,
		cv_PtrOfTileLayer_delete, cv_PtrOfTileLayer_get_inner_ptr, cv_PtrOfTileLayer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::dnn::TileLayer, cv_PtrOfTileLayer_new }
	
	impl core::Ptr<crate::dnn::TileLayer> {
		#[inline] pub fn as_raw_PtrOfTileLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTileLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn::TileLayerTraitConst for core::Ptr<crate::dnn::TileLayer> {
		#[inline] fn as_raw_TileLayer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::TileLayerTrait for core::Ptr<crate::dnn::TileLayer> {
		#[inline] fn as_raw_mut_TileLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::TileLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::dnn::TileLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::TileLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::TileLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type TupleOfBackend_Target = core::Tuple<(crate::dnn::Backend, crate::dnn::Target)>;
	
	impl core::Tuple<(crate::dnn::Backend, crate::dnn::Target)> {
		pub fn as_raw_TupleOfBackend_Target(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfBackend_Target(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	tuple_extern! { (crate::dnn::Backend, crate::dnn::Target),
		cv_TupleOfBackend_Target_new, cv_TupleOfBackend_Target_delete,
		0 = arg: crate::dnn::Backend, get_0 via cv_TupleOfBackend_Target_get_0,
		1 = arg_1: crate::dnn::Target, get_1 via cv_TupleOfBackend_Target_get_1
	}
	
	pub type VectorOfMatShape = core::Vector<crate::dnn::MatShape>;
	
	impl core::Vector<crate::dnn::MatShape> {
		pub fn as_raw_VectorOfMatShape(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfMatShape(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	pub type VectorOfPtrOfBackendNode = core::Vector<core::Ptr<crate::dnn::BackendNode>>;
	
	impl core::Vector<core::Ptr<crate::dnn::BackendNode>> {
		pub fn as_raw_VectorOfPtrOfBackendNode(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfBackendNode(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Ptr<crate::dnn::BackendNode>,
		cv_VectorOfPtrOfBackendNode_new, cv_VectorOfPtrOfBackendNode_delete,
		cv_VectorOfPtrOfBackendNode_len, cv_VectorOfPtrOfBackendNode_is_empty,
		cv_VectorOfPtrOfBackendNode_capacity, cv_VectorOfPtrOfBackendNode_shrink_to_fit,
		cv_VectorOfPtrOfBackendNode_reserve, cv_VectorOfPtrOfBackendNode_remove,
		cv_VectorOfPtrOfBackendNode_swap, cv_VectorOfPtrOfBackendNode_clear,
		cv_VectorOfPtrOfBackendNode_get, cv_VectorOfPtrOfBackendNode_set,
		cv_VectorOfPtrOfBackendNode_push, cv_VectorOfPtrOfBackendNode_insert,
	}
	vector_non_copy_or_bool! { core::Ptr<crate::dnn::BackendNode> }
	
	pub type VectorOfPtrOfBackendWrapper = core::Vector<core::Ptr<crate::dnn::BackendWrapper>>;
	
	impl core::Vector<core::Ptr<crate::dnn::BackendWrapper>> {
		pub fn as_raw_VectorOfPtrOfBackendWrapper(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfBackendWrapper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Ptr<crate::dnn::BackendWrapper>,
		cv_VectorOfPtrOfBackendWrapper_new, cv_VectorOfPtrOfBackendWrapper_delete,
		cv_VectorOfPtrOfBackendWrapper_len, cv_VectorOfPtrOfBackendWrapper_is_empty,
		cv_VectorOfPtrOfBackendWrapper_capacity, cv_VectorOfPtrOfBackendWrapper_shrink_to_fit,
		cv_VectorOfPtrOfBackendWrapper_reserve, cv_VectorOfPtrOfBackendWrapper_remove,
		cv_VectorOfPtrOfBackendWrapper_swap, cv_VectorOfPtrOfBackendWrapper_clear,
		cv_VectorOfPtrOfBackendWrapper_get, cv_VectorOfPtrOfBackendWrapper_set,
		cv_VectorOfPtrOfBackendWrapper_push, cv_VectorOfPtrOfBackendWrapper_insert,
	}
	vector_non_copy_or_bool! { core::Ptr<crate::dnn::BackendWrapper> }
	
	pub type VectorOfPtrOfLayer = core::Vector<core::Ptr<crate::dnn::Layer>>;
	
	impl core::Vector<core::Ptr<crate::dnn::Layer>> {
		pub fn as_raw_VectorOfPtrOfLayer(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Ptr<crate::dnn::Layer>,
		cv_VectorOfPtrOfLayer_new, cv_VectorOfPtrOfLayer_delete,
		cv_VectorOfPtrOfLayer_len, cv_VectorOfPtrOfLayer_is_empty,
		cv_VectorOfPtrOfLayer_capacity, cv_VectorOfPtrOfLayer_shrink_to_fit,
		cv_VectorOfPtrOfLayer_reserve, cv_VectorOfPtrOfLayer_remove,
		cv_VectorOfPtrOfLayer_swap, cv_VectorOfPtrOfLayer_clear,
		cv_VectorOfPtrOfLayer_get, cv_VectorOfPtrOfLayer_set,
		cv_VectorOfPtrOfLayer_push, cv_VectorOfPtrOfLayer_insert,
	}
	vector_non_copy_or_bool! { core::Ptr<crate::dnn::Layer> }
	
	pub type VectorOfTarget = core::Vector<crate::dnn::Target>;
	
	impl core::Vector<crate::dnn::Target> {
		pub fn as_raw_VectorOfTarget(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfTarget(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::dnn::Target,
		cv_VectorOfTarget_new, cv_VectorOfTarget_delete,
		cv_VectorOfTarget_len, cv_VectorOfTarget_is_empty,
		cv_VectorOfTarget_capacity, cv_VectorOfTarget_shrink_to_fit,
		cv_VectorOfTarget_reserve, cv_VectorOfTarget_remove,
		cv_VectorOfTarget_swap, cv_VectorOfTarget_clear,
		cv_VectorOfTarget_get, cv_VectorOfTarget_set,
		cv_VectorOfTarget_push, cv_VectorOfTarget_insert,
	}
	vector_copy_non_bool! { crate::dnn::Target,
		cv_VectorOfTarget_data, cv_VectorOfTarget_data_mut, cv_VectorOfTarget_from_slice,
		cv_VectorOfTarget_clone,
	}
	
	pub type VectorOfTupleOfBackend_Target = core::Vector<core::Tuple<(crate::dnn::Backend, crate::dnn::Target)>>;
	
	impl core::Vector<core::Tuple<(crate::dnn::Backend, crate::dnn::Target)>> {
		pub fn as_raw_VectorOfTupleOfBackend_Target(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfTupleOfBackend_Target(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Tuple<(crate::dnn::Backend, crate::dnn::Target)>,
		cv_VectorOfTupleOfBackend_Target_new, cv_VectorOfTupleOfBackend_Target_delete,
		cv_VectorOfTupleOfBackend_Target_len, cv_VectorOfTupleOfBackend_Target_is_empty,
		cv_VectorOfTupleOfBackend_Target_capacity, cv_VectorOfTupleOfBackend_Target_shrink_to_fit,
		cv_VectorOfTupleOfBackend_Target_reserve, cv_VectorOfTupleOfBackend_Target_remove,
		cv_VectorOfTupleOfBackend_Target_swap, cv_VectorOfTupleOfBackend_Target_clear,
		cv_VectorOfTupleOfBackend_Target_get, cv_VectorOfTupleOfBackend_Target_set,
		cv_VectorOfTupleOfBackend_Target_push, cv_VectorOfTupleOfBackend_Target_insert,
	}
	vector_non_copy_or_bool! { core::Tuple<(crate::dnn::Backend, crate::dnn::Target)> }
	
	pub type VectorOfVectorOfMatShape = core::Vector<core::Vector<crate::dnn::MatShape>>;
	
	impl core::Vector<core::Vector<crate::dnn::MatShape>> {
		pub fn as_raw_VectorOfVectorOfMatShape(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfMatShape(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<crate::dnn::MatShape>,
		cv_VectorOfVectorOfMatShape_new, cv_VectorOfVectorOfMatShape_delete,
		cv_VectorOfVectorOfMatShape_len, cv_VectorOfVectorOfMatShape_is_empty,
		cv_VectorOfVectorOfMatShape_capacity, cv_VectorOfVectorOfMatShape_shrink_to_fit,
		cv_VectorOfVectorOfMatShape_reserve, cv_VectorOfVectorOfMatShape_remove,
		cv_VectorOfVectorOfMatShape_swap, cv_VectorOfVectorOfMatShape_clear,
		cv_VectorOfVectorOfMatShape_get, cv_VectorOfVectorOfMatShape_set,
		cv_VectorOfVectorOfMatShape_push, cv_VectorOfVectorOfMatShape_insert,
	}
	vector_non_copy_or_bool! { core::Vector<crate::dnn::MatShape> }
	
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
	
	impl core::Ptr<crate::dnn_superres::DnnSuperResImpl> {
		#[inline] pub fn as_raw_PtrOfDnnSuperResImpl(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDnnSuperResImpl(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dnn_superres::DnnSuperResImplTraitConst for core::Ptr<crate::dnn_superres::DnnSuperResImpl> {
		#[inline] fn as_raw_DnnSuperResImpl(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dnn_superres::DnnSuperResImplTrait for core::Ptr<crate::dnn_superres::DnnSuperResImpl> {
		#[inline] fn as_raw_mut_DnnSuperResImpl(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_dnn_superres)]
pub use dnn_superres_types::*;

#[cfg(ocvrs_has_module_dpm)]
mod dpm_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfDPMDetector = core::Ptr<crate::dpm::DPMDetector>;
	
	ptr_extern! { crate::dpm::DPMDetector,
		cv_PtrOfDPMDetector_delete, cv_PtrOfDPMDetector_get_inner_ptr, cv_PtrOfDPMDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::dpm::DPMDetector> {
		#[inline] pub fn as_raw_PtrOfDPMDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDPMDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::dpm::DPMDetectorTraitConst for core::Ptr<crate::dpm::DPMDetector> {
		#[inline] fn as_raw_DPMDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::dpm::DPMDetectorTrait for core::Ptr<crate::dpm::DPMDetector> {
		#[inline] fn as_raw_mut_DPMDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfDPMDetector_ObjectDetection = core::Vector<crate::dpm::DPMDetector_ObjectDetection>;
	
	impl core::Vector<crate::dpm::DPMDetector_ObjectDetection> {
		pub fn as_raw_VectorOfDPMDetector_ObjectDetection(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDPMDetector_ObjectDetection(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::dpm::DPMDetector_ObjectDetection,
		cv_VectorOfDPMDetector_ObjectDetection_new, cv_VectorOfDPMDetector_ObjectDetection_delete,
		cv_VectorOfDPMDetector_ObjectDetection_len, cv_VectorOfDPMDetector_ObjectDetection_is_empty,
		cv_VectorOfDPMDetector_ObjectDetection_capacity, cv_VectorOfDPMDetector_ObjectDetection_shrink_to_fit,
		cv_VectorOfDPMDetector_ObjectDetection_reserve, cv_VectorOfDPMDetector_ObjectDetection_remove,
		cv_VectorOfDPMDetector_ObjectDetection_swap, cv_VectorOfDPMDetector_ObjectDetection_clear,
		cv_VectorOfDPMDetector_ObjectDetection_get, cv_VectorOfDPMDetector_ObjectDetection_set,
		cv_VectorOfDPMDetector_ObjectDetection_push, cv_VectorOfDPMDetector_ObjectDetection_insert,
	}
	vector_non_copy_or_bool! { crate::dpm::DPMDetector_ObjectDetection }
	
}
#[cfg(ocvrs_has_module_dpm)]
pub use dpm_types::*;

#[cfg(ocvrs_has_module_face)]
mod face_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBIF = core::Ptr<crate::face::BIF>;
	
	ptr_extern! { crate::face::BIF,
		cv_PtrOfBIF_delete, cv_PtrOfBIF_get_inner_ptr, cv_PtrOfBIF_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::face::BIF> {
		#[inline] pub fn as_raw_PtrOfBIF(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBIF(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::face::BIFTraitConst for core::Ptr<crate::face::BIF> {
		#[inline] fn as_raw_BIF(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::BIFTrait for core::Ptr<crate::face::BIF> {
		#[inline] fn as_raw_mut_BIF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::face::BIF> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::face::BIF> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEigenFaceRecognizer = core::Ptr<crate::face::EigenFaceRecognizer>;
	
	ptr_extern! { crate::face::EigenFaceRecognizer,
		cv_PtrOfEigenFaceRecognizer_delete, cv_PtrOfEigenFaceRecognizer_get_inner_ptr, cv_PtrOfEigenFaceRecognizer_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::face::EigenFaceRecognizer> {
		#[inline] pub fn as_raw_PtrOfEigenFaceRecognizer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEigenFaceRecognizer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::face::EigenFaceRecognizerTraitConst for core::Ptr<crate::face::EigenFaceRecognizer> {
		#[inline] fn as_raw_EigenFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::EigenFaceRecognizerTrait for core::Ptr<crate::face::EigenFaceRecognizer> {
		#[inline] fn as_raw_mut_EigenFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::face::EigenFaceRecognizer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::face::EigenFaceRecognizer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::BasicFaceRecognizerTraitConst for core::Ptr<crate::face::EigenFaceRecognizer> {
		#[inline] fn as_raw_BasicFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::BasicFaceRecognizerTrait for core::Ptr<crate::face::EigenFaceRecognizer> {
		#[inline] fn as_raw_mut_BasicFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FaceRecognizerTraitConst for core::Ptr<crate::face::EigenFaceRecognizer> {
		#[inline] fn as_raw_FaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::FaceRecognizerTrait for core::Ptr<crate::face::EigenFaceRecognizer> {
		#[inline] fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFacemark = core::Ptr<crate::face::Facemark>;
	
	ptr_extern! { crate::face::Facemark,
		cv_PtrOfFacemark_delete, cv_PtrOfFacemark_get_inner_ptr, cv_PtrOfFacemark_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::face::Facemark> {
		#[inline] pub fn as_raw_PtrOfFacemark(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFacemark(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::face::FacemarkTraitConst for core::Ptr<crate::face::Facemark> {
		#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::FacemarkTrait for core::Ptr<crate::face::Facemark> {
		#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::face::Facemark> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::face::Facemark> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFacemarkAAM = core::Ptr<crate::face::FacemarkAAM>;
	
	ptr_extern! { crate::face::FacemarkAAM,
		cv_PtrOfFacemarkAAM_delete, cv_PtrOfFacemarkAAM_get_inner_ptr, cv_PtrOfFacemarkAAM_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::face::FacemarkAAM> {
		#[inline] pub fn as_raw_PtrOfFacemarkAAM(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFacemarkAAM(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::face::FacemarkAAMTraitConst for core::Ptr<crate::face::FacemarkAAM> {
		#[inline] fn as_raw_FacemarkAAM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::FacemarkAAMTrait for core::Ptr<crate::face::FacemarkAAM> {
		#[inline] fn as_raw_mut_FacemarkAAM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::face::FacemarkAAM> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::face::FacemarkAAM> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FacemarkTraitConst for core::Ptr<crate::face::FacemarkAAM> {
		#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::FacemarkTrait for core::Ptr<crate::face::FacemarkAAM> {
		#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FacemarkTrainTraitConst for core::Ptr<crate::face::FacemarkAAM> {
		#[inline] fn as_raw_FacemarkTrain(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::FacemarkTrainTrait for core::Ptr<crate::face::FacemarkAAM> {
		#[inline] fn as_raw_mut_FacemarkTrain(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFacemarkKazemi = core::Ptr<crate::face::FacemarkKazemi>;
	
	ptr_extern! { crate::face::FacemarkKazemi,
		cv_PtrOfFacemarkKazemi_delete, cv_PtrOfFacemarkKazemi_get_inner_ptr, cv_PtrOfFacemarkKazemi_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::face::FacemarkKazemi> {
		#[inline] pub fn as_raw_PtrOfFacemarkKazemi(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFacemarkKazemi(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::face::FacemarkKazemiTraitConst for core::Ptr<crate::face::FacemarkKazemi> {
		#[inline] fn as_raw_FacemarkKazemi(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::FacemarkKazemiTrait for core::Ptr<crate::face::FacemarkKazemi> {
		#[inline] fn as_raw_mut_FacemarkKazemi(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::face::FacemarkKazemi> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::face::FacemarkKazemi> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FacemarkTraitConst for core::Ptr<crate::face::FacemarkKazemi> {
		#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::FacemarkTrait for core::Ptr<crate::face::FacemarkKazemi> {
		#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFacemarkLBF = core::Ptr<crate::face::FacemarkLBF>;
	
	ptr_extern! { crate::face::FacemarkLBF,
		cv_PtrOfFacemarkLBF_delete, cv_PtrOfFacemarkLBF_get_inner_ptr, cv_PtrOfFacemarkLBF_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::face::FacemarkLBF> {
		#[inline] pub fn as_raw_PtrOfFacemarkLBF(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFacemarkLBF(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::face::FacemarkLBFTraitConst for core::Ptr<crate::face::FacemarkLBF> {
		#[inline] fn as_raw_FacemarkLBF(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::FacemarkLBFTrait for core::Ptr<crate::face::FacemarkLBF> {
		#[inline] fn as_raw_mut_FacemarkLBF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::face::FacemarkLBF> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::face::FacemarkLBF> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FacemarkTraitConst for core::Ptr<crate::face::FacemarkLBF> {
		#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::FacemarkTrait for core::Ptr<crate::face::FacemarkLBF> {
		#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FacemarkTrainTraitConst for core::Ptr<crate::face::FacemarkLBF> {
		#[inline] fn as_raw_FacemarkTrain(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::FacemarkTrainTrait for core::Ptr<crate::face::FacemarkLBF> {
		#[inline] fn as_raw_mut_FacemarkTrain(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFisherFaceRecognizer = core::Ptr<crate::face::FisherFaceRecognizer>;
	
	ptr_extern! { crate::face::FisherFaceRecognizer,
		cv_PtrOfFisherFaceRecognizer_delete, cv_PtrOfFisherFaceRecognizer_get_inner_ptr, cv_PtrOfFisherFaceRecognizer_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::face::FisherFaceRecognizer> {
		#[inline] pub fn as_raw_PtrOfFisherFaceRecognizer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFisherFaceRecognizer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::face::FisherFaceRecognizerTraitConst for core::Ptr<crate::face::FisherFaceRecognizer> {
		#[inline] fn as_raw_FisherFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::FisherFaceRecognizerTrait for core::Ptr<crate::face::FisherFaceRecognizer> {
		#[inline] fn as_raw_mut_FisherFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::face::FisherFaceRecognizer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::face::FisherFaceRecognizer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::BasicFaceRecognizerTraitConst for core::Ptr<crate::face::FisherFaceRecognizer> {
		#[inline] fn as_raw_BasicFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::BasicFaceRecognizerTrait for core::Ptr<crate::face::FisherFaceRecognizer> {
		#[inline] fn as_raw_mut_BasicFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FaceRecognizerTraitConst for core::Ptr<crate::face::FisherFaceRecognizer> {
		#[inline] fn as_raw_FaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::FaceRecognizerTrait for core::Ptr<crate::face::FisherFaceRecognizer> {
		#[inline] fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLBPHFaceRecognizer = core::Ptr<crate::face::LBPHFaceRecognizer>;
	
	ptr_extern! { crate::face::LBPHFaceRecognizer,
		cv_PtrOfLBPHFaceRecognizer_delete, cv_PtrOfLBPHFaceRecognizer_get_inner_ptr, cv_PtrOfLBPHFaceRecognizer_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::face::LBPHFaceRecognizer> {
		#[inline] pub fn as_raw_PtrOfLBPHFaceRecognizer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLBPHFaceRecognizer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::face::LBPHFaceRecognizerTraitConst for core::Ptr<crate::face::LBPHFaceRecognizer> {
		#[inline] fn as_raw_LBPHFaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::LBPHFaceRecognizerTrait for core::Ptr<crate::face::LBPHFaceRecognizer> {
		#[inline] fn as_raw_mut_LBPHFaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::face::LBPHFaceRecognizer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::face::LBPHFaceRecognizer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::FaceRecognizerTraitConst for core::Ptr<crate::face::LBPHFaceRecognizer> {
		#[inline] fn as_raw_FaceRecognizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::FaceRecognizerTrait for core::Ptr<crate::face::LBPHFaceRecognizer> {
		#[inline] fn as_raw_mut_FaceRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMACE = core::Ptr<crate::face::MACE>;
	
	ptr_extern! { crate::face::MACE,
		cv_PtrOfMACE_delete, cv_PtrOfMACE_get_inner_ptr, cv_PtrOfMACE_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::face::MACE> {
		#[inline] pub fn as_raw_PtrOfMACE(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMACE(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::face::MACETraitConst for core::Ptr<crate::face::MACE> {
		#[inline] fn as_raw_MACE(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::MACETrait for core::Ptr<crate::face::MACE> {
		#[inline] fn as_raw_mut_MACE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::face::MACE> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::face::MACE> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPredictCollector = core::Ptr<crate::face::PredictCollector>;
	
	ptr_extern! { crate::face::PredictCollector,
		cv_PtrOfPredictCollector_delete, cv_PtrOfPredictCollector_get_inner_ptr, cv_PtrOfPredictCollector_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::face::PredictCollector> {
		#[inline] pub fn as_raw_PtrOfPredictCollector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPredictCollector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::face::PredictCollectorTraitConst for core::Ptr<crate::face::PredictCollector> {
		#[inline] fn as_raw_PredictCollector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::PredictCollectorTrait for core::Ptr<crate::face::PredictCollector> {
		#[inline] fn as_raw_mut_PredictCollector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStandardCollector = core::Ptr<crate::face::StandardCollector>;
	
	ptr_extern! { crate::face::StandardCollector,
		cv_PtrOfStandardCollector_delete, cv_PtrOfStandardCollector_get_inner_ptr, cv_PtrOfStandardCollector_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::face::StandardCollector, cv_PtrOfStandardCollector_new }
	
	impl core::Ptr<crate::face::StandardCollector> {
		#[inline] pub fn as_raw_PtrOfStandardCollector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStandardCollector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::face::StandardCollectorTraitConst for core::Ptr<crate::face::StandardCollector> {
		#[inline] fn as_raw_StandardCollector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::StandardCollectorTrait for core::Ptr<crate::face::StandardCollector> {
		#[inline] fn as_raw_mut_StandardCollector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::face::PredictCollectorTraitConst for core::Ptr<crate::face::StandardCollector> {
		#[inline] fn as_raw_PredictCollector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::face::PredictCollectorTrait for core::Ptr<crate::face::StandardCollector> {
		#[inline] fn as_raw_mut_PredictCollector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfStandardCollector, core::Ptr<crate::face::PredictCollector>, cv_PtrOfStandardCollector_to_PtrOfPredictCollector }
	
	pub type VectorOfFacemarkAAM_Config = core::Vector<crate::face::FacemarkAAM_Config>;
	
	impl core::Vector<crate::face::FacemarkAAM_Config> {
		pub fn as_raw_VectorOfFacemarkAAM_Config(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfFacemarkAAM_Config(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::face::FacemarkAAM_Config,
		cv_VectorOfFacemarkAAM_Config_new, cv_VectorOfFacemarkAAM_Config_delete,
		cv_VectorOfFacemarkAAM_Config_len, cv_VectorOfFacemarkAAM_Config_is_empty,
		cv_VectorOfFacemarkAAM_Config_capacity, cv_VectorOfFacemarkAAM_Config_shrink_to_fit,
		cv_VectorOfFacemarkAAM_Config_reserve, cv_VectorOfFacemarkAAM_Config_remove,
		cv_VectorOfFacemarkAAM_Config_swap, cv_VectorOfFacemarkAAM_Config_clear,
		cv_VectorOfFacemarkAAM_Config_get, cv_VectorOfFacemarkAAM_Config_set,
		cv_VectorOfFacemarkAAM_Config_push, cv_VectorOfFacemarkAAM_Config_insert,
	}
	vector_non_copy_or_bool! { crate::face::FacemarkAAM_Config }
	
	pub type VectorOfFacemarkAAM_Model_Texture = core::Vector<crate::face::FacemarkAAM_Model_Texture>;
	
	impl core::Vector<crate::face::FacemarkAAM_Model_Texture> {
		pub fn as_raw_VectorOfFacemarkAAM_Model_Texture(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfFacemarkAAM_Model_Texture(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::face::FacemarkAAM_Model_Texture,
		cv_VectorOfFacemarkAAM_Model_Texture_new, cv_VectorOfFacemarkAAM_Model_Texture_delete,
		cv_VectorOfFacemarkAAM_Model_Texture_len, cv_VectorOfFacemarkAAM_Model_Texture_is_empty,
		cv_VectorOfFacemarkAAM_Model_Texture_capacity, cv_VectorOfFacemarkAAM_Model_Texture_shrink_to_fit,
		cv_VectorOfFacemarkAAM_Model_Texture_reserve, cv_VectorOfFacemarkAAM_Model_Texture_remove,
		cv_VectorOfFacemarkAAM_Model_Texture_swap, cv_VectorOfFacemarkAAM_Model_Texture_clear,
		cv_VectorOfFacemarkAAM_Model_Texture_get, cv_VectorOfFacemarkAAM_Model_Texture_set,
		cv_VectorOfFacemarkAAM_Model_Texture_push, cv_VectorOfFacemarkAAM_Model_Texture_insert,
	}
	vector_non_copy_or_bool! { crate::face::FacemarkAAM_Model_Texture }
	
}
#[cfg(ocvrs_has_module_face)]
pub use face_types::*;

#[cfg(ocvrs_has_module_features2d)]
mod features2d_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAKAZE = core::Ptr<crate::features2d::AKAZE>;
	
	ptr_extern! { crate::features2d::AKAZE,
		cv_PtrOfAKAZE_delete, cv_PtrOfAKAZE_get_inner_ptr, cv_PtrOfAKAZE_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::features2d::AKAZE> {
		#[inline] pub fn as_raw_PtrOfAKAZE(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAKAZE(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::AKAZETraitConst for core::Ptr<crate::features2d::AKAZE> {
		#[inline] fn as_raw_AKAZE(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::AKAZETrait for core::Ptr<crate::features2d::AKAZE> {
		#[inline] fn as_raw_mut_AKAZE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::AKAZE> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::features2d::AKAZE> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::features2d::AKAZE> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::features2d::AKAZE> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfAKAZE, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfAKAZE_to_PtrOfFeature2D }
	
	pub type PtrOfAffineFeature = core::Ptr<crate::features2d::AffineFeature>;
	
	ptr_extern! { crate::features2d::AffineFeature,
		cv_PtrOfAffineFeature_delete, cv_PtrOfAffineFeature_get_inner_ptr, cv_PtrOfAffineFeature_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::features2d::AffineFeature> {
		#[inline] pub fn as_raw_PtrOfAffineFeature(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAffineFeature(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::AffineFeatureTraitConst for core::Ptr<crate::features2d::AffineFeature> {
		#[inline] fn as_raw_AffineFeature(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::AffineFeatureTrait for core::Ptr<crate::features2d::AffineFeature> {
		#[inline] fn as_raw_mut_AffineFeature(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::AffineFeature> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::features2d::AffineFeature> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::features2d::AffineFeature> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::features2d::AffineFeature> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfAffineFeature, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfAffineFeature_to_PtrOfFeature2D }
	
	pub type PtrOfAgastFeatureDetector = core::Ptr<crate::features2d::AgastFeatureDetector>;
	
	ptr_extern! { crate::features2d::AgastFeatureDetector,
		cv_PtrOfAgastFeatureDetector_delete, cv_PtrOfAgastFeatureDetector_get_inner_ptr, cv_PtrOfAgastFeatureDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::features2d::AgastFeatureDetector> {
		#[inline] pub fn as_raw_PtrOfAgastFeatureDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAgastFeatureDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::AgastFeatureDetectorTraitConst for core::Ptr<crate::features2d::AgastFeatureDetector> {
		#[inline] fn as_raw_AgastFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::AgastFeatureDetectorTrait for core::Ptr<crate::features2d::AgastFeatureDetector> {
		#[inline] fn as_raw_mut_AgastFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::AgastFeatureDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::features2d::AgastFeatureDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::features2d::AgastFeatureDetector> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::features2d::AgastFeatureDetector> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfAgastFeatureDetector, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfAgastFeatureDetector_to_PtrOfFeature2D }
	
	pub type PtrOfBFMatcher = core::Ptr<crate::features2d::BFMatcher>;
	
	ptr_extern! { crate::features2d::BFMatcher,
		cv_PtrOfBFMatcher_delete, cv_PtrOfBFMatcher_get_inner_ptr, cv_PtrOfBFMatcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::features2d::BFMatcher, cv_PtrOfBFMatcher_new }
	
	impl core::Ptr<crate::features2d::BFMatcher> {
		#[inline] pub fn as_raw_PtrOfBFMatcher(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBFMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::BFMatcherTraitConst for core::Ptr<crate::features2d::BFMatcher> {
		#[inline] fn as_raw_BFMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::BFMatcherTrait for core::Ptr<crate::features2d::BFMatcher> {
		#[inline] fn as_raw_mut_BFMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::BFMatcher> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::features2d::BFMatcher> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::DescriptorMatcherTraitConst for core::Ptr<crate::features2d::BFMatcher> {
		#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::DescriptorMatcherTrait for core::Ptr<crate::features2d::BFMatcher> {
		#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBRISK = core::Ptr<crate::features2d::BRISK>;
	
	ptr_extern! { crate::features2d::BRISK,
		cv_PtrOfBRISK_delete, cv_PtrOfBRISK_get_inner_ptr, cv_PtrOfBRISK_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::features2d::BRISK> {
		#[inline] pub fn as_raw_PtrOfBRISK(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBRISK(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::BRISKTraitConst for core::Ptr<crate::features2d::BRISK> {
		#[inline] fn as_raw_BRISK(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::BRISKTrait for core::Ptr<crate::features2d::BRISK> {
		#[inline] fn as_raw_mut_BRISK(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::BRISK> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::features2d::BRISK> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::features2d::BRISK> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::features2d::BRISK> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfBRISK, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfBRISK_to_PtrOfFeature2D }
	
	pub type PtrOfDescriptorMatcher = core::Ptr<crate::features2d::DescriptorMatcher>;
	
	ptr_extern! { crate::features2d::DescriptorMatcher,
		cv_PtrOfDescriptorMatcher_delete, cv_PtrOfDescriptorMatcher_get_inner_ptr, cv_PtrOfDescriptorMatcher_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::features2d::DescriptorMatcher> {
		#[inline] pub fn as_raw_PtrOfDescriptorMatcher(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDescriptorMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::DescriptorMatcherTraitConst for core::Ptr<crate::features2d::DescriptorMatcher> {
		#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::DescriptorMatcherTrait for core::Ptr<crate::features2d::DescriptorMatcher> {
		#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::DescriptorMatcher> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::features2d::DescriptorMatcher> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFastFeatureDetector = core::Ptr<crate::features2d::FastFeatureDetector>;
	
	ptr_extern! { crate::features2d::FastFeatureDetector,
		cv_PtrOfFastFeatureDetector_delete, cv_PtrOfFastFeatureDetector_get_inner_ptr, cv_PtrOfFastFeatureDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::features2d::FastFeatureDetector> {
		#[inline] pub fn as_raw_PtrOfFastFeatureDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFastFeatureDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::FastFeatureDetectorTraitConst for core::Ptr<crate::features2d::FastFeatureDetector> {
		#[inline] fn as_raw_FastFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::FastFeatureDetectorTrait for core::Ptr<crate::features2d::FastFeatureDetector> {
		#[inline] fn as_raw_mut_FastFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::FastFeatureDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::features2d::FastFeatureDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::features2d::FastFeatureDetector> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::features2d::FastFeatureDetector> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfFastFeatureDetector, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfFastFeatureDetector_to_PtrOfFeature2D }
	
	pub type PtrOfFeature2D = core::Ptr<crate::features2d::Feature2D>;
	
	ptr_extern! { crate::features2d::Feature2D,
		cv_PtrOfFeature2D_delete, cv_PtrOfFeature2D_get_inner_ptr, cv_PtrOfFeature2D_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::features2d::Feature2D, cv_PtrOfFeature2D_new }
	
	impl core::Ptr<crate::features2d::Feature2D> {
		#[inline] pub fn as_raw_PtrOfFeature2D(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFeature2D(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::features2d::Feature2D> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::features2d::Feature2D> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::Feature2D> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::features2d::Feature2D> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFlannBasedMatcher = core::Ptr<crate::features2d::FlannBasedMatcher>;
	
	ptr_extern! { crate::features2d::FlannBasedMatcher,
		cv_PtrOfFlannBasedMatcher_delete, cv_PtrOfFlannBasedMatcher_get_inner_ptr, cv_PtrOfFlannBasedMatcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::features2d::FlannBasedMatcher, cv_PtrOfFlannBasedMatcher_new }
	
	impl core::Ptr<crate::features2d::FlannBasedMatcher> {
		#[inline] pub fn as_raw_PtrOfFlannBasedMatcher(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFlannBasedMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::FlannBasedMatcherTraitConst for core::Ptr<crate::features2d::FlannBasedMatcher> {
		#[inline] fn as_raw_FlannBasedMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::FlannBasedMatcherTrait for core::Ptr<crate::features2d::FlannBasedMatcher> {
		#[inline] fn as_raw_mut_FlannBasedMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::FlannBasedMatcher> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::features2d::FlannBasedMatcher> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::DescriptorMatcherTraitConst for core::Ptr<crate::features2d::FlannBasedMatcher> {
		#[inline] fn as_raw_DescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::DescriptorMatcherTrait for core::Ptr<crate::features2d::FlannBasedMatcher> {
		#[inline] fn as_raw_mut_DescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGFTTDetector = core::Ptr<crate::features2d::GFTTDetector>;
	
	ptr_extern! { crate::features2d::GFTTDetector,
		cv_PtrOfGFTTDetector_delete, cv_PtrOfGFTTDetector_get_inner_ptr, cv_PtrOfGFTTDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::features2d::GFTTDetector> {
		#[inline] pub fn as_raw_PtrOfGFTTDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGFTTDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::GFTTDetectorTraitConst for core::Ptr<crate::features2d::GFTTDetector> {
		#[inline] fn as_raw_GFTTDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::GFTTDetectorTrait for core::Ptr<crate::features2d::GFTTDetector> {
		#[inline] fn as_raw_mut_GFTTDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::GFTTDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::features2d::GFTTDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::features2d::GFTTDetector> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::features2d::GFTTDetector> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfGFTTDetector, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfGFTTDetector_to_PtrOfFeature2D }
	
	pub type PtrOfKAZE = core::Ptr<crate::features2d::KAZE>;
	
	ptr_extern! { crate::features2d::KAZE,
		cv_PtrOfKAZE_delete, cv_PtrOfKAZE_get_inner_ptr, cv_PtrOfKAZE_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::features2d::KAZE> {
		#[inline] pub fn as_raw_PtrOfKAZE(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKAZE(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::KAZETraitConst for core::Ptr<crate::features2d::KAZE> {
		#[inline] fn as_raw_KAZE(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::KAZETrait for core::Ptr<crate::features2d::KAZE> {
		#[inline] fn as_raw_mut_KAZE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::KAZE> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::features2d::KAZE> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::features2d::KAZE> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::features2d::KAZE> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfKAZE, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfKAZE_to_PtrOfFeature2D }
	
	pub type PtrOfMSER = core::Ptr<crate::features2d::MSER>;
	
	ptr_extern! { crate::features2d::MSER,
		cv_PtrOfMSER_delete, cv_PtrOfMSER_get_inner_ptr, cv_PtrOfMSER_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::features2d::MSER> {
		#[inline] pub fn as_raw_PtrOfMSER(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMSER(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::MSERTraitConst for core::Ptr<crate::features2d::MSER> {
		#[inline] fn as_raw_MSER(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::MSERTrait for core::Ptr<crate::features2d::MSER> {
		#[inline] fn as_raw_mut_MSER(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::MSER> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::features2d::MSER> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::features2d::MSER> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::features2d::MSER> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMSER, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfMSER_to_PtrOfFeature2D }
	
	pub type PtrOfORB = core::Ptr<crate::features2d::ORB>;
	
	ptr_extern! { crate::features2d::ORB,
		cv_PtrOfORB_delete, cv_PtrOfORB_get_inner_ptr, cv_PtrOfORB_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::features2d::ORB> {
		#[inline] pub fn as_raw_PtrOfORB(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfORB(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::ORBTraitConst for core::Ptr<crate::features2d::ORB> {
		#[inline] fn as_raw_ORB(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::ORBTrait for core::Ptr<crate::features2d::ORB> {
		#[inline] fn as_raw_mut_ORB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::ORB> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::features2d::ORB> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::features2d::ORB> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::features2d::ORB> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfORB, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfORB_to_PtrOfFeature2D }
	
	pub type PtrOfSIFT = core::Ptr<crate::features2d::SIFT>;
	
	ptr_extern! { crate::features2d::SIFT,
		cv_PtrOfSIFT_delete, cv_PtrOfSIFT_get_inner_ptr, cv_PtrOfSIFT_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::features2d::SIFT> {
		#[inline] pub fn as_raw_PtrOfSIFT(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSIFT(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::SIFTTraitConst for core::Ptr<crate::features2d::SIFT> {
		#[inline] fn as_raw_SIFT(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::SIFTTrait for core::Ptr<crate::features2d::SIFT> {
		#[inline] fn as_raw_mut_SIFT(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::SIFT> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::features2d::SIFT> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::features2d::SIFT> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::features2d::SIFT> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSIFT, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfSIFT_to_PtrOfFeature2D }
	
	pub type PtrOfSimpleBlobDetector = core::Ptr<crate::features2d::SimpleBlobDetector>;
	
	ptr_extern! { crate::features2d::SimpleBlobDetector,
		cv_PtrOfSimpleBlobDetector_delete, cv_PtrOfSimpleBlobDetector_get_inner_ptr, cv_PtrOfSimpleBlobDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::features2d::SimpleBlobDetector> {
		#[inline] pub fn as_raw_PtrOfSimpleBlobDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSimpleBlobDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::features2d::SimpleBlobDetectorTraitConst for core::Ptr<crate::features2d::SimpleBlobDetector> {
		#[inline] fn as_raw_SimpleBlobDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::SimpleBlobDetectorTrait for core::Ptr<crate::features2d::SimpleBlobDetector> {
		#[inline] fn as_raw_mut_SimpleBlobDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::SimpleBlobDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::features2d::SimpleBlobDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::features2d::SimpleBlobDetector> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::features2d::SimpleBlobDetector> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSimpleBlobDetector, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfSimpleBlobDetector_to_PtrOfFeature2D }
	
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
	
	impl core::Ptr<crate::flann::IndexParams> {
		#[inline] pub fn as_raw_PtrOfIndexParams(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfIndexParams(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::flann::IndexParamsTraitConst for core::Ptr<crate::flann::IndexParams> {
		#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::flann::IndexParamsTrait for core::Ptr<crate::flann::IndexParams> {
		#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSearchParams = core::Ptr<crate::flann::SearchParams>;
	
	ptr_extern! { crate::flann::SearchParams,
		cv_PtrOfSearchParams_delete, cv_PtrOfSearchParams_get_inner_ptr, cv_PtrOfSearchParams_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::flann::SearchParams, cv_PtrOfSearchParams_new }
	
	impl core::Ptr<crate::flann::SearchParams> {
		#[inline] pub fn as_raw_PtrOfSearchParams(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSearchParams(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::flann::SearchParamsTraitConst for core::Ptr<crate::flann::SearchParams> {
		#[inline] fn as_raw_SearchParams(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::flann::SearchParamsTrait for core::Ptr<crate::flann::SearchParams> {
		#[inline] fn as_raw_mut_SearchParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::flann::IndexParamsTraitConst for core::Ptr<crate::flann::SearchParams> {
		#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::flann::IndexParamsTrait for core::Ptr<crate::flann::SearchParams> {
		#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfFlannIndexType = core::Vector<crate::flann::FlannIndexType>;
	
	impl core::Vector<crate::flann::FlannIndexType> {
		pub fn as_raw_VectorOfFlannIndexType(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfFlannIndexType(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::flann::FlannIndexType,
		cv_VectorOfFlannIndexType_new, cv_VectorOfFlannIndexType_delete,
		cv_VectorOfFlannIndexType_len, cv_VectorOfFlannIndexType_is_empty,
		cv_VectorOfFlannIndexType_capacity, cv_VectorOfFlannIndexType_shrink_to_fit,
		cv_VectorOfFlannIndexType_reserve, cv_VectorOfFlannIndexType_remove,
		cv_VectorOfFlannIndexType_swap, cv_VectorOfFlannIndexType_clear,
		cv_VectorOfFlannIndexType_get, cv_VectorOfFlannIndexType_set,
		cv_VectorOfFlannIndexType_push, cv_VectorOfFlannIndexType_insert,
	}
	vector_copy_non_bool! { crate::flann::FlannIndexType,
		cv_VectorOfFlannIndexType_data, cv_VectorOfFlannIndexType_data_mut, cv_VectorOfFlannIndexType_from_slice,
		cv_VectorOfFlannIndexType_clone,
	}
	
}
#[cfg(ocvrs_has_module_flann)]
pub use flann_types::*;

#[cfg(ocvrs_has_module_freetype)]
mod freetype_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfFreeType2 = core::Ptr<crate::freetype::FreeType2>;
	
	ptr_extern! { crate::freetype::FreeType2,
		cv_PtrOfFreeType2_delete, cv_PtrOfFreeType2_get_inner_ptr, cv_PtrOfFreeType2_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::freetype::FreeType2> {
		#[inline] pub fn as_raw_PtrOfFreeType2(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFreeType2(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::freetype::FreeType2TraitConst for core::Ptr<crate::freetype::FreeType2> {
		#[inline] fn as_raw_FreeType2(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::freetype::FreeType2Trait for core::Ptr<crate::freetype::FreeType2> {
		#[inline] fn as_raw_mut_FreeType2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::freetype::FreeType2> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::freetype::FreeType2> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_freetype)]
pub use freetype_types::*;

#[cfg(ocvrs_has_module_gapi)]
mod gapi_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type TupleOfGBackend_GKernelImpl = core::Tuple<(crate::gapi::GBackend, crate::gapi::GKernelImpl)>;
	
	impl core::Tuple<(crate::gapi::GBackend, crate::gapi::GKernelImpl)> {
		pub fn as_raw_TupleOfGBackend_GKernelImpl(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfGBackend_GKernelImpl(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	tuple_extern! { (crate::gapi::GBackend, crate::gapi::GKernelImpl),
		cv_TupleOfGBackend_GKernelImpl_new, cv_TupleOfGBackend_GKernelImpl_delete,
		0 = arg: crate::gapi::GBackend, get_0 via cv_TupleOfGBackend_GKernelImpl_get_0,
		1 = arg_1: crate::gapi::GKernelImpl, get_1 via cv_TupleOfGBackend_GKernelImpl_get_1
	}
	
	pub type TupleOfGMat_GMat = core::Tuple<(crate::gapi::GMat, crate::gapi::GMat)>;
	
	impl core::Tuple<(crate::gapi::GMat, crate::gapi::GMat)> {
		pub fn as_raw_TupleOfGMat_GMat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfGMat_GMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	tuple_extern! { (crate::gapi::GMat, crate::gapi::GMat),
		cv_TupleOfGMat_GMat_new, cv_TupleOfGMat_GMat_delete,
		0 = arg: crate::gapi::GMat, get_0 via cv_TupleOfGMat_GMat_get_0,
		1 = arg_1: crate::gapi::GMat, get_1 via cv_TupleOfGMat_GMat_get_1
	}
	
	pub type TupleOfGMat_GMat_GMat = core::Tuple<(crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat)>;
	
	impl core::Tuple<(crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat)> {
		pub fn as_raw_TupleOfGMat_GMat_GMat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfGMat_GMat_GMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	tuple_extern! { (crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat),
		cv_TupleOfGMat_GMat_GMat_new, cv_TupleOfGMat_GMat_GMat_delete,
		0 = arg: crate::gapi::GMat, get_0 via cv_TupleOfGMat_GMat_GMat_get_0,
		1 = arg_1: crate::gapi::GMat, get_1 via cv_TupleOfGMat_GMat_GMat_get_1,
		2 = arg_2: crate::gapi::GMat, get_2 via cv_TupleOfGMat_GMat_GMat_get_2
	}
	
	pub type TupleOfGMat_GMat_GMat_GMat = core::Tuple<(crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat)>;
	
	impl core::Tuple<(crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat)> {
		pub fn as_raw_TupleOfGMat_GMat_GMat_GMat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfGMat_GMat_GMat_GMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	tuple_extern! { (crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat, crate::gapi::GMat),
		cv_TupleOfGMat_GMat_GMat_GMat_new, cv_TupleOfGMat_GMat_GMat_GMat_delete,
		0 = arg: crate::gapi::GMat, get_0 via cv_TupleOfGMat_GMat_GMat_GMat_get_0,
		1 = arg_1: crate::gapi::GMat, get_1 via cv_TupleOfGMat_GMat_GMat_GMat_get_1,
		2 = arg_2: crate::gapi::GMat, get_2 via cv_TupleOfGMat_GMat_GMat_GMat_get_2,
		3 = arg_3: crate::gapi::GMat, get_3 via cv_TupleOfGMat_GMat_GMat_GMat_get_3
	}
	
	pub type TupleOfGMat_GScalar = core::Tuple<(crate::gapi::GMat, crate::gapi::GScalar)>;
	
	impl core::Tuple<(crate::gapi::GMat, crate::gapi::GScalar)> {
		pub fn as_raw_TupleOfGMat_GScalar(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfGMat_GScalar(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	tuple_extern! { (crate::gapi::GMat, crate::gapi::GScalar),
		cv_TupleOfGMat_GScalar_new, cv_TupleOfGMat_GScalar_delete,
		0 = arg: crate::gapi::GMat, get_0 via cv_TupleOfGMat_GScalar_get_0,
		1 = arg_1: crate::gapi::GScalar, get_1 via cv_TupleOfGMat_GScalar_get_1
	}
	
	pub type VectorOfGArg = core::Vector<crate::gapi::GArg>;
	
	impl core::Vector<crate::gapi::GArg> {
		pub fn as_raw_VectorOfGArg(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfGArg(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::gapi::GArg,
		cv_VectorOfGArg_new, cv_VectorOfGArg_delete,
		cv_VectorOfGArg_len, cv_VectorOfGArg_is_empty,
		cv_VectorOfGArg_capacity, cv_VectorOfGArg_shrink_to_fit,
		cv_VectorOfGArg_reserve, cv_VectorOfGArg_remove,
		cv_VectorOfGArg_swap, cv_VectorOfGArg_clear,
		cv_VectorOfGArg_get, cv_VectorOfGArg_set,
		cv_VectorOfGArg_push, cv_VectorOfGArg_insert,
	}
	vector_non_copy_or_bool! { crate::gapi::GArg }
	
	pub type VectorOfGBackend = core::Vector<crate::gapi::GBackend>;
	
	impl core::Vector<crate::gapi::GBackend> {
		pub fn as_raw_VectorOfGBackend(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfGBackend(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::gapi::GBackend,
		cv_VectorOfGBackend_new, cv_VectorOfGBackend_delete,
		cv_VectorOfGBackend_len, cv_VectorOfGBackend_is_empty,
		cv_VectorOfGBackend_capacity, cv_VectorOfGBackend_shrink_to_fit,
		cv_VectorOfGBackend_reserve, cv_VectorOfGBackend_remove,
		cv_VectorOfGBackend_swap, cv_VectorOfGBackend_clear,
		cv_VectorOfGBackend_get, cv_VectorOfGBackend_set,
		cv_VectorOfGBackend_push, cv_VectorOfGBackend_insert,
	}
	vector_non_copy_or_bool! { crate::gapi::GBackend }
	
	pub type VectorOfGCompileArg = core::Vector<crate::gapi::GCompileArg>;
	
	impl core::Vector<crate::gapi::GCompileArg> {
		pub fn as_raw_VectorOfGCompileArg(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfGCompileArg(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::gapi::GCompileArg,
		cv_VectorOfGCompileArg_new, cv_VectorOfGCompileArg_delete,
		cv_VectorOfGCompileArg_len, cv_VectorOfGCompileArg_is_empty,
		cv_VectorOfGCompileArg_capacity, cv_VectorOfGCompileArg_shrink_to_fit,
		cv_VectorOfGCompileArg_reserve, cv_VectorOfGCompileArg_remove,
		cv_VectorOfGCompileArg_swap, cv_VectorOfGCompileArg_clear,
		cv_VectorOfGCompileArg_get, cv_VectorOfGCompileArg_set,
		cv_VectorOfGCompileArg_push, cv_VectorOfGCompileArg_insert,
	}
	vector_non_copy_or_bool! { crate::gapi::GCompileArg }
	
	pub type VectorOfGMat = core::Vector<crate::gapi::GMat>;
	
	impl core::Vector<crate::gapi::GMat> {
		pub fn as_raw_VectorOfGMat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfGMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::gapi::GMat,
		cv_VectorOfGMat_new, cv_VectorOfGMat_delete,
		cv_VectorOfGMat_len, cv_VectorOfGMat_is_empty,
		cv_VectorOfGMat_capacity, cv_VectorOfGMat_shrink_to_fit,
		cv_VectorOfGMat_reserve, cv_VectorOfGMat_remove,
		cv_VectorOfGMat_swap, cv_VectorOfGMat_clear,
		cv_VectorOfGMat_get, cv_VectorOfGMat_set,
		cv_VectorOfGMat_push, cv_VectorOfGMat_insert,
	}
	vector_non_copy_or_bool! { clone crate::gapi::GMat }
	
	pub type VectorOfGRunArg = core::Vector<crate::gapi::GRunArg>;
	
	impl core::Vector<crate::gapi::GRunArg> {
		pub fn as_raw_VectorOfGRunArg(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfGRunArg(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::gapi::GRunArg,
		cv_VectorOfGRunArg_new, cv_VectorOfGRunArg_delete,
		cv_VectorOfGRunArg_len, cv_VectorOfGRunArg_is_empty,
		cv_VectorOfGRunArg_capacity, cv_VectorOfGRunArg_shrink_to_fit,
		cv_VectorOfGRunArg_reserve, cv_VectorOfGRunArg_remove,
		cv_VectorOfGRunArg_swap, cv_VectorOfGRunArg_clear,
		cv_VectorOfGRunArg_get, cv_VectorOfGRunArg_set,
		cv_VectorOfGRunArg_push, cv_VectorOfGRunArg_insert,
	}
	vector_non_copy_or_bool! { crate::gapi::GRunArg }
	
	pub type VectorOfGShape = core::Vector<crate::gapi::GShape>;
	
	impl core::Vector<crate::gapi::GShape> {
		pub fn as_raw_VectorOfGShape(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfGShape(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::gapi::GShape,
		cv_VectorOfGShape_new, cv_VectorOfGShape_delete,
		cv_VectorOfGShape_len, cv_VectorOfGShape_is_empty,
		cv_VectorOfGShape_capacity, cv_VectorOfGShape_shrink_to_fit,
		cv_VectorOfGShape_reserve, cv_VectorOfGShape_remove,
		cv_VectorOfGShape_swap, cv_VectorOfGShape_clear,
		cv_VectorOfGShape_get, cv_VectorOfGShape_set,
		cv_VectorOfGShape_push, cv_VectorOfGShape_insert,
	}
	vector_copy_non_bool! { crate::gapi::GShape,
		cv_VectorOfGShape_data, cv_VectorOfGShape_data_mut, cv_VectorOfGShape_from_slice,
		cv_VectorOfGShape_clone,
	}
	
	pub type VectorOfGTransform = core::Vector<crate::gapi::GTransform>;
	
	impl core::Vector<crate::gapi::GTransform> {
		pub fn as_raw_VectorOfGTransform(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfGTransform(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::gapi::GTransform,
		cv_VectorOfGTransform_new, cv_VectorOfGTransform_delete,
		cv_VectorOfGTransform_len, cv_VectorOfGTransform_is_empty,
		cv_VectorOfGTransform_capacity, cv_VectorOfGTransform_shrink_to_fit,
		cv_VectorOfGTransform_reserve, cv_VectorOfGTransform_remove,
		cv_VectorOfGTransform_swap, cv_VectorOfGTransform_clear,
		cv_VectorOfGTransform_get, cv_VectorOfGTransform_set,
		cv_VectorOfGTransform_push, cv_VectorOfGTransform_insert,
	}
	vector_non_copy_or_bool! { crate::gapi::GTransform }
	
	pub type VectorOfGTypeInfo = core::Vector<crate::gapi::GTypeInfo>;
	
	impl core::Vector<crate::gapi::GTypeInfo> {
		pub fn as_raw_VectorOfGTypeInfo(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfGTypeInfo(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::gapi::GTypeInfo,
		cv_VectorOfGTypeInfo_new, cv_VectorOfGTypeInfo_delete,
		cv_VectorOfGTypeInfo_len, cv_VectorOfGTypeInfo_is_empty,
		cv_VectorOfGTypeInfo_capacity, cv_VectorOfGTypeInfo_shrink_to_fit,
		cv_VectorOfGTypeInfo_reserve, cv_VectorOfGTypeInfo_remove,
		cv_VectorOfGTypeInfo_swap, cv_VectorOfGTypeInfo_clear,
		cv_VectorOfGTypeInfo_get, cv_VectorOfGTypeInfo_set,
		cv_VectorOfGTypeInfo_push, cv_VectorOfGTypeInfo_insert,
	}
	vector_non_copy_or_bool! { clone crate::gapi::GTypeInfo }
	
	pub type VectorOfOpaqueKind = core::Vector<crate::gapi::OpaqueKind>;
	
	impl core::Vector<crate::gapi::OpaqueKind> {
		pub fn as_raw_VectorOfOpaqueKind(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfOpaqueKind(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::gapi::OpaqueKind,
		cv_VectorOfOpaqueKind_new, cv_VectorOfOpaqueKind_delete,
		cv_VectorOfOpaqueKind_len, cv_VectorOfOpaqueKind_is_empty,
		cv_VectorOfOpaqueKind_capacity, cv_VectorOfOpaqueKind_shrink_to_fit,
		cv_VectorOfOpaqueKind_reserve, cv_VectorOfOpaqueKind_remove,
		cv_VectorOfOpaqueKind_swap, cv_VectorOfOpaqueKind_clear,
		cv_VectorOfOpaqueKind_get, cv_VectorOfOpaqueKind_set,
		cv_VectorOfOpaqueKind_push, cv_VectorOfOpaqueKind_insert,
	}
	vector_copy_non_bool! { crate::gapi::OpaqueKind,
		cv_VectorOfOpaqueKind_data, cv_VectorOfOpaqueKind_data_mut, cv_VectorOfOpaqueKind_from_slice,
		cv_VectorOfOpaqueKind_clone,
	}
	
}
#[cfg(ocvrs_has_module_gapi)]
pub use gapi_types::*;

#[cfg(ocvrs_has_module_hdf)]
mod hdf_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfHDF5 = core::Ptr<crate::hdf::HDF5>;
	
	ptr_extern! { crate::hdf::HDF5,
		cv_PtrOfHDF5_delete, cv_PtrOfHDF5_get_inner_ptr, cv_PtrOfHDF5_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::hdf::HDF5> {
		#[inline] pub fn as_raw_PtrOfHDF5(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHDF5(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::hdf::HDF5TraitConst for core::Ptr<crate::hdf::HDF5> {
		#[inline] fn as_raw_HDF5(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::hdf::HDF5Trait for core::Ptr<crate::hdf::HDF5> {
		#[inline] fn as_raw_mut_HDF5(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_hdf)]
pub use hdf_types::*;

#[cfg(ocvrs_has_module_hfs)]
mod hfs_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfHfsSegment = core::Ptr<crate::hfs::HfsSegment>;
	
	ptr_extern! { crate::hfs::HfsSegment,
		cv_PtrOfHfsSegment_delete, cv_PtrOfHfsSegment_get_inner_ptr, cv_PtrOfHfsSegment_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::hfs::HfsSegment> {
		#[inline] pub fn as_raw_PtrOfHfsSegment(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHfsSegment(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::hfs::HfsSegmentTraitConst for core::Ptr<crate::hfs::HfsSegment> {
		#[inline] fn as_raw_HfsSegment(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::hfs::HfsSegmentTrait for core::Ptr<crate::hfs::HfsSegment> {
		#[inline] fn as_raw_mut_HfsSegment(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::hfs::HfsSegment> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::hfs::HfsSegment> {
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
	
	impl core::Ptr<crate::img_hash::AverageHash> {
		#[inline] pub fn as_raw_PtrOfAverageHash(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAverageHash(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::img_hash::AverageHashTraitConst for core::Ptr<crate::img_hash::AverageHash> {
		#[inline] fn as_raw_AverageHash(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::AverageHashTrait for core::Ptr<crate::img_hash::AverageHash> {
		#[inline] fn as_raw_mut_AverageHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::img_hash::AverageHash> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::img_hash::AverageHash> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTraitConst for core::Ptr<crate::img_hash::AverageHash> {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for core::Ptr<crate::img_hash::AverageHash> {
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBlockMeanHash = core::Ptr<crate::img_hash::BlockMeanHash>;
	
	ptr_extern! { crate::img_hash::BlockMeanHash,
		cv_PtrOfBlockMeanHash_delete, cv_PtrOfBlockMeanHash_get_inner_ptr, cv_PtrOfBlockMeanHash_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::img_hash::BlockMeanHash, cv_PtrOfBlockMeanHash_new }
	
	impl core::Ptr<crate::img_hash::BlockMeanHash> {
		#[inline] pub fn as_raw_PtrOfBlockMeanHash(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBlockMeanHash(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::img_hash::BlockMeanHashTraitConst for core::Ptr<crate::img_hash::BlockMeanHash> {
		#[inline] fn as_raw_BlockMeanHash(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::BlockMeanHashTrait for core::Ptr<crate::img_hash::BlockMeanHash> {
		#[inline] fn as_raw_mut_BlockMeanHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::img_hash::BlockMeanHash> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::img_hash::BlockMeanHash> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTraitConst for core::Ptr<crate::img_hash::BlockMeanHash> {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for core::Ptr<crate::img_hash::BlockMeanHash> {
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfColorMomentHash = core::Ptr<crate::img_hash::ColorMomentHash>;
	
	ptr_extern! { crate::img_hash::ColorMomentHash,
		cv_PtrOfColorMomentHash_delete, cv_PtrOfColorMomentHash_get_inner_ptr, cv_PtrOfColorMomentHash_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::img_hash::ColorMomentHash, cv_PtrOfColorMomentHash_new }
	
	impl core::Ptr<crate::img_hash::ColorMomentHash> {
		#[inline] pub fn as_raw_PtrOfColorMomentHash(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfColorMomentHash(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::img_hash::ColorMomentHashTraitConst for core::Ptr<crate::img_hash::ColorMomentHash> {
		#[inline] fn as_raw_ColorMomentHash(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::ColorMomentHashTrait for core::Ptr<crate::img_hash::ColorMomentHash> {
		#[inline] fn as_raw_mut_ColorMomentHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::img_hash::ColorMomentHash> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::img_hash::ColorMomentHash> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTraitConst for core::Ptr<crate::img_hash::ColorMomentHash> {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for core::Ptr<crate::img_hash::ColorMomentHash> {
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMarrHildrethHash = core::Ptr<crate::img_hash::MarrHildrethHash>;
	
	ptr_extern! { crate::img_hash::MarrHildrethHash,
		cv_PtrOfMarrHildrethHash_delete, cv_PtrOfMarrHildrethHash_get_inner_ptr, cv_PtrOfMarrHildrethHash_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::img_hash::MarrHildrethHash, cv_PtrOfMarrHildrethHash_new }
	
	impl core::Ptr<crate::img_hash::MarrHildrethHash> {
		#[inline] pub fn as_raw_PtrOfMarrHildrethHash(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMarrHildrethHash(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::img_hash::MarrHildrethHashTraitConst for core::Ptr<crate::img_hash::MarrHildrethHash> {
		#[inline] fn as_raw_MarrHildrethHash(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::MarrHildrethHashTrait for core::Ptr<crate::img_hash::MarrHildrethHash> {
		#[inline] fn as_raw_mut_MarrHildrethHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::img_hash::MarrHildrethHash> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::img_hash::MarrHildrethHash> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTraitConst for core::Ptr<crate::img_hash::MarrHildrethHash> {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for core::Ptr<crate::img_hash::MarrHildrethHash> {
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPHash = core::Ptr<crate::img_hash::PHash>;
	
	ptr_extern! { crate::img_hash::PHash,
		cv_PtrOfPHash_delete, cv_PtrOfPHash_get_inner_ptr, cv_PtrOfPHash_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::img_hash::PHash, cv_PtrOfPHash_new }
	
	impl core::Ptr<crate::img_hash::PHash> {
		#[inline] pub fn as_raw_PtrOfPHash(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPHash(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::img_hash::PHashTraitConst for core::Ptr<crate::img_hash::PHash> {
		#[inline] fn as_raw_PHash(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::PHashTrait for core::Ptr<crate::img_hash::PHash> {
		#[inline] fn as_raw_mut_PHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::img_hash::PHash> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::img_hash::PHash> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTraitConst for core::Ptr<crate::img_hash::PHash> {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for core::Ptr<crate::img_hash::PHash> {
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRadialVarianceHash = core::Ptr<crate::img_hash::RadialVarianceHash>;
	
	ptr_extern! { crate::img_hash::RadialVarianceHash,
		cv_PtrOfRadialVarianceHash_delete, cv_PtrOfRadialVarianceHash_get_inner_ptr, cv_PtrOfRadialVarianceHash_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::img_hash::RadialVarianceHash, cv_PtrOfRadialVarianceHash_new }
	
	impl core::Ptr<crate::img_hash::RadialVarianceHash> {
		#[inline] pub fn as_raw_PtrOfRadialVarianceHash(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRadialVarianceHash(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::img_hash::RadialVarianceHashTraitConst for core::Ptr<crate::img_hash::RadialVarianceHash> {
		#[inline] fn as_raw_RadialVarianceHash(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::RadialVarianceHashTrait for core::Ptr<crate::img_hash::RadialVarianceHash> {
		#[inline] fn as_raw_mut_RadialVarianceHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::img_hash::RadialVarianceHash> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::img_hash::RadialVarianceHash> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::img_hash::ImgHashBaseTraitConst for core::Ptr<crate::img_hash::RadialVarianceHash> {
		#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::img_hash::ImgHashBaseTrait for core::Ptr<crate::img_hash::RadialVarianceHash> {
		#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_img_hash)]
pub use img_hash_types::*;

#[cfg(ocvrs_has_module_imgproc)]
mod imgproc_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfCLAHE = core::Ptr<crate::imgproc::CLAHE>;
	
	ptr_extern! { crate::imgproc::CLAHE,
		cv_PtrOfCLAHE_delete, cv_PtrOfCLAHE_get_inner_ptr, cv_PtrOfCLAHE_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::imgproc::CLAHE> {
		#[inline] pub fn as_raw_PtrOfCLAHE(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCLAHE(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::imgproc::CLAHETraitConst for core::Ptr<crate::imgproc::CLAHE> {
		#[inline] fn as_raw_CLAHE(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::imgproc::CLAHETrait for core::Ptr<crate::imgproc::CLAHE> {
		#[inline] fn as_raw_mut_CLAHE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::imgproc::CLAHE> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::imgproc::CLAHE> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGeneralizedHoughBallard = core::Ptr<crate::imgproc::GeneralizedHoughBallard>;
	
	ptr_extern! { crate::imgproc::GeneralizedHoughBallard,
		cv_PtrOfGeneralizedHoughBallard_delete, cv_PtrOfGeneralizedHoughBallard_get_inner_ptr, cv_PtrOfGeneralizedHoughBallard_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline] pub fn as_raw_PtrOfGeneralizedHoughBallard(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGeneralizedHoughBallard(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::imgproc::GeneralizedHoughBallardTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_GeneralizedHoughBallard(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::imgproc::GeneralizedHoughBallardTrait for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_mut_GeneralizedHoughBallard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::imgproc::GeneralizedHoughTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_GeneralizedHough(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::imgproc::GeneralizedHoughTrait for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGeneralizedHoughGuil = core::Ptr<crate::imgproc::GeneralizedHoughGuil>;
	
	ptr_extern! { crate::imgproc::GeneralizedHoughGuil,
		cv_PtrOfGeneralizedHoughGuil_delete, cv_PtrOfGeneralizedHoughGuil_get_inner_ptr, cv_PtrOfGeneralizedHoughGuil_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline] pub fn as_raw_PtrOfGeneralizedHoughGuil(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGeneralizedHoughGuil(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::imgproc::GeneralizedHoughGuilTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_GeneralizedHoughGuil(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::imgproc::GeneralizedHoughGuilTrait for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_mut_GeneralizedHoughGuil(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::imgproc::GeneralizedHoughTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_GeneralizedHough(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::imgproc::GeneralizedHoughTrait for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLineSegmentDetector = core::Ptr<crate::imgproc::LineSegmentDetector>;
	
	ptr_extern! { crate::imgproc::LineSegmentDetector,
		cv_PtrOfLineSegmentDetector_delete, cv_PtrOfLineSegmentDetector_get_inner_ptr, cv_PtrOfLineSegmentDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::imgproc::LineSegmentDetector> {
		#[inline] pub fn as_raw_PtrOfLineSegmentDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLineSegmentDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::imgproc::LineSegmentDetectorTraitConst for core::Ptr<crate::imgproc::LineSegmentDetector> {
		#[inline] fn as_raw_LineSegmentDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::imgproc::LineSegmentDetectorTrait for core::Ptr<crate::imgproc::LineSegmentDetector> {
		#[inline] fn as_raw_mut_LineSegmentDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::imgproc::LineSegmentDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::imgproc::LineSegmentDetector> {
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
	
	impl core::Ptr<crate::line_descriptor::BinaryDescriptor> {
		#[inline] pub fn as_raw_PtrOfBinaryDescriptor(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBinaryDescriptor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::line_descriptor::BinaryDescriptorTraitConst for core::Ptr<crate::line_descriptor::BinaryDescriptor> {
		#[inline] fn as_raw_BinaryDescriptor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::line_descriptor::BinaryDescriptorTrait for core::Ptr<crate::line_descriptor::BinaryDescriptor> {
		#[inline] fn as_raw_mut_BinaryDescriptor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::line_descriptor::BinaryDescriptor> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::line_descriptor::BinaryDescriptor> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBinaryDescriptorMatcher = core::Ptr<crate::line_descriptor::BinaryDescriptorMatcher>;
	
	ptr_extern! { crate::line_descriptor::BinaryDescriptorMatcher,
		cv_PtrOfBinaryDescriptorMatcher_delete, cv_PtrOfBinaryDescriptorMatcher_get_inner_ptr, cv_PtrOfBinaryDescriptorMatcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::line_descriptor::BinaryDescriptorMatcher, cv_PtrOfBinaryDescriptorMatcher_new }
	
	impl core::Ptr<crate::line_descriptor::BinaryDescriptorMatcher> {
		#[inline] pub fn as_raw_PtrOfBinaryDescriptorMatcher(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBinaryDescriptorMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::line_descriptor::BinaryDescriptorMatcherTraitConst for core::Ptr<crate::line_descriptor::BinaryDescriptorMatcher> {
		#[inline] fn as_raw_BinaryDescriptorMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::line_descriptor::BinaryDescriptorMatcherTrait for core::Ptr<crate::line_descriptor::BinaryDescriptorMatcher> {
		#[inline] fn as_raw_mut_BinaryDescriptorMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::line_descriptor::BinaryDescriptorMatcher> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::line_descriptor::BinaryDescriptorMatcher> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLSDDetector = core::Ptr<crate::line_descriptor::LSDDetector>;
	
	ptr_extern! { crate::line_descriptor::LSDDetector,
		cv_PtrOfLSDDetector_delete, cv_PtrOfLSDDetector_get_inner_ptr, cv_PtrOfLSDDetector_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::line_descriptor::LSDDetector, cv_PtrOfLSDDetector_new }
	
	impl core::Ptr<crate::line_descriptor::LSDDetector> {
		#[inline] pub fn as_raw_PtrOfLSDDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLSDDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::line_descriptor::LSDDetectorTraitConst for core::Ptr<crate::line_descriptor::LSDDetector> {
		#[inline] fn as_raw_LSDDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::line_descriptor::LSDDetectorTrait for core::Ptr<crate::line_descriptor::LSDDetector> {
		#[inline] fn as_raw_mut_LSDDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::line_descriptor::LSDDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::line_descriptor::LSDDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfKeyLine = core::Vector<crate::line_descriptor::KeyLine>;
	
	impl core::Vector<crate::line_descriptor::KeyLine> {
		pub fn as_raw_VectorOfKeyLine(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfKeyLine(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::line_descriptor::KeyLine,
		cv_VectorOfKeyLine_new, cv_VectorOfKeyLine_delete,
		cv_VectorOfKeyLine_len, cv_VectorOfKeyLine_is_empty,
		cv_VectorOfKeyLine_capacity, cv_VectorOfKeyLine_shrink_to_fit,
		cv_VectorOfKeyLine_reserve, cv_VectorOfKeyLine_remove,
		cv_VectorOfKeyLine_swap, cv_VectorOfKeyLine_clear,
		cv_VectorOfKeyLine_get, cv_VectorOfKeyLine_set,
		cv_VectorOfKeyLine_push, cv_VectorOfKeyLine_insert,
	}
	vector_copy_non_bool! { crate::line_descriptor::KeyLine,
		cv_VectorOfKeyLine_data, cv_VectorOfKeyLine_data_mut, cv_VectorOfKeyLine_from_slice,
		cv_VectorOfKeyLine_clone,
	}
	
	pub type VectorOfVectorOfKeyLine = core::Vector<core::Vector<crate::line_descriptor::KeyLine>>;
	
	impl core::Vector<core::Vector<crate::line_descriptor::KeyLine>> {
		pub fn as_raw_VectorOfVectorOfKeyLine(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfKeyLine(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<crate::line_descriptor::KeyLine>,
		cv_VectorOfVectorOfKeyLine_new, cv_VectorOfVectorOfKeyLine_delete,
		cv_VectorOfVectorOfKeyLine_len, cv_VectorOfVectorOfKeyLine_is_empty,
		cv_VectorOfVectorOfKeyLine_capacity, cv_VectorOfVectorOfKeyLine_shrink_to_fit,
		cv_VectorOfVectorOfKeyLine_reserve, cv_VectorOfVectorOfKeyLine_remove,
		cv_VectorOfVectorOfKeyLine_swap, cv_VectorOfVectorOfKeyLine_clear,
		cv_VectorOfVectorOfKeyLine_get, cv_VectorOfVectorOfKeyLine_set,
		cv_VectorOfVectorOfKeyLine_push, cv_VectorOfVectorOfKeyLine_insert,
	}
	vector_non_copy_or_bool! { clone core::Vector<crate::line_descriptor::KeyLine> }
	
}
#[cfg(ocvrs_has_module_line_descriptor)]
pub use line_descriptor_types::*;

#[cfg(ocvrs_has_module_mcc)]
mod mcc_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfMCC_CChecker = core::Ptr<crate::mcc::MCC_CChecker>;
	
	ptr_extern! { crate::mcc::MCC_CChecker,
		cv_PtrOfMCC_CChecker_delete, cv_PtrOfMCC_CChecker_get_inner_ptr, cv_PtrOfMCC_CChecker_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::mcc::MCC_CChecker> {
		#[inline] pub fn as_raw_PtrOfMCC_CChecker(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMCC_CChecker(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::mcc::MCC_CCheckerTraitConst for core::Ptr<crate::mcc::MCC_CChecker> {
		#[inline] fn as_raw_MCC_CChecker(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::mcc::MCC_CCheckerTrait for core::Ptr<crate::mcc::MCC_CChecker> {
		#[inline] fn as_raw_mut_MCC_CChecker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMCC_CCheckerDetector = core::Ptr<crate::mcc::MCC_CCheckerDetector>;
	
	ptr_extern! { crate::mcc::MCC_CCheckerDetector,
		cv_PtrOfMCC_CCheckerDetector_delete, cv_PtrOfMCC_CCheckerDetector_get_inner_ptr, cv_PtrOfMCC_CCheckerDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::mcc::MCC_CCheckerDetector> {
		#[inline] pub fn as_raw_PtrOfMCC_CCheckerDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMCC_CCheckerDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::mcc::MCC_CCheckerDetectorTraitConst for core::Ptr<crate::mcc::MCC_CCheckerDetector> {
		#[inline] fn as_raw_MCC_CCheckerDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::mcc::MCC_CCheckerDetectorTrait for core::Ptr<crate::mcc::MCC_CCheckerDetector> {
		#[inline] fn as_raw_mut_MCC_CCheckerDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::mcc::MCC_CCheckerDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::mcc::MCC_CCheckerDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMCC_CCheckerDraw = core::Ptr<crate::mcc::MCC_CCheckerDraw>;
	
	ptr_extern! { crate::mcc::MCC_CCheckerDraw,
		cv_PtrOfMCC_CCheckerDraw_delete, cv_PtrOfMCC_CCheckerDraw_get_inner_ptr, cv_PtrOfMCC_CCheckerDraw_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::mcc::MCC_CCheckerDraw> {
		#[inline] pub fn as_raw_PtrOfMCC_CCheckerDraw(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMCC_CCheckerDraw(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::mcc::MCC_CCheckerDrawTraitConst for core::Ptr<crate::mcc::MCC_CCheckerDraw> {
		#[inline] fn as_raw_MCC_CCheckerDraw(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::mcc::MCC_CCheckerDrawTrait for core::Ptr<crate::mcc::MCC_CCheckerDraw> {
		#[inline] fn as_raw_mut_MCC_CCheckerDraw(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMCC_DetectorParameters = core::Ptr<crate::mcc::MCC_DetectorParameters>;
	
	ptr_extern! { crate::mcc::MCC_DetectorParameters,
		cv_PtrOfMCC_DetectorParameters_delete, cv_PtrOfMCC_DetectorParameters_get_inner_ptr, cv_PtrOfMCC_DetectorParameters_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::mcc::MCC_DetectorParameters, cv_PtrOfMCC_DetectorParameters_new }
	
	impl core::Ptr<crate::mcc::MCC_DetectorParameters> {
		#[inline] pub fn as_raw_PtrOfMCC_DetectorParameters(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMCC_DetectorParameters(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::mcc::MCC_DetectorParametersTraitConst for core::Ptr<crate::mcc::MCC_DetectorParameters> {
		#[inline] fn as_raw_MCC_DetectorParameters(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::mcc::MCC_DetectorParametersTrait for core::Ptr<crate::mcc::MCC_DetectorParameters> {
		#[inline] fn as_raw_mut_MCC_DetectorParameters(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfPtrOfMCC_CChecker = core::Vector<core::Ptr<crate::mcc::MCC_CChecker>>;
	
	impl core::Vector<core::Ptr<crate::mcc::MCC_CChecker>> {
		pub fn as_raw_VectorOfPtrOfMCC_CChecker(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfMCC_CChecker(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Ptr<crate::mcc::MCC_CChecker>,
		cv_VectorOfPtrOfMCC_CChecker_new, cv_VectorOfPtrOfMCC_CChecker_delete,
		cv_VectorOfPtrOfMCC_CChecker_len, cv_VectorOfPtrOfMCC_CChecker_is_empty,
		cv_VectorOfPtrOfMCC_CChecker_capacity, cv_VectorOfPtrOfMCC_CChecker_shrink_to_fit,
		cv_VectorOfPtrOfMCC_CChecker_reserve, cv_VectorOfPtrOfMCC_CChecker_remove,
		cv_VectorOfPtrOfMCC_CChecker_swap, cv_VectorOfPtrOfMCC_CChecker_clear,
		cv_VectorOfPtrOfMCC_CChecker_get, cv_VectorOfPtrOfMCC_CChecker_set,
		cv_VectorOfPtrOfMCC_CChecker_push, cv_VectorOfPtrOfMCC_CChecker_insert,
	}
	vector_non_copy_or_bool! { core::Ptr<crate::mcc::MCC_CChecker> }
	
}
#[cfg(ocvrs_has_module_mcc)]
pub use mcc_types::*;

#[cfg(ocvrs_has_module_ml)]
mod ml_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfANN_MLP = core::Ptr<crate::ml::ANN_MLP>;
	
	ptr_extern! { crate::ml::ANN_MLP,
		cv_PtrOfANN_MLP_delete, cv_PtrOfANN_MLP_get_inner_ptr, cv_PtrOfANN_MLP_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ml::ANN_MLP> {
		#[inline] pub fn as_raw_PtrOfANN_MLP(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfANN_MLP(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::ANN_MLPTraitConst for core::Ptr<crate::ml::ANN_MLP> {
		#[inline] fn as_raw_ANN_MLP(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::ANN_MLPTrait for core::Ptr<crate::ml::ANN_MLP> {
		#[inline] fn as_raw_mut_ANN_MLP(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ml::ANN_MLP> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ml::ANN_MLP> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelTraitConst for core::Ptr<crate::ml::ANN_MLP> {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModelTrait for core::Ptr<crate::ml::ANN_MLP> {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBoost = core::Ptr<crate::ml::Boost>;
	
	ptr_extern! { crate::ml::Boost,
		cv_PtrOfBoost_delete, cv_PtrOfBoost_get_inner_ptr, cv_PtrOfBoost_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ml::Boost> {
		#[inline] pub fn as_raw_PtrOfBoost(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBoost(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::BoostTraitConst for core::Ptr<crate::ml::Boost> {
		#[inline] fn as_raw_Boost(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::BoostTrait for core::Ptr<crate::ml::Boost> {
		#[inline] fn as_raw_mut_Boost(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ml::Boost> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ml::Boost> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::DTreesTraitConst for core::Ptr<crate::ml::Boost> {
		#[inline] fn as_raw_DTrees(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::DTreesTrait for core::Ptr<crate::ml::Boost> {
		#[inline] fn as_raw_mut_DTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelTraitConst for core::Ptr<crate::ml::Boost> {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModelTrait for core::Ptr<crate::ml::Boost> {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDTrees = core::Ptr<crate::ml::DTrees>;
	
	ptr_extern! { crate::ml::DTrees,
		cv_PtrOfDTrees_delete, cv_PtrOfDTrees_get_inner_ptr, cv_PtrOfDTrees_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ml::DTrees> {
		#[inline] pub fn as_raw_PtrOfDTrees(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDTrees(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::DTreesTraitConst for core::Ptr<crate::ml::DTrees> {
		#[inline] fn as_raw_DTrees(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::DTreesTrait for core::Ptr<crate::ml::DTrees> {
		#[inline] fn as_raw_mut_DTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ml::DTrees> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ml::DTrees> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelTraitConst for core::Ptr<crate::ml::DTrees> {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModelTrait for core::Ptr<crate::ml::DTrees> {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEM = core::Ptr<crate::ml::EM>;
	
	ptr_extern! { crate::ml::EM,
		cv_PtrOfEM_delete, cv_PtrOfEM_get_inner_ptr, cv_PtrOfEM_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ml::EM> {
		#[inline] pub fn as_raw_PtrOfEM(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEM(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::EMTraitConst for core::Ptr<crate::ml::EM> {
		#[inline] fn as_raw_EM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::EMTrait for core::Ptr<crate::ml::EM> {
		#[inline] fn as_raw_mut_EM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ml::EM> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ml::EM> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelTraitConst for core::Ptr<crate::ml::EM> {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModelTrait for core::Ptr<crate::ml::EM> {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfKNearest = core::Ptr<crate::ml::KNearest>;
	
	ptr_extern! { crate::ml::KNearest,
		cv_PtrOfKNearest_delete, cv_PtrOfKNearest_get_inner_ptr, cv_PtrOfKNearest_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ml::KNearest> {
		#[inline] pub fn as_raw_PtrOfKNearest(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKNearest(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::KNearestTraitConst for core::Ptr<crate::ml::KNearest> {
		#[inline] fn as_raw_KNearest(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::KNearestTrait for core::Ptr<crate::ml::KNearest> {
		#[inline] fn as_raw_mut_KNearest(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ml::KNearest> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ml::KNearest> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelTraitConst for core::Ptr<crate::ml::KNearest> {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModelTrait for core::Ptr<crate::ml::KNearest> {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLogisticRegression = core::Ptr<crate::ml::LogisticRegression>;
	
	ptr_extern! { crate::ml::LogisticRegression,
		cv_PtrOfLogisticRegression_delete, cv_PtrOfLogisticRegression_get_inner_ptr, cv_PtrOfLogisticRegression_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ml::LogisticRegression> {
		#[inline] pub fn as_raw_PtrOfLogisticRegression(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLogisticRegression(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::LogisticRegressionTraitConst for core::Ptr<crate::ml::LogisticRegression> {
		#[inline] fn as_raw_LogisticRegression(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::LogisticRegressionTrait for core::Ptr<crate::ml::LogisticRegression> {
		#[inline] fn as_raw_mut_LogisticRegression(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ml::LogisticRegression> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ml::LogisticRegression> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelTraitConst for core::Ptr<crate::ml::LogisticRegression> {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModelTrait for core::Ptr<crate::ml::LogisticRegression> {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfNormalBayesClassifier = core::Ptr<crate::ml::NormalBayesClassifier>;
	
	ptr_extern! { crate::ml::NormalBayesClassifier,
		cv_PtrOfNormalBayesClassifier_delete, cv_PtrOfNormalBayesClassifier_get_inner_ptr, cv_PtrOfNormalBayesClassifier_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ml::NormalBayesClassifier> {
		#[inline] pub fn as_raw_PtrOfNormalBayesClassifier(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNormalBayesClassifier(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::NormalBayesClassifierTraitConst for core::Ptr<crate::ml::NormalBayesClassifier> {
		#[inline] fn as_raw_NormalBayesClassifier(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::NormalBayesClassifierTrait for core::Ptr<crate::ml::NormalBayesClassifier> {
		#[inline] fn as_raw_mut_NormalBayesClassifier(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ml::NormalBayesClassifier> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ml::NormalBayesClassifier> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelTraitConst for core::Ptr<crate::ml::NormalBayesClassifier> {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModelTrait for core::Ptr<crate::ml::NormalBayesClassifier> {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfParamGrid = core::Ptr<crate::ml::ParamGrid>;
	
	ptr_extern! { crate::ml::ParamGrid,
		cv_PtrOfParamGrid_delete, cv_PtrOfParamGrid_get_inner_ptr, cv_PtrOfParamGrid_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::ml::ParamGrid, cv_PtrOfParamGrid_new }
	
	impl core::Ptr<crate::ml::ParamGrid> {
		#[inline] pub fn as_raw_PtrOfParamGrid(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfParamGrid(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::ParamGridTraitConst for core::Ptr<crate::ml::ParamGrid> {
		#[inline] fn as_raw_ParamGrid(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::ParamGridTrait for core::Ptr<crate::ml::ParamGrid> {
		#[inline] fn as_raw_mut_ParamGrid(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRTrees = core::Ptr<crate::ml::RTrees>;
	
	ptr_extern! { crate::ml::RTrees,
		cv_PtrOfRTrees_delete, cv_PtrOfRTrees_get_inner_ptr, cv_PtrOfRTrees_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ml::RTrees> {
		#[inline] pub fn as_raw_PtrOfRTrees(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRTrees(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::RTreesTraitConst for core::Ptr<crate::ml::RTrees> {
		#[inline] fn as_raw_RTrees(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::RTreesTrait for core::Ptr<crate::ml::RTrees> {
		#[inline] fn as_raw_mut_RTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ml::RTrees> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ml::RTrees> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::DTreesTraitConst for core::Ptr<crate::ml::RTrees> {
		#[inline] fn as_raw_DTrees(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::DTreesTrait for core::Ptr<crate::ml::RTrees> {
		#[inline] fn as_raw_mut_DTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelTraitConst for core::Ptr<crate::ml::RTrees> {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModelTrait for core::Ptr<crate::ml::RTrees> {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSVM = core::Ptr<crate::ml::SVM>;
	
	ptr_extern! { crate::ml::SVM,
		cv_PtrOfSVM_delete, cv_PtrOfSVM_get_inner_ptr, cv_PtrOfSVM_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ml::SVM> {
		#[inline] pub fn as_raw_PtrOfSVM(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSVM(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::SVMTraitConst for core::Ptr<crate::ml::SVM> {
		#[inline] fn as_raw_SVM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::SVMTrait for core::Ptr<crate::ml::SVM> {
		#[inline] fn as_raw_mut_SVM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ml::SVM> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ml::SVM> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelTraitConst for core::Ptr<crate::ml::SVM> {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModelTrait for core::Ptr<crate::ml::SVM> {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSVMSGD = core::Ptr<crate::ml::SVMSGD>;
	
	ptr_extern! { crate::ml::SVMSGD,
		cv_PtrOfSVMSGD_delete, cv_PtrOfSVMSGD_get_inner_ptr, cv_PtrOfSVMSGD_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ml::SVMSGD> {
		#[inline] pub fn as_raw_PtrOfSVMSGD(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSVMSGD(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::SVMSGDTraitConst for core::Ptr<crate::ml::SVMSGD> {
		#[inline] fn as_raw_SVMSGD(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::SVMSGDTrait for core::Ptr<crate::ml::SVMSGD> {
		#[inline] fn as_raw_mut_SVMSGD(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ml::SVMSGD> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ml::SVMSGD> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ml::StatModelTraitConst for core::Ptr<crate::ml::SVMSGD> {
		#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::StatModelTrait for core::Ptr<crate::ml::SVMSGD> {
		#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSVM_Kernel = core::Ptr<crate::ml::SVM_Kernel>;
	
	ptr_extern! { crate::ml::SVM_Kernel,
		cv_PtrOfSVM_Kernel_delete, cv_PtrOfSVM_Kernel_get_inner_ptr, cv_PtrOfSVM_Kernel_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ml::SVM_Kernel> {
		#[inline] pub fn as_raw_PtrOfSVM_Kernel(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSVM_Kernel(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::SVM_KernelTraitConst for core::Ptr<crate::ml::SVM_Kernel> {
		#[inline] fn as_raw_SVM_Kernel(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::SVM_KernelTrait for core::Ptr<crate::ml::SVM_Kernel> {
		#[inline] fn as_raw_mut_SVM_Kernel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ml::SVM_Kernel> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ml::SVM_Kernel> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrainData = core::Ptr<crate::ml::TrainData>;
	
	ptr_extern! { crate::ml::TrainData,
		cv_PtrOfTrainData_delete, cv_PtrOfTrainData_get_inner_ptr, cv_PtrOfTrainData_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ml::TrainData> {
		#[inline] pub fn as_raw_PtrOfTrainData(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrainData(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ml::TrainDataTraitConst for core::Ptr<crate::ml::TrainData> {
		#[inline] fn as_raw_TrainData(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ml::TrainDataTrait for core::Ptr<crate::ml::TrainData> {
		#[inline] fn as_raw_mut_TrainData(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfDTrees_Node = core::Vector<crate::ml::DTrees_Node>;
	
	impl core::Vector<crate::ml::DTrees_Node> {
		pub fn as_raw_VectorOfDTrees_Node(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDTrees_Node(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::ml::DTrees_Node,
		cv_VectorOfDTrees_Node_new, cv_VectorOfDTrees_Node_delete,
		cv_VectorOfDTrees_Node_len, cv_VectorOfDTrees_Node_is_empty,
		cv_VectorOfDTrees_Node_capacity, cv_VectorOfDTrees_Node_shrink_to_fit,
		cv_VectorOfDTrees_Node_reserve, cv_VectorOfDTrees_Node_remove,
		cv_VectorOfDTrees_Node_swap, cv_VectorOfDTrees_Node_clear,
		cv_VectorOfDTrees_Node_get, cv_VectorOfDTrees_Node_set,
		cv_VectorOfDTrees_Node_push, cv_VectorOfDTrees_Node_insert,
	}
	vector_non_copy_or_bool! { crate::ml::DTrees_Node }
	
	pub type VectorOfDTrees_Split = core::Vector<crate::ml::DTrees_Split>;
	
	impl core::Vector<crate::ml::DTrees_Split> {
		pub fn as_raw_VectorOfDTrees_Split(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDTrees_Split(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::ml::DTrees_Split,
		cv_VectorOfDTrees_Split_new, cv_VectorOfDTrees_Split_delete,
		cv_VectorOfDTrees_Split_len, cv_VectorOfDTrees_Split_is_empty,
		cv_VectorOfDTrees_Split_capacity, cv_VectorOfDTrees_Split_shrink_to_fit,
		cv_VectorOfDTrees_Split_reserve, cv_VectorOfDTrees_Split_remove,
		cv_VectorOfDTrees_Split_swap, cv_VectorOfDTrees_Split_clear,
		cv_VectorOfDTrees_Split_get, cv_VectorOfDTrees_Split_set,
		cv_VectorOfDTrees_Split_push, cv_VectorOfDTrees_Split_insert,
	}
	vector_non_copy_or_bool! { crate::ml::DTrees_Split }
	
}
#[cfg(ocvrs_has_module_ml)]
pub use ml_types::*;

#[cfg(ocvrs_has_module_objdetect)]
mod objdetect_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBaseCascadeClassifier = core::Ptr<crate::objdetect::BaseCascadeClassifier>;
	
	ptr_extern! { crate::objdetect::BaseCascadeClassifier,
		cv_PtrOfBaseCascadeClassifier_delete, cv_PtrOfBaseCascadeClassifier_get_inner_ptr, cv_PtrOfBaseCascadeClassifier_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::objdetect::BaseCascadeClassifier> {
		#[inline] pub fn as_raw_PtrOfBaseCascadeClassifier(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBaseCascadeClassifier(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::objdetect::BaseCascadeClassifierTraitConst for core::Ptr<crate::objdetect::BaseCascadeClassifier> {
		#[inline] fn as_raw_BaseCascadeClassifier(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::BaseCascadeClassifierTrait for core::Ptr<crate::objdetect::BaseCascadeClassifier> {
		#[inline] fn as_raw_mut_BaseCascadeClassifier(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::objdetect::BaseCascadeClassifier> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::objdetect::BaseCascadeClassifier> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBaseCascadeClassifier_MaskGenerator = core::Ptr<crate::objdetect::BaseCascadeClassifier_MaskGenerator>;
	
	ptr_extern! { crate::objdetect::BaseCascadeClassifier_MaskGenerator,
		cv_PtrOfBaseCascadeClassifier_MaskGenerator_delete, cv_PtrOfBaseCascadeClassifier_MaskGenerator_get_inner_ptr, cv_PtrOfBaseCascadeClassifier_MaskGenerator_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::objdetect::BaseCascadeClassifier_MaskGenerator> {
		#[inline] pub fn as_raw_PtrOfBaseCascadeClassifier_MaskGenerator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBaseCascadeClassifier_MaskGenerator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::objdetect::BaseCascadeClassifier_MaskGeneratorTraitConst for core::Ptr<crate::objdetect::BaseCascadeClassifier_MaskGenerator> {
		#[inline] fn as_raw_BaseCascadeClassifier_MaskGenerator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::BaseCascadeClassifier_MaskGeneratorTrait for core::Ptr<crate::objdetect::BaseCascadeClassifier_MaskGenerator> {
		#[inline] fn as_raw_mut_BaseCascadeClassifier_MaskGenerator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBoard = core::Ptr<crate::objdetect::Board>;
	
	ptr_extern! { crate::objdetect::Board,
		cv_PtrOfBoard_delete, cv_PtrOfBoard_get_inner_ptr, cv_PtrOfBoard_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::objdetect::Board, cv_PtrOfBoard_new }
	
	impl core::Ptr<crate::objdetect::Board> {
		#[inline] pub fn as_raw_PtrOfBoard(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBoard(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::objdetect::BoardTraitConst for core::Ptr<crate::objdetect::Board> {
		#[inline] fn as_raw_Board(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::BoardTrait for core::Ptr<crate::objdetect::Board> {
		#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCharucoBoard = core::Ptr<crate::objdetect::CharucoBoard>;
	
	ptr_extern! { crate::objdetect::CharucoBoard,
		cv_PtrOfCharucoBoard_delete, cv_PtrOfCharucoBoard_get_inner_ptr, cv_PtrOfCharucoBoard_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::objdetect::CharucoBoard, cv_PtrOfCharucoBoard_new }
	
	impl core::Ptr<crate::objdetect::CharucoBoard> {
		#[inline] pub fn as_raw_PtrOfCharucoBoard(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCharucoBoard(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::objdetect::CharucoBoardTraitConst for core::Ptr<crate::objdetect::CharucoBoard> {
		#[inline] fn as_raw_CharucoBoard(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::CharucoBoardTrait for core::Ptr<crate::objdetect::CharucoBoard> {
		#[inline] fn as_raw_mut_CharucoBoard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::objdetect::BoardTraitConst for core::Ptr<crate::objdetect::CharucoBoard> {
		#[inline] fn as_raw_Board(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::BoardTrait for core::Ptr<crate::objdetect::CharucoBoard> {
		#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetectionBasedTracker_IDetector = core::Ptr<crate::objdetect::DetectionBasedTracker_IDetector>;
	
	ptr_extern! { crate::objdetect::DetectionBasedTracker_IDetector,
		cv_PtrOfDetectionBasedTracker_IDetector_delete, cv_PtrOfDetectionBasedTracker_IDetector_get_inner_ptr, cv_PtrOfDetectionBasedTracker_IDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::objdetect::DetectionBasedTracker_IDetector> {
		#[inline] pub fn as_raw_PtrOfDetectionBasedTracker_IDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetectionBasedTracker_IDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::objdetect::DetectionBasedTracker_IDetectorTraitConst for core::Ptr<crate::objdetect::DetectionBasedTracker_IDetector> {
		#[inline] fn as_raw_DetectionBasedTracker_IDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::DetectionBasedTracker_IDetectorTrait for core::Ptr<crate::objdetect::DetectionBasedTracker_IDetector> {
		#[inline] fn as_raw_mut_DetectionBasedTracker_IDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetectorParameters = core::Ptr<crate::objdetect::DetectorParameters>;
	
	ptr_extern! { crate::objdetect::DetectorParameters,
		cv_PtrOfDetectorParameters_delete, cv_PtrOfDetectorParameters_get_inner_ptr, cv_PtrOfDetectorParameters_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::objdetect::DetectorParameters, cv_PtrOfDetectorParameters_new }
	
	impl core::Ptr<crate::objdetect::DetectorParameters> {
		#[inline] pub fn as_raw_PtrOfDetectorParameters(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetectorParameters(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::objdetect::DetectorParametersTraitConst for core::Ptr<crate::objdetect::DetectorParameters> {
		#[inline] fn as_raw_DetectorParameters(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::DetectorParametersTrait for core::Ptr<crate::objdetect::DetectorParameters> {
		#[inline] fn as_raw_mut_DetectorParameters(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDictionary = core::Ptr<crate::objdetect::Dictionary>;
	
	ptr_extern! { crate::objdetect::Dictionary,
		cv_PtrOfDictionary_delete, cv_PtrOfDictionary_get_inner_ptr, cv_PtrOfDictionary_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::objdetect::Dictionary, cv_PtrOfDictionary_new }
	
	impl core::Ptr<crate::objdetect::Dictionary> {
		#[inline] pub fn as_raw_PtrOfDictionary(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDictionary(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::objdetect::DictionaryTraitConst for core::Ptr<crate::objdetect::Dictionary> {
		#[inline] fn as_raw_Dictionary(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::DictionaryTrait for core::Ptr<crate::objdetect::Dictionary> {
		#[inline] fn as_raw_mut_Dictionary(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFaceDetectorYN = core::Ptr<crate::objdetect::FaceDetectorYN>;
	
	ptr_extern! { crate::objdetect::FaceDetectorYN,
		cv_PtrOfFaceDetectorYN_delete, cv_PtrOfFaceDetectorYN_get_inner_ptr, cv_PtrOfFaceDetectorYN_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::objdetect::FaceDetectorYN> {
		#[inline] pub fn as_raw_PtrOfFaceDetectorYN(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFaceDetectorYN(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::objdetect::FaceDetectorYNTraitConst for core::Ptr<crate::objdetect::FaceDetectorYN> {
		#[inline] fn as_raw_FaceDetectorYN(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::FaceDetectorYNTrait for core::Ptr<crate::objdetect::FaceDetectorYN> {
		#[inline] fn as_raw_mut_FaceDetectorYN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFaceRecognizerSF = core::Ptr<crate::objdetect::FaceRecognizerSF>;
	
	ptr_extern! { crate::objdetect::FaceRecognizerSF,
		cv_PtrOfFaceRecognizerSF_delete, cv_PtrOfFaceRecognizerSF_get_inner_ptr, cv_PtrOfFaceRecognizerSF_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::objdetect::FaceRecognizerSF> {
		#[inline] pub fn as_raw_PtrOfFaceRecognizerSF(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFaceRecognizerSF(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::objdetect::FaceRecognizerSFTraitConst for core::Ptr<crate::objdetect::FaceRecognizerSF> {
		#[inline] fn as_raw_FaceRecognizerSF(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::FaceRecognizerSFTrait for core::Ptr<crate::objdetect::FaceRecognizerSF> {
		#[inline] fn as_raw_mut_FaceRecognizerSF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfQRCodeEncoder = core::Ptr<crate::objdetect::QRCodeEncoder>;
	
	ptr_extern! { crate::objdetect::QRCodeEncoder,
		cv_PtrOfQRCodeEncoder_delete, cv_PtrOfQRCodeEncoder_get_inner_ptr, cv_PtrOfQRCodeEncoder_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::objdetect::QRCodeEncoder> {
		#[inline] pub fn as_raw_PtrOfQRCodeEncoder(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfQRCodeEncoder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::objdetect::QRCodeEncoderTraitConst for core::Ptr<crate::objdetect::QRCodeEncoder> {
		#[inline] fn as_raw_QRCodeEncoder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::objdetect::QRCodeEncoderTrait for core::Ptr<crate::objdetect::QRCodeEncoder> {
		#[inline] fn as_raw_mut_QRCodeEncoder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfDetectionBasedTracker_ExtObject = core::Vector<crate::objdetect::DetectionBasedTracker_ExtObject>;
	
	impl core::Vector<crate::objdetect::DetectionBasedTracker_ExtObject> {
		pub fn as_raw_VectorOfDetectionBasedTracker_ExtObject(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetectionBasedTracker_ExtObject(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::objdetect::DetectionBasedTracker_ExtObject,
		cv_VectorOfDetectionBasedTracker_ExtObject_new, cv_VectorOfDetectionBasedTracker_ExtObject_delete,
		cv_VectorOfDetectionBasedTracker_ExtObject_len, cv_VectorOfDetectionBasedTracker_ExtObject_is_empty,
		cv_VectorOfDetectionBasedTracker_ExtObject_capacity, cv_VectorOfDetectionBasedTracker_ExtObject_shrink_to_fit,
		cv_VectorOfDetectionBasedTracker_ExtObject_reserve, cv_VectorOfDetectionBasedTracker_ExtObject_remove,
		cv_VectorOfDetectionBasedTracker_ExtObject_swap, cv_VectorOfDetectionBasedTracker_ExtObject_clear,
		cv_VectorOfDetectionBasedTracker_ExtObject_get, cv_VectorOfDetectionBasedTracker_ExtObject_set,
		cv_VectorOfDetectionBasedTracker_ExtObject_push, cv_VectorOfDetectionBasedTracker_ExtObject_insert,
	}
	vector_non_copy_or_bool! { clone crate::objdetect::DetectionBasedTracker_ExtObject }
	
	pub type VectorOfDetectionBasedTracker_Object = core::Vector<crate::objdetect::DetectionBasedTracker_Object>;
	
	impl core::Vector<crate::objdetect::DetectionBasedTracker_Object> {
		pub fn as_raw_VectorOfDetectionBasedTracker_Object(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetectionBasedTracker_Object(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::objdetect::DetectionBasedTracker_Object,
		cv_VectorOfDetectionBasedTracker_Object_new, cv_VectorOfDetectionBasedTracker_Object_delete,
		cv_VectorOfDetectionBasedTracker_Object_len, cv_VectorOfDetectionBasedTracker_Object_is_empty,
		cv_VectorOfDetectionBasedTracker_Object_capacity, cv_VectorOfDetectionBasedTracker_Object_shrink_to_fit,
		cv_VectorOfDetectionBasedTracker_Object_reserve, cv_VectorOfDetectionBasedTracker_Object_remove,
		cv_VectorOfDetectionBasedTracker_Object_swap, cv_VectorOfDetectionBasedTracker_Object_clear,
		cv_VectorOfDetectionBasedTracker_Object_get, cv_VectorOfDetectionBasedTracker_Object_set,
		cv_VectorOfDetectionBasedTracker_Object_push, cv_VectorOfDetectionBasedTracker_Object_insert,
	}
	vector_non_copy_or_bool! { crate::objdetect::DetectionBasedTracker_Object }
	
	pub type VectorOfDetectionROI = core::Vector<crate::objdetect::DetectionROI>;
	
	impl core::Vector<crate::objdetect::DetectionROI> {
		pub fn as_raw_VectorOfDetectionROI(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetectionROI(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::objdetect::DetectionROI,
		cv_VectorOfDetectionROI_new, cv_VectorOfDetectionROI_delete,
		cv_VectorOfDetectionROI_len, cv_VectorOfDetectionROI_is_empty,
		cv_VectorOfDetectionROI_capacity, cv_VectorOfDetectionROI_shrink_to_fit,
		cv_VectorOfDetectionROI_reserve, cv_VectorOfDetectionROI_remove,
		cv_VectorOfDetectionROI_swap, cv_VectorOfDetectionROI_clear,
		cv_VectorOfDetectionROI_get, cv_VectorOfDetectionROI_set,
		cv_VectorOfDetectionROI_push, cv_VectorOfDetectionROI_insert,
	}
	vector_non_copy_or_bool! { crate::objdetect::DetectionROI }
	
}
#[cfg(ocvrs_has_module_objdetect)]
pub use objdetect_types::*;

#[cfg(ocvrs_has_module_optflow)]
mod optflow_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfDenseRLOFOpticalFlow = core::Ptr<crate::optflow::DenseRLOFOpticalFlow>;
	
	ptr_extern! { crate::optflow::DenseRLOFOpticalFlow,
		cv_PtrOfDenseRLOFOpticalFlow_delete, cv_PtrOfDenseRLOFOpticalFlow_get_inner_ptr, cv_PtrOfDenseRLOFOpticalFlow_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::optflow::DenseRLOFOpticalFlow> {
		#[inline] pub fn as_raw_PtrOfDenseRLOFOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDenseRLOFOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::optflow::DenseRLOFOpticalFlowTraitConst for core::Ptr<crate::optflow::DenseRLOFOpticalFlow> {
		#[inline] fn as_raw_DenseRLOFOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::optflow::DenseRLOFOpticalFlowTrait for core::Ptr<crate::optflow::DenseRLOFOpticalFlow> {
		#[inline] fn as_raw_mut_DenseRLOFOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::optflow::DenseRLOFOpticalFlow> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::optflow::DenseRLOFOpticalFlow> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DenseOpticalFlowTraitConst for core::Ptr<crate::optflow::DenseRLOFOpticalFlow> {
		#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::DenseOpticalFlowTrait for core::Ptr<crate::optflow::DenseRLOFOpticalFlow> {
		#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDualTVL1OpticalFlow = core::Ptr<crate::optflow::DualTVL1OpticalFlow>;
	
	ptr_extern! { crate::optflow::DualTVL1OpticalFlow,
		cv_PtrOfDualTVL1OpticalFlow_delete, cv_PtrOfDualTVL1OpticalFlow_get_inner_ptr, cv_PtrOfDualTVL1OpticalFlow_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::optflow::DualTVL1OpticalFlow> {
		#[inline] pub fn as_raw_PtrOfDualTVL1OpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDualTVL1OpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::optflow::DualTVL1OpticalFlowTraitConst for core::Ptr<crate::optflow::DualTVL1OpticalFlow> {
		#[inline] fn as_raw_DualTVL1OpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::optflow::DualTVL1OpticalFlowTrait for core::Ptr<crate::optflow::DualTVL1OpticalFlow> {
		#[inline] fn as_raw_mut_DualTVL1OpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::optflow::DualTVL1OpticalFlow> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::optflow::DualTVL1OpticalFlow> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DenseOpticalFlowTraitConst for core::Ptr<crate::optflow::DualTVL1OpticalFlow> {
		#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::DenseOpticalFlowTrait for core::Ptr<crate::optflow::DualTVL1OpticalFlow> {
		#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGPCTrainingSamples = core::Ptr<crate::optflow::GPCTrainingSamples>;
	
	ptr_extern! { crate::optflow::GPCTrainingSamples,
		cv_PtrOfGPCTrainingSamples_delete, cv_PtrOfGPCTrainingSamples_get_inner_ptr, cv_PtrOfGPCTrainingSamples_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::optflow::GPCTrainingSamples, cv_PtrOfGPCTrainingSamples_new }
	
	impl core::Ptr<crate::optflow::GPCTrainingSamples> {
		#[inline] pub fn as_raw_PtrOfGPCTrainingSamples(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGPCTrainingSamples(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::optflow::GPCTrainingSamplesTraitConst for core::Ptr<crate::optflow::GPCTrainingSamples> {
		#[inline] fn as_raw_GPCTrainingSamples(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::optflow::GPCTrainingSamplesTrait for core::Ptr<crate::optflow::GPCTrainingSamples> {
		#[inline] fn as_raw_mut_GPCTrainingSamples(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGPCTree = core::Ptr<crate::optflow::GPCTree>;
	
	ptr_extern! { crate::optflow::GPCTree,
		cv_PtrOfGPCTree_delete, cv_PtrOfGPCTree_get_inner_ptr, cv_PtrOfGPCTree_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::optflow::GPCTree, cv_PtrOfGPCTree_new }
	
	impl core::Ptr<crate::optflow::GPCTree> {
		#[inline] pub fn as_raw_PtrOfGPCTree(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGPCTree(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::optflow::GPCTreeTraitConst for core::Ptr<crate::optflow::GPCTree> {
		#[inline] fn as_raw_GPCTree(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::optflow::GPCTreeTrait for core::Ptr<crate::optflow::GPCTree> {
		#[inline] fn as_raw_mut_GPCTree(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::optflow::GPCTree> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::optflow::GPCTree> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPCAPrior = core::Ptr<crate::optflow::PCAPrior>;
	
	ptr_extern! { crate::optflow::PCAPrior,
		cv_PtrOfPCAPrior_delete, cv_PtrOfPCAPrior_get_inner_ptr, cv_PtrOfPCAPrior_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::optflow::PCAPrior, cv_PtrOfPCAPrior_new }
	
	impl core::Ptr<crate::optflow::PCAPrior> {
		#[inline] pub fn as_raw_PtrOfPCAPrior(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPCAPrior(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::optflow::PCAPriorTraitConst for core::Ptr<crate::optflow::PCAPrior> {
		#[inline] fn as_raw_PCAPrior(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::optflow::PCAPriorTrait for core::Ptr<crate::optflow::PCAPrior> {
		#[inline] fn as_raw_mut_PCAPrior(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRLOFOpticalFlowParameter = core::Ptr<crate::optflow::RLOFOpticalFlowParameter>;
	
	ptr_extern! { crate::optflow::RLOFOpticalFlowParameter,
		cv_PtrOfRLOFOpticalFlowParameter_delete, cv_PtrOfRLOFOpticalFlowParameter_get_inner_ptr, cv_PtrOfRLOFOpticalFlowParameter_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::optflow::RLOFOpticalFlowParameter, cv_PtrOfRLOFOpticalFlowParameter_new }
	
	impl core::Ptr<crate::optflow::RLOFOpticalFlowParameter> {
		#[inline] pub fn as_raw_PtrOfRLOFOpticalFlowParameter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRLOFOpticalFlowParameter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::optflow::RLOFOpticalFlowParameterTraitConst for core::Ptr<crate::optflow::RLOFOpticalFlowParameter> {
		#[inline] fn as_raw_RLOFOpticalFlowParameter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::optflow::RLOFOpticalFlowParameterTrait for core::Ptr<crate::optflow::RLOFOpticalFlowParameter> {
		#[inline] fn as_raw_mut_RLOFOpticalFlowParameter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSparseRLOFOpticalFlow = core::Ptr<crate::optflow::SparseRLOFOpticalFlow>;
	
	ptr_extern! { crate::optflow::SparseRLOFOpticalFlow,
		cv_PtrOfSparseRLOFOpticalFlow_delete, cv_PtrOfSparseRLOFOpticalFlow_get_inner_ptr, cv_PtrOfSparseRLOFOpticalFlow_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::optflow::SparseRLOFOpticalFlow> {
		#[inline] pub fn as_raw_PtrOfSparseRLOFOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSparseRLOFOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::optflow::SparseRLOFOpticalFlowTraitConst for core::Ptr<crate::optflow::SparseRLOFOpticalFlow> {
		#[inline] fn as_raw_SparseRLOFOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::optflow::SparseRLOFOpticalFlowTrait for core::Ptr<crate::optflow::SparseRLOFOpticalFlow> {
		#[inline] fn as_raw_mut_SparseRLOFOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::optflow::SparseRLOFOpticalFlow> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::optflow::SparseRLOFOpticalFlow> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::SparseOpticalFlowTraitConst for core::Ptr<crate::optflow::SparseRLOFOpticalFlow> {
		#[inline] fn as_raw_SparseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::SparseOpticalFlowTrait for core::Ptr<crate::optflow::SparseRLOFOpticalFlow> {
		#[inline] fn as_raw_mut_SparseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfGPCPatchDescriptor = core::Vector<crate::optflow::GPCPatchDescriptor>;
	
	impl core::Vector<crate::optflow::GPCPatchDescriptor> {
		pub fn as_raw_VectorOfGPCPatchDescriptor(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfGPCPatchDescriptor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::optflow::GPCPatchDescriptor,
		cv_VectorOfGPCPatchDescriptor_new, cv_VectorOfGPCPatchDescriptor_delete,
		cv_VectorOfGPCPatchDescriptor_len, cv_VectorOfGPCPatchDescriptor_is_empty,
		cv_VectorOfGPCPatchDescriptor_capacity, cv_VectorOfGPCPatchDescriptor_shrink_to_fit,
		cv_VectorOfGPCPatchDescriptor_reserve, cv_VectorOfGPCPatchDescriptor_remove,
		cv_VectorOfGPCPatchDescriptor_swap, cv_VectorOfGPCPatchDescriptor_clear,
		cv_VectorOfGPCPatchDescriptor_get, cv_VectorOfGPCPatchDescriptor_set,
		cv_VectorOfGPCPatchDescriptor_push, cv_VectorOfGPCPatchDescriptor_insert,
	}
	vector_non_copy_or_bool! { crate::optflow::GPCPatchDescriptor }
	
}
#[cfg(ocvrs_has_module_optflow)]
pub use optflow_types::*;

#[cfg(ocvrs_has_module_ovis)]
mod ovis_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfWindowScene = core::Ptr<crate::ovis::WindowScene>;
	
	ptr_extern! { crate::ovis::WindowScene,
		cv_PtrOfWindowScene_delete, cv_PtrOfWindowScene_get_inner_ptr, cv_PtrOfWindowScene_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ovis::WindowScene> {
		#[inline] pub fn as_raw_PtrOfWindowScene(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfWindowScene(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ovis::WindowSceneTraitConst for core::Ptr<crate::ovis::WindowScene> {
		#[inline] fn as_raw_WindowScene(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ovis::WindowSceneTrait for core::Ptr<crate::ovis::WindowScene> {
		#[inline] fn as_raw_mut_WindowScene(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_ovis)]
pub use ovis_types::*;

#[cfg(ocvrs_has_module_phase_unwrapping)]
mod phase_unwrapping_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfHistogramPhaseUnwrapping = core::Ptr<crate::phase_unwrapping::HistogramPhaseUnwrapping>;
	
	ptr_extern! { crate::phase_unwrapping::HistogramPhaseUnwrapping,
		cv_PtrOfHistogramPhaseUnwrapping_delete, cv_PtrOfHistogramPhaseUnwrapping_get_inner_ptr, cv_PtrOfHistogramPhaseUnwrapping_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::phase_unwrapping::HistogramPhaseUnwrapping> {
		#[inline] pub fn as_raw_PtrOfHistogramPhaseUnwrapping(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHistogramPhaseUnwrapping(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::phase_unwrapping::HistogramPhaseUnwrappingTraitConst for core::Ptr<crate::phase_unwrapping::HistogramPhaseUnwrapping> {
		#[inline] fn as_raw_HistogramPhaseUnwrapping(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::phase_unwrapping::HistogramPhaseUnwrappingTrait for core::Ptr<crate::phase_unwrapping::HistogramPhaseUnwrapping> {
		#[inline] fn as_raw_mut_HistogramPhaseUnwrapping(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::phase_unwrapping::HistogramPhaseUnwrapping> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::phase_unwrapping::HistogramPhaseUnwrapping> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::phase_unwrapping::PhaseUnwrappingTraitConst for core::Ptr<crate::phase_unwrapping::HistogramPhaseUnwrapping> {
		#[inline] fn as_raw_PhaseUnwrapping(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::phase_unwrapping::PhaseUnwrappingTrait for core::Ptr<crate::phase_unwrapping::HistogramPhaseUnwrapping> {
		#[inline] fn as_raw_mut_PhaseUnwrapping(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_phase_unwrapping)]
pub use phase_unwrapping_types::*;

#[cfg(ocvrs_has_module_photo)]
mod photo_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAlignMTB = core::Ptr<crate::photo::AlignMTB>;
	
	ptr_extern! { crate::photo::AlignMTB,
		cv_PtrOfAlignMTB_delete, cv_PtrOfAlignMTB_get_inner_ptr, cv_PtrOfAlignMTB_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::photo::AlignMTB> {
		#[inline] pub fn as_raw_PtrOfAlignMTB(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAlignMTB(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::photo::AlignMTBTraitConst for core::Ptr<crate::photo::AlignMTB> {
		#[inline] fn as_raw_AlignMTB(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::AlignMTBTrait for core::Ptr<crate::photo::AlignMTB> {
		#[inline] fn as_raw_mut_AlignMTB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::photo::AlignMTB> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::photo::AlignMTB> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::AlignExposuresTraitConst for core::Ptr<crate::photo::AlignMTB> {
		#[inline] fn as_raw_AlignExposures(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::AlignExposuresTrait for core::Ptr<crate::photo::AlignMTB> {
		#[inline] fn as_raw_mut_AlignExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCalibrateDebevec = core::Ptr<crate::photo::CalibrateDebevec>;
	
	ptr_extern! { crate::photo::CalibrateDebevec,
		cv_PtrOfCalibrateDebevec_delete, cv_PtrOfCalibrateDebevec_get_inner_ptr, cv_PtrOfCalibrateDebevec_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::photo::CalibrateDebevec> {
		#[inline] pub fn as_raw_PtrOfCalibrateDebevec(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCalibrateDebevec(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::photo::CalibrateDebevecTraitConst for core::Ptr<crate::photo::CalibrateDebevec> {
		#[inline] fn as_raw_CalibrateDebevec(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::CalibrateDebevecTrait for core::Ptr<crate::photo::CalibrateDebevec> {
		#[inline] fn as_raw_mut_CalibrateDebevec(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::photo::CalibrateDebevec> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::photo::CalibrateDebevec> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::CalibrateCRFTraitConst for core::Ptr<crate::photo::CalibrateDebevec> {
		#[inline] fn as_raw_CalibrateCRF(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::CalibrateCRFTrait for core::Ptr<crate::photo::CalibrateDebevec> {
		#[inline] fn as_raw_mut_CalibrateCRF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfCalibrateRobertson = core::Ptr<crate::photo::CalibrateRobertson>;
	
	ptr_extern! { crate::photo::CalibrateRobertson,
		cv_PtrOfCalibrateRobertson_delete, cv_PtrOfCalibrateRobertson_get_inner_ptr, cv_PtrOfCalibrateRobertson_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::photo::CalibrateRobertson> {
		#[inline] pub fn as_raw_PtrOfCalibrateRobertson(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCalibrateRobertson(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::photo::CalibrateRobertsonTraitConst for core::Ptr<crate::photo::CalibrateRobertson> {
		#[inline] fn as_raw_CalibrateRobertson(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::CalibrateRobertsonTrait for core::Ptr<crate::photo::CalibrateRobertson> {
		#[inline] fn as_raw_mut_CalibrateRobertson(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::photo::CalibrateRobertson> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::photo::CalibrateRobertson> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::CalibrateCRFTraitConst for core::Ptr<crate::photo::CalibrateRobertson> {
		#[inline] fn as_raw_CalibrateCRF(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::CalibrateCRFTrait for core::Ptr<crate::photo::CalibrateRobertson> {
		#[inline] fn as_raw_mut_CalibrateCRF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMergeDebevec = core::Ptr<crate::photo::MergeDebevec>;
	
	ptr_extern! { crate::photo::MergeDebevec,
		cv_PtrOfMergeDebevec_delete, cv_PtrOfMergeDebevec_get_inner_ptr, cv_PtrOfMergeDebevec_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::photo::MergeDebevec> {
		#[inline] pub fn as_raw_PtrOfMergeDebevec(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMergeDebevec(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::photo::MergeDebevecTraitConst for core::Ptr<crate::photo::MergeDebevec> {
		#[inline] fn as_raw_MergeDebevec(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::MergeDebevecTrait for core::Ptr<crate::photo::MergeDebevec> {
		#[inline] fn as_raw_mut_MergeDebevec(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::photo::MergeDebevec> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::photo::MergeDebevec> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::MergeExposuresTraitConst for core::Ptr<crate::photo::MergeDebevec> {
		#[inline] fn as_raw_MergeExposures(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::MergeExposuresTrait for core::Ptr<crate::photo::MergeDebevec> {
		#[inline] fn as_raw_mut_MergeExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMergeMertens = core::Ptr<crate::photo::MergeMertens>;
	
	ptr_extern! { crate::photo::MergeMertens,
		cv_PtrOfMergeMertens_delete, cv_PtrOfMergeMertens_get_inner_ptr, cv_PtrOfMergeMertens_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::photo::MergeMertens> {
		#[inline] pub fn as_raw_PtrOfMergeMertens(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMergeMertens(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::photo::MergeMertensTraitConst for core::Ptr<crate::photo::MergeMertens> {
		#[inline] fn as_raw_MergeMertens(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::MergeMertensTrait for core::Ptr<crate::photo::MergeMertens> {
		#[inline] fn as_raw_mut_MergeMertens(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::photo::MergeMertens> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::photo::MergeMertens> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::MergeExposuresTraitConst for core::Ptr<crate::photo::MergeMertens> {
		#[inline] fn as_raw_MergeExposures(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::MergeExposuresTrait for core::Ptr<crate::photo::MergeMertens> {
		#[inline] fn as_raw_mut_MergeExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMergeRobertson = core::Ptr<crate::photo::MergeRobertson>;
	
	ptr_extern! { crate::photo::MergeRobertson,
		cv_PtrOfMergeRobertson_delete, cv_PtrOfMergeRobertson_get_inner_ptr, cv_PtrOfMergeRobertson_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::photo::MergeRobertson> {
		#[inline] pub fn as_raw_PtrOfMergeRobertson(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMergeRobertson(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::photo::MergeRobertsonTraitConst for core::Ptr<crate::photo::MergeRobertson> {
		#[inline] fn as_raw_MergeRobertson(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::MergeRobertsonTrait for core::Ptr<crate::photo::MergeRobertson> {
		#[inline] fn as_raw_mut_MergeRobertson(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::photo::MergeRobertson> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::photo::MergeRobertson> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::MergeExposuresTraitConst for core::Ptr<crate::photo::MergeRobertson> {
		#[inline] fn as_raw_MergeExposures(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::MergeExposuresTrait for core::Ptr<crate::photo::MergeRobertson> {
		#[inline] fn as_raw_mut_MergeExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemap = core::Ptr<crate::photo::Tonemap>;
	
	ptr_extern! { crate::photo::Tonemap,
		cv_PtrOfTonemap_delete, cv_PtrOfTonemap_get_inner_ptr, cv_PtrOfTonemap_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::photo::Tonemap> {
		#[inline] pub fn as_raw_PtrOfTonemap(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTonemap(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::photo::TonemapTraitConst for core::Ptr<crate::photo::Tonemap> {
		#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::TonemapTrait for core::Ptr<crate::photo::Tonemap> {
		#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::photo::Tonemap> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::photo::Tonemap> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemapDrago = core::Ptr<crate::photo::TonemapDrago>;
	
	ptr_extern! { crate::photo::TonemapDrago,
		cv_PtrOfTonemapDrago_delete, cv_PtrOfTonemapDrago_get_inner_ptr, cv_PtrOfTonemapDrago_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::photo::TonemapDrago> {
		#[inline] pub fn as_raw_PtrOfTonemapDrago(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTonemapDrago(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::photo::TonemapDragoTraitConst for core::Ptr<crate::photo::TonemapDrago> {
		#[inline] fn as_raw_TonemapDrago(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::TonemapDragoTrait for core::Ptr<crate::photo::TonemapDrago> {
		#[inline] fn as_raw_mut_TonemapDrago(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::photo::TonemapDrago> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::photo::TonemapDrago> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::TonemapTraitConst for core::Ptr<crate::photo::TonemapDrago> {
		#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::TonemapTrait for core::Ptr<crate::photo::TonemapDrago> {
		#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemapMantiuk = core::Ptr<crate::photo::TonemapMantiuk>;
	
	ptr_extern! { crate::photo::TonemapMantiuk,
		cv_PtrOfTonemapMantiuk_delete, cv_PtrOfTonemapMantiuk_get_inner_ptr, cv_PtrOfTonemapMantiuk_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::photo::TonemapMantiuk> {
		#[inline] pub fn as_raw_PtrOfTonemapMantiuk(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTonemapMantiuk(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::photo::TonemapMantiukTraitConst for core::Ptr<crate::photo::TonemapMantiuk> {
		#[inline] fn as_raw_TonemapMantiuk(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::TonemapMantiukTrait for core::Ptr<crate::photo::TonemapMantiuk> {
		#[inline] fn as_raw_mut_TonemapMantiuk(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::photo::TonemapMantiuk> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::photo::TonemapMantiuk> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::TonemapTraitConst for core::Ptr<crate::photo::TonemapMantiuk> {
		#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::TonemapTrait for core::Ptr<crate::photo::TonemapMantiuk> {
		#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemapReinhard = core::Ptr<crate::photo::TonemapReinhard>;
	
	ptr_extern! { crate::photo::TonemapReinhard,
		cv_PtrOfTonemapReinhard_delete, cv_PtrOfTonemapReinhard_get_inner_ptr, cv_PtrOfTonemapReinhard_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::photo::TonemapReinhard> {
		#[inline] pub fn as_raw_PtrOfTonemapReinhard(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTonemapReinhard(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::photo::TonemapReinhardTraitConst for core::Ptr<crate::photo::TonemapReinhard> {
		#[inline] fn as_raw_TonemapReinhard(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::TonemapReinhardTrait for core::Ptr<crate::photo::TonemapReinhard> {
		#[inline] fn as_raw_mut_TonemapReinhard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::photo::TonemapReinhard> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::photo::TonemapReinhard> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::TonemapTraitConst for core::Ptr<crate::photo::TonemapReinhard> {
		#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::TonemapTrait for core::Ptr<crate::photo::TonemapReinhard> {
		#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_photo)]
pub use photo_types::*;

#[cfg(ocvrs_has_module_plot)]
mod plot_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfPlot2d = core::Ptr<crate::plot::Plot2d>;
	
	ptr_extern! { crate::plot::Plot2d,
		cv_PtrOfPlot2d_delete, cv_PtrOfPlot2d_get_inner_ptr, cv_PtrOfPlot2d_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::plot::Plot2d> {
		#[inline] pub fn as_raw_PtrOfPlot2d(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPlot2d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::plot::Plot2dTraitConst for core::Ptr<crate::plot::Plot2d> {
		#[inline] fn as_raw_Plot2d(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::plot::Plot2dTrait for core::Ptr<crate::plot::Plot2d> {
		#[inline] fn as_raw_mut_Plot2d(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::plot::Plot2d> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::plot::Plot2d> {
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
	
	impl core::Ptr<crate::quality::QualityBRISQUE> {
		#[inline] pub fn as_raw_PtrOfQualityBRISQUE(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfQualityBRISQUE(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::quality::QualityBRISQUETraitConst for core::Ptr<crate::quality::QualityBRISQUE> {
		#[inline] fn as_raw_QualityBRISQUE(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::quality::QualityBRISQUETrait for core::Ptr<crate::quality::QualityBRISQUE> {
		#[inline] fn as_raw_mut_QualityBRISQUE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::quality::QualityBRISQUE> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::quality::QualityBRISQUE> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::quality::QualityBaseTraitConst for core::Ptr<crate::quality::QualityBRISQUE> {
		#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::quality::QualityBaseTrait for core::Ptr<crate::quality::QualityBRISQUE> {
		#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfQualityGMSD = core::Ptr<crate::quality::QualityGMSD>;
	
	ptr_extern! { crate::quality::QualityGMSD,
		cv_PtrOfQualityGMSD_delete, cv_PtrOfQualityGMSD_get_inner_ptr, cv_PtrOfQualityGMSD_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::quality::QualityGMSD, cv_PtrOfQualityGMSD_new }
	
	impl core::Ptr<crate::quality::QualityGMSD> {
		#[inline] pub fn as_raw_PtrOfQualityGMSD(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfQualityGMSD(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::quality::QualityGMSDTraitConst for core::Ptr<crate::quality::QualityGMSD> {
		#[inline] fn as_raw_QualityGMSD(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::quality::QualityGMSDTrait for core::Ptr<crate::quality::QualityGMSD> {
		#[inline] fn as_raw_mut_QualityGMSD(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::quality::QualityGMSD> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::quality::QualityGMSD> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::quality::QualityBaseTraitConst for core::Ptr<crate::quality::QualityGMSD> {
		#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::quality::QualityBaseTrait for core::Ptr<crate::quality::QualityGMSD> {
		#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfQualityMSE = core::Ptr<crate::quality::QualityMSE>;
	
	ptr_extern! { crate::quality::QualityMSE,
		cv_PtrOfQualityMSE_delete, cv_PtrOfQualityMSE_get_inner_ptr, cv_PtrOfQualityMSE_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::quality::QualityMSE, cv_PtrOfQualityMSE_new }
	
	impl core::Ptr<crate::quality::QualityMSE> {
		#[inline] pub fn as_raw_PtrOfQualityMSE(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfQualityMSE(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::quality::QualityMSETraitConst for core::Ptr<crate::quality::QualityMSE> {
		#[inline] fn as_raw_QualityMSE(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::quality::QualityMSETrait for core::Ptr<crate::quality::QualityMSE> {
		#[inline] fn as_raw_mut_QualityMSE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::quality::QualityMSE> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::quality::QualityMSE> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::quality::QualityBaseTraitConst for core::Ptr<crate::quality::QualityMSE> {
		#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::quality::QualityBaseTrait for core::Ptr<crate::quality::QualityMSE> {
		#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfQualityPSNR = core::Ptr<crate::quality::QualityPSNR>;
	
	ptr_extern! { crate::quality::QualityPSNR,
		cv_PtrOfQualityPSNR_delete, cv_PtrOfQualityPSNR_get_inner_ptr, cv_PtrOfQualityPSNR_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::quality::QualityPSNR, cv_PtrOfQualityPSNR_new }
	
	impl core::Ptr<crate::quality::QualityPSNR> {
		#[inline] pub fn as_raw_PtrOfQualityPSNR(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfQualityPSNR(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::quality::QualityPSNRTraitConst for core::Ptr<crate::quality::QualityPSNR> {
		#[inline] fn as_raw_QualityPSNR(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::quality::QualityPSNRTrait for core::Ptr<crate::quality::QualityPSNR> {
		#[inline] fn as_raw_mut_QualityPSNR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::quality::QualityPSNR> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::quality::QualityPSNR> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::quality::QualityBaseTraitConst for core::Ptr<crate::quality::QualityPSNR> {
		#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::quality::QualityBaseTrait for core::Ptr<crate::quality::QualityPSNR> {
		#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfQualitySSIM = core::Ptr<crate::quality::QualitySSIM>;
	
	ptr_extern! { crate::quality::QualitySSIM,
		cv_PtrOfQualitySSIM_delete, cv_PtrOfQualitySSIM_get_inner_ptr, cv_PtrOfQualitySSIM_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::quality::QualitySSIM, cv_PtrOfQualitySSIM_new }
	
	impl core::Ptr<crate::quality::QualitySSIM> {
		#[inline] pub fn as_raw_PtrOfQualitySSIM(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfQualitySSIM(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::quality::QualitySSIMTraitConst for core::Ptr<crate::quality::QualitySSIM> {
		#[inline] fn as_raw_QualitySSIM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::quality::QualitySSIMTrait for core::Ptr<crate::quality::QualitySSIM> {
		#[inline] fn as_raw_mut_QualitySSIM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::quality::QualitySSIM> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::quality::QualitySSIM> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::quality::QualityBaseTraitConst for core::Ptr<crate::quality::QualitySSIM> {
		#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::quality::QualityBaseTrait for core::Ptr<crate::quality::QualitySSIM> {
		#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_quality)]
pub use quality_types::*;

#[cfg(ocvrs_has_module_rapid)]
mod rapid_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfOLSTracker = core::Ptr<crate::rapid::OLSTracker>;
	
	ptr_extern! { crate::rapid::OLSTracker,
		cv_PtrOfOLSTracker_delete, cv_PtrOfOLSTracker_get_inner_ptr, cv_PtrOfOLSTracker_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::rapid::OLSTracker> {
		#[inline] pub fn as_raw_PtrOfOLSTracker(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOLSTracker(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rapid::OLSTrackerTraitConst for core::Ptr<crate::rapid::OLSTracker> {
		#[inline] fn as_raw_OLSTracker(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rapid::OLSTrackerTrait for core::Ptr<crate::rapid::OLSTracker> {
		#[inline] fn as_raw_mut_OLSTracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::rapid::OLSTracker> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::rapid::OLSTracker> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rapid::TrackerTraitConst for core::Ptr<crate::rapid::OLSTracker> {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rapid::TrackerTrait for core::Ptr<crate::rapid::OLSTracker> {
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRapid = core::Ptr<crate::rapid::Rapid>;
	
	ptr_extern! { crate::rapid::Rapid,
		cv_PtrOfRapid_delete, cv_PtrOfRapid_get_inner_ptr, cv_PtrOfRapid_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::rapid::Rapid> {
		#[inline] pub fn as_raw_PtrOfRapid(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRapid(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rapid::RapidTraitConst for core::Ptr<crate::rapid::Rapid> {
		#[inline] fn as_raw_Rapid(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rapid::RapidTrait for core::Ptr<crate::rapid::Rapid> {
		#[inline] fn as_raw_mut_Rapid(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::rapid::Rapid> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::rapid::Rapid> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rapid::TrackerTraitConst for core::Ptr<crate::rapid::Rapid> {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rapid::TrackerTrait for core::Ptr<crate::rapid::Rapid> {
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_rapid)]
pub use rapid_types::*;

#[cfg(ocvrs_has_module_rgbd)]
mod rgbd_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfColoredKinfu_ColoredKinFu = core::Ptr<crate::rgbd::ColoredKinfu_ColoredKinFu>;
	
	ptr_extern! { crate::rgbd::ColoredKinfu_ColoredKinFu,
		cv_PtrOfColoredKinfu_ColoredKinFu_delete, cv_PtrOfColoredKinfu_ColoredKinFu_get_inner_ptr, cv_PtrOfColoredKinfu_ColoredKinFu_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::rgbd::ColoredKinfu_ColoredKinFu> {
		#[inline] pub fn as_raw_PtrOfColoredKinfu_ColoredKinFu(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfColoredKinfu_ColoredKinFu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::ColoredKinfu_ColoredKinFuTraitConst for core::Ptr<crate::rgbd::ColoredKinfu_ColoredKinFu> {
		#[inline] fn as_raw_ColoredKinfu_ColoredKinFu(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::ColoredKinfu_ColoredKinFuTrait for core::Ptr<crate::rgbd::ColoredKinfu_ColoredKinFu> {
		#[inline] fn as_raw_mut_ColoredKinfu_ColoredKinFu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfColoredKinfu_Params = core::Ptr<crate::rgbd::ColoredKinfu_Params>;
	
	ptr_extern! { crate::rgbd::ColoredKinfu_Params,
		cv_PtrOfColoredKinfu_Params_delete, cv_PtrOfColoredKinfu_Params_get_inner_ptr, cv_PtrOfColoredKinfu_Params_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::ColoredKinfu_Params, cv_PtrOfColoredKinfu_Params_new }
	
	impl core::Ptr<crate::rgbd::ColoredKinfu_Params> {
		#[inline] pub fn as_raw_PtrOfColoredKinfu_Params(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfColoredKinfu_Params(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::ColoredKinfu_ParamsTraitConst for core::Ptr<crate::rgbd::ColoredKinfu_Params> {
		#[inline] fn as_raw_ColoredKinfu_Params(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::ColoredKinfu_ParamsTrait for core::Ptr<crate::rgbd::ColoredKinfu_Params> {
		#[inline] fn as_raw_mut_ColoredKinfu_Params(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDepthCleaner = core::Ptr<crate::rgbd::DepthCleaner>;
	
	ptr_extern! { crate::rgbd::DepthCleaner,
		cv_PtrOfDepthCleaner_delete, cv_PtrOfDepthCleaner_get_inner_ptr, cv_PtrOfDepthCleaner_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::DepthCleaner, cv_PtrOfDepthCleaner_new }
	
	impl core::Ptr<crate::rgbd::DepthCleaner> {
		#[inline] pub fn as_raw_PtrOfDepthCleaner(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDepthCleaner(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::DepthCleanerTraitConst for core::Ptr<crate::rgbd::DepthCleaner> {
		#[inline] fn as_raw_DepthCleaner(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::DepthCleanerTrait for core::Ptr<crate::rgbd::DepthCleaner> {
		#[inline] fn as_raw_mut_DepthCleaner(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::rgbd::DepthCleaner> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::rgbd::DepthCleaner> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDynafu_DynaFu = core::Ptr<crate::rgbd::Dynafu_DynaFu>;
	
	ptr_extern! { crate::rgbd::Dynafu_DynaFu,
		cv_PtrOfDynafu_DynaFu_delete, cv_PtrOfDynafu_DynaFu_get_inner_ptr, cv_PtrOfDynafu_DynaFu_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::rgbd::Dynafu_DynaFu> {
		#[inline] pub fn as_raw_PtrOfDynafu_DynaFu(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDynafu_DynaFu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Dynafu_DynaFuTraitConst for core::Ptr<crate::rgbd::Dynafu_DynaFu> {
		#[inline] fn as_raw_Dynafu_DynaFu(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Dynafu_DynaFuTrait for core::Ptr<crate::rgbd::Dynafu_DynaFu> {
		#[inline] fn as_raw_mut_Dynafu_DynaFu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFastICPOdometry = core::Ptr<crate::rgbd::FastICPOdometry>;
	
	ptr_extern! { crate::rgbd::FastICPOdometry,
		cv_PtrOfFastICPOdometry_delete, cv_PtrOfFastICPOdometry_get_inner_ptr, cv_PtrOfFastICPOdometry_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::FastICPOdometry, cv_PtrOfFastICPOdometry_new }
	
	impl core::Ptr<crate::rgbd::FastICPOdometry> {
		#[inline] pub fn as_raw_PtrOfFastICPOdometry(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFastICPOdometry(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::FastICPOdometryTraitConst for core::Ptr<crate::rgbd::FastICPOdometry> {
		#[inline] fn as_raw_FastICPOdometry(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::FastICPOdometryTrait for core::Ptr<crate::rgbd::FastICPOdometry> {
		#[inline] fn as_raw_mut_FastICPOdometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::rgbd::FastICPOdometry> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::rgbd::FastICPOdometry> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::OdometryTraitConst for core::Ptr<crate::rgbd::FastICPOdometry> {
		#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::OdometryTrait for core::Ptr<crate::rgbd::FastICPOdometry> {
		#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfICPOdometry = core::Ptr<crate::rgbd::ICPOdometry>;
	
	ptr_extern! { crate::rgbd::ICPOdometry,
		cv_PtrOfICPOdometry_delete, cv_PtrOfICPOdometry_get_inner_ptr, cv_PtrOfICPOdometry_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::ICPOdometry, cv_PtrOfICPOdometry_new }
	
	impl core::Ptr<crate::rgbd::ICPOdometry> {
		#[inline] pub fn as_raw_PtrOfICPOdometry(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfICPOdometry(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::ICPOdometryTraitConst for core::Ptr<crate::rgbd::ICPOdometry> {
		#[inline] fn as_raw_ICPOdometry(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::ICPOdometryTrait for core::Ptr<crate::rgbd::ICPOdometry> {
		#[inline] fn as_raw_mut_ICPOdometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::rgbd::ICPOdometry> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::rgbd::ICPOdometry> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::OdometryTraitConst for core::Ptr<crate::rgbd::ICPOdometry> {
		#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::OdometryTrait for core::Ptr<crate::rgbd::ICPOdometry> {
		#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfKinfu_Detail_PoseGraph = core::Ptr<crate::rgbd::Kinfu_Detail_PoseGraph>;
	
	ptr_extern! { crate::rgbd::Kinfu_Detail_PoseGraph,
		cv_PtrOfKinfu_Detail_PoseGraph_delete, cv_PtrOfKinfu_Detail_PoseGraph_get_inner_ptr, cv_PtrOfKinfu_Detail_PoseGraph_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::rgbd::Kinfu_Detail_PoseGraph> {
		#[inline] pub fn as_raw_PtrOfKinfu_Detail_PoseGraph(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKinfu_Detail_PoseGraph(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Kinfu_Detail_PoseGraphTraitConst for core::Ptr<crate::rgbd::Kinfu_Detail_PoseGraph> {
		#[inline] fn as_raw_Kinfu_Detail_PoseGraph(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Kinfu_Detail_PoseGraphTrait for core::Ptr<crate::rgbd::Kinfu_Detail_PoseGraph> {
		#[inline] fn as_raw_mut_Kinfu_Detail_PoseGraph(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfKinfu_KinFu = core::Ptr<crate::rgbd::Kinfu_KinFu>;
	
	ptr_extern! { crate::rgbd::Kinfu_KinFu,
		cv_PtrOfKinfu_KinFu_delete, cv_PtrOfKinfu_KinFu_get_inner_ptr, cv_PtrOfKinfu_KinFu_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::rgbd::Kinfu_KinFu> {
		#[inline] pub fn as_raw_PtrOfKinfu_KinFu(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKinfu_KinFu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Kinfu_KinFuTraitConst for core::Ptr<crate::rgbd::Kinfu_KinFu> {
		#[inline] fn as_raw_Kinfu_KinFu(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Kinfu_KinFuTrait for core::Ptr<crate::rgbd::Kinfu_KinFu> {
		#[inline] fn as_raw_mut_Kinfu_KinFu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfKinfu_Params = core::Ptr<crate::rgbd::Kinfu_Params>;
	
	ptr_extern! { crate::rgbd::Kinfu_Params,
		cv_PtrOfKinfu_Params_delete, cv_PtrOfKinfu_Params_get_inner_ptr, cv_PtrOfKinfu_Params_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::Kinfu_Params, cv_PtrOfKinfu_Params_new }
	
	impl core::Ptr<crate::rgbd::Kinfu_Params> {
		#[inline] pub fn as_raw_PtrOfKinfu_Params(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKinfu_Params(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Kinfu_ParamsTraitConst for core::Ptr<crate::rgbd::Kinfu_Params> {
		#[inline] fn as_raw_Kinfu_Params(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Kinfu_ParamsTrait for core::Ptr<crate::rgbd::Kinfu_Params> {
		#[inline] fn as_raw_mut_Kinfu_Params(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfKinfu_Volume = core::Ptr<crate::rgbd::Kinfu_Volume>;
	
	ptr_extern! { crate::rgbd::Kinfu_Volume,
		cv_PtrOfKinfu_Volume_delete, cv_PtrOfKinfu_Volume_get_inner_ptr, cv_PtrOfKinfu_Volume_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::rgbd::Kinfu_Volume> {
		#[inline] pub fn as_raw_PtrOfKinfu_Volume(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKinfu_Volume(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Kinfu_VolumeTraitConst for core::Ptr<crate::rgbd::Kinfu_Volume> {
		#[inline] fn as_raw_Kinfu_Volume(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Kinfu_VolumeTrait for core::Ptr<crate::rgbd::Kinfu_Volume> {
		#[inline] fn as_raw_mut_Kinfu_Volume(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfKinfu_VolumeParams = core::Ptr<crate::rgbd::Kinfu_VolumeParams>;
	
	ptr_extern! { crate::rgbd::Kinfu_VolumeParams,
		cv_PtrOfKinfu_VolumeParams_delete, cv_PtrOfKinfu_VolumeParams_get_inner_ptr, cv_PtrOfKinfu_VolumeParams_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::Kinfu_VolumeParams, cv_PtrOfKinfu_VolumeParams_new }
	
	impl core::Ptr<crate::rgbd::Kinfu_VolumeParams> {
		#[inline] pub fn as_raw_PtrOfKinfu_VolumeParams(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKinfu_VolumeParams(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Kinfu_VolumeParamsTraitConst for core::Ptr<crate::rgbd::Kinfu_VolumeParams> {
		#[inline] fn as_raw_Kinfu_VolumeParams(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Kinfu_VolumeParamsTrait for core::Ptr<crate::rgbd::Kinfu_VolumeParams> {
		#[inline] fn as_raw_mut_Kinfu_VolumeParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLargeKinfu = core::Ptr<crate::rgbd::LargeKinfu>;
	
	ptr_extern! { crate::rgbd::LargeKinfu,
		cv_PtrOfLargeKinfu_delete, cv_PtrOfLargeKinfu_get_inner_ptr, cv_PtrOfLargeKinfu_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::rgbd::LargeKinfu> {
		#[inline] pub fn as_raw_PtrOfLargeKinfu(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLargeKinfu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::LargeKinfuTraitConst for core::Ptr<crate::rgbd::LargeKinfu> {
		#[inline] fn as_raw_LargeKinfu(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::LargeKinfuTrait for core::Ptr<crate::rgbd::LargeKinfu> {
		#[inline] fn as_raw_mut_LargeKinfu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLinemod_ColorGradient = core::Ptr<crate::rgbd::Linemod_ColorGradient>;
	
	ptr_extern! { crate::rgbd::Linemod_ColorGradient,
		cv_PtrOfLinemod_ColorGradient_delete, cv_PtrOfLinemod_ColorGradient_get_inner_ptr, cv_PtrOfLinemod_ColorGradient_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::Linemod_ColorGradient, cv_PtrOfLinemod_ColorGradient_new }
	
	impl core::Ptr<crate::rgbd::Linemod_ColorGradient> {
		#[inline] pub fn as_raw_PtrOfLinemod_ColorGradient(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLinemod_ColorGradient(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Linemod_ColorGradientTraitConst for core::Ptr<crate::rgbd::Linemod_ColorGradient> {
		#[inline] fn as_raw_Linemod_ColorGradient(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Linemod_ColorGradientTrait for core::Ptr<crate::rgbd::Linemod_ColorGradient> {
		#[inline] fn as_raw_mut_Linemod_ColorGradient(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::Linemod_ModalityTraitConst for core::Ptr<crate::rgbd::Linemod_ColorGradient> {
		#[inline] fn as_raw_Linemod_Modality(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Linemod_ModalityTrait for core::Ptr<crate::rgbd::Linemod_ColorGradient> {
		#[inline] fn as_raw_mut_Linemod_Modality(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLinemod_DepthNormal = core::Ptr<crate::rgbd::Linemod_DepthNormal>;
	
	ptr_extern! { crate::rgbd::Linemod_DepthNormal,
		cv_PtrOfLinemod_DepthNormal_delete, cv_PtrOfLinemod_DepthNormal_get_inner_ptr, cv_PtrOfLinemod_DepthNormal_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::Linemod_DepthNormal, cv_PtrOfLinemod_DepthNormal_new }
	
	impl core::Ptr<crate::rgbd::Linemod_DepthNormal> {
		#[inline] pub fn as_raw_PtrOfLinemod_DepthNormal(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLinemod_DepthNormal(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Linemod_DepthNormalTraitConst for core::Ptr<crate::rgbd::Linemod_DepthNormal> {
		#[inline] fn as_raw_Linemod_DepthNormal(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Linemod_DepthNormalTrait for core::Ptr<crate::rgbd::Linemod_DepthNormal> {
		#[inline] fn as_raw_mut_Linemod_DepthNormal(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::Linemod_ModalityTraitConst for core::Ptr<crate::rgbd::Linemod_DepthNormal> {
		#[inline] fn as_raw_Linemod_Modality(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Linemod_ModalityTrait for core::Ptr<crate::rgbd::Linemod_DepthNormal> {
		#[inline] fn as_raw_mut_Linemod_Modality(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLinemod_Detector = core::Ptr<crate::rgbd::Linemod_Detector>;
	
	ptr_extern! { crate::rgbd::Linemod_Detector,
		cv_PtrOfLinemod_Detector_delete, cv_PtrOfLinemod_Detector_get_inner_ptr, cv_PtrOfLinemod_Detector_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::Linemod_Detector, cv_PtrOfLinemod_Detector_new }
	
	impl core::Ptr<crate::rgbd::Linemod_Detector> {
		#[inline] pub fn as_raw_PtrOfLinemod_Detector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLinemod_Detector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Linemod_DetectorTraitConst for core::Ptr<crate::rgbd::Linemod_Detector> {
		#[inline] fn as_raw_Linemod_Detector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Linemod_DetectorTrait for core::Ptr<crate::rgbd::Linemod_Detector> {
		#[inline] fn as_raw_mut_Linemod_Detector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLinemod_Modality = core::Ptr<crate::rgbd::Linemod_Modality>;
	
	ptr_extern! { crate::rgbd::Linemod_Modality,
		cv_PtrOfLinemod_Modality_delete, cv_PtrOfLinemod_Modality_get_inner_ptr, cv_PtrOfLinemod_Modality_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::rgbd::Linemod_Modality> {
		#[inline] pub fn as_raw_PtrOfLinemod_Modality(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLinemod_Modality(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Linemod_ModalityTraitConst for core::Ptr<crate::rgbd::Linemod_Modality> {
		#[inline] fn as_raw_Linemod_Modality(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Linemod_ModalityTrait for core::Ptr<crate::rgbd::Linemod_Modality> {
		#[inline] fn as_raw_mut_Linemod_Modality(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLinemod_QuantizedPyramid = core::Ptr<crate::rgbd::Linemod_QuantizedPyramid>;
	
	ptr_extern! { crate::rgbd::Linemod_QuantizedPyramid,
		cv_PtrOfLinemod_QuantizedPyramid_delete, cv_PtrOfLinemod_QuantizedPyramid_get_inner_ptr, cv_PtrOfLinemod_QuantizedPyramid_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::rgbd::Linemod_QuantizedPyramid> {
		#[inline] pub fn as_raw_PtrOfLinemod_QuantizedPyramid(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLinemod_QuantizedPyramid(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::Linemod_QuantizedPyramidTraitConst for core::Ptr<crate::rgbd::Linemod_QuantizedPyramid> {
		#[inline] fn as_raw_Linemod_QuantizedPyramid(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::Linemod_QuantizedPyramidTrait for core::Ptr<crate::rgbd::Linemod_QuantizedPyramid> {
		#[inline] fn as_raw_mut_Linemod_QuantizedPyramid(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOdometry = core::Ptr<crate::rgbd::Odometry>;
	
	ptr_extern! { crate::rgbd::Odometry,
		cv_PtrOfOdometry_delete, cv_PtrOfOdometry_get_inner_ptr, cv_PtrOfOdometry_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::rgbd::Odometry> {
		#[inline] pub fn as_raw_PtrOfOdometry(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOdometry(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::OdometryTraitConst for core::Ptr<crate::rgbd::Odometry> {
		#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::OdometryTrait for core::Ptr<crate::rgbd::Odometry> {
		#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::rgbd::Odometry> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::rgbd::Odometry> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOdometryFrame = core::Ptr<crate::rgbd::OdometryFrame>;
	
	ptr_extern! { crate::rgbd::OdometryFrame,
		cv_PtrOfOdometryFrame_delete, cv_PtrOfOdometryFrame_get_inner_ptr, cv_PtrOfOdometryFrame_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::OdometryFrame, cv_PtrOfOdometryFrame_new }
	
	impl core::Ptr<crate::rgbd::OdometryFrame> {
		#[inline] pub fn as_raw_PtrOfOdometryFrame(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOdometryFrame(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::OdometryFrameTraitConst for core::Ptr<crate::rgbd::OdometryFrame> {
		#[inline] fn as_raw_OdometryFrame(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::OdometryFrameTrait for core::Ptr<crate::rgbd::OdometryFrame> {
		#[inline] fn as_raw_mut_OdometryFrame(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::RgbdFrameTraitConst for core::Ptr<crate::rgbd::OdometryFrame> {
		#[inline] fn as_raw_RgbdFrame(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::RgbdFrameTrait for core::Ptr<crate::rgbd::OdometryFrame> {
		#[inline] fn as_raw_mut_RgbdFrame(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfParams = core::Ptr<crate::rgbd::Params>;
	
	ptr_extern! { crate::rgbd::Params,
		cv_PtrOfParams_delete, cv_PtrOfParams_get_inner_ptr, cv_PtrOfParams_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::Params, cv_PtrOfParams_new }
	
	impl core::Ptr<crate::rgbd::Params> {
		#[inline] pub fn as_raw_PtrOfParams(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfParams(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::ParamsTraitConst for core::Ptr<crate::rgbd::Params> {
		#[inline] fn as_raw_Params(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::ParamsTrait for core::Ptr<crate::rgbd::Params> {
		#[inline] fn as_raw_mut_Params(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRgbdFrame = core::Ptr<crate::rgbd::RgbdFrame>;
	
	ptr_extern! { crate::rgbd::RgbdFrame,
		cv_PtrOfRgbdFrame_delete, cv_PtrOfRgbdFrame_get_inner_ptr, cv_PtrOfRgbdFrame_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::RgbdFrame, cv_PtrOfRgbdFrame_new }
	
	impl core::Ptr<crate::rgbd::RgbdFrame> {
		#[inline] pub fn as_raw_PtrOfRgbdFrame(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRgbdFrame(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::RgbdFrameTraitConst for core::Ptr<crate::rgbd::RgbdFrame> {
		#[inline] fn as_raw_RgbdFrame(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::RgbdFrameTrait for core::Ptr<crate::rgbd::RgbdFrame> {
		#[inline] fn as_raw_mut_RgbdFrame(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRgbdICPOdometry = core::Ptr<crate::rgbd::RgbdICPOdometry>;
	
	ptr_extern! { crate::rgbd::RgbdICPOdometry,
		cv_PtrOfRgbdICPOdometry_delete, cv_PtrOfRgbdICPOdometry_get_inner_ptr, cv_PtrOfRgbdICPOdometry_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::RgbdICPOdometry, cv_PtrOfRgbdICPOdometry_new }
	
	impl core::Ptr<crate::rgbd::RgbdICPOdometry> {
		#[inline] pub fn as_raw_PtrOfRgbdICPOdometry(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRgbdICPOdometry(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::RgbdICPOdometryTraitConst for core::Ptr<crate::rgbd::RgbdICPOdometry> {
		#[inline] fn as_raw_RgbdICPOdometry(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::RgbdICPOdometryTrait for core::Ptr<crate::rgbd::RgbdICPOdometry> {
		#[inline] fn as_raw_mut_RgbdICPOdometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::rgbd::RgbdICPOdometry> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::rgbd::RgbdICPOdometry> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::OdometryTraitConst for core::Ptr<crate::rgbd::RgbdICPOdometry> {
		#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::OdometryTrait for core::Ptr<crate::rgbd::RgbdICPOdometry> {
		#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRgbdNormals = core::Ptr<crate::rgbd::RgbdNormals>;
	
	ptr_extern! { crate::rgbd::RgbdNormals,
		cv_PtrOfRgbdNormals_delete, cv_PtrOfRgbdNormals_get_inner_ptr, cv_PtrOfRgbdNormals_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::RgbdNormals, cv_PtrOfRgbdNormals_new }
	
	impl core::Ptr<crate::rgbd::RgbdNormals> {
		#[inline] pub fn as_raw_PtrOfRgbdNormals(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRgbdNormals(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::RgbdNormalsTraitConst for core::Ptr<crate::rgbd::RgbdNormals> {
		#[inline] fn as_raw_RgbdNormals(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::RgbdNormalsTrait for core::Ptr<crate::rgbd::RgbdNormals> {
		#[inline] fn as_raw_mut_RgbdNormals(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::rgbd::RgbdNormals> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::rgbd::RgbdNormals> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRgbdOdometry = core::Ptr<crate::rgbd::RgbdOdometry>;
	
	ptr_extern! { crate::rgbd::RgbdOdometry,
		cv_PtrOfRgbdOdometry_delete, cv_PtrOfRgbdOdometry_get_inner_ptr, cv_PtrOfRgbdOdometry_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::RgbdOdometry, cv_PtrOfRgbdOdometry_new }
	
	impl core::Ptr<crate::rgbd::RgbdOdometry> {
		#[inline] pub fn as_raw_PtrOfRgbdOdometry(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRgbdOdometry(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::RgbdOdometryTraitConst for core::Ptr<crate::rgbd::RgbdOdometry> {
		#[inline] fn as_raw_RgbdOdometry(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::RgbdOdometryTrait for core::Ptr<crate::rgbd::RgbdOdometry> {
		#[inline] fn as_raw_mut_RgbdOdometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::rgbd::RgbdOdometry> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::rgbd::RgbdOdometry> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::rgbd::OdometryTraitConst for core::Ptr<crate::rgbd::RgbdOdometry> {
		#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::OdometryTrait for core::Ptr<crate::rgbd::RgbdOdometry> {
		#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRgbdPlane = core::Ptr<crate::rgbd::RgbdPlane>;
	
	ptr_extern! { crate::rgbd::RgbdPlane,
		cv_PtrOfRgbdPlane_delete, cv_PtrOfRgbdPlane_get_inner_ptr, cv_PtrOfRgbdPlane_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::rgbd::RgbdPlane, cv_PtrOfRgbdPlane_new }
	
	impl core::Ptr<crate::rgbd::RgbdPlane> {
		#[inline] pub fn as_raw_PtrOfRgbdPlane(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRgbdPlane(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::rgbd::RgbdPlaneTraitConst for core::Ptr<crate::rgbd::RgbdPlane> {
		#[inline] fn as_raw_RgbdPlane(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::rgbd::RgbdPlaneTrait for core::Ptr<crate::rgbd::RgbdPlane> {
		#[inline] fn as_raw_mut_RgbdPlane(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::rgbd::RgbdPlane> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::rgbd::RgbdPlane> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfLinemod_Feature = core::Vector<crate::rgbd::Linemod_Feature>;
	
	impl core::Vector<crate::rgbd::Linemod_Feature> {
		pub fn as_raw_VectorOfLinemod_Feature(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfLinemod_Feature(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::rgbd::Linemod_Feature,
		cv_VectorOfLinemod_Feature_new, cv_VectorOfLinemod_Feature_delete,
		cv_VectorOfLinemod_Feature_len, cv_VectorOfLinemod_Feature_is_empty,
		cv_VectorOfLinemod_Feature_capacity, cv_VectorOfLinemod_Feature_shrink_to_fit,
		cv_VectorOfLinemod_Feature_reserve, cv_VectorOfLinemod_Feature_remove,
		cv_VectorOfLinemod_Feature_swap, cv_VectorOfLinemod_Feature_clear,
		cv_VectorOfLinemod_Feature_get, cv_VectorOfLinemod_Feature_set,
		cv_VectorOfLinemod_Feature_push, cv_VectorOfLinemod_Feature_insert,
	}
	vector_copy_non_bool! { crate::rgbd::Linemod_Feature,
		cv_VectorOfLinemod_Feature_data, cv_VectorOfLinemod_Feature_data_mut, cv_VectorOfLinemod_Feature_from_slice,
		cv_VectorOfLinemod_Feature_clone,
	}
	
	pub type VectorOfLinemod_Match = core::Vector<crate::rgbd::Linemod_Match>;
	
	impl core::Vector<crate::rgbd::Linemod_Match> {
		pub fn as_raw_VectorOfLinemod_Match(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfLinemod_Match(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::rgbd::Linemod_Match,
		cv_VectorOfLinemod_Match_new, cv_VectorOfLinemod_Match_delete,
		cv_VectorOfLinemod_Match_len, cv_VectorOfLinemod_Match_is_empty,
		cv_VectorOfLinemod_Match_capacity, cv_VectorOfLinemod_Match_shrink_to_fit,
		cv_VectorOfLinemod_Match_reserve, cv_VectorOfLinemod_Match_remove,
		cv_VectorOfLinemod_Match_swap, cv_VectorOfLinemod_Match_clear,
		cv_VectorOfLinemod_Match_get, cv_VectorOfLinemod_Match_set,
		cv_VectorOfLinemod_Match_push, cv_VectorOfLinemod_Match_insert,
	}
	vector_non_copy_or_bool! { clone crate::rgbd::Linemod_Match }
	
	pub type VectorOfLinemod_Template = core::Vector<crate::rgbd::Linemod_Template>;
	
	impl core::Vector<crate::rgbd::Linemod_Template> {
		pub fn as_raw_VectorOfLinemod_Template(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfLinemod_Template(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::rgbd::Linemod_Template,
		cv_VectorOfLinemod_Template_new, cv_VectorOfLinemod_Template_delete,
		cv_VectorOfLinemod_Template_len, cv_VectorOfLinemod_Template_is_empty,
		cv_VectorOfLinemod_Template_capacity, cv_VectorOfLinemod_Template_shrink_to_fit,
		cv_VectorOfLinemod_Template_reserve, cv_VectorOfLinemod_Template_remove,
		cv_VectorOfLinemod_Template_swap, cv_VectorOfLinemod_Template_clear,
		cv_VectorOfLinemod_Template_get, cv_VectorOfLinemod_Template_set,
		cv_VectorOfLinemod_Template_push, cv_VectorOfLinemod_Template_insert,
	}
	vector_non_copy_or_bool! { clone crate::rgbd::Linemod_Template }
	
	pub type VectorOfPtrOfLinemod_Modality = core::Vector<core::Ptr<crate::rgbd::Linemod_Modality>>;
	
	impl core::Vector<core::Ptr<crate::rgbd::Linemod_Modality>> {
		pub fn as_raw_VectorOfPtrOfLinemod_Modality(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfLinemod_Modality(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Ptr<crate::rgbd::Linemod_Modality>,
		cv_VectorOfPtrOfLinemod_Modality_new, cv_VectorOfPtrOfLinemod_Modality_delete,
		cv_VectorOfPtrOfLinemod_Modality_len, cv_VectorOfPtrOfLinemod_Modality_is_empty,
		cv_VectorOfPtrOfLinemod_Modality_capacity, cv_VectorOfPtrOfLinemod_Modality_shrink_to_fit,
		cv_VectorOfPtrOfLinemod_Modality_reserve, cv_VectorOfPtrOfLinemod_Modality_remove,
		cv_VectorOfPtrOfLinemod_Modality_swap, cv_VectorOfPtrOfLinemod_Modality_clear,
		cv_VectorOfPtrOfLinemod_Modality_get, cv_VectorOfPtrOfLinemod_Modality_set,
		cv_VectorOfPtrOfLinemod_Modality_push, cv_VectorOfPtrOfLinemod_Modality_insert,
	}
	vector_non_copy_or_bool! { core::Ptr<crate::rgbd::Linemod_Modality> }
	
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
	
	impl core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014> {
		#[inline] pub fn as_raw_PtrOfMotionSaliencyBinWangApr2014(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMotionSaliencyBinWangApr2014(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::saliency::MotionSaliencyBinWangApr2014TraitConst for core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014> {
		#[inline] fn as_raw_MotionSaliencyBinWangApr2014(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::MotionSaliencyBinWangApr2014Trait for core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014> {
		#[inline] fn as_raw_mut_MotionSaliencyBinWangApr2014(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::MotionSaliencyTraitConst for core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014> {
		#[inline] fn as_raw_MotionSaliency(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::MotionSaliencyTrait for core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014> {
		#[inline] fn as_raw_mut_MotionSaliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::SaliencyTraitConst for core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014> {
		#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::SaliencyTrait for core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014> {
		#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfObjectnessBING = core::Ptr<crate::saliency::ObjectnessBING>;
	
	ptr_extern! { crate::saliency::ObjectnessBING,
		cv_PtrOfObjectnessBING_delete, cv_PtrOfObjectnessBING_get_inner_ptr, cv_PtrOfObjectnessBING_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::saliency::ObjectnessBING, cv_PtrOfObjectnessBING_new }
	
	impl core::Ptr<crate::saliency::ObjectnessBING> {
		#[inline] pub fn as_raw_PtrOfObjectnessBING(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfObjectnessBING(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::saliency::ObjectnessBINGTraitConst for core::Ptr<crate::saliency::ObjectnessBING> {
		#[inline] fn as_raw_ObjectnessBING(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::ObjectnessBINGTrait for core::Ptr<crate::saliency::ObjectnessBING> {
		#[inline] fn as_raw_mut_ObjectnessBING(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::saliency::ObjectnessBING> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::saliency::ObjectnessBING> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::ObjectnessTraitConst for core::Ptr<crate::saliency::ObjectnessBING> {
		#[inline] fn as_raw_Objectness(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::ObjectnessTrait for core::Ptr<crate::saliency::ObjectnessBING> {
		#[inline] fn as_raw_mut_Objectness(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::SaliencyTraitConst for core::Ptr<crate::saliency::ObjectnessBING> {
		#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::SaliencyTrait for core::Ptr<crate::saliency::ObjectnessBING> {
		#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStaticSaliencyFineGrained = core::Ptr<crate::saliency::StaticSaliencyFineGrained>;
	
	ptr_extern! { crate::saliency::StaticSaliencyFineGrained,
		cv_PtrOfStaticSaliencyFineGrained_delete, cv_PtrOfStaticSaliencyFineGrained_get_inner_ptr, cv_PtrOfStaticSaliencyFineGrained_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::saliency::StaticSaliencyFineGrained, cv_PtrOfStaticSaliencyFineGrained_new }
	
	impl core::Ptr<crate::saliency::StaticSaliencyFineGrained> {
		#[inline] pub fn as_raw_PtrOfStaticSaliencyFineGrained(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStaticSaliencyFineGrained(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::saliency::StaticSaliencyFineGrainedTraitConst for core::Ptr<crate::saliency::StaticSaliencyFineGrained> {
		#[inline] fn as_raw_StaticSaliencyFineGrained(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::StaticSaliencyFineGrainedTrait for core::Ptr<crate::saliency::StaticSaliencyFineGrained> {
		#[inline] fn as_raw_mut_StaticSaliencyFineGrained(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::saliency::StaticSaliencyFineGrained> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::saliency::StaticSaliencyFineGrained> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::SaliencyTraitConst for core::Ptr<crate::saliency::StaticSaliencyFineGrained> {
		#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::SaliencyTrait for core::Ptr<crate::saliency::StaticSaliencyFineGrained> {
		#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::StaticSaliencyTraitConst for core::Ptr<crate::saliency::StaticSaliencyFineGrained> {
		#[inline] fn as_raw_StaticSaliency(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::StaticSaliencyTrait for core::Ptr<crate::saliency::StaticSaliencyFineGrained> {
		#[inline] fn as_raw_mut_StaticSaliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStaticSaliencySpectralResidual = core::Ptr<crate::saliency::StaticSaliencySpectralResidual>;
	
	ptr_extern! { crate::saliency::StaticSaliencySpectralResidual,
		cv_PtrOfStaticSaliencySpectralResidual_delete, cv_PtrOfStaticSaliencySpectralResidual_get_inner_ptr, cv_PtrOfStaticSaliencySpectralResidual_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::saliency::StaticSaliencySpectralResidual, cv_PtrOfStaticSaliencySpectralResidual_new }
	
	impl core::Ptr<crate::saliency::StaticSaliencySpectralResidual> {
		#[inline] pub fn as_raw_PtrOfStaticSaliencySpectralResidual(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStaticSaliencySpectralResidual(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::saliency::StaticSaliencySpectralResidualTraitConst for core::Ptr<crate::saliency::StaticSaliencySpectralResidual> {
		#[inline] fn as_raw_StaticSaliencySpectralResidual(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::StaticSaliencySpectralResidualTrait for core::Ptr<crate::saliency::StaticSaliencySpectralResidual> {
		#[inline] fn as_raw_mut_StaticSaliencySpectralResidual(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::saliency::StaticSaliencySpectralResidual> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::saliency::StaticSaliencySpectralResidual> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::SaliencyTraitConst for core::Ptr<crate::saliency::StaticSaliencySpectralResidual> {
		#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::SaliencyTrait for core::Ptr<crate::saliency::StaticSaliencySpectralResidual> {
		#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::saliency::StaticSaliencyTraitConst for core::Ptr<crate::saliency::StaticSaliencySpectralResidual> {
		#[inline] fn as_raw_StaticSaliency(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::saliency::StaticSaliencyTrait for core::Ptr<crate::saliency::StaticSaliencySpectralResidual> {
		#[inline] fn as_raw_mut_StaticSaliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_saliency)]
pub use saliency_types::*;

#[cfg(ocvrs_has_module_sfm)]
mod sfm_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfSFMLibmvEuclideanReconstruction = core::Ptr<crate::sfm::SFMLibmvEuclideanReconstruction>;
	
	ptr_extern! { crate::sfm::SFMLibmvEuclideanReconstruction,
		cv_PtrOfSFMLibmvEuclideanReconstruction_delete, cv_PtrOfSFMLibmvEuclideanReconstruction_get_inner_ptr, cv_PtrOfSFMLibmvEuclideanReconstruction_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::sfm::SFMLibmvEuclideanReconstruction> {
		#[inline] pub fn as_raw_PtrOfSFMLibmvEuclideanReconstruction(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSFMLibmvEuclideanReconstruction(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::sfm::SFMLibmvEuclideanReconstructionTraitConst for core::Ptr<crate::sfm::SFMLibmvEuclideanReconstruction> {
		#[inline] fn as_raw_SFMLibmvEuclideanReconstruction(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::sfm::SFMLibmvEuclideanReconstructionTrait for core::Ptr<crate::sfm::SFMLibmvEuclideanReconstruction> {
		#[inline] fn as_raw_mut_SFMLibmvEuclideanReconstruction(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::sfm::BaseSFMTraitConst for core::Ptr<crate::sfm::SFMLibmvEuclideanReconstruction> {
		#[inline] fn as_raw_BaseSFM(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::sfm::BaseSFMTrait for core::Ptr<crate::sfm::SFMLibmvEuclideanReconstruction> {
		#[inline] fn as_raw_mut_BaseSFM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_sfm)]
pub use sfm_types::*;

#[cfg(ocvrs_has_module_shape)]
mod shape_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAffineTransformer = core::Ptr<crate::shape::AffineTransformer>;
	
	ptr_extern! { crate::shape::AffineTransformer,
		cv_PtrOfAffineTransformer_delete, cv_PtrOfAffineTransformer_get_inner_ptr, cv_PtrOfAffineTransformer_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::shape::AffineTransformer> {
		#[inline] pub fn as_raw_PtrOfAffineTransformer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAffineTransformer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::shape::AffineTransformerTraitConst for core::Ptr<crate::shape::AffineTransformer> {
		#[inline] fn as_raw_AffineTransformer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::AffineTransformerTrait for core::Ptr<crate::shape::AffineTransformer> {
		#[inline] fn as_raw_mut_AffineTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::shape::AffineTransformer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::shape::AffineTransformer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::ShapeTransformerTraitConst for core::Ptr<crate::shape::AffineTransformer> {
		#[inline] fn as_raw_ShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::ShapeTransformerTrait for core::Ptr<crate::shape::AffineTransformer> {
		#[inline] fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfAffineTransformer, core::Ptr<crate::shape::ShapeTransformer>, cv_PtrOfAffineTransformer_to_PtrOfShapeTransformer }
	
	pub type PtrOfChiHistogramCostExtractor = core::Ptr<crate::shape::ChiHistogramCostExtractor>;
	
	ptr_extern! { crate::shape::ChiHistogramCostExtractor,
		cv_PtrOfChiHistogramCostExtractor_delete, cv_PtrOfChiHistogramCostExtractor_get_inner_ptr, cv_PtrOfChiHistogramCostExtractor_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::shape::ChiHistogramCostExtractor> {
		#[inline] pub fn as_raw_PtrOfChiHistogramCostExtractor(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfChiHistogramCostExtractor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::shape::ChiHistogramCostExtractorTraitConst for core::Ptr<crate::shape::ChiHistogramCostExtractor> {
		#[inline] fn as_raw_ChiHistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::ChiHistogramCostExtractorTrait for core::Ptr<crate::shape::ChiHistogramCostExtractor> {
		#[inline] fn as_raw_mut_ChiHistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::shape::ChiHistogramCostExtractor> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::shape::ChiHistogramCostExtractor> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::HistogramCostExtractorTraitConst for core::Ptr<crate::shape::ChiHistogramCostExtractor> {
		#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::HistogramCostExtractorTrait for core::Ptr<crate::shape::ChiHistogramCostExtractor> {
		#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfChiHistogramCostExtractor, core::Ptr<crate::shape::HistogramCostExtractor>, cv_PtrOfChiHistogramCostExtractor_to_PtrOfHistogramCostExtractor }
	
	pub type PtrOfEMDHistogramCostExtractor = core::Ptr<crate::shape::EMDHistogramCostExtractor>;
	
	ptr_extern! { crate::shape::EMDHistogramCostExtractor,
		cv_PtrOfEMDHistogramCostExtractor_delete, cv_PtrOfEMDHistogramCostExtractor_get_inner_ptr, cv_PtrOfEMDHistogramCostExtractor_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::shape::EMDHistogramCostExtractor> {
		#[inline] pub fn as_raw_PtrOfEMDHistogramCostExtractor(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEMDHistogramCostExtractor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::shape::EMDHistogramCostExtractorTraitConst for core::Ptr<crate::shape::EMDHistogramCostExtractor> {
		#[inline] fn as_raw_EMDHistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::EMDHistogramCostExtractorTrait for core::Ptr<crate::shape::EMDHistogramCostExtractor> {
		#[inline] fn as_raw_mut_EMDHistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::shape::EMDHistogramCostExtractor> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::shape::EMDHistogramCostExtractor> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::HistogramCostExtractorTraitConst for core::Ptr<crate::shape::EMDHistogramCostExtractor> {
		#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::HistogramCostExtractorTrait for core::Ptr<crate::shape::EMDHistogramCostExtractor> {
		#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfEMDHistogramCostExtractor, core::Ptr<crate::shape::HistogramCostExtractor>, cv_PtrOfEMDHistogramCostExtractor_to_PtrOfHistogramCostExtractor }
	
	pub type PtrOfEMDL1HistogramCostExtractor = core::Ptr<crate::shape::EMDL1HistogramCostExtractor>;
	
	ptr_extern! { crate::shape::EMDL1HistogramCostExtractor,
		cv_PtrOfEMDL1HistogramCostExtractor_delete, cv_PtrOfEMDL1HistogramCostExtractor_get_inner_ptr, cv_PtrOfEMDL1HistogramCostExtractor_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::shape::EMDL1HistogramCostExtractor> {
		#[inline] pub fn as_raw_PtrOfEMDL1HistogramCostExtractor(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEMDL1HistogramCostExtractor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::shape::EMDL1HistogramCostExtractorTraitConst for core::Ptr<crate::shape::EMDL1HistogramCostExtractor> {
		#[inline] fn as_raw_EMDL1HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::EMDL1HistogramCostExtractorTrait for core::Ptr<crate::shape::EMDL1HistogramCostExtractor> {
		#[inline] fn as_raw_mut_EMDL1HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::shape::EMDL1HistogramCostExtractor> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::shape::EMDL1HistogramCostExtractor> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::HistogramCostExtractorTraitConst for core::Ptr<crate::shape::EMDL1HistogramCostExtractor> {
		#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::HistogramCostExtractorTrait for core::Ptr<crate::shape::EMDL1HistogramCostExtractor> {
		#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfEMDL1HistogramCostExtractor, core::Ptr<crate::shape::HistogramCostExtractor>, cv_PtrOfEMDL1HistogramCostExtractor_to_PtrOfHistogramCostExtractor }
	
	pub type PtrOfHausdorffDistanceExtractor = core::Ptr<crate::shape::HausdorffDistanceExtractor>;
	
	ptr_extern! { crate::shape::HausdorffDistanceExtractor,
		cv_PtrOfHausdorffDistanceExtractor_delete, cv_PtrOfHausdorffDistanceExtractor_get_inner_ptr, cv_PtrOfHausdorffDistanceExtractor_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::shape::HausdorffDistanceExtractor> {
		#[inline] pub fn as_raw_PtrOfHausdorffDistanceExtractor(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHausdorffDistanceExtractor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::shape::HausdorffDistanceExtractorTraitConst for core::Ptr<crate::shape::HausdorffDistanceExtractor> {
		#[inline] fn as_raw_HausdorffDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::HausdorffDistanceExtractorTrait for core::Ptr<crate::shape::HausdorffDistanceExtractor> {
		#[inline] fn as_raw_mut_HausdorffDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::shape::HausdorffDistanceExtractor> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::shape::HausdorffDistanceExtractor> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::ShapeDistanceExtractorTraitConst for core::Ptr<crate::shape::HausdorffDistanceExtractor> {
		#[inline] fn as_raw_ShapeDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::ShapeDistanceExtractorTrait for core::Ptr<crate::shape::HausdorffDistanceExtractor> {
		#[inline] fn as_raw_mut_ShapeDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfHistogramCostExtractor = core::Ptr<crate::shape::HistogramCostExtractor>;
	
	ptr_extern! { crate::shape::HistogramCostExtractor,
		cv_PtrOfHistogramCostExtractor_delete, cv_PtrOfHistogramCostExtractor_get_inner_ptr, cv_PtrOfHistogramCostExtractor_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::shape::HistogramCostExtractor> {
		#[inline] pub fn as_raw_PtrOfHistogramCostExtractor(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHistogramCostExtractor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::shape::HistogramCostExtractorTraitConst for core::Ptr<crate::shape::HistogramCostExtractor> {
		#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::HistogramCostExtractorTrait for core::Ptr<crate::shape::HistogramCostExtractor> {
		#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::shape::HistogramCostExtractor> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::shape::HistogramCostExtractor> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfNormHistogramCostExtractor = core::Ptr<crate::shape::NormHistogramCostExtractor>;
	
	ptr_extern! { crate::shape::NormHistogramCostExtractor,
		cv_PtrOfNormHistogramCostExtractor_delete, cv_PtrOfNormHistogramCostExtractor_get_inner_ptr, cv_PtrOfNormHistogramCostExtractor_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::shape::NormHistogramCostExtractor> {
		#[inline] pub fn as_raw_PtrOfNormHistogramCostExtractor(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNormHistogramCostExtractor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::shape::NormHistogramCostExtractorTraitConst for core::Ptr<crate::shape::NormHistogramCostExtractor> {
		#[inline] fn as_raw_NormHistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::NormHistogramCostExtractorTrait for core::Ptr<crate::shape::NormHistogramCostExtractor> {
		#[inline] fn as_raw_mut_NormHistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::shape::NormHistogramCostExtractor> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::shape::NormHistogramCostExtractor> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::HistogramCostExtractorTraitConst for core::Ptr<crate::shape::NormHistogramCostExtractor> {
		#[inline] fn as_raw_HistogramCostExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::HistogramCostExtractorTrait for core::Ptr<crate::shape::NormHistogramCostExtractor> {
		#[inline] fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfNormHistogramCostExtractor, core::Ptr<crate::shape::HistogramCostExtractor>, cv_PtrOfNormHistogramCostExtractor_to_PtrOfHistogramCostExtractor }
	
	pub type PtrOfShapeContextDistanceExtractor = core::Ptr<crate::shape::ShapeContextDistanceExtractor>;
	
	ptr_extern! { crate::shape::ShapeContextDistanceExtractor,
		cv_PtrOfShapeContextDistanceExtractor_delete, cv_PtrOfShapeContextDistanceExtractor_get_inner_ptr, cv_PtrOfShapeContextDistanceExtractor_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::shape::ShapeContextDistanceExtractor> {
		#[inline] pub fn as_raw_PtrOfShapeContextDistanceExtractor(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfShapeContextDistanceExtractor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::shape::ShapeContextDistanceExtractorTraitConst for core::Ptr<crate::shape::ShapeContextDistanceExtractor> {
		#[inline] fn as_raw_ShapeContextDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::ShapeContextDistanceExtractorTrait for core::Ptr<crate::shape::ShapeContextDistanceExtractor> {
		#[inline] fn as_raw_mut_ShapeContextDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::shape::ShapeContextDistanceExtractor> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::shape::ShapeContextDistanceExtractor> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::ShapeDistanceExtractorTraitConst for core::Ptr<crate::shape::ShapeContextDistanceExtractor> {
		#[inline] fn as_raw_ShapeDistanceExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::ShapeDistanceExtractorTrait for core::Ptr<crate::shape::ShapeContextDistanceExtractor> {
		#[inline] fn as_raw_mut_ShapeDistanceExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfShapeTransformer = core::Ptr<crate::shape::ShapeTransformer>;
	
	ptr_extern! { crate::shape::ShapeTransformer,
		cv_PtrOfShapeTransformer_delete, cv_PtrOfShapeTransformer_get_inner_ptr, cv_PtrOfShapeTransformer_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::shape::ShapeTransformer> {
		#[inline] pub fn as_raw_PtrOfShapeTransformer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfShapeTransformer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::shape::ShapeTransformerTraitConst for core::Ptr<crate::shape::ShapeTransformer> {
		#[inline] fn as_raw_ShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::ShapeTransformerTrait for core::Ptr<crate::shape::ShapeTransformer> {
		#[inline] fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::shape::ShapeTransformer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::shape::ShapeTransformer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfThinPlateSplineShapeTransformer = core::Ptr<crate::shape::ThinPlateSplineShapeTransformer>;
	
	ptr_extern! { crate::shape::ThinPlateSplineShapeTransformer,
		cv_PtrOfThinPlateSplineShapeTransformer_delete, cv_PtrOfThinPlateSplineShapeTransformer_get_inner_ptr, cv_PtrOfThinPlateSplineShapeTransformer_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::shape::ThinPlateSplineShapeTransformer> {
		#[inline] pub fn as_raw_PtrOfThinPlateSplineShapeTransformer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfThinPlateSplineShapeTransformer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::shape::ThinPlateSplineShapeTransformerTraitConst for core::Ptr<crate::shape::ThinPlateSplineShapeTransformer> {
		#[inline] fn as_raw_ThinPlateSplineShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::ThinPlateSplineShapeTransformerTrait for core::Ptr<crate::shape::ThinPlateSplineShapeTransformer> {
		#[inline] fn as_raw_mut_ThinPlateSplineShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::shape::ThinPlateSplineShapeTransformer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::shape::ThinPlateSplineShapeTransformer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::shape::ShapeTransformerTraitConst for core::Ptr<crate::shape::ThinPlateSplineShapeTransformer> {
		#[inline] fn as_raw_ShapeTransformer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::shape::ShapeTransformerTrait for core::Ptr<crate::shape::ThinPlateSplineShapeTransformer> {
		#[inline] fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfThinPlateSplineShapeTransformer, core::Ptr<crate::shape::ShapeTransformer>, cv_PtrOfThinPlateSplineShapeTransformer_to_PtrOfShapeTransformer }
	
}
#[cfg(ocvrs_has_module_shape)]
pub use shape_types::*;

#[cfg(ocvrs_has_module_stereo)]
mod stereo_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfQuasiDenseStereo = core::Ptr<crate::stereo::QuasiDenseStereo>;
	
	ptr_extern! { crate::stereo::QuasiDenseStereo,
		cv_PtrOfQuasiDenseStereo_delete, cv_PtrOfQuasiDenseStereo_get_inner_ptr, cv_PtrOfQuasiDenseStereo_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::stereo::QuasiDenseStereo> {
		#[inline] pub fn as_raw_PtrOfQuasiDenseStereo(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfQuasiDenseStereo(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stereo::QuasiDenseStereoTraitConst for core::Ptr<crate::stereo::QuasiDenseStereo> {
		#[inline] fn as_raw_QuasiDenseStereo(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stereo::QuasiDenseStereoTrait for core::Ptr<crate::stereo::QuasiDenseStereo> {
		#[inline] fn as_raw_mut_QuasiDenseStereo(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfMatchQuasiDense = core::Vector<crate::stereo::MatchQuasiDense>;
	
	impl core::Vector<crate::stereo::MatchQuasiDense> {
		pub fn as_raw_VectorOfMatchQuasiDense(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfMatchQuasiDense(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::stereo::MatchQuasiDense,
		cv_VectorOfMatchQuasiDense_new, cv_VectorOfMatchQuasiDense_delete,
		cv_VectorOfMatchQuasiDense_len, cv_VectorOfMatchQuasiDense_is_empty,
		cv_VectorOfMatchQuasiDense_capacity, cv_VectorOfMatchQuasiDense_shrink_to_fit,
		cv_VectorOfMatchQuasiDense_reserve, cv_VectorOfMatchQuasiDense_remove,
		cv_VectorOfMatchQuasiDense_swap, cv_VectorOfMatchQuasiDense_clear,
		cv_VectorOfMatchQuasiDense_get, cv_VectorOfMatchQuasiDense_set,
		cv_VectorOfMatchQuasiDense_push, cv_VectorOfMatchQuasiDense_insert,
	}
	vector_copy_non_bool! { crate::stereo::MatchQuasiDense,
		cv_VectorOfMatchQuasiDense_data, cv_VectorOfMatchQuasiDense_data_mut, cv_VectorOfMatchQuasiDense_from_slice,
		cv_VectorOfMatchQuasiDense_clone,
	}
	
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
	
	impl core::Ptr<crate::stitching::AffineWarper> {
		#[inline] pub fn as_raw_PtrOfAffineWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAffineWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::AffineWarperTraitConst for core::Ptr<crate::stitching::AffineWarper> {
		#[inline] fn as_raw_AffineWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::AffineWarperTrait for core::Ptr<crate::stitching::AffineWarper> {
		#[inline] fn as_raw_mut_AffineWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::AffineWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::AffineWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfAffineWarper, core::Ptr<crate::stitching::WarperCreator>, cv_PtrOfAffineWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfCompressedRectilinearPortraitWarper = core::Ptr<crate::stitching::CompressedRectilinearPortraitWarper>;
	
	ptr_extern! { crate::stitching::CompressedRectilinearPortraitWarper,
		cv_PtrOfCompressedRectilinearPortraitWarper_delete, cv_PtrOfCompressedRectilinearPortraitWarper_get_inner_ptr, cv_PtrOfCompressedRectilinearPortraitWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::CompressedRectilinearPortraitWarper, cv_PtrOfCompressedRectilinearPortraitWarper_new }
	
	impl core::Ptr<crate::stitching::CompressedRectilinearPortraitWarper> {
		#[inline] pub fn as_raw_PtrOfCompressedRectilinearPortraitWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCompressedRectilinearPortraitWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::CompressedRectilinearPortraitWarperTraitConst for core::Ptr<crate::stitching::CompressedRectilinearPortraitWarper> {
		#[inline] fn as_raw_CompressedRectilinearPortraitWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::CompressedRectilinearPortraitWarperTrait for core::Ptr<crate::stitching::CompressedRectilinearPortraitWarper> {
		#[inline] fn as_raw_mut_CompressedRectilinearPortraitWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::CompressedRectilinearPortraitWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::CompressedRectilinearPortraitWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfCompressedRectilinearPortraitWarper, core::Ptr<crate::stitching::WarperCreator>, cv_PtrOfCompressedRectilinearPortraitWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfCompressedRectilinearWarper = core::Ptr<crate::stitching::CompressedRectilinearWarper>;
	
	ptr_extern! { crate::stitching::CompressedRectilinearWarper,
		cv_PtrOfCompressedRectilinearWarper_delete, cv_PtrOfCompressedRectilinearWarper_get_inner_ptr, cv_PtrOfCompressedRectilinearWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::CompressedRectilinearWarper, cv_PtrOfCompressedRectilinearWarper_new }
	
	impl core::Ptr<crate::stitching::CompressedRectilinearWarper> {
		#[inline] pub fn as_raw_PtrOfCompressedRectilinearWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCompressedRectilinearWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::CompressedRectilinearWarperTraitConst for core::Ptr<crate::stitching::CompressedRectilinearWarper> {
		#[inline] fn as_raw_CompressedRectilinearWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::CompressedRectilinearWarperTrait for core::Ptr<crate::stitching::CompressedRectilinearWarper> {
		#[inline] fn as_raw_mut_CompressedRectilinearWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::CompressedRectilinearWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::CompressedRectilinearWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfCompressedRectilinearWarper, core::Ptr<crate::stitching::WarperCreator>, cv_PtrOfCompressedRectilinearWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfCylindricalWarper = core::Ptr<crate::stitching::CylindricalWarper>;
	
	ptr_extern! { crate::stitching::CylindricalWarper,
		cv_PtrOfCylindricalWarper_delete, cv_PtrOfCylindricalWarper_get_inner_ptr, cv_PtrOfCylindricalWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::CylindricalWarper, cv_PtrOfCylindricalWarper_new }
	
	impl core::Ptr<crate::stitching::CylindricalWarper> {
		#[inline] pub fn as_raw_PtrOfCylindricalWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCylindricalWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::CylindricalWarperTraitConst for core::Ptr<crate::stitching::CylindricalWarper> {
		#[inline] fn as_raw_CylindricalWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::CylindricalWarperTrait for core::Ptr<crate::stitching::CylindricalWarper> {
		#[inline] fn as_raw_mut_CylindricalWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::CylindricalWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::CylindricalWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfCylindricalWarper, core::Ptr<crate::stitching::WarperCreator>, cv_PtrOfCylindricalWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfCylindricalWarperGpu = core::Ptr<crate::stitching::CylindricalWarperGpu>;
	
	ptr_extern! { crate::stitching::CylindricalWarperGpu,
		cv_PtrOfCylindricalWarperGpu_delete, cv_PtrOfCylindricalWarperGpu_get_inner_ptr, cv_PtrOfCylindricalWarperGpu_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::CylindricalWarperGpu, cv_PtrOfCylindricalWarperGpu_new }
	
	impl core::Ptr<crate::stitching::CylindricalWarperGpu> {
		#[inline] pub fn as_raw_PtrOfCylindricalWarperGpu(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCylindricalWarperGpu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::CylindricalWarperGpuTraitConst for core::Ptr<crate::stitching::CylindricalWarperGpu> {
		#[inline] fn as_raw_CylindricalWarperGpu(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::CylindricalWarperGpuTrait for core::Ptr<crate::stitching::CylindricalWarperGpu> {
		#[inline] fn as_raw_mut_CylindricalWarperGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::CylindricalWarperGpu> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::CylindricalWarperGpu> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfCylindricalWarperGpu, core::Ptr<crate::stitching::WarperCreator>, cv_PtrOfCylindricalWarperGpu_to_PtrOfWarperCreator }
	
	pub type PtrOfDetail_AffineBasedEstimator = core::Ptr<crate::stitching::Detail_AffineBasedEstimator>;
	
	ptr_extern! { crate::stitching::Detail_AffineBasedEstimator,
		cv_PtrOfDetail_AffineBasedEstimator_delete, cv_PtrOfDetail_AffineBasedEstimator_get_inner_ptr, cv_PtrOfDetail_AffineBasedEstimator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_AffineBasedEstimator, cv_PtrOfDetail_AffineBasedEstimator_new }
	
	impl core::Ptr<crate::stitching::Detail_AffineBasedEstimator> {
		#[inline] pub fn as_raw_PtrOfDetail_AffineBasedEstimator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_AffineBasedEstimator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_AffineBasedEstimatorTraitConst for core::Ptr<crate::stitching::Detail_AffineBasedEstimator> {
		#[inline] fn as_raw_Detail_AffineBasedEstimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_AffineBasedEstimatorTrait for core::Ptr<crate::stitching::Detail_AffineBasedEstimator> {
		#[inline] fn as_raw_mut_Detail_AffineBasedEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_EstimatorTraitConst for core::Ptr<crate::stitching::Detail_AffineBasedEstimator> {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_EstimatorTrait for core::Ptr<crate::stitching::Detail_AffineBasedEstimator> {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_AffineBasedEstimator, core::Ptr<crate::stitching::Detail_Estimator>, cv_PtrOfDetail_AffineBasedEstimator_to_PtrOfDetail_Estimator }
	
	pub type PtrOfDetail_AffineBestOf2NearestMatcher = core::Ptr<crate::stitching::Detail_AffineBestOf2NearestMatcher>;
	
	ptr_extern! { crate::stitching::Detail_AffineBestOf2NearestMatcher,
		cv_PtrOfDetail_AffineBestOf2NearestMatcher_delete, cv_PtrOfDetail_AffineBestOf2NearestMatcher_get_inner_ptr, cv_PtrOfDetail_AffineBestOf2NearestMatcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_AffineBestOf2NearestMatcher, cv_PtrOfDetail_AffineBestOf2NearestMatcher_new }
	
	impl core::Ptr<crate::stitching::Detail_AffineBestOf2NearestMatcher> {
		#[inline] pub fn as_raw_PtrOfDetail_AffineBestOf2NearestMatcher(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_AffineBestOf2NearestMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_AffineBestOf2NearestMatcherTraitConst for core::Ptr<crate::stitching::Detail_AffineBestOf2NearestMatcher> {
		#[inline] fn as_raw_Detail_AffineBestOf2NearestMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_AffineBestOf2NearestMatcherTrait for core::Ptr<crate::stitching::Detail_AffineBestOf2NearestMatcher> {
		#[inline] fn as_raw_mut_Detail_AffineBestOf2NearestMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BestOf2NearestMatcherTraitConst for core::Ptr<crate::stitching::Detail_AffineBestOf2NearestMatcher> {
		#[inline] fn as_raw_Detail_BestOf2NearestMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BestOf2NearestMatcherTrait for core::Ptr<crate::stitching::Detail_AffineBestOf2NearestMatcher> {
		#[inline] fn as_raw_mut_Detail_BestOf2NearestMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_AffineBestOf2NearestMatcher, core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher>, cv_PtrOfDetail_AffineBestOf2NearestMatcher_to_PtrOfDetail_BestOf2NearestMatcher }
	
	impl crate::stitching::Detail_FeaturesMatcherTraitConst for core::Ptr<crate::stitching::Detail_AffineBestOf2NearestMatcher> {
		#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_FeaturesMatcherTrait for core::Ptr<crate::stitching::Detail_AffineBestOf2NearestMatcher> {
		#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_AffineBestOf2NearestMatcher, core::Ptr<crate::stitching::Detail_FeaturesMatcher>, cv_PtrOfDetail_AffineBestOf2NearestMatcher_to_PtrOfDetail_FeaturesMatcher }
	
	pub type PtrOfDetail_BestOf2NearestMatcher = core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher>;
	
	ptr_extern! { crate::stitching::Detail_BestOf2NearestMatcher,
		cv_PtrOfDetail_BestOf2NearestMatcher_delete, cv_PtrOfDetail_BestOf2NearestMatcher_get_inner_ptr, cv_PtrOfDetail_BestOf2NearestMatcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_BestOf2NearestMatcher, cv_PtrOfDetail_BestOf2NearestMatcher_new }
	
	impl core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher> {
		#[inline] pub fn as_raw_PtrOfDetail_BestOf2NearestMatcher(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BestOf2NearestMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BestOf2NearestMatcherTraitConst for core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher> {
		#[inline] fn as_raw_Detail_BestOf2NearestMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BestOf2NearestMatcherTrait for core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher> {
		#[inline] fn as_raw_mut_Detail_BestOf2NearestMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_FeaturesMatcherTraitConst for core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher> {
		#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_FeaturesMatcherTrait for core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher> {
		#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BestOf2NearestMatcher, core::Ptr<crate::stitching::Detail_FeaturesMatcher>, cv_PtrOfDetail_BestOf2NearestMatcher_to_PtrOfDetail_FeaturesMatcher }
	
	pub type PtrOfDetail_BestOf2NearestRangeMatcher = core::Ptr<crate::stitching::Detail_BestOf2NearestRangeMatcher>;
	
	ptr_extern! { crate::stitching::Detail_BestOf2NearestRangeMatcher,
		cv_PtrOfDetail_BestOf2NearestRangeMatcher_delete, cv_PtrOfDetail_BestOf2NearestRangeMatcher_get_inner_ptr, cv_PtrOfDetail_BestOf2NearestRangeMatcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_BestOf2NearestRangeMatcher, cv_PtrOfDetail_BestOf2NearestRangeMatcher_new }
	
	impl core::Ptr<crate::stitching::Detail_BestOf2NearestRangeMatcher> {
		#[inline] pub fn as_raw_PtrOfDetail_BestOf2NearestRangeMatcher(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BestOf2NearestRangeMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BestOf2NearestRangeMatcherTraitConst for core::Ptr<crate::stitching::Detail_BestOf2NearestRangeMatcher> {
		#[inline] fn as_raw_Detail_BestOf2NearestRangeMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BestOf2NearestRangeMatcherTrait for core::Ptr<crate::stitching::Detail_BestOf2NearestRangeMatcher> {
		#[inline] fn as_raw_mut_Detail_BestOf2NearestRangeMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BestOf2NearestMatcherTraitConst for core::Ptr<crate::stitching::Detail_BestOf2NearestRangeMatcher> {
		#[inline] fn as_raw_Detail_BestOf2NearestMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BestOf2NearestMatcherTrait for core::Ptr<crate::stitching::Detail_BestOf2NearestRangeMatcher> {
		#[inline] fn as_raw_mut_Detail_BestOf2NearestMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BestOf2NearestRangeMatcher, core::Ptr<crate::stitching::Detail_BestOf2NearestMatcher>, cv_PtrOfDetail_BestOf2NearestRangeMatcher_to_PtrOfDetail_BestOf2NearestMatcher }
	
	impl crate::stitching::Detail_FeaturesMatcherTraitConst for core::Ptr<crate::stitching::Detail_BestOf2NearestRangeMatcher> {
		#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_FeaturesMatcherTrait for core::Ptr<crate::stitching::Detail_BestOf2NearestRangeMatcher> {
		#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BestOf2NearestRangeMatcher, core::Ptr<crate::stitching::Detail_FeaturesMatcher>, cv_PtrOfDetail_BestOf2NearestRangeMatcher_to_PtrOfDetail_FeaturesMatcher }
	
	pub type PtrOfDetail_Blender = core::Ptr<crate::stitching::Detail_Blender>;
	
	ptr_extern! { crate::stitching::Detail_Blender,
		cv_PtrOfDetail_Blender_delete, cv_PtrOfDetail_Blender_get_inner_ptr, cv_PtrOfDetail_Blender_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_Blender, cv_PtrOfDetail_Blender_new }
	
	impl core::Ptr<crate::stitching::Detail_Blender> {
		#[inline] pub fn as_raw_PtrOfDetail_Blender(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_Blender(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlenderTraitConst for core::Ptr<crate::stitching::Detail_Blender> {
		#[inline] fn as_raw_Detail_Blender(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BlenderTrait for core::Ptr<crate::stitching::Detail_Blender> {
		#[inline] fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_BlocksChannelsCompensator = core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator>;
	
	ptr_extern! { crate::stitching::Detail_BlocksChannelsCompensator,
		cv_PtrOfDetail_BlocksChannelsCompensator_delete, cv_PtrOfDetail_BlocksChannelsCompensator_get_inner_ptr, cv_PtrOfDetail_BlocksChannelsCompensator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_BlocksChannelsCompensator, cv_PtrOfDetail_BlocksChannelsCompensator_new }
	
	impl core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator> {
		#[inline] pub fn as_raw_PtrOfDetail_BlocksChannelsCompensator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BlocksChannelsCompensator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlocksChannelsCompensatorTraitConst for core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator> {
		#[inline] fn as_raw_Detail_BlocksChannelsCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BlocksChannelsCompensatorTrait for core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator> {
		#[inline] fn as_raw_mut_Detail_BlocksChannelsCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlocksCompensatorTraitConst for core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator> {
		#[inline] fn as_raw_Detail_BlocksCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BlocksCompensatorTrait for core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator> {
		#[inline] fn as_raw_mut_Detail_BlocksCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BlocksChannelsCompensator, core::Ptr<crate::stitching::Detail_BlocksCompensator>, cv_PtrOfDetail_BlocksChannelsCompensator_to_PtrOfDetail_BlocksCompensator }
	
	impl crate::stitching::Detail_ExposureCompensatorTraitConst for core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator> {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorTrait for core::Ptr<crate::stitching::Detail_BlocksChannelsCompensator> {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BlocksChannelsCompensator, core::Ptr<crate::stitching::Detail_ExposureCompensator>, cv_PtrOfDetail_BlocksChannelsCompensator_to_PtrOfDetail_ExposureCompensator }
	
	pub type PtrOfDetail_BlocksCompensator = core::Ptr<crate::stitching::Detail_BlocksCompensator>;
	
	ptr_extern! { crate::stitching::Detail_BlocksCompensator,
		cv_PtrOfDetail_BlocksCompensator_delete, cv_PtrOfDetail_BlocksCompensator_get_inner_ptr, cv_PtrOfDetail_BlocksCompensator_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::stitching::Detail_BlocksCompensator> {
		#[inline] pub fn as_raw_PtrOfDetail_BlocksCompensator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BlocksCompensator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlocksCompensatorTraitConst for core::Ptr<crate::stitching::Detail_BlocksCompensator> {
		#[inline] fn as_raw_Detail_BlocksCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BlocksCompensatorTrait for core::Ptr<crate::stitching::Detail_BlocksCompensator> {
		#[inline] fn as_raw_mut_Detail_BlocksCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorTraitConst for core::Ptr<crate::stitching::Detail_BlocksCompensator> {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorTrait for core::Ptr<crate::stitching::Detail_BlocksCompensator> {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BlocksCompensator, core::Ptr<crate::stitching::Detail_ExposureCompensator>, cv_PtrOfDetail_BlocksCompensator_to_PtrOfDetail_ExposureCompensator }
	
	pub type PtrOfDetail_BlocksGainCompensator = core::Ptr<crate::stitching::Detail_BlocksGainCompensator>;
	
	ptr_extern! { crate::stitching::Detail_BlocksGainCompensator,
		cv_PtrOfDetail_BlocksGainCompensator_delete, cv_PtrOfDetail_BlocksGainCompensator_get_inner_ptr, cv_PtrOfDetail_BlocksGainCompensator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_BlocksGainCompensator, cv_PtrOfDetail_BlocksGainCompensator_new }
	
	impl core::Ptr<crate::stitching::Detail_BlocksGainCompensator> {
		#[inline] pub fn as_raw_PtrOfDetail_BlocksGainCompensator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BlocksGainCompensator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlocksGainCompensatorTraitConst for core::Ptr<crate::stitching::Detail_BlocksGainCompensator> {
		#[inline] fn as_raw_Detail_BlocksGainCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BlocksGainCompensatorTrait for core::Ptr<crate::stitching::Detail_BlocksGainCompensator> {
		#[inline] fn as_raw_mut_Detail_BlocksGainCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlocksCompensatorTraitConst for core::Ptr<crate::stitching::Detail_BlocksGainCompensator> {
		#[inline] fn as_raw_Detail_BlocksCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BlocksCompensatorTrait for core::Ptr<crate::stitching::Detail_BlocksGainCompensator> {
		#[inline] fn as_raw_mut_Detail_BlocksCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BlocksGainCompensator, core::Ptr<crate::stitching::Detail_BlocksCompensator>, cv_PtrOfDetail_BlocksGainCompensator_to_PtrOfDetail_BlocksCompensator }
	
	impl crate::stitching::Detail_ExposureCompensatorTraitConst for core::Ptr<crate::stitching::Detail_BlocksGainCompensator> {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorTrait for core::Ptr<crate::stitching::Detail_BlocksGainCompensator> {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BlocksGainCompensator, core::Ptr<crate::stitching::Detail_ExposureCompensator>, cv_PtrOfDetail_BlocksGainCompensator_to_PtrOfDetail_ExposureCompensator }
	
	pub type PtrOfDetail_BundleAdjusterAffine = core::Ptr<crate::stitching::Detail_BundleAdjusterAffine>;
	
	ptr_extern! { crate::stitching::Detail_BundleAdjusterAffine,
		cv_PtrOfDetail_BundleAdjusterAffine_delete, cv_PtrOfDetail_BundleAdjusterAffine_get_inner_ptr, cv_PtrOfDetail_BundleAdjusterAffine_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_BundleAdjusterAffine, cv_PtrOfDetail_BundleAdjusterAffine_new }
	
	impl core::Ptr<crate::stitching::Detail_BundleAdjusterAffine> {
		#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterAffine(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterAffine(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterAffineTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterAffine> {
		#[inline] fn as_raw_Detail_BundleAdjusterAffine(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterAffineTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterAffine> {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterAffine(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterAffine> {
		#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterAffine> {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterAffine, core::Ptr<crate::stitching::Detail_BundleAdjusterBase>, cv_PtrOfDetail_BundleAdjusterAffine_to_PtrOfDetail_BundleAdjusterBase }
	
	impl crate::stitching::Detail_EstimatorTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterAffine> {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_EstimatorTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterAffine> {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterAffine, core::Ptr<crate::stitching::Detail_Estimator>, cv_PtrOfDetail_BundleAdjusterAffine_to_PtrOfDetail_Estimator }
	
	pub type PtrOfDetail_BundleAdjusterAffinePartial = core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial>;
	
	ptr_extern! { crate::stitching::Detail_BundleAdjusterAffinePartial,
		cv_PtrOfDetail_BundleAdjusterAffinePartial_delete, cv_PtrOfDetail_BundleAdjusterAffinePartial_get_inner_ptr, cv_PtrOfDetail_BundleAdjusterAffinePartial_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_BundleAdjusterAffinePartial, cv_PtrOfDetail_BundleAdjusterAffinePartial_new }
	
	impl core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial> {
		#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterAffinePartial(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterAffinePartial(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterAffinePartialTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial> {
		#[inline] fn as_raw_Detail_BundleAdjusterAffinePartial(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterAffinePartialTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial> {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterAffinePartial(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial> {
		#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial> {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterAffinePartial, core::Ptr<crate::stitching::Detail_BundleAdjusterBase>, cv_PtrOfDetail_BundleAdjusterAffinePartial_to_PtrOfDetail_BundleAdjusterBase }
	
	impl crate::stitching::Detail_EstimatorTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial> {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_EstimatorTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial> {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterAffinePartial, core::Ptr<crate::stitching::Detail_Estimator>, cv_PtrOfDetail_BundleAdjusterAffinePartial_to_PtrOfDetail_Estimator }
	
	pub type PtrOfDetail_BundleAdjusterBase = core::Ptr<crate::stitching::Detail_BundleAdjusterBase>;
	
	ptr_extern! { crate::stitching::Detail_BundleAdjusterBase,
		cv_PtrOfDetail_BundleAdjusterBase_delete, cv_PtrOfDetail_BundleAdjusterBase_get_inner_ptr, cv_PtrOfDetail_BundleAdjusterBase_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::stitching::Detail_BundleAdjusterBase> {
		#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterBase(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterBase(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterBase> {
		#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterBase> {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_EstimatorTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterBase> {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_EstimatorTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterBase> {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterBase, core::Ptr<crate::stitching::Detail_Estimator>, cv_PtrOfDetail_BundleAdjusterBase_to_PtrOfDetail_Estimator }
	
	pub type PtrOfDetail_BundleAdjusterRay = core::Ptr<crate::stitching::Detail_BundleAdjusterRay>;
	
	ptr_extern! { crate::stitching::Detail_BundleAdjusterRay,
		cv_PtrOfDetail_BundleAdjusterRay_delete, cv_PtrOfDetail_BundleAdjusterRay_get_inner_ptr, cv_PtrOfDetail_BundleAdjusterRay_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_BundleAdjusterRay, cv_PtrOfDetail_BundleAdjusterRay_new }
	
	impl core::Ptr<crate::stitching::Detail_BundleAdjusterRay> {
		#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterRay(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterRay(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterRayTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterRay> {
		#[inline] fn as_raw_Detail_BundleAdjusterRay(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterRayTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterRay> {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterRay(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterRay> {
		#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterRay> {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterRay, core::Ptr<crate::stitching::Detail_BundleAdjusterBase>, cv_PtrOfDetail_BundleAdjusterRay_to_PtrOfDetail_BundleAdjusterBase }
	
	impl crate::stitching::Detail_EstimatorTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterRay> {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_EstimatorTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterRay> {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterRay, core::Ptr<crate::stitching::Detail_Estimator>, cv_PtrOfDetail_BundleAdjusterRay_to_PtrOfDetail_Estimator }
	
	pub type PtrOfDetail_BundleAdjusterReproj = core::Ptr<crate::stitching::Detail_BundleAdjusterReproj>;
	
	ptr_extern! { crate::stitching::Detail_BundleAdjusterReproj,
		cv_PtrOfDetail_BundleAdjusterReproj_delete, cv_PtrOfDetail_BundleAdjusterReproj_get_inner_ptr, cv_PtrOfDetail_BundleAdjusterReproj_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_BundleAdjusterReproj, cv_PtrOfDetail_BundleAdjusterReproj_new }
	
	impl core::Ptr<crate::stitching::Detail_BundleAdjusterReproj> {
		#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterReproj(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterReproj(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterReprojTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterReproj> {
		#[inline] fn as_raw_Detail_BundleAdjusterReproj(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterReprojTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterReproj> {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterReproj(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterReproj> {
		#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterReproj> {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterReproj, core::Ptr<crate::stitching::Detail_BundleAdjusterBase>, cv_PtrOfDetail_BundleAdjusterReproj_to_PtrOfDetail_BundleAdjusterBase }
	
	impl crate::stitching::Detail_EstimatorTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterReproj> {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_EstimatorTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterReproj> {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_BundleAdjusterReproj, core::Ptr<crate::stitching::Detail_Estimator>, cv_PtrOfDetail_BundleAdjusterReproj_to_PtrOfDetail_Estimator }
	
	pub type PtrOfDetail_ChannelsCompensator = core::Ptr<crate::stitching::Detail_ChannelsCompensator>;
	
	ptr_extern! { crate::stitching::Detail_ChannelsCompensator,
		cv_PtrOfDetail_ChannelsCompensator_delete, cv_PtrOfDetail_ChannelsCompensator_get_inner_ptr, cv_PtrOfDetail_ChannelsCompensator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_ChannelsCompensator, cv_PtrOfDetail_ChannelsCompensator_new }
	
	impl core::Ptr<crate::stitching::Detail_ChannelsCompensator> {
		#[inline] pub fn as_raw_PtrOfDetail_ChannelsCompensator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_ChannelsCompensator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ChannelsCompensatorTraitConst for core::Ptr<crate::stitching::Detail_ChannelsCompensator> {
		#[inline] fn as_raw_Detail_ChannelsCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_ChannelsCompensatorTrait for core::Ptr<crate::stitching::Detail_ChannelsCompensator> {
		#[inline] fn as_raw_mut_Detail_ChannelsCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorTraitConst for core::Ptr<crate::stitching::Detail_ChannelsCompensator> {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorTrait for core::Ptr<crate::stitching::Detail_ChannelsCompensator> {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_ChannelsCompensator, core::Ptr<crate::stitching::Detail_ExposureCompensator>, cv_PtrOfDetail_ChannelsCompensator_to_PtrOfDetail_ExposureCompensator }
	
	pub type PtrOfDetail_DpSeamFinder = core::Ptr<crate::stitching::Detail_DpSeamFinder>;
	
	ptr_extern! { crate::stitching::Detail_DpSeamFinder,
		cv_PtrOfDetail_DpSeamFinder_delete, cv_PtrOfDetail_DpSeamFinder_get_inner_ptr, cv_PtrOfDetail_DpSeamFinder_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_DpSeamFinder, cv_PtrOfDetail_DpSeamFinder_new }
	
	impl core::Ptr<crate::stitching::Detail_DpSeamFinder> {
		#[inline] pub fn as_raw_PtrOfDetail_DpSeamFinder(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_DpSeamFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_DpSeamFinderTraitConst for core::Ptr<crate::stitching::Detail_DpSeamFinder> {
		#[inline] fn as_raw_Detail_DpSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_DpSeamFinderTrait for core::Ptr<crate::stitching::Detail_DpSeamFinder> {
		#[inline] fn as_raw_mut_Detail_DpSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_SeamFinderTraitConst for core::Ptr<crate::stitching::Detail_DpSeamFinder> {
		#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_SeamFinderTrait for core::Ptr<crate::stitching::Detail_DpSeamFinder> {
		#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_DpSeamFinder, core::Ptr<crate::stitching::Detail_SeamFinder>, cv_PtrOfDetail_DpSeamFinder_to_PtrOfDetail_SeamFinder }
	
	pub type PtrOfDetail_Estimator = core::Ptr<crate::stitching::Detail_Estimator>;
	
	ptr_extern! { crate::stitching::Detail_Estimator,
		cv_PtrOfDetail_Estimator_delete, cv_PtrOfDetail_Estimator_get_inner_ptr, cv_PtrOfDetail_Estimator_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::stitching::Detail_Estimator> {
		#[inline] pub fn as_raw_PtrOfDetail_Estimator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_Estimator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_EstimatorTraitConst for core::Ptr<crate::stitching::Detail_Estimator> {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_EstimatorTrait for core::Ptr<crate::stitching::Detail_Estimator> {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_ExposureCompensator = core::Ptr<crate::stitching::Detail_ExposureCompensator>;
	
	ptr_extern! { crate::stitching::Detail_ExposureCompensator,
		cv_PtrOfDetail_ExposureCompensator_delete, cv_PtrOfDetail_ExposureCompensator_get_inner_ptr, cv_PtrOfDetail_ExposureCompensator_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::stitching::Detail_ExposureCompensator> {
		#[inline] pub fn as_raw_PtrOfDetail_ExposureCompensator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_ExposureCompensator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorTraitConst for core::Ptr<crate::stitching::Detail_ExposureCompensator> {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorTrait for core::Ptr<crate::stitching::Detail_ExposureCompensator> {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_FeatherBlender = core::Ptr<crate::stitching::Detail_FeatherBlender>;
	
	ptr_extern! { crate::stitching::Detail_FeatherBlender,
		cv_PtrOfDetail_FeatherBlender_delete, cv_PtrOfDetail_FeatherBlender_get_inner_ptr, cv_PtrOfDetail_FeatherBlender_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_FeatherBlender, cv_PtrOfDetail_FeatherBlender_new }
	
	impl core::Ptr<crate::stitching::Detail_FeatherBlender> {
		#[inline] pub fn as_raw_PtrOfDetail_FeatherBlender(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_FeatherBlender(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_FeatherBlenderTraitConst for core::Ptr<crate::stitching::Detail_FeatherBlender> {
		#[inline] fn as_raw_Detail_FeatherBlender(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_FeatherBlenderTrait for core::Ptr<crate::stitching::Detail_FeatherBlender> {
		#[inline] fn as_raw_mut_Detail_FeatherBlender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlenderTraitConst for core::Ptr<crate::stitching::Detail_FeatherBlender> {
		#[inline] fn as_raw_Detail_Blender(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BlenderTrait for core::Ptr<crate::stitching::Detail_FeatherBlender> {
		#[inline] fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_FeatherBlender, core::Ptr<crate::stitching::Detail_Blender>, cv_PtrOfDetail_FeatherBlender_to_PtrOfDetail_Blender }
	
	pub type PtrOfDetail_FeaturesMatcher = core::Ptr<crate::stitching::Detail_FeaturesMatcher>;
	
	ptr_extern! { crate::stitching::Detail_FeaturesMatcher,
		cv_PtrOfDetail_FeaturesMatcher_delete, cv_PtrOfDetail_FeaturesMatcher_get_inner_ptr, cv_PtrOfDetail_FeaturesMatcher_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::stitching::Detail_FeaturesMatcher> {
		#[inline] pub fn as_raw_PtrOfDetail_FeaturesMatcher(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_FeaturesMatcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_FeaturesMatcherTraitConst for core::Ptr<crate::stitching::Detail_FeaturesMatcher> {
		#[inline] fn as_raw_Detail_FeaturesMatcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_FeaturesMatcherTrait for core::Ptr<crate::stitching::Detail_FeaturesMatcher> {
		#[inline] fn as_raw_mut_Detail_FeaturesMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_GainCompensator = core::Ptr<crate::stitching::Detail_GainCompensator>;
	
	ptr_extern! { crate::stitching::Detail_GainCompensator,
		cv_PtrOfDetail_GainCompensator_delete, cv_PtrOfDetail_GainCompensator_get_inner_ptr, cv_PtrOfDetail_GainCompensator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_GainCompensator, cv_PtrOfDetail_GainCompensator_new }
	
	impl core::Ptr<crate::stitching::Detail_GainCompensator> {
		#[inline] pub fn as_raw_PtrOfDetail_GainCompensator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_GainCompensator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_GainCompensatorTraitConst for core::Ptr<crate::stitching::Detail_GainCompensator> {
		#[inline] fn as_raw_Detail_GainCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_GainCompensatorTrait for core::Ptr<crate::stitching::Detail_GainCompensator> {
		#[inline] fn as_raw_mut_Detail_GainCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorTraitConst for core::Ptr<crate::stitching::Detail_GainCompensator> {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorTrait for core::Ptr<crate::stitching::Detail_GainCompensator> {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_GainCompensator, core::Ptr<crate::stitching::Detail_ExposureCompensator>, cv_PtrOfDetail_GainCompensator_to_PtrOfDetail_ExposureCompensator }
	
	pub type PtrOfDetail_GraphCutSeamFinder = core::Ptr<crate::stitching::Detail_GraphCutSeamFinder>;
	
	ptr_extern! { crate::stitching::Detail_GraphCutSeamFinder,
		cv_PtrOfDetail_GraphCutSeamFinder_delete, cv_PtrOfDetail_GraphCutSeamFinder_get_inner_ptr, cv_PtrOfDetail_GraphCutSeamFinder_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_GraphCutSeamFinder, cv_PtrOfDetail_GraphCutSeamFinder_new }
	
	impl core::Ptr<crate::stitching::Detail_GraphCutSeamFinder> {
		#[inline] pub fn as_raw_PtrOfDetail_GraphCutSeamFinder(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_GraphCutSeamFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_GraphCutSeamFinderTraitConst for core::Ptr<crate::stitching::Detail_GraphCutSeamFinder> {
		#[inline] fn as_raw_Detail_GraphCutSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_GraphCutSeamFinderTrait for core::Ptr<crate::stitching::Detail_GraphCutSeamFinder> {
		#[inline] fn as_raw_mut_Detail_GraphCutSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_GraphCutSeamFinderBaseTraitConst for core::Ptr<crate::stitching::Detail_GraphCutSeamFinder> {
		#[inline] fn as_raw_Detail_GraphCutSeamFinderBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_GraphCutSeamFinderBaseTrait for core::Ptr<crate::stitching::Detail_GraphCutSeamFinder> {
		#[inline] fn as_raw_mut_Detail_GraphCutSeamFinderBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_SeamFinderTraitConst for core::Ptr<crate::stitching::Detail_GraphCutSeamFinder> {
		#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_SeamFinderTrait for core::Ptr<crate::stitching::Detail_GraphCutSeamFinder> {
		#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_GraphCutSeamFinder, core::Ptr<crate::stitching::Detail_SeamFinder>, cv_PtrOfDetail_GraphCutSeamFinder_to_PtrOfDetail_SeamFinder }
	
	pub type PtrOfDetail_GraphCutSeamFinderGpu = core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu>;
	
	ptr_extern! { crate::stitching::Detail_GraphCutSeamFinderGpu,
		cv_PtrOfDetail_GraphCutSeamFinderGpu_delete, cv_PtrOfDetail_GraphCutSeamFinderGpu_get_inner_ptr, cv_PtrOfDetail_GraphCutSeamFinderGpu_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_GraphCutSeamFinderGpu, cv_PtrOfDetail_GraphCutSeamFinderGpu_new }
	
	impl core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu> {
		#[inline] pub fn as_raw_PtrOfDetail_GraphCutSeamFinderGpu(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_GraphCutSeamFinderGpu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_GraphCutSeamFinderGpuTraitConst for core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu> {
		#[inline] fn as_raw_Detail_GraphCutSeamFinderGpu(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_GraphCutSeamFinderGpuTrait for core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu> {
		#[inline] fn as_raw_mut_Detail_GraphCutSeamFinderGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_GraphCutSeamFinderBaseTraitConst for core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu> {
		#[inline] fn as_raw_Detail_GraphCutSeamFinderBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_GraphCutSeamFinderBaseTrait for core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu> {
		#[inline] fn as_raw_mut_Detail_GraphCutSeamFinderBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_PairwiseSeamFinderTraitConst for core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu> {
		#[inline] fn as_raw_Detail_PairwiseSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_PairwiseSeamFinderTrait for core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu> {
		#[inline] fn as_raw_mut_Detail_PairwiseSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_GraphCutSeamFinderGpu, core::Ptr<crate::stitching::Detail_PairwiseSeamFinder>, cv_PtrOfDetail_GraphCutSeamFinderGpu_to_PtrOfDetail_PairwiseSeamFinder }
	
	impl crate::stitching::Detail_SeamFinderTraitConst for core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu> {
		#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_SeamFinderTrait for core::Ptr<crate::stitching::Detail_GraphCutSeamFinderGpu> {
		#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_GraphCutSeamFinderGpu, core::Ptr<crate::stitching::Detail_SeamFinder>, cv_PtrOfDetail_GraphCutSeamFinderGpu_to_PtrOfDetail_SeamFinder }
	
	pub type PtrOfDetail_HomographyBasedEstimator = core::Ptr<crate::stitching::Detail_HomographyBasedEstimator>;
	
	ptr_extern! { crate::stitching::Detail_HomographyBasedEstimator,
		cv_PtrOfDetail_HomographyBasedEstimator_delete, cv_PtrOfDetail_HomographyBasedEstimator_get_inner_ptr, cv_PtrOfDetail_HomographyBasedEstimator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_HomographyBasedEstimator, cv_PtrOfDetail_HomographyBasedEstimator_new }
	
	impl core::Ptr<crate::stitching::Detail_HomographyBasedEstimator> {
		#[inline] pub fn as_raw_PtrOfDetail_HomographyBasedEstimator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_HomographyBasedEstimator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_HomographyBasedEstimatorTraitConst for core::Ptr<crate::stitching::Detail_HomographyBasedEstimator> {
		#[inline] fn as_raw_Detail_HomographyBasedEstimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_HomographyBasedEstimatorTrait for core::Ptr<crate::stitching::Detail_HomographyBasedEstimator> {
		#[inline] fn as_raw_mut_Detail_HomographyBasedEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_EstimatorTraitConst for core::Ptr<crate::stitching::Detail_HomographyBasedEstimator> {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_EstimatorTrait for core::Ptr<crate::stitching::Detail_HomographyBasedEstimator> {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_HomographyBasedEstimator, core::Ptr<crate::stitching::Detail_Estimator>, cv_PtrOfDetail_HomographyBasedEstimator_to_PtrOfDetail_Estimator }
	
	pub type PtrOfDetail_MultiBandBlender = core::Ptr<crate::stitching::Detail_MultiBandBlender>;
	
	ptr_extern! { crate::stitching::Detail_MultiBandBlender,
		cv_PtrOfDetail_MultiBandBlender_delete, cv_PtrOfDetail_MultiBandBlender_get_inner_ptr, cv_PtrOfDetail_MultiBandBlender_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_MultiBandBlender, cv_PtrOfDetail_MultiBandBlender_new }
	
	impl core::Ptr<crate::stitching::Detail_MultiBandBlender> {
		#[inline] pub fn as_raw_PtrOfDetail_MultiBandBlender(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_MultiBandBlender(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_MultiBandBlenderTraitConst for core::Ptr<crate::stitching::Detail_MultiBandBlender> {
		#[inline] fn as_raw_Detail_MultiBandBlender(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_MultiBandBlenderTrait for core::Ptr<crate::stitching::Detail_MultiBandBlender> {
		#[inline] fn as_raw_mut_Detail_MultiBandBlender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BlenderTraitConst for core::Ptr<crate::stitching::Detail_MultiBandBlender> {
		#[inline] fn as_raw_Detail_Blender(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BlenderTrait for core::Ptr<crate::stitching::Detail_MultiBandBlender> {
		#[inline] fn as_raw_mut_Detail_Blender(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_MultiBandBlender, core::Ptr<crate::stitching::Detail_Blender>, cv_PtrOfDetail_MultiBandBlender_to_PtrOfDetail_Blender }
	
	pub type PtrOfDetail_NoBundleAdjuster = core::Ptr<crate::stitching::Detail_NoBundleAdjuster>;
	
	ptr_extern! { crate::stitching::Detail_NoBundleAdjuster,
		cv_PtrOfDetail_NoBundleAdjuster_delete, cv_PtrOfDetail_NoBundleAdjuster_get_inner_ptr, cv_PtrOfDetail_NoBundleAdjuster_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_NoBundleAdjuster, cv_PtrOfDetail_NoBundleAdjuster_new }
	
	impl core::Ptr<crate::stitching::Detail_NoBundleAdjuster> {
		#[inline] pub fn as_raw_PtrOfDetail_NoBundleAdjuster(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_NoBundleAdjuster(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_NoBundleAdjusterTraitConst for core::Ptr<crate::stitching::Detail_NoBundleAdjuster> {
		#[inline] fn as_raw_Detail_NoBundleAdjuster(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_NoBundleAdjusterTrait for core::Ptr<crate::stitching::Detail_NoBundleAdjuster> {
		#[inline] fn as_raw_mut_Detail_NoBundleAdjuster(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseTraitConst for core::Ptr<crate::stitching::Detail_NoBundleAdjuster> {
		#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_BundleAdjusterBaseTrait for core::Ptr<crate::stitching::Detail_NoBundleAdjuster> {
		#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_NoBundleAdjuster, core::Ptr<crate::stitching::Detail_BundleAdjusterBase>, cv_PtrOfDetail_NoBundleAdjuster_to_PtrOfDetail_BundleAdjusterBase }
	
	impl crate::stitching::Detail_EstimatorTraitConst for core::Ptr<crate::stitching::Detail_NoBundleAdjuster> {
		#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_EstimatorTrait for core::Ptr<crate::stitching::Detail_NoBundleAdjuster> {
		#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_NoBundleAdjuster, core::Ptr<crate::stitching::Detail_Estimator>, cv_PtrOfDetail_NoBundleAdjuster_to_PtrOfDetail_Estimator }
	
	pub type PtrOfDetail_NoExposureCompensator = core::Ptr<crate::stitching::Detail_NoExposureCompensator>;
	
	ptr_extern! { crate::stitching::Detail_NoExposureCompensator,
		cv_PtrOfDetail_NoExposureCompensator_delete, cv_PtrOfDetail_NoExposureCompensator_get_inner_ptr, cv_PtrOfDetail_NoExposureCompensator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_NoExposureCompensator, cv_PtrOfDetail_NoExposureCompensator_new }
	
	impl core::Ptr<crate::stitching::Detail_NoExposureCompensator> {
		#[inline] pub fn as_raw_PtrOfDetail_NoExposureCompensator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_NoExposureCompensator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_NoExposureCompensatorTraitConst for core::Ptr<crate::stitching::Detail_NoExposureCompensator> {
		#[inline] fn as_raw_Detail_NoExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_NoExposureCompensatorTrait for core::Ptr<crate::stitching::Detail_NoExposureCompensator> {
		#[inline] fn as_raw_mut_Detail_NoExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorTraitConst for core::Ptr<crate::stitching::Detail_NoExposureCompensator> {
		#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_ExposureCompensatorTrait for core::Ptr<crate::stitching::Detail_NoExposureCompensator> {
		#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_NoExposureCompensator, core::Ptr<crate::stitching::Detail_ExposureCompensator>, cv_PtrOfDetail_NoExposureCompensator_to_PtrOfDetail_ExposureCompensator }
	
	pub type PtrOfDetail_NoSeamFinder = core::Ptr<crate::stitching::Detail_NoSeamFinder>;
	
	ptr_extern! { crate::stitching::Detail_NoSeamFinder,
		cv_PtrOfDetail_NoSeamFinder_delete, cv_PtrOfDetail_NoSeamFinder_get_inner_ptr, cv_PtrOfDetail_NoSeamFinder_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_NoSeamFinder, cv_PtrOfDetail_NoSeamFinder_new }
	
	impl core::Ptr<crate::stitching::Detail_NoSeamFinder> {
		#[inline] pub fn as_raw_PtrOfDetail_NoSeamFinder(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_NoSeamFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_NoSeamFinderTraitConst for core::Ptr<crate::stitching::Detail_NoSeamFinder> {
		#[inline] fn as_raw_Detail_NoSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_NoSeamFinderTrait for core::Ptr<crate::stitching::Detail_NoSeamFinder> {
		#[inline] fn as_raw_mut_Detail_NoSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_SeamFinderTraitConst for core::Ptr<crate::stitching::Detail_NoSeamFinder> {
		#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_SeamFinderTrait for core::Ptr<crate::stitching::Detail_NoSeamFinder> {
		#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_NoSeamFinder, core::Ptr<crate::stitching::Detail_SeamFinder>, cv_PtrOfDetail_NoSeamFinder_to_PtrOfDetail_SeamFinder }
	
	pub type PtrOfDetail_PairwiseSeamFinder = core::Ptr<crate::stitching::Detail_PairwiseSeamFinder>;
	
	ptr_extern! { crate::stitching::Detail_PairwiseSeamFinder,
		cv_PtrOfDetail_PairwiseSeamFinder_delete, cv_PtrOfDetail_PairwiseSeamFinder_get_inner_ptr, cv_PtrOfDetail_PairwiseSeamFinder_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::stitching::Detail_PairwiseSeamFinder> {
		#[inline] pub fn as_raw_PtrOfDetail_PairwiseSeamFinder(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_PairwiseSeamFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_PairwiseSeamFinderTraitConst for core::Ptr<crate::stitching::Detail_PairwiseSeamFinder> {
		#[inline] fn as_raw_Detail_PairwiseSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_PairwiseSeamFinderTrait for core::Ptr<crate::stitching::Detail_PairwiseSeamFinder> {
		#[inline] fn as_raw_mut_Detail_PairwiseSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_SeamFinderTraitConst for core::Ptr<crate::stitching::Detail_PairwiseSeamFinder> {
		#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_SeamFinderTrait for core::Ptr<crate::stitching::Detail_PairwiseSeamFinder> {
		#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_PairwiseSeamFinder, core::Ptr<crate::stitching::Detail_SeamFinder>, cv_PtrOfDetail_PairwiseSeamFinder_to_PtrOfDetail_SeamFinder }
	
	pub type PtrOfDetail_RotationWarper = core::Ptr<crate::stitching::Detail_RotationWarper>;
	
	ptr_extern! { crate::stitching::Detail_RotationWarper,
		cv_PtrOfDetail_RotationWarper_delete, cv_PtrOfDetail_RotationWarper_get_inner_ptr, cv_PtrOfDetail_RotationWarper_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::stitching::Detail_RotationWarper> {
		#[inline] pub fn as_raw_PtrOfDetail_RotationWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_RotationWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_RotationWarperTraitConst for core::Ptr<crate::stitching::Detail_RotationWarper> {
		#[inline] fn as_raw_Detail_RotationWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_RotationWarperTrait for core::Ptr<crate::stitching::Detail_RotationWarper> {
		#[inline] fn as_raw_mut_Detail_RotationWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_SeamFinder = core::Ptr<crate::stitching::Detail_SeamFinder>;
	
	ptr_extern! { crate::stitching::Detail_SeamFinder,
		cv_PtrOfDetail_SeamFinder_delete, cv_PtrOfDetail_SeamFinder_get_inner_ptr, cv_PtrOfDetail_SeamFinder_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::stitching::Detail_SeamFinder> {
		#[inline] pub fn as_raw_PtrOfDetail_SeamFinder(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_SeamFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_SeamFinderTraitConst for core::Ptr<crate::stitching::Detail_SeamFinder> {
		#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_SeamFinderTrait for core::Ptr<crate::stitching::Detail_SeamFinder> {
		#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDetail_VoronoiSeamFinder = core::Ptr<crate::stitching::Detail_VoronoiSeamFinder>;
	
	ptr_extern! { crate::stitching::Detail_VoronoiSeamFinder,
		cv_PtrOfDetail_VoronoiSeamFinder_delete, cv_PtrOfDetail_VoronoiSeamFinder_get_inner_ptr, cv_PtrOfDetail_VoronoiSeamFinder_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Detail_VoronoiSeamFinder, cv_PtrOfDetail_VoronoiSeamFinder_new }
	
	impl core::Ptr<crate::stitching::Detail_VoronoiSeamFinder> {
		#[inline] pub fn as_raw_PtrOfDetail_VoronoiSeamFinder(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetail_VoronoiSeamFinder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::Detail_VoronoiSeamFinderTraitConst for core::Ptr<crate::stitching::Detail_VoronoiSeamFinder> {
		#[inline] fn as_raw_Detail_VoronoiSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_VoronoiSeamFinderTrait for core::Ptr<crate::stitching::Detail_VoronoiSeamFinder> {
		#[inline] fn as_raw_mut_Detail_VoronoiSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::Detail_PairwiseSeamFinderTraitConst for core::Ptr<crate::stitching::Detail_VoronoiSeamFinder> {
		#[inline] fn as_raw_Detail_PairwiseSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_PairwiseSeamFinderTrait for core::Ptr<crate::stitching::Detail_VoronoiSeamFinder> {
		#[inline] fn as_raw_mut_Detail_PairwiseSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_VoronoiSeamFinder, core::Ptr<crate::stitching::Detail_PairwiseSeamFinder>, cv_PtrOfDetail_VoronoiSeamFinder_to_PtrOfDetail_PairwiseSeamFinder }
	
	impl crate::stitching::Detail_SeamFinderTraitConst for core::Ptr<crate::stitching::Detail_VoronoiSeamFinder> {
		#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::Detail_SeamFinderTrait for core::Ptr<crate::stitching::Detail_VoronoiSeamFinder> {
		#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDetail_VoronoiSeamFinder, core::Ptr<crate::stitching::Detail_SeamFinder>, cv_PtrOfDetail_VoronoiSeamFinder_to_PtrOfDetail_SeamFinder }
	
	pub type PtrOfFisheyeWarper = core::Ptr<crate::stitching::FisheyeWarper>;
	
	ptr_extern! { crate::stitching::FisheyeWarper,
		cv_PtrOfFisheyeWarper_delete, cv_PtrOfFisheyeWarper_get_inner_ptr, cv_PtrOfFisheyeWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::FisheyeWarper, cv_PtrOfFisheyeWarper_new }
	
	impl core::Ptr<crate::stitching::FisheyeWarper> {
		#[inline] pub fn as_raw_PtrOfFisheyeWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFisheyeWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::FisheyeWarperTraitConst for core::Ptr<crate::stitching::FisheyeWarper> {
		#[inline] fn as_raw_FisheyeWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::FisheyeWarperTrait for core::Ptr<crate::stitching::FisheyeWarper> {
		#[inline] fn as_raw_mut_FisheyeWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::FisheyeWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::FisheyeWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfFisheyeWarper, core::Ptr<crate::stitching::WarperCreator>, cv_PtrOfFisheyeWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfMercatorWarper = core::Ptr<crate::stitching::MercatorWarper>;
	
	ptr_extern! { crate::stitching::MercatorWarper,
		cv_PtrOfMercatorWarper_delete, cv_PtrOfMercatorWarper_get_inner_ptr, cv_PtrOfMercatorWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::MercatorWarper, cv_PtrOfMercatorWarper_new }
	
	impl core::Ptr<crate::stitching::MercatorWarper> {
		#[inline] pub fn as_raw_PtrOfMercatorWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMercatorWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::MercatorWarperTraitConst for core::Ptr<crate::stitching::MercatorWarper> {
		#[inline] fn as_raw_MercatorWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::MercatorWarperTrait for core::Ptr<crate::stitching::MercatorWarper> {
		#[inline] fn as_raw_mut_MercatorWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::MercatorWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::MercatorWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMercatorWarper, core::Ptr<crate::stitching::WarperCreator>, cv_PtrOfMercatorWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfPaniniPortraitWarper = core::Ptr<crate::stitching::PaniniPortraitWarper>;
	
	ptr_extern! { crate::stitching::PaniniPortraitWarper,
		cv_PtrOfPaniniPortraitWarper_delete, cv_PtrOfPaniniPortraitWarper_get_inner_ptr, cv_PtrOfPaniniPortraitWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::PaniniPortraitWarper, cv_PtrOfPaniniPortraitWarper_new }
	
	impl core::Ptr<crate::stitching::PaniniPortraitWarper> {
		#[inline] pub fn as_raw_PtrOfPaniniPortraitWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPaniniPortraitWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::PaniniPortraitWarperTraitConst for core::Ptr<crate::stitching::PaniniPortraitWarper> {
		#[inline] fn as_raw_PaniniPortraitWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::PaniniPortraitWarperTrait for core::Ptr<crate::stitching::PaniniPortraitWarper> {
		#[inline] fn as_raw_mut_PaniniPortraitWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::PaniniPortraitWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::PaniniPortraitWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfPaniniPortraitWarper, core::Ptr<crate::stitching::WarperCreator>, cv_PtrOfPaniniPortraitWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfPaniniWarper = core::Ptr<crate::stitching::PaniniWarper>;
	
	ptr_extern! { crate::stitching::PaniniWarper,
		cv_PtrOfPaniniWarper_delete, cv_PtrOfPaniniWarper_get_inner_ptr, cv_PtrOfPaniniWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::PaniniWarper, cv_PtrOfPaniniWarper_new }
	
	impl core::Ptr<crate::stitching::PaniniWarper> {
		#[inline] pub fn as_raw_PtrOfPaniniWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPaniniWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::PaniniWarperTraitConst for core::Ptr<crate::stitching::PaniniWarper> {
		#[inline] fn as_raw_PaniniWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::PaniniWarperTrait for core::Ptr<crate::stitching::PaniniWarper> {
		#[inline] fn as_raw_mut_PaniniWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::PaniniWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::PaniniWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfPaniniWarper, core::Ptr<crate::stitching::WarperCreator>, cv_PtrOfPaniniWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfPlaneWarper = core::Ptr<crate::stitching::PlaneWarper>;
	
	ptr_extern! { crate::stitching::PlaneWarper,
		cv_PtrOfPlaneWarper_delete, cv_PtrOfPlaneWarper_get_inner_ptr, cv_PtrOfPlaneWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::PlaneWarper, cv_PtrOfPlaneWarper_new }
	
	impl core::Ptr<crate::stitching::PlaneWarper> {
		#[inline] pub fn as_raw_PtrOfPlaneWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPlaneWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::PlaneWarperTraitConst for core::Ptr<crate::stitching::PlaneWarper> {
		#[inline] fn as_raw_PlaneWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::PlaneWarperTrait for core::Ptr<crate::stitching::PlaneWarper> {
		#[inline] fn as_raw_mut_PlaneWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::PlaneWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::PlaneWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfPlaneWarper, core::Ptr<crate::stitching::WarperCreator>, cv_PtrOfPlaneWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfPlaneWarperGpu = core::Ptr<crate::stitching::PlaneWarperGpu>;
	
	ptr_extern! { crate::stitching::PlaneWarperGpu,
		cv_PtrOfPlaneWarperGpu_delete, cv_PtrOfPlaneWarperGpu_get_inner_ptr, cv_PtrOfPlaneWarperGpu_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::PlaneWarperGpu, cv_PtrOfPlaneWarperGpu_new }
	
	impl core::Ptr<crate::stitching::PlaneWarperGpu> {
		#[inline] pub fn as_raw_PtrOfPlaneWarperGpu(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPlaneWarperGpu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::PlaneWarperGpuTraitConst for core::Ptr<crate::stitching::PlaneWarperGpu> {
		#[inline] fn as_raw_PlaneWarperGpu(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::PlaneWarperGpuTrait for core::Ptr<crate::stitching::PlaneWarperGpu> {
		#[inline] fn as_raw_mut_PlaneWarperGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::PlaneWarperGpu> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::PlaneWarperGpu> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfPlaneWarperGpu, core::Ptr<crate::stitching::WarperCreator>, cv_PtrOfPlaneWarperGpu_to_PtrOfWarperCreator }
	
	pub type PtrOfSphericalWarper = core::Ptr<crate::stitching::SphericalWarper>;
	
	ptr_extern! { crate::stitching::SphericalWarper,
		cv_PtrOfSphericalWarper_delete, cv_PtrOfSphericalWarper_get_inner_ptr, cv_PtrOfSphericalWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::SphericalWarper, cv_PtrOfSphericalWarper_new }
	
	impl core::Ptr<crate::stitching::SphericalWarper> {
		#[inline] pub fn as_raw_PtrOfSphericalWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSphericalWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::SphericalWarperTraitConst for core::Ptr<crate::stitching::SphericalWarper> {
		#[inline] fn as_raw_SphericalWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::SphericalWarperTrait for core::Ptr<crate::stitching::SphericalWarper> {
		#[inline] fn as_raw_mut_SphericalWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::SphericalWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::SphericalWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSphericalWarper, core::Ptr<crate::stitching::WarperCreator>, cv_PtrOfSphericalWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfSphericalWarperGpu = core::Ptr<crate::stitching::SphericalWarperGpu>;
	
	ptr_extern! { crate::stitching::SphericalWarperGpu,
		cv_PtrOfSphericalWarperGpu_delete, cv_PtrOfSphericalWarperGpu_get_inner_ptr, cv_PtrOfSphericalWarperGpu_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::SphericalWarperGpu, cv_PtrOfSphericalWarperGpu_new }
	
	impl core::Ptr<crate::stitching::SphericalWarperGpu> {
		#[inline] pub fn as_raw_PtrOfSphericalWarperGpu(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSphericalWarperGpu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::SphericalWarperGpuTraitConst for core::Ptr<crate::stitching::SphericalWarperGpu> {
		#[inline] fn as_raw_SphericalWarperGpu(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::SphericalWarperGpuTrait for core::Ptr<crate::stitching::SphericalWarperGpu> {
		#[inline] fn as_raw_mut_SphericalWarperGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::SphericalWarperGpu> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::SphericalWarperGpu> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSphericalWarperGpu, core::Ptr<crate::stitching::WarperCreator>, cv_PtrOfSphericalWarperGpu_to_PtrOfWarperCreator }
	
	pub type PtrOfStereographicWarper = core::Ptr<crate::stitching::StereographicWarper>;
	
	ptr_extern! { crate::stitching::StereographicWarper,
		cv_PtrOfStereographicWarper_delete, cv_PtrOfStereographicWarper_get_inner_ptr, cv_PtrOfStereographicWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::StereographicWarper, cv_PtrOfStereographicWarper_new }
	
	impl core::Ptr<crate::stitching::StereographicWarper> {
		#[inline] pub fn as_raw_PtrOfStereographicWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStereographicWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::StereographicWarperTraitConst for core::Ptr<crate::stitching::StereographicWarper> {
		#[inline] fn as_raw_StereographicWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::StereographicWarperTrait for core::Ptr<crate::stitching::StereographicWarper> {
		#[inline] fn as_raw_mut_StereographicWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::StereographicWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::StereographicWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfStereographicWarper, core::Ptr<crate::stitching::WarperCreator>, cv_PtrOfStereographicWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfStitcher = core::Ptr<crate::stitching::Stitcher>;
	
	ptr_extern! { crate::stitching::Stitcher,
		cv_PtrOfStitcher_delete, cv_PtrOfStitcher_get_inner_ptr, cv_PtrOfStitcher_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::Stitcher, cv_PtrOfStitcher_new }
	
	impl core::Ptr<crate::stitching::Stitcher> {
		#[inline] pub fn as_raw_PtrOfStitcher(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStitcher(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::StitcherTraitConst for core::Ptr<crate::stitching::Stitcher> {
		#[inline] fn as_raw_Stitcher(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::StitcherTrait for core::Ptr<crate::stitching::Stitcher> {
		#[inline] fn as_raw_mut_Stitcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTransverseMercatorWarper = core::Ptr<crate::stitching::TransverseMercatorWarper>;
	
	ptr_extern! { crate::stitching::TransverseMercatorWarper,
		cv_PtrOfTransverseMercatorWarper_delete, cv_PtrOfTransverseMercatorWarper_get_inner_ptr, cv_PtrOfTransverseMercatorWarper_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::stitching::TransverseMercatorWarper, cv_PtrOfTransverseMercatorWarper_new }
	
	impl core::Ptr<crate::stitching::TransverseMercatorWarper> {
		#[inline] pub fn as_raw_PtrOfTransverseMercatorWarper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTransverseMercatorWarper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::TransverseMercatorWarperTraitConst for core::Ptr<crate::stitching::TransverseMercatorWarper> {
		#[inline] fn as_raw_TransverseMercatorWarper(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::TransverseMercatorWarperTrait for core::Ptr<crate::stitching::TransverseMercatorWarper> {
		#[inline] fn as_raw_mut_TransverseMercatorWarper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::TransverseMercatorWarper> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::TransverseMercatorWarper> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfTransverseMercatorWarper, core::Ptr<crate::stitching::WarperCreator>, cv_PtrOfTransverseMercatorWarper_to_PtrOfWarperCreator }
	
	pub type PtrOfWarperCreator = core::Ptr<crate::stitching::WarperCreator>;
	
	ptr_extern! { crate::stitching::WarperCreator,
		cv_PtrOfWarperCreator_delete, cv_PtrOfWarperCreator_get_inner_ptr, cv_PtrOfWarperCreator_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::stitching::WarperCreator> {
		#[inline] pub fn as_raw_PtrOfWarperCreator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfWarperCreator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::WarperCreator> {
		#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::WarperCreator> {
		#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfDetail_CameraParams = core::Vector<crate::stitching::Detail_CameraParams>;
	
	impl core::Vector<crate::stitching::Detail_CameraParams> {
		pub fn as_raw_VectorOfDetail_CameraParams(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetail_CameraParams(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::stitching::Detail_CameraParams,
		cv_VectorOfDetail_CameraParams_new, cv_VectorOfDetail_CameraParams_delete,
		cv_VectorOfDetail_CameraParams_len, cv_VectorOfDetail_CameraParams_is_empty,
		cv_VectorOfDetail_CameraParams_capacity, cv_VectorOfDetail_CameraParams_shrink_to_fit,
		cv_VectorOfDetail_CameraParams_reserve, cv_VectorOfDetail_CameraParams_remove,
		cv_VectorOfDetail_CameraParams_swap, cv_VectorOfDetail_CameraParams_clear,
		cv_VectorOfDetail_CameraParams_get, cv_VectorOfDetail_CameraParams_set,
		cv_VectorOfDetail_CameraParams_push, cv_VectorOfDetail_CameraParams_insert,
	}
	vector_non_copy_or_bool! { clone crate::stitching::Detail_CameraParams }
	
	pub type VectorOfDetail_ImageFeatures = core::Vector<crate::stitching::Detail_ImageFeatures>;
	
	impl core::Vector<crate::stitching::Detail_ImageFeatures> {
		pub fn as_raw_VectorOfDetail_ImageFeatures(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetail_ImageFeatures(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::stitching::Detail_ImageFeatures,
		cv_VectorOfDetail_ImageFeatures_new, cv_VectorOfDetail_ImageFeatures_delete,
		cv_VectorOfDetail_ImageFeatures_len, cv_VectorOfDetail_ImageFeatures_is_empty,
		cv_VectorOfDetail_ImageFeatures_capacity, cv_VectorOfDetail_ImageFeatures_shrink_to_fit,
		cv_VectorOfDetail_ImageFeatures_reserve, cv_VectorOfDetail_ImageFeatures_remove,
		cv_VectorOfDetail_ImageFeatures_swap, cv_VectorOfDetail_ImageFeatures_clear,
		cv_VectorOfDetail_ImageFeatures_get, cv_VectorOfDetail_ImageFeatures_set,
		cv_VectorOfDetail_ImageFeatures_push, cv_VectorOfDetail_ImageFeatures_insert,
	}
	vector_non_copy_or_bool! { clone crate::stitching::Detail_ImageFeatures }
	
	pub type VectorOfDetail_MatchesInfo = core::Vector<crate::stitching::Detail_MatchesInfo>;
	
	impl core::Vector<crate::stitching::Detail_MatchesInfo> {
		pub fn as_raw_VectorOfDetail_MatchesInfo(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDetail_MatchesInfo(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::stitching::Detail_MatchesInfo,
		cv_VectorOfDetail_MatchesInfo_new, cv_VectorOfDetail_MatchesInfo_delete,
		cv_VectorOfDetail_MatchesInfo_len, cv_VectorOfDetail_MatchesInfo_is_empty,
		cv_VectorOfDetail_MatchesInfo_capacity, cv_VectorOfDetail_MatchesInfo_shrink_to_fit,
		cv_VectorOfDetail_MatchesInfo_reserve, cv_VectorOfDetail_MatchesInfo_remove,
		cv_VectorOfDetail_MatchesInfo_swap, cv_VectorOfDetail_MatchesInfo_clear,
		cv_VectorOfDetail_MatchesInfo_get, cv_VectorOfDetail_MatchesInfo_set,
		cv_VectorOfDetail_MatchesInfo_push, cv_VectorOfDetail_MatchesInfo_insert,
	}
	vector_non_copy_or_bool! { clone crate::stitching::Detail_MatchesInfo }
	
}
#[cfg(ocvrs_has_module_stitching)]
pub use stitching_types::*;

#[cfg(ocvrs_has_module_structured_light)]
mod structured_light_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfGrayCodePattern = core::Ptr<crate::structured_light::GrayCodePattern>;
	
	ptr_extern! { crate::structured_light::GrayCodePattern,
		cv_PtrOfGrayCodePattern_delete, cv_PtrOfGrayCodePattern_get_inner_ptr, cv_PtrOfGrayCodePattern_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::structured_light::GrayCodePattern> {
		#[inline] pub fn as_raw_PtrOfGrayCodePattern(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGrayCodePattern(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::structured_light::GrayCodePatternTraitConst for core::Ptr<crate::structured_light::GrayCodePattern> {
		#[inline] fn as_raw_GrayCodePattern(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::structured_light::GrayCodePatternTrait for core::Ptr<crate::structured_light::GrayCodePattern> {
		#[inline] fn as_raw_mut_GrayCodePattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::structured_light::GrayCodePattern> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::structured_light::GrayCodePattern> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::structured_light::StructuredLightPatternTraitConst for core::Ptr<crate::structured_light::GrayCodePattern> {
		#[inline] fn as_raw_StructuredLightPattern(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::structured_light::StructuredLightPatternTrait for core::Ptr<crate::structured_light::GrayCodePattern> {
		#[inline] fn as_raw_mut_StructuredLightPattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSinusoidalPattern = core::Ptr<crate::structured_light::SinusoidalPattern>;
	
	ptr_extern! { crate::structured_light::SinusoidalPattern,
		cv_PtrOfSinusoidalPattern_delete, cv_PtrOfSinusoidalPattern_get_inner_ptr, cv_PtrOfSinusoidalPattern_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::structured_light::SinusoidalPattern> {
		#[inline] pub fn as_raw_PtrOfSinusoidalPattern(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSinusoidalPattern(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::structured_light::SinusoidalPatternTraitConst for core::Ptr<crate::structured_light::SinusoidalPattern> {
		#[inline] fn as_raw_SinusoidalPattern(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::structured_light::SinusoidalPatternTrait for core::Ptr<crate::structured_light::SinusoidalPattern> {
		#[inline] fn as_raw_mut_SinusoidalPattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::structured_light::SinusoidalPattern> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::structured_light::SinusoidalPattern> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::structured_light::StructuredLightPatternTraitConst for core::Ptr<crate::structured_light::SinusoidalPattern> {
		#[inline] fn as_raw_StructuredLightPattern(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::structured_light::StructuredLightPatternTrait for core::Ptr<crate::structured_light::SinusoidalPattern> {
		#[inline] fn as_raw_mut_StructuredLightPattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSinusoidalPattern_Params = core::Ptr<crate::structured_light::SinusoidalPattern_Params>;
	
	ptr_extern! { crate::structured_light::SinusoidalPattern_Params,
		cv_PtrOfSinusoidalPattern_Params_delete, cv_PtrOfSinusoidalPattern_Params_get_inner_ptr, cv_PtrOfSinusoidalPattern_Params_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::structured_light::SinusoidalPattern_Params, cv_PtrOfSinusoidalPattern_Params_new }
	
	impl core::Ptr<crate::structured_light::SinusoidalPattern_Params> {
		#[inline] pub fn as_raw_PtrOfSinusoidalPattern_Params(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSinusoidalPattern_Params(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::structured_light::SinusoidalPattern_ParamsTraitConst for core::Ptr<crate::structured_light::SinusoidalPattern_Params> {
		#[inline] fn as_raw_SinusoidalPattern_Params(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::structured_light::SinusoidalPattern_ParamsTrait for core::Ptr<crate::structured_light::SinusoidalPattern_Params> {
		#[inline] fn as_raw_mut_SinusoidalPattern_Params(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_structured_light)]
pub use structured_light_types::*;

#[cfg(ocvrs_has_module_superres)]
mod superres_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfSuperres_BroxOpticalFlow = core::Ptr<crate::superres::Superres_BroxOpticalFlow>;
	
	ptr_extern! { crate::superres::Superres_BroxOpticalFlow,
		cv_PtrOfSuperres_BroxOpticalFlow_delete, cv_PtrOfSuperres_BroxOpticalFlow_get_inner_ptr, cv_PtrOfSuperres_BroxOpticalFlow_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::superres::Superres_BroxOpticalFlow> {
		#[inline] pub fn as_raw_PtrOfSuperres_BroxOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperres_BroxOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::superres::Superres_BroxOpticalFlowTraitConst for core::Ptr<crate::superres::Superres_BroxOpticalFlow> {
		#[inline] fn as_raw_Superres_BroxOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_BroxOpticalFlowTrait for core::Ptr<crate::superres::Superres_BroxOpticalFlow> {
		#[inline] fn as_raw_mut_Superres_BroxOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::superres::Superres_BroxOpticalFlow> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::superres::Superres_BroxOpticalFlow> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExtTraitConst for core::Ptr<crate::superres::Superres_BroxOpticalFlow> {
		#[inline] fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExtTrait for core::Ptr<crate::superres::Superres_BroxOpticalFlow> {
		#[inline] fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_DenseOpticalFlowExt = core::Ptr<crate::superres::Superres_DenseOpticalFlowExt>;
	
	ptr_extern! { crate::superres::Superres_DenseOpticalFlowExt,
		cv_PtrOfSuperres_DenseOpticalFlowExt_delete, cv_PtrOfSuperres_DenseOpticalFlowExt_get_inner_ptr, cv_PtrOfSuperres_DenseOpticalFlowExt_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::superres::Superres_DenseOpticalFlowExt> {
		#[inline] pub fn as_raw_PtrOfSuperres_DenseOpticalFlowExt(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperres_DenseOpticalFlowExt(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExtTraitConst for core::Ptr<crate::superres::Superres_DenseOpticalFlowExt> {
		#[inline] fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExtTrait for core::Ptr<crate::superres::Superres_DenseOpticalFlowExt> {
		#[inline] fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::superres::Superres_DenseOpticalFlowExt> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::superres::Superres_DenseOpticalFlowExt> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_DualTVL1OpticalFlow = core::Ptr<crate::superres::Superres_DualTVL1OpticalFlow>;
	
	ptr_extern! { crate::superres::Superres_DualTVL1OpticalFlow,
		cv_PtrOfSuperres_DualTVL1OpticalFlow_delete, cv_PtrOfSuperres_DualTVL1OpticalFlow_get_inner_ptr, cv_PtrOfSuperres_DualTVL1OpticalFlow_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::superres::Superres_DualTVL1OpticalFlow> {
		#[inline] pub fn as_raw_PtrOfSuperres_DualTVL1OpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperres_DualTVL1OpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::superres::Superres_DualTVL1OpticalFlowTraitConst for core::Ptr<crate::superres::Superres_DualTVL1OpticalFlow> {
		#[inline] fn as_raw_Superres_DualTVL1OpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_DualTVL1OpticalFlowTrait for core::Ptr<crate::superres::Superres_DualTVL1OpticalFlow> {
		#[inline] fn as_raw_mut_Superres_DualTVL1OpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::superres::Superres_DualTVL1OpticalFlow> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::superres::Superres_DualTVL1OpticalFlow> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExtTraitConst for core::Ptr<crate::superres::Superres_DualTVL1OpticalFlow> {
		#[inline] fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExtTrait for core::Ptr<crate::superres::Superres_DualTVL1OpticalFlow> {
		#[inline] fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_FarnebackOpticalFlow = core::Ptr<crate::superres::Superres_FarnebackOpticalFlow>;
	
	ptr_extern! { crate::superres::Superres_FarnebackOpticalFlow,
		cv_PtrOfSuperres_FarnebackOpticalFlow_delete, cv_PtrOfSuperres_FarnebackOpticalFlow_get_inner_ptr, cv_PtrOfSuperres_FarnebackOpticalFlow_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::superres::Superres_FarnebackOpticalFlow> {
		#[inline] pub fn as_raw_PtrOfSuperres_FarnebackOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperres_FarnebackOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::superres::Superres_FarnebackOpticalFlowTraitConst for core::Ptr<crate::superres::Superres_FarnebackOpticalFlow> {
		#[inline] fn as_raw_Superres_FarnebackOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_FarnebackOpticalFlowTrait for core::Ptr<crate::superres::Superres_FarnebackOpticalFlow> {
		#[inline] fn as_raw_mut_Superres_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::superres::Superres_FarnebackOpticalFlow> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::superres::Superres_FarnebackOpticalFlow> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExtTraitConst for core::Ptr<crate::superres::Superres_FarnebackOpticalFlow> {
		#[inline] fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExtTrait for core::Ptr<crate::superres::Superres_FarnebackOpticalFlow> {
		#[inline] fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_FrameSource = core::Ptr<crate::superres::Superres_FrameSource>;
	
	ptr_extern! { crate::superres::Superres_FrameSource,
		cv_PtrOfSuperres_FrameSource_delete, cv_PtrOfSuperres_FrameSource_get_inner_ptr, cv_PtrOfSuperres_FrameSource_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::superres::Superres_FrameSource> {
		#[inline] pub fn as_raw_PtrOfSuperres_FrameSource(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperres_FrameSource(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::superres::Superres_FrameSourceTraitConst for core::Ptr<crate::superres::Superres_FrameSource> {
		#[inline] fn as_raw_Superres_FrameSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_FrameSourceTrait for core::Ptr<crate::superres::Superres_FrameSource> {
		#[inline] fn as_raw_mut_Superres_FrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_PyrLKOpticalFlow = core::Ptr<crate::superres::Superres_PyrLKOpticalFlow>;
	
	ptr_extern! { crate::superres::Superres_PyrLKOpticalFlow,
		cv_PtrOfSuperres_PyrLKOpticalFlow_delete, cv_PtrOfSuperres_PyrLKOpticalFlow_get_inner_ptr, cv_PtrOfSuperres_PyrLKOpticalFlow_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::superres::Superres_PyrLKOpticalFlow> {
		#[inline] pub fn as_raw_PtrOfSuperres_PyrLKOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperres_PyrLKOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::superres::Superres_PyrLKOpticalFlowTraitConst for core::Ptr<crate::superres::Superres_PyrLKOpticalFlow> {
		#[inline] fn as_raw_Superres_PyrLKOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_PyrLKOpticalFlowTrait for core::Ptr<crate::superres::Superres_PyrLKOpticalFlow> {
		#[inline] fn as_raw_mut_Superres_PyrLKOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::superres::Superres_PyrLKOpticalFlow> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::superres::Superres_PyrLKOpticalFlow> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExtTraitConst for core::Ptr<crate::superres::Superres_PyrLKOpticalFlow> {
		#[inline] fn as_raw_Superres_DenseOpticalFlowExt(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_DenseOpticalFlowExtTrait for core::Ptr<crate::superres::Superres_PyrLKOpticalFlow> {
		#[inline] fn as_raw_mut_Superres_DenseOpticalFlowExt(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperres_SuperResolution = core::Ptr<crate::superres::Superres_SuperResolution>;
	
	ptr_extern! { crate::superres::Superres_SuperResolution,
		cv_PtrOfSuperres_SuperResolution_delete, cv_PtrOfSuperres_SuperResolution_get_inner_ptr, cv_PtrOfSuperres_SuperResolution_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::superres::Superres_SuperResolution> {
		#[inline] pub fn as_raw_PtrOfSuperres_SuperResolution(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperres_SuperResolution(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::superres::Superres_SuperResolutionTraitConst for core::Ptr<crate::superres::Superres_SuperResolution> {
		#[inline] fn as_raw_Superres_SuperResolution(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_SuperResolutionTrait for core::Ptr<crate::superres::Superres_SuperResolution> {
		#[inline] fn as_raw_mut_Superres_SuperResolution(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::superres::Superres_SuperResolution> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::superres::Superres_SuperResolution> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::superres::Superres_FrameSourceTraitConst for core::Ptr<crate::superres::Superres_SuperResolution> {
		#[inline] fn as_raw_Superres_FrameSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::superres::Superres_FrameSourceTrait for core::Ptr<crate::superres::Superres_SuperResolution> {
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
	
	impl core::Ptr<crate::surface_matching::Pose3D> {
		#[inline] pub fn as_raw_PtrOfPose3D(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPose3D(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::surface_matching::Pose3DTraitConst for core::Ptr<crate::surface_matching::Pose3D> {
		#[inline] fn as_raw_Pose3D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::surface_matching::Pose3DTrait for core::Ptr<crate::surface_matching::Pose3D> {
		#[inline] fn as_raw_mut_Pose3D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPoseCluster3D = core::Ptr<crate::surface_matching::PoseCluster3D>;
	
	ptr_extern! { crate::surface_matching::PoseCluster3D,
		cv_PtrOfPoseCluster3D_delete, cv_PtrOfPoseCluster3D_get_inner_ptr, cv_PtrOfPoseCluster3D_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::surface_matching::PoseCluster3D, cv_PtrOfPoseCluster3D_new }
	
	impl core::Ptr<crate::surface_matching::PoseCluster3D> {
		#[inline] pub fn as_raw_PtrOfPoseCluster3D(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPoseCluster3D(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::surface_matching::PoseCluster3DTraitConst for core::Ptr<crate::surface_matching::PoseCluster3D> {
		#[inline] fn as_raw_PoseCluster3D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::surface_matching::PoseCluster3DTrait for core::Ptr<crate::surface_matching::PoseCluster3D> {
		#[inline] fn as_raw_mut_PoseCluster3D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfPose3DPtr = core::Vector<crate::surface_matching::Pose3DPtr>;
	
	impl core::Vector<crate::surface_matching::Pose3DPtr> {
		pub fn as_raw_VectorOfPose3DPtr(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPose3DPtr(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::surface_matching::Pose3DPtr,
		cv_VectorOfPose3DPtr_new, cv_VectorOfPose3DPtr_delete,
		cv_VectorOfPose3DPtr_len, cv_VectorOfPose3DPtr_is_empty,
		cv_VectorOfPose3DPtr_capacity, cv_VectorOfPose3DPtr_shrink_to_fit,
		cv_VectorOfPose3DPtr_reserve, cv_VectorOfPose3DPtr_remove,
		cv_VectorOfPose3DPtr_swap, cv_VectorOfPose3DPtr_clear,
		cv_VectorOfPose3DPtr_get, cv_VectorOfPose3DPtr_set,
		cv_VectorOfPose3DPtr_push, cv_VectorOfPose3DPtr_insert,
	}
	vector_non_copy_or_bool! { crate::surface_matching::Pose3DPtr }
	
}
#[cfg(ocvrs_has_module_surface_matching)]
pub use surface_matching_types::*;

#[cfg(ocvrs_has_module_text)]
mod text_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfERFilter = core::Ptr<crate::text::ERFilter>;
	
	ptr_extern! { crate::text::ERFilter,
		cv_PtrOfERFilter_delete, cv_PtrOfERFilter_get_inner_ptr, cv_PtrOfERFilter_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::text::ERFilter> {
		#[inline] pub fn as_raw_PtrOfERFilter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfERFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::text::ERFilterTraitConst for core::Ptr<crate::text::ERFilter> {
		#[inline] fn as_raw_ERFilter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::ERFilterTrait for core::Ptr<crate::text::ERFilter> {
		#[inline] fn as_raw_mut_ERFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::text::ERFilter> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::text::ERFilter> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfERFilter_Callback = core::Ptr<crate::text::ERFilter_Callback>;
	
	ptr_extern! { crate::text::ERFilter_Callback,
		cv_PtrOfERFilter_Callback_delete, cv_PtrOfERFilter_Callback_get_inner_ptr, cv_PtrOfERFilter_Callback_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::text::ERFilter_Callback> {
		#[inline] pub fn as_raw_PtrOfERFilter_Callback(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfERFilter_Callback(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::text::ERFilter_CallbackTraitConst for core::Ptr<crate::text::ERFilter_Callback> {
		#[inline] fn as_raw_ERFilter_Callback(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::ERFilter_CallbackTrait for core::Ptr<crate::text::ERFilter_Callback> {
		#[inline] fn as_raw_mut_ERFilter_Callback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRBeamSearchDecoder = core::Ptr<crate::text::OCRBeamSearchDecoder>;
	
	ptr_extern! { crate::text::OCRBeamSearchDecoder,
		cv_PtrOfOCRBeamSearchDecoder_delete, cv_PtrOfOCRBeamSearchDecoder_get_inner_ptr, cv_PtrOfOCRBeamSearchDecoder_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::text::OCRBeamSearchDecoder, cv_PtrOfOCRBeamSearchDecoder_new }
	
	impl core::Ptr<crate::text::OCRBeamSearchDecoder> {
		#[inline] pub fn as_raw_PtrOfOCRBeamSearchDecoder(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOCRBeamSearchDecoder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::text::OCRBeamSearchDecoderTraitConst for core::Ptr<crate::text::OCRBeamSearchDecoder> {
		#[inline] fn as_raw_OCRBeamSearchDecoder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::OCRBeamSearchDecoderTrait for core::Ptr<crate::text::OCRBeamSearchDecoder> {
		#[inline] fn as_raw_mut_OCRBeamSearchDecoder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::text::BaseOCRTraitConst for core::Ptr<crate::text::OCRBeamSearchDecoder> {
		#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::BaseOCRTrait for core::Ptr<crate::text::OCRBeamSearchDecoder> {
		#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRBeamSearchDecoder_ClassifierCallback = core::Ptr<crate::text::OCRBeamSearchDecoder_ClassifierCallback>;
	
	ptr_extern! { crate::text::OCRBeamSearchDecoder_ClassifierCallback,
		cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_delete, cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_get_inner_ptr, cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::text::OCRBeamSearchDecoder_ClassifierCallback, cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_new }
	
	impl core::Ptr<crate::text::OCRBeamSearchDecoder_ClassifierCallback> {
		#[inline] pub fn as_raw_PtrOfOCRBeamSearchDecoder_ClassifierCallback(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOCRBeamSearchDecoder_ClassifierCallback(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::text::OCRBeamSearchDecoder_ClassifierCallbackTraitConst for core::Ptr<crate::text::OCRBeamSearchDecoder_ClassifierCallback> {
		#[inline] fn as_raw_OCRBeamSearchDecoder_ClassifierCallback(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::OCRBeamSearchDecoder_ClassifierCallbackTrait for core::Ptr<crate::text::OCRBeamSearchDecoder_ClassifierCallback> {
		#[inline] fn as_raw_mut_OCRBeamSearchDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRHMMDecoder = core::Ptr<crate::text::OCRHMMDecoder>;
	
	ptr_extern! { crate::text::OCRHMMDecoder,
		cv_PtrOfOCRHMMDecoder_delete, cv_PtrOfOCRHMMDecoder_get_inner_ptr, cv_PtrOfOCRHMMDecoder_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::text::OCRHMMDecoder, cv_PtrOfOCRHMMDecoder_new }
	
	impl core::Ptr<crate::text::OCRHMMDecoder> {
		#[inline] pub fn as_raw_PtrOfOCRHMMDecoder(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOCRHMMDecoder(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::text::OCRHMMDecoderTraitConst for core::Ptr<crate::text::OCRHMMDecoder> {
		#[inline] fn as_raw_OCRHMMDecoder(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::OCRHMMDecoderTrait for core::Ptr<crate::text::OCRHMMDecoder> {
		#[inline] fn as_raw_mut_OCRHMMDecoder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::text::BaseOCRTraitConst for core::Ptr<crate::text::OCRHMMDecoder> {
		#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::BaseOCRTrait for core::Ptr<crate::text::OCRHMMDecoder> {
		#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRHMMDecoder_ClassifierCallback = core::Ptr<crate::text::OCRHMMDecoder_ClassifierCallback>;
	
	ptr_extern! { crate::text::OCRHMMDecoder_ClassifierCallback,
		cv_PtrOfOCRHMMDecoder_ClassifierCallback_delete, cv_PtrOfOCRHMMDecoder_ClassifierCallback_get_inner_ptr, cv_PtrOfOCRHMMDecoder_ClassifierCallback_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::text::OCRHMMDecoder_ClassifierCallback, cv_PtrOfOCRHMMDecoder_ClassifierCallback_new }
	
	impl core::Ptr<crate::text::OCRHMMDecoder_ClassifierCallback> {
		#[inline] pub fn as_raw_PtrOfOCRHMMDecoder_ClassifierCallback(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOCRHMMDecoder_ClassifierCallback(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::text::OCRHMMDecoder_ClassifierCallbackTraitConst for core::Ptr<crate::text::OCRHMMDecoder_ClassifierCallback> {
		#[inline] fn as_raw_OCRHMMDecoder_ClassifierCallback(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::OCRHMMDecoder_ClassifierCallbackTrait for core::Ptr<crate::text::OCRHMMDecoder_ClassifierCallback> {
		#[inline] fn as_raw_mut_OCRHMMDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRHolisticWordRecognizer = core::Ptr<crate::text::OCRHolisticWordRecognizer>;
	
	ptr_extern! { crate::text::OCRHolisticWordRecognizer,
		cv_PtrOfOCRHolisticWordRecognizer_delete, cv_PtrOfOCRHolisticWordRecognizer_get_inner_ptr, cv_PtrOfOCRHolisticWordRecognizer_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::text::OCRHolisticWordRecognizer> {
		#[inline] pub fn as_raw_PtrOfOCRHolisticWordRecognizer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOCRHolisticWordRecognizer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::text::OCRHolisticWordRecognizerTraitConst for core::Ptr<crate::text::OCRHolisticWordRecognizer> {
		#[inline] fn as_raw_OCRHolisticWordRecognizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::OCRHolisticWordRecognizerTrait for core::Ptr<crate::text::OCRHolisticWordRecognizer> {
		#[inline] fn as_raw_mut_OCRHolisticWordRecognizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::text::BaseOCRTraitConst for core::Ptr<crate::text::OCRHolisticWordRecognizer> {
		#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::BaseOCRTrait for core::Ptr<crate::text::OCRHolisticWordRecognizer> {
		#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfOCRTesseract = core::Ptr<crate::text::OCRTesseract>;
	
	ptr_extern! { crate::text::OCRTesseract,
		cv_PtrOfOCRTesseract_delete, cv_PtrOfOCRTesseract_get_inner_ptr, cv_PtrOfOCRTesseract_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::text::OCRTesseract> {
		#[inline] pub fn as_raw_PtrOfOCRTesseract(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOCRTesseract(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::text::OCRTesseractTraitConst for core::Ptr<crate::text::OCRTesseract> {
		#[inline] fn as_raw_OCRTesseract(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::OCRTesseractTrait for core::Ptr<crate::text::OCRTesseract> {
		#[inline] fn as_raw_mut_OCRTesseract(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::text::BaseOCRTraitConst for core::Ptr<crate::text::OCRTesseract> {
		#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::BaseOCRTrait for core::Ptr<crate::text::OCRTesseract> {
		#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTextDetectorCNN = core::Ptr<crate::text::TextDetectorCNN>;
	
	ptr_extern! { crate::text::TextDetectorCNN,
		cv_PtrOfTextDetectorCNN_delete, cv_PtrOfTextDetectorCNN_get_inner_ptr, cv_PtrOfTextDetectorCNN_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::text::TextDetectorCNN> {
		#[inline] pub fn as_raw_PtrOfTextDetectorCNN(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTextDetectorCNN(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::text::TextDetectorCNNTraitConst for core::Ptr<crate::text::TextDetectorCNN> {
		#[inline] fn as_raw_TextDetectorCNN(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::TextDetectorCNNTrait for core::Ptr<crate::text::TextDetectorCNN> {
		#[inline] fn as_raw_mut_TextDetectorCNN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::text::TextDetectorTraitConst for core::Ptr<crate::text::TextDetectorCNN> {
		#[inline] fn as_raw_TextDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::text::TextDetectorTrait for core::Ptr<crate::text::TextDetectorCNN> {
		#[inline] fn as_raw_mut_TextDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type VectorOfERStat = core::Vector<crate::text::ERStat>;
	
	impl core::Vector<crate::text::ERStat> {
		pub fn as_raw_VectorOfERStat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfERStat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::text::ERStat,
		cv_VectorOfERStat_new, cv_VectorOfERStat_delete,
		cv_VectorOfERStat_len, cv_VectorOfERStat_is_empty,
		cv_VectorOfERStat_capacity, cv_VectorOfERStat_shrink_to_fit,
		cv_VectorOfERStat_reserve, cv_VectorOfERStat_remove,
		cv_VectorOfERStat_swap, cv_VectorOfERStat_clear,
		cv_VectorOfERStat_get, cv_VectorOfERStat_set,
		cv_VectorOfERStat_push, cv_VectorOfERStat_insert,
	}
	vector_non_copy_or_bool! { crate::text::ERStat }
	
	pub type VectorOfVectorOfERStat = core::Vector<core::Vector<crate::text::ERStat>>;
	
	impl core::Vector<core::Vector<crate::text::ERStat>> {
		pub fn as_raw_VectorOfVectorOfERStat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfERStat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { core::Vector<crate::text::ERStat>,
		cv_VectorOfVectorOfERStat_new, cv_VectorOfVectorOfERStat_delete,
		cv_VectorOfVectorOfERStat_len, cv_VectorOfVectorOfERStat_is_empty,
		cv_VectorOfVectorOfERStat_capacity, cv_VectorOfVectorOfERStat_shrink_to_fit,
		cv_VectorOfVectorOfERStat_reserve, cv_VectorOfVectorOfERStat_remove,
		cv_VectorOfVectorOfERStat_swap, cv_VectorOfVectorOfERStat_clear,
		cv_VectorOfVectorOfERStat_get, cv_VectorOfVectorOfERStat_set,
		cv_VectorOfVectorOfERStat_push, cv_VectorOfVectorOfERStat_insert,
	}
	vector_non_copy_or_bool! { core::Vector<crate::text::ERStat> }
	
}
#[cfg(ocvrs_has_module_text)]
pub use text_types::*;

#[cfg(ocvrs_has_module_tracking)]
mod tracking_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfTrackerCSRT = core::Ptr<crate::tracking::TrackerCSRT>;
	
	ptr_extern! { crate::tracking::TrackerCSRT,
		cv_PtrOfTrackerCSRT_delete, cv_PtrOfTrackerCSRT_get_inner_ptr, cv_PtrOfTrackerCSRT_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::tracking::TrackerCSRT> {
		#[inline] pub fn as_raw_PtrOfTrackerCSRT(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerCSRT(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::tracking::TrackerCSRTTraitConst for core::Ptr<crate::tracking::TrackerCSRT> {
		#[inline] fn as_raw_TrackerCSRT(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::tracking::TrackerCSRTTrait for core::Ptr<crate::tracking::TrackerCSRT> {
		#[inline] fn as_raw_mut_TrackerCSRT(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::TrackerTraitConst for core::Ptr<crate::tracking::TrackerCSRT> {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::TrackerTrait for core::Ptr<crate::tracking::TrackerCSRT> {
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerKCF = core::Ptr<crate::tracking::TrackerKCF>;
	
	ptr_extern! { crate::tracking::TrackerKCF,
		cv_PtrOfTrackerKCF_delete, cv_PtrOfTrackerKCF_get_inner_ptr, cv_PtrOfTrackerKCF_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::tracking::TrackerKCF> {
		#[inline] pub fn as_raw_PtrOfTrackerKCF(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerKCF(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::tracking::TrackerKCFTraitConst for core::Ptr<crate::tracking::TrackerKCF> {
		#[inline] fn as_raw_TrackerKCF(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::tracking::TrackerKCFTrait for core::Ptr<crate::tracking::TrackerKCF> {
		#[inline] fn as_raw_mut_TrackerKCF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::TrackerTraitConst for core::Ptr<crate::tracking::TrackerKCF> {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::TrackerTrait for core::Ptr<crate::tracking::TrackerKCF> {
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_tracking)]
pub use tracking_types::*;

#[cfg(ocvrs_has_module_video)]
mod video_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfBackgroundSubtractorKNN = core::Ptr<crate::video::BackgroundSubtractorKNN>;
	
	ptr_extern! { crate::video::BackgroundSubtractorKNN,
		cv_PtrOfBackgroundSubtractorKNN_delete, cv_PtrOfBackgroundSubtractorKNN_get_inner_ptr, cv_PtrOfBackgroundSubtractorKNN_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::video::BackgroundSubtractorKNN> {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorKNN(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorKNN(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorKNNTraitConst for core::Ptr<crate::video::BackgroundSubtractorKNN> {
		#[inline] fn as_raw_BackgroundSubtractorKNN(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractorKNNTrait for core::Ptr<crate::video::BackgroundSubtractorKNN> {
		#[inline] fn as_raw_mut_BackgroundSubtractorKNN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::video::BackgroundSubtractorKNN> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::video::BackgroundSubtractorKNN> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorTraitConst for core::Ptr<crate::video::BackgroundSubtractorKNN> {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractorTrait for core::Ptr<crate::video::BackgroundSubtractorKNN> {
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfBackgroundSubtractorMOG2 = core::Ptr<crate::video::BackgroundSubtractorMOG2>;
	
	ptr_extern! { crate::video::BackgroundSubtractorMOG2,
		cv_PtrOfBackgroundSubtractorMOG2_delete, cv_PtrOfBackgroundSubtractorMOG2_get_inner_ptr, cv_PtrOfBackgroundSubtractorMOG2_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::video::BackgroundSubtractorMOG2> {
		#[inline] pub fn as_raw_PtrOfBackgroundSubtractorMOG2(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorMOG2(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorMOG2TraitConst for core::Ptr<crate::video::BackgroundSubtractorMOG2> {
		#[inline] fn as_raw_BackgroundSubtractorMOG2(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractorMOG2Trait for core::Ptr<crate::video::BackgroundSubtractorMOG2> {
		#[inline] fn as_raw_mut_BackgroundSubtractorMOG2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::video::BackgroundSubtractorMOG2> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::video::BackgroundSubtractorMOG2> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::BackgroundSubtractorTraitConst for core::Ptr<crate::video::BackgroundSubtractorMOG2> {
		#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::BackgroundSubtractorTrait for core::Ptr<crate::video::BackgroundSubtractorMOG2> {
		#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDISOpticalFlow = core::Ptr<crate::video::DISOpticalFlow>;
	
	ptr_extern! { crate::video::DISOpticalFlow,
		cv_PtrOfDISOpticalFlow_delete, cv_PtrOfDISOpticalFlow_get_inner_ptr, cv_PtrOfDISOpticalFlow_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::video::DISOpticalFlow> {
		#[inline] pub fn as_raw_PtrOfDISOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDISOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::video::DISOpticalFlowTraitConst for core::Ptr<crate::video::DISOpticalFlow> {
		#[inline] fn as_raw_DISOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::DISOpticalFlowTrait for core::Ptr<crate::video::DISOpticalFlow> {
		#[inline] fn as_raw_mut_DISOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::video::DISOpticalFlow> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::video::DISOpticalFlow> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DenseOpticalFlowTraitConst for core::Ptr<crate::video::DISOpticalFlow> {
		#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::DenseOpticalFlowTrait for core::Ptr<crate::video::DISOpticalFlow> {
		#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDenseOpticalFlow = core::Ptr<crate::video::DenseOpticalFlow>;
	
	ptr_extern! { crate::video::DenseOpticalFlow,
		cv_PtrOfDenseOpticalFlow_delete, cv_PtrOfDenseOpticalFlow_get_inner_ptr, cv_PtrOfDenseOpticalFlow_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::video::DenseOpticalFlow> {
		#[inline] pub fn as_raw_PtrOfDenseOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDenseOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::video::DenseOpticalFlowTraitConst for core::Ptr<crate::video::DenseOpticalFlow> {
		#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::DenseOpticalFlowTrait for core::Ptr<crate::video::DenseOpticalFlow> {
		#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::video::DenseOpticalFlow> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::video::DenseOpticalFlow> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFarnebackOpticalFlow = core::Ptr<crate::video::FarnebackOpticalFlow>;
	
	ptr_extern! { crate::video::FarnebackOpticalFlow,
		cv_PtrOfFarnebackOpticalFlow_delete, cv_PtrOfFarnebackOpticalFlow_get_inner_ptr, cv_PtrOfFarnebackOpticalFlow_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::video::FarnebackOpticalFlow> {
		#[inline] pub fn as_raw_PtrOfFarnebackOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFarnebackOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::video::FarnebackOpticalFlowTraitConst for core::Ptr<crate::video::FarnebackOpticalFlow> {
		#[inline] fn as_raw_FarnebackOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::FarnebackOpticalFlowTrait for core::Ptr<crate::video::FarnebackOpticalFlow> {
		#[inline] fn as_raw_mut_FarnebackOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::video::FarnebackOpticalFlow> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::video::FarnebackOpticalFlow> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DenseOpticalFlowTraitConst for core::Ptr<crate::video::FarnebackOpticalFlow> {
		#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::DenseOpticalFlowTrait for core::Ptr<crate::video::FarnebackOpticalFlow> {
		#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSparseOpticalFlow = core::Ptr<crate::video::SparseOpticalFlow>;
	
	ptr_extern! { crate::video::SparseOpticalFlow,
		cv_PtrOfSparseOpticalFlow_delete, cv_PtrOfSparseOpticalFlow_get_inner_ptr, cv_PtrOfSparseOpticalFlow_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::video::SparseOpticalFlow> {
		#[inline] pub fn as_raw_PtrOfSparseOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSparseOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::video::SparseOpticalFlowTraitConst for core::Ptr<crate::video::SparseOpticalFlow> {
		#[inline] fn as_raw_SparseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::SparseOpticalFlowTrait for core::Ptr<crate::video::SparseOpticalFlow> {
		#[inline] fn as_raw_mut_SparseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::video::SparseOpticalFlow> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::video::SparseOpticalFlow> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSparsePyrLKOpticalFlow = core::Ptr<crate::video::SparsePyrLKOpticalFlow>;
	
	ptr_extern! { crate::video::SparsePyrLKOpticalFlow,
		cv_PtrOfSparsePyrLKOpticalFlow_delete, cv_PtrOfSparsePyrLKOpticalFlow_get_inner_ptr, cv_PtrOfSparsePyrLKOpticalFlow_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::video::SparsePyrLKOpticalFlow> {
		#[inline] pub fn as_raw_PtrOfSparsePyrLKOpticalFlow(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSparsePyrLKOpticalFlow(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::video::SparsePyrLKOpticalFlowTraitConst for core::Ptr<crate::video::SparsePyrLKOpticalFlow> {
		#[inline] fn as_raw_SparsePyrLKOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::SparsePyrLKOpticalFlowTrait for core::Ptr<crate::video::SparsePyrLKOpticalFlow> {
		#[inline] fn as_raw_mut_SparsePyrLKOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::video::SparsePyrLKOpticalFlow> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::video::SparsePyrLKOpticalFlow> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::SparseOpticalFlowTraitConst for core::Ptr<crate::video::SparsePyrLKOpticalFlow> {
		#[inline] fn as_raw_SparseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::SparseOpticalFlowTrait for core::Ptr<crate::video::SparsePyrLKOpticalFlow> {
		#[inline] fn as_raw_mut_SparseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerDaSiamRPN = core::Ptr<crate::video::TrackerDaSiamRPN>;
	
	ptr_extern! { crate::video::TrackerDaSiamRPN,
		cv_PtrOfTrackerDaSiamRPN_delete, cv_PtrOfTrackerDaSiamRPN_get_inner_ptr, cv_PtrOfTrackerDaSiamRPN_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::video::TrackerDaSiamRPN> {
		#[inline] pub fn as_raw_PtrOfTrackerDaSiamRPN(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerDaSiamRPN(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::video::TrackerDaSiamRPNTraitConst for core::Ptr<crate::video::TrackerDaSiamRPN> {
		#[inline] fn as_raw_TrackerDaSiamRPN(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::TrackerDaSiamRPNTrait for core::Ptr<crate::video::TrackerDaSiamRPN> {
		#[inline] fn as_raw_mut_TrackerDaSiamRPN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::TrackerTraitConst for core::Ptr<crate::video::TrackerDaSiamRPN> {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::TrackerTrait for core::Ptr<crate::video::TrackerDaSiamRPN> {
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerGOTURN = core::Ptr<crate::video::TrackerGOTURN>;
	
	ptr_extern! { crate::video::TrackerGOTURN,
		cv_PtrOfTrackerGOTURN_delete, cv_PtrOfTrackerGOTURN_get_inner_ptr, cv_PtrOfTrackerGOTURN_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::video::TrackerGOTURN> {
		#[inline] pub fn as_raw_PtrOfTrackerGOTURN(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerGOTURN(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::video::TrackerGOTURNTraitConst for core::Ptr<crate::video::TrackerGOTURN> {
		#[inline] fn as_raw_TrackerGOTURN(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::TrackerGOTURNTrait for core::Ptr<crate::video::TrackerGOTURN> {
		#[inline] fn as_raw_mut_TrackerGOTURN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::TrackerTraitConst for core::Ptr<crate::video::TrackerGOTURN> {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::TrackerTrait for core::Ptr<crate::video::TrackerGOTURN> {
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerMIL = core::Ptr<crate::video::TrackerMIL>;
	
	ptr_extern! { crate::video::TrackerMIL,
		cv_PtrOfTrackerMIL_delete, cv_PtrOfTrackerMIL_get_inner_ptr, cv_PtrOfTrackerMIL_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::video::TrackerMIL> {
		#[inline] pub fn as_raw_PtrOfTrackerMIL(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerMIL(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::video::TrackerMILTraitConst for core::Ptr<crate::video::TrackerMIL> {
		#[inline] fn as_raw_TrackerMIL(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::TrackerMILTrait for core::Ptr<crate::video::TrackerMIL> {
		#[inline] fn as_raw_mut_TrackerMIL(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::TrackerTraitConst for core::Ptr<crate::video::TrackerMIL> {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::TrackerTrait for core::Ptr<crate::video::TrackerMIL> {
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTrackerNano = core::Ptr<crate::video::TrackerNano>;
	
	ptr_extern! { crate::video::TrackerNano,
		cv_PtrOfTrackerNano_delete, cv_PtrOfTrackerNano_get_inner_ptr, cv_PtrOfTrackerNano_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::video::TrackerNano> {
		#[inline] pub fn as_raw_PtrOfTrackerNano(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTrackerNano(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::video::TrackerNanoTraitConst for core::Ptr<crate::video::TrackerNano> {
		#[inline] fn as_raw_TrackerNano(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::TrackerNanoTrait for core::Ptr<crate::video::TrackerNano> {
		#[inline] fn as_raw_mut_TrackerNano(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::TrackerTraitConst for core::Ptr<crate::video::TrackerNano> {
		#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::TrackerTrait for core::Ptr<crate::video::TrackerNano> {
		#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfVariationalRefinement = core::Ptr<crate::video::VariationalRefinement>;
	
	ptr_extern! { crate::video::VariationalRefinement,
		cv_PtrOfVariationalRefinement_delete, cv_PtrOfVariationalRefinement_get_inner_ptr, cv_PtrOfVariationalRefinement_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::video::VariationalRefinement> {
		#[inline] pub fn as_raw_PtrOfVariationalRefinement(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfVariationalRefinement(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::video::VariationalRefinementTraitConst for core::Ptr<crate::video::VariationalRefinement> {
		#[inline] fn as_raw_VariationalRefinement(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::VariationalRefinementTrait for core::Ptr<crate::video::VariationalRefinement> {
		#[inline] fn as_raw_mut_VariationalRefinement(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::video::VariationalRefinement> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::video::VariationalRefinement> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::video::DenseOpticalFlowTraitConst for core::Ptr<crate::video::VariationalRefinement> {
		#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::video::DenseOpticalFlowTrait for core::Ptr<crate::video::VariationalRefinement> {
		#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_video)]
pub use video_types::*;

#[cfg(ocvrs_has_module_videoio)]
mod videoio_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type VectorOfVideoCapture = core::Vector<crate::videoio::VideoCapture>;
	
	impl core::Vector<crate::videoio::VideoCapture> {
		pub fn as_raw_VectorOfVideoCapture(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVideoCapture(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::videoio::VideoCapture,
		cv_VectorOfVideoCapture_new, cv_VectorOfVideoCapture_delete,
		cv_VectorOfVideoCapture_len, cv_VectorOfVideoCapture_is_empty,
		cv_VectorOfVideoCapture_capacity, cv_VectorOfVideoCapture_shrink_to_fit,
		cv_VectorOfVideoCapture_reserve, cv_VectorOfVideoCapture_remove,
		cv_VectorOfVideoCapture_swap, cv_VectorOfVideoCapture_clear,
		cv_VectorOfVideoCapture_get, cv_VectorOfVideoCapture_set,
		cv_VectorOfVideoCapture_push, cv_VectorOfVideoCapture_insert,
	}
	vector_non_copy_or_bool! { crate::videoio::VideoCapture }
	
	pub type VectorOfVideoCaptureAPIs = core::Vector<crate::videoio::VideoCaptureAPIs>;
	
	impl core::Vector<crate::videoio::VideoCaptureAPIs> {
		pub fn as_raw_VectorOfVideoCaptureAPIs(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVideoCaptureAPIs(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::videoio::VideoCaptureAPIs,
		cv_VectorOfVideoCaptureAPIs_new, cv_VectorOfVideoCaptureAPIs_delete,
		cv_VectorOfVideoCaptureAPIs_len, cv_VectorOfVideoCaptureAPIs_is_empty,
		cv_VectorOfVideoCaptureAPIs_capacity, cv_VectorOfVideoCaptureAPIs_shrink_to_fit,
		cv_VectorOfVideoCaptureAPIs_reserve, cv_VectorOfVideoCaptureAPIs_remove,
		cv_VectorOfVideoCaptureAPIs_swap, cv_VectorOfVideoCaptureAPIs_clear,
		cv_VectorOfVideoCaptureAPIs_get, cv_VectorOfVideoCaptureAPIs_set,
		cv_VectorOfVideoCaptureAPIs_push, cv_VectorOfVideoCaptureAPIs_insert,
	}
	vector_copy_non_bool! { crate::videoio::VideoCaptureAPIs,
		cv_VectorOfVideoCaptureAPIs_data, cv_VectorOfVideoCaptureAPIs_data_mut, cv_VectorOfVideoCaptureAPIs_from_slice,
		cv_VectorOfVideoCaptureAPIs_clone,
	}
	
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
	
	impl core::Ptr<crate::videostab::ColorAverageInpainter> {
		#[inline] pub fn as_raw_PtrOfColorAverageInpainter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfColorAverageInpainter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::ColorAverageInpainterTraitConst for core::Ptr<crate::videostab::ColorAverageInpainter> {
		#[inline] fn as_raw_ColorAverageInpainter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ColorAverageInpainterTrait for core::Ptr<crate::videostab::ColorAverageInpainter> {
		#[inline] fn as_raw_mut_ColorAverageInpainter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::InpainterBaseTraitConst for core::Ptr<crate::videostab::ColorAverageInpainter> {
		#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::InpainterBaseTrait for core::Ptr<crate::videostab::ColorAverageInpainter> {
		#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfColorAverageInpainter, core::Ptr<crate::videostab::InpainterBase>, cv_PtrOfColorAverageInpainter_to_PtrOfInpainterBase }
	
	pub type PtrOfColorInpainter = core::Ptr<crate::videostab::ColorInpainter>;
	
	ptr_extern! { crate::videostab::ColorInpainter,
		cv_PtrOfColorInpainter_delete, cv_PtrOfColorInpainter_get_inner_ptr, cv_PtrOfColorInpainter_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::ColorInpainter, cv_PtrOfColorInpainter_new }
	
	impl core::Ptr<crate::videostab::ColorInpainter> {
		#[inline] pub fn as_raw_PtrOfColorInpainter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfColorInpainter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::ColorInpainterTraitConst for core::Ptr<crate::videostab::ColorInpainter> {
		#[inline] fn as_raw_ColorInpainter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ColorInpainterTrait for core::Ptr<crate::videostab::ColorInpainter> {
		#[inline] fn as_raw_mut_ColorInpainter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::InpainterBaseTraitConst for core::Ptr<crate::videostab::ColorInpainter> {
		#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::InpainterBaseTrait for core::Ptr<crate::videostab::ColorInpainter> {
		#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfColorInpainter, core::Ptr<crate::videostab::InpainterBase>, cv_PtrOfColorInpainter_to_PtrOfInpainterBase }
	
	pub type PtrOfConsistentMosaicInpainter = core::Ptr<crate::videostab::ConsistentMosaicInpainter>;
	
	ptr_extern! { crate::videostab::ConsistentMosaicInpainter,
		cv_PtrOfConsistentMosaicInpainter_delete, cv_PtrOfConsistentMosaicInpainter_get_inner_ptr, cv_PtrOfConsistentMosaicInpainter_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::ConsistentMosaicInpainter, cv_PtrOfConsistentMosaicInpainter_new }
	
	impl core::Ptr<crate::videostab::ConsistentMosaicInpainter> {
		#[inline] pub fn as_raw_PtrOfConsistentMosaicInpainter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConsistentMosaicInpainter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::ConsistentMosaicInpainterTraitConst for core::Ptr<crate::videostab::ConsistentMosaicInpainter> {
		#[inline] fn as_raw_ConsistentMosaicInpainter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ConsistentMosaicInpainterTrait for core::Ptr<crate::videostab::ConsistentMosaicInpainter> {
		#[inline] fn as_raw_mut_ConsistentMosaicInpainter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::InpainterBaseTraitConst for core::Ptr<crate::videostab::ConsistentMosaicInpainter> {
		#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::InpainterBaseTrait for core::Ptr<crate::videostab::ConsistentMosaicInpainter> {
		#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfConsistentMosaicInpainter, core::Ptr<crate::videostab::InpainterBase>, cv_PtrOfConsistentMosaicInpainter_to_PtrOfInpainterBase }
	
	pub type PtrOfDeblurerBase = core::Ptr<crate::videostab::DeblurerBase>;
	
	ptr_extern! { crate::videostab::DeblurerBase,
		cv_PtrOfDeblurerBase_delete, cv_PtrOfDeblurerBase_get_inner_ptr, cv_PtrOfDeblurerBase_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::videostab::DeblurerBase> {
		#[inline] pub fn as_raw_PtrOfDeblurerBase(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDeblurerBase(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::DeblurerBaseTraitConst for core::Ptr<crate::videostab::DeblurerBase> {
		#[inline] fn as_raw_DeblurerBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::DeblurerBaseTrait for core::Ptr<crate::videostab::DeblurerBase> {
		#[inline] fn as_raw_mut_DeblurerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDensePyrLkOptFlowEstimatorGpu = core::Ptr<crate::videostab::DensePyrLkOptFlowEstimatorGpu>;
	
	ptr_extern! { crate::videostab::DensePyrLkOptFlowEstimatorGpu,
		cv_PtrOfDensePyrLkOptFlowEstimatorGpu_delete, cv_PtrOfDensePyrLkOptFlowEstimatorGpu_get_inner_ptr, cv_PtrOfDensePyrLkOptFlowEstimatorGpu_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::DensePyrLkOptFlowEstimatorGpu, cv_PtrOfDensePyrLkOptFlowEstimatorGpu_new }
	
	impl core::Ptr<crate::videostab::DensePyrLkOptFlowEstimatorGpu> {
		#[inline] pub fn as_raw_PtrOfDensePyrLkOptFlowEstimatorGpu(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDensePyrLkOptFlowEstimatorGpu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::DensePyrLkOptFlowEstimatorGpuTraitConst for core::Ptr<crate::videostab::DensePyrLkOptFlowEstimatorGpu> {
		#[inline] fn as_raw_DensePyrLkOptFlowEstimatorGpu(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::DensePyrLkOptFlowEstimatorGpuTrait for core::Ptr<crate::videostab::DensePyrLkOptFlowEstimatorGpu> {
		#[inline] fn as_raw_mut_DensePyrLkOptFlowEstimatorGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IDenseOptFlowEstimatorTraitConst for core::Ptr<crate::videostab::DensePyrLkOptFlowEstimatorGpu> {
		#[inline] fn as_raw_IDenseOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IDenseOptFlowEstimatorTrait for core::Ptr<crate::videostab::DensePyrLkOptFlowEstimatorGpu> {
		#[inline] fn as_raw_mut_IDenseOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDensePyrLkOptFlowEstimatorGpu, core::Ptr<crate::videostab::IDenseOptFlowEstimator>, cv_PtrOfDensePyrLkOptFlowEstimatorGpu_to_PtrOfIDenseOptFlowEstimator }
	
	impl crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst for core::Ptr<crate::videostab::DensePyrLkOptFlowEstimatorGpu> {
		#[inline] fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::PyrLkOptFlowEstimatorBaseTrait for core::Ptr<crate::videostab::DensePyrLkOptFlowEstimatorGpu> {
		#[inline] fn as_raw_mut_PyrLkOptFlowEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFromFileMotionReader = core::Ptr<crate::videostab::FromFileMotionReader>;
	
	ptr_extern! { crate::videostab::FromFileMotionReader,
		cv_PtrOfFromFileMotionReader_delete, cv_PtrOfFromFileMotionReader_get_inner_ptr, cv_PtrOfFromFileMotionReader_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::FromFileMotionReader, cv_PtrOfFromFileMotionReader_new }
	
	impl core::Ptr<crate::videostab::FromFileMotionReader> {
		#[inline] pub fn as_raw_PtrOfFromFileMotionReader(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFromFileMotionReader(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::FromFileMotionReaderTraitConst for core::Ptr<crate::videostab::FromFileMotionReader> {
		#[inline] fn as_raw_FromFileMotionReader(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::FromFileMotionReaderTrait for core::Ptr<crate::videostab::FromFileMotionReader> {
		#[inline] fn as_raw_mut_FromFileMotionReader(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::ImageMotionEstimatorBaseTraitConst for core::Ptr<crate::videostab::FromFileMotionReader> {
		#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ImageMotionEstimatorBaseTrait for core::Ptr<crate::videostab::FromFileMotionReader> {
		#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfFromFileMotionReader, core::Ptr<crate::videostab::ImageMotionEstimatorBase>, cv_PtrOfFromFileMotionReader_to_PtrOfImageMotionEstimatorBase }
	
	pub type PtrOfGaussianMotionFilter = core::Ptr<crate::videostab::GaussianMotionFilter>;
	
	ptr_extern! { crate::videostab::GaussianMotionFilter,
		cv_PtrOfGaussianMotionFilter_delete, cv_PtrOfGaussianMotionFilter_get_inner_ptr, cv_PtrOfGaussianMotionFilter_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::GaussianMotionFilter, cv_PtrOfGaussianMotionFilter_new }
	
	impl core::Ptr<crate::videostab::GaussianMotionFilter> {
		#[inline] pub fn as_raw_PtrOfGaussianMotionFilter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGaussianMotionFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::GaussianMotionFilterTraitConst for core::Ptr<crate::videostab::GaussianMotionFilter> {
		#[inline] fn as_raw_GaussianMotionFilter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::GaussianMotionFilterTrait for core::Ptr<crate::videostab::GaussianMotionFilter> {
		#[inline] fn as_raw_mut_GaussianMotionFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IMotionStabilizerTraitConst for core::Ptr<crate::videostab::GaussianMotionFilter> {
		#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IMotionStabilizerTrait for core::Ptr<crate::videostab::GaussianMotionFilter> {
		#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfGaussianMotionFilter, core::Ptr<crate::videostab::IMotionStabilizer>, cv_PtrOfGaussianMotionFilter_to_PtrOfIMotionStabilizer }
	
	impl crate::videostab::MotionFilterBaseTraitConst for core::Ptr<crate::videostab::GaussianMotionFilter> {
		#[inline] fn as_raw_MotionFilterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MotionFilterBaseTrait for core::Ptr<crate::videostab::GaussianMotionFilter> {
		#[inline] fn as_raw_mut_MotionFilterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfGaussianMotionFilter, core::Ptr<crate::videostab::MotionFilterBase>, cv_PtrOfGaussianMotionFilter_to_PtrOfMotionFilterBase }
	
	pub type PtrOfIDenseOptFlowEstimator = core::Ptr<crate::videostab::IDenseOptFlowEstimator>;
	
	ptr_extern! { crate::videostab::IDenseOptFlowEstimator,
		cv_PtrOfIDenseOptFlowEstimator_delete, cv_PtrOfIDenseOptFlowEstimator_get_inner_ptr, cv_PtrOfIDenseOptFlowEstimator_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::videostab::IDenseOptFlowEstimator> {
		#[inline] pub fn as_raw_PtrOfIDenseOptFlowEstimator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfIDenseOptFlowEstimator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::IDenseOptFlowEstimatorTraitConst for core::Ptr<crate::videostab::IDenseOptFlowEstimator> {
		#[inline] fn as_raw_IDenseOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IDenseOptFlowEstimatorTrait for core::Ptr<crate::videostab::IDenseOptFlowEstimator> {
		#[inline] fn as_raw_mut_IDenseOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfIFrameSource = core::Ptr<crate::videostab::IFrameSource>;
	
	ptr_extern! { crate::videostab::IFrameSource,
		cv_PtrOfIFrameSource_delete, cv_PtrOfIFrameSource_get_inner_ptr, cv_PtrOfIFrameSource_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::videostab::IFrameSource> {
		#[inline] pub fn as_raw_PtrOfIFrameSource(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfIFrameSource(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::IFrameSourceTraitConst for core::Ptr<crate::videostab::IFrameSource> {
		#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IFrameSourceTrait for core::Ptr<crate::videostab::IFrameSource> {
		#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfILog = core::Ptr<crate::videostab::ILog>;
	
	ptr_extern! { crate::videostab::ILog,
		cv_PtrOfILog_delete, cv_PtrOfILog_get_inner_ptr, cv_PtrOfILog_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::videostab::ILog> {
		#[inline] pub fn as_raw_PtrOfILog(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfILog(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::ILogTraitConst for core::Ptr<crate::videostab::ILog> {
		#[inline] fn as_raw_ILog(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ILogTrait for core::Ptr<crate::videostab::ILog> {
		#[inline] fn as_raw_mut_ILog(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfIMotionStabilizer = core::Ptr<crate::videostab::IMotionStabilizer>;
	
	ptr_extern! { crate::videostab::IMotionStabilizer,
		cv_PtrOfIMotionStabilizer_delete, cv_PtrOfIMotionStabilizer_get_inner_ptr, cv_PtrOfIMotionStabilizer_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::videostab::IMotionStabilizer> {
		#[inline] pub fn as_raw_PtrOfIMotionStabilizer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfIMotionStabilizer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::IMotionStabilizerTraitConst for core::Ptr<crate::videostab::IMotionStabilizer> {
		#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IMotionStabilizerTrait for core::Ptr<crate::videostab::IMotionStabilizer> {
		#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfIOutlierRejector = core::Ptr<crate::videostab::IOutlierRejector>;
	
	ptr_extern! { crate::videostab::IOutlierRejector,
		cv_PtrOfIOutlierRejector_delete, cv_PtrOfIOutlierRejector_get_inner_ptr, cv_PtrOfIOutlierRejector_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::videostab::IOutlierRejector> {
		#[inline] pub fn as_raw_PtrOfIOutlierRejector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfIOutlierRejector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::IOutlierRejectorTraitConst for core::Ptr<crate::videostab::IOutlierRejector> {
		#[inline] fn as_raw_IOutlierRejector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IOutlierRejectorTrait for core::Ptr<crate::videostab::IOutlierRejector> {
		#[inline] fn as_raw_mut_IOutlierRejector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfISparseOptFlowEstimator = core::Ptr<crate::videostab::ISparseOptFlowEstimator>;
	
	ptr_extern! { crate::videostab::ISparseOptFlowEstimator,
		cv_PtrOfISparseOptFlowEstimator_delete, cv_PtrOfISparseOptFlowEstimator_get_inner_ptr, cv_PtrOfISparseOptFlowEstimator_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::videostab::ISparseOptFlowEstimator> {
		#[inline] pub fn as_raw_PtrOfISparseOptFlowEstimator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfISparseOptFlowEstimator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::ISparseOptFlowEstimatorTraitConst for core::Ptr<crate::videostab::ISparseOptFlowEstimator> {
		#[inline] fn as_raw_ISparseOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ISparseOptFlowEstimatorTrait for core::Ptr<crate::videostab::ISparseOptFlowEstimator> {
		#[inline] fn as_raw_mut_ISparseOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfImageMotionEstimatorBase = core::Ptr<crate::videostab::ImageMotionEstimatorBase>;
	
	ptr_extern! { crate::videostab::ImageMotionEstimatorBase,
		cv_PtrOfImageMotionEstimatorBase_delete, cv_PtrOfImageMotionEstimatorBase_get_inner_ptr, cv_PtrOfImageMotionEstimatorBase_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::videostab::ImageMotionEstimatorBase> {
		#[inline] pub fn as_raw_PtrOfImageMotionEstimatorBase(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfImageMotionEstimatorBase(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::ImageMotionEstimatorBaseTraitConst for core::Ptr<crate::videostab::ImageMotionEstimatorBase> {
		#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ImageMotionEstimatorBaseTrait for core::Ptr<crate::videostab::ImageMotionEstimatorBase> {
		#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfInpainterBase = core::Ptr<crate::videostab::InpainterBase>;
	
	ptr_extern! { crate::videostab::InpainterBase,
		cv_PtrOfInpainterBase_delete, cv_PtrOfInpainterBase_get_inner_ptr, cv_PtrOfInpainterBase_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::videostab::InpainterBase> {
		#[inline] pub fn as_raw_PtrOfInpainterBase(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfInpainterBase(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::InpainterBaseTraitConst for core::Ptr<crate::videostab::InpainterBase> {
		#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::InpainterBaseTrait for core::Ptr<crate::videostab::InpainterBase> {
		#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfInpaintingPipeline = core::Ptr<crate::videostab::InpaintingPipeline>;
	
	ptr_extern! { crate::videostab::InpaintingPipeline,
		cv_PtrOfInpaintingPipeline_delete, cv_PtrOfInpaintingPipeline_get_inner_ptr, cv_PtrOfInpaintingPipeline_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::InpaintingPipeline, cv_PtrOfInpaintingPipeline_new }
	
	impl core::Ptr<crate::videostab::InpaintingPipeline> {
		#[inline] pub fn as_raw_PtrOfInpaintingPipeline(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfInpaintingPipeline(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::InpaintingPipelineTraitConst for core::Ptr<crate::videostab::InpaintingPipeline> {
		#[inline] fn as_raw_InpaintingPipeline(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::InpaintingPipelineTrait for core::Ptr<crate::videostab::InpaintingPipeline> {
		#[inline] fn as_raw_mut_InpaintingPipeline(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::InpainterBaseTraitConst for core::Ptr<crate::videostab::InpaintingPipeline> {
		#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::InpainterBaseTrait for core::Ptr<crate::videostab::InpaintingPipeline> {
		#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfInpaintingPipeline, core::Ptr<crate::videostab::InpainterBase>, cv_PtrOfInpaintingPipeline_to_PtrOfInpainterBase }
	
	pub type PtrOfKeypointBasedMotionEstimator = core::Ptr<crate::videostab::KeypointBasedMotionEstimator>;
	
	ptr_extern! { crate::videostab::KeypointBasedMotionEstimator,
		cv_PtrOfKeypointBasedMotionEstimator_delete, cv_PtrOfKeypointBasedMotionEstimator_get_inner_ptr, cv_PtrOfKeypointBasedMotionEstimator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::KeypointBasedMotionEstimator, cv_PtrOfKeypointBasedMotionEstimator_new }
	
	impl core::Ptr<crate::videostab::KeypointBasedMotionEstimator> {
		#[inline] pub fn as_raw_PtrOfKeypointBasedMotionEstimator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKeypointBasedMotionEstimator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::KeypointBasedMotionEstimatorTraitConst for core::Ptr<crate::videostab::KeypointBasedMotionEstimator> {
		#[inline] fn as_raw_KeypointBasedMotionEstimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::KeypointBasedMotionEstimatorTrait for core::Ptr<crate::videostab::KeypointBasedMotionEstimator> {
		#[inline] fn as_raw_mut_KeypointBasedMotionEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::ImageMotionEstimatorBaseTraitConst for core::Ptr<crate::videostab::KeypointBasedMotionEstimator> {
		#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ImageMotionEstimatorBaseTrait for core::Ptr<crate::videostab::KeypointBasedMotionEstimator> {
		#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfKeypointBasedMotionEstimator, core::Ptr<crate::videostab::ImageMotionEstimatorBase>, cv_PtrOfKeypointBasedMotionEstimator_to_PtrOfImageMotionEstimatorBase }
	
	pub type PtrOfKeypointBasedMotionEstimatorGpu = core::Ptr<crate::videostab::KeypointBasedMotionEstimatorGpu>;
	
	ptr_extern! { crate::videostab::KeypointBasedMotionEstimatorGpu,
		cv_PtrOfKeypointBasedMotionEstimatorGpu_delete, cv_PtrOfKeypointBasedMotionEstimatorGpu_get_inner_ptr, cv_PtrOfKeypointBasedMotionEstimatorGpu_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::KeypointBasedMotionEstimatorGpu, cv_PtrOfKeypointBasedMotionEstimatorGpu_new }
	
	impl core::Ptr<crate::videostab::KeypointBasedMotionEstimatorGpu> {
		#[inline] pub fn as_raw_PtrOfKeypointBasedMotionEstimatorGpu(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfKeypointBasedMotionEstimatorGpu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::KeypointBasedMotionEstimatorGpuTraitConst for core::Ptr<crate::videostab::KeypointBasedMotionEstimatorGpu> {
		#[inline] fn as_raw_KeypointBasedMotionEstimatorGpu(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::KeypointBasedMotionEstimatorGpuTrait for core::Ptr<crate::videostab::KeypointBasedMotionEstimatorGpu> {
		#[inline] fn as_raw_mut_KeypointBasedMotionEstimatorGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::ImageMotionEstimatorBaseTraitConst for core::Ptr<crate::videostab::KeypointBasedMotionEstimatorGpu> {
		#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ImageMotionEstimatorBaseTrait for core::Ptr<crate::videostab::KeypointBasedMotionEstimatorGpu> {
		#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfKeypointBasedMotionEstimatorGpu, core::Ptr<crate::videostab::ImageMotionEstimatorBase>, cv_PtrOfKeypointBasedMotionEstimatorGpu_to_PtrOfImageMotionEstimatorBase }
	
	pub type PtrOfLogToStdout = core::Ptr<crate::videostab::LogToStdout>;
	
	ptr_extern! { crate::videostab::LogToStdout,
		cv_PtrOfLogToStdout_delete, cv_PtrOfLogToStdout_get_inner_ptr, cv_PtrOfLogToStdout_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::LogToStdout, cv_PtrOfLogToStdout_new }
	
	impl core::Ptr<crate::videostab::LogToStdout> {
		#[inline] pub fn as_raw_PtrOfLogToStdout(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLogToStdout(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::LogToStdoutTraitConst for core::Ptr<crate::videostab::LogToStdout> {
		#[inline] fn as_raw_LogToStdout(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::LogToStdoutTrait for core::Ptr<crate::videostab::LogToStdout> {
		#[inline] fn as_raw_mut_LogToStdout(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::ILogTraitConst for core::Ptr<crate::videostab::LogToStdout> {
		#[inline] fn as_raw_ILog(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ILogTrait for core::Ptr<crate::videostab::LogToStdout> {
		#[inline] fn as_raw_mut_ILog(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfLogToStdout, core::Ptr<crate::videostab::ILog>, cv_PtrOfLogToStdout_to_PtrOfILog }
	
	pub type PtrOfLpMotionStabilizer = core::Ptr<crate::videostab::LpMotionStabilizer>;
	
	ptr_extern! { crate::videostab::LpMotionStabilizer,
		cv_PtrOfLpMotionStabilizer_delete, cv_PtrOfLpMotionStabilizer_get_inner_ptr, cv_PtrOfLpMotionStabilizer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::LpMotionStabilizer, cv_PtrOfLpMotionStabilizer_new }
	
	impl core::Ptr<crate::videostab::LpMotionStabilizer> {
		#[inline] pub fn as_raw_PtrOfLpMotionStabilizer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLpMotionStabilizer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::LpMotionStabilizerTraitConst for core::Ptr<crate::videostab::LpMotionStabilizer> {
		#[inline] fn as_raw_LpMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::LpMotionStabilizerTrait for core::Ptr<crate::videostab::LpMotionStabilizer> {
		#[inline] fn as_raw_mut_LpMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IMotionStabilizerTraitConst for core::Ptr<crate::videostab::LpMotionStabilizer> {
		#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IMotionStabilizerTrait for core::Ptr<crate::videostab::LpMotionStabilizer> {
		#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfLpMotionStabilizer, core::Ptr<crate::videostab::IMotionStabilizer>, cv_PtrOfLpMotionStabilizer_to_PtrOfIMotionStabilizer }
	
	pub type PtrOfMaskFrameSource = core::Ptr<crate::videostab::MaskFrameSource>;
	
	ptr_extern! { crate::videostab::MaskFrameSource,
		cv_PtrOfMaskFrameSource_delete, cv_PtrOfMaskFrameSource_get_inner_ptr, cv_PtrOfMaskFrameSource_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::MaskFrameSource, cv_PtrOfMaskFrameSource_new }
	
	impl core::Ptr<crate::videostab::MaskFrameSource> {
		#[inline] pub fn as_raw_PtrOfMaskFrameSource(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMaskFrameSource(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::MaskFrameSourceTraitConst for core::Ptr<crate::videostab::MaskFrameSource> {
		#[inline] fn as_raw_MaskFrameSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MaskFrameSourceTrait for core::Ptr<crate::videostab::MaskFrameSource> {
		#[inline] fn as_raw_mut_MaskFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IFrameSourceTraitConst for core::Ptr<crate::videostab::MaskFrameSource> {
		#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IFrameSourceTrait for core::Ptr<crate::videostab::MaskFrameSource> {
		#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMaskFrameSource, core::Ptr<crate::videostab::IFrameSource>, cv_PtrOfMaskFrameSource_to_PtrOfIFrameSource }
	
	pub type PtrOfMoreAccurateMotionWobbleSuppressor = core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressor>;
	
	ptr_extern! { crate::videostab::MoreAccurateMotionWobbleSuppressor,
		cv_PtrOfMoreAccurateMotionWobbleSuppressor_delete, cv_PtrOfMoreAccurateMotionWobbleSuppressor_get_inner_ptr, cv_PtrOfMoreAccurateMotionWobbleSuppressor_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::MoreAccurateMotionWobbleSuppressor, cv_PtrOfMoreAccurateMotionWobbleSuppressor_new }
	
	impl core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressor> {
		#[inline] pub fn as_raw_PtrOfMoreAccurateMotionWobbleSuppressor(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMoreAccurateMotionWobbleSuppressor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::MoreAccurateMotionWobbleSuppressorTraitConst for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressor> {
		#[inline] fn as_raw_MoreAccurateMotionWobbleSuppressor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MoreAccurateMotionWobbleSuppressorTrait for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressor> {
		#[inline] fn as_raw_mut_MoreAccurateMotionWobbleSuppressor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTraitConst for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressor> {
		#[inline] fn as_raw_MoreAccurateMotionWobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTrait for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressor> {
		#[inline] fn as_raw_mut_MoreAccurateMotionWobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMoreAccurateMotionWobbleSuppressor, core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorBase>, cv_PtrOfMoreAccurateMotionWobbleSuppressor_to_PtrOfMoreAccurateMotionWobbleSuppressorBase }
	
	impl crate::videostab::WobbleSuppressorBaseTraitConst for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressor> {
		#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::WobbleSuppressorBaseTrait for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressor> {
		#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMoreAccurateMotionWobbleSuppressor, core::Ptr<crate::videostab::WobbleSuppressorBase>, cv_PtrOfMoreAccurateMotionWobbleSuppressor_to_PtrOfWobbleSuppressorBase }
	
	pub type PtrOfMoreAccurateMotionWobbleSuppressorBase = core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorBase>;
	
	ptr_extern! { crate::videostab::MoreAccurateMotionWobbleSuppressorBase,
		cv_PtrOfMoreAccurateMotionWobbleSuppressorBase_delete, cv_PtrOfMoreAccurateMotionWobbleSuppressorBase_get_inner_ptr, cv_PtrOfMoreAccurateMotionWobbleSuppressorBase_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorBase> {
		#[inline] pub fn as_raw_PtrOfMoreAccurateMotionWobbleSuppressorBase(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMoreAccurateMotionWobbleSuppressorBase(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTraitConst for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorBase> {
		#[inline] fn as_raw_MoreAccurateMotionWobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTrait for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorBase> {
		#[inline] fn as_raw_mut_MoreAccurateMotionWobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::WobbleSuppressorBaseTraitConst for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorBase> {
		#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::WobbleSuppressorBaseTrait for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorBase> {
		#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMoreAccurateMotionWobbleSuppressorBase, core::Ptr<crate::videostab::WobbleSuppressorBase>, cv_PtrOfMoreAccurateMotionWobbleSuppressorBase_to_PtrOfWobbleSuppressorBase }
	
	pub type PtrOfMoreAccurateMotionWobbleSuppressorGpu = core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorGpu>;
	
	ptr_extern! { crate::videostab::MoreAccurateMotionWobbleSuppressorGpu,
		cv_PtrOfMoreAccurateMotionWobbleSuppressorGpu_delete, cv_PtrOfMoreAccurateMotionWobbleSuppressorGpu_get_inner_ptr, cv_PtrOfMoreAccurateMotionWobbleSuppressorGpu_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::MoreAccurateMotionWobbleSuppressorGpu, cv_PtrOfMoreAccurateMotionWobbleSuppressorGpu_new }
	
	impl core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorGpu> {
		#[inline] pub fn as_raw_PtrOfMoreAccurateMotionWobbleSuppressorGpu(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMoreAccurateMotionWobbleSuppressorGpu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::MoreAccurateMotionWobbleSuppressorGpuTraitConst for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorGpu> {
		#[inline] fn as_raw_MoreAccurateMotionWobbleSuppressorGpu(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MoreAccurateMotionWobbleSuppressorGpuTrait for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorGpu> {
		#[inline] fn as_raw_mut_MoreAccurateMotionWobbleSuppressorGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTraitConst for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorGpu> {
		#[inline] fn as_raw_MoreAccurateMotionWobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MoreAccurateMotionWobbleSuppressorBaseTrait for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorGpu> {
		#[inline] fn as_raw_mut_MoreAccurateMotionWobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMoreAccurateMotionWobbleSuppressorGpu, core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorBase>, cv_PtrOfMoreAccurateMotionWobbleSuppressorGpu_to_PtrOfMoreAccurateMotionWobbleSuppressorBase }
	
	impl crate::videostab::WobbleSuppressorBaseTraitConst for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorGpu> {
		#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::WobbleSuppressorBaseTrait for core::Ptr<crate::videostab::MoreAccurateMotionWobbleSuppressorGpu> {
		#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMoreAccurateMotionWobbleSuppressorGpu, core::Ptr<crate::videostab::WobbleSuppressorBase>, cv_PtrOfMoreAccurateMotionWobbleSuppressorGpu_to_PtrOfWobbleSuppressorBase }
	
	pub type PtrOfMotionEstimatorBase = core::Ptr<crate::videostab::MotionEstimatorBase>;
	
	ptr_extern! { crate::videostab::MotionEstimatorBase,
		cv_PtrOfMotionEstimatorBase_delete, cv_PtrOfMotionEstimatorBase_get_inner_ptr, cv_PtrOfMotionEstimatorBase_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::videostab::MotionEstimatorBase> {
		#[inline] pub fn as_raw_PtrOfMotionEstimatorBase(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMotionEstimatorBase(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::MotionEstimatorBaseTraitConst for core::Ptr<crate::videostab::MotionEstimatorBase> {
		#[inline] fn as_raw_MotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MotionEstimatorBaseTrait for core::Ptr<crate::videostab::MotionEstimatorBase> {
		#[inline] fn as_raw_mut_MotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfMotionEstimatorL1 = core::Ptr<crate::videostab::MotionEstimatorL1>;
	
	ptr_extern! { crate::videostab::MotionEstimatorL1,
		cv_PtrOfMotionEstimatorL1_delete, cv_PtrOfMotionEstimatorL1_get_inner_ptr, cv_PtrOfMotionEstimatorL1_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::MotionEstimatorL1, cv_PtrOfMotionEstimatorL1_new }
	
	impl core::Ptr<crate::videostab::MotionEstimatorL1> {
		#[inline] pub fn as_raw_PtrOfMotionEstimatorL1(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMotionEstimatorL1(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::MotionEstimatorL1TraitConst for core::Ptr<crate::videostab::MotionEstimatorL1> {
		#[inline] fn as_raw_MotionEstimatorL1(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MotionEstimatorL1Trait for core::Ptr<crate::videostab::MotionEstimatorL1> {
		#[inline] fn as_raw_mut_MotionEstimatorL1(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::MotionEstimatorBaseTraitConst for core::Ptr<crate::videostab::MotionEstimatorL1> {
		#[inline] fn as_raw_MotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MotionEstimatorBaseTrait for core::Ptr<crate::videostab::MotionEstimatorL1> {
		#[inline] fn as_raw_mut_MotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMotionEstimatorL1, core::Ptr<crate::videostab::MotionEstimatorBase>, cv_PtrOfMotionEstimatorL1_to_PtrOfMotionEstimatorBase }
	
	pub type PtrOfMotionEstimatorRansacL2 = core::Ptr<crate::videostab::MotionEstimatorRansacL2>;
	
	ptr_extern! { crate::videostab::MotionEstimatorRansacL2,
		cv_PtrOfMotionEstimatorRansacL2_delete, cv_PtrOfMotionEstimatorRansacL2_get_inner_ptr, cv_PtrOfMotionEstimatorRansacL2_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::MotionEstimatorRansacL2, cv_PtrOfMotionEstimatorRansacL2_new }
	
	impl core::Ptr<crate::videostab::MotionEstimatorRansacL2> {
		#[inline] pub fn as_raw_PtrOfMotionEstimatorRansacL2(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMotionEstimatorRansacL2(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::MotionEstimatorRansacL2TraitConst for core::Ptr<crate::videostab::MotionEstimatorRansacL2> {
		#[inline] fn as_raw_MotionEstimatorRansacL2(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MotionEstimatorRansacL2Trait for core::Ptr<crate::videostab::MotionEstimatorRansacL2> {
		#[inline] fn as_raw_mut_MotionEstimatorRansacL2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::MotionEstimatorBaseTraitConst for core::Ptr<crate::videostab::MotionEstimatorRansacL2> {
		#[inline] fn as_raw_MotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MotionEstimatorBaseTrait for core::Ptr<crate::videostab::MotionEstimatorRansacL2> {
		#[inline] fn as_raw_mut_MotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMotionEstimatorRansacL2, core::Ptr<crate::videostab::MotionEstimatorBase>, cv_PtrOfMotionEstimatorRansacL2_to_PtrOfMotionEstimatorBase }
	
	pub type PtrOfMotionFilterBase = core::Ptr<crate::videostab::MotionFilterBase>;
	
	ptr_extern! { crate::videostab::MotionFilterBase,
		cv_PtrOfMotionFilterBase_delete, cv_PtrOfMotionFilterBase_get_inner_ptr, cv_PtrOfMotionFilterBase_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::videostab::MotionFilterBase> {
		#[inline] pub fn as_raw_PtrOfMotionFilterBase(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMotionFilterBase(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::MotionFilterBaseTraitConst for core::Ptr<crate::videostab::MotionFilterBase> {
		#[inline] fn as_raw_MotionFilterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MotionFilterBaseTrait for core::Ptr<crate::videostab::MotionFilterBase> {
		#[inline] fn as_raw_mut_MotionFilterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IMotionStabilizerTraitConst for core::Ptr<crate::videostab::MotionFilterBase> {
		#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IMotionStabilizerTrait for core::Ptr<crate::videostab::MotionFilterBase> {
		#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMotionFilterBase, core::Ptr<crate::videostab::IMotionStabilizer>, cv_PtrOfMotionFilterBase_to_PtrOfIMotionStabilizer }
	
	pub type PtrOfMotionInpainter = core::Ptr<crate::videostab::MotionInpainter>;
	
	ptr_extern! { crate::videostab::MotionInpainter,
		cv_PtrOfMotionInpainter_delete, cv_PtrOfMotionInpainter_get_inner_ptr, cv_PtrOfMotionInpainter_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::MotionInpainter, cv_PtrOfMotionInpainter_new }
	
	impl core::Ptr<crate::videostab::MotionInpainter> {
		#[inline] pub fn as_raw_PtrOfMotionInpainter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMotionInpainter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::MotionInpainterTraitConst for core::Ptr<crate::videostab::MotionInpainter> {
		#[inline] fn as_raw_MotionInpainter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MotionInpainterTrait for core::Ptr<crate::videostab::MotionInpainter> {
		#[inline] fn as_raw_mut_MotionInpainter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::InpainterBaseTraitConst for core::Ptr<crate::videostab::MotionInpainter> {
		#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::InpainterBaseTrait for core::Ptr<crate::videostab::MotionInpainter> {
		#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMotionInpainter, core::Ptr<crate::videostab::InpainterBase>, cv_PtrOfMotionInpainter_to_PtrOfInpainterBase }
	
	pub type PtrOfMotionStabilizationPipeline = core::Ptr<crate::videostab::MotionStabilizationPipeline>;
	
	ptr_extern! { crate::videostab::MotionStabilizationPipeline,
		cv_PtrOfMotionStabilizationPipeline_delete, cv_PtrOfMotionStabilizationPipeline_get_inner_ptr, cv_PtrOfMotionStabilizationPipeline_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::MotionStabilizationPipeline, cv_PtrOfMotionStabilizationPipeline_new }
	
	impl core::Ptr<crate::videostab::MotionStabilizationPipeline> {
		#[inline] pub fn as_raw_PtrOfMotionStabilizationPipeline(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMotionStabilizationPipeline(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::MotionStabilizationPipelineTraitConst for core::Ptr<crate::videostab::MotionStabilizationPipeline> {
		#[inline] fn as_raw_MotionStabilizationPipeline(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::MotionStabilizationPipelineTrait for core::Ptr<crate::videostab::MotionStabilizationPipeline> {
		#[inline] fn as_raw_mut_MotionStabilizationPipeline(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IMotionStabilizerTraitConst for core::Ptr<crate::videostab::MotionStabilizationPipeline> {
		#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IMotionStabilizerTrait for core::Ptr<crate::videostab::MotionStabilizationPipeline> {
		#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMotionStabilizationPipeline, core::Ptr<crate::videostab::IMotionStabilizer>, cv_PtrOfMotionStabilizationPipeline_to_PtrOfIMotionStabilizer }
	
	pub type PtrOfNullDeblurer = core::Ptr<crate::videostab::NullDeblurer>;
	
	ptr_extern! { crate::videostab::NullDeblurer,
		cv_PtrOfNullDeblurer_delete, cv_PtrOfNullDeblurer_get_inner_ptr, cv_PtrOfNullDeblurer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::NullDeblurer, cv_PtrOfNullDeblurer_new }
	
	impl core::Ptr<crate::videostab::NullDeblurer> {
		#[inline] pub fn as_raw_PtrOfNullDeblurer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNullDeblurer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::NullDeblurerTraitConst for core::Ptr<crate::videostab::NullDeblurer> {
		#[inline] fn as_raw_NullDeblurer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::NullDeblurerTrait for core::Ptr<crate::videostab::NullDeblurer> {
		#[inline] fn as_raw_mut_NullDeblurer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::DeblurerBaseTraitConst for core::Ptr<crate::videostab::NullDeblurer> {
		#[inline] fn as_raw_DeblurerBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::DeblurerBaseTrait for core::Ptr<crate::videostab::NullDeblurer> {
		#[inline] fn as_raw_mut_DeblurerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfNullDeblurer, core::Ptr<crate::videostab::DeblurerBase>, cv_PtrOfNullDeblurer_to_PtrOfDeblurerBase }
	
	pub type PtrOfNullFrameSource = core::Ptr<crate::videostab::NullFrameSource>;
	
	ptr_extern! { crate::videostab::NullFrameSource,
		cv_PtrOfNullFrameSource_delete, cv_PtrOfNullFrameSource_get_inner_ptr, cv_PtrOfNullFrameSource_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::NullFrameSource, cv_PtrOfNullFrameSource_new }
	
	impl core::Ptr<crate::videostab::NullFrameSource> {
		#[inline] pub fn as_raw_PtrOfNullFrameSource(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNullFrameSource(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::NullFrameSourceTraitConst for core::Ptr<crate::videostab::NullFrameSource> {
		#[inline] fn as_raw_NullFrameSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::NullFrameSourceTrait for core::Ptr<crate::videostab::NullFrameSource> {
		#[inline] fn as_raw_mut_NullFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IFrameSourceTraitConst for core::Ptr<crate::videostab::NullFrameSource> {
		#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IFrameSourceTrait for core::Ptr<crate::videostab::NullFrameSource> {
		#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfNullFrameSource, core::Ptr<crate::videostab::IFrameSource>, cv_PtrOfNullFrameSource_to_PtrOfIFrameSource }
	
	pub type PtrOfNullInpainter = core::Ptr<crate::videostab::NullInpainter>;
	
	ptr_extern! { crate::videostab::NullInpainter,
		cv_PtrOfNullInpainter_delete, cv_PtrOfNullInpainter_get_inner_ptr, cv_PtrOfNullInpainter_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::NullInpainter, cv_PtrOfNullInpainter_new }
	
	impl core::Ptr<crate::videostab::NullInpainter> {
		#[inline] pub fn as_raw_PtrOfNullInpainter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNullInpainter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::NullInpainterTraitConst for core::Ptr<crate::videostab::NullInpainter> {
		#[inline] fn as_raw_NullInpainter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::NullInpainterTrait for core::Ptr<crate::videostab::NullInpainter> {
		#[inline] fn as_raw_mut_NullInpainter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::InpainterBaseTraitConst for core::Ptr<crate::videostab::NullInpainter> {
		#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::InpainterBaseTrait for core::Ptr<crate::videostab::NullInpainter> {
		#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfNullInpainter, core::Ptr<crate::videostab::InpainterBase>, cv_PtrOfNullInpainter_to_PtrOfInpainterBase }
	
	pub type PtrOfNullLog = core::Ptr<crate::videostab::NullLog>;
	
	ptr_extern! { crate::videostab::NullLog,
		cv_PtrOfNullLog_delete, cv_PtrOfNullLog_get_inner_ptr, cv_PtrOfNullLog_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::NullLog, cv_PtrOfNullLog_new }
	
	impl core::Ptr<crate::videostab::NullLog> {
		#[inline] pub fn as_raw_PtrOfNullLog(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNullLog(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::NullLogTraitConst for core::Ptr<crate::videostab::NullLog> {
		#[inline] fn as_raw_NullLog(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::NullLogTrait for core::Ptr<crate::videostab::NullLog> {
		#[inline] fn as_raw_mut_NullLog(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::ILogTraitConst for core::Ptr<crate::videostab::NullLog> {
		#[inline] fn as_raw_ILog(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ILogTrait for core::Ptr<crate::videostab::NullLog> {
		#[inline] fn as_raw_mut_ILog(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfNullLog, core::Ptr<crate::videostab::ILog>, cv_PtrOfNullLog_to_PtrOfILog }
	
	pub type PtrOfNullOutlierRejector = core::Ptr<crate::videostab::NullOutlierRejector>;
	
	ptr_extern! { crate::videostab::NullOutlierRejector,
		cv_PtrOfNullOutlierRejector_delete, cv_PtrOfNullOutlierRejector_get_inner_ptr, cv_PtrOfNullOutlierRejector_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::NullOutlierRejector, cv_PtrOfNullOutlierRejector_new }
	
	impl core::Ptr<crate::videostab::NullOutlierRejector> {
		#[inline] pub fn as_raw_PtrOfNullOutlierRejector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNullOutlierRejector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::NullOutlierRejectorTraitConst for core::Ptr<crate::videostab::NullOutlierRejector> {
		#[inline] fn as_raw_NullOutlierRejector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::NullOutlierRejectorTrait for core::Ptr<crate::videostab::NullOutlierRejector> {
		#[inline] fn as_raw_mut_NullOutlierRejector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IOutlierRejectorTraitConst for core::Ptr<crate::videostab::NullOutlierRejector> {
		#[inline] fn as_raw_IOutlierRejector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IOutlierRejectorTrait for core::Ptr<crate::videostab::NullOutlierRejector> {
		#[inline] fn as_raw_mut_IOutlierRejector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfNullOutlierRejector, core::Ptr<crate::videostab::IOutlierRejector>, cv_PtrOfNullOutlierRejector_to_PtrOfIOutlierRejector }
	
	pub type PtrOfNullWobbleSuppressor = core::Ptr<crate::videostab::NullWobbleSuppressor>;
	
	ptr_extern! { crate::videostab::NullWobbleSuppressor,
		cv_PtrOfNullWobbleSuppressor_delete, cv_PtrOfNullWobbleSuppressor_get_inner_ptr, cv_PtrOfNullWobbleSuppressor_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::NullWobbleSuppressor, cv_PtrOfNullWobbleSuppressor_new }
	
	impl core::Ptr<crate::videostab::NullWobbleSuppressor> {
		#[inline] pub fn as_raw_PtrOfNullWobbleSuppressor(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNullWobbleSuppressor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::NullWobbleSuppressorTraitConst for core::Ptr<crate::videostab::NullWobbleSuppressor> {
		#[inline] fn as_raw_NullWobbleSuppressor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::NullWobbleSuppressorTrait for core::Ptr<crate::videostab::NullWobbleSuppressor> {
		#[inline] fn as_raw_mut_NullWobbleSuppressor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::WobbleSuppressorBaseTraitConst for core::Ptr<crate::videostab::NullWobbleSuppressor> {
		#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::WobbleSuppressorBaseTrait for core::Ptr<crate::videostab::NullWobbleSuppressor> {
		#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfNullWobbleSuppressor, core::Ptr<crate::videostab::WobbleSuppressorBase>, cv_PtrOfNullWobbleSuppressor_to_PtrOfWobbleSuppressorBase }
	
	pub type PtrOfOnePassStabilizer = core::Ptr<crate::videostab::OnePassStabilizer>;
	
	ptr_extern! { crate::videostab::OnePassStabilizer,
		cv_PtrOfOnePassStabilizer_delete, cv_PtrOfOnePassStabilizer_get_inner_ptr, cv_PtrOfOnePassStabilizer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::OnePassStabilizer, cv_PtrOfOnePassStabilizer_new }
	
	impl core::Ptr<crate::videostab::OnePassStabilizer> {
		#[inline] pub fn as_raw_PtrOfOnePassStabilizer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOnePassStabilizer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::OnePassStabilizerTraitConst for core::Ptr<crate::videostab::OnePassStabilizer> {
		#[inline] fn as_raw_OnePassStabilizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::OnePassStabilizerTrait for core::Ptr<crate::videostab::OnePassStabilizer> {
		#[inline] fn as_raw_mut_OnePassStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IFrameSourceTraitConst for core::Ptr<crate::videostab::OnePassStabilizer> {
		#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IFrameSourceTrait for core::Ptr<crate::videostab::OnePassStabilizer> {
		#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfOnePassStabilizer, core::Ptr<crate::videostab::IFrameSource>, cv_PtrOfOnePassStabilizer_to_PtrOfIFrameSource }
	
	impl crate::videostab::StabilizerBaseTraitConst for core::Ptr<crate::videostab::OnePassStabilizer> {
		#[inline] fn as_raw_StabilizerBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::StabilizerBaseTrait for core::Ptr<crate::videostab::OnePassStabilizer> {
		#[inline] fn as_raw_mut_StabilizerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSparsePyrLkOptFlowEstimator = core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimator>;
	
	ptr_extern! { crate::videostab::SparsePyrLkOptFlowEstimator,
		cv_PtrOfSparsePyrLkOptFlowEstimator_delete, cv_PtrOfSparsePyrLkOptFlowEstimator_get_inner_ptr, cv_PtrOfSparsePyrLkOptFlowEstimator_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::SparsePyrLkOptFlowEstimator, cv_PtrOfSparsePyrLkOptFlowEstimator_new }
	
	impl core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimator> {
		#[inline] pub fn as_raw_PtrOfSparsePyrLkOptFlowEstimator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSparsePyrLkOptFlowEstimator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::SparsePyrLkOptFlowEstimatorTraitConst for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimator> {
		#[inline] fn as_raw_SparsePyrLkOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::SparsePyrLkOptFlowEstimatorTrait for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimator> {
		#[inline] fn as_raw_mut_SparsePyrLkOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::ISparseOptFlowEstimatorTraitConst for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimator> {
		#[inline] fn as_raw_ISparseOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ISparseOptFlowEstimatorTrait for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimator> {
		#[inline] fn as_raw_mut_ISparseOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSparsePyrLkOptFlowEstimator, core::Ptr<crate::videostab::ISparseOptFlowEstimator>, cv_PtrOfSparsePyrLkOptFlowEstimator_to_PtrOfISparseOptFlowEstimator }
	
	impl crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimator> {
		#[inline] fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::PyrLkOptFlowEstimatorBaseTrait for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimator> {
		#[inline] fn as_raw_mut_PyrLkOptFlowEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSparsePyrLkOptFlowEstimatorGpu = core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimatorGpu>;
	
	ptr_extern! { crate::videostab::SparsePyrLkOptFlowEstimatorGpu,
		cv_PtrOfSparsePyrLkOptFlowEstimatorGpu_delete, cv_PtrOfSparsePyrLkOptFlowEstimatorGpu_get_inner_ptr, cv_PtrOfSparsePyrLkOptFlowEstimatorGpu_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::SparsePyrLkOptFlowEstimatorGpu, cv_PtrOfSparsePyrLkOptFlowEstimatorGpu_new }
	
	impl core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimatorGpu> {
		#[inline] pub fn as_raw_PtrOfSparsePyrLkOptFlowEstimatorGpu(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSparsePyrLkOptFlowEstimatorGpu(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::SparsePyrLkOptFlowEstimatorGpuTraitConst for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimatorGpu> {
		#[inline] fn as_raw_SparsePyrLkOptFlowEstimatorGpu(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::SparsePyrLkOptFlowEstimatorGpuTrait for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimatorGpu> {
		#[inline] fn as_raw_mut_SparsePyrLkOptFlowEstimatorGpu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::ISparseOptFlowEstimatorTraitConst for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimatorGpu> {
		#[inline] fn as_raw_ISparseOptFlowEstimator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ISparseOptFlowEstimatorTrait for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimatorGpu> {
		#[inline] fn as_raw_mut_ISparseOptFlowEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSparsePyrLkOptFlowEstimatorGpu, core::Ptr<crate::videostab::ISparseOptFlowEstimator>, cv_PtrOfSparsePyrLkOptFlowEstimatorGpu_to_PtrOfISparseOptFlowEstimator }
	
	impl crate::videostab::PyrLkOptFlowEstimatorBaseTraitConst for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimatorGpu> {
		#[inline] fn as_raw_PyrLkOptFlowEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::PyrLkOptFlowEstimatorBaseTrait for core::Ptr<crate::videostab::SparsePyrLkOptFlowEstimatorGpu> {
		#[inline] fn as_raw_mut_PyrLkOptFlowEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfToFileMotionWriter = core::Ptr<crate::videostab::ToFileMotionWriter>;
	
	ptr_extern! { crate::videostab::ToFileMotionWriter,
		cv_PtrOfToFileMotionWriter_delete, cv_PtrOfToFileMotionWriter_get_inner_ptr, cv_PtrOfToFileMotionWriter_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::ToFileMotionWriter, cv_PtrOfToFileMotionWriter_new }
	
	impl core::Ptr<crate::videostab::ToFileMotionWriter> {
		#[inline] pub fn as_raw_PtrOfToFileMotionWriter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfToFileMotionWriter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::ToFileMotionWriterTraitConst for core::Ptr<crate::videostab::ToFileMotionWriter> {
		#[inline] fn as_raw_ToFileMotionWriter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ToFileMotionWriterTrait for core::Ptr<crate::videostab::ToFileMotionWriter> {
		#[inline] fn as_raw_mut_ToFileMotionWriter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::ImageMotionEstimatorBaseTraitConst for core::Ptr<crate::videostab::ToFileMotionWriter> {
		#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::ImageMotionEstimatorBaseTrait for core::Ptr<crate::videostab::ToFileMotionWriter> {
		#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfToFileMotionWriter, core::Ptr<crate::videostab::ImageMotionEstimatorBase>, cv_PtrOfToFileMotionWriter_to_PtrOfImageMotionEstimatorBase }
	
	pub type PtrOfTranslationBasedLocalOutlierRejector = core::Ptr<crate::videostab::TranslationBasedLocalOutlierRejector>;
	
	ptr_extern! { crate::videostab::TranslationBasedLocalOutlierRejector,
		cv_PtrOfTranslationBasedLocalOutlierRejector_delete, cv_PtrOfTranslationBasedLocalOutlierRejector_get_inner_ptr, cv_PtrOfTranslationBasedLocalOutlierRejector_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::TranslationBasedLocalOutlierRejector, cv_PtrOfTranslationBasedLocalOutlierRejector_new }
	
	impl core::Ptr<crate::videostab::TranslationBasedLocalOutlierRejector> {
		#[inline] pub fn as_raw_PtrOfTranslationBasedLocalOutlierRejector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTranslationBasedLocalOutlierRejector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::TranslationBasedLocalOutlierRejectorTraitConst for core::Ptr<crate::videostab::TranslationBasedLocalOutlierRejector> {
		#[inline] fn as_raw_TranslationBasedLocalOutlierRejector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::TranslationBasedLocalOutlierRejectorTrait for core::Ptr<crate::videostab::TranslationBasedLocalOutlierRejector> {
		#[inline] fn as_raw_mut_TranslationBasedLocalOutlierRejector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IOutlierRejectorTraitConst for core::Ptr<crate::videostab::TranslationBasedLocalOutlierRejector> {
		#[inline] fn as_raw_IOutlierRejector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IOutlierRejectorTrait for core::Ptr<crate::videostab::TranslationBasedLocalOutlierRejector> {
		#[inline] fn as_raw_mut_IOutlierRejector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfTranslationBasedLocalOutlierRejector, core::Ptr<crate::videostab::IOutlierRejector>, cv_PtrOfTranslationBasedLocalOutlierRejector_to_PtrOfIOutlierRejector }
	
	pub type PtrOfTwoPassStabilizer = core::Ptr<crate::videostab::TwoPassStabilizer>;
	
	ptr_extern! { crate::videostab::TwoPassStabilizer,
		cv_PtrOfTwoPassStabilizer_delete, cv_PtrOfTwoPassStabilizer_get_inner_ptr, cv_PtrOfTwoPassStabilizer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::TwoPassStabilizer, cv_PtrOfTwoPassStabilizer_new }
	
	impl core::Ptr<crate::videostab::TwoPassStabilizer> {
		#[inline] pub fn as_raw_PtrOfTwoPassStabilizer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTwoPassStabilizer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::TwoPassStabilizerTraitConst for core::Ptr<crate::videostab::TwoPassStabilizer> {
		#[inline] fn as_raw_TwoPassStabilizer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::TwoPassStabilizerTrait for core::Ptr<crate::videostab::TwoPassStabilizer> {
		#[inline] fn as_raw_mut_TwoPassStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IFrameSourceTraitConst for core::Ptr<crate::videostab::TwoPassStabilizer> {
		#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IFrameSourceTrait for core::Ptr<crate::videostab::TwoPassStabilizer> {
		#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfTwoPassStabilizer, core::Ptr<crate::videostab::IFrameSource>, cv_PtrOfTwoPassStabilizer_to_PtrOfIFrameSource }
	
	impl crate::videostab::StabilizerBaseTraitConst for core::Ptr<crate::videostab::TwoPassStabilizer> {
		#[inline] fn as_raw_StabilizerBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::StabilizerBaseTrait for core::Ptr<crate::videostab::TwoPassStabilizer> {
		#[inline] fn as_raw_mut_StabilizerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfVideoFileSource = core::Ptr<crate::videostab::VideoFileSource>;
	
	ptr_extern! { crate::videostab::VideoFileSource,
		cv_PtrOfVideoFileSource_delete, cv_PtrOfVideoFileSource_get_inner_ptr, cv_PtrOfVideoFileSource_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::VideoFileSource, cv_PtrOfVideoFileSource_new }
	
	impl core::Ptr<crate::videostab::VideoFileSource> {
		#[inline] pub fn as_raw_PtrOfVideoFileSource(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfVideoFileSource(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::VideoFileSourceTraitConst for core::Ptr<crate::videostab::VideoFileSource> {
		#[inline] fn as_raw_VideoFileSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::VideoFileSourceTrait for core::Ptr<crate::videostab::VideoFileSource> {
		#[inline] fn as_raw_mut_VideoFileSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::IFrameSourceTraitConst for core::Ptr<crate::videostab::VideoFileSource> {
		#[inline] fn as_raw_IFrameSource(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::IFrameSourceTrait for core::Ptr<crate::videostab::VideoFileSource> {
		#[inline] fn as_raw_mut_IFrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfVideoFileSource, core::Ptr<crate::videostab::IFrameSource>, cv_PtrOfVideoFileSource_to_PtrOfIFrameSource }
	
	pub type PtrOfWeightingDeblurer = core::Ptr<crate::videostab::WeightingDeblurer>;
	
	ptr_extern! { crate::videostab::WeightingDeblurer,
		cv_PtrOfWeightingDeblurer_delete, cv_PtrOfWeightingDeblurer_get_inner_ptr, cv_PtrOfWeightingDeblurer_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::videostab::WeightingDeblurer, cv_PtrOfWeightingDeblurer_new }
	
	impl core::Ptr<crate::videostab::WeightingDeblurer> {
		#[inline] pub fn as_raw_PtrOfWeightingDeblurer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfWeightingDeblurer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::WeightingDeblurerTraitConst for core::Ptr<crate::videostab::WeightingDeblurer> {
		#[inline] fn as_raw_WeightingDeblurer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::WeightingDeblurerTrait for core::Ptr<crate::videostab::WeightingDeblurer> {
		#[inline] fn as_raw_mut_WeightingDeblurer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::videostab::DeblurerBaseTraitConst for core::Ptr<crate::videostab::WeightingDeblurer> {
		#[inline] fn as_raw_DeblurerBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::DeblurerBaseTrait for core::Ptr<crate::videostab::WeightingDeblurer> {
		#[inline] fn as_raw_mut_DeblurerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfWeightingDeblurer, core::Ptr<crate::videostab::DeblurerBase>, cv_PtrOfWeightingDeblurer_to_PtrOfDeblurerBase }
	
	pub type PtrOfWobbleSuppressorBase = core::Ptr<crate::videostab::WobbleSuppressorBase>;
	
	ptr_extern! { crate::videostab::WobbleSuppressorBase,
		cv_PtrOfWobbleSuppressorBase_delete, cv_PtrOfWobbleSuppressorBase_get_inner_ptr, cv_PtrOfWobbleSuppressorBase_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::videostab::WobbleSuppressorBase> {
		#[inline] pub fn as_raw_PtrOfWobbleSuppressorBase(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfWobbleSuppressorBase(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::videostab::WobbleSuppressorBaseTraitConst for core::Ptr<crate::videostab::WobbleSuppressorBase> {
		#[inline] fn as_raw_WobbleSuppressorBase(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::videostab::WobbleSuppressorBaseTrait for core::Ptr<crate::videostab::WobbleSuppressorBase> {
		#[inline] fn as_raw_mut_WobbleSuppressorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_videostab)]
pub use videostab_types::*;

#[cfg(ocvrs_has_module_xfeatures2d)]
mod xfeatures2d_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAffineFeature2D = core::Ptr<crate::xfeatures2d::AffineFeature2D>;
	
	ptr_extern! { crate::xfeatures2d::AffineFeature2D,
		cv_PtrOfAffineFeature2D_delete, cv_PtrOfAffineFeature2D_get_inner_ptr, cv_PtrOfAffineFeature2D_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::xfeatures2d::AffineFeature2D> {
		#[inline] pub fn as_raw_PtrOfAffineFeature2D(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAffineFeature2D(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::AffineFeature2DTraitConst for core::Ptr<crate::xfeatures2d::AffineFeature2D> {
		#[inline] fn as_raw_AffineFeature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::AffineFeature2DTrait for core::Ptr<crate::xfeatures2d::AffineFeature2D> {
		#[inline] fn as_raw_mut_AffineFeature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::AffineFeature2D> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::AffineFeature2D> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::AffineFeature2D> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::AffineFeature2D> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfAffineFeature2D, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfAffineFeature2D_to_PtrOfFeature2D }
	
	pub type PtrOfBEBLID = core::Ptr<crate::xfeatures2d::BEBLID>;
	
	ptr_extern! { crate::xfeatures2d::BEBLID,
		cv_PtrOfBEBLID_delete, cv_PtrOfBEBLID_get_inner_ptr, cv_PtrOfBEBLID_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::xfeatures2d::BEBLID> {
		#[inline] pub fn as_raw_PtrOfBEBLID(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBEBLID(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::BEBLIDTraitConst for core::Ptr<crate::xfeatures2d::BEBLID> {
		#[inline] fn as_raw_BEBLID(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::BEBLIDTrait for core::Ptr<crate::xfeatures2d::BEBLID> {
		#[inline] fn as_raw_mut_BEBLID(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::BEBLID> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::BEBLID> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::BEBLID> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::BEBLID> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfBEBLID, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfBEBLID_to_PtrOfFeature2D }
	
	pub type PtrOfBoostDesc = core::Ptr<crate::xfeatures2d::BoostDesc>;
	
	ptr_extern! { crate::xfeatures2d::BoostDesc,
		cv_PtrOfBoostDesc_delete, cv_PtrOfBoostDesc_get_inner_ptr, cv_PtrOfBoostDesc_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::xfeatures2d::BoostDesc> {
		#[inline] pub fn as_raw_PtrOfBoostDesc(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBoostDesc(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::BoostDescTraitConst for core::Ptr<crate::xfeatures2d::BoostDesc> {
		#[inline] fn as_raw_BoostDesc(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::BoostDescTrait for core::Ptr<crate::xfeatures2d::BoostDesc> {
		#[inline] fn as_raw_mut_BoostDesc(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::BoostDesc> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::BoostDesc> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::BoostDesc> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::BoostDesc> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfBoostDesc, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfBoostDesc_to_PtrOfFeature2D }
	
	pub type PtrOfBriefDescriptorExtractor = core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor>;
	
	ptr_extern! { crate::xfeatures2d::BriefDescriptorExtractor,
		cv_PtrOfBriefDescriptorExtractor_delete, cv_PtrOfBriefDescriptorExtractor_get_inner_ptr, cv_PtrOfBriefDescriptorExtractor_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor> {
		#[inline] pub fn as_raw_PtrOfBriefDescriptorExtractor(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBriefDescriptorExtractor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::BriefDescriptorExtractorTraitConst for core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor> {
		#[inline] fn as_raw_BriefDescriptorExtractor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::BriefDescriptorExtractorTrait for core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor> {
		#[inline] fn as_raw_mut_BriefDescriptorExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfBriefDescriptorExtractor, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfBriefDescriptorExtractor_to_PtrOfFeature2D }
	
	pub type PtrOfDAISY = core::Ptr<crate::xfeatures2d::DAISY>;
	
	ptr_extern! { crate::xfeatures2d::DAISY,
		cv_PtrOfDAISY_delete, cv_PtrOfDAISY_get_inner_ptr, cv_PtrOfDAISY_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::xfeatures2d::DAISY> {
		#[inline] pub fn as_raw_PtrOfDAISY(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDAISY(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::DAISYTraitConst for core::Ptr<crate::xfeatures2d::DAISY> {
		#[inline] fn as_raw_DAISY(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::DAISYTrait for core::Ptr<crate::xfeatures2d::DAISY> {
		#[inline] fn as_raw_mut_DAISY(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::DAISY> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::DAISY> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::DAISY> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::DAISY> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfDAISY, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfDAISY_to_PtrOfFeature2D }
	
	pub type PtrOfFREAK = core::Ptr<crate::xfeatures2d::FREAK>;
	
	ptr_extern! { crate::xfeatures2d::FREAK,
		cv_PtrOfFREAK_delete, cv_PtrOfFREAK_get_inner_ptr, cv_PtrOfFREAK_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::xfeatures2d::FREAK> {
		#[inline] pub fn as_raw_PtrOfFREAK(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFREAK(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::FREAKTraitConst for core::Ptr<crate::xfeatures2d::FREAK> {
		#[inline] fn as_raw_FREAK(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::FREAKTrait for core::Ptr<crate::xfeatures2d::FREAK> {
		#[inline] fn as_raw_mut_FREAK(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::FREAK> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::FREAK> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::FREAK> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::FREAK> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfFREAK, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfFREAK_to_PtrOfFeature2D }
	
	pub type PtrOfHarrisLaplaceFeatureDetector = core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector>;
	
	ptr_extern! { crate::xfeatures2d::HarrisLaplaceFeatureDetector,
		cv_PtrOfHarrisLaplaceFeatureDetector_delete, cv_PtrOfHarrisLaplaceFeatureDetector_get_inner_ptr, cv_PtrOfHarrisLaplaceFeatureDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector> {
		#[inline] pub fn as_raw_PtrOfHarrisLaplaceFeatureDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHarrisLaplaceFeatureDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::HarrisLaplaceFeatureDetectorTraitConst for core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector> {
		#[inline] fn as_raw_HarrisLaplaceFeatureDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::HarrisLaplaceFeatureDetectorTrait for core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector> {
		#[inline] fn as_raw_mut_HarrisLaplaceFeatureDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfHarrisLaplaceFeatureDetector, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfHarrisLaplaceFeatureDetector_to_PtrOfFeature2D }
	
	pub type PtrOfLATCH = core::Ptr<crate::xfeatures2d::LATCH>;
	
	ptr_extern! { crate::xfeatures2d::LATCH,
		cv_PtrOfLATCH_delete, cv_PtrOfLATCH_get_inner_ptr, cv_PtrOfLATCH_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::xfeatures2d::LATCH> {
		#[inline] pub fn as_raw_PtrOfLATCH(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLATCH(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::LATCHTraitConst for core::Ptr<crate::xfeatures2d::LATCH> {
		#[inline] fn as_raw_LATCH(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::LATCHTrait for core::Ptr<crate::xfeatures2d::LATCH> {
		#[inline] fn as_raw_mut_LATCH(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::LATCH> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::LATCH> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::LATCH> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::LATCH> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfLATCH, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfLATCH_to_PtrOfFeature2D }
	
	pub type PtrOfLUCID = core::Ptr<crate::xfeatures2d::LUCID>;
	
	ptr_extern! { crate::xfeatures2d::LUCID,
		cv_PtrOfLUCID_delete, cv_PtrOfLUCID_get_inner_ptr, cv_PtrOfLUCID_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::xfeatures2d::LUCID> {
		#[inline] pub fn as_raw_PtrOfLUCID(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLUCID(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::LUCIDTraitConst for core::Ptr<crate::xfeatures2d::LUCID> {
		#[inline] fn as_raw_LUCID(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::LUCIDTrait for core::Ptr<crate::xfeatures2d::LUCID> {
		#[inline] fn as_raw_mut_LUCID(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::LUCID> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::LUCID> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::LUCID> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::LUCID> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfLUCID, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfLUCID_to_PtrOfFeature2D }
	
	pub type PtrOfMSDDetector = core::Ptr<crate::xfeatures2d::MSDDetector>;
	
	ptr_extern! { crate::xfeatures2d::MSDDetector,
		cv_PtrOfMSDDetector_delete, cv_PtrOfMSDDetector_get_inner_ptr, cv_PtrOfMSDDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::xfeatures2d::MSDDetector> {
		#[inline] pub fn as_raw_PtrOfMSDDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMSDDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::MSDDetectorTraitConst for core::Ptr<crate::xfeatures2d::MSDDetector> {
		#[inline] fn as_raw_MSDDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::MSDDetectorTrait for core::Ptr<crate::xfeatures2d::MSDDetector> {
		#[inline] fn as_raw_mut_MSDDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::MSDDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::MSDDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::MSDDetector> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::MSDDetector> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfMSDDetector, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfMSDDetector_to_PtrOfFeature2D }
	
	pub type PtrOfPCTSignatures = core::Ptr<crate::xfeatures2d::PCTSignatures>;
	
	ptr_extern! { crate::xfeatures2d::PCTSignatures,
		cv_PtrOfPCTSignatures_delete, cv_PtrOfPCTSignatures_get_inner_ptr, cv_PtrOfPCTSignatures_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::xfeatures2d::PCTSignatures> {
		#[inline] pub fn as_raw_PtrOfPCTSignatures(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPCTSignatures(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::PCTSignaturesTraitConst for core::Ptr<crate::xfeatures2d::PCTSignatures> {
		#[inline] fn as_raw_PCTSignatures(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::PCTSignaturesTrait for core::Ptr<crate::xfeatures2d::PCTSignatures> {
		#[inline] fn as_raw_mut_PCTSignatures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::PCTSignatures> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::PCTSignatures> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfPCTSignaturesSQFD = core::Ptr<crate::xfeatures2d::PCTSignaturesSQFD>;
	
	ptr_extern! { crate::xfeatures2d::PCTSignaturesSQFD,
		cv_PtrOfPCTSignaturesSQFD_delete, cv_PtrOfPCTSignaturesSQFD_get_inner_ptr, cv_PtrOfPCTSignaturesSQFD_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::xfeatures2d::PCTSignaturesSQFD> {
		#[inline] pub fn as_raw_PtrOfPCTSignaturesSQFD(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPCTSignaturesSQFD(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::PCTSignaturesSQFDTraitConst for core::Ptr<crate::xfeatures2d::PCTSignaturesSQFD> {
		#[inline] fn as_raw_PCTSignaturesSQFD(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::PCTSignaturesSQFDTrait for core::Ptr<crate::xfeatures2d::PCTSignaturesSQFD> {
		#[inline] fn as_raw_mut_PCTSignaturesSQFD(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::PCTSignaturesSQFD> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::PCTSignaturesSQFD> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSURF = core::Ptr<crate::xfeatures2d::SURF>;
	
	ptr_extern! { crate::xfeatures2d::SURF,
		cv_PtrOfSURF_delete, cv_PtrOfSURF_get_inner_ptr, cv_PtrOfSURF_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::xfeatures2d::SURF> {
		#[inline] pub fn as_raw_PtrOfSURF(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSURF(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::SURFTraitConst for core::Ptr<crate::xfeatures2d::SURF> {
		#[inline] fn as_raw_SURF(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::SURFTrait for core::Ptr<crate::xfeatures2d::SURF> {
		#[inline] fn as_raw_mut_SURF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::SURF> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::SURF> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::SURF> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::SURF> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSURF, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfSURF_to_PtrOfFeature2D }
	
	pub type PtrOfSURF_CUDA = core::Ptr<crate::xfeatures2d::SURF_CUDA>;
	
	ptr_extern! { crate::xfeatures2d::SURF_CUDA,
		cv_PtrOfSURF_CUDA_delete, cv_PtrOfSURF_CUDA_get_inner_ptr, cv_PtrOfSURF_CUDA_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::xfeatures2d::SURF_CUDA, cv_PtrOfSURF_CUDA_new }
	
	impl core::Ptr<crate::xfeatures2d::SURF_CUDA> {
		#[inline] pub fn as_raw_PtrOfSURF_CUDA(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSURF_CUDA(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::SURF_CUDATraitConst for core::Ptr<crate::xfeatures2d::SURF_CUDA> {
		#[inline] fn as_raw_SURF_CUDA(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::SURF_CUDATrait for core::Ptr<crate::xfeatures2d::SURF_CUDA> {
		#[inline] fn as_raw_mut_SURF_CUDA(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfStarDetector = core::Ptr<crate::xfeatures2d::StarDetector>;
	
	ptr_extern! { crate::xfeatures2d::StarDetector,
		cv_PtrOfStarDetector_delete, cv_PtrOfStarDetector_get_inner_ptr, cv_PtrOfStarDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::xfeatures2d::StarDetector> {
		#[inline] pub fn as_raw_PtrOfStarDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStarDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::StarDetectorTraitConst for core::Ptr<crate::xfeatures2d::StarDetector> {
		#[inline] fn as_raw_StarDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::StarDetectorTrait for core::Ptr<crate::xfeatures2d::StarDetector> {
		#[inline] fn as_raw_mut_StarDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::StarDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::StarDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::StarDetector> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::StarDetector> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfStarDetector, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfStarDetector_to_PtrOfFeature2D }
	
	pub type PtrOfTBMR = core::Ptr<crate::xfeatures2d::TBMR>;
	
	ptr_extern! { crate::xfeatures2d::TBMR,
		cv_PtrOfTBMR_delete, cv_PtrOfTBMR_get_inner_ptr, cv_PtrOfTBMR_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::xfeatures2d::TBMR> {
		#[inline] pub fn as_raw_PtrOfTBMR(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTBMR(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::TBMRTraitConst for core::Ptr<crate::xfeatures2d::TBMR> {
		#[inline] fn as_raw_TBMR(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::TBMRTrait for core::Ptr<crate::xfeatures2d::TBMR> {
		#[inline] fn as_raw_mut_TBMR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::TBMR> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::TBMR> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::TBMR> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::TBMR> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfTBMR, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfTBMR_to_PtrOfFeature2D }
	
	impl crate::xfeatures2d::AffineFeature2DTraitConst for core::Ptr<crate::xfeatures2d::TBMR> {
		#[inline] fn as_raw_AffineFeature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::AffineFeature2DTrait for core::Ptr<crate::xfeatures2d::TBMR> {
		#[inline] fn as_raw_mut_AffineFeature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTEBLID = core::Ptr<crate::xfeatures2d::TEBLID>;
	
	ptr_extern! { crate::xfeatures2d::TEBLID,
		cv_PtrOfTEBLID_delete, cv_PtrOfTEBLID_get_inner_ptr, cv_PtrOfTEBLID_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::xfeatures2d::TEBLID, cv_PtrOfTEBLID_new }
	
	impl core::Ptr<crate::xfeatures2d::TEBLID> {
		#[inline] pub fn as_raw_PtrOfTEBLID(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTEBLID(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::TEBLIDTraitConst for core::Ptr<crate::xfeatures2d::TEBLID> {
		#[inline] fn as_raw_TEBLID(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::TEBLIDTrait for core::Ptr<crate::xfeatures2d::TEBLID> {
		#[inline] fn as_raw_mut_TEBLID(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::TEBLID> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::TEBLID> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::TEBLID> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::TEBLID> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfTEBLID, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfTEBLID_to_PtrOfFeature2D }
	
	pub type PtrOfVGG = core::Ptr<crate::xfeatures2d::VGG>;
	
	ptr_extern! { crate::xfeatures2d::VGG,
		cv_PtrOfVGG_delete, cv_PtrOfVGG_get_inner_ptr, cv_PtrOfVGG_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::xfeatures2d::VGG> {
		#[inline] pub fn as_raw_PtrOfVGG(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfVGG(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xfeatures2d::VGGTraitConst for core::Ptr<crate::xfeatures2d::VGG> {
		#[inline] fn as_raw_VGG(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xfeatures2d::VGGTrait for core::Ptr<crate::xfeatures2d::VGG> {
		#[inline] fn as_raw_mut_VGG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::VGG> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::VGG> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::VGG> {
		#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::VGG> {
		#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfVGG, core::Ptr<crate::features2d::Feature2D>, cv_PtrOfVGG_to_PtrOfFeature2D }
	
	pub type VectorOfElliptic_KeyPoint = core::Vector<crate::xfeatures2d::Elliptic_KeyPoint>;
	
	impl core::Vector<crate::xfeatures2d::Elliptic_KeyPoint> {
		pub fn as_raw_VectorOfElliptic_KeyPoint(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfElliptic_KeyPoint(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	vector_extern! { crate::xfeatures2d::Elliptic_KeyPoint,
		cv_VectorOfElliptic_KeyPoint_new, cv_VectorOfElliptic_KeyPoint_delete,
		cv_VectorOfElliptic_KeyPoint_len, cv_VectorOfElliptic_KeyPoint_is_empty,
		cv_VectorOfElliptic_KeyPoint_capacity, cv_VectorOfElliptic_KeyPoint_shrink_to_fit,
		cv_VectorOfElliptic_KeyPoint_reserve, cv_VectorOfElliptic_KeyPoint_remove,
		cv_VectorOfElliptic_KeyPoint_swap, cv_VectorOfElliptic_KeyPoint_clear,
		cv_VectorOfElliptic_KeyPoint_get, cv_VectorOfElliptic_KeyPoint_set,
		cv_VectorOfElliptic_KeyPoint_push, cv_VectorOfElliptic_KeyPoint_insert,
	}
	vector_non_copy_or_bool! { crate::xfeatures2d::Elliptic_KeyPoint }
	
}
#[cfg(ocvrs_has_module_xfeatures2d)]
pub use xfeatures2d_types::*;

#[cfg(ocvrs_has_module_ximgproc)]
mod ximgproc_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfAdaptiveManifoldFilter = core::Ptr<crate::ximgproc::AdaptiveManifoldFilter>;
	
	ptr_extern! { crate::ximgproc::AdaptiveManifoldFilter,
		cv_PtrOfAdaptiveManifoldFilter_delete, cv_PtrOfAdaptiveManifoldFilter_get_inner_ptr, cv_PtrOfAdaptiveManifoldFilter_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::AdaptiveManifoldFilter> {
		#[inline] pub fn as_raw_PtrOfAdaptiveManifoldFilter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAdaptiveManifoldFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::AdaptiveManifoldFilterTraitConst for core::Ptr<crate::ximgproc::AdaptiveManifoldFilter> {
		#[inline] fn as_raw_AdaptiveManifoldFilter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::AdaptiveManifoldFilterTrait for core::Ptr<crate::ximgproc::AdaptiveManifoldFilter> {
		#[inline] fn as_raw_mut_AdaptiveManifoldFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::AdaptiveManifoldFilter> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::AdaptiveManifoldFilter> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfContourFitting = core::Ptr<crate::ximgproc::ContourFitting>;
	
	ptr_extern! { crate::ximgproc::ContourFitting,
		cv_PtrOfContourFitting_delete, cv_PtrOfContourFitting_get_inner_ptr, cv_PtrOfContourFitting_get_inner_ptr_mut
	}
	
	ptr_extern_ctor! { crate::ximgproc::ContourFitting, cv_PtrOfContourFitting_new }
	
	impl core::Ptr<crate::ximgproc::ContourFitting> {
		#[inline] pub fn as_raw_PtrOfContourFitting(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfContourFitting(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::ContourFittingTraitConst for core::Ptr<crate::ximgproc::ContourFitting> {
		#[inline] fn as_raw_ContourFitting(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::ContourFittingTrait for core::Ptr<crate::ximgproc::ContourFitting> {
		#[inline] fn as_raw_mut_ContourFitting(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::ContourFitting> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::ContourFitting> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDTFilter = core::Ptr<crate::ximgproc::DTFilter>;
	
	ptr_extern! { crate::ximgproc::DTFilter,
		cv_PtrOfDTFilter_delete, cv_PtrOfDTFilter_get_inner_ptr, cv_PtrOfDTFilter_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::DTFilter> {
		#[inline] pub fn as_raw_PtrOfDTFilter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDTFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::DTFilterTraitConst for core::Ptr<crate::ximgproc::DTFilter> {
		#[inline] fn as_raw_DTFilter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::DTFilterTrait for core::Ptr<crate::ximgproc::DTFilter> {
		#[inline] fn as_raw_mut_DTFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::DTFilter> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::DTFilter> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfDisparityWLSFilter = core::Ptr<crate::ximgproc::DisparityWLSFilter>;
	
	ptr_extern! { crate::ximgproc::DisparityWLSFilter,
		cv_PtrOfDisparityWLSFilter_delete, cv_PtrOfDisparityWLSFilter_get_inner_ptr, cv_PtrOfDisparityWLSFilter_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::DisparityWLSFilter> {
		#[inline] pub fn as_raw_PtrOfDisparityWLSFilter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDisparityWLSFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::DisparityWLSFilterTraitConst for core::Ptr<crate::ximgproc::DisparityWLSFilter> {
		#[inline] fn as_raw_DisparityWLSFilter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::DisparityWLSFilterTrait for core::Ptr<crate::ximgproc::DisparityWLSFilter> {
		#[inline] fn as_raw_mut_DisparityWLSFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::DisparityWLSFilter> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::DisparityWLSFilter> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::DisparityFilterTraitConst for core::Ptr<crate::ximgproc::DisparityWLSFilter> {
		#[inline] fn as_raw_DisparityFilter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::DisparityFilterTrait for core::Ptr<crate::ximgproc::DisparityWLSFilter> {
		#[inline] fn as_raw_mut_DisparityFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEdgeAwareInterpolator = core::Ptr<crate::ximgproc::EdgeAwareInterpolator>;
	
	ptr_extern! { crate::ximgproc::EdgeAwareInterpolator,
		cv_PtrOfEdgeAwareInterpolator_delete, cv_PtrOfEdgeAwareInterpolator_get_inner_ptr, cv_PtrOfEdgeAwareInterpolator_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::EdgeAwareInterpolator> {
		#[inline] pub fn as_raw_PtrOfEdgeAwareInterpolator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEdgeAwareInterpolator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::EdgeAwareInterpolatorTraitConst for core::Ptr<crate::ximgproc::EdgeAwareInterpolator> {
		#[inline] fn as_raw_EdgeAwareInterpolator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::EdgeAwareInterpolatorTrait for core::Ptr<crate::ximgproc::EdgeAwareInterpolator> {
		#[inline] fn as_raw_mut_EdgeAwareInterpolator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::EdgeAwareInterpolator> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::EdgeAwareInterpolator> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SparseMatchInterpolatorTraitConst for core::Ptr<crate::ximgproc::EdgeAwareInterpolator> {
		#[inline] fn as_raw_SparseMatchInterpolator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SparseMatchInterpolatorTrait for core::Ptr<crate::ximgproc::EdgeAwareInterpolator> {
		#[inline] fn as_raw_mut_SparseMatchInterpolator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEdgeBoxes = core::Ptr<crate::ximgproc::EdgeBoxes>;
	
	ptr_extern! { crate::ximgproc::EdgeBoxes,
		cv_PtrOfEdgeBoxes_delete, cv_PtrOfEdgeBoxes_get_inner_ptr, cv_PtrOfEdgeBoxes_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::EdgeBoxes> {
		#[inline] pub fn as_raw_PtrOfEdgeBoxes(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEdgeBoxes(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::EdgeBoxesTraitConst for core::Ptr<crate::ximgproc::EdgeBoxes> {
		#[inline] fn as_raw_EdgeBoxes(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::EdgeBoxesTrait for core::Ptr<crate::ximgproc::EdgeBoxes> {
		#[inline] fn as_raw_mut_EdgeBoxes(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::EdgeBoxes> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::EdgeBoxes> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfEdgeDrawing = core::Ptr<crate::ximgproc::EdgeDrawing>;
	
	ptr_extern! { crate::ximgproc::EdgeDrawing,
		cv_PtrOfEdgeDrawing_delete, cv_PtrOfEdgeDrawing_get_inner_ptr, cv_PtrOfEdgeDrawing_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::EdgeDrawing> {
		#[inline] pub fn as_raw_PtrOfEdgeDrawing(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEdgeDrawing(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::EdgeDrawingTraitConst for core::Ptr<crate::ximgproc::EdgeDrawing> {
		#[inline] fn as_raw_EdgeDrawing(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::EdgeDrawingTrait for core::Ptr<crate::ximgproc::EdgeDrawing> {
		#[inline] fn as_raw_mut_EdgeDrawing(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::EdgeDrawing> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::EdgeDrawing> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFastBilateralSolverFilter = core::Ptr<crate::ximgproc::FastBilateralSolverFilter>;
	
	ptr_extern! { crate::ximgproc::FastBilateralSolverFilter,
		cv_PtrOfFastBilateralSolverFilter_delete, cv_PtrOfFastBilateralSolverFilter_get_inner_ptr, cv_PtrOfFastBilateralSolverFilter_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::FastBilateralSolverFilter> {
		#[inline] pub fn as_raw_PtrOfFastBilateralSolverFilter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFastBilateralSolverFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::FastBilateralSolverFilterTraitConst for core::Ptr<crate::ximgproc::FastBilateralSolverFilter> {
		#[inline] fn as_raw_FastBilateralSolverFilter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::FastBilateralSolverFilterTrait for core::Ptr<crate::ximgproc::FastBilateralSolverFilter> {
		#[inline] fn as_raw_mut_FastBilateralSolverFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::FastBilateralSolverFilter> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::FastBilateralSolverFilter> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFastGlobalSmootherFilter = core::Ptr<crate::ximgproc::FastGlobalSmootherFilter>;
	
	ptr_extern! { crate::ximgproc::FastGlobalSmootherFilter,
		cv_PtrOfFastGlobalSmootherFilter_delete, cv_PtrOfFastGlobalSmootherFilter_get_inner_ptr, cv_PtrOfFastGlobalSmootherFilter_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::FastGlobalSmootherFilter> {
		#[inline] pub fn as_raw_PtrOfFastGlobalSmootherFilter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFastGlobalSmootherFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::FastGlobalSmootherFilterTraitConst for core::Ptr<crate::ximgproc::FastGlobalSmootherFilter> {
		#[inline] fn as_raw_FastGlobalSmootherFilter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::FastGlobalSmootherFilterTrait for core::Ptr<crate::ximgproc::FastGlobalSmootherFilter> {
		#[inline] fn as_raw_mut_FastGlobalSmootherFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::FastGlobalSmootherFilter> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::FastGlobalSmootherFilter> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfFastLineDetector = core::Ptr<crate::ximgproc::FastLineDetector>;
	
	ptr_extern! { crate::ximgproc::FastLineDetector,
		cv_PtrOfFastLineDetector_delete, cv_PtrOfFastLineDetector_get_inner_ptr, cv_PtrOfFastLineDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::FastLineDetector> {
		#[inline] pub fn as_raw_PtrOfFastLineDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFastLineDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::FastLineDetectorTraitConst for core::Ptr<crate::ximgproc::FastLineDetector> {
		#[inline] fn as_raw_FastLineDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::FastLineDetectorTrait for core::Ptr<crate::ximgproc::FastLineDetector> {
		#[inline] fn as_raw_mut_FastLineDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::FastLineDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::FastLineDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGraphSegmentation = core::Ptr<crate::ximgproc::GraphSegmentation>;
	
	ptr_extern! { crate::ximgproc::GraphSegmentation,
		cv_PtrOfGraphSegmentation_delete, cv_PtrOfGraphSegmentation_get_inner_ptr, cv_PtrOfGraphSegmentation_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::GraphSegmentation> {
		#[inline] pub fn as_raw_PtrOfGraphSegmentation(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGraphSegmentation(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::GraphSegmentationTraitConst for core::Ptr<crate::ximgproc::GraphSegmentation> {
		#[inline] fn as_raw_GraphSegmentation(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::GraphSegmentationTrait for core::Ptr<crate::ximgproc::GraphSegmentation> {
		#[inline] fn as_raw_mut_GraphSegmentation(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::GraphSegmentation> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::GraphSegmentation> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfGuidedFilter = core::Ptr<crate::ximgproc::GuidedFilter>;
	
	ptr_extern! { crate::ximgproc::GuidedFilter,
		cv_PtrOfGuidedFilter_delete, cv_PtrOfGuidedFilter_get_inner_ptr, cv_PtrOfGuidedFilter_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::GuidedFilter> {
		#[inline] pub fn as_raw_PtrOfGuidedFilter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGuidedFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::GuidedFilterTraitConst for core::Ptr<crate::ximgproc::GuidedFilter> {
		#[inline] fn as_raw_GuidedFilter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::GuidedFilterTrait for core::Ptr<crate::ximgproc::GuidedFilter> {
		#[inline] fn as_raw_mut_GuidedFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::GuidedFilter> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::GuidedFilter> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRFFeatureGetter = core::Ptr<crate::ximgproc::RFFeatureGetter>;
	
	ptr_extern! { crate::ximgproc::RFFeatureGetter,
		cv_PtrOfRFFeatureGetter_delete, cv_PtrOfRFFeatureGetter_get_inner_ptr, cv_PtrOfRFFeatureGetter_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::RFFeatureGetter> {
		#[inline] pub fn as_raw_PtrOfRFFeatureGetter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRFFeatureGetter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::RFFeatureGetterTraitConst for core::Ptr<crate::ximgproc::RFFeatureGetter> {
		#[inline] fn as_raw_RFFeatureGetter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::RFFeatureGetterTrait for core::Ptr<crate::ximgproc::RFFeatureGetter> {
		#[inline] fn as_raw_mut_RFFeatureGetter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::RFFeatureGetter> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::RFFeatureGetter> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRICInterpolator = core::Ptr<crate::ximgproc::RICInterpolator>;
	
	ptr_extern! { crate::ximgproc::RICInterpolator,
		cv_PtrOfRICInterpolator_delete, cv_PtrOfRICInterpolator_get_inner_ptr, cv_PtrOfRICInterpolator_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::RICInterpolator> {
		#[inline] pub fn as_raw_PtrOfRICInterpolator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRICInterpolator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::RICInterpolatorTraitConst for core::Ptr<crate::ximgproc::RICInterpolator> {
		#[inline] fn as_raw_RICInterpolator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::RICInterpolatorTrait for core::Ptr<crate::ximgproc::RICInterpolator> {
		#[inline] fn as_raw_mut_RICInterpolator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::RICInterpolator> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::RICInterpolator> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SparseMatchInterpolatorTraitConst for core::Ptr<crate::ximgproc::RICInterpolator> {
		#[inline] fn as_raw_SparseMatchInterpolator(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SparseMatchInterpolatorTrait for core::Ptr<crate::ximgproc::RICInterpolator> {
		#[inline] fn as_raw_mut_SparseMatchInterpolator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfRidgeDetectionFilter = core::Ptr<crate::ximgproc::RidgeDetectionFilter>;
	
	ptr_extern! { crate::ximgproc::RidgeDetectionFilter,
		cv_PtrOfRidgeDetectionFilter_delete, cv_PtrOfRidgeDetectionFilter_get_inner_ptr, cv_PtrOfRidgeDetectionFilter_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::RidgeDetectionFilter> {
		#[inline] pub fn as_raw_PtrOfRidgeDetectionFilter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRidgeDetectionFilter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::RidgeDetectionFilterTraitConst for core::Ptr<crate::ximgproc::RidgeDetectionFilter> {
		#[inline] fn as_raw_RidgeDetectionFilter(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::RidgeDetectionFilterTrait for core::Ptr<crate::ximgproc::RidgeDetectionFilter> {
		#[inline] fn as_raw_mut_RidgeDetectionFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::RidgeDetectionFilter> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::RidgeDetectionFilter> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfScanSegment = core::Ptr<crate::ximgproc::ScanSegment>;
	
	ptr_extern! { crate::ximgproc::ScanSegment,
		cv_PtrOfScanSegment_delete, cv_PtrOfScanSegment_get_inner_ptr, cv_PtrOfScanSegment_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::ScanSegment> {
		#[inline] pub fn as_raw_PtrOfScanSegment(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfScanSegment(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::ScanSegmentTraitConst for core::Ptr<crate::ximgproc::ScanSegment> {
		#[inline] fn as_raw_ScanSegment(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::ScanSegmentTrait for core::Ptr<crate::ximgproc::ScanSegment> {
		#[inline] fn as_raw_mut_ScanSegment(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::ScanSegment> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::ScanSegment> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSelectiveSearchSegmentation = core::Ptr<crate::ximgproc::SelectiveSearchSegmentation>;
	
	ptr_extern! { crate::ximgproc::SelectiveSearchSegmentation,
		cv_PtrOfSelectiveSearchSegmentation_delete, cv_PtrOfSelectiveSearchSegmentation_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentation_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::SelectiveSearchSegmentation> {
		#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentation(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentation(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentation> {
		#[inline] fn as_raw_SelectiveSearchSegmentation(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentation> {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentation(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentation> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentation> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSelectiveSearchSegmentationStrategy = core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategy>;
	
	ptr_extern! { crate::ximgproc::SelectiveSearchSegmentationStrategy,
		cv_PtrOfSelectiveSearchSegmentationStrategy_delete, cv_PtrOfSelectiveSearchSegmentationStrategy_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategy_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategy> {
		#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategy(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategy> {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategy> {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategy> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategy> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSelectiveSearchSegmentationStrategyColor = core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyColor>;
	
	ptr_extern! { crate::ximgproc::SelectiveSearchSegmentationStrategyColor,
		cv_PtrOfSelectiveSearchSegmentationStrategyColor_delete, cv_PtrOfSelectiveSearchSegmentationStrategyColor_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategyColor_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyColor> {
		#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategyColor(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategyColor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyColorTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyColor> {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategyColor(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyColorTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyColor> {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategyColor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyColor> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyColor> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyColor> {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyColor> {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSelectiveSearchSegmentationStrategyColor, core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategy>, cv_PtrOfSelectiveSearchSegmentationStrategyColor_to_PtrOfSelectiveSearchSegmentationStrategy }
	
	pub type PtrOfSelectiveSearchSegmentationStrategyFill = core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyFill>;
	
	ptr_extern! { crate::ximgproc::SelectiveSearchSegmentationStrategyFill,
		cv_PtrOfSelectiveSearchSegmentationStrategyFill_delete, cv_PtrOfSelectiveSearchSegmentationStrategyFill_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategyFill_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyFill> {
		#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategyFill(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategyFill(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyFillTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyFill> {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategyFill(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyFillTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyFill> {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategyFill(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyFill> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyFill> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyFill> {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyFill> {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSelectiveSearchSegmentationStrategyFill, core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategy>, cv_PtrOfSelectiveSearchSegmentationStrategyFill_to_PtrOfSelectiveSearchSegmentationStrategy }
	
	pub type PtrOfSelectiveSearchSegmentationStrategyMultiple = core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>;
	
	ptr_extern! { crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple,
		cv_PtrOfSelectiveSearchSegmentationStrategyMultiple_delete, cv_PtrOfSelectiveSearchSegmentationStrategyMultiple_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategyMultiple_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple> {
		#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategyMultiple(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategyMultiple(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyMultipleTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple> {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategyMultiple(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyMultipleTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple> {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategyMultiple(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple> {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple> {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSelectiveSearchSegmentationStrategyMultiple, core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategy>, cv_PtrOfSelectiveSearchSegmentationStrategyMultiple_to_PtrOfSelectiveSearchSegmentationStrategy }
	
	pub type PtrOfSelectiveSearchSegmentationStrategySize = core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategySize>;
	
	ptr_extern! { crate::ximgproc::SelectiveSearchSegmentationStrategySize,
		cv_PtrOfSelectiveSearchSegmentationStrategySize_delete, cv_PtrOfSelectiveSearchSegmentationStrategySize_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategySize_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategySize> {
		#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategySize(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategySize(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategySizeTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategySize> {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategySize(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategySizeTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategySize> {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategySize(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategySize> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategySize> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategySize> {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategySize> {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSelectiveSearchSegmentationStrategySize, core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategy>, cv_PtrOfSelectiveSearchSegmentationStrategySize_to_PtrOfSelectiveSearchSegmentationStrategy }
	
	pub type PtrOfSelectiveSearchSegmentationStrategyTexture = core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyTexture>;
	
	ptr_extern! { crate::ximgproc::SelectiveSearchSegmentationStrategyTexture,
		cv_PtrOfSelectiveSearchSegmentationStrategyTexture_delete, cv_PtrOfSelectiveSearchSegmentationStrategyTexture_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategyTexture_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyTexture> {
		#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategyTexture(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategyTexture(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyTextureTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyTexture> {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategyTexture(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyTextureTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyTexture> {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategyTexture(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyTexture> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyTexture> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyTraitConst for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyTexture> {
		#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SelectiveSearchSegmentationStrategyTrait for core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategyTexture> {
		#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	ptr_cast_base! { PtrOfSelectiveSearchSegmentationStrategyTexture, core::Ptr<crate::ximgproc::SelectiveSearchSegmentationStrategy>, cv_PtrOfSelectiveSearchSegmentationStrategyTexture_to_PtrOfSelectiveSearchSegmentationStrategy }
	
	pub type PtrOfStructuredEdgeDetection = core::Ptr<crate::ximgproc::StructuredEdgeDetection>;
	
	ptr_extern! { crate::ximgproc::StructuredEdgeDetection,
		cv_PtrOfStructuredEdgeDetection_delete, cv_PtrOfStructuredEdgeDetection_get_inner_ptr, cv_PtrOfStructuredEdgeDetection_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::StructuredEdgeDetection> {
		#[inline] pub fn as_raw_PtrOfStructuredEdgeDetection(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfStructuredEdgeDetection(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::StructuredEdgeDetectionTraitConst for core::Ptr<crate::ximgproc::StructuredEdgeDetection> {
		#[inline] fn as_raw_StructuredEdgeDetection(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::StructuredEdgeDetectionTrait for core::Ptr<crate::ximgproc::StructuredEdgeDetection> {
		#[inline] fn as_raw_mut_StructuredEdgeDetection(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::StructuredEdgeDetection> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::StructuredEdgeDetection> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperpixelLSC = core::Ptr<crate::ximgproc::SuperpixelLSC>;
	
	ptr_extern! { crate::ximgproc::SuperpixelLSC,
		cv_PtrOfSuperpixelLSC_delete, cv_PtrOfSuperpixelLSC_get_inner_ptr, cv_PtrOfSuperpixelLSC_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::SuperpixelLSC> {
		#[inline] pub fn as_raw_PtrOfSuperpixelLSC(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperpixelLSC(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::SuperpixelLSCTraitConst for core::Ptr<crate::ximgproc::SuperpixelLSC> {
		#[inline] fn as_raw_SuperpixelLSC(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SuperpixelLSCTrait for core::Ptr<crate::ximgproc::SuperpixelLSC> {
		#[inline] fn as_raw_mut_SuperpixelLSC(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::SuperpixelLSC> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::SuperpixelLSC> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperpixelSEEDS = core::Ptr<crate::ximgproc::SuperpixelSEEDS>;
	
	ptr_extern! { crate::ximgproc::SuperpixelSEEDS,
		cv_PtrOfSuperpixelSEEDS_delete, cv_PtrOfSuperpixelSEEDS_get_inner_ptr, cv_PtrOfSuperpixelSEEDS_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::SuperpixelSEEDS> {
		#[inline] pub fn as_raw_PtrOfSuperpixelSEEDS(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperpixelSEEDS(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::SuperpixelSEEDSTraitConst for core::Ptr<crate::ximgproc::SuperpixelSEEDS> {
		#[inline] fn as_raw_SuperpixelSEEDS(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SuperpixelSEEDSTrait for core::Ptr<crate::ximgproc::SuperpixelSEEDS> {
		#[inline] fn as_raw_mut_SuperpixelSEEDS(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::SuperpixelSEEDS> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::SuperpixelSEEDS> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSuperpixelSLIC = core::Ptr<crate::ximgproc::SuperpixelSLIC>;
	
	ptr_extern! { crate::ximgproc::SuperpixelSLIC,
		cv_PtrOfSuperpixelSLIC_delete, cv_PtrOfSuperpixelSLIC_get_inner_ptr, cv_PtrOfSuperpixelSLIC_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::ximgproc::SuperpixelSLIC> {
		#[inline] pub fn as_raw_PtrOfSuperpixelSLIC(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSuperpixelSLIC(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::ximgproc::SuperpixelSLICTraitConst for core::Ptr<crate::ximgproc::SuperpixelSLIC> {
		#[inline] fn as_raw_SuperpixelSLIC(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::ximgproc::SuperpixelSLICTrait for core::Ptr<crate::ximgproc::SuperpixelSLIC> {
		#[inline] fn as_raw_mut_SuperpixelSLIC(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::ximgproc::SuperpixelSLIC> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::ximgproc::SuperpixelSLIC> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_ximgproc)]
pub use ximgproc_types::*;

#[cfg(ocvrs_has_module_xobjdetect)]
mod xobjdetect_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfWBDetector = core::Ptr<crate::xobjdetect::WBDetector>;
	
	ptr_extern! { crate::xobjdetect::WBDetector,
		cv_PtrOfWBDetector_delete, cv_PtrOfWBDetector_get_inner_ptr, cv_PtrOfWBDetector_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::xobjdetect::WBDetector> {
		#[inline] pub fn as_raw_PtrOfWBDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfWBDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xobjdetect::WBDetectorTraitConst for core::Ptr<crate::xobjdetect::WBDetector> {
		#[inline] fn as_raw_WBDetector(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xobjdetect::WBDetectorTrait for core::Ptr<crate::xobjdetect::WBDetector> {
		#[inline] fn as_raw_mut_WBDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_xobjdetect)]
pub use xobjdetect_types::*;

#[cfg(ocvrs_has_module_xphoto)]
mod xphoto_types {
	use crate::{mod_prelude::*, core, types, sys};

	pub type PtrOfGrayworldWB = core::Ptr<crate::xphoto::GrayworldWB>;
	
	ptr_extern! { crate::xphoto::GrayworldWB,
		cv_PtrOfGrayworldWB_delete, cv_PtrOfGrayworldWB_get_inner_ptr, cv_PtrOfGrayworldWB_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::xphoto::GrayworldWB> {
		#[inline] pub fn as_raw_PtrOfGrayworldWB(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGrayworldWB(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xphoto::GrayworldWBTraitConst for core::Ptr<crate::xphoto::GrayworldWB> {
		#[inline] fn as_raw_GrayworldWB(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xphoto::GrayworldWBTrait for core::Ptr<crate::xphoto::GrayworldWB> {
		#[inline] fn as_raw_mut_GrayworldWB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::xphoto::GrayworldWB> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::xphoto::GrayworldWB> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xphoto::WhiteBalancerTraitConst for core::Ptr<crate::xphoto::GrayworldWB> {
		#[inline] fn as_raw_WhiteBalancer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xphoto::WhiteBalancerTrait for core::Ptr<crate::xphoto::GrayworldWB> {
		#[inline] fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfLearningBasedWB = core::Ptr<crate::xphoto::LearningBasedWB>;
	
	ptr_extern! { crate::xphoto::LearningBasedWB,
		cv_PtrOfLearningBasedWB_delete, cv_PtrOfLearningBasedWB_get_inner_ptr, cv_PtrOfLearningBasedWB_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::xphoto::LearningBasedWB> {
		#[inline] pub fn as_raw_PtrOfLearningBasedWB(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLearningBasedWB(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xphoto::LearningBasedWBTraitConst for core::Ptr<crate::xphoto::LearningBasedWB> {
		#[inline] fn as_raw_LearningBasedWB(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xphoto::LearningBasedWBTrait for core::Ptr<crate::xphoto::LearningBasedWB> {
		#[inline] fn as_raw_mut_LearningBasedWB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::xphoto::LearningBasedWB> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::xphoto::LearningBasedWB> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xphoto::WhiteBalancerTraitConst for core::Ptr<crate::xphoto::LearningBasedWB> {
		#[inline] fn as_raw_WhiteBalancer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xphoto::WhiteBalancerTrait for core::Ptr<crate::xphoto::LearningBasedWB> {
		#[inline] fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfSimpleWB = core::Ptr<crate::xphoto::SimpleWB>;
	
	ptr_extern! { crate::xphoto::SimpleWB,
		cv_PtrOfSimpleWB_delete, cv_PtrOfSimpleWB_get_inner_ptr, cv_PtrOfSimpleWB_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::xphoto::SimpleWB> {
		#[inline] pub fn as_raw_PtrOfSimpleWB(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSimpleWB(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xphoto::SimpleWBTraitConst for core::Ptr<crate::xphoto::SimpleWB> {
		#[inline] fn as_raw_SimpleWB(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xphoto::SimpleWBTrait for core::Ptr<crate::xphoto::SimpleWB> {
		#[inline] fn as_raw_mut_SimpleWB(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::xphoto::SimpleWB> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::xphoto::SimpleWB> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::xphoto::WhiteBalancerTraitConst for core::Ptr<crate::xphoto::SimpleWB> {
		#[inline] fn as_raw_WhiteBalancer(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xphoto::WhiteBalancerTrait for core::Ptr<crate::xphoto::SimpleWB> {
		#[inline] fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	pub type PtrOfTonemapDurand = core::Ptr<crate::xphoto::TonemapDurand>;
	
	ptr_extern! { crate::xphoto::TonemapDurand,
		cv_PtrOfTonemapDurand_delete, cv_PtrOfTonemapDurand_get_inner_ptr, cv_PtrOfTonemapDurand_get_inner_ptr_mut
	}
	
	impl core::Ptr<crate::xphoto::TonemapDurand> {
		#[inline] pub fn as_raw_PtrOfTonemapDurand(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTonemapDurand(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}
	
	impl crate::xphoto::TonemapDurandTraitConst for core::Ptr<crate::xphoto::TonemapDurand> {
		#[inline] fn as_raw_TonemapDurand(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::xphoto::TonemapDurandTrait for core::Ptr<crate::xphoto::TonemapDurand> {
		#[inline] fn as_raw_mut_TonemapDurand(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl core::AlgorithmTraitConst for core::Ptr<crate::xphoto::TonemapDurand> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl core::AlgorithmTrait for core::Ptr<crate::xphoto::TonemapDurand> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
	impl crate::photo::TonemapTraitConst for core::Ptr<crate::xphoto::TonemapDurand> {
		#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
	}
	
	impl crate::photo::TonemapTrait for core::Ptr<crate::xphoto::TonemapDurand> {
		#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}
	
}
#[cfg(ocvrs_has_module_xphoto)]
pub use xphoto_types::*;

pub use crate::manual::types::*;
