use chrono::{DateTime, TimeZone, Utc};
use eframe::{
    egui::{self, Color32, Context, Layout, RichText},
    emath::Align,
};

#[derive(Default)]
pub struct ErrorBar {
    pub error: Option<String>,
    time_til_end: Option<DateTime<Utc>>,
}

impl ErrorBar {
    pub fn run(&mut self, ctx: &Context) {
        if self.time_til_end.is_some() {
            if let Some(error) = &self.error {
                egui::TopBottomPanel::bottom("error_bar").show(ctx, |ui| {
                    ui.with_layout(Layout::right_to_left(Align::default()), |ui| {
                        ui.label(
                            RichText::new("Error: ".to_string() + error).color(Color32::LIGHT_RED),
                        );
                    })
                });
            }
            if self.time_til_end.unwrap().timestamp_nanos() < Utc::now().timestamp_nanos() {
                self.time_til_end = None;
                self.error = None;
            }
        }
    }

    pub fn new_error(&mut self, error: String) {
        self.time_til_end = Some(Utc.timestamp(Utc::now().timestamp() + 3, 0));
        self.error = Some(error);
    }
}
