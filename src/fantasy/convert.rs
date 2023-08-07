#[cfg(test)]
mod tests {
    const SRC: &str = "./data/fantasy/prices/2023.yaml";
    const DST_YAML: &str = "./src/fantasy/data/prices/2023_temp.yaml";
    const DST_JSON: &str = "./src/fantasy/data/prices/2023_temp.json";
    const STABLE: &str = "./src/fantasy/data/prices/2023.yaml";

    use crate::fantasy::price::{Constructor, Driver, Round, Season};
    use crate::fantasy::season_prices::SeasonPrices;
    use std::fs;

    const NO_PRICE: f32 = 0.0;
    const LAST_ROUND: u32 = 13;

    #[test]
    #[ignore]
    fn convert() {
        let season_prices = SeasonPrices::load_from_yaml_str(&fs::read_to_string(SRC).unwrap());
        let drivers: Vec<_> = season_prices.drivers().collect();
        let constructors: Vec<_> = season_prices.constructors().collect();

        let mut season = Season { rounds: Vec::new() };

        for round in 1..=LAST_ROUND {
            let mut new_round = Round {
                round,
                drivers: drivers
                    .iter()
                    .filter(|driver_id| season_prices.driver_price(driver_id, round).unwrap_or(NO_PRICE) > 0.0)
                    .map(|driver_id| Driver {
                        driver_id: driver_id.to_string(),
                        price: season_prices.driver_price(driver_id, round).unwrap(),
                    })
                    .collect(),
                constructors: constructors
                    .iter()
                    .filter(|constructor_id| {
                        season_prices
                            .constructor_price(constructor_id, round)
                            .unwrap_or(NO_PRICE)
                            > 0.0
                    })
                    .map(|constructor_id| Constructor {
                        constructor_id: constructor_id.to_string(),
                        price: season_prices.constructor_price(constructor_id, round).unwrap(),
                    })
                    .collect(),
            };

            new_round.drivers.sort_by(|a, b| {
                if a.price != b.price {
                    b.price.partial_cmp(&a.price).unwrap()
                } else {
                    a.driver_id.cmp(&b.driver_id)
                }
            });

            new_round.constructors.sort_by(|a, b| {
                if a.price != b.price {
                    b.price.partial_cmp(&a.price).unwrap()
                } else {
                    a.constructor_id.cmp(&b.constructor_id)
                }
            });

            season.rounds.push(new_round);
        }

        fs::write(DST_YAML, serde_yaml::to_string(&season).unwrap()).unwrap();
        fs::write(DST_JSON, serde_json::to_string_pretty(&season).unwrap()).unwrap();

        let season_from_yaml = serde_yaml::from_str::<Season>(&fs::read_to_string(DST_YAML).unwrap()).unwrap();
        let season_from_json = serde_json::from_str::<Season>(&fs::read_to_string(DST_JSON).unwrap()).unwrap();
        let season_stable = serde_yaml::from_str::<Season>(&fs::read_to_string(STABLE).unwrap()).unwrap();
        assert_eq!(season_from_yaml, season);
        assert_eq!(season_from_json, season);
        assert_eq!(season_stable, season);

        assert_eq!(season.rounds.len(), usize::try_from(LAST_ROUND).unwrap());
        assert_eq!(season_stable.rounds.len(), usize::try_from(LAST_ROUND).unwrap());

        for round in season_stable.rounds {
            for driver in round.drivers {
                assert_eq!(season_prices.driver_price(&driver.driver_id, round.round).unwrap(), driver.price);
            }
            for constructor in round.constructors {
                assert_eq!(
                    season_prices
                        .constructor_price(&constructor.constructor_id, round.round)
                        .unwrap(),
                    constructor.price
                );
            }
        }
    }
}
