extern crate opencv;
extern crate libc;

use opencv::core;

#[test]
fn test_primitives() {
    let ticks = core::getTickCount();
    assert!(ticks > 10000);
    let freq = core::getTickFrequency();
    assert!(freq > 1000f64);
    let cpus = core::getNumberOfCPUs();
    assert!(cpus > 0);
    core::setUseOptimized(true);
    let optims = core::useOptimized();
    assert!(optims);
    core::setUseOptimized(false);
    let optims = core::useOptimized();
    assert!(!optims);
}

#[test]
fn test_return_string() {
    let info = core::getBuildInformation();
    assert!(info.contains("\nGeneral configuration for OpenCV"));
}
