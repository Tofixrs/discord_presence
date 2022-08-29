use serde::{Deserialize, Serialize};

use crate::{preset::InAppPreset, timestamp::TimestampEnum};
use derivative::Derivative;
#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Default)]
pub struct Storage<'a> {
    pub id: &'a str,
    pub details: &'a str,
    pub state: &'a str,
    pub party: u8,
    pub party_of: u8,
    pub timestamp: TimestampEnum,
    pub large_image_key: &'a str,
    pub small_image_key: &'a str,
    pub large_image_label: &'a str,
    pub small_image_label: &'a str,
    pub first_btn_label: &'a str,
    pub second_btn_label: &'a str,
    pub first_btn_url: &'a str,
    pub second_btn_url: &'a str,
    pub autoconnect: bool,
    #[derivative(Default(value = "true"))]
    pub darkmode: bool,
    pub preset_switch_1: Option<InAppPreset>,
    pub preset_switch_2: Option<InAppPreset>,
    pub preset_switch_time: u8,
}
