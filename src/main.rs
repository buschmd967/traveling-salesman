mod point_manager;

use eframe::{App, run_native, egui::{CentralPanel, TextEdit, Window, Visuals}};

struct MainWindow{ 
    dummy:f32
}

impl App for MainWindow {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::dark());
    }
}

impl Default for MainWindow {

    fn default() -> Self {
        Self {
            dummy: 1.0
        }
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