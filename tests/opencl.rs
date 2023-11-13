use opencv::{core, Result};

#[test]
fn convert_type_str() -> Result<()> {
	if core::have_opencl()? {
		// this function writes to buf argument and returns it
		let mut test = "test".to_string();
		assert_eq!("noconvert", core::convert_type_str(1, 1, 1, &mut test)?);
		assert_eq!("", test);
		assert_eq!("convert_ushort_sat", core::convert_type_str(1, 2, 1, &mut test)?);
		assert_eq!("convert_ushort_sat", test);
	}
	Ok(())
}
