extern crate opencv_sys;

#[test]
fn test_get_tick_count() {
    let ticks = unsafe { opencv_sys::cv_getTickCount() };
    assert!(ticks > 10000);
}
