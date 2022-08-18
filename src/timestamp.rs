use chrono::{Date, Utc};
use eframe::{
    egui::{Layout, Ui},
    emath::Align,
};
// use egui_datepicker::DatePicker;
use serde::{Deserialize, Serialize};

//stfu rust analyzer this code compiles
#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
pub enum TimestampEnum {
    #[default]
    None,
    SinceStart,
    SinceLastUpdate,
    LocalTime,
    CustomTimeStamp,
}
pub struct Timestamp {
    pub timestamp: TimestampEnum,
    pub date: Date<Utc>,
}

impl Default for Timestamp {
    fn default() -> Self {
        Self {
            timestamp: TimestampEnum::default(),
            date: Utc::now().date(),
        }
    }
}

impl Timestamp {
    pub fn run(&mut self, ui: &mut Ui) {
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.heading("Timestamp");
            ui.add_space(5.);
            ui.radio_value(&mut self.timestamp, TimestampEnum::None, "None");
            ui.add_space(5.);
            ui.radio_value(
                &mut self.timestamp,
                TimestampEnum::SinceStart,
                "Since Discord Presence Started",
            );
            ui.add_space(5.);
            ui.radio_value(
                &mut self.timestamp,
                TimestampEnum::SinceLastUpdate,
                "Since Last Presence Update",
            );
            ui.add_space(5.);
            ui.radio_value(
                &mut self.timestamp,
                TimestampEnum::LocalTime,
                "Your local time",
            );
            ui.add_space(5.);
            // ui.radio_value(
            //     &mut self.timestamp,
            //     TimestampEnum::CustomTimeStamp,
            //     "Custom timestamp",
            // );
            ui.add_space(5.);
            // ui.add(DatePicker::new("datepicker-unique-id", &mut self.date).movable(true));
            ui.add_space(5.);
        });
    }
}
