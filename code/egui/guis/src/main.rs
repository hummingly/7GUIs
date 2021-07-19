use eframe::{egui, epi};

use guis::{
    circle_drawer::*, counter::*, crud::*, flight_booker::*, temperature_converter::*, timer::*,
};

#[derive(Default)]
struct App {
    counter_model: CounterModel,
    temperature_converter_model: TemperatureConverterModel,
    flight_booker_model: FlightBookerModel,
    timer_model: TimerModel,
    crud_model: CrudModel,
    circle_drawer_model: CircleDrawerModel,
}

impl epi::App for App {
    fn name(&self) -> &str {
        "7GUIs"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::auto_sized().show_viewport(ui, |ui, viewport| {
                ui.heading("7GUIs");
                task_container(ui, "Counter", |ui| counter(ui, &mut self.counter_model));
                task_container(ui, "Temperature Container", |ui| {
                    temperature_converter(ui, &mut self.temperature_converter_model)
                });
                task_container(ui, "Flight Booker", |ui| {
                    flight_booker(ui, &mut self.flight_booker_model)
                });
                task_container(ui, "CRUD", |ui| crud(ui, &mut self.crud_model));
                task_container(ui, "Timer", |ui| timer(ui, &mut self.timer_model));
                task_container(ui, "Circle Drawer", |ui| {
                    circle_drawer(ui, &mut self.circle_drawer_model, viewport)
                });
                task_container(ui, "Cells", |ui| {
                    ui.label("TODO");
                });
            })
        });
    }
}

fn task_container(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui)) {
    ui.group(|ui| {
        ui.heading(title);
        content(ui);
    });
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(App::default()), options);
}
