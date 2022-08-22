mod point_manager;
mod point;

use eframe::{App, run_native, egui::{self, Context, Color32, Ui, Button, plot::{self, MarkerShape, Text, Plot, PlotPoint}, CentralPanel, Visuals, TopBottomPanel}};
use point_manager::PointManager;
use std::{thread, time};
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy)]
pub enum RunMode {
    None,
    GenerateRandom,
    RandomPointSwap,
    RandomPathSwap
} 

struct LocalData{
    swap_n:i32,
    ctx: Context,
    mode: RunMode,
    show_saved_path: bool
}

impl LocalData{
    fn new(c: Context) -> Self {
        Self{
            swap_n: 1,
            ctx: c,
            mode: RunMode::None,
            show_saved_path: false
        }
    }

    fn toggle_show_saved_path(&mut self) {
        self.show_saved_path = !self.show_saved_path;
    }
}


struct MainWindow{ 
    point_manager:Arc<Mutex<PointManager>>,
    // context:Context,
    local_data: Arc<Mutex<LocalData>>
}

impl App for MainWindow {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {

        ctx.set_visuals(Visuals::dark());

        TopBottomPanel::bottom("ButtonPanel").show(ctx, |ui| {

            egui::Grid::new("Control Panel").show(ui, |control_ui| {
                self.control_panel(control_ui);                
             
            });
        });

        CentralPanel::default()
        .show(ctx, |ui| {
            self.graph(ui);

        });

    }
}

impl Default for MainWindow {

    fn default() -> Self {
        Self {
            point_manager: Arc::new(Mutex::new(PointManager::default())),
            // context: Context::default(),
            local_data: Arc::new(Mutex::new(LocalData::new(Context::default())))
        }
    }

}

impl MainWindow{

    fn new(cc: &eframe::CreationContext<'_>) -> Self {

        let mut mw = Self { 
            point_manager: Arc::new(Mutex::new(PointManager::default())),
            local_data: Arc::new(Mutex::new(LocalData::new(cc.egui_ctx.clone())))
        };

        mw.start_main_loop();
        return mw;
    }

    fn start_main_loop(&mut self) {
        let pm = Arc::clone(&self.point_manager);
        let ld = Arc::clone(&self.local_data);
        thread::spawn(move || {
            MainWindow::main_loop(pm, ld);
        });
    }

    fn main_loop(point_manager: Arc<Mutex<PointManager>>, local_data: Arc<Mutex<LocalData>>) {
        
        let mut last_repaint = time::Instant::now();
        let update_interval = time::Duration::from_millis(1);
        
        let ctx = local_data.lock().unwrap().ctx.clone();

        loop{
            
            if last_repaint + update_interval <= time::Instant::now() {
                ctx.request_repaint();
                last_repaint = time::Instant::now();
            } 
            let mode = local_data.lock().unwrap().mode.clone(); 
            match mode {
                RunMode::GenerateRandom => {
                    point_manager.lock().unwrap().random_path_step();
                },
                RunMode::RandomPointSwap => {
                    point_manager.lock().unwrap().random_swap_step(local_data.lock().unwrap().swap_n);
                },
                RunMode::RandomPathSwap => {
                    point_manager.lock().unwrap().random_connection_swap_step(local_data.lock().unwrap().swap_n);
                }
                RunMode::None => {
                    point_manager.lock().unwrap().current_path = Vec::new();
                }
            }
        }

    }

    fn control_panel(&mut self, ui: &mut Ui) {
        // ROW 1
        if ui.add(Button::new("Add Random Point")).clicked() {
            let pm = Arc::clone(&self.point_manager);
            pm.lock().unwrap().add_random_point();
        }

        if ui.add(Button::new("Start Random Search")).clicked() {
            let ld = Arc::clone(&self.local_data);
            ld.lock().unwrap().mode = RunMode::GenerateRandom;
        }
        
        if ui.add(Button::new("Start Random Point Swap")).clicked() {
            let ld = Arc::clone(&self.local_data);
            ld.lock().unwrap().mode = RunMode::RandomPointSwap;
        }

        if ui.add(Button::new("Toggle Saved Path")).clicked() {
            let ld = Arc::clone(&self.local_data);
            ld.lock().unwrap().toggle_show_saved_path();
            
        }

        ui.end_row();
        // ROW 2

        if ui.add(Button::new("Remove Last Point")).clicked() {
            let pm = Arc::clone(&self.point_manager);
            pm.lock().unwrap().remove_last_point();
        }

        if ui.add(Button::new("Reset Paths")).clicked() {
            let pm = Arc::clone(&self.point_manager);
            pm.lock().unwrap().reset_paths();
        }
        
        if ui.add(Button::new("Start Random Path Swap")).clicked() {
            let ld = Arc::clone(&self.local_data);
            ld.lock().unwrap().mode = RunMode::RandomPathSwap;
        }

        if ui.add(Button::new("Save Current Path")).clicked() {
            let pm = Arc::clone(&self.point_manager);
            pm.lock().unwrap().save_current_path();
            
        }
        
        ui.end_row();
        //ROW 3

        if ui.add(Button::new("Clear Points")).clicked() {
            let pm = Arc::clone(&self.point_manager);
            pm.lock().unwrap().clear_points();
        }

        if ui.add(Button::new("Stop")).clicked() {
            let ld = Arc::clone(&self.local_data);
            ld.lock().unwrap().mode = RunMode::None;
        }

        ui.horizontal(|horizontal_ui| {
            horizontal_ui.add(
                egui::Slider::new(&mut self.local_data.lock().unwrap().swap_n, 0..=5)
                .text("Swap n"));
        });

        if ui.add(Button::new("Reset Saved Path")).clicked() {
            let pm = Arc::clone(&self.point_manager);
            pm.lock().unwrap().reset_saved_path();
            
        }

    }
    
