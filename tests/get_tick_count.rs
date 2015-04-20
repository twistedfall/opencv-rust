extern crate opencv;
extern crate libc;

use opencv::core;

#[test]
fn test_primitives() {
    let ticks = core::getTickCount().unwrap();
    assert!(ticks > 10000);
    let freq = core::getTickFrequency().unwrap();
    assert!(freq > 1000f64);
    let cpus = core::getNumberOfCPUs().unwrap();
    assert!(cpus > 0);
    core::setUseOptimized(true).unwrap();
    let optims = core::useOptimized().unwrap();
    assert!(optims);
    core::setUseOptimized(false).unwrap();
    let optims = core::useOptimized().unwrap();
    assert!(!optims);
}

#[test]
fn test_return_string() {
    let info = core::getBuildInformation().unwrap();
    assert!(info.contains("\nGeneral configuration for OpenCV"));
}
