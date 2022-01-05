//! Contains all tests that cover marshalling types to and from C++

use opencv::{core::{self, Scalar}, Error, prelude::*, Result};

/// Passing simple struct as argument
#[test]
fn simple_struct_arg() -> Result<()> {
	use opencv::{imgproc, core::{Point, Size}};

	let res = imgproc::get_structuring_element(imgproc::MORPH_CROSS, Size { width: 100, height: 100 }, Point { x: 50, y: 50 })?;
	assert_eq!(res.typ(), 0);
	let size = res.size()?;
	assert_eq!(size.width, 100);
	assert_eq!(size.height, 100);
	Ok(())
}

#[test]
fn scalar_arg() -> Result<()> {
	let mut m = Mat::new_rows_cols_with_default(1, 3, u8::typ(), Scalar::new(2., 0., 0., 0.))?;
	let sum = core::sum_elems(&m)?;
	assert_eq!(sum[0], 6.);
	let s = m.at_row_mut::<u8>(0)?;
	s[0] = 1;
	s[1] = 2;
	s[2] = 3;
	let sum = core::sum_elems(&m)?;
	assert_eq!(sum[0], 6.);
	Ok(())
}

/// Return of the primitive types and passing them as arguments
#[test]
fn primitives_return() -> Result<()> {
	assert!(core::get_tick_count()? > 10000);
	assert!(core::get_cpu_tick_count()? > 10000);
	assert!(core::get_tick_frequency()? > 1000.);
	assert!(core::get_number_of_cpus()? >= 1);
	core::set_use_optimized(true)?;
	assert!(core::use_optimized()?);
	core::set_use_optimized(false)?;
	assert!(!core::use_optimized()?);
	Ok(())
}

/// Passing callback and checking that it's called
#[test]
fn callback() -> Result<()> {
	#![cfg(ocvrs_has_module_highgui)]
	use std::sync::{Arc, Mutex};

	use opencv::highgui;

	// only run under X11 on linux
	if cfg!(target_os = "linux") && option_env!("DISPLAY").is_some() {
		{
			highgui::named_window("test_1", 0)?;
			let mut value = 50;
			let cb_value = Arc::new(Mutex::new(0));
			highgui::create_trackbar("test_track_1", "test_1", Some(&mut value), 100, Some(Box::new({
				let cb_value = cb_value.clone();
				move |s| {
					*cb_value.lock().unwrap() = s;
				}
			})))?;
			assert_eq!(value, 50);
			highgui::set_trackbar_pos("test_track_1", "test_1", 10)?;
			assert_eq!(value, 10);
			assert_eq!(*cb_value.lock().unwrap(), 10);
		}

		{
			highgui::named_window("test_2", 0)?;
			let cb_value = Arc::new(Mutex::new(0));
			highgui::create_trackbar("test_track_2", "test_2", None, 100, Some(Box::new({
				let cb_value = cb_value.clone();
				move |s| {
					*cb_value.lock().unwrap() = s;
				}
			})))?;
			highgui::set_trackbar_pos("test_track_2", "test_2", 10)?;
			assert_eq!(*cb_value.lock().unwrap(), 10);
		}
	}
	Ok(())
}

/// Return of fixed array
#[test]
fn fixed_array_return() -> Result<()> {
	// mutable fixed array return and modification
	{
		let m = Mat::new_rows_cols_with_default(5, 3, i32::typ(), Scalar::all(1.))?;
		let mut mat_step = m.mat_step();
		assert_eq!([12, 4], *mat_step.buf());
		mat_step.buf()[0] = 16;
		mat_step.buf()[1] = 2;
		assert_eq!([16, 2], *mat_step.buf());
	}

	Ok(())
}

/// Return of an owned String
#[test]
fn string_return() -> Result<()> {
	let info = core::get_build_information()?;
	assert!(info.contains("\nGeneral configuration for OpenCV"));
	Ok(())
}

