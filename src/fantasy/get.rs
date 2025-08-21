use crate::{error::Result, fantasy::price::Season, id::SeasonID};

#[cfg(doc)]
use crate::error::Error;

/// Get a list of fantasy league prices for all drivers and constructors for all rounds of a given
/// season. An [`Error::Io`] is returned if data is not available for the requested season.
///
/// # Examples
///
/// ```
/// use f1_data::fantasy::get::get_season_prices;
///
/// let season_prices = get_season_prices(2023).unwrap();
/// assert!(season_prices.rounds.len() >= 13);
///
/// let first_round = &season_prices.rounds[0];
/// assert_eq!(first_round.round, 1);
///
/// let max_opening_price = first_round
///     .drivers
///     .iter()
///     .find(|d| d.driver_id == "max_verstappen")
///     .unwrap()
///     .price;
///
/// let red_bull_opening_price = first_round
///     .constructors
///     .iter()
///     .find(|c| c.constructor_id == "red_bull")
///     .unwrap()
///     .price;
///
/// assert_eq!(max_opening_price, 26.9);
/// assert_eq!(red_bull_opening_price, 27.2);
/// ```
pub fn get_season_prices(season: SeasonID) -> Result<Season> {
    serde_yaml::from_str(&std::fs::read_to_string(format!("./src/fantasy/data/prices/{season}.yaml"))?).map_err(into)
}

/// Shorthand for closure `|e| e.into()` and/or `std::convert::Into::into`.
fn into<T: Into<U>, U>(t: T) -> U {
    t.into()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn get_season_prices() {
        let season_prices_2023 = super::get_season_prices(2023).unwrap();
        assert!(season_prices_2023.rounds.len() >= 13);

        for (idx, round) in season_prices_2023.rounds.iter().enumerate() {
            assert_eq!(round.round, idx as u32 + 1);

            assert_eq!(round.drivers.len(), 20);
            assert_eq!(round.constructors.len(), 10);

            for driver in &round.drivers {
                assert!(driver.price > 0.0);
            }

            for constructor in &round.constructors {
                assert!(constructor.price > 0.0);
            }
        }
    }

    #[test]
    fn get_season_prices_err() {
        assert!(super::get_season_prices(1949).is_err());
    }
}
