use std::collections::HashMap;
use std::hash::Hash;

use yaml_rust::{Yaml, YamlLoader};

use crate::id::{ConstructorID, DriverID, RoundID};

type Price = f32;

type IdPriceMap<ID> = HashMap<ID, HashMap<RoundID, Price>>;
type StrPriceMap<'a> = HashMap<&'a str, Vec<Price>>;

#[derive(Debug)]
pub struct SeasonPrices {
    drivers: IdPriceMap<DriverID>,
    constructors: IdPriceMap<ConstructorID>,
}

impl SeasonPrices {
    /// Returns an iterator over all the drivers for which there is price information
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// use f1_data::{id::DriverID, fantasy::prices::SeasonPrices};
    ///
    /// let season_prices = SeasonPrices::from_str_price_map(
    ///     &HashMap::from([("max_verstappen", vec![1.0])]),
    ///     &HashMap::new(),
    /// );
    ///
    /// assert_eq!(
    ///     season_prices.drivers().collect::<Vec<_>>(),
    ///     vec![&DriverID::from("max_verstappen")]
    /// );
    /// ```
    pub fn drivers(&self) -> impl Iterator<Item = &DriverID> {
        self.drivers.keys()
    }

    /// Returns an iterator over all the constructors for which there is price information
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// use f1_data::{id::ConstructorID, fantasy::prices::SeasonPrices};
    ///
    /// let season_prices = SeasonPrices::from_str_price_map(
    ///     &HashMap::new(),
    ///     &HashMap::from([("red_bull", vec![1.0])]),
    /// );
    ///
    /// assert_eq!(
    ///     season_prices.constructors().collect::<Vec<_>>(),
    ///     vec![&ConstructorID::from("red_bull")]
    /// );
    /// ```
    pub fn constructors(&self) -> impl Iterator<Item = &ConstructorID> {
        self.constructors.keys()
    }

    pub fn driver_price(&self, driver: &DriverID, round: &RoundID) -> Option<Price> {
        Some(*self.drivers.get(driver)?.get(round)?)
    }

    pub fn constructor_price(&self, constructor: &ConstructorID, round: &RoundID) -> Option<Price> {
        Some(*self.constructors.get(constructor)?.get(round)?)
    }
}

impl SeasonPrices {
    fn str_price_map_to_id_price_map<'a, ID>(map: &StrPriceMap<'a>) -> IdPriceMap<ID>
    where
        ID: From<&'a str> + Eq + Hash,
    {
        map.iter()
            .map(|(id, prices)| {
                (
                    ID::from(id),
                    prices
                        .iter()
                        .enumerate()
                        .map(|(idx, price)| (RoundID::from(idx as u32 + 1), *price))
                        .collect::<HashMap<_, _>>(),
                )
            })
            .collect::<HashMap<_, _>>()
    }

    pub fn from_str_price_map(drivers: &StrPriceMap, constructors: &StrPriceMap) -> SeasonPrices {
        SeasonPrices {
            drivers: Self::str_price_map_to_id_price_map(drivers),
            constructors: Self::str_price_map_to_id_price_map(constructors),
        }
    }
}

