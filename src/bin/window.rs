extern crate opencv_sys;

use opencv_sys::*;

use std::ffi::CString;

fn main() {
    unsafe {
        let name = CString::new("hello opencv!").unwrap();
        let filename = CString::new("jarres.jpg").unwrap();
        let image = cv_highgui_imread_SI(filename.as_ptr(), 0);
        cv_highgui_namedWindow_SI(name.as_ptr(), 0);
        cv_highgui_imshow_SM(name.as_ptr(), image);
        let _ = cv_highgui_waitKey_I(10000);
    }
}
