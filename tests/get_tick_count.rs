use opencv::core;

#[test]
fn test_primitives() {
    assert!(core::get_tick_count().unwrap() > 10000);
    assert!(core::get_cpu_tick_count().unwrap() > 10000);
    assert!(core::get_tick_frequency().unwrap() > 1000.);
    assert!(core::get_number_of_cpus().unwrap() >= 1);
    core::set_use_optimized(true).unwrap();
    assert!(core::use_optimized().unwrap());
    core::set_use_optimized(false).unwrap();
    assert!(!core::use_optimized().unwrap());
}

#[test]
fn test_return_string() {
    let info = core::get_build_information().unwrap();
    assert!(info.contains("\nGeneral configuration for OpenCV"));
}
