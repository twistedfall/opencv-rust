use opencv::{
	not_opencv_branch_34, not_opencv_branch_4, not_opencv_branch_5, opencv_branch_34, opencv_branch_4, opencv_branch_5,
	opencv_has_module_core,
};

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
