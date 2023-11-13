use opencv::core::{Point2d, Point2f, Point3d, Point3f, Point3i, Vec3d, Vec3i};

#[test]
fn point3_add() {
	let src = Point3i::new(1, 2, 3);

	let res = Point3i::new(11, 22, 33);
	{
		let out = src + Point3i::new(10, 20, 30);
		assert_eq!(out, res);
	}

	{
		let mut out = src;
		out += Point3i::new(10, 20, 30);
		assert_eq!(out, res);
	}
}

#[test]
fn point3_sub() {
	let src = Point3i::new(10, 20, 30);

	let res = Point3i::new(9, 18, 27);
	{
		let out = src - Point3i::new(1, 2, 3);
		assert_eq!(out, res);
	}

	{
		let mut out = src;
		out -= Point3i::new(1, 2, 3);
		assert_eq!(out, res);
	}
}

#[test]
fn point3_mul() {
	let src = Point3f::new(10., 20., 30.);

	let res = Point3f::new(20., 40., 60.);
	{
		let out = src * 2.;
		assert_eq!(out, res);
	}

	{
		let mut out = src;
		out *= 2.;
		assert_eq!(out, res);
	}
}

#[test]
fn point3_div() {
	let src = Point3i::new(10, 20, 30);

	let res = Point3i::new(5, 10, 15);
	{
		let out = src / 2;
		assert_eq!(out, res);
	}

	{
		let mut out = src;
		out /= 2;
		assert_eq!(out, res);
	}
}

#[test]
fn point3_constructor() {
	let r = Point3d::from_point(Point2d::new(10., 20.));
	assert_eq!(10., r.x);
	assert_eq!(20., r.y);
	assert_eq!(0., r.z);
}

#[test]
fn point3_methods() {
	let pt = Point3i::new(2, 3, 4);
	assert_eq!(5.385164807134504, pt.norm());

	assert_eq!(Point3i::new(1, -2, 1), pt.cross(Point3i::new(1, 2, 3)));
	assert_eq!(Point3i::new(2, -4, 2), pt.cross(Point3i::new(-4, -5, -6)));
	assert_eq!(Point3i::new(-2, -28, 22), pt.cross(Point3i::new(-4, 5, 6)));

	assert_eq!(20, pt.dot(Point3i::new(1, 2, 3)));
	assert_eq!(29., pt.ddot(Point3i::new(2, 3, 4)));
}

#[test]
fn point3_conv() {
	let ptf = Point3d::new(1.2, 2.3, 3.4);
	assert_eq!(Point3i::new(1, 2, 3), ptf.to::<i32>().unwrap());
	assert_eq!(Point3f::new(1.2, 2.3, 3.4), ptf.to::<f32>().unwrap());
	let pti = Point3i::new(1, 2, 3);
	assert_eq!(Point3f::new(1., 2., 3.), pti.to::<f32>().unwrap());

	let vec = Vec3d::from([10., 20., 30.]);
	assert_eq!(vec, Point3d::from_vec3(vec).to_vec3());
}

#[test]
fn point3_from() {
	assert_eq!(Point3d::new(1., 2., 3.), (1., 2., 3.).into());
	assert_eq!(Point3i::new(1, 2, 3), Vec3i::from([1, 2, 3]).into());
	assert_eq!(Point3f::new(1., 2., 0.), Point2f::new(1., 2.).into());
}
