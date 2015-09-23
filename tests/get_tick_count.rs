extern crate opencv;
extern crate libc;

#[test]
fn test_primitives() {
    let ticks = opencv::get_tick_count().unwrap();
    assert!(ticks > 10000);
    let freq = opencv::get_tick_frequency().unwrap();
    assert!(freq > 1000f64);
    let cpus = opencv::get_number_of_cp_us().unwrap();
    assert!(cpus > 0);
    opencv::set_use_optimized(true).unwrap();
    let optims = opencv::use_optimized().unwrap();
    assert!(optims);
    opencv::set_use_optimized(false).unwrap();
    let optims = opencv::use_optimized().unwrap();
    assert!(!optims);
}

#[test]
fn test_return_string() {
    let info = opencv::get_build_information().unwrap();
    assert!(info.contains("\nGeneral configuration for OpenCV"));
}
