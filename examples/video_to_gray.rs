extern crate opencv;

use opencv::core;
use opencv::highgui;
use opencv::imgproc;
use opencv::videoio;

fn run() -> opencv::Result<()> {
    let window = "video capture";
    highgui::named_window(window, 1)?;
    #[cfg(feature = "opencv-32")]
    let mut cam = videoio::VideoCapture::new(0)?;  // 0 is the default camera
    #[cfg(not(feature = "opencv-32"))]
    let mut cam = videoio::VideoCapture::new_with_backend(0, videoio::CAP_ANY)?;  // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }
    loop {
        let mut frame = core::Mat::new()?;
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            let mut gray = core::Mat::new()?;
            imgproc::cvt_color(
                &frame,
                &mut gray,
                imgproc::COLOR_BGR2GRAY,
                0
            )?;
            highgui::imshow(window, &gray)?;
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
