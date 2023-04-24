use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::id::DriverID;

#[serde_as]
#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Driver {
    #[serde_as(as = "DisplayFromStr")]
    pub driver_id: DriverID,
    pub permanent_number: Option<String>,
    pub code: Option<String>,
    pub url: String,
    pub given_name: String,
    pub family_name: String,
    pub date_of_birth: String,
    pub nationality: String,
}
