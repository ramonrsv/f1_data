use serde::{Deserialize, Serialize};

use crate::id::{ConstructorID, DriverID, RoundID};

/// Type alias for the price of a driver or constructor, in millions of dollars.
pub type Price = f32;

/// Holds price data for all rounds of a single season.
#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct Season {
    /// Sequence of [`Round`]s, for all available rounds of a given season.
    pub rounds: Vec<Round>,
}

/// Holds price data for a single round of a season.
#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct Round {
    /// ID/index of the round, e.g. `1` for the first round of the season.
    #[allow(clippy::struct_field_names)]
    pub round: RoundID,
    /// Sequence of [`Driver`]s, for all drivers that have a price for this round.
    pub drivers: Vec<Driver>,
    /// Sequence of [`Constructor`]s, for all constructors that have a price for this round.
    pub constructors: Vec<Constructor>,
}

/// Holds price data for a single driver.
#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct Driver {
    /// ID of the driver, e.g. `max_verstappen` for _Max Verstappen_.
    pub driver_id: DriverID,
    /// Price of the driver, in millions of dollars.
    pub price: Price,
}

/// Holds price data for a single constructor.
#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct Constructor {
    /// ID of the constructor, e.g. `red_bull` for _Red Bull Racing_.
    pub constructor_id: ConstructorID,
    /// Price of the constructor, in millions of dollars.
    pub price: Price,
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use std::sync::LazyLock;

    use const_format::formatcp;

    use crate::tests::asserts::*;
    use shadow_asserts::assert_eq;

    use super::*;

    impl Driver {
        fn new(driver_id: &str, price: Price) -> Self {
            Self {
                driver_id: driver_id.into(),
                price,
            }
        }
    }

    impl Constructor {
        fn new(constructor_id: &str, price: Price) -> Self {
            Self {
                constructor_id: constructor_id.into(),
                price,
            }
        }
    }

    const DRIVER_MAX_R1_STR: &str = r#"{ driver_id: "max_verstappen", price: 26.9 }"#;
    const DRIVER_MAX_R2_STR: &str = r#"{ driver_id: "max_verstappen", price: 27.0 }"#;
    const DRIVER_HAM_R1_STR: &str = r#"{ driver_id: "hamilton", price: 23.7 }"#;
    const DRIVER_HAM_R2_STR: &str = r#"{ driver_id: "hamilton", price: 23.7 }"#;

    static DRIVER_MAX_R1: LazyLock<Driver> = LazyLock::new(|| Driver::new("max_verstappen", 26.9));
    static DRIVER_MAX_R2: LazyLock<Driver> = LazyLock::new(|| Driver::new("max_verstappen", 27.0));
    static DRIVER_HAM_R1: LazyLock<Driver> = LazyLock::new(|| Driver::new("hamilton", 23.7));
    static DRIVER_HAM_R2: LazyLock<Driver> = LazyLock::new(|| Driver::new("hamilton", 23.7));

    const CONSTRUCTOR_RED_BULL_R1_STR: &str = r#"{ constructor_id: "red_bull", price: 27.2 }"#;
    const CONSTRUCTOR_RED_BULL_R2_STR: &str = r#"{ constructor_id: "red_bull", price: 27.3 }"#;
    const CONSTRUCTOR_MERCEDES_R1_STR: &str = r#"{ constructor_id: "mercedes", price: 25.1 }"#;
    const CONSTRUCTOR_MERCEDES_R2_STR: &str = r#"{ constructor_id: "mercedes", price: 25.1 }"#;

    static CONSTRUCTOR_RED_BULL_R1: LazyLock<Constructor> = LazyLock::new(|| Constructor::new("red_bull", 27.2));
    static CONSTRUCTOR_RED_BULL_R2: LazyLock<Constructor> = LazyLock::new(|| Constructor::new("red_bull", 27.3));
    static CONSTRUCTOR_MERCEDES_R1: LazyLock<Constructor> = LazyLock::new(|| Constructor::new("mercedes", 25.1));
    static CONSTRUCTOR_MERCEDES_R2: LazyLock<Constructor> = LazyLock::new(|| Constructor::new("mercedes", 25.1));

    const ROUND_R1_STR: &str = formatcp!(
        r#"{{
        round: 1,
        drivers:
          [
            {DRIVER_MAX_R1_STR},
            {DRIVER_HAM_R1_STR},
          ],
        constructors:
          [
            {CONSTRUCTOR_RED_BULL_R1_STR},
            {CONSTRUCTOR_MERCEDES_R1_STR},
          ],
      }}"#
    );

    const ROUND_R2_STR: &str = formatcp!(
        r#"{{
        round: 2,
        drivers:
          [
            {DRIVER_MAX_R2_STR},
            {DRIVER_HAM_R2_STR},
          ],
        constructors:
          [
            {CONSTRUCTOR_RED_BULL_R2_STR},
            {CONSTRUCTOR_MERCEDES_R2_STR},
          ],
      }}"#
    );

    static ROUND_R1: LazyLock<Round> = LazyLock::new(|| Round {
        round: 1,
        drivers: vec![DRIVER_MAX_R1.clone(), DRIVER_HAM_R1.clone()],
        constructors: vec![CONSTRUCTOR_RED_BULL_R1.clone(), CONSTRUCTOR_MERCEDES_R1.clone()],
    });

    static ROUND_R2: LazyLock<Round> = LazyLock::new(|| Round {
        round: 2,
        drivers: vec![DRIVER_MAX_R2.clone(), DRIVER_HAM_R2.clone()],
        constructors: vec![CONSTRUCTOR_RED_BULL_R2.clone(), CONSTRUCTOR_MERCEDES_R2.clone()],
    });

    const SEASON_STR: &str = formatcp!(
        r#"rounds: [
        {ROUND_R1_STR},
        {ROUND_R2_STR},
      ]
    "#
    );

    static SEASON: LazyLock<Season> = LazyLock::new(|| Season {
        rounds: vec![ROUND_R1.clone(), ROUND_R2.clone()],
    });

    #[test]
    fn season() {
        assert_eq!(serde_yaml::from_str::<Season>(SEASON_STR).unwrap(), *SEASON);
    }

    #[test]
    fn round() {
        for (round_str, round) in [(ROUND_R1_STR, &ROUND_R1), (ROUND_R2_STR, &ROUND_R2)] {
            assert_eq!(serde_yaml::from_str::<Round>(round_str).unwrap(), **round);
        }
    }

    #[test]
    fn driver() {
        for (driver_str, driver) in [
            (DRIVER_MAX_R1_STR, &DRIVER_MAX_R1),
            (DRIVER_MAX_R2_STR, &DRIVER_MAX_R2),
            (DRIVER_HAM_R1_STR, &DRIVER_HAM_R1),
            (DRIVER_HAM_R2_STR, &DRIVER_HAM_R2),
        ] {
            assert_eq!(serde_yaml::from_str::<Driver>(driver_str).unwrap(), **driver);
        }
    }

    #[test]
    fn constructor() {
        for (constructor_str, constructor) in [
            (CONSTRUCTOR_RED_BULL_R1_STR, &CONSTRUCTOR_RED_BULL_R1),
            (CONSTRUCTOR_RED_BULL_R2_STR, &CONSTRUCTOR_RED_BULL_R2),
            (CONSTRUCTOR_MERCEDES_R1_STR, &CONSTRUCTOR_MERCEDES_R1),
            (CONSTRUCTOR_MERCEDES_R2_STR, &CONSTRUCTOR_MERCEDES_R2),
        ] {
            assert_eq!(serde_yaml::from_str::<Constructor>(constructor_str).unwrap(), **constructor);
        }
    }

    #[test]
    fn serialize_deserialize() {
        let season_str = serde_yaml::to_string(&*SEASON).unwrap();
        let season: Season = serde_yaml::from_str(&season_str).unwrap();

        assert_eq!(season, *SEASON);
    }
}
