extern crate opencv;

use opencv::core;
use opencv::highgui;

fn run() -> Result<(),String> {
    let window = "video capture";
    try!(highgui::named_window(window,1));
    let mut cam = try!(highgui::VideoCapture::device(1));
    let opened = try!(highgui::VideoCapture::is_opened(&cam));
    if ! opened {
        println!("Using different camera");
        cam = try!(highgui::VideoCapture::device(0));
    }
    loop {
        let mut frame = try!(core::Mat::new());
        try!(cam.read(&mut frame));
        if try!(frame.size()).width > 0 {
            try!(highgui::imshow(window, &mut frame));
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
