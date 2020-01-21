use opencv::core::Vec3b;

#[test]
fn vec() {
    let mut a = Vec3b::default();
    assert_eq!(a[0], 0);
    assert_eq!(a.len(), 3);

    a[1] = 2;
    assert_eq!(a[1], 2);
}

#[test]
fn vec_deref() {
    assert_eq!(vec![10, 20, 30], Vec3b::from([10, 20, 30]).to_vec());
}