    fn graph(&self, ui: &mut Ui) {

        let x = 15.0;
        let y = 15.0;

        ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);

        Plot::new("plot")
            .show_axes([false, false])
            .allow_drag(false)
            .include_x(x)
            .include_x(-1.0 * x)
            .include_y(y)
            .include_y(-1.0 * y)
            .show_x(false)
            .show_y(false)
            .allow_boxed_zoom(false)
            .allow_scroll(false)
            .allow_zoom(false)
            .legend(plot::Legend::default())
            .show(ui, |plot_ui| {
                

                
                //Path stuff
                let pm = Arc::clone(&self.point_manager);
                let best_path = pm.lock().unwrap().best_path.clone();
                let current_path = pm.lock().unwrap().current_path.clone();
                let saved_path = pm.lock().unwrap().saved_path.clone();

                let show_saved_path = self.local_data.lock().unwrap().show_saved_path.clone();
                
                let mut current_best_color = Color32::from_rgb(0, 255, 0);  

                if show_saved_path {
                    current_best_color = Color32::from_rgba_unmultiplied(0, 255, 0, 255);    

                }


                //Current Best
                let best_score = pm.lock().unwrap().score;
                let current_best_text = "Best Path Score: ".to_owned() + &best_score.to_string(); 
                plot_ui.line(plot::Line::new(self.generate_path_values(&best_path))
                    .highlight(true)
                    .color(current_best_color)
                    .name(current_best_text)
                );

                //Saved Path
                if show_saved_path {
                    let saved_score = pm.lock().unwrap().saved_score;
                    let saved_path_text = "Saved Path Score: ".to_owned() + &saved_score.to_string(); 
                    plot_ui.line(plot::Line::new(self.generate_path_values(&saved_path))
                        .color(Color32::from_rgba_unmultiplied(0, 0, 255, 20))
                        .name(saved_path_text)
                        .width(12.0)
                        .highlight(true)
                    );
                }

                //Current Try
                plot_ui.line(
                    plot::Line::new(self.generate_path_values(&current_path))
                    .color(Color32::from_rgba_unmultiplied(80, 120, 255, 100))
                );

                //plot nodes
                plot_ui.points(
                    self.generate_points()
                    .shape(MarkerShape::Circle)
                    .filled(true)
                    .radius(3.0)
                    .color(Color32::RED)
                );

            });
    }

    fn generate_path_values(&self, points: &Vec<traveling_salesman::point::Point>) -> plot::PlotPoints {
        let mut values_vector = Vec::<[f64; 2]>::new();

        for point in points {
            values_vector.push(point.to_value());
        }

        match values_vector.get(0) {
            Some(v) => values_vector.push(v.clone()),
            None => {}
        }

        return plot::PlotPoints::new(values_vector);
    }

    fn generate_points(&self) -> plot::Points {


        let mut values_vector = Vec::<[f64;2]>::new();
        let pm = Arc::clone(&self.point_manager);

        for point in &pm.lock().unwrap().points {
            values_vector.push(point.to_value());
        }

        let values =  plot::PlotPoints::new(values_vector);
        return plot::Points::new(values);
    }
}

fn main(){

    let options = eframe::NativeOptions {
        #[cfg(feature = "wgpu")]
        renderer: eframe::Renderer::Wgpu,

        ..Default::default()
    };

    // let mut main_window = MainWindow::default();

    // main_window.start_main_loop();

    // thread::spawn(|| {
        run_native(
            "Traveling Salesman",
            options,
            Box::new(|cc| Box::new(MainWindow::new(cc)))
        );
    // });

}