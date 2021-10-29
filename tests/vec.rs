use opencv::core::{Vec2d, Vec2i, Vec3b, Vec3d, Vec4f, Vec4s};

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

#[test]
fn vec_conj() {
	assert_eq!(Vec4f::from([1., -2., -3., -4.]), Vec4f::from([1., 2., 3., 4.]).conj());
	assert_eq!(Vec2d::from([2., 4.]), Vec2d::from([2., -4.]).conj());
}

#[test]
fn vec_cross() {
	let a = Vec3d::from([10., 15., 20.]);
	let b = Vec3d::from([-90., 85., -80.]);
	assert_eq!(Vec3d::from([-2900., -1000., 2200.]), a.cross(b));
}

#[test]
fn vec_mul() {
	let mut a = Vec3d::from([10., 15., 20.]);
	let b = Vec3d::from([-90., 85., -80.]);
	assert_eq!(Vec3d::from([-900., 1275., -1600.]), a.mul(b));

	a *= 3.;
	let c = a * 2.;
	assert_eq!(Vec3d::from([30., 45., 60.]), a);
	assert_eq!(Vec3d::from([60., 90., 120.]), c);

	let a = Vec4s::from([4, 19, 7, -4]);
	let b = Vec4s::from([0, 8, -15, 43]);
	assert_eq!(Vec4s::from([125, 273, -909, -169]), a * b);
}

#[test]
fn vec_div() {
	let mut a = Vec3d::from([10., 15., 20.]);
	a /= 5.;
	let c = a / 2.;
	assert_eq!(Vec3d::from([2., 3., 4.]), a);
	assert_eq!(Vec3d::from([1., 1.5, 2.]), c);
}

#[test]
fn vec_add() {
	let mut a = Vec2i::from([10, 15]);
	let b = Vec2i::from([-90, 85]);
	a += b;
	let c = a + b;
	assert_eq!(Vec2i::from([-80, 100]), a);
	assert_eq!(Vec2i::from([-170, 185]), c);
}

#[test]
fn vec_sub() {
	let mut a = Vec2i::from([10, 15]);
	let b = Vec2i::from([-90, 85]);
	a -= b;
	let c = a - b;
	assert_eq!(Vec2i::from([100, -70]), a);
	assert_eq!(Vec2i::from([190, -155]), c);
}

#[test]
fn vec_neg() {
	let a = Vec2i::from([10, -15]);
	assert_eq!(Vec2i::from([-10, 15]), -a);
}
