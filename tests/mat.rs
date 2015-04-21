extern crate opencv;

use opencv::core;

#[test]
fn main() {
    let mut mat = core::Mat::for_rows_and_cols(400,300, 3).unwrap();
    let size = mat.size().unwrap();
    assert_eq!(core::Size { width: 300, height: 400 }, size);
}
