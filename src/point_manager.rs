use rand::{seq::SliceRandom, Rng};
use traveling_salesman::point::Point;




pub struct PointManager {
    pub points:Vec<Point>,
    pub current_path: Vec<Point>,
    pub best_path: Vec<Point>,
    pub saved_path: Vec<Point>,
    pub score: f32,
    pub saved_score: f32,
    radius:f32  
}

impl PointManager {
    pub fn default() -> Self {
        Self{
            points: Vec::new(),
            current_path: Vec::new(),
            best_path: Vec::new(),
            saved_path: Vec::new(),
            score: f32::INFINITY,
            saved_score: f32::INFINITY,
            radius: 10.0
        }
    }

    // pub fn change_mode(&mut self, mode:RunMode){
    //     self.mode = mode;
    //     self.current_path = Vec::new();

    //     //Sanity check
    //     if self.best_path.len() <= 1 {
    //         self.score = f32::INFINITY;
    //     }
    // }

    pub fn add_random_point(&mut self) {
        let mut new_point:Point = Point::generate_random(self.radius);

        while self.point_exists(&new_point) {
            new_point = Point::generate_random(self.radius);
        }
        
        self.points.push(new_point);
        self.reset_paths();
    }

    pub fn add_random_points(&mut self, n: i32) {
        for _ in 0..n {
            self.add_random_point();
        }
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

    pub fn clear_points(&mut self) {
        self.points = Vec::new();
        self.reset_paths();
    }

    fn point_exists(&self, p:&Point) -> bool {
        let potential_index = self.points.iter().position(|point| *point == *p );
        match potential_index{
            Some(_) =>  return true,
            None => return false
        }
    }

    pub fn save_current_path(&mut self) {
        self.saved_path = self.best_path.clone();
        self.saved_score = self.score.clone();
    }

    pub fn reset_saved_path(&mut self) {
        self.saved_path = Vec::new();
        self.saved_score = f32::INFINITY;
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
                score += point.distance_to(&path[0]);
            } 
            else{
                score += point.distance_to(&path[index + 1]);
            }
        }

        return score;
    }

    fn test_replace_new_path(&mut self, path:Vec<Point>) {
        let new_score = PointManager::evaluate_path(&path);
        if new_score <= self.score {
            self.score = new_score;
            self.best_path = path.clone();
        }
        self.current_path = path.clone();
    }

    pub fn random_path_step(&mut self) {
        let random_path = self.generate_random_path();
        self.test_replace_new_path(random_path);
    }

    fn generate_random_path(&self) -> Vec<Point> {
        let mut rng = rand::thread_rng();

        let mut random_path = self.points.clone();
        random_path.shuffle(&mut rng);
        
        return random_path;
    }

    pub fn random_swap_step(&mut self, n: i32) {
        if self.best_path.len() > 0 {    
            let path = self.generage_random_swap(n);
            self.test_replace_new_path(path);
        }
    }

    fn generage_random_swap(&self, n:i32) -> Vec<Point> {
        let mut rng = rand::thread_rng();

        let mut path = self.best_path.clone();
        let size = path.len();
        if size > 1 {
            for _ in 0..n {
                let i = rng.gen_range(0..size);
                let j = rng.gen_range(0..size);

                let el = path.remove(i);
                path.insert(j, el);
            }

        }

        return path;
    }

    pub fn random_connection_swap_step(&mut self, n:i32) {
        if self.best_path.len() > 0 {
            let path = self.generate_random_connection_swap(n);
            self.test_replace_new_path(path);
        }
    }

    fn generate_random_connection_swap(&self, n:i32) -> Vec<Point> {
        let mut rng = rand::thread_rng();

        let mut path = self.best_path.clone();
        let size = path.len();
        if size > 1 {
            for _ in 0..n {
                let i = rng.gen_range(0..size-1);
                let j = rng.gen_range(i..size);

                path[i..j].reverse();
            }
        }

        return path;
    }

    pub fn set_radial_path(&mut self) {
        self.best_path = self.generate_radial_path();
        self.score = PointManager::evaluate_path(&self.best_path);
    }

    fn generate_radial_path_step(remaining_points: &mut Vec<Point>, path: &mut Vec<Point>, radius: f32){
        let origin: Point = Point::new(0.0, 0.0);

        for el in remaining_points.clone() {
            if el.distance_to(&origin) >= radius {
                //Remove point from remaining points
                let index = remaining_points.iter().position(|x| *x == el).unwrap();
                remaining_points.remove(index);

                //Find closest point in path
                let mut best_index = 0;
                let mut shortest_distance = f32::INFINITY;

                for (index, p) in path.iter().enumerate() {
                    let distance = p.distance_to(&el);
                    if  distance < shortest_distance {
                        shortest_distance = distance;
                        best_index = index;
                    }
                }
                
                //Add el next to closest point in path
                path.insert(best_index, el);
                
            }
        }

    }

    fn generate_radial_path(&self) -> Vec<Point> {
        let mut remaining_points: Vec<Point> = self.points.clone();
        let mut path: Vec<Point> = Vec::new();
        let origin: Point = Point::new(0.0, 0.0);

        let mut r = self.radius.clone() + 10.0;
        loop{
            if r < 0.0 || remaining_points.len() == 0 {
                break;
            }
            
            for el in remaining_points.clone() {
                if el.distance_to(&origin) >= r {
                    //Remove point from remaining points
                    let index = remaining_points.iter().position(|x| *x == el).unwrap();
                    remaining_points.remove(index);

                    //Find closest point in path
                    let mut best_index = 0;
                    let mut shortest_distance = f32::INFINITY;

                    for (index, p) in path.iter().enumerate() {
                        let distance = p.distance_to(&el);
                        if  distance < shortest_distance {
                            shortest_distance = distance;
                            best_index = index+1;
                        }
                    }
                    
                    //Add el next to closest point in path
                    path.insert(best_index, el);
                    
                }
            }

            r -= 0.01
        }


        return path;
    }


}

