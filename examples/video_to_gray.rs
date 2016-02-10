extern crate opencv;

use opencv::core;
use opencv::imgproc;
use opencv::highgui;
use opencv::videoio;

fn run() -> Result<(), String> {
    let window = "video capture";
    try!(highgui::named_window(window, 1));
    let mut cam = try!(videoio::VideoCapture::index(1));
    let opened = try!(videoio::VideoCapture::is_opened(&cam));
    if !opened {
        println!("Using different camera");
        cam = try!(videoio::VideoCapture::index(0));
    }
    loop {
        let mut frame = try!(core::Mat::new());
        try!(cam.read(&mut frame));
        if try!(frame.size()).width > 0 {
            let mut gray = try!(core::Mat::new());
            try!(imgproc::cvt_color(
                &frame,
                &mut gray,
                imgproc::COLOR_BGR2GRAY,
                0
            ));
            try!(highgui::imshow(window, &gray));
        }
        if try!(highgui::wait_key(10)) > 0 {
            break;
        }
    }
    Ok(())
}

fn main() {
    run().unwrap()
}
