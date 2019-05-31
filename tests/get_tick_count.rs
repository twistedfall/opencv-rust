use opencv::{core, Result};

#[test]
fn test_primitives() -> Result<()> {
    assert!(core::get_tick_count()? > 10000);
    assert!(core::get_cpu_tick_count()? > 10000);
    assert!(core::get_tick_frequency()? > 1000.);
    assert!(core::get_number_of_cpus()? >= 1);
    core::set_use_optimized(true)?;
    assert!(core::use_optimized()?);
    core::set_use_optimized(false)?;
    assert!(!core::use_optimized()?);
    Ok(())
}

#[test]
fn test_return_string() -> Result<()> {
    let info = core::get_build_information()?;
    assert!(info.contains("\nGeneral configuration for OpenCV"));
    Ok(())
}
