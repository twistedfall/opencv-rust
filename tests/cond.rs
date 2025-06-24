use opencv::{
	not_opencv_branch_34, not_opencv_branch_4, not_opencv_branch_5, opencv_branch_34, opencv_branch_4, opencv_branch_5,
	opencv_has_inherent_feature_cuda, opencv_has_module_core, opencv_has_module_cudalegacy, opencv_has_module_imgproc,
};

not_opencv_branch_34! {
	use opencv::imgproc::LINE_8;
}
opencv_branch_34! {
	use opencv::core::LINE_8;
}
opencv_has_module_imgproc! {
	use opencv::imgproc::line_def;
}
opencv_has_inherent_feature_cuda! {
	use opencv::core::GpuMat;
}

#[test]
fn test_opencv_branch_cond_macros_code() {
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
fn test_opencv_branch_cond_macros_use() {
	assert_eq!(8, LINE_8);
}

#[test]
fn test_opencv_module_cond_macros_code() {
	{
		let always_has_core = opencv_has_module_core! { true };
		assert!(always_has_core);
	}

	{
		let likely_has_imgproc = opencv_has_module_imgproc! { { true } else { false } };

		#[cfg(ocvrs_has_module_imgproc)]
		assert!(likely_has_imgproc);
		#[cfg(not(ocvrs_has_module_imgproc))]
		assert!(!likely_has_imgproc);
	}

	{
		let unlikely_has_cudalegacy = opencv_has_module_cudalegacy! { { true } else { false } };

		#[cfg(ocvrs_has_module_cudalegacy)]
		assert!(unlikely_has_cudalegacy);
		#[cfg(not(ocvrs_has_module_cudalegacy))]
		assert!(!unlikely_has_cudalegacy);
	}
}

#[test]
fn test_opencv_module_cond_macros_use() {
	opencv_has_module_imgproc! {
		use opencv::prelude::*;
		use opencv::core::Point;

		let mut m = Mat::new_rows_cols_with_default(1, 3, u8::opencv_type(), 0.into()).unwrap();
		line_def(&mut m, Point::new(0, 0), Point::new(2,0), 255.into()).unwrap();
		assert_eq!([255, 255, 255], m.data_typed::<u8>().unwrap());
	}
}

#[test]
fn test_opencv_inherent_feature_cond_macros_code() {
	let cuda_available = opencv_has_inherent_feature_cuda! { { true } else { false } };
	#[cfg(ocvrs_has_inherent_feature_cuda)]
	assert!(cuda_available);
	#[cfg(not(ocvrs_has_inherent_feature_cuda))]
	assert!(!cuda_available);
}

#[test]
fn test_opencv_inherent_feature_cond_macros_use() {
	opencv_has_inherent_feature_cuda! {
		use opencv::prelude::*;

		let mut m = GpuMat::new_def().unwrap();
		m.upload(&[1, 2, 3]).unwrap()
	}
}
