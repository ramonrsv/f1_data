use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

/// Uniquely identifies a driver by a string, e.g. `"max_verstappen"` for _Max Verstappen_
pub type DriverID = String;

#[allow(clippy::doc_markdown)] // False positive, complains about "_McLaren_".
/// Uniquely identifies a constructor by a string, e.g. `"mclaren"` for _McLaren_
pub type ConstructorID = String;

/// Uniquely identifies a circuit by a string, e.g. `"spa"` for _Circuit de Spa-Francorchamps_
pub type CircuitID = String;

/// Uniquely identifies a finishing status by a numeric value, e.g. `1` for `"Finished"`
pub type StatusID = u32;

/// Uniquely identifies a season by the numeric year that it took place in, e.g. `2023` for the
/// _2023 FIA Formula One World Championship_
pub type SeasonID = u32;

/// Uniquely identifies a round (race weekend), in a given season, by an index, with `1` being the
/// first round of the season. Note that a round is only unique within a given season, and does not
/// uniquely identify a race in the championship. See [`RaceID`] for a unique race identifier.
pub type RoundID = u32;

/// Uniquely identifies a race by the season that it took place in, and by its round index, e.g.
/// season `2023` round `1` for the first race of the _2023 FIA Formula One World Championship_.
///
/// # Examples
///
/// ```
/// use f1_data::id::RaceID;
///
/// let race_id = RaceID::from(2023, 1);
/// assert_eq!(race_id.season, 2023);
/// assert_eq!(race_id.round, 1);
/// ```
#[serde_as]
#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub struct RaceID {
    /// The season/year that a race took place in, e.g. `2023` for the 2023 season.
    #[serde_as(as = "DisplayFromStr")]
    pub season: SeasonID,
    /// The round index for a race within a season, e.g. `1` for the first race.
    #[serde_as(as = "DisplayFromStr")]
    pub round: RoundID,
}

impl RaceID {
    /// Create a new [`RaceID`] from a season and round.
    pub fn from(season: SeasonID, round: RoundID) -> Self {
        Self { season, round }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn race_id_from_season_and_round() {
        assert_eq!(RaceID { season: 2023, round: 1 }, RaceID::from(2023, 1));

        assert_eq!(RaceID::from(2023, 1).season, 2023);
        assert_eq!(RaceID::from(2023, 1).round, 1);

        assert_eq!(RaceID::from(2023, 1), RaceID::from(2023, 1));
        assert_ne!(RaceID::from(2023, 1), RaceID::from(2022, 1));
        assert_ne!(RaceID::from(2023, 1), RaceID::from(2023, 2));
    }

    #[test]
    fn race_id_deserialize() {
        assert_eq!(
            serde_json::from_str::<RaceID>(
                r#"{
                "season": "2023",
                "round": "4"
              }"#
            )
            .unwrap(),
            RaceID::from(2023, 4)
        );
    }
}
