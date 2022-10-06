use opencv::core::{Point, Point2d, Point2f, Point2i, Point2l, Size2d, Size2l, Vec2d, Vec2f};

#[test]
fn point_add() {
	let src = Point::new(0, 0);

	let res = Point::new(50, 50);
	{
		let out = src + Point::new(50, 50);
		assert_eq!(out, res);
	}

	{
		let mut out = src;
		out += Point::new(50, 50);
		assert_eq!(out, res);
	}
}

#[test]
fn point_sub() {
	let src = Point::new(50, 50);

	let res = Point::new(25, 25);
	{
		let out = src - Point::new(25, 25);
		assert_eq!(out, res);
	}

	{
		let mut out = src;
		out -= Point::new(25, 25);
		assert_eq!(out, res);
	}
}

#[test]
fn point_mul() {
	let src = Point2f::new(50., 50.);

	let res = Point2f::new(100., 100.);
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
fn point_div() {
	let src = Point::new(50, 50);

	let res = Point::new(25, 25);
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
fn point_constructor() {
	let r = Point2l::from_size(Size2l::new(1, 2));
	assert_eq!(1, r.x);
	assert_eq!(2, r.y);
}

#[test]
fn point_methods() {
	let pt = Point::new(2, 2);
	assert_eq!(2.8284271247461903, pt.norm());

	assert_eq!(0., pt.cross(Point::new(2, 2)));
	assert_eq!(0., pt.cross(Point::new(-4, -4)));
	assert_eq!(16., pt.cross(Point::new(-4, 4)));

	assert_eq!(8, pt.dot(Point::new(2, 2)));
	assert_eq!(8., pt.ddot(Point::new(2, 2)));
}

#[test]
fn point_conv() {
	let ptf = Point2d::new(1.2, 2.3);
	assert_eq!(Point2i::new(1, 2), ptf.to::<i32>().unwrap());
	assert_eq!(Point2f::new(1.2, 2.3), ptf.to::<f32>().unwrap());
	let pti = Point2i::new(1, 2);
	assert_eq!(Point2f::new(1., 2.), pti.to::<f32>().unwrap());

	let vec = Vec2d::from([10., 20.]);
	assert_eq!(vec, Point2d::from_vec2(vec).to_vec2());
}

#[test]
fn point_from() {
	assert_eq!(Point2l::new(1, 2), (1, 2).into());
	assert_eq!(Point2f::new(1., 2.), Vec2f::from([1., 2.]).into());
	assert_eq!(Point2d::new(1., 2.), Size2d::new(1., 2.).into());
}
