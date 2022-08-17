#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod image;
mod menu_bar;
mod presence_button;
mod timestamp;
use std::time::Duration;
use std::vec;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use timestamp::TimestampEnum;

use discord_rich_presence::activity::{Activity, Assets, Button, Party, Timestamps};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};

use chrono::{DateTime, Local, Utc};
use eframe::egui::{self, Layout, Vec2};
use eframe::emath::Align;
use eframe::{run_native, NativeOptions};

fn main() {
    let options = NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: Some(Vec2::new(600.0, 650.0)),
        min_window_size: None,
        max_window_size: None,
        resizable: false,
        transparent: false,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
    };
    run_native(
        "Discord Presence",
        options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}

struct App {
    menu_bar: menu_bar::MenuBar,
    first_btn: presence_button::PresenceButton,
    second_btn: presence_button::PresenceButton,
    first_img: image::Image,
    second_img: image::Image,
    id: String,
    details: String,
    state: String,
    party: u8,
    party_of: u8,
    timestamp: timestamp::Timestamp,
    client: DiscordIpcClient,
    connected: bool,
    started: DateTime<Utc>,
    last_update: DateTime<Utc>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            menu_bar: menu_bar::MenuBar {
                run_on_startup: true,
                start_minimized: true,
                autoconnect: true,
                check_updates: true,
            },
            first_btn: presence_button::PresenceButton {
                label: "".to_string(),
                url: "".to_string(),
            },
            second_btn: presence_button::PresenceButton {
                label: "".to_string(),
                url: "".to_string(),
            },
            first_img: image::Image {
                key: "".to_string(),
                text: "".to_string(),
            },
            second_img: image::Image {
                key: "".to_string(),
                text: "".to_string(),
            },
            id: "".to_string(),
            details: "".to_string(),
            party: 0,
            party_of: 0,
            state: "".to_string(),
            timestamp: timestamp::Timestamp {
                timestamp: timestamp::TimestampEnum::None,
                date: Utc::now().date(),
            },
            client: DiscordIpcClient::new("0").unwrap(),
            connected: false,
            started: Utc::now(),
            last_update: Utc::now(),
        }
    }
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let storage = match cc.storage.unwrap().get_string("settings") {
            None => "".to_string(),
            Some(value) => value,
        };
        let storage: Storage = match from_str(&storage) {
            Ok(storage) => storage,
            Err(_) => Storage::default(),
        };
        App {
            id: storage.id,
            details: storage.details,
            state: storage.state,
            party: storage.party,
            party_of: storage.party_of,
            timestamp: timestamp::Timestamp {
                timestamp: storage.timestamp,
                date: Utc::now().date(),
            },
            first_btn: presence_button::PresenceButton {
                label: storage.first_btn_label,
                url: storage.first_btn_url,
            },
            second_btn: presence_button::PresenceButton {
                label: storage.second_btn_label,
                url: storage.second_btn_url,
            },
            first_img: image::Image {
                key: storage.large_image_key,
                text: storage.large_image_label,
            },
            second_img: image::Image {
                key: storage.small_image_key,
                text: storage.small_image_label,
            },
            ..Default::default()
        }
    }
}

//rust analyzer fuck off it compiles correctly
#[derive(Serialize, Deserialize)]
struct Storage {
    id: String,
    details: String,
    state: String,
    party: u8,
    party_of: u8,
    timestamp: TimestampEnum,
    large_image_key: String,
    small_image_key: String,
    large_image_label: String,
    small_image_label: String,
    first_btn_label: String,
    second_btn_label: String,
    first_btn_url: String,
    second_btn_url: String,
}

impl Storage {
    fn new(
        id: &str,
        details: &str,
        state: &str,
        party: u8,
        party_of: u8,
        timestamp: TimestampEnum,
        large_image_key: &str,
        small_image_key: &str,
        large_image_label: &str,
        small_image_label: &str,
        first_btn_label: &str,
        second_btn_label: &str,
        first_btn_url: &str,
        second_btn_url: &str,
    ) -> Self {
        Storage {
            id: id.to_string(),
            details: details.to_string(),
            state: state.to_string(),
            party,
            party_of,
            timestamp,
            large_image_key: large_image_key.to_string(),
            small_image_key: small_image_key.to_string(),
            large_image_label: large_image_label.to_string(),
            small_image_label: small_image_label.to_string(),
            first_btn_label: first_btn_label.to_string(),
            second_btn_label: second_btn_label.to_string(),
            first_btn_url: first_btn_url.to_string(),
            second_btn_url: second_btn_url.to_string(),
        }
    }
}

