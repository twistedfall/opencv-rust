#![cfg(ocvrs_has_module_imgproc)]

use matches::assert_matches;
use opencv::{core, Error, Result};

#[test]
fn fourcc() -> Result<()> {
	#![cfg(ocvrs_has_module_videoio)]
	use opencv::videoio::VideoWriter;

	let fourcc = VideoWriter::fourcc('a', 'v', 'c', '1')?;
	assert_eq!(fourcc, 0x31637661);
	let fourcc_error = VideoWriter::fourcc('😀', 'v', 'c', '1');
	assert_matches!(
		fourcc_error,
		Err(Error {
			code: core::StsBadArg,
			..
		})
	);
	Ok(())
}