/// Return String via a mutable argument
#[test]
fn string_out_argument() -> Result<()> {
	#![cfg(ocvrs_opencv_branch_4)]
	use opencv::core::{FileStorage_Mode, FileStorage, FileNode};

	use matches::assert_matches;

	{
		let mut st = FileStorage::new(".yml", FileStorage_Mode::WRITE as i32 | FileStorage_Mode::MEMORY as i32, "")?;
		st.write_str("test_str", "test string")?;
		let serialized = st.release_and_get_string()?;
		let st = FileStorage::new(&serialized, FileStorage_Mode::MEMORY as _, "")?;
		let str_node = st.get("test_str")?;
		let mut str_out = String::new();
		core::read_str(&str_node, &mut str_out, "default string")?;
		assert_eq!("test string", str_out);
	}

	// correctly handle output string on error condition
	{
		let st = FileStorage::new("", FileStorage_Mode::WRITE as i32 | FileStorage_Mode::MEMORY as i32, "")?;
		let node = FileNode::new(&st, 0, 0)?;
		let mut out = String::new();
		assert_matches!(core::read_str(&node, &mut out, "123"), Err(Error { code: core::StsAssert, .. }));
		assert_eq!("", out);
	}

	Ok(())
}

/// Setting and getting fields through Ptr
#[test]
fn field_access_on_ptr() -> Result<()> {
	#![cfg(ocvrs_has_module_aruco)]
	use opencv::aruco::DetectorParameters;

	let mut ptr = DetectorParameters::create()?;
	let mut plain = DetectorParameters::default()?;

	assert_eq!(plain.adaptive_thresh_win_size_min(), ptr.adaptive_thresh_win_size_min());
	ptr.set_adaptive_thresh_win_size_min(4);
	assert_eq!(ptr.adaptive_thresh_win_size_min(), 4);

	assert_eq!(plain.adaptive_thresh_win_size_max(), ptr.adaptive_thresh_win_size_max());
	ptr.set_adaptive_thresh_win_size_max(24);
	assert_eq!(ptr.adaptive_thresh_win_size_max(), 24);

	assert_eq!(plain.adaptive_thresh_win_size_step(), ptr.adaptive_thresh_win_size_step());
	ptr.set_adaptive_thresh_win_size_step(11);
	assert_eq!(ptr.adaptive_thresh_win_size_step(), 11);

	assert_eq!(plain.adaptive_thresh_constant(), ptr.adaptive_thresh_constant());
	ptr.set_adaptive_thresh_constant(8.0);
	assert_eq!(ptr.adaptive_thresh_constant(), 8.0);

	assert_eq!(plain.min_marker_perimeter_rate(), ptr.min_marker_perimeter_rate());
	ptr.set_min_marker_perimeter_rate(1.0);
	assert_eq!(ptr.min_marker_perimeter_rate(), 1.0);

	assert_eq!(plain.max_marker_perimeter_rate(), ptr.max_marker_perimeter_rate());
	ptr.set_max_marker_perimeter_rate(5.0);
	assert_eq!(ptr.max_marker_perimeter_rate(), 5.0);

	assert_eq!(plain.min_corner_distance_rate(), ptr.min_corner_distance_rate());
	ptr.set_min_corner_distance_rate(1.0);
	assert_eq!(ptr.min_corner_distance_rate(), 1.0);

	assert_eq!(plain.min_distance_to_border(), ptr.min_distance_to_border());
	ptr.set_min_distance_to_border(4);
	assert_eq!(ptr.min_distance_to_border(), 4);

	assert_eq!(plain.min_marker_distance_rate(), ptr.min_marker_distance_rate());
	ptr.set_min_marker_distance_rate(1.0);
	assert_eq!(ptr.min_marker_distance_rate(), 1.0);

	assert_eq!(plain.corner_refinement_win_size(), ptr.corner_refinement_win_size());
	ptr.set_corner_refinement_win_size(6);
	assert_eq!(ptr.corner_refinement_win_size(), 6);

	assert_eq!(plain.corner_refinement_max_iterations(), ptr.corner_refinement_max_iterations());
	ptr.set_corner_refinement_max_iterations(31);
	assert_eq!(ptr.corner_refinement_max_iterations(), 31);

	assert_eq!(plain.corner_refinement_min_accuracy(), ptr.corner_refinement_min_accuracy());
	ptr.set_corner_refinement_min_accuracy(1.0);
	assert_eq!(ptr.corner_refinement_min_accuracy(), 1.0);

	assert_eq!(plain.marker_border_bits(), ptr.marker_border_bits());
	ptr.set_marker_border_bits(2);
	assert_eq!(ptr.marker_border_bits(), 2);

	assert_eq!(plain.perspective_remove_ignored_margin_per_cell(), ptr.perspective_remove_ignored_margin_per_cell());
	ptr.set_perspective_remove_ignored_margin_per_cell(1.0);
	assert_eq!(ptr.perspective_remove_ignored_margin_per_cell(), 1.0);

	assert_eq!(plain.max_erroneous_bits_in_border_rate(), ptr.max_erroneous_bits_in_border_rate());
	ptr.set_max_erroneous_bits_in_border_rate(1.0);
	assert_eq!(ptr.max_erroneous_bits_in_border_rate(), 1.0);

	assert_eq!(plain.min_otsu_std_dev(), ptr.min_otsu_std_dev());
	ptr.set_min_otsu_std_dev(6.0);
	assert_eq!(ptr.min_otsu_std_dev(), 6.0);

	assert_eq!(plain.error_correction_rate(), ptr.error_correction_rate());
	ptr.set_error_correction_rate(1.0);
	assert_eq!(ptr.error_correction_rate(), 1.0);

	assert_eq!(plain.perspective_remove_pixel_per_cell(), ptr.perspective_remove_pixel_per_cell());
	ptr.set_perspective_remove_pixel_per_cell(5);
	assert_eq!(ptr.perspective_remove_pixel_per_cell(), 5);

	plain.set_adaptive_thresh_constant(123.);
	assert_eq!(123., plain.adaptive_thresh_constant());
	plain.set_adaptive_thresh_constant(87.);
	assert_eq!(87., plain.adaptive_thresh_constant());

	Ok(())
}

