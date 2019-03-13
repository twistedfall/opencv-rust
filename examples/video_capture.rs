extern crate opencv;

use opencv::core;
use opencv::highgui;
use opencv::videoio;

fn run() -> Result<(), String> {
    let window = "video capture";
    highgui::named_window(window, 1)?;
    let mut cam = videoio::VideoCapture::index(1)?;
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        println!("Using different camera");
        cam = videoio::VideoCapture::index(0)?;
    }
    loop {
        let mut frame = core::Mat::new()?;
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            highgui::imshow(window, &mut frame)?;
        }
        if highgui::wait_key(10)? > 0 {
            break;
        }
    }
    Ok(())
}

fn main() {
    run().unwrap()
}
