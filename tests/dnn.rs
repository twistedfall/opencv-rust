use opencv::dnn::{Net, LayerParams};

#[test]
fn net() {
    let mut net = Net::new().unwrap();
    assert!(net.empty().unwrap());
    net.enable_fusion(false).unwrap();
    let mut params = LayerParams::new().unwrap();
    assert_eq!(params.name().unwrap(), "");
    assert_eq!(params._type().unwrap(), "");
    params.set_name("param name").unwrap();
    params.set_type("param type").unwrap();
    assert_eq!(params.name().unwrap(), "param name");
    assert_eq!(params._type().unwrap(), "param type");
    let blobs = params.blobs().unwrap();
    assert_eq!(0, blobs.len());
    let res = net.add_layer("layer", "type", &mut params).unwrap();
    assert_ne!(-1, res);
    assert!(!net.empty().unwrap());
}
