use opencv::core::{self, Point2d, Point2f, Point2i, Rect, Rect2d, Rect2f, Rect2i, Size2d, Size2f, Size2i};

#[test]
fn rect_add() {
	let src = Rect::new(0, 0, 100, 100);

	// Point
	let res = Rect::new(50, 50, 100, 100);
	{
		let out = src + core::Point::new(50, 50);
		assert_eq!(out, res);
	}

	{
		let mut out = src;
		out += core::Point::new(50, 50);
		assert_eq!(out, res);
	}

	// Size
	let res = Rect::new(0, 0, 200, 200);
	{
		let out = src + core::Size::new(100, 100);
		assert_eq!(out, res);
	}

	{
		let mut out = src;
		out += core::Size::new(100, 100);
		assert_eq!(out, res);
	}
}

#[test]
fn rect_sub() {
	let src = Rect::new(50, 50, 100, 100);

	// Point
	let res = Rect::new(25, 25, 100, 100);
	{
		let out = src - core::Point::new(25, 25);
		assert_eq!(out, res);
	}

	{
		let mut out = src;
		out -= core::Point::new(25, 25);
		assert_eq!(out, res);
	}

	// Size
	let res = Rect::new(50, 50, 75, 75);
	{
		let out = src - core::Size::new(25, 25);
		assert_eq!(out, res);
	}

	{
		let mut out = src;
		out -= core::Size::new(25, 25);
		assert_eq!(out, res);
	}
}

#[test]
fn rect_intersect() {
	{
		assert_eq!(
			Rect::new(50, 50, 100, 100),
			Rect::new(50, 50, 100, 100) & Rect::new(50, 50, 100, 100)
		);
		assert_eq!(
			Rect2d::new(149., 149., 1., 1.),
			Rect2d::new(50., 50., 100., 100.) & Rect2d::new(149., 149., 100., 100.)
		);
		assert_eq!(
			Rect2d::new(0., 0., 0., 0.),
			Rect2d::new(50., 50., 100., 100.) & Rect2d::new(150., 50., 100., 100.)
		);
		assert_eq!(
			Rect2f::new(0., 0., 0., 0.),
			Rect2f::new(50., 50., 100., 100.) & Rect2f::new(50., 150., 100., 100.)
		);
		assert_eq!(
			Rect2d::new(0., 0., 0., 0.),
			Rect2d::new(50., 50., 100., 100.) & Rect2d::new(-50., 50., 100., 100.)
		);
		assert_eq!(
			Rect::new(0, 0, 0, 0),
			Rect::new(50, 50, 100, 100) & Rect::new(50, -50, 100, 100)
		);
		assert_eq!(
			Rect2f::new(75., 75., 75., 75.),
			Rect2f::new(50., 50., 100., 100.) & Rect2f::new(75., 75., 100., 100.)
		);
	}

	{
		let mut r = Rect::new(50, 50, 100, 100);
		r &= Rect::new(75, 75, 100, 10);
		assert_eq!(Rect::new(75, 75, 75, 10), r);
	}
}

#[test]
fn rect_union() {
	{
		assert_eq!(
			Rect::new(50, 50, 100, 100),
			Rect::new(50, 50, 100, 100) | Rect::new(50, 50, 100, 100)
		);
		assert_eq!(
			Rect2d::new(50., 50., 199., 199.),
			Rect2d::new(50., 50., 100., 100.) | Rect2d::new(149., 149., 100., 100.)
		);
		assert_eq!(
			Rect2d::new(50., 50., 200., 100.),
			Rect2d::new(50., 50., 100., 100.) | Rect2d::new(150., 50., 100., 100.)
		);
		assert_eq!(
			Rect2f::new(50., 50., 100., 200.),
			Rect2f::new(50., 50., 100., 100.) | Rect2f::new(50., 150., 100., 100.)
		);
		assert_eq!(
			Rect2d::new(-50., 50., 200., 100.),
			Rect2d::new(50., 50., 100., 100.) | Rect2d::new(-50., 50., 100., 100.)
		);
		assert_eq!(
			Rect::new(50, -50, 100, 200),
			Rect::new(50, 50, 100, 100) | Rect::new(50, -50, 100, 100)
		);
		assert_eq!(
			Rect2f::new(50., 50., 125., 125.),
			Rect2f::new(50., 50., 100., 100.) | Rect2f::new(75., 75., 100., 100.)
		);
	}

	{
		let mut r = Rect::new(50, 50, 100, 100);
		r |= Rect::new(75, 75, 100, 10);
		assert_eq!(Rect::new(50, 50, 125, 100), r);
	}
}

#[test]
fn rect_constructor() {
	let r = Rect2i::from_point_size(Point2i::new(1, 2), Size2i::new(3, 4));
	assert_eq!(1, r.x);
	assert_eq!(2, r.y);
	assert_eq!(3, r.width);
	assert_eq!(4, r.height);

	let r = Rect2d::from_points(Point2d::new(10., 20.), Point2d::new(20., 10.));
	assert_eq!(10., r.x);
	assert_eq!(10., r.y);
	assert_eq!(10., r.width);
	assert_eq!(10., r.height);
}

#[test]
fn rect_methods() {
	let mut rect = Rect2d::new(10., 20., 100., 200.);
	assert!(!rect.empty());
	assert_eq!(rect.empty(), rect.size().empty());
	assert_eq!(20000., rect.area());
	assert_eq!(rect.area(), rect.size().area());
	assert_eq!(Point2d::new(10., 20.), rect.tl());
	assert_eq!(Point2d::new(110., 220.), rect.br());
	assert_eq!(Size2d::new(100., 200.), rect.size());
	assert!(rect.contains(Point2d::new(20., 20.)));
	assert!(!rect.contains(Point2d::new(120., 120.)));
	rect.width = 0.;
	assert!(rect.empty());
	assert_eq!(rect.empty(), rect.size().empty());
}

#[test]
fn rect_conv() {
	let rectf = Rect2d::new(1.2, 2.3, 3.4, 4.5);
	assert_eq!(Rect2i::new(1, 2, 3, 4), rectf.to::<i32>().unwrap());
	assert_eq!(Rect2f::new(1.2, 2.3, 3.4, 4.5), rectf.to::<f32>().unwrap());
	let recti = Rect2i::new(1, 2, 3, 4);
	assert_eq!(Rect2f::new(1., 2., 3., 4.), recti.to::<f32>().unwrap());
}

#[test]
fn rect_from() {
	assert_eq!(Rect2i::new(1, 2, 3, 4), (1, 2, 3, 4).into());
	assert_eq!(
		Rect2f::new(1., 2., 3., 4.),
		(Point2f::new(1., 2.), Size2f::new(3., 4.)).into()
	);
	assert_eq!(
		Rect2d::new(1., 2., 3., 4.),
		(Point2d::new(1., 2.), Point2d::new(4., 6.)).into()
	);
}
