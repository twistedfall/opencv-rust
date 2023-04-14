use matches::assert_matches;

use opencv::core;
use opencv::core::{Matx22d, Matx23f, Matx32f, Matx33d, Matx66f, Scalar};
use opencv::prelude::*;
use opencv::Result;

#[test]
fn matx_get() {
	let mut a = Matx33d::eye();
	assert_eq!(a[(0, 0)], 1.);
	assert_eq!(a[(1, 0)], 0.);
	assert_eq!(*a.get((0, 1)).unwrap(), 0.);
	assert_eq!(*a.get_mut((1, 1)).unwrap(), 1.);
	assert_matches!(a.get((3, 1)), None);
	assert_matches!(a.get_mut((1, 3)), None);
	assert_matches!(a.get((3, 3)), None);
}

#[test]
fn matx_set() {
	let mut a = Matx33d::eye();
	*a.get_mut((0, 1)).unwrap() = 2.;
	a[(1, 2)] = 3.;
	assert_eq!(a[(0, 1)], 2.);
	assert_eq!(a[(1, 2)], 3.);
	assert_eq!(a[(0, 0)], 1.);
	assert_eq!(a[(1, 0)], 0.);
	let mut a = Matx66f::eye();
	*a.get_mut((0, 1)).unwrap() = 2.;
	a[(1, 2)] = 3.;
	assert_eq!(a[(0, 1)], 2.);
	assert_eq!(a[(1, 2)], 3.);
	assert_eq!(a[(0, 0)], 1.);
	assert_eq!(a[(1, 0)], 0.);
}

#[cfg(all(ocvrs_opencv_branch_4, not(target_env = "msvc")))]
#[test]
fn matx_return() -> Result<()> {
	use opencv::core::Point2f;
	use opencv::imgproc;

	let mat = imgproc::get_rotation_matrix_2d_matx(Point2f::new(10., 10.), 90., 2.)?;
	assert_eq!(2, mat.rows());
	assert_eq!(3, mat.cols());
	assert_eq!(mat[(0, 0)], mat[(1, 1)]);
	assert_eq!(-mat[(0, 1)], mat[(1, 0)]);
	Ok(())
}

#[cfg(ocvrs_has_module_surface_matching)]
#[test]
fn matx_arg() -> Result<()> {
	use opencv::{core::Matx44d, surface_matching::Pose3D};

	let mut pose = Pose3D::default()?;
	assert!(&pose.pose().val.iter().all(|&x| x == 0.));
	pose.set_pose(Matx44d::all(9.));
	assert!(&pose.pose().val.iter().all(|&x| x == 9.));
	Ok(())
}

#[test]
fn matx_input_array() -> Result<()> {
	assert_eq!(Scalar::from(2.), core::sum_elems(&Matx32f::eye())?);
	assert_eq!(Scalar::from(2.), core::sum_elems(&Matx23f::eye())?);
	Ok(())
}

#[test]
fn matx_input_output_array() -> Result<()> {
	let mut mat = Matx33d::from_array([1., 2., 3., 4., 5., 6., 9., 8., 9.]);
	core::complete_symm(&mut mat, false)?;
	let expected = Matx33d::from_array([1., 2., 3., 2., 5., 6., 3., 6., 9.]);
	assert_eq!(expected, mat);
	Ok(())
}

#[test]
fn matx_default() -> Result<()> {
	let mat = Matx22d::default();
	assert_eq!(mat[(1, 1)], f64::default());
	let mat = Matx66f::default();
	assert_eq!(mat[(1, 1)], f32::default());
	Ok(())
}

#[test]
fn matx_all() -> Result<()> {
	let mat = Matx22d::all(9.);
	assert_eq!(mat[(0, 1)], 9.);
	let mat = Matx66f::all(81.);
	assert_eq!(mat[(3, 4)], 81.);
	Ok(())
}
