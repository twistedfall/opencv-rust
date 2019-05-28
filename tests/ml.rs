use opencv::{
    core::{DataType, Mat, Scalar, Size},
    ml::{self, KNearest, StatModel},
    types::PtrOfKNearest,
};

#[test]
fn knn() {
    let mut knn: PtrOfKNearest = KNearest::create().unwrap();
    assert!(knn.empty().unwrap());
    let samp = Mat::new_rows_cols_with_default(1, 1, f32::typ(), Scalar::all(1.)).unwrap();
    let resp = Mat::new_rows_cols_with_default(1, 1, f32::typ(), Scalar::all(2.)).unwrap();
    knn.train(&samp, ml::ROW_SAMPLE, &resp).unwrap();
    let mut resp = Mat::default();
    let mut neigh = Mat::default();
    let mut dist = Mat::default();
    knn.find_nearest(&samp, 3, &mut resp, &mut neigh, &mut dist).unwrap();
    assert_eq!(2., *resp.at_2d::<f32>(0, 0).unwrap());
    assert_eq!(Size::new(1, 1), resp.size().unwrap());
    assert_eq!(2., *neigh.at_2d::<f32>(0, 0).unwrap());
    assert_eq!(Size::new(1, 1), neigh.size().unwrap());
    assert_eq!(0., *dist.at_2d::<f32>(0, 0).unwrap());
    assert_eq!(Size::new(1, 1), dist.size().unwrap());
}
