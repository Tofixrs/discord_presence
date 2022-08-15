use chrono::{Date, Utc};
use eframe::{
    egui::{Layout, Ui},
    emath::Align,
};
use egui_datepicker::DatePicker;

#[derive(PartialEq)]
pub enum TimestampEnum {
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
            ui.radio_value(
                &mut self.timestamp,
                TimestampEnum::CustomTimeStamp,
                "Custom timestamp",
            );
            ui.add_space(5.);
            ui.add(DatePicker::new("datepicker-unique-id", &mut self.date));
            ui.add_space(5.);
        });
    }
}
