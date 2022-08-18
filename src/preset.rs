#![allow(non_snake_case)]

use crate::TimestampEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Preset<'a> {
    pub ID: Option<&'a str>,
    pub Details: Option<&'a str>,
    pub State: Option<&'a str>,
    pub PartySize: Option<u8>,
    pub PartyMax: Option<u8>,
    pub Timestamps: Option<u8>,
    pub CustomTimestamp: Option<&'a str>,
    pub LargeKey: Option<&'a str>,
    pub LargeText: Option<&'a str>,
    pub SmallKey: Option<&'a str>,
    pub SmallText: Option<&'a str>,
    pub Button1Text: Option<&'a str>,
    pub Button1URL: Option<&'a str>,
    pub Button2Text: Option<&'a str>,
    pub Button2URL: Option<&'a str>,
}

impl Preset<'_> {
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
