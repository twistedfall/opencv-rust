#[cfg(not(feature = "opencv-32"))]
#[test]
fn ptr_f32() {
	use opencv::core::Ptr;

	let mut p = Ptr::new(10f32);
	assert_eq!(10., *p);
	*p = 30.123;
	assert_eq!(30.123, *p);

	let d = Ptr::<f32>::default();
	assert_eq!(*d, f32::default());
}
