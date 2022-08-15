use eframe::egui::{self, Ui};
use std::process::exit;

pub struct MenuBar {
    pub run_on_startup: bool,
    pub start_minimized: bool,
    pub autoconnect: bool,
    pub check_updates: bool,
}
impl MenuBar {
    pub fn run(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                self.file(ui);
                self.settings(ui);
                self.help(ui);
            })
        });
    }
    fn file(&self, ui: &mut Ui) {
        ui.menu_button("File", |ui| {
            if ui.button("Load Preset | Ctrl + O").clicked() {}
            if ui.button("Save Preset | Ctrl + S").clicked() {}
            if ui.button("Upload Assets | Ctrl + U").clicked() {}
            if ui.button("Exit | Alt + F4").clicked() {
                exit(0)
            }
        });
    }
    fn settings(&mut self, ui: &mut Ui) {
        ui.menu_button("Settings", |ui| {
            ui.checkbox(&mut self.run_on_startup, "Run on startup");
            ui.checkbox(&mut self.start_minimized, "Start minimized");
            ui.checkbox(&mut self.autoconnect, "Autoconnect");
            ui.checkbox(&mut self.check_updates, "Check for updates");
            egui::widgets::global_dark_light_mode_buttons(ui);
        });
    }
    fn help(&mut self, ui: &mut Ui) {
        ui.menu_button("Help", |ui| {
            ui.hyperlink_to("Github Page", "https://github.com/Tofix26/discord_presence");
        });
    }
}
