//! To run this example, use `RUST_LOG=validate_jolpica=<level> cargo run --example validate_jolpica`
//! where `<level>` is one of `error` (the default), `warn`, `info`, `debug`, or `trace`.
//! See the [`env_logger`](https://docs.rs/env_logger/latest/env_logger/) crate for details.
use std::sync::LazyLock;

use anyhow::anyhow;
use colored::Colorize;
use log::{debug, error, info, trace};

use f1_data::{
    error::{Error, Result},
    id::{RaceID, RoundID, SeasonID},
    jolpica::{
        get::JolpicaF1,
        resource::{Filters, PitStopFilters},
        response::{self, QualifyingResult, RaceResult, SprintResult},
    },
};

static JOLPICA: LazyLock<JolpicaF1> = LazyLock::new(|| JolpicaF1::default());

fn section_header(name: &str) {
    info!("===== {} =====", name);
}

fn section_sub_header(name: &str) {
    debug!("-- {} --", name);
}

fn section_div_line() {
    trace!("――――――――――――――――――――――――――――――――――――――――――――――――――――――――――――");
}

fn table_header(sections: &str) {
    section_div_line();
    trace!("{sections}");
    section_div_line();
}

fn count(name: &str, count: usize) {
    debug!("{name} count: {count}");
}

fn log_error<T>(result: Result<T>) {
    let msg = match result.err().unwrap() {
        Error::Http(_) => "HTTP error".into(),
        Error::Io(e) => e.to_string(),
        Error::Parse(e) => e.to_string(),
        e => e.to_string(),
    };

    error!("Error: {}", msg);
}

/// Call the provided function, retrying on HTTP errors, and forwarding anything else.
fn retry_http<T>(f: impl Fn() -> Result<T>) -> Result<T> {
    for retry_idx in 1..5 {
        match f() {
            Ok(value) => return Ok(value),
            Err(Error::Http(_)) => {
                debug!("HTTP error, retrying {retry_idx}");
                debug!("Sleeping for 10 seconds");
                std::thread::sleep(std::time::Duration::from_secs(10));
            }
            other => return other,
        }
    }
    panic!("Retried 4 times on HTTP errors, giving up");
}

fn validate_seasons(_: Configurations) -> anyhow::Result<()> {
    section_header("seasons");

    let seasons = retry_http(|| JOLPICA.get_seasons(Filters::none()))?;
    count("season", seasons.len());

    table_header("|     | year |     url");
    for (idx, season) in seasons.iter().enumerate() {
        trace!("[{idx:3}]   {:4}   {}", season.season, season.url);
    }

    Ok(())
}

fn validate_drivers(_: Configurations) -> anyhow::Result<()> {
    section_header("drivers");

    let drivers = retry_http(|| JOLPICA.get_drivers(Filters::none()))?;
    count("driver", drivers.len());

    table_header("|     |     driver_id      |     given_name family_name");
    for (idx, driver) in drivers.iter().enumerate() {
        trace!("[{idx:3}]   {:20} {} {}", driver.driver_id, driver.given_name, driver.family_name);
    }

    Ok(())
}

fn validate_constructors(_: Configurations) -> anyhow::Result<()> {
    section_header("constructors");

    let constructors = retry_http(|| JOLPICA.get_constructors(Filters::none()))?;
    count("constructor", constructors.len());

    table_header("|     |   constructor_id   |     name");
    for (idx, constructor) in constructors.iter().enumerate() {
        trace!("[{idx:3}]   {:20} {}", constructor.constructor_id, constructor.name);
    }

    Ok(())
}

fn validate_circuits(_: Configurations) -> anyhow::Result<()> {
    section_header("circuits");

    let circuits = retry_http(|| JOLPICA.get_circuits(Filters::none()))?;
    count("circuit", circuits.len());

    table_header("|     |     circuit_id     |     name");
    for (idx, circuit) in circuits.iter().enumerate() {
        trace!("[{idx:3}]   {:20} {}", circuit.circuit_id, circuit.circuit_name);
    }

    Ok(())
}

