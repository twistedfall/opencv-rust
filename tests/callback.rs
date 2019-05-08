use std::sync::{Arc, Mutex};

use opencv::highgui;

#[test]
fn callback() {
    #[cfg(target_os = "linux")]
    {
        // only run under X11 if on linux
        if std::env::var("DISPLAY").is_err() {
            return;
        }
    }
    highgui::named_window("test", 0).unwrap();
    let mut value = 50;
    let cb_value = Arc::new(Mutex::new(0));
    highgui::create_trackbar("test_track", "test", &mut value, 100, Some(Box::new({
        let cb_value = cb_value.clone();
        move |s| {
            *cb_value.lock().unwrap() = s;
        }
    }))).unwrap();
    highgui::set_trackbar_pos("test_track", "test", 10).unwrap();
    assert_eq!(value, 10);
    assert_eq!(*cb_value.lock().unwrap(), 10);
}

