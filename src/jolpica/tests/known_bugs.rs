//! This module contains tests for known bugs and issues, and the associated workarounds.
//!
//! This partly serves as a collection of @todo items to investigate and potentially push for fixes
//! in the jolpica-f1 API. If and when any of these tests start failing, it may indicate that
//! the underlying issue has been fixed, and the associated workaround can be removed.

mod tests {
    use pretty_assertions::assert_eq;

    use crate::jolpica::{agent::Agent, resource::Filters, response::RaceResult};

    use crate::jolpica::tests::assets::*;

    #[test]
    fn race_result_buggy_time() {
        assert!(RACE_RESULT_2023_3_P15.time.is_none());
        assert_eq!(serde_json::from_str::<RaceResult>(RACE_RESULT_2023_3_P15_STR).unwrap(), *RACE_RESULT_2023_3_P15);
    }

    #[test]
    #[ignore]
    fn get_race_result_buggy_time() {
        // @todo Some race times in the jolpica-f1 API seem to be buggy, e.g. 2023, R3, P13+,
        // non-lapped cars have 'millis' that are lower than P12, and the 'time', expected as a
        // "+hh:mm:ss.sss" string, is instead something like "+-1:24:07.342" for P15, for example.
        //
        // I don't understand what this is. For now, as a workaround, those values are being parsed
        // as None for the race time field. If/when this test fails, we can investigate further.
        assert_eq!(
            Agent::default()
                .get_race_result(Filters::new().season(2023).round(3).finish_pos(15))
                .unwrap()
                .race_result(),
            &*RACE_RESULT_2023_3_P15
        );
    }
}
