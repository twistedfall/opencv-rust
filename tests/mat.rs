use opencv::core;

#[test]
fn mat_for_rows_and_cols() {
    let typ = core::CV_8UC3;
    let mat = core::Mat::new_rows_cols(400, 300, typ).unwrap();
    let size = mat.size().unwrap();
    assert_eq!(
        core::Size {
            width: 300,
            height: 400
        },
        size
    );
    assert_eq!(core::CV_8U, mat.depth().unwrap());
    assert_eq!(3, mat.channels().unwrap());
}

#[test]
fn mat_at() {
    let mut mat = core::Mat::new_rows_cols(100, 100, core::CV_32F).unwrap();
    assert_eq!(*mat.at_2d::<f32>(0, 0).unwrap(), 0.);
    *mat.at_2d_mut::<f32>(0, 0).unwrap() = 1.;
    assert_eq!(*mat.at_2d::<f32>(0, 0).unwrap(), 1.);

    if let Ok(..) = mat.at::<i32>(0) {
        assert!(false, "different types");
    }

    let row = mat.at_row::<f32>(0).unwrap();
    assert_eq!(row.len(), 100);
    assert_eq!(row[0], 1.);

    let row = mat.at_row_mut::<f32>(1).unwrap();
    row[0..4].copy_from_slice(&[10., 20., 30., 40.]);

    let data = mat.data::<f32>().unwrap();
    assert_eq!(data[0], 1.);
    assert_eq!(data[100], 10.);
    assert_eq!(data[103], 40.);

    // todo unallocated Mat, zero sized Mat
}

#[test]
fn mat_vec() {
    let s = vec![
        vec![1.0f32, 2., 3.],
        vec![4., 5., 6.],
        vec![7., 8., 9.],
    ];

    let mat = core::Mat::from_slice_2d(&s).unwrap();
    assert_eq!(mat.size().unwrap(), core::Size { width: 3, height: 3 });
    assert_eq!(*mat.at_2d::<f32>(1, 1).unwrap(), 5.);

    let v = mat.to_vec_2d::<f32>().unwrap();
    assert_eq!(s, v);
}
