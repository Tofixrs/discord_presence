use serde::{Deserialize, Serialize};

use crate::timestamp::TimestampEnum;

#[derive(Serialize, Deserialize, Default)]
pub struct Storage {
    pub id: String,
    pub details: String,
    pub state: String,
    pub party: u8,
    pub party_of: u8,
    pub timestamp: TimestampEnum,
    pub large_image_key: String,
    pub small_image_key: String,
    pub large_image_label: String,
    pub small_image_label: String,
    pub first_btn_label: String,
    pub second_btn_label: String,
    pub first_btn_url: String,
    pub second_btn_url: String,
    pub autoconnect: bool,
    pub darkmode: bool,
}

impl Storage {
    pub fn new(
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
        autoconnect: bool,
        darkmode: bool,
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
            autoconnect,
            darkmode,
        }
    }
}
