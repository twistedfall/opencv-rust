use std::sync::{Arc, Mutex};

use opencv::{highgui, Result};

#[test]
fn callback() -> Result<()> {
    // only run under X11 on linux
    if cfg!(target_os = "linux") && option_env!("DISPLAY").is_some() {
        {
            highgui::named_window("test_1", 0)?;
            let mut value = 50;
            let cb_value = Arc::new(Mutex::new(0));
            highgui::create_trackbar("test_track_1", "test_1", Some(&mut value), 100, Some(Box::new({
                let cb_value = cb_value.clone();
                move |s| {
                    *cb_value.lock().unwrap() = s;
                }
            })))?;
            assert_eq!(value, 50);
            highgui::set_trackbar_pos("test_track_1", "test_1", 10)?;
            assert_eq!(value, 10);
            assert_eq!(*cb_value.lock().unwrap(), 10);
        }

        {
            highgui::named_window("test_2", 0)?;
            let cb_value = Arc::new(Mutex::new(0));
            highgui::create_trackbar("test_track_2", "test_2", None, 100, Some(Box::new({
                let cb_value = cb_value.clone();
                move |s| {
                    *cb_value.lock().unwrap() = s;
                }
            })))?;
            highgui::set_trackbar_pos("test_track_2", "test_2", 10)?;
            assert_eq!(*cb_value.lock().unwrap(), 10);
        }
    }
    Ok(())
}
