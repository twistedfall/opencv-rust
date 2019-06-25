use opencv::{
    core::{self, Point, Scalar, Size, Vec3b},
    imgproc,
    prelude::*,
    Result,
    types,
};

#[test]
fn simple_struct() -> Result<()> {
    let res = imgproc::get_structuring_element(imgproc::MORPH_CROSS, Size { width: 100, height: 100 }, Point { x: 50, y: 50 })?;
    assert_eq!(res.typ()?, 0);
    let size = res.size()?;
    assert_eq!(size.width, 100);
    assert_eq!(size.height, 100);
    Ok(())
}

#[test]
fn vec() -> Result<()> {
    let mut m = Mat::new_rows_cols_with_default(10, 10, Vec3b::typ(), Scalar::default())?;
    let mut ps = types::VectorOfMat::new();
    assert_eq!(ps.len(), 0);
    let mut p1 = unsafe { Mat::new_rows_cols(3, 2, i32::typ()) }?;
    p1.at_row_mut::<i32>(0)?.copy_from_slice(&[0, 0]);
    p1.at_row_mut::<i32>(1)?.copy_from_slice(&[0, 9]);
    p1.at_row_mut::<i32>(2)?.copy_from_slice(&[9, 9]);
    ps.push(p1);
    assert_eq!(ps.len(), 1);
    #[cfg(not(feature = "opencv-41"))]
    use self::core::LINE_8;
    #[cfg(feature = "opencv-41")]
    use self::imgproc::LINE_8;
    imgproc::fill_poly(&mut m, &ps, Scalar::new(127., 127., 127., 0.), LINE_8, 0, Point::default())?;
    assert_eq!(*m.at_2d::<Vec3b>(0, 0)?, Vec3b::from([127, 127, 127]));
    assert_eq!(*m.at_2d::<Vec3b>(0, 9)?, Vec3b::default());
    assert_eq!(*m.at_2d::<Vec3b>(9, 9)?, Vec3b::from([127, 127, 127]));
    Ok(())
}

#[test]
fn scalar() -> Result<()> {
    let mut m = Mat::new_rows_cols_with_default(1, 3, u8::typ(), Scalar::new(2., 0., 0., 0.))?;
    let sum = core::sum(&m)?;
    assert_eq!(sum[0], 6.);
    let s = m.at_row_mut::<u8>(0)?;
    s[0] = 1;
    s[1] = 2;
    s[2] = 3;
    let sum = core::sum(&m)?;
    assert_eq!(sum[0], 6.);
    Ok(())
}
