use traveling_salesman::point::{self, Point};

pub struct PointManager {
    pub points:Vec<Point>,
    pub current_path: Vec<Point>,
    pub best_path: Vec<Point>,
    radius:f32  
}

impl PointManager {
    pub fn default() -> Self {
        Self{
            points: Vec::new(),
            current_path: Vec::new(),
            best_path: Vec::new(),
            radius: 10.0
        }
    }

    pub fn add_random_point(&mut self) {
        let mut new_point:Point = Point::generateRandom(self.radius);

        while self.point_exists(&new_point) {
            new_point = Point::generateRandom(self.radius);
        }
        
        self.points.push(new_point);
    }

    pub fn add_point(&mut self, p:Point) -> bool {
        if !self.point_exists(&p){
            self.points.push(p);
            return true;
        }
        return false;
    }

    pub fn remove_last_point(&mut self) {
        self.points.pop();
    }

    fn point_exists(&self, p:&Point) -> bool {
        let potential_index = self.points.iter().position(|point| *point == *p );
        match potential_index{
            Some(_) =>  return true,
            None => return false
        }
    }

    


}

