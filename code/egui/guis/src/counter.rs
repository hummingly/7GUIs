use eframe::egui;

pub fn counter(ui: &mut egui::Ui, state: &mut CounterModel) {
    ui.horizontal(|ui| {
        ui.label(format!("{}", state.0));
        if ui.button("Count").clicked() {
            state.0 += 1;
        }
    });
}

#[derive(Default)]
pub struct CounterModel(u32);
