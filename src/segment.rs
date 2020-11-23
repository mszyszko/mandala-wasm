use crate::point::Point;
use rand::rngs::SmallRng;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Segment {
    pub points: Vec<Point>,
}

impl Segment {
    pub fn new(rng: &mut SmallRng, angle: f32, radius: f32, n: u32) -> Segment {
        let mut points: Vec<Point> = Vec::new();
        for _i in 0..=n {
            let mut r = rng.gen_range(0.0, radius);
            points.push(Point::new(r, 0.0));
            r = rng.gen_range(0.0, radius);
            points.push(Point::new(r, angle));
        }
        Segment { points }
    }

    pub fn reflect_along(&self, angle: f32) -> Segment {
        let mut points: Vec<Point> = Vec::new();
        for p in self.points.iter() {
            points.push(p.reflect_along(angle));
        }
        Segment { points }
    }

    pub fn rotate(&self, angle: f32) -> Segment {
        let mut points: Vec<Point> = Vec::new();
        for p in self.points.iter() {
            points.push(p.rotate(angle));
        }
        Segment { points }
    }
}
