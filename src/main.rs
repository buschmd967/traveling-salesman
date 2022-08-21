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
    RandomSwap
} 

struct LocalData{
    swap_n:i32,
    ctx: Context,
    mode: RunMode
}

impl LocalData{
    fn new(c: Context) -> Self {
        Self{
            swap_n: 1,
            ctx: c,
            mode: RunMode::None
        }
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

        CentralPanel::default().show(ctx, |ui| {
            self.graph(ui);
        });

        TopBottomPanel::bottom("ButtonPanel").show(ctx, |ui| {
            if ui.add(Button::new("Add Random Point")).clicked() {
                let pm = Arc::clone(&self.point_manager);
                pm.lock().unwrap().add_random_point();
            }
            if ui.add(Button::new("Remove Last Point")).clicked() {
                let pm = Arc::clone(&self.point_manager);
                pm.lock().unwrap().remove_last_point();
            }
            if ui.add(Button::new("Start Random Search")).clicked() {
                let ld = Arc::clone(&self.local_data);
                ld.lock().unwrap().mode = RunMode::GenerateRandom;
            }
            if ui.add(Button::new("Start Random Swap")).clicked() {
                let ld = Arc::clone(&self.local_data);
                ld.lock().unwrap().mode = RunMode::RandomSwap;
            }
            if ui.add(Button::new("Stop")).clicked() {
                let ld = Arc::clone(&self.local_data);
                ld.lock().unwrap().mode = RunMode::None;
            }
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
            // context: cc.egui_ctx.clone(),
            local_data: Arc::new(Mutex::new(LocalData::new(cc.egui_ctx.clone())))
        };

        mw.start_main_loop();
        return mw;
    }

    fn start_main_loop(&mut self) {
        let pm = Arc::clone(&self.point_manager);
        let ld = Arc::clone(&self.local_data);
        // let ctx = self.context.clone();
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

            match local_data.lock().unwrap().mode {
                RunMode::GenerateRandom => {
                    point_manager.lock().unwrap().random_path_step();
                },
                RunMode::RandomSwap => {
                    point_manager.lock().unwrap().random_swap_step(4);
                }
                RunMode::None => {}
            }
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
            .show(ui, |plot_ui| {
                
                //plot nodes
                plot_ui.points(
                    self.generate_points()
                    .shape(MarkerShape::Circle)
                    .filled(true)
                    .radius(3.0)
                );
                
                //Path stuff
                let pm = Arc::clone(&self.point_manager);
                let best_path = pm.lock().unwrap().best_path.clone();
                let current_path = pm.lock().unwrap().current_path.clone();

                //Current Best
                plot_ui.line(plot::Line::new(self.generate_path_values(&best_path))
                    .color(Color32::from_rgb(0, 255, 0))
                );

                //Current Try
                plot_ui.line(
                    plot::Line::new(self.generate_path_values(&current_path))
                );

                //Info text
                //score text
                let score = pm.lock().unwrap().score;
                let score_text = "score: ".to_owned() + &score.to_string(); 
                plot_ui.text(
                    plot::Text::new(PlotPoint::new(-13.5, 14), score_text)
                    .name("Text")

                    
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