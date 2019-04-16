use opencv::core;

#[test]
fn vec() {
    let mut a = core::Vec3b::default();
    assert_eq!(a[0], 0);
    assert_eq!(a.len(), 3);

    a[1] = 2;
    assert_eq!(a[1], 2);
}
