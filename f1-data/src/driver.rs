use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Driver {
    pub driver_id: String,
    pub permanent_number: Option<String>,
    pub code: Option<String>,
    pub url: String,
    pub given_name: String,
    pub family_name: String,
    pub date_of_birth: String,
    pub nationality: String,
}
