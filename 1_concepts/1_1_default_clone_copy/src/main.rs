use smart_default::SmartDefault;

#[derive(SmartDefault)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Point {
    #[default = 0.0]
    x: f64,
    #[default = 0.0]
    y: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let p1 = Point::default();
        let p2 = Point{x: 3.5, y: 2.1,};
        let p3 = Point{x: 1.0, ..Default::default()};
        assert!(p1 == Point{x: 0.0, y: 0.0,});
        assert!(p2 == Point{x: 3.5, y: 2.1,});
        assert!(p3 == Point{x: 1.0, y: 0.0,});
    }
}
