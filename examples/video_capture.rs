extern crate opencv;

use opencv::core;
use opencv::highgui;

fn run() -> Result<(),String> {
    let window = "video capture";
    try!(highgui::namedWindow(window,1));
    let mut cam = try!(highgui::VideoCapture::for_device(0));
    loop {
        let mut frame = core::mat();
        try!(cam.read(&mut frame));
        if try!(frame.size()).width > 0 {
            try!(highgui::imshow(window, &mut frame));
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
