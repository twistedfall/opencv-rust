use opencv::{
    core,
    core::Mat,
    imgproc,
    types
};

#[test]
fn simple_struct() {
    let res = imgproc::get_structuring_element(imgproc::CV_SHAPE_CROSS, core::Size { width: 100, height: 100 }, core::Point { x: 50, y: 50 }).unwrap();
    assert_eq!(res.typ().unwrap(), 0);
    let size = res.size().unwrap();
    assert_eq!(size.width, 100);
    assert_eq!(size.height, 100);
}

#[test]
fn vec() {
    let mut m = Mat::new_rows_cols_with_default(10, 10, core::CV_8UC3, core::Scalar::default()).unwrap();
    let mut ps = types::VectorOfMat::new();
    assert_eq!(ps.len(), 0);
    let mut p1 = Mat::new_rows_cols(3, 2, core::CV_32S).unwrap();
    p1.at_row_mut::<i32>(0).unwrap().copy_from_slice(&[0, 0]);
    p1.at_row_mut::<i32>(1).unwrap().copy_from_slice(&[0, 9]);
    p1.at_row_mut::<i32>(2).unwrap().copy_from_slice(&[9, 9]);
    ps.push(p1);
    assert_eq!(ps.len(), 1);
    imgproc::fill_poly(&mut m, &ps, core::Scalar::new(127., 127., 127., 0.), core::LINE_8, 0, core::Point::default()).unwrap();
    assert_eq!(*m.at_2d::<core::Vec3b>(0, 0).unwrap(), core::Vec3b::from([127, 127, 127]));
    assert_eq!(*m.at_2d::<core::Vec3b>(0, 9).unwrap(), core::Vec3b::default());
    assert_eq!(*m.at_2d::<core::Vec3b>(9, 9).unwrap(), core::Vec3b::from([127, 127, 127]));
}

#[test]
fn scalar() {
    let mut m = Mat::new_rows_cols_with_default(1, 3, core::CV_8U, core::Scalar::new(2., 0., 0., 0.)).unwrap();
    let sum = core::sum(&m).unwrap();
    assert_eq!(sum[0], 6.);
    let s = m.at_row_mut::<u8>(0).unwrap();
    s[0] = 1;
    s[1] = 2;
    s[2] = 3;
    let sum = core::sum(&m).unwrap();
    assert_eq!(sum[0], 6.);
}
