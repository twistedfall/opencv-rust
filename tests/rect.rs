use opencv::core::{self, Point2d, Point2i, Rect2d, Rect2f, Rect2i, Size2d, Size2i};

#[test]
fn rect_add() {
    let src = core::Rect::new(0, 0, 100, 100);

    // Point
    let res = core::Rect::new(50, 50, 100, 100);
    {
        let src = src.clone();
        let out = src + core::Point::new(50, 50);
        assert_eq!(out, res);
    }

    {
        let mut out = src.clone();
        out += core::Point::new(50, 50);
        assert_eq!(out, res);
    }

    // Size
    let res = core::Rect::new(0, 0, 200, 200);
    {
        let src = src.clone();
        let out = src + core::Size::new(100, 100);
        assert_eq!(out, res);
    }

    {
        let mut out = src.clone();
        out += core::Size::new(100, 100);
        assert_eq!(out, res);
    }
}

#[test]
fn rect_sub() {
    let src = core::Rect::new(50, 50, 100, 100);

    // Point
    let res = core::Rect::new(25, 25, 100, 100);
    {
        let src = src.clone();
        let out = src - core::Point::new(25, 25);
        assert_eq!(out, res);
    }

    {
        let mut out = src.clone();
        out -= core::Point::new(25, 25);
        assert_eq!(out, res);
    }

    // Size
    let res = core::Rect::new(50, 50, 75, 75);
    {
        let src = src.clone();
        let out = src - core::Size::new(25, 25);
        assert_eq!(out, res);
    }

    {
        let mut out = src.clone();
        out -= core::Size::new(25, 25);
        assert_eq!(out, res);
    }
}

#[test]
fn rect_constructor() {
    let r = Rect2i::from_point_size(Point2i::new(1, 2), Size2i::new(3,4));
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
