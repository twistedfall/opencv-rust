extern crate opencv as cv;

fn run() -> Result<(),String> {
    let window = "video capture";
    try!(cv::named_window(window,1));
    let mut cam = try!(cv::VideoCapture::device(1));
    loop {
        let mut frame = try!(cv::Mat::new());
        try!(cam.read(&mut frame));
        if try!(frame.size()).width > 0 {
            try!(cv::imshow(window, &mut frame));
        }
        if try!(cv::wait_key(10)) > 0 {
            break;
        }
    }
    Ok(())
}

fn main() {
    run().unwrap()
}