fn validate_statuses(_: Configurations) -> anyhow::Result<()> {
    section_header("statuses");

    let statuses = retry_http(|| JOLPICA.get_statuses(Filters::none()))?;
    count("status", statuses.len());

    table_header("|     |  id  | count |     status");
    for (idx, status) in statuses.iter().enumerate() {
        trace!("[{idx:3}]   {:3}     {:4}   {}", status.status_id, status.count, status.status);
    }

    Ok(())
}

fn validate_race_schedules(_: Configurations) -> anyhow::Result<()> {
    section_header("race schedules");

    let seasons = retry_http(|| JOLPICA.get_seasons(Filters::none()))?;

    for season in seasons {
        section_sub_header(&format!("season: {}", season.season));

        let races = retry_http(|| JOLPICA.get_race_schedules(Filters::new().season(season.season)))?;
        count("race", races.len());

        table_header("|    | round |    date    |     name");
        for (idx, race) in races.iter().enumerate() {
            trace!("[{idx:2}]  {:4}     {}   {}", race.round, race.date, race.race_name);
        }
    }

    Ok(())
}

trait SessionResult {
    fn name() -> &'static str;
    fn driver_id(&self) -> &str;
    fn position(&self) -> u32;
    fn add_pos_filter(filters: Filters, pos: u32) -> Filters;
    fn number(&self) -> u32;
}

impl SessionResult for QualifyingResult {
    fn name() -> &'static str {
        "QualifyingResults"
    }

    fn driver_id(&self) -> &str {
        &self.driver.driver_id
    }

    fn position(&self) -> u32 {
        self.position
    }

    fn add_pos_filter(filters: Filters, pos: u32) -> Filters {
        filters.qualifying_pos(pos)
    }

    fn number(&self) -> u32 {
        self.number
    }
}

impl SessionResult for SprintResult {
    fn name() -> &'static str {
        "SprintResults"
    }

    fn driver_id(&self) -> &str {
        &self.driver.driver_id
    }

    fn position(&self) -> u32 {
        self.position
    }

    fn add_pos_filter(filters: Filters, pos: u32) -> Filters {
        filters.sprint_pos(pos)
    }

    fn number(&self) -> u32 {
        self.number
    }
}

impl SessionResult for RaceResult {
    fn name() -> &'static str {
        "RaceResults"
    }

    fn driver_id(&self) -> &str {
        &self.driver.driver_id
    }

    fn position(&self) -> u32 {
        self.position
    }

    fn add_pos_filter(filters: Filters, pos: u32) -> Filters {
        filters.finish_pos(pos)
    }

    fn number(&self) -> u32 {
        self.number
    }
}

fn validate_granular_session_results_for_round<T>(season: SeasonID, round: RoundID)
where
    T: response::SessionResult + SessionResult,
{
    section_sub_header(&format!("granular - {}, R{}", season, round));

    let round_filters = Filters::new().season(season).round(round);

    let pos_count = retry_http(|| JOLPICA.get_drivers(round_filters.clone()))
        .unwrap()
        .iter()
        .count();

    for pos in 1..(pos_count as u32 + 1) {
        let race = retry_http(|| JOLPICA.get_session_result::<T>(T::add_pos_filter(round_filters.clone(), pos)));

        if let Ok(race) = race {
            trace!("P{pos}: {}", race.payload.driver_id());
        }
        // We ignore NotFound errors because the .qualifying_pos/sprint_pos/finish_pos filters do
        // not work for non-fishing positions, i.e. those where position_text is not a number.
        else if !matches!(race, Err(Error::NotFound)) {
            error!("P{pos} failed");
            log_error(race);
        }
    }
}

