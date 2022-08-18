use crate::preset::Preset;
use eframe::egui::{self, Ui};
use rfd::FileDialog;
use std::{fs, path::PathBuf, process::exit};

pub struct MenuBar {
    pub run_on_startup: bool,
    pub start_minimized: bool,
    pub autoconnect: bool,
    pub check_updates: bool,
    pub darkmode: bool,
    pub about_me: bool,
    pub loaded_preset: Option<Preset<'static>>,
}

impl Default for MenuBar {
    fn default() -> Self {
        Self {
            run_on_startup: false,
            start_minimized: false,
            autoconnect: false,
            check_updates: true,
            darkmode: true,
            about_me: false,
            loaded_preset: None,
        }
    }
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
    fn file(&mut self, ui: &mut Ui) {
        ui.menu_button("File", |ui| {
            if ui.button("Load Preset | Ctrl + O").clicked() {
                self.load_preset();
            }
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
            self.darkmode = ui.ctx().style().visuals.clone().dark_mode
        });
    }
    fn help(&mut self, ui: &mut Ui) {
        ui.menu_button("Help", |ui| {
            ui.hyperlink_to("Github Page", "https://github.com/Tofix26/discord_presence");
            if ui.button("About").clicked() {
                self.about_me = true
            }
        });
    }
    fn load_preset(&mut self) {
        let file = match FileDialog::new()
            .add_filter("Preset", &["crp"])
            .set_directory("/")
            .set_title("Load preset")
            .pick_file()
        {
            None => PathBuf::new(),
            Some(path) => path,
        };
        if file.to_str().unwrap() != "" {
            let file = fs::read_to_string(file).unwrap();
            let xml: Preset = serde_xml_rs::from_str(&file).unwrap();
            self.loaded_preset = Some(xml);
        }
    }
}
