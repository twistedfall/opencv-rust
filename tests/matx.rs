use opencv::{
	core::Matx33d,
	prelude::*,
};

#[test]
fn matx() {
	let mut a = Matx33d::eye();
	assert_eq!(a[(0, 0)], 1.);
	assert_eq!(a[(1, 0)], 0.);
	assert_eq!(*a.get((0, 1)).unwrap(), 0.);
	assert_eq!(*a.get_mut((1, 1)).unwrap(), 1.);
}