fn validate_granular_session_results_for_season<T>(season: SeasonID)
where
    T: response::SessionResult + SessionResult,
{
    section_sub_header(&format!("granular - {}", season));

    let races = retry_http(|| JOLPICA.get_race_schedules(Filters::new().season(season))).unwrap();

    for race in races {
        section_sub_header(&format!("round: {}", race.round));

        let round_filters = Filters::new().season(race.season).round(race.round);

        let race_res = retry_http(|| JOLPICA.get_session_results_for_event::<T>(round_filters.clone()));

        if let Ok(race_res) = race_res {
            count("result", race_res.payload.len());
        } else {
            error!("{}, R{} - get_session_results_for_event failed", race.season, race.round);
            log_error(race_res);

            validate_granular_session_results_for_round::<T>(race.season, race.round);
        }
    }
}

/// Check if a given [`RaceResult`] is known to have [`RaceResult::NO_NUMBER`].
fn is_known_no_number(season: SeasonID, round: RoundID, position: u32) -> bool {
    const KNOWN_RACE_RESULTS_WITH_NO_NUMBER: &[(u32, u32, &[u32])] = &[(1962, 4, &[19, 20, 21, 22]), (1963, 10, &[23])];

    KNOWN_RACE_RESULTS_WITH_NO_NUMBER
        .iter()
        .any(|(s, r, positions)| season == *s && round == *r && positions.contains(&position))
}

fn validate_session_results<T>(configs: Configurations) -> anyhow::Result<()>
where
    T: response::SessionResult + SessionResult,
{
    let mut status = Ok(());

    section_header(T::name());

    let seasons = retry_http(|| JOLPICA.get_seasons(Filters::none()))?;

    for season in seasons {
        section_sub_header(&format!("season: {}", season.season));

        let races = retry_http(|| JOLPICA.get_session_results::<T>(Filters::new().season(season.season)));

        if let Ok(races) = races {
            count("race", races.len());

            for (idx, race) in races.iter().enumerate() {
                section_div_line();
                debug!("[{idx:2}] round {:2}, result count: {:2}", race.round, race.payload.len());

                table_header("|    | pos |     driver");
                for (idx, result) in race.payload.iter().enumerate() {
                    trace!("[{idx:2}]   {:2}    {:2}", result.position(), result.driver_id());

                    if result.number() == RaceResult::NO_NUMBER
                        && !is_known_no_number(race.season, race.round, result.position())
                    {
                        error!("{}, R{} - P{} has unexpected NO_NUMBER", race.season, race.round, result.position());
                    }
                }
            }
        } else {
            let msg = format!("{} - get_session_results failed", season.season);

            error!("{msg}");
            log_error(races);
            status = status.and(Err(anyhow!(msg)));

            if configs.granular_validation_on_error.is_enabled() {
                validate_granular_session_results_for_season::<T>(season.season);
            }

            if configs.early_exit_on_error.is_enabled() {
                return status;
            }
        }
    }

    status
}

