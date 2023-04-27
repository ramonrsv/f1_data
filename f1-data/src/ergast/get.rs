use ureq;

use serde_json::{Result, Value};

use crate::ergast::orm::Driver;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_driver_all_fields_present() {
        let body: String = ureq::get("http://ergast.com/api/f1/drivers/alonso.json?")
            .call()
            .unwrap()
            .into_string()
            .unwrap();

        let json: Value = serde_json::from_str(&body).unwrap();

        let driver = json["MRData"]["DriverTable"]["Drivers"][0].clone();
        let driver: Driver = serde_json::from_value(driver).unwrap();

        assert_eq!(driver.driver_id, String::from("alonso"));
        assert_eq!(driver.permanent_number, Some(String::from("14")));
        assert_eq!(driver.code, Some(String::from("ALO")));
        assert_eq!(driver.url, String::from("http://en.wikipedia.org/wiki/Fernando_Alonso"));
        assert_eq!(driver.given_name, String::from("Fernando"));
        assert_eq!(driver.family_name, String::from("Alonso"));
        assert_eq!(driver.date_of_birth, String::from("1981-07-29"));
        assert_eq!(driver.nationality, String::from("Spanish"));
    }

    #[test]
    fn get_driver_some_fields_missing() {
        let body: String = ureq::get("http://ergast.com/api/f1/drivers/abate.json?")
            .call()
            .unwrap()
            .into_string()
            .unwrap();

        let json: Value = serde_json::from_str(&body).unwrap();

        let driver = json["MRData"]["DriverTable"]["Drivers"][0].clone();
        let driver: Driver = serde_json::from_value(driver).unwrap();

        assert_eq!(driver.driver_id, String::from("abate"));
        assert_eq!(driver.permanent_number, None);
        assert_eq!(driver.code, None);
        assert_eq!(driver.url, String::from("http://en.wikipedia.org/wiki/Carlo_Mario_Abate"));
        assert_eq!(driver.given_name, String::from("Carlo"));
        assert_eq!(driver.family_name, String::from("Abate"));
        assert_eq!(driver.date_of_birth, String::from("1932-07-10"));
        assert_eq!(driver.nationality, String::from("Italian"));
    }
}
