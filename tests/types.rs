use opencv::{
    core::{self, DataType, Mat, Point, Scalar, Size},
    imgproc,
    types,
};
use opencv::core::Vec3b;

#[test]
fn simple_struct() {
    let res = imgproc::get_structuring_element(imgproc::MORPH_CROSS, Size { width: 100, height: 100 }, Point { x: 50, y: 50 }).unwrap();
    assert_eq!(res.typ().unwrap(), 0);
    let size = res.size().unwrap();
    assert_eq!(size.width, 100);
    assert_eq!(size.height, 100);
}

#[test]
fn vec() {
    let mut m = Mat::new_rows_cols_with_default(10, 10, Vec3b::typ(), Scalar::default()).unwrap();
    let mut ps = types::VectorOfMat::new();
    assert_eq!(ps.len(), 0);
    let mut p1 = unsafe { Mat::new_rows_cols(3, 2, i32::typ()) }.unwrap();
    p1.at_row_mut::<i32>(0).unwrap().copy_from_slice(&[0, 0]);
    p1.at_row_mut::<i32>(1).unwrap().copy_from_slice(&[0, 9]);
    p1.at_row_mut::<i32>(2).unwrap().copy_from_slice(&[9, 9]);
    ps.push(p1);
    assert_eq!(ps.len(), 1);
    imgproc::fill_poly(&mut m, &ps, Scalar::new(127., 127., 127., 0.), core::LINE_8, 0, Point::default()).unwrap();
    assert_eq!(*m.at_2d::<core::Vec3b>(0, 0).unwrap(), core::Vec3b::from([127, 127, 127]));
    assert_eq!(*m.at_2d::<core::Vec3b>(0, 9).unwrap(), core::Vec3b::default());
    assert_eq!(*m.at_2d::<core::Vec3b>(9, 9).unwrap(), core::Vec3b::from([127, 127, 127]));
}

#[test]
fn scalar() {
    let mut m = Mat::new_rows_cols_with_default(1, 3, u8::typ(), Scalar::new(2., 0., 0., 0.)).unwrap();
    let sum = core::sum(&m).unwrap();
    assert_eq!(sum[0], 6.);
    let s = m.at_row_mut::<u8>(0).unwrap();
    s[0] = 1;
    s[1] = 2;
    s[2] = 3;
    let sum = core::sum(&m).unwrap();
    assert_eq!(sum[0], 6.);
}