fn validate_driver_laps(configs: Configurations) -> anyhow::Result<()> {
    let mut status = Ok(());

    section_header("driver laps");

    let seasons = retry_http(|| JOLPICA.get_seasons(Filters::none()))?;

    'season_loop: for season in seasons {
        // We know that Resource::LapTimes are not available prior to 1996, so skip those seasons.
        // Still check 1995 to validate handling of NotFound errors, etc.
        if season.season < 1995 {
            continue 'season_loop;
        }

        section_sub_header(&format!("season: {}", season.season));

        let races = retry_http(|| JOLPICA.get_race_schedules(Filters::new().season(season.season)))?;
        count("race", races.len());

        for (race_idx, race) in races.iter().enumerate() {
            let round_filters = Filters::new().season(race.season).round(race.round);
            let race_id = RaceID::from(race.season, race.round);

            let drivers = retry_http(|| JOLPICA.get_drivers(round_filters.clone())).unwrap();

            section_div_line();
            debug!("[{race_idx:2}] round {:2}, driver count: {:2}", race.round, drivers.len());

            let mut some_laps_found = false;
            let mut max_lap_count = 0;

            'driver_loop: for (driver_idx, driver) in drivers.iter().enumerate() {
                let laps = retry_http(|| JOLPICA.get_driver_laps(race_id, &driver.driver_id));

                let laps = if let Ok(laps) = laps {
                    some_laps_found = true;
                    max_lap_count = std::cmp::max(max_lap_count, laps.len());
                    laps
                }
                // We ignore NotFound errors because those are valid for seasons prior to 1996,
                // where Resource::LapTimes were not available; an overall message is still logged.
                else if let Err(Error::NotFound) = laps {
                    continue 'driver_loop;
                } else {
                    let msg =
                        format!("{}, R{}, {} - get_driver_laps_failed", race.season, race.round, driver.driver_id);

                    error!("{msg}");
                    log_error(laps);
                    status = status.and(Err(anyhow!(msg)));

                    if configs.early_exit_on_error.is_enabled() {
                        return status;
                    } else {
                        continue 'driver_loop;
                    }
                };

                section_div_line();
                trace!("[{driver_idx:2}] driver: {:20} lap count: {:2}", driver.driver_id, laps.len());

                table_header("|    | lap | pos |     time");
                for (idx, lap) in laps.iter().enumerate() {
                    trace!("[{idx:2}]   {:2}    {:2}   {:2}", lap.number, lap.position, lap.time);
                }
            }

            if !some_laps_found {
                debug!(">> No driver laps found");
            } else {
                debug!(">> Max lap count: {}", max_lap_count);
            }
        }
    }

    status
}

fn validate_lap_timings(configs: Configurations) -> anyhow::Result<()> {
    let mut status = Ok(());

    section_header("lap timings");

    let seasons = retry_http(|| JOLPICA.get_seasons(Filters::none())).unwrap();

    'season_loop: for season in seasons {
        // We know that Resource::LapTimes are not available prior to 1996, so skip those seasons.
        // Still check 1995 to validate handling of NotFound errors, etc.
        if season.season < 1995 {
            continue 'season_loop;
        }

        section_sub_header(&format!("season: {}", season.season));

        let races = retry_http(|| JOLPICA.get_race_schedules(Filters::new().season(season.season))).unwrap();
        count("race", races.len());

        for (race_idx, race) in races.iter().enumerate() {
            let race_id = RaceID::from(race.season, race.round);

            section_div_line();
            debug!("[{race_idx:2}] round {:2}", race.round);

            'lap_loop: for lap in 1..1000 {
                let lap_timings = retry_http(|| JOLPICA.get_lap_timings(race_id, lap));

                let lap_timings = if let Ok(lap_timings) = lap_timings {
                    lap_timings
                }
                // Stop loop once we find the first lap for which there are no lap timings.
                else if let Err(Error::NotFound) = lap_timings {
                    if lap == 1 {
                        debug!(">> No lap timings found");
                    }
                    break 'lap_loop;
                } else {
                    let msg = format!("{}, R{}, {} - get_lap_timings failed", race.season, race.round, lap);

                    error!("{msg}");
                    log_error(lap_timings);
                    status = status.and(Err(anyhow!(msg)));

                    if configs.early_exit_on_error.is_enabled() {
                        return status;
                    } else {
                        continue 'lap_loop;
                    }
                };

                section_div_line();
                trace!("lap: {lap:2}, timing count: {:2}", lap_timings.len());

                table_header("|    | pos |        driver        |     time");
                for (idx, timing) in lap_timings.iter().enumerate() {
                    trace!("[{idx:2}]   {:2}    {:20}   {}", timing.position, timing.driver_id, timing.time);
                }
            }
        }
    }

    status
}

