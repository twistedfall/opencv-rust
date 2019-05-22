use std::mem::transmute;

use opencv::core::{DataType, Mat, Scalar};

#[test]
fn layout() {
    let mat = Mat::new_rows_cols_with_default(1, 3, f32::typ(), Scalar::all(10.)).unwrap();
    let mat_ref: &mut Mat = unsafe { transmute(&mut mat.as_raw_Mat()) };
    assert_eq!(mat.size().unwrap(), mat_ref.size().unwrap());
    assert_eq!(mat.typ().unwrap(), mat_ref.typ().unwrap());
    assert_eq!(mat.rows().unwrap(), mat_ref.rows().unwrap());
    assert_eq!(mat.cols().unwrap(), mat_ref.cols().unwrap());
    assert_eq!(mat.at_2d::<f32>(0, 1).unwrap(), mat_ref.at_2d::<f32>(0, 1).unwrap());
}
