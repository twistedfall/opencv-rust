use opencv::cond_functions::opencv_has_module;
use opencv::{
	not_opencv_branch_34, not_opencv_branch_4, not_opencv_branch_5, opencv_branch_34, opencv_branch_4, opencv_branch_5,
	opencv_has_module_core, opencv_has_module_cudalegacy, opencv_has_module_imgproc,
};
use opencv_binding_generator::SupportedModule;

#[test]
fn test_opencv_branch_cond_macros() {
	opencv_branch_5! { let cond_macro_branch_5 =  true; }
	not_opencv_branch_5! { let cond_macro_branch_5 =  false; }
	let cfg_branch_5 = cfg!(ocvrs_opencv_branch_5);
	assert_eq!(cond_macro_branch_5, cfg_branch_5);

	opencv_branch_4! { let cond_macro_branch_4 =  true; }
	not_opencv_branch_4! { let cond_macro_branch_4 =  false; }
	let cfg_branch_4 = cfg!(ocvrs_opencv_branch_4);
	assert_eq!(cond_macro_branch_4, cfg_branch_4);

	opencv_branch_34! { let cond_macro_branch_34 =  true; }
	not_opencv_branch_34! { let cond_macro_branch_34 =  false; }
	let cfg_branch_34 = cfg!(ocvrs_opencv_branch_34);
	assert_eq!(cond_macro_branch_34, cfg_branch_34);
}

#[test]
fn test_opencv_module_cond_macros() {
	#![allow(unused_assignments, unused_mut)]
	{
		let always_has_core = opencv_has_module_core! { true };
		assert!(always_has_core);
	}

	{
		let mut has_imgproc = false;
		opencv_has_module_imgproc! { has_imgproc = true; }

		assert_eq!(has_imgproc, opencv_has_module(SupportedModule::ImgProc));

		#[cfg(ocvrs_has_module_imgproc)]
		assert!(has_imgproc);

		#[cfg(not(ocvrs_has_module_imgproc))]
		assert!(!has_imgproc);
	}

	{
		let mut has_cudalegacy = false;
		opencv_has_module_cudalegacy! { has_cudalegacy = true; }

		#[cfg(ocvrs_has_module_cudalegacy)]
		assert!(has_cudalegacy);

		#[cfg(not(ocvrs_has_module_cudalegacy))]
		assert!(!has_cudalegacy);

		assert_eq!(has_cudalegacy, opencv_has_module(SupportedModule::CudaLegacy));
	}
}
