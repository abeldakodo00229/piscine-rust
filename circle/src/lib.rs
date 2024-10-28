
#[derive(Debug, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

#[derive(Debug, PartialEq)]
pub struct Point {
   pub x: f64,
   pub y: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            center: Point { x: x, y : y },
            radius: radius,
        }
    }

    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    pub fn intersect(&self, circle1: &Circle) -> bool {
        let d = ((circle1.center.x - self.center.x).powi(2) + (circle1.center.y - self.center.y).powi(2)).sqrt();
        d <= (circle1.radius + self.radius)
    }
}

impl Point {
    pub fn distance(&self, point: &Point) -> f64 {
        let dx = point.x - self.x;
        let dy = point.y - self.y;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}
