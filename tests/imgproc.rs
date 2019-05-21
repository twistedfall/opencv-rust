use opencv::{
    core::{
        DataType,
        Mat,
        Scalar,
        Vec2f,
    },
    imgproc,
};
use opencv::core::Point2f;

#[test]
fn min_enclosing() {
    let mut pts = Mat::new_rows_cols_with_default(1, 2, Vec2f::typ(), Scalar::default()).unwrap();
    let points = pts.at_row_mut::<Vec2f>(0).unwrap();
    points[0].copy_from_slice(&[10., 10.]);
    points[1].copy_from_slice(&[20., 10.]);

    let mut center = Point2f::default();
    let mut radius = 0.;
    imgproc::min_enclosing_circle(&pts, &mut center, &mut radius).unwrap();
    assert_eq!(radius, 5.0001);
    assert_eq!(center, Point2f::new(15., 10.))
}