/// Test whether the return of the small simple structures works as intended
#[test]
fn simple_struct_return_infallible() -> Result<()> {
	#![cfg(ocvrs_has_module_imgproc)]
	use opencv::{
		core::{Vector, Rect, Point2f, Size2f},
		imgproc,
	};
	/*
	There previously was in issue that return of the small simple structs like Point2f (2 floats, 64 bits in total) was handled
	differently by Rust (v1.57.0) and default compilers in Ubuntu 20.04 (Gcc 9.3.0 & Clang 10.0.0). That mismatch led to
	miscellaneous memory problems like segmentation faults. That shouldn't happen anymore because such returns are now handled by
	the output argument.
	 */
	let contour = Vector::<Point2f>::from_iter(IntoIterator::into_iter([
		Point2f::new(5., 5.),
		Point2f::new(5., 15.),
		Point2f::new(15., 15.),
		Point2f::new(15., 5.),
		Point2f::new(5., 5.),
	]));
	let bound_rect = imgproc::bounding_rect(&contour)?;
	assert_eq!(Rect::new(5, 5, 11, 11), bound_rect);
	let min_area_rect = imgproc::min_area_rect(&contour)?;
	assert_eq!(Point2f::new(10., 10.), min_area_rect.center());
	assert_eq!(Size2f::new(10., 10.), min_area_rect.size());
	// different versions of OpenCV return -90 and 90
	assert_eq!(90., min_area_rect.angle().abs());
	Ok(())
}

// The TrackerSamplerPF_Params no longer exists since OpenCV 4.5.1 and there no other methods that
// accept typed Mat's, so disable the test for now.
// #[test]
// fn typed_mat() -> Result<()> {
// 	#![cfg(ocvrs_has_module_tracking)]
//
// 	use opencv::{tracking::TrackerSamplerPF_Params};
// 	let mut params = TrackerSamplerPF_Params::default()?;
// 	assert_eq!(&[15., 15., 15., 15.], params.std().data_typed()?);
// 	let mat = Mat::new_rows_cols_with_default(1, 4, f64::typ(), Scalar::all(8.))?.try_into_typed::<f64>()?;
// 	let mat_src = mat.try_clone()?.try_into_typed::<f64>()?;
// 	params.set_std(mat);
// 	let mat = params.std();
// 	assert_eq!(mat.size()?, mat_src.size()?);
// 	assert_eq!(mat.data_typed()?, mat_src.data_typed()?);
// 	Ok(())
// }
