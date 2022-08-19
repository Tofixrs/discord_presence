#![allow(non_snake_case)]

use crate::{App, TimestampEnum};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Preset {
    pub ID: Option<String>,
    pub Details: Option<String>,
    pub State: Option<String>,
    pub PartySize: Option<u8>,
    pub PartyMax: Option<u8>,
    pub Timestamps: Option<u8>,
    pub CustomTimestamp: Option<String>,
    pub LargeKey: Option<String>,
    pub LargeText: Option<String>,
    pub SmallKey: Option<String>,
    pub SmallText: Option<String>,
    pub Button1Text: Option<String>,
    pub Button1URL: Option<String>,
    pub Button2Text: Option<String>,
    pub Button2URL: Option<String>,
}

impl Preset {
    pub fn timestamp_from_num(&self) -> TimestampEnum {
        if self.Timestamps == None {
            return TimestampEnum::None;
        }
        match self.Timestamps.unwrap() {
            0 => TimestampEnum::None,
            1 => TimestampEnum::SinceStart,
            4 => TimestampEnum::SinceLastUpdate,
            2 => TimestampEnum::LocalTime,
            // Custom TimeStamp
            3 => TimestampEnum::None,
            _ => TimestampEnum::None,
        }
    }

    pub fn from_app(app: &App) -> Self {
        Self {
            ID: Some(app.id.clone()),
            Details: Some(app.details.clone()),
            State: Some(app.state.clone()),
            PartySize: Some(app.party),
            PartyMax: Some(app.party_of),
            Timestamps: Some(app.timestamp.timestamp.to_num()),
            CustomTimestamp: None,
            LargeKey: Some(app.first_img.key.clone()),
            LargeText: Some(app.first_img.text.clone()),
            SmallKey: Some(app.second_img.key.clone()),
            SmallText: Some(app.second_img.text.clone()),
            Button1Text: Some(app.first_btn.label.clone()),
            Button1URL: Some(app.first_btn.url.clone()),
            Button2Text: Some(app.second_btn.label.clone()),
            Button2URL: Some(app.second_btn.url.clone()),
        }
    }

    pub fn to_xml(&self) -> String {
        let file = serde_xml_rs::to_string(self).unwrap();
        let (_, xml) = file.split_at(8);
        let xml = r#"<?xml version="1.0"?><Preset xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">"#.to_string()
                + xml;

        xml.replace("<CustomTimestamp></CustomTimestamp>", "")
    }
}
