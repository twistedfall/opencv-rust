use matches::assert_matches;
use opencv::core::BorderTypes;
use opencv::Result;

#[test]
fn enum_from() -> Result<()> {
	assert_eq!(BorderTypes::BORDER_CONSTANT, BorderTypes::try_from(0)?);
	assert_eq!(BorderTypes::BORDER_WRAP, BorderTypes::try_from(3)?);
	assert_matches!(
		BorderTypes::try_from(10),
		Err(opencv::Error {
			code: opencv::core::StsBadArg,
			..
		})
	);
	Ok(())
}

#[test]
fn enum_into() -> Result<()> {
	assert_eq!(0, BorderTypes::BORDER_CONSTANT.into());
	assert_eq!(3, BorderTypes::BORDER_WRAP.into());
	Ok(())
}
