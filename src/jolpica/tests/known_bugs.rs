//! This module contains tests for known bugs and issues, and the associated workarounds.
//!
//! This partly serves as a collection of @todo items to investigate and or fix in the crate, or
//! potentially push for fixes in the jolpica-f1 API. If any of these tests start failing, it may
//! indicate that the underlying issue has been fixed, and the associated workaround can be removed.

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use serde::Deserialize;

    use crate::{
        error::Error,
        jolpica::{
            resource::Filters,
            response::{Position, QualifyingResult, RaceResult, SprintResult},
            tests::util::JOLPICA_MP,
            time::{
                QualifyingTime, RaceTime, deserialize_buggy_race_time, duration_hms_ms, duration_m_s_ms,
                duration_millis,
            },
        },
    };

    use crate::jolpica::tests::assets::*;
    use crate::tests::asserts::*;
    use shadow_asserts::assert_eq;

    // @todo Some race times in the jolpica-f1 API seem to be buggy. I don't fully understand these
    // issues yet. For now, as a workaround, [`RaceTime`]s are being parsed with a temporary hack in
    // `jolpica::time::deserialize_buggy_race_time`, which is being tested here. If/when any of
    // these tests fail, we can investigate further, and potentially remove the workaround.
    //
    // "+-" issue, for example:
    //   - 2024, R5, sprint P20, sprint result has 'millis' that is lower than P19, and the
    //     'time', expected as a "+hh:mm:ss.sss" string, is instead "+-1:57:34.853"
    //   - 2023, R3, P13+, non-lapped cars have 'millis' that are lower than P12, and the 'time',
    //     expected as a "+hh:mm:ss.sss" string, is instead something like "+-1:24:07.342" for P15
    //
    // "hh:mm" issue, for example:
    //   - 1950, R5, P1, the 'time' should be "2:47:26" but is instead "2:47". 'millis' is correct
    //
    // 2020, R9, P1 issue:
    //   - 2020, R9, P1 race result has incorrect 'millis', off by 1ms, it should be 8375060.
    //     This causes a parsing error as it finds that the 'time' and 'millis' do not match.

    #[derive(Deserialize, PartialEq, Clone, Debug)]
    struct Proxy {
        #[serde(flatten, rename = "Time", default, deserialize_with = "deserialize_buggy_race_time")]
        time: Option<RaceTime>,
    }

    impl Proxy {
        fn new(time: RaceTime) -> Self {
            Self { time: Some(time) }
        }

        fn none() -> Self {
            Self { time: None }
        }
    }

    #[test]
    fn deserialize_race_time() {
        assert_eq!(
            serde_json::from_str::<RaceTime>(r#"{"millis": "7373700", "time": "2:02:53.7"}"#).unwrap(),
            RaceTime::lead(duration_millis(7373700))
        );

        assert_eq!(
            serde_json::from_str::<Proxy>(r#"{"millis": "7373700", "time": "2:02:53.7"}"#).unwrap(),
            Proxy::new(RaceTime::lead(duration_millis(7373700)))
        );
    }

    #[test]
    fn deserialize_buggy_race_time_workarounds() {
        // "+-" issue, works when we use `deserialize_buggy_race_time`, should return [`None`]
        assert_eq!(
            serde_json::from_str::<Proxy>(r#"{"millis": "1779513", "time": "+-1:57:34.853"}"#).unwrap(),
            Proxy::none()
        );

        assert_eq!(duration_millis(10046000), duration_hms_ms(2, 47, 26, 0));

        // "hh:mm" issue, works when we use `deserialize_buggy_race_time` (in Proxy's
        // `deserialize_with` attribute), using 'millis' to get the seconds component.
        assert_eq!(
            serde_json::from_str::<Proxy>(r#"{"millis": "10046000", "time": "2:47"}"#).unwrap(),
            Proxy::new(RaceTime::lead(duration_millis(10046000)))
        );
    }

    #[test]
    fn deserialize_buggy_race_time_workarounds_error_not_using_deserialize_with() {
        // "hh:mm" issue, doesn't work when we deserialize a `RaceTime` directly, without workaround
        let result = serde_json::from_str::<RaceTime>(r#"{"millis": "10046000", "time": "2:47"}"#);
        assert!(matches!(result, Err(serde_json::Error { .. })));

        let err_msg = format!("{}", result.unwrap_err());
        assert_true!(err_msg.contains("Invalid duration: \"2:47\""));
    }

    #[test]
    fn deserialize_buggy_race_time_workarounds_error_does_not_match_within_60s() {
        // "hh:mm" issue, doesn't work when the 'time' doesn't match 'millis' to within 60s
        let result = serde_json::from_str::<Proxy>(r#"{"millis": "10046000", "time": "2:49"}"#);
        assert!(matches!(result, Err(serde_json::Error { .. })));

        let err_msg = format!("{}", result.unwrap_err());
        assert_true!(err_msg.contains("Buggy delta 'time: 2:49' does not match 'millis: 10046000' to within 60s"));
    }

    #[test]
    fn sprint_result_buggy_time() {
        // "+-" issue
        assert_true!(SPRINT_RESULT_2024_5_P20.time.is_none());
        assert_eq!(
            serde_json::from_str::<SprintResult>(SPRINT_RESULT_2024_5_P20_STR).unwrap(),
            *SPRINT_RESULT_2024_5_P20
        );
    }

    #[test]
    #[ignore]
    fn get_sprint_result_buggy_time() {
        // "+-" issue
        let result = JOLPICA_MP.get_sprint_result(Filters::new().season(2024).round(5).driver_id("alonso".into()));
        assert_eq!(result.unwrap().sprint_result(), &*SPRINT_RESULT_2024_5_P20);
    }

    #[test]
    fn race_result_buggy_time() {
        // "+-" issue
        assert_true!(RACE_RESULT_2023_3_P15.time.is_none());
        assert_eq!(serde_json::from_str::<RaceResult>(RACE_RESULT_2023_3_P15_STR).unwrap(), *RACE_RESULT_2023_3_P15);

        // "hh:mm" issue
        assert_true!(RACE_RESULT_1950_5_P1.time.is_some());
        assert_eq!(serde_json::from_str::<RaceResult>(RACE_RESULT_1950_5_P1_STR).unwrap(), *RACE_RESULT_1950_5_P1);

        // "hh:mm" issue
        assert_true!(RACE_RESULT_1998_8_P1.time.is_some());
        assert_eq!(serde_json::from_str::<RaceResult>(RACE_RESULT_1998_8_P1_STR).unwrap(), *RACE_RESULT_1998_8_P1);
    }

    #[test]
    #[ignore]
    fn get_race_result_buggy_time() {
        // "+-" issue
        let result = JOLPICA_MP.get_race_result(Filters::new().season(2023).round(3).finish_pos(15));
        assert_eq!(result.unwrap().race_result(), &*RACE_RESULT_2023_3_P15);

        // "hh:mm" issue
        let result = JOLPICA_MP.get_race_result(Filters::new().season(1950).round(5).finish_pos(1));
        assert_eq!(result.unwrap().race_result(), &*RACE_RESULT_1950_5_P1);

        // "hh:mm" issue
        let result = JOLPICA_MP.get_race_result(Filters::new().season(1998).round(8).finish_pos(1));
        assert_eq!(result.unwrap().race_result(), &*RACE_RESULT_1998_8_P1);
    }

    // @todo jolpica-f1 is incorrectly reporting Q1 time as 1:41.131, but it should be 1:41.756
    #[test]
    fn qualifying_result_2023_4_p3() {
        assert_eq!(QUALIFYING_RESULT_2023_4_P3.q1, Some(QualifyingTime::Time(duration_m_s_ms(1, 41, 131))));
        let result = serde_json::from_str::<QualifyingResult>(QUALIFYING_RESULT_2023_4_P3_STR);
        assert_eq!(result.unwrap(), *QUALIFYING_RESULT_2023_4_P3);
    }

    #[test]
    #[ignore]
    fn get_qualifying_result_2023_4_p3() {
        let result = JOLPICA_MP.get_qualifying_result(Filters::new().season(2023).round(4).driver_id("perez".into()));
        assert_eq!(result.unwrap().qualifying_result(), &*QUALIFYING_RESULT_2023_4_P3);
    }

    // @todo The 'millis' field is incorrect by 26ms in jolpica-f1, it should be "5685026"
    #[test]
    fn race_result_1998_8_p1() {
        assert_eq!(RACE_RESULT_1998_8_P1.time, Some(RaceTime::lead(duration_millis(5685000))));
        let result = serde_json::from_str::<RaceResult>(RACE_RESULT_1998_8_P1_STR);
        assert_eq!(result.unwrap(), *RACE_RESULT_1998_8_P1);
    }

    #[test]
    #[ignore]
    fn get_race_result_1998_8_p1() {
        let result = JOLPICA_MP.get_race_result(Filters::new().season(1998).round(8).finish_pos(1));
        assert_eq!(result.unwrap().race_result(), &*RACE_RESULT_1998_8_P1);
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

    // @todo 2020, R9, P1 "hamilton" has incorrect 'millis', off by 1ms, it should be 8375060
    // This causes a parsing error as it find that the 'time' and 'millis' do not match.
    //
    // !!! <<<
    // There is a ridiculous workaround for this one specific case implemented in
    // `jolpica::time::deserialize_buggy_race_time`, where we check for an exact match
    // of the known incorrect value, and substitute them with the known correct value.
    #[test]
    fn race_result_2020_9_p1_workaround() {
        // Works when we use `deserialize_buggy_race_time` (in Proxy's `deserialize_with`
        // attribute), using the monstrous workaround to hard-code 8375059 -> 8375060
        assert_eq!(
            serde_json::from_str::<Proxy>(r#"{"millis": "8375059", "time": "2:19:35.060"}"#).unwrap(),
            Proxy::new(RaceTime::lead(duration_millis(8375060)))
        );

        let result = serde_json::from_str::<RaceResult>(RACE_RESULT_2020_9_P1_STR);
        assert_eq!(result.unwrap(), *RACE_RESULT_2020_9_P1);
    }

    #[test]
    fn race_result_2020_9_p1_error_not_using_deserialize_with() {
        // Parsing error when we deserialize a `RaceTime` directly, without the workaround
        let result = serde_json::from_str::<RaceTime>(r#"{"millis": "8375059", "time": "2:19:35.060"}"#);
        assert!(matches!(result, Err(serde_json::Error { .. })));
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("Non-delta 'time: 2:19:35.060' must match 'millis: 8375059'"));
    }

    #[test]
    #[ignore]
    fn get_race_result_2020_9_p1() {
        let result = JOLPICA_MP.get_race_result(Filters::new().season(2020).round(9).finish_pos(1));
        assert_eq!(result.unwrap().race_result(), &*RACE_RESULT_2020_9_P1);
    }
}
