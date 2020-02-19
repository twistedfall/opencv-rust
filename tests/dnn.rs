#![cfg(not(feature = "opencv-32"))]

use matches::assert_matches;

use opencv::{
	core,
	dnn::{DictTrait, DictValue, LayerTrait, LayerParams, Net},
	Error,
	prelude::*,
	Result,
	types::VectorOfMat,
};

#[cfg(all(feature = "opencv-4", not(target_env = "msvc")))]
use opencv::dnn::NetTrait;

#[test]
fn net() -> Result<()> {
	let mut net = Net::default()?;
	assert!(net.empty()?);
	net.enable_fusion(false)?;
	let mut params = LayerParams::default()?;
	assert_eq!(params.name(), "");
	assert_eq!(params.typ(), "");
	params.set_name("param name");
	params.set_type("param type");
	assert_eq!(params.name(), "param name");
	assert_eq!(params.typ(), "param type");
	assert!(!params.has("test")?);
	params.set("test", &DictValue::from_f64(345.9)?)?;
	assert!(params.has("test")?);
	let v = params.get("test")?;
	assert_eq!(345.9, v.get_real_value(-1)?);
	params.set_f64("test", &98.71)?;
	assert_eq!(98.71, params.get("test")?.get_real_value(-1)?);
	params.set("test_str", &DictValue::from_str("test string".to_string().as_str())?)?;
	assert_eq!("test string", params.get("test_str")?.get_string_value(-1)?);
	let ret = params.set_str("test_str", "new test string".to_string().as_str())?;
	assert_eq!("new test string", ret);
	assert_eq!("new test string", params.get("test_str")?.get_string_value(-1)?);
	let res = net.add_layer("layer", "type", &mut params)?;
	assert_ne!(-1, res);
	assert!(!net.empty()?);
	let mut blobs = VectorOfMat::new();
	blobs.push(Mat::default()?);
	params.set_blobs(blobs);
	let blobs = params.blobs();
	assert_eq!(1, blobs.len());
	Ok(())
}

#[test]
fn layer() -> Result<()> {
	use opencv::dnn::CropAndResizeLayer;
	let mut params = LayerParams::default()?;
	params.set("width", &DictValue::from_i32(32)?)?;
	params.set("height", &DictValue::from_i32(32)?)?;
	let layer = CropAndResizeLayer::create(&params)?;
	assert_eq!(0, layer.preferable_target());
	Ok(())
}

#[test]
fn dict() -> Result<()> {
	{
		let v = DictValue::from_f64(123.456)?;
		assert!(v.is_real()?);
		assert!(!v.is_int()?);
		assert!(!v.is_string()?);
		assert_eq!(1, v.size()?);
		assert_matches!(v.get_i64(-1), Err(Error{code: core::StsAssert, ..}));
		assert_eq!(123.456, v.get_f64(-1)?);
		assert_matches!(v.get_int_value(-1), Err(Error{code: core::StsAssert, ..}));
	}

	{
		let v = DictValue::from_i64(123)?;
		assert!(v.is_int()?);
		assert!(v.is_real()?);
		assert!(!v.is_string()?);
		assert_eq!(1, v.size()?);
		assert_eq!(123, v.get_i64(-1)?);
		assert_eq!(123, v.get_i32(-1)?);
		assert_eq!(123., v.get_f64(-1)?);
		assert_matches!(v.get_string_value(-1), Err(Error{code: core::StsAssert, ..}));
	}

	{
		let v = DictValue::from_str("876543.123")?;
		assert!(!v.is_int()?);
		assert!(!v.is_real()?);
		assert!(v.is_string()?);
		assert_eq!(1, v.size()?);
		assert_eq!(876543, v.get_i64(-1)?);
		assert_eq!(876543.123, v.get_real_value(-1)?);
		assert_eq!("876543.123", v.get_string_value(-1)?);
	}
	Ok(())
}
