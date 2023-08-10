use log::{debug, error, info, trace};

use f1_data::{
    ergast::{
        error::{Error, Result},
        get::{
            self, get_circuits, get_constructors, get_drivers, get_race_schedules, get_seasons, get_session_result,
            get_session_results, get_session_results_for_event, get_statuses,
        },
        resource::Filters,
        response::{QualifyingResult, RaceResult, SprintResult},
    },
    id::{RoundID, SeasonID},
};

fn section_header(name: &str) {
    info!("===== {} =====", name);
}

fn section_sub_header(name: &str) {
    debug!("-- {} --", name);
}

fn table_header(sections: &str) {
    trace!("――――――――――――――――――――――――――――――――――――――――――――――――――");
    trace!("{sections}");
    trace!("――――――――――――――――――――――――――――――――――――――――――――――――――");
}

fn count(name: &str, count: usize) {
    debug!("{name} count: {count}");
}

/// Call the provided function, retrying on HTTP errors, and forwarding anything else.
fn retry_http<T>(f: impl Fn() -> Result<T>) -> Result<T> {
    for retry_idx in 1..5 {
        match f() {
            Ok(value) => return Ok(value),
            Err(Error::Http(_)) => debug!("HTTP error, retrying {retry_idx}"),
            other => return other,
        }
    }
    panic!("Retried 4 times on HTTP errors, giving up");
}

fn validate_seasons() {
    section_header("seasons");

    let seasons = retry_http(|| get_seasons(Filters::none())).unwrap();
    count("season", seasons.len());

    table_header("|     | year |     url");
    for (idx, season) in seasons.iter().enumerate() {
        trace!("[{idx:3}]   {:4}   {}", season.season, season.url);
    }
}

fn validate_drivers() {
    section_header("drivers");

    let drivers = retry_http(|| get_drivers(Filters::none())).unwrap();
    count("driver", drivers.len());

    table_header("|     |     driver_id      |     given_name family_name");
    for (idx, driver) in drivers.iter().enumerate() {
        trace!("[{idx:3}]   {:20} {} {}", driver.driver_id, driver.given_name, driver.family_name);
    }
}

fn validate_constructors() {
    section_header("constructors");

    let constructors = retry_http(|| get_constructors(Filters::none())).unwrap();
    count("constructor", constructors.len());

    table_header("|     |   constructor_id   |     name");
    for (idx, constructor) in constructors.iter().enumerate() {
        trace!("[{idx:3}]   {:20} {}", constructor.constructor_id, constructor.name);
    }
}

fn validate_circuits() {
    section_header("circuits");

    let circuits = retry_http(|| get_circuits(Filters::none())).unwrap();
    count("circuit", circuits.len());

    table_header("|     |     circuit_id     |     name");
    for (idx, circuit) in circuits.iter().enumerate() {
        trace!("[{idx:3}]   {:20} {}", circuit.circuit_id, circuit.circuit_name);
    }
}

fn validate_statuses() {
    section_header("statuses");

    let statuses = retry_http(|| get_statuses(Filters::none())).unwrap();
    count("status", statuses.len());

    table_header("|     |  id  | count |     status");
    for (idx, status) in statuses.iter().enumerate() {
        trace!("[{idx:3}]   {:3}     {:4}   {}", status.status_id, status.count, status.status);
    }
}

fn validate_race_schedules() {
    section_header("race schedules");

    let seasons = retry_http(|| get_seasons(Filters::none())).unwrap();

    for season in seasons {
        section_sub_header(&format!("season: {}", season.season));

        let races = retry_http(|| get_race_schedules(Filters::new().season(season.season))).unwrap();
        count("race", races.len());

        table_header("|    | round |    date    |     name");
        for (idx, race) in races.iter().enumerate() {
            trace!("[{idx:2}]  {:4}     {}   {}", race.round, race.date, race.race_name);
        }
    }
}

trait SessionResult {
    fn name() -> &'static str;
    fn driver_id(&self) -> &str;
    fn position(&self) -> u32;
    fn add_pos_filter(filters: Filters, pos: u32) -> Filters;
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

fn validate_granular_session_results_for_round<T>(season: SeasonID, round: RoundID)
where
    T: get::SessionResult + SessionResult,
{
    section_sub_header(&format!("granular - {} - R{}", season, round));

    let round_filters = Filters::new().season(season).round(round);

    let pos_count = retry_http(|| get_drivers(round_filters.clone()))
        .unwrap()
        .iter()
        .count();

    for pos in 1..(pos_count as u32 + 1) {
        let race = retry_http(|| get_session_result::<T>(T::add_pos_filter(round_filters.clone(), pos)));

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
    T: get::SessionResult + SessionResult,
{
    section_sub_header(&format!("granular - {}", season));

    let races = retry_http(|| get_race_schedules(Filters::new().season(season))).unwrap();

    for race in races {
        section_sub_header(&format!("round: {}", race.round));

        let round_filters = Filters::new().season(race.season).round(race.round);

        let race_res = retry_http(|| get_session_results_for_event::<T>(round_filters.clone()));

        if let Ok(race_res) = race_res {
            count("result", race_res.payload.len());
        } else {
            error!("{} - R{} - get_session_results_for_event failed", race.season, race.round);
            log_error(race_res);

            validate_granular_session_results_for_round::<T>(race.season, race.round);
        }
    }
}

fn validate_session_results<T>()
where
    T: get::SessionResult + SessionResult,
{
    section_header(T::name());

    let seasons = retry_http(|| get_seasons(Filters::none())).unwrap();

    for season in seasons {
        section_sub_header(&format!("season: {}", season.season));

        let races = retry_http(|| get_session_results::<T>(Filters::new().season(season.season)));

        if let Ok(races) = races {
            count("race", races.len());

            for (idx, race) in races.iter().enumerate() {
                trace!("――――――――――――――――――――――――――――――――――――――――――――――――――");
                debug!("[{idx:2}] round {:2}, result count: {:2}", race.round, race.payload.len());

                table_header("|    | pos |     driver");
                for (idx, result) in race.payload.iter().enumerate() {
                    trace!("[{idx:2}]   {:2}    {:2}", result.position(), result.driver_id());
                }
            }
        } else {
            error!("{} - get_session_results failed", season.season);
            log_error(races);

            validate_granular_session_results_for_season::<T>(season.season);
        }
    }
}

fn main() {
    env_logger::builder().format_timestamp(None).init();

    validate_seasons();
    validate_drivers();
    validate_constructors();
    validate_circuits();
    validate_statuses();

    validate_race_schedules();

    validate_session_results::<QualifyingResult>();
    validate_session_results::<SprintResult>();
    validate_session_results::<RaceResult>();
}
