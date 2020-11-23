#[derive(Debug, Clone)]
pub struct Point {
    r: f32,
    a: f32,
}

impl Point {
    pub fn new(r: f32, a: f32) -> Point {
        Point { r, a }
    }

    pub fn reflect_along(&self, angle: f32) -> Point {
        let a = if angle > self.a {
            self.a - (self.a - angle) * 2.0
        } else if angle < self.a {
            self.a + (angle - self.a) * 2.0
        } else {
            self.a
        };
        Point { r: self.r, a }
    }

    pub fn rotate(&self, angle: f32) -> Point {
        Point {
            r: self.r,
            a: self.a + angle,
        }
    }

    pub fn to_cartesian(&self, x0: f32, y0: f32) -> (f32, f32) {
        (x0 + self.r * self.a.cos(), y0 + self.r * self.a.sin())
    }
}
