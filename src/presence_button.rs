use eframe::egui::Ui;

#[derive(Default)]
pub struct PresenceButton {
    pub label: String,
    pub url: String,
}

impl PresenceButton {
    pub fn run(&mut self, ui: &mut Ui, label: &str) {
        ui.vertical(|ui| {
            ui.set_max_width(200.);
            ui.horizontal(|ui| {
                ui.add_space(40.);
                ui.label(label);
            });
            ui.add_space(2.);
            ui.horizontal(|ui| {
                ui.label("Label");
                ui.text_edit_singleline(&mut self.label)
            });
            ui.add_space(2.);
            ui.horizontal(|ui| {
                ui.label("URL");
                ui.add_space(7.);
                ui.text_edit_singleline(&mut self.url)
            })
        });
    }
}
