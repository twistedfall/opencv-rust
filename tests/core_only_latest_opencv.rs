//! Tests that will not be run in CI on OpenCV 4.2.0, 4.5.4 and 3.4.16 due to missing function
#![cfg(not(ocvrs_opencv_branch_34))]

use opencv::core::{Point2f, RotatedRect, Size2f, Vector};
use opencv::Result;

#[test]
fn rotated_rect_points_vec() -> Result<()> {
	let rect = RotatedRect::new(Point2f::new(100., 100.), Size2f::new(100., 100.), 90.)?;
	let mut vec_pts = Vector::new();
	rect.points_vec(&mut vec_pts)?;
	assert_eq!(Point2f::new(50., 50.), vec_pts.get(0)?);
	assert_eq!(Point2f::new(150., 50.), vec_pts.get(1)?);
	assert_eq!(Point2f::new(150., 150.), vec_pts.get(2)?);
	assert_eq!(Point2f::new(50., 150.), vec_pts.get(3)?);

	Ok(())
}
