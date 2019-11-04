use opencv::{
    core::{self, CV_32S, CV_32SC3, CV_64F, CV_64FC1, CV_8U, CV_8UC2, MAKETYPE, Moments, Point2f, RotatedRect, Scalar, Size2f},
    prelude::*,
    Result,
    types::VectorOfMat,
};

#[test]
fn make_type() {
    assert_eq!(MAKETYPE(CV_8U, 2), CV_8UC2);
    assert_eq!(MAKETYPE(CV_32S, 3), CV_32SC3);
    assert_eq!(MAKETYPE(CV_64F, 1), CV_64FC1);
}

#[test]
fn moments() -> Result<()> {
    let moments = Moments::default()?;
    assert_eq!(0., moments.m00);
    assert_eq!(0., moments.m12);
    assert_eq!(0., moments.mu30);
    Ok(())
}

#[test]
#[cfg(not(feature = "opencv-32"))]
fn cpu_features_line() -> Result<()> {
    let cpu_feats = core::get_cpu_features_line()?;
    assert!(cpu_feats.is_ascii());
    Ok(())
}

#[test]
fn rotated_rect() -> Result<()> {
    let rect = RotatedRect::new(Point2f::new(100., 100.), Size2f::new(100., 100.), 90.)?;
    let mut pts = [Point2f::default(); 4];
    rect.points(&mut pts)?;
    assert_eq!(Point2f::new(50., 50.), pts[0]);
    assert_eq!(Point2f::new(150., 50.), pts[1]);
    assert_eq!(Point2f::new(150., 150.), pts[2]);
    assert_eq!(Point2f::new(50., 150.), pts[3]);
    Ok(())
}

#[test]
fn in_range() -> Result<()> {
    let mut cs = VectorOfMat::new();
    cs.push(Mat::from_slice_2d(&[
        &[1., 2., 3.],
        &[4., 5., 6.],
        &[7., 8., 9.],
    ])?);
    cs.push(Mat::from_slice_2d(&[
        &[11., 12., 13.],
        &[14., 15., 16.],
        &[17., 18., 19.],
    ])?);
    let mut m = Mat::default()?;
    core::merge(&cs, &mut m)?;
    let mut out = Mat::default()?;
    core::in_range(&m, &Scalar::new(2., 10., 0., 0.), &Scalar::new(6., 15., 0., 0.), &mut out)?;
    assert_eq!(&[0, 255, 255, 255, 255, 0, 0, 0, 0], &out.data_typed::<u8>()?);
    Ok(())
}
