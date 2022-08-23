use crate::preset::Preset;
use eframe::{
    egui::{self, Layout, Ui},
    emath::Align,
    epaint::Vec2,
};
use rfd::FileDialog;
use std::{fs, path::PathBuf, process::exit};

pub struct MenuBar {
    pub run_on_startup: bool,
    pub start_minimized: bool,
    pub autoconnect: bool,
    pub check_updates: bool,
    pub darkmode: bool,
    pub about_me: bool,
    pub loaded_preset: Option<Preset>,
    pub preset_save_location: Option<PathBuf>,
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
            preset_save_location: None,
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

        //about me
        egui::containers::Window::new("About")
            .open(&mut self.about_me)
            .resizable(false)
            .fixed_size(Vec2::new(200., 100.))
            .show(ctx, |ui| {
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.heading("Discord Presence");
                    ui.label("Version v0.5.1-beta");
                });
            });
    }
    fn file(&mut self, ui: &mut Ui) {
        ui.menu_button("File", |ui| {
            if ui.button("Load Preset | Ctrl + O").clicked() {
                self.load_preset();
            }
            if ui.button("Save Preset | Ctrl + S").clicked() {
                self.save_preset();
            }
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
        let file = FileDialog::new()
            .add_filter("Preset", &["crp"])
            .set_directory("/")
            .set_title("Load preset")
            .pick_file();
        if let Some(file) = file {
            let file = fs::read_to_string(file).unwrap();
            let xml: Preset = serde_xml_rs::from_str(&file).unwrap();
            self.loaded_preset = Some(xml);
        }
    }

    fn save_preset(&mut self) {
        let file = FileDialog::new()
            .add_filter("Preset", &["crp"])
            .set_directory("/")
            .set_title("Save Preset")
            .set_file_name("Preset")
            .save_file();
        self.preset_save_location = file;
    }
}
