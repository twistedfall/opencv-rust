extern crate opencv;

use opencv::core;
use opencv::highgui;
use opencv::imgproc;

fn run() -> Result<(),String> {
    let window = "video capture";
    try!(highgui::namedWindow(window,1));
    let mut cam = try!(highgui::VideoCapture::for_fd(0));
    loop {
        let mut frame = try!(core::Mat::new());
        try!(cam.read(&mut frame));
        if try!(frame.size()).width > 0 {
            let mut gray = try!(core::Mat::new());
            try!(imgproc::cvtColor(&frame, &mut gray, imgproc::CV_BGR2GRAY, 0));
            try!(highgui::imshow(window, &gray));
        }
        if try!(highgui::waitKey(10)) > 0 {
            break;
        }
    }
    Ok(())
}

fn main() {
    run().unwrap()
}
