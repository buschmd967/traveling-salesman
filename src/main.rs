mod point_manager;
mod point;

use eframe::{App, run_native, egui::{Color32, Ui, Button, plot::{self, MarkerShape}, plot::Plot, CentralPanel, Visuals, TopBottomPanel}};
use point_manager::PointManager;
use std::thread;


enum RunMode {
    None,
    GenerateRandom
} 

struct MainWindow{ 
    point_manager:PointManager,
    mode:RunMode
}

impl App for MainWindow {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::dark());

        CentralPanel::default().show(ctx, |ui| {
            self.graph(ui);
        });

        TopBottomPanel::bottom("ButtonPanel").show(ctx, |ui| {
            if ui.add(Button::new("Add Random Point")).clicked() {
                self.point_manager.add_random_point();
            }
            if ui.add(Button::new("Remove Last Point")).clicked() {
                self.point_manager.remove_last_point();
            }
            if ui.add(Button::new("Start Random Search")).clicked() {
                self.mode = RunMode::GenerateRandom;
            }
            if ui.add(Button::new("Stop")).clicked() {
                self.mode = RunMode::None;
            }
        });

    }
}

impl Default for MainWindow {

    fn default() -> Self {
        Self {
            point_manager: PointManager::default(),
            mode: RunMode::None
        }
    }


}

impl MainWindow{

    fn main_loop(&mut self) {

        loop{
            match self.mode {
                RunMode::GenerateRandom => self.point_manager.random_path_step(),
                RunMode::None => {}
            }
        }

    }
    
    fn graph(&self, ui: &mut Ui) {

        let x = 15.0;
        let y = 15.0;

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
                plot_ui.points(
                    self.generate_points()
                    .shape(MarkerShape::Circle)
                    .filled(true)
                    .radius(3.0)
                );

                //Current Best
                plot_ui.line(plot::Line::new(self.generate_path_values(&self.point_manager.best_path))
                    .color(Color32::from_rgb(0, 255, 0))
                );

                //Current Try
                plot_ui.line(
                    plot::Line::new(self.generate_path_values(&self.point_manager.current_path))
                );

            });
    }

    fn generate_path_values(&self, points: &Vec<traveling_salesman::point::Point>) -> plot::Values {
        let mut values_vector = Vec::<plot::Value>::new();

        for (point) in points {
            values_vector.push(point.to_value());
        }

        match values_vector.get(0) {
            Some(v) => values_vector.push(v.clone()),
            None => {}
        }

        return plot::Values::from_values(values_vector);
    }

    fn generate_points(&self) -> plot::Points {


        let mut values_vector = Vec::<plot::Value>::new();

        for (point) in &self.point_manager.points {
            values_vector.push(point.to_value());
        }

        let values =  plot::Values::from_values(values_vector);
        return plot::Points::new(values);
    }
}

fn main(){

    let options = eframe::NativeOptions {
        #[cfg(feature = "wgpu")]
        renderer: eframe::Renderer::Wgpu,

        ..Default::default()
    };

    let mut main_window = MainWindow::default();

    thread::spawn(|| {
        run_native(
            "Traveling Salesman",
            options,
            Box::new(|_cc| Box::new(main_window))
        );
    });

    // main_window.main_loop();
}