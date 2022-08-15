use eframe::egui::Ui;

pub struct Image {
    pub key: String,
    pub text: String,
}

impl Image {
    pub fn run(&mut self, ui: &mut Ui, label: &str) {
        ui.vertical(|ui| {
            ui.set_max_width(200.);
            ui.horizontal(|ui| {
                ui.add_space(40.);
                ui.label(label);
            });
            ui.horizontal(|ui| {
                ui.label("Label");
                ui.text_edit_singleline(&mut self.text)
            });
            ui.horizontal(|ui| {
                ui.label("Key");
                ui.add_space(9.);
                ui.text_edit_singleline(&mut self.key)
            })
        });
    }
}