impl SeasonPrices {
    fn yaml_price_map_to_id_price_map<'a, ID>(yaml: &'a Yaml) -> IdPriceMap<ID>
    where
        ID: From<&'a str> + Eq + Hash,
    {
        yaml.as_hash()
            .unwrap()
            .into_iter()
            .map(|(id, prices)| {
                (
                    ID::from(id.as_str().unwrap()),
                    prices
                        .as_vec()
                        .unwrap()
                        .iter()
                        .map(|price| price.as_f64().unwrap())
                        .enumerate()
                        .map(|(idx, price)| (RoundID::from(idx as u32 + 1), price as Price))
                        .collect::<HashMap<_, _>>(),
                )
            })
            .collect::<HashMap<_, _>>()
    }

    pub fn load_from_yaml_str(yaml: &str) -> SeasonPrices {
        const DRIVER_KEY: &str = "driver";
        const CONSTRUCTOR_KEY: &str = "constructor";

        let docs = YamlLoader::load_from_str(yaml).unwrap();
        assert!(docs.len() == 1);

        SeasonPrices {
            drivers: Self::yaml_price_map_to_id_price_map(&docs[0][DRIVER_KEY]),
            constructors: Self::yaml_price_map_to_id_price_map(&docs[0][CONSTRUCTOR_KEY]),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use lazy_static::lazy_static;

    use super::*;

    const VER_STR: &str = "max_verstappen";
    const HAM_STR: &str = "hamilton";

    const REDB_STR: &str = "red_bull";
    const MERC_STR: &str = "mercedes";

    lazy_static! {
        static ref VER_ID: DriverID = DriverID::from(VER_STR);
        static ref HAM_ID: DriverID = DriverID::from(HAM_STR);
        static ref REDB_ID: ConstructorID = ConstructorID::from(REDB_STR);
        static ref MERC_ID: ConstructorID = ConstructorID::from(MERC_STR);
        static ref DRIVERS: Vec<DriverID> = vec![VER_ID.clone(), HAM_ID.clone()];
        static ref CONSTRUCTORS: Vec<ConstructorID> = vec![REDB_ID.clone(), MERC_ID.clone()];
    }

    lazy_static! {
        static ref DRIVER_PRICES: HashMap<DriverID, Vec<Price>> = HashMap::from([
            (VER_ID.clone(), vec![26.9, 27.0, 27.1, 27.2]),
            (HAM_ID.clone(), vec![23.7, 23.7, 23.7, 23.8]),
        ]);
        static ref CONSTRUCTOR_PRICES: HashMap<ConstructorID, Vec<Price>> = HashMap::from([
            (REDB_ID.clone(), vec![27.2, 27.3, 27.4, 27.5]),
            (MERC_ID.clone(), vec![25.1, 25.1, 25.1, 25.1])
        ]);
    }

    fn as_hashset<'a, ID, T>(iter: T) -> HashSet<ID>
    where
        ID: Clone + Eq + Hash + 'a,
        T: Iterator<Item = &'a ID>,
    {
        iter.cloned().collect::<HashSet<_>>()
    }

    #[test]
    fn season_prices_validate_test_data() {
        assert_eq!(DRIVERS.len(), 2);
        assert!(DRIVER_PRICES.contains_key(&VER_ID));
        assert!(DRIVER_PRICES.contains_key(&HAM_ID));
        assert_eq!(DRIVER_PRICES.get(&VER_ID).unwrap().len(), 4);
        assert_eq!(DRIVER_PRICES.get(&HAM_ID).unwrap().len(), 4);

        assert_eq!(CONSTRUCTORS.len(), 2);
        assert!(CONSTRUCTOR_PRICES.contains_key(&REDB_ID));
        assert!(CONSTRUCTOR_PRICES.contains_key(&MERC_ID));
        assert_eq!(CONSTRUCTOR_PRICES.get(&REDB_ID).unwrap().len(), 4);
        assert_eq!(CONSTRUCTOR_PRICES.get(&MERC_ID).unwrap().len(), 4);
    }

    fn validate_drivers(season_prices: &SeasonPrices) {
        assert_eq!(as_hashset(season_prices.drivers()), as_hashset(DRIVERS.iter()));
    }

    fn validate_constructors(season_prices: &SeasonPrices) {
        assert_eq!(
            as_hashset(season_prices.constructors()),
            as_hashset(CONSTRUCTORS.iter())
        );
    }

    fn validate_driver_price(season_prices: &SeasonPrices) {
        for driver in DRIVERS.iter() {
            for (idx, price) in DRIVER_PRICES.get(driver).unwrap().iter().enumerate() {
                assert_eq!(
                    season_prices
                        .driver_price(driver, &RoundID::from(idx as u32 + 1))
                        .unwrap(),
                    price.clone()
                );
            }
        }
    }

    fn validate_constructor_price(season_prices: &SeasonPrices) {
        for constructor in CONSTRUCTORS.iter() {
            for (idx, price) in CONSTRUCTOR_PRICES.get(constructor).unwrap().iter().enumerate() {
                assert_eq!(
                    season_prices
                        .constructor_price(constructor, &RoundID::from(idx as u32 + 1))
                        .unwrap(),
                    price.clone()
                );
            }
        }
    }

    #[test]
    fn season_prices_from_str_price_map_happy_path() {
        let season_prices = SeasonPrices::from_str_price_map(
            &HashMap::from([
                (VER_STR, DRIVER_PRICES.get(&VER_ID).unwrap().clone()),
                (HAM_STR, DRIVER_PRICES.get(&HAM_ID).unwrap().clone()),
            ]),
            &HashMap::from([
                (REDB_STR, CONSTRUCTOR_PRICES.get(&REDB_ID).unwrap().clone()),
                (MERC_STR, CONSTRUCTOR_PRICES.get(&MERC_ID).unwrap().clone()),
            ]),
        );

        validate_drivers(&season_prices);
        validate_constructors(&season_prices);
        validate_driver_price(&season_prices);
        validate_constructor_price(&season_prices);
    }

    #[test]
    fn season_prices_load_from_yaml_str_happy_path() {
        let season_prices = SeasonPrices::load_from_yaml_str(
            "
            driver:
                max_verstappen:  [26.9, 27.0, 27.1, 27.2]
                hamilton:        [23.7, 23.7, 23.7, 23.8]

            constructor:
                red_bull:        [27.2, 27.3, 27.4, 27.5]
                mercedes:        [25.1, 25.1, 25.1, 25.1]
            ",
        );

        validate_drivers(&season_prices);
        validate_constructors(&season_prices);
        validate_driver_price(&season_prices);
        validate_constructor_price(&season_prices);
    }
}
