mod point_manager;

use eframe::{App, run_native, egui::{Ui, Button, plot::{self, MarkerShape}, plot::Plot, CentralPanel, Visuals, TopBottomPanel}};
use point_manager::PointManager;

struct MainWindow{ 
    point_manager:PointManager
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
        });
    }
}

impl Default for MainWindow {

    fn default() -> Self {
        Self {
            point_manager: PointManager::default()
        }
    }


}

impl MainWindow{
    
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
                    self.generate_values()
                    .shape(MarkerShape::Circle)
                    .filled(true)
                    .radius(3.0)
                )

            });
    }

    fn generate_values(&self) -> plot::Points {


        let mut values_vector = Vec::<plot::Value>::new();

        for (point) in &self.point_manager.points {
            values_vector.push(point.to_value());
        }

        let values =  plot::Values::from_values(values_vector);
        return plot::Points::new(values);
    }
}

fn main(){
    let options = eframe::NativeOptions::default();
    run_native(
        "Traveling Salesman",
        options,
        Box::new(|_cc| Box::new(MainWindow::default()))
    );
}