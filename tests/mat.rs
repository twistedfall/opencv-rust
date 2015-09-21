extern crate opencv;

use opencv::core;

#[test]
fn mat_for_rows_and_cols() {
    let typ = core::CV_8UC3;
    let mat = core::Mat::new_rows_cols(400,300,typ).unwrap();
    let size = mat.size().unwrap();
    assert_eq!(core::Size { width: 300, height: 400 }, size);
    assert_eq!(core::CV_8U, mat.depth().unwrap());
    assert_eq!(3, mat.channels().unwrap());
}
