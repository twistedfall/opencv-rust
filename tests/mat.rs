extern crate opencv;

use opencv::core;

#[test]
fn mat_for_rows_and_cols() {
    let typ = core::make_type(core::ChannelDepth::CV_8U, 3);
    let mat = core::Mat::for_rows_and_cols(400,300,typ).unwrap();
    let size = mat.size().unwrap();
    assert_eq!(core::Size { width: 300, height: 400 }, size);
    assert_eq!(core::ChannelDepth::CV_8U as u32, mat.depth().unwrap());
    assert_eq!(3, mat.channels().unwrap());
}
