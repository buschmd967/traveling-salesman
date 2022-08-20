use traveling_salesman::point::{self, Point};

struct PointManager {
    points:Vec<Point>,
    radius:f32  
}

impl PointManager {
    fn default() -> Self {
        Self{
            points: Vec::new(),
            radius: 20.0
        }
    }

    fn add_random_point(&mut self) {
        let mut new_point:Point = Point::generateRandom(self.radius);

        while self.point_exists(&new_point) {
            new_point = Point::generateRandom(self.radius);
        }
        
        self.points.push(new_point);
    }

    fn add_point(&mut self, p:Point) -> bool {
        if !self.point_exists(&p){
            self.points.push(p);
            return true;
        }
        return false;
    }

    fn point_exists(&self, p:&Point) -> bool {
        let potential_index = self.points.iter().position(|point| *point == *p );
        match potential_index{
            Some(_) =>  return true,
            None => return false
        }
    }


}

