#[cfg(not(feature = "opencv-32"))]
use matches::assert_matches;

#[cfg(not(feature = "opencv-32"))]
use opencv::{
    core,
    dnn::{DictTrait, DictValue, LayerTrait, LayerParams, Net},
    Error,
    prelude::*,
    Result,
    types::VectorOfMat,
};

#[cfg(all(feature = "opencv-41", not(target_env = "msvc")))]
use opencv::dnn::NetTrait;

#[test]
#[cfg(not(feature = "opencv-32"))]
fn net() -> Result<()> {
    let mut net = Net::default()?;
    assert!(net.empty()?);
    net.enable_fusion(false)?;
    let mut params = LayerParams::default()?;
    assert_eq!(params.name()?, "");
    assert_eq!(params._type()?, "");
    params.set_name("param name")?;
    params.set_type("param type")?;
    assert_eq!(params.name()?, "param name");
    assert_eq!(params._type()?, "param type");
    assert!(!params.has("test")?);
    params.set("test", &mut DictValue::from_f64(345.9)?)?;
    assert!(params.has("test")?);
    let v = params.get("test")?;
    assert_eq!(345.9, v.get_f64(-1)?);
    let res = net.add_layer("layer", "type", &mut params)?;
    assert_ne!(-1, res);
    assert!(!net.empty()?);
    let mut blobs = VectorOfMat::new();
    blobs.push(Mat::default()?);
    params.set_blobs(blobs)?;
    let blobs = params.blobs()?;
    assert_eq!(1, blobs.len());
    Ok(())
}

#[test]
#[cfg(not(feature = "opencv-32"))]
fn layer() -> Result<()> {
    use opencv::dnn::CropAndResizeLayer;
    let mut params = LayerParams::default()?;
    params.set("width", &mut DictValue::from_i32(32)?)?;
    params.set("height", &mut DictValue::from_i32(32)?)?;
    let layer = CropAndResizeLayer::create(&mut params)?;
    assert_eq!(0, layer.preferable_target()?);
    Ok(())
}

#[test]
#[cfg(not(feature = "opencv-32"))]
fn dict() -> Result<()> {
    {
        let v = DictValue::from_f64(123.456)?;
        assert!(v.is_real()?);
        assert!(!v.is_int()?);
        assert!(!v.is_string()?);
        assert_eq!(1, v.size()?);
        assert_eq!(123.456, v.get_f64(-1)?);
        assert_matches!(v.get_i64(-1), Err(Error{code: core::StsAssert, ..}));
        assert_eq!(123.456, v.get_real_value(-1)?);
        assert_matches!(v.get_int_value(-1), Err(Error{code: core::StsAssert, ..}));
    }

    {
        let v = DictValue::from_i64(123)?;
        assert!(v.is_int()?);
        assert!(v.is_real()?);
        assert!(!v.is_string()?);
        assert_eq!(1, v.size()?);
        assert_eq!(123, v.get_i64(-1)?);
        assert_eq!(123., v.get_f64(-1)?);
        assert_eq!(123, v.get_int_value(-1)?);
        assert_eq!(123., v.get_real_value(-1)?);
        assert_matches!(v.get_string_value(-1), Err(Error{code: core::StsAssert, ..}));
    }

    {
        let v = DictValue::from_str("876543.123")?;
        assert!(!v.is_int()?);
        assert!(!v.is_real()?);
        assert!(v.is_string()?);
        assert_eq!(1, v.size()?);
        assert_eq!("876543.123", v.get_string(-1)?);
        assert_eq!(876543, v.get_i64(-1)?);
        assert_eq!(876543.123, v.get_real_value(-1)?);
        assert_eq!("876543.123", v.get_string_value(-1)?);
    }
    Ok(())
}
