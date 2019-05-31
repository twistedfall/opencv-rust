use opencv::{
    core::Mat,
    dnn::{Dict, LayerParams, Net},
    Result,
    types::VectorOfMat,
};

#[test]
fn net() -> Result<()> {
    let mut net = Net::new()?;
    assert!(net.empty()?);
    net.enable_fusion(false)?;
    let mut params = LayerParams::new()?;
    assert_eq!(params.name()?, "");
    assert_eq!(params._type()?, "");
    params.set_name("param name")?;
    params.set_type("param type")?;
    let mut blobs = VectorOfMat::new();
    blobs.push(Mat::new()?);
    params.set_blobs(blobs)?;
    assert_eq!(params.name()?, "param name");
    assert_eq!(params._type()?, "param type");
    assert!(!params.has("test")?);
    let blobs = params.blobs()?;
    assert_eq!(1, blobs.len());
    let res = net.add_layer("layer", "type", &mut params)?;
    assert_ne!(-1, res);
    assert!(!net.empty()?);
    Ok(())
}
