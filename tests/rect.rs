use opencv::core;
use opencv::core::{Rect2d, Point2d, Size2d};

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
fn rect_methods() {
    let mut rect = Rect2d::new(10., 20., 100., 200.);
    assert!(!rect.empty());
    assert_eq!(20000., rect.area());
    assert_eq!(Point2d::new(10., 20.), rect.tl());
    assert_eq!(Point2d::new(110., 220.), rect.br());
    assert_eq!(Size2d::new(100., 200.), rect.size());
    assert!(rect.contains(Point2d::new(20., 20.)));
    assert!(!rect.contains(Point2d::new(120., 120.)));
    rect.width = 0.;
    assert!(rect.empty());
}
