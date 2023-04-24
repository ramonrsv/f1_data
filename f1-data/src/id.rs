use std::str::FromStr;
use void::Void;

/// Uniquely identifies a driver by a string, e.g. "max_verstappen" for Max Verstappen
///
/// # Examples
///
/// ```
/// use f1_data::id::DriverID;
///
/// let max = DriverID::from("max_verstappen");
/// assert_eq!(max.id, "max_verstappen".to_string());
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DriverID {
    pub id: String,
}

/// Uniquely identifies a constructor by a string, e.g. "mclaren" for McLaren
///
/// # Examples
///
/// ```
/// use f1_data::id::ConstructorID;
///
/// let mclaren = ConstructorID::from("mclaren");
/// assert_eq!(mclaren.id, "mclaren".to_string());
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ConstructorID {
    pub id: String,
}

/// Uniquely identifies a circuit by a string, e.g. "spa" for Circuit de Spa-Francorchamps
///
/// # Examples
///
/// ```
/// use f1_data::id::CircuitID;
///
/// let spa = CircuitID::from("spa");
/// assert_eq!(spa.id, "spa".to_string());
/// ```
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CircuitID {
    pub id: String,
}

type Year = u32;
type Round = u32;

/// Uniquely identifies a season by a numeric year, e.g. 2023 for the 2023 FIA Formula One World Championship
///
/// # Examples
///
/// ```
/// use f1_data::id::SeasonID;
///
/// let s2023 = SeasonID::from(2023);
/// assert_eq!(s2023.year, 2023);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SeasonID {
    pub year: Year,
}

/// Uniquely identifies a round (race weekend), in a given season, by an index, with 1 being the first round
///
/// Note that a round is only unique within a given season, and does not uniquely identify a race in the Formula One
/// championship. See [RaceID] for a unique race identifier.
///
/// # Examples
///
/// ```
/// use f1_data::id::RoundID;
///
/// let first = RoundID::from(1);
/// assert_eq!(first.round, 1);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct RoundID {
    pub round: Round,
}

/// Uniquely identifies a race by year and round, e.g. 2023 round 1 for the first race of the 2023 season
///
/// # Examples
///
/// ```
/// use f1_data::id::RaceID;
///
/// let race = RaceID::from(2023, 1);
/// assert_eq!(race.year, 2023);
/// assert_eq!(race.round, 1);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RaceID {
    pub year: Year,
    pub round: Round,
}

impl From<&str> for DriverID {
    fn from(id: &str) -> DriverID {
        DriverID { id: id.to_string() }
    }
}

impl From<&str> for ConstructorID {
    fn from(id: &str) -> ConstructorID {
        ConstructorID { id: id.to_string() }
    }
}

impl From<&str> for CircuitID {
    fn from(id: &str) -> CircuitID {
        CircuitID { id: id.to_string() }
    }
}

impl From<Year> for SeasonID {
    fn from(year: Year) -> SeasonID {
        SeasonID { year }
    }
}

impl From<Round> for RoundID {
    fn from(round: Round) -> RoundID {
        RoundID { round }
    }
}

impl RaceID {
    pub fn from(year: Year, round: Round) -> RaceID {
        RaceID { year, round }
    }
}

impl From<RaceID> for SeasonID {
    fn from(race: RaceID) -> SeasonID {
        SeasonID { year: race.year }
    }
}

impl From<RaceID> for RoundID {
    fn from(race: RaceID) -> RoundID {
        RoundID { round: race.round }
    }
}

impl FromStr for DriverID {
    type Err = Void;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(Self { id: id.to_string() })
    }
}

impl FromStr for ConstructorID {
    type Err = Void;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(Self { id: id.to_string() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn driver_id_from_str() {
        assert_eq!(
            DriverID {
                id: "alonso".to_string()
            },
            DriverID::from("alonso")
        );

        assert_eq!(DriverID::from("alonso").id, String::from("alonso"));

        assert_eq!(DriverID::from("alonso"), DriverID::from("alonso"));
        assert_ne!(DriverID::from("alonso"), DriverID::from("hamilton"));
    }

    #[test]
    fn constructor_id_from_str() {
        assert_eq!(
            ConstructorID {
                id: "mclaren".to_string()
            },
            ConstructorID::from("mclaren")
        );

        assert_eq!(ConstructorID::from("mclaren").id, String::from("mclaren"));

        assert_eq!(
            ConstructorID::from("mclaren"),
            ConstructorID::from("mclaren")
        );
        assert_ne!(
            ConstructorID::from("mclaren"),
            ConstructorID::from("ferrari")
        );
    }

    #[test]
    fn circuit_id_from_str() {
        assert_eq!(
            CircuitID {
                id: "bahrain".to_string()
            },
            CircuitID::from("bahrain")
        );

        assert_eq!(CircuitID::from("bahrain").id, String::from("bahrain"));

        assert_eq!(CircuitID::from("bahrain"), CircuitID::from("bahrain"));
        assert_ne!(CircuitID::from("bahrain"), CircuitID::from("albert_park"));
    }

    #[test]
    fn season_id_from_year() {
        assert_eq!(SeasonID { year: 2023 }, SeasonID::from(2023));

        assert_eq!(SeasonID::from(2023).year, 2023);

        assert_eq!(SeasonID::from(2023), SeasonID::from(2023));
        assert_ne!(SeasonID::from(2023), SeasonID::from(2022));
    }

    #[test]
    fn round_id_from_round() {
        assert_eq!(RoundID { round: 1 }, RoundID::from(1));

        assert_eq!(RoundID::from(1).round, 1);

        assert_eq!(RoundID::from(1), RoundID::from(1));
        assert_ne!(RoundID::from(1), RoundID::from(2));
    }

    #[test]
    fn race_id_from_year_and_round() {
        assert_eq!(
            RaceID {
                year: 2023,
                round: 1
            },
            RaceID::from(2023, 1)
        );

        assert_eq!(RaceID::from(2023, 1).year, 2023);
        assert_eq!(RaceID::from(2023, 1).round, 1);

        assert_eq!(RaceID::from(2023, 1), RaceID::from(2023, 1));
        assert_ne!(RaceID::from(2023, 1), RaceID::from(2022, 1));
        assert_ne!(RaceID::from(2023, 1), RaceID::from(2023, 2));
    }

    #[test]
    fn season_id_from_race_id() {
        assert_eq!(
            SeasonID { year: 2023 },
            SeasonID::from(RaceID::from(2023, 1))
        );
        assert_ne!(
            SeasonID { year: 2022 },
            SeasonID::from(RaceID::from(2023, 1))
        );

        assert_eq!(SeasonID::from(RaceID::from(2023, 1)).year, 2023);
    }

    #[test]
    fn round_id_from_race_id() {
        assert_eq!(RoundID { round: 1 }, RoundID::from(RaceID::from(2023, 1)));
        assert_ne!(RoundID { round: 2 }, RoundID::from(RaceID::from(2023, 1)));

        assert_eq!(RoundID::from(RaceID::from(2023, 2)).round, 2);
    }
}
