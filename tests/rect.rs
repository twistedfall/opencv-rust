use opencv::core;

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