impl Default for Storage {
    fn default() -> Self {
        Storage {
            id: "".to_string(),
            details: "".to_string(),
            state: "".to_string(),
            party: 0,
            party_of: 0,
            timestamp: TimestampEnum::None,
            large_image_key: "".to_string(),
            small_image_key: "".to_string(),
            large_image_label: "".to_string(),
            small_image_label: "".to_string(),
            first_btn_label: "".to_string(),
            second_btn_label: "".to_string(),
            first_btn_url: "".to_string(),
            second_btn_url: "".to_string(),
        }
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        let save = Storage::new(
            &self.id,
            &self.details,
            &self.state,
            self.party,
            self.party_of,
            self.timestamp.timestamp,
            &self.first_img.key,
            &self.second_img.key,
            &self.first_img.text,
            &self.second_img.text,
            &self.first_btn.label,
            &self.second_btn.label,
            &self.first_btn.url,
            &self.second_btn.url,
        );
        storage.set_string(
            "settings",
            to_string(&save).expect("Failed to parse save struct"),
        );
    }
    fn auto_save_interval(&self) -> std::time::Duration {
        Duration::from_secs(5)
    }
    fn persist_egui_memory(&self) -> bool {
        true
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.menu_bar.run(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                ui.heading("Discord Presence");
            });
            ui.horizontal(|ui| {
                ui.add_space(60.);
                ui.label("ID");
                ui.text_edit_singleline(&mut self.id);
                ui.add_space(10.);
                if ui
                    .add_enabled(!self.connected, egui::Button::new("Connect"))
                    .clicked()
                {
                    if self.id != "".to_string() {
                        self.client = DiscordIpcClient::new(&self.id).expect("sus");
                        self.client.connect().expect("Failed to connect");
                        self.last_update = Utc::now();
                        self.set_presence();
                        self.connected = true;
                    }
                }
                ui.add_space(10.);
                if ui
                    .add_enabled(self.connected, egui::Button::new("Disconnect"))
                    .clicked()
                {
                    self.client.close().expect("Failed to disconnect");
                    self.connected = false;
                }
            });
            ui.add_space(5.);
            ui.horizontal(|ui| {
                ui.add_space(34.);
                ui.label("Details");
                ui.text_edit_singleline(&mut self.details);
            });
            ui.add_space(5.);
            ui.horizontal(|ui| {
                ui.add_space(42.);
                ui.label("State");
                ui.text_edit_singleline(&mut self.state);
                ui.label("Party");
                ui.add(egui::DragValue::new(&mut self.party).clamp_range(0..=32));
                ui.label("of");
                ui.add(egui::DragValue::new(&mut self.party_of).clamp_range(1..=32));
            });
            ui.add_space(15.);
            self.timestamp.run(ui);
            ui.add_space(15.);
            ui.horizontal(|ui| {
                ui.add_space(75.);
                self.first_img.run(ui, "Large Image");
                self.second_img.run(ui, "Small Image");
            });
            ui.horizontal(|ui| {
                ui.add_space(75.);
                self.first_btn.run(ui, "Button 1");
                self.second_btn.run(ui, "Button 2");
            });
            ui.add_space(50.);
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                if ui
                    .add_enabled(
                        self.connected,
                        egui::widgets::Button::new("Update Presence"),
                    )
                    .clicked()
                {
                    self.last_update = Utc::now();
                    self.set_presence()
                }
            });
        });
    }
}
impl App {
    fn set_presence(&mut self) {
        let first_btn = Button::new(&self.first_btn.label, &self.first_btn.url);
        let second_btn = Button::new(&self.second_btn.label, &self.second_btn.url);
        let mut buttons = vec![];
        let timestamp = match self.timestamp.timestamp {
            TimestampEnum::LocalTime => {
                let hour = Local::now()
                    .format("%H")
                    .to_string()
                    .parse::<i64>()
                    .unwrap()
                    * 3_600;
                let minute = Local::now()
                    .format("%M")
                    .to_string()
                    .parse::<i64>()
                    .unwrap()
                    * 60;
                let second = Local::now()
                    .format("%S")
                    .to_string()
                    .parse::<i64>()
                    .unwrap();
                let local_time = Utc::now().timestamp() - (hour + minute + second);
                Timestamps::new().start(local_time)
            }
            TimestampEnum::CustomTimeStamp => Timestamps::new()
                .start(self.timestamp.date.naive_utc().and_hms(0, 0, 0).timestamp()),
            TimestampEnum::SinceStart => Timestamps::new().start(self.started.timestamp()),
            TimestampEnum::SinceLastUpdate => Timestamps::new().start(self.last_update.timestamp()),
            _ => Timestamps::new(),
        };
        let assets = Assets::new();
        let assets = match self.first_img.key.as_str() {
            "" => assets,
            _ => assets.large_image(&self.first_img.key),
        };
        let assets = match self.first_img.text.as_str() {
            "" => assets,
            _ => assets.large_text(&self.first_img.text),
        };
        let assets = match self.second_img.key.as_str() {
            "" => assets,
            _ => assets.small_image(&self.second_img.key),
        };
        let assets = match self.second_img.text.as_str() {
            "" => assets,
            _ => assets.small_text(&self.second_img.text),
        };
        let activity = Activity::new().timestamps(timestamp).assets(assets);

        let activity = match self.details.as_str() {
            "" => activity,
            _ => activity.details(&self.details),
        };

        let activity = match self.state.as_str() {
            "" => activity,
            _ => activity.state(&self.state),
        };
        let first_btn_label_exists = self.first_btn.label != "".to_string();
        let first_btn_url_exists = self.first_btn.url != "".to_string();
        if first_btn_label_exists && first_btn_url_exists {
            buttons.push(first_btn);
        }

        let second_btn_label_exists = self.second_btn.label != "".to_string();
        let second_btn_url_exists = self.second_btn.url != "".to_string();
        if second_btn_label_exists && second_btn_url_exists {
            buttons.push(second_btn);
        }

        let activity = match buttons.len() > 0 {
            true => activity.buttons(buttons),
            false => activity,
        };

        let part_exists = self.party != 0;
        let activity = match part_exists {
            true => activity.party(Party::new().size([self.party_of as i32, self.party as i32])),
            false => activity,
        };
        self.client
            .set_activity(activity)
            .expect("Failed to set activity");
    }
}
