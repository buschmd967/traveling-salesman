use rand::seq::SliceRandom;
use traveling_salesman::point::{self, Point};
use rand::Rng;

#[derive(Clone, Copy)]
pub enum RunMode {
    None,
    GenerateRandom
} 

pub struct PointManager {
    pub points:Vec<Point>,
    pub current_path: Vec<Point>,
    pub best_path: Vec<Point>,
    pub score: f32,
    pub mode: RunMode,
    radius:f32  
}

impl PointManager {
    pub fn default() -> Self {
        Self{
            points: Vec::new(),
            current_path: Vec::new(),
            best_path: Vec::new(),
            score: f32::INFINITY,
            mode: RunMode::None,
            radius: 10.0
        }
    }

    pub fn change_mode(&mut self, mode:RunMode){
        self.mode = mode;
        self.current_path = Vec::new();
    }

    pub fn add_random_point(&mut self) {
        let mut new_point:Point = Point::generateRandom(self.radius);

        while self.point_exists(&new_point) {
            new_point = Point::generateRandom(self.radius);
        }
        
        self.points.push(new_point);
        self.reset_paths();
    }

    pub fn add_point(&mut self, p:Point) -> bool {
        if !self.point_exists(&p){
            self.points.push(p);
            self.reset_paths();
            return true;
        }
        return false;
    }

    pub fn remove_last_point(&mut self) {
        self.points.pop();
        self.reset_paths();
    }

    fn point_exists(&self, p:&Point) -> bool {
        let potential_index = self.points.iter().position(|point| *point == *p );
        match potential_index{
            Some(_) =>  return true,
            None => return false
        }
    }

    pub fn reset_paths(&mut self) {
        self.current_path = Vec::new();
        self.best_path = Vec::new();
        self.score = f32::INFINITY;
    }

    fn evaluate_path(path:&Vec<Point>) -> f32 {
        let mut score = 0.0;
        
        for (index, &point) in path.iter().enumerate() {
            if index == path.len() - 1 {
                score += point.distanceTo(&path[0]);
            } 
            else{
                score += point.distanceTo(&path[index + 1]);
            }
        }

        return score;
    }

    pub fn random_path_step(&mut self) {
        let random_path = self.generate_random_path();
        let new_score = PointManager::evaluate_path(&random_path);
        if new_score <= self.score {
            self.score = new_score;
            self.best_path = random_path.clone();
        }
        self.current_path = random_path.clone();
    }

    fn generate_random_path(&self) -> Vec<Point> {
        let mut rng = rand::thread_rng();

        let mut random_path = self.points.clone();
        random_path.shuffle(&mut rng);
        
        return random_path;
    }


}

