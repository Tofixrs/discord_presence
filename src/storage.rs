use serde::{Deserialize, Serialize};

use crate::timestamp::TimestampEnum;
use derivative::Derivative;
#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Default)]
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
    #[derivative(Default(value = "true"))]
    pub darkmode: bool,
}
