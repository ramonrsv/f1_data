//! This module contains tests for known bugs and issues, and the associated workarounds.
//!
//! This partly serves as a collection of @todo items to investigate and or fix in the crate, or
//! potentially push for fixes in the jolpica-f1 API. If any of these tests start failing, it may
//! indicate that the underlying issue has been fixed, and the associated workaround can be removed.

mod tests {
    use std::sync::LazyLock;

    use crate::{
        error::Error,
        jolpica::{
            agent::Agent,
            resource::Filters,
            response::{Position, QualifyingResult, RaceResult, SprintResult},
            time::{QualifyingTime, duration_m_s_ms},
        },
    };

    use crate::jolpica::tests::assets::*;
    use crate::tests::asserts::*;
    use shadow_asserts::assert_eq;

    /// Shared instance of [`Agent`] for use in tests, to share a rate limiter, cache, etc.
    static JOLPICA_MP: LazyLock<Agent<'_>> = LazyLock::new(|| Agent::default());

    // @todo Some race times in the jolpica-f1 API seem to be buggy, e.g. 2024, R5, sprint P20,
    // sprint result has 'millis' that is lower than P19, and the 'time', expected as a
    // "+hh:mm:ss.sss" string, is instead "+-1:57:34.853"
    //
    // I don't understand what this is. For now, as a workaround, those values are being parsed
    // as None for the race time field. If/when this test fails, we can investigate further.
    #[test]
    fn sprint_result_buggy_time() {
        assert_true!(SPRINT_RESULT_2024_5_P20.time.is_none());
        assert_eq!(
            serde_json::from_str::<SprintResult>(SPRINT_RESULT_2024_5_P20_STR).unwrap(),
            *SPRINT_RESULT_2024_5_P20
        );
    }

    #[test]
    #[ignore]
    fn get_sprint_result_buggy_time() {
        assert_eq!(
            JOLPICA_MP
                .get_sprint_result(Filters::new().season(2024).round(5).driver_id("alonso".into()))
                .unwrap()
                .sprint_result(),
            &*SPRINT_RESULT_2024_5_P20
        );
    }

    // @todo Some race times in the jolpica-f1 API seem to be buggy, e.g. 2023, R3, P13+,
    // non-lapped cars have 'millis' that are lower than P12, and the 'time', expected as a
    // "+hh:mm:ss.sss" string, is instead something like "+-1:24:07.342" for P15, for example.
    //
    // I don't understand what this is. For now, as a workaround, those values are being parsed
    // as None for the race time field. If/when this test fails, we can investigate further.
    #[test]
    fn race_result_buggy_time() {
        assert_true!(RACE_RESULT_2023_3_P15.time.is_none());
        assert_eq!(serde_json::from_str::<RaceResult>(RACE_RESULT_2023_3_P15_STR).unwrap(), *RACE_RESULT_2023_3_P15);
    }

    #[test]
    #[ignore]
    fn get_race_result_buggy_time() {
        assert_eq!(
            JOLPICA_MP
                .get_race_result(Filters::new().season(2023).round(3).finish_pos(15))
                .unwrap()
                .race_result(),
            &*RACE_RESULT_2023_3_P15
        );
    }

    // @todo jolpica-f1 is incorrectly reporting Q1 time as 1:41.131, but it should be 1:41.756
    #[test]
    fn qualifying_result_2023_4_p3() {
        assert_eq!(QUALIFYING_RESULT_2023_4_P3.q1, Some(QualifyingTime::Time(duration_m_s_ms(1, 41, 131))));
        assert_eq!(
            serde_json::from_str::<QualifyingResult>(QUALIFYING_RESULT_2023_4_P3_STR).unwrap(),
            *QUALIFYING_RESULT_2023_4_P3
        );
    }

    #[test]
    #[ignore]
    fn get_qualifying_result_2023_4_p3() {
        assert_eq!(
            JOLPICA_MP
                .get_qualifying_result(Filters::new().season(2023).round(4).driver_id("perez".into()))
                .unwrap()
                .qualifying_result(),
            &*QUALIFYING_RESULT_2023_4_P3
        );
    }

    // @todo [`Filters::qualifying_pos`] appears to not be functional in the new jolpica-f1 API
    // If/when this test begins to fail, and we can add tests filtering by `qualifying_pos`
    #[test]
    #[ignore]
    fn get_qualifying_result_by_qualifying_pos_filter() {
        assert!(matches!(
            JOLPICA_MP.get_qualifying_result(Filters::new().season(2023).round(4).qualifying_pos(1)),
            Err(Error::TooMany)
        ));
    }

    // @todo Counterintuitively, non-finishing race results cannot be filtered by .finish_pos,
    // even though .position would be set. Is it only meant to filter by Position::Finished(_)?
    #[test]
    #[ignore]
    fn get_race_result_by_finish_pos_filter_for_non_finishing() {
        assert_eq!(RACE_RESULT_2023_4_P20.position, 20);
        assert_eq!(RACE_RESULT_2023_4_P20.position_text, Position::R);

        assert!(matches!(
            JOLPICA_MP.get_race_result(Filters::new().season(2023).round(4).finish_pos(20)),
            Err(Error::NotFound)
        ));
    }

    // @todo Getting all race results for "michael_schumacher" produces the following error:
    //     Http(Json(Error("Invalid duration: \"1:34\"", ...
    #[test]
    #[ignore]
    fn get_race_result_for_events_michael_schumacher() {
        assert!(matches!(
            JOLPICA_MP.get_race_result_for_events(Filters::new().driver_id("michael_schumacher".into())),
            Err(Error::Parse(_))
        ));
    }

    // @todo Getting all race results for "hamilton" produces the following error:
    //     Http(Json(Error("Non-delta 'time: 2:19:35.060' must match 'millis: 8375059'", ...
    #[test]
    #[ignore]
    fn get_race_result_for_events_hamilton() {
        assert!(matches!(
            JOLPICA_MP.get_race_result_for_events(Filters::new().driver_id("hamilton".into())),
            Err(Error::Parse(_))
        ));
    }
}
