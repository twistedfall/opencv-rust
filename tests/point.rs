use opencv::core::Point;

#[test]
fn point_add() {
    let src = Point::new(0, 0);

    let res = Point::new(50, 50);
    {
        let src = src.clone();
        let out = src + Point::new(50, 50);
        assert_eq!(out, res);
    }

    {
        let mut out = src.clone();
        out += Point::new(50, 50);
        assert_eq!(out, res);
    }
}

#[test]
fn point_sub() {
    let src = Point::new(50, 50);

    let res = Point::new(25, 25);
    {
        let src = src.clone();
        let out = src - Point::new(25, 25);
        assert_eq!(out, res);
    }

    {
        let mut out = src.clone();
        out -= Point::new(25, 25);
        assert_eq!(out, res);
    }
}

#[test]
fn point_mul() {
    let src = Point::new(50, 50);

    let res = Point::new(100, 100);
    {
        let src = src.clone();
        let out = src * 2;
        assert_eq!(out, res);
    }

    {
        let mut out = src.clone();
        out *= 2;
        assert_eq!(out, res);
    }
}

#[test]
fn point_div() {
    let src = Point::new(50, 50);

    let res = Point::new(25, 25);
    {
        let src = src.clone();
        let out = src / 2;
        assert_eq!(out, res);
    }

    {
        let mut out = src.clone();
        out /= 2;
        assert_eq!(out, res);
    }
}

#[test]
fn point_methods() {
    let pt = Point::new(2, 2);
    assert_eq!(2.8284271247461903, pt.norm());
}