fn validate_pit_stops(_: Configurations) -> anyhow::Result<()> {
    let mut status = Ok(());

    section_header("pit stops");

    let seasons = retry_http(|| JOLPICA.get_seasons(Filters::none())).unwrap();

    'season_loop: for season in seasons {
        // We know that Resource::LapTimes are not available prior to 2011, so skip those seasons.
        // Still check 2010 to validate handling of NotFound errors, etc.
        if season.season < 2010 {
            continue 'season_loop;
        }

        section_sub_header(&format!("season: {}", season.season));

        let races = retry_http(|| JOLPICA.get_race_schedules(Filters::new().season(season.season))).unwrap();
        count("race", races.len());

        'race_loop: for (race_idx, race) in races.iter().enumerate() {
            section_div_line();
            debug!("[{race_idx:2}] round {:2}", race.round);

            let pit_stops = retry_http(|| JOLPICA.get_pit_stops(PitStopFilters::new(race.season, race.round)));

            let pit_stops = if let Ok(pit_stops) = pit_stops {
                pit_stops
            }
            // Stop loop once we find the first lap for which there are no lap timings.
            else if let Err(Error::NotFound) = pit_stops {
                debug!(">> No pit stops found");
                continue 'race_loop;
            } else {
                let msg = format!("{}, R{} - get_pit_stops failed", race.season, race.round);

                error!("{msg}");
                log_error(pit_stops);
                status = status.and(Err(anyhow!(msg)));

                continue 'race_loop;
            };

            table_header("|    | lap | stop |       driver        |     duration");
            for (idx, pit_stop) in pit_stops.iter().enumerate() {
                trace!(
                    "[{idx:2}]   {:2}     {:2}    {:20}  {}",
                    pit_stop.lap, pit_stop.stop, pit_stop.driver_id, pit_stop.duration
                );
            }
        }
    }

    status
}

#[derive(Clone, Copy, Debug)]
enum Toggle {
    Enabled,
    Disabled,
}

impl Toggle {
    fn is_enabled(&self) -> bool {
        matches!(self, Toggle::Enabled)
    }
}

#[derive(Clone, Copy, Debug)]
struct Configurations {
    pub early_exit_on_error: Toggle,
    pub granular_validation_on_error: Toggle,
}

fn main() -> anyhow::Result<()> {
    env_logger::builder().format_timestamp(None).init();

    const VALIDATORS: &[fn(Configurations) -> anyhow::Result<()>] = &[
        validate_seasons,
        validate_drivers,
        validate_constructors,
        validate_circuits,
        validate_statuses,
        validate_race_schedules,
        validate_session_results::<QualifyingResult>,
        validate_session_results::<SprintResult>,
        validate_session_results::<RaceResult>,
        validate_driver_laps,
        validate_lap_timings,
        validate_pit_stops,
    ];

    // @todo Add command line interface to control configuration options.
    let configs = Configurations {
        early_exit_on_error: Toggle::Disabled,
        granular_validation_on_error: Toggle::Enabled,
    };

    let mut status = Ok(());

    for validate in VALIDATORS {
        if status.is_err() && configs.early_exit_on_error.is_enabled() {
            info!("Stopping validation due to previous error");
            break;
        }

        status = status.and(validate(configs));
    }

    // ASCII art generated with https://patorjk.com/software/taag/#p=display&h=1&v=1&f=Banner3

    const PASSED: &str = r#"
        ########     ###     ######   ######  ######## ########
        ##     ##   ## ##   ##    ## ##    ## ##       ##     ##
        ##     ##  ##   ##  ##       ##       ##       ##     ##
        ########  ##     ##  ######   ######  ######   ##     ##
        ##        #########       ##       ## ##       ##     ##
        ##        ##     ## ##    ## ##    ## ##       ##     ##
        ##        ##     ##  ######   ######  ######## ########
        "#;

    const FAILED: &str = r#"
        ########    ###    #### ##       ######## ########
        ##         ## ##    ##  ##       ##       ##     ##
        ##        ##   ##   ##  ##       ##       ##     ##
        ######   ##     ##  ##  ##       ######   ##     ##
        ##       #########  ##  ##       ##       ##     ##
        ##       ##     ##  ##  ##       ##       ##     ##
        ##       ##     ## #### ######## ######## ########
        "#;

    match &status {
        Ok(_) => info!("{}", PASSED.bold().green()),
        Err(_) => info!("{}", FAILED.bold().red()),
    }

    // Regardless of `configs.early_exit_on_error`, `status` will only contain the first error.
    status
}
