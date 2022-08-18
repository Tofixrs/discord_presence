#![allow(non_snake_case)]

use crate::TimestampEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
    pub fn timestamp(&self) -> TimestampEnum {
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
}
