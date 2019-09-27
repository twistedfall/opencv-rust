use std::sync::{Arc, Mutex};

use opencv::{highgui, Result};

#[test]
fn callback() -> Result<()> {
    // only run under X11 on linux
    if cfg!(target_os = "linux") && option_env!("DISPLAY").is_some() {
        highgui::named_window("test", 0)?;
        let mut value = 50;
        let cb_value = Arc::new(Mutex::new(0));
        highgui::create_trackbar("test_track", "test", &mut value, 100, Some(Box::new({
            let cb_value = cb_value.clone();
            move |s| {
                *cb_value.lock().unwrap() = s;
            }
        })))?;
        highgui::set_trackbar_pos("test_track", "test", 10)?;
        assert_eq!(value, 10);
        assert_eq!(*cb_value.lock().unwrap(), 10);
    }
    Ok(())
}
