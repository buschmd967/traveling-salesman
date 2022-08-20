use rand::Rng;

#[derive(PartialEq)]

pub struct Point {
    pub x: f32,
    pub y: f32
}

impl Point {

    pub fn distanceBetween(p1: Point, p2: Point) -> f32 {
        let delta_x = p1.x - p2.x;
        let delta_y = p1.y - p2.y;
        let ans = (delta_x.powf(2.0) + delta_y.powf(2.0)).sqrt();
        return ans;
    }

    pub fn distanceTo(self, other_point: Point) -> f32 {
        return Point::distanceBetween(self, other_point);
    }

    pub fn generateRandom(radius:f32) -> Point {
        let mut rng = rand::thread_rng();

        let x: f32 = rng.gen_range( (-1.0 * radius)..radius);
        let y: f32 = rng.gen_range( (-1.0 * radius)..radius);
        
        return Self{x, y};
    }
}