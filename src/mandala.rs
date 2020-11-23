use crate::segment::Segment;
use rand::rngs::SmallRng;
use rand::SeedableRng;

fn hash_sum(hash: &str) -> u64 {
    hash.as_bytes().into_iter().map(|&x| x as u64).sum()
}

pub struct Mandala {
    segments: Vec<Segment>,
}

impl Mandala {
    pub fn new(hash: &str, nr_of_segments: u32, radius: f32, detail: u32) -> Mandala {
        let angle = 2.0 * std::f32::consts::PI / nr_of_segments as f32;
        let mut rng = SmallRng::seed_from_u64(hash_sum(hash));
        let segment = Segment::new(&mut rng, angle, radius, detail);

        let mut segments: Vec<Segment> = Vec::new();
        for i in (0..nr_of_segments).step_by(2) {
            let rotated = segment.rotate(i as f32 * angle);
            segments.push(rotated.clone());
            segments.push(rotated.reflect_along(i as f32 * angle));
        }
        Mandala { segments }
    }
    pub fn to_svg(&self, width: f32, height: f32, fill: &str) -> String {
        let mut svg = format!(
            concat!(
                r#"<?xml version="1.0" standalone="yes"?>"#,
                r#"<svg xmlns="http://www.w3.org/2000/svg""#,
                r#" version="1.1""#,
                r#" viewBox="0 0 {w} {h}">"#,
                r#"<path fill="{fill}" d=""#,
            ),
            w = width,
            h = height,
            fill = fill,
        );

        for segment in self.segments.iter() {
            let points: Vec<String> = segment
                .points
                .iter()
                .map(|p| {
                    let (x, y) = p.to_cartesian(width / 2.0, height / 2.0);
                    format!("{x} {y}", x = x, y = y)
                })
                .collect();
            let path = format!(r#"M{p}z"#, p = points.join(" L "));
            svg.push_str(&path);
        }
        svg.push_str(r#""/></svg>"#);
        svg.into()
    }
}
